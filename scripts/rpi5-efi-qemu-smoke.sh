#!/bin/sh
set -eu

QEMU="${QEMU:-qemu-system-aarch64}"
FIRMWARE="${AAVMF_CODE:-/usr/share/qemu-efi-aarch64/QEMU_EFI.fd}"
FAT_IMAGE="${1:-target/talos-rpi5-efi-diagnostic.img}"
LOG="${2:-target/rpi5-efi-qemu-smoke.log}"

if [ ! -f "$FIRMWARE" ]; then
    echo "AArch64 UEFI firmware not found: $FIRMWARE" >&2
    exit 1
fi

./scripts/rpi5-efi-diagnostic-fat.sh "$FAT_IMAGE" >/dev/null

status=0
timeout 25 "$QEMU" \
    -M virt \
    -cpu cortex-a72 \
    -m 512M \
    -nographic \
    -bios "$FIRMWARE" \
    -drive if=virtio,format=raw,file="$FAT_IMAGE" \
    > "$LOG" 2>&1 || status=$?

if ! grep -q "Talos EFI first-light PASS" "$LOG"; then
    cat "$LOG" >&2
    exit 1
fi

# UEFI falls back to its UI after the diagnostic returns, so timeout is the
# expected process exit. The PASS marker is the validation signal.
printf 'efi_qemu_smoke=pass\n'
printf 'qemu_status=%s\n' "$status"
printf 'log=%s\n' "$LOG"
