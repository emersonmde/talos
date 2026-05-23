use super::{BootInfo, rpi5_reports::*};
use crate::{KERNEL_GLOBAL_ALLOCATOR, arch, diagnostics, memory_map, println, target};

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

    let services = target::services(boot_info);

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

    println!("talos: status early boot log ready");

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

    target::console::write_static("TALOS: reserved-memory start\n");
    target::rpi5::wait_uart10_empty_early_phase();
    #[cfg(talos_rpi5_translation_fault_diagnostic)]
    let reserved_memory_ranges = None;
    #[cfg(not(talos_rpi5_translation_fault_diagnostic))]
    let reserved_memory_ranges = unsafe { services.device_tree.reserved_memory_ranges() };
    target::console::write_static("TALOS: reserved-memory done\n");
    target::rpi5::wait_uart10_empty_early_phase();

    #[cfg(not(any(
        talos_rpi5_translation_fault_diagnostic,
        talos_rpi5_vec_growth_diagnostic,
        talos_rpi5_string_growth_diagnostic,
        talos_rpi5_alloc_format_diagnostic
    )))]
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
        target::console::write_static("talos: reserved-memory: unavailable\n");
        target::rpi5::wait_uart10_empty_early_phase();
    }

    target::rpi5::write_early_phase_line(target::rpi5::EarlyPhaseLine::DtbMemoryScanStart);
    let memory_banks = unsafe { services.device_tree.memory_banks() };
    target::rpi5::write_early_phase_line(target::rpi5::EarlyPhaseLine::DtbMemoryScanDone);

    if let Some(memory_banks) = memory_banks {
        let shown = memory_banks.reported_len();

        let kernel_layout = rpi5_kernel_layout();
        #[cfg(not(any(
            talos_rpi5_vec_growth_diagnostic,
            talos_rpi5_string_growth_diagnostic,
            talos_rpi5_alloc_format_diagnostic
        )))]
        write_rpi5_memory_layout_kernel_line(kernel_layout);

        let dtb_blob = dtb_header
            .and_then(|header| {
                services
                    .device_tree
                    .physical_address()
                    .map(|address| (header, address))
            })
            .map(|(header, address)| memory_map::FdtBlobRange {
                address: address as u64,
                size: header.total_size as u64,
            });

        #[cfg(not(any(
            talos_rpi5_vec_growth_diagnostic,
            talos_rpi5_string_growth_diagnostic,
            talos_rpi5_alloc_format_diagnostic
        )))]
        if let Some(dtb_blob) = dtb_blob {
            write_rpi5_memory_layout_dtb_line(dtb_blob);
        } else {
            target::console::write_static("talos: memory layout: dtb=unavailable\n");
            target::rpi5::wait_uart10_empty_early_phase();
        }

        if let Some(candidate) = memory_map::conservative_low_memory_candidate(
            &memory_banks,
            dtb_reservations.as_ref(),
            reserved_memory_ranges.as_ref(),
            dtb_blob,
            kernel_layout,
        ) {
            #[cfg(not(any(
                talos_rpi5_vec_growth_diagnostic,
                talos_rpi5_string_growth_diagnostic,
                talos_rpi5_alloc_format_diagnostic
            )))]
            write_rpi5_memory_usable_candidate_line(candidate);
            if let Some(seed) = memory_map::early_page_frame_seed_span(candidate) {
                #[cfg(not(any(
                    talos_rpi5_vec_growth_diagnostic,
                    talos_rpi5_string_growth_diagnostic,
                    talos_rpi5_alloc_format_diagnostic
                )))]
                write_rpi5_page_frame_seed_line(seed);
                if let Some(reservation) = memory_map::early_bootstrap_page_reservation(
                    seed,
                    memory_map::EARLY_BOOTSTRAP_RESERVE_PAGES,
                ) {
                    #[cfg(not(any(
                        talos_rpi5_vec_growth_diagnostic,
                        talos_rpi5_string_growth_diagnostic,
                        talos_rpi5_alloc_format_diagnostic
                    )))]
                    write_rpi5_bootstrap_page_reservation_early_line(reservation);
                    if let Some(layout) = memory_map::early_translation_table_layout(reservation) {
                        #[cfg(not(any(
                            talos_rpi5_vec_growth_diagnostic,
                            talos_rpi5_string_growth_diagnostic,
                            talos_rpi5_alloc_format_diagnostic
                        )))]
                        write_rpi5_translation_table_layout_line(layout);
                        #[cfg(not(any(
                            talos_rpi5_vec_growth_diagnostic,
                            talos_rpi5_string_growth_diagnostic,
                            talos_rpi5_alloc_format_diagnostic
                        )))]
                        write_rpi5_translation_table_slots_line(layout);
                        if let Some(population) =
                            unsafe { memory_map::populate_early_translation_tables(layout) }
                        {
                            #[cfg(not(any(
                                talos_rpi5_vec_growth_diagnostic,
                                talos_rpi5_string_growth_diagnostic,
                                talos_rpi5_alloc_format_diagnostic
                            )))]
                            write_rpi5_translation_table_population_line(population);
                            #[cfg(not(any(
                                talos_rpi5_vec_growth_diagnostic,
                                talos_rpi5_string_growth_diagnostic,
                                talos_rpi5_alloc_format_diagnostic
                            )))]
                            write_rpi5_translation_table_policy_line(population);
                            if let Some(register_plan) = memory_map::early_translation_register_plan(
                                layout,
                                boot_info.exception_level,
                            ) {
                                #[cfg(not(any(
                                    talos_rpi5_vec_growth_diagnostic,
                                    talos_rpi5_string_growth_diagnostic,
                                    talos_rpi5_alloc_format_diagnostic
                                )))]
                                write_rpi5_translation_register_plan_line(register_plan);
                                target::rpi5::write_early_phase_line(
                                    target::rpi5::EarlyPhaseLine::MmuEnableStart,
                                );
                                if let Some(sctlr) = unsafe {
                                    arch::aarch64::enable_el2_mmu_from_plan(register_plan)
                                } {
                                    target::rpi5::write_early_phase_line(
                                        target::rpi5::EarlyPhaseLine::MmuEnableDone,
                                    );
                                    #[cfg(not(any(
                                        talos_rpi5_vec_growth_diagnostic,
                                        talos_rpi5_string_growth_diagnostic,
                                        talos_rpi5_alloc_format_diagnostic
                                    )))]
                                    write_rpi5_translation_enabled_line(register_plan, sctlr);
                                    #[cfg(not(talos_rpi5_translation_fault_diagnostic))]
                                    if let Some(icache_plan) =
                                        memory_map::early_instruction_cache_enable_plan(
                                            boot_info.exception_level,
                                            sctlr,
                                        )
                                    {
                                        #[cfg(not(any(
                                            talos_rpi5_vec_growth_diagnostic,
                                            talos_rpi5_string_growth_diagnostic,
                                            talos_rpi5_alloc_format_diagnostic
                                        )))]
                                        write_rpi5_instruction_cache_plan_line(icache_plan);
                                        target::rpi5::write_early_phase_line(
                                            target::rpi5::EarlyPhaseLine::IcacheEnableStart,
                                        );
                                        if let Some(icache_sctlr) = unsafe {
                                            arch::aarch64::enable_el2_instruction_cache_from_plan(
                                                icache_plan,
                                            )
                                        } {
                                            target::rpi5::write_early_phase_line(
                                                target::rpi5::EarlyPhaseLine::IcacheEnableDone,
                                            );
                                            #[cfg(not(any(
                                                talos_rpi5_vec_growth_diagnostic,
                                                talos_rpi5_string_growth_diagnostic,
                                                talos_rpi5_alloc_format_diagnostic
                                            )))]
                                            write_rpi5_instruction_cache_enabled_line(
                                                icache_plan,
                                                icache_sctlr,
                                            );
                                            if let Some(dcache_plan) =
                                                memory_map::early_data_cache_enable_plan(
                                                    boot_info.exception_level,
                                                    icache_sctlr,
                                                )
                                            {
                                                #[cfg(not(any(
                                                    talos_rpi5_vec_growth_diagnostic,
                                                    talos_rpi5_string_growth_diagnostic,
                                                    talos_rpi5_alloc_format_diagnostic
                                                )))]
                                                write_rpi5_data_cache_plan_line(dcache_plan);
                                                target::console::write_static(
                                                    "TALOS: dcache enable start\n",
                                                );
                                                target::rpi5::wait_uart10_empty_early_phase();
                                                if let Some(dcache_sctlr) = unsafe {
                                                    arch::aarch64::enable_el2_data_cache_from_plan(
                                                        dcache_plan,
                                                    )
                                                } {
                                                    target::console::write_static(
                                                        "TALOS: dcache enable done\n",
                                                    );
                                                    target::rpi5::wait_uart10_empty_early_phase();
                                                    #[cfg(not(any(
                                                        talos_rpi5_vec_growth_diagnostic,
                                                        talos_rpi5_string_growth_diagnostic,
                                                        talos_rpi5_alloc_format_diagnostic
                                                    )))]
                                                    write_rpi5_data_cache_enabled_line(
                                                        dcache_plan,
                                                        dcache_sctlr,
                                                    );
                                                    if let Some(allocator_plan) =
                                                        memory_map::early_bootstrap_allocator_plan(
                                                            reservation.remaining,
                                                        )
                                                    {
                                                        #[cfg(not(any(
                                                            talos_rpi5_vec_growth_diagnostic,
                                                            talos_rpi5_string_growth_diagnostic,
                                                            talos_rpi5_alloc_format_diagnostic
                                                        )))]
                                                        write_rpi5_bootstrap_allocator_plan_line(
                                                            allocator_plan,
                                                        );
                                                        if let Some(allocator_state) =
                                                            KERNEL_GLOBAL_ALLOCATOR
                                                                .init_from_plan(allocator_plan)
                                                        {
                                                            write_rpi5_bootstrap_allocator_init_line(
                                                                allocator_state,
                                                            );
                                                            diagnostics::rpi5::run_allocator_diagnostic_or_smoke();
                                                        } else {
                                                            target::console::write_static(
                                                                "talos: bootstrap allocator init: unavailable\n",
                                                            );
                                                            target::rpi5::wait_uart10_empty_early_phase(
                                                            );
                                                        }
                                                    } else {
                                                        target::console::write_static(
                                                            "talos: bootstrap allocator plan: unavailable\n",
                                                        );
                                                        target::rpi5::wait_uart10_empty_early_phase(
                                                        );
                                                    }
                                                } else {
                                                    target::console::write_static(
                                                        "talos: data cache enable: unavailable\n",
                                                    );
                                                    target::rpi5::wait_uart10_empty_early_phase();
                                                }
                                            } else {
                                                target::console::write_static(
                                                    "talos: data cache plan: unavailable\n",
                                                );
                                                target::rpi5::wait_uart10_empty_early_phase();
                                            }
                                        } else {
                                            target::console::write_static(
                                                "talos: instruction cache enable: unavailable\n",
                                            );
                                            target::rpi5::wait_uart10_empty_early_phase();
                                        }
                                    } else {
                                        target::console::write_static(
                                            "talos: instruction cache plan: unavailable\n",
                                        );
                                        target::rpi5::wait_uart10_empty_early_phase();
                                    }
                                    #[cfg(talos_rpi5_translation_fault_diagnostic)]
                                    unsafe {
                                        diagnostics::rpi5::rpi5_translation_fault_diagnostic();
                                    }
                                } else {
                                    target::console::write_static(
                                        "talos: translation enable: unavailable\n",
                                    );
                                    target::rpi5::wait_uart10_empty_early_phase();
                                }
                            } else {
                                target::console::write_static(
                                    "talos: translation control plan: unavailable\n",
                                );
                                target::rpi5::wait_uart10_empty_early_phase();
                            }
                        } else {
                            target::console::write_static(
                                "talos: translation table population: unavailable\n",
                            );
                            target::rpi5::wait_uart10_empty_early_phase();
                        }
                    } else {
                        target::console::write_static("talos: translation tables: unavailable\n");
                        target::rpi5::wait_uart10_empty_early_phase();
                    }
                    #[cfg(not(any(
                        talos_rpi5_vec_growth_diagnostic,
                        talos_rpi5_string_growth_diagnostic,
                        talos_rpi5_alloc_format_diagnostic
                    )))]
                    write_rpi5_translation_table_layout_post_allocator_line(reservation);
                    #[cfg(not(any(
                        talos_rpi5_vec_growth_diagnostic,
                        talos_rpi5_string_growth_diagnostic,
                        talos_rpi5_alloc_format_diagnostic
                    )))]
                    write_rpi5_translation_table_slots_post_allocator_line(reservation);
                    #[cfg(not(any(
                        talos_rpi5_vec_growth_diagnostic,
                        talos_rpi5_string_growth_diagnostic,
                        talos_rpi5_alloc_format_diagnostic
                    )))]
                    write_rpi5_translation_table_population_post_allocator_line(reservation);
                    #[cfg(not(any(
                        talos_rpi5_vec_growth_diagnostic,
                        talos_rpi5_string_growth_diagnostic,
                        talos_rpi5_alloc_format_diagnostic
                    )))]
                    write_rpi5_memory_usable_candidate_println_line(candidate);
                    #[cfg(not(any(
                        talos_rpi5_vec_growth_diagnostic,
                        talos_rpi5_string_growth_diagnostic,
                        talos_rpi5_alloc_format_diagnostic
                    )))]
                    println!(
                        "talos: page frames seed: start={:#x} end={:#x} pages={:#x} page_size={:#x} source=memory-usable phase=post-allocator",
                        seed.start, seed.end, seed.page_count, seed.page_size,
                    );
                    #[cfg(not(any(
                        talos_rpi5_vec_growth_diagnostic,
                        talos_rpi5_string_growth_diagnostic,
                        talos_rpi5_alloc_format_diagnostic
                    )))]
                    target::rpi5::wait_uart10_empty_early_phase();
                    #[cfg(not(any(
                        talos_rpi5_vec_growth_diagnostic,
                        talos_rpi5_string_growth_diagnostic,
                        talos_rpi5_alloc_format_diagnostic
                    )))]
                    write_rpi5_bootstrap_page_reservation_line(reservation);
                    #[cfg(not(any(
                        talos_rpi5_vec_growth_diagnostic,
                        talos_rpi5_string_growth_diagnostic,
                        talos_rpi5_alloc_format_diagnostic
                    )))]
                    write_rpi5_page_frame_remaining_line(reservation.remaining);
                } else {
                    target::console::write_static("talos: bootstrap reserve: unavailable\n");
                    target::rpi5::wait_uart10_empty_early_phase();
                }
            } else {
                target::console::write_static("talos: page frames seed: unavailable\n");
                target::rpi5::wait_uart10_empty_early_phase();
            }
        } else {
            target::console::write_static("talos: memory usable: unavailable\n");
            target::rpi5::wait_uart10_empty_early_phase();
        }

        #[cfg(not(any(
            talos_rpi5_vec_growth_diagnostic,
            talos_rpi5_string_growth_diagnostic,
            talos_rpi5_alloc_format_diagnostic
        )))]
        write_rpi5_dtb_memory_summary_line(
            memory_banks.address_cells as usize,
            memory_banks.size_cells as usize,
            memory_banks.count,
            shown,
            memory_banks.truncated,
        );

        #[cfg(not(any(
            talos_rpi5_vec_growth_diagnostic,
            talos_rpi5_string_growth_diagnostic,
            talos_rpi5_alloc_format_diagnostic
        )))]
        let mut index = 0usize;
        #[cfg(not(any(
            talos_rpi5_vec_growth_diagnostic,
            talos_rpi5_string_growth_diagnostic,
            talos_rpi5_alloc_format_diagnostic
        )))]
        while index < shown {
            if let Some(bank) = memory_banks.entries[index] {
                write_rpi5_dtb_memory_entry_line(index, bank.address, bank.size);
            }
            index += 1;
        }
    } else {
        target::console::write_static("talos: dtb memory: unavailable\n");
        target::rpi5::wait_uart10_empty_early_phase();
    }

    unsafe { diagnostics::rpi5::run_exception_fault_panic_diagnostics() }

    arch::aarch64::halt()
}
