# Interrupts and Timers

This note is the Phase 4 source inventory for the first interrupt and timer
bring-up tasks. It turns the roadmap intent into a target contract; it does not
enable interrupts, program a timer, or implement a driver.

## Source Evidence

The accepted facts below come from these sources:

- QEMU 9.2.0 virt machine source: hw/arm/virt.c in the QEMU v9.2.0 tree.
- QEMU-generated device tree from qemu-system-aarch64 with
  -M virt,gic-version=2,dumpdtb=... and -cpu cortex-a76.
- Raspberry Pi Linux rpi-6.12.y device-tree sources:
  arch/arm64/boot/dts/broadcom/bcm2712.dtsi, bcm2712-rpi.dtsi, and
  bcm2712-rpi-5-b.dts.
- A lab-staged Pi 5 bcm2712-rpi-5-b.dtb copied from the accepted boot tree
  target/tmp/rpi5-bootstrap-reserve-post-println-20260523T1248Z-tree.
- Linux devicetree bindings for arm,gic.yaml and arm,arch_timer.yaml, plus
  include/dt-bindings/interrupt-controller/arm-gic.h and irq.h.
- Linux driver references drivers/irqchip/irq-gic.c,
  include/linux/irqchip/arm-gic.h, and drivers/clocksource/arm_arch_timer.c.
- ARM Architecture Reference Manual and GIC architecture references remain the
  primary architectural specifications for final register semantics.

The temporary source excerpts used by this inventory were staged under
target/tmp/phase4-interrupt-timer-source-inventory/. They are build evidence,
not repository inputs.

## Interrupt Specifier Contract

Both current targets use the standard GIC devicetree interrupt specifier with
#interrupt-cells = <3>:

- Cell 0 is the interrupt class: GIC_SPI = 0, GIC_PPI = 1.
- Cell 1 is the interrupt number within that class. For PPIs, the hardware
  INTID is 16 + cell1.
- Cell 2 contains trigger flags in bits 3:0 and, for PPIs, a CPU mask in bits
  15:8.

Linux irq.h defines IRQ_TYPE_LEVEL_HIGH = 4 and IRQ_TYPE_LEVEL_LOW = 8. QEMU's
generated timer PPIs use active-high level flags. The Pi 5 DTB uses active-low
level flags with a four-CPU PPI mask.

## QEMU Virt Target

Talos' fast target is talos-aarch64-virt, run as QEMU virt with gic-version=2
and -cpu cortex-a76.

Accepted QEMU interrupt-controller facts:

- Main controller: GICv2 compatible arm,cortex-a15-gic.
- Distributor base: 0x0800_0000, size 0x0001_0000.
- CPU interface base: 0x0801_0000, size 0x0001_0000.
- Optional GICv2m MSI frame exists at 0x0802_0000, but it is out of scope for
  timer bring-up.
- PL011 UART0 remains at 0x0900_0000; its SPI is a later UART-interrupt task,
  not the first timer task.

Accepted QEMU architectural timer facts:

- Timer node compatible: arm,armv8-timer, arm,armv7-timer.
- The timer node has always-on.
- Timer interrupts in DTB order:
  - secure physical: GIC_PPI 13, INTID 29, flags 0x104.
  - non-secure physical: GIC_PPI 14, INTID 30, flags 0x104.
  - virtual: GIC_PPI 11, INTID 27, flags 0x104.
  - hypervisor physical: GIC_PPI 10, INTID 26, flags 0x104.

Talos starts at EL2 when QEMU virt is run with virtualization enabled. The
first QEMU timer smoke targets the EL2 hypervisor physical timer path:
CNTHP_*_EL2 plus PPI 10 / INTID 26. Virtual timer, EL1 physical timer, and
lower-EL timer routing remain deferred.

Accepted QEMU implementation contract:

- The focused timer diagnostic runs QEMU with
  `-M virt,gic-version=2,virtualization=on` so Talos enters EL2. The older
  default QEMU smoke may still boot without virtualization and report EL1.
- Before unmasking `PSTATE.I`, Talos sets `HCR_EL2.IMO` so physical IRQs
  route to EL2 while the diagnostic is executing at EL2.
- The diagnostic leaves PPI 10 / INTID 26 in the reset interrupt group, enables
  the GICv2 distributor and CPU interface, sets a permissive priority mask,
  enables the INTID 26 PPI bank bit, and programs `CNTHP_CVAL_EL2` plus
  `CNTHP_CTL_EL2.ENABLE`.
- The current-EL IRQ handler acknowledges the active interrupt with `GICC_IAR`,
  recognizes INTID 26, increments the shared monotonic tick counter, reprograms
  `CNTHP_CVAL_EL2` for the next tick, writes the same acknowledge value to
  `GICC_EOIR`, and returns through the saved `x0..x30` frame.
- Unexpected GIC INTIDs are counted with atomics, EOI'd when they are real
  active INTIDs, and reported after interrupts are masked again. The IRQ hot
  path still does not allocate or format.

The accepted QEMU smoke transcript is captured by
`scripts/qemu-timer-irq-smoke.sh` in `target/qemu-timer-irq-smoke.log`. A
passing run includes EL2 boot, `intid=26`, `irq-count=1`,
`iar=0x0000001a`, and `qemu-timer-irq-smoke: PASS`.
The monotonic tick extension updates this to `tick-count=4 target=4`, proving
periodic reprogramming rather than a one-shot interrupt.

## Raspberry Pi 5 Target

The physical target is talos-rpi5-bcm2712. The Pi 5 DTB root declares
compatible = "raspberrypi,5-model-b", "brcm,bcm2712" and routes interrupts
through the gicv2 node.

The soc@107c000000 bus maps child addresses to CPU physical addresses with:

    ranges = <0x00000000 0x10 0x00000000 0x80000000>

So a child address 0x7fff9000 becomes CPU physical 0x10_7fff9000.

Accepted Pi 5 interrupt-controller facts:

- Main controller: GIC-400 / GICv2 compatible arm,gic-400.
- Distributor base: 0x10_7fff9000, size 0x1000.
- CPU interface base: 0x10_7fffa000, size 0x2000.
- Virtualization control/interface regions are present in the DTB at
  0x10_7fffc000 and 0x10_7fffe000, each size 0x2000, but they are not needed
  for the first timer smoke.
- GIC maintenance interrupt is GIC_PPI 9, INTID 25, flags 0xf04.
- BCM2712 secondary interrupt controllers and the BCM2835-compatible system
  timer are present in the DTB, but they are out of scope for the first
  scheduler clock.

Accepted Pi 5 architectural timer facts:

- Timer node compatible: arm,armv8-timer.
- Timer interrupts in DTB order:
  - secure physical: GIC_PPI 13, INTID 29, flags 0xf08.
  - non-secure physical: GIC_PPI 14, INTID 30, flags 0xf08.
  - virtual: GIC_PPI 11, INTID 27, flags 0xf08.
  - hypervisor physical: GIC_PPI 10, INTID 26, flags 0xf08.
  - hypervisor virtual: GIC_PPI 12, INTID 28, flags 0xf08.

Talos' accepted Pi 5 boot evidence enters non-secure EL2. The first accepted
Pi 5 timer hardware smoke uses the same EL2 hypervisor physical timer target as
QEMU: CNTHP_*_EL2 plus PPI 10 / INTID 26. The extra Pi 5 hypervisor virtual PPI
is recorded evidence, not an implementation target for the first smoke.

Accepted Pi 5 implementation contract:

- The focused timer IRQ diagnostic is gated by
  TALOS_RPI5_TIMER_IRQ_DIAGNOSTIC, and the focused timer-preemption diagnostic
  is gated by TALOS_RPI5_TIMER_PREEMPTION_DIAGNOSTIC. The normal Pi 5 image
  continues to build without running either smoke.
- Before unmasking PSTATE.I, Talos sets HCR_EL2.IMO, masks stale
  CNTHP_CTL_EL2 state, enables PPI 10 / INTID 26 in the GIC-400 distributor
  and CPU interface, and programs CNTHP_CVAL_EL2 plus CNTHP_CTL_EL2.ENABLE.
- The current-EL IRQ handler acknowledges the active interrupt with GICC_IAR,
  recognizes INTID 26, increments the shared monotonic tick counter, reprograms
  CNTHP_CVAL_EL2 for the next tick, writes the same acknowledge value to
  GICC_EOIR, and returns through the saved x0..x30 frame.
- Unexpected GIC INTIDs are counted with atomics, EOI'd when they are real
  active INTIDs, and reported after interrupts are masked again. The IRQ hot
  path still does not allocate or format.

The accepted hardware evidence is in
tasks/evidence/2026-05-24-pi5-el2-timer-irq-smoke/. The serial capture shows
rpi5-timer-irq-smoke: irq-count=1 vector=5 iar=0x0000001a intid=26
unexpected=0 ctl=0x2, the GIC state after EOI, continued post-IRQ workload,
and rpi5-timer-irq-smoke: PASS. TFTP evidence shows the Pi fetched the
86,429-byte kernel_2712.img served from the candidate archive.
The monotonic tick evidence extends this shape to `tick-count=4 target=4` on the
same INTID with continued post-tick workload progress.
The timer-preemption hardware evidence extends it again in
tasks/evidence/2026-05-24-pi5-timer-preemption-hardware-proof/: the Pi fetched
the 103,152-byte candidate `kernel_2712.img`, reached
`rpi5-timer-preemption-smoke`, reported task1=3, task2=3, ticks=6,
requests=6, handled=6, timer-preemptions=6, dispatch-switches=6,
voluntary-yields=0, vector=5, iar=0x0000001a, INTID 26, unexpected=0, and PASS.

## Timer-Smoke Checkpoint

The accepted QEMU and Pi 5 smokes prove the EL2 physical timer interrupt
boundary on both current targets. The shared contract is:

- Program `CNTHP_CVAL_EL2` and `CNTHP_CTL_EL2` for PPI 10 / INTID 26.
- Route physical IRQs to EL2 with `HCR_EL2.IMO` while executing at EL2.
- Acknowledge with `GICC_IAR`, recognize INTID 26, reprogram the EL2 physical
  timer before EOI, write `GICC_EOIR`, and return through the saved current-EL
  IRQ frame.
- Keep allocation and formatting outside the IRQ hot path; report bounded
  counters after IRQs are masked again.

## Monotonic Tick Accounting

Talos now has a minimal single-core monotonic tick counter for the accepted EL2
physical timer path. The diagnostic cadence is one centisecond of the reported
`CNTFRQ_EL0` frequency, with a 1,000-counter floor for unusually small
frequencies. The proof target is four ticks.

The IRQ hot path is intentionally narrow:

- no allocation, formatting, serial output, scheduler callbacks, or sleeping;
- relaxed atomic tick accounting only;
- reprogram `CNTHP_CVAL_EL2` from the current architectural counter before
  writing `GICC_EOIR`, so the level interrupt is deasserted before completion;
- keep the GIC acknowledgement/EOI value target-local while the tick counter and
  reprogramming policy live in the shared generic-timer module.

This is still not scheduler policy. It does not define time slicing, sleep
queues, preemption disable counters, wall-clock time, SMP per-core state,
lower-EL timer routing, or POSIX clocks. The next Phase 4 slice is the explicit
interrupt masking and critical-section contract.

## Single-Core Critical Sections

Talos now has a deliberately small interrupt masking contract for single-core
kernel code. `single_core_irq_mask_save()` snapshots `DAIF`, masks `PSTATE.I`,
and returns a `SingleCoreIrqMaskState`; `single_core_irq_restore()` restores the
previous IRQ-mask state from that snapshot. A nested critical section therefore
keeps IRQs masked when the outer scope entered masked, while a scope entered
with IRQs unmasked restores unmasked delivery on exit.

This policy is only a boot-CPU critical-section primitive. It does not provide
SMP mutual exclusion, a spinlock, a blocking lock, a sleepable lock, a preemption
disable counter, or lower-EL interrupt policy. Scheduler work may use it to
protect very short single-core invariants, but SMP will need per-core state and
real locking before secondary cores can share scheduler data.

The QEMU EL2 timer diagnostic proves the contract before the timer workload by
checking nested masked restore and unmasked save/restore. It also wraps each
bounded workload iteration in a short save/restore critical section while the
periodic timer still reaches the four-tick proof target. Diagnostic output
remains outside the IRQ handler, after interrupts are masked again.

The QEMU and Pi 5 timer-preemption smokes keep this timer IRQ contract. INTID
26 adds only a bounded preemption-request counter to the hot path; the
scheduler dispatch, context switch, and diagnostic output happen after the
handler has reprogrammed the timer, written GICC_EOIR, and returned from IRQ
context.

After the scheduler/preemption consolidation, the timer IRQ ownership boundary
is unchanged. The shared QEMU and Pi 5 hot path may acknowledge with
`GICC_IAR`, classify INTID 26, store vector/IAR/INTID evidence, record the
monotonic tick, increment a bounded preemption-request counter when the
timer-preemption diagnostic is enabled, reprogram `CNTHP_CVAL_EL2`, write
`GICC_EOIR`, and return. It must not allocate, format, print to serial, block,
sleep, call scheduler dispatch, mutate runnable queues, or perform the context
switch.

`TALOS_RPI5_TIMER_IRQ_DIAGNOSTIC` and
`TALOS_RPI5_TIMER_PREEMPTION_DIAGNOSTIC` remain retained hardware-validation
surfaces, not supported kernel interfaces. Their owner is Phase 4 validation;
their role is to recreate serialized Pi 5 evidence for EL2 physical timer IRQ
delivery and timer-driven scheduler handoff; their revisit condition is Phase 4
closeout or the first Phase 5 local diagnostic command channel that can report
the same counters without special boot images.

## Minimal GICv2 Register Checklist

The next implementation task should keep GICv2 target-specific but share the
small architectural register surface where possible.

Minimum distributor/CPU-interface register references from the Linux GICv2
headers:

- Distributor:
  - GICD_CTLR offset 0x000.
  - GICD_ISENABLERn base offset 0x100.
  - GICD_ICENABLERn base offset 0x180.
  - GICD_IPRIORITYRn base offset 0x400.
  - GICD_ITARGETSRn base offset 0x800 for SPIs; PPIs are per-CPU/banked and
    should not need target routing for the first timer smoke.
  - GICD_ICFGRn base offset 0xc00; PPI configurability is implementation
    defined, so DTB trigger flags are evidence but the task must tolerate fixed
    PPI trigger behavior.
- CPU interface:
  - GICC_CTLR offset 0x00.
  - GICC_PMR offset 0x04.
  - GICC_IAR offset 0x0c.
  - GICC_EOIR offset 0x10.

For the first diagnostic, set a permissive priority mask, enable the CPU
interface, enable the distributor, enable INTID 26 in the PPI/SGI enable
register bank, then acknowledge and EOI the active interrupt in the IRQ handler.
Do not add MSI, SPI target routing, cascaded interrupt controllers, UART
interrupts, or RP1 interrupt routing in the first timer task.

## Minimal Generic-Timer Checklist

Linux's ARM architected timer driver uses the same basic shape Talos needs for
the first diagnostic:

- Program the selected timer compare value or delta.
- Set ENABLE.
- Clear the interrupt mask bit.
- In the handler, observe the interrupt status and mask or reprogram the timer
  before returning.

For EL2 hypervisor physical timer bring-up, the first tasks should use the EL2
CNTHP_*_EL2 register family and read the counter/frequency from the
architectural counter registers. The diagnostic must prove interrupt delivery by
showing timer programming, interrupt unmasking, IRQ handler entry/count, and
return to a simple workload or post-IRQ line. Polling the counter is not
acceptance for timer delivery.

## Deferred Uncertainties

These items are deliberately outside this inventory and must not be hidden in
the first implementation tasks:

- Whether Pi 5 PPI polarity/configuration bits are writable or fixed by GIC-400
  implementation behavior. The DTB records active-low level flags; the driver
  should avoid depending on reprogramming PPI trigger mode for acceptance.
- Whether Talos should eventually use EL2 physical, EL2 virtual, EL1 physical,
  or virtual timers for scheduler time after lower-EL work exists.
- UART interrupts, BCM2835 system timer SPIs, BCM2712 secondary interrupt
  controllers, RP1/PCIe interrupts, MSI, DMA, and IOMMU/cache policy.
- SMP routing, per-core timer setup, secondary-core enablement, and
  interrupt-safe locking beyond the boot CPU.
- Lower-EL/user timer access and CNTKCTL_EL* policy.

## Next Implementation Tasks

The source-backed order is:

1. Establish the IRQ entry/exit saved-register frame contract with interrupts
   still disabled. This is now the accepted Phase 4 entry foundation: normal
   vector slots save `x0..x30`, current-EL IRQs call
   `rust_irq_handler(vector, elr, spsr, frame)`, and the shim restores the full
   frame before `ERET`. The Rust stub only records an unexpected-IRQ count and
   the last vector/ELR/SPSR with atomics; it does not allocate, print, program a
   controller, acknowledge an interrupt, or enable delivery.
2. Add a QEMU-only GICv2 plus EL2 generic-timer smoke for PPI 10 / INTID 26.
   This is accepted at commit `bce215d`.
3. Carry the same EL2 timer shape to Pi 5 GIC-400 hardware with serialized lab
   evidence. This is accepted at commit `966d453`.
4. Add monotonic tick accounting, then critical-section policy, before any
   scheduler task structures.
