#!/bin/sh
set -eu

env \
    TALOS_BOOT_SCENARIO=rpi5_normal_exception_report \
    ./scripts/rpi5-image.sh
