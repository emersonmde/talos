# Phase 12 Pi 5 Lab Known-Good Power-Cycle Recovery

Task id: phase12-pi5-lab-known-good-power-cycle-recovery-20260610
Status: accepted
Owner: worker
Classification:
known-good-power-cycle-tftp-recovered-serial-silent-blocker
Evidence level: lab-controller API, TFTP/capture evidence, serial hardware
output, and static task/doc inspection. No GPIO32 / ETH_RST_N write/restore
retry, PHY reset assertion/deassertion proof, MDIO or PHY ownership, Ethernet
driver readiness, packet I/O, networking, sockets, SSH, Phase 12.2, or phase
transition was accepted.

## Goal

Recover or precisely re-block the Pi 5 known-good power-cycle evidence gate
before any same-shaped GPIO32 PHY-reset write/restore retry.

## Findings

- fixed: pre-power identity was recorded through `GET /status` and matched the
  restored known-good tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10,
  effective kernel kernel_2712.img, and da591740/kernel_2712.img at 104136
  bytes.
- fixed: the deployed lab API again proved that `GET /` is not the identity
  endpoint; the task retained the unknown-endpoint response as endpoint
  semantics evidence and used `GET /status` for identity.
- fixed: fresh pre-power cursors were retained: serial cursor 4194304 and
  TFTP cursor 4418657.
- fixed: the single authorized known-good power cycle returned ok=true and
  final identity remained the same restored known-good tree, effective kernel,
  and 104136-byte da591740/kernel_2712.img.
- fixed: the previous no-fetch part of the lab blocker recovered for
  known-good: the TFTP delta from cursor 4418657 retained 13 events, including
  two da591740/kernel_2712.img fetches at 104136 bytes.
- blocked: the serial gate did not recover; observe from the fresh serial
  cursor captured 0 bytes, no `TALOS:` marker, and no expected known-good Talos
  output.
- deferred: GPIO32 write/restore proof, GPIO32 ownership, PHY reset
  assertion/deassertion proof, MDIO/PHY ownership, Ethernet driver readiness,
  interrupt completion, DMA/descriptors, packet I/O, networking, sockets, SSH,
  Phase 12.2, and phase transition remain unaccepted.

No findings were removed.

## Accepted Boundary

The accepted recovery result is narrower than the previous
lab-power-cycle-no-fetch blocker: known-good TFTP fetch evidence recovered, but
expected known-good Talos serial output remained absent. This task therefore
accepts `known-good-power-cycle-tftp-recovered-serial-silent-blocker`, not a
ready GPIO32 write/restore retry gate.

Same-shaped GPIO32 PHY-reset write/restore hardware retries remain held. A
future retry needs a supervisor-planned follow-up that explains and resolves
the serial-silent known-good boot evidence gap, or defines a new discriminator
with explicit acceptance criteria.

## Evidence

- Classification:
  tasks/evidence/2026-06-10-phase12-pi5-lab-known-good-power-cycle-recovery/classification.json.
- Evidence map:
  tasks/evidence/2026-06-10-phase12-pi5-lab-known-good-power-cycle-recovery/evidence-map.json.
- Capture summary:
  tasks/evidence/2026-06-10-phase12-pi5-lab-known-good-power-cycle-recovery/capture-summary.json.
- Pre-power status:
  tasks/evidence/2026-06-10-phase12-pi5-lab-known-good-power-cycle-recovery/known-good-recovery-run/pre-power-status.json.
- TFTP delta from the fresh pre-power cursor:
  tasks/evidence/2026-06-10-phase12-pi5-lab-known-good-power-cycle-recovery/known-good-recovery-run/tftp-delta-after-power-from-pre-cursor.json.
- Serial observe after power:
  tasks/evidence/2026-06-10-phase12-pi5-lab-known-good-power-cycle-recovery/known-good-recovery-run/serial-observe-after-power.json.
- Final identity:
  tasks/evidence/2026-06-10-phase12-pi5-lab-known-good-power-cycle-recovery/known-good-recovery-run/final-status.json.

## Validation

- lab-controller API: `GET /status` before and after the run matched the
  restored known-good tree and kernel identity.
- TFTP/capture evidence: `GET /tftp/logs` from fresh cursor 4418657 retained
  13 events and two 104136-byte known-good kernel fetches.
- serial hardware output: observe from fresh cursor 4194304 captured 0 bytes;
  no expected known-good Talos serial output was present.
- hardware lock: acquired before the power cycle and released only after final
  known-good identity was recorded.
- JSON validation: jq empty on classification/evidence-map/capture summary
  JSON passed.
- diff check: git diff --check passed.
- docs build: /home/node/.cargo/bin/mdbook build passed.
- staged diff check: git diff --cached --check passed.

## Next Action

Close out this recovery result before any same-shaped GPIO32 write/restore
retry. Do not rerun GPIO32 write/restore until the supervisor accepts a bounded
serial-silent known-good boot discriminator or another explicit recovery gate.
