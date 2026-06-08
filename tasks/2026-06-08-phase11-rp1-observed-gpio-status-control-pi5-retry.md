# Phase 11 RP1 Observed GPIO Status Control Pi 5 Retry

Task id: phase11-rp1-observed-gpio-status-control-pi5-retry-20260608

Status: accepted

Classification: no-mmio-observed-gpio-status-control-visible

## Goal

Rerun the no-MMIO/no-RP1/no-GIC observed GPIO status control on Pi 5 after the
serial-drain freshness blocker repair, then classify whether the control proof
is accepted or still blocked.

## Scope

- Acquired hardwareTestLock for the serialized Pi 5 work.
- Published only
  target/talos-rpi5-rp1-observed-gpio-status-no-mmio-control-core.tar.gz for
  the candidate control attempts.
- Used the repaired bounded pre-power serial-drain procedure with
  --serial-drain-attempts 96 --serial-drain-read-timeout 1
  --serial-drain-settle-ms 100 --serial-drain-max-bytes 65536.
- Captured candidate identity, fresh serial/freshness discriminator evidence,
  TFTP deltas, serial markers, final selected-tree identity, restore evidence,
  and known-good control triage after the first candidate was inconclusive.

## Non-Goals

No real observed-aperture GPIO MMIO read, endpoint ownership claim, broad RP1
mapping claim, endpoint config retry, BAR discovery or programming, bridge
setup writes, PERST/link-control changes, GPIO/pad/clock/reset writes,
interrupt enablement or delivery, GIC acknowledgement, DMA/cache, storage,
generated-root, networking, SSH, Milestone 11.3, phase transition, runtime
source changes, or source-contract expansion.

## Classification

Accepted as no-mmio-observed-gpio-status-control-visible.

The first candidate retry selected boot tree
133f2a9b4a4c5c21b206d8f9eb8eba4ffe41f787a69b33bced0ea164a8cf83ab and had an
empty pre-power serial drain after one attempt. It retained 41
TALOS: rp1-observed-gpio-status-control markers, but the identity join rejected
the run because the TFTP delta and final pre-restore identity showed the
104,136-byte known-good tree, not the selected 48,952-byte candidate tree.

The required known-good production-timer control then passed the repaired
freshness gate and pi5-capture-transaction-v2 with no rejection reasons. It
selected boot tree 407d10f6ed4457e89f9023f769c00920a4ebbe0f42ca65b0165b8db014140697,
retained an empty pre-power drain after two attempts, captured two 104,136-byte
da591740/kernel_2712.img TFTP fetches, retained the production-timer PASS
marker, and restored to the baseline tree.

The unchanged candidate rerun then passed pi5-capture-transaction-v2 with no
rejection reasons. It selected boot tree
133f2a9b4a4c5c21b206d8f9eb8eba4ffe41f787a69b33bced0ea164a8cf83ab, retained an
empty pre-power drain after one attempt, captured two 48,952-byte
da591740/kernel_2712.img TFTP fetches, retained 41 task-owned control markers,
kept final pre-restore identity on the selected tree, and restored the lab to
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

This accepts only the paired no-MMIO control proof. It does not accept the real
observed-aperture GPIO14 STATUS/CTRL read, GPIO ownership, event generation,
interrupt pending generation, interrupt delivery, endpoint ownership, broad RP1
mapping, pad/RIO/clock/reset ownership, DMA/cache, networking, SSH,
Milestone 11.3, or phase transition.

## Findings And Disposition

- fixed: acquired and released hardwareTestLock around the serialized Pi 5
  work.
- fixed: retained static archive identity for the accepted no-MMIO control
  archive.
- fixed: the initial candidate retry proved the repaired serial-drain
  freshness discriminator reached empty-read-before-power.
- fixed: the initial candidate retry was rejected as capture-staging-blocked
  because TFTP/final identity did not match the selected candidate tree.
- fixed: retained known-good production-timer control triage after the
  inconclusive candidate retry.
- fixed: the unchanged candidate rerun passed the repaired freshness gate,
  stable TFTP evidence, final selected-tree identity, and restore proof.
- not-an-issue: the first retry's visible no-MMIO markers were not accepted
  until the TFTP/final identity mismatch was eliminated by the unchanged rerun.
- not-an-issue: no real RP1 aperture, endpoint, interrupt, GPIO, clock/reset,
  or DMA behavior is inferred from this no-MMIO control proof.

No findings were removed or deferred in this task.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-control-pi5-retry/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-control-pi5-retry/classification.json.
- Initial candidate retry:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-control-pi5-retry/control-run/.
- Known-good control after inconclusive candidate:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-control-pi5-retry/known-good-control-after-inconclusive/.
- Accepted unchanged candidate rerun:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-control-pi5-retry/candidate-rerun-after-known-good/.

## Validation

- static archive identity check: passed against the accepted no-MMIO control
  archive.
- lab-controller API candidate identity check before power-cycle: passed for
  candidate retry, known-good control, and unchanged candidate rerun.
- serial hardware boot/output: accepted candidate rerun retained 41 task-owned
  control markers.
- repaired serial freshness discriminator: accepted candidate rerun reached
  empty-read-before-power before a saturated direct-read serial window.
- stable same-cursor TFTP evidence before restore: accepted candidate rerun
  retained two 48,952-byte candidate fetches.
- pi5-capture-transaction-v2 identity join: initial candidate rejected for
  TFTP/final identity mismatch; known-good control and unchanged candidate
  rerun passed with no rejection reasons.
- restore proof: passed; all retained runs restored to the pre-run baseline
  tree.
- git diff --check: passed.
- mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted as a no-MMIO observed GPIO status control proof. The real observed
GPIO status Pi 5 task is mechanically unblocked if hardwareTestLock remains
unlocked/restored.
