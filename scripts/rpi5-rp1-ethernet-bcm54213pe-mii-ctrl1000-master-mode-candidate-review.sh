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
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-ethernet-bcm54213pe-mii-ctrl1000-master-mode-candidate-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-ethernet-bcm54213pe-mii-ctrl1000-master-mode-candidate: start" \
    "before-ctrl1000-read-modify-write-readback" \
    "TALOS: rp1-ethernet-bcm54213pe-mii-ctrl1000-master-mode-candidate" \
    "bcm54213pe-mii-ctrl1000-master-mode-contract-id=phase12-rp1-ethernet-bcm54213pe-mii-ctrl1000-master-mode-source-contract-v1" \
    "task-id=phase12-rp1-ethernet-bcm54213pe-selected-link-not-ready-pi5-proof-20260618" \
    "source-core-task-id=phase12-rp1-ethernet-bcm54213pe-selected-link-not-ready-source-contract-core-20260618" \
    "source-core-commit=1f78a3cd68590cccf32b4848c1b9dbe60991d7a7" \
    "selection-task-id=phase12-rp1-ethernet-bcm54213pe-link-not-ready-discriminator-selection-20260618" \
    "selection-commit=a18e51bf4b44680ff9071b01c238f12d1c37872c" \
    "selected-discriminator=bcm54213pe-phy1-mii-ctrl1000-master-mode-gate-source-contract" \
    "report-kind=" \
    "candidate" \
    "hardware-proof-boundary-classification=bcm54213pe-mii-ctrl1000-master-mode-pi5-proof-runtime" \
    "target=phy1-mii-ctrl1000-master-mode-read-modify-write" \
    "phy-model=Broadcom-BCM54213PE physid1=0x600d physid2=0x84a2" \
    "phy-handle=phy1 phy-node=ethernet-phy@1 phy-address=1" \
    "ctrl1000-read-frame=0x60a60000" \
    "ctrl1000-write-frame-prefix=0x50a60000" \
    "ctrl1000-write-mask=0x1800 accepted-pre-ctrl1000=0x0200" \
    "expected-write-value=0x1a00 expected-write-frame=0x50a61a00" \
    "ctrl1000-pre-raw=" \
    "ctrl1000-write-value=" \
    "ctrl1000-readback-raw=" \
    "ctrl1000-pre-read-completed=" \
    "ctrl1000-write-completed=" \
    "ctrl1000-readback-completed=" \
    "selected-registers=MII_CTRL1000:read-modify-write-readback" \
    "stage-boundaries=pre-mdio-marker,ncr-mpe-precondition,ctrl1000-pre-read,ctrl1000-write,ctrl1000-readback,stop-before-link-ready-or-packet-work" \
    "mdio-man-transactions-performed=" \
    "macb-read-performed=true macb-write-performed=false" \
    "phy-reset-or-gpio32-action=false" \
    "allowed-hardware-classifications=mii-ctrl1000-master-mode-write-readback-visible,mii-ctrl1000-master-mode-precondition-blocker,mii-ctrl1000-master-mode-readback-mismatch,mii-ctrl1000-master-mode-capture-blocker,no-mdio-no-ethernet-bcm54213pe-mii-ctrl1000-master-mode-control" \
    "rejected-runtime-hardware-claims=same-shaped-status-restart-poll-retry,bmcr-restart,rgmii-delay-retry,extra-phy-writes,gpio32-reset-action,interrupt-ownership,apd-eee-lifecycle,mac-phylink-configuration,link-ready-acceptance,autoneg-complete-acceptance,packet-io,networking,sockets,ssh,phase-12-2,phase-transition" \
    "claims-ctrl1000-write-completed=" \
    "claims-link-ready-acceptance=false claims-autoneg-complete-acceptance=false" \
    "classification="; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing BCM54213PE MII_CTRL1000 master-mode candidate string: $required" >&2
        exit 1
    fi
done

if [ -n "$CAPTURE_NONCE" ]; then
    if ! grep -Fq -- " capture-nonce=" "$kernel_strings"; then
        echo "kernel image missing BCM54213PE MII_CTRL1000 master-mode candidate capture nonce label" >&2
        exit 1
    fi
    if ! grep -Fq -- "$CAPTURE_NONCE" "$kernel_strings"; then
        echo "kernel image missing BCM54213PE MII_CTRL1000 master-mode candidate capture nonce: $CAPTURE_NONCE" >&2
        exit 1
    fi
fi

for forbidden in \
    "TALOS: rp1-ethernet-bcm54213pe-mii-ctrl1000-master-mode-control" \
    "target=none controller=none compatible=none" \
    "classification=no-mdio-no-ethernet-bcm54213pe-mii-ctrl1000-master-mode-control" \
    "claims-link-ready-acceptance=true" \
    "claims-autoneg-complete-acceptance=true" \
    "claims-packet-io=true" \
    "claims-networking=true" \
    "claims-ssh=true" \
    "phy-reset-or-gpio32-action=true" \
    "gpio32-event-clear" \
    "gpio32-phy-reset"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden BCM54213PE MII_CTRL1000 master-mode candidate string: $forbidden" >&2
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
printf 'forbidden_bcm54213pe_mii_ctrl1000_master_mode_candidate_runtime_strings_absent=true\n'
