#!/bin/sh
set -eu

cargo -Zjson-target-spec build "$@"

ELF_FILE="target/aarch64-talos-virt/debug/talos"
IMG_FILE="$ELF_FILE.img"
LOG_FILE="target/qemu-timer-irq-smoke.log"

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
grep -q "qemu-timer-irq-smoke: gicd=0x08000000 gicc=0x08010000 intid=26" "$LOG_FILE"
grep -q "qemu-timer-irq-smoke: irq-count=1 .* intid=26 unexpected=0" "$LOG_FILE"
grep -q "qemu-timer-irq-smoke: PASS" "$LOG_FILE"

cat "$LOG_FILE"
