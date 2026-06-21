# Phase 12.5 shell sshkeydiag smoke

Task id: phase12-shell-sshkeydiag-smoke-20260621
Status: accepted
Classification: phase12-shell-sshkeydiag-smoke-accepted

## Goal

Retain host/QEMU-substitute smoke evidence that reaches the accepted
sshkeydiag diagnostic surface and proves the default SSH key-readiness state
fails closed without key generation, secret persistence, crypto/SSH
dependencies, live packet I/O, hardware reachability, public ABI acceptance, or
phase transition.

## Scope

- Add a task-owned smoke script that reaches the accepted diagnostic command
  surface through cargo no_std host/QEMU-substitute tests.
- Retain the smoke transcript under task evidence.
- Preserve the accepted entropy fail-closed boundary when checking SSH
  key-readiness.
- Record findings with disposition.

## Non-goals

- No host key generation, authorized-key parsing, secret seed/key persistence,
  crypto/SSH dependency adoption, SSH service readiness, service lifecycle,
  hardware/lab action, live packet I/O, hardware reachability, public
  ABI/POSIX/Linux compatibility, broad expansion, or phase transition.

## Findings

- fixed: added scripts/qemu-shell-sshkeydiag-smoke.sh as the retained
  host/QEMU-substitute smoke gate for sshkeydiag.
- fixed: retained
  tasks/evidence/2026-06-21-shell-sshkeydiag-smoke/qemu-shell-sshkeydiag-smoke.log
  with the smoke transcript.
- not-an-issue: the smoke uses cargo no_std host/QEMU-substitute tests rather
  than live QEMU serial boot because the accepted diagnostic surface is an
  internal command/classifier boundary and the task explicitly rejects hardware
  and live packet claims.
- deferred: actual host key provisioning, authorized-key storage, seed
  persistence, crypto RNG/DRBG selection, SSH service lifecycle, and exposure
  policy remain future work after readiness closeout and supervisor planning.

## Smoke Evidence

Script:

```text
scripts/qemu-shell-sshkeydiag-smoke.sh
```

Retained transcript:

```text
tasks/evidence/2026-06-21-shell-sshkeydiag-smoke/qemu-shell-sshkeydiag-smoke.log
```

The retained transcript records:

```text
qemu-shell-sshkeydiag-smoke: boundary=internal diagnostic command sshkeydiag over explicit metadata-only fail-closed SSH key-readiness classifier
qemu-shell-sshkeydiag-smoke: expected-labels=sshkeydiag-not-ready,sshkeydiag-missing-host-key,sshkeydiag-missing-authorized-key,sshkeydiag-entropy-unready,sshkeydiag-seed-material-missing,sshkeydiag-persistence-unavailable,sshkeydiag-exposure-disabled,ssh-ready=false
qemu-shell-sshkeydiag-smoke: entropy-boundary=entropydiag-fail-closed-no-input,entropydiag-hardware-rng-unaccepted,entropydiag-operator-seed-required,cryptographic-strength=false,ssh-ready=false
qemu-shell-sshkeydiag-smoke: PASS classification=host-qemu-substitute-shell-sshkeydiag-fail-closed-smoke-complete
```

The smoke gate runs:

- dispatcher_reports_ssh_key_readiness_fail_closed_without_secret_material
- dispatcher_reports_entropy_diagnostic_fail_closed_without_crypto_claim
- all_missing_default_reports_every_fail_closed_label

The underlying no_std test harness reported all tests passing in the retained
transcript.

## Validation

- scripts/qemu-shell-sshkeydiag-smoke.sh: pass.
- cargo -Zjson-target-spec test --quiet: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

## Acceptance

- accepted: retained evidence exercises the accepted key-readiness diagnostic
  path and reports SSH not ready for missing host key, missing authorized key,
  unready entropy, missing seed material, unavailable persistence, and disabled
  exposure.
- accepted: evidence preserves the entropy fail-closed boundary.
- accepted: no SSH service, key generation, crypto dependency, live packet I/O,
  hardware reachability, public ABI/POSIX/Linux compatibility, broad expansion,
  or phase-transition claim is made.

selected_next_task=phase12-ssh-key-management-readiness-closeout-20260621
