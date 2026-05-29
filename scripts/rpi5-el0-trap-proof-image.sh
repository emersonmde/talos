#!/bin/sh
set -eu

TALOS_BOOT_SCENARIO=rpi5_el0_trap_proof \
    ./scripts/rpi5-image.sh "$@" >/dev/null

img_file="target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712-el0-trap-proof.img"
cp target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712.img "$img_file"
printf '%s\n' "$img_file"
