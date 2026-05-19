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

if ! grep -qx 'enable_rp1_uart=1' "$extract_dir/config.txt"; then
    echo "config.txt must preserve RP1 UART0 for first-light serial" >&2
    exit 1
fi

if ! grep -qx 'pciex4_reset=0' "$extract_dir/config.txt"; then
    echo "config.txt must preserve RP1 state with pciex4_reset=0" >&2
    exit 1
fi

if grep -qx 'dtoverlay=uart0-pi5' "$extract_dir/config.txt"; then
    echo "config.txt should not apply the Linux uart0-pi5 overlay during bare-metal first light" >&2
    exit 1
fi

if ! cmp -s "$extract_dir/kernel_2712.img" "$extract_dir/kernel8.img"; then
    echo "kernel_2712.img and kernel8.img should match during first-light fallback testing" >&2
    exit 1
fi

image_size="$(wc -c < "$extract_dir/kernel_2712.img" | tr -d ' ')"
text_offset="$(od -An -tu8 -j8 -N8 "$extract_dir/kernel_2712.img" | tr -d ' ')"
header_image_size="$(od -An -tu8 -j16 -N8 "$extract_dir/kernel_2712.img" | tr -d ' ')"
magic="$(dd if="$extract_dir/kernel_2712.img" bs=1 skip=56 count=4 2>/dev/null)"

if [ "$text_offset" != "2097152" ]; then
    echo "unexpected arm64 Image text offset: $text_offset" >&2
    exit 1
fi

if [ "$header_image_size" != "$image_size" ]; then
    echo "arm64 Image header size mismatch: header=$header_image_size file=$image_size" >&2
    exit 1
fi

if [ "$magic" != "ARMd" ]; then
    echo "arm64 Image magic missing at header offset 56" >&2
    exit 1
fi

printf 'archive=%s\n' "$ARCHIVE"
printf 'sha256=%s\n' "$(sha256sum "$ARCHIVE" | awk '{print $1}')"
printf 'file_count=%s\n' "$(wc -l < "$manifest" | tr -d ' ')"
printf 'kernel_size=%s\n' "$image_size"
printf 'header_image_size=%s\n' "$header_image_size"
printf 'text_offset=%s\n' "$text_offset"
printf 'manifest:\n'
sed 's/^/  /' "$manifest"
