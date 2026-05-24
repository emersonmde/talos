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
            talos_rpi5_exception_report_diagnostic,
            talos_rpi5_normal_exception_report_diagnostic,
            talos_rpi5_undefined_instruction_report_diagnostic,
            talos_rpi5_data_abort_report_diagnostic,
            talos_rpi5_translation_fault_diagnostic,
            talos_rpi5_current_sp0_sync_diagnostic,
            talos_rpi5_exception_return_diagnostic,
            talos_rpi5_panic_report_diagnostic,
            talos_rpi5_full_panic_info_diagnostic,
            talos_rpi5_nested_panic_diagnostic,
            talos_rpi5_alloc_oom_diagnostic,
            talos_rpi5_realloc_growth_diagnostic,
            talos_rpi5_vec_growth_diagnostic,
            talos_rpi5_string_growth_diagnostic,
            talos_rpi5_alloc_format_diagnostic,
            talos_rpi5_page_frame_reuse_diagnostic,
            talos_rpi5_timer_preemption_diagnostic,
            talos_rpi5_diagnostic_command_channel_proof
        )
    ),
    allow(dead_code, unused_imports, unused_variables)
)]
#![cfg_attr(
    all(
        not(test),
        any(
            talos_qemu_polling_tty_rx_diagnostic,
            talos_qemu_diagnostic_command_channel_smoke,
            talos_qemu_secondary_core_discriminator
        )
    ),
    allow(dead_code, unused_imports, unused_variables, unreachable_code)
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
#[cfg_attr(
    all(
        not(test),
        not(any(
            talos_qemu_diagnostic_command_channel_smoke,
            talos_rpi5_diagnostic_command_channel_proof
        ))
    ),
    allow(dead_code)
)]
mod diagnostic_command;
mod diagnostics;
mod early_format;
mod memory_map;
mod mmio;
mod pl011;
mod runtime_console;
// Phase 4.3 accepts scheduler data structures before wiring boot-time use.
#[cfg_attr(not(test), allow(dead_code))]
mod scheduler;
mod target;
#[cfg_attr(
    not(any(
        talos_qemu_polling_tty_rx_diagnostic,
        talos_qemu_diagnostic_command_channel_smoke,
        talos_rpi5_diagnostic_command_channel_proof
    )),
    allow(dead_code)
)]
mod tty;

use core::panic::PanicInfo;
#[cfg(talos_target_rpi5_bcm2712)]
pub(crate) struct PanicInProgress(core::cell::UnsafeCell<bool>);

use boot::BootInfo;

#[cfg_attr(not(test), global_allocator)]
#[cfg_attr(not(talos_target_rpi5_bcm2712), allow(dead_code))]
pub(crate) static KERNEL_GLOBAL_ALLOCATOR: allocator::BumpAllocator =
    allocator::BumpAllocator::new();

#[cfg(talos_target_rpi5_bcm2712)]
unsafe impl Sync for PanicInProgress {}

#[cfg(talos_target_rpi5_bcm2712)]
impl PanicInProgress {
    const fn new() -> Self {
        Self(core::cell::UnsafeCell::new(false))
    }

    #[cfg(talos_rpi5_nested_panic_diagnostic)]
    pub(crate) fn prearm(&self) {
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
pub(crate) static PANIC_IN_PROGRESS: PanicInProgress = PanicInProgress::new();

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

#[unsafe(no_mangle)]
pub extern "C" fn rust_entry(dtb_pa: usize) -> ! {
    #[cfg(talos_target_rpi5_bcm2712)]
    target::rpi5::write_early_phase_line(target::rpi5::EarlyPhaseLine::RustEntry);

    let boot_info = BootInfo::from_aarch64_x0(dtb_pa);

    #[cfg(talos_target_rpi5_bcm2712)]
    target::rpi5::write_early_phase_line(target::rpi5::EarlyPhaseLine::BootInfoParsed);

    target::init(&boot_info);

    #[cfg(talos_target_rpi5_bcm2712)]
    target::rpi5::write_early_phase_line(target::rpi5::EarlyPhaseLine::TargetInit);

    arch::aarch64::exceptions::init();

    #[cfg(talos_target_rpi5_bcm2712)]
    target::rpi5::write_early_phase_line(target::rpi5::EarlyPhaseLine::ExceptionsReady);

    #[cfg(test)]
    {
        test_main();
        target::qemu::exit_success();
    }

    #[cfg(not(test))]
    kernel_main(&boot_info)
}

#[cfg_attr(
    any(
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
    boot::rpi5::kernel_main(boot_info);

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
        if boot_info.target == target::TargetKind::QemuVirt && boot_info.exception_level == 2 {
            #[cfg(talos_qemu_secondary_core_discriminator)]
            {
                if target::qemu_virt::run_secondary_core_discriminator() {
                    target::qemu::exit_success();
                }
                target::qemu::exit_failure();
            }

            #[cfg(talos_qemu_diagnostic_command_channel_smoke)]
            {
                if target::qemu_virt::run_diagnostic_command_channel_smoke() {
                    target::qemu::exit_success();
                }
                target::qemu::exit_failure();
            }

            #[cfg(talos_qemu_polling_tty_rx_diagnostic)]
            {
                if target::qemu_virt::run_polling_tty_rx_diagnostic() {
                    target::qemu::exit_success();
                }
                target::qemu::exit_failure();
            }

            #[cfg(talos_qemu_timer_preemption_smoke)]
            {
                if target::qemu_virt::run_el2_timer_preemption_smoke() {
                    target::qemu::exit_success();
                }
                target::qemu::exit_failure();
            }

            #[cfg(all(
                not(talos_qemu_timer_preemption_smoke),
                talos_qemu_scheduler_yield_smoke
            ))]
            {
                if target::qemu_virt::run_el2_scheduler_yield_smoke() {
                    target::qemu::exit_success();
                }
                target::qemu::exit_failure();
            }

            #[cfg(all(
                not(talos_qemu_timer_preemption_smoke),
                not(talos_qemu_scheduler_yield_smoke),
                talos_qemu_context_switch_smoke
            ))]
            {
                if target::qemu_virt::run_el2_context_switch_smoke() {
                    target::qemu::exit_success();
                }
                target::qemu::exit_failure();
            }

            #[cfg(not(any(
                talos_qemu_diagnostic_command_channel_smoke,
                talos_qemu_polling_tty_rx_diagnostic,
                talos_qemu_timer_preemption_smoke,
                talos_qemu_scheduler_yield_smoke,
                talos_qemu_context_switch_smoke,
                talos_qemu_secondary_core_discriminator
            )))]
            if target::qemu_virt::run_el2_timer_irq_smoke() {
                target::qemu::exit_success();
            }
            #[cfg(not(any(
                talos_qemu_diagnostic_command_channel_smoke,
                talos_qemu_timer_preemption_smoke,
                talos_qemu_scheduler_yield_smoke,
                talos_qemu_context_switch_smoke,
                talos_qemu_secondary_core_discriminator
            )))]
            target::qemu::exit_failure();
        }
        println!("talos: hello from {}", boot_info.target.name());
        println!("talos: qemu smoke PASS");
        match boot_info.target {
            target::TargetKind::QemuVirt => target::qemu::exit_success(),
            target::TargetKind::Rpi5Bcm2712 => arch::aarch64::halt(),
        }
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
