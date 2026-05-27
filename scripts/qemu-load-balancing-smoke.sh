#!/bin/sh
set -eu

TALOS_BOOT_SCENARIO=qemu_load_balancing_smoke cargo -Zjson-target-spec build "$@"

ELF_FILE="target/aarch64-talos-virt/debug/talos"
IMG_FILE="$ELF_FILE.img"
LOG_FILE="target/qemu-load-balancing-smoke.log"

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

grep -q "qemu-load-balancing-smoke: start" "$LOG_FILE"
grep -q "qemu-load-balancing-smoke: PASS" "$LOG_FILE"
grep -q "qemu-load-balancing-smoke: report source-owner=0 destination-owner=1 task=109 task-state=runnable registered-generation=.* plan-generation=.* publish-reserved-state=migration-reserved publish-queued-state=shared-queued consume-queued-state=shared-queued consume-destination-state=destination-enqueued source-queue-before=1 source-queue-after-publish=0 shared-len-after-publish=1 shared-len-after-consume=0 destination-queue-len=1 destination-front=109 metadata-owner-after-consume=1 .* selected-front=true source-removed=true destination-enqueued=true metadata-migrated=true errors=0 ok=true" "$LOG_FILE"
grep -q "qemu-load-balancing-smoke: final participants=1 expected=1 errors=0 classification=qemu-load-balancing-smoke-complete" "$LOG_FILE"

cat "$LOG_FILE"
