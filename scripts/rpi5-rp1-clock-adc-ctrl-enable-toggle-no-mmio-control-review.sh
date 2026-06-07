#!/bin/sh
set -eu

if [ "$#" -ne 1 ]; then
    echo "usage: $0 <candidate-archive.tar.gz>" >&2
    exit 2
fi

ARCHIVE="$1"
CONTROL_MARKER="TALOS: rp1-clock-adc-ctrl-enable-toggle-control"

./scripts/rpi5-archive-review.sh "$ARCHIVE" >/dev/null

tmp_root="target/tmp"
mkdir -p "$tmp_root"
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-clock-adc-ctrl-enable-toggle-control-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-clock-adc-ctrl-enable-toggle-control: start" \
    "rpi5-rp1-clock-adc-ctrl-enable-toggle-control: no-rp1-clock-reset-gpio-rio-pads-msix-pcie-mip-gic-mmio" \
    "$CONTROL_MARKER" \
    "phase11-rp1-clock-adc-enable-toggle-source-contract-v1" \
    "rp1-clk-adc-ctrl-enable-bit-toggle-restore" \
    " register=" \
    "CLK_ADC_CTRL" \
    "clock-manager-base=not-constructed" \
    "source-offset=" \
    "address=not-constructed" \
    "width=32" \
    "transition-mask=" \
    "pre-raw=" \
    "pre-enable=" \
    "pre-auxsrc=" \
    "pre-source=" \
    "transition-raw=" \
    "post-raw=" \
    "post-enable=" \
    "post-auxsrc=" \
    "post-source=" \
    "restore-raw=" \
    "restore-enable=" \
    "restore-auxsrc=" \
    "restore-source=" \
    "one-bit-transition=true" \
    "post-enable-flipped=true" \
    "post-delta-is-transition-mask=true" \
    "restore-eq-pre=true" \
    "retained-idempotent-proof=rp1-clock-adc-ctrl-idempotent-write-restored" \
    "retained-gpio14-blocker=fsel13" \
    "retained-gpio16-blocker=fsel13" \
    "classification=simulated/control"; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing clock ADC ctrl enable-toggle control string: $required" >&2
        exit 1
    fi
done

for forbidden in \
    "rpi5-rp1-clock-adc-ctrl-enable-toggle: before-rp1-clock-enable-toggle" \
    "TALOS: rp1-clock-adc-ctrl-enable-toggle-result" \
    "rp1-clock-adc-ctrl-enable-toggle-restored" \
    "rp1-clock-adc-ctrl-enable-toggle-mismatch-restored" \
    "rp1-clock-adc-ctrl-enable-toggle-restore-failed" \
    "0x1f00018144" \
    "0x1f00018000" \
    "0x1f00020000" \
    "0x1f000d0070" \
    "0x1f000d0080" \
    "0x107fff9000" \
    "0x107fffa000"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden clock ADC ctrl enable-toggle control string: $forbidden" >&2
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
