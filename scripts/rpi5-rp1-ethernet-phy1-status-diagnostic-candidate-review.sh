#!/bin/sh
set -eu

if [ "$#" -ne 1 ] && [ "$#" -ne 3 ]; then
    echo "usage: $0 <candidate-archive.tar.gz> [--capture-nonce NONCE]" >&2
    exit 2
fi

ARCHIVE="$1"
MARKER="TALOS: rp1-ethernet-phy1-status-diagnostic-candidate"
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
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-ethernet-phy1-status-diagnostic-candidate-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-ethernet-phy1-status-diagnostic-candidate: start" \
    "before-readonly-corrected-target-phy1-status-vector" \
    "$MARKER" \
    "phase12-rp1-ethernet-phy1-status-diagnostic-report-contract-v1" \
    "phase12-rp1-ethernet-phy1-status-diagnostic-pi5-proof-20260614" \
    "source-contract-id=phase12-rp1-ethernet-mdio-register-vector-source-contract-v1" \
    "accepted-frontier=rp1-ethernet-mdio-register-vector-phy1-visible-frontier-closed" \
    "selected-discriminator=rp1-ethernet-phy1-status-decode-from-accepted-register-vector" \
    "hardware-proof-boundary-classification=hardware-proof-limited-to-phy1-status-decode-control-output" \
    "report-kind=" \
    "candidate" \
    "target=rp1-ethernet-phy1-status-decode-from-accepted-register-vector" \
    "controller=rp1_eth compatible=raspberrypi,rp1-gem,cdns,macb" \
    "phy-handle=phy1 phy-node=ethernet-phy@1 phy-address=1" \
    "ncr-observed-target=" \
    "nsr-observed-target=" \
    "man-observed-target=" \
    "ncr-mpe-bit=4 nsr-idle-bit=2 man-data-bits=15:0" \
    "ncr-write-performed=false phy-config-write-performed=false" \
    "register-vector=MII_BMCR:0x00,MII_BMSR:0x01,MII_PHYSID1:0x02,MII_PHYSID2:0x03,MII_ADVERTISE_ANAR:0x04,MII_LPA_ANLPAR:0x05" \
    "raw-bmcr=" \
    "raw-bmsr=" \
    "raw-physid1=" \
    "raw-physid2=" \
    "raw-anar=" \
    "raw-anlpar=" \
    "bmcr-reset=" \
    "bmcr-loopback=" \
    "bmcr-speed-select-lsb-100=" \
    "bmcr-speed-select-msb-1000=" \
    "bmcr-speed=" \
    "bmcr-autoneg-enable=" \
    "bmsr-link-status=" \
    "bmsr-autoneg-complete=" \
    "bmsr-autoneg-ability=" \
    "bmsr-capabilities=" \
    "phy-id-oui=" \
    "phy-id-model=" \
    "phy-id-revision=" \
    "anar-selector=" \
    "anar-10hd=" \
    "anar-10fd=" \
    "anar-100tx-hd=" \
    "anar-100tx-fd=" \
    "anlpar-selector=" \
    "anlpar-acknowledge=" \
    "completed-register-count=" \
    "man-writes-performed=" \
    "man-restore-write-performed=false touched-fields=" \
    "allowed-classifications=mdio-phy1-status-diagnostic-visible" \
    "mdio-phy1-status-diagnostic-timeout" \
    "mdio-phy1-status-diagnostic-precondition-blocker" \
    "no-mdio-no-ethernet-rp1-ethernet-phy1-status-diagnostic-control" \
    "claims-runtime-mdio-transaction=" \
    "claims-mdio-phy-ownership=false claims-phy-config-write=false" \
    "claims-ncr-write-executed=false" \
    "claims-phy-reset-or-gpio32-action=false" \
    "claims-autoneg-restart=false claims-link-forcing=false" \
    "claims-ethernet-ready=false" \
    "claims-networking=false claims-sockets=false" \
    "claims-ssh=false claims-phase-12-2=false claims-phase-transition=false" \
    "classification="; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing PHY1 status candidate string: $required" >&2
        exit 1
    fi
done

if [ -n "$CAPTURE_NONCE" ]; then
    if ! grep -Fq -- " capture-nonce=" "$kernel_strings"; then
        echo "kernel image missing PHY1 status candidate capture nonce label" >&2
        exit 1
    fi
    if ! grep -Fq -- "$CAPTURE_NONCE" "$kernel_strings"; then
        echo "kernel image missing PHY1 status candidate capture nonce: $CAPTURE_NONCE" >&2
        exit 1
    fi
fi

for forbidden in \
    "TALOS: rp1-ethernet-phy1-status-diagnostic-control" \
    "no-mdio-no-ethernet-no-mmio-target-construction" \
    "target=none controller=none compatible=none" \
    "classification=no-mdio-no-ethernet-rp1-ethernet-phy1-status-diagnostic-control" \
    "phy-config-write-performed=true" \
    "claims-autoneg-restart=true" \
    "claims-link-forcing=true" \
    "gpio32-event-clear" \
    "gpio32-phy-reset-write-restore"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden PHY1 status candidate string: $forbidden" >&2
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
printf 'forbidden_phy1_status_candidate_runtime_strings_absent=true\n'
