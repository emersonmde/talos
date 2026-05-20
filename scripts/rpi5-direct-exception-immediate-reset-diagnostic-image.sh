#!/bin/sh
set -eu

TALOS_RPI5_DIRECT_EXCEPTION_IMMEDIATE_RESET_DIAGNOSTIC=1 \
    ./scripts/rpi5-image.sh
