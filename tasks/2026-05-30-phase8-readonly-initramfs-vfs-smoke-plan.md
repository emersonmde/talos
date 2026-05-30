# Phase 8 Read-Only Initramfs/VFS Smoke Plan Task

Task: phase8-readonly-initramfs-vfs-smoke-plan-20260530

Status: accepted

## Scope

Documentation-only QEMU/substitute smoke plan after the accepted Phase 8
read-only initramfs/VFS contract. The task defines the deterministic fixture,
lookup/read/offset/EOF observations, errno cases, exact PASS/classification
output, failure classification, retained evidence path, regression gates, and
next bounded implementation tasks.

Non-goals: no Rust, assembly, QEMU execution, Pi 5 hardware run, boot archive
publication, hardwareTestLock acquisition, VFS implementation, initramfs
parser, descriptor-backed filesystem read, ELF/program loader, argv/envp setup,
process creation, shell, networking, SSH, RP1/PCIe, UART interrupt ownership,
or DMA/cache-driver policy.

## Evidence

- static inspection: git status --short before edits was clean.
- static documentation review: reviewed the accepted read-only initramfs/VFS
  contract, Phase 8 source inventory, roadmap, ADR index, and prior QEMU smoke
  plan patterns for exact output, evidence retention, and deferred-surface
  language.
- documentation diff: added
  docs/src/project/phase8-readonly-initramfs-vfs-smoke-plan.md, linked it from
  docs/src/SUMMARY.md, updated docs/src/roadmap.md and
  docs/src/decisions/README.md, and added this task record.
- fixture/PASS specification: required retained log path is
  tasks/evidence/2026-05-30-qemu-readonly-initramfs-vfs-smoke-core/qemu-readonly-initramfs-vfs-smoke.log;
  required final lines are
  qemu-readonly-initramfs-vfs-smoke: final participants=8 expected=8 errors=0
  classification=qemu-readonly-initramfs-vfs-smoke-complete and
  qemu-readonly-initramfs-vfs-smoke: PASS.
- regression gates: cargo fmt, cargo test, the planned smoke script, shared
  scalar/descriptor/read smokes only when touched, git diff --check, and
  mdbook build when docs are touched.
- recommendation: next bounded task should be
  phase8-readonly-initramfs-vfs-core-20260530; after that, the QEMU/substitute
  evidence task should be
  phase8-qemu-readonly-initramfs-vfs-smoke-core-20260530.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

## Result

Accepted and committed as the documentation-only read-only initramfs/VFS smoke
plan. No Phase 8 runtime behavior, QEMU evidence, or Pi 5 hardware evidence is
accepted by this task. The target-independent core implementation remains the
next bounded task; QEMU/substitute smoke evidence, Pi 5 hardware proof,
ELF/program loading, argv/envp setup, process creation, shell, networking, SSH,
RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy remain blocked
until later explicit tasks accept their contracts and gates.
