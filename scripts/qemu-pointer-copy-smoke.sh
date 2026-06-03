#!/bin/sh
set -eu

LOG_FILE="target/qemu-pointer-copy-smoke.log"
EVIDENCE_DIR="tasks/evidence/2026-05-29-qemu-pointer-copy-smoke-core"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-pointer-copy-smoke.log"

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
. "$script_dir/qemu-nographic-smoke-lib.sh"

talos_qemu_prepare_image "qemu_pointer_copy_smoke" "debug" "" "$@"
talos_qemu_run_nographic "virt,gic-version=2,virtualization=on" "" "$LOG_FILE"

grep -q "boot-info: .* el=2 " "$LOG_FILE"
grep -q "qemu-pointer-copy-smoke: start" "$LOG_FILE"
grep -q "qemu-pointer-copy-smoke: validated elr=0x0000000000100000 sp=0x0000000000200000 user-data=0x0000000000110000 user-data-len=0x0000000000001000 guard-blocked=true" "$LOG_FILE"
grep -q "qemu-pointer-copy-smoke: syscall case=copy_probe_success vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=0x0000000000007001 args=\[x0=0x0000000000110000 x1=0x0000000000000010 x2=0x000000000000002a x3=0x00000000000000a5 x4=0x0000000000000000 x5=0x0000000000000000\] return-x0=0x0000000000000010" "$LOG_FILE"
grep -q "qemu-pointer-copy-smoke: user-observed case=copy_probe_success x0=0x0000000000000010 data=0xa5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5 ok=true" "$LOG_FILE"
grep -q "qemu-pointer-copy-smoke: syscall case=copy_probe_efault vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=0x0000000000007001 args=\[x0=0x00000000001e0000 x1=0x0000000000000010 x2=0x000000000000002a x3=0x00000000000000a5 x4=0x0000000000000000 x5=0x0000000000000000\] return-x0=0xfffffffffffffff2 expected=-EFAULT" "$LOG_FILE"
grep -q "qemu-pointer-copy-smoke: user-observed case=copy_probe_efault x0=0xfffffffffffffff2 ok=true" "$LOG_FILE"
grep -q "qemu-pointer-copy-smoke: syscall case=unknown vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=17 return-x0=0xffffffffffffffda expected=-ENOSYS" "$LOG_FILE"
grep -q "qemu-pointer-copy-smoke: user-observed case=unknown x0=0xffffffffffffffda ok=true" "$LOG_FILE"
grep -q "qemu-pointer-copy-smoke: diagnostic-marker marker=0x7a10 stable-syscall=false dispatched=false" "$LOG_FILE"
grep -q "qemu-pointer-copy-smoke: final participants=3 expected=3 errors=0 classification=qemu-pointer-copy-smoke-complete" "$LOG_FILE"
grep -q "qemu-pointer-copy-smoke: PASS" "$LOG_FILE"

mkdir -p "$EVIDENCE_DIR"
cp "$LOG_FILE" "$EVIDENCE_LOG"
cat "$EVIDENCE_LOG"
