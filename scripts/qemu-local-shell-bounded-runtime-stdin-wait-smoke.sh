#!/usr/bin/env bash
set -euo pipefail

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"

export TALOS_QEMU_LOCAL_COMMAND_LOOP_BOOT_SCENARIO="qemu_local_shell_stdin"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_LABEL="qemu-local-shell-userspace-stdin"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_CLASSIFICATION="qemu-local-shell-userspace-stdin-complete"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_STDIN_BOUNDED_WAIT_SMOKE=1
export TALOS_QEMU_LOCAL_COMMAND_LOOP_LOG_FILE="target/qemu-local-shell-bounded-runtime-stdin-wait-smoke.log"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_QEMU_LOG_FILE="target/qemu-local-shell-bounded-runtime-stdin-wait-smoke.qemu.log"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_DIR="tasks/evidence/2026-06-04-phase10-bounded-runtime-stdin-wait-core"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_LOG="$TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_DIR/qemu-local-shell-bounded-runtime-stdin-wait-smoke.log"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_PORT="${TALOS_QEMU_LOCAL_COMMAND_LOOP_PORT:-54365}"

exec "$script_dir/qemu-local-serial-command-loop-smoke.sh" "$@"
