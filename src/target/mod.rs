use crate::{boot::BootInfo, device_tree::DeviceTree, mmio::MmioMap};

#[cfg_attr(talos_target_rpi5_bcm2712, allow(dead_code))]
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
    use crate::early_format;
    use core::fmt::{self, Write};

    pub fn _print(args: fmt::Arguments<'_>) {
        crate::runtime_console::write_kernel_output(runtime_backend(), args)
            .expect("serial console write failed");
    }

    #[allow(dead_code)]
    pub fn write_static(s: &str) {
        #[cfg(talos_target_rpi5_bcm2712)]
        {
            crate::target::rpi5::write_early_static(s);
        }

        #[cfg(not(talos_target_rpi5_bcm2712))]
        runtime_backend()
            .write_str(s)
            .expect("serial console write failed");
    }

    #[allow(dead_code)]
    pub fn write_hex_usize(value: usize) {
        #[cfg(talos_target_rpi5_bcm2712)]
        {
            crate::target::rpi5::write_early_hex_u64(value as u64);
        }

        #[cfg(not(talos_target_rpi5_bcm2712))]
        early_format::write_hex_usize(runtime_backend(), value)
            .expect("serial console write failed");
    }

    #[allow(dead_code)]
    pub fn write_hex_u64(value: u64) {
        #[cfg(talos_target_rpi5_bcm2712)]
        {
            crate::target::rpi5::write_early_hex_u64(value);
        }

        #[cfg(not(talos_target_rpi5_bcm2712))]
        early_format::write_hex_u64(runtime_backend(), value).expect("serial console write failed");
    }

    #[allow(dead_code)]
    pub fn write_dec_usize(value: usize) {
        early_format::write_dec_usize(runtime_backend(), value)
            .expect("serial console write failed");
    }

    #[cfg(talos_target_rpi5_bcm2712)]
    fn runtime_backend() -> impl Write {
        crate::target::rpi5::firmware_console()
    }

    #[cfg(not(talos_target_rpi5_bcm2712))]
    fn runtime_backend() -> impl Write {
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
    ($($arg:tt)*) => {
        $crate::print!("{}\n", format_args!($($arg)*));
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn target_services_include_qemu_console() {
        let boot_info = crate::boot::BootInfo::from_aarch64_x0(0x4000_0000);
        let services = services(&boot_info);

        assert_eq!(services.uart, UartKind::Pl011);
        assert_eq!(services.uart.name(), "pl011");
        assert_eq!(services.timer.name(), "arm-generic");
        assert_eq!(services.interrupt_controller.name(), "gic-v2");
        let regions = services.mmio_map.regions();
        assert_eq!(regions.len(), 3);
        assert_eq!(regions[0].name, "qemu-virt-gicv2-distributor");
        assert_eq!(regions[1].name, "qemu-virt-gicv2-cpu-interface");
        assert_eq!(regions[2].name, "qemu-virt-pl011-uart0");
        assert!(services.device_tree.physical_address().is_some());
        assert_eq!(services.device_tree.physical_address(), Some(0x4000_0000));
        assert_eq!(boot_info.target.name(), "talos-aarch64-virt");
    }
}
