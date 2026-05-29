#!/bin/sh
set -eu

TALOS_BOOT_SCENARIO=qemu_syscall_smoke cargo -Zjson-target-spec build "$@"

ELF_FILE="target/aarch64-talos-virt/debug/talos"
IMG_FILE="$ELF_FILE.img"
LOG_FILE="target/qemu-syscall-smoke.log"

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
. "$script_dir/objcopy-tool.sh"

"$objcopy_tool" -O binary "$ELF_FILE" "$IMG_FILE"

qemu-system-aarch64 \
    -M virt,gic-version=2,virtualization=on \
    -cpu cortex-a76 \
    -m 256M \
    -nographic \
    -serial mon:stdio \
    -semihosting-config enable=on,target=native \
    -kernel "$IMG_FILE" >"$LOG_FILE" 2>&1

grep -q "boot-info: .* el=2 " "$LOG_FILE"
grep -q "qemu-syscall-smoke: start" "$LOG_FILE"
grep -q "qemu-syscall-smoke: validated elr=0x0000000000100000 sp=0x0000000000200000 spsr=0x00000000000003c0 guard-blocked=true" "$LOG_FILE"
grep -q "qemu-syscall-smoke: syscall case=talos_nop vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=0 args=\[x0=0x0000000000000000 x1=0x0000000000000000 x2=0x0000000000000000 x3=0x0000000000000000 x4=0x0000000000000000 x5=0x0000000000000000\] return-x0=0x0000000000000000" "$LOG_FILE"
grep -q "qemu-syscall-smoke: user-observed case=talos_nop x0=0x0000000000000000 ok=true" "$LOG_FILE"
grep -q "qemu-syscall-smoke: syscall case=unknown vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=17 return-x0=0xffffffffffffffda expected=-ENOSYS" "$LOG_FILE"
grep -q "qemu-syscall-smoke: user-observed case=unknown x0=0xffffffffffffffda ok=true" "$LOG_FILE"
grep -q "qemu-syscall-smoke: diagnostic-marker marker=0x7a10 stable-syscall=false dispatched=false" "$LOG_FILE"
grep -q "qemu-syscall-smoke: final participants=2 expected=2 errors=0 classification=qemu-syscall-smoke-complete" "$LOG_FILE"
grep -q "qemu-syscall-smoke: PASS" "$LOG_FILE"

cat "$LOG_FILE"
