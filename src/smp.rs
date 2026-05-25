//! Minimal Phase 6 per-core state and stack ownership.
//!
//! This module is deliberately smaller than an SMP scheduler. It records CPU
//! identity, lifecycle, and stack ownership for secondary-core bring-up, but it
//! does not provide locks, migration, load balancing, IPIs, or cross-core task
//! scheduling.

use core::sync::atomic::{AtomicU64, Ordering};

pub const MAX_CORES: usize = 4;
pub const SECONDARY_KERNEL_STACK_SIZE: usize = 4096;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u64)]
pub enum CoreLifecycle {
    Parked = 0,
    Entered = 1,
    StackReady = 2,
    Registered = 3,
    HandoffReady = 4,
    WorkloadRunning = 5,
    WorkloadComplete = 6,
}

impl CoreLifecycle {
    pub const fn from_raw(raw: u64) -> Option<Self> {
        match raw {
            0 => Some(Self::Parked),
            1 => Some(Self::Entered),
            2 => Some(Self::StackReady),
            3 => Some(Self::Registered),
            4 => Some(Self::HandoffReady),
            5 => Some(Self::WorkloadRunning),
            6 => Some(Self::WorkloadComplete),
            _ => None,
        }
    }

    pub const fn raw(self) -> u64 {
        self as u64
    }

    pub const fn name(self) -> &'static str {
        match self {
            Self::Parked => "parked",
            Self::Entered => "entered",
            Self::StackReady => "stack-ready",
            Self::Registered => "registered",
            Self::HandoffReady => "handoff-ready",
            Self::WorkloadRunning => "workload-running",
            Self::WorkloadComplete => "workload-complete",
        }
    }
}

pub const SECONDARY_CORE_WORKLOAD_TARGET: u64 = 64;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CoreRegistration {
    pub logical_cpu: usize,
    pub context: usize,
    pub mpidr: u64,
    pub affinity: u64,
    pub stack_pointer: usize,
    pub lifecycle: CoreLifecycle,
    pub workload_progress: u64,
}

pub struct PerCoreState {
    lifecycle: AtomicU64,
    context: AtomicU64,
    mpidr: AtomicU64,
    affinity: AtomicU64,
    stack_pointer: AtomicU64,
    workload_progress: AtomicU64,
}

impl PerCoreState {
    pub const fn new() -> Self {
        Self {
            lifecycle: AtomicU64::new(CoreLifecycle::Parked.raw()),
            context: AtomicU64::new(0),
            mpidr: AtomicU64::new(0),
            affinity: AtomicU64::new(0),
            stack_pointer: AtomicU64::new(0),
            workload_progress: AtomicU64::new(0),
        }
    }

    pub fn reset(&self) {
        self.context.store(0, Ordering::Release);
        self.mpidr.store(0, Ordering::Release);
        self.affinity.store(0, Ordering::Release);
        self.stack_pointer.store(0, Ordering::Release);
        self.workload_progress.store(0, Ordering::Release);
        self.lifecycle
            .store(CoreLifecycle::Parked.raw(), Ordering::Release);
    }

    pub fn enter(&self, context: usize, mpidr: u64, affinity: u64) {
        self.context.store(context as u64, Ordering::Release);
        self.mpidr.store(mpidr, Ordering::Release);
        self.affinity.store(affinity, Ordering::Release);
        self.lifecycle
            .store(CoreLifecycle::Entered.raw(), Ordering::Release);
    }

    pub fn mark_stack_ready(&self, stack_pointer: usize) {
        self.stack_pointer
            .store(stack_pointer as u64, Ordering::Release);
        self.lifecycle
            .store(CoreLifecycle::StackReady.raw(), Ordering::Release);
    }

    pub fn mark_registered(&self) {
        self.lifecycle
            .store(CoreLifecycle::Registered.raw(), Ordering::Release);
    }

    pub fn republish_identity(
        &self,
        context: usize,
        mpidr: u64,
        affinity: u64,
        stack_pointer: usize,
    ) {
        self.context.store(context as u64, Ordering::Release);
        self.mpidr.store(mpidr, Ordering::Release);
        self.affinity.store(affinity, Ordering::Release);
        self.stack_pointer
            .store(stack_pointer as u64, Ordering::Release);
    }

    pub fn mark_handoff_ready(&self) {
        self.lifecycle
            .store(CoreLifecycle::HandoffReady.raw(), Ordering::Release);
    }

    pub fn mark_workload_running(&self) {
        self.workload_progress.store(0, Ordering::Release);
        self.lifecycle
            .store(CoreLifecycle::WorkloadRunning.raw(), Ordering::Release);
    }

    pub fn record_workload_progress(&self, progress: u64) {
        self.workload_progress.store(progress, Ordering::Release);
    }

    pub fn mark_workload_complete(&self, progress: u64) {
        self.workload_progress.store(progress, Ordering::Release);
        self.lifecycle
            .store(CoreLifecycle::WorkloadComplete.raw(), Ordering::Release);
    }

    pub fn snapshot(&self, logical_cpu: usize) -> CoreRegistration {
        CoreRegistration {
            logical_cpu,
            context: self.context.load(Ordering::Acquire) as usize,
            mpidr: self.mpidr.load(Ordering::Acquire),
            affinity: self.affinity.load(Ordering::Acquire),
            stack_pointer: self.stack_pointer.load(Ordering::Acquire) as usize,
            lifecycle: CoreLifecycle::from_raw(self.lifecycle.load(Ordering::Acquire))
                .unwrap_or(CoreLifecycle::Parked),
            workload_progress: self.workload_progress.load(Ordering::Acquire),
        }
    }

    pub fn clean_to_poc(&self) {
        clean_cache_line_to_poc(&self.lifecycle);
        clean_cache_line_to_poc(&self.context);
        clean_cache_line_to_poc(&self.mpidr);
        clean_cache_line_to_poc(&self.affinity);
        clean_cache_line_to_poc(&self.stack_pointer);
        clean_cache_line_to_poc(&self.workload_progress);
    }

    #[cfg(any(
        talos_rpi5_psci_secondary_core_alive_proof,
        talos_rpi5_secondary_core_workload_proof,
        talos_rpi5_smp_lock_cache_coherence_proof,
        talos_rpi5_cross_core_ipi_delivery_proof,
        talos_rpi5_remote_wakeup_request_proof,
        talos_rpi5_production_secondary_dispatch_proof
    ))]
    pub fn invalidate_from_poc(&self) {
        invalidate_cache_line_from_poc(&self.lifecycle);
        invalidate_cache_line_from_poc(&self.context);
        invalidate_cache_line_from_poc(&self.mpidr);
        invalidate_cache_line_from_poc(&self.affinity);
        invalidate_cache_line_from_poc(&self.stack_pointer);
        invalidate_cache_line_from_poc(&self.workload_progress);
    }
}

pub fn run_controlled_secondary_workload(state: &PerCoreState, target: u64) -> u64 {
    state.mark_workload_running();
    state.clean_to_poc();

    let mut progress = 0;
    while progress < target {
        progress += 1;
        state.record_workload_progress(progress);
        if progress == target || progress & 0xf == 0 {
            state.clean_to_poc();
        }
        core::hint::spin_loop();
    }

    state.mark_workload_complete(progress);
    state.clean_to_poc();
    progress
}

pub static SECONDARY_CORE_STATES: [PerCoreState; MAX_CORES] =
    [const { PerCoreState::new() }; MAX_CORES];

pub fn reset_secondary_core_states() {
    for state in SECONDARY_CORE_STATES.iter() {
        state.reset();
        state.clean_to_poc();
    }
}

#[cfg(target_arch = "aarch64")]
fn clean_cache_line_to_poc<T>(value: &T) {
    unsafe {
        core::arch::asm!(
            "dc cvac, {addr}",
            "dsb sy",
            addr = in(reg) value as *const T as usize,
            options(nostack, preserves_flags)
        );
    }
}

#[cfg(not(target_arch = "aarch64"))]
fn clean_cache_line_to_poc<T>(_value: &T) {}

#[cfg(all(
    target_arch = "aarch64",
    any(
        talos_rpi5_psci_secondary_core_alive_proof,
        talos_rpi5_secondary_core_workload_proof,
        talos_rpi5_smp_lock_cache_coherence_proof,
        talos_rpi5_cross_core_ipi_delivery_proof,
        talos_rpi5_remote_wakeup_request_proof,
        talos_rpi5_production_secondary_dispatch_proof
    )
))]
fn invalidate_cache_line_from_poc<T>(value: &T) {
    unsafe {
        core::arch::asm!(
            "dc ivac, {addr}",
            "dsb sy",
            addr = in(reg) value as *const T as usize,
            options(nostack, preserves_flags)
        );
    }
}

#[cfg(all(
    not(target_arch = "aarch64"),
    any(
        talos_rpi5_psci_secondary_core_alive_proof,
        talos_rpi5_secondary_core_workload_proof,
        talos_rpi5_smp_lock_cache_coherence_proof,
        talos_rpi5_cross_core_ipi_delivery_proof,
        talos_rpi5_remote_wakeup_request_proof,
        talos_rpi5_production_secondary_dispatch_proof
    )
))]
fn invalidate_cache_line_from_poc<T>(_value: &T) {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CoreStackSlot {
    pub logical_cpu: usize,
    pub bottom: usize,
    pub top: usize,
}

impl CoreStackSlot {
    pub const fn contains_stack_pointer(self, stack_pointer: usize) -> bool {
        self.bottom <= stack_pointer && stack_pointer <= self.top
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CoreStackLayout {
    base: usize,
    end: usize,
    cores: usize,
    stack_size: usize,
}

impl CoreStackLayout {
    pub const fn new(base: usize, end: usize, cores: usize, stack_size: usize) -> Option<Self> {
        let expected_size = match cores.checked_mul(stack_size) {
            Some(size) => size,
            None => return None,
        };
        if cores == 0
            || cores > MAX_CORES
            || stack_size == 0
            || end < base
            || end - base < expected_size
        {
            None
        } else {
            Some(Self {
                base,
                end,
                cores,
                stack_size,
            })
        }
    }

    pub const fn slot(self, logical_cpu: usize) -> Option<CoreStackSlot> {
        if logical_cpu >= self.cores {
            return None;
        }
        let bottom = self.base + logical_cpu * self.stack_size;
        let top = bottom + self.stack_size;
        if top > self.end {
            None
        } else {
            Some(CoreStackSlot {
                logical_cpu,
                bottom,
                top,
            })
        }
    }
}

pub const fn pi5_logical_cpu_from_mpidr_affinity(affinity: u64) -> Option<usize> {
    match affinity {
        0x000 => Some(0),
        0x100 => Some(1),
        0x200 => Some(2),
        0x300 => Some(3),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CoreLifecycle, CoreStackLayout, MAX_CORES, SECONDARY_CORE_STATES,
        SECONDARY_KERNEL_STACK_SIZE, pi5_logical_cpu_from_mpidr_affinity,
        reset_secondary_core_states,
    };

    #[test_case]
    fn core_lifecycle_names_match_bringup_contract() {
        assert_eq!(CoreLifecycle::Parked.name(), "parked");
        assert_eq!(CoreLifecycle::Entered.name(), "entered");
        assert_eq!(CoreLifecycle::StackReady.name(), "stack-ready");
        assert_eq!(CoreLifecycle::Registered.name(), "registered");
        assert_eq!(CoreLifecycle::HandoffReady.name(), "handoff-ready");
        assert_eq!(CoreLifecycle::WorkloadRunning.name(), "workload-running");
        assert_eq!(CoreLifecycle::WorkloadComplete.name(), "workload-complete");
        assert!(CoreLifecycle::HandoffReady > CoreLifecycle::Registered);
        assert!(CoreLifecycle::WorkloadComplete > CoreLifecycle::HandoffReady);
    }

    #[test_case]
    fn per_core_state_records_identity_stack_and_handoff() {
        reset_secondary_core_states();
        let state = &SECONDARY_CORE_STATES[2];
        state.enter(2, 0x8000_0002, 2);
        state.mark_stack_ready(0x4022_1fa0);
        state.mark_registered();
        state.mark_handoff_ready();

        let snapshot = state.snapshot(2);
        assert_eq!(snapshot.logical_cpu, 2);
        assert_eq!(snapshot.context, 2);
        assert_eq!(snapshot.mpidr, 0x8000_0002);
        assert_eq!(snapshot.affinity, 2);
        assert_eq!(snapshot.stack_pointer, 0x4022_1fa0);
        assert_eq!(snapshot.lifecycle, CoreLifecycle::HandoffReady);
        assert_eq!(snapshot.workload_progress, 0);
    }

    #[test_case]
    fn controlled_workload_records_progress_and_completion() {
        reset_secondary_core_states();
        let state = &SECONDARY_CORE_STATES[1];
        state.enter(1, 0x8000_0001, 1);
        state.mark_stack_ready(0x4022_0fa0);
        state.mark_registered();
        state.mark_handoff_ready();

        assert_eq!(super::SECONDARY_CORE_WORKLOAD_TARGET, 64);

        let progress = super::run_controlled_secondary_workload(state, 8);
        let snapshot = state.snapshot(1);

        assert_eq!(progress, 8);
        assert_eq!(snapshot.lifecycle, CoreLifecycle::WorkloadComplete);
        assert_eq!(snapshot.workload_progress, 8);
    }

    #[test_case]
    fn republish_identity_refreshes_identity_without_resetting_progress() {
        reset_secondary_core_states();
        let state = &SECONDARY_CORE_STATES[3];
        state.enter(0, 0, 0);
        state.mark_workload_running();
        state.record_workload_progress(7);

        state.republish_identity(3, 0x8100_0300, 0x300, 0x21c_f50);
        let snapshot = state.snapshot(3);

        assert_eq!(snapshot.logical_cpu, 3);
        assert_eq!(snapshot.context, 3);
        assert_eq!(snapshot.mpidr, 0x8100_0300);
        assert_eq!(snapshot.affinity, 0x300);
        assert_eq!(snapshot.stack_pointer, 0x21c_f50);
        assert_eq!(snapshot.lifecycle, CoreLifecycle::WorkloadRunning);
        assert_eq!(snapshot.workload_progress, 7);
    }

    #[test_case]
    fn stack_layout_gives_each_possible_core_a_distinct_slot() {
        let base = 0x4020_0000;
        let end = base + MAX_CORES * SECONDARY_KERNEL_STACK_SIZE;
        let layout = CoreStackLayout::new(base, end, MAX_CORES, SECONDARY_KERNEL_STACK_SIZE)
            .expect("valid secondary stack layout");

        for logical_cpu in 0..MAX_CORES {
            let slot = layout.slot(logical_cpu).expect("slot for each core");
            assert_eq!(slot.logical_cpu, logical_cpu);
            assert_eq!(
                slot.bottom,
                base + logical_cpu * SECONDARY_KERNEL_STACK_SIZE
            );
            assert_eq!(slot.top, slot.bottom + SECONDARY_KERNEL_STACK_SIZE);
            assert!(slot.contains_stack_pointer(slot.top));
            assert!(slot.contains_stack_pointer(slot.top - 16));
            assert!(!slot.contains_stack_pointer(slot.bottom - 1));
        }

        assert!(layout.slot(MAX_CORES).is_none());
    }

    #[test_case]
    fn pi5_mpidr_affinity_maps_four_cortex_a76_cores() {
        assert_eq!(pi5_logical_cpu_from_mpidr_affinity(0x000), Some(0));
        assert_eq!(pi5_logical_cpu_from_mpidr_affinity(0x100), Some(1));
        assert_eq!(pi5_logical_cpu_from_mpidr_affinity(0x200), Some(2));
        assert_eq!(pi5_logical_cpu_from_mpidr_affinity(0x300), Some(3));
        assert_eq!(pi5_logical_cpu_from_mpidr_affinity(0x400), None);
        assert_eq!(pi5_logical_cpu_from_mpidr_affinity(0x001), None);
    }
}
