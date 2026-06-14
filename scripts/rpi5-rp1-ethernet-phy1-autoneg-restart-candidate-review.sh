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
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-ethernet-phy1-autoneg-restart-candidate-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-ethernet-phy1-autoneg-restart-candidate: start" \
    "before-guarded-corrected-target-bmcr-autoneg-restart" \
    "TALOS: rp1-ethernet-phy1-autoneg-restart-candidate" \
    "phy1-autoneg-restart-contract-id=phase12-rp1-ethernet-phy1-autoneg-restart-contract-v1" \
    "task-id=phase12-rp1-ethernet-phy1-autoneg-restart-pi5-proof-20260614" \
    "guard-task-id=phase12-rp1-ethernet-phy1-autoneg-restart-guard-core-20260614" \
    "source-contract-task-id=phase12-rp1-ethernet-phy1-autoneg-restart-source-contract-20260614" \
    "accepted-phy1-bmcr=0x1000" \
    "accepted-phy1-bmsr-first=0x7949 accepted-phy1-bmsr-second=0x7949" \
    "accepted-phy1-anar=0x01e1 accepted-phy1-anlpar=0x0000" \
    "accepted-macb-nsr-raw=0x00000006 accepted-macb-nsr-link=false" \
    "selected-discriminator=rp1-ethernet-phy1-autoneg-restart" \
    "bmcr-register=0x00 bmcr-anenable=0x1000 bmcr-anrestart=0x0200" \
    "report-kind=" \
    "candidate" \
    "hardware-proof-boundary-classification=hardware-proof-limited-to-guarded-phy1-bmcr-autoneg-restart-control-output" \
    "target=corrected-target-clause22-phy1-bmcr-autoneg-restart" \
    "controller=rp1_eth compatible=raspberrypi,rp1-gem,cdns,macb" \
    "phy-handle=phy1 phy-node=ethernet-phy@1 phy-address=1" \
    "observed-window-macb-mid-context-cpu-physical-target=" \
    "observed-window-macb-mid-context-raw=" \
    "ncr-observed-target=" \
    "nsr-observed-target=" \
    "man-observed-target=" \
    "ncr-before=" \
    "ncr-mpe-precondition-met=" \
    "ncr-after=" \
    "selected-reads=pre-BMCR,pre-BMSR,pre-ANAR,pre-ANLPAR,post-BMCR,post-BMSR-first,post-BMSR-second,post-ANAR,post-ANLPAR" \
    "pre-bmcr=" \
    "pre-bmsr=" \
    "pre-anar=" \
    "pre-anlpar=" \
    "bmcr-isolate-precondition-clear=" \
    "bmcr-write-value=" \
    "bmcr-write-count=" \
    "touched-fields=BMCR_ANENABLE,BMCR_ANRESTART" \
    "post-bmcr=" \
    "post-bmsr-first=" \
    "post-bmsr-second=" \
    "post-anar=" \
    "post-anlpar=" \
    "pre-bmcr-autoneg-enable=" \
    "pre-bmcr-autoneg-restart=" \
    "post-bmcr-autoneg-enable=" \
    "post-bmcr-autoneg-restart=" \
    "post-bmsr-link-status=" \
    "post-bmsr-autoneg-complete=" \
    "post-anlpar-nonzero=" \
    "passive-macb-nsr-raw=" \
    "passive-macb-nsr-link=" \
    "bmcr-write-performed=" \
    "mdio-man-transactions-performed=" \
    "macb-read-performed=true macb-write-performed=false" \
    "phy-reset-or-gpio32-action=false link-forcing=false" \
    "allowed-classifications=phy1-autoneg-restart-link-ready,phy1-autoneg-restart-link-still-not-ready,phy1-autoneg-restart-physical-or-operator-precondition-blocker,phy1-autoneg-restart-precondition-blocker,phy1-autoneg-restart-capture-blocker,no-mdio-no-macb-phy1-autoneg-restart-control" \
    "rejected-runtime-hardware-claims=phy-reset-ownership,gpio32-action,macb-write,ncr-write,link-forcing,ethernet-readiness,packet-io,dma-descriptors,interrupt-completion,networking,sockets,ssh,phase-12-2,phase-transition" \
    "claims-runtime-mdio-transaction=" \
    "claims-bmcr-write-executed=" \
    "claims-exactly-one-bmcr-write=" \
    "claims-phy-reset-ownership=false claims-gpio32-action=false" \
    "claims-macb-write=false claims-ncr-write=false claims-link-forcing=false" \
    "claims-ethernet-ready=false claims-packet-io=false" \
    "claims-networking=false claims-sockets=false claims-ssh=false" \
    "claims-phase-12-2=false claims-phase-transition=false" \
    "classification="; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing PHY1 autoneg restart candidate string: $required" >&2
        exit 1
    fi
done

if [ -n "$CAPTURE_NONCE" ]; then
    if ! grep -Fq -- " capture-nonce=" "$kernel_strings"; then
        echo "kernel image missing PHY1 autoneg restart candidate capture nonce label" >&2
        exit 1
    fi
    if ! grep -Fq -- "$CAPTURE_NONCE" "$kernel_strings"; then
        echo "kernel image missing PHY1 autoneg restart candidate capture nonce: $CAPTURE_NONCE" >&2
        exit 1
    fi
fi

for forbidden in \
    "TALOS: rp1-ethernet-phy1-autoneg-restart-control" \
    "no-mdio-no-macb-no-target-construction" \
    "target=none controller=none compatible=none" \
    "classification=no-mdio-no-macb-phy1-autoneg-restart-control" \
    "macb-write-performed=true" \
    "phy-reset-or-gpio32-action=true" \
    "claims-link-forcing=true" \
    "gpio32-event-clear" \
    "gpio32-phy-reset-write-restore"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden PHY1 autoneg restart candidate string: $forbidden" >&2
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
printf 'forbidden_phy1_autoneg_restart_candidate_runtime_strings_absent=true\n'
