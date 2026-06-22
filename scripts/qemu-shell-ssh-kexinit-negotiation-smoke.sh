#!/bin/sh
set -eu

EVIDENCE_DIR="tasks/evidence/2026-06-22-ssh-kexinit-negotiation-smoke"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-shell-ssh-kexinit-negotiation-smoke.log"
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
    echo "qemu-shell-ssh-kexinit-negotiation-smoke: cargo-test-filter=$filter"
    cargo -Zjson-target-spec test --quiet "$filter" || {
        status=$?
        echo "qemu-shell-ssh-kexinit-negotiation-smoke: FAIL cargo-test-filter=$filter status=$status"
        return "$status"
    }
}

if (
    echo "qemu-shell-ssh-kexinit-negotiation-smoke: start"
    echo "qemu-shell-ssh-kexinit-negotiation-smoke: substitute=target-cargo-test-via-qemu-runner"
    echo "qemu-shell-ssh-kexinit-negotiation-smoke: boundary=internal diagnostic command sshservicediag over accepted local KEXINIT/algorithm-negotiation model"
    echo "qemu-shell-ssh-kexinit-negotiation-smoke: negotiated-state=sshservicediag-kexinit-modeled,sshservicediag-kexinit-cookie-generated-redacted,sshservicediag-kexinit-client-packet-valid,sshservicediag-kexinit-algorithm-negotiated,sshservicediag-kexinit-selected-kex-curve25519-sha256,sshservicediag-kexinit-selected-hostkey-ssh-ed25519,sshservicediag-kexinit-selected-cipher-chacha20-poly1305-openssh,sshservicediag-kexinit-selected-mac-hmac-sha2-256,sshservicediag-kexinit-selected-compression-none,ssh-ready=false"
    echo "qemu-shell-ssh-kexinit-negotiation-smoke: negative-states=unsupported-algorithm,malformed-packet,packet-over-limit,list-over-limit,first-packet-follows-ignored"
    echo "qemu-shell-ssh-kexinit-negotiation-smoke: disabled-state=sshservicediag-exposure-disabled,sshservicediag-prerequisites-missing,sshservicediag-transport-unaccepted,listener-count=0,transport-enabled=false,accepted-connection-count=0,ssh-ready=false"
    echo "qemu-shell-ssh-kexinit-negotiation-smoke: prerequisite-missing-state=sshservicediag-prerequisites-missing,sshservicediag-transport-unaccepted,listener-count=0,transport-enabled=false,accepted-connection-count=0,ssh-ready=false"
    echo "qemu-shell-ssh-kexinit-negotiation-smoke: redaction=fixed-labels-counters-booleans-only,no-random-bytes,no-kex-cookies,no-client-packet-bytes,no-client-algorithm-list-text,no-key-material,no-peer-text,no-peer-addresses,no-digests,no-fingerprints,no-operator-identity,no-key-derived-identifiers,no-stable-transport-session-identifiers"
    echo "qemu-shell-ssh-kexinit-negotiation-smoke: non-goals=no-actual-kex,no-encryption-mac,no-newkeys,no-hostkey-signing,no-authentication-success,no-session-channel,no-pty,no-shell-attachment,no-hardware,no-boot-publication,no-reachability,no-public-compatibility,no-phase-transition"
    run_filter "prerequisite_satisfied_shape_models_local_transport_but_remains_not_ready" || exit $?
    run_filter "kexinit_negotiates_policy_with_redacted_cookie_and_ignored_followup" || exit $?
    run_filter "kexinit_rejects_unsupported_algorithm_without_retaining_client_text" || exit $?
    run_filter "kexinit_rejects_malformed_packet_and_size_limits" || exit $?
    run_filter "kexinit_rejects_list_over_limits" || exit $?
    run_filter "exposure_disabled_state_fails_closed_without_service_caps" || exit $?
    run_filter "exposure_enabled_with_missing_metadata_stays_prerequisites_missing" || exit $?
    run_filter "dispatcher_reports_ssh_service_readiness_fail_closed_without_live_service" || exit $?
    echo "qemu-shell-ssh-kexinit-negotiation-smoke: PASS classification=host-qemu-substitute-shell-ssh-kexinit-negotiation-smoke-complete"
) >"$RAW_LOG" 2>&1; then
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
