#![cfg_attr(not(talos_target_rpi5_bcm2712), allow(dead_code))]

use core::{
    alloc::{GlobalAlloc, Layout},
    ptr,
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::memory_map::EarlyBootstrapAllocatorPlan;

pub struct BumpAllocator {
    start: AtomicUsize,
    next: AtomicUsize,
    end: AtomicUsize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BumpAllocatorState {
    pub start: usize,
    pub next: usize,
    pub end: usize,
    pub used_bytes: usize,
    pub remaining_bytes: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BumpAllocatorAllocError {
    Uninitialized,
    InvalidAlignment,
    AddressOverflow,
    Exhausted {
        requested_size: usize,
        requested_align: usize,
        remaining_bytes: usize,
    },
}

impl BumpAllocator {
    pub const fn new() -> Self {
        Self {
            start: AtomicUsize::new(0),
            next: AtomicUsize::new(0),
            end: AtomicUsize::new(0),
        }
    }

    pub fn init_from_plan(&self, plan: EarlyBootstrapAllocatorPlan) -> Option<BumpAllocatorState> {
        let start = usize::try_from(plan.start).ok()?;
        let end = usize::try_from(plan.end).ok()?;
        if start >= end {
            return None;
        }

        self.end.store(end, Ordering::Release);
        self.start.store(start, Ordering::Release);
        self.next.store(start, Ordering::Release);

        Some(BumpAllocatorState {
            start,
            next: start,
            end,
            used_bytes: 0,
            remaining_bytes: end - start,
        })
    }

    pub fn state(&self) -> Option<BumpAllocatorState> {
        let start = self.start.load(Ordering::Acquire);
        let next = self.next.load(Ordering::Acquire);
        let end = self.end.load(Ordering::Acquire);
        if start == 0 || next < start || next > end {
            return None;
        }

        Some(BumpAllocatorState {
            start,
            next,
            end,
            used_bytes: next - start,
            remaining_bytes: end - next,
        })
    }

    pub fn try_allocate_layout(&self, layout: Layout) -> Result<*mut u8, BumpAllocatorAllocError> {
        self.try_allocate(layout.size(), layout.align())
    }

    fn try_allocate(
        &self,
        size: usize,
        alignment: usize,
    ) -> Result<*mut u8, BumpAllocatorAllocError> {
        let end = self.end.load(Ordering::Acquire);
        let mut current = self.next.load(Ordering::Acquire);
        if current == 0 || end == 0 {
            return Err(BumpAllocatorAllocError::Uninitialized);
        }

        loop {
            let aligned =
                align_up(current, alignment).ok_or(BumpAllocatorAllocError::InvalidAlignment)?;
            let alloc_end = aligned
                .checked_add(size)
                .ok_or(BumpAllocatorAllocError::AddressOverflow)?;
            if alloc_end > end {
                return Err(BumpAllocatorAllocError::Exhausted {
                    requested_size: size,
                    requested_align: alignment,
                    remaining_bytes: end.saturating_sub(current),
                });
            }

            match self.next.compare_exchange(
                current,
                alloc_end,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => return Ok(aligned as *mut u8),
                Err(next) => current = next,
            }
        }
    }
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.try_allocate_layout(layout).unwrap_or(ptr::null_mut())
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

fn align_up(value: usize, alignment: usize) -> Option<usize> {
    if alignment == 0 || !alignment.is_power_of_two() {
        return None;
    }
    let mask = alignment - 1;
    value.checked_add(mask).map(|value| value & !mask)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory_map::{EARLY_PAGE_SIZE, EarlyBootstrapAllocatorPlan};

    #[test_case]
    fn uninitialized_allocation_returns_null() {
        let allocator = BumpAllocator::new();

        let ptr = unsafe {
            GlobalAlloc::alloc(&allocator, Layout::from_size_align(8, 8).expect("layout"))
        };

        assert!(ptr.is_null());
        assert_eq!(allocator.state(), None);
    }

    #[test_case]
    fn init_state_tracks_allocator_bounds_and_capacity() {
        let allocator = BumpAllocator::new();
        let plan = EarlyBootstrapAllocatorPlan {
            start: 0x2f01_0000,
            end: 0x2f02_0000,
            page_size: EARLY_PAGE_SIZE,
            page_count: 0x10,
            size: 0x1_0000,
        };

        let state = allocator.init_from_plan(plan).expect("allocator state");

        assert_eq!(state.start, 0x2f01_0000);
        assert_eq!(state.next, 0x2f01_0000);
        assert_eq!(state.end, 0x2f02_0000);
        assert_eq!(state.used_bytes, 0);
        assert_eq!(state.remaining_bytes, 0x1_0000);
        assert_eq!(allocator.state(), Some(state));
    }

    #[test_case]
    fn allocation_advances_with_alignment_and_updates_accounting() {
        let allocator = BumpAllocator::new();
        let plan = EarlyBootstrapAllocatorPlan {
            start: 0x2f01_0000,
            end: 0x2f01_0100,
            page_size: EARLY_PAGE_SIZE,
            page_count: 1,
            size: 0x100,
        };
        allocator.init_from_plan(plan).expect("allocator state");

        let first = unsafe {
            GlobalAlloc::alloc(
                &allocator,
                Layout::from_size_align(24, 16).expect("first layout"),
            )
        };
        let second = unsafe {
            GlobalAlloc::alloc(
                &allocator,
                Layout::from_size_align(8, 64).expect("second layout"),
            )
        };

        assert_eq!(first as usize, 0x2f01_0000);
        assert_eq!(second as usize, 0x2f01_0040);

        let state = allocator.state().expect("allocator state");
        assert_eq!(state.start, 0x2f01_0000);
        assert_eq!(state.next, 0x2f01_0048);
        assert_eq!(state.used_bytes, 0x48);
        assert_eq!(state.remaining_bytes, 0xb8);
    }

    #[test_case]
    fn oversized_allocation_returns_null_without_advancing() {
        let allocator = BumpAllocator::new();
        let plan = EarlyBootstrapAllocatorPlan {
            start: 0x2f01_0000,
            end: 0x2f01_0040,
            page_size: EARLY_PAGE_SIZE,
            page_count: 1,
            size: 0x40,
        };
        allocator.init_from_plan(plan).expect("allocator state");

        let ptr = unsafe {
            GlobalAlloc::alloc(
                &allocator,
                Layout::from_size_align(0x80, 8).expect("oversized layout"),
            )
        };

        assert!(ptr.is_null());
        let state = allocator.state().expect("allocator state");
        assert_eq!(state.next, 0x2f01_0000);
        assert_eq!(state.used_bytes, 0);
        assert_eq!(state.remaining_bytes, 0x40);
    }

    #[test_case]
    fn fallible_allocation_reports_exhaustion_without_advancing() {
        let allocator = BumpAllocator::new();
        let plan = EarlyBootstrapAllocatorPlan {
            start: 0x2f01_0000,
            end: 0x2f01_0040,
            page_size: EARLY_PAGE_SIZE,
            page_count: 1,
            size: 0x40,
        };
        allocator.init_from_plan(plan).expect("allocator state");
        let before = allocator.state().expect("before state");

        let result =
            allocator.try_allocate_layout(Layout::from_size_align(0x80, 8).expect("layout"));

        assert_eq!(
            result,
            Err(BumpAllocatorAllocError::Exhausted {
                requested_size: 0x80,
                requested_align: 8,
                remaining_bytes: 0x40,
            })
        );
        assert_eq!(allocator.state(), Some(before));
    }

    #[test_case]
    fn direct_global_allocator_style_smoke_preserves_accounting() {
        let allocator = BumpAllocator::new();
        let mut buffer = [0u64; 8];
        let start = buffer.as_mut_ptr() as usize;
        let size = core::mem::size_of_val(&buffer);
        let plan = EarlyBootstrapAllocatorPlan {
            start: start as u64,
            end: (start + size) as u64,
            page_size: core::mem::size_of::<u64>() as u64,
            page_count: buffer.len() as u64,
            size: size as u64,
        };
        allocator.init_from_plan(plan).expect("allocator state");

        let layout = Layout::from_size_align(4 * core::mem::size_of::<u64>(), 8)
            .expect("global-style layout");
        let ptr = unsafe { GlobalAlloc::alloc(&allocator, layout) } as *mut u64;

        assert!(!ptr.is_null());
        unsafe {
            ptr.add(0).write_volatile(1);
            ptr.add(1).write_volatile(2);
            ptr.add(2).write_volatile(3);
            ptr.add(3).write_volatile(0x41);
        }

        let mut sum = 0u64;
        let mut index = 0usize;
        while index < 4 {
            sum += unsafe { ptr.add(index).read_volatile() };
            index += 1;
        }

        assert_eq!(sum, 0x47);
        let state = allocator.state().expect("allocator state");
        assert_eq!(state.next, start + 0x20);
        assert_eq!(state.used_bytes, 0x20);
        assert_eq!(state.remaining_bytes, size - 0x20);
    }

    #[test_case]
    fn realloc_growth_allocates_new_region_and_preserves_prefix() {
        let allocator = BumpAllocator::new();
        let mut buffer = [0u64; 8];
        let start = buffer.as_mut_ptr() as usize;
        let size = core::mem::size_of_val(&buffer);
        let plan = EarlyBootstrapAllocatorPlan {
            start: start as u64,
            end: (start + size) as u64,
            page_size: core::mem::size_of::<u64>() as u64,
            page_count: buffer.len() as u64,
            size: size as u64,
        };
        allocator.init_from_plan(plan).expect("allocator state");

        let old_layout =
            Layout::from_size_align(2 * core::mem::size_of::<u64>(), 8).expect("old layout");
        let old_ptr = unsafe { GlobalAlloc::alloc(&allocator, old_layout) } as *mut u64;
        assert!(!old_ptr.is_null());
        unsafe {
            old_ptr.add(0).write(1);
            old_ptr.add(1).write(0x41);
        }

        let new_ptr = unsafe {
            GlobalAlloc::realloc(
                &allocator,
                old_ptr.cast::<u8>(),
                old_layout,
                4 * core::mem::size_of::<u64>(),
            )
        } as *mut u64;

        assert_eq!(old_ptr as usize, start);
        assert_eq!(new_ptr as usize, start + 0x10);
        assert_eq!(unsafe { new_ptr.add(0).read() }, 1);
        assert_eq!(unsafe { new_ptr.add(1).read() }, 0x41);

        let state = allocator.state().expect("allocator state");
        assert_eq!(state.next, start + 0x30);
        assert_eq!(state.used_bytes, 0x30);
        assert_eq!(state.remaining_bytes, size - 0x30);
    }
}
