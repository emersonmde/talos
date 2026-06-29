# Phase 12 SSH Live TCP Pi 5 Candidate Preflight

Task id: phase12-ssh-live-tcp-pi5-candidate-preflight-20260629

Status: accepted after commit.

Classification: blocked-candidate-identity.

Evidence level: git/source identity, Pi 5 boot archive build attempt,
lab-controller API baseline/restore identity, task-owned JSON validation, docs
build, and diff checks. No boot archive was published, no power-cycle was
performed, and no known-good control, candidate run, candidate rerun, packet-I/O
discriminator, OpenSSH retry, generated-root retry, remote receipt,
compatibility claim, service success claim, ssh-ready=true claim, runtime russh
adoption, fake command expansion, broad shell work, or phase transition was
performed.

## Goal

Execute the accepted Pi 5 live TCP proof preflight contract far enough to either
produce candidate-capture-ready evidence or identify the first missing fact
before any packet-I/O discriminator or OpenSSH retry.

## Scope Performed

- Acquired hardwareTestLock before lab-controller API reads or restore action.
- Retained baseline lab identity from `GET /status`, `GET /boot/files`, and
  `GET /boot/snapshots`.
- Checked source identity at commit
  `5999653e6835bed996bd1dbb666c29609ab411d3` and confirmed the accepted runtime
  commits are ancestors.
- Attempted to build the smallest current-source Pi 5 boot archive through
  `scripts/rpi5-boot-tree.sh`.
- Stopped before publication because the current source commit does not build
  for the Pi 5 archive target.
- Restored the lab to the named snapshot
  `abcontrol-secondary-workload-pre-20260524T231449Z` and retained post-restore
  lab identity.

## Terminal Classification

blocked-candidate-identity.

The source/boot-archive identity could not be joined because the current source
commit fails the Pi 5 boot archive build:

```text
error[E0599]: no variant, associated function, or constant named `TimedOut` found for enum `PosixError` in the current scope
    --> src/network.rs:4078:35
     |
4078 |     Err(crate::posix::PosixError::TimedOut)
     |                                   ^^^^^^^^ variant, associated function, or constant not found in `PosixError`
```

No archive path, archive SHA-256, archive byte count, or expected candidate
kernel byte count is accepted for this preflight because the build failed before
those artifacts existed.

## Findings

- fixed: acquired hardwareTestLock before any lab-controller API evidence or
  restore action.
- fixed: retained baseline lab identity, source commit identity, accepted commit
  ancestry evidence, build stderr, restore result, and post-restore lab identity
  in one task-owned evidence bundle.
- fixed: classified the first missing fact as blocked-candidate-identity rather
  than retrying hardware or broadening into the packet-I/O discriminator.
- deferred: known-good control, fresh serial cursor, fresh TFTP cursor,
  candidate run, and candidate rerun remain deferred until the source can build
  a candidate archive.
- blocked: `src/network.rs` references `PosixError::TimedOut`, which is absent
  from `src/posix.rs`; a later explicit task must repair or replan that source
  boundary before Pi 5 candidate preflight can continue.
- not-an-issue: no TFTP, serial, OpenSSH, or remote receipt evidence is missing
  from this classification because the pre-publication candidate identity gate
  failed first.
- removed: no source, helper, docs, or evidence artifact was removed.

## Evidence Map

- Evidence directory:
  `tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight/candidate-preflight-20260629T100258Z/`.
- Classification:
  `tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight/candidate-preflight-20260629T100258Z/classification.json`.
- Evidence map:
  `tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight/candidate-preflight-20260629T100258Z/evidence-map.json`.
- Build stderr:
  `tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight/candidate-preflight-20260629T100258Z/validation/rpi5-boot-tree.stderr.txt`.
- Baseline and restore identity:
  `baseline/status.json`, `baseline/boot-files.json`,
  `baseline/boot-snapshots.json`, `restore/restore-result.json`,
  `restore/status-after-restore.json`, `restore/boot-files-after-restore.json`,
  and `restore/boot-snapshots-after-restore.json`.

## Redaction Review

Durable evidence retains task ids, run labels, source commit, public file paths,
snapshot names, lab status metadata, validation commands/results, and public
compiler diagnostics. It does not retain peer identifiers, addresses, packet
payload contents, key material, session material, boot artifact bytes, private
user data, stable secret-derived identifiers, or unnecessary hardware data.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before task evidence.
- source commit and accepted commit ancestry: pass.
- Pi 5 boot archive build attempt through `scripts/rpi5-boot-tree.sh`: failed
  before archive creation with missing `PosixError::TimedOut`.
- Lab API GET /status, GET /boot/files, GET /boot/snapshots before candidate
  action: pass.
- Fresh serial cursor, TFTP cursor, known-good control, candidate run, and
  candidate rerun: not run because the pre-publication candidate identity gate
  failed first.
- Restore to named snapshot and post-restore GET /status, GET /boot/files, and
  GET /boot/snapshots: pass; post-restore status reports
  `tree_hash=6ead8933b3287cddaf08ddcdbda7d4c770b365658ef1d0ad80a5dd0cf323f3ef`
  and `effective_kernel=kernel_2712.img`.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass after this record.
- /home/node/.cargo/bin/mdbook build: pass after docs updates.
- git diff --cached --check: pass before commit.

## Result

selected_next_task: null.

planningNeeded: true.

planningReason: current source commit cannot produce a Pi 5 boot archive for
candidate preflight because `src/network.rs` references missing
`PosixError::TimedOut`; supervisor planning is required for the bounded source
repair or replacement candidate identity task before further Pi 5 live TCP
preflight, packet-I/O discriminator, OpenSSH retry, generated-root retry,
remote receipt, compatibility, service success, ssh-ready=true, runtime russh
adoption, fake command expansion, broad shell work, or phase transition.

Commit: recorded in talos-supervisor-state.json after final commit.
