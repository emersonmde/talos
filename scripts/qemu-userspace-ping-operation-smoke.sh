#!/bin/sh
set -eu

EVIDENCE_DIR="tasks/evidence/2026-06-19-userspace-ping-operation-substitute-smoke"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-userspace-ping-operation-smoke.log"
RAW_LOG="$EVIDENCE_LOG.raw"
TEST_FILTER="userspace_ping_operation"
QEMU_TOOL_DIR="/opt/strider/openclaw/current/workspace/tools/qemu-9.2.0-install/bin"

mkdir -p "$EVIDENCE_DIR"

if [ -d "$QEMU_TOOL_DIR" ]; then
    PATH="$QEMU_TOOL_DIR:$PATH"
    export PATH
fi

{
    echo "qemu-userspace-ping-operation-smoke: start"
    echo "qemu-userspace-ping-operation-smoke: substitute=target-cargo-test-via-qemu-runner"
    echo "qemu-userspace-ping-operation-smoke: lifecycle=unresolved-arp-pending -> matching-arp-advances-to-icmp-transmit -> inflight-recorded -> matching-echo-reply-completes -> terminal-completed-status"
    echo "qemu-userspace-ping-operation-smoke: boundary=start/pump/status/retry/timeout over UserspacePingOperation + SinglePingPacketService + fake NetworkDevice"
    echo "qemu-userspace-ping-operation-smoke: retry-timeout-edge=caller-driven-arp-retry-budget-exhaustion-then-explicit-timeout"
    echo "qemu-userspace-ping-operation-smoke: error-edges=busy-start, transmit-io-error, receive-io-error"
    cargo -Zjson-target-spec test --quiet "$TEST_FILTER"
    echo "qemu-userspace-ping-operation-smoke: PASS classification=host-substitute-userspace-ping-operation-smoke-complete"
} 2>&1 | tee "$RAW_LOG"
tr -d '\r' <"$RAW_LOG" >"$EVIDENCE_LOG"
rm -f "$RAW_LOG"
