#!/bin/sh
set -eu

if [ "$#" -ne 2 ]; then
    echo "usage: $0 <pi-firmware-boot-source> <output-boot-tree>" >&2
    exit 2
fi

SOURCE_DIR="$1"
OUTPUT_DIR="$2"

./scripts/rpi5-boot-tree.sh "$SOURCE_DIR" "$OUTPUT_DIR"

if ! grep -qx 'boot_ramdisk=1' "$OUTPUT_DIR/config.txt"; then
    printf '%s\n' 'boot_ramdisk=1' >> "$OUTPUT_DIR/config.txt"
fi

./scripts/rpi5-boot-img.sh "$OUTPUT_DIR" "$OUTPUT_DIR/boot.img" >/dev/null

find "$OUTPUT_DIR" -type f | sort
