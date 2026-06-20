#!/bin/sh
set -eu

EVIDENCE_DIR="tasks/evidence/2026-06-20-runtime-ping-syscall-substitute-smoke"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-runtime-ping-syscall-substitute-smoke.log"
RAW_LOG="$EVIDENCE_LOG.raw"
TEST_FILTER="runtime_ping_syscall_substitute"
QEMU_TOOL_DIR="/opt/strider/openclaw/current/workspace/tools/qemu-9.2.0-install/bin"

mkdir -p "$EVIDENCE_DIR"

if [ -d "$QEMU_TOOL_DIR" ]; then
    PATH="$QEMU_TOOL_DIR:$PATH"
    export PATH
fi

{
    echo "qemu-runtime-ping-syscall-substitute-smoke: start"
    echo "qemu-runtime-ping-syscall-substitute-smoke: substitute=target-cargo-test-via-qemu-runner"
    echo "qemu-runtime-ping-syscall-substitute-smoke: boundary=RuntimePingOperationSyscallSubstitute over NetworkRuntimeDevicePump + local ARP/ICMP responder + active ping descriptor dispatch + UserspacePingOperation + SinglePingPacketService + fake NetworkDevice + caller-owned buffers"
    echo "qemu-runtime-ping-syscall-substitute-smoke: lifecycle=open -> status-idle -> start-unresolved-arp-pending -> runtime-pump-arp-advances-to-icmp-transmit -> inflight-status -> runtime-pump-echo-reply-completes -> terminal-completed-status -> close"
    echo "qemu-runtime-ping-syscall-substitute-smoke: local-responder=local-arp-reply-while-descriptor-open, local-icmp-echo-reply-while-descriptor-open"
    echo "qemu-runtime-ping-syscall-substitute-smoke: retry-timeout-edge=caller-driven-arp-retry-budget-exhaustion-then-explicit-timeout"
    echo "qemu-runtime-ping-syscall-substitute-smoke: descriptor-edges=invalid-descriptor, closed-descriptor, zero-capacity, busy-open"
    echo "qemu-runtime-ping-syscall-substitute-smoke: error-edges=receive-io-error, local-transmit-io-error, active-ping-transmit-io-error"
    cargo -Zjson-target-spec test --quiet "$TEST_FILTER"
    echo "qemu-runtime-ping-syscall-substitute-smoke: PASS classification=host-substitute-runtime-ping-syscall-substitute-smoke-complete"
} 2>&1 | tee "$RAW_LOG"
tr -d '\r' <"$RAW_LOG" >"$EVIDENCE_LOG"
rm -f "$RAW_LOG"
