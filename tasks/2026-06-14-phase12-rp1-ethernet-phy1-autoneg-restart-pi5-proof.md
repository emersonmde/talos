# Phase 12 RP1 Ethernet PHY1 Autoneg Restart Pi 5 Proof

Task id: phase12-rp1-ethernet-phy1-autoneg-restart-pi5-proof-20260614

Status: accepted

Classification: phy1-autoneg-restart-capture-staging-blocked

Evidence level: static archive/image review, fmt/lint/typecheck, unit tests,
lab-controller API, serial/TFTP hardware capture attempt, capture-chain-v4
replay, boot-staging identity replay, and restore proof. No accepted PHY1
autonegotiation restart hardware result was produced.

## Goal

Run the guarded PHY1 BMCR autonegotiation-restart candidate and paired control
on Pi 5 only if the capture chain can prove selected-tree identity, serial
freshness, same-power-cycle TFTP byte agreement, final pre-restore identity,
and restore.

## Scope Performed

- Added task-owned candidate/control image, boot-tree, archive, and static
  review scripts for the accepted guard-core scenarios.
- Built candidate and control archives with run-unique nonces.
- Candidate static review passed: archive SHA-256
  f255179a044954dba9881995a47fe3eb6aacaca4e2d94f1884bead1ac0d442e9,
  kernel SHA-256
  1a1a00c14a2c91045f1941c284a375e7e70ac7866e465e7a7b34e827acf2650f,
  and kernel_2712.img size 52360 bytes.
- Control static review passed: archive SHA-256
  5e16142075994af6142a4715ea26cfb751425b9657db0c0c7da3bf487894d763,
  kernel SHA-256
  ea20c6cebc44dc33671ab785fafedf7b7f0c8a7786f4c4c3c8aa0613af5a04dd,
  and kernel_2712.img size 49864 bytes.
- Candidate publication was visible through /boot/files as selected tree
  6bf7d36a3f07426f450fd8a4def73b9cc8bbbc5b730ba50503fd0ee8f41609e1
  with expected da591740/kernel_2712.img size 52360 bytes.
- Candidate pre-power serial drain reached empty-read-before-power after five
  attempts and retained fresh cursors.
- Candidate same-power-cycle evidence was rejected: TFTP served baseline-sized
  104136-byte da591740/kernel_2712.img fetches, final pre-restore identity was
  baseline tree a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10,
  and the run-unique candidate marker was absent.
- Known-good baseline triage restored the baseline snapshot and power-cycled
  once; power succeeded, final identity stayed baseline, but the stable
  same-cursor TFTP delta had zero events. A candidate rerun was not attempted
  because the control did not prove fresh TFTP capture was currently observable.
- The lab was restored to baseline tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Findings

- fixed: added missing task-owned archive/image/review helpers for the
  candidate and control guard-core scenarios.
- fixed: candidate and control static archive reviews verify run-unique nonce
  strings, report-surface strings, and forbidden runtime claim strings.
- blocked: candidate hardware evidence failed the capture-chain-v4 and
  boot-staging identity gates before any autonegotiation runtime claim could be
  accepted.
- blocked: known-good baseline triage did not produce a fresh TFTP delta, so a
  same-shaped candidate rerun would not create decisive evidence.
- deferred: paired control hardware publication/run remains deferred until the
  closeout/supervisor selects a capture-layer recovery path.
- not-an-issue: no GPIO32/PHY reset, MACB write, NCR write, link forcing,
  packet I/O, DMA/descriptors, interrupts, networking, sockets, SSH,
  Phase 12.2, or phase transition was introduced.
- removed: no obsolete source, docs, or evidence was removed.

## Candidate Blocker

~~~text
classification=capture-staging-blocked
selected_tree=6bf7d36a3f07426f450fd8a4def73b9cc8bbbc5b730ba50503fd0ee8f41609e1
expected_fetch=da591740/kernel_2712.img
expected_fetch_bytes=52360
tftp_expected_fetch_count=4
tftp_expected_fetch_byte_match_count=0
tftp_expected_fetch_bytes_seen=104136,104136,104136,104136
final_pre_restore_tree=a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
serial_marker_present=false
rejection_reasons=run-unique-capture-nonce-not-present-after-power,required-marker-not-present-after-power,tftp-expected-fetch-byte-mismatch,final-pre-restore-selected-tree-mismatch,final-pre-restore-expected-fetch-byte-mismatch
~~~

## Known-Good Triage

~~~text
classification=known-good-baseline-no-fresh-tftp-after-power
pre_tree_hash=a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
pre_fetch_bytes=104136
final_tree_hash=a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
final_fetch_bytes=104136
tftp_stable=true
tftp_expected_fetch_count=0
~~~

## Boundary

Accepted: the proof task reached a precise capture-staging blocker with
retained candidate identity mismatch evidence, known-good baseline triage
evidence, and restore proof.

Not accepted: runtime PHY1 BMCR write evidence, autonegotiation restart
success/failure, link readiness, Ethernet readiness, packet I/O, networking,
sockets, SSH, Phase 12.2, or a phase transition.

## Evidence

- Classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-pi5-proof/classification.json.
- Evidence map:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-pi5-proof/evidence-map.json.
- Candidate archive review:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-pi5-proof/archive-review/candidate-static-review.txt.
- Control archive review:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-pi5-proof/archive-review/control-static-review.txt.
- Candidate capture-chain-v4:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-pi5-proof/candidate-run/v4-check.json.
- Candidate boot-staging identity:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-pi5-proof/candidate-run/boot-staging-identity.json.
- Known-good baseline triage:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-pi5-proof/triage-known-good-baseline/classification.json.

## Validation

- passed: cargo fmt --all -- --check.
- passed: cargo -Zjson-target-spec test --quiet.
- passed: sh -n on task-owned scripts.
- passed: static archive/image review for candidate and control.
- blocked: serialized Pi 5 candidate run reached capture-staging-blocked before
  candidate runtime output could be accepted.
- passed: candidate identity via lab API /boot/files before run.
- passed: fresh serial cursor and empty pre-power serial drain were retained.
- blocked: same-power-cycle TFTP delta served baseline bytes instead of
  selected candidate bytes.
- blocked: capture-chain-v4 replay rejected candidate evidence.
- blocked: boot-staging identity replay rejected candidate evidence.
- passed: known-good baseline power-cycle triage and final identity capture.
- passed: restore proof to baseline tree.
- passed: jq empty on task-owned JSON evidence.
- passed: git diff --check.
- not run: mdbook build because docs/src files were not touched.

## Next Action

Mechanically promote
phase12-rp1-ethernet-phy1-autoneg-restart-closeout-20260614 on the next worker
wake if dependencies remain satisfied. The closeout must reconcile this as a
capture-staging blocker, not as PHY/autoneg runtime evidence.
