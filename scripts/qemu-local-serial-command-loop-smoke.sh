#!/usr/bin/env bash
set -euo pipefail

TALOS_BOOT_SCENARIO=qemu_local_serial_command_loop cargo -Zjson-target-spec build "$@"

ELF_FILE="target/aarch64-talos-virt/debug/talos"
IMG_FILE="$ELF_FILE.local-serial-command-loop.img"
LOG_FILE="target/qemu-local-serial-command-loop-smoke.log"
QEMU_LOG_FILE="target/qemu-local-serial-command-loop-smoke.qemu.log"
EVIDENCE_DIR="tasks/evidence/2026-05-31-qemu-local-serial-command-loop-core"
EVIDENCE_LOG="$EVIDENCE_DIR/qemu-local-serial-command-loop-smoke.log"
PORT="${TALOS_QEMU_LOCAL_COMMAND_LOOP_PORT:-54324}"

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
            *"qemu-local-serial-command-loop: ready command=0"*)
                if [ "$sent" -eq 0 ]; then
                    printf 'help\r' >&3
                    sent=1
                fi
                ;;
            *"qemu-local-serial-command-loop: ready command=1"*)
                if [ "$sent" -eq 1 ]; then
                    printf '\r' >&3
                    sent=2
                fi
                ;;
            *"qemu-local-serial-command-loop: ready command=2"*)
                if [ "$sent" -eq 2 ]; then
                    printf 'bogus\r' >&3
                    sent=3
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

grep -q "qemu-local-serial-command-loop: start" "$LOG_FILE"
grep -q "qemu-local-serial-command-loop: ready command=0" "$LOG_FILE"
grep -q "qemu-local-serial-command-loop: ready command=1" "$LOG_FILE"
grep -q "qemu-local-serial-command-loop: ready command=2" "$LOG_FILE"
grep -q "talos> help" "$LOG_FILE"
grep -q "talos: ok help" "$LOG_FILE"
grep -q "talos: commands help status" "$LOG_FILE"
grep -q "qemu-local-serial-command-loop: line command=0 hex=68 65 6c 70" "$LOG_FILE"
grep -q "qemu-local-serial-command-loop: dispatch command=0 status=handled responses=2" "$LOG_FILE"
grep -q "talos: empty-command" "$LOG_FILE"
grep -q "qemu-local-serial-command-loop: line command=1 hex=" "$LOG_FILE"
grep -q "qemu-local-serial-command-loop: dispatch command=1 status=empty-command responses=1" "$LOG_FILE"
grep -q "talos> bogus" "$LOG_FILE"
grep -q "talos: unknown-command" "$LOG_FILE"
grep -q "qemu-local-serial-command-loop: line command=2 hex=62 6f 67 75 73" "$LOG_FILE"
grep -q "qemu-local-serial-command-loop: dispatch command=2 status=unknown-command responses=1" "$LOG_FILE"
grep -q "qemu-local-serial-command-loop: ready-for-next prompt=true" "$LOG_FILE"
grep -q "qemu-local-serial-command-loop: final participants=4 expected=4 errors=0 classification=qemu-local-serial-command-loop-complete" "$LOG_FILE"
grep -q "qemu-local-serial-command-loop: PASS" "$LOG_FILE"

mkdir -p "$EVIDENCE_DIR"
tr -d '\r' <"$LOG_FILE" | sed 's/[[:space:]]*$//' >"$EVIDENCE_LOG"
cat "$EVIDENCE_LOG"
