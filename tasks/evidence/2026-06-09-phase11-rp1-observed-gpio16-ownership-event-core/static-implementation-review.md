Static source inspection for GPIO16 observed-aperture core

Accepted real observed reads and output/classification helpers:
173:pub const RP1_GPIO16_OBSERVED_APERTURE_STATUS: usize = 0x1c_000d_0080;
175:pub const RP1_GPIO16_OBSERVED_APERTURE_CTRL: usize = 0x1c_000d_0084;
177:pub const RP1_IO_BANK0_OBSERVED_APERTURE_INTE: usize = 0x1c_000d_011c;
179:pub const RP1_IO_BANK0_OBSERVED_APERTURE_INTS: usize = 0x1c_000d_0124;
181:pub const RP1_RIO0_OBSERVED_APERTURE_OUT: usize = 0x1c_000e_0000;
183:pub const RP1_RIO0_OBSERVED_APERTURE_OE: usize = 0x1c_000e_0004;
185:pub const RP1_RIO0_OBSERVED_APERTURE_IN: usize = 0x1c_000e_0008;
189:pub const RP1_GPIO16_OBSERVED_APERTURE_PAD: usize = 0x1c_000f_0044;
14608:    let io_bank0_inte = read_rp1_reg_u32(RP1_IO_BANK0_OBSERVED_APERTURE_INTE);
14609:    let io_bank0_ints = read_rp1_reg_u32(RP1_IO_BANK0_OBSERVED_APERTURE_INTS);
14610:    let rio_out = read_rp1_reg_u32(RP1_RIO0_OBSERVED_APERTURE_OUT);
14611:    let rio_oe = read_rp1_reg_u32(RP1_RIO0_OBSERVED_APERTURE_OE);
14612:    let rio_in = read_rp1_reg_u32(RP1_RIO0_OBSERVED_APERTURE_IN);
14656:        write_early_hex_u64(RP1_IO_BANK0_OBSERVED_APERTURE_INTE as u64);
14658:        write_early_hex_u64(RP1_IO_BANK0_OBSERVED_APERTURE_INTS as u64);
14660:        write_early_hex_u64(RP1_RIO0_OBSERVED_APERTURE_OUT as u64);
14662:        write_early_hex_u64(RP1_RIO0_OBSERVED_APERTURE_OE as u64);
14664:        write_early_hex_u64(RP1_RIO0_OBSERVED_APERTURE_IN as u64);
14822:    let preflight = read_gpio16_event_discriminator_snapshot(GIC_INTID);
14824:        gpio16_observed_ownership_event_preflight_classification(preflight, GPIO16_MASK, GIC_INTID);
14836:            RP1_GPIO16_OBSERVED_APERTURE_STATUS,
14837:            RP1_GPIO16_OBSERVED_APERTURE_CTRL,
14838:            RP1_IO_BANK0_OBSERVED_APERTURE_INTE,
14839:            RP1_IO_BANK0_OBSERVED_APERTURE_INTS,
14840:            RP1_RIO0_OBSERVED_APERTURE_OUT,
14841:            RP1_RIO0_OBSERVED_APERTURE_OE,
14842:            RP1_RIO0_OBSERVED_APERTURE_IN,
14843:            RP1_GPIO16_OBSERVED_APERTURE_PAD,
14849:        write_gpio16_ownership_event_preflight_fields(preflight, GPIO16_MASK);
14883:        write_gpio16_ownership_event_preflight_fields(SNAPSHOT, GPIO16_MASK);
14892:fn read_gpio16_event_discriminator_snapshot(gic_intid: u32) -> Gpio16EventDiscriminatorSnapshot {
14893:    let gpio16_status = read_rp1_reg_u32(RP1_GPIO16_OBSERVED_APERTURE_STATUS);
14894:    let gpio16_ctrl = read_rp1_reg_u32(RP1_GPIO16_OBSERVED_APERTURE_CTRL);
14895:    let io_bank0_inte = read_rp1_reg_u32(RP1_IO_BANK0_OBSERVED_APERTURE_INTE);
14896:    let io_bank0_ints = read_rp1_reg_u32(RP1_IO_BANK0_OBSERVED_APERTURE_INTS);
14897:    let rio_out = read_rp1_reg_u32(RP1_RIO0_OBSERVED_APERTURE_OUT);
14898:    let rio_oe = read_rp1_reg_u32(RP1_RIO0_OBSERVED_APERTURE_OE);
14899:    let rio_in = read_rp1_reg_u32(RP1_RIO0_OBSERVED_APERTURE_IN);
14900:    let pad = read_rp1_reg_u32(RP1_GPIO16_OBSERVED_APERTURE_PAD);
14930:fn gpio16_observed_ownership_event_preflight_classification(
15021:fn write_gpio16_ownership_event_preflight_fields(

Real GPIO16 function body write/static-action scan:
no real-function GPIO/RIO/pad/INTE/CTRL writes or action/restore markers found

No-MMIO control function body MMIO/address scan:
no control-function RP1/GIC address construction, volatile MMIO, action, or restore markers found

Review scripts quarantine old write-backed output strings as forbidden:
scripts/rpi5-rp1-gpio16-owned-event-discriminator-review.sh:82:    "classification=simulated/control" \
scripts/rpi5-rp1-gpio16-owned-event-discriminator-review.sh:83:    "phase11-rp1-gpio-owned-event-discriminator-source-contract-v1" \
scripts/rpi5-rp1-gpio16-owned-event-discriminator-review.sh:86:    "action-level-high-enable=" \
scripts/rpi5-rp1-gpio16-owned-event-discriminator-review.sh:87:    "restore-attempted=" \
scripts/rpi5-rp1-gpio16-owned-event-discriminator-no-mmio-control-review.sh:84:    "classification=simulated/control" \
scripts/rpi5-rp1-gpio16-owned-event-discriminator-no-mmio-control-review.sh:85:    "phase11-rp1-gpio-owned-event-discriminator-source-contract-v1" \
scripts/rpi5-rp1-gpio16-owned-event-discriminator-no-mmio-control-review.sh:88:    "restore-attempted=" \
