#!/bin/sh
set -eu

TALOS_RPI5_CARGO_ASM_UART_PROOF=1 \
    cargo -Zjson-target-spec build --target targets/aarch64-talos-rpi5-bcm2712.json "$@"

ELF_FILE="target/aarch64-talos-rpi5-bcm2712/debug/talos"
IMG_FILE="target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712-cargo-asm-uart-proof.img"

rust-objcopy -O binary "$ELF_FILE" "$IMG_FILE"

if [ ! -s "$IMG_FILE" ]; then
    echo "Cargo-linked assembly UART proof image is empty: $IMG_FILE" >&2
    exit 1
fi

printf '%s\n' "$IMG_FILE"
