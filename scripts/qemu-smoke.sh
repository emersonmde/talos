#!/bin/sh
set -eu

LOG_FILE="target/qemu-smoke.log"

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
. "$script_dir/qemu-nographic-smoke-lib.sh"

talos_qemu_prepare_image "" "debug" "" "$@"
talos_qemu_run_nographic "virt" "" "$LOG_FILE"

grep -q "Talos" "$LOG_FILE"
grep -q "talos: qemu smoke PASS" "$LOG_FILE"

cat "$LOG_FILE"
