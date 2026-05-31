# Phase 10 Pi 5 Local Command Stdio Bridge Proof Task

Task: phase10-pi5-local-command-stdio-bridge-proof-20260531

Status: accepted

## Scope

Carry the accepted descriptor-backed local stdio command feature to serialized
Raspberry Pi 5 hardware evidence. The proof keeps the feature bounded to the
kernel-backed local command loop: type stdio at the talos> prompt, dispatch the
command through inherited stdio descriptors, print the visible response, and
restore the previous accepted boot tree.

Changed files:

- src/target/rpi5.rs
- scripts/rpi5-local-command-stdio-bridge-image.sh
- scripts/rpi5-local-command-stdio-bridge-boot-tree.sh
- tasks/2026-05-31-phase10-pi5-local-command-stdio-bridge-proof.md
- tasks/evidence/2026-05-31-pi5-local-command-stdio-bridge-proof/

## Outcome

The Pi 5 proof now emits a hardware-visible stdio response after the
descriptor-backed command dispatch. The retained accepted transcript is
local11-fresh-tftp-visible-response-candidate/serial-transcript.txt; it shows
the physical prompt, typed stdio, handled dispatch with six response lines,
visible fd 0/fd 1/fd 2 stdio identity, runtime-console0, the
descriptor-backed-output=true marker, ready-for-next prompt, final
pi5-local-command-stdio-bridge-complete classification, and PASS.

The accepted candidate identity is shared by local10 and local11:

- archive sha256: 0885f021f34ab1398f91fa8206d587a40295663570ce8c505daa3b21ac8c2f02
- kernel sha256: 45934e74174388e3346cf76f63af3568abf260526fd749a707b2a67568191899
- kernel size: 97472 bytes

## Inconclusive-Run Triage

- local1-candidate: reached stdio dispatch/classification/PASS, but did not
  retain serial-visible talos: ok stdio response lines.
- local2-known-good-control: restored control passed with the prior accepted
  production timer/preemption boot.
- local3-unchanged-candidate-rerun: repeated the original candidate and remained
  inconclusive for visible response lines.
- local4-fixed-backend-candidate: added a proof-local ConsoleBackend; the run
  did not produce accepted visible response evidence before collection ended.
- local5-known-good-control-after-local4: fresh restored control passed.
- local6/local7/local8: bounded proof-output iterations narrowed the issue to
  the retained serial response window; local8 made later response markers
  visible, but not the full response.
- local9/local11: after the final proof-local replay, the visible stdio
  response, fd identities, runtime-console marker, descriptor-backed marker,
  ready prompt, classification, and PASS were retained.
- local10: same candidate identity as local11 and retained fresh TFTP evidence
  for a 97472-byte candidate kernel fetch, but missed the prompt write window and
  ended with input timeout. It is retained as same-candidate TFTP evidence, not
  as the accepted serial transcript.

## Evidence

- Hardware lock:
  /opt/strider/openclaw/current/workspace/memory/talos-supervisor-state.json
  recorded acquisition before hardware action and release after restore.
- Accepted serial transcript:
  tasks/evidence/2026-05-31-pi5-local-command-stdio-bridge-proof/local11-fresh-tftp-visible-response-candidate/serial-transcript.txt.
- Accepted proof summary:
  tasks/evidence/2026-05-31-pi5-local-command-stdio-bridge-proof/local11-fresh-tftp-visible-response-candidate/proof-result-stdio.txt.
- Same-candidate fresh TFTP proof:
  tasks/evidence/2026-05-31-pi5-local-command-stdio-bridge-proof/local10-fresh-tftp-visible-response-candidate/tftp-kernel-fetch-local10.txt.
- Accepted candidate archive/image review:
  tasks/evidence/2026-05-31-pi5-local-command-stdio-bridge-proof/local11-fresh-tftp-visible-response-candidate/archive-review.txt.
- Restore proof:
  tasks/evidence/2026-05-31-pi5-local-command-stdio-bridge-proof/local11-fresh-tftp-visible-response-candidate/post-snapshot-restore-status.json.

## Validation

- fmt/lint: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed.
- static archive/image inspection: accepted candidate archive review passed with
  kernel size 97472, header image size 97472, and flags 12.
- serialized Pi 5 hardware: local11 retained visible stdio response, fd
  identities, runtime-console0, descriptor-backed marker, next prompt,
  classification, and PASS; local10 retained same-candidate 97472-byte TFTP
  fetch evidence.
- restore proof: local11 restore returned tree hash
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- static inspection: git diff --check passed.
- documentation: mdbook build was not run because docs/src was not touched.
- staged whitespace inspection: git diff --cached --check passed before commit.

## Commit

Implementation and evidence commit: 115a347.
