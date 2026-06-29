# Phase 12 SSH Live TCP Candidate Kernel Entry Blocker Reconciliation

Task id: phase12-ssh-live-tcp-candidate-kernel-entry-blocker-reconciliation-20260629

Status: accepted after commit.

Classification: candidate-entry-discriminator-ready.

Evidence level: static inspection, accepted task/evidence review,
non-published Pi 5 boot-tree/archive static review, task-owned JSON evidence,
docs build, and diff checks. No lab publication, Pi 5 power-cycle,
hardwareTestLock acquisition, packet-I/O, OpenSSH/generated-root retry, remote
receipt, compatibility, service success, ssh-ready=true, broad shell work, or
phase transition was performed.

## Goal

Resolve the first missing fact after v8 proved selected candidate bytes were
served but the nonce-bearing runtime marker never appeared, without repeating a
candidate preflight or expanding to packet-I/O/OpenSSH.

## Scope Performed

- Reviewed the accepted v6 and v8 blocked-candidate-kernel-not-starting task
  records, the runtime-marker-route repair, the selected-fetch-path repair,
  and the v8 helper-owned capture evidence.
- Inspected the runtime-marker-route owners: build scenario routing, Pi 5 boot
  entry routing, runtime marker source, network runtime boundary, linker/image
  layout, and runtime-marker boot-tree/archive helpers.
- Rebuilt a non-published runtime-marker-route boot tree from the retained v8
  boot source and retained only metadata, manifests, hashes, and token review.
- Defined the next hardware discriminator as the smallest selected-fetch entry
  marker check: selected da591740/kernel_2712.img fetch -> TALOS: kernel_main
  -> TALOS: ssh-service-smoltcp-runtime-route-start -> nonce-bearing
  TALOS: ssh-service-smoltcp-runtime-ready.

## Terminal Classification

candidate-entry-discriminator-ready.

The first missing fact after v8 is not selected archive publication, selected
TFTP fetch identity, helper-owned pre-restore identity, or a static route
wiring defect. v8 proved two selected da591740/kernel_2712.img serves at
152,160 bytes and final pre-restore candidate tree
2f5083a58d2371dc13431cd545c5f9846ca9287a00531bcb31d1656d5665fb3a, but the
post-power serial window did not show TALOS: kernel_main or the nonce-bearing
runtime marker.

The selected next task is
phase12-ssh-live-tcp-pi5-entry-marker-discriminator-v9-20260629. It must keep
the v8 selected-fetch/final-pre-restore contract, then fail closed as:

- blocked-candidate-identity if selected fetch or final pre-restore identity is
  missing or mismatched;
- blocked-candidate-entry-not-starting if selected fetch is proven but
  TALOS: kernel_main is absent;
- candidate-entry-ready-runtime-blocked if TALOS: kernel_main appears but the
  runtime route-start or ready marker is absent;
- candidate-entry-and-runtime-ready only if selected fetch identity, clean
  helper-owned pre-restore candidate window, TALOS: kernel_main, route-start,
  and nonce-bearing runtime-ready marker all appear in order.

planningNeeded: false.

## Findings

- fixed: v7 selected-fetch path repair carried into v8 hardware evidence; v8
  proved selected da591740/kernel_2712.img serves and final pre-restore
  candidate identity.
- not-an-issue: the runtime-marker route is wired through build.rs,
  src/boot/rpi5.rs, src/target/rpi5.rs, and src/network.rs; static review
  found no route, linker, Image header, or helper defect explaining the missing
  serial marker.
- not-an-issue: TALOS: kernel_main is emitted byte-by-byte by
  write_early_phase_line, so it is a valid hardware serial marker even though
  it is not a contiguous static string in the kernel image.
- deferred: the remaining question is hardware execution/serial visibility
  after selected fetch. v9 must distinguish selected-fetch/no-entry from
  entry-ready/runtime-route-blocked before packet-I/O or OpenSSH can resume.
- removed: the temporary non-published archive and boot tree generated for this
  reconciliation were removed after retaining metadata and static review.

## Evidence Map

- Evidence map:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-candidate-kernel-entry-blocker-reconciliation/evidence-map.json.
- Source owner inspection:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-candidate-kernel-entry-blocker-reconciliation/static/source-owner-inspection.md.
- Static string review:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-candidate-kernel-entry-blocker-reconciliation/static/string-token-review.txt.
- Kernel byte/hash review:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-candidate-kernel-entry-blocker-reconciliation/static/kernel-bytes.txt,
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-candidate-kernel-entry-blocker-reconciliation/static/kernel-sha256.txt.
- Non-published archive review:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-candidate-kernel-entry-blocker-reconciliation/validation/runtime-marker-archive-review.stdout.txt.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes.
- Static inspection of accepted v6/v8 task records, runtime-marker-route
  source owners, selected-fetch helper changes, boot-tree/archive helpers,
  linker-rpi5.ld, src/main.rs, src/boot/rpi5.rs, and src/target/rpi5.rs:
  pass.
- Non-published Pi 5 boot-tree/archive static review: pass; selected
  da591740/kernel_2712.img and root kernel_2712.img match at 152,144 bytes
  with SHA-256 980af8c3973a205f1d3e69b99f01900ac7def66c3c88815c97ab02daad269676,
  and runtime route-start/runtime-ready/nonce/claim-boundary tokens are
  embedded.
- cargo fmt --all -- --check: not run; Rust source was not touched.
- cargo -Zjson-target-spec test --quiet: not run; Rust source was not touched.
- sh -n: not run; shell helpers were not touched.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-pi5-entry-marker-discriminator-v9-20260629.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
