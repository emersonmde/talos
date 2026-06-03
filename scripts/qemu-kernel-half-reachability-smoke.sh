#!/bin/sh
set -eu

TALOS_BOOT_SCENARIO=qemu_kernel_half_reachability_smoke cargo -Zjson-target-spec build "$@"

ELF_FILE="target/aarch64-talos-virt/debug/talos"
IMG_FILE="$ELF_FILE.img"
LOG_FILE="target/qemu-kernel-half-reachability-smoke.log"
EVIDENCE_DIR="tasks/evidence/2026-05-31-qemu-kernel-half-reachability-smoke-core"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-kernel-half-reachability-smoke.log"

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
grep -Fq "qemu-kernel-half-reachability-smoke: start" "$LOG_FILE"
grep -Eq "qemu-kernel-half-reachability-smoke: fixture name=phase8-program-loader-elf64-aarch64-v1 path=/bin/init source-digest=0x[0-9a-f]+ install-boundary=phase8-process-install-plan-v1 address-space-boundary=phase8-process-address-space-model-v1 materialization-boundary=phase8-process-page-table-materialization-v1 launch-boundary=phase8-initial-process-launch-plan-v1 stack-boundary=phase8-initial-user-stack-plan-v1 activation-boundary=phase8-live-address-space-activation-plan-v1 kernel-half-boundary=phase8-kernel-half-reachability-plan-v1 kernel-half-policy=preflight-ttbr1-shared-kernel-root-reachability-v1" "$LOG_FILE"
grep -Fq "qemu-kernel-half-reachability-smoke: success output=KernelHalfReachabilityPlan published=true copied-identities=true kernel-half-boundary=phase8-kernel-half-reachability-plan-v1 kernel-half-policy=preflight-ttbr1-shared-kernel-root-reachability-v1 ok=true" "$LOG_FILE"
grep -Fq "qemu-kernel-half-reachability-smoke: root-policy ttbr0-root=materialized-process-root-lease ttbr0-written=false ttbr1-policy=shared-privileged-kernel-root ttbr1-written=false descriptor-image=blocked-no-kernel-half-descriptor-image ok=true" "$LOG_FILE"
grep -Fq "qemu-kernel-half-reachability-smoke: reachability kernel-text=true rodata=true data=true bss=true vectors=true active-stack=true heap=true page-frames=true uart-mmio-diagnostics=true scheduler-code-data=true panic-fault-reporting=true ok=true" "$LOG_FILE"
grep -Fq "qemu-kernel-half-reachability-smoke: permissions text-exec=privileged-only data-exec=false device-normal-memory=false el0-kernel-access=false ok=true" "$LOG_FILE"
grep -Fq "qemu-kernel-half-reachability-smoke: compatibility tcr-state=split-compatibility-record-only mair-state=normal-device-compatibility-record-only sctlr-state=mutation-blocked ok=true" "$LOG_FILE"
grep -Fq "qemu-kernel-half-reachability-smoke: blocked-states asid=blocked-no-asid-allocation tlb=blocked-no-live-tlbi barriers=planned-only-no-live-dsb-isb live-register-sequence=blocked-no-live-register-sequence ok=true" "$LOG_FILE"
grep -Fq "qemu-kernel-half-reachability-smoke: side-effects ttbr-mutated=false tcr-mutated=false mair-mutated=false sctlr-mutated=false descriptor-image-installed=false asid-allocated=false tlb-mutated=false live-dsb-isb=false lower-el-eret=false scheduler-published=false process-table-mutated=false descriptor-table-mutated=false ok=true" "$LOG_FILE"
grep -Fq "qemu-kernel-half-reachability-smoke: teardown plan-local-released=true input-records-owned=true descriptor-image-installed=false idempotent=true ok=true" "$LOG_FILE"
grep -Fq "qemu-kernel-half-reachability-smoke: error case=identity-mismatch errno=-EINVAL partial-plan=false ok=true" "$LOG_FILE"
grep -Fq "qemu-kernel-half-reachability-smoke: error case=missing-kernel-range errno=-EACCES partial-plan=false ok=true" "$LOG_FILE"
grep -Fq "qemu-kernel-half-reachability-smoke: error case=missing-diagnostic-fault-reporting errno=-ENOSYS partial-plan=false ok=true" "$LOG_FILE"
grep -Fq "qemu-kernel-half-reachability-smoke: error case=forbidden-el0-access errno=-EACCES partial-plan=false ok=true" "$LOG_FILE"
grep -Fq "qemu-kernel-half-reachability-smoke: error case=bad-device-attribute-intent errno=-EACCES partial-plan=false ok=true" "$LOG_FILE"
grep -Fq "qemu-kernel-half-reachability-smoke: error case=live-register-request errno=-ENOSYS partial-plan=false ok=true" "$LOG_FILE"
grep -Fq "qemu-kernel-half-reachability-smoke: error case=descriptor-image-request errno=-ENOSYS partial-plan=false ok=true" "$LOG_FILE"
grep -Fq "qemu-kernel-half-reachability-smoke: error case=scheduler-publication-request errno=-ENOSYS partial-plan=false ok=true" "$LOG_FILE"
grep -Fq "qemu-kernel-half-reachability-smoke: error case=lower-el-launch-request errno=-ENOSYS partial-plan=false ok=true" "$LOG_FILE"
grep -Fq "qemu-kernel-half-reachability-smoke: error case=resource-exhaustion errno=-ENOMEM partial-plan=false ok=true" "$LOG_FILE"
grep -Fq "qemu-kernel-half-reachability-smoke: final participants=16 expected=16 errors=0 classification=qemu-kernel-half-reachability-smoke-complete" "$LOG_FILE"
grep -Fq "qemu-kernel-half-reachability-smoke: PASS" "$LOG_FILE"

mkdir -p "$EVIDENCE_DIR"
tr -d '\r' <"$LOG_FILE" >"$EVIDENCE_LOG"
cat "$EVIDENCE_LOG"
