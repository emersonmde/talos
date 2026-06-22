#!/bin/sh
set -eu

EVIDENCE_DIR="tasks/evidence/2026-06-22-shell-ssh-runtime-kex-smoke"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-shell-ssh-runtime-kex-smoke.log"
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
    echo "qemu-shell-ssh-runtime-kex-smoke: cargo-test-filter=$filter"
    cargo -Zjson-target-spec test --quiet "$filter"
}

if {
    echo "qemu-shell-ssh-runtime-kex-smoke: start"
    echo "qemu-shell-ssh-runtime-kex-smoke: substitute=target-cargo-test-via-qemu-runner"
    echo "qemu-shell-ssh-runtime-kex-smoke: boundary=internal sshservicediag runtime KEX integration plus focused runtime crypto tests"
    echo "qemu-shell-ssh-runtime-kex-smoke: success=real-curve25519-sha256,accepted-host-key-signing-handle,operator-seeded-csprng,sshservicediag-crypto-backend-ready,sshservicediag-encrypted-packet-state-ready,ssh-ready=false"
    echo "qemu-shell-ssh-runtime-kex-smoke: fail-closed=sshservicediag-kex-csprng-not-ready,sshservicediag-kex-host-key-not-ready,sshservicediag-kex-peer-public-key-invalid,sshservicediag-kex-transcript-invalid,sshservicediag-prerequisites-missing,sshservicediag-crypto-backend-unaccepted,ssh-ready=false"
    echo "qemu-shell-ssh-runtime-kex-smoke: redaction=fixed-labels-counters-booleans-test-names-only,no-private-keys,no-random-bytes,no-shared-secrets,no-exchange-hashes,no-derived-keys,no-signature-bytes,no-public-key-blobs,no-packet-plaintext,no-packet-ciphertext,no-tags,no-peer-raw-input,no-operator-identity,no-key-derived-identifiers,no-stable-session-identifiers"
    echo "qemu-shell-ssh-runtime-kex-smoke: non-goals=no-newkeys,no-encrypted-packet-io,no-authentication,no-session,no-shell-attachment,no-live-transport,no-hardware,no-boot-publication,no-reachability,no-compatibility,no-phase-transition"
    run_filter "runtime_kex_integration_marks_crypto_ready_without_ssh_readiness"
    run_filter "runtime_kex_integration_reports_fail_closed_labels_without_secret_evidence"
    run_filter "runtime_kex_success_uses_real_crypto_and_private_packet_state_handles"
    run_filter "runtime_kex_fail_closed_labels_cover_missing_prerequisites"
    run_filter "host_key_private_material_maps_to_fail_closed_states"
    run_filter "host_key_private_material_clears_only_host_key_prerequisite"
    run_filter "exposure_disabled_state_fails_closed_without_service_caps"
    run_filter "exposure_enabled_with_missing_metadata_stays_prerequisites_missing"
    echo "qemu-shell-ssh-runtime-kex-smoke: PASS classification=host-qemu-substitute-shell-ssh-runtime-kex-smoke-complete"
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
