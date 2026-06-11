# Phase 12 RP1 Ethernet MDIO Register Vector Staging Sentinel Pi 5 Proof

Task id:
phase12-rp1-ethernet-mdio-register-vector-staging-sentinel-pi5-proof-20260611

Status: accepted

Classification: selected-tree-identity-durability-accepted

Evidence level: static archive review, image/archive inspection,
lab-controller API, serial hardware boot/output, stable TFTP delta,
capture-chain-v4 replay, staging identity gate, and restore evidence.

## Goal

Run a serialized Pi 5 no-MDIO staging sentinel proof to determine whether a
candidate-shaped boot archive survives publish-to-power-to-TFTP/final identity
before retrying the register-vector hardware proof.

## Scope Performed

- Added task-owned no-MDIO/no-Ethernet staging sentinel candidate and control
  boot scenarios plus archive/review helpers.
- Built candidate and paired control archives with fresh run-unique nonces.
- Statically reviewed both archives before publication.
- Acquired hardwareTestLock before lab publication, power action, serial
  capture, TFTP observation, or restore.
- Published and captured the candidate sentinel, then restored the baseline
  boot tree.
- Published and captured the control sentinel, then restored the baseline boot
  tree.
- Classified candidate/control evidence through capture-chain-v4 and the
  accepted staging identity gate.

## Findings

- fixed: the task now has explicit staging sentinel candidate/control scenarios
  instead of overloading the register-vector runtime candidate or control.
- fixed: candidate archive review passed with archive SHA-256
  3b8e94338838be7e1a4636e22efb4d700d6c77e487ee6fea248aa72575b95bf5,
  kernel SHA-256
  645557e7b408fce11410a4d57a9a064cacb6e20f60985d95a16c9e9fb9feea53,
  and kernel_2712 size 47,832 bytes.
- fixed: control archive review passed with archive SHA-256
  e1921ccfb604501c1017b93cb9b152b3447f31072e10d4b1d46b7f0465eb79c2,
  kernel SHA-256
  39f3c9a26cce11d4c9e49d48ce5ac482ab7ac6eda89573424fc3da8b83f07d12,
  and kernel_2712 size 47,824 bytes.
- fixed: candidate capture-chain-v4 and staging identity gate both returned
  ready. The candidate selected tree was
  a804458a439c20200a14b8d338341dca427ed7faba6bc7fb2c875049de586cc0 with two
  matching 47,832-byte TFTP fetches, final pre-restore identity on the selected
  tree, restore proof to baseline, and true serial freshness.
- fixed: control capture-chain-v4 and staging identity gate both returned
  ready. The control selected tree was
  9d9b3cdb7b1f230d9cd2bf0c04b7c32dd98b53dd8ec7de77e99860c5b231908d with two
  matching 47,824-byte TFTP fetches, final pre-restore identity on the selected
  tree, restore proof to baseline, and true serial freshness.
- fixed: the final lab state after the proof restored to baseline tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 with
  kernel_2712.img at 104,136 bytes.
- deferred: the guarded register-vector v3 retry remains a separate queued
  hardware proof and must still carry candidate/control staging gate output and
  evidence-consistency guard output before accepting any MAN.DATA values.
- removed: no stale source, docs, or evidence was removed.
- not-an-issue: the candidate/control capture bundles used nonce freshness
  because saturated serial direct-read retained stale data, but each nonce was
  absent before power and present after power.

## Candidate Result

~~~text
capture-chain-v4=capture-chain-v4-ready
staging-identity-gate=selected-tree-identity-ready
selected_tree=a804458a439c20200a14b8d338341dca427ed7faba6bc7fb2c875049de586cc0
expected_fetch=da591740/kernel_2712.img
expected_fetch_bytes=47832
expected_fetch_count=2
expected_fetch_byte_match_count=2
final_pre_restore_tree=a804458a439c20200a14b8d338341dca427ed7faba6bc7fb2c875049de586cc0
post_restore_tree=a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
serial_freshness_ok=true
nonce_token_occurrences=83
pre_power_nonce_occurrences=0
~~~

## Control Result

~~~text
capture-chain-v4=capture-chain-v4-ready
staging-identity-gate=selected-tree-identity-ready
selected_tree=9d9b3cdb7b1f230d9cd2bf0c04b7c32dd98b53dd8ec7de77e99860c5b231908d
expected_fetch=da591740/kernel_2712.img
expected_fetch_bytes=47824
expected_fetch_count=2
expected_fetch_byte_match_count=2
final_pre_restore_tree=9d9b3cdb7b1f230d9cd2bf0c04b7c32dd98b53dd8ec7de77e99860c5b231908d
post_restore_tree=a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
serial_freshness_ok=true
nonce_token_occurrences=84
pre_power_nonce_occurrences=0
~~~

## Boundary

Accepted: selected-tree identity durability for no-MDIO/no-Ethernet staging
sentinel candidate/control archives only.

Not accepted: RP1 MDIO register-vector MAN.DATA values, NCR writes, MAN writes,
GPIO32/PHY reset, broad MDIO/PHY ownership, PHY absence, Ethernet behavior,
interrupts, DMA/descriptors, packet I/O, networking, sockets, SSH, Phase 12.2,
or a phase transition.

## Evidence

- Candidate archive review:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-sentinel-pi5-proof/archive-review/candidate-static-review.txt.
- Control archive review:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-sentinel-pi5-proof/archive-review/control-static-review.txt.
- Candidate capture-chain-v4 replay:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-sentinel-pi5-proof/candidate-run/v4-check.json.
- Control capture-chain-v4 replay:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-sentinel-pi5-proof/control-run/v4-check.json.
- Candidate staging identity gate:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-sentinel-pi5-proof/candidate-run/staging-identity-gate-output.json.
- Control staging identity gate:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-sentinel-pi5-proof/control-run/staging-identity-gate-output.json.
- Classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-sentinel-pi5-proof/classification.json.
- Capture summary:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-sentinel-pi5-proof/capture-summary.json.
- Evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-sentinel-pi5-proof/evidence-map.json.
- Final lab state:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-sentinel-pi5-proof/final-lab-status.json.

## Validation

- static archive review: candidate and control archive reviews passed.
- image/archive inspection: candidate/control archive SHA, kernel SHA, and
  kernel image sizes retained.
- fmt/lint/typecheck: cargo fmt --all -- --check.
- unit tests/QEMU substitute: cargo -Zjson-target-spec test --quiet with local
  QEMU path, 507 tests passed.
- lab-controller API: snapshot, candidate/control archive publication, power
  cycles, serial/TFTP/final identity capture, restores, and final baseline
  status retained.
- serial hardware boot/output: candidate and control nonce-bearing markers were
  present after power and absent before power.
- stable TFTP delta: candidate and control each observed two matching expected
  fetches.
- capture-chain-v4 replay: candidate and control classified
  capture-chain-v4-ready.
- staging identity gate: candidate and control classified
  selected-tree-identity-ready.
- JSON validation: jq empty on task-owned classification, capture-summary,
  evidence-map, candidate/control gate outputs, and candidate/control v4
  checks.
- diff check: git diff --check.
- docs validation: not required; docs/src files were not touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Findings list with dispositions: satisfied.
- HardwareTestLock acquisition/release and boot restore state recorded:
  satisfied.
- Candidate and control each have staging identity gate output with selected
  tree, expected fetch bytes/count, observed TFTP bytes/count, final
  pre-restore tree, restore tree, serial freshness, and classification:
  satisfied.
- Accepted success requires both candidate and control selected-tree identity
  durability through publish-to-power-to-TFTP/final identity plus restore proof:
  satisfied.
- No RP1 MDIO, NCR, MAN, GPIO32/PHY reset, Ethernet, DMA, interrupt, packet
  I/O, networking, sockets, SSH, Phase 12.2, or phase-transition claim is made:
  satisfied.
- Accepted proof committed before closeout starts: satisfied by this task
  commit.

## Next Action

Mechanically promote
phase12-rp1-ethernet-mdio-register-vector-staging-sentinel-closeout-20260611 on
the next worker wake if dependencies remain satisfied. Do not start
register-vector v3, broad MDIO/PHY ownership, Ethernet behavior, networking,
SSH, Phase 12.2, or a phase transition from this proof directly.
