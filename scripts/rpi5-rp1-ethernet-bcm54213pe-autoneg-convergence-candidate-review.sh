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
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-ethernet-bcm54213pe-autoneg-convergence-candidate-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-ethernet-bcm54213pe-autoneg-convergence-candidate: start" \
    "before-bounded-phy1-bmcr-autoneg-convergence-poll" \
    "TALOS: rp1-ethernet-bcm54213pe-autoneg-convergence-candidate" \
    "bcm54213pe-autoneg-convergence-proof-contract-id=phase12-rp1-ethernet-bcm54213pe-autoneg-convergence-proof-contract-v1" \
    "task-id=phase12-rp1-ethernet-bcm54213pe-autoneg-convergence-pi5-proof-20260616" \
    "proof-core-task-id=phase12-rp1-ethernet-bcm54213pe-autoneg-convergence-core-20260616" \
    "accepted-restart-closeout-commit=0de9450e8e1c2d5f458d1c778f4f25e80860c0a2" \
    "selected-discriminator=bcm54213pe-phy1-bmcr-autoneg-restart-plus-convergence-poll" \
    "report-kind=" \
    "candidate" \
    "hardware-proof-boundary-classification=bcm54213pe-autoneg-convergence-proof-core-local-static" \
    "target=corrected-target-phy1-bmcr-autoneg-restart-plus-convergence-poll" \
    "phy-model=Broadcom-BCM54213PE" \
    "phy-handle=phy1 phy-node=ethernet-phy@1 phy-address=1" \
    "bmcr-write-frame=0x50821200" \
    "touched-fields=BMCR_ANENABLE,BMCR_ANRESTART" \
    "poll-count-bound=" \
    "poll-delay-spins=" \
    "selected-registers=restart-BMCR,poll-BMCR,poll-BMSR-first,poll-BMSR-second,poll-ANAR,poll-ANLPAR,poll-MII_CTRL1000,poll-MII_STAT1000,poll-passive-MACB_NSR_LINK" \
    "link-ready-terminal=" \
    "bmcr-write-performed=" \
    "mdio-man-transactions-performed=" \
    "macb-read-performed=true macb-write-performed=false" \
    "allowed-hardware-classifications=bcm54213pe-autoneg-convergence-link-ready,bcm54213pe-autoneg-convergence-timeout-link-not-ready,bcm54213pe-autoneg-convergence-precondition-blocker,bcm54213pe-autoneg-convergence-capture-blocker,no-mdio-no-ethernet-bcm54213pe-autoneg-convergence-control" \
    "rejected-runtime-hardware-claims=target-drift,extra-phy-writes,selector-config-access,gpio32-reset-action,broadcom-shadow-mmd-aux-access,interrupt-ownership,phy-mac-configuration,link-forcing,packet-io,networking,sockets,ssh,phase-12-2,phase-transition" \
    "claims-exactly-one-bmcr-write=" \
    "claims-extra-phy-writes=false claims-selector-config-access=false"; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing BCM54213PE autoneg convergence candidate string: $required" >&2
        exit 1
    fi
done

if [ -n "$CAPTURE_NONCE" ]; then
    if ! grep -Fq -- " capture-nonce=" "$kernel_strings"; then
        echo "kernel image missing BCM54213PE convergence candidate capture nonce label" >&2
        exit 1
    fi
    if ! grep -Fq -- "$CAPTURE_NONCE" "$kernel_strings"; then
        echo "kernel image missing BCM54213PE convergence candidate capture nonce: $CAPTURE_NONCE" >&2
        exit 1
    fi
fi

for forbidden in \
    "TALOS: rp1-ethernet-bcm54213pe-autoneg-convergence-control" \
    "no-mdio-no-macb-no-gpio32-no-phy-target-construction" \
    "target=none controller=none compatible=none" \
    "classification=no-mdio-no-ethernet-bcm54213pe-autoneg-convergence-control" \
    "macb-write-performed=true" \
    "phy-reset-or-gpio32-action=true" \
    "claims-link-forcing=true" \
    "selector-config-access=true" \
    "broadcom-shadow-mmd-aux-access=true" \
    "gpio32-event-clear" \
    "gpio32-phy-reset"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden BCM54213PE convergence candidate string: $forbidden" >&2
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
printf 'forbidden_bcm54213pe_autoneg_convergence_candidate_runtime_strings_absent=true\n'
