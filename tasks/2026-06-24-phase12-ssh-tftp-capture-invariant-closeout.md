# Phase 12.6 SSH TFTP capture invariant closeout

Task id: phase12-ssh-tftp-capture-invariant-closeout-20260624
Status: accepted
Owner: worker
Classification: capture-invariant-closed-live-openssh-no-tcp-connect

## Goal

Close the v9/v10 TFTP capture invariant thread, state the accepted boundary,
and leave the next SSH feature step to supervisor planning without accepting
remote receipt, compatibility, phase transition, or ssh-ready behavior.

## Reviewed Inputs

- memory/talos-supervisor-state.json current task and taskQueue entries.
- tasks/2026-06-24-phase12-ssh-tftp-capture-invariant-reconciliation-v9.md.
- tasks/2026-06-24-phase12-ssh-selected-publish-extended-tftp-window-pi5-proof-v10.md.
- tasks/2026-06-24-phase12-ssh-live-openssh-preclient-fetch-gated-discriminator-v10.md.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Execution Summary

The closeout performed static task, docs, and evidence review only. No boot
archive was published, no hardware was power-cycled, no hardwareTestLock was
acquired, no lab POST endpoint was called, and no OpenSSH client was launched.

The accepted invariant boundary is now precise: the earlier stable-zero
pre-client helper results were capture timing/window evidence, not durable
no-boot-request proof. The changed-timing v10 selected-publish proof kept the
selected tree published through the pre-restore visibility window and observed
two selected 87,432-byte da591740/kernel_2712.img serves from the same saved
cursor before restore. The dependency-gated v10 live OpenSSH task then proved
its own same-task pre-client selected-fetch gate before launching exactly one
bounded workspace-local OpenSSH client attempt.

The accepted live-client frontier is no-tcp-connect / tcp-timeout after proven
selected-byte TFTP service. That is a useful feature result, but it is not
TCP reachability, remote receipt, SSH compatibility, PTY/SCP/SFTP support,
broad command expansion, a phase transition, or ssh-ready=true.

Supervisor planning is required for the next feature step. The next step should
be framed around the no-tcp-connect / tcp-timeout boundary before any remote
receipt, compatibility, or ssh-ready claim is attempted.

## Evidence Map

- v9 reconciliation:
  tasks/2026-06-24-phase12-ssh-tftp-capture-invariant-reconciliation-v9.md.
  It accepted capture-helper-timing-root-cause-ready and quarantined
  post-restore TFTP replay from selected-vs-baseline byte identity.
- v10 selected-publish extended-window proof:
  tasks/2026-06-24-phase12-ssh-selected-publish-extended-tftp-window-pi5-proof-v10.md
  and
  tasks/evidence/2026-06-24-phase12-ssh-selected-publish-extended-tftp-window-pi5-proof-v10/selected-publish-extended-window-v10.summary.sanitized.json.
  It accepted selected-publish-extended-window-fetch-observed with selected
  87,432-byte kernel service observed before restore and no OpenSSH launch.
- v10 live OpenSSH pre-client gated discriminator:
  tasks/2026-06-24-phase12-ssh-live-openssh-preclient-fetch-gated-discriminator-v10.md
  and
  tasks/evidence/2026-06-24-phase12-ssh-live-openssh-preclient-fetch-gated-discriminator-v10/rerun-clean/live-openssh-preclient-fetch-gated-discriminator-v10.summary.sanitized.json.
  It accepted a bounded OpenSSH client attempt only after same-task selected
  fetch proof, classified the public result as no-tcp-connect / tcp-timeout,
  and deleted raw OpenSSH output.

## Superseded Path Disposition

- fixed: v9/v10 replaced the stale stable-zero interpretation with an
  accepted same-cursor, pre-restore selected-byte service boundary.
- fixed: v10 live OpenSSH replaced the stale v8 closeout path by proving the
  pre-client selected-fetch gate before launch.
- removed: the v10 live OpenSSH first attempt remains quarantined because
  evidence collection later sampled TFTP after restore changed byte labels.
- deferred: TCP reachability, remote receipt, compatibility, PTY/SCP/SFTP,
  broad command expansion, phase transition, and ssh-ready=true remain outside
  the accepted boundary.
- not-an-issue: the helper timing repair task was not selected because v10
  produced selected-byte service in-window rather than a helper/cursor
  semantics blocker.

## Findings And Disposition

- fixed: closed the capture invariant thread with v9 read-only replay, v10
  selected-publish extended-window selected-byte proof, and v10 live OpenSSH
  pre-client gate evidence.
- fixed: preserved the quarantine rule that post-restore replay may prove
  cursor/log visibility but must not prove selected-vs-baseline byte identity.
- fixed: recorded the current live-client frontier as no-tcp-connect /
  tcp-timeout after selected-byte TFTP service, without overclaiming SSH
  readiness.
- deferred: the next feature discriminator for the no-tcp-connect boundary
  requires supervisor planning.
- not-an-issue: no source, lab helper, Cargo metadata, hardware state, package
  installation, raw OpenSSH transcript retention, or key material retention was
  required for this closeout.

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

Pass. This task record retains only task ids, public artifact and evidence
paths, public boot tree and kernel byte categories, public OpenSSH phase/exit
categories, validation commands, and classifications. It retains no raw serial
text, raw serial base64, raw TFTP log lines, client identities, user names,
addresses, MAC addresses, OpenSSH logs, known_hosts, host keys, authorized
keys, key material, fingerprints, signatures, session identifiers, channel
identifiers, command bytes, payload bytes, packet captures, boot artifact
bytes, stable peer identifiers, or private user data.

## Acceptance

Accepted as capture-invariant-closed-live-openssh-no-tcp-connect.

capture_invariant_closed=true.
preclient_selected_fetch_gate_passed=true.
live_openssh_client_discriminator_observed=false.
openssh_public_phase=no-tcp-connect.
openssh_exit_category=tcp-timeout.
selected_next_task=null.
planningNeeded=true.

planningReason=Supervisor planning is required for the next bounded feature
step around the accepted no-tcp-connect / tcp-timeout boundary before any
remote receipt, compatibility, PTY/SCP/SFTP, phase transition, or ssh-ready
claim.

No TCP reachability, remote receipt, compatibility, PTY/SCP/SFTP, broad command
expansion, phase transition, or ssh-ready=true is accepted.
