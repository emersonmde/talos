#!/bin/sh
set -eu

env \
    TALOS_BOOT_SCENARIO=rpi5_nested_panic \
    ./scripts/rpi5-image.sh
