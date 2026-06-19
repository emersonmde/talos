#!/bin/sh
set -eu

EVIDENCE_DIR="tasks/evidence/2026-06-19-qemu-single-ping-transaction-smoke"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-single-ping-transaction-smoke.log"
RAW_LOG="$EVIDENCE_LOG.raw"
TEST_NAME="qemu_substitute_single_ping_transaction_smoke_covers_lifecycle_and_retry_timeout"

mkdir -p "$EVIDENCE_DIR"

{
    echo "qemu-single-ping-transaction-smoke: start"
    echo "qemu-single-ping-transaction-smoke: substitute=host-cargo-test"
    echo "qemu-single-ping-transaction-smoke: lifecycle=unresolved-arp-pending -> matching-arp-advances-to-icmp-transmit -> inflight-recorded -> matching-echo-reply-completes -> idle"
    echo "qemu-single-ping-transaction-smoke: retry-timeout-edge=caller-driven-arp-retry-budget-exhaustion-then-pending-timeout"
    cargo -Zjson-target-spec test --quiet "$TEST_NAME"
    echo "qemu-single-ping-transaction-smoke: PASS classification=host-substitute-single-ping-transaction-smoke-complete"
} 2>&1 | tee "$RAW_LOG"
tr -d '\r' <"$RAW_LOG" >"$EVIDENCE_LOG"
rm -f "$RAW_LOG"
