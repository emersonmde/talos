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

payload="$(jq -n \
    --argjson cursor "$SERIAL_CURSOR" \
    --argjson timeout_seconds "$TIMEOUT_SECONDS" \
    --argjson settle_ms "$SETTLE_MS" \
    --argjson max_bytes "$MAX_BYTES" \
    '{cursor: $cursor, timeout_seconds: $timeout_seconds, settle_ms: $settle_ms, max_bytes: $max_bytes}')"

response="$(curl -fsS -X POST -H 'Content-Type: application/json' \
    --data "$payload" \
    "${API_BASE}/serial/observe")"

annotated="$(printf '%s' "$response" | jq \
    --arg required_marker "$REQUIRED_MARKER" \
    --argjson timeout_seconds "$TIMEOUT_SECONDS" \
    --argjson settle_ms "$SETTLE_MS" \
    --argjson max_bytes "$MAX_BYTES" \
    '(.text // "") as $text
     | ($text | contains("TALOS: kernel_main")) as $has_kernel_main
     | ($text | contains($required_marker)) as $has_required_marker
     | ($text | contains("talos>")) as $has_prompt
     | ($has_kernel_main and $has_required_marker) as $ready
     | . + {talos_runtime_readiness: {
         timeout_seconds: $timeout_seconds,
         settle_ms: $settle_ms,
         max_bytes: $max_bytes,
         required_kernel_marker: "TALOS: kernel_main",
         required_success_marker: $required_marker,
         prompt_marker: "talos>",
         has_kernel_main: $has_kernel_main,
         has_required_success_marker: $has_required_marker,
         has_prompt_marker: $has_prompt,
         valid_known_good_talos_readiness: $ready,
         classification: (if $ready then "valid-known-good-talos-readiness" else "known-good-fetch-observed-without-talos-readiness" end)
     }}')"

printf '%s\n' "$annotated"

printf '%s' "$annotated" |
    jq -e '.talos_runtime_readiness.valid_known_good_talos_readiness' >/dev/null
