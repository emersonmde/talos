#!/bin/sh
set -eu

if [ "$#" -ne 1 ] && [ "$#" -ne 3 ]; then
    echo "usage: $0 <candidate-archive.tar.gz> [--capture-nonce NONCE]" >&2
    exit 2
fi

ARCHIVE="$1"
MARKER="TALOS: rp1-ethernet-observed-window-discriminator-candidate"
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
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-ethernet-observed-window-discriminator-candidate-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-ethernet-observed-window-discriminator-candidate: start" \
    "before-observed-sysinfo-and-observed-window-gem-mid-read-only-volatile-loads" \
    "$MARKER" \
    "phase12-rp1-ethernet-observed-window-contract-v1" \
    "phase12-rp1-ethernet-observed-window-discriminator-contract-v1" \
    "phase12-rp1-ethernet-observed-window-contract-20260610" \
    "phase12-rp1-ethernet-gem-mid-source-contract-20260609" \
    "report-kind=" \
    "candidate" \
    "hardware-proof-limited-to-observed-window-gem-mid-discriminator-control-output" \
    "same-run-required=true" \
    "material-difference-from-translated-window=true" \
    "observed-positive-control-register=SYSINFO_CHIP_ID" \
    "observed-positive-control-cpu-physical-target=" \
    "observed-positive-control-expected=" \
    "observed-positive-control-raw=" \
    "observed-positive-control-matches-expected=" \
    "observed-positive-control-is-deaddead=" \
    "raspberrypi,rp1-gem,cdns,macb" \
    "controller=rp1_eth" \
    "register=MACB_MID" \
    "source-offset-from-observed-rp1-base=" \
    "observed-rp1-base=" \
    "observed-window-cpu-physical-target=" \
    "translated-window-comparator-cpu-physical-target=" \
    "translated-window-comparator-role=comparator-sentinel-only" \
    "width=32" \
    "little-endian" \
    "read-only-volatile-load" \
    "raw=" \
    "idnum=" \
    "rev=" \
    "raw-is-zero=" \
    "raw-is-all-ones=" \
    "raw-is-deaddead=" \
    "observed-window-macb-mid-sentinel-with-sysinfo-positive-control" \
    "observed-window-macb-mid-visible" \
    "observed-window-positive-control-sentinel" \
    "inconclusive-capture" \
    "ethernet-driver-readiness" \
    "broad-live-ethernet-mmio-readiness" \
    "rp1-mmio-dma-programming" \
    "descriptor-rings" \
    "transfer-completion" \
    "interrupt-completion" \
    "packet-io" \
    "networking" \
    "sockets" \
    "ssh" \
    "phase-12-2" \
    "phase-transition" \
    "claims-ethernet-ready=false" \
    "claims-broad-mmio-ready=false" \
    "claims-rp1-mmio-dma-programming=false" \
    "claims-descriptor-rings=false" \
    "claims-transfer-completion=false" \
    "claims-interrupt-completion=false" \
    "claims-packet-io=false" \
    "classification="; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing RP1 Ethernet observed-window discriminator candidate string: $required" >&2
        exit 1
    fi
done

if [ -n "$CAPTURE_NONCE" ]; then
    if ! grep -Fq -- " capture-nonce=" "$kernel_strings"; then
        echo "kernel image missing RP1 Ethernet observed-window discriminator candidate capture nonce label" >&2
        exit 1
    fi
    if ! grep -Fq -- "$CAPTURE_NONCE" "$kernel_strings"; then
        echo "kernel image missing RP1 Ethernet observed-window discriminator candidate capture nonce: $CAPTURE_NONCE" >&2
        exit 1
    fi
fi

for forbidden in \
    "TALOS: rp1-ethernet-observed-window-discriminator-control" \
    "no-mmio-no-ethernet-rp1-ethernet-observed-window-control" \
    "observed-positive-control-register=none" \
    "observed-window-cpu-physical-target=not-constructed" \
    "access=not-constructed" \
    "classification=simulated/control"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden RP1 Ethernet observed-window discriminator candidate string: $forbidden" >&2
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
printf 'candidate_marker=%s\n' "$MARKER"
if [ -n "$CAPTURE_NONCE" ]; then
    printf 'capture_nonce=%s\n' "$CAPTURE_NONCE"
fi
printf 'forbidden_rp1_ethernet_observed_window_discriminator_candidate_runtime_strings_absent=true\n'
