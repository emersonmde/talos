#!/bin/sh
set -eu

if [ "$#" -ne 1 ] && [ "$#" -ne 3 ]; then
    echo "usage: $0 <candidate-archive.tar.gz> [--capture-nonce NONCE]" >&2
    exit 2
fi

ARCHIVE="$1"
MARKER="TALOS: rp1-ethernet-clk-eth-ctrl-write-restore-candidate"
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
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-ethernet-clk-eth-ctrl-write-restore-candidate-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-ethernet-clk-eth-ctrl-write-restore-candidate: start" \
    "rpi5-rp1-ethernet-clk-eth-ctrl-write-restore-candidate: before-clk-eth-ctrl-idempotent-write-restore" \
    "$MARKER" \
    "phase12-rp1-ethernet-clk-eth-ctrl-write-restore-report-contract-v1" \
    "phase12-rp1-ethernet-clk-eth-ctrl-write-target-source-contract-v1" \
    "phase12-rp1-ethernet-clk-eth-ctrl-source-contract-20260610" \
    "report-kind=" \
    "candidate" \
    "target=rp1-ethernet-clk-eth-ctrl-idempotent-write-restore" \
    "register=" \
    "CLK_ETH_CTRL" \
    "clock-name=tx_clk clock-id=16" \
    "observed-rp1-base=" \
    "source-offset=" \
    "address=" \
    "width=32 allowed-write-value=pre-raw-only" \
    "pre-raw=" \
    "pre-enable=" \
    "pre-auxsrc=" \
    "pre-source=" \
    "post-raw=" \
    "restore-raw=" \
    "post-eq-pre=" \
    "restore-eq-pre=" \
    "preserved-fields=full-raw,enable-bit11,auxsrc-bits9-5,source-bits1-0,reserved-bits" \
    "claims-clk-eth-ctrl-idempotent-write=true" \
    "future-proof-classifications=rp1-ethernet-clk-eth-ctrl-idempotent-write-restored" \
    "rejected-runtime-hardware-claims=ethernet-driver-readiness" \
    "claims-ethernet-ready=false" \
    "claims-shared-rp1-clk-sys-write=false" \
    "claims-clk-eth-tsu-ctrl-retry=false" \
    "claims-clk-eth-ctrl-non-idempotent-transition=false" \
    "claims-reset-controller-ownership=false" \
    "claims-gpio32-phy-reset-ownership=false" \
    "claims-mdio-phy-ownership=false" \
    "claims-dma-descriptor-ownership=false" \
    "claims-packet-io=false" \
    "claims-networking=false" \
    "claims-sockets=false" \
    "claims-ssh=false" \
    "claims-phase-12-2=false" \
    "claims-phase-transition=false" \
    "classification=" \
    "rp1-ethernet-clk-eth-ctrl-idempotent-write-restored" \
    "rp1-ethernet-clk-eth-ctrl-idempotent-write-mismatch-restored" \
    "rp1-ethernet-clk-eth-ctrl-idempotent-write-restore-failed"; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing RP1 Ethernet CLK_ETH_CTRL write/restore candidate string: $required" >&2
        exit 1
    fi
done

if [ -n "$CAPTURE_NONCE" ]; then
    if ! grep -Fq -- " capture-nonce=" "$kernel_strings"; then
        echo "kernel image missing RP1 Ethernet CLK_ETH_CTRL write/restore candidate capture nonce label" >&2
        exit 1
    fi
    if ! grep -Fq -- "$CAPTURE_NONCE" "$kernel_strings"; then
        echo "kernel image missing RP1 Ethernet CLK_ETH_CTRL write/restore candidate capture nonce: $CAPTURE_NONCE" >&2
        exit 1
    fi
fi

for forbidden in \
    "TALOS: rp1-ethernet-clock-reset-write-restore-control" \
    "target=rp1-ethernet-clk-eth-tsu-ctrl-idempotent-write-restore" \
    "register=CLK_ETH_TSU_CTRL" \
    "classification=no-clock-write-no-ethernet-rp1-ethernet-clk-eth-ctrl-control" \
    "target=none register=none" \
    "address=not-constructed"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden RP1 Ethernet CLK_ETH_CTRL candidate string: $forbidden" >&2
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
printf 'candidate_marker=%s\n' "$MARKER"
if [ -n "$CAPTURE_NONCE" ]; then
    printf 'capture_nonce=%s\n' "$CAPTURE_NONCE"
fi
printf 'forbidden_rp1_ethernet_clk_eth_ctrl_write_restore_candidate_runtime_strings_absent=true\n'
