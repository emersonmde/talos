# Phase 12 RP1 Ethernet MDIO MPE Enable Guard Core

Task id: phase12-rp1-ethernet-mdio-mpe-enable-guard-core-20260611

Status: accepted

Classification:
rp1-ethernet-mdio-mpe-enable-guard-core-accepted

Evidence level: local/static implementation, focused unit tests, JSON
validation, diff hygiene, and git history. No Pi 5 hardware run, boot archive
publication, hardwareTestLock acquisition, runtime RP1 MMIO write, MAN write,
PHY-ID read, PHY reset or GPIO32 action, Ethernet driver behavior, DMA,
interrupt, packet I/O, networking, sockets, SSH, Phase 12.2, or phase transition
was performed.

## Goal

Implement the local/static NCR.MPE write/restore guard report and paired control
selected by the accepted source contract.

## Scope Performed

- Added deterministic NCR.MPE enable source-contract and guard-report types in
  `src/rp1_ethernet.rs`.
- Candidate evidence preserves the accepted source contract id, observed-window
  MACB/GEM NCR target 0x1c00100000, MPE bit 4, mask 0x00000010, write rule
  `pre_raw | 0x00000010`, restore invariant `restore_raw == pre_raw`,
  allowed classifications, rejected claims, retained risks, and source evidence.
- Paired control evidence uses the same report path while withholding NCR/MPE
  target facts and write intent.
- Validators reject missing source contract, control target leakage, source
  contract identity/target/field drift, missing source evidence, runtime NCR
  write execution, MAN writes, PHY-ID reads, broad MDIO/PHY ownership,
  PHY reset/GPIO32 ownership, Ethernet readiness, broad MMIO readiness,
  DMA/descriptors, interrupts, packet I/O, networking, sockets, SSH, Phase 12.2,
  and phase-transition claims.
- Focused `rp1_ethernet` tests cover candidate construction, paired control
  construction, and deterministic rejection cases.

## Findings

- fixed: implemented the accepted source-backed candidate guard for MACB/GEM NCR
  MPE set/readback/restore at observed target 0x1c00100000.
- fixed: implemented the no-MDIO/no-Ethernet control report that withholds NCR
  target facts and candidate write intent.
- fixed: validators reject MAN writes, PHY-ID reads, broad MDIO/PHY claims,
  GPIO32/PHY-reset claims, runtime NCR write execution, downstream Ethernet
  readiness claims, and phase-transition overclaims before any future proof can
  consume the guard.
- deferred: serialized Pi 5 NCR.MPE write/readback/restore proof, MAN
  transactions, PHY-ID retry, PHY reset, Ethernet runtime behavior, packet I/O,
  networking, sockets, SSH, Phase 12.2, and phase transition remain future
  queued or supervisor-owned work.
- not-an-issue: hardwareTestLock was not acquired because this task is
  local/static only and performs no hardware action.
- removed: no obsolete code or evidence was removed.

## Accepted Guard Surface

Candidate classification:
rp1-ethernet-mdio-mpe-enable-guard-candidate-local-static.

Control classification:
no-mdio-no-ethernet-rp1-ethernet-mdio-mpe-enable-control.

~~~text
source contract: phase12-rp1-ethernet-mdio-mpe-enable-source-contract-v1
report contract: phase12-rp1-ethernet-mdio-mpe-enable-guard-report-contract-v1
future operation: rp1-ethernet-mdio-mpe-enable-set-readback-restore
register: MACB/GEM NCR
observed target: 0x1c00100000
MPE bit: 4
MPE mask: 0x00000010
write rule: pre_raw | 0x00000010
restore invariant: restore_raw must equal pre_raw
~~~

The guard is a local/static report surface only. It does not perform or accept
the NCR write.

## Evidence

- Implementation:
  `src/rp1_ethernet.rs`.
- Classification:
  `tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-mpe-enable-guard-core/classification.json`.
- Evidence map:
  `tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-mpe-enable-guard-core/evidence-map.json`.
- Accepted source contract:
  `tasks/2026-06-11-phase12-rp1-ethernet-mdio-mpe-enable-source-contract.md`.
- Accepted MDIO PHY-ID closeout:
  `tasks/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-proof-closeout.md`.

## Validation

- static inspection: accepted source contract and touched `src/rp1_ethernet.rs`.
- fmt: `cargo fmt --all -- --check`.
- focused tests: `cargo -Zjson-target-spec test --quiet rp1_ethernet`.
- JSON validation: `jq empty` on task-owned classification/evidence-map JSON.
- diff check: `git diff --check`.
- documentation build: not required; no `docs/src` files were touched.
- staged diff check: `git diff --cached --check` before commit.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Candidate report preserves the accepted contract id, observed-window NCR
  target, MPE bit, restore invariant, allowed classifications, rejected claims,
  and retained risks: satisfied.
- Control report uses the same report path while constructing no NCR/MPE target
  and no write intent: satisfied.
- Validators reject MAN writes, PHY-ID reads, broad MDIO/PHY ownership, PHY
  reset/GPIO32 ownership, Ethernet readiness, DMA/descriptors, interrupts,
  packet I/O, networking, sockets, SSH, Phase 12.2, and phase transition:
  satisfied.
- Focused tests cover candidate, control, and deterministic rejection cases:
  satisfied.
- Accepted implementation/evidence is committed before closeout starts:
  satisfied by the commit recorded in supervisor state after this task.

## Next Action

Mechanically promote
phase12-rp1-ethernet-mdio-mpe-enable-guard-closeout-20260611 on the next worker
wake if dependencies remain satisfied. Do not run hardware, write NCR, construct
MAN frames, retry PHY-ID, or infer broader Ethernet readiness from this task.
