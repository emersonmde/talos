#!/bin/sh
set -eu

for name in $(env | sed -n 's/^\(TALOS_RPI5_[A-Za-z0-9_]*\)=.*/\1/p'); do
    unset "$name"
done

env TALOS_RPI5_REALLOC_GROWTH_DIAGNOSTIC=1 ./scripts/rpi5-image.sh
