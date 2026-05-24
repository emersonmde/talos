# Phase 6 QEMU Secondary-Core Bring-Up Discriminator

Task: `phase6-qemu-secondary-core-bringup-discriminator-20260524`
Status: accepted

## Scope

This task proves the accepted Phase 6.1 secondary-core bring-up contract in
QEMU/substitute only. It adds a focused boot diagnostic behind
`TALOS_QEMU_SECONDARY_CORE_DISCRIMINATOR` and
`scripts/qemu-secondary-core-discriminator.sh`.

The diagnostic starts QEMU virt with EL2 virtualization, GICv2, Cortex-A76, and
four CPUs. The boot CPU calls PSCI `CPU_ON` through SMC for logical CPUs 1, 2,
and 3, using a narrow secondary entry trampoline that selects a reserved stack
slot before entering Rust. Secondary CPUs record MPIDR identity, logical mapping,
stack pointer, context argument, and the state transition to `handoff-ready`,
then park in `wfe`.

This does not publish a Pi 5 boot archive, power-cycle hardware, acquire the
hardware lock, add production SMP scheduler behavior, add SMP-safe locks, migrate
tasks, add cross-core preemption, add userspace, syscalls, descriptors,
filesystem, networking, SSH, shell behavior, or weaken Pi 5 hardware acceptance.

## Transcript Summary

Captured QEMU/substitute evidence:

```text
qemu-secondary-core-discriminator: start conduit=smc cores=4 boot-mpidr=0x0000000080000000 boot-affinity=0x0 boot-logical=Some(0) entry=0x0000000040200090 stack-range=[0x000000004021f000,0x0000000040223000)
qemu-secondary-core-discriminator: cpu-on logical=1 target-affinity=0x1 result=0
qemu-secondary-core-discriminator: cpu-on logical=2 target-affinity=0x2 result=0
qemu-secondary-core-discriminator: cpu-on logical=3 target-affinity=0x3 result=0
qemu-secondary-core-discriminator: report logical=1 state=handoff-ready context=1 mpidr=0x0000000080000001 affinity=0x1 mapped=Some(1) sp=0x0000000040220fa0 stack=[0x0000000040220000,0x0000000040221000) ok=true
qemu-secondary-core-discriminator: report logical=2 state=handoff-ready context=2 mpidr=0x0000000080000002 affinity=0x2 mapped=Some(2) sp=0x0000000040221fa0 stack=[0x0000000040221000,0x0000000040222000) ok=true
qemu-secondary-core-discriminator: report logical=3 state=handoff-ready context=3 mpidr=0x0000000080000003 affinity=0x3 mapped=Some(3) sp=0x0000000040222fa0 stack=[0x0000000040222000,0x0000000040223000) ok=true
qemu-secondary-core-discriminator: wait-remaining=10000000 classification=qemu-psci-smc-secondary-cores-alive
qemu-secondary-core-discriminator: PASS
```

During development, an HVC `CPU_ON` attempt from the EL2 diagnostic path raised
a current-SPx synchronous exception with ESR `0x5a000000`. The accepted
discriminator therefore uses SMC under QEMU's EL2 boot model and records that
classification separately from Pi 5 hardware proof.

## Classification

- QEMU PSCI SMC `CPU_ON` can start secondary CPUs in the current EL2 virt boot
  model.
- QEMU MPIDR affinity values map logical CPUs as `0x0`, `0x1`, `0x2`, and
  `0x3`.
- The diagnostic proves core identity, per-core stack ownership, state
  registration, and controlled parked handoff for QEMU/substitute.
- Pi 5 still requires serialized hardware evidence with firmware DTB/SMC,
  TFTP/archive proof, serial transcript, distinct MPIDR identities, distinct
  stack ownership, and post-run review.

## Validation

- fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test` passed 92 no_std tests.
- QEMU/substitute: `scripts/qemu-smoke.sh` passed and preserved the accepted
  generic QEMU smoke output.
- QEMU/substitute: `scripts/qemu-secondary-core-discriminator.sh` passed with
  the transcript summary above.
- image/archive inspection: `scripts/rpi5-image.sh` passed because shared boot
  assembly and build configuration changed.
- fmt/lint/typecheck: `git diff --check` and `git diff --cached --check`
  passed.
- static inspection: `mdbook` was unavailable in the container, so mdBook build
  was not run.

## Next Task

The next queued Milestone 6.1 task is
`phase6-per-core-state-and-stacks-20260524`. It may build on the QEMU
diagnostic's identity and stack evidence, but Pi 5 hardware proof, SMP-safe
locking, scheduler migration, cross-core preemption, filesystem, networking,
SSH, and shell behavior remain out of scope until supervisor-planned tasks.
