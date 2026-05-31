#!/bin/sh
set -eu

TALOS_BOOT_SCENARIO=qemu_live_translation_register_activation_smoke cargo -Zjson-target-spec build "$@"

ELF_FILE="target/aarch64-talos-virt/debug/talos"
IMG_FILE="$ELF_FILE.img"
LOG_FILE="target/qemu-live-translation-register-activation-smoke.log"
EVIDENCE_DIR="tasks/evidence/2026-05-31-qemu-live-translation-register-activation-smoke-core"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-live-translation-register-activation-smoke.log"

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
grep -Fq "qemu-live-translation-register-activation-smoke: start" "$LOG_FILE"
grep -Eq "qemu-live-translation-register-activation-smoke: fixture name=phase8-program-loader-elf64-aarch64-v1 path=/bin/init source-digest=0x[0-9a-f]+ install-boundary=phase8-process-install-plan-v1 address-space-boundary=phase8-process-address-space-model-v1 materialization-boundary=phase8-process-page-table-materialization-v1 launch-boundary=phase8-initial-process-launch-plan-v1 stack-boundary=phase8-initial-user-stack-plan-v1 activation-plan-boundary=phase8-live-address-space-activation-plan-v1 reachability-boundary=phase8-kernel-half-reachability-plan-v1 descriptor-image-boundary=phase8-kernel-half-descriptor-image-v1 installation-boundary=phase8-live-descriptor-image-installation-v1 activation-boundary=phase8-live-translation-register-activation-v1 activation-policy=model-ttbr0-ttbr1-activation-commit-below-live-registers-v1" "$LOG_FILE"
grep -Fq "qemu-live-translation-register-activation-smoke: success output=LiveTranslationRegisterActivation published=true copied-identities=true activation-boundary=phase8-live-translation-register-activation-v1 activation-policy=model-ttbr0-ttbr1-activation-commit-below-live-registers-v1 ok=true" "$LOG_FILE"
grep -Fq "qemu-live-translation-register-activation-smoke: input-state installation-published=true installation-destroyed=false below-live-registers=true descriptor-image-installed=false ttbr0-written=false ttbr1-written=false sctlr-mutated=false active-root-copied=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-translation-register-activation-smoke: ttbr-provenance ttbr0-root=materialized-process-root-provenance ttbr0-written=false ttbr1-root=descriptor-image-kernel-root-provenance ttbr1-written=false active-root-copied=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-translation-register-activation-smoke: compatibility tcr-state=compatibility-record-only mair-state=compatibility-record-only sctlr-state=mutation-blocked ok=true" "$LOG_FILE"
grep -Fq "qemu-live-translation-register-activation-smoke: blocked-states asid=blocked-no-asid-allocation tlb=blocked-no-live-tlbi barriers=planned-only-no-live-dsb-isb live-register-sequence=blocked-no-live-register-sequence ok=true" "$LOG_FILE"
grep -Fq "qemu-live-translation-register-activation-smoke: kernel-reachability vbar=true vectors=true active-stack=true kernel-text=true rodata=true data=true bss=true heap=true allocator=true uart-mmio-diagnostics=true scheduler-code-data=true runtime-console=true panic-fault-reporting=true ok=true" "$LOG_FILE"
grep -Fq "qemu-live-translation-register-activation-smoke: activation-state previous=installation-ready-activation-binding next=model-only-activation-commit-intent lower-el-eret=false scheduler-published=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-translation-register-activation-smoke: side-effects ttbr-mutated=false tcr-mutated=false mair-mutated=false sctlr-mutated=false active-root-copied=false asid-allocated=false tlb-mutated=false live-dsb-isb=false lower-el-eret=false scheduler-published=false process-table-mutated=false descriptor-table-published=false filesystem-mutated=false hardware-action=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-translation-register-activation-smoke: teardown phase=first activation-cleared=true installation-input-owned=true descriptor-input-owned=true activation-plan-owned=true materialized-root-owned=true live-state-mutated=false already-destroyed=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-translation-register-activation-smoke: teardown phase=second activation-cleared=false installation-input-owned=true descriptor-input-owned=true activation-plan-owned=true materialized-root-owned=true already-destroyed=true ok=true" "$LOG_FILE"
grep -Fq "qemu-live-translation-register-activation-smoke: error case=missing-input errno=-EINVAL partial-activation=false consumed=false live-state-mutated=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-translation-register-activation-smoke: error case=destroyed-input errno=-EINVAL partial-activation=false consumed=false live-state-mutated=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-translation-register-activation-smoke: error case=identity-mismatch errno=-EINVAL partial-activation=false consumed=false live-state-mutated=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-translation-register-activation-smoke: error case=lineage-mismatch errno=-ENOEXEC partial-activation=false consumed=false live-state-mutated=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-translation-register-activation-smoke: error case=stale-root-provenance errno=-EBUSY partial-activation=false consumed=false live-state-mutated=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-translation-register-activation-smoke: error case=already-consumed-installation errno=-EBUSY partial-activation=false consumed=true live-state-mutated=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-translation-register-activation-smoke: error case=forbidden-el0-kernel-access errno=-EACCES partial-activation=false consumed=false live-state-mutated=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-translation-register-activation-smoke: error case=diagnostic-reachability-loss errno=-EACCES partial-activation=false consumed=false live-state-mutated=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-translation-register-activation-smoke: error case=live-register-request errno=-ENOSYS partial-activation=false consumed=false live-state-mutated=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-translation-register-activation-smoke: error case=active-root-copy-request errno=-ENOSYS partial-activation=false consumed=false live-state-mutated=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-translation-register-activation-smoke: error case=lower-el-launch-request errno=-ENOSYS partial-activation=false consumed=false live-state-mutated=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-translation-register-activation-smoke: error case=scheduler-publication-request errno=-ENOSYS partial-activation=false consumed=false live-state-mutated=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-translation-register-activation-smoke: error case=filesystem-request errno=-ENOSYS partial-activation=false consumed=false live-state-mutated=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-translation-register-activation-smoke: error case=resource-exhaustion errno=-ENOMEM partial-activation=false consumed=false live-state-mutated=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-translation-register-activation-smoke: final participants=17 expected=17 errors=0 classification=qemu-live-translation-register-activation-smoke-complete" "$LOG_FILE"
grep -Fq "qemu-live-translation-register-activation-smoke: PASS" "$LOG_FILE"

mkdir -p "$EVIDENCE_DIR"
tr -d '\r' <"$LOG_FILE" >"$EVIDENCE_LOG"
