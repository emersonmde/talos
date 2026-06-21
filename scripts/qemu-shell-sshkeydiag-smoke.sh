#!/bin/sh
set -eu

EVIDENCE_DIR="tasks/evidence/2026-06-21-shell-sshkeydiag-smoke"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-shell-sshkeydiag-smoke.log"
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
    echo "qemu-shell-sshkeydiag-smoke: cargo-test-filter=$filter"
    cargo -Zjson-target-spec test --quiet "$filter"
}

if {
    echo "qemu-shell-sshkeydiag-smoke: start"
    echo "qemu-shell-sshkeydiag-smoke: substitute=target-cargo-test-via-qemu-runner"
    echo "qemu-shell-sshkeydiag-smoke: boundary=internal diagnostic command sshkeydiag over explicit metadata-only fail-closed SSH key-readiness classifier"
    echo "qemu-shell-sshkeydiag-smoke: expected-labels=sshkeydiag-not-ready,sshkeydiag-missing-host-key,sshkeydiag-missing-authorized-key,sshkeydiag-entropy-unready,sshkeydiag-seed-material-missing,sshkeydiag-persistence-unavailable,sshkeydiag-exposure-disabled,ssh-ready=false"
    echo "qemu-shell-sshkeydiag-smoke: entropy-boundary=entropydiag-fail-closed-no-input,entropydiag-hardware-rng-unaccepted,entropydiag-operator-seed-required,cryptographic-strength=false,ssh-ready=false"
    echo "qemu-shell-sshkeydiag-smoke: non-goals=no-key-generation,no-secret-persistence,no-crypto-ssh-dependencies,no-ssh-service,no-live-packet-io,no-hardware-reachability,no-public-abi,no-phase-transition"
    run_filter "dispatcher_reports_ssh_key_readiness_fail_closed_without_secret_material"
    run_filter "dispatcher_reports_entropy_diagnostic_fail_closed_without_crypto_claim"
    run_filter "all_missing_default_reports_every_fail_closed_label"
    echo "qemu-shell-sshkeydiag-smoke: PASS classification=host-qemu-substitute-shell-sshkeydiag-fail-closed-smoke-complete"
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
