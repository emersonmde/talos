#!/bin/sh
set -eu

elf_file="${1:-target/aarch64-talos-rpi5-bcm2712/debug/talos}"

if [ ! -f "$elf_file" ]; then
    echo "ELF file does not exist: $elf_file" >&2
    exit 1
fi

llvm_nm="${LLVM_NM:-}"
if [ -z "$llvm_nm" ]; then
    llvm_nm="$(find "${HOME}/.rustup/toolchains" -path '*/lib/rustlib/*/bin/llvm-nm' -type f | sort | tail -1)"
fi

if [ ! -x "$llvm_nm" ]; then
    echo "llvm-nm unavailable; set LLVM_NM" >&2
    exit 1
fi

tmp_symbols="target/tmp/rpi5-el0-trap-proof-symbols.txt"
mkdir -p target/tmp
"$llvm_nm" -n "$elf_file" > "$tmp_symbols"

sym() {
    name="$1"
    awk -v name="$name" '$NF == name { print "0x" $1; found=1; exit } END { if (!found) exit 1 }' "$tmp_symbols"
}

hex_to_dec() {
    printf '%u' "$1"
}

require_aligned() {
    name="$1"
    value="$2"
    alignment="$3"
    if [ $((value % alignment)) -ne 0 ]; then
        printf 'symbol %s is not %#x-aligned: %#x\n' "$name" "$alignment" "$value" >&2
        exit 1
    fi
}

kernel_start="$(hex_to_dec "$(sym __kernel_start)")"
stack_top="$(hex_to_dec "$(sym __stack_top)")"
handoff="$(hex_to_dec "$(sym talos_aarch64_enter_el1_then_el0)")"
regular_vectors="$(hex_to_dec "$(sym __exception_vectors)")"
entered_el1="$(hex_to_dec "$(sym talos_rpi5_el0_trap_proof_entered_el1)")"

require_aligned __kernel_start "$kernel_start" 4096
require_aligned __exception_vectors "$regular_vectors" 2048

block_size=$((0x200000))
page_size=$((0x1000))
first_block=$((kernel_start / block_size))
last_block=$(((stack_top + block_size - 1) / block_size - 1))
first_page=$((kernel_start / page_size))
last_page=$(((stack_top + page_size - 1) / page_size - 1))

for pair in \
    "handoff:$handoff" \
    "regular_vectors:$regular_vectors" \
    "entered_el1_callback:$entered_el1" \
    "stack_top:$((stack_top - 1))"
do
    label="${pair%%:*}"
    value="${pair#*:}"
    block=$((value / block_size))
    if [ "$block" -lt "$first_block" ] || [ "$block" -gt "$last_block" ]; then
        printf '%s at %#x is outside identity discriminator blocks [%#x,%#x]\n' \
            "$label" "$value" "$first_block" "$last_block" >&2
        exit 1
    fi
done

if [ "$first_block" -ne "$last_block" ]; then
    printf 'identity page discriminator spans multiple L2 slots: %#x..%#x\n' \
        "$first_block" "$last_block" >&2
    exit 1
fi

uart10=$((0x107d001000))
uart_l1=$(((uart10 >> 30) & 0x1ff))
uart_l2=$(((uart10 >> 21) & 0x1ff))
mmio_l2_start=$(((0x107c000000 >> 21) & 0x1ff))
mmio_l2_end=$(((0x1080000000 >> 21) & 0x1ff))
if [ "$mmio_l2_end" -eq 0 ]; then
    mmio_l2_end=512
fi
if [ "$uart_l1" -ne "$((0x41))" ]; then
    printf 'UART10 L1 index mismatch: got %#x expected 0x41\n' "$uart_l1" >&2
    exit 1
fi
if [ "$uart_l2" -lt "$mmio_l2_start" ] || [ "$uart_l2" -ge "$mmio_l2_end" ]; then
    printf 'UART10 L2 index %#x is outside BCM2712 MMIO L2 descriptor range [%#x,%#x]\n' \
        "$uart_l2" "$mmio_l2_start" "$((mmio_l2_end - 1))" >&2
    exit 1
fi

printf 'rpi5-el0-trap-proof-static-check: PASS elf=%s kernel_start=%#x stack_top=%#x identity_l2=%#x identity_pages=%#x..%#x handoff=%#x regular_vectors=%#x uart10_l1=%#x uart10_l2=%#x mmio_l2=%#x..%#x\n' \
    "$elf_file" "$kernel_start" "$stack_top" "$first_block" "$first_page" "$last_page" "$handoff" "$regular_vectors" "$uart_l1" "$uart_l2" "$mmio_l2_start" "$((mmio_l2_end - 1))"
