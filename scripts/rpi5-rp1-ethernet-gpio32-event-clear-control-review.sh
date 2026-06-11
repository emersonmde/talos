#!/bin/sh
set -eu

if [ "$#" -ne 1 ] && [ "$#" -ne 3 ]; then
    echo "usage: $0 <control-archive.tar.gz> [--capture-nonce NONCE]" >&2
    exit 2
fi

ARCHIVE="$1"
MARKER="TALOS: rp1-ethernet-gpio32-event-clear-control"
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
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-ethernet-gpio32-event-clear-control-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-ethernet-gpio32-event-clear-control: start" \
    "no-gpio32-rio-pad-ethernet-no-mmio-target-construction-no-event-clear" \
    "$MARKER" \
    "phase12-rp1-ethernet-gpio32-event-clear-guard-report-contract-v1" \
    "phase12-rp1-ethernet-gpio32-event-clear-source-contract-v1" \
    "phase12-rp1-ethernet-gpio32-event-clear-source-contract-20260611" \
    "hardware-proof-boundary-classification=hardware-proof-limited-to-gpio32-irqreset-event-clear-control-output" \
    "report-kind=" \
    "no-gpio-no-ethernet-control" \
    "target=none gpio-controller=none gpio-line=none signal=none" \
    "gpio32-status-address=not-constructed" \
    "gpio32-ctrl-set-address=not-constructed" \
    "pre-status=withheld pre-ctrl=withheld" \
    "event-clear-write-value=withheld event-clear-performed=false" \
    "touched-fields=none" \
    "allowed-classifications=event-clear-visible-with-invariants" \
    "no-gpio-no-ethernet-rp1-ethernet-gpio32-event-clear-control" \
    "claims-event-clear-executed=false" \
    "claims-gpio32-ownership=false" \
    "claims-ethernet-driver-ready=false" \
    "claims-networking=false claims-sockets=false claims-ssh=false" \
    "classification=no-gpio-no-ethernet-rp1-ethernet-gpio32-event-clear-control"; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing GPIO32 event-clear control string: $required" >&2
        exit 1
    fi
done

if [ -n "$CAPTURE_NONCE" ]; then
    if ! grep -Fq -- " capture-nonce=" "$kernel_strings"; then
        echo "kernel image missing GPIO32 event-clear control capture nonce label" >&2
        exit 1
    fi
    if ! grep -Fq -- "$CAPTURE_NONCE" "$kernel_strings"; then
        echo "kernel image missing GPIO32 event-clear control capture nonce: $CAPTURE_NONCE" >&2
        exit 1
    fi
fi

for forbidden in \
    "TALOS: rp1-ethernet-gpio32-event-clear-candidate" \
    "before-guarded-gpio32-irqreset-write" \
    "target=rp1-ethernet-gpio32-event-clear-irqreset" \
    "claims-event-clear-executed=true" \
    "touched-fields=GPIO32_CTRL_SET.IRQRESET"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden GPIO32 event-clear control string: $forbidden" >&2
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
printf 'forbidden_gpio32_event_clear_control_runtime_strings_absent=true\n'
