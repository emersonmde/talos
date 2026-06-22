#!/bin/sh
set -eu

EVIDENCE_DIR="tasks/evidence/2026-06-22-ssh-session-channel-open-smoke"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-shell-ssh-session-channel-open-smoke.log"
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
    echo "qemu-shell-ssh-session-channel-open-smoke: cargo-test-filter=$filter"
    cargo -Zjson-target-spec test --quiet "$filter" || {
        status=$?
        echo "qemu-shell-ssh-session-channel-open-smoke: FAIL cargo-test-filter=$filter status=$status"
        return "$status"
    }
}

if (
    echo "qemu-shell-ssh-session-channel-open-smoke: start"
    echo "qemu-shell-ssh-session-channel-open-smoke: substitute=target-cargo-test-via-qemu-runner"
    echo "qemu-shell-ssh-session-channel-open-smoke: boundary=internal diagnostic command sshservicediag over accepted single-session SSH_MSG_CHANNEL_OPEN classifier"
    echo "qemu-shell-ssh-session-channel-open-smoke: success-state=sshservicediag-authentication-success-local-only,sshservicediag-session-channel-open-prerequisite-only,sshservicediag-session-channel-open-session-type,sshservicediag-session-open-local-only,sshservicediag-channel-open-local-only,sshservicediag-shell-unattached,ssh-msg-channel-open=90,ssh-msg-channel-open-confirmation=91,authentication-success=true,session-count=1,channel-count=1,shell-attached=false,live-reachability=false,ssh-ready=false"
    echo "qemu-shell-ssh-session-channel-open-smoke: fail-closed-states=authentication-missing,wrong-message,unsupported-channel-type,request-malformed,duplicate-existing-channel,policy-disabled,redaction-sensitive"
    echo "qemu-shell-ssh-session-channel-open-smoke: failure-message=ssh-msg-channel-open-failure=92,session-count=0,channel-count=0,shell-attached=false,live-reachability=false,ssh-ready=false"
    echo "qemu-shell-ssh-session-channel-open-smoke: redaction=fixed-labels-public-message-numbers-public-field-counts-public-channel-type-length-category-readiness-counters-only,no-request-payload-bytes,no-channel-identifiers,no-window-sizes,no-packet-sizes,no-user-or-operator-identity,no-key-material,no-session-id-bytes,no-signatures,no-hardware-data,no-boot-artifacts"
    echo "qemu-shell-ssh-session-channel-open-smoke: non-goals=no-pty,no-tty,no-process,no-shell-attachment,no-channel-data,no-eof-close-window-flow-control,no-shell-pty-exec-subsystem-requests,no-live-sockets,no-hardware,no-boot-publication,no-compatibility,no-phase-transition,no-ssh-ready"
    run_filter "session_channel_open" || exit $?
    echo "qemu-shell-ssh-session-channel-open-smoke: PASS classification=host-qemu-substitute-shell-ssh-session-channel-open-smoke-complete"
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
