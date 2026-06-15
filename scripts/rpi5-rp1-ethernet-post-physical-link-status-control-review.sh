#!/bin/sh
set -eu

if [ "$#" -ne 1 ] && [ "$#" -ne 3 ]; then
    echo "usage: $0 <control-archive.tar.gz> [--capture-nonce NONCE]" >&2
    exit 2
fi

ARCHIVE="$1"
MARKER="TALOS: rp1-ethernet-post-physical-link-status-control"
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
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-ethernet-post-physical-link-status-control-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-ethernet-post-physical-link-status-control: start" \
    "no-mdio-no-macb-no-target-construction" \
    "$MARKER" \
    "post-physical-link-status-contract-id=phase12-rp1-ethernet-post-physical-precondition-link-status-contract-v1" \
    "task-id=phase12-rp1-ethernet-post-physical-precondition-link-status-pi5-proof-20260614" \
    "source-contract-task-id=phase12-rp1-ethernet-post-physical-precondition-link-status-source-contract-20260614" \
    "accepted-frontier=operator-confirmed-physical-link-and-v2-autoneg-status-frontier" \
    "selected-discriminator=rp1-ethernet-post-physical-precondition-link-status" \
    "report-kind=" \
    "no-mdio-no-macb-control" \
    "target=none controller=none compatible=none" \
    "phy-handle=none phy-node=none phy-address=none" \
    "physical-link-precondition=confirmed" \
    "observed-window-macb-mid-context-cpu-physical-target=not-constructed" \
    "ncr-observed-target=not-constructed" \
    "nsr-observed-target=not-constructed" \
    "man-observed-target=not-constructed" \
    "macb-nsr-target=not-constructed" \
    "selected-reads=withheld" \
    "bmcr-raw=withheld bmsr-first-raw=withheld bmsr-second-raw=withheld" \
    "anar-raw=withheld anlpar-raw=withheld macb-nsr-raw=withheld" \
    "mdio-read-count=0 mdio-write-count=0 macb-read-count=0 macb-write-count=0" \
    "bmcr-write-performed=false phy-config-write-performed=false" \
    "phy-reset-or-gpio32-action=false link-forcing=false packet-io-performed=false" \
    "allowed-classifications=post-physical-link-status-link-ready" \
    "no-mdio-no-macb-post-physical-link-status-control" \
    "claims-runtime-mdio-reads=" \
    "claims-runtime-macb-read=" \
    "claims-bmcr-write=false claims-autoneg-restart=false" \
    "claims-macb-write=false claims-link-forcing=false" \
    "claims-networking=false claims-sockets=false claims-ssh=false" \
    "classification=no-mdio-no-macb-post-physical-link-status-control"; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing post-physical link-status control string: $required" >&2
        exit 1
    fi
done

if [ -n "$CAPTURE_NONCE" ]; then
    if ! grep -Fq -- " capture-nonce=" "$kernel_strings"; then
        echo "kernel image missing post-physical link-status control capture nonce label" >&2
        exit 1
    fi
    if ! grep -Fq -- "$CAPTURE_NONCE" "$kernel_strings"; then
        echo "kernel image missing post-physical link-status control capture nonce: $CAPTURE_NONCE" >&2
        exit 1
    fi
fi

for forbidden in \
    "TALOS: rp1-ethernet-post-physical-link-status-candidate" \
    "before-readonly-phy1-and-macb-link-status-sample" \
    "target=rp1-ethernet-post-physical-precondition-link-status" \
    "classification=post-physical-link-status-link-ready" \
    "classification=post-physical-link-status-phy-not-ready" \
    "classification=post-physical-link-status-mac-not-ready" \
    "classification=post-physical-link-status-phy-mac-disagreement" \
    "mdio-read-count=5" \
    "macb-read-count=1"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden post-physical link-status control string: $forbidden" >&2
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
printf 'kernel_2712_bytes=%s\n' "$image_size"
printf 'kernel_header_text_offset=%s\n' "$text_offset"
printf 'kernel_header_image_size=%s\n' "$header_image_size"
printf 'kernel_header_flags=%s\n' "$flags"
printf 'required_marker=%s\n' "$MARKER"
if [ -n "$CAPTURE_NONCE" ]; then
    printf 'capture_nonce=%s\n' "$CAPTURE_NONCE"
fi
printf 'post_physical_link_status_control_static_review=passed\n'
