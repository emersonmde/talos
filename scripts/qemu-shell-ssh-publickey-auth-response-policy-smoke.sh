#!/bin/sh
set -eu

EVIDENCE_DIR="tasks/evidence/2026-06-22-ssh-publickey-auth-response-policy-smoke"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-shell-ssh-publickey-auth-response-policy-smoke.log"
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
    echo "qemu-shell-ssh-publickey-auth-response-policy-smoke: cargo-test-filter=$filter"
    cargo -Zjson-target-spec test --quiet "$filter" || {
        status=$?
        echo "qemu-shell-ssh-publickey-auth-response-policy-smoke: FAIL cargo-test-filter=$filter status=$status"
        return "$status"
    }
}

if (
    echo "qemu-shell-ssh-publickey-auth-response-policy-smoke: start"
    echo "qemu-shell-ssh-publickey-auth-response-policy-smoke: substitute=target-cargo-test-via-qemu-runner"
    echo "qemu-shell-ssh-publickey-auth-response-policy-smoke: boundary=internal diagnostic command sshservicediag over accepted publickey USERAUTH_PK_OK/USERAUTH_FAILURE response-policy model"
    echo "qemu-shell-ssh-publickey-auth-response-policy-smoke: pk-ok-state=sshservicediag-publickey-auth-response-pk-ok-prerequisite-only,ssh-msg-userauth-pk-ok=60,authentication-success=false,session-count=0,channel-count=0,shell-attached=false,live-reachability=false,ssh-ready=false"
    echo "qemu-shell-ssh-publickey-auth-response-policy-smoke: signed-valid-state=sshservicediag-publickey-auth-response-failure-signature-valid-success-deferred,ssh-msg-userauth-failure=51,authentication-success=false,ssh-ready=false"
    echo "qemu-shell-ssh-publickey-auth-response-policy-smoke: fail-closed-states=signature-rejected,signature-malformed,authorized-key-missing,authorized-key-no-match,request-malformed,algorithm-unsupported,prerequisite-missing,policy-disabled,redaction-sensitive"
    echo "qemu-shell-ssh-publickey-auth-response-policy-smoke: redaction=fixed-labels-public-message-numbers-public-length-count-fields-false-zero-readiness-only,no-session-id-bytes,no-authorized-key-bytes,no-public-key-blobs,no-signatures,no-signed-data,no-fingerprints,no-digests,no-user-or-peer-strings,no-operator-identity,no-key-derived-identifiers,no-stable-identifiers,no-hardware-data,no-boot-artifacts"
    echo "qemu-shell-ssh-publickey-auth-response-policy-smoke: non-goals=no-userauth-success,no-authentication-success,no-account-database,no-sessions,no-channels,no-pty,no-shell-attachment,no-live-sockets,no-hardware,no-boot-publication,no-compatibility,no-phase-transition,no-ssh-ready"
    run_filter "publickey_auth_response" || exit $?
    echo "qemu-shell-ssh-publickey-auth-response-policy-smoke: PASS classification=host-qemu-substitute-shell-ssh-publickey-auth-response-policy-smoke-complete"
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
