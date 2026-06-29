# Phase 12 SSH Local Substrate Closeout

Task id: phase12-ssh-local-substrate-closeout-20260629

Status: accepted after commit.

Classification: ssh-local-substrate-closeout-planning-needed.

Evidence level: static source/docs/task review, task-owned JSON evidence, docs
build, and diff checks. No Rust source change, Pi 5 hardware/lab action, boot
publication, generated-root retry, OpenSSH retry, live network/SSH readiness,
live authentication/session/shell success, fake command expansion, or phase
transition was performed.

## Goal

Close out the reconciled local SSH substrate frontier after the accepted
credential-readiness, service-readiness prerequisite, and publickey-auth lineage
revalidation tasks, while preserving the existing live Ethernet pause before
any generated-root/OpenSSH retry.

## Scope Performed

- Reconciled accepted local SSH credential, service-readiness, publickey-auth,
  session/channel, shell attachment, channel-data, channel-window, lifecycle,
  POSIX EOF/wait, peer-output, and socket-delivery task records.
- Reviewed current supervisor state for hardware lock, intervention status,
  queue frontier, and the retained selected_discriminator=null Ethernet pause.
- Reviewed src/ssh_service_readiness.rs, src/ssh_key_readiness.rs,
  src/ssh_runtime_crypto.rs, and src/diagnostic_command.rs for fail-closed
  readiness and local-modeled-only boundaries.
- Updated Phase 12 docs and roadmap notes with the accepted local SSH substrate
  closeout and planning-needed frontier.

## Non-goals Preserved

- No Rust implementation, listener/TCP work, packet I/O, live sockets,
  generated-root retry, OpenSSH retry, Pi 5 hardware/lab action, boot
  publication, session/channel/shell broadening, POSIX/Linux/OpenSSH
  compatibility claim, fake command expansion, or phase transition.
- No durable retention of real seed bytes, host-key bytes, authorized_keys
  bytes, public-key blobs, request payloads, command bytes, channel data,
  signatures, signed-data bytes, fingerprints, digests, comments, peer strings,
  operator identity, account identity, channel identifiers, session-id bytes,
  exchange hashes, transport identifiers, packet payloads, boot artifact bytes,
  hardware data, or stable secret-derived identifiers.

## Reviewed Inputs

- memory/talos-supervisor-state.json current task, hardware lock, intervention,
  and task queue frontier.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.
- docs/src/decisions/README.md.
- src/ssh_service_readiness.rs.
- src/ssh_key_readiness.rs.
- src/ssh_runtime_crypto.rs.
- src/diagnostic_command.rs.
- tasks/2026-06-29-phase12-ssh-credential-readiness-closeout.md.
- tasks/2026-06-29-phase12-ssh-service-readiness-prerequisite-revalidation-core.md.
- tasks/2026-06-29-phase12-ssh-publickey-auth-lineage-revalidation-core.md.
- tasks/2026-06-22-phase12-ssh-session-channel-open-closeout.md.
- tasks/2026-06-23-phase12-ssh-session-shell-attachment-closeout.md.
- tasks/2026-06-23-phase12-ssh-channel-data-stdio-closeout.md.
- tasks/2026-06-23-phase12-ssh-channel-window-accounting-closeout.md.
- tasks/2026-06-23-phase12-ssh-channel-lifecycle-exit-status-closeout.md.
- tasks/2026-06-23-phase12-ssh-posix-eof-wait-closeout.md.
- tasks/2026-06-23-phase12-ssh-peer-output-receipt-closeout.md.
- tasks/2026-06-23-phase12-ssh-live-socket-delivery-closeout.md.
- tasks/2026-06-24-phase12-ssh-no-tcp-connect-live-network-substrate-checkpoint.md.
- tasks/2026-06-29-phase12-ethernet-paused-ssh-entropy-frontier-checkpoint.md.

## Reconciled Local SSH Substrate

The accepted Phase 12 SSH substrate is local/static or substitute-only:

- operator seed metadata, operator-provisioned read-only VFS host-key material,
  and operator-provisioned read-only VFS authorized_keys metadata are
  prerequisites only;
- sshkeydiag and sshservicediag retain fixed labels, public counts/booleans,
  and fail-closed ssh-ready=false behavior;
- local modeled publickey auth can reach authentication_success=true only on
  the accepted account-success report after service-userauth, private
  session-id handle, same-request authorized-key match, supported algorithm,
  valid signature, account match, and enabled account policy;
- local modeled session/channel, shell attachment, channel-data/stdio,
  channel-window accounting, lifecycle EOF/exit-status/close, POSIX EOF/wait,
  peer-output receipt, and in-kernel stream socket delivery remain accepted only
  for their bounded local success paths.

These accepted surfaces do not prove live network reachability, TCP connection
establishment, live OpenSSH compatibility, Pi 5 hardware behavior, remote host
receipt, deployed cryptographic sufficiency, ssh-ready=true, or a phase
transition.

## Ethernet And Live Retry Boundary

Phase 12.1 Ethernet remains paused at selected_discriminator=null. The
selected-discriminator local/hardware/closeout queue is still dependency-blocked
until future accepted source evidence selects a concrete Ethernet
discriminator. The accepted no-tcp-connect checkpoint still requires
source-grounded live Ethernet/network-substrate evidence before another
OpenSSH-compatible client retry would be useful.

No live/generated-root/OpenSSH retry is selected by this closeout. Any future
hardware/live task must explicitly include candidate identity, fresh serial
cursor, TFTP delta, known-good control, candidate rerun requirements, hardware
lock ownership, publication identity, and post-hardware review.

## Findings

- fixed: reconciled the accepted local SSH credential, service-readiness,
  publickey-auth, session/channel, shell, channel-data, channel-window,
  lifecycle, POSIX EOF/wait, peer-output, and socket-delivery records into one
  local/substitute substrate frontier.
- fixed: updated the Phase 12 networking/SSH project note and roadmap with the
  accepted local SSH substrate closeout and planning-needed boundary.
- fixed: confirmed source readiness getters still keep session_count=0,
  channel_count=0, authentication_success=false, shell_attached=false,
  reachability_accepted=false, and ssh_ready=false on the public service
  readiness report.
- blocked: live/generated-root/OpenSSH retry, live TCP connection,
  network/SSH reachability, remote receipt, compatibility, hardware proof, and
  ssh-ready=true remain blocked by the retained Ethernet
  selected_discriminator=null boundary and lack of a queued objective successor.
- deferred: future source evidence may reopen a bounded Ethernet discriminator
  or a local/static substrate task, but the worker did not create or choose that
  next direction.
- not-an-issue: no Rust source change was required; this closeout documents and
  reconciles already accepted local/static behavior.
- removed: no stale helper, diagnostic label, source path, task record, or
  evidence artifact was removed.

## Acceptance Check

- Findings are recorded with disposition: satisfied.
- Accepted local/substitute SSH substrate behavior is distinguished from
  unaccepted live network, OpenSSH compatibility, hardware proof, and
  ssh-ready=true claims: satisfied.
- Phase 12.1 Ethernet remains paused by selected_discriminator=null and no new
  source evidence selected a bounded return task: satisfied.
- No next task is mechanically objective, so planningNeeded=true records the
  first missing fact: satisfied.
- No live/generated-root/OpenSSH retry was selected without candidate identity,
  fresh serial cursor, TFTP delta, known-good control, and candidate rerun
  requirements: satisfied.

## Evidence Map

- Task-owned classification:
  tasks/evidence/2026-06-29-phase12-ssh-local-substrate-closeout/classification.json.
- Task-owned evidence map:
  tasks/evidence/2026-06-29-phase12-ssh-local-substrate-closeout/evidence-map.json.
- Accepted credential/service/auth revalidations:
  tasks/2026-06-29-phase12-ssh-credential-readiness-closeout.md,
  tasks/2026-06-29-phase12-ssh-service-readiness-prerequisite-revalidation-core.md,
  and tasks/2026-06-29-phase12-ssh-publickey-auth-lineage-revalidation-core.md.
- Accepted local SSH substrate closeouts:
  tasks/2026-06-22-phase12-ssh-session-channel-open-closeout.md,
  tasks/2026-06-23-phase12-ssh-session-shell-attachment-closeout.md,
  tasks/2026-06-23-phase12-ssh-channel-data-stdio-closeout.md,
  tasks/2026-06-23-phase12-ssh-channel-window-accounting-closeout.md,
  tasks/2026-06-23-phase12-ssh-channel-lifecycle-exit-status-closeout.md,
  tasks/2026-06-23-phase12-ssh-posix-eof-wait-closeout.md,
  tasks/2026-06-23-phase12-ssh-peer-output-receipt-closeout.md, and
  tasks/2026-06-23-phase12-ssh-live-socket-delivery-closeout.md.
- Live blocker records:
  tasks/2026-06-24-phase12-ssh-no-tcp-connect-live-network-substrate-checkpoint.md
  and tasks/2026-06-29-phase12-ethernet-paused-ssh-entropy-frontier-checkpoint.md.
- Source reviewed:
  src/ssh_service_readiness.rs, src/ssh_key_readiness.rs,
  src/ssh_runtime_crypto.rs, and src/diagnostic_command.rs.

## Validation

- git status --short --branch before edits: pass; main ahead of origin with no
  uncommitted Talos changes before task edits.
- static review of accepted local SSH task records and current supervisor
  state: pass.
- cargo fmt --all -- --check: not run; no Rust source touched.
- focused cargo SSH tests: not run; no Rust source or expected diagnostics
  touched.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task: null.

planningNeeded: true.

planningReason: No mechanically objective queued worker task exists after the
local SSH substrate closeout. Phase 12.1 Ethernet remains paused with
selected_discriminator=null, selected-discriminator follow-ups remain
dependency-blocked, and no new source evidence selected a bounded live
Ethernet/network or local/static SSH successor. Supervisor planning is required
before further Phase 12 work.

No hardware/lab action, boot publication, generated-root/OpenSSH retry, live
SSH readiness, live authentication/session/shell success, fake command
expansion, or phase transition is accepted.
