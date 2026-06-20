#!/bin/sh
set -eu

EVIDENCE_DIR="tasks/evidence/2026-06-20-descriptor-shaped-ping-control-smoke"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-descriptor-shaped-ping-control-smoke.log"
RAW_LOG="$EVIDENCE_LOG.raw"
TEST_FILTER="descriptor_shaped_ping_control"
QEMU_TOOL_DIR="/opt/strider/openclaw/current/workspace/tools/qemu-9.2.0-install/bin"
CARGO_TOOL_DIR="$HOME/.cargo/bin"

mkdir -p "$EVIDENCE_DIR"

if [ -d "$QEMU_TOOL_DIR" ]; then
    PATH="$QEMU_TOOL_DIR:$PATH"
    export PATH
fi

if [ -d "$CARGO_TOOL_DIR" ]; then
    PATH="$CARGO_TOOL_DIR:$PATH"
    export PATH
fi

if {
    echo "qemu-descriptor-shaped-ping-control-smoke: start"
    echo "qemu-descriptor-shaped-ping-control-smoke: substitute=target-cargo-test-via-qemu-runner"
    echo "qemu-descriptor-shaped-ping-control-smoke: boundary=DescriptorShapedPingControl over RuntimePingOperationSyscallSubstitute + NetworkRuntimeDevicePump + fake NetworkDevice + caller-owned buffers"
    echo "qemu-descriptor-shaped-ping-control-smoke: lifecycle=open -> status-idle -> start-unresolved-arp-pending -> pump-arp-advances-to-inflight -> pump-echo-reply-completes -> terminal-completed-status -> close"
    echo "qemu-descriptor-shaped-ping-control-smoke: descriptor-edges=invalid-descriptor, closed-descriptor, zero-capacity, duplicate-active-open"
    echo "qemu-descriptor-shaped-ping-control-smoke: buffer-retry-timeout-edges=caller-receive-buffer-pressure, retry-exhaustion, explicit-timeout"
    echo "qemu-descriptor-shaped-ping-control-smoke: error-edges=receive-io-error, local-transmit-io-error, active-ping-transmit-io-error"
    cargo -Zjson-target-spec test --quiet "$TEST_FILTER"
    echo "qemu-descriptor-shaped-ping-control-smoke: PASS classification=host-substitute-descriptor-shaped-ping-control-smoke-complete"
} >"$RAW_LOG" 2>&1; then
    cat "$RAW_LOG"
    tr -d '\r' <"$RAW_LOG" >"$EVIDENCE_LOG"
    rm -f "$RAW_LOG"
else
    status=$?
    cat "$RAW_LOG"
    tr -d '\r' <"$RAW_LOG" >"$EVIDENCE_LOG"
    rm -f "$RAW_LOG"
    exit "$status"
fi
