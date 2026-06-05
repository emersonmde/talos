# Phase 11 RP1 Register Read Pi 5 Proof

Task: phase11-rp1-register-read-pi5-proof-20260605

Status: completed with hardware blocker, mapping not accepted.

## Goal

Run the serialized Pi 5 proof for the accepted RP1 UART0 flag-register read
diagnostic and retain decisive hardware evidence or a blocker without expanding
into GPIO ownership, interrupts, DMA/cache policy, networking, SSH, storage, or
generated-root work.

## Outcome

The worker acquired the hardware lock, published only the accepted
`rpi5-rp1-uart0-fr-read` candidate archive, captured fresh serial and TFTP
cursors, captured candidate TFTP evidence before restore, ran a known-good
control, reran the candidate after that control, and restored the prior boot
tree after each candidate publication.

The proof does not accept the RP1 mapping contract. Both decisive candidate
runs fetched the selected 87,392-byte kernel from TFTP, but the diagnostic did
not reach its serial output. The known-good control booted the restored
104,136-byte accepted tree and retained `TALOS: kernel_main` plus accepted
command-loop output, so serial capture and the restored boot tree were viable.
The retained result is therefore a hardware blocker at
`blocked-pre-entry-or-handoff-after-candidate-fetch`, not
`mapped/read-value`.

## Evidence

Evidence directory:

`tasks/evidence/2026-06-05-phase11-rp1-register-read-pi5-proof/`

Primary files:

- `proof-summary.txt`
- `post-hardware-review.txt`
- `local1-candidate/candidate-identity.txt`
- `local2-candidate-rerun/tftp-delta-candidate-before-restore.json`
- `local2-candidate-rerun/serial-observe-candidate.json`
- `local2-candidate-rerun/restore.json`
- `local2a-known-good-control/tftp-delta-control.json`
- `local2a-known-good-control/serial-tail-control.json`
- `local3-candidate-after-control/tftp-delta-candidate-before-restore.json`
- `local3-candidate-after-control/serial-observe-candidate.json`
- `local3-candidate-after-control/restore.json`

Candidate identity:

- archive: `target/talos-rpi5-rp1-uart0-fr-read-proof-20260605.tar.gz`
- archive SHA-256:
  `937d749b4fe2ef40a5ee730461ebae7108edad437b3b216856d7b549b5129e0a`
- kernel SHA-256:
  `bed60fc8babf5c91117dd1ccb7c9a105af2bcd30cfedcc098a414029b46fe3c5`
- kernel size: `87392`

Boot tree hashes:

- pre-run/restored accepted tree:
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`
- published candidate tree:
  `a96f0d8dc17a4872cb52e94c37c85d5adc5312255d26f988dbd8b71e1b6118c9`

## Findings

- fixed: local2 and local3 captured fresh candidate identity, serial cursor,
  TFTP cursor, pre/post status, pre-restore TFTP deltas, and restore evidence.
- fixed: local2 and local3 both proved the Pi fetched the selected
  `da591740/kernel_2712.img` at 87,392 bytes while the candidate tree was still
  published.
- fixed: the known-good control booted the restored accepted tree and retained
  `TALOS: kernel_main` plus accepted command-loop output, showing that serial
  capture and the restored boot tree remained viable.
- deferred: source-level root cause for the candidate pre-entry/handoff stop is
  outside this proof task and needs supervisor planning; do not churn RP1
  constants or broaden into GPIO/interrupt/DMA/networking work from this task.
- not-an-issue: candidate archive publication and TFTP placement worked.

## Validation

- Lab-controller API: `/status`, boot snapshot, archive publish, power cycle,
  TFTP logs, serial observe/tail, and restore artifacts retained.
- TFTP hardware evidence: local2 and local3 captured pre-restore TFTP deltas
  with `da591740/kernel_2712.img` served at 87,392 bytes.
- Serial hardware boot/output: local2 observed only NUL/LF after the fresh
  candidate cursor; local3 observed zero bytes after the fresh candidate cursor.
  No `rpi5-rp1-uart0-fr-read`, `mapped/read-value`, or `PASS` line was
  retained for either candidate.
- Known-good control: retained serial tail includes `TALOS: kernel_main` and
  accepted command-loop output from the restored accepted boot tree.
- Restore proof: local2 and local3 restored the prior boot tree hash
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.
- diff hygiene: `git diff --check` passed in local2 and local3 evidence.

## Next Action

Close out Milestone 11.1 with this blocked hardware proof. The worker should
not infer a new RP1 direction from this result; supervisor planning is needed
for any source-level pre-entry/handoff investigation or revised diagnostic
shape.
