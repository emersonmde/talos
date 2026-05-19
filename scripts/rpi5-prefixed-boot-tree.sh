#!/bin/sh
set -eu

if [ "$#" -ne 2 ] && [ "$#" -ne 3 ]; then
    echo "usage: $0 <pi-firmware-boot-source> <output-boot-tree> [serial-prefix]" >&2
    exit 2
fi

SOURCE_DIR="$1"
OUTPUT_DIR="$2"
SERIAL_PREFIX="da591740"
if [ "$#" -eq 3 ]; then
    SERIAL_PREFIX="$3"
fi

case "$SERIAL_PREFIX" in
    ""|*/*|.*|*..*)
        echo "unsafe serial prefix: $SERIAL_PREFIX" >&2
        exit 1
        ;;
esac

./scripts/rpi5-boot-tree.sh "$SOURCE_DIR" "$OUTPUT_DIR"

prefix_dir="$OUTPUT_DIR/$SERIAL_PREFIX"
mkdir -p "$prefix_dir"

for path in config.txt cmdline.txt bcm2712-rpi-5-b.dtb kernel_2712.img kernel8.img; do
    cp "$OUTPUT_DIR/$path" "$prefix_dir/$path"
done

if [ -d "$OUTPUT_DIR/overlays" ]; then
    mkdir -p "$prefix_dir/overlays"
    for overlay in "$OUTPUT_DIR"/overlays/*; do
        if [ -f "$overlay" ]; then
            cp "$overlay" "$prefix_dir/overlays/"
        fi
    done
fi

find "$OUTPUT_DIR" -type f | sort
