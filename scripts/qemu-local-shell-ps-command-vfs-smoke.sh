#!/usr/bin/env bash
set -euo pipefail

export TALOS_QEMU_LOCAL_COMMAND_LOOP_BOOT_SCENARIO="qemu_local_shell_ps_command_vfs"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_LABEL="qemu-local-shell-ps-command-vfs"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_CLASSIFICATION="qemu-local-shell-ps-command-vfs-complete"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_PS_COMMAND_VFS_SMOKE="1"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_LOG_FILE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_LOG_FILE:-target/qemu-local-shell-ps-command-vfs-smoke.log}"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_QEMU_LOG_FILE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_QEMU_LOG_FILE:-target/qemu-local-shell-ps-command-vfs-smoke.qemu.log}"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_DIR="${TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_DIR:-tasks/evidence/2026-06-26-phase12-local-ps-command-vfs-backed-core}"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_LOG="${TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_LOG:-$TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_DIR/qemu-local-shell-ps-command-vfs-smoke.log}"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_PORT="${TALOS_QEMU_LOCAL_COMMAND_LOOP_PORT:-54352}"

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
exec "$script_dir/qemu-local-serial-command-loop-smoke.sh" "$@"
