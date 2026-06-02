# No-Fresh-TFTP Supervisor Intervention Checklist

Task: phase10-pi5-tftp-fetch-freshness-control-discriminator-20260602
Completed-at: 2026-06-02T05:19:00Z
Classification: supervisor-intervention-checklist

## First-Principles Problem Statement

The lab is producing fresh Raspberry Pi firmware/RP1 serial reboot bytes after
fixed-port power cycles, but accepted control and candidate runs are producing
zero fresh `/tftp/logs` events from the pre-run cursors captured before those
power cycles. Those facts cannot both satisfy the current Pi 5 proof invariant:
if the Pi is booting the newly published network boot tree through the lab TFTP
service, the TFTP log should advance with `config.txt` and selected kernel
requests before any restore. Fresh serial proves the board power-cycled and the
UART path is alive; it does not prove that the board fetched the published TFTP
tree or that `/tftp/logs` is observing the fetch path.

The immediate unknown is therefore not Talos shell behavior, cat banner output,
or prompt responsiveness. The immediate unknown is the boot-source/logging
chain: whether the Pi is still selecting network boot first, whether it can
reach the lab DHCP/TFTP path, whether it is falling back to another boot source,
whether the lab TFTP log cursor is missing or losing events, or whether the
published tree/prefix is not the tree the firmware is attempting to load.

## Expected Invariant

With EEPROM network boot selected first, after publishing a complete boot tree
and power-cycling the fixed Pi 5 port, the Pi should request `config.txt` and
the selected kernel from the lab TFTP service. A
`/tftp/logs?cursor=<pre-run-cursor>` query captured before the power cycle
should then advance and include fresh boot-file requests before any restore or
rollback.

## Contradicting Evidence

- `phase10-pi5-prompt-baseline-after-cat-blocker-20260602` local1 retained
  708 bytes of fresh firmware/RP1 serial output after a fixed-port power cycle,
  but `tftp-cursor-start=4035548`, `tftp-kernel-fetch-object-count=0`, and no
  fresh TFTP events or prompt/PASS evidence.
- `phase10-pi5-prompt-baseline-after-cat-blocker-20260602` local2 retained
  708 bytes of fresh firmware/RP1 serial output after a fixed-port power cycle,
  but `tftp-cursor-start=4036899`, `tftp-line-count=0`, and no fresh TFTP
  events or prompt/PASS evidence after a longer settle window.
- `phase10-pi5-tftp-fetch-freshness-control-discriminator-20260602` retained
  708 bytes of fresh firmware/RP1 serial output from serial cursor `3996319`,
  but `/tftp/logs?cursor=4038250` returned `cursor_start=4038250`,
  `cursor_end=4038250`, `events=[]`, and `lines=[]`.
- All three attempts used accepted prompt-capable literal-echo control
  artifacts and restored the pre-run boot tree afterward, but none proved a
  fresh selected-control TFTP fetch.

## Unproven Assumptions

- EEPROM `BOOT_ORDER=0xf12` is still active on the physical Pi and network boot
  is still selected before SD/local fallback.
- SD/local fallback is absent, disabled, or not the source of the observed
  firmware reboot output.
- The Pi can still reach the lab DHCP/TFTP path from the fixed port after a
  power cycle.
- dnsmasq request logs are still visible through the lab-controller
  `/tftp/logs` endpoint.
- `/tftp/logs` cursor semantics remain valid under log rotation, truncation,
  or service restart; the captured `cursor_end` is a safe pre-run delta cursor.
- `/boot/archive` publication and the reported boot `tree_hash` are the tree
  and prefix that the Pi firmware is actually attempting to fetch.
- `config.txt`, `kernel_2712.img`, and any fallback `kernel8.img` naming in the
  published archive match the firmware-selected path for this boot.
- Fresh firmware/RP1 serial output corresponds to a TFTP-backed reboot rather
  than a non-TFTP boot source or an early bootloader path that never reaches
  the lab TFTP server.

## Qualitatively Different Approaches Before Any More Pi 5 Rerun

1. Restore or publish a known-good network-boot/Linux snapshot and capture a
   fresh `/tftp/logs` cursor before a fixed-port power cycle. This tests the
   lab boot-source/logging invariant with an artifact outside the Talos
   candidate/control proof archives.
2. Run an independent lab-side TFTP/logging diagnostic that does not depend on
   a Talos candidate/control archive, such as validating dnsmasq log growth,
   cursor behavior across a synthetic TFTP request, and whether `/tftp/logs`
   reports that request with the expected cursor advancement.
3. Inspect or prove the boot source independently of Talos proof output, for
   example by using a boot-source diagnostic tree whose only purpose is to
   force observable `config.txt`/kernel request evidence or to show SD/local
   fallback behavior.

## Smallest Decisive Discriminator

The smallest decisive discriminator is a boot-source/TFTP check that proves one
of these outcomes without changing Talos runtime behavior or proof criteria:

- after a fixed-port Pi 5 power cycle, `/tftp/logs?cursor=<pre-run-cursor>`
  advances and includes fresh `config.txt` plus selected-kernel requests for a
  known-good non-Talos or explicitly diagnostic network-boot tree; or
- the Pi is rebooting from a non-TFTP source, cannot reach the lab TFTP path,
  or `/tftp/logs` is not observing requests even when an independent TFTP
  request should be visible.

Until one of those outcomes is recorded, Pi 5 candidate/control reruns cannot
support hardware acceptance for local shell features because the lab has not
proved it is booting the tested artifact.

## Workaround Quarantine And Removal Plan

No Talos runtime behavior, cat-banner command behavior, parser behavior,
filesystem/syscall behavior, proof-harness visibility shim, candidate archive,
settle window, marker name, or acceptance criterion should be changed to
compensate for no-fresh-TFTP evidence. Existing cat-banner and accepted-control
archives remain quarantined as evidence, not as inputs for more reruns.

The quarantine can be removed only after a later explicit supervisor task
records either fresh TFTP `config.txt`/kernel request evidence from a
fixed-port Pi 5 reboot or a documented lab/boot-source diagnosis explaining why
fresh firmware serial output is not accompanied by `/tftp/logs` advancement.

## Follow-up Cursor Replay

A later lab-side replay changed the conclusion. Re-querying saved cursor
`4038250` returned the 2026-06-02 03:54:59-03:55:01 TFTP request sequence
from `10.42.1.4`, including `da591740/config.txt`,
`da591740/kernel_2712.img`, `da591740/bcm2712-rpi-5-b.dtb`, overlays, and
`cmdline.txt`. The immediate zero-delta artifact should therefore be treated
as a TFTP log collection timing problem, not as decisive proof that the Pi
skipped the lab TFTP boot source.

The active follow-up record is
`tasks/2026-06-02-phase10-pi5-boot-source-tftp-invariant-analysis.md`. Future
no-fresh-TFTP classifications must use settled repeated cursor queries and
live-tail health checks before treating TFTP absence as a hardware fact.
