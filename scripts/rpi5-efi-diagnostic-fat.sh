#!/bin/sh
set -eu

if [ "$#" -ne 1 ]; then
    echo "usage: $0 <output-fat-image>" >&2
    exit 2
fi

out_img="$1"
efi_file="$(./scripts/rpi5-efi-diagnostic.sh)"

mkdir -p "$(dirname "$out_img")"
rm -f "$out_img"
truncate -s 16M "$out_img"
mformat -i "$out_img" -v TALOS_EFI ::

mmd -i "$out_img" ::/EFI ::/EFI/BOOT
mcopy -i "$out_img" "$efi_file" ::/EFI/BOOT/BOOTAA64.EFI
printf 'fs0:\\EFI\\BOOT\\BOOTAA64.EFI\r\n' > target/rpi5-efi-startup.nsh
mcopy -i "$out_img" target/rpi5-efi-startup.nsh ::/startup.nsh

mdir -i "$out_img" ::/EFI/BOOT/BOOTAA64.EFI ::/startup.nsh >/dev/null
printf '%s\n' "$out_img"
