# Phase 11 Pi 5 Capture Freshness V3 Known-Good Proof

Task id: phase11-pi5-capture-freshness-v3-known-good-pi5-20260608

## Goal

Prove the accepted pi5-capture-transaction-v3 freshness procedure on the
known-good production-timer Pi 5 control before any observed GPIO14 STATUS/CTRL
retry.

## Result

Status: accepted

Classification: capture-transaction-v3-known-good-ready

The known-good production-timer control passed the v3 identity and freshness
join. The pre-power serial drain reached an empty read before power, the fresh
serial window contained `rpi5-production-timer-preemption: PASS`, TFTP retained
two stable `da591740/kernel_2712.img` fetches of 104,136 bytes, and the lab was
restored to the same production-timer tree.

This task makes no observed GPIO14 STATUS/CTRL, GPIO ownership, event
generation, interrupt delivery, broad RP1 mapping, DMA/cache, storage,
networking, SSH, Milestone 11.3, or phase-transition claim.

## Findings

- fixed: hardware lock acquisition, known-good power cycle, final selected-tree
  identity, and restore proof were retained for the known-good control.
- fixed: v3 serial freshness passed by the empty-read-before-power path:
  attempts=1, total_bytes=0, final_cursor=4194304.
- fixed: fresh serial output after power contained the required
  `rpi5-production-timer-preemption: PASS` marker.
- fixed: stable TFTP evidence retained two 104,136-byte
  `da591740/kernel_2712.img` fetches from cursor 4338948 to 4340299.
- not-an-issue: the saturated direct-read marker-differential path was not
  required because the empty pre-power drain was decisive.
- deferred: this proof only validates the v3 capture path on the known-good
  control. The observed GPIO14 STATUS/CTRL control and real candidate remain
  separate queued tasks.

## Evidence

- Evidence map:
  `tasks/evidence/2026-06-08-phase11-pi5-capture-freshness-v3-known-good-pi5/evidence-map.json`.
- Classification:
  `tasks/evidence/2026-06-08-phase11-pi5-capture-freshness-v3-known-good-pi5/classification.json`.
- V3 identity/freshness join:
  `tasks/evidence/2026-06-08-phase11-pi5-capture-freshness-v3-known-good-pi5/local2-v3-check.json`.
- Known-good capture summary:
  `tasks/evidence/2026-06-08-phase11-pi5-capture-freshness-v3-known-good-pi5/local2-known-good-control/capture-invariant-summary.json`.
- Pre-run lab status:
  `tasks/evidence/2026-06-08-phase11-pi5-capture-freshness-v3-known-good-pi5/local2-known-good-control/pre-status.json`.
- Serial cursor and drain evidence:
  `tasks/evidence/2026-06-08-phase11-pi5-capture-freshness-v3-known-good-pi5/local2-known-good-control/serial-cursor-before-power.txt`,
  `tasks/evidence/2026-06-08-phase11-pi5-capture-freshness-v3-known-good-pi5/local2-known-good-control/serial-drain-before-power.json`,
  and
  `tasks/evidence/2026-06-08-phase11-pi5-capture-freshness-v3-known-good-pi5/local2-known-good-control/serial-observe-window.json`.
- TFTP cursor and stable delta:
  `tasks/evidence/2026-06-08-phase11-pi5-capture-freshness-v3-known-good-pi5/local2-known-good-control/tftp-cursor-before-power.json`
  and
  `tasks/evidence/2026-06-08-phase11-pi5-capture-freshness-v3-known-good-pi5/local2-known-good-control/tftp-delta-stable-pre-restore.json`.
- Final selected-tree identity:
  `tasks/evidence/2026-06-08-phase11-pi5-capture-freshness-v3-known-good-pi5/local2-known-good-control/final-pre-restore-status.json`
  and
  `tasks/evidence/2026-06-08-phase11-pi5-capture-freshness-v3-known-good-pi5/local2-known-good-control/final-pre-restore-boot-files.json`.
- Restore proof:
  `tasks/evidence/2026-06-08-phase11-pi5-capture-freshness-v3-known-good-pi5/local2-known-good-control/restore-snapshot.json`,
  `tasks/evidence/2026-06-08-phase11-pi5-capture-freshness-v3-known-good-pi5/local2-known-good-control/post-restore-status.json`,
  and
  `tasks/evidence/2026-06-08-phase11-pi5-capture-freshness-v3-known-good-pi5/post-local2-lab-status.json`.

## Validation

- lab-controller API: `GET /` before the run recorded
  `effective_kernel=kernel_2712.img` and tree
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.
- serial hardware boot/output: fresh cursor and pre-power drain were retained;
  the post-power serial window contained the production-timer PASS marker.
- lab-controller API: `GET /tftp/logs` after the run recorded stable TFTP delta
  with two 104,136-byte expected fetches.
- lab-controller API: post-restore `GET /` recorded restored tree
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.
- static tool check: `jq empty` passed on retained JSON evidence.
- static diff checks: `git diff --check` and `git diff --cached --check`
  passed.

## Next Action

On the next worker wake, promote
`phase11-rp1-observed-gpio-status-v3-control-pi5-20260608` if the hardware lock
remains unlocked/restored. Use the accepted v3 procedure and do not make an
observed GPIO14 STATUS/CTRL claim in the control task.
