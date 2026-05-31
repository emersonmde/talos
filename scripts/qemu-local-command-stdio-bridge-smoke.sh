#!/usr/bin/env bash
set -euo pipefail

export TALOS_QEMU_LOCAL_COMMAND_LOOP_LOG_FILE="target/qemu-local-command-stdio-bridge-smoke.log"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_QEMU_LOG_FILE="target/qemu-local-command-stdio-bridge-smoke.qemu.log"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_DIR="tasks/evidence/2026-05-31-qemu-local-command-stdio-bridge-core"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_LOG="$TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_DIR/qemu-local-command-stdio-bridge-smoke.log"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_PORT="${TALOS_QEMU_LOCAL_COMMAND_LOOP_PORT:-54325}"

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
exec "$script_dir/qemu-local-serial-command-loop-smoke.sh" "$@"
