#!/usr/bin/env bash
set -euo pipefail

TALOS_BOOT_SCENARIO="${TALOS_QEMU_LOCAL_COMMAND_LOOP_BOOT_SCENARIO:-qemu_local_serial_command_loop}" cargo -Zjson-target-spec build "$@"

ELF_FILE="target/aarch64-talos-virt/debug/talos"
IMG_FILE="$ELF_FILE.local-serial-command-loop.img"
LOG_FILE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_LOG_FILE:-target/qemu-local-serial-command-loop-smoke.log}"
QEMU_LOG_FILE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_QEMU_LOG_FILE:-target/qemu-local-serial-command-loop-smoke.qemu.log}"
EVIDENCE_DIR="${TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_DIR:-tasks/evidence/2026-05-31-qemu-local-serial-command-loop-core}"
EVIDENCE_LOG="${TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_LOG:-$EVIDENCE_DIR/qemu-local-serial-command-loop-smoke.log}"
PORT="${TALOS_QEMU_LOCAL_COMMAND_LOOP_PORT:-54324}"
LABEL="${TALOS_QEMU_LOCAL_COMMAND_LOOP_LABEL:-qemu-local-serial-command-loop}"
CLASSIFICATION="${TALOS_QEMU_LOCAL_COMMAND_LOOP_CLASSIFICATION:-qemu-local-serial-command-loop-complete}"
LITERAL_ECHO_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_LITERAL_ECHO_SMOKE:-0}"
ECHO_COMMAND_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_ECHO_COMMAND_SMOKE:-0}"
PWD_COMMAND_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_PWD_COMMAND_SMOKE:-0}"
LINE_EDITING_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_LINE_EDITING_SMOKE:-0}"
LINE_CANCEL_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_LINE_CANCEL_SMOKE:-0}"
LINE_KILL_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_LINE_KILL_SMOKE:-0}"

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
            *"$LABEL: ready command=0"*)
                if [ "$sent" -eq 0 ]; then
                    printf 'help\r' >&3
                    sent=1
                fi
                ;;
            *"$LABEL: ready command=1"*)
                if [ "$sent" -eq 1 ]; then
                    printf 'status\r' >&3
                    sent=2
                fi
                ;;
            *"$LABEL: ready command=2"*)
                if [ "$sent" -eq 2 ]; then
                    printf 'stdio\r' >&3
                    sent=3
                fi
                ;;
            *"$LABEL: ready command=3"*)
                if [ "$sent" -eq 3 ]; then
                    if [ "$LINE_KILL_SMOKE" -eq 1 ]; then
                        printf 'bogus\025pwd\r' >&3
                    elif [ "$LINE_CANCEL_SMOKE" -eq 1 ]; then
                        printf 'bogus\003' >&3
                    elif [ "$LINE_EDITING_SMOKE" -eq 1 ]; then
                        printf 'pwx\bd\r' >&3
                    elif [ "$PWD_COMMAND_SMOKE" -eq 1 ]; then
                        printf 'pwd\r' >&3
                    elif [ "$LITERAL_ECHO_SMOKE" -eq 1 ]; then
                        printf 'echo local serial works\r' >&3
                    elif [ "$ECHO_COMMAND_SMOKE" -eq 1 ]; then
                        printf 'echo hello\r' >&3
                    else
                        printf '\r' >&3
                    fi
                    sent=4
                fi
                ;;
            *"$LABEL: ready command=4"*)
                if [ "$sent" -eq 4 ]; then
                    if [ "$LINE_KILL_SMOKE" -eq 1 ]; then
                        printf 'pwd\r' >&3
                    elif [ "$LINE_CANCEL_SMOKE" -eq 1 ]; then
                        printf 'pwd\r' >&3
                    elif [ "$LINE_EDITING_SMOKE" -eq 1 ]; then
                        printf 'pwx\177d\r' >&3
                    elif [ "$PWD_COMMAND_SMOKE" -eq 1 ]; then
                        printf 'echo hello\r' >&3
                    elif [ "$LITERAL_ECHO_SMOKE" -eq 1 ] || [ "$ECHO_COMMAND_SMOKE" -eq 1 ]; then
                        printf '\r' >&3
                    else
                        printf 'bogus\r' >&3
                    fi
                    sent=5
                fi
                ;;
            *"$LABEL: ready command=5"*)
                if [ "$sent" -eq 5 ]; then
                    if [ "$LINE_EDITING_SMOKE" -eq 1 ]; then
                        printf 'echo hello\r' >&3
                    elif [ "$PWD_COMMAND_SMOKE" -eq 1 ]; then
                        printf '\r' >&3
                    else
                        printf 'bogus\r' >&3
                    fi
                    sent=6
                fi
                ;;
            *"$LABEL: ready command=6"*)
                if [ "$sent" -eq 6 ]; then
                    if [ "$LINE_EDITING_SMOKE" -eq 1 ]; then
                        printf '\r' >&3
                    else
                        printf 'bogus\r' >&3
                    fi
                    sent=7
                fi
                ;;
            *"$LABEL: ready command=7"*)
                if [ "$sent" -eq 7 ]; then
                    printf 'bogus\r' >&3
                    sent=8
                fi
                ;;
            *"$LABEL: ready command=8"*)
                if [ "$sent" -eq 8 ]; then
                    printf 'status now\r' >&3
                    sent=9
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

grep -q "$LABEL: start" "$LOG_FILE"
grep -q "$LABEL: ready command=0" "$LOG_FILE"
grep -q "$LABEL: ready command=1" "$LOG_FILE"
grep -q "$LABEL: ready command=2" "$LOG_FILE"
grep -q "$LABEL: ready command=3" "$LOG_FILE"
grep -q "$LABEL: ready command=4" "$LOG_FILE"
if [ "$PWD_COMMAND_SMOKE" -eq 1 ] || [ "$LITERAL_ECHO_SMOKE" -eq 1 ] || [ "$ECHO_COMMAND_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=5" "$LOG_FILE"
fi
if [ "$PWD_COMMAND_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=6" "$LOG_FILE"
fi
if [ "$LINE_EDITING_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=7" "$LOG_FILE"
    grep -q "$LABEL: ready command=8" "$LOG_FILE"
fi
grep -q "talos> help" "$LOG_FILE"
grep -q "talos: ok help" "$LOG_FILE"
grep -q "talos: commands help status stdio pwd echo" "$LOG_FILE"
grep -q "talos: echo forms echo hello; echo local serial works" "$LOG_FILE"
grep -q "talos: editing backspace delete ctrl-c ctrl-u" "$LOG_FILE"
grep -q "$LABEL: line command=0 hex=68 65 6c 70" "$LOG_FILE"
grep -q "$LABEL: dispatch command=0 status=handled responses=4" "$LOG_FILE"
grep -q "talos> status" "$LOG_FILE"
grep -q "talos: ok status" "$LOG_FILE"
grep -q "talos: version phase10.1-kernel-builtins-v1" "$LOG_FILE"
grep -q "talos: runtime-console runtime-console0" "$LOG_FILE"
grep -q "talos: builtins kernel-backed" "$LOG_FILE"
grep -q "$LABEL: line command=1 hex=73 74 61 74 75 73" "$LOG_FILE"
grep -q "$LABEL: dispatch command=1 status=handled responses=4" "$LOG_FILE"
grep -q "talos> stdio" "$LOG_FILE"
grep -q "talos: ok stdio" "$LOG_FILE"
grep -q "talos: fd 0 stdio-input" "$LOG_FILE"
grep -q "talos: fd 1 stdio-output" "$LOG_FILE"
grep -q "talos: fd 2 stdio-output" "$LOG_FILE"
grep -q "talos: descriptor-backed-input=true" "$LOG_FILE"
grep -q "talos: descriptor-backed-output=true" "$LOG_FILE"
grep -q "$LABEL: line command=2 hex=73 74 64 69 6f" "$LOG_FILE"
grep -q "$LABEL: dispatch command=2 status=handled responses=7" "$LOG_FILE"
if [ "$LINE_EDITING_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: dispatch command=3 status=handled responses=1 raw-bytes=6 backspaces=1 deletes=0" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=4 status=handled responses=1 raw-bytes=6 backspaces=0 deletes=1" "$LOG_FILE"
    grep -q "^/" "$LOG_FILE"
    grep -q "$LABEL: line command=3 hex=70 77 64" "$LOG_FILE"
    grep -q "$LABEL: line command=4 hex=70 77 64" "$LOG_FILE"
    grep -q "talos> echo hello" "$LOG_FILE"
    grep -q "^hello" "$LOG_FILE"
    grep -q "$LABEL: line command=5 hex=65 63 68 6f 20 68 65 6c 6c 6f" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=5 status=handled responses=1" "$LOG_FILE"
    grep -q "talos: empty-command" "$LOG_FILE"
    grep -q "$LABEL: line command=6 hex=" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=6 status=empty-command responses=1" "$LOG_FILE"
    grep -q "talos> bogus" "$LOG_FILE"
    grep -q "talos: unknown-command" "$LOG_FILE"
    grep -q "$LABEL: line command=7 hex=62 6f 67 75 73" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=7 status=unknown-command responses=1" "$LOG_FILE"
    grep -q "talos> status now" "$LOG_FILE"
    grep -q "talos: unexpected-argument" "$LOG_FILE"
    grep -q "$LABEL: line command=8 hex=73 74 61 74 75 73 20 6e 6f 77" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=8 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: final participants=9 expected=9 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$LINE_KILL_SMOKE" -eq 1 ]; then
    grep -q "talos> bogus" "$LOG_FILE"
    grep -q "talos: line-killed" "$LOG_FILE"
    grep -q "^/" "$LOG_FILE"
    grep -q "$LABEL: line command=3 hex=70 77 64" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=3 status=handled responses=2 raw-bytes=10 backspaces=0 deletes=0 truncated=false controls=1" "$LOG_FILE"
    grep -q "talos> pwd" "$LOG_FILE"
    grep -q "$LABEL: line command=4 hex=70 77 64" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=4 status=handled responses=1" "$LOG_FILE"
    grep -q "$LABEL: final participants=5 expected=5 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$LINE_CANCEL_SMOKE" -eq 1 ]; then
    grep -q "talos> bogus" "$LOG_FILE"
    grep -q "talos: line-canceled" "$LOG_FILE"
    grep -q "$LABEL: line command=3 hex=" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=3 status=line-canceled responses=1 raw-bytes=6 backspaces=0 deletes=0 truncated=false controls=1" "$LOG_FILE"
    grep -q "talos> pwd" "$LOG_FILE"
    grep -q "^/" "$LOG_FILE"
    grep -q "$LABEL: line command=4 hex=70 77 64" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=4 status=handled responses=1" "$LOG_FILE"
    grep -q "$LABEL: final participants=5 expected=5 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$PWD_COMMAND_SMOKE" -eq 1 ]; then
    grep -q "talos> pwd" "$LOG_FILE"
    grep -q "^/" "$LOG_FILE"
    grep -q "$LABEL: line command=3 hex=70 77 64" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=3 status=handled responses=1" "$LOG_FILE"
    grep -q "talos> echo hello" "$LOG_FILE"
    grep -q "^hello" "$LOG_FILE"
    grep -q "$LABEL: line command=4 hex=65 63 68 6f 20 68 65 6c 6c 6f" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=4 status=handled responses=1" "$LOG_FILE"
    grep -q "talos: empty-command" "$LOG_FILE"
    grep -q "$LABEL: line command=5 hex=" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=5 status=empty-command responses=1" "$LOG_FILE"
    grep -q "talos> bogus" "$LOG_FILE"
    grep -q "talos: unknown-command" "$LOG_FILE"
    grep -q "$LABEL: line command=6 hex=62 6f 67 75 73" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=6 status=unknown-command responses=1" "$LOG_FILE"
    grep -q "$LABEL: final participants=7 expected=7 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$LITERAL_ECHO_SMOKE" -eq 1 ]; then
    grep -q "talos> echo local serial works" "$LOG_FILE"
    grep -q "^local serial works" "$LOG_FILE"
    grep -q "$LABEL: line command=3 hex=65 63 68 6f 20 6c 6f 63 61 6c 20 73 65 72 69 61 6c 20 77 6f 72 6b 73" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=3 status=handled responses=1" "$LOG_FILE"
    grep -q "talos: empty-command" "$LOG_FILE"
    grep -q "$LABEL: line command=4 hex=" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=4 status=empty-command responses=1" "$LOG_FILE"
    grep -q "talos> bogus" "$LOG_FILE"
    grep -q "talos: unknown-command" "$LOG_FILE"
    grep -q "$LABEL: line command=5 hex=62 6f 67 75 73" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=5 status=unknown-command responses=1" "$LOG_FILE"
    grep -q "$LABEL: final participants=6 expected=6 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$ECHO_COMMAND_SMOKE" -eq 1 ]; then
    grep -q "talos> echo hello" "$LOG_FILE"
    grep -q "^hello" "$LOG_FILE"
    grep -q "$LABEL: line command=3 hex=65 63 68 6f 20 68 65 6c 6c 6f" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=3 status=handled responses=1" "$LOG_FILE"
    grep -q "talos: empty-command" "$LOG_FILE"
    grep -q "$LABEL: line command=4 hex=" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=4 status=empty-command responses=1" "$LOG_FILE"
    grep -q "talos> bogus" "$LOG_FILE"
    grep -q "talos: unknown-command" "$LOG_FILE"
    grep -q "$LABEL: line command=5 hex=62 6f 67 75 73" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=5 status=unknown-command responses=1" "$LOG_FILE"
    grep -q "$LABEL: final participants=6 expected=6 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
else
grep -q "talos: empty-command" "$LOG_FILE"
grep -q "$LABEL: line command=3 hex=" "$LOG_FILE"
grep -q "$LABEL: dispatch command=3 status=empty-command responses=1" "$LOG_FILE"
grep -q "talos> bogus" "$LOG_FILE"
grep -q "talos: unknown-command" "$LOG_FILE"
grep -q "$LABEL: line command=4 hex=62 6f 67 75 73" "$LOG_FILE"
grep -q "$LABEL: dispatch command=4 status=unknown-command responses=1" "$LOG_FILE"
    grep -q "$LABEL: final participants=5 expected=5 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
fi
grep -q "$LABEL: ready-for-next prompt=true" "$LOG_FILE"
grep -q "$LABEL: PASS" "$LOG_FILE"

mkdir -p "$EVIDENCE_DIR"
tr -d '\r' <"$LOG_FILE" | sed 's/[[:space:]]*$//' >"$EVIDENCE_LOG"
cat "$EVIDENCE_LOG"
