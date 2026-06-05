# Phase 10 to Phase 11 Transition Checkpoint

Task: phase10-to-phase11-transition-checkpoint-20260605

Status: accepted

## Goal

Decide explicitly whether Phase 10 is ready to close and Phase 11 RP1/PCIe/DMA
substrate work can begin.

## Outcome

Phase 10 is closed at the accepted local shell and local/QEMU generated-root
frontier. Phase 11 may begin with Milestone 11.1 RP1 and PCIe mapping.

The accepted Phase 10 boundary includes:

- local serial shell interaction and line editing on Pi 5;
- shell-visible VFS/open/read behavior backed by descriptor, syscall-substitute,
  loader, userspace launch, process lifecycle, argv/envp, standard descriptor,
  waitpid, PATH-style lookup, pipe, redirection, cwd, async job, and
  generated-root records;
- local/QEMU generated-root no-kernel-rebuild transport with distinct external
  talos-generated-root-v1 artifacts;
- Milestone 10.3 acceptance at the local/QEMU generated-root transport
  frontier.

The deferred Phase 10 work is not a blocker for Phase 11 entry:

- Pi 5 generated-root artifact consumption remains deferred until Talos reserves
  or copies the firmware initramfs range before early memory setup and passes a
  fresh serialized proof;
- writable persistence, SD/USB/block storage, broader filesystem mutation,
  networking, SSH, and terminal/session expansion remain future roadmap work.

The next phase should start from the smallest useful user-visible substrate
capability: source-backed RP1/PCIe mapping, then a narrow RP1 register-read
diagnostic, then a serialized Pi 5 proof if local review justifies hardware
time.

## Evidence

Accepted closeout records:

- tasks/2026-06-03-phase10-startup-abi-closeout.md
- tasks/2026-06-03-phase10-absolute-vfs-exec-dispatch-closeout.md
- tasks/2026-06-03-phase10-vfs-exec-nonzero-status-closeout.md
- tasks/2026-06-03-phase10-waitpid-lifecycle-observation-closeout.md
- tasks/2026-06-03-phase10-standard-descriptor-inheritance-closeout.md
- tasks/2026-06-03-phase10-literal-argv-exec-closeout.md
- tasks/2026-06-03-phase10-minimal-path-lookup-exec-closeout.md
- tasks/2026-06-03-phase10-userspace-stdio-triad-closeout.md
- tasks/2026-06-04-phase10-file-state-frontier-closeout.md
- tasks/2026-06-04-phase10-process-cwd-frontier-closeout.md
- tasks/2026-06-05-phase10-process-control-frontier-checkpoint.md
- tasks/2026-06-05-phase10-local-storage-milestone-closeout.md

Key blocker record:

- tasks/2026-06-05-phase10-pi5-generated-root-boot-transport-proof.md

Supporting project context:

- docs/src/roadmap.md
- docs/src/project/reference-notes.md
- docs/src/project/lab-controller.md
- docs/src/project/testing-strategy.md

## Findings

- fixed: Confirmed Phase 10 closeout is explicit and source-backed through
  accepted task records and roadmap status.
- fixed: Kept the Pi 5 generated-root blocker deferred rather than letting it
  hold Phase 11 RP1/PCIe mapping.
- fixed: Selected the first Phase 11 tasks around RP1/PCIe mapping before
  networking, SSH, GPIO, interrupts, or DMA driver work.
- deferred: Pi 5 generated-root transport acceptance still requires a later
  firmware-initramfs reservation/copy implementation and serialized proof.
- deferred: RP1 interrupt routing, GPIO ownership, DMA/cache rules, networking,
  and SSH remain outside Milestone 11.1's first task slice.
- not-an-issue: No hardware lock is needed for this transition checkpoint.

## Validation

- static evidence and roadmap inspection: passed.
- diff hygiene: git diff --check passed.
- documentation: /home/node/.cargo/bin/mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before commit.

## Next Action

Begin Phase 11 Milestone 11.1 with
phase11-rp1-pcie-map-source-contract-20260605. The worker should not jump to
networking, SSH, DMA, GPIO, or hardware proof before the source-backed mapping
contract and diagnostic core are accepted.
