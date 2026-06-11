# Phase 12 RP1 Ethernet GPIO32 Event-State Source Contract

Task id: phase12-rp1-ethernet-gpio32-event-state-source-contract-20260611
Status: accepted
Owner: worker
Classification:
rp1-ethernet-gpio32-event-state-source-contract-accepted
Evidence level: static inspection of accepted GPIO32 write/restore v2 proof,
write/restore source/guard records, retained RP1 pinctrl source, project docs,
and git history. No hardware run, GPIO/RIO/pad/MMIO write, event clear,
write/restore retry, MDIO/PHY work, Ethernet driver behavior, packet I/O,
networking, sockets, SSH, Phase 12.2, or phase transition was performed.

## Goal

Define the smallest safe read-only GPIO32 event-state discriminator after the
accepted GPIO32 / ETH_RST_N write/restore v2 blocker observed event bits
before any write.

## Scope

- Consumed the accepted v2 proof and closeout, including the candidate
  blocker state: baseline-status=0xabe3300, baseline-ctrl=0x85,
  baseline-out=0x10, baseline-oe=0x10, baseline-in=0x12,
  event-bits=0xab00000, touched-fields=RIO1_OUT.bit4,RIO1_OE.bit4, and
  writes-performed=false.
- Consumed the accepted GPIO32 write/restore source and guard contract
  records for exact GPIO32 target identity, observed-aperture addresses,
  active-low ETH_RST_N polarity, no-write preconditions, rejected claims, and
  paired control behavior.
- Retained source-backed RP1 GPIO STATUS bit names from
  tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/pinctrl-rp1.c:
  raw falling/rising/low/high bits 20-23 and filtered
  falling/rising/low/high bits 24-27.
- Defined a read-only candidate report shape that may load and report GPIO32
  STATUS, GPIO32 CTRL, RIO1 OUT/OE/IN, and GPIO32 pad state from the accepted
  observed 0x1c aperture, but must not clear events or write any register.
- Defined a paired no-GPIO/no-Ethernet control shape that preserves the same
  report path while constructing no GPIO32/RIO/pad/MMIO target facts.

## Non-Goals

No source code implementation, Pi 5 hardware run, boot archive publication,
hardwareTestLock acquisition, GPIO/RIO/pad/MMIO write, event clear, IRQRESET,
INTE/CTRL write, PHY reset assertion/deassertion, GPIO32 ownership,
write/restore retry, MDIO/PHY ownership, Ethernet driver behavior,
interrupt delivery/completion, DMA/descriptors, packet I/O, networking,
sockets, SSH, Phase 12.2, or phase transition.

## Contract Summary

Accepted contract id:
phase12-rp1-ethernet-gpio32-event-state-source-contract-v1.

~~~text
name: rp1-ethernet-gpio32-event-state-readonly-discriminator
line: RP1 GPIO32 / ETH_RST_N
bank: bank1
bank-local bit: 4
operation: read-only event-state discriminator; no event clear, no write,
  no PHY reset assertion/deassertion, and no ownership claim
source lineage: accepted GPIO32 write/restore v2 blocked/no-write proof
~~~

Allowed candidate reads, in order:

- GPIO32 STATUS at 0x1c000d4020, 32-bit volatile load.
- GPIO32 CTRL at 0x1c000d4024, 32-bit volatile load.
- RIO1 OUT at 0x1c000e4000, 32-bit volatile load.
- RIO1 OE at 0x1c000e4004, 32-bit volatile load.
- RIO1 IN at 0x1c000e4008, 32-bit volatile load.
- GPIO32 pad at 0x1c000f4014, 32-bit volatile load.

No writes are allowed by this contract. In particular, the discriminator must
not write GPIO32 CTRL/SET/CLR, IRQRESET, IO_BANK1 INTE, RIO1 OUT/OE aliases,
GPIO32 pad state, clock/reset registers, MDIO/PHY registers, Ethernet MAC/GEM
registers, DMA/descriptors, or interrupt-controller registers.

## Source-Backed Event Decode

The retained RP1 pinctrl source defines GPIO STATUS event-state bits:

- bit 20: raw falling
- bit 21: raw rising
- bit 22: raw low
- bit 23: raw high
- bit 24: filtered falling
- bit 25: filtered rising
- bit 26: filtered low
- bit 27: filtered high

The follow-up core may decode only those source-backed names. Any event-state
field outside bits 20-27, any status value that cannot be read, or any
unretained semantic such as whether an event is stale, clearable, harmless,
owned by firmware, or safe to ignore must be reported as
source-unresolved-event-state rather than inferred.

## Candidate Fields

The candidate report must preserve:

- event-state contract id and accepted write/restore source/guard contract
  ids;
- v2 blocker task id, closeout task id, classification, commit, and
  writes-performed=false lineage;
- GPIO32 / ETH_RST_N target identity: bank1 bit 4, active-low route, STATUS,
  CTRL, RIO1 OUT/OE/IN, and pad observed addresses;
- read-only raw values for STATUS, CTRL, RIO1 OUT, RIO1 OE, RIO1 IN, and pad
  when available;
- STATUS event mask bits 20-27 and source-backed decoded names listed above;
- source-decoding status: source-backed-bits-20-27, source-unresolved, or
  capture-chain-inconclusive;
- rejected claims for event clearing, GPIO32 ownership, PHY reset
  assertion/deassertion, MDIO/PHY ownership, Ethernet driver behavior,
  interrupts, DMA/descriptors, packet I/O, networking, sockets, SSH, Phase
  12.2, and phase transition.

## Classifications

The local/static core and later read-only Pi 5 proof may classify only:

- rp1-ethernet-gpio32-event-state-clear-precondition
- rp1-ethernet-gpio32-event-state-blocked-event-state
- rp1-ethernet-gpio32-event-state-source-unresolved-event-state
- rp1-ethernet-gpio32-event-state-inconclusive-capture
- no-gpio-no-ethernet-rp1-ethernet-gpio32-event-state-control
- staging/build-blocker

The clear-precondition classification means the report read all selected
fields and bits 20-27 contain no source-backed event bits requiring this
discriminator to hold the write/restore path. It is not GPIO32 ownership and
does not authorize a write/restore retry by itself.

The blocked-event-state classification means one or more source-backed STATUS
event bits in bits 20-27 are present, including the accepted v2 observed
event-bits 0xab00000. It is evidence for a future supervisor decision, not
permission to clear events or write GPIO32.

The source-unresolved-event-state classification means the source-backed bit
names are insufficient to decide whether the observed state is clearable,
stale, firmware-owned, or safe for write/restore. The result must remain a
planning input.

The inconclusive-capture classification means selected-tree identity, TFTP,
serial freshness, required report fields, or final evidence retention is
incomplete.

## Paired Control

The paired control must branch through the same report path and emit the same
classification field, but must construct no GPIO32/RIO/pad/MMIO target facts,
perform no volatile load or store, and withhold candidate-only GPIO32 /
ETH_RST_N facts. It must classify as
no-gpio-no-ethernet-rp1-ethernet-gpio32-event-state-control.

## Findings

- fixed: distinguished this contract from GPIO32 write/restore ownership and
  from event clearing. It is read-only event-state discrimination only.
- fixed: preserved the exact accepted v2 blocker lineage and observed state
  without reclassifying the v2 proof as a success.
- fixed: retained source-backed STATUS bit names for raw and filtered
  falling/rising/low/high bits 20-27 from pinctrl-rp1.c.
- fixed: marked any stale/clearable/owned/harmless event interpretation as
  source-unresolved rather than inferred.
- fixed: defined candidate and paired control behavior, accepted
  classifications, rejected claims, and next follow-up boundary.
- deferred: local/static implementation, serialized Pi 5 read-only proof,
  event clearing, GPIO32 write/restore ownership, MDIO/PHY ownership, and
  Ethernet runtime behavior remain future explicitly queued work.
- not-an-issue: this task did not acquire hardwareTestLock because it is
  source/docs/evidence only and performs no hardware action.

No findings were removed.

## Accepted Claims

This task accepts only the source/docs/evidence contract for a read-only
GPIO32 event-state discriminator and its paired no-GPIO/no-Ethernet control.
It does not accept event clearing, GPIO32 ownership, PHY reset
assertion/deassertion, a write/restore retry, MDIO/PHY ownership, Ethernet
driver readiness, interrupt delivery/completion, DMA/descriptors, packet I/O,
networking, sockets, SSH, Phase 12.2, or a phase transition.

## Evidence

- Classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-gpio32-event-state-source-contract/classification.json.
- Evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-gpio32-event-state-source-contract/evidence-map.json.
- Accepted v2 proof:
  tasks/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-v2-pi5-proof.md.
- Accepted v2 closeout:
  tasks/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-v2-proof-closeout.md.
- Retained RP1 pinctrl source:
  tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/pinctrl-rp1.c.

## Validation

- static inspection: accepted v2 proof/closeout, GPIO32 source contract,
  guard core/closeout, Phase 12 project doc, retained RP1 GPIO source notes,
  and pinctrl-rp1.c reviewed.
- JSON validation: jq empty on task-owned classification/evidence-map JSON
  passed.
- diff check: git diff --check passed.
- docs build: /home/node/.cargo/bin/mdbook build passed.
- staged diff check: git diff --cached --check passed.

## Next Action

Promote
phase12-rp1-ethernet-gpio32-event-state-discriminator-core-20260611 on the
next worker wake if dependencies remain satisfied. Keep that task local/static
only; do not run hardware, clear events, write GPIO/RIO/pad/MMIO, or retry
GPIO32 write/restore from the core task.
