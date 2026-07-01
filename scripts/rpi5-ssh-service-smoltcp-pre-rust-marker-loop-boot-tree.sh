#!/bin/sh
set -eu

if [ "$#" -ne 2 ]; then
    echo "usage: $0 <pi-firmware-boot-source> <output-dir>" >&2
    exit 2
fi

: "${TALOS_CAPTURE_NONCE:=runtime-marker-route-static}"
export TALOS_CAPTURE_NONCE
export TALOS_BOOT_SCENARIO=rpi5_ssh_service_smoltcp_pre_rust_marker_loop

./scripts/rpi5-boot-tree.sh "$1" "$2" >/dev/null

serial_prefix="da591740"
mkdir -p "$2/$serial_prefix"
for file in config.txt cmdline.txt bcm2712-rpi-5-b.dtb kernel_2712.img kernel8.img; do
    cp "$2/$file" "$2/$serial_prefix/$file"
done

if [ -d "$2/overlays" ]; then
    mkdir -p "$2/$serial_prefix/overlays"
    find "$2/overlays" -maxdepth 1 -type f -exec cp {} "$2/$serial_prefix/overlays/" \;
fi

find "$2" -type f | sort
