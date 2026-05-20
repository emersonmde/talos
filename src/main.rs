#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod arch;
mod boot;
mod device_tree;
mod mmio;
mod pl011;
mod target;

use core::panic::PanicInfo;

use boot::BootInfo;

#[unsafe(no_mangle)]
#[cfg_attr(
    all(talos_target_rpi5_bcm2712, talos_rpi5_rust_entry_diagnostic),
    allow(unreachable_code, unused_variables)
)]
pub extern "C" fn rust_entry(dtb_pa: usize) -> ! {
    let boot_info = BootInfo::from_aarch64_x0(dtb_pa);

    target::init(&boot_info);

    arch::aarch64::exceptions::init();

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
            "mov w11, #0x55",
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

#[cfg(not(test))]
fn kernel_main(boot_info: &BootInfo) -> ! {
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

#[panic_handler]
fn panic(info: &PanicInfo<'_>) -> ! {
    println!();
    println!("talos panic: {}", info);

    #[cfg(test)]
    target::qemu::exit_failure();

    #[cfg(not(test))]
    arch::aarch64::halt()
}

pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        print!("{} ... ", core::any::type_name::<T>());
        self();
        println!("ok");
    }
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Testable]) {
    println!();
    println!("running {} talos no_std tests", tests.len());
    for test in tests {
        test.run();
    }
    println!("test result: ok. {} passed", tests.len());
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
