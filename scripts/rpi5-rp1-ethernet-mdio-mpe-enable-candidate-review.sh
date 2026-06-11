#!/bin/sh
set -eu

if [ "$#" -ne 1 ] && [ "$#" -ne 3 ]; then
    echo "usage: $0 <candidate-archive.tar.gz> [--capture-nonce NONCE]" >&2
    exit 2
fi

ARCHIVE="$1"
MARKER="TALOS: rp1-ethernet-mdio-mpe-enable-candidate"
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
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-ethernet-mdio-mpe-enable-candidate-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-ethernet-mdio-mpe-enable-candidate: start" \
    "before-ncr-mpe-set-readback-restore" \
    "$MARKER" \
    "phase12-rp1-ethernet-mdio-mpe-enable-guard-report-contract-v1" \
    "phase12-rp1-ethernet-mdio-mpe-enable-source-contract-v1" \
    "phase12-rp1-ethernet-mdio-mpe-enable-source-contract-20260611" \
    "hardware-proof-boundary-classification=hardware-proof-limited-to-ncr-mpe-enable-write-restore-control-output" \
    "report-kind=" \
    "candidate" \
    "target=rp1-ethernet-mdio-mpe-enable-set-readback-restore" \
    "controller=rp1_eth compatible=raspberrypi,rp1-gem,cdns,macb" \
    "register=MACB/GEM_NCR access=32-bit-little-endian-volatile-load-store" \
    "observed-window-macb-mid-context-cpu-physical-target=" \
    "ncr-observed-target=" \
    "ncr-offset=0x0 ncr-mpe-bit=4 ncr-mpe-mask=0x00000010" \
    "write-rule=pre_raw|0x00000010 restore-invariant=restore_raw==pre_raw" \
    "pre-raw=" \
    "write-value=" \
    "post-raw=" \
    "restore-raw=" \
    "pre-mpe-set=" \
    "post-mpe-set=" \
    "restore-eq-pre=" \
    "ncr-mpe-write-performed=" \
    "ncr-restore-write-performed=" \
    "man-writes-performed=false phy-id-reads-performed=false" \
    "touched-fields=" \
    "allowed-classifications=rp1-ethernet-mdio-mpe-enable-set-readback-restored" \
    "rp1-ethernet-mdio-mpe-enable-already-set-restored" \
    "rp1-ethernet-mdio-mpe-enable-readback-mismatch-restored" \
    "rp1-ethernet-mdio-mpe-enable-restore-failed" \
    "rp1-ethernet-mdio-mpe-enable-blocked-target-inconclusive" \
    "precise-staging-capture-blocker" \
    "no-mdio-no-ethernet-rp1-ethernet-mdio-mpe-enable-control" \
    "claims-ncr-mpe-write-readback-restore-boundary=" \
    "claims-man-writes=false claims-phy-id-reads=false" \
    "claims-mdio-phy-ownership=false claims-gpio32-phy-reset-ownership=false" \
    "claims-ethernet-ready=false claims-broad-mmio-ready=false" \
    "claims-networking=false claims-sockets=false" \
    "classification="; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing MDIO MPE enable candidate string: $required" >&2
        exit 1
    fi
done

if [ -n "$CAPTURE_NONCE" ]; then
    if ! grep -Fq -- " capture-nonce=" "$kernel_strings"; then
        echo "kernel image missing MDIO MPE enable candidate capture nonce label" >&2
        exit 1
    fi
    if ! grep -Fq -- "$CAPTURE_NONCE" "$kernel_strings"; then
        echo "kernel image missing MDIO MPE enable candidate capture nonce: $CAPTURE_NONCE" >&2
        exit 1
    fi
fi

for forbidden in \
    "TALOS: rp1-ethernet-mdio-mpe-enable-control" \
    "no-ncr-mpe-no-mdio-no-ethernet-target-construction" \
    "target=none controller=none compatible=none" \
    "ncr-observed-target=not-constructed" \
    "man-writes-performed=true" \
    "phy-id-reads-performed=true" \
    "TALOS: rp1-ethernet-mdio-phy-id-candidate" \
    "physid1-man-frame" \
    "gpio32-event-clear" \
    "gpio32-phy-reset-write-restore"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden MDIO MPE enable candidate string: $forbidden" >&2
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
printf 'forbidden_mdio_mpe_enable_candidate_runtime_strings_absent=true\n'
