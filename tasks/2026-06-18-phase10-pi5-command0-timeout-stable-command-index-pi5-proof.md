# Phase 10 Pi 5 Command0 Timeout-Stable Command Index Pi 5 Proof

Task id: phase10-pi5-command0-timeout-stable-command-index-pi5-proof-20260618

Status: accepted

Classification:
command0-timeout-stable-command-index-pi5-proof-accepted

Evidence level: serialized Pi 5 hardware proof under hardwareTestLock, lab
controller API identity/status evidence, TFTP delta evidence, direct serial
hardware output, baseline restore proof, task-owned JSON evidence, docs build,
and diff checks.

## Goal

Run the serialized Pi 5 proof that command0 remains pending across empty
timeout/readiness churn and is delivered only after the lab writes the command.

## Result

Command0 input delivery is accepted for the timeout-stable command-index proof.
The candidate boot retained selected-kernel identity with the 208984-byte
`kernel_2712.img`, two same-power-cycle
`da591740/kernel_2712.img` TFTP serves at the selected size, firmware
initramfs valid-artifact readiness, and the proof-visible
`timeout-hold command=0 ... pending=true source=timeout-stable-command-index`
marker before the write.

After the hold, the lab `/serial/write` endpoint accepted the 9-byte
`rootinfo\n` payload. Direct serial output retained ordered command0
delivery:

- `line command=0 hex=72 6f 6f 74 69 6e 66 6f`
- `dispatch command=0 status=handled responses=1 raw-bytes=9`
- `ready command=1`

Immediate and final pre-restore status still exposed the selected tree, and
the named pre-run snapshot restore returned the boot tree to the baseline hash
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.

The selected next task is
phase10-pi5-command0-timeout-stable-command-index-closeout-20260618.

## Findings

- fixed: the runtime/source timeout-hold path was proven on Pi 5 hardware; an
  empty no-data timeout emitted a hold marker without advancing command0.
- fixed: command0 stayed pending long enough for the lab write to deliver
  `rootinfo` as command0 and advance exactly to ready command=1.
- fixed: retained selected-kernel/TFTP identity, direct serial delivery, and
  baseline restore proof under hardwareTestLock.
- not-an-issue: selected-kernel/TFTP identity did not regress; the accepted
  proof observed two selected 208984-byte kernel serves.
- deferred: source-response retention and generated-root command-input success
  beyond command0 delivery remain gated behind the follow-up closeout.

## Evidence

- Classification:
  tasks/evidence/2026-06-18-phase10-pi5-command0-timeout-stable-command-index-pi5-proof/candidate-timeout-stable-command-index-20260618T072652Z/classification.json.
- Evidence map:
  tasks/evidence/2026-06-18-phase10-pi5-command0-timeout-stable-command-index-pi5-proof/candidate-timeout-stable-command-index-20260618T072652Z/evidence-map.json.
- Readiness and timeout hold:
  tasks/evidence/2026-06-18-phase10-pi5-command0-timeout-stable-command-index-pi5-proof/candidate-timeout-stable-command-index-20260618T072652Z/serial/readiness-summary.json.
- Ordered command0 delivery:
  tasks/evidence/2026-06-18-phase10-pi5-command0-timeout-stable-command-index-pi5-proof/candidate-timeout-stable-command-index-20260618T072652Z/serial/post-command-summary.json.
- TFTP delta:
  tasks/evidence/2026-06-18-phase10-pi5-command0-timeout-stable-command-index-pi5-proof/candidate-timeout-stable-command-index-20260618T072652Z/tftp/tftp-delta-after-command.json.
- Restore proof:
  tasks/evidence/2026-06-18-phase10-pi5-command0-timeout-stable-command-index-pi5-proof/candidate-timeout-stable-command-index-20260618T072652Z/restore/post-restore-status.json.

## Acceptance Check

- HardwareTestLock serialized and released only after baseline restore proof was
  retained: satisfied.
- Candidate identity, selected TFTP bytes/hash, fresh serial cursor,
  immediate/final selected identity, and restore evidence retained: satisfied.
- Accepted command0 input delivery requires a fresh command0 readiness boundary
  that survives empty timeout/readiness churn, immediate write, command0 line,
  dispatch command=0 status=handled responses=1, and ready command=1 before
  advancement beyond 1: satisfied.
- Inconclusive triage path was not needed because the proof reached a terminal
  accepted classification.
- selected_next_task is
  phase10-pi5-command0-timeout-stable-command-index-closeout-20260618:
  satisfied.

## Validation

- Pi 5 serialized hardware proof under hardwareTestLock: pass, accepted
  classification.
- Candidate identity via lab API status: pass.
- Fresh serial cursor and retained serial evidence: pass.
- TFTP delta evidence for selected kernel_2712.img: pass, two selected
  208984-byte serves.
- Restore proof before hardwareTestLock release: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote
phase10-pi5-command0-timeout-stable-command-index-closeout-20260618 on the
next worker wake if dependencies remain satisfied, hardwareTestLock is
unlocked/restored, supervisorIntervention is inactive, and the repository has
no conflicting uncommitted changes. Do not claim source-response retention,
generated-root command-input success beyond command0 delivery, storage,
networking, SSH, Phase 11/12 expansion, or phase transition from this proof.
