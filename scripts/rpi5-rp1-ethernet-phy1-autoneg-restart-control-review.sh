#!/bin/sh
set -eu

if [ "$#" -ne 1 ] && [ "$#" -ne 3 ]; then
    echo "usage: $0 <control-archive.tar.gz> [--capture-nonce NONCE]" >&2
    exit 2
fi

ARCHIVE="$1"
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
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-ethernet-phy1-autoneg-restart-control-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-ethernet-phy1-autoneg-restart-control: start" \
    "no-mdio-no-macb-no-target-construction" \
    "TALOS: rp1-ethernet-phy1-autoneg-restart-control" \
    "phy1-autoneg-restart-contract-id=phase12-rp1-ethernet-phy1-autoneg-restart-contract-v1" \
    "task-id=phase12-rp1-ethernet-phy1-autoneg-restart-pi5-proof-20260614" \
    "guard-task-id=phase12-rp1-ethernet-phy1-autoneg-restart-guard-core-20260614" \
    "source-contract-task-id=phase12-rp1-ethernet-phy1-autoneg-restart-source-contract-20260614" \
    "accepted-phy1-bmcr=0x1000" \
    "accepted-phy1-bmsr-first=0x7949 accepted-phy1-bmsr-second=0x7949" \
    "accepted-phy1-anar=0x01e1 accepted-phy1-anlpar=0x0000" \
    "accepted-macb-nsr-raw=0x00000006 accepted-macb-nsr-link=false" \
    "selected-discriminator=rp1-ethernet-phy1-autoneg-restart" \
    "report-kind=" \
    "no-mdio-no-macb-control" \
    "hardware-proof-boundary-classification=hardware-proof-limited-to-guarded-phy1-bmcr-autoneg-restart-control-output" \
    "target=none controller=none compatible=none" \
    "phy-handle=none phy-node=none phy-address=none" \
    "observed-window-macb-mid-context-cpu-physical-target=not-constructed" \
    "observed-window-macb-mid-context-raw=withheld" \
    "ncr-observed-target=not-constructed" \
    "nsr-observed-target=not-constructed" \
    "man-observed-target=not-constructed" \
    "selected-reads=withheld" \
    "pre-bmcr=withheld pre-bmsr=withheld pre-anar=withheld pre-anlpar=withheld" \
    "bmcr-isolate-precondition-clear=false" \
    "bmcr-write-value=withheld bmcr-write-count=0 touched-fields=none" \
    "post-bmcr=withheld post-bmsr-first=withheld post-bmsr-second=withheld" \
    "post-anar=withheld post-anlpar=withheld" \
    "post-anlpar-nonzero=withheld" \
    "passive-macb-nsr-raw=withheld passive-macb-nsr-link=withheld" \
    "bmcr-write-performed=false mdio-man-transactions-performed=false" \
    "macb-read-performed=false macb-write-performed=false" \
    "phy-reset-or-gpio32-action=false link-forcing=false" \
    "claims-runtime-mdio-transaction=" \
    "claims-bmcr-write-executed=" \
    "claims-exactly-one-bmcr-write=" \
    "claims-phy-reset-ownership=false claims-gpio32-action=false" \
    "claims-macb-write=false claims-ncr-write=false claims-link-forcing=false" \
    "claims-ethernet-ready=false claims-packet-io=false" \
    "claims-networking=false claims-sockets=false claims-ssh=false" \
    "claims-phase-12-2=false claims-phase-transition=false" \
    "classification=no-mdio-no-macb-phy1-autoneg-restart-control"; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing PHY1 autoneg restart control string: $required" >&2
        exit 1
    fi
done

if [ -n "$CAPTURE_NONCE" ]; then
    if ! grep -Fq -- " capture-nonce=" "$kernel_strings"; then
        echo "kernel image missing PHY1 autoneg restart control capture nonce label" >&2
        exit 1
    fi
    if ! grep -Fq -- "$CAPTURE_NONCE" "$kernel_strings"; then
        echo "kernel image missing PHY1 autoneg restart control capture nonce: $CAPTURE_NONCE" >&2
        exit 1
    fi
fi

for forbidden in \
    "TALOS: rp1-ethernet-phy1-autoneg-restart-candidate" \
    "before-guarded-corrected-target-bmcr-autoneg-restart" \
    "target=corrected-target-clause22-phy1-bmcr-autoneg-restart" \
    "bmcr-write-performed=true" \
    "mdio-man-transactions-performed=true" \
    "macb-read-performed=true" \
    "macb-write-performed=true" \
    "claims-bmcr-write-executed=true" \
    "claims-exactly-one-bmcr-write=true"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden PHY1 autoneg restart control string: $forbidden" >&2
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
printf 'forbidden_phy1_autoneg_restart_control_runtime_strings_absent=true\n'
