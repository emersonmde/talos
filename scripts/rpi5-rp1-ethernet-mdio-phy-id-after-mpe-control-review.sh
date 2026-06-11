#!/bin/sh
set -eu

if [ "$#" -ne 1 ] && [ "$#" -ne 3 ]; then
    echo "usage: $0 <control-archive.tar.gz> [--capture-nonce NONCE]" >&2
    exit 2
fi

ARCHIVE="$1"
MARKER="TALOS: rp1-ethernet-mdio-phy-id-after-mpe-control"
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
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-ethernet-mdio-phy-id-after-mpe-control-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-ethernet-mdio-phy-id-after-mpe-control: start" \
    "no-mdio-no-ethernet-no-mmio-target-construction" \
    "$MARKER" \
    "phase12-rp1-ethernet-mdio-phy-id-after-mpe-guard-report-contract-v1" \
    "phase12-rp1-ethernet-mdio-phy-id-after-mpe-source-contract-v1" \
    "phase12-rp1-ethernet-mdio-phy-id-after-mpe-source-contract-20260611" \
    "accepted-mpe-frontier=rp1-ethernet-mdio-mpe-enable-ownership-frontier-closed" \
    "hardware-proof-boundary-classification=hardware-proof-limited-to-corrected-target-after-mpe-mdio-phy-id-control-output" \
    "report-kind=" \
    "no-mdio-no-ethernet-control" \
    "target=none controller=none compatible=none" \
    "phy-handle=none phy-node=none phy-address=none" \
    "observed-window-macb-mid-context-cpu-physical-target=not-constructed" \
    "ncr-observed-target=not-constructed" \
    "nsr-observed-target=not-constructed" \
    "man-observed-target=not-constructed" \
    "ncr-mpe-bit=withheld nsr-idle-bit=withheld man-data-bits=withheld" \
    "ncr-mpe-write-performed=false" \
    "phy-id-registers=withheld" \
    "physid1-man-frame=withheld physid2-man-frame=withheld" \
    "man-writes-performed=false" \
    "man-restore-write-performed=false touched-fields=none" \
    "allowed-classifications=mdio-phy1-physid-after-mpe-visible" \
    "no-mdio-no-ethernet-rp1-ethernet-mdio-phy-id-after-mpe-control" \
    "claims-runtime-mdio-transaction=" \
    "claims-mdio-phy-ownership=false" \
    "claims-mpe-write-permission=false" \
    "claims-ncr-write-permission=false" \
    "claims-gpio32-phy-reset-ownership=false" \
    "claims-ethernet-ready=false" \
    "claims-networking=false claims-sockets=false" \
    "claims-ssh=false claims-phase-12-2=false claims-phase-transition=false" \
    "classification=no-mdio-no-ethernet-rp1-ethernet-mdio-phy-id-after-mpe-control"; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing after-MPE MDIO PHY-ID control string: $required" >&2
        exit 1
    fi
done

if [ -n "$CAPTURE_NONCE" ]; then
    if ! grep -Fq -- " capture-nonce=" "$kernel_strings"; then
        echo "kernel image missing after-MPE MDIO PHY-ID control capture nonce label" >&2
        exit 1
    fi
    if ! grep -Fq -- "$CAPTURE_NONCE" "$kernel_strings"; then
        echo "kernel image missing after-MPE MDIO PHY-ID control capture nonce: $CAPTURE_NONCE" >&2
        exit 1
    fi
fi

for forbidden in \
    "TALOS: rp1-ethernet-mdio-phy-id-after-mpe-candidate" \
    "before-corrected-target-guarded-phy1-physid-man-transactions" \
    "target=rp1-ethernet-mdio-after-mpe-clause22-phy1-physid1-physid2" \
    "ncr-observed-target=0x" \
    "man-observed-target=0x" \
    "claims-runtime-mdio-transaction=true" \
    "touched-fields=MAN" \
    "gpio32-event-clear" \
    "gpio32-phy-reset-write-restore"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden after-MPE MDIO PHY-ID control string: $forbidden" >&2
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
printf 'forbidden_mdio_phy_id_after_mpe_control_runtime_strings_absent=true\n'
