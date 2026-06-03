#!/bin/sh
set -eu

LOG_FILE="target/qemu-initial-userspace-process-launch-smoke.log"
EVIDENCE_DIR="tasks/evidence/2026-06-03-qemu-initial-userspace-process-launch"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-initial-userspace-process-launch-smoke.log"

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
. "$script_dir/qemu-nographic-smoke-lib.sh"

talos_qemu_prepare_image "qemu_initial_userspace_process_launch_smoke" "debug" "" "$@"
talos_qemu_run_nographic "virt,gic-version=2,virtualization=on" "" "$LOG_FILE"

grep -q "boot-info: .* el=2 " "$LOG_FILE"
grep -Fq "qemu-initial-userspace-process-launch-smoke: start" "$LOG_FILE"
grep -Eq "qemu-initial-userspace-process-launch-smoke: fixture name=phase8-program-loader-elf64-aarch64-v1 path=/bin/init source-digest=0x[0-9a-f]+ launch-boundary=phase8-initial-process-launch-plan-v1 stack-boundary=phase8-initial-user-stack-plan-v1" "$LOG_FILE"
grep -Eq "qemu-initial-userspace-process-launch-smoke: launch-input entry=0x[0-9a-f]+ initial-sp=0x[0-9a-f]+ stack-state=model-only-initial-user-stack-ready stack-pages=4 user-stack-ready=true ok=true" "$LOG_FILE"
grep -Eq "qemu-initial-userspace-process-launch-smoke: translation-ready text=\[0x[0-9a-f]+,0x[0-9a-f]+\) data=\[0x[0-9a-f]+,0x[0-9a-f]+\) stack=\[0x[0-9a-f]+,0x[0-9a-f]+\) guard=\[0x[0-9a-f]+,0x[0-9a-f]+\)" "$LOG_FILE"
grep -Eq "qemu-initial-userspace-process-launch-smoke: userspace-signal vector=lower-aarch64-sync esr=0x0000000054007a10 far=0x[0-9a-f]+ elr=0x0000000000010108 sp=0x0000800000000000 spsr=0x[0-9a-f]+ marker=0x7a10" "$LOG_FILE"
grep -Fq "qemu-initial-userspace-process-launch-smoke: frame available=true x0=0x0000000000000000" "$LOG_FILE"
grep -Fq "status=0x0000000000000000 complete=true" "$LOG_FILE"
grep -Fq "qemu-initial-userspace-process-launch-smoke: final participants=1 expected=1 errors=0 classification=qemu-initial-userspace-process-launch-smoke-complete" "$LOG_FILE"
grep -Fq "qemu-initial-userspace-process-launch-smoke: PASS" "$LOG_FILE"

mkdir -p "$EVIDENCE_DIR"
tr -d '\r' <"$LOG_FILE" >"$EVIDENCE_LOG"
cat "$EVIDENCE_LOG"
