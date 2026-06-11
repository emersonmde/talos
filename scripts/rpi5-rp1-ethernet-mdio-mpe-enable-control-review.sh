#!/bin/sh
set -eu

if [ "$#" -ne 1 ] && [ "$#" -ne 3 ]; then
    echo "usage: $0 <control-archive.tar.gz> [--capture-nonce NONCE]" >&2
    exit 2
fi

ARCHIVE="$1"
MARKER="TALOS: rp1-ethernet-mdio-mpe-enable-control"
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
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-ethernet-mdio-mpe-enable-control-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-ethernet-mdio-mpe-enable-control: start" \
    "no-ncr-mpe-no-mdio-no-ethernet-target-construction" \
    "$MARKER" \
    "phase12-rp1-ethernet-mdio-mpe-enable-guard-report-contract-v1" \
    "phase12-rp1-ethernet-mdio-mpe-enable-source-contract-v1" \
    "phase12-rp1-ethernet-mdio-mpe-enable-source-contract-20260611" \
    "hardware-proof-boundary-classification=hardware-proof-limited-to-ncr-mpe-enable-write-restore-control-output" \
    "report-kind=" \
    "no-mdio-no-ethernet-control" \
    "target=none controller=none compatible=none" \
    "register=none access=none" \
    "observed-window-macb-mid-context-cpu-physical-target=not-constructed" \
    "observed-window-macb-mid-context-raw=withheld" \
    "ncr-observed-target=not-constructed" \
    "ncr-offset=withheld ncr-mpe-bit=withheld ncr-mpe-mask=withheld" \
    "write-rule=withheld restore-invariant=withheld" \
    "pre-raw=withheld write-value=withheld post-raw=withheld restore-raw=withheld" \
    "pre-mpe-set=false post-mpe-set=false restore-eq-pre=true" \
    "ncr-mpe-write-performed=false ncr-restore-write-performed=false" \
    "man-writes-performed=false phy-id-reads-performed=false" \
    "touched-fields=none" \
    "claims-ncr-mpe-write-readback-restore-boundary=" \
    "claims-man-writes=false claims-phy-id-reads=false" \
    "claims-mdio-phy-ownership=false claims-gpio32-phy-reset-ownership=false" \
    "claims-ethernet-ready=false claims-broad-mmio-ready=false" \
    "claims-networking=false claims-sockets=false" \
    "classification=no-mdio-no-ethernet-rp1-ethernet-mdio-mpe-enable-control"; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing MDIO MPE enable control string: $required" >&2
        exit 1
    fi
done

if [ -n "$CAPTURE_NONCE" ]; then
    if ! grep -Fq -- " capture-nonce=" "$kernel_strings"; then
        echo "kernel image missing MDIO MPE enable control capture nonce label" >&2
        exit 1
    fi
    if ! grep -Fq -- "$CAPTURE_NONCE" "$kernel_strings"; then
        echo "kernel image missing MDIO MPE enable control capture nonce: $CAPTURE_NONCE" >&2
        exit 1
    fi
fi

for forbidden in \
    "TALOS: rp1-ethernet-mdio-mpe-enable-candidate" \
    "before-ncr-mpe-set-readback-restore" \
    "target=rp1-ethernet-mdio-mpe-enable-set-readback-restore" \
    "ncr-observed-target=0x" \
    "ncr-mpe-write-performed=true" \
    "ncr-restore-write-performed=true" \
    "TALOS: rp1-ethernet-mdio-phy-id-candidate" \
    "physid1-man-frame" \
    "gpio32-event-clear" \
    "gpio32-phy-reset-write-restore"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden MDIO MPE enable control string: $forbidden" >&2
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
printf 'forbidden_mdio_mpe_enable_control_runtime_strings_absent=true\n'
