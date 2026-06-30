# Phase 12 SSH Live TCP Pi 5 Minimal-Entry Polled-Console Preflight V20

Task id: phase12-ssh-live-tcp-pi5-minimal-entry-polled-console-preflight-v20-20260630

Status: accepted after commit.

Classification: blocked-minimal-entry-polled-console-marker-missing.

Evidence level: serialized Pi 5 hardware preflight with lab-controller
selected-tree identity, stable TFTP selected-byte evidence, drained fresh
serial window, final pre-restore identity, restore proof, task-owned JSON
evidence, docs build, and diff checks.

## Goal

Run the repaired v17 minimal-entry polled-console archive contract on the Pi 5
and determine whether the `TALOS: minimal-entry-control-ready` marker is
retained after selected-byte service.

## Scope Performed

- Promoted the ready v20 preflight after the v17 route repair was accepted and
  committed at 6a73669b3059268c4974e6f991c0f478dea3a9a8.
- Materialized the repaired v17 minimal-entry selected image with
  `source=kernel-main-entry-control-polled-console`.
- Acquired `hardwareTestLock` before publication, power cycle, and restore.
- Published only the repaired selected archive, power-cycled once, captured
  fresh serial and TFTP evidence, recorded final pre-restore identity, and
  restored `phase12-ssh-v10-openssh-clean-pre-20260624T074100Z`.

## Hardware Result

Run `minimal-entry-polled-console-v20-20260630T195900Z` published selected tree
`3eee516f3047e8fab9b44007cbb433b67ea3c25f1ae64563b0d719e7e7be3b39` with
effective kernel `kernel_2712.img`.

Static review and publication matched the repaired v17 contract:

- selected path: `da591740/kernel_2712.img`;
- kernel byte count: 52,728;
- kernel SHA-256:
  `ccc95535706f1d896788800c8bba712cdeaac8bc6fedbbae313de06623040b33`;
- required marker:
  `TALOS: minimal-entry-control-ready capture-nonce=phase12-route-repair-v17-static`;
- no-service/no-phase-transition guard tokens remained present.

The Pi 5 run retained decisive selected-byte and restore evidence:

- pre-power serial drain was empty on the first drain attempt;
- stable same-cursor TFTP delta captured 13 events, including two selected
  `da591740/kernel_2712.img` serves at 52,728 bytes;
- post-power direct-read serial captured firmware output but zero required
  minimal-entry marker occurrences;
- final pre-restore identity remained on selected tree `3eee516f...`;
- restore returned the lab to tree
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.

Because identity, TFTP, serial freshness, final identity, and restore proof
were decisive, no known-good control or candidate rerun was required.

## Terminal Classification

blocked-minimal-entry-polled-console-marker-missing.

selected_next_task:
phase12-ssh-live-tcp-minimal-entry-polled-console-closeout-v20-20260630.

planningNeeded: false.

Packet-I/O, OpenSSH/generated-root retry, remote receipt,
compatibility/service readiness, `ssh-ready=true`, broad shell work,
fake/kernel-backed command expansion, and phase transition remain blocked.

## Findings

- fixed: executed the repaired v17 minimal-entry polled-console preflight under
  `hardwareTestLock` with the exact v17 selected-byte contract.
- not-an-issue: selected-byte service, final pre-restore identity, and restore
  proof all matched the repaired v17 candidate contract.
- deferred: the repaired `minimal-entry-control-ready` marker was still absent
  from the fresh post-power serial window; closeout must reconcile the first
  missing route fact before any broader live TCP work.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, broad shell work, fake command expansion,
  and phase transition as immediate successors.

## Evidence Map

- Evidence map:
  `tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-minimal-entry-polled-console-preflight-v20/evidence-map.json`.
- Classification:
  `tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-minimal-entry-polled-console-preflight-v20/classification.json`.
- Hardware run directory:
  `tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-minimal-entry-polled-console-preflight-v20/minimal-entry-polled-console-v20-20260630T195900Z/`.
- Static materialization:
  `tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-minimal-entry-polled-console-preflight-v20/static/`.

## Redaction Review

Task-owned lab-controller serial and TFTP artifacts are retained under the run
evidence directory. This summary records task ids, labels, hashes, byte counts,
marker names, classifications, and validation outcomes; it omits packet
payloads, SSH/session/key material, and private user data.

## Validation

- git status before action: recorded.
- static archive review: pass.
- Lab API identity before publication and after publication: pass.
- Fresh serial cursor/drain: pass; pre-power drain was empty.
- TFTP delta before restore: pass; two selected 52,728-byte fetches.
- Final pre-restore identity before restore: pass.
- Restore to predecessor-named baseline and confirm with lab API: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

Commit: recorded in talos-supervisor-state.json after final commit.
