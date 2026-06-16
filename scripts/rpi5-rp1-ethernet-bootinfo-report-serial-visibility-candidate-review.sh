#!/bin/sh
set -eu

if [ "$#" -ne 1 ] && [ "$#" -ne 3 ]; then
    echo "usage: $0 <candidate-archive.tar.gz> [--capture-nonce NONCE]" >&2
    exit 2
fi

ARCHIVE="$1"
CAPTURE_NONCE=

if [ "$#" -eq 3 ]; then
    if [ "$2" != "--capture-nonce" ] || [ -z "$3" ]; then
        echo "usage: $0 <candidate-archive.tar.gz> [--capture-nonce NONCE]" >&2
        exit 2
    fi
    CAPTURE_NONCE="$3"
fi

./scripts/rpi5-archive-review.sh "$ARCHIVE" >/dev/null

tmp_root="target/tmp"
mkdir -p "$tmp_root"
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-bootinfo-report-visibility-candidate-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "TALOS: rp1-ethernet-bootinfo-report-serial-visibility" \
    "contract-id=phase12-rp1-ethernet-bootinfo-report-serial-visibility-contract-v1" \
    "core-task-id=phase12-rp1-ethernet-bootinfo-report-serial-visibility-core-20260616" \
    "proof-task-id=phase12-rp1-ethernet-bootinfo-report-serial-visibility-pi5-proof-20260616" \
    "selected-discriminator=dual-stage-earliest-entry-and-post-bootinfo-report-path-serial-visibility" \
    "accepted-frontier=kernel-entry-serial-beacon-frontier-closed-beacon-observed" \
    "bootinfo-source=report_boot_identity" \
    "marker=bootinfo-report-visibility-earliest-entry-marker" \
    "marker=bootinfo-report-visibility-post-bootinfo-report-path-marker" \
    "marker-stage=earliest-entry" \
    "marker-stage=post-bootinfo-report-path" \
    "emits-before-boot-info=true emits-after-boot-info=false" \
    "emits-before-boot-info=false emits-after-boot-info=true" \
    "post-bootinfo-report-path-marker-visible=true" \
    "target=none controller=none compatible=none phy-model=none" \
    "selected-registers=none bcm54213pe-register-values=withheld" \
    "ethernet-target-facts-constructed=false" \
    "mdio-target-constructed=false man-frame-constructed=false" \
    "macb-target-constructed=false gpio32-or-phy-target-constructed=false" \
    "volatile-ethernet-access-performed=false" \
    "selector-write-performed=false bmcr-write-performed=false" \
    "broadcom-shadow-mmd-aux-access-performed=false" \
    "packet-io-performed=false networking-performed=false ssh-performed=false" \
    "hardware-outcomes=no-selected-tftp,no-earliest-marker,earliest-marker-only,both-markers-observed" \
    "rejected-claims=bcm54213pe-register-values,ethernet-readiness,link-readiness" \
    "classification=bootinfo-report-serial-visibility-candidate-local-static"; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing BootInfo/report visibility candidate string: $required" >&2
        exit 1
    fi
done

if [ -n "$CAPTURE_NONCE" ]; then
    if ! grep -Fq -- " capture-nonce=" "$kernel_strings"; then
        echo "kernel image missing BootInfo/report visibility capture nonce label" >&2
        exit 1
    fi
    if ! grep -Fq -- "$CAPTURE_NONCE" "$kernel_strings"; then
        echo "kernel image missing BootInfo/report visibility capture nonce: $CAPTURE_NONCE" >&2
        exit 1
    fi
fi

for forbidden in \
    "TALOS: rp1-ethernet-bootinfo-report-serial-visibility-control" \
    "classification=no-bootinfo-report-path-rp1-ethernet-serial-visibility-control" \
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
        echo "kernel image contains forbidden BootInfo/report visibility candidate string: $forbidden" >&2
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
printf 'earliest_marker=bootinfo-report-visibility-earliest-entry-marker\n'
printf 'post_bootinfo_marker=bootinfo-report-visibility-post-bootinfo-report-path-marker\n'
if [ -n "$CAPTURE_NONCE" ]; then
    printf 'capture_nonce=%s\n' "$CAPTURE_NONCE"
fi
printf 'forbidden_bootinfo_report_visibility_candidate_runtime_strings_absent=true\n'
