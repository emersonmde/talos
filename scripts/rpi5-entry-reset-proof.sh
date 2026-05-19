#!/bin/sh
set -eu

out_dir="target/aarch64-talos-rpi5-bcm2712/debug"
obj_file="$out_dir/rpi5_entry_reset_proof.o"
elf_file="$out_dir/rpi5_entry_reset_proof.elf"
bin_file="$out_dir/rpi5_entry_reset_proof.img"

mkdir -p "$out_dir"

clang -target aarch64-none-elf -ffreestanding -fno-stack-protector \
    -c src/arch/aarch64/rpi5_entry_reset_proof.S -o "$obj_file"

ld.lld -Ttext=0x80000 -nostdlib "$obj_file" -o "$elf_file"
objcopy_tool="${OBJCOPY:-}"
if [ -z "$objcopy_tool" ]; then
    if command -v llvm-objcopy >/dev/null 2>&1; then
        objcopy_tool="llvm-objcopy"
    elif command -v llvm-objcopy-14 >/dev/null 2>&1; then
        objcopy_tool="llvm-objcopy-14"
    else
        objcopy_tool="objcopy"
    fi
fi

"$objcopy_tool" -O binary "$elf_file" "$bin_file"

if [ ! -s "$bin_file" ]; then
    echo "entry reset proof image is empty: $bin_file" >&2
    exit 1
fi

printf '%s\n' "$bin_file"
