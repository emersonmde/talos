#!/bin/sh
set -eu

if [ "$#" -ne 2 ]; then
    echo "usage: $0 <pi-firmware-boot-source> <output-boot-tree>" >&2
    exit 2
fi

SOURCE_DIR="$1"
OUTPUT_DIR="$2"

if [ ! -d "$SOURCE_DIR" ]; then
    echo "boot source does not exist: $SOURCE_DIR" >&2
    exit 1
fi

for file in config.txt cmdline.txt bcm2712-rpi-5-b.dtb; do
    if [ ! -f "$SOURCE_DIR/$file" ]; then
        echo "boot source is missing required file: $file" >&2
        exit 1
    fi
done

KERNEL_IMG="$(./scripts/rpi5-image.sh)"

rm -rf "$OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR"

cp "$SOURCE_DIR/config.txt" "$OUTPUT_DIR/config.txt"
cp "$SOURCE_DIR/cmdline.txt" "$OUTPUT_DIR/cmdline.txt"
cp "$SOURCE_DIR/bcm2712-rpi-5-b.dtb" "$OUTPUT_DIR/bcm2712-rpi-5-b.dtb"
cp "$KERNEL_IMG" "$OUTPUT_DIR/kernel_2712.img"
cp "$KERNEL_IMG" "$OUTPUT_DIR/kernel8.img"

# Keep diagnostic command-line hints aligned with the firmware-preserved RP1 UART0
# MMIO address used by Talos early boot.
sed -i 's/earlycon=pl011,mmio32,0x1f00030000/earlycon=pl011,mmio32,0x1c00030000/g' "$OUTPUT_DIR/cmdline.txt"

# The first-light kernel is bare metal and writes the firmware-preserved RP1
# UART0 directly. Avoid applying the Linux UART overlay before Talos entry;
# it is useful for Linux, but it adds an unnecessary firmware/device-tree step
# to the earliest hardware diagnostic path.
sed -i '/^dtoverlay=uart0-pi5$/d' "$OUTPUT_DIR/config.txt"

if [ -d "$SOURCE_DIR/overlays" ]; then
    mkdir -p "$OUTPUT_DIR/overlays"
    for overlay in overlay_map.dtb bcm2712d0.dtbo uart0-pi5.dtbo; do
        if [ -f "$SOURCE_DIR/overlays/$overlay" ]; then
            cp "$SOURCE_DIR/overlays/$overlay" "$OUTPUT_DIR/overlays/$overlay"
        fi
    done
fi

if [ -f "$SOURCE_DIR/fixup4.dat" ]; then
    cp "$SOURCE_DIR/fixup4.dat" "$OUTPUT_DIR/fixup4.dat"
fi

if [ -f "$SOURCE_DIR/start4.elf" ]; then
    cp "$SOURCE_DIR/start4.elf" "$OUTPUT_DIR/start4.elf"
fi

find "$OUTPUT_DIR" -type f | sort
