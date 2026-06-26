#!/usr/bin/env bash
set -euo pipefail

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"

export TALOS_QEMU_LOCAL_COMMAND_LOOP_BOOT_SCENARIO="qemu_local_shell_bare_name_command_argv"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_LABEL="qemu-local-shell-bare-name-command-argv"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_CLASSIFICATION="qemu-local-shell-bare-name-command-argv-complete"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_BARE_NAME_COMMAND_ARGV_SMOKE=1
export TALOS_QEMU_LOCAL_COMMAND_LOOP_LOG_FILE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_LOG_FILE:-target/qemu-local-shell-bare-name-command-argv-smoke.log}"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_QEMU_LOG_FILE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_QEMU_LOG_FILE:-target/qemu-local-shell-bare-name-command-argv-smoke.qemu.log}"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_DIR="${TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_DIR:-tasks/evidence/2026-06-26-phase12-local-bare-name-command-argv-core}"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_LOG="${TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_LOG:-$TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_DIR/qemu-local-shell-bare-name-command-argv-smoke.log}"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_PORT="${TALOS_QEMU_LOCAL_COMMAND_LOOP_PORT:-54388}"

exec "$script_dir/qemu-local-serial-command-loop-smoke.sh" "$@"
