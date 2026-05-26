#!/bin/sh
set -eu

TALOS_BOOT_SCENARIO=qemu_production_secondary_dispatch cargo -Zjson-target-spec build "$@"

ELF_FILE="target/aarch64-talos-virt/debug/talos"
IMG_FILE="$ELF_FILE.img"
LOG_FILE="target/qemu-production-secondary-dispatch-smoke.log"

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

grep -q "qemu-production-secondary-dispatch: start" "$LOG_FILE"
grep -q "qemu-production-secondary-dispatch: PASS" "$LOG_FILE"
grep -q "qemu-production-secondary-dispatch: report logical=1 .* owner=1 role=secondary-production-diagnostic production=true current=203 queue-len=0 front=0 progress=3 transitions=6 production-dispatches=3 context-switches=3 cross-owner-rejected=true cross-owner-dispatch-rejected=true .* errors=0 ok=true" "$LOG_FILE"
grep -q "qemu-production-secondary-dispatch: report logical=2 .* owner=2 role=secondary-production-diagnostic production=true current=303 queue-len=0 front=0 progress=3 transitions=6 production-dispatches=3 context-switches=3 cross-owner-rejected=true cross-owner-dispatch-rejected=true .* errors=0 ok=true" "$LOG_FILE"
grep -q "qemu-production-secondary-dispatch: report logical=3 .* owner=3 role=secondary-production-diagnostic production=true current=403 queue-len=0 front=0 progress=3 transitions=6 production-dispatches=3 context-switches=3 cross-owner-rejected=true cross-owner-dispatch-rejected=true .* errors=0 ok=true" "$LOG_FILE"
grep -q "qemu-production-secondary-dispatch: final participants=3 expected=3 errors=0 lock-available=true .* classification=qemu-production-secondary-dispatch-complete" "$LOG_FILE"

cat "$LOG_FILE"
