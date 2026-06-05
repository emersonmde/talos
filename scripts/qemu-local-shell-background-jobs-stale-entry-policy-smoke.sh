#!/usr/bin/env bash
set -euo pipefail

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"

export TALOS_QEMU_LOCAL_COMMAND_LOOP_BOOT_SCENARIO="qemu_local_shell_background_jobs_stale_entry_policy"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_LABEL="qemu-local-shell-background-jobs-stale-entry-policy"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_CLASSIFICATION="qemu-local-shell-background-jobs-stale-entry-policy-complete"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_BACKGROUND_JOBS_STALE_ENTRY_POLICY_SMOKE=1
export TALOS_QEMU_LOCAL_COMMAND_LOOP_LOG_FILE="target/qemu-local-shell-background-jobs-stale-entry-policy-smoke.log"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_QEMU_LOG_FILE="target/qemu-local-shell-background-jobs-stale-entry-policy-smoke.qemu.log"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_DIR="tasks/evidence/2026-06-05-phase10-background-jobs-stale-entry-policy-core"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_LOG="$TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_DIR/qemu-local-shell-background-jobs-stale-entry-policy-smoke.log"

"$script_dir/qemu-local-serial-command-loop-smoke.sh" "$@"
