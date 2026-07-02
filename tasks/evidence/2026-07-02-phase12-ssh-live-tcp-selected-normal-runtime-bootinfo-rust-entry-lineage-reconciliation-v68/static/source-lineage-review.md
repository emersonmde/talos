# Source Lineage Review

Task id: phase12-ssh-live-tcp-selected-normal-runtime-bootinfo-rust-entry-lineage-reconciliation-v68-20260702

## Reviewed Paths

- src/main.rs: rust_entry(dtb_pa) scenario dispatch and normal selected
  runtime body.
- src/target/rpi5.rs: EarlyPhaseLine serialization and
  run_ssh_service_smoltcp_bootinfo_marker_loop().
- scripts/rpi5-ssh-service-smoltcp-bootinfo-marker-loop-archive-review.sh:
  selected archive token contract.

## Result

The selected BootInfo scenario is
talos_boot_scenario=rpi5_ssh_service_smoltcp_bootinfo_marker_loop. It is not
one of the early diagnostic scenarios excluded from the normal rust_entry body.
The normal body writes the RustEntry early-phase line, calls
BootInfo::from_aarch64_x0(dtb_pa), writes the BootInfoParsed early-phase line,
then enters run_ssh_service_smoltcp_bootinfo_marker_loop().

run_ssh_service_smoltcp_bootinfo_marker_loop() is the source path that emits
the selected TALOS: boot info parsed marker, the capture nonce, and the
negative claims for target init, exceptions, kernel_main, route-start,
runtime-ready, packet-I/O, service success, ssh-ready, and phase transition.

Therefore the retained v66 TALOS: boot info parsed output from the selected
archive cannot be emitted without first reaching rust_entry and parsing
BootInfo. The missing same-window v66 TALOS: rust_entry count is retained as a
marker-capture limitation, not a control-flow blocker.

## Negative Claims

This review does not prove target init, exceptions, kernel_main, route-start,
runtime-ready, packet-I/O, OpenSSH/generated-root behavior, remote receipt,
compatibility/service readiness, ssh-ready=true, fake command expansion, broad
shell work, or a phase transition.
