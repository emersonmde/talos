#!/bin/sh
set -eu

LOG_FILE="target/qemu-per-core-scheduler-ownership-smoke.log"

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
. "$script_dir/qemu-nographic-smoke-lib.sh"

talos_qemu_prepare_image "qemu_per_core_scheduler_ownership" "debug" "" "$@"
talos_qemu_run_nographic "virt,gic-version=2,virtualization=on" "4" "$LOG_FILE"

grep -q "qemu-per-core-scheduler-ownership: start" "$LOG_FILE"
grep -q "qemu-per-core-scheduler-ownership: PASS" "$LOG_FILE"
grep -q "qemu-per-core-scheduler-ownership: report logical=0 .* owner=0 role=boot-production .* progress=4 .* errors=0 ok=true" "$LOG_FILE"
grep -q "qemu-per-core-scheduler-ownership: report logical=1 .* owner=1 role=secondary-deferred .* progress=4 .* dispatch-deferred=true .* errors=0 ok=true" "$LOG_FILE"
grep -q "qemu-per-core-scheduler-ownership: report logical=2 .* owner=2 role=secondary-deferred .* progress=4 .* dispatch-deferred=true .* errors=0 ok=true" "$LOG_FILE"
grep -q "qemu-per-core-scheduler-ownership: report logical=3 .* owner=3 role=secondary-deferred .* progress=4 .* dispatch-deferred=true .* errors=0 ok=true" "$LOG_FILE"
grep -q "qemu-per-core-scheduler-ownership: final participants=4 expected=4 errors=0 lock-available=true irq-ok=true" "$LOG_FILE"
grep -q "classification=qemu-per-core-scheduler-ownership-complete" "$LOG_FILE"

cat "$LOG_FILE"
