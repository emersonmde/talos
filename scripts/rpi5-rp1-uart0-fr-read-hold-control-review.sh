#!/bin/sh
set -eu

if [ "$#" -ne 1 ]; then
    echo "usage: $0 <candidate-archive.tar.gz>" >&2
    exit 2
fi

ARCHIVE="$1"
PRE_LOOP_MARKER="TALOS: fr-hold-control-pre-read-loop"
PRE_READ_MARKER="rpi5-rp1-uart0-fr-read-hold-control: pre-read-control-marker"
POST_READ_MARKER="rpi5-rp1-uart0-fr-read-hold-control: post-read-terminal-hold-marker"
POST_LOOP_MARKER="TALOS: fr-hold-control-post-read-loop"

./scripts/rpi5-archive-review.sh "$ARCHIVE" >/dev/null

tmp_root="target/tmp"
mkdir -p "$tmp_root"
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-uart0-fr-read-hold-control-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-uart0-fr-read: start" \
    "rpi5-rp1-uart0-fr-read: pre-mmio-read" \
    "rpi5-rp1-uart0-fr-read-hold-control: classification=pre-read-control-before-rp1-read" \
    "$PRE_LOOP_MARKER" \
    "$PRE_READ_MARKER" \
    "phase11-rp1-pcie-map-contract-v1" \
    "target=rp1-uart0-fr-read" \
    "classification=mapped/read-value" \
    "$POST_READ_MARKER" \
    "$POST_LOOP_MARKER"; do
    if ! grep -Fq "$required" "$kernel_strings"; then
        echo "kernel image missing hold-control FR-read string: $required" >&2
        exit 1
    fi
done

image_size="$(wc -c < "$extract_dir/kernel_2712.img" | tr -d ' ')"
text_offset="$(od -An -tu8 -j8 -N8 "$extract_dir/kernel_2712.img" | tr -d ' ')"
header_image_size="$(od -An -tu8 -j16 -N8 "$extract_dir/kernel_2712.img" | tr -d ' ')"
flags="$(od -An -tu8 -j24 -N8 "$extract_dir/kernel_2712.img" | tr -d ' ')"

printf 'candidate_archive=%s\n' "$ARCHIVE"
printf 'candidate_archive_sha256=%s\n' "$(sha256sum "$ARCHIVE" | awk '{print $1}')"
printf 'kernel_2712_sha256=%s\n' "$(sha256sum "$extract_dir/kernel_2712.img" | awk '{print $1}')"
printf 'kernel_2712_size=%s\n' "$image_size"
printf 'header_image_size=%s\n' "$header_image_size"
printf 'text_offset=%s\n' "$text_offset"
printf 'flags=%s\n' "$flags"
printf 'pre_loop_marker=%s\n' "$PRE_LOOP_MARKER"
printf 'pre_read_marker=%s\n' "$PRE_READ_MARKER"
printf 'post_read_marker=%s\n' "$POST_READ_MARKER"
printf 'post_loop_marker=%s\n' "$POST_LOOP_MARKER"
