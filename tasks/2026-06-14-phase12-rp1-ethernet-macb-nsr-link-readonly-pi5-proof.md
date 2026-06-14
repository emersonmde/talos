# Phase 12 RP1 Ethernet MACB NSR_LINK Read-Only Pi 5 Proof

Task id: phase12-rp1-ethernet-macb-nsr-link-readonly-pi5-proof-20260614

Status: accepted

Classification: macb-nsr-link-readonly-link-clear

Evidence level: static archive/image review, fmt/lint/typecheck, unit tests, lab-controller API, serial hardware boot/output, stable same-cursor TFTP delta, capture-chain-v4 replay, boot-staging identity replay, and restore proof.

## Goal

Prove the smallest passive MAC-side link discriminator after the accepted PHY1 link-not-ready result: read-only MACB_NSR at observed-window address 0x1c00100008, decoding NSR_LINK bit 0, with a paired no-MMIO/no-Ethernet control.

## Scope Performed

- Added candidate and paired no-MMIO/no-Ethernet control boot scenarios plus task-owned image, boot-tree, archive, and static review scripts.
- Candidate volatile-read only observed-window MACB_MID context and MACB_NSR at 0x1c00100008, decoded bit 0, and printed the accepted PHY1/GPIO32 frontier as evidence context.
- Control used the same reporting surface while constructing no MACB_NSR target and performing no Ethernet volatile load/store.
- Acquired hardwareTestLock before lab publication, power-cycle, serial/TFTP capture, or restore.
- Ran serialized candidate and control Pi 5 captures with run-unique serial nonces, stable same-cursor TFTP deltas, final pre-restore identity, restore proof, capture-chain-v4 replay, and boot-staging identity replay.
- Restored the lab to the pre-run baseline tree a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Findings

- fixed: candidate archive review passed with archive SHA-256 88cfaf976d346ed0f248c05246dd051928c7b940052325a372d81c43b70887c2, kernel SHA-256 141e5da1e899502137e2838b11bc7c9740d75d2dd185709647338238c626a377, and kernel_2712.img size 49728 bytes.
- fixed: control archive review passed with archive SHA-256 9da94e4d2f98297a87269c2822b6ea02bd52343930ae588fbfc9477a75b31d79, kernel SHA-256 06babf44184aed7bed0772507e77d7dcdbfdb43a6d756a285f0d77e5ec010086, and kernel_2712.img size 49480 bytes.
- fixed: candidate capture-chain-v4 and boot-staging identity checks passed. The selected tree 937e30a34797c672f393e6cd7f4c4b12c6b1a0ea1e5b97c8c6afbbc8788a3522 served da591740/kernel_2712.img twice at 49728 bytes and remained staged through final pre-restore identity.
- fixed: control capture-chain-v4 and boot-staging identity checks passed. The selected tree ff82cf02034aa877cf5907a8456be504e966109a6a9ac51992e23a9b79457c70 served da591740/kernel_2712.img twice at 49480 bytes and remained staged through final pre-restore identity.
- fixed: candidate serial output reported MACB_MID context 0x70109, MACB_NSR raw 0x6, NSR_LINK=false, and classification macb-nsr-link-readonly-link-clear.
- fixed: paired control serial output withheld candidate-only target/raw/decode fields, constructed no MACB_NSR target, performed no Ethernet volatile load/store, and classified as no-mmio-no-ethernet-macb-nsr-link-control.
- deferred: PHY reset/GPIO32 ownership, PHY configuration, autonegotiation restart, link forcing, MACB write ownership, broad MDIO/PHY ownership, Ethernet driver behavior, interrupts, DMA/descriptors, packet I/O, networking, sockets, SSH, Phase 12.2, and phase transition remain future explicit tasks.
- removed: no source code, docs, or evidence was removed.
- not-an-issue: MACB_NSR_LINK is only a MAC-side comparator at the selected instant; the accepted PHY1 BMSR link-not-ready frontier remains in force.

## Candidate Result

~~~text
classification=macb-nsr-link-readonly-link-clear
capture-chain-v4=capture-chain-v4-ready
boot-staging-identity=selected-tree-identity-ready
selected_tree=937e30a34797c672f393e6cd7f4c4b12c6b1a0ea1e5b97c8c6afbbc8788a3522
expected_fetch=da591740/kernel_2712.img
expected_fetch_bytes=49728
expected_fetch_count=2
expected_fetch_byte_match_count=2
final_pre_restore_tree=937e30a34797c672f393e6cd7f4c4b12c6b1a0ea1e5b97c8c6afbbc8788a3522
macb_mid_context_raw=0x70109
macb_nsr_target=0x1c00100008
macb_nsr_raw=0x6
macb_nsr_link=false
macb_read_performed=true
macb_write_performed=false
mdio_target_constructed=false
man_frame_constructed=false
accepted_phy1_bmcr=0x1000
accepted_phy1_bmsr_first=0x7949
accepted_phy1_bmsr_second=0x7949
accepted_phy1_anar=0x01e1
accepted_phy1_anlpar=0x0000
pre_power_nonce_occurrences=0
post_power_nonce_occurrences=23
~~~

## Control Result

~~~text
classification=no-mmio-no-ethernet-macb-nsr-link-control
capture-chain-v4=capture-chain-v4-ready
boot-staging-identity=selected-tree-identity-ready
selected_tree=ff82cf02034aa877cf5907a8456be504e966109a6a9ac51992e23a9b79457c70
expected_fetch=da591740/kernel_2712.img
expected_fetch_bytes=49480
expected_fetch_count=2
expected_fetch_byte_match_count=2
final_pre_restore_tree=ff82cf02034aa877cf5907a8456be504e966109a6a9ac51992e23a9b79457c70
macb_nsr_target=not-constructed
macb_nsr_raw=withheld
macb_nsr_link=withheld
macb_read_performed=false
macb_write_performed=false
mdio_target_constructed=false
man_frame_constructed=false
pre_power_nonce_occurrences=0
post_power_nonce_occurrences=23
~~~

## Boundary

Accepted: a read-only MACB_NSR bit-0 observation at 0x1c00100008 under capture-chain-v4, boot-staging identity, stable same-cursor TFTP byte agreement, final pre-restore identity, serial freshness, evidence-consistency, and restore proof. The observed result is MACB_NSR_LINK clear.

Not accepted: link recovery, Ethernet readiness, MACB writes, MDIO/PHY access, PHY configuration writes, BMCR writes, autonegotiation restart, link forcing, PHY reset/GPIO32 action, DMA/descriptors, interrupts, packet I/O, networking, sockets, SSH, Phase 12.2, or a phase transition.

## Evidence

- Classification: tasks/evidence/2026-06-14-phase12-rp1-ethernet-macb-nsr-link-readonly-pi5-proof/classification.json.
- Evidence map: tasks/evidence/2026-06-14-phase12-rp1-ethernet-macb-nsr-link-readonly-pi5-proof/evidence-map.json.
- Capture summary: tasks/evidence/2026-06-14-phase12-rp1-ethernet-macb-nsr-link-readonly-pi5-proof/capture-summary.json.
- Candidate archive review: tasks/evidence/2026-06-14-phase12-rp1-ethernet-macb-nsr-link-readonly-pi5-proof/archive-review/candidate-static-review.txt.
- Control archive review: tasks/evidence/2026-06-14-phase12-rp1-ethernet-macb-nsr-link-readonly-pi5-proof/archive-review/control-static-review.txt.
- Candidate capture-chain-v4: tasks/evidence/2026-06-14-phase12-rp1-ethernet-macb-nsr-link-readonly-pi5-proof/candidate-run/v4-check.json.
- Control capture-chain-v4: tasks/evidence/2026-06-14-phase12-rp1-ethernet-macb-nsr-link-readonly-pi5-proof/control-run/v4-check.json.
- Candidate boot-staging identity: tasks/evidence/2026-06-14-phase12-rp1-ethernet-macb-nsr-link-readonly-pi5-proof/candidate-run/boot-staging-identity.json.
- Control boot-staging identity: tasks/evidence/2026-06-14-phase12-rp1-ethernet-macb-nsr-link-readonly-pi5-proof/control-run/boot-staging-identity.json.
- Evidence consistency guard: tasks/evidence/2026-06-14-phase12-rp1-ethernet-macb-nsr-link-readonly-pi5-proof/evidence-consistency-guard.json.
- Final restored lab status: tasks/evidence/2026-06-14-phase12-rp1-ethernet-macb-nsr-link-readonly-pi5-proof/final-lab-status.json.

## Validation

- formatting: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed with 508 tests.
- script syntax: sh -n on task-owned scripts passed.
- static archive/image review: candidate and control passed.
- lab-controller API: snapshot creation, publication, power cycle, serial, TFTP, final identity, restore, and post-restore identity retained.
- serial hardware boot/output: candidate/control nonce-bearing markers were absent before power and present after power.
- stable TFTP delta: candidate/control each observed two matching expected da591740/kernel_2712.img fetches.
- capture-chain-v4 replay: candidate/control classified capture-chain-v4-ready.
- boot-staging identity replay: candidate/control classified selected-tree-identity-ready.
- evidence-consistency guard: classified evidence-consistency-ready.
- JSON validation: jq empty passed over task-owned JSON evidence.
- diff check: git diff --check passed.
- documentation build: mdbook build passed after docs updates.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Candidate performs only read-only volatile 32-bit MACB_NSR observation at 0x1c00100008 and decodes NSR_LINK bit 0: satisfied.
- Control constructs no MACB_NSR target, performs no Ethernet volatile load/store, and withholds candidate-only target/raw/decode/result fields: satisfied.
- Candidate output records contract id, accepted PHY1 BMCR/BMSR/ANAR/ANLPAR, corrected-target MDIO boundary, and retained GPIO32 blocker labels: satisfied.
- No MACB write, MDIO/PHY access, PHY configuration, BMCR write, autonegotiation restart, link forcing, GPIO32/PHY reset action, DMA, interrupt, packet I/O, networking, sockets, SSH, Phase 12.2, or phase transition is introduced or accepted: satisfied.
- Candidate/control evidence passes capture-chain-v4, boot-staging identity, same-power-cycle TFTP byte agreement, serial freshness, final pre-restore identity, and restore proof: satisfied.
- Classification is an allowed MACB_NSR_LINK clear result and paired no-MMIO control: satisfied.

## Next Action

Mechanically promote phase12-rp1-ethernet-macb-nsr-link-readonly-closeout-20260614 on the next worker wake if dependencies remain satisfied. Do not start PHY configuration, reset/GPIO32 action, packet I/O, networking, SSH, Phase 12.2, or a phase transition directly from this proof.
