#!/bin/sh
set -eu

if [ "$#" -ne 1 ] && [ "$#" -ne 3 ]; then
    echo "usage: $0 <control-archive.tar.gz> [--capture-nonce NONCE]" >&2
    exit 2
fi

ARCHIVE="$1"
MARKER="TALOS: rp1-ethernet-phy1-status-diagnostic-control"
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
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-ethernet-phy1-status-diagnostic-control-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-ethernet-phy1-status-diagnostic-control: start" \
    "no-mdio-no-ethernet-no-mmio-target-construction" \
    "$MARKER" \
    "phase12-rp1-ethernet-phy1-status-diagnostic-report-contract-v1" \
    "phase12-rp1-ethernet-phy1-status-diagnostic-pi5-proof-20260614" \
    "source-contract-id=phase12-rp1-ethernet-mdio-register-vector-source-contract-v1" \
    "accepted-frontier=rp1-ethernet-mdio-register-vector-phy1-visible-frontier-closed" \
    "selected-discriminator=rp1-ethernet-phy1-status-decode-from-accepted-register-vector" \
    "hardware-proof-boundary-classification=hardware-proof-limited-to-phy1-status-decode-control-output" \
    "report-kind=" \
    "no-mdio-no-ethernet-control" \
    "target=none controller=none compatible=none" \
    "phy-handle=none phy-node=none phy-address=none" \
    "ncr-observed-target=not-constructed" \
    "nsr-observed-target=not-constructed" \
    "man-observed-target=not-constructed" \
    "ncr-mpe-bit=withheld nsr-idle-bit=withheld man-data-bits=withheld" \
    "ncr-write-performed=false phy-config-write-performed=false" \
    "register-vector=withheld raw-bmcr=withheld raw-bmsr=withheld raw-physid1=withheld raw-physid2=withheld raw-anar=withheld raw-anlpar=withheld" \
    "bmcr-valid=false bmsr-valid=false physid1-valid=false physid2-valid=false anar-valid=false anlpar-valid=false" \
    "decoded-bmcr=withheld decoded-bmsr=withheld decoded-phy-id=withheld decoded-anar=withheld decoded-anlpar=withheld" \
    "completed-register-count=0" \
    "nsr-before-vector=withheld nsr-after-vector=withheld man-after-vector=withheld" \
    "man-writes-performed=false" \
    "man-restore-write-performed=false touched-fields=none" \
    "allowed-classifications=mdio-phy1-status-diagnostic-visible" \
    "no-mdio-no-ethernet-rp1-ethernet-phy1-status-diagnostic-control" \
    "claims-runtime-mdio-transaction=" \
    "claims-mdio-phy-ownership=false claims-phy-config-write=false" \
    "claims-ncr-write-executed=false" \
    "claims-phy-reset-or-gpio32-action=false" \
    "claims-autoneg-restart=false claims-link-forcing=false" \
    "claims-ethernet-ready=false" \
    "claims-networking=false claims-sockets=false" \
    "claims-ssh=false claims-phase-12-2=false claims-phase-transition=false" \
    "classification=no-mdio-no-ethernet-rp1-ethernet-phy1-status-diagnostic-control"; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing PHY1 status control string: $required" >&2
        exit 1
    fi
done

if [ -n "$CAPTURE_NONCE" ]; then
    if ! grep -Fq -- " capture-nonce=" "$kernel_strings"; then
        echo "kernel image missing PHY1 status control capture nonce label" >&2
        exit 1
    fi
    if ! grep -Fq -- "$CAPTURE_NONCE" "$kernel_strings"; then
        echo "kernel image missing PHY1 status control capture nonce: $CAPTURE_NONCE" >&2
        exit 1
    fi
fi

for forbidden in \
    "TALOS: rp1-ethernet-phy1-status-diagnostic-candidate" \
    "before-readonly-corrected-target-phy1-status-vector" \
    "target=rp1-ethernet-phy1-status-decode-from-accepted-register-vector" \
    "ncr-observed-target=0x" \
    "man-observed-target=0x" \
    "raw-bmcr=0x" \
    "bmcr-reset=" \
    "claims-runtime-mdio-transaction=true" \
    "touched-fields=MAN" \
    "gpio32-event-clear" \
    "gpio32-phy-reset-write-restore"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden PHY1 status control string: $forbidden" >&2
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
printf 'forbidden_phy1_status_control_runtime_strings_absent=true\n'
