#!/bin/sh
set -eu

TALOS_RPI5_TIMER_IRQ_DIAGNOSTIC=1 ./scripts/rpi5-image.sh "$@"
