#!/bin/sh
set -eu

cargo -Zjson-target-spec build --target targets/aarch64-talos-rpi5-bcm2712.json "$@"

ELF_FILE="target/aarch64-talos-rpi5-bcm2712/debug/talos"
IMG_FILE="target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712.img"

rust-objcopy -O binary "$ELF_FILE" "$IMG_FILE"

image_size="$(wc -c < "$IMG_FILE" | tr -d ' ')"
header_image_size="$(od -An -tu8 -j16 -N8 "$IMG_FILE" | tr -d ' ')"

if [ "$header_image_size" != "$image_size" ]; then
    echo "arm64 Image header size mismatch: header=$header_image_size file=$image_size" >&2
    exit 1
fi

printf '%s\n' "$IMG_FILE"
