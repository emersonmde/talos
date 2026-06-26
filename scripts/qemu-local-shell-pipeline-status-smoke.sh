#!/usr/bin/env bash
set -euo pipefail

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"

export TALOS_QEMU_LOCAL_COMMAND_LOOP_BOOT_SCENARIO="qemu_local_shell_pipeline_status"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_LABEL="qemu-local-shell-pipeline-status"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_CLASSIFICATION="qemu-local-shell-pipeline-status-complete"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_PIPELINE_STATUS_SMOKE=1
export TALOS_QEMU_LOCAL_COMMAND_LOOP_LOG_FILE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_LOG_FILE:-target/qemu-local-shell-pipeline-status-smoke.log}"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_QEMU_LOG_FILE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_QEMU_LOG_FILE:-target/qemu-local-shell-pipeline-status-smoke.qemu.log}"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_DIR="${TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_DIR:-tasks/evidence/2026-06-26-phase12-local-pipefail-status-core}"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_LOG="${TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_LOG:-$TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_DIR/qemu-local-shell-pipeline-status-smoke.log}"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_PORT="${TALOS_QEMU_LOCAL_COMMAND_LOOP_PORT:-54384}"

exec "$script_dir/qemu-local-serial-command-loop-smoke.sh" "$@"
