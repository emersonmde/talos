# Phase 12 RP1 Ethernet Lab TFTP Root Minimal Sentinel Pi 5 Proof

Task id:
phase12-rp1-ethernet-lab-tftp-root-minimal-sentinel-pi5-proof-20260611

Status: accepted

Classification: minimal-sentinel-served-root-proof-accepted

Evidence level: static archive review, image/archive inspection,
lab-controller API, serial hardware boot/output, stable same-cursor TFTP delta,
capture-chain-v4 replay, staging identity gate, and restore evidence.

## Goal

Run the operator-selected minimal Pi 5 sentinel proof using hardware TFTP fetch
logs as the read-only served-root measurement before any MDIO/register-vector
retry is considered.

## Scope Performed

- Built one no-MDIO/no-Ethernet staging sentinel archive from the accepted Pi 5
  boot source with run-unique nonce
  minimal-sentinel-candidate-20260614T0127Z.
- Statically reviewed the archive before publication.
- Acquired the hardware lock before lab publication, power action, serial
  capture, TFTP observation, or restore.
- Published the sentinel archive, power-cycled the Pi 5, captured serial and
  stable same-cursor TFTP evidence before restore, then restored the pre-run
  baseline snapshot.
- Replayed the retained evidence through capture-chain-v4 and the staging
  identity gate.

## Findings

- fixed: Matthew's operator resolution selected Pi hardware TFTP fetches as the
  served-root measurement. This proof records that contract as a single
  minimal sentinel run instead of requiring a separate served-root endpoint.
- fixed: archive review passed. The sentinel archive SHA-256 was
  00f1dcb02a8e7aa9e174dd81f636bb9a116f321bc6eee360d751c7c471d1ef54; the
  kernel SHA-256 was
  78c944f115e1786d1584c15e3676d2d2198244515aac90061566aa84f32cdc54;
  kernel_2712.img was 47,832 bytes.
- fixed: capture-chain-v4 classified the run capture-chain-v4-ready. The
  selected tree was
  5dd6afef125a27bbb4e76423fbd189fe1dc020bc9cf2186e42bba7eae5581441.
- fixed: the stable same-cursor TFTP delta observed two served
  da591740/kernel_2712.img fetches, both at the expected 47,832 bytes, before
  any restore.
- fixed: final pre-restore identity stayed on the selected tree and restore
  returned the lab to baseline tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
  with kernel_2712.img at 104,136 bytes.
- fixed: serial freshness was proven by nonce differential: the nonce was
  absent before power and present 84 times after power.
- deferred: MDIO/register-vector retry planning remains blocked on the separate
  root-recovery closeout task; this proof accepts only the served-root/TFTP
  measurement discriminator.
- removed: no stale source or docs were removed.
- not-an-issue: the proof uses the existing no-MDIO staging sentinel scenario;
  the image does not contain MDIO/MAN register-vector code.

## Result

~~~text
classification=minimal-sentinel-served-root-proof-accepted
capture-chain-v4=capture-chain-v4-ready
staging-identity-gate=selected-tree-identity-ready
selected_tree=5dd6afef125a27bbb4e76423fbd189fe1dc020bc9cf2186e42bba7eae5581441
expected_fetch=da591740/kernel_2712.img
expected_fetch_bytes=47832
expected_fetch_count=2
expected_fetch_byte_match_count=2
final_pre_restore_tree=5dd6afef125a27bbb4e76423fbd189fe1dc020bc9cf2186e42bba7eae5581441
post_restore_tree=a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
serial_freshness_ok=true
nonce_token_occurrences=84
pre_power_nonce_occurrences=0
~~~

## Boundary

Accepted: the operator-selected served-root measurement contract and
selected-tree identity for one no-MDIO/no-Ethernet minimal sentinel using Pi
hardware TFTP fetch evidence.

Not accepted: RP1 MDIO register-vector MAN.DATA values, NCR writes, MAN
writes, GPIO32/PHY reset, broad MDIO/PHY ownership, PHY absence, Ethernet
behavior, interrupts, DMA/descriptors, packet I/O, networking, sockets, SSH,
Phase 12.2, or a phase transition.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-lab-tftp-root-minimal-sentinel-pi5-proof/evidence-map.json.
- Capture summary:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-lab-tftp-root-minimal-sentinel-pi5-proof/capture-summary.json.
- Archive review:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-lab-tftp-root-minimal-sentinel-pi5-proof/archive-review/minimal-sentinel-candidate-static-review.txt.
- Capture-chain-v4 replay:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-lab-tftp-root-minimal-sentinel-pi5-proof/minimal-sentinel-run/v4-check.json.
- Staging identity gate:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-lab-tftp-root-minimal-sentinel-pi5-proof/minimal-sentinel-run/staging-identity-gate-output.json.
- Stable TFTP delta:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-lab-tftp-root-minimal-sentinel-pi5-proof/minimal-sentinel-run/tftp-delta-stable-pre-restore.json.
- Serial window:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-lab-tftp-root-minimal-sentinel-pi5-proof/minimal-sentinel-run/serial-observe-window.json.
- Restore proof:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-lab-tftp-root-minimal-sentinel-pi5-proof/minimal-sentinel-run/restore-snapshot.json.

## Validation

- static archive review: passed.
- image/archive inspection: archive SHA, kernel SHA, archive file list, and
  kernel size retained.
- lab-controller API: pre-publication boot files/snapshots/TFTP cursor,
  snapshot creation, archive publication, power cycle, final pre-restore
  identity, restore, and final lab status retained.
- serial hardware boot/output: nonce-bearing sentinel marker present after
  power and absent before power.
- stable TFTP delta: same-cursor pre-restore delta observed two matching
  expected fetches.
- capture-chain-v4 replay: capture-chain-v4-ready.
- staging identity gate: selected-tree-identity-ready.
- JSON validation: jq empty passed over task-owned JSON evidence.
- diff check: git diff --check passed.
- docs validation: not required; docs/src files were not touched.

## Acceptance Check

- Explicit operator resolution selected this proof and named Pi hardware TFTP
  fetches as served-root evidence: satisfied in state before promotion.
- Conditional Pi 5 hardware proof with TFTP, serial, and artifact identity
  evidence: satisfied.
- Hardware lock acquisition/release and boot restore state recorded:
  satisfied.
- No MDIO/MAN code or register-vector retry was run: satisfied.

## Next Action

Mechanically promote
phase12-rp1-ethernet-mdio-register-vector-root-recovery-closeout-20260611 on the
next worker wake if dependencies remain satisfied. Do not start a
register-vector retry, Ethernet behavior, networking, SSH, Phase 12.2, or a
phase transition from this proof directly.
