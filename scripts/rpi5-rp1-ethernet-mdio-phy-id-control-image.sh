#!/bin/sh
set -eu

capture_nonce="$(printenv TALOS_CAPTURE_NONCE || true)"
if [ -n "$capture_nonce" ]; then
    case "$capture_nonce" in
        *[!A-Za-z0-9_.:-]*)
            echo "TALOS_CAPTURE_NONCE may contain only A-Z, a-z, 0-9, _, ., :, and -" >&2
            exit 2
            ;;
    esac
fi

TALOS_BOOT_SCENARIO=rpi5_rp1_ethernet_mdio_phy_id_no_mdio_control \
    ./scripts/rpi5-image.sh "$@" >/dev/null

img_file="target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712-rp1-ethernet-mdio-phy-id-control.img"
cp target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712.img "$img_file"
printf '%s\n' "$img_file"
