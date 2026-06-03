#!/bin/sh
set -eu

TALOS_BOOT_SCENARIO=qemu_process_install_smoke cargo -Zjson-target-spec build "$@"

ELF_FILE="target/aarch64-talos-virt/debug/talos"
IMG_FILE="$ELF_FILE.img"
LOG_FILE="target/qemu-process-install-smoke.log"
EVIDENCE_DIR="tasks/evidence/2026-05-30-qemu-process-install-smoke-core"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-process-install-smoke.log"

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
grep -Fq "qemu-process-install-smoke: start" "$LOG_FILE"
grep -Eq "qemu-process-install-smoke: fixture name=phase8-program-loader-elf64-aarch64-v1 path=/bin/init source-digest=0x[0-9a-f]+ install-boundary=phase8-process-install-plan-v1" "$LOG_FILE"
grep -Eq "qemu-process-install-smoke: success output=ProcessImageInstallPlan metadata-only=true entry=0x[0-9a-f]+ entry-preserved=true footprint=0x[0-9a-f]+ pages=[0-9]+ ok=true" "$LOG_FILE"
grep -Eq "qemu-process-install-smoke: page index=0 kind=UserText flags=R-X copy-offset=0x[0-9a-f]+ copy-len=0x[0-9a-f]+ zero-offset=0x[0-9a-f]+ zero-len=0x[0-9a-f]+ action-order=allocate,copy,zero,map permission-widened=false ok=true" "$LOG_FILE"
grep -Eq "qemu-process-install-smoke: page index=1 kind=UserData flags=RW- copy-offset=0x[0-9a-f]+ copy-len=0x[0-9a-f]+ zero-offset=0x[0-9a-f]+ zero-len=0x[0-9a-f]+ action-order=allocate,copy,zero,map permission-widened=false ok=true" "$LOG_FILE"
grep -Fq "qemu-process-install-smoke: side-effects frames-allocated=0 mappings-installed=0 process-created=false descriptors-mutated=false lower-el-frame=false runnable=false ok=true" "$LOG_FILE"
grep -Fq "qemu-process-install-smoke: error case=bad-plan-invariant errno=-EINVAL partial-install=false ok=true" "$LOG_FILE"
grep -Fq "qemu-process-install-smoke: error case=overlap errno=-EACCES partial-install=false ok=true" "$LOG_FILE"
grep -Fq "qemu-process-install-smoke: error case=permission-widening errno=-EACCES partial-install=false ok=true" "$LOG_FILE"
grep -Fq "qemu-process-install-smoke: error case=bad-entry errno=-ENOEXEC partial-install=false ok=true" "$LOG_FILE"
grep -Fq "qemu-process-install-smoke: error case=budget-overflow errno=-ENOMEM partial-install=false ok=true" "$LOG_FILE"
grep -Fq "qemu-process-install-smoke: final participants=7 expected=7 errors=0 classification=qemu-process-install-smoke-complete" "$LOG_FILE"
grep -Fq "qemu-process-install-smoke: PASS" "$LOG_FILE"

mkdir -p "$EVIDENCE_DIR"
tr -d '\r' <"$LOG_FILE" >"$EVIDENCE_LOG"
cat "$EVIDENCE_LOG"
