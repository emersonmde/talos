#!/bin/sh
set -eu

TALOS_RPI5_ASM_DIRECT_RESET_DIAGNOSTIC=1 \
    ./scripts/rpi5-image.sh
