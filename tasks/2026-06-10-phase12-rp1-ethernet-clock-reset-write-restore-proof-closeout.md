# Phase 12 RP1 Ethernet Clock/Reset Write-Restore Proof Closeout

Task id: phase12-rp1-ethernet-clock-reset-write-restore-proof-closeout-20260610
Status: accepted
Owner: worker
Classification:
rp1-ethernet-clk-eth-tsu-ctrl-write-restore-frontier-closed
Evidence level: static inspection of accepted proof evidence, task records,
documentation, and git history.

## Goal

Close out the accepted CLK_ETH_TSU_CTRL write/restore Pi 5 proof and decide
whether the next Phase 12.1 ownership slice is mechanically objective.

## Findings

- fixed: reconciled the accepted proof classification
  rp1-ethernet-clk-eth-tsu-ctrl-idempotent-write-restored-with-control from
  commit 76edd28400da10a73e32af5e85cf872f7125de17.
- fixed: confirmed the candidate capture-chain-v4 joined selected tree
  a8c5f9b18e4443887fa7a834d8ee22691f49c0c5b7f7122cfe7ed36d064377a2,
  two expected da591740/kernel_2712.img TFTP fetches at 49704 bytes, fresh
  serial nonce output, final pre-restore identity, and restore proof.
- fixed: confirmed candidate serial reported CLK_ETH_TSU_CTRL at
  0x1c00018134 with pre_raw 0x10000800, post_raw 0x10000800,
  restore_raw 0x10000800, post_eq_pre=true, restore_eq_pre=true, and
  classification rp1-ethernet-clk-eth-tsu-ctrl-idempotent-write-restored.
- fixed: confirmed the paired control capture-chain-v4 joined selected tree
  457859469383c34f4d3c241f46c164f0ab560e81cb275154cde4e7ad5152f458,
  two expected da591740/kernel_2712.img TFTP fetches at 49120 bytes, fresh
  serial nonce output, final pre-restore identity, and restore proof while
  withholding writable clock target construction and candidate-only facts.
- fixed: confirmed the lab boot tree was restored to
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 before
  hardwareTestLock release.
- not-an-issue: the idempotent write used the accepted source contract's
  pre-read-raw-only write rule and preserved full raw value, enable,
  auxsource, source, and reserved fields by writing/restoring the observed raw
  value.
- deferred: broad clock/reset ownership, shared-clock ownership, CLK_ETH_CTRL,
  reset-controller ownership, GPIO32/PHY reset ownership, MDIO/PHY ownership,
  DMA, descriptor rings, interrupt completion, packet I/O, networking, sockets,
  SSH, Phase 12.2, and phase transition remain future or rejected scope.
- deferred: no mechanically objective next Phase 12.1 ownership slice follows
  from this exact idempotent CLK_ETH_TSU_CTRL proof without supervisor-planned
  scope and acceptance criteria for a different register, reset, GPIO32/PHY,
  MDIO/PHY, interrupt, DMA, descriptor, or packet dependency.

No findings were removed.

## Accepted Boundary

The accepted frontier is closed at one Ethernet-private
CLK_ETH_TSU_CTRL idempotent write/readback/restore proof with a paired
no-clock-write control. The candidate proves the selected pre-read raw value
could be written back, read back, restored, and read back again on Pi 5 while
preserving the observed raw value. The paired control proves the same
report/capture path while constructing no writable clock target.

This closeout does not accept broad clock/reset ownership, shared-clock
ownership, CLK_ETH_CTRL ownership, reset-controller ownership, GPIO32/PHY reset
ownership, MDIO/PHY ownership, DMA, descriptor rings, transfer completion,
interrupt completion, packet I/O, networking, sockets, SSH, Phase 12.2, or a
phase transition.

## Same-Shaped Retry Policy

Same-shaped CLK_ETH_TSU_CTRL idempotent write/restore hardware retries are
closed for this candidate/control pair. A future task must provide materially
different scope and explicit acceptance criteria, such as a non-idempotent
field transition with restore proof, a separate CLK_ETH_CTRL or shared-clock
safety contract, reset-controller evidence, GPIO32/PHY reset ownership,
MDIO/PHY ownership, interrupt completion, DMA/descriptor ownership, or packet
I/O scope. This closeout does not choose such a task.

## Evidence

- Proof task record:
  tasks/2026-06-10-phase12-rp1-ethernet-clock-reset-write-restore-pi5-proof.md.
- Proof classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clock-reset-write-restore-pi5-proof/classification.json.
- Proof evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clock-reset-write-restore-pi5-proof/evidence-map.json.
- Proof capture summary:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clock-reset-write-restore-pi5-proof/capture-summary.json.
- Closeout classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clock-reset-write-restore-proof-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clock-reset-write-restore-proof-closeout/evidence-map.json.

## Validation

- static inspection: proof task record, proof classification/evidence map,
  capture summary, project docs, and git history reviewed.
- JSON validation: jq empty on closeout classification/evidence-map JSON
  passed.
- diff check: git diff --check passed.
- docs build: /home/node/.cargo/bin/mdbook build passed.
- staged diff check: git diff --cached --check passed.

## Next Action

Supervisor planning is required for the next explicit Phase 12.1 ownership
slice. No mechanically objective follow-up is selected from this exact
CLK_ETH_TSU_CTRL idempotent write/restore proof alone.
