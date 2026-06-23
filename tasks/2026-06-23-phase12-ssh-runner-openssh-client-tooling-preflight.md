# Phase 12.6 SSH runner OpenSSH client tooling preflight

Task id: phase12-ssh-runner-openssh-client-tooling-preflight-20260623
Status: accepted
Owner: worker
Classification: openssh-client-tooling-unavailable

## Goal

Establish whether the worker runner has a sanitized OpenSSH-capable client path
before any further live-client or hardware discriminator work.

## Reviewed Inputs

- memory/talos-supervisor-state.json task
  phase12-ssh-runner-openssh-client-tooling-preflight-20260623.
- tasks/2026-06-23-phase12-ssh-live-openssh-client-contract.md.
- tasks/2026-06-23-phase12-ssh-live-openssh-client-discriminator-pi5-evidence.md.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Execution Summary

The worker promoted exactly one ready Phase 12.6 task and kept the work local to
runner/tooling inspection. No Talos runtime code changed, no lab or hardware
action ran, no boot archive was published, and no network connection was made
to Talos or any external host.

The local runner has no ssh executable and no alternate OpenSSH-compatible
client path on PATH. The worker also found no workspace-local or apt cache
archive for OpenSSH/dropbear-style client provisioning. Because this task
explicitly forbids network connections to external hosts and irreversible system
mutation, the worker did not fetch packages or install system tooling. The
accepted public blocker label is openssh-client-tooling-unavailable.

## Evidence

- sanitized client tooling evidence:
  tasks/evidence/2026-06-23-phase12-ssh-runner-openssh-client-tooling-preflight/client-tooling.sanitized.json.
- usable ssh/OpenSSH-compatible client path: false.
- selected_next_task: null.
- planningNeeded: true.

## Findings And Disposition

- fixed: reviewed the accepted live OpenSSH client contract and fail-closed
  discriminator blocker before choosing the next action.
- fixed: checked local PATH for ssh, dbclient, dropbear, sshpass, slogin, scp,
  and sftp without connecting to any host.
- fixed: checked for a local package/archive provisioning path and recorded
  that no recoverable local archive was available.
- deferred: lab boot-capture preflight and live OpenSSH discriminator retry are
  not mechanically unblocked because no usable client path exists.
- not-an-issue: no Rust source, Talos runtime implementation, lab/hardware
  action, boot publication, network connection, key material, raw OpenSSH log,
  user/address identifier, stable peer identifier, or ADR entry is required for
  this tooling preflight.

## Validation

- static task/docs/source review: pass.
- ssh -V or equivalent version/capability check: conditional skip, no client
  path available.
- cargo fmt --all -- --check: conditional skip, no Rust source, tests, scripts
  that generate Rust artifacts, or Cargo metadata touched.
- cargo -Zjson-target-spec test --quiet: conditional skip, no Rust source,
  tests, or Cargo metadata touched.
- jq empty on client-tooling.sanitized.json: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.

Evidence levels: static task/docs/source review, local sanitized tooling
inspection, diff checks, and docs build. No host connection, OpenSSH launch,
lab/hardware evidence, boot publication, live reachability claim, remote
receipt claim, compatibility=true claim, broad command expansion, phase
transition, or ssh-ready=true was performed.

## Redaction Review

Pass. Retained evidence contains only task ids, public executable command names,
public package/tool category names, fixed availability labels, validation
commands, and classifications. It retains no raw OpenSSH output, key material,
host keys, authorized keys, fingerprints, signatures, session identifiers,
channel identifiers, command bytes, payload bytes, user names, addresses,
stable peer identifiers, packet captures, hardware serial text, or private user
data.

## Acceptance

Accepted only as a fail-closed tooling blocker:
openssh-client-tooling-unavailable.

selected_next_task=null.
planningNeeded=true.
planningReason=Supervisor must provide an OpenSSH-capable runner/tool path or
authorize a different recoverable provisioning path before the lab boot-capture
preflight or live OpenSSH discriminator retry can be mechanically unblocked.
