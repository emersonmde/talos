# AArch64 Exceptions

Talos installs a 2 KiB-aligned AArch64 exception vector table during early kernel
initialization. On Raspberry Pi 5, the vector base must be relocated from the
linked address to the runtime address before writing `VBAR_EL2`; the current
network-boot contract has hardware evidence for a `+0x180000` runtime delta, but
code must continue deriving that delta at runtime instead of hard-coding it.

The Pi 5 firmware currently hands Talos to EL2. Exception setup writes the vector
base for the current exception level, then the normal boot path emits
`TALOS: exceptions ready` after the `VBAR` write succeeds. A deliberate `BRK`
from the exception-report diagnostic has hardware-proven that `VBAR_EL2` points
at the relocated vector table: the serial log shows normal boot/status output,
`TALOS: before BRK vbar=0x200800 el=2`, and `TALOS: vector-entry`.

For the accepted early BRK report, the diagnostic vector path reads the matching
EL2 syndrome registers in assembly and writes a formatter-free UART10 report
before entering the Rust exception handler:

```text
TALOS: handler-entered vector=0x0000000000000004
TALOS: exception-info esr=0x00000000f2000000 elr=0x0000000000203a68 far=0x1a9bbff767d79fef
TALOS: rust-exception vector=0x4 esr=0xf2000000 elr=0x203b88 far=0x9abbbfffffd7dfef
```

This is accepted as a narrow Pi 5 hardware diagnostic for synchronous BRK state:
the vector table is active, the current-EL register bank is readable, and Talos
can produce a readable exception report without relying on `core::fmt` or rodata
relocation. The assembly report remains a guardrail before Rust entry, and the
`TALOS: rust-exception ...` line is emitted by a normal compiler-generated
Rust `rust_exception_handler` function after its stack-frame prologue using
formatter-free static and hex helpers. That proves the accepted vector path can
cross into Rust and consume the vector, ESR, ELR, and FAR arguments for a bounded
Rust-origin report.

The normal Rust exception handler now has its own accepted BRK report path
without the diagnostic assembly report enabled. The first accepted form was
formatter-free. After the Pi 5 pointer contract was fixed and normal
`println!` formatting was accepted, the normal BRK preamble and exception report
moved onto the same standard `println!` surface as the rest of early boot. The
accepted `rpi5-normal-exception-println-halt-pickup-20260521T2256Z` pickup
served the 79,536-byte
`9900e29a441cb2b96246a237a2df0b084f7aa8e8630b8ea61a01c707d9bfec29` image from
`da591740/kernel_2712.img` and captured:

```text
TALOS: before normal BRK vbar=0x200800 el=2
talos exception: current-spx-sync
exception-info: esr=0x00000000f2000000 elr=0x000000000020420c far=0x9abbb7ffffd7dfef
```

The report uses ordinary `println!` calls in the normal Rust
`rust_exception_handler` branch, including vector-name formatting and three
lower-hex exception fields. This accepts the normal synchronous BRK fatal report
shape through the Daedalus-like kernel print API for Pi 5 bring-up.

The normal synchronous BRK diagnostic now also has accepted exception-return
evidence. The vector shim calls `rust_exception_handler`, writes the returned
resume address back to the active `ELR_ELx` bank, and executes `ERET`. The
`rpi5-normal-brk-exception-return-rerun-20260522T022222Z` pickup served the
75,344-byte
`b94aa5b87268f5d46f4df0b6cab7383d3c3225550fa0024a5602529f6b4c3f2e` image
from `da591740/kernel_2712.img` and captured:

```text
TALOS: before normal BRK vbar=0x200800 el=2
talos exception: current-spx-sync
exception-info: esr=0x00000000f2000000 elr=0x0000000000203824 far=0x1a9bbff767d79fef
talos exception: resume elr=0x0000000000203828
TALOS: after normal BRK resume
```

This accepts a narrow same-EL synchronous BRK report-and-resume path on Pi 5.
It is not a general exception recovery policy: IRQ/FIQ/SError dispatch, lower-EL
exceptions, and nested exception behavior remain unaccepted.

The diagnostic return shim now preserves the interrupted general-purpose
register set while calling into Rust. The vector slot saves the interrupted
`x0`/`x1` and vector kind, branches to a return-capable entry shim, and that
shim saves/restores `x0..x30` around `rust_exception_handler` before `ERET`.
The accepted phase-enabled register-preservation pickup served the 83,984-byte
`e730a6d2903e40836b1784e5e40f08fb900f1517d4a50829b453e2851f2c1c70`
kernel from `da591740/kernel_2712.img` and captured:

```text
TALOS: before normal BRK vbar=0x200800 el=2
talos exception: current-spx-sync
exception-info: esr=0x00000000f2000000 elr=0x000000000020ff4c far=0x9abbbfffffd7dfef
talos exception: resume elr=0x000000000020ff50
TALOS: after normal BRK resume x9=0x1122334455667788 x19=0x8877665544332211
TALOS: exception registers preserved
```

This remains a diagnostic-only BRK return contract, but resumed code can now
rely on the tested shim not clobbering representative caller-saved and
callee-saved registers across the Rust handler call.

The same return-capable diagnostic now also reports a bounded slice of the
interrupted exception context. The shim reads the active `SPSR_ELx` bank and
passes the saved interrupted `x9` and `x19` values directly to Rust before
restoring the full `x0..x30` frame. The accepted direct-register context pickup
served the 84,080-byte
`44ea7e494c86e48a8bc60d8b8ea5bb98910ceebb1ff36ae941d432461406ef54`
kernel from `da591740/kernel_2712.img` and captured:

```text
TALOS: before normal BRK vbar=0x200800 el=2
talos exception: current-spx-sync
exception-info: esr=0x00000000f2000000 elr=0x0000000000210248 far=0xdb9bfff7f7cfdfcf
exception-status: spsr=0x200003c9
exception-regs: x9=0x1122334455667788 x19=0x8877665544332211
talos exception: resume elr=0x000000000021024c
TALOS: after normal BRK resume x9=0x1122334455667788 x19=0x8877665544332211
TALOS: exception registers preserved
```

This accepts `SPSR_ELx` plus representative saved GP register reporting for the
same deliberate BRK resume diagnostic. It still does not promote BRK resume to a
production exception-recovery policy.

The diagnostic report now prints the saved interrupted general-purpose register
frame in bounded groups before restoring that same frame for `ERET`. Hardware
run `rpi5-exception-full-gpr-20260522T063059Z` staged the 84,488-byte
`2ce08f296176c368fba6b8228db5bf597b543ee7751a2595fef05d8e715e1d94` kernel.
The accepted serial pickup captured:

```text
exception-status: spsr=0x200003c9
exception-regs0: x0=0x354930 x1=0x354938 x2=0x1b x3=0x2129f0
exception-regs1: x4=0x35407f x5=0x1 x6=0x0 x7=0x0
exception-regs2: x8=0x0 x9=0x1122334455667788 x10=0x1 x11=0x35403c
exception-regs3: x12=0x0 x13=0x0 x14=0x0 x15=0x0
exception-regs4: x16=0x0 x17=0x0 x18=0x0 x19=0x8877665544332211
exception-regs5: x20=0x0 x21=0x0 x22=0x0 x23=0x0
exception-regs6: x24=0x0 x25=0x0 x26=0x0 x27=0x0
exception-regs7: x28=0x0 x29=0x0 x30=0x204228
```

The following resume lines again showed the saved `x9` and `x19` sentinels
surviving the handler call and return. This closes the Phase 2.3 diagnostic
state-dump goal for deliberate same-EL BRK while leaving general exception
recovery, IRQ/FIQ/SError dispatch, and lower-EL handling for later milestones.

The default Pi 5 fatal exception path now uses the same saved-frame reporting
shape without attempting to resume. Non-diagnostic Pi 5 vector slots save the
interrupted `x0..x30` frame, read the active `SPSR_ELx` bank, and pass a
read-only frame view into the Rust fatal handler. The normal BRK diagnostic
uses that default non-returning path; hardware run
`rpi5-default-fatal-exception-frame-20260522T064710Z` staged the 84,248-byte
`e2a5f013ecffc8415e6c78d1596ac085a11cf75d186406307fd84df6b35b4258` kernel and
captured:

```text
talos exception: current-spx-sync
exception-info: esr=0x00000000f2000000 elr=0x0000000000204134 far=0x9abbbfffffd7dfef
exception-status: spsr=0x200003c9
exception-regs0: x0=0x0 x1=0x212898 x2=0x1b x3=0x212900
exception-regs1: x4=0x35400f x5=0x1 x6=0x0 x7=0x0
exception-regs2: x8=0x0 x9=0x2127a2 x10=0x1 x11=0x353fcc
exception-regs3: x12=0x0 x13=0x0 x14=0x0 x15=0x0
exception-regs4: x16=0x0 x17=0x0 x18=0x0 x19=0x2efec600
exception-regs5: x20=0x0 x21=0x0 x22=0x0 x23=0x0
exception-regs6: x24=0x0 x25=0x0 x26=0x0 x27=0x0
exception-regs7: x28=0x0 x29=0x0 x30=0x204134
```

This makes saved-register reporting the default Pi 5 fatal exception contract
while keeping the `ERET` resume logic confined to the explicit
exception-return diagnostic build.

Phase 4 promotes the same saved-register frame shape into the production
current-EL IRQ entry contract for both QEMU virt and Pi 5. In normal builds,
each vector slot saves interrupted `x0`/`x1` plus the vector number, then the
shared entry shim saves `x0..x30` in a 31-register `ExceptionFrame`. Current-EL
IRQ vectors dispatch to `rust_irq_handler(vector, elr, spsr, frame)`, restore
the full saved frame, and return with `ERET`. The Rust IRQ stub is intentionally
inert: it records a count plus the last vector, ELR, and SPSR using atomics and
does not allocate, print, acknowledge a GIC interrupt, or program a timer.

This contract is only a state-preserving IRQ entry/return foundation.
Interrupts remain masked by the normal boot path until the later GIC and generic
timer smoke tasks explicitly unmask them. FIQ, SError, nested IRQs, lower-EL IRQ
policy, interrupt acknowledgement/EOI, and preemption/context switching remain
unaccepted.

The default fatal report is now also accepted for a non-BRK same-EL synchronous
exception. The undefined-instruction diagnostic emits a fresh entry label, runs
through normal boot/status output, prints `TALOS: before undefined instruction`,
then executes `udf #0` and takes the same non-returning saved-frame path. Hardware
run `rpi5-long-window-undefined-20260522T091507Z` staged the 84,296-byte
`5a6c5b7703ab08b30f8e85771d319ff24b996b657de1b80356444ab3f8d97676` kernel and
captured:

```text
TALOS: before undefined instruction
talos exception: current-spx-sync
exception-info: esr=0x0000000002000000 elr=0x0000000000204158 far=0x9b9bfff7f7d7dfcf
exception-status: spsr=0x200003c9
exception-regs0: x0=0x0 x1=0x2128c8 x2=0x1b x3=0x212930
exception-regs1: x4=0x35403f x5=0x1 x6=0x0 x7=0x0
exception-regs2: x8=0x0 x9=0x107d001000 x10=0x98 x11=0x353ffc
exception-regs3: x12=0xdeaddead x13=0x0 x14=0x0 x15=0x0
exception-regs4: x16=0x0 x17=0x0 x18=0x0 x19=0x2efec600
exception-regs5: x20=0x0 x21=0x0 x22=0x0 x23=0x0
exception-regs6: x24=0x0 x25=0x0 x26=0x0 x27=0x0
exception-regs7: x28=0x0 x29=0x0 x30=0x204158
```

The ESR value `0x02000000` is the captured syndrome for this `udf #0` trap
and is distinct from the BRK syndrome `0xf2000000`, so this proves the default
fatal report is not BRK-specific. IRQ/FIQ/SError dispatch, lower-EL vectors,
data-abort policy, and exception recovery remain separate work.

Fatal Pi 5 reports now include a bounded ESR exception-class line before the
saved status and register dump. The class is decoded from `ESR_ELx[31:26]`
without using generic dynamic formatting. Hardware run
`rpi5-exception-class-undefined-20260522T0924Z` staged the 84,472-byte
`404eec08e93b2d43ab5b6903edbf6e237b41304cc09a9c6b5d85a61a57884ccb` kernel
and captured the existing undefined-instruction report with the added line:

```text
exception-info: esr=0x0000000002000000 elr=0x0000000000204394 far=0x9bbbbfffffd7dfef
exception-class: unknown-or-undefined-instruction ec=0x0
exception-status: spsr=0x200003c9
exception-regs7: x28=0x0 x29=0x0 x30=0x204394
```

The `ec=0x0` label reflects ARM's uncategorized/unknown EC value for this
`udf #0` trap, so the report preserves both the human diagnostic label and the
raw class needed for later policy.

The same fatal report path is also accepted for a deliberately triggered
same-EL data abort. The Pi 5 alignment diagnostic enables `SCTLR_ELx.A` at the
current exception level, performs an unaligned load from a valid stack-backed
object after vectors are installed, and lets the default non-returning handler
report the fault. A long-window fresh-entry control first recovered current
Talos-origin serial output, then hardware run
`rpi5-data-abort-long-window-rerun-20260522T102820Z` staged the 84,496-byte
`6e0573eac9dba36b6fa5573d6d2cc854363bb75105ed494bed723a59620813ce` kernel
from tree `7423149ada6d0cfe853fe38d923e16edc593784a806dc8686574ec7c0c204a1f`
and captured:

```text
TALOS: before alignment data abort ad0x354989 vbar=0x200800 el=2
talos exception: current-spx-sync
exception-info: esr=0x0000000096000021 elr=0x0000000000204480 far=0x0000000000354989
exception-class: data-abort-same-el ec=0x25
exception-status: spsr=0x600003c9
exception-regs0: x0=0x2 x1=0x2129b8 x2=0x1b x3=0x2129d8
exception-regs1: x4=0x212080 x5=0x8 x6=0x0 x7=0x0
exception-regs2: x8=0x354989 x9=0x107d001000 x10=0x98 x11=0x30
exception-regs3: x12=0xdeaddead x13=0x0 x14=0x0 x15=0x0
exception-regs4: x16=0x0 x17=0x0 x18=0x0 x19=0x2efec600
exception-regs5: x20=0x0 x21=0x0 x22=0x0 x23=0x0
exception-regs6: x24=0x0 x25=0x0 x26=0x0 x27=0x0
exception-regs7: x28=0x0 x29=0x0 x30=0x20447c
```

This proves the fatal saved-frame report covers a same-EL data abort with a
nonzero `FAR_ELx` under the current Pi 5 early-runtime configuration. It does
not define data-abort recovery, page-table fault policy, lower-EL abort
handling, IRQ/FIQ/SError dispatch, or production exception recovery.

The current-SP0 synchronous vector slot has also been hardware-proven through a
diagnostic-only BRK path. The diagnostic installs the normal exception vectors,
sets `SP_EL0` to an aligned scratch stack, selects `SPSel=0`, and executes
`brk #0`. Before accepting the result, a long-window fresh-entry control
captured the unique `TALOS: sp0 ctrl reset 1056Z` label plus `Starting OS`
and BL31, proving the lab was seeing current Talos-origin serial output.

`rpi5-sp0-long-window-rerun-20260522T1059Z` then staged the 84,440-byte
`d187c54675c5b3be65875d3c875ec55ebe968096e71feaa47dfce227d1edb72c` kernel
from tree `55d0fd256d6bf6401a5446f51a0e84588d162d5fb5c4a15f5520368ed1fc574d`
and captured:

```text
TALOS: before SP0 BRK sp0=0x354950 vbar=0x200800 el=2
talos exception: current-sp0-sync
exception-info: esr=0x00000000f2000000 elr=0x000000000020441c far=0xdab9bfffffd7dfef
exception-class: brk-aarch64 ec=0x3c
exception-status: spsr=0x200003c8
exception-regs0: x0=0x0 x1=0x212980 x2=0x1b x3=0x2129a0
exception-regs1: x4=0x212080 x5=0x8 x6=0x0 x7=0x0
exception-regs2: x8=0x354950 x9=0x107d001000 x10=0x98 x11=0x30
exception-regs3: x12=0x0 x13=0x0 x14=0x0 x15=0x0
exception-regs4: x16=0x0 x17=0x0 x18=0x0 x19=0x2efec600
exception-regs5: x20=0x0 x21=0x0 x22=0x0 x23=0x0
exception-regs6: x24=0x0 x25=0x0 x26=0x0 x27=0x0
exception-regs7: x28=0x0 x29=0x0 x30=0x20440c
```

This proves the Pi 5 vector table dispatches the current-SP0 synchronous slot
into the same default fatal saved-frame report path. It does not establish a
production SP0 runtime policy, SP0 recovery, lower-EL handling, or IRQ/FIQ/SError
dispatch.

The Pi 5 panic handler now has accepted output through the standard
`println!` surface. A bounded EL/VBAR diagnostic first proved that the panic
handler can enter after normal boot/status output and print:

```text
TALOS: panic handler entered
talos panic: el=2 vbar=0x200800
```

The entry marker stays on the tiny static serial helper, while the
`talos panic: ...` line uses ordinary `println!` with formatted EL and VBAR
fields. This proves the panic handler can produce a bounded human-readable
fatal line through the standard print surface after the Pi 5 link/runtime
pointer contract was fixed.

A follow-up full `PanicInfo` diagnostic then proved the same panic handler can
format `PanicInfo` through ordinary `println!("talos panic: {}", info)` after
normal Pi 5 boot/status output:

```text
TALOS: panic handler entered
talos panic: panicked at src/main.rs:433:9:
talos diagnostic panic
```

That diagnostic was promoted into the default Pi 5 panic report before the
proof-only panic image was retired. Hardware run
`rpi5-default-panic-info-20260522T031900Z` served the 79,576-byte
`90ca0a62a7855b5ecbc81dccd434f01a2e807ec4a79441bcb335625b763cc53b`
kernel from `da591740/kernel_2712.img` and captured normal boot/status
output, the static panic marker, the source location, and the diagnostic panic
message. The active source path remains the normal panic handler
`println!("talos panic: {}", info)`; the retired `rpi5-panic-report` wrapper
and full-panic cfg are historical evidence only.

Nested panic behavior has bounded Pi 5 historical evidence. The panic handler
uses a word-sized atomic panic-in-progress guard so simultaneous panics on
separate cores do not race through a shared `UnsafeCell<bool>`. On the first
panic, the handler records the
guard and prints the full `PanicInfo` line. On guarded re-entry, it avoids
formatting and emits only:

```text
TALOS: panic handler entered
TALOS: nested panic
```

The accepted `rpi5-nested-panic-volatile-20260522T050507Z` pickup captured
normal boot/status output, the nested-panic diagnostic prearm/trigger lines,
the panic-handler entry marker, and `TALOS: nested panic` from the 79,752-byte
`55290b303cce34390023cbd49c8bc5a843e3b757b352b9de1297ccc039a091d7` kernel.
This is still a halt-only early panic policy. Non-sync exception paths,
multi-core panic coordination, and richer panic/exception unification remain
later work.
