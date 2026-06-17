use crate::device_tree::{
    FdtInitrdRange, FdtMemoryBanks, FdtMemoryReservations, FdtReservedMemoryRanges,
};

use super::common::{align_down, align_up, contains_address, reserve_after};

pub const EARLY_USABLE_ALIGNMENT: u64 = 0x1000;
pub const EARLY_USABLE_POLICY: &str = "low-tail";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KernelLayout {
    pub start: u64,
    pub end: u64,
    pub heap_start: u64,
    pub heap_end: u64,
    pub stack_bottom: u64,
    pub stack_top: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FdtBlobRange {
    pub address: u64,
    pub size: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EarlyUsableMemory {
    pub bank_index: usize,
    pub start: u64,
    pub end: u64,
    pub size: u64,
    pub alignment: u64,
}

pub fn conservative_low_memory_candidate(
    banks: &FdtMemoryBanks,
    reservations: Option<&FdtMemoryReservations>,
    reserved_memory: Option<&FdtReservedMemoryRanges>,
    dtb: Option<FdtBlobRange>,
    firmware_initrd: Option<FdtInitrdRange>,
    kernel: KernelLayout,
) -> Option<EarlyUsableMemory> {
    let bank_count = banks.reported_len();
    let mut bank_index = 0usize;

    while bank_index < bank_count {
        let bank = banks.entries[bank_index]?;
        let bank_end = bank.address.checked_add(bank.size)?;
        if contains_address(bank.address, bank_end, kernel.start) {
            let mut candidate_start = bank.address;
            candidate_start = reserve_after(
                candidate_start,
                bank.address,
                bank_end,
                kernel.start,
                kernel.end,
            )?;

            if let Some(dtb) = dtb {
                let dtb_end = dtb.address.checked_add(dtb.size)?;
                candidate_start = reserve_after(
                    candidate_start,
                    bank.address,
                    bank_end,
                    dtb.address,
                    dtb_end,
                )?;
            }

            if let Some(initrd) = firmware_initrd {
                let initrd_start = align_down(initrd.start, EARLY_USABLE_ALIGNMENT);
                let initrd_end = align_up(initrd.end, EARLY_USABLE_ALIGNMENT)?;
                candidate_start = reserve_after(
                    candidate_start,
                    bank.address,
                    bank_end,
                    initrd_start,
                    initrd_end,
                )?;
            }

            if let Some(reservations) = reservations {
                let mut reservation_index = 0usize;
                let reservation_count = reservations.reported_len();
                while reservation_index < reservation_count {
                    let reservation = reservations.entries[reservation_index]?;
                    let reservation_end = reservation.address.checked_add(reservation.size)?;
                    candidate_start = reserve_after(
                        candidate_start,
                        bank.address,
                        bank_end,
                        reservation.address,
                        reservation_end,
                    )?;
                    reservation_index += 1;
                }
            }

            if let Some(reserved_memory) = reserved_memory {
                let mut range_index = 0usize;
                let range_count = reserved_memory.reported_len();
                while range_index < range_count {
                    let range = reserved_memory.entries[range_index]?;
                    let range_end = range.address.checked_add(range.size)?;
                    candidate_start = reserve_after(
                        candidate_start,
                        bank.address,
                        bank_end,
                        range.address,
                        range_end,
                    )?;
                    range_index += 1;
                }
            }

            candidate_start = align_up(candidate_start, EARLY_USABLE_ALIGNMENT)?;
            let candidate_end = align_down(bank_end, EARLY_USABLE_ALIGNMENT);
            if candidate_start < candidate_end {
                return Some(EarlyUsableMemory {
                    bank_index,
                    start: candidate_start,
                    end: candidate_end,
                    size: candidate_end - candidate_start,
                    alignment: EARLY_USABLE_ALIGNMENT,
                });
            }

            return None;
        }

        bank_index += 1;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device_tree::{
        FdtMemoryBank, FdtMemoryBanks, FdtMemoryReservation, FdtMemoryReservations,
        FdtReservedMemoryRange, FdtReservedMemoryRanges,
    };

    #[test_case]
    fn conservative_candidate_uses_tail_after_kernel_and_dtb() {
        assert_eq!(EARLY_USABLE_POLICY, "low-tail");

        let banks = FdtMemoryBanks {
            address_cells: 2,
            size_cells: 2,
            count: 2,
            entries: [
                Some(FdtMemoryBank {
                    address: 0,
                    size: 0x4000_0000,
                }),
                Some(FdtMemoryBank {
                    address: 0x1_0000_0000,
                    size: 0x4000_0000,
                }),
                None,
                None,
            ],
            truncated: false,
        };
        let kernel = KernelLayout {
            start: 0x20_0000,
            end: 0x36_1234,
            heap_start: 0x30_0000,
            heap_end: 0x34_0000,
            stack_bottom: 0x34_0000,
            stack_top: 0x36_1234,
        };
        let dtb = FdtBlobRange {
            address: 0x2efe_c600,
            size: 0x1f000,
        };

        let candidate =
            conservative_low_memory_candidate(&banks, None, None, Some(dtb), None, kernel)
                .expect("candidate");

        assert_eq!(candidate.bank_index, 0);
        assert_eq!(candidate.start, 0x2f00_c000);
        assert_eq!(candidate.end, 0x4000_0000);
        assert_eq!(candidate.size, 0x10ff_4000);
        assert_eq!(candidate.alignment, EARLY_USABLE_ALIGNMENT);
    }

    #[test_case]
    fn conservative_candidate_respects_reported_reservation_tail() {
        let banks = FdtMemoryBanks {
            address_cells: 2,
            size_cells: 2,
            count: 1,
            entries: [
                Some(FdtMemoryBank {
                    address: 0,
                    size: 0x1000_0000,
                }),
                None,
                None,
                None,
            ],
            truncated: false,
        };
        let reservations = FdtMemoryReservations {
            count: 1,
            entries: [
                Some(FdtMemoryReservation {
                    address: 0x0800_1000,
                    size: 0x1234,
                }),
                None,
                None,
                None,
            ],
            truncated: false,
        };
        let kernel = KernelLayout {
            start: 0x20_0000,
            end: 0x30_0000,
            heap_start: 0x30_0000,
            heap_end: 0x40_0000,
            stack_bottom: 0x40_0000,
            stack_top: 0x44_0000,
        };

        let candidate = conservative_low_memory_candidate(
            &banks,
            Some(&reservations),
            None,
            None,
            None,
            kernel,
        )
        .expect("candidate");

        assert_eq!(candidate.start, 0x0800_3000);
        assert_eq!(candidate.end, 0x1000_0000);
    }

    #[test_case]
    fn conservative_candidate_respects_reported_reserved_memory_ranges() {
        let banks = FdtMemoryBanks {
            address_cells: 2,
            size_cells: 2,
            count: 1,
            entries: [
                Some(FdtMemoryBank {
                    address: 0,
                    size: 0x4000_0000,
                }),
                None,
                None,
                None,
            ],
            truncated: false,
        };
        let reserved_memory = FdtReservedMemoryRanges {
            address_cells: 2,
            size_cells: 2,
            node_count: 2,
            range_count: 2,
            entries: [
                Some(FdtReservedMemoryRange {
                    address: 0x3fd2_3160,
                    size: 0x3d,
                    no_map: true,
                    reusable: false,
                }),
                Some(FdtReservedMemoryRange {
                    address: 0,
                    size: 0,
                    no_map: true,
                    reusable: false,
                }),
                None,
                None,
            ],
            truncated: false,
        };
        let kernel = KernelLayout {
            start: 0x20_0000,
            end: 0x30_0000,
            heap_start: 0x30_0000,
            heap_end: 0x40_0000,
            stack_bottom: 0x40_0000,
            stack_top: 0x44_0000,
        };

        let candidate = conservative_low_memory_candidate(
            &banks,
            None,
            Some(&reserved_memory),
            None,
            None,
            kernel,
        )
        .expect("candidate");

        assert_eq!(candidate.start, 0x3fd2_4000);
        assert_eq!(candidate.end, 0x4000_0000);
    }

    #[test_case]
    fn conservative_candidate_ignores_zero_sized_reserved_memory_ranges() {
        let banks = FdtMemoryBanks {
            address_cells: 2,
            size_cells: 2,
            count: 1,
            entries: [
                Some(FdtMemoryBank {
                    address: 0,
                    size: 0x1000_0000,
                }),
                None,
                None,
                None,
            ],
            truncated: false,
        };
        let reserved_memory = FdtReservedMemoryRanges {
            address_cells: 2,
            size_cells: 2,
            node_count: 1,
            range_count: 1,
            entries: [
                Some(FdtReservedMemoryRange {
                    address: 0x0fff_0000,
                    size: 0,
                    no_map: true,
                    reusable: false,
                }),
                None,
                None,
                None,
            ],
            truncated: false,
        };
        let kernel = KernelLayout {
            start: 0x20_0000,
            end: 0x30_0000,
            heap_start: 0x30_0000,
            heap_end: 0x40_0000,
            stack_bottom: 0x40_0000,
            stack_top: 0x44_0000,
        };

        let candidate = conservative_low_memory_candidate(
            &banks,
            None,
            Some(&reserved_memory),
            None,
            None,
            kernel,
        )
        .expect("candidate");

        assert_eq!(candidate.start, 0x30_0000);
        assert_eq!(candidate.end, 0x1000_0000);
    }

    #[test_case]
    fn conservative_candidate_excludes_page_rounded_firmware_initrd_range() {
        let banks = FdtMemoryBanks {
            address_cells: 2,
            size_cells: 2,
            count: 1,
            entries: [
                Some(FdtMemoryBank {
                    address: 0,
                    size: 0x4000_0000,
                }),
                None,
                None,
                None,
            ],
            truncated: false,
        };
        let kernel = KernelLayout {
            start: 0x20_0000,
            end: 0x30_0000,
            heap_start: 0x30_0000,
            heap_end: 0x40_0000,
            stack_bottom: 0x40_0000,
            stack_top: 0x44_0000,
        };
        let initrd = FdtInitrdRange {
            start: 0x2eff_f000,
            end: 0x2eff_f296,
        };

        let candidate =
            conservative_low_memory_candidate(&banks, None, None, None, Some(initrd), kernel)
                .expect("candidate");

        assert_eq!(candidate.start, 0x2f00_0000);
        assert_eq!(candidate.end, 0x4000_0000);
    }

    #[test_case]
    fn conservative_candidate_fails_when_firmware_initrd_exhausts_low_tail() {
        let banks = FdtMemoryBanks {
            address_cells: 2,
            size_cells: 2,
            count: 1,
            entries: [
                Some(FdtMemoryBank {
                    address: 0,
                    size: 0x2f00_0000,
                }),
                None,
                None,
                None,
            ],
            truncated: false,
        };
        let kernel = KernelLayout {
            start: 0x20_0000,
            end: 0x30_0000,
            heap_start: 0x30_0000,
            heap_end: 0x40_0000,
            stack_bottom: 0x40_0000,
            stack_top: 0x44_0000,
        };
        let initrd = FdtInitrdRange {
            start: 0x2eff_f000,
            end: 0x2eff_f296,
        };

        let candidate =
            conservative_low_memory_candidate(&banks, None, None, None, Some(initrd), kernel);

        assert_eq!(candidate, None);
    }
}
