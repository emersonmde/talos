#!/bin/sh
set -eu

TALOS_RPI5_REMOTE_WAKEUP_REQUEST_PROOF=1 \
TALOS_RPI5_REMOTE_WAKE_TO_LOCAL_RUNNABLE_PROOF=1 \
    ./scripts/rpi5-image.sh "$@" >/dev/null

img_file="target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712-remote-wake-to-local-runnable.img"
cp target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712.img "$img_file"
printf '%s\n' "$img_file"
