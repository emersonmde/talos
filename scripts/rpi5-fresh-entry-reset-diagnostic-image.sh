#!/bin/sh
set -eu

if [ "$#" -gt 1 ]; then
    echo "usage: $0 [fresh-label]" >&2
    exit 2
fi

for name in $(env | sed -n 's/^\(TALOS_RPI5_[A-Za-z0-9_]*\)=.*/\1/p'); do
    unset "$name"
done

label="${1:-TALOS: fresh entry reset}"

if ! printf '%s' "$label" | LC_ALL=C grep -Eq '^[A-Za-z0-9: ./_-]+$'; then
    echo "fresh label must be printable ASCII without shell metacharacters" >&2
    exit 2
fi

fresh_label="$(printf '%s\r\n' "$label")"

TALOS_RPI5_FRESH_ENTRY_RESET_DIAGNOSTIC=1 \
TALOS_RPI5_FRESH_ENTRY_LABEL="$fresh_label" \
    cargo -Zjson-target-spec build --target targets/aarch64-talos-rpi5-bcm2712.json

ELF_FILE="target/aarch64-talos-rpi5-bcm2712/debug/talos"
IMG_FILE="target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712-fresh-entry-reset-diagnostic.img"

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
. "$script_dir/objcopy-tool.sh"

"$objcopy_tool" -O binary "$ELF_FILE" "$IMG_FILE"

image_size="$(wc -c < "$IMG_FILE" | tr -d ' ')"
header_image_size="$(od -An -tu8 -j16 -N8 "$IMG_FILE" | tr -d ' ')"

if [ "$header_image_size" != "$image_size" ]; then
    echo "arm64 Image header size mismatch: header=$header_image_size file=$image_size" >&2
    exit 1
fi

printf '%s\n' "$IMG_FILE"
