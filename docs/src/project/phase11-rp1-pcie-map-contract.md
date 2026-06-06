# Phase 11 RP1/PCIe Map Contract

This contract defines the first Talos RP1/PCIe mapping slice. It is a source-backed contract for one narrow read-only diagnostic; it does not claim a driver, GPIO ownership, interrupt routing, DMA/cache policy, Ethernet, networking, SSH, or generated-root progress.

## Inputs

- Raspberry Pi Linux `rpi-6.12.y` device-tree sources retained under `tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/`: `bcm2712.dtsi`, `bcm2712-rpi-5-b.dts`, `bcm2712-rpi.dtsi`, and `rp1.dtsi`.
- Existing Talos reference notes for Pi 5, RP1 UART0, and RP1 pin-control bring-up evidence.
- Lab-controller status from `GET /status` showing the current boot tree is restored and the boot config includes `enable_rp1_uart=1`.
- Talos' existing PL011 model, where the flag register is at offset `0x18`.

## Address Contract

Linux `bcm2712.dtsi` defines the Pi 5 `pcie2` non-prefetchable PCIe window:

~~~text
PCIe 00_00000000 -> CPU physical 0x1f_0000_0000
size             -> 0xffff_fffc bytes in the source range
~~~

Linux `bcm2712-rpi-5-b.dts` supplies the RP1 child range under `pcie2`:

~~~text
RP1 bus 0xc0_40000000 -> PCIe 00_00000000
size                  -> 0x0041_0000 bytes
~~~

For this first slice Talos treats the initial RP1 peripheral CPU mapping as:

~~~text
cpu_phys = 0x1f_0000_0000 + (rp1_bus - 0xc0_4000_0000)
~~~

Relevant translated addresses:

| RP1 object | RP1 bus address | CPU physical address | Width | Use |
| --- | ---: | ---: | ---: | --- |
| RP1 UART0 PL011 base | `0xc0_4003_0000` | `0x1f_0003_0000` | 32-bit MMIO | Parent block only |
| RP1 UART0 PL011 flag register | `0xc0_4003_0018` | `0x1f_0003_0018` | 32-bit read | First diagnostic target |
| RP1 GPIO bank0 control base | `0xc0_400d_0000` | `0x1f_000d_0000` | 32-bit MMIO | Documented only |
| RP1 pads bank0 base | `0xc0_400f_0000` | `0x1f_000f_0000` | 32-bit MMIO | Documented only |

## Firmware-State Assumptions

The first diagnostic depends on firmware/boot configuration leaving RP1 and UART0 mapped and clocked enough for a non-destructive flag-register read. The current lab boot config reports `enable_rp1_uart=1`, and historical Talos first-light work already used RP1 UART0 through the `pcie2` address path.

The next task must not configure GPIO14/GPIO15, change UART baud/programming, touch RP1 clocks or resets, enable interrupts, or allocate DMA. If the read faults or returns an implausible value, classify that as evidence about this firmware-state assumption rather than expanding the diagnostic in place.

## Diagnostic Contract

The next diagnostic-core task may implement only this first read:

~~~text
name: rp1-uart0-fr-read
address: 0x1f_0003_0018
width: 32-bit volatile little-endian load
source target: RP1 UART0 PL011 flag register
expected success class: mapped/read-value
~~~

The diagnostic should report:

- the source contract id `phase11-rp1-pcie-map-contract-v1`;
- the target name, address, width, and raw value;
- a classification: `mapped/read-value`, `bus-fault/trap`, `firmware-state-dependency`, or `staging/build-blocker`;
- enough serial text to tie the output to the candidate artifact in the later Pi 5 proof task.

The accepted local core may add build/static/archive-review evidence only. The serialized Pi 5 proof remains a separate task that must acquire `hardwareTestLock`, capture candidate identity, serial cursor, TFTP delta, and restore evidence.

## Pi 5 Proof Status

`phase11-rp1-register-read-pi5-proof-20260605` completed with a hardware
blocker, not a mapping acceptance. The lab published candidate tree
`a96f0d8dc17a4872cb52e94c37c85d5adc5312255d26f988dbd8b71e1b6118c9`; TFTP
served the selected 87,392-byte `da591740/kernel_2712.img` before restore in
two candidate runs. Neither run reached `rpi5-rp1-uart0-fr-read`,
`mapped/read-value`, or `PASS` serial output from a fresh cursor. A
known-good control on the restored accepted tree
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10` retained
`TALOS: kernel_main` and accepted command-loop output, so the proof boundary
is `blocked-pre-entry-or-handoff-after-candidate-fetch`.

Do not treat this as evidence that the RP1 UART0 flag register is mapped or
unmapped. The next source-level investigation or revised diagnostic shape must
be supervisor-planned before changing RP1 constants or broadening into GPIO,
interrupt, DMA/cache, networking, SSH, or storage work.

`phase11-rp1-diagnostic-entry-pi5-proof-20260605` reran the revised
pre-MMIO-marker candidate after the source-level handoff task. The candidate
archive SHA-256 was
`2640ab9ceabee343ee1426b7137e1597687517f56d3b61f58a7ac0e7ab4b6608`, and the
published boot tree was
`0b25c8e08b7cdbac0447ee80a962ed7ee0fa9d219eafc3f060cfcd902c035511`. After
mandatory inconclusive-run triage, the known-good control restored
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`, fetched
the 104,136-byte control kernel, reached `TALOS: kernel_main`, and retained
PASS output. The candidate rerun fetched the selected 87,480-byte
`da591740/kernel_2712.img` twice before restore, but serial output still did
not reach `TALOS: kernel_main`, `rpi5-rp1-uart0-fr-read: start`,
`rpi5-rp1-uart0-fr-read: pre-mmio-read`, `mapped/read-value`, or `PASS`.
The classification remains `blocked-pre-entry-or-handoff-after-candidate-fetch`;
the proof does not accept RP1 mapped or unmapped behavior.

`phase11-rp1-diagnostic-entry-closeout-20260605` reconciles the revised
source/handoff and Pi 5 proof evidence. It accepts only the source contract,
local diagnostic candidate, candidate publication/TFTP fetch, known-good
control, and restore evidence. It does not accept RP1 mapped/read-value,
unmapped, trap, firmware-state, GPIO, interrupt, DMA/cache, networking, SSH,
storage, or Milestone 11.2 behavior. The next source-level handoff change or
diagnostic revision requires supervisor planning.

phase11-rp1-diagnostic-entry-control-source-core-20260605 adds a local
entry-control candidate for the next serialized proof. The candidate emits
rpi5-rp1-entry-control: rust-entry-control,
rpi5-rp1-entry-control: no-rp1-mmio,
rpi5-rp1-entry-control: classification=entry-control-reached, and
rpi5-rp1-entry-control: PASS immediately after the normal Pi 5 rust_entry
early-phase line, then stops before BootInfo parsing, target::init, RP1
GPIO/pin flushes, boot reports, memory planning, and the RP1 UART0 FR read
path. This is only a source/local discriminator and does not accept handoff
reachability or any RP1 mapped/unmapped behavior until the queued Pi 5 proof
publishes the candidate and retains hardware serial/TFTP evidence.

phase11-rp1-diagnostic-entry-control-pi5-proof-20260605 completed with a
staging-or-capture blocker. The task published only the accepted entry-control
candidate archive
`target/talos-rpi5-rp1-entry-control-source-core.tar.gz`, staging tree
`ab88a3d8549837459c8cebf8cb22580b52b39665421b7eb6d6773ebce8c6f9c2` and a
51,808-byte `da591740/kernel_2712.img`. The first candidate run, known-good
control, and candidate rerun all reached visible Raspberry Pi firmware serial
output through `Boot mode: NETWORK`, but fresh TFTP deltas were empty in all
three runs. The proof therefore does not show candidate fetch, Rust entry,
entry-control PASS, RP1 mapped/read-value, or RP1 unmapped/trap behavior. The
boot tree was restored to
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10` before
completion.

phase11-rp1-diagnostic-entry-control-closeout-20260605 reconciles that
source/local discriminator and blocker proof. It accepts only the source
contract, the local pre-BootInfo entry-control candidate, recorded
publication/staging state, and restore state. It does not accept candidate
fetch, Rust entry or entry-control reachability, RP1 mapped/read-value,
unmapped, trap, firmware-state, GPIO, interrupt, DMA/cache, networking, SSH,
storage, generated-root, broader PCIe, or Milestone 11.2 behavior. The next
bounded Phase 11 slice requires supervisor planning.

phase11-rp1-entry-control-candidate-rerun-20260605 later reran the accepted
entry-control candidate under the repaired known-good readiness and stable TFTP
rules. The candidate archive
target/talos-rpi5-rp1-entry-control-source-core.tar.gz staged a 51,808-byte
da591740/kernel_2712.img, and stable pre-restore TFTP evidence observed two
candidate kernel fetches. Fresh serial still did not reach TALOS: kernel_main,
the entry-control markers, or PASS. The result classification is
candidate-fetch-observed-without-entry-control; it accepts candidate fetch only,
not Rust entry, entry-control reachability, RP1 mapped/unmapped behavior, or
firmware-state behavior.

phase11-rp1-entry-control-handoff-discriminator-core-20260606 replaces the next
source discriminator with a non-published, no-RP1-MMIO reset-side-effect
candidate. The rpi5_rp1_handoff_reset scenario calls PSCI SYSTEM_RESET
immediately from rust_entry, before BootInfo::from_aarch64_x0, target::init,
boot reports, memory planning, allocator setup, or the RP1 UART0 FR read path.
Static inspection of target/talos-rpi5-rp1-handoff-reset-discriminator-core.tar.gz
proves the 45,248-byte image keeps text_offset=0, header_image_size=45248,
flags=12, ARMd magic, and _start -> rust_entry -> smc #0 side-effect
provenance. The next Pi 5 discriminator may accept only candidate fetch and
pre-BootInfo handoff reachability if a repeated TFTP boot/fetch sequence proves
the reset side effect after one candidate power cycle; RP1 mapped/unmapped
behavior remains blocked.

phase11-rp1-entry-control-handoff-pi5-discriminator-20260606 then published
only that accepted archive. The published tree was
760e7e3c59c3d6d6da4f465c9f67fc53a445bfa18850c6a76f2a3972af680d2d with a
45,248-byte da591740/kernel_2712.img. From one fresh power cycle, same-cursor
stable pre-restore TFTP evidence retained four candidate kernel fetches across
two boot sequences at 05:51:46/05:51:47 and 05:52:04/05:52:05 UTC. That
accepts pre-BootInfo rust_entry handoff reachability by the PSCI reset side
effect. It does not accept TALOS: kernel_main serial visibility, RP1
mapped/read-value, unmapped/trap, firmware-state, GPIO, interrupts, DMA/cache,
storage, generated-root, networking, SSH, broader PCIe, or Milestone 11.2
behavior. The lab restored the pre-run tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 before
hardware-lock release.

phase11-rp1-entry-control-handoff-closeout-20260606 reconciles that source and
hardware evidence as pre-bootinfo-handoff-reachability-accepted. The accepted
boundary is limited to candidate fetch and the rust_entry-to-PSCI-reset side
effect before BootInfo parsing, target initialization, boot reports, memory
planning, allocator setup, or the RP1 UART0 FR read path. Candidate serial
visibility, entry-control UART marker visibility, RP1 mapped/read-value,
RP1 unmapped/trap, firmware-state behavior, GPIO, interrupts, DMA/cache,
storage, generated-root, networking, SSH, broader PCIe, and Milestone 11.2
remain unaccepted. The next bounded step requires supervisor planning for a
focused post-handoff observability or entry-control repair before returning to
the serial-reported RP1 UART0 flag-register diagnostic.

phase11-rp1-post-handoff-marker-reset-core-20260606 adds the no-hardware
rpi5_rp1_post_handoff_marker_reset candidate for that focused observability
repair. The candidate enters rust_entry, emits the normal TALOS: rust_entry line
and a unique rpi5-rp1-post-handoff-marker-reset marker/classification through
the current UART10 early-serial path, flushes with wait_uart10_empty_early_phase,
then calls PSCI SYSTEM_RESET. Static archive/disassembly evidence for
target/talos-rpi5-post-handoff-marker-reset-core.tar.gz shows a 51,736-byte
kernel_2712.img, text_offset=0, header_image_size=51736, flags=12, ARMd magic,
_start -> rust_entry -> marker writes -> smc #0, and no RP1 UART0 FR-read
symbol/string. This is source/static evidence only; post-handoff marker
visibility, reset side effect, staging/capture behavior, and restore remain for
the queued serialized Pi 5 discriminator. RP1 mapped/read-value,
unmapped/trap, firmware-state behavior, GPIO, interrupts, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe, and Milestone 11.2 remain
unaccepted.

phase11-rp1-post-handoff-marker-reset-pi5-discriminator-20260606 published
only that accepted marker/reset archive, but completed as
staging-capture-blocker. The candidate publication staged tree
37995c483190ddcfaef70c9cf5be04244f75c4fcd9cf25fdd90f941ccc48c4f2 with a
51,736-byte `kernel_2712.img`, and fresh serial reached Raspberry Pi
firmware/RP1 output without any `TALOS: rust_entry` or
`rpi5-rp1-post-handoff-marker-reset` marker text. Stable same-cursor TFTP
samples for the candidate, candidate rerun, and restored known-good control
did not retain candidate-tied fetch evidence in their bounded windows. A late
first-run TFTP replay is retained as capture-timing evidence only, not
candidate identity proof, because status had already returned to the restored
tree when it was queried. The closeout classification is
staging-capture-blocked: visible post-handoff serial observability, reset
side-effect evidence, marker-path hang/fault evidence, RP1 UART0 FR-read
readiness, RP1 mapped/unmapped behavior, GPIO, interrupts, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe, and Milestone 11.2 remain
unaccepted. The queued RP1 UART0 FR-read refresh is therefore not
mechanically unblocked.

phase11-rp1-post-handoff-marker-reset-capture-recheck-pi5-20260606 reran the
same accepted marker/reset archive under the repaired capture-invariant rule.
Preflight and final pre-restore identity matched selected tree
37995c483190ddcfaef70c9cf5be04244f75c4fcd9cf25fdd90f941ccc48c4f2,
effective kernel kernel_2712.img, and the 51,736-byte
da591740/kernel_2712.img. Stable same-cursor TFTP evidence from fresh cursor
4111814 retained 65 events and 10 served candidate kernel fetches before
restore. Fresh serial from cursor 4113931 retained 19,625 bytes over 90
seconds and showed repeated firmware NETWORK boot/fetch cycles, but did not
show `TALOS: kernel_main` or `rpi5-rp1-post-handoff-marker-reset`.
The capture-recheck closeout classification is
reset-side-effect-accepted-marker-visibility-blocked: candidate fetch and the
PSCI reset-loop side effect are accepted for the selected no-RP1-MMIO
marker/reset candidate only. Visible post-handoff serial observability,
RP1 UART0 FR-read readiness, RP1 mapped/read-value, RP1 unmapped/trap,
firmware-state behavior, GPIO, interrupts, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe, Milestone 11.2, and phase transition remain
unaccepted. The queued RP1 UART0 FR-read refresh is still not mechanically
unblocked because reset-side-effect-only evidence is not visible marker
observability.

phase11-rp1-rust-entry-uart10-marker-loop-core-20260606 adds the next
no-hardware discriminator candidate without changing RP1 source. The
rpi5_rust_entry_uart10_marker_loop scenario branches directly from rust_entry
to a repeated UART10 marker loop before BootInfo parsing, target::init, boot
reports, memory planning, allocator setup, scheduler work, PSCI SYSTEM_RESET,
or RP1 UART0 MMIO. Static archive/disassembly evidence for
target/talos-rpi5-rust-entry-uart10-marker-loop-core.tar.gz shows a
45,328-byte kernel_2712.img, arm64 Image fields text_offset=0,
header_image_size=45328, flags=12, and _start -> rust_entry ->
run_rust_entry_uart10_marker_loop. The candidate marker is TALOS: reu10-loop;
string review confirms the RP1 UART0 FR-read report strings are absent. This
accepts only the source/static marker-loop candidate and does not accept
visible marker serial output, RP1 UART0 FR-read readiness, RP1
mapped/unmapped behavior, GPIO, interrupts, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe, Milestone 11.2, or phase transition. The
queued RP1 UART0 FR-read refresh remains blocked until the marker-loop
hardware discriminator and closeout accept visible Rust-entry UART10 marker
observability.

phase11-staging-capture-log-stability-core-20260605 repairs the Pi 5
proof-rule boundary exposed by that blocker. Replay from the retained cursor
`4088847` later returned 13 TFTP events, including a restored known-good
104,136-byte `da591740/kernel_2712.img` fetch, so future hardware proofs must
not classify a single empty `/tftp/logs?cursor=<fresh>` response as no-fetch
evidence. A proof must capture a fresh TFTP cursor immediately before power
cycle, observe serial, then re-query the same TFTP cursor until `cursor_end`,
`log_size`, `truncated`, and parsed events are stable for the accepted
sample count or until a bounded timeout is recorded. Fetch-byte classification
must happen before restore; any zero-event result is meaningful only after this
stable-log rule. This changes evidence semantics only and does not accept
entry-control reachability or RP1 mapped/unmapped behavior.

phase11-staging-capture-known-good-pi5-proof-20260605 then applied that rule to
the restored accepted boot tree
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10` with
`effective_kernel=kernel_2712.img`. Two serialized known-good power cycles
captured fresh serial and TFTP cursors, stable pre-restore TFTP deltas, and
restore evidence, but both stable deltas had zero events and serial did not
reach `TALOS: kernel_main`, command-loop readiness, or PASS. The deployed lab
API also returned `404` for `GET /`, so this proof used the documented
`GET /status` boot identity endpoint. Classification remains
`staging-capture-still-blocked`; the repaired rule is not yet accepted for RP1
candidate reuse.

phase11-staging-capture-repair-closeout-20260605 accepts the stable TFTP
evidence semantics only. It does not accept the lab/staging path for RP1
candidate reruns because the known-good control still produced stable
zero-event TFTP deltas and no Talos serial readiness. The next bounded task
must be supervisor-planned around lab-controller/capture or staging-publication
discrimination before any RP1 diagnostic/source changes, candidate rerun,
Milestone 11.2, networking, SSH, GPIO, interrupts, DMA/cache, storage,
generated-root, or broader PCIe work.

phase11-lab-evidence-contract-repair-core-20260605 repairs the proof contract
for the next discriminator without touching hardware, boot publication, or RP1
runtime code. The deployed lab API's authoritative boot identity endpoint is
`GET /status`; `GET /` returning `404 unknown endpoint: GET /` is recorded as
endpoint-semantics evidence only. The next proof checklist must retain
`GET /status`, `GET /boot/files`, `GET /boot/snapshots`, fresh serial and TFTP
cursors, stable pre-restore TFTP evidence, and final pre-restore
status/boot-file samples when inconclusive. Classification now separates
`staging-publication-mismatch`, `tftp-capture-logging-blindness`,
`serial-only-firmware-reboot`, and `valid-known-good-talos-readiness` without
accepting candidate fetch, Rust entry, RP1 mapped/read-value, RP1 unmapped/trap,
or Milestone 11.2 behavior.

phase11-known-good-capture-staging-pi5-discriminator-20260605 ran one
serialized power cycle on the restored accepted boot tree
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10` with
`effective_kernel=kernel_2712.img`. Final stable pre-restore TFTP evidence
from fresh cursor `4094251` contained 13 events, including two served
104,136-byte `da591740/kernel_2712.img` fetches, so the discriminator observed
known-good capture/staging before restore. Serial from the fresh cursor reached
Raspberry Pi firmware/RP1 boot output but did not reach `TALOS: kernel_main`,
command-loop readiness, or PASS. The task is classified
`known-good-fetch-observed-without-talos-readiness`; it accepts no RP1
candidate fetch, Rust entry, RP1 mapped/read-value, RP1 unmapped/trap, GPIO,
interrupt, DMA/cache, networking, SSH, storage, broader PCIe, or Milestone 11.2
behavior.

phase11-staging-capture-discriminator-closeout-20260605 accepts the repaired
proof semantics and known-good capture/staging health only. It reconciles the
initial zero-event TFTP sample as capture-latency evidence superseded by the
final stable pre-restore replay. The remaining blocker is
boot-runtime-readiness-after-known-good-fetch: the restored known-good tree
fetched kernel_2712.img, but serial did not reach Talos runtime readiness.
RP1 candidate/source work, candidate reruns, mapped/unmapped claims, GPIO,
interrupts, DMA/cache, networking, SSH, storage, generated-root, broader PCIe,
Milestone 11.2, and phase transition remain blocked pending supervisor
planning.

phase11-known-good-runtime-readiness-contract-core-20260605 repairs that
blocker contract without hardware. The next known-good runtime proof must keep
the restored tree identity
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`,
`effective_kernel=kernel_2712.img`, and the stable pre-restore
104,136-byte `da591740/kernel_2712.img` fetch separate from runtime
readiness. Runtime readiness requires a bounded serial observation from the
fresh cursor with a 75-second timeout, 1000 ms settle, 65536-byte cap, and
the markers `TALOS: kernel_main` plus
`rpi5-production-timer-preemption: PASS` for the current restored control.
`scripts/rpi5-observe-runtime-readiness.sh` records that classification. This
does not accept RP1 candidate fetch, Rust entry, entry-control reachability,
mapped/read-value, unmapped/trap, GPIO, interrupt, DMA/cache, networking, SSH,
storage, generated-root, broader PCIe, Milestone 11.2, or phase transition.

phase11-known-good-runtime-readiness-pi5-discriminator-20260605 applied that
contract to one serialized known-good power cycle. Stable replay from retained
fresh TFTP cursor `4095602` returned 13 events on both checks, including two
served 104,136-byte `da591740/kernel_2712.img` fetches, but the bounded serial
window retained only 708 bytes of Raspberry Pi firmware/RP1 output and did not
reach `TALOS: kernel_main`, `talos>`, or
`rpi5-production-timer-preemption: PASS`. The closeout classification is
`known-good-fetch-accepted-runtime-readiness-blocked`; RP1 entry-control
candidate rerun remains blocked until supervisor-planned work accepts
valid known-good Talos runtime readiness or repairs the blocker.

phase11-known-good-runtime-lineage-and-cursor-repair-20260605 then fixed the
blank-cursor caveat for the reusable TFTP helpers and mapped the restored
known-good tree to the same 104,136-byte da591740/kernel_2712.img image.
phase11-known-good-runtime-direct-cursor-pi5-recheck-20260605 repeated the
known-good hardware proof with fresh serial cursor 4096040 and fresh
authoritative TFTP cursor 4096953. Stable pre-restore replay retained 13
events, including two served 104,136-byte da591740/kernel_2712.img fetches,
and pre-run, pre-restore, and post-restore status all retained the restored
tree hash a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
with effective_kernel=kernel_2712.img. The direct-cursor closeout
classification is known-good-direct-cursor-fetch-runtime-readiness-blocked:
fetch visibility and restore hygiene are accepted, but the serial readiness
window still did not reach TALOS: kernel_main, talos>, or
rpi5-production-timer-preemption: PASS. RP1 entry-control candidate rerun,
candidate fetch, Rust entry, entry-control reachability, mapped/read-value,
unmapped/trap, and firmware-state behavior remain blocked until a
supervisor-planned boot/runtime readiness repair or discriminator accepts valid
known-good Talos runtime readiness.

phase11-known-good-runtime-serial-window-contract-20260606 repairs the
serial-observation side of that blocker without running hardware. Static
inspection showed the previous helper used one settled `/serial/observe` call,
which could stop after the first 708-byte firmware/RP1 burst and before the
later network/TFTP/kernel window. `scripts/rpi5-observe-runtime-readiness.sh`
now loops until the requested deadline from the fresh serial cursor, advances
the observe cursor between calls, accumulates serial text, and records
`observe_contract=deadline-loop-accumulated-from-fresh-cursor`. The next
serialized known-good proof is ready only for this serial-window discriminator:
it must retain selected boot identity, stable pre-restore TFTP fetch evidence,
the deadline-looped serial JSON, pre-restore state, restore, and lock-release
evidence before it can accept or reject valid known-good Talos readiness. RP1
candidate rerun/source work remains blocked until that proof and closeout
accept valid known-good Talos readiness.

phase11-known-good-runtime-serial-window-pi5-discriminator-20260606 then
retained a deadline-looped fresh serial window and stable pre-restore TFTP
evidence for the restored known-good tree. TFTP replay showed two served
104,136-byte `da591740/kernel_2712.img` fetches. The fresh serial window
omitted `TALOS: kernel_main` but reached
`rpi5-production-timer-preemption: PASS`. The marker-boundary review
`phase11-known-good-runtime-marker-boundary-review-core-20260606` accepts that
as `valid-known-good-talos-readiness-by-downstream-marker` for the current
restored production-timer control: source order proves the PASS line is emitted
only after the Pi 5 path has entered `kernel_main` and completed the
production-timer proof predicates. This does not accept RP1 candidate fetch,
Rust entry, entry-control reachability, mapped/read-value, unmapped/trap,
firmware-state behavior, GPIO, interrupts, DMA/cache, networking, SSH, storage,
generated-root, broader PCIe, Milestone 11.2, or phase transition. The queued
marker-boundary closeout must reconcile this classification before any RP1
candidate rerun is promoted.

## Diagnostic Core Implementation

The local diagnostic core is compiled only when `TALOS_BOOT_SCENARIO=rpi5_rp1_uart0_fr_read` is selected. That path first reports `rpi5-rp1-uart0-fr-read: start` and `rpi5-rp1-uart0-fr-read: pre-mmio-read`, flushes UART10, then reads exactly `RP1_UART0_FR` (`0x1f_0003_0018`) with one 32-bit volatile load. A returned read reports the contract id, target name, address, width, raw value, `mapped/read-value` success classification, and PASS before returning to the existing final halt path. The pre-MMIO marker is a discriminator for the next serialized proof: if hardware reaches that marker but not the read-value line, the result is entry/handoff reachability plus an RP1 read trap/hang boundary, not a mapping acceptance.

The diagnostic does not add raw assembly early-entry UART markers. Prior Phase 10 evidence quarantined that path from prompt-capable Pi 5 controls after it made accepted controls fail, so this slice keeps the marker inside the existing Rust/serial path. It does not add GPIO, pin-control, clock, reset, interrupt, DMA/cache, Ethernet, networking, SSH, storage, generated-root, or shell behavior.

Artifact helpers:

- `scripts/rpi5-rp1-uart0-fr-read-image.sh` builds the candidate image.
- `scripts/rpi5-rp1-uart0-fr-read-boot-tree.sh` stages the candidate image into a Pi 5 boot tree for the later serialized hardware proof.

## Deferred Work

- PCIe enumeration and BAR discovery for general external devices.
- Talos-owned RP1 reset, clock, GPIO, pinctrl, UART initialization, and interrupt routing.
- DMA addressability, cache maintenance, IOMMU policy, Ethernet, networking, SSH, storage drivers, and writable persistent filesystems.
- Any fix for the Phase 10 Pi 5 generated-root firmware-initramfs overlap blocker.
