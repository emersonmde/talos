# Phase 11 RP1/PCIe Mapping Closeout Static Evidence Inspection

Task: phase11-rp1-pcie-mapping-closeout-20260605

## Inspected Records

- `tasks/2026-06-05-phase11-rp1-pcie-map-source-contract.md`
- `docs/src/project/phase11-rp1-pcie-map-contract.md`
- `tasks/2026-06-05-phase11-rp1-register-read-diagnostic-core.md`
- `tasks/2026-06-05-phase11-rp1-register-read-pi5-proof.md`
- `tasks/evidence/2026-06-05-phase11-rp1-register-read-pi5-proof/proof-summary.txt`
- `tasks/evidence/2026-06-05-phase11-rp1-register-read-pi5-proof/post-hardware-review.txt`

## Findings

- fixed: source contract v1 records `cpu_phys = 0x1f_0000_0000 + (rp1_bus - 0xc0_4000_0000)` and selects RP1 UART0 PL011 `FR` at `0x1f_0003_0018` for a 32-bit read-only diagnostic.
- fixed: diagnostic core output is selected only by the explicit Pi 5 boot scenario and does not change accepted shell/VFS behavior.
- fixed: diagnostic core evidence records the candidate image identity:
  `kernel_2712-rp1-uart0-fr-read.img`, SHA-256
  `bed60fc8babf5c91117dd1ccb7c9a105af2bcd30cfedcc098a414029b46fe3c5`,
  size `87392`.
- fixed: hardware proof records archive SHA-256
  `937d749b4fe2ef40a5ee730461ebae7108edad437b3b216856d7b549b5129e0a`,
  candidate tree
  `a96f0d8dc17a4872cb52e94c37c85d5adc5312255d26f988dbd8b71e1b6118c9`,
  and restored accepted tree
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.
- fixed: local2 and local3 TFTP deltas both show the candidate
  `da591740/kernel_2712.img` served at 87,392 bytes before restore.
- fixed: known-good control retained `TALOS: kernel_main` and accepted
  command-loop output on the restored boot tree, so serial capture and restore
  viability are covered.
- deferred: no candidate run reached `rpi5-rp1-uart0-fr-read`,
  `mapped/read-value`, or `PASS`; the retained boundary is
  `blocked-pre-entry-or-handoff-after-candidate-fetch`.
- not-an-issue: the blocked proof is not evidence for or against the RP1
  register mapping itself.

## Acceptance Boundary

Accepted for Milestone 11.1:

- source-backed initial RP1/PCIe address contract;
- local non-destructive RP1 UART0 flag-register diagnostic candidate;
- serialized Pi 5 proof evidence showing candidate publication/fetch and a
  pre-entry/handoff blocker.

Not accepted:

- stable RP1 register read on hardware;
- RP1 mapped/read-value classification;
- GPIO ownership, interrupts, DMA/cache policy, Ethernet, networking, SSH,
  storage drivers, broader PCIe enumeration, or generated-root blocker work.
