#!/bin/sh
set -eu

LOG_FILE="target/qemu-program-loader-from-vfs-smoke.log"
EVIDENCE_DIR="tasks/evidence/2026-06-03-qemu-program-loader-from-vfs-file"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-program-loader-from-vfs-smoke.log"

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
. "$script_dir/qemu-nographic-smoke-lib.sh"

talos_qemu_prepare_image "qemu_program_loader_from_vfs_smoke" "debug" "" "$@"
talos_qemu_run_nographic "virt,gic-version=2,virtualization=on" "" "$LOG_FILE"

grep -q "boot-info: .* el=2 " "$LOG_FILE"
grep -Fq "qemu-program-loader-from-vfs-smoke: start" "$LOG_FILE"
grep -Eq "qemu-program-loader-from-vfs-smoke: fixture name=phase8-program-loader-elf64-aarch64-v1 path=/bin/init digest-algorithm=stable-elf-manifest digest=0x[0-9a-f]+" "$LOG_FILE"
grep -Fq "qemu-program-loader-from-vfs-smoke: vfs-source path=/bin/init object=regular-file read-boundary=kernel-file-object bytes=516 eof=true ok=true" "$LOG_FILE"
grep -Fq "qemu-program-loader-from-vfs-smoke: success format=elf64-aarch64-static-et-exec type=ET_EXEC machine=EM_AARCH64 phdrs=2 loadable=2 dynamic=false relocations=false ok=true" "$LOG_FILE"
grep -Eq "qemu-program-loader-from-vfs-smoke: segment index=0 kind=UserText flags=R-X file-bytes=0x[0-9a-f]+ mem-bytes=0x[0-9a-f]+ zero-fill=0x0 wx=false ok=true" "$LOG_FILE"
grep -Eq "qemu-program-loader-from-vfs-smoke: segment index=1 kind=UserData flags=RW- file-bytes=0x[0-9a-f]+ mem-bytes=0x[0-9a-f]+ zero-fill=0x[0-9a-f]+ wx=false ok=true" "$LOG_FILE"
grep -Eq "qemu-program-loader-from-vfs-smoke: entry va=0x[0-9a-f]+ in-user=true in-text=true aligned=true ok=true" "$LOG_FILE"
grep -Fq "qemu-program-loader-from-vfs-smoke: image-plan source=/bin/init output=image-plan-only process-created=false stack-built=false descriptors-installed=false ok=true" "$LOG_FILE"
grep -Fq "qemu-program-loader-from-vfs-smoke: error case=bad-magic errno=-ENOEXEC partial-install=false ok=true" "$LOG_FILE"
grep -Fq "qemu-program-loader-from-vfs-smoke: error case=dynamic-interpreter errno=-ENOTSUP partial-install=false ok=true" "$LOG_FILE"
grep -Fq "qemu-program-loader-from-vfs-smoke: error case=wx-segment errno=-EACCES partial-install=false ok=true" "$LOG_FILE"
grep -Fq "qemu-program-loader-from-vfs-smoke: error case=out-of-user-range errno=-EACCES partial-install=false ok=true" "$LOG_FILE"
grep -Fq "qemu-program-loader-from-vfs-smoke: error case=overlap errno=-EACCES partial-install=false ok=true" "$LOG_FILE"
grep -Fq "qemu-program-loader-from-vfs-smoke: error case=bad-entry errno=-ENOEXEC partial-install=false ok=true" "$LOG_FILE"
grep -Fq "qemu-program-loader-from-vfs-smoke: error case=file-range-overflow errno=-ENOEXEC partial-install=false ok=true" "$LOG_FILE"
grep -Fq "qemu-program-loader-from-vfs-smoke: final participants=8 expected=8 errors=0 classification=qemu-program-loader-from-vfs-smoke-complete" "$LOG_FILE"
grep -Fq "qemu-program-loader-from-vfs-smoke: PASS" "$LOG_FILE"

mkdir -p "$EVIDENCE_DIR"
tr -d '\r' <"$LOG_FILE" >"$EVIDENCE_LOG"
cat "$EVIDENCE_LOG"
