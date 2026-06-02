# Phase 10 Pi 5 Local Cat Banner Proof Task

Task: phase10-pi5-local-cat-banner-proof-20260602

Status: blocked-control

## Goal

Carry the accepted bounded `cat /etc/banner.txt` local command to serialized
Raspberry Pi 5 serial hardware evidence.

## Scope

This task added the Pi 5 proof scenario and boot scripts needed to publish the
accepted kernel-backed `cat /etc/banner.txt` command. The command-loop
semantics remain narrow: exact `cat /etc/banner.txt`, descriptor-backed
fd0/stdout, visible `Talos initramfs fixture`, and next-prompt readiness.

Changed files:

- build.rs
- src/main.rs
- src/target/rpi5.rs
- scripts/rpi5-local-cat-banner-image.sh
- scripts/rpi5-local-cat-banner-boot-tree.sh
- tasks/2026-06-02-phase10-pi5-local-cat-banner-proof.md
- tasks/evidence/2026-06-02-pi5-local-cat-banner-proof/

## Evidence

Candidate local1:

- Archive: `target/talos-rpi5-local-cat-banner-local1.tar.gz`
- Archive SHA-256:
  `35937283006c1079df2d95836343c4cd81e54655989e238fea70aa746778feb0`
- Kernel SHA-256:
  `5300184ebc40ac3b5bb44c9c96828f0d4b1c71b2a8f4431593fff8e5394abce3`
- Evidence summary:
  `tasks/evidence/2026-06-02-pi5-local-cat-banner-proof/local1-candidate/proof-result.txt`

The candidate archive passed static archive review and contains the proof label,
cat-banner-observed marker, final classification string, and banner fixture
string. Lab status showed the candidate tree hash
`5535fe57219ff7d2873926ba7b443603dd0ae85d00006a1fe31828087c9345d9`; TFTP
logs showed fresh `da591740/kernel_2712.img` fetches of 107520 bytes.

Retained serial output did not satisfy the acceptance gate. It reached Talos
boot output through `TALOS: dtb memory scan start` and then retained only
NUL/newline bytes. A subsequent lab serial write of `cat /etc/banner.txt`
retained no command response, no descriptor-backed fd0/stdout markers, no
`Talos initramfs fixture` output, no ready-for-next prompt, no
`pi5-local-cat-banner-complete`, and no
`rpi5-local-cat-banner-proof: PASS`.

Known-good control local2:

- Control archive: `target/talos-rpi5-local-literal-echo-local3.tar.gz`
- Control archive SHA-256:
  `7cc63a02e2d5dc68abd2127bad0867bc0c1bd0830f56c7ba74a6efbfe64438f5`
- Evidence summary:
  `tasks/evidence/2026-06-02-pi5-local-cat-banner-proof/local2-known-good-literal-echo-control/control-result.txt`

The previously accepted literal-echo control also fetched over TFTP, but did
not produce retained prompt-responsive serial evidence. It stopped at the same
Talos `dtb memory scan start` boundary plus NUL/newline bytes, and a later
`echo local serial works` write retained no response.

The lab boot tree was restored after the control to the pre-run hash
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.

## Validation

- fmt/lint: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed with 351
  no_std tests.
- image/archive inspection: `scripts/rpi5-archive-review.sh` passed for the
  candidate archive and the known-good control archive.
- image string inspection: the candidate kernel contains the proof label,
  bounded response marker, final classification, and banner fixture string.
- lab-controller API: candidate and control status, publish, power-cycle,
  TFTP, serial, and restore artifacts are retained.
- serial hardware boot/output: candidate and known-good control are
  inconclusive/not responsive; neither produced command-response/PASS evidence.
- restore proof: post-control restore returned the boot tree to pre-run hash
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.

## Blocker

This task is not accepted. The feature candidate has static identity and fresh
TFTP evidence, but the hardware acceptance gate requires retained Pi 5 serial
evidence with the prompt, command response, descriptor-backed markers, visible
banner output, final classification, and PASS line.

Because the previously accepted literal-echo control also failed to produce
prompt-responsive evidence in the same run, the next step requires supervisor
planning around the Pi 5 control/serial responsiveness issue before changing
the cat banner proof strategy or rerunning the unchanged candidate.
