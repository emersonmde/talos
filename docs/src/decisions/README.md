# ADR Index

Architectural decision records live here.

Create an ADR when a decision is expensive to reverse, affects subsystem boundaries, constrains future POSIX compatibility, or changes the hardware lab contract.

ADR template:

- Status:
- Context:
- Decision:
- Consequences:
- Alternatives considered:

## 2026-05-18 - Repository and Target Strategy

- Status: accepted
- Context: Talos needs to be a standalone project that Matthew can later push to GitHub, while still allowing fast generic kernel iteration before physical Pi 5 serial feedback is available.
- Decision: Keep Talos in its own git repository under `projects/talos`. Build the kernel as a Rust no_std project with a generic AArch64/QEMU virt target for fast validation and a separate `talos-rpi5-bcm2712` target for the first physical board.
- Consequences: Generated artifacts stay out of git. QEMU results may validate generic architecture and toolchain behavior, but physical Pi 5 claims require lab-controller and serial evidence.
- Alternatives considered: continue evolving Daedalus directly, make the project Pi 5-only without a generic target split, or delay repo setup until first hardware boot. Those options would make the design harder to review, harder to publish, or more likely to mix Pi 4 assumptions into Talos.
