#!/bin/sh
set -eu

EVIDENCE_DIR="tasks/evidence/2026-06-20-driver-packet-pump-smoke"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-driver-packet-pump-smoke.log"
RAW_LOG="$EVIDENCE_LOG.raw"
QEMU_TOOL_DIR="/opt/strider/openclaw/current/workspace/tools/qemu-9.2.0-install/bin"

mkdir -p "$EVIDENCE_DIR"

if [ -d "$QEMU_TOOL_DIR" ]; then
    PATH="$QEMU_TOOL_DIR:$PATH"
    export PATH
fi

run_filter() {
    filter="$1"
    echo "qemu-driver-packet-pump-smoke: cargo-test-filter=$filter"
    cargo -Zjson-target-spec test --quiet "$filter"
}

{
    echo "qemu-driver-packet-pump-smoke: start"
    echo "qemu-driver-packet-pump-smoke: substitute=target-cargo-test-via-qemu-runner"
    echo "qemu-driver-packet-pump-smoke: boundary=PacketQueueNetworkDevice::pump_driver over trait-level NetworkDevice behavior plus VFS /bin/pingdiag diagnostic SVC lifecycle"
    echo "qemu-driver-packet-pump-smoke: lifecycle=/bin/pingdiag-open -> start-unresolved-arp -> pump-outbound-arp-to-driver -> inject-arp-reply-through-driver -> pump-outbound-icmp-to-driver -> inject-icmp-reply-through-driver -> completed-status -> close"
    echo "qemu-driver-packet-pump-smoke: positive-frames=outbound-arp-request,outbound-ipv4-icmp-echo-request,inbound-arp-reply,inbound-icmp-echo-reply"
    echo "qemu-driver-packet-pump-smoke: deterministic-controls=missing-vfs-identity,malformed-arguments,wrong-owner,invalid-or-closed-descriptor,queue-capacity,caller-buffer-pressure,malformed-received-frames,timeout-retry,transmit-receive-device-errors,close-drop,stable-syscall-vocabulary"
    run_filter "packet_queue_driver_pump"
    run_filter "vfs_ping_diagnostic_svc_fixture_records_packet_queue_lifecycle"
    run_filter "vfs_ping_diagnostic_svc_fixture_maps_packet_queue_controls"
    run_filter "vfs_ping_diagnostic_svc_fixture_maps_contract_error_controls"
    run_filter "process_local_ping_user_arguments_reject_malformed_owner_descriptor_capacity_and_buffers"
    echo "qemu-driver-packet-pump-smoke: PASS classification=host-substitute-driver-packet-pump-smoke-complete"
} 2>&1 | tee "$RAW_LOG"
tr -d '\r' <"$RAW_LOG" >"$EVIDENCE_LOG"
rm -f "$RAW_LOG"
