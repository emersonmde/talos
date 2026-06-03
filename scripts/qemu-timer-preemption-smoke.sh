#!/bin/sh
set -eu

LOG_FILE="target/qemu-timer-preemption-smoke.log"

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
. "$script_dir/qemu-nographic-smoke-lib.sh"

talos_qemu_prepare_image "qemu_timer_preemption" "debug" "" "$@"
talos_qemu_run_nographic "virt,gic-version=2,virtualization=on" "" "$LOG_FILE"

grep -q "boot-info: .* el=2 " "$LOG_FILE"
grep -q "qemu-timer-preemption-smoke: start current=1 runnable=2 preempted=0 requests=0" "$LOG_FILE"
grep -Eq "qemu-timer-preemption-smoke: progress task1=[3-9][0-9]* task2=[3-9][0-9]* ticks=[5-9][0-9]* requests=[5-9][0-9]* handled=[5-9][0-9]* timer-preemptions=[5-9][0-9]* dispatch-switches=[5-9][0-9]* voluntary-yields=0 transitions=[6-9][0-9]* current=[12] runnable=[12] preempted=[12]" "$LOG_FILE"
grep -Eq "qemu-timer-preemption-smoke: irq vector=5 iar=0x0000001a intid=26 unexpected=0" "$LOG_FILE"
grep -q "qemu-timer-preemption-smoke: PASS" "$LOG_FILE"

cat "$LOG_FILE"
