#!/bin/sh
set -eu

if [ "$#" -ne 1 ] && [ "$#" -ne 3 ]; then
    echo "usage: $0 <candidate-archive.tar.gz> [--capture-nonce NONCE]" >&2
    exit 2
fi

ARCHIVE="$1"
MARKER="TALOS: rp1-ethernet-gpio32-phy-reset-write-restore-candidate"
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
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-ethernet-gpio32-phy-reset-write-restore-candidate-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-ethernet-gpio32-phy-reset-write-restore-candidate: start" \
    "rpi5-rp1-ethernet-gpio32-phy-reset-write-restore-candidate: before-gpio32-eth-rst-n-write-restore" \
    "$MARKER" \
    "phase12-rp1-ethernet-gpio32-phy-reset-write-restore-report-contract-v1" \
    "phase12-rp1-ethernet-gpio32-phy-reset-write-restore-source-contract-v1" \
    "phase12-rp1-ethernet-gpio32-phy-reset-write-restore-source-contract-20260610" \
    "hardware-proof-limited-to-gpio32-phy-reset-write-restore-control-output" \
    "report-kind=" \
    "candidate" \
    "target=rp1-ethernet-gpio32-eth-rst-n-write-restore" \
    "gpio-controller=rp1_gpio gpio-line=32 signal=ETH_RST_N bank=bank1 bank-local-bit=4" \
    "active-low=true reset-duration-ms=5" \
    "gpio32-status-address=" \
    "gpio32-ctrl-address=" \
    "rio1-out-address=" \
    "rio1-oe-address=" \
    "rio1-in-address=" \
    "gpio32-pad-address=" \
    "baseline-status=" \
    "baseline-ctrl=" \
    "baseline-out=" \
    "baseline-oe=" \
    "baseline-in=" \
    "baseline-pad=" \
    "funcsel=" \
    "override-bits=" \
    "event-bits=" \
    "irq-bits=" \
    "pad-out-disabled=" \
    "writes-performed=" \
    "assertion-out=" \
    "assertion-oe=" \
    "assertion-in=" \
    "deassertion-out=" \
    "deassertion-oe=" \
    "deassertion-in=" \
    "restore-out=" \
    "restore-oe=" \
    "restore-in=" \
    "restore-out-eq-baseline=" \
    "restore-oe-eq-baseline=" \
    "wait-ticks=" \
    "touched-fields=RIO1_OUT.bit4,RIO1_OE.bit4" \
    "future-proof-classifications=rp1-ethernet-gpio32-phy-reset-write-restored" \
    "rejected-runtime-hardware-claims=mdio-transactions-or-phy-ownership" \
    "claims-gpio32-write-restore-only=true" \
    "claims-mdio-transactions=false" \
    "claims-phy-ownership=false" \
    "claims-ethernet-driver-ready=false" \
    "claims-non-gpio32-writes=false" \
    "claims-interrupt-ownership=false" \
    "claims-dma-descriptor-ownership=false" \
    "claims-packet-io=false" \
    "claims-networking=false" \
    "claims-sockets=false" \
    "claims-ssh=false" \
    "claims-phase-12-2=false" \
    "claims-phase-transition=false" \
    "classification=" \
    "rp1-ethernet-gpio32-phy-reset-write-restored" \
    "rp1-ethernet-gpio32-phy-reset-write-assertion-mismatch-restored" \
    "rp1-ethernet-gpio32-phy-reset-write-deassertion-mismatch-restored" \
    "rp1-ethernet-gpio32-phy-reset-write-restore-failed" \
    "rp1-ethernet-gpio32-phy-reset-blocked-sentinel-read" \
    "rp1-ethernet-gpio32-phy-reset-blocked-unsafe-function" \
    "rp1-ethernet-gpio32-phy-reset-blocked-unexpected-event-state"; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing GPIO32 PHY-reset write/restore candidate string: $required" >&2
        exit 1
    fi
done

if [ -n "$CAPTURE_NONCE" ]; then
    if ! grep -Fq -- " capture-nonce=" "$kernel_strings"; then
        echo "kernel image missing GPIO32 PHY-reset write/restore candidate capture nonce label" >&2
        exit 1
    fi
    if ! grep -Fq -- "$CAPTURE_NONCE" "$kernel_strings"; then
        echo "kernel image missing GPIO32 PHY-reset write/restore candidate capture nonce: $CAPTURE_NONCE" >&2
        exit 1
    fi
fi

for forbidden in \
    "TALOS: rp1-ethernet-gpio32-phy-reset-write-restore-control" \
    "classification=no-gpio-write-no-ethernet-rp1-ethernet-gpio32-phy-reset-control" \
    "target=none gpio-controller=none" \
    "gpio32-status-address=not-constructed"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden GPIO32 PHY-reset write/restore candidate string: $forbidden" >&2
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
