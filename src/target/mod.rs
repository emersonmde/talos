use crate::boot::BootInfo;

pub mod qemu;
pub mod qemu_virt;
pub mod rpi5;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TargetKind {
    QemuVirt,
    #[allow(dead_code)]
    Rpi5Bcm2712,
}

impl TargetKind {
    #[cfg(not(test))]
    pub const fn name(self) -> &'static str {
        match self {
            Self::QemuVirt => "talos-aarch64-virt",
            Self::Rpi5Bcm2712 => "talos-rpi5-bcm2712",
        }
    }
}

pub fn init(boot_info: &BootInfo) {
    match boot_info.target {
        TargetKind::QemuVirt => qemu_virt::init(),
        TargetKind::Rpi5Bcm2712 => rpi5::init_stub(),
    }
}

pub mod console {
    use core::fmt::{self, Write};

    pub fn _print(args: fmt::Arguments<'_>) {
        crate::target::qemu_virt::console()
            .write_fmt(args)
            .expect("serial console write failed");
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::target::console::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n");
    };
    ($fmt:expr) => {
        $crate::print!(concat!($fmt, "\n"));
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::print!(concat!($fmt, "\n"), $($arg)*);
    };
}
