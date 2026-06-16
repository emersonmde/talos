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
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-control-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-control: start" \
    "no-mdio-no-macb-no-gpio32-no-phy-target-construction" \
    "TALOS: rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-control" \
    "bcm54213pe-bmcr-autoneg-restart-proof-contract-id=phase12-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-proof-contract-v1" \
    "task-id=phase12-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-pi5-proof-20260616" \
    "proof-core-task-id=phase12-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-core-20260616" \
    "source-checkpoint-task-id=phase12-rp1-ethernet-bcm54213pe-link-recovery-source-checkpoint-20260616" \
    "selected-discriminator=bcm54213pe-phy1-bmcr-autoneg-restart" \
    "report-kind=" \
    "no-mdio-no-ethernet-control" \
    "hardware-proof-boundary-classification=bcm54213pe-bmcr-autoneg-restart-proof-core-local-static" \
    "target=none controller=none compatible=none" \
    "phy-model=none" \
    "physid1=withheld physid2=withheld" \
    "phy-handle=none phy-node=none phy-address=none" \
    "ncr-observed-target=not-constructed" \
    "nsr-observed-target=not-constructed" \
    "man-observed-target=not-constructed" \
    "selected-registers=withheld" \
    "bmcr-write-value=withheld bmcr-write-frame=withheld bmcr-write-count=0 touched-fields=none" \
    "bmcr-write-performed=false mdio-man-transactions-performed=false" \
    "macb-read-performed=false macb-write-performed=false" \
    "phy-reset-or-gpio32-action=false link-forcing=false" \
    "classification=no-mdio-no-ethernet-bcm54213pe-bmcr-autoneg-restart-control" \
    "claims-extra-phy-writes=false claims-selector-write-surface=false claims-gpio32-reset-action=false" \
    "claims-broadcom-shadow-mmd-aux-access=false claims-interrupt-ownership=false" \
    "claims-link-ready-acceptance=false claims-link-forcing=false" \
    "claims-packet-io=false claims-networking=false claims-sockets=false claims-ssh=false" \
    "claims-phase-12-2=false claims-phase-transition=false"; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing BCM54213PE BMCR autoneg restart control string: $required" >&2
        exit 1
    fi
done

if [ -n "$CAPTURE_NONCE" ]; then
    if ! grep -Fq -- " capture-nonce=" "$kernel_strings"; then
        echo "kernel image missing BCM54213PE BMCR control capture nonce label" >&2
        exit 1
    fi
    if ! grep -Fq -- "$CAPTURE_NONCE" "$kernel_strings"; then
        echo "kernel image missing BCM54213PE BMCR control capture nonce: $CAPTURE_NONCE" >&2
        exit 1
    fi
fi

for forbidden in \
    "TALOS: rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-candidate" \
    "before-bounded-phy1-bmcr-autoneg-restart" \
    "target=corrected-target-phy1-bmcr-autoneg-restart" \
    "phy-model=Broadcom-BCM54213PE" \
    "bmcr-write-performed=true" \
    "mdio-man-transactions-performed=true" \
    "macb-read-performed=true" \
    "macb-write-performed=true" \
    "claims-bmcr-write-executed=true" \
    "claims-exactly-one-bmcr-write=true" \
    "claims-link-ready-acceptance=true"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden BCM54213PE BMCR control string: $forbidden" >&2
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
printf 'forbidden_bcm54213pe_bmcr_autoneg_restart_control_runtime_strings_absent=true\n'
