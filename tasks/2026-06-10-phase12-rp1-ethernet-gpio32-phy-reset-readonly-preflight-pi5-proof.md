# Phase 12 RP1 Ethernet GPIO32 PHY-Reset Read-Only Preflight Pi 5 Proof

Task id: phase12-rp1-ethernet-gpio32-phy-reset-readonly-preflight-pi5-proof-20260610
Status: accepted
Owner: worker
Classification:
rp1-ethernet-gpio32-phy-reset-readonly-preflight-visible-with-control
Evidence level: image/archive inspection, lab-controller API, serial hardware
boot/output, TFTP/capture evidence, capture-chain-v4 replay, and restore
proof. No GPIO/RIO/pad/MMIO write, PHY reset assertion/deassertion, MDIO
transaction, packet I/O, networking, sockets, SSH, Phase 12.2, or phase
transition was performed or accepted.

## Goal

Run the serialized Pi 5 proof for the accepted read-only GPIO32 / ETH_RST_N
PHY-reset preflight report and paired no-GPIO/no-Ethernet control.

## Findings

- fixed: added the missing candidate/control boot scenarios and archive/review
  helpers for the accepted GPIO32 PHY-reset read-only preflight proof boundary.
- fixed: candidate archive review passed with nonce
  gpio32-phy-reset-candidate-20260610T1608Z, archive sha256
  1bb11f9024e3652b6f99214c578990fc135a14b13606884bf9df2d316e37eb89,
  kernel_2712.img sha256
  17ca70a5a5429e8fe2aa35a74e298e28307a8d9d62a467601a44d3a084ef97bc,
  and kernel_2712.img size 49528 bytes.
- fixed: control archive review passed with nonce
  gpio32-phy-reset-control-20260610T1608Z, archive sha256
  264ed64e4629ad2f6477b2af6cc8c9b900ba7b0987cbd1d174a56592584e729f,
  kernel_2712.img sha256
  a68ebcf7a7e0c7c2165a110e60d336d5453d1c49d8df9406223f5f4260394673,
  and kernel_2712.img size 48688 bytes.
- fixed: candidate capture-chain-v4 joined selected-tree identity
  25933d095429b5b91ab2185caa1e5c2ce586346452d838a853dbebacea5c4ba7,
  two expected TFTP fetches of da591740/kernel_2712.img at 49528 bytes,
  run-unique serial nonce freshness, final pre-restore identity, and restore
  proof.
- fixed: candidate serial output retained the accepted read-only GPIO32
  PHY-reset preflight fields: source/report contract ids, accepted input
  frontier, rp1_eth / phy1 identity, rp1_gpio line 32 / ETH_RST_N route,
  active-low logical assertion/deassertion mapping, 5 ms duration, Linux MACB
  MDIO reset hook relationship, Phase 11 GPIO constraints, future
  write/restore invariants, rejected claims, retained risks, and
  classification
  rp1-ethernet-gpio32-phy-reset-readonly-preflight-report-visible.
- fixed: paired control capture-chain-v4 joined selected-tree identity
  ddd753ab2040cdadde6a6b665b24a96886db2377be76bac006806ea035907bda,
  two expected TFTP fetches at 48688 bytes, run-unique serial freshness,
  final pre-restore identity, and restore proof.
- fixed: paired control serial output used the same report path while
  withholding candidate-only GPIO32/ETH_RST_N/PHY-reset facts and reporting
  classification no-gpio-no-ethernet-rp1-ethernet-gpio32-phy-reset-control.
- fixed: final lab boot tree was restored to
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 with
  effective kernel kernel_2712.img and da591740/kernel_2712.img at 104136
  bytes.
- not-an-issue: the proof used saturated-cursor direct serial reads; freshness
  was proven by run-unique capture nonce absence before power and presence
  after power in both candidate and control captures.
- deferred: GPIO ownership, PHY reset assertion/deassertion, MDIO/PHY
  ownership, write/restore GPIO state, Ethernet driver readiness, packet I/O,
  networking, sockets, SSH, Phase 12.2, and phase transition remain future or
  rejected scope.

No findings were removed.

## Accepted Boundary

The accepted hardware frontier is a read-only GPIO32 PHY-reset preflight
visibility/control proof. The candidate proved the Pi 5 can publish and boot a
Talos artifact that emits the accepted source-backed GPIO32 / ETH_RST_N
preflight report over serial. The paired no-GPIO/no-Ethernet control proved the
same serial/reporting path while constructing no GPIO32, ETH_RST_N, PHY reset,
MDIO, Ethernet MMIO, clock, DMA, descriptor, interrupt, packet, socket, SSH, or
phase-transition target.

This proof does not accept GPIO ownership, PHY reset assertion/deassertion,
MDIO/PHY ownership, RP1 GPIO/RIO/pad/MMIO writes, Ethernet driver readiness,
DMA, descriptors, interrupts, packet I/O, networking, sockets, SSH, Phase 12.2,
or a phase transition.

## Evidence

- Candidate archive review:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-readonly-preflight-pi5-proof/archive-review/candidate-review.txt.
- Control archive review:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-readonly-preflight-pi5-proof/archive-review/control-review.txt.
- Candidate capture:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-readonly-preflight-pi5-proof/candidate-run/.
- Control capture:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-readonly-preflight-pi5-proof/control-run/.
- Classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-readonly-preflight-pi5-proof/classification.json.
- Evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-readonly-preflight-pi5-proof/evidence-map.json.
- Capture summary:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-readonly-preflight-pi5-proof/capture-summary.json.
- Source changes:
  build.rs, src/main.rs, src/target/rpi5.rs, and
  scripts/rpi5-rp1-ethernet-gpio32-phy-reset-preflight-*.

## Validation

- Static archive/image review: candidate and control review scripts passed.
- Lab-controller API: restore snapshot
  pre-gpio32-phy-reset-preflight-proof-20260610T1612Z created; candidate and
  control archives published; final status captured after restore.
- Serial hardware boot/output: candidate and control serial outputs retained
  run-unique capture nonce markers and expected report lines.
- TFTP evidence: candidate and control each observed two expected
  da591740/kernel_2712.img fetches with matching byte counts.
- Capture replay: rpi5-proof-identity-join-v4-check.sh returned
  capture-chain-v4-ready for candidate and control with no rejection reasons.
- Formatting/build: cargo fmt --all -- --check and
  cargo -Zjson-target-spec check --quiet --target
  targets/aarch64-talos-rpi5-bcm2712.json passed.
- JSON validation: jq empty on task-owned evidence-map/classification/capture
  summary JSON passed.
- Diff check: git diff --check passed.
- Documentation build: /home/node/.cargo/bin/mdbook build passed.
- Staged diff check: git diff --cached --check passed before commit.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- hardwareTestLock acquisition/release/restored fields identify this task and
  final restore evidence: satisfied in supervisor state and task evidence.
- Candidate and control each join selected-tree identity, expected TFTP
  fetches, fresh serial marker/cursor evidence, final pre-restore identity,
  and restore proof: satisfied by capture-chain-v4-ready evidence.
- Candidate serial output contains accepted read-only GPIO32 PHY-reset
  preflight report fields; control output proves same path while withholding
  candidate-only facts: satisfied.
- Inconclusive candidate triage: not triggered; both candidate and control
  captures were decisive.
- Classification rejects GPIO ownership, PHY reset assertion/deassertion,
  MDIO/PHY ownership, packet I/O, networking, sockets, SSH, Phase 12.2, and
  phase transition: satisfied.
- Accepted proof is committed before proof closeout starts: satisfied after
  this task commit.

## Next Action

Mechanically promote
phase12-rp1-ethernet-gpio32-phy-reset-readonly-preflight-proof-closeout-20260610
on the next worker wake. The closeout must reconcile this proof and must not
select a write-backed PHY reset task unless that checkpoint makes it
mechanically objective.
