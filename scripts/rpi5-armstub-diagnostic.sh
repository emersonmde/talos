#!/bin/sh
set -eu

out_dir="target/aarch64-talos-rpi5-bcm2712/debug"
obj_file="$out_dir/rpi5_armstub.o"
elf_file="$out_dir/armstub8-2712.elf"
bin_file="$out_dir/armstub8-2712.bin"

mkdir -p "$out_dir"

clang -target aarch64-none-elf -ffreestanding -fno-stack-protector \
    -c src/arch/aarch64/rpi5_armstub.S -o "$obj_file"

rust-lld -flavor gnu --image-base=0 -Ttext=0x0 -nostdlib "$obj_file" -o "$elf_file"
rust-objcopy -O binary "$elf_file" "$bin_file"

if [ ! -s "$bin_file" ]; then
    echo "armstub diagnostic binary is empty: $bin_file" >&2
    exit 1
fi

printf '%s\n' "$bin_file"
