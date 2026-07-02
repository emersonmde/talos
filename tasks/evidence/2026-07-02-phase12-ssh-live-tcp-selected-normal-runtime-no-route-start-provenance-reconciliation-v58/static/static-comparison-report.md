# V58 Static Comparison Report

Task: phase12-ssh-live-tcp-selected-normal-runtime-no-route-start-provenance-reconciliation-v58-20260702

## Inputs

- Route-start-good archive: target/tmp/selected-normal-runtime-route-start-v49.tar.gz.
- Runtime-ready archive: target/tmp/selected-normal-runtime-runtime-ready-v51.tar.gz.
- Marker-family contract: tasks/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-runtime-ready-marker-family-reconciliation-v56.md.
- No-route-start evidence: tasks/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-runtime-ready-marker-family-preflight-v57.md and the accepted v57 closeout.
- Refreshed discriminator archive: target/tmp/selected-normal-runtime-entry-loop-v58.tar.gz.

## Findings

- source routing: not-an-issue. The base rpi5_ssh_service_smoltcp_runtime_ready scenario is registered in build.rs and calls run_ssh_service_smoltcp_runtime_ready_route from src/boot/rpi5.rs.
- target selection: not-an-issue. The runtime-ready route writes route-start, then runtime-ready or runtime-blocked, and stops before packet-I/O/OpenSSH acceptance claims.
- linker/Image/header: not-an-issue. The refreshed v58 archive has kernel_size=152144, header_image_size=152144, text_offset=0, flags=12, and no loader diagnostic tokens.
- archive materialization: fixed. Re-materialized a current-source selected normal-runtime entry-loop archive at target/tmp/selected-normal-runtime-entry-loop-v58.tar.gz with SHA-256 9988a761539867a50db538d64533df78b0af6d9cd3277ee0a1189cd3b2effc37.
- marker-source presence: fixed. The refreshed discriminator preserves the selected normal-runtime runtime-ready strings and adds the repeated assembly TALOS: asm_start loop before Rust-side runtime work.
- helper contract: fixed. The v59 dry-run contract uses TALOS: asm_start as required_success_marker and records fail-closed marker-family classifications from no-route-start through runtime-ready.
- evidence/redaction hygiene: not-an-issue. This no-hardware reconciliation retains only task ids, local paths, hashes, byte counts, marker names, classification labels, and validation outcomes.

## Reconciliation

The accepted v50 route-start proof shows that a selected normal-runtime archive can be served by TFTP and retain a repeated marker on Pi 5. The accepted v57 marker-family preflight shows the v51 runtime-ready archive was selected and served at the expected 152144-byte size, but retained zero marker-family members from TALOS: asm_start through runtime-ready. That makes packet-I/O dependency-blocked and shifts the next proof back to the earliest retained execution point.

The refreshed v58 discriminator uses the already established selected normal-runtime entry-loop shape: it keeps the selected normal-runtime service cfg and 152144-byte Image contract, but loops on TALOS: asm_start before CPACR setup, BSS clear, stack setup, Rust entry, BootInfo parsing, kernel_main, route-start, runtime-blocked, runtime-ready, packet-I/O, service readiness, ssh-ready, fake command expansion, broad shell work, or phase transition.

If v59 retains TALOS: asm_start, the current selected no-route-start frontier is repaired back to the entry boundary and the closeout can decide the next continuation. If v59 serves the selected bytes but still retains no marker after known-good control/rerun triage, the first missing fact remains before selected Talos Image entry.

## Refreshed Discriminator Artifact

- Source commit for artifact: f7f5133e3cc107d6d9079a9128effe38bb77b8c7.
- Archive path: target/tmp/selected-normal-runtime-entry-loop-v58.tar.gz.
- Archive SHA-256: 9988a761539867a50db538d64533df78b0af6d9cd3277ee0a1189cd3b2effc37.
- Selected fetch path: da591740/kernel_2712.img.
- Selected kernel SHA-256: 6a7b970144e43c5b57b343c5ee4ff1275b077403ee83c3806dedd740acc89301.
- Selected kernel size: 152144 bytes.
- Restore target: phase12-ssh-v10-openssh-clean-pre-20260624T074100Z.
- Required v59 marker: TALOS: asm_start.
- Marker family: TALOS: asm_start; TALOS: asm_pre_rust_entry; TALOS: kernel_main; TALOS: ssh-service-smoltcp-runtime-route-start capture-nonce=runtime-no-route-start-v58; TALOS: ssh-service-smoltcp-runtime-blocked capture-nonce=runtime-no-route-start-v58; TALOS: ssh-service-smoltcp-runtime-ready capture-nonce=runtime-no-route-start-v58.
