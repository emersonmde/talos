#!/bin/sh
set -eu

if [ "$#" -ne 2 ]; then
    echo "usage: $0 <pi-firmware-boot-source> <output-boot-tree>" >&2
    exit 2
fi

SOURCE_DIR="$1"
OUTPUT_DIR="$2"

./scripts/rpi5-boot-tree.sh "$SOURCE_DIR" "$OUTPUT_DIR"

ARMSTUB_BIN="$(./scripts/rpi5-armstub-diagnostic.sh)"
cp "$ARMSTUB_BIN" "$OUTPUT_DIR/armstub8-2712.bin"

if ! grep -qx 'armstub=armstub8-2712.bin' "$OUTPUT_DIR/config.txt"; then
    printf '%s\n' 'armstub=armstub8-2712.bin' >> "$OUTPUT_DIR/config.txt"
fi

find "$OUTPUT_DIR" -type f | sort
