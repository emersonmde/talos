# Supervisor Intervention Required-Before-Resume Record

Task id:
phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v3-after-staging-identity-20260611

Status: blocked pending supervisor planning

Classification: tftp-root-diverges-from-lab-api-selected-tree

## First-Principles Problem Statement

For a selected Pi 5 boot archive, the expected path is:

1. publish one complete boot tree through `PUT /boot/archive`;
2. confirm `GET /status` and `GET /boot/files` report the selected tree,
   `kernel=kernel_2712.img`, and the selected
   `da591740/kernel_2712.img` byte count before power;
3. power-cycle the Pi once;
4. observe dnsmasq TFTP serving `da591740/kernel_2712.img` from the same
   selected tree byte count;
5. confirm final pre-restore lab identity still reports the selected tree until
   an explicit restore occurs.

This statement does not depend on prior helper names. It is the direct
publication-to-power contract for `kernel_2712.img`.

## Invariant

After selecting a candidate/control tree and before hardware power-cycle, the
lab API effective tree, API-visible boot archive `kernel_2712.img` size/hash,
TFTP-served `da591740/kernel_2712.img` bytes, and final pre-restore tree must
all match the selected tree unless an explicit restore occurs after capture.

## Contradicting Evidence

- Accepted staging sentinel evidence: candidate selected tree
  `a804458a439c20200a14b8d338341dca427ed7faba6bc7fb2c875049de586cc0`
  reported 47,832-byte TFTP fetches and selected final pre-restore identity;
  control selected tree
  `9d9b3cdb7b1f230d9cd2bf0c04b7c32dd98b53dd8ec7de77e99860c5b231908d`
  reported 47,824-byte TFTP fetches and selected final pre-restore identity.
- Register-vector v3 evidence: candidate selected tree
  `e81550ef7ba1252f10763a055d89c1f72b9cbc0b85bb60e512d0b7890bf0c724`
  expected 52,352 bytes but observed 104,136-byte TFTP fetches; control selected
  tree `aed051ee00bc30a808a4ad8b84b983d4c06924971b3305a3fd7c0cae905eb93d`
  expected 50,120 bytes but observed 104,136-byte TFTP fetches. Both final
  pre-restore identities were baseline.
- Intervention discriminator evidence: the no-MDIO sentinel candidate archive
  selected tree
  `a804458a439c20200a14b8d338341dca427ed7faba6bc7fb2c875049de586cc0`
  was API-visible before power and at final pre-restore with 47,832-byte
  `kernel_2712.img`, but dnsmasq served 104,136-byte baseline
  `da591740/kernel_2712.img` during the power-cycle.

## Unproven Assumptions

- The lab API `/status` and `/boot/files` tree view is assumed to match the
  dnsmasq TFTP root, but the discriminator shows that assumption is false for
  this run.
- The publish endpoint is assumed to update the exact root/cache used by
  dnsmasq immediately before power.
- Snapshot/restore is assumed to be explicit and not racing with boot fetches.
- `effective_kernel=kernel_2712.img` is assumed to describe the same file path
  the firmware fetches under the serial prefix.
- TFTP log byte counts are assumed to describe the bytes actually served by the
  live TFTP daemon; this is now the strongest evidence stream for the mismatch.

## Approaches Considered

- Lab/API publication-path trace without hardware power: record local archive
  SHA/size, publish result, `GET /status`, and `GET /boot/files` before
  power. This proved the API-visible boot tree selected the 47,832-byte no-MDIO
  archive.
- Hardware-backed minimal sentinel: power-cycle only the selected no-MDIO
  sentinel archive with no MDIO/MAN code, then compare TFTP delta and final
  identity. This proved the TFTP daemon served baseline-sized
  `da591740/kernel_2712.img` while the lab API still reported the selected
  tree.

## Smallest Decisive Discriminator

The discriminator reused the accepted no-MDIO staging-sentinel candidate archive
and recorded:

- pre-power `GET /status`: selected tree
  `a804458a439c20200a14b8d338341dca427ed7faba6bc7fb2c875049de586cc0`;
- pre-power `GET /boot/files`: `kernel_2712.img` and
  `da591740/kernel_2712.img` both 47,832 bytes;
- power-cycle TFTP delta: dnsmasq served `da591740/kernel_2712.img` twice at
  104,136 bytes;
- final pre-restore `GET /status`: selected tree still
  `a804458a439c20200a14b8d338341dca427ed7faba6bc7fb2c875049de586cc0`;
- restore proof: lab returned to baseline tree
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.

The failing layer is therefore between the lab API-visible boot-root
publication/reporting path and the actual dnsmasq-served TFTP root/cache. It is
not caused by register-vector MDIO/MAN code.

## Workaround Removal And Quarantine Plan

- Quarantine the prior staging sentinel closeout as insufficient to unblock
  register-vector retries. It established one historical selected-tree run, but
  it did not prove durable equivalence between the lab API boot view and the
  actual TFTP-served root/cache.
- Do not add another register-vector retry, helper shim, or broader MDIO/PHY
  task until a supervisor-planned task reconciles the lab API/dnsmasq root
  mismatch.
- Supersede capture helpers that treat `/boot/files` alone as decisive selected
  tree identity for hardware proofs. Future gates must require TFTP-served
  kernel byte agreement from the same power-cycle before accepting runtime
  evidence.

## Evidence

- classification: `classification.json`
- evidence map: `evidence-map.json`
- late TFTP delta: `tftp-delta-after-power-late.json`
- pre-power lab API status/files: `pre-power-status.json`,
  `pre-power-boot-files.json`
- final pre-restore lab API status/files: `final-pre-restore-status.json`,
  `final-pre-restore-boot-files.json`
- restore proof: `restore-snapshot.json`, `post-restore-status.json`
