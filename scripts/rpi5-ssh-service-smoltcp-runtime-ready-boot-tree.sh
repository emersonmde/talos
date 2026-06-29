#!/bin/sh
set -eu

if [ "$#" -ne 2 ]; then
    echo "usage: $0 <pi-firmware-boot-source> <output-dir>" >&2
    exit 2
fi

: "${TALOS_CAPTURE_NONCE:=runtime-marker-route-static}"
export TALOS_CAPTURE_NONCE
export TALOS_BOOT_SCENARIO=rpi5_ssh_service_smoltcp_runtime_ready

./scripts/rpi5-boot-tree.sh "$1" "$2"
