use crate::{boot::BootInfo, device_tree::DeviceTree, mmio::MmioMap};

pub mod qemu;
pub mod qemu_virt;
pub mod rpi5;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TargetKind {
    #[allow(dead_code)]
    QemuVirt,
    #[allow(dead_code)]
    Rpi5Bcm2712,
}

impl TargetKind {
    pub const fn name(self) -> &'static str {
        match self {
            Self::QemuVirt => "talos-aarch64-virt",
            Self::Rpi5Bcm2712 => "talos-rpi5-bcm2712",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UartKind {
    Pl011,
    FirmwarePreserved,
}

impl UartKind {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Pl011 => "pl011",
            Self::FirmwarePreserved => "firmware-preserved",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TimerKind {
    ArmGeneric,
}

impl TimerKind {
    pub const fn name(self) -> &'static str {
        match self {
            Self::ArmGeneric => "arm-generic",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InterruptControllerKind {
    GicV2,
}

impl InterruptControllerKind {
    pub const fn name(self) -> &'static str {
        match self {
            Self::GicV2 => "gic-v2",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TargetServices {
    pub uart: UartKind,
    pub timer: TimerKind,
    pub interrupt_controller: InterruptControllerKind,
    pub mmio_map: MmioMap,
    pub device_tree: DeviceTree,
}

pub fn services(boot_info: &BootInfo) -> TargetServices {
    match boot_info.target {
        TargetKind::QemuVirt => qemu_virt::services(boot_info),
        TargetKind::Rpi5Bcm2712 => rpi5::services(boot_info),
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

    #[cfg_attr(
        all(talos_target_rpi5_bcm2712, talos_rpi5_rust_entry_diagnostic),
        allow(unreachable_code, unused_variables)
    )]
    pub fn _print(args: fmt::Arguments<'_>) {
        #[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_rust_entry_diagnostic))]
        crate::rpi5_rust_entry_reset_probe();

        console()
            .write_fmt(args)
            .expect("serial console write failed");
    }

    #[cfg(talos_target_rpi5_bcm2712)]
    fn console() -> impl Write {
        crate::target::rpi5::firmware_console()
    }

    #[cfg(not(talos_target_rpi5_bcm2712))]
    fn console() -> impl Write {
        crate::target::qemu_virt::console()
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
