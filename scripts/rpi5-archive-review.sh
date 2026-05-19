#!/bin/sh
set -eu

if [ "$#" -ne 1 ]; then
    echo "usage: $0 <talos-rpi5-boot.tar.gz>" >&2
    exit 2
fi

ARCHIVE="$1"

if [ ! -f "$ARCHIVE" ]; then
    echo "archive does not exist: $ARCHIVE" >&2
    exit 1
fi

tmp_root="target/tmp"
mkdir -p "$tmp_root"
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-archive-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

manifest="$work_dir/manifest.txt"
tar -tzf "$ARCHIVE" | sed 's#^[.]/##; s#/$##' | sed '/^[.]$/d; /^$/d' | LC_ALL=C sort > "$manifest"

required_files="config.txt cmdline.txt bcm2712-rpi-5-b.dtb kernel_2712.img kernel8.img"
for file in $required_files; do
    if ! grep -qx "$file" "$manifest"; then
        echo "archive missing required file: $file" >&2
        exit 1
    fi
done

serial_prefix="da591740"
if grep -q "^$serial_prefix/" "$manifest"; then
    for file in $required_files; do
        if ! grep -qx "$serial_prefix/$file" "$manifest"; then
            echo "serial-prefixed boot mirror missing file: $serial_prefix/$file" >&2
            exit 1
        fi
    done
    if grep -qx "armstub8-2712.bin" "$manifest" && ! grep -qx "$serial_prefix/armstub8-2712.bin" "$manifest"; then
        echo "serial-prefixed boot mirror missing file: $serial_prefix/armstub8-2712.bin" >&2
        exit 1
    fi
fi

if grep -Eq '(^/|(^|/)\.\.?(/|$)|(^|/)\.[^/]+)' "$manifest"; then
    echo "archive contains an unsafe path" >&2
    exit 1
fi

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

if ! grep -qx 'kernel=kernel_2712.img' "$extract_dir/config.txt"; then
    echo "config.txt must select kernel_2712.img" >&2
    exit 1
fi

loader_diagnostic=false
circle_config_loader_diagnostic=false
if grep -qx 'talos_loader_diagnostic=raw-pi5' "$extract_dir/config.txt"; then
    loader_diagnostic=true
fi
if grep -qx 'talos_loader_diagnostic=asm-uart-proof' "$extract_dir/config.txt"; then
    loader_diagnostic=true
fi
if grep -qx 'talos_loader_diagnostic=asm-entry-reset-proof' "$extract_dir/config.txt"; then
    loader_diagnostic=true
fi
if grep -qx 'talos_loader_diagnostic=raw-pi5-circle-config' "$extract_dir/config.txt"; then
    loader_diagnostic=true
    circle_config_loader_diagnostic=true
fi

if [ "$circle_config_loader_diagnostic" = false ]; then
    if ! grep -qx 'enable_rp1_uart=1' "$extract_dir/config.txt"; then
        echo "config.txt must preserve RP1 UART0 for first-light serial" >&2
        exit 1
    fi

    if ! grep -qx 'pciex4_reset=0' "$extract_dir/config.txt"; then
        echo "config.txt must preserve RP1 state with pciex4_reset=0" >&2
        exit 1
    fi
fi

if ! grep -qx 'kernel_address=0x80000' "$extract_dir/config.txt"; then
    echo "config.txt must select the Circle-style Pi 5 bare-metal kernel address" >&2
    exit 1
fi

if grep -qx 'dtoverlay=uart0-pi5' "$extract_dir/config.txt"; then
    echo "config.txt should not apply the Linux uart0-pi5 overlay during bare-metal first light" >&2
    exit 1
fi

if grep -qx 'boot_ramdisk=1' "$extract_dir/config.txt"; then
    if [ ! -f "$extract_dir/boot.img" ]; then
        echo "config.txt enables boot_ramdisk=1 but archive has no boot.img" >&2
        exit 1
    fi
    if ! mdir -i "$extract_dir/boot.img" ::/config.txt ::/kernel_2712.img ::/kernel8.img >/dev/null 2>&1; then
        echo "boot.img must be a readable FAT image containing config and kernel files" >&2
        exit 1
    fi
fi

if grep -qx 'armstub=armstub8-2712.bin' "$extract_dir/config.txt"; then
    if [ ! -s "$extract_dir/armstub8-2712.bin" ]; then
        echo "config.txt selects armstub8-2712.bin but archive has no non-empty armstub" >&2
        exit 1
    fi
    if [ -d "$extract_dir/$serial_prefix" ] && [ ! -s "$extract_dir/$serial_prefix/armstub8-2712.bin" ]; then
        echo "serial-prefixed config selects armstub8-2712.bin but mirror has no non-empty armstub" >&2
        exit 1
    fi
fi

if ! cmp -s "$extract_dir/kernel_2712.img" "$extract_dir/kernel8.img"; then
    echo "kernel_2712.img and kernel8.img should match during first-light fallback testing" >&2
    exit 1
fi

if [ -d "$extract_dir/$serial_prefix" ]; then
    for file in kernel_2712.img kernel8.img config.txt cmdline.txt bcm2712-rpi-5-b.dtb; do
        if ! cmp -s "$extract_dir/$file" "$extract_dir/$serial_prefix/$file"; then
            echo "serial-prefixed boot mirror differs from root file: $file" >&2
            exit 1
        fi
    done
    if [ -f "$extract_dir/armstub8-2712.bin" ] && ! cmp -s "$extract_dir/armstub8-2712.bin" "$extract_dir/$serial_prefix/armstub8-2712.bin"; then
        echo "serial-prefixed boot mirror differs from root file: armstub8-2712.bin" >&2
        exit 1
    fi
fi

image_size="$(wc -c < "$extract_dir/kernel_2712.img" | tr -d ' ')"
text_offset="$(od -An -tu8 -j8 -N8 "$extract_dir/kernel_2712.img" | tr -d ' ')"
header_image_size="$(od -An -tu8 -j16 -N8 "$extract_dir/kernel_2712.img" | tr -d ' ')"
flags="$(od -An -tu8 -j24 -N8 "$extract_dir/kernel_2712.img" | tr -d ' ')"
magic="$(dd if="$extract_dir/kernel_2712.img" bs=1 skip=56 count=4 2>/dev/null)"

if [ "$loader_diagnostic" = false ]; then
    if [ "$text_offset" != "0" ]; then
        echo "unexpected arm64 Image text offset: $text_offset" >&2
        exit 1
    fi

    if [ "$header_image_size" != "$image_size" ]; then
        echo "arm64 Image header size mismatch: header=$header_image_size file=$image_size" >&2
        exit 1
    fi

    if [ "$flags" != "12" ]; then
        echo "unexpected arm64 Image flags: $flags" >&2
        exit 1
    fi

    if [ "$magic" != "ARMd" ]; then
        echo "arm64 Image magic missing at header offset 56" >&2
        exit 1
    fi
elif { grep -qx 'talos_loader_diagnostic=asm-uart-proof' "$extract_dir/config.txt" || grep -qx 'talos_loader_diagnostic=asm-entry-reset-proof' "$extract_dir/config.txt"; } && [ "$magic" = "ARMd" ]; then
    if [ "$header_image_size" != "$image_size" ]; then
        echo "asm proof Image header size mismatch: header=$header_image_size file=$image_size" >&2
        exit 1
    fi

    if [ "$flags" != "12" ]; then
        echo "unexpected asm proof Image flags: $flags" >&2
        exit 1
    fi
fi

printf 'archive=%s\n' "$ARCHIVE"
printf 'sha256=%s\n' "$(sha256sum "$ARCHIVE" | awk '{print $1}')"
printf 'file_count=%s\n' "$(wc -l < "$manifest" | tr -d ' ')"
printf 'kernel_size=%s\n' "$image_size"
printf 'header_image_size=%s\n' "$header_image_size"
printf 'text_offset=%s\n' "$text_offset"
printf 'flags=%s\n' "$flags"
printf 'loader_diagnostic=%s\n' "$loader_diagnostic"
printf 'circle_config_loader_diagnostic=%s\n' "$circle_config_loader_diagnostic"
printf 'manifest:\n'
sed 's/^/  /' "$manifest"
