# Phase 11 Pi 5 Run-Unique Capture Marker Core

Task id: phase11-pi5-run-unique-capture-marker-core-20260608

Status: accepted

Classification: run-unique-capture-marker-contract-accepted

## Goal

Create a stronger Pi 5 serial freshness discriminator after V3 rejected the
observed GPIO status no-MMIO control because its constant marker was already
present before power.

## Scope

- Inspected the committed V3 control blocker, V3 core/known-good evidence,
  lab-controller serial docs, capture-invariant helper, V3 checker, and
  observed GPIO status archive review scripts.
- Added run-unique nonce support to the observed GPIO status result/control
  marker using TALOS_CAPTURE_NONCE.
- Added a run-unique replay checker that keeps V3 and rejects required markers
  without capture-nonce=.
- Updated the lab-controller and RP1/PCIe map contract docs for the next
  mechanical hardware control procedure.

## Findings And Disposition

- fixed: embedded optional TALOS_CAPTURE_NONCE into the observed GPIO status
  result/control serial markers without changing the source contract or MMIO
  behavior.
- fixed: added scripts/rpi5-proof-identity-join-run-unique-check.sh.
- fixed: updated archive review scripts to statically require the nonce when a
  task supplies --capture-nonce.
- fixed: retained a negative replay proving the constant marker failure remains
  rejected.
- fixed: retained a synthetic run-unique replay proving the stronger marker is
  mechanically checkable when V3 identity/freshness checks pass.
- deferred: no lab-controller monotonic serial cursor endpoint was added.
- not-an-issue: the prior marker-visible GPIO14 STATUS/CTRL values remain
  unaccepted hardware behavior; this task only repairs capture freshness.

No findings were removed.

## Next Hardware Procedure

The next no-MMIO control proof must generate one nonce for its staged archive,
build with TALOS_CAPTURE_NONCE=<nonce>, static-review with
--capture-nonce <nonce>, capture with required marker
TALOS: rp1-observed-gpio-status-control capture-nonce=<nonce>, and replay
with rpi5-proof-identity-join-run-unique-check.sh --nonce <nonce>.

The proof must still retain pre-power absence, post-power presence, selected
tree identity, TFTP delta, final pre-restore identity, and restore proof before
accepting the control output/capture path.

## Evidence

- Static problem analysis:
  tasks/evidence/2026-06-08-phase11-pi5-run-unique-capture-marker-core/static-problem-analysis.md.
- Evidence map:
  tasks/evidence/2026-06-08-phase11-pi5-run-unique-capture-marker-core/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-08-phase11-pi5-run-unique-capture-marker-core/classification.json.
- Constant-marker rejection replay:
  tasks/evidence/2026-06-08-phase11-pi5-run-unique-capture-marker-core/stale-constant-marker-rejected.json.
- Run-unique synthetic replay:
  tasks/evidence/2026-06-08-phase11-pi5-run-unique-capture-marker-core/run-unique-synthetic-pass.json.
- Archive reviews with nonce:
  tasks/evidence/2026-06-08-phase11-pi5-run-unique-capture-marker-core/control-archive-review.txt and
  tasks/evidence/2026-06-08-phase11-pi5-run-unique-capture-marker-core/read-archive-review.txt.
- Runtime nonce string inspection:
  tasks/evidence/2026-06-08-phase11-pi5-run-unique-capture-marker-core/control-runtime-nonce-strings.txt and
  tasks/evidence/2026-06-08-phase11-pi5-run-unique-capture-marker-core/read-runtime-nonce-strings.txt.

## Validation

- static inspection of blocker evidence, V3 proof evidence, lab-controller
  serial docs, capture-invariant helper, V3 checker, and archive review scripts:
  passed.
- bash -n on touched shell scripts: passed.
- task-owned fixture replay: passed; constant marker rejected and synthetic
  run-unique marker accepted.
- image/archive inspection with TALOS_CAPTURE_NONCE=core20260608A: passed for
  control and read archives.
- jq empty on evidence-map/classification/replay JSON: passed.
- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet: passed; 423 no_std tests passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed.

## Result

Accepted as run-unique-capture-marker-contract-accepted. This task does not
acquire hardware, publish an archive, power-cycle the Pi 5, or accept GPIO14
STATUS/CTRL visibility.
