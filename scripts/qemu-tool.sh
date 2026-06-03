qemu_tool="${QEMU_SYSTEM_AARCH64:-${QEMU:-}}"
if [ -z "$qemu_tool" ]; then
    qemu_tool="qemu-system-aarch64"
fi

if ! command -v "$qemu_tool" >/dev/null 2>&1; then
    echo "$qemu_tool not found; install QEMU or set QEMU_SYSTEM_AARCH64" >&2
    exit 1
fi
