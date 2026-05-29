# ADR Index

Architectural decision records live here.

Create an ADR when a decision is expensive to reverse, affects subsystem boundaries, constrains future POSIX compatibility, or changes the hardware lab contract.

ADR template:

- Status:
- Context:
- Decision:
- Consequences:
- Alternatives considered:

## 2026-05-29 - Phase 7.3 Pi 5 Syscall Proof Closeout Accepted

- Status: accepted as the documentation closeout for the first serialized
  Raspberry Pi 5 production syscall routing proof. No Rust behavior, assembly
  behavior, QEMU rerun, Pi 5 hardware rerun, archive publishing, hardware-lock
  acquisition, descriptor I/O, copy-in/copy-out, process loading,
  VFS/filesystem, shell behavior, networking, SSH, RP1/PCIe, UART interrupt
  ownership, or DMA/cache-driver policy was added.
- Context: The accepted syscall ABI contract, target-independent dispatch
  core, production trap-routing contract, QEMU syscall smoke, and Pi 5 syscall
  proof needed one checkpoint that states the exact physical capability,
  retained evidence, hardware-lock timeline, restore proof, deferred surfaces,
  and next bounded task.
- Decision: Accept
  phase7-pi5-syscall-proof-closeout-checkpoint-20260529. The accepted frontier
  is physical Pi 5 production routing for lower-AArch64 svc #0: talos_nop
  returns x0 = 0, unknown syscall number 17 returns
  x0 = 0xffffffffffffffda (-ENOSYS), diagnostic marker 0x7a10 remains
  proof-only, and the local3 rerun reports
  classification=pi5-syscall-proof-complete plus rpi5-syscall-proof: PASS.
- Evidence level: static documentation inspection and retained evidence review.
  Retained QEMU logs are in
  tasks/evidence/2026-05-29-qemu-syscall-smoke-core/. Retained Pi 5 proof,
  TFTP, and restore evidence is in
  tasks/evidence/2026-05-29-pi5-syscall-proof/.
- Validation: git status --short before edits was clean; git diff --check
  passed; mdbook build passed. No QEMU or Pi 5 rerun was performed by the
  closeout checkpoint.
- Consequences: Pi 5 production syscall routing is closed out for the first
  scalar syscall boundary. Descriptor I/O, byte copy-in/copy-out,
  pointer-taking syscalls, process loading, VFS/filesystem, shell, networking,
  SSH, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy remain
  blocked. The next bounded task is
  phase7-copyin-copyout-helper-contract-20260529.

## 2026-05-29 - Phase 7.3 Pi 5 Syscall Proof Accepted

- Status: accepted as the serialized Raspberry Pi 5 production syscall routing
  proof for the first scalar syscall boundary. No descriptor I/O,
  copy-in/copy-out byte helper, pointer-taking syscall, process loading,
  VFS/filesystem, shell behavior, networking, SSH, RP1/PCIe, UART interrupt
  ownership, or DMA/cache-driver policy is accepted.
- Context: The accepted QEMU syscall smoke and Pi 5 syscall proof plan required
  physical evidence that lower-AArch64 svc #0 reaches the production dispatch
  core on Pi 5 and returns the accepted x0 values to lower EL.
- Decision: Accept phase7-pi5-syscall-proof-20260529. The implementation adds
  rpi5_syscall_proof, a focused Pi 5 recoverable exception path for lower
  AArch64 svc #0, Pi 5 image/boot-tree helpers, and retained lab evidence.
  The accepted physical invariant is talos_nop returning x0 = 0, unknown
  syscall number 17 returning x0 = 0xffffffffffffffda (-ENOSYS), and diagnostic
  marker 0x7a10 remaining proof-only and outside stable dispatch.
- Evidence level: serialized Pi 5 hardware boot/output under hardwareTestLock,
  static archive/image inspection, QEMU/substitute regression evidence, unit
  tests, and restore proof. The first candidate run was inconclusive, so the
  accepted record includes candidate identity, fresh serial and TFTP cursors,
  a passing production-timer known-good control, and an unchanged candidate
  rerun before acceptance. Retained evidence is in
  tasks/evidence/2026-05-29-pi5-syscall-proof/.
- Validation: cargo fmt --all -- --check passed; cargo -Zjson-target-spec test
  passed; scripts/qemu-syscall-smoke.sh passed; scripts/rpi5-archive-review.sh
  target/talos-rpi5-syscall-proof-boot.tar.gz passed; serialized Pi 5 local3
  candidate rerun reported classification=pi5-syscall-proof-complete and
  rpi5-syscall-proof: PASS; restore returned the prior boot-tree hash; git
  diff --check passed; mdbook build passed.
- Consequences: Pi 5 production routing for stable svc #0 is physically
  accepted. Descriptor I/O, copy-in/copy-out, process loading, VFS/filesystem,
  shell, networking, SSH, RP1/PCIe, UART interrupt ownership, and
  DMA/cache-driver policy remain blocked. The next bounded work should be the
  planned Pi 5 syscall proof closeout checkpoint before any copy-in/copy-out or
  descriptor syscall work.

## 2026-05-29 - Phase 7.3 Syscall Routing Closeout Accepted

- Status: accepted as the documentation closeout for the first QEMU-only
  production syscall routing frontier. No Rust behavior, assembly behavior,
  QEMU rerun, Pi 5 hardware run, archive publishing, hardware-lock
  acquisition, descriptor I/O, copy-in/copy-out, process loading,
  VFS/filesystem, shell behavior, networking, SSH, RP1/PCIe, UART interrupt
  ownership, or DMA/cache-driver policy was added.
- Context: The accepted syscall ABI contract, target-independent dispatch
  core, production trap-routing contract, QEMU syscall smoke plan, and QEMU
  syscall smoke core together needed one checkpoint that states the exact
  proven capability and keeps deferred POSIX and hardware surfaces blocked.
- Decision: Accept
  phase7-syscall-routing-closeout-checkpoint-20260529. The accepted frontier
  is QEMU/substitute production syscall routing for lower-AArch64 svc #0:
  talos_nop returns x0 = 0, unknown syscall number 17 returns
  x0 = 0xffffffffffffffda, and diagnostic marker 0x7a10 remains proof-only.
- Evidence level: static documentation inspection and retained QEMU/substitute
  evidence review. Retained logs are in
  tasks/evidence/2026-05-29-qemu-syscall-smoke-core/.
- Validation: git status --short before edits was clean; git diff --check
  passed; mdbook build passed.
- Consequences: QEMU-only production syscall routing is closed out. Pi 5
  production syscall proof, descriptor I/O, copy-in/copy-out, process loading,
  VFS/filesystem, shell, networking, SSH, RP1/PCIe, UART interrupt ownership,
  and DMA/cache-driver policy remain blocked. The checkpoint recommends
  supervisor planning for a documentation-only Pi 5 syscall proof plan before
  any serialized hardware action.

## 2026-05-29 - Phase 7.3 QEMU Syscall Smoke Core Accepted

- Status: accepted as the first QEMU-only production syscall routing smoke. No
  Pi 5 hardware run, archive publishing, hardware-lock acquisition, descriptor
  I/O, copy-in/copy-out, process loading, VFS/filesystem, shell behavior,
  networking, SSH, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver
  policy was added or claimed.
- Context: The accepted syscall trap-routing contract and QEMU syscall smoke
  plan required retained QEMU evidence that lower-AArch64 svc #0 reaches the
  target-independent dispatch core and returns accepted x0 values.
- Decision: Accept phase7-qemu-syscall-smoke-core-20260529. The implementation
  adds qemu_syscall_smoke, a bounded recoverable lower-AArch64 svc #0 route,
  saved-frame x0 mutation, user-observed talos_nop and unknown-syscall return
  evidence, diagnostic marker 0x7a10 quarantine, and scripts/qemu-syscall-smoke.sh.
- Evidence level: QEMU/substitute plus unit tests and static inspection.
  Retained logs are in
  tasks/evidence/2026-05-29-qemu-syscall-smoke-core/.
- Validation: git status --short before edits was clean; cargo fmt --all --
  --check passed; cargo -Zjson-target-spec test passed;
  scripts/qemu-el0-trap-smoke.sh passed; scripts/qemu-syscall-smoke.sh passed;
  git diff --check passed; mdbook build passed.
- Consequences: Talos now has QEMU evidence for the first production syscall
  routing path, but Pi 5 syscall hardware proof, descriptor I/O,
  copy-in/copy-out, process loading, VFS/filesystem, shell, networking, SSH,
  RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy remain
  blocked.

## 2026-05-29 - Phase 7.3 QEMU Syscall Smoke Plan Accepted

- Status: accepted as a documentation-only QEMU production syscall smoke plan.
  No Rust behavior, assembly behavior, boot scenario, QEMU run, Pi 5 hardware
  run, archive publishing, hardware-lock acquisition, descriptor I/O,
  copy-in/copy-out, process loading, VFS/filesystem, shell behavior,
  networking, SSH, RP1/PCIe, UART interrupt ownership, or DMA/cache behavior
  was added.
- Context: The accepted production syscall trap-routing contract requires a
  QEMU-only smoke before Talos claims runtime syscall routing behavior.
- Decision: Accept phase7-qemu-syscall-smoke-plan-20260529. The plan defines
  qemu_syscall_smoke, stable svc #0 talos_nop with x8 = 0 and x0 = 0 after
  return, unknown syscall x8 = 17 with x0 = 0xffffffffffffffda after return,
  diagnostic marker 0x7a10 as proof-only completion vocabulary, and exact
  qemu-syscall-smoke classification/PASS lines.
- Evidence level: static documentation inspection, documentation build, and
  whitespace inspection. No QEMU or Pi 5 hardware evidence was claimed.
- Validation: git status --short before edits was clean; git diff --check
  passed; mdbook build passed.
- Consequences: The next bounded task is
  phase7-qemu-syscall-smoke-core-20260529. Pi 5 syscall hardware proof,
  descriptor I/O, copy-in/copy-out, process loading, VFS/filesystem, shell,
  networking, SSH, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver
  policy remain blocked.

## 2026-05-29 - Phase 7.3 Syscall Trap-Routing Contract Accepted

- Status: accepted as a documentation-only production syscall trap-routing
  contract. No Rust behavior, assembly behavior, boot scenario, QEMU run,
  Pi 5 hardware run, archive publishing, hardware-lock acquisition,
  descriptor I/O, copy-in/copy-out, process loading, VFS/filesystem, shell
  behavior, networking, SSH, RP1/PCIe, UART interrupt ownership, or
  DMA/cache behavior was added.
- Context: The accepted trap-routing source inventory identified the source
  owners and gaps for lower-AArch64 SVC detection, svc immediate validation,
  x8 extraction, x0-through-x5 argument capture, x0 return mutation,
  ELR/SPSR handling, diagnostic proof quarantine, and non-syscall fallback.
- Decision: Accept phase7-syscall-trap-routing-contract-20260529. Production
  syscall routing is limited to LowerAarch64Sync with ESR EC 0x15 and svc #0,
  uses x8 as the syscall number and x0 through x5 as scalar arguments, writes
  only the dispatch return value to x0, preserves ELR/SPSR, keeps non-syscall
  traps on the fatal exception path, and keeps diagnostic SVC marker 0x7a10
  proof-only.
- Evidence level: static documentation inspection, documentation build, and
  whitespace inspection. No QEMU or Pi 5 hardware evidence was claimed.
- Validation: git status --short before edits was clean; git diff --check
  passed; mdbook build passed.
- Consequences: The next bounded task is
  phase7-qemu-syscall-smoke-plan-20260529. Production routing implementation,
  QEMU syscall smoke core, Pi 5 hardware proof, descriptor I/O,
  copy-in/copy-out, process loading, VFS/filesystem, shell, networking, SSH,
  RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy remain
  blocked.

## 2026-05-29 - Phase 7.3 Syscall Trap-Routing Source Inventory Accepted

- Status: accepted as a documentation-only production syscall trap-routing
  source inventory. No Rust behavior, assembly behavior, boot scenario, QEMU
  run, Pi 5 hardware run, archive publishing, hardware-lock acquisition,
  descriptor I/O, copy-in/copy-out, process loading, VFS/filesystem, shell
  behavior, networking, SSH, RP1/PCIe, UART interrupt ownership, or
  DMA/cache behavior was added.
- Context: The accepted syscall ABI contract and target-independent dispatch
  core define svc #0, x8 syscall numbers, x0-through-x5 scalar arguments, x0
  return/error encoding, talos_nop success, unknown-syscall -ENOSYS, and
  diagnostic marker 0x7a10 as proof-only. Production exception routing is
  still absent.
- Decision: Accept phase7-syscall-trap-routing-source-inventory-20260529. The
  inventory maps source owners and gaps for lower-AArch64 SVC detection, svc
  immediate validation, x8 extraction, x0-through-x5 argument capture, x0
  return mutation, ELR/SPSR handling, diagnostic proof quarantine, optional
  task/process context, and non-syscall fatal fallback.
- Evidence level: static source inspection, static documentation inspection,
  documentation build, and whitespace inspection. No QEMU or Pi 5 hardware
  evidence was claimed.
- Validation: git status --short before edits was clean; git diff --check
  passed; mdbook build passed.
- Consequences: The next bounded task is
  phase7-syscall-trap-routing-contract-20260529. Production routing
  implementation, QEMU syscall smoke, Pi 5 hardware proof, descriptor I/O,
  copy-in/copy-out, process loading, VFS/filesystem, shell, networking, SSH,
  RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy remain
  blocked.

## 2026-05-29 - Phase 7.3 Syscall Dispatch Core Accepted

- Status: accepted as the first target-independent syscall dispatch core. No
  assembly behavior, production exception routing, boot scenario, QEMU run,
  Pi 5 hardware run, archive publishing, hardware-lock acquisition, process
  loading, descriptor I/O, VFS/filesystem, shell behavior, networking, SSH,
  RP1/PCIe, UART interrupt ownership, or DMA/cache behavior was added.
- Context: The accepted syscall ABI contract fixed lower-AArch64 svc #0, x8 as
  the syscall-number register, x0 through x5 as scalar arguments, x0 as the
  sole return register, negative errno returns, talos_nop = 0, unknown syscall
  = -ENOSYS, and diagnostic marker 0x7a10 as proof-only vocabulary.
- Decision: Accept phase7-syscall-dispatch-core-20260529. The new
  target-independent syscall module owns the stable SVC immediate constant,
  diagnostic marker quarantine constant, talos_nop syscall number, scalar
  argument view, return/error encoder, accepted errno subset, and pure dispatch
  function for talos_nop success and unknown-syscall -ENOSYS.
- Evidence level: static source inspection, target-independent unit tests,
  formatting, whitespace inspection, and documentation build. No QEMU or Pi 5
  hardware evidence was claimed.
- Validation: git status --short before edits was clean;
  cargo fmt --all -- --check passed; cargo -Zjson-target-spec test passed;
  git diff --check passed; mdbook build passed.
- Consequences: Talos has a unit-tested dispatch vocabulary but no production
  syscall trap integration. Production exception-handler routing, QEMU syscall
  smoke, Pi 5 hardware proof, pointer-copy syscalls, descriptor I/O, process
  loading, VFS/filesystem, shell, networking, SSH, RP1/PCIe, UART interrupt
  ownership, and DMA/cache-driver policy remain blocked until later explicit
  tasks.
- Risks: The core intentionally falls back to -ENOSYS for PosixError values
  outside the accepted errno subset, and it does not yet decide process-fatal
  trap policy, per-thread errno, restart semantics, or user-buffer copy
  behavior.

## 2026-05-29 - Phase 7.3 Syscall ABI Source Inventory Accepted

- Status: accepted as the documentation-only source inventory before syscall
  ABI contract or implementation. No Rust behavior, assembly behavior,
  syscall numbers, syscall dispatch, copy-in/copy-out helpers, descriptor I/O,
  process loading, VFS/filesystem, shell behavior, networking, SSH, QEMU rerun,
  Pi 5 hardware run, archive publication, hardware-lock use, RP1/PCIe, UART
  interrupt ownership, or DMA/cache-driver policy was added.
- Context: The bounded QEMU and physical Pi 5 lower-EL trap proof is accepted.
  Before a stable SVC/syscall ABI can be contracted, Talos needed one
  source-backed inventory that distinguishes diagnostic proof markers from
  future user-program ABI behavior.
- Decision: Accept phase7-syscall-abi-source-inventory-20260529. The
  inventory maps lower-EL synchronous exception entry, diagnostic SVC proof
  surfaces, missing syscall number and argument-register contracts,
  return/error constraints from PosixError, user-copy preconditions,
  descriptor-table interaction, and process/task ownership.
- Evidence level: static source inspection, static documentation inspection,
  whitespace inspection, and documentation build. No QEMU or Pi 5 hardware
  evidence was claimed by this documentation-only task.
- Validation: git status --short before edits was clean; git diff --check
  passed; mdbook build passed.
- Consequences: The next bounded task should be
  phase7-syscall-abi-contract-20260529. It should define the minimal stable
  SVC ABI before any syscall implementation starts.
- Risks: Syscall dispatch, numeric errno values, process loading,
  copy-in/copy-out byte movement, descriptor I/O, VFS/filesystem, shell,
  networking, SSH, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver
  policy remain deferred.

## 2026-05-29 - Phase 7.2 EL0 Trap Proof Closeout Accepted

- Status: accepted as the documentation closeout for the bounded QEMU and
  physical Pi 5 lower-EL trap proof frontier. No Rust behavior, assembly
  behavior, QEMU rerun, Pi 5 archive publication, hardware-lock use, serial
  observation, syscall ABI, process loading, descriptor I/O, VFS/filesystem,
  shell behavior, networking, SSH, RP1/PCIe, UART interrupt ownership, or
  DMA/cache-driver policy was added.
- Context: The QEMU EL0 trap smoke core and serialized Pi 5 EL0 trap proof are
  both accepted. The repository needed one checkpoint that reconciles their
  retained evidence, states the exact capability proven, and prevents the
  diagnostic SVC proof marker from becoming an implied syscall ABI.
- Decision: Accept phase7-el0-trap-proof-closeout-checkpoint-20260529. The
  accepted frontier proves one bounded diagnostic lower-EL path: validated
  fixed user frame, EL0t entry, diagnostic SVC marker 0x7a10, regular
  lower-AArch64 synchronous trap handling, saved user state, final
  classification, and PASS on QEMU/substitute and physical Pi 5 hardware.
- Evidence level: static documentation inspection, retained QEMU/substitute
  serial evidence, retained serialized Pi 5 hardware boot/output evidence,
  whitespace inspection, and documentation build.
- Validation: git status --short before edits was clean; git diff --check
  passed; mdbook build passed.
- Consequences: The next recommended bounded task is
  phase7-syscall-abi-source-inventory-20260529. It should remain
  documentation-only and map source owners and gaps before any syscall ABI or
  dispatch implementation starts.
- Risks: General syscall ABI, syscall dispatch, process loading, descriptor
  I/O, VFS/filesystem, shell, networking, SSH, RP1/PCIe, UART interrupt
  ownership, and DMA/cache-driver policy remain deferred.

## 2026-05-28 - Phase 7.2 QEMU EL0 Trap Smoke Closeout Accepted

- Status: accepted as the documentation closeout for the first QEMU-only EL0
  trap smoke proof. No Rust behavior, assembly behavior, QEMU rerun, Pi 5
  hardware run, archive publishing, hardware-lock use, syscall ABI,
  VFS/filesystem, program loader, descriptor I/O, networking, SSH, shell,
  RP1/PCIe, UART interrupt ownership, or DMA/cache-driver policy was added.
- Context: The accepted QEMU core implementation had retained serial evidence
  for the planned diagnostic lower-AArch64 SVC marker path. A checkpoint was
  required to reconcile the proof frontier and prevent QEMU/substitute evidence
  from becoming a physical Pi 5 lower-EL claim.
- Decision: Accept
  phase7-qemu-el0-trap-smoke-closeout-checkpoint-20260528. The checkpoint
  records that retained QEMU evidence contains the saved-state line,
  classification=qemu-el0-trap-smoke-complete, and qemu-el0-trap-smoke: PASS
  for the fixed built-in EL0 payload and marker 0x7a10.
- Evidence level: static inspection, documentation build, whitespace
  inspection, and previously accepted QEMU/substitute serial evidence. No Pi 5
  hardware evidence was claimed.
- Validation: git status --short was clean before edits; git diff --check
  passed; mdbook build passed.
- Consequences: The next bounded task should be
  phase7-pi5-el0-trap-proof-plan-20260528. It should remain documentation-only
  and define hardwareTestLock ownership, candidate archive identity, fresh
  serial cursor, TFTP delta, known-good control, candidate rerun rules after
  inconclusive evidence, restoration, and retained evidence before any
  hardware action.
- Risks: Physical Pi 5 lower-EL behavior, general syscall ABI, process loading,
  copy-in/copy-out implementation, descriptor I/O, VFS/filesystem, shell,
  networking, SSH, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver
  policy remain deferred.

## 2026-05-28 - Phase 7.2 EL0 Address-Space Source Inventory Accepted

- Status: accepted as the Phase 7.2 EL0 trap path and user address-space
  source inventory. No Rust behavior, assembly behavior, boot scenario, QEMU
  run, Pi 5 hardware run, archive publishing, hardware-lock use, EL0 entry,
  SVC/syscall ABI, VFS, filesystem, program loader, descriptor I/O,
  networking, SSH, shell, RP1/PCIe, UART interrupt ownership, or
  DMA/cache-driver policy was added.
- Context: Phase 7.1 closed out target-independent POSIX path/error and
  descriptor-table semantics. Before lower-EL implementation could start, the
  accepted exception, memory, scheduler, POSIX, descriptor, and validation
  surfaces needed one source-backed inventory to prevent diagnostic and
  kernel-only paths from becoming userspace contracts.
- Decision: Accept phase7-el0-address-space-source-inventory-20260528. The
  accepted inventory maps exception vectors and saved frames, same-EL ERET
  diagnostics, EL2 translation setup, page-frame ownership, scheduler
  task/process separation, PosixError/EFAULT vocabulary, descriptor-table
  ownership, retained gates, diagnostic-only surfaces, and Phase 7.2
  implementation gaps.
- Evidence level: static inspection, documentation build, and whitespace
  inspection. No QEMU or Pi 5 hardware evidence was claimed.
- Validation: git status --short was clean before edits; git diff --check
  passed; mdbook build passed.
- Consequences: The next bounded task should be
  phase7-el0-trap-address-space-contract-20260528. It should define
  address-space invariants, lower-EL trap/return invariants, user fault
  classes, copy-in/copy-out preconditions, and evidence levels before
  implementation starts.
- Risks: EL0 entry, user page tables, trap return, user stacks, syscall ABI,
  copy-in/copy-out, VFS/filesystem, descriptor I/O, program loading, shell,
  networking, SSH, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver
  policy remain deferred.

## 2026-05-28 - Phase 7.2 EL0 Trap and Address-Space Contract Accepted

- Status: accepted as the documentation-only Phase 7.2 EL0 trap-return and
  user address-space contract. No Rust behavior, assembly behavior, boot
  scenario, QEMU run, Pi 5 hardware run, archive publishing, hardware-lock
  use, EL0 entry, SVC/syscall numeric ABI, VFS, filesystem, program loader,
  descriptor I/O, networking, SSH, shell, RP1/PCIe, UART interrupt ownership,
  or DMA/cache-driver policy was added.
- Context: The accepted Phase 7.2 source inventory named the exception,
  memory, scheduler, POSIX, descriptor, and validation surfaces that constrain
  lower-EL work. A contract was needed before implementation could add even
  target-independent user-memory validation primitives.
- Decision: Accept phase7-el0-trap-address-space-contract-20260528. The
  contract defines the first canonical user range below
  0x0000_8000_0000_0000, null guard, user text/data/heap/stack/guard
  vocabulary, kernel-only mapping policy while a user task runs, lower-EL
  trap/return frame requirements, user fault classes, copy-in/copy-out
  preconditions, evidence levels, and blocked surfaces.
- Evidence level: static inspection, documentation build, and whitespace
  inspection. No QEMU or Pi 5 hardware evidence was claimed.
- Validation: git status --short was clean before edits; git diff --check
  passed; mdbook build passed.
- Consequences: The next bounded implementation task should be
  phase7-user-memory-permission-core-20260528. It may implement only
  target-independent user virtual range, mapping permission, access kind, and
  copy-boundary validation primitives matching the contract.
- Risks: EL0 entry, trap-return assembly, TTBR/TCR/SCTLR changes, actual page
  table switching, syscall ABI, process tables, VFS/filesystem, descriptor
  I/O, program loading, shell, networking, SSH, RP1/PCIe, UART interrupt
  ownership, DMA/cache-driver policy, QEMU proof, and Pi 5 hardware proof
  remain deferred.

## 2026-05-28 - Phase 7.1 POSIX Baseline Closeout Accepted

- Status: accepted as the Phase 7.1 POSIX baseline closeout checkpoint. No new
  runtime behavior, boot scenario, QEMU run, Pi 5 hardware run, EL0 entry,
  SVC/syscall ABI, VFS, filesystem, program loader, descriptor I/O,
  networking, SSH, shell, RP1/PCIe, UART interrupt ownership, or
  DMA/cache-driver policy was added by the checkpoint.
- Context: Talos had accepted the Phase 7 POSIX contract source inventory, the
  POSIX contract baseline, the target-independent path/error model core, the
  descriptor-table contract, and the target-independent descriptor-table core.
  A closeout checkpoint was required before any lower-EL, syscall, VFS,
  filesystem, loader, descriptor I/O, or shell task could be planned.
- Decision: Accept phase7-posix-baseline-closeout-checkpoint-20260528. The
  accepted Phase 7.1 boundary is target-independent: errno/path primitives and
  descriptor-table data-model semantics are tested, while runtime I/O,
  process tables, EL0, syscall ABI, VFS/filesystem, program loading, shell,
  networking, and hardware claims remain deferred.
- Evidence level: static inspection, documentation build, whitespace
  inspection, and target-independent no_std unit tests. No QEMU or Pi 5
  hardware evidence was claimed.
- Validation: git status --short was clean before edits; git diff --check,
  mdbook build, and cargo -Zjson-target-spec test passed.
- Consequences: The next recommended supervisor-planned task is a Phase 7.2
  EL0 trap path and user address-space source inventory. It should reconcile
  lower-EL readiness, exception-vector constraints, memory permissions,
  task/process separation, PosixError return vocabulary, and descriptor-table
  ownership before implementation.
- Risks: PID allocation, process tables, lower-EL execution, SVC/syscalls,
  copy-in/copy-out, descriptor I/O, VFS/filesystem, program loading,
  scheduler blocking I/O, filesystem-backed commands, local shell, networking,
  SSH, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy remain
  deferred.

## 2026-05-28 - Phase 7.1 POSIX Contract Baseline Accepted

- Status: accepted as the documentation-only Phase 7.1 POSIX baseline
  contract. No Rust implementation, boot scenario, QEMU run, Pi 5 hardware
  run, EL0 entry, SVC/syscall ABI, descriptor table, VFS, filesystem, program
  loader, networking, SSH, shell, RP1/PCIe, UART interrupt ownership, or
  DMA/cache behavior was added.
- Context: The accepted Phase 7 source inventory established the scheduler
  task/process boundary, runtime-console/TTY stdio direction, diagnostic
  command limitations, lower-EL readiness limits, and retained gates. The next
  contract needed stable names and edge cases before implementation tasks could
  add path, errno, or descriptor cores.
- Decision: Accept phase7-posix-contract-baseline-20260528. The baseline
  defines errno-style names, lexical path normalization semantics, process
  lifetime vocabulary, descriptor operation vocabulary, stdio inheritance
  through descriptor-owned handles, early loader/argument/environment
  vocabulary, and target-independent test seams.
- Evidence level: static inspection, documentation build, and whitespace
  inspection. Hardware and QEMU reruns were not required because this task
  changes only documentation and durable state.
- Validation: git status --short was clean before edits; git diff --check
  passed; mdbook build passed.
- Consequences: The next bounded implementation task should be
  phase7-path-error-model-core-20260528. Milestone 7.1 is not complete until
  target-independent tests for path normalization and descriptor-table edge
  cases are accepted.
- Risks: Descriptor table implementation, VFS/filesystem, process tables, EL0,
  SVC/syscalls, program loading, scheduler blocking I/O, filesystem-backed
  commands, local shell, networking, SSH, RP1/PCIe, UART interrupt ownership,
  and DMA/cache-driver policy remain deferred.

## 2026-05-28 - Phase 7.1 POSIX Contract Source Inventory Accepted

- Status: accepted as documentation/source inventory before the first Phase
  7.1 POSIX baseline contract. No Rust implementation, boot scenario, QEMU
  run, Pi 5 hardware run, EL0 entry, SVC/syscall ABI, descriptor table, VFS,
  filesystem, program loader, networking, SSH, shell, RP1/PCIe, UART interrupt
  ownership, or DMA/cache behavior was added.
- Context: The Phase 6.3 production scheduler runtime closeout accepted the
  first normal timer IRQ recording and owner-local service boundary. Before
  Phase 7 implementation, the accepted scheduler, console/TTY, diagnostic
  command, and lower-EL readiness surfaces needed one source-backed inventory
  to prevent diagnostic shortcuts from becoming POSIX interfaces.
- Decision: Accept phase7-posix-contract-source-inventory-20260528. The
  accepted constraints are scheduler task/process separation, runtime-console0
  and TTY as future descriptor-facing stdio backing surfaces, the diagnostic
  command channel as kernel-owned and not a shell/syscall/program-loader path,
  the EL2 identity map as not a userspace isolation contract, and the retained
  Phase 4 through Phase 6 validation gates.
- Evidence level: static inspection, documentation build, and whitespace
  inspection. Hardware and QEMU reruns were not required because this task
  changes only documentation and durable state.
- Validation: git status --short was clean before edits; git diff --check
  passed; mdbook build passed.
- Consequences: The next bounded task should be
  phase7-posix-contract-baseline-20260528. It should define the first
  errno/error, path, process lifetime, descriptor operation, stdio inheritance,
  and early loader/argument/environment vocabulary before any implementation
  task starts.
- Risks: PID allocation, process tables, address spaces, EL0, SVC/syscalls,
  descriptor tables, VFS/filesystem, program loading, scheduler blocking I/O,
  filesystem-backed commands, local shell, networking, SSH, RP1/PCIe, UART
  interrupt ownership, and DMA/cache-driver policy remain deferred.

## 2026-05-28 - Phase 6.3 Production Scheduler Runtime Source Inventory Accepted

- Status: accepted as documentation/source inventory before production
  timer/preemption runtime integration. No Rust behavior, boot image, QEMU
  run, Pi 5 hardware run, direct IRQ/IPI-context scheduling, remote
  current-task switching, running-task migration, autonomous work stealing,
  Phase 7, filesystem, networking, SSH, shell, RP1/PCIe, UART interrupt
  ownership, or DMA/cache behavior was added.
- Context: The multi-core preemption closeout accepted an owner-local
  diagnostic primitive and required a bounded productionization task before
  further scheduler runtime integration. The accepted proof surfaces needed to
  be separated from normal boot, timer, and owner-local runtime entry points
  before a contract could safely name implementation scope.
- Decision: Accept
  phase6-production-scheduler-runtime-source-inventory-20260528. The normal
  timer handlers still acknowledge/classify/rearm/EOI and record only older
  diagnostic counters under retained timer-preemption scenarios; they do not
  yet route timer IRQs into durable `PerCorePreemptionState`. The accepted
  multi-core preemption QEMU and Pi 5 proofs remain proof-only surfaces that
  construct scenario-local scheduler/preemption/metadata objects and call the
  record/service APIs directly from diagnostic flow.
- Evidence level: static inspection, documentation build, and whitespace
  inspection. Hardware and QEMU reruns were not required because this task
  changes only documentation and durable state.
- Validation: git status --short was clean before edits; git diff --check
  passed; mdbook build passed.
- Consequences: The next bounded task should be
  phase6-production-timer-preemption-contract-20260528. That contract must
  name the normal timer IRQ recording path, owner-local post-IRQ service
  point, current-task source of truth, per-CPU runtime objects, secondary
  runtime limits, retained gates, and deterministic disabled/stale/wrong-owner
  outcomes before any Rust implementation starts.
- Risks: Production timer integration, non-diagnostic secondary runtime
  roles, interrupt-driven remote reschedule, running-task migration, remote
  current-task switching, work stealing, Phase 7, filesystem, networking, SSH,
  shell behavior, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver
  policy remain deferred.

## 2026-05-28 - Phase 6.3 Multi-Core Preemption Closeout Accepted

- Status: accepted as the checkpoint for the Phase 6.3 multi-core preemption
  slice. No Rust implementation, boot image, QEMU run, hardware run, direct
  IRQ/IPI-context scheduling, remote current-task switching, running-task
  migration, autonomous work stealing, Phase 7, filesystem, networking, SSH,
  shell, RP1/PCIe, UART interrupt ownership, or DMA behavior was added.
- Context: Talos had accepted the multi-core preemption source inventory,
  contract, target-independent core, QEMU substitute proof, and serialized Pi
  5 proof. A closeout checkpoint was required before any later scheduler
  productionization or phase transition.
- Decision: Accept phase6-multicore-preemption-closeout-checkpoint-20260527.
  The accepted boundary is diagnostic and owner-local: multiple owners can
  record bounded local timer-preemption intent, prove record-only paths do not
  mutate scheduler queues or current tasks, and service the request from the
  owning CPU's normal scheduler control flow. Retained gates preserve the
  scheduler unit tests, QEMU timer-preemption, secondary service-loop,
  shared-runqueue, load-balancing, and multi-core preemption smokes, plus the
  Pi 5 multi-core preemption proof scripts for explicit future hardware tasks.
- Evidence level: static inspection of accepted task/evidence records,
  documentation build, and whitespace inspection. Hardware was not required
  because this checkpoint makes no new physical claim.
- Validation: git diff --check and mdbook build passed.
- Consequences: Further scheduler productionization or Phase 7 work requires a
  new supervisor-planned bounded task with explicit scope, dependencies,
  acceptance criteria, validation gates, documentation requirements, and
  evidence requirements.
- Risks: Production timer integration, interrupt-driven remote reschedule,
  running-task migration, remote current-task switching, work stealing,
  non-diagnostic secondary runtime roles, userspace, descriptors, filesystem,
  networking, SSH, shell behavior, RP1/PCIe, UART interrupt ownership, and
  DMA/cache-driver policy remain deferred.

## 2026-05-27 - Phase 6.3 Multi-Core Preemption Source Inventory Accepted

- Status: accepted as documentation/source inventory before any multi-core
  preemption contract or implementation. No Rust behavior change, boot image,
  QEMU run, hardware run, remote reschedule implementation, work stealing,
  running-task migration, Phase 7, filesystem, networking, SSH, shell,
  RP1/PCIe, UART interrupt ownership, or DMA behavior was added.
- Context: The accepted load-balancing closeout made multi-core preemption
  source inventory the next bounded Phase 6.3 task. The existing timer,
  scheduler, secondary service-loop, IPI/wake, metadata, SharedRunQueue, and
  load-balancing surfaces needed one source-backed reconciliation before the
  contract.
- Decision: Accept phase6-multicore-preemption-source-inventory-20260527. The
  current model remains CPU-local: timer/IPI handlers record bounded state,
  while owner-local normal control flow mutates scheduler queues and current
  tasks. Cross-core mechanisms remain notification, target-owned wake,
  owner-published metadata, or explicit SharedRunQueue owner-transfer
  surfaces, not remote scheduler execution.
- Evidence level: static inspection, documentation build, and whitespace
  inspection. Hardware was not required because no physical claim changed.
- Validation: git status --short was clean before edits; git diff --check
  passed; mdbook build passed.
- Consequences: The next bounded task should be
  phase6-multicore-preemption-contract-20260527. That contract must define
  current-task authority, preemption-disable behavior, IRQ/IPI context
  boundaries, lock ordering, stale metadata outcomes, remote-reschedule
  deferral or notification-only behavior, and proof obligations before any
  implementation starts.

## 2026-05-27 - Phase 6.3 Load-Balancing Closeout Accepted

- Status: accepted as the checkpoint for the Phase 6.3 load-balancing slice.
  No Rust implementation, boot image, QEMU run, hardware run, autonomous work
  stealing, running-task migration, remote reschedule, multi-core preemption,
  Phase 7, filesystem, networking, SSH, shell, RP1/PCIe, UART interrupt
  ownership, or DMA behavior was added.
- Context: Talos had accepted the load-balancing source inventory, policy
  contract, target-independent core, QEMU substitute proof, and serialized
  Pi 5 proof. A closeout checkpoint was required before broader scheduler
  topology or later-phase work.
- Decision: Accept phase6-load-balancing-closeout-checkpoint-20260527. The
  accepted boundary is a deterministic policy primitive: select one
  source-local front runnable task, publish through SharedRunQueue, consume on
  the destination owner, and refresh metadata. The retained gates are the
  scheduler unit tests, shared run-queue migration QEMU smoke,
  qemu-load-balancing smoke, and Pi 5 load-balancing reproduction scripts for
  explicit future hardware tasks.
- Evidence level: static inspection of accepted task/evidence records,
  documentation build, and whitespace inspection. Hardware was not required
  because this checkpoint makes no new physical claim.
- Validation: `git diff --check` and `mdbook build` passed.
- Consequences: The next bounded recommendation is
  phase6-multicore-preemption-source-inventory-20260527, a documentation and
  source-inventory task before any multi-core preemption implementation.
  Autonomous balancing loops, work stealing, running-task migration, remote
  reschedule, multi-core timer preemption, Phase 7, filesystem, networking,
  SSH, shell behavior, RP1/PCIe, UART interrupt ownership, and DMA/cache
  policy remain deferred.

## 2026-05-27 - Phase 6.3 Pi 5 Load-Balancing Evidence Accepted

- Status: accepted as serialized physical Pi 5 evidence for the accepted
  load-balancing core. No autonomous work stealing, running-task migration,
  remote reschedule, multi-core preemption, Phase 7, filesystem, networking,
  SSH, shell, RP1/PCIe, UART interrupt ownership, or DMA behavior was added.
- Context: The target-independent LoadBalancingPolicy core and QEMU substitute
  proof were accepted, but physical acceptance required hardwareTestLock
  serialization, candidate identity, TFTP fetch evidence, cursor-valid serial
  output, classification, and restore proof.
- Decision: Accept phase6-pi5-load-balancing-proof-20260527. The focused Pi 5
  diagnostic adds rpi5_load_balancing_proof and exercises
  LoadBalancingPolicy::plan_front_runnable,
  LoadBalancingPolicy::publish_front_runnable, and
  SharedRunQueue::consume_for_destination on the physical Pi 5 boot path.
- Evidence level: serialized Pi 5 hardware boot/output, TFTP fetch evidence,
  archive/kernel digest inspection, QEMU/substitute preservation gates,
  fmt/lint/typecheck, no_std unit tests, mdBook validation, and whitespace
  inspection.
- Validation: local1 used archive SHA256
  e7d4c80740bac203e9516e68baef29e9d197a8e760d233301cb209605a38d119 and
  kernel SHA256
  ceb75685864c32ed3d5a028c877d6a1d911892d4cbf14b36536d266206d7fecd;
  cursor-valid serial reached classification=pi5-load-balancing-complete and
  PASS; restore returned ok=true with restore-exit.txt equal to 0. Full
  acceptance validation also includes cargo fmt --all -- --check,
  cargo -Zjson-target-spec test, scripts/qemu-shared-runqueue-migration-smoke.sh,
  scripts/qemu-load-balancing-smoke.sh, git diff --check, and mdbook build.
- Risks: This is a bounded diagnostic proof of the deterministic
  load-balancing policy path. It does not accept autonomous balancing loops,
  work stealing, running-task migration, remote scheduler execution in IPI
  context, multi-core timer preemption, userspace, descriptors, filesystem,
  networking, SSH, shell behavior, RP1/PCIe, UART interrupt ownership, or
  DMA/cache policy.

## 2026-05-27 - Phase 6.3 QEMU Load-Balancing Smoke Accepted

- Status: accepted as QEMU substitute evidence for the target-independent
  load-balancing core. No Pi 5 hardware claim, work stealing, running-task
  migration, interrupt-driven remote reschedule, multi-core preemption, Phase
  7, filesystem, networking, SSH, shell, RP1/PCIe, UART interrupt ownership,
  or DMA behavior was added.
- Context: The accepted load-balancing core provides deterministic
  front-runnable selection and publication through `SharedRunQueue`, but it
  still needed a dependency-gated QEMU proof showing the implemented policy
  path rather than the lower-level migration API alone.
- Decision: Accept phase6-qemu-load-balancing-smoke-20260527. The
  `qemu_load_balancing_smoke` boot scenario and
  `scripts/qemu-load-balancing-smoke.sh` prove
  `LoadBalancingPolicy::plan_front_runnable`,
  `LoadBalancingPolicy::publish_front_runnable`, and
  `SharedRunQueue::consume_for_destination` as one deterministic handoff:
  source owner 0 selects task 109 for destination owner 1, removes it from the
  source queue, publishes it through the shared queue, destination owner 1
  enqueues it locally, and metadata owner/generation refresh.
- Evidence level: static inspection, QEMU substitute transcript, fmt/lint,
  unit tests, preserved QEMU smoke/regression gates, whitespace inspection,
  and documentation build. Hardware was not required because this task makes
  no physical claim.
- Validation: `scripts/qemu-load-balancing-smoke.sh`,
  `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test`,
  `scripts/qemu-smoke.sh`, `scripts/qemu-shared-runqueue-migration-smoke.sh`,
  `scripts/qemu-secondary-scheduler-service-loop-smoke.sh`,
  `git diff --check`, and `mdbook build` passed.
- Consequences: The next bounded task may be the serialized Pi 5
  load-balancing proof after supervisor ready-marking and hardware lock
  availability. Physical behavior remains unclaimed until that task is
  accepted or explicitly deferred.
- Alternatives considered: Reuse only the existing shared run-queue migration
  smoke, combine QEMU and Pi 5 proof in one task, or add work-stealing loops.
  The existing migration smoke bypasses the policy selection API; combining
  QEMU and hardware proof would blur the dependency gate; work stealing is a
  later policy layer and unnecessary for the first proof.

## 2026-05-27 - Phase 6.3 Load-Balancing Core Accepted

- Status: accepted as a target-independent Rust scheduler policy core with
  unit-test evidence. No QEMU run, Pi 5 hardware run, autonomous work
  stealing, running-task migration, interrupt-driven remote reschedule,
  multi-core preemption, Phase 7, filesystem, networking, SSH, shell, RP1/PCIe,
  UART interrupt ownership, or DMA behavior was added.
- Context: The accepted load-balancing policy contract permits only a
  conservative source-owner policy that selects one locally runnable,
  non-current task and one eligible destination, then uses the accepted
  SharedRunQueue owner-transfer path. The existing scheduler already has
  owner-local runnable queues, owner-published metadata generations, CPU
  roles, and SharedRunQueue capacity/backpressure checks.
- Decision: Accept phase6-load-balancing-core-20260527. The core adds
  `LoadBalancingPolicy`, `LoadBalancingPlan`,
  `LoadBalancingPublishReport`, and `LoadBalancingPolicyError` in
  `src/scheduler.rs`. Planning chooses the source-local front runnable task,
  rejects invalid or deferred destinations, records the metadata generation,
  and checks destination/shared queue backpressure. Publication calls
  `SharedRunQueue::publish_migration` and relies on that accepted mechanism
  for stale-generation rejection and source-local removal.
- Evidence level: static inspection, fmt/lint, unit tests, QEMU substitute
  smoke/regression gates, whitespace inspection, and documentation build.
  Hardware was not required because this task makes no physical claim.
- Validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test`,
  `scripts/qemu-smoke.sh`,
  `scripts/qemu-secondary-scheduler-service-loop-smoke.sh`,
  `scripts/qemu-shared-runqueue-migration-smoke.sh`, `git diff --check`,
  and `mdbook build` passed.
- Consequences: The next bounded task may be a focused QEMU load-balancing
  smoke that proves the policy selecting a destination and publishing through
  SharedRunQueue. Pi 5 proof, autonomous work stealing, running-task
  migration, interrupt-driven remote reschedule, multi-core preemption, Phase
  7, filesystem, networking, SSH, shell behavior, RP1/PCIe, UART interrupt
  ownership, and DMA/cache-driver policy remain deferred.
- Alternatives considered: Implement destination consumption or proof routing
  in the same task, add fairness/affinity metadata first, or add remote
  reschedule. Combining proof routing would blur the dependency-gated QEMU/Pi
  5 tasks; fairness/affinity still lacks accepted data structures; remote
  reschedule is optional and unnecessary for the polling-first policy core.

## 2026-05-27 - Phase 6.3 Load-Balancing Policy Contract Accepted

- Status: accepted as documentation/policy contract before load-balancing
  implementation. No Rust implementation, QEMU run, Pi 5 hardware run,
  load-balancer, work stealing, running-task migration, remote reschedule,
  multi-core preemption, Phase 7, filesystem, networking, SSH, shell, RP1/PCIe,
  UART interrupt ownership, or DMA behavior was added.
- Context: The accepted load-balancing source inventory identified usable
  scheduler metadata, owner-local runnable pressure, CPU roles,
  SharedRunQueue capacity, wake/timer context, and stale/invalid input failure
  modes. It also showed missing affinity, fairness, production secondary
  idle/wake, remote-reschedule, and multi-core-preemption policy.
- Decision: Accept phase6-load-balancing-policy-contract-20260527. The
  contract permits a conservative deterministic policy to choose one
  source-owned runnable, non-current task and one eligible destination CPU,
  then use the accepted SharedRunQueue owner-transfer mechanism. It keeps
  RemoteWakeQueue separate, makes remote reschedule optional and polling-only
  for the first implementation, and requires deterministic defer/reject
  outcomes for stale metadata, full queues, invalid roles, duplicates, running
  tasks, blocked tasks, and unsupported cross-core trigger paths.
- Evidence level: static inspection, documentation build, and whitespace
  inspection. Hardware was not required because no physical claim changed.
- Validation: git status --short was clean before edits; git diff --check
  passed; mdbook build passed.
- Consequences: The next bounded Phase 6.3 task may implement
  phase6-load-balancing-core-20260527 inside this contract. Work stealing,
  running-task migration, interrupt-driven remote reschedule, multi-core
  preemption, Phase 7, filesystem, networking, SSH, shell behavior, RP1/PCIe,
  UART interrupt ownership, and DMA/cache-driver policy remain deferred.
- Alternatives considered: Implement load balancing directly from the source
  inventory, design full fairness/affinity first, or add interrupt-driven
  remote reschedule before policy. Direct implementation would blur policy and
  mechanism; full fairness/affinity lacks data structures; remote reschedule
  is unnecessary for the first owner-local polling implementation and risks
  running scheduler work in IPI context.

## 2026-05-27 - Shared Run-Queue Migration Closeout Accepted

- Status: accepted as the Phase 6.3 shared run-queue/migration closeout
  checkpoint. No Rust implementation, boot image, QEMU run, Pi 5 hardware run,
  load balancing, work stealing, multi-core timer preemption, Phase 7,
  filesystem, networking, SSH, shell, RP1/PCIe, UART interrupt ownership, or
  DMA behavior was added by the checkpoint.
- Context: The shared run-queue/migration source inventory, contract,
  target-independent core, QEMU substitute proof, and serialized Pi 5 proof are
  accepted. The Pi 5 proof reported all four physical-core participants
  completing the implemented owner-transfer invariant with
  classification=pi5-shared-runqueue-migration-complete and PASS.
- Decision: Accept
  docs/src/project/phase6-shared-runqueue-migration-closeout-checkpoint.md and
  tasks/2026-05-26-phase6-shared-runqueue-migration-closeout-checkpoint.md.
  The next bounded Phase 6.3 task should be
  phase6-load-balancing-source-inventory-20260527 before any load-balancing
  implementation.
- Evidence level: static review of accepted task records, QEMU substitute
  evidence, serialized Pi 5 evidence summary, scheduler architecture docs,
  roadmap, and decision log.
- Validation: git status --short was clean before edits, git diff --check
  passed, and mdbook build passed. Rust fmt/tests, QEMU reruns, and hardware
  runs were not required because this checkpoint changed only Markdown
  documentation and durable task state.
- Consequences: Talos now has accepted QEMU and physical Pi 5 evidence for the
  named shared run-queue/migration invariant, but only as a bounded
  owner-transfer mechanism. Load balancing, work stealing, target selection,
  running-task migration, remote reschedule, multi-core preemption, Phase 7,
  filesystem, networking, SSH, shell behavior, RP1/PCIe, UART interrupt
  ownership, and DMA/cache-driver policy remain deferred.
- Alternatives considered: Start load-balancing implementation directly,
  choose multi-core preemption inventory next, or spend the next task on
  diagnostic cleanup. Direct implementation would skip the policy inventory
  needed to use SharedRunQueue safely; multi-core preemption still depends on
  target-selection and runnable-placement policy; cleanup can remain bounded
  under the diagnostic-surface policy unless it blocks the next inventory.

## 2026-05-26 - Shared Run-Queue Core Accepted

- Status: accepted as the first Phase 6.3 target-independent shared
  run-queue/migration implementation core. No boot image, new diagnostic
  scenario, Pi 5 hardware claim, load balancing, work stealing, multi-core
  timer preemption, Phase 7, filesystem, networking, SSH, shell, RP1/PCIe,
  UART interrupt ownership, or DMA behavior was added.
- Context: The accepted shared run-queue/migration contract required a
  bounded owner-transfer surface that keeps local runnable queues CPU-owned,
  separates remote wake from remote enqueue/migration, uses the accepted SMP
  lock boundary, and reports deterministic failure outcomes before any QEMU
  or physical proof tasks.
- Decision: Add `SharedRunQueue`, `SharedRunQueueEntry`, `MigrationState`,
  `SharedRunQueueError`, and `SharedRunQueueLock` in `src/scheduler.rs`.
  `publish_migration` removes a runnable task from the source-local queue and
  publishes a complete shared handoff after fresh metadata checks;
  `consume_for_destination` lets an accepted production-capable destination
  owner enqueue the task locally and transfer metadata ownership.
- Evidence level: unit tests and static inspection of the target-independent
  scheduler core, plus QEMU/substitute preservation gates.
- Validation: `cargo fmt --all -- --check` passed,
  `cargo -Zjson-target-spec test` passed with 142 no_std tests,
  `scripts/qemu-smoke.sh` passed, the focused existing Phase 6 gate
  `scripts/qemu-secondary-scheduler-service-loop-smoke.sh` passed,
  `git diff --check` passed, and `mdbook build` passed.
- Consequences: The next bounded task may add a focused QEMU shared
  run-queue/migration smoke that proves the implemented core without a bypass.
  Pi 5 proof, load balancing, work stealing, multi-core preemption, Phase 7,
  filesystem, networking, SSH, shell behavior, RP1/PCIe, UART interrupt
  ownership, and DMA/cache-driver policy remain deferred.
- Alternatives considered: Reuse `RemoteWakeQueue`, make metadata the
  ownership-transfer authority, or implement load-balancing policy at the same
  time. Reusing remote wake would blur wake and migration semantics; metadata
  is observational; policy work would exceed the accepted core task.

## 2026-05-26 - Shared Run-Queue and Migration Contract Accepted

- Status: accepted as a Phase 6.3 scheduler-topology contract. No Rust
  implementation, boot image, QEMU claim, Pi 5 hardware claim, load balancing,
  work stealing, multi-core timer preemption, Phase 7, filesystem, networking,
  SSH, shell, RP1/PCIe, UART interrupt ownership, or DMA behavior was added.
- Context: The accepted source inventory showed owner-local runnable queues,
  target-owned remote wake mailboxes, owner-published metadata, accepted SMP
  locks, and diagnostic proof routing, but no accepted shared run queue or
  ownership-transfer contract.
- Decision: Accept
  docs/src/project/phase6-shared-runqueue-migration-contract.md and the
  corresponding scheduler architecture update. The first shared topology must
  keep task mutation single-owner, separate remote wake from remote enqueue,
  use local-IRQ-then-SMP-lock ordering, publish complete shared entries through
  acquire/release lock boundaries, and report deterministic migration failure
  outcomes.
- Evidence level: static review against the accepted source inventory,
  scheduler architecture, src/scheduler.rs, src/smp_sync.rs, roadmap, and
  decision log.
- Validation: git status --short was clean before edits, git diff --check
  passed, and mdbook build passed. Rust fmt/tests, QEMU reruns, and hardware
  runs were not required because this task changed only Markdown documentation
  and durable task state.
- Consequences: The next bounded implementation may add a shared run-queue
  core only if it stays inside this contract. Load balancing, work stealing,
  migration of running tasks, multi-core preemption, Phase 7, filesystem,
  networking, SSH, shell behavior, RP1/PCIe, UART interrupt ownership, and
  DMA/cache-driver policy remain deferred.
- Alternatives considered: Start implementation directly from the source
  inventory, reuse RemoteWakeQueue as a migration queue, or make metadata the
  mutation authority. Direct implementation would leave lock and rollback
  rules ambiguous; reusing remote wake would blur wake versus enqueue
  semantics; metadata is observational and should not become hidden scheduler
  authority.

## 2026-05-26 - Shared Run-Queue and Migration Source Inventory Accepted

- Status: accepted as a Phase 6.3 source inventory. No Rust implementation,
  boot image, hardware run, shared run queue, remote enqueue, task migration,
  load balancing, multi-core preemption, Phase 7, filesystem, networking,
  SSH, shell, RP1/PCIe, UART interrupt ownership, or DMA behavior was added.
- Context: The secondary scheduler service-loop closeout is accepted. Talos
  has owner-local runnable queues, target-owned remote wake mailboxes,
  CPU-local service sequencing, owner-published metadata, accepted SMP lock
  primitives, and retained QEMU/Pi 5 diagnostic gates, but no shared scheduler
  topology.
- Decision: Accept
  docs/src/project/phase6-shared-runqueue-migration-source-inventory.md and
  tasks/2026-05-26-phase6-shared-runqueue-migration-source-inventory.md. The
  next bounded task should be
  phase6-shared-runqueue-migration-contract-20260526 before any shared
  topology implementation.
- Evidence level: static source/doc inventory across scheduler, SMP, SMP sync,
  target proof routing, retained scripts, accepted task records, accepted
  evidence summaries, roadmap, and decision log.
- Validation: git status --short was clean before edits, git diff --check
  passed, and mdbook build passed. Rust fmt/tests, QEMU reruns, and hardware
  runs were not required because this task changed only Markdown documentation
  and durable task state.
- Consequences: The next design boundary is contract-first: global task
  registry authority, shared run-queue structure, migration state machine,
  remote enqueue/reschedule semantics, lock ordering, load-balancing inputs,
  secondary production role, and validation strategy must be written down
  before implementation. Shared run queue implementation, migration
  implementation, load balancing, multi-core preemption, Phase 7, filesystem,
  networking, SSH, shell behavior, RP1/PCIe, UART interrupt ownership, and
  DMA/cache-driver policy remain deferred.
- Alternatives considered: Start shared run-queue implementation immediately,
  extend diagnostic service-loop proofs, or run cleanup first. The inventory
  showed implementation would be premature without a contract for ownership
  transfer, lock ordering, and remote enqueue semantics. Cleanup remains
  queued or blocked under separate policy.

## 2026-05-26 - Secondary Scheduler Service Loop Closeout Accepted

- Status: accepted as the Phase 6.3 secondary scheduler service-loop closeout
  checkpoint. No Rust implementation, boot image, hardware run, shared run
  queue, remote enqueue, task migration, load balancing, multi-core
  preemption, Phase 7, filesystem, networking, SSH, shell, RP1/PCIe, UART
  interrupt ownership, or DMA behavior was added by the checkpoint.
- Context: The secondary scheduler service-loop source inventory,
  SecondarySchedulerServiceLoop implementation, QEMU substitute smoke, and
  serialized Pi 5 proof are accepted. They prove one owner-local secondary
  service cycle after accepted handoff state while preserving interrupt
  hot-path separation and diagnostic-surface boundaries.
- Decision: Accept
  docs/src/project/phase6-secondary-scheduler-service-loop-closeout-checkpoint.md
  and tasks/2026-05-26-phase6-secondary-scheduler-service-loop-closeout-checkpoint.md.
  The next bounded task should be
  phase6-shared-runqueue-migration-source-inventory-20260526, a source
  inventory only, before any shared topology implementation.
- Evidence level: static reconciliation of accepted service-loop source
  inventory, core, QEMU smoke, Pi 5 proof records/evidence, scheduler
  architecture docs, roadmap, and decision log, plus mdBook validation and
  whitespace inspection.
- Validation: git status --short was clean before edits, git diff --check
  passed, and mdbook build passed. Rust fmt/tests, QEMU smoke reruns, and
  hardware runs were not required because this checkpoint changed only
  Markdown documentation and durable task state.
- Consequences: Talos has closed the secondary service-loop productionization
  slice and may plan shared run-queue/migration source inventory next. Shared
  run queue implementation, task migration implementation, load balancing,
  multi-core preemption, Phase 7, filesystem, networking, SSH, shell behavior,
  RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy remain
  deferred.
- Alternatives considered: Start shared topology implementation immediately,
  add more CPU-local service productionization, or prioritize queued cleanup.
  A source inventory is the next conservative boundary because it can reconcile
  CPU-local queues, remote wake ownership, metadata, secondary service-loop
  entry, and IPI/timer recording before implementation. Existing cleanup
  follow-ups remain queued or blocked under their own policy.

## 2026-05-26 - Secondary Scheduler Service Loop Source Inventory Accepted

- Status: accepted as a Phase 6.3 documentation/source-inventory contract. No
  Rust implementation, boot image, hardware run, shared run queue, remote
  enqueue, task migration, load balancing, multi-core preemption, Phase 7,
  filesystem, networking, SSH, shell, RP1/PCIe, UART interrupt ownership, or
  DMA behavior was added.
- Context: The accepted `CpuLocalSchedulerService` core can sequence one
  owning CPU's remote wake drain, local runnable transition,
  timer-preemption request, CPU-local dispatch, and metadata refresh. Before
  implementing the next productionization slice, Talos needs a source-backed
  contract for how secondary CPUs enter a normal service loop from accepted
  handoff state without turning diagnostics into shared scheduler topology.
- Decision: Accept
  docs/src/project/phase6-secondary-scheduler-service-loop-source-inventory.md.
  The secondary service-loop boundary starts after logical CPU identity, stack
  state, and normal control flow are established. Each loop iteration remains
  owner-local, calls the accepted CPU-local scheduler service from normal
  control flow, keeps IPI/timer paths as bounded recorders, dispatches only
  through the owner `PerCoreScheduler`, and preserves
  `SecondaryProductionDiagnostic` as the only accepted secondary production
  role. The next bounded task should be
  phase6-secondary-scheduler-service-loop-core-20260526.
- Evidence level: static inspection, static source/doc review of SMP,
  scheduler, SMP sync, retained QEMU/Pi 5 proof scripts, roadmap, decision
  log, and accepted CPU-local service records, plus mdBook validation and
  whitespace inspection.
- Validation: git status --short was clean before edits, git diff --check
  passed, and mdbook build passed. Rust fmt/tests and hardware runs were not
  required because this task changed only Markdown documentation and durable
  task state.
- Consequences: Talos has a bounded implementation target for a secondary
  owner-local service loop. Shared run queues, remote enqueue queues,
  migration, load balancing, work stealing, multi-core preemption, Phase 7,
  filesystem, networking, SSH, shell behavior, RP1/PCIe, UART interrupt
  ownership, and DMA/cache-driver policy remain deferred.
- Alternatives considered: Implement the loop directly from the closeout,
  rename `SecondaryProductionDiagnostic` into a general runtime role, or skip
  to shared topology. A source inventory keeps the next implementation narrow;
  renaming the role would overstate the accepted evidence; shared topology
  remains out of scope.

## 2026-05-26 - CPU-Local Scheduler Service Core Accepted

- Status: accepted as a target-independent Phase 6.3 implementation slice.
  The change adds Rust scheduler service code, focused tests, architecture and
  roadmap updates, and a task record. No shared run queue, remote enqueue, task
  migration, load balancing, multi-core preemption, Phase 7, filesystem,
  networking, SSH, shell, RP1/PCIe, UART interrupt ownership, DMA behavior,
  boot image, or Pi 5 hardware claim was added.
- Context: The accepted CPU-local scheduler service boundary required one
  normal-control-flow adapter to sequence already accepted diagnostic slices:
  target-owned remote wake drains, local blocked-to-runnable transitions,
  pending timer-preemption handling, CPU-local dispatch, and owner metadata
  refresh.
- Decision: Accept `CpuLocalSchedulerService::run_cycle` in `src/scheduler.rs`.
  The service consumes one target-owned remote wake request, applies the
  matching local wake transition, handles optional pending timer preemption,
  dispatches through the owner `PerCoreScheduler` when timer preemption did not
  already select the next task, and refreshes owner-published metadata after
  local mutations.
- Evidence level: static inspection, unit/QEMU tests, QEMU/substitute smoke,
  architecture documentation update, roadmap update, decision-log update,
  task record, whitespace inspection, and mdBook validation.
- Validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test`
  with the documented QEMU 9.2.0 path, `scripts/qemu-smoke.sh`,
  `git diff --check`, `git diff --cached --check`, and `mdbook build` passed.
- Consequences: The service core is still CPU-local and target-independent.
  It preserves explicit wrong-owner, wrong-target, duplicate-runnable,
  non-blocked task, no-runnable, deferred secondary-role, unknown metadata, and
  stale metadata outcomes without granting remote scheduler mutation or shared
  topology.
- Alternatives considered: Leave the accepted order as documentation only,
  create a proof-specific QEMU entry point instead of a reusable core, or start
  shared run queues. A reusable core is the narrowest implementation boundary;
  proof-specific entry points would repeat old diagnostic-surface drift, and
  shared topology remains out of scope until separately planned.

## 2026-05-26 - CPU-Local Scheduler Service Closeout Accepted

- Status: accepted as the Phase 6.3 CPU-local scheduler service closeout
  checkpoint. No Rust implementation, boot image, hardware run, shared run
  queue, remote enqueue, task migration, load balancing, multi-core
  preemption, Phase 7, filesystem, networking, SSH, shell, RP1/PCIe, UART
  interrupt ownership, or DMA behavior was added.
- Context: The accepted CPU-local scheduler service boundary and
  `CpuLocalSchedulerService` core now agree on one normal-control-flow order:
  drain target-owned remote wakes, convert matching local blocked tasks to
  runnable state, handle pending timer-preemption requests, dispatch through
  the owner scheduler, and refresh owner-published metadata after local
  mutations.
- Decision: Accept
  docs/src/project/phase6-cpu-local-scheduler-service-closeout-checkpoint.md
  and tasks/2026-05-26-phase6-cpu-local-scheduler-service-closeout-checkpoint.md.
  The next bounded task should be
  phase6-secondary-scheduler-service-loop-source-inventory-20260526.
- Evidence level: static reconciliation of scheduler architecture docs,
  CPU-local service boundary inventory, accepted service-core task record,
  roadmap, decision log, and `CpuLocalSchedulerService` implementation/tests,
  plus mdBook validation and whitespace inspection.
- Validation: git status --short was clean before edits, git diff --check
  passed, and mdbook build passed. Rust fmt/tests, QEMU smoke reruns, and
  hardware runs were not required because this checkpoint changed only
  Markdown documentation and durable task state.
- Consequences: Secondary scheduler service-loop productionization may be
  planned as the next bounded Phase 6.3 slice. Shared run queues, remote
  enqueue queues, migration, load balancing, work stealing, multi-core
  preemption, Phase 7, filesystem, networking, SSH, shell behavior, RP1/PCIe,
  UART interrupt ownership, and DMA/cache-driver policy remain deferred.
- Alternatives considered: Start secondary service-loop implementation
  immediately, skip to shared scheduler topology, or rerun QEMU/Pi 5 proof
  gates. A closeout checkpoint is the bounded reconciliation requested by the
  supervisor; shared topology remains premature, and no new physical claim is
  made by this documentation-only task.

## 2026-05-26 - Productionization Boundary Inventory Accepted

- Status: accepted as a source-backed repo-health and productionization
  planning checkpoint. No Rust implementation, boot image, hardware run, shared
  run queue, migration, load balancing, multi-core preemption, Phase 7,
  filesystem, networking, SSH, shell, RP1/PCIe, UART interrupt ownership, or
  DMA behavior was added.
- Context: Phase 6.3 now has accepted diagnostic evidence for secondary
  production dispatch, raw SGI/IPI delivery, remote wake requests,
  target-owned local runnable transitions, and shared scheduler metadata on
  QEMU and Pi 5. The roadmap also has accepted evidence-retention and
  diagnostic-surface audits, so the remaining question is which production
  boundary should come next without overstating diagnostic slices as a general
  OS runtime.
- Decision: Accept
  `tasks/2026-05-25-talos-productionization-boundary-inventory.md`. The next
  recommended task is
  `phase6-cpu-local-scheduler-service-boundary-source-inventory-20260526`, a
  documentation/source-inventory contract for ordering timer-preemption request
  handling, target-owned remote wake drains, local runnable transitions,
  production secondary dispatch entry, and owner metadata refresh.
- Evidence level: static source/doc review of scheduler, SMP sync, wakeup/IPI,
  console/TTY, diagnostic command-channel, roadmap, diagnostic-surface,
  evidence-retention, closeout, decision, and task records, plus mdBook
  validation and whitespace inspection.
- Validation: git status --short was clean before edits, git diff --check
  passed, and mdbook build passed. Rust fmt/tests and hardware runs were not
  required because the task changed only Markdown documentation and durable task
  state.
- Consequences: Shared run queues, remote enqueue queues, task migration, load
  balancing, work stealing, multi-core preemption, Phase 7, filesystem,
  networking, SSH, shell behavior, RP1/PCIe, UART interrupt ownership, and
  DMA/cache-driver policy remain deferred. Existing evidence and diagnostic
  cleanup follow-ups remain separately queued or blocked by their own policy.
- Alternatives considered: Jump directly to shared run-queue or task migration
  inventory, productize the diagnostic command channel into a shell-like
  surface, or prioritize evidence/archive cleanup before scheduler
  productionization. Shared queues and migration would skip the missing
  CPU-local service ordering; command productization belongs after descriptor
  and scheduler-blocking TTY semantics; evidence cleanup can proceed separately
  without blocking the next scheduler boundary.

## 2026-05-26 - CPU-Local Scheduler Service Boundary Accepted

- Status: accepted as a Phase 6.3 source inventory and contract. No Rust
  implementation, boot image, hardware run, shared run queue, remote enqueue,
  task migration, load balancing, multi-core preemption, Phase 7, filesystem,
  networking, SSH, shell, RP1/PCIe, UART interrupt ownership, or DMA behavior
  was added.
- Context: Phase 6.3 has accepted separate diagnostic slices for local
  timer-preemption dispatch, target-owned remote wake drains, local
  blocked-to-runnable transitions, production secondary diagnostic dispatch,
  and owner-published shared metadata. Those slices need one CPU-local service
  order before broader shared scheduler topology work can start.
- Decision: Accept
  docs/src/project/phase6-cpu-local-scheduler-service-boundary-source-inventory.md.
  The CPU-local service drains target-owned remote wakes outside IPI context,
  converts matching local blocked tasks to runnable, handles pending local
  timer-preemption requests, dispatches only through the owner scheduler, then
  refreshes owner-published metadata after local state mutations. Remote wake
  drains intentionally precede timer-preemption dispatch so newly runnable
  local tasks can participate in the preemption decision.
- Evidence level: static source/doc review of scheduler, SMP sync, IPI/wakeup,
  shared metadata, roadmap, decisions, accepted closeouts, and accepted task
  records, plus mdBook validation and whitespace inspection.
- Validation: git status --short was clean before edits, git diff --check
  passed, and mdbook build passed. Rust fmt/tests and hardware runs were not
  required because the task changed only Markdown documentation and durable
  task state.
- Consequences: The next bounded task should implement a target-independent
  CPU-local scheduler service core and QEMU-only smoke for this order. Shared
  run queues, remote enqueue, migration, load balancing, work stealing,
  multi-core preemption, Phase 7, filesystem, networking, SSH, shell behavior,
  RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy remain
  deferred.
- Alternatives considered: Handle timer-preemption dispatch before draining
  remote wakes, jump directly to shared run queues or migration, or treat the
  existing proof-only entry points as a runtime service. Dispatching before the
  wake drain can miss a just-delivered runnable task; shared topology would
  skip the CPU-local production boundary; proof entry points do not provide a
  stable normal-control-flow runtime service.

## 2026-05-25 - Phase 6.3 Shared Scheduler Metadata Closeout Accepted

- Status: accepted as the closeout checkpoint for the bounded shared scheduler
  metadata slice. No Rust implementation, boot image, hardware publish/test,
  shared run queue, remote enqueue, task migration, load balancing,
  multi-core preemption, Phase 7, filesystem, networking, SSH, shell,
  RP1/PCIe, UART interrupt ownership, or DMA behavior was added by the
  checkpoint.
- Context: The source inventory, metadata core, QEMU shared metadata smoke,
  and serialized Pi 5 shared metadata proof now all agree that logical CPUs 0
  through 3 can publish/query an owner-published metadata table while
  preserving CPU-local runnable queue ownership.
- Decision: Accept
  phase6-shared-scheduler-metadata-closeout-checkpoint-20260525. Retain the
  focused shared metadata QEMU and Pi 5 scripts as named validation gates.
  The next bounded task should be
  talos-evidence-retention-policy-and-bloat-audit-20260525 before broader
  scheduler productionization.
- Evidence level: static inspection, accepted task/evidence review, scheduler
  architecture review, roadmap update, decision-log update, mdBook
  validation, and whitespace inspection.
- Validation: git status --short before edits was clean, git diff --check
  passed after edits, and mdbook build passed. Rust fmt/tests and hardware
  runs were not required because this checkpoint changed only Markdown
  documentation and durable task state.
- Rationale: The checkpoint prevents the accepted metadata invariant from
  being treated as permission for shared dispatch. The next risk is evidence
  retention and diagnostic surface growth, not a lack of metadata proof.
- Risks: The metadata table is not a shared scheduler topology. Shared run
  queues, remote enqueue, migration, load balancing, work stealing,
  multi-core preemption, userspace, descriptors, filesystem, networking, SSH,
  shell behavior, RP1/PCIe, UART interrupt ownership, and DMA/cache policy
  remain deferred.

## 2026-05-25 - Pi 5 Shared Scheduler Metadata Proof Accepted

- Status: accepted as the physical Pi 5 hardware proof for the first shared
  scheduler metadata invariant. This does not accept shared run queues, remote
  enqueue queues, task migration, load balancing, multi-core preemption, Phase
  7, filesystem, networking, SSH, shell behavior, RP1/PCIe, UART interrupt
  ownership, or DMA behavior.
- Context: QEMU had already proven the owner-published shared scheduler
  metadata table for logical CPUs 0 through 3. Before that invariant could be
  treated as physical evidence, Pi 5 needed a serialized hardware run using
  the accepted secondary cacheable-MMU handoff and cursor-valid serial/TFTP
  evidence.
- Decision: Accept
  `tasks/2026-05-25-phase6-pi5-shared-scheduler-metadata-proof.md` and
  evidence in
  `tasks/evidence/2026-05-25-pi5-shared-scheduler-metadata-proof/local1/` as
  the hardware proof that logical CPUs 0, 1, 2, and 3 can publish/query the
  bounded shared scheduler metadata table while preserving CPU-local scheduler
  ownership.
- Evidence level: serialized Pi 5 hardware run under `hardwareTestLock`, with
  archive digest
  `7ec358f5809aee223364948fa20ba9b4e73f8fd76a1ac0238081926568f74bf0`,
  kernel digest
  `232cab18a49eb75ddc1969438d45ab1874359492028dfea81522f22507d24382`,
  TFTP fetch evidence for `da591740/kernel_2712.img` at 99,136 bytes,
  cursor-valid serial output, and restore evidence. Serial output shows task
  IDs 101/201/301/401, owner-task lookup and boot-task lookup success,
  cross-owner scheduler and metadata mutation rejected, local queues
  preserved, `final-metadata-len=4`, `errors=0`,
  `classification=pi5-shared-scheduler-metadata-complete`, and PASS.
- Validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test`,
  `scripts/qemu-smoke.sh`, `scripts/qemu-shared-scheduler-metadata-smoke.sh`,
  `scripts/qemu-production-secondary-dispatch-smoke.sh`,
  `scripts/qemu-remote-wake-to-local-runnable-smoke.sh`,
  `scripts/rpi5-shared-scheduler-metadata-image.sh`,
  `scripts/rpi5-shared-scheduler-metadata-boot-tree.sh`,
  `scripts/rpi5-archive-review.sh`, serialized hardware run/restore, and
  `git diff --check` passed.
- Consequences: The shared metadata invariant is accepted on physical Pi 5
  cores and can be reconciled by a closeout checkpoint. Any shared run queue,
  remote enqueue queue, task migration, load balancing, or multi-core
  preemption work still requires a later supervisor-planned task.
- Alternatives considered: Treat QEMU shared metadata evidence as sufficient,
  or fold shared metadata proof into a broader migration task. QEMU alone would
  skip the physical cache/coherency and secondary entry path; a broader task
  would blur the owner-only metadata boundary with deferred scheduler movement.

## 2026-05-25 - Phase 6.3 Target-Owned Wake Consumption Contract Accepted

- Status: accepted as the scheduler ownership contract for converting consumed
  remote wake requests into local scheduler wake actions. No Rust
  implementation, boot image, hardware publish/test, shared run queue, task
  migration, production secondary scheduler dispatch, multi-core preemption,
  Phase 7, filesystem, networking, SSH, shell, RP1/PCIe, UART interrupt, or
  DMA behavior was added.
- Context: QEMU and Pi 5 both prove the bounded remote wake-request model:
  CPU 0 can publish requests, signal with SGI INTID 1, and targets can observe,
  EOI, consume, drain, and reject cross-owner queue mutation. That evidence
  deliberately stops before a consumed request mutates local runnable state.
- Decision: Accept
  `docs/src/project/phase6-target-owned-wake-consumption-contract.md`. A
  remote sender may not mutate another CPU's `RunnableQueue` directly. After
  IPI acknowledgement/EOI, only the target CPU may consume its owned wake
  requests outside IPI context and transition one of its own blocked local
  tasks to runnable under local scheduler rules.
- Evidence level: static source inspection of scheduler, `RemoteWakeQueue`,
  per-core ownership, spinlock/IRQ masking, GICv2 SGI paths, accepted QEMU/Pi
  5 remote wake-request records, architecture docs, roadmap, and decision log.
- Validation: `git status --short` was clean before edits,
  `git diff --check` passed after edits, and `mdbook` was unavailable in
  the container. Rust fmt/tests and hardware runs were not required because
  this task changed only Markdown documentation and durable task state.
- Rationale: Keeping the local runnable transition target-owned preserves the
  accepted CPU-local scheduler topology while giving the next worker task a
  precise QEMU-only proof boundary: blocked-to-runnable local wake consumption,
  duplicate coalescing, cross-owner rejection, drained queues, and no
  production secondary dispatch.
- Risks: The contract is not yet implemented. Shared run queues, global task
  lookup, remote enqueue queues, task migration, load balancing, work
  stealing, production secondary scheduler dispatch, multi-core preemption,
  userspace, descriptors, filesystem, networking, SSH, shell behavior,
  RP1/PCIe, UART interrupt ownership, and DMA/cache-coherent driver policy
  remain deferred.
- Alternatives considered: Allow the remote sender to enqueue directly onto
  the target run queue, wake tasks from IPI context, or skip the contract and
  implement the runnable transition immediately. Direct enqueue violates
  CPU-local ownership; IPI-context wakeups would mix interrupt hot-path work
  with scheduler mutation; skipping the contract would make the next proof's
  acceptance criteria ambiguous.

## 2026-05-25 - Pi 5 SMP Lock Cache/Coherence Proof Accepted

- Status: accepted as the physical Pi 5 hardware proof for the first Milestone
  6.2 SMP-safe primitive. This does not accept scheduler migration, shared run
  queues, cross-core wakeups, IPIs, userspace, descriptors, filesystem,
  networking, SSH, shell behavior, UART interrupts, RP1/PCIe, or DMA policy.
- Context: The initial lock proof found real staging/capture and mixed
  cache/MMU issues. A separate handoff task proved that secondary cores can
  install the same cacheable EL2 stage-1 regime as the boot CPU before generic
  lock access. A report-invariant correction then ensured final per-core
  identity and diagnostic state are reset, republished, and visible after the
  handoff.
- Decision: Accept
  `tasks/2026-05-25-phase6-pi5-smp-lock-cache-coherence-final-proof.md` and
  evidence in
  `tasks/evidence/2026-05-25-pi5-smp-lock-cache-coherence-final-proof/` as
  the hardware proof that `SpinLock<T>` can serialize bounded shared access
  across the boot CPU and secondary cores 1, 2, and 3 on Pi 5 after the
  accepted cacheable-MMU handoff.
- Evidence level: serialized Pi 5 hardware run under `hardwareTestLock`,
  with archive digest
  `73041969803f1153a4277d0f56700df08022451a486cd7088ceabe654e953910`,
  kernel digest
  `e28596b5f259775c4c239c3e18b57e3d61d24ff453aa3c762c879e38075f7278`,
  TFTP fetch evidence, cursor-valid serial output, and restore evidence. The
  final invariant reports `counter=192 expected=192 participants=3`,
  `diag-participants=3`, `errors=0`, `mixed-cache-mmu=false`,
  `classification=pi5-smp-lock-cache-coherence-complete`, and
  `rpi5-smp-lock-cache-coherence: PASS`.
- Validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test`,
  `scripts/qemu-smoke.sh`, `scripts/qemu-smp-lock-contention-smoke.sh`,
  `scripts/rpi5-smp-lock-cache-coherence-image.sh`,
  `scripts/rpi5-archive-review.sh`, and `git diff --check` passed.
  `mdbook` was unavailable, so mdBook build was not run.
- Consequences: The generic lock has Pi 5 physical proof and Milestone 6.2 may
  close after proof scaffolding is quarantined. Cache maintenance remains
  separate from the generic lock API. Scheduler migration and cross-core wakeup
  work require a later supervisor-planned source inventory before
  implementation.
- Alternatives considered: Accept the earlier handoff run as the full lock
  proof, keep iterating on entry-discriminator scaffolding, or add cache
  maintenance inside `SpinLock<T>`. The first overstated evidence because the
  report invariant still failed; the second chased temporary observability
  scaffolding after the real blockers were understood; the third would conflate
  mutual exclusion with cache/coherency policy needed later for DMA and mixed
  memory attributes.

## 2026-05-25 - Pi 5 Raw Cross-Core SGI Delivery Accepted

- Status: accepted as the physical Pi 5 hardware proof for raw GIC-400/GICv2
  SGI delivery before scheduler wakeups depend on IPIs. This does not accept
  scheduler remote wakeups, remote enqueue ownership, shared run queues, task
  migration, production IPI use, userspace, descriptors, filesystem,
  networking, SSH, shell behavior, RP1/PCIe, UART interrupt ownership, or DMA
  behavior.
- Context: QEMU had already proven the raw SGI delivery boundary, but Pi 5
  evidence initially showed secondary cores ready with no receive/EOI counts.
  The decisive implementation gap was in the Pi 5 exception dispatch cfg: the
  cross-core IPI proof installed the target handler, but the target IRQ
  dispatcher did not compile that path for the proof flag.
- Decision: Accept
  `tasks/2026-05-25-phase6-pi5-cross-core-ipi-delivery-proof.md` and evidence
  in `tasks/evidence/2026-05-25-pi5-cross-core-ipi-delivery-proof/irqdispatch1/`
  as the hardware proof that the boot CPU can send SGI INTID 1 to secondary
  cores 1, 2, and 3 and each secondary can receive and EOI it under Talos on
  Pi 5.
- Evidence level: serialized Pi 5 hardware run under `hardwareTestLock`, with
  archive digest
  `a6c5cb6999784e8f8c61a07765d39e9549c19c0ae37a54267c738b116a521a79`,
  kernel digest
  `44792c6681d0e67df08abeaebd18f2408680940ead47e2cf1e0b44f5b3956837`,
  TFTP fetch evidence, cursor-valid serial output, and restore evidence.
  Serial output shows `participants=3`, `errors=0`, `ready-mask=0xe`,
  `complete-mask=0xe`,
  `classification=pi5-cross-core-ipi-delivery-complete`, and PASS.
- Validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test`,
  `scripts/qemu-smoke.sh`, `scripts/qemu-cross-core-ipi-delivery-smoke.sh`,
  `scripts/rpi5-cross-core-ipi-delivery-image.sh`,
  `scripts/rpi5-archive-review.sh`, and `git diff --check` passed.
- Consequences: Phase 6 can plan remote wakeup ownership on top of a proven
  raw Pi 5 SGI delivery path. The raw proof remains diagnostic-only until a
  later task introduces scheduler-owned IPI semantics, remote enqueue
  invariants, and production wakeup policy.
- Alternatives considered: Keep treating the Pi 5 SGI evidence as a hardware
  delivery failure or accept QEMU behavior as sufficient. The first would have
  mistaken a target cfg dispatch gap for GIC non-delivery; the second would
  have skipped required physical evidence before scheduler wakeups rely on
  IPIs.

## 2026-05-25 - Pi 5 Secondary Cacheable-MMU Handoff Accepted

- Status: accepted as the hardware proof for the secondary cacheable EL2 stage-1 handoff gate. The Pi 5 SMP lock/cache-coherence proof is not accepted by this decision.
- Context: The prior Pi 5 lock proof was blocked because secondaries entered the shared lock diagnostic with cacheable MMU disabled while the boot CPU was cacheable/MMU-enabled. The implementation task added a narrow handoff that publishes MAIR_EL2, TCR_EL2, TTBR0_EL2, and SCTLR_EL2 from the boot CPU and makes each secondary install the same cacheable stage-1 regime before generic lock access.
- Decision: Accept `tasks/2026-05-25-phase6-secondary-cacheable-mmu-handoff-pi5-proof.md` and evidence in `tasks/evidence/2026-05-25-pi5-secondary-cacheable-mmu-handoff-proof/` as the hardware proof that secondaries can join the cacheable EL2 stage-1 regime before shared lock state is used. The original lock proof must resume as a separate bounded task because the same run ended with `pi5-smp-lock-cache-coherence-invariant-failed` after the handoff passed.
- Evidence level: serialized Pi 5 hardware run under `hardwareTestLock`, with archive digest `21f4e80cef35b40d13792fdac4f7a0fa6cce463af0d3eb3c825d9d6c87653d90`, kernel digest `acc334beb5bc82555d6d4c3309d3e24b0b669593768cb9d01e479bc40e350e40`, TFTP event evidence, serial output, and restore evidence. Serial output shows logical cores 1, 2, and 3 reporting `diag-sctlr-el2=0x0000000030c51835` and `diag-cacheable-mmu=true`.
- Validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test`, `scripts/qemu-smoke.sh`, `scripts/qemu-smp-lock-contention-smoke.sh`, `scripts/rpi5-archive-review.sh`, and `git diff --check` passed. `mdbook` was unavailable, so mdBook build was not run.
- Consequences: The mixed cache/MMU blocker is closed for the handoff gate. The lock proof remains unaccepted and should resume with a narrower discriminator for the final per-core identity/report invariant rather than reopening the cache/MMU question.
- Alternatives considered: Treat the full lock proof as accepted because the final shared counter reached 192/192, or keep the handoff task open until the entire lock proof passes. The first would overstate the evidence because logical cores 1 and 2 had zeroed final identity fields; the second would conflate the handoff proof with a later lock diagnostic invariant.

## 2026-05-24 - Phase 4 Closeout Accepted

- Status: accepted as the checkpoint closing Phase 4 interrupts, timers, and
  single-core scheduler preemption before Phase 5 planning. No kernel code,
  boot image, hardware lock, or hardware run changed in this task.
- Context: Talos had accepted QEMU and Pi 5 EL2 timer IRQ delivery, monotonic
  tick accounting, single-core interrupt-mask critical sections, scheduler
  structures, cooperative context switching, voluntary yield dispatch, QEMU
  timer-driven preemption, Pi 5 timer-driven preemption, and a consolidated
  scheduler/preemption contract. Before Phase 5 local console planning starts,
  the accepted evidence and remaining deferrals needed one closeout record.
- Decision: Accept `docs/src/project/phase4-closeout-checkpoint.md` as the
  Phase 4 closeout. Phase 5 planning may start with
  `phase5-console-device-model-source-inventory-20260524`, limited to source
  inventory and early/runtime console ownership docs. Console/TTY
  implementation, descriptor tables, userspace, filesystems, networking, SSH,
  and shell behavior remain out of scope until separately planned.
- Evidence level: static documentation checkpoint over accepted commits and
  task records: `0fb6260`, `de40482`, `bce215d`, `966d453`, `957bbc8`,
  `54d2075`, `1bbfec6`, `68e3529`, `37ce658`, `7ce1a91`, `988ea31`,
  `6f24076`, `24c25c6`, `8134e7c`, `2cf0e64`, `9e53676`, and `f1e0cd2`.
  Pi 5 hardware evidence remains in
  `tasks/evidence/2026-05-24-pi5-el2-timer-irq-smoke/`,
  `tasks/evidence/2026-05-24-pi5-monotonic-tick-accounting/`, and
  `tasks/evidence/2026-05-24-pi5-timer-preemption-hardware-proof/`.
- Validation: `git status --short` was inspected before checkpoint edits and
  was clean. `git diff --check` and `git diff --cached --check` passed.
  `mdbook build` was not run because `mdbook` is unavailable in the container.
- Consequences: The worker queue may mark the Phase 5 console inventory task
  ready. The next task must not infer runtime console, TTY, descriptor,
  userspace, filesystem, networking, SSH, or shell implementation from the
  Phase 4 scheduler evidence.
- Risks: Current timer-preemption boot images are validation surfaces, not
  supported kernel interfaces. The scheduler remains single-core only; the
  accepted IRQ-mask primitive is not an SMP lock. Phase 5 must preserve the
  early logging/runtime console boundary before introducing input or TTY state.

## 2026-05-24 - Phase 6.1 Secondary-Core Bring-Up Contract Accepted

- Status: accepted as the source inventory and contract before Phase 6.1 SMP
  implementation. No code, boot archive, hardware lock, hardware run,
  scheduler migration, SMP-safe primitive, userspace, filesystem, networking,
  SSH, descriptor, or shell behavior changed in this task.
- Context: Phase 5.3 closed the local diagnostic command channel and deferred
  SMP until a bounded supervisor-planned task. The roadmap names PSCI as the
  primary Pi 5 secondary-core path, and Raspberry Pi Linux device-tree evidence
  advertises four Cortex-A76 CPU nodes using `enable-method = "psci"` with an
  SMC PSCI node.
- Decision: Accept
  `docs/src/project/phase6-secondary-core-bringup-source-inventory.md` as the
  Phase 6.1 contract. PSCI `CPU_ON` via SMC is the default path; spin-table,
  VideoCore mailbox, or custom mailbox bring-up remains fallback research only.
  A secondary core is not considered alive for scheduler work until it proves
  MPIDR/logical identity, exclusive stack ownership, per-core state
  registration, and controlled handoff.
- Evidence level: static source inventory over repository roadmap/reference
  docs, current AArch64 boot and target boundaries, Raspberry Pi Linux
  `bcm2712.dtsi`, and QEMU 9.2.0 generated `virt` DTB strings for
  `virt,gic-version=2,virtualization=on -cpu cortex-a76 -smp 4`.
- Validation: `git status --short` was inspected before edits and was clean.
  `git diff --check` passed. `mdbook build` was not run because `mdbook` is
  unavailable in the container. Rust fmt/tests were not required because no
  Rust files changed.
- Consequences: The next bounded worker task is
  `phase6-qemu-secondary-core-bringup-discriminator-20260524`. QEMU
  substitute evidence and Pi 5 hardware proof requirements stay separate.
- Risks: Talos does not yet read `MPIDR_EL1`, parse CPU nodes, allocate
  per-core stacks/state, call PSCI, or coordinate concurrent console output.
  Those risks belong to later explicit Phase 6.1 tasks.

## 2026-05-24 - Accept Phase 6.2 SMP-Safe Primitive Contract

- Status: accepted as the Milestone 6.2 synchronization contract before shared
  lock implementation. No Rust implementation, boot archive, hardware publish,
  power-cycle, hardware lock, scheduler migration, shared run queue,
  cross-core wakeup, userspace, descriptor, filesystem, networking, SSH, shell,
  UART interrupt, RP1/PCIe, or DMA behavior changed in this task.
- Context: Phase 6.1 proved Pi 5 secondary-core startup and a controlled
  diagnostic workload, but the accepted hardware proof required explicit
  cache maintenance before primary-side state snapshots became reliable. The
  existing `single_core_irq_mask_save()` primitive is documented as boot-CPU
  IRQ masking only, and `src/scheduler.rs` remains ordinary single-core mutable
  state.
- Decision: Accept
  `docs/src/project/phase6-smp-safe-primitives-source-inventory.md` as the
  source-backed contract for Milestone 6.2. Talos will keep local IRQ masking,
  SMP mutual exclusion, memory ordering, and cache maintenance as separate
  responsibilities. The next implementation task is
  `phase6-spinlock-barrier-core-20260524`.
- Evidence level: static source inventory over `src/smp.rs`,
  `src/arch/aarch64/mod.rs`, `src/scheduler.rs`, target diagnostic call sites,
  Phase 6.1 evidence summaries, and the accepted scheduler/interrupt docs.
- Validation: `git status --short` was inspected before documentation edits and
  was clean. `git diff --check` passed. `mdbook build` was not run because
  `mdbook` is unavailable in the container. Rust fmt/tests were not required
  because no Rust files changed.
- Consequences: A future lock primitive must document held-lock constraints,
  acquire/release ordering, IRQ-mask composition, and non-recursive misuse.
  Scheduler shared data structures, task migration, load balancing, IPIs, and
  cross-core wakeups remain deferred until separately planned.

## 2026-05-24 - Accept Phase 6.1 Per-Core State/Stack Boundary

- Status: accepted as the first implementation boundary for Phase 6.1
  secondary-core state and stack ownership. No Pi 5 hardware publish,
  power-cycle, hardware lock, scheduler migration, SMP-safe primitive,
  cross-core preemption, load balancing, EL0, syscall, descriptor, filesystem,
  networking, SSH, or shell behavior changed in this task.
- Context: The accepted Phase 6.1 contract required each secondary core to
  prove identity, exclusive stack ownership, per-core state registration, and a
  controlled handoff before any scheduler work. The QEMU discriminator had
  already proven PSCI SMC secondary startup under QEMU virt, but its state and
  stack reporting was diagnostic-local.
- Decision: Add `src/smp.rs` as the shared Phase 6.1 per-core ownership
  boundary: four possible cores, 4 KiB secondary kernel stack slots, the
  `parked -> entered -> stack-ready -> registered -> handoff-ready` lifecycle,
  per-core atomic identity/state records, stack-slot validation, and the Pi 5
  MPIDR affinity map. Retain the QEMU discriminator as the focused substitute
  gate using this boundary.
- Evidence level: static inspection, no_std unit tests, QEMU/substitute, and
  image/archive inspection. `cargo -Zjson-target-spec test` passed 96 tests;
  `scripts/qemu-secondary-core-discriminator.sh` reported all three secondary
  QEMU cores at `handoff-ready` on distinct stack slots; `scripts/qemu-smoke.sh`,
  `scripts/rpi5-image.sh`, and `scripts/rpi5-format-guard-check.sh` passed.
- Consequences: The next Phase 6.1 hardware proof can reuse the accepted
  state/stack ownership vocabulary, but it still needs serialized Pi 5 evidence
  before claiming hardware behavior. Scheduler migration and SMP-safe
  primitives remain explicitly deferred.
- Alternatives considered: keep state reporting inside the QEMU diagnostic or
  combine this task with Pi 5 hardware proof. Keeping it diagnostic-local would
  blur the production ownership boundary; combining it with hardware would
  increase risk and skip the queued task decomposition.

## 2026-05-24 - Accept QEMU Polling TTY RX Diagnostic

- Status: accepted as the first Milestone 5.2 local serial input proof. No Pi 5
  hardware publish, power-cycle, hardware lock, descriptor table, syscall,
  userspace, shell, filesystem, networking, SSH, UART interrupt, or scheduler
  blocking behavior changed in this task.
- Context: Phase 5.1 accepted `runtime-console0` as the default runtime console
  identity and inventoried QEMU PL011 polling RX as the lowest-risk first local
  input surface. The TTY/stdio shape then required canonical-lite behavior to
  sit above the runtime-console/TTY boundary rather than in a target UART
  shortcut.
- Decision: Add a QEMU-only diagnostic gated by
  `TALOS_QEMU_POLLING_TTY_RX_DIAGNOSTIC` and
  `scripts/qemu-tty-rx-diagnostic.sh`. The PL011 backend exposes
  `poll_read_byte`, runtime console exposes a polling input trait, and the TTY
  diagnostic records exact raw input, line bytes, echo bytes, control events,
  truncation, and bounded timeout classification.
- Evidence level: QEMU/substitute plus unit tests. The focused script injects
  `61 62 58 08 63 59 7f 64 03 65 66 67 68 69 0d` and the serial log reports
  `line-hex=61 62 63 64 65 66 67 68`,
  `echo-hex=61 62 58 08 20 08 63 59 08 20 08 64 65 66 67 68 0d 0a`,
  `control-events=ctrl-c`, truncation=true, timeout=false, and
  `qemu-tty-rx-diagnostic: PASS`.
- Consequences: Milestone 5.2 now has a local QEMU input proof. The next worker
  task may promote the line discipline into target-independent code or define
  the input result contract, but Pi 5 UART10 input remains a serialized
  hardware task and must not be inferred from QEMU evidence.
- Alternatives considered: read PL011 directly from the diagnostic client, wait
  for Pi 5 input first, or build descriptor/syscall reads now. Those would blur
  subsystem boundaries, add hardware risk before local proof, or jump ahead of
  the accepted roadmap.

## 2026-05-24 - Phase 4 Pre-Scheduler Closeout Accepted

- Status: accepted as the checkpoint between Phase 4.1/4.2 interrupt/timer
  bring-up and Milestone 4.3 scheduler shape work. No kernel code, boot image,
  hardware lock, or hardware run changed in this task.
- Context: Talos had accepted QEMU and Pi 5 EL2 physical timer IRQ delivery,
  a timer-smoke checkpoint, periodic monotonic tick accounting on QEMU and
  Pi 5, and an explicit single-core IRQ mask/restore primitive. Before
  scheduler structures begin, those facts needed one documented boundary with
  deferred work named explicitly.
- Decision: Accept the Phase 4.1/4.2 boundary documented in
  `docs/src/project/phase4-prescheduler-closeout.md`. Milestone 4.3 may start
  with a bounded scheduler-shape task that checks task/process terminology and
  ownership against `docs/src/project/early-posix-shape.md` before committing
  scheduler structs.
- Evidence level: static documentation checkpoint over accepted commits and
  task records: `0fb6260`, `de40482`, `bce215d`, `966d453`, `957bbc8`,
  `54d2075`, and `1bbfec6`. Pi 5 hardware evidence remains in
  `tasks/evidence/2026-05-24-pi5-el2-timer-irq-smoke/` and
  `tasks/evidence/2026-05-24-pi5-monotonic-tick-accounting/`.
- Validation: `git status --short` was inspected before checkpoint edits and
  was clean. `git diff --check` passed. `mdbook build` was not run because
  `mdbook` is unavailable in the container.
- Consequences: The worker queue may move to scheduler-shape planning. The next
  task must stay single-core first and must not infer SMP, userspace, blocking
  I/O, POSIX clocks, filesystems, networking, or SSH from the timer evidence.
- Risks: This does not implement kernel threads, runnable queues, context
  switching, preemptive time slicing, sleep queues, preemption-disable policy,
  SMP locks, UART interrupts, lower-EL timer access, DMA, RP1/PCIe routing,
  filesystems, userspace, networking, or SSH.

## 2026-05-24 - Consolidate Scheduler/Preemption Contract

- Status: accepted as the final scheduler/preemption contract checkpoint before
  Phase 4 closeout. No kernel code, boot image, hardware lock, or hardware run
  changed in this task.
- Context: QEMU and Pi 5 both accepted the timer-driven single-core
  kernel-thread preemption smoke. Before closeout, Talos needed to separate the
  durable scheduler contract from one-off diagnostic boot surfaces.
- Decision: Treat the production Phase 4 contract as the scheduler-owned
  task/runnable-queue/context-frame model, short boot-CPU IRQ-masked scheduler
  mutation windows, and a timer IRQ hot path limited to
  acknowledge/classify/tick/request/reprogram/EOI. Retain the QEMU
  context-switch, scheduler-yield, timer-preemption, and Pi 5 timer diagnostic
  surfaces only as validation gates with named revisit conditions.
- Evidence level: static inspection and documentation checkpoint over accepted
  QEMU proof commit `2cf0e64`, Pi 5 hardware proof commit `9e53676`,
  `src/target/qemu_virt.rs`, `src/target/rpi5.rs`,
  `src/scheduler.rs`, and the task/evidence records under
  `tasks/evidence/2026-05-24-pi5-timer-preemption-hardware-proof/`.
- Validation: `git diff --check` passed. `mdbook build` was not run because
  `mdbook` is unavailable in the container. Rust tests and hardware gates were
  not rerun because this task changed docs and task records only.
- Consequences: The Phase 4 closeout checkpoint may start next. Phase 5 must
  not treat the retained timer-preemption boot images as supported kernel
  interfaces; they are regression/evidence surfaces until ordinary boot or
  local console diagnostics cover the same counters.
- Alternatives considered: delete the diagnostic surfaces immediately, keep
  them undocumented, or promote the smoke harnesses into general scheduler
  interfaces. Immediate deletion would remove useful regression gates before
  closeout, while undocumented retention or promotion would blur the Phase 4
  kernel contract.

## 2026-05-24 - Accept EL2 Physical Timer IRQ Smoke on Pi 5

- Status: accepted
- Context: QEMU virt had accepted evidence for CNTHP_*_EL2 raising PPI 10 /
  INTID 26 through GICv2 and returning through the current-EL IRQ frame. Phase 4
  needed the same shape proven on Pi 5 GIC-400 before moving toward reusable
  tick accounting.
- Decision: Add a focused Pi 5 diagnostic gated by
  TALOS_RPI5_TIMER_IRQ_DIAGNOSTIC. It uses GIC-400 distributor
  0x10_7fff_9000, CPU interface 0x10_7fff_a000, CNTHP_*_EL2, and PPI 10 /
  INTID 26. The IRQ path acknowledges with GICC_IAR, masks the EL2 physical
  timer, EOIs with GICC_EOIR, records bounded atomics, and returns through the
  saved exception frame; all formatting remains outside the IRQ path.
- Evidence: Serialized lab run
  tasks/evidence/2026-05-24-pi5-el2-timer-irq-smoke/ published archive
  1861b6978b505381fd28ffb21320f1db9434405c4ce44af69354d6e1e82f5bb2 with
  image hash 850902110e96af341e595f1493c0802f742e6618ad57546f0f37dc06236d3e0a
  and size 86,429 bytes. TFTP served kernel_2712.img; serial showed
  irq-count=1, vector 5, iar=0x0000001a, intid=26, unexpected=0, post-IRQ
  workload progress, and rpi5-timer-irq-smoke: PASS.
- Consequences: Phase 4 can checkpoint the target timer-smoke behavior before
  periodic tick accounting. UART interrupts, SMP routing, lower ELs, scheduler
  policy, RP1/PCIe, DMA, and networking remain explicitly out of scope.
- Alternatives considered: accept QEMU-only evidence, switch to the EL1
  physical or virtual timer first, or poll the timer status without IRQ
  delivery. Those would not satisfy the Phase 4 hardware interrupt-delivery
  requirement.

## 2026-05-24 - Phase 4 Timer-Smoke Checkpoint Accepted

- Status: accepted as the checkpoint between one-shot timer delivery smokes and
  reusable timekeeping policy. No kernel code, boot image, hardware lock, or
  hardware run changed in this task.
- Context: QEMU virt and Pi 5 both had accepted EL2 physical timer IRQ evidence
  for CNTHP_*_EL2, PPI 10 / INTID 26, GIC acknowledgement/EOI, bounded IRQ
  accounting, and return to post-IRQ work. Phase 4 needed a checkpoint before
  monotonic tick accounting or scheduler-adjacent work could start.
- Decision: Accept the shared one-shot timer delivery boundary and make
  monotonic tick accounting the next bounded implementation slice. No additional
  delivery discriminator is needed first, but the tick task must keep
  interrupt-time constraints, reprogramming order, and single-core limitations
  explicit.
- Evidence level: static documentation checkpoint over accepted commits and
  task records. QEMU evidence is commit `bce215d` and
  `tasks/2026-05-24-phase4-qemu-el2-timer-irq-smoke.md`; Pi 5 evidence is
  commit `966d453` and
  `tasks/evidence/2026-05-24-pi5-el2-timer-irq-smoke/`.
- Validation: `git diff --check` passed. `mdbook build` was not run because
  `mdbook` is unavailable in the container.
- Rationale: Both active targets have now proven interrupt-driven delivery for
  the same EL2 physical timer PPI, so another one-shot probe would add little
  value before periodic tick accounting. The checkpoint prevents scheduler or
  preemption policy from being inferred from a smoke test.
- Risks: This does not implement periodic ticks, interrupt mask/restore policy,
  scheduler structures, preemption, SMP, UART interrupts, lower-EL timer
  routing, DMA, RP1/PCIe, filesystem/userland, networking, or SSH.

## 2026-05-18 - Repository and Target Strategy

- Status: accepted
- Context: Talos needs to be a standalone project that Matthew can later push to GitHub, while still allowing fast generic kernel iteration before physical Pi 5 serial feedback is available.
- Decision: Keep Talos in its own git repository under `projects/talos`. Build the kernel as a Rust no_std project with a generic AArch64/QEMU virt target for fast validation and a separate `talos-rpi5-bcm2712` target for the first physical board.
- Consequences: Generated artifacts stay out of git. QEMU results may validate generic architecture and toolchain behavior, but physical Pi 5 claims require lab-controller and serial evidence.
- Alternatives considered: continue evolving Daedalus directly, make the project Pi 5-only without a generic target split, or delay repo setup until first hardware boot. Those options would make the design harder to review, harder to publish, or more likely to mix Pi 4 assumptions into Talos.

## 2026-05-19 - Target-Specific Physical Link Bases

- Status: superseded by 2026-05-19 - Match Raspberry Pi 5 Kernel Image Text Offset
- Context: The first Pi 5 hardware boot emitted RP1 firmware serial messages but no Talos banner. Static inspection showed the shared linker script placed Talos at `0x40200000`, matching QEMU virt RAM base `0x40000000` plus the arm64 Image `0x200000` text offset. Raspberry Pi firmware should instead load the arm64 Image at the text offset from the Pi RAM base, so early absolute symbols such as BSS and stack must resolve near `0x00200000` for the physical Pi path.
- Decision: Keep the arm64 Image text offset at `0x00200000`, keep QEMU virt linked at `0x40200000`, and give the Pi 5 target its own linker script that links `kernel_2712.img` at `0x00200000`.
- Required validation: QEMU smoke must continue passing for the generic target. Pi 5 target builds must show `_start` and `__kernel_start` at `0x00200000` before the next hardware archive is published. Physical acceptance still requires lab-controller publish, one controlled power-cycle, serial output proving Talos reached entry, and rollback/recovery if the boot fails.
- Risks: If Raspberry Pi firmware uses a nonzero physical base in the lab configuration, this needs revisiting. The current decision is based on the Linux arm64 Image contract plus the prior failure shape; serial hardware evidence remains the deciding proof.
- Alternatives considered: keep one QEMU-oriented linker layout for both targets, use a custom armstub, or add an assembly-only UART probe before addressing the load-base mismatch. The separate Pi 5 linker is the smallest correction that preserves the firmware contract and avoids changing the hardware boot path.

## 2026-05-19 - Pi 5 RP1 UART0 Preserved Mapping

- Status: accepted
- Context: The lab serial cable observes the Pi 5 40-pin header UART. The first RP1 UART0 marker used `0x1f00030000`, derived from the PCIe non-prefetchable window for pcie2. Hardware attempts did not emit the marker. Raspberry Pi firmware documentation says `enable_rp1_uart=1` initializes RP1 UART0 for bare-metal debug and does not reset RP1 when paired with `pciex4_reset=0`; a Pi 5 bare-metal reference reports firmware output `RP1_UART 0000001c00030000` for that mode. Raspberry Pi Linux describes RP1 UART0 as RP1 bus register offset `0xc0_40030000`.
- Decision: Treat `0x1c00030000` as the firmware-preserved RP1 UART0 physical mapping for first-light diagnostics. Use it in the Pi 5 target map, the assembly marker, and staged `earlycon` hints. Keep `0x1f...` documented as the pcie2 non-prefetchable CPU window, not the preserved firmware UART path.
- Required validation: Local Pi 5 builds must show the marker uses `0x1c00030000`. Physical acceptance still requires a controlled lab power-cycle and serial output proving Talos reached entry.
- Risks: The current hardware evidence still stops after firmware DDR logs, so this decision fixes a concrete address bug but does not yet prove the kernel image is reached. If later TFTP/image-format evidence shows the firmware uses a different mapped view at handoff, revisit this ADR.
- Alternatives considered: continue using the PCIe window address, use BCM2712 UART10 instead of the header UART, or wait for a full RP1/PCIe driver before serial diagnostics. The preserved RP1 UART path is the narrowest path aligned with the attached cable and Raspberry Pi firmware support.

## 2026-05-19 - Arm64 Image Header Size Must Match the Binary

- Status: accepted
- Context: Pi 5 hardware attempts reached firmware DDR logging but not Talos entry. A non-hardware image-format review found Talos' arm64 Image header advertised `image_size=0x200000`, while the generated `kernel_2712.img` was 82616 bytes. That stale constant came from the text offset, not the produced binary size.
- Decision: Emit `__kernel_image_end - _start` in the arm64 Image header and define `__kernel_image_end` before `.bss` in each linker script. Keep generated heap/stack reservations as `NOLOAD` memory owned by early kernel setup, not bytes claimed in the boot image file. Make `scripts/rpi5-image.sh` fail if the header size and file size diverge.
- Required validation: Pi 5 image generation must report matching file/header sizes. QEMU smoke must continue to boot the generic target, and physical Pi 5 acceptance still requires a controlled hardware run with serial evidence.
- Risks: If Raspberry Pi firmware wants a different interpretation of the arm64 `image_size` field for this network boot mode, revisit with TFTP/firmware evidence. The current choice aligns the header with the actual binary loaded by firmware.
- Alternatives considered: leave `image_size` at the text offset, set `image_size=0`, or include `NOLOAD` reservations in the file size. Matching the generated file size is the narrowest correction and gives the build a regression check.

## 2026-05-19 - Keep First-Light Firmware Configuration Minimal

- Status: accepted
- Context: Corrected-image hardware evidence reached Raspberry Pi firmware and RP1 firmware logging, but still did not emit the Talos entry marker. The staged boot tree inherited `dtoverlay=uart0-pi5` from the Linux boot source. Talos first-light writes RP1 UART0 directly through the firmware-preserved mapping, before parsing or relying on the device tree.
- Decision: Strip `dtoverlay=uart0-pi5` from Talos first-light `config.txt`. Keep `enable_rp1_uart=1`, `pciex4_reset=0`, and `uart_2ndstage=1`, because those are directly relevant to preserving the 40-pin header UART and observing firmware logs.
- Required validation: The archive review gate must fail if `dtoverlay=uart0-pi5` remains in the staged config. Physical acceptance still requires one controlled hardware run and serial output proving Talos reached entry.
- Risks: If later Talos relies on firmware-applied overlays or Linux-compatible DTB mutations, this should be revisited after a DTB parser exists. For first-light, removing the overlay reduces firmware work before entry and narrows the failure surface.
- Alternatives considered: keep all Linux boot-source config lines unchanged, remove all overlays from the archive, or switch to a boot ramdisk flow. Stripping only the Linux UART overlay is the smallest change tied to the current failure mode.

## 2026-05-19 - Match Raspberry Pi 5 Kernel Image Text Offset

- Status: accepted
- Context: Repeated hardware attempts reached Raspberry Pi firmware and RP1 firmware logging but never emitted the Talos entry marker. The Talos Pi 5 image advertised arm64 Image `text_offset=0x00200000`. A comparison against the official Raspberry Pi `kernel_2712.img` showed the decompressed Pi 5 kernel image advertises `text_offset=0`, `image_size=30081024`, flags `0xc`, and `ARMd` magic.
- Decision: Link the Talos Pi 5 image at physical `0x00000000` and advertise arm64 Image `text_offset=0` for `kernel_2712.img`. Keep the QEMU virt target at its QEMU-specific `0x40200000` link/load address.
- Follow-up: Match the official Pi 5 arm64 Image flags field as well: Talos now advertises flags `0xc` for the Pi 5 image while keeping the QEMU image flags unchanged.
- Required validation: Local Pi 5 image generation and archive review must show `text_offset=0`, matching the Raspberry Pi 5 kernel image convention. Physical acceptance still requires a controlled hardware run that reaches the Talos entry marker or later serial output.
- Risks: If the firmware places the image at a nonzero physical base while using a zero header offset, Talos' absolute BSS/stack symbols will still be wrong; hardware evidence decides this. If that happens, the next iteration should move the earliest marker to fully position-independent code before any absolute symbol use.
- Alternatives considered: keep the generic arm64 `0x200000` offset, set `image_size=0` legacy mode, or add a custom armstub. Matching the official Pi 5 kernel image header is the narrowest project-local correction.

## 2026-05-19 - Test Pi 5 Boot Ramdisk Path

- Status: accepted
- Context: Direct TFTP boot-tree attempts repeatedly reached the same firmware/RP1 boundary before Talos entry. Raspberry Pi documentation describes `boot_ramdisk=1` as useful for network boot, where the bootloader loads a raw FAT32 `boot.img` and reads subsequent boot files from it.
- Decision: Add a bounded first-light experiment that stages `boot_ramdisk=1` and a plain FAT32 `boot.img` containing the same Talos config, DTB, overlays, and kernel images. Keep the ordinary root files in the archive as well so the lab archive contract remains satisfied.
- Required validation: Local archive review must prove `boot.img` is readable by mtools and contains `config.txt`, `kernel_2712.img`, and `kernel8.img`. Physical validation requires one controlled Pi 5 power cycle and serial evidence.
- Risks: If the firmware stops even earlier or ignores `boot_ramdisk=1` for this network path, the evidence should push the next iteration back toward bootloader/TFTP visibility or a lower-level firmware diagnostic.
- Alternatives considered: keep iterating only on the raw `kernel_2712.img`, require a lab API TFTP-log endpoint first, or add a custom armstub. The boot ramdisk path is a documented Pi 5 network-boot shape and is small enough to test safely.

## 2026-05-19 - Add a Custom Armstub Diagnostic

- Status: accepted
- Context: Direct-root, minimal-config, Pi 5 Image-header-matched, and `boot_ramdisk=1` hardware attempts all rebooted the board and emitted Raspberry Pi firmware/RP1 serial output, but none emitted Talos' `T1` entry marker. The repeated boundary suggests the next useful evidence should come before the normal `kernel_2712.img` handoff rather than from another kernel header tweak.
- Decision: Add a bounded custom armstub diagnostic path. The normal Talos boot-tree script remains unchanged; a separate staging script appends `armstub=armstub8-2712.bin` and includes a tiny AArch64 binary that writes `S1\r\n` to firmware-preserved RP1 UART0 at `0x1c00030000`, then waits.
- Required validation: Local validation must prove the armstub binary is non-empty and the archive review gate accepts the optional armstub file. Physical validation is exactly one controlled lab power-cycle under the hardware lock. `S1` on serial proves the custom armstub path ran; no `S1` keeps the investigation at the firmware/config/file-load boundary.
- Risks: A custom armstub is diagnostic-only and does not prove the normal kernel handoff. If it runs, the next step is to decide whether to evolve it into a real handoff helper or use it only to instrument the bootloader boundary. If it does not run, the issue is still earlier than that path or the Pi 5 network boot firmware ignores this armstub setting.
- Alternatives considered: require a new lab TFTP-log endpoint, keep iterating on `kernel_2712.img`, or change rollback strategy. The armstub diagnostic is small, local, and reversible, and it creates pre-entry evidence without new privileged host access.

## 2026-05-19 - Test Serial-Prefixed Network Boot Mirror

- Status: accepted
- Context: The known-good Pi OS Lite TFTP sequence probes `da591740/config.txt` before falling back to root `config.txt`. Earlier evidence says that miss was not fatal for Linux, but repeated Talos runs stop before any kernel or armstub marker. If the Talos archive shape changes the fallback behavior, a serial-prefixed mirror is a small way to test that boundary without changing the kernel image.
- Decision: Add a separate staging script that keeps the normal root boot files and duplicates the same required files under `da591740/`. The archive review gate verifies the prefixed mirror is complete and byte-identical to the root files when present.
- Required validation: Local archive review must pass and show both root and `da591740/` files. Physical validation requires one controlled lab power-cycle and serial evidence.
- Risks: If this runs, it proves the root-only tree was not equivalent in this lab network-boot path, but it does not explain why fallback differed. If it does not run, it rules out the simplest serial-prefix hypothesis and pushes the next step back toward lab-side TFTP visibility or firmware/EEPROM diagnostics.
- Alternatives considered: require direct TFTP logs, keep adding kernel diagnostics, or restore a full Pi OS Lite source tree. The prefix mirror is reversible and can be tested with existing archive tooling.

## 2026-05-19 - Stop Archive-Shape Iterations Without File-Load Evidence

- Status: accepted
- Context: Direct-root, minimal-config, Image-header-matched, `boot_ramdisk=1`, custom armstub, serial-prefix mirror, and combined serial-prefix plus armstub archives all rebooted the Pi and emitted the same Raspberry Pi firmware/RP1 serial boundary, but none emitted the `S1` armstub marker, `T1` Talos entry marker, or Talos banner.
- Decision: Stop adding new Talos archive-shape variants until the workflow has lab-side TFTP request/file-load visibility, EEPROM boot diagnostics, or a recreated known-good Pi OS Lite boot source that can be compared directly. The current evidence is pre-entry and does not justify more Rust-side or arm64 Image-header changes.
- Required validation: The next hardware-dependent step should first prove which files the Pi requests and successfully loads, or prove the known-good boot source shape that differs from the staged Talos source. Hardware claims still require one controlled Pi 5 power-cycle under the hardware lock and serial/TFTP evidence.
- Risks: This hold delays continued trial-and-error, but it avoids consuming rollback history and power cycles on low-signal variants. If new lab visibility shows the firmware is loading Talos files correctly, revisit position-independent earliest-entry code or a different UART assumption.
- Alternatives considered: continue adding config variants, restore older header/linker choices, or evolve the diagnostic armstub into a handoff helper. Those paths now have low expected value because the configured armstub itself has not produced output.

## 2026-05-19 - Separate Firmware-Preserved UART From UART Reinit

- Status: accepted
- Context: The upgraded lab API proved the Pi is served the prefixed `config.txt`, `kernel_2712.img`, DTB, overlays, `cmdline.txt`, and `armstub8-2712.bin`. The original armstub and kernel markers reinitialized PL011 before writing but did not fully mirror the Rust PL011 baud and interrupt-mask setup.
- Decision: Make the custom armstub and Pi 5 entry marker write through the firmware-preserved RP1 UART0 before changing any PL011 registers, then run the explicit PL011 init and write the existing initialized marker. The armstub now attempts `P0` then `S1`; the kernel entry marker attempts `P1` then `T1`.
- Required validation: Local validation must pass formatting, unit tests, Pi 5 target build, image generation, archive review, and QEMU smoke. Physical validation is a controlled Pi 5 power-cycle with serial and TFTP-log evidence.
- Risks: If no preserved or initialized marker appears while TFTP logs prove the armstub and kernel files were served, the next failure boundary is no longer ordinary archive layout or PL011 initialization. It points toward firmware handoff semantics, custom armstub execution assumptions, or a mismatch between loaded files and executed code.
- Alternatives considered: keep testing more archive layouts, restore older Image header fields, or require Matthew input immediately. The preserved-UART marker is a small code diagnostic that directly tests the remaining UART-reinit hypothesis.

## 2026-05-19 - Use Circle-Style Pi 5 Bare-Metal Kernel Address

- Status: accepted
- Context: After TFTP logs proved the Pi is served Talos' config, kernel, DTB, overlays, cmdline, and custom armstub, serial still stopped before any preserved-UART or initialized-UART marker. The official Raspberry Pi documentation says `kernel` selects `kernel_2712.img` on Pi 5 and `kernel_address` can control the load address. Circle's Pi 5 bare-metal `config64.txt` keeps `kernel_2712.img` and sets `kernel_address=0x80000`.
- Decision: Keep the Pi 5 arm64 Image `text_offset=0` and flags `0xc`, but link Talos at physical `0x80000` and stage `kernel_address=0x80000` in the first-light `config.txt`. Keep QEMU virt on its separate QEMU-specific link base.
- Required validation: Pi 5 target builds must show `_start` and `__kernel_start` at `0x80000`; archive review must require `kernel_address=0x80000`; physical validation requires one controlled hardware run with serial and TFTP evidence.
- Revisit: The later `asm-entry-reset-firmware-address` hardware proof produced a repeated TFTP boot sequence only after removing forced `kernel_address=0x80000`. Treat the Circle-style address as useful raw-binary reference evidence, not as the preferred Image-header path, until a follow-up UART proof says otherwise.
- Risks: This does not explain why the custom armstub marker did not appear; if the next hardware run still stops at the same boundary, the remaining issue is likely firmware handoff semantics or lab-visible UART assumptions rather than ordinary kernel load address.
- Alternatives considered: continue with `kernel_address` omitted and link at zero, add more archive-layout variants, or switch immediately to a Linux-loaded payload. The Circle-style address is a reference-backed, bounded change that can be validated locally before one hardware iteration.

## 2026-05-19 - Add Raw Pi 5 Loader Diagnostic

- Status: accepted
- Context: Circle's Pi 5 bootloader builds `kernel_2712.img` as a raw position-linked binary at `0x80000`, while Talos' normal Pi 5 image starts with an arm64 Image header before branching to code. After Circle-style `kernel_address=0x80000` still produced no marker, Matthew clarified the workflow should keep using bounded reference-backed diagnostics rather than treat the state as blocked.
- Decision: Add a separate raw loader diagnostic path that stages a tiny `kernel_2712.img` without the arm64 Image header. The diagnostic writes markers to firmware-preserved RP1 UART0, reinitialized RP1 UART0, and BCM2712 UART10, then loops with heartbeat dots. The normal Talos kernel image and boot tree remain unchanged.
- Required validation: Local validation must include shell syntax checks, raw diagnostic binary generation, archive review in `loader_diagnostic=true` mode, standard Talos formatting/tests/Pi 5 build/QEMU smoke/mdBook, and exactly one controlled Pi 5 hardware run under the hardware lock.
- Risks: Absence of RP1 UART output from a raw executable does not by itself prove CPU execution never happened, because the lab-visible UART path may be the wrong ARM-side output path despite firmware logs using it. If the raw diagnostic still shows no marker while TFTP proves the 216-byte image was served, the next diagnostic should avoid relying solely on RP1 UART visibility.
- Alternatives considered: keep changing arm64 Image header fields, require EEPROM/vclog support before continuing, or switch immediately to a Linux-loaded payload. The raw loader diagnostic is smaller and directly tests a public Pi 5 bare-metal image shape already used by Circle.

## 2026-05-19 - Make the Raw Loader Diagnostic Exception-Tolerant

- Status: accepted
- Context: The first raw loader diagnostic attempted RP1 UART0 before the BCM2712 UART10 path. If RP1 MMIO was inaccessible after firmware handoff, a synchronous abort could stop the diagnostic before it reached alternate output paths.
- Decision: Install a current-EL exception vector in the raw loader diagnostic before touching MMIO, advance `ELR_EL1` or `ELR_EL2` by one instruction on exceptions, and try BCM2712 UART10 before RP1 UART0. This keeps the diagnostic tiny while making MMIO-abort behavior observable by continued control flow.
- Required validation: Build/disassemble the raw diagnostic, pass archive review in loader-diagnostic mode, pass standard local Talos gates, then run one controlled Pi 5 hardware iteration under `hardwareTestLock`.
- Risks: Skipping faulting MMIO instructions can only keep the diagnostic moving; it cannot make an unobservable UART path visible. If this still emits no marker while TFTP proves the image was served, the next evidence needs a different side effect or a different boot path.
- Alternatives considered: switch immediately to Linux-loaded payload work, keep trying UART-only variants, or require EEPROM/vclog support. Exception-tolerant control flow is a bounded diagnostic improvement that addresses a concrete flaw in the first raw-loader attempt.

## 2026-05-19 - Add Watchdog Reset as a Non-UART Execution Signal

- Status: accepted
- Context: The exception-tolerant raw loader still emitted no UART marker. Circle's Pi 5-capable watchdog/reset path documents the power-manager watchdog registers at `ARM_IO_BASE + 0x1200000` for Pi 5, with writes to `ARM_PM_WDOG` and `ARM_PM_RSTC` causing a reset. A watchdog-triggered second firmware boot would prove CPU execution even if UART output is unavailable.
- Decision: Add a watchdog reset attempt after the raw loader's UART attempts. The hardware test must roll back the archive after observation so a successful watchdog diagnostic does not leave the Pi in a reset loop.

- Required validation: Build/disassemble the raw loader, pass archive review and standard local Talos gates, run exactly one hardware cycle, observe serial long enough for a watchdog reset, inspect TFTP evidence, then restore the previous archive.
- Risks: If the CPU never reaches the raw loader, no watchdog reset occurs. If PM watchdog MMIO is inaccessible or the reset sequence is wrong for this boot state, the result is still no side effect. A successful reset would be useful but requires immediate cleanup.
- Alternatives considered: another UART-only variant, requiring EEPROM/vclog evidence, or switching directly to a Linux-loaded payload. Watchdog reset is a small non-UART side effect available from public Pi references and fits one controlled hardware iteration.

## 2026-05-19 - Try Linux-Derived RP1 UART0 CPU Address First

- Status: accepted
- Context: Matthew clarified that no-UART-output first-light failures should be treated as hardware-contract evidence and that correct Pi 5 offsets should make first UART output simple. A fresh Raspberry Pi Linux reference checkout shows `rp1.dtsi` declaring RP1 UART0 at RP1 bus address `0xc0_40030000`, while `bcm2712.dtsi` maps the pcie2 32-bit non-prefetchable window to CPU physical `0x1f00000000`. That implies the CPU-visible RP1 UART0 address is `0x1f00030000`. Talos had centered first-light diagnostics on `0x1c00030000`, based on earlier firmware-preserved UART evidence, and repeated hardware runs produced no marker.
- Decision: Treat `0x1f00030000` as the Linux-derived RP1 UART0 CPU address for new Pi 5 diagnostics and keep `0x1c00030000` as a fallback firmware-preserved probe. The raw loader now writes `N0`/`N1` through the Linux-derived address before trying the older `L0`/`L1` probes.
- Required validation: Local validation must show the raw diagnostic embeds both RP1 UART0 addresses, standard Talos formatting/tests/build/smoke gates pass, and one controlled Pi 5 hardware run under `hardwareTestLock` records whether `N0`, `N1`, `L0`, `L1`, or other side effects appear.
- Risks: The pcie2 non-prefetchable address may only become valid after a state transition the raw loader has not reached, or the firmware-preserved address may still be the only inherited early mapping. Trying both addresses keeps the experiment bounded.
- Alternatives considered: keep using only `0x1c00030000`, switch to a non-UART side effect immediately, or wait for vclog/EEPROM diagnostics. The source-backed address correction is the smallest productive offset experiment.
- Hardware result: The first controlled run with this diagnostic published archive `b5cb364106dae20de1a61a25fed66ef1df9f36023362ec1c443f34b44205dc90`. TFTP served the updated 4096-byte raw loader, but serial again stopped at the firmware/RP1 boundary with no `N0`/`N1`/`L0`/`L1`/`U1`/`W0` marker and no reset side effect. This rules out the Linux-derived RP1 UART0 address as sufficient by itself; the workflow remains unblocked for the next hardware-contract or handoff diagnostic.

## 2026-05-19 - Try PSCI Reset Before MMIO Watchdog

- Status: accepted
- Context: The watchdog raw loader diagnostic did not produce UART output or a second firmware boot. That leaves two different possibilities: the firmware never transfers CPU execution to the raw loader, or the loader runs in a state where both lab-visible UART and BCM2712 watchdog MMIO are ineffective.
- Decision: Add a PSCI `SYSTEM_RESET` SMC call before the MMIO watchdog reset in the diagnostic loader. This is still a diagnostic-only path and leaves the normal Talos image unchanged.
- Required validation: Build and disassemble the raw loader to confirm the SMC instruction is present, pass archive review and standard Talos local gates, run exactly one hardware cycle under `hardwareTestLock`, observe serial long enough for a possible monitor-mediated reboot, then roll back the archive.
- Risks: PSCI may be unavailable in the firmware handoff state, or the SMC may return without side effects. If no reset occurs, the result is evidence against this specific non-MMIO side channel, not proof that Talos can never execute.
- Alternatives considered: switch immediately to a Linux/UEFI-loaded payload, require EEPROM/vclog support, or keep adding UART-only markers. PSCI reset is a small, reversible diagnostic and tests a different side-effect class than RP1 UART or PM watchdog MMIO.

## 2026-05-19 - Add UEFI Intermediate-Loader Diagnostic

- Status: accepted
- Context: Direct Raspberry Pi firmware handoff has loaded Talos files but has not produced markers through UART, custom armstub, raw loader, watchdog reset, or PSCI reset. A known-running intermediate loader can separate Talos execution mechanics from the Pi firmware handoff boundary.
- Decision: Add a minimal AArch64 UEFI application diagnostic that prints `Talos EFI first-light PASS` through UEFI text output, plus a FAT-image staging script and QEMU/AAVMF smoke test. This creates a locally validated substitute payload for understanding loader behavior without changing the normal Talos kernel image.
- Required validation: The EFI file must be PE32+ AArch64 with EFI application subsystem, the FAT image must contain `EFI/BOOT/BOOTAA64.EFI` and `startup.nsh`, QEMU/AAVMF must print the PASS marker, and standard Talos gates must still pass.
- Risks: QEMU/AAVMF validation proves the payload and UEFI call path, not Pi 5 hardware execution. A physical test should not depend on an external bootloader; if this payload remains useful, Talos should reach it through a Talos-owned loader path.
- Alternatives considered: continue direct firmware diagnostics, require EEPROM/vclog support, or build a Linux/kexec path first. UEFI is useful substitute validation, but it is not the Talos hardware boot path.

## 2026-05-19 - Stage U-Boot as the UEFI Hardware Bridge

- Status: superseded
- Context: The UEFI diagnostic runs under QEMU/AAVMF, but the lab Pi 5 still boots directly from Raspberry Pi firmware into `kernel_2712.img`. A physical UEFI test needs an intermediate loader that can run on Pi 5 and launch `EFI/BOOT/BOOTAA64.EFI`.
- Decision: Superseded by project direction. Talos should develop its own kernel and bootloader from scratch; U-Boot must not be used as an implementation dependency, boot target, compatibility layer, or shortcut.
- Required validation: Remove U-Boot-specific staging from the active tool path. Continue with Talos-owned loader diagnostics, local gates, and one controlled hardware test at a time.
- Risks: External bootloader staging would hide Talos bootloader bugs and move the project away from the from-scratch kernel goal.
- Alternatives considered: use U-Boot as a bridge. Rejected because it does not match the project direction.

## 2026-05-19 - Test Raw Loader Under Circle-Style Minimal Config

- Status: accepted
- Context: Matthew clarified that Talos must own its bootloader path. The existing raw loader is Talos-owned, but previous hardware tests used the Talos first-light config with `enable_rp1_uart=1`, `pciex4_reset=0`, UART debug settings, and extra diagnostic options. Circle's Pi 5 bare-metal config is much smaller.
- Decision: Add a separate raw-loader staging path that keeps the Talos-owned loader binary but uses a Circle-style minimal Pi 5 config: `arm_64bit=1`, `kernel_address=0x80000`, `initial_turbo=0`, `[pi5]`, and `kernel=kernel_2712.img`. This tests the config-shape hypothesis without introducing an external bootloader.
- Required validation: Archive review must identify the diagnostic as `raw-pi5-circle-config`, allow the intentionally omitted RP1-preservation settings only for that diagnostic, and standard local gates must pass before exactly one hardware run under `hardwareTestLock`.
- Risks: Omitting RP1 UART preservation may make loader UART output less likely, so the meaningful hardware side effects are still firmware re-entry/reset evidence and any serial/TFTP movement. No marker still does not prove CPU execution is impossible.
- Alternatives considered: continue with U-Boot staging, repeat the prior first-light config, or wait for EEPROM/vclog evidence. The minimal-config raw loader is the smallest Talos-owned experiment that directly follows from the reference comparison.

## 2026-05-19 - Recombine Raw Loader With Debug Firmware Settings

- Status: accepted
- Context: The Circle-style minimal-config raw loader and the Linux-derived RP1 UART0 probe each stopped at the same firmware/RP1 boundary. The minimal config intentionally omitted the normal Talos first-light debug knobs, while Raspberry Pi firmware documentation says `os_check=0` is appropriate for bare-metal development, `enable_rp1_uart=1` initializes RP1 UART0, `pciex4_reset=0` preserves RP1 state, and `uart_2ndstage=1` plus `sha256=1` can increase firmware logging.
- Decision: Run one controlled hardware test that keeps the latest Talos-owned raw loader, including `0x1f00030000` and `0x1c00030000` UART probes, but stages it through the normal first-light debug config instead of the Circle-style minimal config. This tests whether the missing debug/preservation settings explain the no-marker state without introducing an external bootloader.
- Required validation: Local validation must pass formatting, tests, Pi 5 build, raw marker inspection, archive review, QEMU smoke, mdBook, and diff check before acquiring `hardwareTestLock` for exactly one Pi 5 run.
- Hardware result: Archive `4831f8acdfab1b9303c78f062190e2f149b85363f11b902b28097140bc845ff4` published and power-cycled successfully. Serial advanced `40997->41705` through Raspberry Pi firmware/RP1 logs but still showed no `N0`, `N1`, `L0`, `L1`, `U1`, `W0`, heartbeat, Talos output, or reset side effect during the 85-second observe. The TFTP cursor did not expose a fresh delta for this run, so this result is publish/power-cycle/serial evidence rather than fresh file-load proof. Rollback restored the previous archive.
- Risks: Since no marker appeared even with the debug settings restored, the likely issue is deeper than optional firmware logging or RP1 UART preservation. Repeating config-only variants is now low signal unless new reference evidence identifies a specific setting.
- Alternatives considered: repeat the Circle-style minimal config, switch immediately to a different side effect, or wait for vclog/EEPROM diagnostics. The recombined test was the smallest remaining config-shape hypothesis.

## 2026-05-19 - Cover EL3 Exceptions in Raw Loader Diagnostic

- Status: accepted
- Context: Repeated raw-loader diagnostics reached the same no-marker boundary even after UART address, config, PSCI reset, and watchdog reset variants. The raw loader installed same-EL exception vectors only for EL1 and EL2. If Raspberry Pi firmware enters a diagnostic at EL3, an early MMIO abort could bypass the skip handler before any UART or reset side effect becomes visible.
- Decision: Extend the Talos-owned raw loader diagnostic to install `VBAR_EL3` when `CurrentEL` reports EL3, and teach the skip handler to advance `ELR_EL3` before `eret`. This keeps the diagnostic from depending on an unverified firmware entry exception level.
- Required validation: Build the raw loader, inspect disassembly for `VBAR_EL3` and `ELR_EL3`, run formatting, tests, Pi 5 target build, raw marker inspection, archive review, QEMU smoke, mdBook, and diff check before one controlled hardware run.
- Hardware result: Archive `a1485244630ceef8f74f06d68a23717eab1415d4ba0475010f7b920b520f88d0` was published and power-cycled successfully. Serial advanced from cursor `41705` to `42355` through Raspberry Pi firmware logs with no `N0`, `N1`, `L0`, `L1`, `U1`, `W0`, heartbeat, Talos output, or reset side effect. Recent TFTP logs showed the usual config, DTB, overlay, cmdline, and `kernel_2712.img` requests at the boot time; rollback restored the previous archive.
- Risks: This does not prove the real entry EL. It only removes a plausible diagnostic blind spot. Since no reset side effect appeared, further direct-firmware variants should focus on entry contract or lab-visible side effects rather than more exception-level-only changes.
- Alternatives considered: assume EL1/EL2 per Linux boot protocol, repeat the same raw loader, or wait for EEPROM/vclog diagnostics. The EL3 vector change was small, local, and directly tied to the observed lack of any post-entry side effect.

## 2026-05-19 - Simplify to an Assembly-Only UART Proof

- Status: accepted
- Context: Matthew clarified that first-light should stop accumulating loader complexity until UART text is proven. The next proof should follow Daedalus' Pi 4 boot/UART shape and minimal-OS principles: firmware entry, preserve `x0` if useful, park secondary cores, and write fixed bytes through the simplest plausible UART path.
- Decision: Add a separate `asm-uart-proof` diagnostic image that is only 144 bytes of AArch64 assembly linked at `0x80000`. It preserves `x0` in `x19`, parks non-primary cores using `MPIDR_EL1`, initializes one PL011 path at Linux/Circle's RP1 UART0 physical address `0x1f00030000`, and repeatedly writes `TA\r\n`. It deliberately avoids Rust, stack setup, BSS clearing, exception vectors, PSCI, watchdog, and multi-UART fallback logic.
- Required validation: Local validation must inspect the disassembly for the small entry shape and UART polling loop, confirm the marker bytes exist, pass archive review, formatting, tests, Pi 5 build, QEMU smoke, mdBook, and diff check, then run exactly one hardware test under `hardwareTestLock`.
- Hardware result: Archive `3e405afa92020ca74c02d9e64c2b8f79711be31122b88bd21c1f2d9819f4c17b` published and power-cycled successfully. Serial advanced from cursor `42355` to `43005` through Raspberry Pi firmware logs but showed no repeated `TA` marker. The TFTP cursor did not expose a fresh delta for this run, so evidence is publish/power-cycle/serial only. A repeated rollback check toggled the one-archive rollback back to the 144-byte proof tree; the previous EL3 diagnostic archive `a1485244630ceef8f74f06d68a23717eab1415d4ba0475010f7b920b520f88d0` was republished without power-cycling so the lab boot tree was not left on the tiny proof archive.
- Risks: This is intentionally too small to recover from bad UART assumptions. A no-marker result does not prove firmware never enters the image, but it does show that the simplest direct RP1 UART0 proof still does not reach the lab-visible serial stream.
- Alternatives considered: continue extending the raw loader with exception/reset side effects, add another UART fallback, or use Rust first-light. The point of this decision is to remove those moving parts until a fixed-byte assembly marker works.

## 2026-05-19 - Mirror the Assembly UART Proof Under Serial Prefix

- Status: accepted
- Context: The first assembly-only UART proof used a root-only boot tree and did not expose a fresh TFTP delta, while many prior proven file-load runs used the Pi serial-number-prefixed `da591740/` mirror. Before changing UART code again, the smallest remaining premise was whether the simplified proof needed the same mirrored archive shape in this lab network-boot path.
- Decision: Add a prefixed staging script for the same 144-byte `asm-uart-proof` image. Keep the assembly unchanged and mirror only `config.txt`, `cmdline.txt`, `bcm2712-rpi-5-b.dtb`, `kernel_2712.img`, `kernel8.img`, and overlays under `da591740/`.
- Required validation: Local validation must pass shell syntax, proof image build, disassembly inspection, marker-byte inspection, prefixed archive review, formatting, tests, Pi 5 build, QEMU smoke, mdBook, and diff check before one controlled hardware run.
- Hardware result: Archive `ea6bf1f7c94d7a175ae018872980c0d0d6d2bf7300a99b2dceffa274f4e923e2` published and power-cycled successfully with the mirrored tree. Serial advanced from cursor `43005` to `43655` through Raspberry Pi firmware logs but still showed no repeated `TA` marker. The TFTP cursor again did not expose a fresh delta for this run, so evidence is publish/power-cycle/serial only. A repeated rollback check again toggled the one-archive rollback back to the 144-byte proof tree; the previous EL3 diagnostic archive `a1485244630ceef8f74f06d68a23717eab1415d4ba0475010f7b920b520f88d0` was republished without power-cycling to leave the lab boot tree on the 4120-byte previous diagnostic archive.
- Risks: The no-marker result now covers the simplified assembly proof with and without the serial-prefixed mirror. Since the code remained single-UART by design, the next simplification-compatible experiment should change only the UART base/preservation assumption, not add loader machinery.
- Alternatives considered: switch immediately to the firmware-preserved `0x1c00030000` UART mapping, reintroduce multi-UART fallback, or add reset side effects. Mirroring the archive shape was the smaller single-premise test after the root-only run lacked fresh TFTP evidence.

## 2026-05-19 - Test Assembly UART Proof at Firmware-Preserved RP1 Mapping

- Status: accepted
- Context: The simplified assembly proof did not produce `TA` at Linux/Circle's RP1 UART0 CPU physical address `0x1f00030000`, either root-only or mirrored under `da591740/`. Earlier firmware documentation and boot logs identified a firmware-preserved RP1 UART mapping at `0x1c00030000` when `enable_rp1_uart=1` and `pciex4_reset=0` are used.
- Decision: Keep the assembly-only proof and prefixed archive shape unchanged except for the single UART base literal, switching from `0x1f00030000` to `0x1c00030000`. This preserves Matthew's simplification constraint while testing the strongest remaining reference-backed UART address premise.
- Required validation: Local validation must pass shell syntax, proof image build, disassembly inspection showing the `0x1c` high word literal, marker-byte inspection, prefixed archive review, formatting, tests, Pi 5 build, QEMU smoke, mdBook, and diff check before one controlled hardware run.
- Hardware result: Archive `cf3ec38488a46752a52a27893a73bafecb1daf83590f0e20f17e0c606e131f80` published and power-cycled successfully. Serial advanced from cursor `43655` to `44305` through Raspberry Pi firmware logs but still showed no repeated `TA` marker. The TFTP cursor again did not expose a fresh delta for this run, so evidence is publish/power-cycle/serial only. The previous EL3 diagnostic archive `a1485244630ceef8f74f06d68a23717eab1415d4ba0475010f7b920b520f88d0` was republished without power-cycling afterward, and status confirmed the 4120-byte previous diagnostic tree.
- Risks: This result covers the two main RP1 UART0 base assumptions within the minimal assembly proof. More UART-base churn is low signal unless new reference evidence identifies a different single UART path or missing GPIO/clock step.
- Alternatives considered: add multi-UART fallback back into the proof, add exception/reset side effects, or change archive layout again. The single literal change was the smallest bounded test still aligned with the simplification policy.

## 2026-05-19 - Test Assembly UART Proof on BCM2712 UART10

- Status: accepted
- Context: The 144-byte assembly-only proof did not emit `TA` through RP1 UART0 at either `0x1f00030000` or `0x1c00030000`. Reference notes and the Talos Pi 5 target map identify Raspberry Pi 5 firmware console `serial10` / debug UART as BCM2712 UART10 at `0x107d001000`.
- Decision: Keep the assembly proof, prefixed archive shape, marker, parking loop, and no-stack/no-BSS/no-exception constraint unchanged except for the single UART base literal, switching it to `0x107d001000`. This tests the simplest Talos-owned serial10 hypothesis without adding loader complexity.
- Required validation: Local validation must pass shell syntax, proof image build, disassembly inspection showing the `0x107d001000` literal, marker-byte inspection, prefixed archive review, formatting, tests, Pi 5 build, QEMU smoke, mdBook, and diff check before one controlled hardware run.
- Hardware result: Archive `4099e52e8bc660727dd0081ddd332453e4e0b438b118065ec5c05a575f8c9f82` published and power-cycled successfully. Serial advanced from cursor `44305` to `44955` through Raspberry Pi firmware logs but still showed no repeated `TA` marker. The TFTP follow-up did not show a current 144-byte serial10 image fetch; it showed repeated earlier 4120-byte diagnostic fetches, so the useful evidence for this run is publish/power-cycle/serial rather than fresh file-load proof. The previous EL3 diagnostic archive `a1485244630ceef8f74f06d68a23717eab1415d4ba0475010f7b920b520f88d0` was republished without power-cycling afterward, and status confirmed the 4120-byte previous diagnostic tree.
- Risks: This result argues against the three obvious single-address UART paths being sufficient in the current handoff state. It does not prove the image never executes; it may still indicate a missing UART clock/reset/GPIO mux prerequisite, a firmware handoff mismatch, or lab serial being attached only to RP1 UART0 while serial10 is elsewhere.
- Alternatives considered: reintroduce multi-UART fallback, add reset/exception side effects, or pivot to another loader shape. The serial10 literal change was the last small single-UART experiment before returning to reference review of GPIO, clock, reset, and handoff premises.

## 2026-05-19 - Test Assembly UART Proof With Explicit RP1 GPIO Mux

- Status: accepted
- Context: The single-address assembly proofs covered RP1 UART0 at `0x1f00030000` and `0x1c00030000`, plus BCM2712 UART10 at `0x107d001000`, without a marker. Linux RP1 references show the 40-pin header UART path is RP1 UART0 on GPIO14/GPIO15, with RP1 GPIO control at bus `0xc0_400d0000`, pads at `0xc0_400f0000`, and GPIO14/GPIO15 selecting `uart0` at function select value 4. The corresponding pcie2 CPU physical addresses are `0x1f000d0000`, `0x1f000f0000`, and `0x1f00030000`.
- Decision: Keep the proof assembly-only and single-UART, but add the minimum Linux-derived RP1 pin setup before PL011 initialization: set GPIO14 pad input-enable/no-pull, GPIO15 pad input-enable/pull-up, set both GPIO control registers to function select 4, issue a barrier, then write `TA\r\n` through RP1 UART0 at `0x1f00030000`.
- Required validation: Local validation must inspect the disassembly for the pad/control register literals and PL011 write loop, confirm marker bytes exist, pass prefixed archive review, formatting, tests, Pi 5 build, QEMU smoke, mdBook, and diff check before one controlled hardware run.
- Hardware result: Archive `323e972a2da9bb56552d0c0a6d3abbd742e2f543d2bdbfe5a26cfbd3f29ef479` published and power-cycled successfully. Serial advanced from cursor `44955` to `45667` through Raspberry Pi firmware logs and reached a network-wait line, but no repeated `TA` marker appeared. The TFTP follow-up did not capture a current 224-byte proof-image fetch before rollback; later TFTP evidence showed the restored 4120-byte diagnostic image, so this result is publish/power-cycle/serial evidence rather than fresh file-load proof. The previous EL3 diagnostic archive `a1485244630ceef8f74f06d68a23717eab1415d4ba0475010f7b920b520f88d0` was republished without power-cycling afterward, and status confirmed the 4120-byte previous diagnostic tree.
- Risks: Writing RP1 pad/control registers assumes the pcie2 RP1 window is usable at firmware handoff. If it is not, the proof can fail before the UART write without an exception handler by design. This still tests the smallest GPIO-mux premise while preserving the simplification constraint.
- Alternatives considered: add exception recovery around GPIO writes, add multi-UART fallback, or use a non-UART side effect. Those would reintroduce loader complexity before the GPIO-mux premise had been tested in the minimal proof.

## 2026-05-19 - Capture Full TFTP Cursor Before Hardware Tests

- Status: accepted
- Context: Several recent Pi 5 UART-proof hardware runs published and power-cycled correctly, but their TFTP follow-up deltas showed stale 4120-byte diagnostic fetches instead of the current tiny proof image. Review found the lab endpoint defaults to a 64 KiB `max_bytes` window; `/tftp/logs?cursor=0&limit=1` returned `cursor_end=65536` even though the real EOF cursor was already beyond `215000`.
- Decision: Add `scripts/rpi5-tftp-cursor.sh` and require it, or an equivalent `/tftp/logs?cursor=0&max_bytes=1048576&limit=1` call, to capture the pre-run TFTP EOF cursor before controlled hardware tests. Also use `scripts/rpi5-wait-tftp-delta.sh <cursor>` after power-cycle because `/serial/observe` can return after the first serial burst before the Pi reaches the network/TFTP phase. Treat prior stale or empty deltas as an evidence-collection flaw, not as proof the hardware tests did or did not fetch the current archive.
- Required validation: The helpers must pass shell syntax validation. The cursor helper must return the same current EOF cursor as the expanded TFTP log query before it is used as a hardware-test gate, and the wait helper must be used before rollback in the next hardware run.
- Hardware follow-up: A wait-for-TFTP rerun of the 224-byte RP1 GPIO-mux assembly UART proof captured fresh TFTP events for `da591740/kernel_2712.img` at 224 bytes before rollback. The serial output still stopped at the same firmware/DDR boundary with no `TA` marker, so the current archive is now proven fetched even though the UART proof is still not visible.
- Risks: If the TFTP log grows past 1 MiB, this helper can become truncated again. If that happens, use the endpoint's large-cursor clamp behavior or add a lab API cursor endpoint rather than relying on the default window.
- Alternatives considered: ignore TFTP deltas and use only serial evidence, or continue manually tuning cursor requests per run. A small helper gives repeatable evidence without changing the Talos boot image.

## 2026-05-19 - Add Minimal Image Header to Assembly UART Proof

- Status: accepted
- Context: The wait-for-TFTP hardware run proved the current 224-byte assembly UART proof is fetched as `da591740/kernel_2712.img`, but the lab UART still shows no `TA` marker. Circle's Pi 5 examples use raw binaries, but the Linux arm64 boot ABI and Talos' normal kernel image use an arm64 Image header with magic `ARMd`, size, text offset, and flags.
- Decision: Keep the first-light proof assembly-only and direct-entry, but prepend the minimal arm64 Image header before branching to the existing UART proof code. The header advertises `text_offset=0`, exact image size, flags `0xc`, and magic `ARMd`. This tests a firmware image-contract hypothesis without adding Rust, stack, BSS, exception handling, reset side effects, or loader machinery.
- Required validation: Local validation must inspect the generated header, confirm the marker bytes still exist, pass archive review, mdBook, and diff check before any future hardware run.
- Hardware result: Archive `6fb98f25ed3d43aaf501cd156e8fa523d00ff4f76ee515bcbb44d1a6666079b6` was published and power-cycled successfully. Corrected TFTP evidence captured cursor `351353->352704` with 13 fresh events, including served `da591740/kernel_2712.img` at 288 bytes twice, the prefixed config, DTB, overlays, and cmdline. Serial observed only a trailing NUL/newline from cursor `46965->46967` and no repeated `TA` marker. Rollback restored the previous boot tree with hash `02a1311a6419ca764a2b19b2a34e4ad1b71e74972c6484e7b5620fc8018ec7d9`.
- Risks: If Raspberry Pi firmware happily boots raw binaries, this will not change behavior. If firmware uses the header for placement or validation in this TFTP path, the header may be the missing contract needed before entry.
- Alternatives considered: keep repeating raw 224-byte proof runs, add more UART-side setup, or reintroduce loader diagnostics. The header is a smaller single-premise change now that fresh TFTP evidence proves the tiny image is fetched.

## 2026-05-19 - Test Firmware-Preserved UART Without RP1 GPIO Writes

- Status: accepted
- Context: The 288-byte headered proof is definitely fetched as the selected prefixed `kernel_2712.img`, but it still emits no `TA` marker. That proof writes RP1 pad/control registers before reaching the UART transmit loop. If those pcie2/RP1 MMIO mappings are unavailable or faulting in the firmware handoff state, the proof can die before the lab-visible UART write. Raspberry Pi firmware docs say `enable_rp1_uart=1` initializes RP1 UART0 for 115200 bps bare-metal output, and `pciex4_reset=0` preserves RP1 state.
- Decision: Keep the proof assembly-only and headered, but remove all GPIO mux and PL011 reinitialization writes. The proof now parks secondary cores, uses the firmware-preserved RP1 UART0 base `0x1c00030000`, polls TX-full, and repeatedly writes `TA\r\n`.
- Required validation: Local validation must inspect the disassembly for the absence of RP1 GPIO pad/control literals, the presence of the `0x1c00030000` UART literal, the Image header fields, marker bytes, prefixed archive review, mdBook, and diff check before one controlled hardware run.
- Hardware result: Archive `f839443aae5673677d2cf4f571517347d8efc5b666309b426567191a1c7c05e6` was published and power-cycled successfully. Corrected TFTP evidence captured cursor `352704->354055` with 13 fresh events, including served `da591740/kernel_2712.img` at 152 bytes twice. Serial advanced from `46967->47389` and, unlike the previous headered GPIO proof, reached firmware `Starting OS` messages followed by `NOTICE: BL31: v2.6...`; it still showed no repeated `TA` marker. Rollback restored the previous boot tree hash `02a1311a6419ca764a2b19b2a34e4ad1b71e74972c6484e7b5620fc8018ec7d9`.
- Risks: If the firmware-preserved mapping is not valid for the loaded kernel, this will still produce no marker. The useful distinction is that failure will no longer be attributable to pre-UART GPIO/control writes.
- Alternatives considered: add exception recovery around RP1 MMIO, switch back to raw loader diagnostics, or add non-UART side effects. Those add complexity before this single-premise simplification has been tested with corrected TFTP evidence.

## 2026-05-19 - Remove MPIDR Core Filter From UART Proof

- Status: accepted
- Context: The preserved-UART proof reached the `Starting OS` / BL31 boundary but still emitted no `TA`. The remaining pre-UART logic read `MPIDR_EL1` and parked unless `AFF0 == 0`. If BL31 hands control to a CPU whose low affinity is not zero, or if the handoff state makes that system-register path unsuitable, the proof can silently park before the first UART write.
- Decision: Remove MPIDR filtering and secondary-core parking from the first-light proof. The entry CPU now preserves `x0`, loads the firmware-preserved UART base `0x1c00030000`, and immediately enters the `TA\r\n` write loop.
- Required validation: Local validation must inspect the disassembly for the absence of `mrs MPIDR_EL1`, `wfe`, and the presence of the UART literal, Image header fields, marker bytes, prefixed archive review, mdBook, and diff check before one controlled hardware run.
- Hardware result: Archive `ddf1c574c3470c608f7a2618b69243d4b8012575bb81c40f9f12f2e7e6805160` was published and power-cycled successfully. Corrected TFTP evidence captured cursor `354055->355406` with 13 fresh events, including served `da591740/kernel_2712.img` at 136 bytes twice. Serial observed only a trailing NUL/newline from cursor `47389->47391` and no repeated `TA` marker. Rollback restored the previous boot tree hash `02a1311a6419ca764a2b19b2a34e4ad1b71e74972c6484e7b5620fc8018ec7d9`.
- Risks: If multiple cores actually enter this image, they may all write the same `TA` marker. That is acceptable for first light because any `TA` proves execution; later boot code can reintroduce disciplined CPU parking.
- Alternatives considered: keep the core filter, add exception vectors, or change UART/device-map assumptions again. Removing the filter is the smallest BL31-handoff-sensitive simplification before adding exception machinery.

## 2026-05-19 - Remove PL011 Flag Polling From UART Proof

- Status: accepted
- Context: The no-MPIDR proof was fetched as the selected prefixed `kernel_2712.img`, but still emitted no `TA`. The remaining instruction before the first UART write was a PL011 flag-register read and TX-full branch. Raspberry Pi's Pi 5 documentation says `enable_rp1_uart=1` initializes RP1 UART0 for bare-metal output and `pciex4_reset=0` preserves RP1 state before OS entry, while TF-A's Pi 5 port hands a preloaded BL33 image to a 64-bit EL2 payload using the firmware-provided kernel address.
- Decision: Keep the proof assembly-only, headered, no-stack, no-BSS, no-exception, no-MPIDR, and single-UART, but remove PL011 FR polling. The entry CPU now preserves `x0`, loads the firmware-preserved RP1 UART0 base `0x1c00030000`, and repeatedly writes `TA\r\n` directly to the PL011 data register.
- Required validation: Local validation must inspect the disassembly for the absence of flag-register polling, `MPIDR_EL1`, and `wfe`; confirm the `0x1c00030000` UART literal, Image header fields, marker bytes, prefixed archive review, formatting, tests, Pi 5 build, QEMU smoke, mdBook, and diff check before one controlled hardware run.
- Hardware result: Archive `0fef9e02a43559e85060048eda0a101cb1507f14200c243f9a761c67913a7b1f` was published and power-cycled successfully. Corrected TFTP evidence captured cursor `355406->356564` with 11 fresh events, including served `da591740/kernel_2712.img` at 128 bytes. Serial observed only a NUL/newline from cursor `47391->47393` and no repeated `TA` marker. A retry during publish displaced the one-archive rollback slot, so the previous 4120-byte EL3 diagnostic tree was republished without another power-cycle; status confirmed restored tree hash `02a1311a6419ca764a2b19b2a34e4ad1b71e74972c6484e7b5620fc8018ec7d9`.
- Risks: Direct writes can overrun a real PL011 FIFO if the UART is not ready, but the marker is only four repeated bytes and first-light needs any lab-visible byte more than disciplined throughput. If this still produces no marker, the next premise moves further away from UART polling and toward BL31/kernel-entry address, cache visibility, or firmware-preserved UART mapping.
- Alternatives considered: add exception vectors around the UART read, switch back to RP1 GPIO setup, or add a non-UART side effect. Removing the read is the smaller single-premise simplification and keeps the proof aligned with Matthew's assembly-only first-light constraint.

## 2026-05-19 - Add Entry Reset Proof To Separate BL33 Entry From UART

- Status: accepted
- Context: The simplified assembly UART proof has now been fetched as the selected `kernel_2712.img` with an arm64 Image header, no GPIO writes, no PL011 reinitialization, no MPIDR filter, and no PL011 flag polling, but no `TA` marker appears. Raspberry Pi and TF-A references say the Pi 5 firmware preloads a 64-bit BL33 image and BL31 enters it at the firmware-provided kernel address. The next useful question is whether BL33 entry happens at all, independent of RP1 UART.
- Decision: Add a separate 96-byte assembly-only `asm-entry-reset-proof` diagnostic. It preserves `x0`, performs no UART or other MMIO, and repeatedly invokes PSCI `SYSTEM_RESET` with `smc #0`. If BL31 enters the image, the board should reboot again; if no second boot appears after a fresh TFTP fetch, the failure boundary shifts toward BL31/kernel-entry semantics rather than UART availability.
- Required validation: Local validation must inspect the disassembly for `smc #0`, absence of UART/MMIO writes, arm64 Image header fields, prefixed archive review, shell syntax, formatting, tests, Pi 5 build, QEMU smoke, mdBook, and diff check before one controlled hardware run. Hardware acceptance requires fresh TFTP service of the 96-byte `da591740/kernel_2712.img` and evidence of a second firmware boot or repeated TFTP sequence without another lab power-cycle.
- Hardware result: Archive `6429ebf9ca33b4443dd75719a5babccfc1ddfade78ec551977685be9ed20d4f4` was published and power-cycled successfully. A pre-run boot snapshot `pre-entry-reset-20260519231251` was captured and restored afterward. Corrected TFTP evidence captured cursor `363319->364670` with 13 fresh events, including served `da591740/kernel_2712.img` at 96 bytes twice. Serial advanced `48045->48265` through one firmware/RP1 boot fragment and no Talos output. No repeated TFTP sequence or clear second firmware boot appeared before restore; status confirmed the previous 4120-byte diagnostic tree hash `02a1311a6419ca764a2b19b2a34e4ad1b71e74972c6484e7b5620fc8018ec7d9` after restore.
- Risks: A successful reset proof may create a short reset loop until rollback, so the hardware run must be single-attempt, bounded, and rolled back promptly. It does not prove UART works; it only proves BL33 execution.
- Alternatives considered: continue UART address/setup variants, add exception vectors around UART writes, or return to raw-loader complexity. The reset proof is smaller than those options and tests the entry premise without depending on BCM2712/RP1 MMIO.

## 2026-05-19 - Test Entry Reset Proof Without Forced Kernel Address

- Status: accepted
- Context: The no-MMIO entry reset proof was fetched twice at 96 bytes but did not produce a visible reset side effect. Current Talos diagnostic staging forces `kernel_address=0x80000` while advertising arm64 Image `text_offset=0`. Public Pi 5 boot logs commonly describe the firmware loading a 64-bit Image at base plus offset and relocating it to `0x80000`, so forcing the address may be a bad interaction with the firmware/BL31 entry calculation.
- Decision: Add a separate `asm-entry-reset-firmware-address` staging path that keeps the same 96-byte Image-header reset proof and serial-prefixed mirror but removes `kernel_address=0x80000` from `config.txt`. This lets the Pi 5 firmware choose the kernel placement while preserving the no-UART/no-MMIO entry discriminator.
- Required validation: Local validation must prove `kernel_address` is absent from the staged config, the 96-byte Image header remains valid, archive review passes, shell syntax passes, and the standard Talos gates pass before one controlled hardware run.
- Hardware result: Archive `75e2dac0e3fa75bd2866c655cade5abbb801e78aaed32fd13f2a163f7cc179dc` was published under pre-run snapshot `pre-fwaddr-reset-20260519232742` and power-cycled successfully. The first captured TFTP delta `421412->422763` served the no-`kernel_address` config at 175 bytes and `da591740/kernel_2712.img` at 96 bytes twice. A second TFTP boot sequence appeared at 23:28:23/24 before the snapshot restore request, satisfying the narrow BL33-entry/reset-side-effect acceptance criterion. The lab was restored afterward to tree hash `02a1311a6419ca764a2b19b2a34e4ad1b71e74972c6484e7b5620fc8018ec7d9` with the previous 4120-byte diagnostic image. Later TFTP metadata from the merged query reflected restored file sizes for historical lines, so the first pre-restore delta is the authoritative file-size evidence.
- Risks: If the firmware loads the image somewhere other than the linked `0x80000` address and does not enter the header branch position-independently, this may still fail. The proof's code path itself is position-independent aside from the literal-pool load of the PSCI function ID, so it should tolerate normal Image placement.
- Alternatives considered: keep repeating `kernel_address=0x80000`, change Image `text_offset` to `0x80000`, or add more UART instrumentation. Removing the forced address is the smaller single-premise test because the official Pi 5 Image convention uses `text_offset=0`.

## 2026-05-19 - Retry Assembly UART Proof With Firmware-Selected Image Placement

- Status: accepted
- Context: The no-MMIO reset proof produced a repeated TFTP boot sequence only when `kernel_address=0x80000` was removed from the Pi 5 config, which strongly suggests BL31 reaches an Image-header payload under firmware-selected placement. The prior UART proofs used forced Circle-style address staging, so they did not test the same handoff contract as the successful reset discriminator.
- Decision: Add `asm-uart-proof-firmware-address`, a serial-prefixed staging path that keeps the smallest current assembly-only UART proof but removes `kernel_address=0x80000` from `config.txt`. This preserves the known-good Image placement contract while testing whether the firmware-preserved RP1 UART0 marker appears.
- Required validation: Local validation must prove `kernel_address` is absent, the archive contains the serial-prefixed mirror, Image header fields remain valid, and the normal formatting/test/build/doc gates pass before one controlled hardware run.
- Hardware result: Archive `2af5611a7fdb49821c7488700e60b452390a6a29cdb7a160d385f5a36410f9f9` was published under pre-run snapshot `pre-fwaddr-uart-20260519234409` and power-cycled successfully. Published status showed no-`kernel_address` config at 174 bytes and `da591740/kernel_2712.img` at 128 bytes. TFTP evidence from cursor `492185->492649` served the 128-byte kernel and companion DTB/overlays. Serial advanced `50237->50945` through Raspberry Pi firmware and RP1 boot text, but no repeated `TA` marker appeared. The lab was restored to tree hash `02a1311a6419ca764a2b19b2a34e4ad1b71e74972c6484e7b5620fc8018ec7d9`.
- Risks: This still depends on the firmware-preserved UART path at `0x1c00030000`, so a no-marker result would not disprove BL33 entry; it would move the next iteration back to UART routing, pinmux, or exception-state evidence under the now-working Image placement.

## 2026-05-19 - Retry RP1 GPIO-Mux UART Proof With Firmware-Selected Image Placement

- Status: accepted
- Context: Firmware-selected Image placement visibly executes the reset proof, but the matching direct-write UART proof produced no marker through firmware-preserved RP1 UART0 at `0x1c00030000`. Earlier GPIO-mux UART work used forced `kernel_address=0x80000`, so it did not test the now-working Image placement.
- Decision: Keep the proof assembly-only and no-`kernel_address`, add only the Linux-derived RP1 GPIO14/GPIO15 pad/control writes for UART0, then directly write `TA\r\n` to RP1 UART0 at `0x1f00030000`. Do not reintroduce PL011 reinitialization, flag polling, MPIDR filtering, or fallback UART paths.
- Required validation: Local validation must inspect the proof image for the RP1 pad/control/UART literals, no `kernel_address` in the staged config, valid Image header fields, the serial-prefixed mirror, and normal formatting/test/build/doc gates before one controlled hardware run.
- Hardware result: Archive `061191ba3d3b9993cfdc29ccc388ce600950b2cd413dc8dbcf647e3646bd4033` was published under pre-run snapshot `pre-fwaddr-gpio-uart-20260519235644` and power-cycled successfully. Published status showed no-`kernel_address` config at 174 bytes and `da591740/kernel_2712.img` at 208 bytes. TFTP evidence from cursor `546689->548040` served the 208-byte kernel twice plus config, DTB, overlays, and cmdline. Serial advanced `50945->52445` through firmware/RP1 logs, `Starting OS`, and BL31 notices, but no repeated `TA` marker appeared. The lab was restored to tree hash `02a1311a6419ca764a2b19b2a34e4ad1b71e74972c6484e7b5620fc8018ec7d9`.
- Risks: If the RP1 pcie2 window is unavailable at BL33 handoff, the GPIO writes can fault before any marker. That is acceptable for this single-premise test because the reset proof already established a non-UART execution side effect under the same Image placement.

## 2026-05-20 - Localize UART Stores With Reset Side Effect

- Status: accepted
- Context: Firmware-selected Image placement executes the no-MMIO reset proof, while both firmware-preserved UART0 and GPIO-mux UART0 direct-write proofs emit no `TA` marker. The remaining ambiguity is whether UART MMIO stores stop execution or whether code continues but the lab-visible UART path is not routed/clocked/transmitting.
- Decision: Add `asm-uart-then-reset-firmware-address`, a 136-byte assembly-only Image that removes forced `kernel_address`, writes `TA\r\n` once to firmware-preserved RP1 UART0 at `0x1c00030000`, then invokes PSCI `SYSTEM_RESET` in a loop. A repeated TFTP boot sequence after the single power cycle is the non-UART side effect proving execution continued past the UART stores.
- Required validation: Local validation must prove the Image header size/flags, no forced `kernel_address`, serial-prefixed mirror, UART literal, marker bytes, and PSCI reset instruction before one controlled hardware run.
- Hardware result: Archive `9c830903652c9662f99eccc282e1b18459f65bd61ed0c0aef995fcbd70d4e829` was published under snapshot `pre-uart-then-reset-20260520001342` and power-cycled successfully. TFTP evidence captured cursor `548040->552093` with 39 fresh events, including three repeated boot sequences serving `da591740/kernel_2712.img` at 136 bytes. Serial advanced `52445->53093` through early firmware/DDR logs and showed no `TA` marker. The lab was restored to tree hash `02a1311a6419ca764a2b19b2a34e4ad1b71e74972c6484e7b5620fc8018ec7d9`.
- Rationale: The reset side effect proves the UART stores did not halt or trap before PSCI reset. The no-marker condition is now localized to UART visibility: routing, pinmux, reset/clock state, line selection, or register path semantics, rather than firmware file load, BL33 entry, Image placement, or a hard abort on the direct UART write.
- Risks: The diagnostic creates a reset loop until restored, so hardware use must remain single-run, bounded, and snapshot-restored promptly.

## 2026-05-20 - Flush RP1 Posted Writes For Pi 5 First Light

- Status: accepted
- Context: The UART-then-reset proof showed execution continued past RP1 UART stores, which left UART visibility rather than BL33 entry as the failure boundary. Linux's RP1 pinctrl path reads back RP1 registers after writes, consistent with posted MMIO writes across the RP1 PCIe window. The lab cable observes the 40-pin header, so RP1 UART0 on GPIO14/GPIO15 remains the target first-light UART; BCM2712 `uart10` is only the separate debug-UART connector.
- Decision: Keep the first-light image assembly-only and firmware-selected with no Rust, stack, BSS, exceptions, PSCI, or MPIDR filtering. Configure RP1 GPIO14/GPIO15 for UART0, then flush each RP1 pin-control write with a readback. After each UART data-register write, read the PL011 flag register to force posted writes through the RP1 path.
- Required validation: Local validation must prove a valid arm64 Image header, absence of forced `kernel_address`, expected `uart10`, RP1 pad/control, and RP1 UART0 literals, readbacks after RP1 writes, archive review, formatting/tests/Pi 5 build/QEMU smoke, mdBook, and diff check before treating the proof as the accepted first-light baseline.
- Hardware result: The 272-byte proof was published to the lab and power-cycled successfully. TFTP served the selected image, serial reached BL31 handoff, and the 40-pin header printed repeated `TA` markers. A later non-destructive serial peek still showed repeated `TA` after BL31. The lab was intentionally left serving/running this successful proof to preserve first-light evidence.
- Rationale: This is the first accepted Talos Pi 5 first light. The working premise is that RP1 posted writes, not wrong UART routing or failed BL33 entry, caused the prior silent proofs.
- Risks: The proof is still a diagnostic, not the final kernel path. The next implementation step should carry the readback-flush rule into the Talos-owned Pi 5 UART/MMIO abstraction before layering normal Rust boot code back on top.

## 2026-05-20 - Treat High-Volume Serial Cursor Evidence As Inconclusive

- Status: accepted
- Context: Post-stack diagnostics used repeated marker loops to localize the Pi 5 bring-up boundary. A re-run of the known-good `SP` transition diagnostic initially looked silent through `/serial/observe`, but a later `/serial/peek` showed a large backlog of `SP`-shaped output. Repeated `/serial/peek?drain=true` calls continued returning the same cursor and same high-volume sample, so the serial cursor/drain contract is unreliable once a marker loop floods the lab serial buffer.
- Decision: Do not classify high-volume repeated-marker hardware tests from `/serial/observe` alone. For repeated marker loops, require a post-run tail/peek review that searches for the marker text, and treat stale marker backlogs as an evidence-collection fault rather than a Talos execution result. Prefer future diagnostics that emit bounded, distinctive marker bursts plus a second side effect, so stale serial backlog cannot mask the current run.
- Required validation: Before accepting or rejecting the next hardware diagnostic, capture the published boot tree hash and kernel size, power-cycle result, and a serial sample that is either freshly bounded or demonstrably not stale. If the lab API cursor remains fixed while `drain=true` returns data, record the run as serial-inconclusive.
- Risks: Some recent no-marker classifications for high-volume loops may be weaker than originally recorded. This does not invalidate the accepted first-light proof or local build evidence, but it means further post-stack claims should avoid depending on the current cursor semantics until the lab serial endpoint is repaired or a bounded-marker protocol is used.
- Alternatives considered: continue trusting `/serial/observe` NUL/LF returns, or stop Talos bring-up until the lab API is fixed. The first would overstate evidence; the second is unnecessary while bounded-marker and non-serial side-effect diagnostics can still reduce uncertainty.

## 2026-05-20 - Use Reset Side Effects For Post-Stack Boundary Validation

- Status: accepted
- Context: The Pi 5 lab serial endpoint can retain stale high-volume marker backlog, so post-stack marker loops are not enough by themselves to classify execution progress. The bounded `RS` diagnostic preserves the normal Cargo/Rust-linked 83,304-byte Image, emits eight post-stack marker bursts with the proven helper shape, then invokes PSCI `SYSTEM_RESET`. The first hardware window showed the archive was fetched but did not wait long enough to prove a reset-induced second boot sequence.
- Decision: For post-stack and Rust-entry boundary work, prefer bounded markers plus a non-serial side effect, currently PSCI reset observed as a delayed fresh TFTP boot/fetch sequence. Treat the delayed TFTP sequence, not stale serial backlog, as the acceptance signal for crossing a boundary when serial freshness is suspect.
- Hardware result: The delayed `RS` run published archive `763f0563f8b16cdf08b711307742d4fa40f71e073b5dbf10fb7a42fe3c79566a` under snapshot `pre-post-stack-reset-delayed-20260520055534`. The initial TFTP cursor `624311->625662` captured the normal first boot. The delayed cursor `625662->628364` captured 26 fresh events, including four 83,304-byte `da591740/kernel_2712.img` serves at 05:56:15 and 05:56:32/05:56:33 UTC. The lab was restored to accepted first-light proof tree hash `da58994fc0492a8cfee9c7b081c49a4cd0a15f7ed97a0508e0035a9671d5e102`.
- Rationale: A repeated TFTP boot/fetch sequence after the bounded marker point proves execution reached the PSCI reset side effect even when the 40-pin serial stream cannot be trusted as fresh. This keeps hardware iteration bounded and avoids blocking Talos bring-up on lab serial API fixes.
- Required validation: Each reset-side-effect diagnostic must have local disassembly evidence for the marker and `smc #0`, one active hardware run under `hardwareTestLock`, an initial and delayed TFTP cursor window, post-run restore evidence, and a post-hardware review classifying serial separately from TFTP side-effect evidence.
- Risks: A reset loop is disruptive if left staged, so every run must snapshot before publish and restore immediately after delayed evidence capture. TFTP repeats prove control reached reset, but they do not prove marker bytes were visible on the target UART.

## 2026-05-20 - Validate Rust Entry With Reset Side Effect

- Status: accepted
- Context: The post-stack reset diagnostic proved execution reaches the assembly boundary immediately before `rust_entry`, but serial output is currently unreliable after high-volume marker loops. The next question is whether control crosses into Rust at all, without depending on fresh UART text.
- Decision: Change the Rust-entry diagnostic to emit eight bounded `RI` marker bursts from inline assembly at the start of `rust_entry`, then invoke PSCI `SYSTEM_RESET` in a loop. Keep the normal Cargo/Rust-linked arm64 Image path and serial-prefixed boot tree.
- Required validation: Local validation must pass archive review, disassembly inspection for the bounded `RI` loop and `smc #0`, formatting, unit tests, QEMU smoke, mdBook, and diff check. Hardware validation requires one controlled Pi 5 run under `hardwareTestLock`, TFTP evidence that the diagnostic kernel was served, post-run restore evidence, and a repeated boot/fetch sequence as the acceptance signal.
- Hardware result: Archive `c96f10cd4dad5c382804b079c1b78300e659bd406cb806e3a772b118738ca964` with 69,728-byte kernel `b6d4ef09bc6e1d20683d535847ee19fb3150d60eec8605184a12c96bc5d4e44e` was published under snapshot `pre-rpi5-rust-entry-reset-20260520061332` and power-cycled successfully. Initial TFTP cursor `629715->631066` served the diagnostic `da591740/kernel_2712.img` twice. After restoring the snapshot, a later TFTP query showed cursor `629715->632417` with two additional accepted first-light proof boot sequences serving the restored 272-byte kernel, without another lab power-cycle. Serial remained stale/inconclusive. The lab status confirmed restore to tree hash `da58994fc0492a8cfee9c7b081c49a4cd0a15f7ed97a0508e0035a9671d5e102`.
- Rationale: The post-restore boot/fetch sequence can only happen if the diagnostic reached a reset side effect after the first diagnostic fetch. This accepts the Rust-entry boundary at TFTP-side-effect level and moves the next boundary to Rust code after the inline entry marker/reset block.
- Risks: The run restored earlier than the planned delayed-cursor script step, so the reset-induced sequence fetched the restored accepted proof rather than the diagnostic again. That is acceptable evidence for reaching reset, but future runs should keep the delayed query and restore ordering explicit in the automation to avoid ambiguity.

## 2026-05-20 - Validate BootInfo Construction With Reset Side Effect

- Status: accepted
- Context: The Rust-entry reset diagnostic proved control can cross into `rust_entry`, but the reset side effect was still inside an inline assembly block before any real Rust work. The next bounded boundary is whether Talos can execute the first Rust initialization step, `BootInfo::from_aarch64_x0`, before a non-serial reset side effect.
- Decision: Move the bounded Rust-entry reset probe after `BootInfo` construction. The diagnostic now emits eight bounded `RB` marker bursts and then invokes PSCI `SYSTEM_RESET`, keeping the normal Cargo/Rust-linked image and serial-prefixed boot tree.
- Required validation: Local validation must pass archive review, disassembly inspection showing `rust_entry` calls `BootInfo::from_aarch64_x0` before the reset probe, bounded `RB` marker writes, `smc #0`, formatting, unit tests, QEMU smoke, mdBook, and diff check. Hardware validation requires one controlled Pi 5 run under `hardwareTestLock`, TFTP evidence for the diagnostic kernel, delayed repeated boot/fetch evidence before restore, and post-run restore evidence.
- Hardware result: Archive `1c7bc710783ddfc36ea53f808639d0a152d108c9129ba07eae6871e42825992d` with 69,728-byte kernel `38461bc110e2acaadedc58e67f3bda9146c5c04da39410b9bd62b42125edc1e3` was published under snapshot `pre-rpi5-rust-bootinfo-reset-clean-20260520062828` and power-cycled successfully. TFTP cursor `635119->640523` captured 52 fresh events, including four boot/fetch sequences and eight served `da591740/kernel_2712.img` events at 69,728 bytes before restore. The lab was restored to accepted first-light proof tree hash `da58994fc0492a8cfee9c7b081c49a4cd0a15f7ed97a0508e0035a9671d5e102`.
- Rationale: Repeated diagnostic kernel fetches before restore prove the post-`BootInfo` reset side effect executed. This accepts `BootInfo::from_aarch64_x0` at TFTP-side-effect hardware level and moves the next boundary to `target::init` or exception setup.
- Risks: Serial remains stale/inconclusive, so this result proves the non-serial reset side effect, not UART marker visibility or normal console output. The failed/incomplete earlier attempts in this run were automation-ordering artifacts and were restored before the accepted clean run.

## 2026-05-20 - Validate Target Init With Reset Side Effect

- Status: accepted
- Context: The BootInfo reset diagnostic proved Talos can execute `BootInfo::from_aarch64_x0` on Pi 5 hardware. The next bounded boundary is `target::init`, which runs the Pi 5 RP1 UART0 GPIO pad/control writes with posted-write flushes before any exception-vector setup or normal console formatting.
- Decision: Move the bounded reset probe after `target::init(&boot_info)`. The diagnostic now emits eight bounded `RT` marker bursts and invokes PSCI `SYSTEM_RESET`, still using the normal Cargo/Rust-linked image and serial-prefixed boot tree.
- Required validation: Local validation must pass archive review, disassembly inspection showing `rust_entry` calls `BootInfo::from_aarch64_x0`, then `target::init`, then the reset probe; the probe must contain bounded `RT` marker writes and `smc #0`. Formatting, unit tests, QEMU smoke, mdBook, and diff check must pass before one hardware run.
- Hardware result: Archive `d15ad68e95b2ef8a0d8bb7f3d4a0b1d48d4429d119f34b2be9fabcfeb6228b4e` with 74,344-byte kernel `d674b82557e05c0c1ca95fbc1341ebcee6bf13905ab747aa145454eae95cbd6b` was published under snapshot `pre-rpi5-rust-target-init-reset-20260520064040` and power-cycled successfully. TFTP cursor `643225->648629` captured 52 fresh events, including four boot/fetch sequences and eight served `da591740/kernel_2712.img` events at 74,344 bytes before restore. The lab was restored to accepted first-light proof tree hash `da58994fc0492a8cfee9c7b081c49a4cd0a15f7ed97a0508e0035a9671d5e102`.
- Rationale: Repeated diagnostic kernel fetches before restore prove the post-`target::init` reset side effect executed. This accepts the RP1 pin setup path at TFTP-side-effect hardware level and moves the next boundary to exception-vector setup.
- Risks: This does not prove the normal Rust console output is usable; serial remains unreliable and was not used as the acceptance signal. The reset-loop diagnostic must remain bounded by snapshot/restore.

## 2026-05-20 - Validate Exception Vector Setup With Reset Side Effect

- Status: accepted
- Context: The target-init reset diagnostic proved Talos can execute the Pi 5 target initialization path on hardware. The next bounded Rust boundary is `arch::aarch64::exceptions::init()`, which installs the exception vector base with `VBAR_EL1` and `isb` before normal console formatting.
- Decision: Move the bounded reset probe after `arch::aarch64::exceptions::init()`. The diagnostic emits eight bounded `RX` marker bursts and invokes PSCI `SYSTEM_RESET`, still using the normal Cargo/Rust-linked image and serial-prefixed boot tree.
- Required validation: Local validation must pass archive review, disassembly inspection showing `rust_entry` calls `BootInfo::from_aarch64_x0`, `target::init`, `arch::aarch64::exceptions::init`, then the reset probe; the exception path must show the `VBAR_EL1` write and `isb`, and the probe must contain bounded `RX` marker writes plus `smc #0`. Formatting, unit tests, QEMU smoke, mdBook, and diff check must pass before one hardware run.
- Hardware result: Archive `d64f1f12631adaa830030a631f4d267c05d72e2eee0a905186d98a8f1bcc5d9d` with 74,344-byte kernel `30a62bfbc721fc95021523286cb6d2ca7a22dcd071277536b5ed3441fa7a6625` was published under snapshot `pre-rpi5-rust-exceptions-reset-20260520065638` and power-cycled successfully. TFTP cursor `651331->654033` captured 26 fresh events, including two boot/fetch sequences and four served `da591740/kernel_2712.img` events at 74,344 bytes before restore. The lab was restored to accepted first-light proof tree hash `da58994fc0492a8cfee9c7b081c49a4cd0a15f7ed97a0508e0035a9671d5e102`.
- Rationale: Repeated diagnostic kernel fetches before restore prove the post-exception-vector reset side effect executed. This accepts exception-vector setup at TFTP-side-effect hardware level and moves the next boundary to the first normal Rust console print path.
- Risks: This still does not prove normal console output; serial remains stale/inconclusive and was not used as the acceptance signal. The reset-loop diagnostic must remain bounded by snapshot/restore.

## 2026-05-20 - Validate Console Print Entry With Reset Side Effect

- Status: accepted
- Context: Exception-vector setup is accepted, so the next boundary is the first normal Rust console path. A first attempt placed the reset probe after the initial `println!()` in `kernel_main`; that 74,344-byte archive was fetched twice at TFTP cursor `656735->658086` but produced no delayed reset-induced boot sequence, so the first `println!()` did not return.
- Decision: Move the bounded reset probe to the entry of `target::console::_print`, before `console().write_fmt(args)`, using `RF` marker bursts and PSCI `SYSTEM_RESET`. This separates the macro/`fmt::Arguments` path from the console writer path without depending on fresh serial output.
- Required validation: Local validation must pass archive review, disassembly inspection showing the first `kernel_main` print calls `target::console::_print`, and `_print` immediately calls the bounded reset probe before `write_fmt`; the probe must contain bounded `RF` marker writes plus `smc #0`. Formatting, unit tests, QEMU smoke, mdBook, and diff check must pass before one hardware run.
- Hardware result: Archive `e69d2743179e21787a42218d68dbd4ecaeb9884b1f7dc52e8c78ce9ce0c02322` with 78,528-byte kernel `ebccaba708fdce60ef0ec2ed9a722347a710367ec176fd9843fa362d8c9e12ba` was published under snapshot `pre-rpi5-print-entry-reset-20260520071625` and power-cycled successfully. TFTP cursor `658086->664841` captured 65 fresh events, including five boot/fetch sequences and ten served `da591740/kernel_2712.img` events at 78,528 bytes before restore. The lab was restored to accepted first-light proof tree hash `da58994fc0492a8cfee9c7b081c49a4cd0a15f7ed97a0508e0035a9671d5e102`.
- Rationale: Repeated diagnostic kernel fetches before restore prove the first `println!()` reaches `target::console::_print` and executes the pre-`write_fmt` reset side effect. Combined with the failed post-`println!()` run, the current boundary is inside the first `write_fmt`/PL011 write path, likely before it returns.
- Risks: This accepts only the print-entry boundary. It does not prove `fmt::Write::write_fmt`, `Pl011::write_str`, or `Pl011::write_byte` can complete on Pi 5 hardware. Serial remains stale/inconclusive and was not used as the acceptance signal.

## 2026-05-20 - Validate PL011 WriteFmt Entry With Reset Side Effect

- Status: accepted
- Context: The print-entry diagnostic proved the first `println!()` reaches `target::console::_print`, while the first post-`println!()` return diagnostic did not reset. The next boundary is whether dynamic dispatch reaches the Pi 5 `Pl011` formatter method before it tries to emit bytes.
- Decision: Override `fmt::Write::write_fmt` for `Pl011` under the Pi 5 diagnostic cfg and put the bounded reset probe at the start of that method, before delegating to `fmt::write(self, args)`. A companion `Pl011::write_str` entry probe was tried first and did not reset, which means the first normal print reaches `write_fmt` but not `write_str`.
- Required validation: Local validation must pass archive review, disassembly inspection showing `target::console::_print` calls `Pl011::write_fmt`, `Pl011::write_fmt` immediately calls the bounded reset probe, and the failed `write_str` diagnostic placed its probe at `Pl011::write_str` entry. The accepted probe must contain bounded `RM` marker writes plus `smc #0`. Formatting, unit tests, QEMU smoke, mdBook, and diff check must pass before hardware runs.
- Hardware result: The failed `write_str` entry archive `4ad33f57b02097689191aa6740e2bd80d812dd433edce32ce273a2b39e2bd17a` with 83,280-byte kernel `b33e8b752aaa78c3941306854a943349868f461b22c3b6709715eea8dd4503bb` was fetched twice at TFTP cursor `667543->668894` but did not produce a delayed reset-induced boot sequence. The accepted `write_fmt` entry archive `a2a48983784f078e1d9d8cbe440f48b397f30cdfe73ab6555f6e7fa4a88b0296` with 78,624-byte kernel `8d2d802221fef69291b89779db44f25f8b924047864b58ef06af74627fc5b3a6` was published under snapshot `pre-rpi5-write-fmt-reset-20260520072852` and power-cycled successfully. TFTP cursor `668894->675649` captured 65 fresh events, including five boot/fetch sequences and ten served `da591740/kernel_2712.img` events at 78,624 bytes before restore. The lab was restored to accepted first-light proof tree hash `da58994fc0492a8cfee9c7b081c49a4cd0a15f7ed97a0508e0035a9671d5e102`.
- Rationale: Repeated diagnostic kernel fetches before restore prove the first normal print reaches `Pl011::write_fmt`. The failed `write_str` entry probe narrows the current boundary to inside `fmt::write` before it calls `Pl011::write_str` for the first newline-only print.
- Risks: This still does not prove normal console text can be emitted. The next diagnostic should move inside the formatting delegation path or avoid the generic formatter for the first newline, then retest the PL011 byte path.

## 2026-05-20 - Validate PL011 TXFF Poll Boundary

- Status: accepted
- Context: The `Pl011::write_fmt` diagnostic proved the first normal print reaches the formatter method, while the `write_str` entry diagnostic did not reset. The next useful split was to bypass generic formatting and test the Pi 5 PL011 byte path directly from `target::console::_print`.
- Decision: Add a diagnostic-only direct-byte path in `_print`. The first version called `Pl011::write_byte` for CR/LF before the reset probe; it did not produce a reset. The second version called a diagnostic `Pl011::write_byte_unchecked` that writes the data register without polling `UART_FR_TXFF`, then invokes the bounded reset probe with `RU` marker bursts.
- Required validation: Local validation must pass archive review, disassembly inspection showing `_print` calls the direct byte helper before the reset probe, bounded marker writes plus `smc #0`, formatting, unit tests, QEMU smoke, mdBook, and diff check. Hardware validation requires one active Pi 5 run per hypothesis under `hardwareTestLock`, corrected TFTP cursor evidence, and snapshot restore.
- Hardware result: The direct `write_byte` archive `566bb621b86a24b96ecd8123a05ce2594a985b93c3f658f9498aab0ffc322f9a` with kernel `4a82327c0640492a5f18add4220542aa308769f8d3dc9dd1864d346ee7fbadf1` / 78,552 bytes was published under snapshot `pre-rpi5-direct-byte-reset-20260520074513`. Corrected TFTP cursor `675649->679702` captured the post-power-cycle boot/fetch pair but no delayed reset-induced repeat before restore, so the polled byte path did not reach reset. The unpolled archive `4b91b94a52cd85687e7d4e15cd8e196bcfc99382160d827821bbc8530352dbe6` with kernel `aa86634fac70cef50f802248828b03e503e9d79fa0106f11866aa70a6380b53b` / 78,552 bytes was published under snapshot `pre-rpi5-unpolled-byte-reset-20260520074935`; corrected TFTP cursor `679702->685106` captured 52 fresh events, including four boot/fetch sequences and eight diagnostic kernel serves before restore. The lab was restored to accepted first-light proof tree hash `da58994fc0492a8cfee9c7b081c49a4cd0a15f7ed97a0508e0035a9671d5e102`.
- Rationale: The unpolled data-register path reaches PSCI reset, while the polled `write_byte` path does not. That isolates the first normal console hang to the `UART_FR_TXFF` polling loop on the Pi 5/RP1 firmware-preserved UART path, not to Rust formatting, `write_str`, or data-register writes themselves.
- Risks: This result proves the diagnostic path, not yet the normal banner. The next implementation should make the Pi 5 early firmware console use a no-TXFF-poll writer or a bounded poll fallback, then validate the first `println!()` return path.

## 2026-05-20 - Use Static Pi 5 Early Console Path

- Status: accepted
- Context: The TXFF-poll diagnostic isolated the first console hang to the RP1 UART flag-poll path. Changing the Pi 5 firmware console to skip that poll was necessary but not sufficient: a first-`println!()` return diagnostic without a static-string fast path still failed, which kept `core::fmt::write` in the failure boundary.
- Decision: Make the Pi 5 firmware console use posted-write flushes without polling `UART_FR_TXFF`, and add a static-string fast path in `target::console::_print` using `fmt::Arguments::as_str()` so literal boot messages call `write_str` directly. Until dynamic formatting is validated, the Pi 5 runtime path emits static early-console lines and halts instead of using formatted boot-info output.
- Required validation: Local validation must pass archive review, disassembly inspection of the static fast path and reset probe, formatting, unit tests, QEMU smoke, mdBook, and diff check. Hardware validation requires one active Pi 5 run per boundary under `hardwareTestLock`, corrected TFTP cursor evidence, and snapshot restore.
- Hardware result: The no-TXFF-poll first-`println!()` archive `87d67b83260e4e14251bbaf6a372f012e1476762f335fc3305f110e7237d16bd` with kernel `06d5c2bd3034ede62efdc323de59c8df7d0c2432de0bab46261529b58d2aca37` / 74,344 bytes fetched once at TFTP cursor `686457->687808` but did not reset. After adding the `as_str` fast path, archive `8d2911c92b247475eb7cd204e0ffeb44083723ce3c611be3df277b120b23e9ec` with kernel `a27575ed68edfb36123004a971a35adac67866baf11a9bb001d66caa61b7fddf` / 74,368 bytes produced four boot/fetch sequences at cursor `687808->693212`, proving the first static `println!()` returned. A formatted banner archive `95bb9ed7a95e0f829cf86f52554bafca793404a89220e15820b84878c8fb3df2` with kernel `50851f212746b46ca70313ff4b968e3aebce016bf6787c68c8c037d41ae4fd21` / 74,440 bytes fetched once at cursor `694563->695824` but did not reset, so dynamic formatting remains unaccepted. A static two-line banner archive `d27122b4200aecc580f2c5fd183f7a8db2c7f61d2a4c9418701bb8e45f741007` with kernel `2b240b74f3d62099eb844a2fd48a0ca0ad68813c689db6db0937388d0953d1e2` / 74,400 bytes produced four boot/fetch sequences at cursor `695824->701228`. Each run restored the accepted first-light proof tree hash `da58994fc0492a8cfee9c7b081c49a4cd0a15f7ed97a0508e0035a9671d5e102`.
- Rationale: Static boot output now has hardware-level TFTP side-effect proof through two literal `println!` calls, while dynamic `fmt::write` remains a separate unresolved boundary. Keeping Pi 5 early output static creates durable Talos progress without pretending the full formatter path is ready.
- Risks: The static path is a bring-up limitation, not the final console design. Dynamic formatting still needs a bounded diagnostic, likely inside `core::fmt::write` or a Talos-owned formatting substitute, before restoring formatted Pi 5 boot-info output.

## 2026-05-20 - Use Minimal Formatter For Pi 5 Early Boot Info

- Status: accepted
- Context: Static Pi 5 console output is accepted, but generic dynamic formatting remains unaccepted. The runtime still needs useful boot metadata before the full formatter path is debugged.
- Decision: Add a Talos-owned minimal formatter for decimal and hexadecimal `usize` values and use it only on the Pi 5 early boot path. The formatter writes directly through `fmt::Write::write_str`/characters and avoids `core::fmt::write`, preserving the accepted no-TXFF-poll static console path.
- Required validation: Local validation must pass formatter unit tests, QEMU smoke, normal Pi 5 image build, diagnostic image build, archive review, disassembly inspection for a post-formatter `smc #0` reset probe, mdBook, and diff check. Hardware validation requires one controlled Pi 5 run under `hardwareTestLock`, corrected TFTP cursor evidence of repeated 79,984-byte diagnostic kernel serves before restore, and restore to the accepted first-light proof tree.
- Hardware result: The accepted long-window run `rpi5-minimal-format-reset-long-20260520084921` used archive `4571b47eeb02386dc4c53d9080aba949785f425d649825e77a1bfe8b12be44fc` with kernel `9a0251dbba2f0c2441c321159ca59789f032c32583560e6fef9e01031a12ebf1` / 79,984 bytes. TFTP cursor `709334->716089` captured 65 fresh events, including ten served `da591740/kernel_2712.img` events at 79,984 bytes before restore. Restore returned the lab to accepted first-light proof tree `da58994fc0492a8cfee9c7b081c49a4cd0a15f7ed97a0508e0035a9671d5e102`, with post-restore cursor evidence showing four 272-byte proof-image serves.
- Rationale: This restores useful Pi 5 boot-info output without depending on the still-failing generic formatting path. It keeps the bring-up path simple and evidence-backed while leaving dynamic formatting as a separate future boundary.
- Risks: This is not a replacement for the final formatting stack. The generic `core::fmt::write` path remains unaccepted and should be investigated separately before formatted Pi 5 banners or panic output are treated as hardware-ready.

## 2026-05-20 - Keep Pi 5 Panic And Exception Output Off Dynamic Formatting

- Status: accepted
- Context: Pi 5 generic dynamic formatting is still unaccepted. The previous panic and exception paths used formatted `println!` output, so a later panic or exception on hardware could hang inside the same formatting boundary instead of reporting useful fault state.
- Decision: On Pi 5, keep panic output static and route exception-vector reporting through the minimal early formatter. Exception output writes the vector name and ESR/ELR/FAR as `u64` hex values without calling `core::fmt::write`. QEMU keeps the richer formatted output.
- Required validation: Local validation must pass formatting, unit tests for the added `u64` formatter, QEMU smoke, Pi 5 image build, diagnostic image/archive review, disassembly inspection for deliberate `brk #0` and post-report `smc #0`, mdBook, and diff check. Hardware validation requires one controlled Pi 5 run under `hardwareTestLock`, corrected TFTP cursor evidence of repeated 71,528-byte diagnostic kernel serves before restore, and restore to the accepted first-light proof tree.
- Hardware result: The deliberate exception-report run `rpi5-exception-report-reset-20260520092633` used archive `f41eb916fb13c3cbaeaf40f4bc35f3a461aefb6c1214d26c9aebc8a355b4f36d` with kernel `ef346d16be9833c7bbadf1c2a3f106181d1e2bdf1b758a9598919d1dd2a89af0` / 71,528 bytes. TFTP cursor `716089->720142` captured 39 fresh events and six served `da591740/kernel_2712.img` events at 71,528 bytes before restore, proving the exception report path reached the PSCI reset side effect. Restore returned the lab to accepted first-light proof tree `da58994fc0492a8cfee9c7b081c49a4cd0a15f7ed97a0508e0035a9671d5e102`.
- Rationale: This protects fault reporting from a known unaccepted dependency while preserving QEMU developer ergonomics. It also extends the minimal formatter only as far as the current fault-reporting need requires.
- Risks: The hardware proof is TFTP reset-side-effect evidence, not clean serial capture of the exception text. Panic output still intentionally drops detail on Pi 5 until the formatter path is accepted or a dedicated panic formatter exists.

## 2026-05-20 - Refuse Dynamic Formatting On Pi 5 Early Console

- Status: accepted
- Context: Pi 5 static and minimal formatter output are accepted, but generic `core::fmt::write` remains unaccepted and previously caused early console calls with runtime arguments to hang.
- Decision: On Pi 5 only, make `target::console::_print` refuse non-static `fmt::Arguments` by writing a static `dynamic formatting disabled` diagnostic line and returning. Also make formatted `print!`/`println!` macro calls a compile-time error for Pi 5 builds outside the explicit dynamic-format fallback diagnostic cfg. QEMU keeps the normal `write_fmt` path.
- Required validation: Local validation must pass formatting, unit tests, QEMU smoke, normal Pi 5 image build, diagnostic image/archive review, disassembly inspection showing a formatted call returns to a post-call `smc #0` reset probe, the Pi 5 formatted macro guard regression script, mdBook, and diff check. Hardware validation requires one controlled Pi 5 run with repeated 71,584-byte diagnostic kernel serves before restore.
- Hardware result: The run `rpi5-dynamic-format-fallback-reset-20260520094112` used archive `9a4ae327352050fc8d3e57b6e4155061d040706b51cc50f44d91229d627f0279` with kernel `c5d05c73fb3198f6ff6cfd8725af73555e16f908e830d55cf43d1e296de5ecfc` / 71,584 bytes. TFTP cursor `720142->724195` captured 39 fresh events and six served `da591740/kernel_2712.img` events at 71,584 bytes before restore, proving the dynamic-format fallback returned to the caller and reached the PSCI reset side effect. Restore returned the lab to accepted first-light proof tree `da58994fc0492a8cfee9c7b081c49a4cd0a15f7ed97a0508e0035a9671d5e102`.
- Rationale: Refusing the known-bad path is safer than allowing accidental formatted Pi 5 early-console calls to hang. The macro guard catches ordinary formatted early-output mistakes during local Pi 5 builds, while the runtime fallback remains a defensive backstop for direct `_print(format_args!(...))` calls and for the accepted diagnostic. This preserves bring-up momentum while keeping full formatting as a separate future investigation.
- Risks: Runtime arguments are intentionally unavailable on Pi 5 early output until generic formatting is accepted or more Talos-owned minimal formatting helpers are added. Any new Pi 5 early output that needs values must use the minimal formatter helpers deliberately.

## 2026-05-20 - Isolate Pi 5 Generic Formatting From UART

- Status: not accepted for both dynamic and static no-MMIO sink diagnostics
- Context: The first formatted Pi 5 console call reached `Pl011::write_fmt` but did not reach `Pl011::write_str`. Static/minimal formatter output and the dynamic-format refusal path are accepted, but the exact `core::fmt::write` failure boundary is still unclear.
- Decision: Add `TALOS_RPI5_FMT_SINK_DIAGNOSTIC=1`, which calls `core::fmt::write` with a local sink writer whose `write_str` has no UART/MMIO side effects, then invokes PSCI `SYSTEM_RESET`. Also add `TALOS_RPI5_FMT_STATIC_SINK_DIAGNOSTIC=1`, which uses the same sink but static-only `format_args!`. These test formatter machinery separately from PL011 writes.
- Required validation: Local validation must pass formatting, unit tests, QEMU smoke, normal Pi 5 image build, fmt-sink diagnostic image build, static fmt-sink diagnostic image build, format-guard regression, diagnostic archive review, disassembly inspection showing `core::fmt::write` before the post-diagnostic `smc #0`, mdBook, and diff check. Hardware validation requires one controlled Pi 5 run with repeated diagnostic kernel serves before restore.
- Hardware result: Dynamic-argument sink run `rpi5-fmt-sink-reset-retry-20260520101800` served the 75,808-byte diagnostic kernel twice at TFTP cursor `728248` but did not show a reset-induced repeat before restore, so it is not accepted. The first static-only sink run `rpi5-fmt-static-sink-reset-20260520102130` published the 75,808-byte diagnostic tree, but the captured cursor only showed first-light 272-byte serves before restore, so that run was inconclusive. The long static-only retry `rpi5-fmt-static-sink-reset-long-retry-20260520104630` used archive `e89d67e6e7cff9c96235e07876c7c2e277e7c672ba561ac8f4d6a2cd12150ec2` with kernel `ccbf324f04dc1cec918417989a143f8b31f9ce56866a624976d8a793cfdcde62` / 75,872 bytes. Status verified both root and serial-prefixed kernels were 75,872 bytes before the power cycle. TFTP cursor `732301->733652` captured only the first boot's two 75,872-byte diagnostic kernel serves and no reset-induced repeat over the long window. The lab was restored to accepted first-light proof tree `da58994fc0492a8cfee9c7b081c49a4cd0a15f7ed97a0508e0035a9671d5e102`.
- Follow-up result: Added `TALOS_RPI5_FMT_SINK_DIRECT_DIAGNOSTIC=1` to bypass `core::fmt::write` and directly call the same no-MMIO sink's `write_str`, branch on `Result`, and then call the PSCI reset probe. Local disassembly showed the direct diagnostic path calls `Rpi5FmtSink::write_str`, branches with `cbz`, then calls `rpi5_fmt_sink_reset_probe`; `core::fmt::write` remains in the image for other paths but is not on this diagnostic path. Hardware run `rpi5-fmt-sink-direct-reset-20260520110400` used archive `8613c235e0eed4fd916e968a27ac56052930f5f27d134ab6106921e27d6a376c` with kernel `b368d6583ad14fd46f798a6c03b5154de95122d1d35c71ec9b023edc85db9aed` / 71,616 bytes. TFTP cursor `733652->737705` captured six 71,616-byte diagnostic kernel serves at 11:04:22, 11:04:39/40, and 11:04:57/58 before restore, proving the direct sink path reached the reset side effect. The lab was restored to accepted first-light proof tree `da58994fc0492a8cfee9c7b081c49a4cd0a15f7ed97a0508e0035a9671d5e102`.
- Follow-up result: Added `TALOS_RPI5_FMT_SINK_DYN_DIRECT_DIAGNOSTIC=1` to test dynamic trait-object dispatch to the same no-MMIO sink without entering `core::fmt::write`. Local disassembly showed the diagnostic builds a `dyn core::fmt::Write` trait object, loads the vtable `write_str` slot, calls it with `blr`, branches on the returned `Result`, and then calls the PSCI reset probe. Local validation passed: `cargo fmt --check`, `cargo -Zjson-target-spec test`, QEMU smoke, normal Pi 5 image build, dynamic-direct diagnostic image build, archive review, disassembly inspection, and `git diff --check`. Two earlier evidence attempts queried TFTP after restore, which made the endpoint report restored 272-byte file sizes for prior diagnostic serves. The corrected pre-restore run `rpi5-fmt-sink-dyn-direct-limitfix-20260520115810` used archive `19e7265b5fc339eef864c9195d0f559aaa96cae66469e992ddfc7f063ef0130d` with kernel `d3a59ca7b52eb760ddcdde930063636b56aff2fc7d3b5de5dead9695cbe348fe` / 75,824 bytes. TFTP cursor `741758->743109` captured only the first boot's two 75,824-byte diagnostic kernel serves and no reset-induced repeat over the 120-second window before restore, so the dynamic trait-object direct sink path is not accepted.
- Follow-up result: Added `TALOS_RPI5_FMT_SINK_FNPTR_DIRECT_DIAGNOSTIC=1` to test an indirect function-pointer call into a helper that immediately performs the same no-MMIO sink `write_str`. Local validation passed: `cargo fmt --check`, `cargo -Zjson-target-spec test`, QEMU smoke, normal Pi 5 image build, function-pointer diagnostic image build, archive review, disassembly showing `blr x8` -> `cbz` -> reset probe without `core::fmt::write` on that path, and `git diff --check`. Hardware run `rpi5-fmt-sink-fnptr-direct-20260520121210` used archive `5c42c7c84f062f0f593bc39c6386dc38aad351fd6354a3c37e3ae1fa56bc3301` with kernel `3385a527f10d0afb0f93169966d3c7a25ae8368485dac7272e2b470be23d6bc0` / 71,624 bytes. TFTP cursor `743109->744460` captured only the first boot's two 71,624-byte diagnostic kernel serves and no reset-induced repeat over 120 seconds before restore, so the function-pointer indirect-call path is not accepted.
- Follow-up result: Added `TALOS_RPI5_FNPTR_RESET_DIAGNOSTIC=1` to test only a black-boxed Rust function pointer to the PSCI reset probe, with no sink writer or formatter call. Local validation passed: `cargo fmt --check`, `cargo -Zjson-target-spec test`, QEMU smoke, normal Pi 5 image build, function-pointer reset diagnostic image build, archive review, disassembly showing `blr x0` directly to the reset probe, and `git diff --check`. Hardware run `rpi5-fnptr-reset-20260520122640` used archive `4d4a7cd5ad8148416c02bd1512f591c9491301e95cf0c2cf80104bbbe6fa7fdd` with kernel `164593e42ba8d479e4beb749c737411e408dddff4aeabff21a3e9f8e5813e24c` / 71,608 bytes. TFTP cursor `744460->745811` captured only the first boot's two 71,608-byte diagnostic kernel serves and no reset-induced repeat over 120 seconds before restore, so even the minimal black-boxed function-pointer reset path is not accepted.
- Follow-up result: Added `TALOS_RPI5_ASM_INDIRECT_RESET_DIAGNOSTIC=1` to test an assembly-owned indirect branch to a same-section reset probe before entering Rust. Local validation passed: `cargo fmt --check`, `cargo -Zjson-target-spec test`, QEMU smoke, normal Pi 5 image build, assembly-indirect diagnostic image build, archive review, disassembly showing `blr x20` to `rpi5_asm_indirect_reset_probe` before `rust_entry`, and `git diff --check`. Hardware run `rpi5-asm-indirect-reset-20260520124205` used archive `7d862def5ef888a5b515785528c89de32afc6398caa38e263147ea66e2592212` with kernel `c252b9d178566ed96b866985774628b0cebfbf8cd7a737c6d689c1eae13962d2` / 71,608 bytes. TFTP cursor `745811->747162` captured only the first boot's two 71,608-byte diagnostic kernel serves and no reset-induced repeat over 120 seconds before restore, so an assembly-owned indirect reset branch is also not accepted.
- Follow-up result: Added `TALOS_RPI5_ASM_DIRECT_RESET_DIAGNOSTIC=1` to test a direct assembly `bl` to the same-style PSCI reset probe before entering Rust. Local validation passed: `cargo fmt --check`, `cargo -Zjson-target-spec test`, QEMU smoke, normal Pi 5 image build, assembly-direct diagnostic image build, archive review, disassembly showing `bl rpi5_asm_direct_reset_probe` before `rust_entry` and `smc #0` in the probe, pre-hardware review, and `git diff --check`. Hardware run `rpi5-asm-direct-reset-20260520130030` used archive `0333cb8269a83df5c44d84b25591b18c49cb6cca5b8d86ac1f4aa82c5b7ed5d1` with kernel `1d946afbc8179d439792a4c9dced414963c132eefa9b3b63dac1094e96a509b9` / 71,608 bytes. TFTP cursor `747162->748513` captured only the first boot's two 71,608-byte diagnostic kernel serves and no reset-induced repeat over 120 seconds before restore, so this direct assembly branch to the new same-style reset probe is also not accepted.
- Follow-up result: Added `TALOS_RPI5_ASM_TO_RUST_RESET_DIAGNOSTIC=1` to branch directly from the assembly post-stack point into an exported Rust wrapper around the same `rpi5_fmt_sink_reset_probe` shape that the accepted direct sink diagnostic used. Local validation passed: `cargo fmt --check`, `cargo -Zjson-target-spec test`, QEMU smoke, normal Pi 5 image build, asm-to-Rust diagnostic image build, archive review, disassembly showing `bl rpi5_asm_to_rust_reset_probe` before `rust_entry`, wrapper `bl rpi5_fmt_sink_reset_probe`, `smc #0` in the probe, pre-hardware review, and `git diff --check`. Hardware run `rpi5-asm-to-rust-reset-20260520131400` used archive `232ad31ff67c72018b6fcff5bc605ff24840a3ffaa0808756d4ad9a9fe40a4f2` with kernel `d2114d80941a3d2078de0be85b8f7913352ce49850d2afbb4d9c6457b2570415` / 71,608 bytes. The primary pre-restore hardware log captured repeated 71,608-byte `da591740/kernel_2712.img` serves from cursor `749864->759321`, including reset-induced sequences at 13:15:20, 13:15:37/38, 13:15:55, 13:16:12/13, and 13:16:30 before restore. This accepts branching from assembly into the Rust reset-probe shape.
- Follow-up result: Added `TALOS_RPI5_ASM_INDIRECT_TO_RUST_RESET_DIAGNOSTIC=1` to use an assembly-owned indirect `blr x20` into the same exported Rust reset wrapper that the accepted direct asm-to-Rust diagnostic used. Local validation passed: `cargo fmt --check`, `cargo -Zjson-target-spec test`, QEMU smoke, normal Pi 5 image build, asm-indirect-to-Rust diagnostic image build, archive review, disassembly showing `ldr x20, =rpi5_asm_to_rust_reset_probe` and `blr x20` before `rust_entry`, pre-hardware review, and `git diff --check`. Hardware run `rpi5-asm-indirect-to-rust-reset-20260520132900` used archive `2791708787ef0973e1903d34ace2cbe53bc9064171da94360cd5c9afa1d6a90a` with kernel `764854c7f1f877aa61e4269b204537c74252d9829dce78fd5212f1e35fb43ff2` / 71,608 bytes. The lab published tree `543bc426012f20d6617621cd53278263599aea4b7289fadce2ffe82d772fef99` with 71,608-byte root and serial-prefixed kernels, then power-cycled once. TFTP cursor `759321->760672` captured only the first boot's two diagnostic `da591740/kernel_2712.img` serves at 13:30:03/04 and no reset-induced repeat over the 120-second window before restore, so assembly-owned indirect branching into the otherwise accepted Rust reset wrapper is not accepted. The lab was restored to accepted first-light tree `da58994fc0492a8cfee9c7b081c49a4cd0a15f7ed97a0508e0035a9671d5e102`.
- Follow-up result: Added `TALOS_RPI5_ASM_BTI_INDIRECT_TO_RUST_RESET_DIAGNOSTIC=1` to make the indirect target a tiny assembly veneer whose first instruction is `hint #34` / `bti c`, followed by a direct branch to the same exported Rust reset wrapper. Local validation passed: `cargo fmt --check`, `cargo -Zjson-target-spec test`, QEMU smoke, normal Pi 5 image build, BTI indirect diagnostic image build, archive review, disassembly showing `blr x20` to the veneer, `bti c`, then `b rpi5_asm_to_rust_reset_probe`, pre-hardware review, and `git diff --check`. Hardware run `rpi5-asm-bti-indirect-to-rust-reset-20260520134500` published archive `bbc524b449ff8d6c8dfaeadd9325f2e792f6dd51d6c4dbf9ee959fb286e977ba` with kernel `3f22bad0c91edbe4130d7c3c27b5b5ec11cfa5e99e3a99438bcb22fa6d703419` / 71,608 bytes. The lab published tree `42195d59acfeaf1c007e04b3a02da53ba27ec19818a24265a6dbdd6e200a061e` with 71,608-byte root and serial-prefixed kernels, then power-cycled once. Corrected TFTP evidence from cursor `760672->762023` showed only the first boot's two `da591740/kernel_2712.img` serves at 13:46:57 and no reset-induced repeat; because the query was finalized after restore, the endpoint relabeled those historical lines with restored 272-byte file sizes, so the pre-restore status and archive review are the authoritative diagnostic size evidence. The lab was restored to accepted first-light tree `da58994fc0492a8cfee9c7b081c49a4cd0a15f7ed97a0508e0035a9671d5e102`. This does not accept the BTI landing-pad veneer as sufficient.
- Follow-up result: Added `TALOS_RPI5_ASM_TEXT_INDIRECT_RESET_DIAGNOSTIC=1` to test a pure assembly indirect branch to a reset probe in executable `.text`, with a 16-byte-aligned `bti c` landing instruction and inline PSCI `SYSTEM_RESET` SMC. Local validation passed: `cargo fmt --check`, `cargo -Zjson-target-spec test`, QEMU smoke, normal Pi 5 image build, text-indirect diagnostic image build, archive review, disassembly showing `blr x20` to `rpi5_asm_text_indirect_reset_probe`, pre-hardware review, and `git diff --check`. Pre-hardware review caught that the first archive attempt used stale normal-image kernel `3f22bad0c91edbe4130d7c3c27b5b5ec11cfa5e99e3a99438bcb22fa6d703419`; the archive was regenerated with the diagnostic env passed through the prefixed boot-tree build. Hardware run `rpi5-asm-text-indirect-reset-20260520140200` used archive `f34316a2e8b0af7c9062102694ac44d6a488bc2c22c7fa0efdebcef0aaf11c9e` with kernel `99fdbb256a0149d28a01b09a7771e5bb7c3b911928bd8cfa055dd466d7977e98` / 71,608 bytes. The lab published tree `00828b6e7632d924527fe912cf6b5809440197bba8f7e6f222fcc0931d5e7873` with 71,608-byte root and serial-prefixed kernels, then power-cycled once. TFTP cursor `762023->763374` captured only the first boot's two diagnostic `da591740/kernel_2712.img` serves at 14:01:19/20 and no reset-induced repeat before restore. The lab was restored to accepted first-light tree `da58994fc0492a8cfee9c7b081c49a4cd0a15f7ed97a0508e0035a9671d5e102`. This does not accept a pure assembly `.text` indirect reset target with a BTI landing pad and inline SMC.
- Follow-up result: Added `TALOS_RPI5_ASM_TEXT_DIRECT_RESET_DIAGNOSTIC=1` to direct-branch to the same 16-byte-aligned executable `.text` reset probe used by the rejected text-indirect diagnostic. Local validation passed: `cargo fmt --check`, `cargo -Zjson-target-spec test`, QEMU smoke, normal Pi 5 image build, text-direct diagnostic image build, text-indirect diagnostic image build regression, archive review, disassembly showing `bl rpi5_asm_text_reset_probe` before `rust_entry`, pre-hardware review, and `git diff --check`. Hardware run `rpi5-asm-text-direct-reset-20260520141800` used archive `c5bde6ed156b6dd902c01842167fb1f5655433366f698d775a6b3c7bc59183eb` with kernel `541f349dc7f2584226156a577491ae7a578949119b94fadbed6efb659897cbac` / 71,608 bytes. The lab published tree `ff729f9a317d8e958ed8173cd4d39605b0479e9c778349343dad7e33cfc7c70a`, then power-cycled once. TFTP cursor `763374->771480` captured 78 fresh events and twelve 71,608-byte `da591740/kernel_2712.img` serves at 14:16:20/21, 14:16:37/38, 14:16:55/56, 14:17:12/13, 14:17:30/31, and 14:17:47/48 before restore, proving the direct `.text` reset target reached the PSCI reset side effect. The lab was restored to accepted first-light tree `da58994fc0492a8cfee9c7b081c49a4cd0a15f7ed97a0508e0035a9671d5e102`.
- Follow-up result: Added `TALOS_RPI5_ASM_TEXT_JC_INDIRECT_RESET_DIAGNOSTIC=1` to retry the pure assembly `.text` indirect reset target with `hint #38` / `bti jc`, rather than `bti c`, as the landing instruction. Local validation passed: `cargo fmt --check`, `cargo -Zjson-target-spec test`, QEMU smoke, normal Pi 5 image build, JC indirect diagnostic image build, archive review, disassembly showing `blr x20` to `rpi5_asm_text_jc_reset_probe`, `bti jc`, `ldr x0`, and `smc #0`, pre-hardware review, and `git diff --check`. Hardware run `rpi5-asm-text-jc-indirect-reset-20260520143100` used archive `dbee47c60ec4f939589b246cc3a55a6a9fbc6f6e5e6286624d3ab08a8832eaef` with kernel `1e94a85765c67da3196dc40658ca14e401ebd1eef11e2bd6735ecd1fcc7720c4` / 71,608 bytes. The lab published tree `d00349f1c67944d1098e1afe48dee7b5ba17745f59000ef13004ff9c0e253564`, then power-cycled once. TFTP cursor `772831->774182` captured only the first boot's two 71,608-byte `da591740/kernel_2712.img` serves at 14:30:38 and no reset-induced repeat before restore. The lab was restored to accepted first-light tree `da58994fc0492a8cfee9c7b081c49a4cd0a15f7ed97a0508e0035a9671d5e102`. This rejects the `bti jc` landing-pad compatibility hypothesis by itself.
- Follow-up result: Added `TALOS_RPI5_BTI_EXCEPTION_CLASSIFIER_DIAGNOSTIC=1` to install a local vector table for the current exception level, execute `blr x20` to a `bti c` target that resets immediately on success, and reset only after a long delay if the synchronous exception handler sees `ESR_ELx.EC == 0x0d` Branch Target Exception. Local validation passed: `cargo fmt --check`, `cargo -Zjson-target-spec test`, QEMU smoke, normal Pi 5 image build, classifier diagnostic image build, archive review, disassembly showing `msr VBAR_EL1/2/3`, `blr x20`, target `bti c` + `smc #0`, handler `mrs ESR_EL1/2/3`, EC compare against `0x0d`, delay loop, pre-hardware review, and `git diff --check`. Hardware run `rpi5-bti-exception-classifier-20260520144500` used archive `94d2ffd2275d006855d1ccbd2b5d6838cbaf32bed9df7a9f3fc09bfce2b75c74` with kernel `fa2432f42eb21873feb94864e5616f30d7461e935db37151e336e651a7672f5f` / 75,704 bytes. The lab published tree `1d72883531b420ca6d6cabe08274abd8a35cfaa451f8aa6f12eef9e5686d2337`, then power-cycled once. TFTP cursor `774182->775533` captured only the first boot's two 75,704-byte `da591740/kernel_2712.img` serves at 14:46:24 and no immediate or delayed reset-induced repeat before restore. The lab was restored to accepted first-light tree `da58994fc0492a8cfee9c7b081c49a4cd0a15f7ed97a0508e0035a9671d5e102`. This does not classify the failure as a caught Branch Target Exception through the installed vectors.
- Follow-up result: Added `TALOS_RPI5_DIRECT_EXCEPTION_CONTROL_DIAGNOSTIC=1` to validate the vector/exception-control side of the classifier without any indirect branch. The diagnostic installs the same CurrentEL-selected VBAR shape, executes `brk #0`, routes all vector slots to a handler, reads `ESR_EL1/2/3`, compares EC against `0x3c` AArch64 BRK, resets after a distinct delay on the expected exception, and parks otherwise. Local validation passed: `cargo fmt --check`, `cargo -Zjson-target-spec test`, QEMU smoke, normal Pi 5 image build, direct exception-control diagnostic image build, disassembly showing `msr VBAR_EL1/2/3`, `brk #0`, vector branches, `mrs ESR_EL1/2/3`, EC compare against `0x3c`, delay loop, `smc #0`, pre-hardware review, mdBook, and `git diff --check`. Hardware run `rpi5-direct-exception-control-20260520154000` used archive `269131009874a9f035be3c77269faf5bdbaba72045e4795c2f16b8532bd6ce99` with kernel `2c3039dab5bbc0c62997c749fd47f72dbbf02757db09437169ff77e558b90b87` / 75,704 bytes. The lab published tree `fc0e404723fba926165932d1648798668f91fab8d7c2aec60402c0a18a2e216c`, then power-cycled once. TFTP cursor `775533->776884` captured only the first boot's two 75,704-byte `da591740/kernel_2712.img` serves at 15:42:17/18 and no delayed reset-induced repeat before restore. The lab was restored to accepted first-light tree `da58994fc0492a8cfee9c7b081c49a4cd0a15f7ed97a0508e0035a9671d5e102`. Post-hardware review classified this first run as not accepted and likely inconclusive because the original delay loop was too long for the observed dwell, so the delay was shortened from 96 outer loops to 4. The short-delay artifact used archive `0b3db220ae2730393093d377dda547343d98b44d711f2e3911a910da13cf1cb6`, kernel `8955c65e47c6fc564052923188bb8f1025e500953497734e81304b186eb56c79` / 75,704 bytes, and published tree `cafe86bf08ef9ed3763b30bd0682b1adf0efe596161c236d350488accd1230a5`. The broad short-run cursor `776884->779586` showed four 75,704-byte serves, but post-hardware review rejected it as contaminated because those serves spanned two separate lab power cycles. The isolated clean rerun cursor `778235->779586` showed only the initial boot pair at 15:48:12/13 and no reset-induced repeat before restore; the post-restore TFTP JSON relabeled historical sizes as 272 bytes, but the publish/archive records prove the diagnostic tree was 75,704 bytes at power-cycle time. The short-delay rerun is valid negative evidence for this acceptance criterion, not an accepted exception-handler proof.
- Follow-up result: Added `TALOS_RPI5_DIRECT_EXCEPTION_IMMEDIATE_RESET_DIAGNOSTIC=1` to remove delay-window ambiguity from the direct BRK/vector control. The diagnostic installs the CurrentEL-selected VBAR table, executes `brk #0`, routes all vector slots to one handler, reads `ESR_EL1/2/3`, compares EC against `0x3c`, and calls PSCI `SYSTEM_RESET` immediately on success; unexpected EL/EC parks. Local validation passed: `cargo fmt --check`, `cargo -Zjson-target-spec test`, QEMU smoke, normal Pi 5 image build, immediate-reset diagnostic image build, pre-hardware review, and `git diff --check`. Early hardware attempts were rejected because restore timing contaminated the evidence windows. The accepted clean delayed-restore run `rpi5-direct-exception-immediate-reset-clean-20260520163500` used archive `16121cc00678687f703930e568ff01cd82ba9572b034f34f5b9fe15cbfe1cc39` with kernel `9c1e8a642d990044353a02425106de43408e4653b95276c612ccc3efb90e5b65` / 75,704 bytes and published tree `f452e17db0b5fd34451cc0b3ce3fd40776f91fa88d8f93e3284c51602d1112b1`. TFTP cursor `786341->790394` captured 39 events, including six 75,704-byte `da591740/kernel_2712.img` serves at 16:28:08/09, 16:28:26/26, and 16:28:43/44 before restore while `status-before-restore` still showed 75,704-byte root and serial-prefixed kernels. Restore returned the lab to accepted first-light tree `da58994fc0492a8cfee9c7b081c49a4cd0a15f7ed97a0508e0035a9671d5e102`. This accepts the simple direct BRK exception path and proves the CurrentEL-selected VBAR/ESR handler/reset side effect can work in the Pi 5 boot context.
- Follow-up result: Added `TALOS_RPI5_BTI_CLASSIFIER_WITH_BRK_PRECHECK_DIAGNOSTIC=1` to combine the accepted direct BRK/vector control with the indirect BTI classifier in one boot attempt. The diagnostic installs the CurrentEL-selected VBAR table, executes `brk #0`, handles EC `0x3c` by rewriting `ELR_EL1/2/3` to a continuation label and returning with `eret`, then executes `blr x20` to a 16-byte-aligned `bti c` target that resets immediately on success. The same handler resets after a distinct delay if it sees EC `0x0d` Branch Target Exception and parks otherwise. Local validation passed: `cargo fmt --check`, `cargo -Zjson-target-spec test`, QEMU smoke, normal Pi 5 image build, BRK-precheck classifier image build, archive review, disassembly showing `msr VBAR_EL1/2/3`, `brk #0`, `ELR_EL1/2/3` rewrite + `eret`, `blr x20`, `bti c`, EC compares for `0x3c` and `0x0d`, pre-hardware review, and `git diff --check`. The first hardware run `rpi5-bti-brk-precheck-classifier-20260520165000` used archive `9511a58fd363dee57b3d38ad1ae9535bf4d8974aea687a35a48340c71ec1b076` with kernel `8304afbfc4a67224b451c1186f65a757044e8d28f5b66f60e0551bbd9edd2c9e` / 79,800 bytes, but post-hardware review rejected it because the diagnostic-phase TFTP capture files were empty. The clean rerun `rpi5-bti-brk-precheck-classifier-rerun-20260520165636` used the same archive and kernel, published tree `71bee9f27d53050ed6f307c6723ae7af91a0b769ca19277c1a5e86d2266931bb`, and captured authoritative pre-restore TFTP cursor `793096->794447` with only the first boot's two 79,800-byte `da591740/kernel_2712.img` serves at 16:56:59/17:00 and no immediate or delayed reset-induced repeat. A duplicate post-restore capture overwrote JSON byte labels with restored 272-byte sizes, so `pre-restore-capture-note.txt` preserves the authoritative pre-restore size evidence. The lab was restored to accepted first-light tree `da58994fc0492a8cfee9c7b081c49a4cd0a15f7ed97a0508e0035a9671d5e102`. This does not prove the indirect branch reached the `bti c` target and does not prove a caught Branch Target Exception reset path after a successful BRK precheck.
- Follow-up result: Added `TALOS_RPI5_BRK_ERET_RESUME_RESET_DIAGNOSTIC=1` to remove the indirect branch from the BRK-precheck continuation. The diagnostic installs the CurrentEL-selected VBAR table, executes `brk #0`, requires EC `0x3c`, writes the matching `ELR_EL1/2/3` to a continuation label, returns with `eret`, and has the continuation immediately call PSCI `SYSTEM_RESET`; unexpected EL/EC and return after SMC park. Local validation passed: `cargo fmt --check`, `cargo -Zjson-target-spec test`, QEMU smoke, normal Pi 5 image build, BRK/ERET resume diagnostic image build, pre-hardware review, disassembly review, and `git diff --check`. Hardware run `rpi5-brk-eret-resume-reset-20260520171909` used archive `ba07c83b5bd431cc6c632a054281ae5e0b748a2474b0e2649d69f156f706f701` with kernel `0238a85bdef75f9806a1b1319c214bc074f9a309ff8e55d7a64337adc46b19cd` / 75,704 bytes and published tree `be5e71cebce719f9ef2e06faf1ef3ce044771c5b9dbcee69163aeb8c4d7e1856`. TFTP cursor `794447->795798` captured only the first boot's two 75,704-byte `da591740/kernel_2712.img` serves at 17:20:12 and no reset-induced repeat before restore. The lab was restored to accepted first-light tree `da58994fc0492a8cfee9c7b081c49a4cd0a15f7ed97a0508e0035a9671d5e102`. This does not prove the BRK handler rewrote `ELR` and resumed via `eret` to the reset continuation.
- Follow-up result: Added `TALOS_RPI5_BRK_ELR_WRITE_RESET_DIAGNOSTIC=1` to test whether writing `ELR_ELx` in the BRK handler is itself safe before testing `eret`. The diagnostic installs the CurrentEL-selected VBAR table, executes `brk #0`, requires EC `0x3c`, writes the matching `ELR_EL1/2/3` to a continuation label, then branches directly to handler-local PSCI `SYSTEM_RESET` without `eret` or any indirect branch; the continuation parks if reached unexpectedly. Local validation passed: `cargo fmt --check`, `cargo -Zjson-target-spec test`, QEMU smoke, BRK/ELR-write diagnostic image build, pre-hardware review, disassembly review, and `git diff --check`. Hardware run `rpi5-brk-elr-write-reset-20260520175453` used archive `a136218253c793d6097bb497f8149d4d0235c2293b5720e1c6c9382aa3dcd840` with kernel `6e188ee584f49ccc6ed0140e0c6da3b4b7d63320084290ffea5604b9b17f528a` / 75,704 bytes and published tree with 75,704-byte root and serial-prefixed kernels. TFTP cursor `795798->803904` captured 78 fresh events, including twelve 75,704-byte `da591740/kernel_2712.img` serves at 17:55:50/51, 17:56:08, 17:56:25/26, 17:56:43, 17:57:00/01, and 17:57:18 before restore. Post-hardware review accepted this as indirect reset-loop proof that the BRK handler wrote `ELR_ELx` and reached the handler-local PSCI reset path. The lab was restored to accepted first-light tree `da58994fc0492a8cfee9c7b081c49a4cd0a15f7ed97a0508e0035a9671d5e102`.
- Follow-up result: Added `TALOS_RPI5_BRK_SPSR_ERET_RESET_DIAGNOSTIC=1` to test whether normalizing `SPSR_ELx` before `eret` is enough to resume from the accepted BRK handler path. The diagnostic installs the CurrentEL-selected VBAR table, executes `brk #0`, requires EC `0x3c`, writes `SPSR_EL1/2/3` to DAIF-masked same-EL h-mode values `0x3c5`, `0x3c9`, and `0x3cd`, writes `ELR_EL1/2/3` to a PSCI reset continuation, then returns with `eret`. Local validation passed: `cargo fmt --check`, `cargo -Zjson-target-spec test`, QEMU smoke, normal Pi 5 image build, SPSR/ERET diagnostic image build, pre-hardware review, disassembly review, and `git diff --check`. The first hardware attempt was invalid because the boot-tree staging regenerated a normal 71,608-byte kernel after the diagnostic build; the lab was restored and the corrected archive was rebuilt by replacing the staged root and serial-prefixed kernels with the 75,704-byte diagnostic image. Corrected hardware run `rpi5-brk-spsr-eret-reset-corrected-20260520T182716` used archive `4893820050c99139aac81f51f31e380436b10c411636f10d4a94c8c372cb0b7c` with kernel `c1381bf09d231262801b6e1ba36f6dabaa53c9db314b16e91b8ee4ee901fcfba` / 75,704 bytes. Published and pre-restore status showed 75,704-byte root and serial-prefixed kernels, but TFTP cursor `806606->807957` captured only the initial two 75,704-byte `da591740/kernel_2712.img` serves at 18:28:10/11 and no reset-induced repeat over the capture window. Post-hardware review rejected this as proof of SPSR-normalized ERET resume. The lab was restored to the accepted 272-byte first-light tree.
- Follow-up result: Added `TALOS_RPI5_BRK_ERET_UART_MARKER_DIAGNOSTIC=1` to distinguish failed BRK/ERET resume from a resumed continuation whose PSCI reset side effect might fail. The diagnostic keeps the same CurrentEL-selected VBAR, BRK EC `0x3c` check, same-EL DAIF-masked `SPSR_ELx` values, `ELR_ELx` rewrite, and `eret`, but the continuation first emits bounded `EM\r\n` marker bursts to both existing Pi 5 UART MMIO paths before calling PSCI `SYSTEM_RESET`. Local validation passed: `cargo fmt --check`, `cargo -Zjson-target-spec test`, QEMU smoke, normal Pi 5 image build, marker diagnostic image build, fallthrough-Rust diagnostic regression build, pre-hardware review, disassembly review, and `git diff --check`. The first hardware attempt was invalid because the serial observe returned immediately on firmware text before any TFTP activity, so the lab was restored and the run was repeated with a fixed 120-second dwell before restore. Corrected hardware run `rpi5-brk-eret-uart-marker-long-20260520T194442` used archive `8c9e1d6a194e2c9df5360ceb12d2f16e5881cb304eba27a4c94359b713ef351a` with kernel `e4712d4eb2d8b6567a2598c8b5946cc736fa292c12ee24c74c03b0474b52531a` / 75,704 bytes. Published and pre-restore status showed 75,704-byte root and serial-prefixed kernels, but TFTP captured only the initial two 75,704-byte `da591740/kernel_2712.img` serves at 19:45:40/41, and serial after the 120-second dwell contained only NUL/LF with no `BI` or `EM` marker. The lab was restored to the accepted 272-byte first-light tree. This rejects the marker continuation as hardware proof of successful `eret` resume.
- Follow-up result: Added `TALOS_RPI5_BRK_SPSR_HANDLER_RESET_DIAGNOSTIC=1` to isolate whether writing `SPSR_ELx` in the accepted BRK handler is safe when no `eret` is attempted. The diagnostic installs the same CurrentEL-selected VBAR, executes `brk #0`, requires EC `0x3c`, writes `SPSR_EL1/2/3` to `0x3c5`, `0x3c9`, and `0x3cd`, then branches directly to handler-local PSCI `SYSTEM_RESET` without `eret` or any indirect branch. Local validation passed: `cargo fmt --check`, `cargo -Zjson-target-spec test`, QEMU smoke, normal Pi 5 image build, SPSR handler-reset diagnostic image build, pre-hardware review, disassembly review, and `git diff --check`. Hardware run `rpi5-brk-spsr-handler-reset-20260520T202446` used archive `ba4d7bf2caba44ad1cdc40d5c1fa5ef8be186e4b02024d444aa1084ef62e8683` with kernel `6861a96ecc42d7f051da5deb84ca4e0edf22751ab39caa0738c504b450470b08` / 75,704 bytes. Published and pre-restore status showed 75,704-byte root and serial-prefixed kernels. TFTP captured twelve 75,704-byte `da591740/kernel_2712.img` serves at 20:25:44/45, 20:26:01/02, 20:26:19/20, 20:26:36/37, 20:26:54/54, and 20:27:11/12 before restore. Serial after the 120-second dwell contained only NUL/LF, but serial was not the acceptance signal. The lab was restored to the accepted 272-byte first-light tree. This accepts that writing same-EL DAIF-masked `SPSR_ELx` in the BRK handler is safe when control stays handler-local and resets without `eret`.
- Follow-up result: Added `TALOS_RPI5_READABLE_BOOT_LOG_DIAGNOSTIC=1` as a Pi 5-only formatter-free readable boot-log diagnostic, then revised it after post-hardware review showed no readable `TALOS` serial lines and no single-shot `F0`/`B0`/`R0` primitive. The active diagnostic now emits only the repeated pre-boundary marker `TALOS-FW\r\n` on the firmware-preserved RP1 UART0 mapping at `0x1c00030000`, immediately after the Image header branch and `x0` preservation and before RP1 GPIO preparation, BSS/stack setup, VBAR setup, BRK, Rust, formatting, polling, or readback. It emits the line 64 times with a bounded `0x8000`-iteration delay between lines, then intentionally parks in `wfe` instead of continuing into the later readable BRK/reset path; this parks the later `EP`/`CL`/`BH`/`RR` BRK path for this proof so the hardware question is only whether Talos-origin bytes can be observed on the firmware-visible serial path. Marker legend: repeated `TALOS-FW` means the earliest firmware-preserved RP1 write-only path reached; no `TALOS-FW` means the serial proof is still blocked before BRK/readability/runtime dependencies and later BRK markers must not be interpreted. The selected path follows the post-hardware recommendation to prefer the firmware-preserved RP1 mapping, stays write-only until the marker is complete, and avoids heap, Rust console, `eret`, `ret`, indirect branches, and PSCI reset side effects. Local validation passed: `cargo fmt --check`, `cargo -Zjson-target-spec test`, QEMU smoke, normal Pi 5 image build, readable boot-log diagnostic image build, BRK/SPSR handler-reset diagnostic image build, SPSR/ERET diagnostic image build, disassembly/string review, and `git diff --check`. No hardware run was performed for this local staging change; future hardware should run one bounded Pi 5 serial capture and look only for repeated `TALOS-FW` before changing BRK control flow again.
- Rationale: Dynamic and static `core::fmt::write` calls both fail to reach the post-format reset probe even with a no-MMIO sink writer, dynamic trait-object dispatch to the same direct `write_str` fails, a simpler function-pointer indirect call to the same sink helper fails, a function-pointer call directly to the reset probe fails, and assembly-owned indirect and direct branches to tiny reset probes also fail. A direct assembly branch into the exported Rust reset wrapper resets repeatedly, the direct monomorphic no-MMIO `write_str` + `Result` branch path resets repeatedly, a direct branch to the pure assembly `.text` reset target resets repeatedly, a direct `brk #0` through the installed CurrentEL vector table reaches a handler-local PSCI reset path, a BRK handler that writes `ELR_ELx` before handler-local reset also reaches PSCI reset, and a BRK handler that writes same-EL `SPSR_ELx` before handler-local reset reaches PSCI reset. Adding compatible BTI landing pads, including pure assembly `.text` targets with inline SMC and either `bti c` or `bti jc`, has not made the indirect reset shape reach the side effect. The first vector-based classifier did not catch a Branch Target Exception at the installed current-EL vectors, the BRK-precheck classifier still produced only the initial fetch pair after proving the direct BRK/vector control path can work independently, and the BRK/ERET resume diagnostic also produced only the initial fetch pair. Normalizing `SPSR_ELx` to same-EL h-mode with DAIF masked before `eret` also produced only the initial fetch pair, and adding a bounded post-`eret` UART marker before the reset continuation produced neither the marker nor a reset-induced boot sequence. That moves the active Pi 5 boundary away from UART interaction, sink writer, formatter data, generic direct branching, Rust crossing, section placement, simple BTI landing-pad compatibility, handler-local current-EL exception routing, `ELR_ELx` write safety, handler-local `SPSR_ELx` write safety, and post-`eret` reset-side-effect observability; it points at exception-return state/control itself before the continuation is reached.
- Risks: This does not re-enable Pi 5 dynamic early-console output. The normal PL011 formatted-output path remains guarded, and the next investigation should stay away from user-visible output until a smaller formatter-internal boundary is accepted.

## 2026-05-21 - Reopen Pi 5 Formatted println With Practical Serial Iteration

- Status: partially accepted locally; not accepted on Pi 5 hardware
- Context: Matthew asked to stop over-cautious formatter isolation and converge Talos toward the Daedalus public printing surface. Daedalus uses normal `print!`/`println!` macros over `format_args!` and a UART writer implementing `core::fmt::Write`.
- Decision: Restore the normal kernel-facing macro shape: `print!` calls `_print(format_args!(...))`, and `println!` appends newlines with the standard `concat!($fmt, "\\n")` pattern. `target::console::_print` keeps a static-string fast path via `fmt::Arguments::as_str()`, then uses `Write::write_fmt` for formatted arguments. This removes the Pi 5 compile-time refusal of formatted `println!` calls.
- Local validation: `cargo fmt --check`, `cargo -Zjson-target-spec test`, `scripts/rpi5-format-guard-check.sh`, `scripts/rpi5-image.sh`, `scripts/qemu-smoke.sh`, and `git diff --check` passed after the macro/backend changes.
- Hardware result: Several controlled Pi 5 runs were staged without restoring after every failure. UART10 with 32-bit data writes produced Talos-time bytes after BL31, but they were unreadable, so the failure was below `core::fmt` and in the hardware serial path. Preserving UART10 state plus polling remained garbled. Reusing the accepted RP1 UART0 first-light write/readback shape produced no Talos text after BL31 in the normal Rust image. UART10 byte data writes also failed to produce readable Talos output.
- Rationale: The Daedalus-like `println!` API now works for QEMU and builds for Pi 5. The remaining hardware blocker is reliable Pi 5 runtime serial ownership: the formatter path is no longer the first thing to blame when static output itself is not readable through the selected UART backend.
- Next step: Continue from the serial backend boundary. The most useful next probes are a normal-image early assembly marker just before Rust handoff using the accepted RP1 sequence, followed by a Rust-side first instruction marker using the same exact MMIO primitive, before further changes to `core::fmt`.

## 2026-05-21 - Handoff UART Diagnostic Uses an RE Rust-Entry Marker

- Context: The handoff diagnostic proved repeated `AH` markers are readable immediately before `rust_entry`, but the Rust-entry RP1-only `RR` and `TALOS-RR` park variants did not appear in raw serial even with fresh TFTP evidence and the diagnostic image left staged. Reviewers noted stray post-BL31 bytes and recommended separating Rust-entry control flow from RP1 GPIO/UART reconfiguration.
- Decision: Change the handoff UART diagnostic so `rust_entry` immediately calls an assembly-owned `RE\r\n` marker routine that does no GPIO, pad, or UART reconfiguration. The routine reuses the same direct data-register write plus flag-register readback shape as the readable `AH` path, writes to both UART10 at `0x107d001000` and RP1 UART0 pcie2 at `0x1f00030000`, delays between repeated markers, and parks instead of resetting.
- Required validation: Before hardware, local validation must show the image contains `AH` before `bl rust_entry`, `rust_entry` calls the handoff diagnostic immediately, the diagnostic branches to `rpi5_rust_entry_re_marker_park`, and the `RE` marker bytes are present. Hardware acceptance for this iteration requires a fresh TFTP window and a non-overlapping raw serial capture classified for `AH`, `RE`, and any post-`AH` raw bytes.
- Rationale: If `RE` appears, Rust entry and the basic post-entry UART write primitive are proven, and the bug moves to the RP1 setup/console backend. If `AH` stays readable but `RE` does not appear, the active boundary is the `_start` to `rust_entry` transition or Rust/ABI assumptions, not formatter internals.

## 2026-05-21 - Entry-Loop Diagnostic Proves Current-Run Kernel Entry And Serial Capture

- Status: accepted for current-run entry/capture; broader readable logging task still open
- Context: The capture-focused `RE` rerun served the expected staged image but did not capture current-run `Starting OS`, BL31, `AH`, or `RE`. That made another `RE` rerun a poor next step because it could not distinguish missing capture/current-run handoff from a Rust-entry marker failure.
- Decision: Add `TALOS_RPI5_ENTRY_LOOP_DIAGNOSTIC=1`, which emits repeated `EL\r\n` markers immediately after the arm64 Image header branch and `x0` preservation, before CPACR, BSS clearing, stack setup, `rust_entry`, or normal UART setup. It uses the same `rpi5_write_marker` data-register write plus flag-register readback helper as the readable `AH` marker and writes to both UART10 and RP1 UART0 pcie2 candidates.
- Local validation: `cargo fmt --check`, `cargo -Zjson-target-spec test`, `scripts/qemu-smoke.sh`, normal Pi 5 image build, readable boot-log diagnostic build, runtime UART probe diagnostic build, handoff UART diagnostic build, entry-loop diagnostic build, format guard, and `git diff --check` passed. The entry-loop image was `c94897012877a2e7a67ab4915e706f9043c15a289b45c42d346bb73b01c93fe8` / 79,688 bytes. Disassembly showed the `EL` loop before CPACR/BSS/stack/`rust_entry`.
- Hardware result: `rpi5-entry-loop-el-20260521T040000Z` published the 79,688-byte diagnostic and left it staged. The initial 90-second serial window captured only early Raspberry Pi firmware text and no TFTP kernel serve, so it was inconclusive. The extended window captured two fresh `da591740/kernel_2712.img` serves at 79,688 bytes and 21,594 exact `EL\r\n` markers over 87,085 raw serial bytes.
- Rationale: Current-run kernel entry and the AH-style serial write/readback primitive are now proven under the modern capture discipline. The next useful output iteration should convert this proven earliest-entry path from a two-letter marker into a bounded `TALOS:` phase line, then move the same primitive later toward `rust_entry` and the console backend.
- Risks: `EL` is a diagnostic marker, not the desired human-readable kernel log or Daedalus-like `println!` surface. Absence of `Starting OS`/BL31 in a short serial observe window is not sufficient negative evidence; extend capture before classifying a run that has not shown a fresh TFTP kernel serve.

## 2026-05-21 - Rust-Origin UART10 Output Accepted With No-Rodata Immediate Bytes

- Status: accepted for Rust-origin UART10 serial; normal formatted Pi 5 output still open
- Context: Earlier entry-loop diagnostics proved human-readable Talos-origin output before Rust and identified BCM2712 UART10 at `0x107d001000` as the lab-visible serial path. The next boundary was whether first Rust code after stack/BSS could emit readable bytes before boot-info parsing, target init, the console abstraction, or `core::fmt`.
- Decision: Keep the Pi 5 early UART backend on firmware-preserved UART10 and use 32-bit PL011 data-register writes with flag-register readback. For the diagnostic, avoid early rodata by emitting immediate byte constants from Rust inline assembly; pin the byte value to `w11` so scratch setup in `x9`/`x10` cannot corrupt marker bytes.
- Hardware result: The first Rust UART10 diagnostic served the 70,768-byte `97bb806b97821741e32d9941d11c207d668192aad907dab4d8ef5fcea88d042c` image but emitted a repeated binary-looking 20-byte pattern. Switching to word writes did not fix that because the static marker string still came from early rodata. The no-rodata immediate-byte diagnostic served the 70,048-byte `b30054a7e36207e073ba33d7ad8f53e61470217bf2c0472df032c94346a1e8e8` image and produced repeated `TALOS: rus` fragments, proving Rust-origin UART10 output but exposing inline-asm register overlap. The fixed-register run `rpi5-rust-uart10-fixedreg-20260521T0626Z` served the 70,048-byte `83cffc24d746e1f7128a2bace0341e9110928f62c46d9bf53cbec14203d73cd5` image twice from `da591740/kernel_2712.img`; bounded serial captured 920+ exact `TALOS: rust-uart10\\r\\n` lines and no stale `TALOS: uart10`/`TALOS: entry` counts in the post-drain window.
- Rationale: This separates the reliable serial path from unresolved early Rust data-addressing. Rust execution and UART10 output are now hardware-proven with immediate bytes, while static rodata and generic formatting remain separate contracts to prove before treating normal Pi 5 `println!` as accepted.
- Risks: The accepted diagnostic is deliberately formatter-free and no-rodata. It does not prove formatted output, panic/exception reporting, or that all early Rust static data references are safe under the current load/link contract.

## 2026-05-21 - Pi 5 Println Phase Diagnostic Is Not Yet Accepted

- Status: not accepted on Pi 5 hardware
- Context: After accepting Rust-origin UART10 output with immediate bytes, the next practical layer was the normal Daedalus-like `print!`/`println!` surface on the Pi 5 firmware-preserved UART10 backend.
- Decision: Add `TALOS_RPI5_PRINTLN_PHASE_DIAGNOSTIC=1`, which reaches the normal Pi 5 `kernel_main` path and then loops on `println!("TALOS: println phase")` and `println!("TALOS: println count {}", count)`. This keeps the public macro/backend shape under test instead of returning to isolated formatter internals.
- Hardware result: Run `rpi5-println-phase-20260521T0648Z` served the 84,024-byte `abc4af0de6611f421396ebb9c117023bacf3e5702eb06a97a45ccf7ad93a4f35` diagnostic twice from `da591740/kernel_2712.img`, but serial captured zero `TALOS: println phase` or `TALOS: println count` lines. A follow-up run, `rpi5-println-phase-linkbase0-20260521T0655Z`, temporarily changed the Pi 5 linker base from `0x80000` to `0x0` to test the Image-header/load-address hypothesis. It served the 84,024-byte `65c3e699b42cd7803a0e524ca28a8f1b559f56770f0c8f85bdfcc735bc4c4fd6` diagnostic twice but also produced zero println lines, so that linker change was reverted.
- Rationale: The accepted no-rodata Rust UART10 diagnostic proves Rust can emit human-readable bytes, but normal `println!` still depends on static format strings and formatter code paths that are not hardware-accepted. The failed link-base experiment is useful negative evidence but not a durable architecture change.
- Next step: Keep the `println!` diagnostic available, but localize the static-data/format-string boundary with a narrower no-rodata or address-classification diagnostic before claiming normal Pi 5 formatted output.

## 2026-05-21 - Phase P0 Reset Diagnostic Accepts Earliest Human-Readable UART10

- Status: accepted for narrow P0 assembly-entry serial output; broader serial/println task still open
- Context: Phase-ladder and static/format-boundary diagnostics had produced confusing no-marker runs when the capture window missed handoff serial. The accepted boundary-entry and Rust UART10 diagnostics proved readable output was possible, but the current phase-ladder image needed a minimal reset-classified baseline.
- Decision: Add `TALOS_RPI5_PHASE_P0_RESET_DIAGNOSTIC=1` as a phase-ladder variant that writes repeated `TALOS: P0 asm-entry` markers from `_start` through the accepted UART10 write/readback primitive and then requests PSCI reset. The P0 loop reloads the marker pointer before every helper call because `rpi5_write_marker_payload` consumes `x10`.
- Hardware result: Run `rpi5-phase-p0-reset-20260521T0915Z` served the 70,400-byte `e2b0297e410be08f512491273b26b4147aa711ee319826cd651aa0a26b09f2ce` image from `da591740/kernel_2712.img`. Post-TFTP serial captured one exact `TALOS: P0 asm-entry` line plus BL31/Starting OS context, followed by a partial/garbled second marker before reset. Post-hardware review accepted the narrow P0 claim.
- Rationale: This pins the current Cargo-linked phase diagnostic tree at earliest assembly entry. It does not prove CPACR/BSS/stack, Rust entry, rodata, the console backend, or formatting.
- Next step: Add a P1 reset variant that preserves the P0 entry marker, continues through CPACR, BSS clearing, and stack setup, emits repeated `TALOS: P1 asm-stack` lines, and resets before Rust.

## 2026-05-21 - Phase P1 Full Marker Is Not Yet Accepted

- Status: not accepted for exact full P1 line; partial post-stack signal observed
- Context: The accepted P0 diagnostic proved earliest assembly-entry serial. The next boundary was whether serial remained usable after CPACR, BSS clearing, and stack setup, before Rust.
- Decision: Add `TALOS_RPI5_PHASE_P1_RESET_DIAGNOSTIC=1`, preserving the P0 breadcrumb, then emitting repeated `TALOS: P1 asm-stack` lines and resetting before `rust_entry`.
- Hardware result: Run `rpi5-phase-p1-reset-20260521T0930Z` served the 70,400-byte `65b74bc0b3e857b37d069484dc7f3400e72d9fa5f43a6d74f0e54ae000b8a752` image five times from `da591740/kernel_2712.img`. Serial captured exact `TALOS: P0 asm-entry` and a partial `TALOS: P1` fragment, but no exact `TALOS: P1 asm-stack` line. Post-hardware review rejected the exact P1 claim.
- Rationale: The partial P1 fragment suggests the post-stack path may be reached, but exact bytes are still the acceptance bar.
- Next step: Add `TALOS_RPI5_PHASE_P1_SHORT_RESET_DIAGNOSTIC=1`, which emits repeated shorter `TALOS: P1` lines after stack setup and resets before Rust. If that is exact and repeatable, restore the full P1 acceptance string.

## 2026-05-21 - Phase P1 Short Marker Accepts Post-Stack UART10

- Status: accepted for narrow post-stack `TALOS: P1` serial output; full P1 line and Rust entry remain open
- Context: The full P1 reset diagnostic produced an exact P0 line and a partial P1 fragment, which left ambiguity between post-stack reachability and UART transmit/reset timing for a longer string.
- Decision: Add `TALOS_RPI5_PHASE_P1_SHORT_RESET_DIAGNOSTIC=1`, preserving the P0 breadcrumb, then emitting repeated short `TALOS: P1` lines after CPACR, BSS clearing, and stack setup before requesting PSCI reset.
- Hardware result: Capture-focused run `rpi5-phase-p1-short-reset-rerun-20260521T1031Z` served the 70,400-byte `3d9154708e749634a363f84cb5cbd9598bba2b1540b84ca6d897cc642f9f988c` image from `da591740/kernel_2712.img`. The bounded serial windows captured BL31/Starting OS context, one exact `TALOS: P0 asm-entry`, and one exact `TALOS: P1` line before reset.
- Rationale: This proves the current Cargo-linked Pi 5 image can emit human-readable UART10 output after stack setup. The earlier full-marker failure is now best treated as an output-length or reset-drain timing issue, not proof that the post-stack path is unreachable.
- Next step: Move the same reset-classified phase ladder to the next boundary before `rust_entry`, while keeping output short or adding an explicit transmit-drain delay before reset.

## 2026-05-21 - Paced Phase P2 Accepts Pre-Rust UART10

- Status: accepted for narrow pre-`rust_entry` `TALOS: P2` serial output with paced writes; broader `println!` path remains open
- Context: The P1 short run proved the post-stack boundary. The first P2 short run served the expected image and captured exact P0/P1 but no exact P2, only a partial next marker, which pointed at transmit pacing or reset drain rather than a control-flow failure.
- Decision: Keep `TALOS_RPI5_PHASE_P2_RESET_DIAGNOSTIC=1` as the pre-Rust reset boundary and use a paced assembly helper for this diagnostic. The helper waits, bounded, for UART10 TXFF to clear before each byte and waits, bounded, for TXFE before PSCI reset.
- Hardware result: Run `rpi5-phase-p2-paced-reset-20260521T1039Z` served the 70,400-byte `fece3f608932f40c9c1885fa22b2b050ac7b16910f94bdc739246766bbbb6e90` image from `da591740/kernel_2712.img`. Bounded serial captured BL31/Starting OS context, exact `TALOS: P0 asm-entry`, exact `TALOS: P1`, and 64 exact `TALOS: P2` lines.
- Rationale: The accepted paced P2 run proves readable UART10 survives through the assembly path immediately before `rust_entry`. It also explains several partial-line reset diagnostics: fast reset after unpaced writes can truncate otherwise reachable output.
- Next step: Carry the same bounded pacing/drain contract into the Pi 5 UART10 console path, then rerun the normal `print!`/`println!` phase diagnostic before spending more time on generic formatter internals.

## 2026-05-21 - Marked Println Run Reaches Rust But Fails Static String Output

- Status: partially accepted as localization evidence; normal Pi 5 `println!` remains unaccepted
- Context: After P2 accepted the paced pre-Rust UART10 contract, the console backend was changed to poll TXFF before each PL011 write and wait, bounded, for TXFE at the end of flushed writes. The normal `TALOS_RPI5_PRINTLN_PHASE_DIAGNOSTIC` still produced no println lines in hardware.
- Decision: Add no-rodata immediate `TALOS: phase 0` through `TALOS: phase 4` markers around the normal println path. Phase 0 is the first instruction in `rust_entry`; phase 1 follows `BootInfo` parsing; phase 2 follows target init; phase 3 follows exception-vector init; phase 4 is immediately before the diagnostic println loop in `kernel_main`.
- Hardware result: Run `rpi5-println-phase-markered-console-20260521T1059Z` served the 84,072-byte `d48f4318df32548f6f951265270f80a0d524f32619f10ccbc2bb4951b434992a` image twice from `da591740/kernel_2712.img`. Bounded serial captured BL31/Starting OS plus exact `TALOS: phase 0`, `phase 1`, `phase 2`, and `phase 3`. It then captured binary-looking bytes where the first normal static println should have emitted `talos: boot start`; `phase 4`, `talos: boot start`, and `TALOS: println phase` were absent.
- Rationale: The current failure is no longer serial ownership or Rust-entry reachability. Immediate bytes remain readable through exception init, but normal static-string output corrupts before the println loop. The next productive boundary is static string/rodata addressing or `fmt::Arguments::as_str()` representation on the Pi 5 image, not more UART micro-probes.

## 2026-05-21 - Pi 5 Static String Pointers Need Runtime Relocation

- Status: accepted for narrow static-string address localization; normal Pi 5 `println!` remains layered work
- Context: The marked println run reached phase 3, then emitted binary-looking bytes where the first static println should have printed `talos: boot start`. That left the immediate question of whether `fmt::Arguments::as_str()` exposed a static string whose pointer still referenced the linked rodata address while the image was running elsewhere.
- Decision: Add `TALOS_RPI5_RODATA_ADDRESS_DIAGNOSTIC=1`, which avoids formatter output and prints immediate-byte labels plus computed address values. It compares a runtime code label against its linked literal address, computes the runtime placement delta, then samples the first static boot string at both its linked pointer and `pointer + delta`.
- Hardware result: The first rodata run captured all expected lines but had a shifted local hex printer, so it was kept as supporting evidence only. The fixed-hex run `rpi5-rodata-address-fixedhex-20260521T1130Z` served the 70,704-byte `c63521a95d529f2b7b14916881d059e5959f73f000f072879d45dc33ba61d777` image from `da591740/kernel_2712.img`. Bounded serial captured `runtime=0x201760`, `linked=0x81760`, `delta=0x180000`, `str-ptr=0x8ee20`, `str-len=0x12`, non-text bytes at the linked string pointer, and `talos: boot start\\n` at `str-ptr + delta`.
- Rationale: This explains the binary bytes in the marked println run without changing the accepted UART10 ownership story. Early Pi 5 static string pointers currently require runtime relocation or PC-relative access before they are safe to hand to the console backend.
- Risks: The observed `0x180000` delta is evidence for this boot contract, not a constant to hard-code. This result does not accept formatted arguments, panic/exception reporting, or the generic formatter path; those remain separate layers after static strings print reliably.

## 2026-05-21 - Pi 5 Early Console Uses Bounded Runtime Rodata Relocation

- Status: accepted for static println strings; formatted arguments still pending
- Context: Matthew pointed out that Talos already has the Phil Opp-style bare-metal baseline: `no_std`/`no_main`, custom target, `panic=abort`, redzone disabled, `build-std` core/compiler_builtins, custom linker, and an explicit panic handler. The stronger current evidence is not that `core::fmt` is inherently broken; it is that the Pi 5 lab/network boot path executes the image at linked address plus `0x180000`, while static string pointers still contain linked rodata addresses. Daedalus likely avoided this on Pi 4 because its firmware/load path matched the linker assumption at `0x80000`.
- Decision: Keep the near-term Pi 5 early console deliberately relocation-aware instead of broad probing formatter internals. The console computes the runtime placement delta from code labels and wraps the UART writer so every `write_str` call relocates only slices whose linked address range falls inside linker-provided `__rodata_start..__rodata_end`. Non-rodata slices, such as formatter-generated numeric buffers, are left unchanged. This is the constrained early-console option, not a final MMU/loader design; future options remain linking at the actual runtime address or making the early image properly position-independent.
- Hardware result: Run `rpi5-println-relocated-static-20260521T1230Z` served the 71,088-byte `b67c7f223d4eebf5013083e0cb533088a396bf00f229d038c835246c480c6526` image twice from `da591740/kernel_2712.img`. Bounded serial captured exact phase 0 through phase 4 markers, plus relocated static println lines: `talos: boot start`, `talos: board raspberry-pi-5-bcm2712`, `talos: console early-uart fmt`, and `TALOS: println phase`. It did not capture `TALOS: println count`.
- Rationale: This accepts the static-string layer and keeps effort focused on the specific Pi 5 link/load contract. The next useful hardware discriminator is formatted println with the same rodata-bounded relocation wrapper, because it tests whether formatter callbacks now handle rodata format pieces and non-rodata numeric output without corrupting either.
- Risks: This does not make arbitrary early pointers safe, does not validate panic/exception reporting, and does not settle the final Pi 5 load strategy. The observed `0x180000` delta must remain runtime-derived rather than encoded as a constant.

## 2026-05-21 - Pi 5 Dynamic core::fmt Arguments Remain Unaccepted

- Status: accepted evidence for relocated static println replay; dynamic `core::fmt` arguments rejected for now
- Context: The relocated static-string layer made the normal boot-start println strings readable on Pi 5. The remaining discriminator was whether the generic formatter path could also emit a runtime numeric argument once every `write_str` callback relocated only linker-bounded rodata slices.
- Hardware result: Run `rpi5-println-relocated-format-20260521T1242Z` served the 75,184-byte `ad8a24dab7454584fb124f3ffe63bd82817f97057f6dd783f062e4a449ead805` image from `da591740/kernel_2712.img`. Bounded serial captured BL31/Starting OS, exact `TALOS: phase 0` through `TALOS: phase 4`, the relocated static lines `talos: boot start`, `talos: board raspberry-pi-5-bcm2712`, `talos: console early-uart fmt`, and `TALOS: println phase`. It did not capture `TALOS: println count`.
- Decision: Do not spend the next iteration on broad `core::fmt` internals. Keep the accepted Daedalus-like static `print!`/`println!` surface and add a constrained Talos-owned early numeric formatting path for Pi 5 phase output using the existing `early_format` helpers. The generic formatter can be revisited after the early console and link/load contract are more stable.
- Risks: This is not full formatted println parity. It is a practical bring-up layer for bounded numeric diagnostics while preserving the accepted UART10 ownership and rodata relocation behavior.

## 2026-05-21 - Pi 5 Early Decimal Formatting Works in Diagnostic Path

- Status: accepted for diagnostic static println plus Talos-owned decimal output; normal image still not accepted
- Context: After generic dynamic `core::fmt` arguments failed to emit `TALOS: println count`, the println phase diagnostic kept static `println!` for `TALOS: println phase` and changed the count line to use `target::console::write_static` plus `write_dec_usize`. This tests the intended near-term Pi 5 approach: Daedalus-like static printing where it is already reliable, with Talos-owned early numeric formatting for bounded diagnostics.
- Hardware result: Run `rpi5-println-early-dec-20260521T1405Z` served the 75,352-byte `a476880eab58f5600d5762c7a82b2d8553faedb5ad3f1225140005e5604acb2a` image from `da591740/kernel_2712.img`. Bounded serial captured BL31/Starting OS, exact `TALOS: phase 0` through `TALOS: phase 4`, relocated static boot lines, and 13 pairs of `TALOS: println phase` plus `TALOS: println count N` for counts 0 through 12.
- Follow-up result: Normal Pi 5 boot-info/services output was rewritten to avoid generic dynamic `core::fmt` and use static strings plus `early_format` hex/decimal helpers. Local gates passed, but hardware run `rpi5-normal-early-console-20260521T1410Z` served the 75,768-byte `3175b86941835ab50425c90d02fb987c79688930892cff28da87675823974d0e` normal image twice and captured no Talos kernel serial lines. This rejects normal-image boot-log acceptance for now and points at a normal-vs-diagnostic codegen/control-flow difference.
- Decision: Treat early decimal formatting as accepted only inside the println phase diagnostic. Do not claim normal Pi 5 boot logs are ready until a narrow marker or codegen comparison explains why the non-diagnostic image fails before observable Talos serial.
- Risks: The accepted diagnostic path is useful but still cfg-shaped. Normal-image acceptance needs another bounded localization step before this becomes a durable boot-log capability.

## 2026-05-21 - Normal Pi 5 Path Emits Early Phase and Static Boot Lines

- Status: accepted for normal-image reachability, formatter-free phase lines, static boot lines, and status line; readable hex fields remain pending
- Context: The diagnostic println path accepted static lines and Talos-owned decimal output, but a normal image with the same early helpers initially produced no Talos serial. The next question was whether the non-diagnostic image reached Rust and the normal console path at all.
- Decision: Add formatter-free UART10 phase lines to the normal Pi 5 path at `rust_entry`, after `BootInfo` parse, after target init, after exception init, and at `kernel_main`. These lines use immediate byte writes through the proven UART10 path and do not depend on rodata, `core::fmt`, or the early console wrapper.
- Hardware result: After one early-capture negative and a successful current println diagnostic control, rerun `rpi5-normal-phase-lines-rerun-20260521T142534Z` served the 79,864-byte `088714ec8104ad363fff6cafd375278b476d10882ae701a54a6d5ab26f995089` image from `da591740/kernel_2712.img`. Bounded serial captured BL31/Starting OS, exact `TALOS: rust_entry`, `TALOS: boot info parsed`, `TALOS: target init`, `TALOS: exceptions ready`, `TALOS: kernel_main`, static boot lines, and `talos: status early boot log ready`.
- Follow-up result: The same run showed corrupt DTB hex fields because `early_format::write_hex_*` used a linked rodata digit table. A no-rodata hex-digit fix passed local gates, but hardware run `rpi5-normal-readable-boot-20260521T142753Z` served the 79,896-byte `5e817b5ba741c1054ad1ac79abd2443f3752177dc5152ae89f76f6aa967d52a7` image and did not reach `Starting OS` or Talos serial in the captured window. That hex-fixed image is not accepted yet.
- Rationale: Normal Rust reachability and the static boot-log layer are now proven outside diagnostic cfg, but readable boot-info/service numeric fields still need a bounded follow-up. Keep generic dynamic `core::fmt` unaccepted; use Talos-owned early numeric formatting once the no-rodata hex fix is classified.
- Next step: Rerun or narrow the hex-fixed normal image. Acceptance should require the same normal phase/static lines plus readable `dtb=0x...` fields before the broader readable boot-log task is marked accepted.

## 2026-05-21 - Normal Pi 5 Hex Fields Remain Unaccepted

- Status: not accepted for readable normal numeric fields
- Context: The accepted normal phase/static boot-log image exposed corrupt bytes after `dtb=0x`. The first no-rodata hex fix needed classification because its initial hardware run did not reach Talos serial.
- Hardware result: A rerun of the 79,896-byte `5e817b5ba741c1054ad1ac79abd2443f3752177dc5152ae89f76f6aa967d52a7` image reached the normal phase/static lines, but both DTB fields still emitted binary bytes after `0x`. A stack-backed ASCII digit helper (`17b824c9...` / 79,896 bytes) and a Pi 5 direct UART10 hex writer (`de89763a...` / 79,792 bytes) passed local gates and were served twice each, but neither reached `Starting OS` or Talos serial in the captured hardware windows.
- Decision: Do not accept the normal numeric field layer yet. Keep the direct Pi 5 hex writer staged as the latest development image, but compare image layout/codegen against the accepted `088714ec...` normal image before spending another hardware run.
- Rationale: The rerun proves normal boot still reaches the accepted static layer with the old hex shape, while generated hex digits are still unsafe. The two follow-up no-start images suggest the next bounded work is local comparison rather than another blind serial retry.

## 2026-05-21 - Normal Pi 5 Numeric Fields Use Arithmetic Hex

- Status: accepted for normal-image readable boot-info/services numeric fields; generic dynamic `core::fmt` remains unaccepted
- Context: The match-arm follow-up image eventually fetched from TFTP but emitted only `NUL`/`x` in the bounded serial window. Local disassembly showed `write_early_hex_digit` had compiled to an absolute linked-address jump table, reintroducing the Pi 5 rodata relocation hazard.
- Decision: Replace the match-arm hex digit dispatch with wrapping arithmetic and one direct UART10 byte write. Extend `scripts/rpi5-format-guard-check.sh` so local gates reject jump tables, panic paths, or literal data inside `write_early_hex_digit`.
- Hardware result: Run `rpi5-normal-readable-boot-arithhex-20260521T1520Z` served the 75,128-byte `3af8078e640c7da010cc88851edb29899052c7ca590f224beb164847e5d78741` normal image twice from `da591740/kernel_2712.img`. Bounded serial captured `kernel_main`, relocated static boot lines, `talos: boot info: dtb=0x2efec600 core=0 el=2 target=talos-rpi5-bcm2712`, `talos: services: uart=firmware-preserved timer=arm-generic irq=gic-v2 mmio_regions=7 dtb=0x2efec600`, and `talos: status early boot log ready`.
- Rationale: This keeps the near-term Pi 5 path focused on reliable serial output and Talos-owned early numeric formatting instead of generic formatter internals. The disassembly guard protects the specific failure mode found during bring-up.
- Risks: This does not validate dynamic `println!("... {}", value)`, panic/exception reports, or the final loader/link strategy. The early numeric path is a bring-up layer behind the console surface, not the final formatting architecture.

## 2026-05-21 - Pi 5 BRK Exception State Has a Formatter-Free Report

- Status: accepted for a narrow synchronous BRK diagnostic report on Pi 5 hardware; Rust exception handler entry remains unaccepted.
- Context: Normal Pi 5 boot logs and early numeric formatting were already accepted. The next exception task installed the relocated AArch64 vector table and triggered a deliberate `BRK` after normal boot/status output. Initial runs proved `VBAR_EL2=0x200800` and `TALOS: vector-entry`, but the Rust exception handler did not produce readable ESR/ELR/FAR output.
- Decision: Keep the exception-report diagnostic formatter-free and move the accepted report to the assembly vector path. The diagnostic reads the current EL's ESR/ELR/FAR registers after vector entry and writes fixed-width hex fields directly to firmware-preserved UART10 before any Rust handler prologue, rodata string, or `core::fmt` path.
- Hardware result: Run `rpi5-readable-exception-report-asm-report-fixedhex-20260521T175131Z` served the 78,800-byte `8e6ea39dc5b6a8b30e682f87f354e89669aad02e222e969df3e3e264a1ed728c` image twice from `da591740/kernel_2712.img`. Bounded serial captured normal Talos boot/status, `TALOS: before BRK vbar=0x200800 el=2`, `TALOS: vector-entry`, `TALOS: handler-entered vector=0x0000000000000004`, and `TALOS: exception-info esr=0x00000000f2000000 elr=0x0000000000203a68 far=0x1a9bbff767d79fef`.
- Rationale: This accepts the hardware-critical exception state path without mixing in unresolved Rust handler entry and formatting risks. It also confirms the BRK syndrome value and current-EL vector slot under the Pi 5 handoff.
- Risks: The accepted report is diagnostic assembly, not the final exception subsystem. It does not validate Rust exception handler entry, panic report formatting, exception return/resume, IRQ/FIQ/SError dispatch, or the generic dynamic formatter.

## 2026-05-21 - BRK Diagnostic Reaches the Rust-Owned Handler Symbol

- Status: accepted for the vector path branching to the `rust_exception_handler` symbol through a Rust-owned assembly shim; normal compiler-generated Rust handler body remains unaccepted.
- Context: The accepted assembly BRK report proved VBAR, vector entry, and readable ESR/ELR/FAR state, but earlier attempts to enter the Rust handler body failed before a Rust-origin diagnostic byte. Local disassembly showed the normal handler begins with stack-frame stores and rodata/console helper calls, so the next narrow question was whether the vector path can cross to the handler symbol at all after the assembly state report.
- Decision: Keep the formatter-free assembly exception-state report as a guardrail, then branch to a `rust_exception_handler` symbol defined from the Rust exception module with `global_asm!`. The shim emits `TALOS: rust-handler` using immediate UART10 bytes and requests PSCI reset. This deliberately avoids a compiler-generated Rust prologue for this diagnostic; that prologue is a separate follow-up.
- Hardware result: Rerun `rpi5-rust-exception-handler-entry-rerun-20260521T1821Z` served the 78,800-byte `51bc30b120a4cb270eabb79ecb427582f9b410e13bbf2bfc84c9f3c62fd51980` image twice from `da591740/kernel_2712.img`. Bounded serial captured normal Talos boot/status, `TALOS: before BRK vbar=0x200800 el=2`, `TALOS: vector-entry`, `TALOS: handler-entered vector=0x0000000000000004`, `TALOS: exception-info esr=0x00000000f2000000 elr=0x0000000000203b70 far=0x9b9bfff7f7dfdfdf`, and `TALOS: rust-handler`.
- Rationale: This proves the exception vector path can call into the Rust-owned handler symbol after reporting exception state. The remaining failure class is now inside the normal Rust handler body/prologue or its formatting/static-data dependencies, not the vector branch target itself.
- Risks: This is still diagnostic code. It does not validate a normal Rust stack frame, panic-style exception formatting, exception return/resume, non-sync vectors, or generic dynamic `core::fmt`.

## 2026-05-21 - BRK Diagnostic Enters a Compiler-Generated Rust Handler Body

- Status: accepted for a minimal compiler-generated Rust exception handler body after the assembly BRK state report; panic-style formatting and exception return remain unaccepted.
- Context: The prior accepted BRK diagnostic proved the vector path could branch to the `rust_exception_handler` symbol through a Rust-owned assembly shim. The remaining narrow question was whether the normal Rust function prologue/body boundary itself could run after the assembly report.
- Decision: Replace the diagnostic `global_asm!` handler shim with a normal `extern "C" fn rust_exception_handler` that emits a no-rodata immediate-byte `TALOS: rust-body` marker through UART10, drains TX, and requests PSCI reset. Keep the assembly `TALOS: handler-entered` and `TALOS: exception-info` report as the guardrail before entering Rust.
- Local validation: `cargo fmt --check`, `cargo -Zjson-target-spec test`, `scripts/qemu-smoke.sh`, the Pi 5 exception-report diagnostic image build, `scripts/rpi5-format-guard-check.sh`, and `git diff --check` passed. Disassembly showed `rust_exception_handler` begins with compiler-generated stack-frame stores, then calls the Rust marker helper, UART10 drain helper, and reset helper.
- Hardware result: Run `rpi5-rust-exception-compiler-handler-20260521T1834Z` staged the 78,800-byte `d8d4119a5a792b9973eb7ebfbc9683187945298c7eb94f258d6fd613c617d778` image under root and `da591740/` kernel paths. TFTP served `da591740/kernel_2712.img` repeatedly, and the extended run-bounded serial capture showed normal Talos boot/status output, `TALOS: before BRK vbar=0x200800 el=2`, `TALOS: vector-entry`, `TALOS: handler-entered vector=0x0000000000000004`, `TALOS: exception-info esr=0x00000000f2000000 elr=0x0000000000203eb4 far=0x9abbbfffffd7dfef`, and `TALOS: rust-body`.
- Rationale: The old failure is now narrowed away from the vector branch target and the Rust prologue itself. Remaining exception-reporting work should focus on the richer Rust handler body dependencies: static-data use, panic-style formatting, controlled reset/park policy, and later exception return/resume.
- Risks: This is still a diagnostic body with immediate UART10 bytes. It does not validate formatted panic/exception output, exception return, non-sync vectors, or generic dynamic `core::fmt`.

## 2026-05-21 - Rust BRK Handler Emits Formatter-Free Exception Fields

- Status: accepted for a bounded Rust-origin BRK exception report with vector, ESR, ELR, and FAR fields; panic-style formatting and exception return remain unaccepted.
- Context: The previous BRK diagnostic proved that the vector path could branch into a compiler-generated Rust handler body after the assembly exception-state report. The next narrow question was whether that Rust body could consume the exception arguments and emit a readable report without generic dynamic `core::fmt`.
- Decision: Keep the assembly `TALOS: handler-entered` and `TALOS: exception-info` report as a guardrail, then have the normal Rust `rust_exception_handler` call a formatter-free helper that writes static labels plus early hex values for vector, ESR, ELR, and FAR before draining UART10 and requesting PSCI reset.
- Local validation: `cargo fmt --check`, `cargo -Zjson-target-spec test`, `scripts/qemu-smoke.sh`, the Pi 5 exception-report diagnostic image build, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, archive review, and targeted disassembly inspection passed. Disassembly showed a compiler-generated handler stack frame, stored exception arguments, a call to the Rust report helper, UART10 drain, and reset.
- Hardware result: Run `rpi5-rust-exception-report-20260521T185843Z` staged the 78,848-byte `da72dc94cbc52344689ae24a5c1c3f176888f9a97e747ed52e5f31e317a1b849` image under root and `da591740/` kernel paths. Bounded serial captured normal Talos boot/status output, `TALOS: before BRK vbar=0x200800 el=2`, `TALOS: vector-entry`, `TALOS: handler-entered vector=0x0000000000000004`, `TALOS: exception-info esr=0x00000000f2000000 elr=0x0000000000203b88 far=0x9abbbfffffd7dfef`, and `TALOS: rust-exception vector=0x4 esr=0xf2000000 elr=0x203b88 far=0x9abbbfffffd7dfef`.
- Rationale: This accepts the practical next layer: Rust-owned exception code can receive the assembly-captured state and produce a human-readable formatter-free report. The immediate follow-up should move toward a panic-style exception line or shared exception-reporting surface while preserving the accepted assembly guardrail until the Rust path is broad enough.
- Risks: This does not validate exception return/resume, non-sync vectors, full panic formatting, or generic dynamic `core::fmt`.

## 2026-05-21 - Pi 5 Panic Handler Emits a Formatter-Free Fatal Line

- Status: accepted for a bounded panic-handler fatal report on Pi 5 hardware; full panic formatting remains unaccepted.
- Context: The accepted Rust BRK handler can now emit formatter-free vector/ESR/ELR/FAR fields. The next narrow panic-style question was whether a normal Rust panic handler, reached after the normal boot/status path, can produce a human-readable fatal line without formatting `PanicInfo` or depending on generic dynamic `core::fmt`.
- Decision: Keep the Pi 5 panic path formatter-free for now. The handler writes a static prefix plus early decimal/hex fields for current EL and VBAR, drains UART10, then halts. A new `TALOS_RPI5_PANIC_REPORT_DIAGNOSTIC` triggers `panic!("talos diagnostic panic")` after the normal boot/status line so the panic report is accepted separately from BRK exception handling.
- Local validation: `cargo fmt --check`, `cargo -Zjson-target-spec test`, `scripts/qemu-smoke.sh`, normal Pi 5 image build, panic-report diagnostic image build, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, archive review, and targeted strings/disassembly inspection passed. Inspection showed the panic handler calls the formatter-free static/decimal/hex console helpers and the diagnostic image contains `TALOS: panic el= vbar=`.
- Hardware result: Run `rpi5-panic-report-20260521T1917Z` staged the 75,192-byte `91b313d58de8b0883b4a35a804428d06679a6ae3a60e38330aba6e9c9858237f` image under root and `da591740/` kernel paths. TFTP served `da591740/kernel_2712.img` twice, and run-bounded serial captured normal boot/status output followed by `TALOS: panic el=2 vbar=0x200800`.
- Rationale: This proves a Rust panic-handler-owned, human-readable fatal report is viable on the accepted Pi 5 early serial path. It gives Talos a useful panic line while the richer formatter and panic message path remain unaccepted.
- Risks: This does not validate formatting `PanicInfo`, source locations, panic messages, exception return/resume, non-sync vectors, or generic dynamic `core::fmt`.

## 2026-05-21 - Pi 5 Panic Source Location Remains Deferred

- Status: not accepted for `PanicInfo::location()` in the early Pi 5 panic handler; accepted fatal EL/VBAR line remains the supported panic report.
- Context: After accepting the formatter-free panic fatal line, the next task tried to add bounded source-location fields without formatting `PanicInfo` or using generic dynamic `core::fmt`.
- Hardware result: The full `PanicInfo::location()` variant staged a 79,296-byte kernel and the static-file/line-column variant staged a 75,216-byte kernel; both were TFTP-served but produced no current-run Talos panic-location output. A split-prefix probe that wrote the accepted EL/VBAR line before touching location data staged a 79,376-byte kernel and was also served, but current-run serial showed only firmware. A no-`PanicInfo` control with the same split/probe shape staged a 75,256-byte kernel and was served on rerun, but likewise produced no current-run Talos output.
- Decision: Do not add source-location fields to the Pi 5 panic handler yet. Restore the accepted formatter-free `TALOS: panic el=... vbar=...` line as the stable behavior and treat source locations as a later task after the early panic path has stronger code-layout and capture guardrails.
- Rationale: The accepted fatal line is useful and hardware-proven. The source-location experiments repeatedly crossed an unaccepted early-runtime/code-layout boundary before yielding durable panic output, so continuing to layer location logic onto the panic handler is lower value than moving on to the next serial/exception capability.
- Risks: Panic reports still lack file/line/column and message payloads on Pi 5. This decision does not rule out a later Talos-owned call-site panic macro or a safer source-location channel once early runtime constraints are better understood.

## 2026-05-21 - Normal Pi 5 Rust BRK Handler Emits Formatter-Free Exception Report

- Status: accepted for the normal Rust synchronous BRK fatal report on Pi 5 hardware; exception return and non-sync vectors remain unaccepted.
- Context: The assembly BRK diagnostic and Rust-origin report were already accepted, but the normal `rust_exception_handler` branch without the assembly diagnostic report still needed proof. The first normal diagnostic reached `TALOS: before normal BRK` but printed no report. Local disassembly showed the handler converted the vector through `ExceptionVector::from/name`, which generated linked-address jump-table reads before the first report output.
- Decision: Keep the normal Pi 5 exception report formatter-free. Avoid jump-table vector-name dispatch in the early exception path by writing the vector name with a simple if-chain, and keep a diagnostic-only no-rodata `TALOS: normal-exception-handler` marker plus PSCI reset around the acceptance run.
- Local validation: `cargo fmt --check`, `cargo -Zjson-target-spec test`, `scripts/qemu-smoke.sh`, normal Pi 5 image build, `scripts/rpi5-format-guard-check.sh`, normal-exception diagnostic image build, `git diff --check`, and targeted disassembly inspection passed. The handler, vector-name writer, and marker had no `ldrsw` or `br xN` jump-table pattern.
- Hardware result: Run `rpi5-normal-exception-report-marker-reset-20260521T202231Z` staged the 79,144-byte `28c850e6e9732933befea856f9af2251673549820901fe9f95f4a6a942648d32` image under root and `da591740/` kernel paths. TFTP served `da591740/kernel_2712.img` twice. Bounded serial captured normal boot/status, `TALOS: before normal BRK vbar=0x200800 el=2`, `TALOS: normal-exception-handler`, `talos exception: current-spx-sync`, and `exception-info: esr=0xf2000000 elr=0x2045e4 far=0x1a9bbff767d79fef`, followed by the expected reset.
- Rationale: This accepts the normal Rust handler report shape independently of the assembly diagnostic report. The result keeps the current Pi 5 fatal exception path practical and human-readable while avoiding unresolved generic formatter work.
- Risks: The no-rodata marker and reset are diagnostic-only. This does not validate exception return/resume, IRQ/FIQ/SError dispatch, source-rich panic formatting, or generic dynamic `core::fmt`.

## 2026-05-21 - Pi 5 println! Supports One Early Numeric Placeholder

- Status: accepted for a constrained Pi 5 `print!`/`println!` one-placeholder early numeric path; generic dynamic `core::fmt` remains unaccepted.
- Context: Static `println!` and explicit `target::console::write_dec_*` calls were already accepted on Pi 5 hardware, but the familiar `println!("... {}", value)` shape still routed through generic `fmt::Arguments` internals that previously failed under the current Pi 5 relocation contract.
- Decision: Specialize the Pi 5 macro path for a single literal `{}` placeholder. The macro calls a Talos-owned `_print_one` helper, relocates the format string with the early rodata-bound wrapper, writes the prefix/suffix through the accepted early console, and writes integer/string arguments through small `early_format` helpers. Multiple placeholders, format specifiers, and generic dynamic `core::fmt` are intentionally left for later.
- Local validation: `cargo fmt --check`, `cargo -Zjson-target-spec test`, `scripts/qemu-smoke.sh`, normal Pi 5 image build, `scripts/rpi5-println-phase-diagnostic-image.sh`, `scripts/rpi5-format-guard-check.sh`, and `git diff --check` passed. Static inspection showed `rpi5_println_phase_diagnostic` calls `talos::target::console::_print_one::<usize>` for `println!("TALOS: println count {}", count)`.
- Hardware result: Run `rpi5-one-placeholder-println-20260521T2049Z` staged the 100,536-byte `1d0fc95bbcaf13b1c163083158d06fd69a546f1042c676dd2133e800ad2323fe` diagnostic image under root and `da591740/` kernel paths. TFTP served `da591740/kernel_2712.img` twice. Serial observe captured 41 `TALOS: println phase` / `TALOS: println count N` pairs, with counts 90 through 130 in the accepted bounded window.
- Rationale: This restores a useful Daedalus-like call surface for the common one-value early diagnostic case without depending on unresolved formatter internals. The broader formatter remains a separate task once the early runtime/link contract is less fragile.
- Risks: This is a constrained early bring-up path. It does not support multiple arguments, width/debug/hex format specifiers, arbitrary `Display` implementations, panic message formatting, or full generic `core::fmt` parity.

## 2026-05-21 - Normal Pi 5 Boot Status Uses One-Placeholder println!

- Status: accepted for one normal Pi 5 boot/status value through the constrained `println!("... {}", value)` path; generic dynamic `core::fmt` remains unaccepted.
- Context: The prior task accepted the one-placeholder `println!` helper in a looping diagnostic, but normal boot/status output still used static `println!` plus explicit formatter-free helper calls for numeric fields.
- Decision: Move one ordinary boot value onto the accepted macro surface with `println!("talos: boot core {}", boot_info.primary_core as usize)`. Keep services and wider numeric fields on the existing formatter-free helpers until broader formatting is validated.
- Local validation: `cargo fmt --check`, `cargo -Zjson-target-spec test`, `scripts/qemu-smoke.sh`, normal Pi 5 image build, `scripts/rpi5-println-phase-diagnostic-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, string inspection, and targeted disassembly inspection passed. Disassembly showed `kernel_main` calling `talos::target::console::_print_one::<usize>` for the new line.
- Hardware result: Run `rpi5-normal-one-placeholder-bootcore-rerun-20260521T211700Z` left the 100,912-byte `7e7ef44f87049550290d8a149262cf2b0d89a708b6b961f0ed0f87da21e2ff36` normal image staged under root and `da591740/`. TFTP logs since publish show four served `da591740/kernel_2712.img` events for that image size. Serial peek after the rerun captured `Starting OS`, BL31, `TALOS: rust_entry`, `TALOS: kernel_main`, `talos: boot core 0`, readable boot-info/services fields, and `talos: status early boot log ready`.
- Rationale: This proves the constrained one-placeholder surface is usable in normal Pi 5 kernel-facing boot logs, not only in a dedicated diagnostic loop.
- Risks: Capture for the accepted line came from retained serial peek after the rerun, not the first bounded observe window; the TFTP and serial evidence are still current-run-correlated by the staged image hash/size and timestamps. Multiple placeholders, format specifiers, arbitrary `Display`, panic messages, and generic dynamic `core::fmt` remain unaccepted.

## 2026-05-21 - Pi 5 One-Placeholder println! Supports Early Hex

- Status: accepted for one normal Pi 5 boot/status address through the constrained one-placeholder `println!("... {}", value)` path using a Talos-owned early hex wrapper; generic dynamic `core::fmt` and `{:x}` format specifiers remain unaccepted.
- Context: Decimal one-placeholder output was accepted in both a diagnostic loop and one normal boot/status line. Normal boot still needed address-shaped values without returning to generic formatter internals.
- Decision: Add a Pi 5-only `target::console::hex(value)` wrapper that implements `EarlyFormatArg` by calling the accepted arithmetic `write_early_hex_u64` path. Move one normal address line onto the macro surface with `println!("talos: boot dtb {}", target::console::hex(boot_info.dtb_pa))`.
- Local validation: `cargo fmt --check`, `cargo -Zjson-target-spec test`, `scripts/qemu-smoke.sh`, normal Pi 5 image build, `scripts/rpi5-println-phase-diagnostic-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, string inspection, and targeted disassembly inspection passed. Disassembly showed `kernel_main` calling `talos::target::console::_print_one::<talos::target::console::Hex>`, and `Hex::write_to` calls `talos::target::rpi5::write_early_hex_u64` with no `write_fmt` call in that path.
- Hardware result: Run `rpi5-normal-one-placeholder-hex-20260521T2136Z` staged the 100,936-byte `6696a949854ae8444c4adddc0391df7d06b62749fb49a49de5eaf82ef1ffb32a` normal image under root and `da591740/` kernel paths. TFTP served `da591740/kernel_2712.img` twice for that image size. The bounded serial observe captured `TALOS: rust_entry`, `TALOS: kernel_main`, `talos: boot core 0`, `talos: boot dtb 0x2efec600`, readable boot-info/services fields, and `talos: status early boot log ready`.
- Rationale: This keeps the public kernel-facing macro shape moving toward Daedalus-like use while preserving the hard-won Pi 5 early-runtime constraints: literal one-placeholder calls only, owned early decimal/hex writers, and no dependency on generic formatter internals.
- Risks: This does not support `{:x}`, multiple placeholders, arbitrary `Display`, full panic source formatting, exception return/resume, or generic dynamic `core::fmt`.

## 2026-05-21 - Pi 5 One-Placeholder println! Supports a Static String

- Status: accepted for one normal Pi 5 boot/status static string through the constrained one-placeholder `println!("... {}", value)` path; generic dynamic `core::fmt` remains unaccepted.
- Context: Decimal and early-hex one-placeholder output were already accepted in normal Pi 5 boot/status lines. The next narrow macro-surface question was whether a static string argument could pass through the same helper while respecting the Pi 5 rodata relocation constraint.
- Decision: Move one normal target-name line onto the constrained macro surface with `println!("talos: boot target {}", boot_info.target.name())`. Keep the wider boot-info/services lines on formatter-free helpers until multiple placeholders and richer formatting are validated.
- Local validation: `cargo fmt --check`, `cargo -Zjson-target-spec test`, `scripts/qemu-smoke.sh`, normal Pi 5 image build, `scripts/rpi5-println-phase-diagnostic-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, string inspection, and targeted disassembly inspection passed. Disassembly showed `kernel_main` calling `talos::target::console::_print_one::<&str>`, and the string argument flows through `Rpi5EarlyConsole::write_str`, which relocates linked rodata strings before writing.
- Hardware result: Run `rpi5-normal-one-placeholder-string-20260521T2148Z` staged the 100,952-byte `230f6f566324aedd8f5eaba73740d9cb9d60d85052c567b811e030b5e3bebe2d` normal image under root and `da591740/` kernel paths. TFTP served `da591740/kernel_2712.img` twice for that image size. The bounded serial observe captured normal phase lines, `talos: boot core 0`, `talos: boot dtb 0x2efec600`, `talos: boot target talos-rpi5-bcm2712`, readable boot-info/services fields, and `talos: status early boot log ready`.
- Rationale: This proves the constrained one-placeholder path can carry the common static-name case without returning to generic formatter internals. It is a practical step toward Daedalus-like kernel logging while preserving the accepted Pi 5 early console guardrails.
- Risks: This does not support multiple placeholders, format specifiers, arbitrary `Display`, panic source/message formatting, exception return/resume, or generic dynamic `core::fmt`.

## 2026-05-21 - Pi 5 Pointer Contract Restores Generic Formatting

- Status: accepted for normal Pi 5 boot/status logging through generic dynamic `core::fmt` after correcting the link/runtime address contract.
- Context: Matthew correctly challenged the idea that formatter failures should be explained by the serial backend. Daedalus uses ordinary `format_args!` and `core::fmt::Write::write_fmt` successfully because its linked/static addresses match the runtime image placement. Talos was linked at `0x80000` while the Pi 5 firmware-selected Image path was executing around `0x200000`, leaving static strings, format templates, and formatter function pointers at the wrong linked addresses unless ad hoc relocation helpers intercepted a narrow path.
- Decision: Link the Pi 5 kernel at `0x00200000` while keeping the arm64 Image header `text_offset=0`. Remove the Pi-only one-placeholder `_print_one` macro bypass and restore the public `print!`/`println!` macros to the standard `format_args!` route for Pi 5 as well as QEMU. Add a normal boot verification line, `talos: pointer delta {:#x}`, computed from the runtime-vs-linked code-label delta.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test`, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, and `git diff --check` passed. Symbol/header inspection showed `_start=0x200000`, `__exception_vectors=0x200800`, `__rodata_start=0x210000`, Image `text_offset=0`, header/file size `79256`, flags `12`, and magic `ARMd`. Disassembly/string inspection showed no `_print_one`/ `EarlyFormatArg` symbols and showed normal `kernel_main` calls through `core::fmt::Arguments::new`, `new_display`, `new_lower_hex`, and `target::console::_print`.
- Hardware result: Run `rpi5-pointer-contract-standard-fmt-20260521T215626Z` staged archive `7c52570da25283ba5f0ff937fb3e57fc6731d3296081b1279374460b74a2c8a4` with the 79,256-byte kernel `06afe837ab8627cce4365c54fd443937b127f23d7cd9ab0f87bbbb9644ec6894` under root and `da591740/` paths. TFTP logs captured 13 fresh events, including two 79,256-byte `da591740/kernel_2712.img` serves. Bounded serial observe captured normal phase lines, `talos: boot core 0`, `talos: boot dtb 0x2efec600`, `talos: boot target talos-rpi5-bcm2712`, `talos: pointer delta 0x0`, readable boot-info/services fields, and `talos: status early boot log ready`.
- Rationale: `fmt` was not special; it needed valid pointers. Correcting the Pi 5 load/link contract makes standard Rust formatting work without growing the formatter shim.
- Risks: A narrow linked-address adjustment helper remains for vector installation and diagnostics, though it is a no-op when the observed delta is zero. Panic `PanicInfo` formatting, source locations, exception return/resume, and broader runtime facilities remain separate validation tasks.

## 2026-05-21 - Pi 5 Normal Console Drops Static-String Relocation Wrapper

- Status: accepted by local inspection and build gates; hardware rerun not required because the accepted pointer-contract run already proved `pointer delta 0x0` for the normal image.
- Context: The pointer-contract fix restored generic `format_args!` output by linking the Pi 5 Image at its firmware-selected runtime base. After that, the normal console's `Rpi5EarlyConsole` string relocation wrapper was obsolete defensive scaffolding from the earlier linked-at-`0x80000` phase.
- Decision: Remove the Pi 5 normal-console static-string relocation wrapper and rely directly on the firmware-preserved console writer. Keep `relocate_early_linked_addr` only for exception-vector installation and explicit address-contract diagnostics while early bring-up is still active.
- Rationale: The normal serial path should reflect the accepted architecture: valid static pointers, ordinary `core::fmt`, and no hidden string rewrite layer. Keeping only the narrow linked-address helper preserves useful diagnostics without reviving the old formatter workaround.

## 2026-05-21 - Pi 5 Normal Boot Status Uses Multi-Field println!

- Status: accepted with local formatter/disassembly gates and Pi 5 serial hardware evidence.
- Context: After the pointer-contract fix, early boot already used ordinary `println!` for static strings, one decimal field, one hex field, one string field, and the pointer-delta check. The aggregate boot-info and services lines still used formatter-free helper composition from the earlier bring-up phase.
- Decision: Replace those remaining normal Pi 5 helper-composed status lines with ordinary `println!` calls: `talos: boot info: dtb={:#x} core={} el={} target={}` and `talos: services: uart={} timer={} irq={} mmio_regions={} dtb={:#x}`, while preserving the `dtb=none` branch if no DTB physical address is available.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test`, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, and `git diff --check` passed. The normal Pi 5 image is 79,320 bytes with sha256 `bdfa8d8986e3902b4e09c3de4ca0f282a5e2eaa950d835d1cd36ffd16fbd0bb8`. String/symbol/disassembly inspection showed the expected status templates, no `_print_one`/`EarlyFormatArg`/`Rpi5EarlyConsole`/static-string relocation wrapper symbols, and `kernel_main` constructing the new lines through `core::fmt::Arguments::new::<55, 4>`, `new::<70, 5>`, and `new::<68, 4>`.
- Hardware result: Run `rpi5-normal-multifield-println-20260521T2232Z` published tree hash `c6123b8131f6544f5a9dd19ed97a43ef84f9a1d2066ad5fb1144955f6769e20b`; TFTP served `da591740/kernel_2712.img` at 79,320 bytes, and serial captured normal phase lines plus `talos: pointer delta 0x0`, `talos: boot info: dtb=0x2efec600 core=0 el=2 target=talos-rpi5-bcm2712`, `talos: services: uart=firmware-preserved timer=arm-generic irq=gic-v2 mmio_regions=7 dtb=0x2efec600`, and `talos: status early boot log ready`.
- Rationale: The standard `print!`/`println!` surface is now carrying the normal multi-field Pi 5 status output, matching the Daedalus-style API and reducing the remaining formatter-free code to exception/panic diagnostics and tiny early helpers.

## 2026-05-21 - Normal Pi 5 BRK Exception Report Uses println!

- Status: accepted with local formatter/disassembly gates and Pi 5 serial hardware evidence.
- Context: The normal Rust BRK fatal report was previously accepted through formatter-free helper output. After the Pi 5 pointer contract and normal multi-field `println!` output were accepted, the next remaining exception-report gap was whether the normal BRK preamble and Rust exception handler report could use the same Daedalus-like `println!` surface.
- Decision: Move the normal-exception diagnostic preamble and the normal `rust_exception_handler` report to ordinary `println!` calls: `TALOS: before normal BRK vbar={:#x} el={}`, `talos exception: {}`, and `exception-info: esr={:#018x} elr={:#018x} far={:#018x}`. Keep the diagnostic assembly exception-report path separate as a guardrail and halt after the normal exception report in the diagnostic image so the serial output remains capturable.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test`, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-normal-exception-report-diagnostic-image.sh`, `scripts/rpi5-format-guard-check.sh`, and `git diff --check` passed. Targeted string/symbol/disassembly inspection showed the expected exception templates, no obsolete normal marker/vector-name writer symbols, the BRK preamble using `core::fmt::Arguments::new::<45, 2>`, and the handler using `println!`/`core::fmt::Arguments::new::<22, 1>` plus `new::<57, 3>` with lower-hex arguments for ESR/ELR/FAR.
- Hardware result: Run `rpi5-normal-exception-println-halt-pickup-20260521T2256Z` used the staged 79,536-byte `9900e29a441cb2b96246a237a2df0b084f7aa8e8630b8ea61a01c707d9bfec29` halt-after-report diagnostic image. TFTP served `da591740/kernel_2712.img` at 79,536 bytes at 22:56:31 and again at 22:57:03. Retained serial from that pickup captured normal boot/status output followed by `TALOS: before normal BRK vbar=0x200800 el=2`, `talos exception: current-spx-sync`, and `exception-info: esr=0x00000000f2000000 elr=0x000000000020420c far=0x9abbb7ffffd7dfef`.
- Rationale: The normal fatal BRK path now uses the same standard `print!`/`println!` formatting surface as ordinary Pi 5 boot logs. That removes the formatter-free exception-report special case from the steady-state path while preserving the lower-level assembly diagnostic for future exception bring-up.
- Risks: This remains a fatal halt diagnostic. It does not validate exception return/resume, IRQ/FIQ/SError dispatch, nested exception behavior, or full `PanicInfo` formatting.

## 2026-05-21 - Pi 5 Panic Fatal Line Uses println!

- Status: accepted for a bounded Pi 5 panic-handler fatal line with runtime EL/VBAR fields through ordinary `println!`; full `PanicInfo` formatting remains unaccepted.
- Context: Panic output previously used formatter-free static/decimal/hex helpers. After normal boot/status and normal BRK exception reports were accepted through the standard print surface, the next panic question was whether the panic handler itself could use that same path.
- Decision: Keep a tiny static panic-handler entry marker for classification, then emit `talos panic: el={} vbar={:#x}` through ordinary `println!` and halt after draining UART. Do not display the full `PanicInfo` yet.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test`, `scripts/qemu-smoke.sh`, `scripts/rpi5-panic-report-diagnostic-image.sh`, `scripts/rpi5-format-guard-check.sh`, and `git diff --check` passed. Targeted strings and disassembly showed `TALOS: panic handler entered`, `talos panic: el=`, and `rust_begin_unwind` constructing the EL/VBAR line with `core::fmt::Arguments::new::<33, 2>` before draining UART and halting.
- Hardware result: Run `rpi5-panic-println-bounded-diag-20260521T2324Z` staged archive `b6c3d78f7ec9035ce16ec9cb78a468da975ced9e5e7232a69692826084287b94` with the 75,256-byte kernel `0ca770312cf1490df781bfe7f0eff3b36a7830a6dc5afef97a1a02a4ae854525`. TFTP served `da591740/kernel_2712.img` twice, and bounded serial observe captured normal boot/status output followed by `TALOS: panic handler entered` and `talos panic: el=2 vbar=0x200800`.
- Rationale: This moves the stable panic fatal report onto the Daedalus-like print surface without reintroducing the older source-location instability. A full `println!("talos panic: {}", info)` variant was also built and served at 83,600 bytes, but hardware did not capture panic output from that variant, so `PanicInfo` display remains a later, separate task.
- Risks: This does not validate panic message formatting, source location display, nested panic behavior, exception return/resume, or non-sync exception paths.

## 2026-05-22 - Pi 5 Full PanicInfo Display Uses println!

- Status: accepted for full `PanicInfo` display through ordinary `println!` in the Pi 5 panic handler after normal boot/status output.
- Context: The bounded panic fatal line had already proved that the panic handler can use the standard print surface for EL/VBAR fields. The next narrow question was whether the richer `PanicInfo` `Display` implementation itself can run on the accepted Pi 5 link/runtime and console path.
- Decision: Keep the tiny static `TALOS: panic handler entered` marker for classification, then allow the diagnostic panic path to emit `println!("talos panic: {}", info)` before draining UART and halting.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test`, `scripts/qemu-smoke.sh`, normal Pi 5 image build, `scripts/rpi5-full-panic-info-diagnostic-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, strings inspection, archive review, and targeted disassembly inspection passed. The diagnostic image is 79,576 bytes with kernel sha256 `90ca0a62a7855b5ecbc81dccd434f01a2e807ec4a79441bcb335625b763cc53b`.
- Hardware result: Restaged archive `26929816dabe2d8d295e677abeec70bb7eebd0bf9f86eb2c1e33449fe0da59b3` as tree `a6ecdf3805fd96a48e95585c96b6159820a2f6cd3a9abb0ac0b7149cbe2bf7ad`. TFTP served the 79,576-byte `da591740/kernel_2712.img` twice. Serial captured normal boot/status output, `TALOS: panic handler entered`, and `talos panic: panicked at src/main.rs:433:9:` followed by `talos diagnostic panic`.
- Rationale: This closes the immediate panic-output gap left after the bounded EL/VBAR line. The early Pi 5 panic path can now report the panic source location and message through the same Daedalus-like `println!` surface as normal boot logs and normal BRK exception reports.
- Risks: This is a diagnostic-triggered panic path, not a complete panic policy. Nested panics, arbitrary panic sites, non-sync exception paths, and exception recovery policy remain separate validation work.

## 2026-05-22 - Default Pi 5 Panic Report Uses Full PanicInfo

- Status: accepted with local formatter/disassembly gates and Pi 5 serial hardware evidence.
- Context: The previous full `PanicInfo` run proved `println!("talos panic: {}", info)` in a diagnostic-only panic-handler branch. The stable Pi 5 panic path still used the older bounded EL/VBAR line unless `TALOS_RPI5_FULL_PANIC_INFO_DIAGNOSTIC` was set.
- Decision: Make the Pi 5 panic handler's default report emit the full `PanicInfo` display through ordinary `println!` after the tiny static `TALOS: panic handler entered` marker, then drain UART and halt. Keep the normal panic-report diagnostic trigger so this policy can be tested without a separate full-PanicInfo build flag.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test`, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-panic-report-diagnostic-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, strings/symbol inspection, archive review, and targeted disassembly inspection passed. The normal panic-report diagnostic image is 79,576 bytes with sha256 `90ca0a62a7855b5ecbc81dccd434f01a2e807ec4a79441bcb335625b763cc53b`; disassembly shows `rust_begin_unwind` writing the static marker, building a `Display` argument for `&PanicInfo`, calling `target::console::_print`, draining UART10, and halting.
- Hardware result: Run `rpi5-default-panic-info-20260522T031900Z` staged archive `c0e13b49eea6fcbda2ecb5f8ac1e2b351c96cab50dafe35a53cf2fb6489a6c12` as tree `a6ecdf3805fd96a48e95585c96b6159820a2f6cd3a9abb0ac0b7149cbe2bf7ad`. After delayed network-boot pickup, TFTP served the 79,576-byte `da591740/kernel_2712.img` twice and serial captured normal boot/status output, `TALOS: panic handler entered`, `talos panic: panicked at src/main.rs:433:9:`, and `talos diagnostic panic`.
- Rationale: Panic reporting now follows the same Daedalus-like print surface by default as normal boot logs and normal BRK exception reports. The separate full-PanicInfo diagnostic flag is no longer needed to get source-location and message output from the Pi 5 panic handler.
- Risks: This does not validate nested panics, non-sync exception paths, or a complete exception/panic recovery policy.

## 2026-05-22 - Current Pi 5 Entry Reaches Stack Before Rust Handoff

- Status: accepted for narrow transition diagnostics through CPACR enable, BSS clearing, and stack setup; exception return/resume remains unaccepted.
- Context: While validating BRK exception return/resume, current normal and Rust-stage images TFTP-served correctly but stopped producing fresh Talos Rust-stage serial. A current-source entry-line diagnostic still printed, so the next question was which post-entry boundary failed before `rust_entry`.
- Decision: Isolate the transition with reset diagnostics that print bounded UART10 markers before and after one boundary at a time. Keep exception-return code unchanged until the handoff into Rust is proven again.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test`, `scripts/qemu-smoke.sh`, the new diagnostic image builds, `scripts/rpi5-format-guard-check.sh`, and `git diff --check` passed. String inspection confirmed the expected marker literals; CPACR disassembly showed `msr CPACR_EL1`, `isb`, the after-CPACR marker loop, and PSCI reset.
- Hardware result: `rpi5-phase-cpacr-reset-20260522T0055Z` served the 75,200-byte `7f57af9456979ace1fb3608c4a994ebd0d5b5b58114d8170f20fc2545281114e` kernel and captured BL31 plus 16 `TALOS: before CPACR` and 64 `TALOS: after CPACR` lines. `rpi5-phase-bss-reset-20260522T0058Z` served the 75,200-byte `171f72e301391ab07a2b64da093b8cab3e1508fe0312949c942b7a716c839ee9` kernel and captured `TALOS: after BSS`. `rpi5-phase-stack-reset-20260522T0105Z` served the 75,200-byte `56e710795ed0d7d471199580248c5da90bc42492800f0d28dae172eedaedb398` kernel and captured repeated `TALOS: after stack` lines.
- Rationale: The current no-Rust-output boundary is no longer TFTP, serial capture, Image placement, CPACR, BSS clearing, or stack setup. The next bounded diagnostic should test the branch/call into a tiny Rust or assembly veneer before returning to the BRK exception-return image.

## 2026-05-22 - Post-Stack Branch To .text Is Not Yet Accepted

- Status: superseded by the later branch-marker rerun and exception-return acceptance below.
- Context: The stack-to-Rust probe stopped before a Rust handoff marker or reset side effect. The next distinction was whether the post-stack path could branch into a tiny assembly veneer in the normal .text region before testing any Rust prologue.
- Decision: Keep the exception-return implementation unchanged and continue isolating the handoff boundary. The current evidence points at the post-stack branch into .text or immediate execution there, before any useful Rust prologue evidence.
- Local validation: cargo fmt --all -- --check, cargo -Zjson-target-spec test, scripts/qemu-smoke.sh, scripts/rpi5-phase-stack-to-text-reset-diagnostic-image.sh, scripts/rpi5-phase-stack-reset-diagnostic-image.sh, scripts/rpi5-phase-stack-to-rust-reset-diagnostic-image.sh, scripts/rpi5-format-guard-check.sh, git diff --check, archive review, and targeted disassembly inspection passed across the two stack-to-text variants. The self-contained .text variant is 75,200 bytes with kernel sha256 b55320989ba1376808bebcf125353280e7bb58afe87561a60ff023048aae16cf; _start branches from the post-stack point at 0x200120 to rpi5_stack_to_text_reset_probe at 0x20e940.
- Hardware result: The helper-call .text variant rpi5-phase-stack-to-text-reset-20260522T0146Z served the 75,200-byte 801e7a6303891ac97b220a9eb3548d0f560750dca6136d3021a592377a7713b5 kernel twice and captured current boundary output through TALOS: after stack, but not TALOS: stack to text or a reset-induced second boot. The self-contained .text variant rpi5-phase-stack-to-text-self-reset-20260522T0156Z staged archive 5f0c9a011e01e0c6938ae04776aa1f9bd6c599edf159f6c73f3ed590c91a5063, served the 75,200-byte b55320989ba1376808bebcf125353280e7bb58afe87561a60ff023048aae16cf kernel twice, and likewise captured TALOS: after stack with no TALOS: stack to text marker.
- Rationale: Removing helper calls from the .text target did not change the hardware result, so the next useful probe is exception/hang classification at the post-stack branch itself, not another Rust formatter or prologue experiment.

## 2026-05-22 - Post-Stack Handoff Diagnostics Cleared For Exception-Return Rerun

- Status: accepted as supporting transition evidence for the active BRK exception-return task.
- Context: The previous stack-to-text captures stopped after repeated `TALOS: after stack` output, but the diagnostic repeated that marker heavily before the branch. That made it ambiguous whether hardware had reached the actual branch into `.text`.
- Decision: Shorten the stack-to-text diagnostic's after-stack loop, emit `TALOS: before text branch` immediately before the `.text` call, and remove the Rust handoff probe's accidental dependency on unoptimized range-iterator code by replacing its first-output path with inline assembly bytes.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test`, `scripts/qemu-smoke.sh`, `scripts/rpi5-phase-stack-to-text-reset-diagnostic-image.sh`, `scripts/rpi5-phase-stack-to-rust-reset-diagnostic-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, strings inspection, archive review, and targeted disassembly inspection passed. The stack-to-text branch-marker image is 75,200 bytes with sha256 `45a9486667edc40807d9b2c710d5816ce7ba8c4abc72e7b843617377e0883302`; the Rust-symbol inline-asm handoff image is 75,200 bytes with sha256 `e749b60ed1558739ba668d196fe034e222c7f8f3545ca779854a81c3dd29391b`.
- Hardware result: `rpi5-phase-stack-to-text-branch-marker-20260522T021536Z` staged archive `d2c8800999fa9dcf7a263ebf9fc32278563062910c5bf91b2f78e1f096917fc4`; TFTP logs from cursor 3125821 captured 75,200-byte `da591740/kernel_2712.img` serves at 02:16:00/01 and reset-induced serves at 02:16:38. Serial captured `TALOS: before text branch`, sixteen `TALOS: stack to text` lines, and a firmware reboot. `rpi5-phase-stack-to-rust-inline-asm-20260522T021949Z` staged archive `b9151057f5ba6e34c6708487d3884a366d864ad4a31408e6156f5d520d10957c`; TFTP logs from cursor 3135278 captured 75,200-byte kernel serves at 02:20:13 and reset-induced serves at 02:20:58/59. Serial captured boundary output through stack setup, `TALOS: rust handoff`, and a firmware reboot.
- Rationale: The post-stack path can branch into `.text`, execute a tiny target there, call a Rust-exported symbol whose first instructions are inline assembly, and request reset. The prior Rust handoff failure was not evidence against the branch or Rust symbol boundary; it was polluted by the diagnostic's unoptimized iterator setup before first output.

## 2026-05-22 - Normal Pi 5 BRK Exception Returns With ERET

- Status: accepted with local image/disassembly gates and Pi 5 serial hardware evidence for the narrow same-EL synchronous BRK diagnostic.
- Context: The normal BRK report through `println!` was accepted, but the next question was whether the handler could return an advanced ELR, the vector shim could write it back to the active ELR bank, and `ERET` could resume kernel code after the BRK.
- Decision: Keep this as a diagnostic-only normal BRK return path. The Rust handler emits the normal report and a `talos exception: resume elr=...` line, returns the advanced ELR, and the vector shim writes that value to `ELR_EL1`, `ELR_EL2`, or `ELR_EL3` based on `CurrentEL` before executing `ERET`.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test`, `scripts/qemu-smoke.sh`, `scripts/rpi5-exception-return-diagnostic-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, strings inspection, archive review, and targeted disassembly inspection passed. The accepted image is 75,344 bytes with sha256 `b94aa5b87268f5d46f4df0b6cab7383d3c3225550fa0024a5602529f6b4c3f2e`; disassembly shows `__exception_entry` calling `rust_exception_handler`, writing returned `x0` to the current `ELR_ELx`, and executing `eret`.
- Hardware result: `rpi5-normal-brk-exception-return-rerun-20260522T022222Z` staged archive `448a858f16fe785266aae3ca844f14f23fb3a2489c924281ab5a950b800c6c9a` and tree hash `4ae0209c85a1d77beed412df97626778c3c84c07437a20d471e9820c6c1daaf5`. TFTP logs from cursor 3139331 captured four 75,344-byte `da591740/kernel_2712.img` serves across the initial and repower pickups. Serial captured normal boot/status tail, `TALOS: before normal BRK vbar=0x200800 el=2`, `talos exception: current-spx-sync`, `exception-info: esr=0x00000000f2000000 elr=0x0000000000203824 far=0x1a9bbff767d79fef`, `talos exception: resume elr=0x0000000000203828`, and `TALOS: after normal BRK resume`.
- Rationale: This accepts the minimal report-and-resume control path needed before richer panic/exception policy work. It does not turn arbitrary exceptions into recoverable events; it proves the vector shim and Rust handler can cooperate for one deliberate same-EL synchronous BRK resume.
- Risks: General exception recovery policy, IRQ/FIQ/SError dispatch, lower-EL vectors, nested exceptions, and full `PanicInfo` display remain unaccepted.

## 2026-05-22 - Pi 5 BRK Exception Return Preserves Interrupted Registers

- Status: accepted with local register-save disassembly gates, a lab control pickup, and Pi 5 serial hardware evidence from the phase-enabled diagnostic image.
- Context: The first accepted `ERET` diagnostic proved that the Rust handler could return an advanced ELR and resume after a deliberate same-EL BRK. The initial return shim did not preserve the interrupted general-purpose register file around the Rust handler call, so resumed code could not treat the path as a trustworthy diagnostic continuation.
- Decision: For the Pi 5 exception-return diagnostic, make each vector slot save interrupted `x0`/`x1` plus the vector kind, then branch to `__exception_entry_return`. That shim saves/restores `x0..x30`, calls `rust_exception_handler`, writes the returned ELR to the active `ELR_ELx`, restores the interrupted registers, and executes `ERET`. The diagnostic probe sets sentinel values in caller-saved `x9` and callee-saved `x19`, executes BRK, and prints success only if both sentinels survive.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test`, `scripts/qemu-smoke.sh`, `scripts/rpi5-exception-return-diagnostic-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, strings inspection, and targeted disassembly inspection passed. The base register-preservation image is 83,984 bytes with sha256 `52acfff4edb4b450d9d39851f63515816055d90af518ecf67152bb70af96dcd9`; disassembly shows `rpi5_brk_register_preserve_probe` setting `x9`/`x19` sentinels and `__exception_entry_return` saving/restoring the GP register context before `ERET`.
- Hardware result: A control run first republished the accepted nested-panic archive `7ed6dedb2ceb10be04d939c3777fbc6ba55ceb3585fc0adc90f2e7d2bb14e0b5` and captured current Talos serial output, proving the lab serial/TFTP path was healthy after earlier premature no-output observations. The accepted register-preservation pickup `rpi5-exception-return-phase-rerun-20260522T054616Z` then published archive `f67d0eb860961887728e2f68ef043b29fc249455d30c9ac4554f12c8d288ac32` as tree `f52674fa998d83a6e3173e4409809351ff7bcb77474ba0de674801ccd2caac77`; TFTP served the 83,984-byte `da591740/kernel_2712.img` twice. Serial captured the phase markers, normal boot/status output, `TALOS: before normal BRK vbar=0x200800 el=2`, `talos exception: resume elr=0x000000000020ff50`, `TALOS: after normal BRK resume x9=0x1122334455667788 x19=0x8877665544332211`, and `TALOS: exception registers preserved`.
- Rationale: This makes the diagnostic BRK return path useful for continued bring-up probes because the interrupted register context is no longer intentionally sacrificed to call the Rust handler.
- Risks: This still does not define general exception recovery, lower-EL recovery, IRQ/FIQ/SError dispatch, or production resume policy. The accepted hardware evidence is from the phase-enabled diagnostic image, which carries extra entry markers but exercises the same register-preserving return shim and sentinel probe.

## 2026-05-22 - Pi 5 BRK Exception Report Includes SPSR And Saved Sentinels

- Status: accepted with local image/disassembly gates and Pi 5 serial hardware evidence.
- Context: The register-preserving BRK return diagnostic proved that interrupted `x9` and `x19` survive the Rust exception handler and `ERET`, but the exception report itself still only printed ESR, ELR, and FAR. The Phase 2 exception milestone calls for enough state to debug early faults, including status and register context.
- Decision: Extend the return-capable diagnostic shim to read the active `SPSR_ELx` bank and pass the saved interrupted `x9` and `x19` values directly to Rust in ABI registers before restoring the full `x0..x30` frame. Emit the new `exception-status` and `exception-regs` lines with the formatter-free console helpers inside the diagnostic report while leaving the normal exception identity/info/resume lines on the accepted `println!` path.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test`, `scripts/qemu-smoke.sh`, phase-enabled Pi 5 exception-return image build, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, strings inspection, archive review, and targeted disassembly inspection passed. The accepted diagnostic image is 84,080 bytes with sha256 `44ea7e494c86e48a8bc60d8b8ea5bb98910ceebb1ff36ae941d432461406ef54`; disassembly shows `__exception_entry_return` reading `SPSR_EL1/2/3`, loading saved `x9`/`x19` from the preserved frame, calling `rust_exception_handler`, then restoring `x0..x30` before `ERET`.
- Hardware result: Run `rpi5-exception-context-direct-20260522T0634Z` staged archive `41bdfa2ca40401ddb68199ffa0db5fb86bebc40e5274634e24067a5933182e46` as tree `34f22bfbea8756a270f340d5bfbda2f15e67646bc38400a656517b52de0d80d8`. TFTP served the 84,080-byte `da591740/kernel_2712.img` twice. Serial captured normal boot/status output, `TALOS: before normal BRK vbar=0x200800 el=2`, the normal BRK exception report, `exception-status: spsr=0x200003c9`, `exception-regs: x9=0x1122334455667788 x19=0x8877665544332211`, `talos exception: resume elr=0x000000000021024c`, after-resume `x9`/`x19` sentinel values, and `TALOS: exception registers preserved`.
- Rationale: Passing the two saved sentinel registers directly avoids exposing a raw frame pointer to Rust while still proving that the report is reading interrupted register context from the same saved frame that is later restored for `ERET`.
- Risks: This remains a deliberate same-EL BRK diagnostic. It does not validate a full register dump, arbitrary synchronous faults, lower-EL recovery, IRQ/FIQ/SError dispatch, or production exception recovery policy.

## 2026-05-22 - Pi 5 Nested Panic Emits Static Marker And Halts

- Status: accepted with local formatter/disassembly gates and Pi 5 serial hardware evidence.
- Context: The Pi 5 panic handler can print full `PanicInfo`, but a panic while reporting a panic needed its own bounded path that does not recurse through formatting. The first guard used `AtomicBool::compare_exchange`; hardware converted the exclusive-byte access in `core::sync::atomic` into a synchronous exception at `ldaxrb`, after `TALOS: panic handler entered` and before the nested marker.
- Decision: Use a tiny volatile single-core panic-in-progress guard for the current Pi 5 early panic path. The first entry writes the guard and continues to `println!("talos panic: {}", info)`. A guarded re-entry writes only `TALOS: nested panic`, drains UART10, and halts.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test`, `scripts/qemu-smoke.sh`, normal Pi 5 image, panic-report diagnostic image, nested-panic diagnostic image, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, archive review, strings inspection, and targeted disassembly inspection passed. The nested-panic diagnostic image is 79,752 bytes with sha256 `55290b303cce34390023cbd49c8bc5a843e3b757b352b9de1297ccc039a091d7`; `rust_begin_unwind` calls `PanicInProgress::enter`, and the checked disassembly contains volatile guard access with no `ldaxr`/`ldaxrb`/`stlxr`/`stlxrb` exclusive atomics in the panic handler.
- Hardware result: Run `rpi5-nested-panic-volatile-20260522T050507Z` published archive `7ed6dedb2ceb10be04d939c3777fbc6ba55ceb3585fc0adc90f2e7d2bb14e0b5` as tree `5dde903b272d75176121bff75bedc16d1b8a2b26cd5fe4909c1a538c82cab058` with matching 79,752-byte root and `da591740/` kernels. The TFTP delta endpoint returned no new entries for this pickup, but the lab publish response and retained serial after the power cycle correlate the staged size/tree with current boot output. Serial captured normal boot/status output, `TALOS: nested panic diagnostic prearm`, `TALOS: nested panic diagnostic trigger`, `TALOS: panic handler entered`, and `TALOS: nested panic`.
- Rationale: The nested-panic branch now avoids formatter recursion and avoids early-MMU atomic exclusive instructions that fault on this hardware configuration. The guard is deliberately narrow and appropriate for current single-core early bring-up; a later SMP/runtime memory-model pass should revisit panic ownership before enabling multicore panic coordination.
- Risks: This does not define a complete panic policy, backtrace reporting, multi-core panic coordination, or recovery from nested panic.

## 2026-05-22 - Pi 5 BRK Exception Report Dumps Saved GPR Frame

- Status: accepted with local image/disassembly gates, lab-controller publish/status evidence, and Pi 5 serial hardware evidence.
- Context: The previous BRK return diagnostic reported `SPSR_ELx` plus saved `x9` and `x19` sentinels. Phase 2.3 needs enough exception state to debug early faults, so the next narrow step was a readable saved general-purpose register dump without changing the restore/ERET contract.
- Decision: For the Pi 5 exception-return diagnostic, pass the saved interrupted register-frame base from `__exception_entry_return` to Rust as a read-only `ExceptionFrame` view. The Rust report emits `exception-regs0` through `exception-regs7` before returning the advanced ELR; the assembly shim still restores `x0..x30` from the same frame before `ERET`.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test`, `scripts/qemu-smoke.sh`, `scripts/rpi5-exception-return-diagnostic-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, strings inspection, archive review, and targeted disassembly inspection passed. The accepted diagnostic image is 84,488 bytes with sha256 `2ce08f296176c368fba6b8228db5bf597b543ee7751a2595fef05d8e715e1d94`; disassembly shows the shim moving `sp` into `x5` for the Rust call, then restoring `x0..x30` before `ERET`.
- Hardware result: Run `rpi5-exception-full-gpr-20260522T063059Z` published archive `2167e5f1aa4c5c6e17dd6c85863e78d7bde46e57b2fa32a2479c285354b58660` as tree `49f13722428a92c9e1a554b0acff7dc424ca1b16ea7ca417b60bb08829590a01`; lab status confirmed matching 84,488-byte root and `da591740/` kernels. The TFTP delta endpoint returned no fresh events from the selected cursors, but the post-power serial pickup captured the newly added `exception-regs0` through `exception-regs7` lines, including `x9=0x1122334455667788`, `x19=0x8877665544332211`, and `x30=0x204228`, followed by `talos exception: resume elr=0x0000000000210870`, the after-resume sentinel values, and `TALOS: exception registers preserved`.
- Rationale: Passing the frame base keeps the code honest about where the report comes from while avoiding a broader production exception-frame abstraction. The printed lines are diagnostic output from the same saved frame that ERET later restores.
- Risks: This remains a deliberate same-EL BRK diagnostic. It does not validate arbitrary synchronous faults, lower-EL recovery, IRQ/FIQ/SError dispatch, nested exceptions, or a production exception recovery policy.

## 2026-05-22 - Default Pi 5 Fatal Exceptions Dump Saved GPR Frame

- Status: accepted with local image/disassembly gates, lab-controller publish/status/TFTP evidence, and Pi 5 serial hardware evidence.
- Context: The return diagnostic proved the saved-frame report and restore path, but normal fatal Pi 5 exceptions still reported only vector name, ESR, ELR, and FAR. Phase 2.3 calls for enough state to debug early faults without requiring a resume diagnostic build.
- Decision: Make the default non-returning Pi 5 vector path save the interrupted `x0..x30` frame, read `SPSR_ELx`, and pass a read-only `ExceptionFrame` view to the Rust fatal handler. Keep `ERET` and register restoration only in the explicit `TALOS_RPI5_EXCEPTION_RETURN_DIAGNOSTIC` path.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test`, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-normal-exception-report-diagnostic-image.sh`, `scripts/rpi5-exception-return-diagnostic-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, strings inspection, and targeted disassembly inspection passed. The normal image is 84,200 bytes with sha256 `d9838979b53dc8b1282f5da6aed1acc26e2fb8b2d081dcaa71bdaabf5a90c11a`; the normal BRK fatal diagnostic image is 84,248 bytes with sha256 `e2a5f013ecffc8415e6c78d1596ac085a11cf75d186406307fd84df6b35b4258`; the return diagnostic still builds at 84,488 bytes with sha256 `7a4e197cd09ff8b7a13bd59b345c085c32cd67581f453745729c031cd4c3ba4f`. Disassembly shows `__exception_entry_frame` saving the GP frame, reading `SPSR_ELx`, and passing `sp` in `x5`; `__exception_entry_return` still restores the frame and reaches `ERET`.
- Hardware result: Run `rpi5-default-fatal-exception-frame-20260522T064710Z` staged archive `5ac109fd9d91ad95f48725149f147465286c0468c33fb00c850433e86c72a231`. Lab publish/status confirmed matching 84,248-byte root and `da591740/` kernels, and TFTP served `da591740/kernel_2712.img` at 84,248 bytes. Serial captured normal boot/status output, `TALOS: before normal BRK vbar=0x200800 el=2`, the normal BRK report, `exception-status: spsr=0x200003c9`, and full `exception-regs0` through `exception-regs7` lines including `x0`, `x1`, `x9`, `x19`, `x29`, and `x30=0x204134`.
- Rationale: Default fatal exceptions now carry the same saved-register context as the diagnostic return report, which is the useful state for early MMU and driver faults. The risky policy decision, whether an exception can resume, remains isolated to a diagnostic build.
- Risks: This does not validate arbitrary data/instruction abort causes, lower-EL recovery, IRQ/FIQ/SError dispatch, nested exceptions, or production recovery policy.

## 2026-05-22 - Default Pi 5 Fatal Report Covers Undefined Instruction

- Status: accepted with prior local formatter/test/QEMU/image/disassembly gates, a fresh long-window serial control, and Pi 5 serial hardware evidence.
- Context: The default fatal frame report was hardware-proven with BRK, but BRK has a distinct ESR class. Talos needed proof that the saved-frame fatal path is not BRK-specific before relying on it for broader early-fault diagnostics.
- Decision: Add a Pi 5 undefined-instruction diagnostic that reaches normal boot/status output, prints a fresh entry discriminator and pre-trigger line, executes `udf #0`, and lets the default fatal handler print ESR/ELR/FAR, `SPSR_ELx`, and saved `x0..x30` groups before halting.
- Local validation: The diagnostic code previously passed `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test`, `scripts/qemu-smoke.sh`, normal and undefined Pi 5 image builds, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, string inspection, and targeted disassembly showing `UDF` after the pre-trigger line. The accepted rerun only changed the build-supplied fresh label and passed archive review, strings inspection, and diff check.
- Hardware result: Long-window control `rpi5-long-window-fresh-entry-control-20260522T091159Z` first recovered current Talos-origin serial from the known fresh-entry reset image. Then `rpi5-long-window-undefined-20260522T091507Z` served the 84,296-byte `5a6c5b7703ab08b30f8e85771d319ff24b996b657de1b80356444ab3f8d97676` kernel twice from `da591740/kernel_2712.img`; serial captured the fresh undefined entry prefix, normal boot/status output, `TALOS: before undefined instruction`, `exception-info: esr=0x0000000002000000`, `exception-status: spsr=0x200003c9`, and saved-register groups through `exception-regs7`.
- Rationale: ESR `0x02000000` is the captured syndrome for this deliberate `udf #0` trap and is distinct from the BRK syndrome `0xf2000000`, so the default fatal report contract now covers at least one non-BRK same-EL synchronous exception and can be used for near-term early fault reports.
- Risks: This remains a halt-only fatal path. IRQ/FIQ/SError, lower-EL vectors, data-abort policy, nested exception behavior, and recovery/resume outside the explicit BRK diagnostic remain unaccepted.

## 2026-05-22 - Default Pi 5 Fatal Report Includes ESR Class Label

- Status: accepted with local formatter/test/QEMU/image/disassembly gates and Pi 5 serial hardware evidence.
- Context: The default fatal report already printed vector, ESR/ELR/FAR, SPSR, and saved GPR groups. Reading the raw ESR still required manual class decoding while reviewing serial logs.
- Decision: Add a formatter-free `exception-class: ... ec=...` line in the default Pi 5 fatal exception handler. The line derives `ec` from `ESR_ELx[31:26]`, prints a small bounded label for currently useful classes, and preserves the raw EC value for exact review. ERET and recovery policy remain isolated to the explicit BRK return diagnostic.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test`, `scripts/qemu-smoke.sh`, normal Pi 5 image, undefined-instruction diagnostic image, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, strings inspection, and targeted disassembly passed. The accepted undefined diagnostic image is 84,472 bytes with sha256 `404eec08e93b2d43ab5b6903edbf6e237b41304cc09a9c6b5d85a61a57884ccb`.
- Hardware result: Run `rpi5-exception-class-undefined-20260522T0924Z` staged archive `bb23cc4f1b3003e8c2eda8b9efa9a73b05acc0e6b93bb58000e27ea669c6c256`. TFTP served `da591740/kernel_2712.img` twice at 84,472 bytes. Serial captured the fresh `TALOS: class undef 20260522T0924Z` entry label, normal boot/status output, `TALOS: before undefined instruction`, `exception-info: esr=0x0000000002000000`, `exception-class: unknown-or-undefined-instruction ec=0x0`, `exception-status: spsr=0x200003c9`, and saved-register groups through `exception-regs7`.
- Rationale: Review now gets both a human-readable class label and the raw EC value in the serial report, reducing manual decoding while keeping the fatal path simple and halt-only.
- Risks: The label table is intentionally small. IRQ/FIQ/SError policy, lower-EL vectors, data-abort triggering, nested exception behavior, and production recovery remain separate work.

## 2026-05-22 - Default Pi 5 Fatal Report Covers Alignment Data Abort

- Status: accepted with prior local formatter/test/QEMU/image/disassembly gates, a fresh long-window serial control, and Pi 5 serial hardware evidence.
- Context: The default fatal saved-frame report had been hardware-proven for BRK and undefined-instruction traps. Talos also needs confidence that early data aborts produce a readable report with the fault address before page-table/MMU work makes aborts more likely.
- Decision: Add a narrow Pi 5 alignment data-abort diagnostic. It installs the normal exception vectors, enables `SCTLR_ELx.A` at the current exception level, performs an unaligned load through a valid stack-derived pointer, and leaves the default non-returning fatal handler responsible for printing ESR/ELR/FAR, the ESR class label, SPSR, and saved GPR groups. This remains diagnostic-only and does not introduce data-abort recovery or MMU translation faults.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test`, `scripts/qemu-smoke.sh`, normal Pi 5 image build, `scripts/rpi5-data-abort-report-diagnostic-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, strings inspection, and targeted disassembly passed. The accepted diagnostic image is 84,496 bytes with sha256 `6e0573eac9dba36b6fa5573d6d2cc854363bb75105ed494bed723a59620813ce`; the archive is `1eda1d4ba7362d42dde4178dd9c3e037af8ca366ec92489a136871ca5720f6c8`.
- Hardware result: Long-window fresh-entry control `rpi5-data-abort-long-window-fresh-control-20260522T102700Z` first recovered current Talos-origin serial from the unique reset control `TALOS: data ctrl reset 101633Z`, with TFTP serving the 84,376-byte control kernel. The accepted data-abort rerun `rpi5-data-abort-long-window-rerun-20260522T102820Z` then published tree `7423149ada6d0cfe853fe38d923e16edc593784a806dc8686574ec7c0c204a1f`; TFTP cursor `3375563` captured two 84,496-byte `da591740/kernel_2712.img` serves. Serial from cursor `1456046` captured `TALOS: data abort 20260522T1012Z`, normal boot/status output, `TALOS: before alignment data abort ad0x354989 vbar=0x200800 el=2`, `exception-info: esr=0x0000000096000021 elr=0x0000000000204480 far=0x0000000000354989`, `exception-class: data-abort-same-el ec=0x25`, `exception-status: spsr=0x600003c9`, and saved-register groups through `exception-regs7`.
- Rationale: Alignment checking gives Talos a controlled same-EL data abort without relying on translation tables. The nonzero FAR and EC `0x25` prove the default fatal report now carries the key diagnostic fields needed for early memory/runtime faults.
- Risks: This proves one deliberate alignment fault only. Page-table aborts, lower-EL aborts, IRQ/FIQ/SError dispatch, data-abort recovery, nested exception behavior, and production exception policy remain separate work.

## 2026-05-22 - Pi 5 Current-SP0 Synchronous Vector Reaches Fatal Report

- Status: accepted with prior local formatter/test/QEMU/image/disassembly gates, a fresh long-window serial control, and Pi 5 serial hardware evidence.
- Context: The default fatal report was proven for current-SPx synchronous exceptions, but the vector table still needed direct coverage of the current-SP0 synchronous slot. This matters because SP0 uses `SP_EL0` and has a distinct vector-table offset from the ordinary current-stack path.
- Decision: Add a narrow Pi 5 current-SP0 BRK diagnostic. It installs the normal exception vectors, writes `SP_EL0` with a valid aligned scratch stack, selects `SPSel=0`, executes `brk #0`, and lets the default non-returning fatal handler print vector identity, ESR/ELR/FAR, ESR class, SPSR, and saved GPR groups. This remains diagnostic-only and does not define production SP0 ownership or recovery.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test`, `scripts/qemu-smoke.sh`, normal Pi 5 image build, `scripts/rpi5-current-sp0-sync-diagnostic-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, archive review, strings inspection, and targeted disassembly passed. The accepted diagnostic image is 84,440 bytes with sha256 `d187c54675c5b3be65875d3c875ec55ebe968096e71feaa47dfce227d1edb72c`; the archive is `14be758bdd6e97a8399e4b26a860d14de850091a9311684e198354ec01bbee26`.
- Hardware result: Long-window fresh-entry control `rpi5-sp0-long-window-fresh-control-20260522T1056Z` first recovered current Talos-origin serial from `TALOS: sp0 ctrl reset 1056Z`, with TFTP serving the 84,376-byte control kernel and serial capturing `Starting OS` plus BL31. The accepted SP0 rerun `rpi5-sp0-long-window-rerun-20260522T1059Z` then published tree `55d0fd256d6bf6401a5446f51a0e84588d162d5fb5c4a15f5520368ed1fc574d`; TFTP served the 84,440-byte `da591740/kernel_2712.img` twice. Serial captured `TALOS: before SP0 BRK sp0=0x354950 vbar=0x200800 el=2`, `talos exception: current-sp0-sync`, `exception-info: esr=0x00000000f2000000 elr=0x000000000020441c`, `exception-class: brk-aarch64 ec=0x3c`, `exception-status: spsr=0x200003c8`, and saved-register groups through `exception-regs7`.
- Rationale: The current-SP0 vector slot now has direct hardware evidence, not just inference from current-SPx. The fatal saved-frame path can be trusted for the SP0 synchronous slot during near-term exception bring-up.
- Risks: This proves one deliberate current-SP0 BRK only. It does not define production use of SP0, SP0 resume/recovery, IRQ/FIQ/SError behavior, lower-EL vectors, or general exception policy.

## 2026-05-22 - Normal Pi 5 Boot Log Includes Version String

- Status: accepted with local formatter/test/QEMU/image/disassembly gates and Pi 5 serial hardware evidence from a clean non-diagnostic boot config.
- Context: Phase 2.1 calls for normal Pi 5 serial output to include a version string, exception level, core ID, and panic path. The standard `print!`/`println!` path, multi-field boot-info line, and panic path were already accepted, but the normal boot banner did not yet report the package version.
- Decision: Add `println!("talos: version {}", env!("CARGO_PKG_VERSION"))` to the normal Pi 5 boot banner immediately after the board line. Do not introduce git metadata, timestamps, build IDs, or a broader versioning policy yet.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test`, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, strings inspection, and targeted `kernel_main` disassembly passed. The normal image contains `talos: version 0.1.0`; disassembly shows the version line prints through `target::console::_print` before the existing formatted boot identity lines.
- Hardware result: Run `rpi5-normal-version-line-clean-20260522T111827Z` staged archive `63d0300e9e623a4491af5a76acce444af58ac6296ba56175de0c9cdc7fba686a` as tree `544b59ae74f1479658e489cb021054698a1d07513276e93512e15733d9e61201`. Archive review reported `loader_diagnostic=false`. TFTP served the 84,400-byte `da591740/kernel_2712.img` twice with kernel sha256 `af1c11309ca5efa68601a3e563f75513e5adeae244d6f4d13b28a67bc7af1bae`. Serial captured `talos: version 0.1.0`, `talos: boot core 0`, `talos: boot info: dtb=0x2efec600 core=0 el=2 target=talos-rpi5-bcm2712`, the services line, and `talos: status early boot log ready`.
- Rationale: This closes the roadmap version-string gap with the smallest useful normal-path change and keeps the accepted Daedalus-like printing surface as the user-facing API.
- Risks: The version is only the Cargo package version. This does not define release provenance, git revision reporting, build reproducibility metadata, or a complete boot identity schema.

## 2026-05-22 - Normal Pi 5 Boot Reads Firmware DTB Header

- Status: accepted with local formatter/test/QEMU/image/disassembly gates and Pi 5 serial hardware evidence from the normal boot path.
- Context: Phase 2.2 requires preserving and inspecting the firmware-provided device tree from the arm64 boot `x0` pointer. Talos already printed the raw DTB address, but had not read memory at that pointer.
- Decision: Add a minimal FDT header reader to `DeviceTree`. It performs volatile big-endian reads of the 40-byte header, accepts only magic `0xd00dfeed`, and leaves node walking, reservations, `/chosen`, and memory extraction for later tasks. Normal Pi 5 boot prints magic, total size, version, last compatible version, structure size, and strings size through the standard `println!` surface.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 7 no_std tests, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, strings inspection, archive review, and targeted disassembly passed. The normal image is 88,896 bytes with sha256 `8bb1acc7c91d48f52e0bd13da387344b83d2742714aae4121d142122e74ff078`; disassembly shows `kernel_main` calling `DeviceTree::fdt_header`, which calls the volatile big-endian word reader before the DTB header `println!`.
- Hardware result: Run `rpi5-dtb-header-normal-20260522T113146Z` staged archive `12e8134e9bc5d37cadc76ee95855d77f0f949911f54dcecff8b4610221c5df03`; archive review reported `loader_diagnostic=false`. Lab publish/status showed matching 88,896-byte root and `da591740/` kernels. TFTP logs captured `da591740/kernel_2712.img` served at 88,896 bytes, and serial captured normal boot/status output plus `talos: dtb header: magic=0xd00dfeed size=80254 version=17 last_comp=16 struct=72496 strings=7702`.
- Rationale: This is the smallest useful DTB handoff acceptance step: the pointer is readable, the header is a real FDT, and Talos now has exact bounds for later parser work.
- Risks: This does not parse the device tree structure, validate the full blob length against mapped memory, extract memory reservations, read `/chosen`, or derive the usable DRAM map.

## 2026-05-22 - Normal Pi 5 Boot Reads /chosen Bootargs

- Status: accepted with local formatter/test/QEMU/image/disassembly gates, a known-good DTB-header control run, and Pi 5 serial hardware evidence from the normal boot path.
- Context: After the FDT header was accepted, Phase 2.2 needed one narrow proof that Talos can walk the firmware-provided structure block far enough to read boot-state data. `/chosen/bootargs` is the most useful next property because it confirms the firmware command line, console path, and Talos-specific first-light flag.
- Decision: Extend `DeviceTree` with a bounded no-allocation structure-block walker for exactly the root `/chosen` node and `bootargs` property. It reads FDT tokens directly, resolves property names through the strings block, and returns the FDT-backed bootargs string. This is not a full parser and does not model arbitrary nodes, memory reservations, or memory banks.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 8 no_std tests, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, strings inspection, archive review, and targeted disassembly passed. The accepted image is 119,800 bytes with sha256 `42c6dc6be26fd6e40aae36e4dd36a07a9e14342cf151c449f5d2ae560c52356c`; disassembly shows `kernel_main` calling `DeviceTree::chosen_bootargs` and the chunked bootargs writer.
- Hardware result: The first bootargs archive `b8d6a5cae936a80699a9fd8b1d39054e578a7dca4dcd7c10c0fe4f8727e77cd8` served a 98,304-byte kernel and reached the bootargs line, but the long serial string corrupted after the `8250.nr_uarts` prefix. A known accepted DTB-header control archive `12e8134e9bc5d37cadc76ee95855d77f0f949911f54dcecff8b4610221c5df03` then booted cleanly and captured the prior DTB header/status output, proving the lab path. The accepted rerun `rpi5-chosen-bootargs-chunked-20260522T115821Z` staged archive `ddbb92b5003871a6dbc08c80c52c868825425ea14ac3dc36d23a2f52178c2472` as tree `cc62aa91fc93c07398d9bc1a75abba22c41daf255e484a82ccf8b9971608d37a`; TFTP served `da591740/kernel_2712.img` at 119,800 bytes and serial captured the full bootargs line including `console=ttyAMA10,115200`, `earlycon=pl011,mmio32,0x1f00030000`, and `talos.boot=first-light`.
- Rationale: The parser and output are intentionally scoped to one accepted property. The chunked writer keeps a long FDT-backed string readable on the early UART path without changing the public `print!`/`println!` API or broad UART ownership policy.
- Risks: This still does not validate a complete FDT parser, memory reservations, `/memory` nodes, `reserved-memory`, aliases, interrupt topology, or usable-DRAM extraction. The bootargs line is observation evidence, not a command-line policy parser.

## 2026-05-22 - Normal Pi 5 Boot Reads FDT Memory Reservations

- Status: accepted with local formatter/test/QEMU/image inspection gates, one partial hardware iteration, and Pi 5 serial hardware evidence from the normal boot path.
- Context: After accepting the FDT header and /chosen bootargs, Talos needed a narrow reservation-block observation before Phase 3.1 physical-memory-map work. The FDT reservation map is separate from /memory and reserved-memory nodes, so it can be read without a full node model.
- Decision: Add bounded FDT memory-reservation scanning to DeviceTree. The reader uses volatile big-endian 64-bit pair reads from off_mem_rsvmap, stores up to four entries for reporting, stops at the zero/zero terminator, and caps scanning at 64 entries. Normal Pi 5 boot prints the dtb reserved summary after the accepted bootargs/status context.
- Local validation: cargo fmt --all -- --check, cargo -Zjson-target-spec test with 9 no_std tests, scripts/qemu-smoke.sh, scripts/rpi5-image.sh, scripts/rpi5-format-guard-check.sh, git diff --check, strings inspection, and targeted disassembly passed. The accepted normal image is 124,552 bytes with sha256 `07dd0fdd8650b2f4b2433d7752b4899c2ea20fed5b17d54758efac1347a3414e`.
- Hardware result: Initial run `rpi5-dtb-reservations-normal-20260522T1218Z` staged archive `c6791e21cff0aecc0decf8c6fc6cfc1a05ef0c36c4c69fc936038268fca76b88` and served a 124,560-byte kernel. Serial captured normal identity/header output and `talos: dtb reserved: count=0 shown=0 truncated=false`, but stopped before the existing bootargs/status context, so the line was reordered after the accepted bootargs/status path. Accepted rerun `rpi5-dtb-reservations-after-status-20260522T1227Z` staged archive `2f8add11eff46fae8f04dcc01e52e53af44b9ba1fbfc0f41ac9d2e24c7cd7773` as tree `b36bf2df7c4e56846cf85e7e8fa3352f7afd730dddb9d3d77dfb4365c962da65`; TFTP served the 124,552-byte `da591740/kernel_2712.img` twice and serial captured normal identity/header output, the full /chosen bootargs line, `talos: status early boot log ready`, and `talos: dtb reserved: count=0 shown=0 truncated=false`.
- Rationale: This proves the FDT reserve-map pointer and terminator are readable on the normal Pi 5 boot path and gives Phase 3.1 a small accepted parser primitive before broader memory ownership work.
- Risks: The accepted boot tree has zero FDT reservation entries. This does not parse /memory, reserved-memory, GPU/firmware carveouts represented as nodes/properties, aliases, MMIO topology, or the final usable-DRAM map.

## 2026-05-22 - Normal Pi 5 Boot Reads /memory reg Banks

- Status: accepted with local formatter/test/QEMU/image/archive/disassembly gates, several diagnostic hardware iterations, and Pi 5 serial hardware evidence from the normal boot path.
- Context: After accepting the FDT header, `/chosen/bootargs`, and FDT reservation block, Phase 2.2 needed a bounded proof that Talos can extract DRAM bank data from the firmware-provided FDT before designing Phase 3.1 memory ownership.
- Decision: Extend `DeviceTree` with a bounded root `/memory` `reg` reader. It reads root `#address-cells` and `#size-cells`, parses the first root-level memory node's `reg` property into up to four reported banks, caps total scanned entries, and returns when that memory node closes. Normal Pi 5 boot prints address/size cell counts, total and shown bank counts, truncation status, and each shown bank.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 10 no_std tests, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, archive review, string inspection, and targeted disassembly passed. The accepted image is 133,264 bytes with sha256 `ff2ea673ef39b87676578c1994ecd8c139826e86bb7ff7a58b872dcc429cba37`; the archive is `b08bd703758335cf6b4c044e03321ea967b5116d43fbdfc1c8bdbbbfb45ba023`.
- Hardware result: Earlier attempts proved the boot path reached the accepted DTB header, bootargs, status, and reservation line, but reset or lost current serial before the memory scan line. The accepted long-window run `rpi5-dtb-memory-reg-reservation-boundary-20260522T1354Z` published tree `61ddffb5ecaf8e0164f1196e09636015fefb977d61079b4f681dc2e1ff8075a6`; TFTP served `kernel_2712.img` twice at 133,264 bytes. Serial captured `TALOS: dtb reserved start`, `talos: dtb reserved: count=0 shown=0 truncated=false`, `TALOS: dtb reserved done`, `TALOS: dtb memory scan start`, `TALOS: dtb memory scan done`, `talos: dtb memory: address_cells=2 size_cells=2 count=3 shown=3 truncated=false`, and banks `[0] addr=0x0 size=0x3fc00000`, `[1] addr=0x40000000 size=0xc0000000`, and `[2] addr=0x100000000 size=0x100000000`.
- Rationale: Talos now has direct hardware evidence that the normal firmware DTB exposes readable memory bank data through `/memory/reg`, including the split below the firmware/GPU carveout and the high 4 GiB bank. This is the right input to the next memory-map task, not a final allocation policy.
- Risks: The reported banks are observation data only. Talos still must parse/respect `reserved-memory`, firmware carveouts, the loaded kernel image, stacks, page tables, MMIO regions, and allocator ownership before using this as a physical memory map.

## 2026-05-22 - Normal Pi 5 Boot Reports Conservative Low Usable RAM

- Status: accepted with local formatter/test/QEMU/image/archive/symbol gates, a fresh serial control, and Pi 5 serial hardware evidence from the normal boot path.
- Context: After accepting FDT reservations and `/memory/reg` banks, Phase 3.1 needed a first conservative bridge from observation data to a future physical allocator. The goal was one bounded candidate, not allocator ownership or a complete memory policy.
- Decision: Add a no-allocation early memory-map helper that picks the low `/memory` bank containing the Talos linker-owned kernel image, excludes the kernel/runtime range, the firmware-provided DTB blob, and any reported FDT reservation entries that intersect that bank, then reports one 4 KiB-aligned `low-tail` candidate. Normal Pi 5 boot prints the kernel/heap/stack range, DTB blob range, and usable candidate before the per-bank `/memory` lines.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 12 no_std tests, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, archive review, string inspection, and readelf symbol inspection passed. The accepted image is 137,896 bytes with sha256 `c08b61be87e29cae17431164bda93dcb14ba6f579d4a92d1c02259aabac0b4dd`; the archive is `99d1787faac8e5f6d3dbb8908796259350a7275bebf25e701222fe6c64063532`.
- Hardware result: The first served usable-RAM image reached the accepted DTB memory bank lines but reset before the new layout lines, so the report was moved before the per-bank output. A later run served the moved image but the first serial observe returned only NUL; a fresh-entry reset control `rpi5-usable-control-rerun-20260522T1432Z-control` then proved current serial output by capturing `TALOS: usable ctrl reset 1432Z`. The accepted rerun `rpi5-usable-control-rerun-20260522T1432Z-usable` republished tree `48c791afbe3cc67583ae0ac0cae538e5544d445e18e6010bd4fd1f1796001e86`; TFTP served the 137,896-byte `kernel_2712.img` twice. Serial captured normal boot identity/status context, FDT header and bootargs, reservation context, `/memory` bank context, `talos: memory layout: kernel=0x200000..0x362000 heap=0x221ac0..0x321ac0 stack=0x321ac0..0x361ac0`, `talos: memory layout: dtb=0x2efec600..0x2effff7e size=0x1397e`, and `talos: memory usable: bank=0 start=0x2f000000 end=0x3fc00000 size=0x10c00000 align=0x1000 policy=low-tail`.
- Rationale: The low-tail candidate gives allocator work a precise, hardware-observed starting point while staying conservative about firmware/DTB/kernel ownership and high memory. It deliberately avoids consuming memory above 4 GiB or treating the raw `/memory` banks as free lists.
- Risks: This does not parse `reserved-memory` child nodes, MMIO topology, page-table reservations, high-memory policy, or allocator/free-list ownership. The reported candidate is an accepted observation and future allocator input, not a final physical memory manager.

## 2026-05-22 - Normal Pi 5 Boot Reads /reserved-memory Ranges

- Status: accepted with local formatter/test/QEMU/image/archive/symbol gates, one partial hardware run that exposed serial-label fragility, and Pi 5 serial hardware evidence from the normal boot path.
- Context: The accepted low-tail usable-RAM observation still treated `reserved-memory` child nodes as future work. Before allocator ownership, Talos needed a bounded observation of the firmware DTB's node-based reservations in addition to the zero-entry FDT reserve map.
- Decision: Extend `DeviceTree` with a bounded no-allocation `/reserved-memory` reader. It tracks the parent address/size cell counts, counts first-level child nodes, parses child `reg` properties into up to four reported ranges, caps scanned ranges, and records `no-map` / `reusable` flags. Normal Pi 5 boot prints short reserved-memory scan markers, a summary line, and shown ranges before the existing `/memory` bank and low-tail usable-RAM output.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 13 no_std tests, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, strings inspection, archive review, and readelf symbol inspection passed. The accepted image is 146,568 bytes with sha256 `c3f169e2fb64dd11b941e4f85a3ce535f3f2aa810ad254e6e34cca5e02cbd7dd`; the archive is `8f71733ce50529c195040a9e578e251e6d1cdb140b37415f9c9c40f4ea1439ab`.
- Hardware result: The first published tree `7231c07adbf0667f2c431e5de764f56e5983e41129f6b1a173385ed89fd6dc99` served the new kernel but serial captured only firmware/NUL. A rerun captured current Talos output through reserved-memory data, but long labels were partially corrupted, so the labels were shortened. The accepted run `rpi5-reserved-memory-observation-20260522T1502Z` published tree `a42402a01e8ccfb301573132ce2d58634fcf2716fdf61719db4e4977ba536e03`; TFTP served `kernel_2712.img` twice. Serial captured normal boot identity/status, DTB header and bootargs, `talos: dtb reserved: count=0 shown=0 truncated=false`, `TALOS: reserved-memory start`, `TALOS: reserved-memory done`, `talos: reserved-memory: addr_cells=2 size_cells=2 nodes=4 ranges=3 shown=3 truncated=false`, ranges `[0] addr=0x0 size=0x80000 no_map=true reusable=false`, `[1] addr=0x3fd23160 size=0x3d no_map=true reusable=false`, and `[2] addr=0x0 size=0x0 no_map=true reusable=false`, followed by the accepted `/memory` banks and low-tail candidate.
- Rationale: Talos now has hardware evidence for both FDT reservation-map entries and `/reserved-memory` child-node ranges on the normal Pi 5 boot path. Keeping this as observation output avoids prematurely treating all firmware-provided node data as allocator-owned policy.
- Risks: The reader reports observed ranges; it does not yet filter zero-sized ranges, merge reservations into the low-tail candidate, reserve page tables/MMIO, parse arbitrary child properties, or implement allocator/free-list ownership.

## 2026-05-22 - Conservative Low Usable RAM Filters /reserved-memory Ranges

- Status: accepted with local formatter/test/QEMU/image/archive/string gates and Pi 5 serial hardware evidence from the normal boot path.
- Context: The accepted reserved-memory observation showed three reported ranges, including one zero-sized artifact and one nonzero no-map range near the end of low memory. Before allocator ownership, the low-tail candidate needed to consume those bounded observations instead of merely printing them.
- Decision: Extend the no-allocation `conservative_low_memory_candidate` helper to accept optional `FdtReservedMemoryRanges`. It now reserves around each reported nonzero `/reserved-memory` range that intersects the selected bank, after excluding the kernel/runtime range, DTB blob, and FDT reservation-map entries. Zero-sized ranges keep the existing empty-range behavior and do not move the candidate.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 15 no_std tests, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, target image string inspection, and `scripts/rpi5-archive-review.sh` passed. Unit coverage now checks that an intersecting reserved-memory range moves the candidate start and that a zero-sized reserved-memory range is ignored. The accepted normal image is 146,616 bytes with sha256 `3d844c5b6a7aa433292f1b0cfbaa3d37d380014d77fd4f4a8bdb0b83c2a9c90a`; the archive is `46a4c91bab0ffce576ab6afacc3a266b8d3ec7807c7f752a037c3c02df951f19`.
- Hardware result: Run `rpi5-reserved-memory-usable-filter-20260522T1509Z` published tree `1f7362afb9819c49c676b2036f754e00a2a2b08b2a8198698ddea16d424ab5c4`; TFTP served the 146,616-byte `kernel_2712.img` twice. Serial captured normal boot identity/status, DTB header and bootargs, `talos: dtb reserved: count=0 shown=0 truncated=false`, reserved-memory ranges `[0] addr=0x0 size=0x80000`, `[1] addr=0x3fd23160 size=0x3d`, and `[2] addr=0x0 size=0x0`, followed by `talos: memory usable: bank=0 start=0x2f000000 end=0x3fc00000 size=0x10c00000 align=0x1000 policy=low-tail`.
- Rationale: The hardware result confirms the filtering is active without changing this boot's candidate: the only high nonzero reserved-memory range starts above bank 0's `0x3fc00000` end, the low range is below the current DTB-derived start, and the third range is zero-sized. The policy is now conservative for future intersecting ranges while preserving the current accepted usable candidate.
- Risks: This still reports one early low-tail candidate only. It does not allocate memory, build a page-frame list, reserve page tables, exclude MMIO topology, consume unreported/truncated reserved-memory ranges, or define high-memory policy.

## 2026-05-22 - Low-Tail Usable RAM Seeds a Page-Frame Span

- Status: accepted with local formatter/test/QEMU/image/archive/string gates, one partial hardware capture, and a successful Pi 5 serial hardware rerun from the normal boot path.
- Context: After filtering `/reserved-memory` into the conservative low-tail candidate, Phase 3.1 needed a first allocator-prep boundary that describes the candidate in page-frame terms without implementing allocator ownership.
- Decision: Add a no-allocation `early_page_frame_seed_span` helper that rounds an `EarlyUsableMemory` candidate to 4 KiB boundaries, rejects empty or sub-page spans, and reports `start`, `end`, `page_size`, and `page_count`. Normal Pi 5 boot prints the seed immediately after the existing `memory usable` line.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 17 no_std tests, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, target image string inspection, and `scripts/rpi5-archive-review.sh` passed. Unit coverage checks aligned seed derivation and sub-page rejection. The accepted normal image is 150,816 bytes with sha256 `c49a2a48057d9fd098f250e121be3b3f18bf9b2e7bf7d3db643435936b5653b1`; the archive is `82c82657b4b44c41c2c39d38675e6710f4fbeeb198cf944b4575e54682c68a20`.
- Hardware result: Initial run `rpi5-page-frame-seed-20260522T1524Z` published tree `9ac871e58212067e0d74d8e82a3d0fd8922ec3abc417b4f8f9b5351e67c2eb6a` and served the 150,816-byte kernel twice, but the serial observe stopped before the memory lines. Rerun `rpi5-page-frame-seed-rerun-20260522T1534Z` served `da591740/kernel_2712.img` at 150,816 bytes and captured `talos: memory usable: bank=0 start=0x2f000000 end=0x3fc00000 size=0x10c00000 align=0x1000 policy=low-tail`, followed by `talos: page frames seed: start=0x2f000000 end=0x3fc00000 pages=0x10c00 page_size=0x1000 source=memory-usable`.
- Rationale: This gives the future physical page allocator a concrete, hardware-observed seed span while preserving the current conservative ownership boundary. The seed is derived from accepted usable-memory filtering rather than independently reinterpreting FDT data.
- Risks: This does not initialize a mutable allocator, free list, page-table reservation, MMIO exclusion list, high-memory policy, allocator metadata placement, or ownership handoff.

## 2026-05-22 - Early Bootstrap Pages Reserved From Seed Span

- Status: accepted with local formatter/test/QEMU/image/archive/string gates, one partial hardware capture, and a successful Pi 5 serial hardware rerun from the normal boot path.
- Context: After accepting a page-frame seed span from the conservative low-tail usable candidate, Phase 3.1 needed a first static reservation boundary for future page-table/bootstrap memory without implementing a mutable allocator or enabling the MMU.
- Decision: Add a no-allocation `early_bootstrap_page_reservation` helper that reserves a fixed 16 pages from the start of an `EarlyPageFrameSeed`, rejects zero-page reservations and reservations that consume or exceed the seed, and returns the remaining page-frame seed. Normal Pi 5 boot prints the bootstrap reservation and the remaining frame span immediately after the accepted page-frame seed line.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 19 no_std tests, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, target image string inspection, and `scripts/rpi5-archive-review.sh` passed. Unit coverage checks the 16-page carveout and rejection of empty or fully consumed seeds. The accepted normal image is 150,976 bytes with sha256 `ffbe36c30106aa171df81d984224078c8865f225cf2a9bdfb70336d0bd484ae3`; the archive is `8cd351aafbc825554cadef8149534cc180f684d0f95c271126f5ae132a099eeb`.
- Hardware result: Initial run `rpi5-bootstrap-page-reserve-20260522T154359Z` published tree `fd12299210cf044aa04695fd2ae46bc8f63193c96eb8b3aad38a94bda37eea10` and served the 150,976-byte kernel, but serial capture stopped around the DTB header. Rerun `rpi5-bootstrap-page-reserve-rerun-20260522T154645Z` served `da591740/kernel_2712.img` twice at 150,976 bytes and captured the accepted memory usable line, the page-frame seed line, `talos: bootstrap reserve: start=0x2f000000 end=0x2f010000 pages=0x10 page_size=0x1000 reason=bootstrap-page-tables`, and `talos: page frames remaining: start=0x2f010000 end=0x3fc00000 pages=0x10bf0 page_size=0x1000 source=bootstrap-reserve`.
- Rationale: Reserving a small fixed bootstrap span creates a concrete input for the next MMU/page-table task while preserving the current ownership boundary. The remaining seed stays explicit so later allocator work starts from the post-reservation span, not from the raw low-tail candidate.
- Risks: This does not build translation tables, enable the MMU, initialize a free list, exclude MMIO ranges beyond the already accepted low-tail policy, define allocator metadata placement, or consume high memory.

## 2026-05-22 - Stage a Layout-Only Translation Table Area

- Status: accepted
- Context: With the 16-page bootstrap reservation accepted on hardware, Phase 3.2 needed a deterministic page-table staging layout before descriptor population or MMU enablement.
- Decision: Add a no-allocation `early_translation_table_layout` helper that derives four 4 KiB table slots from the front of the accepted bootstrap reservation: `root`, `l1`, `l2_low`, and `l2_mmio`. The helper rejects reservations smaller than the fixed layout. Normal Pi 5 boot prints the layout span and slot addresses immediately after the bootstrap reservation line.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test`, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, archive review, and targeted string inspection passed. The hardware image was 151,128 bytes with sha256 `eceb78da86073490b314ace1813649947e18305b7483fb7bc8a74b75faeb3b85`; archive `target/tmp/rpi5-translation-table-layout-20260522T1554Z.tar.gz` had sha256 `f6d74749f0ebc31b79d2c8e242c6669a96392f850456c9142ca79bc09a8c77c6`.
- Hardware result: Run `rpi5-translation-table-layout-20260522T1554Z` published tree `9f4ccc3238054cca561b86332bc9be96ccbf1f2df6277c7fd4803319d33a91ec`. TFTP served `kernel_2712.img` twice at 151,128 bytes. Serial captured the accepted memory usable, page-frame seed, and bootstrap reservation lines, followed by `talos: translation tables: start=0x2f000000 end=0x2f004000 pages=0x4 page_size=0x1000 kind=layout-only` and `talos: translation table slots: root=0x2f000000 l1=0x2f001000 l2_low=0x2f002000 l2_mmio=0x2f003000`.
- Rationale: Fixing the table-slot addresses makes the next descriptor-population task concrete and keeps the reserved bootstrap memory visible in normal boot evidence.
- Risks: This is a layout-only reservation. It does not populate descriptors, choose final memory attributes, enable the MMU, test post-MMU serial, or transfer ownership of remaining frames to a mutable allocator.

## 2026-05-22 - Populate Early Translation Table Descriptors Without Enabling MMU

- Status: accepted
- Context: The fixed `root`, `l1`, `l2_low`, and `l2_mmio` table-slot layout was accepted on normal Pi 5 hardware, making the next Phase 3.2 step descriptor population rather than another layout probe.
- Decision: Add no-allocation descriptor helpers and an unsafe early population helper that zeroes the four accepted table pages, writes `root[0] -> l1`, `l1[0] -> l2_low`, and `l1[0x41] -> l2_mmio`, maps low `0x0..0x40000000` with 512 normal 2 MiB block descriptors, and maps BCM2712 local peripherals `0x107c000000..0x1080000000` with 32 device 2 MiB block descriptors. Normal boot reports descriptor counts, block size, map windows, indices, and attribute indices. MMU enablement remains explicitly out of scope.
- Local validation: Initial implementation passed `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 24 no_std tests, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, archive review, and string inspection. The first hardware image was 155,808 bytes with sha256 `741794cee8c24d1dad0e0e0bf4f43b201263c75932a983cb8d714d7a458afbda`; archive `target/tmp/rpi5-translation-table-descriptors-20260522T1612Z.tar.gz` had sha256 `a76cbabcaa23a53a9f2b01f22f81cdf6910d7d3bbbf4014e9d236e9c8c76feec`.
- Hardware iteration: The first descriptor run published tree `183d49a70809eb1b3a01ef7bad57558606814bee5e4c2d5752f9c8cf971ad5db` and served the 155,808-byte kernel twice. Serial reached the descriptor lines, but the population label lost bytes (`oi0x1` instead of `root_entries=0x1`) because the initial static UART write was too long for the unpolled early UART path. The result was useful evidence but not accepted.
- Fix validation: Split the long descriptor summary labels into shorter static writes, then reran `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test`, QEMU smoke, normal Pi 5 image, format guard, `git diff --check`, archive review, and strings. The fixed image is 155,808 bytes with sha256 `79145f6dc5213c59fd8cb86153ec91f93a271735c01d0b3ea9734ac62a9bfacb`; archive `target/tmp/rpi5-translation-table-descriptors-fix-20260522T1624Z.tar.gz` has sha256 `7e39704ceccb566505677cbfcb0d9ecf724ac403952022295056b703a62758c9`.
- Hardware result: Rerun `rpi5-translation-table-descriptors-fix-rerun2-20260522T1623Z` left the fixed tree staged with hash `80c15ca2ccc23555af5998cd5d4d8d068b1928222f6b86ae32f4f1ae4c47d2dc`. TFTP served `da591740/kernel_2712.img` twice at 155,808 bytes. Serial captured `talos: translation table population: root_entries=0x1 l1_entries=0x2 low_l2_blocks=0x200 mmio_l2_blocks=0x20 block_size=0x200000 kind=stage1-4k-no-enable` and `talos: translation map policy: low=0x0..0x40000000 mmio=0x107c000000..0x1080000000 root_index=0x0 low_l1_index=0x0 mmio_l1_index=0x41 normal_attr=0x0 device_attr=0x1`, followed by the unchanged remaining page-frame line and DTB memory banks.
- Rationale: Populating descriptors now turns the accepted reservation into concrete boot-time state while keeping MMU enablement as a separately reviewable task with clear register and post-enable serial evidence requirements.
- Risks: This does not write `MAIR_ELx`, `TCR_ELx`, `TTBRx_ELx`, or `SCTLR_ELx.M`; it maps only low memory and the BCM2712 local-peripheral window, not RP1 or high memory; it does not validate post-MMU serial, cacheability, final shareability, allocator ownership, or translation-fault behavior.

## 2026-05-22 - Report EL2 Translation-Control Plan Without Enabling MMU

- Status: accepted
- Context: After accepting populated stage-1 descriptors for the Pi 5 early table skeleton, the next risky step is writing translation-control registers and setting `SCTLR_EL2.M`. Before doing that, Talos needed a hardware-visible no-write plan that ties the register values to the accepted table root and attributes.
- Decision: Add a no-allocation `early_translation_register_plan` helper for the current EL2 Pi 5 boot path. It reports `MAIR_EL2=0x4ff`, `TCR_EL2=0x53510`, `TTBR0_EL2=<root table>`, `sctlr_set=0x1`, and 48-bit VA/PA coverage. The normal boot prints this as `kind=el2-stage1-4k-no-enable` after descriptor population and before the remaining page-frame line.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 26 no_std tests, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, archive review, and target string inspection passed. The accepted image is 160,136 bytes with sha256 `bb6b7c88fe3c221004735bb9126bec90d368d1739e8e5db3d21def0a14562bf7`; archive `target/tmp/rpi5-translation-control-plan-20260522T1645Z.tar.gz` has sha256 `6784032ea32580d326ec6ec52aeba5f3d18297850ce90a92e43d0483ae7d9036`.
- Hardware result: Run `rpi5-translation-control-plan-20260522T1645Z` published tree `c651056a46ff892a9b1162ea18acd39238d4aff98490ba192410bff99ccf4143`. TFTP served `da591740/kernel_2712.img` twice at 160,136 bytes. Serial captured the accepted descriptor population and map-policy lines followed by `talos: translation control plan: el=0x2 mair=0x4ff tcr=0x53510 ttbr0=0x2f000000 sctlr_set=0x1 va_bits=0x30 pa_bits=0x30 kind=el2-stage1-4k-no-enable`, then the remaining page-frame and DTB memory-bank lines.
- Rationale: Making the register plan visible on hardware creates a reviewable baseline for the first MMU-enable diagnostic and avoids mixing register selection mistakes with post-enable serial or translation-fault behavior.
- Risks: This still does not write `MAIR_EL2`, `TCR_EL2`, `TTBR0_EL2`, or `SCTLR_EL2`; does not enable caches or the MMU; maps only low memory and the BCM2712 local-peripheral window; and does not cover RP1, high memory, lower-EL translation, or translation-fault recovery.

## 2026-05-23 - Enable EL2 Stage-1 Translation With Post-MMU Serial Proof

- Status: accepted with local formatter/test/QEMU/image/archive/string/disassembly gates, one partial hardware iteration, and Pi 5 serial hardware evidence from the normal boot path.
- Context: The populated early translation tables and no-write EL2 register plan had been accepted on hardware. The next Phase 3.2 step was a controlled write of the same plan with a post-enable UART proof, not a general virtual-memory subsystem.
- Decision: Add a Pi 5 EL2 helper that writes `MAIR_EL2`, `TCR_EL2`, and `TTBR0_EL2`, executes `isb`, `tlbi alle2`, `dsb sy`, `isb`, reads `SCTLR_EL2`, ORs in `SCTLR_EL2.M`, writes it back, and returns the resulting control value. Normal boot prints `TALOS: mmu enable start`, performs the enable, prints `TALOS: mmu enable done`, then reports `talos: translation enabled: el=0x2 ... kind=el2-stage1-4k-enabled`.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 26 no_std tests, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, target image string inspection, archive review, and targeted `llvm-objdump` inspection passed. The accepted marker image is 160,280 bytes with sha256 `b6a98f54d328371fa831f8863d56364935c3576e3ee06fa607040c518db28c59`; archive `target/tmp/rpi5-el2-mmu-enable-markers-20260523T0110Z.tar.gz` has sha256 `cbc2fd484a2b2587e0a589e038ddae1bc4dadc38cfc7448310343125432bb43b`.
- Hardware result: The first image `rpi5-el2-mmu-enable-20260523T0100Z` published tree `50f8e578a68e49c19676c35132b1fdfdbc72671f922e434f355dd7e47b92cdb5` and TFTP-served a 160,232-byte kernel, but serial captured the no-write control-plan line without the new post-enable line, so it was treated as partial evidence. The accepted marker rerun `rpi5-el2-mmu-enable-markers-20260523T0110Z` published tree `9bd63c90ade213dde998abf02cbe68bad27a90aaee13c7264e5a0b60eb51bc0b`; TFTP served `da591740/kernel_2712.img` twice at 160,280 bytes. Serial captured the current-run plan line, `TALOS: mmu enable start`, `TALOS: mmu enable done`, `talos: translation enabled: el=0x2 sctlr=0x30c50831 ttbr0=0x2f000000 kind=el2-stage1-4k-enabled`, and subsequent page-frame/DTB memory output.
- Rationale: The accepted evidence proves the minimal identity map covers the running kernel, stack, DTB access used in the current path, and the BCM2712 UART10 MMIO path after `SCTLR_EL2.M` is set.
- Risks: This still does not enable caches, map RP1 or high memory, define final kernel/user virtual-address policy, initialize allocator-owned page tables, support lower ELs, or recover from translation faults.

## 2026-05-23 - Pi 5 Translation-Fault Diagnostic After EL2 MMU Enable

- Status: accepted with local formatter/test/QEMU/image/disassembly/archive gates, three non-acceptance diagnostic hardware runs, one normal-path hardware control, and final Pi 5 serial hardware evidence.
- Context: Phase 3.2 requires a page-fault diagnostic after proving that serial output survives EL2 MMU enablement. The accepted early map covers low memory and BCM2712 local-peripheral MMIO only, so `0x80000000` is a canonical VA outside the current identity map.
- Decision: Add a narrow `TALOS_RPI5_TRANSLATION_FAULT_DIAGNOSTIC` build. The final accepted form minimizes unrelated DTB bootargs/reservation reporting, follows the same memory-bank scan, low-tail page-table layout, descriptor population, EL2 register plan, and `SCTLR_EL2.M` enable path, then calls an inline-never noreturn helper that prints the fault VA and performs a single load from `0x80000000`. The existing non-returning fatal exception path reports the trap.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 26 no_std tests, `scripts/qemu-smoke.sh`, normal Pi 5 image build, translation-fault diagnostic image build, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, string inspection, archive review, and helper disassembly passed. The accepted diagnostic image is 120,904 bytes with sha256 `f7ef4fe01713552b5b38018c5b43e8c96c76209a3d65294bc58f2854e4e42208`; archive `target/tmp/rpi5-translation-fault-minimal-20260523T0219Z.tar.gz` has sha256 `d2426cf035d751c391e9067eeb1cc4f619584037dd788575d6aeeb29127aa004`.
- Hardware result: Earlier full-diagnostic images were TFTP-served but stopped after `TALOS: dtb memory scan start`; a normal current-image control then proved the base path still reached `TALOS: dtb memory scan done`, `talos: translation enabled`, and the DTB memory-bank lines. The accepted minimized run `rpi5-translation-fault-minimal-20260523T0219Z` published tree `b21da6ab77a01f81c11b23c5ab3ed8dd90a70dab1a081615ea31a0bd822d6916`; TFTP served `da591740/kernel_2712.img` twice at 120,904 bytes. Serial captured `talos: translation enabled: el=0x2 sctlr=0x30c50831 ttbr0=0x2f000000 kind=el2-stage1-4k-enabled`, `TALOS: before translation fault va0x80000000 vbar=0x200800 el=2`, `exception-info: esr=0x0000000096000005 ... far=0x0000000080000000`, `exception-class: data-abort-same-el ec=0x25`, and saved register groups through `exception-regs7`.
- Rationale: The result proves an unmapped access after enabling the accepted EL2 stage-1 map produces a readable Talos fatal report with a precise FAR, giving the next MMU/runtime work a hardware-proven fault evidence path.
- Risks: This is diagnostic-only and intentionally halts. It does not add translation-fault recovery, demand paging, high-memory or RP1 mappings, lower-EL fault handling, cache enablement, or final kernel/user virtual-address policy.

## 2026-05-23 - Enable EL2 Instruction Cache After Stage-1 MMU

- Status: accepted with local formatter/test/QEMU/image/archive/string/disassembly gates, two hardware pickup attempts, and final Pi 5 serial hardware evidence from the normal boot path.
- Context: After accepting EL2 stage-1 translation and a translation-fault diagnostic, the next cache step needed to be narrow enough to avoid data-cache/DMA policy while proving that an architectural cache-control write does not break the normal Talos serial path.
- Decision: Add an `early_instruction_cache_enable_plan` helper that requires EL2 and an already-active `SCTLR_EL2.M`, then enable only `SCTLR_EL2.I` on the normal Pi 5 boot path. The AArch64 helper invalidates instruction cache to PoU with `ic iallu`, executes `dsb sy; isb`, sets the plan mask in `SCTLR_EL2`, and returns the resulting control value. The existing translation-fault diagnostic remains unchanged and still halts before this normal-path cache step.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 27 no_std tests, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, target image string inspection, archive review, and targeted `llvm-objdump` inspection passed. The accepted normal image is 164,576 bytes with sha256 `179ead0930dfd104617c9526f5102f32387349efc2fca77aa73ea73f29fc22bc`; archive `target/tmp/rpi5-icache-enable-20260523T0300Z.tar.gz` has sha256 `765e1d84b211c234c714be47da3731889e3d0cf48dfe4317ef9ebac3bf207693`.
- Hardware result: The first power-cycle captured only early firmware bytes before TFTP, and a second scripted wait missed usable serial even though later TFTP logs showed the 164,576-byte `da591740/kernel_2712.img` was served. The accepted evidence window published tree `ca326f1bd84b5c50e855d5b684be61eff1ff71ea88cab9f9ac18e7a302b5a37a`; TFTP logs show the accepted image served multiple times. Serial captured `talos: translation enabled: el=0x2 sctlr=0x30c50831 ...`, `talos: instruction cache plan: el=0x2 sctlr_before=0x30c50831 sctlr_set=0x1000 kind=el2-stage1-icache-enabled`, `TALOS: icache enable start`, `TALOS: icache enable done`, `talos: instruction cache enabled: el=0x2 sctlr=0x30c51831 kind=el2-stage1-icache-enabled`, and subsequent page-frame/DTB memory lines.
- Rationale: This proves the instruction-side cache bit can be enabled after the accepted EL2 stage-1 map while preserving UART10 device output and normal boot progress. Keeping data cache disabled leaves mutable data, DMA, and maintenance policy for a separate task.
- Risks: This does not enable `SCTLR_EL2.C`, perform data-cache maintenance, define DMA/cache coherency policy, map RP1 or high memory, or transfer frame ownership to an allocator.

## 2026-05-23 - Enable EL2 Data Cache After Stage-1 MMU and I-Cache

- Status: accepted with local formatter/test/QEMU/image/archive/disassembly gates and Pi 5 serial hardware evidence from the normal boot path.
- Context: After accepting EL2 stage-1 translation and instruction-cache enablement, the next direct cache-control step was enabling `SCTLR_EL2.C` without changing memory attributes, mappings, allocator ownership, or DMA policy.
- Decision: Add an `early_data_cache_enable_plan` helper that requires EL2 plus active `SCTLR_EL2.M` and `SCTLR_EL2.I`, then enable only `SCTLR_EL2.C` on the normal Pi 5 boot path. The AArch64 helper walks CLIDR/CCSIDR-selected data/unified caches, invalidates by set/way with `dc isw`, executes `dsb sy; isb`, sets the plan mask in `SCTLR_EL2`, and returns the resulting control value. The existing translation-fault diagnostic remains unchanged and still halts before normal-path cache steps.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 28 no_std tests, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, string inspection, archive review, and targeted disassembly passed. The accepted archive is `target/tmp/rpi5-dcache-enable-static-markers-20260523T0221Z.tar.gz` with sha256 `30413ae022638b9564e44907cb20a647cc12be28ab0d666d0c4ae9bd25a8b37a`; the kernel is 165,096 bytes with sha256 `8b8b7ceb113407151a3aa94a63f3e0223b9f864ca98af941ba475e6ff108c939`.
- Hardware result: The first archive `rpi5-dcache-enable-20260523T0216Z` was TFTP-served as a 165,136-byte kernel but serial after the pre-run cursor contained only two NUL/newline pairs, so it was treated as non-acceptance evidence. The follow-up removed a new early-phase payload helper from the pre-entry writer and used formatter-free static console writes for dcache start/done markers. Rerun evidence from `target/tmp/rpi5-dcache-enable-static-markers-20260523T0221Z-evidence/rerun` shows TFTP serving `da591740/kernel_2712.img` at 165,096 bytes and serial reaching `talos: data cache plan: el=0x2 sctlr_before=0x30c51831 sctlr_set=0x4 kind=el2-stage1-dcache-enabled`, `TALOS: dcache enable start`, `TALOS: dcache enable done`, `talos: data cache enabled: el=0x2 sctlr=0x30c51835 kind=el2-stage1-dcache-enabled`, and subsequent page-frame and DTB memory lines.
- Rationale: The accepted map already uses write-back/write-allocate normal-memory descriptors and Device-nGnRE for the UART/local-peripheral window. Invalidating data/unified caches before setting `SCTLR_EL2.C` gives Talos a narrow, hardware-proven cache-enabled normal path while keeping driver/DMA coherency policy separate.
- Risks: This does not define DMA buffer ownership, driver cache-maintenance APIs, allocator metadata placement under data cache, final kernel/user virtual-address policy, RP1/high-memory mappings, lower-EL translation, or translation-fault recovery.

## 2026-05-23 - Bootstrap Allocator Smoke After MMU and Caches

- Status: accepted with local formatter/test/QEMU/image/archive/string gates, three premature/non-acceptance hardware captures, a known-good data-cache control rerun, and final Pi 5 serial hardware evidence from the normal boot path.
- Context: After accepting the low-tail frame seed, fixed bootstrap page reservation, EL2 stage-1 translation, instruction cache, and data cache, Talos needed one narrow allocator ownership proof before enabling the full Rust `alloc` crate surface.
- Decision: Add `early_bootstrap_allocator_plan`, deriving a no-free low-tail bump span from `reservation.remaining`. The plan rejects empty, unaligned, or outside-low-map spans. Normal Pi 5 boot initializes a static `BumpAllocator` from that plan and performs a bounded direct `GlobalAlloc` smoke allocation of four `u64` values after the accepted MMU/I-cache/D-cache path.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 30 no_std tests, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, string inspection, and archive review passed. The accepted direct-allocation archive is `target/tmp/rpi5-bootstrap-manual-alloc-smoke-20260523T0316Z.tar.gz` with sha256 `0fed10084d5352f62f82fd2c12067d48aa7f6444742d172fd691807cf5171527`; the kernel is 174,936 bytes with sha256 `489b3cc30f195cb0cfc9242eb6032ece56f2dd26723488010a7ab763e632139f`.
- Hardware result: Initial `alloc`-crate String/Vec, trimmed Box, and direct-GlobalAlloc runs were TFTP-served but first observed only NUL/newline bytes, so they were treated as non-acceptance. A control rerun of the accepted data-cache archive `target/tmp/rpi5-alloc-regression-dcache-control-20260523T031428Z-evidence` proved current TFTP and serial by recapturing the MMU/I-cache/D-cache lines. The accepted allocator rerun `target/tmp/rpi5-bootstrap-manual-alloc-smoke-rerun-20260523T031629Z-evidence` published tree `85c07768e565fbec9260c63d3b408b3290ae3e9f7dc23b756580db21e8f6ba15`; TFTP served `da591740/kernel_2712.img` at 174,936 bytes, and serial captured the normal boot through `talos: data cache enabled`, then `talos: bootstrap allocator plan: start=0x2f010000 end=0x3fc00000 bytes=0x10bf0000 pages=0x10bf0 page_size=0x1000 kind=bump-no-free-low-tail`, `talos: bootstrap allocator init: start=0x2f010000 next=0x2f010000 end=0x3fc00000 policy=no-free`, `talos: alloc smoke: box=0x2f010000 items=4 sum=0x47 next=0x2f010020 ok=true`, and subsequent normal DTB memory-bank output.
- Rationale: This proves Talos can transfer the accepted remaining low-tail frames into a minimal mutable allocator and survive one cache-enabled allocation without disturbing UART10, DTB parsing, page-table memory, or the current normal output path.
- Risks: This is not a free-capable physical page allocator and does not define high-memory policy, allocator metadata placement beyond the low-tail bump span, DMA/cache-maintenance ownership, RP1 mappings, userspace mappings, or the Rust `alloc` crate/global allocator API.

## 2026-05-23 - Bootstrap Allocator Accounting and Exhaustion Guard

- Status: accepted with local formatter/test/QEMU/image/archive/string gates, one hardware review iteration, and final Pi 5 serial hardware evidence from the normal boot path.
- Context: After accepting a direct no-free bump allocation from `reservation.remaining`, Talos needed slightly stronger allocator introspection before expanding the allocation surface. The previous line also exposed that long static labels could drop bytes on the unpolled Pi 5 UART10 path.
- Decision: Store the allocator `start` alongside `next` and `end`, report `used_bytes` and `remaining_bytes` in `BumpAllocatorState`, and extend the normal Pi 5 direct allocation smoke with a deliberately oversized allocation that must return null without advancing `next`. Route Pi 5 `write_static` through the bytewise early UART10 word-write helper with a bounded empty wait after each byte so long static labels remain readable.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 33 no_std tests, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, string inspection, and archive review passed. The accepted archive is `target/tmp/rpi5-bootstrap-alloc-accounting-static-20260523T033337Z.tar.gz` with sha256 `379470ce3a233ee6d42070fcc1c91b883654712781f35dba6bb4586c06254e95`; the kernel is 175,016 bytes with sha256 `e7a0e6f487da9d50024cbd9888924150ff1271bb227014ebfd9c5401e54ef7dc`.
- Hardware result: The first accounting run published tree `29331e5338720cc783efa6a0cb2a7d6abcefc2bb5ff9fb0e24e59a7ce753c568`, served a 175,040-byte kernel, and proved `used=0x20 remaining=0x10beffe0 exhaustion_ok=true`, but the allocator plan/init labels lost the `start=` fragment. The accepted static-label rerun published tree `3df28e1aa88fb8192b9733f642d3113a33bafa90d487cbacdd98807879f6e169`; TFTP served `da591740/kernel_2712.img` at 175,016 bytes, and serial captured `talos: data cache enabled`, `talos: bootstrap allocator plan: start=0x2f010000 end=0x3fc00000 ...`, `talos: bootstrap allocator init: start=0x2f010000 next=0x2f010000 ...`, `talos: alloc smoke: box=0x2f010000 items=4 sum=0x47 next=0x2f010020 used=0x20 remaining=0x10beffe0 exhaustion_ok=true ok=true`, and subsequent DTB memory-bank lines.
- Rationale: The allocator now has a hardware-proven accounting boundary and a basic exhaustion invariant before Talos enables any broader allocation API. Fixing static label transmission also removes a recurring source of ambiguous hardware evidence.
- Risks: This remains a no-free bootstrap bump allocator. It does not enable `alloc`, install `#[global_allocator]`, introduce free/reuse, define high-memory or DMA/cache ownership, map RP1, or make a final virtual-memory heap policy.

## 2026-05-23 - Global Bootstrap Allocator Surface

- Status: accepted with local formatter/test/QEMU/image/archive/string gates, one firmware-only hardware pickup, and final Pi 5 serial hardware evidence from the normal boot path.
- Context: After accepting direct no-free bump allocation and accounting, Talos needed the Rust global allocator symbol installed before trying the broader alloc-crate surface again.
- Decision: Mark the accepted `BumpAllocator` static as the kernel `#[global_allocator]`, rename it to `KERNEL_GLOBAL_ALLOCATOR`, and keep the normal boot smoke on direct `GlobalAlloc` calls through that global symbol. The Rust `alloc` crate containers remain deferred.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 35 no_std tests, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, target string inspection, and archive review passed. Unit coverage now checks uninitialized null allocation and a direct global-style allocation/accounting smoke. The accepted archive is `target/tmp/rpi5-global-allocator-smoke-20260523T0348Z.tar.gz` with sha256 `bad26c815ab8b201cd0ad89694b8ace0ffda0b133dbdbe5952af898674589e42`; the kernel is 175,040 bytes with sha256 `93f63a609a2b28c9ceca4e00a2e8705bfa5a89818f7b99ba94278d6739188b19`.
- Hardware result: The first power cycle captured only firmware serial and no fresh TFTP delta, so it was treated as non-acceptance pickup evidence. A later observe captured normal Talos output from the staged tree through cache enablement, allocator plan/init, `talos: global alloc smoke: ptr=0x2f010000 items=4 sum=0x47 next=0x2f010020 used=0x20 remaining=0x10beffe0 exhaustion_ok=true ok=true`, and subsequent DTB memory-bank lines. The lab status after publish reported the staged 175,040-byte root and serial-prefixed kernels under tree hash `2cecbe8857ed14846e90dd249557b5374a1bc79d5b936d4ad7bed1b63565d2f7`; the TFTP delta endpoint did not expose fresh events for this run.
- Rationale: This establishes the standard Rust allocator hook without increasing the boot-time allocation surface. Any future `Box`, `Vec`, `String`, or collection work can now be tested as an alloc-crate policy decision instead of a global-symbol bring-up step.
- Risks: This is still a no-free bump allocator. It does not add OOM policy for alloc-crate containers, free/reuse, page-frame metadata, high-memory allocation, DMA/cache ownership, RP1 mappings, or userspace mappings.

## 2026-05-23 - Minimal Alloc-Crate Box Smoke

- Status: accepted with local formatter/test/QEMU/image/archive/string gates, one non-acceptance hardware pickup, and final Pi 5 serial hardware evidence from the normal boot path.
- Context: After accepting the kernel `#[global_allocator]` symbol with direct `GlobalAlloc` smoke, the next allocator surface needed to be a single alloc-crate container, not a broad `Vec`/`String`/collections enablement.
- Decision: Build the Pi 5 target with `alloc` in `build-std`, add the Pi 5 `alloc_error_handler` boundary, and replace the normal allocator smoke with exactly one `Box<[u64; 4]>` allocation after `KERNEL_GLOBAL_ALLOCATOR.init_from_plan`. The smoke preserves the existing oversized direct allocation guard and reports the same accounting fields.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 35 no_std tests, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, target string inspection, and archive review passed. The accepted archive is `target/tmp/rpi5-box-alloc-smoke-20260523T0358Z.tar.gz` with sha256 `5b1565f9274e392cd1b597377fd99d47212170be5563b7a9518dd9380481026f`; the kernel is 180,208 bytes with sha256 `0f7b15f1ce198c04679199b992644bfffd2d5c1929a10ab7279ff9851e714d73`.
- Hardware result: The first pickup served the 180,208-byte kernel but fresh serial stopped at `TALOS: dtb memory scan start`, so it was treated as non-acceptance and rerun without code changes. The accepted rerun under `target/tmp/rpi5-box-alloc-smoke-20260523T0358Z-evidence/rerun1` served `da591740/kernel_2712.img` twice at 180,208 bytes and serial captured the normal path through MMU/I-cache/D-cache enablement, allocator plan/init, `talos: box alloc smoke: ptr=0x2f010000 items=4 sum=0x47 next=0x2f010020 used=0x20 remaining=0x10beffe0 exhaustion_ok=true ok=true`, and subsequent DTB memory-bank lines.
- Rationale: This proves the accepted global allocator can back one concrete Rust `alloc` crate container under the cache-enabled normal path while keeping broad dynamic allocation policy out of the kernel.
- Risks: `Box` drop still maps to the no-op deallocator and the smoke intentionally forgets the box. Talos still lacks free/reuse, recoverable OOM handling, `Vec`, `String`, collections, page-frame allocator ownership, high-memory allocation, DMA/cache ownership, RP1 mappings, lower-EL mappings, and userspace mappings.

## 2026-05-23 - Pi 5 Hardware Images Use Size-Optimized Dev Builds

- Status: accepted for Pi 5 hardware image generation; the current Vec allocator smoke remains under investigation.
- Context: While attempting the first bounded `Vec<u64>` alloc-crate smoke, Box-only padding classifiers exposed a reproducible normal-image handoff/output cliff between 181,176 bytes and 181,184 bytes. A +904-byte Box control with a clamped 181,176-byte arm64 Image header still failed, and an accepted +896-byte control with eight appended file bytes also failed. That points at the loaded file length/boot-chain behavior rather than only the header field.
- Decision: Make `scripts/rpi5-image.sh` default Pi 5 dev-profile builds to `CARGO_PROFILE_DEV_OPT_LEVEL=z`. This keeps the normal debug artifact path and debuginfo, but produces much smaller hardware images for the Pi 5 boot loop. Callers can still override `CARGO_PROFILE_DEV_OPT_LEVEL` explicitly.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 35 no_std tests, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, target string inspection, and archive review passed for the active Vec diagnostic image. The marker Vec image is 80,552 bytes with sha256 `a55adc9789d376bea4ae7c59dba56ca15fc209fda3b710b776a24b7c6ebf7f8d`; archive `target/tmp/rpi5-vec-alloc-smoke-optz-marker-20260523T0612Z.tar.gz` has sha256 `5b35c2da3c30415a3440dc3c81f26884dc544d4de1117809f25d8d0ee9fc0b97`.
- Hardware result: The optimized Box +904 control image was 79,832 bytes and restored normal Talos serial output through the accepted Box alloc smoke, confirming the size-optimized hardware-image path works. The optimized Vec marker image was TFTP-served twice at 80,552 bytes but the retained serial evidence did not show the unique `TALOS: vec smoke start`, `TALOS: vec smoke allocated`, or `talos: vec alloc smoke` lines, so the Vec task is not accepted yet.
- Rationale: The Pi 5 hardware loop needs images that stay away from the observed firmware/lab handoff cliff while Talos is still built from the dev profile. This is a build-artifact policy, not an allocator feature acceptance.
- Risks: The exact boot-chain/file-length limit is not fully explained, and optimized codegen can move layout-sensitive bugs. Keep `scripts/rpi5-format-guard-check.sh`, archive review, and hardware evidence in the loop. `Vec`, `String`, collection growth, free/reuse, and recoverable OOM remain deferred until separate hardware acceptance.

## 2026-05-23 - Optimized Pi 5 Image File-Length Classifier

- Status: accepted as diagnostic evidence; the bounded `Vec<u64>` smoke remains unaccepted.
- Context: The optimized Vec call-site discriminator image was 80,632 bytes and produced no Talos-origin serial after TFTP served it. A same-size padded Box comparator was needed to separate Vec codegen/content from the Pi firmware/lab boot-chain file-length class.
- Decision: Treat the optimized normal-image file-length cliff as a current Pi 5 hardware validation constraint. Valid arm64 Image controls made from the accepted optimized Box image booted through the Box alloc smoke at 80,032, 80,132, 80,182, 80,208, 80,220, 80,226, 80,230, and 80,231 bytes. The same Box content failed to produce Talos-origin serial at 80,232 and 80,632 bytes.
- Hardware result: The accepted controls were TFTP-served as `da591740/kernel_2712.img` at their tested lengths and serial tail captured the normal cache-enabled path through `talos: box alloc smoke: ... exhaustion_ok=true ok=true`. The 80,232-byte and 80,632-byte controls were TFTP-served but fresh serial contained only firmware/RP1/NUL output and no Talos lines.
- Rationale: The Vec non-acceptance is explained by the staged image sitting above the current optimized file-length cliff before the Vec allocator code gets a chance to prove itself. The next Vec iteration should first reduce or otherwise change the boot artifact so the valid image is no larger than 80,231 bytes, then rerun hardware acceptance.
- Risks: The root cause of the 80,232-byte cliff is still unexplained and may depend on firmware, config, serial-prefix lookup, or lab boot-chain behavior. This diagnostic does not accept Vec, String, collection growth, free/reuse, recoverable OOM, or a final image-size policy.

## 2026-05-23 - Reject Profile-Level Shrink Mitigations for Vec Smoke

- Status: accepted as diagnostic evidence; the bounded `Vec<u64>` smoke remains unaccepted.
- Context: The compact optimized Vec smoke still exceeded the accepted 80,231-byte optimized file-length ceiling. The next obvious mitigations were build-profile changes that reduce image size without dropping the Vec semantics.
- Decision: Do not use `lto=true`, `codegen-units=1`, or `debug_assertions=false` as the default Pi 5 hardware-image mitigation for this task. Each setting reduced the Vec image below the file-length ceiling, but hardware failed before Talos-origin output; comparator Box images using the same profile changes also failed for LTO and disabled debug assertions. The image script remains on the accepted `opt-level=z` default only.
- Local validation: The LTO Vec image was 68,088 bytes, the codegen-units=1 Vec image was 75,408 bytes, and the disabled-debug-assertions Vec image was 56,432 bytes. Each passed the local image generation path before hardware testing. After reverting profile mitigations, the compact Vec source builds as an 80,520-byte image under the accepted default profile.
- Hardware result: The LTO Vec, LTO Box control, codegen-units=1 Vec, disabled-debug-assertions Vec, and disabled-debug-assertions Box control were all TFTP-served at their expected reduced sizes. Fresh serial evidence showed firmware/RP1/NUL output only, with no `TALOS: rust_entry`, allocator smoke, or later Talos lines.
- Rationale: These profile changes perturb the boot artifact or early code layout enough to invalidate the accepted Pi 5 hardware path, even when the Rust allocation surface is reduced back to the known-good Box control. The Vec task should continue with code-only shrinkage under the accepted profile, or with an explicit boot-artifact/layout fix, rather than adopting a profile-level workaround.
- Risks: The codegen-units=1 path was tested only with Vec in this run, not with a Box comparator, so it remains lower-confidence than the rejected LTO and disabled-debug-assertions paths. The root cause of the profile-sensitive failures is still unresolved.

## 2026-05-23 - Bounded Vec Smoke Accepted With Pi 5 RODATA Alignment Relaxed

- Status: accepted for one bounded `Vec<u64>` capacity/fill smoke on Pi 5 hardware; `String`, collections, Vec growth beyond capacity, free/reuse, and recoverable OOM remain unaccepted.
- Context: The compact Vec smoke under the accepted `opt-level=z` Pi 5 image path still built to 80,520 bytes, above the optimized-image file-length ceiling where Box controls accepted at 80,231 bytes and failed at 80,232 bytes. Profile-level shrink mitigations were rejected because they also broke known-good controls.
- Decision: Keep the accepted Pi 5 image profile and relax only the Pi 5 linker script `.rodata` output-section alignment from 4 KiB to 16 bytes. The current early identity map does not require a page-aligned `.rodata` boundary, and no runtime code consumes `__rodata_start`/`__rodata_end` for a page-granular contract. This reduces the Vec image to 79,928 bytes while preserving the Vec smoke semantics.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 35 no_std tests, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, archive review, target string inspection, and section/header inspection passed. The accepted archive is `target/tmp/rpi5-vec-rodata-align16-20260523T071536Z.tar.gz` with sha256 `49c5c30b8fb9e4e65461fb01458457252c11cbd9a5910d9d7daa345edf12d30c`; the kernel is 79,928 bytes with sha256 `fb1f5f60c47979dc01bc818927de14cfa44a2ad5a160efe103458c59c675e484`.
- Hardware result: The lab published the archive and power-cycled the Pi 5. Status after publish showed both root and serial-prefixed `kernel_2712.img`/`kernel8.img` at 79,928 bytes. TFTP logs from cursor 3534000 show `da591740/kernel_2712.img` served at 79,928 bytes at May 23 07:16:21 UTC, plus the matching DTB/config/cmdline/overlay files. Serial evidence in `target/tmp/rpi5-vec-rodata-align16-20260523T071536Z-evidence/serial-observe-late.json` captured the normal path through data-cache enablement, allocator plan/init, `talos: vec smoke: ptr=0x2f010000 len=4 cap=4 sum=0x47 next=0x2f010020 used=0x20 rem=0x10beffe0 ex=true ok=true`, and later page-frame/DTB memory output.
- Rationale: This accepts the narrow Vec capacity/fill allocation surface without broadening build-profile risk. The linker alignment change is a targeted boot-artifact/layout mitigation for the current Pi 5 file-length constraint, not a claim that the underlying firmware/lab cliff is fully explained.
- Risks: The no-free bump allocator still leaks/drops by policy, and the smoke intentionally avoids Vec growth. The root cause of the 80,232-byte optimized-image cliff remains unresolved. Future work still needs `String`, collections, recoverable OOM, free/reuse, final allocator metadata ownership, high-memory allocation, DMA/cache ownership, RP1 mappings, lower-EL mappings, and userspace mappings.

## 2026-05-23 - Bounded String Smoke Accepted on Pi 5

- Status: accepted for one bounded ASCII `String` fill smoke on Pi 5 hardware; collections, grow-on-demand containers, free/reuse, and recoverable OOM remain unaccepted.
- Context: After accepting the global allocator, one `Box<[u64; 4]>`, and one bounded `Vec<u64>`, the next alloc-crate surface was a single `String` allocation. The task kept the accepted Pi 5 `opt-level=z` image profile and the `.rodata ALIGN(16)` linker mitigation from the Vec acceptance.
- Decision: Replace the normal Pi 5 allocator smoke with one `String::with_capacity(8)` allocation after `KERNEL_GLOBAL_ALLOCATOR.init_from_plan`. The smoke fills the ASCII bytes for `Talos` within the preallocated capacity, verifies the pointer remains stable, preserves the oversized direct-allocation exhaustion guard, and reports the same allocator accounting fields.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 35 no_std tests, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, target string inspection, and archive review passed. The accepted archive is `target/tmp/rpi5-string-alloc-smoke-20260523T072816Z.tar.gz` with sha256 `d1c596310c48b53ffbbe0f652cbb7a100d28236cd949e3a2fdd3ae74903cc038`; the kernel is 79,296 bytes with sha256 `e25329af5223613c3421dc05c35b1533fd3402a53fa274be136ca796ff85cde2`.
- Hardware result: The lab published the archive and power-cycled the Pi 5. TFTP logs from cursor 3535507 show `da591740/kernel_2712.img` served at 79,296 bytes, plus the matching config and DTB. Serial evidence in `target/tmp/rpi5-string-alloc-smoke-20260523T072816Z-evidence/serial-observe.json` captured the normal path through data-cache enablement, allocator plan/init, `talos: string smoke: ptr=0x2f010000 len=5 cap=8 sum=0x203 next=0x2f010008 used=0x8 rem=0x10befff8 ex=true stable=true ok=true`, and later page-frame/DTB memory output.
- Rationale: This accepts a narrow String-backed allocation without broadening allocator policy. Filling bytes directly keeps the task focused on allocation ownership and stable capacity; higher-level string mutation, collections, growth, deallocation/reuse, and recoverable allocation failure remain separate tasks.
- Risks: The no-free bump allocator still leaks/drops by policy, the smoke intentionally avoids growth and UTF-8 mutation APIs, and the root cause of the optimized-image file-length cliff remains unresolved.

## 2026-05-23 - Fatal Alloc-Crate OOM Diagnostic Accepted on Pi 5

- Status: accepted for an explicit cfg-gated Pi 5 allocation-failure diagnostic; recoverable OOM, allocator growth, collections, free/reuse, and heap expansion remain unaccepted.
- Context: After accepting bounded `Box`, `Vec`, and `String` alloc-crate surfaces, Talos needed to prove that allocation failure through the Rust alloc crate produces readable Talos-origin output rather than a silent hang.
- Decision: Add `TALOS_RPI5_ALLOC_OOM_DIAGNOSTIC` and `scripts/rpi5-alloc-oom-diagnostic-image.sh`. The diagnostic runs only after `KERNEL_GLOBAL_ALLOCATOR` initialization, prints the requested capacity and remaining bump span, then calls `Vec::<u8>::with_capacity` with a capacity eight bytes larger than the remaining span. The existing `alloc_error_handler` prints the failed layout and spins.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 35 no_std tests, `scripts/qemu-smoke.sh`, normal `scripts/rpi5-image.sh`, diagnostic `scripts/rpi5-alloc-oom-diagnostic-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, target string inspection, and archive review passed. The accepted archive is `target/tmp/rpi5-alloc-oom-diagnostic-20260523T074627Z.tar.gz` with sha256 `34c54254a47f79d9273c2f289e840e8ff65cba634a41e6921cb87c25e79e5202`; the kernel is 79,504 bytes with sha256 `ecc63e1ea883a93fa589aaaae5ce9ddc993f986a9a3c30ef3f8087e12f5853b2`.
- Hardware result: The lab published tree `ba522bc40cd0c6cd972b3987e3548e4a72a213e6d74240d6b168639591c88bd3`, TFTP served `da591740/kernel_2712.img` at 79,504 bytes, and serial captured the normal cache-enabled path through allocator plan/init, `talos: alloc oom diagnostic: request=0x10bf0008 remaining=0x10bf0000 align=0x1`, and `talos: alloc error: size=0x10bf0008 align=0x1`.
- Rationale: The kernel now has a hardware-proven fatal OOM report for alloc-crate callers. This keeps the current no-free bump allocator honest without implying that allocation failure is recoverable.
- Risks: The diagnostic intentionally stops in the fatal OOM loop and is not the normal boot path. Talos still lacks allocator free/reuse, heap growth, page-frame-backed allocation, high-memory allocation, DMA/cache ownership, RP1 mappings, lower-EL mappings, userspace mappings, and recoverable allocation failure handling.

## 2026-05-23 - Direct Realloc Growth Diagnostic Accepted

- Status: accepted for the direct `GlobalAlloc::realloc` growth boundary on Pi 5 hardware; `Vec`/`String` grow-on-demand behavior, collections, free/reuse, recoverable OOM, allocator expansion, and page-frame-backed heap growth remain unaccepted.
- Context: After accepting bounded `Box`, bounded `Vec`, bounded `String`, and a fatal alloc-crate OOM diagnostic, the next allocator boundary was growth. A first Vec-growth diagnostic built to 82,400 bytes, above the current accepted 80,231-byte optimized Pi 5 image ceiling, so the task pivoted to the smaller prerequisite direct realloc growth path while leaving the normal bounded String smoke unchanged.
- Decision: Add `TALOS_RPI5_REALLOC_GROWTH_DIAGNOSTIC` and `scripts/rpi5-realloc-growth-diagnostic-image.sh`. The diagnostic runs only after `KERNEL_GLOBAL_ALLOCATOR` initialization, allocates two bytes through the global allocator, grows that allocation to four bytes through `GlobalAlloc::realloc`, verifies the copied prefix plus newly written tail, checks an oversized direct-allocation guard, then spins after printing the diagnostic line.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 36 no_std tests, `scripts/qemu-smoke.sh`, normal `scripts/rpi5-image.sh`, diagnostic `scripts/rpi5-realloc-growth-diagnostic-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, target string/header inspection, and `scripts/rpi5-archive-review.sh` passed. The accepted archive is `target/tmp/rpi5-realloc-growth-diagnostic-20260523T080210Z.tar.gz` with sha256 `ee1bf3bc6f760cc4e7dc2aaa8bdaa916f00525afd56b9ae6e91778b5062879d3`; the diagnostic kernel is 78,064 bytes with sha256 `cb7d203152c1ba4e1fa82c58a902d420bcd1b75bfa6c74d688c87bb9df6bb571`. The normal String-smoke image remains 79,296 bytes with sha256 `d29111ee2f34043d769baf364142d3a6a32dfaf8b49aba6a379e4725374a0ac2`.
- Hardware result: The lab published the archive, power-cycled the Pi 5, and TFTP served `da591740/kernel_2712.img` at 78,064 bytes. Serial evidence in `target/tmp/rpi5-realloc-growth-diagnostic-20260523T080210Z-evidence/serial-observe-late.json` captured normal boot through data-cache enablement and allocator initialization, then `talos: realloc grow smoke: old=0x2f010000 new=0x2f010002 size=4 sum=0x47 next=0x2f010006 used=0x6 rem=0x10befffa ex=true moved=true ok=true`.
- Rationale: This establishes the no-free bump allocator realloc contract directly before trying higher-level grow-on-demand containers again. The old allocation is retained, the grown allocation is separate, and allocator accounting shows exactly six bytes consumed.
- Risks: This diagnostic intentionally uses direct `GlobalAlloc` calls rather than `Vec` or `String` growth. It does not add deallocation, reuse, allocator expansion, recoverable allocation failure, cache/DMA ownership policy, high-memory allocation, lower-EL mappings, userspace mappings, or collection support.

## 2026-05-23 - Vec Growth Diagnostic Accepted

- Status: accepted for one cfg-gated `Vec<u8>` growth path on Pi 5 hardware; `String` growth, broad collections, free/reuse, recoverable OOM, allocator expansion, and page-frame-backed heap growth remain unaccepted.
- Context: The first Vec-growth diagnostic passed local gates but did not produce Talos-origin serial, and a rebuilt same-size String control also failed in that pickup window. A known accepted realloc archive rerun was needed before changing code again. The accepted realloc control rerun booted cleanly, proving the lab/boot-chain path was usable and moving the issue back to the Vec diagnostic's source/layout.
- Decision: Keep the diagnostic cfg-gated under `TALOS_RPI5_VEC_GROWTH_DIAGNOSTIC`, but make the growth path narrower and more explicit: allocate capacity two, fill the first two initialized slots with direct pointer writes, call `Vec::reserve_exact(2)` to force growth, fill the new tail, and report the resulting pointer movement, length, capacity, sum, allocator accounting, oversized direct-allocation guard, and ok status. The diagnostic image script now leaves the image unpadded by default and only pads when `TALOS_RPI5_VEC_GROWTH_DIAGNOSTIC_PAD_SIZE` is explicitly set.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 36 no_std tests, `scripts/qemu-smoke.sh`, normal `scripts/rpi5-image.sh`, diagnostic `scripts/rpi5-vec-growth-diagnostic-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, target string/header inspection, and `scripts/rpi5-archive-review.sh` passed. The accepted archive is `target/tmp/rpi5-vec-growth-reserveexact-20260523T090032Z.tar.gz` with sha256 `c83cb8fe959607d3a16f6362a697b076e81bafe8fc71af64050592dec2c88d7c`; the diagnostic kernel is 76,152 bytes with sha256 `44c245b20e59874899debbe186b80134f67d80ec68fdee264cfe5749240b9c29`.
- Hardware result: First, the known accepted realloc control archive `target/tmp/rpi5-realloc-growth-diagnostic-20260523T080210Z.tar.gz` was republished and power-cycled; TFTP served `da591740/kernel_2712.img` at 78,064 bytes and serial again captured `talos: realloc grow smoke: old=0x2f010000 new=0x2f010002 size=4 sum=0x47 next=0x2f010006 used=0x6 rem=0x10befffa ex=true moved=true ok=true`. Then the Vec growth archive was published; TFTP served `da591740/kernel_2712.img` at 76,152 bytes and serial evidence in `target/tmp/rpi5-vec-growth-reserveexact-20260523T090055Z-evidence` captured `talos: vec grow start` followed by `talos: vec grow smoke: old=0x2f010000 new=0x2f010002 len=4 cap=4 sum=0x47 next=0x2f010006 used=0x6 rem=0x10befffa ex=true moved=true ok=true`.
- Rationale: This accepts the next narrow alloc-crate growth layer while staying inside the current Pi 5 image-size/layout constraints. The result matches the direct realloc accounting boundary and proves the alloc-crate reserve path preserves the existing prefix under the no-free bump policy.
- Risks: This remains a diagnostic-only `Vec` path and intentionally leaks by policy. Talos still lacks `String` growth, general collection policy, free/reuse, recoverable OOM, heap expansion, high-memory allocation, DMA/cache ownership, RP1 mappings, lower-EL mappings, and userspace mappings.

## 2026-05-23 - String Growth Diagnostic Accepted

- Status: accepted for one cfg-gated ASCII `String` growth path on Pi 5 hardware; general string formatting, UTF-8 mutation policy, broad collections, free/reuse, recoverable OOM, allocator expansion, and page-frame-backed heap growth remain unaccepted.
- Context: After accepting direct realloc growth and one `Vec<u8>` reserve growth diagnostic, the next narrow alloc-crate growth boundary was `String`. The first unprefixed String diagnostic image was TFTP-served but produced no Talos-origin output, and immediate follow-up cycles did not reach fresh TFTP. A known accepted Vec-growth control was rerun first to recover a clean lab/boot-chain signal.
- Decision: Keep the diagnostic cfg-gated under `TALOS_RPI5_STRING_GROWTH_DIAGNOSTIC`. The path creates an ASCII string with capacity two, writes the first two bytes directly through the backing `Vec<u8>`, calls `reserve_exact(2)` to force growth, fills the new tail, verifies the four-byte ASCII payload, and reports pointer movement, length, capacity, sum, allocator accounting, oversized direct-allocation guard, and ok status. The diagnostic uses the same growth-only output pruning and no-default-padding image shape as the accepted Vec growth diagnostic.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 36 no_std tests, `scripts/qemu-smoke.sh`, normal `scripts/rpi5-image.sh`, diagnostic `scripts/rpi5-string-growth-diagnostic-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, target string/header inspection, and `scripts/rpi5-archive-review.sh` passed. The accepted prefixed archive is `target/tmp/rpi5-string-growth-diagnostic-prefixed-20260523T092331Z.tar.gz` with sha256 `3618534945d2e05915fa96a2fdbc012f049403f706e442c635d9a442ab1559ee`; the diagnostic kernel is 76,152 bytes with sha256 `f3a172e6aae09fd015ff10c2393ed52b276e7a44ce3c705b609df7ef0e95ce99`.
- Hardware result: First, the known accepted Vec-growth control `target/tmp/rpi5-vec-growth-reserveexact-20260523T090032Z.tar.gz` was republished and power-cycled; TFTP served `da591740/kernel_2712.img` at 76,152 bytes and serial evidence in `target/tmp/rpi5-string-growth-control-vec-recovery-20260523T094140Z-evidence` captured `talos: vec grow smoke: old=0x2f010000 new=0x2f010002 len=4 cap=4 sum=0x47 next=0x2f010006 used=0x6 rem=0x10befffa ex=true moved=true ok=true`. Then the prefixed String growth archive was published; TFTP served `da591740/kernel_2712.img` at 76,152 bytes and serial evidence in `target/tmp/rpi5-string-growth-diagnostic-prefixed-rerun-20260523T094315Z-evidence` captured `talos: string grow start` followed by `talos: string grow smoke: old=0x2f010000 new=0x2f010002 len=4 cap=4 sum=0x190 next=0x2f010006 used=0x6 rem=0x10befffa ex=true moved=true ok=true`.
- Rationale: This accepts the next narrow alloc-crate growth layer while keeping String behavior deliberately ASCII and bounded. The allocator accounting matches the direct realloc and Vec growth boundaries: the old two-byte allocation is retained by no-free policy, the grown four-byte allocation is separate, and exactly six bytes are consumed.
- Risks: This remains a diagnostic-only String growth path and intentionally leaks by policy. Talos still lacks general string/formatting allocation policy, UTF-8 mutation policy beyond this four-byte ASCII diagnostic, general collection policy, free/reuse, recoverable OOM, heap expansion, high-memory allocation, DMA/cache ownership, RP1 mappings, lower-EL mappings, and userspace mappings.

## 2026-05-23 - Alloc Format String Diagnostic Accepted

- Status: accepted for one cfg-gated `alloc::format!` String construction path on Pi 5 hardware; broad runtime string formatting, collections, free/reuse, recoverable OOM, allocator expansion, and page-frame-backed heap growth remain unaccepted.
- Context: After accepting direct realloc growth, `Vec<u8>` growth, and ASCII `String` growth, the next narrow boundary was alloc-backed formatting. The first unpadded 73,384-byte alloc-format image passed local gates but produced no Talos-origin serial after TFTP served it. A plain 76,152-byte padded variant also failed. Adding a fresh-entry marker to the unpadded image proved insufficient, producing only a NUL/newline after TFTP.
- Decision: Keep the diagnostic cfg-gated under `TALOS_RPI5_ALLOC_FORMAT_DIAGNOSTIC`, build `alloc::format!("{} {}", "Talos", 5usize)` after `KERNEL_GLOBAL_ALLOCATOR` initialization, verify the exact `Talos 5` ASCII bytes, and report pointer, length, capacity, sum, allocator accounting, oversized direct-allocation guard, and `ok=true`. The accepted hardware archive uses the same 76,152-byte staged image size as the accepted String-growth diagnostic plus a fresh-entry marker, so the current boot-chain/layout constraint is documented rather than hidden.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 36 no_std tests, `scripts/qemu-smoke.sh`, normal `scripts/rpi5-image.sh`, diagnostic `scripts/rpi5-alloc-format-diagnostic-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, target string/header inspection, and `scripts/rpi5-archive-review.sh` passed. The accepted archive is `target/tmp/rpi5-alloc-format-entry-marker-padded76152-20260523T1025Z.tar.gz` with sha256 `c5c4d436071bf91d3f5ecfd2746a6140026b0ff3211219ff3f6d90c5ad36454f`; the diagnostic kernel is 76,152 bytes with sha256 `1585eb1b4e4c91eaf8198677ca6cb4b4ad6297ec9f2d51e0157d6ad8b6311bbc`.
- Hardware result: The lab published the accepted archive and power-cycled the Pi 5. TFTP served `da591740/kernel_2712.img` twice at 76,152 bytes. Serial evidence in `target/tmp/rpi5-alloc-format-entry-marker-padded76152-20260523T1025Z-evidence/serial-observe.txt` captured repeated `TALOS: alloc-format entry` markers, `TALOS: rust_entry`, cache enablement through `TALOS: dcache enable done`, then `talos: alloc format start` and `talos: alloc format smoke: ptr=0x2f010000 len=7 cap=7 sum=0x258 next=0x2f010007 used=0x7 rem=0x10befff9 ex=true ascii=true ok=true`.
- Rationale: This accepts the smallest useful alloc-backed formatting construction path while keeping normal boot on the previously accepted bounded String smoke. The result proves the no-free bump allocator supports the allocation pattern used by `alloc::format!` for this fixed ASCII string.
- Risks: This remains a diagnostic-only formatting path and intentionally leaks by policy. Talos still lacks broad runtime formatting allocation policy, general string/UTF-8 mutation policy, collections, free/reuse, recoverable OOM, heap expansion, high-memory allocation, DMA/cache ownership, RP1 mappings, lower-EL mappings, and userspace mappings. The need for the 76,152-byte staged image shape is a boot-chain/layout constraint, not an explained architectural limit.

## 2026-05-23 - Post-Allocator Println Smoke Accepted

- Status: accepted for one normal Pi 5 post-allocator runtime report emitted through the ordinary Daedalus-like `println!` surface; broad alloc-backed runtime formatting and allocator policy changes remain unaccepted.
- Context: The normal boot path already had hardware acceptance for multi-field `println!` before memory-management setup and for panic/exception reporting. After accepting the global allocator, bounded String smoke, grow diagnostics, and one cfg-gated `alloc::format!` construction path, Talos needed to prove the normal print surface still works after MMU, instruction-cache, data-cache, and allocator initialization.
- Decision: Keep the existing normal bounded `String::with_capacity(8)` smoke and oversized direct-allocation exhaustion guard, but emit the final String-smoke accounting line through ordinary `println!` instead of piecemeal `target::console::write_static`/`write_hex_u64`/`write_dec_usize` calls.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 36 no_std tests, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, target string/header inspection, and `scripts/rpi5-archive-review.sh` passed. The accepted archive is `target/tmp/rpi5-post-allocator-println-smoke-20260523T102902Z.tar.gz` with sha256 `a35f0d9b191ce672b776f354986327e007cda41697b5056df58bc1772b6638f8`; the normal kernel is 79,533 bytes with sha256 `b79abda4bafd991a30346426cce4959501235e0d3ccffa2dda57f4af6ca6988d`.
- Hardware result: The lab published the archive and power-cycled the Pi 5. TFTP served `da591740/kernel_2712.img` twice at 79,533 bytes. Serial evidence in `target/tmp/rpi5-post-allocator-println-smoke-20260523T102902Z-evidence/serial-observe-followup.txt` captured normal boot through `TALOS: dcache enable done`, allocator plan/init, and `talos: string smoke: ptr=0x2f010000 len=5 cap=8 sum=0x203 next=0x2f010008 used=0x8 rem=0x10befff8 ex=true stable=true ok=true`.
- Rationale: This makes the accepted print surface useful later in the runtime bring-up path without changing allocation semantics. It proves that one standard formatted `println!` line survives the current cache-enabled, allocator-initialized Pi 5 environment.
- Risks: The result is one normal runtime report, not a blanket policy for collection-heavy logging, alloc-backed formatting in arbitrary kernel paths, recoverable allocation failure, free/reuse, heap expansion, DMA/cache ownership, RP1 mappings, lower-EL mappings, or userspace mappings.

## 2026-05-23 - Bootstrap Allocator Init Println Accepted

- Status: accepted for the normal Pi 5 bootstrap allocator init report emitted through the ordinary Daedalus-like `println!` surface; broad alloc-backed runtime formatting and allocator policy changes remain unaccepted.
- Context: The post-allocator String-smoke report had already proved one formatted `println!` after allocator initialization. The allocator-init line is longer and runs immediately after `KERNEL_GLOBAL_ALLOCATOR.init_from_plan`, so it was a useful next boundary for normal runtime logging.
- Decision: Emit `talos: bootstrap allocator init: start=... next=... end=... policy=no-free` through `println!` instead of piecemeal static/hex writes. The first hardware run TFTP-served the image and reached the String smoke, but serial showed `talos: bootstrap allocator init:0x2f010000 ...`, missing the required `start=` literal. Treat that as non-acceptance and change the Pi 5 formatter console backend from posted-write flush-only to TX-ready polling plus posted-write flush while preserving 32-bit UART10 data-register writes.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 36 no_std tests, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, target string/header inspection, and `scripts/rpi5-archive-review.sh` passed. The accepted archive is `target/tmp/rpi5-bootstrap-allocator-init-println-polled-20260523T104717Z.tar.gz` with sha256 `702839e2ec54df1595296399ed7cdb20d93ef44c145692f0a2e2d8bd912c9698`; the normal kernel is 79,557 bytes with sha256 `3bd15a7792ecf300ca2c22e9656ba11896efe5e2ef5510e330a62eadb5fa1b7b`.
- Hardware result: The lab published the accepted archive and power-cycled the Pi 5. TFTP served `da591740/kernel_2712.img` twice at 79,557 bytes. Serial evidence in `target/tmp/rpi5-bootstrap-allocator-init-println-polled-20260523T104717Z-evidence/serial-peek-after-rerun2-late.json` captured normal boot through data-cache enablement, `talos: bootstrap allocator init: start=0x2f010000 next=0x2f010000 end=0x3fc00000 policy=no-free`, and `talos: string smoke: ptr=0x2f010000 len=5 cap=8 sum=0x203 next=0x2f010008 used=0x8 rem=0x10befff8 ex=true stable=true ok=true`.
- Rationale: The result hardens the Pi 5 `core::fmt::Write` backend for longer formatted literals and moves one more allocator-adjacent runtime report onto the normal print surface without changing allocation semantics.
- Risks: TX-ready polling is still tied to the firmware-preserved UART10 path and current hardware evidence. This does not add collection-heavy logging, alloc-backed formatting in arbitrary kernel paths, recoverable allocation failure, free/reuse, heap expansion, DMA/cache ownership, RP1 mappings, lower-EL mappings, or userspace mappings.

## 2026-05-23 - Bootstrap Allocator Plan Println Accepted

- Status: accepted for the normal Pi 5 bootstrap allocator plan report emitted through the ordinary Daedalus-like `println!` surface; broad alloc-backed runtime formatting and allocator policy changes remain unaccepted.
- Context: After accepting the post-allocator String-smoke `println!` and the allocator-init `println!`, the allocator plan line was the next allocator-adjacent report still using piecemeal static/hex writes. It runs after MMU, instruction-cache, and data-cache enablement, but before `KERNEL_GLOBAL_ALLOCATOR.init_from_plan`, so it must not allocate.
- Decision: Emit `talos: bootstrap allocator plan: start=... end=... bytes=... pages=... page_size=... kind=bump-no-free-low-tail` through `println!` while preserving the allocator plan, allocator initialization, no-free bump behavior, and normal bounded String smoke.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 36 no_std tests, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, target string/header inspection, and `scripts/rpi5-archive-review.sh` passed. The accepted local archive is `target/tmp/rpi5-bootstrap-allocator-plan-println-20260523T105643Z.tar.gz` with sha256 `3a1483566c272c4ffbe3b71282df98c3d8aa885fc601759d07e6123d2d085799`; the normal kernel is 79,709 bytes with sha256 `8c76beb7745d4cb6338a1ca55beb1484dd2320c96133a4ff65611a9b7b724396`.
- Hardware result: The lab published boot archive `target/tmp/rpi5-bootstrap-allocator-plan-println-20260523T105926Z-boot.tar.gz` with sha256 `0c6e26c358af91d338667a214c9fb2b575b768a180975dcf21cbf3eb063a8e00` and power-cycled the Pi 5. TFTP served `da591740/kernel_2712.img` twice at 79,709 bytes. Serial evidence in `target/tmp/rpi5-bootstrap-allocator-plan-println-20260523T105926Z-evidence/serial-observe-extended.json` captured normal boot through data-cache enablement, `talos: bootstrap allocator plan: start=0x2f010000 end=0x3fc00000 bytes=0x10bf0000 pages=0x10bf0 page_size=0x1000 kind=bump-no-free-low-tail`, `talos: bootstrap allocator init: start=0x2f010000 next=0x2f010000 end=0x3fc00000 policy=no-free`, and `talos: string smoke: ptr=0x2f010000 len=5 cap=8 sum=0x203 next=0x2f010008 used=0x8 rem=0x10befff8 ex=true stable=true ok=true`.
- Rationale: This keeps moving normal allocator-adjacent boot reports onto the public `print!`/`println!` API now that the Pi 5 backend has hardware-proven TX-ready pacing for long formatted lines.
- Risks: The plan report still depends on the firmware-preserved UART10 path and current post-cache boot ordering. This does not accept arbitrary pre-allocator formatting, alloc-backed logging, recoverable allocation failure, free/reuse, heap expansion, DMA/cache ownership, RP1 mappings, lower-EL mappings, or userspace mappings.

## 2026-05-23 - Data Cache Enabled Println Accepted

- Status: accepted for the normal Pi 5 data-cache-enabled status report emitted through the ordinary Daedalus-like `println!` surface; cache-transition mechanics and broader pre-allocator formatting policy remain unchanged.
- Context: After accepting post-allocator, allocator-init, and allocator-plan `println!` reports, the next nearby normal runtime/logging boundary was the data-cache-enabled line. It runs after MMU, instruction-cache, and data-cache enablement but before `KERNEL_GLOBAL_ALLOCATOR.init_from_plan`, so it must not allocate and must preserve the following allocator plan/init/String smoke output.
- Decision: Emit `talos: data cache enabled: el=... sctlr=... kind=el2-stage1-dcache-enabled` through `println!` instead of piecemeal static/hex writes. Keep `TALOS: dcache enable start` and `TALOS: dcache enable done` formatter-free around the actual cache transition.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 36 no_std tests, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, target string/header inspection, and `scripts/rpi5-archive-review.sh` passed. The accepted boot archive is `target/tmp/rpi5-data-cache-enabled-println-20260523T111335Z-boot.tar.gz` with sha256 `5f56602af4db6a03e25c7d3a4bea53f9cb3a1def5e23625ef4530e94a68dfc09`; the normal kernel is 79,757 bytes with sha256 `fc00cce302eafc6b776e1729197f4fd424aa4899576854fce8edaa011b61ad24`.
- Hardware result: The lab published the archive and power-cycled the Pi 5. TFTP evidence in `target/tmp/rpi5-data-cache-enabled-println-20260523T111335Z-evidence/tftp-full-tail.json` showed `da591740/kernel_2712.img` served at 79,757 bytes. Retained serial evidence in `target/tmp/rpi5-data-cache-enabled-println-20260523T111335Z-evidence/serial-peek-manual.json` captured `TALOS: dcache enable start`, `TALOS: dcache enable done`, `talos: data cache enabled: el=0x2 sctlr=0x30c51835 kind=el2-stage1-dcache-enabled`, the accepted allocator plan/init `println!` lines, and the normal String smoke with `ok=true`.
- Rationale: This expands the normal boot log surface one step earlier while preserving a formatter-free marker pair around the cache transition itself. The evidence also confirms that a pre-allocator `println!` does not disturb the subsequent allocator setup path.
- Risks: The line still depends on the firmware-preserved UART10 path and current post-cache boot ordering. This does not change MMU/cache programming, add cache/DMA ownership policy, accept arbitrary pre-allocator formatting, accept alloc-backed logging before allocator init, or add allocator free/reuse, heap expansion, lower-EL mappings, or userspace mappings.

## 2026-05-23 - Instruction Cache Enabled Println Not Accepted

- Status: not accepted for the normal Pi 5 instruction-cache-enabled status report; the line remains on formatter-free static/hex output, and the accepted normal `println!` cache boundary remains after data-cache enablement.
- Context: After accepting `println!` for post-allocator String smoke, bootstrap allocator init, bootstrap allocator plan, and the data-cache-enabled status line, the next candidate was the earlier instruction-cache-enabled status line. It runs after `SCTLR_EL2.I` is set but before `SCTLR_EL2.C`, so it is a stricter pre-data-cache formatting boundary than the accepted data-cache report.
- Decision: Do not promote `talos: instruction cache enabled: el=... sctlr=... kind=el2-stage1-icache-enabled` to ordinary `println!` yet. Keep the instruction-cache enabled report on bounded formatter-free writes and retain formatter-free start/done markers around the cache transition.
- Local validation: The promoted candidate passed `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 36 no_std tests, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, target string/header inspection, and `scripts/rpi5-archive-review.sh`. Candidate archive `target/tmp/rpi5-instruction-cache-enabled-println-20260523T1124Z-boot.tar.gz` had sha256 `83adbf46e3fccf09ed66643a25922da9d20ebae149d2c182943675dd0d0a80ed`; the candidate kernel was 79,773 bytes with sha256 `db0c2f9411c8dc9dd04c5ee8db932ec385f96a11f7b3d6e090fad9b7068db303`.
- Hardware result: The lab published the candidate archive and power-cycled the Pi 5. TFTP evidence in `target/tmp/rpi5-instruction-cache-enabled-println-20260523T1124Z-evidence` showed `da591740/kernel_2712.img` served at 79,773 bytes, but fresh serial observation captured only firmware/RP1 boot lines or no new bytes, not current Talos-origin output or the downstream data-cache/allocator/String smoke lines. A rerun and extended wait produced the same non-accepted class.
- Rationale: This separates the accepted post-data-cache, pre-allocator `println!` boundary from the still-unaccepted pre-data-cache formatter boundary. The result avoids weakening the steady boot log just to force a line earlier than the current cache/console evidence supports.
- Risks: This is a negative hardware decision, not an explanation of the exact source/layout/cache interaction. Future work may revisit pre-data-cache `println!` with a narrower discriminator, but the normal path should continue advancing from the accepted post-data-cache formatter surface.

## 2026-05-23 - Page Frames Remaining Println Accepted

- Status: accepted for the normal Pi 5 page-frames-remaining report emitted through the ordinary Daedalus-like `println!` surface; page-frame ownership and allocator policy remain unchanged.
- Context: After closing the earlier instruction-cache-enabled `println!` candidate as not accepted, the next normal boot-log candidate moved forward from the hardware-proven post-data-cache formatter boundary. The page-frames-remaining report runs after data-cache enablement, bootstrap allocator initialization, and the bounded String smoke.
- Decision: Emit `talos: page frames remaining: start=... end=... pages=... page_size=... source=bootstrap-reserve` through `println!` while preserving the existing bootstrap reserve span, page-frame accounting, allocator setup, and normal String smoke.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 36 no_std tests, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, target string/header inspection, and `scripts/rpi5-archive-review.sh` passed. The accepted archive is `target/tmp/rpi5-page-frames-remaining-println-20260523T115914Z-boot.tar.gz` with sha256 `3ee3313876108a0244eebd04a497d00b7b4a5a36762dee415d90f40a84a854b8`; the normal kernel is 79,661 bytes with sha256 `e254da8960114773f5dfb809f6e09093f4873bba618f6bf649978f44b4a46f83`.
- Hardware result: The lab published the archive and power-cycled the Pi 5. TFTP evidence in `target/tmp/rpi5-page-frames-remaining-println-20260523T120006Z-evidence` showed `da591740/kernel_2712.img` served repeatedly at 79,661 bytes. Serial evidence captured the accepted data-cache-enabled report, allocator plan/init reports, String smoke `ok=true`, and `talos: page frames remaining: start=0x2f010000 end=0x3fc00000 pages=0x10bf0 page_size=0x1000 source=bootstrap-reserve`.
- Rationale: This keeps extending the normal boot log from the proven post-data-cache formatter surface without changing memory-management behavior. The line is post-allocator and does not exercise the pre-data-cache formatter boundary that remains unaccepted.
- Risks: The report still describes a static bootstrap-reserve span, not a mutable physical page allocator. This does not add free/reuse, allocator expansion, page-frame-backed heap growth, MMIO exclusion ownership, high-memory allocation, DMA/cache ownership, lower-EL mappings, or userspace mappings.

## 2026-05-23 - DTB Memory Entry Println Accepted

- Status: accepted for the normal Pi 5 DTB memory entry report emitted through the ordinary Daedalus-like `println!` surface; DTB parsing, memory-bank accounting, page-frame reservation, and allocator policy remain unchanged.
- Context: The page-frames-remaining `println!` line had already accepted the post-data-cache, post-allocator formatter boundary. The DTB memory entry report runs immediately after that accepted line, so it was the next narrow normal boot-log candidate without revisiting the unaccepted pre-data-cache formatter boundary.
- Decision: Emit `talos: dtb memory[N]: addr=... size=...` through `println!` while preserving the existing DTB memory-bank scan, displayed-entry loop, allocator setup, cache/MMU behavior, and surrounding boot output.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 36 no_std tests, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, target string/header inspection, and `scripts/rpi5-archive-review.sh` passed. The accepted archive is `target/tmp/rpi5-dtb-memory-entry-println-20260523T1211Z-boot.tar.gz` with sha256 `88c45d169d0b88954e0a7aa8f2fbb48c3650e36c73783c4a835722a0cb295e4d`; the normal kernel is 79,677 bytes with sha256 `9c406f00932b87635198d6ff1ed875bf152f49b7fa7f1c4cc79e203365f88e47`.
- Hardware result: The lab published the archive and power-cycled the Pi 5. TFTP evidence in `target/tmp/rpi5-dtb-memory-entry-println-20260523T1217Z-evidence` showed fresh boot-file requests including the 79,677-byte `da591740/kernel_2712.img`. Serial evidence captured the accepted data-cache-enabled report, allocator plan/init reports, String smoke `ok=true`, page-frames-remaining report, and the three DTB memory entry `println!` lines.
- Rationale: This extends the normal boot log from the proven post-data-cache formatter surface into the memory-bank reporting tail without changing memory-management behavior. The line is post-allocator and avoids the unaccepted pre-data-cache formatter boundary.
- Risks: The report is still read-only DTB memory-bank logging. It does not add bank selection changes, free/reuse, allocator expansion, page-frame-backed heap growth, MMIO exclusion ownership, high-memory allocation, DMA/cache ownership, lower-EL mappings, or userspace mappings.

## 2026-05-23 - DTB Memory Summary Println Accepted

- Status: accepted for the normal Pi 5 DTB memory summary report emitted through the ordinary Daedalus-like `println!` surface; DTB parsing, memory-bank accounting, page-frame reservation, and allocator policy remain unchanged.
- Context: The DTB memory entry `println!` line had already accepted the post-data-cache, post-allocator reporting tail. The summary contains the same parsed DTB memory-bank metadata, but it previously ran before memory setup through formatter-free helpers. Moving only the summary report to the accepted tail avoids revisiting the unaccepted pre-data-cache `println!` boundary.
- Decision: Emit `talos: dtb memory: address_cells=... size_cells=... count=... shown=... truncated=...` through `println!` immediately before the existing DTB memory entry `println!` lines, while preserving DTB parsing, memory-bank selection, allocator setup, cache/MMU behavior, page-frame reservation policy, and entry reporting.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 36 no_std tests, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, target string/header inspection, and `scripts/rpi5-archive-review.sh` passed. The accepted archive is `target/tmp/rpi5-dtb-memory-summary-println-20260523T1228Z-boot.tar.gz` with sha256 `93a6975610612b06b88883aed829bf8555adbab93130ef3fa9b8322b0064acaa`; the normal kernel is 79,709 bytes with sha256 `10e73d9329fd81bfc68ac8853f9fa9f3dfc313ae3115f43ef55e6e2e954708df`.
- Hardware result: The lab published the archive and power-cycled the Pi 5. TFTP evidence in `target/tmp/rpi5-dtb-memory-summary-println-20260523T1228Z-evidence` showed `da591740/kernel_2712.img` served repeatedly at 79,709 bytes. A follow-up serial observe captured the accepted data-cache-enabled report, allocator plan/init reports, String smoke `ok=true`, page-frames-remaining report, `talos: dtb memory: address_cells=2 size_cells=2 count=3 shown=3 truncated=false`, and the three DTB memory entry `println!` lines.
- Rationale: This keeps broadening the normal human-readable boot log on the proven post-data-cache formatter surface without weakening the earlier cache-transition boundary or changing memory-management behavior.
- Risks: The summary is still read-only DTB memory-bank logging. It does not add bank selection changes, reserved-memory policy changes, free/reuse, allocator expansion, page-frame-backed heap growth, MMIO exclusion ownership, high-memory allocation, DMA/cache ownership, lower-EL mappings, or userspace mappings.

## 2026-05-23 - Bootstrap Reserve Post-Allocator Println Accepted

- Status: accepted for a post-allocator bootstrap-reserve report emitted through the ordinary Daedalus-like `println!` surface; the existing pre-data-cache formatter-free bootstrap-reserve diagnostic remains in place.
- Context: The page-frames-remaining and DTB memory reports had already accepted the post-data-cache, post-allocator formatter tail. The bootstrap-reserve accounting line was still formatter-free before translation-table layout. A first candidate that removed the early formatter-free copy and emitted only a later `println!` copy passed local gates and was TFTP-served as an 79,677-byte kernel, but fresh serial captures after power cycles showed only firmware/RP1 output. Treat that as non-acceptance rather than stale retained serial success.
- Decision: Preserve the original pre-data-cache formatter-free bootstrap-reserve line, and add a second post-allocator `println!` report immediately after the accepted String smoke and before the accepted page-frames-remaining line. This keeps early diagnostics stable while proving the same accounting line on the accepted formatter surface.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 36 no_std tests, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, target string/header inspection, and `scripts/rpi5-archive-review.sh` passed. The accepted archive is `target/tmp/rpi5-bootstrap-reserve-post-println-20260523T1248Z-boot.tar.gz` with sha256 `30426c4777d8975196702575b9b4eedff99ca01ea829a430f00478fa545f92ce`; the normal kernel is 80,069 bytes with sha256 `ece1e1db61f21474c34c9f2db32782d719f114989f30fcb04c6e2af6daba2144`.
- Hardware result: The lab published the archive and power-cycled the Pi 5. TFTP evidence in `target/tmp/rpi5-bootstrap-reserve-post-println-20260523T1248Z-evidence` showed repeated serves of `da591740/kernel_2712.img` at 80,069 bytes. Fresh serial evidence captured normal boot through the early formatter-free bootstrap-reserve line, cache enablement, allocator plan/init, String smoke, the post-allocator `talos: bootstrap reserve: start=0x2f000000 end=0x2f010000 pages=0x10 page_size=0x1000 reason=bootstrap-page-tables` line, page-frames-remaining, DTB memory summary, and DTB memory[0..2].
- Rationale: Duplicating the line is deliberate until the pre-data-cache formatter boundary is understood. It advances the normal `println!` surface without removing a useful early-memory diagnostic or changing memory-management behavior.
- Risks: The report still describes a fixed bootstrap reservation, not page-frame ownership transfer or allocator metadata policy. This does not add free/reuse, allocator expansion, page-frame-backed heap growth, MMIO exclusion ownership, high-memory allocation, DMA/cache ownership, lower-EL mappings, or userspace mappings.

## 2026-05-23 - Memory Usable Post-Allocator Println Accepted

- Status: accepted for a post-allocator memory-usable candidate report emitted through the ordinary Daedalus-like `println!` surface; the existing pre-data-cache formatter-free memory-usable diagnostic remains in place.
- Context: The bootstrap-reserve, page-frames-remaining, and DTB memory reports had already accepted the post-data-cache, post-allocator formatter tail. The memory-usable candidate was still only emitted before page-frame seed and translation-table layout, where formatter-free output remains the accepted policy.
- Decision: Preserve the original pre-data-cache formatter-free `talos: memory usable: ...` line, and add a second post-allocator `println!` report immediately after the accepted String smoke and before the accepted bootstrap-reserve/page-frames-remaining reports. This keeps early memory-selection diagnostics stable while proving the same low-tail candidate on the accepted formatter surface.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 36 no_std tests, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, target string/header inspection, and `scripts/rpi5-archive-review.sh` passed. The accepted archive is `target/tmp/rpi5-memory-usable-println-20260523T1256Z-boot.tar.gz` with sha256 `cb53e6537619a0bec40cedaa8ceb533fb93172b5bb8e28d78b29de7b559a377a`; the normal kernel is 80,293 bytes with sha256 `58b16229a0b5e3fe1fb4ffb4413cfe198f1037f9723ee1ecde51a5c3c1f789d2`.
- Hardware result: The lab published the archive and power-cycled the Pi 5. TFTP evidence in `target/tmp/rpi5-memory-usable-println-20260523T1256Z-evidence` showed `da591740/kernel_2712.img` served at 80,293 bytes. Serial evidence captured normal boot through the early formatter-free memory-usable line, cache enablement, allocator plan/init, String smoke, the post-allocator `talos: memory usable: bank=0 start=0x2f000000 end=0x3fc00000 size=0x10c00000 align=0x1000 policy=low-tail` line, bootstrap reserve, page-frames-remaining, DTB memory summary, and DTB memory[0..2].
- Rationale: Duplicating the line extends the normal human-readable `println!` boot log without removing an early diagnostic that still runs before the accepted formatter boundary.
- Risks: The report still describes the conservative low-tail candidate. It does not add a complete physical memory map, reserved-memory policy changes, allocator ownership transfer, free/reuse, allocator expansion, page-frame-backed heap growth, MMIO exclusion ownership, high-memory allocation, DMA/cache ownership, lower-EL mappings, or userspace mappings.

## 2026-05-23 - Page Frame Seed Post-Allocator Println Accepted

- Status: accepted for a post-allocator page-frame seed report emitted through the ordinary Daedalus-like `println!` surface; the existing pre-data-cache formatter-free page-frame seed diagnostic remains in place.
- Context: The memory-usable, bootstrap-reserve, page-frames-remaining, and DTB memory reports had already accepted the post-data-cache, post-allocator formatter tail. The page-frame seed line was still only emitted before bootstrap reservation and translation-table layout, where formatter-free output remains the accepted policy.
- Decision: Preserve the original pre-data-cache formatter-free `talos: page frames seed: ...` line, and add a second post-allocator `println!` report immediately after the accepted post-allocator memory-usable line and before the accepted bootstrap-reserve/page-frames-remaining reports. The post-allocator line includes `phase=post-allocator` to distinguish it from the early formatter-free copy in hardware captures.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 36 no_std tests, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, target string/header inspection, and `scripts/rpi5-archive-review.sh` passed. The accepted serial-prefixed archive is `target/tmp/rpi5-page-frame-seed-callsite-prefixed-20260523T142844Z-boot.tar.gz` with sha256 `9a5716719e709e3c7152609de65c97035cb95e19c8a589197459f716b5230126`; the normal kernel is 80,541 bytes with sha256 `059d1d879c8d7d5c26bbb02cdfa7b65412c256410b0fa520e9c6549c2fc44824`.
- Hardware result: A first accepted-control run `target/tmp/rpi5-page-frame-seed-control-alloc-format-recovery-20260523T144158Z-evidence` republished the accepted 76,152-byte alloc-format entry-marker archive and captured fresh `Starting OS`, BL31, `TALOS: rust_entry`, and `talos: alloc format smoke: ... ok=true`, proving the serial/handoff evidence path was live. The follow-up candidate run `target/tmp/rpi5-page-frame-seed-callsite-recovery-rerun-20260523T144337Z-evidence` TFTP-served `da591740/kernel_2712.img` twice at 80,541 bytes and serial captured normal boot through the early formatter-free memory-usable/page-frame/bootstrap lines, data-cache enablement, allocator plan/init, String smoke, the post-allocator memory-usable line, `talos: page frames seed: start=0x2f000000 end=0x3fc00000 pages=0x10c00 page_size=0x1000 source=memory-usable phase=post-allocator`, bootstrap reserve, page-frames-remaining, DTB memory summary, and DTB memory[0..2].
- Rationale: Duplicating the seed line extends the normal human-readable `println!` boot log without removing an early diagnostic that still runs before the accepted formatter boundary. The control-first rerun separates earlier no-Talos serial pickups from the page-frame call site itself.
- Risks: The report still describes the conservative low-tail page-frame seed, not mutable physical page allocator ownership. This does not change low-memory selection, bootstrap reservation sizing, translation-table layout, allocator setup, cache/MMU behavior, free/reuse, allocator expansion, MMIO exclusion ownership, high-memory allocation, DMA/cache ownership, lower-EL mappings, or userspace mappings.

## 2026-05-23 - Translation Table Layout Post-Allocator Println Accepted

- Status: accepted for a post-allocator translation-table layout report emitted through the ordinary Daedalus-like `println!` surface; the existing pre-data-cache formatter-free translation-table layout, slots, population, map-policy, and control-plan diagnostics remain in place.
- Context: The memory-usable, page-frame seed, bootstrap-reserve, page-frames-remaining, and DTB memory reports had already accepted the post-data-cache, post-allocator formatter tail. The translation-table layout line was still only emitted before MMU/cache transitions, where formatter-free output remains the accepted policy. Early candidate captures were inconclusive or stale-sensitive, so the accepted run used a serial read-loop after TFTP recovery instead of relying on retained tail text.
- Decision: Preserve the original pre-data-cache formatter-free `talos: translation tables: ... kind=layout-only` line, and add a second post-allocator `println!` report immediately after the accepted String smoke and before the accepted post-allocator memory/page-frame reports. The post-allocator line includes `phase=post-allocator` to distinguish it from the early formatter-free copy in hardware captures.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 36 no_std tests, `scripts/qemu-smoke.sh`, normal `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, target string/header inspection, and `scripts/rpi5-archive-review.sh` passed. The accepted archive is `target/tmp/rpi5-translation-table-layout-post-literal-20260523T151734Z-boot.tar.gz` with sha256 `2e2c59939acbf3612f8d3ccd3556b3158e4b77dc166ba55a5ea43e4f67f0fa08`; the normal kernel is 81,093 bytes with sha256 `786163b8f4d55a7f9782e28afd9ab3aa1e4e25a0c585d19db139c0cfe1808cc0`.
- Hardware result: A control run republished the accepted page-frame seed archive and recovered the normal surrounding boot output. The follow-up candidate run `target/tmp/rpi5-translation-table-layout-post-literal-readloop-20260523T153123Z-evidence` TFTP-served `da591740/kernel_2712.img` twice at 81,093 bytes and serial captured normal boot through data-cache enablement, allocator plan/init, String smoke, `talos: translation tables: start=0x2f000000 end=0x2f004000 pages=0x4 page_size=0x1000 kind=layout-only phase=post-allocator`, post-allocator memory-usable, page-frame seed, bootstrap reserve, page-frames-remaining, DTB memory summary, and DTB memory[0..2].
- Rationale: Duplicating the layout line extends the normal human-readable `println!` boot log without removing early translation-table diagnostics that still run before the accepted formatter boundary. The read-loop evidence separates earlier serial pickup artifacts from the candidate behavior.
- Risks: The report still describes the fixed four-page bootstrap translation-table staging area. This does not change table placement, population, MAIR/TCR/TTBR/SCTLR programming, cache/MMU behavior, allocator setup, low-memory selection, page-frame ownership, free/reuse, allocator expansion, MMIO exclusion ownership, high-memory allocation, DMA/cache ownership, lower-EL mappings, or userspace mappings.

## 2026-05-23 - Translation Table Slots Post-Allocator Println Accepted

- Status: accepted for a post-allocator translation-table slot-address report emitted through the ordinary Daedalus-like `println!` surface; the existing pre-data-cache formatter-free translation-table layout, slots, population, map-policy, and control-plan diagnostics remain in place.
- Context: The post-allocator translation-table layout line had just accepted the formatter tail immediately after the String smoke. The slot-address line reports the same fixed pages as the earlier formatter-free diagnostic, so it was the next narrow boot-log promotion without changing translation-table construction or cache/MMU programming.
- Decision: Preserve the original pre-data-cache formatter-free `talos: translation table slots: ...` line, and add a second post-allocator `println!` report immediately after the accepted post-allocator layout line and before the accepted memory/page-frame reports. The post-allocator line includes `phase=post-allocator` to distinguish it from the early formatter-free copy in hardware captures.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 36 no_std tests, `scripts/qemu-smoke.sh`, normal `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, target string/header inspection, and `scripts/rpi5-archive-review.sh` passed. The accepted archive is `target/tmp/rpi5-translation-table-slots-post-println-20260523T154311Z-boot.tar.gz` with sha256 `1d387af9e4a1db94a00365ff11edd527c4a1e62eedba8e369985cc4c217e1842`; the normal kernel is 81,517 bytes with sha256 `c43e72b76cd785eadaa4bc9c623345897ee8eca84ed5516e82a4309f1fa97790`.
- Hardware result: The lab published the archive and power-cycled the Pi 5. Corrected TFTP evidence in `target/tmp/rpi5-translation-table-slots-post-println-20260523T154311Z-evidence/summary-corrected.json` showed current-run serves of `da591740/kernel_2712.img` at 81,517 bytes at 15:47:06/07. The serial read-loop captured normal boot through data-cache enablement, allocator plan/init, String smoke, the post-allocator layout line, `talos: translation table slots: root=0x2f000000 l1=0x2f001000 l2_low=0x2f002000 l2_mmio=0x2f003000 phase=post-allocator`, post-allocator memory-usable, page-frame seed, bootstrap reserve, page-frames-remaining, DTB memory summary, and DTB memory[0..2].
- Rationale: Duplicating the slot line extends the normal human-readable `println!` boot log without removing early translation-table diagnostics that still run before the accepted formatter boundary.
- Risks: The report still describes the fixed translation-table slot addresses inside the bootstrap reservation. This does not change table placement, zeroing, population, MAIR/TCR/TTBR/SCTLR programming, cache/MMU behavior, allocator setup, low-memory selection, page-frame ownership, free/reuse, allocator expansion, MMIO exclusion ownership, high-memory allocation, DMA/cache ownership, lower-EL mappings, or userspace mappings.

## 2026-05-23 - Translation Table Population Post-Allocator Println Accepted

- Status: accepted for a post-allocator translation-table population report emitted through the ordinary Daedalus-like `println!` surface; the existing pre-data-cache formatter-free translation-table population, map-policy, and control-plan diagnostics remain in place.
- Context: The post-allocator translation-table layout and slot lines had just accepted the formatter tail immediately after the String smoke. The population line reports the already-built descriptor counts and block size, so it was the next narrow boot-log promotion without changing translation-table construction or cache/MMU programming.
- Decision: Preserve the original pre-data-cache formatter-free `talos: translation table population: ...` line, and add a second post-allocator `println!` report immediately after the accepted post-allocator slot line and before the accepted memory/page-frame reports. The post-allocator line includes `phase=post-allocator` to distinguish it from the early formatter-free copy in hardware captures.
- Local validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 36 no_std tests, `scripts/qemu-smoke.sh`, normal `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, `git diff --check`, target string/header inspection, and `scripts/rpi5-archive-review.sh` passed. The accepted archive is `target/tmp/rpi5-translation-population-post-println-20260523T1558Z-boot.tar.gz` with sha256 `5b099de0526119ae7fefe7a8e616790b2800237440ebf325c283d2740c2ccdb7`; the normal kernel is 82,045 bytes with sha256 `aa93a86a36ae942e410e11767dc75a657b2d02855fe6fa08249a0f4e2f07be0e`.
- Hardware result: A read-loop control run `target/tmp/rpi5-translation-population-control-slots-readloop-20260523T1558Z-evidence` first recovered fresh serial output for the accepted 81,517-byte slot image, proving the serial/handoff evidence path was live after initial short-observe windows saw only firmware text. The follow-up candidate run `target/tmp/rpi5-translation-population-post-println-readloop-20260523T1558Z-evidence` TFTP-served `da591740/kernel_2712.img` at 82,045 bytes and serial captured normal boot through data-cache enablement, allocator plan/init, String smoke, the post-allocator layout and slot lines, `talos: translation table population: root_entries=0x1 l1_entries=0x2 low_l2_blocks=0x200 mmio_l2_blocks=0x20 block_size=0x200000 kind=stage1-4k-no-enable phase=post-allocator`, post-allocator memory-usable, page-frame seed, bootstrap reserve, page-frames-remaining, DTB memory summary, and DTB memory[0..2].
- Rationale: Duplicating the population line extends the normal human-readable `println!` boot log without removing early translation-table diagnostics that still run before the accepted formatter boundary. The control-first read-loop separates earlier short-observe firmware-only captures from the candidate behavior.
- Risks: The report still describes the existing populated table skeleton; it does not repopulate entries or change map policy. This does not change table placement, zeroing, population behavior, MAIR/TCR/TTBR/SCTLR programming, cache/MMU behavior, allocator setup, low-memory selection, page-frame ownership, free/reuse, allocator expansion, MMIO exclusion ownership, high-memory allocation, DMA/cache ownership, lower-EL mappings, or userspace mappings.

## 2026-05-23 - Stale Pi 5 Diagnostic Surface Retired

- Status: accepted as a maintainability cleanup. No normal Pi 5 boot output, MMU/cache programming, allocator policy, FDT interpretation, or hardware-facing normal boot behavior intentionally changed.
- Context: The maintainability review accepted the main/boot, diagnostic, memory_map, and device_tree module boundaries but found the repo still advertised many historical Pi 5 probes through wrapper scripts and build.rs env/cfg plumbing. Those probes came from first-light and early serial boundary work, while the accepted normal path now has UART10, Rust-entry, early phase, formatter pacing, post-data-cache println, allocator, memory/page-frame/translation-table, and DTB report evidence.
- Decision: Delete stale loader/armstub/EFI alternatives, UART proof trees, entry/fresh-entry/candidate serial probes, transition/text/vector/fallthrough/post-stack probes, direct assembly reset/BRK/BTI classifier probes, boundary-entry reset, and phase-ladder reset wrapper scripts. Remove matching build.rs env/cfg advertisement and delete stale target/rpi5.rs runtime UART, handoff UART, rust-uart10, boundary-entry reset, and phase-ladder entry points. Keep only current normal Pi 5 gates plus allocator, panic, exception/fault, and translation-fault diagnostic wrappers.
- Evidence level: static stale-script/cfg inventory, no_std unit tests, QEMU substitute, normal Pi 5 image generation, format-guard build, representative retained diagnostic image builds, and git diff checks.
- Validation: cargo fmt --all -- --check, cargo -Zjson-target-spec test with 41 no_std tests, scripts/qemu-smoke.sh, scripts/rpi5-image.sh, scripts/rpi5-format-guard-check.sh, representative retained diagnostic builds for panic report, normal exception report, translation fault, alloc OOM, and alloc format, and git diff --check passed. mdbook build was not run because mdbook is unavailable in the container.
- Rationale: Keeping one-off proof wrappers after their facts are accepted makes stale experiments look like supported kernel interfaces. The accepted facts belong in architecture docs, decision records, and reusable tests/gates; old probe wrappers should not stay runnable by accident.
- Risks: Dormant historical conditional blocks still exist inside boot.S, but no retained script or build.rs path advertises or enables their flags. A future cleanup can remove unreachable assembly bodies if it is worth the diff risk; this task removed the active repository surface future workers would otherwise run.

## 2026-05-23 - Phase 3 Runtime Inventory Checkpoint

- Status: accepted as a documentation checkpoint only. No kernel/runtime code, boot image, hardware lock, or hardware run changed in this task.
- Context: After the post-allocator translation-table population report was accepted and committed at `d3be399`, the supervisor queued a Phase 3 closeout-oriented inventory before more implementation work. The goal was to reconcile accepted Pi 5/QEMU evidence with current memory/MMU/allocator contracts and to keep remaining gaps explicit.
- Decision: Add a Phase 3 runtime inventory checkpoint to `docs/src/architecture/memory.md` that distinguishes accepted capabilities from deferred work, labels each accepted area by evidence level, cites current evidence directories and commit `d3be399`, and lists the supervisor-owned next implementation backlog.
- Evidence level: static documentation inspection plus git diff checks. The checkpoint references prior Pi 5 serial/TFTP hardware evidence, local/QEMU gates, and diagnostic-only evidence; it does not create new hardware evidence.
- Validation: `git status --short` was recorded before and after docs changes. `git diff --check` passed. `mdbook build` was not run because `mdbook` is unavailable in the container. Rust formatting and tests were not required because no Rust files changed.
- Rationale: Phase 3 now has enough accepted runtime/memory surface area that the next worker should not infer direction from scattered historical entries. The inventory makes page-frame ownership, free/reuse, heap/OOM policy, high-memory/DMA/cache boundaries, and lower-EL/userspace mapping readiness explicit follow-up tasks without declaring Phase 3 complete.
- Risks: The checkpoint is not a runtime feature and does not close Phase 3. It intentionally does not claim free/reuse allocation, heap expansion, high-memory ownership, DMA-safe memory, driver cache coherency, lower-EL mappings, userspace mappings, networking, or shell readiness.

## 2026-05-23 - Page-Frame Ownership Contract Accepted

- Status: accepted for a code-level Phase 3 page-frame ownership contract with local/QEMU evidence only. No normal Pi 5 boot output, boot archive, hardware lock, or hardware run changed in this task.
- Context: The prior Phase 3 runtime inventory identified page-frame ownership as the next closeout slice after the accepted low-tail, bootstrap-reserve, translation-table, allocator, and DTB memory reports. The goal was to name ownership boundaries before implementing free/reuse or heap expansion.
- Decision: Add `early_page_frame_ownership_contract` in `src/memory_map.rs`. The contract derives from the accepted conservative low-tail seed, 16-page bootstrap reservation, four-page translation-table layout, and no-free bootstrap allocator plan. It names `bootstrap-reserved`, `bootstrap-reserved-unused`, `bootstrap-bump-owned`, and `outside-conservative-low-tail` so later free/reuse work has explicit inputs instead of inferring ownership from boot logs.
- Evidence level: static code inspection, no-std unit tests, QEMU substitute, normal Pi 5 image generation, and format-guard build. The focused tests are `page_frame_ownership_contract_names_current_low_tail_partitions`, `page_frame_ownership_contract_rejects_mismatched_allocator_span`, and `page_frame_ownership_contract_excludes_kernel_dtb_reservations_stack_and_tables`.
- Validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 39 no-std tests, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, and `git diff --check` passed. `mdbook build` was not run because `mdbook` is unavailable in the container.
- Rationale: The contract makes the current frame partitions auditable without changing allocator behavior. Tests prove the accepted low-tail span excludes kernel/runtime ranges, DTB/FDT reservation inputs, boot stack ranges, bootstrap table pages, and the allocator-owned span from each other.
- Risks: This is metadata and static checking only. It does not implement free/reuse, place mutable page allocator metadata, expand the heap, add recoverable OOM, claim high-memory or DMA-safe ownership, alter MMU/cache programming, add lower-EL mappings, or change normal Pi 5 serial output.

## 2026-05-23 - Bounded Page-Frame Reuse Diagnostic Accepted

- Status: accepted for a bounded page-frame allocate/free/reallocate diagnostic with local/QEMU/image evidence only. No normal Pi 5 boot output, boot archive, hardware lock, or hardware run changed in this task.
- Context: The accepted page-frame ownership contract named `bootstrap-bump-owned` but intentionally did not prove mutable frame reuse. The next Phase 3 slice needed a tiny reusable-frame model without converting the Rust global allocator or widening memory ownership.
- Decision: Add `early_page_frame_reuse_allocator` under `src/memory_map/page_frames.rs`. It manages an explicit tracked window inside the accepted allocator-owned span using caller-provided metadata, rejects metadata that intersects managed frames, allocates 4 KiB frames, accepts a free/reallocate cycle, and rejects double-free, unaligned, and out-of-range frees. Add the cfg-gated `TALOS_RPI5_PAGE_FRAME_REUSE_DIAGNOSTIC` and `scripts/rpi5-page-frame-reuse-diagnostic-image.sh` so the Pi 5 path can exercise the same primitive after the bootstrap allocator plan is known, while leaving normal boot and the global heap unchanged.
- Evidence level: static code inspection, no-std unit tests, QEMU substitute, normal Pi 5 image generation, cfg-gated diagnostic image generation, format-guard build, and git diff checks. The focused tests are `page_frame_reuse_allocator_allocates_frees_and_reuses_frame`, `page_frame_reuse_allocator_rejects_double_free_and_out_of_range_frame`, and `page_frame_reuse_allocator_requires_metadata_outside_managed_frames`.
- Validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 44 no-std tests, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-page-frame-reuse-diagnostic-image.sh`, `scripts/rpi5-format-guard-check.sh`, and `git diff --check` passed. `mdbook build` was not run because `mdbook` is unavailable in the container.
- Rationale: A tracked-window diagnostic proves the minimal free/reuse mechanics needed for later heap-policy work without pretending the current no-free bump allocator has become a general physical allocator. Keeping metadata caller-owned and outside the managed frame range makes the ownership handoff visible instead of implicit.
- Risks: This does not implement heap deallocation, heap expansion, recoverable OOM, high-memory allocation, DMA-safe frame policy, page-fault recovery, lower-EL mappings, userspace, networking, or Phase 4 work. The normal no-diagnostic Pi 5 path still initializes `KERNEL_GLOBAL_ALLOCATOR` as a no-free bump allocator.

## 2026-05-23 - Main Entry And Pi 5 Diagnostics Refactor Accepted

- Status: accepted as a behavior-preserving maintainability refactor with local/QEMU/image evidence only. No normal Pi 5 boot output, boot archive, hardware lock, or hardware run changed in this task.
- Context: The maintainability audit found src/main.rs was carrying rust_entry, the normal Pi 5 boot pipeline, report formatting, allocator smoke tests, retained hardware diagnostics, and stale one-off probes. The cleanup policy was deletion-first: retain only diagnostics with a current boot-probe, regression-test, or validation-gate purpose.
- Decision: Delete stale rust-entry reset, formatter/rodata, static-format-boundary, and stack/asm-to-Rust reset probes and their scripts/cfg plumbing. Split retained code so src/main.rs keeps top-level entry, panic/OOM handling, QEMU smoke entry, and tests; src/boot/rpi5.rs owns the normal Pi 5 boot pipeline; src/boot/rpi5_reports.rs owns normal Pi 5 report helpers; and src/diagnostics/rpi5.rs owns retained allocator, translation-fault, exception/fault, and panic diagnostic bodies.
- Evidence level: static cleanup inspection, no-std unit tests, QEMU substitute, normal Pi 5 image generation, format-guard build, representative retained diagnostic image builds, and git diff checks. The representative diagnostic image builds covered panic, normal exception, translation fault, and alloc OOM paths after the module split.
- Validation: cargo fmt --all -- --check, cargo -Zjson-target-spec test with 39 no-std tests, scripts/qemu-smoke.sh, scripts/rpi5-image.sh, scripts/rpi5-format-guard-check.sh, scripts/rpi5-panic-report-diagnostic-image.sh, scripts/rpi5-normal-exception-report-diagnostic-image.sh, scripts/rpi5-translation-fault-diagnostic-image.sh, scripts/rpi5-alloc-oom-diagnostic-image.sh, and git diff --check passed. mdbook build was not run because mdbook is unavailable in the container.
- Rationale: The refactor separates normal boot orchestration from retained diagnostic bodies and removes stale probes instead of preserving them behind nicer names. This keeps future Phase 3 memory work from reusing obsolete bring-up diagnostics as implicit policy.
- Risks: This does not change memory/MMU/cache/allocator behavior, normal Pi 5 serial line order, or diagnostic acceptance criteria. It does not implement page-frame free/reuse, heap expansion, high-memory ownership, DMA/cache ownership, lower-EL mappings, userspace, networking, or shell work.

## 2026-05-23 - Maintainability Review Requires Target Diagnostic Deletion Pass

- Status: accepted as a review checkpoint decision, not as the end of the cleanup sequence. No kernel/runtime code, boot image, hardware lock, or hardware run changed in this checkpoint.
- Context: After the main/boot diagnostic split and memory/FDT module split, src/main.rs is reduced to top-level entry/panic/QEMU/test ownership, retained diagnostic bodies live in src/diagnostics/rpi5.rs, normal Pi 5 boot orchestration lives in src/boot/rpi5.rs, and memory/FDT responsibilities are visible under src/memory_map/ and src/device_tree/. The review still found many stale Pi 5 bring-up wrappers and some still-wired target helpers from the audit's document-and-delete/delete-as-stale families.
- Decision: Keep the new module boundaries, but do not resume Phase 3 feature work yet. Queue one deletion-focused task, phase3-target-rpi5-diagnostic-deletion-20260523, before page-frame free/reuse or other Phase 3 closeout work. That task owns src/target/rpi5.rs, build.rs diagnostic cfg/env plumbing, and stale scripts/rpi5-* diagnostic/proof wrappers; it should keep only current boot/format gates and still-useful allocator, exception/fault, panic, and translation-fault diagnostics.
- Evidence level: static inspection of Rust file sizes, module paths, cfg inventory, and script inventory.
- Validation: cargo fmt --all -- --check, cargo -Zjson-target-spec test with 41 no-std tests, scripts/qemu-smoke.sh, scripts/rpi5-image.sh, scripts/rpi5-format-guard-check.sh, and git diff --check passed for this docs/state checkpoint. mdbook build was not run because mdbook is unavailable in the container.
- Rationale: The first two refactors fixed the largest catch-all files but did not finish the deletion-first policy. Leaving stale loader/armstub/EFI, UART proof, entry/transition/vector/text, direct assembly reset, boundary-entry, and phase-ladder probes in the normal repository shape would invite future workers to treat historical bring-up experiments as active kernel interfaces.
- Risks: This checkpoint does not itself delete those probes and does not claim the cleanup sequence is complete. The next cleanup task must avoid normal boot-output changes unless it captures hardware evidence; it must not implement page-frame free/reuse, heap expansion, high-memory/DMA, lower-EL/userspace, networking, or Phase 4 work.

## 2026-05-23 - Recoverable OOM And Heap Expansion Policy Accepted

- Status: accepted for a bounded recoverable allocation-failure API and heap-expansion policy with local/QEMU/image evidence only. No normal Pi 5 boot output, boot archive, hardware lock, or hardware run changed in this task.
- Context: The accepted page-frame ownership contract and bounded reuse diagnostic named the low-tail allocator-owned span and proved a tiny reusable-frame model, but the global heap still only had fatal `alloc_error_handler` behavior for infallible `alloc`-crate allocation failure.
- Decision: Add `BumpAllocator::try_allocate_layout` as the explicit recoverable direct-allocation API. Add `early_heap_expansion_policy` to bind any future early heap growth to the accepted `bootstrap-bump-owned-low-tail` source while protecting the bootstrap reservation and translation-table pages. Add the cfg-gated `TALOS_RPI5_HEAP_EXPANSION_POLICY_DIAGNOSTIC` and `scripts/rpi5-heap-expansion-policy-diagnostic-image.sh` so the Pi 5 path can report the policy and prove an oversized fallible request returns a typed exhaustion error without advancing `next`.
- Evidence level: static code inspection, no-std unit tests, QEMU substitute, normal Pi 5 image generation, cfg-gated diagnostic image generation, format-guard build, and git diff checks. The focused tests are `fallible_allocation_reports_exhaustion_without_advancing`, `heap_expansion_policy_uses_allocator_owned_low_tail_and_protects_reserved_frames`, and `heap_expansion_policy_rejects_non_allocator_owned_source`.
- Validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 47 no-std tests, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-heap-expansion-policy-diagnostic-image.sh`, `scripts/rpi5-format-guard-check.sh`, and `git diff --check` passed. `mdbook build` was not run because `mdbook` is unavailable in the container.
- Rationale: Recoverable OOM should be an explicit caller choice, not an accidental change to infallible `alloc` behavior. The current policy names the only accepted frame source and proves failure does not mutate allocator state, while preserving the fatal OOM diagnostic for normal `alloc`-crate failure.
- Risks: This does not install dynamic page-frame-backed heap growth, make `Vec`/`String` allocation failure recoverable, add object deallocation, claim high memory or DMA-safe frames, add page-fault recovery, lower-EL mappings, userspace, networking, or Phase 4 work. The normal no-diagnostic Pi 5 path still initializes `KERNEL_GLOBAL_ALLOCATOR` as a no-free bump allocator.

## 2026-05-23 - High-Memory DMA And Cache Boundary Accepted

- Status: accepted as a documentation boundary for Phase 3 closeout. No kernel/runtime code, normal Pi 5 boot output, boot image, hardware lock, or hardware run changed in this task.
- Context: The accepted page-frame ownership, bounded free/reuse diagnostic, and heap-expansion policy all use the same low identity-mapped allocator-owned span. Before Phase 4 interrupt/timer work, Talos needed to make explicit that DTB discovery of larger memory banks and the accepted data-cache-enabled status do not imply high-memory or DMA readiness.
- Decision: Document the accepted allocation source as `0x2f010000..0x3fc00000` inside low bank 0, and explicitly leave bank 1 `0x40000000..0x100000000` and bank 2 `0x100000000..0x200000000` unowned by current allocation policy. Document the cache-enabled state as an early-kernel execution boundary only, with DMA-safe buffers, RP1/PCIe addressability, `dma-ranges`/IOMMU policy, cache-maintenance APIs, cacheable/non-cacheable DMA mapping rules, and driver-pressure allocator metadata ownership deferred.
- Evidence level: static documentation inspection plus git diff checks. This task intentionally added no code guards because the existing ownership and heap policy code already rejects sources outside the accepted low-tail span.
- Validation: `git diff --check` passed. `mdbook build` was not run because `mdbook` is unavailable in the container. Rust formatting and tests were not required because no Rust files changed.
- Rationale: Phase 4 can use ordinary early-kernel allocations, but should not accidentally depend on unaccepted high-memory mapping, DMA-safe allocation, or driver cache-coherency behavior. Keeping the boundary in the architecture doc prevents later driver or interrupt work from treating a boot log or DTB bank as an ownership handoff.
- Risks: This does not map high memory, implement DMA APIs, add cache-maintenance routines, change allocator behavior, retarget work to RP1/PCIe/networking, or close Phase 3.

## 2026-05-23 - Lower-EL And Userspace Mapping Readiness Boundary

- Status: accepted as a documentation boundary for Phase 3 closeout. No kernel/runtime code, normal Pi 5 boot output, boot image, hardware lock, or hardware run changed in this task.
- Context: The current Pi 5 kernel has hardware evidence for an EL2 stage-1 identity map, cache-enabled early execution, and same-EL exception diagnostics. Phase 7 will eventually need EL0, syscalls, and user address spaces, but Phase 3 closeout must not let the current identity map masquerade as userspace isolation.
- Decision: Document the current map as an EL2 kernel bring-up map only. It proves the kernel can run with low DRAM and the BCM2712 local-peripheral MMIO window identity-mapped, but it does not prove user/kernel permissions, lower-EL trap return, user stack/heap mappings, syscall ABI, or invalid-user-memory handling.
- Consequences: Phase 4 may proceed using the current EL2 kernel map for kernel execution, but no task may treat it as permission to enter EL0, run untrusted payloads, or expose MMIO/kernel memory to user code. EL0 work remains gated on explicit address-space shape, descriptor permissions, TTBR/TCR/SCTLR policy, lower-EL exception routing, copy-in/copy-out, and bad-pointer tests.
- Validation: Documentation-only readiness audit. `git diff --check` passed; `mdbook build` was unavailable; Rust fmt/tests and Pi 5 hardware were not required because no code or normal boot behavior changed.
- Rationale: A dedicated readiness boundary keeps Phase 7 prerequisites explicit before interrupt/timer work resumes, without starting userspace implementation or overloading the Phase 3 closeout checkpoint.
- Risks: This does not enter EL0, implement syscalls, create process address spaces, add file descriptors, change exception-vector behavior, change translation-table contents, or close Phase 3.

## 2026-05-23 - Phase 3 Closeout Checkpoint Accepted

- Status: accepted as the explicit go/no-go checkpoint before Phase 4 planning. No kernel/runtime code, normal Pi 5 boot output, boot image, hardware lock, or hardware run changed in this task.
- Context: Phase 3 now has accepted evidence for the current memory/MMU/runtime boundary, maintainability cleanup, page-frame ownership, a bounded free/reuse diagnostic, recoverable direct-allocation failure, heap-expansion source policy, high-memory/DMA/cache deferrals, and lower-EL/userspace readiness. The supervisor queue requires an explicit checkpoint before any interrupt/timer/preemption task starts.
- Decision: Close Phase 3 for the current boundary and recommend Phase 4 planning next. The accepted checkpoint is documented in `docs/src/project/phase3-closeout-checkpoint.md` and linked from the roadmap and mdBook summary.
- Consequences: The supervisor should plan bounded Phase 4 interrupt-controller discovery and timer interrupt work. Those tasks may use the accepted EL2 kernel map and low-tail allocation boundary, but must not rely on high-memory allocation, DMA-safe buffers, lower-EL isolation, process address spaces, or a free/reuse global heap unless a new task explicitly designs and validates that dependency.
- Validation: Documentation-only checkpoint. `git status --short` was inspected before edits, `git diff --check` passed after edits, and `mdbook build` was unavailable. Rust fmt/tests, QEMU, Pi 5 image builds, and hardware were not required because only docs and durable task state changed.
- Rationale: A dedicated checkpoint prevents Phase 4 from inheriting hidden Phase 3 assumptions and gives the supervisor a clear planning handoff rather than letting the worker invent the next phase queue.
- Risks: This does not implement interrupts, timers, preemption, SMP, lower-EL entry, userspace, dynamic heap growth, DMA/cache APIs, filesystems, networking, or SSH.

## 2026-05-24 - Phase 4 Interrupt Timer Source Inventory Accepted

- Status: accepted as a source inventory and implementation checklist for Phase 4 interrupt/timer bring-up. No Rust code, normal boot output, boot image, hardware lock, or hardware run changed in this task.
- Context: Phase 3 closeout recommended Phase 4 planning before implementation. The first Phase 4 queue item needed exact target topology for QEMU virt and Pi 5 rather than letting driver work infer values from the roadmap.
- Decision: Add docs/src/architecture/interrupts-timers.md and link it from the architecture index and mdBook summary. The note accepts QEMU virt GICv2 as arm,cortex-a15-gic with distributor 0x0800_0000 and CPU interface 0x0801_0000, and Pi 5 GIC-400/GICv2 as arm,gic-400 with distributor 0x10_7fff9000 and CPU interface 0x10_7fffa000 through the soc@107c000000 ranges mapping. It records the ARM generic timer PPIs for both targets and selects the EL2 hypervisor physical timer, PPI 10 / INTID 26, as the first QEMU and Pi 5 timer smoke target.
- Evidence level: static source inspection and generated/local DTB inspection. Sources included QEMU v9.2.0 hw/arm/virt.c, a QEMU-generated virt GICv2 DTB, Raspberry Pi Linux rpi-6.12.y bcm2712 DTS/DTSI files, a lab-staged Pi 5 bcm2712-rpi-5-b.dtb, Linux GIC and arch-timer devicetree bindings, Linux GICv2 register offsets, and Linux ARM architected timer driver behavior.
- Validation: git status --short was inspected before edits. git diff --check and git diff --cached --check passed after edits. mdbook build was not run because mdbook is unavailable in the container. Rust fmt/tests, QEMU smoke, Pi 5 image builds, and hardware were not required because this task changed only documentation and durable task state.
- Rationale: The next worker can implement the IRQ frame contract and first timer smokes against named bases, PPI numbers, DTB evidence, and deferred uncertainties instead of broad Phase 4 intent.
- Risks: This does not enable interrupts, program the GIC, program the timer, define scheduler policy, prove PPI polarity behavior on Pi 5, add UART interrupts, route RP1/PCIe interrupts, support SMP, or change lower-EL/user timer policy.

## 2026-05-24 - Phase 4 IRQ Entry Frame Contract Accepted

- Status: accepted as an inert production IRQ entry/return contract for QEMU virt and Pi 5. Interrupts remain disabled; no GIC, timer, normal boot-output, boot archive, hardware lock, or hardware run changed in this task.
- Context: The next Phase 4 tasks need a state-preserving vector path before they can enable the generic timer IRQ. Existing Pi 5 synchronous exception diagnostics already had a saved `x0..x30` frame, but QEMU and normal current-EL IRQ slots did not have a production Rust dispatch/return contract.
- Decision: Route normal vector slots through the shared saved-frame entry shim. The shim saves interrupted `x0..x30` in `ExceptionFrame`; IRQ vector slots call `rust_irq_handler(vector, elr, spsr, frame)`; the shim then restores the full frame and returns with `ERET`. The Rust stub records an unexpected-IRQ count plus last vector/ELR/SPSR with atomics and deliberately performs no allocation, formatting, controller acknowledgement, timer programming, or unmasking.
- Evidence level: static code inspection, no-std unit tests for IRQ vector classification and dispatch context recording, QEMU substitute, normal Pi 5 image generation, format-guard build, representative retained exception/panic diagnostic image builds, and git diff checks.
- Validation: cargo fmt --all -- --check, cargo -Zjson-target-spec test, scripts/qemu-smoke.sh, scripts/rpi5-image.sh, scripts/rpi5-format-guard-check.sh, representative retained exception/panic diagnostic image scripts, and git diff --check passed. mdbook build was not run because mdbook is unavailable in the container.
- Rationale: The timer tasks can now consume a narrow IRQ entry contract without mixing it with GIC setup, timer setup, scheduler policy, or hardware evidence. Keeping the stub inert prevents an unexpected IRQ from relying on logging or controller state before those responsibilities exist.
- Risks: This does not enable external IRQ delivery, acknowledge/EOI GIC interrupts, program CNTHP_*_EL2, prove Pi 5 PPI behavior, support nested IRQs/FIQ/SError, lower-EL IRQs, preemption, context switching, SMP routing, or interrupt-safe locking.

## 2026-05-24 - QEMU EL2 Timer IRQ Smoke Accepted

- Status: accepted as the first QEMU-only interrupt-driven timer proof for Phase 4. No Pi 5 boot archive, hardware lock, or hardware run changed in this task.
- Context: The accepted IRQ frame contract could save and restore `x0..x30`, but no task had yet enabled a GIC interrupt, programmed the generic timer, or proven return from a real IRQ. The default QEMU smoke boots at EL1, so the focused timer smoke needed an explicit EL2 QEMU run.
- Decision: Add a small GICv2 MMIO surface for the distributor and CPU interface, a minimal EL2 generic-timer helper for `CNTHP_*_EL2`, and `scripts/qemu-timer-irq-smoke.sh` running `-M virt,gic-version=2,virtualization=on`. The diagnostic sets `HCR_EL2.IMO`, enables GICv2 PPI 10 / INTID 26, programs `CNTHP_CVAL_EL2`, unmasks IRQs, and handles vector 5/current-SPx IRQ by reading `GICC_IAR`, recognizing INTID 26, masking the timer, writing `GICC_EOIR`, incrementing bounded atomics, and returning through the saved frame.
- Evidence level: static code inspection, no-std unit tests, QEMU EL1 substitute smoke, QEMU EL2 timer-interrupt smoke, normal Pi 5 image generation, Pi 5 format-guard build, and git diff checks.
- Validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 51 no-std tests, `scripts/qemu-smoke.sh`, `scripts/qemu-timer-irq-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, and `git diff --check` passed. `mdbook build` was not run because `mdbook` is unavailable in the container. The timer smoke log showed EL2 boot, `gicd=0x08000000`, `gicc=0x08010000`, `intid=26`, `irq-count=1`, `vector=5`, `iar=0x0000001a`, `unexpected=0`, `ctl=0x2`, no pending/active GIC bit after EOI, continued post-IRQ workload, and `qemu-timer-irq-smoke: PASS`.
- Rationale: The QEMU proof keeps the first interrupt delivery loop small and repeatable before carrying the same shape to Pi 5 hardware. Setting `HCR_EL2.IMO` is part of the EL2 diagnostic contract because QEMU otherwise shows the timer and GIC pending state without vectoring to EL2.
- Risks: This does not prove Pi 5 GIC-400 PPI polarity/group behavior, UART interrupts, SPIs, MSI, cascaded interrupt controllers, RP1/PCIe, DMA, SMP routing, nested IRQs, monotonic tick policy, preemption, scheduler work, lower-EL timer routing, or userspace timer access.

## 2026-05-24 - Validation Hygiene Cleanup Accepted

- Status: accepted as a maintainability remediation checkpoint before resuming Phase 4 feature work. No normal boot output, Pi 5 boot archive, hardware lock, or hardware run changed in this task.
- Context: The senior review found validation hygiene drift after the first timer-smoke implementation: `cargo fmt --check` had previously reported formatting drift, and broad module-level `dead_code` allowances in `generic_timer` and `gicv2` could hide avoidable warning noise while those modules become active Phase 4 surfaces.
- Decision: Remove the module-level `cfg_attr(..., allow(dead_code))` allowances from `src/arch/aarch64/generic_timer.rs` and `src/arch/aarch64/gicv2.rs`. The current QEMU and Pi 5 builds use enough of those modules that no replacement dead-code allowance is needed. Retained `dead_code` allowances elsewhere remain outside this task's scope.
- Evidence level: static inspection, fmt/lint/typecheck, no-std unit tests, QEMU substitute, normal Pi 5 image generation, format-guard build, and git diff checks.
- Validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 51 no-std tests, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, and `git diff --check` passed. A clean rebuild after removing the allowances produced no unexpected `generic_timer` or `gicv2` dead-code warnings.
- Rationale: Warning hygiene should stay local to the unfinished surface that needs it. Removing broad module allowances keeps Phase 4 interrupt/timer code honest without changing the GIC or timer behavior.
- Risks: This does not implement Pi 5 timer IRQ delivery, monotonic ticks, interrupt masking policy, scheduler work, or cleanup of stale Pi 5 proof/probe surfaces.

## 2026-05-24 - Stale Pi 5 Assembly Probes Deleted

- Status: accepted as the second maintainability remediation checkpoint. No normal Pi 5 boot behavior, allocator policy, exception semantics, MMU/cache programming, GIC/timer behavior, boot archive, hardware lock, or hardware run intentionally changed.
- Context: The 2026-05-23 stale diagnostic cleanup removed active wrapper and build.rs surfaces, but senior review found dormant historical probe bodies still present in `src/arch/aarch64/boot.S`, old standalone Pi 5 proof assembly files still tracked, and `scripts/rpi5-archive-review.sh` still accepting deleted loader/proof modes.
- Decision: Replace `boot.S` with only the supported normal assembly entry path: arm64 Image header, `x0` preservation, CPACR enable, BSS clear, stack setup, and `rust_entry` handoff. Delete standalone `src/arch/aarch64/rpi5_*.S` loader/armstub/EFI/UART/reset proof files. Keep only the retained current assembly diagnostics in `vectors.S`: `TALOS_RPI5_EXCEPTION_REPORT_DIAGNOSTIC` and `TALOS_RPI5_EXCEPTION_RETURN_DIAGNOSTIC`. Tighten `rpi5-archive-review.sh` so it recognizes only `raw-pi5` and `raw-pi5-circle-config` as loader diagnostic modes and no longer accepts `asm-uart-proof`, `asm-entry-reset-proof`, `cargo-asm-uart-proof`, `transition-diagnostic`, or firmware-address proof variants.
- Evidence level: static stale-flag/proof-file inventory, fmt/lint/typecheck, no-std unit tests, QEMU substitute, normal Pi 5 image/archive inspection, format-guard build, representative retained diagnostic image builds, shell syntax inspection, and git diff checks.
- Validation: see `tasks/2026-05-24-maintainability-delete-stale-pi5-probes.md` for the before/after inventory and gate output.
- Rationale: Historical one-off probes are still useful as decision-log evidence, but leaving them in runnable assembly or archive-validation paths makes them look like supported kernel interfaces. The current repository surface should expose normal Pi 5 boot plus actively retained allocator, panic, exception/fault, translation-fault, page-frame, and heap-policy diagnostics.
- Risks: This deliberately removes the ability to rebuild old proof binaries from current source. Reproducing those historical experiments now requires checking out the commits named in their task records or decision entries.

## 2026-05-24 - Pi 5 Boot Pipeline Phase Helpers

- Status: accepted as the third maintainability remediation checkpoint. No normal Pi 5 serial contract, memory/MMU/cache policy, allocator behavior, diagnostic routing, boot archive, hardware lock, or hardware run intentionally changed.
- Context: Senior review found that `src/boot/rpi5.rs` had grown into one deeply nested `kernel_main` spanning DTB reporting, memory selection, page-frame reservation, translation-table setup, MMU/cache enable, allocator initialization, diagnostics, and post-allocator reports. The normal boot path was evidence-backed, but the structure made phase ownership and failure reporting difficult to audit.
- Decision: Keep `kernel_main` as the ordered boot phase list and move the work into named helpers: `report_boot_identity`, `plan_boot_memory`, `enable_translation_and_caches`, `init_bootstrap_allocator`, `report_post_allocator_memory`, and `report_dtb_memory_banks`. Keep failure paths serial-visible through a shared `report_unavailable` helper. Centralize the repeated Vec/String/alloc-format diagnostic output-suppression predicate as `suppress_growth_diagnostic_boot_reports`.
- Evidence level: static structure inspection, fmt/lint/typecheck, no-std unit tests, QEMU substitute, normal Pi 5 image generation, format-guard build, representative retained diagnostic image builds, and git diff checks.
- Validation: see `tasks/2026-05-24-maintainability-flatten-pi5-boot-pipeline.md` for the before/after structure summary and gate output.
- Rationale: The refactor preserves the current Pi 5 serial/output contract while making each boot phase invariant and failure line explicit enough for Phase 4 work to continue without navigating a 500-line nested body.
- Risks: This does not prove unchanged hardware output with a fresh Pi 5 run because the task intentionally made no hardware-facing behavior change. The next hardware task should treat any serial-order discrepancy as a regression against the accepted normal boot contract.

## 2026-05-24 - Maintainability Remediation Checkpoint

- Status: accepted as the closeout checkpoint for the senior-review remediation sequence that paused Phase 4 feature work. No runtime feature, normal Pi 5 boot behavior, GIC/timer behavior, allocator policy, MMU/cache programming, boot archive, hardware lock, or hardware run intentionally changed in this checkpoint.
- Context: The review required four cleanup tasks before Phase 4 resumed: restore validation hygiene, delete or quarantine stale Pi 5 probe/proof surfaces, flatten the Pi 5 boot pipeline into named phases, and move cross-module tests out of src/main.rs.
- Decision: Accept the remediation sequence. 45e9e1a restored warning/format hygiene, 964be83 deleted stale Pi 5 assembly proof surfaces and stale archive modes, 6169369 split the Pi 5 boot path into named phase helpers, and aee54d2 moved FDT and target tests to owning modules. The next queued Phase 4 task is allowed to resume only after this checkpoint commit is recorded in supervisor state.
- Evidence level: static finding-to-commit review, source/script stale-inventory inspection, fmt/lint/typecheck, no-std unit tests, QEMU substitute, normal Pi 5 image generation, retained diagnostic image builds, whitespace inspection, and mdBook availability inspection.
- Validation: see docs/src/project/maintainability-remediation-checkpoint.md for the final gate output and source inventory. cargo fmt --all -- --check, cargo -Zjson-target-spec test with 51 no-std tests, scripts/qemu-smoke.sh, scripts/rpi5-image.sh, scripts/rpi5-format-guard-check.sh, representative retained diagnostic image builds, and git diff --check passed. mdbook build was not run because mdbook is unavailable in the container.
- Rationale: Phase 4 timer work can now continue without carrying stale probe surfaces, misleading warning suppressions, a nested Pi 5 boot body, or misplaced cross-module tests into the next hardware-facing task.
- Risks: The checkpoint does not prove new Pi 5 hardware behavior. The next Pi 5 timer smoke remains responsible for serialized hardware evidence, including TFTP/archive proof and serial classification.

## 2026-05-24 - Phase 4 Monotonic Tick Accounting Accepted

- Status: accepted as the first minimal monotonic tick counter for the single-core EL2 physical timer path. No scheduler structures, context switching, sleep queues, wall-clock time, lower-EL timer policy, SMP state, UART interrupts, DMA, RP1/PCIe routing, filesystem, userspace, or networking were added.
- Context: The accepted QEMU and Pi 5 timer smokes proved one-shot delivery of PPI 10 / INTID 26 through GICv2/GIC-400. The next bounded slice needed to prove periodic reprogramming and a shared tick count without putting formatting, allocation, or scheduler policy into the IRQ handler.
- Decision: Add shared generic-timer monotonic tick accounting with a centisecond cadence derived from `CNTFRQ_EL0` and a 1,000-counter floor. The handler now acknowledges the target-local GIC INTID, increments the shared relaxed atomic tick counter, reprograms `CNTHP_CVAL_EL2` before `GICC_EOIR`, and leaves diagnostic printing outside the IRQ path after interrupts are masked again. The proof target is four ticks.
- Evidence level: static code inspection, fmt/lint/typecheck, no-std unit tests, QEMU EL1 substitute smoke, QEMU EL2 periodic tick smoke, normal Pi 5 image generation, diagnostic image/archive inspection, Pi 5 format-guard build, serialized Pi 5 hardware boot/output, TFTP/archive proof, and git diff checks.
- Validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 54 no-std tests, `scripts/qemu-smoke.sh`, `scripts/qemu-timer-irq-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-timer-irq-diagnostic-image.sh`, `scripts/rpi5-format-guard-check.sh`, `scripts/rpi5-archive-review.sh target/talos-rpi5-monotonic-tick-boot.tar.gz`, and `git diff --check` passed. `mdbook` was unavailable in the container. The QEMU log showed `tick-count=4 target=4`, INTID 26, unexpected=0, and PASS. Pi 5 hardware evidence in `tasks/evidence/2026-05-24-pi5-monotonic-tick-accounting/` shows the 86,661-byte candidate `kernel_2712.img` was served by TFTP, serial reached `tick-count=4 target=4 vector=5 iar=0x0000001a intid=26 unexpected=0 ctl=0x1`, post-tick workload advanced, and PASS. The pre-run boot snapshot was restored afterward.
- Rationale: Reprogramming before EOI gives the eventual scheduler a real periodic time base while preserving the current interrupt-time constraints. Keeping the counter and cadence in the shared generic-timer module avoids duplicating policy between QEMU and Pi 5, while leaving GIC addresses and acknowledgement target-local.
- Risks: The cadence is a diagnostic scheduler-tick placeholder, not a committed preemption quantum or POSIX clock source. The relaxed atomic counter is single-core only and must be revisited for SMP. The next task must define interrupt mask/restore and critical-section semantics before scheduler structures consume ticks.

## 2026-05-24 - Phase 4 Single-Core Critical Section Policy Accepted

- Status: accepted as the first explicit IRQ mask/restore and critical-section contract for single-core kernel code. No scheduler structures, context switching, sleep queues, preemption time slicing, SMP locks, lower-EL interrupt policy, UART interrupts, DMA, RP1/PCIe routing, filesystem, userspace, or networking were added.
- Context: Monotonic tick accounting proved periodic EL2 physical timer interrupts, but scheduler work needs a named way to protect very short boot-CPU invariants before runnable queues or preemption can consume ticks.
- Decision: Add `SingleCoreIrqMaskState` plus `single_core_irq_mask_save()` and `single_core_irq_restore()`. The API snapshots `DAIF`, masks `PSTATE.I`, and restores the previous IRQ-mask state, so nested masked scopes remain masked and scopes entered with IRQs unmasked restore unmasked delivery on exit. The name deliberately marks the API as single-core-only; SMP will need per-core state and real locking before shared scheduler data can be protected across cores.
- Evidence level: static code inspection, fmt/lint/typecheck, no-std unit tests, QEMU EL1 substitute smoke, QEMU EL2 timer/tick smoke with critical-section diagnostics, normal Pi 5 image generation, Pi 5 format-guard build, and git diff checks.
- Validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test`, `scripts/qemu-smoke.sh`, `scripts/qemu-timer-irq-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, and `git diff --check` passed. The QEMU timer log showed `irq-mask nested-start=true inner-restored=true outer-restored=true unmasked-start=true saved-mask=true restored-unmasked=true`, `tick-count=4 target=4`, INTID 26, unexpected=0, bounded critical-section workload progress, and PASS. `mdbook` was unavailable in the container.
- Rationale: A save/restore primitive gives upcoming scheduler code a precise single-core contract without adding locks, preemption counters, or broader policy too early. Proving it inside the existing timer diagnostic keeps the task QEMU-focused because it does not change accepted Pi 5 physical timer behavior.
- Risks: The primitive is not a replacement for SMP-safe locking, preemption disable accounting, interrupt threading, or lower-EL interrupt policy. It must not be used for long sections that would hide timer latency once scheduler policy exists.

## 2026-05-24 - Phase 4 Scheduler Shape And POSIX Alignment Accepted

- Status: accepted as the Milestone 4.3 shape checkpoint before scheduler structs. No scheduler structs, runnable queues, context switching, preemption time slicing, SMP, userspace, syscalls, descriptor tables, filesystem, console/TTY, networking, SSH, boot image, hardware lock, or hardware run changed in this task.
- Context: The Phase 4.1/4.2 pre-scheduler closeout allowed Milestone 4.3 to start only with a bounded scheduler-shape task. The early POSIX note already warned that scheduler structures should not assume every schedulable context owns process resources directly.
- Decision: Accept a single-core, kernel-thread-first scheduler shape. Talos will schedule tasks; the first concrete tasks are kernel threads running in kernel address space. Process-owned resources such as address spaces, descriptor tables, current working directory, credentials, exit/wait state, and child state remain deferred to later POSIX/EL0 phases. The first scheduler structs should leave an extension point for a later process owner without implementing process semantics now.
- Evidence level: static documentation inspection, early POSIX reconciliation, supervisor-state update, whitespace inspection, and mdBook availability inspection.
- Validation: `git diff --check` passed. `mdbook build` was not run because `mdbook` is unavailable in the container. Rust fmt/tests and Pi 5 hardware were not required because this task changed only documentation and durable task state.
- Rationale: Keeping task, kernel-thread, process, and user-thread terminology explicit prevents the first scheduler structs from becoming a dead-end for future POSIX process, descriptor, blocking I/O, and wakeup semantics. The accepted single-core IRQ mask/restore primitive may guard very short boot-CPU scheduler invariants, but it is not a hidden preemption-disable or SMP-locking policy.
- Risks: The checkpoint does not implement a scheduler, prove context switching, or define a preemption quantum. SMP locks, secondary-core run queues, EL0, file descriptors, process IDs, wait/exit, console/TTY, filesystem, networking, and SSH remain deferred.

## 2026-05-24 - Pi 5 Timer-Preemption Hardware Proof Accepted

- Status: accepted as the physical Raspberry Pi 5 proof for the Phase 4.3 timer-driven single-core kernel-thread preemption shape. No SMP, lower-EL state, userspace, descriptors, filesystem, console/TTY, networking, or SSH behavior was added.
- Context: The QEMU timer-preemption smoke at 2cf0e64 proved that EL2 timer ticks can request preemption in the IRQ hot path while scheduler dispatch, context switching, and diagnostics remain after IRQ return. The next bounded task needed serialized hardware evidence on the Pi 5 GIC-400 / CNTHP_*_EL2 path.
- Decision: Add the focused `TALOS_RPI5_TIMER_PREEMPTION_DIAGNOSTIC` path and `scripts/rpi5-timer-preemption-diagnostic-image.sh`. The Pi 5 handler continues to acknowledge/classify INTID 26, record the monotonic tick and preemption-request counters, reprogram `CNTHP_CVAL_EL2`, write `GICC_EOIR`, and return. Two static kernel-thread contexts consume the request after IRQ return and call `SingleCoreScheduler::timer_preempt()` only inside the existing short single-core IRQ-masked scheduler mutation window.
- Evidence level: static code inspection, fmt/lint/typecheck, no-std unit tests, QEMU substitute, QEMU timer-preemption substitute, normal Pi 5 image generation, Pi 5 diagnostic image/archive inspection, serialized Pi 5 hardware boot/output, TFTP/archive proof, restore proof, and whitespace inspection.
- Validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 70 no-std tests, `scripts/qemu-smoke.sh`, `scripts/qemu-timer-preemption-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-timer-preemption-diagnostic-image.sh`, `scripts/rpi5-format-guard-check.sh`, `scripts/rpi5-archive-review.sh target/talos-rpi5-timer-preemption-boot.tar.gz`, and `git diff --check` passed. `mdbook` was unavailable in the container. The accepted hardware archive SHA256 was `950763917580e17aadfacd0f4e1ba3bba9e2b6960e800285e85db83cfaaa5f07`; kernel SHA256 was `417fd5f589b851c1fc1b2b1c77d7640fedc2abad32d0573effe8bc9606e550cb`; TFTP served `kernel_2712.img` at 103,152 bytes; serial reported task1=3, task2=3, ticks=6, requests=6, handled=6, timer-preemptions=6, dispatch-switches=6, voluntary-yields=0, vector=5, iar=0x0000001a, INTID 26, unexpected=0, and `rpi5-timer-preemption-smoke: PASS`. The pre-run snapshot `pre-phase4-pi5-timer-preemption-20260524T090536Z` was restored afterward.
- Rationale: Carrying the QEMU-proven shape to physical Pi 5 before contract consolidation avoids assuming that GIC-400 PPI delivery, timer reprogramming, context switching, and the post-IRQ scheduler handoff behave identically on hardware.
- Risks: This is still a diagnostic scheduler tick, not a committed quantum policy. It does not switch directly from an asynchronous exception frame, provide preemption-disable counters, add sleeping/wakeup policy, or make the scheduler SMP-safe.

## 2026-05-24 - Phase 5 Console Device Model Source Inventory Accepted

- Status: accepted as the first Phase 5.1 console-device-model planning boundary. No kernel code, normal boot output, boot archive, hardware lock, hardware run, scheduler/timer contract, descriptor table, TTY, input, userspace, filesystem, networking, SSH, or shell behavior changed in this task.
- Context: Phase 4 closeout allowed Phase 5 planning to start only with a bounded source inventory. Talos already has early logging through `print!` / `println!`, QEMU PL011, and Pi 5 firmware-preserved UART10, but it does not yet have a runtime console device that later descriptors and TTY work can share.
- Decision: Add `docs/src/architecture/console.md` and `docs/src/project/phase5-console-device-model-source-inventory.md`. The accepted boundary keeps early logging target-owned and polling-only, defines the runtime console as an output-only owner for normal kernel writes, and requires descriptor/TTY compatibility without implementing POSIX resources.
- Evidence level: static source inspection and documentation inventory. Sources included `src/target/mod.rs`, `src/pl011.rs`, `src/target/qemu_virt.rs`, `src/target/rpi5.rs`, `src/boot/rpi5.rs`, `src/boot/rpi5_reports.rs`, `src/main.rs`, `docs/src/architecture/early-serial.md`, and `docs/src/project/early-posix-shape.md`.
- Validation: `git diff --check` and `git diff --cached --check` passed. `mdbook build` was not run because `mdbook` is unavailable in the container. Rust tests, QEMU smokes, Pi 5 image builds, archives, and hardware were not required because only docs, task records, and durable state changed.
- Rationale: A runtime console write core should grow from the accepted early serial contracts instead of from a shell-only shortcut or a descriptor implementation invented too early. Naming the ownership boundary now keeps Phase 5 focused on local console capability before userspace and networking.
- Risks: This does not prove a new runtime console implementation. Pi 5 output remains constrained by the accepted UART10 early serial contract, and input/TTY/descriptor semantics remain deferred until explicit tasks design and validate them.

## 2026-05-24 - Phase 5 Console Input Source Inventory Accepted

- Status: accepted as the Phase 5.1 local input source inventory and first-input recommendation. No kernel code, boot output, boot archive, hardware lock, hardware run, UART RX path, UART interrupts, descriptor table, TTY, userspace, filesystem, networking, SSH, or shell behavior changed in this task.
- Context: The output-side runtime console is now named `runtime-console0`, but `stdin` still needs a real local input source before Milestone 5.2 TTY/stdio work can bind descriptors to console objects.
- Decision: Record that QEMU's PL011 UART0 at `0x0900_0000` is the first recommended input implementation target through a polling RX diagnostic. Pi 5 input should follow only with serialized hardware evidence, starting from UART10 if possible because it is the accepted `runtime-console0` output backend. RP1 UART0 remains deferred until a task explicitly owns RP1/PCIe and pinmux risk.
- Evidence level: static source and documentation inspection. Sources included `src/pl011.rs`, `src/target/qemu_virt.rs`, `src/target/rpi5.rs`, `src/target/mod.rs`, `src/runtime_console.rs`, `docs/src/architecture/console.md`, `docs/src/project/lab-controller.md`, and `docs/src/project/early-posix-shape.md`.
- Validation: `git status --short` was clean before edits. `git diff --check` passed after edits. `mdbook build` was not run because `mdbook` is unavailable in the container. Rust fmt/tests were not required because this task changed only documentation.
- Rationale: QEMU polling RX can prove the console input shape without the Pi 5 hardware lock or RP1 hardware ambiguity. Requiring explicit Pi 5 serial-write, serial-capture, TFTP, digest, and classification evidence prevents a future task from treating output-only UART evidence as an input claim.
- Risks: This does not implement input or prove that Pi 5 receives bytes on UART10. Blocking reads, echo, canonical mode, descriptor lifetime, scheduler wakeups, UART interrupts, userspace, local shell behavior, and RP1 UART0 ownership remain deferred.

## 2026-05-24 - Phase 5 Console Model Checkpoint Accepted

- Status: accepted as the Phase 5.1 closeout checkpoint before Milestone 5.2. No kernel code, boot image, hardware publish, hardware test, descriptor implementation, input implementation, TTY implementation, shell, networking, or scheduler behavior changed in this task.
- Context: Phase 5.1 accepted the source inventory, runtime console write core, write-result contract, default console identity, and input-source inventory. A checkpoint was required before TTY/stdio work could begin.
- Decision: Accept the Phase 5.1 console model as output-capable and input-planned. Normal diagnostics route through runtime-console0; target modules still own QEMU and Pi 5 PL011 backend selection; the write-result contract remains internal kernel-console state. Milestone 5.2 may start with the documentation-only phase5-tty-stdio-shape-doc-20260524 task.
- Evidence level: static documentation inspection, task-record reconciliation, whitespace inspection, and mdBook availability inspection.
- Validation: git status --short was clean before edits and git diff --check passed. mdbook build was not run because mdbook is unavailable in the container. Rust fmt/tests and hardware gates were not required because this checkpoint changed only Markdown documentation and durable task state.
- Rationale: The output-side console object is stable enough for TTY/stdio design, but input and descriptor implementation are not. Keeping Milestone 5.2 documentation-only prevents descriptor or shell shortcuts from bypassing the accepted console boundary.
- Risks: stdin still has no input backend; Pi 5 UART10 input is unproven; descriptor lifetime, syscall errno mapping, scheduler blocking I/O, line discipline, PTYs, userspace, filesystems, networking, and SSH remain deferred.

## 2026-05-24 - Phase 5 TTY and Stdio Shape Accepted

- Status: accepted as the Milestone 5.2 TTY/stdio design boundary. No kernel code, boot image, hardware publish, hardware test, UART RX implementation, TTY implementation, descriptor table, syscall ABI, userspace, shell, filesystem, networking, SSH, or scheduler behavior changed in this task.
- Context: Phase 5.1 accepted runtime-console0 as an output-capable and input-planned console object. Milestone 5.2 needed a behavior contract before workers implement input, echo, line editing, or descriptor-facing stdio surfaces.
- Decision: Define the first local serial TTY as a line-discipline object above the runtime console backend. Raw mode delivers bytes without translation and defaults to echo off. Canonical mode collects a bounded byte-oriented line, treats CR/LF/CRLF as line termination, handles backspace/delete erase, echoes deterministically, and records control bytes as diagnostic events rather than POSIX signals. stdin, stdout, and stderr are future descriptor-capable streams; descriptor tables and syscall ABI remain separate tasks.
- Evidence level: static documentation inspection over docs/src/architecture/console.md, docs/src/architecture/tty-stdio.md, docs/src/project/early-posix-shape.md, docs/src/project/phase5-tty-stdio-shape.md, and tasks/2026-05-24-phase5-tty-stdio-shape.md.
- Validation: git status --short was clean before edits, git diff --check passed, and git diff --cached --check passed. mdbook build was not run because mdbook is unavailable in the container. Rust fmt/tests and hardware gates were not required because this task changed only Markdown documentation and durable task state.
- Rationale: The next implementation needs exact behavior for newline, backspace, echo, control characters, and stdio attachment points before adding UART RX code. Keeping descriptor ownership separate prevents a diagnostic TTY from becoming a shell-only or target-UART shortcut.
- Risks: QEMU input is still unimplemented; Pi 5 input remains unproven; scheduler blocking I/O, descriptor lifetime, errno mapping, termios, signals, sessions, PTYs, userspace, filesystems, shell, networking, and SSH remain deferred.

## 2026-05-24 - Pi 5 UART10 Polling RX Proof Accepted

- Status: accepted as the first physical Raspberry Pi 5 proof that the firmware-preserved UART10 console path can also receive local serial input through the internal runtime-console/TTY polling boundary. No RP1 UART0 input, UART interrupts, scheduler blocking I/O, descriptor tables, syscalls, userspace, shell, filesystem, networking, SSH, termios, PTY, or POSIX signal behavior was added.
- Context: QEMU had already proved PL011 polling RX and the shared canonical-lite TTY core. The Pi 5 input source inventory required serialized hardware evidence before any Pi 5 input claim could be used by later TTY/stdio closeout work.
- Decision: Add `TALOS_RPI5_UART10_POLLING_RX_DIAGNOSTIC` plus `scripts/rpi5-uart10-rx-diagnostic-image.sh`. The diagnostic keeps target hardware selection in `target::rpi5::firmware_console()`, then calls the shared `tty::run_polling_rx_diagnostic_with_limit` path over the runtime-console input contract. The accepted run uses the same 15-byte injected sequence as QEMU: `61 62 58 08 63 59 7f 64 03 65 66 67 68 69 0d`.
- Evidence level: static code inspection, fmt/lint/typecheck, no-std unit tests, QEMU substitute smoke, normal Pi 5 image generation, Pi 5 diagnostic image/archive inspection, serialized Pi 5 hardware boot/output, lab serial injection, TFTP/archive proof, restore proof, and whitespace inspection.
- Validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 84 no-std tests, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-uart10-rx-diagnostic-image.sh`, `scripts/rpi5-format-guard-check.sh`, `scripts/rpi5-archive-review.sh target/talos-rpi5-uart10-rx-boot.tar.gz`, and `git diff --check` passed. `mdbook` was unavailable in the container. The accepted archive SHA256 was `bab86eacea7868b4fd92423370c4991b11cc6f270c60b7b38ae5960336f54209`; kernel SHA256 was `2a497150163f6e53ec6b5d4b33c4e44f0f3d29f6f34f4b319a9e93515ba83a6d`; TFTP served `kernel_2712.img` at 90,344 bytes; serial output reached `rpi5-uart10-rx-diagnostic: PASS` after the lab wrote 15 bytes; and the pre-run snapshot was restored to tree hash `6ead8933b3287cddaf08ddcdbda7d4c770b365658ef1d0ad80a5dd0cf323f3ef`.
- Rationale: Proving receive on the already accepted UART10 console avoids pulling RP1 UART0, PCIe, or pinmux ownership into Milestone 5.2. Keeping the proof polling and diagnostic-only gives later descriptor and blocking-I/O work concrete input evidence without prematurely committing POSIX semantics.
- Risks: This is still a bounded polling diagnostic. It does not prove interrupt-driven UART receive, scheduler wakeups, readiness notification, EOF semantics, descriptor lifetime, user/kernel copy, shell command routing, or behavior under long interactive sessions.

## 2026-05-24 - Phase 5 TTY And Stdio Closeout Accepted

- Status: accepted as the Milestone 5.2 closeout checkpoint. No Rust code, boot image, hardware publish, hardware test, descriptor implementation, syscall ABI, userspace, shell, filesystem, networking, SSH, SMP, or scheduler behavior changed in this checkpoint.
- Context: Milestone 5.2 accepted the TTY/stdio design, QEMU polling TTY RX diagnostic, target-independent canonical-lite line-discipline core, internal console input result contract, and serialized Pi 5 UART10 polling RX proof. A checkpoint was required before any Milestone 5.3 local diagnostic command channel, descriptor, shell, filesystem, networking, SSH, or later phase work starts.
- Decision: Accept Milestone 5.2 as an evidence-backed local serial input boundary. QEMU PL011 and Pi 5 UART10 both prove the same runtime-console/TTY diagnostic path with the same injected byte sequence, deterministic echo, line capture, truncation, and ctrl-c control-event reporting. Keep descriptor reads, scheduler blocking I/O, UART interrupts, shell input, POSIX termios/signals/PTYs, filesystem behavior, networking, and SSH deferred. Recommend that the supervisor plan phase5-local-diagnostic-command-channel-source-inventory-20260524 as the next bounded Milestone 5.3 documentation/source-inventory task.
- Evidence level: static documentation reconciliation, accepted task-record review, serialized hardware evidence review, whitespace inspection, and mdBook availability inspection.
- Validation: git status --short was clean before edits and git diff --check passed after edits. mdbook build was not run because mdbook is unavailable in the container. Rust fmt/tests were not required because this checkpoint changed only Markdown documentation and durable task state.
- Rationale: Closing Milestone 5.2 prevents later command-channel or descriptor work from silently treating polling diagnostics as POSIX reads, shell input, or interrupt-driven UART ownership. Naming the next source-inventory slice keeps Milestone 5.3 grounded in the accepted runtime-console and TTY surfaces.
- Risks: The accepted input path is still polling and diagnostic-only. It does not prove interactive session longevity, UART interrupt delivery, descriptor lifetime, readiness wakeups, EOF/errno semantics, user/kernel copy, command parsing, userspace shell behavior, filesystem integration, networking, SSH, RP1 UART0, or SMP.

## 2026-05-24 - Phase 5 Diagnostic Command Channel Source Inventory Accepted

- Status: accepted as the Milestone 5.3 source inventory before any local diagnostic command-channel implementation. No Rust code, boot image, hardware publish, hardware test, descriptor table, syscall ABI, userspace shell, filesystem, networking, SSH, SMP, UART interrupt, or scheduler blocking behavior changed in this task.
- Context: Milestone 5.2 closed with accepted QEMU and Pi 5 local serial input evidence. Before implementing a kernel-owned command channel, Talos needed to classify which existing diagnostics may become command providers and which should remain boot-only validation gates.
- Decision: Add docs/src/project/phase5-local-diagnostic-command-channel-source-inventory.md and tasks/2026-05-24-phase5-local-diagnostic-command-channel-source-inventory.md. The accepted boundary says the command channel consumes complete TTY lines, writes bounded responses through runtime-console0, and stays separate from descriptor/syscall/POSIX shell semantics. Help/list, status, timer/tick, scheduler, and memory/runtime summaries are candidates for the next contract task; QEMU smokes, Pi 5 hardware diagnostic images, destructive fault triggers, allocator stress diagnostics, and retired stale proof paths are not first command-channel interfaces.
- Evidence level: static source and documentation inventory over the accepted runtime-console, TTY, scheduler, timer, memory/runtime, diagnostic script, and Phase 5.2 checkpoint surfaces.
- Validation: git status --short was clean before edits and git diff --check passed after edits. mdbook build was not run because mdbook is unavailable in the container. Rust fmt/tests were not required because this task changed only Markdown documentation and durable task state.
- Rationale: A command channel should grow from the accepted TTY boundary rather than become a hidden shell, a direct UART client, or a dumping ground for one-off boot proofs. Classifying diagnostics first keeps later command names and parser behavior bounded.
- Risks: The inventory does not prove command parsing, command response framing, long interactive sessions, descriptor integration, blocking reads, filesystem-backed commands, process execution, networking, SSH, SMP, UART interrupts, or RP1 UART0.

## 2026-05-24 - Phase 5 Diagnostic Command Channel Contract Accepted

- Status: accepted as the first Milestone 5.3 command-channel contract and target-independent parser/dispatcher shape. No hardware run, hardware publish, Pi 5-specific behavior, descriptor table, syscall ABI, userspace shell, filesystem command execution, networking, SSH, SMP, UART interrupt, or scheduler blocking read behavior was added.
- Context: The source inventory accepted runtime-console0 and the TTY line discipline as the command-channel boundary, and classified help/list/status as the first bounded command set. The next task needed a source-backed contract before the QEMU command-channel smoke could inject serial commands.
- Decision: Add `src/diagnostic_command.rs`, `docs/src/architecture/diagnostic-command-channel.md`, and `tasks/2026-05-24-phase5-diagnostic-command-channel-contract.md`. The parser consumes completed TTY lines, bounds command and argument tokens, supports `help`, `list`, and `status`, reports deterministic parser/unknown/argument errors, and writes newline-framed `diag:` responses through a `DiagnosticResponseSink` that can attach to `runtime_console::RuntimeConsole`.
- Evidence level: static code inspection, fmt/lint/typecheck, no_std unit tests, QEMU substitute smoke, normal Pi 5 image generation, Pi 5 format-guard build, documentation diff inspection, and mdBook availability inspection.
- Validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 90 no_std tests, `scripts/qemu-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-format-guard-check.sh`, and `git diff --check` passed. `mdbook` was unavailable in the container.
- Rationale: A small parser/dispatcher makes the next QEMU smoke deterministic without turning the TTY path into a shell or descriptor API. Keeping the response sink abstract lets runtime-console0 carry output while target modules continue to own hardware backends.
- Risks: The contract does not prove long interactive sessions, Pi 5 command-channel behavior, descriptor lifetime, POSIX read/write mapping, syscall ABI, filesystem-backed commands, process execution, networking, SSH, SMP safety, UART interrupts, or scheduler blocking I/O.

## 2026-05-24 - Phase 5 Diagnostic Command Channel Closeout Accepted

- Status: accepted as the Milestone 5.3 local diagnostic command-channel closeout checkpoint. No Rust code, boot image, hardware publish, hardware test, descriptor table, syscall ABI, userspace shell, filesystem, networking, SSH, SMP, UART interrupt, RP1 UART0, scheduler blocking I/O, or phase transition changed in this task.
- Context: Milestone 5.3 had accepted source inventory at `e038fd5`, command-channel contract at `2fed739`, QEMU command-channel smoke at `6dc9165`, and serialized Pi 5 UART10 command-channel proof at `7c8598c`. A checkpoint was required before any later roadmap work could treat the command channel as a stable local diagnostic boundary.
- Decision: Add `docs/src/project/phase5-diagnostic-command-channel-closeout-checkpoint.md` and `tasks/2026-05-24-phase5-diagnostic-command-channel-closeout-checkpoint.md`. Accept Milestone 5.3 as an evidence-backed kernel-owned diagnostic-only command channel over canonical-lite TTY input and runtime-console0 output. Retain `help`, `list`, `status`, parser-error labels, and deterministic `unknown-command` behavior; retain QEMU and Pi 5 command-channel smokes as regression gates; keep descriptors, syscalls, shell behavior, filesystem-backed commands, networking, SSH, SMP, UART interrupts, RP1 UART0, scheduler blocking I/O, termios, POSIX signals, sessions, and PTYs deferred.
- Evidence level: static documentation reconciliation over accepted task records, QEMU/substitute evidence, serialized Pi 5 hardware boot/output evidence, TFTP/archive proof, restore proof, whitespace inspection, and mdBook availability inspection.
- Validation: `git status --short` was clean before checkpoint edits and `git diff --check` passed after edits. `mdbook build` was not run because `mdbook` is unavailable in the container. Rust fmt/tests were not required because this checkpoint changed only Markdown documentation and durable task state.
- Rationale: Closing Milestone 5.3 prevents later descriptor, syscall, shell, filesystem, networking, or SMP work from silently expanding a polling diagnostic channel into a POSIX or shell interface. Naming the next work as supervisor-planned keeps the worker from inferring a phase transition.
- Risks: The command channel remains polling and diagnostic-only. It does not prove long interactive sessions, interrupt-driven UART receive, descriptor lifetime, syscall return conventions, user/kernel copy, shell command routing, filesystem traversal, network sessions, SSH login, SMP synchronization, or scheduler wakeups.

## 2026-05-24 - Phase 6 QEMU Secondary-Core Discriminator Accepted

- Status: accepted as the QEMU/substitute discriminator for Milestone 6.1 secondary-core bring-up. No Pi 5 hardware publish, hardware lock, hardware run, production SMP scheduler, SMP-safe lock, task migration, cross-core preemption, userspace, syscalls, descriptors, filesystem, networking, SSH, or shell behavior was added.
- Context: The accepted Phase 6.1 source inventory and contract require secondary cores to prove identity, stack ownership, per-core state registration, and controlled handoff before scheduler work. The next bounded task needed to determine whether QEMU virt can exercise a useful PSCI secondary-core startup path before serialized Pi 5 hardware proof.
- Decision: Add `TALOS_QEMU_SECONDARY_CORE_DISCRIMINATOR`, `scripts/qemu-secondary-core-discriminator.sh`, an AArch64 secondary entry trampoline, MPIDR affinity helpers, and a QEMU-only diagnostic path. Under QEMU virt with EL2 virtualization, GICv2, Cortex-A76, and four CPUs, PSCI `CPU_ON` through SMC starts logical CPUs 1, 2, and 3. Each secondary reports distinct MPIDR affinity, runs on its reserved stack slot, records `handoff-ready`, and parks in `wfe`.
- Evidence level: static code inspection, fmt/lint/typecheck, no_std unit tests, QEMU substitute smoke, QEMU secondary-core substitute boot/output, normal Pi 5 image generation, whitespace inspection, and mdBook availability inspection.
- Validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 92 no_std tests, `scripts/qemu-smoke.sh`, `scripts/qemu-secondary-core-discriminator.sh`, `scripts/rpi5-image.sh`, `git diff --check`, and `git diff --cached --check` passed. `mdbook` was unavailable in the container. The QEMU transcript classified the path as `qemu-psci-smc-secondary-cores-alive` with MPIDR affinities `0x1`, `0x2`, and `0x3`.
- Rationale: Keeping this as a QEMU-only discriminator gives the next per-core state/stack task concrete startup evidence without treating QEMU as Pi 5 hardware proof. Recording SMC as the accepted EL2 QEMU conduit avoids the HVC trap seen during development while preserving the Pi 5 firmware/DTB SMC contract.
- Risks: QEMU MPIDR shape differs from the Pi 5 source inventory (`0x1/0x2/0x3` versus expected Pi 5 affinity spacing of `0x100/0x200/0x300` for secondaries). The diagnostic parks secondaries and does not prove Pi 5 PSCI behavior, cache coherency under hardware, concurrent console writes, SMP-safe primitives, scheduler migration, IPIs, cross-core preemption, userspace, descriptors, filesystem, networking, SSH, or shell behavior.

## 2026-05-24 - Pi 5 PSCI Secondary Entry Not Yet Observed

- Status: interim hardware decision for the in-progress phase6-pi5-psci-secondary-core-alive-proof-20260524 task. The Pi 5 proof is not accepted.
- Context: The Phase 6.1 source inventory expected Pi 5 PSCI SMC CPU_ON with target affinities 0x100, 0x200, and 0x300 to branch secondary cores into Talos' provided entry point. Hardware returned CPU_ON result 0 for all three targets, but secondary state stayed parked.
- Decision: Treat this as pi5-psci-accepted-secondary-entry-not-observed, not as a stack/state registration failure. A secondary-entry discriminator placed fixed UART10 markers at the first trampoline instruction and just before the Rust secondary entry; the corrected hardware transcript showed primary boot and PSCI success but neither secondary marker appeared.
- Evidence level: image/archive inspection, fmt/lint/typecheck, no_std unit tests, QEMU substitute, serialized Pi 5 TFTP/archive proof, serial hardware boot/output, and restore proof.
- Validation: cargo fmt --all -- --check, cargo -Zjson-target-spec test with 96 no_std tests, scripts/qemu-secondary-core-discriminator.sh, scripts/qemu-smoke.sh, scripts/rpi5-psci-secondary-core-alive-image.sh, scripts/rpi5-archive-review.sh target/talos-rpi5-psci-secondary-core-alive-entry-discriminator-boot.tar.gz, scripts/rpi5-format-guard-check.sh, and git diff --check passed. mdbook was unavailable in the container. Candidate archive SHA256 was 1ef6ec1daf33cc99feae786dc2daa765dbff9aa9308edd71b3240f117769df6f; kernel SHA256 was 5e099ff4e75986cc7043fc196d41565fb9ada25321a8f8386be9c45c7d0931e7; TFTP served da591740/kernel_2712.img at 90,016 bytes.
- Rationale: The hardware evidence contradicts the simple assumption that PSCI success is enough to prove secondary control transfer. Keeping the classification narrow prevents later SMP tasks from building on an unproven Pi 5 secondary-entry path.
- Risks: The current evidence does not yet distinguish an entry-address interpretation issue, PSCI affinity-state behavior, firmware policy, or a missing cache/coherency barrier around the secondary entry handoff. No SMP-safe primitive, scheduler migration, cross-core wakeup, userspace, filesystem, networking, SSH, or shell behavior is accepted.

## 2026-05-24 - Pi 5 PSCI State Discriminator Needs Local Image Review

- Status: interim hardware decision for the in-progress phase6-pi5-psci-secondary-core-alive-proof-20260524 task. The Pi 5 proof is not accepted.
- Context: After CPU_ON returned success without secondary-entry markers, a bounded state discriminator added PSCI_VERSION, PSCI_FEATURES, and AFFINITY_INFO calls around the same CPU_ON path. The image was intended to distinguish firmware affinity state from entry-address/state publication issues.
- Decision: Treat the corrected state-discriminator run as `pi5-state-discriminator-candidate-fetched-no-bl31-or-asm-entry`, not as accepted PSCI state evidence. TFTP proved the Pi fetched the 91,000-byte candidate image twice, but serial did not show BL31, `TALOS: asm_start`, `TALOS: rust_entry`, or the new PSCI state lines for that boot. The next iteration should review the image/disassembly locally or shrink the discriminator before another hardware attempt.
- Evidence level: image/archive inspection, fmt/lint/typecheck, no_std unit tests, QEMU substitute, serialized Pi 5 TFTP/archive proof, partial serial hardware output, and restore proof.
- Validation: cargo fmt --all -- --check, cargo -Zjson-target-spec test with 96 no_std tests, scripts/qemu-secondary-core-discriminator.sh, scripts/qemu-smoke.sh, scripts/rpi5-psci-secondary-core-alive-image.sh, scripts/rpi5-archive-review.sh target/talos-rpi5-psci-secondary-core-alive-state-discriminator-boot.tar.gz, scripts/rpi5-format-guard-check.sh, and git diff --check passed. mdbook was unavailable in the container. Candidate archive SHA256 was `e2e16f292d5f8ad9eff8b139af47f0491d7f9af44397488941ec2dbf8a449bca`; kernel SHA256 was `cd90dde7543838ad8f95203b92a0d90914ff62695d7c881d717a1cf8d478d954`; TFTP served `da591740/kernel_2712.img` at 91,000 bytes.
- Rationale: The candidate-fetch/no-entry result is a property of this discriminator image and should not be conflated with the prior CPU_ON-success/no-secondary-entry result. Keeping the classification narrow avoids building Phase 6.1 acceptance on ambiguous serial or staging evidence.
- Risks: The current evidence still does not explain whether the original PSCI success path is blocked by entry-address interpretation, affinity-state behavior, firmware policy, cache/coherency requirements, or a discriminator-specific image issue. No SMP-safe primitive, scheduler migration, cross-core wakeup, userspace, filesystem, networking, SSH, or shell behavior is accepted.

## 2026-05-24 - Pi 5 Minimal PSCI State Discriminator Still Not Accepted

- Status: interim hardware decision for the in-progress phase6-pi5-psci-secondary-core-alive-proof-20260524 task. The Pi 5 proof is not accepted.
- Context: Local image/disassembly review found the larger PSCI state-discriminator image structurally valid. The next bounded iteration tightened the SMC wrapper clobbers and reduced the state probe to a smaller post-CPU_ON AFFINITY_INFO discriminator.
- Decision: Classify the minimal-state run as `pi5-minstate-discriminator-candidate-fetched-no-current-entry`. TFTP proved repeated fetches of `da591740/kernel_2712.img` at 90,416 bytes, but serial observed after the run did not contain current-candidate BL31-to-Talos entry or the new affinity-state diagnostic lines before the pre-run snapshot was restored.
- Evidence level: static image/disassembly inspection, image/archive inspection, fmt/lint/typecheck, no_std unit tests, QEMU substitute, serialized Pi 5 TFTP/archive proof, partial serial hardware output, and restore proof.
- Validation: cargo fmt --all -- --check, cargo -Zjson-target-spec test with 96 no_std tests, scripts/qemu-secondary-core-discriminator.sh, scripts/qemu-smoke.sh, scripts/rpi5-psci-secondary-core-alive-image.sh, scripts/rpi5-archive-review.sh target/talos-rpi5-psci-secondary-core-alive-minstate-boot.tar.gz, scripts/rpi5-format-guard-check.sh, and git diff --check passed. mdbook was unavailable in the container. Candidate archive SHA256 was `542f3b87302b82a91776f72d0e04408c24cf9680205537acd19447a00e0475dd`; kernel SHA256 was `50295ba874792d6e732c2af6b70fdffd708e86847e4b30c0fb873442dd71807f`.
- Rationale: The smaller state probe rules out a simple local image-header/string/symbol problem, but it still does not produce accepted PSCI affinity-state evidence. Keeping it non-accepted prevents Phase 6.1 from treating repeated candidate fetches as proof of secondary-core control.
- Risks: The current evidence still does not explain whether the gap is entry-address interpretation, firmware policy, cache/coherency requirements, image/layout sensitivity, or lab observation of repeated network-boot retries without completed BL31-to-kernel handoff. No SMP-safe primitive, scheduler migration, cross-core wakeup, userspace, filesystem, networking, SSH, or shell behavior is accepted.

## 2026-05-24 - Pi 5 PSCI Secondary-Core Alive Proof Accepted

- Status: accepted as the Pi 5 hardware proof for Phase 6.1 boot-time secondary-core alive/park behavior.
- Context: Earlier hardware iterations proved PSCI CPU_ON returned success and secondaries reached Rust/state-published markers, but the primary still read parked zero state until the diagnostic made per-core state cache-visible across cores.
- Decision: Accept phase6-pi5-psci-secondary-core-alive-proof-20260524. The proof keeps PSCI SMC as the Pi 5 bring-up path, uses target affinities 0x100, 0x200, and 0x300, and records secondary identity, stack ownership, per-core registration, and controlled handoff-ready parking. The diagnostic cleans secondary state updates to the point of coherency and invalidates the primary view before snapshots.
- Evidence level: static code inspection, fmt/lint/typecheck, no_std unit tests, QEMU substitute, image/archive inspection, serialized Pi 5 TFTP/archive proof, serial hardware boot/output, and restore proof.
- Validation: cargo fmt --all -- --check, cargo -Zjson-target-spec test with 96 no_std tests, scripts/qemu-secondary-core-discriminator.sh, scripts/qemu-smoke.sh, scripts/rpi5-format-guard-check.sh, scripts/rpi5-archive-review.sh target/talos-rpi5-psci-secondary-core-alive-cachecoherent-boot.tar.gz, and git diff --check passed. mdbook was unavailable in the container. Candidate archive SHA256 was 58803e6c4fd21a7c40d2f36245e8e7c366e80ea50dbcdba2afd2952d952c4d22; kernel SHA256 was 2f1622d7694f84446153240d1136b9f095df0cd09d69e9f0ae88de2ae1ef9996; TFTP served da591740/kernel_2712.img at 90,784 bytes before restore.
- Rationale: The accepted run shows logical cores 1-3 with MPIDR affinities 0x100, 0x200, and 0x300, distinct stack slots, handoff-ready lifecycle, ok=true reports, classification pi5-psci-smc-secondary-cores-alive, and PASS. This satisfies the alive-proof acceptance criteria without claiming scheduler, lock, workload, or userspace behavior.
- Risks: This remains a boot-time alive/park proof. It does not accept controlled secondary kernel-thread workload, SMP-safe primitives, scheduler migration, cross-core wakeups, concurrent console ownership, UART interrupts, EL0, descriptors, syscalls, filesystem, networking, SSH, or shell behavior.

## 2026-05-24 - Phase 6 Secondary-Core Controlled Workload Accepted

- Status: accepted as the Phase 6.1 diagnostic controlled secondary-core workload proof. No production SMP scheduler, SMP-safe lock, run-queue sharing, task migration, load balancing, cross-core preemption, userspace, syscalls, descriptors, filesystem, networking, SSH, or shell behavior was added.
- Context: The accepted Pi 5 alive proof showed secondary cores could start, own stacks, publish per-core state, and park. Milestone 6.1 still required a bounded workload before closeout, while the roadmap keeps scheduler migration and SMP-safe primitives in later milestones.
- Decision: Add `TALOS_QEMU_SECONDARY_CORE_WORKLOAD_SMOKE`, `TALOS_RPI5_SECONDARY_CORE_WORKLOAD_PROOF`, per-core `workload_progress`, lifecycle states `workload-running` and `workload-complete`, and focused QEMU/Pi 5 image scripts. The workload records deterministic progress to 64 through the accepted secondary trampoline and stack boundary, then leaves the production scheduler single-core.
- Evidence level: static code inspection, fmt/lint/typecheck, no_std unit tests, QEMU substitute boot/output, image/archive inspection, serialized Pi 5 TFTP/archive proof, serial hardware boot/output, restore proof, and whitespace inspection.
- Validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 97 no_std tests, `scripts/qemu-smoke.sh`, `scripts/qemu-secondary-core-workload-smoke.sh`, `scripts/rpi5-image.sh`, `scripts/rpi5-secondary-core-workload-image.sh`, `scripts/rpi5-archive-review.sh target/talos-rpi5-secondary-core-workload-boot.tar.gz`, `scripts/rpi5-format-guard-check.sh`, and `git diff --check` passed. `mdbook` was unavailable in the container. Candidate archive SHA256 was `73e7419eef2ddc0e5ba6a4ac3756d5c0b1d0c2f5b6888b7759b9b921f6621fa7`; kernel SHA256 was `a0ecfe8fef7ad4d144ed68ceefeadf325c4a5fa3ca9cb7b703f7c6e6927d8092`; TFTP served `da591740/kernel_2712.img` at 91,288 bytes before restore.
- Rationale: The accepted QEMU and Pi 5 transcripts show cores 1-3 reaching `workload-complete` with `progress=64 target=64 ok=true`, proving a bounded secondary workload without broadening into scheduler work.
- Risks: The workload is still diagnostic-only. It does not prove shared scheduler data structures, synchronization policy, interrupt routing, IPIs, fair scheduling, concurrent console ownership, user/kernel boundaries, blocking I/O, filesystem behavior, network behavior, SSH, or shell behavior.

## 2026-05-24 - Phase 6.1 Secondary-Core Bring-Up Closeout Accepted

- Status: accepted as the Milestone 6.1 secondary-core bring-up closeout checkpoint. No Rust code, boot image, hardware publish, hardware test, SMP-safe primitive, scheduler migration, shared run queue, EL0, syscall, descriptor, filesystem, networking, SSH, shell, UART interrupt, RP1/PCIe, or DMA behavior was added.
- Context: Phase 6.1 had accepted the source inventory and contract at `50e2bbf`, QEMU secondary-core discriminator at `80ffca0`, per-core state/stacks at `78db923`, Pi 5 PSCI secondary-core alive proof at `4f5f1a9`, and controlled secondary-core workload proof at `19cd241`. A checkpoint was required before Milestone 6.2 SMP-safe primitives or later scheduler work.
- Decision: Add `docs/src/project/phase6-secondary-core-bringup-closeout-checkpoint.md` and `tasks/2026-05-24-phase6-secondary-core-bringup-closeout-checkpoint.md`. Milestone 6.1 accepts PSCI SMC secondary-core startup, MPIDR/logical identity, distinct secondary stack ownership, per-core state publication, handoff-ready parking, and a bounded diagnostic-only workload with `progress=64 target=64 ok=true`.
- Evidence level: static documentation reconciliation, accepted task/evidence review, whitespace inspection, and mdBook availability inspection.
- Validation: `git status --short` was clean before edits and `git diff --check` passed after edits. `mdbook` was unavailable in the container. Rust fmt/tests were not required because this checkpoint changed only Markdown documentation and durable task state.
- Rationale: Closing Milestone 6.1 prevents later SMP or scheduler work from silently treating boot-time secondary-core diagnostics as shared scheduler infrastructure. The accepted Pi 5 proof also makes the cache-maintenance lesson explicit before locks or shared queues are designed.
- Risks: Milestone 6.1 remains a bring-up and diagnostic workload boundary. SMP-safe primitives, per-core critical-section policy, shared scheduler data structures, IPIs, cross-core wakeups, multi-core preemption, concurrent console ownership, userspace, descriptors, filesystem, networking, SSH, shell behavior, UART interrupts, RP1/PCIe, and DMA/cache policy remain deferred.

## 2026-05-24 - Phase 6.2 Spinlock/Barrier Core Accepted

- Status: accepted as the first Milestone 6.2 SMP-safe primitive implementation. No scheduler migration, shared run queue, cross-core wakeup, IPI, userspace, descriptor, filesystem, networking, SSH, shell, UART interrupt, RP1/PCIe, DMA, hardware publish, or hardware test behavior was added.
- Context: The accepted SMP-safe primitive source inventory required a narrow mutual-exclusion primitive, explicit IRQ-save composition, acquire/release memory ordering, non-recursive policy, and separation from early-boot cache maintenance before any shared scheduler data structures could be attempted.
- Decision: Add `src/smp_sync.rs` and register it from `src/main.rs`. `SpinLock<T>` uses an `AtomicBool` compare-exchange loop with acquire ordering on successful acquisition and release ordering on unlock. `SpinLockGuard` releases on drop and carries a CPU-local marker; `try_lock()` exposes non-recursive misuse to tests; AArch64 `lock_irqsave()` masks local IRQs before acquiring and restores after release; `smp_full_barrier()` names the first `dmb ish` barrier boundary.
- Evidence level: static source inspection, fmt/lint/typecheck, no_std unit tests, QEMU/substitute smoke, whitespace inspection, and mdBook availability inspection.
- Validation: `git status --short` was clean before edits. `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 102 no_std tests, `scripts/qemu-smoke.sh`, and `git diff --check` passed. `mdbook` was unavailable in the container.
- Rationale: This gives later QEMU and Pi 5 contention diagnostics a reusable synchronization primitive without prematurely converting the scheduler to SMP or hiding cache-maintenance policy inside a generic lock.
- Risks: The primitive is not yet proven under multi-core contention, and no Pi 5 cache/coherence proof has exercised it. Scheduler runnable queues, task migration, cross-core wakeups, IPIs, multi-core preemption, concurrent console ownership, userspace, descriptors, filesystem, networking, SSH, shell behavior, UART interrupts, RP1/PCIe, and DMA/cache-coherent driver policy remain deferred.

## 2026-05-24 - Phase 6.2 QEMU SMP Lock Contention Smoke Accepted

- Status: accepted as the QEMU/substitute contention proof for the first Milestone 6.2 SMP-safe primitive. No Pi 5 hardware publish/test, scheduler migration, shared run queue, cross-core wakeup, IPI, userspace, descriptor, filesystem, networking, SSH, shell, UART interrupt, RP1/PCIe, or DMA behavior was added.
- Context: The accepted spinlock/barrier core needed a bounded multi-core contention diagnostic before moving to physical Pi 5 cache/coherence proof.
- Decision: Add `TALOS_QEMU_SMP_LOCK_CONTENTION_SMOKE`, `scripts/qemu-smp-lock-contention-smoke.sh`, and a QEMU virt diagnostic that starts secondary cores through the accepted PSCI/trampoline path and has cores 1-3 contend on a shared `SpinLock<T>` around a deterministic counter.
- Evidence level: static source inspection, fmt/lint/typecheck, no_std unit tests, QEMU/substitute smoke, focused QEMU SMP contention transcript, whitespace inspection, and mdBook availability inspection.
- Validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 102 no_std tests, `scripts/qemu-smoke.sh`, `scripts/qemu-smp-lock-contention-smoke.sh`, and `git diff --check` passed. `mdbook` was unavailable in the container.
- Rationale: The focused QEMU transcript reports cores 1, 2, and 3 each reached `workload-complete` with `lock-count=64`, the final invariant `counter=192 expected=192 participants=3 errors=0`, and classification `qemu-smp-lock-contention-complete`.
- Risks: This remains QEMU/substitute evidence only. The separate Pi 5 task must prove or decisively classify physical cache/coherence behavior; scheduler migration, shared run queues, cross-core wakeups, IPIs, userspace, descriptors, filesystem, networking, SSH, shell behavior, UART interrupts, RP1/PCIe, and DMA/cache-coherent driver policy remain deferred.

## 2026-05-24 - Pi 5 SMP Lock Proof Reclassified To Workload Stall

- Status: interim hardware decision for the in-progress `phase6-pi5-smp-lock-cache-coherence-proof-20260524` task. The Pi 5 lock proof is not accepted.
- Context: Earlier lock-proof and entry-discriminator runs proved TFTP candidate fetches but did not produce cursor-valid candidate serial. A supervisor intervention required separating serial capture, candidate execution, and lock behavior before more marker-only changes.
- Decision: Treat the latest evidence as `pi5-smp-lock-cache-coherence-secondary-lock-workload-stall`, not as serial-windowing or fetched-but-not-executed. The known-good secondary-workload control produced cursor-valid PASS evidence with the immediate read loop, and the entry-discriminator candidate reached Talos entry/normal boot under the same method. The full lock proof then reached the lock proof start line, logical-1 CPU_ON, secondary entry/Rust-entry/state-publish interleaving, and logical-1 affinity-on state, but did not reach lock reports, final invariant, PASS, or FAIL before restore.
- Evidence level: image/archive inspection, serialized Pi 5 hardware run under hardwareTestLock, cursor-valid serial hardware output, TFTP/archive/status evidence where available, and restore proof.
- Validation: `scripts/rpi5-archive-review.sh` passed for the known-good secondary workload archive and the early-entry lock discriminator archive; previous task-local gates for the full lock archive, QEMU substitute, tests, and diff check remained the baseline. The clean full-lock rerun restored the pre-run 82,045-byte boot snapshot after the observe loop hung.
- Rationale: The A/B control proves the lab serial method can capture a current Pi 5 SMP workload to PASS, and the entry-discriminator proves the lock candidate image shape can execute. The remaining fault boundary is therefore inside the full lock workload after secondary-core entry begins.
- Risks: The leading hypothesis is mixed cacheability/MMU state: the boot CPU runs with caches enabled while secondaries enter the proof without joining the same cacheable memory regime, unlike the accepted controlled workload that used explicit per-core state clean/invalidate handoff. A bounded follow-up must distinguish pre-lock stall, lock-held stall, non-visible progress, and cache-regime invalidity before accepting any Pi 5 lock/coherence claim. Scheduler migration, shared run queues, cross-core wakeups, IPIs, userspace, descriptors, filesystem, networking, SSH, shell behavior, UART interrupts, RP1/PCIe, and DMA/cache-coherent driver policy remain deferred.

## 2026-05-24 - Pi 5 SMP Lock Proof Classified As Invalid Mixed Cache/MMU Regime

- Status: interim hardware decision for the in-progress `phase6-pi5-smp-lock-cache-coherence-proof-20260524` task. The Pi 5 lock proof is not accepted as a passing hardware proof.
- Context: The previous hardware run showed the full lock proof reached secondary entry but stalled before reports or a final invariant. A bounded discriminator was required to separate pre-lock stall, lock-held stall, non-visible progress, and cache-regime invalidity.
- Decision: Add diagnostic-only lock/cache-regime wait observations to the Pi 5 proof and classify the current physical proof setup as `pi5-smp-lock-cache-coherence-invalid-mixed-cache-mmu-regime`. The boot CPU reports `boot-sctlr-el2=0x0000000030c51835` and `boot-cacheable-mmu=true`, while logical cores 1, 2, and 3 report `diag-sctlr-el2=0x0000000030c50830` and `diag-cacheable-mmu=false` before first lock attempt.
- Evidence level: static source inspection, fmt/lint/typecheck, no_std unit tests, QEMU/substitute smoke, focused QEMU SMP contention smoke, image/archive inspection, serialized Pi 5 hardware run under hardwareTestLock, TFTP/archive evidence, cursor-valid serial hardware output, and restore proof.
- Validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 102 no_std tests, `scripts/qemu-smoke.sh`, `scripts/qemu-smp-lock-contention-smoke.sh`, and `scripts/rpi5-archive-review.sh target/talos-rpi5-smp-lock-cache-coherence-boot.tar.gz` passed. TFTP served the 95,608-byte discriminator kernel twice before restore, and the pre-run 82,045-byte boot snapshot was restored.
- Rationale: The accepted secondary workload can complete using explicit per-core clean/invalidate handoff, but the generic lock proof cannot claim physical cache/coherence behavior while the boot CPU and secondaries are in different cacheable MMU regimes. The discriminator proves the failure boundary before the first generic lock acquisition.
- Risks: This does not prove the generic `SpinLock<T>` on physical Pi 5. A future supervisor-planned task must either bring secondary cores into the same cacheable translation regime before lock contention or close the hardware proof as a documented cache-regime blocker. Scheduler migration, shared run queues, cross-core wakeups, IPIs, userspace, descriptors, filesystem, networking, SSH, shell behavior, UART interrupts, RP1/PCIe, and DMA/cache-coherent driver policy remain deferred.

## 2026-05-25 - Secondary Cacheable MMU Handoff Core Accepted

- Status: accepted as the narrow implementation prerequisite for rerunning the Pi 5 SMP lock proof. No Pi 5 hardware run, scheduler migration, shared run queue, cross-core wakeup, IPI, userspace, descriptor, filesystem, networking, SSH, shell, UART interrupt, RP1/PCIe, or DMA behavior was added.
- Context: The Pi 5 lock proof was classified as invalid because secondaries reached the proof with SCTLR_EL2.C clear while the boot CPU had the accepted cacheable EL2 stage-1 regime active. The source-inventory task required fixing that proof precondition without hiding cache maintenance inside `SpinLock<T>`.
- Decision: Add an AArch64 `El2Stage1CacheRegime` boundary and Pi 5 lock-proof handoff path. The boot CPU publishes MAIR_EL2/TCR_EL2/TTBR0_EL2/SCTLR_EL2 after normal cache/MMU enablement; secondaries install that plan, invalidate translations/caches with barriers, enable SCTLR_EL2.M/I/C, and park before lock access if the handoff cannot establish cacheable-MMU state.
- Evidence level: static source inspection, fmt/lint/typecheck, no_std unit tests, QEMU/substitute smoke, focused QEMU SMP contention transcript, Pi 5 image-generation inspection, whitespace inspection, and mdBook availability inspection.
- Validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 102 no_std tests, `scripts/qemu-smoke.sh`, `scripts/qemu-smp-lock-contention-smoke.sh`, `scripts/rpi5-smp-lock-cache-coherence-image.sh`, and `git diff --check` passed. The generated Pi 5 lock image was 96,792 bytes with SHA256 `acc334beb5bc82555d6d4c3309d3e24b0b669593768cb9d01e479bc40e350e40`. `mdbook` was unavailable in the container.
- Rationale: Physical shared cached atomics require participating cores to use compatible cacheable normal-memory attributes before generic lock contention begins. The handoff makes that proof precondition explicit while preserving the generic lock contract.
- Risks: This is local implementation evidence only. The follow-up Pi 5 handoff proof must still capture serialized hardware output showing secondary cacheable-MMU state before the lock attempt and classify whether the physical lock proof can resume.

## 2026-05-25 - Pi 5 SMP Lock Cache/Coherence Proof Accepted

- Status: accepted as the physical Pi 5 cache/coherence proof for the first Milestone 6.2 SMP-safe primitive. No scheduler migration, shared run queue, cross-core wakeup, IPI, userspace, descriptor, filesystem, networking, SSH, shell, UART interrupt, RP1/PCIe, or DMA behavior was added.
- Context: The original Pi 5 lock proof first failed because secondaries reached the proof without the boot CPU's cacheable EL2 stage-1 regime, then failed its report invariant after the cacheable-MMU handoff. The accepted follow-up fixed report publication without changing the generic `SpinLock<T>` contract and reran the serialized hardware proof.
- Decision: Accept `phase6-pi5-smp-lock-cache-coherence-final-proof-20260525`. The accepted proof requires all secondary participants to report the cacheable-MMU handoff before generic lock access, then contend on the same `SpinLock<T>` counter invariant as the QEMU smoke.
- Evidence level: static inspection, fmt/lint/typecheck, no_std unit tests, QEMU/substitute smoke, focused QEMU SMP contention transcript, image/archive inspection, serialized Pi 5 TFTP/archive proof, cursor-valid serial hardware output, restore proof, and whitespace inspection.
- Validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test` with 103 no_std tests, `scripts/qemu-smoke.sh`, `scripts/qemu-smp-lock-contention-smoke.sh`, `scripts/rpi5-smp-lock-cache-coherence-image.sh`, `scripts/rpi5-archive-review.sh target/talos-rpi5-smp-lock-cache-coherence-boot.tar.gz`, and `git diff --check` passed. `mdbook` was unavailable in the container. The accepted archive SHA256 is `73041969803f1153a4277d0f56700df08022451a486cd7088ceabe654e953910`; kernel SHA256 is `e28596b5f259775c4c239c3e18b57e3d61d24ff453aa3c762c879e38075f7278`; kernel size is 96,824 bytes.
- Rationale: The cursor-valid Pi 5 transcript shows boot CPU and logical cores 1, 2, and 3 in the same cacheable-MMU regime, each secondary with stable identity/report fields and `ok=true`, final `counter=192 expected=192 participants=3 errors=0`, `mixed-cache-mmu=false`, `classification=pi5-smp-lock-cache-coherence-complete`, and `PASS`. This closes the physical lock/cache-coherence proof without broadening into scheduler behavior.
- Risks: The proof is a diagnostic shared-lock workload, not an SMP scheduler. Shared run queues, cross-core wakeups, IPIs, multi-core preemption, concurrent console ownership, userspace, descriptors, filesystem, networking, SSH, shell behavior, RP1/PCIe, UART interrupt ownership, and DMA/cache-coherent driver policy remain deferred.

## 2026-05-25 - Phase 6.3 Cross-Core Wakeup/IPI Source Inventory Accepted

- Status: accepted as the source inventory and first proof strategy for cross-core wakeup and IPI work. No SGI implementation, scheduler migration, shared run queue, remote wakeup, task migration, Pi 5 hardware publish/test, userspace, descriptor, filesystem, networking, SSH, shell, UART interrupt, RP1/PCIe, or DMA behavior was added.
- Context: Phase 6.3 already accepted CPU-local scheduler ownership and QEMU per-core scheduler ownership evidence. Before any scheduler uses remote wakeups, Talos needs a raw interrupt-delivery proof and a scheduler-facing ownership contract.
- Decision: Split raw IPI delivery from scheduler wakeup. The next bounded task should be `phase6-qemu-cross-core-ipi-delivery-smoke-20260525`, adding the minimal GICv2 SGI surface and QEMU evidence for CPU 0 sending a diagnostic SGI to secondary logical CPUs. A later serialized Pi 5 proof is required before physical scheduler wakeups can depend on SGIs.
- Evidence level: static source inspection, accepted task/evidence review, whitespace inspection, and mdBook availability inspection.
- Validation: `git status --short` was clean before edits, `git diff --check` passed, and `mdbook` was unavailable in the container. Rust fmt/tests were not required because this was documentation and durable-state work only.
- Rationale: The current scheduler owns CPU-local queues only. Raw SGI delivery must be classified separately from remote enqueue/wake-list ownership so a later scheduler task cannot accidentally mutate another CPU's local scheduler state without a proven lock and memory-ordering model.
- Risks: QEMU SGI proof will remain substitute evidence only. Pi 5 GIC-400 SGI target-list mapping, concurrent console output, shared wake lists, remote enqueue, task migration, multi-core preemption, userspace, descriptors, filesystem, networking, SSH, shell behavior, RP1/PCIe, UART interrupt ownership, and DMA/cache-coherent driver policy remain deferred.

## 2026-05-25 - Phase 6.3 Scheduler Migration First Slice Accepted

- Status: accepted as the first Milestone 6.3 scheduler-migration slice checkpoint. No Rust code, script, boot image, hardware publish/test, SGI implementation, production scheduler wakeup, shared run queue, task migration, Phase 7, filesystem, networking, SSH, shell, RP1/PCIe, UART interrupt, or DMA behavior was added.
- Context: The first slice now has an accepted readiness inventory, per-core scheduler ownership implementation, QEMU per-core ownership smoke, and cross-core wakeup/IPI source inventory. This checkpoint was required before the work could drift into broader scheduler migration or later roadmap phases.
- Decision: Keep the accepted topology as CPU-local scheduler ownership for now. CPU 0 remains the only production scheduler owner; secondary schedulers are diagnostic/deferred owners; raw IPI delivery is the next implementation boundary. The next bounded task should be `phase6-qemu-cross-core-ipi-delivery-smoke-20260525`, not Pi 5 scheduler proof, remote wake queues, or task migration.
- Evidence level: static documentation reconciliation, accepted task/evidence review, QEMU/substitute transcript review, whitespace inspection, and mdBook availability inspection.
- Validation: `git status --short` was clean before edits and `git diff --check` passed after edits. `mdbook` was unavailable in the container. Rust fmt/tests were not required because this checkpoint changed only Markdown documentation and durable task state.
- Rationale: Closing the slice records that CPU-local ownership and raw IPI delivery must be proven separately from scheduler wakeups. This prevents the accepted `SpinLock<T>` and QEMU per-core ownership evidence from being treated as permission to mutate remote scheduler queues or to start Phase 7/networking/shell work.
- Risks: The slice still has no shared run queue, global task lookup, remote enqueue queue, wake list, task migration, load balancing, production secondary scheduler dispatch, Pi 5 SGI proof, multi-core preemption, userspace, descriptors, filesystem, networking, SSH, shell behavior, RP1/PCIe, UART interrupt ownership, or DMA/cache-coherent driver policy.

## 2026-05-25 - Phase 6.3 Remote Wake-Request Ownership Accepted

- Status: accepted as the source-backed scheduler-facing ownership model for the first remote wakeup implementation proof. No Rust code, script, boot image, hardware publish/test, direct remote enqueue, shared run queue, task migration, production secondary scheduler dispatch, Phase 7, filesystem, networking, SSH, shell, RP1/PCIe, UART interrupt, or DMA behavior was added.
- Context: QEMU and Pi 5 raw SGI delivery are accepted, but raw interrupt delivery does not define who may publish scheduler work, who may mutate local runnable queues, or what an IPI handler may do.
- Decision: Select a bounded per-target remote wake-request list. A remote sender may publish a bounded request for a scheduler `TaskId` into the target CPU's request list and signal with SGI INTID 1. The target CPU owns request consumption and any later local scheduler effect. Direct remote mutation of another CPU's `RunnableQueue` remains forbidden.
- Evidence level: static source inspection, accepted QEMU/Pi 5 raw IPI evidence review, architecture documentation review, whitespace inspection, and mdBook availability inspection.
- Validation: `git status --short` was clean before edits and `git diff --check` passed after edits. `mdbook` was unavailable in the container. Rust fmt/tests were not required because this task changed only Markdown documentation and durable task state.
- Rationale: A request list preserves CPU-local scheduler ownership while creating a bounded bridge between raw SGI delivery and future local scheduler wake processing. It avoids prematurely introducing shared run queues, global task lookup, migration, or production secondary dispatch.
- Risks: The model is not yet implemented. The next bounded task must prove request publication, IPI signaling, target-side observation, target-owned consumption, duplicate semantics, and rejected cross-owner runnable-queue mutation under QEMU before any Pi 5 scheduler-facing wakeup proof or broader scheduler migration proceeds.

## 2026-05-25 - Phase 6.3 Cross-Core Wakeup Closeout Accepted

- Status: accepted as the cross-core wakeup closeout checkpoint after raw QEMU/Pi 5 SGI delivery, remote wake-request ownership, and QEMU remote wake-request evidence. No Rust code, script, boot image, hardware publish/test, broader scheduler migration, shared run queue, task migration, production secondary scheduler dispatch, Phase 7, filesystem, networking, SSH, shell, RP1/PCIe, UART interrupt, or DMA behavior was added by the checkpoint.
- Context: Raw SGI delivery is accepted on QEMU and Pi 5, and the first scheduler-facing remote wake-request model is accepted and proven under QEMU. A checkpoint is required before carrying that model to physical scheduler-facing wakeup evidence or drifting into broader scheduler migration.
- Decision: Keep CPU-local scheduler ownership and the bounded per-target `RemoteWakeQueue` model. The next bounded task should be `phase6-pi5-remote-wakeup-request-proof-20260525`, a serialized hardware proof of the accepted QEMU model. Do not start shared scheduler metadata, shared run queues, task migration, production secondary dispatch, multi-core preemption, or later roadmap phases without a separate durable task.
- Evidence level: static documentation reconciliation, accepted task/evidence review, QEMU/substitute transcript review, Pi 5 serial hardware evidence review, whitespace inspection, and mdBook availability inspection.
- Validation: `git status --short` was clean before edits and `git diff --check` passed after edits. `mdbook` was unavailable in the container. Rust fmt/tests and hardware runs were not required because this checkpoint changed only Markdown documentation and durable task state.
- Rationale: The accepted QEMU remote wake-request smoke proves request publication, duplicate coalescing, SGI signaling, target-owned observation/EOI/consumption, and cross-owner mutation rejection at substitute level. The remaining useful next discriminator is the same scheduler-facing path on Pi 5 hardware, not a new topology or broader scheduler migration.
- Risks: The Pi 5 scheduler-facing wake-request path is not yet proven. Shared run queues, global task lookup, remote enqueue queues, task migration, load balancing, work stealing, production secondary scheduler dispatch, multi-core preemption, userspace, descriptors, filesystem, networking, SSH, shell behavior, RP1/PCIe, UART interrupt ownership, and DMA/cache-coherent driver policy remain deferred.

## 2026-05-25 - Phase 6.3 Pi 5 Remote Wake-Request Proof Accepted

- Status: accepted as serialized Pi 5 hardware evidence for the bounded scheduler-facing remote wake-request model. No local runnable transition from remote requests, shared run queue, global task lookup, remote enqueue queue, task migration, load balancing, work stealing, production secondary scheduler dispatch, multi-core preemption, Phase 7, filesystem, networking, SSH, shell, RP1/PCIe, UART interrupt, or DMA behavior was added.
- Context: QEMU already proved the remote wake-request model at substitute level, and Pi 5 raw SGI delivery was accepted. The remaining question was whether the scheduler-facing request publication/signaling/target-consumption path also holds on physical Pi 5 hardware.
- Decision: Accept `phase6-pi5-remote-wakeup-request-proof-20260525`. The accepted Pi 5 run proves CPU 0 request publication for logical CPUs 1, 2, and 3, duplicate coalescing for target 1, SGI INTID 1 signaling, target-side observation/EOI, target-owned request consumption, queue drain, rejected cross-owner local scheduler mutation, and deferred secondary production dispatch. The next bounded task should be `phase6-target-owned-wake-consumption-contract-20260525`, not broader scheduler migration.
- Evidence level: fmt/lint, unit tests, QEMU/substitute gates, image/archive inspection, lab-controller API, TFTP fetch evidence, cursor-valid serial hardware output, artifact digests, classification, and restore proof.
- Validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test`, `scripts/qemu-remote-wakeup-request-smoke.sh`, `scripts/qemu-cross-core-ipi-delivery-smoke.sh`, `scripts/qemu-smoke.sh`, the focused Pi 5 boot-tree script, archive review, and serialized Pi 5 hardware run passed. `mdbook` remained unavailable in the container.
- Rationale: This accepts the narrow bridge from raw SGI delivery to scheduler-facing request mailboxes while preserving CPU-local scheduler ownership. It deliberately stops before local runnable transitions from remote requests, because that requires a target-owned wake-consumption contract.
- Risks: Local blocked-to-runnable transitions, shared run queues, task lookup beyond scheduler-local diagnostic IDs, production secondary dispatch, task migration, multi-core preemption, userspace, descriptors, filesystem, networking, SSH, shell behavior, RP1/PCIe, UART interrupt ownership, and DMA/cache-coherent driver policy remain deferred.

## 2026-05-25 - Phase 6.3 Pi 5 Remote Wake To Local Runnable Proof Accepted

- Status: accepted as serialized Pi 5 hardware evidence for target-owned remote wake consumption into a target-local runnable queue. No shared run queue, global task lookup, remote enqueue queue, task migration, load balancing, work stealing, production secondary scheduler dispatch, multi-core preemption, Phase 7, filesystem, networking, SSH, shell, RP1/PCIe, UART interrupt, or DMA behavior was added.
- Context: QEMU already proved that, after a target CPU drains its own remote wake request outside IPI context, only that target may transition a matching local blocked task to runnable. The remaining question was whether the same scheduler-facing invariant held on physical Pi 5 hardware after the accepted SGI and remote wake-request proofs.
- Decision: Accept phase6-pi5-remote-wake-to-local-runnable-proof-20260525. The accepted Pi 5 run proves request publication, duplicate request coalescing, SGI INTID 1 signaling, target-side observation/EOI, target-owned request drain, local Blocked -> Runnable transitions for diagnostic tasks 201/202/203 on logical CPUs 1/2/3, duplicate local enqueue rejection, drained request queues, cross-owner mutation rejection, deferred secondary production dispatch, and final classification=pi5-remote-wake-to-local-runnable-complete.
- Evidence level: fmt/lint, unit tests, QEMU/substitute gates, image/archive inspection, lab-controller API, TFTP fetch evidence, cursor-valid serial hardware output, artifact digests, classification, restore proof, and whitespace inspection.
- Validation: cargo fmt --all -- --check, cargo -Zjson-target-spec test with 116 no_std tests, scripts/qemu-remote-wake-to-local-runnable-smoke.sh, scripts/qemu-remote-wakeup-request-smoke.sh, scripts/qemu-smoke.sh, scripts/rpi5-image.sh, focused Pi 5 boot-tree generation, scripts/rpi5-archive-review.sh target/talos-rpi5-remote-wake-to-local-runnable-boot.tar.gz, serialized Pi 5 hardware run, and git diff --check passed. mdbook remained unavailable in the container.
- Rationale: This closes the narrow bridge from remote request mailboxes to target-owned local wake consumption without granting remote CPUs permission to mutate another CPU's runnable queue. The IPI hot path remains bounded to observation/accounting/EOI; the local runnable mutation happens only after target-side drain.
- Risks: This is still diagnostic scheduler integration, not production SMP scheduling. Shared scheduler metadata, global task lookup, production secondary dispatch, task migration, multi-core preemption, userspace, descriptors, filesystem, networking, SSH, shell behavior, RP1/PCIe, UART interrupt ownership, and DMA/cache-coherent driver policy remain deferred.

## 2026-05-25 - Phase 6.3 Remote Wakeup Scheduler Integration Closeout Accepted

- Status: accepted as the Phase 6.3 closeout checkpoint for raw SGI delivery, bounded remote wake-request publication/consumption, and target-owned local runnable transitions. No Rust implementation, boot image, hardware publish/test, production secondary scheduler dispatch, shared run queue, task migration, multi-core preemption, Phase 7, filesystem, networking, SSH, shell, RP1/PCIe, UART interrupt, or DMA behavior was added by the checkpoint.
- Context: QEMU and Pi 5 now both prove the scheduler-facing remote wake path through target-owned local Blocked -> Runnable transitions for diagnostic tasks, while production secondary dispatch remains explicitly deferred.
- Decision: Accept phase6-remote-wakeup-scheduler-integration-closeout-20260525. The next bounded task should be phase6-production-secondary-scheduler-dispatch-source-inventory-20260525, a documentation/source-inventory and contract task only. Do not implement production secondary dispatch, shared run queues, task migration, multi-core preemption, or later roadmap work without a separate durable task.
- Evidence level: static inspection, static review of task records, QEMU transcripts, Pi 5 evidence summaries, architecture docs, roadmap, decision log, documentation updates, and whitespace inspection.
- Validation: git status --short before edits was clean and git diff --check passed. mdbook remained unavailable in the container. Rust fmt/tests and hardware runs were not required because this checkpoint changed only Markdown documentation and durable task state.
- Rationale: The accepted evidence is sufficient to plan production secondary scheduler dispatch from a source-backed contract, but not to implement it. The checkpoint prevents the diagnostic remote wake bridge from being treated as a shared scheduler topology.
- Risks: Production secondary dispatch still needs an explicit ownership and validation contract for target-local runnable queues, context-switch boundaries, timer/preemption state, IPI observation, remote wake drains, console/output ownership, and failure diagnostics. Shared run queues, migration, load balancing, multi-core preemption, userspace, filesystem, networking, SSH, shell, RP1/PCIe, UART interrupt ownership, and DMA/cache policy remain deferred.

## 2026-05-25 - Phase 6.3 Production Secondary Dispatch Source Inventory Accepted

- Status: accepted as the documentation/source-inventory and contract for the first production secondary scheduler dispatch slice. No Rust implementation, boot image, hardware publish/test, shared run queue, global task lookup, task migration, multi-core preemption, Phase 7, filesystem, networking, SSH, shell, RP1/PCIe, UART interrupt, or DMA behavior was added.
- Context: Remote wakeup scheduler integration is accepted through target-owned local runnable transitions, but secondary CPUs still only prove diagnostic/deferred scheduler ownership. Before any secondary enters production scheduler work, Talos needs a bounded ownership and validation contract.
- Decision: Accept phase6-production-secondary-scheduler-dispatch-source-inventory-20260525. The first implementation may dispatch only explicitly seeded CPU-local diagnostic kernel threads on secondary CPUs, from normal secondary control flow, with local PerCoreScheduler ownership, per-core current-task reporting, local runnable transitions, and bounded dispatch counters. Remote wake consumption remains target-owned. The next bounded task should be phase6-production-secondary-dispatch-core-20260525.
- Evidence level: static inspection, static source review, accepted task/evidence review, architecture documentation update, roadmap update, decision-log update, whitespace inspection, and mdBook availability inspection.
- Validation: git status --short before edits was clean, git diff --check passed after edits, and mdbook build passed. Rust fmt/tests and hardware runs were not required because this task changed only Markdown documentation and durable task state.
- Rationale: This contract turns the accepted diagnostic scheduler surfaces into a narrow production-secondary entry plan without treating remote wake evidence as permission for shared scheduler mutation. It keeps IPI handlers hot-path bounded and requires QEMU and serialized Pi 5 proof before the capability is accepted.
- Risks: The contract is not an implementation. Shared run queues, global task lookup, remote enqueue queues, task migration, load balancing, work stealing, multi-core preemption, userspace, descriptors, filesystem, networking, SSH, shell behavior, RP1/PCIe, UART interrupt ownership, and DMA/cache policy remain deferred.

## 2026-05-25 - Phase 6.3 QEMU Production Secondary Dispatch Accepted

- Status: accepted as QEMU substitute evidence for the first production secondary dispatch slice. No Pi 5 hardware claim, shared run queue, global task lookup, task migration, multi-core preemption, Phase 7, filesystem, networking, SSH, shell, RP1/PCIe, UART interrupt, or DMA behavior was added.
- Context: The production secondary dispatch core introduced the explicit `SecondaryProductionDiagnostic` role and local dispatch API, but required QEMU evidence before any serialized Pi 5 hardware proof could start.
- Decision: Accept `phase6-qemu-production-secondary-dispatch-smoke-20260525`. The accepted QEMU run starts logical CPUs 1, 2, and 3 through PSCI, has each secondary enter `SecondaryProductionDiagnostic`, dispatch three CPU-local diagnostic tasks, publish stable current-task/local-queue/dispatch-counter snapshots, and reject cross-owner local queue and production-dispatch attempts.
- Evidence level: static source inspection, fmt/lint/typecheck, no_std unit tests, QEMU/substitute transcript, retained QEMU regression gates, architecture/roadmap/task documentation, mdBook validation, and whitespace inspection.
- Validation: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test`, `scripts/qemu-production-secondary-dispatch-smoke.sh`, `scripts/qemu-smoke.sh`, `scripts/qemu-per-core-scheduler-ownership-smoke.sh`, `scripts/qemu-remote-wake-to-local-runnable-smoke.sh`, `mdbook build`, and `git diff --check` passed. The focused transcript is `target/qemu-production-secondary-dispatch-smoke.log` with classification `qemu-production-secondary-dispatch-complete`.
- Rationale: This proves the first production secondary dispatch behavior at substitute level while preserving CPU-local scheduler ownership. It deliberately keeps local dispatch separate from remote wake request publication/consumption and forbids cross-owner local scheduler mutation.
- Risks: QEMU evidence is not physical Pi 5 evidence. Serialized Pi 5 production secondary dispatch proof remains required before the capability can be treated as hardware accepted. Shared scheduler metadata, shared run queues, global task lookup, migration, load balancing, multi-core preemption, userspace, descriptors, filesystem, networking, SSH, shell behavior, RP1/PCIe, UART interrupt ownership, and DMA/cache policy remain deferred.

## 2026-05-25 - Phase 6.3 Pi 5 Production Secondary Dispatch Accepted

- Status: accepted as serialized Pi 5 hardware evidence for the first production secondary dispatch slice. No shared run queue, global task lookup, remote enqueue queue, task migration, load balancing, work stealing, multi-core preemption, Phase 7, filesystem, networking, SSH, shell, RP1/PCIe, UART interrupt ownership, or DMA behavior was added.
- Context: QEMU already proved the explicit production-secondary diagnostic role and CPU-local dispatch API. The remaining question was whether the same per-core current-task, local runnable, dispatch-counter, and cross-owner rejection invariants held on physical Pi 5 hardware after the accepted cacheable secondary MMU handoff.
- Decision: Accept phase6-pi5-production-secondary-dispatch-proof-20260525. The accepted Pi 5 run starts logical CPUs 1, 2, and 3 through PSCI, has each secondary enter SecondaryProductionDiagnostic, dispatch three CPU-local diagnostic tasks, publish stable current-task/local-queue/dispatch-counter snapshots, and reject cross-owner local queue and production-dispatch attempts. The next bounded task should be the production secondary dispatch closeout checkpoint, not shared scheduler metadata or broader migration.
- Evidence level: static source inspection, fmt/lint/typecheck, no_std unit tests, QEMU/substitute transcript, image/archive inspection, lab-controller API, TFTP fetch evidence, cursor-valid serial hardware output, artifact digests, classification, restore proof, and whitespace inspection.
- Validation: cargo fmt --all -- --check, cargo -Zjson-target-spec test with 119 no_std tests, scripts/qemu-production-secondary-dispatch-smoke.sh, scripts/qemu-smoke.sh, scripts/rpi5-image.sh, scripts/rpi5-production-secondary-dispatch-boot-tree.sh, scripts/rpi5-archive-review.sh target/talos-rpi5-production-secondary-dispatch-boot.tar.gz, serialized Pi 5 hardware run, and git diff --check passed. mdbook build passed after the documentation update.
- Rationale: This accepts only CPU-local production dispatch on secondary cores for explicitly seeded diagnostic kernel threads. It does not grant remote CPUs permission to mutate another CPU's local scheduler or introduce a shared scheduler topology.
- Risks: The capability is still diagnostic and CPU-local. Shared scheduler metadata, shared run queues, global task lookup, migration, load balancing, work stealing, multi-core preemption, userspace, descriptors, filesystem, networking, SSH, shell behavior, RP1/PCIe, UART interrupt ownership, and DMA/cache policy remain deferred.

## 2026-05-25 - Phase 6.3 Production Secondary Dispatch Closeout Accepted

- Status: accepted as the closeout checkpoint for the CPU-local production secondary dispatch slice. No Rust implementation, boot image, hardware publish/test, shared scheduler metadata, shared run queue, global task lookup, task migration, multi-core preemption, Phase 7, filesystem, networking, SSH, shell, RP1/PCIe, UART interrupt ownership, or DMA behavior was added by the checkpoint.
- Context: The source inventory, scheduler-core implementation, QEMU substitute proof, and serialized Pi 5 hardware proof now all agree that logical CPUs 1, 2, and 3 can run explicitly seeded CPU-local diagnostic kernel threads through SecondaryProductionDiagnostic while rejecting cross-owner local scheduler mutation.
- Decision: Accept phase6-production-secondary-dispatch-closeout-checkpoint-20260525. The next bounded task should be phase6-shared-scheduler-metadata-source-inventory-20260525, a documentation/source-inventory and contract task only. Do not implement shared scheduler metadata, shared run queues, migration, load balancing, multi-core preemption, or later roadmap work without a separate durable task.
- Evidence level: static inspection, accepted task/evidence review, scheduler architecture review, roadmap update, decision-log update, mdBook validation, and whitespace inspection.
- Validation: git status --short before edits was clean, git diff --check passed after edits, and mdbook build passed. Rust fmt/tests and hardware runs were not required because this checkpoint changed only Markdown documentation and durable task state.
- Rationale: Closing this slice prevents a proven CPU-local diagnostic dispatch path from being treated as permission for shared scheduler topology. The next useful boundary is the metadata contract needed before any future shared run queue or task migration work.
- Risks: The accepted capability remains diagnostic and CPU-local. There is no global task registry, shared run queue, migration policy, load balancer, work stealing, remote enqueue authority, or multi-core preemption policy. Userspace, descriptors, filesystem, networking, SSH, shell behavior, RP1/PCIe, UART interrupt ownership, and DMA/cache policy remain deferred.

## 2026-05-25 - Phase 6.3 Shared Scheduler Metadata Source Inventory Accepted

- Status: accepted as the documentation/source-inventory and ownership contract
  for the first shared scheduler metadata slice. No Rust implementation, boot
  image, hardware publish/test, shared run queue, remote enqueue, task
  migration, load balancing, multi-core preemption, Phase 7, filesystem,
  networking, SSH, shell, RP1/PCIe, UART interrupt ownership, or DMA behavior
  was added.
- Context: Production secondary dispatch is accepted only for explicitly seeded
  CPU-local diagnostic kernel threads. Before any future shared scheduler
  topology, Talos needs a metadata boundary that can name a task across cores
  without allowing remote CPUs to mutate target-local scheduler state.
- Decision: Accept phase6-shared-scheduler-metadata-source-inventory-20260525.
  The first implementation should add only metadata types and local-owner APIs
  for task ID, owning CPU, task state, optional process owner, stack bounds,
  current/runnable membership, and stale snapshot rejection. The next bounded
  task should be phase6-shared-scheduler-metadata-core-20260525.
- Evidence level: static inspection, static source review of scheduler/SMP/QEMU
  and Pi 5 target paths, accepted Phase 6.3 task/evidence review,
  architecture documentation update, roadmap update, decision-log update,
  mdBook validation, and whitespace inspection.
- Validation: git status --short before edits showed a clean Talos repo, git
  diff --check passed after edits, and mdbook build passed. Rust fmt/tests and
  hardware runs were not required because this task changed only Markdown
  documentation and durable task state.
- Rationale: The metadata contract lets later code identify task ownership
  across CPUs while preserving the accepted CPU-local runnable queue and
  target-owned wake-consumption rules. It keeps IPI and timer context as
  bounded observation paths, not scheduler mutation paths.
- Risks: This contract is not an implementation. Shared run queues, global task
  lookup with mutation authority, remote enqueue, task migration, load
  balancing, work stealing, multi-core preemption, userspace, descriptors,
  filesystem, networking, SSH, shell behavior, RP1/PCIe, UART interrupt
  ownership, and DMA/cache policy remain deferred.

## 2026-05-25 - Phase 6.3 Shared Scheduler Metadata Core Accepted

- Status: accepted as the first shared scheduler metadata implementation
  slice. The change adds Rust data structures and tests only; no boot archive,
  Pi 5 hardware publish/test, shared run queue, remote enqueue, task migration,
  load balancing, multi-core preemption, Phase 7, filesystem, networking, SSH,
  shell, RP1/PCIe, UART interrupt ownership, or DMA behavior was added.
- Context: The accepted source inventory selected read-oriented metadata that
  can name CPU-local scheduler tasks across cores while preserving
  target-owned local runnable queues and production secondary dispatch
  invariants.
- Decision: Accept phase6-shared-scheduler-metadata-core-20260525.
  SchedulerTaskSnapshot records task ID, owning CPU, state, optional process
  owner, kernel-stack bounds, owner-local current/runnable membership, and a
  generation. SharedSchedulerMetadata exposes owner-only registration/refresh,
  read-only lookup, duplicate/unknown/invalid-owner/stale-snapshot outcomes,
  and SharedSchedulerMetadataLock names the accepted SpinLock boundary for
  future shared table use.
- Evidence level: static inspection, no_std unit tests, retained QEMU
  substitute smokes, mdBook validation, and whitespace inspection.
- Validation: cargo fmt --all -- --check passed; cargo -Zjson-target-spec test
  passed with 125 no_std tests; scripts/qemu-smoke.sh,
  scripts/qemu-per-core-scheduler-ownership-smoke.sh,
  scripts/qemu-remote-wake-to-local-runnable-smoke.sh, and
  scripts/qemu-production-secondary-dispatch-smoke.sh passed; git diff --check
  passed; mdbook build passed.
- Rationale: The metadata table gives future QEMU and Pi 5 proofs a bounded
  identity surface without granting remote mutation authority over local
  scheduler state. Keeping the table separate from RunnableQueue preserves the
  CPU-local dispatch topology.
- Risks: The metadata is not a global mutable task registry. Shared run queues,
  remote enqueue, migration, load balancing, work stealing, multi-core
  preemption, userspace, descriptors, filesystem, networking, SSH, shell
  behavior, RP1/PCIe, UART interrupt ownership, and DMA/cache policy remain
  deferred.

## 2026-05-25 - Phase 6.3 QEMU Shared Scheduler Metadata Evidence Accepted

- Status: accepted as QEMU substitute evidence for the first shared scheduler
  metadata invariant. No Pi 5 hardware claim, shared run queue, remote enqueue,
  task migration, load balancing, multi-core preemption, Phase 7, filesystem,
  networking, SSH, shell, RP1/PCIe, UART interrupt ownership, or DMA behavior
  was added.
- Context: The shared scheduler metadata core had static, unit-test, and
  retained QEMU-smoke evidence, but still needed a focused SMP transcript
  proving all QEMU logical CPUs could publish/query the table through the
  accepted lock boundary.
- Decision: Accept phase6-qemu-shared-scheduler-metadata-smoke-20260525. The
  focused QEMU diagnostic starts logical CPUs 0 through 3, publishes owner
  metadata for task IDs 101, 201, 301, and 401, proves owner-task and boot-task
  lookup, rejects cross-owner local scheduler mutation, rejects cross-owner
  metadata publication, preserves target-owned local runnable queues, and
  reports classification=qemu-shared-scheduler-metadata-complete.
- Evidence level: QEMU/substitute transcript, fmt/lint/typecheck, no_std unit
  tests, retained QEMU substitute gates, mdBook validation, and whitespace
  inspection.
- Validation: scripts/qemu-shared-scheduler-metadata-smoke.sh passed with
  target/qemu-shared-scheduler-metadata-smoke.log; full acceptance validation
  also includes cargo fmt --all -- --check, cargo -Zjson-target-spec test,
  scripts/qemu-smoke.sh, scripts/qemu-production-secondary-dispatch-smoke.sh,
  scripts/qemu-remote-wake-to-local-runnable-smoke.sh, git diff --check, and
  mdbook build.
- Rationale: This proves the metadata table under QEMU SMP while keeping
  ownership read-oriented and separate from local runnable queue mutation. It
  creates substitute evidence for the next serialized Pi 5 proof without
  broadening scheduler topology.
- Risks: QEMU evidence is not physical Pi 5 evidence. Serialized Pi 5 shared
  scheduler metadata proof remains required before checkpoint work. Shared run
  queues, global mutable task lookup, migration, load balancing, work stealing,
  remote enqueue, multi-core preemption, userspace, descriptors, filesystem,
  networking, SSH, shell behavior, RP1/PCIe, UART interrupt ownership, and
  DMA/cache policy remain deferred.

## 2026-05-26 - Phase 6.3 Pi 5 Secondary Scheduler Service Loop Evidence Accepted

- Status: accepted as serialized Pi 5 hardware evidence for the secondary
  scheduler service-loop invariant. No shared run queue, remote enqueue, task
  migration, load balancing, work stealing, remote reschedule, multi-core
  preemption, Phase 7, filesystem, networking, SSH, shell, RP1/PCIe, UART
  interrupt ownership, or DMA behavior was added.
- Context: The QEMU substitute proof was accepted at f6eefd2, but the
  secondary service loop still needed physical evidence after the accepted Pi 5
  secondary cacheable-MMU handoff.
- Decision: Accept phase6-pi5-secondary-scheduler-service-loop-proof-20260526.
  The focused Pi 5 proof starts logical CPUs 1, 2, and 3, runs one owner-local
  service-loop cycle per secondary, drains target-owned remote wake state,
  dispatches the local diagnostic task, refreshes owner metadata, rejects
  cross-owner and deferred-role use, preserves local queues, and reports
  classification=pi5-secondary-scheduler-service-loop-complete.
- Evidence level: serial hardware boot/output, lab-controller API, TFTP fetch
  proof, image/archive inspection, fmt/lint/typecheck, no_std unit tests, QEMU
  substitute gates, mdBook validation, and whitespace inspection.
- Validation: cargo fmt --all -- --check passed; cargo -Zjson-target-spec test
  passed with 134 no_std tests; scripts/qemu-smoke.sh and
  scripts/qemu-secondary-scheduler-service-loop-smoke.sh passed;
  scripts/rpi5-archive-review.sh
  target/talos-rpi5-secondary-scheduler-service-loop-boot.tar.gz passed; the
  serialized Pi 5 run fetched a 102,824-byte da591740/kernel_2712.img and
  reported PASS before the pre-run snapshot was restored.
- Rationale: Physical Pi 5 evidence closes the service-loop proof gap without
  changing the accepted CPU-local topology. The proof remains a diagnostic
  validation surface until a later task defines a non-diagnostic secondary
  runtime role.
- Risks: The proof covers one bounded owner-local service cycle per secondary,
  not a continuously scheduled multi-core runtime. Shared run queues, global
  mutable task lookup, migration, load balancing, work stealing, remote enqueue,
  multi-core preemption, userspace, descriptors, filesystem, networking, SSH,
  shell behavior, RP1/PCIe, UART interrupt ownership, and DMA/cache policy
  remain deferred.

## 2026-05-26 - Phase 6.3 QEMU Shared Run-Queue Migration Evidence Accepted

- Status: accepted as QEMU substitute evidence for the shared
  run-queue/migration core. No Pi 5 hardware claim, load balancing, work
  stealing, multi-core preemption, Phase 7, filesystem, networking, SSH,
  shell, RP1/PCIe, UART interrupt ownership, or DMA behavior was added.
- Context: The target-independent shared run-queue core was accepted at
  4e69f9d with unit coverage, but still needed a focused QEMU transcript
  proving the source-owner publish and destination-owner consume sequence
  through the implemented core rather than a bypass.
- Decision: Accept phase6-qemu-shared-runqueue-migration-smoke-20260526. The
  focused QEMU diagnostic adds `qemu_shared_runqueue_migration`, publishes task
  107 from source owner 0 to destination owner 1 through
  `SharedRunQueue::publish_migration`, consumes it through
  `SharedRunQueue::consume_for_destination`, proves source-local queue removal,
  shared queue drain, destination-local enqueue, metadata owner transfer, and
  reports classification=qemu-shared-runqueue-migration-complete.
- Evidence level: QEMU/substitute transcript, fmt/lint/typecheck, no_std unit
  tests, retained QEMU substitute smoke, mdBook validation, and whitespace
  inspection.
- Validation: scripts/qemu-shared-runqueue-migration-smoke.sh passed with
  target/qemu-shared-runqueue-migration-smoke.log; full acceptance validation
  also includes cargo fmt --all -- --check, cargo -Zjson-target-spec test,
  scripts/qemu-smoke.sh, git diff --check, and mdbook build.
- Rationale: The diagnostic proves the accepted owner-transfer invariant
  without adding secondary-core orchestration that the target-independent core
  does not need. Keeping it as a named boot scenario and script makes the
  proof reusable as a retained Phase 6.3 regression gate.
- Risks: QEMU evidence is not physical Pi 5 evidence. Serialized Pi 5
  shared run-queue/migration proof remains required before physical acceptance.
  Target selection, load balancing, work stealing, running-task migration,
  multi-core preemption, userspace, descriptors, filesystem, networking, SSH,
  shell behavior, RP1/PCIe, UART interrupt ownership, and DMA/cache policy
  remain deferred.

## 2026-05-27 - Phase 6.3 Pi 5 Shared Run-Queue Migration Evidence Accepted

- Status: accepted as serialized physical Pi 5 evidence for the shared
  run-queue/migration core. No load balancing, work stealing, multi-core
  preemption, Phase 7, filesystem, networking, SSH, shell, RP1/PCIe, UART
  interrupt ownership, or DMA behavior was added.
- Context: The target-independent core and QEMU substitute proof were accepted,
  but physical acceptance required hardwareTestLock serialization, candidate
  identity, TFTP fetch evidence, cursor-valid serial output, classification,
  and restore proof.
- Decision: Accept phase6-pi5-shared-runqueue-migration-proof-20260526. The
  focused Pi 5 diagnostic adds `rpi5_shared_runqueue_migration` and exercises
  the implemented `SharedRunQueue::publish_migration` and
  `SharedRunQueue::consume_for_destination` invariant on physical cores. The
  proof reports participants=4 expected=4, errors=0, lock-available=true, and
  classification=pi5-shared-runqueue-migration-complete.
- Evidence level: serialized Pi 5 hardware boot/output, TFTP fetch evidence,
  archive/kernel digest inspection, QEMU/substitute preservation gates,
  fmt/lint/typecheck, no_std unit tests, mdBook validation, and whitespace
  inspection.
- Validation: local1 served da591740/kernel_2712.img from 10.42.1.4 with
  bytes=102952 before restore; serial reached
  classification=pi5-shared-runqueue-migration-complete and PASS; restore
  returned ok=true with restore-exit.txt equal to 0. Full acceptance validation
  also includes cargo fmt --all -- --check, cargo -Zjson-target-spec test,
  scripts/qemu-smoke.sh, scripts/qemu-shared-runqueue-migration-smoke.sh,
  scripts/rpi5-archive-review.sh, git diff --check, and mdbook build.
- Risks: This is a bounded diagnostic proof of explicit migration handoff. It
  does not accept load-balancing policy, work stealing, running-task migration,
  multi-core timer preemption, userspace, descriptors, filesystem, networking,
  SSH, shell behavior, RP1/PCIe, UART interrupt ownership, or DMA/cache policy.

## 2026-05-27 - Phase 6.3 Load-Balancing Source Inventory Accepted

- Status: accepted as documentation/source inventory before load-balancing
  policy design. No Rust implementation, QEMU run, Pi 5 hardware run,
  load-balancer, work stealing, running-task migration, remote reschedule,
  multi-core preemption, Phase 7, filesystem, networking, SSH, shell, RP1/PCIe,
  UART interrupt ownership, or DMA behavior was added.
- Context: The shared run-queue/migration closeout accepted the owner-transfer
  mechanism and QEMU/Pi 5 proof evidence, but explicitly deferred destination
  selection, fairness/affinity, remote reschedule, and production secondary
  runtime policy.
- Decision: Accept phase6-load-balancing-source-inventory-20260527. The
  inventory names the current scheduler, metadata, wake, timer, SMP, and
  diagnostic surfaces; lists accepted policy inputs and stale/invalid input
  failure modes; and separates target selection, fairness/affinity,
  remote-reschedule notification, and migration mechanism boundaries.
- Evidence level: static inspection, documentation build, and whitespace
  inspection. Hardware was not required because no physical claim changed.
- Validation: git status --short was clean before edits; git diff --check
  passed; mdbook build passed.
- Rationale: A contract-first step is needed because the accepted shared
  run-queue core can move a runnable task after a destination is chosen, but it
  intentionally does not decide which CPU should receive work, when balancing
  should run, or how stale observations should be handled.
- Risks: Talos still lacks per-task affinity, priority/fairness accounting,
  production secondary idle/wake behavior, remote reschedule semantics, and
  multi-core preemption. The next bounded task should be
  phase6-load-balancing-contract-20260527 before implementation.

## 2026-05-27 - Phase 6.3 Multi-Core Preemption Contract Accepted

- Status: accepted as documentation/architecture contract before multi-core
  preemption implementation. No Rust implementation, QEMU run, Pi 5 hardware
  run, direct IRQ/IPI-context scheduling, running-task migration, work
  stealing, general remote reschedule, Phase 7, filesystem, networking, SSH,
  shell, RP1/PCIe, UART interrupt ownership, or DMA behavior was added.
- Context: The accepted source inventory mapped timer IRQ recording,
  owner-local scheduler service, secondary service-loop dispatch, IPI/wake,
  metadata, SharedRunQueue, and load-balancing boundaries. A contract was
  needed before code could add multi-core preemption state.
- Decision: Accept phase6-multicore-preemption-contract-20260527. The first
  allowed invariant is that timer/IPI paths record bounded state only, while
  owner-local normal control flow performs scheduler mutation after interrupt
  return. Current-task authority remains per PerCoreScheduler owner; shared
  metadata remains advisory and owner-published; SharedRunQueue and
  LoadBalancingPolicy continue to move only runnable non-current tasks.
- Evidence level: static inspection, documentation build, and whitespace
  inspection. Hardware was not required because no physical claim changed.
- Validation: git status --short was clean before edits; git diff --check
  passed; mdbook build passed.
- Rationale: Multi-core preemption touches timer, scheduler, SMP lock, IPI,
  wake, metadata, and migration boundaries. Contracting the owner-local model
  first prevents a QEMU or Pi 5 proof from becoming a marker-only shortcut or
  an accidental remote scheduler authority.
- Risks: Talos still lacks the target-independent multi-core preemption core,
  QEMU substitute proof, Pi 5 hardware proof, non-diagnostic secondary runtime,
  running-task migration, work stealing, and general remote reschedule. The
  next bounded task should be phase6-multicore-preemption-core-20260527.

## 2026-05-27 - Obsolete Diagnostic Bloat Removal Accepted

- Status: accepted as repository-health cleanup before the Phase 6.3
  multi-core preemption core. No scheduler feature, hardware proof, Phase 7,
  filesystem, networking, SSH, shell, RP1/PCIe, UART interrupt ownership, or
  DMA behavior was added.
- Context: The full obsolete-bloat inventory classified the historical QEMU
  secondary-core discriminator and old Pi 5 allocator, exception, panic, and
  translation-fault proof paths as remove-now. Accepted evidence summaries
  already preserve their classifications and artifact facts.
- Decision: Accept talos-obsolete-bloat-removal-sweep-20260527. Delete 20
  obsolete scripts, remove 18 boot-scenario registry entries, remove the
  QEMU discriminator dispatch/function, and simplify the Pi 5 boot,
  diagnostics, exception, and vector paths back to retained active behavior.
- Evidence level: static inspection, fmt/lint/typecheck, no_std unit tests,
  QEMU/substitute regression gates, documentation build, and whitespace
  inspection. Hardware was not required because no physical claim changed.
- Validation: cargo fmt --all -- --check, cargo -Zjson-target-spec test with
  147 no_std tests, scripts/qemu-smoke.sh,
  scripts/qemu-timer-preemption-smoke.sh,
  scripts/qemu-secondary-scheduler-service-loop-smoke.sh,
  scripts/qemu-shared-runqueue-migration-smoke.sh,
  scripts/qemu-load-balancing-smoke.sh, stale-reference rg checks over
  build.rs/src/scripts, git diff --check, and mdbook build passed.
- Rationale: Removing stale proof-only paths before multi-core preemption
  reduces cfg routing and boot-scenario ambiguity without weakening accepted
  evidence. Current Phase 4/5 Pi 5 proof scripts and Phase 6 QEMU/Pi 5
  scheduler gates remain retained until replaced by a named later task.
- Risks: Historical task and decision records still mention retired paths as
  accepted evidence; those records are intentionally preserved. Future cleanup
  should continue to distinguish executable/current validation surfaces from
  historical evidence summaries.

## 2026-05-27 - Senior Engineer Repo Review/Fix Pass 1 Accepted

- Status: accepted as repository-health remediation before post-review hardware
  validation, review pass 2, and Phase 6.3 multi-core preemption core work.
  No scheduler feature, new hardware proof, Phase 7, filesystem, networking,
  SSH, shell, RP1/PCIe, UART interrupt ownership, or DMA behavior was added.
- Context: The cleanup baseline was physically validated on Pi 5. A full-repo
  senior-engineer pass then reviewed unsafe code, SMP/scheduler boundaries,
  boot-scenario routing, scripts, docs, tests, and evidence policy.
- Decision: Accept talos-senior-engineer-repo-review-fix-pass-1-20260527. The
  pass replaces the Pi 5 panic recursion guard's shared volatile
  `UnsafeCell<bool>` with a word-sized atomic compare_exchange path, and updates
  architecture docs that still described retired Pi 5 allocator, panic,
  exception, and translation-fault proof-only diagnostics as active surfaces.
- Evidence level: static inspection, fmt/lint/typecheck, no_std unit tests,
  QEMU/substitute smoke, Pi 5 image build inspection, documentation build, and
  whitespace inspection. Hardware was not required because this pass does not
  claim a new physical behavior.
- Validation: cargo fmt --all -- --check, cargo -Zjson-target-spec test with
  147 no_std tests, scripts/qemu-smoke.sh, scripts/rpi5-image.sh,
  git diff --check, and mdbook build passed.
- Risks: The panic guard now depends on ordinary AArch64 word-sized atomic code
  generation rather than the old volatile byte. The change removes a real SMP
  data race while avoiding the retired byte-atomic shape, but a later
  post-review Pi 5 hardware validation remains queued before review pass 2.

## 2026-05-28 - Multi-Core Preemption Core Accepted

- Status: accepted as the target-independent Phase 6.3 multi-core preemption
  core. No QEMU boot scenario, Pi 5 hardware proof, Phase 7, filesystem,
  networking, SSH, shell, RP1/PCIe, UART interrupt ownership, or DMA/cache
  behavior was added.
- Context: The accepted contract permits timer/IPI paths to record bounded
  state while owner-local normal control flow performs scheduler mutation
  after interrupt return. The core needed explicit pending-state, owner,
  current-task, and preemption-disabled behavior before proof routing could be
  added safely.
- Decision: Accept phase6-multicore-preemption-core-20260527.
  PerCorePreemptionState records local pending timer requests, coalesces
  duplicate requests, tracks nested preemption-disable depth, and exposes
  deterministic defer/error outcomes. CpuLocalSchedulerService now has an
  owner-local preemption-cycle entry that preflights owner/current-task
  authority before wake draining, timer preemption, optional dispatch, and
  metadata refresh.
- Evidence level: static inspection, fmt/lint/typecheck, no_std unit tests,
  QEMU/substitute regression gates, documentation build, and whitespace
  inspection. Hardware was not required because this task makes no physical
  claim.
- Validation: cargo fmt --all -- --check, cargo -Zjson-target-spec test with
  153 no_std tests, scripts/qemu-smoke.sh,
  scripts/qemu-timer-preemption-smoke.sh,
  scripts/qemu-secondary-scheduler-service-loop-smoke.sh,
  scripts/qemu-load-balancing-smoke.sh, git diff --check, and mdbook build
  passed.
- Rationale: The core keeps IRQ-side behavior to bounded local recording and
  leaves all scheduler mutation in owner-local normal control flow. Failed
  service attempts keep pending requests and single-owner queue state intact,
  which lets later QEMU/Pi 5 proof tasks exercise the invariant without
  introducing remote current-task authority.
- Risks: QEMU and Pi 5 multi-core preemption proofs are still queued. The core
  does not yet wire real timer IRQ recorders into this state or add a
  non-diagnostic secondary runtime role.

## 2026-05-28 - QEMU Multi-Core Preemption Smoke Accepted

- Status: accepted as QEMU substitute evidence for the Phase 6.3 multi-core
  preemption core. No Pi 5 hardware claim, direct IRQ/IPI-context scheduling,
  remote current-task switching, running-task migration, autonomous work
  stealing, Phase 7, filesystem, networking, SSH, shell, RP1/PCIe, UART
  interrupt ownership, or DMA/cache behavior was added.
- Context: The accepted core added PerCorePreemptionState and
  CpuLocalSchedulerService::run_preemption_cycle, but proof routing still
  needed to show multiple owners recording bounded local timer-preemption state
  and servicing it only from owner-local normal control flow.
- Decision: Accept phase6-qemu-multicore-preemption-smoke-20260527. The
  qemu_multicore_preemption_smoke boot scenario and
  scripts/qemu-multicore-preemption-smoke.sh start logical CPUs 1, 2, and 3
  through the QEMU SMP path. Each owner records a local pending request,
  coalesces a duplicate local record, rejects cross-owner recording, proves the
  record-only step leaves current task, runnable queue, task states, and
  metadata unchanged, then services the request through
  CpuLocalSchedulerService::run_preemption_cycle.
- Evidence level: static inspection, QEMU substitute transcript, fmt/lint,
  no_std unit tests, preserved QEMU smoke/regression gates, whitespace
  inspection, and documentation build. Hardware was not required because this
  task makes no physical claim.
- Validation: scripts/qemu-multicore-preemption-smoke.sh,
  cargo fmt --all -- --check, cargo -Zjson-target-spec test,
  scripts/qemu-smoke.sh, scripts/qemu-timer-preemption-smoke.sh,
  scripts/qemu-secondary-scheduler-service-loop-smoke.sh,
  scripts/qemu-shared-runqueue-migration-smoke.sh,
  scripts/qemu-load-balancing-smoke.sh, git diff --check, and mdbook build
  passed.
- Consequences: The next bounded task may be the serialized Pi 5 multi-core
  preemption proof after supervisor ready-marking and hardware lock
  availability. Physical behavior remains unclaimed until that task is accepted
  or explicitly deferred.

## 2026-05-28 - Pi 5 Multi-Core Preemption Proof Accepted

- Status: accepted as serialized Raspberry Pi 5 hardware evidence for the
  Phase 6.3 multi-core preemption core. No direct IRQ/IPI-context scheduling,
  remote current-task switching, running-task migration, autonomous work
  stealing, Phase 7, filesystem, networking, SSH, shell, RP1/PCIe, UART
  interrupt ownership, or DMA/cache behavior was added.
- Context: The accepted QEMU proof showed the invariant with logical CPUs 1, 2,
  and 3. The Pi 5 proof needed to carry that same owner-local record/service
  invariant to physical hardware under hardwareTestLock, with TFTP, serial,
  artifact identity, participant count, and restore evidence.
- Decision: Accept phase6-pi5-multicore-preemption-proof-20260527. The
  rpi5_multicore_preemption_proof boot scenario and
  scripts/rpi5-multicore-preemption-image.sh /
  scripts/rpi5-multicore-preemption-boot-tree.sh stage the retained Pi 5
  proof. The physical run reached
  classification=pi5-multicore-preemption-complete with participants=3,
  expected=3, errors=0, and PASS.
- Evidence level: static inspection, fmt/lint/typecheck, no_std unit tests,
  focused QEMU substitute rerun, image/archive inspection, serialized Pi 5
  hardware serial output, TFTP fetch evidence, and restore proof.
- Validation: cargo fmt --all -- --check, cargo -Zjson-target-spec test,
  scripts/qemu-multicore-preemption-smoke.sh,
  scripts/rpi5-multicore-preemption-image.sh,
  scripts/rpi5-archive-review.sh
  target/talos-rpi5-multicore-preemption-boot.tar.gz, serialized Pi 5
  publish/power-cycle/observe/restore, git diff --check, and mdbook build
  passed.
- Rationale: The only code correction after the inconclusive candidate runs was
  to include rpi5_multicore_preemption_proof in the already accepted secondary
  cacheable-MMU handoff guard. That matches the accepted secondary
  service-loop proof path and avoids changing scheduler semantics.
- Consequences: Multi-core preemption now has contract, target-independent
  core, QEMU substitute proof, and serialized Pi 5 proof evidence. The next
  bounded task should be the multi-core preemption closeout checkpoint before
  any Phase 7 or later subsystem work.

## 2026-05-28 - Production Timer/Preemption Contract Accepted

- Status: accepted as a documentation-only Phase 6.3 production scheduler
  runtime contract. No Rust implementation, QEMU proof, Pi 5 hardware claim,
  direct IRQ/IPI-context scheduling, remote current-task switching,
  running-task migration, autonomous work stealing, Phase 7, filesystem,
  networking, SSH, shell, RP1/PCIe, UART interrupt ownership, or DMA/cache
  behavior was added.
- Context: The accepted production scheduler runtime source inventory showed
  that existing multi-core preemption proofs construct scenario-local
  scheduler state and call the preemption primitive directly from diagnostic
  flow. Normal QEMU and Pi 5 timer handlers still rearm/EOI without recording
  into durable production preemption state.
- Decision: Accept
  phase6-production-timer-preemption-contract-20260528. The next
  implementation may touch only the normal QEMU and Pi 5 timer IRQ recording
  paths, the generic timer tick/rearm helper as a non-mutating signal source,
  owner-local primary and secondary post-IRQ service points, and minimal
  durable per-CPU runtime state for local scheduler/preemption/wake/metadata
  access, current-task source, and role/capability.
- Rationale: The contract preserves the accepted invariant: IRQ/IPI context is
  record-only, and all scheduler mutation happens in owner-local normal
  control flow through CpuLocalSchedulerService. Remote wake is consumed before
  timer preemption, optional local dispatch happens only when timer preemption
  did not run, and metadata refresh happens after local mutation.
- Deterministic outcomes: disabled preemption, stale metadata, wrong owner,
  missing current task, current-task mismatch, non-production-capable roles,
  and no runnable peer are defer/reject cases that must not grant remote
  current-task authority or mutate another owner's scheduler.
- Evidence level: static inspection, documentation update, whitespace
  inspection, and documentation build. Rust fmt/tests, QEMU, and Pi 5 hardware
  were not required because this task changes only Markdown documentation and
  durable worker state.
- Consequences: The next bounded task may implement
  phase6-production-timer-preemption-core-20260528 within the named contract
  surface. Focused QEMU and serialized Pi 5 proof remain separate later tasks;
  Phase 7 remains blocked until the production scheduler runtime slice is
  accepted or explicitly deferred and closed out.

## 2026-05-28 - Production Timer/Preemption Core Accepted

- Status: accepted as the first bounded Phase 6.3 production scheduler runtime
  implementation. No new QEMU production proof, Pi 5 hardware claim, direct
  IRQ/IPI-context scheduling, remote current-task switching, running-task
  migration, autonomous work stealing, Phase 7, filesystem, networking, SSH,
  shell, RP1/PCIe, UART interrupt ownership, or DMA/cache behavior was added.
- Context: The accepted contract allowed only the normal QEMU and Pi 5 timer
  IRQ recording paths, a minimal durable per-CPU runtime boundary, and
  owner-local post-IRQ service through the accepted scheduler service order.
- Decision: Accept phase6-production-timer-preemption-core-20260528.
  `ProductionSchedulerRuntime` now holds the local scheduler, local
  preemption state, target-owned remote-wake queue, and role/capability. QEMU
  and Pi 5 timer IRQ handlers record bounded local production preemption state
  after the generic timer rearm helper and before EOI.
- Rationale: IRQ context remains record-only. Production scheduler mutation is
  still delegated to owner-local normal control flow through
  `ProductionSchedulerRuntime::service_pending_preemption` and
  `CpuLocalSchedulerService::run_preemption_cycle`, preserving remote wake
  before timer preemption, optional dispatch only when timer preemption did
  not run, and metadata refresh last.
- Evidence level: static inspection, fmt/lint/typecheck, no_std unit tests,
  retained QEMU substitute gates, whitespace inspection, and documentation
  build. No hardware evidence was claimed.
- Validation: cargo fmt --all -- --check, cargo -Zjson-target-spec test,
  scripts/qemu-smoke.sh, scripts/qemu-timer-preemption-smoke.sh,
  scripts/qemu-secondary-scheduler-service-loop-smoke.sh,
  scripts/qemu-shared-runqueue-migration-smoke.sh,
  scripts/qemu-load-balancing-smoke.sh,
  scripts/qemu-multicore-preemption-smoke.sh, git diff --check, and mdbook
  build passed.
- Consequences: The next bounded task may be the focused QEMU production
  timer/preemption smoke. Serialized Pi 5 production proof and the production
  scheduler runtime closeout remain separate later tasks; Phase 7 remains
  blocked until this production runtime slice is proved or explicitly deferred
  and closed out.

## 2026-05-28 - QEMU Production Timer/Preemption Smoke Accepted

- Status: accepted as a focused QEMU substitute proof for the Phase 6.3
  production timer/preemption runtime integration. No Pi 5 hardware claim,
  direct IRQ/IPI-context scheduler mutation, remote current-task switching,
  running-task migration, autonomous work stealing, Phase 7, filesystem,
  networking, SSH, shell, RP1/PCIe, UART interrupt ownership, or DMA/cache
  behavior was added.
- Context: The production timer/preemption core wired normal timer IRQ handlers
  to record bounded local production preemption state, but the retained QEMU
  proof gates still exercised either timer interrupts without production
  scheduler service or direct diagnostic preemption primitives without the
  production runtime boundary.
- Decision: Accept
  phase6-qemu-production-timer-preemption-smoke-20260528. The new
  `qemu_production_timer_preemption_smoke` boot scenario and
  `scripts/qemu-production-timer-preemption-smoke.sh` exercise the
  target-owned production timer IRQ adapter and owner-local
  `ProductionSchedulerRuntime::service_pending_preemption` together on QEMU
  logical CPUs 1, 2, and 3.
- Rationale: The proof distinguishes production runtime entry from direct
  diagnostic helper calls while preserving the accepted rule that IRQ-side
  work records only bounded state. Scheduler mutation remains in owner-local
  normal control flow, with current-task, runnable-queue, and metadata state
  unchanged across the record-only step.
- Evidence level: static inspection, fmt/lint/typecheck, no_std unit tests,
  retained QEMU substitute gates, focused QEMU production smoke transcript,
  whitespace inspection, and documentation build. No hardware evidence was
  claimed.
- Validation: cargo fmt --all -- --check, cargo -Zjson-target-spec test,
  scripts/qemu-smoke.sh, scripts/qemu-timer-preemption-smoke.sh,
  scripts/qemu-secondary-scheduler-service-loop-smoke.sh,
  scripts/qemu-shared-runqueue-migration-smoke.sh,
  scripts/qemu-load-balancing-smoke.sh,
  scripts/qemu-multicore-preemption-smoke.sh,
  scripts/qemu-production-timer-preemption-smoke.sh, git diff --check, and
  mdbook build passed.
- Consequences: The next bounded task may carry the same production
  timer/preemption invariant to serialized Pi 5 hardware evidence under the
  hardwareTestLock. The production scheduler runtime closeout remains a
  separate later task.

## 2026-05-28 - Pi 5 Production Timer/Preemption Proof Accepted

- Status: accepted as serialized Raspberry Pi 5 hardware evidence for the
  Phase 6.3 production timer/preemption runtime integration. No direct
  IRQ/IPI-context scheduler mutation, remote current-task switching,
  running-task migration, autonomous work stealing, Phase 7, filesystem,
  networking, SSH, shell, RP1/PCIe, UART interrupt ownership, or DMA/cache
  behavior was added.
- Context: The focused QEMU production timer/preemption proof had accepted
  the target-owned production timer IRQ adapter plus owner-local service path
  as a QEMU substitute invariant. The same invariant needed serialized Pi 5
  evidence before the production scheduler runtime slice could close out.
- Decision: Accept
  phase6-pi5-production-timer-preemption-proof-20260528. The new
  `rpi5_production_timer_preemption_proof` boot scenario and retained Pi 5
  image/boot-tree scripts carry the production timer/preemption invariant to
  hardware. The accepted local8 multi-observe run reached logical CPU reports
  for CPUs 1, 2, and 3, `participants=3 expected=3 errors=0`,
  `classification=pi5-production-timer-preemption-complete`, and PASS.
- Rationale: Earlier one-shot serial observes were inconclusive even though
  TFTP fetched the candidate image. The required triage recorded candidate
  identity, fresh serial cursor, TFTP delta, known-good control, padded-size
  control, static/image comparison, and candidate reruns before the
  multi-observe capture proved the candidate. The final evidence shows the
  candidate, not only a control tree, reached the named proof lines.
- Evidence level: static inspection, image/archive inspection,
  fmt/lint/typecheck, no_std unit tests, focused QEMU substitute rerun,
  serialized hardware boot/output, lab-controller TFTP/status records,
  whitespace inspection, documentation build, and restore proof.
- Validation: cargo fmt --all -- --check, cargo -Zjson-target-spec test,
  scripts/qemu-production-timer-preemption-smoke.sh,
  scripts/rpi5-archive-review.sh on the staged archive, serialized Pi 5
  lab publish/power-cycle/serial observe, git diff --check, and mdbook build
  passed. The accepted archive SHA256 is
  739810c8480893e1878967dd0409f2705e71481453fc08038e9aacffdebcc11e and the
  kernel SHA256 is
  fdf8858d0740c0d7bf4fc0df884d4052d8309fd9c020ba65e5df1472198e7dfa.
- Consequences: The production scheduler runtime closeout may reconcile the
  accepted inventory, contract, core, QEMU proof, and Pi 5 proof. General
  scheduler productionization and Phase 7 remain blocked until a later
  supervisor-planned bounded task.

## 2026-05-28 - Production Scheduler Runtime Closeout Accepted

- Status: accepted as the Phase 6.3 production scheduler runtime closeout
  checkpoint for the production timer/preemption slice. No Rust
  implementation, boot image, QEMU run, hardware run, direct IRQ/IPI-context
  scheduling, remote current-task switching, running-task migration,
  autonomous work stealing, Phase 7, filesystem, networking, SSH, shell,
  RP1/PCIe, UART interrupt ownership, or DMA/cache behavior was added.
- Context: Talos had accepted the production scheduler runtime source
  inventory, production timer/preemption contract, target-independent core,
  focused QEMU substitute proof, and serialized Pi 5 hardware proof.
- Decision: Accept
  phase6-production-scheduler-runtime-closeout-checkpoint-20260528. The
  accepted boundary is the first production timer/preemption runtime
  integration: normal target timer IRQ handlers may record local pending
  preemption in durable per-CPU runtime state, and owner-local normal
  scheduler control flow may service that pending state through
  `ProductionSchedulerRuntime::service_pending_preemption`.
- Evidence level: static inspection of accepted task/evidence records,
  documentation build, and whitespace inspection. Hardware was not rerun
  because this checkpoint makes no new physical claim beyond the already
  accepted Pi 5 proof.
- Validation: git status --short was clean before edits; git diff --check
  passed; mdbook build passed.
- Consequences: Further scheduler productionization or Phase 7 work requires a
  new supervisor-planned bounded task with explicit scope, dependencies,
  acceptance criteria, validation gates, documentation requirements, and
  evidence requirements.
- Risks: Interrupt-driven remote reschedule, work stealing, autonomous
  balancing cadence, running-task migration, remote current-task switching,
  asynchronous context capture, non-diagnostic secondary runtime roles, Phase
  7, filesystem, networking, SSH, shell behavior, RP1/PCIe, UART interrupt
  ownership, and DMA/cache-driver policy remain deferred.

## 2026-05-28 - Phase 7 Descriptor Table Contract Accepted

- Status: accepted as a documentation-only Phase 7.1 descriptor-table
  contract. No Rust implementation, descriptor table core, syscall ABI, EL0,
  VFS/filesystem, pipe, socket, shell behavior, networking, SSH, RP1/PCIe,
  UART interrupt ownership, or DMA/cache-driver policy was added.
- Context: The accepted POSIX baseline defined descriptor vocabulary and
  stdio direction, and the accepted path/error model core provided the first
  target-independent PosixError vocabulary. Before descriptor-table code, the
  table needed a narrower process-local contract for entry lifetime, dup,
  close, inherited stdio, and deterministic errors.
- Decision: Accept
  phase7-descriptor-table-contract-20260528. Descriptor numbers are
  process-local table indexes; entries reference shared open descriptions or
  reserved kernel object handles; dup creates a new descriptor number pointing
  at the same referenced object; close releases one entry; fd 0, fd 1, and
  fd 2 are inherited stdio descriptor entries backed later by TTY and
  runtime-console0 handles.
- Evidence level: static inspection, documentation build, and whitespace
  inspection. No QEMU or Pi 5 hardware evidence was claimed.
- Validation: git status --short was clean before edits; git diff --check
  passed; mdbook build passed.
- Consequences: The next bounded implementation task may add only the
  target-independent descriptor-table core with unit tests for invalid fd,
  close, double close, dup, table full, inherited stdio, reserved object
  kinds, and deterministic PosixError results. Runtime console/TTY descriptor
  I/O integration, syscall ABI, EL0, VFS/filesystem, pipe, socket, shell,
  networking, SSH, and hardware claims remain blocked.

## 2026-05-28 - Phase 7 QEMU EL0 Trap Smoke Plan Accepted

- Status: accepted as a documentation-only Phase 7.2 plan for the first
  QEMU-only EL0 trap smoke. No Rust implementation, assembly implementation,
  boot scenario, QEMU run, Pi 5 hardware run, archive publishing,
  hardware-lock acquisition, syscall ABI, process loading, VFS/filesystem,
  descriptor I/O, shell behavior, networking, SSH, RP1/PCIe, UART interrupt
  ownership, or DMA/cache behavior was added.
- Context: The Phase 7.2 EL0 trap/address-space contract and
  target-independent user-memory permission core are accepted. Before lower-EL
  assembly or translation-table work starts, the first QEMU proof needs fixed
  output, source ownership, and evidence-retention rules.
- Decision: Accept phase7-qemu-el0-trap-smoke-plan-20260528. The next QEMU
  proof boundary is one qemu_el0_trap_smoke boot scenario with a built-in
  user payload mapped inside fixed UserText/UserStack/UserGuard ranges. The
  payload may execute only diagnostic SVC marker 0x7a10, which is not a
  syscall ABI. The required final lines are
  qemu-el0-trap-smoke: final participants=1 expected=1 errors=0 classification=qemu-el0-trap-smoke-complete
  and qemu-el0-trap-smoke: PASS, plus a saved-state line with vector, ESR,
  FAR, ELR, SP, SPSR, and marker fields.
- Evidence level: static documentation inspection, documentation build, and
  whitespace inspection. No QEMU or Pi 5 hardware evidence was claimed.
- Validation: git status --short was clean before edits; git diff --check
  passed; mdbook build passed.
- Consequences: The next mechanically derivable task is
  phase7-qemu-el0-trap-smoke-core-20260528, bounded to the QEMU boot
  scenario, built-in payload mapping, validated ERET handoff, lower-EL trap
  capture, script gate, and retained QEMU evidence. Pi 5 proof, general
  syscall ABI, process loading, filesystem behavior, shell behavior,
  networking, and SSH remain blocked.

## 2026-05-28 - Phase 7 Pi 5 EL0 Trap Proof Plan Accepted

- Status: accepted as a documentation-only Phase 7.2 plan for the serialized
  Raspberry Pi 5 lower-EL trap proof. No Rust implementation, assembly
  implementation, boot archive publishing, power-cycle, serial observe,
  hardware-lock acquisition, general syscall ABI, process loading,
  VFS/filesystem, descriptor I/O, shell behavior, networking, SSH, RP1/PCIe,
  UART interrupt ownership, or DMA/cache behavior was added.
- Context: The QEMU EL0 trap smoke implementation and closeout are accepted
  with retained QEMU/substitute evidence. Before the invariant can be carried
  to physical Pi 5 hardware, the candidate identity, lock ownership, fresh
  serial/TFTP evidence, inconclusive-run triage, and restoration requirements
  must be fixed.
- Decision: Accept phase7-pi5-el0-trap-proof-plan-20260528. The future
  hardware task must acquire hardwareTestLock, stage a focused
  rpi5_el0_trap_proof candidate, prove fresh candidate fetch and serial
  capture, and require saved-state output plus
  classification=pi5-el0-trap-proof-complete and rpi5-el0-trap-proof: PASS.
  If the first candidate run is inconclusive, no code may change until
  candidate identity, fresh serial cursor, TFTP delta, known-good control, and
  candidate rerun evidence are recorded.
- Evidence level: static documentation inspection, documentation build, and
  whitespace inspection. No Pi 5 hardware evidence was claimed.
- Validation: git status --short before edits was clean; git diff --check
  passed; mdbook build passed.
- Consequences: The next mechanically derivable task is
  phase7-pi5-el0-trap-proof-20260528, gated by an unlocked hardwareTestLock.
  It may add only the focused Pi 5 proof source/script surfaces named in the
  plan and must preserve the blocked syscall ABI, process loading, descriptor
  I/O, filesystem, shell, networking, SSH, RP1/PCIe, UART interrupt, and
  DMA/cache-driver surfaces.

## 2026-05-29 - Phase 7 Pi 5 EL0 Trap Proof Accepted

- Status: accepted as the serialized Raspberry Pi 5 hardware proof for the
  bounded Phase 7.2 lower-EL trap path.
- Context: Earlier Pi 5 attempts stopped at the EL1 translation-enable
  boundary. The accepted run followed the required inconclusive-run triage,
  added a source-backed translation feature/legal-shape report, fixed the
  BCM2712 MMIO L2 descriptor range, and returned VBAR_EL1 to the regular
  exception vectors before entering EL0.
- Decision: Accept phase7-pi5-el0-trap-proof-20260528. Physical serial
  evidence from local62 contains the expected saved lower-AArch64 synchronous
  SVC trap state, final
  classification=pi5-el0-trap-proof-complete, and rpi5-el0-trap-proof: PASS.
- Evidence level: fmt/lint/typecheck, unit tests, QEMU/substitute smoke,
  static image/archive inspection, lab-controller API, serialized Pi 5
  hardware boot/output, repeated control/rerun evidence for prior inconclusive
  boundaries, and restoration proof.
- Validation: cargo fmt --all -- --check passed; cargo -Zjson-target-spec test
  passed with 189 tests; scripts/qemu-el0-trap-smoke.sh passed;
  scripts/rpi5-el0-trap-proof-static-check.sh passed; archive review passed;
  local62 served da591740/kernel_2712.img at 97,781 bytes and reached PASS;
  restore reported production-timer tree hash
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- Consequences: Phase 7.2 has a physical lower-EL trap proof, but this does
  not accept a general syscall ABI, process loader, descriptor I/O,
  filesystem, shell, networking, SSH, RP1/PCIe, UART interrupt ownership, or
  DMA/cache-driver policy.

## 2026-05-29 - Phase 7 Syscall ABI Contract Accepted

- Status: accepted as a documentation-only Phase 7.3 syscall ABI contract.
  No Rust implementation, assembly implementation, boot scenario, QEMU run,
  Pi 5 hardware run, archive publishing, hardware-lock acquisition, process
  loading, descriptor I/O, VFS/filesystem, shell behavior, networking, SSH,
  RP1/PCIe, UART interrupt ownership, or DMA/cache behavior was added.
- Context: The accepted lower-EL proof establishes that QEMU and Pi 5 can
  enter lower EL, execute a diagnostic SVC marker, and trap back with saved
  state. The accepted syscall ABI source inventory identified the missing
  production ABI decisions before implementation.
- Decision: Accept phase7-syscall-abi-contract-20260529. The first stable
  syscall trap is lower-AArch64 svc #0. x8 carries the syscall number, x0
  through x5 carry scalar arguments, x0 is the sole return register, negative
  x0 values encode -errno, talos_nop = 0 returns 0, and unknown syscall numbers
  return -ENOSYS from a valid trap frame. Diagnostic SVC marker 0x7a10 remains
  proof-only and is not a syscall number.
- Evidence level: static documentation inspection, documentation build, and
  whitespace inspection. No QEMU or Pi 5 hardware evidence was claimed.
- Validation: git status --short before edits was clean; git diff --check
  passed; mdbook build passed.
- Consequences: The next bounded implementation task may add only the
  target-independent syscall dispatch/error-conversion core and unit tests.
  Production exception-handler integration, QEMU syscall smoke, Pi 5 hardware
  proof, pointer-copy syscalls, descriptor I/O, process loading, VFS,
  filesystem, shell, networking, SSH, RP1/PCIe, UART interrupt ownership, and
  DMA/cache-driver policy remain blocked.

## 2026-05-29 - Phase 7 Pi 5 Syscall Proof Plan Accepted

- Status: accepted as a documentation-only Phase 7.3 plan for the serialized
  Raspberry Pi 5 production syscall routing proof. No Rust implementation,
  assembly implementation, boot archive publishing, power-cycle, serial
  observe, hardware-lock acquisition, descriptor I/O, byte copy-in/copy-out,
  process loading, VFS/filesystem, shell behavior, networking, SSH, RP1/PCIe,
  UART interrupt ownership, or DMA/cache behavior was added.
- Context: The accepted QEMU syscall smoke core proves lower-AArch64 svc #0
  routing through the production exception path and target-independent syscall
  dispatch core only at QEMU/substitute evidence level. Before carrying that
  invariant to physical Pi 5 hardware, the candidate identity, lock ownership,
  fresh serial/TFTP evidence, diagnostic marker quarantine, inconclusive-run
  triage, restoration requirements, and expected PASS/classification lines
  must be fixed.
- Decision: Accept phase7-pi5-syscall-proof-plan-20260529. The future hardware
  task must acquire hardwareTestLock, stage a focused rpi5_syscall_proof
  candidate, prove fresh candidate fetch and serial capture, observe stable
  svc #0 talos_nop returning x0 = 0 and unknown syscall number 17 returning
  x0 = -ENOSYS in lower EL, keep diagnostic marker 0x7a10 proof-only, and
  require classification=pi5-syscall-proof-complete plus
  rpi5-syscall-proof: PASS. If any candidate run is inconclusive, no code may
  change until candidate identity, fresh serial cursor, TFTP delta, known-good
  control, and unchanged candidate rerun evidence are recorded.
- Evidence level: static documentation inspection, documentation build, and
  whitespace inspection. No Pi 5 hardware evidence was claimed.
- Validation: git status --short before edits was clean; git diff --check
  passed; mdbook build passed.
- Consequences: The next mechanically derivable task is
  phase7-pi5-syscall-proof-20260529, gated by an unlocked hardwareTestLock and
  this accepted plan. It may add only the focused Pi 5 proof source/script
  surfaces named in the plan and must preserve the blocked descriptor I/O,
  copy-in/copy-out, process loading, filesystem, shell, networking, SSH,
  RP1/PCIe, UART interrupt, and DMA/cache-driver surfaces.
