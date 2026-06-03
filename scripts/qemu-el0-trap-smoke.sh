#!/bin/sh
set -eu

LOG_FILE="target/qemu-el0-trap-smoke.log"

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
. "$script_dir/qemu-nographic-smoke-lib.sh"

talos_qemu_prepare_image "qemu_el0_trap_smoke" "debug" "" "$@"
talos_qemu_run_nographic "virt,gic-version=2,virtualization=on" "" "$LOG_FILE"

grep -q "boot-info: .* el=2 " "$LOG_FILE"
grep -q "qemu-el0-trap-smoke: start" "$LOG_FILE"
grep -q "qemu-el0-trap-smoke: validated elr=0x0000000000100000 sp=0x0000000000200000 spsr=0x00000000000003c0 guard-blocked=true" "$LOG_FILE"
grep -q "qemu-el0-trap-smoke: trap vector=lower-aarch64-sync esr=0x0000000054007a10 far=0x" "$LOG_FILE"
grep -q " marker=0x7a10" "$LOG_FILE"
grep -q "qemu-el0-trap-smoke: final participants=1 expected=1 errors=0 classification=qemu-el0-trap-smoke-complete" "$LOG_FILE"
grep -q "qemu-el0-trap-smoke: PASS" "$LOG_FILE"

cat "$LOG_FILE"
