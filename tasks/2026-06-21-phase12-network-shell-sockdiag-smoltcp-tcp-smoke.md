# Phase 12.4 Shell Sockdiag smoltcp TCP Smoke

Task: phase12-network-shell-sockdiag-smoltcp-tcp-smoke-20260621

Status: accepted

Classification: phase12-network-shell-sockdiag-smoltcp-tcp-smoke-accepted

## Scope

Retain host/QEMU-substitute smoke evidence proving shell-visible /bin/sockdiag
reaches the accepted private host-only smoltcp TCP bridge diagnostic through
VFS/userspace and the existing private socket syscall path.

This task does not change source behavior, acquire hardwareTestLock, mutate
the lab, publish a boot artifact, retain generated-root evidence, claim live
packet I/O, claim Pi 5 hardware behavior, claim hardware reachability, accept
SSH, accept a public stable socket ABI, broaden sockets, add UDP/raw sockets,
or transition phase.

## Findings And Dispositions

- fixed: Added scripts/qemu-shell-sockdiag-smoltcp-tcp-smoke.sh as the
  task-owned retained host/QEMU-substitute smoke command.
- fixed: Retained smoke output under
  tasks/evidence/2026-06-21-shell-sockdiag-smoltcp-tcp-smoke/ with command
  log, source anchors, transcript, classification, and evidence map.
- fixed: The smoke records exec /bin/sockdiag through VFS executable
  lookup/open/read, startup ABI, the existing private socket syscall path,
  Established smoltcp handshake states, accepted-descriptor attachment, one
  bounded payload-transfer observation, waitpid, and laststatus.
- fixed: Deterministic controls remain retained for malformed arguments,
  missing executable identity, unchanged local socket diagnostics, unchanged
  /bin/pingdiag, the predecessor smoltcp bridge regression, and bounded
  syscall vocabulary.
- not-an-issue: The no_std QEMU runner executes the full target test binary
  for each filtered smoke invocation. The retained transcript records three
  passing 693-test invocations and labels the intended boundary checks.
- removed: No source behavior change, lab artifact, hardware claim, public
  socket ABI claim, live packet I/O claim, broad socket claim, UDP/raw socket
  claim, SSH claim, or phase-transition claim was added.
- deferred: Smoke closeout remains the next dependency-gated reconciliation
  task before supervisor planning decides any broader socket/network direction.

## Evidence

- Smoke command:
  scripts/qemu-shell-sockdiag-smoltcp-tcp-smoke.sh.
- Retained transcript:
  tasks/evidence/2026-06-21-shell-sockdiag-smoltcp-tcp-smoke/smoke-transcript.md.
- Command transcript:
  tasks/evidence/2026-06-21-shell-sockdiag-smoltcp-tcp-smoke/qemu-shell-sockdiag-smoltcp-tcp-smoke.log.
- Source anchors:
  tasks/evidence/2026-06-21-shell-sockdiag-smoltcp-tcp-smoke/source-anchors.txt.
- Classification:
  tasks/evidence/2026-06-21-shell-sockdiag-smoltcp-tcp-smoke/classification.json.
- Evidence map:
  tasks/evidence/2026-06-21-shell-sockdiag-smoltcp-tcp-smoke/evidence-map.json.
- Accepted predecessor:
  phase12-network-shell-sockdiag-smoltcp-tcp-core-20260621 accepted and
  committed at b56b6f09bf3aca195a6f5776bc7bb0fe82880229.

## Validation

- scripts/qemu-shell-sockdiag-smoltcp-tcp-smoke.sh: passed, three
  host/QEMU-substitute test invocations each reporting 693 no_std tests passed.
- cargo fmt --all -- --check: not run because no Rust/source formatting change
  was made in this smoke task.
- cargo -Zjson-target-spec test --quiet: not run because no source behavior
  changed; the task-owned smoke ran the focused QEMU-substitute filters.
- jq empty on task-owned JSON evidence: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

No Pi 5 hardware run, hardwareTestLock acquisition, boot archive publication,
lab mutation, power cycle, generated-root publication, live driver adapter,
live packet I/O, hardware reachability, UDP/raw socket work, SSH, broad socket
expansion, public stable socket ABI acceptance, or phase transition was
performed.

## Acceptance

Accepted.

selected_next_task=phase12-network-shell-sockdiag-smoltcp-tcp-closeout-20260621.

The accepted evidence level is host/QEMU-substitute smoke evidence over
shell-visible VFS/userspace /bin/sockdiag execution, VFS executable identity,
startup ABI, private socket syscalls, descriptor-backed smoltcp TCP bridge
records, Established handshake states, accepted-descriptor attachment, one
bounded payload-transfer observation, waitpid, laststatus, deterministic
controls, unchanged accepted local socket diagnostics, unchanged /bin/pingdiag,
and unchanged bounded syscall vocabulary. Kernel fake TCP commands, live
driver adapters, live packet I/O, hardware reachability, SSH, lab mutation,
boot publication, generated-root publication, broad socket expansion, UDP/raw
sockets, public stable socket ABI acceptance, and phase transition remain
rejected.

Commit: recorded in durable supervisor state after commit creation.
