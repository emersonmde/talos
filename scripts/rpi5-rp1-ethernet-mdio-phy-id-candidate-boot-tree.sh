#!/bin/sh
set -eu

if [ "$#" -ne 2 ]; then
    echo "usage: $0 <pi-firmware-boot-source> <output-dir>" >&2
    exit 2
fi

SOURCE_DIR="$1"
OUTPUT_DIR="$2"
SERIAL_PREFIX="da591740"

./scripts/rpi5-boot-tree.sh "$SOURCE_DIR" "$OUTPUT_DIR" >/dev/null

KERNEL_IMG="$(./scripts/rpi5-rp1-ethernet-mdio-phy-id-candidate-image.sh)"
cp "$KERNEL_IMG" "$OUTPUT_DIR/kernel_2712.img"
cp "$KERNEL_IMG" "$OUTPUT_DIR/kernel8.img"

mkdir -p "$OUTPUT_DIR/$SERIAL_PREFIX"
for file in config.txt cmdline.txt bcm2712-rpi-5-b.dtb kernel_2712.img kernel8.img; do
    cp "$OUTPUT_DIR/$file" "$OUTPUT_DIR/$SERIAL_PREFIX/$file"
done

if [ -d "$OUTPUT_DIR/overlays" ]; then
    mkdir -p "$OUTPUT_DIR/$SERIAL_PREFIX/overlays"
    find "$OUTPUT_DIR/overlays" -maxdepth 1 -type f -exec cp {} "$OUTPUT_DIR/$SERIAL_PREFIX/overlays/" \;
fi

printf '%s\n' "$OUTPUT_DIR"
