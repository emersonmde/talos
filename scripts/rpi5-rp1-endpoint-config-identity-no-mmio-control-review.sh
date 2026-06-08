#!/bin/sh
set -eu

if [ "$#" -ne 1 ]; then
    echo "usage: $0 <candidate-archive.tar.gz>" >&2
    exit 2
fi

ARCHIVE="$1"
CONTROL_MARKER="TALOS: rp1-endpoint-config-identity-control"

./scripts/rpi5-archive-review.sh "$ARCHIVE" >/dev/null

tmp_root="target/tmp"
mkdir -p "$tmp_root"
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-endpoint-config-identity-control-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-endpoint-config-identity-control: start" \
    "rpi5-rp1-endpoint-config-identity-control: no-bcm2712-pcie-rp1-sysinfo-clock-gpio-msix-mip-gic-dma-mmio" \
    "$CONTROL_MARKER" \
    "phase11-rp1-endpoint-config-identity-source-contract-v1" \
    "rp1-endpoint-config-vendor-device-read" \
    "not-constructed" \
    "pcie2-controller-base=" \
    "pci-domain=2" \
    "precondition-register=" \
    "PCIE_MISC_PCIE_STATUS" \
    "precondition-source-offset=" \
    "precondition-address=" \
    "precondition-width=32" \
    "precondition-raw=" \
    "dl-active=" \
    "phylinkup=" \
    "status-is-deaddead=" \
    "config-bdf=" \
    "0002:01:00.0" \
    "config-offset=" \
    "index-register=" \
    "EXT_CFG_INDEX" \
    "index-source-offset=" \
    "index-address=" \
    "index-value=" \
    "index-write-performed=" \
    "data-register=" \
    "EXT_CFG_DATA" \
    "data-source-offset=" \
    "data-address=" \
    "width=32" \
    "raw-config=" \
    "vendor-id=" \
    "device-id=" \
    "expected-vendor-id=" \
    "expected-device-id=" \
    "vendor-device-match=" \
    "raw-config-is-all-ones=" \
    "raw-config-is-zero=" \
    "raw-config-is-deaddead=" \
    "classification=no-mmio-rp1-endpoint-config-id-control-visible"; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing endpoint config identity control string: $required" >&2
        exit 1
    fi
done

for forbidden in \
    "TALOS: rp1-endpoint-config-identity-result" \
    "rpi5-rp1-endpoint-config-identity-read: before-precondition-load" \
    "rpi5-rp1-endpoint-config-identity-read: before-ext-cfg-index-write" \
    "rpi5-rp1-endpoint-config-identity-read: before-ext-cfg-data-load" \
    "rp1-endpoint-config-id-visible" \
    "rp1-endpoint-config-id-unexpected" \
    "rp1-endpoint-config-id-all-ones" \
    "rp1-endpoint-config-id-zero" \
    "rp1-endpoint-config-id-sentinel" \
    "rp1-endpoint-config-link-down-skip" \
    "rp1-endpoint-config-id-inconclusive-capture" \
    "0x1000120000" \
    "0x1000124068" \
    "0x1000128000" \
    "0x1000129000" \
    "0x1f00000000" \
    "0x1f00018144" \
    "0x1f000d0070" \
    "0x1f00108008" \
    "0x107fff9000" \
    "0x107fffa000"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden endpoint config identity control string: $forbidden" >&2
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
