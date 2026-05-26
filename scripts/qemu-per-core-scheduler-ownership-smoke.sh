#!/bin/sh
set -eu

TALOS_BOOT_SCENARIO=qemu_per_core_scheduler_ownership cargo -Zjson-target-spec build "$@"

ELF_FILE="target/aarch64-talos-virt/debug/talos"
IMG_FILE="$ELF_FILE.img"
LOG_FILE="target/qemu-per-core-scheduler-ownership-smoke.log"

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

grep -q "qemu-per-core-scheduler-ownership: start" "$LOG_FILE"
grep -q "qemu-per-core-scheduler-ownership: PASS" "$LOG_FILE"
grep -q "qemu-per-core-scheduler-ownership: report logical=0 .* owner=0 role=boot-production .* progress=4 .* errors=0 ok=true" "$LOG_FILE"
grep -q "qemu-per-core-scheduler-ownership: report logical=1 .* owner=1 role=secondary-deferred .* progress=4 .* dispatch-deferred=true .* errors=0 ok=true" "$LOG_FILE"
grep -q "qemu-per-core-scheduler-ownership: report logical=2 .* owner=2 role=secondary-deferred .* progress=4 .* dispatch-deferred=true .* errors=0 ok=true" "$LOG_FILE"
grep -q "qemu-per-core-scheduler-ownership: report logical=3 .* owner=3 role=secondary-deferred .* progress=4 .* dispatch-deferred=true .* errors=0 ok=true" "$LOG_FILE"
grep -q "qemu-per-core-scheduler-ownership: final participants=4 expected=4 errors=0 lock-available=true irq-ok=true" "$LOG_FILE"
grep -q "classification=qemu-per-core-scheduler-ownership-complete" "$LOG_FILE"

cat "$LOG_FILE"
