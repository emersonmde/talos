#!/bin/sh
set -eu

if [ "$#" -ne 1 ] && [ "$#" -ne 3 ]; then
    echo "usage: $0 <control-archive.tar.gz> [--capture-nonce NONCE]" >&2
    exit 2
fi

ARCHIVE="$1"
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
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-ethernet-bcm54213pe-lifecycle-powerdown-exit-control-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-ethernet-bcm54213pe-lifecycle-powerdown-exit-control: start" \
    "no-mdio-no-macb-no-gpio32-no-phy-target-construction" \
    "TALOS: rp1-ethernet-bcm54213pe-lifecycle-powerdown-exit-control" \
    "bcm54213pe-lifecycle-ownership-contract-id=phase12-rp1-ethernet-bcm54213pe-lifecycle-ownership-source-contract-v1" \
    "task-id=phase12-rp1-ethernet-bcm54213pe-lifecycle-ownership-pi5-proof-20260621" \
    "source-core-task-id=phase12-rp1-ethernet-bcm54213pe-lifecycle-ownership-source-core-20260621" \
    "selected-discriminator=bcm54213pe-phy1-bmcr-powerdown-exit-gate" \
    "report-kind=" \
    "no-mdio-no-ethernet-control" \
    "target=none controller=none compatible=none" \
    "bmcr-read-frame=withheld bmcr-write-frame-prefix=withheld" \
    "pre-bmcr-raw=withheld bmcr-clear-value=withheld post-bmcr-raw=withheld" \
    "bmcr-write-performed=false" \
    "mdio-man-transactions-performed=false" \
    "macb-read-performed=false macb-write-performed=false" \
    "claims-runtime-mdio-transaction=" \
    "claims-bmcr-pdown-clear-write-executed=" \
    "classification=no-mdio-no-ethernet-bcm54213pe-lifecycle-ownership-control"; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing BCM54213PE lifecycle control string: $required" >&2
        exit 1
    fi
done

if [ -n "$CAPTURE_NONCE" ]; then
    if ! grep -Fq -- " capture-nonce=" "$kernel_strings"; then
        echo "kernel image missing BCM54213PE lifecycle control capture nonce label" >&2
        exit 1
    fi
    if ! grep -Fq -- "$CAPTURE_NONCE" "$kernel_strings"; then
        echo "kernel image missing BCM54213PE lifecycle control capture nonce: $CAPTURE_NONCE" >&2
        exit 1
    fi
fi

for forbidden in \
    "TALOS: rp1-ethernet-bcm54213pe-lifecycle-powerdown-exit-candidate" \
    "target=phy1-bmcr-powerdown-exit-gate" \
    "phy-model=Broadcom-BCM54213PE" \
    "bmcr-read-frame=0x60820000" \
    "bmcr-write-frame-prefix=0x50820000" \
    "macb-read-performed=true" \
    "claims-runtime-mdio-transaction=true" \
    "claims-bmcr-pdown-clear-write-executed=true" \
    "claims-packet-io=true" \
    "claims-networking=true" \
    "claims-ssh=true" \
    "claims-phase-12-2=true" \
    "claims-phase-transition=true"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden BCM54213PE lifecycle control string: $forbidden" >&2
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
printf 'forbidden_bcm54213pe_lifecycle_control_runtime_strings_absent=true\n'
