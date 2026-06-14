#!/bin/sh
set -eu

if [ "$#" -ne 1 ] && [ "$#" -ne 3 ]; then
    echo "usage: $0 <candidate-archive.tar.gz> [--capture-nonce NONCE]" >&2
    exit 2
fi

ARCHIVE="$1"
MARKER="TALOS: rp1-ethernet-phy1-bmsr-double-sample-link-readiness-candidate"
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
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-candidate-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-candidate: start" \
    "before-readonly-corrected-target-bmcr-bmsr-double-sample" \
    "$MARKER" \
    "phase12-rp1-ethernet-phy1-bmsr-latch-low-double-sample-link-readiness-contract-v1" \
    "phase12-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-pi5-proof-20260614" \
    "source-contract-id=phase12-rp1-ethernet-phy1-link-readiness-source-contract-20260614" \
    "accepted-frontier=rp1-ethernet-phy1-status-diagnostic-frontier-closed" \
    "selected-discriminator=rp1-ethernet-phy1-bmsr-latch-low-double-sample-link-readiness" \
    "hardware-proof-boundary-classification=hardware-proof-limited-to-phy1-bmsr-double-sample-link-readiness-control-output" \
    "report-kind=" \
    "candidate" \
    "target=rp1-ethernet-phy1-bmsr-latch-low-double-sample-link-readiness" \
    "controller=rp1_eth compatible=raspberrypi,rp1-gem,cdns,macb" \
    "phy-handle=phy1 phy-node=ethernet-phy@1 phy-address=1" \
    "ncr-observed-target=" \
    "nsr-observed-target=" \
    "man-observed-target=" \
    "ncr-mpe-bit=4 nsr-idle-bit=2 man-data-bits=15:0" \
    "ncr-write-performed=false phy-config-write-performed=false" \
    "selected-reads=BMCR:MII_BMCR:0x00:MAN:0x60820000,BMSR-first:MII_BMSR:0x01:MAN:0x60860000,BMSR-second:MII_BMSR:0x01:MAN:0x60860000" \
    "raw-bmcr=" \
    "raw-bmsr-first=" \
    "raw-bmsr-second=" \
    "bmcr-valid=" \
    "bmsr-first-valid=" \
    "bmsr-second-valid=" \
    "bmcr-reset=" \
    "bmcr-loopback=" \
    "bmcr-restart-autoneg=" \
    "bmcr-autoneg-enable=" \
    "bmcr-preconditions-clear=" \
    "bmsr-first-link-status=" \
    "bmsr-first-autoneg-complete=" \
    "bmsr-second-link-status=" \
    "bmsr-second-autoneg-complete=" \
    "bmsr-second-autoneg-ability=" \
    "latch-low-sample-policy=classify-from-second-bmsr-sample" \
    "link-readiness-result-valid=" \
    "man-writes-performed=" \
    "man-restore-write-performed=false touched-fields=" \
    "allowed-classifications=mdio-phy1-bmsr-double-sample-link-ready" \
    "mdio-phy1-bmsr-double-sample-link-not-ready" \
    "mdio-phy1-bmsr-double-sample-bmcr-precondition-blocker" \
    "mdio-phy1-bmsr-double-sample-timeout" \
    "no-mdio-no-ethernet-rp1-ethernet-phy1-link-readiness-control" \
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
        echo "kernel image missing PHY1 BMSR double-sample candidate string: $required" >&2
        exit 1
    fi
done

if [ -n "$CAPTURE_NONCE" ]; then
    if ! grep -Fq -- " capture-nonce=" "$kernel_strings"; then
        echo "kernel image missing PHY1 BMSR double-sample candidate capture nonce label" >&2
        exit 1
    fi
    if ! grep -Fq -- "$CAPTURE_NONCE" "$kernel_strings"; then
        echo "kernel image missing PHY1 BMSR double-sample candidate capture nonce: $CAPTURE_NONCE" >&2
        exit 1
    fi
fi

for forbidden in \
    "TALOS: rp1-ethernet-phy1-bmsr-double-sample-link-readiness-control" \
    "no-mdio-no-ethernet-no-mmio-target-construction" \
    "target=none controller=none compatible=none" \
    "classification=no-mdio-no-ethernet-rp1-ethernet-phy1-link-readiness-control" \
    "phy-config-write-performed=true" \
    "claims-autoneg-restart=true" \
    "claims-link-forcing=true" \
    "gpio32-event-clear" \
    "gpio32-phy-reset-write-restore"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden PHY1 BMSR double-sample candidate string: $forbidden" >&2
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
printf 'forbidden_phy1_bmsr_double_sample_candidate_runtime_strings_absent=true\n'
