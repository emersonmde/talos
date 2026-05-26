#!/bin/sh
set -eu

TALOS_BOOT_SCENARIO=qemu_shared_scheduler_metadata cargo -Zjson-target-spec build "$@"

ELF_FILE="target/aarch64-talos-virt/debug/talos"
IMG_FILE="$ELF_FILE.img"
LOG_FILE="target/qemu-shared-scheduler-metadata-smoke.log"

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

grep -q "qemu-shared-scheduler-metadata: start" "$LOG_FILE"
grep -q "qemu-shared-scheduler-metadata: PASS" "$LOG_FILE"
grep -q "qemu-shared-scheduler-metadata: report logical=0 .* owner=0 role=boot-production production=true task=101 task-state=running current=101 queue-len=0 front=0 .* lookup-owner=0 lookup-task=101 .* boot-lookup-owner=0 boot-lookup-task=101 .* cross-owner-rejected=true metadata-cross-owner-rejected=true local-queue-preserved=true .* errors=0 ok=true" "$LOG_FILE"
grep -q "qemu-shared-scheduler-metadata: report logical=1 .* owner=1 role=secondary-production-diagnostic production=true task=201 task-state=running current=201 queue-len=0 front=0 .* lookup-owner=1 lookup-task=201 .* boot-lookup-owner=0 boot-lookup-task=101 .* cross-owner-rejected=true metadata-cross-owner-rejected=true local-queue-preserved=true .* errors=0 ok=true" "$LOG_FILE"
grep -q "qemu-shared-scheduler-metadata: report logical=2 .* owner=2 role=secondary-production-diagnostic production=true task=301 task-state=running current=301 queue-len=0 front=0 .* lookup-owner=2 lookup-task=301 .* boot-lookup-owner=0 boot-lookup-task=101 .* cross-owner-rejected=true metadata-cross-owner-rejected=true local-queue-preserved=true .* errors=0 ok=true" "$LOG_FILE"
grep -q "qemu-shared-scheduler-metadata: report logical=3 .* owner=3 role=secondary-production-diagnostic production=true task=401 task-state=running current=401 queue-len=0 front=0 .* lookup-owner=3 lookup-task=401 .* boot-lookup-owner=0 boot-lookup-task=101 .* cross-owner-rejected=true metadata-cross-owner-rejected=true local-queue-preserved=true .* errors=0 ok=true" "$LOG_FILE"
grep -q "qemu-shared-scheduler-metadata: final participants=4 expected=4 errors=0 state-lock-available=true metadata-lock-available=true final-metadata-len=4 .* classification=qemu-shared-scheduler-metadata-complete" "$LOG_FILE"

cat "$LOG_FILE"
