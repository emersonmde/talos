# Phase 11 Known-Good Boot-State API Probe

Task id: phase11-known-good-boot-state-api-probe-20260605

Status: accepted

## Goal

Capture a read-only lab API boot-state probe for the restored known-good tree
before spending serialized Pi 5 hardware time.

## Scope

- Queried only read-only lab-controller endpoints: GET /health, GET /status,
  GET /boot/files, GET /boot/snapshots, and GET /tftp/logs?limit=1.
- Recorded restored boot tree identity, effective kernel, config fields, boot
  file listing, snapshots, and TFTP cursor/tail state.
- Compared deployed endpoint behavior against the repaired proof contract.
- Classified the next serialized known-good hardware discriminator as
  mechanically safe to run.

## Non-Goals Honored

No code changes, hardwareTestLock acquisition, power cycle, serial write, boot
archive publication, restore, RP1 diagnostic/source change, candidate rerun,
GPIO, interrupts, DMA/cache, storage, generated-root, networking, SSH, broader
PCIe, Milestone 11.2 work, or phase transition was performed.

## Findings And Disposition

- fixed: retained health/status/boot-files/boot-snapshots/TFTP-tail API
  evidence under
  tasks/evidence/2026-06-05-phase11-known-good-boot-state-api-probe/.
- fixed: GET /status and GET /boot/files agree on restored tree hash
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- fixed: both status surfaces report configured_kernel=kernel_2712.img and
  effective_kernel=kernel_2712.img.
- fixed: visible boot files include root and da591740/ copies of
  kernel_2712.img, kernel8.img, config.txt, DTB, and overlays.
- fixed: TFTP tail reports cursor/log_size state and a prior
  da591740/kernel_2712.img served event, giving the next hardware task a
  concrete fresh-cursor baseline.
- not-an-issue: active_name=kernel8.img does not block the next task because
  the accepted proof contract treats effective_kernel=kernel_2712.img as
  authoritative boot identity.
- removed: no workaround capture path, publish/restore action, or hardware
  mutation was added.
- deferred: known-good Talos serial readiness and stable fresh TFTP delta are
  still unaccepted until the next serialized hardware discriminator runs.

## Evidence

- Health: tasks/evidence/2026-06-05-phase11-known-good-boot-state-api-probe/health.json.
- Status: tasks/evidence/2026-06-05-phase11-known-good-boot-state-api-probe/status.json.
- Boot files: tasks/evidence/2026-06-05-phase11-known-good-boot-state-api-probe/boot-files.json.
- Boot snapshots: tasks/evidence/2026-06-05-phase11-known-good-boot-state-api-probe/boot-snapshots.json.
- TFTP tail: tasks/evidence/2026-06-05-phase11-known-good-boot-state-api-probe/tftp-tail.json.
- Derived summary:
  tasks/evidence/2026-06-05-phase11-known-good-boot-state-api-probe/read-only-api-summary.json.
- Static inspection:
  tasks/evidence/2026-06-05-phase11-known-good-boot-state-api-probe/static-evidence-inspection.md.
- Classification:
  tasks/evidence/2026-06-05-phase11-known-good-boot-state-api-probe/classification.json.

## Validation

- lab-controller API evidence: passed for health, status, boot files, boot
  snapshots, and TFTP tail.
- static evidence inspection: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed with existing warnings only.
- git diff --cached --check before commit: passed.

## Result

Accepted read-only API probe with classification
ready-for-serialized-discriminator.

The next queued task may run one serialized known-good Pi 5 discriminator if
hardwareTestLock remains unlocked/restored. RP1 candidate publication, RP1
diagnostic/source changes, candidate reruns, Milestone 11.2, networking, SSH,
GPIO, interrupts, DMA/cache, storage, generated-root, and broader PCIe remain
blocked.
