#!/bin/sh
set -eu

EVIDENCE_DIR="tasks/evidence/2026-06-21-shell-sockdiag-smoltcp-tcp-smoke"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-shell-sockdiag-smoltcp-tcp-smoke.log"
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
    echo "qemu-shell-sockdiag-smoltcp-tcp-smoke: cargo-test-filter=$filter"
    cargo -Zjson-target-spec test --quiet "$filter"
}

if {
    echo "qemu-shell-sockdiag-smoltcp-tcp-smoke: start"
    echo "qemu-shell-sockdiag-smoltcp-tcp-smoke: substitute=target-cargo-test-via-qemu-runner"
    echo "qemu-shell-sockdiag-smoltcp-tcp-smoke: boundary=shell-visible exec /bin/sockdiag over VFS/userspace, existing private socket syscalls, and host-only private smoltcp TCP bridge records"
    echo "qemu-shell-sockdiag-smoltcp-tcp-smoke: shell-script=exec /bin/sockdiag -> waitpid -> laststatus -> exec /bin/sockdiag extra -> exec /bin/missingsock"
    echo "qemu-shell-sockdiag-smoltcp-tcp-smoke: lifecycle=vfs-executable-lookup -> exec-open-read -> startup-abi -> socket/bind/listen/connect/accept -> smoltcp-established-handshake -> accepted-descriptor-attachment -> bounded-payload-transfer -> recv -> poll/poll-wait -> close/drop -> waitpid -> laststatus"
    echo "qemu-shell-sockdiag-smoltcp-tcp-smoke: deterministic-controls=malformed-arguments,missing-vfs-identity,unchanged-local-socket-diagnostics,unchanged-pingdiag,bridge-regression,bounded-syscall-vocabulary,no-public-abi,no-live-packet-io"
    run_filter "local_command_loop_execs_shell_visible_sockdiag_through_vfs_socket_syscalls"
    run_filter "talos_smoltcp_socket_bridge_transfers_payload_through_private_syscalls"
    run_filter "local_command_loop_execs_shell_visible_pingdiag_through_vfs_diagnostic_layers"
    echo "qemu-shell-sockdiag-smoltcp-tcp-smoke: PASS classification=host-substitute-shell-sockdiag-smoltcp-tcp-smoke-complete"
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
