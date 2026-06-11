#!/bin/sh
set -eu

if [ "$#" -ne 1 ] && [ "$#" -ne 3 ]; then
    echo "usage: $0 <control-archive.tar.gz> [--capture-nonce NONCE]" >&2
    exit 2
fi

ARCHIVE="$1"
MARKER="TALOS: rp1-ethernet-mdio-register-vector-control"
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
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-ethernet-mdio-register-vector-control-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-ethernet-mdio-register-vector-control: start" \
    "no-mdio-no-ethernet-no-mmio-target-construction" \
    "$MARKER" \
    "phase12-rp1-ethernet-mdio-register-vector-guard-report-contract-v1" \
    "phase12-rp1-ethernet-mdio-register-vector-source-contract-v1" \
    "phase12-rp1-ethernet-mdio-register-vector-source-contract-20260611" \
    "selected-discriminator=rp1-ethernet-mdio-after-mpe-clause22-phy1-register-vector" \
    "hardware-proof-boundary-classification=hardware-proof-limited-to-corrected-target-mdio-register-vector-control-output" \
    "report-kind=" \
    "no-mdio-no-ethernet-control" \
    "target=none controller=none compatible=none" \
    "phy-handle=none phy-node=none phy-address=none" \
    "observed-window-macb-mid-context-cpu-physical-target=not-constructed" \
    "corrected-window-comparator-cpu-physical-target=none" \
    "ncr-observed-target=not-constructed" \
    "nsr-observed-target=not-constructed" \
    "man-observed-target=not-constructed" \
    "ncr-mpe-bit=withheld nsr-idle-bit=withheld man-data-bits=withheld" \
    "ncr-mpe-write-performed=false" \
    "register-vector=withheld" \
    "bmcr-man-frame=withheld bmsr-man-frame=withheld physid1-man-frame=withheld physid2-man-frame=withheld anar-man-frame=withheld anlpar-man-frame=withheld" \
    "bmcr=withheld bmcr-valid=false" \
    "bmsr=withheld bmsr-valid=false" \
    "physid1=withheld physid1-valid=false" \
    "physid2=withheld physid2-valid=false" \
    "anar=withheld anar-valid=false" \
    "anlpar=withheld anlpar-valid=false" \
    "completed-register-count=0" \
    "man-writes-performed=false" \
    "man-restore-write-performed=false touched-fields=none" \
    "no-mdio-no-ethernet-rp1-ethernet-mdio-register-vector-control" \
    "claims-runtime-mdio-transaction=" \
    "claims-mdio-phy-ownership=false" \
    "claims-ncr-write-executed=false" \
    "claims-phy-absence-from-all-ones=false" \
    "claims-gpio32-phy-reset-ownership=false" \
    "claims-ethernet-ready=false" \
    "claims-networking=false claims-sockets=false" \
    "claims-ssh=false claims-phase-12-2=false claims-phase-transition=false" \
    "classification=no-mdio-no-ethernet-rp1-ethernet-mdio-register-vector-control"; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing MDIO register-vector control string: $required" >&2
        exit 1
    fi
done

if [ -n "$CAPTURE_NONCE" ]; then
    if ! grep -Fq -- " capture-nonce=" "$kernel_strings"; then
        echo "kernel image missing MDIO register-vector control capture nonce label" >&2
        exit 1
    fi
    if ! grep -Fq -- "$CAPTURE_NONCE" "$kernel_strings"; then
        echo "kernel image missing MDIO register-vector control capture nonce: $CAPTURE_NONCE" >&2
        exit 1
    fi
fi

for forbidden in \
    "TALOS: rp1-ethernet-mdio-register-vector-candidate" \
    "before-corrected-target-guarded-phy1-register-vector-man-transactions" \
    "target=rp1-ethernet-mdio-after-mpe-clause22-phy1-register-vector" \
    "ncr-observed-target=0x" \
    "man-observed-target=0x" \
    "claims-runtime-mdio-transaction=true" \
    "touched-fields=MAN" \
    "gpio32-event-clear" \
    "gpio32-phy-reset-write-restore"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden MDIO register-vector control string: $forbidden" >&2
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
printf 'forbidden_mdio_register_vector_control_runtime_strings_absent=true\n'
