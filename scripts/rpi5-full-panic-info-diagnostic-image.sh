#!/bin/sh
set -eu

env \
    TALOS_BOOT_SCENARIO=rpi5_full_panic_info \
    ./scripts/rpi5-image.sh
