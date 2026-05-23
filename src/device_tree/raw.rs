use super::DeviceTree;

#[cfg_attr(not(any(test, talos_target_rpi5_bcm2712)), allow(dead_code))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FdtHeader {
    pub magic: u32,
    pub total_size: u32,
    pub off_dt_struct: u32,
    pub off_dt_strings: u32,
    pub off_mem_rsvmap: u32,
    pub version: u32,
    pub last_comp_version: u32,
    pub boot_cpuid_phys: u32,
    pub size_dt_strings: u32,
    pub size_dt_struct: u32,
}

#[cfg_attr(not(any(test, talos_target_rpi5_bcm2712)), allow(dead_code))]
impl FdtHeader {
    pub const MAGIC: u32 = 0xd00d_feed;
}

impl DeviceTree {
    #[cfg_attr(not(any(test, talos_target_rpi5_bcm2712)), allow(dead_code))]
    pub unsafe fn fdt_header(self) -> Option<FdtHeader> {
        let physical_address = self.physical_address?;
        let words = physical_address as *const u32;
        let header = FdtHeader {
            magic: unsafe { read_be_u32(words, 0) },
            total_size: unsafe { read_be_u32(words, 1) },
            off_dt_struct: unsafe { read_be_u32(words, 2) },
            off_dt_strings: unsafe { read_be_u32(words, 3) },
            off_mem_rsvmap: unsafe { read_be_u32(words, 4) },
            version: unsafe { read_be_u32(words, 5) },
            last_comp_version: unsafe { read_be_u32(words, 6) },
            boot_cpuid_phys: unsafe { read_be_u32(words, 7) },
            size_dt_strings: unsafe { read_be_u32(words, 8) },
            size_dt_struct: unsafe { read_be_u32(words, 9) },
        };

        if header.magic == FdtHeader::MAGIC {
            Some(header)
        } else {
            None
        }
    }
}

#[cfg_attr(not(any(test, talos_target_rpi5_bcm2712)), allow(dead_code))]
unsafe fn read_be_u32(words: *const u32, index: usize) -> u32 {
    u32::from_be(unsafe { core::ptr::read_volatile(words.add(index)) })
}

pub(super) unsafe fn read_be_u64_at(start: usize, offset: usize, limit: usize) -> Option<u64> {
    let end = offset.checked_add(8)?;
    if end > limit {
        return None;
    }

    let bytes = (start + offset) as *const u8;
    let value = ((unsafe { core::ptr::read_volatile(bytes) } as u64) << 56)
        | ((unsafe { core::ptr::read_volatile(bytes.add(1)) } as u64) << 48)
        | ((unsafe { core::ptr::read_volatile(bytes.add(2)) } as u64) << 40)
        | ((unsafe { core::ptr::read_volatile(bytes.add(3)) } as u64) << 32)
        | ((unsafe { core::ptr::read_volatile(bytes.add(4)) } as u64) << 24)
        | ((unsafe { core::ptr::read_volatile(bytes.add(5)) } as u64) << 16)
        | ((unsafe { core::ptr::read_volatile(bytes.add(6)) } as u64) << 8)
        | (unsafe { core::ptr::read_volatile(bytes.add(7)) } as u64);
    Some(value)
}

pub(super) const FDT_BEGIN_NODE: u32 = 1;
pub(super) const FDT_END_NODE: u32 = 2;
pub(super) const FDT_PROP: u32 = 3;
pub(super) const FDT_NOP: u32 = 4;
pub(super) const FDT_END: u32 = 9;

pub(super) struct FdtProperty {
    pub(super) name_offset: usize,
    pub(super) value: &'static [u8],
}

pub(super) struct FdtStructCursor {
    start: usize,
    offset: usize,
    end: usize,
}

impl FdtStructCursor {
    pub(super) fn new(start: usize, size: usize) -> Self {
        Self {
            start,
            offset: 0,
            end: size,
        }
    }

    pub(super) unsafe fn read_token(&mut self) -> Option<u32> {
        let token = unsafe { self.read_be_u32_at(self.offset)? };
        self.offset = self.offset.checked_add(4)?;
        Some(token)
    }

    pub(super) unsafe fn read_node_name(&mut self) -> Option<&'static [u8]> {
        let start = self.offset;
        let mut end = start;
        while end < self.end {
            let byte = unsafe { core::ptr::read_volatile((self.start + end) as *const u8) };
            if byte == 0 {
                let name = unsafe {
                    core::slice::from_raw_parts((self.start + start) as *const u8, end - start)
                };
                self.offset = align4(end.checked_add(1)?)?;
                return Some(name);
            }
            end = end.checked_add(1)?;
        }

        None
    }

    pub(super) unsafe fn read_property(&mut self) -> Option<FdtProperty> {
        let len = unsafe { self.read_be_u32_at(self.offset)? } as usize;
        self.offset = self.offset.checked_add(4)?;
        let name_offset = unsafe { self.read_be_u32_at(self.offset)? } as usize;
        self.offset = self.offset.checked_add(4)?;

        let value_start = self.offset;
        let value_end = value_start.checked_add(len)?;
        if value_end > self.end {
            return None;
        }

        let value =
            unsafe { core::slice::from_raw_parts((self.start + value_start) as *const u8, len) };
        self.offset = align4(value_end)?;

        Some(FdtProperty { name_offset, value })
    }

    unsafe fn read_be_u32_at(&self, offset: usize) -> Option<u32> {
        let end = offset.checked_add(4)?;
        if end > self.end {
            return None;
        }

        let bytes = (self.start + offset) as *const u8;
        let value = ((unsafe { core::ptr::read_volatile(bytes) } as u32) << 24)
            | ((unsafe { core::ptr::read_volatile(bytes.add(1)) } as u32) << 16)
            | ((unsafe { core::ptr::read_volatile(bytes.add(2)) } as u32) << 8)
            | (unsafe { core::ptr::read_volatile(bytes.add(3)) } as u32);
        Some(value)
    }
}

pub(super) fn checked_block_start(physical_address: usize, offset: u32) -> Option<usize> {
    physical_address.checked_add(offset as usize)
}

pub(super) fn validate_fdt_block(header: &FdtHeader, offset: u32, size: usize) -> Option<()> {
    let end = (offset as usize).checked_add(size)?;
    if end <= header.total_size as usize {
        Some(())
    } else {
        None
    }
}

fn align4(value: usize) -> Option<usize> {
    value.checked_add(3).map(|value| value & !3)
}

pub(super) fn fdt_node_name_matches(name: &[u8], expected: &[u8]) -> bool {
    name == expected
        || (name.starts_with(expected) && name.get(expected.len()).copied() == Some(b'@'))
}

pub(super) unsafe fn fdt_string_at(
    start: usize,
    size: usize,
    offset: usize,
) -> Option<&'static [u8]> {
    if offset >= size {
        return None;
    }

    let mut end = offset;
    while end < size {
        let byte = unsafe { core::ptr::read_volatile((start + end) as *const u8) };
        if byte == 0 {
            return Some(unsafe {
                core::slice::from_raw_parts((start + offset) as *const u8, end - offset)
            });
        }
        end = end.checked_add(1)?;
    }

    None
}

pub(super) fn fdt_string_property(value: &'static [u8]) -> Option<&'static str> {
    let string = if let Some((&0, without_nul)) = value.split_last() {
        without_nul
    } else {
        value
    };

    core::str::from_utf8(string).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    static STRINGS: &[u8] = b"bootargs\0stdout-path\0";

    #[test_case]
    fn raw_fdt_helpers_bound_alignment_blocks_and_strings() {
        let header = FdtHeader {
            magic: FdtHeader::MAGIC,
            total_size: 64,
            off_dt_struct: 16,
            off_dt_strings: 32,
            off_mem_rsvmap: 8,
            version: 17,
            last_comp_version: 16,
            boot_cpuid_phys: 0,
            size_dt_strings: 16,
            size_dt_struct: 16,
        };

        assert_eq!(align4(0), Some(0));
        assert_eq!(align4(5), Some(8));
        assert_eq!(align4(usize::MAX), None);
        assert_eq!(validate_fdt_block(&header, 16, 48), Some(()));
        assert_eq!(validate_fdt_block(&header, 17, 48), None);
        assert_eq!(
            unsafe { fdt_string_at(STRINGS.as_ptr() as usize, STRINGS.len(), 9) },
            Some(&b"stdout-path"[..])
        );
        assert_eq!(
            unsafe { fdt_string_at(STRINGS.as_ptr() as usize, STRINGS.len(), STRINGS.len()) },
            None
        );
    }
}
