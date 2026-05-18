#!/bin/sh
set -eu

cargo -Zjson-target-spec build --target targets/aarch64-talos-rpi5-bcm2712.json "$@"

ELF_FILE="target/aarch64-talos-rpi5-bcm2712/debug/talos"
IMG_FILE="target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712.img"

rust-objcopy -O binary "$ELF_FILE" "$IMG_FILE"

printf '%s\n' "$IMG_FILE"
