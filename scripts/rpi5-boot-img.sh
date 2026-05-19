#!/bin/sh
set -eu

if [ "$#" -ne 2 ]; then
    echo "usage: $0 <boot-tree-dir> <output-boot.img>" >&2
    exit 2
fi

BOOT_TREE="$1"
BOOT_IMG="$2"

if [ ! -d "$BOOT_TREE" ]; then
    echo "boot tree does not exist: $BOOT_TREE" >&2
    exit 1
fi

for tool in mformat mcopy mmd mdir; do
    if ! command -v "$tool" >/dev/null 2>&1; then
        echo "required tool not found: $tool" >&2
        exit 1
    fi
done

for file in config.txt cmdline.txt bcm2712-rpi-5-b.dtb kernel_2712.img kernel8.img; do
    if [ ! -f "$BOOT_TREE/$file" ]; then
        echo "boot tree is missing required file: $file" >&2
        exit 1
    fi
done

rm -f "$BOOT_IMG"
mkdir -p "$(dirname "$BOOT_IMG")"

tree_kib="$(du -sk "$BOOT_TREE" | awk '{print $1}')"
image_kib=$((tree_kib + 8192))
if [ "$image_kib" -lt 65536 ]; then
    image_kib=65536
fi

truncate -s "${image_kib}K" "$BOOT_IMG"
mformat -i "$BOOT_IMG" -F -v TALOSBOOT ::

mcopy -i "$BOOT_IMG" "$BOOT_TREE/config.txt" "$BOOT_TREE/cmdline.txt" \
    "$BOOT_TREE/bcm2712-rpi-5-b.dtb" "$BOOT_TREE/kernel_2712.img" \
    "$BOOT_TREE/kernel8.img" ::

if [ -d "$BOOT_TREE/overlays" ]; then
    mmd -i "$BOOT_IMG" ::/overlays
    for overlay in "$BOOT_TREE"/overlays/*; do
        if [ -f "$overlay" ]; then
            mcopy -i "$BOOT_IMG" "$overlay" ::/overlays/
        fi
    done
fi

mdir -i "$BOOT_IMG" -/ ::
printf '%s\n' "$BOOT_IMG"
