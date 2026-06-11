#!/bin/sh
set -eu

if [ "$#" -ne 4 ]; then
    echo "usage: $0 <candidate|control> <archive.tar.gz> --capture-nonce NONCE" >&2
    exit 2
fi

KIND="$1"
ARCHIVE="$2"
CAPTURE_NONCE="$4"

if [ "$3" != "--capture-nonce" ] || [ -z "$CAPTURE_NONCE" ]; then
    echo "usage: $0 <candidate|control> <archive.tar.gz> --capture-nonce NONCE" >&2
    exit 2
fi

case "$KIND" in
    candidate|control)
        ;;
    *)
        echo "kind must be candidate or control" >&2
        exit 2
        ;;
esac

MARKER="TALOS: rp1-ethernet-mdio-register-vector-staging-sentinel-$KIND"

./scripts/rpi5-archive-review.sh "$ARCHIVE" >/dev/null

tmp_root="target/tmp"
mkdir -p "$tmp_root"
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-ethernet-mdio-register-vector-staging-sentinel-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-ethernet-mdio-register-vector-staging-sentinel-$KIND: start" \
    "no-mdio-no-ethernet-no-mmio-target-construction" \
    "$MARKER" \
    "capture-nonce=" \
    "$CAPTURE_NONCE" \
    "staging-sentinel-contract-id=phase12-rp1-ethernet-mdio-register-vector-staging-identity-sentinel-v1" \
    "selected-discriminator=rp1-ethernet-mdio-register-vector-staging-identity-only" \
    "purpose=prove-publish-power-tftp-final-selected-tree-durability-before-register-vector-retry" \
    "report-kind=" \
    "$KIND" \
    "target=none controller=none compatible=none" \
    "phy-handle=none phy-node=none phy-address=none" \
    "rp1-mmio-targets-constructed=false" \
    "ncr-write-performed=false man-write-performed=false" \
    "gpio32-phy-reset-action=false ethernet-action=false" \
    "hardware-proof-boundary-classification=staging-identity-durability-only" \
    "allowed-classifications=selected-tree-identity-ready,selected-tree-identity-blocked" \
    "claims-runtime-mdio-transaction=false" \
    "claims-ncr-write-executed=false claims-man-write-executed=false" \
    "claims-mdio-phy-ownership=false claims-phy-absence=false" \
    "claims-gpio32-phy-reset-ownership=false" \
    "claims-ethernet-ready=false claims-interrupt-completion=false" \
    "claims-networking=false claims-sockets=false claims-ssh=false" \
    "claims-phase-12-2=false claims-phase-transition=false" \
    "classification=no-mdio-no-ethernet-rp1-ethernet-mdio-register-vector-staging-sentinel-$KIND"; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing staging sentinel $KIND string: $required" >&2
        exit 1
    fi
done

for forbidden in \
    "TALOS: rp1-ethernet-mdio-register-vector-candidate" \
    "before-corrected-target-guarded-phy1-register-vector-man-transactions" \
    "target=rp1-ethernet-mdio-after-mpe-clause22-phy1-register-vector" \
    "ncr-observed-target=0x" \
    "man-observed-target=0x" \
    "claims-runtime-mdio-transaction=true" \
    "claims-ncr-write-executed=true" \
    "claims-man-write-executed=true" \
    "touched-fields=MAN" \
    "gpio32-event-clear" \
    "gpio32-phy-reset-write-restore"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden staging sentinel $KIND string: $forbidden" >&2
        exit 1
    fi
done

image_size="$(wc -c < "$extract_dir/kernel_2712.img" | tr -d ' ')"
text_offset="$(od -An -tu8 -j8 -N8 "$extract_dir/kernel_2712.img" | tr -d ' ')"
header_image_size="$(od -An -tu8 -j16 -N8 "$extract_dir/kernel_2712.img" | tr -d ' ')"
flags="$(od -An -tu8 -j24 -N8 "$extract_dir/kernel_2712.img" | tr -d ' ')"

printf '%s_archive=%s\n' "$KIND" "$ARCHIVE"
printf '%s_archive_sha256=%s\n' "$KIND" "$(sha256sum "$ARCHIVE" | awk '{print $1}')"
printf 'kernel_2712_sha256=%s\n' "$(sha256sum "$extract_dir/kernel_2712.img" | awk '{print $1}')"
printf 'kernel_2712_size=%s\n' "$image_size"
printf 'header_image_size=%s\n' "$header_image_size"
printf 'text_offset=%s\n' "$text_offset"
printf 'flags=%s\n' "$flags"
printf 'forbidden_staging_sentinel_%s_runtime_strings_absent=true\n' "$KIND"
