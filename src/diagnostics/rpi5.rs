use crate::{KERNEL_GLOBAL_ALLOCATOR, memory_map, println, target};

pub(crate) fn run_allocator_diagnostic_or_smoke(
    _allocator_plan: memory_map::EarlyBootstrapAllocatorPlan,
) {
    rpi5_bootstrap_alloc_smoke();
}

pub(crate) unsafe fn run_exception_fault_panic_diagnostics() {}

#[cfg(all(not(test), talos_target_rpi5_bcm2712))]
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
