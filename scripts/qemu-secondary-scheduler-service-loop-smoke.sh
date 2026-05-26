#!/bin/sh
set -eu

TALOS_BOOT_SCENARIO=qemu_secondary_scheduler_service_loop cargo -Zjson-target-spec build --release "$@"

ELF_FILE="target/aarch64-talos-virt/release/talos"
IMG_FILE="$ELF_FILE.img"
LOG_FILE="target/qemu-secondary-scheduler-service-loop-smoke.log"

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
. "$script_dir/objcopy-tool.sh"

"$objcopy_tool" -O binary "$ELF_FILE" "$IMG_FILE"

qemu-system-aarch64 \
    -M virt,gic-version=2,virtualization=on \
    -cpu cortex-a76 \
    -smp 4 \
    -m 256M \
    -nographic \
    -serial mon:stdio \
    -semihosting-config enable=on,target=native \
    -kernel "$IMG_FILE" >"$LOG_FILE" 2>&1

grep -q "qemu-secondary-scheduler-service-loop: start" "$LOG_FILE"
grep -q "qemu-secondary-scheduler-service-loop: PASS" "$LOG_FILE"
grep -q "qemu-secondary-scheduler-service-loop: report logical=1 .* owner=1 role=secondary-production-diagnostic task=201 task-state=running current=201 queue-len=0 front=0 remote-wake=201 dispatch=201 no-work-did-work=false .* observed-remote-wake=true pending-timer-preemption=false dispatch-requested=true cross-owner-rejected=true deferred-role-rejected=true local-queue-preserved=true .* errors=0 ok=true" "$LOG_FILE"
grep -q "qemu-secondary-scheduler-service-loop: report logical=2 .* owner=2 role=secondary-production-diagnostic task=301 task-state=running current=301 queue-len=0 front=0 remote-wake=301 dispatch=301 no-work-did-work=false .* observed-remote-wake=true pending-timer-preemption=false dispatch-requested=true cross-owner-rejected=true deferred-role-rejected=true local-queue-preserved=true .* errors=0 ok=true" "$LOG_FILE"
grep -q "qemu-secondary-scheduler-service-loop: report logical=3 .* owner=3 role=secondary-production-diagnostic task=401 task-state=running current=401 queue-len=0 front=0 remote-wake=401 dispatch=401 no-work-did-work=false .* observed-remote-wake=true pending-timer-preemption=false dispatch-requested=true cross-owner-rejected=true deferred-role-rejected=true local-queue-preserved=true .* errors=0 ok=true" "$LOG_FILE"
grep -q "qemu-secondary-scheduler-service-loop: final participants=3 expected=3 errors=0 state-lock-available=true metadata-lock-available=true final-metadata-len=3 .* classification=qemu-secondary-scheduler-service-loop-complete" "$LOG_FILE"

cat "$LOG_FILE"
