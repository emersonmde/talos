#!/bin/sh
set -eu

TALOS_BOOT_SCENARIO=rpi5_rp1_gpio_bank_source_status_read \
    ./scripts/rpi5-image.sh "$@" >/dev/null

img_file="target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712-rp1-gpio-bank-source-status-read.img"
cp target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712.img "$img_file"
printf '%s\n' "$img_file"
