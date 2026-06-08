#!/bin/sh
set -eu

TALOS_BOOT_SCENARIO=rpi5_rp1_clock_adc_window_coherence_no_mmio_control \
    ./scripts/rpi5-image.sh "$@" >/dev/null

img_file="target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712-rp1-clock-adc-window-coherence-no-mmio-control.img"
cp target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712.img "$img_file"
printf '%s\n' "$img_file"
