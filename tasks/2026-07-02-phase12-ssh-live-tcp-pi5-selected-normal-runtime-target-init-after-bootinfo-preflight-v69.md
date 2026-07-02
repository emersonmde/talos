# Phase 12 Selected Normal-Runtime Target-Init After BootInfo Preflight v69

Status: accepted

Terminal classification:
inconclusive-selected-normal-runtime-target-init-preflight

Commit: pending

## Scope

This task ran the serialized Pi 5 preflight selected by the accepted v67
post-BootInfo reconciliation. It used the accepted non-published archive
target/tmp/selected-normal-runtime-target-init-v67.tar.gz and did not change
source code, packet I/O, OpenSSH, shell behavior, or phase status.

The selected archive SHA-256 is
18270d2ca0bef45c72898beaa55971b48d748f3a87a767556074423821f17352.
The selected da591740/kernel_2712.img is 152,880 bytes with SHA-256
4513bd97689673f904a849b60aee0377d6ddcc813ad0d00a18e422b3cc52ef82.

## Result

The first selected candidate run staged tree
3a87fb0afcb024cd6cec78652e42935ce276f95471de525f697611c2bc8f4cf1,
served da591740/kernel_2712.img twice at 152,880 bytes, retained 1,978
TALOS: target init occurrences, and restored the lab to
phase12-ssh-v10-openssh-clean-pre-20260624T074100Z.

The retained-proof gate also required the literal TALOS: boot info parsed
marker in the same selected candidate window. The first selected run retained
zero occurrences of that literal marker. Per the task gate, the worker ran the
inconclusive-run triage before classifying:

- candidate identity: selected tree, effective kernel, and selected TFTP byte
  service were decisive;
- fresh serial cursor: captured by the helper using direct-read fallback from
  the saturated cursor boundary;
- TFTP delta: stable same-cursor delta retained two selected kernel serves;
- known-good control: restored baseline served its 104,136-byte kernel twice,
  but did not produce a decisive target-init identity join;
- candidate rerun: selected tree was republished, served the selected kernel
  twice at 152,880 bytes, and retained 382 TALOS: target init occurrences,
  again with zero literal TALOS: boot info parsed occurrences.

The terminal result is therefore inconclusive rather than
selected-normal-runtime-target-init-marker-retained. The first unresolved
evidence reason is narrow: the selected serial line proves target-init marker
emission and carries claims-bootinfo-parsed=true, but the v69 retained-proof
acceptance gate asked for the exact TALOS: boot info parsed marker in the same
selected window.

## Findings

- fixed: acquired hardwareTestLock before lab publication, boot snapshot
  mutation, Pi 5 power action, serial capture, or TFTP capture.
- fixed: staged and reviewed the exact v67 selected target-init archive before
  publication.
- fixed: retained pre-publication, post-publication, serial, TFTP, final
  pre-restore, restore, and post-restore lab evidence.
- fixed: performed the required inconclusive-run triage before classification.
- not-an-issue: TALOS: target init was retained in two selected candidate
  windows, so the selected artifact does reach the target-init marker loop.
- deferred: whether claims-bootinfo-parsed=true plus source lineage is enough
  to close the target-init frontier belongs to the selected no-hardware v69
  closeout, not this hardware preflight.

## Evidence

- Classification:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-pi5-selected-normal-runtime-target-init-after-bootinfo-preflight-v69/classification.json.
- Evidence map:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-pi5-selected-normal-runtime-target-init-after-bootinfo-preflight-v69/evidence-map.json.
- First selected run:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-pi5-selected-normal-runtime-target-init-after-bootinfo-preflight-v69/run-20260702T075703Z.
- Inconclusive triage:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-pi5-selected-normal-runtime-target-init-after-bootinfo-preflight-v69/validation/inconclusive-triage-summary.json.

## Validation

- git status --short --branch before lab action: pass.
- jq empty on supervisor state and task-owned JSON evidence: pass.
- Static archive review: pass.
- Target-init archive review: pass.
- Lab API status before publication: captured.
- Fresh serial cursor and TFTP cursor before power action: captured.
- Serialized Pi 5 selected candidate run: retained selected TFTP service and
  target-init marker evidence.
- Inconclusive-run triage: known-good control and candidate rerun captured.
- Restore proof: lab restored to tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 with
  effective kernel kernel_2712.img.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote
phase12-ssh-live-tcp-selected-normal-runtime-target-init-after-bootinfo-closeout-v69-20260702
on the next worker wake if dependencies remain satisfied. The closeout must not
perform additional hardware action.
