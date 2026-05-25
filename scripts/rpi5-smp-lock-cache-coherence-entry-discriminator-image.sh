#!/bin/sh
set -eu

TALOS_RPI5_SMP_LOCK_CACHE_COHERENCE_PROOF=1 \
TALOS_RPI5_SMP_LOCK_CACHE_COHERENCE_ENTRY_DISCRIMINATOR=1 \
    ./scripts/rpi5-image.sh "$@" >/dev/null

img_file="target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712-smp-lock-cache-coherence-entry-discriminator.img"
cp target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712.img "$img_file"
printf '%s\n' "$img_file"
