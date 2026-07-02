# V61 Static Comparison Report

Task id: phase12-ssh-live-tcp-selected-normal-runtime-pre-rust-after-entry-reconciliation-v61-20260702

Classification: selected-normal-runtime-pre-rust-discriminator-ready.

## Inputs Compared

- Accepted v59 closeout commit dc7f065f3fe6df3abfa874ddc65c47a5d371e2c6.
- v59 selected hardware evidence: selected post-power identity, selected same-window TFTP service, final selected pre-restore identity, restore proof, and TALOS: asm_start retained 547 times.
- Earlier v35/v36 pre-rust line: current-source helper and scenario stop after assembly setup and before rust_entry, then require TALOS: asm_pre_rust_entry.
- Current source/helper state: build.rs registers rpi5_ssh_service_smoltcp_pre_rust_marker_loop with TALOS_RPI5_PRE_RUST_ENTRY_LOOP_SCENARIO; scripts/rpi5-ssh-service-smoltcp-pre-rust-marker-loop-boot-tree.sh materializes the selected normal-runtime serial-prefixed boot tree.

## Current Source Facts

- src/arch/aarch64/boot.S emits TALOS: asm_start for the pre-rust loop scenario.
- The selected pre-rust loop then executes CPACR setup, BSS clearing, and stack setup.
- The loop at _start+0x90 through _start+0xac emits TALOS: asm_pre_rust_entry and branches back before rust_entry.
- The rust_entry branch remains later at _start+0xb4 and is not reached by this discriminator.
- The refreshed archive preserves the selected normal-runtime service cfg strings and route/runtime marker strings, but this task does not claim those later markers are reachable.

## Artifact Contract

- Source commit: dc7f065f3fe6df3abfa874ddc65c47a5d371e2c6.
- Archive: target/tmp/selected-normal-runtime-pre-rust-v61.tar.gz.
- Archive SHA-256: 6e7a35f4d875a510719ca8fbdb256f6513d8d0b1eb6c5e321e198b75f8878cd9.
- Selected fetch path: da591740/kernel_2712.img.
- Selected kernel SHA-256: 90c72361bc67be8933436ddc5e6807dc127a8cb329a3fcab49404c10f8086d59.
- Selected kernel size: 152144 bytes.
- Image header: text_offset=0, header_image_size=152144, flags=12, magic=ARMd.
- Root and selected da591740/kernel_2712.img are byte-identical.
- Required marker for v62: TALOS: asm_pre_rust_entry.
- Restore target: phase12-ssh-v10-openssh-clean-pre-20260624T074100Z.

## Why This Is Not Re-running V59

v59 used an entry-loop archive whose required marker was TALOS: asm_start and whose loop stopped before CPACR/BSS/stack setup. v61 refreshes a different current-source archive: it preserves the selected normal-runtime boot-tree shape but moves the loop after CPACR enable, BSS clear, and stack setup. A v62 hardware pass for TALOS: asm_pre_rust_entry would prove the next boundary after v59; a miss would isolate the failure between selected assembly entry and the post-stack pre-rust marker.

## Findings

- fixed: refreshed the selected normal-runtime pre-rust discriminator contract from current source without touching source code.
- fixed: selected TALOS: asm_pre_rust_entry as the exact v62 required success marker.
- fixed: recorded exact archive, selected-kernel, Image header, restore target, marker family, fail-closed classifications, and dry-run capture contract.
- not-an-issue: the pre-rust discriminator intentionally does not reach rust_entry, route-start, runtime-blocked, runtime-ready, packet-I/O, OpenSSH, or service readiness.
- deferred: serialized Pi 5 evidence for this exact archive belongs to v62 under hardwareTestLock.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt, compatibility/service readiness, ssh-ready=true, fake command expansion, broad shell work, hardware action, and phase transition as immediate outcomes of v61.
