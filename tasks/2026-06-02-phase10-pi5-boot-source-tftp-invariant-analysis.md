# Phase 10 Pi 5 Boot Source and TFTP Invariant Analysis

Task: phase10-pi5-boot-source-tftp-invariant-analysis-20260602
Status: resolved-log-timing-discriminator-selected

## Problem Statement

The 2026-06-02 Pi 5 control attempts reached an impossible-looking state: fixed-port power cycles produced fresh Raspberry Pi firmware/RP1 serial output, but immediate TFTP cursor deltas reported no fresh events. If taken literally, that would mean the Pi rebooted without asking the configured network-boot TFTP service for config, DTB, overlay, cmdline, or kernel files.

That literal interpretation is now contradicted by a later lab-side replay of the saved discriminator cursor. The saved pre-run cursor for phase10-pi5-tftp-fetch-freshness-control-discriminator-20260602 was 4038250. The immediate retained artifact returned cursor_start=4038250, cursor_end=4038250, events=[], and lines=[]. A later query of the same cursor returned fresh 2026-06-02 03:54:59-03:55:01 TFTP lines from 10.42.1.4, including da591740/config.txt, da591740//config.txt, da591740/kernel_2712.img, da591740/bcm2712-rpi-5-b.dtb, da591740//overlays/overlay_map.dtb, da591740//overlays/bcm2712d0.dtbo, and da591740//cmdline.txt.

This changes the failure boundary. The strongest current explanation is not that the Pi rebooted from a non-TFTP source. It is that the immediate TFTP evidence collector queried before dnsmasq log data was visible through the lab API, then classified the empty immediate delta too early. The after-restore replay cannot be used for served-byte-size acceptance because the endpoint computes bytes from the current TFTP tree, but the raw dnsmasq lines are sufficient to prove that the fixed-port reboot did issue fresh TFTP boot-file requests.

## Expected Invariant

With EEPROM network boot selected first, after publishing a boot tree and power-cycling the fixed Pi 5 port, the Pi should request config.txt and the selected kernel from the lab TFTP service. A later TFTP log query from the pre-run cursor should advance and include boot-file requests from 10.42.1.4 before any restore-dependent size interpretation is made.

The invariant must be sharpened: an empty immediate delta is not decisive unless the collector has also proved that the TFTP log cursor has settled after the expected boot window. The endpoint cursor is a log-observation cursor, not a hardware fact by itself.

## Contradicting Evidence

- phase10-pi5-prompt-baseline-after-cat-blocker-20260602 local1 and local2 captured fresh firmware/RP1 serial reboot output but retained zero immediate TFTP events from their pre-run cursors.
- phase10-pi5-tftp-fetch-freshness-control-discriminator-20260602 captured fresh firmware/RP1 serial reboot output from serial cursor 3996319, changed the boot tree from a045245... to b742bba..., and retained an immediate TFTP delta from cursor 4038250 with zero events.
- Re-querying the same TFTP cursor later returned fresh 03:54:59-03:55:01 requests from the Pi for config, kernel, DTB, overlays, and cmdline.

## Unproven Assumptions

- EEPROM BOOT_ORDER=0xf12 is still the active EEPROM behavior on every reboot.
- SD/local fallback behavior cannot emit the same fresh firmware/RP1 serial prefix before network boot resumes.
- DHCP/TFTP reachability from the Pi is continuous during the first seconds after fixed-port power is restored.
- dnsmasq writes and flushes TFTP log lines synchronously enough for an immediate post-observe TFTP call to be decisive.
- TFTP cursor semantics are stable under truncation, buffering, and delayed writes.
- Served bytes in replayed old events reflect the boot tree active at request time. They do not; they reflect the current TFTP tree when the API parses the log.
- The published tree/prefix selection is the same for root and da591740/ paths in every control archive.
- Firmware-only serial output proves kernel entry. It does not.

## Qualitatively Different Approaches Considered

- Known-good network-boot/Linux snapshot control: restore or publish a pinned Linux/network-boot tree, power-cycle once, and require fresh TFTP cursor advancement plus SSH/Linux evidence.
- Independent lab TFTP/logging diagnostic outside Talos archives: use saved cursors and repeated TFTP log replays to prove whether immediate zero deltas can become non-empty after delay, without changing Talos code or consuming another Pi reboot.
- Boot-source diagnostic archive: stage an armstub or serial-prefixed diagnostic only if the log-timing question remains unresolved.

## Selected Discriminator

Use the independent lab TFTP/logging discriminator first.

For any future hardware run, capture the pre-run TFTP cursor with omitted cursor. After the expected boot window, query logs from that cursor. If the result is empty, do not classify no-fresh-TFTP immediately; re-query the same cursor after a short settle loop until either cursor_end advances with boot-file lines or a bounded timeout expires. Treat a delayed non-empty replay as TFTP-present/log-timing-delayed. Only treat no-fresh-TFTP as decisive if repeated settled queries stay empty and a live tail query shows the log endpoint itself is healthy.

The existing saved-cursor replay already proves the discriminator matters: cursor 4038250 was empty in the immediate artifact and non-empty later.

## Quarantine Plan

Do not alter Talos runtime behavior, cat-banner command behavior, marker names, candidate archives, proof acceptance criteria, or kernel-visible waits to work around no-fresh-TFTP artifacts. The fix belongs in the lab evidence collection procedure: TFTP absence must require settled cursor evidence, and old-event byte sizes must not be used after restore.

The cat-banner proof remains blocked on prompt/control behavior, not on a literal no-TFTP boot-source claim. The next worker may resume planning from the accepted local-interactivity feature path after carrying this TFTP settle rule into the hardware evidence checklist.
