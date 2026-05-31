# Phase 10 Local Pwd Command Closeout Checkpoint Task

Task: phase10-local-pwd-command-closeout-checkpoint-20260531

Status: accepted

## Scope

Close out the root-only `pwd` local command feature as a documentation-only
checkpoint. The checkpoint reconciles accepted QEMU/substitute and serialized
Pi 5 evidence, records the accepted descriptor-backed `pwd` frontier and
current-directory placeholder limits, records deferred shell/userspace/filesystem
surfaces, and recommends the next smallest feature-led Phase 10 local
interactivity task.

Changed files:

- docs/src/project/phase10-local-pwd-command-closeout-checkpoint.md
- docs/src/SUMMARY.md
- docs/src/roadmap.md
- tasks/2026-05-31-phase10-local-pwd-command-closeout-checkpoint.md

No Rust or assembly behavior changed. No QEMU scenario was rerun. No Pi 5
hardware action, boot archive publication, or hardwareTestLock acquisition was
performed.

## Evidence

- Closeout doc:
  docs/src/project/phase10-local-pwd-command-closeout-checkpoint.md.
- Retained QEMU/substitute `pwd` transcript:
  tasks/evidence/2026-05-31-qemu-local-pwd-command-core/qemu-local-pwd-command-smoke.log.
- Retained Pi 5 accepted serial transcript:
  tasks/evidence/2026-05-31-pi5-local-pwd-command-proof/local2-clean-candidate/serial-transcript.txt.
- Retained Pi 5 normalized serial transcript:
  tasks/evidence/2026-05-31-pi5-local-pwd-command-proof/local2-clean-candidate/serial-transcript-normalized.txt.
- Retained Pi 5 proof summary:
  tasks/evidence/2026-05-31-pi5-local-pwd-command-proof/local2-clean-candidate/proof-result-local2.txt.
- Retained archive/image review:
  tasks/evidence/2026-05-31-pi5-local-pwd-command-proof/local2-clean-candidate/archive-review.txt.
- Retained TFTP proof:
  tasks/evidence/2026-05-31-pi5-local-pwd-command-proof/local2-clean-candidate/tftp-kernel-fetch-local2.txt.
- Retained restore proof:
  tasks/evidence/2026-05-31-pi5-local-pwd-command-proof/local2-clean-candidate/post-snapshot-restore-status.json.
- Retained inconclusive-run triage:
  tasks/evidence/2026-05-31-pi5-local-pwd-command-proof/local1-clean-candidate/
  and
  tasks/evidence/2026-05-31-pi5-local-pwd-command-proof/local1-known-good-control/.

## Accepted Frontier

Talos can print the serial prompt, accept typed `pwd` through
fd0/runtime-console0 canonical-lite input, dispatch the kernel-backed `pwd`
built-in, print the visible `/` response through descriptor-backed stdout,
preserve descriptor-backed input/output markers, and return to a ready prompt.

The current-directory placeholder is root-only and process-local in shape. It
does not claim `cd`, path traversal, path normalization, VFS lookup, directory
listing, userspace process execution, or filesystem command lookup.

Existing `help`, `status`, `stdio`, `echo hello`, empty-input, and
unknown-input behavior remains deterministic.

## Deferred Surfaces

Deferred after this checkpoint: `cd`, path traversal/normalization, VFS
lookup, directory listing, userspace shell execution, external commands,
process lifecycle, filesystem-backed commands, broad POSIX read/stdio
readiness, argv/envp process ABI, quoting/escaping/globbing, pipes,
redirection, termios/job control, networking, SSH, RP1/PCIe, UART interrupts,
DMA/cache policy, and paused Phase 8 proof-only work.

## Next Recommendation

Recommend the next feature-led Phase 10 task as bounded Backspace/Delete line
editing over the accepted descriptor-backed serial input path. The expected
user-visible behavior is `talos>`, a mistyped command character, Backspace or
Delete, corrected input, Enter, dispatch of the corrected command, visible
response, and a ready prompt. The task should stay below termios, cursor
addressing, shell history, general escape-sequence parsing, userspace process
execution, and filesystem command lookup.

## Validation

- static inspection: retained QEMU/substitute and Pi 5 evidence paths reviewed.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

HardwareTestLock remained unlocked/restored and unused by this checkpoint.

## Commit

Closeout commit: recorded in durable supervisor state after commit.
