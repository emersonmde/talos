# Phase 8 Initial User Stack Contract

Status: accepted documentation-only contract for
phase8-initial-user-stack-contract-20260530.

This contract follows the accepted
[Phase 8 Initial User Stack Source Inventory](phase8-initial-user-stack-source-inventory.md).
It selects the first target-independent initial user stack record below live
launch. It adds no Rust behavior, assembly behavior, QEMU execution, Pi 5
hardware run, boot archive publication, hardware-lock acquisition,
TTBR/TCR/MAIR/SCTLR writes, ASID allocation, live TLB invalidation, lower-EL
ERET, scheduler runnable publication, process lifecycle, shell behavior,
descriptor-backed filesystem syscalls, writable filesystems, networking, SSH,
RP1/PCIe, UART interrupt ownership, or DMA/cache-driver policy.

## Selected Boundary

The next implementation boundary should be an InitialUserStackPlan record. It
consumes the accepted image, install, address-space, materialization, and
initial launch records and produces an inspectable stack plan only. It does
not activate an address space, write SP_EL0, mutate an exception frame,
publish a scheduler task, allocate a PID, or make /bin/init runnable.

Accepted inputs:

- ProgramImagePlan from src/program_loader.rs, including fixture identity
  phase8-program-loader-elf64-aarch64-v1, source path, digest, entry,
  segment list, rounded memory footprint, and LOADER_PAGE_SIZE.
- ProcessImageInstallPlan from src/process_install.rs, including boundary
  identity phase8-process-install-plan-v1, page records, copied and zeroed
  image bytes, and lower_el_launch_blocked=true.
- ProcessAddressSpace from src/process_address_space.rs, including boundary
  identity phase8-process-address-space-model-v1, ProcessAddressSpaceId, optional
  ProcessOwnerId, image mapping list, publication state, teardown state, lease
  counters, and no-runnable-publication state.
- ProcessPageTableMaterialization from
  src/process_page_table_materialization.rs, including boundary identity
  phase8-process-page-table-materialization-v1, materialization id,
  descriptor records, materialized user-frame records, activation_blocked=true,
  kernel_mapping_policy=activation-blocked-no-kernel-half, teardown state, and
  lease counters.
- InitialProcessLaunchPlan from src/initial_process_launch.rs, including
  boundary identity phase8-initial-process-launch-plan-v1, entry_pc,
  user_sp_state=blocked-missing-initial-user-stack,
  activation_state=blocked-no-ttbr-activation, saved-frame intent, and zero
  live-launch side effects.
- POSIX user-range and error vocabulary from src/posix.rs, including
  USER_NULL_GUARD_END, USER_ADDRESS_SPACE_END, UserRange,
  UserMappingPermissions::USER_DATA, EINVAL, EFAULT, EACCES, ENOMEM, ENOSYS,
  and ENOEXEC.

Accepted output:

- one InitialUserStackPlan whose boundary identity should be
  phase8-initial-user-stack-plan-v1;
- copied identities for the image, install plan, address-space record,
  materialization record, and initial launch plan;
- a deterministic stack layout record with guard range, usable range,
  top-of-stack, initial SP value, page size, alignment, and permissions;
- stack frame/page ownership records for zero-filled user data pages only;
- copied_bytes=0 and zeroed_bytes equal to the usable stack length;
- teardown and rollback state proving stack-owned leases can be released
  idempotently without touching image mappings;
- a launch-plan stack-state binding that changes the model from
  blocked-missing-initial-user-stack to model-only-initial-user-stack-ready
  without permitting activation or runnable publication; and
- side-effect counters showing no TTBR/TCR/MAIR/SCTLR writes, no ASID
  allocation, no live TLB invalidation, no lower-EL ERET, no scheduler
  publication, no process-table mutation, and no descriptor-table mutation.

The selected boundary accepts a stack record, not process execution. It exists
so a later QEMU/substitute smoke can prove stack construction and launch-plan
integration before any task attempts live TTBR activation or ERET.

## Stack Layout

The first stack layout is intentionally fixed and target-independent:

| Field | Contract value |
| --- | --- |
| Page size | LOADER_PAGE_SIZE, 0x1000 bytes. |
| Stack top | USER_ADDRESS_SPACE_END, 0x0000_8000_0000_0000. |
| Initial SP | Stack top, 16-byte aligned. |
| Usable stack length | 4 pages, 0x4000 bytes. |
| Usable stack range | [0x0000_7fff_ffff_c000, 0x0000_8000_0000_0000). |
| Guard length | 1 page, 0x1000 bytes. |
| Guard range | [0x0000_7fff_ffff_b000, 0x0000_7fff_ffff_c000). |
| Growth direction | Downward from initial SP. |
| Permissions | UserMappingPermissions::USER_DATA for usable pages. |
| Guard permissions | Unmapped model-only guard; no descriptor or frame lease. |

The layout must be rejected before publication if any guard or usable page:

- is not page aligned;
- wraps, crosses USER_ADDRESS_SPACE_END, or enters the null guard below
  USER_NULL_GUARD_END;
- overlaps any ProgramImagePlan segment, ProcessImageInstallPlan page,
  ProcessAddressSpace mapping, or ProcessPageTableMaterialization descriptor;
- would require executable stack permissions; or
- cannot be represented by the target-independent stack plan capacity.

The guard is a contract record, not a live page fault path. Guard-fault
handling, dynamic stack growth, signal stacks, copy-on-write, and demand
paging remain blocked.

## Ownership And Accounting

Stack construction should use a separate stack lease source or stack-owned
lease records rather than stealing ownership from image mappings. The record
may reference ProcessAddressSpaceId and materialization id for lineage, but it
must not mutate the existing ProcessAddressSpace or
ProcessPageTableMaterialization values in this slice.

Required ownership fields:

- stack layout identity and stack plan id;
- one lease token per usable stack page;
- virtual page address for each usable stack page;
- UserMappingPermissions::USER_DATA for every usable page;
- zeroed_before_copy=true for every usable page;
- copied_bytes=0 for every usable page;
- zeroed_bytes=LOADER_PAGE_SIZE for every usable page;
- source_page_ordinal covering only stack pages, not image pages;
- guard_pages_reserved=1 with no frame lease and no descriptor slot; and
- released=false until rollback or teardown.

Successful construction must account for total_usable_pages=4,
total_guard_pages=1, total_copied_bytes=0, and total_zeroed_bytes=0x4000.
The first implementation must not copy argv, envp, auxv, TLS, file bytes, or
kernel strings into the stack.

Rollback and teardown are all-or-nothing:

- validation failure returns no InitialUserStackPlan and leases no stack
  frame;
- allocation failure releases already leased stack frames and returns ENOMEM;
- overlap or permission failure releases already leased stack frames and
  returns EACCES;
- identity, state, alignment, or unsupported request failure releases already
  leased stack frames and returns EINVAL or ENOSYS as specified below; and
- teardown may be called repeatedly and must preserve image, address-space,
  materialization, scheduler, process-table, and descriptor-table state.

## Startup Payload Policy

This slice accepts only a minimal empty startup payload placeholder:

- argc is 0;
- argv is NULL;
- envp is NULL;
- auxv is blocked-pending-startup-abi;
- TLS is blocked-pending-startup-abi; and
- no bytes are copied to the stack for argument, environment, auxiliary vector,
  platform string, random seed, or thread-local storage data.

The placeholder may be represented in the InitialUserStackPlan and in a
future saved-frame intent update as model-only metadata. It must not require
dereferencing NULL, validating an argv/envp user range, copying data into user
memory, or claiming libc startup compatibility. Broad argv/envp/auxv/TLS ABI
work remains blocked for a later explicit contract.

## Launch Integration

The stack plan may bind to an InitialProcessLaunchPlan only when:

- the launch plan has boundary identity phase8-initial-process-launch-plan-v1;
- the launch plan references the same image, install, address-space, and
  materialization lineage;
- entry_pc remains covered by the accepted UserText mapping and executable
  descriptor;
- user_sp_state is blocked-missing-initial-user-stack before binding;
- activation_state is blocked-no-ttbr-activation;
- side effects are all false; and
- published=true means only the model record exists, not runnable state.

The resulting binding should expose:

- user_sp_state=model-only-initial-user-stack-ready;
- saved-frame SP_EL0 intent equal to the stack plan initial SP;
- startup payload state=minimal-empty-argc0;
- activation_state still blocked-no-ttbr-activation;
- no-runnable-publication=true for commit requests; and
- no-partial-launch=true for any rejected live launch request.

Requests to activate TTBRs, write SP_EL0, write ELR/SPSR, ERET to lower EL,
publish a scheduler task, allocate a process, mutate descriptor inheritance,
or perform filesystem syscalls return ENOSYS with zero side effects.

## Deterministic Errors

| Case | Error | Required side effect |
| --- | --- | --- |
| Image, install, address-space, materialization, or launch identity mismatch | EINVAL | No stack plan published. |
| Entry, descriptor, or mapping lineage no longer matches accepted launch plan | ENOEXEC or EINVAL | No stack plan published. |
| Stack constants are unaligned, wrap, or exceed USER_ADDRESS_SPACE_END | EFAULT | No stack plan published. |
| Guard or usable range enters USER_NULL_GUARD_END | EFAULT | No stack plan published. |
| Stack guard or usable range overlaps image mappings/descriptors | EACCES | No stack plan published. |
| Stack permissions are executable, read-only, or otherwise not USER_DATA | EACCES | No stack plan published. |
| Stack lease, page, descriptor, or capacity budget is exhausted | ENOMEM | All partial stack leases released. |
| Input launch plan already has a stack-ready or activation-ready state | EINVAL | No stack plan published. |
| Request asks for live activation, ERET, runnable publication, process lifecycle, descriptor inheritance, or filesystem syscall behavior | ENOSYS | no-partial-launch and no-runnable-publication remain true. |

The implementation may split internal error enums more finely, but the
observable POSIX mapping and no-partial-stack/no-partial-launch behavior must
remain deterministic.

## Evidence Vocabulary

The mechanically next task is
phase8-qemu-initial-user-stack-smoke-plan-20260530. That smoke plan should
define exact fixture identities, a retained evidence path, and PASS lines for
the following observations:

- stack boundary identity phase8-initial-user-stack-plan-v1;
- guard range and usable range match this contract;
- initial SP equals 0x0000_8000_0000_0000 and is 16-byte aligned;
- total_usable_pages=4, total_guard_pages=1, copied_bytes=0, and
  zeroed_bytes=0x4000;
- every usable page is USER_DATA, zeroed_before_copy=true, and stack-owned;
- guard page has no frame lease and no descriptor;
- deterministic rejection cases cover overlap, range, permission, capacity,
  and unsupported live-launch requests;
- launch-plan binding changes only the model user_sp_state and saved-frame
  SP intent;
- activation_state remains blocked-no-ttbr-activation;
- TTBR/TLB/SCTLR/MAIR/TCR, lower-EL ERET, scheduler, process-table, and
  descriptor-table side effects remain zero; and
- final classification and PASS vocabulary should be
  qemu-initial-user-stack-smoke-complete and
  qemu-initial-user-stack-smoke: PASS.

The evidence level remains QEMU/substitute or target-independent inspection.
Pi 5 hardware proof is not part of this stack contract.

## Deferred Surfaces

This contract keeps these surfaces blocked:

- live TTBR0_EL1, TTBR1_EL1, TCR_EL1, MAIR_EL1, SCTLR_EL1, ASID, TLB, DSB,
  and ISB mutation;
- architectural SP_EL0/ELR_EL1/SPSR_EL1 writes, lower-EL ERET, and live trap
  return behavior;
- scheduler runnable publication, process table/PID/current-process state,
  parent/child lifecycle, exit status, wait state, signals, credentials, and
  descriptor inheritance;
- broad argv/envp/auxv/TLS ABI, libc startup, dynamic stack growth, signal
  stacks, guard-fault handling, copy-on-write, and demand paging;
- descriptor-backed filesystem syscalls, writable filesystem, shell behavior,
  and process execution;
- QEMU execution for this slice, Pi 5 hardware proof, boot archive
  publication, TFTP/serial evidence, and hardwareTestLock acquisition; and
- networking, SSH, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver
  policy.

## Reviewed Materials

- docs/src/project/phase8-initial-user-stack-source-inventory.md
- docs/src/project/phase8-initial-process-launch-contract.md
- docs/src/project/phase8-initial-process-launch-closeout-checkpoint.md
- src/posix.rs
- src/program_loader.rs
- src/process_install.rs
- src/process_address_space.rs
- src/process_page_table_materialization.rs
- src/initial_process_launch.rs
- docs/src/architecture/lower-el-userspace.md
- docs/src/roadmap.md
- docs/src/decisions/README.md

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/source review: inspected the accepted initial user
  stack source inventory, initial process launch contract and closeout,
  POSIX user range/error vocabulary, loader, process-install,
  ProcessAddressSpace, process page-table materialization, and initial launch
  source owners.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
