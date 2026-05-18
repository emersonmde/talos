use core::arch::asm;

pub mod exceptions;

pub fn current_el() -> u8 {
    let el: u64;
    unsafe {
        asm!("mrs {el}, CurrentEL", el = out(reg) el, options(nomem, nostack, preserves_flags));
    }
    ((el >> 2) & 0b11) as u8
}

pub fn halt() -> ! {
    loop {
        unsafe {
            asm!("wfe", options(nomem, nostack, preserves_flags));
        }
    }
}
