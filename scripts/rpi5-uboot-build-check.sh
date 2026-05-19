#!/bin/sh
set -eu

U_BOOT_DIR="${U_BOOT_DIR:-/opt/strider/openclaw/current/workspace/tmp/talos-refs/u-boot}"
BUILD_DIR="${BUILD_DIR:-/opt/strider/openclaw/current/workspace/tmp/talos-u-boot-build}"

missing=false
for tool in bison flex dtc; do
    if ! command -v "$tool" >/dev/null 2>&1; then
        echo "missing build dependency: $tool" >&2
        missing=true
    fi
done

if ! command -v aarch64-linux-gnu-gcc >/dev/null 2>&1; then
    echo "missing cross compiler: aarch64-linux-gnu-gcc" >&2
    echo "LLVM=1 may work after Kconfig dependencies are installed." >&2
fi

if [ "$missing" = true ]; then
    exit 1
fi

mkdir -p "$BUILD_DIR"
make -C "$U_BOOT_DIR" O="$BUILD_DIR" ARCH=arm LLVM=1 rpi_arm64_defconfig

printf 'u_boot_dir=%s\n' "$U_BOOT_DIR"
printf 'build_dir=%s\n' "$BUILD_DIR"
printf 'defconfig=pass\n'
