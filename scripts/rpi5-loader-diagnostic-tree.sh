#!/bin/sh
set -eu

if [ "$#" -ne 2 ]; then
    echo "usage: $0 <pi-firmware-boot-source> <output-boot-tree>" >&2
    exit 2
fi

SOURCE_DIR="$1"
OUTPUT_DIR="$2"

./scripts/rpi5-boot-tree.sh "$SOURCE_DIR" "$OUTPUT_DIR"

RAW_IMG="$(./scripts/rpi5-loader-diagnostic.sh)"
cp "$RAW_IMG" "$OUTPUT_DIR/kernel_2712.img"
cp "$RAW_IMG" "$OUTPUT_DIR/kernel8.img"

sed -i '/^talos_loader_diagnostic=/d' "$OUTPUT_DIR/config.txt"
printf '%s\n' 'talos_loader_diagnostic=raw-pi5' >> "$OUTPUT_DIR/config.txt"

find "$OUTPUT_DIR" -type f | sort
