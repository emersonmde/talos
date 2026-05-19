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

if [ ! -d "$SOURCE_DIR" ]; then
    echo "boot source does not exist: $SOURCE_DIR" >&2
    exit 1
fi

for path in bcm2712-rpi-5-b.dtb overlays/overlay_map.dtb overlays/bcm2712d0.dtbo; do
    if [ ! -f "$SOURCE_DIR/$path" ]; then
        echo "firmware source missing required file: $path" >&2
        exit 1
    fi
done

RAW_IMG="$(./scripts/rpi5-loader-diagnostic.sh)"

rm -rf "$OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR/overlays"

cp "$SOURCE_DIR/bcm2712-rpi-5-b.dtb" "$OUTPUT_DIR/bcm2712-rpi-5-b.dtb"
cp "$SOURCE_DIR/overlays/overlay_map.dtb" "$OUTPUT_DIR/overlays/overlay_map.dtb"
cp "$SOURCE_DIR/overlays/bcm2712d0.dtbo" "$OUTPUT_DIR/overlays/bcm2712d0.dtbo"
cp "$RAW_IMG" "$OUTPUT_DIR/kernel_2712.img"
cp "$RAW_IMG" "$OUTPUT_DIR/kernel8.img"

cat > "$OUTPUT_DIR/config.txt" <<'CONFIG'
arm_64bit=1
kernel_address=0x80000
initial_turbo=0
[pi5]
kernel=kernel_2712.img
talos_loader_diagnostic=raw-pi5-circle-config
CONFIG

cat > "$OUTPUT_DIR/cmdline.txt" <<'CMDLINE'
talos.boot=raw-pi5-circle-config
CMDLINE

prefix_dir="$OUTPUT_DIR/$SERIAL_PREFIX"
mkdir -p "$prefix_dir/overlays"
for path in config.txt cmdline.txt bcm2712-rpi-5-b.dtb kernel_2712.img kernel8.img; do
    cp "$OUTPUT_DIR/$path" "$prefix_dir/$path"
done
for overlay in "$OUTPUT_DIR"/overlays/*; do
    if [ -f "$overlay" ]; then
        cp "$overlay" "$prefix_dir/overlays/"
    fi
done

find "$OUTPUT_DIR" -type f | sort
