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

route_start_token="TALOS: ssh-service-smoltcp-runtime-route-start"
runtime_blocked_token="TALOS: ssh-service-smoltcp-runtime-blocked"
route_start_marker="$route_start_token capture-nonce=$CAPTURE_NONCE"
runtime_blocked_marker="$runtime_blocked_token capture-nonce=$CAPTURE_NONCE"
required_marker="TALOS: ssh-service-smoltcp-runtime-ready capture-nonce=$CAPTURE_NONCE"
for token in \
    "TALOS: asm_start" \
    "TALOS: asm_pre_rust_entry" \
    "talos: boot start" \
    "$route_start_token" \
    "$runtime_blocked_token" \
    "TALOS: ssh-service-smoltcp-runtime-ready" \
    "capture-nonce=" \
    "$CAPTURE_NONCE" \
    "source=network-device-smoltcp-runtime provider-route-entry=source-bound-rp1 claims-runtime-ready=false" \
    "runtime-binding=accepted-deterministic-device-interface-delivery" \
    "runtime-binding=error live-packet-io-accepted=false live-reachability-accepted=false remote-receipt-accepted=false compatibility-accepted=false ssh-ready=false claims-service-success=false claims-phase-transition=false" \
    "descriptor-facing-connection-delivered=" \
    "deterministic-device-interface-bound=" \
    "hardware-frame-provider-bound=" \
    "hardware-frame-provider-classification=" \
    "rp1-ethernet-hardware-frame-provider-source-bound-local-only" \
    "no-rp1-ethernet-hardware-frame-provider-bound" \
    "rp1-ethernet-hardware-frame-provider-link-not-ready" \
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
printf 'early_marker_hierarchy=%s\n' "asm_start|asm_pre_rust_entry|rust_entry|boot info parsed|target init|exceptions ready|kernel_main|report_boot_identity|route_start|runtime_blocked|runtime_ready"
printf 'route_start_marker=%s\n' "$route_start_marker"
printf 'runtime_blocked_marker=%s\n' "$runtime_blocked_marker"
printf 'required_marker=%s\n' "$required_marker"
printf 'marker_family=%s|%s|%s\n' \
    "$route_start_marker" \
    "$runtime_blocked_marker" \
    "$required_marker"
printf 'archive=%s\n' "$ARCHIVE"
