if [ -z "${script_dir:-}" ]; then
    echo "script_dir must be set before sourcing qemu-nographic-smoke-lib.sh" >&2
    exit 2
fi

. "$script_dir/objcopy-tool.sh"
. "$script_dir/qemu-tool.sh"

talos_qemu_prepare_image() {
    talos_qemu_scenario="$1"
    talos_qemu_profile="$2"
    talos_qemu_image_suffix="$3"
    shift 3

    case "$talos_qemu_profile" in
        debug)
            if [ -n "$talos_qemu_scenario" ]; then
                TALOS_BOOT_SCENARIO="$talos_qemu_scenario" cargo -Zjson-target-spec build "$@"
            else
                cargo -Zjson-target-spec build "$@"
            fi
            ;;
        release)
            if [ -n "$talos_qemu_scenario" ]; then
                TALOS_BOOT_SCENARIO="$talos_qemu_scenario" cargo -Zjson-target-spec build --release "$@"
            else
                cargo -Zjson-target-spec build --release "$@"
            fi
            ;;
        *)
            echo "unsupported QEMU build profile: $talos_qemu_profile" >&2
            exit 2
            ;;
    esac

    ELF_FILE="target/aarch64-talos-virt/$talos_qemu_profile/talos"
    IMG_FILE="$ELF_FILE$talos_qemu_image_suffix.img"
    "$objcopy_tool" -O binary "$ELF_FILE" "$IMG_FILE"
}

talos_qemu_run_nographic() {
    talos_qemu_machine="$1"
    talos_qemu_smp="$2"
    talos_qemu_log="$3"

    if [ -n "$talos_qemu_smp" ]; then
        "$qemu_tool" \
            -M "$talos_qemu_machine" \
            -cpu cortex-a76 \
            -smp "$talos_qemu_smp" \
            -m 256M \
            -nographic \
            -serial mon:stdio \
            -semihosting-config enable=on,target=native \
            -kernel "$IMG_FILE" >"$talos_qemu_log" 2>&1
    else
        "$qemu_tool" \
            -M "$talos_qemu_machine" \
            -cpu cortex-a76 \
            -m 256M \
            -nographic \
            -serial mon:stdio \
            -semihosting-config enable=on,target=native \
            -kernel "$IMG_FILE" >"$talos_qemu_log" 2>&1
    fi
}
