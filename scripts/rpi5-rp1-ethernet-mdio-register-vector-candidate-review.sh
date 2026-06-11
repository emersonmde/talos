#!/bin/sh
set -eu

if [ "$#" -ne 1 ] && [ "$#" -ne 3 ]; then
    echo "usage: $0 <candidate-archive.tar.gz> [--capture-nonce NONCE]" >&2
    exit 2
fi

ARCHIVE="$1"
MARKER="TALOS: rp1-ethernet-mdio-register-vector-candidate"
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
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-ethernet-mdio-register-vector-candidate-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-ethernet-mdio-register-vector-candidate: start" \
    "before-corrected-target-guarded-phy1-register-vector-man-transactions" \
    "$MARKER" \
    "phase12-rp1-ethernet-mdio-register-vector-guard-report-contract-v1" \
    "phase12-rp1-ethernet-mdio-register-vector-source-contract-v1" \
    "phase12-rp1-ethernet-mdio-register-vector-source-contract-20260611" \
    "selected-discriminator=rp1-ethernet-mdio-after-mpe-clause22-phy1-register-vector" \
    "hardware-proof-boundary-classification=hardware-proof-limited-to-corrected-target-mdio-register-vector-control-output" \
    "report-kind=" \
    "candidate" \
    "target=rp1-ethernet-mdio-after-mpe-clause22-phy1-register-vector" \
    "controller=rp1_eth compatible=raspberrypi,rp1-gem,cdns,macb" \
    "phy-handle=phy1 phy-node=ethernet-phy@1 phy-address=1" \
    "observed-window-macb-mid-context-cpu-physical-target=" \
    "corrected-window-comparator-cpu-physical-target=" \
    "ncr-observed-target=" \
    "nsr-observed-target=" \
    "man-observed-target=" \
    "ncr-mpe-bit=4 nsr-idle-bit=2 man-data-bits=15:0" \
    "ncr-mpe-precondition-met=" \
    "ncr-mpe-write-performed=false" \
    "register-vector=MII_BMCR:0x00,MII_BMSR:0x01,MII_PHYSID1:0x02,MII_PHYSID2:0x03,MII_ADVERTISE_ANAR:0x04,MII_LPA_ANLPAR:0x05" \
    "bmcr-man-frame=" \
    "bmsr-man-frame=" \
    "physid1-man-frame=" \
    "physid2-man-frame=" \
    "anar-man-frame=" \
    "anlpar-man-frame=" \
    "bmcr-valid=" \
    "bmsr-valid=" \
    "physid1-valid=" \
    "physid2-valid=" \
    "anar-valid=" \
    "anlpar-valid=" \
    "completed-register-count=" \
    "man-writes-performed=" \
    "man-restore-write-performed=false touched-fields=" \
    "allowed-classifications=mdio-phy1-register-vector-visible" \
    "mdio-phy1-register-vector-global-all-ones-visible" \
    "mdio-phy1-register-vector-physid-only-all-ones-mixed-visible" \
    "mdio-phy1-register-vector-timeout" \
    "mdio-phy1-register-vector-precondition-blocker" \
    "precise-staging-capture-blocker" \
    "claims-runtime-mdio-transaction=" \
    "claims-ncr-write-executed=false" \
    "claims-phy-absence-from-all-ones=false" \
    "claims-mdio-phy-ownership=false" \
    "claims-gpio32-phy-reset-ownership=false" \
    "claims-ethernet-ready=false" \
    "claims-networking=false claims-sockets=false" \
    "claims-ssh=false claims-phase-12-2=false claims-phase-transition=false" \
    "classification="; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing MDIO register-vector candidate string: $required" >&2
        exit 1
    fi
done

if [ -n "$CAPTURE_NONCE" ]; then
    if ! grep -Fq -- " capture-nonce=" "$kernel_strings"; then
        echo "kernel image missing MDIO register-vector candidate capture nonce label" >&2
        exit 1
    fi
    if ! grep -Fq -- "$CAPTURE_NONCE" "$kernel_strings"; then
        echo "kernel image missing MDIO register-vector candidate capture nonce: $CAPTURE_NONCE" >&2
        exit 1
    fi
fi

for forbidden in \
    "TALOS: rp1-ethernet-mdio-register-vector-control" \
    "no-mdio-no-ethernet-no-mmio-target-construction" \
    "target=none controller=none compatible=none" \
    "claims-runtime-mdio-transaction=false" \
    "classification=no-mdio-no-ethernet-rp1-ethernet-mdio-register-vector-control" \
    "ncr-mpe-write-performed=true" \
    "claims-ncr-write-executed=true" \
    "gpio32-event-clear" \
    "gpio32-phy-reset-write-restore"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden MDIO register-vector candidate string: $forbidden" >&2
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
printf 'forbidden_mdio_register_vector_candidate_runtime_strings_absent=true\n'
