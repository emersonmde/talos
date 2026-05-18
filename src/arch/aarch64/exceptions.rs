use core::arch::asm;

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

unsafe extern "C" {
    static __exception_vectors: u8;
}

pub fn init() {
    let vectors = core::ptr::addr_of!(__exception_vectors) as usize;
    unsafe {
        asm!(
            "msr VBAR_EL1, {vectors}",
            "isb",
            vectors = in(reg) vectors,
            options(nostack, preserves_flags)
        );
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_exception_handler(esr: u64, elr: u64, far: u64, vector: u64) -> ! {
    let vector = ExceptionVector::from(vector);
    println!();
    println!("talos exception: {}", vector.name());
    println!(
        "exception-info: esr={:#018x} elr={:#018x} far={:#018x}",
        esr, elr, far
    );

    crate::arch::aarch64::halt()
}
