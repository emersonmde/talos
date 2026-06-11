# Phase 12 RP1 Ethernet MDIO MPE Enable Guard Closeout

Task id: phase12-rp1-ethernet-mdio-mpe-enable-guard-closeout-20260611

Status: accepted

Classification:
rp1-ethernet-mdio-mpe-enable-guard-static-frontier-closed

Evidence level: static inspection of accepted source contract, guard core task
record, guard core classification/evidence JSON, focused tests, touched source,
project docs, and git history. No Pi 5 hardware run, boot archive publication,
hardwareTestLock acquisition, runtime RP1 MMIO write, NCR write, MAN write,
PHY-ID read, PHY reset or GPIO32 action, Ethernet driver behavior, DMA,
interrupt, packet I/O, networking, sockets, SSH, Phase 12.2, or phase
transition was performed.

## Goal

Close the local/static NCR.MPE enable guard frontier and decide whether the
serialized Pi 5 set/readback/restore proof is mechanically objective.

## Scope Performed

- Reconciled the accepted source contract
  `phase12-rp1-ethernet-mdio-mpe-enable-source-contract-v1` with the accepted
  guard report contract
  `phase12-rp1-ethernet-mdio-mpe-enable-guard-report-contract-v1`.
- Confirmed the candidate guard preserves the exact source-backed MACB/GEM NCR
  target, MPE bit, mask, write rule, restore invariant, allowed future proof
  classifications, rejected claims, retained risks, and source evidence.
- Confirmed the paired no-MDIO/no-Ethernet control uses the same report path
  while constructing no NCR/MPE target and no write intent.
- Closed same-shaped local/static guard retries for this candidate/control pair.
- Selected only the queued serialized Pi 5 NCR.MPE set/readback/restore proof
  as the next mechanically objective task.

## Findings

- fixed: accepted guard evidence carries the exact source-backed target facts
  for the future proof: MACB/GEM NCR observed target 0x1c00100000, source
  target 0xc040100000, offset 0x0000, MPE bit 4, and mask 0x00000010.
- fixed: accepted guard evidence preserves the write/readback/restore boundary:
  pre-read NCR, write `pre_raw | 0x00000010`, read back MPE set state,
  restore-write exact `pre_raw`, and restore-read exact `pre_raw`.
- fixed: paired control withholds NCR/MPE target facts and candidate write
  intent while retaining the same report path and explicit no-MDIO/no-Ethernet
  classification.
- fixed: validators and focused tests reject missing source contract, target
  drift, control target leakage, runtime NCR write execution, MAN writes,
  PHY-ID reads, broad MDIO/PHY ownership, PHY reset/GPIO32 ownership, Ethernet
  readiness, broad MMIO readiness, DMA/descriptors, interrupts, packet I/O,
  networking, sockets, SSH, Phase 12.2, and phase-transition claims.
- deferred: serialized Pi 5 NCR.MPE set/readback/restore proof, MAN
  transactions, PHY-ID retry, PHY reset, Ethernet runtime behavior, packet I/O,
  networking, sockets, SSH, Phase 12.2, and phase transition remain future
  explicitly queued or supervisor-owned work.
- not-an-issue: hardwareTestLock was not acquired because this closeout is
  static-only and performs no hardware action.
- removed: same-shaped local/static NCR.MPE guard retries for this
  candidate/control pair are closed; no source or evidence files were removed.

## Accepted Checkpoint

Accepted source contract:
phase12-rp1-ethernet-mdio-mpe-enable-source-contract-v1.

Accepted guard report contract:
phase12-rp1-ethernet-mdio-mpe-enable-guard-report-contract-v1.

Accepted candidate classification:
rp1-ethernet-mdio-mpe-enable-guard-candidate-local-static.

Accepted control classification:
no-mdio-no-ethernet-rp1-ethernet-mdio-mpe-enable-control.

Accepted closeout classification:
rp1-ethernet-mdio-mpe-enable-guard-static-frontier-closed.

~~~text
future proof operation: rp1-ethernet-mdio-mpe-enable-set-readback-restore
controller: rp1_eth / raspberrypi,rp1-gem / cdns,macb
register: MACB/GEM NCR
source target: 0xc040100000
observed target: 0x1c00100000
offset: 0x0000
width: 32-bit little-endian volatile load/store
MPE bit: 4
MPE mask: 0x00000010
write value rule: pre_raw | 0x00000010
restore baseline: exact pre_raw
restore invariant: restore_raw must equal pre_raw
~~~

This checkpoint authorizes only the already queued serialized proof task to
attempt the guarded NCR.MPE set/readback/restore boundary under
hardwareTestLock. It does not accept the write itself, MAN writes, PHY-ID
reads, broad MDIO/PHY ownership, PHY reset/GPIO32 ownership, Ethernet driver
behavior, interrupt completion, DMA/descriptors, packet I/O, networking,
sockets, SSH, Phase 12.2, or a phase transition.

## Selected Proof Gates

The serialized Pi 5 proof is mechanically authorized because the accepted
contract and guard core define the exact target identity, write rule, restore
invariant, paired control, capture requirements, and allowed classifications
needed for a bounded candidate/control hardware run.

The proof must:

- acquire hardwareTestLock before archive publication, staging, power action,
  or runtime RP1 MMIO access;
- retain candidate/control identity, archive review output, fresh serial cursor
  and transcript, TFTP delta, final pre-restore identity, lab boot restore
  evidence, capture summary, classification JSON, and evidence map;
- classify only as rp1-ethernet-mdio-mpe-enable-set-readback-restored,
  rp1-ethernet-mdio-mpe-enable-already-set-restored,
  rp1-ethernet-mdio-mpe-enable-readback-mismatch-restored,
  rp1-ethernet-mdio-mpe-enable-restore-failed,
  rp1-ethernet-mdio-mpe-enable-blocked-target-inconclusive,
  precise-staging-capture-blocker, or
  no-mdio-no-ethernet-rp1-ethernet-mdio-mpe-enable-control;
- perform no MAN write, no PHY-ID read, and no GPIO32/PHY reset write;
- infer no broad MDIO/PHY ownership, Ethernet runtime readiness, interrupts,
  DMA/descriptors, packet I/O, networking, sockets, SSH, Phase 12.2, or phase
  transition.

## Evidence

- Accepted source contract:
  `tasks/2026-06-11-phase12-rp1-ethernet-mdio-mpe-enable-source-contract.md`.
- Accepted guard core:
  `tasks/2026-06-11-phase12-rp1-ethernet-mdio-mpe-enable-guard-core.md`.
- Guard core classification:
  `tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-mpe-enable-guard-core/classification.json`.
- Guard core evidence map:
  `tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-mpe-enable-guard-core/evidence-map.json`.
- Implementation:
  `src/rp1_ethernet.rs`.
- Project docs:
  `docs/src/project/phase12-networking-ssh.md`.
- Closeout classification:
  `tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-mpe-enable-guard-closeout/classification.json`.
- Closeout evidence map:
  `tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-mpe-enable-guard-closeout/evidence-map.json`.

## Validation

- static inspection: accepted source contract, guard core task record, guard
  core classification/evidence JSON, focused tests, touched source, project
  docs, and git history reviewed.
- JSON validation: jq empty on task-owned classification/evidence-map JSON.
- diff check: git diff --check.
- documentation build: /home/node/.cargo/bin/mdbook build.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Checkpoint reconciles accepted guard evidence without expanding beyond
  NCR.MPE set/readback/restore report boundaries: satisfied.
- NextAction explicitly names the serialized Pi 5 NCR.MPE proof and preserves
  no-MAN/no-PHY-ID/no-Ethernet boundaries: satisfied.
- Hardware proof is selected, so planningNeeded remains false: satisfied.
- Accepted checkpoint is committed before any hardware proof starts: satisfied
  by the commit recorded in supervisor state after this task.

## Next Action

Mechanically promote
phase12-rp1-ethernet-mdio-mpe-enable-pi5-proof-20260611 on the next worker wake
if dependencies remain satisfied, hardwareTestLock remains unlocked, and
supervisorIntervention.active remains false. That task must serialize under
hardwareTestLock and preserve candidate/control identity, archive review
output, fresh serial cursor/output, TFTP delta, final pre-restore identity, lab
boot restore evidence, task-owned capture summary, classification JSON, and
evidence map before accepting any NCR.MPE proof or precise blocker.
