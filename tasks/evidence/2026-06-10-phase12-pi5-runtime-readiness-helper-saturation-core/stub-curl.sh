#!/bin/sh
set -eu

url=""
for arg in "$@"; do
    case "$arg" in
        http://stub/*)
            url="$arg"
            ;;
    esac
done

state_file="${TALOS_STUB_STATE:?TALOS_STUB_STATE required}"
mode="${TALOS_STUB_MODE:?TALOS_STUB_MODE required}"
count=0
if [ -f "$state_file" ]; then
    count="$(cat "$state_file")"
fi
count=$((count + 1))
printf '%s\n' "$count" >"$state_file"

case "$mode:$url:$count" in
    non_saturated:http://stub/serial/observe:1)
        printf '%s\n' '{"ok":true,"cursor_start":100,"cursor_end":128,"bytes":28,"text":"RP1 FW: load 0\n","truncated":false}'
        ;;
    non_saturated:http://stub/serial/observe:2)
        printf '%s\n' '{"ok":true,"cursor_start":128,"cursor_end":220,"bytes":92,"text":"TALOS: kernel_main\nrpi5-production-timer-preemption: PASS\ntalos> ","truncated":false}'
        ;;
    saturated:http://stub/serial/read:1)
        printf '%s\n' '{"ok":true,"bytes":0,"text":"","truncated":false}'
        ;;
    *)
        printf 'unexpected stub call mode=%s url=%s count=%s\n' "$mode" "$url" "$count" >&2
        exit 42
        ;;
esac
