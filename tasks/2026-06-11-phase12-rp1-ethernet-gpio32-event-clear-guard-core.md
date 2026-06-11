# Phase 12 RP1 Ethernet GPIO32 Event-Clear Guard Core

Task id: phase12-rp1-ethernet-gpio32-event-clear-guard-core-20260611

Status: accepted

Classification:
rp1-ethernet-gpio32-event-clear-guard-core-accepted

Evidence level: local/static implementation, focused unit tests, JSON
validation, diff hygiene, and git history. No Pi 5 hardware run, boot archive
publication, hardwareTestLock acquisition, volatile MMIO execution, event clear,
GPIO32 write/restore retry, PHY reset assertion/deassertion, MDIO/PHY work,
Ethernet driver behavior, packet I/O, networking, sockets, SSH, Phase 12.2, or
phase transition was performed.

## Goal

Implement the local/static GPIO32 event-clear guard report and validators
selected by the accepted source contract.

## Scope Performed

- Added deterministic GPIO32 event-clear guard contract/report types in
  \`src/rp1_ethernet.rs\`.
- Candidate evidence preserves the accepted source contract id, event-state
  blocker lineage, GPIO32 STATUS/CTRL/RIO/pad read targets, CTRL SET-alias
  IRQRESET clear target, event mask 0x0ff00000, accepted event bits
  0x0ab00000, write value 0x10000000, pre-read/post-read requirements, forbidden
  writes, rejected claims, retained risks, and source evidence.
- Paired control evidence uses the same report path while withholding
  GPIO32/RIO/pad/MMIO target facts and classifies as
  no-gpio-no-ethernet-rp1-ethernet-gpio32-event-clear-control.
- Validators reject missing source contract, control target leakage, wrong
  identity, wrong target, wrong event mask/field values, wrong lineage, missing
  source evidence, event-clear execution, volatile MMIO execution, writes outside
  IRQRESET, CTRL RW/CLR/XOR writes, RIO/pad/function mutation, GPIO32 ownership,
  PHY reset, GPIO32 write/restore retry, MDIO/PHY, Ethernet readiness,
  interrupt completion, DMA/descriptors, packet I/O, networking, sockets, SSH,
  Phase 12.2, and phase transition claims.
- Focused \`gpio32\` tests cover accepted candidate construction, accepted paired
  control construction, and deterministic rejection cases.

## Findings

- fixed: implemented source-backed local/static candidate guard evidence for the
  accepted GPIO32 CTRL SET-alias IRQRESET operation at observed target
  0x1c000d6024.
- fixed: implemented paired no-GPIO/no-Ethernet control evidence that withholds
  GPIO32/RIO/pad/MMIO facts while preserving the same report path.
- fixed: validators reject target, mask, lineage, source-evidence, forbidden
  write, ownership, downstream runtime, and phase-transition overclaims before
  any future proof can consume the guard.
- fixed: focused tests cover candidate, control, and deterministic rejection
  cases under the \`gpio32\` filter.
- deferred: serialized Pi 5 event-clear proof, GPIO32 write/restore ownership,
  PHY reset, MDIO/PHY, Ethernet runtime behavior, packet I/O, networking, sockets,
  SSH, Phase 12.2, and phase transition remain future queued or supervisor-owned
  work.
- not-an-issue: hardwareTestLock was not acquired because this task is
  local/static only and performs no hardware action.
- removed: no obsolete code or evidence was removed.

## Accepted Guard Surface

Candidate classification:
rp1-ethernet-gpio32-event-clear-guard-candidate-local-static.

Control classification:
no-gpio-no-ethernet-rp1-ethernet-gpio32-event-clear-control.

~~~text
source contract: phase12-rp1-ethernet-gpio32-event-clear-source-contract-v1
report contract: phase12-rp1-ethernet-gpio32-event-clear-guard-report-contract-v1
future operation: GPIO32 CTRL SET alias IRQRESET clear
source target: 0xc0400d6024
observed target: 0x1c000d6024
write value: 0x10000000
status event mask: 0x0ff00000
accepted event bits: 0x0ab00000
accepted STATUS raw: 0x0abe3300
accepted CTRL raw: 0x00000085
accepted RIO1 OUT/OE/IN raw: 0x10 / 0x10 / 0x12
~~~

The guard is a local/static report surface only. It does not perform or accept
the clear write.

## Evidence

- Implementation:
  \`src/rp1_ethernet.rs\`.
- Classification:
  \`tasks/evidence/2026-06-11-phase12-rp1-ethernet-gpio32-event-clear-guard-core/classification.json\`.
- Evidence map:
  \`tasks/evidence/2026-06-11-phase12-rp1-ethernet-gpio32-event-clear-guard-core/evidence-map.json\`.
- Accepted source contract:
  \`tasks/2026-06-11-phase12-rp1-ethernet-gpio32-event-clear-source-contract.md\`.
- Accepted event-state closeout:
  \`tasks/2026-06-11-phase12-rp1-ethernet-gpio32-event-state-proof-closeout.md\`.

## Validation

- static inspection: accepted event-clear source contract and touched
  \`src/rp1_ethernet.rs\`.
- fmt: \`cargo fmt --all -- --check\`.
- focused tests: \`cargo -Zjson-target-spec test --quiet gpio32\`.
- JSON validation: jq empty on task-owned classification/evidence-map JSON.
- diff check: \`git diff --check\`.
- documentation build: not required; no \`docs/src\` files were touched.
- staged diff check: \`git diff --cached --check\` before commit.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Implementation exposes deterministic candidate and paired control event-clear
  guard reports from the accepted contract: satisfied.
- Candidate report includes exact target/mask/value/pre-read/post-read
  requirements, accepted evidence lineage, forbidden-register invariants, and
  rejected downstream claims: satisfied.
- Control report preserves the same reporting path while withholding
  GPIO32/RIO/pad/MMIO target facts: satisfied.
- Focused tests cover accepted candidate construction, accepted control
  construction, and deterministic rejection of forbidden writes/claims:
  satisfied.
- Accepted implementation/evidence is committed before closeout starts:
  satisfied by the commit recorded in supervisor state after this task.

## Next Action

Mechanically promote
phase12-rp1-ethernet-gpio32-event-clear-guard-closeout-20260611 on the next
worker wake if dependencies remain satisfied. Do not run hardware or clear
events from this task.
