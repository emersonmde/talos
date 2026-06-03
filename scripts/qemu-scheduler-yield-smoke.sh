#!/bin/sh
set -eu

LOG_FILE="target/qemu-scheduler-yield-smoke.log"

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
. "$script_dir/qemu-nographic-smoke-lib.sh"

talos_qemu_prepare_image "qemu_scheduler_yield" "debug" "" "$@"
talos_qemu_run_nographic "virt,gic-version=2,virtualization=on" "" "$LOG_FILE"

grep -q "boot-info: .* el=2 " "$LOG_FILE"
grep -q "qemu-scheduler-yield-smoke: start current=1 runnable=2 yielded=0" "$LOG_FILE"
grep -Eq "qemu-scheduler-yield-smoke: progress task1=[3-9][0-9]* task2=[3-9][0-9]* yields=[5-9][0-9]* dispatch-switches=[5-9][0-9]* transitions=[6-9][0-9]* current=[12] runnable=[12] yielded=[12]" "$LOG_FILE"
grep -q "qemu-scheduler-yield-smoke: PASS" "$LOG_FILE"

cat "$LOG_FILE"
