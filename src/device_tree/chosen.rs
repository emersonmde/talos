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
#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn device_tree_reads_chosen_bootargs() {
        #[repr(align(4))]
        struct Aligned<const N: usize>([u8; N]);

        static TEST_FDT: Aligned<136> = Aligned([
            0xd0, 0x0d, 0xfe, 0xed, // magic
            0x00, 0x00, 0x00, 0x80, // totalsize
            0x00, 0x00, 0x00, 0x38, // off_dt_struct
            0x00, 0x00, 0x00, 0x68, // off_dt_strings
            0x00, 0x00, 0x00, 0x28, // off_mem_rsvmap
            0x00, 0x00, 0x00, 0x11, // version
            0x00, 0x00, 0x00, 0x10, // last_comp_version
            0x00, 0x00, 0x00, 0x00, // boot_cpuid_phys
            0x00, 0x00, 0x00, 0x09, // size_dt_strings
            0x00, 0x00, 0x00, 0x30, // size_dt_struct
            0x00, 0x00, 0x00, 0x00, // mem_rsvmap address
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // mem_rsvmap size
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, // FDT_BEGIN_NODE
            0x00, 0x00, 0x00, 0x00, // root node name
            0x00, 0x00, 0x00, 0x01, // FDT_BEGIN_NODE
            b'c', b'h', b'o', b's', b'e', b'n', 0x00, 0x00, // chosen node name
            0x00, 0x00, 0x00, 0x03, // FDT_PROP
            0x00, 0x00, 0x00, 0x04, // property length
            0x00, 0x00, 0x00, 0x00, // property name offset
            b'a', b'b', b'c', 0x00, // bootargs value
            0x00, 0x00, 0x00, 0x02, // FDT_END_NODE
            0x00, 0x00, 0x00, 0x02, // FDT_END_NODE
            0x00, 0x00, 0x00, 0x09, // FDT_END
            b'b', b'o', b'o', b't', b'a', b'r', b'g', b's', 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ]);

        let device_tree = DeviceTree::from_physical_address(TEST_FDT.0.as_ptr() as usize);
        let bootargs = unsafe { device_tree.chosen_bootargs() }.expect("chosen bootargs");

        assert_eq!(bootargs, "abc");
    }
}
