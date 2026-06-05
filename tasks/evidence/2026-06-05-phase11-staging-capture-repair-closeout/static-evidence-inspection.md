# Phase 11 Staging/Capture Repair Closeout Static Evidence Inspection

Task id: phase11-staging-capture-repair-closeout-20260605

## Inputs Inspected

- `tasks/2026-06-05-phase11-staging-capture-log-stability-core.md`
- `tasks/evidence/2026-06-05-phase11-staging-capture-log-stability-core/static-inspection.md`
- `tasks/evidence/2026-06-05-phase11-staging-capture-log-stability-core/tftp-cursor-4088847-stable-replay.json`
- `tasks/2026-06-05-phase11-staging-capture-known-good-pi5-proof.md`
- `tasks/evidence/2026-06-05-phase11-staging-capture-known-good-pi5-proof/proof-summary.json`
- `tasks/evidence/2026-06-05-phase11-staging-capture-known-good-pi5-proof/known-good-tftp-delta-stable-pre-restore.json`
- `tasks/evidence/2026-06-05-phase11-staging-capture-known-good-pi5-proof/attempt2-known-good-tftp-delta-stable-pre-restore.json`
- `tasks/evidence/2026-06-05-phase11-staging-capture-known-good-pi5-proof/lab-status-before.json`
- `tasks/evidence/2026-06-05-phase11-staging-capture-known-good-pi5-proof/attempt2-lab-status-before.json`
- `tasks/evidence/2026-06-05-phase11-staging-capture-known-good-pi5-proof/final-restore-attempt2.json`

## Stable-Log Rule Evidence

The accepted proof-rule repair defines the TFTP stable-log condition as repeated `/tftp/logs` reads from the same cursor whose `cursor_end`, `log_size`, `truncated`, and parsed event set remain unchanged for the required sample count or until the bounded timeout is recorded.

Replay from cursor `4088847` completed with `stable=true`, `stable_samples=2`, `required_samples=2`, and returned 13 events. The retained replay included late-visible restored known-good `da591740/kernel_2712.img` events totaling 104,136 bytes. This proves that a single proof-time empty delta was not sufficient evidence of no fetch.

Disposition: fixed. The stable-log rule is accepted as the required TFTP classification method for future Pi 5 proof records.

## Known-Good Hardware Evidence

The known-good proof used boot tree `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10` with `effective_kernel=kernel_2712.img`.

Attempt 1 recorded fresh serial cursor and TFTP cursor `4091549`; the stable pre-restore TFTP result had zero events, and serial did not reach `TALOS: kernel_main`, command-loop readiness, or PASS.

Attempt 2 recorded fresh serial cursor and TFTP cursor `4092900`; the stable pre-restore TFTP result had zero events, and serial reached Raspberry Pi firmware/RP1 boot output through `RP1 FW: load 0` but not `TALOS: kernel_main`, command-loop readiness, or PASS.

The final restore returned the boot tree to `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`, and the durable supervisor state records `hardwareTestLock.locked=false` and `hardwareTestLock.restored=true`.

Disposition: deferred. The stable-log rule was exercised, but the known-good control did not show a known-good `kernel_2712.img` fetch or Talos serial readiness, so RP1 candidate reruns remain blocked.

## Endpoint Semantics

The known-good task recorded that the deployed lab API returned `404 unknown endpoint: GET /`, while `GET /status` reported the boot identity and restore state. The repository notes currently document `GET /` as authoritative in some places and `/status` in others.

Disposition: deferred. The closeout records the discrepancy but does not change lab-controller API semantics or add workaround tooling.

## Closeout Classification

Accepted:

- The stable TFTP classification rule is accepted as proof-record evidence semantics.
- The log-stability repair and known-good blocker evidence are committed.
- Hardware lock state is restored.

Unresolved:

- The repaired rule is not yet proven sufficient for a known-good control fetch.
- The lab/staging/capture path remains blocked for RP1 candidate reruns.
- RP1 candidate fetch, Rust entry, entry-control reachability, RP1 mapped/read-value, RP1 trap/unmapped, GPIO, interrupts, DMA/cache, storage, generated-root work, networking, SSH, broader PCIe, and Milestone 11.2 remain unaccepted.

Next planning should choose a bounded lab-controller/capture or staging-publication discriminator before any RP1 diagnostic/source rerun.
