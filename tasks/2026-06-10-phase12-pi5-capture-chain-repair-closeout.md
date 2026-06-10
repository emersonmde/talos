# Phase 12 Pi 5 Capture Chain Repair Closeout

Task id: phase12-pi5-capture-chain-repair-closeout-20260610

Status: accepted

Classification: pi5-capture-chain-repair-closeout-accepted

## Goal

Close out the local/static pi5-capture-chain-v4 repair and decide whether the
guarded GEM MID decode-discriminator v2 Pi 5 proof is mechanically unblocked.

## Scope

- Consumed accepted repair core
  phase12-pi5-capture-chain-repair-core-20260610 at commit
  9eeadb6a37099096336ddfc8f2a665fe98d034c4.
- Reconciled the repaired helper/checker behavior against the accepted GEM MID
  decode-discriminator capture-chain blocker
  phase12-rp1-ethernet-gem-mid-decode-discriminator-pi5-proof-20260610.
- Did not run Pi 5 hardware, publish boot archives, acquire hardwareTestLock,
  change Ethernet behavior, program RP1 MMIO/DMA, create descriptor rings,
  perform packet I/O, add networking/sockets/SSH, start Phase 12.2, or create a
  phase transition.

## Findings And Disposition

- fixed: pi5-capture-chain-v4 now retains GET / endpoint failure separately
  from /boot/files selected-tree identity, so the lab-controller GET / 404 from
  the accepted blocker no longer prevents identity-gated replay by itself.
- fixed: selected-tree identity, expected TFTP fetch bytes, final pre-restore
  identity, and restore identity are all explicit gates before a candidate or
  control run can support decisive RP1 hardware classification.
- fixed: saturated direct serial capture is now first-class evidence when it
  retains the run-unique marker, nonce occurrence counts, response byte count,
  cursor fields, capture mode, observe contract, and marker excerpt.
- fixed: paired control marker retention is an explicit v4 requirement; a
  missing control marker remains a deterministic capture-staging blocker.
- fixed: retained fixture replay confirms the checker accepts the retained
  positive candidate marker shape only when identity and freshness gates are
  satisfied, and rejects missing identity, missing expected TFTP, missing final
  identity, missing marker, stale nonce, and missing control marker cases.
- deferred: the translated 0x1f RP1 window result is still a hardware question;
  the repair only fixes the evidence path needed for a new proof attempt.
- not-an-issue: Ethernet diagnostic semantics and RP1 runtime behavior were not
  changed by the repair.

No findings were removed.

## Repaired Capture-Chain Gates

- endpoint identity: GET / may be retained as unusable evidence, with
  /boot/files selected-tree identity used as the authoritative fallback.
- staged identity: selected tree hash, effective kernel, expected fetch path,
  and expected fetch byte count must be present before the run.
- TFTP identity: the expected fetch path must appear in the stable TFTP delta
  and all observed served byte counts must match the expected bytes.
- final identity: final pre-restore tree, kernel, and expected fetch bytes must
  still match the selected candidate/control archive before restore.
- serial freshness: the run-unique capture nonce must be absent before power and
  present after power, or an explicitly empty pre-power drain must be retained.
- serial retention: direct serial read after a saturated cursor is acceptable
  only with retained marker occurrence count, nonce count, response bytes,
  cursor fields, capture mode, observe contract, and marker excerpt.
- paired control: the control run must retain its no-MMIO/no-Ethernet marker;
  missing control marker evidence blocks decisive classification.

## Retained Risks

- The 0x1f translated RP1 window sentinel remains unproven until a repaired
  Pi 5 proof joins candidate/control identity, TFTP, serial, and restore
  evidence.
- PCIe/RP1 bridge or address-window enablement remains unaccepted.
- Ethernet clock/reset, PHY/MDIO ownership, interrupts, descriptor rings,
  packet I/O, networking, sockets, SSH, and Phase 12.2 remain out of scope.
- A future hardware run can still classify as inconclusive or as another
  precise blocker if the repaired capture chain does not retain all required
  gates.

## Decision

Selected: phase12-rp1-ethernet-gem-mid-decode-discriminator-v2-pi5-proof-20260610
is mechanically unblocked for the next worker wake, provided hardwareTestLock
is still unlocked before acquisition.

The only changed acceptance slice is the repaired capture-chain evidence path
over the already accepted candidate/control archive shape from the GEM MID
decode-discriminator proof. The v2 proof must not broaden into Ethernet driver
work or runtime RP1 ownership.

## Required V2 Proof Gates

- acquire hardwareTestLock before boot archive publication or Pi 5 hardware
  interaction;
- capture candidate and control archive identity, artifact digests, fresh
  serial cursor/output, TFTP delta, final pre-restore identity, restore proof,
  classification JSON, and evidence map;
- use pi5-capture-chain-v4 helper/checker output for candidate and control
  evidence;
- run the standard Pi 5 inconclusive-run triage order before changing code after
  an inconclusive run: candidate identity via lab API GET / or /boot/files
  fallback, fresh serial cursor, TFTP delta, known-good control when
  appropriate, then candidate rerun;
- allowed classifications are
  observed-rp1-positive-control-gem-mid-0x1f-window-sentinel,
  observed-rp1-positive-control-and-gem-mid-visible,
  observed-rp1-positive-control-sentinel, staging/build-blocker,
  inconclusive-capture, or another precise blocker supported by evidence.

## Evidence

- Classification:
  tasks/evidence/2026-06-10-phase12-pi5-capture-chain-repair-closeout/classification.json.
- Evidence map:
  tasks/evidence/2026-06-10-phase12-pi5-capture-chain-repair-closeout/evidence-map.json.
- Fixture rerun:
  tasks/evidence/2026-06-10-phase12-pi5-capture-chain-repair-closeout/retained-fixture-rerun.json.

## Validation

- static inspection: accepted repair-core task record, classification,
  evidence map, retained fixture replay, touched helper/checker scripts, and
  accepted GEM MID decode-discriminator blocker reviewed.
- task-owned replay/fixture command: passed; retained-fixture-rerun.json shows
  seven fixture cases and passed=true.
- jq empty on task-owned JSON evidence: passed.
- git diff --check: passed.
- git diff --cached --check before commit: passed.

No docs/src files, Rust source files, or shell scripts were touched by this
closeout, so mdbook, cargo, and bash -n gates were not required.

## Next Action

Mechanically promote
phase12-rp1-ethernet-gem-mid-decode-discriminator-v2-pi5-proof-20260610 on the
next worker wake only as a serialized repaired capture-chain Pi 5 proof. It
must acquire hardwareTestLock before boot archive publication or Pi 5 hardware
interaction, retain candidate/control archive identity, artifact digests, fresh
serial cursor/output, TFTP delta, final pre-restore identity, restore proof,
classification JSON, and evidence map, and restore/release the hardware lock
before acceptance. Do not implement Ethernet behavior, program RP1 MMIO/DMA,
create descriptor rings, claim interrupts/clock/reset/PHY ownership, perform
packet I/O, add networking/sockets/SSH, start Phase 12.2, or claim a phase
transition.
