#!/bin/sh
set -eu

EVIDENCE_DIR="tasks/evidence/2026-06-20-ping-operation-syscall-substitute-smoke"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-ping-operation-syscall-substitute-smoke.log"
RAW_LOG="$EVIDENCE_LOG.raw"
TEST_FILTER="ping_operation_syscall_substitute"
QEMU_TOOL_DIR="/opt/strider/openclaw/current/workspace/tools/qemu-9.2.0-install/bin"

mkdir -p "$EVIDENCE_DIR"

if [ -d "$QEMU_TOOL_DIR" ]; then
    PATH="$QEMU_TOOL_DIR:$PATH"
    export PATH
fi

{
    echo "qemu-ping-operation-syscall-substitute-smoke: start"
    echo "qemu-ping-operation-syscall-substitute-smoke: substitute=target-cargo-test-via-qemu-runner"
    echo "qemu-ping-operation-syscall-substitute-smoke: lifecycle=open -> status-idle -> unresolved-arp-pending -> matching-arp-advances-to-icmp-transmit -> inflight-status -> matching-echo-reply-completes -> terminal-completed-status -> close"
    echo "qemu-ping-operation-syscall-substitute-smoke: boundary=PingOperationSyscallSubstitute over NetworkPingOperationDescriptorTable + UserspacePingOperation + SinglePingPacketService + fake NetworkDevice + caller-owned buffers"
    echo "qemu-ping-operation-syscall-substitute-smoke: retry-timeout-edge=caller-driven-arp-retry-budget-exhaustion-then-explicit-timeout"
    echo "qemu-ping-operation-syscall-substitute-smoke: descriptor-edges=invalid-descriptor, closed-descriptor, zero-capacity, busy-open"
    echo "qemu-ping-operation-syscall-substitute-smoke: error-edges=start-transmit-io-error, receive-io-error, pump-transmit-io-error"
    cargo -Zjson-target-spec test --quiet "$TEST_FILTER"
    echo "qemu-ping-operation-syscall-substitute-smoke: PASS classification=host-substitute-ping-operation-syscall-substitute-smoke-complete"
} 2>&1 | tee "$RAW_LOG"
tr -d '\r' <"$RAW_LOG" >"$EVIDENCE_LOG"
rm -f "$RAW_LOG"
