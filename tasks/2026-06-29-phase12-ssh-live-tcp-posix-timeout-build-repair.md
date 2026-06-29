# Phase 12 SSH Live TCP POSIX Timeout Build Repair

Task id: phase12-ssh-live-tcp-posix-timeout-build-repair-20260629

Status: accepted after commit.

Classification: candidate-build-ready.

Evidence level: source repair, POSIX/syscall errno tests, full no_std unit
suite through the QEMU runner, local Pi 5 boot-tree materialization, docs
build, JSON validation, and diff checks. No hardwareTestLock was acquired, no
lab or hardware action was performed, no boot archive was published, and no
power-cycle, serial/TFTP capture, known-good control, candidate run/rerun,
packet-I/O discriminator, OpenSSH/generated-root retry, remote receipt,
compatibility claim, service success claim, ssh-ready=true claim, runtime russh
adoption, fake command expansion, broad shell work, or phase transition was
performed.

## Goal

Repair the source-level `PosixError::TimedOut` build blocker that stopped the
previous Pi 5 candidate preflight, then prove the current source can locally
materialize a Pi 5 boot tree without publishing it.

## Scope Performed

- Added `PosixError::TimedOut` with POSIX name `ETIMEDOUT`.
- Added `ETIMEDOUT = 110` to the syscall errno table and covered it in the
  existing negative-x0 syscall encoding test.
- Updated the Phase 7 POSIX baseline and early POSIX shape docs to include the
  explicit bounded-timeout errno vocabulary.
- Built a local Pi 5 boot tree through `scripts/rpi5-boot-tree.sh` without
  lab publication or hardware interaction.
- Updated the Phase 12 live TCP frontier docs to select the replacement
  candidate preflight task.

## Timeout/Error Contract

The repair extends the accepted POSIX/syscall vocabulary rather than remapping
the network timeout to `EAGAIN` or `EIO`.

`EAGAIN` remains the nonblocking/resource-temporary result already used for
would-block and retry-exhaustion paths. `EIO` remains an unclassified backend
I/O failure. `ETIMEDOUT` now names an explicit bounded timeout, matching the
network runtime path at `src/network.rs` where the smoltcp client/server
runtime proof exhausts its bounded step budget.

## Findings

- fixed: `src/network.rs` now compiles against the POSIX error vocabulary
  because `PosixError::TimedOut` exists.
- fixed: `PosixError::TimedOut.name()` returns `ETIMEDOUT`.
- fixed: syscall errno encoding maps `PosixError::TimedOut` to
  `ETIMEDOUT = 110` and returns negative x0 in the existing encoding test.
- fixed: local Pi 5 boot-tree materialization now succeeds without lab
  publication.
- fixed: docs record the extended public errno vocabulary and candidate-build
  frontier.
- not-an-issue: no hardware lock was acquired because this task is explicitly
  source/local-materialization only.
- removed: no dead code or stale helper was removed; the repair was narrower
  than a source refactor.
- deferred: Pi 5 publication, capture preflight, known-good control, candidate
  run/rerun, and packet-I/O discriminator remain deferred to the explicitly
  queued replacement hardware task.

## Materialization Evidence

- Command:
  `scripts/rpi5-boot-tree.sh target/rpi5-production-timer-preemption-boot-tree target/tmp/posix-timeout-build-repair-20260629T110046Z-boot-tree`
- Output path:
  `target/tmp/posix-timeout-build-repair-20260629T110046Z-boot-tree`
- Materialized file manifest hash:
  `3c6c153d4f13e7891e2085c946c9f6125db0f0f41de17901efed5dfe5056a02f`
- `kernel_2712.img`: 87432 bytes,
  `516b0014eaead2a090779fba7bd8ea4da630f71e923e01182f0570aa9fc2de43`
- `kernel8.img`: 87432 bytes,
  `516b0014eaead2a090779fba7bd8ea4da630f71e923e01182f0570aa9fc2de43`
- `config.txt`: 118 bytes,
  `8e2972a9f5c17b887d556a4e1048551e46ffe239751a9ad92f64f9222eae2ad8`
- `cmdline.txt`: 81 bytes,
  `2f4eada7b5b83810b796594d8a9d8928aa5d3c6c728bd109ffa117fd3e647f08`

## Evidence Map

- Classification:
  `tasks/evidence/2026-06-29-phase12-ssh-live-tcp-posix-timeout-build-repair/classification.json`.
- Evidence map:
  `tasks/evidence/2026-06-29-phase12-ssh-live-tcp-posix-timeout-build-repair/evidence-map.json`.

## Redaction Review

Durable evidence retains task ids, source paths, public errno names/numbers,
validation commands/results, local artifact paths, byte counts, and SHA-256
hashes. It does not retain peer identifiers, packet payload contents, key
material, session material, boot artifact bytes, private user data, stable
secret-derived identifiers, or hardware/lab captures.

## Validation

- git status --short --branch before edits: pass; main ahead of origin with no
  uncommitted Talos changes before task edits.
- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test --quiet
  posix_error_names_match_errno_style_vocabulary: pass; the custom no_std QEMU
  runner executed the full suite, including POSIX name, syscall errno, and
  network timeout/runtime tests; 892 passed.
- scripts/rpi5-boot-tree.sh on the accepted candidate input/output paths:
  pass; local boot tree materialized at the path above with no publish or
  hardware action.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass; warning only: search index is very
  large.
- git diff --cached --check: pass before commit.

## Result

selected_next_task: phase12-ssh-live-tcp-pi5-candidate-preflight-v2-20260629.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
