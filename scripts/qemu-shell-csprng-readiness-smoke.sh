#!/bin/sh
set -eu

EVIDENCE_DIR="tasks/evidence/2026-06-21-shell-csprng-readiness-smoke"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-shell-csprng-readiness-smoke.log"
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
    echo "qemu-shell-csprng-readiness-smoke: cargo-test-filter=$filter"
    cargo -Zjson-target-spec test --quiet "$filter"
}

if {
    echo "qemu-shell-csprng-readiness-smoke: start"
    echo "qemu-shell-csprng-readiness-smoke: substitute=target-cargo-test-via-qemu-runner"
    echo "qemu-shell-csprng-readiness-smoke: boundary=internal csprng readiness plus entropy and sshkeydiag metadata classifiers"
    echo "qemu-shell-csprng-readiness-smoke: missing-seed=csprng-missing-seed,entropydiag-operator-seed-required,sshkeydiag-seed-material-missing,cryptographic-strength=false,ssh-ready=false"
    echo "qemu-shell-csprng-readiness-smoke: insufficient-seed=csprng-insufficient-seed,sshkeydiag-seed-material-insufficient,cryptographic-strength=false,ssh-ready=false"
    echo "qemu-shell-csprng-readiness-smoke: sufficient-public-fixture=csprng-ready,cryptographic-strength=true,sshkeydiag-not-ready,ssh-ready=false"
    echo "qemu-shell-csprng-readiness-smoke: redaction=no-seed-bytes,no-generated-bytes,no-digest,no-fingerprint,no-stream-identifier,no-secret-material"
    echo "qemu-shell-csprng-readiness-smoke: non-goals=no-host-key-generation,no-authorized-key-storage,no-writable-persistence,no-ssh-service,no-live-transport,no-hardware-reachability,no-public-abi,no-phase-transition"
    run_filter "missing_invalid_and_insufficient_operator_seeds_fail_closed"
    run_filter "not_ready_fill_zeroizes_caller_buffer_without_partial_output"
    run_filter "public_fixture_seed_initializes_bounded_deterministic_output"
    run_filter "ready_csprng_metadata_sets_crypto_strength_without_ssh_readiness"
    run_filter "csprng_ready_entropy_clears_only_entropy_prerequisite"
    echo "qemu-shell-csprng-readiness-smoke: PASS classification=host-qemu-substitute-shell-csprng-readiness-smoke-complete"
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
