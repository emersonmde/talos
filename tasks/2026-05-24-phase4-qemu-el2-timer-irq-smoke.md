# Phase 4 QEMU EL2 Timer IRQ Smoke

Task: `phase4-qemu-gicv2-el2-timer-smoke-20260524`

Status: accepted locally and committed as the QEMU-only proof before Pi 5
hardware carry-over.

## Scope

- QEMU virt only, run with `-M virt,gic-version=2,virtualization=on`.
- GICv2 distributor `0x08000000`, CPU interface `0x08010000`.
- EL2 hypervisor physical timer, PPI 10 / INTID 26, using `CNTHP_*_EL2`.
- No Pi 5 hardware run, scheduler, preemption, SMP, UART interrupts, SPIs, MSI,
  RP1/PCIe, DMA, or lower-EL timer routing.

## Evidence

Final `scripts/qemu-timer-irq-smoke.sh` evidence level: QEMU EL2
timer-interrupt smoke.

```text
Talos 0.1.0 booting on talos-aarch64-virt
boot-info: dtb_pa=0x0000000048000000 core=0 el=2 target=talos-aarch64-virt
target-services: uart=pl011 timer=arm-generic irq=gic-v2 dtb=Some(
    0x0000000048000000,
)
mmio-regions: 3
qemu-timer-irq-smoke: gicd=0x08000000 gicc=0x08010000 intid=26
qemu-timer-irq-smoke: cntfrq=62500000 start=889427 cval=1514427 delta=625000
qemu-timer-irq-smoke: irq-count=1 vector=5 iar=0x0000001a intid=26 unexpected=0 ctl=0x2
qemu-timer-irq-smoke: gic enable=0x0400ffff pending=0x00000000 active=0x00000000 hppir=0x000003ff daif=0x3c0
qemu-timer-irq-smoke: post-irq workload=0x8de234a4a9c61080 remaining=990286
qemu-timer-irq-smoke: PASS
```

The handler entered through vector 5/current-SPx IRQ, acknowledged INTID 26 via
`GICC_IAR`, masked the EL2 physical timer, wrote `GICC_EOIR`, recorded one
bounded timer count and zero unexpected GIC interrupts, and returned to the
post-IRQ workload.

## Validation

- `cargo fmt --all -- --check`: passed.
- `cargo -Zjson-target-spec test`: passed with 51 no-std tests.
- `scripts/qemu-smoke.sh`: passed as QEMU EL1 substitute/default smoke.
- `scripts/qemu-timer-irq-smoke.sh`: passed as focused QEMU EL2 timer IRQ
  smoke.
- `scripts/rpi5-image.sh`: passed, producing
  `target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712.img`.
- `scripts/rpi5-format-guard-check.sh`: passed.
- `git diff --check`: passed.
- `mdbook build`: unavailable in the container.
