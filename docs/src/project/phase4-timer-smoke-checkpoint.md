# Phase 4 Timer-Smoke Checkpoint

Status: accepted for the QEMU virt and Raspberry Pi 5 EL2 physical timer
interrupt smoke boundary.

## Scope

This checkpoint reconciles the first interrupt-driven timer evidence before
Talos moves from one-shot delivery smokes into reusable timekeeping policy. It
does not add scheduler structures, preemption, SMP, UART interrupts,
lower-EL/user timer policy, DMA, RP1/PCIe interrupt routing, or networking.

## Accepted Behavior

- The current-EL IRQ frame path saves `x0..x30`, calls
  `rust_irq_handler(vector, elr, spsr, frame)`, restores the full interrupted
  frame, and returns with `ERET`.
- QEMU virt, when run as `-M virt,gic-version=2,virtualization=on`, can deliver
  the EL2 hypervisor physical timer through GICv2 PPI 10 / INTID 26.
- Raspberry Pi 5 can deliver the same EL2 hypervisor physical timer shape
  through GIC-400 PPI 10 / INTID 26.
- Both accepted smokes acknowledge with `GICC_IAR`, mask `CNTHP_CTL_EL2`, EOI
  with `GICC_EOIR`, keep unexpected interrupt accounting bounded, avoid
  allocation/formatting in the IRQ path, and return to a bounded post-IRQ
  workload.

## Target Differences

- QEMU virt uses GICv2 distributor `0x0800_0000` and CPU interface
  `0x0801_0000`; the focused smoke must request virtualization so Talos enters
  EL2.
- Raspberry Pi 5 uses GIC-400 distributor `0x10_7fff_9000` and CPU interface
  `0x10_7fff_a000`, discovered through the BCM2712 device-tree bus mapping.
- The Pi 5 DTB records active-low level timer PPI flags and also exposes a
  hypervisor virtual PPI 12 / INTID 28. The accepted first smoke avoids relying
  on PPI trigger reconfiguration and does not target the hypervisor virtual
  timer.

## Evidence

- IRQ frame contract: commit `de40482`.
- QEMU EL2 timer smoke: commit `bce215d`; task record
  `tasks/2026-05-24-phase4-qemu-el2-timer-irq-smoke.md`; log
  `target/qemu-timer-irq-smoke.log`.
- Pi 5 EL2 timer smoke: commit `966d453`; task record
  `tasks/2026-05-24-phase4-pi5-el2-timer-irq-smoke.md`; hardware evidence
  `tasks/evidence/2026-05-24-pi5-el2-timer-irq-smoke/`.
- Pi 5 archive proof: candidate archive sha256
  `1861b6978b505381fd28ffb21320f1db9434405c4ce44af69354d6e1e82f5bb2`,
  `kernel_2712.img` sha256
  `850902110e96af341e595f1493c0802f742e6618ad57546f0f37dc06236d3e0a`, size
  86,429 bytes, with TFTP serving that image to the Pi during the observed
  boot.

## Deferred Work

- Monotonic tick accounting and periodic timer reprogramming.
- Critical-section and IRQ mask/restore policy.
- Scheduler task structures, preemption, sleep queues, and context switching.
- SMP routing, per-core timer state, and interrupt-safe locking for secondary
  cores.
- UART interrupts, SPIs beyond the timer PPI, BCM2712 secondary interrupt
  controllers, RP1/PCIe interrupts, MSI, DMA, and cache-coherent driver policy.
- Lower-EL timer routing, EL0/user timer access, and POSIX clock APIs.

## Decision

The next implementation slice is monotonic tick accounting. No additional
timer-delivery discriminator is needed before that slice because both current
targets have accepted one-shot EL2 physical timer IRQ evidence. The monotonic
tick task must keep interrupt-time constraints explicit and must not let the
one-shot smoke become scheduler or preemption policy by implication.

## Validation

- `git diff --check`: passed for this documentation checkpoint.
- `mdbook build`: unavailable in the container.
