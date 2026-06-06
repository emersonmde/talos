#!/bin/sh
set -eu

TALOS_BOOT_SCENARIO=rpi5_rp1_uart0_fr_shaped_no_mmio_marker \
    ./scripts/rpi5-image.sh "$@" >/dev/null

img_file="target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712-rp1-uart0-fr-shaped-no-mmio-marker.img"
cp target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712.img "$img_file"
printf '%s\n' "$img_file"
