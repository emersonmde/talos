#!/bin/sh
set -eu

EVIDENCE_DIR="tasks/evidence/2026-06-20-process-local-ping-svc-dispatch-smoke"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-process-local-ping-svc-dispatch-smoke.log"
RAW_LOG="$EVIDENCE_LOG.raw"
TEST_FILTER="process_local_ping_dispatch"
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
    echo "qemu-process-local-ping-svc-dispatch-smoke: start"
    echo "qemu-process-local-ping-svc-dispatch-smoke: substitute=target-cargo-test-via-qemu-runner"
    echo "qemu-process-local-ping-svc-dispatch-smoke: boundary=dispatch_process_local_ping_descriptor_operation over ProcessLocalPingDescriptorControl + ProcessDescriptorStore + DescriptorShapedPingControl + RuntimePingOperationSyscallSubstitute + NetworkRuntimeDevicePump + fake NetworkDevice + caller-owned buffers"
    echo "qemu-process-local-ping-svc-dispatch-smoke: lifecycle=open -> start-unresolved-arp-pending -> pump-arp-advances-to-inflight -> pump-echo-reply-completes -> terminal-completed-status -> close"
    echo "qemu-process-local-ping-svc-dispatch-smoke: dispatch-operations=open,start,pump_or_read_result,status,retry_arp,timeout,close"
    echo "qemu-process-local-ping-svc-dispatch-smoke: descriptor-edges=invalid-descriptor,closed-descriptor,missing-owner,full-process-descriptor-table-unwinds-backing-descriptor,duplicate-active-operation"
    echo "qemu-process-local-ping-svc-dispatch-smoke: retry-timeout-edges=retry-exhaustion,explicit-timeout"
    echo "qemu-process-local-ping-svc-dispatch-smoke: buffer-error-edges=caller-receive-buffer-pressure"
    echo "qemu-process-local-ping-svc-dispatch-smoke: io-error-edges=receive-io-error,local-transmit-io-error,active-transmit-io-error"
    cargo -Zjson-target-spec test --quiet "$TEST_FILTER"
    echo "qemu-process-local-ping-svc-dispatch-smoke: PASS classification=host-substitute-process-local-ping-svc-dispatch-smoke-complete"
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
