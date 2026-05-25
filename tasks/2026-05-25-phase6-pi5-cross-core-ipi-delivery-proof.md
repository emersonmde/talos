# Phase 6 Pi 5 Cross-Core IPI Delivery Proof

Task ID: phase6-pi5-cross-core-ipi-delivery-proof-20260525
Status: accepted

## Goal

Capture serialized Pi 5 hardware evidence for raw GIC-400/GICv2 SGI delivery before physical scheduler wakeups depend on IPIs.

## Scope

- Reuse the accepted QEMU raw IPI delivery boundary and Pi 5 secondary-core cacheable-MMU handoff.
- Build and review a Pi 5 diagnostic archive that sends SGI INTID 1 from the boot CPU to secondary logical CPUs.
- Acquire hardwareTestLock before staging or power-affecting hardware work and restore the pre-run boot tree after each attempt.
- Capture archive digest, kernel size/hash, TFTP fetch evidence, cursor-valid serial output, per-core SGI counts, errors, classification, and restore evidence.

## Evidence

Evidence directory: tasks/evidence/2026-05-25-pi5-cross-core-ipi-delivery-proof/.

- Unit tests: cargo -Zjson-target-spec test passed 110 no_std tests after adding SGI target-filter encoding coverage.
- QEMU/substitute: scripts/qemu-cross-core-ipi-delivery-smoke.sh still passed with classification=qemu-cross-core-ipi-delivery-complete.
- Image/archive inspection: the focused Pi 5 boot-tree script and archive review passed.
- Hardware run 1: directed SGIR target-list writes fetched and executed, but classified pi5-cross-core-ipi-delivery-invariant-failed; all secondaries reached ready-mask=0xe, but complete-mask=0x0 and per-core receive/EOI counts remained zero.
- Hardware run 2: explicit SGI Group 1/PPI enable setup fetched and executed, but classified pi5-cross-core-ipi-delivery-invariant-failed with the same ready/no-receive shape.
- Hardware run 3: SGIR target-filter all-except-self (sgir=0x01000001) fetched and executed, but classified pi5-cross-core-ipi-delivery-invariant-failed; all target cores stayed workload-running with zero receive/EOI counts.
- Hardware run 4: secondary-side CPU-interface discriminator with HPPIR/IAR/DAIF/HCR fields in the report fetched and executed, but the first probe shape still used WFE before polling. Reports that included the new fields showed poll-count=0, so the discriminator did not actually sample the target CPU interfaces.
- Hardware run 5: active-spin CPU-interface discriminator archive was built and served at 96,696 bytes with archive SHA256 668c81cf5d38e50a9bb46722568a24c1bd385bc61bad5d38dbec05d7dd30aae6 and kernel SHA256 2a0bded765427afb0c678729d89931cc289c27ab2bab94598ad8549072bed0ef. The Pi repeatedly fetched the candidate, but cursor-valid serial observe did not contain the unique cpuif-poll=active-spin marker before restore, so this run is evidence for candidate fetch without a reliable current Talos transcript, not evidence for SGI delivery or non-delivery.
- Implementation correction: the Pi 5 exception dispatcher now compiles the raw IPI proof into the target IRQ handler. Earlier runs could receive an SGI at the vector level without dispatching it to the proof receive/EOI accounting.
- Hardware run 6: after the Pi 5 IRQ dispatch correction, archive SHA256 a6c5cb6999784e8f8c61a07765d39e9549c19c0ae37a54267c738b116a521a79 and kernel SHA256 44792c6681d0e67df08abeaebd18f2408680940ead47e2cf1e0b44f5b3956837, size 97,016 bytes, were served to the Pi. Cursor-valid serial showed `cpuif-poll=active-spin`, SGIR 0x01000001, receivers 1, 2, and 3 each at `receive-count=1 eoi-count=1 intid=1`, `participants=3`, `errors=0`, `classification=pi5-cross-core-ipi-delivery-complete`, and `PASS`.
- Restore proof: all recorded hardware runs restored their pre-run snapshots; each restore-exit.txt is 0.

## Current Classification

Accepted. Pi 5 evidence proves candidate fetch, Talos entry, secondary-core readiness, cacheable-MMU handoff, SGIR all-except-self delivery, per-core SGI receive/EOI accounting, PASS classification, and restore.

## Acceptance

Accepted for raw Pi 5 GIC-400/GICv2 SGI delivery as a future scheduler wakeup prerequisite. Scheduler remote wakeups, remote enqueue state, shared run queues, task migration, production IPI use, and broader scheduler migration remain deferred.
