# Phase 12 RP1 Ethernet GPIO32 Event-Clear Guard Closeout

Task id: phase12-rp1-ethernet-gpio32-event-clear-guard-closeout-20260611

Status: accepted

Classification:
rp1-ethernet-gpio32-event-clear-guard-static-frontier-closed

Evidence level: static inspection of accepted source contract, guard core task
record, guard core classification/evidence JSON, focused tests, touched source,
project docs, and git history. No Pi 5 hardware run, boot archive publication,
hardwareTestLock acquisition, volatile MMIO execution, event clear, GPIO32
write/restore retry, PHY reset assertion/deassertion, MDIO/PHY work, Ethernet
driver behavior, packet I/O, networking, sockets, SSH, Phase 12.2, or phase
transition was performed.

## Goal

Close the local/static GPIO32 event-clear guard frontier and decide whether the
serialized Pi 5 event-clear proof is mechanically objective.

## Scope Performed

- Reconciled the accepted source contract
  `phase12-rp1-ethernet-gpio32-event-clear-source-contract-v1` with the
  accepted guard report contract
  `phase12-rp1-ethernet-gpio32-event-clear-guard-report-contract-v1`.
- Confirmed the candidate guard preserves the exact source-backed GPIO32 CTRL
  SET-alias IRQRESET target, write value, event mask, accepted event bits,
  pre-read requirements, post-read requirements, forbidden writes, accepted
  event-state lineage, rejected claims, retained risks, and source evidence.
- Confirmed the paired control uses the same report path while withholding
  GPIO32/RIO/pad/MMIO target facts.
- Closed same-shaped local/static guard retries for this candidate/control pair.
- Selected only the queued serialized Pi 5 event-clear proof as the next
  mechanically objective task.

## Findings

- fixed: accepted guard evidence carries exact target/mask/value facts for the
  future proof: observed target 0x1c000d6024, source target 0xc0400d6024,
  write value 0x10000000, status event mask 0x0ff00000, and accepted event bits
  0x0ab00000.
- fixed: accepted guard evidence preserves the event-state blocker lineage and
  source-backed pre/post read requirements for GPIO32 STATUS/CTRL, RIO1
  OUT/OE/IN, and GPIO32 pad state.
- fixed: paired control withholds GPIO32/RIO/pad/MMIO target facts while
  retaining the same report path and explicit no-GPIO/no-Ethernet
  classification.
- fixed: validators and focused tests reject missing source contract, target
  mismatch, event mask mismatch, forbidden writes, volatile MMIO execution,
  GPIO32 ownership, GPIO32 write/restore retry, PHY reset, MDIO/PHY, Ethernet
  readiness, interrupt completion, DMA/descriptors, packet I/O, networking,
  sockets, SSH, Phase 12.2, and phase-transition claims.
- deferred: serialized Pi 5 event-clear proof, runtime event clearing,
  GPIO32 write/restore ownership, PHY reset, MDIO/PHY, Ethernet runtime
  behavior, packet I/O, networking, sockets, SSH, Phase 12.2, and phase
  transition remain future explicitly queued or supervisor-owned work.
- not-an-issue: hardwareTestLock was not acquired because this closeout is
  static-only and performs no hardware action.
- removed: same-shaped local/static event-clear guard retries for this
  candidate/control pair are closed; no source or evidence files were removed.

## Accepted Checkpoint

Accepted source contract:
phase12-rp1-ethernet-gpio32-event-clear-source-contract-v1.

Accepted guard report contract:
phase12-rp1-ethernet-gpio32-event-clear-guard-report-contract-v1.

Accepted candidate classification:
rp1-ethernet-gpio32-event-clear-guard-candidate-local-static.

Accepted control classification:
no-gpio-no-ethernet-rp1-ethernet-gpio32-event-clear-control.

Accepted closeout classification:
rp1-ethernet-gpio32-event-clear-guard-static-frontier-closed.

~~~text
future proof operation: GPIO32 CTRL SET alias IRQRESET event clear
source target: 0xc0400d6024
observed target: 0x1c000d6024
write value: 0x10000000
status event mask: 0x0ff00000
accepted event bits: 0x0ab00000
accepted status raw: 0x0abe3300
accepted ctrl raw: 0x00000085
accepted RIO1 OUT/OE/IN raw: 0x10 / 0x10 / 0x12
~~~

This checkpoint authorizes only the already queued serialized proof task to
attempt the guarded event-clear discriminator under hardwareTestLock. It does
not accept the write itself, GPIO32 ownership, GPIO32 write/restore ownership,
PHY reset assertion/deassertion, MDIO/PHY ownership, Ethernet driver behavior,
interrupt completion, DMA/descriptors, packet I/O, networking, sockets, SSH,
Phase 12.2, or a phase transition.

## Evidence

- Accepted source contract:
  `tasks/2026-06-11-phase12-rp1-ethernet-gpio32-event-clear-source-contract.md`.
- Accepted guard core:
  `tasks/2026-06-11-phase12-rp1-ethernet-gpio32-event-clear-guard-core.md`.
- Guard core classification:
  `tasks/evidence/2026-06-11-phase12-rp1-ethernet-gpio32-event-clear-guard-core/classification.json`.
- Guard core evidence map:
  `tasks/evidence/2026-06-11-phase12-rp1-ethernet-gpio32-event-clear-guard-core/evidence-map.json`.
- Implementation:
  `src/rp1_ethernet.rs`.
- Project docs:
  `docs/src/project/phase12-networking-ssh.md`.
- Closeout classification:
  `tasks/evidence/2026-06-11-phase12-rp1-ethernet-gpio32-event-clear-guard-closeout/classification.json`.
- Closeout evidence map:
  `tasks/evidence/2026-06-11-phase12-rp1-ethernet-gpio32-event-clear-guard-closeout/evidence-map.json`.

## Validation

- static inspection: accepted source contract, guard core task record,
  guard core classification/evidence JSON, focused tests, touched source,
  project docs, and git history reviewed.
- JSON validation: jq empty on task-owned classification/evidence-map JSON.
- diff check: git diff --check.
- documentation build: /home/node/.cargo/bin/mdbook build.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Checkpoint reconciles source contract and guard implementation without
  expanding acceptance to hardware/runtime ownership: satisfied.
- Same-shaped local/static guard retries are closed for this candidate/control
  pair: satisfied.
- NextAction mechanically selects the serialized Pi 5 event-clear proof:
  satisfied.
- Accepted checkpoint is committed before any hardware task starts: satisfied
  by the commit recorded in supervisor state after this task.

## Next Action

Mechanically promote
phase12-rp1-ethernet-gpio32-event-clear-pi5-proof-20260611 on the next worker
wake if dependencies remain satisfied and hardwareTestLock remains unlocked.
That task must serialize under hardwareTestLock and preserve candidate/control
identity, TFTP, serial freshness, final identity, restore proof, and task-owned
JSON before accepting any event-clear proof or blocker.
