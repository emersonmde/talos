# Memory Management

Talos' Pi 5 physical-memory work is still in the no-allocation bring-up phase.
The normal boot path reads only the firmware-provided FDT data needed to seed a
conservative low-memory candidate:

- the FDT reservation block,
- the root `/memory` node's `reg` banks,
- first-level `/reserved-memory` child-node ranges,
- the physical DTB blob range from the accepted boot handoff,
- and the linker-owned kernel, early heap reservation, and boot stack range.

The current policy is intentionally narrow. Talos selects the low `/memory` bank
that contains the kernel image, reserves after the kernel/runtime range, DTB
blob, FDT reservation entries, and any nonzero reported `/reserved-memory` range
that intersects that bank, then aligns the remaining tail to 4 KiB. This
produces one `low-tail` usable candidate for future allocator work; it is not a
complete physical memory map and does not claim high memory.

On the accepted Pi 5 boot, the reported banks are:

```text
talos: dtb memory[0]: addr=0x0 size=0x3fc00000
talos: dtb memory[1]: addr=0x40000000 size=0xc0000000
talos: dtb memory[2]: addr=0x100000000 size=0x100000000
```

The accepted low-tail candidate remains:

```text
talos: memory usable: bank=0 start=0x2f000000 end=0x3fc00000 size=0x10c00000 align=0x1000 policy=low-tail
```

The normal Pi 5 path intentionally emits this candidate twice for now. The
first copy stays formatter-free before page-frame seed and translation-table
layout, and the second copy uses ordinary `println!` after data-cache enablement,
bootstrap allocator initialization, and the bounded String smoke. Hardware
acceptance for the `println!` copy captured it immediately before the accepted
post-allocator bootstrap-reserve and page-frames-remaining reports. The
candidate is still a conservative low-tail policy decision, not a complete
physical memory map or allocator ownership transfer.

The first allocator-prep step now exposes that candidate as a page-frame seed
span:

```text
talos: page frames seed: start=0x2f000000 end=0x3fc00000 pages=0x10c00 page_size=0x1000 source=memory-usable
```

The normal Pi 5 path intentionally emits this seed twice for now. The first copy
stays formatter-free before bootstrap reservation and translation-table layout.
The second copy uses ordinary `println!` after data-cache enablement, bootstrap
allocator initialization, the bounded String smoke, and the post-allocator
memory-usable line:

```text
talos: page frames seed: start=0x2f000000 end=0x3fc00000 pages=0x10c00 page_size=0x1000 source=memory-usable phase=post-allocator
```

This line means the span is page-aligned and large enough to describe 4 KiB
frames. It does not mean Talos has initialized a mutable allocator, handed the
span to a free list, or changed the underlying low-tail candidate. The
post-allocator `println!` copy is a human-readable report on the accepted
formatter surface, not a new ownership boundary.

The next bootstrap boundary reserves the first 16 pages, or 64 KiB, from the
seed for early page-table/bootstrap work and reports the remaining span:

```text
talos: bootstrap reserve: start=0x2f000000 end=0x2f010000 pages=0x10 page_size=0x1000 reason=bootstrap-page-tables
talos: page frames remaining: start=0x2f010000 end=0x3fc00000 pages=0x10bf0 page_size=0x1000 source=bootstrap-reserve
```

The bootstrap-reserve report is intentionally emitted twice on the current
normal Pi 5 path. The first copy stays formatter-free before translation table
layout so early-memory diagnostics remain available before the accepted
post-data-cache `println!` boundary. The second copy uses ordinary `println!`
after data-cache enablement, bootstrap allocator initialization, and the
bounded String smoke, immediately before the accepted `page frames remaining`
line. Hardware acceptance for the `println!` copy captured both reports in
that order, then the DTB memory summary and entries. This reservation is still
static no-allocation policy. It does not build translation tables, enable the
MMU by itself, create allocator metadata, or transfer ownership to a mutable
physical page allocator. MMIO exclusions, high-memory policy, allocator
metadata placement, and ownership handoff remain follow-up work.

Talos now derives a fixed layout-only translation-table staging area from the
front of that bootstrap reservation:

```text
talos: translation tables: start=0x2f000000 end=0x2f004000 pages=0x4 page_size=0x1000 kind=layout-only
talos: translation table slots: root=0x2f000000 l1=0x2f001000 l2_low=0x2f002000 l2_mmio=0x2f003000
```

The layout consumes four 4 KiB table slots inside the reserved 64 KiB bootstrap
span and leaves the reported post-reservation page-frame seed unchanged. These
addresses are the only table pages Talos mutates during the current early MMU
prep.

Talos now zeroes and populates those four pages with a deterministic stage-1
4 KiB translation skeleton, but still does not enable translation:

```text
talos: translation table population: root_entries=0x1 l1_entries=0x2 low_l2_blocks=0x200 mmio_l2_blocks=0x20 block_size=0x200000 kind=stage1-4k-no-enable
talos: translation map policy: low=0x0..0x40000000 mmio=0x107c000000..0x1080000000 root_index=0x0 low_l1_index=0x0 mmio_l1_index=0x41 normal_attr=0x0 device_attr=0x1
```

The skeleton uses `root[0]` to reach the L1 table, `l1[0]` to reach the
low-memory L2 table, and `l1[0x41]` to reach the BCM2712 local-peripheral L2
table. The low table maps the first 1 GiB as 2 MiB normal-memory block
descriptors. The MMIO table maps only the `0x107c000000..0x1080000000`
BCM2712 local-peripheral window as 2 MiB device block descriptors, covering the
accepted UART10/GIC-local window needed by the near-term console/MMU path.

Talos also reports the EL2 register plan that matches the populated skeleton:

```text
talos: translation control plan: el=0x2 mair=0x4ff tcr=0x53510 ttbr0=0x2f000000 sctlr_set=0x1 va_bits=0x30 pa_bits=0x30 kind=el2-stage1-4k-no-enable
```

This line is printed immediately before Talos performs its first controlled EL2
MMU-enable diagnostic. `MAIR_EL2=0x4ff` assigns attr index 0 to normal
write-back/write-allocate memory and attr index 1 to Device-nGnRE memory.
`TCR_EL2=0x53510` selects a 48-bit VA/PA shape, 4 KiB granule, inner-shareable
translation walks, and write-back/write-allocate walk cacheability.
`TTBR0_EL2` points at the accepted root table, and `sctlr_set=0x1` is the
`SCTLR_EL2.M` enable bit ORed into the existing EL2 control value.

The enable path writes `MAIR_EL2`, `TCR_EL2`, and `TTBR0_EL2`, invalidates EL2
translations, sets `SCTLR_EL2.M`, then emits immediate and formatted
post-enable serial evidence:

```text
TALOS: mmu enable start
TALOS: mmu enable done
talos: translation enabled: el=0x2 sctlr=0x30c50831 ttbr0=0x2f000000 kind=el2-stage1-4k-enabled
```

Talos also has a controlled Pi 5 translation-fault diagnostic for this exact
early EL2 map. The diagnostic minimizes unrelated DTB reporting, enables the
same EL2 stage-1 tables, then deliberately reads from `0x80000000`, which is
outside the current identity map. Accepted hardware output shows the post-MMU
line followed by:

```text
TALOS: before translation fault va0x80000000 vbar=0x200800 el=2
exception-info: esr=0x0000000096000005 elr=0x0000000000209010 far=0x0000000080000000
exception-class: data-abort-same-el ec=0x25
```

This proves the current fatal exception path can report an early translation
fault with the expected fault address after `SCTLR_EL2.M` is enabled. It is
diagnostic-only: Talos still halts after the report and does not implement page
fault recovery, demand mapping, lower-EL faults, or a general virtual-memory
policy.

Normal Pi 5 boot now enables the EL2 instruction cache after the accepted
stage-1 map is active. The helper requires EL2 and `SCTLR_EL2.M`, invalidates
the instruction cache to PoU with `ic iallu`, then sets only `SCTLR_EL2.I`.
Accepted hardware output shows:

```text
talos: translation enabled: el=0x2 sctlr=0x30c50831 ttbr0=0x2f000000 kind=el2-stage1-4k-enabled
talos: instruction cache plan: el=0x2 sctlr_before=0x30c50831 sctlr_set=0x1000 kind=el2-stage1-icache-enabled
TALOS: icache enable start
TALOS: icache enable done
talos: instruction cache enabled: el=0x2 sctlr=0x30c51831 kind=el2-stage1-icache-enabled
```

The instruction-cache-enabled status report remains formatter-free. A
2026-05-23 attempt to emit that pre-data-cache line through ordinary
`println!` passed local gates and was TFTP-served as a 79,773-byte normal Pi 5
image, but the hardware run did not produce fresh Talos-origin serial after the
firmware/RP1 boot lines. Treat that as a non-accepted boundary: the first
currently accepted pre-allocator `println!` cache report is the later
data-cache-enabled line after `SCTLR_EL2.C` is set.

Normal Pi 5 boot now enables the EL2 data cache after both the stage-1 map and
instruction cache are active. The helper requires EL2, `SCTLR_EL2.M`, and
`SCTLR_EL2.I`, invalidates data/unified caches by set/way, then sets only
`SCTLR_EL2.C`. The final cache-enabled report now uses the ordinary
`println!` surface, while the surrounding start/done markers stay formatter
free for the actual transition. Accepted hardware output shows:

```text
talos: data cache plan: el=0x2 sctlr_before=0x30c51831 sctlr_set=0x4 kind=el2-stage1-dcache-enabled
TALOS: dcache enable start
TALOS: dcache enable done
talos: data cache enabled: el=0x2 sctlr=0x30c51835 kind=el2-stage1-dcache-enabled
```

This is still an early-kernel cache boundary, not a complete coherency policy.
Device mappings remain Device-nGnRE and Talos has not yet defined DMA buffer
ownership, explicit clean/invalidate APIs for drivers, or allocator metadata
placement under the cache-enabled regime.

## Bootstrap Allocator Smoke

Normal Pi 5 boot now derives a first bootstrap allocator plan from the
post-page-table remaining frame span. The plan only accepts the low identity map
window, requires 4 KiB alignment, and starts after the fixed 16-page bootstrap
reservation at `0x2f010000`. A no-free bump allocator owns that span for the
current early-runtime smoke path.

Talos installs this allocator as the kernel `#[global_allocator]` and builds
the Pi 5 target with `alloc` available. The first accepted alloc-crate
container was a bounded `Box<[u64; 4]>` after allocator initialization. The
next accepted container was a bounded `Vec<u64>` capacity/fill smoke after the
same allocator initialization. The current accepted steady boot path uses a
bounded ASCII `String` smoke after the same allocator initialization.
Collections, free/reuse semantics, container growth beyond preallocated
capacity, and general-purpose allocation in kernel code remain deferred.

The accepted Box hardware run allocated the box through the global allocator,
read its values, kept the existing oversized direct-allocation exhaustion guard,
and printed:

```text
talos: bootstrap allocator plan: start=0x2f010000 end=0x3fc00000 bytes=0x10bf0000 pages=0x10bf0 page_size=0x1000 kind=bump-no-free-low-tail
talos: bootstrap allocator init: start=0x2f010000 next=0x2f010000 end=0x3fc00000 policy=no-free
talos: box alloc smoke: ptr=0x2f010000 items=4 sum=0x47 next=0x2f010020 used=0x20 remaining=0x10beffe0 exhaustion_ok=true ok=true
```

The accepted Vec hardware run allocates capacity for four `u64` values, writes
exactly those four slots without requiring growth, reads the values back, keeps
the same oversized direct-allocation exhaustion guard, and prints:

```text
talos: vec smoke: ptr=0x2f010000 len=4 cap=4 sum=0x47 next=0x2f010020 used=0x20 rem=0x10beffe0 ex=true ok=true
```

The accepted String hardware run allocates capacity for eight bytes, fills the
ASCII payload `Talos` without requiring growth, reads the bytes back, verifies
the allocation pointer is stable after the fill, keeps the same oversized
direct-allocation exhaustion guard, and prints:

```text
talos: string smoke: ptr=0x2f010000 len=5 cap=8 sum=0x203 next=0x2f010008 used=0x8 rem=0x10befff8 ex=true stable=true ok=true
```

The current normal Pi 5 boot emits the bootstrap allocator plan, allocator-init
accounting, and final String-smoke accounting lines through the ordinary
`println!` surface after the MMU, instruction cache, and data cache are enabled.
Hardware evidence for the allocator-init promotion required the Pi 5 formatter
backend to poll the UART10 TX-ready flag before each byte; the earlier unpolled
chunk-write path could drop the `start=` literal from the longer formatted line.
The follow-up allocator-plan promotion proved the same polled backend preserves
the longer pre-init plan line, and the data-cache-enabled promotion proved one
pre-allocator cache-transition status line can use the same surface:

```text
talos: data cache enabled: el=0x2 sctlr=0x30c51835 kind=el2-stage1-dcache-enabled
talos: bootstrap allocator plan: start=0x2f010000 end=0x3fc00000 bytes=0x10bf0000 pages=0x10bf0 page_size=0x1000 kind=bump-no-free-low-tail
talos: bootstrap allocator init: start=0x2f010000 next=0x2f010000 end=0x3fc00000 policy=no-free
talos: string smoke: ptr=0x2f010000 len=5 cap=8 sum=0x203 next=0x2f010008 used=0x8 rem=0x10befff8 ex=true stable=true ok=true
talos: memory usable: bank=0 start=0x2f000000 end=0x3fc00000 size=0x10c00000 align=0x1000 policy=low-tail
```

The allocation behavior and oversized direct-allocation guard remain unchanged;
the accepted boundary is still a small set of post-cache, allocator-adjacent
runtime reports using the Daedalus-like `print!`/`println!` path with
formatted fields.

Talos also has an explicit Pi 5 allocation-failure diagnostic image gated by
`TALOS_RPI5_ALLOC_OOM_DIAGNOSTIC`. It runs after
`KERNEL_GLOBAL_ALLOCATOR` initialization and requests one `Vec<u8>` capacity
larger than the remaining bump span. This is a fatal diagnostic boundary: the
accepted hardware run proves the `alloc_error_handler` line is readable, then
Talos spins instead of pretending that OOM is recoverable.

```text
talos: alloc oom diagnostic: request=0x10bf0008 remaining=0x10bf0000 align=0x1
talos: alloc error: size=0x10bf0008 align=0x1
```

Talos also has a direct realloc growth diagnostic gated by
`TALOS_RPI5_REALLOC_GROWTH_DIAGNOSTIC`. It allocates two bytes through the
global allocator, grows that allocation to four bytes through
`GlobalAlloc::realloc`, verifies the copied prefix plus newly written tail, and
reports the no-free bump-accounting boundary:

```text
talos: realloc grow smoke: old=0x2f010000 new=0x2f010002 size=4 sum=0x47 next=0x2f010006 used=0x6 rem=0x10befffa ex=true moved=true ok=true
```

This proves the current realloc growth path allocates a new region and retains
the old region by policy. It is not a free/reuse policy, not a recoverable OOM
policy, and not by itself an acceptance of alloc-crate container growth.

Talos also has a cfg-gated `Vec<u8>` growth diagnostic under
`TALOS_RPI5_VEC_GROWTH_DIAGNOSTIC`. It creates a vector with capacity two,
fills those two slots, then uses `reserve_exact(2)` to force alloc-crate growth
through the global allocator before filling and verifying four bytes. The
accepted hardware run reports that the no-free bump allocator moved the vector
from the original two-byte allocation to a new four-byte allocation:

```text
talos: vec grow start
talos: vec grow smoke: old=0x2f010000 new=0x2f010002 len=4 cap=4 sum=0x47 next=0x2f010006 used=0x6 rem=0x10befffa ex=true moved=true ok=true
```

This accepts one narrow grow-on-demand `Vec` path. It does not accept
`String` growth, collection-heavy kernel code, deallocation/reuse,
recoverable OOM, allocator expansion, or page-frame-backed heap growth.

Talos also has a cfg-gated `String` growth diagnostic under
`TALOS_RPI5_STRING_GROWTH_DIAGNOSTIC`. It creates an ASCII string with
capacity two, writes the first two bytes, then uses the string's backing
`Vec<u8>` `reserve_exact(2)` path to force growth before filling and
verifying four bytes. The accepted hardware run reports the same no-free bump
growth boundary as the direct realloc and `Vec` growth diagnostics:

```text
talos: string grow start
talos: string grow smoke: old=0x2f010000 new=0x2f010002 len=4 cap=4 sum=0x190 next=0x2f010006 used=0x6 rem=0x10befffa ex=true moved=true ok=true
```

This accepts one narrow ASCII `String` grow-on-demand path. It does not accept
general string formatting, UTF-8 mutation policy, collection-heavy kernel code,
deallocation/reuse, recoverable OOM, allocator expansion, or page-frame-backed
heap growth.

Talos also has a cfg-gated `alloc::format!` diagnostic under
`TALOS_RPI5_ALLOC_FORMAT_DIAGNOSTIC`. It runs after
`KERNEL_GLOBAL_ALLOCATOR` initialization, builds the formatted string
`alloc::format!("{} {}", "Talos", 5usize)`, verifies the exact ASCII bytes,
keeps the oversized direct-allocation guard, and reports allocator accounting:

```text
talos: alloc format start
talos: alloc format smoke: ptr=0x2f010000 len=7 cap=7 sum=0x258 next=0x2f010007 used=0x7 rem=0x10befff9 ex=true ascii=true ok=true
```

The accepted Pi 5 hardware archive uses the same 76,152-byte staged image size
as the accepted `String` growth diagnostic plus a fresh-entry marker, because
the unpadded alloc-format image and a plain padded variant both failed before
Talos-origin serial output. This accepts one narrow alloc-backed formatting
construction path, not general runtime string-formatting policy for kernel code.

The allocator state now preserves `start`, `next`, `end`, `used`, and
`remaining` accounting. The smoke path also attempts one deliberately oversized
direct allocation and requires it to return null without advancing `next`. This
is an exhaustion guard for the bump allocator contract, not a free/reuse policy.

Earlier broad `alloc`-crate variants using multiple containers at once were
deferred after inconclusive hardware captures. The accepted boundary is now the
global allocator symbol plus one `Box<[u64; 4]>` smoke, one bounded `Vec<u64>`
capacity/fill smoke, one bounded ASCII `String` fill smoke, and one explicit
fatal alloc-crate OOM diagnostic under the already-enabled MMU, instruction
cache, and data cache. It also has one direct realloc growth diagnostic for the
underlying global allocator, one cfg-gated `Vec<u8>` growth diagnostic, and one
cfg-gated ASCII `String` growth diagnostic that use the alloc-crate reserve
path. Talos has not accepted recoverable OOM handling, collection-heavy runtime
allocation, UTF-8/string policy beyond the four-byte ASCII diagnostics, broad
runtime string-formatting policy beyond the explicit `alloc::format!` smoke, or
free/reuse.

During the follow-up `Vec` smoke investigation, normal dev-profile Pi 5 images
hit a hardware handoff/output cliff between 181,176 and 181,184 bytes, and an
optimized-image classifier later exposed a narrow file-length ceiling at 80,231
bytes. Pi 5 hardware image generation now defaults to a size-optimized dev build
(`CARGO_PROFILE_DEV_OPT_LEVEL=z`) while preserving the existing debug artifact
path. Profile-level shrink mitigations such as LTO and disabled debug assertions
were rejected after hardware controls failed before Talos-origin output. The
accepted bounded `Vec` run instead keeps the accepted profile and relaxes only
the Pi 5 linker script's `.rodata` section alignment from 4 KiB to 16 bytes,
producing a 79,928-byte image that boots through the bounded Vec smoke. The
later accepted `Vec` and `String` growth diagnostics stay below the current
image-size ceiling by keeping only the growth-specific output in their cfg-gated
paths and by avoiding extra padding by default.

The normal memory-bank summary and entry reports are also hardware-accepted
through ordinary `println!` after the accepted page-frame reservation line. The
reports preserve the DTB parser and memory-bank accounting, and on the current
Pi 5 boot report:

```text
talos: dtb memory: address_cells=2 size_cells=2 count=3 shown=3 truncated=false
talos: dtb memory[0]: addr=0x0 size=0x3fc00000
talos: dtb memory[1]: addr=0x40000000 size=0xc0000000
talos: dtb memory[2]: addr=0x100000000 size=0x100000000
```

This is a logging-surface acceptance point only. It does not change bank
selection, reservation handling, or ownership of the remaining physical frames.

The current mapping is still intentionally narrow. Talos has not enabled data
cache-aware DMA ownership, selected a final complete attribute policy, mapped
the RP1 PCIe aperture or high memory, added lower-EL translation, implemented
translation-fault recovery, accepted general collections or unconstrained
grow-on-demand containers, or made a mutable page allocator responsible for the
remaining frames.
