#!/bin/sh
set -eu

EVIDENCE_DIR="tasks/evidence/2026-06-22-shell-ssh-host-key-private-material-smoke"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-shell-ssh-host-key-private-material-smoke.log"
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
    echo "qemu-shell-ssh-host-key-private-material-smoke: cargo-test-filter=$filter"
    cargo -Zjson-target-spec test --quiet "$filter"
}

if {
    echo "qemu-shell-ssh-host-key-private-material-smoke: start"
    echo "qemu-shell-ssh-host-key-private-material-smoke: substitute=target-cargo-test-via-qemu-runner"
    echo "qemu-shell-ssh-host-key-private-material-smoke: boundary=internal diagnostic command sshkeydiag plus focused loader/signing tests over read-only VFS host-key private-material classifier"
    echo "qemu-shell-ssh-host-key-private-material-smoke: missing-host-key=sshkeydiag-missing-host-key,sshkeydiag-missing-authorized-key,sshkeydiag-entropy-unready,ssh-ready=false"
    echo "qemu-shell-ssh-host-key-private-material-smoke: invalid-host-key=non-regular-empty-oversized-malformed-encrypted-unsupported,sshkeydiag-host-key-invalid,ssh-ready=false"
    echo "qemu-shell-ssh-host-key-private-material-smoke: insufficient-host-key=sshkeydiag-host-key-insufficient,ssh-ready=false"
    echo "qemu-shell-ssh-host-key-private-material-smoke: sufficient-public-fixture=host-key-private-material-prerequisite-cleared,sshkeydiag-missing-authorized-key,sshkeydiag-entropy-unready,sshkeydiag-persistence-unavailable,sshkeydiag-exposure-disabled,ssh-ready=false"
    echo "qemu-shell-ssh-host-key-private-material-smoke: signing-api=public-fixture-loads-and-signs-caller-owned-exchange-hash,in-memory-ephemeral-signature-only"
    echo "qemu-shell-ssh-host-key-private-material-smoke: redaction=no-real-private-key,no-private-bytes,no-signature-bytes,no-fingerprint,no-digest,no-random-bytes,no-shared-secret,no-operator-identity,no-stable-identifier,no-key-derived-identifier"
    echo "qemu-shell-ssh-host-key-private-material-smoke: non-goals=no-runtime-kex,no-encryption-mac,no-newkeys,no-authentication,no-session,no-shell-attachment,no-live-transport,no-hardware,no-boot-publication,no-reachability,no-compatibility,no-phase-transition"
    run_filter "dispatcher_reports_host_key_metadata_invalid_insufficient_and_sufficient_from_vfs"
    run_filter "host_key_private_material_maps_to_fail_closed_states"
    run_filter "host_key_private_material_clears_only_host_key_prerequisite"
    run_filter "host_key_private_material_loads_and_signs_public_fixture"
    run_filter "host_key_private_material_loader_rejects_nonaccepted_inputs"
    echo "qemu-shell-ssh-host-key-private-material-smoke: PASS classification=host-qemu-substitute-shell-ssh-host-key-private-material-smoke-complete"
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
