#!/bin/sh
set -eu

LOG_FILE="target/qemu-process-page-table-materialization-smoke.log"
EVIDENCE_DIR="tasks/evidence/2026-05-30-qemu-process-page-table-materialization-smoke-core"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-process-page-table-materialization-smoke.log"

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
. "$script_dir/qemu-nographic-smoke-lib.sh"

talos_qemu_prepare_image "qemu_process_page_table_materialization_smoke" "debug" "" "$@"
talos_qemu_run_nographic "virt,gic-version=2,virtualization=on" "" "$LOG_FILE"

grep -q "boot-info: .* el=2 " "$LOG_FILE"
grep -Fq "qemu-process-page-table-materialization-smoke: start" "$LOG_FILE"
grep -Eq "qemu-process-page-table-materialization-smoke: fixture name=phase8-program-loader-elf64-aarch64-v1 path=/bin/init source-digest=0x[0-9a-f]+ install-boundary=phase8-process-install-plan-v1 address-space-boundary=phase8-process-address-space-model-v1 materialization-boundary=phase8-process-page-table-materialization-v1" "$LOG_FILE"
grep -Eq "qemu-process-page-table-materialization-smoke: success output=ProcessPageTableMaterialization published=true id=0x[0-9a-f]+ root-pages=1 table-pages=[0-9]+ user-frames=[0-9]+ descriptors=[0-9]+ activation-blocked=true kernel-mapping-policy=activation-blocked-no-kernel-half ok=true" "$LOG_FILE"
grep -Eq "qemu-process-page-table-materialization-smoke: frame index=0 kind=UserText virtual-page=0x[0-9a-f]+ physical-frame=0x[0-9a-f]+ copy-bytes=0x[0-9a-f]+ zero-bytes=0x[0-9a-f]+ zero-before-copy=true source-page=0 scrub-required=true ok=true" "$LOG_FILE"
grep -Eq "qemu-process-page-table-materialization-smoke: frame index=1 kind=UserData virtual-page=0x[0-9a-f]+ physical-frame=0x[0-9a-f]+ copy-bytes=0x[0-9a-f]+ zero-bytes=0x[0-9a-f]+ zero-before-copy=true source-page=1 scrub-required=true ok=true" "$LOG_FILE"
grep -Fq "qemu-process-page-table-materialization-smoke: descriptor index=0 kind=UserText flags=R-X ap=EL0_RO pxn=true uxn=false attr=normal-inner-shareable af=true wx=false ok=true" "$LOG_FILE"
grep -Fq "qemu-process-page-table-materialization-smoke: descriptor index=1 kind=UserData flags=RW- ap=EL0_RW pxn=true uxn=true attr=normal-inner-shareable af=true wx=false ok=true" "$LOG_FILE"
grep -Eq "qemu-process-page-table-materialization-smoke: side-effects root-pages-leased=1 table-pages-leased=[0-9]+ user-frames-leased=[0-9]+ descriptors-installed=[0-9]+ copied-bytes=0x[0-9a-f]+ zeroed-bytes=0x[0-9a-f]+ ttbr-mutated=false tlb-mutated=false scheduler-published=false lower-el-frame=false runnable=false ok=true" "$LOG_FILE"
grep -Eq "qemu-process-page-table-materialization-smoke: teardown phase=first descriptors-cleared=[0-9]+ table-pages-released=[0-9]+ user-frames-released=[0-9]+ root-released=true already-destroyed=false ok=true" "$LOG_FILE"
grep -Fq "qemu-process-page-table-materialization-smoke: teardown phase=second descriptors-cleared=0 table-pages-released=0 user-frames-released=0 root-released=false already-destroyed=true ok=true" "$LOG_FILE"
grep -Fq "qemu-process-page-table-materialization-smoke: error case=bad-address-space errno=-EINVAL partial-materialization=false leaked-leases=false ok=true" "$LOG_FILE"
grep -Fq "qemu-process-page-table-materialization-smoke: error case=forbidden-range errno=-EACCES partial-materialization=false leaked-leases=false ok=true" "$LOG_FILE"
grep -Fq "qemu-process-page-table-materialization-smoke: error case=permission-widening errno=-EACCES partial-materialization=false leaked-leases=false ok=true" "$LOG_FILE"
grep -Fq "qemu-process-page-table-materialization-smoke: error case=resource-exhaustion errno=-ENOMEM partial-materialization=false leaked-leases=false ok=true" "$LOG_FILE"
grep -Fq "qemu-process-page-table-materialization-smoke: error case=unsupported-topology errno=-ENOTSUP partial-materialization=false leaked-leases=false ok=true" "$LOG_FILE"
grep -Fq "qemu-process-page-table-materialization-smoke: error case=copy-zero-mismatch errno=-EINVAL partial-materialization=false leaked-leases=false ok=true" "$LOG_FILE"
grep -Fq "qemu-process-page-table-materialization-smoke: error case=activation-request errno=-ENOSYS partial-materialization=false leaked-leases=false ok=true" "$LOG_FILE"
grep -Fq "qemu-process-page-table-materialization-smoke: final participants=12 expected=12 errors=0 classification=qemu-process-page-table-materialization-smoke-complete" "$LOG_FILE"
grep -Fq "qemu-process-page-table-materialization-smoke: PASS" "$LOG_FILE"

mkdir -p "$EVIDENCE_DIR"
tr -d '\r' <"$LOG_FILE" >"$EVIDENCE_LOG"
cat "$EVIDENCE_LOG"
