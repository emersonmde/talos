#!/bin/sh
set -eu

if [ "$#" -ne 1 ] && [ "$#" -ne 3 ]; then
    echo "usage: $0 <control-archive.tar.gz> [--capture-nonce NONCE]" >&2
    exit 2
fi

ARCHIVE="$1"
MARKER="TALOS: rp1-ethernet-clock-reset-write-restore-control"
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
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-ethernet-clock-reset-write-restore-control-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-ethernet-clock-reset-write-restore-control: start" \
    "rpi5-rp1-ethernet-clock-reset-write-restore-control: no-clock-write-no-ethernet-no-mmio-target-construction" \
    "$MARKER" \
    "phase12-rp1-ethernet-clock-reset-write-restore-report-contract-v1" \
    "phase12-rp1-ethernet-clock-reset-write-target-source-contract-v1" \
    "phase12-rp1-ethernet-clock-reset-write-target-source-contract-20260610" \
    "report-kind=" \
    "no-clock-write-no-ethernet-control" \
    "target=none register=none clock-name=none clock-id=none" \
    "observed-rp1-base=not-constructed" \
    "source-offset=0x18134 address=not-constructed" \
    "width=32 allowed-write-value=withheld" \
    "pre-raw=" \
    "pre-enable=" \
    "pre-auxsrc=" \
    "pre-source=" \
    "post-raw=" \
    "post-enable=" \
    "post-auxsrc=" \
    "post-source=" \
    "restore-raw=" \
    "restore-enable=" \
    "restore-auxsrc=" \
    "restore-source=" \
    "post-eq-pre=true" \
    "restore-eq-pre=true" \
    "preserved-fields=withheld" \
    "future-proof-classifications=rp1-ethernet-clk-eth-tsu-ctrl-idempotent-write-restored" \
    "rejected-runtime-hardware-claims=ethernet-driver-readiness" \
    "claims-ethernet-ready=false" \
    "claims-rp1-clk-sys-transition=false" \
    "claims-clk-eth-ctrl-write=false" \
    "claims-reset-controller-ownership=false" \
    "claims-gpio32-phy-reset-ownership=false" \
    "claims-mdio-phy-ownership=false" \
    "claims-interrupt-ownership=false" \
    "claims-dma-descriptor-ownership=false" \
    "claims-packet-io=false" \
    "claims-networking=false" \
    "claims-sockets=false" \
    "claims-ssh=false" \
    "claims-phase-12-2=false" \
    "claims-phase-transition=false" \
    "classification=no-clock-write-no-ethernet-rp1-ethernet-write-restore-control"; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing RP1 Ethernet clock/reset write/restore control string: $required" >&2
        exit 1
    fi
done

if [ -n "$CAPTURE_NONCE" ]; then
    if ! grep -Fq -- " capture-nonce=" "$kernel_strings"; then
        echo "kernel image missing RP1 Ethernet clock/reset write/restore control capture nonce label" >&2
        exit 1
    fi
    if ! grep -Fq -- "$CAPTURE_NONCE" "$kernel_strings"; then
        echo "kernel image missing RP1 Ethernet clock/reset write/restore control capture nonce: $CAPTURE_NONCE" >&2
        exit 1
    fi
fi

for forbidden in \
    "TALOS: rp1-ethernet-clock-reset-write-restore-candidate" \
    "rpi5-rp1-ethernet-clock-reset-write-restore-candidate: before-clk-eth-tsu-ctrl-idempotent-write-restore" \
    "classification=rp1-ethernet-clk-eth-tsu-ctrl-idempotent-write-restored" \
    "classification=rp1-ethernet-clk-eth-tsu-ctrl-idempotent-write-mismatch-restored" \
    "classification=rp1-ethernet-clk-eth-tsu-ctrl-idempotent-write-restore-failed" \
    "target=rp1-ethernet-clk-eth-tsu-ctrl-idempotent-write-restore" \
    "address=0x1c00018134"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden RP1 Ethernet clock/reset write/restore control string: $forbidden" >&2
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
printf 'forbidden_rp1_ethernet_clock_reset_write_restore_control_runtime_strings_absent=true\n'
