# Phase 11 RP1 UART0 FR Read Hold-Control Repaired-Proof Pi 5

Task id: phase11-rp1-uart0-fr-read-hold-control-repaired-proof-pi5-20260606

Status: accepted

## Goal

Run one serialized Pi 5 RP1 UART0 FR-read hold-control candidate proof under
the accepted repaired proof-chain contract, only after the proof-chain closeout
accepted candidate-rerun readiness.

## Scope

- Acquired hardwareTestLock for the repaired-proof Pi 5 candidate run.
- Verified the accepted hold-control candidate archive SHA-256 before
  publication.
- Published only the accepted hold-control RP1 UART0 FR-read candidate archive.
- Captured candidate identity, fresh serial cursor, TFTP cursor, serial window,
  stable TFTP delta, final pre-restore identity, restore, and post-restore
  identity through the repaired proof-chain bundle.
- Performed the required triage after the candidate run could not join selected
  candidate TFTP identity to the observed serial bytes: known-good control,
  capture-helper trace, and one same-shaped candidate rerun attempt.
- Restored the pre-run boot tree before releasing hardwareTestLock.

## Classification

capture-staging-blocked.

The accepted core archive SHA-256 matched
`e9ab45b6dd15e4e80395302a116fb8aa751d699c5b679e5b9cee22077059a9b2` before
publication. Candidate publication reported lab tree
`ae324bd791d7df59a0a8eabc74c936b1bd68ba6c2c9b645dcdc19f561e4e80c0`,
effective `kernel_2712.img`, and the expected 46,320-byte
`da591740/kernel_2712.img`.

The main candidate run retained a direct-read serial window with 973,431 bytes
and 24,796 `TALOS: fr-hold-control-post-read-loop` occurrences. That serial
text is not accepted as RP1 mapped/read-value evidence because the repaired
`pi5-proof-identity-join-v1` contract rejected the run. Stable same-cursor
TFTP before restore retained two `da591740/kernel_2712.img` fetches, but both
were 104,136-byte restored known-good fetches rather than the selected
46,320-byte candidate fetches. The final pre-restore identity likewise matched
the restored known-good tree. Rejection reasons were:
`tftp-expected-fetch-byte-mismatch`, `final-pre-restore-tree-mismatch`,
`final-pre-restore-selected-tree-mismatch`, and
`final-pre-restore-expected-fetch-byte-mismatch`.

Required triage confirmed the repaired proof chain itself remained capable of
joining evidence: the known-good control passed with tree
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`, two
104,136-byte expected TFTP fetches, and a 7,075-byte direct-read serial window
containing `rpi5-production-timer-preemption: PASS`. One candidate rerun was
attempted and then stopped; recovery TFTP after restore again retained two
104,136-byte known-good fetches and zero 46,320-byte candidate fetches.

Accepted claims are limited to capture-staging-blocked. RP1 UART0 FR
mapped/read-value behavior, bus-fault/trap behavior,
pre-read-control-visible-without-read-result, candidate-fetch-without-control-
marker, firmware-state behavior, GPIO, interrupts, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe, Milestone 11.2, and phase
transition remain unaccepted.

## Findings And Disposition

- fixed: acquired and released hardwareTestLock around the serialized
  candidate proof and triage.
- fixed: checked the selected archive SHA-256 against accepted core evidence
  before publication.
- fixed: retained publication identity showing effective `kernel_2712.img`
  and the expected 46,320-byte selected kernel in the staged boot tree.
- fixed: retained the main candidate serial window and stable TFTP evidence
  before restore.
- fixed: applied the repaired `pi5-proof-identity-join-v1` gate and rejected
  the serial window as non-decisive because TFTP/final identity matched
  restored known-good bytes, not selected-candidate bytes.
- fixed: performed required known-good control triage and proved the repaired
  identity-join checker can still pass on known-good evidence.
- fixed: attempted one candidate rerun and stopped same-shaped repetition after
  recovery evidence again lacked candidate-byte TFTP identity.
- deferred: a decisive RP1 FR-read hardware classification still needs a lab
  path that keeps the selected candidate staged through the capture window and
  ties 46,320-byte candidate fetch evidence to serial marker/read/trap output.
- removed: no RP1 mapped/read-value or trap claim is inferred from post-read
  loop serial text that failed the repaired identity-join contract.
- not-an-issue: known-good control marker visibility is proof-chain health
  evidence only; it does not accept candidate or RP1 behavior.

## Evidence

- Classification:
  `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-repaired-proof-pi5/classification.json`.
- Evidence map:
  `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-repaired-proof-pi5/evidence-map.json`.
- Main candidate run:
  `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-repaired-proof-pi5/candidate-run/`.
- Known-good control:
  `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-repaired-proof-pi5/known-good-control-run/`.
- Candidate rerun:
  `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-repaired-proof-pi5/candidate-rerun/`.
- Observe-helper trace:
  `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-repaired-proof-pi5/observe-helper-trace.stderr`.

## Validation

- serialized Pi 5 hardware run through lab-controller endpoints under
  hardwareTestLock: completed with capture-staging-blocked.
- static archive identity check: passed against accepted core SHA-256.
- repaired proof-chain identity summary: failed the main candidate run with
  identity-join mismatch as blocker evidence.
- known-good control: passed the repaired proof-chain checker.
- candidate rerun: attempted once and stopped same-shaped repetition.
- stable same-cursor TFTP before restore: retained for the main run, but not
  candidate-tied.
- restore proof before hardware-lock release: passed; post-restore tree hash
  returned to
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.
- git diff --check: passed.
- mdbook build: passed because docs/src files changed.
- git diff --cached --check before commit: passed.

## Result

Accepted as capture-staging-blocked. Same-shaped RP1 UART0 FR-read hardware
reruns should stop until supervisor planning defines a different discriminator
for why the selected candidate tree is not the identity joined to serial/TFTP
evidence through the capture window.
