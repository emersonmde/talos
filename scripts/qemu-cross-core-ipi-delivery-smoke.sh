#!/bin/sh
set -eu

TALOS_BOOT_SCENARIO=qemu_cross_core_ipi_delivery cargo -Zjson-target-spec build "$@"

ELF_FILE="target/aarch64-talos-virt/debug/talos"
IMG_FILE="$ELF_FILE.img"
LOG_FILE="target/qemu-cross-core-ipi-delivery-smoke.log"

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

grep -q "qemu-cross-core-ipi-delivery: start" "$LOG_FILE"
grep -q "qemu-cross-core-ipi-delivery: PASS" "$LOG_FILE"
grep -q "qemu-cross-core-ipi-delivery: send sender=0 target-logical=1 target-list-bit=0x02 sgi-intid=1 sgir=0x00020001" "$LOG_FILE"
grep -q "qemu-cross-core-ipi-delivery: send sender=0 target-logical=2 target-list-bit=0x04 sgi-intid=1 sgir=0x00040001" "$LOG_FILE"
grep -q "qemu-cross-core-ipi-delivery: send sender=0 target-logical=3 target-list-bit=0x08 sgi-intid=1 sgir=0x00080001" "$LOG_FILE"
grep -q "qemu-cross-core-ipi-delivery: report sender=0 receiver=1 .* target-list-bit=0x02 .* intid=1 receive-count=1 eoi-count=1 .* ok=true" "$LOG_FILE"
grep -q "qemu-cross-core-ipi-delivery: report sender=0 receiver=2 .* target-list-bit=0x04 .* intid=1 receive-count=1 eoi-count=1 .* ok=true" "$LOG_FILE"
grep -q "qemu-cross-core-ipi-delivery: report sender=0 receiver=3 .* target-list-bit=0x08 .* intid=1 receive-count=1 eoi-count=1 .* ok=true" "$LOG_FILE"
grep -q "qemu-cross-core-ipi-delivery: final participants=3 expected=3 errors=0 .* classification=qemu-cross-core-ipi-delivery-complete" "$LOG_FILE"

cat "$LOG_FILE"
