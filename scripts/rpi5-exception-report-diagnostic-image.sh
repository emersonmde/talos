#!/bin/sh
set -eu

TALOS_RPI5_EXCEPTION_REPORT_DIAGNOSTIC=1 \
    ./scripts/rpi5-image.sh
