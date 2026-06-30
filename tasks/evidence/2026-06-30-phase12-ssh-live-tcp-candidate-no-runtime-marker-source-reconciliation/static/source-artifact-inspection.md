# Source and Artifact Inspection

Task id: phase12-ssh-live-tcp-candidate-no-runtime-marker-source-reconciliation-20260630

## Reviewed Inputs

- Accepted v10 Pi 5 candidate preflight:
  tasks/2026-06-30-phase12-ssh-live-tcp-pi5-candidate-preflight-v10.md.
- Accepted v10 evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-candidate-preflight-v10/candidate-preflight-v10-20260630T064353Z/evidence-map.json.
- Accepted current-tree production-timer control:
  tasks/2026-06-30-phase12-ssh-live-tcp-pi5-current-tree-production-timer-entry-baseline-discriminator.md.
- Runtime-marker source route:
  src/boot/rpi5.rs and src/target/rpi5.rs.
- Serial readiness helper:
  scripts/rpi5-observe-runtime-readiness.sh.

## Candidate Artifact Findings

- Fresh materialization command used
  TALOS_CAPTURE_NONCE=candidate-no-runtime-marker-reconciliation with
  scripts/rpi5-ssh-service-smoltcp-runtime-ready-boot-tree.sh.
- Archive review passed with matching root and selected
  da591740/kernel_2712.img, valid Image header fields, and expected
  runtime-marker tokens.
- The reviewed candidate kernel was 152,160 bytes with SHA-256
  77a751a54f4d71d461778daac35326f966a1cced9064f1bafd4b64095a842ee1.
  The size differs from v10 only because the static review nonce is longer.
- Optimized symbol inspection kept _start, rust_entry,
  boot::rpi5::kernel_main, run_ssh_service_smoltcp_runtime_ready_route, and
  live_tcp_runtime_marker_route_report in the image.

Disposition: not-an-issue.

## Helper Contract Finding

scripts/rpi5-observe-runtime-readiness.sh previously made
TALOS: kernel_main mandatory for all success-marker checks. Accepted
production-timer controls proved kernel_main can be metadata-only absent while
a downstream marker is valid. The runtime-marker candidate contract requires
nonce-bearing route-start and runtime-ready markers; it does not require
kernel_main as a mandatory success marker.

Disposition: fixed by adding TALOS_READINESS_REQUIRED_MARKERS for all-of marker
checks and TALOS_READINESS_REQUIRE_KERNEL_MARKER=false for contracts where
kernel_main is metadata-only. The default keeps kernel_main required.

## Remaining Hardware Question

The accepted v10 serial window still did not contain route-start or
runtime-ready. This no-hardware task cannot distinguish a candidate runtime
route failure from a candidate-specific hardware/capture limitation. The
bounded repair makes the next v11 checker match the accepted marker contract.

Disposition: deferred to
phase12-ssh-live-tcp-pi5-candidate-runtime-marker-preflight-v11-20260630.
