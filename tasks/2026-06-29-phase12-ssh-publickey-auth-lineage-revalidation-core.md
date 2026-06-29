# Phase 12 SSH Publickey Auth Lineage Revalidation Core

Task id: phase12-ssh-publickey-auth-lineage-revalidation-core-20260629

Status: accepted after commit.

Classification: ssh-publickey-auth-lineage-revalidation-core-accepted.

Evidence level: static source/docs/task review, focused unit tests,
task-owned JSON evidence, docs build, and diff checks. No Rust source change,
real seed/key/signature/session/transport material retention, hardware/lab
action, boot publication, generated-root retry, OpenSSH retry, live SSH
readiness, session/channel/shell success, fake command expansion, or phase
transition was performed.

## Goal

Revalidate that the local modeled publickey-auth lineage still composes from
the accepted credential and service-readiness contracts before any
live/generated-root/OpenSSH retry is considered.

## Scope Performed

- Reviewed the accepted service-readiness prerequisite revalidation and its
  credential-readiness inputs.
- Reviewed src/ssh_key_readiness.rs, src/ssh_service_readiness.rs, and
  src/ssh_runtime_crypto.rs for authorized_keys matching, session-id lifetime,
  signature verification, auth response policy, account policy, and local-only
  success boundaries.
- Reviewed retained publickey verification, response-policy, and account
  closeout records from 2026-06-22.
- Ran the focused authorized_key, publickey, and userauth test filters.
- Updated the Phase 12 SSH project note to record this revalidation and the
  selected local SSH substrate closeout follow-up.

## Non-goals Preserved

- No new authentication method, account database expansion, live sockets,
  packet I/O, generated-root retry, OpenSSH retry, Pi 5 hardware/lab action,
  boot publication, session/channel allocation, shell attachment acceptance,
  broad compatibility claim, fake command expansion, or phase transition.
- No durable retention of real seed bytes, host-key bytes, authorized_keys
  bytes, request public-key blobs, session-id bytes, signatures, signed-data
  bytes, fingerprints, digests, comments, peer strings, operator identity,
  account identity, transport identifiers, packet payloads, or stable
  key-derived identifiers.

## Reviewed Inputs

- src/ssh_key_readiness.rs.
- src/ssh_service_readiness.rs.
- src/ssh_runtime_crypto.rs.
- docs/src/project/phase12-networking-ssh.md.
- tasks/2026-06-29-phase12-ssh-service-readiness-prerequisite-revalidation-core.md.
- tasks/2026-06-29-phase12-ssh-credential-readiness-closeout.md.
- tasks/2026-06-29-phase12-ssh-authorized-key-provisioning-revalidation-core.md.
- tasks/2026-06-22-phase12-ssh-publickey-verification-closeout.md.
- tasks/2026-06-22-phase12-ssh-publickey-auth-response-policy-closeout.md.
- tasks/2026-06-22-phase12-ssh-publickey-auth-success-account-closeout.md.
- memory/talos-supervisor-state.json queue metadata for
  phase12-ssh-local-substrate-closeout-20260629.

## Revalidated Lineage

Local modeled publickey auth success still requires all accepted prerequisites:

- service-userauth recognition before a publickey userauth request is accepted;
- a private in-memory userauth session-id handle from accepted runtime KEX
  readiness, never durable session-id bytes;
- an in-memory same-request authorized_keys public-key match whose request and
  matched blob lengths equal the modeled request public-key blob length;
- supported ssh-ed25519 request and signature algorithms;
- signature-present=true and valid signature verification over reconstructed
  RFC 4252 signed data;
- reserved Talos account match plus enabled account policy;
- redaction-sensitive paths disabled before any success classification.

Metadata-only authorized_keys readiness is not enough for authentication. It
can clear only diagnostic prerequisites. The userauth lineage consumes an
in-memory AuthorizedKeyMatchReport produced from the modeled request key, and
the service-readiness source rechecks the match flag plus request and matched
public-key blob lengths before verifier or account success can proceed.

The only accepted success remains local/modelled:
authentication_success=true may appear only on the account success report, and
that report still has service_success=false, session_count=0, channel_count=0,
shell_attached=false, reachability_accepted=false, and ssh_ready=false.

## Findings

- fixed: confirmed publickey signature verification requires service-userauth,
  signature-present=true, a private session-id handle, same-request
  authorized-key match, supported ssh-ed25519 algorithm, well-formed signed
  data, and a valid signature before reporting
  sshservicediag-publickey-verification-prerequisite-only.
- fixed: confirmed publickey response policy keeps unsigned authorized probes
  limited to USERAUTH_PK_OK prerequisite-only and signed-valid requests limited
  to USERAUTH_FAILURE until the explicit account success policy accepts local
  modeled success.
- fixed: confirmed publickey auth account success requires reserved Talos
  account match, enabled account policy, service/session/key-match/verifier
  prerequisites, valid signature, and non-redaction-sensitive inputs.
- fixed: updated docs/src/project/phase12-networking-ssh.md to record the
  accepted publickey-auth lineage revalidation and selected local substrate
  closeout follow-up.
- deferred: local SSH substrate closeout remains the next bounded local/static
  task.
- deferred: live/generated-root/OpenSSH retry, network reachability, hardware
  proof, session/channel success, shell attachment acceptance, broad account
  model, compatibility claims, and phase transition remain outside this task.
- not-an-issue: no Rust source change was required; focused tests already
  cover malformed, unsupported, unauthorized, unsigned, bad-signature,
  missing-session, missing-service, missing-account-policy, and
  account-mismatch paths as fixed-label fail-closed states.
- removed: no stale helper, diagnostic label, evidence artifact, source path,
  or task record was removed.

## Acceptance Check

- Findings are recorded with disposition: satisfied.
- Source/tests/static evidence prove publickey auth success remains
  local/modelled and requires service/userauth prerequisites, private
  session-id handle, same-request authorized-key match, valid signature,
  supported algorithm, and enabled account policy: satisfied.
- Malformed, unsupported, unauthorized, unsigned, bad-signature,
  missing-session, missing-service, missing-account-policy, and
  account-mismatch paths fail closed with fixed labels: satisfied by focused
  publickey/userauth tests and source review.
- Durable evidence retains only fixed labels, public length/count/message-number
  fields, validation commands, task ids, and classifications: satisfied.
- selected_next_task is phase12-ssh-local-substrate-closeout-20260629 because
  the local SSH substrate is reconciled enough for closeout and the queued
  closeout task is explicit and mechanically objective: satisfied.

## Evidence Map

- Task-owned classification:
  tasks/evidence/2026-06-29-phase12-ssh-publickey-auth-lineage-revalidation-core/classification.json.
- Task-owned evidence map:
  tasks/evidence/2026-06-29-phase12-ssh-publickey-auth-lineage-revalidation-core/evidence-map.json.
- Task-owned validation summary:
  tasks/evidence/2026-06-29-phase12-ssh-publickey-auth-lineage-revalidation-core/validation-summary.json.
- Source reviewed:
  src/ssh_key_readiness.rs, src/ssh_service_readiness.rs, and
  src/ssh_runtime_crypto.rs.
- Accepted prerequisite records:
  tasks/2026-06-29-phase12-ssh-service-readiness-prerequisite-revalidation-core.md,
  tasks/2026-06-29-phase12-ssh-credential-readiness-closeout.md, and
  tasks/2026-06-29-phase12-ssh-authorized-key-provisioning-revalidation-core.md.
- Retained publickey auth records:
  tasks/2026-06-22-phase12-ssh-publickey-verification-closeout.md,
  tasks/2026-06-22-phase12-ssh-publickey-auth-response-policy-closeout.md,
  and tasks/2026-06-22-phase12-ssh-publickey-auth-success-account-closeout.md.

## Validation

- git status --short --branch before edits: pass; main ahead of origin with no
  uncommitted Talos changes before task edits.
- static source/docs/task review: pass.
- cargo fmt --all -- --check: not run; no Rust source touched.
- cargo -Zjson-target-spec test --quiet authorized_key: pass; custom no_std
  runner reported 887 passed and included authorized_key, diagnostic_command,
  ssh_key_readiness, and ssh_service_readiness coverage.
- cargo -Zjson-target-spec test --quiet publickey: pass; custom no_std runner
  reported 887 passed and included publickey verification, response-policy, and
  account-success coverage.
- cargo -Zjson-target-spec test --quiet userauth: pass; custom no_std runner
  reported 887 passed and included service-userauth, session-id, publickey, and
  account-success coverage.
- cargo -Zjson-target-spec test --quiet ssh_service_readiness: not run; the
  source file was not touched, and the required publickey/userauth gates
  already exercised ssh_service_readiness tests through the repo runner.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass; search index size warning retained.
- jq empty on task-owned JSON evidence: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task: phase12-ssh-local-substrate-closeout-20260629.

planningNeeded: false.

The local SSH substrate closeout is mechanically objective because it is
already queued with complete dependencies, acceptance criteria, validation
gates, documentation requirements, and evidence requirements, and this task
satisfies its accepted publickey-auth lineage dependency. No live SSH
readiness, generated-root/OpenSSH retry, hardware proof, session or shell
success, fake command expansion, or phase transition is accepted.
