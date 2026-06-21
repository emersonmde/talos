# Phase 12.1 RP1 Ethernet BCM54213PE Lifecycle Ownership Closeout

Task id: phase12-rp1-ethernet-bcm54213pe-lifecycle-ownership-closeout-20260621

Status: accepted

Classification:
bcm54213pe-lifecycle-ownership-closeout-no-link-ready-planning-needed

Evidence level: static task/docs/evidence review, task-owned JSON evidence,
docs build, and diff checks. No hardwareTestLock acquisition, lab mutation,
boot publication, power cycle, live packet I/O, ping/hardware reachability,
SSH, UDP/raw sockets, libc/std wrappers, public ABI/POSIX/Linux compatibility,
broad socket expansion, or phase transition was performed.

## Goal

Reconcile the accepted BCM54213PE lifecycle ownership source/core and Pi 5 proof
evidence, preserve the exact accepted/rejected claims, and decide whether any
already queued follow-up is mechanically unblocked.

## Scope Performed

- Reviewed the accepted source/core task, the accepted serialized Pi 5 proof,
  task-owned evidence JSON, retained hardware evidence, Phase 12 docs, roadmap,
  hardwareTestLock state, and queued follow-up dependencies.
- Preserved the accepted terminal result:
  bcm54213pe-lifecycle-powerdown-exit-no-change-link-not-ready.
- Recorded that the selected BMCR_PDOWN exit gate did not produce link-ready,
  autoneg-complete, packet-readiness, live RX/TX, packet I/O, networking, SSH,
  Phase 12.2, or phase-transition evidence.
- Set planningNeeded=true because no later queued task is mechanically
  unblocked by a no-change link-not-ready terminal.

## Findings

- fixed: closeout now records the accepted source/core and Pi 5 proof evidence
  as one reconciled Phase 12.1 frontier.
- fixed: hardwareTestLock remained unlocked/restored after the proof, with
  restore evidence retained in the proof task.
- fixed: the accepted hardware-visible BCM54213PE frontier remains
  MII_CTRL1000 master-mode write/readback, one BMCR autoneg restart, and the
  BMCR_PDOWN-exit gate that observed no-change because BMCR_PDOWN was already
  clear.
- rejected: the no-change link-not-ready terminal does not unblock the queued
  generic link-ready discriminator core/proof/closeout chain, because those
  tasks require accepted selected link-ready discriminator/core/proof evidence
  that does not exist.
- rejected: link-ready, autoneg-complete, packet-readiness, live RX/TX, packet
  I/O, ping/hardware reachability, Ethernet driver readiness, networking,
  sockets, SSH, UDP/raw sockets, libc/std wrappers, public ABI/POSIX/Linux
  compatibility, broad socket expansion, Phase 12.2 acceptance, and phase
  transition remain unaccepted.
- deferred: any further Phase 12.1 hardware strategy, any different lifecycle
  ownership sequence, GPIO32 reset action, APD/EEE/IDDQ, interrupt ownership,
  MAC/phylink ownership, packet-readiness follow-up, or return to host-only
  network work requires supervisor planning with explicit task gates.
- removed: no source behavior, task evidence, or helper files were removed.
- not-an-issue: BMCR 0x1000 in the proof differs from the source/core accepted
  context example BMCR 0x1200, but both have BMCR_PDOWN clear, so the accepted
  fail-closed gate performed no clear write and reached an allowed terminal.

## Reconciled Evidence

The accepted source/core selected exactly one hardware sequence:
bcm54213pe-phy1-bmcr-powerdown-exit-gate. It allowed a future candidate to
pre-read PHY1 BMCR, clear only BMCR_PDOWN bit 11 if set, preserve all other
BMCR bits, wait at least 40us after any powerdown exit, and post-sample BMCR,
double-sampled BMSR, ANAR, ANLPAR, MII_CTRL1000, MII_STAT1000, and passive
MACB_NSR_LINK context. It rejected APD, EEE, IDDQ/TOP_MISC, soft reset without
accepted IDDQ prerequisite, interrupt ownership, GPIO32 reset action,
MAC/phylink, live packet I/O, reachability, SSH, broad socket expansion, and
phase transition.

The accepted Pi 5 proof ran the selected control and candidate under serialized
hardwareTestLock. The control classified
no-mdio-no-ethernet-bcm54213pe-lifecycle-ownership-control. The candidate
observed BMCR 0x1000 with BMCR_PDOWN already clear, performed no BMCR clear
write, then retained BMCR 0x1000, BMSR 0x7949/0x7949, ANAR 0x01e1, ANLPAR
0x0000, MII_CTRL1000 0x0200, MII_STAT1000 0x0000, and passive
MACB_NSR_LINK=false.

The accepted terminal classification is
bcm54213pe-lifecycle-powerdown-exit-no-change-link-not-ready. That terminal is
evidence against promoting packet-readiness or the queued generic link-ready
proof chain.

## Evidence

- Classification:
  tasks/evidence/2026-06-21-phase12-rp1-ethernet-bcm54213pe-lifecycle-ownership-closeout/classification.json.
- Evidence map:
  tasks/evidence/2026-06-21-phase12-rp1-ethernet-bcm54213pe-lifecycle-ownership-closeout/evidence-map.json.
- Accepted source/core:
  tasks/2026-06-21-phase12-rp1-ethernet-bcm54213pe-lifecycle-ownership-source-core.md.
- Accepted Pi 5 proof:
  tasks/2026-06-21-phase12-rp1-ethernet-bcm54213pe-lifecycle-ownership-pi5-proof.md.
- Proof restore evidence:
  tasks/evidence/2026-06-21-phase12-rp1-ethernet-bcm54213pe-lifecycle-ownership-pi5-proof/final-post-restore-boot-files.json.

## Validation

- static task/docs/evidence review: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Acceptance Check

- Closeout reconciles accepted source/core evidence, Pi 5 proof classification,
  restore state, docs, deferred work, and risks: satisfied.
- Findings are recorded with dispositions: satisfied.
- No mechanically unblocked next task exists from the no-change link-not-ready
  terminal; planningNeeded=true is recorded with a concrete reason: satisfied.
- No link-ready, packet I/O, reachability, SSH, public ABI/POSIX/Linux
  compatibility, broad expansion, or phase-transition claim is made: satisfied.

## Next Action

Supervisor planning is required before any further Phase 12.1 hardware action,
generic link-ready discriminator promotion, packet I/O, reachability, SSH,
Phase 12.2 acceptance, return to host-only network work, broad expansion, or
phase transition. Do not promote the queued generic link-ready core/proof/
closeout chain from this closeout.
