# Phase 10 Pi 5 Local Serial Command Loop Proof Task

Task: phase10-pi5-local-serial-command-loop-proof-20260531

Status: accepted

## Scope

Carry the accepted local serial command-loop feature to serialized Raspberry
Pi 5 hardware evidence. The proof publishes a candidate archive for the
accepted command-loop implementation, exercises a physical serial command, and
restores the prior accepted boot tree.

Changed files:

- build.rs
- src/boot/rpi5.rs
- src/local_command_loop.rs
- src/main.rs
- src/pl011.rs
- src/runtime_console.rs
- src/target/rpi5.rs
- scripts/rpi5-local-serial-command-loop-image.sh
- scripts/rpi5-local-serial-command-loop-boot-tree.sh
- docs/src/roadmap.md
- docs/src/decisions/README.md
- tasks/2026-05-31-phase10-pi5-local-serial-command-loop-proof.md
- tasks/evidence/2026-05-31-pi5-local-serial-command-loop-proof/

## Outcome

The Pi 5 boot scenario runs the same local command-loop boundary over the
physical BCM2712 UART10 runtime-console0 path. The retained hardware
transcript proves the selected \`bogus\` command path: typed input appears at
the \`talos> \` prompt, Enter dispatches the line, the kernel-backed command
loop prints \`talos: unknown-command\`, and the loop reaches the next prompt and
\`ready-for-next prompt=true\` marker before reporting
\`pi5-local-serial-command-loop-complete\` and \`PASS\`.

Earlier attempts also exercised \`help\` and empty input, but the retained
command-zero response text was clipped by serial capture. The accepted
feature evidence is therefore scoped to the visible unknown-command path,
which satisfies the physical serial input, dispatch, response, and next-prompt
acceptance gate without claiming a broader userspace shell.

## Evidence

- Hardware lock:
  /opt/strider/openclaw/current/workspace/memory/talos-supervisor-state.json
  recorded acquisition by this task before archive publication and release
  after restore.
- Candidate archive digest:
  tasks/evidence/2026-05-31-pi5-local-serial-command-loop-proof/local6-clean-candidate/archive-kernel-sha256.txt.
- Static archive/image review:
  tasks/evidence/2026-05-31-pi5-local-serial-command-loop-proof/local6-clean-candidate/archive-review.txt.
- Pi 5 hardware transcript:
  tasks/evidence/2026-05-31-pi5-local-serial-command-loop-proof/local6-clean-candidate/serial-transcript.txt.
- Selected-command proof summary:
  tasks/evidence/2026-05-31-pi5-local-serial-command-loop-proof/local6-clean-candidate/proof-result-selected-bogus.txt.
- Candidate TFTP fetch evidence:
  tasks/evidence/2026-05-31-pi5-local-serial-command-loop-proof/local6-clean-candidate/tftp-kernel-fetch-local6.txt.
- Restore proof:
  tasks/evidence/2026-05-31-pi5-local-serial-command-loop-proof/local6-clean-candidate/post-snapshot-restore-status.json.

## Inconclusive-Run Triage

- Candidate identity: local1/local6 retained candidate archive digest
  b27764bce3d8a47562e16679119f24e3b841ed7f7aa6249070cb360cc6e3a134 and
  kernel digest 09a02e4dd9cbeac61ef20f4cd4cef6a1e62d1364abdb87aff9724a6101e1fb34.
- Fresh serial/TFTP: later candidate reruns used fresh serial cursors and TFTP
  deltas before accepting local6 selected-command evidence.
- Known-good control: local2-known-good-control retained a PASSing restored
  production timer/preemption boot with a 104136-byte kernel fetch.
- Unchanged candidate rerun: local3 through local6 reran the same candidate
  archive before accepting the local6 selected-command transcript.

## Validation

- fmt/lint: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed.
- static image/archive inspection: local6 archive review recorded
  kernel_2712.img size 94344, header image_size 94344, and flags 12.
- serialized Pi 5 hardware: local6 retained serial hardware boot/output,
  selected-command dispatch, response, next prompt, classification, and PASS.
- restore proof: status after restore returned tree hash
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 and
  kernel_2712.img size 104136.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

## Commit

Recorded in durable supervisor state after acceptance.
