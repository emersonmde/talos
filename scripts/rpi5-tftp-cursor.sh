#!/bin/sh
set -eu

API_BASE="${TALOS_LAB_API:-http://talos-lab-api:8080}"

curl -fsS "${API_BASE}/tftp/logs?max_bytes=1048576&limit=1" |
    jq -er '(.tftp.cursor_end // .cursor_end) | select(type == "number")'
