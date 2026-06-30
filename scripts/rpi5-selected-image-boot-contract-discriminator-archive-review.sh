#!/bin/sh
set -eu

if [ "$#" -ne 1 ]; then
    echo "usage: $0 <talos-rpi5-boot.tar.gz>" >&2
    exit 2
fi

ARCHIVE="$1"
REQUIRED_MARKER="TALOS: boot-contract-v18"
FORBIDDEN_V16_MARKER="TALOS: selected-image-handoff-sentinel-v16"

tmp_root="${TMPDIR:-target/tmp}"
mkdir -p "$tmp_root"
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-boot-contract-v18-review.XXXXXX")"
cleanup() {
    rm -rf "$work_dir"
}
trap cleanup EXIT INT TERM

./scripts/rpi5-archive-review.sh "$ARCHIVE" >/dev/null
tar -xzf "$ARCHIVE" -C "$work_dir"

if ! cmp -s "$work_dir/kernel_2712.img" "$work_dir/da591740/kernel_2712.img"; then
    echo "serial-prefixed selected kernel differs from root kernel_2712.img" >&2
    exit 1
fi

strings "$work_dir/kernel_2712.img" > "$work_dir/kernel_2712.strings"
if ! grep -Fq "$REQUIRED_MARKER" "$work_dir/kernel_2712.strings"; then
    echo "missing boot contract marker in kernel_2712.img: $REQUIRED_MARKER" >&2
    exit 1
fi

for forbidden in \
    "$FORBIDDEN_V16_MARKER" \
    "TALOS: minimal-entry-control-ready" \
    "TALOS: ssh-service-smoltcp-runtime"; do
    if grep -Fq "$forbidden" "$work_dir/kernel_2712.strings"; then
        echo "boot contract discriminator archive contains forbidden marker: $forbidden" >&2
        exit 1
    fi
done

image_size="$(wc -c < "$work_dir/kernel_2712.img" | tr -d ' ')"
text_offset="$(od -An -tu8 -j8 -N8 "$work_dir/kernel_2712.img" | tr -d ' ')"
header_image_size="$(od -An -tu8 -j16 -N8 "$work_dir/kernel_2712.img" | tr -d ' ')"
flags="$(od -An -tu8 -j24 -N8 "$work_dir/kernel_2712.img" | tr -d ' ')"

printf 'boot_contract_discriminator=ready\n'
printf 'required_marker=%s\n' "$REQUIRED_MARKER"
printf 'archive=%s\n' "$ARCHIVE"
printf 'archive_sha256=%s\n' "$(sha256sum "$ARCHIVE" | awk '{print $1}')"
printf 'kernel_2712_sha256=%s\n' "$(sha256sum "$work_dir/kernel_2712.img" | awk '{print $1}')"
printf 'kernel_2712_size=%s\n' "$image_size"
printf 'header_image_size=%s\n' "$header_image_size"
printf 'text_offset=%s\n' "$text_offset"
printf 'flags=%s\n' "$flags"
printf 'forbidden_v16_and_later_markers_absent=true\n'
