#!/bin/sh
set -eu

LOG_FILE="target/qemu-production-secondary-dispatch-smoke.log"

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
. "$script_dir/qemu-nographic-smoke-lib.sh"

talos_qemu_prepare_image "qemu_production_secondary_dispatch" "debug" "" "$@"
talos_qemu_run_nographic "virt,gic-version=2,virtualization=on" "4" "$LOG_FILE"

grep -q "qemu-production-secondary-dispatch: start" "$LOG_FILE"
grep -q "qemu-production-secondary-dispatch: PASS" "$LOG_FILE"
grep -q "qemu-production-secondary-dispatch: report logical=1 .* owner=1 role=secondary-production-diagnostic production=true current=203 queue-len=0 front=0 progress=3 transitions=6 production-dispatches=3 context-switches=3 cross-owner-rejected=true cross-owner-dispatch-rejected=true .* errors=0 ok=true" "$LOG_FILE"
grep -q "qemu-production-secondary-dispatch: report logical=2 .* owner=2 role=secondary-production-diagnostic production=true current=303 queue-len=0 front=0 progress=3 transitions=6 production-dispatches=3 context-switches=3 cross-owner-rejected=true cross-owner-dispatch-rejected=true .* errors=0 ok=true" "$LOG_FILE"
grep -q "qemu-production-secondary-dispatch: report logical=3 .* owner=3 role=secondary-production-diagnostic production=true current=403 queue-len=0 front=0 progress=3 transitions=6 production-dispatches=3 context-switches=3 cross-owner-rejected=true cross-owner-dispatch-rejected=true .* errors=0 ok=true" "$LOG_FILE"
grep -q "qemu-production-secondary-dispatch: final participants=3 expected=3 errors=0 lock-available=true .* classification=qemu-production-secondary-dispatch-complete" "$LOG_FILE"

cat "$LOG_FILE"
