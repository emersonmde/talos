#!/usr/bin/env bash
set -euo pipefail

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"

export TALOS_QEMU_LOCAL_COMMAND_LOOP_BOOT_SCENARIO="qemu_local_shell_pipeline_producer_file_redirection_away"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_LABEL="qemu-local-shell-pipeline-producer-file-redirection-away"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_CLASSIFICATION="qemu-local-shell-pipeline-producer-file-redirection-away-complete"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_PIPELINE_PRODUCER_FILE_REDIRECTION_AWAY_SMOKE=1
export TALOS_QEMU_LOCAL_COMMAND_LOOP_LOG_FILE="target/qemu-local-shell-pipeline-producer-file-redirection-away-smoke.log"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_QEMU_LOG_FILE="target/qemu-local-shell-pipeline-producer-file-redirection-away-smoke.qemu.log"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_DIR="tasks/evidence/2026-06-05-phase10-pipeline-producer-file-redirection-away-core"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_LOG="$TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_DIR/qemu-local-shell-pipeline-producer-file-redirection-away-smoke.log"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_PORT="${TALOS_QEMU_LOCAL_COMMAND_LOOP_PORT:-54406}"

exec "$script_dir/qemu-local-serial-command-loop-smoke.sh" "$@"
