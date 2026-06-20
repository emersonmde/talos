#!/bin/sh
set -eu

EVIDENCE_DIR="tasks/evidence/2026-06-20-shell-sockdiag-connect-accept-smoke"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-shell-sockdiag-connect-accept-smoke.log"
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
    echo "qemu-shell-sockdiag-connect-accept-smoke: cargo-test-filter=$filter"
    cargo -Zjson-target-spec test --quiet "$filter"
}

if {
    echo "qemu-shell-sockdiag-connect-accept-smoke: start"
    echo "qemu-shell-sockdiag-connect-accept-smoke: substitute=target-cargo-test-via-qemu-runner"
    echo "qemu-shell-sockdiag-connect-accept-smoke: boundary=shell-visible exec /bin/sockdiag over VFS/userspace, TALOS_SOCKET open, TALOS_BIND endpoint, TALOS_LISTEN state, TALOS_CONNECT client, TALOS_ACCEPT server descriptor, TALOS_CLOSE close/drop, waitpid, and laststatus"
    echo "qemu-shell-sockdiag-connect-accept-smoke: shell-script=exec /bin/sockdiag -> waitpid -> laststatus -> exec /bin/sockdiag extra -> exec /bin/missingsock"
    echo "qemu-shell-sockdiag-connect-accept-smoke: lifecycle=vfs-executable-lookup -> exec-open-read -> startup-abi -> TALOS_SOCKET(AF_INET,SOCK_STREAM,0) listener -> TALOS_BIND(127.0.0.1:8080) -> TALOS_LISTEN(backlog=1) -> TALOS_SOCKET client -> TALOS_CONNECT(127.0.0.1:8080) -> TALOS_ACCEPT -> listener/client/accepted state -> TALOS_CLOSE accepted/client/listener -> backing-drop -> waitpid -> laststatus"
    echo "qemu-shell-sockdiag-connect-accept-smoke: deterministic-controls=malformed-arguments,missing-vfs-identity,unsupported-domain,unsupported-type,unsupported-protocol,listen-before-bind,invalid-bind-endpoint,invalid-backlog,repeated-bind,repeated-listen-backlog-update,accept-before-connect,missing-listener,queue-backpressure,non-socket-descriptor,invalid-or-closed-descriptor,descriptor-capacity,backing-capacity,scalar-dispatch-enotsup,bounded-syscall-vocabulary,pingdiag-regression"
    run_filter "local_command_loop_execs_shell_visible_sockdiag_through_vfs_socket_syscalls"
    run_filter "local_command_loop_execs_shell_visible_pingdiag_through_vfs_diagnostic_layers"
    run_filter "talos_connect_accept_records_local_handshake_state"
    run_filter "talos_connect_accept_errors_are_all_or_nothing"
    run_filter "talos_accept_rejects_capacity_failures_without_dequeueing_peer"
    run_filter "talos_bind_listen_records_socket_state_and_close_drops_backing"
    run_filter "talos_bind_listen_errors_are_deterministic_and_do_not_mutate_state"
    run_filter "talos_socket_opens_af_inet_stream_descriptor_and_close_drops_backing"
    run_filter "talos_socket_errors_are_deterministic_and_do_not_allocate_on_failure"
    run_filter "talos_socket_close_rejects_wrong_owner_socket_backing"
    run_filter "socket_number_requires_socket_table_context_in_scalar_dispatch"
    echo "qemu-shell-sockdiag-connect-accept-smoke: PASS classification=host-substitute-shell-sockdiag-connect-accept-smoke-complete"
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
