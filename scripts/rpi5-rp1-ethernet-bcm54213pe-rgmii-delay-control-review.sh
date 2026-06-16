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
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-ethernet-bcm54213pe-rgmii-delay-control-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-ethernet-bcm54213pe-rgmii-delay-control: start" \
    "no-mdio-no-macb-no-gpio32-no-phy-target-construction" \
    "TALOS: rp1-ethernet-bcm54213pe-rgmii-delay-control" \
    "bcm54213pe-rgmii-delay-proof-contract-id=phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-proof-contract-v1" \
    "task-id=phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-pi5-proof-20260616" \
    "proof-core-task-id=phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-proof-core-20260616" \
    "source-correction-task-id=phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-source-correction-20260616" \
    "source-correction-commit=0b947e8e9bc2025b2072490266479b490f34327e" \
    "selected-discriminator=bcm54213pe-phy1-rgmii-id-rx-then-tx-delay-stage-accounting" \
    "report-kind=" \
    "no-mdio-no-ethernet-control" \
    "hardware-proof-boundary-classification=bcm54213pe-rgmii-delay-tx-order-proof-core-local-static" \
    "target=none controller=none compatible=none" \
    "phy-model=none physid1=withheld physid2=withheld" \
    "phy-handle=none phy-node=none phy-address=none" \
    "rx-selector-write-value=withheld rx-read-frame=withheld rx-write-frame-prefix=withheld" \
    "rx-readback-rgmii-skew-en=false" \
    "rx-selector-write-completed=false rx-selected-read-completed=false" \
    "rx-delay-write-completed=false rx-readback-completed=false" \
    "tx-selector-write-value=withheld tx-read-frame=withheld tx-write-frame-prefix=withheld" \
    "tx-readback-gtxclk-en=false" \
    "tx-selector-write-completed=false tx-selected-read-completed=false" \
    "tx-delay-write-completed=false tx-delay-write-skipped-already-enabled=false" \
    "tx-readback-completed=false" \
    "rgmii-delay-write-count=0x0 bmcr-write-frame=withheld bmcr-write-count=0x0" \
    "selected-registers=withheld" \
    "bmcr-write-performed=false mdio-man-transactions-performed=false" \
    "macb-read-performed=false macb-write-performed=false" \
    "phy-reset-or-gpio32-action=false" \
    "allowed-hardware-classifications=rgmii-delay-tx-order-link-ready-frontier,rgmii-delay-tx-order-timeout-link-not-ready,rgmii-delay-tx-order-rx-stage-blocker,rgmii-delay-tx-order-tx-selected-read-visible,rgmii-delay-tx-order-tx-stage-blocker,rgmii-delay-tx-order-readback-mismatch,rgmii-delay-tx-order-precondition-blocker,rgmii-delay-tx-order-capture-blocker,no-mdio-no-ethernet-bcm54213pe-rgmii-delay-tx-order-control" \
    "claims-runtime-mdio-transaction=" \
    "claims-rgmii-delay-write-count=" \
    "claims-bmcr-write-executed=" \
    "claims-mii-ctrl1000-master-mode-write=false" \
    "claims-extra-phy-writes=false claims-uncontracted-selector-config-access=false" \
    "classification=no-mdio-no-ethernet-bcm54213pe-rgmii-delay-tx-order-control"; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing BCM54213PE RGMII delay control string: $required" >&2
        exit 1
    fi
done

if [ -n "$CAPTURE_NONCE" ]; then
    if ! grep -Fq -- " capture-nonce=" "$kernel_strings"; then
        echo "kernel image missing BCM54213PE RGMII delay control capture nonce label" >&2
        exit 1
    fi
    if ! grep -Fq -- "$CAPTURE_NONCE" "$kernel_strings"; then
        echo "kernel image missing BCM54213PE RGMII delay control capture nonce: $CAPTURE_NONCE" >&2
        exit 1
    fi
fi

for forbidden in \
    "TALOS: rp1-ethernet-bcm54213pe-rgmii-delay-candidate" \
    "target=phy1-rgmii-id-rx-tx-delay-write-readback" \
    "phy-model=Broadcom-BCM54213PE" \
    "rx-readback-rgmii-skew-en=true" \
    "tx-readback-gtxclk-en=true" \
    "macb-read-performed=true" \
    "macb-write-performed=true" \
    "phy-reset-or-gpio32-action=true" \
    "claims-runtime-mdio-transaction=true" \
    "claims-mii-ctrl1000-master-mode-write=true" \
    "gpio32-event-clear" \
    "gpio32-phy-reset"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden BCM54213PE RGMII delay control string: $forbidden" >&2
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
printf 'forbidden_bcm54213pe_rgmii_delay_control_runtime_strings_absent=true\n'
