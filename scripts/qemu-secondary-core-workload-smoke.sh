#!/bin/sh
set -eu

LOG_FILE="target/qemu-secondary-core-workload-smoke.log"

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
. "$script_dir/qemu-nographic-smoke-lib.sh"

talos_qemu_prepare_image "qemu_secondary_core_workload" "debug" "" "$@"
talos_qemu_run_nographic "virt,gic-version=2,virtualization=on" "4" "$LOG_FILE"

grep -q "qemu-secondary-core-workload: start" "$LOG_FILE"
grep -q "qemu-secondary-core-workload: PASS" "$LOG_FILE"

cat "$LOG_FILE"
