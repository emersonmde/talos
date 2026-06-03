# Talos Review: VFS, Loader, Userspace Prep

Task: talos-review-vfs-loader-userspace-20260603
Status: accepted
Started: 2026-06-03T06:30:13Z
Completed: 2026-06-03T06:47:24Z

## Scope

Reviewed the Phase 8 initramfs/VFS, program-loader, process-install,
process-address-space, process-page-table-materialization, initial launch,
initial stack, live activation, kernel-half reachability/descriptor-image,
live descriptor installation, and live translation-register activation path.

This task did not implement new VFS/open/read feature work and did not touch
Pi 5 hardware.

## Findings

- Fixed: the loader accepted R-only PT_LOAD data segments as UserData READ,
  but process-install rejected that permission. That made the loader contract
  internally inconsistent before future VFS-backed /bin/init loading.
  Disposition: fixed in src/process_install.rs with a regression preserving
  R-- UserData page records.

- Fixed: process address-space validation also treated UserData as RW-only, so
  even a corrected install plan could not preserve R-only data mappings into
  process-owned mappings and frame leases.
  Disposition: fixed in src/process_address_space.rs with a regression covering
  R-- mappings and leases.

- Fixed: process page-table materialization accepted READ permissions without
  checking the segment kind, allowing a bypassed UserText+READ proof fixture to
  fail after leases were acquired. The QEMU smoke exposed this as
  leaked-leases=true in the permission-widening case.
  Disposition: fixed in src/process_page_table_materialization.rs by validating
  kind+permission pairs before leasing; the materialization smoke now reports
  leaked-leases=false.

- Fixed: a process-address-space test fixture used PHASE8_INIT_BYTES where it
  meant PHASE8_INIT_PATH, hiding source-path intent in a negative fixture.
  Disposition: fixed in src/process_address_space.rs.

- Fixed: process-install, process-address-space, process-page-table
  materialization, and roadmap docs still described UserData as RW-only even
  though the loader contract already accepted R-only data.
  Disposition: fixed in docs/src/project/phase8-process-install-contract.md,
  docs/src/project/phase8-process-address-space-contract.md,
  docs/src/project/phase8-process-page-table-materialization-contract.md, and
  docs/src/roadmap.md.

- Not an issue: the remaining metadata-only launch/live-activation modules are
  still deliberate preflight boundaries. They remain useful as accepted lineage,
  rollback, and no-live-side-effect evidence until the real VFS/open/read and
  initial userspace launch tasks resume.

## Validation

- Static inspection: rg/static review of src/initramfs.rs, src/program_loader.rs,
  src/process_install.rs, src/process_address_space.rs,
  src/process_page_table_materialization.rs, src/initial_process_launch.rs,
  src/initial_user_stack.rs, src/live_address_space_activation.rs,
  src/kernel_half_reachability.rs, src/kernel_half_descriptor_image.rs,
  src/live_descriptor_image_installation.rs, src/live_translation_register_activation.rs,
  docs/src/project/phase8-* contracts, and docs/src/roadmap.md.
- fmt/lint/typecheck: cargo fmt --all; cargo fmt --all -- --check;
  cargo -Zjson-target-spec check.
- unit tests: cargo -Zjson-target-spec test --quiet passed with 366 no_std
  tests.
- target checks: cargo -Zjson-target-spec check --target
  targets/aarch64-talos-virt.json passed; cargo -Zjson-target-spec check
  --target targets/aarch64-talos-rpi5-bcm2712.json passed.
- QEMU/substitute: qemu-readonly-initramfs-vfs-smoke PASS;
  qemu-program-loader-smoke PASS; qemu-process-install-smoke PASS;
  qemu-process-address-space-smoke PASS;
  qemu-process-page-table-materialization-smoke PASS after the lease-leak fix;
  qemu-initial-process-launch-smoke PASS; qemu-initial-user-stack-smoke PASS;
  qemu-live-address-space-activation-smoke PASS;
  qemu-kernel-half-reachability-smoke PASS;
  qemu-kernel-half-descriptor-image-smoke PASS;
  qemu-live-descriptor-image-installation-smoke PASS;
  qemu-live-translation-register-activation-smoke PASS.
- docs validation: /home/node/.cargo/bin/mdbook build passed; mdbook warned
  that the search index is large.
- diff hygiene: git diff --check passed; git diff --cached --check passed
  before commit.

## Remaining Risks

- No Pi 5 hardware claim was made or needed.
- Existing QEMU smoke transcript vocabulary still prints only the current
  /bin/init fixture's RW data segment; R-only data is covered by unit tests.
  A future fixture with real rodata should naturally extend the smoke surface.

## Commit

- Review implementation commit: 47a21f081a8416ab78c59fe124d915d818dc8d35
- Acceptance/state commit: b8ccec78b10de93951f50491e1ca42da21d9a664
