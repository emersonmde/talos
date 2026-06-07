#!/bin/sh
set -eu

if [ "$#" -ne 1 ]; then
    echo "usage: $0 <candidate-archive.tar.gz>" >&2
    exit 2
fi

ARCHIVE="$1"
CONTROL_MARKER="TALOS: rp1-clock-manager-status-control"

./scripts/rpi5-archive-review.sh "$ARCHIVE" >/dev/null

tmp_root="target/tmp"
mkdir -p "$tmp_root"
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-clock-manager-status-control-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-clock-manager-status-control: start" \
    "rpi5-rp1-clock-manager-status-control: no-rp1-clock-reset-gpio-rio-pads-msix-pcie-mip-gic-mmio" \
    "$CONTROL_MARKER" \
    "phase11-rp1-clock-reset-status-source-contract-v1" \
    "target=rp1-clock-manager-status-read" \
    "clock-manager-base=not-constructed" \
    "pll-sys-cs" \
    "clk-sys-ctrl" \
    "clk-sys-div-int" \
    "clk-sys-sel" \
    "clk-slow-sys-ctrl" \
    "clk-uart-ctrl" \
    "clk-uart-div-int" \
    "clk-uart-sel" \
    "-address=not-constructed" \
    "pll-sys-lock=" \
    "clk-sys-enabled=" \
    "clk-slow-sys-enabled=" \
    "clk-uart-enabled=" \
    "retained-gpio14-blocker=fsel13" \
    "retained-gpio16-blocker=fsel13" \
    "classification=simulated/control"; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing clock-manager no-MMIO control string: $required" >&2
        exit 1
    fi
done

for forbidden in \
    "rpi5-rp1-clock-manager-status-read: before-rp1-clock-loads" \
    "classification=rp1-clock-manager-status-visible" \
    "0x1f00018000" \
    "0x1f00020000" \
    "0x1f00018014" \
    "0x1f00018018" \
    "0x1f00018020" \
    "0x1f00018024" \
    "0x1f00018054" \
    "0x1f00018058" \
    "0x1f00018060" \
    "0x1f000d0070" \
    "0x1f000d0080" \
    "0x107fff9000" \
    "0x107fffa000"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden clock-manager control string: $forbidden" >&2
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
