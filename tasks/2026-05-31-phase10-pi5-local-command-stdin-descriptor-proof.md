# Phase 10 Pi 5 Local Command Stdin Descriptor Proof Task

Task: phase10-pi5-local-command-stdin-descriptor-proof-20260531

Status: accepted

## Scope

Carry the accepted local command-loop stdin descriptor path to serialized
Raspberry Pi 5 hardware evidence. The proof remains a kernel-backed local
command-loop feature: type stdio at the talos> prompt, read the completed line
through fd0/runtime-console0 descriptor plumbing, dispatch the command, print
the descriptor-backed response, and restore the previous accepted boot tree.

Changed files:

- src/target/rpi5.rs
- tasks/2026-05-31-phase10-pi5-local-command-stdin-descriptor-proof.md
- tasks/evidence/2026-05-31-pi5-local-command-stdin-descriptor-proof/
- docs/src/roadmap.md
- docs/src/decisions/README.md

## Outcome

The accepted Pi 5 transcript is
tasks/evidence/2026-05-31-pi5-local-command-stdin-descriptor-proof/local8-fresh-tftp-candidate-rerun/serial-transcript.txt.
It shows the physical prompt, typed stdio, fd0/runtime-console0 descriptor
input marker, visible talos: ok stdio response, fd 0/fd 1/fd 2 stdio identity
lines, runtime-console0 backing, descriptor-backed output marker,
ready-for-next prompt, final pi5-local-command-stdio-bridge-complete
classification, and PASS.

The accepted candidate identity is:

- archive sha256: acabea1f0a779abca51c1d0b880b43929c2f09e39eed6304bdc7aaf7685cd65f
- kernel sha256: 9466fb78b30029be15107ab8141fa9d0f033072e92c9fee2299d6c79ccda5d92
- kernel size: 97936 bytes

## Implementation Note

The rpi5 proof-local output console now implements core::fmt::Write directly
so the accepted descriptor-backed stdin/stdout adapter can own the proof
console as its fd1/fd2 write backend. This is proof-harness plumbing for the
local command-loop boot scenario; it does not add userspace process stdio,
filesystem command behavior, networking, SSH, or broad POSIX read readiness.

## Inconclusive-Run Triage

- local5: unchanged candidate rerun retained fresh 97920-byte TFTP evidence
  and restore proof, but did not prove descriptor-backed input for the fresh
  candidate.
- local6: fixed proof-console candidate retained fresh 97936-byte TFTP
  evidence and restore proof, but the serial capture did not isolate the
  fd0 descriptor-backed-input marker before acceptance.
- local7: retained the descriptor-backed-input serial marker and PASS, but the
  TFTP log was queried after restore, so it is retained as triage evidence only.
- local8: reran the same 97936-byte candidate with a fresh serial cursor and
  pre-restore TFTP query; this is the accepted transcript and TFTP evidence.

## Evidence

- Accepted serial transcript:
  tasks/evidence/2026-05-31-pi5-local-command-stdin-descriptor-proof/local8-fresh-tftp-candidate-rerun/serial-transcript.txt.
- Accepted proof summary:
  tasks/evidence/2026-05-31-pi5-local-command-stdin-descriptor-proof/local8-fresh-tftp-candidate-rerun/proof-result-stdin-descriptor.txt.
- Fresh pre-restore TFTP proof:
  tasks/evidence/2026-05-31-pi5-local-command-stdin-descriptor-proof/local8-fresh-tftp-candidate-rerun/tftp-kernel-fetch-local8.txt.
- Candidate archive/image review:
  tasks/evidence/2026-05-31-pi5-local-command-stdin-descriptor-proof/local8-fresh-tftp-candidate-rerun/archive-review.txt.
- Restore proof:
  tasks/evidence/2026-05-31-pi5-local-command-stdin-descriptor-proof/local8-fresh-tftp-candidate-rerun/post-snapshot-restore-status.json.
- Hardware lock:
  /opt/strider/openclaw/current/workspace/memory/talos-supervisor-state.json
  recorded acquisition before hardware action and release after restore.

## Validation

- static archive/image inspection: accepted candidate archive review passed
  with kernel size 97936, header image size 97936, and flags 12.
- serialized Pi 5 hardware boot/output: local8 retained the prompt, typed
  stdio, descriptor-backed-input=true, visible stdio response, fd identities,
  runtime-console0, descriptor-backed-output=true, next prompt, classification,
  and PASS.
- lab-controller API/TFTP: local8 retained pre-restore TFTP evidence for two
  97936-byte da591740/kernel_2712.img candidate fetches.
- restore proof: local8 restore returned tree hash
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- fmt/lint: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed with qemu-system-aarch64
  available on PATH.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: pending before commit.

## Commit

Pending.
