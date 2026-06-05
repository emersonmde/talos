# Phase 11 Known-Good Runtime Readiness Contract Core

Task id: phase11-known-good-runtime-readiness-contract-core-20260605

Status: accepted

## Goal

Repair the known-good runtime-readiness evidence contract after a restored
known-good tree fetched `kernel_2712.img` but did not emit Talos readiness in
the retained serial window.

## Scope

- Compared prior accepted known-good Pi 5 runtime evidence against the current
  known-good capture/staging discriminator for restored tree
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.
- Defined the readiness invariant after an observed
  `da591740/kernel_2712.img` fetch: stable boot identity, fresh serial cursor,
  stable pre-restore TFTP fetch, bounded serial observation, exact Talos
  readiness markers, and restore state.
- Added a bounded serial observation helper so the next hardware task records
  a deterministic readiness window instead of a short or implicit observe.
- Kept the task no-hardware and did not touch RP1/runtime/kernel source.

## Non-Goals Honored

No hardwareTestLock acquisition, Pi 5 power cycle, boot archive publication,
RP1/runtime/kernel source change, candidate rerun, GPIO ownership,
interrupts, DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
Milestone 11.2 work, or phase transition was performed. This task does not
accept RP1 candidate fetch, Rust entry, entry-control reachability,
mapped/read-value, unmapped/trap, or firmware-state behavior.

## Findings And Disposition

- fixed: prior accepted known-good controls on the same restored tree show
  `TALOS: kernel_main` plus accepted success output after the
  104,136-byte control kernel fetch.
- fixed: the latest discriminator accepted capture/staging health only: final
  stable pre-restore TFTP replay from fresh cursor 4094251 contained 13
  events, including two 104,136-byte `da591740/kernel_2712.img` serves, but
  the retained serial observation ended at early firmware/RP1 output and did
  not reach `TALOS: kernel_main`, `talos>`, or PASS.
- fixed: `scripts/rpi5-observe-runtime-readiness.sh` now records an explicit
  default 75-second, 1000 ms settle, 65536-byte serial readiness window from a
  fresh cursor and annotates `valid-known-good-talos-readiness` only when
  `TALOS: kernel_main` and the required success marker are present.
- fixed: the Phase 11/lab-controller docs now separate known-good fetch
  visibility from known-good runtime readiness and name the exact default
  marker for the restored control as `rpi5-production-timer-preemption: PASS`.
- deferred: the next serialized Pi 5 task must run the bounded readiness
  discriminator; this no-hardware task cannot accept or reject runtime
  readiness by itself.
- removed: no open-ended wait stack, alternate capture channel, candidate
  rerun, source change, or boot publication was added.
- not-an-issue: the helper supports `TALOS_READINESS_REQUIRED_MARKER` for
  future accepted known-good controls, but each hardware proof must record the
  exact marker it used.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-05-phase11-known-good-runtime-readiness-contract-core/evidence-map.json.
- Static inspection:
  tasks/evidence/2026-06-05-phase11-known-good-runtime-readiness-contract-core/static-inspection.md.
- Helper syntax check:
  tasks/evidence/2026-06-05-phase11-known-good-runtime-readiness-contract-core/sh-n-rpi5-observe-runtime-readiness.log.
- Diff hygiene:
  tasks/evidence/2026-06-05-phase11-known-good-runtime-readiness-contract-core/git-diff-check.log.
- Docs validation:
  tasks/evidence/2026-06-05-phase11-known-good-runtime-readiness-contract-core/mdbook-build.log.
- Staged diff hygiene:
  tasks/evidence/2026-06-05-phase11-known-good-runtime-readiness-contract-core/git-diff-cached-check.log.

## Validation

- static source/doc/evidence inspection: passed.
- helper syntax check: `sh -n scripts/rpi5-observe-runtime-readiness.sh`
  passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed with the existing large
  search-index warning.
- git diff --cached --check before commit: passed.

## Result

Accepted contract repair. The next queued serialized task may acquire
hardwareTestLock only if the lock is unlocked/restored, then run one
known-good power cycle with the repaired readiness contract. It must classify
known-good runtime readiness separately from TFTP fetch visibility and still
must not accept RP1 candidate/source behavior.
