#![cfg_attr(not(any(test, talos_target_rpi5_bcm2712)), allow(dead_code))]

mod common;
mod layout;
mod page_frames;
mod translation;

#[allow(unused_imports)]
pub use layout::{
    EARLY_USABLE_ALIGNMENT, EARLY_USABLE_POLICY, EarlyUsableMemory, FdtBlobRange, KernelLayout,
    conservative_low_memory_candidate,
};
#[allow(unused_imports)]
pub use page_frames::{
    EARLY_BOOTSTRAP_ALLOCATOR_KIND, EARLY_BOOTSTRAP_RESERVE_PAGES, EARLY_BOOTSTRAP_RESERVE_REASON,
    EARLY_BOOTSTRAP_SLACK_RESERVED_KIND, EARLY_PAGE_FRAME_ALLOCATOR_OWNED_KIND,
    EARLY_PAGE_FRAME_DEFERRED_KIND, EARLY_PAGE_FRAME_RESERVED_KIND, EARLY_PAGE_SIZE,
    EarlyBootstrapAllocatorPlan, EarlyBootstrapPageReservation, EarlyPageFrameOwnershipContract,
    EarlyPageFrameSeed, EarlyPageFrameSpan, early_bootstrap_allocator_plan,
    early_bootstrap_page_reservation, early_page_frame_ownership_contract,
    early_page_frame_seed_span,
};
#[allow(unused_imports)]
pub use translation::{
    EARLY_DATA_CACHE_ENABLE_KIND, EARLY_INSTRUCTION_CACHE_ENABLE_KIND,
    EARLY_TRANSLATION_BCM2712_MMIO_END, EARLY_TRANSLATION_BCM2712_MMIO_L1_INDEX,
    EARLY_TRANSLATION_BCM2712_MMIO_START, EARLY_TRANSLATION_DEVICE_ATTR_INDEX,
    EARLY_TRANSLATION_ENABLE_KIND, EARLY_TRANSLATION_L2_BLOCK_SIZE, EARLY_TRANSLATION_LOW_L1_INDEX,
    EARLY_TRANSLATION_LOW_MAP_END, EARLY_TRANSLATION_LOW_MAP_START,
    EARLY_TRANSLATION_MAIR_DEVICE_NGNRE, EARLY_TRANSLATION_MAIR_NORMAL_WBWA,
    EARLY_TRANSLATION_NORMAL_ATTR_INDEX, EARLY_TRANSLATION_REGISTER_PLAN_KIND,
    EARLY_TRANSLATION_ROOT_INDEX, EARLY_TRANSLATION_SCTLR_C_ENABLE,
    EARLY_TRANSLATION_SCTLR_I_ENABLE, EARLY_TRANSLATION_SCTLR_M_ENABLE,
    EARLY_TRANSLATION_TABLE_KIND, EARLY_TRANSLATION_TABLE_PAGES,
    EARLY_TRANSLATION_TABLE_POPULATION_KIND, EarlyDataCacheEnablePlan,
    EarlyInstructionCacheEnablePlan, EarlyTranslationRegisterPlan, EarlyTranslationTableLayout,
    EarlyTranslationTablePopulation, early_data_cache_enable_plan,
    early_instruction_cache_enable_plan, early_translation_register_plan,
    early_translation_table_layout, early_translation_table_population_plan,
    populate_early_translation_tables,
};
