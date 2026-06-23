# Phase 12.6 SSH live OpenSSH client discriminator retry

Task id: phase12-ssh-live-openssh-client-discriminator-retry-20260623
Status: accepted
Owner: worker
Classification: phase12-ssh-live-openssh-client-discriminator-blocked-lab-capture-regressed

## Goal

Retry the bounded live OpenSSH client discriminator after accepted
workspace-local OpenSSH client provisioning and accepted lab boot-capture
freshness preflight evidence.

## Reviewed Inputs

- memory/talos-supervisor-state.json task
  phase12-ssh-live-openssh-client-discriminator-retry-20260623.
- tasks/2026-06-23-phase12-ssh-runner-openssh-client-provisioning-preflight.md.
- tasks/2026-06-23-phase12-ssh-lab-boot-capture-preflight.md.
- tasks/2026-06-23-phase12-ssh-live-openssh-client-contract.md.
- tasks/2026-06-23-phase12-ssh-live-openssh-client-discriminator-pi5-evidence.md.
- docs/src/project/lab-controller.md.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Execution Summary

The worker promoted exactly one queued task and acquired hardwareTestLock before
boot publication, Pi 5 power/TFTP/serial observation, and the live OpenSSH
client action. It built the current Talos Pi 5 candidate, staged a
serial-prefixed boot archive, published it to the lab TFTP root, power-cycled
the Pi 5 once, captured sanitized serial/TFTP/status evidence, ran one bounded
workspace-local OpenSSH client attempt, restored the pre-run snapshot, and
released the lock.

The candidate publication selected kernel_2712.img with boot tree hash
fe9a0d98aae7e38310a18adf7902d59346cbdef943250f16c948eae6a3f64333,
kernel hash
110e66ef6867a70cc8b72f52c0786a8f4037796b4058ee4a27ba1371ba8c12d5, and
kernel size 87,432 bytes. After the power cycle, the lab API still reported the
published candidate tree, but the TFTP delta from the saved cursor contained
zero events and the serial cursor remained saturated with zero fresh bytes.
Therefore the task did not prove that the selected candidate was fetched or
exercised.

The OpenSSH client did launch from the accepted workspace-local tool path and
failed closed as no-tcp-connect with public exit category tcp-timeout.
Because the candidate fetch was not proven, the accepted blocker is
lab-capture-regressed; the TCP timeout is retained as secondary public client
evidence only. The worker restored the pre-run boot tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Evidence

- sanitized summary:
  tasks/evidence/2026-06-23-phase12-ssh-live-openssh-client-discriminator-retry/live-openssh-discriminator.summary.sanitized.json.
- static boot artifact review:
  tasks/evidence/2026-06-23-phase12-ssh-live-openssh-client-discriminator-retry/archive-review.txt.
- candidate identity:
  tasks/evidence/2026-06-23-phase12-ssh-live-openssh-client-discriminator-retry/candidate-identity.sanitized.json,
  publish.sanitized.json, post-publish-status.sanitized.json,
  archive-sha256.txt, and archive-sizes.txt.
- fresh cursor and lab capture evidence:
  pre-serial-peek.sanitized.json, pre-tftp-tail.sanitized.json,
  serial-observe.sanitized.json, and tftp-delta.sanitized.json.
- sanitized OpenSSH invocation/classification:
  openssh-attempt.sanitized.json.
- restore proof:
  final-restore.sanitized.json and final-status.sanitized.json.

## Findings And Disposition

- fixed: promoted exactly one queued task and acquired hardwareTestLock before
  lab/hardware/OpenSSH action.
- fixed: used the accepted workspace-local OpenSSH-compatible client path and
  launched exactly one bounded OpenSSH attempt.
- fixed: retained sanitized candidate identity, boot publication, fresh serial
  cursor status, TFTP delta, OpenSSH public phase classification, restore
  evidence, and redaction review.
- deferred: live OpenSSH discriminator observation is not accepted because the
  task did not prove a selected-candidate TFTP fetch or serial progression.
- deferred: live-reachability=true, remote-receipt=true, compatibility=true,
  PTY/SCP/SFTP, broad command expansion, phase transition, and ssh-ready=true
  remain rejected.
- not-an-issue: no Rust source change was made after this fail-closed result;
  the blocker is lab-capture evidence regression rather than a narrow runtime
  code defect established by the run.

## Validation

- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test ssh_openssh_compat_discriminator --quiet:
  pass.
- cargo -Zjson-target-spec test ssh_peer_output_receipt --quiet: pass.
- cargo -Zjson-target-spec test --quiet: pass.
- static boot artifact review: pass.
- serialized Pi 5 hardware discriminator evidence: fail-closed blocker,
  lab-capture-regressed; candidate TFTP delta had zero events and no fresh
  serial bytes from the saturated cursor.
- restore proof: pass; final status returned to the pre-run restored tree hash.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.

Evidence levels: fmt/lint/typecheck, unit tests, static boot artifact
inspection, lab-controller API, serialized Pi 5 hardware power/restore/TFTP/
serial evidence, sanitized live OpenSSH client attempt, JSON syntax check, docs
build, and diff checks.

## Redaction Review

Pass. Durable evidence retains only task ids, file paths, public boot tree
hashes, public archive/kernel hashes and sizes, boot configuration keys, cursor
numbers, TFTP event status/filename/byte categories, serial byte counts and
fixed marker booleans, OpenSSH invocation class, public phase/exit categories,
validation commands, and classifications. It retains no raw OpenSSH output, raw
serial text, raw TFTP log lines, user names, addresses, MAC addresses, host
keys, authorized keys, fingerprints, signatures, session identifiers, channel
identifiers, payload bytes, packet captures, hardware serial text, or private
user data.

## Acceptance

Accepted only as fail-closed blocker evidence:
phase12-ssh-live-openssh-client-discriminator-blocked-lab-capture-regressed.

selected_next_task=null.
planningNeeded=true.
planningReason=The live OpenSSH discriminator retry did not prove that the
selected Talos candidate was fetched or exercised: TFTP delta from the saved
cursor had zero events and serial observation from the saturated cursor had
zero fresh bytes. The bounded workspace-local OpenSSH attempt launched and
timed out at TCP connect, but no remote-receipt contract, compatibility claim,
phase transition, or ssh-ready frontier is mechanically unblocked until
supervisor planning repairs or replaces the lab-capture proof.
