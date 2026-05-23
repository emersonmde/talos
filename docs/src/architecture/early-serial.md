# Early Serial and Printing

Talos uses `print!` and `println!` as the normal kernel-facing printing
interface. The macro shape intentionally matches the Daedalus model: callers use
ordinary `format_args!` syntax, and the target console backend implements
`core::fmt::Write`.

## Console Path

The shared macro path is:

1. `print!` builds `format_args!(...)`.
2. `println!` appends a newline through the standard `concat!($fmt, "\\n")`
   macro shape.
3. `target::console::_print` writes the resulting `fmt::Arguments` through the
   active target console.
4. The console backend implements `core::fmt::Write::write_str`.

QEMU uses the normal PL011 path. The Raspberry Pi 5 path currently uses the
firmware/BL31-visible UART10 console returned by `rpi5::firmware_console()`.
During bring-up, that backend preserves firmware baud programming and uses
32-bit PL011 data-register writes, TX-ready polling, and a flag-register
readback. Hardware diagnostics showed byte-width Rust writes produce
unreadable/binary serial on the lab-visible UART10 path, while the accepted
assembly and Rust-origin paths use word writes to the data register at
`0x107d001000`. A later allocator-init `println!` run showed that long
formatter string chunks could drop literal bytes when the backend only flushed
posted writes after each chunk; polling TX-ready before each byte fixed the
normal formatter line while preserving the word-write UART10 contract.

For Pi 5 static boot labels, `target::console::write_static` now uses the
bytewise early UART10 word-write path and waits for the transmitter to empty
after each byte. This keeps long human-readable labels intact under the current
firmware UART ownership boundary. Formatted hex helpers already use the same
early UART10 path. The normal `core::fmt` path now also paces each byte through
the PL011 TX-ready flag, but the accepted policy is still incremental: bounded
runtime reports are promoted only when hardware evidence proves them readable.
The current accepted normal `println!` surface includes the post-data-cache
status line, bootstrap allocator plan/init reports, the bounded String smoke,
and post-allocator bootstrap-reserve, page-frame remaining, and DTB memory
reports. The instruction-cache enabled report remains formatter-free because
its pre-data-cache `println!` candidate did not produce Talos-origin serial on
hardware.

## Pi 5 Bring-Up Policy

The Pi 5 and lab API are dedicated to Talos development. Failed boots, reset
loops, and broken staged images are normal bring-up evidence. The engineering
priority is to establish reliable Talos-origin serial output first, then build
formatting and richer logging on top of that serial path.

The `early_format` helpers remain available for tiny diagnostics and exception
paths that must avoid the generic formatter. They are an escape hatch, not the
desired public interface for normal kernel code.

## Current Risk

Generic `core::fmt` on Pi 5 early boot is being enabled deliberately to converge
with Daedalus and simplify kernel logging. Current hardware evidence shows the
formatter is not the immediate visible-output boundary: UART10 can emit
Talos-time bytes, but they are unreadable, while the RP1 first-light write shape
does not yet produce normal Rust console text after BL31. The next work should
keep the public `print!`/`println!` API stable and continue narrowing the Pi 5
runtime UART backend.

The current handoff UART diagnostic is intentionally narrower than the normal
console path. Assembly emits repeated `AH` markers immediately before
`rust_entry`; hardware has shown those markers are readable after BL31. The
next boundary check has `rust_entry` immediately call an assembly-owned `RE`
marker routine that does not reprogram RP1 pads or UART registers. It reuses
the same simple data-register write plus flag-register readback shape as the
readable `AH` marker, writes the marker to both the firmware/BL31-visible
UART10 candidate and the RP1 UART0 pcie2 candidate, delays, and parks. If `RE`
appears, Rust entry and the basic post-entry write primitive are proven; if
`AH` remains visible while `RE` is absent, the active boundary is the
`_start` to `rust_entry` call/ABI/runtime transition rather than formatter
internals.

The capture-focused RE rerun proved only that the staged 65,952-byte image was
served through TFTP; its fresh serial window contained firmware DHCP/TFTP text
but no current-run `Starting OS`, BL31, `AH`, or `RE`. The next diagnostic
therefore moves back to the earliest kernel instruction after the arm64 Image
header branch: `TALOS_RPI5_ENTRY_LOOP_DIAGNOSTIC` emits repeated `EL` markers
with the same data-register write plus flag-register readback helper used for
readable `AH`, then loops without entering Rust. That keeps the question narrow:
prove current-run kernel entry and serial capture first, then return to the
Rust-entry marker boundary.

Hardware accepted that entry/capture boundary on 2026-05-21. The extended
window for `rpi5-entry-loop-el-20260521T040000Z` captured two fresh
`da591740/kernel_2712.img` TFTP serves at 79,688 bytes and 21,594 exact
`EL\r\n` serial markers from the entry loop. The first 90-second window ended
before TFTP/kernel entry, so future Pi 5 serial runs should keep a wide enough
post-power capture or explicitly extend capture when only early firmware text
appears. The accepted `EL` result proves current-run kernel entry and the
AH-style write/readback serial primitive, but it is not the final logging API:
the next durable output target is a bounded human-readable `TALOS:` line on
the same proven path.

`TALOS_RPI5_ENTRY_TALOS_LINE_DIAGNOSTIC` is that next step. It keeps the
same earliest-entry loop and UART helper but changes the marker payload from
`EL\r\n` to `TALOS: entry\r\n`. Acceptance for that diagnostic requires a
fresh TFTP serve of the new image and exact `TALOS: entry\r\n` bytes in a
bounded raw serial capture.

Hardware accepted the narrow human-readable entry-line claim on 2026-05-21.
The first run only replaced the root kernel files, while the Pi fetched the
serial-number-prefixed `da591740/kernel_2712.img`; that run was a staging
error and repeated the previous `EL` loop. The corrected run staged the
79,688-byte `268d4729ab49ff69e7cbda2af3b10c1832590f31e4d08fb24d20b515e77afd75`
image at root and under `da591740/`. TFTP served the prefixed kernel, and after
draining stale `EL` serial backlog the raw observe window contained 2,740 exact
`TALOS: entry\\r\\n` lines and zero exact `EL\\r\\n` lines.

That result proves earliest Talos entry assembly can produce human-readable
kernel-origin serial text through the current dual-candidate primitive. It does
not prove Rust entry, console ownership, formatting, `println!`, or which UART
candidate is active. The next diagnostic therefore keeps the same earliest
entry loop but emits distinct labels: `TALOS: uart10\\r\\n` to `0x107d001000`
and `TALOS: rp1-uart0\\r\\n` to `0x1f00030000`.

Hardware accepted `uart10` as the serial-visible candidate on 2026-05-21. The
candidate diagnostic staged the 79,688-byte
`9c78a095f2293ef073185f732d49cff1cd2c479ef71c85d5c2ebe2d2a679ed9a`
image under both root and `da591740/`. TFTP served the prefixed kernel twice,
and the post-drain serial observe window contained 3,444 exact
`TALOS: uart10\\r\\n` lines with zero exact `TALOS: rp1-uart0\\r\\n`,
`TALOS: entry\\r\\n`, or `EL\\r\\n` lines. This pins the lab-visible
earliest-entry primitive to BCM2712 UART10 at `0x107d001000`; it does not prove
the Rust console or formatting stack.

Rust-origin UART10 output was accepted on hardware on 2026-05-21. Earlier Rust
diagnostics that read a static marker string from rodata emitted a repeated
20-byte binary pattern, showing that early Rust static-data addressing is not
yet an accepted contract. A no-rodata diagnostic then emitted immediate byte
constants from Rust through inline assembly. The first no-rodata run proved
Rust-origin UART10 output but corrupted the middle of the line because the
inline-asm input register could overlap scratch registers. The fixed-register
run staged the 70,048-byte
`83cffc24d746e1f7128a2bace0341e9110928f62c46d9bf53cbec14203d73cd5`
diagnostic image, TFTP served `da591740/kernel_2712.img` twice, and bounded
serial captures contained exact `TALOS: rust-uart10\\r\\n` lines from Rust.

This proves the current `_start` -> stack/BSS -> `rust_entry` path can produce
human-readable Talos-origin bytes on UART10 before boot-info parsing, target
init, the console abstraction, or `core::fmt`. It does not yet accept rodata
use, formatted output, panic/exception reporting, or the normal `println!`
banner on Pi 5 hardware. The next layer should route a bounded normal
`print!`/`println!` phase line through the UART10 word-write backend while
staying alert to the unresolved early rodata/load-address contract.

`TALOS_RPI5_PRINTLN_PHASE_DIAGNOSTIC` is the first normal-console probe for
that layer. It reaches the usual Pi 5 `kernel_main` path, prints the existing
early boot banner through `println!`, then loops on both a static
`TALOS: println phase` line and a formatted `TALOS: println count {}` line.
The first hardware run served the 84,024-byte diagnostic image but produced no
println serial lines. A follow-up run with the Pi 5 linker base temporarily
changed to match the arm64 Image header text offset also served the diagnostic
image but produced no println serial lines; that linker change was reverted.
Those runs leave normal Pi 5 `println!`, rodata/static format strings, and
`core::fmt` unaccepted. The accepted serial contract remains the no-rodata
Rust UART10 diagnostic above.

## Phase-Ladder Reset Diagnostics

The current serial bring-up ladder keeps firmware/capture and UART ownership
questions separate from formatter behavior. `TALOS_RPI5_PHASE_LADDER_DIAGNOSTIC`
uses the accepted UART10 data-register write plus flag-register readback helper
and labels early boot phases as `TALOS: P0 asm-entry`, `TALOS: P1 asm-stack`,
and `TALOS: P2 asm-before-rust`.

Hardware accepted the first phase on 2026-05-21 with
`TALOS_RPI5_PHASE_P0_RESET_DIAGNOSTIC`. The diagnostic reloads the P0 marker
pointer on every loop iteration, emits repeated `TALOS: P0 asm-entry\\r\\n`
lines from `_start` before CPACR, BSS clearing, stack setup, or Rust, then
requests PSCI reset. Run `rpi5-phase-p0-reset-20260521T0915Z` served the
70,400-byte
`e2b0297e410be08f512491273b26b4147aa711ee319826cd651aa0a26b09f2ce`
image from `da591740/kernel_2712.img`; the post-TFTP serial observe captured
an exact `TALOS: P0 asm-entry` line. This accepts the narrow claim that the
current Cargo-linked Talos image can emit a human-readable UART10 line at
earliest assembly entry.

That P0 result does not prove BSS clearing, stack setup, `rust_entry`, rodata,
the console backend, or formatting. The next bounded diagnostic is
`TALOS_RPI5_PHASE_P1_RESET_DIAGNOSTIC`, which keeps the P0 entry marker, then
continues through CPACR, BSS clearing, and stack setup before emitting repeated
`TALOS: P1 asm-stack` lines and resetting before Rust.

The first P1 run reached the post-stack area enough to emit exact P0 followed
by a partial `TALOS: P1` fragment, but it did not capture a complete
`TALOS: P1 asm-stack` line. `TALOS_RPI5_PHASE_P1_SHORT_RESET_DIAGNOSTIC`
therefore uses the same post-stack reset point with a shorter repeated
`TALOS: P1` line to separate reachability from long-line/reset timing.

Hardware accepted that short P1 boundary on 2026-05-21. Capture-focused run
`rpi5-phase-p1-short-reset-rerun-20260521T1031Z` reused the staged 70,400-byte
`3d9154708e749634a363f84cb5cbd9598bba2b1540b84ca6d897cc642f9f988c` image;
fresh TFTP served `da591740/kernel_2712.img`, and the bounded serial window
captured BL31/Starting OS context followed by exact `TALOS: P0 asm-entry` and
exact `TALOS: P1` lines. This accepts the narrow claim that UART10 output still
works after CPACR, BSS clearing, and stack setup. The full
`TALOS: P1 asm-stack` line remains unaccepted; the failed full-marker run now
looks like a transmit-drain/reset-timing problem rather than a failure to reach
the post-stack boundary.

The next reset-classified boundary is P2, immediately before `rust_entry`.
The first unpaced P2 short run captured exact P0 and P1 but only a partial next
marker, so the P2 variant now uses a paced helper that waits, bounded, for the
UART transmit FIFO not-full before each byte and for transmit empty before the
PSCI reset. Hardware accepted that paced path in
`rpi5-phase-p2-paced-reset-20260521T1039Z`: the 70,400-byte
`fece3f608932f40c9c1885fa22b2b050ac7b16910f94bdc739246766bbbb6e90` image
was served from `da591740/kernel_2712.img`, and bounded serial captured exact
P0, exact P1, and 64 exact `TALOS: P2` lines. This accepts the narrow claim
that the current assembly path can preserve readable UART10 output through the
point immediately before Rust entry, provided the writer paces/drains before a
fast reset.

The paced writer was then carried into the Pi 5 UART10 console backend: Talos
polls TXFF before each PL011 data-register write and waits, bounded, for TXFE
after flushed writes. Hardware still did not accept the normal println phase.
A marked follow-up image added no-rodata immediate markers around the normal
path. Run `rpi5-println-phase-markered-console-20260521T1059Z` served the
84,072-byte `d48f4318df32548f6f951265270f80a0d524f32619f10ccbc2bb4951b434992a`
image twice and captured BL31/Starting OS plus exact `TALOS: phase 0` through
`TALOS: phase 3`. The first normal static println then emitted binary-looking
bytes instead of `talos: boot start`; `TALOS: phase 4` and the diagnostic
println loop were absent. That localizes the current blocker to static
string/rodata or `fmt::Arguments::as_str()` data representation after exception
init, not to UART10 ownership or Rust-entry reachability.

The follow-up rodata-address diagnostic accepted the static-string address
boundary on hardware. The fixed-hex run
`rpi5-rodata-address-fixedhex-20260521T1130Z` served the 70,704-byte
`c63521a95d529f2b7b14916881d059e5959f73f000f072879d45dc33ba61d777`
image from `da591740/kernel_2712.img`. Bounded serial captured:
`runtime=0x201760`, `linked=0x81760`, `delta=0x180000`,
`str-ptr=0x8ee20`, `str-len=0x12`, a non-text sample at the linked
string pointer, and `talos: boot start\\n` at `str-ptr + delta`.

This accepts only the narrow contract that early Pi 5 static string pointers
currently carry linked addresses while the image executes at linked address
plus the runtime placement delta. It does not accept formatted arguments,
panic/exception reporting, or generic `core::fmt` on Pi 5. Normal early
console code should derive the runtime delta from code labels at boot time, or
otherwise use PC-relative access; it must not bake in the observed
`0x180000` delta as a board constant.

The current chosen early-console strategy is deliberately constrained rather
than a permanent relocation model. Pi 5 keeps the firmware-preserved UART10
backend and wraps early console writes in a small relocation-aware writer. That
writer computes the runtime placement delta from code labels, only applies it
to slices whose linked addresses fall inside the linker-provided rodata bounds,
and leaves non-rodata slices unchanged. This matches the current evidence:
Daedalus' Pi 4 path likely worked because firmware loaded at the linker
assumption, while the Talos Pi 5 network/lab path currently executes at linked
address plus `0x180000`.

Hardware run `rpi5-println-relocated-static-20260521T1230Z` accepted the
first layer of that strategy. The 71,088-byte
`b67c7f223d4eebf5013083e0cb533088a396bf00f229d038c835246c480c6526`
image was served twice as `da591740/kernel_2712.img`. Bounded serial captured
exact `TALOS: phase 0` through `TALOS: phase 4`, then the relocated static
println lines `talos: boot start`, `talos: board raspberry-pi-5-bcm2712`,
`talos: console early-uart fmt`, and `TALOS: println phase`. It did not
capture `TALOS: println count`, so formatted arguments are still a separate
layer.

Hardware run `rpi5-println-relocated-format-20260521T1242Z` kept the same
rodata-bounded relocation guard and retried the normal formatted
`println!("TALOS: println count {}", count)` path. The 75,184-byte
`ad8a24dab7454584fb124f3ffe63bd82817f97057f6dd783f062e4a449ead805`
image produced the relocated static println lines again, including
`TALOS: println phase`, but did not capture `TALOS: println count`. That
keeps generic `core::fmt` dynamic arguments unaccepted on Pi 5 and points the
next iteration at a Talos-owned early numeric formatting layer behind the
early console, rather than more generic formatter probing.

Hardware run `rpi5-println-early-dec-20260521T1405Z` accepted that
constrained numeric layer in the diagnostic path. The 75,352-byte
`a476880eab58f5600d5762c7a82b2d8553faedb5ad3f1225140005e5604acb2a`
image kept normal static `println!` for `TALOS: println phase`, then emitted
`TALOS: println count` through Talos' `early_format` decimal helper. Bounded
serial captured 13 phase/count pairs, including counts 0 through 12. A
follow-up normal-image run with boot-info/services rewritten to use the same
early helpers was not accepted: the 75,768-byte
`3175b86941835ab50425c90d02fb987c79688930892cff28da87675823974d0e`
image was served twice, but no Talos kernel serial lines were captured. Normal
boot-log acceptance therefore still needs a narrow normal-vs-diagnostic
localization step.

Normal-image localization now uses the same formatter-free UART10 phase writer
in the non-diagnostic path. The normal image emits:

- `TALOS: rust_entry`
- `TALOS: boot info parsed`
- `TALOS: target init`
- `TALOS: exceptions ready`
- `TALOS: kernel_main`

Hardware run `rpi5-normal-phase-lines-rerun-20260521T142534Z` served the
79,864-byte `088714ec8104ad363fff6cafd375278b476d10882ae701a54a6d5ab26f995089`
image from `da591740/kernel_2712.img`. Bounded serial captured BL31/Starting
OS, all five normal-path `TALOS:` phase lines, the static boot lines
`talos: boot start`, `talos: board raspberry-pi-5-bcm2712`, and
`talos: console early-uart fmt`, plus the status line
`talos: status early boot log ready`.

That accepts the normal-path reachability and static boot-log layer, but it
also exposed that `early_format::write_hex_*` still read a linked-address
rodata digit table. The DTB hex fields printed corrupt bytes after `0x`.
Changing hex formatting to compute digits without a rodata table passed local
gates, but first hardware run `rpi5-normal-readable-boot-20260521T142753Z`
served the 79,896-byte `5e817b5ba741c1054ad1ac79abd2443f3752177dc5152ae89f76f6aa967d52a7`
image and did not reach `Starting OS` or Talos serial in the captured window.
Treat the no-rodata hex fix as unaccepted until a rerun or narrower comparison
classifies that image behavior.

Follow-up hardware narrowed that open item but did not accept it. A rerun of
the 79,896-byte no-rodata hex image reached the same normal Talos phase and
static boot lines, but the DTB fields still emitted binary bytes after the
`0x` prefix. Two local fixes then passed the normal local gate suite: a
stack-backed ASCII digit helper and a Pi 5-only direct UART10 hex writer behind
`target::console::write_hex_*`. Their normal images were served by TFTP
(`17b824c9...` / 79,896 bytes and `de89763a...` / 79,792 bytes), but neither
run reached `Starting OS` or Talos serial in the captured hardware window.
Readable normal numeric fields therefore remain unaccepted. The next step is
local image/layout/codegen comparison against the accepted `088714ec...`
normal image before another hardware iteration.

That comparison found a concrete codegen hazard: the match-arm hex digit writer
compiled to an absolute jump table in the linked rodata range, which is unsafe
under the current Pi 5 runtime relocation. The fix now computes each hex digit
with wrapping arithmetic and a direct UART10 byte write, and
`scripts/rpi5-format-guard-check.sh` disassembles `write_early_hex_digit` to
reject jump tables, panic paths, and literal data. Hardware run
`rpi5-normal-readable-boot-arithhex-20260521T1520Z` served the 75,128-byte
`3af8078e640c7da010cc88851edb29899052c7ca590f224beb164847e5d78741`
normal image twice from `da591740/kernel_2712.img`. Bounded serial captured
the normal static boot lines plus readable boot-info and services fields:
`dtb=0x2efec600`, `core=0`, `el=2`, `mmio_regions=7`, and the final
`talos: status early boot log ready` line. This accepts the normal-image
static boot log plus Talos-owned early numeric formatting layer; generic dynamic
`core::fmt` arguments and panic/exception reports remain separate validation
layers.

The next accepted formatting layer restores a narrow Daedalus-like call shape
for one early value. On Pi 5, `print!`/`println!` macro invocations with one
literal `{}` placeholder route to `target::console::_print_one` instead of the
generic dynamic `core::fmt` path. `_print_one` relocates the format literal
through the same rodata-bounded early wrapper, writes the prefix and suffix
through the accepted early console, and writes the argument through
`early_format` decimal helpers or a relocated string write. This keeps
`println!("TALOS: println count {}", count)` readable while format specifiers,
multiple arguments, and arbitrary `Display` implementations remain unaccepted.

Hardware run `rpi5-one-placeholder-println-20260521T2049Z` served the
100,536-byte `1d0fc95bbcaf13b1c163083158d06fd69a546f1042c676dd2133e800ad2323fe`
diagnostic image twice from `da591740/kernel_2712.img`. Bounded serial captured
41 `TALOS: println phase` / `TALOS: println count N` pairs, with counts 90
through 130 in the accepted observation window. Static inspection of the same
diagnostic showed the count line calls
`talos::target::console::_print_one::<usize>`, not the generic formatter.
This accepts the constrained one-placeholder `println!` surface on Pi 5 while
keeping generic dynamic `core::fmt` as a separate future validation task.

The first normal-image adoption keeps that constrained path narrow by printing
one boot-info value through the same macro surface:
`println!("talos: boot core {}", boot_info.primary_core as usize)`. Hardware
run `rpi5-normal-one-placeholder-bootcore-rerun-20260521T211700Z` served the
100,912-byte `7e7ef44f87049550290d8a149262cf2b0d89a708b6b961f0ed0f87da21e2ff36`
normal image from `da591740/kernel_2712.img` and captured the normal phase
lines, `talos: boot core 0`, readable boot-info/services fields, and
`talos: status early boot log ready`. This accepts one-placeholder
`println!` for a normal Pi 5 boot/status value; multiple arguments, format
specifiers, arbitrary `Display`, and generic dynamic `core::fmt` remain
unaccepted.

The next accepted layer extends that one-placeholder surface to one early hex
argument without accepting `{:x}` or arbitrary `Display`. Pi 5 exposes a
small `target::console::hex(value)` wrapper whose `EarlyFormatArg`
implementation writes through the already accepted arithmetic early hex writer.
Normal boot now emits
`println!("talos: boot dtb {}", target::console::hex(boot_info.dtb_pa))`.
Hardware run `rpi5-normal-one-placeholder-hex-20260521T2136Z` served the
100,936-byte `6696a949854ae8444c4adddc0391df7d06b62749fb49a49de5eaf82ef1ffb32a`
normal image twice from `da591740/kernel_2712.img`, and the bounded serial
observe captured `talos: boot dtb 0x2efec600` with the normal phase lines,
readable boot-info/services fields, and status line. This accepts one early hex
address through the constrained one-placeholder `println!` path while leaving
format specifiers, multiple arguments, arbitrary `Display`, and generic dynamic
`core::fmt` unaccepted.

The next normal-image layer accepts one static string argument through the same
constrained path. Normal boot now emits
`println!("talos: boot target {}", boot_info.target.name())`. Hardware run
`rpi5-normal-one-placeholder-string-20260521T2148Z` served the 100,952-byte
`230f6f566324aedd8f5eaba73740d9cb9d60d85052c567b811e030b5e3bebe2d`
normal image twice from `da591740/kernel_2712.img`, and the bounded serial
observe captured `talos: boot target talos-rpi5-bcm2712` with the normal phase
lines, boot core, boot DTB, readable boot-info/services fields, and status
line. This accepts one static string argument through the constrained
one-placeholder `println!` path while preserving the current limits on multiple
arguments, format specifiers, arbitrary `Display`, and generic dynamic
`core::fmt`.

## Pointer-Correct Pi 5 Formatting

Matthew pushed back on treating formatting as a serial-console problem. The
accepted fix is to make the Pi 5 link/runtime pointer contract coherent before
asking `core::fmt` to work. The normal Pi 5 image now links `_start` at
`0x00200000`, matching the firmware-selected arm64 Image runtime placement,
while keeping the Image header `text_offset=0`.

With that contract in place, the public `print!` and `println!` macros have
returned to the ordinary `format_args!` path instead of the Pi-only
one-placeholder `_print_one` bypass. Normal boot prints a runtime pointer
delta line through generic formatting:

- `talos: boot core 0`
- `talos: boot dtb 0x2efec600`
- `talos: boot target talos-rpi5-bcm2712`
- `talos: pointer delta 0x0`

Hardware run `rpi5-pointer-contract-standard-fmt-20260521T215626Z` staged the
79,256-byte `06afe837ab8627cce4365c54fd443937b127f23d7cd9ab0f87bbbb9644ec6894`
normal image under root and `da591740/` kernel paths. TFTP logs captured 13
fresh events, including two 79,256-byte `da591740/kernel_2712.img` serves.
Bounded serial observe captured the normal phase lines, static boot lines, the
generic-formatted decimal/hex/string lines above, readable boot-info/services
fields, and `talos: status early boot log ready`.

This supersedes the constrained one-placeholder formatter as the normal Pi 5
logging path. The normal console no longer relocates static strings before
writing them; it relies on the accepted link/runtime contract instead. The
remaining linked-address adjustment helper is limited to vector installation and
explicit address-contract diagnostics, where the accepted normal run observed
`pointer delta 0x0`.

A follow-up normal boot/status cleanup moved the remaining Pi 5 boot-info and
services status lines from formatter-free helper composition onto ordinary
`println!` calls. Local disassembly shows the boot-info line constructed with
`core::fmt::Arguments::new::<55, 4>` and the services line with
`new::<70, 5>` when the DTB address is present, proving the normal path now
uses multi-field `format_args!` for these lines. Hardware run
`rpi5-normal-multifield-println-20260521T2232Z` served the 79,320-byte
`bdfa8d8986e3902b4e09c3de4ca0f282a5e2eaa950d835d1cd36ffd16fbd0bb8` kernel and
captured the expected normal boot-info/services lines plus the status line.

The normal boot banner now also reports the package version through the same
standard print surface. Hardware run
`rpi5-normal-version-line-clean-20260522T111827Z` used a clean non-diagnostic
Pi 5 config and served the 84,400-byte
`af1c11309ca5efa68601a3e563f75513e5adeae244d6f4d13b28a67bc7af1bae`
kernel twice from `da591740/kernel_2712.img`. Serial captured:

```text
talos: boot start
talos: board raspberry-pi-5-bcm2712
talos: version 0.1.0
talos: console early-uart fmt
talos: boot core 0
talos: boot dtb 0x2efec600
talos: boot target talos-rpi5-bcm2712
talos: pointer delta 0x0
talos: boot info: dtb=0x2efec600 core=0 el=2 target=talos-rpi5-bcm2712
talos: services: uart=firmware-preserved timer=arm-generic irq=gic-v2 mmio_regions=7 dtb=0x2efec600
talos: status early boot log ready
```

This closes the Phase 2.1 version-string portion of the normal Pi 5 serial
boot log while preserving the accepted `print!`/`println!` path and the
firmware-preserved UART10 backend.

The first Phase 2.2 DTB handoff observation now reads the firmware-provided
FDT header through the normal Pi 5 boot path. Talos treats the arm64 boot
`x0` value as a physical FDT pointer, reads only the 40-byte header with
volatile big-endian word loads, validates magic `0xd00dfeed`, and reports the
header bounds through ordinary `println!`.

Hardware run `rpi5-dtb-header-normal-20260522T113146Z` staged a clean
non-diagnostic archive `12e8134e9bc5d37cadc76ee95855d77f0f949911f54dcecff8b4610221c5df03`
with an 88,896-byte
`8bb1acc7c91d48f52e0bd13da387344b83d2742714aae4121d142122e74ff078`
kernel. TFTP served `da591740/kernel_2712.img` at 88,896 bytes, and serial
captured:

```text
talos: dtb header: magic=0xd00dfeed size=80254 version=17 last_comp=16 struct=72496 strings=7702
```

This accepts the narrow claim that the firmware-provided DTB pointer is
identity-accessible during early normal Pi 5 boot and that Talos can read the
FDT header before a full parser exists. Memory reservations, usable-memory
extraction, and fuller node/property modeling remain separate Phase 2.2/3.1
work.

The second Phase 2.2 DTB observation adds bounded structure-block walking for
only the root `/chosen` node and its `bootargs` property. This is still not a
general-purpose device tree parser: Talos walks tokens directly, resolves the
property name through the strings block, and returns the FDT-backed string only
for this one property.

Hardware run `rpi5-chosen-bootargs-chunked-20260522T115821Z` staged archive
`ddbb92b5003871a6dbc08c80c52c868825425ea14ac3dc36d23a2f52178c2472` with a
119,800-byte
`42c6dc6be26fd6e40aae36e4dd36a07a9e14342cf151c449f5d2ae560c52356c`
kernel. TFTP served `da591740/kernel_2712.img` at 119,800 bytes, and serial
captured the normal boot identity/status context plus:

```text
talos: dtb chosen bootargs: reboot=w coherent_pool=1M 8250.nr_uarts=1 pci=pcie_bus_safe cgroup_disable=memory numa_policy=interleave nvme.max_host_mem_size_mb=32 bcm2708_fb.fbwidth=640 bcm2708_fb.fbheight=480 bcm2708_fb.fbdepth=16 bcm2708_fb.fbswap=1 numa=fake=8 system_heap.max_order=0 iommu_dma_numa_policy=interleave smsc95xx.macaddr=88:A2:9E:AE:C8:7F vc_mem.mem_base=0x3fc00000 vc_mem.mem_size=0x40000000  console=ttyAMA10,115200 earlycon=pl011,mmio32,0x1f00030000 talos.boot=first-light
```

The first bootargs hardware run found the same property but corrupted the long
serial line after the `8250.nr_uarts` prefix. A known accepted DTB-header
control booted cleanly, so the follow-up kept the FDT parser unchanged and
emitted the long FDT-backed string in small chunks with explicit UART drains.
That chunked output is the accepted normal-path evidence for `/chosen`
bootargs.

The third Phase 2.2 DTB observation reads the FDT memory reservation block. The
reader scans big-endian `(address, size)` pairs from `off_mem_rsvmap` with a
fixed scan bound, records up to the first four entries for reporting, and stops
at the standard zero/zero terminator. It does not parse `/memory`,
`reserved-memory`, or derive usable RAM yet.

Hardware run `rpi5-dtb-reservations-after-status-20260522T1227Z` staged archive
`2f8add11eff46fae8f04dcc01e52e53af44b9ba1fbfc0f41ac9d2e24c7cd7773` with a
124,552-byte
`07dd0fdd8650b2f4b2433d7752b4899c2ea20fed5b17d54758efac1347a3414e`
kernel. TFTP served `da591740/kernel_2712.img` at 124,552 bytes, and serial
captured the normal boot identity, DTB header, full `/chosen` bootargs, status
line, and:

```text
talos: dtb reserved: count=0 shown=0 truncated=false
```

That accepts the narrow observation that the Pi 5 firmware DTB reservation block
is present, readable, and empty for this boot tree. It does not imply that no
other memory must be reserved; GPU/firmware carveouts and Linux-style reserved
memory still need later node/property parsing before Talos can own the physical
memory map.

The fourth Phase 2.2 DTB observation reads the root `/memory` node's `reg`
property using the root `#address-cells` and `#size-cells` values. The reader
walks the FDT structure block directly, records up to four banks for reporting,
and returns when the first root-level memory node closes. It is still a bounded
observation helper, not the final usable-RAM ownership policy.

Hardware run `rpi5-dtb-memory-reg-reservation-boundary-20260522T1354Z` staged
archive `b08bd703758335cf6b4c044e03321ea967b5116d43fbdfc1c8bdbbbfb45ba023`
with a 133,264-byte
`ff2ea673ef39b87676578c1994ecd8c139826e86bb7ff7a58b872dcc429cba37`
kernel. The first power cycle captured fresh firmware serial but no TFTP
kernel request. A second long-window pickup served `kernel_2712.img` twice at
133,264 bytes and captured the accepted normal boot identity/status context,
reservation boundary markers, memory scan markers, and:

```text
talos: dtb memory: address_cells=2 size_cells=2 count=3 shown=3 truncated=false
talos: dtb memory[0]: addr=0x0 size=0x3fc00000
talos: dtb memory[1]: addr=0x40000000 size=0xc0000000
talos: dtb memory[2]: addr=0x100000000 size=0x100000000
```

This accepts the narrow observation that the firmware-provided Pi 5 DTB exposes
three memory banks through `/memory/reg`: low memory below the firmware/GPU
carveout, the remaining first 4 GiB window above `0x40000000`, and a 4 GiB bank
above `0x100000000`. Talos still must combine this with reserved-memory nodes,
firmware carveouts, kernel image placement, stacks, page tables, and allocator
policy before claiming a usable physical memory map.

The first Phase 3.1 memory-map bridge keeps that cautious boundary but now
derives one conservative low-memory candidate for future allocator work. The
normal Pi 5 boot path reports the kernel/linker-owned range, heap and boot stack
bounds, the DTB blob range from the accepted FDT header/pointer, and one
page-aligned `low-tail` candidate from the low `/memory` bank after excluding the
kernel/runtime range, DTB blob, and any reported FDT reservation entries that
intersect the bank. This is still observation output, not allocator ownership.

Hardware run `rpi5-usable-control-rerun-20260522T1432Z` first published a
fresh-entry reset control and captured `TALOS: usable ctrl reset 1432Z`, proving
the serial path was reporting current output. The follow-up normal boot
republished archive
`99d1787faac8e5f6d3dbb8908796259350a7275bebf25e701222fe6c64063532` as tree
`48c791afbe3cc67583ae0ac0cae538e5544d445e18e6010bd4fd1f1796001e86`; TFTP
served the 137,896-byte
`c08b61be87e29cae17431164bda93dcb14ba6f579d4a92d1c02259aabac0b4dd`
`kernel_2712.img` twice and serial captured:

```text
talos: memory layout: kernel=0x200000..0x362000 heap=0x221ac0..0x321ac0 stack=0x321ac0..0x361ac0
talos: memory layout: dtb=0x2efec600..0x2effff7e size=0x1397e
talos: memory usable: bank=0 start=0x2f000000 end=0x3fc00000 size=0x10c00000 align=0x1000 policy=low-tail
talos: dtb memory[0]: addr=0x0 size=0x3fc00000
talos: dtb memory[1]: addr=0x40000000 size=0xc0000000
talos: dtb memory[2]: addr=0x100000000 size=0x100000000
```

The candidate starts above both the loaded Talos image/early heap/stack range
and the firmware-provided DTB blob, then aligns up to 4 KiB and ends at the low
bank boundary `0x3fc00000`. Later memory work still needs a real reservation
model for `reserved-memory`, page tables, device/MMIO exclusions, and ownership
handoff before memory can be allocated.

The next Phase 3.1 observation reads first-level `/reserved-memory` child nodes
from the firmware DTB. The helper stays bounded and allocation-free: it tracks
the `/reserved-memory` parent address/size cell counts, counts direct child
nodes, parses child `reg` ranges into up to four reported entries, and records
whether each range is tagged with `no-map` or `reusable`. It does not yet merge
those ranges into the usable-RAM candidate or implement allocator ownership.

Hardware run `rpi5-reserved-memory-observation-20260522T1502Z` published tree
`a42402a01e8ccfb301573132ce2d58634fcf2716fdf61719db4e4977ba536e03` with the
146,568-byte kernel
`c3f169e2fb64dd11b941e4f85a3ce535f3f2aa810ad254e6e34cca5e02cbd7dd`. TFTP
served the kernel twice and serial preserved the normal boot identity/status,
DTB header and bootargs, FDT reserve-map line, reserved-memory scan lines,
`/memory` banks, and conservative low-tail candidate. The accepted
reserved-memory observation was:

```text
TALOS: reserved-memory start
TALOS: reserved-memory done
talos: reserved-memory: addr_cells=2 size_cells=2 nodes=4 ranges=3 shown=3 truncated=false
talos: reserved-memory[0]: addr=0x0 size=0x80000 no_map=true reusable=false
talos: reserved-memory[1]: addr=0x3fd23160 size=0x3d no_map=true reusable=false
talos: reserved-memory[2]: addr=0x0 size=0x0 no_map=true reusable=false
```

This proves Talos can observe the firmware DTB's reserved-memory node data on
the normal Pi 5 boot path. The result is still input to allocator design, not a
complete physical memory map: the next allocator step must decide how to filter
zero-sized ranges, combine these node ranges with FDT reservation-map entries,
reserve page tables/MMIO, and only then hand memory to an allocator.

The next Phase 3.1 step folds the bounded `/reserved-memory` observations into
the conservative low-tail candidate. The helper now excludes the kernel/runtime
range, DTB blob, FDT reservation-map entries, and each reported nonzero
`/reserved-memory` range that intersects the selected low memory bank.
Zero-sized node ranges are ignored by the same empty-range rule used for other
reserved intervals. This is still one early candidate, not allocator ownership.

Hardware run `rpi5-reserved-memory-usable-filter-20260522T1509Z` published tree
`1f7362afb9819c49c676b2036f754e00a2a2b08b2a8198698ddea16d424ab5c4` with the
146,616-byte kernel
`3d844c5b6a7aa433292f1b0cfbaa3d37d380014d77fd4f4a8bdb0b83c2a9c90a`. TFTP
served `kernel_2712.img` twice, and serial captured the normal boot context,
the accepted reserved-memory lines, `/memory` banks, and:

```text
talos: reserved-memory[0]: addr=0x0 size=0x80000 no_map=true reusable=false
talos: reserved-memory[1]: addr=0x3fd23160 size=0x3d no_map=true reusable=false
talos: reserved-memory[2]: addr=0x0 size=0x0 no_map=true reusable=false
talos: memory usable: bank=0 start=0x2f000000 end=0x3fc00000 size=0x10c00000 align=0x1000 policy=low-tail
```

The candidate remains `0x2f000000..0x3fc00000` on this boot because
`0x3fd23160..0x3fd2319d` is above bank 0's `0x3fc00000` end, while the other
reported reserved-memory entries are below the current DTB-derived start or
zero-sized. Unit coverage proves that an intersecting nonzero range does move
the candidate start upward and that a zero-sized range does not.

The next allocator-prep step derives a page-frame seed span from that accepted
low-tail candidate. It does not allocate memory or create a free list; it only
proves the candidate can be rounded to 4 KiB frames and reported on the normal
boot path. Hardware rerun `rpi5-page-frame-seed-rerun-20260522T1534Z` served
the 150,816-byte kernel
`c49a2a48057d9fd098f250e121be3b3f18bf9b2e7bf7d3db643435936b5653b1` from
`da591740/kernel_2712.img` and captured:

```text
talos: memory usable: bank=0 start=0x2f000000 end=0x3fc00000 size=0x10c00000 align=0x1000 policy=low-tail
talos: page frames seed: start=0x2f000000 end=0x3fc00000 pages=0x10c00 page_size=0x1000 source=memory-usable
```

The page count is hexadecimal. `0x10c00` pages at 4 KiB corresponds to the
accepted `0x10c00000` byte low-tail span.

The normal Pi 5 BRK fatal path also uses this standard surface now. The
`rpi5-normal-exception-println-halt-pickup-20260521T2256Z` hardware pickup
served the 79,536-byte
`9900e29a441cb2b96246a237a2df0b084f7aa8e8630b8ea61a01c707d9bfec29`
diagnostic image and captured `TALOS: before normal BRK vbar=0x200800 el=2`,
`talos exception: current-spx-sync`, and the multi-field lower-hex
`exception-info` line emitted by ordinary `println!` calls.

The bounded Pi 5 panic fatal line uses the same standard print surface for its
runtime fields. Hardware run
`rpi5-panic-println-bounded-diag-20260521T2324Z` served the 75,256-byte
`0ca770312cf1490df781bfe7f0eff3b36a7830a6dc5afef97a1a02a4ae854525`
panic diagnostic image and captured normal boot/status output followed by
`TALOS: panic handler entered` and
`talos panic: el=2 vbar=0x200800`.

A later full `PanicInfo` diagnostic used ordinary
`println!("talos panic: {}", info)` in the panic handler. The restaged
`rpi5-full-panic-info-20260522T024739Z` archive served the 79,576-byte
`90ca0a62a7855b5ecbc81dccd434f01a2e807ec4a79441bcb335625b763cc53b`
kernel from `da591740/kernel_2712.img`; serial captured normal boot/status
output, the static panic-handler marker, and:

```text
talos panic: panicked at src/main.rs:433:9:
talos diagnostic panic
```

The accepted full `PanicInfo` output is now the default Pi 5 panic report,
not a separate output policy. Hardware run
`rpi5-default-panic-info-20260522T031900Z` used the normal
`rpi5-panic-report` diagnostic image without
`TALOS_RPI5_FULL_PANIC_INFO_DIAGNOSTIC`; TFTP served the 79,576-byte
`90ca0a62a7855b5ecbc81dccd434f01a2e807ec4a79441bcb335625b763cc53b`
kernel and serial captured the same normal boot/status output, static panic
marker, source location, and panic message.

The post-allocator normal boot path now also has a standard `println!`
acceptance point. Hardware run
`rpi5-post-allocator-println-smoke-20260523T102902Z` served the 79,533-byte
`b79abda4bafd991a30346426cce4959501235e0d3ccffa2dda57f4af6ca6988d`
normal image. Serial captured the boot through MMU, instruction-cache,
data-cache, and global-allocator initialization, followed by the
`println!`-backed line:

```text
talos: string smoke: ptr=0x2f010000 len=5 cap=8 sum=0x203 next=0x2f010008 used=0x8 rem=0x10befff8 ex=true stable=true ok=true
```

This accepts one ordinary formatted `println!` report after allocator
initialization. It does not change the allocator policy or accept broad
alloc-backed runtime formatting beyond the explicit diagnostic boundary.

The cache-enabled normal boot path also has a pre-allocator standard
`println!` acceptance point. Hardware run
`rpi5-data-cache-enabled-println-20260523T111335Z` used a 79,757-byte normal
image with kernel sha256
`fc00cce302eafc6b776e1729197f4fd424aa4899576854fce8edaa011b61ad24`.
TFTP evidence showed `da591740/kernel_2712.img` served at 79,757 bytes, and
retained serial captured the formatter-backed line immediately after the
formatter-free start/done transition markers:

```text
TALOS: dcache enable start
TALOS: dcache enable done
talos: data cache enabled: el=0x2 sctlr=0x30c51835 kind=el2-stage1-dcache-enabled
talos: bootstrap allocator plan: start=0x2f010000 end=0x3fc00000 bytes=0x10bf0000 pages=0x10bf0 page_size=0x1000 kind=bump-no-free-low-tail
talos: bootstrap allocator init: start=0x2f010000 next=0x2f010000 end=0x3fc00000 policy=no-free
talos: string smoke: ptr=0x2f010000 len=5 cap=8 sum=0x203 next=0x2f010008 used=0x8 rem=0x10befff8 ex=true stable=true ok=true
```

This accepts one post-cache, pre-allocator status report on the Daedalus-like
`print!`/`println!` path. It does not change cache register programming,
allocator ownership, or the policy that the actual cache-enable transition
markers stay minimal and formatter-free.

The earlier instruction-cache-enabled status line intentionally remains on the
formatter-free console helper. A follow-up hardware attempt to move that
pre-data-cache line to `println!` built and published a 79,773-byte normal
image, and TFTP served `da591740/kernel_2712.img` at that size, but fresh
serial observation did not capture new Talos-origin output past the firmware/RP1
boot lines. That non-acceptance keeps the current `println!` boundary at
post-data-cache, pre-allocator reporting; the instruction-cache transition
still uses bounded static/hex writes until the cache/formatting boundary is
proved separately.

The normal DTB memory summary and entry reports now also use the standard
`println!` surface after the accepted page-frame-reservation accounting.
Hardware run `rpi5-dtb-memory-summary-println-20260523T1228Z` used a
79,709-byte normal image with kernel sha256
`10e73d9329fd81bfc68ac8853f9fa9f3dfc313ae3115f43ef55e6e2e954708df`.
TFTP evidence showed `da591740/kernel_2712.img` served repeatedly at 79,709
bytes, and serial evidence captured the accepted data-cache, allocator, String
smoke, and page-frames-remaining lines followed by the `println!`-backed DTB
memory summary and entries:

```text
talos: page frames remaining: start=0x2f010000 end=0x3fc00000 pages=0x10bf0 page_size=0x1000 source=bootstrap-reserve
talos: dtb memory: address_cells=2 size_cells=2 count=3 shown=3 truncated=false
talos: dtb memory[0]: addr=0x0 size=0x3fc00000
talos: dtb memory[1]: addr=0x40000000 size=0xc0000000
talos: dtb memory[2]: addr=0x100000000 size=0x100000000
```

This extends the normal post-data-cache `println!` surface without changing
DTB parsing, memory-bank accounting, page-frame reservation policy, or allocator
ownership.

The normal page-frame seed report is also accepted on the post-allocator
`println!` surface. Hardware run
`rpi5-page-frame-seed-callsite-recovery-rerun-20260523T144337Z` used the
80,541-byte serial-prefixed candidate kernel
`059d1d879c8d7d5c26bbb02cdfa7b65412c256410b0fa520e9c6549c2fc44824`.
After a 76,152-byte accepted-control run recovered fresh serial/Talos capture,
the candidate run TFTP-served `da591740/kernel_2712.img` twice and serial
captured the normal boot through data-cache enablement, allocator plan/init,
String smoke, the post-allocator memory-usable line, then the `println!`-backed
seed line before bootstrap reserve and page-frames-remaining:

```text
talos: memory usable: bank=0 start=0x2f000000 end=0x3fc00000 size=0x10c00000 align=0x1000 policy=low-tail
talos: page frames seed: start=0x2f000000 end=0x3fc00000 pages=0x10c00 page_size=0x1000 source=memory-usable phase=post-allocator
talos: bootstrap reserve: start=0x2f000000 end=0x2f010000 pages=0x10 page_size=0x1000 reason=bootstrap-page-tables
talos: page frames remaining: start=0x2f010000 end=0x3fc00000 pages=0x10bf0 page_size=0x1000 source=bootstrap-reserve
```

This keeps the formatter-free pre-data-cache seed diagnostic in place and adds a
second human-readable copy after the accepted formatter boundary. It does not
change low-memory selection, reservation layout, allocator ownership, cache/MMU
programming, or the earlier unaccepted pre-data-cache `println!` boundary.
