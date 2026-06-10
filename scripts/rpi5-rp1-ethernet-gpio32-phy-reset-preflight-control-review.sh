#!/bin/sh
set -eu

if [ "$#" -ne 1 ] && [ "$#" -ne 3 ]; then
    echo "usage: $0 <control-archive.tar.gz> [--capture-nonce NONCE]" >&2
    exit 2
fi

ARCHIVE="$1"
MARKER="TALOS: rp1-ethernet-gpio32-phy-reset-preflight-control"
CAPTURE_NONCE=

if [ "$#" -eq 3 ]; then
    if [ "$2" != "--capture-nonce" ] || [ -z "$3" ]; then
        echo "usage: $0 <control-archive.tar.gz> [--capture-nonce NONCE]" >&2
        exit 2
    fi
    CAPTURE_NONCE="$3"
fi

./scripts/rpi5-archive-review.sh "$ARCHIVE" >/dev/null

tmp_root="target/tmp"
mkdir -p "$tmp_root"
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-ethernet-gpio32-phy-reset-preflight-control-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-ethernet-gpio32-phy-reset-preflight-control: start" \
    "no-gpio-no-ethernet-no-mdio-no-mmio-target-construction" \
    "$MARKER" \
    "phase12-rp1-ethernet-gpio32-phy-reset-preflight-report-contract-v1" \
    "phase12-rp1-ethernet-gpio32-phy-reset-source-contract-v1" \
    "phase12-rp1-ethernet-gpio32-phy-reset-source-contract-20260610" \
    "report-kind=" \
    "no-gpio-no-ethernet-control" \
    "hardware-proof-limited-to-gpio32-phy-reset-readonly-preflight-control-output" \
    "accepted-input-frontier=withheld" \
    "controller=none compatible=none" \
    "phy-mode=none phy-handle=none phy-node=none phy-reg=none" \
    "gpio-controller=none gpio-line=none signal=none" \
    "active-low=false logical-assertion-value=none physical-assertion=none" \
    "logical-deassertion-value=none physical-deassertion=none" \
    "reset-duration-ms=none" \
    "linux-hook-relationship=withheld" \
    "phase11-gpio-constraints=withheld" \
    "source-evidence=withheld" \
    "future-write-restore-invariants=withheld" \
    "rejected-runtime-hardware-claims=gpio-ownership" \
    "claims-gpio-ownership=false" \
    "claims-phy-reset-assertion=false" \
    "claims-phy-reset-deassertion=false" \
    "claims-mdio-transactions=false" \
    "claims-phy-ownership=false" \
    "claims-rp1-mmio-writes=false" \
    "claims-packet-io=false" \
    "claims-networking=false" \
    "claims-sockets=false" \
    "claims-ssh=false" \
    "claims-phase-12-2=false" \
    "claims-phase-transition=false" \
    "classification=no-gpio-no-ethernet-rp1-ethernet-gpio32-phy-reset-control"; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing GPIO32 PHY-reset preflight control string: $required" >&2
        exit 1
    fi
done

if [ -n "$CAPTURE_NONCE" ]; then
    if ! grep -Fq -- " capture-nonce=" "$kernel_strings"; then
        echo "kernel image missing GPIO32 PHY-reset preflight control capture nonce label" >&2
        exit 1
    fi
    if ! grep -Fq -- "$CAPTURE_NONCE" "$kernel_strings"; then
        echo "kernel image missing GPIO32 PHY-reset preflight control capture nonce: $CAPTURE_NONCE" >&2
        exit 1
    fi
fi

for forbidden in \
    "TALOS: rp1-ethernet-gpio32-phy-reset-preflight-candidate" \
    "classification=rp1-ethernet-gpio32-phy-reset-readonly-preflight-report-visible" \
    "controller=rp1_eth compatible=raspberrypi,rp1-gem,cdns,macb" \
    "gpio-controller=rp1_gpio gpio-line=32 signal=ETH_RST_N" \
    "reset-duration-ms=5" \
    "claims-phy-reset-assertion=true" \
    "claims-phy-reset-deassertion=true"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden GPIO32 PHY-reset control string: $forbidden" >&2
        exit 1
    fi
done

image_size="$(wc -c < "$extract_dir/kernel_2712.img" | tr -d ' ')"
text_offset="$(od -An -tu8 -j8 -N8 "$extract_dir/kernel_2712.img" | tr -d ' ')"
header_image_size="$(od -An -tu8 -j16 -N8 "$extract_dir/kernel_2712.img" | tr -d ' ')"
flags="$(od -An -tu8 -j24 -N8 "$extract_dir/kernel_2712.img" | tr -d ' ')"

printf 'control_archive=%s\n' "$ARCHIVE"
printf 'control_archive_sha256=%s\n' "$(sha256sum "$ARCHIVE" | awk '{print $1}')"
printf 'kernel_2712_sha256=%s\n' "$(sha256sum "$extract_dir/kernel_2712.img" | awk '{print $1}')"
printf 'kernel_2712_size=%s\n' "$image_size"
printf 'header_image_size=%s\n' "$header_image_size"
printf 'text_offset=%s\n' "$text_offset"
printf 'flags=%s\n' "$flags"
printf 'control_marker=%s\n' "$MARKER"
if [ -n "$CAPTURE_NONCE" ]; then
    printf 'capture_nonce=%s\n' "$CAPTURE_NONCE"
fi
printf 'forbidden_gpio32_phy_reset_control_runtime_strings_absent=true\n'
