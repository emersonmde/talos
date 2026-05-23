#![cfg_attr(not(any(test, talos_target_rpi5_bcm2712)), allow(dead_code))]

use crate::device_tree::{FdtMemoryBanks, FdtMemoryReservations, FdtReservedMemoryRanges};

pub const EARLY_USABLE_ALIGNMENT: u64 = 0x1000;
pub const EARLY_USABLE_POLICY: &str = "low-tail";
pub const EARLY_PAGE_SIZE: u64 = 0x1000;
pub const EARLY_BOOTSTRAP_RESERVE_PAGES: u64 = 0x10;
pub const EARLY_BOOTSTRAP_RESERVE_REASON: &str = "bootstrap-page-tables";
pub const EARLY_BOOTSTRAP_ALLOCATOR_KIND: &str = "bump-no-free-low-tail";
pub const EARLY_TRANSLATION_TABLE_PAGES: u64 = 0x4;
pub const EARLY_TRANSLATION_TABLE_KIND: &str = "layout-only";
pub const EARLY_TRANSLATION_TABLE_POPULATION_KIND: &str = "stage1-4k-no-enable";
pub const EARLY_TRANSLATION_ROOT_INDEX: u64 = 0;
pub const EARLY_TRANSLATION_LOW_L1_INDEX: u64 = 0;
pub const EARLY_TRANSLATION_BCM2712_MMIO_L1_INDEX: u64 = 0x41;
pub const EARLY_TRANSLATION_L2_BLOCK_SIZE: u64 = 0x20_0000;
pub const EARLY_TRANSLATION_LOW_MAP_START: u64 = 0;
pub const EARLY_TRANSLATION_LOW_MAP_END: u64 = 0x4000_0000;
pub const EARLY_TRANSLATION_BCM2712_MMIO_START: u64 = 0x10_7c00_0000;
pub const EARLY_TRANSLATION_BCM2712_MMIO_END: u64 = 0x10_8000_0000;
pub const EARLY_TRANSLATION_NORMAL_ATTR_INDEX: u64 = 0;
pub const EARLY_TRANSLATION_DEVICE_ATTR_INDEX: u64 = 1;
pub const EARLY_TRANSLATION_REGISTER_PLAN_KIND: &str = "el2-stage1-4k-no-enable";
#[cfg_attr(not(talos_target_rpi5_bcm2712), allow(dead_code))]
pub const EARLY_TRANSLATION_ENABLE_KIND: &str = "el2-stage1-4k-enabled";
#[cfg_attr(not(talos_target_rpi5_bcm2712), allow(dead_code))]
pub const EARLY_INSTRUCTION_CACHE_ENABLE_KIND: &str = "el2-stage1-icache-enabled";
#[cfg_attr(not(talos_target_rpi5_bcm2712), allow(dead_code))]
pub const EARLY_DATA_CACHE_ENABLE_KIND: &str = "el2-stage1-dcache-enabled";
pub const EARLY_TRANSLATION_MAIR_NORMAL_WBWA: u64 = 0xff;
pub const EARLY_TRANSLATION_MAIR_DEVICE_NGNRE: u64 = 0x04;
pub const EARLY_TRANSLATION_SCTLR_M_ENABLE: u64 = 1 << 0;
#[cfg_attr(not(talos_target_rpi5_bcm2712), allow(dead_code))]
pub const EARLY_TRANSLATION_SCTLR_I_ENABLE: u64 = 1 << 12;
#[cfg_attr(not(talos_target_rpi5_bcm2712), allow(dead_code))]
pub const EARLY_TRANSLATION_SCTLR_C_ENABLE: u64 = 1 << 2;

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EarlyTranslationTableLayout {
    pub start: u64,
    pub end: u64,
    pub page_size: u64,
    pub page_count: u64,
    pub root_table: u64,
    pub l1_table: u64,
    pub low_l2_table: u64,
    pub mmio_l2_table: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EarlyTranslationTablePopulation {
    pub root_entries: u64,
    pub l1_entries: u64,
    pub low_l2_blocks: u64,
    pub mmio_l2_blocks: u64,
    pub low_map_start: u64,
    pub low_map_end: u64,
    pub mmio_map_start: u64,
    pub mmio_map_end: u64,
    pub block_size: u64,
    pub root_index: u64,
    pub low_l1_index: u64,
    pub mmio_l1_index: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EarlyTranslationRegisterPlan {
    pub current_el: u8,
    pub mair: u64,
    pub tcr: u64,
    pub ttbr0: u64,
    pub sctlr_set: u64,
    pub va_bits: u64,
    pub pa_bits: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EarlyInstructionCacheEnablePlan {
    pub current_el: u8,
    pub sctlr_before: u64,
    pub sctlr_set: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EarlyDataCacheEnablePlan {
    pub current_el: u8,
    pub sctlr_before: u64,
    pub sctlr_set: u64,
}

pub fn conservative_low_memory_candidate(
    banks: &FdtMemoryBanks,
    reservations: Option<&FdtMemoryReservations>,
    reserved_memory: Option<&FdtReservedMemoryRanges>,
    dtb: Option<FdtBlobRange>,
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

pub fn early_translation_table_layout(
    reservation: EarlyBootstrapPageReservation,
) -> Option<EarlyTranslationTableLayout> {
    if reservation.page_size == 0 || reservation.page_count < EARLY_TRANSLATION_TABLE_PAGES {
        return None;
    }

    let table_bytes = EARLY_TRANSLATION_TABLE_PAGES.checked_mul(reservation.page_size)?;
    let table_end = reservation.start.checked_add(table_bytes)?;
    if table_end > reservation.end {
        return None;
    }

    let l1_table = reservation.start.checked_add(reservation.page_size)?;
    let low_l2_table = l1_table.checked_add(reservation.page_size)?;
    let mmio_l2_table = low_l2_table.checked_add(reservation.page_size)?;

    Some(EarlyTranslationTableLayout {
        start: reservation.start,
        end: table_end,
        page_size: reservation.page_size,
        page_count: EARLY_TRANSLATION_TABLE_PAGES,
        root_table: reservation.start,
        l1_table,
        low_l2_table,
        mmio_l2_table,
    })
}

pub unsafe fn populate_early_translation_tables(
    layout: EarlyTranslationTableLayout,
) -> Option<EarlyTranslationTablePopulation> {
    let population = early_translation_table_population_plan(layout)?;

    unsafe {
        zero_table_page(layout.root_table);
        zero_table_page(layout.l1_table);
        zero_table_page(layout.low_l2_table);
        zero_table_page(layout.mmio_l2_table);

        write_table_entry(
            layout.root_table,
            EARLY_TRANSLATION_ROOT_INDEX,
            table_descriptor(layout.l1_table)?,
        );
        write_table_entry(
            layout.l1_table,
            EARLY_TRANSLATION_LOW_L1_INDEX,
            table_descriptor(layout.low_l2_table)?,
        );
        write_table_entry(
            layout.l1_table,
            EARLY_TRANSLATION_BCM2712_MMIO_L1_INDEX,
            table_descriptor(layout.mmio_l2_table)?,
        );

        populate_block_descriptors(
            layout.low_l2_table,
            EARLY_TRANSLATION_LOW_MAP_START,
            EARLY_TRANSLATION_LOW_MAP_END,
            normal_block_descriptor,
        )?;
        populate_block_descriptors(
            layout.mmio_l2_table,
            EARLY_TRANSLATION_BCM2712_MMIO_START,
            EARLY_TRANSLATION_BCM2712_MMIO_END,
            device_block_descriptor,
        )?;
        core::arch::asm!("dsb sy", options(nostack, preserves_flags));
    }

    Some(population)
}

pub fn early_translation_table_population_plan(
    layout: EarlyTranslationTableLayout,
) -> Option<EarlyTranslationTablePopulation> {
    if layout.page_size != EARLY_PAGE_SIZE || layout.page_count != EARLY_TRANSLATION_TABLE_PAGES {
        return None;
    }
    if !is_aligned(layout.root_table, EARLY_PAGE_SIZE)
        || !is_aligned(layout.l1_table, EARLY_PAGE_SIZE)
        || !is_aligned(layout.low_l2_table, EARLY_PAGE_SIZE)
        || !is_aligned(layout.mmio_l2_table, EARLY_PAGE_SIZE)
    {
        return None;
    }

    let low_l2_blocks = block_count(
        EARLY_TRANSLATION_LOW_MAP_START,
        EARLY_TRANSLATION_LOW_MAP_END,
    )?;
    let mmio_l2_blocks = block_count(
        EARLY_TRANSLATION_BCM2712_MMIO_START,
        EARLY_TRANSLATION_BCM2712_MMIO_END,
    )?;

    Some(EarlyTranslationTablePopulation {
        root_entries: 1,
        l1_entries: 2,
        low_l2_blocks,
        mmio_l2_blocks,
        low_map_start: EARLY_TRANSLATION_LOW_MAP_START,
        low_map_end: EARLY_TRANSLATION_LOW_MAP_END,
        mmio_map_start: EARLY_TRANSLATION_BCM2712_MMIO_START,
        mmio_map_end: EARLY_TRANSLATION_BCM2712_MMIO_END,
        block_size: EARLY_TRANSLATION_L2_BLOCK_SIZE,
        root_index: EARLY_TRANSLATION_ROOT_INDEX,
        low_l1_index: EARLY_TRANSLATION_LOW_L1_INDEX,
        mmio_l1_index: EARLY_TRANSLATION_BCM2712_MMIO_L1_INDEX,
    })
}

pub fn early_translation_register_plan(
    layout: EarlyTranslationTableLayout,
    current_el: u8,
) -> Option<EarlyTranslationRegisterPlan> {
    if current_el != 2 {
        return None;
    }
    early_translation_table_population_plan(layout)?;

    Some(EarlyTranslationRegisterPlan {
        current_el,
        mair: mair_value(),
        tcr: tcr_el2_value(),
        ttbr0: layout.root_table,
        sctlr_set: EARLY_TRANSLATION_SCTLR_M_ENABLE,
        va_bits: 48,
        pa_bits: 48,
    })
}

#[cfg_attr(not(talos_target_rpi5_bcm2712), allow(dead_code))]
pub fn early_instruction_cache_enable_plan(
    current_el: u8,
    sctlr_before: u64,
) -> Option<EarlyInstructionCacheEnablePlan> {
    if current_el != 2 || (sctlr_before & EARLY_TRANSLATION_SCTLR_M_ENABLE) == 0 {
        return None;
    }

    Some(EarlyInstructionCacheEnablePlan {
        current_el,
        sctlr_before,
        sctlr_set: EARLY_TRANSLATION_SCTLR_I_ENABLE,
    })
}

#[cfg_attr(not(talos_target_rpi5_bcm2712), allow(dead_code))]
pub fn early_data_cache_enable_plan(
    current_el: u8,
    sctlr_before: u64,
) -> Option<EarlyDataCacheEnablePlan> {
    let required = EARLY_TRANSLATION_SCTLR_M_ENABLE | EARLY_TRANSLATION_SCTLR_I_ENABLE;
    if current_el != 2 || (sctlr_before & required) != required {
        return None;
    }

    Some(EarlyDataCacheEnablePlan {
        current_el,
        sctlr_before,
        sctlr_set: EARLY_TRANSLATION_SCTLR_C_ENABLE,
    })
}

const STAGE1_DESC_VALID: u64 = 1 << 0;
const STAGE1_DESC_TABLE: u64 = 1 << 1;
const STAGE1_DESC_ATTR_INDEX_SHIFT: u64 = 2;
const STAGE1_DESC_SH_INNER: u64 = 0b11 << 8;
const STAGE1_DESC_AF: u64 = 1 << 10;
const STAGE1_DESC_PXN: u64 = 1 << 53;
const STAGE1_DESC_UXN: u64 = 1 << 54;
const STAGE1_TABLE_ADDR_MASK: u64 = 0x0000_ffff_ffff_f000;
const STAGE1_BLOCK_ADDR_MASK: u64 = 0x0000_ffff_ffe0_0000;
const TABLE_ENTRIES: u64 = 512;
const TCR_EL2_T0SZ_SHIFT: u64 = 0;
const TCR_EL2_IRGN0_SHIFT: u64 = 8;
const TCR_EL2_ORGN0_SHIFT: u64 = 10;
const TCR_EL2_SH0_SHIFT: u64 = 12;
const TCR_EL2_TG0_4K: u64 = 0b00 << 14;
const TCR_EL2_PS_SHIFT: u64 = 16;
const TCR_CACHE_WBWA: u64 = 0b01;
const TCR_SH_INNER: u64 = 0b11;
const TCR_PS_48BIT: u64 = 0b101;

fn table_descriptor(address: u64) -> Option<u64> {
    if !is_aligned(address, EARLY_PAGE_SIZE) {
        return None;
    }

    Some((address & STAGE1_TABLE_ADDR_MASK) | STAGE1_DESC_TABLE | STAGE1_DESC_VALID)
}

fn mair_value() -> u64 {
    EARLY_TRANSLATION_MAIR_NORMAL_WBWA
        | (EARLY_TRANSLATION_MAIR_DEVICE_NGNRE << (EARLY_TRANSLATION_DEVICE_ATTR_INDEX * 8))
}

fn tcr_el2_value() -> u64 {
    let t0sz = 64 - 48;

    (t0sz << TCR_EL2_T0SZ_SHIFT)
        | (TCR_CACHE_WBWA << TCR_EL2_IRGN0_SHIFT)
        | (TCR_CACHE_WBWA << TCR_EL2_ORGN0_SHIFT)
        | (TCR_SH_INNER << TCR_EL2_SH0_SHIFT)
        | TCR_EL2_TG0_4K
        | (TCR_PS_48BIT << TCR_EL2_PS_SHIFT)
}

fn normal_block_descriptor(address: u64) -> Option<u64> {
    block_descriptor(
        address,
        EARLY_TRANSLATION_NORMAL_ATTR_INDEX,
        STAGE1_DESC_SH_INNER,
    )
}

fn device_block_descriptor(address: u64) -> Option<u64> {
    block_descriptor(
        address,
        EARLY_TRANSLATION_DEVICE_ATTR_INDEX,
        STAGE1_DESC_PXN | STAGE1_DESC_UXN,
    )
}

fn block_descriptor(address: u64, attr_index: u64, extra_attrs: u64) -> Option<u64> {
    if !is_aligned(address, EARLY_TRANSLATION_L2_BLOCK_SIZE) {
        return None;
    }

    Some(
        (address & STAGE1_BLOCK_ADDR_MASK)
            | (attr_index << STAGE1_DESC_ATTR_INDEX_SHIFT)
            | extra_attrs
            | STAGE1_DESC_AF
            | STAGE1_DESC_VALID,
    )
}

fn block_count(start: u64, end: u64) -> Option<u64> {
    if start >= end
        || !is_aligned(start, EARLY_TRANSLATION_L2_BLOCK_SIZE)
        || !is_aligned(end, EARLY_TRANSLATION_L2_BLOCK_SIZE)
    {
        return None;
    }

    let bytes = end.checked_sub(start)?;
    let blocks = bytes.checked_div(EARLY_TRANSLATION_L2_BLOCK_SIZE)?;
    if blocks == 0 || blocks > TABLE_ENTRIES {
        return None;
    }

    Some(blocks)
}

unsafe fn zero_table_page(table: u64) {
    let table = table as *mut u64;
    let mut index = 0usize;
    while index < TABLE_ENTRIES as usize {
        unsafe {
            core::ptr::write_volatile(table.add(index), 0);
        }
        index += 1;
    }
}

unsafe fn write_table_entry(table: u64, index: u64, descriptor: u64) {
    debug_assert!(index < TABLE_ENTRIES);
    unsafe {
        core::ptr::write_volatile((table as *mut u64).add(index as usize), descriptor);
    }
}

unsafe fn populate_block_descriptors(
    table: u64,
    start: u64,
    end: u64,
    descriptor: fn(u64) -> Option<u64>,
) -> Option<()> {
    let count = block_count(start, end)?;
    let l1_base = (start >> 30) << 30;
    let first_index = (start - l1_base) / EARLY_TRANSLATION_L2_BLOCK_SIZE;
    if first_index.checked_add(count)? > TABLE_ENTRIES {
        return None;
    }

    let mut block = 0u64;
    while block < count {
        let address = start.checked_add(block.checked_mul(EARLY_TRANSLATION_L2_BLOCK_SIZE)?)?;
        unsafe {
            write_table_entry(table, first_index + block, descriptor(address)?);
        }
        block += 1;
    }

    Some(())
}

fn reserve_after(
    candidate_start: u64,
    bank_start: u64,
    bank_end: u64,
    reserved_start: u64,
    reserved_end: u64,
) -> Option<u64> {
    if reserved_end <= reserved_start {
        return Some(candidate_start);
    }

    if !ranges_intersect(bank_start, bank_end, reserved_start, reserved_end) {
        return Some(candidate_start);
    }

    Some(core::cmp::max(
        candidate_start,
        core::cmp::min(reserved_end, bank_end),
    ))
}

fn contains_address(start: u64, end: u64, address: u64) -> bool {
    start <= address && address < end
}

fn ranges_intersect(a_start: u64, a_end: u64, b_start: u64, b_end: u64) -> bool {
    a_start < b_end && b_start < a_end
}

fn align_up(value: u64, alignment: u64) -> Option<u64> {
    debug_assert!(alignment.is_power_of_two());
    value
        .checked_add(alignment - 1)
        .map(|value| value & !(alignment - 1))
}

fn align_down(value: u64, alignment: u64) -> u64 {
    debug_assert!(alignment.is_power_of_two());
    value & !(alignment - 1)
}

fn is_aligned(value: u64, alignment: u64) -> bool {
    debug_assert!(alignment.is_power_of_two());
    value & (alignment - 1) == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device_tree::{FdtMemoryBank, FdtMemoryReservation, FdtReservedMemoryRange};

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

        let candidate = conservative_low_memory_candidate(&banks, None, None, Some(dtb), kernel)
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

        let candidate =
            conservative_low_memory_candidate(&banks, Some(&reservations), None, None, kernel)
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

        let candidate =
            conservative_low_memory_candidate(&banks, None, Some(&reserved_memory), None, kernel)
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

        let candidate =
            conservative_low_memory_candidate(&banks, None, Some(&reserved_memory), None, kernel)
                .expect("candidate");

        assert_eq!(candidate.start, 0x30_0000);
        assert_eq!(candidate.end, 0x1000_0000);
    }

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
    fn translation_table_layout_uses_first_bootstrap_pages() {
        let seed = EarlyPageFrameSeed {
            start: 0x2f00_0000,
            end: 0x2f20_0000,
            page_size: EARLY_PAGE_SIZE,
            page_count: 0x200,
        };
        let reservation = early_bootstrap_page_reservation(seed, EARLY_BOOTSTRAP_RESERVE_PAGES)
            .expect("bootstrap reservation");

        let layout = early_translation_table_layout(reservation).expect("translation layout");

        assert_eq!(EARLY_TRANSLATION_TABLE_KIND, "layout-only");
        assert_eq!(layout.start, 0x2f00_0000);
        assert_eq!(layout.end, 0x2f00_4000);
        assert_eq!(layout.page_size, EARLY_PAGE_SIZE);
        assert_eq!(layout.page_count, 0x4);
        assert_eq!(layout.root_table, 0x2f00_0000);
        assert_eq!(layout.l1_table, 0x2f00_1000);
        assert_eq!(layout.low_l2_table, 0x2f00_2000);
        assert_eq!(layout.mmio_l2_table, 0x2f00_3000);
    }

    #[test_case]
    fn translation_table_layout_rejects_too_small_reservation() {
        let seed = EarlyPageFrameSeed {
            start: 0x8000,
            end: 0xe000,
            page_size: EARLY_PAGE_SIZE,
            page_count: 6,
        };
        let reservation =
            early_bootstrap_page_reservation(seed, 3).expect("small bootstrap reservation");

        assert_eq!(early_translation_table_layout(reservation), None);
    }

    #[test_case]
    fn translation_table_population_plan_reports_minimal_descriptor_shape() {
        let layout = EarlyTranslationTableLayout {
            start: 0x2f00_0000,
            end: 0x2f00_4000,
            page_size: EARLY_PAGE_SIZE,
            page_count: EARLY_TRANSLATION_TABLE_PAGES,
            root_table: 0x2f00_0000,
            l1_table: 0x2f00_1000,
            low_l2_table: 0x2f00_2000,
            mmio_l2_table: 0x2f00_3000,
        };

        let population = early_translation_table_population_plan(layout).expect("population plan");

        assert_eq!(
            EARLY_TRANSLATION_TABLE_POPULATION_KIND,
            "stage1-4k-no-enable"
        );
        assert_eq!(population.root_entries, 1);
        assert_eq!(population.l1_entries, 2);
        assert_eq!(population.low_l2_blocks, 0x200);
        assert_eq!(population.mmio_l2_blocks, 0x20);
        assert_eq!(population.low_map_start, 0);
        assert_eq!(population.low_map_end, 0x4000_0000);
        assert_eq!(population.mmio_map_start, 0x10_7c00_0000);
        assert_eq!(population.mmio_map_end, 0x10_8000_0000);
        assert_eq!(population.block_size, EARLY_TRANSLATION_L2_BLOCK_SIZE);
        assert_eq!(population.root_index, 0);
        assert_eq!(population.low_l1_index, 0);
        assert_eq!(population.mmio_l1_index, 0x41);
    }

    #[test_case]
    fn translation_register_plan_reports_el2_no_enable_values() {
        let layout = EarlyTranslationTableLayout {
            start: 0x2f00_0000,
            end: 0x2f00_4000,
            page_size: EARLY_PAGE_SIZE,
            page_count: EARLY_TRANSLATION_TABLE_PAGES,
            root_table: 0x2f00_0000,
            l1_table: 0x2f00_1000,
            low_l2_table: 0x2f00_2000,
            mmio_l2_table: 0x2f00_3000,
        };

        let plan = early_translation_register_plan(layout, 2).expect("register plan");

        assert_eq!(
            EARLY_TRANSLATION_REGISTER_PLAN_KIND,
            "el2-stage1-4k-no-enable"
        );
        assert_eq!(plan.current_el, 2);
        assert_eq!(plan.mair, 0x4ff);
        assert_eq!(plan.tcr, 0x5_3510);
        assert_eq!(plan.ttbr0, 0x2f00_0000);
        assert_eq!(plan.sctlr_set, EARLY_TRANSLATION_SCTLR_M_ENABLE);
        assert_eq!(plan.va_bits, 48);
        assert_eq!(plan.pa_bits, 48);
    }

    #[test_case]
    fn translation_register_plan_rejects_non_el2_or_bad_layout() {
        let layout = EarlyTranslationTableLayout {
            start: 0x2f00_0000,
            end: 0x2f00_4000,
            page_size: EARLY_PAGE_SIZE,
            page_count: EARLY_TRANSLATION_TABLE_PAGES,
            root_table: 0x2f00_0000,
            l1_table: 0x2f00_1000,
            low_l2_table: 0x2f00_2000,
            mmio_l2_table: 0x2f00_3000,
        };
        let bad_layout = EarlyTranslationTableLayout {
            root_table: 0x2f00_0001,
            ..layout
        };

        assert_eq!(early_translation_register_plan(layout, 1), None);
        assert_eq!(early_translation_register_plan(layout, 3), None);
        assert_eq!(early_translation_register_plan(bad_layout, 2), None);
    }

    #[test_case]
    fn instruction_cache_enable_plan_requires_el2_and_active_mmu() {
        let sctlr_with_m = 0x30c5_0831;
        let plan = early_instruction_cache_enable_plan(2, sctlr_with_m)
            .expect("instruction cache enable plan");

        assert_eq!(
            EARLY_INSTRUCTION_CACHE_ENABLE_KIND,
            "el2-stage1-icache-enabled"
        );
        assert_eq!(plan.current_el, 2);
        assert_eq!(plan.sctlr_before, sctlr_with_m);
        assert_eq!(plan.sctlr_set, EARLY_TRANSLATION_SCTLR_I_ENABLE);

        assert_eq!(early_instruction_cache_enable_plan(1, sctlr_with_m), None);
        assert_eq!(
            early_instruction_cache_enable_plan(
                2,
                sctlr_with_m & !EARLY_TRANSLATION_SCTLR_M_ENABLE
            ),
            None
        );
    }

    #[test_case]
    fn data_cache_enable_plan_requires_el2_active_mmu_and_icache() {
        let sctlr_with_m_i = 0x30c5_1831;
        let plan = early_data_cache_enable_plan(2, sctlr_with_m_i).expect("data cache enable plan");

        assert_eq!(EARLY_DATA_CACHE_ENABLE_KIND, "el2-stage1-dcache-enabled");
        assert_eq!(plan.current_el, 2);
        assert_eq!(plan.sctlr_before, sctlr_with_m_i);
        assert_eq!(plan.sctlr_set, EARLY_TRANSLATION_SCTLR_C_ENABLE);

        assert_eq!(early_data_cache_enable_plan(1, sctlr_with_m_i), None);
        assert_eq!(
            early_data_cache_enable_plan(2, sctlr_with_m_i & !EARLY_TRANSLATION_SCTLR_M_ENABLE),
            None
        );
        assert_eq!(
            early_data_cache_enable_plan(2, sctlr_with_m_i & !EARLY_TRANSLATION_SCTLR_I_ENABLE),
            None
        );
    }

    #[test_case]
    fn translation_table_descriptors_encode_table_normal_and_device_entries() {
        assert_eq!(table_descriptor(0x2f00_1000), Some(0x2f00_1003));
        assert_eq!(table_descriptor(0x2f00_1001), None);

        let normal = normal_block_descriptor(0x3fe0_0000).expect("normal descriptor");
        assert_eq!(normal & 0x3, STAGE1_DESC_VALID);
        assert_eq!(normal & STAGE1_BLOCK_ADDR_MASK, 0x3fe0_0000);
        assert_eq!(
            normal & (0b111 << STAGE1_DESC_ATTR_INDEX_SHIFT),
            EARLY_TRANSLATION_NORMAL_ATTR_INDEX << STAGE1_DESC_ATTR_INDEX_SHIFT
        );
        assert_eq!(normal & STAGE1_DESC_SH_INNER, STAGE1_DESC_SH_INNER);
        assert_eq!(normal & STAGE1_DESC_AF, STAGE1_DESC_AF);

        let device = device_block_descriptor(0x10_7c00_0000).expect("device descriptor");
        assert_eq!(device & 0x3, STAGE1_DESC_VALID);
        assert_eq!(device & STAGE1_BLOCK_ADDR_MASK, 0x10_7c00_0000);
        assert_eq!(
            device & (0b111 << STAGE1_DESC_ATTR_INDEX_SHIFT),
            EARLY_TRANSLATION_DEVICE_ATTR_INDEX << STAGE1_DESC_ATTR_INDEX_SHIFT
        );
        assert_eq!(device & STAGE1_DESC_PXN, STAGE1_DESC_PXN);
        assert_eq!(device & STAGE1_DESC_UXN, STAGE1_DESC_UXN);
    }

    #[test_case]
    fn populate_translation_tables_writes_expected_entries_and_counts() {
        #[repr(align(4096))]
        struct TablePage([u64; 512]);

        let mut root = TablePage([0xdead_beef; 512]);
        let mut l1 = TablePage([0xdead_beef; 512]);
        let mut low_l2 = TablePage([0xdead_beef; 512]);
        let mut mmio_l2 = TablePage([0xdead_beef; 512]);
        let layout = EarlyTranslationTableLayout {
            start: core::ptr::addr_of!(root) as u64,
            end: core::ptr::addr_of!(mmio_l2) as u64 + EARLY_PAGE_SIZE,
            page_size: EARLY_PAGE_SIZE,
            page_count: EARLY_TRANSLATION_TABLE_PAGES,
            root_table: core::ptr::addr_of_mut!(root) as u64,
            l1_table: core::ptr::addr_of_mut!(l1) as u64,
            low_l2_table: core::ptr::addr_of_mut!(low_l2) as u64,
            mmio_l2_table: core::ptr::addr_of_mut!(mmio_l2) as u64,
        };

        let population =
            unsafe { populate_early_translation_tables(layout) }.expect("populate tables");

        assert_eq!(population.root_entries, 1);
        assert_eq!(
            root.0[0],
            table_descriptor(core::ptr::addr_of!(l1) as u64).expect("l1 descriptor")
        );
        assert_eq!(
            l1.0[EARLY_TRANSLATION_LOW_L1_INDEX as usize],
            table_descriptor(core::ptr::addr_of!(low_l2) as u64).expect("low l2 descriptor")
        );
        assert_eq!(
            l1.0[EARLY_TRANSLATION_BCM2712_MMIO_L1_INDEX as usize],
            table_descriptor(core::ptr::addr_of!(mmio_l2) as u64).expect("mmio l2 descriptor")
        );
        assert_eq!(
            low_l2.0[0],
            normal_block_descriptor(0).expect("low block 0")
        );
        assert_eq!(
            low_l2.0[511],
            normal_block_descriptor(0x3fe0_0000).expect("low block 511")
        );
        assert_eq!(mmio_l2.0[479], 0);
        assert_eq!(
            mmio_l2.0[480],
            device_block_descriptor(0x10_7c00_0000).expect("mmio first block")
        );
        assert_eq!(
            mmio_l2.0[511],
            device_block_descriptor(0x10_7fe0_0000).expect("mmio last block")
        );
    }
}
