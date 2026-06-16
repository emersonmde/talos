# Phase 12.1 RP1 Ethernet Serial Freshness Guard Core

Task:
phase12-rp1-ethernet-serial-freshness-guard-core-20260616.

Status: accepted

Classification:
serial-freshness-guard-core-local-static-accepted.

Evidence level: static source/doc/task inspection, shell syntax checks,
task-owned synthetic validator replay, JSON evidence validation, diff checks,
and docs build. No Pi 5 hardware run, boot archive publication, lab mutation,
hardwareTestLock acquisition, power-cycle, serial write, Ethernet/MMIO/MDIO/MAN
or register access, GPIO32/PHY reset, BMCR/autoneg write, packet I/O,
networking, SSH, Phase 12.2, or phase transition was performed.

## Goal

Implement the local/static guard surface for
cursor-nonce-post-power-freshness-v1 before any serialized Pi 5 freshness proof.

## Findings

- fixed: Added `scripts/rpi5-serial-freshness-guard-v1-check.sh`, a retained
  evidence replay guard that checks pre-power retained serial absence,
  post-power cursor/nonce presence, selected-tree/TFTP identity, final identity,
  and restore proof.
- fixed: Updated `scripts/rpi5-capture-invariant-proof-bundle.sh` to retain
  `pre-power-serial-peek.json`, advertise the v1 freshness contract in dry-run
  metadata, and emit freshness fields in `capture-invariant-summary.json`.
- fixed: Retained synthetic validator evidence accepts one well-formed
  cursor-fresh bundle and rejects stale backlog, cursor mismatch, missing
  marker, selected-tree/TFTP mismatch, and restore failure.
- fixed: Updated lab-controller docs and roadmap with the new guard command,
  output boundary, and selected serialized Pi 5 proof follow-up.
- rejected: The guard accepts serial freshness and capture-chain identity only;
  it does not accept BCM54213PE register values, link readiness, Ethernet
  readiness, GPIO32/PHY reset ownership, BMCR/autoneg, Broadcom shadow/MMD/aux
  access, interrupt ownership, packet I/O, networking, SSH, Phase 12.2, or a
  phase transition.
- removed: No source helpers, docs, task records, or evidence records were
  removed.
- not-an-issue: Empty pre-power drain remains accepted as strong positive
  evidence, while non-empty drain can pass only with run-unique cursor/nonce
  freshness and the selected-tree/TFTP/final-identity/restore join.

## Guard Contract

The guard contract is `cursor-nonce-post-power-freshness-v1`.

Required retained inputs:

- `preflight-identity.json`.
- `pre-power-serial-peek.json`.
- `serial-drain-before-power.json`.
- `serial-observe-window.json`.
- `tftp-delta-stable-pre-restore.json`.
- `final-pre-restore-status.json`.
- `final-pre-restore-boot-files.json`.
- `restore-snapshot.json`.
- `post-restore-status.json`.
- Optional cursor text files and `capture-invariant-summary.json` when present.

Accepted output means the run-unique marker/nonce was absent from the immediate
pre-power retained serial sample, present in the post-power capture bound to the
saved drain cursor or saturated direct-read fallback, and joined with matching
selected-tree, TFTP, final pre-restore identity, and restore proof.

Rejected output records the first applicable classes from the accepted contract:
stale backlog, cursor mismatch, missing marker, nonce-not-unique,
selected-tree/TFTP mismatch, final identity mismatch, restore failure, saturated
direct-read without nonce proof, or inconclusive capture.

## Evidence

- contract input:
  tasks/2026-06-16-phase12-rp1-ethernet-serial-freshness-contract.md.
- guard script:
  scripts/rpi5-serial-freshness-guard-v1-check.sh.
- capture helper:
  scripts/rpi5-capture-invariant-proof-bundle.sh.
- synthetic replay summary:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-serial-freshness-guard-core/validator-results.json.
- task-owned classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-serial-freshness-guard-core/classification.json.
- task-owned evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-serial-freshness-guard-core/evidence-map.json.

## Validation

- shell syntax: `sh -n scripts/rpi5-serial-freshness-guard-v1-check.sh scripts/rpi5-capture-invariant-proof-bundle.sh`.
- dry-run metadata:
  `scripts/rpi5-capture-invariant-proof-bundle.sh --dry-run ...`.
- task-owned validator:
  `scripts/rpi5-serial-freshness-guard-v1-check.sh` over retained synthetic
  fixtures, summarized in `validator-results.json`.
- JSON validation:
  `jq empty tasks/evidence/2026-06-16-phase12-rp1-ethernet-serial-freshness-guard-core/**/*.json`.
- diff check: `git diff --check`.
- docs validation: `/home/node/.cargo/bin/mdbook build`.
- staged diff check: `git diff --cached --check`.

## Acceptance Check

- Local/static guard core records accepted freshness fields and produces
  task-owned classification/evidence-map JSON: satisfied.
- Validator rejects stale/mismatched evidence classes and accepts a well-formed
  synthetic cursor-fresh case: satisfied.
- Marker-only scenario and synthetic evidence contain no Ethernet, MDIO, MAN,
  MACB, GPIO32/PHY, packet, networking, SSH, Phase 12.2, or phase-transition
  target facts: satisfied.
- Next serialized Pi 5 proof remains dependency-gated on this accepted core,
  hardware lock availability, clean working tree, and inactive intervention:
  satisfied.

## Next Action

Mechanically promote
phase12-rp1-ethernet-serial-freshness-pi5-proof-20260616 on the next worker
wake if dependencies remain satisfied, hardwareTestLock remains
unlocked/restored, supervisor intervention remains inactive, and the repository
has no conflicting uncommitted changes. Do not start register-read retry,
packet I/O, networking, SSH, Phase 12.2, or a phase transition from this guard
core.
