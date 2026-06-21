#!/bin/sh
set -eu

EVIDENCE_DIR="tasks/evidence/2026-06-21-shell-ssh-host-keydiag-smoke"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-shell-ssh-host-keydiag-smoke.log"
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
    echo "qemu-shell-ssh-host-keydiag-smoke: cargo-test-filter=$filter"
    cargo -Zjson-target-spec test --quiet "$filter"
}

if {
    echo "qemu-shell-ssh-host-keydiag-smoke: start"
    echo "qemu-shell-ssh-host-keydiag-smoke: substitute=target-cargo-test-via-qemu-runner"
    echo "qemu-shell-ssh-host-keydiag-smoke: boundary=internal diagnostic command sshkeydiag over read-only VFS host-key metadata classifier"
    echo "qemu-shell-ssh-host-keydiag-smoke: missing-host-key=sshkeydiag-missing-host-key,sshkeydiag-missing-authorized-key,sshkeydiag-entropy-unready,sshkeydiag-seed-material-missing,ssh-ready=false"
    echo "qemu-shell-ssh-host-keydiag-smoke: invalid-host-key=sshkeydiag-host-key-invalid,ssh-ready=false"
    echo "qemu-shell-ssh-host-keydiag-smoke: insufficient-host-key=sshkeydiag-host-key-insufficient,ssh-ready=false"
    echo "qemu-shell-ssh-host-keydiag-smoke: sufficient-public-fixture=host-key-prerequisite-cleared,sshkeydiag-missing-authorized-key,sshkeydiag-entropy-unready,ssh-ready=false"
    echo "qemu-shell-ssh-host-keydiag-smoke: redaction=no-real-private-key,no-generated-key,no-derived-public-key,no-digest,no-fingerprint,no-signature,no-stable-secret-identifier"
    echo "qemu-shell-ssh-host-keydiag-smoke: non-goals=no-host-key-generation,no-key-parsing,no-authorized-key-storage,no-writable-persistence,no-ssh-service,no-live-transport,no-hardware-reachability,no-public-abi,no-phase-transition"
    run_filter "dispatcher_reports_operator_seed_missing_from_vfs_without_secret_material"
    run_filter "dispatcher_reports_host_key_metadata_invalid_insufficient_and_sufficient_from_vfs"
    run_filter "host_key_vfs_metadata_maps_to_fail_closed_states_without_reading_key_bytes"
    run_filter "host_key_vfs_metadata_clears_only_host_key_prerequisite"
    echo "qemu-shell-ssh-host-keydiag-smoke: PASS classification=host-qemu-substitute-shell-ssh-host-keydiag-smoke-complete"
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
