#!/bin/sh
set -eu

EVIDENCE_DIR="tasks/evidence/2026-06-22-shell-ssh-persistence-exposure-diag-smoke"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-shell-ssh-persistence-exposure-diag-smoke.log"
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
    echo "qemu-shell-ssh-persistence-exposure-diag-smoke: cargo-test-filter=$filter"
    cargo -Zjson-target-spec test --quiet "$filter"
}

if {
    echo "qemu-shell-ssh-persistence-exposure-diag-smoke: start"
    echo "qemu-shell-ssh-persistence-exposure-diag-smoke: substitute=target-cargo-test-via-qemu-runner"
    echo "qemu-shell-ssh-persistence-exposure-diag-smoke: boundary=internal diagnostic command sshkeydiag over read-only VFS persistence and exposure metadata classifiers"
    echo "qemu-shell-ssh-persistence-exposure-diag-smoke: default-disabled-exposure=sshkeydiag-persistence-unavailable,sshkeydiag-exposure-disabled,sshkeydiag-not-ready,ssh-ready=false"
    echo "qemu-shell-ssh-persistence-exposure-diag-smoke: missing-exposure-marker=sufficient-public-fixture-persistence-metadata,sshkeydiag-exposure-disabled,sshkeydiag-not-ready,ssh-ready=false"
    echo "qemu-shell-ssh-persistence-exposure-diag-smoke: invalid-exposure-marker=sshkeydiag-exposure-disabled,sshkeydiag-not-ready,ssh-ready=false"
    echo "qemu-shell-ssh-persistence-exposure-diag-smoke: sufficient-public-fixture=persistence-unavailable-cleared,exposure-disabled-cleared,sshkeydiag-not-ready,ssh-ready=false"
    echo "qemu-shell-ssh-persistence-exposure-diag-smoke: redaction=no-real-operator-seed,no-real-host-private-key,no-real-authorized-public-key,no-generated-key,no-generated-random-bytes,no-private-csprng-state,no-operator-identity,no-key-derived-identifier,no-digest,no-fingerprint,no-signature"
    echo "qemu-shell-ssh-persistence-exposure-diag-smoke: non-goals=no-ssh-service,no-listener,no-authentication,no-pty-session,no-live-transport,no-packet-io,no-hardware,no-boot-publication,no-reachability,no-writable-persistence,no-public-abi,no-phase-transition"
    run_filter "dispatcher_reports_ssh_key_readiness_fail_closed_without_secret_material"
    run_filter "dispatcher_reports_authorized_key_metadata_invalid_insufficient_and_sufficient_from_vfs"
    run_filter "dispatcher_reports_persistence_exposure_metadata_without_secret_material"
    run_filter "persistence_metadata_requires_all_accepted_material_sources"
    run_filter "exposure_marker_vfs_metadata_fails_closed_until_explicit_marker_is_valid"
    run_filter "persistence_and_exposure_metadata_clear_only_their_prerequisites"
    echo "qemu-shell-ssh-persistence-exposure-diag-smoke: PASS classification=host-qemu-substitute-shell-ssh-persistence-exposure-diag-smoke-complete"
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
