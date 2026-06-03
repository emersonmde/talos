#!/bin/sh
set -eu

LOG_FILE="target/qemu-smp-lock-contention-smoke.log"

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
. "$script_dir/qemu-nographic-smoke-lib.sh"

talos_qemu_prepare_image "qemu_smp_lock_contention" "debug" "" "$@"
talos_qemu_run_nographic "virt,gic-version=2,virtualization=on" "4" "$LOG_FILE"

grep -q "qemu-smp-lock-contention: start" "$LOG_FILE"
grep -q "qemu-smp-lock-contention: PASS" "$LOG_FILE"
grep -q "qemu-smp-lock-contention: report logical=1 .* context=1 .* mapped=Some(1) .* lock-count=64 progress=64 target=64 ok=true" "$LOG_FILE"
grep -q "qemu-smp-lock-contention: report logical=2 .* context=2 .* mapped=Some(2) .* lock-count=64 progress=64 target=64 ok=true" "$LOG_FILE"
grep -q "qemu-smp-lock-contention: report logical=3 .* context=3 .* mapped=Some(3) .* lock-count=64 progress=64 target=64 ok=true" "$LOG_FILE"
grep -q "qemu-smp-lock-contention: final counter=192 expected=192 participants=3 errors=0 lock-available=true" "$LOG_FILE"
grep -q "classification=qemu-smp-lock-contention-complete" "$LOG_FILE"

cat "$LOG_FILE"
