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

tmp_root="${TMPDIR:-target/tmp}"
mkdir -p "$tmp_root"
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-ssh-route-start-review.XXXXXX")"
cleanup() {
    rm -rf "$work_dir"
}
trap cleanup EXIT

./scripts/rpi5-archive-review.sh "$ARCHIVE"
tar -xzf "$ARCHIVE" -C "$work_dir"

strings "$work_dir/kernel_2712.img" > "$work_dir/kernel_2712.strings"

required_marker="TALOS: ssh-service-smoltcp-runtime-route-start capture-nonce=$CAPTURE_NONCE"
for token in \
    "TALOS: asm_start" \
    "TALOS: asm_pre_rust_entry" \
    "TALOS: ssh-service-smoltcp-runtime-route-start" \
    "capture-nonce=" \
    "$CAPTURE_NONCE" \
    "selected-normal-runtime-route-start=true" \
    "claims-runtime-ready=false" \
    "claims-packet-io=false" \
    "claims-service-success=false" \
    "claims-ssh-ready=false" \
    "claims-phase-transition=false" \
    "TALOS: ssh-service-smoltcp-runtime-ready" \
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
    "ssh-ready="
do
    if ! grep -Fq "$token" "$work_dir/kernel_2712.strings"; then
        echo "missing route-start marker loop token in kernel_2712.img: $token" >&2
        exit 1
    fi
done

printf 'route_start_marker_loop=ready\n'
printf 'required_marker=%s\n' "$required_marker"
printf 'archive=%s\n' "$ARCHIVE"
