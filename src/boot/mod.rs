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
            target: TargetKind::QemuVirt,
        }
    }
}
