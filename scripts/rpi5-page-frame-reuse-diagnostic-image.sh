#!/bin/sh
set -eu

for name in $(env | sed -n 's/^\(TALOS_RPI5_[A-Za-z0-9_]*\)=.*/\1/p'); do
    unset "$name"
done

env TALOS_BOOT_SCENARIO=rpi5_page_frame_reuse ./scripts/rpi5-image.sh
