#!/usr/bin/env bash
set -euo pipefail

TALOS_QEMU_DIAGNOSTIC_COMMAND_CHANNEL_SMOKE=1 cargo -Zjson-target-spec build "$@"

ELF_FILE="target/aarch64-talos-virt/debug/talos"
IMG_FILE="$ELF_FILE.diagnostic-command-channel.img"
LOG_FILE="target/qemu-diagnostic-command-channel-smoke.log"
QEMU_LOG_FILE="target/qemu-diagnostic-command-channel-smoke.qemu.log"
PORT="${TALOS_QEMU_DIAGNOSTIC_COMMAND_CHANNEL_PORT:-54323}"

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
. "$script_dir/objcopy-tool.sh"

"$objcopy_tool" -O binary "$ELF_FILE" "$IMG_FILE"
: >"$LOG_FILE"
: >"$QEMU_LOG_FILE"

qemu-system-aarch64 \
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
            *"qemu-diagnostic-command-channel-smoke: ready command=0"*)
                if [ "$sent" -eq 0 ]; then
                    printf 'help\r' >&3
                    sent=1
                fi
                ;;
            *"qemu-diagnostic-command-channel-smoke: ready command=1"*)
                if [ "$sent" -eq 1 ]; then
                    printf 'list\r' >&3
                    sent=2
                fi
                ;;
            *"qemu-diagnostic-command-channel-smoke: ready command=2"*)
                if [ "$sent" -eq 2 ]; then
                    printf 'bogus\r' >&3
                    sent=3
                fi
                ;;
            *"qemu-diagnostic-command-channel-smoke: ready command=3"*)
                if [ "$sent" -eq 3 ]; then
                    printf 'status\r' >&3
                    sent=4
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

grep -q "qemu-diagnostic-command-channel-smoke: PASS" "$LOG_FILE"
grep -q "qemu-diagnostic-command-channel-smoke: line command=0 hex=68 65 6c 70" "$LOG_FILE"
grep -q "qemu-diagnostic-command-channel-smoke: line command=1 hex=6c 69 73 74" "$LOG_FILE"
grep -q "qemu-diagnostic-command-channel-smoke: line command=2 hex=62 6f 67 75 73" "$LOG_FILE"
grep -q "qemu-diagnostic-command-channel-smoke: line command=3 hex=73 74 61 74 75 73" "$LOG_FILE"
grep -q "diag: ok help" "$LOG_FILE"
grep -q "diag: ok list" "$LOG_FILE"
grep -q "diag: error unknown-command" "$LOG_FILE"
grep -q "diag: ok status" "$LOG_FILE"
grep -q "diag: runtime-console runtime-console0" "$LOG_FILE"
grep -q "qemu-diagnostic-command-channel-smoke: dispatch command=0 status=handled responses=2" "$LOG_FILE"
grep -q "qemu-diagnostic-command-channel-smoke: dispatch command=1 status=handled responses=2" "$LOG_FILE"
grep -q "qemu-diagnostic-command-channel-smoke: dispatch command=2 status=unknown-command responses=1" "$LOG_FILE"
grep -q "qemu-diagnostic-command-channel-smoke: dispatch command=3 status=handled responses=6" "$LOG_FILE"

cat "$LOG_FILE"
