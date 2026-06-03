#!/bin/sh
set -eu

LOG_FILE="target/qemu-multicore-preemption-smoke.log"

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
. "$script_dir/qemu-nographic-smoke-lib.sh"

talos_qemu_prepare_image "qemu_multicore_preemption_smoke" "release" "" "$@"
talos_qemu_run_nographic "virt,gic-version=2,virtualization=on" "4" "$LOG_FILE"

grep -q "qemu-multicore-preemption-smoke: start" "$LOG_FILE"
grep -q "qemu-multicore-preemption-smoke: PASS" "$LOG_FILE"
grep -q "qemu-multicore-preemption-smoke: report logical=1 .* owner=1 role=secondary-production-diagnostic current-before-record=201 next=202 queue-len-before-record=1 .* record-outcome=inserted duplicate-outcome=coalesced cross-owner-rejected=true current-after-record=201 queue-len-after-record=1 .* irq-record-scheduler-mutated=false pending-after-record=true service-timer-preemption=202 current-after-service=202 queue-len-after-service=1 front-after-service=201 previous-task-state=runnable selected-task-state=running pending-after-service=false recorded=1 coalesced=1 serviced=1 metadata-owner-after-service=1 metadata-task-after-service=202 .* lock-progress=1 errors=0 ok=true" "$LOG_FILE"
grep -q "qemu-multicore-preemption-smoke: report logical=2 .* owner=2 role=secondary-production-diagnostic current-before-record=301 next=302 queue-len-before-record=1 .* record-outcome=inserted duplicate-outcome=coalesced cross-owner-rejected=true current-after-record=301 queue-len-after-record=1 .* irq-record-scheduler-mutated=false pending-after-record=true service-timer-preemption=302 current-after-service=302 queue-len-after-service=1 front-after-service=301 previous-task-state=runnable selected-task-state=running pending-after-service=false recorded=1 coalesced=1 serviced=1 metadata-owner-after-service=2 metadata-task-after-service=302 .* lock-progress=1 errors=0 ok=true" "$LOG_FILE"
grep -q "qemu-multicore-preemption-smoke: report logical=3 .* owner=3 role=secondary-production-diagnostic current-before-record=401 next=402 queue-len-before-record=1 .* record-outcome=inserted duplicate-outcome=coalesced cross-owner-rejected=true current-after-record=401 queue-len-after-record=1 .* irq-record-scheduler-mutated=false pending-after-record=true service-timer-preemption=402 current-after-service=402 queue-len-after-service=1 front-after-service=401 previous-task-state=runnable selected-task-state=running pending-after-service=false recorded=1 coalesced=1 serviced=1 metadata-owner-after-service=3 metadata-task-after-service=402 .* lock-progress=1 errors=0 ok=true" "$LOG_FILE"
grep -q "qemu-multicore-preemption-smoke: final participants=3 expected=3 errors=0 state-lock-available=true metadata-lock-available=true .* classification=qemu-multicore-preemption-smoke-complete" "$LOG_FILE"

cat "$LOG_FILE"
