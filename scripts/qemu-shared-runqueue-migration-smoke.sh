#!/bin/sh
set -eu

LOG_FILE="target/qemu-shared-runqueue-migration-smoke.log"

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
. "$script_dir/qemu-nographic-smoke-lib.sh"

talos_qemu_prepare_image "qemu_shared_runqueue_migration" "debug" "" "$@"
talos_qemu_run_nographic "virt,gic-version=2,virtualization=on" "4" "$LOG_FILE"

grep -q "qemu-shared-runqueue-migration: start" "$LOG_FILE"
grep -q "qemu-shared-runqueue-migration: PASS" "$LOG_FILE"
grep -q "qemu-shared-runqueue-migration: report source-owner=0 destination-owner=1 task=107 task-state=runnable .* publish-reserved-state=migration-reserved publish-queued-state=shared-queued consume-queued-state=shared-queued consume-destination-state=destination-enqueued source-queue-before=1 source-queue-after-publish=0 shared-len-after-publish=1 shared-len-after-consume=0 destination-queue-len=1 destination-front=107 metadata-owner-after-consume=1 .* source-removed=true destination-enqueued=true metadata-migrated=true errors=0 ok=true" "$LOG_FILE"
grep -q "qemu-shared-runqueue-migration: final participants=1 expected=1 errors=0 classification=qemu-shared-runqueue-migration-complete" "$LOG_FILE"

cat "$LOG_FILE"
