#!/bin/sh
set -eu

if [ "$#" -gt 1 ]; then
    echo "usage: $0 [fresh-label]" >&2
    exit 2
fi

for name in $(env | sed -n 's/^\(TALOS_RPI5_[A-Za-z0-9_]*\)=.*/\1/p'); do
    unset "$name"
done

label="${1:-TALOS: undef entry continue}"

if ! printf '%s' "$label" | LC_ALL=C grep -Eq '^[A-Za-z0-9: ./_-]+$'; then
    echo "fresh label must be printable ASCII without shell metacharacters" >&2
    exit 2
fi

fresh_label="$(printf '%s\r\n' "$label")"

env \
    TALOS_RPI5_UNDEFINED_INSTRUCTION_REPORT_DIAGNOSTIC=1 \
    ./scripts/rpi5-image.sh
