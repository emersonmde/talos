# Static Evidence Inspection

Task id: phase11-known-good-boot-state-api-probe-20260605

Evidence level: lab-controller API + static evidence inspection.

## Inputs

- health.json: read-only GET /health.
- status.json: read-only GET /status.
- boot-files.json: read-only GET /boot/files.
- boot-snapshots.json: read-only GET /boot/snapshots.
- tftp-tail.json: read-only GET /tftp/logs?limit=1.
- classification.json: derived classification from the retained API payloads.

## Findings And Disposition

- fixed: retained the deployed read-only endpoint set required by the repaired
  proof contract: health, status, boot files, boot snapshots, and TFTP tail.
- fixed: GET /status and GET /boot/files agree on restored tree hash
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- fixed: both endpoints report configured_kernel=kernel_2712.img and
  effective_kernel=kernel_2712.img.
- fixed: the boot file listing contains root and da591740/ copies of
  kernel_2712.img, kernel8.img, config.txt, and the Pi 5 DTB/overlays.
- fixed: the lab target metadata reports UniFi guard fixed-port and target
  PoE state UP.
- fixed: the TFTP tail endpoint exposes cursor_start=4028715,
  cursor_end=4094251, log_size=4094251, and one prior
  da591740/kernel_2712.img served event, so the next serialized hardware
  discriminator can start from a fresh cursor.
- not-an-issue: active_name=kernel8.img remains present alongside the
  configured/effective Pi 5 kernel; the proof contract treats
  effective_kernel=kernel_2712.img as authoritative.
- removed: no boot archive publication, restore, serial write, power cycle, or
  hardware lock acquisition was performed during this read-only probe.
- deferred: actual known-good Talos readiness and fresh stable TFTP delta
  evidence remain scoped to the next serialized hardware discriminator.

## Classification

ready-for-serialized-discriminator.

No read-only API field blocked the next queued hardware discriminator. The
classification does not accept RP1 candidate fetch, entry reachability,
mapped/read-value behavior, unmapped/trap behavior, or known-good Talos serial
readiness.
