use core::arch::asm;

#[repr(u64)]
pub enum ExitCode {
    Success = 0,
    #[cfg(test)]
    Failure = 1,
}

pub fn exit_success() -> ! {
    exit(ExitCode::Success)
}

#[cfg(test)]
pub fn exit_failure() -> ! {
    exit(ExitCode::Failure)
}

fn exit(code: ExitCode) -> ! {
    #[repr(C)]
    struct ExitBlock {
        reason: u64,
        status: u64,
    }

    let block = ExitBlock {
        reason: 0x20026,
        status: code as u64,
    };

    unsafe {
        asm!(
            "mov w0, #0x18",
            "mov x1, {block}",
            "hlt #0xf000",
            block = in(reg) &block,
            options(noreturn)
        );
    }
}
