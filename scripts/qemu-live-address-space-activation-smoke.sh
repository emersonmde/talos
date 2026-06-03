#!/bin/sh
set -eu

TALOS_BOOT_SCENARIO=qemu_live_address_space_activation_smoke cargo -Zjson-target-spec build "$@"

ELF_FILE="target/aarch64-talos-virt/debug/talos"
IMG_FILE="$ELF_FILE.img"
LOG_FILE="target/qemu-live-address-space-activation-smoke.log"
EVIDENCE_DIR="tasks/evidence/2026-05-30-qemu-live-address-space-activation-smoke-core"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-live-address-space-activation-smoke.log"

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
grep -Fq "qemu-live-address-space-activation-smoke: start" "$LOG_FILE"
grep -Eq "qemu-live-address-space-activation-smoke: fixture name=phase8-program-loader-elf64-aarch64-v1 path=/bin/init source-digest=0x[0-9a-f]+ install-boundary=phase8-process-install-plan-v1 address-space-boundary=phase8-process-address-space-model-v1 materialization-boundary=phase8-process-page-table-materialization-v1 launch-boundary=phase8-initial-process-launch-plan-v1 stack-boundary=phase8-initial-user-stack-plan-v1 activation-boundary=phase8-live-address-space-activation-plan-v1 activation-policy=preflight-split-user-ttbr0-kernel-reachability-blocked-v1" "$LOG_FILE"
grep -Fq "qemu-live-address-space-activation-smoke: success output=LiveAddressSpaceActivationPlan published=true copied-identities=true activation-boundary=phase8-live-address-space-activation-plan-v1 activation-policy=preflight-split-user-ttbr0-kernel-reachability-blocked-v1 ok=true" "$LOG_FILE"
grep -Fq "qemu-live-address-space-activation-smoke: ttbr-provenance ttbr0-root=materialized-process-root-lease ttbr0-written=false ttbr1-policy=blocked-no-accepted-kernel-half-map ok=true" "$LOG_FILE"
grep -Fq "qemu-live-address-space-activation-smoke: compatibility tcr-state=compatibility-record-only mair-state=compatibility-record-only sctlr-state=mutation-blocked ok=true" "$LOG_FILE"
grep -Fq "qemu-live-address-space-activation-smoke: blocked-states asid=blocked-no-asid-allocation tlb=blocked-no-live-tlbi barriers=planned-only-no-live-dsb-isb live-register-sequence=blocked-no-live-register-sequence ok=true" "$LOG_FILE"
grep -Fq "qemu-live-address-space-activation-smoke: kernel-reachability vbar=true vectors=true active-stack=true kernel-text-data=true allocator=true uart-mmio-diagnostics=true scheduler-code-data=true panic-fault-reporting=true ok=true" "$LOG_FILE"
grep -Fq "qemu-live-address-space-activation-smoke: launch-binding previous=blocked-no-ttbr-activation next=model-only-activation-preflight-ready lower-el-eret=false scheduler-published=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-address-space-activation-smoke: side-effects ttbr-mutated=false tcr-mutated=false mair-mutated=false sctlr-mutated=false asid-allocated=false tlb-mutated=false live-dsb-isb=false lower-el-eret=false scheduler-published=false process-table-mutated=false descriptor-table-mutated=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-address-space-activation-smoke: teardown plan-local-released=true materialization-owned=true launch-owned=true stack-owned=true image-owned=true idempotent=true ok=true" "$LOG_FILE"
grep -Fq "qemu-live-address-space-activation-smoke: error case=identity-mismatch errno=-EINVAL partial-activation=false runnable-published=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-address-space-activation-smoke: error case=entry-stack-descriptor-disagreement errno=-ENOEXEC partial-activation=false runnable-published=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-address-space-activation-smoke: error case=forbidden-range errno=-EACCES partial-activation=false runnable-published=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-address-space-activation-smoke: error case=missing-kernel-reachability errno=-EINVAL partial-activation=false runnable-published=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-address-space-activation-smoke: error case=live-register-request errno=-ENOSYS partial-activation=false runnable-published=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-address-space-activation-smoke: error case=scheduler-publication-request errno=-ENOSYS partial-activation=false runnable-published=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-address-space-activation-smoke: error case=lower-el-launch-request errno=-ENOSYS partial-activation=false runnable-published=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-address-space-activation-smoke: error case=resource-exhaustion errno=-ENOMEM partial-activation=false runnable-published=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-address-space-activation-smoke: final participants=15 expected=15 errors=0 classification=qemu-live-address-space-activation-smoke-complete" "$LOG_FILE"
grep -Fq "qemu-live-address-space-activation-smoke: PASS" "$LOG_FILE"

mkdir -p "$EVIDENCE_DIR"
tr -d '\r' <"$LOG_FILE" >"$EVIDENCE_LOG"
cat "$EVIDENCE_LOG"
