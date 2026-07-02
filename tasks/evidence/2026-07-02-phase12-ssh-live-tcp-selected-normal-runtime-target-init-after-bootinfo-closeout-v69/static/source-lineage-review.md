# Source Lineage Review

This no-hardware review reconciles the v69 target-init evidence against the
current selected normal-runtime source path.

The selected scenario is
`talos_boot_scenario=rpi5_ssh_service_smoltcp_target_init_marker_loop`. In
`src/main.rs`, that scenario is not excluded from the normal `rust_entry`
body. The ordered path writes the RustEntry early-phase line, parses
`BootInfo::from_aarch64_x0(dtb_pa)`, writes the BootInfoParsed early-phase
line, calls `target::init(&boot_info)`, writes the TargetInit early-phase
line, and only then invokes
`target::rpi5::run_ssh_service_smoltcp_target_init_marker_loop()`.

`src/target/rpi5.rs` emits the selected loop marker as `TALOS: target init`
with `selected-normal-runtime-target-init=true`,
`claims-bootinfo-parsed=true`, and negative claims for exceptions ready,
kernel_main, route-start, runtime-ready, packet-I/O, service success,
ssh-ready, and phase transition.

The v69 first selected run and candidate rerun both retained selected TFTP
service for the 152,880-byte `da591740/kernel_2712.img` and retained
`TALOS: target init` repeatedly. They did not retain the literal
`TALOS: boot info parsed` marker in the same v69 serial windows. That absence
is a limitation of the v69 marker-family capture contract, not a source
ordering contradiction: the selected target-init marker is unreachable before
BootInfo parsing and before `target::init(&boot_info)` returns.

The selected normal-runtime target-init frontier is therefore accepted by
lineage. Exceptions ready, kernel_main, route-start, runtime-ready, packet-I/O,
OpenSSH/generated-root behavior, service success, ssh-ready=true, fake command
expansion, broad shell work, and phase transition remain unproved.
