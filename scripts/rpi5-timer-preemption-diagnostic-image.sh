#!/bin/sh
set -eu

TALOS_BOOT_SCENARIO=rpi5_timer_preemption ./scripts/rpi5-image.sh "$@"
