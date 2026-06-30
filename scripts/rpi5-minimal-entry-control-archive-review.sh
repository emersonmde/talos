#!/bin/sh
set -eu

if [ "$#" -lt 1 ] || [ "$#" -gt 2 ]; then
    echo "usage: $0 <talos-rpi5-boot.tar.gz> [capture-nonce]" >&2
    exit 2
fi

ARCHIVE="$1"
CAPTURE_NONCE="${2:-${TALOS_CAPTURE_NONCE:-minimal-entry-control-static}}"

case "$CAPTURE_NONCE" in
    ''|*[!A-Za-z0-9_.:-]*)
        echo "capture nonce may contain only A-Z, a-z, 0-9, _, ., :, and -" >&2
        exit 2
        ;;
esac

tmp_root="${TMPDIR:-/tmp}"
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-minimal-entry-control-review.XXXXXX")"
cleanup() {
    rm -rf "$work_dir"
}
trap cleanup EXIT

./scripts/rpi5-archive-review.sh "$ARCHIVE"
tar -xzf "$ARCHIVE" -C "$work_dir"

if ! cmp -s "$work_dir/kernel_2712.img" "$work_dir/da591740/kernel_2712.img"; then
    echo "serial-prefixed selected kernel differs from root kernel_2712.img" >&2
    exit 1
fi

strings "$work_dir/kernel_2712.img" > "$work_dir/kernel_2712.strings"

required_marker="TALOS: minimal-entry-control-ready capture-nonce=$CAPTURE_NONCE"
for token in \
    "TALOS: minimal-entry-control-ready" \
    "capture-nonce=" \
    "$CAPTURE_NONCE" \
    "contract-id=phase12-ssh-live-tcp-minimal-entry-control-v1" \
    "selected-fetch-path=da591740/kernel_2712.img" \
    "expected-previous-marker=kernel_main" \
    "source=kernel-main-entry-control" \
    "live-tcp-route=false" \
    "packet-io=false" \
    "openssh=false" \
    "ssh-ready=false" \
    "claims-service-success=false" \
    "claims-phase-transition=false"
do
    if ! grep -Fq "$token" "$work_dir/kernel_2712.strings"; then
        echo "missing minimal entry-control token in kernel_2712.img: $token" >&2
        exit 1
    fi
done

printf 'minimal_entry_control=ready\n'
printf 'required_marker=%s\n' "$required_marker"
printf 'archive=%s\n' "$ARCHIVE"
