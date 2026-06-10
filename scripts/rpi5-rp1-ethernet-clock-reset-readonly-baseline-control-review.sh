#!/bin/sh
set -eu

if [ "$#" -ne 1 ] && [ "$#" -ne 3 ]; then
    echo "usage: $0 <control-archive.tar.gz> [--capture-nonce NONCE]" >&2
    exit 2
fi

ARCHIVE="$1"
MARKER="TALOS: rp1-ethernet-clock-reset-readonly-baseline-control"
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
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-ethernet-clock-reset-baseline-control-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-ethernet-clock-reset-readonly-baseline-control: start" \
    "no-clock-reset-no-ethernet-no-mmio-target-construction" \
    "$MARKER" \
    "phase12-rp1-ethernet-clock-reset-guard-contract-v1" \
    "phase12-rp1-ethernet-clock-reset-ownership-contract-20260610" \
    "phase12-rp1-ethernet-prereq-ownership-contract-v1" \
    "hardware-proof-limited-to-clock-reset-readonly-baseline-report-visibility-control-output" \
    "report-kind=" \
    "no-clock-reset-no-ethernet-control" \
    "observed-window-macb-mid-context-cpu-physical-target=not-constructed" \
    "observed-window-macb-mid-context-raw=none" \
    "observed-window-macb-mid-context-idnum=none" \
    "observed-window-macb-mid-context-rev=none" \
    "selected-read-only-baseline-fields=none" \
    "clock-names=none" \
    "clock-policy-classification=no-clock-reset-ownership" \
    "reset-controller-policy-classification=no-accepted-rp1-eth-reset-controller-target" \
    "phy-reset-gpio-context=none" \
    "phy-mdio-policy-classification=no-phy-reset-or-mdio-ownership" \
    "read-only-baseline-requirements=withheld" \
    "write-backed-invariants=withheld" \
    "claims-ethernet-ready=false" \
    "claims-broad-mmio-ready=false" \
    "claims-rp1-mmio-writes=false" \
    "claims-clock-reset-writes=false" \
    "claims-clock-reset-ownership=false" \
    "claims-rp1-clk-sys-transition=false" \
    "claims-reset-controller-ownership=false" \
    "claims-gpio32-phy-reset-ownership=false" \
    "claims-mdio-phy-ownership=false" \
    "claims-interrupt-ownership=false" \
    "claims-dma-descriptor-ownership=false" \
    "classification=no-clock-reset-no-ethernet-rp1-ethernet-clock-reset-baseline-control"; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing RP1 Ethernet clock/reset baseline control string: $required" >&2
        exit 1
    fi
done

if [ -n "$CAPTURE_NONCE" ]; then
    if ! grep -Fq -- " capture-nonce=" "$kernel_strings"; then
        echo "kernel image missing RP1 Ethernet clock/reset baseline control capture nonce label" >&2
        exit 1
    fi
    if ! grep -Fq -- "$CAPTURE_NONCE" "$kernel_strings"; then
        echo "kernel image missing RP1 Ethernet clock/reset baseline control capture nonce: $CAPTURE_NONCE" >&2
        exit 1
    fi
fi

for forbidden in \
    "TALOS: rp1-ethernet-clock-reset-readonly-baseline-candidate" \
    "classification=rp1-ethernet-clock-reset-readonly-baseline-report-visible" \
    "selected-read-only-baseline-fields=pclk,hclk,tsu_clk,tx_clk" \
    "observed-window-macb-mid-context-raw=0x"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden RP1 Ethernet clock/reset baseline control string: $forbidden" >&2
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
printf 'control_marker=%s\n' "$MARKER"
if [ -n "$CAPTURE_NONCE" ]; then
    printf 'capture_nonce=%s\n' "$CAPTURE_NONCE"
fi
printf 'forbidden_rp1_ethernet_clock_reset_baseline_control_runtime_strings_absent=true\n'
