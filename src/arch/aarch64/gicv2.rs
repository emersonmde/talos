const GICD_CTLR: usize = 0x000;
const GICD_ISENABLER: usize = 0x100;
const GICD_ICENABLER: usize = 0x180;
const GICD_ISPENDR: usize = 0x200;
const GICD_ISACTIVER: usize = 0x300;
const GICD_IPRIORITYR: usize = 0x400;

const GICC_CTLR: usize = 0x00;
const GICC_PMR: usize = 0x04;
const GICC_IAR: usize = 0x0c;
const GICC_EOIR: usize = 0x10;
const GICC_HPPIR: usize = 0x18;

pub const SPURIOUS_INTID: u32 = 1023;

#[derive(Clone, Copy)]
pub struct GicV2 {
    distributor_base: usize,
    cpu_interface_base: usize,
}

impl GicV2 {
    pub const fn new(distributor_base: usize, cpu_interface_base: usize) -> Self {
        Self {
            distributor_base,
            cpu_interface_base,
        }
    }

    pub unsafe fn enable_ppi_or_spi(self, intid: u32) {
        let bit = 1u32 << (intid & 31);
        let bank = (intid as usize / 32) * 4;

        unsafe {
            self.write_distributor(GICD_CTLR, 0);
            self.write_distributor(GICD_ICENABLER + bank, bit);
            self.write_priority(intid, 0x80);
            self.write_distributor(GICD_ISENABLER + bank, bit);
            self.write_cpu_interface(GICC_PMR, 0xff);
            self.write_cpu_interface(GICC_CTLR, 0x3);
            self.write_distributor(GICD_CTLR, 0x3);
            core::arch::asm!("dsb sy", "isb", options(nostack, preserves_flags));
        }
    }

    pub unsafe fn acknowledge(self) -> u32 {
        unsafe { self.read_cpu_interface(GICC_IAR) }
    }

    pub unsafe fn end_interrupt(self, iar: u32) {
        unsafe {
            self.write_cpu_interface(GICC_EOIR, iar);
        }
    }

    pub unsafe fn enable_bits(self, intid: u32) -> u32 {
        let bank = (intid as usize / 32) * 4;
        unsafe { self.read_distributor(GICD_ISENABLER + bank) }
    }

    pub unsafe fn pending_bits(self, intid: u32) -> u32 {
        let bank = (intid as usize / 32) * 4;
        unsafe { self.read_distributor(GICD_ISPENDR + bank) }
    }

    pub unsafe fn active_bits(self, intid: u32) -> u32 {
        let bank = (intid as usize / 32) * 4;
        unsafe { self.read_distributor(GICD_ISACTIVER + bank) }
    }

    pub unsafe fn highest_pending(self) -> u32 {
        unsafe { self.read_cpu_interface(GICC_HPPIR) }
    }

    unsafe fn write_priority(self, intid: u32, priority: u8) {
        let addr = (self.distributor_base + GICD_IPRIORITYR + intid as usize) as *mut u8;
        unsafe {
            core::ptr::write_volatile(addr, priority);
        }
    }

    unsafe fn read_distributor(self, offset: usize) -> u32 {
        let addr = (self.distributor_base + offset) as *const u32;
        unsafe { core::ptr::read_volatile(addr) }
    }

    unsafe fn write_distributor(self, offset: usize, value: u32) {
        let addr = (self.distributor_base + offset) as *mut u32;
        unsafe {
            core::ptr::write_volatile(addr, value);
        }
    }

    unsafe fn read_cpu_interface(self, offset: usize) -> u32 {
        let addr = (self.cpu_interface_base + offset) as *const u32;
        unsafe { core::ptr::read_volatile(addr) }
    }

    unsafe fn write_cpu_interface(self, offset: usize, value: u32) {
        let addr = (self.cpu_interface_base + offset) as *mut u32;
        unsafe {
            core::ptr::write_volatile(addr, value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SPURIOUS_INTID;

    #[test_case]
    fn spurious_intid_matches_gicv2_architecture() {
        assert_eq!(SPURIOUS_INTID, 1023);
    }
}
