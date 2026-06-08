#!/bin/sh
set -eu

if [ "$#" -ne 1 ] && [ "$#" -ne 3 ]; then
    echo "usage: $0 <candidate-archive.tar.gz> [--capture-nonce NONCE]" >&2
    exit 2
fi

ARCHIVE="$1"
RESULT_MARKER="TALOS: rp1-observed-gpio-status-result"
CAPTURE_NONCE=

if [ "$#" -eq 3 ]; then
    if [ "$2" != "--capture-nonce" ] || [ -z "$3" ]; then
        echo "usage: $0 <candidate-archive.tar.gz> [--capture-nonce NONCE]" >&2
        exit 2
    fi
    CAPTURE_NONCE="$3"
fi

./scripts/rpi5-archive-review.sh "$ARCHIVE" >/dev/null

tmp_root="target/tmp"
mkdir -p "$tmp_root"
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-observed-gpio-status-read-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-observed-gpio-status-read: start contract=" \
    "rpi5-rp1-observed-gpio-status-read: before-gpio14-status-ctrl-loads addresses=" \
    "$RESULT_MARKER" \
    "phase11-rp1-observed-gpio-status-source-contract-v1" \
    "rp1-gpio14-status-ctrl-observed-aperture-read" \
    "status-source-rp1-bus-address=" \
    "ctrl-source-rp1-bus-address=" \
    "0xc0400d0070" \
    "0xc0400d0074" \
    "status-observed-cpu-physical-address=" \
    "ctrl-observed-cpu-physical-address=" \
    "0x1c000d0070" \
    "0x1c000d0074" \
    "status-register-offset=" \
    "ctrl-register-offset=" \
    "0x70" \
    "0x74" \
    "width=32" \
    "gpio14-status-raw=" \
    "gpio14-ctrl-raw=" \
    "status-raw-falling=" \
    "status-raw-rising=" \
    "status-raw-low=" \
    "status-raw-high=" \
    "status-filtered-falling=" \
    "status-filtered-rising=" \
    "status-filtered-low=" \
    "status-filtered-high=" \
    "ctrl-funcsel=" \
    "ctrl-outover=" \
    "ctrl-oeover=" \
    "ctrl-inover=" \
    "ctrl-irqover=" \
    "ctrl-raw-falling-enabled=" \
    "ctrl-raw-rising-enabled=" \
    "ctrl-raw-low-enabled=" \
    "ctrl-raw-high-enabled=" \
    "ctrl-filtered-falling-enabled=" \
    "ctrl-filtered-rising-enabled=" \
    "ctrl-filtered-low-enabled=" \
    "ctrl-filtered-high-enabled=" \
    "status-raw-is-deaddead=" \
    "status-raw-is-all-ones=" \
    "status-raw-is-zero=" \
    "ctrl-raw-is-deaddead=" \
    "ctrl-raw-is-all-ones=" \
    "ctrl-raw-is-zero=" \
    "retained-observed-uart0-fr-raw=0x187" \
    "retained-observed-uart0-fr-pl011-fr-shaped=true" \
    "classification-vocabulary=" \
    "observed-aperture-gpio14-status-ctrl-visible" \
    "observed-aperture-gpio14-status-ctrl-sentinel" \
    "observed-aperture-gpio14-status-ctrl-all-ones" \
    "observed-aperture-gpio14-status-ctrl-zero" \
    "observed-aperture-gpio14-status-ctrl-no-return-or-trap" \
    "observed-aperture-gpio14-status-ctrl-inconclusive-capture" \
    "no-mmio-observed-gpio-status-control-visible" \
    "staging/build-blocker" \
    "classification="; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing observed GPIO status string: $required" >&2
        exit 1
    fi
done

if [ -n "$CAPTURE_NONCE" ]; then
    if ! grep -Fq -- " capture-nonce=" "$kernel_strings"; then
        echo "kernel image missing observed GPIO status result capture nonce label" >&2
        exit 1
    fi
    if ! grep -Fq -- "$CAPTURE_NONCE" "$kernel_strings"; then
        echo "kernel image missing observed GPIO status result capture nonce: $CAPTURE_NONCE" >&2
        exit 1
    fi
fi

for forbidden in \
    "TALOS: rp1-observed-gpio-status-control" \
    "rpi5-rp1-observed-gpio-status-control: no-bcm2712-pcie-rp1-mip-gic-gpio-rio-pads-clock-reset-dma-mmio" \
    "0x1f000d0070" \
    "0x1f000d0074" \
    "0x1c000d011c" \
    "0x1c000d0124" \
    "0x1f000d011c" \
    "0x1f000d0124" \
    "0x1000120000" \
    "EXT_CFG_INDEX" \
    "EXT_CFG_DATA" \
    "BAR" \
    "bus-master" \
    "dma"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden observed GPIO status string: $forbidden" >&2
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
if [ -n "$CAPTURE_NONCE" ]; then
    printf 'capture_nonce=%s\n' "$CAPTURE_NONCE"
fi
