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

rm -rf "$OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR"

cp "$SOURCE_DIR/config.txt" "$OUTPUT_DIR/config.txt"
cp "$SOURCE_DIR/cmdline.txt" "$OUTPUT_DIR/cmdline.txt"
cp "$SOURCE_DIR/bcm2712-rpi-5-b.dtb" "$OUTPUT_DIR/bcm2712-rpi-5-b.dtb"

sed -i 's/earlycon=pl011,mmio32,0x1c00030000/earlycon=pl011,mmio32,0x1f00030000/g' "$OUTPUT_DIR/cmdline.txt"
sed -i '/^dtoverlay=uart0-pi5$/d' "$OUTPUT_DIR/config.txt"
sed -i '/^kernel_address=/d' "$OUTPUT_DIR/config.txt"

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

IMG="$(./scripts/rpi5-translation-fault-diagnostic-image.sh)"
cp "$IMG" "$OUTPUT_DIR/kernel_2712.img"
cp "$IMG" "$OUTPUT_DIR/kernel8.img"

serial_prefix="da591740"
prefix_dir="$OUTPUT_DIR/$serial_prefix"
mkdir -p "$prefix_dir"
cp "$OUTPUT_DIR/config.txt" "$prefix_dir/config.txt"
cp "$OUTPUT_DIR/cmdline.txt" "$prefix_dir/cmdline.txt"
cp "$OUTPUT_DIR/bcm2712-rpi-5-b.dtb" "$prefix_dir/bcm2712-rpi-5-b.dtb"
cp "$IMG" "$prefix_dir/kernel_2712.img"
cp "$IMG" "$prefix_dir/kernel8.img"

if [ -d "$OUTPUT_DIR/overlays" ]; then
    mkdir -p "$prefix_dir/overlays"
    for overlay in "$OUTPUT_DIR"/overlays/*; do
        if [ -f "$overlay" ]; then
            cp "$overlay" "$prefix_dir/overlays/"
        fi
    done
fi

if [ -f "$OUTPUT_DIR/fixup4.dat" ]; then
    cp "$OUTPUT_DIR/fixup4.dat" "$prefix_dir/fixup4.dat"
fi

if [ -f "$OUTPUT_DIR/start4.elf" ]; then
    cp "$OUTPUT_DIR/start4.elf" "$prefix_dir/start4.elf"
fi

find "$OUTPUT_DIR" -type f | sort
