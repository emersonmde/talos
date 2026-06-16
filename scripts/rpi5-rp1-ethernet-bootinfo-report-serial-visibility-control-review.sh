#!/bin/sh
set -eu

if [ "$#" -ne 1 ] && [ "$#" -ne 3 ]; then
    echo "usage: $0 <control-archive.tar.gz> [--capture-nonce NONCE]" >&2
    exit 2
fi

ARCHIVE="$1"
CAPTURE_NONCE=

if [ "$#" -eq 3 ]; then
    if [ "$2" != "--capture-nonce" ] || [ -z "$3" ]; then
        echo "usage: $0 <control-archive.tar.gz> [--capture-nonce NONCE]" >&2
        exit 2
    fi
    CAPTURE_NONCE="$3"
fi

./scripts/rpi5-archive-review.sh "$ARCHIVE" >/dev/null

tmp_root="target/tmp"
mkdir -p "$tmp_root"
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-bootinfo-report-visibility-control-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "TALOS: rp1-ethernet-bootinfo-report-serial-visibility" \
    "TALOS: rp1-ethernet-bootinfo-report-serial-visibility-control" \
    "contract-id=phase12-rp1-ethernet-bootinfo-report-serial-visibility-contract-v1" \
    "core-task-id=phase12-rp1-ethernet-bootinfo-report-serial-visibility-core-20260616" \
    "proof-task-id=phase12-rp1-ethernet-bootinfo-report-serial-visibility-pi5-proof-20260616" \
    "selected-discriminator=dual-stage-earliest-entry-and-post-bootinfo-report-path-serial-visibility" \
    "marker=bootinfo-report-visibility-earliest-entry-marker" \
    "marker-stage=earliest-entry" \
    "control-kind=earliest-only-before-bootinfo" \
    "marker-stage=control-stop-before-bootinfo" \
    "post-bootinfo-report-path-marker-visible=false" \
    "target=none controller=none compatible=none phy-model=none" \
    "selected-registers=none bcm54213pe-register-values=withheld" \
    "ethernet-target-facts-constructed=false" \
    "mdio-target-constructed=false man-frame-constructed=false" \
    "macb-target-constructed=false gpio32-or-phy-target-constructed=false" \
    "volatile-ethernet-access-performed=false" \
    "selector-write-performed=false bmcr-write-performed=false" \
    "broadcom-shadow-mmd-aux-access-performed=false" \
    "packet-io-performed=false networking-performed=false ssh-performed=false" \
    "classification=bootinfo-report-serial-visibility-earliest-marker" \
    "classification=no-bootinfo-report-path-rp1-ethernet-serial-visibility-control"; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing BootInfo/report visibility control string: $required" >&2
        exit 1
    fi
done

if [ -n "$CAPTURE_NONCE" ]; then
    if ! grep -Fq -- " capture-nonce=" "$kernel_strings"; then
        echo "kernel image missing BootInfo/report visibility control capture nonce label" >&2
        exit 1
    fi
    if ! grep -Fq -- "$CAPTURE_NONCE" "$kernel_strings"; then
        echo "kernel image missing BootInfo/report visibility control capture nonce: $CAPTURE_NONCE" >&2
        exit 1
    fi
fi

for forbidden in \
    "marker=bootinfo-report-visibility-post-bootinfo-report-path-marker" \
    "marker-stage=post-bootinfo-report-path" \
    "post-bootinfo-report-path-marker-visible=true" \
    "classification=bootinfo-report-serial-visibility-candidate-local-static" \
    "MII_CTRL1000" \
    "MII_STAT1000" \
    "physid1=0x600d" \
    "physid2=0x84a2" \
    "mdio-target-constructed=true" \
    "man-frame-constructed=true" \
    "macb-target-constructed=true" \
    "gpio32-or-phy-target-constructed=true" \
    "volatile-ethernet-access-performed=true" \
    "bmcr-write-performed=true"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden BootInfo/report visibility control string: $forbidden" >&2
        exit 1
    fi
done

image_size="$(wc -c < "$extract_dir/kernel_2712.img" | tr -d ' ')"
text_offset="$(od -An -tu8 -j8 -N8 "$extract_dir/kernel_2712.img" | tr -d ' ')"
header_image_size="$(od -An -tu8 -j16 -N8 "$extract_dir/kernel_2712.img" | tr -d ' ')"
flags="$(od -An -tu8 -j24 -N8 "$extract_dir/kernel_2712.img" | tr -d ' ')"

printf 'control_archive=%s\n' "$ARCHIVE"
printf 'control_archive_sha256=%s\n' "$(sha256sum "$ARCHIVE" | awk '{print $1}')"
printf 'kernel_2712_sha256=%s\n' "$(sha256sum "$extract_dir/kernel_2712.img" | awk '{print $1}')"
printf 'kernel_2712_size=%s\n' "$image_size"
printf 'header_image_size=%s\n' "$header_image_size"
printf 'text_offset=%s\n' "$text_offset"
printf 'flags=%s\n' "$flags"
printf 'earliest_marker=bootinfo-report-visibility-earliest-entry-marker\n'
if [ -n "$CAPTURE_NONCE" ]; then
    printf 'capture_nonce=%s\n' "$CAPTURE_NONCE"
fi
printf 'forbidden_bootinfo_report_visibility_control_runtime_strings_absent=true\n'
