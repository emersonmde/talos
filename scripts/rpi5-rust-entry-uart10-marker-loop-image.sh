#!/bin/sh
set -eu

TALOS_BOOT_SCENARIO=rpi5_rust_entry_uart10_marker_loop \
    ./scripts/rpi5-image.sh "$@" >/dev/null

img_file="target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712-rust-entry-uart10-marker-loop.img"
cp target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712.img "$img_file"
printf '%s\n' "$img_file"
