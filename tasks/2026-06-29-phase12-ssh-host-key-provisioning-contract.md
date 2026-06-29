# Phase 12 SSH Host-Key Provisioning Contract

Task id: phase12-ssh-host-key-provisioning-contract-20260629

Status: accepted

Classification: ssh-host-key-provisioning-contract-accepted

Evidence level: static source/docs/task/ADR review, task-owned JSON evidence,
docs build, and diff checks. No source implementation, host-key generation,
authorized_keys generation, packet I/O, live SSH, Pi 5 hardware proof, boot
publication, generated-root retry, OpenSSH retry, or phase transition was
performed.

## Goal

Revalidate the SSH host-key provisioning contract after the accepted entropy
diagnostic local core, without generating, retaining, or exposing real key
material.

## Scope Performed

- Reviewed the accepted entropy source contract and diagnostic local core.
- Reviewed existing host-key readiness, host-key private-material, and SSH
  readiness source/docs/ADR records.
- Selected the already accepted operator-provisioned read-only VFS host-key
  strategy as still authoritative.
- Recorded source/config material boundaries, evidence redaction rules, local
  validation shape for future work, and the absence of a mechanically queued
  follow-up task.

## Findings

- fixed: the host-key provisioning boundary is now reconciled against the
  current entropy diagnostic frontier; CSPRNG readiness metadata alone does not
  authorize host-key generation or live SSH readiness.
- fixed: operator-provisioned read-only VFS material remains the selected
  source/config strategy for host-key material.
- fixed: the selected runtime material format remains unencrypted OpenSSH
  ssh-ed25519 private-key material at /etc/talos/ssh/ssh_host_ed25519_key,
  using the previously accepted narrow parser/signing boundary.
- fixed: durable evidence rules explicitly reject private host-key bytes,
  authorized-key bytes, public-key blobs, signatures, fingerprints, digests,
  comments, operator identity, stable secret-derived identifiers, generated
  random bytes, session identifiers, and transport identifiers.
- deferred: any further host-key implementation or rework, runtime KEX
  consumption, NEWKEYS, encryption/MAC, authentication/session behavior,
  shell attachment, live reachability, OpenSSH compatibility, hardware proof,
  and phase transition require later explicit tasks.
- blocked: no mechanically objective next task is present in taskQueue after
  this contract, so supervisor planning is required before further worker
  promotion.
- removed: no source helper, smoke script, task, or evidence artifact was
  removed.
- not-an-issue: no Rust source change is required for this contract because the
  previously accepted source already models the selected host-key path,
  fail-closed labels, parser boundary, and in-memory signing handle.

## Candidate Dispositions

- selected: operator-provisioned read-only VFS host-key material at
  /etc/talos/ssh/ssh_host_ed25519_key. This is the only selected source/config
  strategy for the first host key.
- selected: unencrypted OpenSSH ssh-ed25519 private-key material for the first
  runtime parser/signing format, through the already accepted narrow ssh-key
  feature boundary.
- selected: metadata-only diagnostics and task evidence. Diagnostics may report
  fixed labels, booleans, public path names, and byte-length classifications
  only.
- deferred: encrypted host keys, passphrase/KDF handling, non-Ed25519 host-key
  algorithms, host certificates, multi-key files, and broad compatibility
  formats.
- deferred: generating a host key from the accepted CSPRNG. Ephemeral keys
  would not preserve server identity, and writable persistence is still outside
  the accepted storage frontier.
- blocked: generating and persisting host keys on first boot. This requires a
  writable persistent storage contract that Talos has not accepted.
- rejected: hardcoded deployment host keys in Rust source or retained evidence.
  Public fixture keys remain allowed only inside tests/task records that clearly
  label them as fixtures.
- rejected: retaining fingerprints, digests, public-key blobs, signatures, key
  comments, operator identity, session identifiers, or stable secret-derived
  identifiers as proof.

## Selected Contract

Material may exist only as operator-provisioned read-only VFS/generated-root
configuration for the current accepted strategy:

~~~text
/etc/talos/ssh/ssh_host_ed25519_key
~~~

The first runtime format remains unencrypted OpenSSH ssh-ed25519 private-key
material. The accepted diagnostic and evidence boundary is metadata-only:
missing, invalid, insufficient, or sufficient/private-material-ready labels may
be recorded, but raw key bytes and derived stable identifiers must not be
retained. A future implementation task may use the existing local source
boundary only if it is explicitly queued with owner files, validation gates,
and evidence rules.

The local validation shape for future host-key work is:

- cargo fmt --all -- --check;
- cargo -Zjson-target-spec test --quiet with the focused host-key
  private-material, sshkeydiag, and service-readiness filters named by the
  future task;
- cargo dependency feature inspection if Cargo metadata changes;
- git diff --check;
- /home/node/.cargo/bin/mdbook build if docs/src files or ADRs change;
- jq empty for task-owned JSON evidence;
- git diff --cached --check before commit.

## Rejected Claims

This task does not accept host-key generation, generated key persistence,
authorized_keys provisioning, runtime KEX consumption, packet/session crypto,
NEWKEYS, encryption/MAC, authentication success, shell attachment, live SSH,
TCP/IP, network reachability, Pi 5 hardware proof, boot publication,
generated-root retry, OpenSSH compatibility, or phase transition.

## Evidence

- Classification:
  tasks/evidence/2026-06-29-phase12-ssh-host-key-provisioning-contract/classification.json.
- Source/evidence map:
  tasks/evidence/2026-06-29-phase12-ssh-host-key-provisioning-contract/evidence-map.json.
- Source reviewed: src/ssh_key_readiness.rs, src/diagnostic_command.rs,
  src/csprng.rs, src/entropy.rs, src/ssh_service_readiness.rs.
- Project doc changed: docs/src/project/phase12-networking-ssh.md.
- ADR index changed: docs/src/decisions/README.md.
- Predecessor task:
  tasks/2026-06-29-phase12-ssh-entropy-diagnostic-local-core.md.

## Acceptance Check

- Findings are recorded with disposition: satisfied.
- Host-key provisioning candidates are classified: satisfied.
- Selected contract states allowed source/config material and evidence
  redaction rules: satisfied.
- ADR is updated because the strategy constrains future SSH server identity:
  satisfied.
- selected_next_task is not set because no mechanically objective queued
  follow-up task exists; planningNeeded=true records the precise blocker:
  satisfied.

## Validation

- git status --short --branch before edits: pass; main ahead of origin with no
  uncommitted Talos changes before task edits.
- static source/docs/task/ADR review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass; search index size warning
  retained.
- jq empty on task-owned JSON evidence: pass.
- git diff --cached --check: pass before commit.

## Next Action

Supervisor planning is required to add exactly one explicit next task if
host-key implementation, rework, closeout, or another SSH substrate slice
should continue from this contract. The next task must preserve the redaction
rules above and must not generate or retain real host keys, authorized_keys
material, private user data, packet/session traffic, hardware proof, live SSH
readiness, boot publication, generated-root retry, or phase transition.
