#!/bin/sh
set -eu

TALOS_BOOT_SCENARIO=qemu_program_loader_smoke cargo -Zjson-target-spec build "$@"

ELF_FILE="target/aarch64-talos-virt/debug/talos"
IMG_FILE="$ELF_FILE.img"
LOG_FILE="target/qemu-program-loader-smoke.log"
EVIDENCE_DIR="tasks/evidence/2026-05-30-qemu-program-loader-smoke-core"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-program-loader-smoke.log"

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
grep -Fq "qemu-program-loader-smoke: start" "$LOG_FILE"
grep -Eq "qemu-program-loader-smoke: fixture name=phase8-program-loader-elf64-aarch64-v1 path=/bin/init digest-algorithm=stable-elf-manifest digest=0x[0-9a-f]+" "$LOG_FILE"
grep -Fq "qemu-program-loader-smoke: success format=elf64-aarch64-static-et-exec type=ET_EXEC machine=EM_AARCH64 phdrs=2 loadable=2 dynamic=false relocations=false ok=true" "$LOG_FILE"
grep -Eq "qemu-program-loader-smoke: segment index=0 kind=UserText flags=R-X file-bytes=0x[0-9a-f]+ mem-bytes=0x[0-9a-f]+ zero-fill=0x0 wx=false ok=true" "$LOG_FILE"
grep -Eq "qemu-program-loader-smoke: segment index=1 kind=UserData flags=RW- file-bytes=0x[0-9a-f]+ mem-bytes=0x[0-9a-f]+ zero-fill=0x[0-9a-f]+ wx=false ok=true" "$LOG_FILE"
grep -Eq "qemu-program-loader-smoke: entry va=0x[0-9a-f]+ in-user=true in-text=true aligned=true ok=true" "$LOG_FILE"
grep -Fq "qemu-program-loader-smoke: image-plan source=/bin/init output=image-plan-only process-created=false stack-built=false descriptors-installed=false ok=true" "$LOG_FILE"
grep -Fq "qemu-program-loader-smoke: error case=bad-magic errno=-ENOEXEC partial-install=false ok=true" "$LOG_FILE"
grep -Fq "qemu-program-loader-smoke: error case=dynamic-interpreter errno=-ENOTSUP partial-install=false ok=true" "$LOG_FILE"
grep -Fq "qemu-program-loader-smoke: error case=wx-segment errno=-EACCES partial-install=false ok=true" "$LOG_FILE"
grep -Fq "qemu-program-loader-smoke: error case=out-of-user-range errno=-EACCES partial-install=false ok=true" "$LOG_FILE"
grep -Fq "qemu-program-loader-smoke: error case=overlap errno=-EACCES partial-install=false ok=true" "$LOG_FILE"
grep -Fq "qemu-program-loader-smoke: error case=bad-entry errno=-ENOEXEC partial-install=false ok=true" "$LOG_FILE"
grep -Fq "qemu-program-loader-smoke: error case=file-range-overflow errno=-ENOEXEC partial-install=false ok=true" "$LOG_FILE"
grep -Fq "qemu-program-loader-smoke: final participants=8 expected=8 errors=0 classification=qemu-program-loader-smoke-complete" "$LOG_FILE"
grep -Fq "qemu-program-loader-smoke: PASS" "$LOG_FILE"

mkdir -p "$EVIDENCE_DIR"
tr -d '\r' <"$LOG_FILE" >"$EVIDENCE_LOG"
cat "$EVIDENCE_LOG"
