# Phase 12 Entropy Source Contract

Task id: phase12-entropy-source-contract-20260621

Status: accepted

Classification:
phase12-entropy-source-contract-accepted

Evidence level: static source/task/docs/evidence review, docs build, and diff
checks. No runtime implementation, crypto dependency adoption, host key
generation, SSH service, hardware/lab action, live packet I/O, hardware
reachability, public ABI/POSIX/Linux compatibility claim, broad expansion, or
phase transition was performed.

## Goal

Create a bounded source/task/docs contract for the first Talos entropy
diagnostic/source surface needed before SSH host key or session crypto work.

## Scope Performed

- Reviewed the accepted SSH strategy contract and network frontier pause.
- Reviewed current source surfaces for entropy/random/crypto/SSH terms.
- Reviewed source-owned timing, scheduler, boot/device-tree, console, socket,
  and lab-facing diagnostic surfaces that a future entropy diagnostic might
  otherwise try to consume.
- Defined accepted and rejected entropy input classes for the next local/source
  diagnostic implementation.
- Defined deterministic validation controls and fail-closed output labels.
- Selected phase12-entropydiag-core-20260621 as the next bounded task.

## Findings

- fixed: the next implementation boundary is a diagnostic/classification
  surface, not a random-byte generator. It may classify available inputs and
  fail-closed behavior but must not emit host keys, session keys, SSH readiness,
  or cryptographic-strength random output.
- fixed: accepted input candidates are limited to source-grounded local facts
  that the diagnostic can classify as untrusted input material: generic timer
  counter/tick samples, scheduler or process event counters once explicitly
  exposed to the diagnostic, console/serial input timing deltas when paired with
  a timer sample, and future operator-provisioned seed material.
- fixed: deterministic validation controls are required: no-input, fixed-timer,
  fixed-scheduler-event, fixed-console-timing, fixed-operator-seed, and mixed
  fixed-input cases must produce stable labels without claiming entropy.
- fixed: fail-closed labels are now concrete enough for implementation:
  entropydiag-fail-closed-no-input, entropydiag-deterministic-control,
  entropydiag-untrusted-timer-only, entropydiag-untrusted-local-mix,
  entropydiag-operator-seed-required, and entropydiag-hardware-rng-unaccepted.
- deferred: a real cryptographic RNG, conditioner/DRBG choice, host-key
  generation policy, seed persistence, authorized-key storage, memory lifetime
  policy, and SSH service wiring remain later tasks.
- deferred: hardware RNG claims remain unavailable until source evidence names
  a concrete Pi 5/BCM2712/RP1 randomness source and a later task proves its
  access path.
- rejected: deterministic boot constants, DTB addresses, kernel layout,
  initramfs contents, generated-root manifests, fixed task IDs, fixed socket
  diagnostic payloads, lab API metadata, TFTP byte counts, and serial
  transcripts are not accepted as entropy sources by themselves.
- rejected: lab-provided randomness or external randomness services are not part
  of this contract.
- rejected: cryptographic-strength, SSH readiness, hardware randomness, live
  packet I/O, reachability, public ABI/POSIX/Linux compatibility, broad
  expansion, and phase transition claims remain unaccepted.
- removed: no source behavior, dependency, hardware/lab helper, task evidence,
  or documentation path was removed.
- not-an-issue: selecting a diagnostic that only classifies and fails closed is
  compatible with the SSH strategy; it exposes the missing kernel responsibility
  before any crypto or SSH dependency can hide it.

## Contract

The next implementation task may add a small Talos-owned entropy diagnostic
core. The diagnostic should consume an explicit snapshot of caller-supplied
input observations rather than ambient hardware or lab state. The accepted
snapshot fields are:

- optional generic timer physical-count/tick observations, labeled untrusted;
- optional scheduler/process event observations, labeled untrusted until the
  diagnostic owns the exact event source;
- optional console/serial input timing observations paired with timer samples,
  labeled untrusted;
- optional operator-provisioned seed material, labeled provisioned but not
  cryptographic-strength by itself in this task.

The diagnostic must classify input availability and failure modes. It must not
generate random bytes, derive keys, persist seeds, expose a public ABI, or imply
that SSH can start. The strongest accepted output from the next task is a
source/unit diagnostic label proving that Talos can distinguish no-input,
deterministic-control, untrusted-local-input, and operator-seed-required states.

## Deterministic Controls

The implementation task must include tests for these controls:

- no accepted input returns entropydiag-fail-closed-no-input;
- fixed timer-only input returns entropydiag-untrusted-timer-only;
- fixed local-event mixes return entropydiag-untrusted-local-mix;
- fixed deterministic test seeds return entropydiag-deterministic-control;
- no hardware RNG source returns entropydiag-hardware-rng-unaccepted;
- accepted local inputs without provisioned seed material preserve an
  operator-seed-required indication for SSH-key readiness.

All controls must be repeatable under source/unit or host/QEMU-substitute tests.
Any non-repeatability claim, hardware randomness claim, or live lab observation
requires a later explicit task.

## Evidence

- Strategy predecessor:
  tasks/2026-06-21-phase12-entropy-ssh-strategy-contract.md.
- Network frontier pause:
  tasks/2026-06-21-phase12-network-frontier-pause-and-ssh-strategy-checkpoint.md.
- Source review: Cargo.toml, src/arch/aarch64/generic_timer.rs,
  src/scheduler.rs, src/boot/mod.rs, src/boot/rpi5_reports.rs,
  src/device_tree/chosen.rs, src/pl011.rs, src/runtime_console.rs,
  src/diagnostic_command.rs, src/local_command_loop.rs, src/network.rs,
  src/syscall.rs, and repository-wide search for entropy/random/crypto/SSH
  terms.
- Docs: docs/src/project/phase12-networking-ssh.md and docs/src/roadmap.md.

## Validation

- static source/task/docs/evidence review: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Acceptance Check

- Contract records findings with disposition and names accepted/rejected entropy
  inputs: satisfied.
- Contract defines deterministic validation controls and fail-closed labels for
  diagnostic output: satisfied.
- selected_next_task=phase12-entropydiag-core-20260621: satisfied.
- No cryptographic-strength, SSH readiness, hardware randomness, live packet
  I/O, reachability, or phase-transition claim is accepted: satisfied.

## Next Action

Promote phase12-entropydiag-core-20260621 on the next worker wake if
dependencies remain satisfied. The next task must implement only the accepted
diagnostic/classification boundary and tests; it must not generate host keys,
emit cryptographic random bytes, adopt an SSH service, claim hardware
randomness, perform hardware/lab action, or accept SSH readiness.
