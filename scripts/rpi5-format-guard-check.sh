#!/bin/sh
set -eu

repo_root="$(git rev-parse --show-toplevel)"
tmp_root="$repo_root/target/tmp"
mkdir -p "$tmp_root"
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-format-guard.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

git -C "$repo_root" ls-files -z \
    | tar -C "$repo_root" --null -T - -cf - \
    | tar -C "$work_dir" -xf -

perl -0pi -e 's/println!\("Talos booting on talos-rpi5-bcm2712"\);/println!("Talos booting on {}", boot_info.target.name());/' \
    "$work_dir/src/main.rs"

set +e
output="$(
    cd "$work_dir"
    env -u TALOS_RPI5_DYNAMIC_FORMAT_FALLBACK_DIAGNOSTIC ./scripts/rpi5-image.sh 2>&1
)"
status="$?"
set -e

if [ "$status" -eq 0 ]; then
    echo "expected Pi 5 formatted early-console build to fail, but it passed" >&2
    exit 1
fi

case "$output" in
    *"Pi 5 early console only accepts static print!/println! literals"*)
        printf '%s\n' "Pi 5 early formatting guard PASS"
        ;;
    *)
        printf '%s\n' "$output" >&2
        echo "Pi 5 formatted early-console build failed for the wrong reason" >&2
        exit 1
        ;;
esac
