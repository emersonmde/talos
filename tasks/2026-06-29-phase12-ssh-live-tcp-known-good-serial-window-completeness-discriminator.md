# Phase 12 SSH Live TCP Known-Good Serial Window Completeness Discriminator

Task id: phase12-ssh-live-tcp-known-good-serial-window-completeness-discriminator-20260629

Status: accepted after commit.

Classification: blocked-known-good-kernel-not-starting.

Evidence level: lab-controller API identity, serial hardware boot/output with
empty pre-power serial drain, stable same-cursor TFTP delta, restore proof,
task-owned JSON evidence, docs build, and diff checks. No candidate archive,
packet-I/O discriminator, OpenSSH/generated-root retry, remote receipt,
compatibility claim, service success claim, ssh-ready=true, runtime russh
adoption, fake command expansion, broad shell work, or phase transition was
performed.

## Goal

Run one bounded known-good-only Pi 5 lab discriminator to decide whether the
missing production marker is a serial capture/window problem or a real
known-good runtime-readiness failure.

## Scope Performed

- Promoted the mechanically selected known-good serial-window discriminator and
  acquired hardwareTestLock before lab action.
- Verified the restored known-good/control tree through lab-controller
  /status, /boot/files, and /boot/snapshots before power.
- Ran one known-good-only power cycle through the capture-invariant bundle with
  an explicit empty pre-power /serial/read drain before power.
- Retained the post-power serial window from the saturated-cursor direct-read
  fallback, stable same-cursor TFTP evidence, final identity, restore evidence,
  and post-restore status/files/snapshots.
- Restored the lab to
  abcontrol-secondary-workload-pre-20260524T231449Z and retained recovery
  metadata for the redundant same-tree safety restore.

## Terminal Classification

blocked-known-good-kernel-not-starting.

The discriminator proved serial-window completeness enough to reject the prior
capture-window ambiguity: the pre-power /serial/read drain reached an empty
read before the power cycle, the post-power serial window retained 4473 bytes
through the saturated-cursor direct-read fallback, and that fresh window
contained two firmware NETWORK markers. It still omitted both TALOS:
kernel_main and the required rpi5-production-timer-preemption: PASS marker.

Boot identity and TFTP evidence joined the same restored known-good tree. The
preflight and final-pre-restore identity both reported tree
6ead8933b3287cddaf08ddcdbda7d4c770b365658ef1d0ad80a5dd0cf323f3ef with
effective_kernel=kernel_2712.img, and stable TFTP evidence retained 13 events
including two matching 82045-byte da591740/kernel_2712.img serves. The
identity-join rejection list was empty.

Because the fresh known-good serial window still does not reach kernel_main or
the downstream production-timer PASS marker, this task does not select
candidate preflight v3. planningNeeded is true before any candidate retry,
packet-I/O discriminator, OpenSSH/generated-root retry, remote receipt,
compatibility, service success, ssh-ready=true, runtime russh adoption, fake
command expansion, broad shell work, or phase transition.

## Findings

- fixed: the selected discriminator used an empty pre-power /serial/read drain
  before power rather than the previous retained-tail/readiness helper alone.
- fixed: stable identity and TFTP fetch evidence joined the same restored tree
  with two matching 82045-byte da591740/kernel_2712.img serves.
- blocked: the post-power serial window captured firmware network output but
  no TALOS: kernel_main or rpi5-production-timer-preemption: PASS marker.
- not-an-issue: a redundant safety restore retained the same 6ead8933 tree and
  was followed by helper restore/post-restore evidence; it is recorded as
  recovery metadata and did not publish a candidate archive.
- deferred: candidate preflight v3, packet I/O, OpenSSH/generated-root retry,
  remote receipt, compatibility, service success, ssh-ready=true, runtime
  russh adoption, broad shell work, and phase transition remain deferred.
- removed: no source, helper, docs, task, or evidence artifact was removed.

## Evidence Map

- Evidence directory:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-known-good-serial-window-completeness-discriminator/known-good-serial-window-completeness-20260629T140602Z/.
- Classification:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-known-good-serial-window-completeness-discriminator/classification.json.
- Evidence map:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-known-good-serial-window-completeness-discriminator/evidence-map.json.
- Capture-invariant summary:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-known-good-serial-window-completeness-discriminator/known-good-serial-window-completeness-20260629T140602Z/capture-invariant-summary.json.
- Redacted summary:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-known-good-serial-window-completeness-discriminator/known-good-serial-window-completeness-20260629T140602Z/redacted-summary.json.
- Serial drain before power:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-known-good-serial-window-completeness-discriminator/known-good-serial-window-completeness-20260629T140602Z/serial-drain-before-power.json.
- Post-power serial window:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-known-good-serial-window-completeness-discriminator/known-good-serial-window-completeness-20260629T140602Z/serial-observe-window.json.
- Stable TFTP delta:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-known-good-serial-window-completeness-discriminator/known-good-serial-window-completeness-20260629T140602Z/tftp-delta-stable-pre-restore.json.
- Restore evidence:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-known-good-serial-window-completeness-discriminator/known-good-serial-window-completeness-20260629T140602Z/restore-snapshot.json.

## Redaction Review

Classification and evidence-map summaries omit peer IP/MAC and packet
contents. Raw lab-controller TFTP artifacts remain task-owned hardware evidence
and include the endpoint local client fields. No key material, session
material, boot artifact bytes, private user data, or secret-derived identifiers
are added.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before task-owned evidence creation.
- Lab API GET /status, GET /boot/files, and GET /boot/snapshots before power
  and after restore: pass; tree/effective kernel restored to the same known-good
  identity.
- Fresh serial cursor/completeness diagnostics before power: pass; empty
  pre-power /serial/read drain retained.
- GET /tftp/logs tail cursor and stable same-cursor delta before restore: pass;
  stable helper retained 13 events and two served kernel_2712.img fetches.
- Known-good readiness classifier or reconciled successor classifier: pass;
  reconciled successor discriminator classified blocked-known-good-kernel-not-starting.
- Restore to named snapshot/control state: pass; post-restore GET /status
  reports tree 6ead8933b3287cddaf08ddcdbda7d4c770b365658ef1d0ad80a5dd0cf323f3ef
  and effective_kernel=kernel_2712.img.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task: null.

planningNeeded: true.

planningReason: known-good serial-window completeness discriminator proved
empty pre-power serial drain, stable boot identity, stable TFTP fetches, and
firmware network output, but the same fresh post-power serial window still
omitted TALOS: kernel_main and rpi5-production-timer-preemption: PASS.
Supervisor planning is required before candidate preflight v3, packet-I/O
discriminator, OpenSSH/generated-root retry, remote receipt, compatibility,
service success, ssh-ready=true, runtime russh adoption, fake command
expansion, broad shell work, or phase transition.

Commit: recorded in talos-supervisor-state.json after final commit.
