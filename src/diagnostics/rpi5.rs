#[allow(unused_imports)]
use crate::boot::rpi5_reports::write_rpi5_bool;
#[allow(unused_imports)]
use crate::{KERNEL_GLOBAL_ALLOCATOR, PANIC_IN_PROGRESS, arch, println, target};

#[cfg(all(talos_target_rpi5_bcm2712, talos_rpi5_exception_return_diagnostic))]
unsafe extern "C" {
    fn rpi5_brk_register_preserve_probe(after_x9: *mut u64, after_x19: *mut u64) -> u64;
}

pub(crate) fn run_allocator_diagnostic_or_smoke() {
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
}

pub(crate) unsafe fn run_exception_fault_panic_diagnostics() {
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
            let preserved = rpi5_brk_register_preserve_probe(&mut after_x9, &mut after_x19) != 0;

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
}

#[cfg(all(
    not(test),
    talos_target_rpi5_bcm2712,
    talos_rpi5_translation_fault_diagnostic
))]
#[inline(never)]
pub(crate) unsafe fn rpi5_translation_fault_diagnostic() -> ! {
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
