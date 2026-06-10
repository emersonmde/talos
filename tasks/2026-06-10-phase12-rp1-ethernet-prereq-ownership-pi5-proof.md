# Phase 12 RP1 Ethernet Prerequisite Ownership Pi 5 Proof

Task: phase12-rp1-ethernet-prereq-ownership-pi5-proof-20260610

Status: accepted

Classification: rp1-ethernet-prereq-ownership-report-visibility-control-output

Evidence level: image/archive inspection, lab-controller API, serial hardware
boot/output, TFTP/capture evidence, and restore proof. This accepts only
candidate/control report visibility, not Ethernet runtime or hardware
ownership of the reported prerequisites.

## Scope

- Promoted the queued serialized Pi 5 proof after the accepted report closeout
  selected it and hardwareTestLock was unlocked.
- Added the thinnest runtime candidate/control boot scenarios and archive
  helpers needed to print the accepted prerequisite ownership report path on Pi
  5 serial.
- Acquired hardwareTestLock before boot archive publication or Pi 5 power
  cycling.
- Rebuilt candidate and paired no-ownership/no-Ethernet control archives with
  run-unique capture nonces.
- Published and captured candidate/control runs with capture-chain-v4
  selected-tree identity, expected TFTP fetch bytes, run-unique serial marker
  freshness, final pre-restore identity, restore proof, and task-owned JSON.
- Restored the pre-task boot tree before releasing hardwareTestLock.
- Did not program RP1 MMIO, write clocks/resets/GPIO, assert or deassert PHY
  reset, perform MDIO transactions, create DMA descriptors or rings, claim
  interrupts/completions, perform packet I/O, add networking/sockets/SSH, start
  Phase 12.2, or infer a phase transition.

## Findings

- fixed: candidate archive review passed with nonce
  prereq-candidate-20260610T082202Z, archive sha256
  3105fbac5e165c540581cd20fbd98738f3d314313ff001661bb3f50e6c5e68c9,
  kernel sha256
  03e86850218b1cefb10aecc1cfced68fc30a63e9d59b2c1f2cff40558a6e9e3f,
  and kernel_2712.img size 49176 bytes.
- fixed: control archive review passed with nonce
  prereq-control-20260610T082202Z, archive sha256
  9ca77355646466f3321b0c485379eb3ee4e5829d9f40fc25a1b4729592acd600,
  kernel sha256
  1eed20ef9c9e07e979c9454055b3ca888b25e726ef44aaa70c644f9c1a94ed78,
  and kernel_2712.img size 48856 bytes.
- fixed: candidate capture-chain-v4 joined selected tree
  66338c41dd7e1166d1ae5222387bee653c18eb5722b69b75183dac5bd935d149,
  two matching TFTP fetches of da591740/kernel_2712.img at 49176 bytes,
  run-unique serial nonce freshness, final pre-restore identity, and restore
  proof.
- fixed: candidate serial retained 25 required marker occurrences and printed
  the accepted prerequisite ownership report fields, including the
  context-only observed-window MACB_MID identity, RP1_INT_ETH, pclk/hclk/
  tsu_clk/tx_clk, RGMII-ID phy1, GPIO32 PHY reset metadata, PHY/MDIO policy,
  DMA/descriptor policy, rejected claims, and classification
  rp1-ethernet-prereq-ownership-report-visible.
- fixed: control capture-chain-v4 joined selected tree
  bdbb7caef7f11abbd56b4104009048c97a9bfcc37be509c15e962bb516f7b24f,
  two matching TFTP fetches of da591740/kernel_2712.img at 48856 bytes,
  run-unique serial nonce freshness, final pre-restore identity, and restore
  proof.
- fixed: control serial retained 28 required marker occurrences through the
  same report path while withholding candidate-only prerequisite facts and
  classifying no-ownership-no-ethernet-rp1-ethernet-prereq-control.
- fixed: final lab boot tree was restored to
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 with
  effective kernel kernel_2712.img.
- not-an-issue: both capture-bundle summaries retain
  serial-drain-not-empty-before-power as the legacy v2 suggestion, but the
  repaired capture-chain-v4 identity join passed for both runs because
  run-unique nonce freshness, selected-tree identity, TFTP, final identity, and
  restore gates all matched.
- deferred: actual clock/reset, GPIO32/PHY reset, MDIO/PHY, interrupt, DMA,
  descriptor-ring, packet I/O, networking, sockets, SSH, Phase 12.2, and phase
  transition work remain future tasks.

No findings were removed.

## Hardware Result

Accepted result:
rp1-ethernet-prereq-ownership-report-visibility-control-output.

The candidate proved that the accepted prerequisite ownership report output is
visible on Pi 5 serial under the repaired capture chain. The paired control
proved the same report path without candidate-only prerequisite facts. This is
report visibility/control output only; it does not prove Talos ownership of
clocks/resets, GPIO32/PHY reset, MDIO/PHY, interrupts, DMA, descriptors, or
packet behavior.

## Rejected Claims And Retained Risks

Rejected: Ethernet driver readiness, broad Ethernet MMIO readiness, RP1 MMIO
writes, clock/reset writes or ownership, GPIO32/PHY reset ownership, MDIO/PHY
ownership, interrupt delivery/completion, DMA, descriptor rings, packet I/O,
networking, sockets, SSH, Phase 12.2, and phase transition.

Retained risks: observed-window MACB_MID identity plus report visibility still
does not prove prerequisite ownership; all write-backed and ownership-backed
Ethernet prerequisites remain unaccepted until a future scoped task implements
and proves them.

## Evidence

- Classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-prereq-ownership-pi5-proof/classification.json.
- Evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-prereq-ownership-pi5-proof/evidence-map.json.
- Capture summary:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-prereq-ownership-pi5-proof/capture-summary.json.
- Candidate run:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-prereq-ownership-pi5-proof/candidate-run/.
- Control run:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-prereq-ownership-pi5-proof/control-run/.
- Archive reviews:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-prereq-ownership-pi5-proof/archive-review/.
- Pre-run snapshot:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-prereq-ownership-pi5-proof/pre-run-snapshot-create.json.
- Final restore:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-prereq-ownership-pi5-proof/final-post-restore-boot-files.json.

## Validation

- static inspection: accepted source contract/report closeout, runtime
  scenarios, archive helpers, capture summaries, identity joins, and docs
  reviewed.
- fmt check: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet rp1_ethernet passed.
- image/archive inspection: candidate and control review scripts passed.
- lab-controller API: hardwareTestLock acquired before publication; snapshot
  created and restored; final /boot/files confirmed restored tree.
- serial hardware output: candidate and control markers retained with
  run-unique nonces from direct-read serial windows.
- TFTP/capture evidence: candidate and control stable deltas both retained two
  expected da591740/kernel_2712.img fetches with matching bytes.
- capture-chain replay: candidate and control identity-join-v4 checks passed.

## Next Action

Mechanically promote
phase12-rp1-ethernet-prereq-ownership-proof-closeout-20260610 on the next
worker wake. The closeout must reconcile the accepted report visibility/control
proof without broadening into hardware/runtime prerequisite ownership, Ethernet
driver implementation, RP1 MMIO writes, DMA, descriptor rings, interrupts,
packet I/O, networking, sockets, SSH, Phase 12.2, or a phase transition.
