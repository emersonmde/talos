#!/bin/sh
set -eu

EVIDENCE_DIR="tasks/evidence/2026-06-21-shell-sockdiag-driver-packet-adapter-smoke"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-shell-sockdiag-driver-packet-adapter-smoke.log"
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
    echo "qemu-shell-sockdiag-driver-packet-adapter-smoke: cargo-test-filter=$filter"
    cargo -Zjson-target-spec test --quiet "$filter"
}

if {
    echo "qemu-shell-sockdiag-driver-packet-adapter-smoke: start"
    echo "qemu-shell-sockdiag-driver-packet-adapter-smoke: substitute=target-cargo-test-via-qemu-runner"
    echo "qemu-shell-sockdiag-driver-packet-adapter-smoke: boundary=shell-visible exec /bin/sockdiag over VFS/userspace, documented private userspace_socket_abi helpers, descriptor-backed socket dispatch, private smoltcp TCP bridge records, and deterministic DriverPacketAdapter substrate observations"
    echo "qemu-shell-sockdiag-driver-packet-adapter-smoke: shell-script=exec /bin/sockdiag -> waitpid -> laststatus -> exec /bin/sockdiag extra -> exec /bin/missingsock"
    echo "qemu-shell-sockdiag-driver-packet-adapter-smoke: lifecycle=vfs-executable-lookup -> exec-open-read -> startup-abi -> userspace_socket_abi socket/bind/listen/connect/accept/send/recv/poll/poll_wait/close -> smoltcp-established-handshake -> accepted-descriptor-attachment -> driver-rx-consumed -> smoltcp-tx-observed -> driver-tx-pop -> tx-backpressure-preserves-rx -> waitpid -> laststatus"
    echo "qemu-shell-sockdiag-driver-packet-adapter-smoke: deterministic-controls=malformed-arguments,missing-vfs-identity,userspace-abi-wrapper-dispatch,unchanged-local-socket-diagnostics,unchanged-smoltcp-bridge,unchanged-pingdiag,adapter-rx-tx-copying,adapter-backpressure,adapter-capacity-errors,bounded-syscall-vocabulary,no-public-abi,no-live-packet-io"
    run_filter "local_command_loop_execs_shell_visible_sockdiag_through_userspace_socket_abi"
    run_filter "driver_packet_adapter_moves_driver_rx_and_smoltcp_tx_with_copied_frames"
    run_filter "driver_packet_adapter_preserves_rx_when_tx_backpressure_blocks_smoltcp_receive"
    run_filter "driver_packet_adapter_maps_capacity_and_device_errors_deterministically"
    run_filter "userspace_socket_abi_wrappers_reach_smoltcp_tcp_socket_bridge"
    run_filter "local_command_loop_execs_shell_visible_pingdiag_through_vfs_diagnostic_layers"
    echo "qemu-shell-sockdiag-driver-packet-adapter-smoke: PASS classification=host-substitute-shell-sockdiag-driver-packet-adapter-smoke-complete"
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
