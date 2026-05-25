use super::{BootInfo, rpi5_reports::*};
use crate::device_tree::{
    FdtHeader, FdtMemoryBanks, FdtMemoryReservations, FdtReservedMemoryRanges,
};
use crate::target::TargetServices;
use crate::{KERNEL_GLOBAL_ALLOCATOR, arch, diagnostics, memory_map, println, target};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Rpi5DtbPhase {
    reservations: Option<FdtMemoryReservations>,
    reserved_memory_ranges: Option<FdtReservedMemoryRanges>,
    memory_banks: Option<FdtMemoryBanks>,
    blob: Option<memory_map::FdtBlobRange>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Rpi5MemoryPhase {
    candidate: memory_map::EarlyUsableMemory,
    seed: memory_map::EarlyPageFrameSeed,
    reservation: memory_map::EarlyBootstrapPageReservation,
    layout: Option<memory_map::EarlyTranslationTableLayout>,
}

#[inline(always)]
fn suppress_growth_diagnostic_boot_reports() -> bool {
    cfg!(any(
        talos_rpi5_vec_growth_diagnostic,
        talos_rpi5_string_growth_diagnostic,
        talos_rpi5_alloc_format_diagnostic
    ))
}

fn report_unavailable(line: &'static str) {
    target::console::write_static(line);
    target::rpi5::wait_uart10_empty_early_phase();
}

#[cfg_attr(
    any(
        talos_rpi5_panic_report_diagnostic,
        talos_rpi5_full_panic_info_diagnostic,
        talos_rpi5_normal_exception_report_diagnostic,
        talos_rpi5_undefined_instruction_report_diagnostic,
        talos_rpi5_data_abort_report_diagnostic,
        talos_rpi5_translation_fault_diagnostic,
        talos_rpi5_current_sp0_sync_diagnostic,
    ),
    allow(unreachable_code, unused_variables)
)]
pub(crate) fn kernel_main(boot_info: &BootInfo) -> ! {
    target::rpi5::write_early_phase_line(target::rpi5::EarlyPhaseLine::KernelMain);

    let services = target::services(boot_info);
    let dtb = report_boot_identity(boot_info, &services);
    let memory_phase = plan_boot_memory(&dtb);

    if let Some(memory_phase) = memory_phase {
        let caches_enabled = memory_phase
            .layout
            .map(|layout| enable_translation_and_caches(boot_info, layout))
            .unwrap_or(false);
        if caches_enabled {
            init_bootstrap_allocator(memory_phase.reservation.remaining);
        }
        report_post_allocator_memory(memory_phase);
    }

    report_dtb_memory_banks(dtb.memory_banks);

    #[cfg(talos_rpi5_psci_secondary_core_alive_proof)]
    target::rpi5::run_psci_secondary_core_alive_proof();

    #[cfg(talos_rpi5_secondary_core_workload_proof)]
    target::rpi5::run_secondary_core_workload_proof();

    #[cfg(talos_rpi5_smp_lock_cache_coherence_proof)]
    target::rpi5::run_smp_lock_cache_coherence_proof();

    #[cfg(talos_rpi5_cross_core_ipi_delivery_proof)]
    target::rpi5::run_cross_core_ipi_delivery_proof();

    #[cfg(talos_rpi5_uart10_polling_rx_diagnostic)]
    target::rpi5::run_uart10_polling_tty_rx_diagnostic();

    #[cfg(talos_rpi5_diagnostic_command_channel_proof)]
    target::rpi5::run_diagnostic_command_channel_proof();

    #[cfg(talos_rpi5_timer_preemption_diagnostic)]
    target::rpi5::run_el2_timer_preemption_smoke();

    #[cfg(all(
        talos_rpi5_timer_irq_diagnostic,
        not(talos_rpi5_timer_preemption_diagnostic)
    ))]
    target::rpi5::run_el2_timer_irq_smoke();

    unsafe { diagnostics::rpi5::run_exception_fault_panic_diagnostics() }

    arch::aarch64::halt()
}

fn report_boot_identity(boot_info: &BootInfo, services: &TargetServices) -> Rpi5DtbPhase {
    println!("\ntalos: boot start");
    println!("talos: board raspberry-pi-5-bcm2712");
    println!("talos: version {}", env!("CARGO_PKG_VERSION"));

    println!("talos: console early-uart fmt");
    println!("talos: boot core {}", boot_info.primary_core as usize);
    println!("talos: boot dtb {:#x}", boot_info.dtb_pa);
    println!("talos: boot target {}", boot_info.target.name());
    println!(
        "talos: pointer delta {:#x}",
        target::rpi5::runtime_relocation_delta()
    );

    println!(
        "talos: boot info: dtb={:#x} core={} el={} target={}",
        boot_info.dtb_pa,
        boot_info.primary_core as usize,
        boot_info.exception_level as usize,
        boot_info.target.name()
    );

    if let Some(services_dtb_pa) = services.device_tree.physical_address() {
        println!(
            "talos: services: uart={} timer={} irq={} mmio_regions={} dtb={:#x}",
            services.uart.name(),
            services.timer.name(),
            services.interrupt_controller.name(),
            services.mmio_map.regions().len(),
            services_dtb_pa
        );
    } else {
        println!(
            "talos: services: uart={} timer={} irq={} mmio_regions={} dtb=none",
            services.uart.name(),
            services.timer.name(),
            services.interrupt_controller.name(),
            services.mmio_map.regions().len()
        );
    }

    let dtb_header = unsafe { services.device_tree.fdt_header() };
    if let Some(dtb_header) = dtb_header {
        println!(
            "talos: dtb header: magic={:#x} size={} version={} last_comp={} struct={} strings={}",
            dtb_header.magic,
            dtb_header.total_size as usize,
            dtb_header.version as usize,
            dtb_header.last_comp_version as usize,
            dtb_header.size_dt_struct as usize,
            dtb_header.size_dt_strings as usize
        );
    } else {
        println!("talos: dtb header: unavailable");
    }

    target::rpi5::wait_uart10_empty_early_phase();

    report_chosen_bootargs(services);

    println!("talos: status early boot log ready");

    let reservations = scan_dtb_reservations(services);
    let reserved_memory_ranges = scan_reserved_memory_ranges(services);
    let memory_banks = scan_memory_banks(services);
    let blob = dtb_blob_range(services, dtb_header);

    Rpi5DtbPhase {
        reservations,
        reserved_memory_ranges,
        memory_banks,
        blob,
    }
}

fn report_chosen_bootargs(services: &TargetServices) {
    #[cfg(not(talos_rpi5_translation_fault_diagnostic))]
    {
        if let Some(chosen_bootargs) = unsafe { services.device_tree.chosen_bootargs() } {
            write_rpi5_chosen_bootargs_line(chosen_bootargs);
        } else {
            println!("talos: dtb chosen bootargs: unavailable");
        }
    }
    #[cfg(talos_rpi5_translation_fault_diagnostic)]
    target::console::write_static(
        "talos: dtb chosen bootargs: skipped=translation-fault-diagnostic\n",
    );
}

fn scan_dtb_reservations(services: &TargetServices) -> Option<FdtMemoryReservations> {
    target::rpi5::write_early_phase_line(target::rpi5::EarlyPhaseLine::DtbReservationsStart);
    #[cfg(talos_rpi5_translation_fault_diagnostic)]
    let dtb_reservations = None;
    #[cfg(not(talos_rpi5_translation_fault_diagnostic))]
    let dtb_reservations = {
        let dtb_reservations = unsafe { services.device_tree.memory_reservations() };
        #[cfg(not(any(
            talos_rpi5_vec_growth_diagnostic,
            talos_rpi5_string_growth_diagnostic,
            talos_rpi5_alloc_format_diagnostic
        )))]
        if let Some(dtb_reservations) = dtb_reservations {
            let shown = dtb_reservations.reported_len();
            write_rpi5_dtb_reserved_summary_line(
                dtb_reservations.count,
                shown,
                dtb_reservations.truncated,
            );

            let mut index = 0usize;
            while index < shown {
                if let Some(entry) = dtb_reservations.entries[index] {
                    write_rpi5_dtb_reserved_entry_line(index, entry.address, entry.size);
                }
                index += 1;
            }
        } else {
            target::console::write_static("talos: dtb reserved: unavailable\n");
            target::rpi5::wait_uart10_empty_early_phase();
        }
        dtb_reservations
    };
    target::rpi5::write_early_phase_line(target::rpi5::EarlyPhaseLine::DtbReservationsDone);
    dtb_reservations
}

fn scan_reserved_memory_ranges(services: &TargetServices) -> Option<FdtReservedMemoryRanges> {
    target::console::write_static("TALOS: reserved-memory start\n");
    target::rpi5::wait_uart10_empty_early_phase();
    #[cfg(talos_rpi5_translation_fault_diagnostic)]
    let reserved_memory_ranges = None;
    #[cfg(not(talos_rpi5_translation_fault_diagnostic))]
    let reserved_memory_ranges = unsafe { services.device_tree.reserved_memory_ranges() };
    target::console::write_static("TALOS: reserved-memory done\n");
    target::rpi5::wait_uart10_empty_early_phase();

    if !suppress_growth_diagnostic_boot_reports() {
        #[cfg(not(talos_rpi5_translation_fault_diagnostic))]
        if let Some(reserved_memory_ranges) = reserved_memory_ranges {
            let shown = reserved_memory_ranges.reported_len();
            write_rpi5_reserved_memory_summary_line(
                reserved_memory_ranges.address_cells as usize,
                reserved_memory_ranges.size_cells as usize,
                reserved_memory_ranges.node_count,
                reserved_memory_ranges.range_count,
                shown,
                reserved_memory_ranges.truncated,
            );

            let mut index = 0usize;
            while index < shown {
                if let Some(entry) = reserved_memory_ranges.entries[index] {
                    write_rpi5_reserved_memory_entry_line(
                        index,
                        entry.address,
                        entry.size,
                        entry.no_map,
                        entry.reusable,
                    );
                }
                index += 1;
            }
        } else {
            report_unavailable("talos: reserved-memory: unavailable\n");
        }
    }
    reserved_memory_ranges
}

fn scan_memory_banks(services: &TargetServices) -> Option<FdtMemoryBanks> {
    target::rpi5::write_early_phase_line(target::rpi5::EarlyPhaseLine::DtbMemoryScanStart);
    let memory_banks = unsafe { services.device_tree.memory_banks() };
    target::rpi5::write_early_phase_line(target::rpi5::EarlyPhaseLine::DtbMemoryScanDone);
    memory_banks
}

fn dtb_blob_range(
    services: &TargetServices,
    dtb_header: Option<FdtHeader>,
) -> Option<memory_map::FdtBlobRange> {
    dtb_header
        .and_then(|header| {
            services
                .device_tree
                .physical_address()
                .map(|address| (header, address))
        })
        .map(|(header, address)| memory_map::FdtBlobRange {
            address: address as u64,
            size: header.total_size as u64,
        })
}

fn plan_boot_memory(dtb: &Rpi5DtbPhase) -> Option<Rpi5MemoryPhase> {
    let memory_banks = if let Some(memory_banks) = dtb.memory_banks {
        memory_banks
    } else {
        report_unavailable("talos: dtb memory: unavailable\n");
        return None;
    };

    let kernel_layout = rpi5_kernel_layout();
    if !suppress_growth_diagnostic_boot_reports() {
        write_rpi5_memory_layout_kernel_line(kernel_layout);
    }

    if !suppress_growth_diagnostic_boot_reports() {
        if let Some(dtb_blob) = dtb.blob {
            write_rpi5_memory_layout_dtb_line(dtb_blob);
        } else {
            report_unavailable("talos: memory layout: dtb=unavailable\n");
        }
    }

    let candidate = if let Some(candidate) = memory_map::conservative_low_memory_candidate(
        &memory_banks,
        dtb.reservations.as_ref(),
        dtb.reserved_memory_ranges.as_ref(),
        dtb.blob,
        kernel_layout,
    ) {
        candidate
    } else {
        report_unavailable("talos: memory usable: unavailable\n");
        return None;
    };
    if !suppress_growth_diagnostic_boot_reports() {
        write_rpi5_memory_usable_candidate_line(candidate);
    }

    let seed = if let Some(seed) = memory_map::early_page_frame_seed_span(candidate) {
        seed
    } else {
        report_unavailable("talos: page frames seed: unavailable\n");
        return None;
    };
    if !suppress_growth_diagnostic_boot_reports() {
        write_rpi5_page_frame_seed_line(seed);
    }

    let reservation = if let Some(reservation) = memory_map::early_bootstrap_page_reservation(
        seed,
        memory_map::EARLY_BOOTSTRAP_RESERVE_PAGES,
    ) {
        reservation
    } else {
        report_unavailable("talos: bootstrap reserve: unavailable\n");
        return None;
    };
    if !suppress_growth_diagnostic_boot_reports() {
        write_rpi5_bootstrap_page_reservation_early_line(reservation);
    }

    let layout = if let Some(layout) = memory_map::early_translation_table_layout(reservation) {
        layout
    } else {
        report_unavailable("talos: translation tables: unavailable\n");
        return Some(Rpi5MemoryPhase {
            candidate,
            seed,
            reservation,
            layout: None,
        });
    };
    if !suppress_growth_diagnostic_boot_reports() {
        write_rpi5_translation_table_layout_line(layout);
        write_rpi5_translation_table_slots_line(layout);
    }

    Some(Rpi5MemoryPhase {
        candidate,
        seed,
        reservation,
        layout: Some(layout),
    })
}

fn enable_translation_and_caches(
    boot_info: &BootInfo,
    layout: memory_map::EarlyTranslationTableLayout,
) -> bool {
    let population = if let Some(population) =
        unsafe { memory_map::populate_early_translation_tables(layout) }
    {
        population
    } else {
        report_unavailable("talos: translation table population: unavailable\n");
        return false;
    };
    if !suppress_growth_diagnostic_boot_reports() {
        write_rpi5_translation_table_population_line(population);
        write_rpi5_translation_table_policy_line(population);
    }

    let register_plan = if let Some(register_plan) =
        memory_map::early_translation_register_plan(layout, boot_info.exception_level)
    {
        register_plan
    } else {
        report_unavailable("talos: translation control plan: unavailable\n");
        return false;
    };
    if !suppress_growth_diagnostic_boot_reports() {
        write_rpi5_translation_register_plan_line(register_plan);
    }

    target::rpi5::write_early_phase_line(target::rpi5::EarlyPhaseLine::MmuEnableStart);
    let sctlr =
        if let Some(sctlr) = unsafe { arch::aarch64::enable_el2_mmu_from_plan(register_plan) } {
            sctlr
        } else {
            report_unavailable("talos: translation enable: unavailable\n");
            return false;
        };
    target::rpi5::write_early_phase_line(target::rpi5::EarlyPhaseLine::MmuEnableDone);
    if !suppress_growth_diagnostic_boot_reports() {
        write_rpi5_translation_enabled_line(register_plan, sctlr);
    }

    #[cfg(talos_rpi5_translation_fault_diagnostic)]
    {
        unsafe { diagnostics::rpi5::rpi5_translation_fault_diagnostic() }
    }

    #[cfg(not(talos_rpi5_translation_fault_diagnostic))]
    {
        enable_instruction_and_data_caches(boot_info, sctlr)
    }
}

#[cfg(not(talos_rpi5_translation_fault_diagnostic))]
fn enable_instruction_and_data_caches(boot_info: &BootInfo, sctlr: u64) -> bool {
    let icache_plan = if let Some(icache_plan) =
        memory_map::early_instruction_cache_enable_plan(boot_info.exception_level, sctlr)
    {
        icache_plan
    } else {
        report_unavailable("talos: instruction cache plan: unavailable\n");
        return false;
    };
    if !suppress_growth_diagnostic_boot_reports() {
        write_rpi5_instruction_cache_plan_line(icache_plan);
    }

    target::rpi5::write_early_phase_line(target::rpi5::EarlyPhaseLine::IcacheEnableStart);
    let icache_sctlr = if let Some(icache_sctlr) =
        unsafe { arch::aarch64::enable_el2_instruction_cache_from_plan(icache_plan) }
    {
        icache_sctlr
    } else {
        report_unavailable("talos: instruction cache enable: unavailable\n");
        return false;
    };
    target::rpi5::write_early_phase_line(target::rpi5::EarlyPhaseLine::IcacheEnableDone);
    if !suppress_growth_diagnostic_boot_reports() {
        write_rpi5_instruction_cache_enabled_line(icache_plan, icache_sctlr);
    }

    let dcache_plan = if let Some(dcache_plan) =
        memory_map::early_data_cache_enable_plan(boot_info.exception_level, icache_sctlr)
    {
        dcache_plan
    } else {
        report_unavailable("talos: data cache plan: unavailable\n");
        return false;
    };
    if !suppress_growth_diagnostic_boot_reports() {
        write_rpi5_data_cache_plan_line(dcache_plan);
    }

    target::console::write_static("TALOS: dcache enable start\n");
    target::rpi5::wait_uart10_empty_early_phase();
    let dcache_sctlr = if let Some(dcache_sctlr) =
        unsafe { arch::aarch64::enable_el2_data_cache_from_plan(dcache_plan) }
    {
        dcache_sctlr
    } else {
        report_unavailable("talos: data cache enable: unavailable\n");
        return false;
    };
    target::console::write_static("TALOS: dcache enable done\n");
    target::rpi5::wait_uart10_empty_early_phase();
    if !suppress_growth_diagnostic_boot_reports() {
        write_rpi5_data_cache_enabled_line(dcache_plan, dcache_sctlr);
    }

    true
}

fn init_bootstrap_allocator(seed: memory_map::EarlyPageFrameSeed) {
    let allocator_plan =
        if let Some(allocator_plan) = memory_map::early_bootstrap_allocator_plan(seed) {
            allocator_plan
        } else {
            report_unavailable("talos: bootstrap allocator plan: unavailable\n");
            return;
        };
    if !suppress_growth_diagnostic_boot_reports() {
        write_rpi5_bootstrap_allocator_plan_line(allocator_plan);
    }

    if let Some(allocator_state) = KERNEL_GLOBAL_ALLOCATOR.init_from_plan(allocator_plan) {
        write_rpi5_bootstrap_allocator_init_line(allocator_state);
        diagnostics::rpi5::run_allocator_diagnostic_or_smoke(allocator_plan);
    } else {
        report_unavailable("talos: bootstrap allocator init: unavailable\n");
    }
}

fn report_post_allocator_memory(memory_phase: Rpi5MemoryPhase) {
    if suppress_growth_diagnostic_boot_reports() {
        return;
    }

    write_rpi5_translation_table_layout_post_allocator_line(memory_phase.reservation);
    write_rpi5_translation_table_slots_post_allocator_line(memory_phase.reservation);
    write_rpi5_translation_table_population_post_allocator_line(memory_phase.reservation);
    write_rpi5_memory_usable_candidate_println_line(memory_phase.candidate);
    println!(
        "talos: page frames seed: start={:#x} end={:#x} pages={:#x} page_size={:#x} source=memory-usable phase=post-allocator",
        memory_phase.seed.start,
        memory_phase.seed.end,
        memory_phase.seed.page_count,
        memory_phase.seed.page_size,
    );
    target::rpi5::wait_uart10_empty_early_phase();
    write_rpi5_bootstrap_page_reservation_line(memory_phase.reservation);
    write_rpi5_page_frame_remaining_line(memory_phase.reservation.remaining);
}

fn report_dtb_memory_banks(memory_banks: Option<FdtMemoryBanks>) {
    if suppress_growth_diagnostic_boot_reports() {
        return;
    }

    let memory_banks = if let Some(memory_banks) = memory_banks {
        memory_banks
    } else {
        return;
    };
    let shown = memory_banks.reported_len();
    write_rpi5_dtb_memory_summary_line(
        memory_banks.address_cells as usize,
        memory_banks.size_cells as usize,
        memory_banks.count,
        shown,
        memory_banks.truncated,
    );

    let mut index = 0usize;
    while index < shown {
        if let Some(bank) = memory_banks.entries[index] {
            write_rpi5_dtb_memory_entry_line(index, bank.address, bank.size);
        }
        index += 1;
    }
}
