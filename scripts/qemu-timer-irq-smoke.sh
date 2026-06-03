#!/bin/sh
set -eu

LOG_FILE="target/qemu-timer-irq-smoke.log"

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
. "$script_dir/qemu-nographic-smoke-lib.sh"

talos_qemu_prepare_image "" "debug" "" "$@"
talos_qemu_run_nographic "virt,gic-version=2,virtualization=on" "" "$LOG_FILE"

grep -q "boot-info: .* el=2 " "$LOG_FILE"
grep -q "qemu-timer-irq-smoke: gicd=0x08000000 gicc=0x08010000 intid=26" "$LOG_FILE"
grep -q "qemu-timer-irq-smoke: irq-mask nested-start=true inner-restored=true outer-restored=true unmasked-start=true saved-mask=true restored-unmasked=true" "$LOG_FILE"
grep -Eq "qemu-timer-irq-smoke: tick-count=[4-9][0-9]* target=4 .* intid=26 unexpected=0" "$LOG_FILE"
grep -q "qemu-timer-irq-smoke: PASS" "$LOG_FILE"

cat "$LOG_FILE"
