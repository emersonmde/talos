#!/bin/sh
set -eu

LOG_FILE="target/qemu-process-address-space-smoke.log"
EVIDENCE_DIR="tasks/evidence/2026-05-30-qemu-process-address-space-smoke-core"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-process-address-space-smoke.log"

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
. "$script_dir/qemu-nographic-smoke-lib.sh"

talos_qemu_prepare_image "qemu_process_address_space_smoke" "debug" "" "$@"
talos_qemu_run_nographic "virt,gic-version=2,virtualization=on" "" "$LOG_FILE"

grep -q "boot-info: .* el=2 " "$LOG_FILE"
grep -Fq "qemu-process-address-space-smoke: start" "$LOG_FILE"
grep -Eq "qemu-process-address-space-smoke: fixture name=phase8-program-loader-elf64-aarch64-v1 path=/bin/init source-digest=0x[0-9a-f]+ install-boundary=phase8-process-install-plan-v1 address-space-boundary=phase8-process-address-space-model-v1" "$LOG_FILE"
grep -Eq "qemu-process-address-space-smoke: success output=ProcessAddressSpace published=true id=0x[0-9a-f]+ owner=0x[0-9a-f]+ root-token=0x[0-9a-f]+ table-leases=[0-9]+ user-frame-leases=[0-9]+ mappings=[0-9]+ ok=true" "$LOG_FILE"
grep -Eq "qemu-process-address-space-smoke: mapping index=0 kind=UserText flags=R-X copy-bytes=0x[0-9a-f]+ zero-bytes=0x[0-9a-f]+ zero-before-copy=true source-page=0 permission-widened=false ok=true" "$LOG_FILE"
grep -Eq "qemu-process-address-space-smoke: mapping index=1 kind=UserData flags=RW- copy-bytes=0x[0-9a-f]+ zero-bytes=0x[0-9a-f]+ zero-before-copy=true source-page=1 permission-widened=false ok=true" "$LOG_FILE"
grep -Eq "qemu-process-address-space-smoke: side-effects root-leased=true table-leases=[0-9]+ user-frame-leases=[0-9]+ mappings-installed=[0-9]+ copied-bytes=0x[0-9a-f]+ zeroed-bytes=0x[0-9a-f]+ scheduler-owner=false descriptors-mutated=false lower-el-frame=false runnable=false ok=true" "$LOG_FILE"
grep -Eq "qemu-process-address-space-smoke: teardown phase=first mappings-released=[0-9]+ user-frame-releases=[0-9]+ table-lease-releases=[0-9]+ root-released=true already-destroyed=false ok=true" "$LOG_FILE"
grep -Fq "qemu-process-address-space-smoke: teardown phase=second mappings-released=0 user-frame-releases=0 table-lease-releases=0 root-released=false already-destroyed=true ok=true" "$LOG_FILE"
grep -Fq "qemu-process-address-space-smoke: error case=bad-install-plan errno=-EINVAL partial-install=false leaked-leases=false ok=true" "$LOG_FILE"
grep -Fq "qemu-process-address-space-smoke: error case=null-guard-or-kernel-split errno=-EACCES partial-install=false leaked-leases=false ok=true" "$LOG_FILE"
grep -Fq "qemu-process-address-space-smoke: error case=overlap errno=-EACCES partial-install=false leaked-leases=false ok=true" "$LOG_FILE"
grep -Fq "qemu-process-address-space-smoke: error case=permission-widening errno=-EACCES partial-install=false leaked-leases=false ok=true" "$LOG_FILE"
grep -Fq "qemu-process-address-space-smoke: error case=lease-exhaustion errno=-ENOMEM partial-install=false leaked-leases=false ok=true" "$LOG_FILE"
grep -Fq "qemu-process-address-space-smoke: error case=copy-zero-model-failure errno=-EINVAL partial-install=false leaked-leases=false ok=true" "$LOG_FILE"
grep -Fq "qemu-process-address-space-smoke: final participants=8 expected=8 errors=0 classification=qemu-process-address-space-smoke-complete" "$LOG_FILE"
grep -Fq "qemu-process-address-space-smoke: PASS" "$LOG_FILE"

mkdir -p "$EVIDENCE_DIR"
tr -d '\r' <"$LOG_FILE" >"$EVIDENCE_LOG"
cat "$EVIDENCE_LOG"
