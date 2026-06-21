#!/bin/sh
set -eu

EVIDENCE_DIR="tasks/evidence/2026-06-21-shell-operator-seed-diag-smoke"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-shell-operator-seed-diag-smoke.log"
RAW_LOG="$EVIDENCE_LOG.raw"
QEMU_TOOL_DIR="/opt/strider/openclaw/current/workspace/tools/qemu-9.2.0-install/bin"
CARGO_TOOL_DIR="$HOME/.cargo/bin"
WORKSPACE_TMP="/opt/strider/openclaw/current/workspace/tmp"

mkdir -p "$EVIDENCE_DIR"

if [ -d "$WORKSPACE_TMP" ]; then
    TMPDIR="$WORKSPACE_TMP"
    export TMPDIR
fi

if [ -d "$QEMU_TOOL_DIR" ]; then
    PATH="$QEMU_TOOL_DIR:$PATH"
    export PATH
fi

if [ -d "$CARGO_TOOL_DIR" ]; then
    PATH="$CARGO_TOOL_DIR:$PATH"
    export PATH
fi

run_filter() {
    filter="$1"
    echo "qemu-shell-operator-seed-diag-smoke: cargo-test-filter=$filter"
    cargo -Zjson-target-spec test --quiet "$filter"
}

if {
    echo "qemu-shell-operator-seed-diag-smoke: start"
    echo "qemu-shell-operator-seed-diag-smoke: substitute=target-cargo-test-via-qemu-runner"
    echo "qemu-shell-operator-seed-diag-smoke: boundary=diagnostic command dispatch over explicit read-only initramfs operator seed metadata"
    echo "qemu-shell-operator-seed-diag-smoke: missing-seed=entropydiag-fail-closed-no-input,entropydiag-operator-seed-required,sshkeydiag-seed-material-missing,cryptographic-strength=false,ssh-ready=false"
    echo "qemu-shell-operator-seed-diag-smoke: insufficient-seed=entropydiag-untrusted-local-mix,sshkeydiag-seed-material-insufficient,cryptographic-strength=false,ssh-ready=false"
    echo "qemu-shell-operator-seed-diag-smoke: sufficient-seed=entropydiag-untrusted-local-mix,no-seed-material-label,cryptographic-strength=false,ssh-ready=false"
    echo "qemu-shell-operator-seed-diag-smoke: redaction=no-seed-bytes,no-seed-digest,no-seed-fingerprint,no-secret-material"
    echo "qemu-shell-operator-seed-diag-smoke: non-goals=no-random-byte-generation,no-csprng-conditioning,no-host-key-generation,no-authorized-key-storage,no-writable-persistence,no-crypto-ssh-dependencies,no-ssh-service,no-live-packet-io,no-hardware-reachability,no-public-abi,no-phase-transition"
    run_filter "dispatcher_reports_operator_seed_missing_from_vfs_without_secret_material"
    run_filter "dispatcher_reports_operator_seed_insufficient_from_vfs_without_secret_material"
    run_filter "dispatcher_reports_operator_seed_sufficient_from_vfs_without_secret_material"
    echo "qemu-shell-operator-seed-diag-smoke: PASS classification=host-qemu-substitute-shell-operator-seed-diag-smoke-complete"
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
