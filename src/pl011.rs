use core::{arch::asm, fmt};

const UART_DR: usize = 0x00;
const UART_FR: usize = 0x18;
const UART_IBRD: usize = 0x24;
const UART_FBRD: usize = 0x28;
const UART_LCRH: usize = 0x2c;
const UART_CR: usize = 0x30;
const UART_IMSC: usize = 0x38;
const UART_ICR: usize = 0x44;

const UART_FR_TXFF: u32 = 1 << 5;

#[derive(Clone, Copy)]
pub struct Pl011 {
    base: usize,
    flush_posted_writes: bool,
}

impl Pl011 {
    pub const fn new(base: usize) -> Self {
        Self {
            base,
            flush_posted_writes: false,
        }
    }

    #[allow(dead_code)]
    pub const fn new_with_posted_write_flush(base: usize) -> Self {
        Self {
            base,
            flush_posted_writes: true,
        }
    }

    pub fn init_early(self) {
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
        self.write_data(byte as u32);
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
        if self.flush_posted_writes {
            let _ = self.read_reg(offset);
        }
    }

    fn write_data(self, value: u32) {
        let addr = (self.base + UART_DR) as *mut u32;
        unsafe {
            asm!(
                "str {value:w}, [{addr}]",
                value = in(reg) value,
                addr = in(reg) addr,
                options(nostack, preserves_flags)
            );
        }
        if self.flush_posted_writes {
            let _ = self.read_reg(UART_FR);
        }
    }
}

impl fmt::Write for Pl011 {
    #[cfg_attr(
        all(talos_target_rpi5_bcm2712, talos_rpi5_rust_entry_diagnostic),
        allow(unreachable_code, unused_variables)
    )]
    fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
        #[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_rust_entry_diagnostic))]
        crate::rpi5_rust_entry_reset_probe();

        fmt::write(self, args)
    }

    #[cfg_attr(
        all(talos_target_rpi5_bcm2712, talos_rpi5_rust_entry_diagnostic),
        allow(unreachable_code, unused_variables)
    )]
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
