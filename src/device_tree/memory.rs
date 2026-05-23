use super::DeviceTree;
use super::raw::{
    FDT_BEGIN_NODE, FDT_END, FDT_END_NODE, FDT_NOP, FDT_PROP, FdtStructCursor, checked_block_start,
    fdt_node_name_matches, fdt_string_at, read_be_u64_at, validate_fdt_block,
};

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

impl DeviceTree {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn memory_cell_decoder_accepts_only_one_or_two_complete_cells() {
        assert_eq!(fdt_cell_value(&[0, 0, 0, 2], 1), Some(2));
        assert_eq!(
            fdt_cell_value(&[0, 0, 0, 1, 0, 0, 0, 0], 2),
            Some(0x1_0000_0000)
        );
        assert_eq!(fdt_cell_value(&[], 0), None);
        assert_eq!(fdt_cell_value(&[0, 0, 0], 1), None);
        assert_eq!(fdt_cell_value(&[0; 12], 3), None);
    }
}
