#!/bin/sh
set -eu

TALOS_BOOT_SCENARIO=qemu_close_syscall_smoke cargo -Zjson-target-spec build "$@"

ELF_FILE="target/aarch64-talos-virt/debug/talos"
IMG_FILE="$ELF_FILE.img"
LOG_FILE="target/qemu-close-syscall-smoke.log"
EVIDENCE_DIR="tasks/evidence/2026-05-29-qemu-close-syscall-smoke-core"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-close-syscall-smoke.log"

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
. "$script_dir/objcopy-tool.sh"
. "$script_dir/qemu-tool.sh"

"$objcopy_tool" -O binary "$ELF_FILE" "$IMG_FILE"

"$qemu_tool" \
    -M virt,gic-version=2,virtualization=on \
    -cpu cortex-a76 \
    -m 256M \
    -nographic \
    -serial mon:stdio \
    -semihosting-config enable=on,target=native \
    -kernel "$IMG_FILE" >"$LOG_FILE" 2>&1

grep -q "boot-info: .* el=2 " "$LOG_FILE"
grep -q "qemu-close-syscall-smoke: start" "$LOG_FILE"
grep -q "qemu-close-syscall-smoke: validated elr=0x0000000000100000 sp=0x0000000000200000 user-data=0x0000000000110000 user-data-len=0x0000000000001000 guard-blocked=true process-owner=0x0000000000000001 current-owner=0x0000000000000001 descriptor-table=process-owned-inherited-stdio runtime-console=runtime-console0" "$LOG_FILE"
grep -q "qemu-close-syscall-smoke: current-descriptor-table lookup=process-owned owner=0x0000000000000001 resolved=true stdio=inherited runtime-console=runtime-console0" "$LOG_FILE"
grep -q "qemu-close-syscall-smoke: syscall case=close_stdout vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=2 args=\[x0=0x0000000000000001 x1=0x0000000000000000 x2=0x0000000000000000 x3=0x0000000000000000 x4=0x0000000000000000 x5=0x0000000000000000\] descriptor-owner=0x0000000000000001 return-x0=0x0000000000000000" "$LOG_FILE"
grep -q "qemu-close-syscall-smoke: syscall case=write_closed_stdout vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=1 return-x0=0xfffffffffffffff7 expected=-EBADF console-unchanged=true" "$LOG_FILE"
grep -q "qemu-close-syscall-smoke: syscall case=close_stderr_reserved vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=2 return-x0=0xffffffffffffffea expected=-EINVAL descriptor=0x0000000000000002 table-unchanged=true" "$LOG_FILE"
grep -q "qemu-close-syscall-smoke: syscall case=write_stderr_after_stdout_close vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=1 args=\[x0=0x0000000000000002 x1=0x0000000000110040 x2=0x0000000000000012 x3=0x0000000000000000 x4=0x0000000000000000 x5=0x0000000000000000\] descriptor-owner=0x0000000000000001 return-x0=0x0000000000000012" "$LOG_FILE"
grep -q "qemu-close-syscall-smoke: runtime-console case=write_stderr_after_stdout_close device=runtime-console0 bytes=18 hex=74616c6f732d7374646572722d71656d750a ok=true" "$LOG_FILE"
grep -q "qemu-close-syscall-smoke: user-observed case=write_stderr_after_stdout_close x0=0x0000000000000012 ok=true" "$LOG_FILE"
grep -q "qemu-close-syscall-smoke: syscall case=close_stderr vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=2 args=\[x0=0x0000000000000002 x1=0x0000000000000000 x2=0x0000000000000000 x3=0x0000000000000000 x4=0x0000000000000000 x5=0x0000000000000000\] descriptor-owner=0x0000000000000001 return-x0=0x0000000000000000" "$LOG_FILE"
grep -q "qemu-close-syscall-smoke: syscall case=write_closed_stderr vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=1 return-x0=0xfffffffffffffff7 expected=-EBADF console-unchanged=true" "$LOG_FILE"
grep -q "qemu-close-syscall-smoke: syscall case=close_stdout_again vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=2 return-x0=0xfffffffffffffff7 expected=-EBADF table-unchanged=true" "$LOG_FILE"
grep -q "qemu-close-syscall-smoke: syscall case=close_badfd vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=2 return-x0=0xfffffffffffffff7 expected=-EBADF table-unchanged=true" "$LOG_FILE"
grep -q "qemu-close-syscall-smoke: syscall case=talos_nop vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=0 return-x0=0x0000000000000000" "$LOG_FILE"
grep -q "qemu-close-syscall-smoke: syscall case=unknown vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=17 return-x0=0xffffffffffffffda expected=-ENOSYS" "$LOG_FILE"
grep -q "qemu-close-syscall-smoke: syscall case=copy_probe_quarantine vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=0x0000000000007001 return-x0=0xffffffffffffffda expected=-ENOSYS dispatched=false" "$LOG_FILE"
grep -q "qemu-close-syscall-smoke: diagnostic-marker marker=0x7a10 stable-syscall=false dispatched=false" "$LOG_FILE"
grep -q "qemu-close-syscall-smoke: final participants=11 expected=11 errors=0 classification=qemu-close-syscall-smoke-complete" "$LOG_FILE"
grep -q "qemu-close-syscall-smoke: PASS" "$LOG_FILE"

mkdir -p "$EVIDENCE_DIR"
tr -d '\r' <"$LOG_FILE" >"$EVIDENCE_LOG"
cat "$EVIDENCE_LOG"
