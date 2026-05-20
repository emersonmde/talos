#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod arch;
mod boot;
mod device_tree;
mod early_format;
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
    ),
    allow(unreachable_code)
)]
#[cfg(not(test))]
fn kernel_main(boot_info: &BootInfo) -> ! {
    #[cfg(talos_target_rpi5_bcm2712)]
    {
        println!("\ntalos: boot start");
        println!("talos: board raspberry-pi-5-bcm2712");

        #[cfg(talos_rpi5_rust_entry_diagnostic)]
        crate::rpi5_rust_entry_reset_probe();

        println!("talos: console early-uart static/minimal");
        target::console::write_static("talos: boot info: dtb=");
        target::console::write_hex_usize(boot_info.dtb_pa);
        target::console::write_static(" core=");
        target::console::write_dec_usize(boot_info.primary_core as usize);
        target::console::write_static(" el=");
        target::console::write_dec_usize(boot_info.exception_level as usize);
        target::console::write_static(" target=");
        target::console::write_static(boot_info.target.name());
        target::console::write_static("\n");

        let services = target::services(boot_info);
        target::console::write_static("talos: services: uart=");
        target::console::write_static(services.uart.name());
        target::console::write_static(" timer=");
        target::console::write_static(services.timer.name());
        target::console::write_static(" irq=");
        target::console::write_static(services.interrupt_controller.name());
        target::console::write_static(" mmio_regions=");
        target::console::write_dec_usize(services.mmio_map.regions().len());
        target::console::write_static(" dtb=");
        if let Some(dtb_pa) = services.device_tree.physical_address() {
            target::console::write_hex_usize(dtb_pa);
        } else {
            target::console::write_static("none");
        }
        target::console::write_static("\n");
        println!("talos: status early boot log ready");

        #[cfg(talos_rpi5_exception_report_diagnostic)]
        unsafe {
            core::arch::asm!("brk #0", options(nomem, nostack, preserves_flags));
        }

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

#[cfg(any(
    all(talos_target_rpi5_bcm2712, talos_rpi5_fmt_sink_diagnostic),
    all(talos_target_rpi5_bcm2712, talos_rpi5_fmt_static_sink_diagnostic),
    all(talos_target_rpi5_bcm2712, talos_rpi5_fmt_sink_direct_diagnostic),
    all(talos_target_rpi5_bcm2712, talos_rpi5_fmt_sink_dyn_direct_diagnostic),
    all(talos_target_rpi5_bcm2712, talos_rpi5_fmt_sink_fnptr_direct_diagnostic),
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

#[panic_handler]
#[cfg_attr(talos_target_rpi5_bcm2712, allow(unused_variables))]
fn panic(info: &PanicInfo<'_>) -> ! {
    #[cfg(talos_target_rpi5_bcm2712)]
    {
        target::console::write_static("\ntalos panic\n");
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
