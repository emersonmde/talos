# Phase 7 Syscall Routing Closeout Checkpoint

Task: phase7-syscall-routing-closeout-checkpoint-20260529
Status: accepted

## Scope

This documentation checkpoint reconciles the accepted Phase 7.3 syscall ABI
contract, target-independent dispatch core, production trap-routing contract,
QEMU syscall smoke evidence, deferred surfaces, and next bounded task. It did
not add Rust or assembly behavior, rerun QEMU, run Pi 5 hardware, publish boot
archives, acquire hardwareTestLock, add descriptor I/O, byte copy-in/copy-out,
pointer-taking syscalls, process loading, VFS/filesystem behavior, shell
behavior, networking, SSH, RP1/PCIe work, UART interrupt ownership, or
DMA/cache-driver policy.

## Accepted Evidence

- syscall ABI contract commit:
  380994e6003c048c4b88497e52c327c18ca3dffd.
- target-independent syscall dispatch core commit:
  734160cee68e69c02c0aea124ba185ea7e36bdc3.
- production syscall trap-routing contract commit:
  10aa4423db70b80a134edc31dbb4c7c34a9f7554.
- QEMU syscall smoke core commit:
  3abaf63ec11830137df15f0e3947161cad11688c.
- retained QEMU syscall smoke log:
  tasks/evidence/2026-05-29-qemu-syscall-smoke-core/qemu-syscall-smoke.log.
- retained QEMU diagnostic preservation log:
  tasks/evidence/2026-05-29-qemu-syscall-smoke-core/qemu-el0-trap-smoke.log.

Accepted QEMU/substitute classification:

~~~text
qemu-syscall-smoke: final participants=2 expected=2 errors=0 classification=qemu-syscall-smoke-complete
qemu-syscall-smoke: PASS
~~~

## Deferred Work

Pi 5 production syscall proof, descriptor I/O, byte copy-in/copy-out,
pointer-taking syscalls, process loading, VFS/filesystem behavior, shell
behavior, networking, SSH, RP1/PCIe, UART interrupt ownership, and
DMA/cache-driver policy remain blocked until later explicit tasks.

## Next Task Recommendation

Recommend supervisor planning for phase7-pi5-syscall-proof-plan-20260529 as
the next bounded documentation-only task if the program wants to establish
physical production syscall evidence before copy-in/copy-out or descriptor
syscall work. A copy-in/copy-out helper contract or descriptor syscall contract
would also be safe future work, but choosing among those alternatives is a
planning decision rather than a mechanically objective worker promotion.

## Evidence

- static inspection: git status --short before edits was clean.
- static inspection: retained QEMU syscall and diagnostic smoke logs were
  reviewed.
- static documentation diff: added
  docs/src/project/phase7-syscall-routing-closeout-checkpoint.md, linked it
  from docs/src/SUMMARY.md, updated docs/src/roadmap.md, updated
  docs/src/decisions/README.md, and added this task record.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
