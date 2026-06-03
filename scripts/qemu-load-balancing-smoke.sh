#!/bin/sh
set -eu

LOG_FILE="target/qemu-load-balancing-smoke.log"

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
. "$script_dir/qemu-nographic-smoke-lib.sh"

talos_qemu_prepare_image "qemu_load_balancing_smoke" "debug" "" "$@"
talos_qemu_run_nographic "virt,gic-version=2,virtualization=on" "4" "$LOG_FILE"

grep -q "qemu-load-balancing-smoke: start" "$LOG_FILE"
grep -q "qemu-load-balancing-smoke: PASS" "$LOG_FILE"
grep -q "qemu-load-balancing-smoke: report source-owner=0 destination-owner=1 task=109 task-state=runnable registered-generation=.* plan-generation=.* publish-reserved-state=migration-reserved publish-queued-state=shared-queued consume-queued-state=shared-queued consume-destination-state=destination-enqueued source-queue-before=1 source-queue-after-publish=0 shared-len-after-publish=1 shared-len-after-consume=0 destination-queue-len=1 destination-front=109 metadata-owner-after-consume=1 .* selected-front=true source-removed=true destination-enqueued=true metadata-migrated=true errors=0 ok=true" "$LOG_FILE"
grep -q "qemu-load-balancing-smoke: final participants=1 expected=1 errors=0 classification=qemu-load-balancing-smoke-complete" "$LOG_FILE"

cat "$LOG_FILE"
