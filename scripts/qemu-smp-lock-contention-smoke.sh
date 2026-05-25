#!/bin/sh
set -eu

TALOS_QEMU_SMP_LOCK_CONTENTION_SMOKE=1 cargo -Zjson-target-spec build "$@"

ELF_FILE="target/aarch64-talos-virt/debug/talos"
IMG_FILE="$ELF_FILE.img"
LOG_FILE="target/qemu-smp-lock-contention-smoke.log"

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

grep -q "qemu-smp-lock-contention: start" "$LOG_FILE"
grep -q "qemu-smp-lock-contention: PASS" "$LOG_FILE"
grep -q "qemu-smp-lock-contention: report logical=1 .* context=1 .* mapped=Some(1) .* lock-count=64 progress=64 target=64 ok=true" "$LOG_FILE"
grep -q "qemu-smp-lock-contention: report logical=2 .* context=2 .* mapped=Some(2) .* lock-count=64 progress=64 target=64 ok=true" "$LOG_FILE"
grep -q "qemu-smp-lock-contention: report logical=3 .* context=3 .* mapped=Some(3) .* lock-count=64 progress=64 target=64 ok=true" "$LOG_FILE"
grep -q "qemu-smp-lock-contention: final counter=192 expected=192 participants=3 errors=0 lock-available=true" "$LOG_FILE"
grep -q "classification=qemu-smp-lock-contention-complete" "$LOG_FILE"

cat "$LOG_FILE"
