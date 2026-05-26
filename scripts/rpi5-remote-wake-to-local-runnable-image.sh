#!/bin/sh
set -eu

TALOS_BOOT_SCENARIO=rpi5_remote_wake_to_local_runnable \
    ./scripts/rpi5-image.sh "$@" >/dev/null

img_file="target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712-remote-wake-to-local-runnable.img"
cp target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712.img "$img_file"
printf '%s\n' "$img_file"
