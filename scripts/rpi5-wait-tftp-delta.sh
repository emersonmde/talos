#!/bin/sh
set -eu

if [ "$#" -lt 1 ] || [ "$#" -gt 2 ]; then
    echo "usage: $0 <cursor> [timeout_seconds]" >&2
    exit 2
fi

API_BASE="${TALOS_LAB_API:-http://talos-lab-api:8080}"
CURSOR="$1"
TIMEOUT_SECONDS="${2:-90}"
DEADLINE=$(($(date +%s) + TIMEOUT_SECONDS))

while :; do
    response="$(curl -fsS "${API_BASE}/tftp/logs?cursor=${CURSOR}&max_bytes=1048576&limit=2000")"

    event_count="$(printf '%s' "$response" | jq '.tftp.events | length')"
    if [ "$event_count" -gt 0 ]; then
        printf '%s\n' "$response"
        exit 0
    fi

    if [ "$(date +%s)" -ge "$DEADLINE" ]; then
        printf '%s\n' "$response"
        exit 1
    fi

    sleep 2
done
