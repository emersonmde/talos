#!/bin/sh
set -eu

if [ "$#" -ne 1 ]; then
    echo "usage: $0 <candidate-archive.tar.gz>" >&2
    exit 2
fi

ARCHIVE="$1"
CONTROL_MARKER="TALOS: rp1-gpio16-owned-event-discriminator-control"

./scripts/rpi5-archive-review.sh "$ARCHIVE" >/dev/null

tmp_root="target/tmp"
mkdir -p "$tmp_root"
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-gpio16-owned-event-discriminator-control-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-gpio16-owned-event-discriminator-control: start" \
    "rpi5-rp1-gpio16-owned-event-discriminator-control: no-rp1-gpio-rio-pads-clock-reset-msix-pcie-mip-gic-mmio" \
    "$CONTROL_MARKER" \
    "phase11-rp1-gpio-owned-event-discriminator-source-contract-v1" \
    "rp1-gpio16-owned-level-high-event-discriminator" \
    "pin=GPIO16" \
    "gpio16-bit-mask=" \
    "gpio16-status-address=not-constructed" \
    "gpio16-ctrl-address=not-constructed" \
    "io-bank0-inte-address=not-constructed" \
    "io-bank0-ints-address=not-constructed" \
    "rio-out-address=not-constructed" \
    "rio-oe-address=not-constructed" \
    "rio-in-address=not-constructed" \
    "pad-address=not-constructed" \
    "gicd-isenabler5-address=not-constructed" \
    "gicd-ispendr5-address=not-constructed" \
    "gicd-isactiver5-address=not-constructed" \
    "gicc-hppir-address=not-constructed" \
    "-gpio16-status-raw=" \
    " pre" \
    " post" \
    " restore" \
    "action-skipped=" \
    "restore-attempted=" \
    "true" \
    "false" \
    "classification=simulated/control"; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing GPIO16 no-MMIO control string: $required" >&2
        exit 1
    fi
done

for forbidden in \
    "TALOS: rp1-gpio16-owned-event-discriminator-result" \
    "0x1f000d0080" \
    "0x1f000d0084" \
    "0x1f000d2084" \
    "0x1f000d3084" \
    "0x1f000d011c" \
    "0x1f000d211c" \
    "0x1f000d311c" \
    "0x1f000d0124" \
    "0x1f000e0000" \
    "0x1f000e0004" \
    "0x1f000e0008" \
    "0x1f000f0044" \
    "0x107fff9114" \
    "0x107fff9214" \
    "0x107fff9314" \
    "0x107fffa018"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden GPIO16 no-MMIO control string: $forbidden" >&2
        exit 1
    fi
done

image_size="$(wc -c < "$extract_dir/kernel_2712.img" | tr -d ' ')"
text_offset="$(od -An -tu8 -j8 -N8 "$extract_dir/kernel_2712.img" | tr -d ' ')"
header_image_size="$(od -An -tu8 -j16 -N8 "$extract_dir/kernel_2712.img" | tr -d ' ')"
flags="$(od -An -tu8 -j24 -N8 "$extract_dir/kernel_2712.img" | tr -d ' ')"

printf 'candidate_archive=%s\n' "$ARCHIVE"
printf 'candidate_archive_sha256=%s\n' "$(sha256sum "$ARCHIVE" | awk '{print $1}')"
printf 'kernel_2712_sha256=%s\n' "$(sha256sum "$extract_dir/kernel_2712.img" | awk '{print $1}')"
printf 'kernel_2712_size=%s\n' "$image_size"
printf 'header_image_size=%s\n' "$header_image_size"
printf 'text_offset=%s\n' "$text_offset"
printf 'flags=%s\n' "$flags"
printf 'control_marker=%s\n' "$CONTROL_MARKER"
printf 'forbidden_gpio16_event_mmio_strings_absent=true\n'
