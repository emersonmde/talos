# Phase 10 Pi 5 Serial Command 0 Post-Write Observe Contract

Task id: phase10-pi5-serial-command0-post-write-observe-contract-20260617

Status: accepted

Classification:
command0-post-write-observe-contract-helper-core-selected

Evidence level: static/source/task evidence inspection, accepted command0
write-delivery v2 closeout, lab-controller serial endpoint contract, accepted
local/Pi 5 serial-write ingress controls, task-owned JSON evidence, docs build,
and diff checks. No kernel behavior change, Pi 5 hardware run, boot archive
publication, lab mutation, hardwareTestLock acquisition, source-response
retention proof, generated-root command-input acceptance, storage, networking,
SSH, Phase 11/12 expansion, or phase transition was performed.

## Goal

Select the smallest feature-led discriminator for command 0 write delivery
after the v2 Pi 5 proof retained an accepted rootinfo write but no post-write
command output.

## First-Principles Problem Statement

The feature under test is still local serial interactivity: after Talos prints a
visible talos> prompt and reports ready command=0, a lab serial write of
rootinfo plus newline should enter the runtime-console fd0 canonical input, the
command loop should dispatch command 0, and the serial output captured after
the saved pre-write cursor should retain the command 0 line or rootinfo text,
dispatch command=0 status=handled, responses=1, and ready command=1.

POST /serial/write returning ok and bytes=9 proves only that the lab API
accepted the bytes for transmission. It does not prove that the bytes reached
the Pi UART, reached the command loop, or were retained by the capture path
after command handling.

## Contradicting Evidence

The v2 Pi 5 proof removed the earlier selected-kernel/TFTP uncertainty. The
candidate boot used selected tree
06eb7fd758522c4da95317cb6c9b6fb515612e22bbea3e291a4c2b3d13952212, the
selected da591740/kernel_2712.img size was 208984 bytes, and the
same-power-cycle TFTP delta retained two selected-kernel serves at 208984
bytes before restore.

The same boot also retained generated-root readiness for command 0:
source=firmware-initramfs, reason=valid-artifact,
rpi5-generated-root-boot-transport-proof: ready command=0, and a visible
talos> prompt. The immediate command 0 pre-write direct read retained only two
bytes and did not already contain rootinfo or command 0 output.

After that fresh boundary, /serial/write accepted 9 bytes for rootinfo. The
post-write direct-read window retained zero bytes. It therefore retained no
rootinfo, no command 0 line marker, no dispatch command=0 status=handled, no
responses=1, and no ready command=1.

This contradicts the invariant that an accepted post-prompt rootinfo write
should become observable as ordered command 0 output in the same boot.

## Unproven Assumptions

- lab endpoint route/observe semantics: the accepted lab-controller contract
  says observe consumes newly available serial bytes and returns log bytes
  after a supplied cursor, and it is the preferred endpoint after write. The v2
  proof used direct /serial/read after write, so it has not yet proved the
  route that binds a saved prompt cursor to post-write retained log bytes.
- write timing: the v2 proof did not prove whether the command loop needed a
  longer or cursor-bound observe window after the write before bytes became
  available.
- direct-read capture: direct /serial/read may consume only newly available
  device bytes during its timeout. An empty read after write is decisive for
  that direct-read transaction, but it does not prove that cursor-bound
  /serial/observe from the prompt boundary would also be empty.
- PL011 RX interrupt/polling behavior: local and prior Pi 5 controls prove
  prompt-live serial input can work, but this generated-root command0 run has
  not yet isolated whether bytes reached the Pi UART and were drained by the
  command-loop input path after the prompt.
- command-loop canonical input: the source command loop contains the rootinfo
  builtin and already emits line/dispatch/response/ready markers in accepted
  local evidence, but the v2 hardware proof did not retain a command 0
  canonical line after the rootinfo write.

## Selected Contract

The smallest decisive discriminator is a helper/core task that changes the
proof transaction, not kernel behavior: save the cursor at the visible prompt
or immediately before the write, write rootinfo with append_newline=true, then
retain a cursor-bound POST /serial/observe window from that saved cursor. The
helper must classify:

- accepted: ordered command 0 line/rootinfo evidence, dispatch
  command=0 status=handled, responses=1, and ready command=1 are retained
  after the saved pre-write cursor.
- blocked: the write is accepted but the post-write observe window lacks the
  ordered command 0 output.
- inconclusive: candidate identity, selected-kernel/TFTP, fresh prompt cursor,
  write response, final identity, or restore proof is missing or stale.

The selected next task is
phase10-pi5-serial-command0-post-write-observe-helper-core-20260617.

## Findings

- fixed: the problem statement now separates write endpoint acceptance from
  feature-level command0 write delivery.
- fixed: the selected-kernel/TFTP precondition is treated as accepted, so the
  next discriminator focuses only on post-write capture and command delivery.
- fixed: lab endpoint semantics now select cursor-bound /serial/observe after
  write as the smallest source/static follow-up instead of a same-shaped direct
  /serial/read retry.
- deferred: any kernel UART/input change remains dependency-gated until the
  observe-backed proof discriminates capture semantics from runtime input
  behavior.
- rejected: command0 write-delivery success, command0 source-response
  retention success, generated-root command-input success, storage, networking,
  SSH, Phase 11/12 expansion, and phase transition.
- not-an-issue: no hardware lock, lab mutation, boot publication, or kernel
  source change was required for this source/static contract.

## Evidence

- Accepted v2 closeout:
  tasks/2026-06-17-phase10-pi5-serial-command0-write-delivery-v2-closeout.md.
- V2 closeout classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-v2-closeout/classification.json.
- V2 Pi 5 proof:
  tasks/2026-06-17-phase10-pi5-serial-command0-write-delivery-pi5-proof-v2-after-tftp-precondition.md.
- V2 run classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-pi5-proof-v2-after-tftp-precondition/candidate-command0-write-delivery-v2-20260617T124424Z/classification.json.
- V2 direct-read evidence:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-pi5-proof-v2-after-tftp-precondition/candidate-command0-write-delivery-v2-20260617T124424Z/direct-read-evidence.json.
- Lab-controller serial endpoint contract:
  docs/src/project/lab-controller.md.
- Accepted local/QEMU serial-write ingress control:
  scripts/qemu-local-serial-write-ingress-control.sh.
- Accepted Pi 5 serial-write ingress control:
  tasks/2026-06-01-phase10-pi5-serial-write-ingress-control-proof.md.
- This task classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-post-write-observe-contract/classification.json.
- This task evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-post-write-observe-contract/evidence-map.json.

## Acceptance Check

- First-principles command0 write-delivery problem statement and invariant:
  satisfied.
- Contradicting v2 evidence names selected-kernel/TFTP agreement, same-boot
  readiness, fresh pre-write boundary, accepted 9-byte write, and empty
  post-write capture: satisfied.
- Unproven assumptions are identified separately for endpoint semantics,
  timing, direct-read capture, PL011 RX/input behavior, and command-loop
  canonical input: satisfied.
- selected_next_task is
  phase10-pi5-serial-command0-post-write-observe-helper-core-20260617:
  satisfied.
- Rejected claims remain explicit: satisfied.

## Validation

- static/source/task evidence inspection: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote phase10-pi5-serial-command0-post-write-observe-helper-core-20260617 on
the next worker wake if dependencies remain satisfied, the repository remains
clean, hardwareTestLock is unlocked/restored, and supervisorIntervention is
inactive. Do not run hardware or accept command0 source-response retention from
this source/static contract.
