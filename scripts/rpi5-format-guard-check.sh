#!/bin/sh
set -eu

repo_root="$(git rev-parse --show-toplevel)"

(
    cd "$repo_root"
    ./scripts/rpi5-image.sh
) >/dev/null

. "$repo_root/scripts/objcopy-tool.sh"
case "$objcopy_tool" in
    *rust-objcopy) objdump_tool="${objcopy_tool%rust-objcopy}llvm-objdump" ;;
    *) objdump_tool="${objcopy_tool%objcopy}objdump" ;;
esac
elf_file="$repo_root/target/aarch64-talos-rpi5-bcm2712/debug/talos"
disasm_file="$repo_root/target/aarch64-talos-rpi5-bcm2712/debug/write_early_hex_digit.disasm"

"$objdump_tool" -d --demangle "$elf_file" |
    awk '/<talos::target::rpi5::write_early_hex_digit>:/ {in_fn=1; next} in_fn && /^$/ {exit} in_fn {print}' > "$disasm_file"

if grep -Eq 'br[[:space:]]+x|ldrsw|panic_const|[.]word[[:space:]]+0x' "$disasm_file"; then
    echo "Pi 5 early hex digit writer must not use jump tables, panic paths, or literal data" >&2
    exit 1
fi

printf '%s\n' "Pi 5 formatted early-console build PASS"
