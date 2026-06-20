#!/bin/sh
set -eu

EVIDENCE_DIR="tasks/evidence/2026-06-20-process-local-ping-svc-user-argument-smoke"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-process-local-ping-svc-user-argument-smoke.log"
RAW_LOG="$EVIDENCE_LOG.raw"
TEST_FILTER="process_local_ping_user_arguments"
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
    echo "qemu-process-local-ping-svc-user-argument-smoke: start"
    echo "qemu-process-local-ping-svc-user-argument-smoke: substitute=target-cargo-test-via-qemu-runner"
    echo "qemu-process-local-ping-svc-user-argument-smoke: boundary=dispatch_process_local_ping_descriptor_user_arguments over UserMapping + ProcessLocalPingDispatchOperation + ProcessLocalPingDescriptorControl + ProcessDescriptorStore + NetworkRuntimeDevicePump + fake NetworkDevice + caller-owned buffers"
    echo "qemu-process-local-ping-svc-user-argument-smoke: lifecycle=open -> idle-status-copy-out -> start-copied-user-payload -> pump-arp-to-icmp-result-copy-out -> echo-reply-completed -> completed-status-copy-out -> close"
    echo "qemu-process-local-ping-svc-user-argument-smoke: user-argument-operations=open,start,pump_or_read_result,status,retry_arp,timeout,close"
    echo "qemu-process-local-ping-svc-user-argument-smoke: user-memory=payload-copy-in,result-copy-out,status-copy-out,bounded-kernel-scratch,caller-owned-buffers"
    echo "qemu-process-local-ping-svc-user-argument-smoke: malformed-argument-edges=stable-syscall-vocabulary-unchanged,malformed-selector,reserved-fields,zero-ttl,invalid-route-prefix"
    echo "qemu-process-local-ping-svc-user-argument-smoke: descriptor-owner-capacity-edges=missing-owner,process-descriptor-capacity,invalid-descriptor"
    echo "qemu-process-local-ping-svc-user-argument-smoke: buffer-fault-edges=output-buffer-pressure,invalid-user-memory,scratch-pressure"
    cargo -Zjson-target-spec test --quiet "$TEST_FILTER"
    echo "qemu-process-local-ping-svc-user-argument-smoke: PASS classification=host-substitute-process-local-ping-svc-user-argument-smoke-complete"
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
