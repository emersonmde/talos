# Phase 12.6 SSH runner OpenSSH client provisioning preflight

Task id: phase12-ssh-runner-openssh-client-provisioning-preflight-20260623
Status: accepted
Owner: worker
Classification: openssh-client-workspace-local-provisioned

## Goal

Provision or prove unavailable a recoverable workspace-local
OpenSSH-compatible client path for one bounded live discriminator retry.

## Reviewed Inputs

- memory/talos-supervisor-state.json task
  phase12-ssh-runner-openssh-client-provisioning-preflight-20260623.
- tasks/2026-06-23-phase12-ssh-runner-openssh-client-tooling-preflight.md.
- tasks/2026-06-23-phase12-ssh-live-openssh-client-contract.md.
- tasks/2026-06-23-phase12-ssh-live-openssh-client-discriminator-pi5-evidence.md.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Execution Summary

The worker promoted exactly one ready Phase 12.6 task after supervisor planning
authorized one recoverable workspace-local public package/tool provisioning
attempt. The preflight used apt-get download openssh-client and dpkg-deb -x to
download and extract a public Debian OpenSSH client package into the
workspace-local tooling area. It did not run apt-get install, mutate system
packages, connect to Talos, run a live OpenSSH discriminator, touch the lab or
hardware, publish a boot archive, or change Talos runtime code.

The extracted workspace-local ssh executable returned a sanitized version
family from ssh -V. ldd showed a dynamically linked executable whose
dependencies are present in the runner through normal runtime/library
categories. The accepted public classification is
openssh-client-workspace-local-provisioned. This accepts only a usable
workspace-local OpenSSH-compatible client path for a future bounded
discriminator attempt.

## Evidence

- sanitized provisioning/capability evidence:
  tasks/evidence/2026-06-23-phase12-ssh-runner-openssh-client-provisioning-preflight/client-provisioning.sanitized.json.
- usable OpenSSH-compatible client path: true.
- provisioning source category: public Debian package repository.
- package action category: workspace-local download and extraction.
- system package installation: false.
- selected_next_task: phase12-ssh-lab-boot-capture-preflight-20260623.
- planningNeeded: false.

## Findings And Disposition

- fixed: reviewed the accepted fail-closed runner/tooling preflight and live
  OpenSSH client contract before provisioning.
- fixed: used only a recoverable workspace-local public package download and
  extraction path; no system package installation or irreversible system
  mutation was performed.
- fixed: retained sanitized package, executable, version-family, capability,
  and dynamic dependency category evidence for the workspace-local client.
- deferred: lab boot-capture freshness and the live OpenSSH client
  discriminator retry remain future tasks; this preflight did not touch the lab
  or run OpenSSH against Talos.
- not-an-issue: no Rust source, Talos runtime implementation, lab/hardware
  action, boot publication, host key material, raw OpenSSH log, user/address
  identifier, stable peer identifier, or ADR entry is required for this
  provisioning preflight.

## Validation

- static task/docs/source review: pass.
- sanitized ssh -V or equivalent version/capability check: pass; retained only
  version-family and capability category.
- ldd or equivalent dynamic dependency category check: pass; retained only
  dependency categories.
- jq empty on client-provisioning.sanitized.json: pass.
- cargo fmt --all -- --check: conditional skip, no Rust source, tests, scripts
  that generate Rust artifacts, Cargo metadata, or build/tooling scripts
  touched.
- cargo -Zjson-target-spec test --quiet: conditional skip, no Rust source,
  tests, or Cargo metadata touched.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.

Evidence levels: static task/docs/source review, workspace-local public package
retrieval/extraction, sanitized local executable capability check, sanitized
dynamic dependency category check, JSON syntax check, diff checks, and docs
build. No system package installation, Talos host connection, lab/hardware
evidence, boot publication, live reachability claim, remote receipt claim,
compatibility=true claim, broad command expansion, phase transition, or
ssh-ready=true was performed.

## Redaction Review

Pass. Retained evidence contains only task ids, public package/tool category
names, public version family, workspace-local artifact categories, artifact
sizes and hashes, dependency category names, validation commands, and
classifications. It retains no raw OpenSSH logs, key material, host keys,
authorized keys, fingerprints, signatures, session identifiers, channel
identifiers, command bytes, payload bytes, user names, addresses, stable peer
identifiers, packet captures, hardware serial text, or private user data.

## Acceptance

Accepted only as a workspace-local OpenSSH-compatible client provisioning
preflight: openssh-client-workspace-local-provisioned.

selected_next_task=phase12-ssh-lab-boot-capture-preflight-20260623.
planningNeeded=false.
