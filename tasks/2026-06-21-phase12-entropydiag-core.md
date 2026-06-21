# Phase 12 Entropy Diagnostic Core

Task id: phase12-entropydiag-core-20260621

Status: accepted

Classification:
phase12-entropydiag-core-accepted

Evidence level: source/unit tests, full host/QEMU-substitute no_std test suite,
docs build, and diff checks. No host key generation, SSH server, cryptographic
RNG strength claim, hardware randomness claim, hardware/lab action, live packet
I/O, reachability, public ABI/POSIX/Linux compatibility claim, broad expansion,
or phase transition was performed.

## Goal

Implement only the source/local diagnostic core accepted by the entropy source
contract, with deterministic controls and fail-closed output labels.

## Scope Performed

- Added src/entropy.rs as a Talos-owned classifier over caller-supplied
  diagnostic observations.
- Exposed an internal diagnostic command, entropy, that reports the default
  no-input fail-closed status without sampling hardware or generating bytes.
- Added focused unit coverage for no-input fail-closed behavior, fixed timer
  input, fixed local-event mixes, deterministic test seed controls, hardware RNG
  rejection, operator-seed-required indication, and negative crypto/SSH claims.
- Updated Phase 12 documentation and roadmap with the accepted diagnostic core
  boundary and next closeout task.

## Findings

- fixed: entropy classification is implemented as a pure source/unit core over
  an explicit EntropyDiagnosticSnapshot, keeping ambient hardware and lab state
  out of the task boundary.
- fixed: the default diagnostic command output is fail-closed:
  entropydiag-fail-closed-no-input, entropydiag-hardware-rng-unaccepted,
  entropydiag-operator-seed-required, cryptographic-strength false, and
  ssh-ready false.
- fixed: deterministic controls are repeatable under source/unit tests for
  timer-only input, mixed local observations, fixed test seed material, and
  hardware RNG observation rejection.
- fixed: the diagnostic preserves operator-seed-required for local inputs that
  do not include provisioned seed material, avoiding an SSH readiness claim.
- deferred: real entropy collection, conditioning/DRBG selection, key
  generation, seed persistence, authorized-key storage, memory lifetime policy,
  service lifecycle, and SSH wiring remain future tasks.
- rejected: observed hardware RNG presence is not accepted as a hardware
  randomness or cryptographic-strength source by this implementation.
- rejected: the entropy diagnostic command is not a public ABI/POSIX/Linux
  compatibility promise and does not accept SSH readiness.
- removed: no source behavior, dependency, hardware/lab helper, task evidence,
  or documentation path was removed.
- not-an-issue: adding a no-input entropy diagnostic command is within the
  existing internal diagnostic command channel; it reports the fail-closed
  baseline only.

## Implementation Boundary

EntropyDiagnosticSnapshot contains only optional caller-supplied observations:
timer samples, scheduler-event samples, console-timing samples,
operator-provisioned seed metadata, deterministic-control state, and a rejected
hardware-RNG-observed flag. classify_entropy_snapshot returns labels and booleans
only. It does not read counters, sample devices, produce random bytes, derive
keys, persist seeds, expose a public ABI, or claim SSH readiness.

The strongest accepted result is the source/unit classification boundary:
Talos can distinguish no-input, deterministic-control, untrusted timer-only,
untrusted local mix, operator-seed-required, and hardware-RNG-unaccepted states.

## Evidence

- Source implementation:
  - src/entropy.rs.
  - src/diagnostic_command.rs.
  - src/main.rs.
- Predecessor contract:
  tasks/2026-06-21-phase12-entropy-source-contract.md.
- Docs:
  docs/src/project/phase12-networking-ssh.md and docs/src/roadmap.md.

## Validation

- cargo fmt --all -- --check: pass.
- focused cargo -Zjson-target-spec test entropy --quiet: pass.
- cargo -Zjson-target-spec test --quiet without QEMU path: failed because
  qemu-system-aarch64 was not on PATH.
- cargo -Zjson-target-spec test --quiet with
  /opt/strider/openclaw/current/workspace/tools/qemu-9.2.0-install/bin on PATH:
  pass, 706 talos no_std tests.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Acceptance Check

- Implementation stays within the accepted entropy source contract and records
  findings with disposition: satisfied.
- Focused tests prove deterministic controls and fail-closed behavior:
  satisfied.
- selected_next_task=phase12-entropy-ssh-strategy-closeout-20260621:
  satisfied.
- No SSH readiness, cryptographic-strength, hardware randomness, live packet
  I/O, reachability, broad expansion, or phase-transition claim is made:
  satisfied.

## Next Action

Promote phase12-entropy-ssh-strategy-closeout-20260621 on the next worker wake
if dependencies remain satisfied. The closeout must reconcile strategy,
contract, implementation evidence, docs, deferred work, and retained risks; it
must not start host key generation, SSH service work, hardware/lab action, live
packet I/O, broad expansion, or a phase transition.
