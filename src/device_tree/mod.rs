#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeviceTree {
    physical_address: Option<usize>,
}

mod chosen;
mod memory;
mod raw;

#[allow(unused_imports)]
pub use memory::{
    FdtMemoryBank, FdtMemoryBanks, FdtMemoryReservation, FdtMemoryReservations,
    FdtReservedMemoryRange, FdtReservedMemoryRanges,
};
#[allow(unused_imports)]
pub use raw::FdtHeader;

impl DeviceTree {
    pub const fn from_physical_address(physical_address: usize) -> Self {
        Self {
            physical_address: Some(physical_address),
        }
    }

    pub const fn physical_address(self) -> Option<usize> {
        self.physical_address
    }
}
