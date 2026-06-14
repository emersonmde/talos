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
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-ethernet-macb-nsr-link-readonly-candidate-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-ethernet-macb-nsr-link-readonly-candidate: start" \
    "before-readonly-observed-window-macb-nsr-link-read" \
    "TALOS: rp1-ethernet-macb-nsr-link-readonly-candidate" \
    "phase12-rp1-ethernet-macb-nsr-link-readonly-contract-v1" \
    "phase12-rp1-ethernet-macb-nsr-link-readonly-pi5-proof-20260614" \
    "source-contract-task-id=phase12-rp1-ethernet-macb-nsr-link-readonly-source-contract-20260614" \
    "accepted-frontier=rp1-ethernet-phy1-link-not-ready-frontier-closed" \
    "selected-discriminator=rp1-ethernet-macb-nsr-link-readonly" \
    "hardware-proof-boundary-classification=hardware-proof-limited-to-macb-nsr-link-readonly-control-output" \
    "report-kind=" \
    "candidate" \
    "target=rp1-ethernet-macb-nsr-link-readonly" \
    "controller=rp1_eth compatible=raspberrypi,rp1-gem,cdns,macb" \
    "observed-window-rp1-eth-base=0x1c00100000" \
    "observed-window-macb-mid-context-cpu-physical-target=" \
    "observed-window-macb-mid-context-raw=" \
    "macb-nsr-target=" \
    "macb-nsr-offset=0x0008 macb-nsr-link-bit=0" \
    "macb-nsr-raw=" \
    "macb-nsr-link=" \
    "macb-nsr-result-valid=true" \
    "accepted-phy1-bmcr=0x1000" \
    "accepted-phy1-bmsr-first=0x7949" \
    "accepted-phy1-bmsr-second=0x7949" \
    "accepted-phy1-anar=0x01e1 accepted-phy1-anlpar=0x0000" \
    "accepted-corrected-target-mdio-boundary=read-only-clause22-phy1-frontier-closed" \
    "retained-gpio32-blockers=gpio32-write-restore-v2-no-write,event-clear-persistent-or-firmware-owned" \
    "macb-read-performed=true macb-write-performed=false" \
    "mdio-target-constructed=false man-frame-constructed=false" \
    "phy-config-write-performed=false phy-reset-or-gpio32-action=false" \
    "dma-descriptor-action=false packet-io-performed=false" \
    "allowed-classifications=macb-nsr-link-readonly-link-set" \
    "macb-nsr-link-readonly-link-clear" \
    "macb-nsr-link-readonly-precondition-blocker" \
    "macb-nsr-link-readonly-capture-blocker" \
    "no-mmio-no-ethernet-macb-nsr-link-control" \
    "claims-runtime-macb-nsr-read=" \
    "claims-macb-write-executed=false" \
    "claims-mdio-or-phy-access=false claims-phy-config-write=false" \
    "claims-bmcr-write=false claims-autoneg-restart=false" \
    "claims-link-forcing=false claims-phy-reset-or-gpio32-action=false" \
    "claims-networking=false claims-sockets=false claims-ssh=false" \
    "claims-phase-12-2=false claims-phase-transition=false" \
    "classification="; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing MACB NSR_LINK candidate string: $required" >&2
        exit 1
    fi
done

if [ -n "$CAPTURE_NONCE" ]; then
    if ! grep -Fq -- " capture-nonce=" "$kernel_strings"; then
        echo "kernel image missing MACB NSR_LINK candidate capture nonce label" >&2
        exit 1
    fi
    if ! grep -Fq -- "$CAPTURE_NONCE" "$kernel_strings"; then
        echo "kernel image missing MACB NSR_LINK candidate capture nonce: $CAPTURE_NONCE" >&2
        exit 1
    fi
fi

for forbidden in \
    "TALOS: rp1-ethernet-macb-nsr-link-readonly-control" \
    "no-mmio-no-ethernet-no-macb-nsr-target-construction" \
    "target=none controller=none compatible=none" \
    "classification=no-mmio-no-ethernet-macb-nsr-link-control" \
    "macb-write-performed=true" \
    "claims-autoneg-restart=true" \
    "claims-link-forcing=true" \
    "gpio32-event-clear" \
    "gpio32-phy-reset-write-restore"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden MACB NSR_LINK candidate string: $forbidden" >&2
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
printf 'forbidden_macb_nsr_link_candidate_runtime_strings_absent=true\n'
