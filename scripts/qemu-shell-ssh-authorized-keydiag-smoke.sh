#!/bin/sh
set -eu

EVIDENCE_DIR="tasks/evidence/2026-06-21-shell-ssh-authorized-keydiag-smoke"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-shell-ssh-authorized-keydiag-smoke.log"
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
    echo "qemu-shell-ssh-authorized-keydiag-smoke: cargo-test-filter=$filter"
    cargo -Zjson-target-spec test --quiet "$filter"
}

if {
    echo "qemu-shell-ssh-authorized-keydiag-smoke: start"
    echo "qemu-shell-ssh-authorized-keydiag-smoke: substitute=target-cargo-test-via-qemu-runner"
    echo "qemu-shell-ssh-authorized-keydiag-smoke: boundary=internal diagnostic command sshkeydiag over read-only VFS authorized-key metadata classifier"
    echo "qemu-shell-ssh-authorized-keydiag-smoke: missing-authorized-key=sshkeydiag-missing-authorized-key,sshkeydiag-entropy-unready,ssh-ready=false"
    echo "qemu-shell-ssh-authorized-keydiag-smoke: invalid-authorized-key=sshkeydiag-authorized-key-invalid,ssh-ready=false"
    echo "qemu-shell-ssh-authorized-keydiag-smoke: insufficient-authorized-key=sshkeydiag-authorized-key-insufficient,ssh-ready=false"
    echo "qemu-shell-ssh-authorized-keydiag-smoke: sufficient-public-fixture=authorized-key-prerequisite-cleared,sshkeydiag-entropy-unready,sshkeydiag-persistence-unavailable,sshkeydiag-exposure-disabled,ssh-ready=false"
    echo "qemu-shell-ssh-authorized-keydiag-smoke: redaction=no-real-authorized-public-key,no-operator-identity,no-key-derived-identifier,no-digest,no-fingerprint,no-signature,no-private-key,no-generated-key"
    echo "qemu-shell-ssh-authorized-keydiag-smoke: non-goals=no-authorized-key-parsing,no-public-key-validation,no-authentication,no-writable-persistence,no-ssh-service,no-live-transport,no-hardware-reachability,no-public-abi,no-phase-transition"
    run_filter "dispatcher_reports_authorized_key_metadata_invalid_insufficient_and_sufficient_from_vfs"
    run_filter "authorized_key_vfs_metadata_maps_to_fail_closed_states_without_reading_key_bytes"
    run_filter "authorized_key_vfs_metadata_clears_only_authorized_key_prerequisite"
    echo "qemu-shell-ssh-authorized-keydiag-smoke: PASS classification=host-qemu-substitute-shell-ssh-authorized-keydiag-smoke-complete"
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
