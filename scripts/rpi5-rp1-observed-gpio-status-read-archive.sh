#!/bin/sh
set -eu

if [ "$#" -ne 2 ]; then
    echo "usage: $0 <pi-firmware-boot-source> <output-archive.tar.gz>" >&2
    exit 2
fi

SOURCE_DIR="$1"
ARCHIVE="$2"

tmp_root="target/tmp"
mkdir -p "$tmp_root"
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-observed-gpio-status-read.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

boot_tree="$work_dir/boot-tree"
./scripts/rpi5-rp1-observed-gpio-status-read-boot-tree.sh "$SOURCE_DIR" "$boot_tree" >/dev/null

mkdir -p "$(dirname "$ARCHIVE")"
tar -czf "$ARCHIVE" -C "$boot_tree" .
printf '%s\n' "$ARCHIVE"
