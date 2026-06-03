#!/bin/sh
set -eu

LOG_FILE="target/qemu-open-read-syscall-surface-smoke.log"
EVIDENCE_DIR="tasks/evidence/2026-06-03-qemu-open-read-syscall-surface"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-open-read-syscall-surface-smoke.log"

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
. "$script_dir/qemu-nographic-smoke-lib.sh"

talos_qemu_prepare_image "qemu_open_read_syscall_surface_smoke" "debug" "" "$@"
talos_qemu_run_nographic "virt,gic-version=2,virtualization=on" "" "$LOG_FILE"

grep -q "boot-info: .* el=2 " "$LOG_FILE"
grep -Fq "qemu-open-read-syscall-surface-smoke: start" "$LOG_FILE"
grep -Fq "qemu-open-read-syscall-surface-smoke: current-descriptor-table lookup=process-owned owner=0x0000000000000001 resolved=true stdio=inherited initramfs=phase8-readonly" "$LOG_FILE"
grep -Fq "qemu-open-read-syscall-surface-smoke: syscall case=open_invalid_flags number=5 return-x0=0xffffffffffffffea expected=-EINVAL descriptor-leak=false ok=true" "$LOG_FILE"
grep -Fq "qemu-open-read-syscall-surface-smoke: syscall case=open_user_fault number=5 return-x0=0xfffffffffffffff2 expected=-EFAULT descriptor-leak=false ok=true" "$LOG_FILE"
grep -Fq "qemu-open-read-syscall-surface-smoke: syscall case=open_directory number=5 return-x0=0xffffffffffffffeb expected=-EISDIR descriptor-leak=false ok=true" "$LOG_FILE"
grep -Fq "qemu-open-read-syscall-surface-smoke: syscall case=open_banner path=/etc/banner.txt number=5 return-x0=3 object=regular-file ok=true" "$LOG_FILE"
grep -Fq "qemu-open-read-syscall-surface-smoke: syscall case=read_user_fault fd=3 number=4 return-x0=0xfffffffffffffff2 expected=-EFAULT offset-unchanged=true ok=true" "$LOG_FILE"
grep -Fq 'qemu-open-read-syscall-surface-smoke: syscall case=read_banner fd=3 number=4 request=64 return-x0=24 data="Talos initramfs fixture\n" ok=true' "$LOG_FILE"
grep -Fq "qemu-open-read-syscall-surface-smoke: syscall case=read_banner_eof fd=3 number=4 return-x0=0 eof=true ok=true" "$LOG_FILE"
grep -Fq "qemu-open-read-syscall-surface-smoke: syscall case=open_init path=/bin/init number=5 return-x0=4 object=regular-file ok=true" "$LOG_FILE"
grep -Fq "qemu-open-read-syscall-surface-smoke: syscall case=read_init fd=4 number=4 request=4 return-x0=4 elf-magic=7f454c46 ok=true" "$LOG_FILE"
grep -Fq "qemu-open-read-syscall-surface-smoke: syscall case=read_badfd number=4 return-x0=0xfffffffffffffff7 expected=-EBADF ok=true" "$LOG_FILE"
grep -Fq "qemu-open-read-syscall-surface-smoke: final participants=10 expected=10 errors=0 classification=qemu-open-read-syscall-surface-smoke-complete" "$LOG_FILE"
grep -Fq "qemu-open-read-syscall-surface-smoke: PASS" "$LOG_FILE"

mkdir -p "$EVIDENCE_DIR"
tr -d '\r' <"$LOG_FILE" >"$EVIDENCE_LOG"
cat "$EVIDENCE_LOG"
