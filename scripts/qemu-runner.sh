#!/bin/sh
set -eu

if [ "$#" -lt 1 ]; then
    echo "usage: $0 <kernel-elf> [args...]" >&2
    exit 2
fi

ELF_FILE="$1"
IMG_FILE="$1.img"

rust-objcopy -O binary "$ELF_FILE" "$IMG_FILE"

exec qemu-system-aarch64 \
    -M virt \
    -cpu cortex-a76 \
    -m 256M \
    -nographic \
    -serial mon:stdio \
    -semihosting-config enable=on,target=native \
    -kernel "$IMG_FILE"
