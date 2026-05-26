#!/bin/sh
set -eu

TALOS_BOOT_SCENARIO=rpi5_uart10_polling_rx ./scripts/rpi5-image.sh "$@" >/dev/null

img_file="target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712-uart10-rx-diagnostic.img"
cp target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712.img "$img_file"
printf '%s\n' "$img_file"
