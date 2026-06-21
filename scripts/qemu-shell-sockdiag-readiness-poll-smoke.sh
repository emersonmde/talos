#!/bin/sh
set -eu

EVIDENCE_DIR="tasks/evidence/2026-06-21-shell-sockdiag-readiness-poll-smoke"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-shell-sockdiag-readiness-poll-smoke.log"
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
    echo "qemu-shell-sockdiag-readiness-poll-smoke: cargo-test-filter=$filter"
    cargo -Zjson-target-spec test --quiet "$filter"
}

if {
    echo "qemu-shell-sockdiag-readiness-poll-smoke: start"
    echo "qemu-shell-sockdiag-readiness-poll-smoke: substitute=target-cargo-test-via-qemu-runner"
    echo "qemu-shell-sockdiag-readiness-poll-smoke: boundary=shell-visible exec /bin/sockdiag over VFS/userspace, TALOS_POLL readiness over descriptor-backed local sockets, deterministic controls, waitpid, and laststatus"
    echo "qemu-shell-sockdiag-readiness-poll-smoke: shell-script=exec /bin/sockdiag -> waitpid -> laststatus -> exec /bin/sockdiag extra -> exec /bin/missingsock"
    echo "qemu-shell-sockdiag-readiness-poll-smoke: lifecycle=vfs-executable-lookup -> exec-open-read -> startup-abi -> socket/listener/client/accepted descriptors -> TALOS_POLL empty listener -> pending listener READ -> empty recv zero -> payload READ -> WRITE capacity -> full FIFO zero-write -> peer close READ|HANGUP -> invalid/non-socket ERROR -> close/drop -> waitpid -> laststatus"
    echo "qemu-shell-sockdiag-readiness-poll-smoke: deterministic-controls=unsupported-poll-events,invalid-descriptor,non-socket-descriptor,malformed-arguments,missing-vfs-identity,unchanged-open-close,unchanged-bind-listen,unchanged-connect-accept,unchanged-send-recv,pingdiag-regression,bounded-syscall-vocabulary"
    run_filter "local_command_loop_execs_shell_visible_sockdiag_through_vfs_socket_syscalls"
    run_filter "talos_poll_reports_listener_local_payload_and_peer_hangup_readiness"
    run_filter "talos_poll_reports_write_backpressure_and_deterministic_entry_errors"
    run_filter "talos_poll_rejects_malformed_calls_and_scalar_dispatch_fails_closed"
    run_filter "local_command_loop_execs_shell_visible_pingdiag_through_vfs_diagnostic_layers"
    echo "qemu-shell-sockdiag-readiness-poll-smoke: PASS classification=host-substitute-shell-sockdiag-readiness-poll-smoke-complete"
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
