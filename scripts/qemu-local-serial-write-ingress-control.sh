#!/usr/bin/env bash
set -euo pipefail

TALOS_BOOT_SCENARIO=qemu_local_literal_echo cargo -Zjson-target-spec build "$@"

ELF_FILE="target/aarch64-talos-virt/debug/talos"
IMG_FILE="$ELF_FILE.local-serial-write-ingress-control.img"
RAW_LOG_FILE="${TALOS_QEMU_SERIAL_WRITE_INGRESS_RAW_LOG_FILE:-target/qemu-local-serial-write-ingress-control.raw.log}"
EVENT_LOG_FILE="${TALOS_QEMU_SERIAL_WRITE_INGRESS_EVENT_LOG_FILE:-target/qemu-local-serial-write-ingress-control.events.log}"
QEMU_LOG_FILE="${TALOS_QEMU_SERIAL_WRITE_INGRESS_QEMU_LOG_FILE:-target/qemu-local-serial-write-ingress-control.qemu.log}"
EVIDENCE_DIR="${TALOS_QEMU_SERIAL_WRITE_INGRESS_EVIDENCE_DIR:-tasks/evidence/2026-06-01-qemu-local-serial-write-ingress-control-core}"
EVIDENCE_LOG="${TALOS_QEMU_SERIAL_WRITE_INGRESS_EVIDENCE_LOG:-$EVIDENCE_DIR/qemu-local-serial-write-ingress-control.log}"
PORT="${TALOS_QEMU_SERIAL_WRITE_INGRESS_PORT:-54332}"
LABEL="qemu-local-serial-write-ingress-control"
CONTROL_DELAY="${TALOS_QEMU_SERIAL_WRITE_INGRESS_DELAY_SECONDS:-0.25}"

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
. "$script_dir/objcopy-tool.sh"
. "$script_dir/qemu-tool.sh"

"$objcopy_tool" -O binary "$ELF_FILE" "$IMG_FILE"
: >"$RAW_LOG_FILE"
: >"$EVENT_LOG_FILE"
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

commands=(
    $'help\r'
    $'status\r'
    $'stdio\r'
    $'echo local serial works\r'
    $'\r'
    $'bogus\r'
)
command_names=(
    "help"
    "status"
    "stdio"
    "echo local serial works"
    "<enter>"
    "bogus"
)

sent=0
prompt_seen=0
buffer=""

while kill -0 "$qemu_pid" 2>/dev/null; do
    if IFS= read -r -n 1 -t 0.2 ch <&3; then
        printf '%s' "$ch" >>"$RAW_LOG_FILE"
        buffer="${buffer}${ch}"

        if [[ "$buffer" == *"talos> " ]] && [ "$sent" -lt "${#commands[@]}" ]; then
            printf '%s: observed-prompt command=%d boundary=talos-prompt delayed-input=%q injection=after-visible-prompt delay-seconds=%s\n' \
                "$LABEL" "$sent" "${command_names[$sent]}" "$CONTROL_DELAY" >>"$EVENT_LOG_FILE"
            sleep "$CONTROL_DELAY"
            printf '%s' "${commands[$sent]}" >&3
            sent=$((sent + 1))
            prompt_seen=$((prompt_seen + 1))
            buffer=""
        elif [ "${#buffer}" -gt 8192 ]; then
            buffer="${buffer: -2048}"
        fi
    fi
done

while IFS= read -r -n 1 -t 0.1 ch <&3; do
    printf '%s' "$ch" >>"$RAW_LOG_FILE"
done || true

wait "$qemu_pid"
trap - EXIT

mkdir -p "$EVIDENCE_DIR"
tr '\r' '\n' <"$RAW_LOG_FILE" | sed 's/[[:space:]]*$//' >"$EVIDENCE_LOG"
cat "$EVENT_LOG_FILE" >>"$EVIDENCE_LOG"

grep -q "$LABEL: observed-prompt command=0" "$EVIDENCE_LOG"
grep -q "$LABEL: observed-prompt command=3" "$EVIDENCE_LOG"
grep -q "qemu-local-literal-echo: start command-count=6 backend=runtime-console0/qemu-virt-pl011 input=fd0/runtime-console0/tty-canonical-lite builtins=kernel-backed descriptor-backed-input=true descriptor-backed-output=true" "$EVIDENCE_LOG"
grep -q "talos> help" "$EVIDENCE_LOG"
grep -q "talos: ok help" "$EVIDENCE_LOG"
grep -q "talos> status" "$EVIDENCE_LOG"
grep -q "talos: runtime-console runtime-console0" "$EVIDENCE_LOG"
grep -q "talos> stdio" "$EVIDENCE_LOG"
grep -q "talos: descriptor-backed-input=true" "$EVIDENCE_LOG"
grep -q "talos: descriptor-backed-output=true" "$EVIDENCE_LOG"
grep -q "talos> echo local serial works" "$EVIDENCE_LOG"
grep -q "^local serial works" "$EVIDENCE_LOG"
grep -q "qemu-local-literal-echo: line command=3 hex=65 63 68 6f 20 6c 6f 63 61 6c 20 73 65 72 69 61 6c 20 77 6f 72 6b 73" "$EVIDENCE_LOG"
grep -q "qemu-local-literal-echo: dispatch command=3 status=handled responses=1" "$EVIDENCE_LOG"
grep -q "talos: empty-command" "$EVIDENCE_LOG"
grep -q "talos: unknown-command" "$EVIDENCE_LOG"
grep -q "qemu-local-literal-echo: ready-for-next prompt=true" "$EVIDENCE_LOG"
grep -q "qemu-local-literal-echo: PASS" "$EVIDENCE_LOG"

{
    printf '%s: prompts-observed=%d commands-written=%d\n' "$LABEL" "$prompt_seen" "$sent"
    printf '%s: final participants=6 expected=6 errors=0 classification=serial-write-ingress-control-complete\n' "$LABEL"
    printf '%s: PASS\n' "$LABEL"
} >>"$EVIDENCE_LOG"

cat "$EVIDENCE_LOG"
