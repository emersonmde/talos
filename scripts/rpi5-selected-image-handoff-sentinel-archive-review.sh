#!/bin/sh
set -eu

if [ "$#" -ne 1 ]; then
    echo "usage: $0 <talos-rpi5-boot.tar.gz>" >&2
    exit 2
fi

ARCHIVE="$1"
REQUIRED_MARKER="TALOS: selected-image-handoff-sentinel-v16"

tmp_root="${TMPDIR:-target/tmp}"
mkdir -p "$tmp_root"
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-handoff-sentinel-review.XXXXXX")"
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
    echo "missing handoff sentinel marker in kernel_2712.img: $REQUIRED_MARKER" >&2
    exit 1
fi

if grep -Fq "TALOS: minimal-entry-control-ready" "$work_dir/kernel_2712.strings"; then
    echo "handoff sentinel archive unexpectedly contains the later minimal-entry marker" >&2
    exit 1
fi

printf 'handoff_sentinel=ready\n'
printf 'required_marker=%s\n' "$REQUIRED_MARKER"
printf 'archive=%s\n' "$ARCHIVE"
