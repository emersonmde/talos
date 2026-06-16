# Lab Controller

Talos uses a narrow lab controller to automate the Raspberry Pi 5 boot loop.
OpenClaw should treat this as the control surface for physical Pi operations.

## Current Shape

The deployed lab service lives outside the Talos repository. In the current lab,
agents reach it through the internal Docker service name:

```text
http://talos-lab-api:8080
```

OpenClaw does not need direct access to the service's host directory or network
controller credentials.

The API owns:

- Network-controller credentials.
- The fixed PoE switch/port mapping.
- Power-cycle actions.
- TFTP boot archive publishing.
- One-archive rollback.
- Serial console access through the attached USB UART cable.

OpenClaw should not call the network controller directly and should not ask for
controller keys.

## Target Facts

```text
talos-pi5: 10.42.1.4
pi5 MAC:   88:a2:9e:ae:c8:7f
prefix:    da591740
serial:    115200 baud
```

Power control is intentionally fixed-port. `POST /power/cycle` always sends the
configured controller action to the configured PoE switch port. It does not
depend on a live client record for `10.42.1.4`, because failed kernel or
bootloader states may leave Talos absent from the client list exactly when a
power cycle is needed.

The API still checks that the configured switch exposes the configured port and
that PoE is enabled on that port. It does not try to discover or choose a port
dynamically.

## API Commands

Health:

```bash
curl -fsS http://talos-lab-api:8080/health
```

Status:

```bash
curl -fsS http://talos-lab-api:8080/status
```

Important `status.boot` fields:

```text
active_name         configured fallback name from lab config
configured_kernel  kernel= value parsed from current config.txt, if present
effective_kernel   configured_kernel when present, otherwise active_name
config             parsed config.txt keys: kernel, armstub, boot_ramdisk,
                   enable_rp1_uart, os_check
tree_hash          SHA-256 over the visible current TFTP boot tree
snapshots          named snapshot list
```

Power-cycle the Pi:

```bash
curl -fsS -X POST http://talos-lab-api:8080/power/cycle
```

List published boot files:

```bash
curl -fsS http://talos-lab-api:8080/boot/files
```

Publish a complete TFTP boot tree:

```bash
tar -C /path/to/boot-tree -czf /tmp/talos-boot.tar.gz .
curl -fsS -X PUT --data-binary @/tmp/talos-boot.tar.gz http://talos-lab-api:8080/boot/archive
rm -f /tmp/talos-boot.tar.gz
```

Rollback to the previous boot tree:

```bash
curl -fsS -X POST http://talos-lab-api:8080/boot/rollback
```

Create or restore a named boot snapshot:

```bash
curl -fsS -X POST 'http://talos-lab-api:8080/boot/snapshot?name=known-good-linux'
curl -fsS 'http://talos-lab-api:8080/boot/snapshots'
curl -fsS -X POST 'http://talos-lab-api:8080/boot/restore?name=known-good-linux'
```

The API keeps one rolling rollback archive after each publish or restore. Named snapshots are separate from that rolling rollback and are better for pinned restore points such as `known-good-linux`. Do not create a snapshot named `known-good-*` unless the current boot tree has actually been validated as good.

The API removes upload/staging files after each publish attempt. Do not leave large boot tarballs in the OpenClaw workspace after upload.

Power-cycle response shape:

```text
POST /power/cycle
Response:
  action, ok, controller.guard.mode, controller.guard.port_idx,
  controller.guard.poe_state, controller.response
```

Expected guard mode is `fixed-port`. A 400 from this endpoint means the fixed
controller device/port/PoE lookup or the controller action failed, not that
Talos was missing from the client list.

Boot snapshot endpoint reference:

```text
GET /boot/snapshots
Response:
  action, ok, snapshots[{name, bytes, mtime}]

POST /boot/snapshot?name=<snapshot-name>
Query:
  name: required; letters, digits, dot, underscore, hyphen; max 80 chars
Response:
  action, ok, snapshot{name, archive, bytes}, snapshots[]

POST /boot/restore?name=<snapshot-name>
Query:
  name: required
Response:
  action, ok, boot, archive{name, files, file_count, extracted_bytes, rollback_archive}
```

Current verified behavior:

- `GET /status` reports guard `fixed-port`, a configured port index, and `poe_state=UP`.
- `GET /` is not an authoritative boot identity endpoint for the deployed lab
  API. If a proof needs boot identity, use `GET /status` and retain the full
  response. A `404 unknown endpoint: GET /` result is endpoint-semantics
  evidence, not proof of a broken boot tree.
- `POST /power/cycle` succeeds without consulting the live client list for `talos-pi5`.
- After a successful power cycle, serial may emit Raspberry Pi firmware/RP1 boot messages before any kernel output. Treat firmware output as proof of reboot and serial wiring, not proof that Talos reached entry.

Serial peek, read, write, and observe:

```bash
curl -fsS 'http://talos-lab-api:8080/serial/peek?max_bytes=500&drain=true'
curl -fsS -X POST -H 'Content-Type: application/json' \
  --data '{"text":"uname -a","append_newline":true}' \
  http://talos-lab-api:8080/serial/write
curl -fsS -X POST -H 'Content-Type: application/json' \
  --data '{"cursor":0,"timeout_seconds":5,"settle_ms":250,"max_bytes":4096}' \
  http://talos-lab-api:8080/serial/observe
```

`peek` returns the latest retained serial log bytes without consuming them. It also returns a `cursor` byte offset. The best agent loop is:

1. `GET /serial/peek?max_bytes=500&drain=true` and save `cursor`.
2. If the text shows a prompt or login state, `POST /serial/write`.
3. `POST /serial/observe` with the saved cursor to get only bytes that appeared after that point.
4. Repeat `peek` as needed. Repeated `peek` calls return the same tail until new serial data arrives.

Use `observe`, not `transact`, for kernel logs and unknown prompts. `transact` is only a convenience helper for known interactive prompts because it depends on regex matching.

## Boot Archive Contract

`/boot/archive` accepts a gzip-compressed tar archive. The archive root becomes the Pi's TFTP boot root.

Required files:

```text
config.txt
cmdline.txt
bcm2712-rpi-5-b.dtb
kernel_2712.img or kernel8.img
```

Useful Pi OS Lite boot tree files also include:

```text
initramfs_2712
overlays/
start*.elf
fixup*.dat
```

For early Talos bring-up, include both `kernel_2712.img` and a duplicate
`kernel8.img` until the lab loop proves which firmware path is selected on every
network boot. The Pi 5 default is `kernel_2712.img`. Use
`GET /status` and check `boot.configured_kernel` / `boot.effective_kernel`
rather than relying on `boot.active_name`, which is only the lab config fallback.

The API rejects unsafe archives:

- Absolute paths.
- `..` traversal.
- Hidden path components.
- Duplicate files.
- Symlinks and hardlinks.
- Device files and FIFOs.
- Archives missing required Pi 5 boot files.
- Archives over configured size/file-count limits.

## Network Boot Configuration

The Pi EEPROM is configured with:

```text
BOOT_ORDER=0xf12
```

Read right-to-left:

```text
2 = network boot first
1 = SD card fallback
f = restart loop
```

The lab network boot configuration points the Pi at the internal TFTP service:

```text
Network Boot filename: config.txt
TFTP Server:           internal lab TFTP host
```

Strider firewalld allows UDP/69 only from the Pi:

```text
10.42.1.4/32 -> UDP/69
```

TFTP is served by `talos-tftp` using dnsmasq in TFTP-only mode. OpenClaw should use the API for TFTP request logs instead of host Docker logs:

```bash
curl -fsS 'http://talos-lab-api:8080/tftp/logs?cursor=0&limit=200'
```

TFTP log endpoint reference:

```text
GET /tftp/logs
Query:
  cursor: optional integer byte offset; omit to read the current log tail
  max_bytes: integer, default 65536, range 1..1048576
  limit: integer line/event limit, default 200, range 1..2000
Response:
  action, ok, tftp.log, tftp.cursor_start, tftp.cursor_end,
  tftp.log_size, tftp.tail_mode, tftp.truncated, tftp.lines[], tftp.events[]

tftp.events[] fields:
  status: served or not_found
  filename: requested TFTP path relative to the TFTP root
  client_ip: requester IP
  client_mac: known target MAC when client_ip is 10.42.1.4, otherwise null
  bytes: current file size for served files when available, otherwise null
  line: raw dnsmasq log line
```

Use `cursor_end` like the serial cursor. To capture the current end of the TFTP log before a power cycle, omit `cursor`:

```bash
cursor="$(curl -fsS 'http://talos-lab-api:8080/tftp/logs?limit=1' | jq -r .tftp.cursor_end)"
```

After the run, call `/tftp/logs?cursor=<old cursor>` to see only new TFTP activity. Hardware proofs must treat this as a stability check, not a single sample: re-query from the same cursor until `cursor_end`, `log_size`, `truncated`, and the parsed event set are unchanged for the required number of samples, or until the bounded timeout is reached. `scripts/rpi5-wait-tftp-delta.sh <cursor> [timeout_seconds] [stable_samples]` implements that rule and annotates its output with `talos_tftp_stability`; exit 0 means a stable non-empty delta, exit 1 means either stable zero events or timeout. A zero-event TFTP delta is meaningful evidence only when it is stable under that rule and is recorded before the boot tree is restored.

For acceptance evidence that depends on served file sizes, query stable TFTP logs before restoring the boot tree. The `bytes` field is computed from the current TFTP file at query time, not parsed from the dnsmasq line, so querying after restore can label an earlier diagnostic serve with the restored file's size. Keep `limit` within the endpoint range, currently `1..2000`, or the request fails and can accidentally push evidence collection until after restore.

## Pi 5 Hardware Proof Record Contract

Every serialized Pi 5 hardware proof must retain one deterministic evidence
bundle before classifying the run. The authoritative boot identity sample is
`GET /status`; record `boot.tree_hash`, `boot.effective_kernel`,
`boot.configured_kernel`, `boot.config`, guard fields, and snapshot state.
Also retain `GET /boot/files` and `GET /boot/snapshots` before any power
cycle so the visible TFTP root and restore candidates are reviewable.

Before the power cycle, retain fresh serial and TFTP cursors:

~~~bash
serial_cursor="$(curl -fsS 'http://talos-lab-api:8080/serial/peek?max_bytes=500&drain=true' | jq -r .cursor)"
tftp_cursor="$(curl -fsS 'http://talos-lab-api:8080/tftp/logs?limit=1' | jq -r .tftp.cursor_end)"
~~~

After the power cycle, observe serial from the saved serial cursor and query
`scripts/rpi5-wait-tftp-delta.sh "$tftp_cursor"` before restoring the boot
tree. Known-good runtime readiness must use an explicit bounded serial
observation instead of a default observe call. The helper loops until the
requested deadline while advancing the serial cursor and accumulating output,
because a single `/serial/observe` request can return after a quiet firmware
burst before the later TFTP/kernel/readiness output appears.

~~~bash
scripts/rpi5-observe-runtime-readiness.sh "$serial_cursor" 75 1000 65536
~~~

For accepted evidence, retain the primary helper JSON through the repository
wrapper so a later follow-up read cannot replace it:

~~~bash
scripts/rpi5-retain-runtime-readiness-primary.sh \
    "$evidence_dir" "$run_label" "$serial_cursor" 75 1000 65536
~~~

The wrapper writes run-label-qualified immutable artifacts:
`$run_label-runtime-readiness-primary.json`,
`$run_label-runtime-readiness-primary-summary.json`, and
`$run_label-runtime-readiness-primary.status`. It refuses to overwrite any of
those paths. Follow-up direct-read or endpoint discriminator evidence must use
separately named artifacts and cannot satisfy or erase the primary readiness
gate. Check the summary only as a derivative of the retained primary JSON, then
check `talos_runtime_readiness.observe_contract` in that primary artifact.
For unsaturated cursors the contract remains
`deadline-loop-accumulated-from-fresh-cursor`. If the saved cursor is already
at the lab controller retention cap, the helper switches to direct
`/serial/read` and records
`deadline-loop-direct-read-after-saturated-cursor`,
`start_cursor_saturated=true`, and `response_bytes`. An empty saturated
direct-read result is `saturated-cursor-capture-blocked`, not an unqualified
known-good runtime-readiness miss. A settled first firmware burst is not the
full readiness window.

For capture-invariant proof bundles that call
`scripts/rpi5-observe-serial-window.sh`, the default auto mode switches from
cursor-based `/serial/observe` to direct `/serial/read` when the saved cursor is
at the lab controller retention cap. Those records must carry
`observe_contract=deadline-loop-direct-read-after-saturated-cursor`; this is the
repository-side repair for cursor saturation until the lab endpoint exposes a
monotonic cursor beyond the retained log cap.

For marker/reset and other focused candidate proofs that need to separate
serial-only firmware reboot, TFTP capture blindness, staging mismatch, and real
candidate progress, use the capture-invariant bundle helper after staging the
intended boot tree:

~~~bash
scripts/rpi5-capture-invariant-proof-bundle.sh \
  --evidence-dir tasks/evidence/<task-id> \
  --restore-snapshot <pre-run-snapshot-name> \
  --label <proof-label> \
  --expected-tree-hash <post-publish-tree-hash> \
  --expected-fetch da591740/kernel_2712.img \
  --expected-fetch-bytes <candidate-kernel-bytes> \
  --serial-marker <candidate-marker>
~~~

The helper writes a deterministic proof bundle with pre-run status/files,
snapshots, an explicit pre-power serial drain, fresh serial and TFTP cursors,
bounded accumulated serial output, stable same-cursor TFTP evidence before
restore, final pre-restore status/files, restore status/files, and
`capture-invariant-summary.json`. The pre-power drain uses `/serial/read`, not
`/serial/peek`, because `peek` reports the retained log tail and does not
consume retained bytes. Its summary suggests only the
capture/observability classification; the task record remains responsible for
accepting or rejecting the feature boundary.

When a retained candidate and known-good control both fail only because the
pre-power serial drain did not reach an empty read, the next retry must make
that freshness discriminator explicit instead of repeating the default fixed
drain. Pass bounded drain options such as
`--serial-drain-attempts 96 --serial-drain-read-timeout 1 --serial-drain-settle-ms 100 --serial-drain-max-bytes 65536`
and retain the dry-run/proof-bundle metadata. A retry may accept saturated
direct-read serial only when `serial-drain-before-power.json` records
`discriminator=empty-read-before-power`; `bounded-drain-exhausted-before-power`
is still a capture-chain blocker, not hardware behavior.

The Phase 12 serial freshness contract
phase12-rp1-ethernet-serial-freshness-contract-v1 refines that rule for future
marker-only transport proofs. Empty pre-power drain remains strong positive
evidence, but cursor/nonce freshness is the stricter replacement for a hard
empty-drain gate. If the drain remains non-empty, the proof must record a
run-unique marker or nonce that is absent from the immediate pre-power retained
sample, then present in post-power serial captured from the saved cursor or the
saturated-cursor direct-read fallback for that same attempt. The same proof must
still retain stable same-cursor TFTP delta before restore, selected-tree
identity, final pre-restore identity, and restore proof. Without those fields,
bounded-drain-exhausted-before-power remains a rejection class and cannot be
used as Ethernet or register evidence.

The capture-invariant bundle now records the retained pre-power serial sample as
`pre-power-serial-peek.json` before the bounded `/serial/read` drain. For
proofs that depend on cursor-nonce freshness, replay the retained bundle through:

~~~bash
scripts/rpi5-serial-freshness-guard-v1-check.sh \
  --evidence-dir tasks/evidence/<task-id>/<run-dir> \
  --label <proof-label> \
  --nonce <run-unique-nonce>
~~~

Exit 0 means the bundle satisfies
`cursor-nonce-post-power-freshness-v1` for serial/capture-chain identity only.
Exit 1 records one or more rejection classes: stale backlog, cursor mismatch,
missing marker, nonce-not-unique, selected-tree/TFTP mismatch, final identity
mismatch, restore failure, saturated direct-read without nonce proof, or
inconclusive capture. This guard does not accept BCM54213PE register values,
link readiness, packet I/O, networking, SSH, Phase 12.2, or a phase transition.

Before a proof task accepts a decisive RP1 hardware classification from that
bundle under the older empty-drain contract, replay the retained files through:

~~~bash
scripts/rpi5-proof-identity-join-check.sh \
  --evidence-dir tasks/evidence/<task-id>/<run-dir> \
  --label <proof-label>
~~~

The checker enforces the `pi5-capture-transaction-v2` contract for older
empty-drain proof bundles: one shared run label must tie the selected tree hash, effective
kernel, expected fetch path and byte count, an empty pre-power serial drain,
serial cursor/window identity, stable TFTP cursor/delta identity, final
pre-restore identity, and restore identity. Missing fields or byte mismatches
classify the bundle as `capture-staging-blocked`; they do not support
`mapped/read-value`, trap/unmapped, or other decisive RP1 behavior. When the
serial cursor is saturated and the helper falls back to direct read, the
direct-read output is decisive only if `serial-drain-before-power.json` proves
the pre-power `/serial/read` drain reached an empty device-buffer read.

The `pi5-capture-transaction-v3` replay checker is a narrower successor for
the observed GPIO14 STATUS/CTRL capture-freshness blocker, where both the real
candidate and a known-good control exhausted the bounded pre-power drain while
retaining matching TFTP/final-identity evidence. Use the same proof-bundle
helper and bounded drain parameters, then replay the retained files through:

~~~bash
scripts/rpi5-proof-identity-join-v3-check.sh \
  --evidence-dir tasks/evidence/<task-id>/<run-dir> \
  --label <proof-label>
~~~

If a same-shaped proof has already failed V3 because the constant required
marker was present before power, the next retry must use a run-unique marker.
For the observed GPIO14 STATUS/CTRL diagnostic, generate one nonce per staged
archive, embed it at build time, and include that nonce in the capture marker:

~~~bash
nonce="$(date -u +%Y%m%dT%H%M%SZ)-$(git rev-parse --short HEAD)"
TALOS_CAPTURE_NONCE="$nonce" \
  scripts/rpi5-rp1-observed-gpio-status-no-mmio-control-archive.sh \
  <pi-firmware-boot-source> \
  target/talos-rpi5-rp1-observed-gpio-status-no-mmio-control.tar.gz

scripts/rpi5-rp1-observed-gpio-status-no-mmio-control-review.sh \
  target/talos-rpi5-rp1-observed-gpio-status-no-mmio-control.tar.gz \
  --capture-nonce "$nonce"

scripts/rpi5-capture-invariant-proof-bundle.sh \
  --evidence-dir tasks/evidence/<task-id>/<run-dir> \
  --restore-snapshot <pre-run-snapshot-name> \
  --label <proof-label> \
  --expected-tree-hash <post-publish-tree-hash> \
  --expected-fetch da591740/kernel_2712.img \
  --expected-fetch-bytes <candidate-kernel-bytes> \
  --serial-marker "capture-nonce=$nonce" \
  --serial-drain-attempts 96 \
  --serial-drain-read-timeout 1 \
  --serial-drain-settle-ms 100 \
  --serial-drain-max-bytes 65536

scripts/rpi5-proof-identity-join-run-unique-check.sh \
  --evidence-dir tasks/evidence/<task-id>/<run-dir> \
  --label <proof-label> \
  --nonce "$nonce"
~~~

The run-unique checker keeps V3's selected-tree, TFTP, final-identity, and
restore checks, then requires the run-unique `capture-nonce=` token to be absent
from all retained pre-power serial drain responses and present in the post-power
serial window. Exact marker matching may still pass, but field ordering inside
the Talos marker line is not part of the freshness contract. This is still a
capture-freshness discriminator only; it does not accept GPIO ownership, event
generation, interrupt delivery, broad RP1 mapping, or any phase transition.

Before accepting any marker-visible run-unique serial output, also replay the
same retained bundle through the boot-staging identity discriminator:

~~~bash
scripts/rpi5-boot-staging-identity-check.sh \
  --evidence-dir tasks/evidence/<task-id>/<run-dir> \
  --baseline-tree-hash <pre-run-restored-tree-hash>
~~~

The staging discriminator ignores serial and RP1 output. It only accepts when
the selected pre-power tree, expected `da591740/kernel_2712.img` fetch bytes,
stable pre-restore TFTP delta, final pre-restore tree, and restore identity all
join to the same candidate tree. If TFTP serves the baseline-sized kernel or
the final pre-restore status has returned to the baseline tree, classify the
run as `boot-staging-identity-blocked` and run a no-RP1/no-MMIO known-good
staging control before retrying a real RP1/GPIO candidate.

V3 keeps the v2 selected-tree, effective-kernel, expected-fetch, TFTP, final
pre-restore, and restore checks. It only changes the saturated serial freshness
gate: a non-empty bounded pre-power drain may be accepted when the serial window
used `deadline-loop-direct-read-after-saturated-cursor`, the required marker is
absent from every retained pre-power drain response, and that same marker is
present after power. If the required marker is already present before power, V3
classifies the run as `capture-staging-blocked`; marker-visible direct-read
serial is not enough by itself.

The Phase 11 capture-transaction v2 closeout accepts this proof-chain boundary
as ready for the queued RP1 UART0 FR-read hold-control v2 proof only after a
no-RP1-MMIO sentinel joined selected-tree identity, empty pre-power serial
drain, fresh marker serial, stable pre-restore TFTP, final pre-restore
identity, and restore proof under one run label. This readiness does not by
itself accept any RP1 mapped/read-value, trap/unmapped, or firmware-state
behavior; the RP1 candidate must pass the same v2 join.

For the current restored known-good control, the preferred readiness markers
are `TALOS: kernel_main` plus `rpi5-production-timer-preemption: PASS` within
that observation window after a stable 104,136-byte
`da591740/kernel_2712.img` fetch. If the same fresh serial window omits
`TALOS: kernel_main` but contains
`rpi5-production-timer-preemption: PASS`, the downstream PASS marker is
sufficient for this restored production-timer control because source order
proves it is reachable only after `kernel_main`. If a future accepted
known-good control uses a different success marker, set
`TALOS_READINESS_REQUIRED_MARKER` and record that exact marker in the proof
bundle. If the run is inconclusive, retain one final pre-restore
`GET /status`, `GET /boot/files`, and TFTP-tail or stable-delta sample.
Restore evidence and hardware lock release evidence belong in the same proof
bundle.

Classification rules:

- `valid-known-good-talos-readiness`: `GET /status` and `GET /boot/files`
  identify the expected boot tree, the stable TFTP delta includes the expected
  `kernel_2712.img` fetch before restore, and the bounded serial observation
  either reaches both `TALOS: kernel_main` and the proof-recorded success
  marker or, for the current restored production-timer control, reaches the
  downstream `rpi5-production-timer-preemption: PASS` marker whose source-order
  proof is recorded in
  `phase11-known-good-runtime-marker-boundary-review-core-20260606`.
- `staging-publication-mismatch`: `GET /status` or `GET /boot/files` shows an
  unexpected tree hash, selected kernel, missing boot file, or boot-config
  mismatch before the run.
- `tftp-capture-logging-blindness`: serial proves a network boot attempt from
  the fresh cursor, but the stable pre-restore TFTP delta has zero events while
  `GET /status` and `GET /boot/files` still show the expected boot tree.
- `serial-only-firmware-reboot`: serial reaches Raspberry Pi firmware NETWORK
  output from the fresh cursor, stable TFTP evidence does not show the expected
  fetch, and no Talos readiness marker appears.

Do not shrink a feature proof's original target based on these classifications.
For example, firmware NETWORK serial with stable zero TFTP events does not
accept candidate fetch, Talos entry, RP1 mapped/read-value, RP1 unmapped/trap,
or known-good boot health.

Verified request sequence:

```text
da591740/config.txt -> not found, expected prefix probe
config.txt
bcm2712-rpi-5-b.dtb
kernel_2712.img
initramfs_2712
overlays/overlay_map.dtb
overlays/bcm2712d0.dtbo
overlays/vc4-kms-v3d-pi5.dtbo
cmdline.txt
```

The missing `da591740/config.txt`, `pieeprom.sig`, and `armstub8-2712.bin` requests are not currently fatal. The firmware falls back to the root TFTP directory and continues booting.

## Boot Files vs Root Filesystem

The boot files and the root filesystem are different stages.

Boot files are loaded by Raspberry Pi firmware before Linux starts:

```text
config.txt
cmdline.txt
kernel_2712.img
initramfs_2712
*.dtb
overlays/*.dtbo
```

In the current lab, these boot files come from Strider over TFTP.

The root filesystem is the Linux userspace mounted by the already-running kernel. It contains:

```text
/etc
/home
/usr
/var
systemd
ssh
users
packages
```

Current `cmdline.txt` intentionally uses:

```text
root=/dev/mmcblk0p2
```

That means the firmware loads the kernel, DTBs, overlays, initramfs, and command line from TFTP, then the kernel mounts partition 2 on the SD card as `/`.

The running system does not switch to a kernel from the root filesystem after boot. The kernel that runs is the one loaded by firmware from TFTP. The root filesystem may contain kernel packages and modules under `/lib/modules`, but it does not replace the already-running kernel.

Implications:

- SSH still reports hostname `talos-pi5` because `/etc/hostname` comes from the SD root filesystem.
- Files in `/home/matthew` persist because `/home` is on the SD root filesystem.
- Kernel image changes should be published into the TFTP boot archive.
- If kernel modules are needed, `/lib/modules/<kernel-release>` on the SD root must match the TFTP-loaded kernel.

## Practical Development Loop

For early kernel bring-up:

1. Build the Pi 5 kernel image, DTBs, overlays, and initramfs if needed.
2. Create a boot tree containing `config.txt`, `cmdline.txt`, `kernel_2712.img`, required DTBs, and overlays.
3. Upload it with `PUT /boot/archive`.
4. Power-cycle with `POST /power/cycle`.
5. Watch TFTP logs for boot file requests.
6. Check whether the Pi returns on `10.42.1.4`.
7. Check `uname -a` and `/proc/cmdline` after boot.

This hybrid TFTP-boot/SD-root setup is good for changing boot files without rebuilding a full OS image.

For the lab cable on the 40-pin header, first-light Talos output should target RP1 UART0. The first Talos archive used:

~~~text
enable_uart=1
enable_rp1_uart=1
pciex4_reset=0
uart_2ndstage=1
kernel=kernel_2712.img
os_check=0
dtoverlay=uart0-pi5
~~~

`enable_rp1_uart=1` asks Pi 5 firmware to initialize RP1 UART0 at 115200 bps and preserve that state for early bare-metal output.

The staging script strips `dtoverlay=uart0-pi5` from the Talos archive even if it exists in the Linux boot-source config. Talos first-light writes RP1 UART0 directly and should avoid Linux-only overlay work before the entry marker is visible.

Talos now has local staging scripts for the first Talos boot archive candidate:

```bash
./scripts/rpi5-image.sh
./scripts/rpi5-boot-tree.sh /path/to/pi-firmware-boot-source target/rpi5-boot-tree
tar -C target/rpi5-boot-tree -czf target/talos-rpi5-boot.tar.gz .
./scripts/rpi5-archive-review.sh target/talos-rpi5-boot.tar.gz
```

To test the Pi 5 `boot_ramdisk=1` network-boot path, stage a FAT32 `boot.img`
inside the archive:

```bash
./scripts/rpi5-boot-ramdisk-tree.sh /path/to/pi-firmware-boot-source target/rpi5-boot-tree
tar -C target/rpi5-boot-tree -czf target/talos-rpi5-boot.tar.gz .
./scripts/rpi5-archive-review.sh target/talos-rpi5-boot.tar.gz
```

To test whether firmware reaches a custom armstub before the kernel handoff,
stage the armstub diagnostic tree. This appends `armstub=armstub8-2712.bin`
and includes a tiny AArch64 binary that writes `S1` to RP1 UART0, then waits.
Seeing `S1` proves the configured armstub ran; not seeing it means the current
failure is still earlier than, or outside, that entry path.

```bash
./scripts/rpi5-armstub-diagnostic-tree.sh /path/to/pi-firmware-boot-source target/rpi5-boot-tree
tar -C target/rpi5-boot-tree -czf target/talos-rpi5-boot.tar.gz .
./scripts/rpi5-archive-review.sh target/talos-rpi5-boot.tar.gz
```

To test the EEPROM network-boot serial-prefix lookup path, stage a mirrored boot
tree. This keeps the required files at the archive root and duplicates them
under `da591740/`, matching the Pi's serial-number prefix probe observed in
the known-good TFTP request sequence.

```bash
./scripts/rpi5-prefixed-boot-tree.sh /path/to/pi-firmware-boot-source target/rpi5-boot-tree
tar -C target/rpi5-boot-tree -czf target/talos-rpi5-boot.tar.gz .
./scripts/rpi5-archive-review.sh target/talos-rpi5-boot.tar.gz
```

To test the remaining combined prefix-plus-armstub hypothesis, stage both the
`armstub=armstub8-2712.bin` diagnostic and the `da591740/` mirror in one
archive. This covers the case where the bootloader reads the serial-prefixed
`config.txt` first and therefore would never see an armstub line that exists
only in the root config.

```bash
./scripts/rpi5-prefixed-armstub-diagnostic-tree.sh /path/to/pi-firmware-boot-source target/rpi5-boot-tree
tar -C target/rpi5-boot-tree -czf target/talos-rpi5-boot.tar.gz .
./scripts/rpi5-archive-review.sh target/talos-rpi5-boot.tar.gz
```

Expected project file layout:

```text
scripts/rpi5-image.sh           # builds the Talos Pi 5 kernel/image artifact
scripts/rpi5-boot-tree.sh       # stages firmware/config/cmdline/kernel into a boot tree
scripts/rpi5-boot-img.sh        # creates a plain FAT32 boot.img from a boot tree
scripts/rpi5-boot-ramdisk-tree.sh # stages boot_ramdisk=1 plus boot.img
scripts/rpi5-armstub-diagnostic.sh # builds the S1 custom armstub diagnostic
scripts/rpi5-armstub-diagnostic-tree.sh # stages armstub=armstub8-2712.bin
scripts/rpi5-prefixed-boot-tree.sh # mirrors boot files under da591740/
scripts/rpi5-prefixed-armstub-diagnostic-tree.sh # combines the S1 armstub and da591740/ mirror
scripts/rpi5-archive-review.sh  # checks archive contents and arm64 Image header fields
target/rpi5-boot-tree/          # generated TFTP boot tree; do not hand-edit as source
target/talos-rpi5-boot.tar.gz   # generated upload archive; remove when no longer needed
src/                            # Talos source code
docs/                           # project documentation
tasks/                          # task records and hardware-test notes
```

Treat `target/` as generated output. If a hardware test needs to preserve exact evidence, record the command output and observed serial/TFTP logs in `tasks/`, not by checking generated boot trees into source.

The source directory must contain at least:

```text
config.txt
cmdline.txt
bcm2712-rpi-5-b.dtb
```

The staging script builds Talos as `kernel_2712.img` and copies only a narrow
set of firmware files needed for early boot. It intentionally does not upload,
power-cycle, or claim hardware success. Publishing remains a controlled hardware
test step after acceptance criteria and review.

Later, if Talos needs reproducible userspace, add a generated root filesystem path such as NFS root, iSCSI root, or a generated SD/image root. That future step must explicitly provision:

- `matthew` user.
- SSH authorized keys.
- Sudo behavior.
- Hostname.
- Required packages and tools.

## Serial Status

Serial is live through a Waveshare FTDI USB UART adapter. Strider sees the adapter as:

```text
/dev/serial/by-id/usb-FTDI_FT232R_USB_UART_BG02PSTC-if00-port0
```

The API container mounts that stable host path as:

```text
/dev/talos-serial
```

The deployed config uses:

```text
device: /dev/talos-serial
baud: 115200
log: /state/serial.log
```

The stable `/dev/serial/by-id/...` path should survive host reboot and plugging the same cable into a different USB port. If the cable is unplugged/replugged while `talos-lab-api` is already running, restart that container before relying on serial. A different USB UART cable will likely have a different by-id path and requires a config/compose update.

The serial log is retained inside Talos Lab state and capped to the most recent bytes so it does not grow unbounded. Default retention is 4 MiB and can be changed with `TALOS_LAB_SERIAL_LOG_MAX_BYTES`.

Serial endpoints:

```text
GET  /serial/peek?max_bytes=4096&drain=true
POST /serial/read
POST /serial/write
POST /serial/observe
POST /serial/transact
GET  /serial/tail?lines=80&max_bytes=65536
```

Endpoint behavior:

- `peek`: returns the last retained serial bytes and a cursor. Does not consume the log. `drain=true` first pulls any currently available device bytes into the log.
- `read`: consumes newly available serial device bytes over a timeout/settle window, appends them to the log, and returns base64 plus best-effort decoded text.
- `write`: writes text to serial, optionally with a newline.
- `observe`: consumes newly available serial bytes, then returns log bytes after a supplied cursor. This is the preferred agent endpoint after `write`.
- `transact`: writes text and waits for a regex. Use only when the expected prompt is known.
- `tail`: line-oriented log view for humans.

Serial endpoint reference:

```text
GET /serial/peek
Query:
  max_bytes: integer, default 4096, range 1..1048576
  drain: boolean, default true
  encoding: string, default utf-8
Response:
  action, ok, bytes, base64, text, encoding, cursor, drained

POST /serial/read
JSON:
  timeout_seconds: number, default 5.0, range >0..300
  settle_ms: integer, default 250, range 0..5000
  max_bytes: integer, default 65536, range 1..1048576
  encoding: string, default utf-8
Response:
  action, ok, bytes, base64, text, encoding, cursor, truncated

POST /serial/write
JSON:
  text: string, required
  append_newline: boolean, default true
Response:
  action, ok, bytes

POST /serial/observe
JSON:
  cursor: integer, optional; omit to observe from current end of log
  timeout_seconds: number, default 5.0, range >0..300
  settle_ms: integer, default 250, range 0..5000
  max_bytes: integer, default 65536, range 1..1048576
  encoding: string, default utf-8
Response:
  action, ok, bytes, base64, text, encoding, cursor_start, cursor_end, truncated

POST /serial/transact
JSON:
  text: string, required
  timeout_seconds: number, default 10.0, range >0..300
  until_regex: string, optional; defaults to configured prompt regex
  settle_ms: integer, default 100, range 0..5000
  max_bytes: integer, default 65536, range 1..1048576
Response:
  action, ok, output

GET /serial/tail
Query:
  lines: integer, default 80, range 1..2000
  max_bytes: integer, default 65536, range 1..1048576
Response:
  action, ok, lines, line_count
```

Booleans accept `true`, `false`, `1`, `0`, `yes`, `no`, `on`, and `off`.

Cursor notes:

- Cursors are byte offsets into the retained serial log.
- `peek` returns the current end-of-log cursor.
- `observe` with a cursor returns the bytes after that cursor.
- `observe` without a cursor starts at the current end of log, waits, and returns only newly arriving bytes.
- If the rolling log has trimmed older bytes, very old cursors may no longer be meaningful. In normal agent loops, use a cursor from the immediately preceding `peek`.
- If the current cursor equals the retention cap, currently 4 MiB by default, cursor-based `observe` can remain pinned while new device bytes are appended and older retained bytes are dropped. Use `scripts/rpi5-observe-serial-window.sh` in its default auto mode so saturated cursors fall back to direct `/serial/read` capture. Do not accept an empty `observe` window from a saturated cursor as proof that the current boot emitted no serial output.

For boot/kernel output, do not wait for `login:` or a shell prompt. Use `read` or `observe` with a timeout and inspect `base64`/`text`. The base64 field preserves raw bytes for diagnosing ANSI escape sequences, encoding problems, or baud-rate issues. The current baud rate is known good because the API has received `talos-pi5 login:` from the Pi.

Example agent interaction from a login prompt:

```bash
cursor="$(curl -fsS 'http://talos-lab-api:8080/serial/peek?max_bytes=500&drain=true' | jq -r .cursor)"
curl -fsS -X POST -H 'Content-Type: application/json' --data '{"text":"matthew"}' http://talos-lab-api:8080/serial/write
curl -fsS -X POST -H 'Content-Type: application/json' --data "{\"cursor\":${cursor},\"timeout_seconds\":5,\"settle_ms\":250,\"max_bytes\":4096}" http://talos-lab-api:8080/serial/observe
```

The observed output should show the next console state, such as `Password:` or `talos-pi5 login:`.

## Failure Signals

No TFTP log entries during reboot:

- Check Pi EEPROM `BOOT_ORDER`.
- Check network-controller boot-server and TFTP-server fields.
- Check host firewall UDP/69 allow rules for `10.42.1.4`.

TFTP requests happen but the Pi falls back to SD boot partition:

- Check `cmdline.txt`.
- Check kernel/initramfs compatibility.
- Check missing DTB/overlay files.
- Use serial logs once serial is connected.

Serial shows RP1 firmware output but no Talos banner:

- The Pi has rebooted and the lab cable is receiving boot-time serial output.
- Confirm the TFTP request sequence from `docker logs talos-tftp`.
- Recheck `kernel_2712.img` format, arm64 image header/load address expectations, `config.txt` kernel settings, and the earliest UART write path.
- For Talos, confirm the Pi 5 target is linked at `0x00000000`, not QEMU virt's `0x40200000`. The official Raspberry Pi 5 `kernel_2712.img` raw image advertises arm64 Image `text_offset=0`, so Talos follows that contract for the physical target while keeping QEMU's separate load address.
- Confirm the arm64 Image header `image_size` matches the generated `kernel_2712.img` byte length. `scripts/rpi5-image.sh` checks this automatically.
- Consider a tiny assembly-only UART diagnostic before Rust clears BSS or switches stacks.
- If several archives all stop immediately after `RP1 FW: load 0` / `RP1_BOOT chip ID`, classify the evidence as pre-entry and possibly pre-config/kernel-load. At that point, prefer bootloader file-load visibility, EEPROM/network-boot diagnostics, or a different firmware entry path over more Rust-side changes.

Pi boots but hostname/files look unchanged:

- Expected while `root=/dev/mmcblk0p2`.
- Confirm the boot path with `docker logs talos-tftp`, `uname -a`, and `cat /proc/cmdline`.
