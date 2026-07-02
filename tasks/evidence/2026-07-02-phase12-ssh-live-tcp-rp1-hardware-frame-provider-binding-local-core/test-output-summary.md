# Test Output Summary

- Initial cargo -Zjson-target-spec test --quiet reached the QEMU runner but
  failed because qemu-system-aarch64 was not on PATH.
- Rerun with PATH=/opt/strider/openclaw/current/workspace/tools/qemu-9.2.0-install/bin:$PATH passed.
- Result: 896 no_std tests passed.

New coverage includes:

- RP1 source-bound hardware frame-provider report remains local-only and keeps
  live packet I/O and ssh_ready false.
- Missing RP1 provider fails closed with no-rp1-ethernet-hardware-frame-provider-bound.
- Link-not-ready provider fails closed with
  rp1-ethernet-hardware-frame-provider-link-not-ready-fail-closed.
- Network runtime binding keeps missing descriptor delivery distinct when RP1
  provider metadata is present.
