#!/bin/sh
set -eu

EVIDENCE_DIR="tasks/evidence/2026-06-21-shell-sockdiag-cross-process-local-socket-smoke"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-shell-sockdiag-cross-process-local-socket-smoke.log"
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
    echo "qemu-shell-sockdiag-cross-process-local-socket-smoke: cargo-test-filter=$filter"
    cargo -Zjson-target-spec test --quiet "$filter"
}

if {
    echo "qemu-shell-sockdiag-cross-process-local-socket-smoke: start"
    echo "qemu-shell-sockdiag-cross-process-local-socket-smoke: substitute=target-cargo-test-via-qemu-runner"
    echo "qemu-shell-sockdiag-cross-process-local-socket-smoke: boundary=shell-visible exec /bin/sockdiag over VFS/userspace, distinct ProcessOwnerId descriptor tables, private cross-process local rendezvous, deterministic controls, waitpid, and laststatus"
    echo "qemu-shell-sockdiag-cross-process-local-socket-smoke: shell-script=exec /bin/sockdiag -> waitpid -> laststatus -> exec /bin/sockdiag extra -> exec /bin/missingsock"
    echo "qemu-shell-sockdiag-cross-process-local-socket-smoke: lifecycle=vfs-executable-lookup -> exec-open-read -> startup-abi -> server/listener owner -> client owner -> cross-process connect -> server-owned accept -> bidirectional payload -> listener/payload bounded waits -> peer close hangup -> cleanup/release -> waitpid -> laststatus"
    echo "qemu-shell-sockdiag-cross-process-local-socket-smoke: deterministic-controls=distinct-descriptor-ownership,same-fd-number-different-owners,payload-transfer,listener-wait-wake,payload-wait-wake,peer-hangup,cleanup-release,malformed-arguments,missing-vfs-identity,unchanged-open-close,unchanged-bind-listen,unchanged-connect-accept,unchanged-send-recv,unchanged-readiness-poll,unchanged-blocking-poll-wait,pingdiag-regression,bounded-syscall-vocabulary"
    run_filter "local_command_loop_execs_shell_visible_sockdiag_through_vfs_socket_syscalls" &&
    run_filter "talos_cross_process_local_socket_rendezvous_preserves_descriptor_ownership" &&
    run_filter "talos_cross_process_poll_wait_wakes_on_accept_payload_and_peer_close" &&
    run_filter "talos_cross_process_close_cleanup_releases_pending_and_connected_capacity" &&
    run_filter "local_command_loop_execs_shell_visible_pingdiag_through_vfs_diagnostic_layers" &&
    echo "qemu-shell-sockdiag-cross-process-local-socket-smoke: PASS classification=host-substitute-shell-sockdiag-cross-process-local-socket-smoke-complete"
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
