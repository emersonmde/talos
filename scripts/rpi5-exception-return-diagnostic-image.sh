#!/bin/sh
set -eu

env \
    TALOS_BOOT_SCENARIO=rpi5_exception_return \
    ./scripts/rpi5-image.sh
