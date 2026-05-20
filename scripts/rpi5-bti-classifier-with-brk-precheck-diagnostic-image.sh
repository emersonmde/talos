#!/bin/sh
set -eu

TALOS_RPI5_BTI_CLASSIFIER_WITH_BRK_PRECHECK_DIAGNOSTIC=1 \
    ./scripts/rpi5-image.sh
