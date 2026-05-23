#![no_std]
#![no_main]
#![cfg_attr(
    all(not(test), talos_target_rpi5_bcm2712),
    feature(alloc_error_handler)
)]
#![cfg_attr(
    all(
        talos_target_rpi5_bcm2712,
        any(
            talos_rpi5_vec_growth_diagnostic,
            talos_rpi5_string_growth_diagnostic,
            talos_rpi5_alloc_format_diagnostic
        )
    ),
    allow(dead_code, unused_variables)
)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
extern crate alloc;

mod allocator;
mod arch;
mod boot;
mod device_tree;
mod early_format;
mod memory_map;
mod mmio;
mod pl011;
mod target;

use core::panic::PanicInfo;
#[cfg(talos_target_rpi5_bcm2712)]
struct PanicInProgress(core::cell::UnsafeCell<bool>);

use boot::BootInfo;

#[cfg_attr(not(test), global_allocator)]
#[cfg_attr(not(talos_target_rpi5_bcm2712), allow(dead_code))]
static KERNEL_GLOBAL_ALLOCATOR: allocator::BumpAllocator = allocator::BumpAllocator::new();

#[cfg(talos_target_rpi5_bcm2712)]
unsafe impl Sync for PanicInProgress {}

#[cfg(talos_target_rpi5_bcm2712)]
impl PanicInProgress {
    const fn new() -> Self {
        Self(core::cell::UnsafeCell::new(false))
    }

    #[cfg(talos_rpi5_nested_panic_diagnostic)]
    fn prearm(&self) {
        unsafe {
            core::ptr::write_volatile(self.0.get(), true);
        }
    }

    fn enter(&self) -> bool {
        unsafe {
            let was_in_progress = core::ptr::read_volatile(self.0.get());
            if !was_in_progress {
                core::ptr::write_volatile(self.0.get(), true);
            }
            was_in_progress
        }
    }
}

#[cfg(talos_target_rpi5_bcm2712)]
static PANIC_IN_PROGRESS: PanicInProgress = PanicInProgress::new();

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
#[alloc_error_handler]
fn alloc_error(layout: core::alloc::Layout) -> ! {
    target::console::write_static("talos: alloc error: size=");
    target::console::write_hex_u64(layout.size() as u64);
    target::console::write_static(" align=");
    target::console::write_hex_u64(layout.align() as u64);
    target::console::write_static("\n");
    target::rpi5::wait_uart10_empty_early_phase();
    loop {
        core::hint::spin_loop();
    }
}

#[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_exception_return_diagnostic))]
unsafe extern "C" {
    fn rpi5_brk_register_preserve_probe(after_x9: *mut u64, after_x19: *mut u64) -> u64;
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
unsafe extern "C" {
    static __kernel_start: u8;
    static __kernel_end: u8;
    static __heap_start: u8;
    static __heap_end: u8;
    static __stack_bottom: u8;
    static __stack_top: u8;
}

#[unsafe(no_mangle)]
#[cfg_attr(
    any(
        all(talos_target_rpi5_bcm2712, talos_rpi5_rust_entry_diagnostic),
        all(talos_target_rpi5_bcm2712, talos_rpi5_runtime_uart_probe_diagnostic),
        all(talos_target_rpi5_bcm2712, talos_rpi5_handoff_uart_diagnostic),
        all(talos_target_rpi5_bcm2712, talos_rpi5_rust_uart10_diagnostic),
        all(talos_target_rpi5_bcm2712, talos_rpi5_println_phase_diagnostic),
        all(talos_target_rpi5_bcm2712, talos_rpi5_rodata_address_diagnostic),
        all(
            talos_target_rpi5_bcm2712,
            talos_rpi5_static_format_boundary_diagnostic
        ),
        all(talos_target_rpi5_bcm2712, talos_rpi5_boundary_entry_reset_diagnostic),
        all(talos_target_rpi5_bcm2712, talos_rpi5_phase_ladder_diagnostic),
        all(
            talos_target_rpi5_bcm2712,
            talos_rpi5_phase_stack_to_rust_reset_diagnostic
        ),
    ),
    allow(unreachable_code, unused_variables)
)]
pub extern "C" fn rust_entry(dtb_pa: usize) -> ! {
    #[cfg(talos_target_rpi5_bcm2712)]
    target::rpi5::write_early_phase_line(target::rpi5::EarlyPhaseLine::RustEntry);

    #[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_println_phase_diagnostic))]
    target::rpi5::println_phase_marker(b'0');

    #[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_rodata_address_diagnostic))]
    target::rpi5::rodata_address_diagnostic();

    #[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_rust_uart10_diagnostic))]
    target::rpi5::rust_uart10_diagnostic();

    #[cfg(all(
        talos_target_rpi5_bcm2712,
        talos_rpi5_static_format_boundary_diagnostic
    ))]
    target::rpi5::static_format_boundary_diagnostic();

    #[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_boundary_entry_reset_diagnostic))]
    target::rpi5::boundary_entry_reset_diagnostic();

    #[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_phase_ladder_diagnostic))]
    target::rpi5::phase_ladder_diagnostic();

    #[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_handoff_uart_diagnostic))]
    target::rpi5::handoff_uart_diagnostic();

    #[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_runtime_uart_probe_diagnostic))]
    target::rpi5::runtime_uart_probe_diagnostic();

    let boot_info = BootInfo::from_aarch64_x0(dtb_pa);

    #[cfg(talos_target_rpi5_bcm2712)]
    target::rpi5::write_early_phase_line(target::rpi5::EarlyPhaseLine::BootInfoParsed);

    #[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_println_phase_diagnostic))]
    target::rpi5::println_phase_marker(b'1');

    target::init(&boot_info);

    #[cfg(talos_target_rpi5_bcm2712)]
    target::rpi5::write_early_phase_line(target::rpi5::EarlyPhaseLine::TargetInit);

    #[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_println_phase_diagnostic))]
    target::rpi5::println_phase_marker(b'2');

    arch::aarch64::exceptions::init();

    #[cfg(talos_target_rpi5_bcm2712)]
    target::rpi5::write_early_phase_line(target::rpi5::EarlyPhaseLine::ExceptionsReady);

    #[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_println_phase_diagnostic))]
    target::rpi5::println_phase_marker(b'3');

    #[cfg(test)]
    {
        test_main();
        target::qemu::exit_success();
    }

    #[cfg(not(test))]
    kernel_main(&boot_info)
}

#[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_rust_entry_diagnostic))]
pub(crate) fn rpi5_rust_entry_reset_probe() -> ! {
    unsafe {
        core::arch::asm!(
            "movz x13, #0x003c",
            "movk x13, #0x000f, lsl #16",
            "movk x13, #0x001f, lsl #32",
            "mov w12, #0x40",
            "str w12, [x13]",
            "ldr w12, [x13]",
            "movz x13, #0x0040",
            "movk x13, #0x000f, lsl #16",
            "movk x13, #0x001f, lsl #32",
            "mov w12, #0x48",
            "str w12, [x13]",
            "ldr w12, [x13]",
            "movz x13, #0x0074",
            "movk x13, #0x000d, lsl #16",
            "movk x13, #0x001f, lsl #32",
            "mov w12, #4",
            "str w12, [x13]",
            "ldr w12, [x13]",
            "movz x13, #0x007c",
            "movk x13, #0x000d, lsl #16",
            "movk x13, #0x001f, lsl #32",
            "mov w12, #4",
            "str w12, [x13]",
            "ldr w12, [x13]",
            "dsb sy",
            "movz x9, #0x1000",
            "movk x9, #0x7d00, lsl #16",
            "movk x9, #0x0010, lsl #32",
            "movz x14, #0x0000",
            "movk x14, #0x0003, lsl #16",
            "movk x14, #0x001f, lsl #32",
            "mov x20, #8",
            "2:",
            "mov w11, #0x52",
            "str w11, [x9]",
            "ldr w12, [x9, #0x18]",
            "str w11, [x14]",
            "ldr w12, [x14, #0x18]",
            "mov w11, #0x53",
            "str w11, [x9]",
            "ldr w12, [x9, #0x18]",
            "str w11, [x14]",
            "ldr w12, [x14, #0x18]",
            "mov w11, #0x0d",
            "str w11, [x9]",
            "ldr w12, [x9, #0x18]",
            "str w11, [x14]",
            "ldr w12, [x14, #0x18]",
            "mov w11, #0x0a",
            "str w11, [x9]",
            "ldr w12, [x9, #0x18]",
            "str w11, [x14]",
            "ldr w12, [x14, #0x18]",
            "dsb sy",
            "subs x20, x20, #1",
            "b.ne 2b",
            "3:",
            "ldr x0, =0x84000009",
            "smc #0",
            "wfe",
            "b 3b",
            options(noreturn)
        );
    }
}

#[cfg(all(
    talos_target_rpi5_bcm2712,
    talos_rpi5_phase_stack_to_rust_reset_diagnostic
))]
#[unsafe(no_mangle)]
pub extern "C" fn rpi5_stack_to_rust_reset_probe() -> ! {
    unsafe {
        core::arch::asm!(
            "movz x9, #0x1000",
            "movk x9, #0x7d00, lsl #16",
            "movk x9, #0x0010, lsl #32",
            "movz x14, #0x0000",
            "movk x14, #0x0003, lsl #16",
            "movk x14, #0x001f, lsl #32",
            "mov x20, #8",
            "2:",
            "mov w11, #0x54",
            "str w11, [x9]",
            "ldr w12, [x9, #0x18]",
            "str w11, [x14]",
            "ldr w12, [x14, #0x18]",
            "mov w11, #0x41",
            "str w11, [x9]",
            "ldr w12, [x9, #0x18]",
            "str w11, [x14]",
            "ldr w12, [x14, #0x18]",
            "mov w11, #0x4c",
            "str w11, [x9]",
            "ldr w12, [x9, #0x18]",
            "str w11, [x14]",
            "ldr w12, [x14, #0x18]",
            "mov w11, #0x4f",
            "str w11, [x9]",
            "ldr w12, [x9, #0x18]",
            "str w11, [x14]",
            "ldr w12, [x14, #0x18]",
            "mov w11, #0x53",
            "str w11, [x9]",
            "ldr w12, [x9, #0x18]",
            "str w11, [x14]",
            "ldr w12, [x14, #0x18]",
            "mov w11, #0x3a",
            "str w11, [x9]",
            "ldr w12, [x9, #0x18]",
            "str w11, [x14]",
            "ldr w12, [x14, #0x18]",
            "mov w11, #0x20",
            "str w11, [x9]",
            "ldr w12, [x9, #0x18]",
            "str w11, [x14]",
            "ldr w12, [x14, #0x18]",
            "mov w11, #0x72",
            "str w11, [x9]",
            "ldr w12, [x9, #0x18]",
            "str w11, [x14]",
            "ldr w12, [x14, #0x18]",
            "mov w11, #0x75",
            "str w11, [x9]",
            "ldr w12, [x9, #0x18]",
            "str w11, [x14]",
            "ldr w12, [x14, #0x18]",
            "mov w11, #0x73",
            "str w11, [x9]",
            "ldr w12, [x9, #0x18]",
            "str w11, [x14]",
            "ldr w12, [x14, #0x18]",
            "mov w11, #0x74",
            "str w11, [x9]",
            "ldr w12, [x9, #0x18]",
            "str w11, [x14]",
            "ldr w12, [x14, #0x18]",
            "mov w11, #0x20",
            "str w11, [x9]",
            "ldr w12, [x9, #0x18]",
            "str w11, [x14]",
            "ldr w12, [x14, #0x18]",
            "mov w11, #0x68",
            "str w11, [x9]",
            "ldr w12, [x9, #0x18]",
            "str w11, [x14]",
            "ldr w12, [x14, #0x18]",
            "mov w11, #0x61",
            "str w11, [x9]",
            "ldr w12, [x9, #0x18]",
            "str w11, [x14]",
            "ldr w12, [x14, #0x18]",
            "mov w11, #0x6e",
            "str w11, [x9]",
            "ldr w12, [x9, #0x18]",
            "str w11, [x14]",
            "ldr w12, [x14, #0x18]",
            "mov w11, #0x64",
            "str w11, [x9]",
            "ldr w12, [x9, #0x18]",
            "str w11, [x14]",
            "ldr w12, [x14, #0x18]",
            "mov w11, #0x6f",
            "str w11, [x9]",
            "ldr w12, [x9, #0x18]",
            "str w11, [x14]",
            "ldr w12, [x14, #0x18]",
            "mov w11, #0x66",
            "str w11, [x9]",
            "ldr w12, [x9, #0x18]",
            "str w11, [x14]",
            "ldr w12, [x14, #0x18]",
            "mov w11, #0x66",
            "str w11, [x9]",
            "ldr w12, [x9, #0x18]",
            "str w11, [x14]",
            "ldr w12, [x14, #0x18]",
            "mov w11, #0x0d",
            "str w11, [x9]",
            "ldr w12, [x9, #0x18]",
            "str w11, [x14]",
            "ldr w12, [x14, #0x18]",
            "mov w11, #0x0a",
            "str w11, [x9]",
            "ldr w12, [x9, #0x18]",
            "str w11, [x14]",
            "ldr w12, [x14, #0x18]",
            "dsb sy",
            "subs x20, x20, #1",
            "b.ne 2b",
            "ldr x0, =0x84000009",
            "smc #0",
            "wfe",
            "b .-4",
            options(noreturn)
        );
    }
}

#[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_minimal_format_diagnostic))]
fn rpi5_minimal_format_reset_probe() -> ! {
    unsafe {
        core::arch::asm!(
            "ldr x0, =0x84000009",
            "smc #0",
            "wfe",
            "b .-4",
            options(noreturn)
        );
    }
}

#[cfg_attr(
    any(
        all(talos_target_rpi5_bcm2712, talos_rpi5_minimal_format_diagnostic),
        all(talos_target_rpi5_bcm2712, talos_rpi5_fmt_sink_diagnostic),
        all(talos_target_rpi5_bcm2712, talos_rpi5_fmt_static_sink_diagnostic),
        all(talos_target_rpi5_bcm2712, talos_rpi5_fmt_sink_direct_diagnostic),
        all(talos_target_rpi5_bcm2712, talos_rpi5_fmt_sink_dyn_direct_diagnostic),
        all(talos_target_rpi5_bcm2712, talos_rpi5_fmt_sink_fnptr_direct_diagnostic),
        all(talos_target_rpi5_bcm2712, talos_rpi5_fnptr_reset_diagnostic),
        all(talos_target_rpi5_bcm2712, talos_rpi5_runtime_uart_probe_diagnostic),
        all(talos_target_rpi5_bcm2712, talos_rpi5_println_phase_diagnostic),
        all(talos_target_rpi5_bcm2712, talos_rpi5_panic_report_diagnostic),
        all(talos_target_rpi5_bcm2712, talos_rpi5_full_panic_info_diagnostic),
        all(
            talos_target_rpi5_bcm2712,
            talos_rpi5_normal_exception_report_diagnostic
        ),
        all(
            talos_target_rpi5_bcm2712,
            talos_rpi5_undefined_instruction_report_diagnostic
        ),
        all(talos_target_rpi5_bcm2712, talos_rpi5_data_abort_report_diagnostic),
        all(talos_target_rpi5_bcm2712, talos_rpi5_translation_fault_diagnostic),
        all(talos_target_rpi5_bcm2712, talos_rpi5_current_sp0_sync_diagnostic),
    ),
    allow(unreachable_code, unused_variables)
)]
#[cfg(not(test))]
fn kernel_main(boot_info: &BootInfo) -> ! {
    #[cfg(talos_target_rpi5_bcm2712)]
    {
        target::rpi5::write_early_phase_line(target::rpi5::EarlyPhaseLine::KernelMain);

        println!("\ntalos: boot start");
        println!("talos: board raspberry-pi-5-bcm2712");
        println!("talos: version {}", env!("CARGO_PKG_VERSION"));

        #[cfg(talos_rpi5_rust_entry_diagnostic)]
        crate::rpi5_rust_entry_reset_probe();

        println!("talos: console early-uart fmt");
        println!("talos: boot core {}", boot_info.primary_core as usize);
        println!("talos: boot dtb {:#x}", boot_info.dtb_pa);
        println!("talos: boot target {}", boot_info.target.name());
        println!(
            "talos: pointer delta {:#x}",
            target::rpi5::runtime_relocation_delta()
        );

        #[cfg(talos_rpi5_println_phase_diagnostic)]
        target::rpi5::println_phase_marker(b'4');

        #[cfg(talos_rpi5_println_phase_diagnostic)]
        rpi5_println_phase_diagnostic();

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
                        if let Some(layout) =
                            memory_map::early_translation_table_layout(reservation)
                        {
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
                                if let Some(register_plan) =
                                    memory_map::early_translation_register_plan(
                                        layout,
                                        boot_info.exception_level,
                                    )
                                {
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
                                                        target::rpi5::wait_uart10_empty_early_phase(
                                                        );
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
                                                                #[cfg(talos_rpi5_alloc_oom_diagnostic)]
                                                                rpi5_alloc_oom_diagnostic();
                                                                #[cfg(all(
                                                                    not(talos_rpi5_alloc_oom_diagnostic),
                                                                    talos_rpi5_realloc_growth_diagnostic
                                                                ))]
                                                                rpi5_realloc_growth_diagnostic();
                                                                #[cfg(all(
                                                                    not(talos_rpi5_alloc_oom_diagnostic),
                                                                    not(talos_rpi5_realloc_growth_diagnostic),
                                                                    talos_rpi5_vec_growth_diagnostic
                                                                ))]
                                                                rpi5_vec_growth_diagnostic();
                                                                #[cfg(all(
                                                                    not(talos_rpi5_alloc_oom_diagnostic),
                                                                    not(talos_rpi5_realloc_growth_diagnostic),
                                                                    not(talos_rpi5_vec_growth_diagnostic),
                                                                    talos_rpi5_string_growth_diagnostic
                                                                ))]
                                                                rpi5_string_growth_diagnostic();
                                                                #[cfg(all(
                                                                    not(talos_rpi5_alloc_oom_diagnostic),
                                                                    not(talos_rpi5_realloc_growth_diagnostic),
                                                                    not(talos_rpi5_vec_growth_diagnostic),
                                                                    not(talos_rpi5_string_growth_diagnostic),
                                                                    talos_rpi5_alloc_format_diagnostic
                                                                ))]
                                                                rpi5_alloc_format_diagnostic();
                                                                #[cfg(not(any(
                                                                    talos_rpi5_alloc_oom_diagnostic,
                                                                    talos_rpi5_realloc_growth_diagnostic,
                                                                    talos_rpi5_vec_growth_diagnostic,
                                                                    talos_rpi5_string_growth_diagnostic,
                                                                    talos_rpi5_alloc_format_diagnostic
                                                                )))]
                                                                rpi5_bootstrap_alloc_smoke();
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
                                                        target::rpi5::wait_uart10_empty_early_phase(
                                                        );
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
                                            rpi5_translation_fault_diagnostic();
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
                            target::console::write_static(
                                "talos: translation tables: unavailable\n",
                            );
                            target::rpi5::wait_uart10_empty_early_phase();
                        }
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
                        write_rpi5_page_frame_seed_println_line(seed);
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

        #[cfg(talos_rpi5_undefined_instruction_report_diagnostic)]
        unsafe {
            target::console::write_static("TALOS: before undefined instruction\n");
            target::rpi5::wait_uart10_empty_early_phase();
            println!(
                "TALOS: before undefined instruction vbar={:#x} el={}",
                arch::aarch64::current_vbar(),
                arch::aarch64::current_el() as usize
            );
            target::rpi5::wait_uart10_empty_early_phase();
            core::arch::asm!("udf #0", options(nomem, nostack, preserves_flags));
        }

        #[cfg(talos_rpi5_data_abort_report_diagnostic)]
        unsafe {
            let probe = [0u64; 2];
            let unaligned_addr = core::ptr::addr_of!(probe) as usize + 1;

            target::console::write_static("TALOS: before alignment data abort addr=");
            target::console::write_hex_u64(unaligned_addr as u64);
            target::console::write_static(" vbar=");
            target::console::write_hex_u64(arch::aarch64::current_vbar());
            target::console::write_static(" el=");
            target::console::write_dec_usize(arch::aarch64::current_el() as usize);
            target::console::write_static("\n");
            target::rpi5::wait_uart10_empty_early_phase();

            arch::aarch64::enable_alignment_faults_current_el();

            let loaded: u64;
            core::arch::asm!(
                "ldr {loaded}, [{addr}]",
                loaded = lateout(reg) loaded,
                addr = in(reg) unaligned_addr,
                options(nostack, readonly, preserves_flags)
            );
            core::hint::black_box(loaded);
            target::console::write_static("TALOS: alignment data abort did not fire\n");
            target::rpi5::wait_uart10_empty_early_phase();
        }

        #[cfg(talos_rpi5_current_sp0_sync_diagnostic)]
        unsafe {
            let mut sp0_stack = [0u64; 128];
            let sp0_top = sp0_stack.as_mut_ptr().add(sp0_stack.len()) as usize;
            core::hint::black_box(&mut sp0_stack);

            target::console::write_static("TALOS: before SP0 BRK sp0=");
            target::console::write_hex_u64(sp0_top as u64);
            target::console::write_static(" vbar=");
            target::console::write_hex_u64(arch::aarch64::current_vbar());
            target::console::write_static(" el=");
            target::console::write_dec_usize(arch::aarch64::current_el() as usize);
            target::console::write_static("\n");
            target::rpi5::wait_uart10_empty_early_phase();

            core::arch::asm!(
                "msr SP_EL0, {sp0}",
                "msr SPSel, #0",
                "isb",
                "brk #0",
                "b .",
                sp0 = in(reg) sp0_top,
                options(noreturn)
            );
        }

        #[cfg(talos_rpi5_exception_report_diagnostic)]
        unsafe {
            target::console::write_static("TALOS: before BRK vbar=");
            target::console::write_hex_u64(arch::aarch64::current_vbar());
            target::console::write_static(" el=");
            target::console::write_dec_usize(arch::aarch64::current_el() as usize);
            target::console::write_static("\n");
            core::arch::asm!("brk #0", options(nomem, nostack, preserves_flags));
        }

        #[cfg(talos_rpi5_normal_exception_report_diagnostic)]
        unsafe {
            println!(
                "TALOS: before normal BRK vbar={:#x} el={}",
                arch::aarch64::current_vbar(),
                arch::aarch64::current_el() as usize
            );
            #[cfg(talos_rpi5_exception_return_diagnostic)]
            {
                let mut after_x9 = 0;
                let mut after_x19 = 0;
                let preserved =
                    rpi5_brk_register_preserve_probe(&mut after_x9, &mut after_x19) != 0;

                println!(
                    "TALOS: after normal BRK resume x9={:#018x} x19={:#018x}",
                    after_x9, after_x19
                );

                if preserved {
                    println!("TALOS: exception registers preserved");
                } else {
                    println!("TALOS: exception register preserve failed");
                    target::rpi5::wait_uart10_empty_early_phase();
                    arch::aarch64::halt()
                }
            }
            #[cfg(not(talos_rpi5_exception_return_diagnostic))]
            core::arch::asm!("brk #0", options(nomem, nostack, preserves_flags));
        }

        #[cfg(talos_rpi5_exception_return_diagnostic)]
        {
            println!("TALOS: after normal BRK resume");
            target::rpi5::wait_uart10_empty_early_phase();
            arch::aarch64::halt()
        }

        #[cfg(talos_rpi5_nested_panic_diagnostic)]
        {
            target::console::write_static("TALOS: nested panic diagnostic prearm\n");
            target::rpi5::wait_uart10_empty_early_phase();
            PANIC_IN_PROGRESS.prearm();
            target::console::write_static("TALOS: nested panic diagnostic trigger\n");
            target::rpi5::wait_uart10_empty_early_phase();
        }

        #[cfg(any(
            talos_rpi5_panic_report_diagnostic,
            talos_rpi5_full_panic_info_diagnostic
        ))]
        panic!("talos diagnostic panic");

        #[cfg(talos_rpi5_dynamic_format_fallback_diagnostic)]
        {
            println!("dynamic formatting fallback probe {}", 1);
            rpi5_dynamic_format_fallback_reset_probe();
        }

        #[cfg(talos_rpi5_fmt_sink_diagnostic)]
        rpi5_fmt_sink_diagnostic();

        #[cfg(talos_rpi5_fmt_static_sink_diagnostic)]
        rpi5_fmt_static_sink_diagnostic();

        #[cfg(talos_rpi5_fmt_sink_direct_diagnostic)]
        rpi5_fmt_sink_direct_diagnostic();

        #[cfg(talos_rpi5_fmt_sink_dyn_direct_diagnostic)]
        rpi5_fmt_sink_dyn_direct_diagnostic();

        #[cfg(talos_rpi5_fmt_sink_fnptr_direct_diagnostic)]
        rpi5_fmt_sink_fnptr_direct_diagnostic();

        #[cfg(talos_rpi5_fnptr_reset_diagnostic)]
        rpi5_fnptr_reset_diagnostic();

        #[cfg(talos_rpi5_minimal_format_diagnostic)]
        rpi5_minimal_format_reset_probe();

        arch::aarch64::halt()
    }

    #[cfg(not(talos_target_rpi5_bcm2712))]
    {
        println!();
        println!(
            "Talos {} booting on {}",
            env!("CARGO_PKG_VERSION"),
            boot_info.target.name()
        );

        println!(
            "boot-info: dtb_pa={:#018x} core={} el={} target={}",
            boot_info.dtb_pa,
            boot_info.primary_core,
            boot_info.exception_level,
            boot_info.target.name()
        );
        let services = target::services(boot_info);
        println!(
            "target-services: uart={} timer={} irq={} dtb={:#018x?}",
            services.uart.name(),
            services.timer.name(),
            services.interrupt_controller.name(),
            services.device_tree.physical_address()
        );
        println!("mmio-regions: {}", services.mmio_map.regions().len());
        println!("talos: hello from {}", boot_info.target.name());
        println!("talos: qemu smoke PASS");
        match boot_info.target {
            target::TargetKind::QemuVirt => target::qemu::exit_success(),
            target::TargetKind::Rpi5Bcm2712 => arch::aarch64::halt(),
        }
    }
}

#[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_println_phase_diagnostic))]
fn rpi5_println_phase_diagnostic() -> ! {
    let mut count = 0usize;

    loop {
        println!("TALOS: println phase");
        println!("TALOS: println count {}", count);
        count = count.wrapping_add(1);

        for _ in 0..0x8000 {
            core::hint::spin_loop();
        }
    }
}

#[cfg(any(
    all(talos_target_rpi5_bcm2712, talos_rpi5_fmt_sink_diagnostic),
    all(talos_target_rpi5_bcm2712, talos_rpi5_fmt_static_sink_diagnostic),
    all(talos_target_rpi5_bcm2712, talos_rpi5_fmt_sink_direct_diagnostic),
    all(talos_target_rpi5_bcm2712, talos_rpi5_fmt_sink_dyn_direct_diagnostic),
    all(talos_target_rpi5_bcm2712, talos_rpi5_fmt_sink_fnptr_direct_diagnostic),
))]
struct Rpi5FmtSink;

#[cfg(any(
    all(talos_target_rpi5_bcm2712, talos_rpi5_fmt_sink_diagnostic),
    all(talos_target_rpi5_bcm2712, talos_rpi5_fmt_static_sink_diagnostic),
    all(talos_target_rpi5_bcm2712, talos_rpi5_fmt_sink_direct_diagnostic),
    all(talos_target_rpi5_bcm2712, talos_rpi5_fmt_sink_dyn_direct_diagnostic),
    all(talos_target_rpi5_bcm2712, talos_rpi5_fmt_sink_fnptr_direct_diagnostic),
))]
impl core::fmt::Write for Rpi5FmtSink {
    #[inline(never)]
    fn write_str(&mut self, _s: &str) -> core::fmt::Result {
        Ok(())
    }
}

#[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_fmt_sink_diagnostic))]
fn rpi5_fmt_sink_diagnostic() -> ! {
    let mut sink = Rpi5FmtSink;
    core::fmt::write(&mut sink, format_args!("fmt sink {}", 1)).expect("sink formatting failed");
    rpi5_fmt_sink_reset_probe()
}

#[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_fmt_static_sink_diagnostic))]
fn rpi5_fmt_static_sink_diagnostic() -> ! {
    let mut sink = Rpi5FmtSink;
    core::fmt::write(&mut sink, format_args!("fmt static")).expect("sink formatting failed");
    rpi5_fmt_sink_reset_probe()
}

#[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_fmt_sink_direct_diagnostic))]
#[inline(never)]
fn rpi5_fmt_sink_direct_diagnostic() -> ! {
    let mut sink = Rpi5FmtSink;
    match core::fmt::Write::write_str(&mut sink, "fmt sink direct") {
        Ok(()) => rpi5_fmt_sink_reset_probe(),
        Err(_) => arch::aarch64::halt(),
    }
}

#[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_fmt_sink_dyn_direct_diagnostic))]
#[inline(never)]
fn rpi5_fmt_sink_dyn_direct_diagnostic() -> ! {
    let mut sink = Rpi5FmtSink;
    let writer: &mut dyn core::fmt::Write = &mut sink;
    match writer.write_str("fmt sink dyn direct") {
        Ok(()) => rpi5_fmt_sink_reset_probe(),
        Err(_) => arch::aarch64::halt(),
    }
}

#[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_fmt_sink_fnptr_direct_diagnostic))]
#[inline(never)]
fn rpi5_fmt_sink_fnptr_write_str(sink: &mut Rpi5FmtSink, text: &str) -> core::fmt::Result {
    core::fmt::Write::write_str(sink, text)
}

#[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_fmt_sink_fnptr_direct_diagnostic))]
#[inline(never)]
fn rpi5_fmt_sink_fnptr_direct_diagnostic() -> ! {
    let mut sink = Rpi5FmtSink;
    let write: fn(&mut Rpi5FmtSink, &str) -> core::fmt::Result =
        core::hint::black_box(rpi5_fmt_sink_fnptr_write_str);
    match write(&mut sink, "fmt sink fnptr direct") {
        Ok(()) => rpi5_fmt_sink_reset_probe(),
        Err(_) => arch::aarch64::halt(),
    }
}

#[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_fnptr_reset_diagnostic))]
#[inline(never)]
fn rpi5_fnptr_reset_diagnostic() -> ! {
    let reset: fn() -> ! = core::hint::black_box(rpi5_fmt_sink_reset_probe);
    reset()
}

#[cfg(any(
    all(talos_target_rpi5_bcm2712, talos_rpi5_fmt_sink_diagnostic),
    all(talos_target_rpi5_bcm2712, talos_rpi5_fmt_static_sink_diagnostic),
    all(talos_target_rpi5_bcm2712, talos_rpi5_fmt_sink_direct_diagnostic),
    all(talos_target_rpi5_bcm2712, talos_rpi5_fmt_sink_dyn_direct_diagnostic),
    all(talos_target_rpi5_bcm2712, talos_rpi5_fmt_sink_fnptr_direct_diagnostic),
    all(talos_target_rpi5_bcm2712, talos_rpi5_fnptr_reset_diagnostic),
    all(talos_target_rpi5_bcm2712, talos_rpi5_asm_to_rust_reset_diagnostic),
    all(
        talos_target_rpi5_bcm2712,
        talos_rpi5_asm_indirect_to_rust_reset_diagnostic
    ),
    all(
        talos_target_rpi5_bcm2712,
        talos_rpi5_asm_bti_indirect_to_rust_reset_diagnostic
    ),
))]
fn rpi5_fmt_sink_reset_probe() -> ! {
    unsafe {
        core::arch::asm!(
            "ldr x0, =0x84000009",
            "smc #0",
            "wfe",
            "b .-4",
            options(noreturn)
        );
    }
}

#[cfg(any(
    all(talos_target_rpi5_bcm2712, talos_rpi5_asm_to_rust_reset_diagnostic),
    all(
        talos_target_rpi5_bcm2712,
        talos_rpi5_asm_indirect_to_rust_reset_diagnostic
    ),
    all(
        talos_target_rpi5_bcm2712,
        talos_rpi5_asm_bti_indirect_to_rust_reset_diagnostic
    ),
))]
#[unsafe(no_mangle)]
pub extern "C" fn rpi5_asm_to_rust_reset_probe() -> ! {
    rpi5_fmt_sink_reset_probe()
}

#[cfg(all(
    talos_target_rpi5_bcm2712,
    talos_rpi5_dynamic_format_fallback_diagnostic
))]
fn rpi5_dynamic_format_fallback_reset_probe() -> ! {
    unsafe {
        core::arch::asm!(
            "ldr x0, =0x84000009",
            "smc #0",
            "wfe",
            "b .-4",
            options(noreturn)
        );
    }
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
fn write_rpi5_chosen_bootargs_line(bootargs: &str) {
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
fn write_rpi5_dtb_reserved_summary_line(count: usize, shown: usize, truncated: bool) {
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
fn write_rpi5_dtb_reserved_entry_line(index: usize, address: u64, size: u64) {
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
fn write_rpi5_dtb_memory_summary_line(
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
fn write_rpi5_dtb_memory_entry_line(index: usize, address: u64, size: u64) {
    println!(
        "talos: dtb memory[{}]: addr={:#x} size={:#x}",
        index, address, size
    );
    target::rpi5::wait_uart10_empty_early_phase();
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
fn write_rpi5_reserved_memory_summary_line(
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
fn write_rpi5_reserved_memory_entry_line(
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

#[cfg(all(
    not(test),
    talos_target_rpi5_bcm2712,
    talos_rpi5_translation_fault_diagnostic
))]
#[inline(never)]
unsafe fn rpi5_translation_fault_diagnostic() -> ! {
    const FAULT_VA: usize = 0x8000_0000;

    target::console::write_static("TALOS: before translation fault va=");
    target::console::write_hex_u64(FAULT_VA as u64);
    target::console::write_static(" vbar=");
    target::console::write_hex_u64(arch::aarch64::current_vbar());
    target::console::write_static(" el=");
    target::console::write_dec_usize(arch::aarch64::current_el() as usize);
    target::console::write_static("\n");
    target::rpi5::wait_uart10_empty_early_phase();

    let loaded: u64;
    unsafe {
        core::arch::asm!(
            "ldr {loaded}, [{addr}]",
            loaded = lateout(reg) loaded,
            addr = in(reg) FAULT_VA,
            options(nostack, readonly, preserves_flags)
        );
    }
    core::hint::black_box(loaded);
    target::console::write_static("TALOS: translation fault did not fire\n");
    target::rpi5::wait_uart10_empty_early_phase();
    arch::aarch64::halt()
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
fn rpi5_kernel_layout() -> memory_map::KernelLayout {
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
fn write_rpi5_memory_layout_kernel_line(layout: memory_map::KernelLayout) {
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
fn write_rpi5_memory_layout_dtb_line(dtb: memory_map::FdtBlobRange) {
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
fn write_rpi5_memory_usable_candidate_line(candidate: memory_map::EarlyUsableMemory) {
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
fn write_rpi5_memory_usable_candidate_println_line(candidate: memory_map::EarlyUsableMemory) {
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
fn write_rpi5_page_frame_seed_line(seed: memory_map::EarlyPageFrameSeed) {
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
#[inline(never)]
fn write_rpi5_page_frame_seed_println_line(seed: memory_map::EarlyPageFrameSeed) {
    print!("talos: page frames seed: start={:#x}", seed.start);
    print!(" end={:#x}", seed.end);
    print!(" pages={:#x}", seed.page_count);
    print!(" page_size={:#x}", seed.page_size);
    println!(" source=memory-usable phase=post-allocator");
    target::rpi5::wait_uart10_empty_early_phase();
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
fn write_rpi5_bootstrap_page_reservation_early_line(
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
fn write_rpi5_bootstrap_page_reservation_line(
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
fn write_rpi5_translation_table_layout_line(layout: memory_map::EarlyTranslationTableLayout) {
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
fn write_rpi5_translation_table_slots_line(layout: memory_map::EarlyTranslationTableLayout) {
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
fn write_rpi5_translation_table_population_line(
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
fn write_rpi5_translation_table_policy_line(
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
fn write_rpi5_translation_register_plan_line(plan: memory_map::EarlyTranslationRegisterPlan) {
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
fn write_rpi5_translation_enabled_line(plan: memory_map::EarlyTranslationRegisterPlan, sctlr: u64) {
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
fn write_rpi5_instruction_cache_plan_line(plan: memory_map::EarlyInstructionCacheEnablePlan) {
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
fn write_rpi5_instruction_cache_enabled_line(
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
fn write_rpi5_data_cache_plan_line(plan: memory_map::EarlyDataCacheEnablePlan) {
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
fn write_rpi5_data_cache_enabled_line(plan: memory_map::EarlyDataCacheEnablePlan, sctlr: u64) {
    println!(
        "talos: data cache enabled: el={:#x} sctlr={:#x} kind={}",
        plan.current_el as u64,
        sctlr,
        memory_map::EARLY_DATA_CACHE_ENABLE_KIND
    );
    target::rpi5::wait_uart10_empty_early_phase();
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
fn write_rpi5_bootstrap_allocator_plan_line(plan: memory_map::EarlyBootstrapAllocatorPlan) {
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
fn write_rpi5_bootstrap_allocator_init_line(state: allocator::BumpAllocatorState) {
    println!(
        "talos: bootstrap allocator init: start={:#x} next={:#x} end={:#x} policy=no-free",
        state.start, state.next, state.end
    );
    target::rpi5::wait_uart10_empty_early_phase();
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
#[cfg_attr(not(talos_rpi5_alloc_oom_diagnostic), allow(dead_code))]
fn rpi5_alloc_oom_diagnostic() -> ! {
    if let Some(state) = KERNEL_GLOBAL_ALLOCATOR.state() {
        let requested_capacity = state.remaining_bytes + 8;
        target::console::write_static("talos: alloc oom diagnostic: request=");
        target::console::write_hex_u64(requested_capacity as u64);
        target::console::write_static(" remaining=");
        target::console::write_hex_u64(state.remaining_bytes as u64);
        target::console::write_static(" align=0x1\n");
        target::rpi5::wait_uart10_empty_early_phase();

        let _oom = alloc::vec::Vec::<u8>::with_capacity(requested_capacity);
    } else {
        target::console::write_static("talos: alloc oom diagnostic: allocator unavailable\n");
        target::rpi5::wait_uart10_empty_early_phase();
    }

    loop {
        core::hint::spin_loop();
    }
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
#[cfg_attr(not(talos_rpi5_realloc_growth_diagnostic), allow(dead_code))]
fn rpi5_realloc_growth_diagnostic() -> ! {
    let old_layout = unsafe { core::alloc::Layout::from_size_align_unchecked(2, 1) };
    let old_ptr = unsafe { core::alloc::GlobalAlloc::alloc(&KERNEL_GLOBAL_ALLOCATOR, old_layout) };
    if !old_ptr.is_null() {
        unsafe {
            old_ptr.add(0).write(1);
            old_ptr.add(1).write(2);
        }
    }
    let new_ptr = unsafe {
        core::alloc::GlobalAlloc::realloc(&KERNEL_GLOBAL_ALLOCATOR, old_ptr, old_layout, 4)
    };
    if !new_ptr.is_null() {
        unsafe {
            new_ptr.add(2).write(3);
            new_ptr.add(3).write(0x41);
        }
    }

    let mut sum = 0u64;
    let mut index = 0usize;
    while !new_ptr.is_null() && index < 4 {
        sum += unsafe { new_ptr.add(index).read_volatile() } as u64;
        index += 1;
    }

    let state = KERNEL_GLOBAL_ALLOCATOR.state();
    let mut exhaustion_ok = false;
    if let Some(before_exhaustion) = state {
        let oversized_layout = unsafe {
            core::alloc::Layout::from_size_align_unchecked(before_exhaustion.remaining_bytes + 8, 8)
        };
        let exhausted =
            unsafe { core::alloc::GlobalAlloc::alloc(&KERNEL_GLOBAL_ALLOCATOR, oversized_layout) };
        if let Some(after_exhaustion) = KERNEL_GLOBAL_ALLOCATOR.state() {
            exhaustion_ok = exhausted.is_null() && after_exhaustion.next == before_exhaustion.next;
        }
    }

    target::console::write_static("talos: realloc grow smoke: old=");
    target::console::write_hex_u64(old_ptr as u64);
    target::console::write_static(" new=");
    target::console::write_hex_u64(new_ptr as u64);
    target::console::write_static(" size=4");
    target::console::write_static(" sum=");
    target::console::write_hex_u64(sum);
    target::console::write_static(" next=");
    if let Some(state) = state {
        target::console::write_hex_u64(state.next as u64);
        target::console::write_static(" used=");
        target::console::write_hex_u64(state.used_bytes as u64);
        target::console::write_static(" rem=");
        target::console::write_hex_u64(state.remaining_bytes as u64);
    }
    target::console::write_static(" ex=");
    if exhaustion_ok {
        target::console::write_static("true");
    } else {
        target::console::write_static("false");
    }
    target::console::write_static(" moved=");
    if !old_ptr.is_null() && !new_ptr.is_null() && new_ptr != old_ptr {
        target::console::write_static("true");
    } else {
        target::console::write_static("false");
    }
    let ok = !old_ptr.is_null()
        && !new_ptr.is_null()
        && new_ptr != old_ptr
        && sum == 0x47
        && exhaustion_ok;
    target::console::write_static(" ok=");
    if ok {
        target::console::write_static("true\n");
    } else {
        target::console::write_static("false\n");
    }
    target::rpi5::wait_uart10_empty_early_phase();

    loop {
        core::hint::spin_loop();
    }
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712, talos_rpi5_vec_growth_diagnostic))]
fn rpi5_vec_growth_diagnostic() -> ! {
    target::console::write_static("talos: vec grow start\n");
    target::rpi5::wait_uart10_empty_early_phase();

    let mut values = alloc::vec::Vec::<u8>::with_capacity(2);
    let old_ptr = values.as_ptr() as u64;
    unsafe {
        let ptr = values.as_mut_ptr();
        ptr.add(0).write(1);
        ptr.add(1).write(2);
        values.set_len(2);
    }
    let before_growth_ptr = values.as_ptr() as u64;
    values.reserve_exact(2);
    let new_ptr = values.as_ptr() as u64;
    unsafe {
        let ptr = values.as_mut_ptr();
        ptr.add(2).write(3);
        ptr.add(3).write(0x41);
        values.set_len(4);
    }

    let mut sum = 0u64;
    let mut index = 0usize;
    while index < values.len() {
        sum += unsafe { values.as_ptr().add(index).read_volatile() } as u64;
        index += 1;
    }

    let state = KERNEL_GLOBAL_ALLOCATOR.state();
    let mut exhaustion_ok = false;
    if let Some(before_exhaustion) = state {
        let oversized_layout = unsafe {
            core::alloc::Layout::from_size_align_unchecked(before_exhaustion.remaining_bytes + 8, 8)
        };
        let exhausted =
            unsafe { core::alloc::GlobalAlloc::alloc(&KERNEL_GLOBAL_ALLOCATOR, oversized_layout) };
        if let Some(after_exhaustion) = KERNEL_GLOBAL_ALLOCATOR.state() {
            exhaustion_ok = exhausted.is_null() && after_exhaustion.next == before_exhaustion.next;
        }
    }

    target::console::write_static("talos: vec grow smoke: old=");
    target::console::write_hex_u64(old_ptr);
    target::console::write_static(" new=");
    target::console::write_hex_u64(new_ptr);
    target::console::write_static(" len=");
    target::console::write_dec_usize(values.len());
    target::console::write_static(" cap=");
    target::console::write_dec_usize(values.capacity());
    target::console::write_static(" sum=");
    target::console::write_hex_u64(sum);
    target::console::write_static(" next=");
    if let Some(state) = state {
        target::console::write_hex_u64(state.next as u64);
        target::console::write_static(" used=");
        target::console::write_hex_u64(state.used_bytes as u64);
        target::console::write_static(" rem=");
        target::console::write_hex_u64(state.remaining_bytes as u64);
    }
    target::console::write_static(" ex=");
    write_rpi5_bool(exhaustion_ok);
    target::console::write_static(" moved=");
    let moved = old_ptr != 0 && before_growth_ptr == old_ptr && new_ptr != old_ptr;
    write_rpi5_bool(moved);
    let ok = values.len() == 4 && values.capacity() >= 4 && sum == 0x47 && moved && exhaustion_ok;
    target::console::write_static(" ok=");
    if ok {
        target::console::write_static("true\n");
    } else {
        target::console::write_static("false\n");
    }
    target::rpi5::wait_uart10_empty_early_phase();

    core::mem::forget(values);
    loop {
        core::hint::spin_loop();
    }
}

#[cfg(all(
    not(test),
    talos_target_rpi5_bcm2712,
    talos_rpi5_string_growth_diagnostic
))]
fn rpi5_string_growth_diagnostic() -> ! {
    target::console::write_static("talos: string grow start\n");
    target::rpi5::wait_uart10_empty_early_phase();

    let mut text = alloc::string::String::with_capacity(2);
    let old_ptr = text.as_ptr() as u64;
    unsafe {
        let bytes = text.as_mut_vec();
        let ptr = bytes.as_mut_ptr();
        ptr.add(0).write(b'T');
        ptr.add(1).write(b'a');
        bytes.set_len(2);
    }
    let before_growth_ptr = text.as_ptr() as u64;
    unsafe {
        text.as_mut_vec().reserve_exact(2);
    }
    let new_ptr = text.as_ptr() as u64;
    unsafe {
        let bytes = text.as_mut_vec();
        let ptr = bytes.as_mut_ptr();
        ptr.add(2).write(b'l');
        ptr.add(3).write(b'o');
        bytes.set_len(4);
    }

    let mut sum = 0u64;
    let mut index = 0usize;
    while index < text.len() {
        sum += unsafe { text.as_ptr().add(index).read_volatile() } as u64;
        index += 1;
    }

    let state = KERNEL_GLOBAL_ALLOCATOR.state();
    let mut exhaustion_ok = false;
    if let Some(before_exhaustion) = state {
        let oversized_layout = unsafe {
            core::alloc::Layout::from_size_align_unchecked(before_exhaustion.remaining_bytes + 8, 8)
        };
        let exhausted =
            unsafe { core::alloc::GlobalAlloc::alloc(&KERNEL_GLOBAL_ALLOCATOR, oversized_layout) };
        if let Some(after_exhaustion) = KERNEL_GLOBAL_ALLOCATOR.state() {
            exhaustion_ok = exhausted.is_null() && after_exhaustion.next == before_exhaustion.next;
        }
    }

    target::console::write_static("talos: string grow smoke: old=");
    target::console::write_hex_u64(old_ptr);
    target::console::write_static(" new=");
    target::console::write_hex_u64(new_ptr);
    target::console::write_static(" len=");
    target::console::write_dec_usize(text.len());
    target::console::write_static(" cap=");
    target::console::write_dec_usize(text.capacity());
    target::console::write_static(" sum=");
    target::console::write_hex_u64(sum);
    target::console::write_static(" next=");
    if let Some(state) = state {
        target::console::write_hex_u64(state.next as u64);
        target::console::write_static(" used=");
        target::console::write_hex_u64(state.used_bytes as u64);
        target::console::write_static(" rem=");
        target::console::write_hex_u64(state.remaining_bytes as u64);
    }
    target::console::write_static(" ex=");
    write_rpi5_bool(exhaustion_ok);
    target::console::write_static(" moved=");
    let moved = old_ptr != 0 && before_growth_ptr == old_ptr && new_ptr != old_ptr;
    write_rpi5_bool(moved);
    let ok = text.len() == 4 && text.capacity() >= 4 && sum == 0x190 && moved && exhaustion_ok;
    target::console::write_static(" ok=");
    if ok {
        target::console::write_static("true\n");
    } else {
        target::console::write_static("false\n");
    }
    target::rpi5::wait_uart10_empty_early_phase();

    core::mem::forget(text);
    loop {
        core::hint::spin_loop();
    }
}

#[cfg(all(
    not(test),
    talos_target_rpi5_bcm2712,
    talos_rpi5_alloc_format_diagnostic
))]
fn rpi5_alloc_format_diagnostic() -> ! {
    target::console::write_static("talos: alloc format start\n");
    target::rpi5::wait_uart10_empty_early_phase();

    let text = alloc::format!("{} {}", "Talos", 5usize);
    let ptr = text.as_ptr() as u64;

    let mut sum = 0u64;
    let mut index = 0usize;
    while index < text.len() {
        sum += unsafe { text.as_ptr().add(index).read_volatile() } as u64;
        index += 1;
    }

    let expected = b"Talos 5";
    let matches_expected = text.as_bytes() == expected;
    let state = KERNEL_GLOBAL_ALLOCATOR.state();
    let mut exhaustion_ok = false;
    if let Some(before_exhaustion) = state {
        let oversized_layout = unsafe {
            core::alloc::Layout::from_size_align_unchecked(before_exhaustion.remaining_bytes + 8, 8)
        };
        let exhausted =
            unsafe { core::alloc::GlobalAlloc::alloc(&KERNEL_GLOBAL_ALLOCATOR, oversized_layout) };
        if let Some(after_exhaustion) = KERNEL_GLOBAL_ALLOCATOR.state() {
            exhaustion_ok = exhausted.is_null() && after_exhaustion.next == before_exhaustion.next;
        }
    }

    target::console::write_static("talos: alloc format smoke: ptr=");
    target::console::write_hex_u64(ptr);
    target::console::write_static(" len=");
    target::console::write_dec_usize(text.len());
    target::console::write_static(" cap=");
    target::console::write_dec_usize(text.capacity());
    target::console::write_static(" sum=");
    target::console::write_hex_u64(sum);
    target::console::write_static(" next=");
    if let Some(state) = state {
        target::console::write_hex_u64(state.next as u64);
        target::console::write_static(" used=");
        target::console::write_hex_u64(state.used_bytes as u64);
        target::console::write_static(" rem=");
        target::console::write_hex_u64(state.remaining_bytes as u64);
    }
    target::console::write_static(" ex=");
    write_rpi5_bool(exhaustion_ok);
    target::console::write_static(" ascii=");
    write_rpi5_bool(matches_expected);
    let ok = ptr != 0
        && text.len() == expected.len()
        && sum == 0x258
        && matches_expected
        && exhaustion_ok;
    target::console::write_static(" ok=");
    if ok {
        target::console::write_static("true\n");
    } else {
        target::console::write_static("false\n");
    }
    target::rpi5::wait_uart10_empty_early_phase();

    core::mem::forget(text);
    loop {
        core::hint::spin_loop();
    }
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
#[cfg_attr(
    any(
        talos_rpi5_alloc_oom_diagnostic,
        talos_rpi5_realloc_growth_diagnostic,
        talos_rpi5_vec_growth_diagnostic,
        talos_rpi5_string_growth_diagnostic,
        talos_rpi5_alloc_format_diagnostic
    ),
    allow(dead_code)
)]
fn rpi5_bootstrap_alloc_smoke() {
    let mut text = alloc::string::String::with_capacity(8);
    let allocated_ptr;
    unsafe {
        let bytes = text.as_mut_vec();
        let ptr = bytes.as_mut_ptr();
        allocated_ptr = ptr as u64;
        ptr.add(0).write(b'T');
        ptr.add(1).write(b'a');
        ptr.add(2).write(b'l');
        ptr.add(3).write(b'o');
        ptr.add(4).write(b's');
        bytes.set_len(5);
    }
    let after_fill_ptr = text.as_ptr() as u64;

    let mut sum = 0u64;
    let mut index = 0usize;
    while index < text.len() {
        sum += unsafe { text.as_ptr().add(index).read_volatile() } as u64;
        index += 1;
    }
    let capacity = text.capacity();

    let state = KERNEL_GLOBAL_ALLOCATOR.state();
    let mut exhaustion_ok = false;
    if let Some(before_exhaustion) = state {
        let oversized_layout = unsafe {
            core::alloc::Layout::from_size_align_unchecked(before_exhaustion.remaining_bytes + 8, 8)
        };
        let exhausted =
            unsafe { core::alloc::GlobalAlloc::alloc(&KERNEL_GLOBAL_ALLOCATOR, oversized_layout) };
        if let Some(after_exhaustion) = KERNEL_GLOBAL_ALLOCATOR.state() {
            exhaustion_ok = exhausted.is_null() && after_exhaustion.next == before_exhaustion.next;
        }
    }

    let stable = allocated_ptr == after_fill_ptr;
    let ok = text.len() == 5 && capacity == 8 && sum == 0x203 && stable && exhaustion_ok;

    if let Some(state) = state {
        println!(
            "talos: string smoke: ptr={:#x} len={} cap={} sum={:#x} next={:#x} used={:#x} rem={:#x} ex={} stable={} ok={}",
            allocated_ptr,
            text.len(),
            capacity,
            sum,
            state.next,
            state.used_bytes,
            state.remaining_bytes,
            exhaustion_ok,
            stable,
            ok
        );
    } else {
        println!(
            "talos: string smoke: ptr={:#x} len={} cap={} sum={:#x} next=unavailable ex={} stable={} ok={}",
            allocated_ptr,
            text.len(),
            capacity,
            sum,
            exhaustion_ok,
            stable,
            ok
        );
    }
    target::rpi5::wait_uart10_empty_early_phase();
    core::mem::forget(text);
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
fn write_rpi5_page_frame_remaining_line(seed: memory_map::EarlyPageFrameSeed) {
    println!(
        "talos: page frames remaining: start={:#x} end={:#x} pages={:#x} page_size={:#x} source=bootstrap-reserve",
        seed.start, seed.end, seed.page_count, seed.page_size
    );
    target::rpi5::wait_uart10_empty_early_phase();
}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
fn write_rpi5_bool(value: bool) {
    if value {
        target::console::write_static("true");
    } else {
        target::console::write_static("false");
    }
}

#[panic_handler]
#[cfg_attr(
    all(talos_target_rpi5_bcm2712, talos_rpi5_nested_panic_diagnostic),
    allow(unreachable_code, unused_variables)
)]
fn panic(info: &PanicInfo<'_>) -> ! {
    #[cfg(talos_target_rpi5_bcm2712)]
    {
        target::console::write_static("\nTALOS: panic handler entered\n");

        if PANIC_IN_PROGRESS.enter() {
            target::console::write_static("\nTALOS: nested panic\n");
            target::rpi5::wait_uart10_empty_early_phase();
            arch::aarch64::halt()
        }

        println!("talos panic: {}", info);
        target::rpi5::wait_uart10_empty_early_phase();
        arch::aarch64::halt()
    }

    #[cfg(not(talos_target_rpi5_bcm2712))]
    {
        println!();
        println!("talos panic: {}", info);

        #[cfg(test)]
        target::qemu::exit_failure();

        #[cfg(not(test))]
        arch::aarch64::halt()
    }
}

pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        target::console::write_static(core::any::type_name::<T>());
        target::console::write_static(" ... ");
        self();
        println!("ok");
    }
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Testable]) {
    println!();
    target::console::write_static("running ");
    target::console::write_dec_usize(tests.len());
    target::console::write_static(" talos no_std tests\n");
    for test in tests {
        test.run();
    }
    target::console::write_static("test result: ok. ");
    target::console::write_dec_usize(tests.len());
    target::console::write_static(" passed\n");
}

#[cfg(test)]
#[test_case]
fn smoke_test_runs() {
    assert_eq!(2 + 2, 4);
}

#[cfg(test)]
#[test_case]
fn target_services_include_qemu_console() {
    let boot_info = BootInfo::from_aarch64_x0(0x4000_0000);
    let services = target::services(&boot_info);

    assert_eq!(services.uart, target::UartKind::Pl011);
    assert_eq!(services.uart.name(), "pl011");
    assert_eq!(services.timer.name(), "arm-generic");
    assert_eq!(services.interrupt_controller.name(), "gic-v2");
    assert_eq!(services.mmio_map.regions().len(), 1);
    assert!(services.device_tree.physical_address().is_some());
    assert_eq!(services.device_tree.physical_address(), Some(0x4000_0000));
    assert_eq!(boot_info.target.name(), "talos-aarch64-virt");
}

#[cfg(test)]
#[test_case]
fn device_tree_reads_fdt_header() {
    #[repr(align(4))]
    struct Aligned<const N: usize>([u8; N]);

    static TEST_FDT: Aligned<40> = Aligned([
        0xd0, 0x0d, 0xfe, 0xed, // magic
        0x00, 0x00, 0x01, 0x80, // totalsize
        0x00, 0x00, 0x00, 0x38, // off_dt_struct
        0x00, 0x00, 0x01, 0x20, // off_dt_strings
        0x00, 0x00, 0x00, 0x28, // off_mem_rsvmap
        0x00, 0x00, 0x00, 0x11, // version
        0x00, 0x00, 0x00, 0x10, // last_comp_version
        0x00, 0x00, 0x00, 0x00, // boot_cpuid_phys
        0x00, 0x00, 0x00, 0x40, // size_dt_strings
        0x00, 0x00, 0x00, 0xe8, // size_dt_struct
    ]);

    let device_tree = device_tree::DeviceTree::from_physical_address(TEST_FDT.0.as_ptr() as usize);
    let header = unsafe { device_tree.fdt_header() }.expect("valid FDT header");

    assert_eq!(header.magic, device_tree::FdtHeader::MAGIC);
    assert_eq!(header.total_size, 0x180);
    assert_eq!(header.off_dt_struct, 0x38);
    assert_eq!(header.off_dt_strings, 0x120);
    assert_eq!(header.off_mem_rsvmap, 0x28);
    assert_eq!(header.version, 17);
    assert_eq!(header.last_comp_version, 16);
    assert_eq!(header.size_dt_strings, 0x40);
    assert_eq!(header.size_dt_struct, 0xe8);
}

#[cfg(test)]
#[test_case]
fn device_tree_reads_memory_reservations() {
    #[repr(align(4))]
    struct Aligned<const N: usize>([u8; N]);

    static TEST_FDT: Aligned<88> = Aligned([
        0xd0, 0x0d, 0xfe, 0xed, // magic
        0x00, 0x00, 0x00, 0x58, // totalsize
        0x00, 0x00, 0x00, 0x58, // off_dt_struct
        0x00, 0x00, 0x00, 0x58, // off_dt_strings
        0x00, 0x00, 0x00, 0x28, // off_mem_rsvmap
        0x00, 0x00, 0x00, 0x11, // version
        0x00, 0x00, 0x00, 0x10, // last_comp_version
        0x00, 0x00, 0x00, 0x00, // boot_cpuid_phys
        0x00, 0x00, 0x00, 0x00, // size_dt_strings
        0x00, 0x00, 0x00, 0x00, // size_dt_struct
        0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, // reserve 0 address
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0x00, // reserve 0 size
        0x00, 0x00, 0x00, 0x00, 0x3f, 0x00, 0x00, 0x00, // reserve 1 address
        0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, // reserve 1 size
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // terminator address
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // terminator size
    ]);

    let device_tree = device_tree::DeviceTree::from_physical_address(TEST_FDT.0.as_ptr() as usize);
    let reservations = unsafe { device_tree.memory_reservations() }.expect("memory reservations");

    assert_eq!(reservations.count, 2);
    assert_eq!(reservations.reported_len(), 2);
    assert!(!reservations.truncated);
    assert_eq!(
        reservations.entries[0],
        Some(device_tree::FdtMemoryReservation {
            address: 0x10_0000,
            size: 0x2000
        })
    );
    assert_eq!(
        reservations.entries[1],
        Some(device_tree::FdtMemoryReservation {
            address: 0x3f00_0000,
            size: 0x10_0000
        })
    );
}

#[cfg(test)]
#[test_case]
fn device_tree_reads_memory_banks() {
    #[repr(align(4))]
    struct Aligned<const N: usize>([u8; N]);

    static TEST_FDT: Aligned<199> = Aligned([
        0xd0, 0x0d, 0xfe, 0xed, // magic
        0x00, 0x00, 0x00, 0xc7, // totalsize
        0x00, 0x00, 0x00, 0x38, // off_dt_struct
        0x00, 0x00, 0x00, 0xa8, // off_dt_strings
        0x00, 0x00, 0x00, 0x28, // off_mem_rsvmap
        0x00, 0x00, 0x00, 0x11, // version
        0x00, 0x00, 0x00, 0x10, // last_comp_version
        0x00, 0x00, 0x00, 0x00, // boot_cpuid_phys
        0x00, 0x00, 0x00, 0x1f, // size_dt_strings
        0x00, 0x00, 0x00, 0x70, // size_dt_struct
        0x00, 0x00, 0x00, 0x00, // mem_rsvmap address high
        0x00, 0x00, 0x00, 0x00, // mem_rsvmap address low
        0x00, 0x00, 0x00, 0x00, // mem_rsvmap size high
        0x00, 0x00, 0x00, 0x00, // mem_rsvmap size low
        0x00, 0x00, 0x00, 0x01, // FDT_BEGIN_NODE
        0x00, 0x00, 0x00, 0x00, // root node name
        0x00, 0x00, 0x00, 0x03, // FDT_PROP
        0x00, 0x00, 0x00, 0x04, // property length
        0x00, 0x00, 0x00, 0x00, // property name offset: #address-cells
        0x00, 0x00, 0x00, 0x02, // property value
        0x00, 0x00, 0x00, 0x03, // FDT_PROP
        0x00, 0x00, 0x00, 0x04, // property length
        0x00, 0x00, 0x00, 0x0f, // property name offset: #size-cells
        0x00, 0x00, 0x00, 0x02, // property value
        0x00, 0x00, 0x00, 0x01, // FDT_BEGIN_NODE
        b'm', b'e', b'm', b'o', b'r', b'y', b'@', b'0', // node name
        0x00, 0x00, 0x00, 0x00, // node name terminator + padding
        0x00, 0x00, 0x00, 0x03, // FDT_PROP
        0x00, 0x00, 0x00, 0x20, // property length
        0x00, 0x00, 0x00, 0x1b, // property name offset: reg
        0x00, 0x00, 0x00, 0x00, // bank 0 address high
        0x00, 0x00, 0x00, 0x00, // bank 0 address low
        0x00, 0x00, 0x00, 0x00, // bank 0 size high
        0x40, 0x00, 0x00, 0x00, // bank 0 size low
        0x00, 0x00, 0x00, 0x01, // bank 1 address high
        0x00, 0x00, 0x00, 0x00, // bank 1 address low
        0x00, 0x00, 0x00, 0x00, // bank 1 size high
        0x80, 0x00, 0x00, 0x00, // bank 1 size low
        0x00, 0x00, 0x00, 0x02, // FDT_END_NODE
        0x00, 0x00, 0x00, 0x02, // FDT_END_NODE
        0x00, 0x00, 0x00, 0x09, // FDT_END
        b'#', b'a', b'd', b'd', b'r', b'e', b's', b's', b'-', b'c', b'e', b'l', b'l', b's', 0x00,
        b'#', b's', b'i', b'z', b'e', b'-', b'c', b'e', b'l', b'l', b's', 0x00, b'r', b'e', b'g',
        0x00,
    ]);

    let device_tree = device_tree::DeviceTree::from_physical_address(TEST_FDT.0.as_ptr() as usize);
    let banks = unsafe { device_tree.memory_banks() }.expect("memory banks");

    assert_eq!(banks.address_cells, 2);
    assert_eq!(banks.size_cells, 2);
    assert_eq!(banks.count, 2);
    assert_eq!(banks.reported_len(), 2);
    assert!(!banks.truncated);
    assert_eq!(
        banks.entries[0],
        Some(device_tree::FdtMemoryBank {
            address: 0,
            size: 0x4000_0000
        })
    );
    assert_eq!(
        banks.entries[1],
        Some(device_tree::FdtMemoryBank {
            address: 0x1_0000_0000,
            size: 0x8000_0000
        })
    );
}

#[cfg(test)]
#[test_case]
fn device_tree_reads_reserved_memory_ranges() {
    #[repr(align(4))]
    struct Aligned<const N: usize>([u8; N]);

    static TEST_FDT: Aligned<366> = Aligned([
        0xd0, 0x0d, 0xfe, 0xed, // magic
        0x00, 0x00, 0x01, 0x6e, // totalsize
        0x00, 0x00, 0x00, 0x38, // off_dt_struct
        0x00, 0x00, 0x01, 0x38, // off_dt_strings
        0x00, 0x00, 0x00, 0x28, // off_mem_rsvmap
        0x00, 0x00, 0x00, 0x11, // version
        0x00, 0x00, 0x00, 0x10, // last_comp_version
        0x00, 0x00, 0x00, 0x00, // boot_cpuid_phys
        0x00, 0x00, 0x00, 0x36, // size_dt_strings
        0x00, 0x00, 0x01, 0x00, // size_dt_struct
        0x00, 0x00, 0x00, 0x00, // mem_rsvmap address high
        0x00, 0x00, 0x00, 0x00, // mem_rsvmap address low
        0x00, 0x00, 0x00, 0x00, // mem_rsvmap size high
        0x00, 0x00, 0x00, 0x00, // mem_rsvmap size low
        0x00, 0x00, 0x00, 0x01, // FDT_BEGIN_NODE
        0x00, 0x00, 0x00, 0x00, // root node name
        0x00, 0x00, 0x00, 0x03, // FDT_PROP
        0x00, 0x00, 0x00, 0x04, // property length
        0x00, 0x00, 0x00, 0x00, // property name offset: #address-cells
        0x00, 0x00, 0x00, 0x02, // property value
        0x00, 0x00, 0x00, 0x03, // FDT_PROP
        0x00, 0x00, 0x00, 0x04, // property length
        0x00, 0x00, 0x00, 0x0f, // property name offset: #size-cells
        0x00, 0x00, 0x00, 0x02, // property value
        0x00, 0x00, 0x00, 0x01, // FDT_BEGIN_NODE
        b'r', b'e', b's', b'e', b'r', b'v', b'e', b'd', b'-', b'm', b'e', b'm', b'o', b'r', b'y',
        0x00, 0x00, 0x00, 0x00, 0x03, // FDT_PROP
        0x00, 0x00, 0x00, 0x04, // property length
        0x00, 0x00, 0x00, 0x00, // property name offset: #address-cells
        0x00, 0x00, 0x00, 0x02, // property value
        0x00, 0x00, 0x00, 0x03, // FDT_PROP
        0x00, 0x00, 0x00, 0x04, // property length
        0x00, 0x00, 0x00, 0x0f, // property name offset: #size-cells
        0x00, 0x00, 0x00, 0x02, // property value
        0x00, 0x00, 0x00, 0x03, // FDT_PROP
        0x00, 0x00, 0x00, 0x00, // property length
        0x00, 0x00, 0x00, 0x1b, // property name offset: ranges
        0x00, 0x00, 0x00, 0x01, // FDT_BEGIN_NODE
        b'f', b'r', b'a', b'm', b'e', b'b', b'u', b'f', b'f', b'e', b'r', b'@', b'3', b'f', b'0',
        b'0', b'0', b'0', b'0', b'0', 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x03, // FDT_PROP
        0x00, 0x00, 0x00, 0x10, // property length
        0x00, 0x00, 0x00, 0x22, // property name offset: reg
        0x00, 0x00, 0x00, 0x00, // address high
        0x3f, 0x00, 0x00, 0x00, // address low
        0x00, 0x00, 0x00, 0x00, // size high
        0x01, 0x00, 0x00, 0x00, // size low
        0x00, 0x00, 0x00, 0x03, // FDT_PROP
        0x00, 0x00, 0x00, 0x00, // property length
        0x00, 0x00, 0x00, 0x26, // property name offset: no-map
        0x00, 0x00, 0x00, 0x02, // FDT_END_NODE
        0x00, 0x00, 0x00, 0x01, // FDT_BEGIN_NODE
        b'r', b'e', b'u', b's', b'a', b'b', b'l', b'e', b'@', b'1', b'0', b'0', b'0', b'0', b'0',
        b'0', b'0', b'0', 0x00, 0x00, // node name + padding
        0x00, 0x00, 0x00, 0x03, // FDT_PROP
        0x00, 0x00, 0x00, 0x10, // property length
        0x00, 0x00, 0x00, 0x22, // property name offset: reg
        0x00, 0x00, 0x00, 0x01, // address high
        0x00, 0x00, 0x00, 0x00, // address low
        0x00, 0x00, 0x00, 0x00, // size high
        0x02, 0x00, 0x00, 0x00, // size low
        0x00, 0x00, 0x00, 0x03, // FDT_PROP
        0x00, 0x00, 0x00, 0x00, // property length
        0x00, 0x00, 0x00, 0x2d, // property name offset: reusable
        0x00, 0x00, 0x00, 0x02, // FDT_END_NODE
        0x00, 0x00, 0x00, 0x02, // FDT_END_NODE
        0x00, 0x00, 0x00, 0x02, // FDT_END_NODE
        0x00, 0x00, 0x00, 0x09, // FDT_END
        b'#', b'a', b'd', b'd', b'r', b'e', b's', b's', b'-', b'c', b'e', b'l', b'l', b's', 0x00,
        b'#', b's', b'i', b'z', b'e', b'-', b'c', b'e', b'l', b'l', b's', 0x00, b'r', b'a', b'n',
        b'g', b'e', b's', 0x00, b'r', b'e', b'g', 0x00, b'n', b'o', b'-', b'm', b'a', b'p', 0x00,
        b'r', b'e', b'u', b's', b'a', b'b', b'l', b'e', 0x00,
    ]);

    let device_tree = device_tree::DeviceTree::from_physical_address(TEST_FDT.0.as_ptr() as usize);
    let ranges = unsafe { device_tree.reserved_memory_ranges() }.expect("reserved-memory ranges");

    assert_eq!(ranges.address_cells, 2);
    assert_eq!(ranges.size_cells, 2);
    assert_eq!(ranges.node_count, 2);
    assert_eq!(ranges.range_count, 2);
    assert_eq!(ranges.reported_len(), 2);
    assert!(!ranges.truncated);
    assert_eq!(
        ranges.entries[0],
        Some(device_tree::FdtReservedMemoryRange {
            address: 0x3f00_0000,
            size: 0x0100_0000,
            no_map: true,
            reusable: false,
        })
    );
    assert_eq!(
        ranges.entries[1],
        Some(device_tree::FdtReservedMemoryRange {
            address: 0x1_0000_0000,
            size: 0x0200_0000,
            no_map: false,
            reusable: true,
        })
    );
}

#[cfg(test)]
#[test_case]
fn device_tree_reads_chosen_bootargs() {
    #[repr(align(4))]
    struct Aligned<const N: usize>([u8; N]);

    static TEST_FDT: Aligned<136> = Aligned([
        0xd0, 0x0d, 0xfe, 0xed, // magic
        0x00, 0x00, 0x00, 0x80, // totalsize
        0x00, 0x00, 0x00, 0x38, // off_dt_struct
        0x00, 0x00, 0x00, 0x68, // off_dt_strings
        0x00, 0x00, 0x00, 0x28, // off_mem_rsvmap
        0x00, 0x00, 0x00, 0x11, // version
        0x00, 0x00, 0x00, 0x10, // last_comp_version
        0x00, 0x00, 0x00, 0x00, // boot_cpuid_phys
        0x00, 0x00, 0x00, 0x09, // size_dt_strings
        0x00, 0x00, 0x00, 0x30, // size_dt_struct
        0x00, 0x00, 0x00, 0x00, // mem_rsvmap address
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // mem_rsvmap size
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, // FDT_BEGIN_NODE
        0x00, 0x00, 0x00, 0x00, // root node name
        0x00, 0x00, 0x00, 0x01, // FDT_BEGIN_NODE
        b'c', b'h', b'o', b's', b'e', b'n', 0x00, 0x00, // chosen node name
        0x00, 0x00, 0x00, 0x03, // FDT_PROP
        0x00, 0x00, 0x00, 0x04, // property length
        0x00, 0x00, 0x00, 0x00, // property name offset
        b'a', b'b', b'c', 0x00, // bootargs value
        0x00, 0x00, 0x00, 0x02, // FDT_END_NODE
        0x00, 0x00, 0x00, 0x02, // FDT_END_NODE
        0x00, 0x00, 0x00, 0x09, // FDT_END
        b'b', b'o', b'o', b't', b'a', b'r', b'g', b's', 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00,
    ]);

    let device_tree = device_tree::DeviceTree::from_physical_address(TEST_FDT.0.as_ptr() as usize);
    let bootargs = unsafe { device_tree.chosen_bootargs() }.expect("chosen bootargs");

    assert_eq!(bootargs, "abc");
}

#[cfg(test)]
#[test_case]
fn pi5_uart10_address_matches_bcm2712_soc_range() {
    assert_eq!(target::rpi5::UART10_BASE, 0x10_7d00_1000);
    assert_eq!(target::rpi5::RP1_UART0_PCIE2_BASE, 0x1f_0003_0000);
    assert_eq!(target::rpi5::RP1_UART0_FIRMWARE_BASE, 0x1c_0003_0000);
    assert_eq!(target::rpi5::RP1_UART0_GPIO14_PAD, 0x1f_000f_003c);
    assert_eq!(target::rpi5::RP1_UART0_GPIO15_PAD, 0x1f_000f_0040);
    assert_eq!(target::rpi5::RP1_UART0_GPIO14_CTRL, 0x1f_000d_0074);
    assert_eq!(target::rpi5::RP1_UART0_GPIO15_CTRL, 0x1f_000d_007c);
    assert_eq!(
        target::rpi5::RP1_UART0_BASE,
        target::rpi5::RP1_UART0_PCIE2_BASE
    );
}
