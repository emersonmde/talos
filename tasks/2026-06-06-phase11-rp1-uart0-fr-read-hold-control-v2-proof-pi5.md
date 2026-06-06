# Phase 11 RP1 UART0 FR Read Hold-Control V2 Proof Pi 5

Task id: phase11-rp1-uart0-fr-read-hold-control-v2-proof-pi5-20260606

Status: accepted

## Goal

Run one serialized Pi 5 RP1 UART0 FR-read hold-control proof after the
capture-transaction v2 closeout accepted the proof chain as ready.

## Scope

- Acquired hardwareTestLock for the serialized RP1 UART0 FR-read
  hold-control v2 proof.
- Checked the accepted hold-control candidate archive SHA-256 before
  publication.
- Published only the accepted hold-control RP1 UART0 FR-read candidate
  archive.
- Captured candidate identity, pre-power serial drain, fresh serial/TFTP
  cursor evidence, stable pre-restore TFTP, final pre-restore identity,
  restore, and post-restore identity through the v2 proof contract.
- Performed required triage after the first candidate run was rejected by the
  v2 pre-power drain rule: rebooted to the restored known-good tree, ran a
  known-good v2 control, then attempted exactly one candidate rerun.
- Restored the pre-run boot tree before hardware-lock release.

## Non-Goals

No new RP1 constants, GPIO, interrupts, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe, Milestone 11.2, phase transition, or workaround
stack.

## Classification

candidate-fetch-without-control-marker.

The accepted core archive SHA-256 matched
e9ab45b6dd15e4e80395302a116fb8aa751d699c5b679e5b9cee22077059a9b2 before
publication. The selected candidate tree was
ae324bd791d7df59a0a8eabc74c936b1bd68ba6c2c9b645dcdc19f561e4e80c0 with
effective kernel kernel_2712.img and the expected 46,320-byte
da591740/kernel_2712.img.

The first candidate run retained candidate-tied TFTP and final pre-restore
identity, but the v2 identity-join checker rejected decisive RP1 classification
because the pre-power /serial/read drain did not reach an empty response before
the saturated direct-read window. That serial window retained 1,064,894 bytes
and 27,178 occurrences of TALOS: fr-hold-control-post-read-loop, but the
non-empty pre-power drain made the serial window non-decisive under the v2
contract.

Triage proved the proof chain itself was healthy. After rebooting to the
restored known-good tree, the known-good control passed v2 identity join with
an empty pre-power drain, two 104,136-byte expected TFTP fetches, final
pre-restore selected-tree identity, restore proof, and 6,671 serial bytes
containing rpi5-production-timer-preemption: PASS.

The single candidate rerun then passed v2 identity join for the selected
candidate tree. It retained an empty pre-power drain, stable same-cursor TFTP
with two 46,320-byte da591740/kernel_2712.img fetches, final pre-restore
selected-tree identity, and restore proof. The saturated direct-read window
retained 1,064,457 serial bytes and 27,177 occurrences of
TALOS: fr-hold-control-post-read-loop, but it did not retain the contracted
rpi5-rp1-uart0-fr-read read-value/classification line, the pre-read control
marker, the post-read terminal marker, or trap/panic text. The accepted
classification is therefore limited to candidate-fetch-without-control-marker.
The post-read-loop tail is retained as risk evidence but is not upgraded to
mapped/read-value because the contracted value/classification output was not
retained in the v2-joined serial window.

Accepted claims are limited to candidate publication/fetch evidence, v2
identity join, visible post-read-loop tail output without the contracted
control/read-result markers, restore hygiene, and the
candidate-fetch-without-control-marker classification. RP1 UART0 FR
mapped/read-value behavior, bus-fault/trap behavior,
pre-read-control-visible-without-read-result, firmware-state behavior, GPIO,
interrupts, DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
Milestone 11.2, and phase transition remain unaccepted.

## Findings And Disposition

- fixed: acquired hardwareTestLock before publication and restored the lab
  before release.
- fixed: checked the selected archive SHA-256 against accepted core evidence
  before publication.
- fixed: retained publication identity showing effective kernel_2712.img and
  the expected 46,320-byte selected kernel.
- fixed: retained the first candidate run as blocker evidence for the v2
  pre-power serial-drain rule.
- fixed: ran a known-good v2 control and proved the repaired proof path still
  joins selected tree, TFTP, serial, final identity, and restore evidence.
- fixed: attempted exactly one candidate rerun after the known-good control and
  retained a decisive v2 identity join for the selected RP1 candidate.
- deferred: a mapped/read-value classification still needs retained contracted
  read-value/classification output, not just the post-read-loop tail.
- removed: no RP1 mapped/read-value or trap claim is inferred from serial text
  that lacks the contracted value/classification line or trap evidence.
- not-an-issue: the first candidate run's non-empty pre-power drain is retained
  as rejected evidence, not discarded, because it explains why the rerun was
  necessary.

## Evidence

- Classification:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-v2-proof-pi5/classification.json.
- Evidence map:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-v2-proof-pi5/evidence-map.json.
- First candidate run:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-v2-proof-pi5/candidate-run/.
- Known-good control:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-v2-proof-pi5/known-good-control-run/.
- Candidate rerun:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-v2-proof-pi5/candidate-rerun/.

## Validation

- serialized Pi 5 hardware run through lab-controller endpoints under
  hardwareTestLock: completed.
- static archive identity check: passed against accepted core SHA-256.
- capture-transaction v2 proof summary and identity checker output: first
  candidate rejected by pre-power serial drain; known-good control passed;
  candidate rerun passed.
- stable same-cursor TFTP evidence before restore: passed for the known-good
  control and candidate rerun.
- restore proof before hardware-lock release: passed; post-restore tree hash
  returned to
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- git diff --check: passed.
- mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted as candidate-fetch-without-control-marker. The queued v2 closeout
should reconcile the post-read-loop-tail risk evidence without broadening into
new RP1 work.
