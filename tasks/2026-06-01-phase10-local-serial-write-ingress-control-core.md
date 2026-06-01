# Phase 10 Local Serial Write Ingress Control Core

Task: phase10-local-serial-write-ingress-control-core-20260601

Status: accepted

## Goal

Create a prompt-live local/QEMU serial-write ingress control that proves the
descriptor-backed command loop remains serviced for delayed input after a
visible `talos> ` prompt.

## Scope

This task adds a local/QEMU proof-control artifact only:

- `scripts/qemu-local-serial-write-ingress-control.sh`
- `tasks/evidence/2026-06-01-qemu-local-serial-write-ingress-control-core/`

It does not change Talos runtime behavior, shell semantics, Pi 5 proof code,
lab-controller code, boot archive publication, or hardware state. The paused
`phase10-pi5-local-ls-root-proof-20260601` task remains paused.

## Artifact-Lifetime Discriminator

The prior Pi 5 command-response control proved a fresh prompt-capable accepted
control fetched over TFTP and reached `talos>`, but post-cursor writes produced
zero response bytes. That left an artifact-lifetime ambiguity: a proof/control
artifact could reach its proof prompt and then stop servicing input, making a
hardware write failure indistinguishable from a finished local proof harness.

The new local control observes the raw serial stream byte-by-byte, waits until
the visible `talos> ` prompt boundary is present in the retained transcript, then
delays and writes each command over the same serial socket. The accepted
invariant is prompt-live servicing: after a visible prompt, delayed serial input
is accepted through fd0/runtime-console0, dispatches through the
descriptor-backed command loop, writes the visible response through
descriptor-backed stdout, and returns to next-prompt readiness.

## Evidence

Prompt-live serial-write ingress control:

- `tasks/evidence/2026-06-01-qemu-local-serial-write-ingress-control-core/qemu-local-serial-write-ingress-control.log`

The retained transcript shows:

- visible `talos> ` prompts before each host write;
- host markers `injection=after-visible-prompt` for commands 0 through 5;
- `qemu-local-literal-echo: start ... input=fd0/runtime-console0 ... descriptor-backed-input=true descriptor-backed-output=true`;
- delayed `echo local serial works` input after command 3 prompt;
- visible `local serial works` response;
- `qemu-local-literal-echo: dispatch command=3 status=handled responses=1`;
- `qemu-local-literal-echo: ready-for-next prompt=true`;
- `qemu-local-serial-write-ingress-control: final ... classification=serial-write-ingress-control-complete`;
- `qemu-local-serial-write-ingress-control: PASS`.

Targeted command-loop regression evidence copied for this task:

- `tasks/evidence/2026-06-01-qemu-local-serial-write-ingress-control-core/regressions/qemu-local-ls-root-smoke.log`
- `tasks/evidence/2026-06-01-qemu-local-serial-write-ingress-control-core/regressions/qemu-local-help-command-smoke.log`
- `tasks/evidence/2026-06-01-qemu-local-serial-write-ingress-control-core/regressions/qemu-local-literal-echo-smoke.log`
- `tasks/evidence/2026-06-01-qemu-local-serial-write-ingress-control-core/regressions/qemu-local-line-editing-smoke.log`
- `tasks/evidence/2026-06-01-qemu-local-serial-write-ingress-control-core/regressions/qemu-local-line-cancel-smoke.log`
- `tasks/evidence/2026-06-01-qemu-local-serial-write-ingress-control-core/regressions/qemu-local-line-kill-smoke.log`

## Validation

- static inspection: new script only; no Talos runtime source, Pi 5 proof code,
  lab-controller code, boot publication script, or hardware task file was
  changed for this task.
- fmt/lint: `cargo fmt --all -- --check`.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed 349 no_std tests.
- QEMU/substitute: `scripts/qemu-local-serial-write-ingress-control.sh` passed
  with `serial-write-ingress-control-complete` and exact `PASS` vocabulary.
- QEMU/substitute regressions: `scripts/qemu-local-ls-root-smoke.sh`,
  `scripts/qemu-local-help-command-smoke.sh`,
  `scripts/qemu-local-literal-echo-smoke.sh`,
  `scripts/qemu-local-line-editing-smoke.sh`,
  `scripts/qemu-local-line-cancel-smoke.sh`, and
  `scripts/qemu-local-line-kill-smoke.sh` passed.
- hardware lock: `hardwareTestLock` remained unlocked/restored and unused; no
  Pi 5 hardware action, boot archive publication, or lab-controller mutation
  occurred.
- docs: no mdBook docs were touched, so `mdbook build` was not required.

## Accepted Frontier

The prompt-live local/QEMU control is accepted. The next mechanically unblocked
task is `phase10-pi5-serial-write-ingress-control-proof-20260601`, which may
carry this same control invariant to serialized Pi 5 evidence if the hardware
lock remains unlocked/restored.

Deferred surfaces remain shell parsing beyond the accepted bounded commands,
argv/envp process ABI, userspace shell execution, process spawning, terminal
sessions, termios, networking, SSH, RP1/PCIe, UART interrupt ownership, and
DMA/cache-driver policy.
