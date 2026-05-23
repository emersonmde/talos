#!/bin/sh
set -eu

for name in $(env | sed -n 's/^\(TALOS_RPI5_[A-Za-z0-9_]*\)=.*/\1/p'); do
    unset "$name"
done

TALOS_RPI5_PHASE_LADDER_DIAGNOSTIC=1 \
TALOS_RPI5_PHASE_P0_RESET_DIAGNOSTIC=1 \
    cargo -Zjson-target-spec build --target targets/aarch64-talos-rpi5-bcm2712.json "$@"

ELF_FILE="target/aarch64-talos-rpi5-bcm2712/debug/talos"
IMG_FILE="target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712-phase-p0-reset-diagnostic.img"

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
. "$script_dir/objcopy-tool.sh"

"$objcopy_tool" -O binary "$ELF_FILE" "$IMG_FILE"

image_size="$(wc -c < "$IMG_FILE" | tr -d ' ')"
header_image_size="$(od -An -tu8 -j16 -N8 "$IMG_FILE" | tr -d ' ')"

if [ "$header_image_size" != "$image_size" ]; then
    echo "arm64 Image header size mismatch: header=$header_image_size file=$image_size" >&2
    exit 1
fi

printf '%s\n' "$IMG_FILE"
