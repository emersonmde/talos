# Phase 12 SSH Live TCP Pi 5 Selected-Entry Boot-Path Preflight V26

Task id: phase12-ssh-live-tcp-pi5-selected-entry-boot-path-preflight-v26-20260701

Status: accepted after commit.

Classification: selected-entry-boot-path-marker-retained.

Evidence level: serialized Pi 5 hardware preflight with selected-tree identity,
stable same-cursor TFTP selected-byte evidence before restore, fresh post-power
serial marker evidence, final pre-restore identity, restore proof, task-owned
JSON evidence, docs build, and diff checks.

## Goal

Run the accepted v25 selected-entry replacement discriminator archive contract
on the Pi 5 and decide whether the rust_entry UART10 marker loop is retained
after selected-byte service.

## Scope Performed

- Promoted the queued v26 preflight after v25 accepted
  selected-entry-boot-path-rust-entry-discriminator-ready at commit
  540f9e3be36316854563d3ce1021e38421e6f272.
- Acquired hardwareTestLock before lab publication, boot mutation, and Pi 5
  power action.
- Re-materialized and reviewed the v25 rust_entry UART10 marker-loop selected
  archive before publication.
- Published the selected archive, captured selected-tree identity, fresh
  serial/TFTP cursors, post-power serial output, stable TFTP delta, final
  pre-restore identity, restore proof, and post-restore identity.
- Removed packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, and phase transition as immediate successors.

## Hardware Result

Primary run: selected-entry-boot-path-v26-20260701T004528Z.

The published selected archive retained the v25 contract:

- selected path: da591740/kernel_2712.img;
- kernel byte count: 45,400;
- kernel SHA-256:
  b597bc0d28aeda702492b9846ce9110ec5a99db6343c617a319ba265a0c59fa7;
- expected marker: TALOS: reu10-loop;
- Image header fields: text_offset=0, header_image_size=45,400, flags=12.

The Pi 5 run retained decisive selected-entry evidence:

- pre-power serial drain reached an empty read;
- stable same-cursor TFTP delta captured 13 events, including two selected
  da591740/kernel_2712.img serves at 45,400 bytes;
- post-power direct-read serial captured the expected TALOS: reu10-loop marker
  13,796 times;
- final pre-restore identity remained on selected tree
  a8da6043eecde204c1f58a612d25aa4939ad54084b0c1aa90a56da44ecae3e3a;
- restore returned the lab to tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

Because identity, TFTP, serial freshness, marker evidence, final identity, and
restore proof were decisive, the terminal classification is marker-retained.

## Terminal Classification

selected-entry-boot-path-marker-retained.

selected_next_task:
phase12-ssh-live-tcp-selected-entry-boot-path-closeout-v26-20260701.

planningNeeded: false.

Packet-I/O, OpenSSH/generated-root retry, remote receipt,
compatibility/service readiness, ssh-ready=true, broad shell work,
fake/kernel-backed command expansion, and phase transition remain blocked.

## Findings

- fixed: executed the v25 rust_entry UART10 marker-loop replacement
  discriminator under hardwareTestLock with the exact selected archive
  contract.
- not-an-issue: selected-byte service, final pre-restore identity, serial
  freshness, and restore proof matched the v25 discriminator contract.
- fixed: the repeated rust_entry UART10 marker was retained in the fresh
  post-power serial window.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, and phase transition as immediate successors.

## Evidence Map

- Evidence map:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-entry-boot-path-preflight-v26/evidence-map.json.
- Classification:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-entry-boot-path-preflight-v26/classification.json.
- Primary hardware run directory:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-entry-boot-path-preflight-v26/selected-entry-boot-path-v26-20260701T004528Z/.
- Static materialization:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-entry-boot-path-preflight-v26/static/.

## Redaction Review

Task-owned evidence retains task ids, run labels, hashes, byte counts, marker
names, classifications, validation outcomes, selected-tree hashes, and
redacted lab status metadata. Raw serial text, raw TFTP peer/log-line fields,
packet payloads, SSH/session/key material, boot artifact bytes, private data,
and stable secret-derived identifiers were not retained.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before task-owned evidence creation.
- static archive/image review before publication: pass.
- Lab API GET / and /status records before publication, after publication,
  final pre-restore, and after restore: pass.
- fresh serial cursor/drain before Pi 5 power action: pass.
- GET /tftp/logs fresh cursor/delta before restore: pass.
- restore to phase12-ssh-v10-openssh-clean-pre-20260624T074100Z and confirm
  post-restore tree: pass.
- jq empty on task-owned JSON evidence and supervisor state: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.

Commit: recorded in talos-supervisor-state.json after final commit.
