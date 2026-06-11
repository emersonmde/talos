# Phase 12 RP1 Ethernet GPIO32 Event-Clear Source Contract

Task id: phase12-rp1-ethernet-gpio32-event-clear-source-contract-20260611

Status: accepted

Classification:
rp1-ethernet-gpio32-event-clear-source-contract-accepted

Evidence level: static inspection of accepted GPIO32 event-state proof and
closeout, accepted event-state source contract, retained RP1 pinctrl source,
Phase 12 project docs, and git history. No Pi 5 hardware run, boot archive
publication, hardwareTestLock acquisition, GPIO32 write/restore retry, PHY
reset assertion/deassertion, MDIO/PHY work, Ethernet driver behavior, packet
I/O, networking, sockets, SSH, Phase 12.2, or phase transition was performed.

## Goal

Define a source-backed GPIO32 event-clear contract, or stop with a precise
source-unresolved blocker, before any write-backed GPIO32 follow-up.

## Scope Performed

- Consumed the accepted read-only GPIO32 event-state proof and closeout,
  including GPIO32 STATUS 0x0abe3300, CTRL 0x85, RIO1 OUT/OE 0x10,
  RIO1 IN 0x12, pad 0x56, event bits 0x0ab00000, source-backed bits 20-27,
  writes-performed=false, and event-clear-performed=false.
- Re-inspected retained Raspberry Pi Linux pinctrl source for GPIO STATUS
  event bits, GPIO CTRL IRQRESET, SET/CLR alias offsets, GPIO interrupt
  handler acknowledgement, interrupt disable, and IRQ type setup.
- Defined the only source-backed future clear operation as a single 32-bit
  write of 0x10000000, RP1_GPIO_CTRL_IRQRESET, to GPIO32 CTRL through the
  SET alias at observed target 0x1c000d6024.
- Preserved the accepted candidate/control evidence lineage and same-shaped
  read-only retry closure from the event-state closeout.
- Rejected GPIO32 ownership, PHY reset, GPIO32 write/restore retry, MDIO/PHY,
  Ethernet driver, DMA, interrupt completion, packet, networking, socket, SSH,
  Phase 12.2, and phase-transition claims.

## Source Findings

- fixed: retained pinctrl-rp1.c defines event-state bits in GPIO STATUS bits
  20-27: raw falling/rising/low/high and filtered falling/rising/low/high.
- fixed: retained pinctrl-rp1.c defines RP1_GPIO_CTRL_IRQRESET = BIT(28).
- fixed: retained pinctrl-rp1.c clears latched events by writing
  RP1_GPIO_CTRL_IRQRESET to pin->gpio + RP1_SET_OFFSET + RP1_GPIO_CTRL in
  the chained GPIO IRQ handler, IRQ disable path, IRQ type setup path, and
  GPIO IRQ ack path.
- fixed: selected the GPIO32 CTRL SET-alias target from the accepted GPIO32
  CTRL observed target 0x1c000d4024 plus RP1_SET_OFFSET 0x2000, giving
  observed target 0x1c000d6024 and source target 0xc0400d6024.
- fixed: limited any future clear attempt to the accepted GPIO32 STATUS event
  mask 0x0ff00000 and accepted observed event bits 0x0ab00000, with any
  different or unretained event state classified as a blocker before writing.
- fixed: required pre-read and post-read invariants that preserve GPIO32 CTRL
  non-IRQRESET fields, RIO1 OUT/OE/IN, and GPIO32 pad state.
- deferred: local/static event-clear guard implementation, serialized Pi 5
  event-clear proof, GPIO32 write/restore ownership, PHY reset, MDIO/PHY, and
  Ethernet runtime behavior remain future explicitly queued work.
- not-an-issue: no hardwareTestLock was acquired because this task is
  source/local/static only and performs no hardware action.
- removed: no obsolete implementation, docs, or task evidence was removed.

## Accepted Contract

Accepted contract id:
phase12-rp1-ethernet-gpio32-event-clear-source-contract-v1.

~~~text
name: rp1-ethernet-gpio32-event-clear-source-contract
line: RP1 GPIO32 / ETH_RST_N
source-backed operation: clear latched GPIO32 events by writing
  RP1_GPIO_CTRL_IRQRESET through the GPIO32 CTRL SET alias
source target: 0xc0400d6024
observed target: 0x1c000d6024
width: 32-bit little-endian volatile store
write value: 0x10000000
pre-read event mask: 0x0ff00000
accepted event bits: 0x0ab00000
~~~

This contract accepts only source-backed event-clear semantics for a later
guard/proof. It does not perform or accept the write itself.

## Required Pre-Read Invariants

A future guarded candidate must read and retain these values before any event
clear write:

- GPIO32 STATUS at 0x1c000d4020.
- GPIO32 CTRL at 0x1c000d4024.
- RIO1 OUT at 0x1c000e4000.
- RIO1 OE at 0x1c000e4004.
- RIO1 IN at 0x1c000e4008.
- GPIO32 pad at 0x1c000f4014.

Before the write, the candidate must prove:

- selected-tree identity, serial freshness, TFTP delta, and final identity are
  capture-chain-current;
- the selected reads are present and non-sentinel;
- status & 0x0ff00000 == 0x0ab00000;
- no STATUS event bit outside bits 20-27 is being interpreted;
- GPIO32 CTRL FUNCSEL remains the accepted GPIO function value 5;
- GPIO32 CTRL OUTOVER/OEOVER/INOVER do not bypass raw RIO OUT/OE handling;
- RIO1 OUT/OE/IN and GPIO32 pad are retained for post-write comparison;
- the candidate is not claiming GPIO32 ownership, PHY reset assertion or
  deassertion, GPIO32 write/restore retry, MDIO/PHY ownership, interrupt
  ownership/completion, Ethernet readiness, DMA/descriptors, packet I/O,
  networking, sockets, SSH, Phase 12.2, or a phase transition.

If any invariant fails, the future candidate must stop before the write and
classify a precise blocker.

## Required Write

The only selected write for a future proof is:

~~~text
target: GPIO32 CTRL SET alias, observed 0x1c000d6024
value:  0x10000000
meaning: RP1_GPIO_CTRL_IRQRESET
source: retained pinctrl-rp1.c acknowledgement/disable/type-setup paths
~~~

No STATUS, CTRL RW/CLR/XOR, IO_BANK1 INTE/INTS, RIO OUT/OE/IN, pad,
clock/reset, MDIO/PHY, Ethernet MAC/GEM, DMA/descriptor, interrupt-controller,
or non-GPIO32 register write is accepted by this contract.

## Required Post-Read Invariants

After the future write, the candidate must re-read GPIO32 STATUS, CTRL,
RIO1 OUT/OE/IN, and pad. The future proof may only claim event-clear
discrimination when:

- GPIO32 STATUS event bits are cleared or classified as persistent/source-owned
  without broadening to GPIO32 ownership;
- GPIO32 CTRL non-IRQRESET fields match the pre-read value;
- RIO1 OUT, RIO1 OE, RIO1 IN, and GPIO32 pad match pre-read values;
- no GPIO32 PHY reset assertion/deassertion, MDIO/PHY behavior, Ethernet
  driver behavior, interrupt completion, DMA/descriptors, packet I/O,
  networking, sockets, SSH, Phase 12.2, or phase transition is claimed.

## Paired Control Requirement

The paired control must use the same report path while constructing no
GPIO32/RIO/pad/MMIO target facts and performing no volatile load or store. It
must classify as the queued guard/proof control classification, not as event
clear success.

## Rejected Claims And Retained Risks

Rejected claims:

- immediate event clearing or hardware write by this task;
- GPIO32 ownership;
- PHY reset assertion/deassertion;
- GPIO32 write/restore retry or success;
- non-GPIO32 GPIO/RIO/pad/MMIO writes;
- MDIO/PHY ownership;
- Ethernet driver readiness;
- interrupt ownership or completion;
- DMA/descriptors;
- packet I/O, networking, sockets, SSH;
- Phase 12.2 or phase transition.

Retained risks:

- The source-backed IRQRESET write may clear edge latched events while level
  events reassert if the line state remains active.
- Firmware or hardware may repopulate event bits after the clear.
- The event-clear proof may still classify persistent/source-owned event state
  or capture/staging blockers.
- GPIO32 write/restore ownership and PHY reset remain unaccepted even if a
  future event-clear proof succeeds.

## Evidence

- Classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-gpio32-event-clear-source-contract/classification.json.
- Evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-gpio32-event-clear-source-contract/evidence-map.json.
- Accepted read-only event-state proof:
  tasks/2026-06-11-phase12-rp1-ethernet-gpio32-event-state-readonly-pi5-proof.md.
- Accepted read-only proof closeout:
  tasks/2026-06-11-phase12-rp1-ethernet-gpio32-event-state-proof-closeout.md.
- Accepted event-state source contract:
  tasks/2026-06-11-phase12-rp1-ethernet-gpio32-event-state-source-contract.md.
- Retained RP1 pinctrl source:
  tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/pinctrl-rp1.c.
- Project docs:
  docs/src/project/phase12-networking-ssh.md.

## Validation

- static inspection: accepted event-state proof/closeout, event-state source
  contract, retained RP1 pinctrl source, project docs, and git history
  reviewed.
- JSON validation: jq empty on task-owned classification/evidence-map JSON.
- diff check: git diff --check.
- documentation build: /home/node/.cargo/bin/mdbook build.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Contract defines source-backed event-clear semantics with exact
  target/mask/value/pre-read/post-read invariants: satisfied.
- Future clear operation is limited to accepted GPIO32 STATUS event bits and
  explicitly forbids downstream claims: satisfied.
- Accepted candidate/control evidence lineage and same-shaped retry closure are
  preserved: satisfied.
- Accepted contract is committed before event-clear guard implementation starts:
  satisfied by the commit recorded in supervisor state after this task.

## Next Action

Mechanically promote
phase12-rp1-ethernet-gpio32-event-clear-guard-core-20260611 on the next worker
wake if dependencies remain satisfied. Keep that task local/static only; do not
run hardware or clear events from the guard-core task.
