# Phase 12 RP1 Ethernet CLK_ETH_CTRL Write-Restore Proof Closeout

Task id: phase12-rp1-ethernet-clk-eth-ctrl-write-restore-proof-closeout-20260610
Status: accepted
Owner: worker
Classification:
rp1-ethernet-clk-eth-ctrl-write-restore-frontier-closed
Evidence level: static inspection of accepted proof evidence, task records,
documentation, and git history.

## Goal

Close out the accepted CLK_ETH_CTRL write/restore Pi 5 proof and decide
whether the next Phase 12.1 ownership slice is mechanically objective.

## Findings

- fixed: reconciled the accepted proof classification
  rp1-ethernet-clk-eth-ctrl-idempotent-write-restored-with-control from commit
  c3a9387267088f6a9e974fac21db0fd24417b378.
- fixed: confirmed the candidate rerun capture-chain-v4 joined selected tree
  8d71d54345a64913e451969b9303cd7df351baa64950dffd2fca890897cf05b3,
  two expected da591740/kernel_2712.img TFTP fetches at 50040 bytes, fresh
  serial nonce output, final pre-restore identity, and restore proof.
- fixed: confirmed candidate serial reported CLK_ETH_CTRL at 0x1c00018064
  with pre_raw 0x10000800, post_raw 0x10000800, restore_raw 0x10000800,
  post_eq_pre=true, restore_eq_pre=true, and classification
  rp1-ethernet-clk-eth-ctrl-idempotent-write-restored.
- fixed: confirmed the paired control capture-chain-v4 joined selected tree
  5c5144ce68c0537b39dcb216b2ae1343c9197ac7deb310f5c7bcc811efe31d40,
  two expected da591740/kernel_2712.img TFTP fetches at 49464 bytes, fresh
  serial nonce output, final pre-restore identity, and restore proof while
  withholding writable CLK_ETH_CTRL target construction.
- fixed: confirmed the lab boot tree was restored to
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 with
  effective kernel kernel_2712.img and da591740/kernel_2712.img at 104136
  bytes before hardwareTestLock release.
- not-an-issue: the idempotent write used the accepted source contract's
  pre-read-raw-only write rule and preserved the full raw value, enable,
  auxsource, source, and reserved fields by writing/restoring the observed raw
  value.
- deferred: the retained short-window inconclusive candidate capture and
  staging-blocked candidate attempt remain non-acceptance evidence; the paired
  control plus unchanged candidate rerun completed the required triage.
- deferred: broad clock/reset ownership, shared-clock ownership,
  reset-controller ownership, GPIO32/PHY reset ownership, MDIO/PHY ownership,
  DMA, descriptor rings, interrupt completion, packet I/O, networking, sockets,
  SSH, Phase 12.2, and phase transition remain future or rejected scope.
- deferred: no mechanically objective next Phase 12.1 ownership slice follows
  from this exact idempotent CLK_ETH_CTRL proof without supervisor-planned
  scope and acceptance criteria for a different reset, GPIO32/PHY, MDIO/PHY,
  interrupt, DMA, descriptor, packet, or functional-clock dependency.

No findings were removed.

## Accepted Boundary

The accepted frontier is closed at one Ethernet-private CLK_ETH_CTRL
idempotent write/readback/restore proof with a paired no-clock-write control.
The candidate proves the selected pre-read raw value could be written back,
read back, restored, and read back again on Pi 5 while preserving the observed
raw value. The paired control proves the same report/capture path while
constructing no writable clock target.

This closeout does not accept broad clock/reset ownership, shared-clock
ownership, reset-controller ownership, GPIO32/PHY reset ownership, MDIO/PHY
ownership, DMA, descriptor rings, transfer completion, interrupt completion,
packet I/O, networking, sockets, SSH, Phase 12.2, or a phase transition.

## Same-Shaped Retry Policy

Same-shaped CLK_ETH_CTRL idempotent write/restore hardware retries are closed
for this candidate/control pair. A future task must provide materially
different scope and explicit acceptance criteria, such as a functional
non-idempotent field transition with restore proof, shared-clock safety,
reset-controller evidence, GPIO32/PHY reset ownership, MDIO/PHY ownership,
interrupt completion, DMA/descriptor ownership, or packet I/O scope. This
closeout does not choose such a task.

## Evidence

- Proof task record:
  tasks/2026-06-10-phase12-rp1-ethernet-clk-eth-ctrl-write-restore-pi5-proof.md.
- Proof classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clk-eth-ctrl-write-restore-pi5-proof/classification.json.
- Proof evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clk-eth-ctrl-write-restore-pi5-proof/evidence-map.json.
- Proof capture summary:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clk-eth-ctrl-write-restore-pi5-proof/capture-summary.json.
- Closeout classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clk-eth-ctrl-write-restore-proof-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clk-eth-ctrl-write-restore-proof-closeout/evidence-map.json.

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
CLK_ETH_CTRL idempotent write/restore proof alone.
