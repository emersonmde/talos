#!/usr/bin/env bash
set -euo pipefail

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"

export TALOS_QEMU_LOCAL_COMMAND_LOOP_BOOT_SCENARIO="qemu_local_shell_background_vfs_exec_lifecycle"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_LABEL="qemu-local-shell-background-vfs-exec-lifecycle"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_CLASSIFICATION="qemu-local-shell-background-vfs-exec-lifecycle-complete"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_BACKGROUND_VFS_EXEC_LIFECYCLE_SMOKE=1
export TALOS_QEMU_LOCAL_COMMAND_LOOP_LOG_FILE="target/qemu-local-shell-background-vfs-exec-lifecycle-smoke.log"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_QEMU_LOG_FILE="target/qemu-local-shell-background-vfs-exec-lifecycle-smoke.qemu.log"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_DIR="tasks/evidence/2026-06-05-phase10-background-vfs-exec-lifecycle-core"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_LOG="$TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_DIR/qemu-local-shell-background-vfs-exec-lifecycle-smoke.log"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_PORT="${TALOS_QEMU_LOCAL_COMMAND_LOOP_PORT:-54407}"

exec "$script_dir/qemu-local-serial-command-loop-smoke.sh" "$@"
