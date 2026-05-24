use core::arch::asm;
use core::sync::atomic::{AtomicU64, Ordering};

const CNTHP_CTL_ENABLE: u64 = 1 << 0;
const CNTHP_CTL_IMASK: u64 = 1 << 1;
const PERIODIC_TICK_PROOF_COUNT: u64 = 4;

static MONOTONIC_TICKS: AtomicU64 = AtomicU64::new(0);
static MONOTONIC_TICK_DELTA: AtomicU64 = AtomicU64::new(0);

pub fn counter_frequency_hz() -> u64 {
    let value: u64;
    unsafe {
        asm!("mrs {value}, CNTFRQ_EL0", value = out(reg) value, options(nomem, nostack, preserves_flags));
    }
    value
}

pub fn physical_count() -> u64 {
    let value: u64;
    unsafe {
        asm!("mrs {value}, CNTPCT_EL0", value = out(reg) value, options(nomem, nostack, preserves_flags));
    }
    value
}

pub unsafe fn program_el2_physical_compare(compare_value: u64) {
    unsafe {
        asm!(
            "msr CNTHP_CVAL_EL2, {compare_value}",
            "msr CNTHP_CTL_EL2, {control}",
            "isb",
            compare_value = in(reg) compare_value,
            control = in(reg) CNTHP_CTL_ENABLE,
            options(nomem, nostack, preserves_flags)
        );
    }
}

pub unsafe fn program_el2_physical_delta(delta_ticks: u64) -> u64 {
    let compare_value = physical_count().wrapping_add(delta_ticks);
    unsafe {
        program_el2_physical_compare(compare_value);
    }
    compare_value
}

pub unsafe fn mask_el2_physical_timer() {
    unsafe {
        asm!(
            "msr CNTHP_CTL_EL2, {control}",
            "isb",
            control = in(reg) CNTHP_CTL_IMASK,
            options(nomem, nostack, preserves_flags)
        );
    }
}

pub fn el2_physical_control() -> u64 {
    let value: u64;
    unsafe {
        asm!("mrs {value}, CNTHP_CTL_EL2", value = out(reg) value, options(nomem, nostack, preserves_flags));
    }
    value
}

pub fn first_smoke_delta_ticks(freq: u64) -> u64 {
    let centisecond = freq / 100;
    if centisecond < 1_000 {
        1_000
    } else {
        centisecond
    }
}

pub fn periodic_tick_delta_ticks(freq: u64) -> u64 {
    first_smoke_delta_ticks(freq)
}

pub const fn periodic_tick_proof_count() -> u64 {
    PERIODIC_TICK_PROOF_COUNT
}

pub fn reset_monotonic_ticks() {
    MONOTONIC_TICKS.store(0, Ordering::Relaxed);
}

pub fn monotonic_ticks() -> u64 {
    MONOTONIC_TICKS.load(Ordering::Relaxed)
}

pub fn configure_periodic_tick_delta(delta_ticks: u64) {
    MONOTONIC_TICK_DELTA.store(delta_ticks, Ordering::Relaxed);
}

pub fn configured_periodic_tick_delta() -> u64 {
    MONOTONIC_TICK_DELTA.load(Ordering::Relaxed)
}

pub unsafe fn record_el2_physical_tick_and_rearm() -> u64 {
    let tick = MONOTONIC_TICKS.fetch_add(1, Ordering::Relaxed) + 1;
    let delta_ticks = configured_periodic_tick_delta();
    unsafe {
        program_el2_physical_delta(delta_ticks);
    }
    tick
}

#[cfg(test)]
mod tests {
    use super::{
        configure_periodic_tick_delta, configured_periodic_tick_delta, first_smoke_delta_ticks,
        monotonic_ticks, periodic_tick_delta_ticks, periodic_tick_proof_count,
        reset_monotonic_ticks,
    };

    #[test_case]
    fn first_smoke_delta_has_small_frequency_floor() {
        assert_eq!(first_smoke_delta_ticks(10), 1_000);
        assert_eq!(first_smoke_delta_ticks(1_000_000), 10_000);
    }

    #[test_case]
    fn periodic_tick_policy_uses_centisecond_floor() {
        assert_eq!(periodic_tick_delta_ticks(1), 1_000);
        assert_eq!(periodic_tick_delta_ticks(62_500_000), 625_000);
        assert_eq!(periodic_tick_proof_count(), 4);
    }

    #[test_case]
    fn monotonic_tick_counter_resets() {
        reset_monotonic_ticks();
        assert_eq!(monotonic_ticks(), 0);
    }

    #[test_case]
    fn periodic_tick_delta_can_be_configured() {
        configure_periodic_tick_delta(1234);
        assert_eq!(configured_periodic_tick_delta(), 1234);
    }
}
