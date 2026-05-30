#!/bin/sh
set -eu

TALOS_BOOT_SCENARIO=qemu_initial_user_stack_smoke cargo -Zjson-target-spec build "$@"

ELF_FILE="target/aarch64-talos-virt/debug/talos"
IMG_FILE="$ELF_FILE.img"
LOG_FILE="target/qemu-initial-user-stack-smoke.log"
EVIDENCE_DIR="tasks/evidence/2026-05-30-qemu-initial-user-stack-smoke-core"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-initial-user-stack-smoke.log"

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
. "$script_dir/objcopy-tool.sh"

"$objcopy_tool" -O binary "$ELF_FILE" "$IMG_FILE"

qemu-system-aarch64 \
    -M virt,gic-version=2,virtualization=on \
    -cpu cortex-a76 \
    -m 256M \
    -nographic \
    -serial mon:stdio \
    -semihosting-config enable=on,target=native \
    -kernel "$IMG_FILE" >"$LOG_FILE" 2>&1

grep -q "boot-info: .* el=2 " "$LOG_FILE"
grep -Fq "qemu-initial-user-stack-smoke: start" "$LOG_FILE"
grep -Eq "qemu-initial-user-stack-smoke: fixture name=phase8-program-loader-elf64-aarch64-v1 path=/bin/init source-digest=0x[0-9a-f]+ install-boundary=phase8-process-install-plan-v1 address-space-boundary=phase8-process-address-space-model-v1 materialization-boundary=phase8-process-page-table-materialization-v1 launch-boundary=phase8-initial-process-launch-plan-v1 stack-boundary=phase8-initial-user-stack-plan-v1" "$LOG_FILE"
grep -Fq "qemu-initial-user-stack-smoke: success output=InitialUserStackPlan published=true stack-top=0x0000800000000000 initial-sp=0x0000800000000000 sp-aligned-16=true ok=true" "$LOG_FILE"
grep -Fq "qemu-initial-user-stack-smoke: layout usable-start=0x00007fffffffc000 usable-end=0x0000800000000000 guard-start=0x00007fffffffb000 guard-end=0x00007fffffffc000 page-size=0x1000 usable-pages=4 guard-pages=1 ok=true" "$LOG_FILE"
grep -Fq "qemu-initial-user-stack-smoke: ownership usable-user-data=true stack-owned=true guard-has-frame=false guard-has-descriptor=false total-copied-bytes=0 total-zeroed-bytes=0x4000 ok=true" "$LOG_FILE"
grep -Fq "qemu-initial-user-stack-smoke: startup argc=0 argv=null envp=null auxv=blocked-pending-startup-abi tls=blocked-pending-startup-abi copied-startup-bytes=0 ok=true" "$LOG_FILE"
grep -Fq "qemu-initial-user-stack-smoke: launch-binding user-sp-state=model-only-initial-user-stack-ready saved-frame-sp-el0=0x0000800000000000 activation-state=blocked-no-ttbr-activation no-partial-launch=true ok=true" "$LOG_FILE"
grep -Fq "qemu-initial-user-stack-smoke: side-effects ttbr-mutated=false tcr-mutated=false mair-mutated=false sctlr-mutated=false asid-allocated=false tlb-mutated=false lower-el-eret=false scheduler-published=false process-table-mutated=false descriptor-table-mutated=false ok=true" "$LOG_FILE"
grep -Fq "qemu-initial-user-stack-smoke: teardown stack-leases-released=true image-leases-untouched=true idempotent=true ok=true" "$LOG_FILE"
grep -Fq "qemu-initial-user-stack-smoke: error case=identity-mismatch errno=-EINVAL partial-stack=false partial-launch=false ok=true" "$LOG_FILE"
grep -Fq "qemu-initial-user-stack-smoke: error case=range-fault errno=-EFAULT partial-stack=false partial-launch=false ok=true" "$LOG_FILE"
grep -Fq "qemu-initial-user-stack-smoke: error case=image-overlap errno=-EACCES partial-stack=false partial-launch=false ok=true" "$LOG_FILE"
grep -Fq "qemu-initial-user-stack-smoke: error case=executable-stack errno=-EACCES partial-stack=false partial-launch=false ok=true" "$LOG_FILE"
grep -Fq "qemu-initial-user-stack-smoke: error case=capacity-exhausted errno=-ENOMEM partial-stack=false partial-launch=false leases-released=true ok=true" "$LOG_FILE"
grep -Fq "qemu-initial-user-stack-smoke: error case=already-stack-ready errno=-EINVAL partial-stack=false partial-launch=false ok=true" "$LOG_FILE"
grep -Fq "qemu-initial-user-stack-smoke: error case=live-launch-request errno=-ENOSYS partial-stack=false partial-launch=false runnable-published=false ok=true" "$LOG_FILE"
grep -Fq "qemu-initial-user-stack-smoke: final participants=13 expected=13 errors=0 classification=qemu-initial-user-stack-smoke-complete" "$LOG_FILE"
grep -Fq "qemu-initial-user-stack-smoke: PASS" "$LOG_FILE"

mkdir -p "$EVIDENCE_DIR"
tr -d '\r' <"$LOG_FILE" >"$EVIDENCE_LOG"
cat "$EVIDENCE_LOG"
