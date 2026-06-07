#!/bin/sh
set -eu

if [ "$#" -ne 1 ]; then
    echo "usage: $0 <candidate-archive.tar.gz>" >&2
    exit 2
fi

ARCHIVE="$1"
CONTROL_MARKER="TALOS: rp1-gic-route-status-control"

./scripts/rpi5-archive-review.sh "$ARCHIVE" >/dev/null

tmp_root="target/tmp"
mkdir -p "$tmp_root"
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-gic-visible-route-control-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-gic-visible-route-control: start" \
    "rpi5-rp1-gic-visible-route-control: no-gic-rp1-msix-pcie-mip-gpio-pads-rio-clock-reset-mmio" \
    "$CONTROL_MARKER" \
    "phase11-rp1-gic-visible-route-source-contract-v1" \
    "target=rp1-io-bank0-gic-route-status-read" \
    "hwirq=" \
    "predicted-msix-vector=" \
    "predicted-gic-spi=" \
    "predicted-gic-intid=" \
    "gicd-base=not-constructed" \
    "gicc-base=not-constructed" \
    "bit-mask=not-constructed" \
    "isenabler-address=not-constructed" \
    "ispendr-address=not-constructed" \
    "isactiver-address=not-constructed" \
    "hppir-address=not-constructed" \
    "isenabler-raw=" \
    "ispendr-raw=" \
    "isactiver-raw=" \
    "intid-enabled=" \
    "intid-pending=" \
    "intid-active=" \
    "hppir-raw=" \
    "hppir-intid=0" \
    "hppir-spurious=false" \
    "hppir-target-match=false" \
    "classification=simulated/control"; do
    if ! grep -Fq "$required" "$kernel_strings"; then
        echo "kernel image missing GIC-visible route no-MMIO control string: $required" >&2
        exit 1
    fi
done

for forbidden in \
    "rpi5-rp1-gic-visible-route-status-read: before-gic-loads" \
    "classification=gic-route-status-visible" \
    "0x107fff9114" \
    "0x107fff9214" \
    "0x107fff9314" \
    "0x107fffa018" \
    "0x107fff9000" \
    "0x107fffa000" \
    "0x1f00108008" \
    "0x1f000d0070" \
    "0x1f000f003c"; do
    if grep -Fq "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden GIC-visible route control string: $forbidden" >&2
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
printf 'forbidden_gic_visible_route_strings_absent=true\n'
