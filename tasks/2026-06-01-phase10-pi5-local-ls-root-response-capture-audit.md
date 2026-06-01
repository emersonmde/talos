# Phase 10 Pi 5 Local Ls Root Response Capture Audit

Task: phase10-pi5-local-ls-root-response-capture-audit-20260601
Status: accepted-capture-gap-confirmed

## Goal

Audit the existing local4 Pi 5 `ls /` hardware evidence without mutating the
lab or changing Talos code, and decide whether the visible root-entry response
can be recovered from retained serial logs.

## Scope

This task inspected only retained evidence from
phase10-pi5-local-ls-root-proof-20260601 local4 and read-only serial-log API
windows. It did not acquire hardwareTestLock, publish an archive, power-cycle
the Pi, write to serial, restore boot trees, change Talos runtime/proof code,
or relax the visible-entry acceptance gate.

## Evidence

Existing local4 source evidence:

- tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local4-unchanged-candidate-rerun/proof-result.txt
- tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local4-unchanged-candidate-rerun/serial-prompt-transcript.txt
- tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local4-unchanged-candidate-rerun/serial-response-transcript.txt
- tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local4-unchanged-candidate-rerun/serial-retrospective-around-write.json
- tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local4-unchanged-candidate-rerun/serial-observe-after-write-1.json
- tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local4-unchanged-candidate-rerun/tftp-delta-before-restore.json
- tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local4-unchanged-candidate-rerun/post-restore-status.json

Audit evidence directory:

- tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local4-response-capture-audit/

Retained audit files:

- audit-summary.txt
- serial-observe-from-retrospective-start.json
- serial-observe-from-retrospective-start.txt
- serial-observe-from-write-cursor.json
- serial-observe-from-write-cursor.txt
- serial-tail-after-audit.json
- serial-tail-after-audit.txt
- task-id.txt

## Result

Classification: accepted-capture-gap-confirmed.

The local4 proof result recorded `prompt_found=1`, `pass_found=1`,
`selected_tftp_fetch=1`, `restore=PASS`, `entries_found=0`, and
`ready_next_found=0`. The accepted candidate was fetched over TFTP, reached the
`talos>` prompt, accepted the `ls /` serial write endpoint, and retained final
`classification=pi5-local-ls-root-complete` plus
`rpi5-local-ls-root-proof: PASS`.

The retained response window from cursor 3900142 to 3900300 contains only:

```text
xt prompt=true
rpi5-local-ls-root-proof: final participants=1 expected=1 errors=0 classification=pi5-local-ls-root-complete
rpi5-local-ls-root-proof: PASS
```

A read-only `/serial/observe` replay from the broader cursor 3899942 recovered
358 bytes and matched the existing retrospective window. A replay from the
write cursor 3900142 recovered 158 bytes and matched the stored
serial-observe-after-write-1 evidence. Neither replay recovered visible
`bin`, `dir`, `empty`, or `etc` lines, and neither recovered a complete
`ready-for-next prompt=true` line.

The visible-entry acceptance gate is therefore not satisfied by existing local4
evidence. Final PASS alone is not accepted as a substitute for visible root
entries. The paused phase10-pi5-local-ls-root-proof-20260601 should remain
paused for the queued unchanged-candidate capture-window proof.

## Hardware Lock

hardwareTestLock remained unlocked/restored and was not touched by this audit.
No lab mutation occurred.

## Validation

- static inspection: existing local4 source evidence reviewed.
- read-only lab-controller API: `/serial/observe` windows from cursors 3899942
  and 3900142 retained and compared against stored local4 evidence.
- static inspection: git diff --check passed.
- documentation: mdBook was not required because no mdBook docs were touched.
- staged static inspection: git diff --cached --check passed before commit.

## Next Action

Promote phase10-pi5-local-ls-root-capture-window-proof-20260601 on the next
worker wake if hardwareTestLock remains unlocked/restored. That task is already
defined as exactly one unchanged candidate rerun with a corrected retained
response capture window.
