#!/bin/sh
set -eu

cargo -Zjson-target-spec build "$@"

ELF_FILE="target/aarch64-talos-virt/debug/talos"
IMG_FILE="$ELF_FILE.img"
LOG_FILE="target/qemu-smoke.log"

rust-objcopy -O binary "$ELF_FILE" "$IMG_FILE"

qemu-system-aarch64 \
    -M virt \
    -cpu cortex-a76 \
    -m 256M \
    -nographic \
    -serial mon:stdio \
    -semihosting-config enable=on,target=native \
    -kernel "$IMG_FILE" >"$LOG_FILE" 2>&1

grep -q "Talos" "$LOG_FILE"
grep -q "talos: qemu smoke PASS" "$LOG_FILE"

cat "$LOG_FILE"
