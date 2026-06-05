# Static Evidence Inspection

Task: `phase11-rp1-diagnostic-entry-control-closeout-20260605`

Evidence level: static inspection of accepted task records and retained lab
artifacts. No hardware lock, archive publication, TFTP operation, serial
capture, power cycle, or source change was performed by this closeout.

## Source/Local Evidence

- `tasks/2026-06-05-phase11-rp1-diagnostic-entry-control-source-core.md`
  is accepted at commit `4460e80e211cf73e2bd7f2b45a8a8b59cf75ac77`.
- The task-owned archive is
  `target/talos-rpi5-rp1-entry-control-source-core.tar.gz` with SHA-256
  `dcbcf06ebdf2304630dc52d0aac689c6ec363f04074a055bc391a0c7829e5f37`.
- The root/prefixed `kernel_2712.img` SHA-256 is
  `b3e62b950cf007a0ee8d1d7f420fd8c26c28573c5b6925a7f0d93d0b77a367ea`,
  size `51808`, with `text_offset=0`, `header_image_size=51808`,
  `flags=12`, `magic=ARMd`, and entry/`_start` at `0x200000`.
- Source comparison disposition is accepted:
  - fixed: `rpi5_rp1_entry_control` branches immediately after the normal
    Pi 5 `EarlyPhaseLine::RustEntry` output.
  - fixed: the candidate stops before `BootInfo::from_aarch64_x0`,
    `target::init`, boot reports, memory planning, allocator setup, normal
    RP1 GPIO/pin flushes, or the RP1 UART0 FR read path.
  - fixed: marker evidence confirms
    `rpi5-rp1-entry-control: rust-entry-control`,
    `rpi5-rp1-entry-control: no-rp1-mmio`,
    `rpi5-rp1-entry-control: classification=entry-control-reached`, and
    `rpi5-rp1-entry-control: PASS`.
  - removed: quarantined raw assembly provenance markers remain absent.
  - not-an-issue: RP1 register-read classification strings remain absent from
    the entry-control candidate.
  - deferred: only serialized Pi 5 evidence can prove candidate fetch and
    entry-control reachability.

## Hardware Proof Evidence

- `tasks/2026-06-05-phase11-rp1-diagnostic-entry-control-pi5-proof.md`
  is completed with blocker at commit
  `ad6f929aa88b319dc31c0a4d7c4b921ea055d20f`.
- `proof-summary.txt` records
  `classification=staging-or-capture-blocker`.
- Published candidate tree:
  `ab88a3d8549837459c8cebf8cb22580b52b39665421b7eb6d6773ebce8c6f9c2`.
- Pre-run/restored accepted tree:
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.
- Candidate identity records the selected 51,808-byte
  `da591740/kernel_2712.img` with SHA-256
  `b3e62b950cf007a0ee8d1d7f420fd8c26c28573c5b6925a7f0d93d0b77a367ea`.
- First candidate, known-good control, and candidate rerun serial evidence all
  retained visible Raspberry Pi firmware output through `Boot mode: NETWORK`.
- First candidate, known-good control, and candidate rerun TFTP deltas all
  record zero fresh events, so this proof does not prove candidate fetch,
  known-good fetch, or kernel handoff.
- No retained serial evidence reaches `TALOS: kernel_main`,
  `rpi5-rp1-entry-control: rust-entry-control`,
  `rpi5-rp1-entry-control: PASS`, RP1 mapped/read-value, RP1 unmapped/trap,
  or firmware-state classification.
- Final restore evidence records the lab boot tree returned to
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10` and
  `hardwareTestLock` released/restored.

## Closeout Classification

Accepted:

- source-backed RP1/PCIe address contract exists from the prior Milestone 11.1
  contract task.
- local entry-control discriminator exists and is positioned before BootInfo,
  normal Pi 5 target initialization, and any RP1 MMIO side effect.
- candidate publication/staging and restore state are recorded.

Blocked:

- this proof did not retain fresh TFTP fetch evidence for the candidate or the
  known-good control.
- this proof did not reach Talos Rust entry, the entry-control marker/PASS, or
  the RP1 diagnostic path.
- RP1 mapped/read-value, RP1 unmapped/trap, and firmware-state behavior remain
  unaccepted.

Deferred:

- supervisor-planned source-level investigation of the staging/capture or
  pre-entry/handoff boundary.
- any revised diagnostic shape.
- Milestone 11.2, GPIO ownership, interrupts, DMA/cache policy, networking,
  SSH, storage, generated-root blocker work, or broader PCIe work.
