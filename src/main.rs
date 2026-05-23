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
            talos_rpi5_alloc_format_diagnostic
        )
    ),
    allow(dead_code, unused_imports, unused_variables)
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
mod diagnostics;
mod early_format;
mod memory_map;
mod mmio;
mod pl011;
mod target;

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
#[cfg_attr(
    any(
        all(talos_target_rpi5_bcm2712, talos_rpi5_runtime_uart_probe_diagnostic),
        all(talos_target_rpi5_bcm2712, talos_rpi5_handoff_uart_diagnostic),
        all(talos_target_rpi5_bcm2712, talos_rpi5_rust_uart10_diagnostic),
        all(talos_target_rpi5_bcm2712, talos_rpi5_boundary_entry_reset_diagnostic),
        all(talos_target_rpi5_bcm2712, talos_rpi5_phase_ladder_diagnostic),
    ),
    allow(unreachable_code, unused_variables)
)]
pub extern "C" fn rust_entry(dtb_pa: usize) -> ! {
    #[cfg(talos_target_rpi5_bcm2712)]
    target::rpi5::write_early_phase_line(target::rpi5::EarlyPhaseLine::RustEntry);

    #[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_rust_uart10_diagnostic))]
    target::rpi5::rust_uart10_diagnostic();

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
        all(talos_target_rpi5_bcm2712, talos_rpi5_runtime_uart_probe_diagnostic),
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
