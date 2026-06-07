#!/bin/sh
set -eu

if [ "$#" -ne 1 ]; then
    echo "usage: $0 <candidate-archive.tar.gz>" >&2
    exit 2
fi

ARCHIVE="$1"
RESULT_MARKER="TALOS: rp1-gpio14-ownership-route-preflight-result"

./scripts/rpi5-archive-review.sh "$ARCHIVE" >/dev/null

tmp_root="target/tmp"
mkdir -p "$tmp_root"
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-gpio14-ownership-route-preflight-read-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-gpio14-ownership-route-preflight-read: start" \
    "rpi5-rp1-gpio14-ownership-route-preflight-read: before-read-only-loads" \
    "$RESULT_MARKER" \
    "phase11-rp1-gpio-ownership-restore-source-contract-v1" \
    "target=" \
    "rp1-gpio14-ownership-route-preflight-read" \
    "pin=GPIO14" \
    "gpio14-bit-mask=" \
    "gpio14-status-address=" \
    "gpio14-ctrl-address=" \
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
    "width=32" \
    "gpio14-status-raw=" \
    "gpio14-ctrl-raw=" \
    "gpio14-funcsel=" \
    "gpio14-func-name=" \
    "gpio14-outover=" \
    "gpio14-oeover=" \
    "gpio14-inover=" \
    "gpio14-raw-event-enable-mask=" \
    "gpio14-filtered-event-enable-mask=" \
    "gpio14-status-event-mask=" \
    "io-bank0-inte-raw=" \
    "io-bank0-ints-raw=" \
    "rio-out-raw=" \
    "rio-oe-raw=" \
    "rio-in-raw=" \
    "pad-raw=" \
    "pad-input-enable=" \
    "pad-output-disable=" \
    "gicd-isenabler5-raw=" \
    "gicd-ispendr5-raw=" \
    "gicd-isactiver5-raw=" \
    "gicc-hppir-raw=" \
    "intid160-enabled=" \
    "intid160-pending=" \
    "intid160-active=" \
    "hppir-intid=" \
    "classification="; do
    if ! grep -Fq "$required" "$kernel_strings"; then
        echo "kernel image missing GPIO14 ownership preflight diagnostic string: $required" >&2
        exit 1
    fi
done

for forbidden in \
    "TALOS: rp1-gpio14-ownership-route-preflight-control" \
    "classification=simulated/control" \
    "GICC_IAR" \
    "GICC_EOIR"; do
    if grep -Fq "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden GPIO14 ownership preflight diagnostic string: $forbidden" >&2
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
