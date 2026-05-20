use crate::{
    boot::BootInfo,
    device_tree::DeviceTree,
    mmio::{MmioMap, MmioRegion},
    target::{InterruptControllerKind, TargetServices, TimerKind, UartKind},
};

#[cfg(talos_target_rpi5_bcm2712)]
use crate::pl011::Pl011;

pub const UART10_BASE: usize = 0x10_7d00_1000;
pub const RP1_UART0_PCIE2_BASE: usize = 0x1f_0003_0000;
pub const RP1_UART0_FIRMWARE_BASE: usize = 0x1c_0003_0000;
pub const RP1_UART0_BASE: usize = RP1_UART0_PCIE2_BASE;
#[allow(dead_code)]
pub const RP1_UART0_GPIO14_PAD: usize = 0x1f_000f_003c;
#[allow(dead_code)]
pub const RP1_UART0_GPIO15_PAD: usize = 0x1f_000f_0040;
#[allow(dead_code)]
pub const RP1_UART0_GPIO14_CTRL: usize = 0x1f_000d_0074;
#[allow(dead_code)]
pub const RP1_UART0_GPIO15_CTRL: usize = 0x1f_000d_007c;

const MMIO_REGIONS: &[MmioRegion] = &[
    MmioRegion::new("bcm2712-local-peripherals", 0x10_7c00_0000, 0x0400_0000),
    MmioRegion::new("bcm2712-gic-400", 0x10_7fff_9000, 0x0001_0000),
    MmioRegion::new("bcm2712-uart10-pl011", UART10_BASE, 0x0000_0200),
    MmioRegion::new("rp1-uart0-pl011-pcie2", RP1_UART0_BASE, 0x0000_0100),
    MmioRegion::new("rp1-gpio-pads", 0x1f_000f_0000, 0x0000_1000),
    MmioRegion::new("rp1-gpio-ctrl", 0x1f_000d_0000, 0x0000_1000),
    MmioRegion::new(
        "rp1-uart0-pl011-firmware-preserved",
        RP1_UART0_FIRMWARE_BASE,
        0x0000_0100,
    ),
];

pub fn init_stub() {
    init_rp1_uart0_pins();
}

#[cfg(talos_target_rpi5_bcm2712)]
fn init_rp1_uart0_pins() {
    write_rp1_reg_flush(RP1_UART0_GPIO14_PAD, 0x40);
    write_rp1_reg_flush(RP1_UART0_GPIO15_PAD, 0x48);
    write_rp1_reg_flush(RP1_UART0_GPIO14_CTRL, 4);
    write_rp1_reg_flush(RP1_UART0_GPIO15_CTRL, 4);
}

#[cfg(not(talos_target_rpi5_bcm2712))]
fn init_rp1_uart0_pins() {}

#[cfg(talos_target_rpi5_bcm2712)]
fn write_rp1_reg_flush(addr: usize, value: u32) {
    let reg = addr as *mut u32;
    unsafe {
        core::ptr::write_volatile(reg, value);
        let _ = core::ptr::read_volatile(reg);
        core::arch::asm!("dsb sy", options(nostack, preserves_flags));
    }
}

#[cfg(talos_target_rpi5_bcm2712)]
pub fn firmware_console() -> Pl011 {
    Pl011::new_with_posted_write_flush(RP1_UART0_BASE)
}

pub fn services(boot_info: &BootInfo) -> TargetServices {
    TargetServices {
        uart: UartKind::FirmwarePreserved,
        timer: TimerKind::ArmGeneric,
        interrupt_controller: InterruptControllerKind::GicV2,
        mmio_map: MmioMap::new(MMIO_REGIONS),
        device_tree: DeviceTree::from_physical_address(boot_info.dtb_pa),
    }
}
