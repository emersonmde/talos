#!/bin/sh
set -eu

if [ "$#" -ne 1 ]; then
    echo "usage: $0 <candidate-archive.tar.gz>" >&2
    exit 2
fi

ARCHIVE="$1"
RESULT_MARKER="TALOS: rp1-gpio16-owned-event-discriminator-result"

./scripts/rpi5-archive-review.sh "$ARCHIVE" >/dev/null

tmp_root="target/tmp"
mkdir -p "$tmp_root"
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-gpio16-owned-event-discriminator-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-gpio16-owned-event-discriminator: start" \
    "rpi5-rp1-gpio16-owned-event-discriminator: before-preflight-loads" \
    "$RESULT_MARKER" \
    "phase11-rp1-gpio-owned-event-discriminator-source-contract-v1" \
    "rp1-gpio16-owned-level-high-event-discriminator" \
    "pin=GPIO16" \
    "gpio16-bit-mask=" \
    "gpio16-status-address=" \
    "gpio16-ctrl-address=" \
    "io-bank0-inte-address=" \
    "io-bank0-ints-address=" \
    "rio-out-address=" \
    "rio-oe-address=" \
    "rio-in-address=" \
    "pad-address=" \
    "gicd-isenabler5-address=" \
    "gicd-ispendr5-address=" \
    "gicd-isactiver5-address=" \
    "gicc-hppir-address=" \
    "-gpio16-status-raw=" \
    " pre" \
    " post" \
    " restore" \
    "action-io-bank0-inte-clear=" \
    "action-level-high-enable=" \
    "restore-attempted=" \
    "classification="; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing GPIO16 event discriminator string: $required" >&2
        exit 1
    fi
done

for forbidden in \
    "TALOS: rp1-gpio16-owned-event-discriminator-control" \
    "classification=simulated/control" \
    "GPIO14" \
    "GPIO15" \
    "GICC_IAR" \
    "GICC_EOIR"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden GPIO16 event discriminator string: $forbidden" >&2
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
printf 'forbidden_gpio16_control_strings_absent=true\n'
