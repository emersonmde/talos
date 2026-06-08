#!/bin/sh
set -eu

if [ -n "${TALOS_CAPTURE_NONCE:-}" ]; then
    case "$TALOS_CAPTURE_NONCE" in
        *[!A-Za-z0-9_.:-]*)
            echo "TALOS_CAPTURE_NONCE may contain only A-Z, a-z, 0-9, _, ., :, and -" >&2
            exit 2
            ;;
    esac
    if [ "${#TALOS_CAPTURE_NONCE}" -gt 64 ]; then
        echo "TALOS_CAPTURE_NONCE must be 64 characters or fewer" >&2
        exit 2
    fi
fi

TALOS_BOOT_SCENARIO=rpi5_rp1_observed_gpio_status_read \
    ./scripts/rpi5-image.sh "$@" >/dev/null

img_file="target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712-rp1-observed-gpio-status-read.img"
cp target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712.img "$img_file"
printf '%s\n' "$img_file"
