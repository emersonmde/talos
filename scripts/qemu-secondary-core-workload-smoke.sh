#!/bin/sh
set -eu

TALOS_BOOT_SCENARIO=qemu_secondary_core_workload cargo -Zjson-target-spec build "$@"

ELF_FILE="target/aarch64-talos-virt/debug/talos"
IMG_FILE="$ELF_FILE.img"
LOG_FILE="target/qemu-secondary-core-workload-smoke.log"

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
. "$script_dir/objcopy-tool.sh"
. "$script_dir/qemu-tool.sh"

"$objcopy_tool" -O binary "$ELF_FILE" "$IMG_FILE"

"$qemu_tool" \
    -M virt,gic-version=2,virtualization=on \
    -cpu cortex-a76 \
    -smp 4 \
    -m 256M \
    -nographic \
    -serial mon:stdio \
    -semihosting-config enable=on,target=native \
    -kernel "$IMG_FILE" >"$LOG_FILE" 2>&1

grep -q "qemu-secondary-core-workload: start" "$LOG_FILE"
grep -q "qemu-secondary-core-workload: PASS" "$LOG_FILE"

cat "$LOG_FILE"
