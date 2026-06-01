# Phase 10 Pi 5 Help Command Proof Intervention Analysis

Task: phase10-pi5-local-help-command-proof-20260531
Timestamp: 2026-06-01T00:41Z
Role: worker

## First-principles problem statement

The accepted help-command core evidence is local QEMU/substitute evidence. It
proves the Talos feature path can accept help, dispatch it through the
descriptor-backed command loop, print accurate help text through
descriptor-backed stdout, and return to a ready prompt in the local substitute
environment.

The Pi 5 proof requires a different claim: after publishing the accepted help
candidate to the lab boot tree and power-cycling the Pi 5, the lab must retain
fresh serial bytes from that boot and command interaction. Right now that claim
cannot be produced because the lab serial capture cursor is not advancing. TFTP
and lab status evidence show the Pi/lab path changes, but serial evidence keeps
returning an old literal-echo transcript. That means the missing evidence is the
physical serial-capture path, not the help-command feature semantics.

## Required invariant

After a fresh boot or known-good control run, the lab serial cursor should
advance and retained serial output should include new bytes for the selected
boot tree. A fresh cursor plus power cycle plus command write should then let
/serial/observe or equivalent capture bytes emitted after that cursor.

## Contradicting evidence

- Candidate run: the help-command candidate archive was published and the Pi
  fetched da591740/kernel_2712.img over TFTP, but serial stayed at cursor
  3827326 with stale literal-echo output.
- Known-good control: the prior accepted boot tree was restored and fetched
  over TFTP after a power cycle, but serial still reported cursor 3827326 with
  the same stale literal-echo output.
- Serial write: writing help through the lab serial API did not produce a
  cursor advance or fresh retained response.
- Unchanged candidate rerun: the unchanged help candidate was republished and
  fetched again over TFTP after power cycle, but serial still reported cursor
  3827326 with stale literal-echo output.
- Follow-up peeks: later non-invasive health/status/serial checks continued to
  show normal lab health and restored boot-tree status while the serial cursor
  stayed fixed at 3827326.

## Unproven assumptions

- The serial capture service may be stale independently of the Pi boot path.
- The cursor API may be returning stale retained state even if UART bytes are
  arriving somewhere else.
- UART wiring, USB serial adapter state, or collector file descriptor state may
  be disconnected or wedged independently of TFTP and power control.
- The Pi may be booting and emitting output that is not entering the retained
  collector log.
- Lab health/status success does not by itself prove serial retention is fresh.

## Qualitatively different approaches before more polling

- Recovery/control path: reset or restart the serial collector/service using a
  documented lab-maintenance action, then run a known-good control and require
  cursor advancement with fresh retained output before rerunning the unchanged
  help candidate.
- Independent capture path: attach or enable an independent serial capture
  mechanism/control source and prove that a known-good boot emits fresh output
  outside the stuck retained cursor path before deciding whether the lab API
  collector or physical UART path is the failing component.

## Smallest decisive discriminator

Do not rerun the unchanged help candidate yet. First obtain one of:

- A known-good control boot whose serial cursor advances beyond 3827326 and
  whose retained output is visibly fresh for the restored accepted boot tree.
- A documented serial-capture recovery action followed by the same known-good
  cursor-advance proof.

Only after that discriminator succeeds should the unchanged help candidate be
rerun for the Pi 5 proof.

## Workaround removal/quarantine plan

No help proof-code visibility shims or Talos runtime behavior changes are
allowed while the serial-capture path is unproven. The prior proof-local and
candidate files remain quarantined as paused-task artifacts. If a capture-path
recovery later proves fresh known-good serial output, rerun the unchanged help
candidate first; only then consider proof-harness-only visibility changes, and
only with candidate identity, fresh serial cursor, TFTP delta, known-good
control, and unchanged candidate rerun evidence retained.
