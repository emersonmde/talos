#!/bin/sh
set -eu

if [ "$#" -ne 1 ]; then
    echo "usage: $0 <candidate-archive.tar.gz>" >&2
    exit 2
fi

ARCHIVE="$1"
CONTROL_MARKER="TALOS: rp1-sysinfo-clock-sentinel-control"

./scripts/rpi5-archive-review.sh "$ARCHIVE" >/dev/null

tmp_root="target/tmp"
mkdir -p "$tmp_root"
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-sysinfo-clock-sentinel-control-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-sysinfo-clock-sentinel-control: start" \
    "rpi5-rp1-sysinfo-clock-sentinel-control: no-rp1-sysinfo-clock-reset-gpio-rio-pads-msix-pcie-mip-gic-mmio" \
    "$CONTROL_MARKER" \
    "phase11-rp1-clock-sentinel-address-discriminator-source-contract-v1" \
    "rp1-sysinfo-vs-clock-sentinel-read" \
    "sysinfo-base=not-constructed" \
    "clock-manager-base=not-constructed" \
    "sysinfo-chip-id" \
    "sysinfo-platform" \
    "clk-adc-ctrl" \
    "-source-offset=" \
    "address=not-constructed" \
    "width=32" \
    "raw=" \
    "expected-chip-id=" \
    "chip-id-matches-expected=" \
    "chip-id-is-deaddead=" \
    "platform-is-deaddead=" \
    "adc-ctrl-is-deaddead=" \
    "sysinfo-pair-equal=" \
    "sysinfo-vs-adc-same=" \
    "retained-adc-window-classification=" \
    "retained-adc-window-clk-sys-ctrl-raw=" \
    "retained-adc-window-clk-uart-ctrl-raw=" \
    "retained-adc-window-adc-ctrl-first-raw=" \
    "retained-adc-window-adc-ctrl-second-raw=" \
    "retained-adc-window-adc-div-int-raw=" \
    "retained-adc-window-adc-sel-raw=" \
    "classification=no-mmio-sysinfo-clock-sentinel-control-visible"; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing SYSINFO clock sentinel control string: $required" >&2
        exit 1
    fi
done

for forbidden in \
    "TALOS: rp1-sysinfo-clock-sentinel-result" \
    "rpi5-rp1-sysinfo-clock-sentinel-read: before-read-only-loads" \
    "rp1-sysinfo-live-clock-window-sentinel" \
    "rp1-sysinfo-and-clock-window-sentinel" \
    "rp1-sysinfo-live-clock-window-non-sentinel" \
    "rp1-sysinfo-unexpected-chip-id" \
    "rp1-sysinfo-address-decode-blocked" \
    "0x1f00000000" \
    "0x1f00000004" \
    "0x1f00018000" \
    "0x1f00018144" \
    "0x1f000d0070" \
    "0x1f000d0080" \
    "0x107fff9000" \
    "0x107fffa000"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden SYSINFO clock sentinel control string: $forbidden" >&2
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
