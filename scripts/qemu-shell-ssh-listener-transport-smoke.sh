#!/bin/sh
set -eu

EVIDENCE_DIR="tasks/evidence/2026-06-22-ssh-listener-transport-smoke"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-shell-ssh-listener-transport-smoke.log"
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
    echo "qemu-shell-ssh-listener-transport-smoke: cargo-test-filter=$filter"
    cargo -Zjson-target-spec test --quiet "$filter" || {
        status=$?
        echo "qemu-shell-ssh-listener-transport-smoke: FAIL cargo-test-filter=$filter status=$status"
        return "$status"
    }
}

if (
    echo "qemu-shell-ssh-listener-transport-smoke: start"
    echo "qemu-shell-ssh-listener-transport-smoke: substitute=target-cargo-test-via-qemu-runner"
    echo "qemu-shell-ssh-listener-transport-smoke: boundary=internal diagnostic command sshservicediag over accepted local listener/transport model"
    echo "qemu-shell-ssh-listener-transport-smoke: disabled-state=sshservicediag-exposure-disabled,sshservicediag-prerequisites-missing,sshservicediag-transport-unaccepted,listener-count=0,transport-enabled=false,accepted-connection-count=0,ssh-ready=false"
    echo "qemu-shell-ssh-listener-transport-smoke: prerequisite-missing-state=sshservicediag-prerequisites-missing,sshservicediag-transport-unaccepted,listener-count=0,transport-enabled=false,accepted-connection-count=0,ssh-ready=false"
    echo "qemu-shell-ssh-listener-transport-smoke: local-transport-modeled-state=sshservicediag-local-listener-modeled,sshservicediag-local-transport-modeled,sshservicediag-identification-banner-modeled,sshservicediag-remote-identification-valid,sshservicediag-transport-closed-before-kex,listener-count=1,transport-enabled=true,accepted-connection-count=1,authentication-success=false,shell-attached=false,reachability-accepted=false,ssh-ready=false"
    echo "qemu-shell-ssh-listener-transport-smoke: remote-classifications=valid,invalid,over-limit"
    echo "qemu-shell-ssh-listener-transport-smoke: redaction=fixed-labels-counters-booleans-only,no-peer-text,no-peer-addresses,no-key-bytes,no-fingerprints,no-random-bytes,no-operator-identity,no-key-derived-identifiers,no-stable-transport-session-identifiers"
    echo "qemu-shell-ssh-listener-transport-smoke: non-goals=no-runtime-ssh-crypto,no-kex,no-authentication-success,no-session-channel,no-pty,no-shell-attachment,no-hardware,no-boot-publication,no-reachability,no-public-compatibility,no-phase-transition"
    run_filter "dispatcher_reports_ssh_service_readiness_fail_closed_without_live_service" || exit $?
    run_filter "dispatcher_reports_persistence_exposure_metadata_without_secret_material" || exit $?
    run_filter "exposure_enabled_with_missing_metadata_stays_prerequisites_missing" || exit $?
    run_filter "prerequisite_satisfied_shape_models_local_transport_but_remains_not_ready" || exit $?
    run_filter "local_transport_model_classifies_invalid_remote_identification_fail_closed" || exit $?
    run_filter "local_transport_model_classifies_unterminated_remote_identification_as_over_limit" || exit $?
    echo "qemu-shell-ssh-listener-transport-smoke: PASS classification=host-qemu-substitute-shell-ssh-listener-transport-smoke-complete"
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
