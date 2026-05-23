use super::DeviceTree;
use super::raw::{
    FDT_BEGIN_NODE, FDT_END, FDT_END_NODE, FDT_NOP, FDT_PROP, FdtStructCursor, checked_block_start,
    fdt_string_at, fdt_string_property, validate_fdt_block,
};

impl DeviceTree {
    #[cfg_attr(not(any(test, talos_target_rpi5_bcm2712)), allow(dead_code))]
    pub unsafe fn chosen_bootargs(self) -> Option<&'static str> {
        let property = unsafe { self.find_chosen_property("bootargs")? };
        fdt_string_property(property)
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
