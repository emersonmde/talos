#!/bin/sh
set -eu

LOG_FILE="target/qemu-secondary-scheduler-service-loop-smoke.log"

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
. "$script_dir/qemu-nographic-smoke-lib.sh"

talos_qemu_prepare_image "qemu_secondary_scheduler_service_loop" "release" "" "$@"
talos_qemu_run_nographic "virt,gic-version=2,virtualization=on" "4" "$LOG_FILE"

grep -q "qemu-secondary-scheduler-service-loop: start" "$LOG_FILE"
grep -q "qemu-secondary-scheduler-service-loop: PASS" "$LOG_FILE"
grep -q "qemu-secondary-scheduler-service-loop: report logical=1 .* owner=1 role=secondary-production-diagnostic task=201 task-state=running current=201 queue-len=0 front=0 remote-wake=201 dispatch=201 no-work-did-work=false .* observed-remote-wake=true pending-timer-preemption=false dispatch-requested=true cross-owner-rejected=true deferred-role-rejected=true local-queue-preserved=true .* errors=0 ok=true" "$LOG_FILE"
grep -q "qemu-secondary-scheduler-service-loop: report logical=2 .* owner=2 role=secondary-production-diagnostic task=301 task-state=running current=301 queue-len=0 front=0 remote-wake=301 dispatch=301 no-work-did-work=false .* observed-remote-wake=true pending-timer-preemption=false dispatch-requested=true cross-owner-rejected=true deferred-role-rejected=true local-queue-preserved=true .* errors=0 ok=true" "$LOG_FILE"
grep -q "qemu-secondary-scheduler-service-loop: report logical=3 .* owner=3 role=secondary-production-diagnostic task=401 task-state=running current=401 queue-len=0 front=0 remote-wake=401 dispatch=401 no-work-did-work=false .* observed-remote-wake=true pending-timer-preemption=false dispatch-requested=true cross-owner-rejected=true deferred-role-rejected=true local-queue-preserved=true .* errors=0 ok=true" "$LOG_FILE"
grep -q "qemu-secondary-scheduler-service-loop: final participants=3 expected=3 errors=0 state-lock-available=true metadata-lock-available=true final-metadata-len=3 .* classification=qemu-secondary-scheduler-service-loop-complete" "$LOG_FILE"

cat "$LOG_FILE"
