use super::common::is_aligned;
use super::page_frames::{EARLY_PAGE_SIZE, EarlyBootstrapPageReservation};

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

pub(super) const STAGE1_DESC_VALID: u64 = 1 << 0;
const STAGE1_DESC_TABLE: u64 = 1 << 1;
pub(super) const STAGE1_DESC_ATTR_INDEX_SHIFT: u64 = 2;
pub(super) const STAGE1_DESC_SH_INNER: u64 = 0b11 << 8;
pub(super) const STAGE1_DESC_AF: u64 = 1 << 10;
pub(super) const STAGE1_DESC_PXN: u64 = 1 << 53;
pub(super) const STAGE1_DESC_UXN: u64 = 1 << 54;
const STAGE1_TABLE_ADDR_MASK: u64 = 0x0000_ffff_ffff_f000;
pub(super) const STAGE1_BLOCK_ADDR_MASK: u64 = 0x0000_ffff_ffe0_0000;
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

pub(super) fn table_descriptor(address: u64) -> Option<u64> {
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

pub(super) fn normal_block_descriptor(address: u64) -> Option<u64> {
    block_descriptor(
        address,
        EARLY_TRANSLATION_NORMAL_ATTR_INDEX,
        STAGE1_DESC_SH_INNER,
    )
}

pub(super) fn device_block_descriptor(address: u64) -> Option<u64> {
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

#[cfg(test)]
mod tests {
    use super::super::page_frames::{
        EARLY_BOOTSTRAP_RESERVE_PAGES, EARLY_PAGE_SIZE, EarlyPageFrameSeed,
        early_bootstrap_page_reservation,
    };
    use super::*;

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
