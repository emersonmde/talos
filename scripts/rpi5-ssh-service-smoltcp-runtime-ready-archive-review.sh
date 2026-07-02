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
    "TALOS: kernel_main" \
    "$route_start_token" \
    "$runtime_blocked_token" \
    "TALOS: ssh-service-smoltcp-runtime-ready" \
    "capture-nonce=" \
    "$CAPTURE_NONCE" \
    "selected-kernel-entry-retention=v78" \
    "source=runtime-ready-route-retention-loop" \
    "claims-bootinfo-parsed=true" \
    "claims-target-init=true" \
    "claims-exceptions-ready=true" \
    "claims-route-start=true" \
    "source=network-device-smoltcp-runtime provider-route-entry=source-bound-rp1" \
    "retention-replay=true" \
    "retention-replay=" \
    "claims-runtime-ready=false" \
    "runtime-binding=accepted-deterministic-device-interface-delivery" \
    "runtime-binding=error retention-replay=true live-packet-io-accepted=false live-reachability-accepted=false remote-receipt-accepted=false compatibility-accepted=false ssh-ready=false claims-service-success=false claims-phase-transition=false" \
    "descriptor-facing-connection-delivered=" \
    "deterministic-device-interface-bound=" \
    "hardware-frame-provider-bound=" \
    "hardware-frame-provider-classification=" \
    "rp1-ethernet-hardware-frame-provider-source-bound-local-only" \
    "no-rp1-ethernet-hardware-frame-provider-bound" \
    "rp1-ethernet-hardware-frame-provider-link-not-ready" \
    "driver-packet-rx-frames=" \
    "driver-packet-tx-frames=" \
    "live-packet-ingress-discriminator=" \
    "blocked-no-live-frame-provider" \
    "live-frame-provider-owner=source-owned-rp1-dma-rx-descriptor-ring-metadata-only" \
    "dma-rx-descriptor-ring-owner=talos-rp1-ethernet-driver-source-model" \
    "dma-rx-redaction=metadata-only-no-packet-payloads-no-remote-identifiers-no-ssh-material" \
    "packet-stimulus-owner=bounded-lab-network-icmp-echo-contract" \
    "packet-stimulus-contract-id=" \
    "phase12-bounded-packet-stimulus-contract-v1" \
    "packet-stimulus-classification=" \
    "bounded-packet-stimulus-contract-ready" \
    "packet-stimulus-source=" \
    "lab-network-peer-icmp-echo-to-documented-talos-pi5-target" \
    "packet-stimulus-nonce-strategy=" \
    "run-unique-ascii-nonce-in-icmp-echo-payload-retain-only-sha256-and-length" \
    "packet-stimulus-redaction=" \
    "retain-protocol-length-nonce-sha256-and-descriptor-metadata-no-payload-bytes" \
    "packet-stimulus-timing-window=" \
    "after-runtime-ready-marker-and-serial-cursor-before-final-pre-restore-identity" \
    "packet-stimulus-descriptor-handoff-ready=" \
    "packet-stimulus-host-only-discriminator=" \
    "deterministic-driver-packet-adapter-host-only" \
    "packet-stimulus-distinguishes-host-only=" \
    "packet-stimulus-payload-retained=" \
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
printf 'early_marker_hierarchy=%s\n' "asm_start|asm_pre_rust_entry|rust_entry|boot info parsed|target init|exceptions ready|kernel_main_retention|report_boot_identity|route_start_retention|runtime_blocked_retention|runtime_ready_retention"
printf 'route_start_marker=%s\n' "$route_start_marker"
printf 'runtime_blocked_marker=%s\n' "$runtime_blocked_marker"
printf 'required_marker=%s\n' "$required_marker"
printf 'retention_contract=%s\n' "selected-kernel-entry-retention-v78"
printf 'marker_family=%s|%s|%s\n' \
    "$route_start_marker" \
    "$runtime_blocked_marker" \
    "$required_marker"
printf 'archive=%s\n' "$ARCHIVE"
