# Phase 10 Pi 5 Generated-Root Boot Transport Contract

Status: accepted as the contract for the Milestone 10.3 Pi 5 generated-root
boot-transport candidate.

This contract adds no runtime behavior, publishes no archive, runs no Pi 5
hardware, and acquires no hardware lock. It selects the boot-archive and
evidence shape for the next candidate task.

## Selected Path

The Pi 5 candidate transport uses the Raspberry Pi firmware initramfs envelope
to load the existing talos-generated-root-v1 artifact bytes outside the kernel
image:

initramfs initramfs_2712 followkernel

The candidate archive must include identical artifact files at both paths:

initramfs_2712
da591740/initramfs_2712

Both root and da591740/ config.txt files must contain the same initramfs line.
The proof task must record which path the firmware actually fetched after a
fresh TFTP cursor.

## Runtime Source Contract

QEMU's accepted generated-root transport uses a fixed loader-device address at
0x47000000. That address is not a Pi 5 hardware placement contract.

The Pi 5 candidate must derive the artifact range from firmware-provided FDT
/chosen initramfs bounds, conventionally linux,initrd-start and
linux,initrd-end. Serial evidence must report the range, length, digest, and
source before claiming that generated-root reads or execs used the external
artifact.

Invalid or missing firmware initramfs evidence falls back to the compiled
generated-root image. Malformed external artifact bytes must never partially
merge into the VFS.

## Candidate Archive Contract

The non-published candidate archive should be named:

target/talos-rpi5-generated-root-boot-transport-candidate-20260605.tar.gz

Required static evidence:

- source commit and clean/conflict-free status;
- archive SHA-256;
- kernel image SHA-256 and size;
- external generated-root artifact SHA-256 and size;
- root and serial-prefix file equality for kernel images, config, and artifact;
- boot tree listing;
- config proof for initramfs initramfs_2712 followkernel;
- proof strings for rootinfo, /generated/manifest.txt, /generated/status7, and
  the final classification/PASS marker.

The candidate task must not publish the archive or touch hardware.

## Candidate Implementation

`phase10-pi5-generated-root-boot-archive-candidate-core-20260605` accepted the
non-published candidate archive at the contract path. The candidate is built by
`scripts/rpi5-generated-root-boot-transport-boot-tree.sh` and reviewed by
`scripts/rpi5-generated-root-boot-transport-candidate-review.sh`.

The candidate kernel includes the `rpi5_generated_root_boot_transport` scenario.
At Pi 5 startup it reads `/chosen` `linux,initrd-start` and `linux,initrd-end`,
installs the selected artifact as generated-root source `firmware-initramfs`,
and falls back to the compiled generated-root image for missing or oversized
firmware bounds or malformed artifact bytes. The proof harness is limited to
`rootinfo`, `cat /generated/manifest.txt`, `exec /generated/status7 alpha`,
`waitpid`, and `laststatus`.

Accepted candidate identity:

- archive SHA-256:
  `8cb1d731e55f35d13328cf4f618c9dac2bf673311535ddd36038680d8a4ef60e`
- kernel SHA-256:
  `c44e5a55eb600a09a217c6ad23f665a43d1092a8e982423f5162099c34a42169`
- generated-root artifact SHA-256:
  `0341f5393502f54489acb1951633bf2773fb846a82bde89b3e4a2e82000724c6`
- retained evidence:
  `tasks/evidence/2026-06-05-phase10-pi5-generated-root-boot-archive-candidate-core/`

## Hardware Proof Contract

The later hardware proof must own hardwareTestLock, publication, power-cycle,
TFTP delta capture, serial capture, and restore. Acceptance requires:

- candidate identity;
- fresh serial cursor;
- fresh TFTP cursor and delta showing candidate kernel plus generated-root
  artifact fetch;
- serial proof that Talos consumed the firmware-loaded artifact and observed the
  generated file content and generated executable status;
- prompt/readiness and PASS/classification markers;
- post-run restore evidence.

Any inconclusive run must follow the standard triage sequence before code
changes: candidate identity, fresh serial cursor, TFTP delta, known-good
control, then candidate rerun.

## Hardware Blocker

phase10-pi5-generated-root-boot-transport-proof-20260605 completed the first
serialized Pi 5 candidate run with a source-backed blocker, not acceptance. The
Pi fetched the selected candidate files, including da591740/kernel_2712.img at
204888 bytes and da591740/initramfs_2712 at 662 bytes, and Talos received FDT
/chosen initramfs bounds:

start=0x000000002efff000 end=0x000000002efff296 len=0x0000000000000296

Talos then reported source=compiled-fallback reason=missing-artifact. The same
serial transcript shows the initramfs range starts at the early page-frame seed
and bootstrap reservation address, and overlaps the initial translation table
layout at 0x2efff000. The next implementation must reserve or copy the firmware
initramfs range before early page-table/bootstrap allocation can overwrite it.
The accepted archive/TFTP placement is therefore not enough by itself to claim
Pi 5 generated-root transport acceptance.

## Firmware Initramfs Reservation Contract

phase10-pi5-generated-root-firmware-initramfs-reservation-source-contract-20260616
accepts the next implementation boundary as
pi5-generated-root-firmware-initramfs-reserve-by-memory-plan-exclusion-v1.

The implementation must preserve the firmware-loaded artifact by excluding the
FDT /chosen linux,initrd-start..linux,initrd-end range from the Pi 5 early
usable-memory candidate before bootstrap page reservation, translation-table
layout, bootstrap allocator initialization, and cache transition. This keeps the
existing firmware-initramfs installer and all-or-nothing artifact parser, but
changes the memory plan so early kernel setup cannot reuse the artifact bytes.

Source ownership is split narrowly:

- src/device_tree/chosen.rs continues to parse FDT /chosen initrd bounds;
- src/boot/rpi5.rs owns DTB-phase ordering, memory planning, allocator/cache
  startup, and the firmware-initramfs generated-root installer;
- src/memory_map/layout.rs owns the extra exclusion in the conservative
  low-tail candidate policy;
- src/memory_map/page_frames.rs and src/memory_map/translation.rs remain
  consumers of the selected candidate and should not gain generated-root
  knowledge;
- src/initramfs.rs keeps artifact parsing, source reporting, and compiled
  fallback behavior.

Copy-first remediation, static maximum-size buffers, high-memory ownership,
DMA-safe allocation, SD/USB/block persistence, networking, SSH, and phase
transition remain rejected from this boundary. At source-contract acceptance
time, Pi 5 generated-root consumption still required a later local/static
implementation task and a fresh serialized Pi 5 proof; both follow-up tasks are
recorded below.

## Firmware Initramfs Reservation Core

phase10-pi5-generated-root-firmware-initramfs-reservation-core-20260616
implements the accepted local/static boundary. The Pi 5 DTB phase now retains
the optional FDT /chosen initrd bounds and passes them into the boot-memory
planner. The conservative low-tail planner page-rounds that range and excludes
it before page-frame seeding, bootstrap reservation, translation-table
placement, allocator initialization, and cache transition can claim the same
physical pages.

Focused tests cover the retained blocker shape
0x2efff000..0x2efff296 and the no-valid-low-tail failure case. The
generated-root artifact parser and compiled fallback path are unchanged. A
compile-only generated-root Pi 5 image build passed with image SHA-256
c37b34ebc955a7ce11cd0660bc18424ebcec4550de23ae1e6b0b9ad7c867e4bd and size
208984 bytes, but this is not Pi 5 consumption evidence. Boot archive
publication, lab mutation, hardware proof, persistence, SD/USB/block drivers,
networking, SSH, Phase 11/12 work, and phase transition remain deferred.

## Firmware Initramfs Reservation Pi 5 Proof

phase10-pi5-generated-root-firmware-initramfs-reservation-pi5-proof-20260616
accepts the serialized Pi 5 proof for firmware initramfs preservation. The
accepted rerun served the selected 'da591740/kernel_2712.img' twice at 208984
bytes, retained the selected tree through final pre-restore identity, restored
the baseline tree, and reported the firmware initramfs range
0x2efff000..0x2efff296 with 'source=firmware-initramfs' and
'reason=valid-artifact'.

This accepts Pi 5 generated-root artifact consumption through the firmware
initramfs path. The same run did not accept interactive command injection
success because scripted command input arrived empty and the scenario reported
an incomplete command-loop proof. Persistence, SD/USB/block drivers,
networking, SSH, Phase 11/12 work, and phase transition remain deferred.

## Firmware Initramfs Reservation Closeout

phase10-pi5-generated-root-firmware-initramfs-reservation-closeout-20260616
accepts the Pi 5 firmware-initramfs generated-root consumption boundary. The
source contract, core implementation, and serialized Pi 5 proof now reconcile:
Talos preserves the firmware-loaded initramfs_2712 bytes through early memory
setup and installs the artifact as generated-root source firmware-initramfs
with reason valid-artifact on Pi 5 hardware.

The prior hardware blocker from the first boot-transport proof is closed.
Scripted command injection for this proof scenario remains deferred because
the accepted run captured empty command input. Writable persistence,
SD/USB/block drivers, broader filesystem mutation, networking, SSH, Phase
11/12 expansion, and phase transition remain outside this contract.

## Command-Input Source Checkpoint

phase10-pi5-generated-root-command-input-source-checkpoint-20260617 selects
the next generated-root command-input proof contract without changing runtime
code or touching hardware. The selected scenario is
pi5-generated-root-manifest-command-input-v1: after the same Pi 5 boot retains
'source=firmware-initramfs reason=valid-artifact', the proof must wait for a
generated-root proof 'ready command=N' marker plus a visible 'talos> ' prompt,
save the post-prompt serial cursor, write exactly
'cat /generated/manifest.txt' through '/serial/write', and observe from the
saved cursor through '/serial/observe'.

The expected shell-visible output is
'Talos generated-root external artifact A'. Acceptance also requires retained
command text, generated-root proof dispatch with 'status=handled' and
'responses=1', and either 'ready-for-next prompt=true' or a final PASS marker
after the response. Existing Pi 5 serial-write and command-loop proofs remain
control-surface evidence only; they do not prove generated-root command input.
The selected dependency-gated follow-up is
phase10-pi5-generated-root-command-input-proof-core-20260617.

## Command-Input Proof Core

phase10-pi5-generated-root-command-input-proof-core-20260617 accepts the
local/static proof-core helper for the selected generated-root command-input
scenario. The helper
scripts/rpi5-generated-root-command-input-proof-review.sh reviews a
non-published generated-root boot archive, first by running the accepted
candidate archive review and then by checking root and serial-prefixed
initramfs_2712 placement, external artifact strings, and kernel prompt,
dispatch, response-count, ready-for-next, PASS, firmware-initramfs, and
valid-artifact marker strings.

The selected serialized hardware proof remains
pi5-generated-root-manifest-command-input-v1. It must write exactly
cat /generated/manifest.txt through /serial/write after same-boot
source=firmware-initramfs reason=valid-artifact, a generated-root proof ready
marker, and a visible talos> prompt are retained. It must then observe from the
saved post-prompt cursor through /serial/observe and retain the command text,
Talos generated-root external artifact A, dispatch status=handled responses=1,
and a following ready-for-next prompt=true or final PASS marker.

This accepts only local/static proof preparation. Pi 5 command-input success,
boot publication, hardwareTestLock acquisition, power-cycle, TFTP/serial
capture, persistence, storage drivers, networking, SSH, Phase 11/12 expansion,
and phase transition remain deferred to explicit follow-up tasks.

## Command-Input Pi 5 Proof And Closeout

phase10-pi5-generated-root-command-input-pi5-proof-20260617 is blocked with
classification inconclusive-command-input-capture-or-timing. The selected Pi 5
run retained source=firmware-initramfs reason=valid-artifact and a visible
talos> prompt, and the lab /serial/write endpoint accepted the nonempty
cat /generated/manifest.txt payload. Retained serial did not show that command
text, Talos generated-root external artifact A, or a handled manifest-command
dispatch before later empty-command timeouts.

phase10-pi5-generated-root-command-input-closeout-20260617 accepts only the
static reconciliation of that blocker. Pi 5 firmware-initramfs generated-root
consumption remains accepted from the reservation proof, but Pi 5 generated-root
command input remains blocked. No follow-up implementation task is selected;
supervisor planning is required before another timing/capture or harness-
adjusted command-input hardware attempt.

## Command-Input Capture Harness Core

phase10-pi5-generated-root-command-input-capture-harness-core-20260617 accepts
a local/static proof-harness correction for the command-input blocker. The first
failing invariant is now explicit: post-prompt /serial/write accepted bytes must
become shell-visible command text in retained serial, or the proof must classify
why they did not.

The updated helper
scripts/rpi5-generated-root-command-input-proof-review.sh records the source-
backed two-step generated-root proof sequence. The next hardware proof must wait
for command 0 readiness, write rootinfo, observe the rootinfo source-gate
response and ready command=1, then save the command 1 cursor, write
cat /generated/manifest.txt, and observe from that command 1 cursor. Acceptance
requires retained command text, Talos generated-root external artifact A,
dispatch command=1 status=handled responses=1, and ready command=2 or final
PASS evidence. Direct /serial/read fallback is diagnostic only when observe/
cursor evidence is saturated or unavailable.

The selected dependency-gated follow-up is
phase10-pi5-generated-root-command-input-capture-harness-pi5-proof-20260617.
Pi 5 command-input success, persistence, storage drivers, networking, SSH,
Phase 11/12 expansion, and phase transition remain deferred to explicit tasks.

## Command-Input Capture Harness Proof Closeout

phase10-pi5-generated-root-command-input-capture-harness-pi5-proof-20260617 is
blocked with classification command-input-observe-cursor-saturated. The
selected archive was published and the Pi 5 fetched the expected kernel and
initramfs before restore. Direct-read diagnostic serial retained
source=firmware-initramfs reason=valid-artifact, ready command=0, and a visible
talos> prompt, but the accepted /serial/observe contract could not be
evaluated because the saved cursor was already at the 4194304-byte retention
saturation boundary and returned zero bytes.

phase10-pi5-generated-root-command-input-capture-harness-closeout-20260617
accepts only that reconciliation. Pi 5 firmware-initramfs generated-root
consumption remains accepted, but Pi 5 generated-root command input remains
blocked on serial observe/cursor saturation. Direct /serial/read output remains
diagnostic only and cannot replace command-indexed retained command text and
manifest-output evidence. Supervisor planning is required before another
command-input hardware attempt, serial-retention/capture change, evidence-
contract change, persistence, storage drivers, networking, SSH, Phase 11/12
expansion, or phase transition.

## Command-Input Direct-Read Source Contract

phase10-pi5-generated-root-command-input-direct-read-source-contract-20260617
selects direct-read-after-saturated-cursor-command-input-v1 as the next bounded
evidence contract. The prior /serial/observe contract failed because the saved
cursor was pinned at the 4194304-byte retention boundary and returned zero
bytes. The replacement signal is not direct-read output alone; it is
command-indexed direct /serial/read capture bounded by same-boot source
evidence and immediate pre-write freshness reads.

The next helper/core task must require selected-tree identity, stable
same-power-cycle TFTP evidence, final pre-restore identity, and restore proof.
Within that same boot it must retain source=firmware-initramfs
reason=valid-artifact, ready command=0, and a visible talos> prompt, perform a
pre-write freshness read, write rootinfo, and retain rootinfo plus dispatch
command=0 status=handled responses=1 and ready command=1. It must then perform
a second pre-write freshness read, write cat /generated/manifest.txt, and
retain the command text, Talos generated-root external artifact A, dispatch
command=1 status=handled responses=1, and ready command=2,
ready-for-next prompt=true, or final PASS.

The selected dependency-gated follow-up is
phase10-pi5-generated-root-command-input-direct-read-harness-core-20260617.
Pi 5 command-input success, hardware publication, persistence, storage drivers,
networking, SSH, Phase 11/12 expansion, and phase transition remain deferred to
explicit tasks.

phase10-pi5-generated-root-command-input-direct-read-harness-core-20260617
accepts the local/static helper for that direct-read contract. The helper is
scripts/rpi5-generated-root-command-input-direct-read-proof-review.sh. It
retains the archive/source checks, emits the command-indexed direct-read proof
requirements, and can validate task-owned hardware evidence JSON. Its validator
requires same-boot firmware-initramfs valid-artifact source evidence,
selected-tree/TFTP/final-identity/restore proof fields, command 0 rootinfo
pre-write freshness and handled direct-read response, and command 1
cat /generated/manifest.txt pre-write freshness plus command text, Talos
generated-root external artifact A, dispatch command=1 status=handled
responses=1, and post-command readiness. Prompt-only, /serial/write-only,
stale pre-write direct-read, and missing-dispatch fixtures are rejected. The
selected dependency-gated follow-up is
phase10-pi5-generated-root-command-input-direct-read-pi5-proof-20260617.
Hardware publication and command-input acceptance remain deferred to that
serialized Pi 5 proof.

phase10-pi5-generated-root-command-input-direct-read-pi5-proof-20260617
accepts a terminal blocked Pi 5 proof with classification
command-input-command0-prelude-blocked. The candidate satisfied selected-tree
identity, stable same-power-cycle TFTP evidence, final pre-restore identity,
restore proof, same-boot source=firmware-initramfs reason=valid-artifact,
ready command=0, a visible talos> prompt, command 0 pre-write freshness, and a
successful /serial/write of rootinfo. The first failing invariant is the
command 0 direct-read window: it did not retain rootinfo, source evidence, or
dispatch command=0 status=handled responses=1 before the harness advanced.
Generated-root command-input acceptance remains rejected. The selected
dependency-gated follow-up is
phase10-pi5-generated-root-command-input-direct-read-closeout-20260617.

phase10-pi5-generated-root-command-input-direct-read-closeout-20260617 accepts
the static closeout for the direct-read chain with classification
pi5-generated-root-command-input-command0-paused-milestone-closeout-selected.
It preserves Pi 5 firmware-initramfs generated-root consumption as accepted,
but explicitly pauses shell-visible generated-root command input at the command
0 prelude blocker. The blocked invariant is the same as the proof: after a
fresh command 0 pre-write read and successful /serial/write of rootinfo, the
direct-read window did not retain rootinfo, source evidence, or
dispatch command=0 status=handled responses=1. No hardware retry, harness
adjustment, storage work, networking, SSH, Phase 11/12 expansion, or phase
transition is selected. The selected dependency-gated follow-up is
phase10-pi5-generated-root-milestone-10-3-closeout-20260617.

## Milestone 10.3 Closeout

phase10-pi5-generated-root-milestone-10-3-closeout-20260617 accepts the final
Milestone 10.3 generated-root transport boundary. The milestone is closed for
local/QEMU no-kernel-rebuild generated-root transport and for Pi 5
firmware-initramfs generated-root consumption of the same artifact format.

Pi 5 shell-visible generated-root command input remains unaccepted and is
explicitly paused at the command 0 prelude blocker from the direct-read
closeout. That blocker is retained as future supervisor-planned control-surface
work, not as a reason to reopen generated-root transport acceptance.

No post-closeout implementation task is selected because no explicit queued task
exists. Supervisor planning is required before command-input retry, persistence,
storage-driver work, networking, SSH, Phase 11/12 expansion, or phase
transition.

phase10-pi5-serial-command0-prelude-source-contract-20260617 records the
supervisor-planned command 0 control-surface contract. Static inspection of the
accepted direct-read proof shows that the Pi 5 run advanced to ready command=1
after /serial/write accepted rootinfo, but the retained post-write direct-read
window missed rootinfo command text, the talos: generated-root
source=firmware-initramfs reason=valid-artifact response, and dispatch
command=0 status=handled responses=1. This is not accepted generated-root
command input.

The selected local/static follow-up is
phase10-pi5-serial-command0-prelude-guard-core-20260617. It must make the
command 0 prelude contract mechanically checkable: ready command=0, prompt,
immediate pre-write freshness, successful write of rootinfo, retained rootinfo
text or equivalent command=0 line record, retained generated-root source
response, dispatch command=0 status=handled responses=1, and ready command=1
before accepting any command=1 timeout or later-command evidence. Same-shaped
direct-read timing retries, prompt-only evidence, and /serial/write-only
evidence remain rejected.

phase10-pi5-serial-command0-prelude-guard-core-20260617 accepts the
local/static guard-core helper contract. The helper now validates command 0 as
an ordered transaction, accepting either literal rootinfo text or the target
proof's line command=0 hex=726f6f74696e666f record before the
generated-root source response, dispatch command=0 status=handled responses=1,
and ready command=1. The task-owned positive fixture passes the helper and the
retained blocked direct-read proof remains rejected. The selected hardware
follow-up is phase10-pi5-serial-command0-prelude-pi5-proof-20260617; no Pi 5
hardware run or generated-root command-input success is accepted by the
guard-core task.

phase10-pi5-serial-command0-prelude-pi5-proof-20260617 blocks with
classification serial-command0-prelude-source-response-retention-blocked. The
selected hardware run preserves firmware-initramfs generated-root readiness,
selected-tree identity, stable TFTP evidence, final pre-restore identity, and
baseline restore. It proves rootinfo reached the command loop: the retained
direct-read window contains command=0 line evidence, dispatch command=0
status=handled responses=1, and ready command=1. It does not accept
generated-root command input because the same command 0 direct-read window did
not retain the firmware-initramfs valid-artifact source response required by
the accepted guard. Command 1 manifest proof remains out of scope until command
0 satisfies the guard or a closeout changes the accepted evidence contract.

## Deferred

Writable persistence, SD/USB/block drivers, networking, SSH, and Phase 11/12
feature expansion remain deferred. This contract now accepts Pi 5
firmware-initramfs consumption of the already accepted generated-root artifact
format, but not persistence, shell-visible command-input success for the proof
scenario, or a phase transition.
