#!/bin/sh
set -eu

if [ "$#" -ne 1 ] && [ "$#" -ne 3 ]; then
    echo "usage: $0 <candidate-archive.tar.gz> [--capture-nonce NONCE]" >&2
    exit 2
fi

ARCHIVE="$1"
CONTROL_MARKER="TALOS: rp1-clock-reset-dependency-control"
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
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-clock-reset-dependency-control-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-clock-reset-dependency-control: start" \
    "rpi5-rp1-clock-reset-dependency-control: no-rp1-gpio-clock-reset-msix-pcie-mip-gic-dma-mmio" \
    "$CONTROL_MARKER" \
    "phase11-rp1-clock-reset-dependency-source-contract-v1" \
    "rp1-observed-clock-reset-dependency-preflight-read" \
    "observed-base=" \
    "sysinfo-base=" \
    "clock-manager-base=" \
    "not-constructed" \
    "sysinfo-chip-id" \
    "sysinfo-platform" \
    "pll-sys-cs" \
    "clk-sys-ctrl" \
    "clk-sys-div-int" \
    "clk-sys-sel" \
    "clk-slow-sys-ctrl" \
    "clk-uart-ctrl" \
    "clk-uart-div-int" \
    "clk-uart-sel" \
    "-source-offset=" \
    "address=not-constructed" \
    "width=32" \
    "raw=" \
    "expected-chip-id=" \
    "chip-id-matches-expected=" \
    "chip-id-is-deaddead=" \
    "platform-is-deaddead=" \
    "pll-sys-locked=" \
    "clk-sys-enabled=" \
    "clk-slow-sys-enabled=" \
    "clk-uart-enabled=" \
    "any-selected-clock-deaddead=" \
    "all-selected-clock-deaddead=" \
    "reset-status-source=none-selected-read-only" \
    "classification-vocabulary=" \
    "no-mmio-clock-reset-dependency-control-visible" \
    "classification=no-mmio-clock-reset-dependency-control-visible"; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing clock/reset dependency no-MMIO control string: $required" >&2
        exit 1
    fi
done

if [ -n "$CAPTURE_NONCE" ]; then
    if ! grep -Fq -- " capture-nonce=" "$kernel_strings"; then
        echo "kernel image missing clock/reset dependency control capture nonce label" >&2
        exit 1
    fi
    if ! grep -Fq -- "$CAPTURE_NONCE" "$kernel_strings"; then
        echo "kernel image missing clock/reset dependency control capture nonce: $CAPTURE_NONCE" >&2
        exit 1
    fi
fi

for forbidden in \
    "TALOS: rp1-clock-reset-dependency-result" \
    "rpi5-rp1-clock-reset-dependency-read: before-read-only-loads" \
    "classification=simulated/control" \
    "0x1c00000000" \
    "0x1c00000004" \
    "0x1c00020000" \
    "0x1c00018014" \
    "0x1c00018018" \
    "0x1c00018020" \
    "0x1c00018024" \
    "0x1c00018054" \
    "0x1c00018058" \
    "0x1c00018060" \
    "0x1f00000000" \
    "0x1f00020000" \
    "0x1f00018014" \
    "rpi5-rp1-gpio16-owned-event-discriminator: before-read-only-observed-aperture-loads" \
    "rpi5-rp1-clock-adc-ctrl-write-restore: before-rp1-clock-write-restore" \
    "rpi5-rp1-clock-adc-ctrl-enable-toggle: before-rp1-clock-enable-toggle"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden clock/reset dependency no-MMIO control string: $forbidden" >&2
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
printf 'forbidden_clock_reset_dependency_mmio_strings_absent=true\n'
