#!/bin/sh
set -eu

if [ "$#" -lt 1 ] || [ "$#" -gt 3 ]; then
    echo "usage: $0 <cursor> [timeout_seconds] [stable_samples]" >&2
    exit 2
fi

API_BASE="${TALOS_LAB_API:-http://talos-lab-api:8080}"
CURSOR="$1"
TIMEOUT_SECONDS="${2:-90}"
STABLE_SAMPLES="${3:-3}"
DEADLINE=$(($(date +%s) + TIMEOUT_SECONDS))
LAST_FINGERPRINT=""
STABLE_SEEN=0

if [ "$STABLE_SAMPLES" -lt 1 ]; then
    echo "stable_samples must be >= 1" >&2
    exit 2
fi

case "$CURSOR" in
    ''|*[!0-9]*)
        echo "cursor must be a non-empty numeric /tftp/logs cursor_end" >&2
        exit 2
        ;;
esac

annotate_response() {
    jq \
        --argjson stable "$1" \
        --arg reason "$2" \
        --argjson stable_samples "$STABLE_SEEN" \
        --argjson required_samples "$STABLE_SAMPLES" \
        '. + {talos_tftp_stability: {
            stable: $stable,
            reason: $reason,
            stable_samples: $stable_samples,
            required_samples: $required_samples
        }}'
}

while :; do
    response="$(curl -fsS "${API_BASE}/tftp/logs?cursor=${CURSOR}&max_bytes=1048576&limit=2000")"

    event_count="$(printf '%s' "$response" | jq '.tftp.events | length')"
    fingerprint="$(printf '%s' "$response" |
        jq -c '{cursor_end: .tftp.cursor_end, log_size: .tftp.log_size, truncated: .tftp.truncated, events: .tftp.events}')"

    if [ "$fingerprint" = "$LAST_FINGERPRINT" ]; then
        STABLE_SEEN=$((STABLE_SEEN + 1))
    else
        LAST_FINGERPRINT="$fingerprint"
        STABLE_SEEN=1
    fi

    if [ "$STABLE_SEEN" -ge "$STABLE_SAMPLES" ]; then
        printf '%s\n' "$response" | annotate_response true stable
        if [ "$event_count" -gt 0 ]; then
            exit 0
        fi
        exit 1
    fi

    if [ "$(date +%s)" -ge "$DEADLINE" ]; then
        printf '%s\n' "$response" | annotate_response false timeout
        exit 1
    fi

    sleep 2
done
