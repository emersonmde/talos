# Phase 12 RP1 Ethernet MDIO Staging Identity Quarantine Core

Task id: phase12-rp1-ethernet-mdio-staging-identity-quarantine-core-20260611

Status: accepted

Classification: mdio-staging-identity-quarantine-accepted

Evidence level: static inspection, retained evidence review, script contract
inspection, JSON validation, and diff checks.

## Goal

Quarantine the stale selected-tree identity acceptance path after retained
evidence showed lab API-visible selected-tree identity can diverge from the
actual dnsmasq-served TFTP kernel bytes.

## Scope Performed

- Reviewed the accepted staging sentinel closeout and retained sentinel
  evidence.
- Reviewed the blocked register-vector v3 proof and its intervention
  discriminator.
- Reviewed the replay helpers that gate selected-tree identity for retained
  hardware evidence.
- Updated the staging identity and capture-chain-v4 replay helpers to emit the
  trust boundary that API-visible boot files are not sufficient without
  same-power-cycle TFTP-served kernel byte agreement.
- Updated the staging sentinel closeout record and evidence JSON so it no
  longer mechanically unblocks register-vector retries.

## Findings

- fixed: the staging sentinel closeout now has a quarantine addendum that
  marks its original register-vector retry selection as superseded by the later
  tftp-root-diverges-from-lab-api-selected-tree blocker.
- fixed: the staging sentinel closeout classification and evidence map now
  identify the quarantine scope and the future acceptance requirements:
  same-power-cycle TFTP-served kernel byte agreement, final pre-restore
  identity, restore proof, serial freshness, and evidence consistency.
- fixed: scripts/rpi5-staging-identity-gate-v1-check.sh now emits a
  trust_boundary object stating that API-visible /status or /boot/files
  identity alone is insufficient.
- fixed: scripts/rpi5-proof-identity-join-v4-check.sh now emits the same trust
  boundary in endpoint_identity for retained capture-chain-v4 replay output.
- not-an-issue: the existing v4 and staging gate rejection logic already
  blocks when expected TFTP fetch bytes are absent or mismatched, so this task
  did not need to change the pass/fail predicates.
- deferred: the deployed lab API/dnsmasq root mismatch remains for the
  publication/TFTP-root reconciliation trace task; this task made no lab
  service changes and performed no hardware action.
- removed: no runtime MDIO/MAN, Ethernet, or hardware proof code was removed.

## Quarantine Boundary

The retained staging sentinel candidate/control boots remain historical
evidence for their own no-MDIO/no-Ethernet runs. They are not sufficient to
unblock register-vector retries after the later discriminator proved the
expected invariant can fail: the lab API reported a selected 47,832-byte
sentinel tree while dnsmasq served the 104,136-byte baseline kernel during the
same power cycle.

Current required boundary for future runtime hardware acceptance:

- API-visible /status or /boot/files identity is not sufficient by itself.
- Same-power-cycle TFTP-served kernel bytes must match the selected tree's
  expected kernel bytes.
- Final pre-restore identity must still match the selected tree.
- Restore proof, serial freshness, and evidence consistency must agree.

No register-vector MAN.DATA values, broad MDIO/PHY ownership, PHY absence,
PHY reset, Ethernet behavior, networking, SSH, Phase 12.2, or phase transition
are accepted by this task.

## Reviewed Paths

- scripts/rpi5-staging-identity-gate-v1-check.sh: fixed.
- scripts/rpi5-proof-identity-join-v4-check.sh: fixed.
- scripts/rpi5-evidence-consistency-guard.sh: not-an-issue; it already treats
  v4 JSON as authoritative and rejects readiness overclaims when v4 blocks.
- tasks/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-sentinel-closeout.md:
  fixed.
- tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-sentinel-closeout/classification.json:
  fixed.
- tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-sentinel-closeout/evidence-map.json:
  fixed.
- tasks/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v3-after-staging-identity.md:
  not-an-issue; it already records the tftp-root-diverges-from-lab-api-selected-tree blocker and next quarantine action.

## Evidence

- Classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-staging-identity-quarantine-core/classification.json.
- Evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-staging-identity-quarantine-core/evidence-map.json.
- Staging sentinel closeout addendum:
  tasks/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-staging-sentinel-closeout.md.
- Later blocker:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v3-after-staging-identity/intervention-discriminator/classification.json.

## Validation

- static inspection: task records, retained evidence JSON, and helper scripts
  listed above.
- focused script checks: rpi5-capture-chain-v4 retained fixtures and staging
  identity gate replays over retained candidate/control evidence.
- JSON validation: jq empty on new and edited JSON evidence.
- diff check: git diff --check.
- docs validation: mdbook build not run because docs/src files were not
  touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Findings list with dispositions: satisfied.
- Prior staging sentinel closeout explicitly quarantined as insufficient to
  unblock register-vector retries: satisfied.
- Helper/documentation path now requires same-power-cycle TFTP-served kernel
  byte agreement, not only API-visible /status or /boot/files agreement:
  satisfied.
- Register-vector MAN.DATA values and broader MDIO/PHY/Ethernet claims remain
  unaccepted: satisfied.
- Accepted quarantine/guard changes committed before publication-path trace or
  hardware sentinel task starts: satisfied after commit.

## Next Action

Mechanically promote
phase12-rp1-ethernet-lab-tftp-root-reconciliation-trace-20260611 on the next
worker wake if dependencies remain satisfied, hardwareTestLock is
unlocked/restored, and supervisorIntervention remains inactive. Do not run
hardware, power-cycle, retry register-vector proof, mutate deployed lab
service code, start broad MDIO/PHY/Ethernet work, or create a phase transition
from this quarantine task.
