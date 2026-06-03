#!/bin/sh
set -eu

TALOS_BOOT_SCENARIO=qemu_kernel_half_descriptor_image_smoke cargo -Zjson-target-spec build "$@"

ELF_FILE="target/aarch64-talos-virt/debug/talos"
IMG_FILE="$ELF_FILE.img"
LOG_FILE="target/qemu-kernel-half-descriptor-image-smoke.log"
EVIDENCE_DIR="tasks/evidence/2026-05-31-qemu-kernel-half-descriptor-image-smoke-core"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-kernel-half-descriptor-image-smoke.log"

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
grep -Fq "qemu-kernel-half-descriptor-image-smoke: start" "$LOG_FILE"
grep -Eq "qemu-kernel-half-descriptor-image-smoke: fixture name=phase8-program-loader-elf64-aarch64-v1 path=/bin/init source-digest=0x[0-9a-f]+ install-boundary=phase8-process-install-plan-v1 address-space-boundary=phase8-process-address-space-model-v1 materialization-boundary=phase8-process-page-table-materialization-v1 launch-boundary=phase8-initial-process-launch-plan-v1 stack-boundary=phase8-initial-user-stack-plan-v1 activation-boundary=phase8-live-address-space-activation-plan-v1 reachability-boundary=phase8-kernel-half-reachability-plan-v1 descriptor-image-boundary=phase8-kernel-half-descriptor-image-v1 descriptor-image-policy=ttbr1-shared-privileged-kernel-root-descriptor-image-v1" "$LOG_FILE"
grep -Fq "qemu-kernel-half-descriptor-image-smoke: success output=KernelHalfDescriptorImage published=true installed=false copied-identities=true descriptor-image-boundary=phase8-kernel-half-descriptor-image-v1 descriptor-image-policy=ttbr1-shared-privileged-kernel-root-descriptor-image-v1 ok=true" "$LOG_FILE"
grep -Fq "qemu-kernel-half-descriptor-image-smoke: root-policy ttbr0-root=materialized-process-root-provenance ttbr0-written=false ttbr1-root=owned-kernel-root-image ttbr1-written=false descriptor-image-installed=false ok=true" "$LOG_FILE"
grep -Fq "qemu-kernel-half-descriptor-image-smoke: coverage kernel-text=true rodata=true data=true bss=true vectors=true active-stack=true heap=true page-frames=true uart-mmio-diagnostics=true scheduler-code-data=true runtime-console=true panic-fault-reporting=true ok=true" "$LOG_FILE"
grep -Fq "qemu-kernel-half-descriptor-image-smoke: permissions text-exec=privileged-only rodata-write=false data-exec=false device-normal-memory=false el0-kernel-access=false wx-normal-memory=false ok=true" "$LOG_FILE"
grep -Fq "qemu-kernel-half-descriptor-image-smoke: attributes normal-memory=inner-shareable device-memory=device-nGnRE af=true user-access=denied exact-coverage=true ok=true" "$LOG_FILE"
grep -Fq "qemu-kernel-half-descriptor-image-smoke: ownership root-lease=model-owned table-leases=model-owned live-table-borrowed=false input-records-owned=true rollback-ready=true ok=true" "$LOG_FILE"
grep -Fq "qemu-kernel-half-descriptor-image-smoke: compatibility tcr-state=split-compatibility-record-only mair-state=normal-device-compatibility-record-only sctlr-state=mutation-blocked ok=true" "$LOG_FILE"
grep -Fq "qemu-kernel-half-descriptor-image-smoke: blocked-states asid=blocked-no-asid-allocation tlb=blocked-no-live-tlbi barriers=planned-only-no-live-dsb-isb live-register-sequence=blocked-no-live-register-sequence lower-el-eret=false scheduler-publication=false ok=true" "$LOG_FILE"
grep -Fq "qemu-kernel-half-descriptor-image-smoke: side-effects ttbr-mutated=false tcr-mutated=false mair-mutated=false sctlr-mutated=false descriptor-image-installed=false asid-allocated=false tlb-mutated=false live-dsb-isb=false lower-el-eret=false scheduler-published=false process-table-mutated=false descriptor-table-mutated=false ok=true" "$LOG_FILE"
grep -Fq "qemu-kernel-half-descriptor-image-smoke: teardown phase=first descriptors-cleared=true root-released=true tables-released=true published=false input-records-owned=true already-destroyed=false ok=true" "$LOG_FILE"
grep -Fq "qemu-kernel-half-descriptor-image-smoke: teardown phase=second descriptors-cleared=false root-released=false tables-released=false published=false already-destroyed=true ok=true" "$LOG_FILE"
grep -Fq "qemu-kernel-half-descriptor-image-smoke: error case=bad-reachability-plan errno=-EINVAL partial-image=false leaked-leases=false ok=true" "$LOG_FILE"
grep -Fq "qemu-kernel-half-descriptor-image-smoke: error case=lineage-mismatch errno=-EINVAL partial-image=false leaked-leases=false ok=true" "$LOG_FILE"
grep -Fq "qemu-kernel-half-descriptor-image-smoke: error case=missing-kernel-coverage errno=-EINVAL partial-image=false leaked-leases=false ok=true" "$LOG_FILE"
grep -Fq "qemu-kernel-half-descriptor-image-smoke: error case=forbidden-el0-access errno=-EACCES partial-image=false leaked-leases=false ok=true" "$LOG_FILE"
grep -Fq "qemu-kernel-half-descriptor-image-smoke: error case=writable-text errno=-EACCES partial-image=false leaked-leases=false ok=true" "$LOG_FILE"
grep -Fq "qemu-kernel-half-descriptor-image-smoke: error case=executable-data errno=-EACCES partial-image=false leaked-leases=false ok=true" "$LOG_FILE"
grep -Fq "qemu-kernel-half-descriptor-image-smoke: error case=bad-device-attribute-intent errno=-EACCES partial-image=false leaked-leases=false ok=true" "$LOG_FILE"
grep -Fq "qemu-kernel-half-descriptor-image-smoke: error case=overlapping-range errno=-EINVAL partial-image=false leaked-leases=false ok=true" "$LOG_FILE"
grep -Fq "qemu-kernel-half-descriptor-image-smoke: error case=resource-exhaustion errno=-ENOMEM partial-image=false leaked-leases=false ok=true" "$LOG_FILE"
grep -Fq "qemu-kernel-half-descriptor-image-smoke: error case=unsupported-topology errno=-ENOTSUP partial-image=false leaked-leases=false ok=true" "$LOG_FILE"
grep -Fq "qemu-kernel-half-descriptor-image-smoke: error case=live-activation-request errno=-ENOSYS partial-image=false leaked-leases=false ok=true" "$LOG_FILE"
grep -Fq "qemu-kernel-half-descriptor-image-smoke: final participants=17 expected=17 errors=0 classification=qemu-kernel-half-descriptor-image-smoke-complete" "$LOG_FILE"
grep -Fq "qemu-kernel-half-descriptor-image-smoke: PASS" "$LOG_FILE"

mkdir -p "$EVIDENCE_DIR"
tr -d '\r' <"$LOG_FILE" >"$EVIDENCE_LOG"
cat "$EVIDENCE_LOG"
