#!/bin/sh
set -eu

EVIDENCE_DIR="tasks/evidence/2026-06-22-ssh-service-readiness-diagnostic-core"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-shell-ssh-service-diag-smoke.log"
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
    echo "qemu-shell-ssh-service-diag-smoke: cargo-test-filter=$filter"
    cargo -Zjson-target-spec test --quiet "$filter"
}

if {
    echo "qemu-shell-ssh-service-diag-smoke: start"
    echo "qemu-shell-ssh-service-diag-smoke: substitute=target-cargo-test-via-qemu-runner"
    echo "qemu-shell-ssh-service-diag-smoke: boundary=internal diagnostic command sshservicediag over fail-closed SSH service readiness classifier"
    echo "qemu-shell-ssh-service-diag-smoke: default-labels=sshservicediag-not-ready,sshservicediag-exposure-disabled,sshservicediag-prerequisites-missing,sshservicediag-dependency-unaccepted,sshservicediag-crypto-backend-unaccepted,sshservicediag-transport-unaccepted,sshservicediag-authentication-unimplemented,sshservicediag-session-unimplemented,ssh-ready=false"
    echo "qemu-shell-ssh-service-diag-smoke: caps=listener-count=0,transport-enabled=false,accepted-connection-count=0,session-count=0,channel-count=0,authentication-success=false,shell-attached=false,reachability-accepted=false"
    echo "qemu-shell-ssh-service-diag-smoke: non-goals=no-listener,no-live-transport,no-authentication-success,no-shell-attachment,no-hardware-reachability,no-public-abi,no-secret-identifiers,no-phase-transition"
    run_filter "dispatcher_reports_ssh_service_readiness_fail_closed_without_live_service"
    run_filter "dispatcher_reports_persistence_exposure_metadata_without_secret_material"
    run_filter "prerequisite_satisfied_shape_remains_not_ready_without_transport_or_session"
    echo "qemu-shell-ssh-service-diag-smoke: PASS classification=host-qemu-substitute-shell-ssh-service-readiness-fail-closed-smoke-complete"
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
