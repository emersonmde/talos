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
    // serial10 is already active for firmware/BL31 logs; avoid disturbing baud
    // while testing Talos' runtime console path.
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
    Pl011::new_with_posted_write_flush(UART10_BASE)
}

#[cfg(talos_target_rpi5_bcm2712)]
pub enum EarlyPhaseLine {
    RustEntry,
    BootInfoParsed,
    TargetInit,
    ExceptionsReady,
    KernelMain,
    DtbReservationsStart,
    DtbReservationsDone,
    DtbMemoryScanStart,
    DtbMemoryScanDone,
    MmuEnableStart,
    MmuEnableDone,
    IcacheEnableStart,
    IcacheEnableDone,
}

#[cfg(talos_target_rpi5_bcm2712)]
pub fn write_early_phase_line(line: EarlyPhaseLine) {
    write_uart10_byte_early_phase(b'T');
    write_uart10_byte_early_phase(b'A');
    write_uart10_byte_early_phase(b'L');
    write_uart10_byte_early_phase(b'O');
    write_uart10_byte_early_phase(b'S');
    write_uart10_byte_early_phase(b':');
    write_uart10_byte_early_phase(b' ');

    match line {
        EarlyPhaseLine::RustEntry => {
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b'u');
            write_uart10_byte_early_phase(b's');
            write_uart10_byte_early_phase(b't');
            write_uart10_byte_early_phase(b'_');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b't');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b'y');
        }
        EarlyPhaseLine::BootInfoParsed => {
            write_uart10_byte_early_phase(b'b');
            write_uart10_byte_early_phase(b'o');
            write_uart10_byte_early_phase(b'o');
            write_uart10_byte_early_phase(b't');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'i');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b'f');
            write_uart10_byte_early_phase(b'o');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'p');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b's');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'd');
        }
        EarlyPhaseLine::TargetInit => {
            write_uart10_byte_early_phase(b't');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b'g');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b't');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'i');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b'i');
            write_uart10_byte_early_phase(b't');
        }
        EarlyPhaseLine::ExceptionsReady => {
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'x');
            write_uart10_byte_early_phase(b'c');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'p');
            write_uart10_byte_early_phase(b't');
            write_uart10_byte_early_phase(b'i');
            write_uart10_byte_early_phase(b'o');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b's');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'd');
            write_uart10_byte_early_phase(b'y');
        }
        EarlyPhaseLine::KernelMain => {
            write_uart10_byte_early_phase(b'k');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'l');
            write_uart10_byte_early_phase(b'_');
            write_uart10_byte_early_phase(b'm');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'i');
            write_uart10_byte_early_phase(b'n');
        }
        EarlyPhaseLine::DtbReservationsStart => {
            write_uart10_byte_early_phase(b'd');
            write_uart10_byte_early_phase(b't');
            write_uart10_byte_early_phase(b'b');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b's');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b'v');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'd');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b's');
            write_uart10_byte_early_phase(b't');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b't');
        }
        EarlyPhaseLine::DtbReservationsDone => {
            write_uart10_byte_early_phase(b'd');
            write_uart10_byte_early_phase(b't');
            write_uart10_byte_early_phase(b'b');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b's');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b'v');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'd');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'd');
            write_uart10_byte_early_phase(b'o');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b'e');
        }
        EarlyPhaseLine::DtbMemoryScanStart => {
            write_uart10_byte_early_phase(b'd');
            write_uart10_byte_early_phase(b't');
            write_uart10_byte_early_phase(b'b');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'm');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'm');
            write_uart10_byte_early_phase(b'o');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b'y');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b's');
            write_uart10_byte_early_phase(b'c');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b's');
            write_uart10_byte_early_phase(b't');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b't');
        }
        EarlyPhaseLine::DtbMemoryScanDone => {
            write_uart10_byte_early_phase(b'd');
            write_uart10_byte_early_phase(b't');
            write_uart10_byte_early_phase(b'b');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'm');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'm');
            write_uart10_byte_early_phase(b'o');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b'y');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b's');
            write_uart10_byte_early_phase(b'c');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'd');
            write_uart10_byte_early_phase(b'o');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b'e');
        }
        EarlyPhaseLine::MmuEnableStart => {
            write_uart10_byte_early_phase(b'm');
            write_uart10_byte_early_phase(b'm');
            write_uart10_byte_early_phase(b'u');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'b');
            write_uart10_byte_early_phase(b'l');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b's');
            write_uart10_byte_early_phase(b't');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b't');
        }
        EarlyPhaseLine::MmuEnableDone => {
            write_uart10_byte_early_phase(b'm');
            write_uart10_byte_early_phase(b'm');
            write_uart10_byte_early_phase(b'u');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'b');
            write_uart10_byte_early_phase(b'l');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'd');
            write_uart10_byte_early_phase(b'o');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b'e');
        }
        EarlyPhaseLine::IcacheEnableStart => {
            write_uart10_byte_early_phase(b'i');
            write_uart10_byte_early_phase(b'c');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'c');
            write_uart10_byte_early_phase(b'h');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'b');
            write_uart10_byte_early_phase(b'l');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b's');
            write_uart10_byte_early_phase(b't');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b't');
        }
        EarlyPhaseLine::IcacheEnableDone => {
            write_uart10_byte_early_phase(b'i');
            write_uart10_byte_early_phase(b'c');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'c');
            write_uart10_byte_early_phase(b'h');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'b');
            write_uart10_byte_early_phase(b'l');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'd');
            write_uart10_byte_early_phase(b'o');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b'e');
        }
    }

    write_uart10_byte_early_phase(b'\r');
    write_uart10_byte_early_phase(b'\n');
    wait_uart10_empty_early_phase();
}

#[cfg(talos_target_rpi5_bcm2712)]
pub fn write_early_hex_u64(value: u64) {
    write_uart10_byte_early_phase(b'0');
    write_uart10_byte_early_phase(b'x');

    let mut started = false;
    let mut shift = u64::BITS;
    while shift != 0 {
        shift -= 4;
        let nibble = ((value >> shift) & 0xf) as u8;
        if nibble != 0 || started || shift == 0 {
            started = true;
            write_early_hex_digit(nibble);
        }
    }

    wait_uart10_empty_early_phase();
}

#[cfg(talos_target_rpi5_bcm2712)]
pub fn write_early_static(s: &str) {
    for byte in s.bytes() {
        if byte == b'\n' {
            write_uart10_byte_early_phase(b'\r');
            wait_uart10_empty_early_phase();
        }
        write_uart10_byte_early_phase(byte);
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_target_rpi5_bcm2712)]
fn write_early_hex_digit(nibble: u8) {
    let digit = if nibble < 10 {
        b'0'.wrapping_add(nibble)
    } else if nibble < 16 {
        b'a'.wrapping_add(nibble.wrapping_sub(10))
    } else {
        b'?'
    };
    write_uart10_byte_early_phase(digit);
}

#[cfg(talos_target_rpi5_bcm2712)]
#[inline(always)]
pub(crate) fn write_uart10_byte_early_phase(byte: u8) {
    let value = byte as u32;
    unsafe {
        core::arch::asm!(
            "mov x9, #0x1000",
            "movk x9, #0x7d00, lsl #16",
            "movk x9, #0x10, lsl #32",
            "str w11, [x9]",
            "ldr w10, [x9, #0x18]",
            "dsb sy",
            in("w11") value,
            lateout("x9") _,
            lateout("x10") _,
            options(nostack, preserves_flags)
        );
    }
}

#[cfg(talos_target_rpi5_bcm2712)]
pub(crate) fn wait_uart10_empty_early_phase() {
    unsafe {
        core::arch::asm!(
            "mov x9, #0x1000",
            "movk x9, #0x7d00, lsl #16",
            "movk x9, #0x10, lsl #32",
            "mov x21, #0x200000",
            "2:",
            "ldr w10, [x9, #0x18]",
            "tbnz w10, #7, 3f",
            "subs x21, x21, #1",
            "b.ne 2b",
            "3:",
            "dsb sy",
            lateout("x9") _,
            lateout("x10") _,
            lateout("x21") _,
            options(nostack)
        );
    }
}

#[cfg(talos_target_rpi5_bcm2712)]
pub(crate) fn relocate_early_linked_addr(addr: usize) -> usize {
    // The accepted normal Pi 5 Image links and runs at 0x200000, so this is
    // normally a no-op. Keep the helper for vector installation and explicit
    // address-contract diagnostics while that part of bring-up is still active.
    addr.wrapping_add(runtime_relocation_delta())
}

#[cfg(talos_target_rpi5_bcm2712)]
pub(crate) fn runtime_relocation_delta() -> usize {
    let mut runtime_pc: usize;
    let mut linked_pc: usize;
    unsafe {
        core::arch::asm!(
            "adr {runtime}, 1f",
            "ldr {linked}, =1f",
            "1:",
            runtime = out(reg) runtime_pc,
            linked = out(reg) linked_pc,
            options(nostack, preserves_flags)
        );
    }
    runtime_pc.wrapping_sub(linked_pc)
}

#[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_runtime_uart_probe_diagnostic))]
pub fn runtime_uart_probe_diagnostic() -> ! {
    const MARKERS: &[(usize, bool, &[u8])] = &[
        (UART10_BASE, true, b"TALOS UART10B\r\n"),
        (UART10_BASE, false, b"TALOS UART10W\r\n"),
        (RP1_UART0_FIRMWARE_BASE, true, b"TALOS RP1FW\r\n"),
        (RP1_UART0_PCIE2_BASE, true, b"TALOS RP1PCIE\r\n"),
    ];

    for _ in 0..16 {
        write_uart_marker_no_readback(UART10_BASE, b"TALOS EARLY\r\n");
        delay();
    }

    for _ in 0..8 {
        for &(base, byte_write, marker) in MARKERS {
            write_uart_marker(base, byte_write, marker);
        }
        delay();
    }

    unsafe {
        core::arch::asm!(
            "ldr x0, =0x84000009",
            "smc #0",
            "wfe",
            "b .-4",
            options(noreturn)
        );
    }
}

#[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_runtime_uart_probe_diagnostic))]
fn write_uart_marker_no_readback(base: usize, marker: &[u8]) {
    for &byte in marker {
        unsafe {
            core::ptr::write_volatile((base + 0) as *mut u8, byte);
        }
    }
    unsafe {
        core::arch::asm!("dsb sy", options(nostack, preserves_flags));
    }
}

#[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_runtime_uart_probe_diagnostic))]
fn write_uart_marker(base: usize, byte_write: bool, marker: &[u8]) {
    for &byte in marker {
        if byte_write {
            unsafe {
                core::ptr::write_volatile((base + 0) as *mut u8, byte);
            }
        } else {
            unsafe {
                core::ptr::write_volatile((base + 0) as *mut u32, byte as u32);
            }
        }

        unsafe {
            let _ = core::ptr::read_volatile((base + 0x18) as *const u32);
            core::arch::asm!("dsb sy", options(nostack, preserves_flags));
        }
    }
}

#[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_runtime_uart_probe_diagnostic))]
fn delay() {
    for _ in 0..0x8000 {
        core::hint::spin_loop();
    }
}

#[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_handoff_uart_diagnostic))]
pub fn handoff_uart_diagnostic() -> ! {
    unsafe extern "C" {
        fn rpi5_rust_entry_re_marker_park() -> !;
    }

    unsafe {
        rpi5_rust_entry_re_marker_park();
    }
}

#[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_rust_uart10_diagnostic))]
pub fn rust_uart10_diagnostic() -> ! {
    loop {
        write_uart10_byte(b'T');
        write_uart10_byte(b'A');
        write_uart10_byte(b'L');
        write_uart10_byte(b'O');
        write_uart10_byte(b'S');
        write_uart10_byte(b':');
        write_uart10_byte(b' ');
        write_uart10_byte(b'r');
        write_uart10_byte(b'u');
        write_uart10_byte(b's');
        write_uart10_byte(b't');
        write_uart10_byte(b'-');
        write_uart10_byte(b'u');
        write_uart10_byte(b'a');
        write_uart10_byte(b'r');
        write_uart10_byte(b't');
        write_uart10_byte(b'1');
        write_uart10_byte(b'0');
        write_uart10_byte(b'\r');
        write_uart10_byte(b'\n');
        delay();
    }
}

#[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_rust_uart10_diagnostic))]
#[inline(always)]
fn write_uart10_byte(byte: u8) {
    let value = byte as u32;
    unsafe {
        core::arch::asm!(
            "mov x9, #0x1000",
            "movk x9, #0x7d00, lsl #16",
            "movk x9, #0x10, lsl #32",
            "str w11, [x9]",
            "ldr w10, [x9, #0x18]",
            "dsb sy",
            in("w11") value,
            lateout("x9") _,
            lateout("x10") _,
            options(nostack, preserves_flags)
        );
    }
}

#[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_rust_uart10_diagnostic))]
fn delay() {
    unsafe {
        core::arch::asm!(
            "mov x9, #0x8000",
            "2:",
            "subs x9, x9, #1",
            "b.ne 2b",
            lateout("x9") _,
            options(nostack)
        );
    }
}

#[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_boundary_entry_reset_diagnostic))]
pub fn boundary_entry_reset_diagnostic() -> ! {
    unsafe {
        core::arch::asm!(
            "mov x9, #0x1000",
            "movk x9, #0x7d00, lsl #16",
            "movk x9, #0x10, lsl #32",
            "mov x20, #64",
            "2:",
            "mov w11, #0x54",
            "str w11, [x9]",
            "ldr w10, [x9, #0x18]",
            "mov w11, #0x41",
            "str w11, [x9]",
            "ldr w10, [x9, #0x18]",
            "mov w11, #0x4c",
            "str w11, [x9]",
            "ldr w10, [x9, #0x18]",
            "mov w11, #0x4f",
            "str w11, [x9]",
            "ldr w10, [x9, #0x18]",
            "mov w11, #0x53",
            "str w11, [x9]",
            "ldr w10, [x9, #0x18]",
            "mov w11, #0x3a",
            "str w11, [x9]",
            "ldr w10, [x9, #0x18]",
            "mov w11, #0x20",
            "str w11, [x9]",
            "ldr w10, [x9, #0x18]",
            "mov w11, #0x62",
            "str w11, [x9]",
            "ldr w10, [x9, #0x18]",
            "mov w11, #0x6f",
            "str w11, [x9]",
            "ldr w10, [x9, #0x18]",
            "mov w11, #0x75",
            "str w11, [x9]",
            "ldr w10, [x9, #0x18]",
            "mov w11, #0x6e",
            "str w11, [x9]",
            "ldr w10, [x9, #0x18]",
            "mov w11, #0x64",
            "str w11, [x9]",
            "ldr w10, [x9, #0x18]",
            "mov w11, #0x61",
            "str w11, [x9]",
            "ldr w10, [x9, #0x18]",
            "mov w11, #0x72",
            "str w11, [x9]",
            "ldr w10, [x9, #0x18]",
            "mov w11, #0x79",
            "str w11, [x9]",
            "ldr w10, [x9, #0x18]",
            "mov w11, #0x2d",
            "str w11, [x9]",
            "ldr w10, [x9, #0x18]",
            "mov w11, #0x65",
            "str w11, [x9]",
            "ldr w10, [x9, #0x18]",
            "mov w11, #0x6e",
            "str w11, [x9]",
            "ldr w10, [x9, #0x18]",
            "mov w11, #0x74",
            "str w11, [x9]",
            "ldr w10, [x9, #0x18]",
            "mov w11, #0x72",
            "str w11, [x9]",
            "ldr w10, [x9, #0x18]",
            "mov w11, #0x79",
            "str w11, [x9]",
            "ldr w10, [x9, #0x18]",
            "mov w11, #0x0d",
            "str w11, [x9]",
            "ldr w10, [x9, #0x18]",
            "mov w11, #0x0a",
            "str w11, [x9]",
            "ldr w10, [x9, #0x18]",
            "dsb sy",
            "subs x20, x20, #1",
            "b.ne 2b",
            "ldr x0, =0x84000009",
            "smc #0",
            "wfe",
            "b .-4",
            options(noreturn)
        );
    }
}

#[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_phase_ladder_diagnostic))]
pub fn phase_ladder_diagnostic() -> ! {
    use core::fmt::Write;

    static STATIC_LINE: &[u8] = b"TALOS: P4 static-line\r\n";

    loop {
        write_phase_line_immediate(PhaseLine::P3Raw);

        write_phase_line_immediate(PhaseLine::P4BeforeStatic);
        for &byte in STATIC_LINE {
            write_uart10_byte_phase(byte);
        }
        write_phase_line_immediate(PhaseLine::P4AfterStatic);

        write_phase_line_immediate(PhaseLine::P5BeforeWriteStr);
        let mut console = firmware_console();
        console
            .write_str("TALOS: P5 write-str\n")
            .expect("phase ladder write_str failed");
        write_phase_line_immediate(PhaseLine::P5AfterWriteStr);

        write_phase_line_immediate(PhaseLine::P6BeforeWriteFmt);
        let mut console = firmware_console();
        console
            .write_fmt(format_args!("TALOS: P6 write-fmt {}\n", 7usize))
            .expect("phase ladder write_fmt failed");
        write_phase_line_immediate(PhaseLine::P6AfterWriteFmt);

        write_phase_line_immediate(PhaseLine::P7BeforePrintln);
        crate::println!("TALOS: P7 println {}", 9usize);
        write_phase_line_immediate(PhaseLine::P7AfterPrintln);

        delay_phase();
    }
}

#[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_phase_ladder_diagnostic))]
enum PhaseLine {
    P3Raw,
    P4BeforeStatic,
    P4AfterStatic,
    P5BeforeWriteStr,
    P5AfterWriteStr,
    P6BeforeWriteFmt,
    P6AfterWriteFmt,
    P7BeforePrintln,
    P7AfterPrintln,
}

#[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_phase_ladder_diagnostic))]
fn write_phase_line_immediate(line: PhaseLine) {
    write_uart10_byte_phase(b'T');
    write_uart10_byte_phase(b'A');
    write_uart10_byte_phase(b'L');
    write_uart10_byte_phase(b'O');
    write_uart10_byte_phase(b'S');
    write_uart10_byte_phase(b':');
    write_uart10_byte_phase(b' ');
    match line {
        PhaseLine::P3Raw => write_phase_suffix(b'3', b'r', b'a', b'w', 0),
        PhaseLine::P4BeforeStatic => write_phase_suffix(b'4', b'b', b's', b't', b'a'),
        PhaseLine::P4AfterStatic => write_phase_suffix(b'4', b'a', b's', b't', b'a'),
        PhaseLine::P5BeforeWriteStr => write_phase_suffix(b'5', b'b', b'w', b's', 0),
        PhaseLine::P5AfterWriteStr => write_phase_suffix(b'5', b'a', b'w', b's', 0),
        PhaseLine::P6BeforeWriteFmt => write_phase_suffix(b'6', b'b', b'w', b'f', 0),
        PhaseLine::P6AfterWriteFmt => write_phase_suffix(b'6', b'a', b'w', b'f', 0),
        PhaseLine::P7BeforePrintln => write_phase_suffix(b'7', b'b', b'p', b'l', 0),
        PhaseLine::P7AfterPrintln => write_phase_suffix(b'7', b'a', b'p', b'l', 0),
    }
    write_uart10_byte_phase(b'\r');
    write_uart10_byte_phase(b'\n');
}

#[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_phase_ladder_diagnostic))]
fn write_phase_suffix(phase: u8, a: u8, b: u8, c: u8, d: u8) {
    write_uart10_byte_phase(b'P');
    write_uart10_byte_phase(phase);
    write_uart10_byte_phase(b' ');
    write_uart10_byte_phase(a);
    write_uart10_byte_phase(b);
    write_uart10_byte_phase(c);
    if d != 0 {
        write_uart10_byte_phase(d);
    }
}

#[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_phase_ladder_diagnostic))]
#[inline(always)]
fn write_uart10_byte_phase(byte: u8) {
    let value = byte as u32;
    unsafe {
        core::arch::asm!(
            "mov x9, #0x1000",
            "movk x9, #0x7d00, lsl #16",
            "movk x9, #0x10, lsl #32",
            "str w11, [x9]",
            "ldr w10, [x9, #0x18]",
            "dsb sy",
            in("w11") value,
            lateout("x9") _,
            lateout("x10") _,
            options(nostack, preserves_flags)
        );
    }
}

#[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_phase_ladder_diagnostic))]
fn delay_phase() {
    unsafe {
        core::arch::asm!(
            "mov x9, #0x8000",
            "2:",
            "subs x9, x9, #1",
            "b.ne 2b",
            lateout("x9") _,
            options(nostack)
        );
    }
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
