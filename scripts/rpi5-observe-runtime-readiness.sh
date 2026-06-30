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
REQUIRED_MARKERS="${TALOS_READINESS_REQUIRED_MARKERS:-}"
REQUIRE_KERNEL_MARKER="${TALOS_READINESS_REQUIRE_KERNEL_MARKER:-true}"
CAPTURE_MODE="${TALOS_SERIAL_CAPTURE_MODE:-auto}"
SATURATION_LIMIT="${TALOS_SERIAL_CURSOR_SATURATION_LIMIT:-4194304}"

case "$SERIAL_CURSOR" in
    ''|*[!0-9]*)
        echo "serial_cursor must be a non-empty numeric /serial cursor" >&2
        exit 2
        ;;
esac

case "$CAPTURE_MODE" in
    auto|observe|read)
        ;;
    *)
        echo "TALOS_SERIAL_CAPTURE_MODE must be auto, observe, or read" >&2
        exit 2
        ;;
esac

case "$REQUIRE_KERNEL_MARKER" in
    true|false)
        ;;
    *)
        echo "TALOS_READINESS_REQUIRE_KERNEL_MARKER must be true or false" >&2
        exit 2
        ;;
esac

case "$SATURATION_LIMIT" in
    ''|*[!0-9]*)
        echo "TALOS_SERIAL_CURSOR_SATURATION_LIMIT must be numeric" >&2
        exit 2
        ;;
esac

if [ "$CAPTURE_MODE" = "auto" ] && [ "$SERIAL_CURSOR" -ge "$SATURATION_LIMIT" ]; then
    CAPTURE_MODE=read
elif [ "$CAPTURE_MODE" = "auto" ]; then
    CAPTURE_MODE=observe
fi

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

    if [ "$CAPTURE_MODE" = "read" ]; then
        payload="$(jq -n \
            --argjson timeout_seconds "$remaining_seconds" \
            --argjson settle_ms "$SETTLE_MS" \
            --argjson max_bytes "$MAX_BYTES" \
            '{timeout_seconds: $timeout_seconds, settle_ms: $settle_ms, max_bytes: $max_bytes}')"

        response="$(curl -fsS -X POST -H 'Content-Type: application/json' \
            --data "$payload" \
            "${API_BASE}/serial/read")"
    else
        payload="$(jq -n \
            --argjson cursor "$observe_cursor" \
            --argjson timeout_seconds "$remaining_seconds" \
            --argjson settle_ms "$SETTLE_MS" \
            --argjson max_bytes "$MAX_BYTES" \
            '{cursor: $cursor, timeout_seconds: $timeout_seconds, settle_ms: $settle_ms, max_bytes: $max_bytes}')"

        response="$(curl -fsS -X POST -H 'Content-Type: application/json' \
            --data "$payload" \
            "${API_BASE}/serial/observe")"
    fi

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
        --arg required_markers_list "$REQUIRED_MARKERS" \
        --arg require_kernel_marker "$REQUIRE_KERNEL_MARKER" \
        --argjson cursor_start "$SERIAL_CURSOR" \
        --argjson cursor_end "$observe_cursor" \
        --argjson requested_timeout_seconds "$TIMEOUT_SECONDS" \
        --argjson elapsed_seconds "$elapsed_seconds" \
        --argjson settle_ms "$SETTLE_MS" \
        --argjson max_bytes "$MAX_BYTES" \
        --argjson attempts "$attempts" \
        --arg capture_mode "$CAPTURE_MODE" \
        --argjson saturation_limit "$SATURATION_LIMIT" \
        '(if length == 0 then {} else .[-1] end) as $last
         | (map(.text // "") | add) as $text
         | (if $required_markers_list == "" then [$required_marker]
            else ($required_markers_list | split("|") | map(select(. != "")))
            end) as $configured_required_markers
         | (if ($configured_required_markers | length) == 0 then [$required_marker]
            else $configured_required_markers
            end) as $required_markers
         | (map((.bytes // ((.text // "") | length)) | tonumber) | add) as $response_bytes
         | ($text | contains("TALOS: kernel_main")) as $has_kernel_main
         | (all($required_markers[]; $text | contains(.))) as $has_required_marker
         | ($text | contains("talos>")) as $has_prompt
         | ((if $require_kernel_marker == "true" then $has_kernel_main else true end) and $has_required_marker) as $ready
         | (if $capture_mode == "read" then $response_bytes else ($cursor_end - $cursor_start) end) as $window_bytes
         | $last + {
             cursor_start: $cursor_start,
             cursor_end: $cursor_end,
             bytes: $window_bytes,
             text: $text,
             truncated: (any(.[]; .truncated == true)),
             talos_runtime_readiness: {
                 requested_timeout_seconds: $requested_timeout_seconds,
                 elapsed_seconds: $elapsed_seconds,
                 settle_ms: $settle_ms,
                 max_bytes: $max_bytes,
                 observe_attempts: $attempts,
                 observe_contract: (if $capture_mode == "read" then "deadline-loop-direct-read-after-saturated-cursor" else "deadline-loop-accumulated-from-fresh-cursor" end),
                 capture_mode: $capture_mode,
                 saturation_limit: $saturation_limit,
                 start_cursor_saturated: ($cursor_start >= $saturation_limit),
                 response_bytes: $response_bytes,
                 required_kernel_marker: "TALOS: kernel_main",
                 require_kernel_marker: ($require_kernel_marker == "true"),
                 required_success_marker: $required_marker,
                 required_success_markers: $required_markers,
                 prompt_marker: "talos>",
                 has_kernel_main: $has_kernel_main,
                 has_required_success_marker: $has_required_marker,
                 has_prompt_marker: $has_prompt,
                 valid_known_good_talos_readiness: $ready,
                 classification: (
                     if $ready then "valid-known-good-talos-readiness"
                     elif $capture_mode == "read" and $response_bytes == 0 then "saturated-cursor-capture-blocked"
                     else "known-good-fetch-observed-without-talos-readiness"
                     end)
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
        --arg required_markers_list "$REQUIRED_MARKERS" \
        --arg require_kernel_marker "$REQUIRE_KERNEL_MARKER" \
        --argjson requested_timeout_seconds "$TIMEOUT_SECONDS" \
        --argjson settle_ms "$SETTLE_MS" \
        --argjson max_bytes "$MAX_BYTES" \
        --argjson cursor_start "$SERIAL_CURSOR" \
        --arg capture_mode "$CAPTURE_MODE" \
        --argjson saturation_limit "$SATURATION_LIMIT" \
        '(if $required_markers_list == "" then [$required_marker]
          else ($required_markers_list | split("|") | map(select(. != "")))
          end) as $configured_required_markers
         | (if ($configured_required_markers | length) == 0 then [$required_marker]
            else $configured_required_markers
            end) as $required_markers
         | {talos_runtime_readiness: {
            requested_timeout_seconds: $requested_timeout_seconds,
            elapsed_seconds: 0,
            settle_ms: $settle_ms,
            max_bytes: $max_bytes,
            observe_attempts: 0,
            observe_contract: (if $capture_mode == "read" then "deadline-loop-direct-read-after-saturated-cursor" else "deadline-loop-accumulated-from-fresh-cursor" end),
            capture_mode: $capture_mode,
            saturation_limit: $saturation_limit,
            start_cursor_saturated: ($cursor_start >= $saturation_limit),
            response_bytes: 0,
            required_kernel_marker: "TALOS: kernel_main",
            require_kernel_marker: ($require_kernel_marker == "true"),
            required_success_marker: $required_marker,
            required_success_markers: $required_markers,
            prompt_marker: "talos>",
            has_kernel_main: false,
            has_required_success_marker: false,
            has_prompt_marker: false,
            valid_known_good_talos_readiness: false,
            classification: (if $capture_mode == "read" then "saturated-cursor-capture-blocked" else "known-good-fetch-observed-without-talos-readiness" end)
        }}')"
fi

printf '%s\n' "$annotated"

printf '%s' "$annotated" |
    jq -e '.talos_runtime_readiness.valid_known_good_talos_readiness' >/dev/null
