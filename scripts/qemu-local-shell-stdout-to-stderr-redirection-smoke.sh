#!/usr/bin/env bash
set -euo pipefail

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"

export TALOS_QEMU_LOCAL_COMMAND_LOOP_BOOT_SCENARIO="qemu_local_shell_stdout_to_stderr_redirection"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_LABEL="qemu-local-shell-stdout-to-stderr-redirection"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_CLASSIFICATION="qemu-local-shell-stdout-to-stderr-redirection-complete"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_STDOUT_TO_STDERR_REDIRECTION_SMOKE=1
export TALOS_QEMU_LOCAL_COMMAND_LOOP_LOG_FILE="target/qemu-local-shell-stdout-to-stderr-redirection-smoke.log"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_QEMU_LOG_FILE="target/qemu-local-shell-stdout-to-stderr-redirection-smoke.qemu.log"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_DIR="tasks/evidence/2026-06-04-phase10-stdout-to-stderr-fd-dup-redirection-core"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_LOG="$TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_DIR/qemu-local-shell-stdout-to-stderr-redirection-smoke.log"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_PORT="${TALOS_QEMU_LOCAL_COMMAND_LOOP_PORT:-54374}"

exec "$script_dir/qemu-local-serial-command-loop-smoke.sh" "$@"
