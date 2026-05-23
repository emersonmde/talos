use crate::{allocator, memory_map, println, target};

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
unsafe extern "C" {
    static __kernel_start: u8;
    static __kernel_end: u8;
    static __heap_start: u8;
    static __heap_end: u8;
    static __stack_bottom: u8;
    static __stack_top: u8;
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
pub(crate) fn write_rpi5_chosen_bootargs_line(bootargs: &str) {
    const CHUNK_BYTES: usize = 16;

    target::console::write_static("talos: dtb chosen bootargs: ");

    let mut start = 0;
    while start < bootargs.len() {
        let mut end = core::cmp::min(start + CHUNK_BYTES, bootargs.len());
        while !bootargs.is_char_boundary(end) {
            end -= 1;
        }

        target::console::write_static(&bootargs[start..end]);
        target::rpi5::wait_uart10_empty_early_phase();
        start = end;
    }

    target::console::write_static("\n");
    target::rpi5::wait_uart10_empty_early_phase();
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
pub(crate) fn write_rpi5_dtb_reserved_summary_line(count: usize, shown: usize, truncated: bool) {
    target::console::write_static("talos: dtb reserved: count=");
    target::console::write_dec_usize(count);
    target::console::write_static(" shown=");
    target::console::write_dec_usize(shown);
    target::console::write_static(" truncated=");
    write_rpi5_bool(truncated);
    target::console::write_static("\n");
    target::rpi5::wait_uart10_empty_early_phase();
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
pub(crate) fn write_rpi5_dtb_reserved_entry_line(index: usize, address: u64, size: u64) {
    target::console::write_static("talos: dtb reserved[");
    target::console::write_dec_usize(index);
    target::console::write_static("]: addr=");
    target::console::write_hex_u64(address);
    target::console::write_static(" size=");
    target::console::write_hex_u64(size);
    target::console::write_static("\n");
    target::rpi5::wait_uart10_empty_early_phase();
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
pub(crate) fn write_rpi5_dtb_memory_summary_line(
    address_cells: usize,
    size_cells: usize,
    count: usize,
    shown: usize,
    truncated: bool,
) {
    println!(
        "talos: dtb memory: address_cells={} size_cells={} count={} shown={} truncated={}",
        address_cells, size_cells, count, shown, truncated
    );
    target::rpi5::wait_uart10_empty_early_phase();
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
pub(crate) fn write_rpi5_dtb_memory_entry_line(index: usize, address: u64, size: u64) {
    println!(
        "talos: dtb memory[{}]: addr={:#x} size={:#x}",
        index, address, size
    );
    target::rpi5::wait_uart10_empty_early_phase();
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
pub(crate) fn write_rpi5_reserved_memory_summary_line(
    address_cells: usize,
    size_cells: usize,
    node_count: usize,
    range_count: usize,
    shown: usize,
    truncated: bool,
) {
    target::console::write_static("talos: reserved-memory: addr_cells=");
    target::console::write_dec_usize(address_cells);
    target::console::write_static(" size_cells=");
    target::console::write_dec_usize(size_cells);
    target::console::write_static(" nodes=");
    target::console::write_dec_usize(node_count);
    target::console::write_static(" ranges=");
    target::console::write_dec_usize(range_count);
    target::console::write_static(" shown=");
    target::console::write_dec_usize(shown);
    target::console::write_static(" truncated=");
    write_rpi5_bool(truncated);
    target::console::write_static("\n");
    target::rpi5::wait_uart10_empty_early_phase();
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
pub(crate) fn write_rpi5_reserved_memory_entry_line(
    index: usize,
    address: u64,
    size: u64,
    no_map: bool,
    reusable: bool,
) {
    target::console::write_static("talos: reserved-memory[");
    target::console::write_dec_usize(index);
    target::console::write_static("]: addr=");
    target::console::write_hex_u64(address);
    target::console::write_static(" size=");
    target::console::write_hex_u64(size);
    target::console::write_static(" no_map=");
    write_rpi5_bool(no_map);
    target::console::write_static(" reusable=");
    write_rpi5_bool(reusable);
    target::console::write_static("\n");
    target::rpi5::wait_uart10_empty_early_phase();
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
pub(crate) fn rpi5_kernel_layout() -> memory_map::KernelLayout {
    memory_map::KernelLayout {
        start: core::ptr::addr_of!(__kernel_start) as u64,
        end: core::ptr::addr_of!(__kernel_end) as u64,
        heap_start: core::ptr::addr_of!(__heap_start) as u64,
        heap_end: core::ptr::addr_of!(__heap_end) as u64,
        stack_bottom: core::ptr::addr_of!(__stack_bottom) as u64,
        stack_top: core::ptr::addr_of!(__stack_top) as u64,
    }
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
pub(crate) fn write_rpi5_memory_layout_kernel_line(layout: memory_map::KernelLayout) {
    target::console::write_static("talos: memory layout: kernel=");
    target::console::write_hex_u64(layout.start);
    target::console::write_static("..");
    target::console::write_hex_u64(layout.end);
    target::console::write_static(" heap=");
    target::console::write_hex_u64(layout.heap_start);
    target::console::write_static("..");
    target::console::write_hex_u64(layout.heap_end);
    target::console::write_static(" stack=");
    target::console::write_hex_u64(layout.stack_bottom);
    target::console::write_static("..");
    target::console::write_hex_u64(layout.stack_top);
    target::console::write_static("\n");
    target::rpi5::wait_uart10_empty_early_phase();
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
pub(crate) fn write_rpi5_memory_layout_dtb_line(dtb: memory_map::FdtBlobRange) {
    target::console::write_static("talos: memory layout: dtb=");
    target::console::write_hex_u64(dtb.address);
    target::console::write_static("..");
    target::console::write_hex_u64(dtb.address + dtb.size);
    target::console::write_static(" size=");
    target::console::write_hex_u64(dtb.size);
    target::console::write_static("\n");
    target::rpi5::wait_uart10_empty_early_phase();
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
pub(crate) fn write_rpi5_memory_usable_candidate_line(candidate: memory_map::EarlyUsableMemory) {
    target::console::write_static("talos: memory usable: bank=");
    target::console::write_dec_usize(candidate.bank_index);
    target::console::write_static(" start=");
    target::console::write_hex_u64(candidate.start);
    target::console::write_static(" end=");
    target::console::write_hex_u64(candidate.end);
    target::console::write_static(" size=");
    target::console::write_hex_u64(candidate.size);
    target::console::write_static(" align=");
    target::console::write_hex_u64(candidate.alignment);
    target::console::write_static(" policy=");
    target::console::write_static(memory_map::EARLY_USABLE_POLICY);
    target::console::write_static("\n");
    target::rpi5::wait_uart10_empty_early_phase();
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
pub(crate) fn write_rpi5_memory_usable_candidate_println_line(
    candidate: memory_map::EarlyUsableMemory,
) {
    println!(
        "talos: memory usable: bank={} start={:#x} end={:#x} size={:#x} align={:#x} policy={}",
        candidate.bank_index,
        candidate.start,
        candidate.end,
        candidate.size,
        candidate.alignment,
        memory_map::EARLY_USABLE_POLICY
    );
    target::rpi5::wait_uart10_empty_early_phase();
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
pub(crate) fn write_rpi5_page_frame_seed_line(seed: memory_map::EarlyPageFrameSeed) {
    target::console::write_static("talos: page frames seed: start=");
    target::console::write_hex_u64(seed.start);
    target::console::write_static(" end=");
    target::console::write_hex_u64(seed.end);
    target::console::write_static(" pages=");
    target::console::write_hex_u64(seed.page_count);
    target::console::write_static(" page_size=");
    target::console::write_hex_u64(seed.page_size);
    target::console::write_static(" source=memory-usable\n");
    target::rpi5::wait_uart10_empty_early_phase();
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
pub(crate) fn write_rpi5_bootstrap_page_reservation_early_line(
    reservation: memory_map::EarlyBootstrapPageReservation,
) {
    target::console::write_static("talos: bootstrap reserve: start=");
    target::console::write_hex_u64(reservation.start);
    target::console::write_static(" end=");
    target::console::write_hex_u64(reservation.end);
    target::console::write_static(" pages=");
    target::console::write_hex_u64(reservation.page_count);
    target::console::write_static(" page_size=");
    target::console::write_hex_u64(reservation.page_size);
    target::console::write_static(" reason=");
    target::console::write_static(memory_map::EARLY_BOOTSTRAP_RESERVE_REASON);
    target::console::write_static("\n");
    target::rpi5::wait_uart10_empty_early_phase();
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
pub(crate) fn write_rpi5_bootstrap_page_reservation_line(
    reservation: memory_map::EarlyBootstrapPageReservation,
) {
    println!(
        "talos: bootstrap reserve: start={:#x} end={:#x} pages={:#x} page_size={:#x} reason={}",
        reservation.start,
        reservation.end,
        reservation.page_count,
        reservation.page_size,
        memory_map::EARLY_BOOTSTRAP_RESERVE_REASON
    );
    target::rpi5::wait_uart10_empty_early_phase();
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
pub(crate) fn write_rpi5_translation_table_layout_line(
    layout: memory_map::EarlyTranslationTableLayout,
) {
    target::console::write_static("talos: translation tables: start=");
    target::console::write_hex_u64(layout.start);
    target::console::write_static(" end=");
    target::console::write_hex_u64(layout.end);
    target::console::write_static(" pages=");
    target::console::write_hex_u64(layout.page_count);
    target::console::write_static(" page_size=");
    target::console::write_hex_u64(layout.page_size);
    target::console::write_static(" kind=");
    target::console::write_static(memory_map::EARLY_TRANSLATION_TABLE_KIND);
    target::console::write_static("\n");
    target::rpi5::wait_uart10_empty_early_phase();
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
pub(crate) fn write_rpi5_translation_table_layout_post_allocator_line(
    reservation: memory_map::EarlyBootstrapPageReservation,
) {
    let table_bytes = memory_map::EARLY_TRANSLATION_TABLE_PAGES * reservation.page_size;
    let table_end = reservation.start + table_bytes;
    println!(
        "talos: translation tables: start={:#x} end={:#x} pages={:#x} page_size={:#x} kind=layout-only phase=post-allocator",
        reservation.start,
        table_end,
        memory_map::EARLY_TRANSLATION_TABLE_PAGES,
        reservation.page_size,
    );
    target::rpi5::wait_uart10_empty_early_phase();
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
pub(crate) fn write_rpi5_translation_table_slots_line(
    layout: memory_map::EarlyTranslationTableLayout,
) {
    target::console::write_static("talos: translation table slots: root=");
    target::console::write_hex_u64(layout.root_table);
    target::console::write_static(" l1=");
    target::console::write_hex_u64(layout.l1_table);
    target::console::write_static(" l2_low=");
    target::console::write_hex_u64(layout.low_l2_table);
    target::console::write_static(" l2_mmio=");
    target::console::write_hex_u64(layout.mmio_l2_table);
    target::console::write_static("\n");
    target::rpi5::wait_uart10_empty_early_phase();
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
pub(crate) fn write_rpi5_translation_table_slots_post_allocator_line(
    reservation: memory_map::EarlyBootstrapPageReservation,
) {
    if let Some(layout) = memory_map::early_translation_table_layout(reservation) {
        println!(
            "talos: translation table slots: root={:#x} l1={:#x} l2_low={:#x} l2_mmio={:#x} phase=post-allocator",
            layout.root_table, layout.l1_table, layout.low_l2_table, layout.mmio_l2_table,
        );
    } else {
        target::console::write_static(
            "talos: translation table slots: unavailable phase=post-allocator\n",
        );
    }
    target::rpi5::wait_uart10_empty_early_phase();
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
pub(crate) fn write_rpi5_translation_table_population_post_allocator_line(
    reservation: memory_map::EarlyBootstrapPageReservation,
) {
    if let Some(layout) = memory_map::early_translation_table_layout(reservation) {
        if let Some(population) = memory_map::early_translation_table_population_plan(layout) {
            println!(
                "talos: translation table population: root_entries={:#x} l1_entries={:#x} low_l2_blocks={:#x} mmio_l2_blocks={:#x} block_size={:#x} kind={} phase=post-allocator",
                population.root_entries,
                population.l1_entries,
                population.low_l2_blocks,
                population.mmio_l2_blocks,
                population.block_size,
                memory_map::EARLY_TRANSLATION_TABLE_POPULATION_KIND
            );
        } else {
            target::console::write_static(
                "talos: translation table population: unavailable phase=post-allocator\n",
            );
        }
    } else {
        target::console::write_static(
            "talos: translation table population: unavailable phase=post-allocator\n",
        );
    }
    target::rpi5::wait_uart10_empty_early_phase();
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
pub(crate) fn write_rpi5_translation_table_population_line(
    population: memory_map::EarlyTranslationTablePopulation,
) {
    target::console::write_static("talos: translation table ");
    target::console::write_static("population: ");
    target::console::write_static("root_entries=");
    target::console::write_hex_u64(population.root_entries);
    target::console::write_static(" l1_entries=");
    target::console::write_hex_u64(population.l1_entries);
    target::console::write_static(" low_l2_blocks=");
    target::console::write_hex_u64(population.low_l2_blocks);
    target::console::write_static(" mmio_l2_blocks=");
    target::console::write_hex_u64(population.mmio_l2_blocks);
    target::console::write_static(" block_size=");
    target::console::write_hex_u64(population.block_size);
    target::console::write_static(" kind=");
    target::console::write_static(memory_map::EARLY_TRANSLATION_TABLE_POPULATION_KIND);
    target::console::write_static("\n");
    target::rpi5::wait_uart10_empty_early_phase();
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
pub(crate) fn write_rpi5_translation_table_policy_line(
    population: memory_map::EarlyTranslationTablePopulation,
) {
    target::console::write_static("talos: translation map ");
    target::console::write_static("policy: low=");
    target::console::write_hex_u64(population.low_map_start);
    target::console::write_static("..");
    target::console::write_hex_u64(population.low_map_end);
    target::console::write_static(" mmio=");
    target::console::write_hex_u64(population.mmio_map_start);
    target::console::write_static("..");
    target::console::write_hex_u64(population.mmio_map_end);
    target::console::write_static(" root_index=");
    target::console::write_hex_u64(population.root_index);
    target::console::write_static(" low_l1_index=");
    target::console::write_hex_u64(population.low_l1_index);
    target::console::write_static(" mmio_l1_index=");
    target::console::write_hex_u64(population.mmio_l1_index);
    target::console::write_static(" normal_attr=");
    target::console::write_hex_u64(memory_map::EARLY_TRANSLATION_NORMAL_ATTR_INDEX);
    target::console::write_static(" device_attr=");
    target::console::write_hex_u64(memory_map::EARLY_TRANSLATION_DEVICE_ATTR_INDEX);
    target::console::write_static("\n");
    target::rpi5::wait_uart10_empty_early_phase();
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
pub(crate) fn write_rpi5_translation_register_plan_line(
    plan: memory_map::EarlyTranslationRegisterPlan,
) {
    target::console::write_static("talos: translation control ");
    target::console::write_static("plan: el=");
    target::console::write_hex_u64(plan.current_el as u64);
    target::console::write_static(" mair=");
    target::console::write_hex_u64(plan.mair);
    target::console::write_static(" tcr=");
    target::console::write_hex_u64(plan.tcr);
    target::console::write_static(" ttbr0=");
    target::console::write_hex_u64(plan.ttbr0);
    target::console::write_static(" sctlr_set=");
    target::console::write_hex_u64(plan.sctlr_set);
    target::console::write_static(" va_bits=");
    target::console::write_hex_u64(plan.va_bits);
    target::console::write_static(" pa_bits=");
    target::console::write_hex_u64(plan.pa_bits);
    target::console::write_static(" kind=");
    target::console::write_static(memory_map::EARLY_TRANSLATION_REGISTER_PLAN_KIND);
    target::console::write_static("\n");
    target::rpi5::wait_uart10_empty_early_phase();
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
pub(crate) fn write_rpi5_translation_enabled_line(
    plan: memory_map::EarlyTranslationRegisterPlan,
    sctlr: u64,
) {
    target::console::write_static("talos: translation enabled: ");
    target::console::write_static("el=");
    target::console::write_hex_u64(plan.current_el as u64);
    target::console::write_static(" sctlr=");
    target::console::write_hex_u64(sctlr);
    target::console::write_static(" ttbr0=");
    target::console::write_hex_u64(plan.ttbr0);
    target::console::write_static(" kind=");
    target::console::write_static(memory_map::EARLY_TRANSLATION_ENABLE_KIND);
    target::console::write_static("\n");
    target::rpi5::wait_uart10_empty_early_phase();
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
pub(crate) fn write_rpi5_instruction_cache_plan_line(
    plan: memory_map::EarlyInstructionCacheEnablePlan,
) {
    target::console::write_static("talos: instruction cache plan: ");
    target::console::write_static("el=");
    target::console::write_hex_u64(plan.current_el as u64);
    target::console::write_static(" sctlr_before=");
    target::console::write_hex_u64(plan.sctlr_before);
    target::console::write_static(" sctlr_set=");
    target::console::write_hex_u64(plan.sctlr_set);
    target::console::write_static(" kind=");
    target::console::write_static(memory_map::EARLY_INSTRUCTION_CACHE_ENABLE_KIND);
    target::console::write_static("\n");
    target::rpi5::wait_uart10_empty_early_phase();
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
pub(crate) fn write_rpi5_instruction_cache_enabled_line(
    plan: memory_map::EarlyInstructionCacheEnablePlan,
    sctlr: u64,
) {
    target::console::write_static("talos: instruction cache enabled: ");
    target::console::write_static("el=");
    target::console::write_hex_u64(plan.current_el as u64);
    target::console::write_static(" sctlr=");
    target::console::write_hex_u64(sctlr);
    target::console::write_static(" kind=");
    target::console::write_static(memory_map::EARLY_INSTRUCTION_CACHE_ENABLE_KIND);
    target::console::write_static("\n");
    target::rpi5::wait_uart10_empty_early_phase();
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
pub(crate) fn write_rpi5_data_cache_plan_line(plan: memory_map::EarlyDataCacheEnablePlan) {
    target::console::write_static("talos: data cache plan: ");
    target::console::write_static("el=");
    target::console::write_hex_u64(plan.current_el as u64);
    target::console::write_static(" sctlr_before=");
    target::console::write_hex_u64(plan.sctlr_before);
    target::console::write_static(" sctlr_set=");
    target::console::write_hex_u64(plan.sctlr_set);
    target::console::write_static(" kind=");
    target::console::write_static(memory_map::EARLY_DATA_CACHE_ENABLE_KIND);
    target::console::write_static("\n");
    target::rpi5::wait_uart10_empty_early_phase();
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
pub(crate) fn write_rpi5_data_cache_enabled_line(
    plan: memory_map::EarlyDataCacheEnablePlan,
    sctlr: u64,
) {
    println!(
        "talos: data cache enabled: el={:#x} sctlr={:#x} kind={}",
        plan.current_el as u64,
        sctlr,
        memory_map::EARLY_DATA_CACHE_ENABLE_KIND
    );
    target::rpi5::wait_uart10_empty_early_phase();
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
pub(crate) fn write_rpi5_bootstrap_allocator_plan_line(
    plan: memory_map::EarlyBootstrapAllocatorPlan,
) {
    println!(
        "talos: bootstrap allocator plan: start={:#x} end={:#x} bytes={:#x} pages={:#x} page_size={:#x} kind={}",
        plan.start,
        plan.end,
        plan.size,
        plan.page_count,
        plan.page_size,
        memory_map::EARLY_BOOTSTRAP_ALLOCATOR_KIND
    );
    target::rpi5::wait_uart10_empty_early_phase();
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
pub(crate) fn write_rpi5_bootstrap_allocator_init_line(state: allocator::BumpAllocatorState) {
    println!(
        "talos: bootstrap allocator init: start={:#x} next={:#x} end={:#x} policy=no-free",
        state.start, state.next, state.end
    );
    target::rpi5::wait_uart10_empty_early_phase();
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
pub(crate) fn write_rpi5_page_frame_remaining_line(seed: memory_map::EarlyPageFrameSeed) {
    println!(
        "talos: page frames remaining: start={:#x} end={:#x} pages={:#x} page_size={:#x} source=bootstrap-reserve",
        seed.start, seed.end, seed.page_count, seed.page_size
    );
    target::rpi5::wait_uart10_empty_early_phase();
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
pub(crate) fn write_rpi5_bool(value: bool) {
    if value {
        target::console::write_static("true");
    } else {
        target::console::write_static("false");
    }
}
