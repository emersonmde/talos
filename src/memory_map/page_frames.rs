use super::common::{align_down, align_up, is_aligned, page_frame_span, ranges_intersect};
use super::layout::EarlyUsableMemory;
use super::translation::{
    EARLY_TRANSLATION_LOW_MAP_END, EARLY_TRANSLATION_LOW_MAP_START, EarlyTranslationTableLayout,
    early_translation_table_layout,
};

pub const EARLY_PAGE_SIZE: u64 = 0x1000;
pub const EARLY_BOOTSTRAP_RESERVE_PAGES: u64 = 0x10;
pub const EARLY_BOOTSTRAP_RESERVE_REASON: &str = "bootstrap-page-tables";
pub const EARLY_BOOTSTRAP_ALLOCATOR_KIND: &str = "bump-no-free-low-tail";
#[allow(dead_code)]
pub const EARLY_PAGE_FRAME_RESERVED_KIND: &str = "bootstrap-reserved";
#[allow(dead_code)]
pub const EARLY_PAGE_FRAME_ALLOCATOR_OWNED_KIND: &str = "bootstrap-bump-owned";
#[allow(dead_code)]
pub const EARLY_PAGE_FRAME_DEFERRED_KIND: &str = "outside-conservative-low-tail";
#[allow(dead_code)]
pub const EARLY_BOOTSTRAP_SLACK_RESERVED_KIND: &str = "bootstrap-reserved-unused";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EarlyPageFrameSeed {
    pub start: u64,
    pub end: u64,
    pub page_size: u64,
    pub page_count: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EarlyBootstrapPageReservation {
    pub start: u64,
    pub end: u64,
    pub page_size: u64,
    pub page_count: u64,
    pub remaining: EarlyPageFrameSeed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EarlyBootstrapAllocatorPlan {
    pub start: u64,
    pub end: u64,
    pub page_size: u64,
    pub page_count: u64,
    pub size: u64,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EarlyPageFrameSpan {
    pub start: u64,
    pub end: u64,
    pub page_size: u64,
    pub page_count: u64,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EarlyPageFrameOwnershipContract {
    pub seed: EarlyPageFrameSpan,
    pub bootstrap_reserved: EarlyPageFrameSpan,
    pub translation_tables: EarlyPageFrameSpan,
    pub bootstrap_slack_reserved: Option<EarlyPageFrameSpan>,
    pub allocator_owned: EarlyPageFrameSpan,
    pub reserved_kind: &'static str,
    pub bootstrap_slack_reserved_kind: &'static str,
    pub allocator_owned_kind: &'static str,
    pub deferred_kind: &'static str,
}

pub fn early_page_frame_seed_span(candidate: EarlyUsableMemory) -> Option<EarlyPageFrameSeed> {
    let start = align_up(candidate.start, EARLY_PAGE_SIZE)?;
    let end = align_down(candidate.end, EARLY_PAGE_SIZE);
    if start >= end {
        return None;
    }

    let size = end.checked_sub(start)?;
    let page_count = size.checked_div(EARLY_PAGE_SIZE)?;
    if page_count == 0 {
        return None;
    }

    Some(EarlyPageFrameSeed {
        start,
        end,
        page_size: EARLY_PAGE_SIZE,
        page_count,
    })
}

pub fn early_bootstrap_page_reservation(
    seed: EarlyPageFrameSeed,
    reserve_pages: u64,
) -> Option<EarlyBootstrapPageReservation> {
    if seed.page_size == 0 || reserve_pages == 0 || reserve_pages >= seed.page_count {
        return None;
    }

    let reserve_size = reserve_pages.checked_mul(seed.page_size)?;
    let reserve_end = seed.start.checked_add(reserve_size)?;
    if reserve_end > seed.end {
        return None;
    }

    let remaining_page_count = seed.page_count.checked_sub(reserve_pages)?;
    if remaining_page_count == 0 {
        return None;
    }

    Some(EarlyBootstrapPageReservation {
        start: seed.start,
        end: reserve_end,
        page_size: seed.page_size,
        page_count: reserve_pages,
        remaining: EarlyPageFrameSeed {
            start: reserve_end,
            end: seed.end,
            page_size: seed.page_size,
            page_count: remaining_page_count,
        },
    })
}

pub fn early_bootstrap_allocator_plan(
    remaining: EarlyPageFrameSeed,
) -> Option<EarlyBootstrapAllocatorPlan> {
    if remaining.page_size != EARLY_PAGE_SIZE
        || remaining.page_count == 0
        || remaining.start >= remaining.end
        || !is_aligned(remaining.start, EARLY_PAGE_SIZE)
        || !is_aligned(remaining.end, EARLY_PAGE_SIZE)
        || remaining.start < EARLY_TRANSLATION_LOW_MAP_START
        || remaining.end > EARLY_TRANSLATION_LOW_MAP_END
    {
        return None;
    }

    let size = remaining.end.checked_sub(remaining.start)?;
    if size.checked_div(remaining.page_size)? != remaining.page_count {
        return None;
    }

    Some(EarlyBootstrapAllocatorPlan {
        start: remaining.start,
        end: remaining.end,
        page_size: remaining.page_size,
        page_count: remaining.page_count,
        size,
    })
}

#[allow(dead_code)]
pub fn early_page_frame_ownership_contract(
    seed: EarlyPageFrameSeed,
    reservation: EarlyBootstrapPageReservation,
    translation_tables: EarlyTranslationTableLayout,
    allocator: EarlyBootstrapAllocatorPlan,
) -> Option<EarlyPageFrameOwnershipContract> {
    let seed_span = page_frame_span(seed.start, seed.end, seed.page_size)?;
    if seed_span.page_count != seed.page_count {
        return None;
    }

    let expected_reservation = early_bootstrap_page_reservation(seed, reservation.page_count)?;
    if expected_reservation != reservation {
        return None;
    }

    let expected_translation_tables = early_translation_table_layout(reservation)?;
    if expected_translation_tables != translation_tables {
        return None;
    }

    let expected_allocator = early_bootstrap_allocator_plan(reservation.remaining)?;
    if expected_allocator != allocator {
        return None;
    }

    let bootstrap_reserved =
        page_frame_span(reservation.start, reservation.end, reservation.page_size)?;
    let translation_table_span = page_frame_span(
        translation_tables.start,
        translation_tables.end,
        translation_tables.page_size,
    )?;
    let allocator_owned = page_frame_span(allocator.start, allocator.end, allocator.page_size)?;
    let bootstrap_slack_reserved = if translation_tables.end < reservation.end {
        Some(page_frame_span(
            translation_tables.end,
            reservation.end,
            reservation.page_size,
        )?)
    } else {
        None
    };

    if ranges_intersect(
        bootstrap_reserved.start,
        bootstrap_reserved.end,
        allocator_owned.start,
        allocator_owned.end,
    ) || ranges_intersect(
        translation_table_span.start,
        translation_table_span.end,
        allocator_owned.start,
        allocator_owned.end,
    ) {
        return None;
    }

    Some(EarlyPageFrameOwnershipContract {
        seed: seed_span,
        bootstrap_reserved,
        translation_tables: translation_table_span,
        bootstrap_slack_reserved,
        allocator_owned,
        reserved_kind: EARLY_PAGE_FRAME_RESERVED_KIND,
        bootstrap_slack_reserved_kind: EARLY_BOOTSTRAP_SLACK_RESERVED_KIND,
        allocator_owned_kind: EARLY_PAGE_FRAME_ALLOCATOR_OWNED_KIND,
        deferred_kind: EARLY_PAGE_FRAME_DEFERRED_KIND,
    })
}

#[cfg(test)]
mod tests {
    use super::super::common::ranges_intersect;
    use super::super::layout::{
        EARLY_USABLE_ALIGNMENT, FdtBlobRange, KernelLayout, conservative_low_memory_candidate,
    };
    use super::super::translation::{
        EARLY_TRANSLATION_TABLE_PAGES, early_translation_table_layout,
    };
    use super::*;
    use crate::device_tree::{
        FdtMemoryBank, FdtMemoryBanks, FdtMemoryReservation, FdtMemoryReservations,
        FdtReservedMemoryRange, FdtReservedMemoryRanges,
    };

    #[test_case]
    fn page_frame_seed_uses_page_aligned_usable_span() {
        let candidate = EarlyUsableMemory {
            bank_index: 0,
            start: 0x2f00_0123,
            end: 0x2f03_fedc,
            size: 0x3fd_b9,
            alignment: EARLY_USABLE_ALIGNMENT,
        };

        let seed = early_page_frame_seed_span(candidate).expect("seed span");

        assert_eq!(seed.start, 0x2f00_1000);
        assert_eq!(seed.end, 0x2f03_f000);
        assert_eq!(seed.page_size, EARLY_PAGE_SIZE);
        assert_eq!(seed.page_count, 0x3e);
    }

    #[test_case]
    fn page_frame_seed_rejects_sub_page_usable_span() {
        let candidate = EarlyUsableMemory {
            bank_index: 0,
            start: 0x4000,
            end: 0x4fff,
            size: 0xfff,
            alignment: EARLY_USABLE_ALIGNMENT,
        };

        assert_eq!(early_page_frame_seed_span(candidate), None);
    }

    #[test_case]
    fn bootstrap_page_reservation_carves_from_seed_start() {
        let seed = EarlyPageFrameSeed {
            start: 0x2f00_0000,
            end: 0x2f20_0000,
            page_size: EARLY_PAGE_SIZE,
            page_count: 0x200,
        };

        let reservation = early_bootstrap_page_reservation(seed, EARLY_BOOTSTRAP_RESERVE_PAGES)
            .expect("bootstrap reservation");

        assert_eq!(EARLY_BOOTSTRAP_RESERVE_REASON, "bootstrap-page-tables");
        assert_eq!(reservation.start, 0x2f00_0000);
        assert_eq!(reservation.end, 0x2f01_0000);
        assert_eq!(reservation.page_size, EARLY_PAGE_SIZE);
        assert_eq!(reservation.page_count, 0x10);
        assert_eq!(
            reservation.remaining,
            EarlyPageFrameSeed {
                start: 0x2f01_0000,
                end: 0x2f20_0000,
                page_size: EARLY_PAGE_SIZE,
                page_count: 0x1f0,
            }
        );
    }

    #[test_case]
    fn bootstrap_page_reservation_rejects_empty_or_consumed_seed() {
        let seed = EarlyPageFrameSeed {
            start: 0x8000,
            end: 0xc000,
            page_size: EARLY_PAGE_SIZE,
            page_count: 4,
        };

        assert_eq!(early_bootstrap_page_reservation(seed, 0), None);
        assert_eq!(early_bootstrap_page_reservation(seed, 4), None);
        assert_eq!(early_bootstrap_page_reservation(seed, 5), None);
    }

    #[test_case]
    fn bootstrap_allocator_plan_uses_remaining_low_tail_frames() {
        let seed = EarlyPageFrameSeed {
            start: 0x2f00_0000,
            end: 0x2f20_0000,
            page_size: EARLY_PAGE_SIZE,
            page_count: 0x200,
        };
        let reservation = early_bootstrap_page_reservation(seed, EARLY_BOOTSTRAP_RESERVE_PAGES)
            .expect("bootstrap reservation");

        let plan = early_bootstrap_allocator_plan(reservation.remaining)
            .expect("bootstrap allocator plan");

        assert_eq!(EARLY_BOOTSTRAP_ALLOCATOR_KIND, "bump-no-free-low-tail");
        assert_eq!(plan.start, 0x2f01_0000);
        assert_eq!(plan.end, 0x2f20_0000);
        assert_eq!(plan.page_size, EARLY_PAGE_SIZE);
        assert_eq!(plan.page_count, 0x1f0);
        assert_eq!(plan.size, 0x1f0_000);
    }

    #[test_case]
    fn bootstrap_allocator_plan_rejects_unaligned_empty_or_unmapped_spans() {
        let valid = EarlyPageFrameSeed {
            start: 0x2f01_0000,
            end: 0x2f20_0000,
            page_size: EARLY_PAGE_SIZE,
            page_count: 0x1f0,
        };

        assert_eq!(
            early_bootstrap_allocator_plan(EarlyPageFrameSeed {
                start: 0x2f01_0001,
                ..valid
            }),
            None
        );
        assert_eq!(
            early_bootstrap_allocator_plan(EarlyPageFrameSeed {
                end: valid.start,
                page_count: 0,
                ..valid
            }),
            None
        );
        assert_eq!(
            early_bootstrap_allocator_plan(EarlyPageFrameSeed {
                end: EARLY_TRANSLATION_LOW_MAP_END + EARLY_PAGE_SIZE,
                page_count: 0x11000,
                ..valid
            }),
            None
        );
        assert_eq!(
            early_bootstrap_allocator_plan(EarlyPageFrameSeed {
                page_count: valid.page_count - 1,
                ..valid
            }),
            None
        );
    }

    #[test_case]
    fn page_frame_ownership_contract_names_current_low_tail_partitions() {
        let seed = EarlyPageFrameSeed {
            start: 0x2f00_0000,
            end: 0x2f20_0000,
            page_size: EARLY_PAGE_SIZE,
            page_count: 0x200,
        };
        let reservation = early_bootstrap_page_reservation(seed, EARLY_BOOTSTRAP_RESERVE_PAGES)
            .expect("bootstrap reservation");
        let layout = early_translation_table_layout(reservation).expect("translation layout");
        let allocator =
            early_bootstrap_allocator_plan(reservation.remaining).expect("allocator plan");

        let contract = early_page_frame_ownership_contract(seed, reservation, layout, allocator)
            .expect("ownership contract");

        assert_eq!(contract.reserved_kind, EARLY_PAGE_FRAME_RESERVED_KIND);
        assert_eq!(
            contract.bootstrap_slack_reserved_kind,
            EARLY_BOOTSTRAP_SLACK_RESERVED_KIND
        );
        assert_eq!(
            contract.allocator_owned_kind,
            EARLY_PAGE_FRAME_ALLOCATOR_OWNED_KIND
        );
        assert_eq!(contract.deferred_kind, EARLY_PAGE_FRAME_DEFERRED_KIND);
        assert_eq!(
            contract.seed,
            EarlyPageFrameSpan {
                start: 0x2f00_0000,
                end: 0x2f20_0000,
                page_size: EARLY_PAGE_SIZE,
                page_count: 0x200,
            }
        );
        assert_eq!(
            contract.bootstrap_reserved,
            EarlyPageFrameSpan {
                start: 0x2f00_0000,
                end: 0x2f01_0000,
                page_size: EARLY_PAGE_SIZE,
                page_count: 0x10,
            }
        );
        assert_eq!(
            contract.translation_tables,
            EarlyPageFrameSpan {
                start: 0x2f00_0000,
                end: 0x2f00_4000,
                page_size: EARLY_PAGE_SIZE,
                page_count: EARLY_TRANSLATION_TABLE_PAGES,
            }
        );
        assert_eq!(
            contract.bootstrap_slack_reserved,
            Some(EarlyPageFrameSpan {
                start: 0x2f00_4000,
                end: 0x2f01_0000,
                page_size: EARLY_PAGE_SIZE,
                page_count: 0xc,
            })
        );
        assert_eq!(
            contract.allocator_owned,
            EarlyPageFrameSpan {
                start: 0x2f01_0000,
                end: 0x2f20_0000,
                page_size: EARLY_PAGE_SIZE,
                page_count: 0x1f0,
            }
        );
    }

    #[test_case]
    fn page_frame_ownership_contract_rejects_mismatched_allocator_span() {
        let seed = EarlyPageFrameSeed {
            start: 0x2f00_0000,
            end: 0x2f20_0000,
            page_size: EARLY_PAGE_SIZE,
            page_count: 0x200,
        };
        let reservation = early_bootstrap_page_reservation(seed, EARLY_BOOTSTRAP_RESERVE_PAGES)
            .expect("bootstrap reservation");
        let layout = early_translation_table_layout(reservation).expect("translation layout");
        let allocator = EarlyBootstrapAllocatorPlan {
            start: reservation.start,
            end: reservation.remaining.end,
            page_size: EARLY_PAGE_SIZE,
            page_count: seed.page_count,
            size: seed.page_count * EARLY_PAGE_SIZE,
        };

        assert_eq!(
            early_page_frame_ownership_contract(seed, reservation, layout, allocator),
            None
        );
    }

    #[test_case]
    fn page_frame_ownership_contract_excludes_kernel_dtb_reservations_stack_and_tables() {
        let banks = FdtMemoryBanks {
            address_cells: 2,
            size_cells: 2,
            count: 2,
            entries: [
                Some(FdtMemoryBank {
                    address: 0,
                    size: 0x3fc0_0000,
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
        let reservations = FdtMemoryReservations {
            count: 1,
            entries: [
                Some(FdtMemoryReservation {
                    address: 0x2eff_4000,
                    size: 0x2000,
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
                    address: 0x2eff_8000,
                    size: 0x8000,
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
            end: 0x80_0000,
            heap_start: 0x30_0000,
            heap_end: 0x50_0000,
            stack_bottom: 0x2efe_0000,
            stack_top: 0x2eff_0000,
        };
        let dtb = FdtBlobRange {
            address: 0x2eff_1000,
            size: 0x2000,
        };

        let candidate = conservative_low_memory_candidate(
            &banks,
            Some(&reservations),
            Some(&reserved_memory),
            Some(dtb),
            kernel,
        )
        .expect("candidate");
        let seed = early_page_frame_seed_span(candidate).expect("seed");
        let reservation = early_bootstrap_page_reservation(seed, EARLY_BOOTSTRAP_RESERVE_PAGES)
            .expect("bootstrap reservation");
        let layout = early_translation_table_layout(reservation).expect("translation layout");
        let allocator =
            early_bootstrap_allocator_plan(reservation.remaining).expect("allocator plan");
        let contract = early_page_frame_ownership_contract(seed, reservation, layout, allocator)
            .expect("ownership contract");

        assert_eq!(candidate.start, 0x2f00_0000);
        assert_eq!(contract.allocator_owned.start, 0x2f01_0000);
        assert_eq!(contract.allocator_owned.end, 0x3fc0_0000);

        let protected_ranges = [
            (kernel.start, kernel.end),
            (kernel.heap_start, kernel.heap_end),
            (kernel.stack_bottom, kernel.stack_top),
            (dtb.address, dtb.address + dtb.size),
            (0x2eff_4000, 0x2eff_6000),
            (0x2eff_8000, 0x2f00_0000),
        ];
        for (start, end) in protected_ranges {
            assert!(!ranges_intersect(
                start,
                end,
                contract.bootstrap_reserved.start,
                contract.bootstrap_reserved.end
            ));
            assert!(!ranges_intersect(
                start,
                end,
                contract.allocator_owned.start,
                contract.allocator_owned.end
            ));
        }

        assert!(!ranges_intersect(
            contract.translation_tables.start,
            contract.translation_tables.end,
            contract.allocator_owned.start,
            contract.allocator_owned.end
        ));
    }
}
