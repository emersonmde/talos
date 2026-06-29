# Phase 12 SSH Credential Readiness Closeout

Task id: phase12-ssh-credential-readiness-closeout-20260629

Status: accepted after commit.

Classification: ssh-credential-readiness-closeout-accepted.

## Goal

Close out the local/static SSH credential substrate after the accepted entropy,
host-key provisioning, and authorized-key provisioning revalidation tasks, then
select only the next mechanically objective local revalidation step.

## Scope

- Reconcile accepted operator seed metadata, operator-provisioned read-only VFS
  host-key material, and operator-provisioned read-only VFS authorized_keys
  metadata as prerequisite inputs only.
- Review accepted entropy, host-key, authorized-key, service-readiness, and
  publickey-auth task/source boundaries for drift.
- Record findings with disposition and preserve redaction rules for seed,
  key, signature, session-id, exchange-hash, and transport material.
- Select phase12-ssh-service-readiness-prerequisite-revalidation-core-20260629
  only if the follow-up remains mechanically objective.

## Non-goals

- No entropy source change, host-key generation, generated key persistence,
  authorized_keys generation, packet/session crypto, authentication success,
  session/channel behavior, shell attachment, TCP/network work, Pi 5
  hardware/lab action, boot publication, generated-root retry, OpenSSH retry,
  live SSH readiness, fake command expansion, or phase transition.
- No real seed bytes, host-key bytes, authorized_keys bytes, public-key blobs,
  signatures, fingerprints, digests, comments, operator identity, account
  identity, session identifiers, exchange hashes, transport identifiers, or
  stable secret-derived identifiers retained in task evidence.

## Reviewed Inputs

- src/entropy.rs.
- src/csprng.rs.
- src/ssh_key_readiness.rs.
- src/ssh_service_readiness.rs.
- src/diagnostic_command.rs.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/decisions/README.md.
- tasks/2026-06-29-phase12-ssh-entropy-source-contract.md.
- tasks/2026-06-29-phase12-ssh-entropy-diagnostic-local-core.md.
- tasks/2026-06-29-phase12-ssh-host-key-provisioning-contract.md.
- tasks/2026-06-29-phase12-ssh-authorized-key-provisioning-revalidation-core.md.
- tasks/2026-06-22-phase12-ssh-service-readiness-closeout.md.
- tasks/2026-06-22-phase12-ssh-publickey-auth-success-account-closeout.md.
- tasks/2026-06-22-phase12-ssh-publickey-verification-closeout.md.
- tasks/2026-06-22-phase12-ssh-publickey-auth-response-policy-closeout.md.
- memory/talos-supervisor-state.json task queue metadata for the queued
  service-readiness and publickey-auth revalidation follow-ups.

## Reconciled Credential Substrate

The accepted local SSH credential substrate is:

- operator seed metadata from the accepted entropy contract and local diagnostic
  core;
- operator-provisioned read-only VFS host-key material at
  /etc/talos/ssh/ssh_host_ed25519_key as prerequisite metadata/material for
  local host-key paths only;
- operator-provisioned read-only VFS authorized_keys metadata at
  /etc/talos/ssh/authorized_keys as prerequisite metadata for diagnostics and
  in-memory key-match paths only.

These inputs remain prerequisites. They do not accept deployed cryptographic
sufficiency, live transport, authentication success, session/channel behavior,
shell attachment, reachability, OpenSSH compatibility, or ssh-ready true.

The retained redaction boundary is unchanged: durable evidence may retain only
fixed labels, booleans, public path names, public byte counts/lengths, public
message numbers, task ids, validation commands, and classifications. Durable
evidence must not retain real seed bytes, generated random bytes, host-key
bytes, authorized_keys bytes, public-key blobs, signatures, signed-data bytes,
fingerprints, digests, comments, operator identity, account identity, session-id
bytes, exchange hashes, transport identifiers, or stable secret-derived
identifiers.

## Downstream Review

Downstream service-readiness source still needs the explicit queued
prerequisite revalidation before any live/generated-root/OpenSSH retry because
sshservicediag composes the credential prerequisites into the public diagnostic
surface. The revalidation is mechanically objective: the task is already queued
with complete dependencies, acceptance criteria, validation gates, docs
requirements, and evidence requirements; this closeout satisfies its first
dependency by accepting the reconciled credential substrate and selecting it.

The publickey-auth lineage revalidation remains queued behind service-readiness
revalidation. It should not be promoted until the service-readiness task accepts
selected_next_task=phase12-ssh-publickey-auth-lineage-revalidation-core-20260629.

## Findings

- fixed: reconciled operator seed metadata, read-only VFS host-key material, and
  read-only VFS authorized_keys metadata as prerequisite-only credential inputs.
- fixed: confirmed ssh_key_readiness keeps ssh_ready false and always appends
  sshkeydiag-not-ready even when host-key, authorized-key, seed, persistence,
  and exposure metadata are present.
- fixed: confirmed ssh_service_readiness public readiness keeps
  transport-enabled=false, authentication-success=false, shell-attached=false,
  reachability-accepted=false, and ssh-ready=false unless a later explicit live
  task accepts otherwise.
- fixed: updated the Phase 12 SSH project note to include the credential
  closeout boundary and selected service-readiness revalidation follow-up.
- deferred: service-readiness prerequisite revalidation is selected as the next
  bounded local/static task.
- deferred: publickey-auth lineage revalidation remains queued behind
  service-readiness revalidation.
- deferred: live/generated-root/OpenSSH retry, live network reachability,
  packet/session crypto, authentication/session/shell success, Pi 5 hardware
  proof, and phase transition remain outside this closeout.
- not-an-issue: no Rust source change is needed for this closeout; accepted
  source already preserves fixed labels and redacted metadata-only diagnostic
  boundaries for this slice.

## Evidence Map

- Task-owned classification:
  tasks/evidence/2026-06-29-phase12-ssh-credential-readiness-closeout/classification.json.
- Task-owned evidence map:
  tasks/evidence/2026-06-29-phase12-ssh-credential-readiness-closeout/evidence-map.json.
- Accepted entropy source contract:
  tasks/2026-06-29-phase12-ssh-entropy-source-contract.md.
- Accepted entropy diagnostic local core:
  tasks/2026-06-29-phase12-ssh-entropy-diagnostic-local-core.md.
- Accepted host-key provisioning contract:
  tasks/2026-06-29-phase12-ssh-host-key-provisioning-contract.md.
- Accepted authorized-key provisioning revalidation:
  tasks/2026-06-29-phase12-ssh-authorized-key-provisioning-revalidation-core.md.
- Retained service-readiness and publickey-auth records:
  tasks/2026-06-22-phase12-ssh-service-readiness-closeout.md,
  tasks/2026-06-22-phase12-ssh-publickey-auth-success-account-closeout.md,
  tasks/2026-06-22-phase12-ssh-publickey-verification-closeout.md, and
  tasks/2026-06-22-phase12-ssh-publickey-auth-response-policy-closeout.md.
- Static source review:
  src/ssh_key_readiness.rs, src/ssh_service_readiness.rs,
  src/diagnostic_command.rs, src/entropy.rs, and src/csprng.rs.
- Project docs:
  docs/src/project/phase12-networking-ssh.md and
  docs/src/decisions/README.md.

## Validation

- git status --short --branch before edits: passed; main was ahead of origin
  with no uncommitted Talos changes.
- static review of accepted entropy, host-key, authorized-key,
  service-readiness, and publickey-auth task records: passed.
- cargo fmt --all -- --check: not run; no Rust source touched.
- focused cargo tests: not run; no Rust source or diagnostic expected output
  touched.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed; search index size warning
  retained.
- jq empty on task-owned JSON evidence: passed.
- git diff --cached --check: passed.

## Result

selected_next_task: phase12-ssh-service-readiness-prerequisite-revalidation-core-20260629.

planningNeeded: false.

The next task is mechanically objective because it is already queued with
complete scope, non-goals, dependencies, acceptance criteria, validation gates,
docs requirements, and evidence requirements, and this closeout provides its
accepted credential-substrate dependency. No hardware/lab action, boot
publication, generated-root/OpenSSH retry, live SSH readiness, authentication
success, fake command expansion, or phase transition is accepted.
