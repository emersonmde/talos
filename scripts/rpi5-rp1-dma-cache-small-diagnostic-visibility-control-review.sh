#!/bin/sh
set -eu

if [ "$#" -ne 1 ] && [ "$#" -ne 3 ]; then
    echo "usage: $0 <control-archive.tar.gz> [--capture-nonce NONCE]" >&2
    exit 2
fi

ARCHIVE="$1"
MARKER="TALOS: rp1-dma-cache-small-diagnostic-visibility-control"
CAPTURE_NONCE=

if [ "$#" -eq 3 ]; then
    if [ "$2" != "--capture-nonce" ] || [ -z "$3" ]; then
        echo "usage: $0 <control-archive.tar.gz> [--capture-nonce NONCE]" >&2
        exit 2
    fi
    CAPTURE_NONCE="$3"
fi

./scripts/rpi5-archive-review.sh "$ARCHIVE" >/dev/null

tmp_root="target/tmp"
mkdir -p "$tmp_root"
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-dma-cache-small-diagnostic-visibility-control-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-dma-cache-small-diagnostic-visibility-control: start" \
    "no-plan-no-rp1-mmio-no-dma-channel-programming-no-descriptor-ring" \
    "$MARKER" \
    "phase11-rp1-dma-cache-small-diagnostic-visibility-report-contract-v1" \
    "phase11-rp1-dma-cache-small-diagnostic-source-contract-20260609" \
    "report-kind=" \
    "no-plan-control" \
    "small-diagnostic-plan-contract-id=none" \
    "driver-diagnostic-envelope-contract-id=none" \
    "executor-contract-id=none" \
    "descriptor-contract-id=none" \
    "rp1-dma-compatible=none" \
    "cpu-physical=none" \
    "direction=none" \
    "iommu-classification=none" \
    "hardware-proof-limited-to-plan-visibility-control-output" \
    "rp1-mmio-writes" \
    "dma-channel-programming" \
    "descriptor-ring-construction" \
    "transfer-completion" \
    "interrupt-completion" \
    "ethernet-readiness" \
    "storage-readiness" \
    "networking" \
    "ssh" \
    "milestone-11-3-completion" \
    "phase-transition" \
    "claims-rp1-mmio-writes=false" \
    "claims-dma-channel-programming=false" \
    "claims-descriptor-ring-ready=false" \
    "claims-transfer-completion=false" \
    "claims-interrupt-completion=false" \
    "classification=no-plan-rp1-dma-small-diagnostic-visibility-control"; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing DMA cache small diagnostic visibility control string: $required" >&2
        exit 1
    fi
done

if [ -n "$CAPTURE_NONCE" ]; then
    if ! grep -Fq -- " capture-nonce=" "$kernel_strings"; then
        echo "kernel image missing DMA cache visibility control capture nonce label" >&2
        exit 1
    fi
    if ! grep -Fq -- "$CAPTURE_NONCE" "$kernel_strings"; then
        echo "kernel image missing DMA cache visibility control capture nonce: $CAPTURE_NONCE" >&2
        exit 1
    fi
fi

for forbidden in \
    "TALOS: rp1-dma-cache-small-diagnostic-visibility-candidate" \
    "classification=local-static-rp1-dma-small-diagnostic-plan-visibility-candidate" \
    "small-diagnostic-plan-contract-id=phase11-rp1-dma-cache-small-diagnostic-plan-contract-v1" \
    "snps,axi-dma-1.01a" \
    "RP1_INT_DMA" \
    "before-rp1-read" \
    "mapped/read-value" \
    "classification=simulated/control"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden DMA cache visibility control string: $forbidden" >&2
        exit 1
    fi
done

image_size="$(wc -c < "$extract_dir/kernel_2712.img" | tr -d ' ')"
text_offset="$(od -An -tu8 -j8 -N8 "$extract_dir/kernel_2712.img" | tr -d ' ')"
header_image_size="$(od -An -tu8 -j16 -N8 "$extract_dir/kernel_2712.img" | tr -d ' ')"
flags="$(od -An -tu8 -j24 -N8 "$extract_dir/kernel_2712.img" | tr -d ' ')"

printf 'control_archive=%s\n' "$ARCHIVE"
printf 'control_archive_sha256=%s\n' "$(sha256sum "$ARCHIVE" | awk '{print $1}')"
printf 'kernel_2712_sha256=%s\n' "$(sha256sum "$extract_dir/kernel_2712.img" | awk '{print $1}')"
printf 'kernel_2712_size=%s\n' "$image_size"
printf 'header_image_size=%s\n' "$header_image_size"
printf 'text_offset=%s\n' "$text_offset"
printf 'flags=%s\n' "$flags"
printf 'control_marker=%s\n' "$MARKER"
if [ -n "$CAPTURE_NONCE" ]; then
    printf 'capture_nonce=%s\n' "$CAPTURE_NONCE"
fi
printf 'forbidden_dma_visibility_control_runtime_strings_absent=true\n'
