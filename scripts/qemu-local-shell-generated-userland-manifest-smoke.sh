#!/usr/bin/env bash
set -euo pipefail

export TALOS_QEMU_LOCAL_COMMAND_LOOP_BOOT_SCENARIO="qemu_local_shell_generated_userland_manifest"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_LABEL="qemu-local-shell-generated-userland-manifest"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_CLASSIFICATION="qemu-local-shell-generated-userland-manifest-complete"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_GENERATED_USERLAND_MANIFEST_SMOKE="1"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_LOG_FILE="target/qemu-local-shell-generated-userland-manifest-smoke.log"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_QEMU_LOG_FILE="target/qemu-local-shell-generated-userland-manifest-smoke.qemu.log"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_DIR="tasks/evidence/2026-06-05-phase10-generated-userland-executable-core"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_LOG="$TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_DIR/qemu-local-shell-generated-userland-manifest-smoke.log"
export TALOS_QEMU_LOCAL_COMMAND_LOOP_PORT="${TALOS_QEMU_LOCAL_COMMAND_LOOP_PORT:-54386}"

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
exec "$script_dir/qemu-local-serial-command-loop-smoke.sh" "$@"
