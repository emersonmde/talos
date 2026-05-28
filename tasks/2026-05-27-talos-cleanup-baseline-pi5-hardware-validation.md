# Talos Cleanup Baseline Pi 5 Hardware Validation

## Task

- Title: cleanup-baseline Pi 5 hardware validation
- Owner: worker
- Date: 2026-05-27
- Milestone: Repository Health, Pi 5 hardware validation
- Scope: serialized physical Pi 5 validation for cleanup baseline 515a94b

## Goal

Validate the committed obsolete-diagnostic cleanup baseline on Raspberry Pi 5
hardware before any senior-engineer review/fix pass starts.

## Acceptance Criteria

- A Pi 5 hardware run from the post-cleanup baseline reaches a PASS or
  classification line for a retained active proof.
- Serial evidence includes human-readable Talos kernel output, not only
  Raspberry Pi firmware logs.
- Evidence identifies the exact candidate commit, archive SHA256, kernel
  SHA256, kernel size, serial cursor, TFTP delta, and restore status.
- If hardware fails, review pass 1 remains blocked until triage resolves or
  Matthew explicitly accepts proceeding.

## Current Result

Status: accepted.

The paused inconclusive run was resolved by a clean known-good control and a
fresh cleanup-baseline candidate rerun. The control archive from the previously
accepted load-balancing proof reached
`classification=pi5-load-balancing-complete` from serial cursor 2217200, and
the cleanup-baseline candidate reached the same classification from serial
cursor 2230226 after fresh 95,064-byte TFTP fetches.

## Evidence So Far

Evidence directory:
`tasks/evidence/2026-05-27-cleanup-baseline-pi5-hardware-validation/`.

- Candidate identity: commit
  `515a94b30e326844c1596d2ef7d9b093a357f1f0`, archive
  `target/talos-rpi5-cleanup-baseline-load-balancing-boot.tar.gz`.
- Image/archive inspection: `scripts/rpi5-archive-review.sh` passed for
  archive SHA256
  `2273a90146469638c687c70a7b383bb517a1c2b1aa31d1c54d562ba2fb3594b5`;
  kernel SHA256
  `901bf299a080fdb7ece60281b6cea237e98fdc44129e0f1da0ed81370b06e98c`;
  kernel size 95,064 bytes.
- local1: post-publish status identified the 95,064-byte candidate archive;
  late cursor-valid serial observe from cursor 2204785 reached
  `classification=pi5-load-balancing-complete` and
  `rpi5-load-balancing: PASS`.
- local3: candidate TFTP delta before restore recorded fresh
  `da591740/kernel_2712.img` fetches with bytes=95064 and current candidate
  status, but cursor-valid serial observation did not capture a new PASS.
- local4: candidate rerun again recorded fresh TFTP fetches with bytes=95064
  but serial from cursor 2217194 contained only NUL/newline.
- local5-control: restored the local3 pre-run snapshot with
  `da591740/kernel_2712.img` bytes=82045 and power-cycled as a control; serial
  from cursor 2217196 also contained only NUL/newline.
- local6-control: published the accepted prior
  `target/talos-rpi5-load-balancing-boot.tar.gz` control archive
  (archive SHA256
  `e7d4c80740bac203e9516e68baef29e9d197a8e760d233301cb209605a38d119`,
  kernel SHA256
  `ceb75685864c32ed3d5a028c877d6a1d911892d4cbf14b36536d266206d7fecd`,
  kernel size 95,128 bytes). Fresh TFTP delta recorded
  `da591740/kernel_2712.img` served at 95,128 bytes, and serial cursor 2217200
  reached `classification=pi5-load-balancing-complete` and
  `rpi5-load-balancing: PASS`.
- local7-candidate: published the cleanup-baseline candidate archive
  (archive SHA256
  `2273a90146469638c687c70a7b383bb517a1c2b1aa31d1c54d562ba2fb3594b5`,
  kernel SHA256
  `901bf299a080fdb7ece60281b6cea237e98fdc44129e0f1da0ed81370b06e98c`,
  kernel size 95,064 bytes). Fresh TFTP delta recorded
  `da591740/kernel_2712.img` served at 95,064 bytes, and serial cursor 2230226
  reached `classification=pi5-load-balancing-complete` and
  `rpi5-load-balancing: PASS`.
- Restore/lab state: the final restore returned the lab to snapshot
  `pre-cleanup-baseline-local6control-20260528T012920Z`, which reports the
  82,045-byte pre-run boot tree.

## Next Action

Accepted. Senior-engineer review pass 1 may start only when the supervisor
ready-marks the next queued task.
