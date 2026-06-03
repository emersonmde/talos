#!/bin/sh
set -eu

LOG_FILE="target/qemu-remote-wakeup-request-smoke.log"

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
. "$script_dir/qemu-nographic-smoke-lib.sh"

talos_qemu_prepare_image "qemu_remote_wakeup_request" "debug" "" "$@"
talos_qemu_run_nographic "virt,gic-version=2,virtualization=on" "4" "$LOG_FILE"

grep -q "qemu-remote-wakeup-request: start" "$LOG_FILE"
grep -q "qemu-remote-wakeup-request: PASS" "$LOG_FILE"
grep -q "qemu-remote-wakeup-request: publish requester=0 target=1 task=201 outcome=inserted" "$LOG_FILE"
grep -q "qemu-remote-wakeup-request: publish requester=0 target=1 task=201 outcome=duplicate" "$LOG_FILE"
grep -q "qemu-remote-wakeup-request: publish requester=0 target=2 task=202 outcome=inserted" "$LOG_FILE"
grep -q "qemu-remote-wakeup-request: publish requester=0 target=3 task=203 outcome=inserted" "$LOG_FILE"
grep -q "qemu-remote-wakeup-request: send sender=0 target-logical=1 target-list-bit=0x02 sgi-intid=1 sgir=0x00020001" "$LOG_FILE"
grep -q "qemu-remote-wakeup-request: send sender=0 target-logical=2 target-list-bit=0x04 sgi-intid=1 sgir=0x00040001" "$LOG_FILE"
grep -q "qemu-remote-wakeup-request: send sender=0 target-logical=3 target-list-bit=0x08 sgi-intid=1 sgir=0x00080001" "$LOG_FILE"
grep -q "qemu-remote-wakeup-request: report sender=0 receiver=1 .* receive-count=1 eoi-count=1 pending-count=1 consumed-task=201 duplicate-count=1 queue-len-after=0 cross-owner-rejected=true production-deferred=true .* ok=true" "$LOG_FILE"
grep -q "qemu-remote-wakeup-request: report sender=0 receiver=2 .* receive-count=1 eoi-count=1 pending-count=1 consumed-task=202 duplicate-count=0 queue-len-after=0 cross-owner-rejected=true production-deferred=true .* ok=true" "$LOG_FILE"
grep -q "qemu-remote-wakeup-request: report sender=0 receiver=3 .* receive-count=1 eoi-count=1 pending-count=1 consumed-task=203 duplicate-count=0 queue-len-after=0 cross-owner-rejected=true production-deferred=true .* ok=true" "$LOG_FILE"
grep -q "qemu-remote-wakeup-request: final participants=3 expected=3 errors=0 .* classification=qemu-remote-wakeup-request-complete" "$LOG_FILE"

cat "$LOG_FILE"
