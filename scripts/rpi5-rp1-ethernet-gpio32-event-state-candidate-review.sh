#!/bin/sh
set -eu

if [ "$#" -ne 1 ] && [ "$#" -ne 3 ]; then
    echo "usage: $0 <candidate-archive.tar.gz> [--capture-nonce NONCE]" >&2
    exit 2
fi

ARCHIVE="$1"
MARKER="TALOS: rp1-ethernet-gpio32-event-state-candidate"
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
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-ethernet-gpio32-event-state-candidate-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-ethernet-gpio32-event-state-candidate: start" \
    "before-gpio32-rio1-pad-read-only-volatile-loads-no-event-clear-no-mmio-writes" \
    "$MARKER" \
    "phase12-rp1-ethernet-gpio32-event-state-readonly-discriminator-report-v1" \
    "phase12-rp1-ethernet-gpio32-event-state-source-contract-v1" \
    "phase12-rp1-ethernet-gpio32-event-state-source-contract-20260611" \
    "hardware-proof-limited-to-gpio32-readonly-event-state-control-output" \
    "report-kind=" \
    "candidate" \
    "target=rp1-ethernet-gpio32-event-state-readonly-discriminator" \
    "gpio-controller=rp1_gpio gpio-line=32 signal=ETH_RST_N" \
    "bank=bank1 bank-local-bit=4 active-low=true" \
    "gpio32-status-address=" \
    "gpio32-ctrl-address=" \
    "rio1-out-address=" \
    "rio1-oe-address=" \
    "rio1-in-address=" \
    "gpio32-pad-address=" \
    "status-event-mask=" \
    "event-bits=" \
    "source-decoding-status=source-backed-bits-20-27" \
    "source-backed-event-bit-names=bit20-raw-falling" \
    "v2-classification=rp1-ethernet-gpio32-phy-reset-blocked-unexpected-event-state" \
    "v2-writes-performed=false writes-performed=false event-clear-performed=false touched-fields=none" \
    "allowed-classifications=rp1-ethernet-gpio32-event-state-clear-precondition" \
    "rp1-ethernet-gpio32-event-state-blocked-event-state" \
    "rp1-ethernet-gpio32-event-state-source-unresolved-event-state" \
    "rejected-runtime-hardware-claims=event-clearing" \
    "claims-event-clear=false claims-gpio-rio-pad-mmio-write=false" \
    "claims-gpio32-ownership=false" \
    "claims-gpio32-write-restore-retry=false" \
    "claims-mdio-transactions=false claims-phy-ownership=false" \
    "claims-ethernet-driver-ready=false" \
    "claims-networking=false claims-sockets=false claims-ssh=false" \
    "classification="; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing GPIO32 event-state candidate string: $required" >&2
        exit 1
    fi
done

if [ -n "$CAPTURE_NONCE" ]; then
    if ! grep -Fq -- " capture-nonce=" "$kernel_strings"; then
        echo "kernel image missing GPIO32 event-state candidate capture nonce label" >&2
        exit 1
    fi
    if ! grep -Fq -- "$CAPTURE_NONCE" "$kernel_strings"; then
        echo "kernel image missing GPIO32 event-state candidate capture nonce: $CAPTURE_NONCE" >&2
        exit 1
    fi
fi

for forbidden in \
    "TALOS: rp1-ethernet-gpio32-event-state-control" \
    "no-gpio32-rio-pad-ethernet-no-mmio-target-construction" \
    "before-gpio32-eth-rst-n-write-restore" \
    "claims-gpio32-write-restore-only=true" \
    "writes-performed=true"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden GPIO32 event-state candidate string: $forbidden" >&2
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
printf 'forbidden_gpio32_event_state_candidate_runtime_strings_absent=true\n'
