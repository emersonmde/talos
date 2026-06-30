# Selected-Fetch/No-Entry Source And Artifact Inspection

Task id: phase12-ssh-live-tcp-selected-fetch-no-entry-source-reconciliation-20260630

Scope: no hardware, no lab publication, no boot snapshot mutation, no Pi 5
power action, no packet-I/O, and no OpenSSH/generated-root retry.

## V9 Inputs

- V9 terminal classification: blocked-candidate-entry-not-starting.
- V9 retained selected fetch: da591740/kernel_2712.img served twice at
  152,152 bytes with SHA-256
  6c08216a0487b1b78067b939fdfac5f9c456ec5e7e74154ebbcba502207826eb.
- V9 retained final pre-restore candidate tree:
  49a9cb5bc267a3877979356cca273f1747cd7cc3430d82ac6c3bdbfddedc1a3e.
- V9 marker checker observed zero TALOS: kernel_main, zero
  TALOS: ssh-service-smoltcp-runtime-route-start, and zero nonce-bearing
  TALOS: ssh-service-smoltcp-runtime-ready markers.

## Source And Artifact Findings

- not-an-issue: selected-path archive materialization still mirrors the root Pi
  5 boot files under da591740/. The fresh non-published archive has both root
  and da591740/kernel_2712.img at 152,152 bytes with matching SHA-256
  d6f170e1edfe5bb3ed3b9b0455c03bce68b0a9b44b097cf5985f2916b239cc41.
- not-an-issue: Image header fields remain valid for the accepted Pi 5
  firmware-selected Image path: text_offset=0, header_image_size=152152,
  flags=12, and magic ARMd reported by the archive review.
- not-an-issue: linker/startup static inspection shows _start and
  __kernel_start at 0x200000, __kernel_image_end at 0x225258, rust_entry at
  0x20924c, and boot::rpi5::kernel_main at 0x20bbb8. Startup branches over the
  Image header, clears BSS, installs the stack, and branches to rust_entry.
- not-an-issue: the runtime-marker-route source path remains wired through the
  normal Pi 5 Rust entry path. The rpi5_ssh_service_smoltcp_runtime_ready
  scenario is not in the early diagnostic exclusion set in src/main.rs, so it
  reaches BootInfo parsing, target init, exception setup, kernel_main, and then
  run_ssh_service_smoltcp_runtime_ready_route.
- not-an-issue: runtime marker static review confirms route-start,
  runtime-ready, capture-nonce, runtime-binding, descriptor-facing delivery,
  deterministic device-interface, ssh-ready=false, and fail-closed claim tokens
  are embedded in the selected kernel image.
- deferred: raw TALOS_RPI5_EARLY_ENTRY_PROVENANCE_SCENARIO assembly markers are
  intentionally not enabled for this path. Prior accepted evidence quarantined
  that marker route as invasive for accepted Pi 5 controls, so this task does
  not repair v9 by adding that define.
- deferred: V9's clean selected-fetch/no-kernel_main boundary leaves no bounded
  source/archive defect to fix. The next qualitatively different discriminator
  should be a minimal entry-control contract that strips the live TCP runtime
  route while keeping the same selected fetch and earliest-entry mechanism.

## Classification

candidate-entry-control-contract-required.

selected_next_task:
phase12-ssh-live-tcp-minimal-entry-control-contract-20260630.
