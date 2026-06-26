#!/usr/bin/env bash
set -euo pipefail

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"

export TALOS_QEMU_LOCAL_COMMAND_LOOP_BOOT_SCENARIO="qemu_local_shell_absolute_path_vfs_command"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_LABEL="qemu-local-shell-absolute-path-vfs-command"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_CLASSIFICATION="qemu-local-shell-absolute-path-vfs-command-complete"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_ABSOLUTE_PATH_VFS_COMMAND_SMOKE=1
export TALOS_QEMU_LOCAL_COMMAND_LOOP_LOG_FILE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_LOG_FILE:-target/qemu-local-shell-bare-name-vfs-command-smoke.log}"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_QEMU_LOG_FILE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_QEMU_LOG_FILE:-target/qemu-local-shell-bare-name-vfs-command-smoke.qemu.log}"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_DIR="${TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_DIR:-tasks/evidence/2026-06-26-phase12-local-bare-name-vfs-command-core}"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_LOG="${TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_LOG:-$TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_DIR/qemu-local-shell-bare-name-vfs-command-smoke.log}"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_PORT="${TALOS_QEMU_LOCAL_COMMAND_LOOP_PORT:-54387}"

exec "$script_dir/qemu-local-serial-command-loop-smoke.sh" "$@"
