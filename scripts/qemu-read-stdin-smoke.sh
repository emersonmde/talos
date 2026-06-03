#!/bin/sh
set -eu

TALOS_BOOT_SCENARIO=qemu_read_stdin_smoke cargo -Zjson-target-spec build "$@"

ELF_FILE="target/aarch64-talos-virt/debug/talos"
IMG_FILE="$ELF_FILE.img"
LOG_FILE="target/qemu-read-stdin-smoke.log"
EVIDENCE_DIR="tasks/evidence/2026-05-29-qemu-read-stdin-smoke-core"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-read-stdin-smoke.log"

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
grep -q "qemu-read-stdin-smoke: start" "$LOG_FILE"
grep -q "qemu-read-stdin-smoke: validated elr=0x0000000000100000 sp=0x0000000000200000 user-data=0x0000000000110000 user-data-len=0x0000000000001000 guard-blocked=true process-owner=0x0000000000000001 current-owner=0x0000000000000001 descriptor-table=process-owned-inherited-stdio descriptor-capacity=4 fixed-stdin-len=17 fixed-stdin-cursor=0" "$LOG_FILE"
grep -q "qemu-read-stdin-smoke: current-descriptor-table lookup=process-owned owner=0x0000000000000001 resolved=true stdio=inherited fixed-stdin=proof-buffer" "$LOG_FILE"
grep -q "qemu-read-stdin-smoke: syscall case=dup_stdin vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=3 args=\[x0=0x0000000000000000 x1=0x0000000000000000 x2=0x0000000000000000 x3=0x0000000000000000 x4=0x0000000000000000 x5=0x0000000000000000\] descriptor-owner=0x0000000000000001 return-x0=0x0000000000000003 lowest-free=true source-open=true" "$LOG_FILE"
grep -q "qemu-read-stdin-smoke: syscall case=read_guard vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=4 return-x0=0xfffffffffffffff2 expected=-EFAULT fixed-stdin-cursor=0 user-unchanged=true" "$LOG_FILE"
grep -q "qemu-read-stdin-smoke: syscall case=read_reserved vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=4 return-x0=0xffffffffffffffea expected=-EINVAL fixed-stdin-cursor=0 user-unchanged=true" "$LOG_FILE"
grep -q "qemu-read-stdin-smoke: syscall case=read_fd1 vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=4 return-x0=0xfffffffffffffff7 expected=-EBADF fixed-stdin-cursor=0 user-unchanged=true" "$LOG_FILE"
grep -q "qemu-read-stdin-smoke: syscall case=read_badfd vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=4 return-x0=0xfffffffffffffff7 expected=-EBADF fixed-stdin-cursor=0 user-unchanged=true" "$LOG_FILE"
grep -q "qemu-read-stdin-smoke: syscall case=read_stdin_first vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=4 args=\[x0=0x0000000000000000 x1=0x0000000000110080 x2=0x0000000000000005 x3=0x0000000000000000 x4=0x0000000000000000 x5=0x0000000000000000\] descriptor-owner=0x0000000000000001 return-x0=0x0000000000000005 fixed-stdin-cursor=5" "$LOG_FILE"
grep -q "qemu-read-stdin-smoke: user-buffer case=read_stdin_first addr=0x0000000000110080 bytes=5 hex=74616c6f73 ok=true" "$LOG_FILE"
grep -q "qemu-read-stdin-smoke: user-observed case=read_stdin_first x0=0x0000000000000005 ok=true" "$LOG_FILE"
grep -q "qemu-read-stdin-smoke: syscall case=read_stdin_duplicate_remaining vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=4 args=\[x0=0x0000000000000003 x1=0x00000000001100a0 x2=0x0000000000000020 x3=0x0000000000000000 x4=0x0000000000000000 x5=0x0000000000000000\] descriptor-owner=0x0000000000000001 return-x0=0x000000000000000c fixed-stdin-cursor=17 short-read=true" "$LOG_FILE"
grep -q "qemu-read-stdin-smoke: user-buffer case=read_stdin_duplicate_remaining addr=0x00000000001100a0 bytes=12 hex=2d737464696e2d71656d750a ok=true" "$LOG_FILE"
grep -q "qemu-read-stdin-smoke: user-observed case=read_stdin_duplicate_remaining x0=0x000000000000000c ok=true" "$LOG_FILE"
grep -q "qemu-read-stdin-smoke: syscall case=read_stdin_eof vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=4 return-x0=0x0000000000000000 fixed-stdin-cursor=17 user-unchanged=true eof=true" "$LOG_FILE"
grep -q "qemu-read-stdin-smoke: syscall case=talos_nop vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=0 return-x0=0x0000000000000000" "$LOG_FILE"
grep -q "qemu-read-stdin-smoke: syscall case=unknown vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=17 return-x0=0xffffffffffffffda expected=-ENOSYS" "$LOG_FILE"
grep -q "qemu-read-stdin-smoke: syscall case=copy_probe_quarantine vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=0x0000000000007001 return-x0=0xffffffffffffffda expected=-ENOSYS dispatched=false" "$LOG_FILE"
grep -q "qemu-read-stdin-smoke: diagnostic-marker marker=0x7a10 stable-syscall=false dispatched=false" "$LOG_FILE"
grep -q "qemu-read-stdin-smoke: final participants=11 expected=11 errors=0 classification=qemu-read-stdin-smoke-complete" "$LOG_FILE"
grep -q "qemu-read-stdin-smoke: PASS" "$LOG_FILE"

mkdir -p "$EVIDENCE_DIR"
tr -d '\r' <"$LOG_FILE" >"$EVIDENCE_LOG"
cat "$EVIDENCE_LOG"
