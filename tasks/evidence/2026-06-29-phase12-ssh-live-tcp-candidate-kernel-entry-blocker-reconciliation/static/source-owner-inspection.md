## Source Owner Static Inspection

### build scenario
scripts/rpi5-ssh-service-smoltcp-runtime-ready-archive-review.sh:10:CAPTURE_NONCE="${2:-${TALOS_CAPTURE_NONCE:-runtime-marker-route-static}}"
scripts/rpi5-ssh-service-smoltcp-runtime-ready-boot-tree.sh:9:: "${TALOS_CAPTURE_NONCE:=runtime-marker-route-static}"
scripts/rpi5-ssh-service-smoltcp-runtime-ready-boot-tree.sh:10:export TALOS_CAPTURE_NONCE
scripts/rpi5-ssh-service-smoltcp-runtime-ready-boot-tree.sh:11:export TALOS_BOOT_SCENARIO=rpi5_ssh_service_smoltcp_runtime_ready
build.rs:571:        value: "rpi5_ssh_service_smoltcp_runtime_ready",
build.rs:1447:    println!("cargo:rerun-if-env-changed=TALOS_BOOT_SCENARIO");
build.rs:1448:    println!("cargo:rerun-if-env-changed=TALOS_CAPTURE_NONCE");
build.rs:1450:    if let Ok(nonce) = env::var("TALOS_CAPTURE_NONCE") {
build.rs:1452:        println!("cargo:rustc-env=TALOS_CAPTURE_NONCE={nonce}");
build.rs:1494:    let value = env::var("TALOS_BOOT_SCENARIO").ok()?;
build.rs:1499:            .unwrap_or_else(|| panic!("unsupported TALOS_BOOT_SCENARIO: {value}")),
build.rs:1521:        panic!("TALOS_CAPTURE_NONCE must be 64 characters or fewer");
build.rs:1527:        panic!("TALOS_CAPTURE_NONCE may contain only A-Z, a-z, 0-9, _, ., :, and -");

### boot route
src/main.rs:126:            talos_boot_scenario = "rpi5_ssh_service_smoltcp_runtime_ready",
src/boot/rpi5.rs:31:    target::rpi5::write_early_phase_line(target::rpi5::EarlyPhaseLine::KernelMain);
src/boot/rpi5.rs:45:    let dtb = report_boot_identity(boot_info, &services);
src/boot/rpi5.rs:47:    #[cfg(talos_boot_scenario = "rpi5_ssh_service_smoltcp_runtime_ready")]
src/boot/rpi5.rs:48:    target::rpi5::run_ssh_service_smoltcp_runtime_ready_route();
src/boot/rpi5.rs:176:fn report_boot_identity(boot_info: &BootInfo, services: &TargetServices) -> Rpi5DtbPhase {
src/target/rpi5.rs:9105:    KernelMain,
src/target/rpi5.rs:9188:        EarlyPhaseLine::KernelMain => {
src/target/rpi5.rs:13089:    write_early_static(" bootinfo-source=report_boot_identity");
src/target/rpi5.rs:25786:    talos_boot_scenario = "rpi5_ssh_service_smoltcp_runtime_ready"
src/target/rpi5.rs:26027:#[cfg(talos_boot_scenario = "rpi5_ssh_service_smoltcp_runtime_ready")]
src/target/rpi5.rs:26028:pub fn run_ssh_service_smoltcp_runtime_ready_route() -> ! {

### runtime route
src/network.rs:2739:    marker_route_ready: bool,
src/network.rs:2747:    pub(crate) const fn marker_route_ready(self) -> bool {
src/network.rs:2748:        self.marker_route_ready
src/network.rs:2806:pub(crate) fn live_tcp_runtime_marker_route_report()
src/network.rs:2843:    let marker_route_ready = runtime_report.binding_state()
src/network.rs:2858:        marker_route_ready,
src/network.rs:7140:    fn live_tcp_runtime_marker_route_report_reaches_fail_closed_runtime_path() {
src/network.rs:7141:        let report = live_tcp_runtime_marker_route_report().expect("runtime marker route");
src/network.rs:7143:        assert!(report.marker_route_ready());
src/target/rpi5.rs:26029:    write_early_static("TALOS: ssh-service-smoltcp-runtime-route-start");
src/target/rpi5.rs:26038:    match crate::network::live_tcp_runtime_marker_route_report() {
src/target/rpi5.rs:26039:        Ok(report) if report.marker_route_ready() => {
src/target/rpi5.rs:26041:            write_early_static("TALOS: ssh-service-smoltcp-runtime-ready");
src/target/rpi5.rs:26073:            write_early_static("TALOS: ssh-service-smoltcp-runtime-blocked");
src/target/rpi5.rs:26098:            write_early_static("TALOS: ssh-service-smoltcp-runtime-blocked");

### linker and image header
ENTRY(_start)

KERNEL_DRAM_BASE = 0x00200000;
KERNEL_IMAGE_TEXT_OFFSET = 0x00000000;
KERNEL_LOAD_ADDR = KERNEL_DRAM_BASE + KERNEL_IMAGE_TEXT_OFFSET;
STACK_SIZE = 0x00040000; /* 256 KiB */
HEAP_SIZE = 0x00100000;  /* reserved for the first allocator milestone */

SECTIONS
{
    . = KERNEL_LOAD_ADDR;
    __kernel_start = .;

    .text.boot : ALIGN(4K) {
        KEEP(*(.text.boot))
    }

    .vectors : ALIGN(2K) {
        KEEP(*(.vectors.rpi5_vector_section_diagnostic))
        KEEP(*(.vectors))
    }

    .text : ALIGN(4K) {
        KEEP(*(.text.rpi5_text_section_diagnostic))
        *(.text .text.*)
    }

    .rodata : ALIGN(16) {
        __rodata_start = .;
        *(.rodata .rodata.*)
        __rodata_end = .;
    }

    .data : ALIGN(4K) {
        *(.data .data.*)
    }

    __kernel_image_end = .;

    .bss (NOLOAD) : ALIGN(16) {
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        __bss_end = .;
    }

    . = ALIGN(16);
    __heap_start = .;
    . += HEAP_SIZE;
    __heap_end = .;

    . = ALIGN(16);
    __stack_bottom = .;
    . += STACK_SIZE;
    __stack_top = .;

    . = ALIGN(4K);
    __kernel_end = .;

    /DISCARD/ : {
        *(.comment)
        *(.gnu*)
        *(.note*)
        *(.eh_frame*)
    }
}
.section .text.boot, "ax"
.global _start
.type _start, %function

_start:
    b 3f
    .long 0
    .quad KERNEL_IMAGE_TEXT_OFFSET
    .quad __kernel_image_end - _start
#ifdef TALOS_TARGET_RPI5_BCM2712
    .quad 0xc
#else
    .quad 0
#endif
    .quad 0
    .quad 0
    .quad 0
    .long 0x644d5241
    .long 0

3:
    mov x19, x0

#if defined(TALOS_RPI5_SMP_BOOT_SCENARIO) || defined(TALOS_RPI5_EARLY_ENTRY_PROVENANCE_SCENARIO)
    adrp x0, talos_rpi5_asm_start_line
    add x0, x0, :lo12:talos_rpi5_asm_start_line
    mov x1, #(talos_rpi5_asm_start_line_end - talos_rpi5_asm_start_line)
    bl talos_rpi5_early_uart_write
#endif
    mrs x0, cpacr_el1
    orr x0, x0, #(0b11 << 20)
    msr cpacr_el1, x0
    isb

    adrp x0, __bss_start
    add x0, x0, :lo12:__bss_start
    adrp x1, __bss_end
    add x1, x1, :lo12:__bss_end
0:
    cmp x0, x1
    b.hs 1f
    str xzr, [x0], #8
    b 0b

1:
    adrp x0, __stack_top
    add x0, x0, :lo12:__stack_top
    mov sp, x0

#if defined(TALOS_RPI5_SMP_BOOT_SCENARIO) || defined(TALOS_RPI5_EARLY_ENTRY_PROVENANCE_SCENARIO)
    adrp x0, talos_rpi5_asm_pre_rust_line
    add x0, x0, :lo12:talos_rpi5_asm_pre_rust_line
    mov x1, #(talos_rpi5_asm_pre_rust_line_end - talos_rpi5_asm_pre_rust_line)
    bl talos_rpi5_early_uart_write
#endif

    mov x0, x19
    bl rust_entry

2:
    wfe
    b 2b

.size _start, . - _start
