#!/bin/sh
set -eu

TALOS_BOOT_SCENARIO=qemu_live_descriptor_image_installation_smoke cargo -Zjson-target-spec build "$@"

ELF_FILE="target/aarch64-talos-virt/debug/talos"
IMG_FILE="$ELF_FILE.img"
LOG_FILE="target/qemu-live-descriptor-image-installation-smoke.log"
EVIDENCE_DIR="tasks/evidence/2026-05-31-qemu-live-descriptor-image-installation-smoke-core"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-live-descriptor-image-installation-smoke.log"

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
grep -Fq "qemu-live-descriptor-image-installation-smoke: start" "$LOG_FILE"
grep -Eq "qemu-live-descriptor-image-installation-smoke: fixture name=phase8-program-loader-elf64-aarch64-v1 path=/bin/init source-digest=0x[0-9a-f]+ install-boundary=phase8-process-install-plan-v1 address-space-boundary=phase8-process-address-space-model-v1 materialization-boundary=phase8-process-page-table-materialization-v1 launch-boundary=phase8-initial-process-launch-plan-v1 stack-boundary=phase8-initial-user-stack-plan-v1 activation-boundary=phase8-live-address-space-activation-plan-v1 reachability-boundary=phase8-kernel-half-reachability-plan-v1 descriptor-image-boundary=phase8-kernel-half-descriptor-image-v1 installation-boundary=phase8-live-descriptor-image-installation-v1 installation-policy=model-installed-ttbr1-descriptor-image-below-live-registers-v1" "$LOG_FILE"
grep -Fq "qemu-live-descriptor-image-installation-smoke: success output=KernelHalfDescriptorImageInstallation published=true copied-identities=true installation-boundary=phase8-live-descriptor-image-installation-v1 installation-policy=model-installed-ttbr1-descriptor-image-below-live-registers-v1 ok=true" "$LOG_FILE"
grep -Fq "qemu-live-descriptor-image-installation-smoke: input-state descriptor-published=true descriptor-installed=false descriptor-image-installed=false ttbr1-written=false activation-published=true activation-model-only=true ok=true" "$LOG_FILE"
grep -Fq "qemu-live-descriptor-image-installation-smoke: ttbr-provenance ttbr0-root=materialized-process-root-provenance ttbr0-written=false ttbr1-root=descriptor-image-kernel-root-provenance ttbr1-written=false active-root-copied=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-descriptor-image-installation-smoke: coverage kernel-text=true rodata=true data=true bss=true vectors=true active-stack=true heap=true page-frames=true uart-mmio-diagnostics=true scheduler-code-data=true runtime-console=true panic-fault-reporting=true ok=true" "$LOG_FILE"
grep -Fq "qemu-live-descriptor-image-installation-smoke: permissions text-exec=privileged-only rodata-write=false data-exec=false device-normal-memory=false el0-kernel-access=false wx-normal-memory=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-descriptor-image-installation-smoke: installation-state previous=non-installed-descriptor-image next=installation-ready-activation-binding live-register-state=blocked-no-live-register-sequence ok=true" "$LOG_FILE"
grep -Fq "qemu-live-descriptor-image-installation-smoke: compatibility tcr-state=compatibility-record-only mair-state=compatibility-record-only sctlr-state=mutation-blocked ok=true" "$LOG_FILE"
grep -Fq "qemu-live-descriptor-image-installation-smoke: blocked-states asid=blocked-no-asid-allocation tlb=blocked-no-live-tlbi barriers=planned-only-no-live-dsb-isb lower-el-eret=false scheduler-publication=false filesystem-syscalls=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-descriptor-image-installation-smoke: side-effects ttbr-mutated=false tcr-mutated=false mair-mutated=false sctlr-mutated=false active-root-copied=false descriptor-table-published=false asid-allocated=false tlb-mutated=false live-dsb-isb=false lower-el-eret=false scheduler-published=false process-table-mutated=false filesystem-mutated=false hardware-action=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-descriptor-image-installation-smoke: teardown phase=first installation-cleared=true descriptor-input-owned=true activation-input-owned=true live-state-mutated=false already-destroyed=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-descriptor-image-installation-smoke: teardown phase=second installation-cleared=false descriptor-input-owned=true activation-input-owned=true already-destroyed=true ok=true" "$LOG_FILE"
grep -Fq "qemu-live-descriptor-image-installation-smoke: error case=missing-input errno=-EINVAL partial-installation=false descriptor-installed=false live-state-mutated=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-descriptor-image-installation-smoke: error case=destroyed-input errno=-EINVAL partial-installation=false descriptor-installed=false live-state-mutated=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-descriptor-image-installation-smoke: error case=identity-mismatch errno=-EINVAL partial-installation=false descriptor-installed=false live-state-mutated=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-descriptor-image-installation-smoke: error case=lineage-mismatch errno=-ENOEXEC partial-installation=false descriptor-installed=false live-state-mutated=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-descriptor-image-installation-smoke: error case=already-installed-input errno=-EBUSY partial-installation=false descriptor-installed=true live-state-mutated=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-descriptor-image-installation-smoke: error case=forbidden-el0-access errno=-EACCES partial-installation=false descriptor-installed=false live-state-mutated=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-descriptor-image-installation-smoke: error case=diagnostic-reachability-loss errno=-EACCES partial-installation=false descriptor-installed=false live-state-mutated=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-descriptor-image-installation-smoke: error case=resource-exhaustion errno=-ENOMEM partial-installation=false descriptor-installed=false live-state-mutated=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-descriptor-image-installation-smoke: error case=live-register-request errno=-ENOSYS partial-installation=false descriptor-installed=false live-state-mutated=false ok=true" "$LOG_FILE"
grep -Fq "qemu-live-descriptor-image-installation-smoke: final participants=15 expected=15 errors=0 classification=qemu-live-descriptor-image-installation-smoke-complete" "$LOG_FILE"
grep -Fq "qemu-live-descriptor-image-installation-smoke: PASS" "$LOG_FILE"

mkdir -p "$EVIDENCE_DIR"
tr -d '\r' <"$LOG_FILE" >"$EVIDENCE_LOG"
cat "$EVIDENCE_LOG"
