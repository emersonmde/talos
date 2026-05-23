# Lower-EL and Userspace Mapping Readiness

Talos currently runs the accepted Raspberry Pi 5 kernel path at EL2 with one
early stage-1 translation regime. The map is a kernel bring-up map, not a
userspace contract.

## Current EL2 Map

The accepted early table skeleton uses `TTBR0_EL2` and 4 KiB table pages from
the fixed bootstrap reservation. It maps:

- low DRAM `0x0..0x40000000` with 2 MiB normal-memory identity blocks,
- and the BCM2712 local-peripheral window `0x107c000000..0x1080000000` with
  2 MiB device identity blocks.

The matching register plan is still the early EL2 plan:

```text
talos: translation control plan: el=0x2 mair=0x4ff tcr=0x53510 ttbr0=0x2f000000 sctlr_set=0x1 va_bits=0x30 pa_bits=0x30 kind=el2-stage1-4k-no-enable
talos: translation enabled: el=0x2 sctlr=0x30c50831 ttbr0=0x2f000000 kind=el2-stage1-4k-enabled
```

This proves that the early kernel can keep running with EL2 stage-1 translation
enabled, normal memory cacheable, and required MMIO reachable as device memory.
It also proves the current same-EL exception path can report BRK, undefined
instruction, data-abort, and translation-fault diagnostics with saved status and
register state.

## What The Map Does Not Prove

The current identity map does not prove a safe lower-EL execution environment.
It does not establish:

- a user/kernel virtual address split,
- user-accessible page descriptors or per-process page tables,
- kernel-only permissions for kernel text, data, stacks, heap, bootstrap tables,
  DTB data, UART, GIC-local, or other MMIO mappings,
- lower-EL vector routing, trap return, or production fault recovery,
- user stack, heap, code, data, guard, or shared-memory mappings,
- a syscall ABI,
- or safe copy-in/copy-out and invalid-user-memory behavior.

No code should treat the accepted EL2 identity map as permission to enter EL0 or
to run untrusted payloads. The map is intentionally broad enough for early
kernel bring-up and intentionally too broad for userspace isolation.

## Minimum Prerequisites Before EL0 Work

Before Phase 7 can accept EL0 or syscall work, Talos needs explicit design and
validation for:

- the address-space shape: kernel half, user range, guard gaps, and whether the
  kernel remains globally mapped while a user task runs;
- descriptor permissions: privileged-only kernel pages, executable user text,
  non-executable user data/stacks, and no user access to MMIO;
- TTBR/TCR/SCTLR policy for the lower-EL regime, including which exception
  level owns process address spaces;
- exception routing from lower ELs, including saved user register frames,
  `ERET` return rules, and fatal-versus-recoverable fault policy;
- user stack and heap mapping creation from accepted frame ownership, without
  using deferred high-memory or DMA ownership;
- an SVC/syscall ABI with stable argument, return-value, and error conventions;
- copy-in/copy-out helpers that reject invalid user pointers without corrupting
  kernel state;
- and tests or hardware evidence for bad user pointers, bad instruction fetches,
  bad stack accesses, and successful trap return.

Until that work exists, Phase 4 interrupt/timer/preemption tasks may rely on the
current EL2 kernel map only for kernel execution. They must not assume process
isolation, lower-EL recovery, or user memory validation.

## Validation Boundary

This readiness note is documentation only. It changes no Rust code, linker
layout, boot image, translation table contents, MAIR/TCR/TTBR/SCTLR programming,
exception-vector code, allocator policy, or normal Pi 5 boot output. No
hardware run is required for the note itself.
