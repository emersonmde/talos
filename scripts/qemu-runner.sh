#!/bin/sh
set -eu

if [ "$#" -lt 1 ]; then
    echo "usage: $0 <kernel-elf> [args...]" >&2
    exit 2
fi

ELF_FILE="$1"
IMG_FILE="$1.img"

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
. "$script_dir/objcopy-tool.sh"
. "$script_dir/qemu-tool.sh"

"$objcopy_tool" -O binary "$ELF_FILE" "$IMG_FILE"

exec "$qemu_tool" \
    -M virt \
    -cpu cortex-a76 \
    -m 256M \
    -nographic \
    -serial mon:stdio \
    -semihosting-config enable=on,target=native \
    -kernel "$IMG_FILE"
