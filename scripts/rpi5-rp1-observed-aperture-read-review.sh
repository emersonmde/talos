#!/bin/sh
set -eu

if [ "$#" -ne 1 ]; then
    echo "usage: $0 <candidate-archive.tar.gz>" >&2
    exit 2
fi

ARCHIVE="$1"
RESULT_MARKER="TALOS: rp1-observed-aperture-result"

./scripts/rpi5-archive-review.sh "$ARCHIVE" >/dev/null

tmp_root="target/tmp"
mkdir -p "$tmp_root"
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-observed-aperture-read-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-observed-aperture-read: start contract=" \
    "rpi5-rp1-observed-aperture-read: before-rp1-fr-load" \
    "$RESULT_MARKER" \
    "phase11-rp1-observed-aperture-source-contract-v1" \
    "rp1-uart0-fr-observed-aperture-read" \
    "source-rp1-bus-address=" \
    "0xc040030018" \
    "observed-cpu-physical-address=" \
    "0x1c00030018" \
    "register-offset=" \
    "0x18" \
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
    "no-mmio-observed-aperture-control-visible" \
    "staging/build-blocker" \
    "classification="; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing observed-aperture string: $required" >&2
        exit 1
    fi
done

for forbidden in \
    "TALOS: rp1-observed-aperture-control" \
    "rpi5-rp1-observed-aperture-control: no-bcm2712-pcie-rp1-mip-gic-gpio-clock-reset-dma-mmio" \
    "0x1f00030018" \
    "0x1000120000" \
    "EXT_CFG_INDEX" \
    "EXT_CFG_DATA" \
    "BAR" \
    "bus-master" \
    "msi-target" \
    "dma"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden observed-aperture string: $forbidden" >&2
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
