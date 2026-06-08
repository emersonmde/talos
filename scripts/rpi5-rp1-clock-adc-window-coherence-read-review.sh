#!/bin/sh
set -eu

if [ "$#" -ne 1 ]; then
    echo "usage: $0 <candidate-archive.tar.gz>" >&2
    exit 2
fi

ARCHIVE="$1"
RESULT_MARKER="TALOS: rp1-clock-adc-window-coherence-result"

./scripts/rpi5-archive-review.sh "$ARCHIVE" >/dev/null

tmp_root="target/tmp"
mkdir -p "$tmp_root"
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-clock-adc-window-coherence-read-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-clock-adc-window-coherence-read: start" \
    "rpi5-rp1-clock-adc-window-coherence-read: before-rp1-clock-window-loads" \
    "$RESULT_MARKER" \
    "phase11-rp1-clock-write-effect-discriminator-source-contract-v1" \
    "rp1-clk-adc-window-coherence-read" \
    "clock-manager-base=" \
    "clk-sys-ctrl" \
    "clk-uart-ctrl" \
    "adc-ctrl-first" \
    "adc-ctrl-second" \
    "adc-div-int" \
    "adc-sel" \
    "-source-offset=" \
    "address=" \
    "width=32" \
    "raw=" \
    "clk-sys-enable=" \
    "clk-uart-enable=" \
    "-enable=" \
    "-auxsrc=" \
    "-source=" \
    "adc-ctrl-stable=" \
    "adc-window-all-equal=" \
    "adc-window-all-deaddead=" \
    "adc-sel-zero=" \
    "adc-sel-one-hot=" \
    "adc-sel-multi-bit=" \
    "retained-enable-toggle-pre-raw=" \
    "retained-enable-toggle-transition-raw=" \
    "retained-enable-toggle-post-raw=" \
    "retained-enable-toggle-restore-raw=" \
    "retained-enable-toggle-restore-eq-pre=true" \
    "classification=" \
    "rp1-clock-adc-window-coherent-read" \
    "rp1-clock-adc-window-readback-sentinel" \
    "rp1-clock-adc-window-unstable-readback" \
    "rp1-clock-adc-window-blocked-missing-clock-manager" \
    "rp1-clock-adc-window-blocked-uart-clock-disabled"; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing clock ADC window coherence string: $required" >&2
        exit 1
    fi
done

for forbidden in \
    "TALOS: rp1-clock-adc-window-coherence-control" \
    "no-mmio-clock-adc-window-coherence-control-visible" \
    "rpi5-rp1-clock-adc-ctrl-enable-toggle: before-rp1-clock-enable-toggle"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden clock ADC window coherence string: $forbidden" >&2
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
printf 'result_marker=%s\n' "$RESULT_MARKER"
