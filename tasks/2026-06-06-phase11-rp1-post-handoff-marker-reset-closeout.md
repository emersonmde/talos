# Phase 11 RP1 Post-Handoff Marker Reset Closeout

Task id: phase11-rp1-post-handoff-marker-reset-closeout-20260606

Status: accepted

## Goal

Close out the post-handoff marker/reset discriminator and decide whether the
RP1 UART0 flag-register diagnostic is mechanically unblocked.

## Scope

- Reconciled the accepted source/static marker-reset core evidence with the
  completed serialized Pi 5 discriminator evidence.
- Recorded accepted, deferred, and not-an-issue findings.
- Updated the Phase 11 RP1/PCIe map contract with the reconciled boundary.
- Did not run hardware, publish a boot archive, acquire the hardware lock, or
  change runtime/kernel source.

## Final Classification

Classification: staging-capture-blocked.

The source/static core proves the selected 51,736-byte
target/talos-rpi5-post-handoff-marker-reset-core.tar.gz image routes from
_start to rust_entry, emits the current UART10 early-serial marker path, and
then calls PSCI SYSTEM_RESET before BootInfo parsing, target initialization,
boot reports, memory planning, allocator setup, or the RP1 UART0 FR read path.
The Pi 5 discriminator published only that archive, but the bounded candidate,
candidate-rerun, and restored-control windows did not produce stable
same-cursor TFTP fetch evidence tied to the staged tree. Fresh serial showed
Raspberry Pi firmware/RP1 output only, with no TALOS: rust_entry text and no
rpi5-rp1-post-handoff-marker-reset marker.

The late first-run TFTP replay is retained as capture-timing evidence, not as
candidate identity proof, because it appeared after the stable zero-event
sample and after status had already returned to the restored tree. The closeout
therefore cannot accept marker visibility, reset side-effect evidence,
marker-path hang/fault evidence, or RP1 UART0 FR-read readiness.

## Findings And Disposition

- fixed: the source/static core retained a narrow no-RP1-MMIO marker/reset
  candidate, so the intended hardware question was bounded to post-handoff
  serial visibility or reset-side effect.
- fixed: the hardware discriminator retained candidate publication identity:
  archive SHA-256
  73a74db1d08d89a3aa371d5329bc6158553bef172a82f0b479598bc29f15acaa,
  published tree 37995c483190ddcfaef70c9cf5be04244f75c4fcd9cf25fdd90f941ccc48c4f2,
  effective kernel kernel_2712.img, and 51,736-byte staged kernel files.
- fixed: restore hygiene was retained across first run, candidate rerun, and
  restored-control run; each post-restore status returned tree hash
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- deferred: visible post-handoff serial observability remains unaccepted
  because no fresh serial window showed TALOS: rust_entry or the unique
  marker/reset text.
- deferred: reset-side-effect evidence remains unaccepted because the stable
  same-cursor samples did not prove repeated candidate-tied boot/fetch
  sequences before restore.
- deferred: marker-path hangs-or-faults-before-reset remains unaccepted because
  candidate-tied fetch evidence was not stable enough to separate marker-path
  failure from capture/staging behavior.
- deferred: RP1 UART0 FR mapped/read-value, unmapped/trap, firmware-state, or
  diagnostic readiness remains blocked until a later task first resolves the
  staging/capture boundary or otherwise provides decisive post-handoff
  observability.
- not-an-issue: the closeout does not need a new hardware run; its scope is
  static reconciliation of already committed core and Pi 5 discriminator
  evidence.
- not-an-issue: GPIO, interrupts, DMA/cache, storage, generated-root,
  networking, SSH, broader PCIe, Milestone 11.2, and phase transition behavior
  are not accepted by this boundary.

## Evidence

- Static evidence inspection:
  tasks/evidence/2026-06-06-phase11-rp1-post-handoff-marker-reset-closeout/static-evidence-inspection.md.
- Evidence map:
  tasks/evidence/2026-06-06-phase11-rp1-post-handoff-marker-reset-closeout/evidence-map.json.
- Source/static core task:
  tasks/2026-06-06-phase11-rp1-post-handoff-marker-reset-core.md.
- Pi 5 discriminator task:
  tasks/2026-06-06-phase11-rp1-post-handoff-marker-reset-pi5-discriminator.md.
- Core commit: a2d4c6add9e2ec7e91ba9dcd82c549c82ea01807.
- Pi 5 discriminator commit: 0d38d65a19fc0dd31836b5427e30215d9213c057.

## Validation

- static evidence inspection: completed.
- git diff --check: passed.
- mdbook build: passed because docs/src files changed.
- git diff --cached --check before commit: passed.

## Result

Accepted as staging-capture-blocked.

This closeout accepts the source/static no-RP1-MMIO marker/reset candidate and
the recorded publication/restore evidence only. It does not accept visible
post-handoff serial observability, reset side-effect evidence, marker-path
hang/fault evidence, RP1 UART0 FR-read readiness, RP1 mapped/read-value,
RP1 unmapped/trap, firmware-state behavior, GPIO ownership, interrupts,
DMA/cache, storage, generated-root, networking, SSH, broader PCIe, Milestone
11.2, or a phase transition.

No later task is mechanically unblocked in the existing queue. Supervisor
planning is required for the next bounded staging/capture or post-handoff
observability discriminator before returning to the RP1 UART0 flag-register
read.
