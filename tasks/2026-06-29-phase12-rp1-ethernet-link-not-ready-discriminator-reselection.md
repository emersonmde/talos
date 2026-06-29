# Phase 12.1 RP1 Ethernet Link-Not-Ready Discriminator Reselection

Task id: phase12-rp1-ethernet-link-not-ready-discriminator-reselection-20260629

Status: accepted

Classification:
rp1-ethernet-link-not-ready-reselection-paused-no-defensible-discriminator

Evidence level: static source/task/docs/evidence review, task-owned JSON
evidence, docs build, and diff checks. No runtime implementation, Pi 5
hardware run, hardwareTestLock acquisition, lab mutation, boot archive
publication, power-cycle, TFTP/serial capture, packet I/O, live networking,
SSH, OpenSSH retry, generated-root retry, fake command expansion, persistence
claim, or phase transition was performed.

## Goal

Recheck the retained Phase 12.1 BCM54213PE timeout/link-not-ready frontier after
the accepted local POSIX/VFS readiness checkpoint and select exactly one
source/evidence-backed discriminator only if the retained evidence makes it
mechanically objective.

## Scope Performed

- Reviewed the accepted local POSIX/VFS to network readiness checkpoint and
  confirmed it changes no RP1 Ethernet, PHY, MAC, packet, hardware, or live
  networking fact.
- Reviewed the accepted post-master-mode autoneg pause, selected
  link-not-ready closeout, lifecycle ownership closeout, and link-ready source
  contract blocker.
- Reconciled the current source/evidence set against candidate discriminator
  families: same-shaped status/restart/poll/capture retries, GPIO32/ETH_RST_N
  reset ownership, RGMII delay/TX-order, MII_CTRL1000 master-mode plus BMCR
  restart, BMCR powerdown-exit lifecycle gate, APD/EEE/lifecycle, interrupt,
  MAC/phylink, host packet substrate, live packet I/O, and networking/SSH.
- Selected no discriminator. The accepted evidence still requires supervisor
  planning or new source evidence before any local core, hardware proof, packet
  I/O, or networking/SSH task is mechanically unblocked.

## Findings

- fixed: the local POSIX/VFS readiness checkpoint is reconciled as local-only
  descriptor/VFS/userspace evidence. It does not change the retained Ethernet
  pause frontier.
- fixed: retained hardware-visible Ethernet facts remain explicit:
  MII_CTRL1000 master-mode write/readback was accepted, exactly one BMCR
  autoneg restart was accepted, the BMCR_PDOWN exit lifecycle gate observed
  no-change because BMCR_PDOWN was already clear, and terminal link evidence
  remained not ready.
- blocked: GPIO32 / ETH_RST_N reset ownership remains blocked by accepted
  persistent-or-firmware-owned GPIO32 event-state evidence.
- rejected: same-shaped BMCR restart, BMSR/autoneg/MACB_NSR status polling,
  convergence wait tuning, marker-only capture retries, generated-root retry,
  and OpenSSH/live-reachability retries are not feature progress from this
  frontier.
- rejected: RGMII delay/TX-order, MII_CTRL1000 master-mode, BMCR autoneg
  restart, and BMCR_PDOWN-exit lifecycle paths are already exercised or closed
  without accepting link-ready or packet-readiness.
- deferred: APD/EEE/IDDQ, soft reset, suspend/resume, broader lifecycle replay,
  interrupt mask/control, and MAC/phylink work remain possible only under a
  future explicit ownership slice with side-effect, restore, terminal
  classification, and hardware gates.
- not-an-issue: host/QEMU-substitute packet adapter evidence is useful
  substrate work, but it is not RP1/BCM54213PE link evidence and does not
  unblock live packet I/O.
- removed: no source, helper, task, docs, or evidence file was removed.

## Decision

Selected discriminator: null.

Selected next task: null.

Planning needed: true.

Planning reason: The retained Phase 12.1 evidence does not identify a narrow,
source-backed, qualitatively distinct link-not-ready discriminator. Accepted
local POSIX/VFS readiness removes the local shell/redirection detour, but it
does not create new Ethernet source evidence or authorize a broader hardware
ownership slice. The next step requires supervisor planning around either new
source evidence or explicit authorization for a broader APD/EEE/lifecycle,
interrupt, GPIO32 reset, or MAC/phylink ownership slice.

## Candidate Disposition Map

- same-shaped BMCR restart/status/autoneg/convergence/capture retry:
  rejected because it repeats accepted timeout/link-not-ready evidence.
- GPIO32 / ETH_RST_N reset ownership: blocked by persistent-or-firmware-owned
  GPIO32 event-state evidence.
- RGMII delay/TX-order: rejected because it was already exercised and closed.
- MII_CTRL1000 master-mode plus BMCR autoneg restart: rejected because the
  accepted proof ended with BMSR link false, BMSR autoneg-complete false,
  ANLPAR 0x0000, MII_STAT1000 0x0000, and MACB_NSR_LINK false.
- BMCR_PDOWN exit lifecycle gate: rejected as a next discriminator because the
  accepted proof observed BMCR_PDOWN already clear and retained no-change
  link-not-ready evidence.
- APD/EEE/IDDQ/soft reset/suspend-resume/broader lifecycle replay: deferred
  pending an explicit broader ownership/restore contract.
- Broadcom ISR/WOL status reads and interrupt handler state machine: rejected
  for this task because retained source evidence classifies them as
  side-effecting acknowledgement or interrupt ownership surfaces.
- IMR/ECR interrupt mask/control: deferred pending interrupt ownership and
  paired restore rules.
- MAC/phylink: deferred because it is broader MAC/driver ownership while PHY
  link-ready and packet-readiness remain unaccepted.
- host DriverPacketAdapter packet substrate: not-an-issue for this decision;
  it is host/QEMU-substitute packet plumbing, not physical link evidence.
- live packet I/O, networking, SSH, OpenSSH retry, generated-root retry, and
  phase transition: rejected from this task.

## Evidence

- Local POSIX/VFS readiness checkpoint:
  tasks/2026-06-29-phase12-local-posix-vfs-to-network-readiness-checkpoint.md.
- Post-master-mode autoneg pause closeout:
  tasks/2026-06-18-phase12-rp1-ethernet-bcm54213pe-post-master-mode-autoneg-pause-closeout.md.
- Selected link-not-ready closeout:
  tasks/2026-06-18-phase12-rp1-ethernet-bcm54213pe-selected-link-not-ready-closeout.md.
- Master-mode autoneg closeout:
  tasks/2026-06-18-phase12-rp1-ethernet-bcm54213pe-master-mode-autoneg-closeout.md.
- Link-ready discriminator blocker:
  tasks/2026-06-21-phase12-rp1-ethernet-link-ready-discriminator-source-contract.md.
- Lifecycle ownership closeout:
  tasks/2026-06-21-phase12-rp1-ethernet-bcm54213pe-lifecycle-ownership-closeout.md.
- Task classification:
  tasks/evidence/2026-06-29-phase12-rp1-ethernet-link-not-ready-discriminator-reselection/classification.json.
- Task evidence map:
  tasks/evidence/2026-06-29-phase12-rp1-ethernet-link-not-ready-discriminator-reselection/evidence-map.json.

## Acceptance Check

- Every reviewed discriminator candidate has a disposition: satisfied.
- No selected discriminator is set; selected_next_task=null and
  planningNeeded=true are recorded with a concrete reason: satisfied.
- Task record rejects live networking/SSH, packet I/O, hardware proof, and
  phase transition claims: satisfied.
- Source/evidence map lists retained task records, docs, and source/evidence
  owners used to choose or reject discriminators: satisfied.

## Validation

- git status --short --branch before edits: pass; main ahead of origin with no
  uncommitted Talos changes.
- static source/task/docs/evidence review: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass; search index size warning retained.
- git diff --cached --check: pass before commit.

## Next Action

Supervisor planning is required. Do not promote
phase12-rp1-ethernet-selected-discriminator-local-core-20260629,
phase12-rp1-ethernet-selected-discriminator-pi5-proof-20260629, hardware,
packet I/O, live networking/SSH, OpenSSH retry, generated-root retry, or phase
transition work from this task.
