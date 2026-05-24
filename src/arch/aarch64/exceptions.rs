use core::{
    arch::asm,
    sync::atomic::{AtomicU64, Ordering},
};

use crate::println;

#[derive(Clone, Copy)]
#[repr(u64)]
pub enum ExceptionVector {
    CurrentSp0Sync = 0,
    CurrentSp0Irq = 1,
    CurrentSp0Fiq = 2,
    CurrentSp0SError = 3,
    CurrentSpxSync = 4,
    CurrentSpxIrq = 5,
    CurrentSpxFiq = 6,
    CurrentSpxSError = 7,
    LowerAarch64Sync = 8,
    LowerAarch64Irq = 9,
    LowerAarch64Fiq = 10,
    LowerAarch64SError = 11,
    LowerAarch32Sync = 12,
    LowerAarch32Irq = 13,
    LowerAarch32Fiq = 14,
    LowerAarch32SError = 15,
}

impl ExceptionVector {
    #[cfg_attr(talos_target_rpi5_bcm2712, allow(dead_code))]
    pub const fn name(self) -> &'static str {
        match self {
            Self::CurrentSp0Sync => "current-sp0-sync",
            Self::CurrentSp0Irq => "current-sp0-irq",
            Self::CurrentSp0Fiq => "current-sp0-fiq",
            Self::CurrentSp0SError => "current-sp0-serror",
            Self::CurrentSpxSync => "current-spx-sync",
            Self::CurrentSpxIrq => "current-spx-irq",
            Self::CurrentSpxFiq => "current-spx-fiq",
            Self::CurrentSpxSError => "current-spx-serror",
            Self::LowerAarch64Sync => "lower-aarch64-sync",
            Self::LowerAarch64Irq => "lower-aarch64-irq",
            Self::LowerAarch64Fiq => "lower-aarch64-fiq",
            Self::LowerAarch64SError => "lower-aarch64-serror",
            Self::LowerAarch32Sync => "lower-aarch32-sync",
            Self::LowerAarch32Irq => "lower-aarch32-irq",
            Self::LowerAarch32Fiq => "lower-aarch32-fiq",
            Self::LowerAarch32SError => "lower-aarch32-serror",
        }
    }
}

impl From<u64> for ExceptionVector {
    fn from(value: u64) -> Self {
        match value {
            0 => Self::CurrentSp0Sync,
            1 => Self::CurrentSp0Irq,
            2 => Self::CurrentSp0Fiq,
            3 => Self::CurrentSp0SError,
            4 => Self::CurrentSpxSync,
            5 => Self::CurrentSpxIrq,
            6 => Self::CurrentSpxFiq,
            7 => Self::CurrentSpxSError,
            8 => Self::LowerAarch64Sync,
            9 => Self::LowerAarch64Irq,
            10 => Self::LowerAarch64Fiq,
            11 => Self::LowerAarch64SError,
            12 => Self::LowerAarch32Sync,
            13 => Self::LowerAarch32Irq,
            14 => Self::LowerAarch32Fiq,
            _ => Self::LowerAarch32SError,
        }
    }
}

impl ExceptionVector {
    #[allow(dead_code)]
    pub const fn is_irq(self) -> bool {
        matches!(
            self,
            Self::CurrentSp0Irq
                | Self::CurrentSpxIrq
                | Self::LowerAarch64Irq
                | Self::LowerAarch32Irq
        )
    }
}

unsafe extern "C" {
    static __exception_vectors: u8;
}

pub fn init() {
    let vectors = relocated_exception_vectors_addr();
    match crate::arch::aarch64::current_el() {
        1 => unsafe {
            asm!(
                "msr VBAR_EL1, {vectors}",
                "isb",
                vectors = in(reg) vectors,
                options(nostack, preserves_flags)
            );
        },
        2 => unsafe {
            asm!(
                "msr VBAR_EL2, {vectors}",
                "isb",
                vectors = in(reg) vectors,
                options(nostack, preserves_flags)
            );
        },
        3 => unsafe {
            asm!(
                "msr VBAR_EL3, {vectors}",
                "isb",
                vectors = in(reg) vectors,
                options(nostack, preserves_flags)
            );
        },
        _ => crate::arch::aarch64::halt(),
    }
}

fn relocated_exception_vectors_addr() -> usize {
    let vectors = core::ptr::addr_of!(__exception_vectors) as usize;

    #[cfg(talos_target_rpi5_bcm2712)]
    {
        crate::target::rpi5::relocate_early_linked_addr(vectors)
    }

    #[cfg(not(talos_target_rpi5_bcm2712))]
    {
        vectors
    }
}

#[repr(C)]
pub struct ExceptionFrame {
    regs: [u64; 31],
}

#[cfg_attr(not(talos_target_rpi5_bcm2712), allow(dead_code))]
impl ExceptionFrame {
    pub const REGISTER_COUNT: usize = 31;

    pub fn reg(&self, index: usize) -> u64 {
        self.regs[index]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(dead_code)]
pub struct IrqDispatchSnapshot {
    pub count: u64,
    pub vector: u64,
    pub elr: u64,
    pub spsr: u64,
}

static UNEXPECTED_IRQ_COUNT: AtomicU64 = AtomicU64::new(0);
static LAST_UNEXPECTED_IRQ_VECTOR: AtomicU64 = AtomicU64::new(0);
static LAST_UNEXPECTED_IRQ_ELR: AtomicU64 = AtomicU64::new(0);
static LAST_UNEXPECTED_IRQ_SPSR: AtomicU64 = AtomicU64::new(0);

#[allow(dead_code)]
pub fn unexpected_irq_snapshot() -> IrqDispatchSnapshot {
    IrqDispatchSnapshot {
        count: UNEXPECTED_IRQ_COUNT.load(Ordering::Relaxed),
        vector: LAST_UNEXPECTED_IRQ_VECTOR.load(Ordering::Relaxed),
        elr: LAST_UNEXPECTED_IRQ_ELR.load(Ordering::Relaxed),
        spsr: LAST_UNEXPECTED_IRQ_SPSR.load(Ordering::Relaxed),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_irq_handler(
    vector: u64,
    elr: u64,
    spsr: u64,
    _saved_frame: *const ExceptionFrame,
) {
    #[cfg(all(talos_target_qemu_virt, not(test)))]
    if crate::target::qemu_virt::handle_irq(vector) {
        return;
    }

    LAST_UNEXPECTED_IRQ_VECTOR.store(vector, Ordering::Relaxed);
    LAST_UNEXPECTED_IRQ_ELR.store(elr, Ordering::Relaxed);
    LAST_UNEXPECTED_IRQ_SPSR.store(spsr, Ordering::Relaxed);
    UNEXPECTED_IRQ_COUNT.fetch_add(1, Ordering::Relaxed);
}

#[unsafe(no_mangle)]
#[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_exception_return_diagnostic))]
pub extern "C" fn rust_exception_handler(
    esr: u64,
    elr: u64,
    far: u64,
    vector: u64,
    spsr: u64,
    saved_frame: *const ExceptionFrame,
) -> u64 {
    let vector = ExceptionVector::from(vector);

    println!();
    println!("talos exception: {}", vector.name());
    println!(
        "exception-info: esr={:#018x} elr={:#018x} far={:#018x}",
        esr, elr, far
    );
    write_exception_class(esr);
    write_exception_context(spsr, saved_frame);
    println!("talos exception: resume elr={:#018x}", elr.wrapping_add(4));

    if (esr >> 26) != 0x3c {
        println!("talos exception: resume rejected");
        crate::target::rpi5::wait_uart10_empty_early_phase();
        crate::arch::aarch64::halt()
    }

    crate::target::rpi5::wait_uart10_empty_early_phase();
    elr.wrapping_add(4)
}

#[unsafe(no_mangle)]
#[cfg(all(
    talos_target_rpi5_bcm2712,
    not(talos_rpi5_exception_report_diagnostic),
    not(talos_rpi5_exception_return_diagnostic)
))]
pub extern "C" fn rust_exception_handler(
    esr: u64,
    elr: u64,
    far: u64,
    vector: u64,
    spsr: u64,
    saved_frame: *const ExceptionFrame,
) -> ! {
    let vector = ExceptionVector::from(vector);

    println!();
    println!("talos exception: {}", vector.name());
    println!(
        "exception-info: esr={:#018x} elr={:#018x} far={:#018x}",
        esr, elr, far
    );
    write_exception_class(esr);
    write_exception_context(spsr, saved_frame);

    crate::target::rpi5::wait_uart10_empty_early_phase();
    crate::arch::aarch64::halt()
}

#[cfg(all(talos_target_rpi5_bcm2712, not(talos_rpi5_exception_report_diagnostic)))]
fn write_exception_class(esr: u64) {
    crate::target::console::write_static("exception-class: ");
    crate::target::console::write_static(exception_class_name(esr));
    crate::target::console::write_static(" ec=");
    crate::target::console::write_hex_u64(exception_class(esr));
    crate::target::console::write_static("\n");
}

#[cfg(all(talos_target_rpi5_bcm2712, not(talos_rpi5_exception_report_diagnostic)))]
fn exception_class(esr: u64) -> u64 {
    (esr >> 26) & 0x3f
}

#[cfg(all(talos_target_rpi5_bcm2712, not(talos_rpi5_exception_report_diagnostic)))]
fn exception_class_name(esr: u64) -> &'static str {
    let ec = exception_class(esr);
    if ec == 0x00 {
        "unknown-or-undefined-instruction"
    } else if ec == 0x20 {
        "instruction-abort-lower-el"
    } else if ec == 0x21 {
        "instruction-abort-same-el"
    } else if ec == 0x24 {
        "data-abort-lower-el"
    } else if ec == 0x25 {
        "data-abort-same-el"
    } else if ec == 0x2f {
        "serror"
    } else if ec == 0x3c {
        "brk-aarch64"
    } else {
        "unclassified"
    }
}

#[cfg(all(talos_target_rpi5_bcm2712, not(talos_rpi5_exception_report_diagnostic)))]
fn write_exception_context(spsr: u64, saved_frame: *const ExceptionFrame) {
    crate::target::console::write_static("exception-status: spsr=");
    crate::target::console::write_hex_u64(spsr);
    crate::target::console::write_static("\n");

    let Some(frame) = (unsafe { saved_frame.as_ref() }) else {
        crate::target::console::write_static("exception-regs: unavailable\n");
        return;
    };

    write_saved_register_line("exception-regs0:", frame, 0, 4);
    write_saved_register_line("exception-regs1:", frame, 4, 8);
    write_saved_register_line("exception-regs2:", frame, 8, 12);
    write_saved_register_line("exception-regs3:", frame, 12, 16);
    write_saved_register_line("exception-regs4:", frame, 16, 20);
    write_saved_register_line("exception-regs5:", frame, 20, 24);
    write_saved_register_line("exception-regs6:", frame, 24, 28);
    write_saved_register_line(
        "exception-regs7:",
        frame,
        28,
        ExceptionFrame::REGISTER_COUNT,
    );
}

#[cfg(all(talos_target_rpi5_bcm2712, not(talos_rpi5_exception_report_diagnostic)))]
fn write_saved_register_line(prefix: &str, frame: &ExceptionFrame, start: usize, end: usize) {
    crate::target::console::write_static(prefix);
    let mut index = start;
    while index < end {
        crate::target::console::write_static(" x");
        crate::target::console::write_dec_usize(index);
        crate::target::console::write_static("=");
        crate::target::console::write_hex_u64(frame.reg(index));
        index += 1;
    }
    crate::target::console::write_static("\n");
}

#[unsafe(no_mangle)]
#[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_exception_report_diagnostic))]
pub extern "C" fn rust_exception_handler(esr: u64, elr: u64, far: u64, vector: u64) -> ! {
    write_rpi5_exception_report(esr, elr, far, vector);
    crate::target::rpi5::wait_uart10_empty_early_phase();
    rpi5_system_reset();
}

#[unsafe(no_mangle)]
#[cfg(not(talos_target_rpi5_bcm2712))]
pub extern "C" fn rust_exception_handler(
    esr: u64,
    elr: u64,
    far: u64,
    vector: u64,
    _spsr: u64,
    _saved_frame: *const ExceptionFrame,
) -> ! {
    let vector = ExceptionVector::from(vector);

    println!();
    println!("talos exception: {}", vector.name());
    println!(
        "exception-info: esr={:#018x} elr={:#018x} far={:#018x}",
        esr, elr, far
    );

    crate::arch::aarch64::halt()
}

#[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_exception_report_diagnostic))]
fn write_rpi5_exception_report(esr: u64, elr: u64, far: u64, vector: u64) {
    crate::target::console::write_static("TALOS: rust-exception vector=");
    crate::target::console::write_hex_u64(vector);
    crate::target::console::write_static(" esr=");
    crate::target::console::write_hex_u64(esr);
    crate::target::console::write_static(" elr=");
    crate::target::console::write_hex_u64(elr);
    crate::target::console::write_static(" far=");
    crate::target::console::write_hex_u64(far);
    crate::target::console::write_static("\n");
}

#[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_exception_report_diagnostic))]
fn rpi5_system_reset() -> ! {
    unsafe {
        core::arch::asm!(
            "ldr x0, =0x84000009",
            "smc #0",
            "1:",
            "wfe",
            "b 1b",
            options(noreturn)
        );
    }
}

#[cfg(test)]
mod tests {
    use super::{ExceptionVector, rust_irq_handler, unexpected_irq_snapshot};

    #[test_case]
    fn irq_vector_classifier_names_irq_slots() {
        assert!(ExceptionVector::CurrentSp0Irq.is_irq());
        assert!(ExceptionVector::CurrentSpxIrq.is_irq());
        assert!(ExceptionVector::LowerAarch64Irq.is_irq());
        assert!(ExceptionVector::LowerAarch32Irq.is_irq());
        assert!(!ExceptionVector::CurrentSpxSync.is_irq());
        assert!(!ExceptionVector::CurrentSpxFiq.is_irq());
        assert!(!ExceptionVector::CurrentSpxSError.is_irq());
    }

    #[test_case]
    fn irq_dispatch_stub_counts_and_records_context_without_frame() {
        let before = unexpected_irq_snapshot().count;

        rust_irq_handler(
            ExceptionVector::CurrentSpxIrq as u64,
            0x1234_5678,
            0x2000_03c9,
            core::ptr::null(),
        );

        let after = unexpected_irq_snapshot();
        assert_eq!(after.count, before + 1);
        assert_eq!(after.vector, ExceptionVector::CurrentSpxIrq as u64);
        assert_eq!(after.elr, 0x1234_5678);
        assert_eq!(after.spsr, 0x2000_03c9);
    }
}
