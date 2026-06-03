#!/usr/bin/env bash
set -euo pipefail

TALOS_BOOT_SCENARIO=qemu_polling_tty_rx cargo -Zjson-target-spec build "$@"

ELF_FILE="target/aarch64-talos-virt/debug/talos"
IMG_FILE="$ELF_FILE.tty-rx.img"
LOG_FILE="target/qemu-tty-rx-diagnostic.log"
QEMU_LOG_FILE="target/qemu-tty-rx-diagnostic.qemu.log"
PORT="${TALOS_QEMU_TTY_RX_PORT:-54322}"

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
. "$script_dir/objcopy-tool.sh"
. "$script_dir/qemu-tool.sh"

"$objcopy_tool" -O binary "$ELF_FILE" "$IMG_FILE"
: >"$LOG_FILE"
: >"$QEMU_LOG_FILE"

"$qemu_tool" \
    -M virt,gic-version=2,virtualization=on \
    -cpu cortex-a76 \
    -m 256M \
    -display none \
    -monitor none \
    -chardev socket,id=serial0,host=127.0.0.1,port="$PORT",server=on,wait=on \
    -serial chardev:serial0 \
    -semihosting-config enable=on,target=native \
    -kernel "$IMG_FILE" >"$QEMU_LOG_FILE" 2>&1 &
qemu_pid=$!

cleanup() {
    if kill -0 "$qemu_pid" 2>/dev/null; then
        kill "$qemu_pid" 2>/dev/null || true
    fi
}
trap cleanup EXIT

connected=0
for _ in $(seq 1 100); do
    if { exec 3<>"/dev/tcp/127.0.0.1/$PORT"; } 2>/dev/null; then
        connected=1
        break
    fi
    sleep 0.05
done

if [ "$connected" -ne 1 ]; then
    echo "failed to connect to QEMU serial socket on port $PORT" >&2
    exit 1
fi

sent=0
while kill -0 "$qemu_pid" 2>/dev/null; do
    if IFS= read -r -t 0.2 line <&3; then
        printf '%s\n' "$line" >>"$LOG_FILE"
        case "$line" in
            *"qemu-tty-rx-diagnostic: ready"*)
                if [ "$sent" -eq 0 ]; then
                    printf 'abX\bcY\177d\003efghi\r' >&3
                    sent=1
                fi
                ;;
        esac
    fi
done

while IFS= read -r -t 0.1 line <&3; do
    printf '%s\n' "$line" >>"$LOG_FILE"
done || true

wait "$qemu_pid"
trap - EXIT

grep -q "qemu-tty-rx-diagnostic: PASS" "$LOG_FILE"
grep -q "qemu-tty-rx-diagnostic: line-hex=61 62 63 64 65 66 67 68" "$LOG_FILE"
grep -q "qemu-tty-rx-diagnostic: echo-hex=61 62 58 08 20 08 63 59 08 20 08 64 65 66 67 68 0d 0a" "$LOG_FILE"
grep -q "qemu-tty-rx-diagnostic: control-events=ctrl-c" "$LOG_FILE"

cat "$LOG_FILE"
