#!/bin/sh
set -eu

if [ "$#" -ne 1 ]; then
    echo "usage: $0 <candidate-archive.tar.gz>" >&2
    exit 2
fi

ARCHIVE="$1"
RESULT_MARKER="TALOS: rp1-clock-reset-dependency-result"

./scripts/rpi5-archive-review.sh "$ARCHIVE" >/dev/null

tmp_root="target/tmp"
mkdir -p "$tmp_root"
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-clock-reset-dependency-read-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-clock-reset-dependency-read: start" \
    "rpi5-rp1-clock-reset-dependency-read: before-read-only-loads" \
    "$RESULT_MARKER" \
    "phase11-rp1-clock-reset-dependency-source-contract-v1" \
    "rp1-observed-clock-reset-dependency-preflight-read" \
    "observed-base=" \
    "0x1c00000000" \
    "sysinfo-base=" \
    "clock-manager-base=" \
    "sysinfo-chip-id" \
    "sysinfo-platform" \
    "pll-sys-cs" \
    "clk-sys-ctrl" \
    "clk-sys-div-int" \
    "clk-sys-sel" \
    "clk-slow-sys-ctrl" \
    "clk-uart-ctrl" \
    "clk-uart-div-int" \
    "clk-uart-sel" \
    "-source-offset=" \
    "address=" \
    "width=32" \
    "raw=" \
    "expected-chip-id=" \
    "chip-id-matches-expected=" \
    "chip-id-is-deaddead=" \
    "platform-is-deaddead=" \
    "pll-sys-locked=" \
    "clk-sys-enabled=" \
    "clk-slow-sys-enabled=" \
    "clk-uart-enabled=" \
    "any-selected-clock-deaddead=" \
    "all-selected-clock-deaddead=" \
    "reset-status-source=none-selected-read-only" \
    "retained-gpio14-blocker=observed-gpio14-ownership-preflight-blocked-non-gpio-function" \
    "retained-gpio16-blocker=observed-gpio16-ownership-preflight-blocked-non-gpio-function" \
    "retained-0x1f-sysinfo-clock-sentinel=rp1-sysinfo-and-clock-window-sentinel" \
    "classification-vocabulary=" \
    "observed-clock-reset-dependency-visible" \
    "observed-clock-reset-dependency-blocked-sysinfo-sentinel" \
    "observed-clock-reset-dependency-blocked-clock-manager-sentinel" \
    "observed-clock-reset-dependency-blocked-system-clock-disabled" \
    "observed-clock-reset-dependency-blocked-uart-clock-disabled" \
    "observed-clock-reset-dependency-no-return-or-trap" \
    "observed-clock-reset-dependency-inconclusive-capture" \
    "staging/build-blocker" \
    "classification="; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing clock/reset dependency read string: $required" >&2
        exit 1
    fi
done

for forbidden in \
    "TALOS: rp1-clock-reset-dependency-control" \
    "address=not-constructed" \
    "classification=simulated/control" \
    "rpi5-rp1-clock-adc-ctrl-write-restore: before-rp1-clock-write-restore" \
    "rpi5-rp1-clock-adc-ctrl-enable-toggle: before-rp1-clock-enable-toggle" \
    "rpi5-rp1-gpio16-owned-event-discriminator: before-read-only-observed-aperture-loads"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden clock/reset dependency read string: $forbidden" >&2
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
printf 'forbidden_clock_reset_dependency_control_strings_absent=true\n'
