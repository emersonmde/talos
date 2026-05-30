#!/bin/sh
set -eu

TALOS_BOOT_SCENARIO=qemu_initial_process_launch_smoke cargo -Zjson-target-spec build "$@"

ELF_FILE="target/aarch64-talos-virt/debug/talos"
IMG_FILE="$ELF_FILE.img"
LOG_FILE="target/qemu-initial-process-launch-smoke.log"
EVIDENCE_DIR="tasks/evidence/2026-05-30-qemu-initial-process-launch-smoke-core"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-initial-process-launch-smoke.log"

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
grep -Fq "qemu-initial-process-launch-smoke: start" "$LOG_FILE"
grep -Eq "qemu-initial-process-launch-smoke: fixture name=phase8-program-loader-elf64-aarch64-v1 path=/bin/init source-digest=0x[0-9a-f]+ install-boundary=phase8-process-install-plan-v1 address-space-boundary=phase8-process-address-space-model-v1 materialization-boundary=phase8-process-page-table-materialization-v1 launch-boundary=phase8-initial-process-launch-plan-v1" "$LOG_FILE"
grep -Eq "qemu-initial-process-launch-smoke: success output=InitialProcessLaunchPlan published=true entry=0x[0-9a-f]+ user-sp-state=blocked-missing-initial-user-stack activation-state=blocked-no-ttbr-activation ok=true" "$LOG_FILE"
grep -Fq "qemu-initial-process-launch-smoke: entry provenance image=true install=true address-space-user-text=true materialization-user-text-descriptor=true el0-executable=true ok=true" "$LOG_FILE"
grep -Fq "qemu-initial-process-launch-smoke: saved-frame-intent elr=entry-pc sp-el0=blocked-missing-initial-user-stack spsr=blocked-pending-lower-el-pstate-policy x0-x5=blocked-pending-startup-abi daif=blocked-pending-interrupt-mask-policy address-space-token=model-only ok=true" "$LOG_FILE"
grep -Fq "qemu-initial-process-launch-smoke: side-effects ttbr-mutated=false tcr-mutated=false mair-mutated=false sctlr-mutated=false asid-allocated=false tlb-mutated=false lower-el-eret=false scheduler-published=false process-table-mutated=false descriptor-table-mutated=false ok=true" "$LOG_FILE"
grep -Fq "qemu-initial-process-launch-smoke: commit-request target=runnable errno=-ENOSYS no-partial-launch=true no-runnable-publication=true ok=true" "$LOG_FILE"
grep -Fq "qemu-initial-process-launch-smoke: error case=identity-mismatch errno=-EINVAL partial-launch=false runnable-published=false ok=true" "$LOG_FILE"
grep -Fq "qemu-initial-process-launch-smoke: error case=entry-mismatch errno=-ENOEXEC partial-launch=false runnable-published=false ok=true" "$LOG_FILE"
grep -Fq "qemu-initial-process-launch-smoke: error case=missing-user-text-descriptor errno=-ENOEXEC partial-launch=false runnable-published=false ok=true" "$LOG_FILE"
grep -Fq "qemu-initial-process-launch-smoke: error case=forbidden-entry-range errno=-EACCES partial-launch=false runnable-published=false ok=true" "$LOG_FILE"
grep -Fq "qemu-initial-process-launch-smoke: error case=destroyed-input errno=-EINVAL partial-launch=false runnable-published=false ok=true" "$LOG_FILE"
grep -Fq "qemu-initial-process-launch-smoke: error case=activation-request errno=-ENOSYS partial-launch=false runnable-published=false ok=true" "$LOG_FILE"
grep -Fq "qemu-initial-process-launch-smoke: error case=stack-required-launch errno=-ENOSYS partial-launch=false runnable-published=false ok=true" "$LOG_FILE"
grep -Fq "qemu-initial-process-launch-smoke: error case=scheduler-publication-request errno=-ENOSYS partial-launch=false runnable-published=false ok=true" "$LOG_FILE"
grep -Fq "qemu-initial-process-launch-smoke: final participants=11 expected=11 errors=0 classification=qemu-initial-process-launch-smoke-complete" "$LOG_FILE"
grep -Fq "qemu-initial-process-launch-smoke: PASS" "$LOG_FILE"

mkdir -p "$EVIDENCE_DIR"
tr -d '\r' <"$LOG_FILE" >"$EVIDENCE_LOG"
cat "$EVIDENCE_LOG"
