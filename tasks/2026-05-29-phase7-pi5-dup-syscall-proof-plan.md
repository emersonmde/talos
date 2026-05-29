# Phase 7 Pi 5 Dup Syscall Proof Plan

Task: phase7-pi5-dup-syscall-proof-plan-20260529
Owner: worker
Status: accepted
Date: 2026-05-29
Milestone: Phase 7.4, File Descriptor Table

## Goal

Define the serialized Raspberry Pi 5 dup syscall proof before any hardware
action.

## Scope

- Added docs/src/project/phase7-pi5-dup-syscall-proof-plan.md.
- Linked the plan from docs/src/SUMMARY.md.
- Updated docs/src/roadmap.md and docs/src/decisions/README.md with the
  accepted documentation-only Pi 5 dup proof plan.
- Promoted the already queued task after
  phase7-dup-syscall-closeout-checkpoint-20260529 was accepted and
  hardwareTestLock was unlocked/restored.

## Non-Goals

- No Rust or assembly implementation changes.
- No boot archive publication, hardwareTestLock acquisition, power cycle,
  serial observe, or Pi 5 hardware test.
- No read syscall work, process loading, VFS/filesystem, shell, networking,
  SSH, object finalization, RP1/PCIe, UART interrupt ownership,
  DMA/cache-driver policy, dup2/fcntl, or full POSIX descriptor readiness
  claim.

## Accepted Evidence

- Accepted QEMU/substitute dup evidence:
  tasks/evidence/2026-05-29-qemu-dup-syscall-smoke-core/qemu-dup-syscall-smoke.log.
- Static documentation diff:
  docs/src/project/phase7-pi5-dup-syscall-proof-plan.md defines the exact
  physical invariant, expected serial lines, diagnostic quarantine,
  hardwareTestLock protocol, restoration requirements, and inconclusive-run
  triage.
- Required PASS/classification for the future hardware task:
  rpi5-dup-syscall-proof final participants=14 expected=14 errors=0
  classification=pi5-dup-syscall-proof-complete plus PASS.

## Validation

- static inspection: git status --short before edits was clean.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- Rust fmt/tests, QEMU runs, Pi 5 hardware runs, archive publication, and
  hardware-lock work were not required because this task changes only Markdown
  documentation and durable worker state.

## Result

Accepted as the documentation-only Pi 5 dup syscall proof plan. The next
bounded task is phase7-pi5-dup-syscall-proof-20260529, which may be promoted
only if the accepted plan remains intact and hardwareTestLock is
unlocked/restored.
