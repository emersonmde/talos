#!/bin/sh
set -eu

TALOS_BOOT_SCENARIO=qemu_readonly_initramfs_vfs_smoke cargo -Zjson-target-spec build "$@"

ELF_FILE="target/aarch64-talos-virt/debug/talos"
IMG_FILE="$ELF_FILE.img"
LOG_FILE="target/qemu-readonly-initramfs-vfs-smoke.log"
EVIDENCE_DIR="tasks/evidence/2026-05-30-qemu-readonly-initramfs-vfs-smoke-core"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-readonly-initramfs-vfs-smoke.log"

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
grep -Fq "qemu-readonly-initramfs-vfs-smoke: start" "$LOG_FILE"
grep -Eq "qemu-readonly-initramfs-vfs-smoke: fixture name=phase8-readonly-initramfs-vfs-v1 digest-algorithm=stable-manifest digest=0x[0-9a-f]+" "$LOG_FILE"
grep -Fq "qemu-readonly-initramfs-vfs-smoke: lookup path=/ kind=directory entries=4 ok=true" "$LOG_FILE"
grep -Fq "qemu-readonly-initramfs-vfs-smoke: lookup path=/etc/banner.txt kind=regular length=24 ok=true" "$LOG_FILE"
grep -Fq "qemu-readonly-initramfs-vfs-smoke: read path=/etc/banner.txt offset-before=0 request=64 result=24 offset-after=24 data=\"Talos initramfs fixture\\n\" ok=true" "$LOG_FILE"
grep -Fq "qemu-readonly-initramfs-vfs-smoke: read path=/etc/banner.txt offset-before=24 request=64 result=0 offset-after=24 eof=true ok=true" "$LOG_FILE"
grep -Fq "qemu-readonly-initramfs-vfs-smoke: read path=/empty offset-before=0 request=64 result=0 offset-after=0 eof=true ok=true" "$LOG_FILE"
grep -Fq "qemu-readonly-initramfs-vfs-smoke: lookup path=/dir/nested.txt kind=regular length=15 ok=true" "$LOG_FILE"
grep -Fq "qemu-readonly-initramfs-vfs-smoke: error case=missing path=/missing errno=-ENOENT ok=true" "$LOG_FILE"
grep -Fq "qemu-readonly-initramfs-vfs-smoke: error case=not-directory path=/etc/banner.txt/child errno=-ENOTDIR ok=true" "$LOG_FILE"
grep -Fq "qemu-readonly-initramfs-vfs-smoke: error case=is-directory path=/etc errno=-EISDIR ok=true" "$LOG_FILE"
grep -Fq "qemu-readonly-initramfs-vfs-smoke: error case=name-too-long errno=-ENAMETOOLONG ok=true" "$LOG_FILE"
grep -Fq "qemu-readonly-initramfs-vfs-smoke: error case=bad-descriptor errno=-EBADF offset-unchanged=true ok=true" "$LOG_FILE"
grep -Fq "qemu-readonly-initramfs-vfs-smoke: error case=user-fault errno=-EFAULT offset-unchanged=true ok=true" "$LOG_FILE"
grep -Fq "qemu-readonly-initramfs-vfs-smoke: error case=invalid-input errno=-EINVAL offset-unchanged=true ok=true" "$LOG_FILE"
grep -Fq "qemu-readonly-initramfs-vfs-smoke: error case=unsupported-operation errno=-ENOTSUP ok=true" "$LOG_FILE"
grep -Fq "qemu-readonly-initramfs-vfs-smoke: final participants=8 expected=8 errors=0 classification=qemu-readonly-initramfs-vfs-smoke-complete" "$LOG_FILE"
grep -Fq "qemu-readonly-initramfs-vfs-smoke: PASS" "$LOG_FILE"

mkdir -p "$EVIDENCE_DIR"
tr -d '\r' <"$LOG_FILE" >"$EVIDENCE_LOG"
cat "$EVIDENCE_LOG"
