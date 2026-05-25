#!/bin/sh
set -eu

TALOS_QEMU_REMOTE_WAKEUP_REQUEST_SMOKE=1 cargo -Zjson-target-spec build "$@"

ELF_FILE="target/aarch64-talos-virt/debug/talos"
IMG_FILE="$ELF_FILE.img"
LOG_FILE="target/qemu-remote-wakeup-request-smoke.log"

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
. "$script_dir/objcopy-tool.sh"

"$objcopy_tool" -O binary "$ELF_FILE" "$IMG_FILE"

qemu-system-aarch64 \
    -M virt,gic-version=2,virtualization=on \
    -cpu cortex-a76 \
    -smp 4 \
    -m 256M \
    -nographic \
    -serial mon:stdio \
    -semihosting-config enable=on,target=native \
    -kernel "$IMG_FILE" >"$LOG_FILE" 2>&1

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
