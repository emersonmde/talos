# Phase 12 RP1 Ethernet MDIO MPE Enable Source Contract

Task id: phase12-rp1-ethernet-mdio-mpe-enable-source-contract-20260611

Status: accepted

Classification: rp1-ethernet-mdio-mpe-enable-source-contract-accepted

Evidence level: static inspection of the accepted MDIO PHY-ID proof/closeout
records, retained Raspberry Pi Linux MACB/RP1 source evidence, rp1_ethernet
source, Phase 12 docs, roadmap, and task-owned JSON. No Pi 5 hardware run,
boot archive publication, hardwareTestLock acquisition, runtime RP1 MMIO
write, MAN write, MDIO transaction, PHY-ID read, GPIO32/PHY reset work,
Ethernet driver behavior, DMA/descriptors, packet I/O, networking, sockets,
SSH, Phase 12.2, or phase transition was performed.

## Goal

Define the smallest source-backed NCR.MPE enable/readback/restore ownership
contract after the accepted MDIO PHY-ID proof closed as a no-write MPE-clear
blocker.

## Scope Performed

- Consumed the accepted MDIO PHY-ID source contract, Pi 5 proof, and closeout
  evidence, including the same-shaped PHY-ID retry closure.
- Inspected retained Linux MACB source for the management-port enable point,
  MACB register offsets, and MPE bit definition.
- Selected one exact observed-window target and one bit: MACB/GEM NCR at
  0x1c00100000, MPE bit 4.
- Defined a future candidate/control report shape for NCR.MPE set/readback/
  restore ownership only.
- Rejected MAN writes, PHY-ID reads, broad MDIO/PHY ownership, GPIO32/PHY reset
  ownership, Ethernet driver readiness, packet I/O, networking, sockets, SSH,
  Phase 12.2, and phase transition.
- Recorded findings with disposition.

## Non-Goals

No hardware run, no boot archive publication, no hardwareTestLock acquisition,
no runtime MMIO write, no MAN write, no runtime MDIO transaction, no PHY-ID
read, no PHY reset assertion/deassertion, no GPIO32 retry, no Ethernet driver
implementation, no DMA/descriptors, no interrupts, no packet I/O, no
networking, no sockets, no SSH, no Phase 12.2 work, and no phase transition.

## Reconciled Inputs

- tasks/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-source-contract.md.
- tasks/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-pi5-proof.md.
- tasks/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-proof-closeout.md.
- tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-pi5-proof/classification.json.
- tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-pi5-proof/capture-summary.json.
- tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-proof-closeout/classification.json.
- tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-macb_main.c.
- tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-source-contract/source/linux-rpi-6.12-macb.h.
- src/rp1_ethernet.rs.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.
- git history through 89183acc48b900dfa21e4074abe0c28ebd6b6532.

## Source Facts

- Linux macb_mii_init enables the management port before MII bus allocation by
  writing MACB_BIT(MPE) to NCR.
- macb.h defines MACB_NCR offset 0x0000 and MACB_MPE_OFFSET 4.
- The accepted rp1_eth source identity is raspberrypi,rp1-gem / cdns,macb at
  source RP1 bus offset 0x00100000.
- The accepted observed-window MACB_MID context is 0x1c001000fc, so the
  source-backed observed-window MACB/GEM controller base is 0x1c00100000.
- Therefore the source-backed observed-window NCR target for MPE is
  0x1c00100000 + 0x0000 = 0x1c00100000.
- The accepted MDIO PHY-ID proof classified a no-write blocker and performed
  no NCR, NSR, MAN, GPIO32, or PHY reset write. Same-shaped PHY-ID retries
  remain closed until NCR.MPE ownership is separately accepted.

## Selected Contract

Accepted contract id:
phase12-rp1-ethernet-mdio-mpe-enable-source-contract-v1.

~~~text
name: rp1-ethernet-mdio-mpe-enable-set-readback-restore
controller: rp1_eth / raspberrypi,rp1-gem / cdns,macb
source-backed operation: set NCR.MPE, read back, then restore baseline NCR
source RP1 bus target: 0xc040100000
observed-window target: 0x1c00100000
register: MACB/GEM NCR
offset: 0x0000
width: 32-bit little-endian volatile load/store
bit: MPE bit 4
write value rule: pre_raw | 0x00000010
restore baseline: exact pre_raw
restore invariant: restore_raw == pre_raw
~~~

The write value rule intentionally differs from Linux's full-value
MACB_BIT(MPE) write. Linux proves the source-backed semantic target: enable
the management port through NCR.MPE. The Talos proof must preserve unrelated
NCR fields by setting only bit 4 over the pre-read baseline, then restoring
the exact pre-read value before accepting the proof.

## Future Candidate Operation Order

1. Print a candidate start marker and accepted input frontier.
2. Read observed-window MACB_MID context at 0x1c001000fc as context only.
3. Pre-read NCR at 0x1c00100000 and retain pre_raw.
4. If target identity, source contract, serial/TFTP freshness, or the pre-read
   is inconclusive, perform no write and classify a precise blocker.
5. Write pre_raw | 0x00000010 to NCR at 0x1c00100000.
6. Read NCR after the write and retain post_raw; MPE bit 4 must be set for an
   accepted set/readback classification.
7. Restore-write pre_raw to NCR at 0x1c00100000.
8. Restore-read NCR and retain restore_raw; restore_raw must equal pre_raw for
   an accepted restored classification.
9. Classify only NCR.MPE write/readback/restore ownership. Do not construct a
   MAN frame, write MAN, read PHY ID registers, infer broad MDIO/PHY ownership,
   or infer Ethernet readiness.

## Allowed Future Proof Classifications

- rp1-ethernet-mdio-mpe-enable-set-readback-restored.
- rp1-ethernet-mdio-mpe-enable-already-set-restored.
- rp1-ethernet-mdio-mpe-enable-readback-mismatch-restored.
- rp1-ethernet-mdio-mpe-enable-restore-failed.
- rp1-ethernet-mdio-mpe-enable-blocked-target-inconclusive.
- precise-staging-capture-blocker.
- no-mdio-no-ethernet-rp1-ethernet-mdio-mpe-enable-control.

## Paired Control Boundary

The paired no-MDIO/no-Ethernet control must use the same reporting path while:

- constructing no NCR target and no candidate write value;
- performing no volatile load/store to NCR, NSR, MAN, GPIO32, RIO, pad,
  clock/reset, DMA, interrupt, packet, or network paths;
- withholding candidate-only raw values and set/readback/restore booleans;
- retaining contract id, rejected-claim labels, and the control
  classification;
- classifying only as
  no-mdio-no-ethernet-rp1-ethernet-mdio-mpe-enable-control.

## Findings

- fixed: selected MACB/GEM NCR MPE bit 4 as the smallest source-backed
  prerequisite for a later MDIO PHY-ID retry.
- fixed: selected the source-backed observed-window NCR target
  0x1c00100000 from accepted rp1_eth offset 0x00100000 and MACB_NCR offset
  0x0000.
- fixed: reconciled the accepted MDIO PHY-ID no-write proof as proof that the
  prior same-shaped PHY-ID candidate/control path is closed; the future MPE
  task must use this source-backed NCR target and must not reuse the PHY-ID
  proof as MPE ownership.
- fixed: defined write value, post-read, restore-write, restore-read, restore
  invariant, and allowed classifications for a bounded future Pi 5 proof.
- fixed: defined a paired no-MDIO/no-Ethernet control shape.
- deferred: local/static guard implementation, serialized Pi 5
  write/readback/restore proof, MAN writes, PHY-ID reads, broad MDIO/PHY
  ownership, PHY reset, Ethernet driver behavior, packet I/O, networking,
  sockets, SSH, Phase 12.2, and phase transition remain future explicit tasks.
- not-an-issue: no hardwareTestLock was acquired because this task is
  source/docs/evidence only.
- removed: no obsolete implementation, docs, or evidence was removed.

## Rejected Claims And Retained Risks

Rejected claims:

- runtime NCR.MPE write/readback/restore success by this task;
- MAN writes or runtime MDIO transaction;
- PHY-ID reads;
- broad MDIO/PHY ownership;
- PHY reset or GPIO32 ownership;
- Ethernet driver readiness;
- interrupt delivery/completion;
- DMA/descriptors;
- packet I/O;
- networking;
- sockets;
- SSH;
- Phase 12.2;
- phase transition.

Retained risks:

- The future proof may classify target-inconclusive if MACB/GEM NCR visibility
  is not capture-clean at 0x1c00100000.
- Setting MPE may be necessary but not sufficient for visible PHY-ID reads.
- GPIO32/ETH_RST_N, PHY reset state, link state, DMA/descriptors, interrupts,
  packet I/O, sockets, SSH, and Phase 12.2 readiness remain unaccepted.

## Evidence

- Classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-mpe-enable-source-contract/classification.json.
- Evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-mpe-enable-source-contract/evidence-map.json.
- Accepted MDIO PHY-ID closeout:
  tasks/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-proof-closeout.md.
- Retained Linux MACB source:
  tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-macb_main.c.
- Retained MACB register source:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-source-contract/source/linux-rpi-6.12-macb.h.
- Project docs:
  docs/src/project/phase12-networking-ssh.md.

## Validation

- static inspection: accepted MDIO PHY-ID proof/closeout records, retained
  Linux MACB/RP1 source evidence, rp1_ethernet source, Phase 12 docs, roadmap,
  and git history reviewed.
- JSON validation: jq empty on task-owned classification/evidence-map JSON.
- diff check: git diff --check.
- documentation build: /home/node/.cargo/bin/mdbook build.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Contract selects one exact NCR.MPE write/readback/restore target and restore
  invariant: satisfied.
- Contract names observed-window NCR target, MPE bit, precondition handling,
  restore baseline, allowed classifications, and paired no-MDIO/no-Ethernet
  control shape: satisfied.
- Contract explicitly rejects MAN writes, PHY-ID reads, broad MDIO/PHY
  ownership, PHY reset/GPIO32 ownership, Ethernet driver readiness, packet I/O,
  networking, sockets, SSH, Phase 12.2, and phase transition: satisfied.
- Same-shaped MDIO PHY-ID proof retries remain closed until NCR.MPE ownership
  is separately accepted: satisfied.
- Accepted source contract is committed before any follow-up starts: satisfied
  by the commit recorded in supervisor state after this task.

## Next Action

Mechanically promote
phase12-rp1-ethernet-mdio-mpe-enable-guard-core-20260611 on the next worker
wake if dependencies remain satisfied. Keep that task local/static only; do
not run hardware, acquire hardwareTestLock, perform runtime writes, construct
or write MAN frames, retry PHY-ID reads, touch GPIO32/PHY reset, implement
Ethernet, DMA, interrupts, packet I/O, networking, sockets, SSH, Phase 12.2,
or a phase transition.
