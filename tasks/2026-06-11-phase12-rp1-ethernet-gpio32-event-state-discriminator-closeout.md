# Phase 12 RP1 Ethernet GPIO32 Event-State Discriminator Closeout

Task id: phase12-rp1-ethernet-gpio32-event-state-discriminator-closeout-20260611
Status: accepted
Classification:
rp1-ethernet-gpio32-event-state-discriminator-static-frontier-closed
Evidence level: static inspection of accepted contract/core records,
classification/evidence JSON, source implementation, project docs, and git
history. No hardware run, archive publication, event clear, GPIO/RIO/pad/MMIO
write, GPIO32 write/restore retry, MDIO/PHY work, Ethernet driver behavior,
packet I/O, networking, sockets, SSH, Phase 12.2, or phase transition was
performed.

## Goal

Close the local/static GPIO32 event-state discriminator core and decide whether
the queued serialized read-only Pi 5 proof is mechanically selected.

## Scope Performed

- Reconciled the accepted event-state source contract against the accepted
  local/static report implementation in src/rp1_ethernet.rs.
- Confirmed the candidate report shape preserves the accepted v2 blocked/no-write
  lineage, exact GPIO32 / ETH_RST_N target identity, read-only event-state
  fields, source-decoding status, rejected claims, retained risks, and
  hardware-proof boundary classification.
- Confirmed the paired no-GPIO/no-Ethernet control uses the same report path
  while carrying no GPIO32/RIO/pad/MMIO target facts or raw GPIO values.
- Confirmed the implementation and task evidence do not expand acceptance to
  hardware evidence, event clearing, write/restore ownership, GPIO32 ownership,
  MDIO/PHY ownership, Ethernet readiness, packet I/O, networking, SSH, Phase
  12.2, or a phase transition.
- Selected the already queued serialized read-only Pi 5 proof as the next
  bounded task because candidate/control report shape, source-unresolved
  event-state handling, and capture requirements are explicit.

## Findings

- fixed: the accepted static core provides the report surface needed by the
  queued read-only Pi 5 proof without requiring GPIO32 ownership or writes.
- fixed: candidate/control report boundaries are explicit enough for a
  serialized hardware proof: candidate may report read-only GPIO32 STATUS/CTRL,
  RIO1 OUT/OE/IN, and pad state; control withholds all GPIO32/RIO/pad/MMIO
  target facts.
- fixed: source-backed event decoding is limited to retained STATUS bits 20-27,
  and stale/clearable/firmware-owned/harmless/safe-to-ignore interpretations
  remain source-unresolved rather than inferred.
- not-an-issue: docs/src/project/phase12-networking-ssh.md needed only a
  checkpoint update; no API or frontier name changed.
- deferred: Pi 5 archive publication, serial/TFTP proof, and any future
  event-clear or GPIO32 write/restore authority remain outside this static
  closeout.
- removed: no obsolete implementation, docs, or task evidence was removed.

## Accepted Claims

This closeout accepts only the local/static GPIO32 event-state discriminator
frontier and the paired no-GPIO/no-Ethernet control report path. It also
accepts that the next mechanically selected task is the serialized read-only
Pi 5 proof.

The closeout does not accept hardware evidence, event clearing, GPIO/RIO/pad
MMIO writes, GPIO32 ownership, GPIO32 write/restore retry or success, PHY reset
assertion/deassertion, MDIO/PHY ownership, Ethernet driver readiness,
interrupt completion, DMA/descriptors, packet I/O, networking, sockets, SSH,
Phase 12.2, or phase transition.

## Next Action

Promote
phase12-rp1-ethernet-gpio32-event-state-readonly-pi5-proof-20260611 on the
next worker wake if hardwareTestLock remains unlocked and supervisorIntervention
remains inactive. The proof must acquire hardwareTestLock before archive
publication, staging, power action, or any lab run. It must retain
candidate/control selected-tree identity, archive digest, run-unique serial
marker, stable TFTP delta, final status/restore evidence, classification JSON,
evidence map, and capture summary. It must not perform GPIO/RIO/pad/MMIO
writes or event clearing.

## Evidence

- Classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-gpio32-event-state-discriminator-closeout/classification.json.
- Evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-gpio32-event-state-discriminator-closeout/evidence-map.json.
- Accepted source contract:
  tasks/2026-06-11-phase12-rp1-ethernet-gpio32-event-state-source-contract.md.
- Accepted static core:
  tasks/2026-06-11-phase12-rp1-ethernet-gpio32-event-state-discriminator-core.md.
- Implementation:
  src/rp1_ethernet.rs.
- Project checkpoint:
  docs/src/project/phase12-networking-ssh.md.

## Validation

- static inspection: accepted source contract, accepted static core record,
  classification/evidence JSON, src/rp1_ethernet.rs, project docs, and git
  history reviewed.
- JSON validation: jq empty on task-owned classification/evidence-map JSON
  passed.
- diff check: git diff --check passed.
- docs build: /home/node/.cargo/bin/mdbook build passed.
- staged diff check: git diff --cached --check passed.

## Acceptance

Accepted. The static discriminator frontier is closed and the serialized
read-only Pi 5 proof is mechanically selected as the next explicit queued
task. No hardware action, archive publication, event clear, GPIO/RIO/pad/MMIO
write, GPIO32 write/restore retry, or broader Ethernet/networking claim was
accepted.
