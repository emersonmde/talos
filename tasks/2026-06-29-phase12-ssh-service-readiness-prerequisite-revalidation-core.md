# Phase 12 SSH Service Readiness Prerequisite Revalidation Core

Task id: phase12-ssh-service-readiness-prerequisite-revalidation-core-20260629

Status: accepted after commit.

Classification: ssh-service-readiness-prerequisite-revalidation-core-accepted.

Evidence level: static source/docs/task review, focused unit tests, task-owned
JSON evidence, docs build, and diff checks. No Rust source change, real
seed/key/signature/session/transport material retention, hardware/lab action,
boot publication, generated-root retry, OpenSSH retry, live SSH readiness,
authentication success, fake command expansion, or phase transition was
performed.

## Goal

Revalidate that sshkeydiag and sshservicediag compose the accepted credential
substrate as prerequisite metadata only, while keeping the public service
readiness surface fail-closed before any live/generated-root/OpenSSH retry.

## Scope Performed

- Reviewed the accepted credential-readiness closeout and its prerequisite
  entropy, host-key, and authorized-key records.
- Reviewed src/ssh_key_readiness.rs, src/ssh_service_readiness.rs, and
  src/diagnostic_command.rs for prerequisite composition and public diagnostic
  labels.
- Ran the focused sshkeydiag and sshservicediag test filters.
- Updated the Phase 12 SSH project note to record this revalidation and the
  selected local publickey-auth lineage follow-up.

## Non-goals Preserved

- No host-key generation, authorized_keys generation, writable credential
  persistence, packet/session crypto implementation, live transport, TCP/IP
  listener work, account/user policy expansion, shell attachment acceptance,
  Pi 5 hardware/lab action, boot publication, generated-root retry, OpenSSH
  retry, fake command expansion, or phase transition.
- No durable retention of real seed bytes, generated random bytes, host-key
  bytes, authorized_keys bytes, public-key blobs, signatures, signed-data
  bytes, fingerprints, digests, comments, operator identity, account identity,
  session-id bytes, exchange hashes, transport identifiers, packet payloads, or
  stable secret-derived identifiers.

## Reviewed Inputs

- src/ssh_key_readiness.rs.
- src/ssh_service_readiness.rs.
- src/diagnostic_command.rs.
- docs/src/project/phase12-networking-ssh.md.
- tasks/2026-06-29-phase12-ssh-credential-readiness-closeout.md.
- tasks/2026-06-29-phase12-ssh-authorized-key-provisioning-revalidation-core.md.
- tasks/2026-06-29-phase12-ssh-host-key-provisioning-contract.md.
- tasks/2026-06-29-phase12-ssh-entropy-diagnostic-local-core.md.
- tasks/2026-06-29-phase12-ssh-entropy-source-contract.md.
- memory/talos-supervisor-state.json queue metadata for
  phase12-ssh-publickey-auth-lineage-revalidation-core-20260629.

## Revalidated Composition

The accepted credential substrate remains prerequisite-only:

- operator seed metadata and internal CSPRNG readiness may clear only entropy
  prerequisite labels;
- operator-provisioned read-only VFS host-key material may clear only the
  host-key prerequisite labels;
- operator-provisioned read-only VFS authorized_keys metadata may clear only
  the authorized-key prerequisite labels;
- persistence and exposure metadata remain fixed prerequisite inputs and do not
  imply live service readiness.

sshkeydiag and sshservicediag retain only fixed labels, booleans, public path
names, public byte counts/lengths, public message numbers, validation
commands, task ids, and classifications. sshservicediag remains fail-closed:
transport-enabled=false, authentication-success=false, shell-attached=false,
reachability-accepted=false, and ssh-ready=false unless a later explicit live
task accepts otherwise.

## Findings

- fixed: confirmed missing, invalid, insufficient, and sufficient host-key and
  authorized-key metadata retain fixed sshkeydiag labels and redact secret
  material; sufficient metadata clears only its matching prerequisite.
- fixed: confirmed operator-seed/CSPRNG readiness, persistence metadata, and
  exposure opt-in compose consistently into sshkeydiag/sshservicediag without
  exposing secret material.
- fixed: confirmed the default and prerequisite-satisfied sshservicediag
  surfaces remain fail-closed; public readiness booleans for transport,
  authentication, shell attachment, reachability, and ssh-ready stay false.
- fixed: updated docs/src/project/phase12-networking-ssh.md to record the
  accepted service-readiness revalidation and selected publickey-auth lineage
  follow-up.
- deferred: local publickey-auth lineage revalidation remains the next bounded
  local/static task.
- deferred: live/generated-root/OpenSSH retry, network reachability, hardware
  proof, final authentication acceptance, session/channel success, shell
  attachment acceptance, and phase transition remain outside this task.
- not-an-issue: no Rust source change was required; focused tests already cover
  the requested fixed-label and fail-closed boundaries.
- removed: no stale helper, diagnostic label, evidence artifact, source path,
  or task record was removed.

## Acceptance Check

- Findings are recorded with disposition: satisfied.
- Focused tests/static evidence prove missing/invalid/insufficient/sufficient
  host-key and authorized-key metadata remain fixed-label and redacted:
  satisfied by sshkeydiag and sshservicediag test filters.
- Focused tests/static evidence prove operator-seed/CSPRNG readiness,
  persistence metadata, and exposure opt-in compose into sshkeydiag and
  sshservicediag without secret exposure: satisfied.
- sshservicediag remains fail-closed with transport-enabled=false,
  authentication-success=false, shell-attached=false,
  reachability-accepted=false, and ssh-ready=false unless a future live task
  accepts otherwise: satisfied.
- selected_next_task is
  phase12-ssh-publickey-auth-lineage-revalidation-core-20260629 because the
  queued task is mechanically objective after this revalidation: satisfied.

## Evidence Map

- Task-owned classification:
  tasks/evidence/2026-06-29-phase12-ssh-service-readiness-prerequisite-revalidation-core/classification.json.
- Task-owned evidence map:
  tasks/evidence/2026-06-29-phase12-ssh-service-readiness-prerequisite-revalidation-core/evidence-map.json.
- Task-owned validation summary:
  tasks/evidence/2026-06-29-phase12-ssh-service-readiness-prerequisite-revalidation-core/validation-summary.json.
- Source reviewed:
  src/ssh_key_readiness.rs, src/ssh_service_readiness.rs, and
  src/diagnostic_command.rs.
- Accepted prerequisite records:
  tasks/2026-06-29-phase12-ssh-credential-readiness-closeout.md,
  tasks/2026-06-29-phase12-ssh-authorized-key-provisioning-revalidation-core.md,
  tasks/2026-06-29-phase12-ssh-host-key-provisioning-contract.md,
  tasks/2026-06-29-phase12-ssh-entropy-diagnostic-local-core.md, and
  tasks/2026-06-29-phase12-ssh-entropy-source-contract.md.

## Validation

- git status --short --branch before edits: pass; main ahead of origin with no
  uncommitted Talos changes before task edits.
- static source/docs/task review: pass.
- cargo fmt --all -- --check: not run; no Rust source touched.
- cargo -Zjson-target-spec test --quiet sshkeydiag: pass; custom no_std runner
  reported 887 passed and included sshkeydiag, diagnostic_command,
  ssh_key_readiness, ssh_runtime_crypto, and ssh_service_readiness coverage.
- cargo -Zjson-target-spec test --quiet sshservicediag: pass; custom no_std
  runner reported 887 passed and included sshservicediag, diagnostic_command,
  ssh_key_readiness, ssh_runtime_crypto, and ssh_service_readiness coverage.
- cargo -Zjson-target-spec test --quiet ssh_service_readiness: not run; the
  source file was not touched, and the required sshservicediag gate already
  exercised ssh_service_readiness tests through the repo runner.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass; search index size warning retained.
- jq empty on task-owned JSON evidence: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task: phase12-ssh-publickey-auth-lineage-revalidation-core-20260629.

planningNeeded: false.

The publickey-auth lineage revalidation is mechanically objective because it is
already queued with complete dependencies, acceptance criteria, validation
gates, documentation requirements, and evidence requirements, and this task
satisfies its accepted service-readiness dependency. No live SSH readiness,
authentication success, generated-root/OpenSSH retry, hardware proof, fake
command expansion, or phase transition is accepted.
