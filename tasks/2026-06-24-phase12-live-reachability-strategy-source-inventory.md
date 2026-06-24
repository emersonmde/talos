# Phase 12 Live Reachability Strategy Source Inventory

Task id: phase12-live-reachability-strategy-source-inventory-20260624
Status: accepted
Owner: worker
Classification: live-reachability-strategy-inventory-paused-no-narrow-discriminator

## Goal

Choose the next defensible live Ethernet/TCP reachability strategy after
selected-byte boot service and a gated OpenSSH attempt reached no-tcp-connect,
or record why no source-bounded discriminator is ready for worker execution.

## Reviewed Inputs

- memory/talos-supervisor-state.json current task and queued follow-ups.
- tasks/2026-06-24-phase12-ssh-tftp-capture-invariant-closeout.md.
- tasks/2026-06-24-phase12-ssh-live-openssh-preclient-fetch-gated-discriminator-v10.md.
- tasks/2026-06-24-phase12-rp1-ethernet-live-reachability-source-reconciliation.md.
- tasks/2026-06-24-phase12-network-frontier-pause-and-local-capability-selection.md.
- tasks/2026-06-21-phase12-rp1-ethernet-link-ready-discriminator-source-contract.md.
- tasks/2026-06-21-phase12-rp1-ethernet-bcm54213pe-lifecycle-ownership-closeout.md.
- tasks/2026-06-21-phase12-network-driver-packet-adapter-closeout.md.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.
- src/rp1_ethernet.rs.
- src/target/rpi5.rs.

## Execution Summary

This task performed static source, task, docs, and evidence review only. It
did not change runtime code, acquire hardwareTestLock, publish a boot archive,
mutate lab state, power-cycle the Pi 5, launch OpenSSH, perform packet I/O,
run ping, or claim remote receipt, compatibility, phase transition, or
ssh-ready=true.

The accepted live frontier is still selected-byte TFTP service followed by one
bounded OpenSSH client attempt that reached no-tcp-connect/tcp-timeout. That
evidence proves the boot tree was served before the client attempt and that
the blocker is below the SSH protocol path, but it does not add new RP1,
BCM54213PE, MACB/GEM, GPIO32, interrupt, DMA, or live packet ownership.

The accepted local foundations remain already satisfied: descriptor-backed
read-only initramfs/VFS file I/O, open/read syscall-shaped surface,
VFS-backed program loading, initial userspace launch, VFS-backed shell exec
and status, stdio descriptor inheritance, pipelines, redirection, volatile VFS
writes, generated-root transport, Pi 5 firmware-initramfs generated-root
consumption, and Pi 5 generated-root command-input success. They are not
replanned here.

The reviewed live-reachability strategy surfaces all remain either blocked by
accepted source/evidence, too broad for a worker-selected discriminator, or
host/substitute-only. No selected_strategy is defensible from the current
accepted evidence set.

## Accepted Frontier Map

| Frontier | Accepted Evidence | Current Limit |
| --- | --- | --- |
| Selected-byte TFTP service | phase12-ssh-live-openssh-preclient-fetch-gated-discriminator-v10-20260624 and phase12-ssh-tftp-capture-invariant-closeout-20260624 | Proves selected kernel bytes were served in-window before the client attempt; does not prove TCP reachability. |
| OpenSSH live attempt | phase12-ssh-live-openssh-preclient-fetch-gated-discriminator-v10-20260624 | Public result is no-tcp-connect/tcp-timeout; no remote receipt, compatibility, PTY/SCP/SFTP, or ssh-ready claim. |
| Host/model network and SSH surfaces | phase12-network-driver-packet-adapter-closeout-20260621 and prior Phase 12 host/QEMU-substitute diagnostics | Source/unit and host/QEMU-substitute evidence only; no live RP1/GEM RX/TX, DMA, interrupt, packet scheduling, or hardware reachability. |
| BCM54213PE hardware frontier | phase12-rp1-ethernet-bcm54213pe-lifecycle-ownership-closeout-20260621 and phase12-rp1-ethernet-link-ready-discriminator-source-contract-20260621 | MII_CTRL1000 master-mode write/readback, one BMCR autoneg restart, and BMCR_PDOWN-exit gate are accepted, but link remains not ready. |
| Local POSIX/VFS/userspace shell foundations | docs/src/roadmap.md current status chain through Phase 8/10 generated-root command-input success | Already accepted prerequisites; not a basis for fake command expansion or a generated-root retry. |

## Candidate Strategy Disposition

| Candidate Strategy | Source Ownership | Side Effects And Restore | Decisive Evidence Needed | Disposition |
| --- | --- | --- | --- | --- |
| BCM54213PE APD/EEE/interrupt/lifecycle ownership | Retained Linux/Broadcom source evidence exists, and the lifecycle closeout accepted only a BMCR_PDOWN exit gate. APD, EEE/MMD, WOL/IDDQ, suspend/resume, interrupt mask/control, and Broadcom selector surfaces remain classified as separate ownership domains. | Mostly side-effecting PHY writes or interrupt acknowledgements. Restore semantics and terminal classifications would have to be selected before implementation; existing accepted proof ended link-not-ready after no-change BMCR_PDOWN exit. | A source contract that chooses one register/bit sequence, proves forbidden selector/interrupt side effects are absent or restored, and defines link-not-ready/link-ready/capture/source-precondition terminals. | deferred. This is not a worker-selectable narrow discriminator from the current evidence set. |
| GPIO32 / ETH_RST_N reset ownership | src/rp1_ethernet.rs records source-backed GPIO32 route, polarity, preconditions, write/restore operation sequence, and future classifications, but the accepted link-ready source contract records GPIO32 reset ownership as blocked by persistent-or-firmware-owned event-state evidence. | Would assert/deassert active-low ETH_RST_N through GPIO32 bank1 bit 4 and must restore STATUS/CTRL/RIO/pad fields. Existing guardrails require complete baseline, no unexpected event state, and paired no-GPIO control. | Fresh source/strategy authorization resolving the persistent event-state blocker, then a source contract/proof chain with candidate identity, serial freshness, TFTP delta, final identity, and restore evidence. | blocked. It is narrow in shape but not mechanically defensible until the accepted event-state blocker is resolved or explicitly overridden by supervisor/Matthew strategy. |
| MAC/phylink/RP1 GEM RX/TX coupling | RP1 GEM identity and packet adapter substrate are accepted below live packet I/O, but no accepted source contract owns MAC configuration, phylink state, DMA descriptors, interrupts, or live RX/TX scheduling. | Would likely cross MACB/GEM configuration, DMA rings, interrupt/polling, packet buffer ownership, and stack scheduling. Restore and fail-closed rules are not selected. | A broader supervisor-planned ownership slice that names exact MAC/GEM registers or no-write coupling points, allowed side effects, DMA/interrupt policy, and live packet terminal classifications. | deferred/rejected for this task. It is too broad to select as the next source-bounded discriminator. |
| Same-shaped OpenSSH, generic link-ready, packet I/O, ping, or generated-root retry | Accepted tasks already classify these as stale or out of layer for the current blocker. | Would repeat prior no-tcp-connect, link-not-ready, host-only, or already accepted generated-root evidence. | None from current evidence; a changed invariant would be required. | rejected. |
| Explicit live-network pause | Owned by supervisor planning rather than worker execution. | No hardware or lab side effects. | Supervisor/Matthew selects a broader ownership strategy or supplies new source evidence that removes a named blocker. | selected as the only safe outcome of this inventory. |

## Findings And Disposition

- fixed: mapped the accepted selected-byte TFTP service and
  no-tcp-connect/tcp-timeout frontier without expanding it to reachability,
  remote receipt, compatibility, phase transition, or ssh-ready=true.
- fixed: preserved host/model network and SSH surfaces as source/unit or
  host/QEMU-substitute evidence only; they remain below live RP1 hardware
  reachability.
- fixed: preserved the accepted local POSIX/VFS/userspace/shell foundation map
  so Phase 8/10 prerequisites and generated-root command-input success are not
  replanned or retried.
- blocked: selected_strategy remains null because GPIO32/ETH_RST_N reset is
  still blocked by accepted persistent-or-firmware-owned event-state evidence.
- deferred: BCM54213PE APD/EEE/interrupt/lifecycle ownership and MAC/phylink
  or RP1/GEM RX/TX coupling require supervisor-planned source contracts with
  side-effect and restore rules before implementation.
- rejected: same-shaped OpenSSH retry, generic link-ready polling, packet I/O,
  ping reachability, remote receipt, compatibility, phase transition,
  ssh-ready=true, generated-root command-input retry, and fake command
  expansion remain unaccepted.
- not-an-issue: no task-owned JSON evidence was required for this static
  inventory.
- removed: no source, helper, docs, task, or evidence files were removed.

## Decision

Selected strategy: null.

Selected next task: null.

Planning needed: true.

Planning reason: the accepted no-tcp-connect result identifies live
Ethernet/TCP reachability as the missing layer, but the current accepted source
and evidence set does not provide a narrow worker-executable discriminator.
GPIO32/ETH_RST_N reset is narrow but blocked by accepted event-state evidence;
BCM54213PE APD/EEE/interrupt/lifecycle and MAC/phylink/RP1 GEM RX/TX coupling
are broader ownership slices requiring supervisor/Matthew strategy selection or
new source evidence before worker execution.

The queued
phase12-live-reachability-selected-source-contract-core-20260624 task is not
mechanically unblocked because this task selected no non-null strategy.

Packet I/O, ping reachability, OpenSSH retry, remote receipt, compatibility,
phase transition, ssh-ready=true, generated-root command-input retry, and fake
command expansion remain rejected.

## Validation

- static source/task/docs/evidence review: pass.
- git status --short --branch: pass, clean/ahead before edits.
- jq empty on task-owned JSON evidence: conditional skip, no task-owned JSON
  evidence was created.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: conditional skip, no docs/src files were
  touched.
- git diff --cached --check before commit: pass.

Evidence levels: static source/task/docs/evidence review and diff checks.

## Acceptance

Accepted as live-reachability-strategy-inventory-paused-no-narrow-discriminator.

selected_strategy=null.
selected_next_task=null.
planningNeeded=true.

No packet I/O, ping reachability, OpenSSH retry, remote receipt,
compatibility, phase transition, ssh-ready=true, generated-root command-input
retry, or fake command expansion is accepted.
