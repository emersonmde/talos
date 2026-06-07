#!/bin/sh
set -eu

if [ "$#" -ne 1 ]; then
    echo "usage: $0 <candidate-archive.tar.gz>" >&2
    exit 2
fi

ARCHIVE="$1"
CONTROL_MARKER="TALOS: rp1-interrupt-routing-control"

./scripts/rpi5-archive-review.sh "$ARCHIVE" >/dev/null

tmp_root="target/tmp"
mkdir -p "$tmp_root"
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-interrupt-routing-control-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-interrupt-routing-control: start" \
    "rpi5-rp1-interrupt-routing-control: no-rp1-msix-pcie-gic-mmio" \
    "$CONTROL_MARKER" \
    "phase11-rp1-interrupt-routing-source-contract-v1" \
    "target=rp1-io-bank0-msix-cfg-read" \
    "hwirq=" \
    "predicted-msix-vector=" \
    "predicted-gic-spi=" \
    "predicted-gic-intid=" \
    "address=not-constructed" \
    "width=32 raw=" \
    "enable=" \
    "test=" \
    "iack=" \
    "iack-en=" \
    "classification=simulated/control"; do
    if ! grep -Fq "$required" "$kernel_strings"; then
        echo "kernel image missing interrupt-routing no-MMIO control string: $required" >&2
        exit 1
    fi
done

for forbidden in \
    "rpi5-rp1-interrupt-routing-msix-cfg-read: before-rp1-load" \
    "classification=routing-msix-cfg-visible" \
    "0x1f00108008" \
    "0x1f000d0070" \
    "0x1f000e0008" \
    "0x1f000f003c" \
    "0x107fff9000" \
    "0x107fffa000"; do
    if grep -Fq "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden interrupt-routing control string: $forbidden" >&2
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
printf 'forbidden_interrupt_routing_strings_absent=true\n'
