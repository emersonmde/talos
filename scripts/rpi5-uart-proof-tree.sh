#!/bin/sh
set -eu

if [ "$#" -ne 2 ]; then
    echo "usage: $0 <pi-firmware-boot-source> <output-boot-tree>" >&2
    exit 2
fi

SOURCE_DIR="$1"
OUTPUT_DIR="$2"

./scripts/rpi5-boot-tree.sh "$SOURCE_DIR" "$OUTPUT_DIR"

UART_PROOF="$(./scripts/rpi5-uart-proof.sh)"
cp "$UART_PROOF" "$OUTPUT_DIR/kernel_2712.img"
cp "$UART_PROOF" "$OUTPUT_DIR/kernel8.img"

sed -i '/^talos_loader_diagnostic=/d' "$OUTPUT_DIR/config.txt"
printf '%s\n' 'talos_loader_diagnostic=asm-uart-proof' >> "$OUTPUT_DIR/config.txt"

find "$OUTPUT_DIR" -type f | sort
