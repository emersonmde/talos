#!/bin/sh
set -eu

TALOS_BOOT_SCENARIO=qemu_timer_preemption cargo -Zjson-target-spec build "$@"

ELF_FILE="target/aarch64-talos-virt/debug/talos"
IMG_FILE="$ELF_FILE.img"
LOG_FILE="target/qemu-timer-preemption-smoke.log"

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
grep -q "qemu-timer-preemption-smoke: start current=1 runnable=2 preempted=0 requests=0" "$LOG_FILE"
grep -Eq "qemu-timer-preemption-smoke: progress task1=[3-9][0-9]* task2=[3-9][0-9]* ticks=[5-9][0-9]* requests=[5-9][0-9]* handled=[5-9][0-9]* timer-preemptions=[5-9][0-9]* dispatch-switches=[5-9][0-9]* voluntary-yields=0 transitions=[6-9][0-9]* current=[12] runnable=[12] preempted=[12]" "$LOG_FILE"
grep -Eq "qemu-timer-preemption-smoke: irq vector=5 iar=0x0000001a intid=26 unexpected=0" "$LOG_FILE"
grep -q "qemu-timer-preemption-smoke: PASS" "$LOG_FILE"

cat "$LOG_FILE"
