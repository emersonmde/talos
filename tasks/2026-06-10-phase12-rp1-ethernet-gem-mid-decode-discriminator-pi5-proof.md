# Phase 12 RP1 Ethernet GEM MID Decode Discriminator Pi 5 Proof

Task: phase12-rp1-ethernet-gem-mid-decode-discriminator-pi5-proof-20260610

Status: accepted

Classification: rp1-ethernet-gem-mid-decode-discriminator-capture-chain-blocked

Evidence level: image/archive inspection, lab-controller boot archive
publication/snapshot/restore evidence, serial hardware output, TFTP/capture
evidence, and post-run restore evidence. The hardware result is accepted only
as a precise capture-chain blocker, not as Ethernet readiness.

## Scope

- Promoted the serialized Pi 5 proof selected by the accepted discriminator
  closeout.
- Acquired hardwareTestLock before boot archive publication or Pi 5 hardware
  interaction.
- Added and reviewed candidate/control boot archives for the selected
  same-run observed SYSINFO positive-control plus translated GEM MID
  discriminator.
- Published candidate and control archives under the lock, retained
  lab-controller identity, serial/TFTP capture attempts, and restored the
  pre-run snapshot after staged attempts.
- Recorded that this lab API returns 404 for GET /; /boot/files retained the
  boot identity fields used by the repository lab-controller contract.

## Findings

- fixed: added Pi 5 candidate/control boot scenarios and archive/review
  scripts for the selected discriminator.
- fixed: registered the new boot scenarios in build.rs to avoid check-cfg
  warning noise.
- fixed: candidate/control archive reviews passed with nonce
  gemdisc-20260610T0349Z.
- fixed: snapshot pre-gem-mid-decode-discriminator-proof-20260610T0402Z was
  created and restored before lock release.
- fixed: candidate serial output showed observed-positive-control-raw
  0x20001927, observed-positive-control-matches-expected=true, GEM MID raw
  0xdeaddead, and classification
  observed-rp1-positive-control-gem-mid-0x1f-window-sentinel.
- deferred: decisive identity-joined hardware classification. The useful
  candidate line was captured through saturated direct-read serial after the
  generic bundle stopped, while TFTP/final-identity evidence did not join into
  one accepted transaction.
- deferred: control hardware marker visibility. The control archive was
  reviewed and staged, but the control run stopped at the same serial-observe
  capture boundary before retaining the no-MMIO/no-Ethernet marker.
- not-an-issue: no Ethernet behavior was accepted.

No findings were removed.

## Hardware Result

Accepted blocker:
rp1-ethernet-gem-mid-decode-discriminator-capture-chain-blocked.

The candidate serial line is useful: observed RP1 SYSINFO_CHIP_ID at
0x1c00000000 returned 0x20001927 and translated MACB_MID at 0x1f001000fc
returned 0xdeaddead. That preserves the selected discriminator result, but it
is not a decisive hardware proof because the capture transaction did not join
with stable expected TFTP fetch, final pre-restore identity, and paired
control marker evidence.

## Rejected Claims And Retained Risks

Rejected: live GEM visibility, broad Ethernet MMIO readiness, Ethernet driver
readiness, RP1 MMIO/DMA programming, descriptor rings, DMA ownership, transfer
completion, interrupt completion, clock/reset ownership, PHY reset ownership,
packet I/O, networking, sockets, SSH, Phase 12.2, and phase transition.

Retained risks: the 0x1f translated RP1 window sentinel source remains
unproven; PCIe/RP1 bridge or window enablement remains unaccepted; Ethernet
clock/reset and PHY/MDIO ownership remain unaccepted; the Pi 5 capture
helper/serial observation path needs repair before another same-shaped
hardware proof.

## Evidence

- Archive reviews:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-pi5-proof/archive-review/.
- Root endpoint probe:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-pi5-proof/root-endpoint-probe.txt.
- Pre-run snapshot:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-pi5-proof/pre-run-snapshot-create.json.
- Candidate positive serial window:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-pi5-proof/candidate-full-run/direct-serial-window-after-observe-stop.json.
- Candidate capture blocker attempts: candidate-debug-run, candidate-full-run,
  and candidate-manual-run under the task evidence directory.
- Control capture blocker attempt: control-direct-run under the task evidence
  directory.
- Final restored lab identity:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-pi5-proof/final-lab-root-before-lock-release.json.

## Validation

- static inspection: accepted closeout, source implementation, helper scripts,
  and retained evidence reviewed.
- format check: cargo fmt --all -- --check passed after formatting.
- image/archive inspection: candidate and control review scripts passed.
- lab-controller API: hardwareTestLock acquired before publication; snapshot
  created and restored after staged attempts.
- serial hardware output: candidate direct serial window retained the selected
  positive-control/GEM MID sentinel line.
- TFTP/capture evidence: retained blocker evidence, but no decisive
  identity-joined transaction.

## Next Action

Supervisor planning is required for a capture-chain repair or a different
bounded acceptance slice. Do not retry the same-shaped hardware proof,
implement Ethernet behavior, or start Phase 12.2 from this blocker.
