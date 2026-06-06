#!/bin/sh
set -eu

if [ "$#" -lt 1 ] || [ "$#" -gt 4 ]; then
    echo "usage: $0 <serial_cursor> [timeout_seconds] [settle_ms] [max_bytes]" >&2
    exit 2
fi

API_BASE="${TALOS_LAB_API:-http://talos-lab-api:8080}"
SERIAL_CURSOR="$1"
TIMEOUT_SECONDS="${2:-75}"
SETTLE_MS="${3:-1000}"
MAX_BYTES="${4:-65536}"
REQUIRED_MARKER="${TALOS_READINESS_REQUIRED_MARKER:-rpi5-production-timer-preemption: PASS}"

start_epoch="$(date +%s)"
attempts=0
annotated=
observe_cursor="$SERIAL_CURSOR"
responses_file="$(mktemp)"
trap 'rm -f "$responses_file"' EXIT

while :; do
    now_epoch="$(date +%s)"
    elapsed_seconds=$((now_epoch - start_epoch))
    remaining_seconds=$((TIMEOUT_SECONDS - elapsed_seconds))

    if [ "$remaining_seconds" -le 0 ]; then
        break
    fi

    payload="$(jq -n \
        --argjson cursor "$observe_cursor" \
        --argjson timeout_seconds "$remaining_seconds" \
        --argjson settle_ms "$SETTLE_MS" \
        --argjson max_bytes "$MAX_BYTES" \
        '{cursor: $cursor, timeout_seconds: $timeout_seconds, settle_ms: $settle_ms, max_bytes: $max_bytes}')"

    response="$(curl -fsS -X POST -H 'Content-Type: application/json' \
        --data "$payload" \
        "${API_BASE}/serial/observe")"

    attempts=$((attempts + 1))
    printf '%s\n' "$response" >> "$responses_file"
    observe_cursor="$(printf '%s' "$response" | jq -r '.cursor_end // .cursor // empty')"
    if [ -z "$observe_cursor" ]; then
        observe_cursor="$SERIAL_CURSOR"
    fi
    now_epoch="$(date +%s)"
    elapsed_seconds=$((now_epoch - start_epoch))

    annotated="$(jq -s \
        --arg required_marker "$REQUIRED_MARKER" \
        --argjson cursor_start "$SERIAL_CURSOR" \
        --argjson cursor_end "$observe_cursor" \
        --argjson requested_timeout_seconds "$TIMEOUT_SECONDS" \
        --argjson elapsed_seconds "$elapsed_seconds" \
        --argjson settle_ms "$SETTLE_MS" \
        --argjson max_bytes "$MAX_BYTES" \
        --argjson attempts "$attempts" \
        '(if length == 0 then {} else .[-1] end) as $last
         | (map(.text // "") | add) as $text
         | ($text | contains("TALOS: kernel_main")) as $has_kernel_main
         | ($text | contains($required_marker)) as $has_required_marker
         | ($text | contains("talos>")) as $has_prompt
         | ($has_kernel_main and $has_required_marker) as $ready
         | $last + {
             cursor_start: $cursor_start,
             cursor_end: $cursor_end,
             bytes: ($cursor_end - $cursor_start),
             text: $text,
             truncated: (any(.[]; .truncated == true)),
             talos_runtime_readiness: {
                 requested_timeout_seconds: $requested_timeout_seconds,
                 elapsed_seconds: $elapsed_seconds,
                 settle_ms: $settle_ms,
                 max_bytes: $max_bytes,
                 observe_attempts: $attempts,
                 observe_contract: "deadline-loop-accumulated-from-fresh-cursor",
                 required_kernel_marker: "TALOS: kernel_main",
                 required_success_marker: $required_marker,
                 prompt_marker: "talos>",
                 has_kernel_main: $has_kernel_main,
                 has_required_success_marker: $has_required_marker,
                 has_prompt_marker: $has_prompt,
                 valid_known_good_talos_readiness: $ready,
                 classification: (if $ready then "valid-known-good-talos-readiness" else "known-good-fetch-observed-without-talos-readiness" end)
             }}' "$responses_file")"

    if printf '%s' "$annotated" |
        jq -e '.talos_runtime_readiness.valid_known_good_talos_readiness' >/dev/null; then
        break
    fi

    now_epoch="$(date +%s)"
    elapsed_seconds=$((now_epoch - start_epoch))
    if [ "$elapsed_seconds" -ge "$TIMEOUT_SECONDS" ]; then
        break
    fi

    sleep 1
done

if [ -z "$annotated" ]; then
    annotated="$(jq -n \
        --arg required_marker "$REQUIRED_MARKER" \
        --argjson requested_timeout_seconds "$TIMEOUT_SECONDS" \
        --argjson settle_ms "$SETTLE_MS" \
        --argjson max_bytes "$MAX_BYTES" \
        '{talos_runtime_readiness: {
            requested_timeout_seconds: $requested_timeout_seconds,
            elapsed_seconds: 0,
            settle_ms: $settle_ms,
            max_bytes: $max_bytes,
            observe_attempts: 0,
            observe_contract: "deadline-loop-accumulated-from-fresh-cursor",
            required_kernel_marker: "TALOS: kernel_main",
            required_success_marker: $required_marker,
            prompt_marker: "talos>",
            has_kernel_main: false,
            has_required_success_marker: false,
            has_prompt_marker: false,
            valid_known_good_talos_readiness: false,
            classification: "known-good-fetch-observed-without-talos-readiness"
        }}')"
fi

printf '%s\n' "$annotated"

printf '%s' "$annotated" |
    jq -e '.talos_runtime_readiness.valid_known_good_talos_readiness' >/dev/null
