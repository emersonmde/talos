# Phase 12.6 SSH no-tcp-connect live network substrate checkpoint

Task id: phase12-ssh-no-tcp-connect-live-network-substrate-checkpoint-20260624
Status: accepted
Owner: worker
Classification: no-tcp-connect-requires-live-network-substrate-evidence

## Goal

Checkpoint the accepted v10 SSH no-tcp-connect boundary and select the next
bounded worker task around live network substrate evidence before any further
OpenSSH, remote-receipt, compatibility, phase-transition, or ssh-ready work.

## Reviewed Inputs

- memory/talos-supervisor-state.json current task and taskQueue entries.
- tasks/2026-06-24-phase12-ssh-tftp-capture-invariant-closeout.md.
- tasks/2026-06-24-phase12-ssh-live-openssh-preclient-fetch-gated-discriminator-v10.md.
- tasks/2026-06-24-phase12-ssh-selected-publish-extended-tftp-window-pi5-proof-v10.md.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Execution Summary

This checkpoint performed static task, docs, and evidence review only. It did
not publish a boot archive, acquire hardwareTestLock, power-cycle the Pi,
mutate lab state, launch OpenSSH, implement source, or touch runtime artifacts.

The accepted SSH frontier is now deliberately narrow:

- selected-byte TFTP service is accepted from the v10 selected-publish proof;
- one bounded workspace-local OpenSSH attempt is accepted only after the v10
  same-task pre-client selected-fetch gate passed;
- the public OpenSSH result is no-tcp-connect / tcp-timeout;
- TCP connection establishment, live remote receipt, OpenSSH/POSIX/Linux
  compatibility, PTY/SCP/SFTP, broad command expansion, phase transition, and
  ssh-ready=true remain unaccepted.

Because the live client reached no TCP connection after selected boot service
was proved, another same-shaped OpenSSH retry would not be the smallest useful
feature step. The missing evidence is below SSH: a source-grounded live
Ethernet/network-substrate reachability discriminator that explains whether
Talos can expose a live path capable of TCP connection establishment.

The selected next bounded task is
phase12-rp1-ethernet-live-reachability-source-reconciliation-20260624.

## Evidence Map

- capture invariant closeout:
  tasks/2026-06-24-phase12-ssh-tftp-capture-invariant-closeout.md.
  It accepted capture-invariant-closed-live-openssh-no-tcp-connect and kept
  remote receipt, compatibility, phase transition, and ssh-ready blocked.
- v10 live OpenSSH pre-client gated discriminator:
  tasks/2026-06-24-phase12-ssh-live-openssh-preclient-fetch-gated-discriminator-v10.md
  and
  tasks/evidence/2026-06-24-phase12-ssh-live-openssh-preclient-fetch-gated-discriminator-v10/rerun-clean/live-openssh-preclient-fetch-gated-discriminator-v10.summary.sanitized.json.
  It proved same-task selected-byte TFTP service before launching one bounded
  OpenSSH client attempt and classified the public result as no-tcp-connect /
  tcp-timeout.
- v10 selected-publish extended-window proof:
  tasks/2026-06-24-phase12-ssh-selected-publish-extended-tftp-window-pi5-proof-v10.md
  and
  tasks/evidence/2026-06-24-phase12-ssh-selected-publish-extended-tftp-window-pi5-proof-v10/selected-publish-extended-window-v10.summary.sanitized.json.
  It accepted selected-publish-extended-window-fetch-observed with selected
  87,432-byte da591740/kernel_2712.img service before restore.

## Findings And Disposition

- fixed: converted the accepted no-tcp-connect / tcp-timeout result into an
  explicit network-substrate evidence dependency rather than leaving the
  frontier as an unqualified SSH planning gap.
- fixed: selected the next bounded worker task as
  phase12-rp1-ethernet-live-reachability-source-reconciliation-20260624.
- blocked: another OpenSSH retry, remote receipt, compatibility claim, phase
  transition, or ssh-ready=true claim remains blocked until live-network
  substrate evidence exists.
- deferred: packet I/O, ping reachability, SSH live receipt, and public stable
  ABI claims require later explicit tasks after a source-grounded
  discriminator is selected and proved.
- not-an-issue: no source implementation, hardware action, lab mutation,
  boot-archive publication, or OpenSSH execution was required for this
  checkpoint.

## Validation

- static task/docs/evidence review: pass.
- jq empty on task-owned JSON evidence: conditional skip, no new JSON evidence
  was created.
- redaction review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.

Evidence levels: static task/docs/evidence review, redaction grep review,
docs build, and diff checks.

## Redaction Review

Pass. This task record retains only task ids, public evidence paths, public
kernel byte categories, public OpenSSH phase/exit categories, validation
commands, and classifications. It retains no raw serial text, raw serial
base64, raw TFTP log lines, client identities, user names, addresses, MAC
addresses, OpenSSH logs, known_hosts, host keys, authorized keys, key material,
fingerprints, signatures, session identifiers, channel identifiers, command
bytes, payload bytes, packet captures, boot artifact bytes, stable peer
identifiers, or private user data.

## Acceptance

Accepted as no-tcp-connect-requires-live-network-substrate-evidence.

selected_byte_tftp_service_accepted=true.
bounded_openssh_attempt_after_preclient_gate=true.
openssh_public_phase=no-tcp-connect.
openssh_exit_category=tcp-timeout.
live_tcp_connect_accepted=false.
remote_receipt_accepted=false.
compatibility_accepted=false.
ssh_ready=false.

selected_next_task=phase12-rp1-ethernet-live-reachability-source-reconciliation-20260624.
planningNeeded=false.

No packet I/O, ping reachability, SSH live receipt, OpenSSH/POSIX/Linux
compatibility, public stable ABI acceptance, broad expansion, phase transition,
or ssh-ready=true is accepted.
