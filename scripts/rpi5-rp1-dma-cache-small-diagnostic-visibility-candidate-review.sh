#!/bin/sh
set -eu

if [ "$#" -ne 1 ]; then
    echo "usage: $0 <candidate-archive.tar.gz>" >&2
    exit 2
fi

ARCHIVE="$1"
MARKER="TALOS: rp1-dma-cache-small-diagnostic-visibility-candidate"

./scripts/rpi5-archive-review.sh "$ARCHIVE" >/dev/null

tmp_root="target/tmp"
mkdir -p "$tmp_root"
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-dma-cache-small-diagnostic-visibility-candidate-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-dma-cache-small-diagnostic-visibility-candidate: start" \
    "no-rp1-mmio-no-dma-channel-programming-no-descriptor-ring" \
    "$MARKER" \
    "phase11-rp1-dma-cache-small-diagnostic-visibility-report-contract-v1" \
    "phase11-rp1-dma-cache-small-diagnostic-source-contract-20260609" \
    "phase11-rp1-dma-cache-small-diagnostic-plan-contract-v1" \
    "phase11-rp1-dma-cache-driver-diagnostic-envelope-contract-v1" \
    "phase11-rp1-dma-cache-maintenance-executor-contract-v1" \
    "phase11-rp1-dma-cache-maintenance-sequence-contract-v1" \
    "phase11-rp1-dma-cache-sync-plan-contract-v1" \
    "phase11-rp1-dma-cache-substrate-contract-v1" \
    "phase11-rp1-dma-cache-source-inventory-20260609" \
    "report-kind=" \
    "candidate" \
    "hardware-proof-limited-to-plan-visibility-control-output" \
    "snps,axi-dma-1.01a" \
    "RP1_INT_DMA" \
    "RP1_CLK_DMA,RP1_CLK_SYS" \
    "line-count=128" \
    "direction=" \
    "to-device" \
    "cacheable-requires-maintenance" \
    "source-unassigned-rp1-dma" \
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
    "classification=local-static-rp1-dma-small-diagnostic-plan-visibility-candidate"; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing DMA cache small diagnostic visibility candidate string: $required" >&2
        exit 1
    fi
done

for forbidden in \
    "TALOS: rp1-dma-cache-small-diagnostic-visibility-control" \
    "classification=no-plan-rp1-dma-small-diagnostic-visibility-control" \
    "before-rp1-read" \
    "before-rp1-clock-write-restore" \
    "before-rp1-clock-enable-toggle" \
    "mapped/read-value" \
    "classification=simulated/control"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden DMA cache visibility candidate string: $forbidden" >&2
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
printf 'candidate_marker=%s\n' "$MARKER"
printf 'forbidden_dma_visibility_candidate_runtime_strings_absent=true\n'
