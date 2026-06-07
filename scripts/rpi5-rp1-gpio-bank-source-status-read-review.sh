#!/bin/sh
set -eu

if [ "$#" -ne 1 ]; then
    echo "usage: $0 <candidate-archive.tar.gz>" >&2
    exit 2
fi

ARCHIVE="$1"
RESULT_MARKER="TALOS: rp1-gpio-bank-source-status-result"

./scripts/rpi5-archive-review.sh "$ARCHIVE" >/dev/null

tmp_root="target/tmp"
mkdir -p "$tmp_root"
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-gpio-bank-source-status-read-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-gpio-bank-source-status-read: start" \
    "rpi5-rp1-gpio-bank-source-status-read: before-rp1-loads" \
    "$RESULT_MARKER" \
    "phase11-rp1-gpio-bank-source-status-contract-v1" \
    "target=rp1-io-bank0-source-status-read" \
    "source-hwirq=" \
    "bank=" \
    "bank-first-gpio=" \
    "bank-gpio-count=" \
    "bank-last-gpio=" \
    "inte-address=" \
    "ints-address=" \
    "width=32" \
    "inte-raw=" \
    "ints-raw=" \
    "gpio14-mask=" \
    "gpio14-enabled=" \
    "gpio14-source-status=" \
    "source-status-mask=" \
    "source-status-nonzero=" \
    "classification=gpio-bank-source-status-visible"; do
    if ! grep -Fq "$required" "$kernel_strings"; then
        echo "kernel image missing GPIO bank source-status diagnostic string: $required" >&2
        exit 1
    fi
done

for forbidden in \
    "TALOS: rp1-gpio-bank-source-status-control" \
    "classification=simulated/control"; do
    if grep -Fq "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden GPIO bank source-status diagnostic string: $forbidden" >&2
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
printf 'result_marker=%s\n' "$RESULT_MARKER"
