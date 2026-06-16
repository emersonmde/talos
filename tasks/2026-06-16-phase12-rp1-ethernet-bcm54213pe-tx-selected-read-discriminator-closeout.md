# Phase 12.1 RP1 Ethernet BCM54213PE TX Selected Read Discriminator Closeout

Task id: phase12-rp1-ethernet-bcm54213pe-tx-selected-read-discriminator-closeout-20260616

Status: accepted

Classification: tx-selected-register-read-visible-frontier-closed

Evidence level: static/task evidence inspection, accepted local/static
discriminator core review, accepted serialized Pi 5 proof review, JSON evidence
validation, docs build, and diff checks. No new Pi 5 hardware run, boot archive
publication, lab mutation, hardwareTestLock acquisition, power-cycle,
TFTP/serial capture, runtime code change, GPIO32/PHY reset, TX delay write,
BMCR restart, interrupt ownership, packet I/O, networking, SSH, Phase 12.2, or
phase transition was performed.

## Goal

Close out the TX selected-register read discriminator by reconciling the
accepted local/static core and serialized Pi 5 proof. Record the accepted
frontier without broadening it into TX delay write/readback, BMCR restart, link
readiness, packet readiness, networking, SSH, or Phase 12.2.

## Scope Performed

- Inspected the accepted TX selected-register read local/static discriminator
  core, including candidate/control boot scenarios, rejected claim set, and
  allowed terminal classifications.
- Inspected the accepted serialized Pi 5 proof task, classification JSON,
  capture summary, evidence map, selected-tree/TFTP evidence, serial freshness
  evidence, final identity, and restore proof.
- Updated Phase 12 project docs and roadmap with the closed TX selected-register
  read visibility frontier.
- Preserved all rejected TX delay write/readback, BMCR restart, convergence
  polling, link readiness, GPIO32/PHY reset, interrupt, packet, networking,
  SSH, Phase 12.2, and phase-transition claims.

## Findings

- fixed: the local/static core made the discriminator visibly distinct from the
  prior broad RGMII delay proof by performing only the PHY1 TX shadow selector
  write plus selected TX shadow read, while recording tx-delay-write-count=0x0
  and bmcr-write-count=0x0.
- fixed: the paired no-MDIO/no-Ethernet control retained the same reporting and
  capture shape while constructing no MDIO, MAN, MACB, GPIO32, PHY, interrupt,
  packet, networking, SSH, or phase-transition target facts.
- fixed: the serialized Pi 5 proof retained decisive selected-tree identity,
  same-power-cycle TFTP byte agreement, cursor-nonce serial freshness, final
  pre-restore identity, capture-chain-v4 replay, serial freshness guard v1
  replay, and post-run baseline restore proof for both control and candidate.
- fixed: the candidate reached the exact selected-register read boundary: NCR
  before/after 0x10, TX selector write value 0x0c00, selector write count 0x1,
  selected TX read raw 0x0e00, and tx-selected-read-completed=true.
- fixed: the accepted candidate retained rx-delay-write-count=0x0,
  tx-delay-write-count=0x0, and bmcr-write-count=0x0, so this closeout does not
  accept TX delay write/readback or BMCR restart.
- selected: the next mechanically dependency-satisfied boundary is the queued
  post-TX selected-read source checkpoint. It may inspect whether the next
  objective step is TX delay write/readback resume, source-contract correction,
  or explicit pause, but it does not authorize hardware by itself.
- deferred: TX delay write/readback proof, BMCR restart after delay
  configuration, convergence polling, GPIO32/PHY reset ownership, interrupts,
  packet I/O, networking, sockets, SSH, Phase 12.2, and phase transition require
  separate explicit tasks.
- rejected: no link readiness, packet readiness, Ethernet driver readiness,
  networking, sockets, SSH, Phase 12.2, or phase transition is accepted.
- removed: no task-owned source, script, docs, or evidence files were removed.
- not-an-issue: no inconclusive-run triage or additional hardware run was needed
  because the accepted proof evidence was decisive.

## Reconciliation

The accepted TX selected-register proof is a narrow feature attempt against the
first failing layer from the RGMII delay proof. It does not retry the full RGMII
delay path. The candidate writes only the TX shadow selector value 0x0c00 and
then reads the selected TX shadow register through the accepted MAN read frame.
The paired control proves the report/capture path without constructing target
facts.

The serialized Pi 5 proof satisfies the capture chain and closes the specific
read-visibility blocker: the selected TX shadow register is readable and the
observed raw value is 0x0e00. Since the discriminator intentionally stopped
before TX delay write/readback, BMCR restart, and convergence polling, no
link-ready or link-not-ready packet-readiness follow-up is authorized by this
closeout.

## Frontier

Closed frontier: tx-selected-register-read-visible-frontier-closed.

Accepted: selected-tree identity, same-power-cycle TFTP byte agreement,
cursor-nonce serial freshness, final identity, restore proof, paired
no-MDIO/no-Ethernet control evidence, TX selector write completion, and selected
TX shadow register read visibility with raw value 0x0e00.

Deferred: TX delay write/readback resume decision, source-contract correction,
explicit pause, BMCR restart, convergence polling, GPIO32/PHY reset ownership,
interrupts, packet I/O, networking, sockets, SSH, Phase 12.2, and phase
transition.

Not accepted: RX delay write/readback from this discriminator, TX delay
write/readback success, BMCR restart after delay configuration, link readiness,
link-not-ready after the full delay path, Ethernet driver readiness, packet
behavior, networking, sockets, SSH, Phase 12.2, or a phase transition.

## Next Boundary

The queued
phase12-rp1-ethernet-bcm54213pe-post-tx-selected-read-source-checkpoint-20260616
task is mechanically dependency-satisfied once this closeout is committed,
provided hardwareTestLock remains unlocked/restored, supervisorIntervention is
inactive, and projects/talos is clean. That checkpoint must remain
source/task/evidence-only unless it explicitly selects a later local/static
implementation follow-up. It does not authorize hardware, TX delay write,
BMCR restart, packet I/O, networking, SSH, Phase 12.2, or a phase transition.

## Evidence

- TX selected-register discriminator core:
  tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-tx-selected-read-discriminator-core.md.
- Core classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-tx-selected-read-discriminator-core/classification.json.
- TX selected-register discriminator Pi 5 proof:
  tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-tx-selected-read-discriminator-pi5-proof.md.
- Pi 5 proof classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-tx-selected-read-discriminator-pi5-proof/classification.json.
- Pi 5 proof capture summary:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-tx-selected-read-discriminator-pi5-proof/capture-summary.json.
- Pi 5 proof evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-tx-selected-read-discriminator-pi5-proof/evidence-map.json.
- Closeout classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-tx-selected-read-discriminator-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-tx-selected-read-discriminator-closeout/evidence-map.json.

## Acceptance Check

- Closeout reconciles source/core/hardware evidence without broadening
  acceptance beyond what was observed: satisfied.
- Next boundary is explicit, dependency-gated, and preserves feature-led
  planning: satisfied by the queued post-TX selected-read source checkpoint.
- Rejected packet/networking/SSH/Phase 12.2/phase-transition claims remain
  explicit: satisfied.
- Positive TX selected-register read visibility satisfies the conditional
  dependency for the post-TX source checkpoint: satisfied.

## Validation

- static/task evidence inspection: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Mechanically promote
phase12-rp1-ethernet-bcm54213pe-post-tx-selected-read-source-checkpoint-20260616
on the next worker wake if dependencies remain satisfied, hardwareTestLock is
unlocked/restored, supervisorIntervention is inactive, and projects/talos is
clean. Do not start TX delay write/readback, BMCR restart, packet I/O,
networking, SSH, Phase 12.2, or phase-transition work from this closeout.
