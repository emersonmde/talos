#!/bin/sh
set -eu

TALOS_BOOT_SCENARIO=rpi5_shared_scheduler_metadata \
    ./scripts/rpi5-image.sh "$@" >/dev/null

img_file="target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712-shared-scheduler-metadata.img"
cp target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712.img "$img_file"
printf '%s\n' "$img_file"
