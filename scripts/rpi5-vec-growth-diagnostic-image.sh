#!/bin/sh
set -eu

for name in $(env | sed -n 's/^\(TALOS_RPI5_[A-Za-z0-9_]*\)=.*/\1/p'); do
    if [ "$name" = "TALOS_RPI5_VEC_GROWTH_PAD_SIZE" ]; then
        continue
    fi
    unset "$name"
done

base_img="$(env TALOS_BOOT_SCENARIO=rpi5_vec_growth ./scripts/rpi5-image.sh)"
img_file="target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712-vec-growth-diagnostic.img"
target_size="${TALOS_RPI5_VEC_GROWTH_PAD_SIZE:-}"

mkdir -p "$(dirname "$img_file")"
cp "$base_img" "$img_file"

if [ -n "$target_size" ]; then
    image_size="$(wc -c < "$img_file" | tr -d ' ')"
    if [ "$image_size" -gt "$target_size" ]; then
        echo "Vec growth diagnostic image exceeds padded target: image=$image_size target=$target_size" >&2
        exit 1
    fi

    if [ "$image_size" -lt "$target_size" ]; then
        truncate -s "$target_size" "$img_file"
        perl -e 'print pack("Q<", shift)' "$target_size" |
            dd of="$img_file" bs=1 seek=16 conv=notrunc status=none
    fi
fi

printf '%s\n' "$img_file"
