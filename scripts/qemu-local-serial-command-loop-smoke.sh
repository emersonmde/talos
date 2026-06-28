#!/usr/bin/env bash
set -euo pipefail

if [ -z "${TALOS_QEMU_LOCAL_COMMAND_LOOP_PREBUILT_ELF:-}" ]; then
    TALOS_BOOT_SCENARIO="${TALOS_QEMU_LOCAL_COMMAND_LOOP_BOOT_SCENARIO:-qemu_local_serial_command_loop}" cargo -Zjson-target-spec build "$@"
fi

ELF_FILE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_PREBUILT_ELF:-target/aarch64-talos-virt/debug/talos}"
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
SHELL_ABSOLUTE_PATH_VFS_COMMAND_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_ABSOLUTE_PATH_VFS_COMMAND_SMOKE:-0}"
SHELL_ABSOLUTE_PATH_VFS_PIPELINE_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_ABSOLUTE_PATH_VFS_PIPELINE_SMOKE:-0}"
SHELL_DIRECT_PIPELINE_STAGE_ARGV_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_DIRECT_PIPELINE_STAGE_ARGV_SMOKE:-0}"
SHELL_DIRECT_PIPELINE_STDIN_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_DIRECT_PIPELINE_STDIN_REDIRECTION_SMOKE:-0}"
SHELL_DIRECT_PIPELINE_CONSUMER_STDIN_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_DIRECT_PIPELINE_CONSUMER_STDIN_REDIRECTION_SMOKE:-0}"
SHELL_DIRECT_COMBINED_PIPELINE_STDIN_STDOUT_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_DIRECT_COMBINED_PIPELINE_STDIN_STDOUT_REDIRECTION_SMOKE:-0}"
SHELL_DIRECT_COMBINED_PIPELINE_STDOUT_APPEND_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_DIRECT_COMBINED_PIPELINE_STDOUT_APPEND_REDIRECTION_SMOKE:-0}"
SHELL_DIRECT_COMBINED_PIPELINE_STDERR_REGULAR_FILE_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_DIRECT_COMBINED_PIPELINE_STDERR_REGULAR_FILE_REDIRECTION_SMOKE:-0}"
SHELL_DIRECT_COMBINED_PIPELINE_STDERR_APPEND_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_DIRECT_COMBINED_PIPELINE_STDERR_APPEND_REDIRECTION_SMOKE:-0}"
SHELL_BARE_NAME_COMBINED_PIPELINE_STDIN_STDOUT_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_BARE_NAME_COMBINED_PIPELINE_STDIN_STDOUT_REDIRECTION_SMOKE:-0}"
SHELL_BARE_NAME_COMBINED_PIPELINE_STDOUT_APPEND_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_BARE_NAME_COMBINED_PIPELINE_STDOUT_APPEND_REDIRECTION_SMOKE:-0}"
SHELL_BARE_NAME_COMBINED_PIPELINE_STDERR_REGULAR_FILE_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_BARE_NAME_COMBINED_PIPELINE_STDERR_REGULAR_FILE_REDIRECTION_SMOKE:-0}"
SHELL_BARE_NAME_COMBINED_PIPELINE_STDERR_APPEND_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_BARE_NAME_COMBINED_PIPELINE_STDERR_APPEND_REDIRECTION_SMOKE:-0}"
SHELL_BARE_NAME_PIPELINE_STDIN_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_BARE_NAME_PIPELINE_STDIN_REDIRECTION_SMOKE:-0}"
SHELL_BARE_NAME_PIPELINE_CONSUMER_STDIN_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_BARE_NAME_PIPELINE_CONSUMER_STDIN_REDIRECTION_SMOKE:-0}"
SHELL_BARE_NAME_PIPELINE_STAGE_ARGV_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_BARE_NAME_PIPELINE_STAGE_ARGV_SMOKE:-0}"
SHELL_BARE_NAME_COMMAND_ARGV_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_BARE_NAME_COMMAND_ARGV_SMOKE:-0}"
SHELL_BARE_NAME_VFS_PIPELINE_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_BARE_NAME_VFS_PIPELINE_SMOKE:-0}"
SHELL_LITERAL_ARGV_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_LITERAL_ARGV_SMOKE:-0}"
SHELL_PATH_LOOKUP_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_PATH_LOOKUP_SMOKE:-0}"
SHELL_STDOUT_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_STDOUT_SMOKE:-0}"
SHELL_STDIN_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_STDIN_SMOKE:-0}"
SHELL_STDIN_EOF_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_STDIN_EOF_SMOKE:-0}"
SHELL_STDIN_TERMINAL_EOF_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_STDIN_TERMINAL_EOF_SMOKE:-0}"
SHELL_STDIN_READINESS_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_STDIN_READINESS_SMOKE:-0}"
SHELL_STDIN_BOUNDED_WAIT_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_STDIN_BOUNDED_WAIT_SMOKE:-0}"
SHELL_STDIN_SCHEDULER_WAIT_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_STDIN_SCHEDULER_WAIT_SMOKE:-0}"
SHELL_STDERR_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_STDERR_SMOKE:-0}"
SHELL_STDOUT_TO_STDERR_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_STDOUT_TO_STDERR_REDIRECTION_SMOKE:-0}"
SHELL_DEV_NULL_STDOUT_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_DEV_NULL_STDOUT_REDIRECTION_SMOKE:-0}"
SHELL_DEV_NULL_STDERR_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_DEV_NULL_STDERR_REDIRECTION_SMOKE:-0}"
SHELL_DEV_NULL_STDIN_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_DEV_NULL_STDIN_REDIRECTION_SMOKE:-0}"
SHELL_READONLY_REGULAR_FILE_STDIN_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_READONLY_REGULAR_FILE_STDIN_REDIRECTION_SMOKE:-0}"
SHELL_DIRECT_STDIN_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_DIRECT_STDIN_REDIRECTION_SMOKE:-0}"
SHELL_DIRECT_COMBINED_STDIN_STDOUT_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_DIRECT_COMBINED_STDIN_STDOUT_REDIRECTION_SMOKE:-0}"
SHELL_BARE_NAME_COMBINED_STDIN_STDOUT_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_BARE_NAME_COMBINED_STDIN_STDOUT_REDIRECTION_SMOKE:-0}"
SHELL_BARE_NAME_STDIN_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_BARE_NAME_STDIN_REDIRECTION_SMOKE:-0}"
SHELL_BARE_NAME_STDOUT_REGULAR_FILE_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_BARE_NAME_STDOUT_REGULAR_FILE_REDIRECTION_SMOKE:-0}"
SHELL_STDOUT_REGULAR_FILE_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_STDOUT_REGULAR_FILE_REDIRECTION_SMOKE:-0}"
SHELL_DIRECT_STDOUT_REGULAR_FILE_APPEND_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_DIRECT_STDOUT_REGULAR_FILE_APPEND_REDIRECTION_SMOKE:-0}"
SHELL_BARE_NAME_STDOUT_REGULAR_FILE_APPEND_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_BARE_NAME_STDOUT_REGULAR_FILE_APPEND_REDIRECTION_SMOKE:-0}"
SHELL_STDOUT_REGULAR_FILE_APPEND_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_STDOUT_REGULAR_FILE_APPEND_REDIRECTION_SMOKE:-0}"
SHELL_STDOUT_REGULAR_FILE_APPEND_CREATE_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_STDOUT_REGULAR_FILE_APPEND_CREATE_REDIRECTION_SMOKE:-0}"
SHELL_EXPLICIT_FD1_REGULAR_FILE_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_EXPLICIT_FD1_REGULAR_FILE_REDIRECTION_SMOKE:-0}"
SHELL_STDOUT_ARBITRARY_TMP_OUTPUT_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_STDOUT_ARBITRARY_TMP_OUTPUT_REDIRECTION_SMOKE:-0}"
SHELL_STDERR_ARBITRARY_TMP_OUTPUT_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_STDERR_ARBITRARY_TMP_OUTPUT_REDIRECTION_SMOKE:-0}"
SHELL_COMBINED_STDIN_STDOUT_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_COMBINED_STDIN_STDOUT_REDIRECTION_SMOKE:-0}"
SHELL_DIRECT_PIPELINE_OUTPUT_APPEND_REGULAR_FILE_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_DIRECT_PIPELINE_OUTPUT_APPEND_REGULAR_FILE_REDIRECTION_SMOKE:-0}"
SHELL_BARE_NAME_PIPELINE_OUTPUT_APPEND_REGULAR_FILE_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_BARE_NAME_PIPELINE_OUTPUT_APPEND_REGULAR_FILE_REDIRECTION_SMOKE:-0}"
SHELL_DIRECT_PIPELINE_STDERR_REGULAR_FILE_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_DIRECT_PIPELINE_STDERR_REGULAR_FILE_REDIRECTION_SMOKE:-0}"
SHELL_BARE_NAME_PIPELINE_STDERR_REGULAR_FILE_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_BARE_NAME_PIPELINE_STDERR_REGULAR_FILE_REDIRECTION_SMOKE:-0}"
SHELL_PIPELINE_CONSUMER_OUTPUT_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_PIPELINE_CONSUMER_OUTPUT_REDIRECTION_SMOKE:-0}"
SHELL_PIPELINE_PRODUCER_FILE_REDIRECTION_AWAY_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_PIPELINE_PRODUCER_FILE_REDIRECTION_AWAY_SMOKE:-0}"
SHELL_BACKGROUND_VFS_EXEC_LIFECYCLE_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_BACKGROUND_VFS_EXEC_LIFECYCLE_SMOKE:-0}"
SHELL_JOBS_ACCOUNTING_LIST_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_JOBS_ACCOUNTING_LIST_SMOKE:-0}"
SHELL_MULTIPLE_BACKGROUND_JOBS_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_MULTIPLE_BACKGROUND_JOBS_SMOKE:-0}"
SHELL_BACKGROUND_JOBS_STALE_ENTRY_POLICY_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_BACKGROUND_JOBS_STALE_ENTRY_POLICY_SMOKE:-0}"
SHELL_WAITPID_ANY_COMPLETED_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_WAITPID_ANY_COMPLETED_SMOKE:-0}"
SHELL_PROCESS_STATUS_VFS_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_PROCESS_STATUS_VFS_SMOKE:-0}"
SHELL_PS_COMMAND_VFS_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_PS_COMMAND_VFS_SMOKE:-0}"
SHELL_GENERATED_USERLAND_MANIFEST_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_GENERATED_USERLAND_MANIFEST_SMOKE:-0}"
GENERATED_ROOT_ARTIFACT="${TALOS_QEMU_LOCAL_COMMAND_LOOP_GENERATED_ROOT_ARTIFACT:-}"
GENERATED_ROOT_EXPECTED_SOURCE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_GENERATED_ROOT_EXPECTED_SOURCE:-compiled-fallback}"
GENERATED_ROOT_EXPECTED_REASON="${TALOS_QEMU_LOCAL_COMMAND_LOOP_GENERATED_ROOT_EXPECTED_REASON:-missing-artifact}"
GENERATED_ROOT_EXPECTED_CONTENT="${TALOS_QEMU_LOCAL_COMMAND_LOOP_GENERATED_ROOT_EXPECTED_CONTENT:-Talos generated-root manifest fixture}"
GENERATED_ROOT_EXPECTED_STATUS_HEX="${TALOS_QEMU_LOCAL_COMMAND_LOOP_GENERATED_ROOT_EXPECTED_STATUS_HEX:-0x0000000000000007}"
SHELL_STDERR_REGULAR_FILE_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_STDERR_REGULAR_FILE_REDIRECTION_SMOKE:-0}"
SHELL_DIRECT_STDERR_REGULAR_FILE_APPEND_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_DIRECT_STDERR_REGULAR_FILE_APPEND_REDIRECTION_SMOKE:-0}"
SHELL_BARE_NAME_STDERR_REGULAR_FILE_APPEND_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_BARE_NAME_STDERR_REGULAR_FILE_APPEND_REDIRECTION_SMOKE:-0}"
SHELL_STDERR_REGULAR_FILE_APPEND_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_STDERR_REGULAR_FILE_APPEND_REDIRECTION_SMOKE:-0}"
SHELL_STDERR_REGULAR_FILE_APPEND_CREATE_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_STDERR_REGULAR_FILE_APPEND_CREATE_REDIRECTION_SMOKE:-0}"
SHELL_STDERR_TO_STDOUT_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_STDERR_TO_STDOUT_REDIRECTION_SMOKE:-0}"
SHELL_STDOUT_CLOSE_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_STDOUT_CLOSE_REDIRECTION_SMOKE:-0}"
SHELL_STDERR_CLOSE_REDIRECTION_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_STDERR_CLOSE_REDIRECTION_SMOKE:-0}"
SHELL_MINIMAL_STDOUT_TO_STDIN_PIPELINE_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_MINIMAL_STDOUT_TO_STDIN_PIPELINE_SMOKE:-0}"
SHELL_MULTISTAGE_PIPELINE_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_MULTISTAGE_PIPELINE_SMOKE:-0}"
SHELL_PIPELINE_STATUS_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_PIPELINE_STATUS_SMOKE:-0}"
SHELL_PIPELINE_STDERR_NOT_PIPED_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_PIPELINE_STDERR_NOT_PIPED_SMOKE:-0}"
SHELL_PIPELINE_STDERR_DUP_TO_STDOUT_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_PIPELINE_STDERR_DUP_TO_STDOUT_SMOKE:-0}"
SHELL_PIPELINE_STDOUT_REDIRECT_AWAY_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_PIPELINE_STDOUT_REDIRECT_AWAY_SMOKE:-0}"
SHELL_WAITPID_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_WAITPID_SMOKE:-0}"
CD_FIXED_DIRS_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_CD_FIXED_DIRS_SMOKE:-0}"
LS_CWD_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_LS_CWD_SMOKE:-0}"
LINE_EDITING_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_LINE_EDITING_SMOKE:-0}"
LINE_CANCEL_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_LINE_CANCEL_SMOKE:-0}"
LINE_KILL_SMOKE="${TALOS_QEMU_LOCAL_COMMAND_LOOP_LINE_KILL_SMOKE:-0}"
SHELL_STDIO_SMOKE=0
if [ "$SHELL_STDOUT_SMOKE" -eq 1 ] || [ "$SHELL_STDIN_SMOKE" -eq 1 ] || [ "$SHELL_STDIN_EOF_SMOKE" -eq 1 ] || [ "$SHELL_STDIN_TERMINAL_EOF_SMOKE" -eq 1 ] || [ "$SHELL_STDIN_READINESS_SMOKE" -eq 1 ] || [ "$SHELL_STDIN_BOUNDED_WAIT_SMOKE" -eq 1 ] || [ "$SHELL_STDIN_SCHEDULER_WAIT_SMOKE" -eq 1 ] || [ "$SHELL_STDERR_SMOKE" -eq 1 ]; then
    SHELL_STDIO_SMOKE=1
fi

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
. "$script_dir/objcopy-tool.sh"
. "$script_dir/qemu-tool.sh"

"$objcopy_tool" -O binary "$ELF_FILE" "$IMG_FILE"
: >"$LOG_FILE"
: >"$QEMU_LOG_FILE"

qemu_generated_root_loader_args=()
if [ -n "$GENERATED_ROOT_ARTIFACT" ]; then
    qemu_generated_root_loader_args=(-device "loader,file=$GENERATED_ROOT_ARTIFACT,addr=0x47000000")
fi

"$qemu_tool" \
    -M virt,gic-version=2,virtualization=on \
    -cpu cortex-a76 \
    -m 256M \
    -display none \
    -monitor none \
    -chardev socket,id=serial0,host=127.0.0.1,port="$PORT",server=on,wait=on \
    -serial chardev:serial0 \
    -semihosting-config enable=on,target=native \
    "${qemu_generated_root_loader_args[@]}" \
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
delayed_stdin_sent=0
while kill -0 "$qemu_pid" 2>/dev/null; do
    if IFS= read -r -t 0.2 line <&3; then
        printf '%s\n' "$line" >>"$LOG_FILE"
        if [ "$SHELL_COMBINED_STDIN_STDOUT_REDIRECTION_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            combined_redirection_commands=(
                "help"
                "status"
                "stdio"
                "exec stdin </etc/banner.txt >/tmp/stdin-report.txt"
                "waitpid"
                "laststatus"
                "cat /tmp/stdin-report.txt"
                "exec stdin >/tmp/stdin-report.txt </etc/banner.txt"
                "exec stdin </dev/null >/tmp/stdin-report.txt"
                "exec stdin </etc/banner.txt 1>/tmp/stdin-report.txt"
                "exec stdin < /etc/banner.txt >/tmp/stdin-report.txt"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#combined_redirection_commands[@]}" ]; then
                printf '%s\r' "${combined_redirection_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if [ "$SHELL_PIPELINE_CONSUMER_OUTPUT_REDIRECTION_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            pipeline_consumer_output_commands=(
                "help"
                "status"
                "stdio"
                "exec stdout | exec stdin >/tmp/pipe-consumer.txt"
                "waitpid"
                "laststatus"
                "cat /tmp/pipe-consumer.txt"
                "exec stdout | exec stdin"
                "exec stdout | exec stdin >>/tmp/pipe-consumer.txt"
                "exec stderr | exec stdin >/tmp/pipe-consumer.txt"
                "exec stdout >/tmp/src.txt | exec stdin >/tmp/out.txt"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#pipeline_consumer_output_commands[@]}" ]; then
                printf '%s\r' "${pipeline_consumer_output_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if [ "$SHELL_DIRECT_PIPELINE_OUTPUT_APPEND_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            direct_pipeline_output_append_commands=(
                "help"
                "status"
                "stdio"
                "/bin/stdout | /bin/stdin >/tmp/pipeline-report.txt"
                "/bin/stdout | /bin/stdin >>/tmp/pipeline-report.txt"
                "waitpid 0x100001"
                "waitpid 0x100002"
                "laststatus"
                "pipestatus"
                "cat /proc/talos/processes"
                "ps"
                "cat /tmp/pipeline-report.txt"
                "/bin/stdout"
                "/bin/stdout | /bin/stdin >/tmp/stdout.txt"
                "stdout | stdin >>/tmp/pipeline-report.txt"
                "/bin/stdout | /bin/stderr >>/tmp/pipeline-report.txt"
                "/bin/stdout | /bin/stdin 2>>/tmp/x"
                "/bin/stdout | /bin/stdin </etc/banner.txt"
                "/bin/stdout | /bin/stdin >> /tmp/pipeline-report.txt"
                "/bin/stdout | /bin/stdin >>/var/x"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#direct_pipeline_output_append_commands[@]}" ]; then
                printf '%s\r' "${direct_pipeline_output_append_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if [ "$SHELL_DIRECT_PIPELINE_STDERR_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            direct_pipeline_stderr_commands=(
                "help"
                "status"
                "stdio"
                "/bin/stdout | /bin/stderr 2>/tmp/pipeline-stderr.txt"
                "/bin/stdout | /bin/stderr 2>>/tmp/pipeline-stderr.txt"
                "waitpid 0x100001"
                "waitpid 0x100002"
                "laststatus"
                "pipestatus"
                "cat /proc/talos/processes"
                "ps"
                "cat /tmp/pipeline-stderr.txt"
                "/bin/stderr"
                "/bin/stdout | /bin/stderr >/tmp/pipeline-stderr.txt"
                "/bin/stdout | /bin/stderr </etc/banner.txt"
                "/bin/stdout | /bin/stderr 2>>/tmp/stderr.txt"
                "/bin/stdout | /bin/stderr 2> /tmp/pipeline-stderr.txt"
                "/bin/stdout | /bin/stderr 2>>/var/x"
                "stdout | stderr 2>>/tmp/stderr.txt"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#direct_pipeline_stderr_commands[@]}" ]; then
                printf '%s\r' "${direct_pipeline_stderr_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if [ "$SHELL_BARE_NAME_PIPELINE_STDERR_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            bare_name_pipeline_stderr_commands=(
                "help"
                "status"
                "stdio"
                "stdout | stderr 2>/tmp/pipeline-stderr.txt"
                "stdout | stderr 2>>/tmp/pipeline-stderr.txt"
                "waitpid 0x100001"
                "waitpid 0x100002"
                "laststatus"
                "pipestatus"
                "cat /proc/talos/processes"
                "ps"
                "cat /tmp/pipeline-stderr.txt"
                "stderr"
                "/bin/stdout | /bin/stderr 2>>/tmp/pipeline-stderr.txt"
                "stdout | stderr >/tmp/pipeline-stderr.txt"
                "stdout | stderr </etc/banner.txt"
                "stdout | stderr 2>>/tmp/stderr.txt"
                "stdout | stderr 2> /tmp/pipeline-stderr.txt"
                "stdout | stderr 2>>/var/x"
                "stdout | bin/stderr 2>/tmp/pipeline-stderr.txt"
                "/bin/stdout | stderr 2>/tmp/pipeline-stderr.txt"
                "nosuch | stderr 2>/tmp/pipeline-stderr.txt"
                "stdout | nosuch 2>/tmp/pipeline-stderr.txt"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#bare_name_pipeline_stderr_commands[@]}" ]; then
                printf '%s\r' "${bare_name_pipeline_stderr_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if [ "$SHELL_BARE_NAME_PIPELINE_OUTPUT_APPEND_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            bare_name_pipeline_output_append_commands=(
                "help"
                "status"
                "stdio"
                "stdout | stdin >/tmp/pipeline-report.txt"
                "stdout | stdin >>/tmp/pipeline-report.txt"
                "waitpid 0x100001"
                "waitpid 0x100002"
                "laststatus"
                "pipestatus"
                "cat /proc/talos/processes"
                "ps"
                "cat /tmp/pipeline-report.txt"
                "stdout"
                "/bin/stdout | /bin/stdin >>/tmp/pipeline-report.txt"
                "stdout | stdin >/tmp/stdout.txt"
                "stdout | stderr >/tmp/pipeline-report.txt"
                "stdout | stdin 1>/tmp/pipeline-report.txt"
                "stdout | stdin > /tmp/pipeline-report.txt"
                "stdout | bin/stdin >/tmp/pipeline-report.txt"
                "stdout | stdin 2>>/tmp/x"
                "stdout | stdin </etc/banner.txt"
                "stdout | stdin >> /tmp/pipeline-report.txt"
                "stdout | stdin >>/var/x"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#bare_name_pipeline_output_append_commands[@]}" ]; then
                printf '%s\r' "${bare_name_pipeline_output_append_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if [ "$SHELL_PIPELINE_PRODUCER_FILE_REDIRECTION_AWAY_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            pipeline_producer_file_redirection_away_commands=(
                "help"
                "status"
                "stdio"
                "exec stdout >/tmp/pipe-source.txt | exec stdin"
                "waitpid"
                "laststatus"
                "cat /tmp/pipe-source.txt"
                "exec stdout | exec stdin"
                "exec stdout >>/tmp/pipe-source.txt | exec stdin"
                "exec stderr >/tmp/pipe-source.txt | exec stdin"
                "exec stdout >/tmp/src.txt | exec stdin >/tmp/out.txt"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#pipeline_producer_file_redirection_away_commands[@]}" ]; then
                printf '%s\r' "${pipeline_producer_file_redirection_away_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if [ "$SHELL_MULTISTAGE_PIPELINE_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            multistage_pipeline_commands=(
                "help"
                "status"
                "stdio"
                "exec stdout | exec stdin | exec stdin"
                "waitpid 0x100001"
                "waitpid 0x100002"
                "waitpid 0x100003"
                "cat /proc/talos/processes"
                "ps"
                "cat /etc/banner.txt"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#multistage_pipeline_commands[@]}" ]; then
                printf '%s\r' "${multistage_pipeline_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if [ "$SHELL_PIPELINE_STATUS_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            pipeline_status_commands=(
                "help"
                "status"
                "stdio"
                "pipestatus"
                "exec stdout | exec stdin"
                "pipestatus"
                "laststatus"
                "exec status42 | exec stdin"
                "pipestatus"
                "laststatus"
                "exec stdout | exec stdin | exec stdin"
                "pipestatus"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#pipeline_status_commands[@]}" ]; then
                printf '%s\r' "${pipeline_status_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if [ "$SHELL_BACKGROUND_VFS_EXEC_LIFECYCLE_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            background_vfs_exec_lifecycle_commands=(
                "help"
                "status"
                "stdio"
                "exec /bin/status42 &"
                "cat /etc/banner.txt"
                "waitpid"
                "laststatus"
                "exec /bin/zero"
                "waitpid"
                "laststatus"
                "exec stdout | exec stdin"
                "cat /etc/banner.txt"
                "exec /bin/status42&"
                "exec stdout &"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#background_vfs_exec_lifecycle_commands[@]}" ]; then
                printf '%s\r' "${background_vfs_exec_lifecycle_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if [ "$SHELL_JOBS_ACCOUNTING_LIST_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            jobs_accounting_list_commands=(
                "help"
                "status"
                "stdio"
                "jobs"
                "exec /bin/status42 &"
                "jobs"
                "jobs"
                "waitpid"
                "laststatus"
                "exec /bin/zero"
                "waitpid"
                "laststatus"
                "exec stdout | exec stdin"
                "cat /etc/banner.txt"
                "fg"
                "bg"
                "kill %1"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#jobs_accounting_list_commands[@]}" ]; then
                printf '%s\r' "${jobs_accounting_list_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if [ "$SHELL_MULTIPLE_BACKGROUND_JOBS_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            multiple_background_jobs_commands=(
                "help"
                "status"
                "stdio"
                "jobs"
                "exec /bin/status42 &"
                "exec /bin/zero &"
                "jobs"
                "jobs"
                "waitpid"
                "laststatus"
                "exec /bin/zero"
                "waitpid"
                "laststatus"
                "exec /bin/status42&"
                "exec stdout &"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#multiple_background_jobs_commands[@]}" ]; then
                printf '%s\r' "${multiple_background_jobs_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if [ "$SHELL_BACKGROUND_JOBS_STALE_ENTRY_POLICY_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            background_jobs_stale_entry_policy_commands=(
                "help"
                "status"
                "stdio"
                "jobs"
                "exec /bin/status42 &"
                "exec /bin/zero &"
                "jobs"
                "jobs"
                "jobs"
                "waitpid"
                "laststatus"
                "exec /bin/zero"
                "waitpid"
                "laststatus"
                "exec /bin/status42&"
                "exec stdout &"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#background_jobs_stale_entry_policy_commands[@]}" ]; then
                printf '%s\r' "${background_jobs_stale_entry_policy_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if [ "$SHELL_GENERATED_USERLAND_MANIFEST_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            generated_userland_manifest_commands=(
                "help"
                "status"
                "stdio"
                "cat /generated/manifest.txt"
                "ls /"
                "cat /etc/banner.txt"
                "exec /generated/status7 alpha"
                "waitpid"
                "laststatus"
                "exec /bin/status42"
                "waitpid"
                "laststatus"
                "exec stdout | exec stdin"
                "jobs"
                "rootinfo"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#generated_userland_manifest_commands[@]}" ]; then
                printf '%s\r' "${generated_userland_manifest_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if [ "$SHELL_WAITPID_ANY_COMPLETED_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            waitpid_any_completed_commands=(
                "help"
                "status"
                "stdio"
                "waitpid"
                "exec /bin/status42"
                "waitpid"
                "waitpid"
                "laststatus"
                "exec stdout | exec stdin"
                "waitpid"
                "waitpid 0x100001"
                "waitpid 0x100002"
                "exec /bin/status42 &"
                "waitpid"
                "jobs"
                "exec /bin/zero &"
                "cat /etc/banner.txt"
                "jobs"
                "jobs"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#waitpid_any_completed_commands[@]}" ]; then
                printf '%s\r' "${waitpid_any_completed_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if [ "$SHELL_PROCESS_STATUS_VFS_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            process_status_vfs_commands=(
                "help"
                "status"
                "stdio"
                "exec /bin/status42"
                "cat /proc/talos/processes"
                "waitpid"
                "cat /proc/talos/processes"
                "exec stdout | exec stdin"
                "cat /proc/talos/processes"
                "waitpid 0x100001"
                "waitpid 0x100002"
                "cat /proc/talos/processes"
                "exec /bin/status42 &"
                "cat /proc/talos/processes"
                "waitpid 0x100001"
                "cat /proc/talos/processes"
                "cat /proc/talos"
                "cat /etc/banner.txt"
                "jobs"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#process_status_vfs_commands[@]}" ]; then
                printf '%s\r' "${process_status_vfs_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if [ "$SHELL_PS_COMMAND_VFS_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            ps_command_vfs_commands=(
                "help"
                "status"
                "stdio"
                "exec /bin/status42"
                "cat /proc/talos/processes"
                "ps"
                "waitpid"
                "ps"
                "exec stdout | exec stdin"
                "cat /proc/talos/processes"
                "ps"
                "waitpid 0x100001"
                "waitpid 0x100002"
                "ps"
                "exec /bin/status42 &"
                "cat /proc/talos/processes"
                "ps"
                "waitpid 0x100001"
                "ps"
                "ps -a"
                "ps extra"
                "cat /etc/banner.txt"
                "jobs"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#ps_command_vfs_commands[@]}" ]; then
                printf '%s\r' "${ps_command_vfs_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if [ "$SHELL_STDOUT_ARBITRARY_TMP_OUTPUT_REDIRECTION_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            arbitrary_tmp_commands=(
                "help"
                "status"
                "stdio"
                "exec stdout >/tmp/alpha.log"
                "waitpid"
                "laststatus"
                "cat /tmp/alpha.log"
                "exec stdout >>/tmp/beta.out"
                "waitpid"
                "laststatus"
                "cat /tmp/beta.out"
                "exec stdout 1>/tmp/gamma.log"
                "waitpid"
                "laststatus"
                "cat /tmp/gamma.log"
                "exec stdout 1>>/tmp/delta.out"
                "waitpid"
                "laststatus"
                "cat /tmp/delta.out"
                "exec stdout"
                "exec stdout >/var/out.txt"
                "exec stdout >/tmp/nested/out.txt"
                "exec stdout >/tmp/"
                "exec stdout 3>/tmp/alpha.log"
                "exec stdout >/tmp/../bad.txt"
                "exec stdout >/tmp/stderr.txt"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#arbitrary_tmp_commands[@]}" ]; then
                printf '%s\r' "${arbitrary_tmp_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if [ "$SHELL_STDERR_ARBITRARY_TMP_OUTPUT_REDIRECTION_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            stderr_arbitrary_tmp_commands=(
                "help"
                "status"
                "stdio"
                "exec stderr 2>/tmp/omega.err"
                "waitpid"
                "laststatus"
                "cat /tmp/omega.err"
                "exec stderr 2>>/tmp/theta.log"
                "waitpid"
                "laststatus"
                "cat /tmp/theta.log"
                "exec stderr"
                "exec stdout"
                "exec stderr 2>/var/err.txt"
                "exec stderr 2>/tmp/n/e"
                "exec stderr 2>/tmp/"
                "exec stderr 3>/tmp/omega.err"
                "exec stderr 2>/tmp/../bad.txt"
                "exec stderr 2>/tmp/stdout.txt"
                "exec stderr >/tmp/misbound.err"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#stderr_arbitrary_tmp_commands[@]}" ]; then
                printf '%s\r' "${stderr_arbitrary_tmp_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if { [ "$SHELL_STDIN_BOUNDED_WAIT_SMOKE" -eq 1 ] || [ "$SHELL_STDIN_SCHEDULER_WAIT_SMOKE" -eq 1 ]; } && [ "$delayed_stdin_sent" -eq 0 ]; then
            case "$line" in
                *"talos: stdin-wait "*" result=sleep source=scheduler-runtime-console-readiness"*)
                    printf 'talos-console0' >&3
                    delayed_stdin_sent=1
                    ;;
            esac
        fi
        if [ "$SHELL_ABSOLUTE_PATH_VFS_PIPELINE_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            absolute_path_vfs_pipeline_commands=(
                "help"
                "status"
                "stdio"
                "/bin/stdout | /bin/stdin"
                "waitpid 0x100001"
                "waitpid 0x100002"
                "laststatus"
                "pipestatus"
                "cat /proc/talos/processes"
                "ps"
                "/bin/stdout | exec stdin"
                "exec stdout | /bin/stdin"
                "status42 | /bin/stdin"
                "/bin/stdout | /missing"
                "/bin/stdout | /bin/stdin | /bin/stdin"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#absolute_path_vfs_pipeline_commands[@]}" ]; then
                printf '%s\r' "${absolute_path_vfs_pipeline_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if [ "$SHELL_DIRECT_PIPELINE_STAGE_ARGV_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            direct_pipeline_stage_argv_commands=(
                "help"
                "status"
                "stdio"
                "/bin/stdout alpha | /bin/stdin beta"
                "waitpid 0x100001"
                "waitpid 0x100002"
                "laststatus"
                "pipestatus"
                "cat /proc/talos/processes"
                "ps"
                "/bin/stdout alpha beta | /bin/stdin"
                "/bin/stdout * | /bin/stdin"
                "/bin/stdout alpha | /bin/stdin beta gamma"
                "/bin/stdout alpha | /bin/stdin *"
                "/bin/stdout | /bin/stdin"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#direct_pipeline_stage_argv_commands[@]}" ]; then
                printf '%s\r' "${direct_pipeline_stage_argv_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if [ "$SHELL_DIRECT_PIPELINE_STDIN_REDIRECTION_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            direct_pipeline_stdin_redirection_commands=(
                "help"
                "status"
                "stdio"
                "/bin/stdin </etc/banner.txt | /bin/stdin"
                "waitpid 0x100001"
                "waitpid 0x100002"
                "laststatus"
                "pipestatus"
                "cat /proc/talos/processes"
                "ps"
                "/bin/stdin </etc/banner.txt | /bin/stdout"
                "/bin/stdin alpha </etc/banner.txt | /bin/stdin"
                "/bin/stdin </etc/banner.txt | /bin/stdin beta"
                "/bin/stdin </dev/null | /bin/stdin"
                "/bin/stdin < /etc/banner.txt | /bin/stdin"
                "/bin/stdin </etc/banner.txt | /bin/stdin | /bin/stdin"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#direct_pipeline_stdin_redirection_commands[@]}" ]; then
                printf '%s\r' "${direct_pipeline_stdin_redirection_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if [ "$SHELL_DIRECT_PIPELINE_CONSUMER_STDIN_REDIRECTION_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            direct_pipeline_consumer_stdin_redirection_commands=(
                "help"
                "status"
                "stdio"
                "/bin/stdin | /bin/stdin </etc/banner.txt"
                "waitpid 0x100001"
                "waitpid 0x100002"
                "laststatus"
                "pipestatus"
                "cat /proc/talos/processes"
                "ps"
                "/bin/stdout | /bin/stdin </etc/banner.txt"
                "/bin/stdin alpha | /bin/stdin </etc/banner.txt"
                "/bin/stdin | /bin/stdin beta </etc/banner.txt"
                "/bin/stdin | /bin/stdin </dev/null"
                "/bin/stdin | /bin/stdin < /etc/banner.txt"
                "/bin/stdin | /bin/stdin </etc/banner.txt | /bin/stdin"
                "/bin/stdin </etc/banner.txt | /bin/stdin </etc/banner.txt"
                "/bin/stdin </etc/banner.txt | stdin </etc/banner.txt"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#direct_pipeline_consumer_stdin_redirection_commands[@]}" ]; then
                printf '%s\r' "${direct_pipeline_consumer_stdin_redirection_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if [ "$SHELL_DIRECT_COMBINED_PIPELINE_STDIN_STDOUT_REDIRECTION_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            direct_combined_pipeline_stdin_stdout_redirection_commands=(
                "help"
                "status"
                "stdio"
                "/bin/stdin </etc/banner.txt | /bin/stdin >/tmp/pipeline-combined.txt"
                "waitpid 0x100001"
                "waitpid 0x100002"
                "laststatus"
                "pipestatus"
                "cat /proc/talos/processes"
                "ps"
                "cat /tmp/pipeline-combined.txt"
                "/bin/stdin </etc/banner.txt | /bin/stdin >>/tmp/pipeline-combined.txt"
                "/bin/stdin </etc/banner.txt | /bin/stdin >/tmp/pipeline-report.txt"
                "/bin/stdout | /bin/stdin >/tmp/pipeline-combined.txt"
                "/bin/stdin </etc/banner.txt | /bin/stdin 1>/tmp/pipeline-combined.txt"
                "/bin/stdin </etc/banner.txt | /bin/stdin > /tmp/pipeline-combined.txt"
                "/bin/stdin </etc/banner.txt | /bin/stdin >/var/x"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#direct_combined_pipeline_stdin_stdout_redirection_commands[@]}" ]; then
                printf '%s\r' "${direct_combined_pipeline_stdin_stdout_redirection_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if [ "$SHELL_DIRECT_COMBINED_PIPELINE_STDOUT_APPEND_REDIRECTION_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            direct_combined_pipeline_stdout_append_redirection_commands=(
                "help"
                "status"
                "stdio"
                "/bin/stdin </etc/banner.txt | /bin/stdin >/tmp/pipeline-combined-append.txt"
                "/bin/stdin </etc/banner.txt | /bin/stdin >>/tmp/pipeline-combined-append.txt"
                "waitpid 0x100001"
                "waitpid 0x100002"
                "laststatus"
                "pipestatus"
                "cat /proc/talos/processes"
                "ps"
                "cat /tmp/pipeline-combined-append.txt"
                "/bin/stdin </etc/banner.txt"
                "stdin </etc/banner.txt | /bin/stdin >>/tmp/pipeline-combined-append.txt"
                "/bin/stdin </etc/banner.txt | /bin/stdin 2>>/tmp/pipeline-combined-append.txt"
                "/bin/stdin </etc/banner.txt | /bin/stdin >/tmp/pipeline-report.txt"
                "/bin/stdout | /bin/stdin >>/tmp/pipeline-combined-append.txt"
                "/bin/stdin </etc/banner.txt | /bin/stdin 1>/tmp/pipeline-combined-append.txt"
                "/bin/stdin </etc/banner.txt | /bin/stdin > /tmp/pipeline-combined-append.txt"
                "/bin/stdin </etc/banner.txt | /bin/stdin >>/var/x"
                "/bin/stdin </etc/banner.txt | /bin/stdin >>/tmp/pipeline-combined-append.txt | /bin/stdin"
                "missing </etc/banner.txt | /bin/stdin >>/tmp/pipeline-combined-append.txt"
                "/bin/stdin </etc/banner.txt | missing >>/tmp/pipeline-combined-append.txt"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#direct_combined_pipeline_stdout_append_redirection_commands[@]}" ]; then
                printf '%s\r' "${direct_combined_pipeline_stdout_append_redirection_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if [ "$SHELL_DIRECT_COMBINED_PIPELINE_STDERR_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            direct_combined_pipeline_stderr_regular_file_redirection_commands=(
                "help"
                "status"
                "stdio"
                "/bin/stdin </etc/banner.txt | /bin/stderr 2>/tmp/pipeline-combined-stderr.txt"
                "waitpid 0x100001"
                "waitpid 0x100002"
                "laststatus"
                "pipestatus"
                "cat /proc/talos/processes"
                "ps"
                "cat /tmp/pipeline-combined-stderr.txt"
                "/bin/stdin </etc/banner.txt"
                "/bin/stdout"
                "/bin/stderr"
                "stdin </etc/banner.txt | /bin/stderr 2>/tmp/pipeline-combined-stderr.txt"
                "/bin/stdin </etc/banner.txt | /bin/stderr 2>>/tmp/pipeline-combined-stderr.txt"
                "/bin/stdin </etc/banner.txt | /bin/stderr 2>/tmp/pipeline-stderr.txt"
                "/bin/stdout | /bin/stderr 2>/tmp/pipeline-combined-stderr.txt"
                "/bin/stdin </etc/banner.txt | /bin/stderr 2> /tmp/pipeline-combined-stderr.txt"
                "/bin/stdin </etc/banner.txt | /bin/stderr 2>/var/x"
                "/bin/stdin </etc/banner.txt | /bin/stderr 2>/tmp/pipeline-combined-stderr.txt | /bin/stdin"
                "missing </etc/banner.txt | /bin/stderr 2>/tmp/pipeline-combined-stderr.txt"
                "/bin/stdin </etc/banner.txt | missing 2>/tmp/pipeline-combined-stderr.txt"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#direct_combined_pipeline_stderr_regular_file_redirection_commands[@]}" ]; then
                printf '%s\r' "${direct_combined_pipeline_stderr_regular_file_redirection_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if [ "$SHELL_DIRECT_COMBINED_PIPELINE_STDERR_APPEND_REDIRECTION_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            direct_combined_pipeline_stderr_append_redirection_commands=(
                "help"
                "status"
                "stdio"
                "/bin/stdin </etc/banner.txt | /bin/stderr 2>/tmp/pipeline-combined-stderr-append.txt"
                "/bin/stdin </etc/banner.txt | /bin/stderr 2>>/tmp/pipeline-combined-stderr-append.txt"
                "waitpid 0x100001"
                "waitpid 0x100002"
                "laststatus"
                "pipestatus"
                "cat /proc/talos/processes"
                "ps"
                "cat /tmp/pipeline-combined-stderr-append.txt"
                "/bin/stdin </etc/banner.txt"
                "/bin/stderr"
                "stdin </etc/banner.txt | /bin/stderr 2>>/tmp/pipeline-combined-stderr-append.txt"
                "/bin/stdin </etc/banner.txt | /bin/stderr 2>>/tmp/pipeline-combined-stderr.txt"
                "/bin/stdin </etc/banner.txt | /bin/stderr 2>/tmp/pipeline-stderr.txt"
                "/bin/stdout | /bin/stderr 2>>/tmp/pipeline-combined-stderr-append.txt"
                "/bin/stdin </etc/banner.txt | /bin/stderr 2>> /tmp/pipeline-combined-stderr-append.txt"
                "/bin/stdin </etc/banner.txt | /bin/stderr 1>/tmp/pipeline-combined-stderr-append.txt"
                "/bin/stdin </etc/banner.txt | /bin/stderr 2>>/var/x"
                "/bin/stdin </etc/banner.txt | /bin/stderr 2>>/tmp/pipeline-combined-stderr-append.txt | /bin/stdin"
                "missing </etc/banner.txt | /bin/stderr 2>>/tmp/pipeline-combined-stderr-append.txt"
                "/bin/stdin </etc/banner.txt | missing 2>>/tmp/pipeline-combined-stderr-append.txt"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#direct_combined_pipeline_stderr_append_redirection_commands[@]}" ]; then
                printf '%s\r' "${direct_combined_pipeline_stderr_append_redirection_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if [ "$SHELL_BARE_NAME_COMBINED_PIPELINE_STDOUT_APPEND_REDIRECTION_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            bare_name_combined_pipeline_stdout_append_redirection_commands=(
                "help"
                "status"
                "stdio"
                "stdin </etc/banner.txt | stdin >/tmp/pipeline-combined-append.txt"
                "stdin </etc/banner.txt | stdin >>/tmp/pipeline-combined-append.txt"
                "waitpid 0x100001"
                "waitpid 0x100002"
                "laststatus"
                "pipestatus"
                "cat /proc/talos/processes"
                "ps"
                "cat /tmp/pipeline-combined-append.txt"
                "stdin </etc/banner.txt"
                "/bin/stdin </etc/banner.txt | /bin/stdin >/tmp/pipeline-combined-append.txt"
                "missing </etc/banner.txt | stdin >>/tmp/pipeline-combined-append.txt"
                "stdin </etc/banner.txt | missing >>/tmp/pipeline-combined-append.txt"
                "stdin </etc/banner.txt | stdin 2>>/tmp/pipeline-combined-append.txt"
                "stdin </etc/banner.txt | stdin >/tmp/pipeline-report.txt"
                "stdin </etc/banner.txt | stdin 1>/tmp/pipeline-combined-append.txt"
                "stdin </etc/banner.txt | stdin > /tmp/pipeline-combined-append.txt"
                "stdin </etc/banner.txt | stdin >>/var/x"
                "stdin </etc/banner.txt | /bin/stdin >>/tmp/pipeline-combined-append.txt"
                "/bin/stdin </etc/banner.txt | stdin >>/tmp/pipeline-combined-append.txt"
                "stdin </etc/banner.txt | stdin >>/tmp/pipeline-combined-append.txt | stdin"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#bare_name_combined_pipeline_stdout_append_redirection_commands[@]}" ]; then
                printf '%s\r' "${bare_name_combined_pipeline_stdout_append_redirection_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if [ "$SHELL_BARE_NAME_COMBINED_PIPELINE_STDERR_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            bare_name_combined_pipeline_stderr_regular_file_redirection_commands=(
                "help"
                "status"
                "stdio"
                "stdin </etc/banner.txt | stderr 2>/tmp/pipeline-combined-stderr.txt"
                "waitpid 0x100001"
                "waitpid 0x100002"
                "laststatus"
                "pipestatus"
                "cat /proc/talos/processes"
                "ps"
                "cat /tmp/pipeline-combined-stderr.txt"
                "stdin </etc/banner.txt"
                "/bin/stdin </etc/banner.txt | /bin/stderr 2>/tmp/pipeline-combined-stderr.txt"
                "/bin/stderr"
                "missing </etc/banner.txt | stderr 2>/tmp/pipeline-combined-stderr.txt"
                "stdin </etc/banner.txt | missing 2>/tmp/pipeline-combined-stderr.txt"
                "stdin </etc/banner.txt | stderr 2>>/tmp/pipeline-combined-stderr.txt"
                "stdin </etc/banner.txt | stderr 2>/tmp/pipeline-stderr.txt"
                "stdout | stderr 2>/tmp/pipeline-combined-stderr.txt"
                "stdin </etc/banner.txt | stderr 2> /tmp/pipeline-combined-stderr.txt"
                "stdin </etc/banner.txt | stderr 1>/tmp/pipeline-combined-stderr.txt"
                "stdin </etc/banner.txt | stderr 2>/var/x"
                "stdin </etc/banner.txt | /bin/stderr 2>/tmp/pipeline-combined-stderr.txt"
                "/bin/stdin </etc/banner.txt | stderr 2>/tmp/pipeline-combined-stderr.txt"
                "stdin </etc/banner.txt | stderr 2>/tmp/pipeline-combined-stderr.txt | stdin"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#bare_name_combined_pipeline_stderr_regular_file_redirection_commands[@]}" ]; then
                printf '%s\r' "${bare_name_combined_pipeline_stderr_regular_file_redirection_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if [ "$SHELL_BARE_NAME_COMBINED_PIPELINE_STDERR_APPEND_REDIRECTION_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            bare_name_combined_pipeline_stderr_append_redirection_commands=(
                "help"
                "status"
                "stdio"
                "stdin </etc/banner.txt | stderr 2>/tmp/pipeline-combined-stderr-append.txt"
                "stdin </etc/banner.txt | stderr 2>>/tmp/pipeline-combined-stderr-append.txt"
                "waitpid 0x100001"
                "waitpid 0x100002"
                "laststatus"
                "pipestatus"
                "cat /proc/talos/processes"
                "ps"
                "cat /tmp/pipeline-combined-stderr-append.txt"
                "stdin </etc/banner.txt"
                "stderr"
                "/bin/stdin </etc/banner.txt | /bin/stderr 2>>/tmp/pipeline-combined-stderr-append.txt"
                "missing </etc/banner.txt | stderr 2>>/tmp/pipeline-combined-stderr-append.txt"
                "stdin </etc/banner.txt | missing 2>>/tmp/pipeline-combined-stderr-append.txt"
                "stdin </etc/banner.txt | /bin/stderr 2>>/tmp/pipeline-combined-stderr-append.txt"
                "/bin/stdin </etc/banner.txt | stderr 2>>/tmp/pipeline-combined-stderr-append.txt"
                "stdin </etc/banner.txt | stderr 2>>/tmp/pipeline-combined-stderr.txt"
                "stdin </etc/banner.txt | stderr 2>/tmp/pipeline-stderr.txt"
                "stdout | stderr 2>>/tmp/pipeline-combined-stderr-append.txt"
                "stdin </etc/banner.txt | stderr 2>> /tmp/pipeline-combined-stderr-append.txt"
                "stdin </etc/banner.txt | stderr 1>/tmp/pipeline-combined-stderr-append.txt"
                "stdin </etc/banner.txt | stderr 2>>/var/x"
                "stdin </etc/banner.txt | stderr 2>>/tmp/pipeline-combined-stderr-append.txt | stdin"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#bare_name_combined_pipeline_stderr_append_redirection_commands[@]}" ]; then
                printf '%s\r' "${bare_name_combined_pipeline_stderr_append_redirection_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if [ "$SHELL_BARE_NAME_COMBINED_PIPELINE_STDIN_STDOUT_REDIRECTION_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            bare_name_combined_pipeline_stdin_stdout_redirection_commands=(
                "help"
                "status"
                "stdio"
                "stdin </etc/banner.txt | stdin >/tmp/pipeline-combined.txt"
                "waitpid 0x100001"
                "waitpid 0x100002"
                "laststatus"
                "pipestatus"
                "cat /proc/talos/processes"
                "ps"
                "cat /tmp/pipeline-combined.txt"
                "stdin </etc/banner.txt"
                "/bin/stdin </etc/banner.txt | /bin/stdin >/tmp/pipeline-combined.txt"
                "stdin </etc/banner.txt | stdin >>/tmp/pipeline-combined.txt"
                "stdin </etc/banner.txt | stdin >/tmp/pipeline-report.txt"
                "stdout | stdin >/tmp/pipeline-combined.txt"
                "stdin </etc/banner.txt | stdin 1>/tmp/pipeline-combined.txt"
                "stdin </etc/banner.txt | stdin > /tmp/pipeline-combined.txt"
                "stdin </etc/banner.txt | stdin >/var/x"
                "stdin </etc/banner.txt | /bin/stdin >/tmp/pipeline-combined.txt"
                "/bin/stdin </etc/banner.txt | stdin >/tmp/pipeline-combined.txt"
                "stdin </etc/banner.txt | stdin >/tmp/pipeline-combined.txt | stdin"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#bare_name_combined_pipeline_stdin_stdout_redirection_commands[@]}" ]; then
                printf '%s\r' "${bare_name_combined_pipeline_stdin_stdout_redirection_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if [ "$SHELL_BARE_NAME_PIPELINE_STDIN_REDIRECTION_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            bare_name_pipeline_stdin_redirection_commands=(
                "help"
                "status"
                "stdio"
                "stdin </etc/banner.txt | stdin"
                "waitpid 0x100001"
                "waitpid 0x100002"
                "laststatus"
                "pipestatus"
                "cat /proc/talos/processes"
                "ps"
                "stdin </etc/banner.txt | stdout"
                "stdin alpha </etc/banner.txt | stdin"
                "stdin </etc/banner.txt | stdin beta"
                "stdin </dev/null | stdin"
                "stdin < /etc/banner.txt | stdin"
                "stdin </etc/banner.txt | stdin | stdin"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#bare_name_pipeline_stdin_redirection_commands[@]}" ]; then
                printf '%s\r' "${bare_name_pipeline_stdin_redirection_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if [ "$SHELL_BARE_NAME_PIPELINE_CONSUMER_STDIN_REDIRECTION_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            bare_name_pipeline_consumer_stdin_redirection_commands=(
                "help"
                "status"
                "stdio"
                "stdin | stdin </etc/banner.txt"
                "waitpid 0x100001"
                "waitpid 0x100002"
                "laststatus"
                "pipestatus"
                "cat /proc/talos/processes"
                "ps"
                "stdout | stdin </etc/banner.txt"
                "stdin alpha | stdin </etc/banner.txt"
                "stdin | stdin beta </etc/banner.txt"
                "stdin | stdin </dev/null"
                "stdin | stdin < /etc/banner.txt"
                "stdin | stdin </etc/banner.txt | stdin"
                "stdin </etc/banner.txt | stdin </etc/banner.txt"
                "stdin </etc/banner.txt | /bin/stdin </etc/banner.txt"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#bare_name_pipeline_consumer_stdin_redirection_commands[@]}" ]; then
                printf '%s\r' "${bare_name_pipeline_consumer_stdin_redirection_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if [ "$SHELL_BARE_NAME_PIPELINE_STAGE_ARGV_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            bare_name_pipeline_stage_argv_commands=(
                "help"
                "status"
                "stdio"
                "stdout alpha | stdin beta"
                "waitpid 0x100001"
                "waitpid 0x100002"
                "laststatus"
                "pipestatus"
                "cat /proc/talos/processes"
                "ps"
                "stdout alpha beta | stdin"
                "stdout * | stdin"
                "stdout alpha | stdin beta gamma"
                "stdout alpha | stdin *"
                "stdout | stdin"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#bare_name_pipeline_stage_argv_commands[@]}" ]; then
                printf '%s\r' "${bare_name_pipeline_stage_argv_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if [ "$SHELL_BARE_NAME_VFS_PIPELINE_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            bare_name_vfs_pipeline_commands=(
                "help"
                "status"
                "stdio"
                "stdout | stdin"
                "waitpid 0x100001"
                "waitpid 0x100002"
                "laststatus"
                "pipestatus"
                "cat /proc/talos/processes"
                "ps"
                "stdout | /bin/stdin"
                "/bin/stdout | stdin"
                "missing | stdin"
                "stdout | missing"
                "stdout 1>&2 | stdin"
                "stdout | stdin 1>/tmp/stdout.txt"
                "stdout | stdin | stdin"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#bare_name_vfs_pipeline_commands[@]}" ]; then
                printf '%s\r' "${bare_name_vfs_pipeline_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if [ "$SHELL_BARE_NAME_COMMAND_ARGV_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            bare_name_command_argv_commands=(
                "help"
                "status"
                "stdio"
                "status42 alpha beta"
                "waitpid"
                "laststatus"
                "cat /proc/talos/processes"
                "ps"
                "status42"
                "stdout | stdin"
                "status42 alpha beta gamma delta"
                "status42 *"
                "missing alpha"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#bare_name_command_argv_commands[@]}" ]; then
                printf '%s\r' "${bare_name_command_argv_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if [ "$SHELL_ABSOLUTE_PATH_VFS_COMMAND_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            absolute_path_vfs_command_commands=(
                "help"
                "status"
                "stdio"
                "/bin/status42"
                "waitpid"
                "laststatus"
                "cat /proc/talos/processes"
                "ps"
                "/bin/status42 alpha beta"
                "waitpid"
                "laststatus"
                "/bin/status42 alpha beta gamma delta"
                "/bin/status42 *"
                "/missing"
                "bin/status42"
                "/bin"
                "/etc/banner.txt"
                "status42"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#absolute_path_vfs_command_commands[@]}" ]; then
                printf '%s\r' "${absolute_path_vfs_command_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if [ "$SHELL_DIRECT_STDIN_REDIRECTION_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            direct_stdin_redirection_commands=(
                "help"
                "status"
                "stdio"
                "/bin/stdin </etc/banner.txt"
                "waitpid"
                "laststatus"
                "pipestatus"
                "cat /proc/talos/processes"
                "ps"
                "/bin/stdout </etc/banner.txt"
                "/bin/stdin </dev/null"
                "/bin/stdin < /etc/banner.txt"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#direct_stdin_redirection_commands[@]}" ]; then
                printf '%s\r' "${direct_stdin_redirection_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if [ "$SHELL_DIRECT_COMBINED_STDIN_STDOUT_REDIRECTION_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            direct_combined_redirection_commands=(
                "help"
                "status"
                "stdio"
                "/bin/stdin </etc/banner.txt >/tmp/stdin-report.txt"
                "waitpid"
                "laststatus"
                "cat /tmp/stdin-report.txt"
                "/bin/stdin >/tmp/stdin-report.txt </etc/banner.txt"
                "/bin/stdin </dev/null >/tmp/stdin-report.txt"
                "/bin/stdin </etc/banner.txt 1>/tmp/stdin-report.txt"
                "/bin/stdin < /etc/banner.txt >/tmp/stdin-report.txt"
                "/bin/stdin </etc/banner.txt >>/tmp/stdin-report.txt"
                "/bin/stdin </etc/banner.txt 2>/tmp/stdin-report.txt"
                "/bin/stdin </etc/banner.txt >/tmp/other.txt"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#direct_combined_redirection_commands[@]}" ]; then
                printf '%s\r' "${direct_combined_redirection_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if [ "$SHELL_BARE_NAME_COMBINED_STDIN_STDOUT_REDIRECTION_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            bare_name_combined_redirection_commands=(
                "help"
                "status"
                "stdio"
                "stdin </etc/banner.txt >/tmp/stdin-report.txt"
                "waitpid"
                "laststatus"
                "cat /tmp/stdin-report.txt"
                "stdin >/tmp/stdin-report.txt </etc/banner.txt"
                "stdin </dev/null >/tmp/stdin-report.txt"
                "stdin </etc/banner.txt 1>/tmp/stdin-report.txt"
                "stdin < /etc/banner.txt >/tmp/stdin-report.txt"
                "stdin </etc/banner.txt >>/tmp/stdin-report.txt"
                "stdin </etc/banner.txt 2>/tmp/stdin-report.txt"
                "stdout </etc/banner.txt >/tmp/stdin-report.txt"
                "stdin </etc/banner.txt >/tmp/other.txt"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#bare_name_combined_redirection_commands[@]}" ]; then
                printf '%s\r' "${bare_name_combined_redirection_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if [ "$SHELL_BARE_NAME_STDIN_REDIRECTION_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            bare_name_stdin_redirection_commands=(
                "help"
                "status"
                "stdio"
                "stdin </etc/banner.txt"
                "waitpid"
                "laststatus"
                "pipestatus"
                "cat /proc/talos/processes"
                "ps"
                "stdout </etc/banner.txt"
                "stdin </dev/null"
                "stdin < /etc/banner.txt"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#bare_name_stdin_redirection_commands[@]}" ]; then
                printf '%s\r' "${bare_name_stdin_redirection_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if [ "$SHELL_BARE_NAME_STDOUT_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            bare_name_stdout_regular_file_redirection_commands=(
                "help"
                "status"
                "stdio"
                "stdout >/tmp/stdout.txt"
                "waitpid"
                "laststatus"
                "cat /tmp/stdout.txt"
                "stdout"
                "stdout >>/var/append.txt"
                "stdout >/var/other.txt"
                "stdout | stdin >/tmp/stdout.txt"
                "stdin </etc/banner.txt >/tmp/stdout.txt"
                "cat /etc/banner.txt"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#bare_name_stdout_regular_file_redirection_commands[@]}" ]; then
                printf '%s\r' "${bare_name_stdout_regular_file_redirection_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if [ "$SHELL_DIRECT_STDERR_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            direct_stderr_regular_file_append_redirection_commands=(
                "help"
                "status"
                "stdio"
                "/bin/stderr 2>/tmp/stderr.txt"
                "waitpid"
                "laststatus"
                "cat /tmp/stderr.txt"
                "/bin/stderr 2>>/tmp/stderr.txt"
                "waitpid"
                "laststatus"
                "cat /tmp/stderr.txt"
                "/bin/stderr"
                "/bin/stderr 2>>/tmp/other.txt"
                "stderr 2>>/tmp/other.txt"
                "/bin/stderr | /bin/stdin 2>>/tmp/stderr.txt"
                "/bin/stdin </etc/banner.txt 2>>/tmp/stderr.txt"
                "/bin/stdout >>/tmp/stdout.txt"
                "cat /etc/banner.txt"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#direct_stderr_regular_file_append_redirection_commands[@]}" ]; then
                printf '%s\r' "${direct_stderr_regular_file_append_redirection_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
        if [ "$SHELL_BARE_NAME_STDERR_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ] && [[ "$line" =~ ready\ command=([0-9]+) ]]; then
            bare_name_stderr_regular_file_append_redirection_commands=(
                "help"
                "status"
                "stdio"
                "stderr 2>/tmp/stderr.txt"
                "waitpid"
                "laststatus"
                "cat /tmp/stderr.txt"
                "stderr 2>>/tmp/stderr.txt"
                "waitpid"
                "laststatus"
                "cat /tmp/stderr.txt"
                "stderr"
                "stderr 2>>/tmp/other.txt"
                "stderr 2>>/var/err.txt"
                "stderr | stdin 2>>/tmp/stderr.txt"
                "stdin </etc/banner.txt 2>>/tmp/stderr.txt"
                "missing 2>>/tmp/stderr.txt"
                "cat /etc/banner.txt"
            )
            command_index="${BASH_REMATCH[1]}"
            if [ "$sent" -eq "$command_index" ] && [ "$command_index" -lt "${#bare_name_stderr_regular_file_append_redirection_commands[@]}" ]; then
                printf '%s\r' "${bare_name_stderr_regular_file_append_redirection_commands[$command_index]}" >&3
                sent=$((command_index + 1))
            fi
            continue
        fi
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
                elif [ "$sent" -eq 10 ] && [ "$SHELL_STDOUT_TO_STDERR_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'exec stdout | stderr\r' >&3
                    sent=11
                elif [ "$sent" -eq 10 ] && [ "$SHELL_DEV_NULL_STDOUT_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'exec stdout | stderr\r' >&3
                    sent=11
                elif [ "$sent" -eq 10 ] && [ "$SHELL_DEV_NULL_STDERR_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'exec stderr 2>>/dev/null\r' >&3
                    sent=11
                elif [ "$sent" -eq 10 ] && [ "$SHELL_DEV_NULL_STDIN_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'exec stdin < /dev/null\r' >&3
                    sent=11
                elif [ "$sent" -eq 10 ] && [ "$SHELL_READONLY_REGULAR_FILE_STDIN_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'exec stdin < /etc/banner.txt\r' >&3
                    sent=11
                elif [ "$sent" -eq 10 ] && [ "$SHELL_STDOUT_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'exec stdout >/var/other.txt\r' >&3
                    sent=11
                elif [ "$sent" -eq 10 ] && [ "$SHELL_DIRECT_STDOUT_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf '/bin/stdout\r' >&3
                    sent=11
                elif [ "$sent" -eq 10 ] && [ "$SHELL_BARE_NAME_STDOUT_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'stdout\r' >&3
                    sent=11
                elif [ "$sent" -eq 10 ] && [ "$SHELL_STDOUT_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'exec stdout\r' >&3
                    sent=11
                elif [ "$sent" -eq 10 ] && [ "$SHELL_STDOUT_REGULAR_FILE_APPEND_CREATE_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'cat /etc/banner.txt\r' >&3
                    sent=11
                elif [ "$sent" -eq 10 ] && [ "$SHELL_EXPLICIT_FD1_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'laststatus\r' >&3
                    sent=11
                elif [ "$sent" -eq 10 ] && [ "$SHELL_STDERR_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf '/bin/stderr 2>>/tmp/stderr.txt\r' >&3
                    sent=11
                elif [ "$sent" -eq 10 ] && [ "$SHELL_STDERR_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'exec stderr\r' >&3
                    sent=11
                elif [ "$sent" -eq 10 ] && [ "$SHELL_STDERR_REGULAR_FILE_APPEND_CREATE_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'exec stderr 2>>/tmp/stdout.txt\r' >&3
                    sent=11
                elif [ "$sent" -eq 10 ] && [ "$SHELL_STDERR_TO_STDOUT_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'exec stderr 2>file\r' >&3
                    sent=11
                elif [ "$sent" -eq 10 ] && [ "$SHELL_STDOUT_CLOSE_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'exec stdout | stderr\r' >&3
                    sent=11
                elif [ "$sent" -eq 10 ] && [ "$SHELL_STDERR_CLOSE_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'exec stderr 2>file\r' >&3
                    sent=11
                elif [ "$sent" -eq 10 ] && [ "$SHELL_STDIO_SMOKE" -eq 1 ]; then
                    printf 'exec /bin/status42 gamma\r' >&3
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
                elif [ "$sent" -eq 11 ] && [ "$SHELL_STDOUT_TO_STDERR_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'cat /etc/banner.txt\r' >&3
                    sent=12
                elif [ "$sent" -eq 11 ] && [ "$SHELL_DEV_NULL_STDOUT_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'cat /etc/banner.txt\r' >&3
                    sent=12
                elif [ "$sent" -eq 11 ] && [ "$SHELL_DEV_NULL_STDERR_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'exec stderr </dev/null\r' >&3
                    sent=12
                elif [ "$sent" -eq 11 ] && [ "$SHELL_DEV_NULL_STDIN_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'cat /etc/banner.txt\r' >&3
                    sent=12
                elif [ "$sent" -eq 11 ] && [ "$SHELL_READONLY_REGULAR_FILE_STDIN_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'cat /etc/banner.txt\r' >&3
                    sent=12
                elif [ "$sent" -eq 11 ] && [ "$SHELL_STDOUT_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'exec stderr 2>/tmp/stdout.txt\r' >&3
                    sent=12
                elif [ "$sent" -eq 11 ] && [ "$SHELL_DIRECT_STDOUT_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf '/bin/stdout >>/var/other.txt\r' >&3
                    sent=12
                elif [ "$sent" -eq 11 ] && [ "$SHELL_BARE_NAME_STDOUT_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'stdout >>/var/other.txt\r' >&3
                    sent=12
                elif [ "$sent" -eq 11 ] && [ "$SHELL_STDOUT_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'exec stdout >>/var/other.txt\r' >&3
                    sent=12
                elif [ "$sent" -eq 11 ] && [ "$SHELL_EXPLICIT_FD1_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'cat /tmp/stdout.txt\r' >&3
                    sent=12
                elif [ "$sent" -eq 11 ] && [ "$SHELL_STDERR_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf '/bin/stderr 2> /tmp/stderr.txt\r' >&3
                    sent=12
                elif [ "$sent" -eq 11 ] && [ "$SHELL_STDERR_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'exec stdout\r' >&3
                    sent=12
                elif [ "$sent" -eq 11 ] && [ "$SHELL_STDERR_REGULAR_FILE_APPEND_CREATE_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'cat /etc/banner.txt\r' >&3
                    sent=12
                elif [ "$sent" -eq 11 ] && [ "$SHELL_STDERR_TO_STDOUT_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'cat /etc/banner.txt\r' >&3
                    sent=12
                elif [ "$sent" -eq 11 ] && [ "$SHELL_STDOUT_CLOSE_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'cat /etc/banner.txt\r' >&3
                    sent=12
                elif [ "$sent" -eq 11 ] && [ "$SHELL_STDERR_CLOSE_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'cat /etc/banner.txt\r' >&3
                    sent=12
                elif [ "$sent" -eq 11 ] && [ "$SHELL_MINIMAL_STDOUT_TO_STDIN_PIPELINE_SMOKE" -eq 1 ]; then
                    printf 'cat /etc/banner.txt\r' >&3
                    sent=12
                elif [ "$sent" -eq 11 ] && [ "$SHELL_STDIO_SMOKE" -eq 1 ]; then
                    printf 'exec missing\r' >&3
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
                elif [ "$sent" -eq 12 ] && [ "$SHELL_DEV_NULL_STDERR_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'cat /etc/banner.txt\r' >&3
                    sent=13
                elif [ "$sent" -eq 12 ] && [ "$SHELL_STDOUT_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'cat /etc/banner.txt\r' >&3
                    sent=13
                elif [ "$sent" -eq 12 ] && [ "$SHELL_DIRECT_STDOUT_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf '/bin/stderr 2>>/tmp/stderr.txt\r' >&3
                    sent=13
                elif [ "$sent" -eq 12 ] && [ "$SHELL_BARE_NAME_STDOUT_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'stderr 2>>/tmp/other.txt\r' >&3
                    sent=13
                elif [ "$sent" -eq 12 ] && [ "$SHELL_STDOUT_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'exec stderr 2>>/tmp/stderr.txt\r' >&3
                    sent=13
                elif [ "$sent" -eq 12 ] && [ "$SHELL_EXPLICIT_FD1_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'exec stdout 3>/tmp/stdout.txt\r' >&3
                    sent=13
                elif [ "$sent" -eq 12 ] && [ "$SHELL_STDERR_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'exec stdout >/tmp/stderr.txt\r' >&3
                    sent=13
                elif [ "$sent" -eq 12 ] && [ "$SHELL_STDERR_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'exec stdout >>/tmp/stdout.txt\r' >&3
                    sent=13
                elif [ "$sent" -eq 12 ] && [ "$SHELL_STDIO_SMOKE" -eq 1 ]; then
                    printf 'exec bin/status42\r' >&3
                    sent=13
                fi
                ;;
            *"$LABEL: ready command=13"*)
                if [ "$sent" -eq 13 ] && { [ "$SHELL_DIRECT_STDOUT_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ] || [ "$SHELL_BARE_NAME_STDOUT_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ] || [ "$SHELL_STDOUT_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ]; }; then
                    printf 'cat /etc/banner.txt\r' >&3
                    sent=14
                elif [ "$sent" -eq 13 ] && [ "$SHELL_EXPLICIT_FD1_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'exec stdout 1>/var/other.txt\r' >&3
                    sent=14
                elif [ "$sent" -eq 13 ] && [ "$SHELL_STDERR_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'cat /etc/banner.txt\r' >&3
                    sent=14
                elif [ "$sent" -eq 13 ] && [ "$SHELL_STDERR_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'exec stderr 2>>/tmp/other.txt\r' >&3
                    sent=14
                elif [ "$sent" -eq 13 ] && [ "$SHELL_WAITPID_SMOKE" -eq 1 ]; then
                    printf 'exec bin/init\r' >&3
                    sent=14
                elif [ "$sent" -eq 13 ] && [ "$SHELL_VFS_EXEC_SMOKE" -eq 1 ]; then
                    printf 'exec /empty\r' >&3
                    sent=14
                elif [ "$sent" -eq 13 ] && [ "$SHELL_PATH_LOOKUP_SMOKE" -eq 1 ]; then
                    printf 'exec /etc/banner.txt\r' >&3
                    sent=14
                elif [ "$sent" -eq 13 ] && [ "$SHELL_STDIO_SMOKE" -eq 1 ]; then
                    printf 'exec /bin\r' >&3
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
                elif [ "$sent" -eq 14 ] && [ "$SHELL_STDIO_SMOKE" -eq 1 ]; then
                    printf 'exec /etc/banner.txt\r' >&3
                    sent=15
                elif [ "$sent" -eq 14 ] && [ "$SHELL_STDERR_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'exec stdout >/tmp/stderr.txt\r' >&3
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
                elif [ "$sent" -eq 15 ] && [ "$SHELL_STDIO_SMOKE" -eq 1 ]; then
                    printf 'exec /empty\r' >&3
                    sent=16
                elif [ "$sent" -eq 15 ] && [ "$SHELL_STDERR_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'cat /etc/banner.txt\r' >&3
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
                elif [ "$sent" -eq 16 ] && [ "$SHELL_STDIO_SMOKE" -eq 1 ]; then
                    printf 'exec /bin/status42 *\r' >&3
                    sent=17
                fi
                ;;
            *"$LABEL: ready command=17"*)
                if [ "$sent" -eq 17 ] && [ "$SHELL_WAITPID_SMOKE" -eq 1 ]; then
                    printf 'cat /etc/banner.txt\r' >&3
                    sent=18
                elif [ "$sent" -eq 17 ] && [ "$SHELL_STDIO_SMOKE" -eq 1 ]; then
                    printf 'cat /etc/banner.txt\r' >&3
                    sent=18
                fi
                ;;
            *"$LABEL: ready command=18"*)
                if [ "$sent" -eq 18 ] && [ "$SHELL_WAITPID_SMOKE" -eq 1 ]; then
                    printf 'exec stdout | exec stdin\r' >&3
                    sent=19
                fi
                ;;
            *"$LABEL: ready command=19"*)
                if [ "$sent" -eq 19 ] && [ "$SHELL_WAITPID_SMOKE" -eq 1 ]; then
                    printf 'waitpid 0x100001\r' >&3
                    sent=20
                fi
                ;;
            *"$LABEL: ready command=20"*)
                if [ "$sent" -eq 20 ] && [ "$SHELL_WAITPID_SMOKE" -eq 1 ]; then
                    printf 'waitpid 0x100002\r' >&3
                    sent=21
                fi
                ;;
            *"$LABEL: ready command=21"*)
                if [ "$sent" -eq 21 ] && [ "$SHELL_WAITPID_SMOKE" -eq 1 ]; then
                    printf 'waitpid 0x100001\r' >&3
                    sent=22
                fi
                ;;
            *"$LABEL: ready command=22"*)
                if [ "$sent" -eq 22 ] && [ "$SHELL_WAITPID_SMOKE" -eq 1 ]; then
                    printf 'waitpid bogus\r' >&3
                    sent=23
                fi
                ;;
            *"$LABEL: ready command=23"*)
                if [ "$sent" -eq 23 ] && [ "$SHELL_WAITPID_SMOKE" -eq 1 ]; then
                    printf 'waitpid 0x0\r' >&3
                    sent=24
                fi
                ;;
            *"$LABEL: ready command=24"*)
                if [ "$sent" -eq 24 ] && [ "$SHELL_WAITPID_SMOKE" -eq 1 ]; then
                    printf 'cat /etc/banner.txt\r' >&3
                    sent=25
                fi
                ;;
            *"$LABEL: ready command=25"*)
                if [ "$sent" -eq 25 ] && [ "$SHELL_WAITPID_SMOKE" -eq 1 ]; then
                    printf 'exec /bin/status42 &\r' >&3
                    sent=26
                fi
                ;;
            *"$LABEL: ready command=26"*)
                if [ "$sent" -eq 26 ] && [ "$SHELL_WAITPID_SMOKE" -eq 1 ]; then
                    printf 'waitpid 0x100001\r' >&3
                    sent=27
                fi
                ;;
            *"$LABEL: ready command=27"*)
                if [ "$sent" -eq 27 ] && [ "$SHELL_WAITPID_SMOKE" -eq 1 ]; then
                    printf 'waitpid 0x100001\r' >&3
                    sent=28
                fi
                ;;
            *"$LABEL: ready command=28"*)
                if [ "$sent" -eq 28 ] && [ "$SHELL_WAITPID_SMOKE" -eq 1 ]; then
                    printf 'jobs\r' >&3
                    sent=29
                fi
                ;;
            *"$LABEL: ready command=29"*)
                if [ "$sent" -eq 29 ] && [ "$SHELL_WAITPID_SMOKE" -eq 1 ]; then
                    printf 'exec /bin/zero &\r' >&3
                    sent=30
                fi
                ;;
            *"$LABEL: ready command=30"*)
                if [ "$sent" -eq 30 ] && [ "$SHELL_WAITPID_SMOKE" -eq 1 ]; then
                    printf 'waitpid 0x100002\r' >&3
                    sent=31
                fi
                ;;
            *"$LABEL: ready command=31"*)
                if [ "$sent" -eq 31 ] && [ "$SHELL_WAITPID_SMOKE" -eq 1 ]; then
                    printf 'waitpid 0x100002\r' >&3
                    sent=32
                fi
                ;;
            *"$LABEL: ready command=32"*)
                if [ "$sent" -eq 32 ] && [ "$SHELL_WAITPID_SMOKE" -eq 1 ]; then
                    printf 'jobs\r' >&3
                    sent=33
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
                    elif [ "$SHELL_STDOUT_TO_STDERR_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'exec stdout 1>&2\r' >&3
                    elif [ "$SHELL_DEV_NULL_STDOUT_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'exec stdout >/dev/null\r' >&3
                    elif [ "$SHELL_DEV_NULL_STDERR_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'exec stderr 2>/dev/null\r' >&3
                    elif [ "$SHELL_DEV_NULL_STDIN_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'exec stdin </dev/null\r' >&3
                    elif [ "$SHELL_READONLY_REGULAR_FILE_STDIN_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'exec stdin </etc/banner.txt\r' >&3
                    elif [ "$SHELL_STDOUT_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf '/bin/stdout >/tmp/stdout.txt\r' >&3
                    elif [ "$SHELL_DIRECT_STDOUT_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf '/bin/stdout >/tmp/stdout.txt\r' >&3
                    elif [ "$SHELL_BARE_NAME_STDOUT_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'stdout >/tmp/stdout.txt\r' >&3
                    elif [ "$SHELL_STDOUT_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'exec stdout >/tmp/stdout.txt\r' >&3
                    elif [ "$SHELL_STDOUT_REGULAR_FILE_APPEND_CREATE_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'exec stdout >>/tmp/stdout.txt\r' >&3
                    elif [ "$SHELL_EXPLICIT_FD1_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'exec stdout 1>/tmp/stdout.txt\r' >&3
                    elif [ "$SHELL_STDERR_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'stderr 2>/tmp/stderr.txt\r' >&3
                    elif [ "$SHELL_STDERR_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'exec stderr 2>/tmp/stderr.txt\r' >&3
                    elif [ "$SHELL_STDERR_REGULAR_FILE_APPEND_CREATE_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'exec stderr 2>>/tmp/stderr.txt\r' >&3
                    elif [ "$SHELL_STDERR_TO_STDOUT_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'exec stderr 2>&1\r' >&3
                    elif [ "$SHELL_STDOUT_CLOSE_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'exec stdout 1>&-\r' >&3
                    elif [ "$SHELL_STDERR_CLOSE_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'exec stderr 2>&-\r' >&3
                    elif [ "$SHELL_PIPELINE_STDERR_DUP_TO_STDOUT_SMOKE" -eq 1 ]; then
                        printf 'exec stderr 2>&1 | exec stdin\r' >&3
                    elif [ "$SHELL_PIPELINE_STDOUT_REDIRECT_AWAY_SMOKE" -eq 1 ]; then
                        printf 'exec stdout 1>&2 | exec stdin\r' >&3
                    elif [ "$SHELL_PIPELINE_STDERR_NOT_PIPED_SMOKE" -eq 1 ]; then
                        printf 'exec stderr | exec stdin\r' >&3
                    elif [ "$SHELL_MINIMAL_STDOUT_TO_STDIN_PIPELINE_SMOKE" -eq 1 ]; then
                        printf 'exec stdout | exec stdin\r' >&3
                    elif [ "$SHELL_STDERR_SMOKE" -eq 1 ]; then
                        printf 'exec stderr\r' >&3
                    elif [ "$SHELL_STDIN_SMOKE" -eq 1 ]; then
                        printf 'exec stdin\rtalos-console0' >&3
                    elif [ "$SHELL_STDIN_EOF_SMOKE" -eq 1 ] || [ "$SHELL_STDIN_READINESS_SMOKE" -eq 1 ] || [ "$SHELL_STDIN_BOUNDED_WAIT_SMOKE" -eq 1 ] || [ "$SHELL_STDIN_SCHEDULER_WAIT_SMOKE" -eq 1 ]; then
                        printf 'exec stdin\r' >&3
                    elif [ "$SHELL_STDIN_TERMINAL_EOF_SMOKE" -eq 1 ]; then
                        printf 'exec stdin\r\004' >&3
                    elif [ "$SHELL_STDOUT_SMOKE" -eq 1 ]; then
                        printf 'exec stdout\r' >&3
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
                    elif [ "$SHELL_STDOUT_TO_STDERR_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'waitpid\r' >&3
                    elif [ "$SHELL_DEV_NULL_STDOUT_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'waitpid\r' >&3
                    elif [ "$SHELL_DEV_NULL_STDERR_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'waitpid\r' >&3
                    elif [ "$SHELL_DEV_NULL_STDIN_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'waitpid\r' >&3
                    elif [ "$SHELL_READONLY_REGULAR_FILE_STDIN_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'waitpid\r' >&3
                    elif [ "$SHELL_STDOUT_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'waitpid\r' >&3
                    elif [ "$SHELL_DIRECT_STDOUT_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ] || [ "$SHELL_BARE_NAME_STDOUT_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ] || [ "$SHELL_STDOUT_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'waitpid\r' >&3
                    elif [ "$SHELL_STDOUT_REGULAR_FILE_APPEND_CREATE_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'waitpid\r' >&3
                    elif [ "$SHELL_EXPLICIT_FD1_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'waitpid\r' >&3
                    elif [ "$SHELL_STDERR_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'waitpid\r' >&3
                    elif [ "$SHELL_STDERR_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'waitpid\r' >&3
                    elif [ "$SHELL_STDERR_REGULAR_FILE_APPEND_CREATE_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'waitpid\r' >&3
                    elif [ "$SHELL_STDERR_TO_STDOUT_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'waitpid\r' >&3
                    elif [ "$SHELL_STDOUT_CLOSE_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'waitpid\r' >&3
                    elif [ "$SHELL_STDERR_CLOSE_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'waitpid\r' >&3
                    elif [ "$SHELL_PIPELINE_STDERR_DUP_TO_STDOUT_SMOKE" -eq 1 ]; then
                        printf 'waitpid\r' >&3
                    elif [ "$SHELL_PIPELINE_STDOUT_REDIRECT_AWAY_SMOKE" -eq 1 ]; then
                        printf 'waitpid\r' >&3
                    elif [ "$SHELL_PIPELINE_STDERR_NOT_PIPED_SMOKE" -eq 1 ]; then
                        printf 'waitpid\r' >&3
                    elif [ "$SHELL_MINIMAL_STDOUT_TO_STDIN_PIPELINE_SMOKE" -eq 1 ]; then
                        printf 'waitpid\r' >&3
                    elif [ "$SHELL_STDIO_SMOKE" -eq 1 ]; then
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
                    elif [ "$SHELL_STDOUT_TO_STDERR_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'laststatus\r' >&3
                    elif [ "$SHELL_DEV_NULL_STDOUT_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'laststatus\r' >&3
                    elif [ "$SHELL_DEV_NULL_STDERR_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'laststatus\r' >&3
                    elif [ "$SHELL_DEV_NULL_STDIN_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'laststatus\r' >&3
                    elif [ "$SHELL_READONLY_REGULAR_FILE_STDIN_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'laststatus\r' >&3
                    elif [ "$SHELL_STDOUT_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'laststatus\r' >&3
                    elif [ "$SHELL_DIRECT_STDOUT_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ] || [ "$SHELL_BARE_NAME_STDOUT_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ] || [ "$SHELL_STDOUT_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'laststatus\r' >&3
                    elif [ "$SHELL_STDOUT_REGULAR_FILE_APPEND_CREATE_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'laststatus\r' >&3
                    elif [ "$SHELL_EXPLICIT_FD1_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'laststatus\r' >&3
                    elif [ "$SHELL_STDERR_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'laststatus\r' >&3
                    elif [ "$SHELL_STDERR_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'laststatus\r' >&3
                    elif [ "$SHELL_STDERR_REGULAR_FILE_APPEND_CREATE_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'laststatus\r' >&3
                    elif [ "$SHELL_STDERR_TO_STDOUT_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'laststatus\r' >&3
                    elif [ "$SHELL_STDOUT_CLOSE_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'laststatus\r' >&3
                    elif [ "$SHELL_STDERR_CLOSE_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'laststatus\r' >&3
                    elif [ "$SHELL_PIPELINE_STDERR_DUP_TO_STDOUT_SMOKE" -eq 1 ]; then
                        printf 'laststatus\r' >&3
                    elif [ "$SHELL_PIPELINE_STDOUT_REDIRECT_AWAY_SMOKE" -eq 1 ]; then
                        printf 'laststatus\r' >&3
                    elif [ "$SHELL_PIPELINE_STDERR_NOT_PIPED_SMOKE" -eq 1 ]; then
                        printf 'laststatus\r' >&3
                    elif [ "$SHELL_MINIMAL_STDOUT_TO_STDIN_PIPELINE_SMOKE" -eq 1 ]; then
                        printf 'laststatus\r' >&3
                    elif [ "$SHELL_STDIO_SMOKE" -eq 1 ]; then
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
                    elif [ "$SHELL_STDOUT_TO_STDERR_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'exec stdout\r' >&3
                    elif [ "$SHELL_DEV_NULL_STDOUT_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'exec stdout\r' >&3
                    elif [ "$SHELL_DEV_NULL_STDERR_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'exec stderr\r' >&3
                    elif [ "$SHELL_DEV_NULL_STDIN_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'exec stdin\rtalos-console0' >&3
                    elif [ "$SHELL_READONLY_REGULAR_FILE_STDIN_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'exec stdin\rtalos-console0' >&3
                    elif [ "$SHELL_STDOUT_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'cat /tmp/stdout.txt\r' >&3
                    elif [ "$SHELL_DIRECT_STDOUT_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf '/bin/stdout >>/tmp/stdout.txt\r' >&3
                    elif [ "$SHELL_BARE_NAME_STDOUT_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'stdout >>/tmp/stdout.txt\r' >&3
                    elif [ "$SHELL_STDOUT_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'exec stdout >>/tmp/stdout.txt\r' >&3
                    elif [ "$SHELL_STDOUT_REGULAR_FILE_APPEND_CREATE_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'cat /tmp/stdout.txt\r' >&3
                    elif [ "$SHELL_EXPLICIT_FD1_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'cat /tmp/stdout.txt\r' >&3
                    elif [ "$SHELL_STDERR_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'cat /tmp/stderr.txt\r' >&3
                    elif [ "$SHELL_STDERR_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'exec stderr 2>>/tmp/stderr.txt\r' >&3
                    elif [ "$SHELL_STDERR_REGULAR_FILE_APPEND_CREATE_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'cat /tmp/stderr.txt\r' >&3
                    elif [ "$SHELL_STDERR_TO_STDOUT_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'exec stderr\r' >&3
                    elif [ "$SHELL_STDOUT_CLOSE_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'exec stdout\r' >&3
                    elif [ "$SHELL_STDERR_CLOSE_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'exec stderr\r' >&3
                    elif [ "$SHELL_PIPELINE_STDERR_DUP_TO_STDOUT_SMOKE" -eq 1 ]; then
                        printf 'exec stderr | exec stdin\r' >&3
                    elif [ "$SHELL_PIPELINE_STDOUT_REDIRECT_AWAY_SMOKE" -eq 1 ]; then
                        printf 'exec stderr 2>&1 | exec stdin\r' >&3
                    elif [ "$SHELL_PIPELINE_STDERR_NOT_PIPED_SMOKE" -eq 1 ]; then
                        printf 'exec stdout | exec stdin\r' >&3
                    elif [ "$SHELL_MINIMAL_STDOUT_TO_STDIN_PIPELINE_SMOKE" -eq 1 ]; then
                        printf 'cat /etc/banner.txt\r' >&3
                    elif [ "$SHELL_STDIN_SMOKE" -eq 1 ] || [ "$SHELL_STDIN_EOF_SMOKE" -eq 1 ] || [ "$SHELL_STDIN_TERMINAL_EOF_SMOKE" -eq 1 ] || [ "$SHELL_STDIN_READINESS_SMOKE" -eq 1 ] || [ "$SHELL_STDIN_BOUNDED_WAIT_SMOKE" -eq 1 ] || [ "$SHELL_STDIN_SCHEDULER_WAIT_SMOKE" -eq 1 ]; then
                        printf 'exec stdout\r' >&3
                    elif [ "$SHELL_STDERR_SMOKE" -eq 1 ]; then
                        printf 'exec status42\r' >&3
                    elif [ "$SHELL_STDOUT_SMOKE" -eq 1 ]; then
                        printf 'exec status42\r' >&3
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
                    elif [ "$SHELL_STDOUT_TO_STDERR_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'waitpid\r' >&3
                    elif [ "$SHELL_DEV_NULL_STDOUT_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'waitpid\r' >&3
                    elif [ "$SHELL_DEV_NULL_STDERR_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'waitpid\r' >&3
                    elif [ "$SHELL_DEV_NULL_STDIN_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'waitpid\r' >&3
                    elif [ "$SHELL_READONLY_REGULAR_FILE_STDIN_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'waitpid\r' >&3
                    elif [ "$SHELL_STDOUT_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf '/bin/stdout\r' >&3
                    elif [ "$SHELL_DIRECT_STDOUT_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ] || [ "$SHELL_BARE_NAME_STDOUT_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ] || [ "$SHELL_STDOUT_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'waitpid\r' >&3
                    elif [ "$SHELL_STDOUT_REGULAR_FILE_APPEND_CREATE_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'exec stdout\r' >&3
                    elif [ "$SHELL_EXPLICIT_FD1_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'exec stdout\r' >&3
                    elif [ "$SHELL_STDERR_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'stderr\r' >&3
                    elif [ "$SHELL_STDERR_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'waitpid\r' >&3
                    elif [ "$SHELL_STDERR_REGULAR_FILE_APPEND_CREATE_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'exec stderr\r' >&3
                    elif [ "$SHELL_STDERR_TO_STDOUT_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'waitpid\r' >&3
                    elif [ "$SHELL_STDOUT_CLOSE_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'waitpid\r' >&3
                    elif [ "$SHELL_STDERR_CLOSE_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'waitpid\r' >&3
                    elif [ "$SHELL_STDIO_SMOKE" -eq 1 ]; then
                        printf 'waitpid\r' >&3
                    elif [ "$SHELL_PIPELINE_STDERR_NOT_PIPED_SMOKE" -eq 1 ]; then
                        printf 'cat /etc/banner.txt\r' >&3
                    elif [ "$SHELL_PIPELINE_STDERR_DUP_TO_STDOUT_SMOKE" -eq 1 ]; then
                        printf 'exec stdout | exec stdin\r' >&3
                    elif [ "$SHELL_PIPELINE_STDOUT_REDIRECT_AWAY_SMOKE" -eq 1 ]; then
                        printf 'exec stdout | exec stdin\r' >&3
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
                    elif [ "$SHELL_STDOUT_TO_STDERR_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'exec stdout 2>&3\r' >&3
                    elif [ "$SHELL_DEV_NULL_STDOUT_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'exec stdout 1>/dev/null\r' >&3
                    elif [ "$SHELL_DEV_NULL_STDERR_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'exec stdout >/dev/null\r' >&3
                    elif [ "$SHELL_DEV_NULL_STDIN_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'exec stdout </dev/null\r' >&3
                    elif [ "$SHELL_READONLY_REGULAR_FILE_STDIN_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'exec stdout </dev/null\r' >&3
                    elif [ "$SHELL_STDOUT_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'waitpid\r' >&3
                    elif [ "$SHELL_DIRECT_STDOUT_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ] || [ "$SHELL_BARE_NAME_STDOUT_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ] || [ "$SHELL_STDOUT_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'laststatus\r' >&3
                    elif [ "$SHELL_STDOUT_REGULAR_FILE_APPEND_CREATE_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'exec stdout >>/var/other.txt\r' >&3
                    elif [ "$SHELL_EXPLICIT_FD1_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'exec stdout 1>>/tmp/stdout.txt\r' >&3
                    elif [ "$SHELL_STDERR_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'waitpid\r' >&3
                    elif [ "$SHELL_STDERR_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'laststatus\r' >&3
                    elif [ "$SHELL_STDERR_REGULAR_FILE_APPEND_CREATE_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'exec stdout\r' >&3
                    elif [ "$SHELL_STDERR_TO_STDOUT_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'exec stdout 1>&2\r' >&3
                    elif [ "$SHELL_STDOUT_CLOSE_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'exec stderr 2>file\r' >&3
                    elif [ "$SHELL_STDERR_CLOSE_REDIRECTION_SMOKE" -eq 1 ]; then
                        printf 'exec stdout 1>&-\r' >&3
                    elif [ "$SHELL_STDIO_SMOKE" -eq 1 ]; then
                        printf 'exec init\r' >&3
                    elif [ "$SHELL_PIPELINE_STDERR_DUP_TO_STDOUT_SMOKE" -eq 1 ]; then
                        printf 'cat /etc/banner.txt\r' >&3
                    elif [ "$SHELL_PIPELINE_STDOUT_REDIRECT_AWAY_SMOKE" -eq 1 ]; then
                        printf 'cat /etc/banner.txt\r' >&3
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
                elif [ "$sent" -eq 9 ] && [ "$SHELL_STDOUT_TO_STDERR_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'exec stdout 1>file\r' >&3
                    sent=10
                elif [ "$sent" -eq 9 ] && [ "$SHELL_DEV_NULL_STDOUT_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'exec stdout 1>file\r' >&3
                    sent=10
                elif [ "$sent" -eq 9 ] && [ "$SHELL_DEV_NULL_STDERR_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'exec stderr 2>file\r' >&3
                    sent=10
                elif [ "$sent" -eq 9 ] && [ "$SHELL_DEV_NULL_STDIN_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'exec stdin </etc/missing.txt\r' >&3
                    sent=10
                elif [ "$sent" -eq 9 ] && [ "$SHELL_READONLY_REGULAR_FILE_STDIN_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'exec stdin </dev/null\r' >&3
                    sent=10
                elif [ "$sent" -eq 9 ] && [ "$SHELL_STDOUT_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'exec stdout >>/var/other.txt\r' >&3
                    sent=10
                elif [ "$sent" -eq 9 ] && { [ "$SHELL_DIRECT_STDOUT_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ] || [ "$SHELL_BARE_NAME_STDOUT_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ] || [ "$SHELL_STDOUT_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ]; }; then
                    printf 'cat /tmp/stdout.txt\r' >&3
                    sent=10
                elif [ "$sent" -eq 9 ] && [ "$SHELL_STDOUT_REGULAR_FILE_APPEND_CREATE_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'exec stdout >>/tmp/stderr.txt\r' >&3
                    sent=10
                elif [ "$sent" -eq 9 ] && [ "$SHELL_EXPLICIT_FD1_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'waitpid\r' >&3
                    sent=10
                elif [ "$sent" -eq 9 ] && [ "$SHELL_STDERR_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'exec stdout\r' >&3
                    sent=10
                elif [ "$sent" -eq 9 ] && [ "$SHELL_STDERR_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'cat /tmp/stderr.txt\r' >&3
                    sent=10
                elif [ "$sent" -eq 9 ] && [ "$SHELL_STDERR_REGULAR_FILE_APPEND_CREATE_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'exec stderr 2>>/tmp/other.txt\r' >&3
                    sent=10
                elif [ "$sent" -eq 9 ] && [ "$SHELL_STDERR_TO_STDOUT_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'waitpid\r' >&3
                    sent=10
                elif [ "$sent" -eq 9 ] && [ "$SHELL_STDOUT_CLOSE_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'exec stdout 1>file\r' >&3
                    sent=10
                elif [ "$sent" -eq 9 ] && [ "$SHELL_STDERR_CLOSE_REDIRECTION_SMOKE" -eq 1 ]; then
                    printf 'waitpid\r' >&3
                    sent=10
                elif [ "$sent" -eq 9 ] && [ "$SHELL_STDIO_SMOKE" -eq 1 ]; then
                    printf 'exec zero\r' >&3
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

if [ "$SHELL_GENERATED_USERLAND_MANIFEST_SMOKE" -eq 1 ]; then
    manifest_digest="$(sha256sum userland/generated-root.manifest | awk '{print $1}')"
    echo "$LABEL: generated-root identity=phase10-generated-root-manifest-v1 source=userland/generated-root.manifest digest=sha256:$manifest_digest" >>"$LOG_FILE"
    if ! grep -q "Talos generated-root manifest fixture" src/initramfs.rs; then
        echo "$LABEL: generated-root hardcoded-src-initramfs-constant=false path=src/initramfs.rs" >>"$LOG_FILE"
    fi
    if ! grep -q "status7" src/initramfs.rs; then
        echo "$LABEL: generated-root executable-hardcoded-src-initramfs-constant=false path=src/initramfs.rs" >>"$LOG_FILE"
    fi
fi

grep -q "$LABEL: start" "$LOG_FILE"
grep -q "$LABEL: ready command=0" "$LOG_FILE"
grep -q "$LABEL: ready command=1" "$LOG_FILE"
grep -q "$LABEL: ready command=2" "$LOG_FILE"
grep -q "$LABEL: ready command=3" "$LOG_FILE"
grep -q "$LABEL: ready command=4" "$LOG_FILE"
if [ "$PWD_COMMAND_SMOKE" -eq 1 ] || [ "$LS_ROOT_SMOKE" -eq 1 ] || [ "$LS_BIN_SMOKE" -eq 1 ] || [ "$CAT_BANNER_SMOKE" -eq 1 ] || [ "$CAT_CWD_SMOKE" -eq 1 ] || [ "$SHELL_VFS_EXEC_SMOKE" -eq 1 ] || [ "$SHELL_LITERAL_ARGV_SMOKE" -eq 1 ] || [ "$SHELL_PATH_LOOKUP_SMOKE" -eq 1 ] || [ "$SHELL_STDIO_SMOKE" -eq 1 ] || [ "$SHELL_STDOUT_TO_STDERR_REDIRECTION_SMOKE" -eq 1 ] || [ "$SHELL_DEV_NULL_STDOUT_REDIRECTION_SMOKE" -eq 1 ] || [ "$SHELL_STDERR_TO_STDOUT_REDIRECTION_SMOKE" -eq 1 ] || [ "$SHELL_MINIMAL_STDOUT_TO_STDIN_PIPELINE_SMOKE" -eq 1 ] || [ "$SHELL_PIPELINE_STDERR_NOT_PIPED_SMOKE" -eq 1 ] || [ "$SHELL_BACKGROUND_VFS_EXEC_LIFECYCLE_SMOKE" -eq 1 ] || [ "$SHELL_JOBS_ACCOUNTING_LIST_SMOKE" -eq 1 ] || [ "$SHELL_MULTIPLE_BACKGROUND_JOBS_SMOKE" -eq 1 ] || [ "$SHELL_BACKGROUND_JOBS_STALE_ENTRY_POLICY_SMOKE" -eq 1 ] || [ "$SHELL_GENERATED_USERLAND_MANIFEST_SMOKE" -eq 1 ] || [ "$SHELL_WAITPID_SMOKE" -eq 1 ] || [ "$CD_FIXED_DIRS_SMOKE" -eq 1 ] || [ "$LS_CWD_SMOKE" -eq 1 ] || [ "$LITERAL_ECHO_SMOKE" -eq 1 ] || [ "$ECHO_COMMAND_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=5" "$LOG_FILE"
fi
if [ "$PWD_COMMAND_SMOKE" -eq 1 ] || [ "$LS_BIN_SMOKE" -eq 1 ] || [ "$CAT_BANNER_SMOKE" -eq 1 ] || [ "$CAT_CWD_SMOKE" -eq 1 ] || [ "$SHELL_VFS_EXEC_SMOKE" -eq 1 ] || [ "$SHELL_LITERAL_ARGV_SMOKE" -eq 1 ] || [ "$SHELL_PATH_LOOKUP_SMOKE" -eq 1 ] || [ "$SHELL_STDIO_SMOKE" -eq 1 ] || [ "$SHELL_STDOUT_TO_STDERR_REDIRECTION_SMOKE" -eq 1 ] || [ "$SHELL_DEV_NULL_STDOUT_REDIRECTION_SMOKE" -eq 1 ] || [ "$SHELL_STDERR_TO_STDOUT_REDIRECTION_SMOKE" -eq 1 ] || [ "$SHELL_MINIMAL_STDOUT_TO_STDIN_PIPELINE_SMOKE" -eq 1 ] || [ "$SHELL_PIPELINE_STDERR_NOT_PIPED_SMOKE" -eq 1 ] || [ "$SHELL_BACKGROUND_VFS_EXEC_LIFECYCLE_SMOKE" -eq 1 ] || [ "$SHELL_JOBS_ACCOUNTING_LIST_SMOKE" -eq 1 ] || [ "$SHELL_MULTIPLE_BACKGROUND_JOBS_SMOKE" -eq 1 ] || [ "$SHELL_BACKGROUND_JOBS_STALE_ENTRY_POLICY_SMOKE" -eq 1 ] || [ "$SHELL_GENERATED_USERLAND_MANIFEST_SMOKE" -eq 1 ] || [ "$SHELL_WAITPID_SMOKE" -eq 1 ] || [ "$CD_FIXED_DIRS_SMOKE" -eq 1 ] || [ "$LS_CWD_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=6" "$LOG_FILE"
fi
if [ "$SHELL_GENERATED_USERLAND_MANIFEST_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=7" "$LOG_FILE"
    grep -q "$LABEL: ready command=8" "$LOG_FILE"
    grep -q "$LABEL: ready command=9" "$LOG_FILE"
    grep -q "$LABEL: ready command=10" "$LOG_FILE"
    grep -q "$LABEL: ready command=11" "$LOG_FILE"
    grep -q "$LABEL: ready command=12" "$LOG_FILE"
    grep -q "$LABEL: ready command=13" "$LOG_FILE"
    grep -q "$LABEL: ready command=14" "$LOG_FILE"
fi
if [ "$SHELL_WAITPID_ANY_COMPLETED_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=5" "$LOG_FILE"
    grep -q "$LABEL: ready command=6" "$LOG_FILE"
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
    grep -q "$LABEL: ready command=18" "$LOG_FILE"
fi
if [ "$SHELL_DIRECT_PIPELINE_STAGE_ARGV_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=5" "$LOG_FILE"
    grep -q "$LABEL: ready command=6" "$LOG_FILE"
    grep -q "$LABEL: ready command=7" "$LOG_FILE"
    grep -q "$LABEL: ready command=8" "$LOG_FILE"
    grep -q "$LABEL: ready command=9" "$LOG_FILE"
    grep -q "$LABEL: ready command=10" "$LOG_FILE"
    grep -q "$LABEL: ready command=11" "$LOG_FILE"
    grep -q "$LABEL: ready command=12" "$LOG_FILE"
    grep -q "$LABEL: ready command=13" "$LOG_FILE"
    grep -q "$LABEL: ready command=14" "$LOG_FILE"
fi
if [ "$SHELL_DIRECT_PIPELINE_STDIN_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=5" "$LOG_FILE"
    grep -q "$LABEL: ready command=6" "$LOG_FILE"
    grep -q "$LABEL: ready command=7" "$LOG_FILE"
    grep -q "$LABEL: ready command=8" "$LOG_FILE"
    grep -q "$LABEL: ready command=9" "$LOG_FILE"
    grep -q "$LABEL: ready command=10" "$LOG_FILE"
    grep -q "$LABEL: ready command=11" "$LOG_FILE"
    grep -q "$LABEL: ready command=12" "$LOG_FILE"
    grep -q "$LABEL: ready command=13" "$LOG_FILE"
    grep -q "$LABEL: ready command=14" "$LOG_FILE"
    grep -q "$LABEL: ready command=15" "$LOG_FILE"
fi
if [ "$SHELL_DIRECT_PIPELINE_CONSUMER_STDIN_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=5" "$LOG_FILE"
    grep -q "$LABEL: ready command=6" "$LOG_FILE"
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
if [ "$SHELL_BARE_NAME_PIPELINE_STDIN_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=5" "$LOG_FILE"
    grep -q "$LABEL: ready command=6" "$LOG_FILE"
    grep -q "$LABEL: ready command=7" "$LOG_FILE"
    grep -q "$LABEL: ready command=8" "$LOG_FILE"
    grep -q "$LABEL: ready command=9" "$LOG_FILE"
    grep -q "$LABEL: ready command=10" "$LOG_FILE"
    grep -q "$LABEL: ready command=11" "$LOG_FILE"
    grep -q "$LABEL: ready command=12" "$LOG_FILE"
    grep -q "$LABEL: ready command=13" "$LOG_FILE"
    grep -q "$LABEL: ready command=14" "$LOG_FILE"
    grep -q "$LABEL: ready command=15" "$LOG_FILE"
fi
if [ "$SHELL_BARE_NAME_PIPELINE_CONSUMER_STDIN_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=5" "$LOG_FILE"
    grep -q "$LABEL: ready command=6" "$LOG_FILE"
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
if [ "$SHELL_BARE_NAME_PIPELINE_STAGE_ARGV_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=5" "$LOG_FILE"
    grep -q "$LABEL: ready command=6" "$LOG_FILE"
    grep -q "$LABEL: ready command=7" "$LOG_FILE"
    grep -q "$LABEL: ready command=8" "$LOG_FILE"
    grep -q "$LABEL: ready command=9" "$LOG_FILE"
    grep -q "$LABEL: ready command=10" "$LOG_FILE"
    grep -q "$LABEL: ready command=11" "$LOG_FILE"
    grep -q "$LABEL: ready command=12" "$LOG_FILE"
    grep -q "$LABEL: ready command=13" "$LOG_FILE"
    grep -q "$LABEL: ready command=14" "$LOG_FILE"
fi
if [ "$SHELL_PROCESS_STATUS_VFS_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=5" "$LOG_FILE"
    grep -q "$LABEL: ready command=6" "$LOG_FILE"
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
    grep -q "$LABEL: ready command=18" "$LOG_FILE"
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
if [ "$SHELL_STDIO_SMOKE" -eq 1 ]; then
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
if [ "$SHELL_STDOUT_TO_STDERR_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=7" "$LOG_FILE"
    grep -q "$LABEL: ready command=8" "$LOG_FILE"
    grep -q "$LABEL: ready command=9" "$LOG_FILE"
    grep -q "$LABEL: ready command=10" "$LOG_FILE"
    grep -q "$LABEL: ready command=11" "$LOG_FILE"
fi
if [ "$SHELL_DEV_NULL_STDOUT_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=7" "$LOG_FILE"
    grep -q "$LABEL: ready command=8" "$LOG_FILE"
    grep -q "$LABEL: ready command=9" "$LOG_FILE"
    grep -q "$LABEL: ready command=10" "$LOG_FILE"
    grep -q "$LABEL: ready command=11" "$LOG_FILE"
fi
if [ "$SHELL_DEV_NULL_STDERR_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=7" "$LOG_FILE"
    grep -q "$LABEL: ready command=8" "$LOG_FILE"
    grep -q "$LABEL: ready command=9" "$LOG_FILE"
    grep -q "$LABEL: ready command=10" "$LOG_FILE"
    grep -q "$LABEL: ready command=11" "$LOG_FILE"
    grep -q "$LABEL: ready command=12" "$LOG_FILE"
fi
if [ "$SHELL_DEV_NULL_STDIN_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=7" "$LOG_FILE"
    grep -q "$LABEL: ready command=8" "$LOG_FILE"
    grep -q "$LABEL: ready command=9" "$LOG_FILE"
    grep -q "$LABEL: ready command=10" "$LOG_FILE"
    grep -q "$LABEL: ready command=11" "$LOG_FILE"
fi
if [ "$SHELL_READONLY_REGULAR_FILE_STDIN_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=5" "$LOG_FILE"
    grep -q "$LABEL: ready command=6" "$LOG_FILE"
    grep -q "$LABEL: ready command=7" "$LOG_FILE"
    grep -q "$LABEL: ready command=8" "$LOG_FILE"
    grep -q "$LABEL: ready command=9" "$LOG_FILE"
    grep -q "$LABEL: ready command=10" "$LOG_FILE"
    grep -q "$LABEL: ready command=11" "$LOG_FILE"
fi
if [ "$SHELL_DIRECT_STDIN_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=5" "$LOG_FILE"
    grep -q "$LABEL: ready command=6" "$LOG_FILE"
    grep -q "$LABEL: ready command=7" "$LOG_FILE"
    grep -q "$LABEL: ready command=8" "$LOG_FILE"
    grep -q "$LABEL: ready command=9" "$LOG_FILE"
    grep -q "$LABEL: ready command=10" "$LOG_FILE"
    grep -q "$LABEL: ready command=11" "$LOG_FILE"
fi
if [ "$SHELL_DIRECT_COMBINED_STDIN_STDOUT_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=5" "$LOG_FILE"
    grep -q "$LABEL: ready command=6" "$LOG_FILE"
    grep -q "$LABEL: ready command=7" "$LOG_FILE"
    grep -q "$LABEL: ready command=8" "$LOG_FILE"
    grep -q "$LABEL: ready command=9" "$LOG_FILE"
    grep -q "$LABEL: ready command=10" "$LOG_FILE"
    grep -q "$LABEL: ready command=11" "$LOG_FILE"
    grep -q "$LABEL: ready command=12" "$LOG_FILE"
    grep -q "$LABEL: ready command=13" "$LOG_FILE"
fi
if [ "$SHELL_BARE_NAME_COMBINED_STDIN_STDOUT_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=5" "$LOG_FILE"
    grep -q "$LABEL: ready command=6" "$LOG_FILE"
    grep -q "$LABEL: ready command=7" "$LOG_FILE"
    grep -q "$LABEL: ready command=8" "$LOG_FILE"
    grep -q "$LABEL: ready command=9" "$LOG_FILE"
    grep -q "$LABEL: ready command=10" "$LOG_FILE"
    grep -q "$LABEL: ready command=11" "$LOG_FILE"
    grep -q "$LABEL: ready command=12" "$LOG_FILE"
    grep -q "$LABEL: ready command=13" "$LOG_FILE"
    grep -q "$LABEL: ready command=14" "$LOG_FILE"
fi
if [ "$SHELL_BARE_NAME_STDIN_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=5" "$LOG_FILE"
    grep -q "$LABEL: ready command=6" "$LOG_FILE"
    grep -q "$LABEL: ready command=7" "$LOG_FILE"
    grep -q "$LABEL: ready command=8" "$LOG_FILE"
    grep -q "$LABEL: ready command=9" "$LOG_FILE"
    grep -q "$LABEL: ready command=10" "$LOG_FILE"
    grep -q "$LABEL: ready command=11" "$LOG_FILE"
fi
if [ "$SHELL_STDOUT_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=5" "$LOG_FILE"
    grep -q "$LABEL: ready command=6" "$LOG_FILE"
    grep -q "$LABEL: ready command=7" "$LOG_FILE"
    grep -q "$LABEL: ready command=8" "$LOG_FILE"
    grep -q "$LABEL: ready command=9" "$LOG_FILE"
    grep -q "$LABEL: ready command=10" "$LOG_FILE"
    grep -q "$LABEL: ready command=11" "$LOG_FILE"
    grep -q "$LABEL: ready command=12" "$LOG_FILE"
fi
if [ "$SHELL_STDERR_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=5" "$LOG_FILE"
    grep -q "$LABEL: ready command=6" "$LOG_FILE"
    grep -q "$LABEL: ready command=7" "$LOG_FILE"
    grep -q "$LABEL: ready command=8" "$LOG_FILE"
    grep -q "$LABEL: ready command=9" "$LOG_FILE"
    grep -q "$LABEL: ready command=10" "$LOG_FILE"
    grep -q "$LABEL: ready command=11" "$LOG_FILE"
    grep -q "$LABEL: ready command=12" "$LOG_FILE"
    grep -q "$LABEL: ready command=13" "$LOG_FILE"
fi
if [ "$SHELL_STDERR_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=5" "$LOG_FILE"
    grep -q "$LABEL: ready command=6" "$LOG_FILE"
    grep -q "$LABEL: ready command=7" "$LOG_FILE"
    grep -q "$LABEL: ready command=8" "$LOG_FILE"
    grep -q "$LABEL: ready command=9" "$LOG_FILE"
    grep -q "$LABEL: ready command=10" "$LOG_FILE"
    grep -q "$LABEL: ready command=11" "$LOG_FILE"
    grep -q "$LABEL: ready command=12" "$LOG_FILE"
    grep -q "$LABEL: ready command=13" "$LOG_FILE"
fi
if [ "$SHELL_STDOUT_REGULAR_FILE_APPEND_CREATE_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=5" "$LOG_FILE"
    grep -q "$LABEL: ready command=6" "$LOG_FILE"
    grep -q "$LABEL: ready command=7" "$LOG_FILE"
    grep -q "$LABEL: ready command=8" "$LOG_FILE"
    grep -q "$LABEL: ready command=9" "$LOG_FILE"
    grep -q "$LABEL: ready command=10" "$LOG_FILE"
fi
if [ "$SHELL_EXPLICIT_FD1_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=5" "$LOG_FILE"
    grep -q "$LABEL: ready command=6" "$LOG_FILE"
    grep -q "$LABEL: ready command=7" "$LOG_FILE"
    grep -q "$LABEL: ready command=8" "$LOG_FILE"
    grep -q "$LABEL: ready command=9" "$LOG_FILE"
    grep -q "$LABEL: ready command=10" "$LOG_FILE"
    grep -q "$LABEL: ready command=11" "$LOG_FILE"
    grep -q "$LABEL: ready command=12" "$LOG_FILE"
    grep -q "$LABEL: ready command=13" "$LOG_FILE"
fi
if [ "$SHELL_STDERR_REGULAR_FILE_APPEND_CREATE_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=5" "$LOG_FILE"
    grep -q "$LABEL: ready command=6" "$LOG_FILE"
    grep -q "$LABEL: ready command=7" "$LOG_FILE"
    grep -q "$LABEL: ready command=8" "$LOG_FILE"
    grep -q "$LABEL: ready command=9" "$LOG_FILE"
    grep -q "$LABEL: ready command=10" "$LOG_FILE"
    grep -q "$LABEL: ready command=11" "$LOG_FILE"
fi
if [ "$SHELL_STDERR_TO_STDOUT_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=7" "$LOG_FILE"
    grep -q "$LABEL: ready command=8" "$LOG_FILE"
    grep -q "$LABEL: ready command=9" "$LOG_FILE"
    grep -q "$LABEL: ready command=10" "$LOG_FILE"
    grep -q "$LABEL: ready command=11" "$LOG_FILE"
fi
if [ "$SHELL_MINIMAL_STDOUT_TO_STDIN_PIPELINE_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=6" "$LOG_FILE"
fi
if [ "$SHELL_PIPELINE_STDERR_NOT_PIPED_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=6" "$LOG_FILE"
    grep -q "$LABEL: ready command=7" "$LOG_FILE"
fi
if [ "$SHELL_PIPELINE_STDERR_DUP_TO_STDOUT_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=6" "$LOG_FILE"
    grep -q "$LABEL: ready command=7" "$LOG_FILE"
    grep -q "$LABEL: ready command=8" "$LOG_FILE"
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
    grep -q "$LABEL: ready command=18" "$LOG_FILE"
    grep -q "$LABEL: ready command=19" "$LOG_FILE"
    grep -q "$LABEL: ready command=20" "$LOG_FILE"
    grep -q "$LABEL: ready command=21" "$LOG_FILE"
    grep -q "$LABEL: ready command=22" "$LOG_FILE"
    grep -q "$LABEL: ready command=23" "$LOG_FILE"
    grep -q "$LABEL: ready command=24" "$LOG_FILE"
fi
if [ "$SHELL_BACKGROUND_VFS_EXEC_LIFECYCLE_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=7" "$LOG_FILE"
    grep -q "$LABEL: ready command=8" "$LOG_FILE"
    grep -q "$LABEL: ready command=9" "$LOG_FILE"
    grep -q "$LABEL: ready command=10" "$LOG_FILE"
    grep -q "$LABEL: ready command=11" "$LOG_FILE"
    grep -q "$LABEL: ready command=12" "$LOG_FILE"
    grep -q "$LABEL: ready command=13" "$LOG_FILE"
fi
if [ "$SHELL_JOBS_ACCOUNTING_LIST_SMOKE" -eq 1 ]; then
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
if [ "$SHELL_MULTIPLE_BACKGROUND_JOBS_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=7" "$LOG_FILE"
    grep -q "$LABEL: ready command=8" "$LOG_FILE"
    grep -q "$LABEL: ready command=9" "$LOG_FILE"
    grep -q "$LABEL: ready command=10" "$LOG_FILE"
    grep -q "$LABEL: ready command=11" "$LOG_FILE"
    grep -q "$LABEL: ready command=12" "$LOG_FILE"
    grep -q "$LABEL: ready command=13" "$LOG_FILE"
    grep -q "$LABEL: ready command=14" "$LOG_FILE"
fi
if [ "$SHELL_BACKGROUND_JOBS_STALE_ENTRY_POLICY_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: ready command=7" "$LOG_FILE"
    grep -q "$LABEL: ready command=8" "$LOG_FILE"
    grep -q "$LABEL: ready command=9" "$LOG_FILE"
    grep -q "$LABEL: ready command=10" "$LOG_FILE"
    grep -q "$LABEL: ready command=11" "$LOG_FILE"
    grep -q "$LABEL: ready command=12" "$LOG_FILE"
    grep -q "$LABEL: ready command=13" "$LOG_FILE"
    grep -q "$LABEL: ready command=14" "$LOG_FILE"
    grep -q "$LABEL: ready command=15" "$LOG_FILE"
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
grep -q "talos: version phase10.2-kernel-builtins-v2" "$LOG_FILE"
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
if [ "$SHELL_GENERATED_USERLAND_MANIFEST_SMOKE" -eq 1 ]; then
    grep -q "$LABEL: generated-root identity=phase10-generated-root-manifest-v1 source=userland/generated-root.manifest digest=sha256:" "$LOG_FILE"
    grep -q "$LABEL: generated-root hardcoded-src-initramfs-constant=false path=src/initramfs.rs" "$LOG_FILE"
    grep -q "$LABEL: generated-root executable-hardcoded-src-initramfs-constant=false path=src/initramfs.rs" "$LOG_FILE"
    grep -q "$LABEL: generated-root-selection source=$GENERATED_ROOT_EXPECTED_SOURCE reason=$GENERATED_ROOT_EXPECTED_REASON" "$LOG_FILE"
    grep -q "talos: generated-root source=$GENERATED_ROOT_EXPECTED_SOURCE reason=$GENERATED_ROOT_EXPECTED_REASON" "$LOG_FILE"
    grep -q "talos> cat /generated/manifest.txt" "$LOG_FILE"
    grep -q "^$GENERATED_ROOT_EXPECTED_CONTENT" "$LOG_FILE"
    grep -q "$LABEL: line command=3 hex=63 61 74 20 2f 67 65 6e 65 72 61 74 65 64 2f 6d 61 6e 69 66 65 73 74 2e 74 78 74" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=3 status=handled responses=1" "$LOG_FILE"
    grep -q "talos> ls /" "$LOG_FILE"
    grep -q "^generated" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=4 status=handled responses=5" "$LOG_FILE"
    grep -q "talos> cat /etc/banner.txt" "$LOG_FILE"
    grep -q "^Talos initramfs fixture" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=5 status=handled responses=1" "$LOG_FILE"
    grep -q "talos> exec /generated/status7 alpha" "$LOG_FILE"
    grep -q "talos: exec path=/generated/status7 source=vfs-open-read" "$LOG_FILE"
    grep -q "talos: exec-startup-abi state=literal-argv-absolute-empty-envp argc=0x0000000000000002 argv0=/generated/status7 argv1=alpha" "$LOG_FILE"
    grep -q "talos: exec-status boundary=lower-aarch64-svc-status-equivalent marker=0x0000000000007a10 status=$GENERATED_ROOT_EXPECTED_STATUS_HEX complete=true source=lifecycle-record" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=6 status=handled responses=9" "$LOG_FILE"
    grep -q "talos> waitpid" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/generated/status7 state=exited status=$GENERATED_ROOT_EXPECTED_STATUS_HEX observed-status=$GENERATED_ROOT_EXPECTED_STATUS_HEX reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=7 status=handled responses=1" "$LOG_FILE"
    grep -q "talos> laststatus" "$LOG_FILE"
    grep -q "talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/generated/status7 state=exited status=$GENERATED_ROOT_EXPECTED_STATUS_HEX observed-status=$GENERATED_ROOT_EXPECTED_STATUS_HEX reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=8 status=handled responses=1" "$LOG_FILE"
    grep -q "talos> exec /bin/status42" "$LOG_FILE"
    grep -q "talos: exec path=/bin/status42 source=vfs-open-read" "$LOG_FILE"
    grep -q "talos: exec-status boundary=lower-aarch64-svc-status-equivalent marker=0x0000000000007a10 status=0x000000000000002a complete=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos: vfs-exec-lifecycle-status record=phase12-local-vfs-exec-lifecycle-status-record-v2" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=9 status=handled responses=10" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=10 status=handled responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=11 status=handled responses=1" "$LOG_FILE"
    grep -q "talos> exec stdout | exec stdin" "$LOG_FILE"
    grep -q "Talos userspace stdout fixture" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=12 status=handled responses=22" "$LOG_FILE"
    grep -q "talos> jobs" "$LOG_FILE"
    grep -q "talos: jobs none source=background-vfs-exec-accounting" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=13 status=handled responses=1" "$LOG_FILE"
    grep -q "talos> rootinfo" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=14 status=handled responses=1" "$LOG_FILE"
    grep -q "$LABEL: final participants=15 expected=15 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$LINE_EDITING_SMOKE" -eq 1 ]; then
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
elif [ "$SHELL_PIPELINE_STDERR_DUP_TO_STDOUT_SMOKE" -eq 1 ]; then
    grep -Fq "talos> exec stderr 2>&1 | exec stdin" "$LOG_FILE"
    grep -q "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stderr consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000001f bytes-read=0x000000000000001f writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-stderr-dup-to-stdout" "$LOG_FILE"
    grep -q "talos: pipeline-lifecycle-status record=phase12-local-pipeline-distinct-process-lifecycle-status-record-v1 .*producer-pid=0x0000000000100001 .*producer-path=/bin/stderr .*consumer-pid=0x0000000000100002 .*consumer-path=/bin/stdin .*source=kernel-owned-pipeline-lifecycle-status-record" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=pipe-endpoint fd2=pipe-endpoint loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -q "talos: exec-redirection op=dup source-fd=0x0000000000000002 target-fd=0x0000000000000001 target-stream=pipe-writer target-route=pipe:stdout-to-stdin child-only=true shell-restored=true source=shell-redirection-2-to-1" "$LOG_FILE"
    grep -q "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=pipe-writer route=pipe:stdout-to-stdin source=userspace-talos-write" "$LOG_FILE"
    grep -q "^Talos userspace stdin fixture read: Talos userspace stderr fixture" "$LOG_FILE"
    grep -q "talos: exec-stdin fd=0x0000000000000000 bytes=0x000000000000001f return=0x000000000000001f read-source=pipe:stdout-to-stdin stdout-fd=0x0000000000000001 stdout-bytes=0x0000000000000044 stdout-return=0x0000000000000044 source=userspace-talos-read+userspace-talos-write read-result=pipe-eof-after-writer-close" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=3 status=handled responses=23" "$LOG_FILE"
    grep -Eq "talos: waitpid pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -Eq "talos: last-process pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -Fq "talos> exec stderr | exec stdin" "$LOG_FILE"
    grep -q "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stderr consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x0000000000000000 bytes-read=0x0000000000000000 writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-stdout-only-stderr-not-piped" "$LOG_FILE"
    grep -Fq "talos> exec stdout | exec stdin" "$LOG_FILE"
    grep -q "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdout consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000001f bytes-read=0x000000000000001f writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-stdout-to-stdin" "$LOG_FILE"
    grep -q "talos> cat /etc/banner.txt" "$LOG_FILE"
    grep -q "^Talos initramfs fixture" "$LOG_FILE"
    grep -q "$LABEL: final participants=9 expected=9 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_PIPELINE_STDOUT_REDIRECT_AWAY_SMOKE" -eq 1 ]; then
    grep -Fq "talos> exec stdout 1>&2 | exec stdin" "$LOG_FILE"
    grep -q "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdout consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x0000000000000000 bytes-read=0x0000000000000000 writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-stdout-redirect-away" "$LOG_FILE"
    grep -q "talos: pipeline-lifecycle-status record=phase12-local-pipeline-distinct-process-lifecycle-status-record-v1 .*producer-pid=0x0000000000100001 .*producer-path=/bin/stdout .*consumer-pid=0x0000000000100002 .*consumer-path=/bin/stdin .*source=kernel-owned-pipeline-lifecycle-status-record" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -q "talos: exec-redirection op=dup source-fd=0x0000000000000001 target-fd=0x0000000000000002 target-stream=stderr target-route=runtime-console0/stderr child-only=true shell-restored=true source=shell-redirection-1-to-2" "$LOG_FILE"
    grep -q "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=stderr route=runtime-console0/stderr source=userspace-talos-write" "$LOG_FILE"
    grep -q "^Talos userspace stdin fixture read-result: pipe-eof/no-data" "$LOG_FILE"
    grep -q "talos: exec-stdin fd=0x0000000000000000 bytes=0x0000000000000000 return=0x0000000000000000 read-source=pipe:stdout-to-stdin stdout-fd=0x0000000000000001 stdout-bytes=0x000000000000003c stdout-return=0x000000000000003c source=userspace-talos-read+userspace-talos-write read-result=pipe-eof/no-data" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=3 status=handled responses=23" "$LOG_FILE"
    grep -Eq "talos: waitpid pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -Eq "talos: last-process pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -Fq "talos> exec stderr 2>&1 | exec stdin" "$LOG_FILE"
    grep -q "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stderr consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000001f bytes-read=0x000000000000001f writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-stderr-dup-to-stdout" "$LOG_FILE"
    grep -Fq "talos> exec stdout | exec stdin" "$LOG_FILE"
    grep -q "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdout consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000001f bytes-read=0x000000000000001f writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-stdout-to-stdin" "$LOG_FILE"
    grep -q "talos: pipeline-lifecycle-status record=phase12-local-pipeline-distinct-process-lifecycle-status-record-v1 .*producer-pid=0x0000000000100001 .*producer-path=/bin/stdout .*consumer-pid=0x0000000000100002 .*consumer-path=/bin/stdin .*source=kernel-owned-pipeline-lifecycle-status-record" "$LOG_FILE"
    grep -q "talos> cat /etc/banner.txt" "$LOG_FILE"
    grep -q "^Talos initramfs fixture" "$LOG_FILE"
    grep -q "$LABEL: final participants=9 expected=9 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_PIPELINE_STDERR_NOT_PIPED_SMOKE" -eq 1 ]; then
    grep -Fq "talos> exec stderr | exec stdin" "$LOG_FILE"
    grep -q "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stderr consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x0000000000000000 bytes-read=0x0000000000000000 writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-stdout-only-stderr-not-piped" "$LOG_FILE"
    grep -q "talos: pipeline-lifecycle-status record=phase12-local-pipeline-distinct-process-lifecycle-status-record-v1 .*producer-pid=0x0000000000100001 .*producer-path=/bin/stderr .*consumer-pid=0x0000000000100002 .*consumer-path=/bin/stdin .*source=kernel-owned-pipeline-lifecycle-status-record" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=pipe-endpoint fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -q "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=stderr route=runtime-console0/stderr source=userspace-talos-write" "$LOG_FILE"
    grep -q "^Talos userspace stderr fixture" "$LOG_FILE"
    grep -q "^Talos userspace stdin fixture read-result: pipe-eof/no-data" "$LOG_FILE"
    grep -q "talos: exec-stdin fd=0x0000000000000000 bytes=0x0000000000000000 return=0x0000000000000000 read-source=pipe:stdout-to-stdin stdout-fd=0x0000000000000001 stdout-bytes=0x000000000000003c stdout-return=0x000000000000003c source=userspace-talos-read+userspace-talos-write read-result=pipe-eof/no-data" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=3 status=handled responses=22" "$LOG_FILE"
    grep -Eq "talos: waitpid pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -Eq "talos: last-process pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -Fq "talos> exec stdout | exec stdin" "$LOG_FILE"
    grep -q "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdout consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000001f bytes-read=0x000000000000001f writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-stdout-to-stdin" "$LOG_FILE"
    grep -q "talos: pipeline-lifecycle-status record=phase12-local-pipeline-distinct-process-lifecycle-status-record-v1 .*producer-pid=0x0000000000100001 .*producer-path=/bin/stdout .*consumer-pid=0x0000000000100002 .*consumer-path=/bin/stdin .*source=kernel-owned-pipeline-lifecycle-status-record" "$LOG_FILE"
    grep -q "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=pipe-writer route=pipe:stdout-to-stdin source=userspace-talos-write" "$LOG_FILE"
    grep -q "^Talos userspace stdin fixture read: Talos userspace stdout fixture" "$LOG_FILE"
    grep -q "talos> cat /etc/banner.txt" "$LOG_FILE"
    grep -q "^Talos initramfs fixture" "$LOG_FILE"
    grep -q "$LABEL: final participants=8 expected=8 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_MINIMAL_STDOUT_TO_STDIN_PIPELINE_SMOKE" -eq 1 ]; then
    grep -Fq "talos> exec stdout | exec stdin" "$LOG_FILE"
    grep -q "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdout consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000001f bytes-read=0x000000000000001f writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-stdout-to-stdin" "$LOG_FILE"
    grep -q "talos: pipeline-lifecycle-status record=phase12-local-pipeline-distinct-process-lifecycle-status-record-v1 .*producer-pid=0x0000000000100001 .*producer-path=/bin/stdout .*consumer-pid=0x0000000000100002 .*consumer-path=/bin/stdin .*source=kernel-owned-pipeline-lifecycle-status-record" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=pipe-endpoint fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -q "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=pipe-writer route=pipe:stdout-to-stdin source=userspace-talos-write" "$LOG_FILE"
    grep -q "^Talos userspace stdin fixture read: Talos userspace stdout fixture" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=pipe-endpoint fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -q "talos: exec-stdin fd=0x0000000000000000 bytes=0x000000000000001f return=0x000000000000001f read-source=pipe:stdout-to-stdin stdout-fd=0x0000000000000001 stdout-bytes=0x0000000000000044 stdout-return=0x0000000000000044 source=userspace-talos-read+userspace-talos-write read-result=pipe-eof-after-writer-close" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=3 status=handled responses=22" "$LOG_FILE"
    grep -Eq "talos: waitpid pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -Eq "talos: last-process pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos> cat /etc/banner.txt" "$LOG_FILE"
    grep -q "^Talos initramfs fixture" "$LOG_FILE"
    grep -q "$LABEL: final participants=7 expected=7 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_DEV_NULL_STDOUT_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> exec stdout >/dev/null" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stdout source=vfs-open-read" "$LOG_FILE"
    grep -Eq "talos: exec-source bytes=0x[0-9a-f]+ digest=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-loader fixture=phase8-program-loader-elf64-aarch64-v1 entry=0x[0-9a-f]+ segments=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-launch launch-boundary=phase8-initial-process-launch-plan-v1 stack-boundary=phase8-initial-user-stack-plan-v1 address-space=0x[0-9a-f]+ materialization=0x[0-9a-f]+ initial-sp=0x[0-9a-f]+" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=device fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -Eq "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stdout argv0-ptr=0x[0-9a-f]+ argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x[0-9a-f]+ copied-startup-bytes=0x000000000000002c source=initial-user-stack-record" "$LOG_FILE"
    grep -q "talos: exec-redirection op=sink source-fd=0x0000000000000001 target-path=/dev/null target-stream=null-sink target-route=device:/dev/null child-only=true shell-restored=true source=shell-redirection-stdout-dev-null" "$LOG_FILE"
    grep -q "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=null-sink route=device:/dev/null source=userspace-talos-write" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=3 status=handled responses=11" "$LOG_FILE"
    grep -Eq "talos: waitpid pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -Eq "talos: last-process pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos> exec stdout" "$LOG_FILE"
    grep -c "Talos userspace stdout fixture" "$LOG_FILE" | grep -q "^1$"
    grep -q "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=stdout route=runtime-console0/stdout source=userspace-talos-write" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=6 status=handled responses=10" "$LOG_FILE"
    grep -q "talos> exec stdout 1>/dev/null" "$LOG_FILE"
    grep -q "talos> exec stdout 1>file" "$LOG_FILE"
    grep -Fq "talos> exec stdout | stderr" "$LOG_FILE"
    grep -c "talos: exec-invalid-path" "$LOG_FILE" | grep -q "^3$"
    grep -q "talos> cat /etc/banner.txt" "$LOG_FILE"
    grep -q "^Talos initramfs fixture" "$LOG_FILE"
    grep -q "$LABEL: final participants=12 expected=12 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_DEV_NULL_STDERR_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> exec stderr 2>/dev/null" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stderr source=vfs-open-read" "$LOG_FILE"
    grep -Eq "talos: exec-source bytes=0x[0-9a-f]+ digest=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-loader fixture=phase8-program-loader-elf64-aarch64-v1 entry=0x[0-9a-f]+ segments=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-launch launch-boundary=phase8-initial-process-launch-plan-v1 stack-boundary=phase8-initial-user-stack-plan-v1 address-space=0x[0-9a-f]+ materialization=0x[0-9a-f]+ initial-sp=0x[0-9a-f]+" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=stdio-output fd2=device loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -Eq "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stderr argv0-ptr=0x[0-9a-f]+ argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x[0-9a-f]+ copied-startup-bytes=0x000000000000002c source=initial-user-stack-record" "$LOG_FILE"
    grep -q "talos: exec-redirection op=sink source-fd=0x0000000000000002 target-path=/dev/null target-stream=null-sink target-route=device:/dev/null child-only=true shell-restored=true source=shell-redirection-stderr-dev-null" "$LOG_FILE"
    grep -q "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=null-sink route=device:/dev/null source=userspace-talos-write" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=3 status=handled responses=11" "$LOG_FILE"
    grep -Eq "talos: waitpid pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -Eq "talos: last-process pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos> exec stderr" "$LOG_FILE"
    grep -c "Talos userspace stderr fixture" "$LOG_FILE" | grep -q "^1$"
    grep -q "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=stderr route=runtime-console0/stderr source=userspace-talos-write" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=6 status=handled responses=10" "$LOG_FILE"
    grep -q "talos> exec stdout >/dev/null" "$LOG_FILE"
    grep -q "talos: exec-redirection op=sink source-fd=0x0000000000000001 target-path=/dev/null target-stream=null-sink target-route=device:/dev/null child-only=true shell-restored=true source=shell-redirection-stdout-dev-null" "$LOG_FILE"
    grep -q "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=null-sink route=device:/dev/null source=userspace-talos-write" "$LOG_FILE"
    grep -q "talos> exec stderr 2>file" "$LOG_FILE"
    grep -q "talos> exec stderr 2>>/dev/null" "$LOG_FILE"
    grep -q "talos> exec stderr </dev/null" "$LOG_FILE"
    grep -c "talos: exec-invalid-path" "$LOG_FILE" | grep -q "^3$"
    grep -q "talos> cat /etc/banner.txt" "$LOG_FILE"
    grep -q "^Talos initramfs fixture" "$LOG_FILE"
    grep -q "$LABEL: final participants=13 expected=13 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_DEV_NULL_STDIN_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> exec stdin </dev/null" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stdin source=vfs-open-read" "$LOG_FILE"
    grep -Eq "talos: exec-source bytes=0x[0-9a-f]+ digest=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-loader fixture=phase8-program-loader-elf64-aarch64-v1 entry=0x[0-9a-f]+ segments=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-launch launch-boundary=phase8-initial-process-launch-plan-v1 stack-boundary=phase8-initial-user-stack-plan-v1 address-space=0x[0-9a-f]+ materialization=0x[0-9a-f]+ initial-sp=0x[0-9a-f]+" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=device fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -Eq "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stdin argv0-ptr=0x[0-9a-f]+ argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x[0-9a-f]+ copied-startup-bytes=0x000000000000002b source=initial-user-stack-record" "$LOG_FILE"
    grep -q "talos: exec-redirection op=source source-fd=0x0000000000000000 source-path=/dev/null source-stream=null-source source-route=device:/dev/null child-only=true shell-restored=true source=shell-redirection-stdin-dev-null" "$LOG_FILE"
    grep -q "^Talos userspace stdin fixture read-result: null-source-eof/no-data" "$LOG_FILE"
    grep -q "talos: exec-stdin fd=0x0000000000000000 bytes=0x0000000000000000 return=0x0000000000000000 read-source=device:/dev/null stdout-fd=0x0000000000000001 stdout-bytes=0x0000000000000043 stdout-return=0x0000000000000043 source=userspace-talos-read+userspace-talos-write read-result=null-source-eof/no-data" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=3 status=handled responses=11" "$LOG_FILE"
    grep -Eq "talos: waitpid pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -Eq "talos: last-process pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos> exec stdin" "$LOG_FILE"
    grep -q "^Talos userspace stdin fixture read: talos-console0" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -q "talos: exec-stdin fd=0x0000000000000000 bytes=0x000000000000000e return=0x000000000000000e read-source=runtime-console0/local-input stdout-fd=0x0000000000000001 stdout-bytes=0x0000000000000033 stdout-return=0x0000000000000033 source=userspace-talos-read+userspace-talos-write" "$LOG_FILE"
    grep -q "talos> exec stdout </dev/null" "$LOG_FILE"
    grep -q "talos> exec stdin </etc/missing.txt" "$LOG_FILE"
    grep -q "talos> exec stdin < /dev/null" "$LOG_FILE"
    grep -c "talos: exec-invalid-path" "$LOG_FILE" | grep -q "^3$"
    grep -q "talos> cat /etc/banner.txt" "$LOG_FILE"
    grep -q "^Talos initramfs fixture" "$LOG_FILE"
    grep -q "$LABEL: final participants=12 expected=12 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_DIRECT_STDIN_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> /bin/stdin </etc/banner.txt" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stdin source=vfs-open-read" "$LOG_FILE"
    grep -Eq "talos: exec-source bytes=0x[0-9a-f]+ digest=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-loader fixture=phase8-program-loader-elf64-aarch64-v1 entry=0x[0-9a-f]+ segments=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-launch launch-boundary=phase8-initial-process-launch-plan-v1 stack-boundary=phase8-initial-user-stack-plan-v1 address-space=0x[0-9a-f]+ materialization=0x[0-9a-f]+ initial-sp=0x[0-9a-f]+" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=regular-file fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -Eq "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stdin argv0-ptr=0x[0-9a-f]+ argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x[0-9a-f]+ copied-startup-bytes=0x000000000000002b source=initial-user-stack-record" "$LOG_FILE"
    grep -q "talos: exec-redirection op=source source-fd=0x0000000000000000 source-path=/etc/banner.txt source-stream=regular-file source-route=initramfs:/etc/banner.txt child-only=true shell-restored=true source=shell-redirection-stdin-etc-banner" "$LOG_FILE"
    grep -q "^Talos userspace stdin fixture read: Talos initramfs fixture" "$LOG_FILE"
    grep -q "talos: exec-stdin fd=0x0000000000000000 bytes=0x0000000000000018 return=0x0000000000000018 read-source=initramfs:/etc/banner.txt stdout-fd=0x0000000000000001 stdout-bytes=0x000000000000003d stdout-return=0x000000000000003d source=userspace-talos-read+userspace-talos-write read-result=regular-file-eof-after-read" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=3 status=handled responses=11" "$LOG_FILE"
    grep -Eq "talos: waitpid pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -Eq "talos: last-process pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos: pipestatus none source=bounded-process-table-pipeline-status" "$LOG_FILE"
    grep -q "talos> cat /proc/talos/processes" "$LOG_FILE"
    grep -q "path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table" "$LOG_FILE"
    grep -q "talos> ps" "$LOG_FILE"
    grep -q "talos> /bin/stdout </etc/banner.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin </dev/null" "$LOG_FILE"
    grep -q "talos> /bin/stdin < /etc/banner.txt" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=9 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=10 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=11 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -c "talos: exec-invalid-path" "$LOG_FILE" | grep -q "^3$"
    grep -q "$LABEL: final participants=12 expected=12 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_DIRECT_COMBINED_STDIN_STDOUT_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> /bin/stdin </etc/banner.txt >/tmp/stdin-report.txt" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stdin source=vfs-open-read" "$LOG_FILE"
    grep -Eq "talos: exec-source bytes=0x[0-9a-f]+ digest=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-loader fixture=phase8-program-loader-elf64-aarch64-v1 entry=0x[0-9a-f]+ segments=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-launch launch-boundary=phase8-initial-process-launch-plan-v1 stack-boundary=phase8-initial-user-stack-plan-v1 address-space=0x[0-9a-f]+ materialization=0x[0-9a-f]+ initial-sp=0x[0-9a-f]+" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=regular-file fd1=regular-file fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -Eq "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stdin argv0-ptr=0x[0-9a-f]+ argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x[0-9a-f]+ copied-startup-bytes=0x000000000000002b source=initial-user-stack-record" "$LOG_FILE"
    grep -q "talos: exec-redirection op=source source-fd=0x0000000000000000 source-path=/etc/banner.txt source-stream=regular-file source-route=initramfs:/etc/banner.txt child-only=true shell-restored=true source=shell-redirection-stdin-etc-banner" "$LOG_FILE"
    grep -q "talos: exec-redirection op=sink source-fd=0x0000000000000001 target-path=/tmp/stdin-report.txt target-stream=regular-file target-route=volatile-vfs:/tmp/stdin-report.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout" "$LOG_FILE"
    grep -q "talos: exec-stdin fd=0x0000000000000000 bytes=0x0000000000000018 return=0x0000000000000018 read-source=initramfs:/etc/banner.txt stdout-fd=0x0000000000000001 stdout-bytes=0x000000000000003d stdout-return=0x000000000000003d source=userspace-talos-read+userspace-talos-write read-result=regular-file-eof-after-read" "$LOG_FILE"
    grep -q "Talos userspace stdin fixture read: Talos initramfs fixture" "$LOG_FILE"
    grep -q "talos: cat path=/tmp/stdin-report.txt bytes=0x000000000000003d source=volatile-vfs-descriptor-read" "$LOG_FILE"
    grep -q "talos> /bin/stdin >/tmp/stdin-report.txt </etc/banner.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin </dev/null >/tmp/stdin-report.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt 1>/tmp/stdin-report.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin < /etc/banner.txt >/tmp/stdin-report.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt >>/tmp/stdin-report.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt 2>/tmp/stdin-report.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt >/tmp/other.txt" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=7 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=8 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=9 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=10 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=11 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=12 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=13 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -c "talos: exec-invalid-path" "$LOG_FILE" | grep -q "^7$"
    grep -q "$LABEL: final participants=14 expected=14 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_BARE_NAME_COMBINED_STDIN_STDOUT_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> stdin </etc/banner.txt >/tmp/stdin-report.txt" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stdin source=vfs-open-read" "$LOG_FILE"
    grep -Eq "talos: exec-source bytes=0x[0-9a-f]+ digest=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-loader fixture=phase8-program-loader-elf64-aarch64-v1 entry=0x[0-9a-f]+ segments=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-launch launch-boundary=phase8-initial-process-launch-plan-v1 stack-boundary=phase8-initial-user-stack-plan-v1 address-space=0x[0-9a-f]+ materialization=0x[0-9a-f]+ initial-sp=0x[0-9a-f]+" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=regular-file fd1=regular-file fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -Eq "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stdin argv0-ptr=0x[0-9a-f]+ argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x[0-9a-f]+ copied-startup-bytes=0x000000000000002b source=initial-user-stack-record" "$LOG_FILE"
    grep -q "talos: exec-redirection op=source source-fd=0x0000000000000000 source-path=/etc/banner.txt source-stream=regular-file source-route=initramfs:/etc/banner.txt child-only=true shell-restored=true source=shell-redirection-stdin-etc-banner" "$LOG_FILE"
    grep -q "talos: exec-redirection op=sink source-fd=0x0000000000000001 target-path=/tmp/stdin-report.txt target-stream=regular-file target-route=volatile-vfs:/tmp/stdin-report.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout" "$LOG_FILE"
    grep -q "talos: exec-stdin fd=0x0000000000000000 bytes=0x0000000000000018 return=0x0000000000000018 read-source=initramfs:/etc/banner.txt stdout-fd=0x0000000000000001 stdout-bytes=0x000000000000003d stdout-return=0x000000000000003d source=userspace-talos-read+userspace-talos-write read-result=regular-file-eof-after-read" "$LOG_FILE"
    grep -q "Talos userspace stdin fixture read: Talos initramfs fixture" "$LOG_FILE"
    grep -q "talos: cat path=/tmp/stdin-report.txt bytes=0x000000000000003d source=volatile-vfs-descriptor-read" "$LOG_FILE"
    grep -q "talos> stdin >/tmp/stdin-report.txt </etc/banner.txt" "$LOG_FILE"
    grep -q "talos> stdin </dev/null >/tmp/stdin-report.txt" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt 1>/tmp/stdin-report.txt" "$LOG_FILE"
    grep -q "talos> stdin < /etc/banner.txt >/tmp/stdin-report.txt" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt >>/tmp/stdin-report.txt" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt 2>/tmp/stdin-report.txt" "$LOG_FILE"
    grep -q "talos> stdout </etc/banner.txt >/tmp/stdin-report.txt" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt >/tmp/other.txt" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=7 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=8 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=9 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=10 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=11 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=12 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=13 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=14 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -c "talos: exec-invalid-path" "$LOG_FILE" | grep -q "^9$"
    grep -q "$LABEL: final participants=15 expected=15 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_DIRECT_COMBINED_PIPELINE_STDIN_STDOUT_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> /bin/stdin </etc/banner.txt | /bin/stdin >/tmp/pipeline-combined.txt" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stdin source=vfs-open-read" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=regular-file fd1=pipe-endpoint fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=pipe-endpoint fd1=regular-file fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -q "talos: exec-redirection op=source source-fd=0x0000000000000000 source-path=/etc/banner.txt source-stream=regular-file source-route=initramfs:/etc/banner.txt child-only=true shell-restored=true source=shell-redirection-stdin-etc-banner" "$LOG_FILE"
    grep -q "talos: exec-redirection op=sink source-fd=0x0000000000000001 target-path=/tmp/pipeline-combined.txt target-stream=regular-file target-route=volatile-vfs:/tmp/pipeline-combined.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout" "$LOG_FILE"
    grep -q "talos: exec-stdin fd=0x0000000000000000 bytes=0x0000000000000018 return=0x0000000000000018 read-source=initramfs:/etc/banner.txt stdout-fd=0x0000000000000001 stdout-bytes=0x000000000000003d stdout-return=0x000000000000003d source=userspace-talos-read+userspace-talos-write read-result=regular-file-eof-after-read" "$LOG_FILE"
    grep -q "talos: exec-stdin fd=0x0000000000000000 bytes=0x000000000000003d return=0x000000000000003d read-source=pipe:stdout-to-stdin stdout-fd=0x0000000000000001 stdout-bytes=0x0000000000000062 stdout-return=0x0000000000000062 source=userspace-talos-read+userspace-talos-write read-result=pipe-eof-after-writer-close" "$LOG_FILE"
    grep -q "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdin consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000003d bytes-read=0x000000000000003d writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-producer-stdin-consumer-stdout-redirection" "$LOG_FILE"
    grep -q "talos: cat path=/tmp/pipeline-combined.txt bytes=0x0000000000000062 source=volatile-vfs-descriptor-read" "$LOG_FILE"
    grep -q "^Talos userspace stdin fixture read: Talos userspace stdin fixture read: Talos initramfs fixture" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos: pipestatus participants=0x0000000000000002 default-status=0x0000000000000000 pipefail-status=0x0000000000000000 semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt | /bin/stdin >>/tmp/pipeline-combined.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt | /bin/stdin >/tmp/pipeline-report.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdout | /bin/stdin >/tmp/pipeline-combined.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt | /bin/stdin 1>/tmp/pipeline-combined.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt | /bin/stdin > /tmp/pipeline-combined.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt | /bin/stdin >/var/x" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=11 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=12 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=13 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=14 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=15 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=16 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -c "talos: exec-invalid-path" "$LOG_FILE" | grep -q "^6$"
    grep -q "$LABEL: final participants=17 expected=17 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_DIRECT_COMBINED_PIPELINE_STDOUT_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> /bin/stdin </etc/banner.txt | /bin/stdin >/tmp/pipeline-combined-append.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt | /bin/stdin >>/tmp/pipeline-combined-append.txt" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stdin source=vfs-open-read" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=regular-file fd1=pipe-endpoint fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=pipe-endpoint fd1=regular-file fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -q "talos: exec-redirection op=source source-fd=0x0000000000000000 source-path=/etc/banner.txt source-stream=regular-file source-route=initramfs:/etc/banner.txt child-only=true shell-restored=true source=shell-redirection-stdin-etc-banner" "$LOG_FILE"
    grep -q "talos: exec-redirection op=sink source-fd=0x0000000000000001 target-path=/tmp/pipeline-combined-append.txt target-stream=regular-file target-route=volatile-vfs:/tmp/pipeline-combined-append.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout" "$LOG_FILE"
    grep -q "talos: exec-redirection op=append source-fd=0x0000000000000001 target-path=/tmp/pipeline-combined-append.txt target-stream=regular-file target-route=volatile-vfs:/tmp/pipeline-combined-append.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout-append" "$LOG_FILE"
    grep -q "source=shell-pipe-producer-stdin-consumer-stdout-redirection" "$LOG_FILE"
    grep -q "source=shell-pipe-producer-stdin-consumer-stdout-append-redirection" "$LOG_FILE"
    grep -q "talos: cat path=/tmp/pipeline-combined-append.txt bytes=0x00000000000000c4 source=volatile-vfs-descriptor-read" "$LOG_FILE"
    grep -c "Talos userspace stdin fixture read: Talos userspace stdin fixture read: Talos initramfs fixture" "$LOG_FILE" | grep -q "^2$"
    grep -q "talos> /bin/stdin </etc/banner.txt" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos: pipestatus participants=0x0000000000000002 default-status=0x0000000000000000 pipefail-status=0x0000000000000000 semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt | /bin/stdin >>/tmp/pipeline-combined-append.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt | /bin/stdin 2>>/tmp/pipeline-combined-append.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt | /bin/stdin >/tmp/pipeline-report.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdout | /bin/stdin >>/tmp/pipeline-combined-append.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt | /bin/stdin 1>/tmp/pipeline-combined-append.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt | /bin/stdin > /tmp/pipeline-combined-append.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt | /bin/stdin >>/var/x" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt | /bin/stdin >>/tmp/pipeline-combined-append.txt | /bin/stdin" "$LOG_FILE"
    grep -q "talos> missing </etc/banner.txt | /bin/stdin >>/tmp/pipeline-combined-append.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt | missing >>/tmp/pipeline-combined-append.txt" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=13 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=14 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=15 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=16 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=17 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=18 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=19 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=20 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=21 status=unknown-command responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=22 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -c "talos: exec-invalid-path" "$LOG_FILE" | grep -q "^8$"
    grep -c "talos: input-error line-complete" "$LOG_FILE" | grep -q "^1$"
    grep -q "$LABEL: final participants=23 expected=23 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_DIRECT_COMBINED_PIPELINE_STDERR_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> /bin/stdin </etc/banner.txt | /bin/stderr 2>/tmp/pipeline-combined-stderr.txt" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stdin source=vfs-open-read" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stderr source=vfs-open-read" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=regular-file fd1=pipe-endpoint fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=pipe-endpoint fd1=stdio-output fd2=regular-file loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -q "talos: exec-redirection op=source source-fd=0x0000000000000000 source-path=/etc/banner.txt source-stream=regular-file source-route=initramfs:/etc/banner.txt child-only=true shell-restored=true source=shell-redirection-stdin-etc-banner" "$LOG_FILE"
    grep -q "talos: exec-redirection op=sink source-fd=0x0000000000000002 target-path=/tmp/pipeline-combined-stderr.txt target-stream=regular-file target-route=volatile-vfs:/tmp/pipeline-combined-stderr.txt child-only=true shell-restored=true source=shell-redirection-stderr-tmp-stderr" "$LOG_FILE"
    grep -q "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=regular-file route=volatile-vfs:/tmp/pipeline-combined-stderr.txt source=userspace-talos-write" "$LOG_FILE"
    grep -q "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdin consumer-fd=0x0000000000000000 consumer-path=/bin/stderr bytes-written=0x000000000000003d bytes-read=0x0000000000000000 writer-closed=true reader-eof=false shell-restored=true source=shell-pipe-producer-stdin-consumer-stderr-redirection" "$LOG_FILE"
    grep -q "talos: cat path=/tmp/pipeline-combined-stderr.txt bytes=0x000000000000001f source=volatile-vfs-descriptor-read" "$LOG_FILE"
    grep -q "Talos userspace stdin fixture read: Talos initramfs fixture" "$LOG_FILE"
    grep -q "Talos userspace stdout fixture" "$LOG_FILE"
    grep -q "Talos userspace stderr fixture" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos: pipestatus participants=0x0000000000000002 default-status=0x0000000000000000 pipefail-status=0x0000000000000000 semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt | /bin/stderr 2>/tmp/pipeline-combined-stderr.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt | /bin/stderr 2>>/tmp/pipeline-combined-stderr.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt | /bin/stderr 2>/tmp/pipeline-stderr.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdout | /bin/stderr 2>/tmp/pipeline-combined-stderr.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt | /bin/stderr 2> /tmp/pipeline-combined-stderr.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt | /bin/stderr 2>/var/x" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt | /bin/stderr 2>/tmp/pipeline-combined-stderr.txt | /bin/stdin" "$LOG_FILE"
    grep -q "talos> missing </etc/banner.txt | /bin/stderr 2>/tmp/pipeline-combined-stderr.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt | missing 2>/tmp/pipeline-combined-stderr.txt" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=14 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=15 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=16 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=17 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=18 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=19 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=20 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=21 status=unknown-command responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=22 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -c "talos: exec-invalid-path" "$LOG_FILE" | grep -q "^8$"
    grep -q "$LABEL: final participants=23 expected=23 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_DIRECT_COMBINED_PIPELINE_STDERR_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> /bin/stdin </etc/banner.txt | /bin/stderr 2>/tmp/pipeline-combined-stderr-append.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt | /bin/stderr 2>>/tmp/pipeline-combined-stderr-append.txt" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stdin source=vfs-open-read" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stderr source=vfs-open-read" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=regular-file fd1=pipe-endpoint fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=pipe-endpoint fd1=stdio-output fd2=regular-file loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -q "talos: exec-redirection op=source source-fd=0x0000000000000000 source-path=/etc/banner.txt source-stream=regular-file source-route=initramfs:/etc/banner.txt child-only=true shell-restored=true source=shell-redirection-stdin-etc-banner" "$LOG_FILE"
    grep -q "talos: exec-redirection op=sink source-fd=0x0000000000000002 target-path=/tmp/pipeline-combined-stderr-append.txt target-stream=regular-file target-route=volatile-vfs:/tmp/pipeline-combined-stderr-append.txt child-only=true shell-restored=true source=shell-redirection-stderr-tmp-stderr" "$LOG_FILE"
    grep -q "talos: exec-redirection op=append source-fd=0x0000000000000002 target-path=/tmp/pipeline-combined-stderr-append.txt target-stream=regular-file target-route=volatile-vfs:/tmp/pipeline-combined-stderr-append.txt child-only=true shell-restored=true source=shell-redirection-stderr-tmp-stderr-append" "$LOG_FILE"
    grep -q "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=regular-file route=volatile-vfs:/tmp/pipeline-combined-stderr-append.txt source=userspace-talos-write" "$LOG_FILE"
    grep -q "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdin consumer-fd=0x0000000000000000 consumer-path=/bin/stderr bytes-written=0x000000000000003d bytes-read=0x0000000000000000 writer-closed=true reader-eof=false shell-restored=true source=shell-pipe-producer-stdin-consumer-stderr-redirection" "$LOG_FILE"
    grep -q "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdin consumer-fd=0x0000000000000000 consumer-path=/bin/stderr bytes-written=0x000000000000003d bytes-read=0x0000000000000000 writer-closed=true reader-eof=false shell-restored=true source=shell-pipe-producer-stdin-consumer-stderr-append-redirection" "$LOG_FILE"
    grep -q "talos: cat path=/tmp/pipeline-combined-stderr-append.txt bytes=0x000000000000003e source=volatile-vfs-descriptor-read" "$LOG_FILE"
    grep -q "Talos userspace stdin fixture read: Talos initramfs fixture" "$LOG_FILE"
    grep -c "Talos userspace stderr fixture" "$LOG_FILE" | grep -q "^3$"
    grep -q "talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos: pipestatus participants=0x0000000000000002 default-status=0x0000000000000000 pipefail-status=0x0000000000000000 semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt | /bin/stderr 2>>/tmp/pipeline-combined-stderr-append.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt | /bin/stderr 2>>/tmp/pipeline-combined-stderr.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt | /bin/stderr 2>/tmp/pipeline-stderr.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdout | /bin/stderr 2>>/tmp/pipeline-combined-stderr-append.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt | /bin/stderr 2>> /tmp/pipeline-combined-stderr-append.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt | /bin/stderr 1>/tmp/pipeline-combined-stderr-append.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt | /bin/stderr 2>>/var/x" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt | /bin/stderr 2>>/tmp/pipeline-combined-stderr-append.txt | /bin/stdin" "$LOG_FILE"
    grep -q "talos> missing </etc/banner.txt | /bin/stderr 2>>/tmp/pipeline-combined-stderr-append.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt | missing 2>>/tmp/pipeline-combined-stderr-append.txt" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=14 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=15 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=16 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=17 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=18 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=19 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=20 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=21 status=input-error responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=22 status=unknown-command responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=23 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -c "talos: exec-invalid-path" "$LOG_FILE" | grep -q "^9$"
    grep -q "$LABEL: final participants=24 expected=24 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_BARE_NAME_COMBINED_PIPELINE_STDERR_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> stdin </etc/banner.txt | stderr 2>/tmp/pipeline-combined-stderr-append.txt" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt | stderr 2>>/tmp/pipeline-combined-stderr-append.txt" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stdin source=vfs-open-read" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stderr source=vfs-open-read" "$LOG_FILE"
    grep -q "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stdin" "$LOG_FILE"
    grep -q "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stderr" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=regular-file fd1=pipe-endpoint fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=pipe-endpoint fd1=stdio-output fd2=regular-file loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -q "talos: exec-redirection op=source source-fd=0x0000000000000000 source-path=/etc/banner.txt source-stream=regular-file source-route=initramfs:/etc/banner.txt child-only=true shell-restored=true source=shell-redirection-stdin-etc-banner" "$LOG_FILE"
    grep -q "talos: exec-redirection op=sink source-fd=0x0000000000000002 target-path=/tmp/pipeline-combined-stderr-append.txt target-stream=regular-file target-route=volatile-vfs:/tmp/pipeline-combined-stderr-append.txt child-only=true shell-restored=true source=shell-redirection-stderr-tmp-stderr" "$LOG_FILE"
    grep -q "talos: exec-redirection op=append source-fd=0x0000000000000002 target-path=/tmp/pipeline-combined-stderr-append.txt target-stream=regular-file target-route=volatile-vfs:/tmp/pipeline-combined-stderr-append.txt child-only=true shell-restored=true source=shell-redirection-stderr-tmp-stderr-append" "$LOG_FILE"
    grep -q "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=regular-file route=volatile-vfs:/tmp/pipeline-combined-stderr-append.txt source=userspace-talos-write" "$LOG_FILE"
    grep -q "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdin consumer-fd=0x0000000000000000 consumer-path=/bin/stderr bytes-written=0x000000000000003d bytes-read=0x0000000000000000 writer-closed=true reader-eof=false shell-restored=true source=shell-pipe-producer-stdin-consumer-stderr-redirection" "$LOG_FILE"
    grep -q "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdin consumer-fd=0x0000000000000000 consumer-path=/bin/stderr bytes-written=0x000000000000003d bytes-read=0x0000000000000000 writer-closed=true reader-eof=false shell-restored=true source=shell-pipe-producer-stdin-consumer-stderr-append-redirection" "$LOG_FILE"
    grep -q "talos: cat path=/tmp/pipeline-combined-stderr-append.txt bytes=0x000000000000003e source=volatile-vfs-descriptor-read" "$LOG_FILE"
    grep -q "Talos userspace stdin fixture read: Talos initramfs fixture" "$LOG_FILE"
    grep -c "Talos userspace stderr fixture" "$LOG_FILE" | grep -q "^3$"
    grep -q "talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos: pipestatus participants=0x0000000000000002 default-status=0x0000000000000000 pipefail-status=0x0000000000000000 semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt" "$LOG_FILE"
    grep -q "talos> stderr" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt | /bin/stderr 2>>/tmp/pipeline-combined-stderr-append.txt" "$LOG_FILE"
    grep -q "talos> missing </etc/banner.txt | stderr 2>>/tmp/pipeline-combined-stderr-append.txt" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt | missing 2>>/tmp/pipeline-combined-stderr-append.txt" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt | /bin/stderr 2>>/tmp/pipeline-combined-stderr-append.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt | stderr 2>>/tmp/pipeline-combined-stderr-append.txt" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt | stderr 2>>/tmp/pipeline-combined-stderr.txt" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt | stderr 2>/tmp/pipeline-stderr.txt" "$LOG_FILE"
    grep -q "talos> stdout | stderr 2>>/tmp/pipeline-combined-stderr-append.txt" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt | stderr 2>> /tmp/pipeline-combined-stderr-append.txt" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt | stderr 1>/tmp/pipeline-combined-stderr-append.txt" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt | stderr 2>>/var/x" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt | stderr 2>>/tmp/pipeline-combined-stderr-append.txt | stdin" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=15 status=unknown-command responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=16 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=17 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=18 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=19 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=20 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=21 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=22 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=23 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=24 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=25 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -c "talos: exec-invalid-path" "$LOG_FILE" | grep -q "^10$"
    grep -q "$LABEL: final participants=26 expected=26 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_BARE_NAME_COMBINED_PIPELINE_STDOUT_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> stdin </etc/banner.txt | stdin >/tmp/pipeline-combined-append.txt" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt | stdin >>/tmp/pipeline-combined-append.txt" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stdin source=vfs-open-read" "$LOG_FILE"
    grep -q "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stdin" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=regular-file fd1=pipe-endpoint fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=pipe-endpoint fd1=regular-file fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -q "talos: exec-redirection op=source source-fd=0x0000000000000000 source-path=/etc/banner.txt source-stream=regular-file source-route=initramfs:/etc/banner.txt child-only=true shell-restored=true source=shell-redirection-stdin-etc-banner" "$LOG_FILE"
    grep -q "talos: exec-redirection op=sink source-fd=0x0000000000000001 target-path=/tmp/pipeline-combined-append.txt target-stream=regular-file target-route=volatile-vfs:/tmp/pipeline-combined-append.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout" "$LOG_FILE"
    grep -q "talos: exec-redirection op=append source-fd=0x0000000000000001 target-path=/tmp/pipeline-combined-append.txt target-stream=regular-file target-route=volatile-vfs:/tmp/pipeline-combined-append.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout-append" "$LOG_FILE"
    grep -q "source=shell-pipe-producer-stdin-consumer-stdout-redirection" "$LOG_FILE"
    grep -q "source=shell-pipe-producer-stdin-consumer-stdout-append-redirection" "$LOG_FILE"
    grep -q "talos: cat path=/tmp/pipeline-combined-append.txt bytes=0x00000000000000c4 source=volatile-vfs-descriptor-read" "$LOG_FILE"
    grep -c "Talos userspace stdin fixture read: Talos userspace stdin fixture read: Talos initramfs fixture" "$LOG_FILE" | grep -q "^2$"
    grep -q "talos> stdin </etc/banner.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt | /bin/stdin >/tmp/pipeline-combined-append.txt" "$LOG_FILE"
    grep -q "talos> missing </etc/banner.txt | stdin >>/tmp/pipeline-combined-append.txt" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt | missing >>/tmp/pipeline-combined-append.txt" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt | stdin 2>>/tmp/pipeline-combined-append.txt" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt | stdin >/tmp/pipeline-report.txt" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt | stdin 1>/tmp/pipeline-combined-append.txt" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt | stdin > /tmp/pipeline-combined-append.txt" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt | stdin >>/var/x" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt | /bin/stdin >>/tmp/pipeline-combined-append.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt | stdin >>/tmp/pipeline-combined-append.txt" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt | stdin >>/tmp/pipeline-combined-append.txt | stdin" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=14 status=unknown-command responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=15 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=16 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=17 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=18 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=19 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=20 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=21 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=22 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=23 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -c "talos: exec-invalid-path" "$LOG_FILE" | grep -q "^9$"
    grep -q "$LABEL: final participants=24 expected=24 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_BARE_NAME_COMBINED_PIPELINE_STDERR_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> stdin </etc/banner.txt | stderr 2>/tmp/pipeline-combined-stderr.txt" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stdin source=vfs-open-read" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stderr source=vfs-open-read" "$LOG_FILE"
    grep -q "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stdin" "$LOG_FILE"
    grep -q "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stderr" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=regular-file fd1=pipe-endpoint fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=pipe-endpoint fd1=stdio-output fd2=regular-file loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -q "talos: exec-redirection op=source source-fd=0x0000000000000000 source-path=/etc/banner.txt source-stream=regular-file source-route=initramfs:/etc/banner.txt child-only=true shell-restored=true source=shell-redirection-stdin-etc-banner" "$LOG_FILE"
    grep -q "talos: exec-redirection op=sink source-fd=0x0000000000000002 target-path=/tmp/pipeline-combined-stderr.txt target-stream=regular-file target-route=volatile-vfs:/tmp/pipeline-combined-stderr.txt child-only=true shell-restored=true source=shell-redirection-stderr-tmp-stderr" "$LOG_FILE"
    grep -q "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=regular-file route=volatile-vfs:/tmp/pipeline-combined-stderr.txt source=userspace-talos-write" "$LOG_FILE"
    grep -q "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdin consumer-fd=0x0000000000000000 consumer-path=/bin/stderr bytes-written=0x000000000000003d bytes-read=0x0000000000000000 writer-closed=true reader-eof=false shell-restored=true source=shell-pipe-producer-stdin-consumer-stderr-redirection" "$LOG_FILE"
    grep -q "talos: cat path=/tmp/pipeline-combined-stderr.txt bytes=0x000000000000001f source=volatile-vfs-descriptor-read" "$LOG_FILE"
    grep -q "Talos userspace stdin fixture read: Talos initramfs fixture" "$LOG_FILE"
    grep -q "Talos userspace stderr fixture" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos: pipestatus participants=0x0000000000000002 default-status=0x0000000000000000 pipefail-status=0x0000000000000000 semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt | /bin/stderr 2>/tmp/pipeline-combined-stderr.txt" "$LOG_FILE"
    grep -q "talos> /bin/stderr" "$LOG_FILE"
    grep -q "talos> missing </etc/banner.txt | stderr 2>/tmp/pipeline-combined-stderr.txt" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt | missing 2>/tmp/pipeline-combined-stderr.txt" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt | stderr 2>>/tmp/pipeline-combined-stderr.txt" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt | stderr 2>/tmp/pipeline-stderr.txt" "$LOG_FILE"
    grep -q "talos> stdout | stderr 2>/tmp/pipeline-combined-stderr.txt" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt | stderr 2> /tmp/pipeline-combined-stderr.txt" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt | stderr 1>/tmp/pipeline-combined-stderr.txt" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt | stderr 2>/var/x" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt | /bin/stderr 2>/tmp/pipeline-combined-stderr.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt | stderr 2>/tmp/pipeline-combined-stderr.txt" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt | stderr 2>/tmp/pipeline-combined-stderr.txt | stdin" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=14 status=unknown-command responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=15 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=16 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=17 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=18 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=19 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=20 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=21 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=22 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=23 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=24 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -c "talos: exec-invalid-path" "$LOG_FILE" | grep -q "^10$"
    grep -q "$LABEL: final participants=25 expected=25 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_BARE_NAME_COMBINED_PIPELINE_STDIN_STDOUT_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> stdin </etc/banner.txt | stdin >/tmp/pipeline-combined.txt" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stdin source=vfs-open-read" "$LOG_FILE"
    grep -q "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stdin" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=regular-file fd1=pipe-endpoint fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=pipe-endpoint fd1=regular-file fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -q "talos: exec-redirection op=source source-fd=0x0000000000000000 source-path=/etc/banner.txt source-stream=regular-file source-route=initramfs:/etc/banner.txt child-only=true shell-restored=true source=shell-redirection-stdin-etc-banner" "$LOG_FILE"
    grep -q "talos: exec-redirection op=sink source-fd=0x0000000000000001 target-path=/tmp/pipeline-combined.txt target-stream=regular-file target-route=volatile-vfs:/tmp/pipeline-combined.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout" "$LOG_FILE"
    grep -q "talos: exec-stdin fd=0x0000000000000000 bytes=0x0000000000000018 return=0x0000000000000018 read-source=initramfs:/etc/banner.txt stdout-fd=0x0000000000000001 stdout-bytes=0x000000000000003d stdout-return=0x000000000000003d source=userspace-talos-read+userspace-talos-write read-result=regular-file-eof-after-read" "$LOG_FILE"
    grep -q "talos: exec-stdin fd=0x0000000000000000 bytes=0x000000000000003d return=0x000000000000003d read-source=pipe:stdout-to-stdin stdout-fd=0x0000000000000001 stdout-bytes=0x0000000000000062 stdout-return=0x0000000000000062 source=userspace-talos-read+userspace-talos-write read-result=pipe-eof-after-writer-close" "$LOG_FILE"
    grep -q "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdin consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000003d bytes-read=0x000000000000003d writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-producer-stdin-consumer-stdout-redirection" "$LOG_FILE"
    grep -q "talos: cat path=/tmp/pipeline-combined.txt bytes=0x0000000000000062 source=volatile-vfs-descriptor-read" "$LOG_FILE"
    grep -q "^Talos userspace stdin fixture read: Talos userspace stdin fixture read: Talos initramfs fixture" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt | /bin/stdin >/tmp/pipeline-combined.txt" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos: pipestatus participants=0x0000000000000002 default-status=0x0000000000000000 pipefail-status=0x0000000000000000 semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt | stdin >>/tmp/pipeline-combined.txt" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt | stdin >/tmp/pipeline-report.txt" "$LOG_FILE"
    grep -q "talos> stdout | stdin >/tmp/pipeline-combined.txt" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt | stdin 1>/tmp/pipeline-combined.txt" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt | stdin > /tmp/pipeline-combined.txt" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt | stdin >/var/x" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt | /bin/stdin >/tmp/pipeline-combined.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt | stdin >/tmp/pipeline-combined.txt" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt | stdin >/tmp/pipeline-combined.txt | stdin" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=13 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=14 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=15 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=16 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=17 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=18 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=19 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=20 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=21 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -c "talos: exec-invalid-path" "$LOG_FILE" | grep -q "^9$"
    grep -q "$LABEL: final participants=22 expected=22 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_DIRECT_PIPELINE_STDIN_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> /bin/stdin </etc/banner.txt | /bin/stdin" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stdin source=vfs-open-read" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=regular-file fd1=pipe-endpoint fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=pipe-endpoint fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -Eq "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stdin argv0-ptr=0x[0-9a-f]+ argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x[0-9a-f]+ copied-startup-bytes=0x000000000000002b source=initial-user-stack-record" "$LOG_FILE"
    grep -q "talos: exec-redirection op=source source-fd=0x0000000000000000 source-path=/etc/banner.txt source-stream=regular-file source-route=initramfs:/etc/banner.txt child-only=true shell-restored=true source=shell-redirection-stdin-etc-banner" "$LOG_FILE"
    grep -q "^Talos userspace stdin fixture read: Talos userspace stdin fixture read: Talos initramfs fixture" "$LOG_FILE"
    grep -q "talos: exec-stdin fd=0x0000000000000000 bytes=0x0000000000000018 return=0x0000000000000018 read-source=initramfs:/etc/banner.txt stdout-fd=0x0000000000000001 stdout-bytes=0x000000000000003d stdout-return=0x000000000000003d source=userspace-talos-read+userspace-talos-write read-result=regular-file-eof-after-read" "$LOG_FILE"
    grep -q "talos: exec-stdin fd=0x0000000000000000 bytes=0x000000000000003d return=0x000000000000003d read-source=pipe:stdout-to-stdin stdout-fd=0x0000000000000001 stdout-bytes=0x0000000000000062 stdout-return=0x0000000000000062 source=userspace-talos-read+userspace-talos-write read-result=pipe-eof-after-writer-close" "$LOG_FILE"
    grep -q "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdin consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000003d bytes-read=0x000000000000003d writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-producer-stdin-redirection-to-stdin" "$LOG_FILE"
    grep -q "talos: pipeline-lifecycle-status record=phase12-local-pipeline-distinct-process-lifecycle-status-record-v1 pipeline=0x0000000000000001 producer-pid=0x0000000000100001 producer-path=/bin/stdin" "$LOG_FILE"
    grep -q "consumer-pid=0x0000000000100002 consumer-path=/bin/stdin" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos: last-process pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos: pipestatus participants=0x0000000000000002 default-status=0x0000000000000000 pipefail-status=0x0000000000000000 semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status" "$LOG_FILE"
    grep -q "slot=0 capacity=3 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table" "$LOG_FILE"
    grep -q "slot=1 capacity=3 pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt | /bin/stdout" "$LOG_FILE"
    grep -q "talos> /bin/stdin alpha </etc/banner.txt | /bin/stdin" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt | /bin/stdin beta" "$LOG_FILE"
    grep -q "talos> /bin/stdin </dev/null | /bin/stdin" "$LOG_FILE"
    grep -q "talos> /bin/stdin < /etc/banner.txt | /bin/stdin" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt | /bin/stdin | /bin/stdin" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=10 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=11 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=12 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=13 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=14 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=15 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -c "talos: exec-invalid-path" "$LOG_FILE" | grep -q "^6$"
    grep -q "$LABEL: final participants=16 expected=16 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_DIRECT_PIPELINE_CONSUMER_STDIN_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> /bin/stdin | /bin/stdin </etc/banner.txt" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stdin source=vfs-open-read" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=pipe-endpoint fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=regular-file fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -Eq "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stdin argv0-ptr=0x[0-9a-f]+ argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x[0-9a-f]+ copied-startup-bytes=0x000000000000002b source=initial-user-stack-record" "$LOG_FILE"
    grep -q "talos: exec-stdin fd=0x0000000000000000 bytes=0x0000000000000000 return=0xfffffffffffffff5 read-source=runtime-console0/local-input stdout-fd=0x0000000000000001 stdout-bytes=0x0000000000000031 stdout-return=0x0000000000000031 source=userspace-talos-read+userspace-talos-write read-result=readiness/no-data" "$LOG_FILE"
    grep -q "talos: exec-redirection op=source source-fd=0x0000000000000000 source-path=/etc/banner.txt source-stream=regular-file source-route=initramfs:/etc/banner.txt child-only=true shell-restored=true source=shell-redirection-stdin-etc-banner" "$LOG_FILE"
    grep -q "talos: exec-stdin fd=0x0000000000000000 bytes=0x0000000000000018 return=0x0000000000000018 read-source=initramfs:/etc/banner.txt stdout-fd=0x0000000000000001 stdout-bytes=0x000000000000003d stdout-return=0x000000000000003d source=userspace-talos-read+userspace-talos-write read-result=regular-file-eof-after-read" "$LOG_FILE"
    grep -q "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdin consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x0000000000000031 bytes-read=0x0000000000000000 writer-closed=true reader-eof=false shell-restored=true source=shell-pipe-consumer-stdin-redirection-from-file" "$LOG_FILE"
    grep -q "talos: pipeline-lifecycle-status record=phase12-local-pipeline-distinct-process-lifecycle-status-record-v1 pipeline=0x0000000000000001 producer-pid=0x0000000000100001 producer-path=/bin/stdin" "$LOG_FILE"
    grep -q "consumer-pid=0x0000000000100002 consumer-path=/bin/stdin" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos: last-process pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos: pipestatus participants=0x0000000000000002 default-status=0x0000000000000000 pipefail-status=0x0000000000000000 semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status" "$LOG_FILE"
    grep -q "slot=0 capacity=3 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table" "$LOG_FILE"
    grep -q "slot=1 capacity=3 pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table" "$LOG_FILE"
    grep -q "talos> /bin/stdout | /bin/stdin </etc/banner.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin alpha | /bin/stdin </etc/banner.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin | /bin/stdin beta </etc/banner.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin | /bin/stdin </dev/null" "$LOG_FILE"
    grep -q "talos> /bin/stdin | /bin/stdin < /etc/banner.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin | /bin/stdin </etc/banner.txt | /bin/stdin" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt | /bin/stdin </etc/banner.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt | stdin </etc/banner.txt" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=10 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=11 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=12 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=13 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=14 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=15 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=16 status=handled responses=24" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=17 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdin consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000003d bytes-read=0x0000000000000000 writer-closed=true reader-eof=false shell-restored=true source=shell-pipe-dual-stdin-redirection-from-file" "$LOG_FILE"
    grep -c "talos: exec-invalid-path" "$LOG_FILE" | grep -q "^7$"
    grep -q "$LABEL: final participants=18 expected=18 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_BARE_NAME_PIPELINE_STDIN_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> stdin </etc/banner.txt | stdin" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stdin source=vfs-open-read" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=regular-file fd1=pipe-endpoint fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=pipe-endpoint fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -Eq "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stdin argv0-ptr=0x[0-9a-f]+ argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x[0-9a-f]+ copied-startup-bytes=0x000000000000002b source=initial-user-stack-record" "$LOG_FILE"
    grep -q "talos: exec-redirection op=source source-fd=0x0000000000000000 source-path=/etc/banner.txt source-stream=regular-file source-route=initramfs:/etc/banner.txt child-only=true shell-restored=true source=shell-redirection-stdin-etc-banner" "$LOG_FILE"
    grep -q "^Talos userspace stdin fixture read: Talos userspace stdin fixture read: Talos initramfs fixture" "$LOG_FILE"
    grep -q "talos: exec-stdin fd=0x0000000000000000 bytes=0x0000000000000018 return=0x0000000000000018 read-source=initramfs:/etc/banner.txt stdout-fd=0x0000000000000001 stdout-bytes=0x000000000000003d stdout-return=0x000000000000003d source=userspace-talos-read+userspace-talos-write read-result=regular-file-eof-after-read" "$LOG_FILE"
    grep -q "talos: exec-stdin fd=0x0000000000000000 bytes=0x000000000000003d return=0x000000000000003d read-source=pipe:stdout-to-stdin stdout-fd=0x0000000000000001 stdout-bytes=0x0000000000000062 stdout-return=0x0000000000000062 source=userspace-talos-read+userspace-talos-write read-result=pipe-eof-after-writer-close" "$LOG_FILE"
    grep -q "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdin consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000003d bytes-read=0x000000000000003d writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-producer-stdin-redirection-to-stdin" "$LOG_FILE"
    grep -q "talos: pipeline-lifecycle-status record=phase12-local-pipeline-distinct-process-lifecycle-status-record-v1 pipeline=0x0000000000000001 producer-pid=0x0000000000100001 producer-path=/bin/stdin" "$LOG_FILE"
    grep -q "consumer-pid=0x0000000000100002 consumer-path=/bin/stdin" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos: last-process pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos: pipestatus participants=0x0000000000000002 default-status=0x0000000000000000 pipefail-status=0x0000000000000000 semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status" "$LOG_FILE"
    grep -q "slot=0 capacity=3 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table" "$LOG_FILE"
    grep -q "slot=1 capacity=3 pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt | stdout" "$LOG_FILE"
    grep -q "talos> stdin alpha </etc/banner.txt | stdin" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt | stdin beta" "$LOG_FILE"
    grep -q "talos> stdin </dev/null | stdin" "$LOG_FILE"
    grep -q "talos> stdin < /etc/banner.txt | stdin" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt | stdin | stdin" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=10 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=11 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=12 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=13 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=14 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=15 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -c "talos: exec-invalid-path" "$LOG_FILE" | grep -q "^6$"
    grep -q "$LABEL: final participants=16 expected=16 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_BARE_NAME_PIPELINE_CONSUMER_STDIN_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> stdin | stdin </etc/banner.txt" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stdin source=vfs-open-read" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=pipe-endpoint fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=regular-file fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -Eq "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stdin argv0-ptr=0x[0-9a-f]+ argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x[0-9a-f]+ copied-startup-bytes=0x000000000000002b source=initial-user-stack-record" "$LOG_FILE"
    grep -q "talos: exec-stdin fd=0x0000000000000000 bytes=0x0000000000000000 return=0xfffffffffffffff5 read-source=runtime-console0/local-input stdout-fd=0x0000000000000001 stdout-bytes=0x0000000000000031 stdout-return=0x0000000000000031 source=userspace-talos-read+userspace-talos-write read-result=readiness/no-data" "$LOG_FILE"
    grep -q "talos: exec-redirection op=source source-fd=0x0000000000000000 source-path=/etc/banner.txt source-stream=regular-file source-route=initramfs:/etc/banner.txt child-only=true shell-restored=true source=shell-redirection-stdin-etc-banner" "$LOG_FILE"
    grep -q "talos: exec-stdin fd=0x0000000000000000 bytes=0x0000000000000018 return=0x0000000000000018 read-source=initramfs:/etc/banner.txt stdout-fd=0x0000000000000001 stdout-bytes=0x000000000000003d stdout-return=0x000000000000003d source=userspace-talos-read+userspace-talos-write read-result=regular-file-eof-after-read" "$LOG_FILE"
    grep -q "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdin consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x0000000000000031 bytes-read=0x0000000000000000 writer-closed=true reader-eof=false shell-restored=true source=shell-pipe-consumer-stdin-redirection-from-file" "$LOG_FILE"
    grep -q "talos: pipeline-lifecycle-status record=phase12-local-pipeline-distinct-process-lifecycle-status-record-v1 pipeline=0x0000000000000001 producer-pid=0x0000000000100001 producer-path=/bin/stdin" "$LOG_FILE"
    grep -q "consumer-pid=0x0000000000100002 consumer-path=/bin/stdin" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos: last-process pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos: pipestatus participants=0x0000000000000002 default-status=0x0000000000000000 pipefail-status=0x0000000000000000 semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status" "$LOG_FILE"
    grep -q "slot=0 capacity=3 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table" "$LOG_FILE"
    grep -q "slot=1 capacity=3 pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table" "$LOG_FILE"
    grep -q "talos> stdout | stdin </etc/banner.txt" "$LOG_FILE"
    grep -q "talos> stdin alpha | stdin </etc/banner.txt" "$LOG_FILE"
    grep -q "talos> stdin | stdin beta </etc/banner.txt" "$LOG_FILE"
    grep -q "talos> stdin | stdin </dev/null" "$LOG_FILE"
    grep -q "talos> stdin | stdin < /etc/banner.txt" "$LOG_FILE"
    grep -q "talos> stdin | stdin </etc/banner.txt | stdin" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt | stdin </etc/banner.txt" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt | /bin/stdin </etc/banner.txt" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=10 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=11 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=12 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=13 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=14 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=15 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=16 status=handled responses=24" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=17 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdin consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000003d bytes-read=0x0000000000000000 writer-closed=true reader-eof=false shell-restored=true source=shell-pipe-dual-stdin-redirection-from-file" "$LOG_FILE"
    grep -c "talos: exec-invalid-path" "$LOG_FILE" | grep -q "^7$"
    grep -q "$LABEL: final participants=18 expected=18 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_BARE_NAME_STDIN_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> stdin </etc/banner.txt" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stdin source=vfs-open-read" "$LOG_FILE"
    grep -Eq "talos: exec-source bytes=0x[0-9a-f]+ digest=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-loader fixture=phase8-program-loader-elf64-aarch64-v1 entry=0x[0-9a-f]+ segments=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-launch launch-boundary=phase8-initial-process-launch-plan-v1 stack-boundary=phase8-initial-user-stack-plan-v1 address-space=0x[0-9a-f]+ materialization=0x[0-9a-f]+ initial-sp=0x[0-9a-f]+" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=regular-file fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -Eq "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stdin argv0-ptr=0x[0-9a-f]+ argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x[0-9a-f]+ copied-startup-bytes=0x000000000000002b source=initial-user-stack-record" "$LOG_FILE"
    grep -q "talos: exec-redirection op=source source-fd=0x0000000000000000 source-path=/etc/banner.txt source-stream=regular-file source-route=initramfs:/etc/banner.txt child-only=true shell-restored=true source=shell-redirection-stdin-etc-banner" "$LOG_FILE"
    grep -q "^Talos userspace stdin fixture read: Talos initramfs fixture" "$LOG_FILE"
    grep -q "talos: exec-stdin fd=0x0000000000000000 bytes=0x0000000000000018 return=0x0000000000000018 read-source=initramfs:/etc/banner.txt stdout-fd=0x0000000000000001 stdout-bytes=0x000000000000003d stdout-return=0x000000000000003d source=userspace-talos-read+userspace-talos-write read-result=regular-file-eof-after-read" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=3 status=handled responses=11" "$LOG_FILE"
    grep -Eq "talos: waitpid pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -Eq "talos: last-process pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos: pipestatus none source=bounded-process-table-pipeline-status" "$LOG_FILE"
    grep -q "talos> cat /proc/talos/processes" "$LOG_FILE"
    grep -q "path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table" "$LOG_FILE"
    grep -q "talos> ps" "$LOG_FILE"
    grep -q "talos> stdout </etc/banner.txt" "$LOG_FILE"
    grep -q "talos> stdin </dev/null" "$LOG_FILE"
    grep -q "talos> stdin < /etc/banner.txt" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=9 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=10 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=11 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -c "talos: exec-invalid-path" "$LOG_FILE" | grep -q "^3$"
    grep -q "$LABEL: final participants=12 expected=12 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_READONLY_REGULAR_FILE_STDIN_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> exec stdin </etc/banner.txt" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stdin source=vfs-open-read" "$LOG_FILE"
    grep -Eq "talos: exec-source bytes=0x[0-9a-f]+ digest=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-loader fixture=phase8-program-loader-elf64-aarch64-v1 entry=0x[0-9a-f]+ segments=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-launch launch-boundary=phase8-initial-process-launch-plan-v1 stack-boundary=phase8-initial-user-stack-plan-v1 address-space=0x[0-9a-f]+ materialization=0x[0-9a-f]+ initial-sp=0x[0-9a-f]+" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=regular-file fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -Eq "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stdin argv0-ptr=0x[0-9a-f]+ argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x[0-9a-f]+ copied-startup-bytes=0x000000000000002b source=initial-user-stack-record" "$LOG_FILE"
    grep -q "talos: exec-redirection op=source source-fd=0x0000000000000000 source-path=/etc/banner.txt source-stream=regular-file source-route=initramfs:/etc/banner.txt child-only=true shell-restored=true source=shell-redirection-stdin-etc-banner" "$LOG_FILE"
    grep -q "^Talos userspace stdin fixture read: Talos initramfs fixture" "$LOG_FILE"
    grep -q "talos: exec-stdin fd=0x0000000000000000 bytes=0x0000000000000018 return=0x0000000000000018 read-source=initramfs:/etc/banner.txt stdout-fd=0x0000000000000001 stdout-bytes=0x000000000000003d stdout-return=0x000000000000003d source=userspace-talos-read+userspace-talos-write read-result=regular-file-eof-after-read" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=3 status=handled responses=11" "$LOG_FILE"
    grep -Eq "talos: waitpid pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -Eq "talos: last-process pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos> exec stdin" "$LOG_FILE"
    grep -q "^Talos userspace stdin fixture read: talos-console0" "$LOG_FILE"
    grep -q "talos: exec-stdin fd=0x0000000000000000 bytes=0x000000000000000e return=0x000000000000000e read-source=runtime-console0/local-input stdout-fd=0x0000000000000001 stdout-bytes=0x0000000000000033 stdout-return=0x0000000000000033 source=userspace-talos-read+userspace-talos-write" "$LOG_FILE"
    grep -q "talos> exec stdout </dev/null" "$LOG_FILE"
    grep -q "talos> exec stdin </dev/null" "$LOG_FILE"
    grep -q "talos: exec-redirection op=source source-fd=0x0000000000000000 source-path=/dev/null source-stream=null-source source-route=device:/dev/null child-only=true shell-restored=true source=shell-redirection-stdin-dev-null" "$LOG_FILE"
    grep -q "talos> exec stdin < /etc/banner.txt" "$LOG_FILE"
    grep -c "talos: exec-invalid-path" "$LOG_FILE" | grep -q "^2$"
    grep -q "talos> cat /etc/banner.txt" "$LOG_FILE"
    grep -q "^Talos initramfs fixture" "$LOG_FILE"
    grep -q "$LABEL: final participants=12 expected=12 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_BARE_NAME_STDOUT_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> stdout >/tmp/stdout.txt" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stdout source=vfs-open-read" "$LOG_FILE"
    grep -Eq "talos: exec-source bytes=0x[0-9a-f]+ digest=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-loader fixture=phase8-program-loader-elf64-aarch64-v1 entry=0x[0-9a-f]+ segments=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-launch launch-boundary=phase8-initial-process-launch-plan-v1 stack-boundary=phase8-initial-user-stack-plan-v1 address-space=0x[0-9a-f]+ materialization=0x[0-9a-f]+ initial-sp=0x[0-9a-f]+" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=regular-file fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -Eq "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stdout argv0-ptr=0x[0-9a-f]+ argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x[0-9a-f]+ copied-startup-bytes=0x000000000000002c source=initial-user-stack-record" "$LOG_FILE"
    grep -q "talos: exec-redirection op=sink source-fd=0x0000000000000001 target-path=/tmp/stdout.txt target-stream=regular-file target-route=volatile-vfs:/tmp/stdout.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout" "$LOG_FILE"
    grep -q "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=regular-file route=volatile-vfs:/tmp/stdout.txt source=userspace-talos-write" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=3 status=handled responses=11" "$LOG_FILE"
    grep -Eq "talos: waitpid pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -Eq "talos: last-process pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos> cat /tmp/stdout.txt" "$LOG_FILE"
    grep -q "^Talos userspace stdout fixture" "$LOG_FILE"
    grep -q "talos: cat path=/tmp/stdout.txt bytes=0x000000000000001f source=volatile-vfs-descriptor-read" "$LOG_FILE"
    grep -q "talos> stdout" "$LOG_FILE"
    grep -q "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=stdout route=runtime-console0/stdout source=userspace-talos-write" "$LOG_FILE"
    grep -q "talos> stdout >>/var/append.txt" "$LOG_FILE"
    grep -q "talos> stdout >/var/other.txt" "$LOG_FILE"
    grep -q "talos> stdout | stdin >/tmp/stdout.txt" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt >/tmp/stdout.txt" "$LOG_FILE"
    grep -c "talos: exec-invalid-path" "$LOG_FILE" | grep -q "^4$"
    grep -q "talos> cat /etc/banner.txt" "$LOG_FILE"
    grep -q "^Talos initramfs fixture" "$LOG_FILE"
    grep -q "$LABEL: final participants=13 expected=13 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_STDOUT_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> /bin/stdout >/tmp/stdout.txt" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stdout source=vfs-open-read" "$LOG_FILE"
    grep -Eq "talos: exec-source bytes=0x[0-9a-f]+ digest=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-loader fixture=phase8-program-loader-elf64-aarch64-v1 entry=0x[0-9a-f]+ segments=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-launch launch-boundary=phase8-initial-process-launch-plan-v1 stack-boundary=phase8-initial-user-stack-plan-v1 address-space=0x[0-9a-f]+ materialization=0x[0-9a-f]+ initial-sp=0x[0-9a-f]+" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=regular-file fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -Eq "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stdout argv0-ptr=0x[0-9a-f]+ argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x[0-9a-f]+ copied-startup-bytes=0x000000000000002c source=initial-user-stack-record" "$LOG_FILE"
    grep -q "talos: exec-redirection op=sink source-fd=0x0000000000000001 target-path=/tmp/stdout.txt target-stream=regular-file target-route=volatile-vfs:/tmp/stdout.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout" "$LOG_FILE"
    grep -q "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=regular-file route=volatile-vfs:/tmp/stdout.txt source=userspace-talos-write" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=3 status=handled responses=11" "$LOG_FILE"
    grep -Eq "talos: waitpid pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -Eq "talos: last-process pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos> cat /tmp/stdout.txt" "$LOG_FILE"
    grep -q "^Talos userspace stdout fixture" "$LOG_FILE"
    grep -q "talos: cat path=/tmp/stdout.txt bytes=0x000000000000001f source=volatile-vfs-descriptor-read" "$LOG_FILE"
    grep -q "talos> /bin/stdout" "$LOG_FILE"
    grep -q "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=stdout route=runtime-console0/stdout source=userspace-talos-write" "$LOG_FILE"
    grep -q "talos> exec stdout >>/var/other.txt" "$LOG_FILE"
    grep -q "talos> exec stdout >/var/other.txt" "$LOG_FILE"
    grep -q "talos> exec stderr 2>/tmp/stdout.txt" "$LOG_FILE"
    grep -c "talos: exec-invalid-path" "$LOG_FILE" | grep -q "^3$"
    grep -q "talos> cat /etc/banner.txt" "$LOG_FILE"
    grep -q "^Talos initramfs fixture" "$LOG_FILE"
    grep -q "$LABEL: final participants=13 expected=13 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_DIRECT_STDOUT_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> /bin/stdout >/tmp/stdout.txt" "$LOG_FILE"
    grep -q "talos: exec-redirection op=sink source-fd=0x0000000000000001 target-path=/tmp/stdout.txt target-stream=regular-file target-route=volatile-vfs:/tmp/stdout.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout" "$LOG_FILE"
    grep -q "talos> /bin/stdout >>/tmp/stdout.txt" "$LOG_FILE"
    grep -q "talos: exec-redirection op=append source-fd=0x0000000000000001 target-path=/tmp/stdout.txt target-stream=regular-file target-route=volatile-vfs:/tmp/stdout.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout-append" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=regular-file fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -q "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=regular-file route=volatile-vfs:/tmp/stdout.txt source=userspace-talos-write" "$LOG_FILE"
    grep -Eq "talos: waitpid pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -Eq "talos: last-process pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos> cat /tmp/stdout.txt" "$LOG_FILE"
    grep -q "talos: cat path=/tmp/stdout.txt bytes=0x000000000000003e source=volatile-vfs-descriptor-read" "$LOG_FILE"
    grep -q "talos> /bin/stdout" "$LOG_FILE"
    grep -q "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=stdout route=runtime-console0/stdout source=userspace-talos-write" "$LOG_FILE"
    grep -q "talos> /bin/stdout >>/var/other.txt" "$LOG_FILE"
    grep -q "talos> /bin/stderr 2>>/tmp/stderr.txt" "$LOG_FILE"
    grep -q "talos: exec-redirection op=append source-fd=0x0000000000000002 target-path=/tmp/stderr.txt target-stream=regular-file target-route=volatile-vfs:/tmp/stderr.txt child-only=true shell-restored=true source=shell-redirection-stderr-tmp-stderr-append" "$LOG_FILE"
    grep -q "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=regular-file route=volatile-vfs:/tmp/stderr.txt source=userspace-talos-write" "$LOG_FILE"
    grep -c "talos: exec-invalid-path" "$LOG_FILE" | grep -q "^1$"
    grep -q "talos> cat /etc/banner.txt" "$LOG_FILE"
    grep -q "^Talos initramfs fixture" "$LOG_FILE"
    grep -q "$LABEL: final participants=14 expected=14 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_BARE_NAME_STDOUT_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> stdout >/tmp/stdout.txt" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stdout source=vfs-open-read" "$LOG_FILE"
    grep -q "talos: exec-redirection op=sink source-fd=0x0000000000000001 target-path=/tmp/stdout.txt target-stream=regular-file target-route=volatile-vfs:/tmp/stdout.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout" "$LOG_FILE"
    grep -q "talos> stdout >>/tmp/stdout.txt" "$LOG_FILE"
    grep -q "talos: exec-redirection op=append source-fd=0x0000000000000001 target-path=/tmp/stdout.txt target-stream=regular-file target-route=volatile-vfs:/tmp/stdout.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout-append" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=regular-file fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -q "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=regular-file route=volatile-vfs:/tmp/stdout.txt source=userspace-talos-write" "$LOG_FILE"
    grep -Eq "talos: waitpid pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -Eq "talos: last-process pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos> cat /tmp/stdout.txt" "$LOG_FILE"
    grep -q "talos: cat path=/tmp/stdout.txt bytes=0x000000000000003e source=volatile-vfs-descriptor-read" "$LOG_FILE"
    grep -q "talos> stdout" "$LOG_FILE"
    grep -q "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=stdout route=runtime-console0/stdout source=userspace-talos-write" "$LOG_FILE"
    grep -q "talos> stdout >>/var/other.txt" "$LOG_FILE"
    grep -q "talos> stderr 2>>/tmp/other.txt" "$LOG_FILE"
    grep -c "talos: exec-invalid-path" "$LOG_FILE" | grep -q "^2$"
    grep -q "talos> cat /etc/banner.txt" "$LOG_FILE"
    grep -q "^Talos initramfs fixture" "$LOG_FILE"
    grep -q "$LABEL: final participants=14 expected=14 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_STDOUT_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> exec stdout >/tmp/stdout.txt" "$LOG_FILE"
    grep -q "talos: exec-redirection op=sink source-fd=0x0000000000000001 target-path=/tmp/stdout.txt target-stream=regular-file target-route=volatile-vfs:/tmp/stdout.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout" "$LOG_FILE"
    grep -q "talos> exec stdout >>/tmp/stdout.txt" "$LOG_FILE"
    grep -q "talos: exec-redirection op=append source-fd=0x0000000000000001 target-path=/tmp/stdout.txt target-stream=regular-file target-route=volatile-vfs:/tmp/stdout.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout-append" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=regular-file fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -q "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=regular-file route=volatile-vfs:/tmp/stdout.txt source=userspace-talos-write" "$LOG_FILE"
    grep -Eq "talos: waitpid pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -Eq "talos: last-process pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos> cat /tmp/stdout.txt" "$LOG_FILE"
    grep -q "talos: cat path=/tmp/stdout.txt bytes=0x000000000000003e source=volatile-vfs-descriptor-read" "$LOG_FILE"
    grep -q "talos> exec stdout" "$LOG_FILE"
    grep -q "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=stdout route=runtime-console0/stdout source=userspace-talos-write" "$LOG_FILE"
    grep -q "talos> exec stdout >>/var/other.txt" "$LOG_FILE"
    grep -q "talos> exec stderr 2>>/tmp/stderr.txt" "$LOG_FILE"
    grep -c "talos: exec-invalid-path" "$LOG_FILE" | grep -q "^2$"
    grep -q "talos> cat /etc/banner.txt" "$LOG_FILE"
    grep -q "^Talos initramfs fixture" "$LOG_FILE"
    grep -q "$LABEL: final participants=14 expected=14 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_STDOUT_REGULAR_FILE_APPEND_CREATE_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> exec stdout >>/tmp/stdout.txt" "$LOG_FILE"
    grep -q "talos: exec-redirection op=append source-fd=0x0000000000000001 target-path=/tmp/stdout.txt target-stream=regular-file target-route=volatile-vfs:/tmp/stdout.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout-append" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=regular-file fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -q "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=regular-file route=volatile-vfs:/tmp/stdout.txt source=userspace-talos-write" "$LOG_FILE"
    grep -Eq "talos: waitpid pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -Eq "talos: last-process pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos> cat /tmp/stdout.txt" "$LOG_FILE"
    grep -q "talos: cat path=/tmp/stdout.txt bytes=0x000000000000001f source=volatile-vfs-descriptor-read" "$LOG_FILE"
    grep -q "talos> exec stdout" "$LOG_FILE"
    grep -q "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=stdout route=runtime-console0/stdout source=userspace-talos-write" "$LOG_FILE"
    grep -q "talos> exec stdout >>/var/other.txt" "$LOG_FILE"
    grep -q "talos> exec stdout >>/tmp/stderr.txt" "$LOG_FILE"
    grep -c "talos: exec-invalid-path" "$LOG_FILE" | grep -q "^2$"
    grep -q "talos> cat /etc/banner.txt" "$LOG_FILE"
    grep -q "^Talos initramfs fixture" "$LOG_FILE"
    grep -q "$LABEL: final participants=11 expected=11 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_EXPLICIT_FD1_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> exec stdout 1>/tmp/stdout.txt" "$LOG_FILE"
    grep -q "talos: exec-redirection op=sink source-fd=0x0000000000000001 target-path=/tmp/stdout.txt target-stream=regular-file target-route=volatile-vfs:/tmp/stdout.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=regular-file fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -q "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=regular-file route=volatile-vfs:/tmp/stdout.txt source=userspace-talos-write" "$LOG_FILE"
    grep -Eq "talos: waitpid pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -Eq "talos: last-process pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos> cat /tmp/stdout.txt" "$LOG_FILE"
    grep -q "talos: cat path=/tmp/stdout.txt bytes=0x000000000000001f source=volatile-vfs-descriptor-read" "$LOG_FILE"
    grep -q "talos> exec stdout" "$LOG_FILE"
    grep -q "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=stdout route=runtime-console0/stdout source=userspace-talos-write" "$LOG_FILE"
    grep -q "talos> exec stdout 1>>/tmp/stdout.txt" "$LOG_FILE"
    grep -q "talos: exec-redirection op=append source-fd=0x0000000000000001 target-path=/tmp/stdout.txt target-stream=regular-file target-route=volatile-vfs:/tmp/stdout.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout-append" "$LOG_FILE"
    grep -q "talos: cat path=/tmp/stdout.txt bytes=0x000000000000003e source=volatile-vfs-descriptor-read" "$LOG_FILE"
    grep -q "talos> exec stdout 3>/tmp/stdout.txt" "$LOG_FILE"
    grep -q "talos> exec stdout 1>/var/other.txt" "$LOG_FILE"
    grep -c "talos: exec-invalid-path" "$LOG_FILE" | grep -q "^2$"
    grep -q "$LABEL: final participants=14 expected=14 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_STDOUT_ARBITRARY_TMP_OUTPUT_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> exec stdout >/tmp/alpha.log" "$LOG_FILE"
    grep -q "talos: exec-redirection op=sink source-fd=0x0000000000000001 target-path=/tmp/alpha.log target-stream=regular-file target-route=volatile-vfs:/tmp/alpha.log child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout" "$LOG_FILE"
    grep -q "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=regular-file route=volatile-vfs:/tmp/alpha.log source=userspace-talos-write" "$LOG_FILE"
    grep -q "talos> cat /tmp/alpha.log" "$LOG_FILE"
    grep -q "talos: cat path=/tmp/alpha.log bytes=0x000000000000001f source=volatile-vfs-descriptor-read" "$LOG_FILE"
    grep -q "talos> exec stdout >>/tmp/beta.out" "$LOG_FILE"
    grep -q "talos: exec-redirection op=append source-fd=0x0000000000000001 target-path=/tmp/beta.out target-stream=regular-file target-route=volatile-vfs:/tmp/beta.out child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout-append" "$LOG_FILE"
    grep -q "talos: cat path=/tmp/beta.out bytes=0x000000000000001f source=volatile-vfs-descriptor-read" "$LOG_FILE"
    grep -q "talos> exec stdout 1>/tmp/gamma.log" "$LOG_FILE"
    grep -q "talos: exec-redirection op=sink source-fd=0x0000000000000001 target-path=/tmp/gamma.log target-stream=regular-file target-route=volatile-vfs:/tmp/gamma.log child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout" "$LOG_FILE"
    grep -q "talos: cat path=/tmp/gamma.log bytes=0x000000000000001f source=volatile-vfs-descriptor-read" "$LOG_FILE"
    grep -q "talos> exec stdout 1>>/tmp/delta.out" "$LOG_FILE"
    grep -q "talos: exec-redirection op=append source-fd=0x0000000000000001 target-path=/tmp/delta.out target-stream=regular-file target-route=volatile-vfs:/tmp/delta.out child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout-append" "$LOG_FILE"
    grep -q "talos: cat path=/tmp/delta.out bytes=0x000000000000001f source=volatile-vfs-descriptor-read" "$LOG_FILE"
    grep -q "talos> exec stdout" "$LOG_FILE"
    grep -q "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=stdout route=runtime-console0/stdout source=userspace-talos-write" "$LOG_FILE"
    grep -q "talos> exec stdout >/var/out.txt" "$LOG_FILE"
    grep -q "talos> exec stdout >/tmp/nested/out.txt" "$LOG_FILE"
    grep -q "talos> exec stdout >/tmp/" "$LOG_FILE"
    grep -q "talos> exec stdout 3>/tmp/alpha.log" "$LOG_FILE"
    grep -q "talos> exec stdout >/tmp/../bad.txt" "$LOG_FILE"
    grep -q "talos> exec stdout >/tmp/stderr.txt" "$LOG_FILE"
    grep -c "talos: exec-invalid-path" "$LOG_FILE" | grep -q "^6$"
    grep -Eq "talos: waitpid pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -Eq "talos: last-process pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "$LABEL: final participants=26 expected=26 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_STDERR_ARBITRARY_TMP_OUTPUT_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> exec stderr 2>/tmp/omega.err" "$LOG_FILE"
    grep -q "talos: exec-redirection op=sink source-fd=0x0000000000000002 target-path=/tmp/omega.err target-stream=regular-file target-route=volatile-vfs:/tmp/omega.err child-only=true shell-restored=true source=shell-redirection-stderr-tmp-stderr" "$LOG_FILE"
    grep -q "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=regular-file route=volatile-vfs:/tmp/omega.err source=userspace-talos-write" "$LOG_FILE"
    grep -q "talos> cat /tmp/omega.err" "$LOG_FILE"
    grep -q "talos: cat path=/tmp/omega.err bytes=0x000000000000001f source=volatile-vfs-descriptor-read" "$LOG_FILE"
    grep -q "talos> exec stderr 2>>/tmp/theta.log" "$LOG_FILE"
    grep -q "talos: exec-redirection op=append source-fd=0x0000000000000002 target-path=/tmp/theta.log target-stream=regular-file target-route=volatile-vfs:/tmp/theta.log child-only=true shell-restored=true source=shell-redirection-stderr-tmp-stderr-append" "$LOG_FILE"
    grep -q "talos: cat path=/tmp/theta.log bytes=0x000000000000001f source=volatile-vfs-descriptor-read" "$LOG_FILE"
    grep -q "talos> exec stderr" "$LOG_FILE"
    grep -q "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=stderr route=runtime-console0/stderr source=userspace-talos-write" "$LOG_FILE"
    grep -q "talos> exec stdout" "$LOG_FILE"
    grep -q "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=stdout route=runtime-console0/stdout source=userspace-talos-write" "$LOG_FILE"
    grep -q "talos> exec stderr 2>/var/err.txt" "$LOG_FILE"
    grep -q "talos> exec stderr 2>/tmp/n/e" "$LOG_FILE"
    grep -q "talos> exec stderr 2>/tmp/" "$LOG_FILE"
    grep -q "talos> exec stderr 3>/tmp/omega.err" "$LOG_FILE"
    grep -q "talos> exec stderr 2>/tmp/../bad.txt" "$LOG_FILE"
    grep -q "talos> exec stderr 2>/tmp/stdout.txt" "$LOG_FILE"
    grep -q "talos> exec stderr >/tmp/misbound.err" "$LOG_FILE"
    grep -c "talos: exec-invalid-path" "$LOG_FILE" | grep -q "^7$"
    grep -Eq "talos: waitpid pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -Eq "talos: last-process pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "$LABEL: final participants=20 expected=20 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_STDERR_REGULAR_FILE_APPEND_CREATE_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> exec stderr 2>>/tmp/stderr.txt" "$LOG_FILE"
    grep -q "talos: exec-redirection op=append source-fd=0x0000000000000002 target-path=/tmp/stderr.txt target-stream=regular-file target-route=volatile-vfs:/tmp/stderr.txt child-only=true shell-restored=true source=shell-redirection-stderr-tmp-stderr-append" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=stdio-output fd2=regular-file loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -q "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=regular-file route=volatile-vfs:/tmp/stderr.txt source=userspace-talos-write" "$LOG_FILE"
    grep -Eq "talos: waitpid pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -Eq "talos: last-process pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos> cat /tmp/stderr.txt" "$LOG_FILE"
    grep -q "talos: cat path=/tmp/stderr.txt bytes=0x000000000000001f source=volatile-vfs-descriptor-read" "$LOG_FILE"
    grep -q "talos> exec stderr" "$LOG_FILE"
    grep -q "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=stderr route=runtime-console0/stderr source=userspace-talos-write" "$LOG_FILE"
    grep -q "talos> exec stdout" "$LOG_FILE"
    grep -q "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=stdout route=runtime-console0/stdout source=userspace-talos-write" "$LOG_FILE"
    grep -q "talos> exec stderr 2>>/tmp/other.txt" "$LOG_FILE"
    grep -q "talos> exec stderr 2>>/tmp/stdout.txt" "$LOG_FILE"
    grep -c "talos: exec-invalid-path" "$LOG_FILE" | grep -q "^2$"
    grep -q "talos> cat /etc/banner.txt" "$LOG_FILE"
    grep -q "^Talos initramfs fixture" "$LOG_FILE"
    grep -q "$LABEL: final participants=12 expected=12 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_DIRECT_STDERR_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> /bin/stderr 2>/tmp/stderr.txt" "$LOG_FILE"
    grep -q "talos: exec-redirection op=sink source-fd=0x0000000000000002 target-path=/tmp/stderr.txt target-stream=regular-file target-route=volatile-vfs:/tmp/stderr.txt child-only=true shell-restored=true source=shell-redirection-stderr-tmp-stderr" "$LOG_FILE"
    grep -q "talos> /bin/stderr 2>>/tmp/stderr.txt" "$LOG_FILE"
    grep -q "talos: exec-redirection op=append source-fd=0x0000000000000002 target-path=/tmp/stderr.txt target-stream=regular-file target-route=volatile-vfs:/tmp/stderr.txt child-only=true shell-restored=true source=shell-redirection-stderr-tmp-stderr-append" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=stdio-output fd2=regular-file loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -q "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=regular-file route=volatile-vfs:/tmp/stderr.txt source=userspace-talos-write" "$LOG_FILE"
    grep -Eq "talos: waitpid pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -Eq "talos: last-process pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos> cat /tmp/stderr.txt" "$LOG_FILE"
    grep -q "talos: cat path=/tmp/stderr.txt bytes=0x000000000000003e source=volatile-vfs-descriptor-read" "$LOG_FILE"
    grep -q "talos> /bin/stderr" "$LOG_FILE"
    grep -q "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=stderr route=runtime-console0/stderr source=userspace-talos-write" "$LOG_FILE"
    grep -q "talos> /bin/stderr 2>>/tmp/other.txt" "$LOG_FILE"
    grep -q "talos> stderr 2>>/tmp/other.txt" "$LOG_FILE"
    grep -q "talos> /bin/stderr | /bin/stdin 2>>/tmp/stderr.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdin </etc/banner.txt 2>>/tmp/stderr.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdout >>/tmp/stdout.txt" "$LOG_FILE"
    grep -c "talos: exec-invalid-path" "$LOG_FILE" | grep -q "^4$"
    ! grep -q "talos: unexpected-argument" "$LOG_FILE"
    grep -q "talos> cat /etc/banner.txt" "$LOG_FILE"
    grep -q "^Talos initramfs fixture" "$LOG_FILE"
    grep -q "$LABEL: final participants=18 expected=18 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_BARE_NAME_STDERR_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> stderr 2>/tmp/stderr.txt" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stderr source=vfs-open-read" "$LOG_FILE"
    grep -q "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stderr" "$LOG_FILE"
    grep -q "talos: exec-redirection op=sink source-fd=0x0000000000000002 target-path=/tmp/stderr.txt target-stream=regular-file target-route=volatile-vfs:/tmp/stderr.txt child-only=true shell-restored=true source=shell-redirection-stderr-tmp-stderr" "$LOG_FILE"
    grep -q "talos> stderr 2>>/tmp/stderr.txt" "$LOG_FILE"
    grep -q "talos: exec-redirection op=append source-fd=0x0000000000000002 target-path=/tmp/stderr.txt target-stream=regular-file target-route=volatile-vfs:/tmp/stderr.txt child-only=true shell-restored=true source=shell-redirection-stderr-tmp-stderr-append" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=stdio-output fd2=regular-file loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -q "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=regular-file route=volatile-vfs:/tmp/stderr.txt source=userspace-talos-write" "$LOG_FILE"
    grep -Eq "talos: waitpid pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -Eq "talos: last-process pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos> cat /tmp/stderr.txt" "$LOG_FILE"
    grep -q "talos: cat path=/tmp/stderr.txt bytes=0x000000000000003e source=volatile-vfs-descriptor-read" "$LOG_FILE"
    grep -q "talos> stderr" "$LOG_FILE"
    grep -q "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=stderr route=runtime-console0/stderr source=userspace-talos-write" "$LOG_FILE"
    grep -q "talos> stderr 2>>/tmp/other.txt" "$LOG_FILE"
    grep -q "talos> stderr 2>>/var/err.txt" "$LOG_FILE"
    grep -q "talos> stderr | stdin 2>>/tmp/stderr.txt" "$LOG_FILE"
    grep -q "talos> stdin </etc/banner.txt 2>>/tmp/stderr.txt" "$LOG_FILE"
    grep -q "talos> missing 2>>/tmp/stderr.txt" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=12 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=13 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=14 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=15 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=16 status=unknown-command responses=1" "$LOG_FILE"
    grep -c "talos: exec-invalid-path" "$LOG_FILE" | grep -q "^4$"
    ! grep -q "talos: unexpected-argument" "$LOG_FILE"
    grep -q "talos> cat /etc/banner.txt" "$LOG_FILE"
    grep -q "^Talos initramfs fixture" "$LOG_FILE"
    grep -q "$LABEL: final participants=18 expected=18 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
	elif [ "$SHELL_STDERR_REGULAR_FILE_APPEND_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> exec stderr 2>/tmp/stderr.txt" "$LOG_FILE"
    grep -q "talos: exec-redirection op=sink source-fd=0x0000000000000002 target-path=/tmp/stderr.txt target-stream=regular-file target-route=volatile-vfs:/tmp/stderr.txt child-only=true shell-restored=true source=shell-redirection-stderr-tmp-stderr" "$LOG_FILE"
    grep -q "talos> exec stderr 2>>/tmp/stderr.txt" "$LOG_FILE"
    grep -q "talos: exec-redirection op=append source-fd=0x0000000000000002 target-path=/tmp/stderr.txt target-stream=regular-file target-route=volatile-vfs:/tmp/stderr.txt child-only=true shell-restored=true source=shell-redirection-stderr-tmp-stderr-append" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=stdio-output fd2=regular-file loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -q "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=regular-file route=volatile-vfs:/tmp/stderr.txt source=userspace-talos-write" "$LOG_FILE"
    grep -Eq "talos: waitpid pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -Eq "talos: last-process pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos> cat /tmp/stderr.txt" "$LOG_FILE"
    grep -q "talos: cat path=/tmp/stderr.txt bytes=0x000000000000003e source=volatile-vfs-descriptor-read" "$LOG_FILE"
    grep -q "talos> exec stderr" "$LOG_FILE"
    grep -q "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=stderr route=runtime-console0/stderr source=userspace-talos-write" "$LOG_FILE"
    grep -q "talos> exec stdout" "$LOG_FILE"
    grep -q "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=stdout route=runtime-console0/stdout source=userspace-talos-write" "$LOG_FILE"
    grep -q "talos> exec stdout >>/tmp/stdout.txt" "$LOG_FILE"
    grep -q "talos> exec stderr 2>>/tmp/other.txt" "$LOG_FILE"
    grep -q "talos> exec stdout >/tmp/stderr.txt" "$LOG_FILE"
    grep -c "talos: exec-invalid-path" "$LOG_FILE" | grep -q "^3$"
    grep -q "talos> cat /etc/banner.txt" "$LOG_FILE"
    grep -q "^Talos initramfs fixture" "$LOG_FILE"
    grep -q "$LABEL: final participants=16 expected=16 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_STDERR_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> stderr 2>/tmp/stderr.txt" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stderr source=vfs-open-read" "$LOG_FILE"
    grep -Eq "talos: exec-source bytes=0x[0-9a-f]+ digest=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-loader fixture=phase8-program-loader-elf64-aarch64-v1 entry=0x[0-9a-f]+ segments=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-launch launch-boundary=phase8-initial-process-launch-plan-v1 stack-boundary=phase8-initial-user-stack-plan-v1 address-space=0x[0-9a-f]+ materialization=0x[0-9a-f]+ initial-sp=0x[0-9a-f]+" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=stdio-output fd2=regular-file loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -Eq "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stderr argv0-ptr=0x[0-9a-f]+ argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x[0-9a-f]+ copied-startup-bytes=0x000000000000002c source=initial-user-stack-record" "$LOG_FILE"
    grep -q "talos: exec-redirection op=sink source-fd=0x0000000000000002 target-path=/tmp/stderr.txt target-stream=regular-file target-route=volatile-vfs:/tmp/stderr.txt child-only=true shell-restored=true source=shell-redirection-stderr-tmp-stderr" "$LOG_FILE"
    grep -q "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=regular-file route=volatile-vfs:/tmp/stderr.txt source=userspace-talos-write" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=3 status=handled responses=11" "$LOG_FILE"
    grep -Eq "talos: waitpid pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -Eq "talos: last-process pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos> cat /tmp/stderr.txt" "$LOG_FILE"
    grep -q "^Talos userspace stderr fixture" "$LOG_FILE"
    grep -q "talos: cat path=/tmp/stderr.txt bytes=0x000000000000001f source=volatile-vfs-descriptor-read" "$LOG_FILE"
    grep -q "talos> stderr" "$LOG_FILE"
    grep -q "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=stderr route=runtime-console0/stderr source=userspace-talos-write" "$LOG_FILE"
    grep -q "talos> exec stdout" "$LOG_FILE"
    grep -q "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=stdout route=runtime-console0/stdout source=userspace-talos-write" "$LOG_FILE"
    grep -q "talos> /bin/stderr 2>>/tmp/stderr.txt" "$LOG_FILE"
    grep -q "talos: exec-redirection op=append source-fd=0x0000000000000002 target-path=/tmp/stderr.txt target-stream=regular-file target-route=volatile-vfs:/tmp/stderr.txt child-only=true shell-restored=true source=shell-redirection-stderr-tmp-stderr-append" "$LOG_FILE"
    grep -q "talos> /bin/stderr 2> /tmp/stderr.txt" "$LOG_FILE"
    grep -q "talos> exec stdout >/tmp/stderr.txt" "$LOG_FILE"
    grep -c "talos: exec-invalid-path" "$LOG_FILE" | grep -q "^2$"
    grep -q "talos> cat /etc/banner.txt" "$LOG_FILE"
    grep -q "^Talos initramfs fixture" "$LOG_FILE"
    grep -q "$LABEL: final participants=14 expected=14 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_STDOUT_TO_STDERR_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> exec stdout 1>&2" "$LOG_FILE"
    grep -q "^Talos userspace stdout fixture" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stdout source=vfs-open-read" "$LOG_FILE"
    grep -Eq "talos: exec-source bytes=0x[0-9a-f]+ digest=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-loader fixture=phase8-program-loader-elf64-aarch64-v1 entry=0x[0-9a-f]+ segments=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-launch launch-boundary=phase8-initial-process-launch-plan-v1 stack-boundary=phase8-initial-user-stack-plan-v1 address-space=0x[0-9a-f]+ materialization=0x[0-9a-f]+ initial-sp=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-descriptors owner=0x[0-9a-f]+ inherited-count=0x0000000000000003 fd0=stdio-input fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -Eq "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stdout argv0-ptr=0x[0-9a-f]+ argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x[0-9a-f]+ copied-startup-bytes=0x000000000000002c source=initial-user-stack-record" "$LOG_FILE"
    grep -q "talos: exec-redirection op=dup source-fd=0x0000000000000001 target-fd=0x0000000000000002 target-stream=stderr target-route=runtime-console0/stderr child-only=true shell-restored=true source=shell-redirection-1-to-2" "$LOG_FILE"
    grep -q "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=stderr route=runtime-console0/stderr source=userspace-talos-write" "$LOG_FILE"
    grep -Eq "talos: exec-lifecycle pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true" "$LOG_FILE"
    grep -q "talos: exec-status boundary=lower-aarch64-svc-status-equivalent marker=0x0000000000007a10 status=0x0000000000000000 complete=true source=lifecycle-record" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=3 status=handled responses=11" "$LOG_FILE"
    grep -Eq "talos: waitpid pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -Eq "talos: last-process pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos> exec stdout" "$LOG_FILE"
    grep -q "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=stdout route=runtime-console0/stdout source=userspace-talos-write" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=6 status=handled responses=10" "$LOG_FILE"
    grep -q "talos> exec stdout 2>&3" "$LOG_FILE"
    grep -q "talos> exec stdout 1>file" "$LOG_FILE"
    grep -Fq "talos> exec stdout | stderr" "$LOG_FILE"
    grep -c "talos: exec-invalid-path" "$LOG_FILE" | grep -q "^3$"
    grep -q "talos> cat /etc/banner.txt" "$LOG_FILE"
    grep -q "^Talos initramfs fixture" "$LOG_FILE"
    grep -q "$LABEL: final participants=12 expected=12 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_STDOUT_CLOSE_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> exec stdout 1>&-" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stdout source=vfs-open-read" "$LOG_FILE"
    grep -Eq "talos: exec-source bytes=0x[0-9a-f]+ digest=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-loader fixture=phase8-program-loader-elf64-aarch64-v1 entry=0x[0-9a-f]+ segments=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-launch launch-boundary=phase8-initial-process-launch-plan-v1 stack-boundary=phase8-initial-user-stack-plan-v1 address-space=0x[0-9a-f]+ materialization=0x[0-9a-f]+ initial-sp=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-descriptors owner=0x[0-9a-f]+ inherited-count=0x0000000000000002 fd0=stdio-input fd1=closed fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -Eq "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stdout argv0-ptr=0x[0-9a-f]+ argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x[0-9a-f]+ copied-startup-bytes=0x000000000000002c source=initial-user-stack-record" "$LOG_FILE"
    grep -q "talos: exec-redirection op=close source-fd=0x0000000000000001 result=closed-descriptor child-only=true shell-restored=true source=shell-redirection-1-close" "$LOG_FILE"
    grep -q "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0xfffffffffffffff7 stream=closed route=closed-descriptor source=userspace-talos-write" "$LOG_FILE"
    grep -Eq "talos: exec-lifecycle pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true" "$LOG_FILE"
    grep -q "talos: exec-status boundary=lower-aarch64-svc-status-equivalent marker=0x0000000000007a10 status=0x0000000000000000 complete=true source=lifecycle-record" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=3 status=handled responses=11" "$LOG_FILE"
    grep -Eq "talos: waitpid pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -Eq "talos: last-process pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos> exec stdout" "$LOG_FILE"
    grep -q "^Talos userspace stdout fixture" "$LOG_FILE"
    grep -q "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=stdout route=runtime-console0/stdout source=userspace-talos-write" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=6 status=handled responses=10" "$LOG_FILE"
    grep -q "talos> exec stderr 2>file" "$LOG_FILE"
    grep -q "talos> exec stdout 1>file" "$LOG_FILE"
    grep -Fq "talos> exec stdout | stderr" "$LOG_FILE"
    grep -c "talos: exec-invalid-path" "$LOG_FILE" | grep -q "^3$"
    grep -q "talos> cat /etc/banner.txt" "$LOG_FILE"
    grep -q "^Talos initramfs fixture" "$LOG_FILE"
    grep -q "$LABEL: final participants=12 expected=12 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_STDERR_CLOSE_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> exec stderr 2>&-" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stderr source=vfs-open-read" "$LOG_FILE"
    grep -Eq "talos: exec-source bytes=0x[0-9a-f]+ digest=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-loader fixture=phase8-program-loader-elf64-aarch64-v1 entry=0x[0-9a-f]+ segments=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-launch launch-boundary=phase8-initial-process-launch-plan-v1 stack-boundary=phase8-initial-user-stack-plan-v1 address-space=0x[0-9a-f]+ materialization=0x[0-9a-f]+ initial-sp=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-descriptors owner=0x[0-9a-f]+ inherited-count=0x0000000000000002 fd0=stdio-input fd1=stdio-output fd2=closed loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -Eq "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stderr argv0-ptr=0x[0-9a-f]+ argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x[0-9a-f]+ copied-startup-bytes=0x000000000000002c source=initial-user-stack-record" "$LOG_FILE"
    grep -q "talos: exec-redirection op=close source-fd=0x0000000000000002 result=closed-descriptor child-only=true shell-restored=true source=shell-redirection-2-close" "$LOG_FILE"
    grep -q "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0xfffffffffffffff7 stream=closed route=closed-descriptor source=userspace-talos-write" "$LOG_FILE"
    grep -Eq "talos: exec-lifecycle pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true" "$LOG_FILE"
    grep -q "talos: exec-status boundary=lower-aarch64-svc-status-equivalent marker=0x0000000000007a10 status=0x0000000000000000 complete=true source=lifecycle-record" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=3 status=handled responses=11" "$LOG_FILE"
    grep -Eq "talos: waitpid pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -Eq "talos: last-process pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos> exec stderr" "$LOG_FILE"
    grep -q "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=stderr route=runtime-console0/stderr source=userspace-talos-write" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=6 status=handled responses=10" "$LOG_FILE"
    grep -q "talos> exec stdout 1>&-" "$LOG_FILE"
    grep -q "talos: exec-redirection op=close source-fd=0x0000000000000001 result=closed-descriptor child-only=true shell-restored=true source=shell-redirection-1-close" "$LOG_FILE"
    grep -q "talos> exec stderr 2>file" "$LOG_FILE"
    grep -q "talos: exec-invalid-path" "$LOG_FILE"
    grep -q "talos> cat /etc/banner.txt" "$LOG_FILE"
    grep -q "^Talos initramfs fixture" "$LOG_FILE"
    grep -q "$LABEL: final participants=12 expected=12 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_STDERR_TO_STDOUT_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> exec stderr 2>&1" "$LOG_FILE"
    grep -q "^Talos userspace stderr fixture" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stderr source=vfs-open-read" "$LOG_FILE"
    grep -Eq "talos: exec-source bytes=0x[0-9a-f]+ digest=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-loader fixture=phase8-program-loader-elf64-aarch64-v1 entry=0x[0-9a-f]+ segments=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-launch launch-boundary=phase8-initial-process-launch-plan-v1 stack-boundary=phase8-initial-user-stack-plan-v1 address-space=0x[0-9a-f]+ materialization=0x[0-9a-f]+ initial-sp=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-descriptors owner=0x[0-9a-f]+ inherited-count=0x0000000000000003 fd0=stdio-input fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -Eq "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stderr argv0-ptr=0x[0-9a-f]+ argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x[0-9a-f]+ copied-startup-bytes=0x000000000000002c source=initial-user-stack-record" "$LOG_FILE"
    grep -q "talos: exec-redirection op=dup source-fd=0x0000000000000002 target-fd=0x0000000000000001 target-stream=stdout target-route=runtime-console0/stdout child-only=true shell-restored=true source=shell-redirection-2-to-1" "$LOG_FILE"
    grep -q "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=stdout route=runtime-console0/stdout source=userspace-talos-write" "$LOG_FILE"
    grep -Eq "talos: exec-lifecycle pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true" "$LOG_FILE"
    grep -q "talos: exec-status boundary=lower-aarch64-svc-status-equivalent marker=0x0000000000007a10 status=0x0000000000000000 complete=true source=lifecycle-record" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=3 status=handled responses=11" "$LOG_FILE"
    grep -Eq "talos: waitpid pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -Eq "talos: last-process pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos> exec stderr" "$LOG_FILE"
    grep -q "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=stderr route=runtime-console0/stderr source=userspace-talos-write" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=6 status=handled responses=10" "$LOG_FILE"
    grep -q "talos> exec stdout 1>&2" "$LOG_FILE"
    grep -q "talos: exec-redirection op=dup source-fd=0x0000000000000001 target-fd=0x0000000000000002 target-stream=stderr target-route=runtime-console0/stderr child-only=true shell-restored=true source=shell-redirection-1-to-2" "$LOG_FILE"
    grep -q "talos> exec stderr 2>file" "$LOG_FILE"
    grep -q "talos: exec-invalid-path" "$LOG_FILE"
    grep -q "talos> cat /etc/banner.txt" "$LOG_FILE"
    grep -q "^Talos initramfs fixture" "$LOG_FILE"
    grep -q "$LABEL: final participants=12 expected=12 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_STDERR_SMOKE" -eq 1 ]; then
    grep -q "talos> exec stderr" "$LOG_FILE"
    grep -q "^Talos userspace stderr fixture" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stderr source=vfs-open-read" "$LOG_FILE"
    grep -Eq "talos: exec-source bytes=0x[0-9a-f]+ digest=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-loader fixture=phase8-program-loader-elf64-aarch64-v1 entry=0x[0-9a-f]+ segments=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-launch launch-boundary=phase8-initial-process-launch-plan-v1 stack-boundary=phase8-initial-user-stack-plan-v1 address-space=0x[0-9a-f]+ materialization=0x[0-9a-f]+ initial-sp=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-descriptors owner=0x[0-9a-f]+ inherited-count=0x0000000000000003 fd0=stdio-input fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -Eq "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stderr argv0-ptr=0x[0-9a-f]+ argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x[0-9a-f]+ copied-startup-bytes=0x000000000000002c source=initial-user-stack-record" "$LOG_FILE"
    grep -q "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=stderr route=runtime-console0/stderr source=userspace-talos-write" "$LOG_FILE"
    grep -Eq "talos: exec-lifecycle pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true" "$LOG_FILE"
    grep -q "talos: exec-status boundary=lower-aarch64-svc-status-equivalent marker=0x0000000000007a10 status=0x0000000000000000 complete=true source=lifecycle-record" "$LOG_FILE"
    grep -q "$LABEL: line command=3 hex=65 78 65 63 20 73 74 64 65 72 72" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=3 status=handled responses=10" "$LOG_FILE"
    grep -Eq "talos: waitpid pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -Eq "talos: last-process pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos> exec status42" "$LOG_FILE"
    grep -Eq "talos: exec-lifecycle pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true" "$LOG_FILE"
    grep -Eq "talos: waitpid pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos> exec init" "$LOG_FILE"
    grep -Eq "talos: exec-lifecycle pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/init state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true" "$LOG_FILE"
    grep -q "talos> exec zero" "$LOG_FILE"
    grep -Eq "talos: exec-lifecycle pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/zero state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true" "$LOG_FILE"
    grep -q "talos> exec /bin/status42 gamma" "$LOG_FILE"
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
    grep -q "$LABEL: final participants=18 expected=18 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_STDIN_SMOKE" -eq 1 ] || [ "$SHELL_STDIN_EOF_SMOKE" -eq 1 ] || [ "$SHELL_STDIN_TERMINAL_EOF_SMOKE" -eq 1 ] || [ "$SHELL_STDIN_READINESS_SMOKE" -eq 1 ] || [ "$SHELL_STDIN_BOUNDED_WAIT_SMOKE" -eq 1 ] || [ "$SHELL_STDIN_SCHEDULER_WAIT_SMOKE" -eq 1 ]; then
    grep -q "talos> exec stdin" "$LOG_FILE"
    if [ "$SHELL_STDIN_BOUNDED_WAIT_SMOKE" -eq 1 ] || [ "$SHELL_STDIN_SCHEDULER_WAIT_SMOKE" -eq 1 ]; then
        grep -q "^Talos userspace stdin fixture no-data: readiness" "$LOG_FILE"
        grep -Eq "talos: stdin-wait task=0x[0-9a-f]+ fd=0x0000000000000000 sleep-state=blocked wake-state=blocked wait-cycles=0x0000000000000000 result=sleep source=scheduler-runtime-console-readiness" "$LOG_FILE"
        grep -Eq "talos: stdin-wait task=0x[0-9a-f]+ fd=0x0000000000000000 sleep-state=blocked wake-state=runnable wait-cycles=0x[0-9a-f]+ result=wakeup/resume source=scheduler-runtime-console-readiness" "$LOG_FILE"
        grep -q "^Talos userspace stdin fixture read: talos-console0" "$LOG_FILE"
    elif [ "$SHELL_STDIN_TERMINAL_EOF_SMOKE" -eq 1 ]; then
        grep -q "^Talos userspace stdin fixture read-result: terminal-eof" "$LOG_FILE"
        ! grep -q "^Talos userspace stdin fixture no-data: readiness" "$LOG_FILE"
        ! grep -q "talos: stdin-wait task=" "$LOG_FILE"
    elif [ "$SHELL_STDIN_EOF_SMOKE" -eq 1 ] || [ "$SHELL_STDIN_READINESS_SMOKE" -eq 1 ]; then
        grep -q "^Talos userspace stdin fixture no-data: readiness" "$LOG_FILE"
        grep -Eq "talos: stdin-wait task=0x[0-9a-f]+ fd=0x0000000000000000 sleep-state=blocked wake-state=blocked wait-cycles=0x0000000000000000 result=sleep source=scheduler-runtime-console-readiness" "$LOG_FILE"
        grep -Eq "talos: stdin-wait task=0x[0-9a-f]+ fd=0x0000000000000000 sleep-state=blocked wake-state=runnable wait-cycles=0x[0-9a-f]+ result=timeout/no-false-eof source=scheduler-runtime-console-readiness" "$LOG_FILE"
    else
        grep -q "^Talos userspace stdin fixture read: talos-console0" "$LOG_FILE"
    fi
    grep -q "talos: exec path=/bin/stdin source=vfs-open-read" "$LOG_FILE"
    grep -Eq "talos: exec-source bytes=0x[0-9a-f]+ digest=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-loader fixture=phase8-program-loader-elf64-aarch64-v1 entry=0x[0-9a-f]+ segments=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-launch launch-boundary=phase8-initial-process-launch-plan-v1 stack-boundary=phase8-initial-user-stack-plan-v1 address-space=0x[0-9a-f]+ materialization=0x[0-9a-f]+ initial-sp=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-descriptors owner=0x[0-9a-f]+ inherited-count=0x0000000000000003 fd0=stdio-input fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -Eq "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stdin argv0-ptr=0x[0-9a-f]+ argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x[0-9a-f]+ copied-startup-bytes=0x000000000000002b source=initial-user-stack-record" "$LOG_FILE"
    if [ "$SHELL_STDIN_BOUNDED_WAIT_SMOKE" -eq 1 ] || [ "$SHELL_STDIN_SCHEDULER_WAIT_SMOKE" -eq 1 ]; then
        grep -Eq "talos: exec-stdin fd=0x0000000000000000 bytes=0x000000000000000e return=0x000000000000000e read-source=runtime-console0/local-input stdout-fd=0x0000000000000001 stdout-bytes=0x0000000000000064 stdout-return=0x0000000000000064 source=userspace-talos-read\\+userspace-talos-write read-result=scheduler-wait/delayed-input readiness-observations=0x[0-9a-f]+ scheduler-wait-result=wakeup/resume scheduler-wait-cycles=0x[0-9a-f]+ scheduler-wait-source=scheduler-runtime-console-readiness" "$LOG_FILE"
    elif [ "$SHELL_STDIN_TERMINAL_EOF_SMOKE" -eq 1 ]; then
        grep -q "talos: exec-stdin fd=0x0000000000000000 bytes=0x0000000000000000 return=0x0000000000000000 read-source=runtime-console0/local-input stdout-fd=0x0000000000000001 stdout-bytes=0x0000000000000038 stdout-return=0x0000000000000038 source=userspace-talos-read+userspace-talos-write read-result=terminal-eof" "$LOG_FILE"
    elif [ "$SHELL_STDIN_EOF_SMOKE" -eq 1 ] || [ "$SHELL_STDIN_READINESS_SMOKE" -eq 1 ]; then
        grep -q "talos: exec-stdin fd=0x0000000000000000 bytes=0x0000000000000000 return=0xfffffffffffffff5 read-source=runtime-console0/local-input stdout-fd=0x0000000000000001 stdout-bytes=0x0000000000000031 stdout-return=0x0000000000000031 source=userspace-talos-read+userspace-talos-write read-result=readiness/no-data" "$LOG_FILE"
        grep -q "scheduler-wait-result=timeout/no-false-eof" "$LOG_FILE"
    else
        grep -q "talos: exec-stdin fd=0x0000000000000000 bytes=0x000000000000000e return=0x000000000000000e read-source=runtime-console0/local-input stdout-fd=0x0000000000000001 stdout-bytes=0x0000000000000033 stdout-return=0x0000000000000033 source=userspace-talos-read+userspace-talos-write" "$LOG_FILE"
    fi
    grep -Eq "talos: exec-lifecycle pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true" "$LOG_FILE"
    grep -q "talos: exec-status boundary=lower-aarch64-svc-status-equivalent marker=0x0000000000007a10 status=0x0000000000000000 complete=true source=lifecycle-record" "$LOG_FILE"
    grep -q "$LABEL: line command=3 hex=65 78 65 63 20 73 74 64 69 6e" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=3 status=handled responses=10" "$LOG_FILE"
    grep -Eq "talos: waitpid pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -Eq "talos: last-process pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos> exec stdout" "$LOG_FILE"
    grep -q "^Talos userspace stdout fixture" "$LOG_FILE"
    grep -q "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=stdout route=runtime-console0/stdout source=userspace-talos-write" "$LOG_FILE"
    grep -Eq "talos: waitpid pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos> exec init" "$LOG_FILE"
    grep -Eq "talos: exec-lifecycle pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/init state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true" "$LOG_FILE"
    grep -q "talos> exec zero" "$LOG_FILE"
    grep -Eq "talos: exec-lifecycle pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/zero state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true" "$LOG_FILE"
    grep -q "talos> exec /bin/status42 gamma" "$LOG_FILE"
    grep -Eq "talos: exec-startup-abi state=literal-argv-absolute-empty-envp argc=0x0000000000000002 argv0=/bin/status42 argv1=gamma argv0-ptr=0x[0-9a-f]+ argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x[0-9a-f]+ copied-startup-bytes=0x000000000000003c source=initial-user-stack-record" "$LOG_FILE"
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
    grep -q "$LABEL: final participants=18 expected=18 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_STDOUT_SMOKE" -eq 1 ]; then
    grep -q "talos> exec stdout" "$LOG_FILE"
    grep -q "^Talos userspace stdout fixture" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stdout source=vfs-open-read" "$LOG_FILE"
    grep -Eq "talos: exec-source bytes=0x[0-9a-f]+ digest=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-loader fixture=phase8-program-loader-elf64-aarch64-v1 entry=0x[0-9a-f]+ segments=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-launch launch-boundary=phase8-initial-process-launch-plan-v1 stack-boundary=phase8-initial-user-stack-plan-v1 address-space=0x[0-9a-f]+ materialization=0x[0-9a-f]+ initial-sp=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-descriptors owner=0x[0-9a-f]+ inherited-count=0x0000000000000003 fd0=stdio-input fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -Eq "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stdout argv0-ptr=0x[0-9a-f]+ argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x[0-9a-f]+ copied-startup-bytes=0x000000000000002c source=initial-user-stack-record" "$LOG_FILE"
    grep -q "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=stdout route=runtime-console0/stdout source=userspace-talos-write" "$LOG_FILE"
    grep -Eq "talos: exec-lifecycle pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true" "$LOG_FILE"
    grep -q "talos: exec-status boundary=lower-aarch64-svc-status-equivalent marker=0x0000000000007a10 status=0x0000000000000000 complete=true source=lifecycle-record" "$LOG_FILE"
    grep -q "$LABEL: line command=3 hex=65 78 65 63 20 73 74 64 6f 75 74" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=3 status=handled responses=10" "$LOG_FILE"
    grep -Eq "talos: waitpid pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -Eq "talos: last-process pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos> exec status42" "$LOG_FILE"
    grep -Eq "talos: exec-lifecycle pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true" "$LOG_FILE"
    grep -Eq "talos: waitpid pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos> exec init" "$LOG_FILE"
    grep -Eq "talos: exec-lifecycle pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/init state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true" "$LOG_FILE"
    grep -q "talos> exec zero" "$LOG_FILE"
    grep -Eq "talos: exec-lifecycle pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/zero state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true" "$LOG_FILE"
    grep -q "talos> exec /bin/status42 gamma" "$LOG_FILE"
    grep -Eq "talos: exec-startup-abi state=literal-argv-absolute-empty-envp argc=0x0000000000000002 argv0=/bin/status42 argv1=gamma argv0-ptr=0x[0-9a-f]+ argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x[0-9a-f]+ copied-startup-bytes=0x000000000000003c source=initial-user-stack-record" "$LOG_FILE"
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
    grep -q "$LABEL: final participants=18 expected=18 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_PROCESS_STATUS_VFS_SMOKE" -eq 1 ]; then
    grep -q "talos> cat /proc/talos/processes" "$LOG_FILE"
    grep -q "talos-processes-v1" "$LOG_FILE"
    grep -q "slot=0 capacity=3 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true wait-consumed=false job-state=foreground source=bounded-process-table" "$LOG_FILE"
    grep -q "slot=0 capacity=3 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true wait-consumed=true job-state=foreground source=bounded-process-table" "$LOG_FILE"
    grep -q "slot=0 capacity=3 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=false job-state=foreground source=bounded-process-table" "$LOG_FILE"
    grep -q "slot=1 capacity=3 pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=false job-state=foreground source=bounded-process-table" "$LOG_FILE"
    grep -q "slot=0 capacity=3 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table" "$LOG_FILE"
    grep -q "slot=1 capacity=3 pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table" "$LOG_FILE"
    grep -q "slot=0 capacity=3 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true wait-consumed=false job-state=completed source=bounded-process-table" "$LOG_FILE"
    grep -q "talos> cat /proc/talos" "$LOG_FILE"
    grep -q "talos: not-found" "$LOG_FILE"
    grep -q "talos> cat /etc/banner.txt" "$LOG_FILE"
    grep -q "^Talos initramfs fixture" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=16 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: final participants=19 expected=19 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_PS_COMMAND_VFS_SMOKE" -eq 1 ]; then
    grep -q "talos> cat /proc/talos/processes" "$LOG_FILE"
    grep -q "talos> ps" "$LOG_FILE"
    grep -q "talos-processes-v1" "$LOG_FILE"
    grep -q "slot=0 capacity=3 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true wait-consumed=false job-state=foreground source=bounded-process-table" "$LOG_FILE"
    grep -q "slot=0 capacity=3 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true wait-consumed=true job-state=foreground source=bounded-process-table" "$LOG_FILE"
    grep -q "slot=0 capacity=3 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=false job-state=foreground source=bounded-process-table" "$LOG_FILE"
    grep -q "slot=1 capacity=3 pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=false job-state=foreground source=bounded-process-table" "$LOG_FILE"
    grep -q "slot=0 capacity=3 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table" "$LOG_FILE"
    grep -q "slot=1 capacity=3 pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table" "$LOG_FILE"
    grep -q "slot=0 capacity=3 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true wait-consumed=false job-state=completed source=bounded-process-table" "$LOG_FILE"
    grep -q "talos> ps -a" "$LOG_FILE"
    grep -q "talos> ps extra" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=19 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=20 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "talos> cat /etc/banner.txt" "$LOG_FILE"
    grep -q "^Talos initramfs fixture" "$LOG_FILE"
    grep -q "talos> jobs" "$LOG_FILE"
    grep -q "$LABEL: final participants=23 expected=23 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_WAITPID_ANY_COMPLETED_SMOKE" -eq 1 ]; then
    grep -q "talos> waitpid" "$LOG_FILE"
    grep -q "talos: waitpid no-child source=lifecycle-record" "$LOG_FILE"
    grep -q "talos> exec /bin/status42" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=5 status=handled responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=6 status=handled responses=1" "$LOG_FILE"
    grep -q "talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos> exec stdout | exec stdin" "$LOG_FILE"
    grep -q "talos: pipeline-lifecycle-status record=phase12-local-pipeline-distinct-process-lifecycle-status-record-v1 pipeline=0x0000000000000001 producer-pid=0x0000000000100001 producer-path=/bin/stdout producer-state=exited producer-status=0x0000000000000000 producer-observed-status=0x0000000000000000 producer-reaped=true consumer-pid=0x0000000000100002 consumer-path=/bin/stdin consumer-state=exited consumer-status=0x0000000000000000 consumer-observed-status=0x0000000000000000 consumer-reaped=true source=kernel-owned-pipeline-lifecycle-status-record" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos: waitpid no-child pid=0x0000000000100002 source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos> exec /bin/status42 &" "$LOG_FILE"
    grep -q "talos: background-job id=0x0000000000000001 pid=0x0000000000100001 command=/bin/status42 state=completed status=0x000000000000002a observed-status=0x000000000000002a reaped=true shell-responsive=observed source=background-vfs-exec-accounting" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=background-job-lifecycle-record" "$LOG_FILE"
    grep -q "talos: jobs none source=background-vfs-exec-accounting" "$LOG_FILE"
    grep -q "talos> exec /bin/zero &" "$LOG_FILE"
    grep -q "talos: background-job id=0x0000000000000002 pid=0x0000000000100002 command=/bin/zero state=completed status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true shell-responsive=observed source=background-vfs-exec-accounting" "$LOG_FILE"
    grep -q "talos: jobs id=0x0000000000000002 pid=0x0000000000100002 command=/bin/zero state=completed status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=background-vfs-exec-accounting" "$LOG_FILE"
    grep -q "$LABEL: final participants=19 expected=19 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
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
    grep -q "talos> exec stdout | exec stdin" "$LOG_FILE"
    grep -q "talos: pipeline-lifecycle-status record=phase12-local-pipeline-distinct-process-lifecycle-status-record-v1 pipeline=0x0000000000000001 producer-pid=0x0000000000100001 producer-path=/bin/stdout producer-state=exited producer-status=0x0000000000000000 producer-observed-status=0x0000000000000000 producer-reaped=true consumer-pid=0x0000000000100002 consumer-path=/bin/stdin consumer-state=exited consumer-status=0x0000000000000000 consumer-observed-status=0x0000000000000000 consumer-reaped=true source=kernel-owned-pipeline-lifecycle-status-record" "$LOG_FILE"
    grep -q "talos> waitpid 0x100001" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos> waitpid 0x100002" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos: waitpid no-child pid=0x0000000000100001 source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos> waitpid bogus" "$LOG_FILE"
    grep -q "talos: waitpid invalid-pid source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos> waitpid 0x0" "$LOG_FILE"
    grep -q "talos: waitpid unsupported-pid pid=0x0000000000000000 source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos> cat /etc/banner.txt" "$LOG_FILE"
    grep -q "^Talos initramfs fixture" "$LOG_FILE"
    grep -q "talos> exec /bin/status42 &" "$LOG_FILE"
    grep -q "talos: background-job id=0x0000000000000001 pid=0x0000000000100001 command=/bin/status42 state=completed status=0x000000000000002a observed-status=0x000000000000002a reaped=true shell-responsive=observed source=background-vfs-exec-accounting" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=background-job-lifecycle-record" "$LOG_FILE"
    grep -q "talos> exec /bin/zero &" "$LOG_FILE"
    grep -q "talos: background-job id=0x0000000000000002 pid=0x0000000000100002 command=/bin/zero state=completed status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true shell-responsive=observed source=background-vfs-exec-accounting" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/zero state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=background-job-lifecycle-record" "$LOG_FILE"
    grep -q "talos: waitpid no-child pid=0x0000000000100002 source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "$LABEL: final participants=33 expected=33 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
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
    grep -q "$LABEL: dispatch command=3 status=handled responses=10" "$LOG_FILE"
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
    grep -q "$LABEL: dispatch command=3 status=handled responses=10" "$LOG_FILE"
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
elif [ "$SHELL_ABSOLUTE_PATH_VFS_COMMAND_SMOKE" -eq 1 ]; then
    grep -q "talos> /bin/status42" "$LOG_FILE"
    grep -q "talos: exec path=/bin/status42 source=vfs-open-read" "$LOG_FILE"
    grep -Eq "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/status42 argv0-ptr=0x[0-9a-f]+ argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x[0-9a-f]+ copied-startup-bytes=0x000000000000002e source=initial-user-stack-record" "$LOG_FILE"
    grep -Eq "talos: exec-lifecycle pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true" "$LOG_FILE"
    grep -Eq "talos: waitpid pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -Eq "talos: last-process pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos> cat /proc/talos/processes" "$LOG_FILE"
    grep -q "path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true wait-consumed=true job-state=foreground source=bounded-process-table" "$LOG_FILE"
    grep -q "talos> ps" "$LOG_FILE"
    grep -q "talos> /bin/status42 alpha beta" "$LOG_FILE"
    grep -Eq "talos: exec-descriptors owner=0x[0-9a-f]+ inherited-count=0x0000000000000003 fd0=stdio-input fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -Eq "talos: exec-startup-abi state=literal-argv-absolute-empty-envp argc=0x0000000000000003 argv0=/bin/status42 argv1=alpha argv2=beta argv0-ptr=0x[0-9a-f]+ argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x[0-9a-f]+ copied-startup-bytes=0x0000000000000049 source=initial-user-stack-record" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=8 status=handled responses=10" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=9 status=handled responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=10 status=handled responses=1" "$LOG_FILE"
    grep -q "talos> /bin/status42 alpha beta gamma delta" "$LOG_FILE"
    grep -q "talos> /bin/status42 *" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=11 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=12 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "talos> /missing" "$LOG_FILE"
    grep -q "talos: exec-not-found" "$LOG_FILE"
    grep -q "talos> bin/status42" "$LOG_FILE"
    grep -q "talos: unknown-command" "$LOG_FILE"
    grep -q "talos> /bin" "$LOG_FILE"
    grep -q "talos> /etc/banner.txt" "$LOG_FILE"
    grep -q "talos: exec-not-executable" "$LOG_FILE"
    grep -q "talos> status42" "$LOG_FILE"
    grep -q "talos: exec path=/bin/status42 source=vfs-open-read" "$LOG_FILE"
    grep -Eq "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/status42 argv0-ptr=0x[0-9a-f]+ argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x[0-9a-f]+ copied-startup-bytes=0x000000000000002e source=initial-user-stack-record" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=13 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=14 status=unknown-command responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=15 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=16 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=17 status=handled responses=10" "$LOG_FILE"
    grep -q "$LABEL: final participants=18 expected=18 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_ABSOLUTE_PATH_VFS_PIPELINE_SMOKE" -eq 1 ]; then
    grep -q "talos> /bin/stdout | /bin/stdin" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stdout source=vfs-open-read" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stdin source=vfs-open-read" "$LOG_FILE"
    grep -q "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdout consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000001f bytes-read=0x000000000000001f writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-stdout-to-stdin" "$LOG_FILE"
    grep -q "talos: pipeline-lifecycle-status record=phase12-local-pipeline-distinct-process-lifecycle-status-record-v1 pipeline=0x0000000000000001 producer-pid=0x0000000000100001 producer-path=/bin/stdout" "$LOG_FILE"
    grep -q "consumer-pid=0x0000000000100002 consumer-path=/bin/stdin" "$LOG_FILE"
    grep -q "Talos userspace stdin fixture read: Talos userspace stdout fixture" "$LOG_FILE"
    grep -q "talos> waitpid 0x100001" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos> waitpid 0x100002" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos: last-process pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos: pipestatus participants=0x0000000000000002 default-status=0x0000000000000000 pipefail-status=0x0000000000000000 semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status" "$LOG_FILE"
    grep -q "path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table" "$LOG_FILE"
    grep -q "path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table" "$LOG_FILE"
    grep -q "talos> ps" "$LOG_FILE"
    grep -q "talos> /bin/stdout | exec stdin" "$LOG_FILE"
    grep -q "talos> exec stdout | /bin/stdin" "$LOG_FILE"
    grep -q "talos> status42 | /bin/stdin" "$LOG_FILE"
    grep -q "talos> /bin/stdout | /missing" "$LOG_FILE"
    grep -q "talos> /bin/stdout | /bin/stdin | /bin/stdin" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=10 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=11 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=12 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=13 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=14 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: final participants=15 expected=15 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_DIRECT_PIPELINE_STAGE_ARGV_SMOKE" -eq 1 ]; then
    grep -q "talos> /bin/stdout alpha | /bin/stdin beta" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stdout source=vfs-open-read" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stdin source=vfs-open-read" "$LOG_FILE"
    grep -Eq "talos: exec-startup-abi state=literal-argv-absolute-empty-envp argc=0x0000000000000002 argv0=/bin/stdout argv1=alpha argv0-ptr=0x[0-9a-f]+ argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x[0-9a-f]+ copied-startup-bytes=0x[0-9a-f]+ source=initial-user-stack-record" "$LOG_FILE"
    grep -Eq "talos: exec-startup-abi state=literal-argv-absolute-empty-envp argc=0x0000000000000002 argv0=/bin/stdin argv1=beta argv0-ptr=0x[0-9a-f]+ argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x[0-9a-f]+ copied-startup-bytes=0x[0-9a-f]+ source=initial-user-stack-record" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=pipe-endpoint fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=pipe-endpoint fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -q "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdout consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000001f bytes-read=0x000000000000001f writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-stdout-to-stdin" "$LOG_FILE"
    grep -q "talos: pipeline-lifecycle-status record=phase12-local-pipeline-distinct-process-lifecycle-status-record-v1 pipeline=0x0000000000000001 producer-pid=0x0000000000100001 producer-path=/bin/stdout" "$LOG_FILE"
    grep -q "consumer-pid=0x0000000000100002 consumer-path=/bin/stdin" "$LOG_FILE"
    grep -q "Talos userspace stdin fixture read: Talos userspace stdout fixture" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos: last-process pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos: pipestatus participants=0x0000000000000002 default-status=0x0000000000000000 pipefail-status=0x0000000000000000 semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status" "$LOG_FILE"
    grep -q "path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table" "$LOG_FILE"
    grep -q "path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table" "$LOG_FILE"
    grep -q "talos> /bin/stdout alpha beta | /bin/stdin" "$LOG_FILE"
    grep -q "talos> /bin/stdout * | /bin/stdin" "$LOG_FILE"
    grep -q "talos> /bin/stdout alpha | /bin/stdin beta gamma" "$LOG_FILE"
    grep -q "talos> /bin/stdout alpha | /bin/stdin *" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=10 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=11 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=12 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=13 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "talos> /bin/stdout | /bin/stdin" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=14 status=handled responses=22" "$LOG_FILE"
    grep -q "$LABEL: final participants=15 expected=15 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_BARE_NAME_PIPELINE_STAGE_ARGV_SMOKE" -eq 1 ]; then
    grep -q "talos> stdout alpha | stdin beta" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stdout source=vfs-open-read" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stdin source=vfs-open-read" "$LOG_FILE"
    grep -Eq "talos: exec-startup-abi state=literal-argv-absolute-empty-envp argc=0x0000000000000002 argv0=/bin/stdout argv1=alpha argv0-ptr=0x[0-9a-f]+ argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x[0-9a-f]+ copied-startup-bytes=0x[0-9a-f]+ source=initial-user-stack-record" "$LOG_FILE"
    grep -Eq "talos: exec-startup-abi state=literal-argv-absolute-empty-envp argc=0x0000000000000002 argv0=/bin/stdin argv1=beta argv0-ptr=0x[0-9a-f]+ argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x[0-9a-f]+ copied-startup-bytes=0x[0-9a-f]+ source=initial-user-stack-record" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=pipe-endpoint fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=pipe-endpoint fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -q "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdout consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000001f bytes-read=0x000000000000001f writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-stdout-to-stdin" "$LOG_FILE"
    grep -q "talos: pipeline-lifecycle-status record=phase12-local-pipeline-distinct-process-lifecycle-status-record-v1 pipeline=0x0000000000000001 producer-pid=0x0000000000100001 producer-path=/bin/stdout" "$LOG_FILE"
    grep -q "consumer-pid=0x0000000000100002 consumer-path=/bin/stdin" "$LOG_FILE"
    grep -q "Talos userspace stdin fixture read: Talos userspace stdout fixture" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos: last-process pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos: pipestatus participants=0x0000000000000002 default-status=0x0000000000000000 pipefail-status=0x0000000000000000 semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status" "$LOG_FILE"
    grep -q "path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table" "$LOG_FILE"
    grep -q "path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table" "$LOG_FILE"
    grep -q "talos> stdout alpha beta | stdin" "$LOG_FILE"
    grep -q "talos> stdout * | stdin" "$LOG_FILE"
    grep -q "talos> stdout alpha | stdin beta gamma" "$LOG_FILE"
    grep -q "talos> stdout alpha | stdin *" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=10 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=11 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=12 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=13 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "talos> stdout | stdin" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=14 status=handled responses=22" "$LOG_FILE"
    grep -q "$LABEL: final participants=15 expected=15 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_BARE_NAME_VFS_PIPELINE_SMOKE" -eq 1 ]; then
    grep -q "talos> stdout | stdin" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stdout source=vfs-open-read" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stdin source=vfs-open-read" "$LOG_FILE"
    grep -q "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdout consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000001f bytes-read=0x000000000000001f writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-stdout-to-stdin" "$LOG_FILE"
    grep -q "talos: pipeline-lifecycle-status record=phase12-local-pipeline-distinct-process-lifecycle-status-record-v1 pipeline=0x0000000000000001 producer-pid=0x0000000000100001 producer-path=/bin/stdout" "$LOG_FILE"
    grep -q "consumer-pid=0x0000000000100002 consumer-path=/bin/stdin" "$LOG_FILE"
    grep -q "Talos userspace stdin fixture read: Talos userspace stdout fixture" "$LOG_FILE"
    grep -q "talos> waitpid 0x100001" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos> waitpid 0x100002" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos: last-process pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos: pipestatus participants=0x0000000000000002 default-status=0x0000000000000000 pipefail-status=0x0000000000000000 semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status" "$LOG_FILE"
    grep -q "path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table" "$LOG_FILE"
    grep -q "path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table" "$LOG_FILE"
    grep -q "talos> ps" "$LOG_FILE"
    grep -q "talos> stdout | /bin/stdin" "$LOG_FILE"
    grep -q "talos> /bin/stdout | stdin" "$LOG_FILE"
    grep -q "talos> missing | stdin" "$LOG_FILE"
    grep -q "talos> stdout | missing" "$LOG_FILE"
    grep -q "talos> stdout 1>&2 | stdin" "$LOG_FILE"
    grep -q "talos> stdout | stdin 1>/tmp/stdout.txt" "$LOG_FILE"
    grep -q "talos> stdout | stdin | stdin" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=10 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=11 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=12 status=unknown-command responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=13 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=14 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=15 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=16 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: final participants=17 expected=17 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_BARE_NAME_COMMAND_ARGV_SMOKE" -eq 1 ]; then
    grep -q "talos> status42 alpha beta" "$LOG_FILE"
    grep -q "talos: exec path=/bin/status42 source=vfs-open-read" "$LOG_FILE"
    grep -Eq "talos: exec-source bytes=0x[0-9a-f]+ digest=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-loader fixture=phase8-program-loader-elf64-aarch64-v1 entry=0x[0-9a-f]+ segments=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-launch launch-boundary=phase8-initial-process-launch-plan-v1 stack-boundary=phase8-initial-user-stack-plan-v1 address-space=0x[0-9a-f]+ materialization=0x[0-9a-f]+ initial-sp=0x[0-9a-f]+" "$LOG_FILE"
    grep -Eq "talos: exec-descriptors owner=0x[0-9a-f]+ inherited-count=0x0000000000000003 fd0=stdio-input fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -Eq "talos: exec-startup-abi state=literal-argv-absolute-empty-envp argc=0x0000000000000003 argv0=/bin/status42 argv1=alpha argv2=beta argv0-ptr=0x[0-9a-f]+ argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x[0-9a-f]+ copied-startup-bytes=0x0000000000000049 source=initial-user-stack-record" "$LOG_FILE"
    grep -Eq "talos: exec-lifecycle pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true" "$LOG_FILE"
    grep -q "talos: exec-status boundary=lower-aarch64-svc-status-equivalent marker=0x0000000000007a10 status=0x000000000000002a complete=true source=lifecycle-record" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=3 status=handled responses=10" "$LOG_FILE"
    grep -Eq "talos: waitpid pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -Eq "talos: last-process pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true wait-consumed=true job-state=foreground source=bounded-process-table" "$LOG_FILE"
    grep -q "talos> ps" "$LOG_FILE"
    grep -q "talos> status42" "$LOG_FILE"
    grep -Eq "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/status42 argv0-ptr=0x[0-9a-f]+ argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x[0-9a-f]+ copied-startup-bytes=0x000000000000002e source=initial-user-stack-record" "$LOG_FILE"
    grep -q "talos> stdout | stdin" "$LOG_FILE"
    grep -q "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdout consumer-fd=0x0000000000000000 consumer-path=/bin/stdin" "$LOG_FILE"
    grep -q "talos> status42 alpha beta gamma delta" "$LOG_FILE"
    grep -q "talos> status42 *" "$LOG_FILE"
    grep -q "talos> missing alpha" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=10 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=11 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=12 status=unknown-command responses=1" "$LOG_FILE"
    grep -q "$LABEL: final participants=13 expected=13 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
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
    grep -Eq "talos: vfs-exec-lifecycle-status record=phase12-local-vfs-exec-lifecycle-status-record-v2 pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=kernel-owned-vfs-exec-lifecycle-status-record" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=3 status=handled responses=10" "$LOG_FILE"
    grep -q "talos> laststatus" "$LOG_FILE"
    grep -Eq "talos: last-process pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "$LABEL: line command=4 hex=6c 61 73 74 73 74 61 74 75 73" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=4 status=handled responses=1" "$LOG_FILE"
    grep -q "talos> exec /bin/init" "$LOG_FILE"
    grep -q "talos: exec path=/bin/init source=vfs-open-read" "$LOG_FILE"
    grep -Eq "talos: exec-startup-abi state=minimal-argc1-argv0-init-empty-envp argc=0x0000000000000001 argv0=/bin/init argv0-ptr=0x[0-9a-f]+ argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x[0-9a-f]+ copied-startup-bytes=0x000000000000002a source=initial-user-stack-record" "$LOG_FILE"
    grep -Eq "talos: exec-lifecycle pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/init state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true" "$LOG_FILE"
    grep -Eq "talos: init-lifecycle-status record=phase12-local-process-lifecycle-status-record-v1 pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/init state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=kernel-owned-lifecycle-status-record" "$LOG_FILE"
    grep -Eq "talos: vfs-exec-lifecycle-status record=phase12-local-vfs-exec-lifecycle-status-record-v2 pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/init state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=kernel-owned-vfs-exec-lifecycle-status-record" "$LOG_FILE"
    grep -q "$LABEL: line command=5 hex=65 78 65 63 20 2f 62 69 6e 2f 69 6e 69 74" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=5 status=handled responses=11" "$LOG_FILE"
    grep -Eq "talos: last-process pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/init state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "$LABEL: line command=6 hex=6c 61 73 74 73 74 61 74 75 73" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=6 status=handled responses=1" "$LOG_FILE"
    grep -q "talos> exec /bin/zero" "$LOG_FILE"
    grep -q "talos: exec path=/bin/zero source=vfs-open-read" "$LOG_FILE"
    grep -Eq "talos: exec-lifecycle pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/zero state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true" "$LOG_FILE"
    grep -Eq "talos: vfs-exec-lifecycle-status record=phase12-local-vfs-exec-lifecycle-status-record-v2 pid=0x[0-9a-f]+ parent=shell owner=0x[0-9a-f]+ path=/bin/zero state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=kernel-owned-vfs-exec-lifecycle-status-record" "$LOG_FILE"
    grep -q "$LABEL: line command=7 hex=65 78 65 63 20 2f 62 69 6e 2f 7a 65 72 6f" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=7 status=handled responses=10" "$LOG_FILE"
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
    grep -q "^stdout" "$LOG_FILE"
    grep -q "^stdin" "$LOG_FILE"
    grep -q "^stderr" "$LOG_FILE"
    grep -q "$LABEL: line command=8 hex=6c 73" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=8 status=handled responses=6" "$LOG_FILE"
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
    grep -q "^stdout" "$LOG_FILE"
    grep -q "^stdin" "$LOG_FILE"
    grep -q "^stderr" "$LOG_FILE"
    grep -q "$LABEL: line command=3 hex=6c 73 20 2f 62 69 6e" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=3 status=handled responses=6" "$LOG_FILE"
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
    grep -q "^stdout" "$LOG_FILE"
    grep -q "^stdin" "$LOG_FILE"
    grep -q "^stderr" "$LOG_FILE"
    grep -q "^pingdiag" "$LOG_FILE"
    grep -q "^sockdiag" "$LOG_FILE"
    grep -q "$LABEL: line command=4 hex=6c 73 20 2f 62 69 6e" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=4 status=handled responses=8" "$LOG_FILE"
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
elif [ "$SHELL_COMBINED_STDIN_STDOUT_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> exec stdin </etc/banner.txt >/tmp/stdin-report.txt" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=regular-file fd1=regular-file fd2=stdio-output" "$LOG_FILE"
    grep -q "talos: exec-redirection op=source source-fd=0x0000000000000000 source-path=/etc/banner.txt source-stream=regular-file source-route=initramfs:/etc/banner.txt child-only=true shell-restored=true source=shell-redirection-stdin-etc-banner" "$LOG_FILE"
    grep -q "talos: exec-redirection op=sink source-fd=0x0000000000000001 target-path=/tmp/stdin-report.txt target-stream=regular-file target-route=volatile-vfs:/tmp/stdin-report.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout" "$LOG_FILE"
    grep -q "talos: exec-stdin fd=0x0000000000000000 bytes=0x0000000000000018 return=0x0000000000000018 read-source=initramfs:/etc/banner.txt stdout-fd=0x0000000000000001 stdout-bytes=0x000000000000003d stdout-return=0x000000000000003d source=userspace-talos-read+userspace-talos-write read-result=regular-file-eof-after-read" "$LOG_FILE"
    grep -q "Talos userspace stdin fixture read: Talos initramfs fixture" "$LOG_FILE"
    grep -q "talos: cat path=/tmp/stdin-report.txt bytes=0x000000000000003d source=volatile-vfs-descriptor-read" "$LOG_FILE"
    grep -q "talos> exec stdin >/tmp/stdin-report.txt </etc/banner.txt" "$LOG_FILE"
    grep -q "talos> exec stdin </dev/null >/tmp/stdin-report.txt" "$LOG_FILE"
    grep -q "talos> exec stdin </etc/banner.txt 1>/tmp/stdin-report.txt" "$LOG_FILE"
    grep -q "talos> exec stdin < /etc/banner.txt >/tmp/stdin-report.txt" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=7 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=8 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=9 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=10 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: final participants=11 expected=11 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_DIRECT_PIPELINE_OUTPUT_APPEND_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> /bin/stdout | /bin/stdin >/tmp/pipeline-report.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdout | /bin/stdin >>/tmp/pipeline-report.txt" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stdout source=vfs-open-read" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stdin source=vfs-open-read" "$LOG_FILE"
    grep -q "talos: exec-redirection op=sink source-fd=0x0000000000000001 target-path=/tmp/pipeline-report.txt target-stream=regular-file target-route=volatile-vfs:/tmp/pipeline-report.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout" "$LOG_FILE"
    grep -q "talos: exec-redirection op=append source-fd=0x0000000000000001 target-path=/tmp/pipeline-report.txt target-stream=regular-file target-route=volatile-vfs:/tmp/pipeline-report.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout-append" "$LOG_FILE"
    grep -q "source=shell-pipe-consumer-stdout-redirection" "$LOG_FILE"
    grep -q "source=shell-pipe-consumer-stdout-append-redirection" "$LOG_FILE"
    grep -q "talos: cat path=/tmp/pipeline-report.txt bytes=0x0000000000000088 source=volatile-vfs-descriptor-read" "$LOG_FILE"
    grep -c "Talos userspace stdin fixture read: Talos userspace stdout fixture" "$LOG_FILE" | grep -q "^2$"
    grep -q "talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos: pipestatus participants=0x0000000000000002 default-status=0x0000000000000000 pipefail-status=0x0000000000000000 semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status" "$LOG_FILE"
    grep -q "talos> /bin/stdout" "$LOG_FILE"
    grep -q "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=stdout route=runtime-console0/stdout source=userspace-talos-write" "$LOG_FILE"
    grep -q "talos> /bin/stdout | /bin/stdin >/tmp/stdout.txt" "$LOG_FILE"
    grep -q "talos> stdout | stdin >>/tmp/pipeline-report.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdout | /bin/stderr >>/tmp/pipeline-report.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdout | /bin/stdin 2>>/tmp/x" "$LOG_FILE"
    grep -q "talos> /bin/stdout | /bin/stdin </etc/banner.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdout | /bin/stdin >> /tmp/pipeline-report.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdout | /bin/stdin >>/var/x" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=13 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=14 status=handled responses=23" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=15 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=16 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=17 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=18 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=19 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -c "talos: exec-invalid-path" "$LOG_FILE" | grep -q "^6$"
    grep -q "$LABEL: final participants=20 expected=20 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_DIRECT_PIPELINE_STDERR_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> /bin/stdout | /bin/stderr 2>/tmp/pipeline-stderr.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdout | /bin/stderr 2>>/tmp/pipeline-stderr.txt" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stdout source=vfs-open-read" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stderr source=vfs-open-read" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=pipe-endpoint fd1=stdio-output fd2=regular-file loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -q "talos: exec-redirection op=sink source-fd=0x0000000000000002 target-path=/tmp/pipeline-stderr.txt target-stream=regular-file target-route=volatile-vfs:/tmp/pipeline-stderr.txt child-only=true shell-restored=true source=shell-redirection-stderr-tmp-stderr" "$LOG_FILE"
    grep -q "talos: exec-redirection op=append source-fd=0x0000000000000002 target-path=/tmp/pipeline-stderr.txt target-stream=regular-file target-route=volatile-vfs:/tmp/pipeline-stderr.txt child-only=true shell-restored=true source=shell-redirection-stderr-tmp-stderr-append" "$LOG_FILE"
    grep -q "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=regular-file route=volatile-vfs:/tmp/pipeline-stderr.txt source=userspace-talos-write" "$LOG_FILE"
    grep -q "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdout consumer-fd=0x0000000000000000 consumer-path=/bin/stderr bytes-written=0x000000000000001f bytes-read=0x0000000000000000 writer-closed=true reader-eof=false shell-restored=true source=shell-pipe-consumer-stderr-redirection" "$LOG_FILE"
    grep -q "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdout consumer-fd=0x0000000000000000 consumer-path=/bin/stderr bytes-written=0x000000000000001f bytes-read=0x0000000000000000 writer-closed=true reader-eof=false shell-restored=true source=shell-pipe-consumer-stderr-append-redirection" "$LOG_FILE"
    grep -q "talos: cat path=/tmp/pipeline-stderr.txt bytes=0x000000000000003e source=volatile-vfs-descriptor-read" "$LOG_FILE"
    grep -c "Talos userspace stderr fixture" "$LOG_FILE" | grep -q "^3$"
    grep -q "talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos: last-process pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos: pipestatus participants=0x0000000000000002 default-status=0x0000000000000000 pipefail-status=0x0000000000000000 semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status" "$LOG_FILE"
    grep -q "talos> /bin/stderr" "$LOG_FILE"
    grep -q "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=stderr route=runtime-console0/stderr source=userspace-talos-write" "$LOG_FILE"
    grep -q "talos> /bin/stdout | /bin/stderr >/tmp/pipeline-stderr.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdout | /bin/stderr </etc/banner.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdout | /bin/stderr 2>>/tmp/stderr.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdout | /bin/stderr 2> /tmp/pipeline-stderr.txt" "$LOG_FILE"
    grep -q "talos> /bin/stdout | /bin/stderr 2>>/var/x" "$LOG_FILE"
    grep -q "talos> stdout | stderr 2>>/tmp/stderr.txt" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=4 status=handled responses=23" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=13 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=14 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=15 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=16 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=17 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=18 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -c "talos: exec-invalid-path" "$LOG_FILE" | grep -q "^6$"
    grep -q "$LABEL: final participants=19 expected=19 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_BARE_NAME_PIPELINE_STDERR_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> stdout | stderr 2>/tmp/pipeline-stderr.txt" "$LOG_FILE"
    grep -q "talos> stdout | stderr 2>>/tmp/pipeline-stderr.txt" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stdout source=vfs-open-read" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stderr source=vfs-open-read" "$LOG_FILE"
    grep -q "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stdout" "$LOG_FILE"
    grep -q "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stderr" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=pipe-endpoint fd1=stdio-output fd2=regular-file loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table" "$LOG_FILE"
    grep -q "talos: exec-redirection op=sink source-fd=0x0000000000000002 target-path=/tmp/pipeline-stderr.txt target-stream=regular-file target-route=volatile-vfs:/tmp/pipeline-stderr.txt child-only=true shell-restored=true source=shell-redirection-stderr-tmp-stderr" "$LOG_FILE"
    grep -q "talos: exec-redirection op=append source-fd=0x0000000000000002 target-path=/tmp/pipeline-stderr.txt target-stream=regular-file target-route=volatile-vfs:/tmp/pipeline-stderr.txt child-only=true shell-restored=true source=shell-redirection-stderr-tmp-stderr-append" "$LOG_FILE"
    grep -q "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=regular-file route=volatile-vfs:/tmp/pipeline-stderr.txt source=userspace-talos-write" "$LOG_FILE"
    grep -q "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdout consumer-fd=0x0000000000000000 consumer-path=/bin/stderr bytes-written=0x000000000000001f bytes-read=0x0000000000000000 writer-closed=true reader-eof=false shell-restored=true source=shell-pipe-consumer-stderr-redirection" "$LOG_FILE"
    grep -q "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdout consumer-fd=0x0000000000000000 consumer-path=/bin/stderr bytes-written=0x000000000000001f bytes-read=0x0000000000000000 writer-closed=true reader-eof=false shell-restored=true source=shell-pipe-consumer-stderr-append-redirection" "$LOG_FILE"
    grep -q "talos: cat path=/tmp/pipeline-stderr.txt bytes=0x000000000000003e source=volatile-vfs-descriptor-read" "$LOG_FILE"
    grep -c "Talos userspace stderr fixture" "$LOG_FILE" | grep -q "^3$"
    grep -q "talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos: last-process pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos: pipestatus participants=0x0000000000000002 default-status=0x0000000000000000 pipefail-status=0x0000000000000000 semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status" "$LOG_FILE"
    grep -q "talos> stderr" "$LOG_FILE"
    grep -q "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=stderr route=runtime-console0/stderr source=userspace-talos-write" "$LOG_FILE"
    grep -q "talos> /bin/stdout | /bin/stderr 2>>/tmp/pipeline-stderr.txt" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=4 status=handled responses=23" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=13 status=handled responses=23" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=14 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=15 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=16 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=17 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=18 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=19 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=20 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=21 status=unknown-command responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=22 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -c "talos: exec-invalid-path" "$LOG_FILE" | grep -q "^8$"
    grep -q "$LABEL: final participants=23 expected=23 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_BARE_NAME_PIPELINE_OUTPUT_APPEND_REGULAR_FILE_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> stdout | stdin >/tmp/pipeline-report.txt" "$LOG_FILE"
    grep -q "talos> stdout | stdin >>/tmp/pipeline-report.txt" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stdout source=vfs-open-read" "$LOG_FILE"
    grep -q "talos: exec path=/bin/stdin source=vfs-open-read" "$LOG_FILE"
    grep -q "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stdout" "$LOG_FILE"
    grep -q "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stdin" "$LOG_FILE"
    grep -q "talos: exec-redirection op=sink source-fd=0x0000000000000001 target-path=/tmp/pipeline-report.txt target-stream=regular-file target-route=volatile-vfs:/tmp/pipeline-report.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout" "$LOG_FILE"
    grep -q "talos: exec-redirection op=append source-fd=0x0000000000000001 target-path=/tmp/pipeline-report.txt target-stream=regular-file target-route=volatile-vfs:/tmp/pipeline-report.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout-append" "$LOG_FILE"
    grep -q "source=shell-pipe-consumer-stdout-redirection" "$LOG_FILE"
    grep -q "source=shell-pipe-consumer-stdout-append-redirection" "$LOG_FILE"
    grep -q "talos: cat path=/tmp/pipeline-report.txt bytes=0x0000000000000088 source=volatile-vfs-descriptor-read" "$LOG_FILE"
    grep -c "Talos userspace stdin fixture read: Talos userspace stdout fixture" "$LOG_FILE" | grep -q "^2$"
    grep -q "talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos: pipestatus participants=0x0000000000000002 default-status=0x0000000000000000 pipefail-status=0x0000000000000000 semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status" "$LOG_FILE"
    grep -q "talos> stdout" "$LOG_FILE"
    grep -q "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=stdout route=runtime-console0/stdout source=userspace-talos-write" "$LOG_FILE"
    grep -q "talos> /bin/stdout | /bin/stdin >>/tmp/pipeline-report.txt" "$LOG_FILE"
    grep -q "talos> stdout | stdin >/tmp/stdout.txt" "$LOG_FILE"
    grep -q "talos> stdout | stderr >/tmp/pipeline-report.txt" "$LOG_FILE"
    grep -q "talos> stdout | stdin 1>/tmp/pipeline-report.txt" "$LOG_FILE"
    grep -q "talos> stdout | stdin > /tmp/pipeline-report.txt" "$LOG_FILE"
    grep -q "talos> stdout | bin/stdin >/tmp/pipeline-report.txt" "$LOG_FILE"
    grep -q "talos> stdout | stdin 2>>/tmp/x" "$LOG_FILE"
    grep -q "talos> stdout | stdin </etc/banner.txt" "$LOG_FILE"
    grep -q "talos> stdout | stdin >> /tmp/pipeline-report.txt" "$LOG_FILE"
    grep -q "talos> stdout | stdin >>/var/x" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=14 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=15 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=16 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=17 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=18 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=19 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=20 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=21 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=22 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -c "talos: exec-invalid-path" "$LOG_FILE" | grep -q "^9$"
    grep -q "$LABEL: final participants=23 expected=23 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_PIPELINE_CONSUMER_OUTPUT_REDIRECTION_SMOKE" -eq 1 ]; then
    grep -q "talos> exec stdout | exec stdin >/tmp/pipe-consumer.txt" "$LOG_FILE"
    grep -q "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdout consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000001f bytes-read=0x000000000000001f writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-consumer-stdout-redirection" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=pipe-endpoint fd1=regular-file fd2=stdio-output" "$LOG_FILE"
    grep -q "talos: exec-redirection op=sink source-fd=0x0000000000000001 target-path=/tmp/pipe-consumer.txt target-stream=regular-file target-route=volatile-vfs:/tmp/pipe-consumer.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout" "$LOG_FILE"
    grep -q "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=pipe-writer route=pipe:stdout-to-stdin source=userspace-talos-write" "$LOG_FILE"
    grep -q "talos: exec-stdin fd=0x0000000000000000 bytes=0x000000000000001f return=0x000000000000001f read-source=pipe:stdout-to-stdin stdout-fd=0x0000000000000001 stdout-bytes=0x0000000000000044 stdout-return=0x0000000000000044 source=userspace-talos-read+userspace-talos-write read-result=pipe-eof-after-writer-close" "$LOG_FILE"
    grep -q "talos: cat path=/tmp/pipe-consumer.txt bytes=0x0000000000000044 source=volatile-vfs-descriptor-read" "$LOG_FILE"
    grep -q "talos> exec stdout | exec stdin" "$LOG_FILE"
    grep -q "source=shell-pipe-stdout-to-stdin" "$LOG_FILE"
    grep -q "talos> exec stdout | exec stdin >>/tmp/pipe-consumer.txt" "$LOG_FILE"
    grep -q "talos> exec stderr | exec stdin >/tmp/pipe-consumer.txt" "$LOG_FILE"
    grep -q "talos> exec stdout >/tmp/src.txt | exec stdin >/tmp/out.txt" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=8 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=9 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=10 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: final participants=11 expected=11 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_PIPELINE_PRODUCER_FILE_REDIRECTION_AWAY_SMOKE" -eq 1 ]; then
    grep -q "talos> exec stdout >/tmp/pipe-source.txt | exec stdin" "$LOG_FILE"
    grep -q "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdout consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x0000000000000000 bytes-read=0x0000000000000000 writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-producer-file-redirection-away" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=regular-file fd2=stdio-output" "$LOG_FILE"
    grep -q "talos: exec-redirection op=sink source-fd=0x0000000000000001 target-path=/tmp/pipe-source.txt target-stream=regular-file target-route=volatile-vfs:/tmp/pipe-source.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout" "$LOG_FILE"
    grep -q "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=regular-file route=volatile-vfs:/tmp/pipe-source.txt source=userspace-talos-write" "$LOG_FILE"
    grep -q "talos: exec-stdin fd=0x0000000000000000 bytes=0x0000000000000000 return=0x0000000000000000 read-source=pipe:stdout-to-stdin stdout-fd=0x0000000000000001 stdout-bytes=0x000000000000003c stdout-return=0x000000000000003c source=userspace-talos-read+userspace-talos-write read-result=pipe-eof/no-data" "$LOG_FILE"
    grep -q "talos: cat path=/tmp/pipe-source.txt bytes=0x000000000000001f source=volatile-vfs-descriptor-read" "$LOG_FILE"
    grep -q "talos> exec stdout | exec stdin" "$LOG_FILE"
    grep -q "source=shell-pipe-stdout-to-stdin" "$LOG_FILE"
    grep -q "talos> exec stdout >>/tmp/pipe-source.txt | exec stdin" "$LOG_FILE"
    grep -q "talos> exec stderr >/tmp/pipe-source.txt | exec stdin" "$LOG_FILE"
    grep -q "talos> exec stdout >/tmp/src.txt | exec stdin >/tmp/out.txt" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=8 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=9 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=10 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: final participants=11 expected=11 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_PIPELINE_STATUS_SMOKE" -eq 1 ]; then
    grep -q "talos> pipestatus" "$LOG_FILE"
    grep -q "talos: pipestatus none source=bounded-process-table-pipeline-status" "$LOG_FILE"
    grep -q "talos> exec stdout | exec stdin" "$LOG_FILE"
    grep -q "talos: pipestatus participants=0x0000000000000002 default-status=0x0000000000000000 pipefail-status=0x0000000000000000 semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status" "$LOG_FILE"
    grep -q "talos> exec status42 | exec stdin" "$LOG_FILE"
    grep -q "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/status42 consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x0000000000000000 bytes-read=0x0000000000000000 writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-status42-to-stdin" "$LOG_FILE"
    grep -q "talos: pipestatus participants=0x0000000000000002 default-status=0x0000000000000000 pipefail-status=0x000000000000002a semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status" "$LOG_FILE"
    grep -q "talos: pipestatus-participant slot=0 pid=0x0000000000100001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=bounded-process-table-pipeline-status" "$LOG_FILE"
    grep -q "talos: last-process pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos> exec stdout | exec stdin | exec stdin" "$LOG_FILE"
    grep -q "talos: pipestatus participants=0x0000000000000003 default-status=0x0000000000000000 pipefail-status=0x0000000000000000 semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status" "$LOG_FILE"
    grep -q "$LABEL: final participants=12 expected=12 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_MULTISTAGE_PIPELINE_SMOKE" -eq 1 ]; then
    grep -q "talos> exec stdout | exec stdin | exec stdin" "$LOG_FILE"
    grep -q "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdout consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000001f bytes-read=0x000000000000001f writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-multistage-first-stdout-to-stdin" "$LOG_FILE"
    grep -q "talos: pipeline id=0x0000000000000002 producer-fd=0x0000000000000001 producer-path=/bin/stdin consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x0000000000000044 bytes-read=0x0000000000000044 writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-multistage-middle-to-stdin" "$LOG_FILE"
    grep -q "middle-pid=0x0000000000100002 middle-path=/bin/stdin" "$LOG_FILE"
    grep -q "consumer-pid=0x0000000000100003 consumer-path=/bin/stdin" "$LOG_FILE"
    grep -q "talos: exec-stdin fd=0x0000000000000000 bytes=0x000000000000001f return=0x000000000000001f read-source=pipe:stdout-to-stdin stdout-fd=0x0000000000000001 stdout-bytes=0x0000000000000044" "$LOG_FILE"
    grep -q "talos: exec-stdin fd=0x0000000000000000 bytes=0x0000000000000044 return=0x0000000000000044 read-source=pipe:middle-to-stdin stdout-fd=0x0000000000000001 stdout-bytes=0x0000000000000069" "$LOG_FILE"
    grep -q "Talos userspace stdin fixture read: Talos userspace stdin fixture read: Talos userspace stdout fixture" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100003 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record" "$LOG_FILE"
    grep -q "slot=0 capacity=3 pid=0x0000000000100001" "$LOG_FILE"
    grep -q "slot=1 capacity=3 pid=0x0000000000100002" "$LOG_FILE"
    grep -q "slot=2 capacity=3 pid=0x0000000000100003" "$LOG_FILE"
    grep -q "talos> ps" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=3 status=handled responses=33" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=7 status=handled responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=8 status=handled responses=1" "$LOG_FILE"
    grep -q "$LABEL: final participants=10 expected=10 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_BACKGROUND_VFS_EXEC_LIFECYCLE_SMOKE" -eq 1 ]; then
    grep -q "talos> exec /bin/status42 &" "$LOG_FILE"
    grep -q "talos: exec path=/bin/status42 source=vfs-open-read mode=background" "$LOG_FILE"
    grep -q "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=stdio-output fd2=stdio-output" "$LOG_FILE"
    grep -q "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/status42" "$LOG_FILE"
    grep -q "talos: background-job id=0x0000000000000001 pid=0x0000000000100001 command=/bin/status42 state=running reaped=false status=pending shell-responsive=true source=background-vfs-exec-accounting" "$LOG_FILE"
    grep -q "talos: background-signal lower-aarch64-svc-launch-boundary-equivalent" "$LOG_FILE"
    grep -q "talos> cat /etc/banner.txt" "$LOG_FILE"
    grep -q "talos: background-job id=0x0000000000000001 pid=0x0000000000100001 command=/bin/status42 state=completed status=0x000000000000002a observed-status=0x000000000000002a reaped=true shell-responsive=observed source=background-vfs-exec-accounting" "$LOG_FILE"
    grep -q "^Talos initramfs fixture" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=4 status=handled responses=2" "$LOG_FILE"
    grep -q "talos: waitpid no-child source=lifecycle-record" "$LOG_FILE"
    grep -q "talos: last-process none" "$LOG_FILE"
    grep -q "talos> exec /bin/zero" "$LOG_FILE"
    grep -q "talos: exec path=/bin/zero source=vfs-open-read" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/zero state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/zero state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos> exec stdout | exec stdin" "$LOG_FILE"
    grep -q "source=shell-pipe-stdout-to-stdin" "$LOG_FILE"
    grep -q "talos> exec /bin/status42&" "$LOG_FILE"
    grep -q "talos> exec stdout &" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=12 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=13 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: final participants=14 expected=14 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_JOBS_ACCOUNTING_LIST_SMOKE" -eq 1 ]; then
    grep -q "talos> jobs" "$LOG_FILE"
    grep -q "talos: jobs none source=background-vfs-exec-accounting" "$LOG_FILE"
    grep -q "talos> exec /bin/status42 &" "$LOG_FILE"
    grep -q "talos: exec path=/bin/status42 source=vfs-open-read mode=background" "$LOG_FILE"
    grep -q "talos: background-job id=0x0000000000000001 pid=0x0000000000100001 command=/bin/status42 state=running reaped=false status=pending shell-responsive=true source=background-vfs-exec-accounting" "$LOG_FILE"
    grep -q "talos: jobs id=0x0000000000000001 pid=0x0000000000100001 command=/bin/status42 state=running status=pending reaped=false source=background-vfs-exec-accounting" "$LOG_FILE"
    grep -q "talos: jobs id=0x0000000000000001 pid=0x0000000000100001 command=/bin/status42 state=completed status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=background-vfs-exec-accounting" "$LOG_FILE"
    grep -q "talos: waitpid no-child source=lifecycle-record" "$LOG_FILE"
    grep -q "talos: last-process none" "$LOG_FILE"
    grep -q "talos> exec /bin/zero" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/zero state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/zero state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos> exec stdout | exec stdin" "$LOG_FILE"
    grep -q "source=shell-pipe-stdout-to-stdin" "$LOG_FILE"
    grep -q "talos> fg" "$LOG_FILE"
    grep -q "talos> bg" "$LOG_FILE"
    grep -q "talos> kill %1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=14 status=unknown-command responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=15 status=unknown-command responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=16 status=unknown-command responses=1" "$LOG_FILE"
    grep -q "$LABEL: final participants=17 expected=17 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_MULTIPLE_BACKGROUND_JOBS_SMOKE" -eq 1 ]; then
    grep -q "talos> jobs" "$LOG_FILE"
    grep -q "talos: jobs none source=background-vfs-exec-accounting" "$LOG_FILE"
    grep -q "talos> exec /bin/status42 &" "$LOG_FILE"
    grep -q "talos: exec path=/bin/status42 source=vfs-open-read mode=background" "$LOG_FILE"
    grep -q "talos: background-job id=0x0000000000000001 pid=0x0000000000100001 command=/bin/status42 state=running reaped=false status=pending shell-responsive=true source=background-vfs-exec-accounting" "$LOG_FILE"
    grep -q "talos: background-job id=0x0000000000000001 pid=0x0000000000100001 command=/bin/status42 state=completed status=0x000000000000002a observed-status=0x000000000000002a reaped=true shell-responsive=observed source=background-vfs-exec-accounting" "$LOG_FILE"
    grep -q "talos> exec /bin/zero &" "$LOG_FILE"
    grep -q "talos: exec path=/bin/zero source=vfs-open-read mode=background" "$LOG_FILE"
    grep -q "talos: background-job id=0x0000000000000002 pid=0x0000000000100002 command=/bin/zero state=running reaped=false status=pending shell-responsive=true source=background-vfs-exec-accounting" "$LOG_FILE"
    grep -q "talos: jobs id=0x0000000000000001 pid=0x0000000000100001 command=/bin/status42 state=completed status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=background-vfs-exec-accounting" "$LOG_FILE"
    grep -q "talos: jobs id=0x0000000000000002 pid=0x0000000000100002 command=/bin/zero state=running status=pending reaped=false source=background-vfs-exec-accounting" "$LOG_FILE"
    grep -q "talos: jobs id=0x0000000000000002 pid=0x0000000000100002 command=/bin/zero state=completed status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=background-vfs-exec-accounting" "$LOG_FILE"
    grep -q "talos: waitpid no-child source=lifecycle-record" "$LOG_FILE"
    grep -q "talos: last-process none" "$LOG_FILE"
    grep -q "talos> exec /bin/zero" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/zero state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/zero state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos> exec /bin/status42&" "$LOG_FILE"
    grep -q "talos> exec stdout &" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=13 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=14 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: final participants=15 expected=15 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
elif [ "$SHELL_BACKGROUND_JOBS_STALE_ENTRY_POLICY_SMOKE" -eq 1 ]; then
    grep -q "talos> jobs" "$LOG_FILE"
    grep -q "talos: jobs none source=background-vfs-exec-accounting" "$LOG_FILE"
    grep -q "talos> exec /bin/status42 &" "$LOG_FILE"
    grep -q "talos: exec path=/bin/status42 source=vfs-open-read mode=background" "$LOG_FILE"
    grep -q "talos: background-job id=0x0000000000000001 pid=0x0000000000100001 command=/bin/status42 state=running reaped=false status=pending shell-responsive=true source=background-vfs-exec-accounting" "$LOG_FILE"
    grep -q "talos: background-job id=0x0000000000000001 pid=0x0000000000100001 command=/bin/status42 state=completed status=0x000000000000002a observed-status=0x000000000000002a reaped=true shell-responsive=observed source=background-vfs-exec-accounting" "$LOG_FILE"
    grep -q "talos> exec /bin/zero &" "$LOG_FILE"
    grep -q "talos: exec path=/bin/zero source=vfs-open-read mode=background" "$LOG_FILE"
    grep -q "talos: background-job id=0x0000000000000002 pid=0x0000000000100002 command=/bin/zero state=running reaped=false status=pending shell-responsive=true source=background-vfs-exec-accounting" "$LOG_FILE"
    grep -q "talos: jobs id=0x0000000000000001 pid=0x0000000000100001 command=/bin/status42 state=completed status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=background-vfs-exec-accounting" "$LOG_FILE"
    grep -q "talos: jobs id=0x0000000000000002 pid=0x0000000000100002 command=/bin/zero state=running status=pending reaped=false source=background-vfs-exec-accounting" "$LOG_FILE"
    grep -q "talos: jobs id=0x0000000000000002 pid=0x0000000000100002 command=/bin/zero state=completed status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=background-vfs-exec-accounting" "$LOG_FILE"
    grep -q "talos> waitpid" "$LOG_FILE"
    grep -q "talos: waitpid no-child source=lifecycle-record" "$LOG_FILE"
    grep -q "talos: last-process none" "$LOG_FILE"
    grep -q "talos> exec /bin/zero" "$LOG_FILE"
    grep -q "talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/zero state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/zero state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record" "$LOG_FILE"
    grep -q "talos> exec /bin/status42&" "$LOG_FILE"
    grep -q "talos> exec stdout &" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=7 status=handled responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=8 status=handled responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=14 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: dispatch command=15 status=unexpected-argument responses=1" "$LOG_FILE"
    grep -q "$LABEL: final participants=16 expected=16 errors=0 classification=$CLASSIFICATION" "$LOG_FILE"
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
