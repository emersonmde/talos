#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod arch;
mod boot;
mod target;

use core::panic::PanicInfo;

use boot::BootInfo;

#[unsafe(no_mangle)]
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
    println!("talos: hello from qemu virt");
    println!("talos: qemu smoke PASS");
    target::qemu::exit_success();
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
