#!/bin/sh
set -eu

if [ "$#" -ne 1 ] && [ "$#" -ne 3 ]; then
    echo "usage: $0 <candidate-archive.tar.gz> [--capture-nonce NONCE]" >&2
    exit 2
fi

ARCHIVE="$1"
MARKER="TALOS: rp1-ethernet-clock-reset-readonly-baseline-candidate"
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
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-rp1-ethernet-clock-reset-baseline-candidate-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

for required in \
    "rpi5-rp1-ethernet-clock-reset-readonly-baseline-candidate: start" \
    "read-only-baseline-report-no-mmio-writes" \
    "$MARKER" \
    "phase12-rp1-ethernet-clock-reset-guard-contract-v1" \
    "phase12-rp1-ethernet-clock-reset-ownership-contract-20260610" \
    "phase12-rp1-ethernet-prereq-ownership-contract-v1" \
    "hardware-proof-limited-to-clock-reset-readonly-baseline-report-visibility-control-output" \
    "report-kind=" \
    "candidate" \
    "observed-window-macb-mid-context-cpu-physical-target=" \
    "observed-window-macb-mid-context-raw=" \
    "observed-window-macb-mid-context-idnum=" \
    "observed-window-macb-mid-context-rev=" \
    "context-only-not-broad-ethernet-mmio-readiness" \
    "selected-read-only-baseline-fields=pclk,hclk,tsu_clk,tx_clk,shared-rp1-clk-sys,ethernet-private-clock-ids,reset-controller-policy,phy-reset-gpio-context" \
    "clock-names=pclk,hclk,tsu_clk,tx_clk" \
    "clock-ids=12,12,29,16" \
    "clock-sources=RP1_CLK_SYS,RP1_CLK_SYS,RP1_CLK_ETH_TSU,RP1_CLK_ETH" \
    "shared-clock-names=pclk,hclk" \
    "shared-clock-source=RP1_CLK_SYS" \
    "shared-clock-id=12" \
    "ethernet-private-clock-names=tsu_clk,tx_clk" \
    "ethernet-private-clock-sources=RP1_CLK_ETH_TSU,RP1_CLK_ETH" \
    "ethernet-private-clock-ids=29,16" \
    "clock-policy-classification=no-clock-reset-ownership" \
    "reset-controller-policy-classification=no-accepted-rp1-eth-reset-controller-target" \
    "phy-reset-gpio-context=32" \
    "phy-mdio-policy-classification=no-phy-reset-or-mdio-ownership" \
    "read-only-baseline-requirements=same-candidate-control-report-path" \
    "write-backed-invariants=do-not-transition-rp1-clk-sys" \
    "ethernet-driver-readiness" \
    "broad-ethernet-mmio-readiness" \
    "rp1-mmio-writes" \
    "clock-reset-writes" \
    "clock-reset-ownership" \
    "rp1-clk-sys-transition" \
    "reset-controller-ownership" \
    "gpio32-ownership-or-phy-reset" \
    "mdio-transactions-or-phy-ownership" \
    "interrupt-delivery-handler-ownership-or-completion" \
    "dma-descriptor-rings-channel-ownership-or-transfer-completion" \
    "packet-io" \
    "networking" \
    "sockets" \
    "ssh" \
    "phase-12-2" \
    "phase-transition" \
    "claims-ethernet-ready=false" \
    "claims-broad-mmio-ready=false" \
    "claims-rp1-mmio-writes=false" \
    "claims-clock-reset-writes=false" \
    "claims-clock-reset-ownership=false" \
    "claims-rp1-clk-sys-transition=false" \
    "claims-reset-controller-ownership=false" \
    "claims-gpio32-phy-reset-ownership=false" \
    "claims-mdio-phy-ownership=false" \
    "claims-interrupt-ownership=false" \
    "claims-dma-descriptor-ownership=false" \
    "classification=rp1-ethernet-clock-reset-readonly-baseline-report-visible"; do
    if ! grep -Fq -- "$required" "$kernel_strings"; then
        echo "kernel image missing RP1 Ethernet clock/reset baseline candidate string: $required" >&2
        exit 1
    fi
done

if [ -n "$CAPTURE_NONCE" ]; then
    if ! grep -Fq -- " capture-nonce=" "$kernel_strings"; then
        echo "kernel image missing RP1 Ethernet clock/reset baseline candidate capture nonce label" >&2
        exit 1
    fi
    if ! grep -Fq -- "$CAPTURE_NONCE" "$kernel_strings"; then
        echo "kernel image missing RP1 Ethernet clock/reset baseline candidate capture nonce: $CAPTURE_NONCE" >&2
        exit 1
    fi
fi

for forbidden in \
    "TALOS: rp1-ethernet-clock-reset-readonly-baseline-control" \
    "classification=no-clock-reset-no-ethernet-rp1-ethernet-clock-reset-baseline-control" \
    "selected-read-only-baseline-fields=none" \
    "observed-window-macb-mid-context-cpu-physical-target=not-constructed"; do
    if grep -Fq -- "$forbidden" "$kernel_strings"; then
        echo "kernel image contains forbidden RP1 Ethernet clock/reset baseline candidate string: $forbidden" >&2
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
printf 'forbidden_rp1_ethernet_clock_reset_baseline_candidate_runtime_strings_absent=true\n'
