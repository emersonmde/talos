# Phase 12 RP1 Ethernet BCM54213PE Read-Only Preflight Hardware-Proof Closeout

Task id: phase12-rp1-ethernet-bcm54213pe-readonly-preflight-hw-proof-closeout-20260616

Status: accepted

Classification:
bcm54213pe-readonly-preflight-hw-proof-frontier-closed-candidate-fetch-blocker

Evidence level: static/task evidence inspection, accepted proof-core review,
accepted serialized Pi 5 proof review, JSON evidence validation, docs build,
and diff checks. No new Pi 5 hardware run, boot archive publication, lab
mutation, hardwareTestLock acquisition, power-cycle, TFTP/serial capture,
volatile Ethernet access, GPIO32 event clear/reset recovery, BMCR write,
Broadcom shadow/MMD/aux access, interrupt ownership, PHY/MAC configuration,
packet I/O, networking, SSH, Phase 12.2, or phase transition was performed.

## Goal

Close out the BCM54213PE read-only preflight hardware-proof slice, reconcile the
accepted local/static proof-core and serialized Pi 5 hardware evidence, and set
the next explicit Phase 12.1 boundary.

## Scope Performed

- Inspected the accepted hardware-proof core task, classification JSON, evidence
  map, and local/static candidate/control boot-scenario boundary.
- Inspected the accepted serialized Pi 5 hardware proof task, classification
  JSON, evidence map, capture summary, control evidence, candidate first
  attempt, candidate rerun, and restore evidence.
- Reconciled accepted, deferred, blocked, rejected, and removed findings against
  the prior BCM54213PE read-only preflight source/report closeout.
- Updated Phase 12 project docs and roadmap with the closed hardware-proof
  frontier and the precise candidate power/network-fetch blocker.
- Set supervisor planning as the next action because no explicit queued
  follow-up exists after this hardware-proof closeout.

## Findings

- fixed: the hardware-proof core encoded exactly the accepted candidate targets:
  PHY1 MII_CTRL1000 0x09 with MAN frame 0x60a60000 and PHY1 MII_STAT1000 0x0a
  with MAN frame 0x60aa0000.
- fixed: the proof-core control constructs no MDIO target, MAN frame, MACB
  target, GPIO target, RP1 Ethernet target facts, or volatile Ethernet access
  intent.
- fixed: the serialized Pi 5 control proved the no-MDIO/no-Ethernet control
  shape with selected-tree identity, two matching 50536-byte TFTP fetches,
  fresh serial marker output, boot-staging-identity-ready, and restore proof.
- blocked: the accepted candidate rerun published selected tree
  189219336873dd6f335fd3ad2f97bb20b8cb2f4a01e2635e4f3ae9dd5eacb5c8 with an
  expected 51512-byte kernel_2712.img, but post-power TFTP delta stayed empty
  and serial observe saw no fresh candidate output.
- deferred: candidate raw/decoded MII_CTRL1000 and MII_STAT1000 values remain
  deferred until the no-fresh-TFTP/no-serial candidate blocker is explained or
  a supervisor-planned discriminator selects the failing layer.
- rejected: packet I/O, networking, SSH, Phase 12.2, phase transition, link
  readiness, GPIO32 reset ownership, BMCR writes, Broadcom shadow/MMD/aux
  access, interrupt ownership, and broad PHY/MAC configuration remain rejected.
- removed: generated boot archives were already removed after upload by the
  hardware-proof task; retained static review hashes and lab evidence are the
  durable artifacts.
- not-an-issue: GET / returned 404 in this lab deployment during the hardware
  proof; retained /status and /boot/files identity evidence satisfy the deployed
  lab-controller evidence path.

## Reconciliation

The accepted source/report frontier selected only PHY1 MII_CTRL1000 and
MII_STAT1000 as a pure read-only BCM54213PE preflight set. The accepted
proof-core then added two boot scenarios for that closed target set: a candidate
that may report raw/decoded values for those two registers, and a paired control
that withholds all MDIO/MAN/MACB/GPIO/RP1 Ethernet target facts.

The accepted Pi 5 proof did not reach candidate register evidence. It did
produce a decisive control result and a precise candidate blocker. The control
boot path proves the paired no-MDIO/no-Ethernet report/capture shape. The
candidate rerun proves only that the selected candidate tree remained staged in
lab status while power-cycle produced no fresh TFTP fetches and no fresh serial
output from the saturated cursor. Therefore MII_CTRL1000/MII_STAT1000 values,
link readiness, and any PHY/MAC behavior remain unaccepted.

## Frontier

Closed frontier:
bcm54213pe-readonly-preflight-hw-proof-frontier-closed-candidate-fetch-blocker.

Accepted: the local/static proof-core candidate/control shape and the serialized
Pi 5 control proof for the paired no-MDIO/no-Ethernet path.

Blocked: the candidate hardware path is blocked by no fresh TFTP events or
serial output after selected candidate publication and power-cycle. The blocker
is evidence about the candidate power/network-fetch path, not evidence about
MII_CTRL1000, MII_STAT1000, link readiness, PHY readiness, or packet behavior.

Not accepted: candidate register values, Ethernet driver readiness, link
readiness, GPIO32/PHY reset ownership, BMCR/autoneg retry, Broadcom
shadow/MMD/aux access, interrupt ownership, broad PHY/MAC configuration, packet
I/O, networking, SSH, Phase 12.2, or a phase transition.

## Next Boundary

Supervisor planning is required before any follow-up task starts. The next
decision must select one explicit Phase 12.1 boundary, such as a lab
power/network-fetch discriminator for the selected candidate publication path,
a separate source/static contract, or an explicit pause.

No dependency-gated queued task remains mechanically unblocked after this
closeout. Same-shaped BCM54213PE read-only preflight hardware retries are closed
for this candidate/control pair unless a future supervisor-planned task supplies
a distinct discriminator for the no-fresh-TFTP/no-serial candidate blocker.

## Evidence

- Proof-core task:
  tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-hw-proof-core.md.
- Proof-core classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-hw-proof-core/classification.json.
- Proof-core evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-hw-proof-core/evidence-map.json.
- Serialized Pi 5 proof task:
  tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-pi5-proof.md.
- Serialized Pi 5 proof classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-pi5-proof/classification.json.
- Serialized Pi 5 proof evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-pi5-proof/evidence-map.json.
- Serialized Pi 5 capture summary:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-pi5-proof/capture-summary.json.
- Hardware-proof closeout classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-hw-proof-closeout/classification.json.
- Hardware-proof closeout evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-hw-proof-closeout/evidence-map.json.

## Validation

- static/task evidence inspection: proof-core task/classification/evidence map,
  Pi 5 proof task/classification/evidence map, capture summary, docs, roadmap,
  and git history inspected.
- JSON validation: jq empty on task-owned classification/evidence-map JSON.
- diff check: git diff --check.
- docs validation: mdbook build because docs/src files were touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Closeout reconciles proof-core and hardware evidence, including blocked or
  inconclusive paths: satisfied.
- Rejected claims remain explicit: satisfied.
- Next boundary is explicit: satisfied by planningNeeded=true for supervisor
  selection of a distinct Phase 12.1 follow-up or pause.
- Docs/evidence/state updates are committed before any next task starts:
  satisfied once state is updated after this commit.

## Next Action

Set planningNeeded=true for supervisor planning. Do not start hardware, GPIO32
event clear/reset recovery, BMCR write, Broadcom shadow/MMD/aux access,
interrupt ownership, PHY/MAC configuration, packet I/O, networking, SSH, Phase
12.2, or phase transition from this closeout.
