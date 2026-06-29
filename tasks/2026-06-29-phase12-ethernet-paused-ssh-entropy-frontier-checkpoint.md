# Phase 12 Ethernet-Paused SSH Entropy Frontier Checkpoint

Task id: phase12-ethernet-paused-ssh-entropy-frontier-checkpoint-20260629

Status: accepted

Classification: ethernet-paused-ssh-entropy-frontier-checkpoint-accepted

Evidence level: static task/docs/evidence review, task-owned JSON evidence,
docs build, and diff checks. No runtime implementation, Pi 5 hardware run,
hardwareTestLock acquisition, lab mutation, boot archive publication,
power-cycle, TFTP/serial capture, packet I/O, live networking, SSH, OpenSSH
retry, generated-root retry, fake command expansion, or phase transition was
performed.

## Goal

Freeze the accepted Phase 12.1 Ethernet pause boundary after the no-discriminator
reselection task and select the next source-only SSH substrate task if it is
bounded and useful without depending on live link-ready hardware.

## Scope Performed

- Reconciled the accepted local POSIX/VFS readiness checkpoint, the accepted
  RP1 Ethernet link-not-ready discriminator reselection, Phase 12 project docs,
  roadmap, and supervisor queue.
- Preserved the Ethernet pause boundary: selected_discriminator remains null,
  and no selected-discriminator local core, Pi 5 hardware proof, packet I/O,
  live networking, OpenSSH retry, generated-root retry, or phase transition is
  unblocked.
- Selected the source-only entropy contract as the next bounded Phase 12 SSH
  substrate task because eventual SSH host keys and session crypto require an
  entropy/key-management boundary before any live SSH acceptance.

## Findings

- fixed: The accepted Ethernet pause is recorded outside transient supervisor
  planning. The retained boundary remains no selected_discriminator, no
  link-ready, no packet I/O, no live networking/SSH, no OpenSSH retry, no
  generated-root retry, no hardware proof, and no phase transition.
- fixed: The selected-discriminator local, hardware, and closeout tasks remain
  dependency-blocked until future accepted source evidence selects a concrete
  discriminator.
- not-an-issue: The accepted local POSIX/VFS readiness frontier is still useful
  substrate evidence, but it does not reopen shell grammar expansion as the next
  feature by itself.
- not-an-issue: SSH entropy/key-management work is source/contract substrate
  for eventual SSH and does not require live RP1 Ethernet link-ready evidence.
- deferred: Live packet I/O, TCP/IP, OpenSSH or another SSH server, host-key
  generation, cryptographic-readiness claims, Pi 5 proof, and phase transition
  remain outside this checkpoint.
- removed: no source, helper, task, docs, or evidence file was removed.

## Decision

Selected next task:
phase12-ssh-entropy-source-contract-20260629.

Planning needed: false.

Reason: The entropy source contract is already queued with explicit scope,
non-goals, dependencies, acceptance criteria, validation gates, docs
requirements, and evidence requirements. It is a source-only substrate task that
does not claim live networking or hardware progress.

## Accepted Boundary

The accepted Ethernet boundary remains:

- selected_discriminator: null.
- selected-discriminator local/hardware/closeout queue: dependency-blocked.
- link-ready, packet I/O, live networking/SSH, OpenSSH retry, generated-root
  retry, boot publication, Pi 5 hardware proof, fake command expansion,
  persistence claim, and phase transition: not accepted.

The accepted next task is source-only:
phase12-ssh-entropy-source-contract-20260629. It may classify entropy input
candidates, define diagnostic shape, and set non-retention rules for secret
material. It must not accept host keys, live SSH, TCP/IP, packet I/O,
cryptographic sufficiency, Pi 5 proof, or a phase transition.

## Evidence

- Ethernet link-not-ready discriminator reselection:
  tasks/2026-06-29-phase12-rp1-ethernet-link-not-ready-discriminator-reselection.md.
- Local POSIX/VFS readiness checkpoint:
  tasks/2026-06-29-phase12-local-posix-vfs-to-network-readiness-checkpoint.md.
- Phase 12 project note:
  docs/src/project/phase12-networking-ssh.md.
- Roadmap:
  docs/src/roadmap.md.
- Task evidence map:
  tasks/evidence/2026-06-29-phase12-ethernet-paused-ssh-entropy-frontier-checkpoint/evidence-map.json.
- Task classification:
  tasks/evidence/2026-06-29-phase12-ethernet-paused-ssh-entropy-frontier-checkpoint/classification.json.

## Acceptance Check

- Findings are recorded with disposition: satisfied.
- Accepted Ethernet pause boundary remains explicit: satisfied.
- Selected-discriminator queue remains dependency-blocked until future source
  evidence selects a discriminator: satisfied.
- Selected next task is phase12-ssh-entropy-source-contract-20260629:
  satisfied.
- Local shell grammar expansion is not reopened as progress: satisfied.

## Validation

- git status --short --branch before edits: pass; main ahead of origin with no
  uncommitted Talos changes.
- static task/docs/evidence review: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass; search index size warning retained.
- git diff --cached --check: pass before commit.

## Next Action

Promote phase12-ssh-entropy-source-contract-20260629 on the next worker wake if
dependencies remain satisfied. Do not promote
phase12-rp1-ethernet-selected-discriminator-local-core-20260629,
phase12-rp1-ethernet-selected-discriminator-pi5-proof-20260629, or
phase12-rp1-ethernet-selected-discriminator-closeout-20260629 until future
accepted source evidence selects a discriminator.
