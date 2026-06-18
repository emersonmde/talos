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
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-ethernet-bcm54213pe-master-mode-autoneg-candidate-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-ethernet-bcm54213pe-master-mode-autoneg-candidate: start" \
    "before-ctrl1000-master-mode-readback-then-bmcr-autoneg-restart" \
    "TALOS: rp1-ethernet-bcm54213pe-master-mode-autoneg-candidate" \
    "bcm54213pe-master-mode-autoneg-contract-id=phase12-rp1-ethernet-bcm54213pe-master-mode-autoneg-source-contract-v1" \
    "task-id=phase12-rp1-ethernet-bcm54213pe-master-mode-autoneg-pi5-proof-20260618" \
    "source-core-task-id=phase12-rp1-ethernet-bcm54213pe-master-mode-autoneg-source-contract-core-20260618" \
    "source-core-commit=201fdc5ae1c7d50bcf832f5ca022cc38cec69c0d" \
    "accepted-master-mode-proof-task-id=phase12-rp1-ethernet-bcm54213pe-selected-link-not-ready-pi5-proof-20260618" \
    "accepted-master-mode-proof-commit=7f029dc3fbb38032e396cc01b438ab999ace8ecd" \
    "selected-discriminator=bcm54213pe-phy1-mii-ctrl1000-master-mode-plus-bmcr-autoneg-restart" \
    "report-kind=" \
    "candidate" \
    "target=phy1-mii-ctrl1000-master-mode-plus-bmcr-autoneg-restart" \
    "ctrl1000-read-frame=0x60a60000" \
    "ctrl1000-write-frame=0x50a61a00" \
    "expected-ctrl1000-write-value=0x1a00 expected-ctrl1000-readback=0x1a00" \
    "bmcr-write-frame=0x50821200 bmcr-write-value=0x1200" \
    "selected-registers=pre-MII_CTRL1000,write-MII_CTRL1000,readback-MII_CTRL1000,restart-BMCR" \
    "poll-BMSR-first,poll-BMSR-second,poll-ANAR,poll-ANLPAR,poll-MII_CTRL1000,poll-MII_STAT1000,poll-passive-MACB_NSR_LINK" \
    "stage-boundaries=pre-mdio-marker,ncr-mpe-precondition,ctrl1000-pre-read,ctrl1000-write,ctrl1000-readback,bmcr-autoneg-restart,bounded-convergence-sampling" \
    "allowed-hardware-classifications=bcm54213pe-master-mode-autoneg-link-ready,bcm54213pe-master-mode-autoneg-timeout-link-not-ready" \
    "bcm54213pe-master-mode-autoneg-master-mode-readback-mismatch" \
    "bcm54213pe-master-mode-autoneg-bmcr-restart-blocker" \
    "no-mdio-no-ethernet-bcm54213pe-master-mode-autoneg-control" \
    "claims-ctrl1000-write-completed=" \
    "claims-ctrl1000-readback-completed=" \
    "claims-bmcr-write-executed=" \
    "claims-packet-io=false claims-networking=false claims-sockets=false" \
    "claims-ssh=false claims-phase-12-2=false claims-phase-transition=false" \
    "classification="; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing BCM54213PE master-mode-autoneg candidate string: $required" >&2
        exit 1
    fi
done

if [ -n "$CAPTURE_NONCE" ]; then
    if ! grep -Fq -- " capture-nonce=" "$kernel_strings"; then
        echo "kernel image missing BCM54213PE master-mode-autoneg candidate capture nonce label" >&2
        exit 1
    fi
    if ! grep -Fq -- "$CAPTURE_NONCE" "$kernel_strings"; then
        echo "kernel image missing BCM54213PE master-mode-autoneg candidate capture nonce: $CAPTURE_NONCE" >&2
        exit 1
    fi
fi

for forbidden in \
    "TALOS: rp1-ethernet-bcm54213pe-master-mode-autoneg-control" \
    "classification=no-mdio-no-ethernet-bcm54213pe-master-mode-autoneg-control" \
    "claims-packet-io=true" \
    "claims-networking=true" \
    "claims-ssh=true" \
    "claims-phase-12-2=true" \
    "claims-phase-transition=true" \
    "phy-reset-or-gpio32-action=true"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden BCM54213PE master-mode-autoneg candidate string: $forbidden" >&2
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
printf 'forbidden_bcm54213pe_master_mode_autoneg_candidate_runtime_strings_absent=true\n'
