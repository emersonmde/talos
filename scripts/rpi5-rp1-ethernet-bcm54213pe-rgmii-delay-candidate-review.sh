#!/bin/sh
set -eu

if [ "$#" -ne 1 ] && [ "$#" -ne 3 ]; then
    echo "usage: $0 <candidate-archive.tar.gz> [--capture-nonce NONCE]" >&2
    exit 2
fi

ARCHIVE="$1"
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
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-ethernet-bcm54213pe-rgmii-delay-candidate-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-ethernet-bcm54213pe-rgmii-delay-candidate: start" \
    "before-rgmii-id-rx-tx-delay-write-readback" \
    "TALOS: rp1-ethernet-bcm54213pe-rgmii-delay-candidate" \
    "bcm54213pe-rgmii-delay-proof-contract-id=phase12-rp1-ethernet-bcm54213pe-rgmii-delay-proof-contract-v1" \
    "task-id=phase12-rp1-ethernet-bcm54213pe-rgmii-delay-pi5-proof-20260616" \
    "proof-core-task-id=phase12-rp1-ethernet-bcm54213pe-rgmii-delay-proof-core-20260616" \
    "source-contract-task-id=phase12-rp1-ethernet-bcm54213pe-rgmii-delay-source-contract-20260616" \
    "source-contract-commit=817712f6837a7e3ca659cea1833875c22e04f588" \
    "selected-discriminator=bcm54213pe-phy1-rgmii-id-rx-tx-delay-write-readback" \
    "report-kind=" \
    "candidate" \
    "hardware-proof-boundary-classification=bcm54213pe-rgmii-delay-proof-core-local-static" \
    "target=phy1-rgmii-id-rx-tx-delay-write-readback" \
    "phy-model=Broadcom-BCM54213PE" \
    "phy-handle=phy1 phy-node=ethernet-phy@1 phy-address=1" \
    "rx-selector-write-value=0x7007 rx-read-frame=0x60e20000 rx-write-frame-prefix=0x50e20000" \
    "rx-readback-rgmii-skew-en=" \
    "tx-selector-write-value=0x0c00 tx-read-frame=0x60f20000 tx-write-frame-prefix=0x50f20000" \
    "tx-readback-gtxclk-en=" \
    "rgmii-delay-write-count=" \
    "bmcr-write-frame=0x50821200 bmcr-write-count=" \
    "poll-count-bound=" \
    "poll-delay-spins=" \
    "selected-registers=rx-delay-MII_BCM54XX_AUX_CTL,tx-delay-MII_BCM54XX_SHD,restart-BMCR,poll-BMCR,poll-BMSR-first,poll-BMSR-second,poll-ANAR,poll-ANLPAR,poll-MII_CTRL1000,poll-MII_STAT1000,poll-passive-MACB_NSR_LINK" \
    "link-ready-terminal=" \
    "bmcr-write-performed=" \
    "mdio-man-transactions-performed=" \
    "macb-read-performed=true macb-write-performed=false" \
    "phy-reset-or-gpio32-action=false" \
    "allowed-hardware-classifications=rgmii-delay-link-ready-frontier,rgmii-delay-timeout-link-not-ready,rgmii-delay-readback-mismatch,rgmii-delay-precondition-blocker,rgmii-delay-capture-blocker,no-mdio-no-ethernet-bcm54213pe-rgmii-delay-control" \
    "rejected-runtime-hardware-claims=target-drift,mii-ctrl1000-master-mode-writes,extra-phy-writes,uncontracted-selector-config-access,gpio32-reset-action,interrupt-ownership,phy-mac-configuration,link-ready-acceptance,packet-io,networking,sockets,ssh,phase-12-2,phase-transition" \
    "claims-rgmii-delay-write-count=" \
    "claims-mii-ctrl1000-master-mode-write=false" \
    "claims-extra-phy-writes=false claims-uncontracted-selector-config-access=false"; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing BCM54213PE RGMII delay candidate string: $required" >&2
        exit 1
    fi
done

if [ -n "$CAPTURE_NONCE" ]; then
    if ! grep -Fq -- " capture-nonce=" "$kernel_strings"; then
        echo "kernel image missing BCM54213PE RGMII delay candidate capture nonce label" >&2
        exit 1
    fi
    if ! grep -Fq -- "$CAPTURE_NONCE" "$kernel_strings"; then
        echo "kernel image missing BCM54213PE RGMII delay candidate capture nonce: $CAPTURE_NONCE" >&2
        exit 1
    fi
fi

for forbidden in \
    "TALOS: rp1-ethernet-bcm54213pe-rgmii-delay-control" \
    "no-mdio-no-macb-no-gpio32-no-phy-target-construction" \
    "target=none controller=none compatible=none" \
    "classification=no-mdio-no-ethernet-bcm54213pe-rgmii-delay-control" \
    "macb-write-performed=true" \
    "phy-reset-or-gpio32-action=true" \
    "claims-link-ready-acceptance=true" \
    "claims-mii-ctrl1000-master-mode-write=true" \
    "uncontracted-selector-config-access=true" \
    "gpio32-event-clear" \
    "gpio32-phy-reset"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden BCM54213PE RGMII delay candidate string: $forbidden" >&2
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
printf 'forbidden_bcm54213pe_rgmii_delay_candidate_runtime_strings_absent=true\n'
