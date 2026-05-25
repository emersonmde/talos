#!/bin/sh
set -eu

TALOS_RPI5_REMOTE_WAKEUP_REQUEST_PROOF=1 ./scripts/rpi5-image.sh "$@" >/dev/null

img_file="target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712-remote-wakeup-request.img"
cp target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712.img "$img_file"
printf '%s\n' "$img_file"
