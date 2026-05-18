use core::{arch::asm, fmt};

use crate::{
    boot::BootInfo,
    device_tree::DeviceTree,
    mmio::{MmioMap, MmioRegion},
    target::{InterruptControllerKind, TargetServices, TimerKind, UartKind},
};

const PL011_BASE: usize = 0x0900_0000;
const UART_FR: usize = 0x18;
const UART_IBRD: usize = 0x24;
const UART_FBRD: usize = 0x28;
const UART_LCRH: usize = 0x2c;
const UART_CR: usize = 0x30;
const UART_IMSC: usize = 0x38;
const UART_ICR: usize = 0x44;

const UART_FR_TXFF: u32 = 1 << 5;

const MMIO_REGIONS: &[MmioRegion] = &[MmioRegion::new("qemu-virt-pl011-uart0", PL011_BASE, 0x1000)];

#[derive(Clone, Copy)]
pub struct Pl011 {
    base: usize,
}

impl Pl011 {
    pub const fn new(base: usize) -> Self {
        Self { base }
    }

    pub fn init(self) {
        self.write_reg(UART_CR, 0);
        self.write_reg(UART_ICR, 0x7ff);
        self.write_reg(UART_IBRD, 1);
        self.write_reg(UART_FBRD, 40);
        self.write_reg(UART_LCRH, (0b11 << 5) | (1 << 4));
        self.write_reg(UART_IMSC, 0);
        self.write_reg(UART_CR, (1 << 0) | (1 << 8) | (1 << 9));
    }

    pub fn write_byte(self, byte: u8) {
        while self.read_reg(UART_FR) & UART_FR_TXFF != 0 {}
        self.write_reg(0, byte as u32);
    }

    fn read_reg(self, offset: usize) -> u32 {
        let addr = (self.base + offset) as *const u32;
        let value: u32;
        unsafe {
            asm!(
                "ldr {value:w}, [{addr}]",
                value = out(reg) value,
                addr = in(reg) addr,
                options(nostack, preserves_flags, readonly)
            );
        }
        value
    }

    fn write_reg(self, offset: usize, value: u32) {
        let addr = (self.base + offset) as *mut u32;
        unsafe {
            asm!(
                "str {value:w}, [{addr}]",
                value = in(reg) value,
                addr = in(reg) addr,
                options(nostack, preserves_flags)
            );
        }
    }
}

impl fmt::Write for Pl011 {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            if byte == b'\n' {
                self.write_byte(b'\r');
            }
            self.write_byte(byte);
        }
        Ok(())
    }
}

pub fn init() {
    console().init();
}

pub fn console() -> Pl011 {
    Pl011::new(PL011_BASE)
}

pub fn services(boot_info: &BootInfo) -> TargetServices {
    TargetServices {
        uart: UartKind::Pl011,
        timer: TimerKind::ArmGeneric,
        interrupt_controller: InterruptControllerKind::GicV2,
        mmio_map: MmioMap::new(MMIO_REGIONS),
        device_tree: DeviceTree::from_physical_address(boot_info.dtb_pa),
    }
}
