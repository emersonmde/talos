#!/bin/sh
set -eu

TALOS_RPI5_TEXT_BOOT_NEXT_BRANCH_DIAGNOSTIC=1 \
    cargo -Zjson-target-spec build --target targets/aarch64-talos-rpi5-bcm2712.json "$@"

ELF_FILE="target/aarch64-talos-rpi5-bcm2712/debug/talos"
IMG_FILE="target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712-text-boot-next-branch-diagnostic.img"

rust-objcopy -O binary "$ELF_FILE" "$IMG_FILE"

if [ ! -s "$IMG_FILE" ]; then
    echo "text boot next-branch diagnostic image is empty: $IMG_FILE" >&2
    exit 1
fi

printf '%s\n' "$IMG_FILE"
