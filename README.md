# Talos

Talos is an experimental Rust bare-metal operating system project for AArch64.
The first physical target is a Raspberry Pi 5; the fast local validation target
is QEMU `virt`.

This repository is also an experiment in asynchronous agentic kernel
development. Most work is planned, implemented, reviewed, and validated by
OpenClaw agents operating against a real lab controller. Human review still
sets direction and decides what is worth keeping.

Talos is not production software. It is a research and learning project for
kernel architecture, Raspberry Pi 5 bring-up, early serial output, exception
handling, memory discovery, paging, and eventually small Unix/POSIX-like
abstractions.

## Current Status

Talos currently has:

- A `no_std` Rust AArch64 kernel skeleton.
- QEMU `virt` boot/smoke support.
- Raspberry Pi 5 image and TFTP boot-tree staging scripts.
- Early PL011/RP1 UART paths for first-light diagnostics.
- AArch64 exception-vector plumbing and early panic reporting.
- Early firmware device-tree parsing for boot arguments, memory banks, and
  reserved ranges.
- Early low-memory bootstrap allocation and initial EL2 translation-table work.

The project is still before userspace, scheduling, filesystems, networking, or a
stable public API.

## Agentic Workflow

The Talos workflow is intentionally asynchronous:

- A supervisor agent keeps the roadmap, task records, acceptance criteria, and
  publishability constraints aligned.
- Worker agents take bounded tasks such as one hardware diagnostic, one memory
  milestone, or one documentation update.
- The lab controller gives agents a narrow control surface for Raspberry Pi 5
  boot testing: publish a TFTP boot archive, power-cycle the board, read serial
  output, inspect TFTP request logs, and roll back.
- Task notes record what was attempted, what passed locally, what hardware
  evidence was observed, and what remains ambiguous.

The goal is not to pretend the agents are always right. The goal is to make each
increment small enough that a human or later agent can audit the evidence and
continue from a known state.

## Talos Lab

Talos Lab is a private lab-control service used by OpenClaw. It may become a
separate GitHub project later. In this repository it is documented only as the
development interface that makes Pi 5 testing repeatable.

The lab keeps secrets and authority out of the kernel repo:

- Controller credentials stay in the lab service configuration.
- Agents use an internal HTTP API rather than direct controller access.
- Boot archives are bounded and reviewed before publish.
- Rollback is part of every hardware loop.
- Serial and TFTP logs are collected as evidence rather than treated as magic.

The current physical target facts that are useful for reproducing the lab shape
are intentionally narrow:

```text
talos-pi5 IP:  10.42.1.4
talos-pi5 MAC: 88:a2:9e:ae:c8:7f
serial prefix: da591740
serial baud:   115200
```

Deployment-specific switch names, controller IDs, credentials, and private host
paths should stay out of commits.

See `docs/src/project/lab-controller.md` for the internal API contract and the
expected agent loop.

## Build And Run

Use the pinned nightly toolchain from `rust-toolchain.toml`. The Cargo config
builds `core`, `alloc`, and `compiler_builtins` for custom targets.

```bash
cargo -Zjson-target-spec build
./scripts/qemu-smoke.sh
cargo -Zjson-target-spec test
```

The smoke script boots QEMU `virt` with a Cortex-A76 CPU and expects:

```text
talos: qemu smoke PASS
```

Useful explicit forms:

```bash
cargo +nightly-2026-05-20 -Zjson-target-spec -Zbuild-std=core,compiler_builtins,alloc -Zbuild-std-features=compiler-builtins-mem build --target targets/aarch64-talos-virt.json
cargo -Zjson-target-spec run
```

The linker map is emitted at `target/talos-aarch64-virt.map`.

## Raspberry Pi 5 Boot Artifacts

Build and review a Pi 5 boot archive with:

```bash
./scripts/rpi5-image.sh
./scripts/rpi5-boot-tree.sh /path/to/pi-firmware-boot-source target/rpi5-boot-tree
tar -C target/rpi5-boot-tree -czf target/talos-rpi5-boot.tar.gz .
./scripts/rpi5-archive-review.sh target/talos-rpi5-boot.tar.gz
```

Publishing and power cycling are deliberately separate lab actions. The staging
scripts should not imply hardware success; task notes should record the actual
serial and TFTP evidence.

## Documentation

Project docs live in `docs/src/` and are built with mdBook:

```bash
mdbook build
```

Useful entry points:

- `docs/src/vision.md`
- `docs/src/roadmap.md`
- `docs/src/project/operating-model.md`
- `docs/src/project/testing-strategy.md`
- `docs/src/project/lab-controller.md`

## Repository Hygiene

Before committing or pushing:

```bash
git status --short
git diff --check
cargo fmt --check
cargo -Zjson-target-spec test
./scripts/qemu-smoke.sh
mdbook build
```

Do not commit generated output, boot archives, images, private OpenClaw memory,
controller credentials, private keys, tokens, or machine-local deployment state.

## License

Talos is licensed under the MIT License. See `LICENSE`.
