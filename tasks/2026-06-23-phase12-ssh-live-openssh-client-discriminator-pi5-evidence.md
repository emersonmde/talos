# Phase 12.6 SSH live OpenSSH client discriminator Pi 5 evidence

Task id: phase12-ssh-live-openssh-client-discriminator-pi5-evidence-20260623
Status: accepted
Owner: worker
Classification: phase12-ssh-live-openssh-client-discriminator-blocked-openssh-unavailable

## Goal

Run the first hardware-serialized live OpenSSH client discriminator against a
selected Talos Pi 5 candidate, retaining only sanitized public evidence and
failing closed if the discriminator cannot reach a valid public classification.

## Reviewed Inputs

- memory/talos-supervisor-state.json task
  phase12-ssh-live-openssh-client-discriminator-pi5-evidence-20260623.
- tasks/2026-06-23-phase12-ssh-live-openssh-client-contract.md.
- docs/src/project/lab-controller.md.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.
- src/ssh_service_readiness.rs.
- src/network.rs.
- src/userspace_socket_abi.rs.

## Execution Summary

The worker acquired hardwareTestLock before lab, boot publication, power, and
client-discriminator action. It built the current Talos Pi 5 candidate, staged a
serial-prefixed boot archive, published it to the lab TFTP root, power-cycled
the Pi 5, and retained sanitized identity, cursor, TFTP, serial, and restore
evidence under tasks/evidence/2026-06-23-phase12-ssh-live-openssh-client-discriminator-pi5-evidence/run1/.

The candidate publication selected kernel_2712.img with boot tree hash
fe9a0d98aae7e38310a18adf7902d59346cbdef943250f16c948eae6a3f64333, archive
hash a74e1e2f3c53752f1b47cd5074418124b56724412e707d3e415edfd373ae8ebc, and
kernel size 87432 bytes. The initial candidate run, known-good restored control,
and unchanged candidate rerun all produced stable zero-event TFTP deltas from
their saved cursors and no new sanitized serial bytes from the saved saturated
serial cursor. This means the selected Talos candidate was not proved to have
been fetched or exercised.

The bounded host-client discriminator then failed closed before launch because
the runner does not provide an ssh executable. The retained public
classification is openssh-unavailable. No raw OpenSSH output, serial text, TFTP
raw lines, user name, address, host key, authorized key, fingerprint, signature,
session identifier, channel identifier, payload byte, packet capture, or stable
live peer identifier is retained.

The worker restored the pre-run boot snapshot
phase12-ssh-live-openssh-pre-20260623T143823Z. Final sanitized lab status
returned to boot tree hash
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 with
kernel_2712.img selected.

## Evidence

- hardwareTestLock: acquired before lab/hardware/OpenSSH action and released
  after final restore in supervisor state.
- static boot artifact review:
  tasks/evidence/2026-06-23-phase12-ssh-live-openssh-client-discriminator-pi5-evidence/run1/archive-review.txt.
- candidate identity:
  run1/publish.sanitized.json, run1/post-publish-status.sanitized.json,
  run1/archive-kernel-sha256.txt, and run1/archive-kernel-sizes.txt.
- fresh cursors and deltas:
  run1/pre-serial-peek.sanitized.json, run1/pre-tftp-tail.sanitized.json,
  and run1/tftp-delta.sanitized.json.
- known-good control:
  run1/control-restore.sanitized.json,
  run1/control-tftp-delta.sanitized.json, and
  run1/control-serial-observe.sanitized.json.
- unchanged candidate rerun:
  run1/rerun-publish.sanitized.json,
  run1/rerun-tftp-delta.sanitized.json, and
  run1/rerun-serial-observe.sanitized.json.
- sanitized OpenSSH invocation/classification:
  run1/openssh-attempt.sanitized.json.
- restore proof:
  run1/final-restore.sanitized.json and run1/final-status.sanitized.json.

## Findings And Disposition

- fixed: promoted exactly one ready task and acquired hardwareTestLock before
  any lab/hardware/client action.
- fixed: built and reviewed a task-owned Pi 5 boot archive with mirrored
  da591740/ kernel files before publication.
- fixed: retained candidate identity, boot publication, fresh cursor, stable
  same-cursor TFTP, serial observation, control, unchanged-rerun, and final
  restore evidence.
- deferred: no live OpenSSH discriminator observation is accepted because the
  task runner has no ssh executable and the Pi 5 run also lacked a candidate
  TFTP fetch/serial progression.
- deferred: live-reachability=true, remote-receipt=true, compatibility=true,
  PTY/SCP/SFTP, broad command expansion, phase transition, and ssh-ready=true
  remain rejected.
- not-an-issue: no Rust source change was required after the inconclusive
  hardware evidence because the fixed fail-closed blocker is outside Talos
  runtime code for this task.

## Validation

- static boot artifact review: pass.
- serialized Pi 5 hardware discriminator evidence: fail-closed blocker,
  openssh-unavailable; candidate/control/rerun TFTP deltas were stable with
  zero events, so no candidate-fetch claim is accepted.
- restore proof: pass; final status returned to the pre-run restored tree hash.
- cargo fmt --all -- --check: conditional skip, no Rust source, tests, scripts
  that generate Rust artifacts, or Cargo metadata touched.
- cargo -Zjson-target-spec test ssh_openssh_compat_discriminator --quiet:
  conditional skip, no Rust source or tests touched.
- cargo -Zjson-target-spec test ssh_peer_output_receipt --quiet: conditional
  skip, no Rust source or tests touched.
- cargo -Zjson-target-spec test --quiet: conditional skip, no Rust source,
  tests, or Cargo metadata touched.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.

Evidence levels: static boot artifact review, lab-controller API, serialized Pi
5 hardware power/restore/TFTP/serial evidence, sanitized host-client
availability check, docs build, and diff checks.

## Redaction Review

Pass. Retained evidence contains task ids, file paths, public boot tree hashes,
public archive/kernel hashes and sizes, boot configuration keys, cursor numbers,
TFTP event status/filename/byte categories, serial byte counts and fixed marker
booleans, OpenSSH invocation class, fixed failure label, validation commands,
and classifications. It retains no raw OpenSSH output, raw serial text, raw
TFTP log lines, user names, IP addresses, MAC addresses, host keys, authorized
keys, fingerprints, signatures, session identifiers, channel identifiers,
payload bytes, packet captures, hardware serial text, or private user data.

## Acceptance

Accepted only as fail-closed blocker evidence:
phase12-ssh-live-openssh-client-discriminator-blocked-openssh-unavailable.

selected_next_task=null.
planningNeeded=true.
planningReason=The live client discriminator cannot proceed in this runner
because OpenSSH is unavailable, and the same hardware-serialized attempt also
did not prove selected-candidate TFTP fetch or serial progression across
candidate, known-good control, or unchanged candidate rerun. Supervisor planning
must decide whether to provision an OpenSSH-capable runner/tool path, repair the
lab boot-capture precondition, or queue a narrower discriminator before any
remote-receipt contract or compatibility claim.
