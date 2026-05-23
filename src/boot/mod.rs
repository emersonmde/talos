#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
pub(crate) mod rpi5;
#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
pub(crate) mod rpi5_reports;

use crate::{arch, target::TargetKind};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BootInfo {
    pub dtb_pa: usize,
    pub primary_core: u64,
    pub exception_level: u8,
    pub target: TargetKind,
}

impl BootInfo {
    pub fn from_aarch64_x0(dtb_pa: usize) -> Self {
        Self {
            dtb_pa,
            primary_core: 0,
            exception_level: arch::aarch64::current_el(),
            target: active_target(),
        }
    }
}

const fn active_target() -> TargetKind {
    #[cfg(talos_target_rpi5_bcm2712)]
    {
        TargetKind::Rpi5Bcm2712
    }

    #[cfg(not(talos_target_rpi5_bcm2712))]
    {
        TargetKind::QemuVirt
    }
}
