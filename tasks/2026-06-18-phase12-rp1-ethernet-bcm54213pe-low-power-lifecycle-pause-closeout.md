# Phase 12.1 RP1 Ethernet BCM54213PE Low-Power Lifecycle Pause Closeout

Task id: phase12-rp1-ethernet-bcm54213pe-low-power-lifecycle-pause-closeout-20260618

Status: accepted

Classification:
bcm54213pe-low-power-lifecycle-frontier-paused-no-distinct-discriminator

Evidence level: static/task/evidence consistency review, task-owned JSON
evidence, docs build, and diff checks. No runtime implementation, Pi 5 hardware
run, boot archive publication, lab mutation, hardwareTestLock acquisition,
GPIO32/PHY reset action, APD/EEE/lifecycle write, interrupt
mask/unmask/acknowledgement action, MAC/phylink work, packet I/O, networking,
sockets, SSH, Phase 12.2, phase transition, or same-shaped retry was performed.

## Goal

Close the BCM54213PE low-power/lifecycle source-checkpoint frontier after no
source-backed discriminator was selected, preserving the exact accepted Ethernet
state and keeping hardware/networking work gated behind an explicit strategy
decision checkpoint.

## Scope Performed

- Reconciled the accepted low-power/lifecycle source checkpoint with the
  accepted post-master-mode/autoneg pause closeout and Pi 5 proof frontier.
- Preserved the accepted hardware facts: MII_CTRL1000 master-mode
  write/readback and one BMCR autoneg restart are hardware visible, while BMSR
  link, BMSR autoneg-complete, ANLPAR, MII_STAT1000, and MACB_NSR_LINK remain
  not ready.
- Preserved the source-checkpoint finding that APD/powerdown, EEE/MMD,
  interrupt-context, WOL/IDDQ, suspend/resume, BMCR powerdown/soft reset,
  config_init ordering, MAC/phylink-adjacent, and same-shaped polling
  candidates do not produce a mechanically unblocked discriminator.
- Recorded selected_discriminator=null, selected_next_task=null,
  planningNeeded=true, and
  next_strategy_checkpoint=phase12-rp1-ethernet-strategy-decision-checkpoint-after-low-power-lifecycle-20260618.
- Updated visible Phase 12 and roadmap docs with the paused low-power/lifecycle
  frontier and blocked strategy decision checkpoint.

## Findings

- fixed: the accepted low-power/lifecycle source checkpoint is closed into an
  explicit paused Phase 12.1 frontier with no selected discriminator.
- fixed: roadmap and Phase 12 docs now name the low-power/lifecycle pause
  closeout as the latest visible Phase 12.1 Ethernet frontier.
- deferred: APD/auto-power-down, EEE/Clause 45 MMD, WOL/IDDQ, suspend/resume,
  BMCR powerdown recovery, soft reset, and broader config_init replay remain
  lifecycle write/restore surfaces requiring a future ownership and restore
  contract.
- rejected: MII_BCM54XX_ISR, WOL status, and handler-context candidates are not
  bounded discriminators because retained source classifies them as
  read-with-side-effect or interrupt state-machine ownership.
- deferred: IMR/ECR interrupt mask/control paths require interrupt ownership,
  paired restore, and terminal classification rules before any implementation
  task can be selected.
- deferred: MAC/phylink remains broader future work because this frontier
  accepts no link-ready, packet-readiness, or MAC ownership evidence.
- rejected: same-shaped BMCR/BMSR/ANAR/ANLPAR/MII_CTRL1000/MII_STAT1000/
  MACB_NSR polling repeats accepted timeout/link-not-ready evidence and does not
  unblock a new task.
- rejected: hardware, GPIO32/PHY reset action, packet I/O, networking, sockets,
  SSH, Phase 12.2, and phase transition are outside this paused frontier.
- removed: no source, helper, task, docs, or evidence files were removed.
- not-an-issue: no hardware lock, boot publication, or inconclusive-run triage
  was required because this was a static closeout task.

## Decision

Selected discriminator: null.

Selected next task: null.

Planning needed: true.

Next strategy checkpoint:
phase12-rp1-ethernet-strategy-decision-checkpoint-after-low-power-lifecycle-20260618.

The accepted low-power/lifecycle source checkpoint found no BCM54213PE candidate
that is simultaneously source-backed, qualitatively distinct from the accepted
timeout/link-not-ready polling, and safe to run without new lifecycle,
interrupt, MAC/phylink, or restore ownership. This closeout therefore freezes
the current Phase 12.1 Ethernet frontier and requires explicit supervisor/human
strategy selection before any future hardware, networking, or selected-core work.

## Evidence

- Low-power/lifecycle source checkpoint:
  tasks/2026-06-18-phase12-rp1-ethernet-bcm54213pe-low-power-lifecycle-source-checkpoint.md.
- Low-power/lifecycle source-checkpoint classification:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-low-power-lifecycle-source-checkpoint/classification.json.
- Low-power/lifecycle source inspection:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-low-power-lifecycle-source-checkpoint/source-inspection.json.
- Post-master-mode/autoneg pause closeout:
  tasks/2026-06-18-phase12-rp1-ethernet-bcm54213pe-post-master-mode-autoneg-pause-closeout.md.
- Master-mode autoneg Pi 5 proof:
  tasks/2026-06-18-phase12-rp1-ethernet-bcm54213pe-master-mode-autoneg-pi5-proof.md.
- Task classification:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-low-power-lifecycle-pause-closeout/classification.json.
- Task evidence map:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-low-power-lifecycle-pause-closeout/evidence-map.json.

## Acceptance Check

- Task record summarizes the accepted input frontier and the
  low-power/lifecycle source-checkpoint finding that no distinct discriminator
  was selected: satisfied.
- Findings from the accepted checkpoint are preserved with dispositions and
  side-effect classifications rather than re-opened as implementation work:
  satisfied.
- Task-owned classification JSON records selected_discriminator=null,
  selected_next_task=null, planningNeeded=true, next_strategy_checkpoint, and a
  pause rationale: satisfied.
- Roadmap and Phase 12 docs record the paused Phase 12.1 frontier after the
  low-power/lifecycle checkpoint: satisfied.
- Hardware, packet I/O, networking, sockets, SSH, Phase 12.2, phase transition,
  and same-shaped retry work remain explicitly rejected: satisfied.

## Validation

- static/task/evidence consistency review: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Stop for supervisor/human strategy planning. Do not promote
phase12-rp1-ethernet-bcm54213pe-low-power-lifecycle-selected-core-20260618,
phase12-rp1-ethernet-bcm54213pe-low-power-lifecycle-selected-pi5-proof-20260618,
hardware, GPIO32/PHY reset, APD/EEE/lifecycle, interrupt, MAC/phylink, packet
I/O, networking, sockets, SSH, Phase 12.2, or phase-transition work from this
closeout.
