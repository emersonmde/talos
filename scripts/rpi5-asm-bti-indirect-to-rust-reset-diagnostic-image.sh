#!/bin/sh
set -eu

TALOS_RPI5_ASM_BTI_INDIRECT_TO_RUST_RESET_DIAGNOSTIC=1 \
    ./scripts/rpi5-image.sh
