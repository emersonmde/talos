#!/bin/sh
set -eu

out_dir="target/aarch64-talos-rpi5-bcm2712/debug"
obj_file="$out_dir/rpi5_efi_diagnostic.obj"
efi_file="$out_dir/talos-rpi5-efi-diagnostic.efi"

mkdir -p "$out_dir"

clang -target aarch64-pc-win32 -ffreestanding -fno-stack-protector \
    -c src/arch/aarch64/rpi5_efi_diagnostic.S -o "$obj_file"

lld-link /subsystem:efi_application /entry:EfiMain /nodefaultlib \
    /out:"$efi_file" "$obj_file"

if [ ! -s "$efi_file" ]; then
    echo "EFI diagnostic image is empty: $efi_file" >&2
    exit 1
fi

printf '%s\n' "$efi_file"
