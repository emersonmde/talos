#!/bin/sh
set -eu

TALOS_RPI5_DYNAMIC_FORMAT_FALLBACK_DIAGNOSTIC=1 \
    ./scripts/rpi5-image.sh
