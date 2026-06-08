#!/bin/sh
set -eu

if [ "$#" -ne 1 ]; then
    echo "usage: $0 <candidate-archive.tar.gz>" >&2
    exit 2
fi

ARCHIVE="$1"
CONTROL_MARKER="TALOS: rp1-pcie2-host-link-status-control"

./scripts/rpi5-archive-review.sh "$ARCHIVE" >/dev/null

tmp_root="target/tmp"
mkdir -p "$tmp_root"
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-pcie2-host-link-status-control-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-pcie2-host-link-status-control: start" \
    "rpi5-rp1-pcie2-host-link-status-control: no-bcm2712-pcie-rp1-msix-mip-gic-gpio-clock-reset-dma-mmio" \
    "$CONTROL_MARKER" \
    "phase11-rp1-pcie-endpoint-config-discriminator-source-contract-v1" \
    "pcie2-host-link-status-read" \
    "pcie2-controller-base=not-constructed" \
    "PCIE_MISC_PCIE_STATUS" \
    "source-offset=" \
    "address=not-constructed" \
    "width=32" \
    "raw=" \
    "pcie-port=" \
    "dl-active=" \
    "phylinkup=" \
    "link-in-l23=" \
    "status-is-deaddead=" \
    "retained-sysinfo-clock-sentinel-classification=" \
    "rp1-sysinfo-and-clock-window-sentinel" \
    "retained-rp1-window-sentinel=true" \
    "classification=no-mmio-pcie2-host-link-status-control-visible"; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing PCIe2 host-link status control string: $required" >&2
        exit 1
    fi
done

for forbidden in \
    "TALOS: rp1-pcie2-host-link-status-result" \
    "rpi5-rp1-pcie2-host-link-status-read: before-read-only-load" \
    "pcie2-host-link-up-rp1-window-sentinel" \
    "pcie2-host-status-visible-link-down" \
    "pcie2-host-status-sentinel" \
    "0x1000120000" \
    "0x1000124068" \
    "0x1f00000000" \
    "0x1f00018144" \
    "0x1f000d0070" \
    "0x1f00108008" \
    "0x107fff9000" \
    "0x107fffa000"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden PCIe2 host-link status control string: $forbidden" >&2
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
