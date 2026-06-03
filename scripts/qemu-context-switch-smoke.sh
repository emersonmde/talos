#!/bin/sh
set -eu

LOG_FILE="target/qemu-context-switch-smoke.log"

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
. "$script_dir/qemu-nographic-smoke-lib.sh"

talos_qemu_prepare_image "qemu_context_switch" "debug" "" "$@"
talos_qemu_run_nographic "virt,gic-version=2,virtualization=on" "" "$LOG_FILE"

grep -q "boot-info: .* el=2 " "$LOG_FILE"
grep -q "qemu-context-switch-smoke: start current=0 runnable=1" "$LOG_FILE"
grep -Eq "qemu-context-switch-smoke: progress task1=[2-9][0-9]* task2=[2-9][0-9]* switches=[5-9][0-9]* current=[12] runnable=0" "$LOG_FILE"
grep -q "qemu-context-switch-smoke: PASS" "$LOG_FILE"

cat "$LOG_FILE"
