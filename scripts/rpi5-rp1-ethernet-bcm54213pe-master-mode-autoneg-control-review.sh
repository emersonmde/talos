#!/bin/sh
set -eu

if [ "$#" -ne 1 ] && [ "$#" -ne 3 ]; then
    echo "usage: $0 <control-archive.tar.gz> [--capture-nonce NONCE]" >&2
    exit 2
fi

ARCHIVE="$1"
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
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-ethernet-bcm54213pe-master-mode-autoneg-control-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-ethernet-bcm54213pe-master-mode-autoneg-control: start" \
    "no-mdio-no-macb-no-gpio32-no-phy-target-construction" \
    "TALOS: rp1-ethernet-bcm54213pe-master-mode-autoneg-control" \
    "bcm54213pe-master-mode-autoneg-contract-id=phase12-rp1-ethernet-bcm54213pe-master-mode-autoneg-source-contract-v1" \
    "task-id=phase12-rp1-ethernet-bcm54213pe-master-mode-autoneg-pi5-proof-20260618" \
    "source-core-task-id=phase12-rp1-ethernet-bcm54213pe-master-mode-autoneg-source-contract-core-20260618" \
    "selected-discriminator=bcm54213pe-phy1-mii-ctrl1000-master-mode-plus-bmcr-autoneg-restart" \
    "report-kind=" \
    "no-mdio-no-ethernet-control" \
    "target=none controller=none compatible=none" \
    "ctrl1000-read-frame=withheld ctrl1000-write-frame=withheld" \
    "ctrl1000-write-completed=false ctrl1000-readback-completed=false" \
    "bmcr-write-frame=withheld bmcr-write-value=withheld bmcr-write-count=0 touched-fields=none" \
    "selected-registers=withheld" \
    "mdio-man-transactions-performed=false" \
    "macb-read-performed=false macb-write-performed=false" \
    "phy-reset-or-gpio32-action=false link-forcing=false" \
    "claims-runtime-mdio-transaction=" \
    "claims-ctrl1000-write-completed=" \
    "claims-ctrl1000-readback-completed=" \
    "claims-bmcr-write-executed=" \
    "classification=no-mdio-no-ethernet-bcm54213pe-master-mode-autoneg-control"; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing BCM54213PE master-mode-autoneg control string: $required" >&2
        exit 1
    fi
done

if [ -n "$CAPTURE_NONCE" ]; then
    if ! grep -Fq -- " capture-nonce=" "$kernel_strings"; then
        echo "kernel image missing BCM54213PE master-mode-autoneg control capture nonce label" >&2
        exit 1
    fi
    if ! grep -Fq -- "$CAPTURE_NONCE" "$kernel_strings"; then
        echo "kernel image missing BCM54213PE master-mode-autoneg control capture nonce: $CAPTURE_NONCE" >&2
        exit 1
    fi
fi

for forbidden in \
    "TALOS: rp1-ethernet-bcm54213pe-master-mode-autoneg-candidate" \
    "target=phy1-mii-ctrl1000-master-mode-plus-bmcr-autoneg-restart" \
    "phy-model=Broadcom-BCM54213PE" \
    "ctrl1000-read-frame=0x60a60000" \
    "ctrl1000-write-frame=0x50a61a00" \
    "bmcr-write-frame=0x50821200" \
    "macb-read-performed=true" \
    "claims-runtime-mdio-transaction=true" \
    "claims-ctrl1000-write-completed=true" \
    "claims-bmcr-write-executed=true" \
    "claims-packet-io=true" \
    "claims-networking=true" \
    "claims-ssh=true" \
    "phy-reset-or-gpio32-action=true"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden BCM54213PE master-mode-autoneg control string: $forbidden" >&2
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
printf 'forbidden_bcm54213pe_master_mode_autoneg_control_runtime_strings_absent=true\n'
