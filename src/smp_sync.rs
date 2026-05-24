//! Narrow SMP synchronization primitives.
//!
//! These primitives are the first Phase 6.2 building block for data that will
//! later be shared across cores. They do not make scheduler queues SMP-safe,
//! provide sleeping locks, or hide early-boot cache maintenance.

use core::{
    cell::UnsafeCell,
    hint::spin_loop,
    marker::PhantomData,
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

#[cfg(target_arch = "aarch64")]
use crate::arch::aarch64::{
    SingleCoreIrqMaskState, single_core_irq_mask_save, single_core_irq_restore,
};

const LOCK_ACQUIRE_ORDERING: Ordering = Ordering::Acquire;
const LOCK_RELEASE_ORDERING: Ordering = Ordering::Release;
const LOCK_FAILURE_ORDERING: Ordering = Ordering::Relaxed;

pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

// A SpinLock serializes mutable access to T across CPUs. Sharing the lock is
// sound when the protected data may move between threads/cores.
unsafe impl<T: Send> Sync for SpinLock<T> {}
unsafe impl<T: Send> Send for SpinLock<T> {}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    /// Acquire the lock with acquire ordering and spin until it is available.
    ///
    /// This lock is non-recursive. Code that needs both local IRQ exclusion and
    /// SMP mutual exclusion must mask IRQs first, then acquire the spin lock.
    pub fn lock(&self) -> SpinLockGuard<'_, T> {
        while self
            .locked
            .compare_exchange_weak(false, true, LOCK_ACQUIRE_ORDERING, LOCK_FAILURE_ORDERING)
            .is_err()
        {
            while self.locked.load(Ordering::Relaxed) {
                spin_loop();
            }
        }

        SpinLockGuard {
            lock: self,
            _cpu_local: PhantomData,
        }
    }

    /// Try to acquire the lock once.
    ///
    /// A None result while this CPU already holds the guard is the supported
    /// way to observe the non-recursive misuse policy without deadlocking tests.
    pub fn try_lock(&self) -> Option<SpinLockGuard<'_, T>> {
        match self.locked.compare_exchange(
            false,
            true,
            LOCK_ACQUIRE_ORDERING,
            LOCK_FAILURE_ORDERING,
        ) {
            Ok(_) => Some(SpinLockGuard {
                lock: self,
                _cpu_local: PhantomData,
            }),
            Err(_) => None,
        }
    }

    pub fn is_locked(&self) -> bool {
        self.locked.load(Ordering::Relaxed)
    }

    #[cfg(target_arch = "aarch64")]
    /// Mask local IRQs, then acquire the SMP lock.
    ///
    /// Dropping the returned guard releases the lock before restoring the saved
    /// IRQ mask state.
    pub unsafe fn lock_irqsave(&self) -> IrqSpinLockGuard<'_, T> {
        let irq_state = unsafe { single_core_irq_mask_save() };
        let guard = self.lock();
        IrqSpinLockGuard {
            guard: ManuallyDrop::new(guard),
            irq_state,
        }
    }

    fn unlock(&self) {
        self.locked.store(false, LOCK_RELEASE_ORDERING);
    }
}

#[must_use = "dropping the guard releases the spin lock"]
pub struct SpinLockGuard<'a, T> {
    lock: &'a SpinLock<T>,
    _cpu_local: PhantomData<*mut ()>,
}

impl<T> Deref for SpinLockGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for SpinLockGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.value.get() }
    }
}

impl<T> Drop for SpinLockGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.unlock();
    }
}

#[cfg(target_arch = "aarch64")]
#[must_use = "dropping the guard releases the spin lock and restores IRQ mask state"]
pub struct IrqSpinLockGuard<'a, T> {
    guard: ManuallyDrop<SpinLockGuard<'a, T>>,
    irq_state: SingleCoreIrqMaskState,
}

#[cfg(target_arch = "aarch64")]
impl<T> Deref for IrqSpinLockGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.guard.deref()
    }
}

#[cfg(target_arch = "aarch64")]
impl<T> DerefMut for IrqSpinLockGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.guard.deref_mut()
    }
}

#[cfg(target_arch = "aarch64")]
impl<T> Drop for IrqSpinLockGuard<'_, T> {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.guard);
            single_core_irq_restore(self.irq_state);
        }
    }
}

#[cfg(target_arch = "aarch64")]
pub fn smp_full_barrier() {
    unsafe {
        core::arch::asm!("dmb ish", options(nostack, preserves_flags));
    }
}

#[cfg(not(target_arch = "aarch64"))]
pub fn smp_full_barrier() {
    core::sync::atomic::fence(Ordering::SeqCst);
}

#[cfg(test)]
mod tests {
    use super::{SpinLock, smp_full_barrier};

    #[test_case]
    fn spin_lock_starts_unlocked_and_try_lock_sets_state() {
        let lock = SpinLock::new(7u64);

        assert!(!lock.is_locked());
        {
            let guard = lock.try_lock().expect("first lock succeeds");
            assert_eq!(*guard, 7);
            assert!(lock.is_locked());
            assert!(lock.try_lock().is_none());
        }
        assert!(!lock.is_locked());
    }

    #[test_case]
    fn spin_lock_guard_mutates_and_releases_on_drop() {
        let lock = SpinLock::new(0u64);

        {
            let mut guard = lock.lock();
            *guard += 41;
            *guard += 1;
            assert_eq!(*guard, 42);
        }

        let guard = lock.try_lock().expect("dropped guard releases lock");
        assert_eq!(*guard, 42);
    }

    #[test_case]
    fn spin_lock_is_non_recursive_by_policy() {
        let lock = SpinLock::new(());
        let _guard = lock.lock();

        assert!(lock.try_lock().is_none());
    }

    #[test_case]
    fn smp_full_barrier_is_callable_from_tests() {
        smp_full_barrier();
    }

    #[cfg(target_arch = "aarch64")]
    #[test_case]
    fn irqsave_lock_composes_mask_before_guard_and_restores_on_drop() {
        let lock = SpinLock::new(1u64);

        {
            let mut guard = unsafe { lock.lock_irqsave() };
            *guard = 2;
            assert!(lock.is_locked());
        }

        let guard = lock.try_lock().expect("irqsave guard releases lock");
        assert_eq!(*guard, 2);
    }
}
