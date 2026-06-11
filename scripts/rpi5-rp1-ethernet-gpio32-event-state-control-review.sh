#!/bin/sh
set -eu

if [ "$#" -ne 1 ] && [ "$#" -ne 3 ]; then
    echo "usage: $0 <control-archive.tar.gz> [--capture-nonce NONCE]" >&2
    exit 2
fi

ARCHIVE="$1"
MARKER="TALOS: rp1-ethernet-gpio32-event-state-control"
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
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-ethernet-gpio32-event-state-control-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-ethernet-gpio32-event-state-control: start" \
    "no-gpio32-rio-pad-ethernet-no-mmio-target-construction" \
    "$MARKER" \
    "phase12-rp1-ethernet-gpio32-event-state-readonly-discriminator-report-v1" \
    "phase12-rp1-ethernet-gpio32-event-state-source-contract-v1" \
    "phase12-rp1-ethernet-gpio32-event-state-source-contract-20260611" \
    "hardware-proof-limited-to-gpio32-readonly-event-state-control-output" \
    "report-kind=" \
    "no-gpio-no-ethernet-control" \
    "target=none gpio-controller=none gpio-line=none signal=none" \
    "bank=none bank-local-bit=none active-low=false" \
    "gpio32-status-address=not-constructed" \
    "gpio32-ctrl-address=not-constructed" \
    "rio1-out-address=not-constructed" \
    "rio1-oe-address=not-constructed" \
    "rio1-in-address=not-constructed" \
    "gpio32-pad-address=not-constructed" \
    "status=withheld ctrl=withheld" \
    "status-event-mask=withheld event-bits=withheld" \
    "source-decoding-status=capture-chain-inconclusive" \
    "source-backed-event-bit-names=withheld" \
    "v2-writes-performed=false writes-performed=false event-clear-performed=false touched-fields=none" \
    "allowed-classifications=rp1-ethernet-gpio32-event-state-clear-precondition" \
    "rejected-runtime-hardware-claims=event-clearing" \
    "claims-event-clear=false claims-gpio-rio-pad-mmio-write=false" \
    "claims-gpio32-ownership=false" \
    "classification=no-gpio-no-ethernet-rp1-ethernet-gpio32-event-state-control"; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing GPIO32 event-state control string: $required" >&2
        exit 1
    fi
done

if [ -n "$CAPTURE_NONCE" ]; then
    if ! grep -Fq -- " capture-nonce=" "$kernel_strings"; then
        echo "kernel image missing GPIO32 event-state control capture nonce label" >&2
        exit 1
    fi
    if ! grep -Fq -- "$CAPTURE_NONCE" "$kernel_strings"; then
        echo "kernel image missing GPIO32 event-state control capture nonce: $CAPTURE_NONCE" >&2
        exit 1
    fi
fi

for forbidden in \
    "TALOS: rp1-ethernet-gpio32-event-state-candidate" \
    "before-gpio32-rio1-pad-read-only-volatile-loads-no-event-clear-no-mmio-writes" \
    "target=rp1-ethernet-gpio32-event-state-readonly-discriminator" \
    "gpio-controller=rp1_gpio gpio-line=32" \
    "source-decoding-status=source-backed-bits-20-27" \
    "classification=rp1-ethernet-gpio32-event-state-clear-precondition" \
    "classification=rp1-ethernet-gpio32-event-state-blocked-event-state"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden GPIO32 event-state control string: $forbidden" >&2
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
printf 'forbidden_gpio32_event_state_control_runtime_strings_absent=true\n'
