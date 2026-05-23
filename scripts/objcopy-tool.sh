objcopy_tool="${OBJCOPY:-}"
if [ -z "$objcopy_tool" ]; then
    if command -v rust-objcopy >/dev/null 2>&1; then
        objcopy_tool="rust-objcopy"
    else
        host="$(rustc -vV | sed -n 's/^host: //p')"
        sysroot="$(rustc --print sysroot)"
        objcopy_tool="$sysroot/lib/rustlib/$host/bin/rust-objcopy"
    fi
fi

if [ ! -x "$objcopy_tool" ] && ! command -v "$objcopy_tool" >/dev/null 2>&1; then
    echo "rust-objcopy not found; install llvm-tools-preview or set OBJCOPY" >&2
    exit 1
fi
