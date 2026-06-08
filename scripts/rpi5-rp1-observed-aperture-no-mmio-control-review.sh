#!/bin/sh
set -eu

if [ "$#" -ne 1 ]; then
    echo "usage: $0 <candidate-archive.tar.gz>" >&2
    exit 2
fi

ARCHIVE="$1"
CONTROL_MARKER="TALOS: rp1-observed-aperture-control"

./scripts/rpi5-archive-review.sh "$ARCHIVE" >/dev/null

tmp_root="target/tmp"
mkdir -p "$tmp_root"
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-observed-aperture-control-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-observed-aperture-control: start contract=" \
    "rpi5-rp1-observed-aperture-control: no-bcm2712-pcie-rp1-mip-gic-gpio-clock-reset-dma-mmio" \
    "$CONTROL_MARKER" \
    "phase11-rp1-observed-aperture-source-contract-v1" \
    "rp1-uart0-fr-observed-aperture-read" \
    "source-rp1-bus-address=" \
    "observed-cpu-physical-address=" \
    "register-offset=" \
    "not-constructed" \
    "width=32" \
    "raw=" \
    "raw-is-deaddead=" \
    "raw-is-all-ones=" \
    "raw-is-zero=" \
    "raw-is-pl011-fr-shaped=" \
    "retained-bridge-win0-lo=0x80000000" \
    "retained-bridge-win0-base-limit=0x3ff00000" \
    "retained-bridge-win0-base-hi=0x1c" \
    "retained-bridge-win0-limit-hi=0x1c" \
    "retained-bridge-outbound-window0-matches=false" \
    "classification-vocabulary=" \
    "observed-aperture-rp1-uart0-fr-visible" \
    "observed-aperture-rp1-uart0-fr-sentinel" \
    "observed-aperture-rp1-uart0-fr-all-ones" \
    "observed-aperture-rp1-uart0-fr-zero" \
    "observed-aperture-rp1-uart0-fr-no-return-or-trap" \
    "observed-aperture-rp1-uart0-fr-inconclusive-capture" \
    "classification=no-mmio-observed-aperture-control-visible"; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing observed-aperture control string: $required" >&2
        exit 1
    fi
done

for forbidden in \
    "TALOS: rp1-observed-aperture-result" \
    "rpi5-rp1-observed-aperture-read: before-rp1-fr-load" \
    "0xc040030018" \
    "0x1c00030018" \
    "0x1f00030018" \
    "0x1f00000000" \
    "0x1f00018144" \
    "0x1f000d0070" \
    "0x1f000d0080" \
    "0x1f00108008" \
    "0x1000120000" \
    "0x1000124008" \
    "0x1000124068" \
    "EXT_CFG_INDEX" \
    "EXT_CFG_DATA"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden observed-aperture control string: $forbidden" >&2
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
