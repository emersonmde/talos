#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeviceTree {
    physical_address: Option<usize>,
}

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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FdtMemoryReservation {
    pub address: u64,
    pub size: u64,
}

#[cfg_attr(not(any(test, talos_target_rpi5_bcm2712)), allow(dead_code))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FdtMemoryReservations {
    pub count: usize,
    pub entries: [Option<FdtMemoryReservation>; Self::MAX_REPORTED],
    pub truncated: bool,
}

#[cfg_attr(not(any(test, talos_target_rpi5_bcm2712)), allow(dead_code))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FdtMemoryBank {
    pub address: u64,
    pub size: u64,
}

#[cfg_attr(not(any(test, talos_target_rpi5_bcm2712)), allow(dead_code))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FdtMemoryBanks {
    pub address_cells: u8,
    pub size_cells: u8,
    pub count: usize,
    pub entries: [Option<FdtMemoryBank>; Self::MAX_REPORTED],
    pub truncated: bool,
}

#[cfg_attr(not(any(test, talos_target_rpi5_bcm2712)), allow(dead_code))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FdtReservedMemoryRange {
    pub address: u64,
    pub size: u64,
    pub no_map: bool,
    pub reusable: bool,
}

#[cfg_attr(not(any(test, talos_target_rpi5_bcm2712)), allow(dead_code))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FdtReservedMemoryRanges {
    pub address_cells: u8,
    pub size_cells: u8,
    pub node_count: usize,
    pub range_count: usize,
    pub entries: [Option<FdtReservedMemoryRange>; Self::MAX_REPORTED],
    pub truncated: bool,
}

#[cfg_attr(not(any(test, talos_target_rpi5_bcm2712)), allow(dead_code))]
impl FdtMemoryReservations {
    pub const MAX_REPORTED: usize = 4;
    const MAX_SCANNED: usize = 64;

    pub fn reported_len(self) -> usize {
        let mut len = 0;
        while len < self.entries.len() && self.entries[len].is_some() {
            len += 1;
        }
        len
    }
}

#[cfg_attr(not(any(test, talos_target_rpi5_bcm2712)), allow(dead_code))]
impl FdtMemoryBanks {
    pub const MAX_REPORTED: usize = 4;
    const MAX_SCANNED: usize = 16;

    pub fn reported_len(self) -> usize {
        let mut len = 0;
        while len < self.entries.len() && self.entries[len].is_some() {
            len += 1;
        }
        len
    }
}

#[cfg_attr(not(any(test, talos_target_rpi5_bcm2712)), allow(dead_code))]
impl FdtReservedMemoryRanges {
    pub const MAX_REPORTED: usize = 4;
    const MAX_SCANNED: usize = 32;

    pub fn reported_len(self) -> usize {
        let mut len = 0;
        while len < self.entries.len() && self.entries[len].is_some() {
            len += 1;
        }
        len
    }
}

#[cfg_attr(not(any(test, talos_target_rpi5_bcm2712)), allow(dead_code))]
impl FdtHeader {
    pub const MAGIC: u32 = 0xd00d_feed;
}

impl DeviceTree {
    pub const fn from_physical_address(physical_address: usize) -> Self {
        Self {
            physical_address: Some(physical_address),
        }
    }

    pub const fn physical_address(self) -> Option<usize> {
        self.physical_address
    }

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

    #[cfg_attr(not(any(test, talos_target_rpi5_bcm2712)), allow(dead_code))]
    pub unsafe fn chosen_bootargs(self) -> Option<&'static str> {
        let property = unsafe { self.find_chosen_property("bootargs")? };
        fdt_string_property(property)
    }

    #[cfg_attr(not(any(test, talos_target_rpi5_bcm2712)), allow(dead_code))]
    pub unsafe fn memory_reservations(self) -> Option<FdtMemoryReservations> {
        let physical_address = self.physical_address?;
        let header = unsafe { self.fdt_header()? };
        let start = checked_block_start(physical_address, header.off_mem_rsvmap)?;
        let total_size = header.total_size as usize;
        if header.off_mem_rsvmap as usize > total_size {
            return None;
        }

        let max_bytes = total_size.checked_sub(header.off_mem_rsvmap as usize)?;
        let mut entries = [None; FdtMemoryReservations::MAX_REPORTED];
        let mut count = 0usize;
        let mut offset = 0usize;

        while count < FdtMemoryReservations::MAX_SCANNED {
            let address = unsafe { read_be_u64_at(start, offset, max_bytes)? };
            let size = unsafe { read_be_u64_at(start, offset.checked_add(8)?, max_bytes)? };
            offset = offset.checked_add(16)?;

            if address == 0 && size == 0 {
                return Some(FdtMemoryReservations {
                    count,
                    entries,
                    truncated: false,
                });
            }

            if count < entries.len() {
                entries[count] = Some(FdtMemoryReservation { address, size });
            }
            count = count.checked_add(1)?;
        }

        Some(FdtMemoryReservations {
            count,
            entries,
            truncated: true,
        })
    }

    #[cfg_attr(not(any(test, talos_target_rpi5_bcm2712)), allow(dead_code))]
    pub unsafe fn memory_banks(self) -> Option<FdtMemoryBanks> {
        let physical_address = self.physical_address?;
        let header = unsafe { self.fdt_header()? };
        let struct_start = checked_block_start(physical_address, header.off_dt_struct)?;
        let struct_size = header.size_dt_struct as usize;
        let strings_start = checked_block_start(physical_address, header.off_dt_strings)?;
        let strings_size = header.size_dt_strings as usize;

        validate_fdt_block(&header, header.off_dt_struct, struct_size)?;
        validate_fdt_block(&header, header.off_dt_strings, strings_size)?;

        let mut cursor = FdtStructCursor::new(struct_start, struct_size);
        let mut depth = 0usize;
        let mut root_address_cells = 2u8;
        let mut root_size_cells = 1u8;
        let mut memory_depth = None;
        let mut saw_memory_node = false;
        let mut count = 0usize;
        let mut entries = [None; FdtMemoryBanks::MAX_REPORTED];
        let mut truncated = false;

        while let Some(token) = unsafe { cursor.read_token() } {
            match token {
                FDT_BEGIN_NODE => {
                    let name = unsafe { cursor.read_node_name()? };
                    let new_depth = depth.checked_add(1)?;
                    if new_depth == 2 && fdt_node_name_matches(name, b"memory") {
                        memory_depth = Some(new_depth);
                        saw_memory_node = true;
                    }
                    depth = new_depth;
                }
                FDT_END_NODE => {
                    if memory_depth == Some(depth) {
                        return Some(FdtMemoryBanks {
                            address_cells: root_address_cells,
                            size_cells: root_size_cells,
                            count,
                            entries,
                            truncated,
                        });
                    }
                    depth = depth.checked_sub(1)?;
                }
                FDT_PROP => {
                    let property = unsafe { cursor.read_property()? };
                    let name = unsafe {
                        fdt_string_at(strings_start, strings_size, property.name_offset)?
                    };

                    if depth == 1 {
                        match name {
                            b"#address-cells" => {
                                root_address_cells = fdt_single_cell_property(property.value)?;
                            }
                            b"#size-cells" => {
                                root_size_cells = fdt_single_cell_property(property.value)?;
                            }
                            _ => {}
                        }
                    }

                    if memory_depth == Some(depth) && name == b"reg" {
                        parse_memory_reg_property(
                            property.value,
                            root_address_cells,
                            root_size_cells,
                            &mut count,
                            &mut entries,
                            &mut truncated,
                        )?;
                    }
                }
                FDT_NOP => {}
                FDT_END => {
                    if saw_memory_node {
                        return Some(FdtMemoryBanks {
                            address_cells: root_address_cells,
                            size_cells: root_size_cells,
                            count,
                            entries,
                            truncated,
                        });
                    }
                    return None;
                }
                _ => return None,
            }
        }

        None
    }

    #[cfg_attr(not(any(test, talos_target_rpi5_bcm2712)), allow(dead_code))]
    pub unsafe fn reserved_memory_ranges(self) -> Option<FdtReservedMemoryRanges> {
        let physical_address = self.physical_address?;
        let header = unsafe { self.fdt_header()? };
        let struct_start = checked_block_start(physical_address, header.off_dt_struct)?;
        let struct_size = header.size_dt_struct as usize;
        let strings_start = checked_block_start(physical_address, header.off_dt_strings)?;
        let strings_size = header.size_dt_strings as usize;

        validate_fdt_block(&header, header.off_dt_struct, struct_size)?;
        validate_fdt_block(&header, header.off_dt_strings, strings_size)?;

        let mut cursor = FdtStructCursor::new(struct_start, struct_size);
        let mut depth = 0usize;
        let mut root_address_cells = 2u8;
        let mut root_size_cells = 1u8;
        let mut reserved_address_cells = root_address_cells;
        let mut reserved_size_cells = root_size_cells;
        let mut reserved_depth = None;
        let mut child_depth = None;
        let mut child_range_start = 0usize;
        let mut child_no_map = false;
        let mut child_reusable = false;
        let mut saw_reserved_memory = false;
        let mut node_count = 0usize;
        let mut range_count = 0usize;
        let mut entries = [None; FdtReservedMemoryRanges::MAX_REPORTED];
        let mut truncated = false;

        while let Some(token) = unsafe { cursor.read_token() } {
            match token {
                FDT_BEGIN_NODE => {
                    let name = unsafe { cursor.read_node_name()? };
                    let new_depth = depth.checked_add(1)?;
                    if new_depth == 2 && name == b"reserved-memory" {
                        reserved_depth = Some(new_depth);
                        saw_reserved_memory = true;
                        reserved_address_cells = root_address_cells;
                        reserved_size_cells = root_size_cells;
                    } else if reserved_depth == Some(depth) {
                        child_depth = Some(new_depth);
                        child_range_start = range_count;
                        child_no_map = false;
                        child_reusable = false;
                        node_count = node_count.checked_add(1)?;
                    }
                    depth = new_depth;
                }
                FDT_END_NODE => {
                    if child_depth == Some(depth) {
                        child_depth = None;
                    } else if reserved_depth == Some(depth) {
                        return Some(FdtReservedMemoryRanges {
                            address_cells: reserved_address_cells,
                            size_cells: reserved_size_cells,
                            node_count,
                            range_count,
                            entries,
                            truncated,
                        });
                    }
                    depth = depth.checked_sub(1)?;
                }
                FDT_PROP => {
                    let property = unsafe { cursor.read_property()? };
                    let name = unsafe {
                        fdt_string_at(strings_start, strings_size, property.name_offset)?
                    };

                    if depth == 1 {
                        match name {
                            b"#address-cells" => {
                                root_address_cells = fdt_single_cell_property(property.value)?;
                            }
                            b"#size-cells" => {
                                root_size_cells = fdt_single_cell_property(property.value)?;
                            }
                            _ => {}
                        }
                    }

                    if reserved_depth == Some(depth) {
                        match name {
                            b"#address-cells" => {
                                reserved_address_cells = fdt_single_cell_property(property.value)?;
                            }
                            b"#size-cells" => {
                                reserved_size_cells = fdt_single_cell_property(property.value)?;
                            }
                            _ => {}
                        }
                    }

                    if child_depth == Some(depth) {
                        match name {
                            b"no-map" => {
                                child_no_map = true;
                                mark_reserved_memory_child_entries(
                                    &mut entries,
                                    child_range_start,
                                    range_count,
                                    child_no_map,
                                    child_reusable,
                                );
                            }
                            b"reusable" => {
                                child_reusable = true;
                                mark_reserved_memory_child_entries(
                                    &mut entries,
                                    child_range_start,
                                    range_count,
                                    child_no_map,
                                    child_reusable,
                                );
                            }
                            b"reg" => parse_reserved_memory_reg_property(
                                property.value,
                                reserved_address_cells,
                                reserved_size_cells,
                                child_no_map,
                                child_reusable,
                                &mut range_count,
                                &mut entries,
                                &mut truncated,
                            )?,
                            _ => {}
                        }
                    }
                }
                FDT_NOP => {}
                FDT_END => {
                    if saw_reserved_memory {
                        return Some(FdtReservedMemoryRanges {
                            address_cells: reserved_address_cells,
                            size_cells: reserved_size_cells,
                            node_count,
                            range_count,
                            entries,
                            truncated,
                        });
                    }
                    return None;
                }
                _ => return None,
            }
        }

        None
    }

    #[cfg_attr(not(any(test, talos_target_rpi5_bcm2712)), allow(dead_code))]
    unsafe fn find_chosen_property(self, property_name: &str) -> Option<&'static [u8]> {
        let physical_address = self.physical_address?;
        let header = unsafe { self.fdt_header()? };
        let struct_start = checked_block_start(physical_address, header.off_dt_struct)?;
        let struct_size = header.size_dt_struct as usize;
        let strings_start = checked_block_start(physical_address, header.off_dt_strings)?;
        let strings_size = header.size_dt_strings as usize;

        validate_fdt_block(&header, header.off_dt_struct, struct_size)?;
        validate_fdt_block(&header, header.off_dt_strings, strings_size)?;

        let mut cursor = FdtStructCursor::new(struct_start, struct_size);
        let mut depth = 0usize;
        let mut chosen_depth = None;

        while let Some(token) = unsafe { cursor.read_token() } {
            match token {
                FDT_BEGIN_NODE => {
                    let name = unsafe { cursor.read_node_name()? };
                    let new_depth = depth.checked_add(1)?;
                    if new_depth == 2 && name == b"chosen" {
                        chosen_depth = Some(new_depth);
                    }
                    depth = new_depth;
                }
                FDT_END_NODE => {
                    if chosen_depth == Some(depth) {
                        chosen_depth = None;
                    }
                    depth = depth.checked_sub(1)?;
                }
                FDT_PROP => {
                    let property = unsafe { cursor.read_property()? };
                    if chosen_depth == Some(depth) {
                        let name = unsafe {
                            fdt_string_at(strings_start, strings_size, property.name_offset)?
                        };
                        if name == property_name.as_bytes() {
                            return Some(property.value);
                        }
                    }
                }
                FDT_NOP => {}
                FDT_END => return None,
                _ => return None,
            }
        }

        None
    }
}

#[cfg_attr(not(any(test, talos_target_rpi5_bcm2712)), allow(dead_code))]
unsafe fn read_be_u32(words: *const u32, index: usize) -> u32 {
    u32::from_be(unsafe { core::ptr::read_volatile(words.add(index)) })
}

unsafe fn read_be_u64_at(start: usize, offset: usize, limit: usize) -> Option<u64> {
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

const FDT_BEGIN_NODE: u32 = 1;
const FDT_END_NODE: u32 = 2;
const FDT_PROP: u32 = 3;
const FDT_NOP: u32 = 4;
const FDT_END: u32 = 9;

struct FdtProperty {
    name_offset: usize,
    value: &'static [u8],
}

struct FdtStructCursor {
    start: usize,
    offset: usize,
    end: usize,
}

impl FdtStructCursor {
    fn new(start: usize, size: usize) -> Self {
        Self {
            start,
            offset: 0,
            end: size,
        }
    }

    unsafe fn read_token(&mut self) -> Option<u32> {
        let token = unsafe { self.read_be_u32_at(self.offset)? };
        self.offset = self.offset.checked_add(4)?;
        Some(token)
    }

    unsafe fn read_node_name(&mut self) -> Option<&'static [u8]> {
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

    unsafe fn read_property(&mut self) -> Option<FdtProperty> {
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

fn checked_block_start(physical_address: usize, offset: u32) -> Option<usize> {
    physical_address.checked_add(offset as usize)
}

fn validate_fdt_block(header: &FdtHeader, offset: u32, size: usize) -> Option<()> {
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

fn fdt_node_name_matches(name: &[u8], expected: &[u8]) -> bool {
    name == expected
        || (name.starts_with(expected) && name.get(expected.len()).copied() == Some(b'@'))
}

unsafe fn fdt_string_at(start: usize, size: usize, offset: usize) -> Option<&'static [u8]> {
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

fn fdt_string_property(value: &'static [u8]) -> Option<&'static str> {
    let string = if let Some((&0, without_nul)) = value.split_last() {
        without_nul
    } else {
        value
    };

    core::str::from_utf8(string).ok()
}

fn fdt_single_cell_property(value: &[u8]) -> Option<u8> {
    if value.len() != 4 {
        return None;
    }

    let cells = fdt_cell_value(value, 1)? as u8;
    if cells == 1 || cells == 2 {
        Some(cells)
    } else {
        None
    }
}

fn parse_memory_reg_property(
    value: &[u8],
    address_cells: u8,
    size_cells: u8,
    count: &mut usize,
    entries: &mut [Option<FdtMemoryBank>; FdtMemoryBanks::MAX_REPORTED],
    truncated: &mut bool,
) -> Option<()> {
    let address_cells = address_cells as usize;
    let size_cells = size_cells as usize;
    let cells_per_bank = address_cells.checked_add(size_cells)?;
    let bytes_per_bank = cells_per_bank.checked_mul(4)?;
    if bytes_per_bank == 0 || value.len() % bytes_per_bank != 0 {
        return None;
    }

    let mut offset = 0usize;
    while offset < value.len() {
        if *count >= FdtMemoryBanks::MAX_SCANNED {
            *truncated = true;
            return Some(());
        }

        let address = fdt_cell_value(&value[offset..], address_cells)?;
        offset = offset.checked_add(address_cells.checked_mul(4)?)?;
        let size = fdt_cell_value(&value[offset..], size_cells)?;
        offset = offset.checked_add(size_cells.checked_mul(4)?)?;

        if *count < entries.len() {
            entries[*count] = Some(FdtMemoryBank { address, size });
        } else {
            *truncated = true;
        }
        *count = (*count).checked_add(1)?;
    }

    Some(())
}

fn parse_reserved_memory_reg_property(
    value: &[u8],
    address_cells: u8,
    size_cells: u8,
    no_map: bool,
    reusable: bool,
    count: &mut usize,
    entries: &mut [Option<FdtReservedMemoryRange>; FdtReservedMemoryRanges::MAX_REPORTED],
    truncated: &mut bool,
) -> Option<()> {
    let address_cells = address_cells as usize;
    let size_cells = size_cells as usize;
    let cells_per_range = address_cells.checked_add(size_cells)?;
    let bytes_per_range = cells_per_range.checked_mul(4)?;
    if bytes_per_range == 0 || value.len() % bytes_per_range != 0 {
        return None;
    }

    let mut offset = 0usize;
    while offset < value.len() {
        if *count >= FdtReservedMemoryRanges::MAX_SCANNED {
            *truncated = true;
            return Some(());
        }

        let address = fdt_cell_value(&value[offset..], address_cells)?;
        offset = offset.checked_add(address_cells.checked_mul(4)?)?;
        let size = fdt_cell_value(&value[offset..], size_cells)?;
        offset = offset.checked_add(size_cells.checked_mul(4)?)?;

        if *count < entries.len() {
            entries[*count] = Some(FdtReservedMemoryRange {
                address,
                size,
                no_map,
                reusable,
            });
        } else {
            *truncated = true;
        }
        *count = (*count).checked_add(1)?;
    }

    Some(())
}

fn mark_reserved_memory_child_entries(
    entries: &mut [Option<FdtReservedMemoryRange>; FdtReservedMemoryRanges::MAX_REPORTED],
    start: usize,
    end: usize,
    no_map: bool,
    reusable: bool,
) {
    let mut index = start;
    while index < end && index < entries.len() {
        if let Some(mut entry) = entries[index] {
            entry.no_map = no_map;
            entry.reusable = reusable;
            entries[index] = Some(entry);
        }
        index += 1;
    }
}

fn fdt_cell_value(value: &[u8], cells: usize) -> Option<u64> {
    if cells == 0 || cells > 2 {
        return None;
    }

    let mut result = 0u64;
    let mut cell = 0usize;
    while cell < cells {
        let offset = cell.checked_mul(4)?;
        let end = offset.checked_add(4)?;
        if end > value.len() {
            return None;
        }
        let word = ((value[offset] as u64) << 24)
            | ((value[offset + 1] as u64) << 16)
            | ((value[offset + 2] as u64) << 8)
            | (value[offset + 3] as u64);
        result = (result << 32) | word;
        cell += 1;
    }

    Some(result)
}
