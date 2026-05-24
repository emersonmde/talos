# Pi 5 Secondary-Core Controlled Workload Proof Evidence

Task: `phase6-secondary-core-controlled-kthread-workload-20260524`

Accepted candidate archive:

- Archive: `target/talos-rpi5-secondary-core-workload-boot.tar.gz`
- Archive SHA256: `73e7419eef2ddc0e5ba6a4ac3756d5c0b1d0c2f5b6888b7759b9b921f6621fa7`
- Kernel SHA256: `a0ecfe8fef7ad4d144ed68ceefeadf325c4a5fa3ca9cb7b703f7c6e6927d8092`
- Kernel size: 91,288 bytes

Hardware run:

- Hardware lock: acquired at `2026-05-24T20:24:07Z` in durable state before archive publish and power cycle.
- Pre-run snapshot: `pre-phase6-pi5-secondary-workload-20260524T202437Z`.
- Publish proof: `publish.json` and `post-publish-status.json` show root and serial-prefixed `kernel_2712.img` at 91,288 bytes.
- TFTP proof: `tftp-delta-before-restore.json` shows `10.42.1.4` fetched `da591740/kernel_2712.img` twice at 91,288 bytes before restore.
- Serial proof: `serial-observe.json` shows cores 1, 2, and 3 each reached `workload-complete`, reported MPIDR affinities `0x100`, `0x200`, and `0x300`, owned stack pointers, `progress=64 target=64 ok=true`, classification `pi5-secondary-core-controlled-workload-complete`, and `PASS`.
- Restore proof: `restore-pre-snapshot.json` and `post-restore-status.json` show the pre-run snapshot restored to the prior 82,045-byte boot tree.

Classification: `pi5-secondary-core-controlled-workload-complete`.

This is a diagnostic-only controlled secondary-core workload. It does not promote the production scheduler to SMP, share run queues, introduce migration or load balancing, define SMP locks, or add userspace, descriptors, filesystem, networking, SSH, or shell behavior.
