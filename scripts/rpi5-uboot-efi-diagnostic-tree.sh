#!/bin/sh
set -eu

if [ "$#" -ne 3 ] && [ "$#" -ne 4 ]; then
    echo "usage: $0 <pi-firmware-boot-source> <u-boot.bin> <output-boot-tree> [serial-prefix]" >&2
    exit 2
fi

SOURCE_DIR="$1"
U_BOOT_BIN="$2"
OUTPUT_DIR="$3"
SERIAL_PREFIX="da591740"
if [ "$#" -eq 4 ]; then
    SERIAL_PREFIX="$4"
fi

case "$SERIAL_PREFIX" in
    ""|*/*|.*|*..*)
        echo "unsafe serial prefix: $SERIAL_PREFIX" >&2
        exit 1
        ;;
esac

if [ ! -s "$U_BOOT_BIN" ]; then
    echo "U-Boot binary is missing or empty: $U_BOOT_BIN" >&2
    exit 1
fi

if ! command -v mkimage >/dev/null 2>&1; then
    echo "mkimage is required to create boot.scr from boot.txt" >&2
    exit 1
fi

rm -rf "$OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR/EFI/BOOT" "$OUTPUT_DIR/pxelinux.cfg"

for path in bcm2712-rpi-5-b.dtb overlays/overlay_map.dtb overlays/bcm2712d0.dtbo; do
    if [ ! -f "$SOURCE_DIR/$path" ]; then
        echo "firmware source missing required file: $path" >&2
        exit 1
    fi
    mkdir -p "$OUTPUT_DIR/$(dirname "$path")"
    cp "$SOURCE_DIR/$path" "$OUTPUT_DIR/$path"
done

cp "$U_BOOT_BIN" "$OUTPUT_DIR/kernel_2712.img"
cp "$U_BOOT_BIN" "$OUTPUT_DIR/kernel8.img"
cp "$(./scripts/rpi5-efi-diagnostic.sh)" "$OUTPUT_DIR/EFI/BOOT/BOOTAA64.EFI"

cat > "$OUTPUT_DIR/config.txt" <<'CONFIG'
arm_64bit=1
kernel=kernel_2712.img
os_check=0
enable_uart=1
enable_rp1_uart=1
pciex4_reset=0
uart_2ndstage=1
sha256=1
kernel_address=0x80000
talos_loader_diagnostic=uboot-efi
CONFIG

cat > "$OUTPUT_DIR/cmdline.txt" <<'CMDLINE'
console=serial0,115200 talos.boot=uboot-efi
CMDLINE

cat > "$OUTPUT_DIR/boot.txt" <<'BOOT'
echo Talos U-Boot EFI diagnostic handoff
setenv efi_addr_r ${kernel_addr_r}
if load ${devtype} ${devnum}:${distro_bootpart} ${efi_addr_r} EFI/BOOT/BOOTAA64.EFI; then
  bootefi ${efi_addr_r}
fi
if tftpboot ${efi_addr_r} EFI/BOOT/BOOTAA64.EFI; then
  bootefi ${efi_addr_r}
fi
echo Talos U-Boot EFI diagnostic failed to load BOOTAA64.EFI
BOOT

mkimage -A arm64 -T script -C none -n 'Talos U-Boot EFI diagnostic' \
    -d "$OUTPUT_DIR/boot.txt" "$OUTPUT_DIR/boot.scr" >/dev/null

cat > "$OUTPUT_DIR/pxelinux.cfg/default" <<'PXE'
DEFAULT talos-efi
LABEL talos-efi
  MENU LABEL Talos EFI diagnostic
  KERNEL EFI/BOOT/BOOTAA64.EFI
PXE

prefix_dir="$OUTPUT_DIR/$SERIAL_PREFIX"
mkdir -p "$prefix_dir"
for path in config.txt cmdline.txt bcm2712-rpi-5-b.dtb kernel_2712.img kernel8.img boot.txt boot.scr; do
    cp "$OUTPUT_DIR/$path" "$prefix_dir/$path"
done
mkdir -p "$prefix_dir/EFI/BOOT" "$prefix_dir/overlays" "$prefix_dir/pxelinux.cfg"
cp "$OUTPUT_DIR/EFI/BOOT/BOOTAA64.EFI" "$prefix_dir/EFI/BOOT/BOOTAA64.EFI"
cp "$OUTPUT_DIR/pxelinux.cfg/default" "$prefix_dir/pxelinux.cfg/default"
for overlay in "$OUTPUT_DIR"/overlays/*; do
    if [ -f "$overlay" ]; then
        cp "$overlay" "$prefix_dir/overlays/"
    fi
done

find "$OUTPUT_DIR" -type f | sort
