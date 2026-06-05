#!/bin/sh
set -eu

if [ "$#" -ne 1 ]; then
    echo "usage: $0 <candidate-archive.tar.gz>" >&2
    exit 2
fi

ARCHIVE="$1"
INITRAMFS_FILE="initramfs_2712"
SERIAL_PREFIX="da591740"

./scripts/rpi5-archive-review.sh "$ARCHIVE" >/dev/null

tmp_root="target/tmp"
mkdir -p "$tmp_root"
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-generated-root-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

for file in "$INITRAMFS_FILE" "$SERIAL_PREFIX/$INITRAMFS_FILE"; do
    if [ ! -s "$extract_dir/$file" ]; then
        echo "candidate missing generated-root artifact: $file" >&2
        exit 1
    fi
done

if ! cmp -s "$extract_dir/$INITRAMFS_FILE" "$extract_dir/$SERIAL_PREFIX/$INITRAMFS_FILE"; then
    echo "serial-prefixed generated-root artifact differs from root artifact" >&2
    exit 1
fi

for config in config.txt "$SERIAL_PREFIX/config.txt"; do
    if ! grep -qx "initramfs $INITRAMFS_FILE followkernel" "$extract_dir/$config"; then
        echo "candidate config missing firmware initramfs line: $config" >&2
        exit 1
    fi
done

artifact_strings="$work_dir/artifact-strings.txt"
kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/$INITRAMFS_FILE" >"$artifact_strings"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for expected in \
    "Talos generated-root external artifact A" \
    "/generated/manifest.txt" \
    "/generated/status7"; do
    if ! grep -Fq "$expected" "$artifact_strings"; then
        echo "generated-root artifact missing proof string: $expected" >&2
        exit 1
    fi
done

for expected in \
    "rpi5-generated-root-boot-transport-proof" \
    "pi5-generated-root-boot-transport-complete" \
    "firmware-initramfs" \
    "rootinfo" \
    "exec /generated/status7 alpha"; do
    if ! grep -Fq "$expected" "$kernel_strings"; then
        echo "kernel image missing proof marker string: $expected" >&2
        exit 1
    fi
done

manifest="$work_dir/manifest.txt"
tar -tzf "$ARCHIVE" | sed 's#^[.]/##; s#/$##' | sed '/^[.]$/d; /^$/d' | LC_ALL=C sort >"$manifest"

printf 'candidate_archive=%s\n' "$ARCHIVE"
printf 'candidate_archive_sha256=%s\n' "$(sha256sum "$ARCHIVE" | awk '{print $1}')"
printf 'kernel_2712_sha256=%s\n' "$(sha256sum "$extract_dir/kernel_2712.img" | awk '{print $1}')"
printf 'kernel_2712_size=%s\n' "$(wc -c < "$extract_dir/kernel_2712.img" | tr -d ' ')"
printf 'kernel8_matches_kernel_2712=%s\n' "$(cmp -s "$extract_dir/kernel_2712.img" "$extract_dir/kernel8.img" && echo true || echo false)"
printf 'generated_root_artifact_sha256=%s\n' "$(sha256sum "$extract_dir/$INITRAMFS_FILE" | awk '{print $1}')"
printf 'generated_root_artifact_size=%s\n' "$(wc -c < "$extract_dir/$INITRAMFS_FILE" | tr -d ' ')"
printf 'serial_prefixed_artifact_matches_root=true\n'
printf 'initramfs_config_line=%s\n' "initramfs $INITRAMFS_FILE followkernel"
printf 'manifest:\n'
sed 's/^/  /' "$manifest"
