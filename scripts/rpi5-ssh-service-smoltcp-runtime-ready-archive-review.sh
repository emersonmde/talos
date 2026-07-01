#!/bin/sh
set -eu

if [ "$#" -lt 1 ] || [ "$#" -gt 2 ]; then
    echo "usage: $0 <talos-rpi5-boot.tar.gz> [capture-nonce]" >&2
    exit 2
fi

ARCHIVE="$1"
CAPTURE_NONCE="${2:-${TALOS_CAPTURE_NONCE:-runtime-marker-route-static}}"

case "$CAPTURE_NONCE" in
    ''|*[!A-Za-z0-9_.:-]*)
        echo "capture nonce may contain only A-Z, a-z, 0-9, _, ., :, and -" >&2
        exit 2
        ;;
esac

tmp_root="${TMPDIR:-/tmp}"
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-ssh-runtime-review.XXXXXX")"
cleanup() {
    rm -rf "$work_dir"
}
trap cleanup EXIT

./scripts/rpi5-archive-review.sh "$ARCHIVE"
tar -xzf "$ARCHIVE" -C "$work_dir"

strings "$work_dir/kernel_2712.img" > "$work_dir/kernel_2712.strings"

required_marker="TALOS: ssh-service-smoltcp-runtime-ready capture-nonce=$CAPTURE_NONCE"
for token in \
    "TALOS: asm_start" \
    "TALOS: asm_pre_rust_entry" \
    "TALOS: ssh-service-smoltcp-runtime-route-start" \
    "TALOS: ssh-service-smoltcp-runtime-ready" \
    "capture-nonce=" \
    "$CAPTURE_NONCE" \
    "runtime-binding=accepted-deterministic-device-interface-delivery" \
    "descriptor-facing-connection-delivered=" \
    "deterministic-device-interface-bound=" \
    "hardware-frame-provider-bound=" \
    "driver-packet-rx-frames=" \
    "driver-packet-tx-frames=" \
    "live-packet-io-accepted=" \
    "live-reachability-accepted=" \
    "remote-receipt-accepted=" \
    "compatibility-accepted=" \
    "ssh-ready=" \
    "claims-service-success=false" \
    "claims-phase-transition=false"
do
    if ! grep -Fq "$token" "$work_dir/kernel_2712.strings"; then
        echo "missing runtime marker route token in kernel_2712.img: $token" >&2
        exit 1
    fi
done

printf 'runtime_marker_route=ready\n'
printf 'required_marker=%s\n' "$required_marker"
printf 'archive=%s\n' "$ARCHIVE"
