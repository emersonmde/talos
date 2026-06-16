# Phase 12.1 RP1 Ethernet BootInfo Report Serial Visibility Core

Task:
phase12-rp1-ethernet-bootinfo-report-serial-visibility-core-20260616.

Status: accepted

Classification:
bootinfo-report-serial-visibility-core-local-static.

Evidence level: static/source inspection, local/static no-std unit tests,
candidate/control archive static artifact review, script syntax validation,
JSON evidence validation, docs build, and diff checks. No Pi 5 hardware run,
boot archive publication, lab mutation, hardwareTestLock acquisition,
power-cycle, TFTP/serial capture, volatile Ethernet access, BCM54213PE
register retry, MII_CTRL1000/MII_STAT1000 read, GPIO32 event clear/reset
recovery, BMCR write, Broadcom shadow/MMD/aux access, interrupt ownership,
PHY/MAC configuration, packet I/O, networking, SSH, Phase 12.2, or phase
transition was performed.

## Goal

Implement the local/static dual-stage visibility discriminator that separates
earliest Rust-entry serial visibility from post-BootInfo/report-path serial
visibility before any Ethernet or MDIO behavior.

## Scope Performed

- Reviewed the accepted boot-transport sentinel and kernel-entry serial beacon
  closeouts. The unresolved boundary is post-earliest-entry BootInfo/report
  visibility, not generic selected-tree TFTP transport or earliest serial
  capture.
- Added paired boot scenarios:
  rpi5_rp1_ethernet_bootinfo_report_serial_visibility_candidate and
  rpi5_rp1_ethernet_bootinfo_report_serial_visibility_earliest_only_control.
- Added the earliest-entry marker in boot::rpi5::kernel_main before
  target::services and report_boot_identity consume BootInfo.
- Added the post-BootInfo/report-path marker immediately after
  report_boot_identity reports BootInfo and service metadata.
- Added local/static evidence and validators in src/rp1_ethernet.rs so
  marker-shape drift and forbidden Ethernet/MDIO claims fail closed.
- Added candidate/control image, boot-tree, archive, and archive-review
  helpers for the later serialized Pi 5 proof.
- Registered the new boot scenarios in build.rs to keep Rust check-cfg
  warning-free.

## Findings

- fixed: candidate and control both emit
  bootinfo-report-visibility-earliest-entry-marker before BootInfo parsing.
- fixed: only the candidate emits
  bootinfo-report-visibility-post-bootinfo-report-path-marker after
  report_boot_identity, allowing hardware evidence to distinguish
  earliest-only from both-markers-observed.
- fixed: the paired control halts before BootInfo/report-path execution after
  the earliest marker, preserving a same-shape capture control.
- fixed: static review proved the candidate archive contains the run nonce,
  earliest marker, post-BootInfo marker, contract ids, selected discriminator,
  and no forbidden BCM54213PE/MDIO/MAN/MACB/GPIO32/PHY target facts.
- fixed: static review proved the control archive contains the run nonce and
  earliest marker but not the post-BootInfo marker or candidate
  classification.
- fixed: build.rs now registers the new talos_boot_scenario values, so the
  target build no longer relies on unregistered cfg names.
- deferred: Pi 5 publication, hardwareTestLock acquisition, TFTP/serial
  capture, known-good control, candidate rerun, final identity, and restore
  proof remain deferred to
  phase12-rp1-ethernet-bootinfo-report-serial-visibility-pi5-proof-20260616.
- rejected: BCM54213PE register values, link readiness, Ethernet readiness,
  GPIO32/PHY reset ownership, BMCR writes, Broadcom shadow/MMD/aux access,
  interrupt ownership, broad PHY/MAC configuration, packet I/O, networking,
  sockets, SSH, Phase 12.2, and phase transition remain rejected.
- removed: no task-owned source, helper, docs, or evidence files were removed.
- not-an-issue: the helper artifacts are retained under target/ as generated
  local/static evidence; the committed durable evidence records their hashes
  and static review results rather than committing boot archives.

## Static Artifact Review

Candidate:

- Archive:
  target/phase12-bootinfo-report-serial-visibility/candidate.tar.gz.
- Archive SHA-256:
  540b6b32f56a841caee24ade1f8ae26724dc46a2d5f72a7b94a479a879b61cff.
- Kernel SHA-256:
  3e0daad3093cc22943bebc29cdf09390381abd201fb5320d7fca607305939b9c.
- Kernel size: 71,040 bytes.
- Capture nonce: bootinfo-report-core-candidate-20260616.
- Review result:
  forbidden_bootinfo_report_visibility_candidate_runtime_strings_absent=true.

Control:

- Archive:
  target/phase12-bootinfo-report-serial-visibility/control.tar.gz.
- Archive SHA-256:
  576ba0e576223970f0475bd9290c2b1fcad7b0ac1f27119a3221e28bf8c8686c.
- Kernel SHA-256:
  3f51358c50b7fd695c673c13ff61a5d80b3518b60d0054de6eb4482f3615b584.
- Kernel size: 55,016 bytes.
- Capture nonce: bootinfo-report-core-control-20260616.
- Review result:
  forbidden_bootinfo_report_visibility_control_runtime_strings_absent=true.

## Hardware Boundary Selected

The queued serialized Pi 5 proof is mechanically selected as the next boundary
only after this task is accepted and committed. It must classify the hardware
result as one of:

- no-selected-tftp
- no-earliest-marker
- earliest-marker-only
- both-markers-observed
- known-good-control-failed
- staging-capture-inconclusive
- restore-failed

The hardware proof must not accept Ethernet behavior, register values, link
readiness, packet I/O, networking, SSH, Phase 12.2, or phase transition from
marker visibility.

## Evidence

- Classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bootinfo-report-serial-visibility-core/classification.json.
- Evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bootinfo-report-serial-visibility-core/evidence-map.json.
- Static artifact review:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bootinfo-report-serial-visibility-core/static-artifact-review.json.

## Validation

- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test --quiet
  rp1_ethernet_bootinfo_report_serial_visibility: pass.
- sh -n on all new bootinfo-report serial visibility shell helpers: pass.
- candidate/control archive static review: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass because docs/src files were touched.
- git diff --cached --check before commit: pass.

## Acceptance Check

- Findings recorded with dispositions: satisfied.
- Earliest-entry and post-BootInfo/report-path marker positions are named:
  satisfied.
- Candidate/control evidence distinguishes no selected TFTP, no earliest
  marker, earliest marker only, and both markers observed for the later
  hardware task: satisfied by paired scenario shape and static review.
- Static artifact review proves marker presence and forbidden target-fact
  absence: satisfied.
- Validators fail closed for forbidden target facts and claims: satisfied.
- Next boundary is explicit: satisfied by selecting the dependency-gated Pi 5
  proof task.
- Code, docs, task, evidence, and state updates are committed before hardware
  proof starts: satisfied once state is updated after this commit.

## Next Action

After this commit and state acceptance, mechanically promote
phase12-rp1-ethernet-bootinfo-report-serial-visibility-pi5-proof-20260616 if
dependencies remain satisfied, the hardware lock is unlocked/restored, and no
conflicting changes are present. Do not run hardware from this core task.
