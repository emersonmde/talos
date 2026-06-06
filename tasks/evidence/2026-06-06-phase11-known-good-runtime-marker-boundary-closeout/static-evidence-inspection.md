# Static Evidence Inspection

Task id: phase11-known-good-runtime-marker-boundary-closeout-20260606

## Inputs

- `tasks/2026-06-06-phase11-known-good-runtime-serial-window-closeout.md`
- `tasks/evidence/2026-06-06-phase11-known-good-runtime-serial-window-closeout/evidence-map.json`
- `tasks/2026-06-06-phase11-known-good-runtime-marker-boundary-review-core.md`
- `tasks/evidence/2026-06-06-phase11-known-good-runtime-marker-boundary-review-core/evidence-map.json`
- `tasks/evidence/2026-06-06-phase11-known-good-runtime-marker-boundary-review-core/static-inspection.md`
- `tasks/evidence/2026-06-06-phase11-known-good-runtime-serial-window-pi5-discriminator/evidence-map.json`

## Observations

- The serial-window closeout accepted known-good fetch visibility and restore
  identity, but blocked `valid-known-good-talos-readiness` only because the
  fresh serial window omitted the earlier `TALOS: kernel_main` marker.
- The marker-boundary review core accepted
  `valid-known-good-talos-readiness-by-downstream-marker` after static source
  inspection proved `rpi5-production-timer-preemption: PASS` is emitted only
  after the restored known-good production-timer path has entered
  `kernel_main`.
- Retained TFTP evidence contains two stable 104,136-byte
  `da591740/kernel_2712.img` fetches for the restored known-good boot tree.
- Retained fresh serial evidence contains the downstream
  `rpi5-production-timer-preemption: PASS` marker in a 6,746-byte window from
  cursor 4096748, while both helper and direct-large observations omit
  `TALOS: kernel_main`.
- Hardware lock state is restored and released. This closeout does not acquire
  hardware, run a power cycle, publish a boot archive, or change runtime/RP1
  source.

## Boundary

- Accepted: valid known-good Talos runtime readiness for the restored
  production-timer control by the downstream PASS marker boundary.
- Not accepted: RP1 candidate fetch, RP1 Rust entry, RP1 entry-control
  reachability, RP1 mapped/read-value behavior, RP1 unmapped/trap behavior,
  firmware-state behavior, Milestone 11.2 work, networking, SSH, storage,
  DMA/cache, interrupts, GPIO ownership, or broader PCIe work.

## Next Action

The existing queued `phase11-rp1-entry-control-candidate-rerun-20260605` is the
only task made mechanically eligible by this closeout, subject to its own
hardware lock and validation gates. The queued serial-completeness
discriminator remains blocked because this closeout does not classify
`ready-for-serial-completeness-pi5-discriminator`.
