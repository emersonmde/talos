# Phase 12 SSH Authorized-Key Provisioning Revalidation Core

Task id: phase12-ssh-authorized-key-provisioning-revalidation-core-20260629

Status: accepted

Classification: ssh-authorized-key-provisioning-revalidation-core-accepted

Evidence level: static source/docs/task review, unit tests, task-owned JSON
evidence, docs build, and diff checks. No real authorized_keys material,
host-key generation, generated key persistence, authentication success,
packet/session crypto, live SSH, TCP/network work, Pi 5 hardware proof, boot
publication, generated-root retry, OpenSSH retry, or phase transition was
performed.

## Goal

Revalidate the authorized-key provisioning boundary after the accepted host-key
provisioning contract, repairing any drift in metadata-only diagnostics while
keeping SSH readiness fail-closed.

## Scope Performed

- Reviewed the accepted host-key provisioning contract, authorized-key policy
  records, authorized_keys parser records, SSH key-readiness source, diagnostic
  dispatcher, service-readiness source, Phase 12 docs, and ADR index.
- Confirmed /etc/talos/ssh/authorized_keys remains the selected
  operator-provisioned read-only VFS source for this slice.
- Confirmed current source already preserves the metadata-only classifier:
  missing, invalid, insufficient, and sufficient states use fixed labels,
  sufficient metadata clears only the authorized-key prerequisite, and
  ssh-ready remains false.
- Updated the Phase 12 SSH project doc with the revalidated credential
  boundary.

## Findings

- fixed: the current frontier is reconciled with the accepted host-key
  provisioning contract; host-key and authorized_keys material share the same
  non-retention rule family.
- fixed: authorized_keys provisioning remains
  /etc/talos/ssh/authorized_keys as operator-provisioned read-only VFS material
  and not generated, embedded, persisted, fingerprinted, or retained in task
  evidence.
- not-an-issue: src/ssh_key_readiness.rs already classifies
  authorized_keys metadata without parsing or retaining key bytes for
  sshkeydiag. Sufficient metadata clears only the authorized-key prerequisite.
- not-an-issue: src/diagnostic_command.rs already reports only fixed
  sshkeydiag labels, booleans, and public path/length-derived state; the
  dispatcher tests cover invalid, insufficient, and sufficient VFS metadata.
- not-an-issue: src/ssh_service_readiness.rs remains fail-closed. Authorized
  key metadata can help clear prerequisites only inside local shape modeling
  and cannot set authentication-success, shell-attached, reachability, or
  ssh-ready true.
- deferred: account/user binding, per-user authorized_keys paths, writable
  credential persistence, response policy changes, service readiness, live
  transport, OpenSSH compatibility, and Pi 5 hardware proof remain for future
  explicit tasks.
- removed: no helper, source path, diagnostic label, evidence artifact, or
  stale task was removed.

## Candidate Dispositions

- selected: /etc/talos/ssh/authorized_keys as operator-provisioned read-only
  VFS material for the first authorized-key source policy.
- selected: metadata-only sshkeydiag states for this slice: missing, invalid,
  insufficient, and sufficient/metadata-present.
- selected: fixed-label/count/boolean diagnostic and durable evidence only.
  Public byte counts and path names may be retained; key bytes, comments,
  fingerprints, digests, signatures, operator identity, public-key blobs, and
  stable secret-derived identifiers must not be retained.
- deferred: parsing/key matching for authentication prerequisites remains
  accepted only through the earlier in-memory parser/key-match boundary and is
  not promoted by this revalidation task.
- deferred: account policy, final authorization, authentication success,
  session/channel behavior, shell attachment, live transport, and OpenSSH
  compatibility.
- rejected: generated or embedded operator authorized_keys material, retained
  real authorized key bytes, retained fingerprints/digests, and any use of
  authorized_keys metadata as proof of live SSH readiness.
- blocked: writable credential persistence remains blocked until a separate
  persistent storage contract exists.

## Revalidated Contract

The selected authorized-key provisioning contract for this slice is:

~~~text
/etc/talos/ssh/authorized_keys
~~~

The material may exist as operator-provisioned read-only VFS/generated-root
configuration. sshkeydiag may classify only metadata states and must never
retain or print real authorized_keys bytes, comments, fingerprints, digests,
signatures, public-key blobs, operator identity, or stable key-derived
identifiers. Sufficient metadata may clear only the authorized-key prerequisite;
authentication, session behavior, live SSH readiness, OpenSSH compatibility,
and network reachability remain false/unaccepted.

The existing authorized_keys parser/key-match API remains an in-memory
prerequisite-only implementation for later userauth work. This revalidation
does not broaden it, does not add new parsing policy, and does not retain parser
inputs or outputs in durable evidence.

## Evidence

- Classification:
  tasks/evidence/2026-06-29-phase12-ssh-authorized-key-provisioning-revalidation-core/classification.json.
- Evidence map:
  tasks/evidence/2026-06-29-phase12-ssh-authorized-key-provisioning-revalidation-core/evidence-map.json.
- Validation summary:
  tasks/evidence/2026-06-29-phase12-ssh-authorized-key-provisioning-revalidation-core/validation-summary.json.
- Source reviewed: src/ssh_key_readiness.rs, src/diagnostic_command.rs,
  src/ssh_service_readiness.rs.
- Records reviewed:
  tasks/2026-06-29-phase12-ssh-host-key-provisioning-contract.md,
  tasks/2026-06-21-phase12-ssh-authorized-key-policy-contract.md,
  tasks/2026-06-21-phase12-ssh-authorized-key-vfs-metadata-core.md,
  tasks/2026-06-21-phase12-ssh-authorized-key-readiness-closeout.md,
  tasks/2026-06-22-phase12-ssh-authorized-keys-parser-policy-contract.md,
  tasks/2026-06-22-phase12-ssh-authorized-keys-parser-core.md, and
  tasks/2026-06-22-phase12-ssh-service-readiness-diagnostic-core.md.

## Acceptance Check

- Findings are recorded with disposition: satisfied.
- Authorized-key provisioning candidates are classified and
  /etc/talos/ssh/authorized_keys remains selected only as read-only VFS
  operator-provisioned metadata: satisfied.
- Source and diagnostics satisfy the metadata-only contract with fixed labels
  and no retained real key material: satisfied.
- sshkeydiag and sshservicediag remain fail-closed; sufficient authorized-key
  metadata clears only a prerequisite and does not accept live SSH readiness:
  satisfied.
- selected_next_task is not set because no queued credential-readiness closeout
  or implementation follow-up is mechanically objective; planningNeeded=true is
  required: satisfied.

## Validation

- git status --short --branch before edits: pass; main ahead of origin with no
  uncommitted Talos changes before task edits.
- static source/docs/task review: pass.
- cargo -Zjson-target-spec test --quiet sshkeydiag: pass; custom no_std
  runner reported 887 passed and included sshkeydiag, authorized_key,
  diagnostic_command, and ssh_service_readiness coverage.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass; search index size warning retained.
- jq empty on task-owned JSON evidence: pass.
- git diff --cached --check: pass before commit.

## Next Action

Supervisor planning is required before another credential-readiness closeout or
implementation step. No queued follow-up task is selected by this revalidation.
