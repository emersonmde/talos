#!/bin/sh
set -eu

if [ "$#" -ne 2 ]; then
    echo "usage: $0 <pi-firmware-boot-source> <output-boot-tree>" >&2
    exit 2
fi

SOURCE_DIR="$1"
OUTPUT_DIR="$2"
SERIAL_PREFIX="da591740"

./scripts/rpi5-boot-tree.sh "$SOURCE_DIR" "$OUTPUT_DIR"

IMG="$(./scripts/rpi5-rust-entry-diagnostic-image.sh)"
cp "$IMG" "$OUTPUT_DIR/kernel_2712.img"
cp "$IMG" "$OUTPUT_DIR/kernel8.img"

printf '%s\n' 'talos_loader_diagnostic=rust-entry-reset-diagnostic' >> "$OUTPUT_DIR/config.txt"

mkdir -p "$OUTPUT_DIR/$SERIAL_PREFIX"
for path in config.txt cmdline.txt bcm2712-rpi-5-b.dtb kernel_2712.img kernel8.img; do
    cp "$OUTPUT_DIR/$path" "$OUTPUT_DIR/$SERIAL_PREFIX/$path"
done

if [ -d "$OUTPUT_DIR/overlays" ]; then
    mkdir -p "$OUTPUT_DIR/$SERIAL_PREFIX/overlays"
    for overlay in "$OUTPUT_DIR"/overlays/*; do
        if [ -f "$overlay" ]; then
            cp "$overlay" "$OUTPUT_DIR/$SERIAL_PREFIX/overlays/"
        fi
    done
fi

find "$OUTPUT_DIR" -type f | sort
