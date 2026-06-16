# Phase 12.1 RP1 Ethernet Serial Freshness Contract

Task:
phase12-rp1-ethernet-serial-freshness-contract-20260616.

Status: accepted

Classification:
serial-freshness-contract-cursor-nonce-replaces-empty-drain-hard-gate.

Evidence level: static source/doc/task evidence inspection, accepted
BootInfo/report-path proof and closeout evidence inspection, JSON evidence
validation, docs build, and diff checks. No Pi 5 hardware run, boot archive
publication, lab mutation, hardwareTestLock acquisition, power-cycle, serial
write, TFTP capture mutation, Ethernet/MMIO/MDIO/MAN/register access,
GPIO32/PHY reset, BMCR/autoneg write, packet I/O, networking, SSH, Phase 12.2,
or phase transition was performed.

## Goal

Define the smallest source-backed serial freshness contract that explains or
replaces the failed serial-drain-not-empty-before-power invariant before any
Ethernet/register retry.

## Sources Inspected

- docs/src/project/lab-controller.md serial endpoint contract:
  /serial/peek returns retained tail plus cursor and does not consume the log;
  /serial/read consumes newly available device bytes and appends them to the
  retained log; /serial/observe consumes newly available bytes and returns log
  bytes after the supplied cursor; saturated retained-log cursors require the
  helper direct-read fallback and cannot be interpreted as no output.
- scripts/rpi5-observe-serial-window.sh records
  deadline-loop-accumulated-from-fresh-cursor for cursor-based observe and
  deadline-loop-direct-read-after-saturated-cursor for saturated-cursor
  fallback. It records cursor start/end, capture mode, response bytes, required
  marker counts, and optional nonce-token counts.
- scripts/rpi5-capture-invariant-proof-bundle.sh records a pre-power
  /serial/read drain, saves the final drain cursor, observes serial from that
  cursor, records stable same-cursor TFTP delta before restore, records final
  pre-restore identity, restores the saved snapshot, and currently rejects
  decisive classification when the drain did not reach an empty read.
- tasks/2026-06-16-phase12-rp1-ethernet-bootinfo-report-serial-visibility-pi5-proof.md
  and its task-owned JSON evidence showed final control/candidate selected-tree
  identity, matching TFTP byte serves, marker output, final identity, and
  restore proof, but both runs were rejected by
  serial-drain-not-empty-before-power.
- tasks/2026-06-16-phase12-rp1-ethernet-bootinfo-report-serial-visibility-closeout.md
  closed the BootInfo/report-path frontier and required a distinct serial
  drain/backlog freshness discriminator before any same-shaped retry.

## Findings

- fixed: The freshness invariant is now named explicitly as
  cursor-nonce-post-power-freshness-v1. A serial marker is fresh only when a
  run-unique marker/nonce selected for this hardware attempt is absent from the
  pre-power retained sample, then appears in the bounded post-power capture
  after the saved pre-power cursor or in the saturated-cursor direct-read
  fallback for that same attempt.
- fixed: Empty pre-power drain is no longer the only hard gate for future
  marker-only transport proofs. It remains recorded evidence and a rejection
  class when the cursor/nonce freshness contract is not satisfied.
- fixed: The required hardware-proof evidence fields are explicit:
  pre-power peek cursor, pre-power retained text/nonce absence, drain attempts,
  terminal drain state, final drain cursor, post-power observe cursor/input
  cursor, cursor saturation/capture mode, observed byte ranges, marker and nonce
  counts after the saved cursor, TFTP cursor/delta, selected-tree identity,
  final pre-restore identity, restore proof, and helper classification.
- fixed: The rejection classes are explicit: stale backlog, cursor mismatch,
  missing marker, nonce not unique or visible before power, selected-tree/TFTP
  mismatch, final identity mismatch, restore failure, saturated direct-read with
  no nonce proof, and inconclusive capture.
- fixed: The only mechanically unblocked follow-up selected by this task is
  phase12-rp1-ethernet-serial-freshness-guard-core-20260616.
- rejected: This task does not accept the retained BootInfo/report-path marker
  counts as decisive hardware proof. They remain useful evidence that motivated
  the contract.
- rejected: BCM54213PE register values, link readiness, Ethernet readiness,
  GPIO32/PHY reset ownership, BMCR/autoneg, Broadcom shadow/MMD/aux access,
  interrupt ownership, packet I/O, networking, SSH, Phase 12.2, and phase
  transition remain rejected.
- removed: No task-owned source, helper, docs, or evidence files were removed.
- not-an-issue: A non-empty pre-power drain can coexist with fresh post-power
  proof when the post-power marker carries a run-unique nonce that is absent
  before power and is tied to selected-tree/TFTP/final-identity/restore
  evidence for the same attempt.

## Contract

Contract id:
phase12-rp1-ethernet-serial-freshness-contract-v1.

Freshness invariant:
cursor-nonce-post-power-freshness-v1.

A later hardware proof may treat serial marker output as post-power fresh only
if all of these hold:

1. The proof stages a run-unique marker or nonce for the specific selected
   boot tree and records that nonce in static/archive review before publication.
2. Immediately before power-cycle, the proof records a /serial/peek cursor and
   retained sample and proves the run-unique marker/nonce is absent from that
   pre-power retained sample.
3. The proof records bounded pre-power /serial/read drain attempts and the
   terminal drain state. empty-read-before-power remains accepted freshness
   support; bounded-drain-exhausted-before-power is not decisive by itself.
4. The proof records the post-power serial capture mode. For non-saturated
   cursors, /serial/observe must report cursor_start equal to the saved
   pre-power cursor and cursor_end >= cursor_start. For saturated cursors, the
   helper must record deadline-loop-direct-read-after-saturated-cursor,
   start_cursor_saturated=true, and positive response bytes.
5. The run-unique marker/nonce appears in the bounded post-power capture and
   its counted occurrences are computed from bytes associated with the saved
   cursor or the saturated direct-read fallback for that same attempt.
6. Stable same-cursor TFTP delta before restore proves the selected kernel path
   and expected bytes were served for this power cycle.
7. Final pre-restore boot identity still matches the selected tree and expected
   kernel, then restore proof records the baseline tree after the run.

Empty-drain decision:
cursor/nonce freshness is the stricter replacement for the old hard empty-drain
gate. Empty drain remains a strong positive condition, but a non-empty drain is
only a hard rejection when the proof lacks the run-unique pre-power absence and
post-power cursor/nonce evidence above.

## Rejection Classes

- stale-backlog: the run-unique marker/nonce is present before power, or the
  post-power marker is not bound to a fresh cursor/direct-read attempt.
- cursor-mismatch: post-power observe does not start from the saved pre-power
  cursor, omits cursor fields, or reports a cursor window inconsistent with the
  helper contract.
- missing-marker: the required marker or nonce does not appear in the bounded
  post-power capture.
- nonce-not-unique: the marker/nonce was reused across attempts without
  pre-power absence proof.
- selected-tree-tftp-mismatch: selected boot tree, expected kernel path/bytes,
  or stable same-cursor TFTP delta does not match the staged attempt.
- final-identity-mismatch: final pre-restore identity does not match the
  selected tree and expected kernel.
- restore-failure: restore proof is missing or does not return to the expected
  baseline tree.
- saturated-direct-read-without-nonce-proof: saturated direct-read output is
  present but lacks a run-unique nonce absent from pre-power evidence.
- inconclusive-capture: endpoint errors, truncation, missing evidence fields, or
  mutually inconsistent helper outputs prevent a decision.

## Next Boundary

The only selected follow-up is
phase12-rp1-ethernet-serial-freshness-guard-core-20260616. That task should
remain local/static and update the capture helper/validator surface so later
hardware evidence can emit and check the contract fields above.

No hardware, register-read retry, GPIO32 event clear/reset recovery, BMCR
write, Broadcom shadow/MMD/aux access, interrupt ownership, PHY/MAC
configuration, packet I/O, networking, SSH, Phase 12.2, or phase transition is
authorized from this contract.

## Evidence

- classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-serial-freshness-contract/classification.json.
- evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-serial-freshness-contract/evidence-map.json.
- lab-controller serial endpoint docs: docs/src/project/lab-controller.md.
- serial window helper: scripts/rpi5-observe-serial-window.sh.
- capture-invariant bundle helper: scripts/rpi5-capture-invariant-proof-bundle.sh.
- BootInfo/report-path Pi 5 proof evidence:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bootinfo-report-serial-visibility-pi5-proof/classification.json.
- BootInfo/report-path closeout evidence:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bootinfo-report-serial-visibility-closeout/classification.json.

## Validation

- static source/doc/task evidence inspection: lab-controller serial endpoint
  docs, serial window helper, capture-invariant bundle helper, accepted
  BootInfo/report-path proof and closeout task/evidence.
- JSON validation: jq empty on task-owned classification/evidence-map JSON.
- diff check: git diff --check.
- docs validation: mdbook build because docs/src files were touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Findings list includes dispositions: satisfied.
- Contract names freshness invariant, required evidence fields, and rejection
  classes: satisfied.
- Contract selects the next local/static guard-core task as the only
  mechanically unblocked follow-up: satisfied.
- Forbidden Ethernet/register/networking/phase-transition claims remain
  explicit: satisfied.

## Next Action

Mechanically promote
phase12-rp1-ethernet-serial-freshness-guard-core-20260616 on the next worker
wake if dependencies remain satisfied, hardwareTestLock remains
unlocked/restored, supervisor intervention remains inactive, and the repository
has no conflicting uncommitted changes.
