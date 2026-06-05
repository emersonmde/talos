#!/usr/bin/env bash
set -euo pipefail

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"

export TALOS_QEMU_LOCAL_COMMAND_LOOP_BOOT_SCENARIO="qemu_local_shell_multiple_background_jobs"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_LABEL="qemu-local-shell-multiple-background-jobs"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_CLASSIFICATION="qemu-local-shell-multiple-background-jobs-complete"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_MULTIPLE_BACKGROUND_JOBS_SMOKE=1
export TALOS_QEMU_LOCAL_COMMAND_LOOP_LOG_FILE="target/qemu-local-shell-multiple-background-jobs-smoke.log"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_QEMU_LOG_FILE="target/qemu-local-shell-multiple-background-jobs-smoke.qemu.log"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_DIR="tasks/evidence/2026-06-05-phase10-multiple-background-vfs-exec-records-core"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_LOG="$TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_DIR/qemu-local-shell-multiple-background-jobs-smoke.log"

"$script_dir/qemu-local-serial-command-loop-smoke.sh" "$@"
