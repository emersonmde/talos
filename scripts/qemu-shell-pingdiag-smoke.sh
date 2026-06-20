#!/bin/sh
set -eu

EVIDENCE_DIR="tasks/evidence/2026-06-20-shell-pingdiag-smoke"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-shell-pingdiag-smoke.log"
RAW_LOG="$EVIDENCE_LOG.raw"
QEMU_TOOL_DIR="/opt/strider/openclaw/current/workspace/tools/qemu-9.2.0-install/bin"
CARGO_TOOL_DIR="$HOME/.cargo/bin"

mkdir -p "$EVIDENCE_DIR"

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
    echo "qemu-shell-pingdiag-smoke: cargo-test-filter=$filter"
    cargo -Zjson-target-spec test --quiet "$filter"
}

if {
    echo "qemu-shell-pingdiag-smoke: start"
    echo "qemu-shell-pingdiag-smoke: substitute=target-cargo-test-via-qemu-runner"
    echo "qemu-shell-pingdiag-smoke: boundary=shell-visible exec /bin/pingdiag over VFS/userspace diagnostic SVC, process-local descriptor ownership, UserMapping copy-in/copy-out, packet queues, and PacketQueueNetworkDevice::pump_driver"
    echo "qemu-shell-pingdiag-smoke: shell-script=exec /bin/pingdiag -> waitpid -> laststatus -> exec /bin/pingdiag extra -> exec /bin/missingdiag"
    echo "qemu-shell-pingdiag-smoke: lifecycle=vfs-executable-lookup -> exec-open-read -> startup-abi -> open -> start -> pump-outbound-arp-to-driver -> inject-arp-reply -> pump-outbound-icmp-to-driver -> inject-icmp-reply -> completed-status-result-copy-out -> close -> waitpid -> laststatus"
    echo "qemu-shell-pingdiag-smoke: deterministic-controls=malformed-arguments,missing-vfs-identity,wrong-owner,invalid-or-closed-descriptor,queue-capacity,caller-buffer-pressure,malformed-received-frames,timeout-retry,transmit-receive-device-errors,close-drop,stable-syscall-vocabulary"
    run_filter "local_command_loop_execs_shell_visible_pingdiag_through_vfs_diagnostic_layers"
    run_filter "vfs_ping_diagnostic_svc_fixture_records_packet_queue_lifecycle"
    run_filter "vfs_ping_diagnostic_svc_fixture_maps_packet_queue_controls"
    run_filter "vfs_ping_diagnostic_svc_fixture_maps_contract_error_controls"
    run_filter "process_local_ping_user_arguments_reject_malformed_owner_descriptor_capacity_and_buffers"
    echo "qemu-shell-pingdiag-smoke: PASS classification=host-substitute-shell-pingdiag-smoke-complete"
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
