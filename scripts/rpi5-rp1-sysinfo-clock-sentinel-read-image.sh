#!/bin/sh
set -eu

TALOS_BOOT_SCENARIO=rpi5_rp1_sysinfo_clock_sentinel_read \
    ./scripts/rpi5-image.sh "$@" >/dev/null

img_file="target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712-rp1-sysinfo-clock-sentinel-read.img"
cp target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712.img "$img_file"
printf '%s\n' "$img_file"
