#!/bin/sh
set -eu

if [ "$#" -lt 1 ] || [ "$#" -gt 5 ]; then
    echo "usage: $0 <serial_cursor> [timeout_seconds] [settle_ms] [max_bytes] [marker]" >&2
    exit 2
fi

API_BASE="${TALOS_LAB_API:-http://talos-lab-api:8080}"
SERIAL_CURSOR="$1"
TIMEOUT_SECONDS="${2:-90}"
SETTLE_MS="${3:-1000}"
MAX_BYTES="${4:-65536}"
MARKER="${5:-rpi5-rp1-post-handoff-marker-reset}"
CAPTURE_MODE="${TALOS_SERIAL_CAPTURE_MODE:-auto}"
SATURATION_LIMIT="${TALOS_SERIAL_CURSOR_SATURATION_LIMIT:-4194304}"
MARKER_FAMILY="${TALOS_SERIAL_MARKER_FAMILY:-}"

case "$SERIAL_CURSOR" in
    ''|*[!0-9]*)
        echo "serial_cursor must be a non-empty numeric /serial cursor" >&2
        exit 2
        ;;
esac

start_epoch="$(date +%s)"
attempts=0
annotated=
observe_cursor="$SERIAL_CURSOR"
responses_file="$(mktemp)"
trap 'rm -f "$responses_file"' EXIT

case "$CAPTURE_MODE" in
    auto|observe|read)
        ;;
    *)
        echo "TALOS_SERIAL_CAPTURE_MODE must be auto, observe, or read" >&2
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
        --arg marker "$MARKER" \
        --arg marker_family "$MARKER_FAMILY" \
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
         | (if $marker_family == "" then [$marker]
            else ($marker_family | split("|") | map(select(. != "")))
            end) as $configured_marker_family
         | (if ($configured_marker_family | length) == 0 then [$marker]
            else $configured_marker_family
            end) as $markers
         | (map((.bytes // ((.text // "") | length)) | tonumber) | add) as $response_bytes
         | ($text | contains("TALOS: kernel_main")) as $has_kernel_main
         | ($text | contains($marker)) as $has_marker
         | (($marker | capture("capture-nonce=(?<nonce>[A-Za-z0-9_.:-]+)")? // {}) | .nonce // "") as $marker_nonce
         | (if $marker_nonce == "" then "" else ("capture-nonce=" + $marker_nonce) end) as $nonce_token
         | ($text | contains("NETWORK")) as $has_firmware_network
         | (if $text == "" then 0 else (($text | split("NETWORK") | length) - 1) end) as $firmware_network_occurrences
         | (if $text == "" then 0 else (($text | split($marker) | length) - 1) end) as $marker_occurrences
         | ($markers | map(. as $family_marker
            | ($text | index($family_marker)) as $first_index
            | {
                marker: $family_marker,
                present: ($first_index != null),
                occurrences: (if $text == "" then 0 else (($text | split($family_marker) | length) - 1) end),
                first_index: $first_index
              })) as $marker_family_counts
         | ($marker_family_counts | map(select(.present)) | last // null) as $deepest_present_marker
         | (if $nonce_token == "" then 0 else (($text | split($nonce_token) | length) - 1) end) as $nonce_occurrences
         | ($text | index($marker)) as $marker_index
         | (if $marker_index == null then ""
            else ($text[
                (if ($marker_index - 80) < 0 then 0 else ($marker_index - 80) end):
                ($marker_index + ($marker | length) + 160)
            ])
            end) as $marker_excerpt
         | (if $capture_mode == "read" then $response_bytes else ($cursor_end - $cursor_start) end) as $window_bytes
         | $last + {
             cursor_start: $cursor_start,
             cursor_end: $cursor_end,
             bytes: $window_bytes,
             text: $text,
             truncated: (any(.[]; .truncated == true)),
             talos_serial_window: {
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
                 required_marker: $marker,
                 kernel_marker: "TALOS: kernel_main",
                 firmware_network_marker: "NETWORK",
                 has_kernel_main: $has_kernel_main,
                 has_required_marker: $has_marker,
                 marker_family: {
                     markers: $markers,
                     counts: $marker_family_counts,
                     deepest_present_marker: $deepest_present_marker,
                     all_present: (all($marker_family_counts[]; .present == true)),
                     present_count: ($marker_family_counts | map(select(.present)) | length)
                 },
                 has_firmware_network: $has_firmware_network,
                 firmware_network_occurrences: $firmware_network_occurrences,
                 required_marker_occurrences: $marker_occurrences,
                 marker_nonce: (if $marker_nonce == "" then null else $marker_nonce end),
                 nonce_token: (if $nonce_token == "" then null else $nonce_token end),
                 nonce_token_occurrences: $nonce_occurrences,
                 required_marker_excerpt: $marker_excerpt
             }}' "$responses_file")"

    if printf '%s' "$annotated" |
        jq -e '.talos_serial_window.has_required_marker' >/dev/null; then
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
        --arg marker "$MARKER" \
        --arg marker_family "$MARKER_FAMILY" \
        --argjson cursor_start "$SERIAL_CURSOR" \
        --argjson requested_timeout_seconds "$TIMEOUT_SECONDS" \
        --argjson settle_ms "$SETTLE_MS" \
        --argjson max_bytes "$MAX_BYTES" \
        --arg capture_mode "$CAPTURE_MODE" \
        --argjson saturation_limit "$SATURATION_LIMIT" \
        '(if $marker_family == "" then [$marker]
          else ($marker_family | split("|") | map(select(. != "")))
          end) as $configured_marker_family
         | (if ($configured_marker_family | length) == 0 then [$marker]
            else $configured_marker_family
            end) as $markers
         | {cursor_start: $cursor_start,
          cursor_end: $cursor_start,
          bytes: 0,
          text: "",
          truncated: false,
          talos_serial_window: {
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
              required_marker: $marker,
              kernel_marker: "TALOS: kernel_main",
              firmware_network_marker: "NETWORK",
              has_kernel_main: false,
              has_required_marker: false,
              marker_family: {
                  markers: $markers,
                  counts: ($markers | map({
                      marker: .,
                      present: false,
                      occurrences: 0,
                      first_index: null
                  })),
                  deepest_present_marker: null,
                  all_present: false,
                  present_count: 0
              },
              has_firmware_network: false,
              firmware_network_occurrences: 0,
              required_marker_occurrences: 0,
              marker_nonce: null,
              nonce_token: null,
              nonce_token_occurrences: 0,
              required_marker_excerpt: ""
          }}')"
fi

printf '%s\n' "$annotated"

printf '%s' "$annotated" |
    jq -e '.talos_serial_window.has_required_marker' >/dev/null
