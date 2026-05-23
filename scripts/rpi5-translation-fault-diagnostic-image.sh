#!/bin/sh
set -eu

if [ "$#" -ne 0 ]; then
    echo "usage: $0" >&2
    exit 2
fi

for name in $(env | sed -n 's/^\(TALOS_RPI5_[A-Za-z0-9_]*\)=.*/\1/p'); do
    unset "$name"
done

env \
    TALOS_RPI5_TRANSLATION_FAULT_DIAGNOSTIC=1 \
    ./scripts/rpi5-image.sh
