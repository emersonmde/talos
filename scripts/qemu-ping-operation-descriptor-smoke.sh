#!/bin/sh
set -eu

EVIDENCE_DIR="tasks/evidence/2026-06-20-ping-operation-descriptor-substitute-smoke"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-ping-operation-descriptor-smoke.log"
RAW_LOG="$EVIDENCE_LOG.raw"
TEST_FILTER="network_ping_descriptor"
QEMU_TOOL_DIR="/opt/strider/openclaw/current/workspace/tools/qemu-9.2.0-install/bin"

mkdir -p "$EVIDENCE_DIR"

if [ -d "$QEMU_TOOL_DIR" ]; then
    PATH="$QEMU_TOOL_DIR:$PATH"
    export PATH
fi

{
    echo "qemu-ping-operation-descriptor-smoke: start"
    echo "qemu-ping-operation-descriptor-smoke: substitute=target-cargo-test-via-qemu-runner"
    echo "qemu-ping-operation-descriptor-smoke: lifecycle=descriptor-open -> unresolved-arp-pending -> matching-arp-advances-to-icmp-transmit -> inflight-recorded -> matching-echo-reply-completes -> terminal-completed-status -> descriptor-close"
    echo "qemu-ping-operation-descriptor-smoke: boundary=NetworkPingOperationDescriptorTable start/pump/status/retry/timeout/close over UserspacePingOperation + SinglePingPacketService + fake NetworkDevice"
    echo "qemu-ping-operation-descriptor-smoke: retry-timeout-edge=caller-driven-arp-retry-budget-exhaustion-then-explicit-timeout"
    echo "qemu-ping-operation-descriptor-smoke: descriptor-edges=invalid-descriptor, closed-descriptor, zero-capacity, busy-open"
    echo "qemu-ping-operation-descriptor-smoke: error-edges=transmit-io-error, receive-io-error"
    cargo -Zjson-target-spec test --quiet "$TEST_FILTER"
    echo "qemu-ping-operation-descriptor-smoke: PASS classification=host-substitute-ping-operation-descriptor-smoke-complete"
} 2>&1 | tee "$RAW_LOG"
tr -d '\r' <"$RAW_LOG" >"$EVIDENCE_LOG"
rm -f "$RAW_LOG"
