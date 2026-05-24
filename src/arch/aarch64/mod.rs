use core::arch::asm;

#[cfg(talos_target_rpi5_bcm2712)]
use crate::memory_map;

pub mod exceptions;
pub mod generic_timer;
pub mod gicv2;

pub fn current_el() -> u8 {
    let el: u64;
    unsafe {
        asm!("mrs {el}, CurrentEL", el = out(reg) el, options(nomem, nostack, preserves_flags));
    }
    ((el >> 2) & 0b11) as u8
}

#[allow(dead_code)]
pub fn current_vbar() -> u64 {
    let vbar: u64;
    match current_el() {
        1 => unsafe {
            asm!("mrs {vbar}, VBAR_EL1", vbar = out(reg) vbar, options(nomem, nostack, preserves_flags));
        },
        2 => unsafe {
            asm!("mrs {vbar}, VBAR_EL2", vbar = out(reg) vbar, options(nomem, nostack, preserves_flags));
        },
        3 => unsafe {
            asm!("mrs {vbar}, VBAR_EL3", vbar = out(reg) vbar, options(nomem, nostack, preserves_flags));
        },
        _ => halt(),
    }
    vbar
}

#[allow(dead_code)]
pub unsafe fn enable_alignment_faults_current_el() {
    const SCTLR_A: u64 = 1 << 1;
    let sctlr: u64;

    match current_el() {
        1 => unsafe {
            asm!("mrs {sctlr}, SCTLR_EL1", sctlr = out(reg) sctlr, options(nostack, preserves_flags));
            let sctlr = sctlr | SCTLR_A;
            asm!("msr SCTLR_EL1, {sctlr}", "isb", sctlr = in(reg) sctlr, options(nostack, preserves_flags));
        },
        2 => unsafe {
            asm!("mrs {sctlr}, SCTLR_EL2", sctlr = out(reg) sctlr, options(nostack, preserves_flags));
            let sctlr = sctlr | SCTLR_A;
            asm!("msr SCTLR_EL2, {sctlr}", "isb", sctlr = in(reg) sctlr, options(nostack, preserves_flags));
        },
        3 => unsafe {
            asm!("mrs {sctlr}, SCTLR_EL3", sctlr = out(reg) sctlr, options(nostack, preserves_flags));
            let sctlr = sctlr | SCTLR_A;
            asm!("msr SCTLR_EL3, {sctlr}", "isb", sctlr = in(reg) sctlr, options(nostack, preserves_flags));
        },
        _ => halt(),
    }
}

#[cfg(talos_target_rpi5_bcm2712)]
#[allow(dead_code)]
pub unsafe fn enable_el2_mmu_from_plan(
    plan: memory_map::EarlyTranslationRegisterPlan,
) -> Option<u64> {
    if current_el() != 2 || plan.current_el != 2 {
        return None;
    }

    let mut sctlr: u64;
    unsafe {
        asm!(
            "msr MAIR_EL2, {mair}",
            "msr TCR_EL2, {tcr}",
            "msr TTBR0_EL2, {ttbr0}",
            "isb",
            "tlbi alle2",
            "dsb sy",
            "isb",
            mair = in(reg) plan.mair,
            tcr = in(reg) plan.tcr,
            ttbr0 = in(reg) plan.ttbr0,
            options(nostack, preserves_flags)
        );

        asm!("mrs {sctlr}, SCTLR_EL2", sctlr = out(reg) sctlr, options(nostack, preserves_flags));
        sctlr |= plan.sctlr_set;
        asm!(
            "msr SCTLR_EL2, {sctlr}",
            "isb",
            sctlr = in(reg) sctlr,
            options(nostack, preserves_flags)
        );
    }

    Some(sctlr)
}

#[cfg(talos_target_rpi5_bcm2712)]
#[allow(dead_code)]
pub fn current_el2_sctlr() -> Option<u64> {
    if current_el() != 2 {
        return None;
    }

    let sctlr: u64;
    unsafe {
        asm!("mrs {sctlr}, SCTLR_EL2", sctlr = out(reg) sctlr, options(nomem, nostack, preserves_flags));
    }
    Some(sctlr)
}

#[cfg(talos_target_rpi5_bcm2712)]
#[allow(dead_code)]
pub unsafe fn enable_el2_instruction_cache_from_plan(
    plan: memory_map::EarlyInstructionCacheEnablePlan,
) -> Option<u64> {
    if current_el() != 2 || plan.current_el != 2 {
        return None;
    }

    let mut sctlr = current_el2_sctlr()?;
    if (sctlr & memory_map::EARLY_TRANSLATION_SCTLR_M_ENABLE) == 0 {
        return None;
    }

    unsafe {
        asm!(
            "ic iallu",
            "dsb sy",
            "isb",
            options(nostack, preserves_flags)
        );
        sctlr |= plan.sctlr_set;
        asm!(
            "msr SCTLR_EL2, {sctlr}",
            "isb",
            sctlr = in(reg) sctlr,
            options(nostack, preserves_flags)
        );
    }

    Some(sctlr)
}

#[cfg(talos_target_rpi5_bcm2712)]
#[allow(dead_code)]
pub unsafe fn enable_el2_data_cache_from_plan(
    plan: memory_map::EarlyDataCacheEnablePlan,
) -> Option<u64> {
    if current_el() != 2 || plan.current_el != 2 {
        return None;
    }

    let required =
        memory_map::EARLY_TRANSLATION_SCTLR_M_ENABLE | memory_map::EARLY_TRANSLATION_SCTLR_I_ENABLE;
    let mut sctlr = current_el2_sctlr()?;
    if (sctlr & required) != required {
        return None;
    }

    unsafe {
        invalidate_data_unified_caches_by_set_way();
        sctlr |= plan.sctlr_set;
        asm!(
            "msr SCTLR_EL2, {sctlr}",
            "dsb sy",
            "isb",
            sctlr = in(reg) sctlr,
            options(nostack, preserves_flags)
        );
    }

    Some(sctlr)
}

#[cfg(talos_target_rpi5_bcm2712)]
unsafe fn invalidate_data_unified_caches_by_set_way() {
    let clidr: u64;
    unsafe {
        asm!("mrs {clidr}, CLIDR_EL1", clidr = out(reg) clidr, options(nostack, preserves_flags));
    }

    let loc = (clidr >> 24) & 0x7;
    let mut level = 0u64;
    while level < loc {
        let cache_type = (clidr >> (level * 3)) & 0x7;
        if cache_type == 2 || cache_type == 3 || cache_type == 4 {
            let csselr = level << 1;
            let ccsidr: u64;
            unsafe {
                asm!(
                    "msr CSSELR_EL1, {csselr}",
                    "isb",
                    "mrs {ccsidr}, CCSIDR_EL1",
                    csselr = in(reg) csselr,
                    ccsidr = out(reg) ccsidr,
                    options(nostack, preserves_flags)
                );
            }

            let line_shift = (ccsidr & 0x7) + 4;
            let ways = ((ccsidr >> 3) & 0x3ff) + 1;
            let sets = ((ccsidr >> 13) & 0x7fff) + 1;
            let way_shift = 32 - ceil_log2_u64(ways);

            let mut way = ways;
            while way > 0 {
                way -= 1;
                let mut set = sets;
                while set > 0 {
                    set -= 1;
                    let operand = (way << way_shift) | (set << line_shift) | (level << 1);
                    unsafe {
                        asm!("dc isw, {operand}", operand = in(reg) operand, options(nostack, preserves_flags));
                    }
                }
            }
        }
        level += 1;
    }

    unsafe {
        asm!("dsb sy", "isb", options(nostack, preserves_flags));
    }
}

#[cfg(talos_target_rpi5_bcm2712)]
fn ceil_log2_u64(value: u64) -> u64 {
    let mut bits = 0u64;
    let mut remaining = value.saturating_sub(1);
    while remaining > 0 {
        bits += 1;
        remaining >>= 1;
    }
    bits
}

pub fn halt() -> ! {
    loop {
        unsafe {
            asm!("wfe", options(nomem, nostack, preserves_flags));
        }
    }
}

#[allow(dead_code)]
pub unsafe fn enable_irq() {
    unsafe {
        asm!(
            "msr DAIFClr, #2",
            "isb",
            options(nomem, nostack, preserves_flags)
        );
    }
}

#[allow(dead_code)]
pub unsafe fn disable_irq() {
    unsafe {
        asm!(
            "msr DAIFSet, #2",
            "isb",
            options(nomem, nostack, preserves_flags)
        );
    }
}

#[allow(dead_code)]
pub fn daif() -> u64 {
    let value: u64;
    unsafe {
        asm!("mrs {value}, DAIF", value = out(reg) value, options(nomem, nostack, preserves_flags));
    }
    value
}

#[allow(dead_code)]
pub unsafe fn route_physical_irqs_to_el2() -> u64 {
    const HCR_EL2_IMO: u64 = 1 << 4;
    let mut value: u64;
    unsafe {
        asm!("mrs {value}, HCR_EL2", value = out(reg) value, options(nomem, nostack, preserves_flags));
        value |= HCR_EL2_IMO;
        asm!(
            "msr HCR_EL2, {value}",
            "isb",
            value = in(reg) value,
            options(nomem, nostack, preserves_flags)
        );
    }
    value
}
