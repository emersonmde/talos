# Phase 12 SSH Live TCP Pi 5 Proof Contract

Task id: phase12-ssh-live-tcp-pi5-proof-contract-20260629

Status: accepted after commit.

Classification: ssh-live-tcp-pi5-proof-contract-accepted.

Evidence level: static task/docs/state review, task-owned JSON evidence, docs
build, and diff checks. No Pi 5 hardware/lab action, hardwareTestLock
acquisition, boot publication, power-cycle, generated-root retry, OpenSSH
retry, live TCP attempt, packet I/O, remote receipt claim, compatibility claim,
hardware proof, ssh-ready=true claim, service success claim, runtime russh
adoption, fake command expansion, broad shell work, or phase transition was
performed.

## Goal

Define the exact Pi 5 proof contract for the accepted local deterministic
network-device smoltcp runtime boundary before any candidate preflight, boot
publication, hardware action, packet-I/O discriminator, or OpenSSH retry.

## Scope Performed

- Reviewed the accepted runtime closeout, the lab-controller hardware evidence
  contract, Phase 12 docs, roadmap, and current supervisor state.
- Selected a contract-only successor for Pi 5 live TCP proof preflight:
  phase12-ssh-live-tcp-pi5-candidate-preflight-20260629.
- Defined selected_candidate_identity requirements that the preflight must
  satisfy before any hardware claim can be made.
- Defined required evidence windows before publication, after publication,
  before restore, and after restore.
- Preserved all OpenSSH compatibility, remote receipt, service success,
  ssh-ready=true, generated-root retry, runtime russh adoption, fake command
  expansion, and phase-transition claims as rejected.

## Selected Candidate Identity Requirements

The smallest allowed Pi 5 proof candidate is a Talos Pi 5 boot archive built
from a clean repository state that contains the accepted deterministic
DriverPacketAdapter smoltcp runtime boundary and this proof contract. The
preflight task must record the candidate identity before publishing:

- source commit: git rev-parse HEAD from a clean Talos worktree;
- required accepted commits present in history:
  74a1433cfa1312df7c6f96839c825095b28cff26 and
  e91159723769ceeb187b4dd10fc4f03deb0cb3d5;
- boot archive path, byte count, and SHA-256 digest, but not archive contents;
- archive review result and required Pi 5 boot files;
- expected TFTP fetch path: da591740/kernel_2712.img;
- expected kernel byte count from the candidate archive or reviewed boot tree;
- expected post-publication boot.tree_hash, boot.effective_kernel,
  boot.configured_kernel, parsed boot.config, guard fields, and snapshot state
  from lab API GET /status;
- task-owned evidence directory and run labels for known-good control,
  candidate run, and candidate rerun.

The candidate identity is not accepted if any required field is missing, if the
worktree is dirty before archive creation, if the expected fetch path is not
da591740/kernel_2712.img, or if the post-publication GET /status identity does
not match the reviewed candidate tree.

## Required Evidence Contract

The candidate preflight must own hardwareTestLock before any lab action and
must release it only after restore evidence has been retained. It must retain a
single task-owned evidence bundle with the following windows.

Before publication:

- git status --short --branch;
- source commit and accepted commit ancestry check;
- candidate archive metadata: path, byte count, SHA-256 digest, archive review
  result, and expected fetch byte count;
- lab API GET /status, GET /boot/files, and GET /boot/snapshots for the
  restored baseline;
- named restore target selected for the end of the run.

After publication, before power:

- lab API GET /status, GET /boot/files, and GET /boot/snapshots;
- post-publication boot.tree_hash, boot.effective_kernel,
  boot.configured_kernel, parsed boot.config, guard fields, and snapshot state;
- fresh serial cursor from /serial/peek?max_bytes=500&drain=true;
- fresh TFTP cursor from /tftp/logs tail mode using cursor_end;
- hardware lock owner, task id, acquisition time, and restore target.

During known-good control, candidate run, and candidate rerun:

- stable same-cursor TFTP delta captured before restore using the repository
  TFTP stability helper or an equivalent bounded repeated query;
- serial observation from the saved cursor, or a saturated-cursor direct-read
  fallback only when the retained helper records that contract explicitly;
- final pre-restore lab API GET /status and GET /boot/files;
- terminal summary that separates no boot request, delayed/cursor-invisible
  TFTP logging, staging mismatch, frame-provider/link blocker,
  descriptor-delivery blocker, and accepted live packet/device-interface
  progress.

Before restore:

- retained candidate and known-good summaries must be complete enough to review
  candidate identity, TFTP delta, serial freshness, and final pre-restore
  identity without querying mutable endpoints again;
- any served-file byte classification must come from the pre-restore stable
  TFTP query, not a post-restore replay.

After restore:

- restore API result for the named snapshot or contract-selected known state;
- lab API GET /status, GET /boot/files, and GET /boot/snapshots;
- hardware lock release time and restored=true state;
- redaction review of all durable evidence.

## Terminal Classifications

The candidate preflight may accept only these terminal classifications:

- candidate-capture-ready: candidate identity, known-good control, candidate
  run, candidate rerun, TFTP delta, serial freshness, restore, and redaction
  evidence satisfy this contract.
- blocked-candidate-identity: source, archive, fetch path, tree hash, effective
  kernel, config, or expected fetch bytes cannot be joined.
- blocked-tftp-capture: TFTP cursor/delta evidence is missing, unstable,
  post-restore-only, or cannot distinguish selected bytes from restored bytes.
- blocked-serial-capture: serial cursor/freshness evidence is missing,
  saturated without the required fallback contract, or cannot distinguish stale
  backlog from post-power output.
- blocked-known-good-control: the control cannot establish that the lab capture
  path can observe a known-good boot window under the same evidence contract.
- blocked-restore: restore fails or restored identity cannot be proven.
- inconclusive-with-required-discriminator: the evidence is retained and
  redacted, but a named smaller discriminator is required before any code change
  or hardware claim.

Only a later packet-I/O discriminator task may classify
accepted-live-device-interface-packet-io. This contract does not accept that
classification by itself.

## Redaction Contract

Durable evidence may retain task ids, source commits, archive path labels,
archive byte counts, archive digests, tree hashes, public boot config keys,
helper classifications, cursor offsets, file paths, validation commands and
results, boolean readiness flags, and metadata-only counters.

Durable evidence must not retain peer identifiers, addresses, packet payload
contents, key material, session material, boot artifact bytes, private user
data, stable secret-derived identifiers, or unnecessary hardware data.

## Explicit Rejections

This contract rejects OpenSSH compatibility, remote receipt, service success,
ssh-ready=true, generated-root retry, runtime russh adoption, fake command
expansion, broad shell work, and phase transition. It also rejects Pi 5 packet
I/O and hardware proof until later tasks retain hardware evidence under this
contract and explicitly accept those classifications.

## Findings

- fixed: defined selected_candidate_identity requirements for the next Pi 5
  preflight before any boot publication or hardware action.
- fixed: named exact evidence windows before publication, after publication,
  before restore, and after restore.
- fixed: required fresh serial cursor, TFTP cursor/delta, known-good control,
  candidate rerun, hardwareTestLock ownership, restore proof, and redaction.
- fixed: kept OpenSSH compatibility, remote receipt, service success,
  ssh-ready=true, generated-root retry, runtime russh adoption, fake command
  expansion, and phase transition outside the accepted contract.
- deferred: packet-I/O discriminator and any accepted live device-interface
  proof remain deferred to later explicit tasks.
- blocked: candidate preflight remains blocked until a later worker can acquire
  hardwareTestLock and satisfy this contract.
- not-an-issue: no Rust source or focused test rerun was required because this
  task changed only docs/task/evidence contracts.
- removed: no source, docs, helper, task, or evidence artifact was removed.

## Acceptance Check

- Task record lists findings with disposition and selected_candidate_identity
  requirements: satisfied.
- Contract names exact evidence before/after publication and before/after
  restore, including lab API identity, fresh serial cursor, TFTP logs/delta,
  known-good control, candidate rerun, and hardwareTestLock ownership:
  satisfied.
- Contract includes redaction rules excluding peer identifiers, addresses,
  packet payload contents, key/session material, boot artifact bytes, private
  user data, stable secret-derived identifiers, and unnecessary hardware data:
  satisfied.
- Contract explicitly rejects OpenSSH compatibility, remote receipt, service
  success, ssh-ready=true, generated-root retry, runtime russh adoption, fake
  command expansion, and phase transition: satisfied.
- Accepted contract is committed and selects
  phase12-ssh-live-tcp-pi5-candidate-preflight-20260629: satisfied after this
  task commit.

## Evidence Map

- Task-owned classification:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-proof-contract/classification.json.
- Task-owned evidence map:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-proof-contract/evidence-map.json.
- Accepted runtime closeout:
  tasks/2026-06-29-phase12-ssh-live-tcp-network-device-smoltcp-runtime-closeout.md.
- Lab-controller contract reviewed:
  docs/src/project/lab-controller.md.
- Docs changed:
  docs/src/project/phase12-networking-ssh.md and docs/src/roadmap.md.

## Redaction Review

Durable evidence records task ids, source commits, source paths, task paths,
public classifier names, archive metadata fields, tree-hash and boot-config
identity fields, validation commands/results, cursor offsets, and metadata-only
labels. It does not retain peer identifiers, addresses, packet payload
contents, key material, session material, boot artifact bytes, private user
data, stable secret-derived identifiers, or unnecessary hardware data.

## Validation

- git status --short --branch before edits: pass; main ahead of origin with no
  uncommitted Talos changes before task edits.
- static review of accepted runtime closeout, lab-controller docs, Phase 12
  docs, roadmap, and current supervisor state: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task: phase12-ssh-live-tcp-pi5-candidate-preflight-20260629.

planningNeeded: false.

No Pi 5 hardware/lab action, hardwareTestLock acquisition, boot publication,
power-cycle, generated-root/OpenSSH retry, live TCP attempt, packet I/O, remote
receipt claim, compatibility claim, ssh-ready=true claim, service success
claim, runtime russh adoption, fake command expansion, broad shell work, or
phase transition is accepted.

Commit: recorded in talos-supervisor-state.json after final commit.
