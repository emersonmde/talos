# Phase 12.2 RP1 Ethernet Live Reachability Source Reconciliation

Task id: phase12-rp1-ethernet-live-reachability-source-reconciliation-20260624
Status: accepted
Owner: worker
Classification: live-reachability-source-reconciliation-paused-no-defensible-discriminator

## Goal

Select the smallest source-grounded live Ethernet reachability discriminator
after selected boot service was proved but the bounded OpenSSH attempt reached
no TCP connection, or pause if the accepted evidence does not support one.

## Reviewed Inputs

- tasks/2026-06-24-phase12-ssh-no-tcp-connect-live-network-substrate-checkpoint.md.
- tasks/2026-06-24-phase12-ssh-live-openssh-preclient-fetch-gated-discriminator-v10.md.
- tasks/2026-06-24-phase12-ssh-tftp-capture-invariant-closeout.md.
- tasks/2026-06-21-phase12-rp1-ethernet-link-ready-discriminator-source-contract.md.
- tasks/2026-06-21-phase12-rp1-ethernet-bcm54213pe-lifecycle-ownership-closeout.md.
- tasks/2026-06-21-phase12-network-driver-packet-adapter-closeout.md.
- tasks/2026-06-21-phase12-network-frontier-pause-and-ssh-strategy-checkpoint.md.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Execution Summary

This task performed static source, task, docs, and evidence reconciliation
only. It did not acquire hardwareTestLock, publish a boot archive, mutate lab
state, power-cycle the Pi, launch OpenSSH, implement packet I/O, run ping, or
claim remote receipt, compatibility, phase transition, or ssh-ready=true.

The accepted v10 SSH evidence makes the current blocker precise: selected-byte
TFTP service and one bounded OpenSSH attempt after the same-task pre-client
gate are accepted, but TCP connect did not complete. That places the missing
evidence below SSH at live Ethernet/TCP reachability rather than in SSH
protocol modeling or OpenSSH tooling.

The accepted RP1 Ethernet evidence does not yet provide a defensible new live
reachability discriminator. The latest hardware-visible BCM54213PE frontier
still records link-not-ready after MII_CTRL1000 master-mode write/readback,
one BMCR autoneg restart, lifecycle/powerdown-exit inspection, and repeated
BMSR/MII_STAT1000/MACB_NSR samples. The 2026-06-21 link-ready source contract
already rejected the generic link-ready discriminator path with
selected_discriminator=null because GPIO32 reset ownership, interrupt/APD/EEE
lifecycle ownership, MAC/phylink ownership, and same-shaped status/autoneg
polling were not source-bounded. The accepted driver packet adapter, smoltcp,
socket, pingdiag, and sockdiag surfaces remain source/unit or
host/QEMU-substitute evidence only; they do not add RP1/GEM RX/TX coupling,
DMA/interrupt ownership, packet scheduling, hardware reachability, or live link
facts.

Because the no-tcp-connect result identifies the missing layer but does not add
new RP1/BCM54213PE source ownership, selecting the queued live reachability
core would repeat the blocked link-ready path or broaden hardware ownership
without accepted side-effect and restore rules.

## Findings And Disposition

- fixed: recorded that selected-byte TFTP service and a bounded OpenSSH attempt
  are accepted, while live TCP connect, remote receipt, compatibility, phase
  transition, and ssh-ready=true remain unaccepted.
- fixed: identified the missing layer as live Ethernet/TCP reachability below
  SSH, not SSH protocol modeling or OpenSSH client tooling.
- blocked: selected_discriminator remains null because no accepted source
  evidence isolates a narrow live reachability discriminator with side-effect
  rules, restore expectations, and terminal classifications.
- rejected: stale generic link-ready discriminator core/proof/closeout paths
  remain blocked by the accepted 2026-06-21 no-defensible-discriminator source
  contract.
- rejected: another same-shaped OpenSSH retry cannot prove the missing lower
  network substrate and is not selected.
- rejected: packet I/O, ping reachability, SSH live receipt, public socket ABI,
  OpenSSH/POSIX/Linux compatibility, broad expansion, phase transition, and
  ssh-ready=true remain unaccepted.
- deferred: GPIO32/ETH_RST_N reset recovery, interrupt/APD/EEE/lifecycle,
  MAC/phylink configuration, RP1/GEM live RX/TX coupling, and hardware
  reachability require supervisor-planned source contracts or a broader
  explicit strategy decision before implementation.
- not-an-issue: no source implementation, hardware action, boot publication,
  lab mutation, packet I/O, or OpenSSH launch was required for this
  reconciliation task.

## Candidate Disposition Map

- Same-shaped OpenSSH retry: rejected. The accepted public result is
  no-tcp-connect/tcp-timeout after selected-byte TFTP service; retrying the same
  shape does not create live Ethernet/TCP substrate evidence.
- Generic link-ready discriminator: rejected. The accepted 2026-06-21 source
  contract already classified this path as no defensible discriminator.
- BCM54213PE status/autoneg polling: rejected. It repeats accepted terminal
  BMSR link false, BMSR autoneg-complete false, MII_STAT1000 0x0000, and
  MACB_NSR_LINK false evidence.
- GPIO32/ETH_RST_N reset recovery: blocked. Persistent-or-firmware-owned event
  state and no-write guard evidence still block reset ownership.
- Interrupt, APD/EEE, lifecycle, WOL, suspend/resume, and Broadcom selector
  work: deferred. These are side-effecting ownership surfaces needing explicit
  source contracts, restore rules, and hardware gates.
- MAC/phylink or RP1/GEM live RX/TX coupling: deferred. This is broader than a
  discriminator unless supervisor planning defines exact ownership,
  side effects, restore behavior, and terminal classifications.
- DriverPacketAdapter, smoltcp, pingdiag, sockdiag, and socket surfaces:
  not-an-issue as host/substitute plumbing; rejected as live RP1 hardware
  evidence.

## Decision

Selected discriminator: null.

Selected next task: null.

Planning needed: true.

Planning reason: The accepted no-tcp-connect checkpoint establishes that live
Ethernet/TCP reachability is the blocker below SSH, but the accepted RP1
Ethernet and host-network evidence still lacks a source-bounded discriminator
that can be implemented locally before hardware without broadening GPIO32,
interrupt/lifecycle, MAC/phylink, RP1/GEM RX/TX, packet I/O, or link ownership.

The queued
phase12-rp1-ethernet-live-reachability-discriminator-core-20260624 task is not
mechanically unblocked because its dependency requires selected_discriminator
non-null and selected_next_task equal to that core.

## Evidence Map

- No-tcp-connect checkpoint:
  tasks/2026-06-24-phase12-ssh-no-tcp-connect-live-network-substrate-checkpoint.md.
- v10 live OpenSSH pre-client gated discriminator:
  tasks/2026-06-24-phase12-ssh-live-openssh-preclient-fetch-gated-discriminator-v10.md.
- TFTP capture invariant closeout:
  tasks/2026-06-24-phase12-ssh-tftp-capture-invariant-closeout.md.
- Blocked link-ready source contract:
  tasks/2026-06-21-phase12-rp1-ethernet-link-ready-discriminator-source-contract.md.
- Latest BCM54213PE lifecycle closeout:
  tasks/2026-06-21-phase12-rp1-ethernet-bcm54213pe-lifecycle-ownership-closeout.md.
- Driver packet adapter closeout:
  tasks/2026-06-21-phase12-network-driver-packet-adapter-closeout.md.
- Network frontier pause and SSH strategy checkpoint:
  tasks/2026-06-21-phase12-network-frontier-pause-and-ssh-strategy-checkpoint.md.

## Validation

- source/task/docs/evidence consistency review: pass.
- jq empty on task-owned JSON evidence: conditional skip, no task-owned JSON
  evidence was created.
- redaction review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.

Evidence levels: static source/task/docs/evidence review, redaction grep
review, docs build, and diff checks.

## Redaction Review

Pass. This task record retains only task ids, source/evidence boundaries,
public classifications, public byte-size categories already present in task
records, validation commands, and accepted/rejected claim labels. It retains no
raw serial text, raw serial base64, raw TFTP log lines, client identities, user
names, addresses, MAC addresses, OpenSSH logs, known_hosts, host keys,
authorized keys, key material, fingerprints, signatures, session identifiers,
channel identifiers, command bytes, payload bytes, packet captures, boot
artifact bytes, stable peer identifiers, or private user data.

## Acceptance

Accepted as
live-reachability-source-reconciliation-paused-no-defensible-discriminator.

selected_discriminator=null.
selected_next_task=null.
planningNeeded=true.

No packet I/O, ping reachability, hardware reachability, SSH live receipt,
OpenSSH/POSIX/Linux compatibility, public stable ABI acceptance, broad
expansion, phase transition, or ssh-ready=true is accepted.
