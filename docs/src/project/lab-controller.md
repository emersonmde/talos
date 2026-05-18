# Lab Controller

Talos uses a narrow lab controller on Strider to automate the Raspberry Pi 5 boot loop. OpenClaw should treat this as the control surface for physical Pi operations.

## Current Shape

The deployed lab service lives on Strider, outside the OpenClaw workspace:

```text
/opt/strider/talos-lab
```

OpenClaw does not need to read that directory. Use the private API from inside the OpenClaw container:

```text
http://talos-lab-api:8080
```

The API owns:

- UniFi credentials.
- The fixed Weathertop port mapping.
- Power-cycle actions.
- TFTP boot archive publishing.
- One-archive rollback.
- Serial access later, after the serial cable is installed.

OpenClaw should not call UniFi directly and should not ask for UniFi keys.

## Target Facts

```text
strider:   10.42.1.3
talos-pi5: 10.42.1.4
talos DNS: talos.memerson.net
pi5 MAC:   88:a2:9e:ae:c8:7f
gateway:   Weathertop / UDM Pro SE / 10.42.1.1
PoE port:  Weathertop port 8
site ID:   88f7af54-98f8-306a-a1c7-c9349722b1f6
device ID: 99dd2845-8d30-3258-b27f-43295483fa7d
```

The API verifies the live UniFi client mapping before any power action. If `88:a2:9e:ae:c8:7f` / `10.42.1.4` is not still on Weathertop port 8, power control fails closed.

## API Commands

Health:

```bash
curl -fsS http://talos-lab-api:8080/health
```

Status:

```bash
curl -fsS http://talos-lab-api:8080/status
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

The API keeps exactly one rollback archive and removes upload/staging files after each publish attempt. Do not leave large boot tarballs in the OpenClaw workspace after upload.

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

UniFi DHCP on the `10.42.1.0/24` network is configured with:

```text
Network Boot server:   10.42.1.3
Network Boot filename: config.txt
TFTP Server:           10.42.1.3
```

Strider firewalld allows UDP/69 only from the Pi:

```text
10.42.1.4/32 -> UDP/69
```

TFTP is served by `talos-tftp` using dnsmasq in TFTP-only mode. Request logs are visible on Strider with:

```bash
docker logs talos-tftp
```

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

Talos now has local staging scripts for the first Talos boot archive candidate:

```bash
./scripts/rpi5-image.sh
./scripts/rpi5-boot-tree.sh /path/to/pi-firmware-boot-source target/rpi5-boot-tree
tar -C target/rpi5-boot-tree -czf target/talos-rpi5-boot.tar.gz .
```

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

The serial API contract has been tested against a fake PTY-backed serial peer:

```text
POST /serial/transact -> echo: version
POST /serial/write    -> writes command text
GET  /serial/tail     -> returns captured serial log lines
```

The deployed API is currently restored to `serial.configured=false` until the physical USB serial cable is attached.

When the cable arrives, prefer a stable device path:

```text
/dev/serial/by-id/...
```

If the adapter has no unique ID, use:

```text
/dev/serial/by-path/...
```

Avoid raw `/dev/ttyUSB0` or `/dev/ttyACM0` in persistent config because those names can change after reboot or reconnect. The API container should mount the stable host path to a fixed in-container path such as `/dev/talos-serial`, and config should point to that stable in-container path.

## Failure Signals

No TFTP log entries during reboot:

- Check Pi EEPROM `BOOT_ORDER`.
- Check UniFi Network Boot and TFTP Server fields.
- Check Strider firewalld UDP/69 allow from `10.42.1.4`.

TFTP requests happen but the Pi falls back to SD boot partition:

- Check `cmdline.txt`.
- Check kernel/initramfs compatibility.
- Check missing DTB/overlay files.
- Use serial logs once serial is connected.

Pi boots but hostname/files look unchanged:

- Expected while `root=/dev/mmcblk0p2`.
- Confirm the boot path with `docker logs talos-tftp`, `uname -a`, and `cat /proc/cmdline`.
