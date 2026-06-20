#!/bin/sh
set -eu

EVIDENCE_DIR="tasks/evidence/2026-06-20-vfs-ping-diagnostic-svc-smoke"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-vfs-ping-diagnostic-svc-smoke.log"
RAW_LOG="$EVIDENCE_LOG.raw"
TEST_FILTER="vfs_ping_diagnostic_svc_fixture"
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
    echo "qemu-vfs-ping-diagnostic-svc-smoke: start"
    echo "qemu-vfs-ping-diagnostic-svc-smoke: substitute=target-cargo-test-via-qemu-runner"
    echo "qemu-vfs-ping-diagnostic-svc-smoke: boundary=VfsPingDiagnosticSvcFixture over ReadOnlyInitramfs + dispatch_process_local_ping_descriptor_user_arguments + UserMapping + ProcessDescriptorStore + NetworkRuntimeDevicePump + fake NetworkDevice + caller-owned buffers"
    echo "qemu-vfs-ping-diagnostic-svc-smoke: lifecycle=vfs-executable-lookup -> open -> idle-status-copy-out -> start-copied-diagnostic-payload -> pump-arp-to-icmp-result-copy-out -> echo-reply-completed -> completed-status-copy-out -> close"
    echo "qemu-vfs-ping-diagnostic-svc-smoke: user-argument-operations=open,start,pump_or_read_result,status,retry_arp,timeout,close"
    echo "qemu-vfs-ping-diagnostic-svc-smoke: user-memory=payload-copy-in,result-copy-out,status-copy-out,bounded-kernel-scratch,caller-owned-buffers"
    echo "qemu-vfs-ping-diagnostic-svc-smoke: diagnostic-vfs-controls=missing-executable,stable-syscall-vocabulary-unchanged"
    echo "qemu-vfs-ping-diagnostic-svc-smoke: malformed-argument-edges=malformed-selector,malformed-payload,zero-ttl"
    echo "qemu-vfs-ping-diagnostic-svc-smoke: descriptor-owner-lifetime-capacity-edges=missing-owner,process-descriptor-capacity,invalid-descriptor,closed-descriptor"
    echo "qemu-vfs-ping-diagnostic-svc-smoke: buffer-fault-edges=output-buffer-pressure,invalid-user-memory,scratch-pressure,caller-receive-buffer-pressure"
    echo "qemu-vfs-ping-diagnostic-svc-smoke: network-control-edges=retry-exhaustion,explicit-timeout,device-receive-io-error"
    cargo -Zjson-target-spec test --quiet "$TEST_FILTER"
    echo "qemu-vfs-ping-diagnostic-svc-smoke: PASS classification=host-substitute-vfs-ping-diagnostic-svc-smoke-complete"
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
