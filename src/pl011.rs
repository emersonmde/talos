use core::{arch::asm, fmt};

const UART_DR: usize = 0x00;
const UART_FR: usize = 0x18;
const UART_IBRD: usize = 0x24;
const UART_FBRD: usize = 0x28;
const UART_LCRH: usize = 0x2c;
const UART_CR: usize = 0x30;
const UART_IMSC: usize = 0x38;
const UART_ICR: usize = 0x44;

const UART_FR_RXFE: u32 = 1 << 4;
const UART_FR_TXFF: u32 = 1 << 5;
const UART_FR_TXFE: u32 = 1 << 7;
const UART_TX_READY_WAIT_LIMIT: usize = 0x1_0000;
const UART_TX_EMPTY_WAIT_LIMIT: usize = 0x20_0000;

#[derive(Clone, Copy)]
pub struct Pl011 {
    base: usize,
    flush_posted_writes: bool,
    poll_tx_ready: bool,
    byte_data_writes: bool,
}

impl Pl011 {
    pub const fn new(base: usize) -> Self {
        Self {
            base,
            flush_posted_writes: false,
            poll_tx_ready: true,
            byte_data_writes: false,
        }
    }

    #[allow(dead_code)]
    pub const fn new_with_posted_write_flush(base: usize) -> Self {
        Self {
            base,
            flush_posted_writes: true,
            poll_tx_ready: true,
            byte_data_writes: false,
        }
    }

    #[allow(dead_code)]
    pub const fn new_with_posted_write_flush_unpolled(base: usize) -> Self {
        Self {
            base,
            flush_posted_writes: true,
            poll_tx_ready: false,
            byte_data_writes: false,
        }
    }

    #[allow(dead_code)]
    pub const fn new_with_byte_data_writes(base: usize) -> Self {
        Self {
            base,
            flush_posted_writes: true,
            poll_tx_ready: false,
            byte_data_writes: true,
        }
    }

    pub fn init_early(self) {
        self.init_early_with_divisors(1, 40);
    }

    #[allow(dead_code)]
    pub fn init_early_115200_9_216mhz(self) {
        self.init_early_with_divisors(5, 0);
    }

    fn init_early_with_divisors(self, ibrd: u32, fbrd: u32) {
        self.write_reg(UART_CR, 0);
        self.write_reg(UART_ICR, 0x7ff);
        self.write_reg(UART_IBRD, ibrd);
        self.write_reg(UART_FBRD, fbrd);
        self.write_reg(UART_LCRH, (0b11 << 5) | (1 << 4));
        self.write_reg(UART_IMSC, 0);
        self.write_reg(UART_CR, (1 << 0) | (1 << 8) | (1 << 9));
    }

    pub fn write_byte(self, byte: u8) {
        if self.poll_tx_ready {
            self.wait_tx_ready_bounded();
        }
        self.write_data(byte as u32);
    }

    #[cfg_attr(
        not(any(
            test,
            talos_boot_scenario = "qemu_polling_tty_rx",
            talos_boot_scenario = "qemu_local_serial_command_loop",
            talos_boot_scenario = "rpi5_uart10_polling_rx",
            talos_boot_scenario = "rpi5_diagnostic_command_channel",
            talos_boot_scenario = "rpi5_local_serial_command_loop",
            talos_boot_scenario = "rpi5_local_line_editing"
        )),
        allow(dead_code)
    )]
    pub fn poll_read_byte(self) -> Option<u8> {
        if self.read_reg(UART_FR) & UART_FR_RXFE != 0 {
            return None;
        }

        Some((self.read_reg(UART_DR) & 0xff) as u8)
    }

    fn wait_tx_ready_bounded(self) {
        let mut remaining = UART_TX_READY_WAIT_LIMIT;
        while self.read_reg(UART_FR) & UART_FR_TXFF != 0 {
            if remaining == 0 {
                break;
            }
            remaining -= 1;
        }
    }

    fn wait_tx_empty_bounded(self) {
        let mut remaining = UART_TX_EMPTY_WAIT_LIMIT;
        while self.read_reg(UART_FR) & UART_FR_TXFE == 0 {
            if remaining == 0 {
                break;
            }
            remaining -= 1;
        }

        unsafe {
            asm!("dsb sy", options(nostack, preserves_flags));
        }
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
        if self.byte_data_writes {
            let addr = (self.base + UART_DR) as *mut u8;
            unsafe {
                asm!(
                    "strb {value:w}, [{addr}]",
                    value = in(reg) value as u8,
                    addr = in(reg) addr,
                    options(nostack, preserves_flags)
                );
            }
        } else {
            let addr = (self.base + UART_DR) as *mut u32;
            unsafe {
                asm!(
                    "str {value:w}, [{addr}]",
                    value = in(reg) value,
                    addr = in(reg) addr,
                    options(nostack, preserves_flags)
                );
            }
        }
        if self.flush_posted_writes {
            let _ = self.read_reg(UART_FR);
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
        if self.flush_posted_writes {
            self.wait_tx_empty_bounded();
        }
        Ok(())
    }
}

impl crate::runtime_console::ConsoleInputBackend for Pl011 {
    fn poll_read_byte(&mut self) -> Option<u8> {
        (*self).poll_read_byte()
    }
}
