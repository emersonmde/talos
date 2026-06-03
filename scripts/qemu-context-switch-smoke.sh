#!/bin/sh
set -eu

TALOS_BOOT_SCENARIO=qemu_context_switch cargo -Zjson-target-spec build "$@"

ELF_FILE="target/aarch64-talos-virt/debug/talos"
IMG_FILE="$ELF_FILE.img"
LOG_FILE="target/qemu-context-switch-smoke.log"

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
grep -q "qemu-context-switch-smoke: start current=0 runnable=1" "$LOG_FILE"
grep -Eq "qemu-context-switch-smoke: progress task1=[2-9][0-9]* task2=[2-9][0-9]* switches=[5-9][0-9]* current=[12] runnable=0" "$LOG_FILE"
grep -q "qemu-context-switch-smoke: PASS" "$LOG_FILE"

cat "$LOG_FILE"
