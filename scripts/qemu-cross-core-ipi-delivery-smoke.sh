#!/bin/sh
set -eu

LOG_FILE="target/qemu-cross-core-ipi-delivery-smoke.log"

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
. "$script_dir/qemu-nographic-smoke-lib.sh"

talos_qemu_prepare_image "qemu_cross_core_ipi_delivery" "debug" "" "$@"
talos_qemu_run_nographic "virt,gic-version=2,virtualization=on" "4" "$LOG_FILE"

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
