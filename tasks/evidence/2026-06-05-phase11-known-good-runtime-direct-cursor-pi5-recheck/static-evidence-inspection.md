# Static Evidence Inspection

Task id: phase11-known-good-runtime-direct-cursor-pi5-recheck-20260605

Evidence level: static inspection + lab-controller API + serial hardware boot/output.

## Inspection

- Boot identity is consistent across lab-status-before.json, lab-status-pre-restore.json, and lab-status-after-restore.json: tree hash a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 with effective_kernel=kernel_2712.img.
- known-good-serial-cursor.txt contains fresh serial cursor 4096040 captured before the power cycle.
- tftp-cursor-before.txt contains authoritative fresh TFTP cursor 4096953 captured by scripts/rpi5-tftp-cursor.sh before the power cycle.
- known-good-tftp-delta-stable-pre-restore.json is stable and contains 13 events, including two served da591740/kernel_2712.img transfers of 104,136 bytes.
- known-good-runtime-readiness-observe.json reports has_kernel_main=false, has_required_success_marker=false, has_prompt_marker=false, and valid_known_good_talos_readiness=false.
- final-restore.exit is 0, and post-restore status still reports the restored known-good tree.

## Disposition

- fixed: direct-cursor TFTP evidence is retained before restore and no blank cursor was used.
- deferred: valid known-good Talos runtime readiness remains unaccepted because serial readiness markers were absent.
- deferred: RP1 candidate/source work remains blocked until closeout reconciles this blocker.
- removed: no candidate publication, source change, or additional hardware rerun was introduced.
- not-an-issue: nonzero serial readiness helper exit matches the absent-marker classification.
