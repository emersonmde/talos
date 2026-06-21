# Phase 12 SSH Key Management Readiness Contract

Task id: phase12-ssh-key-management-readiness-contract-20260621

Status: accepted

Classification:
phase12-ssh-key-management-readiness-contract-accepted

Evidence level: static source/task/docs/evidence review, focused entropy
source/unit test, docs build, and diff checks. No key generation, secret
material persistence, crypto dependency adoption, SSH service acceptance,
hardware/lab action, live packet I/O, hardware reachability, public
ABI/POSIX/Linux compatibility claim, broad expansion, or phase transition was
performed.

## Goal

Define the smallest Talos-owned SSH key-management readiness contract that can
follow the accepted fail-closed entropy diagnostic without generating keys,
persisting secrets, importing crypto/SSH dependencies, or accepting SSH service
behavior.

## Scope Performed

- Reconciled the accepted prerequisite-first SSH strategy, entropy source
  contract, entropy diagnostic core, and entropy/SSH strategy closeout.
- Reviewed current source for existing entropy, diagnostic-command, crypto, SSH,
  key, authorized-key, persistence, and exposure-control surfaces.
- Fixed a narrow entropy diagnostic test assertion so the no-input case checks
  the hardware-RNG rejection label through hardware_rng_label() while preserving
  the fail-closed input label.
- Defined exact key-readiness states and fail-closed diagnostic labels for host
  key, authorized key, entropy/seed, persistence, and exposure prerequisites.
- Selected phase12-sshkeydiag-core-20260621 as the next bounded implementation
  task.

## Findings

- fixed: the readiness boundary is a classifier/diagnostic contract over
  explicit prerequisite states, not host-key generation, key import, seed
  persistence, crypto dependency adoption, or SSH service startup.
- fixed: the accepted entropy diagnostic baseline remains fail-closed:
  entropydiag-fail-closed-no-input, entropydiag-hardware-rng-unaccepted,
  entropydiag-operator-seed-required, cryptographic-strength false, and
  ssh-ready false.
- fixed: src/entropy.rs now asserts the hardware RNG rejection label via
  report.hardware_rng_label() in the no-input test, matching the accepted report
  shape without changing runtime classifier behavior.
- fixed: exact readiness labels are named for the next diagnostic:
  sshkeydiag-missing-host-key, sshkeydiag-missing-authorized-key,
  sshkeydiag-entropy-unready, sshkeydiag-seed-material-missing,
  sshkeydiag-seed-material-insufficient, sshkeydiag-persistence-unavailable,
  sshkeydiag-exposure-disabled, and sshkeydiag-not-ready.
- deferred: host-key generation versus provisioning, host-key format,
  authorized-key parsing/storage, seed-file format, secret zeroization,
  crypto/DRBG selection, service lifecycle, authentication policy, time policy,
  heap-pressure limits, and SSH server implementation remain future work.
- rejected: treating local timer/scheduler/console observations as sufficient
  SSH entropy, treating an absent seed as acceptable, enabling exposure by
  default, accepting persistence through generated-root/initramfs state, or
  using the diagnostic as a public ABI/POSIX/Linux compatibility surface.
- rejected: live packet I/O, hardware reachability, SSH service readiness,
  broad socket expansion, and phase transition remain outside this contract.
- removed: no source behavior, dependency, hardware/lab helper, task evidence,
  or documentation path was removed.
- not-an-issue: selecting a fail-closed key-readiness diagnostic next is within
  the accepted Phase 12.5 prerequisite-first SSH strategy; it exposes missing
  key-management responsibility before any crypto or SSH stack can hide it.

## Contract

The next implementation task may add a Talos-owned SSH key-readiness classifier
and internal diagnostic surface. The classifier must consume an explicit
snapshot of prerequisite states supplied by tests or a caller. It must not read
or persist secret material, generate keys, derive keys, import crypto/SSH
dependencies, inspect ambient hardware/lab state, expose a public ABI, or start
an SSH service.

The accepted readiness dimensions and fail-closed labels are:

- host key: missing host key reports sshkeydiag-missing-host-key;
- authorized key: missing authorized key source reports
  sshkeydiag-missing-authorized-key;
- entropy: any accepted entropy report with cryptographic-strength false,
  ssh-ready false, entropydiag-fail-closed-no-input,
  entropydiag-deterministic-control, entropydiag-untrusted-timer-only,
  entropydiag-untrusted-local-mix, entropydiag-operator-seed-required, or
  entropydiag-hardware-rng-unaccepted reports sshkeydiag-entropy-unready;
- seed material: absent seed metadata reports sshkeydiag-seed-material-missing,
  and explicitly too-small/provisional seed metadata reports
  sshkeydiag-seed-material-insufficient;
- persistence: no accepted writable/persistent key or seed store reports
  sshkeydiag-persistence-unavailable;
- exposure: the default disabled service exposure state reports
  sshkeydiag-exposure-disabled.

The aggregate default result is sshkeydiag-not-ready. A future ready label is
not accepted by this contract; the next implementation should prove stable
negative controls first. If a future task wants to accept any ready state, it
must separately prove cryptographic entropy, seed persistence or provisioning,
host-key and authorized-key policy, exposure policy, and service lifecycle.

## Deterministic Controls For Next Task

The next implementation task must include focused deterministic tests proving:

- the all-missing default reports sshkeydiag-not-ready plus each missing
  prerequisite label;
- no-input and deterministic-control entropy reports keep SSH key readiness
  false;
- untrusted local entropy input without seed material keeps SSH key readiness
  false;
- absent seed and insufficient seed material are distinguished;
- persistence unavailable and exposure disabled each independently keep
  readiness false;
- a negative control with host/authorized key metadata present still remains not
  ready when entropy, persistence, or exposure is unavailable.

The tests must use fixed metadata only. They must not include real private keys,
authorized public keys tied to an operator, random bytes, lab state, or hardware
observations.

## Evidence

- Strategy predecessor:
  tasks/2026-06-21-phase12-entropy-ssh-strategy-contract.md.
- Entropy source contract:
  tasks/2026-06-21-phase12-entropy-source-contract.md.
- Entropy diagnostic implementation:
  tasks/2026-06-21-phase12-entropydiag-core.md.
- Entropy/SSH strategy closeout:
  tasks/2026-06-21-phase12-entropy-ssh-strategy-closeout.md.
- Source review: src/entropy.rs, src/diagnostic_command.rs, Cargo.toml, and
  repository-wide search for entropy/random/crypto/SSH/key/persistence/exposure
  terms.
- Docs: docs/src/project/phase12-networking-ssh.md and docs/src/roadmap.md.

## Validation

- static source/task/docs/evidence review: pass.
- focused cargo -Zjson-target-spec test entropy --quiet before test fix: failed
  at src/entropy.rs no_input_fails_closed_without_crypto_or_ssh_readiness
  because the test compared input_label() to HardwareRngUnaccepted.
- focused cargo -Zjson-target-spec test entropy --quiet after test fix: pass.
- cargo fmt --all -- --check: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Acceptance Check

- Contract records findings with disposition and reconciles the accepted entropy
  diagnostic boundary: satisfied.
- Contract names exact readiness states and fail-closed labels for missing host
  key, missing authorized key, missing/insufficient entropy or seed material,
  persistence unavailable, and exposure disabled: satisfied.
- selected_next_task=phase12-sshkeydiag-core-20260621: satisfied.
- No key generation, secret material persistence, crypto dependency adoption,
  SSH service acceptance, hardware/lab action, live packet I/O, public
  ABI/POSIX/Linux compatibility, broad expansion, or phase-transition claim is
  accepted: satisfied.

## Next Action

Promote phase12-sshkeydiag-core-20260621 on the next worker wake if
dependencies remain satisfied. The next task must implement only the accepted
classifier/diagnostic negative-control boundary and focused tests; it must not
generate keys, persist secrets, adopt crypto or SSH dependencies, expose SSH,
touch hardware/lab state, or accept SSH readiness.
