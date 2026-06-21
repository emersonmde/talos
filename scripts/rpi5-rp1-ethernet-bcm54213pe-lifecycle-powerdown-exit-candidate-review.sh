#!/bin/sh
set -eu

if [ "$#" -ne 1 ] && [ "$#" -ne 3 ]; then
    echo "usage: $0 <candidate-archive.tar.gz> [--capture-nonce NONCE]" >&2
    exit 2
fi

ARCHIVE="$1"
CAPTURE_NONCE=

if [ "$#" -eq 3 ]; then
    if [ "$2" != "--capture-nonce" ] || [ -z "$3" ]; then
        echo "usage: $0 <candidate-archive.tar.gz> [--capture-nonce NONCE]" >&2
        exit 2
    fi
    CAPTURE_NONCE="$3"
fi

./scripts/rpi5-archive-review.sh "$ARCHIVE" >/dev/null

tmp_root="target/tmp"
mkdir -p "$tmp_root"
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-ethernet-bcm54213pe-lifecycle-powerdown-exit-candidate-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-ethernet-bcm54213pe-lifecycle-powerdown-exit-candidate: start" \
    "before-conditional-bmcr-pdown-clear-then-status-sampling" \
    "TALOS: rp1-ethernet-bcm54213pe-lifecycle-powerdown-exit-candidate" \
    "bcm54213pe-lifecycle-ownership-contract-id=phase12-rp1-ethernet-bcm54213pe-lifecycle-ownership-source-contract-v1" \
    "task-id=phase12-rp1-ethernet-bcm54213pe-lifecycle-ownership-pi5-proof-20260621" \
    "source-core-task-id=phase12-rp1-ethernet-bcm54213pe-lifecycle-ownership-source-core-20260621" \
    "source-core-commit=b4a6d3fe12a05771eb8d831ab4e9aa35042830e9" \
    "selected-discriminator=bcm54213pe-phy1-bmcr-powerdown-exit-gate" \
    "target=phy1-bmcr-powerdown-exit-gate" \
    "bmcr-read-frame=0x60820000 bmcr-write-frame-prefix=0x50820000" \
    "bmcr-powerdown-bit=0x0800 bmcr-clear-pdown-mask=0xf7ff" \
    "accepted-context-pre-bmcr=0x1200 accepted-context-expected-clear-value=0x1200" \
    "accepted-context-write-frame=0x50821200" \
    "selected-registers=pre-BMCR,conditional-clear-BMCR_PDOWN,post-BMCR,post-BMSR-first,post-BMSR-second,post-ANAR,post-ANLPAR,post-MII_CTRL1000,post-MII_STAT1000,post-passive-MACB_NSR_LINK" \
    "allowed-hardware-classifications=bcm54213pe-lifecycle-powerdown-exit-link-ready,bcm54213pe-lifecycle-powerdown-exit-no-change-link-not-ready,bcm54213pe-lifecycle-powerdown-exit-pdown-clear-sampled-link-not-ready" \
    "bcm54213pe-lifecycle-powerdown-exit-readback-mismatch" \
    "no-mdio-no-ethernet-bcm54213pe-lifecycle-ownership-control" \
    "claims-bmcr-pdown-clear-write-executed=" \
    "claims-packet-io=false claims-networking=false claims-sockets=false" \
    "claims-ssh=false claims-phase-12-2=false claims-phase-transition=false" \
    "classification="; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing BCM54213PE lifecycle candidate string: $required" >&2
        exit 1
    fi
done

if [ -n "$CAPTURE_NONCE" ]; then
    if ! grep -Fq -- " capture-nonce=" "$kernel_strings"; then
        echo "kernel image missing BCM54213PE lifecycle candidate capture nonce label" >&2
        exit 1
    fi
    if ! grep -Fq -- "$CAPTURE_NONCE" "$kernel_strings"; then
        echo "kernel image missing BCM54213PE lifecycle candidate capture nonce: $CAPTURE_NONCE" >&2
        exit 1
    fi
fi

for forbidden in \
    "TALOS: rp1-ethernet-bcm54213pe-lifecycle-powerdown-exit-control" \
    "claims-bmcr-pdown-assertion=true" \
    "claims-apd-shadow-write=true" \
    "claims-eee-mmd-write=true" \
    "claims-iddq-top-misc-write=true" \
    "claims-soft-reset=true" \
    "claims-interrupt-ownership=true" \
    "claims-config-init-replay=true" \
    "claims-gpio32-reset-action=true" \
    "claims-mac-phylink-configuration=true" \
    "claims-packet-io=true" \
    "claims-networking=true" \
    "claims-ssh=true" \
    "claims-phase-12-2=true" \
    "claims-phase-transition=true"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden BCM54213PE lifecycle candidate string: $forbidden" >&2
        exit 1
    fi
done

image_size="$(wc -c < "$extract_dir/kernel_2712.img" | tr -d ' ')"
text_offset="$(od -An -tu8 -j8 -N8 "$extract_dir/kernel_2712.img" | tr -d ' ')"
header_image_size="$(od -An -tu8 -j16 -N8 "$extract_dir/kernel_2712.img" | tr -d ' ')"
flags="$(od -An -tu8 -j24 -N8 "$extract_dir/kernel_2712.img" | tr -d ' ')"

printf 'candidate_archive=%s\n' "$ARCHIVE"
printf 'candidate_archive_sha256=%s\n' "$(sha256sum "$ARCHIVE" | awk '{print $1}')"
printf 'kernel_2712_sha256=%s\n' "$(sha256sum "$extract_dir/kernel_2712.img" | awk '{print $1}')"
printf 'kernel_2712_size=%s\n' "$image_size"
printf 'header_image_size=%s\n' "$header_image_size"
printf 'text_offset=%s\n' "$text_offset"
printf 'flags=%s\n' "$flags"
printf 'forbidden_bcm54213pe_lifecycle_candidate_runtime_strings_absent=true\n'
