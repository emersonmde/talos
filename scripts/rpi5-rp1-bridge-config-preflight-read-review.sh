#!/bin/sh
set -eu

if [ "$#" -ne 1 ]; then
    echo "usage: $0 <candidate-archive.tar.gz>" >&2
    exit 2
fi

ARCHIVE="$1"
RESULT_MARKER="TALOS: rp1-bridge-config-preflight-result"

./scripts/rpi5-archive-review.sh "$ARCHIVE" >/dev/null

tmp_root="target/tmp"
mkdir -p "$tmp_root"
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-bridge-config-preflight-read-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-bridge-config-preflight-read: start" \
    "rpi5-rp1-bridge-config-preflight-read: before-status-load" \
    "rpi5-rp1-bridge-config-preflight-read: before-misc-ctrl-load" \
    "$RESULT_MARKER" \
    "phase11-rp1-bridge-config-preflight-source-contract-v1" \
    "pcie2-bridge-misc-ctrl-preflight-read" \
    "pcie2-controller-base=" \
    "status-register=" \
    "PCIE_MISC_PCIE_STATUS" \
    "status-source-offset=" \
    "status-address=" \
    "status-width=32" \
    "status-raw=" \
    "pcie-port=" \
    "dl-active=" \
    "phylinkup=" \
    "link-in-l23=" \
    "status-is-deaddead=" \
    "preflight-register=" \
    "PCIE_MISC_MISC_CTRL" \
    "preflight-source-offset=" \
    "preflight-address=" \
    "preflight-width=32" \
    "misc-ctrl-raw=" \
    "scb-access-en=" \
    "cfg-read-ur-mode=" \
    "rcb-mps-mode=" \
    "rcb-64b-mode=" \
    "max-burst-size=" \
    "misc-ctrl-is-sentinel=" \
    "retained-endpoint-config-classification=" \
    "rp1-endpoint-config-id-all-ones" \
    "classification-vocabulary=" \
    "pcie2-bridge-preflight-ready" \
    "pcie2-bridge-preflight-incomplete" \
    "pcie2-bridge-preflight-sentinel" \
    "pcie2-bridge-preflight-link-down-skip" \
    "pcie2-bridge-preflight-inconclusive-capture" \
    "no-mmio-pcie2-bridge-preflight-control-visible" \
    "classification="; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing bridge config preflight string: $required" >&2
        exit 1
    fi
done

for forbidden in \
    "TALOS: rp1-bridge-config-preflight-control" \
    "rpi5-rp1-bridge-config-preflight-control: no-bcm2712-pcie-rp1-msix-mip-gic-gpio-clock-reset-dma-mmio" \
    "EXT_CFG_INDEX" \
    "EXT_CFG_DATA" \
    "BAR" \
    "bus-master" \
    "msi-target" \
    "dma"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden bridge config preflight string: $forbidden" >&2
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
