# Phase 11 RP1 IRQ/Clock/GPIO Diagnostic Closeout

Task id: phase11-rp1-irq-clock-gpio-diagnostic-closeout-20260607

Status: accepted

## Goal

Close out the first Milestone 11.2 RP1 GPIO/status diagnostic proof chain and
record the exact accepted boundary before any further Phase 11 work.

## Scope

- Reconciled the accepted source contract, local/static diagnostic core,
  no-MMIO Pi 5 control, and real Pi 5 GPIO14 STATUS diagnostic blocker.
- Recorded accepted and unaccepted claims for the interrupt path, clock/reset
  assumptions, GPIO/status behavior, and capture/restore hygiene.
- Updated roadmap and project contract docs for the accepted blocker boundary.
- Set next action to supervisor planning for a qualitatively different
  discriminator or capture/staging repair; same-shaped GPIO14 STATUS hardware
  reruns remain blocked.

## Non-Goals

No hardware run, boot archive publication, hardwareTestLock acquisition,
runtime source changes, GPIO/pin-control writes, clock/reset programming,
interrupt enablement or handling, DMA/cache work, storage, generated-root,
networking, SSH, broader PCIe enumeration, Milestone 11.3, or phase transition.

## Classification

Accepted as capture-staging-blocked.

The source contract selected one read-only diagnostic:
phase11-rp1-irq-clock-gpio-contract-v1 / rp1-gpio14-status-read, a single
32-bit volatile load from CPU physical 0x1f000d0070 for RP1 IO_BANK0 GPIO14
STATUS. The local/static core built that real candidate and a matching no-MMIO
control. Static inspection showed the real candidate has exactly one contracted
RP1 volatile load, and the control constructs no contracted RP1 GPIO/RIO/PADS,
clock, reset, or MSI-X MMIO address.

The no-MMIO Pi 5 control is accepted as
no-mmio-gpio-status-control-visible. The accepted rerun passed the
pi5-capture-transaction-v2 identity join with no rejection reasons, retained two
46,160-byte selected candidate TFTP fetches, retained 795
TALOS: gpio14-status-control markers, and restored the lab to the pre-run boot
tree.

The real diagnostic Pi 5 proof remains capture-staging-blocked. The accepted
real diagnostic archive SHA-256 was
7bc21b39a5d0150221a244701285d733c8faef4e153085a49a34b5069c1fecea, and the lab
publication selected boot tree
cb7827b07a3822370fc610dfd18a8ab580cea31a47c4559e41a242975976f83a with a
46,336-byte da591740/kernel_2712.img. A marker-visible candidate run retained
483 TALOS: gpio14-status-result occurrences, but the v2 identity join rejected
that run due to non-empty pre-power serial drain, expected-fetch byte mismatch,
final selected-tree mismatch, and final expected-fetch byte mismatch. The
required known-good control and candidate rerun were then retained. The final
candidate rerun had empty pre-power serial drain and final selected-tree
identity, but no candidate-tied TFTP fetch and no diagnostic marker. The final
restore returned the lab to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Findings And Disposition

- fixed: reconciled the source-backed RP1 GPIO14 STATUS contract with the
  local/static core evidence and retained the exact diagnostic address,
  operation, archive identity, and no-MMIO control boundary.
- fixed: recorded the no-MMIO Pi 5 control as proof-chain-ready for this output
  shape only; it does not prove real GPIO/status behavior.
- fixed: recorded the marker-visible real diagnostic run as rejected
  capture-staging evidence because candidate identity was not strong enough
  under the v2 transaction contract.
- fixed: retained the required inconclusive-run triage: candidate identity,
  fresh serial cursor, TFTP delta, known-good control, candidate rerun, and
  restore evidence.
- deferred: a next Milestone 11.2 hardware step requires supervisor planning for
  a different discriminator or capture/staging repair. Same-shaped GPIO14
  STATUS reruns are blocked by this closeout.
- not-an-issue: GPIO14 may be muxed as UART0 TXD; the attempted diagnostic was
  read-only and made no GPIO ownership, pinmux, pad, interrupt, clock, or reset
  claim.

## Accepted Claims

- Source references document the RP1 GPIO14 STATUS register translation,
  interrupt routing path, and clock/reset assumptions for this narrow slice.
- The local/static real candidate and no-MMIO control artifacts satisfy the
  accepted source contract.
- The no-MMIO Pi 5 control output shape is visible and identity-joined.
- The real Pi 5 diagnostic proof is closed only as capture-staging-blocked.
- Capture/restore hygiene evidence was retained, including known-good control,
  candidate rerun, and final restore.

## Unaccepted Claims

Real RP1 GPIO14 STATUS read behavior, bus-fault/trap behavior, GPIO ownership,
pin-control or pad writes, interrupt enablement/routing proof/delivery,
clock/reset programming, DMA/cache behavior, storage, generated-root,
networking, SSH, broader PCIe enumeration, Milestone 11.3, and phase transition
remain unaccepted.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-diagnostic-closeout/evidence-map.json.
- Source contract:
  tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/evidence-map.json.
- Diagnostic core:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-status-diagnostic-core/evidence-map.json.
- No-MMIO control:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-status-no-mmio-control-pi5/classification.json.
- Real diagnostic blocker:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-status-diagnostic-pi5/classification.json.

## Validation

- Static evidence inspection: source contract, local/static core, no-MMIO Pi 5
  control, and real Pi 5 blocker records inspected.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.

## Result

Accepted as capture-staging-blocked. No further mechanically unblocked task is
defined in the queue. Supervisor planning is required before the next Phase 11
slice, and same-shaped GPIO14 STATUS hardware reruns remain blocked.
