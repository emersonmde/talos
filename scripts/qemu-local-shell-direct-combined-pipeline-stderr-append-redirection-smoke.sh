#!/usr/bin/env bash
set -euo pipefail

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"

export TALOS_QEMU_LOCAL_COMMAND_LOOP_BOOT_SCENARIO="qemu_local_shell_direct_combined_pipeline_stderr_append_redirection"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_LABEL="qemu-local-shell-direct-combined-pipeline-stderr-append-redirection"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_CLASSIFICATION="qemu-local-shell-direct-combined-pipeline-stderr-append-redirection-complete"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_DIRECT_COMBINED_PIPELINE_STDERR_APPEND_REDIRECTION_SMOKE=1
export TALOS_QEMU_LOCAL_COMMAND_LOOP_LOG_FILE="target/qemu-local-shell-direct-combined-pipeline-stderr-append-redirection-smoke.log"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_QEMU_LOG_FILE="target/qemu-local-shell-direct-combined-pipeline-stderr-append-redirection-smoke.qemu.log"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_DIR="tasks/evidence/2026-06-28-phase12-local-direct-combined-pipeline-stderr-append-redirection-core"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_LOG="$TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_DIR/qemu-local-shell-direct-combined-pipeline-stderr-append-redirection-smoke.log"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_PORT="${TALOS_QEMU_LOCAL_COMMAND_LOOP_PORT:-54441}"

exec "$script_dir/qemu-local-serial-command-loop-smoke.sh" "$@"
