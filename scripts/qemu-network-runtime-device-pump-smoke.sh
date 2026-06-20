#!/bin/sh
set -eu

EVIDENCE_DIR="tasks/evidence/2026-06-20-network-runtime-device-pump-substitute-smoke"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-network-runtime-device-pump-smoke.log"
RAW_LOG="$EVIDENCE_LOG.raw"
TEST_FILTER="network_runtime_device_pump"
QEMU_TOOL_DIR="/opt/strider/openclaw/current/workspace/tools/qemu-9.2.0-install/bin"

mkdir -p "$EVIDENCE_DIR"

if [ -d "$QEMU_TOOL_DIR" ]; then
    PATH="$QEMU_TOOL_DIR:$PATH"
    export PATH
fi

{
    echo "qemu-network-runtime-device-pump-smoke: start"
    echo "qemu-network-runtime-device-pump-smoke: substitute=target-cargo-test-via-qemu-runner"
    echo "qemu-network-runtime-device-pump-smoke: boundary=NetworkRuntimeDevicePump over fake NetworkDevice + local ARP/ICMP responder + NetworkPingOperationDescriptorTable + UserspacePingOperation + SinglePingPacketService + caller-owned buffers"
    echo "qemu-network-runtime-device-pump-smoke: local-replies=local-arp-reply-transmit, local-icmp-echo-reply-transmit, responder-priority-before-active-ping"
    echo "qemu-network-runtime-device-pump-smoke: active-ping-lifecycle=open -> start-unresolved-arp -> matching-arp-advances-to-icmp-transmit -> echo-reply-completes -> terminal-completed-status -> close"
    echo "qemu-network-runtime-device-pump-smoke: retry-timeout-edge=caller-driven-arp-retry-budget-exhaustion-then-explicit-timeout"
    echo "qemu-network-runtime-device-pump-smoke: receive-transmit-buffer-edges=no-frame, receive-buffer-too-small, receive-io-error, local-transmit-io-error, active-transmit-io-error"
    cargo -Zjson-target-spec test --quiet "$TEST_FILTER"
    echo "qemu-network-runtime-device-pump-smoke: PASS classification=host-substitute-network-runtime-device-pump-smoke-complete"
} 2>&1 | tee "$RAW_LOG"
tr -d '\r' <"$RAW_LOG" >"$EVIDENCE_LOG"
rm -f "$RAW_LOG"
