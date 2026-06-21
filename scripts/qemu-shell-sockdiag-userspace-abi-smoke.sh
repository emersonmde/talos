#!/bin/sh
set -eu

EVIDENCE_DIR="tasks/evidence/2026-06-21-shell-sockdiag-userspace-abi-smoke"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-shell-sockdiag-userspace-abi-smoke.log"
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
    echo "qemu-shell-sockdiag-userspace-abi-smoke: cargo-test-filter=$filter"
    cargo -Zjson-target-spec test --quiet "$filter"
}

if {
    echo "qemu-shell-sockdiag-userspace-abi-smoke: start"
    echo "qemu-shell-sockdiag-userspace-abi-smoke: substitute=target-cargo-test-via-qemu-runner"
    echo "qemu-shell-sockdiag-userspace-abi-smoke: boundary=shell-visible exec /bin/sockdiag over VFS/userspace, documented private userspace_socket_abi helpers, existing descriptor-backed socket dispatch, and host-only private smoltcp TCP bridge records"
    echo "qemu-shell-sockdiag-userspace-abi-smoke: abi-surface=userspace-socket-abi-v1 selectors=socket,bind,listen,connect,accept,send,recv,poll,poll_wait,close layout=poll-entry-fd-events-revents"
    echo "qemu-shell-sockdiag-userspace-abi-smoke: shell-script=exec /bin/sockdiag -> waitpid -> laststatus -> exec /bin/sockdiag extra -> exec /bin/missingsock"
    echo "qemu-shell-sockdiag-userspace-abi-smoke: lifecycle=vfs-executable-lookup -> exec-open-read -> startup-abi -> userspace_socket_abi socket/bind/listen/connect/accept/send/recv/poll/poll_wait/close -> smoltcp-established-handshake -> accepted-descriptor-attachment -> bounded-payload-transfer -> recv -> close/drop -> waitpid -> laststatus"
    echo "qemu-shell-sockdiag-userspace-abi-smoke: deterministic-controls=malformed-arguments,missing-vfs-identity,userspace-abi-constant-contract,userspace-abi-wrapper-dispatch,unchanged-local-socket-diagnostics,unchanged-pingdiag,bounded-syscall-vocabulary,no-public-abi,no-live-packet-io"
    run_filter "local_command_loop_execs_shell_visible_sockdiag_through_userspace_socket_abi"
    run_filter "userspace_socket_abi_constants_match_private_kernel_contract"
    run_filter "userspace_socket_abi_wrappers_reach_smoltcp_tcp_socket_bridge"
    run_filter "local_command_loop_execs_shell_visible_pingdiag_through_vfs_diagnostic_layers"
    echo "qemu-shell-sockdiag-userspace-abi-smoke: PASS classification=host-substitute-shell-sockdiag-userspace-abi-smoke-complete"
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
