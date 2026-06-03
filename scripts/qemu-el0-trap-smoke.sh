#!/bin/sh
set -eu

TALOS_BOOT_SCENARIO=qemu_el0_trap_smoke cargo -Zjson-target-spec build "$@"

ELF_FILE="target/aarch64-talos-virt/debug/talos"
IMG_FILE="$ELF_FILE.img"
LOG_FILE="target/qemu-el0-trap-smoke.log"

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
. "$script_dir/objcopy-tool.sh"
. "$script_dir/qemu-tool.sh"

"$objcopy_tool" -O binary "$ELF_FILE" "$IMG_FILE"

"$qemu_tool" \
    -M virt,gic-version=2,virtualization=on \
    -cpu cortex-a76 \
    -m 256M \
    -nographic \
    -serial mon:stdio \
    -semihosting-config enable=on,target=native \
    -kernel "$IMG_FILE" >"$LOG_FILE" 2>&1

grep -q "boot-info: .* el=2 " "$LOG_FILE"
grep -q "qemu-el0-trap-smoke: start" "$LOG_FILE"
grep -q "qemu-el0-trap-smoke: validated elr=0x0000000000100000 sp=0x0000000000200000 spsr=0x00000000000003c0 guard-blocked=true" "$LOG_FILE"
grep -q "qemu-el0-trap-smoke: trap vector=lower-aarch64-sync esr=0x0000000054007a10 far=0x" "$LOG_FILE"
grep -q " marker=0x7a10" "$LOG_FILE"
grep -q "qemu-el0-trap-smoke: final participants=1 expected=1 errors=0 classification=qemu-el0-trap-smoke-complete" "$LOG_FILE"
grep -q "qemu-el0-trap-smoke: PASS" "$LOG_FILE"

cat "$LOG_FILE"
