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
LS_ROOT_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_LS_ROOT_SMOKE:-0}"
LS_BIN_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_LS_BIN_SMOKE:-0}"
CAT_BANNER_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_CAT_BANNER_SMOKE:-0}"
CAT_CWD_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_CAT_CWD_SMOKE:-0}"
SHELL_VFS_EXEC_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_VFS_EXEC_SMOKE:-0}"
SHELL_LITERAL_ARGV_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_LITERAL_ARGV_SMOKE:-0}"
SHELL_PATH_LOOKUP_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_PATH_LOOKUP_SMOKE:-0}"
SHELL_WAITPID_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_WAITPID_SMOKE:-0}"
CD_FIXED_DIRS_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_CD_FIXED_DIRS_SMOKE:-0}"
LS_CWD_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_LS_CWD_SMOKE:-0}"
LINE_EDITING_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_LINE_EDITING_SMOKE:-0}"
LINE_CANCEL_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_LINE_CANCEL_SMOKE:-0}"
LINE_KILL_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_LINE_KILL_SMOKE:-0}"

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
            *"$LABEL: ready command=10"*)
                if [ "$sent" -eq 10 ] && [ "$SHELL_WAITPID_SMOKE" -eq 1 ]; then
                    printf 'exec /bin/zero\r' >&3
                    sent=11
                elif [ "$sent" -eq 10 ] && [ "$SHELL_VFS_EXEC_SMOKE" -eq 1 ]; then
                    printf 'exec bin/init\r' >&3
                    sent=11
                elif [ "$sent" -eq 10 ] && [ "$SHELL_LITERAL_ARGV_SMOKE" -eq 1 ]; then
                    printf 'exec bin/init\r' >&3
                    sent=11
                elif [ "$sent" -eq 10 ] && [ "$SHELL_PATH_LOOKUP_SMOKE" -eq 1 ]; then
                    printf 'exec missing\r' >&3
                    sent=11
                elif [ "$sent" -eq 10 ] && [ "$CD_FIXED_DIRS_SMOKE" -eq 1 ]; then
                    printf 'cd /missing\r' >&3
                    sent=11
                elif [ "$sent" -eq 10 ] && [ "$LS_CWD_SMOKE" -eq 1 ]; then
                    printf 'ls\r' >&3
                    sent=11
                fi
                ;;
            *"$LABEL: ready command=11"*)
                if [ "$sent" -eq 11 ] && [ "$SHELL_WAITPID_SMOKE" -eq 1 ]; then
                    printf 'waitpid\r' >&3
                    sent=12
                elif [ "$sent" -eq 11 ] && [ "$SHELL_VFS_EXEC_SMOKE" -eq 1 ]; then
                    printf 'exec /bin\r' >&3
                    sent=12
                elif [ "$sent" -eq 11 ] && [ "$SHELL_LITERAL_ARGV_SMOKE" -eq 1 ]; then
                    printf 'cat /etc/banner.txt\r' >&3
                    sent=12
                elif [ "$sent" -eq 11 ] && [ "$SHELL_PATH_LOOKUP_SMOKE" -eq 1 ]; then
                    printf 'exec bin/status42\r' >&3
                    sent=12
                elif [ "$sent" -eq 11 ] && [ "$CD_FIXED_DIRS_SMOKE" -eq 1 ]; then
                    printf 'pwd\r' >&3
                    sent=12
                elif [ "$sent" -eq 11 ] && [ "$LS_CWD_SMOKE" -eq 1 ]; then
                    printf 'bogus\r' >&3
                    sent=12
                fi
                ;;
            *"$LABEL: ready command=12"*)
                if [ "$sent" -eq 12 ] && [ "$SHELL_WAITPID_SMOKE" -eq 1 ]; then
                    printf 'exec /missing\r' >&3
                    sent=13
                elif [ "$sent" -eq 12 ] && [ "$SHELL_VFS_EXEC_SMOKE" -eq 1 ]; then
                    printf 'exec /etc/banner.txt\r' >&3
                    sent=13
                elif [ "$sent" -eq 12 ] && [ "$SHELL_PATH_LOOKUP_SMOKE" -eq 1 ]; then
                    printf 'exec /bin\r' >&3
                    sent=13
                fi
                ;;
            *"$LABEL: ready command=13"*)
                if [ "$sent" -eq 13 ] && [ "$SHELL_WAITPID_SMOKE" -eq 1 ]; then
                    printf 'exec bin/init\r' >&3
                    sent=14
                elif [ "$sent" -eq 13 ] && [ "$SHELL_VFS_EXEC_SMOKE" -eq 1 ]; then
                    printf 'exec /empty\r' >&3
                    sent=14
                elif [ "$sent" -eq 13 ] && [ "$SHELL_PATH_LOOKUP_SMOKE" -eq 1 ]; then
                    printf 'exec /etc/banner.txt\r' >&3
                    sent=14
                fi
                ;;
            *"$LABEL: ready command=14"*)
                if [ "$sent" -eq 14 ] && [ "$SHELL_WAITPID_SMOKE" -eq 1 ]; then
                    printf 'exec /bin\r' >&3
                    sent=15
                elif [ "$sent" -eq 14 ] && [ "$SHELL_VFS_EXEC_SMOKE" -eq 1 ]; then
                    printf 'cat /etc/banner.txt\r' >&3
                    sent=15
                elif [ "$sent" -eq 14 ] && [ "$SHELL_PATH_LOOKUP_SMOKE" -eq 1 ]; then
                    printf 'exec /empty\r' >&3
                    sent=15
                fi
                ;;
            *"$LABEL: ready command=15"*)
                if [ "$sent" -eq 15 ] && [ "$SHELL_WAITPID_SMOKE" -eq 1 ]; then
                    printf 'exec /etc/banner.txt\r' >&3
                    sent=16
                elif [ "$sent" -eq 15 ] && [ "$SHELL_PATH_LOOKUP_SMOKE" -eq 1 ]; then
                    printf 'exec /bin/status42 *\r' >&3
                    sent=16
                fi
                ;;
            *"$LABEL: ready command=16"*)
                if [ "$sent" -eq 16 ] && [ "$SHELL_WAITPID_SMOKE" -eq 1 ]; then
                    printf 'exec /empty\r' >&3
                    sent=17
                elif [ "$sent" -eq 16 ] && [ "$SHELL_PATH_LOOKUP_SMOKE" -eq 1 ]; then
                    printf 'cat /etc/banner.txt\r' >&3
                    sent=17
                fi
                ;;
            *"$LABEL: ready command=17"*)
                if [ "$sent" -eq 17 ] && [ "$SHELL_WAITPID_SMOKE" -eq 1 ]; then
                    printf 'cat /etc/banner.txt\r' >&3
                    sent=18
                fi
                ;;
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
                    elif [ "$LS_BIN_SMOKE" -eq 1 ]; then
                        printf 'ls /bin\r' >&3
                    elif [ "$CAT_BANNER_SMOKE" -eq 1 ]; then
                        printf 'cat /etc/banner.txt\r' >&3
                    elif [ "$CAT_CWD_SMOKE" -eq 1 ]; then
                        printf 'cd /etc\r' >&3
                    elif [ "$SHELL_WAITPID_SMOKE" -eq 1 ]; then
                        printf 'waitpid\r' >&3
                    elif [ "$SHELL_LITERAL_ARGV_SMOKE" -eq 1 ]; then
                        printf 'exec /bin/status42 alpha beta\r' >&3
                    elif [ "$SHELL_PATH_LOOKUP_SMOKE" -eq 1 ]; then
                        printf 'exec status42 alpha beta\r' >&3
                    elif [ "$SHELL_VFS_EXEC_SMOKE" -eq 1 ]; then
                        printf 'exec /bin/status42\r' >&3
                    elif [ "$CD_FIXED_DIRS_SMOKE" -eq 1 ]; then
                        printf 'pwd\r' >&3
                    elif [ "$LS_CWD_SMOKE" -eq 1 ]; then
                        printf 'pwd\r' >&3
                    elif [ "$LS_ROOT_SMOKE" -eq 1 ]; then
                        printf 'ls /\r' >&3
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
                    elif [ "$LS_BIN_SMOKE" -eq 1 ]; then
                        printf 'ls /\r' >&3
                    elif [ "$CAT_BANNER_SMOKE" -eq 1 ]; then
                        printf 'ls /bin\r' >&3
                    elif [ "$CAT_CWD_SMOKE" -eq 1 ]; then
                        printf 'cat banner.txt\r' >&3
                    elif [ "$SHELL_WAITPID_SMOKE" -eq 1 ]; then
                        printf 'exec /bin/status42\r' >&3
                    elif [ "$SHELL_LITERAL_ARGV_SMOKE" -eq 1 ]; then
                        printf 'waitpid\r' >&3
                    elif [ "$SHELL_PATH_LOOKUP_SMOKE" -eq 1 ]; then
                        printf 'waitpid\r' >&3
                    elif [ "$SHELL_VFS_EXEC_SMOKE" -eq 1 ]; then
                        printf 'laststatus\r' >&3
                    elif [ "$CD_FIXED_DIRS_SMOKE" -eq 1 ]; then
                        printf 'cd /etc\r' >&3
                    elif [ "$LS_CWD_SMOKE" -eq 1 ]; then
                        printf 'ls\r' >&3
                    elif [ "$LS_ROOT_SMOKE" -eq 1 ]; then
                        printf '\r' >&3
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
                    elif [ "$LS_BIN_SMOKE" -eq 1 ]; then
                        printf '\r' >&3
                    elif [ "$CAT_BANNER_SMOKE" -eq 1 ]; then
                        printf '\r' >&3
                    elif [ "$CAT_CWD_SMOKE" -eq 1 ]; then
                        printf 'cd /\r' >&3
                    elif [ "$SHELL_WAITPID_SMOKE" -eq 1 ]; then
                        printf 'waitpid\r' >&3
                    elif [ "$SHELL_LITERAL_ARGV_SMOKE" -eq 1 ]; then
                        printf 'laststatus\r' >&3
                    elif [ "$SHELL_PATH_LOOKUP_SMOKE" -eq 1 ]; then
                        printf 'laststatus\r' >&3
                    elif [ "$SHELL_VFS_EXEC_SMOKE" -eq 1 ]; then
                        printf 'exec /bin/init\r' >&3
                    elif [ "$CD_FIXED_DIRS_SMOKE" -eq 1 ]; then
                        printf 'pwd\r' >&3
                    elif [ "$LS_CWD_SMOKE" -eq 1 ]; then
                        printf 'cd /etc\r' >&3
                    elif [ "$LS_ROOT_SMOKE" -eq 1 ]; then
                        printf 'bogus\r' >&3
                    else
                        printf 'bogus\r' >&3
                    fi
                    sent=6
                fi
                ;;
            *"$LABEL: ready command=6"*)
                if [ "$sent" -eq 6 ]; then
                    if [ "$CD_FIXED_DIRS_SMOKE" -eq 1 ]; then
                        printf 'cd /bin\r' >&3
                    elif [ "$CAT_CWD_SMOKE" -eq 1 ]; then
                        printf 'cat banner.txt\r' >&3
                    elif [ "$SHELL_WAITPID_SMOKE" -eq 1 ]; then
                        printf 'waitpid\r' >&3
                    elif [ "$SHELL_LITERAL_ARGV_SMOKE" -eq 1 ]; then
                        printf 'exec /bin/init\r' >&3
                    elif [ "$SHELL_PATH_LOOKUP_SMOKE" -eq 1 ]; then
                        printf 'exec init\r' >&3
                    elif [ "$SHELL_VFS_EXEC_SMOKE" -eq 1 ]; then
                        printf 'laststatus\r' >&3
                    elif [ "$LS_CWD_SMOKE" -eq 1 ]; then
                        printf 'ls\r' >&3
                    elif [ "$LINE_EDITING_SMOKE" -eq 1 ]; then
                        printf '\r' >&3
                    else
                        printf 'bogus\r' >&3
                    fi
                    sent=7
                fi
                ;;
            *"$LABEL: ready command=7"*)
                if [ "$sent" -eq 7 ]; then
                    if [ "$CD_FIXED_DIRS_SMOKE" -eq 1 ]; then
                        printf 'pwd\r' >&3
                    elif [ "$LS_CWD_SMOKE" -eq 1 ]; then
                        printf 'cd /bin\r' >&3
                    elif [ "$SHELL_WAITPID_SMOKE" -eq 1 ]; then
                        printf 'laststatus\r' >&3
                    elif [ "$SHELL_LITERAL_ARGV_SMOKE" -eq 1 ]; then
                        printf 'exec /bin/zero\r' >&3
                    elif [ "$SHELL_PATH_LOOKUP_SMOKE" -eq 1 ]; then
                        printf 'exec zero\r' >&3
                    elif [ "$SHELL_VFS_EXEC_SMOKE" -eq 1 ]; then
                        printf 'exec /bin/zero\r' >&3
                    else
                        printf 'bogus\r' >&3
                    fi
                    sent=8
                fi
                ;;
            *"$LABEL: ready command=8"*)
                if [ "$sent" -eq 8 ]; then
                    if [ "$CD_FIXED_DIRS_SMOKE" -eq 1 ]; then
                        printf 'cd /\r' >&3
                    elif [ "$LS_CWD_SMOKE" -eq 1 ]; then
                        printf 'ls\r' >&3
                    elif [ "$SHELL_WAITPID_SMOKE" -eq 1 ]; then
                        printf 'exec /bin/init\r' >&3
                    elif [ "$SHELL_LITERAL_ARGV_SMOKE" -eq 1 ]; then
                        printf 'exec /bin/status42 *\r' >&3
                    elif [ "$SHELL_PATH_LOOKUP_SMOKE" -eq 1 ]; then
                        printf 'exec /bin/status42 gamma\r' >&3
                    elif [ "$SHELL_VFS_EXEC_SMOKE" -eq 1 ]; then
                        printf 'laststatus\r' >&3
                    else
                        printf 'status now\r' >&3
                    fi
                    sent=9
                fi
                ;;
            *"$LABEL: ready command=9"*)
                if [ "$sent" -eq 9 ] && [ "$SHELL_WAITPID_SMOKE" -eq 1 ]; then
                    printf 'waitpid\r' >&3
                    sent=10
                elif [ "$sent" -eq 9 ] && [ "$SHELL_VFS_EXEC_SMOKE" -eq 1 ]; then
                    printf 'exec /missing\r' >&3
                    sent=10
                elif [ "$sent" -eq 9 ] && [ "$SHELL_LITERAL_ARGV_SMOKE" -eq 1 ]; then
                    printf 'exec /missing\r' >&3
                    sent=10
                elif [ "$sent" -eq 9 ] && [ "$SHELL_PATH_LOOKUP_SMOKE" -eq 1 ]; then
                    printf 'laststatus\r' >&3
                    sent=10
                elif [ "$sent" -eq 9 ] && [ "$CD_FIXED_DIRS_SMOKE" -eq 1 ]; then
                    printf 'pwd\r' >&3
                    sent=10
                elif [ "$sent" -eq 9 ] && [ "$LS_CWD_SMOKE" -eq 1 ]; then
                    printf 'cd /\r' >&3
                    sent=10
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
if [ "$PWD_COMMAND_SMOKE" -eq 1 ] || [ "$LS_ROOT_SMOKE" -eq 1 ] || [ "$LS_BIN_SMOKE" -eq 1 ] || [ "$CAT_BANNER_SMOKE" -eq 1 ] || [ "$CAT_CWD_SMOKE" -eq 1 ] || [ "$SHELL_VFS_EXEC_SMOKE" -eq 1 ] || [ "$SHELL_LITERAL_ARGV_SMOKE" -eq 1 ] || [ "$SHELL_PATH_LOOKUP_SMOKE" -eq 1 ] || [ "$SHELL_WAITPID_SMOKE" -eq 1 ] || [ "$CD_FIXED_DIRS_SMOKE" -eq 1 ] || [ "$LS_CWD_SMOKE" -eq 1 ] || [ "$LITERAL_ECHO_SMOKE" -eq 1 ] || [ "$ECHO_COMMAND_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=5" "$LOG_FILE"
fi
if [ "$PWD_COMMAND_SMOKE" -eq 1 ] || [ "$LS_BIN_SMOKE" -eq 1 ] || [ "$CAT_BANNER_SMOKE" -eq 1 ] || [ "$CAT_CWD_SMOKE" -eq 1 ] || [ "$SHELL_VFS_EXEC_SMOKE" -eq 1 ] || [ "$SHELL_LITERAL_ARGV_SMOKE" -eq 1 ] || [ "$SHELL_PATH_LOOKUP_SMOKE" -eq 1 ] || [ "$SHELL_WAITPID_SMOKE" -eq 1 ] || [ "$CD_FIXED_DIRS_SMOKE" -eq 1 ] || [ "$LS_CWD_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=6" "$LOG_FILE"
fi
if [ "$CD_FIXED_DIRS_SMOKE" -eq 1 ] || [ "$LS_CWD_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=7" "$LOG_FILE"
    grep -q "$LABEL: ready command=8" "$LOG_FILE"
    grep -q "$LABEL: ready command=9" "$LOG_FILE"
    grep -q "$LABEL: ready command=10" "$LOG_FILE"
    grep -q "$LABEL: ready command=11" "$LOG_FILE"
fi
if [ "$SHELL_VFS_EXEC_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=7" "$LOG_FILE"
    grep -q "$LABEL: ready command=8" "$LOG_FILE"
    grep -q "$LABEL: ready command=9" "$LOG_FILE"
    grep -q "$LABEL: ready command=10" "$LOG_FILE"
    grep -q "$LABEL: ready command=11" "$LOG_FILE"
    grep -q "$LABEL: ready command=12" "$LOG_FILE"
    grep -q "$LABEL: ready command=13" "$LOG_FILE"
    grep -q "$LABEL: ready command=14" "$LOG_FILE"
fi
if [ "$SHELL_LITERAL_ARGV_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=7" "$LOG_FILE"
    grep -q "$LABEL: ready command=8" "$LOG_FILE"
    grep -q "$LABEL: ready command=9" "$LOG_FILE"
    grep -q "$LABEL: ready command=10" "$LOG_FILE"
    grep -q "$LABEL: ready command=11" "$LOG_FILE"
fi
if [ "$SHELL_PATH_LOOKUP_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=7" "$LOG_FILE"
    grep -q "$LABEL: ready command=8" "$LOG_FILE"
    grep -q "$LABEL: ready command=9" "$LOG_FILE"
    grep -q "$LABEL: ready command=10" "$LOG_FILE"
    grep -q "$LABEL: ready command=11" "$LOG_FILE"
    grep -q "$LABEL: ready command=12" "$LOG_FILE"
    grep -q "$LABEL: ready command=13" "$LOG_FILE"
    grep -q "$LABEL: ready command=14" "$LOG_FILE"
    grep -q "$LABEL: ready command=15" "$LOG_FILE"
    grep -q "$LABEL: ready command=16" "$LOG_FILE"
fi
if [ "$SHELL_WAITPID_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=7" "$LOG_FILE"
    grep -q "$LABEL: ready command=8" "$LOG_FILE"
    grep -q "$LABEL: ready command=9" "$LOG_FILE"
    grep -q "$LABEL: ready command=10" "$LOG_FILE"
    grep -q "$LABEL: ready command=11" "$LOG_FILE"
    grep -q "$LABEL: ready command=12" "$LOG_FILE"
    grep -q "$LABEL: ready command=13" "$LOG_FILE"
    grep -q "$LABEL: ready command=14" "$LOG_FILE"
    grep -q "$LABEL: ready command=15" "$LOG_FILE"
    grep -q "$LABEL: ready command=16" "$LOG_FILE"
    grep -q "$LABEL: ready command=17" "$LOG_FILE"
fi
if [ "$LINE_EDITING_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=7" "$LOG_FILE"
    grep -q "$LABEL: ready command=8" "$LOG_FILE"
fi
grep -q "talos> help" "$LOG_FILE"
grep -q "talos: ok help" "$LOG_FILE"
grep -q "talos: commands help status stdio pwd echo ls cat" "$LOG_FILE"
grep -q "talos: echo forms echo hello; echo local serial works" "$LOG_FILE"
grep -q "talos: editing backspace delete ctrl-c ctrl-u" "$LOG_FILE"
grep -q "$LABEL: line command=0 hex=68 65 6c 70" "$LOG_FILE"
grep -q "$LABEL: dispatch command=0 status=handled responses=4" "$LOG_FILE"
grep -q "talos> status" "$LOG_FILE"
grep -q "talos: ok status" "$LOG_FILE"
grep -q "talos: version phase10.2-kernel-builtins-v1" "$LOG_FILE"
grep -q "talos: runtime-console runtime-console0" "$LOG_FILE"
grep -q "talos: builtins kernel-backed" "$LOG_FILE"
grep -q "$LABEL: line command=1 hex=73 74 61 74 75 73" "$LOG_FILE"
grep -q "talos: commands help status stdio pwd echo ls cat" "$LOG_FILE"
grep -q "$LABEL: dispatch command=1 status=handled responses=5" "$LOG_FILE"
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
elif [ "$CAT_CWD_SMOKE" -eq 1 ]; then
    grep -q "talos> cd /etc" "$LOG_FILE"
    grep -q "$LABEL: line command=3 hex=63 64 20 2f 65 74 63" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=3 status=handled responses=0" "$LOG_FILE"
    grep -q "talos> cat banner.txt" "$LOG_FILE"
    grep -q "^Talos initramfs fixture" "$LOG_FILE"
    grep -q "$LABEL: line command=4 hex=63 61 74 20 62 61 6e 6e 65 72 2e 74 78 74" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=4 status=handled responses=1" "$LOG_FILE"
    grep -q "talos> cd /" "$LOG_FILE"
    grep -q "$LABEL: line command=5 hex=63 64 20 2f" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=5 status=handled responses=0" "$LOG_FILE"
    grep -q "talos: not-found" "$LOG_FILE"
    grep -q "$LABEL: line command=6 hex=63 61 74 20 62 61 6e 6e 65 72 2e 74 78 74" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=6 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: final participants=7 expected=7 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_WAITPID_SMOKE" -eq 1 ]; then
    grep -q "talos> waitpid" "$LOG_FILE"
    grep -q "talos: waitpid no-child source=lifecycle-record" "$LOG_FILE"
    grep -q "$LABEL: line command=3 hex=77 61 69 74 70 69 64" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=3 status=handled responses=1" "$LOG_FILE"
    grep -q "talos> exec /bin/status42" "$LOG_FILE"
    grep -q "talos: exec path=/bin/status42 source=vfs-open-read" "$LOG_FILE"
    grep -Eq "talos: exec-lifecycle pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true" "$LOG_FILE"
    grep -Eq "talos: waitpid pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "$LABEL: line command=5 hex=77 61 69 74 70 69 64" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=5 status=handled responses=1" "$LOG_FILE"
    grep -q "$LABEL: line command=6 hex=77 61 69 74 70 69 64" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=6 status=handled responses=1" "$LOG_FILE"
    grep -Eq "talos: last-process pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "$LABEL: line command=7 hex=6c 61 73 74 73 74 61 74 75 73" "$LOG_FILE"
    grep -q "talos> exec /bin/init" "$LOG_FILE"
    grep -Eq "talos: waitpid pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/init state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "$LABEL: line command=9 hex=77 61 69 74 70 69 64" "$LOG_FILE"
    grep -q "talos> exec /bin/zero" "$LOG_FILE"
    grep -Eq "talos: waitpid pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/zero state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "$LABEL: line command=11 hex=77 61 69 74 70 69 64" "$LOG_FILE"
    grep -q "talos> exec /missing" "$LOG_FILE"
    grep -q "talos: exec-not-found" "$LOG_FILE"
    grep -q "talos> exec bin/init" "$LOG_FILE"
    grep -q "talos: exec-invalid-path" "$LOG_FILE"
    grep -q "talos> exec /bin" "$LOG_FILE"
    grep -q "talos> exec /etc/banner.txt" "$LOG_FILE"
    grep -q "talos> exec /empty" "$LOG_FILE"
    grep -q "talos: exec-not-executable" "$LOG_FILE"
    grep -q "talos> cat /etc/banner.txt" "$LOG_FILE"
    grep -q "^Talos initramfs fixture" "$LOG_FILE"
    grep -q "$LABEL: final participants=18 expected=18 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_PATH_LOOKUP_SMOKE" -eq 1 ]; then
    grep -q "talos> exec status42 alpha beta" "$LOG_FILE"
    grep -q "talos: exec path=/bin/status42 source=vfs-open-read" "$LOG_FILE"
    grep -Eq "talos: exec-source bytes=0x[0-9a-f]+ digest=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-loader fixture=phase8-program-loader-elf64-aarch64-v1 entry=0x[0-9a-f]+ segments=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-launch launch-boundary=phase8-initial-process-launch-plan-v1 stack-boundary=phase8-initial-user-stack-plan-v1 address-space=0x[0-9a-f]+ materialization=0x[0-9a-f]+ initial-sp=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-descriptors owner=0x[0-9a-f]+ inherited-count=0x0000000000000003 fd0=stdio-input fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -Eq "talos: exec-startup-abi state=literal-argv-absolute-empty-envp argc=0x0000000000000003 argv0=/bin/status42 argv1=alpha argv2=beta argv0-ptr=0x[0-9a-f]+ argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x[0-9a-f]+ copied-startup-bytes=0x0000000000000049 source=initial-user-stack-record" "$LOG_FILE"
    grep -Eq "talos: exec-lifecycle pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true" "$LOG_FILE"
    grep -q "talos: exec-status boundary=lower-aarch64-svc-status-equivalent marker=0x0000000000007a10 status=0x000000000000002a complete=true source=lifecycle-record" "$LOG_FILE"
    grep -q "$LABEL: line command=3 hex=65 78 65 63 20 73 74 61 74 75 73 34 32 20 61 6c 70 68 61 20 62 65 74 61" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=3 status=handled responses=9" "$LOG_FILE"
    grep -Eq "talos: waitpid pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -Eq "talos: last-process pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos> exec init" "$LOG_FILE"
    grep -q "talos: exec path=/bin/init source=vfs-open-read" "$LOG_FILE"
    grep -Eq "talos: exec-lifecycle pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/init state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true" "$LOG_FILE"
    grep -q "talos> exec zero" "$LOG_FILE"
    grep -q "talos: exec path=/bin/zero source=vfs-open-read" "$LOG_FILE"
    grep -Eq "talos: exec-lifecycle pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/zero state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true" "$LOG_FILE"
    grep -q "talos> exec /bin/status42 gamma" "$LOG_FILE"
    grep -Eq "talos: exec-startup-abi state=literal-argv-absolute-empty-envp argc=0x0000000000000002 argv0=/bin/status42 argv1=gamma argv0-ptr=0x[0-9a-f]+ argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x[0-9a-f]+ copied-startup-bytes=0x000000000000003c source=initial-user-stack-record" "$LOG_FILE"
    grep -Eq "talos: last-process pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos> exec missing" "$LOG_FILE"
    grep -q "talos: exec-not-found" "$LOG_FILE"
    grep -q "talos> exec bin/status42" "$LOG_FILE"
    grep -q "talos: exec-invalid-path" "$LOG_FILE"
    grep -q "talos> exec /bin" "$LOG_FILE"
    grep -q "talos> exec /etc/banner.txt" "$LOG_FILE"
    grep -q "talos> exec /empty" "$LOG_FILE"
    grep -q "talos: exec-not-executable" "$LOG_FILE"
    grep -q "talos> exec /bin/status42 *" "$LOG_FILE"
    grep -q "talos> cat /etc/banner.txt" "$LOG_FILE"
    grep -q "^Talos initramfs fixture" "$LOG_FILE"
    grep -q "$LABEL: final participants=17 expected=17 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_LITERAL_ARGV_SMOKE" -eq 1 ]; then
    grep -q "talos> exec /bin/status42 alpha beta" "$LOG_FILE"
    grep -q "talos: exec path=/bin/status42 source=vfs-open-read" "$LOG_FILE"
    grep -Eq "talos: exec-source bytes=0x[0-9a-f]+ digest=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-loader fixture=phase8-program-loader-elf64-aarch64-v1 entry=0x[0-9a-f]+ segments=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-launch launch-boundary=phase8-initial-process-launch-plan-v1 stack-boundary=phase8-initial-user-stack-plan-v1 address-space=0x[0-9a-f]+ materialization=0x[0-9a-f]+ initial-sp=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-descriptors owner=0x[0-9a-f]+ inherited-count=0x0000000000000003 fd0=stdio-input fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -Eq "talos: exec-startup-abi state=literal-argv-absolute-empty-envp argc=0x0000000000000003 argv0=/bin/status42 argv1=alpha argv2=beta argv0-ptr=0x[0-9a-f]+ argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x[0-9a-f]+ copied-startup-bytes=0x0000000000000049 source=initial-user-stack-record" "$LOG_FILE"
    grep -Eq "talos: exec-lifecycle pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true" "$LOG_FILE"
    grep -q "talos: exec-status boundary=lower-aarch64-svc-status-equivalent marker=0x0000000000007a10 status=0x000000000000002a complete=true source=lifecycle-record" "$LOG_FILE"
    grep -q "$LABEL: line command=3 hex=65 78 65 63 20 2f 62 69 6e 2f 73 74 61 74 75 73 34 32 20 61 6c 70 68 61 20 62 65 74 61" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=3 status=handled responses=9" "$LOG_FILE"
    grep -Eq "talos: waitpid pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -Eq "talos: last-process pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos> exec /bin/init" "$LOG_FILE"
    grep -Eq "talos: exec-startup-abi state=minimal-argc1-argv0-init-empty-envp argc=0x0000000000000001 argv0=/bin/init argv0-ptr=0x[0-9a-f]+ argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x[0-9a-f]+ copied-startup-bytes=0x000000000000002a source=initial-user-stack-record" "$LOG_FILE"
    grep -q "talos> exec /bin/zero" "$LOG_FILE"
    grep -Eq "talos: exec-lifecycle pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/zero state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true" "$LOG_FILE"
    grep -q "talos> exec /bin/status42 *" "$LOG_FILE"
    grep -q "talos: exec-invalid-path" "$LOG_FILE"
    grep -q "talos> exec /missing" "$LOG_FILE"
    grep -q "talos: exec-not-found" "$LOG_FILE"
    grep -q "talos> exec bin/init" "$LOG_FILE"
    grep -q "talos> cat /etc/banner.txt" "$LOG_FILE"
    grep -q "^Talos initramfs fixture" "$LOG_FILE"
    grep -q "$LABEL: final participants=12 expected=12 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_VFS_EXEC_SMOKE" -eq 1 ]; then
    grep -q "talos> exec /bin/status42" "$LOG_FILE"
    grep -q "talos: exec path=/bin/status42 source=vfs-open-read" "$LOG_FILE"
    grep -Eq "talos: exec-source bytes=0x[0-9a-f]+ digest=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-loader fixture=phase8-program-loader-elf64-aarch64-v1 entry=0x[0-9a-f]+ segments=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-launch launch-boundary=phase8-initial-process-launch-plan-v1 stack-boundary=phase8-initial-user-stack-plan-v1 address-space=0x[0-9a-f]+ materialization=0x[0-9a-f]+ initial-sp=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-descriptors owner=0x[0-9a-f]+ inherited-count=0x0000000000000003 fd0=stdio-input fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -Eq "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/status42 argv0-ptr=0x[0-9a-f]+ argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x[0-9a-f]+ copied-startup-bytes=0x000000000000002e source=initial-user-stack-record" "$LOG_FILE"
    grep -Eq "talos: exec-lifecycle pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true" "$LOG_FILE"
    grep -q "talos: exec-status boundary=lower-aarch64-svc-status-equivalent marker=0x0000000000007a10 status=0x000000000000002a complete=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos: exec-signal lower-aarch64-svc-launch-boundary-equivalent" "$LOG_FILE"
    grep -q "$LABEL: line command=3 hex=65 78 65 63 20 2f 62 69 6e 2f 73 74 61 74 75 73 34 32" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=3 status=handled responses=9" "$LOG_FILE"
    grep -q "talos> laststatus" "$LOG_FILE"
    grep -Eq "talos: last-process pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "$LABEL: line command=4 hex=6c 61 73 74 73 74 61 74 75 73" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=4 status=handled responses=1" "$LOG_FILE"
    grep -q "talos> exec /bin/init" "$LOG_FILE"
    grep -q "talos: exec path=/bin/init source=vfs-open-read" "$LOG_FILE"
    grep -Eq "talos: exec-startup-abi state=minimal-argc1-argv0-init-empty-envp argc=0x0000000000000001 argv0=/bin/init argv0-ptr=0x[0-9a-f]+ argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x[0-9a-f]+ copied-startup-bytes=0x000000000000002a source=initial-user-stack-record" "$LOG_FILE"
    grep -Eq "talos: exec-lifecycle pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/init state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true" "$LOG_FILE"
    grep -q "$LABEL: line command=5 hex=65 78 65 63 20 2f 62 69 6e 2f 69 6e 69 74" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=5 status=handled responses=9" "$LOG_FILE"
    grep -Eq "talos: last-process pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/init state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "$LABEL: line command=6 hex=6c 61 73 74 73 74 61 74 75 73" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=6 status=handled responses=1" "$LOG_FILE"
    grep -q "talos> exec /bin/zero" "$LOG_FILE"
    grep -q "talos: exec path=/bin/zero source=vfs-open-read" "$LOG_FILE"
    grep -Eq "talos: exec-lifecycle pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/zero state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true" "$LOG_FILE"
    grep -q "$LABEL: line command=7 hex=65 78 65 63 20 2f 62 69 6e 2f 7a 65 72 6f" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=7 status=handled responses=9" "$LOG_FILE"
    grep -Eq "talos: last-process pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/zero state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "$LABEL: line command=8 hex=6c 61 73 74 73 74 61 74 75 73" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=8 status=handled responses=1" "$LOG_FILE"
    grep -q "talos> exec /missing" "$LOG_FILE"
    grep -q "talos: exec-not-found" "$LOG_FILE"
    grep -q "$LABEL: line command=9 hex=65 78 65 63 20 2f 6d 69 73 73 69 6e 67" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=9 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "talos> exec bin/init" "$LOG_FILE"
    grep -q "talos: exec-invalid-path" "$LOG_FILE"
    grep -q "$LABEL: line command=10 hex=65 78 65 63 20 62 69 6e 2f 69 6e 69 74" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=10 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "talos> exec /bin" "$LOG_FILE"
    grep -q "$LABEL: line command=11 hex=65 78 65 63 20 2f 62 69 6e" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=11 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "talos> exec /etc/banner.txt" "$LOG_FILE"
    grep -q "talos: exec-not-executable" "$LOG_FILE"
    grep -q "$LABEL: line command=12 hex=65 78 65 63 20 2f 65 74 63 2f 62 61 6e 6e 65 72 2e 74 78 74" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=12 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "talos> exec /empty" "$LOG_FILE"
    grep -q "$LABEL: line command=13 hex=65 78 65 63 20 2f 65 6d 70 74 79" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=13 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "talos> cat /etc/banner.txt" "$LOG_FILE"
    grep -q "^Talos initramfs fixture" "$LOG_FILE"
    grep -q "$LABEL: line command=14 hex=63 61 74 20 2f 65 74 63 2f 62 61 6e 6e 65 72 2e 74 78 74" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=14 status=handled responses=1" "$LOG_FILE"
    grep -q "$LABEL: final participants=15 expected=15 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$LS_CWD_SMOKE" -eq 1 ]; then
    grep -q "talos> pwd" "$LOG_FILE"
    grep -q "^/" "$LOG_FILE"
    grep -q "$LABEL: line command=3 hex=70 77 64" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=3 status=handled responses=1" "$LOG_FILE"
    grep -q "talos> ls" "$LOG_FILE"
    grep -q "^bin" "$LOG_FILE"
    grep -q "^dir" "$LOG_FILE"
    grep -q "^empty" "$LOG_FILE"
    grep -q "^etc" "$LOG_FILE"
    grep -q "$LABEL: line command=4 hex=6c 73" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=4 status=handled responses=4" "$LOG_FILE"
    grep -q "talos> cd /etc" "$LOG_FILE"
    grep -q "$LABEL: line command=5 hex=63 64 20 2f 65 74 63" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=5 status=handled responses=0" "$LOG_FILE"
    grep -q "^banner.txt" "$LOG_FILE"
    grep -q "$LABEL: line command=6 hex=6c 73" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=6 status=handled responses=1" "$LOG_FILE"
    grep -q "talos> cd /bin" "$LOG_FILE"
    grep -q "$LABEL: line command=7 hex=63 64 20 2f 62 69 6e" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=7 status=handled responses=0" "$LOG_FILE"
    grep -q "^init" "$LOG_FILE"
    grep -q "^zero" "$LOG_FILE"
    grep -q "^status42" "$LOG_FILE"
    grep -q "$LABEL: line command=8 hex=6c 73" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=8 status=handled responses=3" "$LOG_FILE"
    grep -q "talos> cd /" "$LOG_FILE"
    grep -q "$LABEL: line command=9 hex=63 64 20 2f" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=9 status=handled responses=0" "$LOG_FILE"
    grep -q "$LABEL: line command=10 hex=6c 73" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=10 status=handled responses=4" "$LOG_FILE"
    grep -q "talos> bogus" "$LOG_FILE"
    grep -q "talos: unknown-command" "$LOG_FILE"
    grep -q "$LABEL: line command=11 hex=62 6f 67 75 73" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=11 status=unknown-command responses=1" "$LOG_FILE"
    grep -q "$LABEL: final participants=12 expected=12 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$CD_FIXED_DIRS_SMOKE" -eq 1 ]; then
    grep -q "talos> pwd" "$LOG_FILE"
    grep -q "^/" "$LOG_FILE"
    grep -q "$LABEL: line command=3 hex=70 77 64" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=3 status=handled responses=1" "$LOG_FILE"
    grep -q "talos> cd /etc" "$LOG_FILE"
    grep -q "$LABEL: line command=4 hex=63 64 20 2f 65 74 63" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=4 status=handled responses=0" "$LOG_FILE"
    grep -q "^/etc" "$LOG_FILE"
    grep -q "$LABEL: line command=5 hex=70 77 64" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=5 status=handled responses=1" "$LOG_FILE"
    grep -q "talos> cd /bin" "$LOG_FILE"
    grep -q "$LABEL: line command=6 hex=63 64 20 2f 62 69 6e" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=6 status=handled responses=0" "$LOG_FILE"
    grep -q "^/bin" "$LOG_FILE"
    grep -q "$LABEL: line command=7 hex=70 77 64" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=7 status=handled responses=1" "$LOG_FILE"
    grep -q "talos> cd /" "$LOG_FILE"
    grep -q "$LABEL: line command=8 hex=63 64 20 2f" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=8 status=handled responses=0" "$LOG_FILE"
    grep -q "$LABEL: line command=9 hex=70 77 64" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=9 status=handled responses=1" "$LOG_FILE"
    grep -q "talos> cd /missing" "$LOG_FILE"
    grep -q "talos: not-directory" "$LOG_FILE"
    grep -q "$LABEL: line command=10 hex=63 64 20 2f 6d 69 73 73 69 6e 67" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=10 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: line command=11 hex=70 77 64" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=11 status=handled responses=1" "$LOG_FILE"
    grep -q "$LABEL: final participants=12 expected=12 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
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
elif [ "$LS_BIN_SMOKE" -eq 1 ]; then
    grep -q "talos> ls /bin" "$LOG_FILE"
    grep -q "^init" "$LOG_FILE"
    grep -q "^zero" "$LOG_FILE"
    grep -q "^status42" "$LOG_FILE"
    grep -q "$LABEL: line command=3 hex=6c 73 20 2f 62 69 6e" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=3 status=handled responses=3" "$LOG_FILE"
    grep -q "talos> ls /" "$LOG_FILE"
    grep -q "^bin" "$LOG_FILE"
    grep -q "^dir" "$LOG_FILE"
    grep -q "^empty" "$LOG_FILE"
    grep -q "^etc" "$LOG_FILE"
    grep -q "$LABEL: line command=4 hex=6c 73 20 2f" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=4 status=handled responses=4" "$LOG_FILE"
    grep -q "talos: empty-command" "$LOG_FILE"
    grep -q "$LABEL: line command=5 hex=" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=5 status=empty-command responses=1" "$LOG_FILE"
    grep -q "talos> bogus" "$LOG_FILE"
    grep -q "talos: unknown-command" "$LOG_FILE"
    grep -q "$LABEL: line command=6 hex=62 6f 67 75 73" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=6 status=unknown-command responses=1" "$LOG_FILE"
    grep -q "$LABEL: final participants=7 expected=7 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$CAT_BANNER_SMOKE" -eq 1 ]; then
    grep -q "talos> cat /etc/banner.txt" "$LOG_FILE"
    grep -q "^Talos initramfs fixture" "$LOG_FILE"
    grep -q "$LABEL: line command=3 hex=63 61 74 20 2f 65 74 63 2f 62 61 6e 6e 65 72 2e 74 78 74" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=3 status=handled responses=1" "$LOG_FILE"
    grep -q "talos> ls /bin" "$LOG_FILE"
    grep -q "^init" "$LOG_FILE"
    grep -q "^zero" "$LOG_FILE"
    grep -q "^status42" "$LOG_FILE"
    grep -q "$LABEL: line command=4 hex=6c 73 20 2f 62 69 6e" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=4 status=handled responses=3" "$LOG_FILE"
    grep -q "talos: empty-command" "$LOG_FILE"
    grep -q "$LABEL: line command=5 hex=" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=5 status=empty-command responses=1" "$LOG_FILE"
    grep -q "talos> bogus" "$LOG_FILE"
    grep -q "talos: unknown-command" "$LOG_FILE"
    grep -q "$LABEL: line command=6 hex=62 6f 67 75 73" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=6 status=unknown-command responses=1" "$LOG_FILE"
    grep -q "$LABEL: final participants=7 expected=7 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$LS_ROOT_SMOKE" -eq 1 ]; then
    grep -q "talos> ls /" "$LOG_FILE"
    grep -q "^bin" "$LOG_FILE"
    grep -q "^dir" "$LOG_FILE"
    grep -q "^empty" "$LOG_FILE"
    grep -q "^etc" "$LOG_FILE"
    grep -q "$LABEL: line command=3 hex=6c 73 20 2f" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=3 status=handled responses=4" "$LOG_FILE"
    grep -q "talos: empty-command" "$LOG_FILE"
    grep -q "$LABEL: line command=4 hex=" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=4 status=empty-command responses=1" "$LOG_FILE"
    grep -q "talos> bogus" "$LOG_FILE"
    grep -q "talos: unknown-command" "$LOG_FILE"
    grep -q "$LABEL: line command=5 hex=62 6f 67 75 73" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=5 status=unknown-command responses=1" "$LOG_FILE"
    grep -q "$LABEL: final participants=6 expected=6 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
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
