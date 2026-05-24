#![cfg_attr(any(test, talos_target_rpi5_bcm2712), allow(dead_code))]

use core::arch::asm;

const CNTHP_CTL_ENABLE: u64 = 1 << 0;
const CNTHP_CTL_IMASK: u64 = 1 << 1;

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

#[cfg(test)]
mod tests {
    use super::first_smoke_delta_ticks;

    #[test_case]
    fn first_smoke_delta_has_small_frequency_floor() {
        assert_eq!(first_smoke_delta_ticks(10), 1_000);
        assert_eq!(first_smoke_delta_ticks(1_000_000), 10_000);
    }
}
