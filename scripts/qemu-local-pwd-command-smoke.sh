#!/usr/bin/env bash
set -euo pipefail

export TALOS_QEMU_LOCAL_COMMAND_LOOP_BOOT_SCENARIO="qemu_local_pwd_command"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_LABEL="qemu-local-pwd-command"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_CLASSIFICATION="qemu-local-pwd-command-complete"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_PWD_COMMAND_SMOKE="1"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_LOG_FILE="target/qemu-local-pwd-command-smoke.log"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_QEMU_LOG_FILE="target/qemu-local-pwd-command-smoke.qemu.log"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_DIR="tasks/evidence/2026-05-31-qemu-local-pwd-command-core"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_LOG="$TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_DIR/qemu-local-pwd-command-smoke.log"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_PORT="${TALOS_QEMU_LOCAL_COMMAND_LOOP_PORT:-54328}"

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
exec "$script_dir/qemu-local-serial-command-loop-smoke.sh" "$@"
