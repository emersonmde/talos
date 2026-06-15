#!/bin/sh
set -eu

if [ "$#" -ne 1 ] && [ "$#" -ne 3 ]; then
    echo "usage: $0 <candidate-archive.tar.gz> [--capture-nonce NONCE]" >&2
    exit 2
fi

ARCHIVE="$1"
MARKER="TALOS: rp1-ethernet-post-physical-link-status-candidate"
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
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-ethernet-post-physical-link-status-candidate-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-ethernet-post-physical-link-status-candidate: start" \
    "before-readonly-phy1-and-macb-link-status-sample" \
    "$MARKER" \
    "post-physical-link-status-contract-id=phase12-rp1-ethernet-post-physical-link-status-contract-v2" \
    "task-id=phase12-rp1-ethernet-post-physical-link-status-v2-pi5-proof-20260615" \
    "source-contract-task-id=phase12-rp1-ethernet-post-physical-link-status-man-read-accounting-core-20260615" \
    "prior-source-contract-task-id=phase12-rp1-ethernet-post-physical-precondition-link-status-source-contract-20260614" \
    "accepted-frontier=operator-confirmed-physical-link-and-v2-autoneg-status-frontier" \
    "selected-discriminator=rp1-ethernet-post-physical-link-status-v2" \
    "hardware-proof-boundary-classification=hardware-proof-limited-to-bounded-clause22-man-read-command-and-passive-macb-nsr-status-output" \
    "report-kind=" \
    "candidate" \
    "target=rp1-ethernet-post-physical-link-status-v2" \
    "physical-link-precondition=confirmed" \
    "operator-evidence-reference=pi5-physical-ethernet-link-cabling-setup-confirmed" \
    "accepted-v2-bmcr-write-value=0x1200 accepted-v2-post-bmcr=0x1000" \
    "accepted-v2-post-bmsr-first=0x7949 accepted-v2-post-bmsr-second=0x7949" \
    "accepted-v2-post-anar=0x01e1 accepted-v2-post-anlpar=0x0000 accepted-v2-passive-macb-nsr-link=false" \
    "retained-gpio32-blockers=gpio32-write-restore-v2-no-write,event-clear-persistent-or-firmware-owned" \
    "selected-reads=BMCR:0x00,BMSR-first:0x01,BMSR-second:0x01,ANAR:0x04,ANLPAR:0x05,MACB_NSR:0x0008" \
    "selected-phy1-man-read-commands=BMCR:0x00:0x60820000,BMSR-first:0x01:0x60860000,BMSR-second:0x01:0x60860000,ANAR:0x04:0x60920000,ANLPAR:0x05:0x60960000" \
    "man-read-command-constraints=clause22,phy1,read-op,selected-registers-only,ncr-mpe-precondition,nsr-idle-before-and-after" \
    "bmcr-raw=" \
    "bmsr-first-raw=" \
    "bmsr-second-raw=" \
    "anar-raw=" \
    "anlpar-raw=" \
    "macb-nsr-raw=" \
    "bmsr-second-link-status=" \
    "bmsr-second-autoneg-complete=" \
    "anlpar-nonzero=" \
    "macb-nsr-link=" \
    "mdio-read-count=" \
    "man-read-command-write-count=5 phy-configuration-write-count=0 bmcr-write-count=0 macb-read-count=1 macb-configuration-write-count=0" \
    "bmcr-write-performed=false phy-config-write-performed=false" \
    "phy-reset-or-gpio32-action=false link-forcing=false packet-io-performed=false" \
    "bounded-runtime-hardware-claims=clause22-phy1-man-read-command-writes,passive-macb-nsr-read" \
    "allowed-classifications=post-physical-link-status-link-ready" \
    "post-physical-link-status-phy-not-ready" \
    "post-physical-link-status-mac-not-ready" \
    "post-physical-link-status-phy-mac-disagreement" \
    "post-physical-link-status-capture-blocker" \
    "post-physical-link-status-source-precondition-blocker" \
    "no-mdio-no-macb-post-physical-link-status-control" \
    "claims-runtime-mdio-reads=" \
    "claims-runtime-man-read-command-writes=" \
    "claims-runtime-macb-read=" \
    "claims-bmcr-write=false claims-autoneg-restart=false" \
    "claims-mac-configuration-write=false claims-link-forcing=false" \
    "claims-networking=false claims-sockets=false claims-ssh=false" \
    "claims-phase-12-2=false claims-phase-transition=false" \
    "classification="; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing post-physical link-status candidate string: $required" >&2
        exit 1
    fi
done

if [ -n "$CAPTURE_NONCE" ]; then
    if ! grep -Fq -- " capture-nonce=" "$kernel_strings"; then
        echo "kernel image missing post-physical link-status candidate capture nonce label" >&2
        exit 1
    fi
    if ! grep -Fq -- "$CAPTURE_NONCE" "$kernel_strings"; then
        echo "kernel image missing post-physical link-status candidate capture nonce: $CAPTURE_NONCE" >&2
        exit 1
    fi
fi

for forbidden in \
    "TALOS: rp1-ethernet-post-physical-link-status-control" \
    "no-mdio-no-macb-no-target-construction" \
    "target=none controller=none compatible=none" \
    "classification=no-mdio-no-macb-post-physical-link-status-control" \
    "bmcr-write-performed=true" \
    "phy-config-write-performed=true" \
    "claims-autoneg-restart=true" \
    "claims-link-forcing=true" \
    "post-physical-link-status-contract-id=phase12-rp1-ethernet-post-physical-precondition-link-status-contract-v1" \
    "mdio-write-count=0 macb-read-count=1 macb-write-count=0" \
    "claims-macb-write=false" \
    "phy-reset-or-gpio32-action=true" \
    "gpio32-event-clear" \
    "gpio32-phy-reset-write-restore"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden post-physical link-status candidate string: $forbidden" >&2
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
printf 'kernel_2712_bytes=%s\n' "$image_size"
printf 'kernel_header_text_offset=%s\n' "$text_offset"
printf 'kernel_header_image_size=%s\n' "$header_image_size"
printf 'kernel_header_flags=%s\n' "$flags"
printf 'required_marker=%s\n' "$MARKER"
if [ -n "$CAPTURE_NONCE" ]; then
    printf 'capture_nonce=%s\n' "$CAPTURE_NONCE"
fi
printf 'post_physical_link_status_candidate_static_review=passed\n'
