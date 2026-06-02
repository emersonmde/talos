#!/usr/bin/env bash
set -euo pipefail

export TALOS_QEMU_LOCAL_COMMAND_LOOP_BOOT_SCENARIO="qemu_local_ls_cwd"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_LABEL="qemu-local-ls-cwd"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_CLASSIFICATION="qemu-local-ls-cwd-complete"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_LS_CWD_SMOKE="1"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_LOG_FILE="target/qemu-local-ls-cwd-smoke.log"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_QEMU_LOG_FILE="target/qemu-local-ls-cwd-smoke.qemu.log"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_DIR="tasks/evidence/2026-06-02-qemu-local-ls-cwd-core"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_LOG="$TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_DIR/qemu-local-ls-cwd-smoke.log"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_PORT="${TALOS_QEMU_LOCAL_COMMAND_LOOP_PORT:-54337}"

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
exec "$script_dir/qemu-local-serial-command-loop-smoke.sh" "$@"
