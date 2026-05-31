#![cfg_attr(any(test, talos_target_rpi5_bcm2712), allow(dead_code))]

#[cfg(any(
    talos_boot_scenario = "qemu_context_switch",
    talos_boot_scenario = "qemu_scheduler_yield",
    talos_boot_scenario = "qemu_timer_preemption"
))]
use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicU64, Ordering};

#[cfg(talos_boot_scenario = "qemu_remote_wake_to_local_runnable")]
use crate::scheduler::TargetWakeConsumptionError;
#[cfg(any(
    talos_boot_scenario = "qemu_context_switch",
    talos_boot_scenario = "qemu_scheduler_yield",
    talos_boot_scenario = "qemu_timer_preemption",
    talos_boot_scenario = "qemu_per_core_scheduler_ownership",
    talos_boot_scenario = "qemu_remote_wakeup_request",
    talos_boot_scenario = "qemu_production_secondary_dispatch",
    talos_boot_scenario = "qemu_shared_scheduler_metadata",
    talos_boot_scenario = "qemu_shared_runqueue_migration",
    talos_boot_scenario = "qemu_load_balancing_smoke",
    talos_boot_scenario = "qemu_secondary_scheduler_service_loop",
    talos_boot_scenario = "qemu_multicore_preemption_smoke",
    talos_boot_scenario = "qemu_production_timer_preemption_smoke"
))]
use crate::scheduler::{ContextFrame, KernelStack, SingleCoreScheduler, Task, TaskId, TaskState};
#[cfg(talos_boot_scenario = "qemu_multicore_preemption_smoke")]
use crate::scheduler::{
    CpuLocalSchedulerService, PerCorePreemptionState, PreemptionRecordOutcome, RemoteWakeQueue,
};
#[cfg(any(
    talos_boot_scenario = "qemu_per_core_scheduler_ownership",
    talos_boot_scenario = "qemu_remote_wakeup_request",
    talos_boot_scenario = "qemu_production_secondary_dispatch",
    talos_boot_scenario = "qemu_shared_scheduler_metadata",
    talos_boot_scenario = "qemu_shared_runqueue_migration",
    talos_boot_scenario = "qemu_load_balancing_smoke",
    talos_boot_scenario = "qemu_secondary_scheduler_service_loop",
    talos_boot_scenario = "qemu_multicore_preemption_smoke",
    talos_boot_scenario = "qemu_production_timer_preemption_smoke"
))]
use crate::scheduler::{
    LoadBalancingPolicy, LogicalCpuId, MigrationState, PerCoreScheduler,
    PerCoreSchedulerAccessError, ProductionDispatchError, SchedulerCoreRole, SharedRunQueue,
    SharedSchedulerMetadata, SharedSchedulerMetadataError, SharedSchedulerMetadataLock,
};
#[cfg(talos_boot_scenario = "qemu_remote_wakeup_request")]
use crate::scheduler::{RemoteWakePublishOutcome, RemoteWakeQueue};
#[cfg(talos_boot_scenario = "qemu_secondary_scheduler_service_loop")]
use crate::scheduler::{
    RemoteWakeQueue, SecondarySchedulerServiceLoop, SecondarySchedulerServiceLoopError,
};
#[cfg(not(any(
    talos_boot_scenario = "qemu_secondary_core_workload",
    talos_boot_scenario = "qemu_smp_lock_contention",
    talos_boot_scenario = "qemu_per_core_scheduler_ownership",
    talos_boot_scenario = "qemu_cross_core_ipi_delivery",
    talos_boot_scenario = "qemu_remote_wakeup_request",
    talos_boot_scenario = "qemu_production_secondary_dispatch",
    talos_boot_scenario = "qemu_shared_scheduler_metadata",
    talos_boot_scenario = "qemu_secondary_scheduler_service_loop",
    talos_boot_scenario = "qemu_multicore_preemption_smoke",
    talos_boot_scenario = "qemu_production_timer_preemption_smoke"
)))]
use crate::smp::MAX_CORES;
#[cfg(talos_boot_scenario = "qemu_secondary_core_workload")]
use crate::smp::SECONDARY_CORE_WORKLOAD_TARGET;
#[cfg(any(
    talos_boot_scenario = "qemu_secondary_core_workload",
    talos_boot_scenario = "qemu_smp_lock_contention",
    talos_boot_scenario = "qemu_per_core_scheduler_ownership",
    talos_boot_scenario = "qemu_cross_core_ipi_delivery",
    talos_boot_scenario = "qemu_remote_wakeup_request",
    talos_boot_scenario = "qemu_production_secondary_dispatch",
    talos_boot_scenario = "qemu_shared_scheduler_metadata",
    talos_boot_scenario = "qemu_secondary_scheduler_service_loop",
    talos_boot_scenario = "qemu_multicore_preemption_smoke",
    talos_boot_scenario = "qemu_production_timer_preemption_smoke"
))]
use crate::smp::{
    self, CoreLifecycle, CoreStackLayout, MAX_CORES, SECONDARY_CORE_STATES,
    SECONDARY_KERNEL_STACK_SIZE,
};
#[cfg(talos_boot_scenario = "qemu_smp_lock_contention")]
use crate::smp_sync::{SpinLock, smp_full_barrier};
#[cfg(any(
    talos_boot_scenario = "qemu_per_core_scheduler_ownership",
    talos_boot_scenario = "qemu_remote_wakeup_request",
    talos_boot_scenario = "qemu_production_secondary_dispatch",
    talos_boot_scenario = "qemu_shared_scheduler_metadata",
    talos_boot_scenario = "qemu_secondary_scheduler_service_loop",
    talos_boot_scenario = "qemu_multicore_preemption_smoke",
    talos_boot_scenario = "qemu_production_timer_preemption_smoke"
))]
use crate::smp_sync::{SpinLock, smp_full_barrier};
#[cfg(any(
    talos_boot_scenario = "qemu_syscall_smoke",
    talos_boot_scenario = "qemu_pointer_copy_smoke",
    talos_boot_scenario = "qemu_descriptor_write_smoke"
))]
use crate::syscall::{self, SyscallNumber};
#[cfg(any(
    talos_boot_scenario = "qemu_el0_trap_smoke",
    talos_boot_scenario = "qemu_syscall_smoke",
    talos_boot_scenario = "qemu_pointer_copy_smoke",
    talos_boot_scenario = "qemu_descriptor_write_smoke"
))]
use crate::{
    arch::aarch64::exceptions::{ExceptionFrame, ExceptionVector},
    posix::{
        PosixError, UserAccessKind, UserMapping, UserMappingPermissions,
        validate_user_memory_access,
    },
};
use crate::{
    arch::aarch64::{
        self, generic_timer,
        gicv2::{GicV2, SPURIOUS_INTID},
    },
    boot::BootInfo,
    device_tree::DeviceTree,
    mmio::{MmioMap, MmioRegion},
    pl011::Pl011,
    target::{InterruptControllerKind, TargetServices, TimerKind, UartKind},
};

#[cfg(talos_boot_scenario = "qemu_initial_user_stack_smoke")]
use crate::initial_user_stack::{
    INITIAL_USER_STACK_BOUNDARY_IDENTITY, INITIAL_USER_STACK_READY, InitialUserStackLayout,
    InitialUserStackLeaseSource, InitialUserStackRequest, plan_initial_user_stack,
};
#[cfg(any(
    talos_boot_scenario = "qemu_initial_process_launch_smoke",
    talos_boot_scenario = "qemu_initial_user_stack_smoke"
))]
use crate::{
    initial_process_launch::{
        INITIAL_ACTIVATION_BLOCKED, INITIAL_ADDRESS_SPACE_TOKEN_STATE, INITIAL_DAIF_BLOCKED,
        INITIAL_PROCESS_LAUNCH_BOUNDARY_IDENTITY, INITIAL_SPSR_BLOCKED, INITIAL_USER_SP_BLOCKED,
        INITIAL_X0_X5_BLOCKED, InitialProcessLaunchCommitTarget, InitialProcessLaunchRequest,
        prepare_initial_process_launch,
    },
    initramfs::{PHASE8_INIT_PATH, phase8_readonly_initramfs_fixture},
    posix::{PosixError, USER_NULL_GUARD_END, UserMappingPermissions},
    process_address_space::{
        PROCESS_ADDRESS_SPACE_BOUNDARY_IDENTITY, ProcessAddressSpace, ProcessAddressSpaceId,
        ProcessAddressSpaceLeaseSource, install_process_address_space,
    },
    process_install::{
        MAX_PROCESS_INSTALL_PAGES, PROCESS_INSTALL_BOUNDARY_IDENTITY, ProcessImageInstallPlan,
        ProcessInstallSideEffects, plan_process_image_install,
    },
    process_page_table_materialization::{
        PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY, ProcessMaterializationRequest,
        ProcessPageTableMaterialization, ProcessPageTableMaterializationLeaseSource,
        materialize_process_page_tables,
    },
    program_loader::{
        LOADER_PAGE_SIZE, MAX_LOAD_SEGMENTS, PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY,
        PlannedUserSegment, ProgramImagePlan, UserSegmentKind, plan_phase8_init_image,
    },
    scheduler::ProcessOwnerId,
};
#[cfg(talos_boot_scenario = "qemu_live_address_space_activation_smoke")]
use crate::{
    initial_process_launch::{
        INITIAL_ACTIVATION_BLOCKED, INITIAL_PROCESS_LAUNCH_BOUNDARY_IDENTITY,
        InitialProcessLaunchRequest, prepare_initial_process_launch,
    },
    initial_user_stack::{
        INITIAL_USER_STACK_BOUNDARY_IDENTITY, InitialUserStackLeaseSource, InitialUserStackRequest,
        plan_initial_user_stack,
    },
    initramfs::{PHASE8_INIT_PATH, phase8_readonly_initramfs_fixture},
    live_address_space_activation::{
        ACTIVATION_PREFLIGHT_READY, ASID_ALLOCATION_BLOCKED, BARRIER_SEQUENCE_PLANNED_ONLY,
        LIVE_ADDRESS_SPACE_ACTIVATION_BOUNDARY_IDENTITY, LIVE_ADDRESS_SPACE_ACTIVATION_POLICY,
        LIVE_REGISTER_SEQUENCE_BLOCKED, LiveAddressSpaceActivationCommitTarget,
        LiveAddressSpaceActivationLeaseSource, LiveAddressSpaceActivationRequest,
        MAIR_COMPATIBILITY_RECORD_ONLY, RUNNABLE_PUBLICATION_BLOCKED, SCTLR_MUTATION_BLOCKED,
        TCR_COMPATIBILITY_RECORD_ONLY, TLB_INVALIDATION_BLOCKED, TTBR0_ROOT_PROVENANCE,
        TTBR1_KERNEL_POLICY_BLOCKED, preflight_live_address_space_activation,
    },
    posix::{PosixError, UserMappingPermissions},
    process_address_space::{
        PROCESS_ADDRESS_SPACE_BOUNDARY_IDENTITY, ProcessAddressSpace, ProcessAddressSpaceId,
        ProcessAddressSpaceLeaseSource, install_process_address_space,
    },
    process_install::{
        PROCESS_INSTALL_BOUNDARY_IDENTITY, ProcessImageInstallPlan, plan_process_image_install,
    },
    process_page_table_materialization::{
        PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY, ProcessMaterializationRequest,
        ProcessPageTableMaterialization, ProcessPageTableMaterializationLeaseSource,
        materialize_process_page_tables,
    },
    program_loader::{
        MAX_LOAD_SEGMENTS, PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY, ProgramImagePlan,
        plan_phase8_init_image,
    },
    scheduler::ProcessOwnerId,
};
#[cfg(talos_boot_scenario = "qemu_kernel_half_reachability_smoke")]
use crate::{
    initial_process_launch::{
        INITIAL_PROCESS_LAUNCH_BOUNDARY_IDENTITY, InitialProcessLaunchPlan,
        InitialProcessLaunchRequest, prepare_initial_process_launch,
    },
    initial_user_stack::{
        INITIAL_USER_STACK_BOUNDARY_IDENTITY, InitialUserStackLeaseSource, InitialUserStackPlan,
        InitialUserStackRequest, plan_initial_user_stack,
    },
    initramfs::{PHASE8_INIT_PATH, phase8_readonly_initramfs_fixture},
    kernel_half_reachability::{
        KERNEL_HALF_DESCRIPTOR_IMAGE_BLOCKED, KERNEL_HALF_REACHABILITY_BOUNDARY_IDENTITY,
        KERNEL_HALF_REACHABILITY_POLICY, KernelHalfReachabilityLeaseSource,
        KernelHalfReachabilityRequest, NORMAL_DEVICE_MAIR_COMPATIBILITY_RECORD_ONLY,
        SPLIT_TCR_COMPATIBILITY_RECORD_ONLY, TTBR1_SHARED_KERNEL_ROOT_POLICY,
        preflight_kernel_half_reachability,
    },
    live_address_space_activation::{
        ASID_ALLOCATION_BLOCKED, BARRIER_SEQUENCE_PLANNED_ONLY,
        LIVE_ADDRESS_SPACE_ACTIVATION_BOUNDARY_IDENTITY, LIVE_REGISTER_SEQUENCE_BLOCKED,
        LiveAddressSpaceActivationLeaseSource, LiveAddressSpaceActivationPlan,
        LiveAddressSpaceActivationRequest, SCTLR_MUTATION_BLOCKED, TLB_INVALIDATION_BLOCKED,
        TTBR0_ROOT_PROVENANCE, preflight_live_address_space_activation,
    },
    posix::{PosixError, UserMappingPermissions},
    process_address_space::{
        PROCESS_ADDRESS_SPACE_BOUNDARY_IDENTITY, ProcessAddressSpace, ProcessAddressSpaceId,
        ProcessAddressSpaceLeaseSource, install_process_address_space,
    },
    process_install::{
        PROCESS_INSTALL_BOUNDARY_IDENTITY, ProcessImageInstallPlan, plan_process_image_install,
    },
    process_page_table_materialization::{
        PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY, ProcessMaterializationRequest,
        ProcessPageTableMaterialization, ProcessPageTableMaterializationLeaseSource,
        materialize_process_page_tables,
    },
    program_loader::{
        MAX_LOAD_SEGMENTS, PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY, ProgramImagePlan,
        plan_phase8_init_image,
    },
    scheduler::ProcessOwnerId,
};
#[cfg(any(
    talos_boot_scenario = "qemu_kernel_half_descriptor_image_smoke",
    talos_boot_scenario = "qemu_live_descriptor_image_installation_smoke"
))]
use crate::{
    initial_process_launch::{
        INITIAL_PROCESS_LAUNCH_BOUNDARY_IDENTITY, InitialProcessLaunchRequest,
        prepare_initial_process_launch,
    },
    initial_user_stack::{
        INITIAL_USER_STACK_BOUNDARY_IDENTITY, InitialUserStackLeaseSource, InitialUserStackRequest,
        plan_initial_user_stack,
    },
    initramfs::{PHASE8_INIT_PATH, phase8_readonly_initramfs_fixture},
    kernel_half_descriptor_image::{
        KERNEL_HALF_DESCRIPTOR_IMAGE_BOUNDARY_IDENTITY, KERNEL_HALF_DESCRIPTOR_IMAGE_POLICY,
        KERNEL_HALF_DESCRIPTOR_IMAGE_READY, KernelHalfDescriptorImageLeaseSource,
        KernelHalfDescriptorImageRequest, TTBR0_ROOT_MATERIALIZED_PROVENANCE_ONLY,
        TTBR1_OWNED_KERNEL_ROOT_IMAGE, construct_kernel_half_descriptor_image,
    },
    kernel_half_reachability::{
        KERNEL_HALF_REACHABILITY_BOUNDARY_IDENTITY, KERNEL_HALF_REACHABILITY_POLICY,
        KernelHalfReachabilityLeaseSource, KernelHalfReachabilityPlan,
        KernelHalfReachabilityRequest, preflight_kernel_half_reachability,
    },
    live_address_space_activation::{
        ASID_ALLOCATION_BLOCKED, BARRIER_SEQUENCE_PLANNED_ONLY,
        LIVE_ADDRESS_SPACE_ACTIVATION_BOUNDARY_IDENTITY, LIVE_REGISTER_SEQUENCE_BLOCKED,
        LiveAddressSpaceActivationLeaseSource, LiveAddressSpaceActivationPlan,
        LiveAddressSpaceActivationRequest, SCTLR_MUTATION_BLOCKED, TCR_COMPATIBILITY_RECORD_ONLY,
        TLB_INVALIDATION_BLOCKED, preflight_live_address_space_activation,
    },
    posix::{PosixError, UserMappingPermissions},
    process_address_space::{
        PROCESS_ADDRESS_SPACE_BOUNDARY_IDENTITY, ProcessAddressSpace, ProcessAddressSpaceId,
        ProcessAddressSpaceLeaseSource, install_process_address_space,
    },
    process_install::{
        PROCESS_INSTALL_BOUNDARY_IDENTITY, ProcessImageInstallPlan, plan_process_image_install,
    },
    process_page_table_materialization::{
        PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY, ProcessMaterializationRequest,
        ProcessPageTableMaterialization, ProcessPageTableMaterializationLeaseSource,
        materialize_process_page_tables,
    },
    program_loader::{
        PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY, ProgramImagePlan, plan_phase8_init_image,
    },
    scheduler::ProcessOwnerId,
};
#[cfg(talos_boot_scenario = "qemu_live_descriptor_image_installation_smoke")]
use crate::live_descriptor_image_installation::{
    DESCRIPTOR_IMAGE_INSTALLATION_READY_STATE, DESCRIPTOR_IMAGE_NOT_INSTALLED_STATE,
    LIVE_DESCRIPTOR_IMAGE_INSTALLATION_BOUNDARY_IDENTITY, LIVE_DESCRIPTOR_IMAGE_INSTALLATION_POLICY,
    LiveDescriptorImageInstallationLeaseSource, LiveDescriptorImageInstallationRequest,
    TTBR1_DESCRIPTOR_IMAGE_ROOT_PROVENANCE, install_live_descriptor_image_binding,
};
#[cfg(talos_boot_scenario = "qemu_readonly_initramfs_vfs_smoke")]
use crate::{
    initramfs::{
        PHASE8_BANNER_BYTES, PHASE8_BANNER_PATH, PHASE8_EMPTY_PATH, PHASE8_FIXTURE_NAME,
        PHASE8_INIT_BYTES, PHASE8_NESTED_PATH, ReadOnlyFileDescriptions, VfsNodeKind,
        phase8_readonly_initramfs_fixture,
    },
    posix::{
        DescriptorAccess, DescriptorEntry, DescriptorFlags, DescriptorObject, DescriptorObjectKind,
        DescriptorTable, PathLimits, PosixError, UserMapping, UserMappingPermissions,
    },
};
#[cfg(talos_boot_scenario = "qemu_program_loader_smoke")]
use crate::{
    initramfs::{
        PHASE8_INIT_BYTES, PHASE8_INIT_ELF_LEN, PHASE8_INIT_PATH, phase8_readonly_initramfs_fixture,
    },
    posix::{PosixError, UserAccessKind, UserMappingPermissions},
    program_loader::{
        PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY, PlannedUserSegment, ProgramImagePlan,
        ProgramLoaderError, UserSegmentKind, plan_elf64_aarch64_image, plan_phase8_init_image,
    },
};
#[cfg(any(
    talos_boot_scenario = "qemu_process_install_smoke",
    talos_boot_scenario = "qemu_process_address_space_smoke",
    talos_boot_scenario = "qemu_process_page_table_materialization_smoke"
))]
use crate::{
    initramfs::{PHASE8_INIT_BYTES, PHASE8_INIT_PATH, phase8_readonly_initramfs_fixture},
    posix::{PosixError, USER_NULL_GUARD_END, UserMappingPermissions},
    process_install::{
        MAX_PROCESS_INSTALL_FOOTPRINT, MAX_PROCESS_INSTALL_PAGES, MAX_ZERO_RANGES_PER_PAGE,
        PROCESS_INSTALL_BOUNDARY_IDENTITY, PageByteRange, ProcessImageInstallPlan,
        ProcessImagePageInstallRecord, ProcessInstallAction, ProcessInstallSideEffects,
        plan_process_image_install,
    },
    program_loader::{
        LOADER_PAGE_SIZE, MAX_LOAD_SEGMENTS, PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY,
        PlannedUserSegment, ProgramImagePlan, UserSegmentKind, plan_phase8_init_image,
    },
};
#[cfg(talos_boot_scenario = "qemu_process_page_table_materialization_smoke")]
use crate::{
    process_address_space::{
        ModelLeaseToken, PROCESS_ADDRESS_SPACE_BOUNDARY_IDENTITY, PageTableRootLease,
        ProcessAddressSpace, ProcessAddressSpaceId, ProcessAddressSpaceLeaseSource,
        ProcessAddressSpaceSideEffects, ProcessUserMapping, TablePageLease, UserFrameLease,
        install_process_address_space,
    },
    process_page_table_materialization::{
        PROCESS_PAGE_TABLE_KERNEL_MAPPING_POLICY,
        PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY, ProcessMaterializationRequest,
        ProcessPageTableMaterializationLeaseSource, materialize_process_page_tables,
    },
    scheduler::ProcessOwnerId,
};
#[cfg(talos_boot_scenario = "qemu_process_address_space_smoke")]
use crate::{
    process_address_space::{
        PROCESS_ADDRESS_SPACE_BOUNDARY_IDENTITY, ProcessAddressSpace, ProcessAddressSpaceId,
        ProcessAddressSpaceLeaseSource, ProcessUserMapping, UserFrameLease,
        install_process_address_space,
    },
    scheduler::ProcessOwnerId,
};

const PL011_BASE: usize = 0x0900_0000;
const GICD_BASE: usize = 0x0800_0000;
const GICC_BASE: usize = 0x0801_0000;
const EL2_PHYSICAL_TIMER_INTID: u32 = 26;
const TIMER_IRQ_WAIT_LIMIT: usize = 1_000_000;
#[cfg(any(
    talos_boot_scenario = "qemu_secondary_core_workload",
    talos_boot_scenario = "qemu_smp_lock_contention",
    talos_boot_scenario = "qemu_per_core_scheduler_ownership",
    talos_boot_scenario = "qemu_cross_core_ipi_delivery",
    talos_boot_scenario = "qemu_remote_wakeup_request",
    talos_boot_scenario = "qemu_production_secondary_dispatch",
    talos_boot_scenario = "qemu_shared_scheduler_metadata",
    talos_boot_scenario = "qemu_secondary_scheduler_service_loop",
    talos_boot_scenario = "qemu_multicore_preemption_smoke",
    talos_boot_scenario = "qemu_production_timer_preemption_smoke"
))]
const QEMU_SECONDARY_WAIT_LIMIT: usize = 10_000_000;
#[cfg(any(
    talos_boot_scenario = "qemu_cross_core_ipi_delivery",
    talos_boot_scenario = "qemu_remote_wakeup_request"
))]
const QEMU_CROSS_CORE_IPI_SGI_INTID: u32 = 1;
#[cfg(talos_boot_scenario = "qemu_remote_wakeup_request")]
const REMOTE_WAKE_QUEUE_CAPACITY: usize = 4;
#[cfg(talos_boot_scenario = "qemu_smp_lock_contention")]
const SMP_LOCK_CONTENTION_TARGET_PER_CORE: u64 = 64;
#[cfg(talos_boot_scenario = "qemu_per_core_scheduler_ownership")]
const PER_CORE_SCHEDULER_PROGRESS_TARGET: u64 = 4;
#[cfg(talos_boot_scenario = "qemu_production_secondary_dispatch")]
const PRODUCTION_SECONDARY_DISPATCH_PROGRESS_TARGET: u64 = 3;
#[cfg(talos_boot_scenario = "qemu_shared_scheduler_metadata")]
const SHARED_SCHEDULER_METADATA_TASK_CAPACITY: usize = MAX_CORES;
#[cfg(talos_boot_scenario = "qemu_shared_scheduler_metadata")]
const SHARED_SCHEDULER_METADATA_WAIT_LIMIT: usize = 100_000_000;
#[cfg(talos_boot_scenario = "qemu_shared_runqueue_migration")]
const SHARED_RUNQUEUE_MIGRATION_TASK_CAPACITY: usize = 1;
#[cfg(talos_boot_scenario = "qemu_shared_runqueue_migration")]
const SHARED_RUNQUEUE_MIGRATION_QUEUE_CAPACITY: usize = 1;
#[cfg(talos_boot_scenario = "qemu_load_balancing_smoke")]
const LOAD_BALANCING_SMOKE_TASK_CAPACITY: usize = 1;
#[cfg(talos_boot_scenario = "qemu_load_balancing_smoke")]
const LOAD_BALANCING_SMOKE_QUEUE_CAPACITY: usize = 1;
#[cfg(talos_boot_scenario = "qemu_secondary_scheduler_service_loop")]
const SECONDARY_SCHEDULER_SERVICE_LOOP_TASK_CAPACITY: usize = 1;
#[cfg(talos_boot_scenario = "qemu_secondary_scheduler_service_loop")]
const SECONDARY_SCHEDULER_SERVICE_LOOP_WAIT_LIMIT: usize = 100_000_000;
#[cfg(talos_boot_scenario = "qemu_multicore_preemption_smoke")]
const MULTICORE_PREEMPTION_SMOKE_TASK_CAPACITY: usize = 2;
#[cfg(talos_boot_scenario = "qemu_multicore_preemption_smoke")]
const MULTICORE_PREEMPTION_SMOKE_WAIT_LIMIT: usize = 100_000_000;
#[cfg(talos_boot_scenario = "qemu_production_timer_preemption_smoke")]
const PRODUCTION_TIMER_PREEMPTION_SMOKE_TASK_CAPACITY: usize = 2;
#[cfg(talos_boot_scenario = "qemu_production_timer_preemption_smoke")]
const PRODUCTION_TIMER_PREEMPTION_SMOKE_WAIT_LIMIT: usize = 100_000_000;
#[cfg(any(
    talos_boot_scenario = "qemu_context_switch",
    talos_boot_scenario = "qemu_scheduler_yield",
    talos_boot_scenario = "qemu_timer_preemption"
))]
const CONTEXT_SWITCH_STACK_SIZE: usize = 4096;
#[cfg(talos_boot_scenario = "qemu_context_switch")]
const CONTEXT_SWITCH_TARGET_PROGRESS: u64 = 2;
#[cfg(talos_boot_scenario = "qemu_scheduler_yield")]
const SCHEDULER_YIELD_TARGET_PROGRESS: u64 = 3;
#[cfg(talos_boot_scenario = "qemu_timer_preemption")]
const TIMER_PREEMPTION_TARGET_PROGRESS: u64 = 3;
#[cfg(talos_boot_scenario = "qemu_timer_preemption")]
const TIMER_PREEMPTION_TARGET_SWITCHES: u64 = 6;

#[cfg(talos_boot_scenario = "qemu_readonly_initramfs_vfs_smoke")]
const READONLY_INITRAMFS_VFS_USER_BASE: u64 = 0x0000_0000_0011_0000;

#[cfg(any(
    talos_boot_scenario = "qemu_el0_trap_smoke",
    talos_boot_scenario = "qemu_syscall_smoke",
    talos_boot_scenario = "qemu_pointer_copy_smoke",
    talos_boot_scenario = "qemu_descriptor_write_smoke"
))]
const EL0_TRAP_USER_TEXT_START: u64 = 0x0000_0000_0010_0000;
#[cfg(any(
    talos_boot_scenario = "qemu_el0_trap_smoke",
    talos_boot_scenario = "qemu_syscall_smoke",
    talos_boot_scenario = "qemu_pointer_copy_smoke",
    talos_boot_scenario = "qemu_descriptor_write_smoke"
))]
const EL0_TRAP_USER_TEXT_LEN: usize = 0x1000;
#[cfg(talos_boot_scenario = "qemu_pointer_copy_smoke")]
const POINTER_COPY_USER_DATA_START: u64 = 0x0000_0000_0011_0000;
#[cfg(talos_boot_scenario = "qemu_pointer_copy_smoke")]
const POINTER_COPY_USER_DATA_LEN: usize = 0x1000;
#[cfg(talos_boot_scenario = "qemu_pointer_copy_smoke")]
const POINTER_COPY_USER_DATA_INIT: u8 = 0x2a;
#[cfg(talos_boot_scenario = "qemu_pointer_copy_smoke")]
const POINTER_COPY_USER_DATA_REPLACEMENT: u8 = 0xa5;
#[cfg(talos_boot_scenario = "qemu_descriptor_write_smoke")]
const DESCRIPTOR_WRITE_USER_DATA_START: u64 = 0x0000_0000_0011_0000;
#[cfg(talos_boot_scenario = "qemu_descriptor_write_smoke")]
const DESCRIPTOR_WRITE_USER_DATA_LEN: usize = 0x1000;
#[cfg(talos_boot_scenario = "qemu_descriptor_write_smoke")]
const DESCRIPTOR_WRITE_STDOUT_OFFSET: usize = 0x00;
#[cfg(talos_boot_scenario = "qemu_descriptor_write_smoke")]
const DESCRIPTOR_WRITE_STDERR_OFFSET: usize = 0x40;
#[cfg(talos_boot_scenario = "qemu_descriptor_write_smoke")]
const DESCRIPTOR_WRITE_STDOUT: &[u8; 18] = b"talos-stdout-qemu\n";
#[cfg(talos_boot_scenario = "qemu_descriptor_write_smoke")]
const DESCRIPTOR_WRITE_STDERR: &[u8; 18] = b"talos-stderr-qemu\n";
#[cfg(talos_boot_scenario = "qemu_descriptor_write_smoke")]
const DESCRIPTOR_WRITE_EXPECTED_EBADF_X0: u64 = (syscall::EBADF as u64).wrapping_neg();
#[cfg(talos_boot_scenario = "qemu_descriptor_write_smoke")]
const DESCRIPTOR_WRITE_EXPECTED_EFAULT_X0: u64 = (syscall::EFAULT as u64).wrapping_neg();
#[cfg(talos_boot_scenario = "qemu_descriptor_write_smoke")]
const DESCRIPTOR_WRITE_EXPECTED_EINVAL_X0: u64 = (syscall::EINVAL as u64).wrapping_neg();
#[cfg(talos_boot_scenario = "qemu_descriptor_write_smoke")]
const DESCRIPTOR_WRITE_COPY_PROBE_NUMBER: u64 = 0x7001;
#[cfg(any(
    talos_boot_scenario = "qemu_el0_trap_smoke",
    talos_boot_scenario = "qemu_syscall_smoke",
    talos_boot_scenario = "qemu_pointer_copy_smoke",
    talos_boot_scenario = "qemu_descriptor_write_smoke"
))]
const EL0_TRAP_USER_STACK_START: u64 = 0x0000_0000_001f_0000;
#[cfg(any(
    talos_boot_scenario = "qemu_el0_trap_smoke",
    talos_boot_scenario = "qemu_syscall_smoke",
    talos_boot_scenario = "qemu_pointer_copy_smoke",
    talos_boot_scenario = "qemu_descriptor_write_smoke"
))]
const EL0_TRAP_USER_STACK_LEN: usize = 0x1_0000;
#[cfg(any(
    talos_boot_scenario = "qemu_el0_trap_smoke",
    talos_boot_scenario = "qemu_syscall_smoke",
    talos_boot_scenario = "qemu_pointer_copy_smoke",
    talos_boot_scenario = "qemu_descriptor_write_smoke"
))]
const EL0_TRAP_USER_GUARD_START: u64 = 0x0000_0000_001e_0000;
#[cfg(any(
    talos_boot_scenario = "qemu_el0_trap_smoke",
    talos_boot_scenario = "qemu_syscall_smoke",
    talos_boot_scenario = "qemu_pointer_copy_smoke",
    talos_boot_scenario = "qemu_descriptor_write_smoke"
))]
const EL0_TRAP_SVC_MARKER: u64 = 0x7a10;
#[cfg(talos_boot_scenario = "qemu_el0_trap_smoke")]
const EL0_TRAP_EXPECTED_ESR: u64 = 0x0000_0000_5400_7a10;
#[cfg(any(
    talos_boot_scenario = "qemu_syscall_smoke",
    talos_boot_scenario = "qemu_pointer_copy_smoke",
    talos_boot_scenario = "qemu_descriptor_write_smoke"
))]
const SYSCALL_SMOKE_EXPECTED_SVC_ESR: u64 = 0x0000_0000_5400_0000;
#[cfg(any(
    talos_boot_scenario = "qemu_syscall_smoke",
    talos_boot_scenario = "qemu_pointer_copy_smoke",
    talos_boot_scenario = "qemu_descriptor_write_smoke"
))]
const SYSCALL_SMOKE_EXPECTED_MARKER_ESR: u64 = 0x0000_0000_5400_7a10;
#[cfg(any(
    talos_boot_scenario = "qemu_syscall_smoke",
    talos_boot_scenario = "qemu_pointer_copy_smoke",
    talos_boot_scenario = "qemu_descriptor_write_smoke"
))]
const SYSCALL_SMOKE_UNKNOWN_NUMBER: u64 = 17;
#[cfg(any(
    talos_boot_scenario = "qemu_syscall_smoke",
    talos_boot_scenario = "qemu_pointer_copy_smoke",
    talos_boot_scenario = "qemu_descriptor_write_smoke"
))]
const SYSCALL_SMOKE_EXPECTED_ENOSYS_X0: u64 = (syscall::ENOSYS as u64).wrapping_neg();
#[cfg(talos_boot_scenario = "qemu_pointer_copy_smoke")]
const POINTER_COPY_EXPECTED_EFAULT_X0: u64 = (syscall::EFAULT as u64).wrapping_neg();
#[cfg(any(
    talos_boot_scenario = "qemu_el0_trap_smoke",
    talos_boot_scenario = "qemu_syscall_smoke",
    talos_boot_scenario = "qemu_pointer_copy_smoke",
    talos_boot_scenario = "qemu_descriptor_write_smoke"
))]
const EL0_TRAP_SPSR_EL0T_DAIF_MASKED: u64 = 0x3c0;
#[cfg(any(
    talos_boot_scenario = "qemu_el0_trap_smoke",
    talos_boot_scenario = "qemu_syscall_smoke",
    talos_boot_scenario = "qemu_pointer_copy_smoke",
    talos_boot_scenario = "qemu_descriptor_write_smoke"
))]
const EL0_TRAP_SPSR_EL1H_DAIF_MASKED: u64 = 0x3c5;

const MMIO_REGIONS: &[MmioRegion] = &[
    MmioRegion::new("qemu-virt-gicv2-distributor", GICD_BASE, 0x0001_0000),
    MmioRegion::new("qemu-virt-gicv2-cpu-interface", GICC_BASE, 0x0001_0000),
    MmioRegion::new("qemu-virt-pl011-uart0", PL011_BASE, 0x1000),
];

static LAST_IRQ_VECTOR: AtomicU64 = AtomicU64::new(0);
static LAST_IAR: AtomicU64 = AtomicU64::new(0);
static LAST_INTID: AtomicU64 = AtomicU64::new(0);
static UNEXPECTED_GIC_IRQ_COUNT: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_timer_preemption")]
static TIMER_PREEMPTION_REQUESTS: AtomicU64 = AtomicU64::new(0);

#[cfg(any(
    talos_boot_scenario = "qemu_el0_trap_smoke",
    talos_boot_scenario = "qemu_syscall_smoke",
    talos_boot_scenario = "qemu_pointer_copy_smoke",
    talos_boot_scenario = "qemu_descriptor_write_smoke"
))]
#[repr(align(4096))]
struct El0TrapPage([u64; 512]);

#[cfg(any(
    talos_boot_scenario = "qemu_el0_trap_smoke",
    talos_boot_scenario = "qemu_syscall_smoke",
    talos_boot_scenario = "qemu_pointer_copy_smoke",
    talos_boot_scenario = "qemu_descriptor_write_smoke"
))]
impl El0TrapPage {
    const fn zeroed() -> Self {
        Self([0; 512])
    }
}

#[cfg(any(
    talos_boot_scenario = "qemu_el0_trap_smoke",
    talos_boot_scenario = "qemu_syscall_smoke",
    talos_boot_scenario = "qemu_pointer_copy_smoke",
    talos_boot_scenario = "qemu_descriptor_write_smoke"
))]
#[repr(align(65536))]
struct El0TrapStack([u8; EL0_TRAP_USER_STACK_LEN]);

#[cfg(any(
    talos_boot_scenario = "qemu_el0_trap_smoke",
    talos_boot_scenario = "qemu_syscall_smoke",
    talos_boot_scenario = "qemu_pointer_copy_smoke",
    talos_boot_scenario = "qemu_descriptor_write_smoke"
))]
impl El0TrapStack {
    const fn zeroed() -> Self {
        Self([0; EL0_TRAP_USER_STACK_LEN])
    }
}

#[cfg(any(
    talos_boot_scenario = "qemu_el0_trap_smoke",
    talos_boot_scenario = "qemu_syscall_smoke",
    talos_boot_scenario = "qemu_pointer_copy_smoke",
    talos_boot_scenario = "qemu_descriptor_write_smoke"
))]
#[repr(align(4096))]
struct El0TrapPayload([u8; EL0_TRAP_USER_TEXT_LEN]);

#[cfg(talos_boot_scenario = "qemu_el0_trap_smoke")]
impl El0TrapPayload {
    const fn svc_marker() -> Self {
        let mut page = [0; EL0_TRAP_USER_TEXT_LEN];
        page[0] = 0x01;
        page[1] = 0x42;
        page[2] = 0x0f;
        page[3] = 0xd4;
        page[4] = 0x00;
        page[5] = 0x00;
        page[6] = 0x00;
        page[7] = 0x14;
        Self(page)
    }
}

#[cfg(talos_boot_scenario = "qemu_syscall_smoke")]
impl El0TrapPayload {
    const fn syscall_smoke() -> Self {
        let mut page = [0; EL0_TRAP_USER_TEXT_LEN];
        page[0] = 0x00;
        page[1] = 0x00;
        page[2] = 0x80;
        page[3] = 0xd2;
        page[4] = 0x01;
        page[5] = 0x00;
        page[6] = 0x80;
        page[7] = 0xd2;
        page[8] = 0x02;
        page[9] = 0x00;
        page[10] = 0x80;
        page[11] = 0xd2;
        page[12] = 0x03;
        page[13] = 0x00;
        page[14] = 0x80;
        page[15] = 0xd2;
        page[16] = 0x04;
        page[17] = 0x00;
        page[18] = 0x80;
        page[19] = 0xd2;
        page[20] = 0x05;
        page[21] = 0x00;
        page[22] = 0x80;
        page[23] = 0xd2;
        page[24] = 0x08;
        page[25] = 0x00;
        page[26] = 0x80;
        page[27] = 0xd2;
        page[28] = 0x01;
        page[29] = 0x00;
        page[30] = 0x00;
        page[31] = 0xd4;
        page[32] = 0x28;
        page[33] = 0x02;
        page[34] = 0x80;
        page[35] = 0xd2;
        page[36] = 0x01;
        page[37] = 0x00;
        page[38] = 0x00;
        page[39] = 0xd4;
        page[40] = 0x01;
        page[41] = 0x42;
        page[42] = 0x0f;
        page[43] = 0xd4;
        page[44] = 0x00;
        page[45] = 0x00;
        page[46] = 0x00;
        page[47] = 0x14;
        Self(page)
    }
}

#[cfg(talos_boot_scenario = "qemu_pointer_copy_smoke")]
impl El0TrapPayload {
    const fn pointer_copy_smoke() -> Self {
        let mut page = [0; EL0_TRAP_USER_TEXT_LEN];
        page[0] = 0x20;
        page[1] = 0x02;
        page[2] = 0xa0;
        page[3] = 0xd2;
        page[4] = 0x01;
        page[5] = 0x02;
        page[6] = 0x80;
        page[7] = 0xd2;
        page[8] = 0x42;
        page[9] = 0x05;
        page[10] = 0x80;
        page[11] = 0xd2;
        page[12] = 0xa3;
        page[13] = 0x14;
        page[14] = 0x80;
        page[15] = 0xd2;
        page[16] = 0x04;
        page[17] = 0x00;
        page[18] = 0x80;
        page[19] = 0xd2;
        page[20] = 0x05;
        page[21] = 0x00;
        page[22] = 0x80;
        page[23] = 0xd2;
        page[24] = 0x28;
        page[25] = 0x00;
        page[26] = 0x8e;
        page[27] = 0xd2;
        page[28] = 0x01;
        page[29] = 0x00;
        page[30] = 0x00;
        page[31] = 0xd4;
        page[32] = 0xc0;
        page[33] = 0x03;
        page[34] = 0xa0;
        page[35] = 0xd2;
        page[36] = 0x01;
        page[37] = 0x02;
        page[38] = 0x80;
        page[39] = 0xd2;
        page[40] = 0x42;
        page[41] = 0x05;
        page[42] = 0x80;
        page[43] = 0xd2;
        page[44] = 0xa3;
        page[45] = 0x14;
        page[46] = 0x80;
        page[47] = 0xd2;
        page[48] = 0x04;
        page[49] = 0x00;
        page[50] = 0x80;
        page[51] = 0xd2;
        page[52] = 0x05;
        page[53] = 0x00;
        page[54] = 0x80;
        page[55] = 0xd2;
        page[56] = 0x28;
        page[57] = 0x00;
        page[58] = 0x8e;
        page[59] = 0xd2;
        page[60] = 0x01;
        page[61] = 0x00;
        page[62] = 0x00;
        page[63] = 0xd4;
        page[64] = 0x28;
        page[65] = 0x02;
        page[66] = 0x80;
        page[67] = 0xd2;
        page[68] = 0x01;
        page[69] = 0x00;
        page[70] = 0x00;
        page[71] = 0xd4;
        page[72] = 0x01;
        page[73] = 0x42;
        page[74] = 0x0f;
        page[75] = 0xd4;
        page[76] = 0x00;
        page[77] = 0x00;
        page[78] = 0x00;
        page[79] = 0x14;
        Self(page)
    }
}

#[cfg(talos_boot_scenario = "qemu_descriptor_write_smoke")]
impl El0TrapPayload {
    const fn descriptor_write_smoke() -> Self {
        let mut page = [0; EL0_TRAP_USER_TEXT_LEN];
        let bytes = [
            0x20, 0x00, 0x80, 0xd2, 0x01, 0x00, 0x80, 0xd2, 0x21, 0x02, 0xa0, 0xf2, 0x42, 0x02,
            0x80, 0xd2, 0x03, 0x00, 0x80, 0xd2, 0x04, 0x00, 0x80, 0xd2, 0x05, 0x00, 0x80, 0xd2,
            0x28, 0x00, 0x80, 0xd2, 0x01, 0x00, 0x00, 0xd4, 0x40, 0x00, 0x80, 0xd2, 0x01, 0x08,
            0x80, 0xd2, 0x21, 0x02, 0xa0, 0xf2, 0x42, 0x02, 0x80, 0xd2, 0x03, 0x00, 0x80, 0xd2,
            0x04, 0x00, 0x80, 0xd2, 0x05, 0x00, 0x80, 0xd2, 0x28, 0x00, 0x80, 0xd2, 0x01, 0x00,
            0x00, 0xd4, 0x00, 0x00, 0x80, 0xd2, 0x01, 0x00, 0x80, 0xd2, 0x21, 0x02, 0xa0, 0xf2,
            0x42, 0x02, 0x80, 0xd2, 0x03, 0x00, 0x80, 0xd2, 0x04, 0x00, 0x80, 0xd2, 0x05, 0x00,
            0x80, 0xd2, 0x28, 0x00, 0x80, 0xd2, 0x01, 0x00, 0x00, 0xd4, 0x60, 0x0c, 0x80, 0xd2,
            0x01, 0x00, 0x80, 0xd2, 0x21, 0x02, 0xa0, 0xf2, 0x42, 0x02, 0x80, 0xd2, 0x03, 0x00,
            0x80, 0xd2, 0x04, 0x00, 0x80, 0xd2, 0x05, 0x00, 0x80, 0xd2, 0x28, 0x00, 0x80, 0xd2,
            0x01, 0x00, 0x00, 0xd4, 0x20, 0x00, 0x80, 0xd2, 0x01, 0x00, 0x80, 0xd2, 0xc1, 0x03,
            0xa0, 0xf2, 0x42, 0x02, 0x80, 0xd2, 0x03, 0x00, 0x80, 0xd2, 0x04, 0x00, 0x80, 0xd2,
            0x05, 0x00, 0x80, 0xd2, 0x28, 0x00, 0x80, 0xd2, 0x01, 0x00, 0x00, 0xd4, 0x20, 0x00,
            0x80, 0xd2, 0x01, 0x00, 0x80, 0xd2, 0x21, 0x02, 0xa0, 0xf2, 0x42, 0x02, 0x80, 0xd2,
            0x23, 0x00, 0x80, 0xd2, 0x04, 0x00, 0x80, 0xd2, 0x05, 0x00, 0x80, 0xd2, 0x28, 0x00,
            0x80, 0xd2, 0x01, 0x00, 0x00, 0xd4, 0x00, 0x00, 0x80, 0xd2, 0x01, 0x00, 0x80, 0xd2,
            0x02, 0x00, 0x80, 0xd2, 0x03, 0x00, 0x80, 0xd2, 0x04, 0x00, 0x80, 0xd2, 0x05, 0x00,
            0x80, 0xd2, 0x08, 0x00, 0x80, 0xd2, 0x01, 0x00, 0x00, 0xd4, 0x28, 0x02, 0x80, 0xd2,
            0x01, 0x00, 0x00, 0xd4, 0x00, 0x00, 0x80, 0xd2, 0x20, 0x02, 0xa0, 0xf2, 0x01, 0x02,
            0x80, 0xd2, 0x42, 0x05, 0x80, 0xd2, 0xa3, 0x14, 0x80, 0xd2, 0x04, 0x00, 0x80, 0xd2,
            0x05, 0x00, 0x80, 0xd2, 0x28, 0x00, 0x8e, 0xd2, 0x01, 0x00, 0x00, 0xd4, 0x01, 0x42,
            0x0f, 0xd4, 0x00, 0x00, 0x00, 0x14,
        ];
        let mut index = 0;
        while index < bytes.len() {
            page[index] = bytes[index];
            index += 1;
        }
        Self(page)
    }
}

#[cfg(talos_boot_scenario = "qemu_close_syscall_smoke")]
impl El0TrapPayload {
    const fn close_syscall_smoke() -> Self {
        let mut page = [0; EL0_TRAP_USER_TEXT_LEN];
        let bytes = [
            0x20, 0x00, 0x80, 0xd2, 0x01, 0x00, 0x80, 0xd2, 0x02, 0x00, 0x80, 0xd2, 0x03, 0x00,
            0x80, 0xd2, 0x04, 0x00, 0x80, 0xd2, 0x05, 0x00, 0x80, 0xd2, 0x48, 0x00, 0x80, 0xd2,
            0x01, 0x00, 0x00, 0xd4, 0x20, 0x00, 0x80, 0xd2, 0x01, 0x00, 0x80, 0xd2, 0x21, 0x02,
            0xa0, 0xf2, 0x42, 0x02, 0x80, 0xd2, 0x03, 0x00, 0x80, 0xd2, 0x04, 0x00, 0x80, 0xd2,
            0x05, 0x00, 0x80, 0xd2, 0x28, 0x00, 0x80, 0xd2, 0x01, 0x00, 0x00, 0xd4, 0x40, 0x00,
            0x80, 0xd2, 0x21, 0x00, 0x80, 0xd2, 0x02, 0x00, 0x80, 0xd2, 0x03, 0x00, 0x80, 0xd2,
            0x04, 0x00, 0x80, 0xd2, 0x05, 0x00, 0x80, 0xd2, 0x48, 0x00, 0x80, 0xd2, 0x01, 0x00,
            0x00, 0xd4, 0x40, 0x00, 0x80, 0xd2, 0x01, 0x08, 0x80, 0xd2, 0x21, 0x02, 0xa0, 0xf2,
            0x42, 0x02, 0x80, 0xd2, 0x03, 0x00, 0x80, 0xd2, 0x04, 0x00, 0x80, 0xd2, 0x05, 0x00,
            0x80, 0xd2, 0x28, 0x00, 0x80, 0xd2, 0x01, 0x00, 0x00, 0xd4, 0x40, 0x00, 0x80, 0xd2,
            0x01, 0x00, 0x80, 0xd2, 0x02, 0x00, 0x80, 0xd2, 0x03, 0x00, 0x80, 0xd2, 0x04, 0x00,
            0x80, 0xd2, 0x05, 0x00, 0x80, 0xd2, 0x48, 0x00, 0x80, 0xd2, 0x01, 0x00, 0x00, 0xd4,
            0x40, 0x00, 0x80, 0xd2, 0x01, 0x08, 0x80, 0xd2, 0x21, 0x02, 0xa0, 0xf2, 0x42, 0x02,
            0x80, 0xd2, 0x03, 0x00, 0x80, 0xd2, 0x04, 0x00, 0x80, 0xd2, 0x05, 0x00, 0x80, 0xd2,
            0x28, 0x00, 0x80, 0xd2, 0x01, 0x00, 0x00, 0xd4, 0x20, 0x00, 0x80, 0xd2, 0x01, 0x00,
            0x80, 0xd2, 0x02, 0x00, 0x80, 0xd2, 0x03, 0x00, 0x80, 0xd2, 0x04, 0x00, 0x80, 0xd2,
            0x05, 0x00, 0x80, 0xd2, 0x48, 0x00, 0x80, 0xd2, 0x01, 0x00, 0x00, 0xd4, 0x60, 0x0c,
            0x80, 0xd2, 0x01, 0x00, 0x80, 0xd2, 0x02, 0x00, 0x80, 0xd2, 0x03, 0x00, 0x80, 0xd2,
            0x04, 0x00, 0x80, 0xd2, 0x05, 0x00, 0x80, 0xd2, 0x48, 0x00, 0x80, 0xd2, 0x01, 0x00,
            0x00, 0xd4, 0x00, 0x00, 0x80, 0xd2, 0x01, 0x00, 0x80, 0xd2, 0x02, 0x00, 0x80, 0xd2,
            0x03, 0x00, 0x80, 0xd2, 0x04, 0x00, 0x80, 0xd2, 0x05, 0x00, 0x80, 0xd2, 0x08, 0x00,
            0x80, 0xd2, 0x01, 0x00, 0x00, 0xd4, 0x00, 0x00, 0x80, 0xd2, 0x01, 0x00, 0x80, 0xd2,
            0x02, 0x00, 0x80, 0xd2, 0x03, 0x00, 0x80, 0xd2, 0x04, 0x00, 0x80, 0xd2, 0x05, 0x00,
            0x80, 0xd2, 0x28, 0x02, 0x80, 0xd2, 0x01, 0x00, 0x00, 0xd4, 0x00, 0x00, 0x80, 0xd2,
            0x20, 0x02, 0xa0, 0xf2, 0x01, 0x02, 0x80, 0xd2, 0x42, 0x05, 0x80, 0xd2, 0xa3, 0x14,
            0x80, 0xd2, 0x04, 0x00, 0x80, 0xd2, 0x05, 0x00, 0x80, 0xd2, 0x28, 0x00, 0x8e, 0xd2,
            0x01, 0x00, 0x00, 0xd4, 0x01, 0x42, 0x0f, 0xd4, 0x00, 0x00, 0x00, 0x14,
        ];
        let mut index = 0;
        while index < bytes.len() {
            page[index] = bytes[index];
            index += 1;
        }
        Self(page)
    }
}

#[cfg(talos_boot_scenario = "qemu_dup_syscall_smoke")]
impl El0TrapPayload {
    const fn dup_syscall_smoke() -> Self {
        let mut page = [0; EL0_TRAP_USER_TEXT_LEN];
        let bytes = [
            0x20, 0x00, 0x80, 0xd2, 0x01, 0x00, 0x80, 0xd2, 0x02, 0x00, 0x80, 0xd2, 0x03, 0x00,
            0x80, 0xd2, 0x04, 0x00, 0x80, 0xd2, 0x05, 0x00, 0x80, 0xd2, 0x68, 0x00, 0x80, 0xd2,
            0x01, 0x00, 0x00, 0xd4, 0x40, 0x00, 0x80, 0xd2, 0x01, 0x00, 0x80, 0xd2, 0x02, 0x00,
            0x80, 0xd2, 0x03, 0x00, 0x80, 0xd2, 0x04, 0x00, 0x80, 0xd2, 0x05, 0x00, 0x80, 0xd2,
            0x68, 0x00, 0x80, 0xd2, 0x01, 0x00, 0x00, 0xd4, 0x20, 0x00, 0x80, 0xd2, 0x21, 0x00,
            0x80, 0xd2, 0x02, 0x00, 0x80, 0xd2, 0x03, 0x00, 0x80, 0xd2, 0x04, 0x00, 0x80, 0xd2,
            0x05, 0x00, 0x80, 0xd2, 0x68, 0x00, 0x80, 0xd2, 0x01, 0x00, 0x00, 0xd4, 0x20, 0x00,
            0x80, 0xd2, 0x01, 0x00, 0x80, 0xd2, 0x21, 0x02, 0xa0, 0xf2, 0x62, 0x02, 0x80, 0xd2,
            0x03, 0x00, 0x80, 0xd2, 0x04, 0x00, 0x80, 0xd2, 0x05, 0x00, 0x80, 0xd2, 0x28, 0x00,
            0x80, 0xd2, 0x01, 0x00, 0x00, 0xd4, 0x60, 0x00, 0x80, 0xd2, 0x01, 0x08, 0x80, 0xd2,
            0x21, 0x02, 0xa0, 0xf2, 0x62, 0x02, 0x80, 0xd2, 0x03, 0x00, 0x80, 0xd2, 0x04, 0x00,
            0x80, 0xd2, 0x05, 0x00, 0x80, 0xd2, 0x28, 0x00, 0x80, 0xd2, 0x01, 0x00, 0x00, 0xd4,
            0x20, 0x00, 0x80, 0xd2, 0x01, 0x00, 0x80, 0xd2, 0x02, 0x00, 0x80, 0xd2, 0x03, 0x00,
            0x80, 0xd2, 0x04, 0x00, 0x80, 0xd2, 0x05, 0x00, 0x80, 0xd2, 0x48, 0x00, 0x80, 0xd2,
            0x01, 0x00, 0x00, 0xd4, 0x20, 0x00, 0x80, 0xd2, 0x01, 0x00, 0x80, 0xd2, 0x21, 0x02,
            0xa0, 0xf2, 0x62, 0x02, 0x80, 0xd2, 0x03, 0x00, 0x80, 0xd2, 0x04, 0x00, 0x80, 0xd2,
            0x05, 0x00, 0x80, 0xd2, 0x28, 0x00, 0x80, 0xd2, 0x01, 0x00, 0x00, 0xd4, 0x60, 0x00,
            0x80, 0xd2, 0x01, 0x08, 0x80, 0xd2, 0x21, 0x02, 0xa0, 0xf2, 0x62, 0x02, 0x80, 0xd2,
            0x03, 0x00, 0x80, 0xd2, 0x04, 0x00, 0x80, 0xd2, 0x05, 0x00, 0x80, 0xd2, 0x28, 0x00,
            0x80, 0xd2, 0x01, 0x00, 0x00, 0xd4, 0x60, 0x00, 0x80, 0xd2, 0x01, 0x00, 0x80, 0xd2,
            0x02, 0x00, 0x80, 0xd2, 0x03, 0x00, 0x80, 0xd2, 0x04, 0x00, 0x80, 0xd2, 0x05, 0x00,
            0x80, 0xd2, 0x48, 0x00, 0x80, 0xd2, 0x01, 0x00, 0x00, 0xd4, 0x60, 0x00, 0x80, 0xd2,
            0x01, 0x08, 0x80, 0xd2, 0x21, 0x02, 0xa0, 0xf2, 0x62, 0x02, 0x80, 0xd2, 0x03, 0x00,
            0x80, 0xd2, 0x04, 0x00, 0x80, 0xd2, 0x05, 0x00, 0x80, 0xd2, 0x28, 0x00, 0x80, 0xd2,
            0x01, 0x00, 0x00, 0xd4, 0x20, 0x00, 0x80, 0xd2, 0x01, 0x00, 0x80, 0xd2, 0x02, 0x00,
            0x80, 0xd2, 0x03, 0x00, 0x80, 0xd2, 0x04, 0x00, 0x80, 0xd2, 0x05, 0x00, 0x80, 0xd2,
            0x68, 0x00, 0x80, 0xd2, 0x01, 0x00, 0x00, 0xd4, 0x00, 0x00, 0x80, 0xd2, 0x01, 0x00,
            0x80, 0xd2, 0x02, 0x00, 0x80, 0xd2, 0x03, 0x00, 0x80, 0xd2, 0x04, 0x00, 0x80, 0xd2,
            0x05, 0x00, 0x80, 0xd2, 0x08, 0x00, 0x80, 0xd2, 0x01, 0x00, 0x00, 0xd4, 0x00, 0x00,
            0x80, 0xd2, 0x01, 0x00, 0x80, 0xd2, 0x02, 0x00, 0x80, 0xd2, 0x03, 0x00, 0x80, 0xd2,
            0x04, 0x00, 0x80, 0xd2, 0x05, 0x00, 0x80, 0xd2, 0x28, 0x02, 0x80, 0xd2, 0x01, 0x00,
            0x00, 0xd4, 0x00, 0x00, 0x80, 0xd2, 0x20, 0x02, 0xa0, 0xf2, 0x01, 0x02, 0x80, 0xd2,
            0x42, 0x05, 0x80, 0xd2, 0xa3, 0x14, 0x80, 0xd2, 0x04, 0x00, 0x80, 0xd2, 0x05, 0x00,
            0x80, 0xd2, 0x28, 0x00, 0x8e, 0xd2, 0x01, 0x00, 0x00, 0xd4, 0x01, 0x42, 0x0f, 0xd4,
            0x00, 0x00, 0x00, 0x14,
        ];
        let mut index = 0;
        while index < bytes.len() {
            page[index] = bytes[index];
            index += 1;
        }
        Self(page)
    }
}

#[cfg(talos_boot_scenario = "qemu_read_stdin_smoke")]
impl El0TrapPayload {
    const fn read_stdin_smoke() -> Self {
        let mut page = [0; EL0_TRAP_USER_TEXT_LEN];
        let bytes = [
            0x00, 0x00, 0x80, 0xd2, 0x01, 0x00, 0x80, 0xd2, 0x02, 0x00, 0x80, 0xd2, 0x03, 0x00,
            0x80, 0xd2, 0x04, 0x00, 0x80, 0xd2, 0x05, 0x00, 0x80, 0xd2, 0x68, 0x00, 0x80, 0xd2,
            0x01, 0x00, 0x00, 0xd4, 0x00, 0x00, 0x80, 0xd2, 0xc1, 0x03, 0xa0, 0xd2, 0xa2, 0x00,
            0x80, 0xd2, 0x03, 0x00, 0x80, 0xd2, 0x04, 0x00, 0x80, 0xd2, 0x05, 0x00, 0x80, 0xd2,
            0x88, 0x00, 0x80, 0xd2, 0x01, 0x00, 0x00, 0xd4, 0x00, 0x00, 0x80, 0xd2, 0x01, 0x10,
            0x80, 0xd2, 0x21, 0x02, 0xa0, 0xf2, 0xa2, 0x00, 0x80, 0xd2, 0x23, 0x00, 0x80, 0xd2,
            0x04, 0x00, 0x80, 0xd2, 0x05, 0x00, 0x80, 0xd2, 0x88, 0x00, 0x80, 0xd2, 0x01, 0x00,
            0x00, 0xd4, 0x20, 0x00, 0x80, 0xd2, 0x01, 0x10, 0x80, 0xd2, 0x21, 0x02, 0xa0, 0xf2,
            0xa2, 0x00, 0x80, 0xd2, 0x03, 0x00, 0x80, 0xd2, 0x04, 0x00, 0x80, 0xd2, 0x05, 0x00,
            0x80, 0xd2, 0x88, 0x00, 0x80, 0xd2, 0x01, 0x00, 0x00, 0xd4, 0x60, 0x0c, 0x80, 0xd2,
            0x01, 0x10, 0x80, 0xd2, 0x21, 0x02, 0xa0, 0xf2, 0xa2, 0x00, 0x80, 0xd2, 0x03, 0x00,
            0x80, 0xd2, 0x04, 0x00, 0x80, 0xd2, 0x05, 0x00, 0x80, 0xd2, 0x88, 0x00, 0x80, 0xd2,
            0x01, 0x00, 0x00, 0xd4, 0x00, 0x00, 0x80, 0xd2, 0x01, 0x10, 0x80, 0xd2, 0x21, 0x02,
            0xa0, 0xf2, 0xa2, 0x00, 0x80, 0xd2, 0x03, 0x00, 0x80, 0xd2, 0x04, 0x00, 0x80, 0xd2,
            0x05, 0x00, 0x80, 0xd2, 0x88, 0x00, 0x80, 0xd2, 0x01, 0x00, 0x00, 0xd4, 0x60, 0x00,
            0x80, 0xd2, 0x01, 0x14, 0x80, 0xd2, 0x21, 0x02, 0xa0, 0xf2, 0x02, 0x04, 0x80, 0xd2,
            0x03, 0x00, 0x80, 0xd2, 0x04, 0x00, 0x80, 0xd2, 0x05, 0x00, 0x80, 0xd2, 0x88, 0x00,
            0x80, 0xd2, 0x01, 0x00, 0x00, 0xd4, 0x00, 0x00, 0x80, 0xd2, 0x01, 0x18, 0x80, 0xd2,
            0x21, 0x02, 0xa0, 0xf2, 0x22, 0x00, 0x80, 0xd2, 0x03, 0x00, 0x80, 0xd2, 0x04, 0x00,
            0x80, 0xd2, 0x05, 0x00, 0x80, 0xd2, 0x88, 0x00, 0x80, 0xd2, 0x01, 0x00, 0x00, 0xd4,
            0x00, 0x00, 0x80, 0xd2, 0x01, 0x00, 0x80, 0xd2, 0x02, 0x00, 0x80, 0xd2, 0x03, 0x00,
            0x80, 0xd2, 0x04, 0x00, 0x80, 0xd2, 0x05, 0x00, 0x80, 0xd2, 0x08, 0x00, 0x80, 0xd2,
            0x01, 0x00, 0x00, 0xd4, 0x00, 0x00, 0x80, 0xd2, 0x01, 0x00, 0x80, 0xd2, 0x02, 0x00,
            0x80, 0xd2, 0x03, 0x00, 0x80, 0xd2, 0x04, 0x00, 0x80, 0xd2, 0x05, 0x00, 0x80, 0xd2,
            0x28, 0x02, 0x80, 0xd2, 0x01, 0x00, 0x00, 0xd4, 0x00, 0x00, 0x80, 0xd2, 0x01, 0x10,
            0x80, 0xd2, 0x21, 0x02, 0xa0, 0xf2, 0xa2, 0x00, 0x80, 0xd2, 0x03, 0x00, 0x80, 0xd2,
            0x04, 0x00, 0x80, 0xd2, 0x05, 0x00, 0x80, 0xd2, 0x28, 0x00, 0x8e, 0xd2, 0x01, 0x00,
            0x00, 0xd4, 0x01, 0x42, 0x0f, 0xd4, 0x00, 0x00, 0x00, 0x14,
        ];
        let mut index = 0;
        while index < bytes.len() {
            page[index] = bytes[index];
            index += 1;
        }
        Self(page)
    }
}

#[cfg(any(
    talos_boot_scenario = "qemu_el0_trap_smoke",
    talos_boot_scenario = "qemu_syscall_smoke",
    talos_boot_scenario = "qemu_pointer_copy_smoke",
    talos_boot_scenario = "qemu_descriptor_write_smoke"
))]
static mut EL0_TRAP_ROOT_TABLE: El0TrapPage = El0TrapPage::zeroed();
#[cfg(any(
    talos_boot_scenario = "qemu_el0_trap_smoke",
    talos_boot_scenario = "qemu_syscall_smoke",
    talos_boot_scenario = "qemu_pointer_copy_smoke",
    talos_boot_scenario = "qemu_descriptor_write_smoke"
))]
static mut EL0_TRAP_L1_TABLE: El0TrapPage = El0TrapPage::zeroed();
#[cfg(any(
    talos_boot_scenario = "qemu_el0_trap_smoke",
    talos_boot_scenario = "qemu_syscall_smoke",
    talos_boot_scenario = "qemu_pointer_copy_smoke",
    talos_boot_scenario = "qemu_descriptor_write_smoke"
))]
static mut EL0_TRAP_LOW_L2_TABLE: El0TrapPage = El0TrapPage::zeroed();
#[cfg(any(
    talos_boot_scenario = "qemu_el0_trap_smoke",
    talos_boot_scenario = "qemu_syscall_smoke",
    talos_boot_scenario = "qemu_pointer_copy_smoke",
    talos_boot_scenario = "qemu_descriptor_write_smoke"
))]
static mut EL0_TRAP_LOW_L3_TABLE: El0TrapPage = El0TrapPage::zeroed();
#[cfg(any(
    talos_boot_scenario = "qemu_el0_trap_smoke",
    talos_boot_scenario = "qemu_syscall_smoke",
    talos_boot_scenario = "qemu_pointer_copy_smoke",
    talos_boot_scenario = "qemu_descriptor_write_smoke"
))]
static mut EL0_TRAP_STACK: El0TrapStack = El0TrapStack::zeroed();
#[cfg(talos_boot_scenario = "qemu_el0_trap_smoke")]
static EL0_TRAP_PAYLOAD: El0TrapPayload = El0TrapPayload::svc_marker();
#[cfg(talos_boot_scenario = "qemu_syscall_smoke")]
static EL0_TRAP_PAYLOAD: El0TrapPayload = El0TrapPayload::syscall_smoke();
#[cfg(talos_boot_scenario = "qemu_pointer_copy_smoke")]
static EL0_TRAP_PAYLOAD: El0TrapPayload = El0TrapPayload::pointer_copy_smoke();
#[cfg(all(
    talos_boot_scenario = "qemu_descriptor_write_smoke",
    not(talos_boot_scenario = "qemu_process_descriptor_stdio_smoke"),
    not(talos_boot_scenario = "qemu_close_syscall_smoke"),
    not(talos_boot_scenario = "qemu_dup_syscall_smoke"),
    not(talos_boot_scenario = "qemu_read_stdin_smoke")
))]
static EL0_TRAP_PAYLOAD: El0TrapPayload = El0TrapPayload::descriptor_write_smoke();
#[cfg(talos_boot_scenario = "qemu_process_descriptor_stdio_smoke")]
static EL0_TRAP_PAYLOAD: El0TrapPayload = El0TrapPayload::descriptor_write_smoke();
#[cfg(talos_boot_scenario = "qemu_close_syscall_smoke")]
static EL0_TRAP_PAYLOAD: El0TrapPayload = El0TrapPayload::close_syscall_smoke();
#[cfg(talos_boot_scenario = "qemu_dup_syscall_smoke")]
static EL0_TRAP_PAYLOAD: El0TrapPayload = El0TrapPayload::dup_syscall_smoke();
#[cfg(talos_boot_scenario = "qemu_read_stdin_smoke")]
static EL0_TRAP_PAYLOAD: El0TrapPayload = El0TrapPayload::read_stdin_smoke();
#[cfg(talos_boot_scenario = "qemu_pointer_copy_smoke")]
static mut POINTER_COPY_USER_DATA: [u8; POINTER_COPY_USER_DATA_LEN] =
    [0; POINTER_COPY_USER_DATA_LEN];
#[cfg(talos_boot_scenario = "qemu_descriptor_write_smoke")]
static mut DESCRIPTOR_WRITE_USER_DATA: [u8; DESCRIPTOR_WRITE_USER_DATA_LEN] =
    [0; DESCRIPTOR_WRITE_USER_DATA_LEN];
#[cfg(talos_boot_scenario = "qemu_descriptor_write_smoke")]
static mut DESCRIPTOR_WRITE_CONSOLE_CAPTURE: [u8; 64] = [0; 64];
#[cfg(talos_boot_scenario = "qemu_descriptor_write_smoke")]
static DESCRIPTOR_WRITE_CONSOLE_LEN: AtomicU64 = AtomicU64::new(0);
#[cfg(any(
    talos_boot_scenario = "qemu_process_descriptor_stdio_smoke",
    talos_boot_scenario = "qemu_close_syscall_smoke",
    talos_boot_scenario = "qemu_dup_syscall_smoke",
    talos_boot_scenario = "qemu_read_stdin_smoke"
))]
static mut PROCESS_DESCRIPTOR_STDIO_STORE: crate::posix::ProcessDescriptorStore<1, 4> =
    crate::posix::ProcessDescriptorStore::new_empty();
#[cfg(any(
    talos_boot_scenario = "qemu_process_descriptor_stdio_smoke",
    talos_boot_scenario = "qemu_close_syscall_smoke",
    talos_boot_scenario = "qemu_dup_syscall_smoke",
    talos_boot_scenario = "qemu_read_stdin_smoke"
))]
const PROCESS_DESCRIPTOR_STDIO_OWNER_RAW: u64 = 1;
#[cfg(talos_boot_scenario = "qemu_read_stdin_smoke")]
static mut READ_STDIN_FIXED_STATE: crate::posix::FixedStdin<'static> =
    crate::posix::FixedStdin::new(READ_STDIN_FIXED_BYTES);
#[cfg(talos_boot_scenario = "qemu_descriptor_write_smoke")]
static DESCRIPTOR_WRITE_STDOUT_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_descriptor_write_smoke")]
static DESCRIPTOR_WRITE_STDERR_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_descriptor_write_smoke")]
static DESCRIPTOR_WRITE_FD0_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_descriptor_write_smoke")]
static DESCRIPTOR_WRITE_BADFD_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_descriptor_write_smoke")]
static DESCRIPTOR_WRITE_EFAULT_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_descriptor_write_smoke")]
static DESCRIPTOR_WRITE_RESERVED_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_descriptor_write_smoke")]
static DESCRIPTOR_WRITE_TALOS_NOP_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_descriptor_write_smoke")]
static DESCRIPTOR_WRITE_UNKNOWN_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_descriptor_write_smoke")]
static DESCRIPTOR_WRITE_COPY_PROBE_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_descriptor_write_smoke")]
static DESCRIPTOR_WRITE_ERRORS: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_close_syscall_smoke")]
static CLOSE_SYSCALL_CLOSE_STDOUT_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_close_syscall_smoke")]
static CLOSE_SYSCALL_WRITE_CLOSED_STDOUT_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_close_syscall_smoke")]
static CLOSE_SYSCALL_CLOSE_STDERR_RESERVED_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_close_syscall_smoke")]
static CLOSE_SYSCALL_WRITE_STDERR_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_close_syscall_smoke")]
static CLOSE_SYSCALL_CLOSE_STDERR_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_close_syscall_smoke")]
static CLOSE_SYSCALL_WRITE_CLOSED_STDERR_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_close_syscall_smoke")]
static CLOSE_SYSCALL_CLOSE_STDOUT_AGAIN_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_close_syscall_smoke")]
static CLOSE_SYSCALL_CLOSE_BADFD_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_close_syscall_smoke")]
static CLOSE_SYSCALL_TALOS_NOP_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_close_syscall_smoke")]
static CLOSE_SYSCALL_UNKNOWN_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_close_syscall_smoke")]
static CLOSE_SYSCALL_COPY_PROBE_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_dup_syscall_smoke")]
const DUP_SYSCALL_SOURCE: &[u8; 19] = b"talos-dup-src-qemu\n";
#[cfg(talos_boot_scenario = "qemu_dup_syscall_smoke")]
const DUP_SYSCALL_DUPLICATE: &[u8; 19] = b"talos-dup-new-qemu\n";
#[cfg(talos_boot_scenario = "qemu_read_stdin_smoke")]
const READ_STDIN_FIXED_BYTES: &[u8; 17] = b"talos-stdin-qemu\n";
#[cfg(talos_boot_scenario = "qemu_dup_syscall_smoke")]
static DUP_SYSCALL_DUP_STDOUT_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_dup_syscall_smoke")]
static DUP_SYSCALL_DUP_STDERR_FULL_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_dup_syscall_smoke")]
static DUP_SYSCALL_DUP_STDOUT_RESERVED_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_dup_syscall_smoke")]
static DUP_SYSCALL_WRITE_SOURCE_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_dup_syscall_smoke")]
static DUP_SYSCALL_WRITE_DUPLICATE_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_dup_syscall_smoke")]
static DUP_SYSCALL_CLOSE_SOURCE_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_dup_syscall_smoke")]
static DUP_SYSCALL_WRITE_SOURCE_CLOSED_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_dup_syscall_smoke")]
static DUP_SYSCALL_WRITE_DUPLICATE_AFTER_SOURCE_CLOSE_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_dup_syscall_smoke")]
static DUP_SYSCALL_CLOSE_DUPLICATE_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_dup_syscall_smoke")]
static DUP_SYSCALL_WRITE_DUPLICATE_CLOSED_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_dup_syscall_smoke")]
static DUP_SYSCALL_DUP_CLOSED_SOURCE_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_dup_syscall_smoke")]
static DUP_SYSCALL_TALOS_NOP_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_dup_syscall_smoke")]
static DUP_SYSCALL_UNKNOWN_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_dup_syscall_smoke")]
static DUP_SYSCALL_COPY_PROBE_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_read_stdin_smoke")]
static READ_STDIN_DUP_STDIN_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_read_stdin_smoke")]
static READ_STDIN_GUARD_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_read_stdin_smoke")]
static READ_STDIN_RESERVED_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_read_stdin_smoke")]
static READ_STDIN_FD1_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_read_stdin_smoke")]
static READ_STDIN_BADFD_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_read_stdin_smoke")]
static READ_STDIN_FIRST_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_read_stdin_smoke")]
static READ_STDIN_DUPLICATE_REMAINING_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_read_stdin_smoke")]
static READ_STDIN_EOF_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_read_stdin_smoke")]
static READ_STDIN_TALOS_NOP_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_read_stdin_smoke")]
static READ_STDIN_UNKNOWN_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_read_stdin_smoke")]
static READ_STDIN_COPY_PROBE_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_read_stdin_smoke")]
static READ_STDIN_ERRORS: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_syscall_smoke")]
static SYSCALL_SMOKE_TALOS_NOP_DISPATCHED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_syscall_smoke")]
static SYSCALL_SMOKE_TALOS_NOP_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_syscall_smoke")]
static SYSCALL_SMOKE_UNKNOWN_DISPATCHED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_syscall_smoke")]
static SYSCALL_SMOKE_UNKNOWN_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_syscall_smoke")]
static SYSCALL_SMOKE_ERRORS: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_pointer_copy_smoke")]
static POINTER_COPY_SUCCESS_DISPATCHED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_pointer_copy_smoke")]
static POINTER_COPY_SUCCESS_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_pointer_copy_smoke")]
static POINTER_COPY_EFAULT_DISPATCHED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_pointer_copy_smoke")]
static POINTER_COPY_EFAULT_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_pointer_copy_smoke")]
static POINTER_COPY_UNKNOWN_DISPATCHED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_pointer_copy_smoke")]
static POINTER_COPY_UNKNOWN_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "qemu_pointer_copy_smoke")]
static POINTER_COPY_ERRORS: AtomicU64 = AtomicU64::new(0);

#[cfg(any(
    talos_boot_scenario = "qemu_secondary_core_workload",
    talos_boot_scenario = "qemu_smp_lock_contention",
    talos_boot_scenario = "qemu_per_core_scheduler_ownership",
    talos_boot_scenario = "qemu_cross_core_ipi_delivery",
    talos_boot_scenario = "qemu_remote_wakeup_request",
    talos_boot_scenario = "qemu_production_secondary_dispatch",
    talos_boot_scenario = "qemu_shared_scheduler_metadata",
    talos_boot_scenario = "qemu_secondary_scheduler_service_loop",
    talos_boot_scenario = "qemu_multicore_preemption_smoke",
    talos_boot_scenario = "qemu_production_timer_preemption_smoke"
))]
unsafe extern "C" {
    fn talos_aarch64_qemu_secondary_entry();
    static talos_secondary_core_stacks: u8;
    static talos_secondary_core_stacks_end: u8;
}

#[cfg(any(
    talos_boot_scenario = "qemu_context_switch",
    talos_boot_scenario = "qemu_scheduler_yield",
    talos_boot_scenario = "qemu_timer_preemption"
))]
#[repr(align(16))]
struct KernelThreadStack([u8; CONTEXT_SWITCH_STACK_SIZE]);

#[cfg(any(
    talos_boot_scenario = "qemu_context_switch",
    talos_boot_scenario = "qemu_scheduler_yield",
    talos_boot_scenario = "qemu_timer_preemption"
))]
impl KernelThreadStack {
    const fn new() -> Self {
        Self([0; CONTEXT_SWITCH_STACK_SIZE])
    }

    fn top(&self) -> usize {
        self.0.as_ptr() as usize + self.0.len()
    }
}

#[cfg(talos_boot_scenario = "qemu_context_switch")]
struct ContextSwitchSmokeState {
    main_context: ContextFrame,
    worker_contexts: [ContextFrame; 2],
    worker_stacks: [KernelThreadStack; 2],
    progress: [u64; 2],
    switch_count: u64,
    current_task: u64,
    runnable_task: u64,
}

#[cfg(talos_boot_scenario = "qemu_context_switch")]
impl ContextSwitchSmokeState {
    const fn new() -> Self {
        Self {
            main_context: ContextFrame::new(0, 0),
            worker_contexts: [ContextFrame::new(0, 0); 2],
            worker_stacks: [KernelThreadStack::new(), KernelThreadStack::new()],
            progress: [0; 2],
            switch_count: 0,
            current_task: 0,
            runnable_task: 0,
        }
    }

    fn reset(&mut self) {
        self.progress = [0; 2];
        self.switch_count = 0;
        self.current_task = 0;
        self.runnable_task = 1;
        self.worker_contexts[0] = ContextFrame::kernel_thread_bootstrap(
            self.worker_stacks[0].top(),
            aarch64::kernel_thread_trampoline_address(),
            qemu_context_switch_thread as *const () as usize,
            0,
        );
        self.worker_contexts[1] = ContextFrame::kernel_thread_bootstrap(
            self.worker_stacks[1].top(),
            aarch64::kernel_thread_trampoline_address(),
            qemu_context_switch_thread as *const () as usize,
            1,
        );
    }

    fn all_workers_made_progress(&self) -> bool {
        self.progress[0] >= CONTEXT_SWITCH_TARGET_PROGRESS
            && self.progress[1] >= CONTEXT_SWITCH_TARGET_PROGRESS
    }
}

#[cfg(talos_boot_scenario = "qemu_context_switch")]
struct ContextSwitchSmokeCell(UnsafeCell<ContextSwitchSmokeState>);

#[cfg(talos_boot_scenario = "qemu_context_switch")]
unsafe impl Sync for ContextSwitchSmokeCell {}

#[cfg(talos_boot_scenario = "qemu_context_switch")]
impl ContextSwitchSmokeCell {
    const fn new() -> Self {
        Self(UnsafeCell::new(ContextSwitchSmokeState::new()))
    }

    unsafe fn get(&self) -> *mut ContextSwitchSmokeState {
        self.0.get()
    }
}

#[cfg(talos_boot_scenario = "qemu_context_switch")]
static CONTEXT_SWITCH_SMOKE: ContextSwitchSmokeCell = ContextSwitchSmokeCell::new();

#[cfg(talos_boot_scenario = "qemu_scheduler_yield")]
struct SchedulerYieldSmokeState {
    main_context: ContextFrame,
    worker_contexts: [ContextFrame; 2],
    worker_stacks: [KernelThreadStack; 2],
    tasks: [Option<Task>; 2],
    scheduler: SingleCoreScheduler<2>,
    progress: [u64; 2],
    current_task: u64,
    runnable_task: u64,
    yielded_task: u64,
}

#[cfg(talos_boot_scenario = "qemu_scheduler_yield")]
impl SchedulerYieldSmokeState {
    const fn new() -> Self {
        Self {
            main_context: ContextFrame::new(0, 0),
            worker_contexts: [ContextFrame::new(0, 0); 2],
            worker_stacks: [KernelThreadStack::new(), KernelThreadStack::new()],
            tasks: [None, None],
            scheduler: SingleCoreScheduler::new(),
            progress: [0; 2],
            current_task: 0,
            runnable_task: 0,
            yielded_task: 0,
        }
    }

    fn reset(&mut self) {
        self.progress = [0; 2];
        self.current_task = 1;
        self.runnable_task = 2;
        self.yielded_task = 0;
        self.scheduler = SingleCoreScheduler::new();

        self.worker_contexts[0] = ContextFrame::kernel_thread_bootstrap(
            self.worker_stacks[0].top(),
            aarch64::kernel_thread_trampoline_address(),
            qemu_scheduler_yield_thread as *const () as usize,
            0,
        );
        self.worker_contexts[1] = ContextFrame::kernel_thread_bootstrap(
            self.worker_stacks[1].top(),
            aarch64::kernel_thread_trampoline_address(),
            qemu_scheduler_yield_thread as *const () as usize,
            1,
        );

        let task1_id = TaskId::new(1).expect("nonzero task id");
        let task2_id = TaskId::new(2).expect("nonzero task id");
        let stack1 = KernelStack::new(
            self.worker_stacks[0].top() - CONTEXT_SWITCH_STACK_SIZE,
            CONTEXT_SWITCH_STACK_SIZE,
        )
        .expect("valid task 1 stack");
        let stack2 = KernelStack::new(
            self.worker_stacks[1].top() - CONTEXT_SWITCH_STACK_SIZE,
            CONTEXT_SWITCH_STACK_SIZE,
        )
        .expect("valid task 2 stack");
        let mut task1 = Task::kernel_thread(task1_id, stack1, self.worker_contexts[0]);
        let mut task2 = Task::kernel_thread(task2_id, stack2, self.worker_contexts[1]);
        task1.set_state(TaskState::Running);
        self.scheduler
            .make_runnable(&mut task2)
            .expect("scheduler-yield smoke has runnable capacity");
        self.tasks = [Some(task1), Some(task2)];
    }

    fn all_workers_made_progress(&self) -> bool {
        self.progress[0] >= SCHEDULER_YIELD_TARGET_PROGRESS
            && self.progress[1] >= SCHEDULER_YIELD_TARGET_PROGRESS
    }

    fn dispatch_voluntary_yield_from(&mut self, task_index: usize) -> usize {
        let current = self.tasks[task_index]
            .as_mut()
            .expect("current scheduler-yield task exists");
        let yielded_task = current.id();
        let next_task = self
            .scheduler
            .voluntary_yield(current)
            .expect("scheduler-yield smoke has a runnable peer");
        let next_task_index = (next_task.raw() - 1) as usize;
        self.tasks[next_task_index]
            .as_mut()
            .expect("next scheduler-yield task exists")
            .set_state(TaskState::Running);
        self.current_task = next_task.raw();
        self.runnable_task = self
            .scheduler
            .runnable()
            .front()
            .map_or(0, |task_id| task_id.raw());
        self.yielded_task = yielded_task.raw();
        next_task_index
    }
}

#[cfg(talos_boot_scenario = "qemu_scheduler_yield")]
struct SchedulerYieldSmokeCell(UnsafeCell<SchedulerYieldSmokeState>);

#[cfg(talos_boot_scenario = "qemu_scheduler_yield")]
unsafe impl Sync for SchedulerYieldSmokeCell {}

#[cfg(talos_boot_scenario = "qemu_scheduler_yield")]
impl SchedulerYieldSmokeCell {
    const fn new() -> Self {
        Self(UnsafeCell::new(SchedulerYieldSmokeState::new()))
    }

    unsafe fn get(&self) -> *mut SchedulerYieldSmokeState {
        self.0.get()
    }
}

#[cfg(talos_boot_scenario = "qemu_scheduler_yield")]
static SCHEDULER_YIELD_SMOKE: SchedulerYieldSmokeCell = SchedulerYieldSmokeCell::new();

#[cfg(talos_boot_scenario = "qemu_timer_preemption")]
struct TimerPreemptionSmokeState {
    main_context: ContextFrame,
    worker_contexts: [ContextFrame; 2],
    worker_stacks: [KernelThreadStack; 2],
    tasks: [Option<Task>; 2],
    scheduler: SingleCoreScheduler<2>,
    progress: [u64; 2],
    handled_requests: u64,
    current_task: u64,
    runnable_task: u64,
    preempted_task: u64,
}

#[cfg(talos_boot_scenario = "qemu_timer_preemption")]
impl TimerPreemptionSmokeState {
    const fn new() -> Self {
        Self {
            main_context: ContextFrame::new(0, 0),
            worker_contexts: [ContextFrame::new(0, 0); 2],
            worker_stacks: [KernelThreadStack::new(), KernelThreadStack::new()],
            tasks: [None, None],
            scheduler: SingleCoreScheduler::new(),
            progress: [0; 2],
            handled_requests: 0,
            current_task: 0,
            runnable_task: 0,
            preempted_task: 0,
        }
    }

    fn reset(&mut self) {
        self.progress = [0; 2];
        self.handled_requests = 0;
        self.current_task = 1;
        self.runnable_task = 2;
        self.preempted_task = 0;
        self.scheduler = SingleCoreScheduler::new();

        self.worker_contexts[0] = ContextFrame::kernel_thread_bootstrap(
            self.worker_stacks[0].top(),
            aarch64::kernel_thread_trampoline_address(),
            qemu_timer_preemption_thread as *const () as usize,
            0,
        );
        self.worker_contexts[1] = ContextFrame::kernel_thread_bootstrap(
            self.worker_stacks[1].top(),
            aarch64::kernel_thread_trampoline_address(),
            qemu_timer_preemption_thread as *const () as usize,
            1,
        );

        let task1_id = TaskId::new(1).expect("nonzero task id");
        let task2_id = TaskId::new(2).expect("nonzero task id");
        let stack1 = KernelStack::new(
            self.worker_stacks[0].top() - CONTEXT_SWITCH_STACK_SIZE,
            CONTEXT_SWITCH_STACK_SIZE,
        )
        .expect("valid timer-preemption task 1 stack");
        let stack2 = KernelStack::new(
            self.worker_stacks[1].top() - CONTEXT_SWITCH_STACK_SIZE,
            CONTEXT_SWITCH_STACK_SIZE,
        )
        .expect("valid timer-preemption task 2 stack");
        let mut task1 = Task::kernel_thread(task1_id, stack1, self.worker_contexts[0]);
        let mut task2 = Task::kernel_thread(task2_id, stack2, self.worker_contexts[1]);
        task1.set_state(TaskState::Running);
        self.scheduler
            .make_runnable(&mut task2)
            .expect("timer-preemption smoke has runnable capacity");
        self.tasks = [Some(task1), Some(task2)];
    }

    fn proof_complete(&self) -> bool {
        let counters = self.scheduler.counters();
        self.progress[0] >= TIMER_PREEMPTION_TARGET_PROGRESS
            && self.progress[1] >= TIMER_PREEMPTION_TARGET_PROGRESS
            && counters.timer_preemptions() >= TIMER_PREEMPTION_TARGET_SWITCHES
    }

    fn dispatch_timer_preemption_from(&mut self, task_index: usize, request_count: u64) -> usize {
        let current = self.tasks[task_index]
            .as_mut()
            .expect("current timer-preemption task exists");
        let preempted_task = current.id();
        let next_task = self
            .scheduler
            .timer_preempt(current)
            .expect("timer-preemption smoke has a runnable peer");
        let next_task_index = (next_task.raw() - 1) as usize;
        self.tasks[next_task_index]
            .as_mut()
            .expect("next timer-preemption task exists")
            .set_state(TaskState::Running);
        self.handled_requests = request_count;
        self.current_task = next_task.raw();
        self.runnable_task = self
            .scheduler
            .runnable()
            .front()
            .map_or(0, |task_id| task_id.raw());
        self.preempted_task = preempted_task.raw();
        next_task_index
    }
}

#[cfg(talos_boot_scenario = "qemu_timer_preemption")]
struct TimerPreemptionSmokeCell(UnsafeCell<TimerPreemptionSmokeState>);

#[cfg(talos_boot_scenario = "qemu_timer_preemption")]
unsafe impl Sync for TimerPreemptionSmokeCell {}

#[cfg(talos_boot_scenario = "qemu_timer_preemption")]
impl TimerPreemptionSmokeCell {
    const fn new() -> Self {
        Self(UnsafeCell::new(TimerPreemptionSmokeState::new()))
    }

    unsafe fn get(&self) -> *mut TimerPreemptionSmokeState {
        self.0.get()
    }
}

#[cfg(talos_boot_scenario = "qemu_timer_preemption")]
static TIMER_PREEMPTION_SMOKE: TimerPreemptionSmokeCell = TimerPreemptionSmokeCell::new();

const PRODUCTION_TIMER_PREEMPTION_RUNNABLE_CAPACITY: usize = 2;
const PRODUCTION_TIMER_PREEMPTION_REMOTE_WAKE_CAPACITY: usize = 1;

struct ProductionSchedulerRuntimeCell(
    core::cell::UnsafeCell<
        crate::scheduler::ProductionSchedulerRuntime<
            PRODUCTION_TIMER_PREEMPTION_RUNNABLE_CAPACITY,
            PRODUCTION_TIMER_PREEMPTION_REMOTE_WAKE_CAPACITY,
        >,
    >,
);

unsafe impl Sync for ProductionSchedulerRuntimeCell {}

impl ProductionSchedulerRuntimeCell {
    const fn new(
        runtime: crate::scheduler::ProductionSchedulerRuntime<
            PRODUCTION_TIMER_PREEMPTION_RUNNABLE_CAPACITY,
            PRODUCTION_TIMER_PREEMPTION_REMOTE_WAKE_CAPACITY,
        >,
    ) -> Self {
        Self(core::cell::UnsafeCell::new(runtime))
    }

    unsafe fn get(
        &self,
    ) -> *mut crate::scheduler::ProductionSchedulerRuntime<
        PRODUCTION_TIMER_PREEMPTION_RUNNABLE_CAPACITY,
        PRODUCTION_TIMER_PREEMPTION_REMOTE_WAKE_CAPACITY,
    > {
        self.0.get()
    }
}

static PRODUCTION_SCHEDULER_RUNTIMES: [ProductionSchedulerRuntimeCell; MAX_CORES] = [
    ProductionSchedulerRuntimeCell::new(crate::scheduler::ProductionSchedulerRuntime::boot_cpu()),
    ProductionSchedulerRuntimeCell::new(
        crate::scheduler::ProductionSchedulerRuntime::deferred_secondary(
            crate::scheduler::LogicalCpuId::new(1),
        ),
    ),
    ProductionSchedulerRuntimeCell::new(
        crate::scheduler::ProductionSchedulerRuntime::deferred_secondary(
            crate::scheduler::LogicalCpuId::new(2),
        ),
    ),
    ProductionSchedulerRuntimeCell::new(
        crate::scheduler::ProductionSchedulerRuntime::deferred_secondary(
            crate::scheduler::LogicalCpuId::new(3),
        ),
    ),
];

static PRODUCTION_TIMER_PREEMPTION_RECORD_MISSES: AtomicU64 = AtomicU64::new(0);

fn record_production_timer_preemption_irq(logical_cpu: Option<usize>) {
    let Some(logical_cpu) = logical_cpu.filter(|cpu| *cpu < MAX_CORES) else {
        PRODUCTION_TIMER_PREEMPTION_RECORD_MISSES.fetch_add(1, Ordering::Relaxed);
        return;
    };

    let runtime = unsafe { &mut *PRODUCTION_SCHEDULER_RUNTIMES[logical_cpu].get() };
    let _ = runtime.record_timer_irq::<MAX_CORES>(Some(logical_cpu));
}

#[derive(Clone, Copy)]
struct SingleCoreIrqMaskProbe {
    nested_start_masked: bool,
    inner_restored_masked: bool,
    outer_restored_masked: bool,
    unmasked_start: bool,
    saved_unmasked_masked: bool,
    restored_unmasked: bool,
}

#[allow(dead_code)]
pub const fn qemu_logical_cpu_from_mpidr_affinity(affinity: u64) -> Option<usize> {
    if affinity < MAX_CORES as u64 {
        Some(affinity as usize)
    } else {
        None
    }
}

#[cfg(any(
    talos_boot_scenario = "qemu_secondary_core_workload",
    talos_boot_scenario = "qemu_smp_lock_contention",
    talos_boot_scenario = "qemu_per_core_scheduler_ownership",
    talos_boot_scenario = "qemu_cross_core_ipi_delivery",
    talos_boot_scenario = "qemu_remote_wakeup_request",
    talos_boot_scenario = "qemu_production_secondary_dispatch",
    talos_boot_scenario = "qemu_shared_scheduler_metadata",
    talos_boot_scenario = "qemu_secondary_scheduler_service_loop",
    talos_boot_scenario = "qemu_multicore_preemption_smoke",
    talos_boot_scenario = "qemu_production_timer_preemption_smoke"
))]
fn secondary_stack_layout() -> CoreStackLayout {
    let base = core::ptr::addr_of!(talos_secondary_core_stacks) as usize;
    let end = core::ptr::addr_of!(talos_secondary_core_stacks_end) as usize;
    CoreStackLayout::new(base, end, MAX_CORES, SECONDARY_KERNEL_STACK_SIZE)
        .expect("valid linked secondary-core stack layout")
}

#[cfg(any(
    talos_boot_scenario = "qemu_secondary_core_workload",
    talos_boot_scenario = "qemu_smp_lock_contention",
    talos_boot_scenario = "qemu_per_core_scheduler_ownership",
    talos_boot_scenario = "qemu_cross_core_ipi_delivery",
    talos_boot_scenario = "qemu_remote_wakeup_request",
    talos_boot_scenario = "qemu_production_secondary_dispatch",
    talos_boot_scenario = "qemu_shared_scheduler_metadata",
    talos_boot_scenario = "qemu_secondary_scheduler_service_loop",
    talos_boot_scenario = "qemu_multicore_preemption_smoke",
    talos_boot_scenario = "qemu_production_timer_preemption_smoke"
))]
fn secondary_state_name(state: u64) -> &'static str {
    CoreLifecycle::from_raw(state).map_or("unknown", CoreLifecycle::name)
}

#[cfg(any(
    talos_boot_scenario = "qemu_secondary_core_workload",
    talos_boot_scenario = "qemu_smp_lock_contention",
    talos_boot_scenario = "qemu_per_core_scheduler_ownership",
    talos_boot_scenario = "qemu_cross_core_ipi_delivery",
    talos_boot_scenario = "qemu_remote_wakeup_request",
    talos_boot_scenario = "qemu_production_secondary_dispatch",
    talos_boot_scenario = "qemu_shared_scheduler_metadata",
    talos_boot_scenario = "qemu_secondary_scheduler_service_loop",
    talos_boot_scenario = "qemu_multicore_preemption_smoke",
    talos_boot_scenario = "qemu_production_timer_preemption_smoke"
))]
unsafe fn psci_cpu_on_smc(target_affinity: u64, entry: usize, context: usize) -> i64 {
    let mut function_id = 0xc400_0003u64;
    unsafe {
        core::arch::asm!(
            "smc #0",
            inout("x0") function_id,
            in("x1") target_affinity,
            in("x2") entry as u64,
            in("x3") context as u64,
            options(nostack)
        );
    }
    function_id as i64
}

impl SingleCoreIrqMaskProbe {
    const fn passed(self) -> bool {
        self.nested_start_masked
            && self.inner_restored_masked
            && self.outer_restored_masked
            && self.unmasked_start
            && self.saved_unmasked_masked
            && self.restored_unmasked
    }
}

pub fn init() {
    console().init_early();
}

pub fn console() -> Pl011 {
    Pl011::new(PL011_BASE)
}

pub fn services(boot_info: &BootInfo) -> TargetServices {
    TargetServices {
        uart: UartKind::Pl011,
        timer: TimerKind::ArmGeneric,
        interrupt_controller: InterruptControllerKind::GicV2,
        mmio_map: MmioMap::new(MMIO_REGIONS),
        device_tree: DeviceTree::from_physical_address(boot_info.dtb_pa),
    }
}

#[cfg(any(
    talos_boot_scenario = "qemu_secondary_core_workload",
    talos_boot_scenario = "qemu_smp_lock_contention",
    talos_boot_scenario = "qemu_per_core_scheduler_ownership",
    talos_boot_scenario = "qemu_cross_core_ipi_delivery",
    talos_boot_scenario = "qemu_remote_wakeup_request",
    talos_boot_scenario = "qemu_production_secondary_dispatch",
    talos_boot_scenario = "qemu_shared_scheduler_metadata",
    talos_boot_scenario = "qemu_secondary_scheduler_service_loop",
    talos_boot_scenario = "qemu_multicore_preemption_smoke",
    talos_boot_scenario = "qemu_production_timer_preemption_smoke"
))]
#[unsafe(no_mangle)]
pub extern "C" fn talos_qemu_secondary_entry(context: usize) -> ! {
    let mpidr = aarch64::mpidr_el1();
    let affinity = aarch64::mpidr_affinity(mpidr);
    let logical_cpu = qemu_logical_cpu_from_mpidr_affinity(affinity).unwrap_or(context);
    if logical_cpu < MAX_CORES {
        let core_state = &SECONDARY_CORE_STATES[logical_cpu];
        core_state.enter(context, mpidr, affinity);

        let stack_pointer: u64;
        unsafe {
            core::arch::asm!("mov {stack_pointer}, sp", stack_pointer = out(reg) stack_pointer, options(nomem, nostack, preserves_flags));
        }
        core_state.mark_stack_ready(stack_pointer as usize);
        core_state.mark_registered();
        core_state.mark_handoff_ready();
        #[cfg(talos_boot_scenario = "qemu_secondary_core_workload")]
        smp::run_controlled_secondary_workload(core_state, SECONDARY_CORE_WORKLOAD_TARGET);
        #[cfg(talos_boot_scenario = "qemu_smp_lock_contention")]
        run_smp_lock_contention_secondary(core_state, logical_cpu);
        #[cfg(talos_boot_scenario = "qemu_per_core_scheduler_ownership")]
        run_per_core_scheduler_ownership_secondary(core_state, logical_cpu);
        #[cfg(talos_boot_scenario = "qemu_cross_core_ipi_delivery")]
        run_cross_core_ipi_delivery_secondary(core_state, logical_cpu);
        #[cfg(talos_boot_scenario = "qemu_remote_wakeup_request")]
        run_remote_wakeup_request_secondary(core_state, logical_cpu);
        #[cfg(talos_boot_scenario = "qemu_production_secondary_dispatch")]
        run_production_secondary_dispatch_secondary(core_state, logical_cpu);
        #[cfg(talos_boot_scenario = "qemu_shared_scheduler_metadata")]
        run_shared_scheduler_metadata_secondary(core_state, logical_cpu);
        #[cfg(talos_boot_scenario = "qemu_secondary_scheduler_service_loop")]
        run_secondary_scheduler_service_loop_secondary(core_state, logical_cpu);
        #[cfg(talos_boot_scenario = "qemu_multicore_preemption_smoke")]
        run_multicore_preemption_secondary(core_state, logical_cpu);
        #[cfg(talos_boot_scenario = "qemu_production_timer_preemption_smoke")]
        run_production_timer_preemption_secondary(core_state, logical_cpu);
    }

    loop {
        unsafe {
            core::arch::asm!("wfe", options(nomem, nostack, preserves_flags));
        }
    }
}

#[cfg(talos_boot_scenario = "qemu_per_core_scheduler_ownership")]
#[derive(Clone, Copy)]
struct PerCoreSchedulerReport {
    owner: u64,
    role: SchedulerCoreRole,
    production_dispatch_enabled: bool,
    current_task: u64,
    queue_len: u64,
    front_task: u64,
    progress: u64,
    state_transitions: u64,
    dispatch_deferred: bool,
    errors: u64,
}

#[cfg(talos_boot_scenario = "qemu_per_core_scheduler_ownership")]
impl PerCoreSchedulerReport {
    const fn empty() -> Self {
        Self {
            owner: u64::MAX,
            role: SchedulerCoreRole::SecondaryDeferred,
            production_dispatch_enabled: false,
            current_task: 0,
            queue_len: 0,
            front_task: 0,
            progress: 0,
            state_transitions: 0,
            dispatch_deferred: false,
            errors: 0,
        }
    }
}

#[cfg(talos_boot_scenario = "qemu_per_core_scheduler_ownership")]
#[derive(Clone, Copy)]
struct PerCoreSchedulerOwnershipState {
    reports: [PerCoreSchedulerReport; MAX_CORES],
    lock_progress: [u64; MAX_CORES],
}

#[cfg(talos_boot_scenario = "qemu_per_core_scheduler_ownership")]
impl PerCoreSchedulerOwnershipState {
    const fn new() -> Self {
        Self {
            reports: [PerCoreSchedulerReport::empty(); MAX_CORES],
            lock_progress: [0; MAX_CORES],
        }
    }
}

#[cfg(talos_boot_scenario = "qemu_per_core_scheduler_ownership")]
static PER_CORE_SCHEDULER_OWNERSHIP_STATE: SpinLock<PerCoreSchedulerOwnershipState> =
    SpinLock::new(PerCoreSchedulerOwnershipState::new());

#[cfg(talos_boot_scenario = "qemu_per_core_scheduler_ownership")]
fn reset_per_core_scheduler_ownership_state() {
    let mut state = unsafe { PER_CORE_SCHEDULER_OWNERSHIP_STATE.lock_irqsave() };
    *state = PerCoreSchedulerOwnershipState::new();
}

#[cfg(any(
    talos_boot_scenario = "qemu_per_core_scheduler_ownership",
    talos_boot_scenario = "qemu_production_secondary_dispatch",
    talos_boot_scenario = "qemu_shared_scheduler_metadata",
    talos_boot_scenario = "qemu_shared_runqueue_migration",
    talos_boot_scenario = "qemu_load_balancing_smoke",
    talos_boot_scenario = "qemu_secondary_scheduler_service_loop",
    talos_boot_scenario = "qemu_multicore_preemption_smoke",
    talos_boot_scenario = "qemu_production_timer_preemption_smoke"
))]
fn scheduler_role_name(role: SchedulerCoreRole) -> &'static str {
    match role {
        SchedulerCoreRole::BootCpuProduction => "boot-production",
        SchedulerCoreRole::SecondaryDeferred => "secondary-deferred",
        SchedulerCoreRole::SecondaryProductionDiagnostic => "secondary-production-diagnostic",
    }
}

#[cfg(any(
    talos_boot_scenario = "qemu_per_core_scheduler_ownership",
    talos_boot_scenario = "qemu_production_secondary_dispatch",
    talos_boot_scenario = "qemu_shared_scheduler_metadata",
    talos_boot_scenario = "qemu_shared_runqueue_migration",
    talos_boot_scenario = "qemu_load_balancing_smoke",
    talos_boot_scenario = "qemu_secondary_scheduler_service_loop",
    talos_boot_scenario = "qemu_multicore_preemption_smoke",
    talos_boot_scenario = "qemu_production_timer_preemption_smoke"
))]
fn task_id(raw: u64) -> TaskId {
    TaskId::new(raw).expect("diagnostic task IDs are nonzero")
}

#[cfg(any(
    talos_boot_scenario = "qemu_per_core_scheduler_ownership",
    talos_boot_scenario = "qemu_production_secondary_dispatch",
    talos_boot_scenario = "qemu_shared_scheduler_metadata",
    talos_boot_scenario = "qemu_shared_runqueue_migration",
    talos_boot_scenario = "qemu_load_balancing_smoke",
    talos_boot_scenario = "qemu_secondary_scheduler_service_loop",
    talos_boot_scenario = "qemu_multicore_preemption_smoke",
    talos_boot_scenario = "qemu_production_timer_preemption_smoke"
))]
fn scheduler_task(logical_cpu: usize, progress: u64) -> Task {
    let raw_task_id = (logical_cpu as u64 + 1) * 100 + progress;
    let stack_base = 0x8000_0000 + logical_cpu * 0x10000 + progress as usize * 0x1000;
    let stack = KernelStack::new(stack_base, 0x1000).expect("diagnostic stack bounds are valid");
    let context = ContextFrame::new(stack.limit() & !0xf, 0x4000_0000 + raw_task_id as usize);
    Task::kernel_thread(task_id(raw_task_id), stack, context)
}

#[cfg(talos_boot_scenario = "qemu_per_core_scheduler_ownership")]
fn build_per_core_scheduler_report(
    logical_cpu: usize,
    scheduler: &mut PerCoreScheduler<2>,
) -> PerCoreSchedulerReport {
    let requester = LogicalCpuId::new(logical_cpu);
    let mut errors = 0;
    let dispatch_deferred = match scheduler.production_scheduler_mut(requester) {
        Ok(_) => {
            errors += 1;
            false
        }
        Err(PerCoreSchedulerAccessError::ProductionDispatchDeferred { owner }) => {
            if owner != requester {
                errors += 1;
            }
            true
        }
        Err(_) => {
            errors += 1;
            false
        }
    };

    let mut progress = 0;
    if let Ok(local_scheduler) = scheduler.local_scheduler_mut(requester) {
        while progress < PER_CORE_SCHEDULER_PROGRESS_TARGET {
            progress += 1;
            let mut task = scheduler_task(logical_cpu, progress);
            if local_scheduler.make_runnable(&mut task).is_err() {
                errors += 1;
                break;
            }
            if local_scheduler.pick_next() != Some(task.id()) {
                errors += 1;
                break;
            }
        }
    } else {
        errors += 1;
    }

    let local_scheduler = scheduler.scheduler();
    PerCoreSchedulerReport {
        owner: scheduler.owner().raw() as u64,
        role: scheduler.role(),
        production_dispatch_enabled: scheduler.production_dispatch_enabled(),
        current_task: scheduler.current_task().map_or(0, TaskId::raw),
        queue_len: local_scheduler.runnable().len() as u64,
        front_task: local_scheduler.runnable().front().map_or(0, TaskId::raw),
        progress,
        state_transitions: local_scheduler.counters().state_transitions(),
        dispatch_deferred,
        errors,
    }
}

#[cfg(talos_boot_scenario = "qemu_per_core_scheduler_ownership")]
fn build_boot_scheduler_report() -> PerCoreSchedulerReport {
    let mut scheduler = PerCoreScheduler::<2>::boot_cpu();
    let requester = LogicalCpuId::BOOT;
    let mut errors = 0;
    if scheduler.set_current_task(requester, task_id(1)).is_err() {
        errors += 1;
    }

    let mut progress = 0;
    if let Ok(local_scheduler) = scheduler.production_scheduler_mut(requester) {
        while progress < PER_CORE_SCHEDULER_PROGRESS_TARGET {
            progress += 1;
            let mut task = scheduler_task(0, progress);
            if local_scheduler.make_runnable(&mut task).is_err() {
                errors += 1;
                break;
            }
            if local_scheduler.pick_next() != Some(task.id()) {
                errors += 1;
                break;
            }
        }
    } else {
        errors += 1;
    }

    let local_scheduler = scheduler.scheduler();
    PerCoreSchedulerReport {
        owner: scheduler.owner().raw() as u64,
        role: scheduler.role(),
        production_dispatch_enabled: scheduler.production_dispatch_enabled(),
        current_task: scheduler.current_task().map_or(0, TaskId::raw),
        queue_len: local_scheduler.runnable().len() as u64,
        front_task: local_scheduler.runnable().front().map_or(0, TaskId::raw),
        progress,
        state_transitions: local_scheduler.counters().state_transitions(),
        dispatch_deferred: false,
        errors,
    }
}

#[cfg(talos_boot_scenario = "qemu_per_core_scheduler_ownership")]
fn publish_per_core_scheduler_report(logical_cpu: usize, report: PerCoreSchedulerReport) {
    let mut state = PER_CORE_SCHEDULER_OWNERSHIP_STATE.lock();
    state.reports[logical_cpu] = report;
    state.lock_progress[logical_cpu] = report.progress;
}

#[cfg(talos_boot_scenario = "qemu_per_core_scheduler_ownership")]
fn run_per_core_scheduler_ownership_secondary(core_state: &smp::PerCoreState, logical_cpu: usize) {
    core_state.mark_workload_running();
    core_state.clean_to_poc();

    let mut scheduler = PerCoreScheduler::<2>::deferred_secondary(LogicalCpuId::new(logical_cpu));
    let report = build_per_core_scheduler_report(logical_cpu, &mut scheduler);
    publish_per_core_scheduler_report(logical_cpu, report);
    smp_full_barrier();

    core_state.mark_workload_complete(report.progress);
    core_state.clean_to_poc();
}

#[cfg(talos_boot_scenario = "qemu_production_secondary_dispatch")]
#[derive(Clone, Copy)]
struct ProductionSecondaryDispatchReport {
    owner: u64,
    role: SchedulerCoreRole,
    production_dispatch_enabled: bool,
    current_task: u64,
    queue_len: u64,
    front_task: u64,
    progress: u64,
    state_transitions: u64,
    production_dispatches: u64,
    context_switches: u64,
    cross_owner_rejected: bool,
    cross_owner_dispatch_rejected: bool,
    errors: u64,
}

#[cfg(talos_boot_scenario = "qemu_production_secondary_dispatch")]
impl ProductionSecondaryDispatchReport {
    const fn empty() -> Self {
        Self {
            owner: u64::MAX,
            role: SchedulerCoreRole::SecondaryDeferred,
            production_dispatch_enabled: false,
            current_task: 0,
            queue_len: 0,
            front_task: 0,
            progress: 0,
            state_transitions: 0,
            production_dispatches: 0,
            context_switches: 0,
            cross_owner_rejected: false,
            cross_owner_dispatch_rejected: false,
            errors: 0,
        }
    }
}

#[cfg(talos_boot_scenario = "qemu_production_secondary_dispatch")]
#[derive(Clone, Copy)]
struct ProductionSecondaryDispatchState {
    reports: [ProductionSecondaryDispatchReport; MAX_CORES],
    lock_progress: [u64; MAX_CORES],
}

#[cfg(talos_boot_scenario = "qemu_production_secondary_dispatch")]
impl ProductionSecondaryDispatchState {
    const fn new() -> Self {
        Self {
            reports: [ProductionSecondaryDispatchReport::empty(); MAX_CORES],
            lock_progress: [0; MAX_CORES],
        }
    }
}

#[cfg(talos_boot_scenario = "qemu_production_secondary_dispatch")]
static PRODUCTION_SECONDARY_DISPATCH_STATE: SpinLock<ProductionSecondaryDispatchState> =
    SpinLock::new(ProductionSecondaryDispatchState::new());

#[cfg(talos_boot_scenario = "qemu_production_secondary_dispatch")]
fn reset_production_secondary_dispatch_state() {
    let mut state = unsafe { PRODUCTION_SECONDARY_DISPATCH_STATE.lock_irqsave() };
    *state = ProductionSecondaryDispatchState::new();
}

#[cfg(talos_boot_scenario = "qemu_production_secondary_dispatch")]
fn build_production_secondary_dispatch_report(
    logical_cpu: usize,
    scheduler: &mut PerCoreScheduler<2>,
) -> ProductionSecondaryDispatchReport {
    let requester = LogicalCpuId::new(logical_cpu);
    let wrong_requester = LogicalCpuId::BOOT;
    let mut errors = 0;

    let cross_owner_rejected = match scheduler.local_scheduler_mut(wrong_requester) {
        Err(PerCoreSchedulerAccessError::WrongOwner {
            owner,
            requester: wrong,
        }) => owner == requester && wrong == wrong_requester,
        _ => false,
    };
    if !cross_owner_rejected {
        errors += 1;
    }

    let mut wrong_owner_task = scheduler_task(logical_cpu, 99);
    let cross_owner_dispatch_rejected = match scheduler
        .dispatch_cpu_local_diagnostic_task(wrong_requester, &mut wrong_owner_task)
    {
        Err(ProductionDispatchError::WrongOwner {
            owner,
            requester: wrong,
        }) => owner == requester && wrong == wrong_requester,
        _ => false,
    };
    if !cross_owner_dispatch_rejected {
        errors += 1;
    }

    let mut progress = 0;
    while progress < PRODUCTION_SECONDARY_DISPATCH_PROGRESS_TARGET {
        let next_progress = progress + 1;
        let mut task = scheduler_task(logical_cpu, next_progress);
        match scheduler.local_scheduler_mut(requester) {
            Ok(local_scheduler) => {
                if local_scheduler.make_runnable(&mut task).is_err() {
                    errors += 1;
                    break;
                }
            }
            Err(_) => {
                errors += 1;
                break;
            }
        }

        match scheduler.dispatch_cpu_local_diagnostic_task(requester, &mut task) {
            Ok(task_id) if task_id == task.id() && task.state() == TaskState::Running => {
                progress = next_progress;
            }
            _ => {
                errors += 1;
                break;
            }
        }
    }

    let local_scheduler = scheduler.scheduler();
    let counters = local_scheduler.counters();
    ProductionSecondaryDispatchReport {
        owner: scheduler.owner().raw() as u64,
        role: scheduler.role(),
        production_dispatch_enabled: scheduler.production_dispatch_enabled(),
        current_task: scheduler.current_task().map_or(0, TaskId::raw),
        queue_len: local_scheduler.runnable().len() as u64,
        front_task: local_scheduler.runnable().front().map_or(0, TaskId::raw),
        progress,
        state_transitions: counters.state_transitions(),
        production_dispatches: counters.production_dispatches(),
        context_switches: counters.context_switches(),
        cross_owner_rejected,
        cross_owner_dispatch_rejected,
        errors,
    }
}

#[cfg(talos_boot_scenario = "qemu_production_secondary_dispatch")]
fn publish_production_secondary_dispatch_report(
    logical_cpu: usize,
    report: ProductionSecondaryDispatchReport,
) {
    let mut state = PRODUCTION_SECONDARY_DISPATCH_STATE.lock();
    state.reports[logical_cpu] = report;
    state.lock_progress[logical_cpu] = report.progress;
}

#[cfg(talos_boot_scenario = "qemu_production_secondary_dispatch")]
fn run_production_secondary_dispatch_secondary(core_state: &smp::PerCoreState, logical_cpu: usize) {
    core_state.mark_workload_running();
    core_state.clean_to_poc();

    let mut scheduler =
        PerCoreScheduler::<2>::production_secondary_diagnostic(LogicalCpuId::new(logical_cpu));
    let report = build_production_secondary_dispatch_report(logical_cpu, &mut scheduler);
    publish_production_secondary_dispatch_report(logical_cpu, report);
    smp_full_barrier();

    core_state.mark_workload_complete(report.progress);
    core_state.clean_to_poc();
}

#[cfg(talos_boot_scenario = "qemu_shared_scheduler_metadata")]
#[derive(Clone, Copy)]
struct SharedSchedulerMetadataReport {
    owner: u64,
    role: SchedulerCoreRole,
    production_dispatch_enabled: bool,
    task_id: u64,
    task_state: u64,
    current_task: u64,
    queue_len: u64,
    front_task: u64,
    metadata_len: u64,
    metadata_generation: u64,
    lookup_owner: u64,
    lookup_task: u64,
    lookup_generation: u64,
    boot_lookup_owner: u64,
    boot_lookup_task: u64,
    boot_lookup_generation: u64,
    cross_owner_rejected: bool,
    metadata_cross_owner_rejected: bool,
    local_queue_preserved: bool,
    errors: u64,
}

#[cfg(talos_boot_scenario = "qemu_shared_scheduler_metadata")]
impl SharedSchedulerMetadataReport {
    const fn empty() -> Self {
        Self {
            owner: u64::MAX,
            role: SchedulerCoreRole::SecondaryDeferred,
            production_dispatch_enabled: false,
            task_id: 0,
            task_state: 0,
            current_task: 0,
            queue_len: 0,
            front_task: 0,
            metadata_len: 0,
            metadata_generation: 0,
            lookup_owner: u64::MAX,
            lookup_task: 0,
            lookup_generation: 0,
            boot_lookup_owner: u64::MAX,
            boot_lookup_task: 0,
            boot_lookup_generation: 0,
            cross_owner_rejected: false,
            metadata_cross_owner_rejected: false,
            local_queue_preserved: false,
            errors: 0,
        }
    }
}

#[cfg(talos_boot_scenario = "qemu_shared_scheduler_metadata")]
#[derive(Clone, Copy)]
struct SharedSchedulerMetadataSmokeState {
    reports: [SharedSchedulerMetadataReport; MAX_CORES],
    lock_progress: [u64; MAX_CORES],
}

#[cfg(talos_boot_scenario = "qemu_shared_scheduler_metadata")]
impl SharedSchedulerMetadataSmokeState {
    const fn new() -> Self {
        Self {
            reports: [SharedSchedulerMetadataReport::empty(); MAX_CORES],
            lock_progress: [0; MAX_CORES],
        }
    }
}

#[cfg(talos_boot_scenario = "qemu_shared_scheduler_metadata")]
static SHARED_SCHEDULER_METADATA_SMOKE_STATE: SpinLock<SharedSchedulerMetadataSmokeState> =
    SpinLock::new(SharedSchedulerMetadataSmokeState::new());

#[cfg(talos_boot_scenario = "qemu_shared_scheduler_metadata")]
static SHARED_SCHEDULER_METADATA_SMOKE_TABLE: SharedSchedulerMetadataLock<
    SHARED_SCHEDULER_METADATA_TASK_CAPACITY,
    MAX_CORES,
> = SpinLock::new(SharedSchedulerMetadata::new());

#[cfg(talos_boot_scenario = "qemu_shared_scheduler_metadata")]
fn reset_shared_scheduler_metadata_smoke_state() {
    let mut state = unsafe { SHARED_SCHEDULER_METADATA_SMOKE_STATE.lock_irqsave() };
    *state = SharedSchedulerMetadataSmokeState::new();
    let mut metadata = unsafe { SHARED_SCHEDULER_METADATA_SMOKE_TABLE.lock_irqsave() };
    *metadata = SharedSchedulerMetadata::new();
}

#[cfg(talos_boot_scenario = "qemu_shared_scheduler_metadata")]
fn build_shared_scheduler_metadata_report(
    logical_cpu: usize,
    scheduler: &mut PerCoreScheduler<2>,
) -> SharedSchedulerMetadataReport {
    let requester = LogicalCpuId::new(logical_cpu);
    let wrong_requester = if logical_cpu == 0 {
        LogicalCpuId::new(1)
    } else {
        LogicalCpuId::BOOT
    };
    let mut task = scheduler_task(logical_cpu, 1);
    let mut errors = 0;

    match scheduler.local_scheduler_mut(requester) {
        Ok(local_scheduler) => {
            if local_scheduler.make_runnable(&mut task).is_err() {
                errors += 1;
            }
        }
        Err(_) => errors += 1,
    }

    if scheduler.dispatch_cpu_local_diagnostic_task(requester, &mut task) != Ok(task.id()) {
        errors += 1;
    }

    let queue_len_before_cross_owner = scheduler.scheduler().runnable().len();
    let cross_owner_rejected = match scheduler.local_scheduler_mut(wrong_requester) {
        Err(PerCoreSchedulerAccessError::WrongOwner { owner, requester }) => {
            owner == scheduler.owner() && requester == wrong_requester
        }
        _ => false,
    };
    if !cross_owner_rejected {
        errors += 1;
    }
    let local_queue_preserved =
        scheduler.scheduler().runnable().len() == queue_len_before_cross_owner;
    if !local_queue_preserved {
        errors += 1;
    }

    let (
        metadata_cross_owner_rejected,
        metadata_len,
        metadata_generation,
        lookup_owner,
        lookup_task,
        lookup_generation,
        boot_lookup_owner,
        boot_lookup_task,
        boot_lookup_generation,
    ) = {
        let mut metadata = unsafe { SHARED_SCHEDULER_METADATA_SMOKE_TABLE.lock_irqsave() };
        if metadata
            .register_local_task(requester, scheduler, &task)
            .is_err()
        {
            errors += 1;
        }

        let own_lookup = metadata.lookup_task(task.id());
        let boot_lookup = metadata.lookup_task(task_id(101));
        let metadata_cross_owner_rejected =
            match metadata.register_local_task(wrong_requester, scheduler, &task) {
                Err(SharedSchedulerMetadataError::WrongOwner { owner, requester }) => {
                    owner == scheduler.owner() && requester == wrong_requester
                }
                _ => false,
            };
        if !metadata_cross_owner_rejected {
            errors += 1;
        }

        let (lookup_owner, lookup_task, lookup_generation) = match own_lookup {
            Ok(snapshot) => (
                snapshot.owner().raw() as u64,
                snapshot.task_id().raw(),
                snapshot.generation(),
            ),
            Err(_) => {
                errors += 1;
                (u64::MAX, 0, 0)
            }
        };
        let (boot_lookup_owner, boot_lookup_task, boot_lookup_generation) = match boot_lookup {
            Ok(snapshot) => (
                snapshot.owner().raw() as u64,
                snapshot.task_id().raw(),
                snapshot.generation(),
            ),
            Err(_) => {
                errors += 1;
                (u64::MAX, 0, 0)
            }
        };

        (
            metadata_cross_owner_rejected,
            metadata.len() as u64,
            metadata.generation(),
            lookup_owner,
            lookup_task,
            lookup_generation,
            boot_lookup_owner,
            boot_lookup_task,
            boot_lookup_generation,
        )
    };

    let local_scheduler = scheduler.scheduler();
    SharedSchedulerMetadataReport {
        owner: scheduler.owner().raw() as u64,
        role: scheduler.role(),
        production_dispatch_enabled: scheduler.production_dispatch_enabled(),
        task_id: task.id().raw(),
        task_state: task_state_code(task.state()),
        current_task: scheduler.current_task().map_or(0, TaskId::raw),
        queue_len: local_scheduler.runnable().len() as u64,
        front_task: local_scheduler.runnable().front().map_or(0, TaskId::raw),
        metadata_len,
        metadata_generation,
        lookup_owner,
        lookup_task,
        lookup_generation,
        boot_lookup_owner,
        boot_lookup_task,
        boot_lookup_generation,
        cross_owner_rejected,
        metadata_cross_owner_rejected,
        local_queue_preserved,
        errors,
    }
}

#[cfg(talos_boot_scenario = "qemu_shared_scheduler_metadata")]
fn publish_shared_scheduler_metadata_report(
    logical_cpu: usize,
    report: SharedSchedulerMetadataReport,
) {
    let mut state = SHARED_SCHEDULER_METADATA_SMOKE_STATE.lock();
    state.reports[logical_cpu] = report;
    state.lock_progress[logical_cpu] = u64::from(report.errors == 0);
}

#[cfg(talos_boot_scenario = "qemu_shared_scheduler_metadata")]
fn run_shared_scheduler_metadata_secondary(core_state: &smp::PerCoreState, logical_cpu: usize) {
    core_state.mark_workload_running();
    core_state.clean_to_poc();

    let mut scheduler =
        PerCoreScheduler::<2>::production_secondary_diagnostic(LogicalCpuId::new(logical_cpu));
    let report = build_shared_scheduler_metadata_report(logical_cpu, &mut scheduler);
    publish_shared_scheduler_metadata_report(logical_cpu, report);
    smp_full_barrier();

    core_state.mark_workload_complete(report.lock_progress());
    core_state.clean_to_poc();
}

#[cfg(talos_boot_scenario = "qemu_secondary_scheduler_service_loop")]
#[derive(Clone, Copy)]
struct SecondarySchedulerServiceLoopReportLine {
    owner: u64,
    role: SchedulerCoreRole,
    task_id: u64,
    task_state: u64,
    current_task: u64,
    queue_len: u64,
    front_task: u64,
    remote_wake_task: u64,
    dispatch_task: u64,
    no_work_did_work: bool,
    metadata_generation: u64,
    metadata_len: u64,
    observed_remote_wake: bool,
    pending_timer_preemption: bool,
    dispatch_requested: bool,
    cross_owner_rejected: bool,
    deferred_role_rejected: bool,
    local_queue_preserved: bool,
    errors: u64,
}

#[cfg(talos_boot_scenario = "qemu_secondary_scheduler_service_loop")]
impl SecondarySchedulerServiceLoopReportLine {
    const fn empty() -> Self {
        Self {
            owner: u64::MAX,
            role: SchedulerCoreRole::SecondaryDeferred,
            task_id: 0,
            task_state: 0,
            current_task: 0,
            queue_len: 0,
            front_task: 0,
            remote_wake_task: 0,
            dispatch_task: 0,
            no_work_did_work: true,
            metadata_generation: 0,
            metadata_len: 0,
            observed_remote_wake: false,
            pending_timer_preemption: true,
            dispatch_requested: false,
            cross_owner_rejected: false,
            deferred_role_rejected: false,
            local_queue_preserved: false,
            errors: 0,
        }
    }

    const fn progress(self) -> u64 {
        if self.errors == 0 && self.remote_wake_task != 0 && self.dispatch_task != 0 {
            1
        } else {
            0
        }
    }
}

#[cfg(talos_boot_scenario = "qemu_secondary_scheduler_service_loop")]
struct SecondarySchedulerServiceLoopState {
    owner: [AtomicU64; MAX_CORES],
    role: [AtomicU64; MAX_CORES],
    task_id: [AtomicU64; MAX_CORES],
    task_state: [AtomicU64; MAX_CORES],
    current_task: [AtomicU64; MAX_CORES],
    queue_len: [AtomicU64; MAX_CORES],
    front_task: [AtomicU64; MAX_CORES],
    remote_wake_task: [AtomicU64; MAX_CORES],
    dispatch_task: [AtomicU64; MAX_CORES],
    no_work_did_work: [AtomicU64; MAX_CORES],
    metadata_generation: [AtomicU64; MAX_CORES],
    metadata_len: [AtomicU64; MAX_CORES],
    observed_remote_wake: [AtomicU64; MAX_CORES],
    pending_timer_preemption: [AtomicU64; MAX_CORES],
    dispatch_requested: [AtomicU64; MAX_CORES],
    cross_owner_rejected: [AtomicU64; MAX_CORES],
    deferred_role_rejected: [AtomicU64; MAX_CORES],
    local_queue_preserved: [AtomicU64; MAX_CORES],
    errors: [AtomicU64; MAX_CORES],
    progress: [AtomicU64; MAX_CORES],
}

#[cfg(talos_boot_scenario = "qemu_secondary_scheduler_service_loop")]
impl SecondarySchedulerServiceLoopState {
    const fn new() -> Self {
        Self {
            owner: [const { AtomicU64::new(u64::MAX) }; MAX_CORES],
            role: [const { AtomicU64::new(2) }; MAX_CORES],
            task_id: [const { AtomicU64::new(0) }; MAX_CORES],
            task_state: [const { AtomicU64::new(0) }; MAX_CORES],
            current_task: [const { AtomicU64::new(0) }; MAX_CORES],
            queue_len: [const { AtomicU64::new(0) }; MAX_CORES],
            front_task: [const { AtomicU64::new(0) }; MAX_CORES],
            remote_wake_task: [const { AtomicU64::new(0) }; MAX_CORES],
            dispatch_task: [const { AtomicU64::new(0) }; MAX_CORES],
            no_work_did_work: [const { AtomicU64::new(1) }; MAX_CORES],
            metadata_generation: [const { AtomicU64::new(0) }; MAX_CORES],
            metadata_len: [const { AtomicU64::new(0) }; MAX_CORES],
            observed_remote_wake: [const { AtomicU64::new(0) }; MAX_CORES],
            pending_timer_preemption: [const { AtomicU64::new(1) }; MAX_CORES],
            dispatch_requested: [const { AtomicU64::new(0) }; MAX_CORES],
            cross_owner_rejected: [const { AtomicU64::new(0) }; MAX_CORES],
            deferred_role_rejected: [const { AtomicU64::new(0) }; MAX_CORES],
            local_queue_preserved: [const { AtomicU64::new(0) }; MAX_CORES],
            errors: [const { AtomicU64::new(0) }; MAX_CORES],
            progress: [const { AtomicU64::new(0) }; MAX_CORES],
        }
    }

    fn reset(&self) {
        for logical_cpu in 0..MAX_CORES {
            self.owner[logical_cpu].store(u64::MAX, Ordering::Release);
            self.role[logical_cpu].store(2, Ordering::Release);
            self.task_id[logical_cpu].store(0, Ordering::Release);
            self.task_state[logical_cpu].store(0, Ordering::Release);
            self.current_task[logical_cpu].store(0, Ordering::Release);
            self.queue_len[logical_cpu].store(0, Ordering::Release);
            self.front_task[logical_cpu].store(0, Ordering::Release);
            self.remote_wake_task[logical_cpu].store(0, Ordering::Release);
            self.dispatch_task[logical_cpu].store(0, Ordering::Release);
            self.no_work_did_work[logical_cpu].store(1, Ordering::Release);
            self.metadata_generation[logical_cpu].store(0, Ordering::Release);
            self.metadata_len[logical_cpu].store(0, Ordering::Release);
            self.observed_remote_wake[logical_cpu].store(0, Ordering::Release);
            self.pending_timer_preemption[logical_cpu].store(1, Ordering::Release);
            self.dispatch_requested[logical_cpu].store(0, Ordering::Release);
            self.cross_owner_rejected[logical_cpu].store(0, Ordering::Release);
            self.deferred_role_rejected[logical_cpu].store(0, Ordering::Release);
            self.local_queue_preserved[logical_cpu].store(0, Ordering::Release);
            self.errors[logical_cpu].store(0, Ordering::Release);
            self.progress[logical_cpu].store(0, Ordering::Release);
        }
    }
}

#[cfg(talos_boot_scenario = "qemu_secondary_scheduler_service_loop")]
static SECONDARY_SCHEDULER_SERVICE_LOOP_STATE: SecondarySchedulerServiceLoopState =
    SecondarySchedulerServiceLoopState::new();

#[cfg(talos_boot_scenario = "qemu_secondary_scheduler_service_loop")]
fn reset_secondary_scheduler_service_loop_state() {
    SECONDARY_SCHEDULER_SERVICE_LOOP_STATE.reset();
}

#[cfg(talos_boot_scenario = "qemu_secondary_scheduler_service_loop")]
fn build_secondary_scheduler_service_loop_report(
    logical_cpu: usize,
) -> SecondarySchedulerServiceLoopReportLine {
    let owner = LogicalCpuId::new(logical_cpu);
    let mut scheduler = PerCoreScheduler::<2>::production_secondary_diagnostic(owner);
    let mut remote_wakes = RemoteWakeQueue::<2>::new(owner);
    let mut metadata =
        SharedSchedulerMetadata::<SECONDARY_SCHEDULER_SERVICE_LOOP_TASK_CAPACITY, MAX_CORES>::new();
    let mut task = scheduler_task(logical_cpu, 1);
    task.set_state(TaskState::Blocked);

    let mut errors = 0;
    let mut metadata_generation = 0;

    if metadata
        .register_local_task(owner, &scheduler, &task)
        .is_err()
    {
        errors += 1;
    }

    if remote_wakes
        .publish(LogicalCpuId::BOOT, owner, task.id())
        .is_err()
    {
        errors += 1;
    }

    let cross_owner_rejected = matches!(
        SecondarySchedulerServiceLoop::run_once(
            LogicalCpuId::BOOT,
            &mut scheduler,
            &mut remote_wakes,
            &mut metadata,
            &mut task,
            None,
            false,
            false,
        ),
        Err(SecondarySchedulerServiceLoopError::WrongOwner { .. })
    );
    if !cross_owner_rejected {
        errors += 1;
    }

    let mut deferred = PerCoreScheduler::<2>::deferred_secondary(owner);
    let deferred_role_rejected = matches!(
        SecondarySchedulerServiceLoop::run_once(
            owner,
            &mut deferred,
            &mut remote_wakes,
            &mut metadata,
            &mut task,
            None,
            false,
            false,
        ),
        Err(SecondarySchedulerServiceLoopError::ProductionDispatchDeferred { .. })
    );
    if !deferred_role_rejected {
        errors += 1;
    }

    let first_cycle = SecondarySchedulerServiceLoop::run_once(
        owner,
        &mut scheduler,
        &mut remote_wakes,
        &mut metadata,
        &mut task,
        None,
        false,
        true,
    );

    let (
        remote_wake_task,
        dispatch_task,
        observed_remote_wake,
        pending_timer_preemption,
        dispatch_requested,
    ) = match first_cycle {
        Ok(report) => (
            report.cycle().remote_wake().map_or(0, TaskId::raw),
            report.cycle().dispatch().map_or(0, TaskId::raw),
            report.observed_remote_wake(),
            report.pending_timer_preemption(),
            report.dispatch_requested(),
        ),
        Err(_) => {
            errors += 1;
            (0, 0, false, true, false)
        }
    };

    let no_work_did_work = match SecondarySchedulerServiceLoop::run_once(
        owner,
        &mut scheduler,
        &mut remote_wakes,
        &mut metadata,
        &mut task,
        None,
        false,
        false,
    ) {
        Ok(report) => {
            metadata_generation = report.cycle().metadata().generation();
            report.did_work()
        }
        Err(_) => {
            errors += 1;
            true
        }
    };

    let metadata_len = metadata.len() as u64;

    let queue_len = scheduler.scheduler().runnable().len() as u64;
    let front_task = scheduler
        .scheduler()
        .runnable()
        .front()
        .map_or(0, TaskId::raw);
    let current_task = scheduler.current_task().map_or(0, TaskId::raw);
    let local_queue_preserved = queue_len == 0
        && front_task == 0
        && current_task == task.id().raw()
        && remote_wakes.is_empty()
        && task.state() == TaskState::Running;

    if !local_queue_preserved {
        errors += 1;
    }

    SecondarySchedulerServiceLoopReportLine {
        owner: scheduler.owner().raw() as u64,
        role: scheduler.role(),
        task_id: task.id().raw(),
        task_state: task_state_code(task.state()),
        current_task,
        queue_len,
        front_task,
        remote_wake_task,
        dispatch_task,
        no_work_did_work,
        metadata_generation,
        metadata_len,
        observed_remote_wake,
        pending_timer_preemption,
        dispatch_requested,
        cross_owner_rejected,
        deferred_role_rejected,
        local_queue_preserved,
        errors,
    }
}

#[cfg(talos_boot_scenario = "qemu_secondary_scheduler_service_loop")]
fn publish_secondary_scheduler_service_loop_report(
    logical_cpu: usize,
    report: SecondarySchedulerServiceLoopReportLine,
) {
    let state = &SECONDARY_SCHEDULER_SERVICE_LOOP_STATE;
    state.owner[logical_cpu].store(report.owner, Ordering::Release);
    state.role[logical_cpu].store(scheduler_role_code(report.role), Ordering::Release);
    state.task_id[logical_cpu].store(report.task_id, Ordering::Release);
    state.task_state[logical_cpu].store(report.task_state, Ordering::Release);
    state.current_task[logical_cpu].store(report.current_task, Ordering::Release);
    state.queue_len[logical_cpu].store(report.queue_len, Ordering::Release);
    state.front_task[logical_cpu].store(report.front_task, Ordering::Release);
    state.remote_wake_task[logical_cpu].store(report.remote_wake_task, Ordering::Release);
    state.dispatch_task[logical_cpu].store(report.dispatch_task, Ordering::Release);
    state.no_work_did_work[logical_cpu]
        .store(u64::from(report.no_work_did_work), Ordering::Release);
    state.metadata_generation[logical_cpu].store(report.metadata_generation, Ordering::Release);
    state.metadata_len[logical_cpu].store(report.metadata_len, Ordering::Release);
    state.observed_remote_wake[logical_cpu]
        .store(u64::from(report.observed_remote_wake), Ordering::Release);
    state.pending_timer_preemption[logical_cpu].store(
        u64::from(report.pending_timer_preemption),
        Ordering::Release,
    );
    state.dispatch_requested[logical_cpu]
        .store(u64::from(report.dispatch_requested), Ordering::Release);
    state.cross_owner_rejected[logical_cpu]
        .store(u64::from(report.cross_owner_rejected), Ordering::Release);
    state.deferred_role_rejected[logical_cpu]
        .store(u64::from(report.deferred_role_rejected), Ordering::Release);
    state.local_queue_preserved[logical_cpu]
        .store(u64::from(report.local_queue_preserved), Ordering::Release);
    state.errors[logical_cpu].store(report.errors, Ordering::Release);
    state.progress[logical_cpu].store(report.progress(), Ordering::Release);
}

#[cfg(talos_boot_scenario = "qemu_secondary_scheduler_service_loop")]
fn scheduler_role_code(role: SchedulerCoreRole) -> u64 {
    match role {
        SchedulerCoreRole::BootCpuProduction => 1,
        SchedulerCoreRole::SecondaryDeferred => 2,
        SchedulerCoreRole::SecondaryProductionDiagnostic => 3,
    }
}

#[cfg(talos_boot_scenario = "qemu_secondary_scheduler_service_loop")]
fn scheduler_role_from_code(code: u64) -> SchedulerCoreRole {
    match code {
        1 => SchedulerCoreRole::BootCpuProduction,
        3 => SchedulerCoreRole::SecondaryProductionDiagnostic,
        _ => SchedulerCoreRole::SecondaryDeferred,
    }
}

#[cfg(talos_boot_scenario = "qemu_secondary_scheduler_service_loop")]
fn load_secondary_scheduler_service_loop_report(
    logical_cpu: usize,
) -> SecondarySchedulerServiceLoopReportLine {
    let state = &SECONDARY_SCHEDULER_SERVICE_LOOP_STATE;
    SecondarySchedulerServiceLoopReportLine {
        owner: state.owner[logical_cpu].load(Ordering::Acquire),
        role: scheduler_role_from_code(state.role[logical_cpu].load(Ordering::Acquire)),
        task_id: state.task_id[logical_cpu].load(Ordering::Acquire),
        task_state: state.task_state[logical_cpu].load(Ordering::Acquire),
        current_task: state.current_task[logical_cpu].load(Ordering::Acquire),
        queue_len: state.queue_len[logical_cpu].load(Ordering::Acquire),
        front_task: state.front_task[logical_cpu].load(Ordering::Acquire),
        remote_wake_task: state.remote_wake_task[logical_cpu].load(Ordering::Acquire),
        dispatch_task: state.dispatch_task[logical_cpu].load(Ordering::Acquire),
        no_work_did_work: state.no_work_did_work[logical_cpu].load(Ordering::Acquire) != 0,
        metadata_generation: state.metadata_generation[logical_cpu].load(Ordering::Acquire),
        metadata_len: state.metadata_len[logical_cpu].load(Ordering::Acquire),
        observed_remote_wake: state.observed_remote_wake[logical_cpu].load(Ordering::Acquire) != 0,
        pending_timer_preemption: state.pending_timer_preemption[logical_cpu]
            .load(Ordering::Acquire)
            != 0,
        dispatch_requested: state.dispatch_requested[logical_cpu].load(Ordering::Acquire) != 0,
        cross_owner_rejected: state.cross_owner_rejected[logical_cpu].load(Ordering::Acquire) != 0,
        deferred_role_rejected: state.deferred_role_rejected[logical_cpu].load(Ordering::Acquire)
            != 0,
        local_queue_preserved: state.local_queue_preserved[logical_cpu].load(Ordering::Acquire)
            != 0,
        errors: state.errors[logical_cpu].load(Ordering::Acquire),
    }
}

#[cfg(talos_boot_scenario = "qemu_secondary_scheduler_service_loop")]
fn run_secondary_scheduler_service_loop_secondary(
    core_state: &smp::PerCoreState,
    logical_cpu: usize,
) {
    core_state.mark_workload_running();
    core_state.clean_to_poc();

    let report = build_secondary_scheduler_service_loop_report(logical_cpu);
    publish_secondary_scheduler_service_loop_report(logical_cpu, report);
    smp_full_barrier();

    core_state.mark_workload_complete(report.progress());
    core_state.clean_to_poc();
}

#[cfg(talos_boot_scenario = "qemu_multicore_preemption_smoke")]
#[derive(Clone, Copy)]
struct MultiCorePreemptionSmokeReport {
    owner: u64,
    role: SchedulerCoreRole,
    current_before_record: u64,
    next_task: u64,
    queue_len_before_record: u64,
    metadata_generation_before_record: u64,
    record_inserted: bool,
    duplicate_coalesced: bool,
    cross_owner_rejected: bool,
    current_after_record: u64,
    queue_len_after_record: u64,
    metadata_generation_after_record: u64,
    scheduler_mutated_during_record: bool,
    pending_after_record: bool,
    service_timer_preemption: u64,
    current_after_service: u64,
    queue_len_after_service: u64,
    front_after_service: u64,
    previous_task_state: u64,
    selected_task_state: u64,
    pending_after_service: bool,
    recorded_requests: u64,
    coalesced_requests: u64,
    serviced_requests: u64,
    metadata_owner_after_service: u64,
    metadata_task_after_service: u64,
    metadata_generation_after_service: u64,
    errors: u64,
}

#[cfg(talos_boot_scenario = "qemu_multicore_preemption_smoke")]
impl MultiCorePreemptionSmokeReport {
    const fn empty() -> Self {
        Self {
            owner: u64::MAX,
            role: SchedulerCoreRole::SecondaryDeferred,
            current_before_record: 0,
            next_task: 0,
            queue_len_before_record: 0,
            metadata_generation_before_record: 0,
            record_inserted: false,
            duplicate_coalesced: false,
            cross_owner_rejected: false,
            current_after_record: 0,
            queue_len_after_record: 0,
            metadata_generation_after_record: 0,
            scheduler_mutated_during_record: true,
            pending_after_record: false,
            service_timer_preemption: 0,
            current_after_service: 0,
            queue_len_after_service: 0,
            front_after_service: 0,
            previous_task_state: 0,
            selected_task_state: 0,
            pending_after_service: true,
            recorded_requests: 0,
            coalesced_requests: 0,
            serviced_requests: 0,
            metadata_owner_after_service: u64::MAX,
            metadata_task_after_service: 0,
            metadata_generation_after_service: 0,
            errors: 0,
        }
    }

    const fn progress(self) -> u64 {
        if self.errors == 0
            && self.record_inserted
            && self.duplicate_coalesced
            && self.cross_owner_rejected
            && !self.scheduler_mutated_during_record
            && self.pending_after_record
            && !self.pending_after_service
            && self.serviced_requests == 1
        {
            1
        } else {
            0
        }
    }
}

#[cfg(talos_boot_scenario = "qemu_multicore_preemption_smoke")]
#[derive(Clone, Copy)]
struct MultiCorePreemptionSmokeState {
    reports: [MultiCorePreemptionSmokeReport; MAX_CORES],
    lock_progress: [u64; MAX_CORES],
}

#[cfg(talos_boot_scenario = "qemu_multicore_preemption_smoke")]
impl MultiCorePreemptionSmokeState {
    const fn new() -> Self {
        Self {
            reports: [MultiCorePreemptionSmokeReport::empty(); MAX_CORES],
            lock_progress: [0; MAX_CORES],
        }
    }
}

#[cfg(talos_boot_scenario = "qemu_multicore_preemption_smoke")]
static MULTICORE_PREEMPTION_SMOKE_STATE: SpinLock<MultiCorePreemptionSmokeState> =
    SpinLock::new(MultiCorePreemptionSmokeState::new());

#[cfg(talos_boot_scenario = "qemu_multicore_preemption_smoke")]
fn reset_multicore_preemption_smoke_state() {
    let mut state = unsafe { MULTICORE_PREEMPTION_SMOKE_STATE.lock_irqsave() };
    *state = MultiCorePreemptionSmokeState::new();
}

#[cfg(talos_boot_scenario = "qemu_multicore_preemption_smoke")]
fn build_multicore_preemption_smoke_report(logical_cpu: usize) -> MultiCorePreemptionSmokeReport {
    let owner = LogicalCpuId::new(logical_cpu);
    let mut scheduler = PerCoreScheduler::<2>::production_secondary_diagnostic(owner);
    let mut preemption = PerCorePreemptionState::new(owner);
    let mut remote_wakes = RemoteWakeQueue::<1>::new(owner);
    let mut metadata =
        SharedSchedulerMetadata::<MULTICORE_PREEMPTION_SMOKE_TASK_CAPACITY, MAX_CORES>::new();
    let mut current = scheduler_task(logical_cpu, 1);
    let mut next = scheduler_task(logical_cpu, 2);
    current.set_state(TaskState::Running);

    let mut errors = 0;
    if scheduler.set_current_task(owner, current.id()).is_err() {
        errors += 1;
    }
    match scheduler.local_scheduler_mut(owner) {
        Ok(local_scheduler) => {
            if local_scheduler.make_runnable(&mut next).is_err() {
                errors += 1;
            }
        }
        Err(_) => errors += 1,
    }
    if metadata
        .register_local_task(owner, &scheduler, &current)
        .is_err()
    {
        errors += 1;
    }
    if metadata
        .register_local_task(owner, &scheduler, &next)
        .is_err()
    {
        errors += 1;
    }

    let current_before_record = scheduler.current_task().map_or(0, TaskId::raw);
    let queue_len_before_record = scheduler.scheduler().runnable().len() as u64;
    let metadata_generation_before_record = metadata.generation();
    let record_inserted =
        preemption.record_local_timer_irq(owner) == Ok(PreemptionRecordOutcome::Inserted);
    if !record_inserted {
        errors += 1;
    }
    let duplicate_coalesced =
        preemption.record_local_timer_irq(owner) == Ok(PreemptionRecordOutcome::Coalesced);
    if !duplicate_coalesced {
        errors += 1;
    }
    let cross_owner_rejected = preemption
        .record_local_timer_irq(LogicalCpuId::BOOT)
        .is_err();
    if !cross_owner_rejected {
        errors += 1;
    }

    let current_after_record = scheduler.current_task().map_or(0, TaskId::raw);
    let queue_len_after_record = scheduler.scheduler().runnable().len() as u64;
    let metadata_generation_after_record = metadata.generation();
    let scheduler_mutated_during_record = current_after_record != current_before_record
        || queue_len_after_record != queue_len_before_record
        || metadata_generation_after_record != metadata_generation_before_record
        || current.state() != TaskState::Running
        || next.state() != TaskState::Runnable;
    if scheduler_mutated_during_record {
        errors += 1;
    }
    let pending_after_record = preemption.pending_timer_request();
    if !pending_after_record {
        errors += 1;
    }

    let service_report = CpuLocalSchedulerService::run_preemption_cycle(
        owner,
        &mut scheduler,
        &mut preemption,
        &mut remote_wakes,
        &mut metadata,
        &mut next,
        Some(&mut current),
        false,
    );
    let service_timer_preemption = match service_report {
        Ok(report) => report.timer_preemption().map_or(0, TaskId::raw),
        Err(_) => {
            errors += 1;
            0
        }
    };

    let current_after_service = scheduler.current_task().map_or(0, TaskId::raw);
    let queue_len_after_service = scheduler.scheduler().runnable().len() as u64;
    let front_after_service = scheduler
        .scheduler()
        .runnable()
        .front()
        .map_or(0, TaskId::raw);
    let final_metadata = metadata.lookup_task(next.id());
    let (
        metadata_owner_after_service,
        metadata_task_after_service,
        metadata_generation_after_service,
    ) = match final_metadata {
        Ok(snapshot) => (
            snapshot.owner().raw() as u64,
            snapshot.task_id().raw(),
            snapshot.generation(),
        ),
        Err(_) => {
            errors += 1;
            (u64::MAX, 0, 0)
        }
    };
    let counters = preemption.counters();

    MultiCorePreemptionSmokeReport {
        owner: scheduler.owner().raw() as u64,
        role: scheduler.role(),
        current_before_record,
        next_task: next.id().raw(),
        queue_len_before_record,
        metadata_generation_before_record,
        record_inserted,
        duplicate_coalesced,
        cross_owner_rejected,
        current_after_record,
        queue_len_after_record,
        metadata_generation_after_record,
        scheduler_mutated_during_record,
        pending_after_record,
        service_timer_preemption,
        current_after_service,
        queue_len_after_service,
        front_after_service,
        previous_task_state: task_state_code(current.state()),
        selected_task_state: task_state_code(next.state()),
        pending_after_service: preemption.pending_timer_request(),
        recorded_requests: counters.recorded_requests(),
        coalesced_requests: counters.coalesced_requests(),
        serviced_requests: counters.serviced_requests(),
        metadata_owner_after_service,
        metadata_task_after_service,
        metadata_generation_after_service,
        errors,
    }
}

#[cfg(talos_boot_scenario = "qemu_multicore_preemption_smoke")]
fn publish_multicore_preemption_smoke_report(
    logical_cpu: usize,
    report: MultiCorePreemptionSmokeReport,
) {
    let mut state = MULTICORE_PREEMPTION_SMOKE_STATE.lock();
    state.reports[logical_cpu] = report;
    state.lock_progress[logical_cpu] = report.progress();
}

#[cfg(talos_boot_scenario = "qemu_multicore_preemption_smoke")]
fn load_multicore_preemption_smoke_report(logical_cpu: usize) -> MultiCorePreemptionSmokeReport {
    let state = MULTICORE_PREEMPTION_SMOKE_STATE.lock();
    state.reports[logical_cpu]
}

#[cfg(talos_boot_scenario = "qemu_multicore_preemption_smoke")]
fn run_multicore_preemption_secondary(core_state: &smp::PerCoreState, logical_cpu: usize) {
    core_state.mark_workload_running();
    core_state.clean_to_poc();

    let report = build_multicore_preemption_smoke_report(logical_cpu);
    publish_multicore_preemption_smoke_report(logical_cpu, report);
    smp_full_barrier();

    core_state.mark_workload_complete(report.progress());
    core_state.clean_to_poc();
}

#[cfg(talos_boot_scenario = "qemu_production_timer_preemption_smoke")]
#[derive(Clone, Copy)]
struct ProductionTimerPreemptionSmokeReport {
    owner: u64,
    role: SchedulerCoreRole,
    current_before_record: u64,
    next_task: u64,
    queue_len_before_record: u64,
    metadata_generation_before_record: u64,
    production_irq_record_inserted: bool,
    production_irq_duplicate_coalesced: bool,
    cross_owner_rejected: bool,
    record_misses: u64,
    timer_record_rejections: u64,
    current_after_record: u64,
    queue_len_after_record: u64,
    metadata_generation_after_record: u64,
    irq_record_scheduler_mutated: bool,
    pending_after_record: bool,
    service_timer_preemption: u64,
    current_after_service: u64,
    queue_len_after_service: u64,
    front_after_service: u64,
    previous_task_state: u64,
    selected_task_state: u64,
    pending_after_service: bool,
    recorded_requests: u64,
    coalesced_requests: u64,
    serviced_requests: u64,
    metadata_owner_after_service: u64,
    metadata_task_after_service: u64,
    metadata_generation_after_service: u64,
    errors: u64,
}

#[cfg(talos_boot_scenario = "qemu_production_timer_preemption_smoke")]
impl ProductionTimerPreemptionSmokeReport {
    const fn empty() -> Self {
        Self {
            owner: u64::MAX,
            role: SchedulerCoreRole::SecondaryDeferred,
            current_before_record: 0,
            next_task: 0,
            queue_len_before_record: 0,
            metadata_generation_before_record: 0,
            production_irq_record_inserted: false,
            production_irq_duplicate_coalesced: false,
            cross_owner_rejected: false,
            record_misses: 0,
            timer_record_rejections: 0,
            current_after_record: 0,
            queue_len_after_record: 0,
            metadata_generation_after_record: 0,
            irq_record_scheduler_mutated: false,
            pending_after_record: false,
            service_timer_preemption: 0,
            current_after_service: 0,
            queue_len_after_service: 0,
            front_after_service: 0,
            previous_task_state: 0,
            selected_task_state: 0,
            pending_after_service: false,
            recorded_requests: 0,
            coalesced_requests: 0,
            serviced_requests: 0,
            metadata_owner_after_service: 0,
            metadata_task_after_service: 0,
            metadata_generation_after_service: 0,
            errors: 0,
        }
    }

    const fn progress(self) -> u64 {
        if self.errors == 0
            && self.production_irq_record_inserted
            && self.production_irq_duplicate_coalesced
            && self.cross_owner_rejected
            && self.record_misses == 0
            && self.timer_record_rejections == 1
            && !self.irq_record_scheduler_mutated
            && self.pending_after_record
            && !self.pending_after_service
            && self.serviced_requests == 1
        {
            1
        } else {
            0
        }
    }
}

#[cfg(talos_boot_scenario = "qemu_production_timer_preemption_smoke")]
#[derive(Clone, Copy)]
struct ProductionTimerPreemptionSmokeState {
    reports: [ProductionTimerPreemptionSmokeReport; MAX_CORES],
    lock_progress: [u64; MAX_CORES],
}

#[cfg(talos_boot_scenario = "qemu_production_timer_preemption_smoke")]
impl ProductionTimerPreemptionSmokeState {
    const fn new() -> Self {
        Self {
            reports: [ProductionTimerPreemptionSmokeReport::empty(); MAX_CORES],
            lock_progress: [0; MAX_CORES],
        }
    }
}

#[cfg(talos_boot_scenario = "qemu_production_timer_preemption_smoke")]
static PRODUCTION_TIMER_PREEMPTION_SMOKE_STATE: SpinLock<ProductionTimerPreemptionSmokeState> =
    SpinLock::new(ProductionTimerPreemptionSmokeState::new());

#[cfg(talos_boot_scenario = "qemu_production_timer_preemption_smoke")]
fn reset_production_timer_preemption_smoke_state() {
    {
        let mut state = unsafe { PRODUCTION_TIMER_PREEMPTION_SMOKE_STATE.lock_irqsave() };
        *state = ProductionTimerPreemptionSmokeState::new();
    }
    for logical_cpu in 0..MAX_CORES {
        let owner = LogicalCpuId::new(logical_cpu);
        let runtime = unsafe { &mut *PRODUCTION_SCHEDULER_RUNTIMES[logical_cpu].get() };
        *runtime = if logical_cpu == 0 {
            crate::scheduler::ProductionSchedulerRuntime::boot_cpu()
        } else {
            crate::scheduler::ProductionSchedulerRuntime::production_secondary_diagnostic(owner)
        };
    }
    PRODUCTION_TIMER_PREEMPTION_RECORD_MISSES.store(0, Ordering::Release);
}

#[cfg(talos_boot_scenario = "qemu_production_timer_preemption_smoke")]
fn build_production_timer_preemption_smoke_report(
    logical_cpu: usize,
) -> ProductionTimerPreemptionSmokeReport {
    let owner = LogicalCpuId::new(logical_cpu);
    let mut metadata =
        SharedSchedulerMetadata::<PRODUCTION_TIMER_PREEMPTION_SMOKE_TASK_CAPACITY, MAX_CORES>::new(
        );
    let mut current = scheduler_task(logical_cpu, 1);
    let mut next = scheduler_task(logical_cpu, 2);
    current.set_state(TaskState::Running);

    let mut errors = 0;
    {
        let runtime = unsafe { &mut *PRODUCTION_SCHEDULER_RUNTIMES[logical_cpu].get() };
        match runtime.scheduler_mut(owner) {
            Ok(scheduler) => {
                if scheduler.set_current_task(owner, current.id()).is_err() {
                    errors += 1;
                }
                match scheduler.local_scheduler_mut(owner) {
                    Ok(local_scheduler) => {
                        if local_scheduler.make_runnable(&mut next).is_err() {
                            errors += 1;
                        }
                    }
                    Err(_) => errors += 1,
                }
            }
            Err(_) => errors += 1,
        }

        if metadata
            .register_local_task(owner, runtime.scheduler(), &current)
            .is_err()
        {
            errors += 1;
        }
        if metadata
            .register_local_task(owner, runtime.scheduler(), &next)
            .is_err()
        {
            errors += 1;
        }
    }

    let (
        role,
        current_before_record,
        queue_len_before_record,
        metadata_generation_before_record,
        rejections_before,
        misses_before,
    ) = {
        let runtime = unsafe { &mut *PRODUCTION_SCHEDULER_RUNTIMES[logical_cpu].get() };
        (
            runtime.role(),
            runtime.scheduler().current_task().map_or(0, TaskId::raw),
            runtime.scheduler().scheduler().runnable().len() as u64,
            metadata.generation(),
            runtime.timer_record_rejections(),
            PRODUCTION_TIMER_PREEMPTION_RECORD_MISSES.load(Ordering::Acquire),
        )
    };

    record_production_timer_preemption_irq(Some(logical_cpu));
    let production_irq_record_inserted = {
        let runtime = unsafe { &mut *PRODUCTION_SCHEDULER_RUNTIMES[logical_cpu].get() };
        let counters = runtime.preemption_state().counters();
        counters.recorded_requests() == 1
            && counters.coalesced_requests() == 0
            && runtime.preemption_state().pending_timer_request()
    };
    if !production_irq_record_inserted {
        errors += 1;
    }

    record_production_timer_preemption_irq(Some(logical_cpu));
    let production_irq_duplicate_coalesced = {
        let runtime = unsafe { &mut *PRODUCTION_SCHEDULER_RUNTIMES[logical_cpu].get() };
        let counters = runtime.preemption_state().counters();
        counters.recorded_requests() == 1
            && counters.coalesced_requests() == 1
            && runtime.preemption_state().pending_timer_request()
    };
    if !production_irq_duplicate_coalesced {
        errors += 1;
    }

    let cross_owner_rejected = {
        let runtime = unsafe { &mut *PRODUCTION_SCHEDULER_RUNTIMES[logical_cpu].get() };
        runtime.record_timer_irq::<MAX_CORES>(Some(0)).is_err()
    };
    if !cross_owner_rejected {
        errors += 1;
    }

    let (
        current_after_record,
        queue_len_after_record,
        metadata_generation_after_record,
        pending_after_record,
        record_misses,
        timer_record_rejections,
    ) = {
        let runtime = unsafe { &mut *PRODUCTION_SCHEDULER_RUNTIMES[logical_cpu].get() };
        (
            runtime.scheduler().current_task().map_or(0, TaskId::raw),
            runtime.scheduler().scheduler().runnable().len() as u64,
            metadata.generation(),
            runtime.preemption_state().pending_timer_request(),
            PRODUCTION_TIMER_PREEMPTION_RECORD_MISSES
                .load(Ordering::Acquire)
                .saturating_sub(misses_before),
            runtime
                .timer_record_rejections()
                .saturating_sub(rejections_before),
        )
    };
    let irq_record_scheduler_mutated = current_after_record != current_before_record
        || queue_len_after_record != queue_len_before_record
        || metadata_generation_after_record != metadata_generation_before_record
        || current.state() != TaskState::Running
        || next.state() != TaskState::Runnable;
    if irq_record_scheduler_mutated || !pending_after_record || record_misses != 0 {
        errors += 1;
    }

    let service_timer_preemption = {
        let runtime = unsafe { &mut *PRODUCTION_SCHEDULER_RUNTIMES[logical_cpu].get() };
        match runtime.service_pending_preemption(
            owner,
            &mut metadata,
            &mut next,
            Some(&mut current),
            false,
        ) {
            Ok(report) => report.timer_preemption().map_or(0, TaskId::raw),
            Err(_) => {
                errors += 1;
                0
            }
        }
    };

    let (
        current_after_service,
        queue_len_after_service,
        front_after_service,
        pending_after_service,
        recorded_requests,
        coalesced_requests,
        serviced_requests,
    ) = {
        let runtime = unsafe { &mut *PRODUCTION_SCHEDULER_RUNTIMES[logical_cpu].get() };
        let counters = runtime.preemption_state().counters();
        (
            runtime.scheduler().current_task().map_or(0, TaskId::raw),
            runtime.scheduler().scheduler().runnable().len() as u64,
            runtime
                .scheduler()
                .scheduler()
                .runnable()
                .front()
                .map_or(0, TaskId::raw),
            runtime.preemption_state().pending_timer_request(),
            counters.recorded_requests(),
            counters.coalesced_requests(),
            counters.serviced_requests(),
        )
    };
    let final_metadata = metadata.lookup_task(next.id());
    let (
        metadata_owner_after_service,
        metadata_task_after_service,
        metadata_generation_after_service,
    ) = match final_metadata {
        Ok(snapshot) => (
            snapshot.owner().raw() as u64,
            snapshot.task_id().raw(),
            snapshot.generation(),
        ),
        Err(_) => {
            errors += 1;
            (u64::MAX, 0, 0)
        }
    };

    ProductionTimerPreemptionSmokeReport {
        owner: owner.raw() as u64,
        role,
        current_before_record,
        next_task: next.id().raw(),
        queue_len_before_record,
        metadata_generation_before_record,
        production_irq_record_inserted,
        production_irq_duplicate_coalesced,
        cross_owner_rejected,
        record_misses,
        timer_record_rejections,
        current_after_record,
        queue_len_after_record,
        metadata_generation_after_record,
        irq_record_scheduler_mutated,
        pending_after_record,
        service_timer_preemption,
        current_after_service,
        queue_len_after_service,
        front_after_service,
        previous_task_state: task_state_code(current.state()),
        selected_task_state: task_state_code(next.state()),
        pending_after_service,
        recorded_requests,
        coalesced_requests,
        serviced_requests,
        metadata_owner_after_service,
        metadata_task_after_service,
        metadata_generation_after_service,
        errors,
    }
}

#[cfg(talos_boot_scenario = "qemu_production_timer_preemption_smoke")]
fn publish_production_timer_preemption_smoke_report(
    logical_cpu: usize,
    report: ProductionTimerPreemptionSmokeReport,
) {
    let mut state = PRODUCTION_TIMER_PREEMPTION_SMOKE_STATE.lock();
    state.reports[logical_cpu] = report;
    state.lock_progress[logical_cpu] = report.progress();
}

#[cfg(talos_boot_scenario = "qemu_production_timer_preemption_smoke")]
fn load_production_timer_preemption_smoke_report(
    logical_cpu: usize,
) -> ProductionTimerPreemptionSmokeReport {
    let state = PRODUCTION_TIMER_PREEMPTION_SMOKE_STATE.lock();
    state.reports[logical_cpu]
}

#[cfg(talos_boot_scenario = "qemu_production_timer_preemption_smoke")]
fn run_production_timer_preemption_secondary(core_state: &smp::PerCoreState, logical_cpu: usize) {
    core_state.mark_workload_running();
    core_state.clean_to_poc();

    let report = build_production_timer_preemption_smoke_report(logical_cpu);
    publish_production_timer_preemption_smoke_report(logical_cpu, report);
    smp_full_barrier();

    core_state.mark_workload_complete(report.progress());
    core_state.clean_to_poc();
}

#[cfg(talos_boot_scenario = "qemu_shared_scheduler_metadata")]
impl SharedSchedulerMetadataReport {
    const fn lock_progress(self) -> u64 {
        if self.errors == 0 { 1 } else { 0 }
    }
}

#[cfg(talos_boot_scenario = "qemu_cross_core_ipi_delivery")]
struct CrossCoreIpiDeliveryState {
    ready_mask: AtomicU64,
    complete_mask: AtomicU64,
    sent_values: [AtomicU64; MAX_CORES],
    target_bits: [AtomicU64; MAX_CORES],
    receive_counts: [AtomicU64; MAX_CORES],
    eoi_counts: [AtomicU64; MAX_CORES],
    last_vectors: [AtomicU64; MAX_CORES],
    last_iars: [AtomicU64; MAX_CORES],
    last_intids: [AtomicU64; MAX_CORES],
    errors: AtomicU64,
}

#[cfg(talos_boot_scenario = "qemu_cross_core_ipi_delivery")]
impl CrossCoreIpiDeliveryState {
    const fn new() -> Self {
        Self {
            ready_mask: AtomicU64::new(0),
            complete_mask: AtomicU64::new(0),
            sent_values: [const { AtomicU64::new(0) }; MAX_CORES],
            target_bits: [const { AtomicU64::new(0) }; MAX_CORES],
            receive_counts: [const { AtomicU64::new(0) }; MAX_CORES],
            eoi_counts: [const { AtomicU64::new(0) }; MAX_CORES],
            last_vectors: [const { AtomicU64::new(0) }; MAX_CORES],
            last_iars: [const { AtomicU64::new(0) }; MAX_CORES],
            last_intids: [const { AtomicU64::new(0) }; MAX_CORES],
            errors: AtomicU64::new(0),
        }
    }

    fn reset(&self) {
        self.ready_mask.store(0, Ordering::Release);
        self.complete_mask.store(0, Ordering::Release);
        self.errors.store(0, Ordering::Release);
        for logical_cpu in 0..MAX_CORES {
            self.sent_values[logical_cpu].store(0, Ordering::Release);
            self.target_bits[logical_cpu].store(0, Ordering::Release);
            self.receive_counts[logical_cpu].store(0, Ordering::Release);
            self.eoi_counts[logical_cpu].store(0, Ordering::Release);
            self.last_vectors[logical_cpu].store(0, Ordering::Release);
            self.last_iars[logical_cpu].store(0, Ordering::Release);
            self.last_intids[logical_cpu].store(0, Ordering::Release);
        }
    }

    fn mark_ready(&self, logical_cpu: usize) {
        self.ready_mask
            .fetch_or(1u64 << logical_cpu, Ordering::AcqRel);
    }

    fn mark_complete(&self, logical_cpu: usize) {
        self.complete_mask
            .fetch_or(1u64 << logical_cpu, Ordering::AcqRel);
    }

    fn record_send(&self, logical_cpu: usize, target_bit: u8, sgir_value: u32) {
        self.target_bits[logical_cpu].store(target_bit as u64, Ordering::Release);
        self.sent_values[logical_cpu].store(sgir_value as u64, Ordering::Release);
    }

    fn record_receive(&self, logical_cpu: Option<usize>, vector: u64, iar: u32, intid: u32) {
        if let Some(logical_cpu) = logical_cpu.filter(|cpu| *cpu < MAX_CORES) {
            self.last_vectors[logical_cpu].store(vector, Ordering::Release);
            self.last_iars[logical_cpu].store(iar as u64, Ordering::Release);
            self.last_intids[logical_cpu].store(intid as u64, Ordering::Release);
            self.receive_counts[logical_cpu].fetch_add(1, Ordering::AcqRel);
        } else {
            self.errors.fetch_add(1, Ordering::AcqRel);
        }
    }

    fn record_eoi(&self, logical_cpu: Option<usize>) {
        if let Some(logical_cpu) = logical_cpu.filter(|cpu| *cpu < MAX_CORES) {
            self.eoi_counts[logical_cpu].fetch_add(1, Ordering::AcqRel);
        } else {
            self.errors.fetch_add(1, Ordering::AcqRel);
        }
    }

    fn receive_count(&self, logical_cpu: usize) -> u64 {
        self.receive_counts[logical_cpu].load(Ordering::Acquire)
    }
}

#[cfg(talos_boot_scenario = "qemu_cross_core_ipi_delivery")]
static CROSS_CORE_IPI_DELIVERY_STATE: CrossCoreIpiDeliveryState = CrossCoreIpiDeliveryState::new();

fn current_qemu_logical_cpu() -> Option<usize> {
    qemu_logical_cpu_from_mpidr_affinity(aarch64::mpidr_affinity(aarch64::mpidr_el1()))
}

#[cfg(talos_boot_scenario = "qemu_cross_core_ipi_delivery")]
fn run_cross_core_ipi_delivery_secondary(core_state: &smp::PerCoreState, logical_cpu: usize) {
    core_state.mark_workload_running();
    core_state.clean_to_poc();

    crate::arch::aarch64::exceptions::init();
    unsafe {
        aarch64::disable_irq();
        aarch64::route_physical_irqs_to_el2();
        let gic = GicV2::new(GICD_BASE, GICC_BASE);
        gic.configure_sgi_priority(QEMU_CROSS_CORE_IPI_SGI_INTID, 0x80);
        gic.enable_cpu_interface();
        aarch64::enable_irq();
    }

    CROSS_CORE_IPI_DELIVERY_STATE.mark_ready(logical_cpu);

    let mut remaining = QEMU_SECONDARY_WAIT_LIMIT;
    while CROSS_CORE_IPI_DELIVERY_STATE.receive_count(logical_cpu) == 0 && remaining > 0 {
        unsafe {
            core::arch::asm!("wfe", options(nomem, nostack, preserves_flags));
        }
        remaining -= 1;
    }

    unsafe {
        aarch64::disable_irq();
    }
    CROSS_CORE_IPI_DELIVERY_STATE.mark_complete(logical_cpu);
    core_state.mark_workload_complete(CROSS_CORE_IPI_DELIVERY_STATE.receive_count(logical_cpu));
    core_state.clean_to_poc();
}

#[cfg(talos_boot_scenario = "qemu_remote_wakeup_request")]
struct RemoteWakeRequestSmokeState {
    ready_mask: AtomicU64,
    complete_mask: AtomicU64,
    sent_values: [AtomicU64; MAX_CORES],
    target_bits: [AtomicU64; MAX_CORES],
    receive_counts: [AtomicU64; MAX_CORES],
    eoi_counts: [AtomicU64; MAX_CORES],
    pending_counts: [AtomicU64; MAX_CORES],
    consumed_task_ids: [AtomicU64; MAX_CORES],
    duplicate_counts: [AtomicU64; MAX_CORES],
    queue_lens_after: [AtomicU64; MAX_CORES],
    cross_owner_rejections: [AtomicU64; MAX_CORES],
    production_deferrals: [AtomicU64; MAX_CORES],
    local_wake_task_ids: [AtomicU64; MAX_CORES],
    local_runnable_lens: [AtomicU64; MAX_CORES],
    local_state_before: [AtomicU64; MAX_CORES],
    local_state_after: [AtomicU64; MAX_CORES],
    duplicate_local_rejections: [AtomicU64; MAX_CORES],
    last_vectors: [AtomicU64; MAX_CORES],
    last_iars: [AtomicU64; MAX_CORES],
    last_intids: [AtomicU64; MAX_CORES],
    errors: AtomicU64,
}

#[cfg(talos_boot_scenario = "qemu_remote_wakeup_request")]
impl RemoteWakeRequestSmokeState {
    const fn new() -> Self {
        Self {
            ready_mask: AtomicU64::new(0),
            complete_mask: AtomicU64::new(0),
            sent_values: [const { AtomicU64::new(0) }; MAX_CORES],
            target_bits: [const { AtomicU64::new(0) }; MAX_CORES],
            receive_counts: [const { AtomicU64::new(0) }; MAX_CORES],
            eoi_counts: [const { AtomicU64::new(0) }; MAX_CORES],
            pending_counts: [const { AtomicU64::new(0) }; MAX_CORES],
            consumed_task_ids: [const { AtomicU64::new(0) }; MAX_CORES],
            duplicate_counts: [const { AtomicU64::new(0) }; MAX_CORES],
            queue_lens_after: [const { AtomicU64::new(0) }; MAX_CORES],
            cross_owner_rejections: [const { AtomicU64::new(0) }; MAX_CORES],
            production_deferrals: [const { AtomicU64::new(0) }; MAX_CORES],
            local_wake_task_ids: [const { AtomicU64::new(0) }; MAX_CORES],
            local_runnable_lens: [const { AtomicU64::new(0) }; MAX_CORES],
            local_state_before: [const { AtomicU64::new(0) }; MAX_CORES],
            local_state_after: [const { AtomicU64::new(0) }; MAX_CORES],
            duplicate_local_rejections: [const { AtomicU64::new(0) }; MAX_CORES],
            last_vectors: [const { AtomicU64::new(0) }; MAX_CORES],
            last_iars: [const { AtomicU64::new(0) }; MAX_CORES],
            last_intids: [const { AtomicU64::new(0) }; MAX_CORES],
            errors: AtomicU64::new(0),
        }
    }

    fn reset(&self) {
        self.ready_mask.store(0, Ordering::Release);
        self.complete_mask.store(0, Ordering::Release);
        self.errors.store(0, Ordering::Release);
        for logical_cpu in 0..MAX_CORES {
            self.sent_values[logical_cpu].store(0, Ordering::Release);
            self.target_bits[logical_cpu].store(0, Ordering::Release);
            self.receive_counts[logical_cpu].store(0, Ordering::Release);
            self.eoi_counts[logical_cpu].store(0, Ordering::Release);
            self.pending_counts[logical_cpu].store(0, Ordering::Release);
            self.consumed_task_ids[logical_cpu].store(0, Ordering::Release);
            self.duplicate_counts[logical_cpu].store(0, Ordering::Release);
            self.queue_lens_after[logical_cpu].store(0, Ordering::Release);
            self.cross_owner_rejections[logical_cpu].store(0, Ordering::Release);
            self.production_deferrals[logical_cpu].store(0, Ordering::Release);
            self.local_wake_task_ids[logical_cpu].store(0, Ordering::Release);
            self.local_runnable_lens[logical_cpu].store(0, Ordering::Release);
            self.local_state_before[logical_cpu].store(0, Ordering::Release);
            self.local_state_after[logical_cpu].store(0, Ordering::Release);
            self.duplicate_local_rejections[logical_cpu].store(0, Ordering::Release);
            self.last_vectors[logical_cpu].store(0, Ordering::Release);
            self.last_iars[logical_cpu].store(0, Ordering::Release);
            self.last_intids[logical_cpu].store(0, Ordering::Release);
        }
    }

    fn mark_ready(&self, logical_cpu: usize) {
        self.ready_mask
            .fetch_or(1u64 << logical_cpu, Ordering::AcqRel);
    }

    fn mark_complete(&self, logical_cpu: usize) {
        self.complete_mask
            .fetch_or(1u64 << logical_cpu, Ordering::AcqRel);
    }

    fn record_send(&self, logical_cpu: usize, target_bit: u8, sgir_value: u32) {
        self.target_bits[logical_cpu].store(target_bit as u64, Ordering::Release);
        self.sent_values[logical_cpu].store(sgir_value as u64, Ordering::Release);
    }

    fn record_receive(&self, logical_cpu: Option<usize>, vector: u64, iar: u32, intid: u32) {
        if let Some(logical_cpu) = logical_cpu.filter(|cpu| *cpu < MAX_CORES) {
            self.last_vectors[logical_cpu].store(vector, Ordering::Release);
            self.last_iars[logical_cpu].store(iar as u64, Ordering::Release);
            self.last_intids[logical_cpu].store(intid as u64, Ordering::Release);
            self.receive_counts[logical_cpu].fetch_add(1, Ordering::AcqRel);
            self.pending_counts[logical_cpu].fetch_add(1, Ordering::AcqRel);
        } else {
            self.errors.fetch_add(1, Ordering::AcqRel);
        }
    }

    fn record_eoi(&self, logical_cpu: Option<usize>) {
        if let Some(logical_cpu) = logical_cpu.filter(|cpu| *cpu < MAX_CORES) {
            self.eoi_counts[logical_cpu].fetch_add(1, Ordering::AcqRel);
        } else {
            self.errors.fetch_add(1, Ordering::AcqRel);
        }
    }

    fn receive_count(&self, logical_cpu: usize) -> u64 {
        self.receive_counts[logical_cpu].load(Ordering::Acquire)
    }
}

#[cfg(talos_boot_scenario = "qemu_remote_wakeup_request")]
static REMOTE_WAKE_REQUEST_SMOKE_STATE: RemoteWakeRequestSmokeState =
    RemoteWakeRequestSmokeState::new();

#[cfg(talos_boot_scenario = "qemu_remote_wakeup_request")]
static REMOTE_WAKE_QUEUES: [SpinLock<RemoteWakeQueue<REMOTE_WAKE_QUEUE_CAPACITY>>; MAX_CORES] = [
    SpinLock::new(RemoteWakeQueue::new(LogicalCpuId::new(0))),
    SpinLock::new(RemoteWakeQueue::new(LogicalCpuId::new(1))),
    SpinLock::new(RemoteWakeQueue::new(LogicalCpuId::new(2))),
    SpinLock::new(RemoteWakeQueue::new(LogicalCpuId::new(3))),
];

#[cfg(talos_boot_scenario = "qemu_remote_wakeup_request")]
fn reset_remote_wakeup_request_state() {
    REMOTE_WAKE_REQUEST_SMOKE_STATE.reset();
    for logical_cpu in 0..MAX_CORES {
        let mut queue = unsafe { REMOTE_WAKE_QUEUES[logical_cpu].lock_irqsave() };
        *queue = RemoteWakeQueue::new(LogicalCpuId::new(logical_cpu));
    }
}

#[cfg(talos_boot_scenario = "qemu_remote_wakeup_request")]
fn publish_remote_wake_request(target: usize, task_id: TaskId) -> bool {
    let target_cpu = LogicalCpuId::new(target);
    let result = {
        let mut queue = unsafe { REMOTE_WAKE_QUEUES[target].lock_irqsave() };
        queue.publish(LogicalCpuId::BOOT, target_cpu, task_id)
    };
    smp_full_barrier();

    match result {
        Ok(RemoteWakePublishOutcome::Inserted) => {
            crate::println!(
                "qemu-remote-wakeup-request: publish requester=0 target={} task={} outcome=inserted",
                target,
                task_id.raw()
            );
            true
        }
        Ok(RemoteWakePublishOutcome::Duplicate) => {
            crate::println!(
                "qemu-remote-wakeup-request: publish requester=0 target={} task={} outcome=duplicate",
                target,
                task_id.raw()
            );
            true
        }
        Err(error) => {
            REMOTE_WAKE_REQUEST_SMOKE_STATE
                .errors
                .fetch_add(1, Ordering::AcqRel);
            crate::println!(
                "qemu-remote-wakeup-request: publish requester=0 target={} task={} outcome=error {:?}",
                target,
                task_id.raw(),
                error
            );
            false
        }
    }
}

#[cfg(any(
    talos_boot_scenario = "qemu_remote_wake_to_local_runnable",
    talos_boot_scenario = "qemu_shared_scheduler_metadata",
    talos_boot_scenario = "qemu_shared_runqueue_migration",
    talos_boot_scenario = "qemu_load_balancing_smoke",
    talos_boot_scenario = "qemu_secondary_scheduler_service_loop",
    talos_boot_scenario = "qemu_multicore_preemption_smoke",
    talos_boot_scenario = "qemu_production_timer_preemption_smoke"
))]
fn task_state_code(state: TaskState) -> u64 {
    match state {
        TaskState::Running => 1,
        TaskState::Runnable => 2,
        TaskState::Blocked => 3,
    }
}

#[cfg(any(
    talos_boot_scenario = "qemu_remote_wake_to_local_runnable",
    talos_boot_scenario = "qemu_shared_scheduler_metadata",
    talos_boot_scenario = "qemu_shared_runqueue_migration",
    talos_boot_scenario = "qemu_load_balancing_smoke",
    talos_boot_scenario = "qemu_secondary_scheduler_service_loop",
    talos_boot_scenario = "qemu_multicore_preemption_smoke",
    talos_boot_scenario = "qemu_production_timer_preemption_smoke"
))]
fn task_state_name(code: u64) -> &'static str {
    match code {
        1 => "running",
        2 => "runnable",
        3 => "blocked",
        _ => "unknown",
    }
}

#[cfg(talos_boot_scenario = "qemu_remote_wakeup_request")]
fn run_remote_wakeup_request_secondary(core_state: &smp::PerCoreState, logical_cpu: usize) {
    core_state.mark_workload_running();
    core_state.clean_to_poc();

    crate::arch::aarch64::exceptions::init();
    unsafe {
        aarch64::disable_irq();
        aarch64::route_physical_irqs_to_el2();
        let gic = GicV2::new(GICD_BASE, GICC_BASE);
        gic.configure_sgi_priority(QEMU_CROSS_CORE_IPI_SGI_INTID, 0x80);
        gic.enable_cpu_interface();
        aarch64::enable_irq();
    }

    REMOTE_WAKE_REQUEST_SMOKE_STATE.mark_ready(logical_cpu);

    let mut remaining = QEMU_SECONDARY_WAIT_LIMIT;
    while REMOTE_WAKE_REQUEST_SMOKE_STATE.receive_count(logical_cpu) == 0 && remaining > 0 {
        unsafe {
            core::arch::asm!("wfe", options(nomem, nostack, preserves_flags));
        }
        remaining -= 1;
    }

    unsafe {
        aarch64::disable_irq();
    }

    let requester = LogicalCpuId::new(logical_cpu);
    let (consumed_request, duplicates, queue_len_after) = {
        let mut queue = unsafe { REMOTE_WAKE_QUEUES[logical_cpu].lock_irqsave() };
        let consumed = queue.consume_next(requester).ok().flatten();
        (consumed, queue.duplicate_count(), queue.len())
    };
    let consumed_task = consumed_request
        .map(|request| request.task_id().raw())
        .unwrap_or(0);

    REMOTE_WAKE_REQUEST_SMOKE_STATE.consumed_task_ids[logical_cpu]
        .store(consumed_task, Ordering::Release);
    REMOTE_WAKE_REQUEST_SMOKE_STATE.duplicate_counts[logical_cpu]
        .store(duplicates, Ordering::Release);
    REMOTE_WAKE_REQUEST_SMOKE_STATE.queue_lens_after[logical_cpu]
        .store(queue_len_after as u64, Ordering::Release);

    let mut scheduler = PerCoreScheduler::<2>::deferred_secondary(requester);
    if matches!(
        scheduler.local_scheduler_mut(LogicalCpuId::BOOT),
        Err(PerCoreSchedulerAccessError::WrongOwner { .. })
    ) {
        REMOTE_WAKE_REQUEST_SMOKE_STATE.cross_owner_rejections[logical_cpu]
            .store(1, Ordering::Release);
    }
    if matches!(
        scheduler.production_scheduler_mut(requester),
        Err(PerCoreSchedulerAccessError::ProductionDispatchDeferred { .. })
    ) {
        REMOTE_WAKE_REQUEST_SMOKE_STATE.production_deferrals[logical_cpu]
            .store(1, Ordering::Release);
    }

    #[cfg(talos_boot_scenario = "qemu_remote_wake_to_local_runnable")]
    {
        let local_task_id =
            TaskId::new(200 + logical_cpu as u64).expect("diagnostic task ID is nonzero");
        let stack_base = 0x80_0000 + logical_cpu * 0x1000;
        let mut task = Task::kernel_thread(
            local_task_id,
            KernelStack::new(stack_base, 0x1000).expect("diagnostic stack is valid"),
            ContextFrame::new(stack_base + 0xff0, 0x40_0000),
        );
        task.set_state(TaskState::Blocked);
        REMOTE_WAKE_REQUEST_SMOKE_STATE.local_state_before[logical_cpu]
            .store(task_state_code(task.state()), Ordering::Release);

        let wake_result = consumed_request
            .map(|request| {
                scheduler.wake_blocked_local_task_from_remote_request(requester, request, &mut task)
            })
            .unwrap_or(Err(TargetWakeConsumptionError::TaskMismatch {
                requested: local_task_id,
                local: local_task_id,
            }));
        if let Ok(woken_task) = wake_result {
            REMOTE_WAKE_REQUEST_SMOKE_STATE.local_wake_task_ids[logical_cpu]
                .store(woken_task.raw(), Ordering::Release);
        } else {
            REMOTE_WAKE_REQUEST_SMOKE_STATE
                .errors
                .fetch_add(1, Ordering::AcqRel);
        }

        let duplicate_rejected = consumed_request
            .map(|request| {
                scheduler
                    .wake_blocked_local_task_from_remote_request(requester, request, &mut task)
                    .is_err()
            })
            .unwrap_or(false);
        if duplicate_rejected {
            REMOTE_WAKE_REQUEST_SMOKE_STATE.duplicate_local_rejections[logical_cpu]
                .store(1, Ordering::Release);
        } else {
            REMOTE_WAKE_REQUEST_SMOKE_STATE
                .errors
                .fetch_add(1, Ordering::AcqRel);
        }

        REMOTE_WAKE_REQUEST_SMOKE_STATE.local_state_after[logical_cpu]
            .store(task_state_code(task.state()), Ordering::Release);
        REMOTE_WAKE_REQUEST_SMOKE_STATE.local_runnable_lens[logical_cpu].store(
            scheduler.scheduler().runnable().len() as u64,
            Ordering::Release,
        );
    }

    REMOTE_WAKE_REQUEST_SMOKE_STATE.mark_complete(logical_cpu);
    core_state.mark_workload_complete(consumed_task);
    core_state.clean_to_poc();
}

#[cfg(talos_boot_scenario = "qemu_smp_lock_contention")]
#[derive(Clone, Copy)]
struct SmpLockContentionState {
    shared_counter: u64,
    per_core_counts: [u64; MAX_CORES],
    error_count: u64,
}

#[cfg(talos_boot_scenario = "qemu_el0_trap_smoke")]
pub fn run_el0_trap_smoke() -> ! {
    crate::println!(
        "qemu-el0-trap-smoke: start user-text=[{:#018x},{:#018x}) user-stack=[{:#018x},{:#018x}) user-guard=[{:#018x},{:#018x}) marker={:#x}",
        EL0_TRAP_USER_TEXT_START,
        EL0_TRAP_USER_TEXT_START + EL0_TRAP_USER_TEXT_LEN as u64,
        EL0_TRAP_USER_STACK_START,
        EL0_TRAP_USER_STACK_START + EL0_TRAP_USER_STACK_LEN as u64,
        EL0_TRAP_USER_GUARD_START,
        EL0_TRAP_USER_STACK_START,
        EL0_TRAP_SVC_MARKER
    );

    let mappings = [
        UserMapping::new(
            EL0_TRAP_USER_TEXT_START,
            EL0_TRAP_USER_TEXT_LEN,
            UserMappingPermissions::USER_TEXT,
        )
        .expect("fixed EL0 text mapping is a valid user mapping"),
        UserMapping::new(
            EL0_TRAP_USER_STACK_START,
            EL0_TRAP_USER_STACK_LEN,
            UserMappingPermissions::USER_DATA,
        )
        .expect("fixed EL0 stack mapping is a valid user mapping"),
    ];
    let entry = validate_user_memory_access(
        &mappings,
        EL0_TRAP_USER_TEXT_START,
        4,
        UserAccessKind::Execute,
        EL0_TRAP_USER_TEXT_LEN,
    )
    .expect("EL0 entry validates inside fixed UserText")
    .start();
    let user_sp = EL0_TRAP_USER_STACK_START + EL0_TRAP_USER_STACK_LEN as u64;
    validate_user_memory_access(
        &mappings,
        user_sp - 16,
        16,
        UserAccessKind::Write,
        EL0_TRAP_USER_STACK_LEN,
    )
    .expect("EL0 stack top validates inside fixed UserStack");
    let guard_result = validate_user_memory_access(
        &mappings,
        EL0_TRAP_USER_GUARD_START,
        8,
        UserAccessKind::Read,
        EL0_TRAP_USER_TEXT_LEN,
    );
    let guard_blocked = matches!(guard_result, Err(PosixError::Fault));

    crate::println!(
        "qemu-el0-trap-smoke: validated elr={:#018x} sp={:#018x} spsr={:#018x} guard-blocked={}",
        entry,
        user_sp,
        EL0_TRAP_SPSR_EL0T_DAIF_MASKED,
        guard_blocked
    );
    if !guard_blocked {
        crate::println!(
            "qemu-el0-trap-smoke: final participants=0 expected=1 errors=1 classification=qemu-el0-trap-smoke-guard-open"
        );
        crate::target::qemu::exit_failure();
    }

    unsafe {
        install_el0_trap_smoke_tables();
        enable_el2_and_el0_translation();
        crate::println!("qemu-el0-trap-smoke: translation-ready");
        enable_el1_and_el0_translation();
        aarch64::enter_el1_then_el0(
            entry as usize,
            user_sp as usize,
            EL0_TRAP_SPSR_EL0T_DAIF_MASKED,
            EL0_TRAP_SPSR_EL1H_DAIF_MASKED,
        );
    }
}

#[cfg(talos_boot_scenario = "qemu_syscall_smoke")]
pub fn run_syscall_smoke() -> ! {
    crate::println!(
        "qemu-syscall-smoke: start user-text=[{:#018x},{:#018x}) user-stack=[{:#018x},{:#018x}) user-guard=[{:#018x},{:#018x}) stable-svc={:#06x} diagnostic-marker={:#06x}",
        EL0_TRAP_USER_TEXT_START,
        EL0_TRAP_USER_TEXT_START + EL0_TRAP_USER_TEXT_LEN as u64,
        EL0_TRAP_USER_STACK_START,
        EL0_TRAP_USER_STACK_START + EL0_TRAP_USER_STACK_LEN as u64,
        EL0_TRAP_USER_GUARD_START,
        EL0_TRAP_USER_STACK_START,
        syscall::STABLE_SVC_IMMEDIATE,
        syscall::DIAGNOSTIC_EL0_TRAP_SVC_IMMEDIATE
    );

    let mappings = [
        UserMapping::new(
            EL0_TRAP_USER_TEXT_START,
            EL0_TRAP_USER_TEXT_LEN,
            UserMappingPermissions::USER_TEXT,
        )
        .expect("fixed syscall smoke text mapping is a valid user mapping"),
        UserMapping::new(
            EL0_TRAP_USER_STACK_START,
            EL0_TRAP_USER_STACK_LEN,
            UserMappingPermissions::USER_DATA,
        )
        .expect("fixed syscall smoke stack mapping is a valid user mapping"),
    ];
    let entry = validate_user_memory_access(
        &mappings,
        EL0_TRAP_USER_TEXT_START,
        4,
        UserAccessKind::Execute,
        EL0_TRAP_USER_TEXT_LEN,
    )
    .expect("syscall smoke entry validates inside fixed UserText")
    .start();
    let user_sp = EL0_TRAP_USER_STACK_START + EL0_TRAP_USER_STACK_LEN as u64;
    validate_user_memory_access(
        &mappings,
        user_sp - 16,
        16,
        UserAccessKind::Write,
        EL0_TRAP_USER_STACK_LEN,
    )
    .expect("syscall smoke stack top validates inside fixed UserStack");
    let guard_result = validate_user_memory_access(
        &mappings,
        EL0_TRAP_USER_GUARD_START,
        8,
        UserAccessKind::Read,
        EL0_TRAP_USER_TEXT_LEN,
    );
    let guard_blocked = matches!(guard_result, Err(PosixError::Fault));

    crate::println!(
        "qemu-syscall-smoke: validated elr={:#018x} sp={:#018x} spsr={:#018x} guard-blocked={}",
        entry,
        user_sp,
        EL0_TRAP_SPSR_EL0T_DAIF_MASKED,
        guard_blocked
    );
    if !guard_blocked {
        crate::println!(
            "qemu-syscall-smoke: final participants=0 expected=2 errors=1 classification=qemu-syscall-smoke-guard-open"
        );
        crate::target::qemu::exit_failure();
    }

    unsafe {
        install_el0_trap_smoke_tables();
        enable_el2_and_el0_translation();
        enable_el1_and_el0_translation();
        aarch64::enter_el1_then_el0(
            entry as usize,
            user_sp as usize,
            EL0_TRAP_SPSR_EL0T_DAIF_MASKED,
            EL0_TRAP_SPSR_EL1H_DAIF_MASKED,
        );
    }
}

#[cfg(talos_boot_scenario = "qemu_pointer_copy_smoke")]
pub fn run_pointer_copy_smoke() -> ! {
    crate::println!("qemu-pointer-copy-smoke: start");

    let mappings = [
        UserMapping::new(
            EL0_TRAP_USER_TEXT_START,
            EL0_TRAP_USER_TEXT_LEN,
            UserMappingPermissions::USER_TEXT,
        )
        .expect("fixed pointer-copy smoke text mapping is a valid user mapping"),
        UserMapping::new(
            POINTER_COPY_USER_DATA_START,
            POINTER_COPY_USER_DATA_LEN,
            UserMappingPermissions::USER_DATA,
        )
        .expect("fixed pointer-copy smoke data mapping is a valid user mapping"),
        UserMapping::new(
            EL0_TRAP_USER_STACK_START,
            EL0_TRAP_USER_STACK_LEN,
            UserMappingPermissions::USER_DATA,
        )
        .expect("fixed pointer-copy smoke stack mapping is a valid user mapping"),
    ];
    let entry = validate_user_memory_access(
        &mappings,
        EL0_TRAP_USER_TEXT_START,
        4,
        UserAccessKind::Execute,
        EL0_TRAP_USER_TEXT_LEN,
    )
    .expect("pointer-copy smoke entry validates inside fixed UserText")
    .start();
    validate_user_memory_access(
        &mappings,
        POINTER_COPY_USER_DATA_START,
        POINTER_COPY_USER_DATA_LEN,
        UserAccessKind::Write,
        POINTER_COPY_USER_DATA_LEN,
    )
    .expect("pointer-copy smoke data validates inside fixed UserData");
    let user_sp = EL0_TRAP_USER_STACK_START + EL0_TRAP_USER_STACK_LEN as u64;
    validate_user_memory_access(
        &mappings,
        user_sp - 16,
        16,
        UserAccessKind::Write,
        EL0_TRAP_USER_STACK_LEN,
    )
    .expect("pointer-copy smoke stack top validates inside fixed UserStack");
    let guard_result = validate_user_memory_access(
        &mappings,
        EL0_TRAP_USER_GUARD_START,
        16,
        UserAccessKind::Read,
        EL0_TRAP_USER_TEXT_LEN,
    );
    let guard_blocked = matches!(guard_result, Err(PosixError::Fault));

    crate::println!(
        "qemu-pointer-copy-smoke: validated elr={:#018x} sp={:#018x} user-data={:#018x} user-data-len={:#018x} guard-blocked={}",
        entry,
        user_sp,
        POINTER_COPY_USER_DATA_START,
        POINTER_COPY_USER_DATA_LEN as u64,
        guard_blocked
    );
    if !guard_blocked {
        crate::println!(
            "qemu-pointer-copy-smoke: final participants=0 expected=3 errors=1 classification=qemu-pointer-copy-smoke-guard-open"
        );
        crate::target::qemu::exit_failure();
    }

    unsafe {
        core::ptr::write_bytes(
            core::ptr::addr_of_mut!(POINTER_COPY_USER_DATA).cast::<u8>(),
            POINTER_COPY_USER_DATA_INIT,
            POINTER_COPY_USER_DATA_LEN,
        );
        install_el0_trap_smoke_tables();
        enable_el2_and_el0_translation();
        enable_el1_and_el0_translation();
        aarch64::enter_el1_then_el0(
            entry as usize,
            user_sp as usize,
            EL0_TRAP_SPSR_EL0T_DAIF_MASKED,
            EL0_TRAP_SPSR_EL1H_DAIF_MASKED,
        );
    }
}

#[cfg(talos_boot_scenario = "qemu_descriptor_write_smoke")]
pub fn run_descriptor_write_smoke() -> ! {
    crate::println!("qemu-descriptor-write-smoke: start");

    let descriptor_table =
        crate::posix::DescriptorTable::<4>::with_inherited_stdio().expect("stdio table");
    let mappings = [
        UserMapping::new(
            EL0_TRAP_USER_TEXT_START,
            EL0_TRAP_USER_TEXT_LEN,
            UserMappingPermissions::USER_TEXT,
        )
        .expect("fixed descriptor-write smoke text mapping is a valid user mapping"),
        UserMapping::new(
            DESCRIPTOR_WRITE_USER_DATA_START,
            DESCRIPTOR_WRITE_USER_DATA_LEN,
            UserMappingPermissions::USER_DATA,
        )
        .expect("fixed descriptor-write smoke data mapping is a valid user mapping"),
        UserMapping::new(
            EL0_TRAP_USER_STACK_START,
            EL0_TRAP_USER_STACK_LEN,
            UserMappingPermissions::USER_DATA,
        )
        .expect("fixed descriptor-write smoke stack mapping is a valid user mapping"),
    ];
    let entry = validate_user_memory_access(
        &mappings,
        EL0_TRAP_USER_TEXT_START,
        4,
        UserAccessKind::Execute,
        EL0_TRAP_USER_TEXT_LEN,
    )
    .expect("descriptor-write smoke entry validates inside fixed UserText")
    .start();
    validate_user_memory_access(
        &mappings,
        DESCRIPTOR_WRITE_USER_DATA_START,
        DESCRIPTOR_WRITE_USER_DATA_LEN,
        UserAccessKind::Write,
        DESCRIPTOR_WRITE_USER_DATA_LEN,
    )
    .expect("descriptor-write smoke data validates inside fixed UserData");
    descriptor_table
        .get(crate::posix::STDOUT_FD)
        .expect("inherited stdout descriptor exists");
    descriptor_table
        .get(crate::posix::STDERR_FD)
        .expect("inherited stderr descriptor exists");
    let user_sp = EL0_TRAP_USER_STACK_START + EL0_TRAP_USER_STACK_LEN as u64;
    validate_user_memory_access(
        &mappings,
        user_sp - 16,
        16,
        UserAccessKind::Write,
        EL0_TRAP_USER_STACK_LEN,
    )
    .expect("descriptor-write smoke stack top validates inside fixed UserStack");
    let guard_result = validate_user_memory_access(
        &mappings,
        EL0_TRAP_USER_GUARD_START,
        16,
        UserAccessKind::Read,
        EL0_TRAP_USER_TEXT_LEN,
    );
    let guard_blocked = matches!(guard_result, Err(PosixError::Fault));

    crate::println!(
        "qemu-descriptor-write-smoke: validated elr={:#018x} sp={:#018x} user-data={:#018x} user-data-len={:#018x} guard-blocked={} descriptor-table=inherited-stdio runtime-console=runtime-console0",
        entry,
        user_sp,
        DESCRIPTOR_WRITE_USER_DATA_START,
        DESCRIPTOR_WRITE_USER_DATA_LEN as u64,
        guard_blocked
    );
    if !guard_blocked {
        crate::println!(
            "qemu-descriptor-write-smoke: final participants=0 expected=8 errors=1 classification=qemu-descriptor-write-smoke-guard-open"
        );
        crate::target::qemu::exit_failure();
    }

    unsafe {
        core::ptr::write_bytes(
            core::ptr::addr_of_mut!(DESCRIPTOR_WRITE_USER_DATA).cast::<u8>(),
            0,
            DESCRIPTOR_WRITE_USER_DATA_LEN,
        );
        let data = &mut *core::ptr::addr_of_mut!(DESCRIPTOR_WRITE_USER_DATA);
        data[DESCRIPTOR_WRITE_STDOUT_OFFSET
            ..DESCRIPTOR_WRITE_STDOUT_OFFSET + DESCRIPTOR_WRITE_STDOUT.len()]
            .copy_from_slice(DESCRIPTOR_WRITE_STDOUT);
        data[DESCRIPTOR_WRITE_STDERR_OFFSET
            ..DESCRIPTOR_WRITE_STDERR_OFFSET + DESCRIPTOR_WRITE_STDERR.len()]
            .copy_from_slice(DESCRIPTOR_WRITE_STDERR);
        core::ptr::write_bytes(
            core::ptr::addr_of_mut!(DESCRIPTOR_WRITE_CONSOLE_CAPTURE).cast::<u8>(),
            0,
            64,
        );
        DESCRIPTOR_WRITE_CONSOLE_LEN.store(0, Ordering::Relaxed);
        install_el0_trap_smoke_tables();
        enable_el2_and_el0_translation();
        enable_el1_and_el0_translation();
        aarch64::enter_el1_then_el0(
            entry as usize,
            user_sp as usize,
            EL0_TRAP_SPSR_EL0T_DAIF_MASKED,
            EL0_TRAP_SPSR_EL1H_DAIF_MASKED,
        );
    }
}

#[cfg(talos_boot_scenario = "qemu_process_descriptor_stdio_smoke")]
pub fn run_process_descriptor_stdio_smoke() -> ! {
    crate::println!("qemu-process-descriptor-stdio-smoke: start");

    let current_owner = crate::scheduler::ProcessOwnerId::new(PROCESS_DESCRIPTOR_STDIO_OWNER_RAW)
        .expect("process descriptor stdio owner id is nonzero");
    unsafe {
        *core::ptr::addr_of_mut!(PROCESS_DESCRIPTOR_STDIO_STORE) =
            crate::posix::ProcessDescriptorStore::new_empty();
        (*core::ptr::addr_of_mut!(PROCESS_DESCRIPTOR_STDIO_STORE))
            .create_owner_with_inherited_stdio(current_owner)
            .expect("process-owned inherited stdio table");
    }
    let store = unsafe { &*core::ptr::addr_of!(PROCESS_DESCRIPTOR_STDIO_STORE) };
    let descriptor_table = store
        .current_descriptor_table(Some(current_owner))
        .expect("current process owner resolves descriptor table");
    descriptor_table
        .get(crate::posix::STDOUT_FD)
        .expect("process-owned stdout descriptor exists");
    descriptor_table
        .get(crate::posix::STDERR_FD)
        .expect("process-owned stderr descriptor exists");

    let mappings = [
        UserMapping::new(
            EL0_TRAP_USER_TEXT_START,
            EL0_TRAP_USER_TEXT_LEN,
            UserMappingPermissions::USER_TEXT,
        )
        .expect("fixed process-descriptor stdio smoke text mapping is a valid user mapping"),
        UserMapping::new(
            DESCRIPTOR_WRITE_USER_DATA_START,
            DESCRIPTOR_WRITE_USER_DATA_LEN,
            UserMappingPermissions::USER_DATA,
        )
        .expect("fixed process-descriptor stdio smoke data mapping is a valid user mapping"),
        UserMapping::new(
            EL0_TRAP_USER_STACK_START,
            EL0_TRAP_USER_STACK_LEN,
            UserMappingPermissions::USER_DATA,
        )
        .expect("fixed process-descriptor stdio smoke stack mapping is a valid user mapping"),
    ];
    let entry = validate_user_memory_access(
        &mappings,
        EL0_TRAP_USER_TEXT_START,
        4,
        UserAccessKind::Execute,
        EL0_TRAP_USER_TEXT_LEN,
    )
    .expect("process-descriptor stdio smoke entry validates inside fixed UserText")
    .start();
    validate_user_memory_access(
        &mappings,
        DESCRIPTOR_WRITE_USER_DATA_START,
        DESCRIPTOR_WRITE_USER_DATA_LEN,
        UserAccessKind::Write,
        DESCRIPTOR_WRITE_USER_DATA_LEN,
    )
    .expect("process-descriptor stdio smoke data validates inside fixed UserData");
    let user_sp = EL0_TRAP_USER_STACK_START + EL0_TRAP_USER_STACK_LEN as u64;
    validate_user_memory_access(
        &mappings,
        user_sp - 16,
        16,
        UserAccessKind::Write,
        EL0_TRAP_USER_STACK_LEN,
    )
    .expect("process-descriptor stdio smoke stack top validates inside fixed UserStack");
    let guard_result = validate_user_memory_access(
        &mappings,
        EL0_TRAP_USER_GUARD_START,
        16,
        UserAccessKind::Read,
        EL0_TRAP_USER_TEXT_LEN,
    );
    let guard_blocked = matches!(guard_result, Err(PosixError::Fault));

    crate::println!(
        "qemu-process-descriptor-stdio-smoke: validated elr={:#018x} sp={:#018x} user-data={:#018x} user-data-len={:#018x} guard-blocked={} process-owner={:#018x} current-owner={:#018x} descriptor-table=process-owned-inherited-stdio runtime-console=runtime-console0",
        entry,
        user_sp,
        DESCRIPTOR_WRITE_USER_DATA_START,
        DESCRIPTOR_WRITE_USER_DATA_LEN as u64,
        guard_blocked,
        current_owner.raw(),
        current_owner.raw()
    );
    crate::println!(
        "qemu-process-descriptor-stdio-smoke: current-descriptor-table lookup=process-owned owner={:#018x} resolved=true stdio=inherited runtime-console=runtime-console0",
        current_owner.raw()
    );
    if !guard_blocked {
        crate::println!(
            "qemu-process-descriptor-stdio-smoke: final participants=0 expected=8 errors=1 classification=qemu-process-descriptor-stdio-smoke-guard-open"
        );
        crate::target::qemu::exit_failure();
    }

    unsafe {
        core::ptr::write_bytes(
            core::ptr::addr_of_mut!(DESCRIPTOR_WRITE_USER_DATA).cast::<u8>(),
            0,
            DESCRIPTOR_WRITE_USER_DATA_LEN,
        );
        let data = &mut *core::ptr::addr_of_mut!(DESCRIPTOR_WRITE_USER_DATA);
        data[DESCRIPTOR_WRITE_STDOUT_OFFSET
            ..DESCRIPTOR_WRITE_STDOUT_OFFSET + DESCRIPTOR_WRITE_STDOUT.len()]
            .copy_from_slice(DESCRIPTOR_WRITE_STDOUT);
        data[DESCRIPTOR_WRITE_STDERR_OFFSET
            ..DESCRIPTOR_WRITE_STDERR_OFFSET + DESCRIPTOR_WRITE_STDERR.len()]
            .copy_from_slice(DESCRIPTOR_WRITE_STDERR);
        core::ptr::write_bytes(
            core::ptr::addr_of_mut!(DESCRIPTOR_WRITE_CONSOLE_CAPTURE).cast::<u8>(),
            0,
            64,
        );
        DESCRIPTOR_WRITE_CONSOLE_LEN.store(0, Ordering::Relaxed);
        DESCRIPTOR_WRITE_STDOUT_OBSERVED.store(0, Ordering::Relaxed);
        DESCRIPTOR_WRITE_STDERR_OBSERVED.store(0, Ordering::Relaxed);
        DESCRIPTOR_WRITE_FD0_OBSERVED.store(0, Ordering::Relaxed);
        DESCRIPTOR_WRITE_BADFD_OBSERVED.store(0, Ordering::Relaxed);
        DESCRIPTOR_WRITE_EFAULT_OBSERVED.store(0, Ordering::Relaxed);
        DESCRIPTOR_WRITE_RESERVED_OBSERVED.store(0, Ordering::Relaxed);
        DESCRIPTOR_WRITE_TALOS_NOP_OBSERVED.store(0, Ordering::Relaxed);
        DESCRIPTOR_WRITE_UNKNOWN_OBSERVED.store(0, Ordering::Relaxed);
        DESCRIPTOR_WRITE_COPY_PROBE_OBSERVED.store(0, Ordering::Relaxed);
        DESCRIPTOR_WRITE_ERRORS.store(0, Ordering::Relaxed);
        install_el0_trap_smoke_tables();
        enable_el2_and_el0_translation();
        enable_el1_and_el0_translation();
        aarch64::enter_el1_then_el0(
            entry as usize,
            user_sp as usize,
            EL0_TRAP_SPSR_EL0T_DAIF_MASKED,
            EL0_TRAP_SPSR_EL1H_DAIF_MASKED,
        );
    }
}

#[cfg(talos_boot_scenario = "qemu_close_syscall_smoke")]
pub fn run_close_syscall_smoke() -> ! {
    crate::println!("qemu-close-syscall-smoke: start");

    let current_owner = crate::scheduler::ProcessOwnerId::new(PROCESS_DESCRIPTOR_STDIO_OWNER_RAW)
        .expect("close syscall smoke owner id is nonzero");
    unsafe {
        *core::ptr::addr_of_mut!(PROCESS_DESCRIPTOR_STDIO_STORE) =
            crate::posix::ProcessDescriptorStore::new_empty();
        (*core::ptr::addr_of_mut!(PROCESS_DESCRIPTOR_STDIO_STORE))
            .create_owner_with_inherited_stdio(current_owner)
            .expect("process-owned inherited stdio table");
    }
    let store = unsafe { &*core::ptr::addr_of!(PROCESS_DESCRIPTOR_STDIO_STORE) };
    let descriptor_table = store
        .current_descriptor_table(Some(current_owner))
        .expect("current process owner resolves descriptor table");
    descriptor_table
        .get(crate::posix::STDOUT_FD)
        .expect("process-owned stdout descriptor exists");
    descriptor_table
        .get(crate::posix::STDERR_FD)
        .expect("process-owned stderr descriptor exists");

    let mappings = [
        UserMapping::new(
            EL0_TRAP_USER_TEXT_START,
            EL0_TRAP_USER_TEXT_LEN,
            UserMappingPermissions::USER_TEXT,
        )
        .expect("fixed close syscall smoke text mapping is a valid user mapping"),
        UserMapping::new(
            DESCRIPTOR_WRITE_USER_DATA_START,
            DESCRIPTOR_WRITE_USER_DATA_LEN,
            UserMappingPermissions::USER_DATA,
        )
        .expect("fixed close syscall smoke data mapping is a valid user mapping"),
        UserMapping::new(
            EL0_TRAP_USER_STACK_START,
            EL0_TRAP_USER_STACK_LEN,
            UserMappingPermissions::USER_DATA,
        )
        .expect("fixed close syscall smoke stack mapping is a valid user mapping"),
    ];
    let entry = validate_user_memory_access(
        &mappings,
        EL0_TRAP_USER_TEXT_START,
        4,
        UserAccessKind::Execute,
        EL0_TRAP_USER_TEXT_LEN,
    )
    .expect("close syscall smoke entry validates inside fixed UserText")
    .start();
    validate_user_memory_access(
        &mappings,
        DESCRIPTOR_WRITE_USER_DATA_START,
        DESCRIPTOR_WRITE_USER_DATA_LEN,
        UserAccessKind::Write,
        DESCRIPTOR_WRITE_USER_DATA_LEN,
    )
    .expect("close syscall smoke data validates inside fixed UserData");
    let user_sp = EL0_TRAP_USER_STACK_START + EL0_TRAP_USER_STACK_LEN as u64;
    validate_user_memory_access(
        &mappings,
        user_sp - 16,
        16,
        UserAccessKind::Write,
        EL0_TRAP_USER_STACK_LEN,
    )
    .expect("close syscall smoke stack top validates inside fixed UserStack");
    let guard_result = validate_user_memory_access(
        &mappings,
        EL0_TRAP_USER_GUARD_START,
        16,
        UserAccessKind::Read,
        EL0_TRAP_USER_TEXT_LEN,
    );
    let guard_blocked = matches!(guard_result, Err(PosixError::Fault));

    crate::println!(
        "qemu-close-syscall-smoke: validated elr={:#018x} sp={:#018x} user-data={:#018x} user-data-len={:#018x} guard-blocked={} process-owner={:#018x} current-owner={:#018x} descriptor-table=process-owned-inherited-stdio runtime-console=runtime-console0",
        entry,
        user_sp,
        DESCRIPTOR_WRITE_USER_DATA_START,
        DESCRIPTOR_WRITE_USER_DATA_LEN as u64,
        guard_blocked,
        current_owner.raw(),
        current_owner.raw()
    );
    crate::println!(
        "qemu-close-syscall-smoke: current-descriptor-table lookup=process-owned owner={:#018x} resolved=true stdio=inherited runtime-console=runtime-console0",
        current_owner.raw()
    );
    if !guard_blocked {
        crate::println!(
            "qemu-close-syscall-smoke: final participants=0 expected=11 errors=1 classification=qemu-close-syscall-smoke-guard-open"
        );
        crate::target::qemu::exit_failure();
    }

    unsafe {
        core::ptr::write_bytes(
            core::ptr::addr_of_mut!(DESCRIPTOR_WRITE_USER_DATA).cast::<u8>(),
            0,
            DESCRIPTOR_WRITE_USER_DATA_LEN,
        );
        let data = &mut *core::ptr::addr_of_mut!(DESCRIPTOR_WRITE_USER_DATA);
        data[DESCRIPTOR_WRITE_STDOUT_OFFSET
            ..DESCRIPTOR_WRITE_STDOUT_OFFSET + DESCRIPTOR_WRITE_STDOUT.len()]
            .copy_from_slice(DESCRIPTOR_WRITE_STDOUT);
        data[DESCRIPTOR_WRITE_STDERR_OFFSET
            ..DESCRIPTOR_WRITE_STDERR_OFFSET + DESCRIPTOR_WRITE_STDERR.len()]
            .copy_from_slice(DESCRIPTOR_WRITE_STDERR);
        core::ptr::write_bytes(
            core::ptr::addr_of_mut!(DESCRIPTOR_WRITE_CONSOLE_CAPTURE).cast::<u8>(),
            0,
            64,
        );
        DESCRIPTOR_WRITE_CONSOLE_LEN.store(0, Ordering::Relaxed);
        DESCRIPTOR_WRITE_ERRORS.store(0, Ordering::Relaxed);
        CLOSE_SYSCALL_CLOSE_STDOUT_OBSERVED.store(0, Ordering::Relaxed);
        CLOSE_SYSCALL_WRITE_CLOSED_STDOUT_OBSERVED.store(0, Ordering::Relaxed);
        CLOSE_SYSCALL_CLOSE_STDERR_RESERVED_OBSERVED.store(0, Ordering::Relaxed);
        CLOSE_SYSCALL_WRITE_STDERR_OBSERVED.store(0, Ordering::Relaxed);
        CLOSE_SYSCALL_CLOSE_STDERR_OBSERVED.store(0, Ordering::Relaxed);
        CLOSE_SYSCALL_WRITE_CLOSED_STDERR_OBSERVED.store(0, Ordering::Relaxed);
        CLOSE_SYSCALL_CLOSE_STDOUT_AGAIN_OBSERVED.store(0, Ordering::Relaxed);
        CLOSE_SYSCALL_CLOSE_BADFD_OBSERVED.store(0, Ordering::Relaxed);
        CLOSE_SYSCALL_TALOS_NOP_OBSERVED.store(0, Ordering::Relaxed);
        CLOSE_SYSCALL_UNKNOWN_OBSERVED.store(0, Ordering::Relaxed);
        CLOSE_SYSCALL_COPY_PROBE_OBSERVED.store(0, Ordering::Relaxed);
        install_el0_trap_smoke_tables();
        enable_el2_and_el0_translation();
        enable_el1_and_el0_translation();
        aarch64::enter_el1_then_el0(
            entry as usize,
            user_sp as usize,
            EL0_TRAP_SPSR_EL0T_DAIF_MASKED,
            EL0_TRAP_SPSR_EL1H_DAIF_MASKED,
        );
    }
}

#[cfg(talos_boot_scenario = "qemu_dup_syscall_smoke")]
pub fn run_dup_syscall_smoke() -> ! {
    crate::println!("qemu-dup-syscall-smoke: start");

    let current_owner = crate::scheduler::ProcessOwnerId::new(PROCESS_DESCRIPTOR_STDIO_OWNER_RAW)
        .expect("dup syscall smoke owner id is nonzero");
    unsafe {
        *core::ptr::addr_of_mut!(PROCESS_DESCRIPTOR_STDIO_STORE) =
            crate::posix::ProcessDescriptorStore::new_empty();
        (*core::ptr::addr_of_mut!(PROCESS_DESCRIPTOR_STDIO_STORE))
            .create_owner_with_inherited_stdio(current_owner)
            .expect("process-owned inherited stdio table");
    }
    let store = unsafe { &*core::ptr::addr_of!(PROCESS_DESCRIPTOR_STDIO_STORE) };
    let descriptor_table = store
        .current_descriptor_table(Some(current_owner))
        .expect("current process owner resolves descriptor table");
    descriptor_table
        .get(crate::posix::STDOUT_FD)
        .expect("process-owned stdout descriptor exists");
    descriptor_table
        .get(crate::posix::STDERR_FD)
        .expect("process-owned stderr descriptor exists");

    let mappings = [
        UserMapping::new(
            EL0_TRAP_USER_TEXT_START,
            EL0_TRAP_USER_TEXT_LEN,
            UserMappingPermissions::USER_TEXT,
        )
        .expect("fixed dup syscall smoke text mapping is a valid user mapping"),
        UserMapping::new(
            DESCRIPTOR_WRITE_USER_DATA_START,
            DESCRIPTOR_WRITE_USER_DATA_LEN,
            UserMappingPermissions::USER_DATA,
        )
        .expect("fixed dup syscall smoke data mapping is a valid user mapping"),
        UserMapping::new(
            EL0_TRAP_USER_STACK_START,
            EL0_TRAP_USER_STACK_LEN,
            UserMappingPermissions::USER_DATA,
        )
        .expect("fixed dup syscall smoke stack mapping is a valid user mapping"),
    ];
    let entry = validate_user_memory_access(
        &mappings,
        EL0_TRAP_USER_TEXT_START,
        4,
        UserAccessKind::Execute,
        EL0_TRAP_USER_TEXT_LEN,
    )
    .expect("dup syscall smoke entry validates inside fixed UserText")
    .start();
    validate_user_memory_access(
        &mappings,
        DESCRIPTOR_WRITE_USER_DATA_START,
        DESCRIPTOR_WRITE_USER_DATA_LEN,
        UserAccessKind::Write,
        DESCRIPTOR_WRITE_USER_DATA_LEN,
    )
    .expect("dup syscall smoke data validates inside fixed UserData");
    let user_sp = EL0_TRAP_USER_STACK_START + EL0_TRAP_USER_STACK_LEN as u64;
    validate_user_memory_access(
        &mappings,
        user_sp - 16,
        16,
        UserAccessKind::Write,
        EL0_TRAP_USER_STACK_LEN,
    )
    .expect("dup syscall smoke stack top validates inside fixed UserStack");
    let guard_result = validate_user_memory_access(
        &mappings,
        EL0_TRAP_USER_GUARD_START,
        16,
        UserAccessKind::Read,
        EL0_TRAP_USER_TEXT_LEN,
    );
    let guard_blocked = matches!(guard_result, Err(PosixError::Fault));

    crate::println!(
        "qemu-dup-syscall-smoke: validated elr={:#018x} sp={:#018x} user-data={:#018x} user-data-len={:#018x} guard-blocked={} process-owner={:#018x} current-owner={:#018x} descriptor-table=process-owned-inherited-stdio descriptor-capacity=4 runtime-console=runtime-console0",
        entry,
        user_sp,
        DESCRIPTOR_WRITE_USER_DATA_START,
        DESCRIPTOR_WRITE_USER_DATA_LEN as u64,
        guard_blocked,
        current_owner.raw(),
        current_owner.raw()
    );
    crate::println!(
        "qemu-dup-syscall-smoke: current-descriptor-table lookup=process-owned owner={:#018x} resolved=true stdio=inherited runtime-console=runtime-console0",
        current_owner.raw()
    );
    if !guard_blocked {
        crate::println!(
            "qemu-dup-syscall-smoke: final participants=0 expected=14 errors=1 classification=qemu-dup-syscall-smoke-guard-open"
        );
        crate::target::qemu::exit_failure();
    }

    unsafe {
        core::ptr::write_bytes(
            core::ptr::addr_of_mut!(DESCRIPTOR_WRITE_USER_DATA).cast::<u8>(),
            0,
            DESCRIPTOR_WRITE_USER_DATA_LEN,
        );
        let data = &mut *core::ptr::addr_of_mut!(DESCRIPTOR_WRITE_USER_DATA);
        data[DESCRIPTOR_WRITE_STDOUT_OFFSET
            ..DESCRIPTOR_WRITE_STDOUT_OFFSET + DUP_SYSCALL_SOURCE.len()]
            .copy_from_slice(DUP_SYSCALL_SOURCE);
        data[DESCRIPTOR_WRITE_STDERR_OFFSET
            ..DESCRIPTOR_WRITE_STDERR_OFFSET + DUP_SYSCALL_DUPLICATE.len()]
            .copy_from_slice(DUP_SYSCALL_DUPLICATE);
        core::ptr::write_bytes(
            core::ptr::addr_of_mut!(DESCRIPTOR_WRITE_CONSOLE_CAPTURE).cast::<u8>(),
            0,
            64,
        );
        DESCRIPTOR_WRITE_CONSOLE_LEN.store(0, Ordering::Relaxed);
        DESCRIPTOR_WRITE_ERRORS.store(0, Ordering::Relaxed);
        DUP_SYSCALL_DUP_STDOUT_OBSERVED.store(0, Ordering::Relaxed);
        DUP_SYSCALL_DUP_STDERR_FULL_OBSERVED.store(0, Ordering::Relaxed);
        DUP_SYSCALL_DUP_STDOUT_RESERVED_OBSERVED.store(0, Ordering::Relaxed);
        DUP_SYSCALL_WRITE_SOURCE_OBSERVED.store(0, Ordering::Relaxed);
        DUP_SYSCALL_WRITE_DUPLICATE_OBSERVED.store(0, Ordering::Relaxed);
        DUP_SYSCALL_CLOSE_SOURCE_OBSERVED.store(0, Ordering::Relaxed);
        DUP_SYSCALL_WRITE_SOURCE_CLOSED_OBSERVED.store(0, Ordering::Relaxed);
        DUP_SYSCALL_WRITE_DUPLICATE_AFTER_SOURCE_CLOSE_OBSERVED.store(0, Ordering::Relaxed);
        DUP_SYSCALL_CLOSE_DUPLICATE_OBSERVED.store(0, Ordering::Relaxed);
        DUP_SYSCALL_WRITE_DUPLICATE_CLOSED_OBSERVED.store(0, Ordering::Relaxed);
        DUP_SYSCALL_DUP_CLOSED_SOURCE_OBSERVED.store(0, Ordering::Relaxed);
        DUP_SYSCALL_TALOS_NOP_OBSERVED.store(0, Ordering::Relaxed);
        DUP_SYSCALL_UNKNOWN_OBSERVED.store(0, Ordering::Relaxed);
        DUP_SYSCALL_COPY_PROBE_OBSERVED.store(0, Ordering::Relaxed);
        install_el0_trap_smoke_tables();
        enable_el2_and_el0_translation();
        enable_el1_and_el0_translation();
        aarch64::enter_el1_then_el0(
            entry as usize,
            user_sp as usize,
            EL0_TRAP_SPSR_EL0T_DAIF_MASKED,
            EL0_TRAP_SPSR_EL1H_DAIF_MASKED,
        );
    }
}

#[cfg(talos_boot_scenario = "qemu_read_stdin_smoke")]
pub fn run_read_stdin_smoke() -> ! {
    crate::println!("qemu-read-stdin-smoke: start");

    let current_owner = crate::scheduler::ProcessOwnerId::new(PROCESS_DESCRIPTOR_STDIO_OWNER_RAW)
        .expect("read stdin smoke owner id is nonzero");
    unsafe {
        *core::ptr::addr_of_mut!(PROCESS_DESCRIPTOR_STDIO_STORE) =
            crate::posix::ProcessDescriptorStore::new_empty();
        (*core::ptr::addr_of_mut!(PROCESS_DESCRIPTOR_STDIO_STORE))
            .create_owner_with_inherited_stdio(current_owner)
            .expect("process-owned inherited stdio table");
        *core::ptr::addr_of_mut!(READ_STDIN_FIXED_STATE) =
            crate::posix::FixedStdin::new(READ_STDIN_FIXED_BYTES);
    }
    let store = unsafe { &*core::ptr::addr_of!(PROCESS_DESCRIPTOR_STDIO_STORE) };
    let descriptor_table = store
        .current_descriptor_table(Some(current_owner))
        .expect("current process owner resolves descriptor table");
    descriptor_table
        .get(crate::posix::STDIN_FD)
        .expect("process-owned stdin descriptor exists");
    descriptor_table
        .get(crate::posix::STDOUT_FD)
        .expect("process-owned stdout descriptor exists");
    descriptor_table
        .get(crate::posix::STDERR_FD)
        .expect("process-owned stderr descriptor exists");

    let mappings = [
        UserMapping::new(
            EL0_TRAP_USER_TEXT_START,
            EL0_TRAP_USER_TEXT_LEN,
            UserMappingPermissions::USER_TEXT,
        )
        .expect("fixed read stdin smoke text mapping is a valid user mapping"),
        UserMapping::new(
            DESCRIPTOR_WRITE_USER_DATA_START,
            DESCRIPTOR_WRITE_USER_DATA_LEN,
            UserMappingPermissions::USER_DATA,
        )
        .expect("fixed read stdin smoke data mapping is a valid user mapping"),
        UserMapping::new(
            EL0_TRAP_USER_STACK_START,
            EL0_TRAP_USER_STACK_LEN,
            UserMappingPermissions::USER_DATA,
        )
        .expect("fixed read stdin smoke stack mapping is a valid user mapping"),
    ];
    let entry = validate_user_memory_access(
        &mappings,
        EL0_TRAP_USER_TEXT_START,
        4,
        UserAccessKind::Execute,
        EL0_TRAP_USER_TEXT_LEN,
    )
    .expect("read stdin smoke entry validates inside fixed UserText")
    .start();
    validate_user_memory_access(
        &mappings,
        DESCRIPTOR_WRITE_USER_DATA_START,
        DESCRIPTOR_WRITE_USER_DATA_LEN,
        UserAccessKind::Write,
        DESCRIPTOR_WRITE_USER_DATA_LEN,
    )
    .expect("read stdin smoke data validates inside fixed UserData");
    let user_sp = EL0_TRAP_USER_STACK_START + EL0_TRAP_USER_STACK_LEN as u64;
    validate_user_memory_access(
        &mappings,
        user_sp - 16,
        16,
        UserAccessKind::Write,
        EL0_TRAP_USER_STACK_LEN,
    )
    .expect("read stdin smoke stack top validates inside fixed UserStack");
    let guard_result = validate_user_memory_access(
        &mappings,
        EL0_TRAP_USER_GUARD_START,
        16,
        UserAccessKind::Read,
        EL0_TRAP_USER_TEXT_LEN,
    );
    let guard_blocked = matches!(guard_result, Err(PosixError::Fault));

    crate::println!(
        "qemu-read-stdin-smoke: validated elr={:#018x} sp={:#018x} user-data={:#018x} user-data-len={:#018x} guard-blocked={} process-owner={:#018x} current-owner={:#018x} descriptor-table=process-owned-inherited-stdio descriptor-capacity=4 fixed-stdin-len={} fixed-stdin-cursor={}",
        entry,
        user_sp,
        DESCRIPTOR_WRITE_USER_DATA_START,
        DESCRIPTOR_WRITE_USER_DATA_LEN as u64,
        guard_blocked,
        current_owner.raw(),
        current_owner.raw(),
        READ_STDIN_FIXED_BYTES.len(),
        unsafe { &*core::ptr::addr_of!(READ_STDIN_FIXED_STATE) }.cursor()
    );
    crate::println!(
        "qemu-read-stdin-smoke: current-descriptor-table lookup=process-owned owner={:#018x} resolved=true stdio=inherited fixed-stdin=proof-buffer",
        current_owner.raw()
    );
    if !guard_blocked {
        crate::println!(
            "qemu-read-stdin-smoke: final participants=0 expected=11 errors=1 classification=qemu-read-stdin-smoke-guard-open"
        );
        crate::target::qemu::exit_failure();
    }

    unsafe {
        core::ptr::write_bytes(
            core::ptr::addr_of_mut!(DESCRIPTOR_WRITE_USER_DATA).cast::<u8>(),
            0,
            DESCRIPTOR_WRITE_USER_DATA_LEN,
        );
        READ_STDIN_ERRORS.store(0, Ordering::Relaxed);
        READ_STDIN_DUP_STDIN_OBSERVED.store(0, Ordering::Relaxed);
        READ_STDIN_GUARD_OBSERVED.store(0, Ordering::Relaxed);
        READ_STDIN_RESERVED_OBSERVED.store(0, Ordering::Relaxed);
        READ_STDIN_FD1_OBSERVED.store(0, Ordering::Relaxed);
        READ_STDIN_BADFD_OBSERVED.store(0, Ordering::Relaxed);
        READ_STDIN_FIRST_OBSERVED.store(0, Ordering::Relaxed);
        READ_STDIN_DUPLICATE_REMAINING_OBSERVED.store(0, Ordering::Relaxed);
        READ_STDIN_EOF_OBSERVED.store(0, Ordering::Relaxed);
        READ_STDIN_TALOS_NOP_OBSERVED.store(0, Ordering::Relaxed);
        READ_STDIN_UNKNOWN_OBSERVED.store(0, Ordering::Relaxed);
        READ_STDIN_COPY_PROBE_OBSERVED.store(0, Ordering::Relaxed);
        install_el0_trap_smoke_tables();
        enable_el2_and_el0_translation();
        enable_el1_and_el0_translation();
        aarch64::enter_el1_then_el0(
            entry as usize,
            user_sp as usize,
            EL0_TRAP_SPSR_EL0T_DAIF_MASKED,
            EL0_TRAP_SPSR_EL1H_DAIF_MASKED,
        );
    }
}

#[cfg(talos_boot_scenario = "qemu_readonly_initramfs_vfs_smoke")]
pub fn run_readonly_initramfs_vfs_smoke() -> bool {
    crate::println!("qemu-readonly-initramfs-vfs-smoke: start");

    let fs = phase8_readonly_initramfs_fixture();
    let digest = readonly_initramfs_vfs_manifest_digest();
    crate::println!(
        "qemu-readonly-initramfs-vfs-smoke: fixture name={} digest-algorithm=stable-manifest digest={:#x}",
        PHASE8_FIXTURE_NAME,
        digest
    );

    let root_ok = readonly_initramfs_vfs_report_lookup(fs, b"/", VfsNodeKind::Directory, 4);
    let banner_lookup_ok = readonly_initramfs_vfs_report_lookup(
        fs,
        PHASE8_BANNER_PATH,
        VfsNodeKind::RegularFile,
        PHASE8_BANNER_BYTES.len(),
    );
    let banner_read_ok = readonly_initramfs_vfs_report_banner_read(fs);
    let empty_read_ok = readonly_initramfs_vfs_report_empty_read(fs);
    let nested_lookup_ok =
        readonly_initramfs_vfs_report_lookup(fs, PHASE8_NESTED_PATH, VfsNodeKind::RegularFile, 15);
    let lookup_error_ok = readonly_initramfs_vfs_report_lookup_errors(fs);
    let descriptor_error_ok = readonly_initramfs_vfs_report_descriptor_errors(fs);
    let unsupported_ok = readonly_initramfs_vfs_report_unsupported_error(fs);

    let participants = u64::from(root_ok)
        + u64::from(banner_lookup_ok)
        + u64::from(banner_read_ok)
        + u64::from(empty_read_ok)
        + u64::from(nested_lookup_ok)
        + u64::from(lookup_error_ok)
        + u64::from(descriptor_error_ok)
        + u64::from(unsupported_ok);
    let errors = 8 - participants;
    let classification = if participants == 8 && errors == 0 {
        "qemu-readonly-initramfs-vfs-smoke-complete"
    } else {
        "qemu-readonly-initramfs-vfs-smoke-failed"
    };

    crate::println!(
        "qemu-readonly-initramfs-vfs-smoke: final participants={} expected=8 errors={} classification={}",
        participants,
        errors,
        classification
    );
    if participants == 8 && errors == 0 {
        crate::println!("qemu-readonly-initramfs-vfs-smoke: PASS");
        true
    } else {
        false
    }
}

#[cfg(talos_boot_scenario = "qemu_readonly_initramfs_vfs_smoke")]
fn readonly_initramfs_vfs_report_lookup(
    fs: crate::initramfs::ReadOnlyInitramfs,
    path: &[u8],
    expected_kind: VfsNodeKind,
    expected_len: usize,
) -> bool {
    let result = fs.lookup_default(path);
    let ok = match result {
        Ok(handle) => {
            handle.metadata().kind() == expected_kind && handle.metadata().len() == expected_len
        }
        Err(_) => false,
    };
    crate::println!(
        "qemu-readonly-initramfs-vfs-smoke: lookup path={} kind={} {}={} ok={}",
        readonly_initramfs_vfs_path_name(path),
        expected_kind.name(),
        if expected_kind == VfsNodeKind::Directory {
            "entries"
        } else {
            "length"
        },
        expected_len,
        ok
    );
    ok
}

#[cfg(talos_boot_scenario = "qemu_readonly_initramfs_vfs_smoke")]
fn readonly_initramfs_vfs_report_banner_read(fs: crate::initramfs::ReadOnlyInitramfs) -> bool {
    let mut description = fs
        .open_regular_file(PHASE8_BANNER_PATH)
        .expect("phase8 banner fixture opens");
    let mut user_memory = [0u8; 64];
    let mappings = [UserMapping::new(
        READONLY_INITRAMFS_VFS_USER_BASE,
        user_memory.len(),
        UserMappingPermissions::USER_DATA,
    )
    .expect("valid readonly initramfs smoke user mapping")];
    let mut scratch = [0u8; 64];
    let offset_before = description.offset();
    let first = fs.read_regular_file(
        &mut description,
        &mappings,
        READONLY_INITRAMFS_VFS_USER_BASE,
        &mut user_memory,
        READONLY_INITRAMFS_VFS_USER_BASE,
        64,
        &mut scratch,
    );
    let first_ok = first == Ok(PHASE8_BANNER_BYTES.len())
        && description.offset() == PHASE8_BANNER_BYTES.len()
        && &user_memory[..PHASE8_BANNER_BYTES.len()] == PHASE8_BANNER_BYTES;
    crate::println!(
        "qemu-readonly-initramfs-vfs-smoke: read path=/etc/banner.txt offset-before={} request=64 result={} offset-after={} data=\"Talos initramfs fixture\\n\" ok={}",
        offset_before,
        first.unwrap_or(usize::MAX),
        description.offset(),
        first_ok
    );

    let offset_before = description.offset();
    let eof = fs.read_regular_file(
        &mut description,
        &mappings,
        READONLY_INITRAMFS_VFS_USER_BASE,
        &mut user_memory,
        READONLY_INITRAMFS_VFS_USER_BASE,
        64,
        &mut scratch,
    );
    let eof_ok = eof == Ok(0) && description.offset() == PHASE8_BANNER_BYTES.len();
    crate::println!(
        "qemu-readonly-initramfs-vfs-smoke: read path=/etc/banner.txt offset-before={} request=64 result={} offset-after={} eof=true ok={}",
        offset_before,
        eof.unwrap_or(usize::MAX),
        description.offset(),
        eof_ok
    );

    first_ok && eof_ok
}

#[cfg(talos_boot_scenario = "qemu_readonly_initramfs_vfs_smoke")]
fn readonly_initramfs_vfs_report_empty_read(fs: crate::initramfs::ReadOnlyInitramfs) -> bool {
    let mut description = fs
        .open_regular_file(PHASE8_EMPTY_PATH)
        .expect("phase8 empty fixture opens");
    let mut user_memory = [0u8; 64];
    let mappings = [UserMapping::new(
        READONLY_INITRAMFS_VFS_USER_BASE,
        user_memory.len(),
        UserMappingPermissions::USER_DATA,
    )
    .expect("valid readonly initramfs smoke user mapping")];
    let mut scratch = [0u8; 64];
    let offset_before = description.offset();
    let result = fs.read_regular_file(
        &mut description,
        &mappings,
        READONLY_INITRAMFS_VFS_USER_BASE,
        &mut user_memory,
        READONLY_INITRAMFS_VFS_USER_BASE,
        64,
        &mut scratch,
    );
    let ok = result == Ok(0) && description.offset() == 0;
    crate::println!(
        "qemu-readonly-initramfs-vfs-smoke: read path=/empty offset-before={} request=64 result={} offset-after={} eof=true ok={}",
        offset_before,
        result.unwrap_or(usize::MAX),
        description.offset(),
        ok
    );
    ok
}

#[cfg(talos_boot_scenario = "qemu_readonly_initramfs_vfs_smoke")]
fn readonly_initramfs_vfs_report_lookup_errors(fs: crate::initramfs::ReadOnlyInitramfs) -> bool {
    let missing = readonly_initramfs_vfs_report_error(
        "missing path=/missing",
        fs.lookup_default(b"/missing"),
        PosixError::NoEntry,
        None,
    );
    let not_directory = readonly_initramfs_vfs_report_error(
        "not-directory path=/etc/banner.txt/child",
        fs.lookup_default(b"/etc/banner.txt/child"),
        PosixError::NotDirectory,
        None,
    );
    let is_directory = readonly_initramfs_vfs_report_error(
        "is-directory path=/etc",
        fs.open_regular_file(b"/etc"),
        PosixError::IsDirectory,
        None,
    );
    let name_too_long = readonly_initramfs_vfs_report_error(
        "name-too-long",
        fs.lookup(b"/abcde", PathLimits::new(4, 8, 4)),
        PosixError::NameTooLong,
        None,
    );

    missing && not_directory && is_directory && name_too_long
}

#[cfg(talos_boot_scenario = "qemu_readonly_initramfs_vfs_smoke")]
fn readonly_initramfs_vfs_report_descriptor_errors(
    fs: crate::initramfs::ReadOnlyInitramfs,
) -> bool {
    let mut description = fs
        .open_regular_file(PHASE8_BANNER_PATH)
        .expect("phase8 banner fixture opens");
    let initial_offset = description.offset();
    let mut descriptions = ReadOnlyFileDescriptions::<1>::new_empty();
    descriptions
        .insert(0, description)
        .expect("fixture file description inserts");
    let descriptor_table = DescriptorTable::<1>::new_empty();
    let mut user_memory = [0u8; 64];
    let mappings = [UserMapping::new(
        READONLY_INITRAMFS_VFS_USER_BASE,
        user_memory.len(),
        UserMappingPermissions::USER_DATA,
    )
    .expect("valid readonly initramfs smoke user mapping")];
    let mut scratch = [0u8; 64];
    let bad_descriptor_result = fs.read_descriptor(
        &descriptor_table,
        &mut descriptions,
        0,
        &mappings,
        READONLY_INITRAMFS_VFS_USER_BASE,
        &mut user_memory,
        READONLY_INITRAMFS_VFS_USER_BASE,
        8,
        &mut scratch,
    );
    let offset_after_bad_descriptor = descriptions
        .get_mut(0)
        .expect("description present")
        .offset();
    let bad_descriptor = readonly_initramfs_vfs_report_error(
        "bad-descriptor",
        bad_descriptor_result,
        PosixError::BadDescriptor,
        Some(offset_after_bad_descriptor == initial_offset),
    );

    let mut descriptor_table = DescriptorTable::<1>::new_empty();
    descriptor_table
        .allocate_at(
            0,
            DescriptorEntry::new(
                DescriptorAccess::ReadOnly,
                DescriptorFlags::EMPTY,
                DescriptorObject::new(DescriptorObjectKind::RegularFile, 0),
            ),
        )
        .expect("fixture regular descriptor allocates");
    let fault_result = fs.read_descriptor(
        &descriptor_table,
        &mut descriptions,
        0,
        &mappings,
        READONLY_INITRAMFS_VFS_USER_BASE,
        &mut user_memory,
        READONLY_INITRAMFS_VFS_USER_BASE + 128,
        8,
        &mut scratch,
    );
    let offset_after_fault = descriptions
        .get_mut(0)
        .expect("description present")
        .offset();
    let user_fault = readonly_initramfs_vfs_report_error(
        "user-fault",
        fault_result,
        PosixError::Fault,
        Some(offset_after_fault == initial_offset),
    );

    description = fs
        .open_regular_file(PHASE8_BANNER_PATH)
        .expect("phase8 banner fixture opens");
    let mut short_scratch = [0u8; 8];
    let invalid_result = fs.read_regular_file(
        &mut description,
        &mappings,
        READONLY_INITRAMFS_VFS_USER_BASE,
        &mut user_memory,
        READONLY_INITRAMFS_VFS_USER_BASE,
        9,
        &mut short_scratch,
    );
    let invalid_input = readonly_initramfs_vfs_report_error(
        "invalid-input",
        invalid_result,
        PosixError::InvalidArgument,
        Some(description.offset() == 0),
    );

    bad_descriptor && user_fault && invalid_input
}

#[cfg(talos_boot_scenario = "qemu_readonly_initramfs_vfs_smoke")]
fn readonly_initramfs_vfs_report_unsupported_error(
    fs: crate::initramfs::ReadOnlyInitramfs,
) -> bool {
    readonly_initramfs_vfs_report_error(
        "unsupported-operation",
        fs.unsupported_operation(),
        PosixError::NotSupported,
        None,
    )
}

#[cfg(talos_boot_scenario = "qemu_readonly_initramfs_vfs_smoke")]
fn readonly_initramfs_vfs_report_error<T>(
    case: &str,
    result: Result<T, PosixError>,
    expected: PosixError,
    offset_unchanged: Option<bool>,
) -> bool {
    let ok = matches!(result, Err(error) if error == expected) && offset_unchanged.unwrap_or(true);
    match offset_unchanged {
        Some(unchanged) => {
            crate::println!(
                "qemu-readonly-initramfs-vfs-smoke: error case={} errno=-{} offset-unchanged={} ok={}",
                case,
                expected.name(),
                unchanged,
                ok
            );
        }
        None => {
            crate::println!(
                "qemu-readonly-initramfs-vfs-smoke: error case={} errno=-{} ok={}",
                case,
                expected.name(),
                ok
            );
        }
    }
    ok
}

#[cfg(talos_boot_scenario = "qemu_readonly_initramfs_vfs_smoke")]
fn readonly_initramfs_vfs_path_name(path: &[u8]) -> &'static str {
    match path {
        b"/" => "/",
        b"/etc/banner.txt" => "/etc/banner.txt",
        b"/dir/nested.txt" => "/dir/nested.txt",
        _ => "<unknown>",
    }
}

#[cfg(talos_boot_scenario = "qemu_readonly_initramfs_vfs_smoke")]
fn readonly_initramfs_vfs_manifest_digest() -> u64 {
    const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
    const MANIFEST: &[&[u8]] = &[
        b"name=phase8-readonly-initramfs-vfs-v1\n",
        b"/=dir:etc,bin,empty,dir\n",
        b"/etc=dir:banner.txt\n",
        b"/etc/banner.txt=file:",
        PHASE8_BANNER_BYTES,
        b"/bin=dir:init\n",
        b"/bin/init=file:",
        PHASE8_INIT_BYTES,
        b"\n",
        b"/empty=file:\n",
        b"/dir=dir:nested.txt\n",
        b"/dir/nested.txt=file:nested fixture\n",
    ];
    let mut hash = FNV_OFFSET;
    let mut chunk_index = 0;
    while chunk_index < MANIFEST.len() {
        let chunk = MANIFEST[chunk_index];
        let mut byte_index = 0;
        while byte_index < chunk.len() {
            hash ^= chunk[byte_index] as u64;
            hash = hash.wrapping_mul(FNV_PRIME);
            byte_index += 1;
        }
        chunk_index += 1;
    }
    hash
}

#[cfg(talos_boot_scenario = "qemu_program_loader_smoke")]
const PROGRAM_LOADER_PHDR0: usize = 64;
#[cfg(talos_boot_scenario = "qemu_program_loader_smoke")]
const PROGRAM_LOADER_PHDR1: usize = 120;
#[cfg(talos_boot_scenario = "qemu_program_loader_smoke")]
const PROGRAM_LOADER_PT_INTERP: u32 = 3;
#[cfg(talos_boot_scenario = "qemu_program_loader_smoke")]
const PROGRAM_LOADER_PF_RWX: u32 = 0x7;

#[cfg(talos_boot_scenario = "qemu_program_loader_smoke")]
pub fn run_program_loader_smoke() -> bool {
    crate::println!("qemu-program-loader-smoke: start");

    let success_ok = program_loader_report_success();
    let bad_magic_ok = program_loader_report_error(
        "bad-magic",
        program_loader_mutated_result(|bytes| bytes[0] = 0),
        PosixError::NotExecutable,
    );
    let dynamic_interpreter_ok = program_loader_report_error(
        "dynamic-interpreter",
        program_loader_mutated_result(|bytes| {
            program_loader_write_u32(bytes, PROGRAM_LOADER_PHDR1, PROGRAM_LOADER_PT_INTERP);
        }),
        PosixError::NotSupported,
    );
    let wx_segment_ok = program_loader_report_error(
        "wx-segment",
        program_loader_mutated_result(|bytes| {
            program_loader_write_u32(bytes, PROGRAM_LOADER_PHDR0 + 4, PROGRAM_LOADER_PF_RWX);
        }),
        PosixError::AccessDenied,
    );
    let out_of_user_range_ok = program_loader_report_error(
        "out-of-user-range",
        program_loader_mutated_result(|bytes| {
            program_loader_write_u64(bytes, PROGRAM_LOADER_PHDR0 + 8, 0);
            program_loader_write_u64(bytes, PROGRAM_LOADER_PHDR0 + 16, 0);
        }),
        PosixError::AccessDenied,
    );
    let overlap_ok = program_loader_report_error(
        "overlap",
        program_loader_mutated_result(|bytes| {
            program_loader_write_u64(bytes, PROGRAM_LOADER_PHDR1 + 16, 0x0000_0000_0001_0200);
        }),
        PosixError::AccessDenied,
    );
    let bad_entry_ok = program_loader_report_error(
        "bad-entry",
        program_loader_mutated_result(|bytes| {
            program_loader_write_u64(bytes, 24, 0x0000_0000_0002_0200);
        }),
        PosixError::NotExecutable,
    );
    let file_range_overflow_ok = program_loader_report_error(
        "file-range-overflow",
        program_loader_mutated_result(|bytes| {
            program_loader_write_u64(bytes, PROGRAM_LOADER_PHDR1 + 32, 8);
        }),
        PosixError::NotExecutable,
    );

    let participants = u64::from(success_ok)
        + u64::from(bad_magic_ok)
        + u64::from(dynamic_interpreter_ok)
        + u64::from(wx_segment_ok)
        + u64::from(out_of_user_range_ok)
        + u64::from(overlap_ok)
        + u64::from(bad_entry_ok)
        + u64::from(file_range_overflow_ok);
    let errors = 8 - participants;
    let classification = if participants == 8 && errors == 0 {
        "qemu-program-loader-smoke-complete"
    } else {
        "qemu-program-loader-smoke-failed"
    };

    crate::println!(
        "qemu-program-loader-smoke: final participants={} expected=8 errors={} classification={}",
        participants,
        errors,
        classification
    );
    if participants == 8 && errors == 0 {
        crate::println!("qemu-program-loader-smoke: PASS");
        true
    } else {
        false
    }
}

#[cfg(talos_boot_scenario = "qemu_program_loader_smoke")]
fn program_loader_report_success() -> bool {
    let result = plan_phase8_init_image(phase8_readonly_initramfs_fixture());
    let Ok(plan) = result else {
        crate::println!(
            "qemu-program-loader-smoke: success format=elf64-aarch64-static-et-exec type=ET_EXEC machine=EM_AARCH64 phdrs=2 loadable=0 dynamic=false relocations=false ok=false"
        );
        return false;
    };

    crate::println!(
        "qemu-program-loader-smoke: fixture name={} path=/bin/init digest-algorithm=stable-elf-manifest digest={:#x}",
        PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY,
        plan.source_digest()
    );

    let text = plan.segment(0);
    let data = plan.segment(1);
    let success_ok = plan.fixture_identity() == PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY
        && plan.source_path() == PHASE8_INIT_PATH
        && plan.source_len() == PHASE8_INIT_BYTES.len()
        && plan.segment_count() == 2
        && matches!(text, Some(segment) if program_loader_segment_ok(segment, UserSegmentKind::UserText, UserMappingPermissions::USER_TEXT, false))
        && matches!(data, Some(segment) if program_loader_segment_ok(segment, UserSegmentKind::UserData, UserMappingPermissions::USER_DATA, true))
        && program_loader_entry_ok(&plan);

    crate::println!(
        "qemu-program-loader-smoke: success format=elf64-aarch64-static-et-exec type=ET_EXEC machine=EM_AARCH64 phdrs=2 loadable={} dynamic=false relocations=false ok={}",
        plan.segment_count(),
        success_ok
    );

    if let Some(segment) = text {
        program_loader_report_segment(0, segment);
    } else {
        crate::println!(
            "qemu-program-loader-smoke: segment index=0 kind=UserText flags=R-X file-bytes=0x0 mem-bytes=0x0 zero-fill=0x0 wx=false ok=false"
        );
    }
    if let Some(segment) = data {
        program_loader_report_segment(1, segment);
    } else {
        crate::println!(
            "qemu-program-loader-smoke: segment index=1 kind=UserData flags=RW- file-bytes=0x0 mem-bytes=0x0 zero-fill=0x0 wx=false ok=false"
        );
    }

    let entry_ok = program_loader_entry_ok(&plan);
    crate::println!(
        "qemu-program-loader-smoke: entry va={:#x} in-user={} in-text={} aligned={} ok={}",
        plan.entry(),
        plan.entry() >= crate::posix::USER_NULL_GUARD_END
            && plan.entry() < crate::posix::USER_ADDRESS_SPACE_END,
        entry_ok,
        plan.entry() & 0x3 == 0,
        entry_ok
    );
    crate::println!(
        "qemu-program-loader-smoke: image-plan source=/bin/init output=image-plan-only process-created=false stack-built=false descriptors-installed=false ok={}",
        success_ok
    );

    success_ok
}

#[cfg(talos_boot_scenario = "qemu_program_loader_smoke")]
fn program_loader_segment_ok(
    segment: PlannedUserSegment,
    kind: UserSegmentKind,
    permissions: UserMappingPermissions,
    requires_zero_fill: bool,
) -> bool {
    let wx = segment.permissions().allows(UserAccessKind::Write)
        && segment.permissions().allows(UserAccessKind::Execute);
    segment.kind() == kind
        && segment.permissions() == permissions
        && segment.file_size() != 0
        && (!requires_zero_fill || segment.zero_fill_len() != 0)
        && !wx
}

#[cfg(talos_boot_scenario = "qemu_program_loader_smoke")]
fn program_loader_report_segment(index: usize, segment: PlannedUserSegment) {
    let flags = program_loader_segment_flags(segment);
    let mem_bytes = segment.virtual_end() - segment.virtual_start();
    let wx = segment.permissions().allows(UserAccessKind::Write)
        && segment.permissions().allows(UserAccessKind::Execute);
    let ok = match index {
        0 => program_loader_segment_ok(
            segment,
            UserSegmentKind::UserText,
            UserMappingPermissions::USER_TEXT,
            false,
        ),
        1 => program_loader_segment_ok(
            segment,
            UserSegmentKind::UserData,
            UserMappingPermissions::USER_DATA,
            true,
        ),
        _ => false,
    } && !wx;

    crate::println!(
        "qemu-program-loader-smoke: segment index={} kind={} flags={} file-bytes={:#x} mem-bytes={:#x} zero-fill={:#x} wx={} ok={}",
        index,
        segment.kind().name(),
        flags,
        segment.file_size(),
        mem_bytes,
        segment.zero_fill_len(),
        wx,
        ok
    );
}

#[cfg(talos_boot_scenario = "qemu_program_loader_smoke")]
fn program_loader_segment_flags(segment: PlannedUserSegment) -> &'static str {
    match (
        segment.permissions().allows(UserAccessKind::Read),
        segment.permissions().allows(UserAccessKind::Write),
        segment.permissions().allows(UserAccessKind::Execute),
    ) {
        (true, false, true) => "R-X",
        (true, true, false) => "RW-",
        (true, false, false) => "R--",
        _ => "---",
    }
}

#[cfg(talos_boot_scenario = "qemu_program_loader_smoke")]
fn program_loader_entry_ok(plan: &ProgramImagePlan) -> bool {
    let mut index = 0;
    while index < plan.segment_count() {
        if let Some(segment) = plan.segment(index) {
            if segment.kind() == UserSegmentKind::UserText
                && segment.virtual_start() <= plan.entry()
                && plan.entry() < segment.virtual_end()
                && plan.entry() & 0x3 == 0
            {
                return true;
            }
        }
        index += 1;
    }
    false
}

#[cfg(talos_boot_scenario = "qemu_program_loader_smoke")]
fn program_loader_report_error(
    case: &str,
    result: Result<ProgramImagePlan, ProgramLoaderError>,
    expected: PosixError,
) -> bool {
    let ok = matches!(result, Err(error) if error.posix_error() == expected);
    crate::println!(
        "qemu-program-loader-smoke: error case={} errno=-{} partial-install=false ok={}",
        case,
        expected.name(),
        ok
    );
    ok
}

#[cfg(talos_boot_scenario = "qemu_program_loader_smoke")]
fn program_loader_mutated_result(
    mutate: impl FnOnce(&mut [u8; PHASE8_INIT_ELF_LEN]),
) -> Result<ProgramImagePlan, ProgramLoaderError> {
    let mut bytes = [0; PHASE8_INIT_ELF_LEN];
    bytes.copy_from_slice(PHASE8_INIT_BYTES);
    mutate(&mut bytes);
    plan_elf64_aarch64_image(
        PHASE8_INIT_PATH,
        PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY,
        &bytes,
    )
}

#[cfg(talos_boot_scenario = "qemu_program_loader_smoke")]
fn program_loader_write_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

#[cfg(talos_boot_scenario = "qemu_program_loader_smoke")]
fn program_loader_write_u64(bytes: &mut [u8], offset: usize, value: u64) {
    bytes[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
}

#[cfg(talos_boot_scenario = "qemu_process_install_smoke")]
pub fn run_process_install_smoke() -> bool {
    crate::println!("qemu-process-install-smoke: start");

    let success_ok = process_install_report_success();
    let side_effects_ok = process_install_report_side_effects(success_ok);
    let bad_plan_ok = process_install_report_error(
        "bad-plan-invariant",
        process_install_bad_plan_invariant(),
        PosixError::InvalidArgument,
    );
    let overlap_ok = process_install_report_error(
        "overlap",
        process_install_overlap_plan(),
        PosixError::AccessDenied,
    );
    let permission_widening_ok = process_install_report_error(
        "permission-widening",
        process_install_permission_widening_plan(),
        PosixError::AccessDenied,
    );
    let bad_entry_ok = process_install_report_error(
        "bad-entry",
        process_install_bad_entry_plan(),
        PosixError::NotExecutable,
    );
    let budget_overflow_ok = process_install_report_error(
        "budget-overflow",
        process_install_budget_overflow_plan(),
        PosixError::NoMemory,
    );

    let participants = u64::from(success_ok)
        + u64::from(bad_plan_ok)
        + u64::from(overlap_ok)
        + u64::from(permission_widening_ok)
        + u64::from(bad_entry_ok)
        + u64::from(budget_overflow_ok)
        + u64::from(side_effects_ok);
    let errors = 7 - participants;
    let classification = if participants == 7 && errors == 0 {
        "qemu-process-install-smoke-complete"
    } else {
        "qemu-process-install-smoke-failed"
    };

    crate::println!(
        "qemu-process-install-smoke: final participants={} expected=7 errors={} classification={}",
        participants,
        errors,
        classification
    );
    if participants == 7 && errors == 0 {
        crate::println!("qemu-process-install-smoke: PASS");
        true
    } else {
        crate::println!("qemu-process-install-smoke: FAIL");
        false
    }
}

#[cfg(talos_boot_scenario = "qemu_process_install_smoke")]
fn process_install_report_success() -> bool {
    let result = plan_phase8_init_image(phase8_readonly_initramfs_fixture())
        .map_err(|error| error.posix_error())
        .and_then(plan_process_image_install);
    let Ok(plan) = result else {
        crate::println!(
            "qemu-process-install-smoke: success output=ProcessImageInstallPlan metadata-only=true entry=0x0 entry-preserved=false footprint=0x0 pages=0 ok=false"
        );
        return false;
    };

    crate::println!(
        "qemu-process-install-smoke: fixture name={} path=/bin/init source-digest={:#x} install-boundary={}",
        PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY,
        plan.source_digest(),
        PROCESS_INSTALL_BOUNDARY_IDENTITY
    );

    let expected_image = plan_phase8_init_image(phase8_readonly_initramfs_fixture());
    let entry_preserved = matches!(expected_image, Ok(image) if image.entry() == plan.entry());
    let success_ok = plan.fixture_identity() == PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY
        && plan.install_boundary_identity() == PROCESS_INSTALL_BOUNDARY_IDENTITY
        && plan.source_path() == PHASE8_INIT_PATH
        && entry_preserved
        && plan.memory_footprint() == 0x3000
        && plan.page_count() == 3
        && plan.lower_el_launch_blocked()
        && process_install_page_ok(plan.page(0), UserSegmentKind::UserText)
        && process_install_page_ok(plan.page(1), UserSegmentKind::UserData);

    crate::println!(
        "qemu-process-install-smoke: success output=ProcessImageInstallPlan metadata-only=true entry={:#x} entry-preserved={} footprint={:#x} pages={} ok={}",
        plan.entry(),
        entry_preserved,
        plan.memory_footprint(),
        plan.page_count(),
        success_ok
    );

    process_install_report_page(0, plan.page(0));
    process_install_report_page(1, plan.page(1));

    success_ok
}

#[cfg(talos_boot_scenario = "qemu_process_install_smoke")]
fn process_install_report_side_effects(success_ok: bool) -> bool {
    let result = plan_phase8_init_image(phase8_readonly_initramfs_fixture())
        .map_err(|error| error.posix_error())
        .and_then(plan_process_image_install);
    let Ok(plan) = result else {
        crate::println!(
            "qemu-process-install-smoke: side-effects frames-allocated=0 mappings-installed=0 process-created=false descriptors-mutated=false lower-el-frame=false runnable=false ok=false"
        );
        return false;
    };
    let side_effects = plan.side_effects();
    let side_effects_ok = success_ok
        && side_effects.frames_allocated() == 0
        && side_effects.mappings_installed() == 0
        && !side_effects.process_created()
        && !side_effects.descriptors_mutated()
        && !side_effects.lower_el_frame()
        && !side_effects.runnable();
    crate::println!(
        "qemu-process-install-smoke: side-effects frames-allocated={} mappings-installed={} process-created={} descriptors-mutated={} lower-el-frame={} runnable={} ok={}",
        side_effects.frames_allocated(),
        side_effects.mappings_installed(),
        side_effects.process_created(),
        side_effects.descriptors_mutated(),
        side_effects.lower_el_frame(),
        side_effects.runnable(),
        side_effects_ok
    );
    side_effects_ok
}

#[cfg(talos_boot_scenario = "qemu_process_install_smoke")]
fn process_install_page_ok(
    page: Option<ProcessImagePageInstallRecord>,
    kind: UserSegmentKind,
) -> bool {
    let Some(page) = page else {
        return false;
    };
    let expected_flags = match kind {
        UserSegmentKind::UserText => "R-X",
        UserSegmentKind::UserData => "RW-",
    };
    page.kind() == kind
        && page.permission_flags() == expected_flags
        && page.copy_len() != 0
        && page.zero_len() != 0
        && page.action().name() == "allocate,copy,zero,map"
}

#[cfg(talos_boot_scenario = "qemu_process_install_smoke")]
fn process_install_report_page(index: usize, page: Option<ProcessImagePageInstallRecord>) {
    let Some(page) = page else {
        crate::println!(
            "qemu-process-install-smoke: page index={} kind=missing flags=--- copy-offset=0x0 copy-len=0x0 zero-offset=0x0 zero-len=0x0 action-order=allocate,copy,zero,map permission-widened=true ok=false",
            index
        );
        return;
    };
    let (zero_offset, zero_len) = if let Some(zero) = page.zero_range(0) {
        (zero.offset(), page.zero_len())
    } else {
        (0, 0)
    };
    let permission_widened = match page.kind() {
        UserSegmentKind::UserText => page.permissions() != UserMappingPermissions::USER_TEXT,
        UserSegmentKind::UserData => page.permissions() != UserMappingPermissions::USER_DATA,
    };
    let ok = process_install_page_ok(Some(page), page.kind()) && !permission_widened;
    crate::println!(
        "qemu-process-install-smoke: page index={} kind={} flags={} copy-offset={:#x} copy-len={:#x} zero-offset={:#x} zero-len={:#x} action-order={} permission-widened={} ok={}",
        index,
        page.kind().name(),
        page.permission_flags(),
        page.copy_page_offset(),
        page.copy_len(),
        zero_offset,
        zero_len,
        page.action().name(),
        permission_widened,
        ok
    );
}

#[cfg(talos_boot_scenario = "qemu_process_install_smoke")]
fn process_install_report_error(
    case: &str,
    result: Result<ProcessImageInstallPlan, PosixError>,
    expected: PosixError,
) -> bool {
    let ok = matches!(result, Err(error) if error == expected);
    crate::println!(
        "qemu-process-install-smoke: error case={} errno=-{} partial-install=false ok={}",
        case,
        expected.name(),
        ok
    );
    ok
}

#[cfg(talos_boot_scenario = "qemu_process_install_smoke")]
fn process_install_segment(
    kind: UserSegmentKind,
    permissions: UserMappingPermissions,
    virtual_start: u64,
    memory_size: u64,
    file_offset: usize,
    file_size: usize,
) -> PlannedUserSegment {
    let virtual_end = virtual_start + memory_size;
    PlannedUserSegment::for_test_unchecked(
        kind,
        permissions,
        virtual_start,
        virtual_end,
        virtual_start & !(LOADER_PAGE_SIZE - 1),
        (virtual_end + LOADER_PAGE_SIZE - 1) & !(LOADER_PAGE_SIZE - 1),
        file_offset,
        file_size,
        virtual_start + file_size as u64,
        virtual_end,
    )
}

#[cfg(talos_boot_scenario = "qemu_process_install_smoke")]
fn process_install_unchecked_plan(
    entry: u64,
    segment_count: usize,
    segments: [Option<PlannedUserSegment>; MAX_LOAD_SEGMENTS],
    memory_footprint: u64,
) -> ProgramImagePlan {
    ProgramImagePlan::for_test_unchecked(
        PHASE8_INIT_PATH,
        PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY,
        PHASE8_INIT_BYTES.len(),
        0x3892_eed2_2390_0c65,
        entry,
        segment_count,
        segments,
        0x0000_0000_0001_0000,
        0x0000_0000_0002_2000,
        memory_footprint,
    )
}

#[cfg(talos_boot_scenario = "qemu_process_install_smoke")]
fn process_install_bad_plan_invariant() -> Result<ProcessImageInstallPlan, PosixError> {
    plan_process_image_install(process_install_unchecked_plan(
        0x0000_0000_0001_0100,
        1,
        [None, None, None, None],
        LOADER_PAGE_SIZE,
    ))
}

#[cfg(talos_boot_scenario = "qemu_process_install_smoke")]
fn process_install_overlap_plan() -> Result<ProcessImageInstallPlan, PosixError> {
    let text = process_install_segment(
        UserSegmentKind::UserText,
        UserMappingPermissions::USER_TEXT,
        0x0000_0000_0001_0100,
        4,
        0x100,
        4,
    );
    let data = process_install_segment(
        UserSegmentKind::UserData,
        UserMappingPermissions::USER_DATA,
        0x0000_0000_0001_0200,
        4,
        0x200,
        4,
    );
    plan_process_image_install(process_install_unchecked_plan(
        0x0000_0000_0001_0100,
        2,
        [Some(text), Some(data), None, None],
        LOADER_PAGE_SIZE * 2,
    ))
}

#[cfg(talos_boot_scenario = "qemu_process_install_smoke")]
fn process_install_permission_widening_plan() -> Result<ProcessImageInstallPlan, PosixError> {
    let text = process_install_segment(
        UserSegmentKind::UserText,
        UserMappingPermissions::USER_DATA,
        0x0000_0000_0001_0100,
        4,
        0x100,
        4,
    );
    plan_process_image_install(process_install_unchecked_plan(
        0x0000_0000_0001_0100,
        1,
        [Some(text), None, None, None],
        LOADER_PAGE_SIZE,
    ))
}

#[cfg(talos_boot_scenario = "qemu_process_install_smoke")]
fn process_install_bad_entry_plan() -> Result<ProcessImageInstallPlan, PosixError> {
    let data = process_install_segment(
        UserSegmentKind::UserData,
        UserMappingPermissions::USER_DATA,
        0x0000_0000_0002_0200,
        4,
        0x200,
        4,
    );
    plan_process_image_install(process_install_unchecked_plan(
        0x0000_0000_0002_0200,
        1,
        [Some(data), None, None, None],
        LOADER_PAGE_SIZE,
    ))
}

#[cfg(talos_boot_scenario = "qemu_process_install_smoke")]
fn process_install_budget_overflow_plan() -> Result<ProcessImageInstallPlan, PosixError> {
    let text = process_install_segment(
        UserSegmentKind::UserText,
        UserMappingPermissions::USER_TEXT,
        0x0000_0000_0001_0000,
        MAX_PROCESS_INSTALL_FOOTPRINT + LOADER_PAGE_SIZE,
        0,
        4,
    );
    plan_process_image_install(process_install_unchecked_plan(
        0x0000_0000_0001_0000,
        1,
        [Some(text), None, None, None],
        MAX_PROCESS_INSTALL_FOOTPRINT + LOADER_PAGE_SIZE,
    ))
}

#[cfg(talos_boot_scenario = "qemu_process_address_space_smoke")]
pub fn run_process_address_space_smoke() -> bool {
    crate::println!("qemu-process-address-space-smoke: start");

    let success_ok = process_address_space_report_success();
    let teardown_ok = process_address_space_report_teardown();
    let bad_install_ok = process_address_space_report_error(
        "bad-install-plan",
        process_address_space_bad_install_plan(),
        PosixError::InvalidArgument,
    );
    let null_guard_ok = process_address_space_report_error(
        "null-guard-or-kernel-split",
        process_address_space_null_guard_plan(),
        PosixError::AccessDenied,
    );
    let overlap_ok = process_address_space_report_error(
        "overlap",
        process_address_space_overlap_plan(),
        PosixError::AccessDenied,
    );
    let permission_widening_ok = process_address_space_report_error(
        "permission-widening",
        process_address_space_permission_widening_plan(),
        PosixError::AccessDenied,
    );
    let lease_exhaustion_ok = process_address_space_report_lease_exhaustion();
    let copy_zero_ok = process_address_space_report_copy_zero_failure();

    let participants = u64::from(success_ok)
        + u64::from(teardown_ok)
        + u64::from(bad_install_ok)
        + u64::from(null_guard_ok)
        + u64::from(overlap_ok)
        + u64::from(permission_widening_ok)
        + u64::from(lease_exhaustion_ok)
        + u64::from(copy_zero_ok);
    let errors = 8 - participants;
    let classification = if participants == 8 && errors == 0 {
        "qemu-process-address-space-smoke-complete"
    } else {
        "qemu-process-address-space-smoke-failed"
    };

    crate::println!(
        "qemu-process-address-space-smoke: final participants={} expected=8 errors={} classification={}",
        participants,
        errors,
        classification
    );
    if participants == 8 && errors == 0 {
        crate::println!("qemu-process-address-space-smoke: PASS");
        true
    } else {
        crate::println!("qemu-process-address-space-smoke: FAIL");
        false
    }
}

#[cfg(talos_boot_scenario = "qemu_process_address_space_smoke")]
fn process_address_space_report_success() -> bool {
    let Ok(plan) = process_address_space_install_fixture() else {
        crate::println!(
            "qemu-process-address-space-smoke: success output=ProcessAddressSpace published=false id=0x0 owner=0x0 root-token=0x0 table-leases=0 user-frame-leases=0 mappings=0 ok=false"
        );
        return false;
    };

    crate::println!(
        "qemu-process-address-space-smoke: fixture name={} path=/bin/init source-digest={:#x} install-boundary={} address-space-boundary={}",
        PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY,
        plan.source_digest(),
        PROCESS_INSTALL_BOUNDARY_IDENTITY,
        PROCESS_ADDRESS_SPACE_BOUNDARY_IDENTITY
    );

    let mut lease_source = ProcessAddressSpaceLeaseSource::for_plan(plan);
    let id = process_address_space_id();
    let owner = process_address_space_owner_id();
    let Ok(address_space) = install_process_address_space(plan, id, Some(owner), &mut lease_source)
    else {
        crate::println!(
            "qemu-process-address-space-smoke: success output=ProcessAddressSpace published=false id={:#x} owner={:#x} root-token=0x0 table-leases=0 user-frame-leases=0 mappings=0 ok=false",
            id.raw(),
            owner.raw()
        );
        return false;
    };

    let success_ok = address_space.published()
        && address_space.id() == id
        && address_space.owner() == Some(owner)
        && address_space.boundary_identity() == PROCESS_ADDRESS_SPACE_BOUNDARY_IDENTITY
        && !address_space.destroyed()
        && address_space.table_lease_count() == 1
        && address_space.user_frame_lease_count() == plan.page_count()
        && address_space.mapping_count() == plan.page_count()
        && process_address_space_mapping_ok(
            address_space.mapping(0),
            address_space.user_frame_lease(0),
            UserSegmentKind::UserText,
        )
        && process_address_space_mapping_ok(
            address_space.mapping(1),
            address_space.user_frame_lease(1),
            UserSegmentKind::UserData,
        );

    crate::println!(
        "qemu-process-address-space-smoke: success output=ProcessAddressSpace published={} id={:#x} owner={:#x} root-token={:#x} table-leases={} user-frame-leases={} mappings={} ok={}",
        address_space.published(),
        address_space.id().raw(),
        owner.raw(),
        address_space.root().token().raw(),
        address_space.table_lease_count(),
        address_space.user_frame_lease_count(),
        address_space.mapping_count(),
        success_ok
    );

    process_address_space_report_mapping(
        0,
        address_space.mapping(0),
        address_space.user_frame_lease(0),
    );
    process_address_space_report_mapping(
        1,
        address_space.mapping(1),
        address_space.user_frame_lease(1),
    );
    if address_space.mapping_count() > 2 {
        process_address_space_report_mapping(
            2,
            address_space.mapping(2),
            address_space.user_frame_lease(2),
        );
    }

    let side_effects = address_space.side_effects();
    let snapshot = lease_source.snapshot();
    let side_effects_ok = success_ok
        && snapshot.root_leased
        && snapshot.table_pages_leased == address_space.table_lease_count()
        && snapshot.user_frames_leased == address_space.user_frame_lease_count()
        && snapshot.mappings_installed == address_space.mapping_count()
        && side_effects.copied_bytes() == 8
        && side_effects.zeroed_bytes() == 0x2ff8;
    crate::println!(
        "qemu-process-address-space-smoke: side-effects root-leased={} table-leases={} user-frame-leases={} mappings-installed={} copied-bytes={:#x} zeroed-bytes={:#x} scheduler-owner=false descriptors-mutated=false lower-el-frame=false runnable=false ok={}",
        snapshot.root_leased,
        snapshot.table_pages_leased,
        snapshot.user_frames_leased,
        snapshot.mappings_installed,
        side_effects.copied_bytes(),
        side_effects.zeroed_bytes(),
        side_effects_ok
    );

    side_effects_ok
}

#[cfg(talos_boot_scenario = "qemu_process_address_space_smoke")]
fn process_address_space_report_teardown() -> bool {
    let Ok(plan) = process_address_space_install_fixture() else {
        process_address_space_report_empty_teardown("first", false);
        process_address_space_report_empty_teardown("second", true);
        return false;
    };
    let mut lease_source = ProcessAddressSpaceLeaseSource::for_plan(plan);
    let Ok(mut address_space) = install_process_address_space(
        plan,
        process_address_space_id(),
        Some(process_address_space_owner_id()),
        &mut lease_source,
    ) else {
        process_address_space_report_empty_teardown("first", false);
        process_address_space_report_empty_teardown("second", true);
        return false;
    };

    let first = address_space.destroy(&mut lease_source);
    let first_ok = first.mappings_released() == plan.page_count()
        && first.user_frame_releases() == plan.page_count()
        && first.table_page_releases() == 1
        && first.root_released()
        && !first.already_destroyed()
        && lease_source.outstanding_leases() == 0;
    crate::println!(
        "qemu-process-address-space-smoke: teardown phase=first mappings-released={} user-frame-releases={} table-lease-releases={} root-released={} already-destroyed={} ok={}",
        first.mappings_released(),
        first.user_frame_releases(),
        first.table_page_releases(),
        first.root_released(),
        first.already_destroyed(),
        first_ok
    );

    let second = address_space.destroy(&mut lease_source);
    let second_ok = second.mappings_released() == 0
        && second.user_frame_releases() == 0
        && second.table_page_releases() == 0
        && !second.root_released()
        && second.already_destroyed()
        && lease_source.outstanding_leases() == 0;
    crate::println!(
        "qemu-process-address-space-smoke: teardown phase=second mappings-released={} user-frame-releases={} table-lease-releases={} root-released={} already-destroyed={} ok={}",
        second.mappings_released(),
        second.user_frame_releases(),
        second.table_page_releases(),
        second.root_released(),
        second.already_destroyed(),
        second_ok
    );

    first_ok && second_ok
}

#[cfg(talos_boot_scenario = "qemu_process_address_space_smoke")]
fn process_address_space_report_empty_teardown(phase: &str, already_destroyed: bool) {
    crate::println!(
        "qemu-process-address-space-smoke: teardown phase={} mappings-released=0 user-frame-releases=0 table-lease-releases=0 root-released=false already-destroyed={} ok=false",
        phase,
        already_destroyed
    );
}

#[cfg(talos_boot_scenario = "qemu_process_address_space_smoke")]
fn process_address_space_report_mapping(
    index: usize,
    mapping: Option<ProcessUserMapping>,
    lease: Option<UserFrameLease>,
) -> bool {
    let (Some(mapping), Some(lease)) = (mapping, lease) else {
        crate::println!(
            "qemu-process-address-space-smoke: mapping index={} kind=missing flags=--- copy-bytes=0x0 zero-bytes=0x0 zero-before-copy=false source-page=0 permission-widened=true ok=false",
            index
        );
        return false;
    };
    let permission_widened = match mapping.kind() {
        UserSegmentKind::UserText => mapping.permissions() != UserMappingPermissions::USER_TEXT,
        UserSegmentKind::UserData => mapping.permissions() != UserMappingPermissions::USER_DATA,
    };
    let ok = lease.zeroed_before_copy()
        && lease.kind() == mapping.kind()
        && lease.permissions() == mapping.permissions()
        && lease.copied_bytes() == mapping.copy_len()
        && lease.zeroed_bytes() == mapping.zero_len()
        && lease.source_page_ordinal() == mapping.source_page_ordinal()
        && mapping.el0_user_access()
        && mapping.write_xor_execute()
        && mapping.normal_memory_intent()
        && mapping.kernel_device_denied()
        && !permission_widened;
    crate::println!(
        "qemu-process-address-space-smoke: mapping index={} kind={} flags={} copy-bytes={:#x} zero-bytes={:#x} zero-before-copy={} source-page={} permission-widened={} ok={}",
        index,
        mapping.kind().name(),
        process_address_space_permission_flags(mapping.permissions()),
        mapping.copy_len(),
        mapping.zero_len(),
        lease.zeroed_before_copy(),
        mapping.source_page_ordinal(),
        permission_widened,
        ok
    );
    ok
}

#[cfg(talos_boot_scenario = "qemu_process_address_space_smoke")]
fn process_address_space_mapping_ok(
    mapping: Option<ProcessUserMapping>,
    lease: Option<UserFrameLease>,
    kind: UserSegmentKind,
) -> bool {
    let (Some(mapping), Some(lease)) = (mapping, lease) else {
        return false;
    };
    mapping.kind() == kind
        && lease.kind() == kind
        && lease.zeroed_before_copy()
        && process_address_space_permission_flags(mapping.permissions())
            == process_address_space_expected_flags(kind)
        && mapping.write_xor_execute()
        && mapping.el0_user_access()
        && mapping.normal_memory_intent()
        && mapping.kernel_device_denied()
}

#[cfg(talos_boot_scenario = "qemu_process_address_space_smoke")]
fn process_address_space_permission_flags(permissions: UserMappingPermissions) -> &'static str {
    match permissions {
        UserMappingPermissions::USER_TEXT => "R-X",
        UserMappingPermissions::USER_DATA => "RW-",
        UserMappingPermissions::READ => "R--",
        _ => "---",
    }
}

#[cfg(talos_boot_scenario = "qemu_process_address_space_smoke")]
fn process_address_space_expected_flags(kind: UserSegmentKind) -> &'static str {
    match kind {
        UserSegmentKind::UserText => "R-X",
        UserSegmentKind::UserData => "RW-",
    }
}

#[cfg(talos_boot_scenario = "qemu_process_address_space_smoke")]
fn process_address_space_report_error(
    case: &str,
    plan: ProcessImageInstallPlan,
    expected: PosixError,
) -> bool {
    let mut lease_source = ProcessAddressSpaceLeaseSource::for_plan(plan);
    let result = install_process_address_space(
        plan,
        process_address_space_id(),
        Some(process_address_space_owner_id()),
        &mut lease_source,
    );
    let ok =
        matches!(result, Err(error) if error == expected) && lease_source.outstanding_leases() == 0;
    crate::println!(
        "qemu-process-address-space-smoke: error case={} errno=-{} partial-install=false leaked-leases={} ok={}",
        case,
        expected.name(),
        lease_source.outstanding_leases() != 0,
        ok
    );
    ok
}

#[cfg(talos_boot_scenario = "qemu_process_address_space_smoke")]
fn process_address_space_report_lease_exhaustion() -> bool {
    let Ok(plan) = process_address_space_install_fixture() else {
        crate::println!(
            "qemu-process-address-space-smoke: error case=lease-exhaustion errno=-ENOMEM partial-install=false leaked-leases=false ok=false"
        );
        return false;
    };
    let mut lease_source =
        ProcessAddressSpaceLeaseSource::with_limits(1, 1, plan.page_count(), plan.page_count());
    let result = install_process_address_space(
        plan,
        process_address_space_id(),
        Some(process_address_space_owner_id()),
        &mut lease_source,
    );
    let ok = matches!(result, Err(PosixError::NoMemory)) && lease_source.outstanding_leases() == 0;
    crate::println!(
        "qemu-process-address-space-smoke: error case=lease-exhaustion errno=-ENOMEM partial-install=false leaked-leases={} ok={}",
        lease_source.outstanding_leases() != 0,
        ok
    );
    ok
}

#[cfg(talos_boot_scenario = "qemu_process_address_space_smoke")]
fn process_address_space_report_copy_zero_failure() -> bool {
    let Ok(plan) = process_address_space_install_fixture() else {
        crate::println!(
            "qemu-process-address-space-smoke: error case=copy-zero-model-failure errno=-EINVAL partial-install=false leaked-leases=false ok=false"
        );
        return false;
    };
    let mut lease_source = ProcessAddressSpaceLeaseSource::for_plan(plan);
    lease_source.fail_copy_zero_at_page(1);
    let result = install_process_address_space(
        plan,
        process_address_space_id(),
        Some(process_address_space_owner_id()),
        &mut lease_source,
    );
    let ok = matches!(result, Err(PosixError::InvalidArgument))
        && lease_source.outstanding_leases() == 0;
    crate::println!(
        "qemu-process-address-space-smoke: error case=copy-zero-model-failure errno=-EINVAL partial-install=false leaked-leases={} ok={}",
        lease_source.outstanding_leases() != 0,
        ok
    );
    ok
}

#[cfg(talos_boot_scenario = "qemu_process_address_space_smoke")]
fn process_address_space_install_fixture() -> Result<ProcessImageInstallPlan, PosixError> {
    plan_phase8_init_image(phase8_readonly_initramfs_fixture())
        .map_err(|error| error.posix_error())
        .and_then(plan_process_image_install)
}

#[cfg(talos_boot_scenario = "qemu_process_address_space_smoke")]
fn process_address_space_id() -> ProcessAddressSpaceId {
    ProcessAddressSpaceId::new(0x8300_0001).expect("nonzero address-space id")
}

#[cfg(talos_boot_scenario = "qemu_process_address_space_smoke")]
fn process_address_space_owner_id() -> ProcessOwnerId {
    ProcessOwnerId::new(0x8300_1001).expect("nonzero process owner id")
}

#[cfg(talos_boot_scenario = "qemu_process_address_space_smoke")]
fn process_address_space_bad_install_plan() -> ProcessImageInstallPlan {
    let fixture = process_address_space_install_fixture().expect("fixture install plan");
    let mut pages = [None; MAX_PROCESS_INSTALL_PAGES];
    pages[0] = fixture.page(0);
    ProcessImageInstallPlan::for_test_unchecked(
        PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY,
        PROCESS_INSTALL_BOUNDARY_IDENTITY,
        PHASE8_INIT_PATH,
        fixture.source_digest(),
        fixture.entry(),
        LOADER_PAGE_SIZE * 2,
        2,
        pages,
        ProcessInstallSideEffects::NONE,
        true,
    )
}

#[cfg(talos_boot_scenario = "qemu_process_address_space_smoke")]
fn process_address_space_null_guard_plan() -> ProcessImageInstallPlan {
    let fixture = process_address_space_install_fixture().expect("fixture install plan");
    let page = ProcessImagePageInstallRecord::for_test_unchecked(
        0,
        0,
        0,
        LOADER_PAGE_SIZE,
        UserSegmentKind::UserText,
        UserMappingPermissions::USER_TEXT,
        0,
        0,
        4,
        [Some(PageByteRange::for_test_unchecked(4, LOADER_PAGE_SIZE - 4));
            MAX_ZERO_RANGES_PER_PAGE],
        1,
        LOADER_PAGE_SIZE - 4,
        ProcessInstallAction::AllocateCopyZeroMap,
    );
    process_address_space_plan_from_pages(
        fixture,
        fixture.entry(),
        LOADER_PAGE_SIZE,
        [
            Some(page),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ],
        1,
    )
}

#[cfg(talos_boot_scenario = "qemu_process_address_space_smoke")]
fn process_address_space_overlap_plan() -> ProcessImageInstallPlan {
    let fixture = process_address_space_install_fixture().expect("fixture install plan");
    let text = fixture.page(0).expect("fixture text page");
    let data = ProcessImagePageInstallRecord::for_test_unchecked(
        1,
        1,
        text.virtual_start(),
        text.virtual_end(),
        UserSegmentKind::UserData,
        UserMappingPermissions::USER_DATA,
        0,
        0,
        0,
        [None; MAX_ZERO_RANGES_PER_PAGE],
        0,
        0,
        ProcessInstallAction::AllocateCopyZeroMap,
    );
    let mut pages = [None; MAX_PROCESS_INSTALL_PAGES];
    pages[0] = Some(text);
    pages[1] = Some(data);
    process_address_space_plan_from_pages(fixture, fixture.entry(), LOADER_PAGE_SIZE * 2, pages, 2)
}

#[cfg(talos_boot_scenario = "qemu_process_address_space_smoke")]
fn process_address_space_permission_widening_plan() -> ProcessImageInstallPlan {
    let fixture = process_address_space_install_fixture().expect("fixture install plan");
    let page = ProcessImagePageInstallRecord::for_test_unchecked(
        0,
        0,
        0x0000_0000_0001_0000,
        0x0000_0000_0001_1000,
        UserSegmentKind::UserText,
        UserMappingPermissions::USER_DATA,
        0x100,
        0x100,
        4,
        [Some(PageByteRange::for_test_unchecked(
            0x104,
            LOADER_PAGE_SIZE - 0x104,
        )); MAX_ZERO_RANGES_PER_PAGE],
        1,
        LOADER_PAGE_SIZE - 0x104,
        ProcessInstallAction::AllocateCopyZeroMap,
    );
    let mut pages = [None; MAX_PROCESS_INSTALL_PAGES];
    pages[0] = Some(page);
    process_address_space_plan_from_pages(
        fixture,
        0x0000_0000_0001_0100,
        LOADER_PAGE_SIZE,
        pages,
        1,
    )
}

#[cfg(talos_boot_scenario = "qemu_process_address_space_smoke")]
fn process_address_space_plan_from_pages(
    fixture: ProcessImageInstallPlan,
    entry: u64,
    memory_footprint: u64,
    pages: [Option<ProcessImagePageInstallRecord>; MAX_PROCESS_INSTALL_PAGES],
    page_count: usize,
) -> ProcessImageInstallPlan {
    ProcessImageInstallPlan::for_test_unchecked(
        PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY,
        PROCESS_INSTALL_BOUNDARY_IDENTITY,
        PHASE8_INIT_PATH,
        fixture.source_digest(),
        entry,
        memory_footprint,
        page_count,
        pages,
        ProcessInstallSideEffects::NONE,
        true,
    )
}

#[cfg(talos_boot_scenario = "qemu_process_page_table_materialization_smoke")]
pub fn run_process_page_table_materialization_smoke() -> bool {
    crate::println!("qemu-process-page-table-materialization-smoke: start");

    let (success_ok, frames_ok, descriptors_ok, side_effects_ok) =
        process_materialization_report_success();
    let teardown_ok = process_materialization_report_teardown();
    let bad_address_space_ok = process_materialization_report_bad_address_space();
    let forbidden_range_ok = process_materialization_report_error(
        "forbidden-range",
        process_materialization_forbidden_range_fixture(),
        PosixError::AccessDenied,
        ProcessMaterializationRequest::DescriptorImageOnly,
        None,
    );
    let permission_widening_ok = process_materialization_report_error(
        "permission-widening",
        process_materialization_permission_widening_fixture(),
        PosixError::AccessDenied,
        ProcessMaterializationRequest::DescriptorImageOnly,
        None,
    );
    let resource_exhaustion_ok = process_materialization_report_error(
        "resource-exhaustion",
        process_materialization_valid_fixture(),
        PosixError::NoMemory,
        ProcessMaterializationRequest::DescriptorImageOnly,
        Some(ProcessPageTableMaterializationLeaseSource::with_limits(
            1, 3, 3,
        )),
    );
    let unsupported_topology_ok = process_materialization_report_error(
        "unsupported-topology",
        process_materialization_unsupported_topology_fixture(),
        PosixError::NotSupported,
        ProcessMaterializationRequest::DescriptorImageOnly,
        None,
    );
    let copy_zero_mismatch_ok = process_materialization_report_error(
        "copy-zero-mismatch",
        process_materialization_valid_fixture(),
        PosixError::InvalidArgument,
        ProcessMaterializationRequest::DescriptorImageOnly,
        Some(process_materialization_copy_zero_failure_source()),
    );
    let activation_request_ok = process_materialization_report_error(
        "activation-request",
        process_materialization_valid_fixture(),
        PosixError::NotImplemented,
        ProcessMaterializationRequest::RunnableLowerElState,
        None,
    );

    let participants = u64::from(success_ok)
        + u64::from(frames_ok)
        + u64::from(descriptors_ok)
        + u64::from(side_effects_ok)
        + u64::from(teardown_ok)
        + u64::from(bad_address_space_ok)
        + u64::from(forbidden_range_ok)
        + u64::from(permission_widening_ok)
        + u64::from(resource_exhaustion_ok)
        + u64::from(unsupported_topology_ok)
        + u64::from(copy_zero_mismatch_ok)
        + u64::from(activation_request_ok);
    let errors = 12 - participants;
    let classification = if participants == 12 && errors == 0 {
        "qemu-process-page-table-materialization-smoke-complete"
    } else {
        "qemu-process-page-table-materialization-smoke-failed"
    };

    crate::println!(
        "qemu-process-page-table-materialization-smoke: final participants={} expected=12 errors={} classification={}",
        participants,
        errors,
        classification
    );
    if participants == 12 && errors == 0 {
        crate::println!("qemu-process-page-table-materialization-smoke: PASS");
        true
    } else {
        crate::println!("qemu-process-page-table-materialization-smoke: FAIL");
        false
    }
}

#[cfg(talos_boot_scenario = "qemu_process_page_table_materialization_smoke")]
fn process_materialization_report_success() -> (bool, bool, bool, bool) {
    let Ok((image, install_plan, address_space)) = process_materialization_valid_fixture() else {
        process_materialization_report_empty_success();
        return (false, false, false, false);
    };
    let mut lease_source =
        ProcessPageTableMaterializationLeaseSource::for_address_space(address_space);
    let Ok(materialization) = materialize_process_page_tables(
        image,
        install_plan,
        address_space,
        ProcessMaterializationRequest::DescriptorImageOnly,
        &mut lease_source,
    ) else {
        process_materialization_report_empty_success();
        return (false, false, false, false);
    };

    crate::println!(
        "qemu-process-page-table-materialization-smoke: fixture name={} path=/bin/init source-digest={:#x} install-boundary={} address-space-boundary={} materialization-boundary={}",
        PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY,
        materialization.source_digest(),
        PROCESS_INSTALL_BOUNDARY_IDENTITY,
        PROCESS_ADDRESS_SPACE_BOUNDARY_IDENTITY,
        PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY
    );

    let success_ok = materialization.published()
        && materialization.boundary_identity()
            == PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY
        && materialization.kernel_mapping_policy() == PROCESS_PAGE_TABLE_KERNEL_MAPPING_POLICY
        && materialization.root().physical_frame() != 0
        && materialization.table_page_count() == 3
        && materialization.user_frame_count() == install_plan.page_count()
        && materialization.descriptor_count() == install_plan.page_count()
        && materialization.activation_blocked();
    crate::println!(
        "qemu-process-page-table-materialization-smoke: success output=ProcessPageTableMaterialization published={} id={:#x} root-pages=1 table-pages={} user-frames={} descriptors={} activation-blocked={} kernel-mapping-policy={} ok={}",
        materialization.published(),
        materialization.id(),
        materialization.table_page_count(),
        materialization.user_frame_count(),
        materialization.descriptor_count(),
        materialization.activation_blocked(),
        materialization.kernel_mapping_policy(),
        success_ok
    );

    let frame0_ok = process_materialization_report_frame(
        0,
        materialization.user_frame(0),
        UserSegmentKind::UserText,
    );
    let frame1_ok = process_materialization_report_frame(
        1,
        materialization.user_frame(1),
        UserSegmentKind::UserData,
    );
    let descriptor0_ok = process_materialization_report_descriptor(
        0,
        materialization.descriptor(0),
        UserSegmentKind::UserText,
    );
    let descriptor1_ok = process_materialization_report_descriptor(
        1,
        materialization.descriptor(1),
        UserSegmentKind::UserData,
    );

    let side_effects = materialization.side_effects();
    let snapshot = lease_source.snapshot();
    let side_effects_ok = snapshot.root_pages_leased == 1
        && snapshot.table_pages_leased == materialization.table_page_count()
        && snapshot.user_frames_leased == materialization.user_frame_count()
        && snapshot.descriptor_slots_installed == materialization.descriptor_count()
        && side_effects.root_pages_leased() == 1
        && side_effects.table_pages_leased() == materialization.table_page_count()
        && side_effects.user_frames_leased() == materialization.user_frame_count()
        && side_effects.descriptors_installed() == materialization.descriptor_count()
        && side_effects.user_frames_populated() == materialization.user_frame_count()
        && side_effects.activation_blocked();
    crate::println!(
        "qemu-process-page-table-materialization-smoke: side-effects root-pages-leased={} table-pages-leased={} user-frames-leased={} descriptors-installed={} copied-bytes={:#x} zeroed-bytes={:#x} ttbr-mutated=false tlb-mutated=false scheduler-published=false lower-el-frame=false runnable=false ok={}",
        snapshot.root_pages_leased,
        snapshot.table_pages_leased,
        snapshot.user_frames_leased,
        snapshot.descriptor_slots_installed,
        side_effects.copied_bytes(),
        side_effects.zeroed_bytes(),
        side_effects_ok
    );

    (
        success_ok,
        frame0_ok && frame1_ok,
        descriptor0_ok && descriptor1_ok,
        side_effects_ok,
    )
}

#[cfg(talos_boot_scenario = "qemu_process_page_table_materialization_smoke")]
fn process_materialization_report_empty_success() {
    crate::println!(
        "qemu-process-page-table-materialization-smoke: fixture name={} path=/bin/init source-digest=0x0 install-boundary={} address-space-boundary={} materialization-boundary={}",
        PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY,
        PROCESS_INSTALL_BOUNDARY_IDENTITY,
        PROCESS_ADDRESS_SPACE_BOUNDARY_IDENTITY,
        PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY
    );
    crate::println!(
        "qemu-process-page-table-materialization-smoke: success output=ProcessPageTableMaterialization published=false id=0x0 root-pages=0 table-pages=0 user-frames=0 descriptors=0 activation-blocked=true kernel-mapping-policy={} ok=false",
        PROCESS_PAGE_TABLE_KERNEL_MAPPING_POLICY
    );
}

#[cfg(talos_boot_scenario = "qemu_process_page_table_materialization_smoke")]
fn process_materialization_report_frame(
    index: usize,
    frame: Option<crate::process_page_table_materialization::MaterializedUserFrameLease>,
    expected_kind: UserSegmentKind,
) -> bool {
    let Some(frame) = frame else {
        crate::println!(
            "qemu-process-page-table-materialization-smoke: frame index={} kind=missing virtual-page=0x0 physical-frame=0x0 copy-bytes=0x0 zero-bytes=0x0 zero-before-copy=false source-page=0 scrub-required=false ok=false",
            index
        );
        return false;
    };
    let ok = frame.kind() == expected_kind
        && frame.permissions() == process_materialization_expected_permissions(expected_kind)
        && frame.zeroed_before_copy()
        && frame.physical_frame() != 0
        && frame.scrub_required()
        && !frame.released();
    crate::println!(
        "qemu-process-page-table-materialization-smoke: frame index={} kind={} virtual-page={:#x} physical-frame={:#x} copy-bytes={:#x} zero-bytes={:#x} zero-before-copy={} source-page={} scrub-required={} ok={}",
        index,
        frame.kind().name(),
        frame.virtual_page(),
        frame.physical_frame(),
        frame.copied_bytes(),
        frame.zeroed_bytes(),
        frame.zeroed_before_copy(),
        frame.source_page_ordinal(),
        frame.scrub_required(),
        ok
    );
    ok
}

#[cfg(talos_boot_scenario = "qemu_process_page_table_materialization_smoke")]
fn process_materialization_report_descriptor(
    index: usize,
    descriptor: Option<crate::process_page_table_materialization::ProcessPageDescriptorRecord>,
    expected_kind: UserSegmentKind,
) -> bool {
    let Some(descriptor) = descriptor else {
        crate::println!(
            "qemu-process-page-table-materialization-smoke: descriptor index={} kind=missing flags=--- ap=EL0_NONE pxn=false uxn=false attr=unknown af=false wx=true ok=false",
            index
        );
        return false;
    };
    let permissions = process_materialization_expected_permissions(expected_kind);
    let wx = descriptor.writable() && descriptor.executable();
    let ok = descriptor.kind() == expected_kind
        && descriptor.privileged_execute_never()
        && descriptor.user_execute_never()
            == !permissions.contains(UserMappingPermissions::EXECUTE)
        && descriptor.normal_inner_shareable()
        && descriptor.write_xor_execute()
        && !wx;
    crate::println!(
        "qemu-process-page-table-materialization-smoke: descriptor index={} kind={} flags={} ap={} pxn={} uxn={} attr=normal-inner-shareable af=true wx={} ok={}",
        index,
        descriptor.kind().name(),
        process_materialization_permission_flags(permissions),
        process_materialization_ap_name(permissions),
        descriptor.privileged_execute_never(),
        descriptor.user_execute_never(),
        wx,
        ok
    );
    ok
}

#[cfg(talos_boot_scenario = "qemu_process_page_table_materialization_smoke")]
fn process_materialization_report_teardown() -> bool {
    let Ok((image, install_plan, address_space)) = process_materialization_valid_fixture() else {
        process_materialization_report_empty_teardown("first", false);
        process_materialization_report_empty_teardown("second", true);
        return false;
    };
    let mut lease_source =
        ProcessPageTableMaterializationLeaseSource::for_address_space(address_space);
    let Ok(mut materialization) = materialize_process_page_tables(
        image,
        install_plan,
        address_space,
        ProcessMaterializationRequest::DescriptorImageOnly,
        &mut lease_source,
    ) else {
        process_materialization_report_empty_teardown("first", false);
        process_materialization_report_empty_teardown("second", true);
        return false;
    };

    let first = materialization.destroy(&mut lease_source);
    let first_ok = first.descriptors_cleared() == install_plan.page_count()
        && first.table_pages_released() == 3
        && first.user_frames_released() == install_plan.page_count()
        && first.root_released()
        && !first.already_destroyed()
        && lease_source.outstanding_leases() == 0;
    crate::println!(
        "qemu-process-page-table-materialization-smoke: teardown phase=first descriptors-cleared={} table-pages-released={} user-frames-released={} root-released={} already-destroyed={} ok={}",
        first.descriptors_cleared(),
        first.table_pages_released(),
        first.user_frames_released(),
        first.root_released(),
        first.already_destroyed(),
        first_ok
    );

    let second = materialization.destroy(&mut lease_source);
    let second_ok = second.descriptors_cleared() == 0
        && second.table_pages_released() == 0
        && second.user_frames_released() == 0
        && !second.root_released()
        && second.already_destroyed()
        && lease_source.outstanding_leases() == 0;
    crate::println!(
        "qemu-process-page-table-materialization-smoke: teardown phase=second descriptors-cleared={} table-pages-released={} user-frames-released={} root-released={} already-destroyed={} ok={}",
        second.descriptors_cleared(),
        second.table_pages_released(),
        second.user_frames_released(),
        second.root_released(),
        second.already_destroyed(),
        second_ok
    );

    first_ok && second_ok
}

#[cfg(talos_boot_scenario = "qemu_process_page_table_materialization_smoke")]
fn process_materialization_report_empty_teardown(phase: &str, already_destroyed: bool) {
    crate::println!(
        "qemu-process-page-table-materialization-smoke: teardown phase={} descriptors-cleared=0 table-pages-released=0 user-frames-released=0 root-released=false already-destroyed={} ok=false",
        phase,
        already_destroyed
    );
}

#[cfg(talos_boot_scenario = "qemu_process_page_table_materialization_smoke")]
fn process_materialization_report_bad_address_space() -> bool {
    let Ok((image, plan, mut address_space)) = process_materialization_valid_fixture() else {
        crate::println!(
            "qemu-process-page-table-materialization-smoke: error case=bad-address-space errno=-EINVAL partial-materialization=false leaked-leases=false ok=false"
        );
        return false;
    };
    let mut address_space_leases = ProcessAddressSpaceLeaseSource::for_plan(plan);
    let _ = address_space.destroy(&mut address_space_leases);
    process_materialization_report_error(
        "bad-address-space",
        Ok((image, plan, address_space)),
        PosixError::InvalidArgument,
        ProcessMaterializationRequest::DescriptorImageOnly,
        None,
    )
}

#[cfg(talos_boot_scenario = "qemu_process_page_table_materialization_smoke")]
fn process_materialization_report_error(
    case: &str,
    fixture: Result<
        (
            ProgramImagePlan,
            ProcessImageInstallPlan,
            ProcessAddressSpace,
        ),
        PosixError,
    >,
    expected: PosixError,
    request: ProcessMaterializationRequest,
    source_override: Option<ProcessPageTableMaterializationLeaseSource>,
) -> bool {
    let Ok((image, plan, address_space)) = fixture else {
        crate::println!(
            "qemu-process-page-table-materialization-smoke: error case={} errno=-{} partial-materialization=false leaked-leases=false ok=false",
            case,
            expected.name()
        );
        return false;
    };
    let mut lease_source = source_override.unwrap_or_else(|| {
        ProcessPageTableMaterializationLeaseSource::for_address_space(address_space)
    });
    let result =
        materialize_process_page_tables(image, plan, address_space, request, &mut lease_source);
    let ok =
        matches!(result, Err(error) if error == expected) && lease_source.outstanding_leases() == 0;
    crate::println!(
        "qemu-process-page-table-materialization-smoke: error case={} errno=-{} partial-materialization=false leaked-leases={} ok={}",
        case,
        expected.name(),
        lease_source.outstanding_leases() != 0,
        ok
    );
    ok
}

#[cfg(talos_boot_scenario = "qemu_process_page_table_materialization_smoke")]
fn process_materialization_valid_fixture() -> Result<
    (
        ProgramImagePlan,
        ProcessImageInstallPlan,
        ProcessAddressSpace,
    ),
    PosixError,
> {
    let image = plan_phase8_init_image(phase8_readonly_initramfs_fixture())
        .map_err(|error| error.posix_error())?;
    let plan = plan_process_image_install(image)?;
    let mut address_source = ProcessAddressSpaceLeaseSource::for_plan(plan);
    let address_space = install_process_address_space(
        plan,
        process_materialization_address_space_id(),
        Some(process_materialization_owner_id()),
        &mut address_source,
    )?;
    Ok((image, plan, address_space))
}

#[cfg(talos_boot_scenario = "qemu_process_page_table_materialization_smoke")]
fn process_materialization_forbidden_range_fixture() -> Result<
    (
        ProgramImagePlan,
        ProcessImageInstallPlan,
        ProcessAddressSpace,
    ),
    PosixError,
> {
    let image = plan_phase8_init_image(phase8_readonly_initramfs_fixture())
        .map_err(|error| error.posix_error())?;
    let page = ProcessImagePageInstallRecord::for_test_unchecked(
        0,
        0,
        0,
        LOADER_PAGE_SIZE,
        UserSegmentKind::UserText,
        UserMappingPermissions::USER_TEXT,
        0,
        0,
        4,
        [
            Some(PageByteRange::for_test_unchecked(4, LOADER_PAGE_SIZE - 4)),
            None,
        ],
        1,
        LOADER_PAGE_SIZE - 4,
        ProcessInstallAction::AllocateCopyZeroMap,
    );
    process_materialization_fixture_from_pages(
        image,
        [
            Some(page),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ],
        1,
    )
}

#[cfg(talos_boot_scenario = "qemu_process_page_table_materialization_smoke")]
fn process_materialization_permission_widening_fixture() -> Result<
    (
        ProgramImagePlan,
        ProcessImageInstallPlan,
        ProcessAddressSpace,
    ),
    PosixError,
> {
    let image = plan_phase8_init_image(phase8_readonly_initramfs_fixture())
        .map_err(|error| error.posix_error())?;
    let page = ProcessImagePageInstallRecord::for_test_unchecked(
        0,
        0,
        USER_NULL_GUARD_END,
        USER_NULL_GUARD_END + LOADER_PAGE_SIZE,
        UserSegmentKind::UserText,
        UserMappingPermissions::READ,
        0,
        0,
        4,
        [
            Some(PageByteRange::for_test_unchecked(4, LOADER_PAGE_SIZE - 4)),
            None,
        ],
        1,
        LOADER_PAGE_SIZE - 4,
        ProcessInstallAction::AllocateCopyZeroMap,
    );
    process_materialization_fixture_from_pages(
        image,
        [
            Some(page),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ],
        1,
    )
}

#[cfg(talos_boot_scenario = "qemu_process_page_table_materialization_smoke")]
fn process_materialization_unsupported_topology_fixture() -> Result<
    (
        ProgramImagePlan,
        ProcessImageInstallPlan,
        ProcessAddressSpace,
    ),
    PosixError,
> {
    let image = plan_phase8_init_image(phase8_readonly_initramfs_fixture())
        .map_err(|error| error.posix_error())?;
    let text = ProcessImagePageInstallRecord::for_test_unchecked(
        0,
        0,
        USER_NULL_GUARD_END,
        USER_NULL_GUARD_END + LOADER_PAGE_SIZE,
        UserSegmentKind::UserText,
        UserMappingPermissions::USER_TEXT,
        0,
        0,
        4,
        [
            Some(PageByteRange::for_test_unchecked(4, LOADER_PAGE_SIZE - 4)),
            None,
        ],
        1,
        LOADER_PAGE_SIZE - 4,
        ProcessInstallAction::AllocateCopyZeroMap,
    );
    let data = ProcessImagePageInstallRecord::for_test_unchecked(
        1,
        1,
        USER_NULL_GUARD_END + 0x20_0000,
        USER_NULL_GUARD_END + 0x20_0000 + LOADER_PAGE_SIZE,
        UserSegmentKind::UserData,
        UserMappingPermissions::USER_DATA,
        0,
        4,
        4,
        [
            Some(PageByteRange::for_test_unchecked(4, LOADER_PAGE_SIZE - 4)),
            None,
        ],
        1,
        LOADER_PAGE_SIZE - 4,
        ProcessInstallAction::AllocateCopyZeroMap,
    );
    process_materialization_fixture_from_pages(
        image,
        [
            Some(text),
            Some(data),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ],
        2,
    )
}

#[cfg(talos_boot_scenario = "qemu_process_page_table_materialization_smoke")]
fn process_materialization_copy_zero_failure_source() -> ProcessPageTableMaterializationLeaseSource
{
    let Ok((_, _, address_space)) = process_materialization_valid_fixture() else {
        return ProcessPageTableMaterializationLeaseSource::with_limits(0, 0, 0);
    };
    let mut source = ProcessPageTableMaterializationLeaseSource::for_address_space(address_space);
    source.fail_population_at_page(1);
    source
}

#[cfg(talos_boot_scenario = "qemu_process_page_table_materialization_smoke")]
fn process_materialization_fixture_from_pages(
    image: ProgramImagePlan,
    pages: [Option<ProcessImagePageInstallRecord>; MAX_PROCESS_INSTALL_PAGES],
    page_count: usize,
) -> Result<
    (
        ProgramImagePlan,
        ProcessImageInstallPlan,
        ProcessAddressSpace,
    ),
    PosixError,
> {
    let plan = ProcessImageInstallPlan::for_test_unchecked(
        PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY,
        PROCESS_INSTALL_BOUNDARY_IDENTITY,
        PHASE8_INIT_PATH,
        image.source_digest(),
        image.entry(),
        image.memory_footprint(),
        page_count,
        pages,
        ProcessInstallSideEffects::NONE,
        true,
    );
    let address_space = process_materialization_address_space_from_plan(plan);
    Ok((image, plan, address_space))
}

#[cfg(talos_boot_scenario = "qemu_process_page_table_materialization_smoke")]
fn process_materialization_address_space_from_plan(
    plan: ProcessImageInstallPlan,
) -> ProcessAddressSpace {
    let mut mappings = [None; MAX_PROCESS_INSTALL_PAGES];
    let mut user_frame_leases = [None; MAX_PROCESS_INSTALL_PAGES];
    let mut index = 0;
    let mut copied_bytes = 0;
    let mut zeroed_bytes = 0;
    while index < plan.page_count() {
        if let Some(page) = plan.page(index) {
            mappings[index] = Some(ProcessUserMapping::for_test_unchecked(
                page.virtual_start(),
                page.virtual_end(),
                page.kind(),
                page.permissions(),
                page.copy_page_offset(),
                page.copy_file_offset(),
                page.copy_len(),
                page.zero_len(),
                page.index(),
                true,
                !(page.permissions().contains(UserMappingPermissions::WRITE)
                    && page.permissions().contains(UserMappingPermissions::EXECUTE)),
                true,
                true,
            ));
            user_frame_leases[index] = Some(UserFrameLease::for_test_unchecked(
                ModelLeaseToken::for_test_unchecked(0x100 + index as u64),
                page.virtual_start(),
                page.kind(),
                page.permissions(),
                true,
                page.copy_len(),
                page.zero_len(),
                page.index(),
                false,
            ));
            copied_bytes += page.copy_len();
            zeroed_bytes += page.zero_len();
        }
        index += 1;
    }
    ProcessAddressSpace::for_test_unchecked(
        process_materialization_address_space_id(),
        Some(process_materialization_owner_id()),
        PROCESS_ADDRESS_SPACE_BOUNDARY_IDENTITY,
        PageTableRootLease::for_test_unchecked(ModelLeaseToken::for_test_unchecked(1), false),
        [Some(TablePageLease::for_test_unchecked(
            ModelLeaseToken::for_test_unchecked(2),
            false,
        ))],
        1,
        user_frame_leases,
        plan.page_count(),
        mappings,
        plan.page_count(),
        ProcessAddressSpaceSideEffects::for_test_unchecked(
            plan.page_count(),
            1,
            plan.page_count(),
            copied_bytes,
            zeroed_bytes,
            0,
        ),
        true,
        false,
    )
}

#[cfg(talos_boot_scenario = "qemu_process_page_table_materialization_smoke")]
fn process_materialization_address_space_id() -> ProcessAddressSpaceId {
    ProcessAddressSpaceId::new(0x8300_2001).expect("nonzero address-space id")
}

#[cfg(talos_boot_scenario = "qemu_process_page_table_materialization_smoke")]
fn process_materialization_owner_id() -> ProcessOwnerId {
    ProcessOwnerId::new(0x8300_2002).expect("nonzero process owner id")
}

#[cfg(talos_boot_scenario = "qemu_process_page_table_materialization_smoke")]
fn process_materialization_expected_permissions(kind: UserSegmentKind) -> UserMappingPermissions {
    match kind {
        UserSegmentKind::UserText => UserMappingPermissions::USER_TEXT,
        UserSegmentKind::UserData => UserMappingPermissions::USER_DATA,
    }
}

#[cfg(talos_boot_scenario = "qemu_process_page_table_materialization_smoke")]
fn process_materialization_permission_flags(permissions: UserMappingPermissions) -> &'static str {
    match permissions {
        UserMappingPermissions::USER_TEXT => "R-X",
        UserMappingPermissions::USER_DATA => "RW-",
        UserMappingPermissions::READ => "R--",
        _ => "---",
    }
}

#[cfg(talos_boot_scenario = "qemu_process_page_table_materialization_smoke")]
fn process_materialization_ap_name(permissions: UserMappingPermissions) -> &'static str {
    if permissions.contains(UserMappingPermissions::WRITE) {
        "EL0_RW"
    } else {
        "EL0_RO"
    }
}

#[cfg(any(
    talos_boot_scenario = "qemu_initial_process_launch_smoke",
    talos_boot_scenario = "qemu_initial_user_stack_smoke"
))]
pub fn run_initial_process_launch_smoke() -> bool {
    crate::println!("qemu-initial-process-launch-smoke: start");

    let (success_ok, provenance_ok, saved_frame_ok, side_effects_ok) =
        initial_process_launch_report_success();
    let commit_ok = initial_process_launch_report_commit_request();
    let identity_ok = initial_process_launch_report_error(
        "identity-mismatch",
        initial_process_launch_identity_mismatch_fixture(),
        PosixError::InvalidArgument,
        InitialProcessLaunchRequest::PreparePlanOnly,
    );
    let entry_ok = initial_process_launch_report_error(
        "entry-mismatch",
        initial_process_launch_entry_mismatch_fixture(),
        PosixError::NotExecutable,
        InitialProcessLaunchRequest::PreparePlanOnly,
    );
    let missing_descriptor_ok = initial_process_launch_report_error(
        "missing-user-text-descriptor",
        initial_process_launch_missing_descriptor_fixture(),
        PosixError::NotExecutable,
        InitialProcessLaunchRequest::PreparePlanOnly,
    );
    let forbidden_range_ok = initial_process_launch_report_error(
        "forbidden-entry-range",
        initial_process_launch_forbidden_entry_fixture(),
        PosixError::AccessDenied,
        InitialProcessLaunchRequest::PreparePlanOnly,
    );
    let destroyed_input_ok = initial_process_launch_report_error(
        "destroyed-input",
        initial_process_launch_destroyed_input_fixture(),
        PosixError::InvalidArgument,
        InitialProcessLaunchRequest::PreparePlanOnly,
    );
    let activation_ok = initial_process_launch_report_error(
        "activation-request",
        initial_process_launch_valid_fixture(),
        PosixError::NotImplemented,
        InitialProcessLaunchRequest::ActivateAddressSpace,
    );
    let stack_required_ok = initial_process_launch_report_error(
        "stack-required-launch",
        initial_process_launch_valid_fixture(),
        PosixError::NotImplemented,
        InitialProcessLaunchRequest::StackRequiredLaunch,
    );
    let scheduler_publication_ok = initial_process_launch_report_error(
        "scheduler-publication-request",
        initial_process_launch_valid_fixture(),
        PosixError::NotImplemented,
        InitialProcessLaunchRequest::PublishSchedulerRunnable,
    );
    let blocked_request_group_ok = activation_ok && stack_required_ok && scheduler_publication_ok;

    let participants = u64::from(success_ok)
        + u64::from(provenance_ok)
        + u64::from(saved_frame_ok)
        + u64::from(side_effects_ok)
        + u64::from(commit_ok)
        + u64::from(identity_ok)
        + u64::from(entry_ok)
        + u64::from(missing_descriptor_ok)
        + u64::from(forbidden_range_ok)
        + u64::from(destroyed_input_ok)
        + u64::from(blocked_request_group_ok);
    let errors = 11 - participants;
    let classification = if participants == 11 && errors == 0 {
        "qemu-initial-process-launch-smoke-complete"
    } else {
        "qemu-initial-process-launch-smoke-failed"
    };

    crate::println!(
        "qemu-initial-process-launch-smoke: final participants={} expected=11 errors={} classification={}",
        participants,
        errors,
        classification
    );
    if participants == 11 && errors == 0 {
        crate::println!("qemu-initial-process-launch-smoke: PASS");
        true
    } else {
        crate::println!("qemu-initial-process-launch-smoke: FAIL");
        false
    }
}

#[cfg(any(
    talos_boot_scenario = "qemu_initial_process_launch_smoke",
    talos_boot_scenario = "qemu_initial_user_stack_smoke"
))]
fn initial_process_launch_report_success() -> (bool, bool, bool, bool) {
    let Ok((image, install_plan, address_space, materialization)) =
        initial_process_launch_valid_fixture()
    else {
        initial_process_launch_report_empty_success();
        return (false, false, false, false);
    };
    let Ok(plan) = prepare_initial_process_launch(
        image,
        install_plan,
        address_space,
        materialization,
        InitialProcessLaunchRequest::PreparePlanOnly,
    ) else {
        initial_process_launch_report_empty_success();
        return (false, false, false, false);
    };

    crate::println!(
        "qemu-initial-process-launch-smoke: fixture name={} path=/bin/init source-digest={:#x} install-boundary={} address-space-boundary={} materialization-boundary={} launch-boundary={}",
        PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY,
        plan.source_digest(),
        PROCESS_INSTALL_BOUNDARY_IDENTITY,
        PROCESS_ADDRESS_SPACE_BOUNDARY_IDENTITY,
        PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY,
        INITIAL_PROCESS_LAUNCH_BOUNDARY_IDENTITY
    );

    let success_ok = plan.published()
        && plan.boundary_identity() == INITIAL_PROCESS_LAUNCH_BOUNDARY_IDENTITY
        && plan.image_fixture_identity() == PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY
        && plan.install_boundary_identity() == PROCESS_INSTALL_BOUNDARY_IDENTITY
        && plan.address_space_boundary_identity() == PROCESS_ADDRESS_SPACE_BOUNDARY_IDENTITY
        && plan.materialization_boundary_identity()
            == PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY
        && plan.source_path() == PHASE8_INIT_PATH
        && plan.source_digest() == image.source_digest()
        && plan.entry_pc() == image.entry()
        && plan.user_sp_state() == INITIAL_USER_SP_BLOCKED
        && plan.activation_state() == INITIAL_ACTIVATION_BLOCKED;
    crate::println!(
        "qemu-initial-process-launch-smoke: success output=InitialProcessLaunchPlan published={} entry={:#x} user-sp-state={} activation-state={} ok={}",
        plan.published(),
        plan.entry_pc(),
        plan.user_sp_state(),
        plan.activation_state(),
        success_ok
    );

    let entry_mapping_ok = address_space.mapping(0).is_some_and(|mapping| {
        mapping.kind() == UserSegmentKind::UserText
            && mapping
                .permissions()
                .contains(UserMappingPermissions::EXECUTE)
            && mapping.virtual_start() <= image.entry()
            && image.entry() < mapping.virtual_end()
    });
    let descriptor_ok = materialization.descriptor(0).is_some_and(|descriptor| {
        descriptor.virtual_page() == (image.entry() & !0xfff)
            && descriptor.executable()
            && !descriptor.user_execute_never()
    });
    let provenance_ok = image.entry() == install_plan.entry() && entry_mapping_ok && descriptor_ok;
    crate::println!(
        "qemu-initial-process-launch-smoke: entry provenance image=true install={} address-space-user-text={} materialization-user-text-descriptor={} el0-executable={} ok={}",
        install_plan.entry() == image.entry(),
        entry_mapping_ok,
        descriptor_ok,
        descriptor_ok,
        provenance_ok
    );

    let frame = plan.saved_frame_intent();
    let saved_frame_ok = frame.elr() == plan.entry_pc()
        && frame.sp_el0_state() == INITIAL_USER_SP_BLOCKED
        && frame.spsr_state() == INITIAL_SPSR_BLOCKED
        && frame.x0_x5_state() == INITIAL_X0_X5_BLOCKED
        && frame.daif_state() == INITIAL_DAIF_BLOCKED
        && frame.address_space_id() == plan.address_space_id()
        && frame.materialization_id() == plan.materialization_id()
        && frame.address_space_token_state() == INITIAL_ADDRESS_SPACE_TOKEN_STATE;
    crate::println!(
        "qemu-initial-process-launch-smoke: saved-frame-intent elr=entry-pc sp-el0={} spsr={} x0-x5={} daif={} address-space-token={} ok={}",
        frame.sp_el0_state(),
        frame.spsr_state(),
        frame.x0_x5_state(),
        frame.daif_state(),
        frame.address_space_token_state(),
        saved_frame_ok
    );

    let side_effects = plan.side_effects();
    let side_effects_ok = !side_effects.ttbr_mutated()
        && !side_effects.tcr_mutated()
        && !side_effects.mair_mutated()
        && !side_effects.sctlr_mutated()
        && !side_effects.asid_allocated()
        && !side_effects.tlb_mutated()
        && !side_effects.lower_el_eret()
        && !side_effects.scheduler_published()
        && !side_effects.process_table_mutated()
        && !side_effects.descriptor_table_mutated();
    crate::println!(
        "qemu-initial-process-launch-smoke: side-effects ttbr-mutated={} tcr-mutated={} mair-mutated={} sctlr-mutated={} asid-allocated={} tlb-mutated={} lower-el-eret={} scheduler-published={} process-table-mutated={} descriptor-table-mutated={} ok={}",
        side_effects.ttbr_mutated(),
        side_effects.tcr_mutated(),
        side_effects.mair_mutated(),
        side_effects.sctlr_mutated(),
        side_effects.asid_allocated(),
        side_effects.tlb_mutated(),
        side_effects.lower_el_eret(),
        side_effects.scheduler_published(),
        side_effects.process_table_mutated(),
        side_effects.descriptor_table_mutated(),
        side_effects_ok
    );

    (success_ok, provenance_ok, saved_frame_ok, side_effects_ok)
}

#[cfg(any(
    talos_boot_scenario = "qemu_initial_process_launch_smoke",
    talos_boot_scenario = "qemu_initial_user_stack_smoke"
))]
fn initial_process_launch_report_empty_success() {
    crate::println!(
        "qemu-initial-process-launch-smoke: fixture name={} path=/bin/init source-digest=0x0 install-boundary={} address-space-boundary={} materialization-boundary={} launch-boundary={}",
        PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY,
        PROCESS_INSTALL_BOUNDARY_IDENTITY,
        PROCESS_ADDRESS_SPACE_BOUNDARY_IDENTITY,
        PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY,
        INITIAL_PROCESS_LAUNCH_BOUNDARY_IDENTITY
    );
    crate::println!(
        "qemu-initial-process-launch-smoke: success output=InitialProcessLaunchPlan published=false entry=0x0 user-sp-state={} activation-state={} ok=false",
        INITIAL_USER_SP_BLOCKED,
        INITIAL_ACTIVATION_BLOCKED
    );
}

#[cfg(any(
    talos_boot_scenario = "qemu_initial_process_launch_smoke",
    talos_boot_scenario = "qemu_initial_user_stack_smoke"
))]
fn initial_process_launch_report_commit_request() -> bool {
    let Ok((image, install_plan, address_space, materialization)) =
        initial_process_launch_valid_fixture()
    else {
        crate::println!(
            "qemu-initial-process-launch-smoke: commit-request target=runnable errno=-ENOSYS no-partial-launch=false no-runnable-publication=false ok=false"
        );
        return false;
    };
    let Ok(plan) = prepare_initial_process_launch(
        image,
        install_plan,
        address_space,
        materialization,
        InitialProcessLaunchRequest::PreparePlanOnly,
    ) else {
        crate::println!(
            "qemu-initial-process-launch-smoke: commit-request target=runnable errno=-ENOSYS no-partial-launch=false no-runnable-publication=false ok=false"
        );
        return false;
    };
    let result = plan.commit_request(InitialProcessLaunchCommitTarget::Runnable);
    let ok = result.is_err_and(|error| {
        error.error() == PosixError::NotImplemented
            && error.no_partial_launch()
            && error.no_runnable_publication()
    });
    crate::println!(
        "qemu-initial-process-launch-smoke: commit-request target=runnable errno=-ENOSYS no-partial-launch=true no-runnable-publication=true ok={}",
        ok
    );
    ok
}

#[cfg(any(
    talos_boot_scenario = "qemu_initial_process_launch_smoke",
    talos_boot_scenario = "qemu_initial_user_stack_smoke"
))]
fn initial_process_launch_report_error(
    case: &str,
    fixture: Result<
        (
            ProgramImagePlan,
            ProcessImageInstallPlan,
            ProcessAddressSpace,
            ProcessPageTableMaterialization,
        ),
        PosixError,
    >,
    expected: PosixError,
    request: InitialProcessLaunchRequest,
) -> bool {
    let result = fixture.and_then(|(image, install_plan, address_space, materialization)| {
        prepare_initial_process_launch(image, install_plan, address_space, materialization, request)
    });
    let (errno, ok) = match result {
        Ok(_) => (expected, false),
        Err(error) => (error, error == expected),
    };
    crate::println!(
        "qemu-initial-process-launch-smoke: error case={} errno=-{} partial-launch=false runnable-published=false ok={}",
        case,
        errno.name(),
        ok
    );
    ok
}

#[cfg(any(
    talos_boot_scenario = "qemu_initial_process_launch_smoke",
    talos_boot_scenario = "qemu_initial_user_stack_smoke"
))]
fn initial_process_launch_valid_fixture() -> Result<
    (
        ProgramImagePlan,
        ProcessImageInstallPlan,
        ProcessAddressSpace,
        ProcessPageTableMaterialization,
    ),
    PosixError,
> {
    let image = plan_phase8_init_image(phase8_readonly_initramfs_fixture())
        .map_err(|error| error.posix_error())?;
    let install_plan = plan_process_image_install(image)?;
    let mut address_source = ProcessAddressSpaceLeaseSource::for_plan(install_plan);
    let address_space = install_process_address_space(
        install_plan,
        initial_process_launch_address_space_id(),
        Some(initial_process_launch_owner_id()),
        &mut address_source,
    )?;
    let mut materialization_source =
        ProcessPageTableMaterializationLeaseSource::for_address_space(address_space);
    let materialization = materialize_process_page_tables(
        image,
        install_plan,
        address_space,
        ProcessMaterializationRequest::DescriptorImageOnly,
        &mut materialization_source,
    )?;
    Ok((image, install_plan, address_space, materialization))
}

#[cfg(any(
    talos_boot_scenario = "qemu_initial_process_launch_smoke",
    talos_boot_scenario = "qemu_initial_user_stack_smoke"
))]
fn initial_process_launch_identity_mismatch_fixture() -> Result<
    (
        ProgramImagePlan,
        ProcessImageInstallPlan,
        ProcessAddressSpace,
        ProcessPageTableMaterialization,
    ),
    PosixError,
> {
    let (image, install_plan, address_space, materialization) =
        initial_process_launch_valid_fixture()?;
    Ok((
        initial_process_launch_image_with_identity(image, "wrong-fixture"),
        install_plan,
        address_space,
        materialization,
    ))
}

#[cfg(any(
    talos_boot_scenario = "qemu_initial_process_launch_smoke",
    talos_boot_scenario = "qemu_initial_user_stack_smoke"
))]
fn initial_process_launch_entry_mismatch_fixture() -> Result<
    (
        ProgramImagePlan,
        ProcessImageInstallPlan,
        ProcessAddressSpace,
        ProcessPageTableMaterialization,
    ),
    PosixError,
> {
    let (image, install_plan, address_space, materialization) =
        initial_process_launch_valid_fixture()?;
    Ok((
        image,
        initial_process_launch_install_with_entry(install_plan, image.entry() + 4),
        address_space,
        materialization,
    ))
}

#[cfg(any(
    talos_boot_scenario = "qemu_initial_process_launch_smoke",
    talos_boot_scenario = "qemu_initial_user_stack_smoke"
))]
fn initial_process_launch_missing_descriptor_fixture() -> Result<
    (
        ProgramImagePlan,
        ProcessImageInstallPlan,
        ProcessAddressSpace,
        ProcessPageTableMaterialization,
    ),
    PosixError,
> {
    let (image, install_plan, address_space, materialization) =
        initial_process_launch_valid_fixture()?;
    Ok((
        image,
        install_plan,
        address_space,
        materialization.for_test_missing_descriptor(0),
    ))
}

#[cfg(any(
    talos_boot_scenario = "qemu_initial_process_launch_smoke",
    talos_boot_scenario = "qemu_initial_user_stack_smoke"
))]
fn initial_process_launch_forbidden_entry_fixture() -> Result<
    (
        ProgramImagePlan,
        ProcessImageInstallPlan,
        ProcessAddressSpace,
        ProcessPageTableMaterialization,
    ),
    PosixError,
> {
    let (image, install_plan, address_space, materialization) =
        initial_process_launch_valid_fixture()?;
    let bad_image =
        initial_process_launch_image_with_entry(image, USER_NULL_GUARD_END.saturating_sub(4));
    Ok((
        bad_image,
        initial_process_launch_install_with_entry(install_plan, bad_image.entry()),
        address_space,
        materialization,
    ))
}

#[cfg(any(
    talos_boot_scenario = "qemu_initial_process_launch_smoke",
    talos_boot_scenario = "qemu_initial_user_stack_smoke"
))]
fn initial_process_launch_destroyed_input_fixture() -> Result<
    (
        ProgramImagePlan,
        ProcessImageInstallPlan,
        ProcessAddressSpace,
        ProcessPageTableMaterialization,
    ),
    PosixError,
> {
    let (image, install_plan, mut address_space, materialization) =
        initial_process_launch_valid_fixture()?;
    let mut address_source = ProcessAddressSpaceLeaseSource::for_plan(install_plan);
    let _ = address_space.destroy(&mut address_source);
    Ok((image, install_plan, address_space, materialization))
}

#[cfg(any(
    talos_boot_scenario = "qemu_initial_process_launch_smoke",
    talos_boot_scenario = "qemu_initial_user_stack_smoke"
))]
fn initial_process_launch_image_with_identity(
    image: ProgramImagePlan,
    identity: &'static str,
) -> ProgramImagePlan {
    ProgramImagePlan::for_test_unchecked(
        image.source_path(),
        identity,
        image.source_len(),
        image.source_digest(),
        image.entry(),
        image.segment_count(),
        initial_process_launch_image_segments(image),
        image.memory_start(),
        image.memory_end(),
        image.memory_footprint(),
    )
}

#[cfg(any(
    talos_boot_scenario = "qemu_initial_process_launch_smoke",
    talos_boot_scenario = "qemu_initial_user_stack_smoke"
))]
fn initial_process_launch_image_with_entry(
    image: ProgramImagePlan,
    entry: u64,
) -> ProgramImagePlan {
    ProgramImagePlan::for_test_unchecked(
        image.source_path(),
        image.fixture_identity(),
        image.source_len(),
        image.source_digest(),
        entry,
        image.segment_count(),
        initial_process_launch_image_segments(image),
        image.memory_start(),
        image.memory_end(),
        image.memory_footprint(),
    )
}

#[cfg(any(
    talos_boot_scenario = "qemu_initial_process_launch_smoke",
    talos_boot_scenario = "qemu_initial_user_stack_smoke"
))]
fn initial_process_launch_image_segments(
    image: ProgramImagePlan,
) -> [Option<PlannedUserSegment>; MAX_LOAD_SEGMENTS] {
    let mut segments = [None; MAX_LOAD_SEGMENTS];
    let mut index = 0;
    while index < image.segment_count() {
        segments[index] = image.segment(index);
        index += 1;
    }
    segments
}

#[cfg(any(
    talos_boot_scenario = "qemu_initial_process_launch_smoke",
    talos_boot_scenario = "qemu_initial_user_stack_smoke"
))]
fn initial_process_launch_install_with_entry(
    install_plan: ProcessImageInstallPlan,
    entry: u64,
) -> ProcessImageInstallPlan {
    ProcessImageInstallPlan::for_test_unchecked(
        install_plan.fixture_identity(),
        install_plan.install_boundary_identity(),
        install_plan.source_path(),
        install_plan.source_digest(),
        entry,
        install_plan.memory_footprint(),
        install_plan.page_count(),
        initial_process_launch_install_pages(install_plan),
        ProcessInstallSideEffects::NONE,
        install_plan.lower_el_launch_blocked(),
    )
}

#[cfg(any(
    talos_boot_scenario = "qemu_initial_process_launch_smoke",
    talos_boot_scenario = "qemu_initial_user_stack_smoke"
))]
fn initial_process_launch_install_pages(
    install_plan: ProcessImageInstallPlan,
) -> [Option<crate::process_install::ProcessImagePageInstallRecord>; MAX_PROCESS_INSTALL_PAGES] {
    let mut pages = [None; MAX_PROCESS_INSTALL_PAGES];
    let mut index = 0;
    while index < install_plan.page_count() {
        pages[index] = install_plan.page(index);
        index += 1;
    }
    pages
}

#[cfg(any(
    talos_boot_scenario = "qemu_initial_process_launch_smoke",
    talos_boot_scenario = "qemu_initial_user_stack_smoke"
))]
fn initial_process_launch_address_space_id() -> ProcessAddressSpaceId {
    ProcessAddressSpaceId::new(0x8300_4001).expect("nonzero address-space id")
}

#[cfg(any(
    talos_boot_scenario = "qemu_initial_process_launch_smoke",
    talos_boot_scenario = "qemu_initial_user_stack_smoke"
))]
fn initial_process_launch_owner_id() -> ProcessOwnerId {
    ProcessOwnerId::new(0x8300_4002).expect("nonzero process owner id")
}

#[cfg(talos_boot_scenario = "qemu_initial_user_stack_smoke")]
pub fn run_initial_user_stack_smoke() -> bool {
    crate::println!("qemu-initial-user-stack-smoke: start");

    let (success_ok, layout_ok, ownership_ok, startup_ok, launch_binding_ok, side_effects_ok) =
        initial_user_stack_report_success();
    let teardown_ok = initial_user_stack_report_teardown();
    let identity_ok = initial_user_stack_report_error(
        "identity-mismatch",
        initial_user_stack_identity_mismatch_fixture(),
        InitialUserStackRequest::PlanOnly,
        InitialUserStackLeaseSource::for_initial_stack(),
        PosixError::InvalidArgument,
    );
    let range_ok = initial_user_stack_report_error(
        "range-fault",
        initial_user_stack_valid_fixture(),
        InitialUserStackRequest::PlanOnly,
        initial_user_stack_bad_range_source(),
        PosixError::Fault,
    );
    let overlap_ok = initial_user_stack_report_error(
        "image-overlap",
        initial_user_stack_overlap_fixture(),
        InitialUserStackRequest::PlanOnly,
        InitialUserStackLeaseSource::for_initial_stack(),
        PosixError::AccessDenied,
    );
    let executable_ok = initial_user_stack_report_error(
        "executable-stack",
        initial_user_stack_valid_fixture(),
        InitialUserStackRequest::PlanOnly,
        initial_user_stack_executable_source(),
        PosixError::AccessDenied,
    );
    let capacity_ok = initial_user_stack_report_error(
        "capacity-exhausted",
        initial_user_stack_valid_fixture(),
        InitialUserStackRequest::PlanOnly,
        InitialUserStackLeaseSource::with_stack_frame_capacity(2),
        PosixError::NoMemory,
    );
    let already_ready_ok = initial_user_stack_report_error(
        "already-stack-ready",
        initial_user_stack_already_ready_fixture(),
        InitialUserStackRequest::PlanOnly,
        InitialUserStackLeaseSource::for_initial_stack(),
        PosixError::InvalidArgument,
    );
    let live_launch_ok = initial_user_stack_report_error(
        "live-launch-request",
        initial_user_stack_valid_fixture(),
        InitialUserStackRequest::LiveLaunch,
        InitialUserStackLeaseSource::for_initial_stack(),
        PosixError::NotImplemented,
    );
    let blocked_request_group_ok = already_ready_ok && live_launch_ok;

    let participants = u64::from(success_ok)
        + u64::from(layout_ok)
        + u64::from(ownership_ok)
        + u64::from(startup_ok)
        + u64::from(launch_binding_ok)
        + u64::from(side_effects_ok)
        + u64::from(teardown_ok)
        + u64::from(identity_ok)
        + u64::from(range_ok)
        + u64::from(overlap_ok)
        + u64::from(executable_ok)
        + u64::from(capacity_ok)
        + u64::from(blocked_request_group_ok);
    let errors = 13 - participants;
    let classification = if participants == 13 && errors == 0 {
        "qemu-initial-user-stack-smoke-complete"
    } else {
        "qemu-initial-user-stack-smoke-failed"
    };

    crate::println!(
        "qemu-initial-user-stack-smoke: final participants={} expected=13 errors={} classification={}",
        participants,
        errors,
        classification
    );
    if participants == 13 && errors == 0 {
        crate::println!("qemu-initial-user-stack-smoke: PASS");
        true
    } else {
        crate::println!("qemu-initial-user-stack-smoke: FAIL");
        false
    }
}

#[cfg(talos_boot_scenario = "qemu_initial_user_stack_smoke")]
fn initial_user_stack_report_success() -> (bool, bool, bool, bool, bool, bool) {
    let Ok((image, install_plan, address_space, materialization, launch_plan)) =
        initial_user_stack_valid_fixture()
    else {
        initial_user_stack_report_empty_success();
        return (false, false, false, false, false, false);
    };
    let mut stack_source = InitialUserStackLeaseSource::for_initial_stack();
    let Ok(plan) = plan_initial_user_stack(
        image,
        install_plan,
        address_space,
        materialization,
        launch_plan,
        InitialUserStackRequest::PlanOnly,
        &mut stack_source,
    ) else {
        initial_user_stack_report_empty_success();
        return (false, false, false, false, false, false);
    };

    crate::println!(
        "qemu-initial-user-stack-smoke: fixture name={} path=/bin/init source-digest={:#x} install-boundary={} address-space-boundary={} materialization-boundary={} launch-boundary={} stack-boundary={}",
        PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY,
        plan.source_digest(),
        PROCESS_INSTALL_BOUNDARY_IDENTITY,
        PROCESS_ADDRESS_SPACE_BOUNDARY_IDENTITY,
        PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY,
        INITIAL_PROCESS_LAUNCH_BOUNDARY_IDENTITY,
        INITIAL_USER_STACK_BOUNDARY_IDENTITY
    );

    let layout = plan.layout();
    let success_ok = plan.published()
        && plan.boundary_identity() == INITIAL_USER_STACK_BOUNDARY_IDENTITY
        && plan.image_fixture_identity() == PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY
        && plan.install_boundary_identity() == PROCESS_INSTALL_BOUNDARY_IDENTITY
        && plan.address_space_boundary_identity() == PROCESS_ADDRESS_SPACE_BOUNDARY_IDENTITY
        && plan.materialization_boundary_identity()
            == PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY
        && plan.launch_boundary_identity() == INITIAL_PROCESS_LAUNCH_BOUNDARY_IDENTITY
        && plan.source_path() == PHASE8_INIT_PATH
        && plan.source_digest() == image.source_digest()
        && plan.address_space_id() == address_space.id().raw()
        && plan.materialization_id() == materialization.id()
        && plan.entry_pc() == image.entry()
        && layout.stack_top() == 0x0000_8000_0000_0000
        && layout.initial_sp() == 0x0000_8000_0000_0000
        && layout.sp_aligned_16();
    crate::println!(
        "qemu-initial-user-stack-smoke: success output=InitialUserStackPlan published={} stack-top={:#018x} initial-sp={:#018x} sp-aligned-16={} ok={}",
        plan.published(),
        layout.stack_top(),
        layout.initial_sp(),
        layout.sp_aligned_16(),
        success_ok
    );

    let layout_ok = layout.usable_start() == 0x0000_7fff_ffff_c000
        && layout.usable_end() == 0x0000_8000_0000_0000
        && layout.guard_start() == 0x0000_7fff_ffff_b000
        && layout.guard_end() == 0x0000_7fff_ffff_c000
        && layout.page_size() == LOADER_PAGE_SIZE
        && layout.usable_pages() == 4
        && layout.guard_pages() == 1;
    crate::println!(
        "qemu-initial-user-stack-smoke: layout usable-start={:#018x} usable-end={:#018x} guard-start={:#018x} guard-end={:#018x} page-size={:#x} usable-pages={} guard-pages={} ok={}",
        layout.usable_start(),
        layout.usable_end(),
        layout.guard_start(),
        layout.guard_end(),
        layout.page_size(),
        layout.usable_pages(),
        layout.guard_pages(),
        layout_ok
    );

    let mut page_index = 0;
    let mut usable_user_data = plan.page_lease_count() == layout.usable_pages();
    let mut stack_owned = stack_source.outstanding_leases() == layout.usable_pages();
    while page_index < plan.page_lease_count() {
        let Some(lease) = plan.page_lease(page_index) else {
            usable_user_data = false;
            stack_owned = false;
            break;
        };
        usable_user_data &= lease.permissions() == UserMappingPermissions::USER_DATA
            && lease.zeroed_before_copy()
            && lease.copied_bytes() == 0
            && lease.zeroed_bytes() == LOADER_PAGE_SIZE;
        stack_owned &= lease.token().raw() != 0 && !lease.released();
        page_index += 1;
    }
    let ownership_ok = usable_user_data
        && stack_owned
        && plan.guard_pages_reserved() == 1
        && plan.total_copied_bytes() == 0
        && plan.total_zeroed_bytes() == 0x4000;
    crate::println!(
        "qemu-initial-user-stack-smoke: ownership usable-user-data={} stack-owned={} guard-has-frame=false guard-has-descriptor=false total-copied-bytes={} total-zeroed-bytes={:#x} ok={}",
        usable_user_data,
        stack_owned,
        plan.total_copied_bytes(),
        plan.total_zeroed_bytes(),
        ownership_ok
    );

    let startup = plan.startup_payload();
    let startup_ok = startup.state() == "minimal-empty-argc0"
        && startup.argc() == 0
        && startup.argv_null()
        && startup.envp_null()
        && startup.auxv_state() == "blocked-pending-startup-abi"
        && startup.tls_state() == "blocked-pending-startup-abi"
        && startup.copied_startup_bytes() == 0;
    crate::println!(
        "qemu-initial-user-stack-smoke: startup argc={} argv={} envp={} auxv={} tls={} copied-startup-bytes={} ok={}",
        startup.argc(),
        if startup.argv_null() {
            "null"
        } else {
            "nonnull"
        },
        if startup.envp_null() {
            "null"
        } else {
            "nonnull"
        },
        startup.auxv_state(),
        startup.tls_state(),
        startup.copied_startup_bytes(),
        startup_ok
    );

    let binding = plan.launch_binding();
    let launch_binding_ok = binding.user_sp_state() == INITIAL_USER_STACK_READY
        && binding.saved_frame_sp_el0() == layout.initial_sp()
        && binding.activation_state() == INITIAL_ACTIVATION_BLOCKED
        && binding.no_partial_launch();
    crate::println!(
        "qemu-initial-user-stack-smoke: launch-binding user-sp-state={} saved-frame-sp-el0={:#018x} activation-state={} no-partial-launch={} ok={}",
        binding.user_sp_state(),
        binding.saved_frame_sp_el0(),
        binding.activation_state(),
        binding.no_partial_launch(),
        launch_binding_ok
    );

    let side_effects = binding.side_effects();
    let side_effects_ok = !side_effects.ttbr_mutated()
        && !side_effects.tcr_mutated()
        && !side_effects.mair_mutated()
        && !side_effects.sctlr_mutated()
        && !side_effects.asid_allocated()
        && !side_effects.tlb_mutated()
        && !side_effects.lower_el_eret()
        && !side_effects.scheduler_published()
        && !side_effects.process_table_mutated()
        && !side_effects.descriptor_table_mutated();
    crate::println!(
        "qemu-initial-user-stack-smoke: side-effects ttbr-mutated={} tcr-mutated={} mair-mutated={} sctlr-mutated={} asid-allocated={} tlb-mutated={} lower-el-eret={} scheduler-published={} process-table-mutated={} descriptor-table-mutated={} ok={}",
        side_effects.ttbr_mutated(),
        side_effects.tcr_mutated(),
        side_effects.mair_mutated(),
        side_effects.sctlr_mutated(),
        side_effects.asid_allocated(),
        side_effects.tlb_mutated(),
        side_effects.lower_el_eret(),
        side_effects.scheduler_published(),
        side_effects.process_table_mutated(),
        side_effects.descriptor_table_mutated(),
        side_effects_ok
    );

    (
        success_ok,
        layout_ok,
        ownership_ok,
        startup_ok,
        launch_binding_ok,
        side_effects_ok,
    )
}

#[cfg(talos_boot_scenario = "qemu_initial_user_stack_smoke")]
fn initial_user_stack_report_empty_success() {
    crate::println!(
        "qemu-initial-user-stack-smoke: fixture name={} path=/bin/init source-digest=0x0 install-boundary={} address-space-boundary={} materialization-boundary={} launch-boundary={} stack-boundary={}",
        PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY,
        PROCESS_INSTALL_BOUNDARY_IDENTITY,
        PROCESS_ADDRESS_SPACE_BOUNDARY_IDENTITY,
        PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY,
        INITIAL_PROCESS_LAUNCH_BOUNDARY_IDENTITY,
        INITIAL_USER_STACK_BOUNDARY_IDENTITY
    );
    crate::println!(
        "qemu-initial-user-stack-smoke: success output=InitialUserStackPlan published=false stack-top=0x0000000000000000 initial-sp=0x0000000000000000 sp-aligned-16=false ok=false"
    );
}

#[cfg(talos_boot_scenario = "qemu_initial_user_stack_smoke")]
fn initial_user_stack_report_teardown() -> bool {
    let Ok((image, install_plan, address_space, materialization, launch_plan)) =
        initial_user_stack_valid_fixture()
    else {
        crate::println!(
            "qemu-initial-user-stack-smoke: teardown stack-leases-released=false image-leases-untouched=false idempotent=false ok=false"
        );
        return false;
    };
    let mut stack_source = InitialUserStackLeaseSource::for_initial_stack();
    let Ok(mut plan) = plan_initial_user_stack(
        image,
        install_plan,
        address_space,
        materialization,
        launch_plan,
        InitialUserStackRequest::PlanOnly,
        &mut stack_source,
    ) else {
        crate::println!(
            "qemu-initial-user-stack-smoke: teardown stack-leases-released=false image-leases-untouched=false idempotent=false ok=false"
        );
        return false;
    };

    let first = plan.destroy(&mut stack_source);
    let second = plan.destroy(&mut stack_source);
    let stack_leases_released =
        first.stack_frame_releases() == 4 && stack_source.outstanding_leases() == 0;
    let image_leases_untouched = first.image_leases_untouched() && second.image_leases_untouched();
    let idempotent = !first.already_destroyed()
        && second.already_destroyed()
        && second.stack_frame_releases() == 0
        && !plan.published()
        && plan.destroyed();
    let ok = stack_leases_released && image_leases_untouched && idempotent;
    crate::println!(
        "qemu-initial-user-stack-smoke: teardown stack-leases-released={} image-leases-untouched={} idempotent={} ok={}",
        stack_leases_released,
        image_leases_untouched,
        idempotent,
        ok
    );
    ok
}

#[cfg(talos_boot_scenario = "qemu_initial_user_stack_smoke")]
fn initial_user_stack_report_error(
    case: &str,
    fixture: Result<
        (
            ProgramImagePlan,
            ProcessImageInstallPlan,
            ProcessAddressSpace,
            ProcessPageTableMaterialization,
            crate::initial_process_launch::InitialProcessLaunchPlan,
        ),
        PosixError,
    >,
    request: InitialUserStackRequest,
    mut stack_source: InitialUserStackLeaseSource,
    expected: PosixError,
) -> bool {
    let result = fixture.and_then(
        |(image, install_plan, address_space, materialization, launch)| {
            plan_initial_user_stack(
                image,
                install_plan,
                address_space,
                materialization,
                launch,
                request,
                &mut stack_source,
            )
        },
    );
    let (errno, ok) = match result {
        Ok(_) => (expected, false),
        Err(error) => (error, error == expected),
    };
    let partial_stack = stack_source.outstanding_leases() != 0;
    let partial_launch = false;
    if case == "capacity-exhausted" {
        crate::println!(
            "qemu-initial-user-stack-smoke: error case={} errno=-{} partial-stack={} partial-launch={} leases-released={} ok={}",
            case,
            errno.name(),
            partial_stack,
            partial_launch,
            stack_source.snapshot().stack_frame_releases == 2 && !partial_stack,
            ok && !partial_stack
        );
    } else if case == "live-launch-request" {
        crate::println!(
            "qemu-initial-user-stack-smoke: error case={} errno=-{} partial-stack={} partial-launch={} runnable-published=false ok={}",
            case,
            errno.name(),
            partial_stack,
            partial_launch,
            ok && !partial_stack
        );
    } else {
        crate::println!(
            "qemu-initial-user-stack-smoke: error case={} errno=-{} partial-stack={} partial-launch={} ok={}",
            case,
            errno.name(),
            partial_stack,
            partial_launch,
            ok && !partial_stack
        );
    }
    ok && !partial_stack && !partial_launch
}

#[cfg(talos_boot_scenario = "qemu_initial_user_stack_smoke")]
fn initial_user_stack_valid_fixture() -> Result<
    (
        ProgramImagePlan,
        ProcessImageInstallPlan,
        ProcessAddressSpace,
        ProcessPageTableMaterialization,
        crate::initial_process_launch::InitialProcessLaunchPlan,
    ),
    PosixError,
> {
    let (image, install_plan, address_space, materialization) =
        initial_process_launch_valid_fixture()?;
    let launch_plan = prepare_initial_process_launch(
        image,
        install_plan,
        address_space,
        materialization,
        InitialProcessLaunchRequest::PreparePlanOnly,
    )?;
    Ok((
        image,
        install_plan,
        address_space,
        materialization,
        launch_plan,
    ))
}

#[cfg(talos_boot_scenario = "qemu_initial_user_stack_smoke")]
fn initial_user_stack_identity_mismatch_fixture() -> Result<
    (
        ProgramImagePlan,
        ProcessImageInstallPlan,
        ProcessAddressSpace,
        ProcessPageTableMaterialization,
        crate::initial_process_launch::InitialProcessLaunchPlan,
    ),
    PosixError,
> {
    let (image, install_plan, address_space, materialization, launch_plan) =
        initial_user_stack_valid_fixture()?;
    Ok((
        initial_process_launch_image_with_identity(image, "wrong-fixture"),
        install_plan,
        address_space,
        materialization,
        launch_plan,
    ))
}

#[cfg(talos_boot_scenario = "qemu_initial_user_stack_smoke")]
fn initial_user_stack_overlap_fixture() -> Result<
    (
        ProgramImagePlan,
        ProcessImageInstallPlan,
        ProcessAddressSpace,
        ProcessPageTableMaterialization,
        crate::initial_process_launch::InitialProcessLaunchPlan,
    ),
    PosixError,
> {
    let (image, install_plan, address_space, materialization, launch_plan) =
        initial_user_stack_valid_fixture()?;
    let mut segments = initial_process_launch_image_segments(image);
    let index = image.segment_count();
    segments[index] = Some(PlannedUserSegment::for_test_unchecked(
        UserSegmentKind::UserData,
        UserMappingPermissions::USER_DATA,
        0x0000_7fff_ffff_c000,
        0x0000_8000_0000_0000,
        0x0000_7fff_ffff_c000,
        0x0000_8000_0000_0000,
        0,
        0,
        0x0000_7fff_ffff_c000,
        0x0000_8000_0000_0000,
    ));
    Ok((
        ProgramImagePlan::for_test_unchecked(
            image.source_path(),
            image.fixture_identity(),
            image.source_len(),
            image.source_digest(),
            image.entry(),
            image.segment_count() + 1,
            segments,
            image.memory_start(),
            image.memory_end(),
            image.memory_footprint(),
        ),
        install_plan,
        address_space,
        materialization,
        launch_plan,
    ))
}

#[cfg(talos_boot_scenario = "qemu_initial_user_stack_smoke")]
fn initial_user_stack_already_ready_fixture() -> Result<
    (
        ProgramImagePlan,
        ProcessImageInstallPlan,
        ProcessAddressSpace,
        ProcessPageTableMaterialization,
        crate::initial_process_launch::InitialProcessLaunchPlan,
    ),
    PosixError,
> {
    let (image, install_plan, address_space, materialization, launch_plan) =
        initial_user_stack_valid_fixture()?;
    Ok((
        image,
        install_plan,
        address_space,
        materialization,
        launch_plan.for_test_with_user_sp_state(INITIAL_USER_STACK_READY),
    ))
}

#[cfg(talos_boot_scenario = "qemu_initial_user_stack_smoke")]
fn initial_user_stack_bad_range_source() -> InitialUserStackLeaseSource {
    let mut source = InitialUserStackLeaseSource::for_initial_stack();
    source.override_layout(InitialUserStackLayout::for_test_unchecked(
        0x0000_8000_0000_0000,
        0x0000_8000_0000_0000,
        0x0000_0000_0000_1000,
        0x0000_0000_0000_5000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_1000,
        LOADER_PAGE_SIZE,
        4,
        1,
        UserMappingPermissions::USER_DATA,
    ));
    source
}

#[cfg(talos_boot_scenario = "qemu_initial_user_stack_smoke")]
fn initial_user_stack_executable_source() -> InitialUserStackLeaseSource {
    let mut source = InitialUserStackLeaseSource::for_initial_stack();
    source.override_permissions(UserMappingPermissions::USER_TEXT);
    source
}

#[cfg(talos_boot_scenario = "qemu_syscall_smoke")]
pub fn handle_syscall_smoke_exception(
    esr: u64,
    _elr: u64,
    far: u64,
    vector: ExceptionVector,
    _spsr: u64,
    saved_frame: *mut ExceptionFrame,
) -> bool {
    let marker = crate::arch::aarch64::exceptions::svc_immediate(esr);
    let reported_esr = esr & !(1 << 25);
    let Some(frame) = (unsafe { saved_frame.as_ref() }) else {
        SYSCALL_SMOKE_ERRORS.fetch_add(1, Ordering::Relaxed);
        return false;
    };

    if marker == syscall::DIAGNOSTIC_EL0_TRAP_SVC_IMMEDIATE {
        let unknown_x0 = frame.reg(0);
        let unknown_ok = unknown_x0 == SYSCALL_SMOKE_EXPECTED_ENOSYS_X0
            && SYSCALL_SMOKE_UNKNOWN_DISPATCHED.load(Ordering::Relaxed) == 1;
        SYSCALL_SMOKE_UNKNOWN_OBSERVED.store(u64::from(unknown_ok), Ordering::Relaxed);
        if !unknown_ok {
            SYSCALL_SMOKE_ERRORS.fetch_add(1, Ordering::Relaxed);
        }
        crate::println!(
            "qemu-syscall-smoke: user-observed case=unknown x0={:#018x} ok={}",
            unknown_x0,
            unknown_ok
        );
        let stable = syscall::is_stable_syscall_svc_immediate(marker);
        crate::println!(
            "qemu-syscall-smoke: diagnostic-marker marker=0x7a10 stable-syscall={} dispatched=false",
            stable
        );
        finish_syscall_smoke(reported_esr == SYSCALL_SMOKE_EXPECTED_MARKER_ESR && far == 0);
    }

    if reported_esr != SYSCALL_SMOKE_EXPECTED_SVC_ESR {
        SYSCALL_SMOKE_ERRORS.fetch_add(1, Ordering::Relaxed);
        return false;
    }

    let raw_number = frame.reg(8);
    if raw_number == SYSCALL_SMOKE_UNKNOWN_NUMBER {
        let talos_nop_x0 = frame.reg(0);
        let talos_nop_ok =
            talos_nop_x0 == 0 && SYSCALL_SMOKE_TALOS_NOP_DISPATCHED.load(Ordering::Relaxed) == 1;
        SYSCALL_SMOKE_TALOS_NOP_OBSERVED.store(u64::from(talos_nop_ok), Ordering::Relaxed);
        if !talos_nop_ok {
            SYSCALL_SMOKE_ERRORS.fetch_add(1, Ordering::Relaxed);
        }
        crate::println!(
            "qemu-syscall-smoke: user-observed case=talos_nop x0={:#018x} ok={}",
            talos_nop_x0,
            talos_nop_ok
        );
    }

    let Some(routed) =
        crate::arch::aarch64::exceptions::try_route_lower_aarch64_syscall(vector, esr, saved_frame)
    else {
        SYSCALL_SMOKE_ERRORS.fetch_add(1, Ordering::Relaxed);
        return false;
    };

    match SyscallNumber::from_raw(routed.raw_number) {
        SyscallNumber::TalosNop => {
            let args = routed.arguments.values();
            let args_ok = args == [0; syscall::MAX_SCALAR_ARGUMENTS];
            if !args_ok || routed.return_x0 != 0 {
                SYSCALL_SMOKE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            SYSCALL_SMOKE_TALOS_NOP_DISPATCHED.store(1, Ordering::Relaxed);
            crate::println!(
                "qemu-syscall-smoke: syscall case=talos_nop vector={} esr={:#018x} svc=0x0000 number={} args=[x0={:#018x} x1={:#018x} x2={:#018x} x3={:#018x} x4={:#018x} x5={:#018x}] return-x0={:#018x}",
                vector.name(),
                reported_esr,
                routed.raw_number,
                args[0],
                args[1],
                args[2],
                args[3],
                args[4],
                args[5],
                routed.return_x0
            );
        }
        SyscallNumber::Unknown(_) => {
            let return_ok = routed.return_x0 == SYSCALL_SMOKE_EXPECTED_ENOSYS_X0;
            if routed.raw_number != SYSCALL_SMOKE_UNKNOWN_NUMBER || !return_ok {
                SYSCALL_SMOKE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            SYSCALL_SMOKE_UNKNOWN_DISPATCHED.store(u64::from(return_ok), Ordering::Relaxed);
            crate::println!(
                "qemu-syscall-smoke: syscall case=unknown vector={} esr={:#018x} svc=0x0000 number={} return-x0={:#018x} expected=-ENOSYS",
                vector.name(),
                reported_esr,
                routed.raw_number,
                routed.return_x0
            );
        }
        SyscallNumber::TalosWrite
        | SyscallNumber::TalosClose
        | SyscallNumber::TalosDup
        | SyscallNumber::TalosRead => {
            SYSCALL_SMOKE_ERRORS.fetch_add(1, Ordering::Relaxed);
            crate::println!(
                "qemu-syscall-smoke: syscall case=unexpected_context_syscall vector={} esr={:#018x} svc=0x0000 number={} return-x0={:#018x}",
                vector.name(),
                reported_esr,
                routed.raw_number,
                routed.return_x0
            );
        }
    }

    true
}

#[cfg(talos_boot_scenario = "qemu_pointer_copy_smoke")]
pub fn handle_pointer_copy_smoke_exception(
    esr: u64,
    _elr: u64,
    far: u64,
    vector: ExceptionVector,
    _spsr: u64,
    saved_frame: *mut ExceptionFrame,
) -> bool {
    let marker = crate::arch::aarch64::exceptions::svc_immediate(esr);
    let reported_esr = esr & !(1 << 25);
    let Some(frame) = (unsafe { saved_frame.as_mut() }) else {
        POINTER_COPY_ERRORS.fetch_add(1, Ordering::Relaxed);
        return false;
    };

    if marker == syscall::DIAGNOSTIC_EL0_TRAP_SVC_IMMEDIATE {
        let unknown_x0 = frame.reg(0);
        let unknown_ok = unknown_x0 == SYSCALL_SMOKE_EXPECTED_ENOSYS_X0
            && POINTER_COPY_UNKNOWN_DISPATCHED.load(Ordering::Relaxed) == 1;
        POINTER_COPY_UNKNOWN_OBSERVED.store(u64::from(unknown_ok), Ordering::Relaxed);
        if !unknown_ok {
            POINTER_COPY_ERRORS.fetch_add(1, Ordering::Relaxed);
        }
        crate::println!(
            "qemu-pointer-copy-smoke: user-observed case=unknown x0={:#018x} ok={}",
            unknown_x0,
            unknown_ok
        );
        let stable = syscall::is_stable_syscall_svc_immediate(marker);
        crate::println!(
            "qemu-pointer-copy-smoke: diagnostic-marker marker=0x7a10 stable-syscall={} dispatched=false",
            stable
        );
        finish_pointer_copy_smoke(reported_esr == SYSCALL_SMOKE_EXPECTED_MARKER_ESR && far == 0);
    }

    if reported_esr != SYSCALL_SMOKE_EXPECTED_SVC_ESR {
        POINTER_COPY_ERRORS.fetch_add(1, Ordering::Relaxed);
        return false;
    }

    let raw_number = frame.reg(8);
    if raw_number == syscall::TALOS_COPY_PROBE_SYSCALL {
        let arguments = syscall::SyscallArguments::new([
            frame.reg(0),
            frame.reg(1),
            frame.reg(2),
            frame.reg(3),
            frame.reg(4),
            frame.reg(5),
        ]);
        let args = arguments.values();
        let mapping = UserMapping::new(
            POINTER_COPY_USER_DATA_START,
            POINTER_COPY_USER_DATA_LEN,
            UserMappingPermissions::USER_DATA,
        )
        .expect("fixed pointer-copy smoke data mapping is a valid user mapping");
        let result = syscall::dispatch_copy_probe(
            arguments,
            &[mapping],
            POINTER_COPY_USER_DATA_START,
            unsafe { &mut *core::ptr::addr_of_mut!(POINTER_COPY_USER_DATA) },
        );
        let return_x0 = result.x0();
        frame.set_reg(0, return_x0);

        if args[0] == POINTER_COPY_USER_DATA_START {
            let data_ok = pointer_copy_user_data_replaced();
            let return_ok = return_x0 == 16 && data_ok;
            if !return_ok {
                POINTER_COPY_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            POINTER_COPY_SUCCESS_DISPATCHED.store(u64::from(return_ok), Ordering::Relaxed);
            POINTER_COPY_SUCCESS_OBSERVED.store(u64::from(return_ok), Ordering::Relaxed);
            crate::println!(
                "qemu-pointer-copy-smoke: syscall case=copy_probe_success vector={} esr={:#018x} svc=0x0000 number={:#018x} args=[x0={:#018x} x1={:#018x} x2={:#018x} x3={:#018x} x4={:#018x} x5={:#018x}] return-x0={:#018x}",
                vector.name(),
                reported_esr,
                raw_number,
                args[0],
                args[1],
                args[2],
                args[3],
                args[4],
                args[5],
                return_x0
            );
            crate::println!(
                "qemu-pointer-copy-smoke: user-observed case=copy_probe_success x0={:#018x} data=0xa5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5 ok={}",
                return_x0,
                return_ok
            );
        } else {
            let return_ok = return_x0 == POINTER_COPY_EXPECTED_EFAULT_X0;
            if !return_ok {
                POINTER_COPY_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            POINTER_COPY_EFAULT_DISPATCHED.store(u64::from(return_ok), Ordering::Relaxed);
            POINTER_COPY_EFAULT_OBSERVED.store(u64::from(return_ok), Ordering::Relaxed);
            crate::println!(
                "qemu-pointer-copy-smoke: syscall case=copy_probe_efault vector={} esr={:#018x} svc=0x0000 number={:#018x} args=[x0={:#018x} x1={:#018x} x2={:#018x} x3={:#018x} x4={:#018x} x5={:#018x}] return-x0={:#018x} expected=-EFAULT",
                vector.name(),
                reported_esr,
                raw_number,
                args[0],
                args[1],
                args[2],
                args[3],
                args[4],
                args[5],
                return_x0
            );
            crate::println!(
                "qemu-pointer-copy-smoke: user-observed case=copy_probe_efault x0={:#018x} ok={}",
                return_x0,
                return_ok
            );
        }

        return true;
    }

    let Some(routed) =
        crate::arch::aarch64::exceptions::try_route_lower_aarch64_syscall(vector, esr, saved_frame)
    else {
        POINTER_COPY_ERRORS.fetch_add(1, Ordering::Relaxed);
        return false;
    };

    let return_ok = routed.raw_number == SYSCALL_SMOKE_UNKNOWN_NUMBER
        && matches!(
            SyscallNumber::from_raw(routed.raw_number),
            SyscallNumber::Unknown(_)
        )
        && routed.return_x0 == SYSCALL_SMOKE_EXPECTED_ENOSYS_X0;
    if !return_ok {
        POINTER_COPY_ERRORS.fetch_add(1, Ordering::Relaxed);
    }
    POINTER_COPY_UNKNOWN_DISPATCHED.store(u64::from(return_ok), Ordering::Relaxed);
    crate::println!(
        "qemu-pointer-copy-smoke: syscall case=unknown vector={} esr={:#018x} svc=0x0000 number={} return-x0={:#018x} expected=-ENOSYS",
        vector.name(),
        reported_esr,
        routed.raw_number,
        routed.return_x0
    );

    true
}

#[cfg(talos_boot_scenario = "qemu_descriptor_write_smoke")]
struct DescriptorWriteCaptureConsole;

#[cfg(talos_boot_scenario = "qemu_descriptor_write_smoke")]
impl crate::runtime_console::ConsoleBackend for DescriptorWriteCaptureConsole {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_bytes(s.as_bytes())
    }

    fn write_bytes(&mut self, bytes: &[u8]) -> core::fmt::Result {
        let len = DESCRIPTOR_WRITE_CONSOLE_LEN.load(Ordering::Relaxed) as usize;
        let Some(end) = len.checked_add(bytes.len()) else {
            return Err(core::fmt::Error);
        };
        if end > 64 {
            return Err(core::fmt::Error);
        }
        unsafe {
            let capture = &mut *core::ptr::addr_of_mut!(DESCRIPTOR_WRITE_CONSOLE_CAPTURE);
            capture[len..end].copy_from_slice(bytes);
        }
        DESCRIPTOR_WRITE_CONSOLE_LEN.store(end as u64, Ordering::Relaxed);
        Ok(())
    }
}

#[cfg(talos_boot_scenario = "qemu_descriptor_write_smoke")]
pub fn handle_descriptor_write_smoke_exception(
    esr: u64,
    _elr: u64,
    far: u64,
    vector: ExceptionVector,
    _spsr: u64,
    saved_frame: *mut ExceptionFrame,
) -> bool {
    let marker = crate::arch::aarch64::exceptions::svc_immediate(esr);
    let reported_esr = esr & !(1 << 25);
    let Some(frame) = (unsafe { saved_frame.as_mut() }) else {
        DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
        return false;
    };

    if marker == syscall::DIAGNOSTIC_EL0_TRAP_SVC_IMMEDIATE {
        let stable = syscall::is_stable_syscall_svc_immediate(marker);
        crate::println!(
            "qemu-descriptor-write-smoke: diagnostic-marker marker=0x7a10 stable-syscall={} dispatched=false",
            stable
        );
        finish_descriptor_write_smoke(
            reported_esr == SYSCALL_SMOKE_EXPECTED_MARKER_ESR && far == 0,
        );
    }

    if reported_esr != SYSCALL_SMOKE_EXPECTED_SVC_ESR {
        DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
        return false;
    }

    let arguments = syscall::SyscallArguments::new([
        frame.reg(0),
        frame.reg(1),
        frame.reg(2),
        frame.reg(3),
        frame.reg(4),
        frame.reg(5),
    ]);
    let args = arguments.values();
    let raw_number = frame.reg(8);
    let before_len = DESCRIPTOR_WRITE_CONSOLE_LEN.load(Ordering::Relaxed) as usize;
    let descriptor_table =
        crate::posix::DescriptorTable::<4>::with_inherited_stdio().expect("stdio table");
    let mappings = [UserMapping::new(
        DESCRIPTOR_WRITE_USER_DATA_START,
        DESCRIPTOR_WRITE_USER_DATA_LEN,
        UserMappingPermissions::USER_DATA,
    )
    .expect("fixed descriptor-write smoke data mapping is valid")];
    let mut scratch = [0u8; 64];
    let mut console = DescriptorWriteCaptureConsole;
    let result = syscall::dispatch_descriptor_write(
        raw_number,
        arguments,
        &descriptor_table,
        &mappings,
        DESCRIPTOR_WRITE_USER_DATA_START,
        unsafe { &*core::ptr::addr_of!(DESCRIPTOR_WRITE_USER_DATA) },
        &mut scratch,
        &mut console,
    );
    let return_x0 = result.return_value().x0();
    frame.set_reg(0, return_x0);
    let after_len = DESCRIPTOR_WRITE_CONSOLE_LEN.load(Ordering::Relaxed) as usize;

    match raw_number {
        syscall::TALOS_WRITE_SYSCALL
            if args[0] == 1 && args[1] == DESCRIPTOR_WRITE_USER_DATA_START && args[3] == 0 =>
        {
            let console_ok = return_x0 == 18
                && after_len == before_len + DESCRIPTOR_WRITE_STDOUT.len()
                && descriptor_write_console_matches(before_len, DESCRIPTOR_WRITE_STDOUT);
            DESCRIPTOR_WRITE_STDOUT_OBSERVED.store(u64::from(console_ok), Ordering::Relaxed);
            if !console_ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-descriptor-write-smoke: syscall case=write_stdout vector={} esr={:#018x} svc=0x0000 number=1 args=[x0={:#018x} x1={:#018x} x2={:#018x} x3={:#018x} x4={:#018x} x5={:#018x}] return-x0={:#018x}",
                vector.name(),
                reported_esr,
                args[0],
                args[1],
                args[2],
                args[3],
                args[4],
                args[5],
                return_x0
            );
            crate::println!(
                "qemu-descriptor-write-smoke: runtime-console case=write_stdout device=runtime-console0 bytes=18 hex=74616c6f732d7374646f75742d71656d750a ok={}",
                console_ok
            );
            crate::println!(
                "qemu-descriptor-write-smoke: user-observed case=write_stdout x0={:#018x} ok={}",
                return_x0,
                console_ok
            );
        }
        syscall::TALOS_WRITE_SYSCALL
            if args[0] == 2
                && args[1]
                    == DESCRIPTOR_WRITE_USER_DATA_START + DESCRIPTOR_WRITE_STDERR_OFFSET as u64 =>
        {
            let console_ok = return_x0 == 18
                && after_len == before_len + DESCRIPTOR_WRITE_STDERR.len()
                && descriptor_write_console_matches(before_len, DESCRIPTOR_WRITE_STDERR);
            DESCRIPTOR_WRITE_STDERR_OBSERVED.store(u64::from(console_ok), Ordering::Relaxed);
            if !console_ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-descriptor-write-smoke: syscall case=write_stderr vector={} esr={:#018x} svc=0x0000 number=1 args=[x0={:#018x} x1={:#018x} x2={:#018x} x3={:#018x} x4={:#018x} x5={:#018x}] return-x0={:#018x}",
                vector.name(),
                reported_esr,
                args[0],
                args[1],
                args[2],
                args[3],
                args[4],
                args[5],
                return_x0
            );
            crate::println!(
                "qemu-descriptor-write-smoke: runtime-console case=write_stderr device=runtime-console0 bytes=18 hex=74616c6f732d7374646572722d71656d750a ok={}",
                console_ok
            );
            crate::println!(
                "qemu-descriptor-write-smoke: user-observed case=write_stderr x0={:#018x} ok={}",
                return_x0,
                console_ok
            );
        }
        syscall::TALOS_WRITE_SYSCALL if args[0] == 0 => {
            let ok = return_x0 == DESCRIPTOR_WRITE_EXPECTED_EBADF_X0 && after_len == before_len;
            DESCRIPTOR_WRITE_FD0_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-descriptor-write-smoke: syscall case=write_fd0 vector={} esr={:#018x} svc=0x0000 number=1 return-x0={:#018x} expected=-EBADF console-unchanged={}",
                vector.name(),
                reported_esr,
                return_x0,
                after_len == before_len
            );
        }
        syscall::TALOS_WRITE_SYSCALL if args[0] == 99 => {
            let ok = return_x0 == DESCRIPTOR_WRITE_EXPECTED_EBADF_X0 && after_len == before_len;
            DESCRIPTOR_WRITE_BADFD_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-descriptor-write-smoke: syscall case=write_badfd vector={} esr={:#018x} svc=0x0000 number=1 return-x0={:#018x} expected=-EBADF console-unchanged={}",
                vector.name(),
                reported_esr,
                return_x0,
                after_len == before_len
            );
        }
        syscall::TALOS_WRITE_SYSCALL if args[1] == EL0_TRAP_USER_GUARD_START => {
            let ok = return_x0 == DESCRIPTOR_WRITE_EXPECTED_EFAULT_X0 && after_len == before_len;
            DESCRIPTOR_WRITE_EFAULT_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-descriptor-write-smoke: syscall case=write_efault vector={} esr={:#018x} svc=0x0000 number=1 return-x0={:#018x} expected=-EFAULT console-unchanged={}",
                vector.name(),
                reported_esr,
                return_x0,
                after_len == before_len
            );
        }
        syscall::TALOS_WRITE_SYSCALL if args[3] != 0 => {
            let ok = return_x0 == DESCRIPTOR_WRITE_EXPECTED_EINVAL_X0 && after_len == before_len;
            DESCRIPTOR_WRITE_RESERVED_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-descriptor-write-smoke: syscall case=write_reserved vector={} esr={:#018x} svc=0x0000 number=1 return-x0={:#018x} expected=-EINVAL console-unchanged={}",
                vector.name(),
                reported_esr,
                return_x0,
                after_len == before_len
            );
        }
        syscall::TALOS_NOP_SYSCALL => {
            let ok = return_x0 == 0 && after_len == before_len;
            DESCRIPTOR_WRITE_TALOS_NOP_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-descriptor-write-smoke: syscall case=talos_nop vector={} esr={:#018x} svc=0x0000 number=0 return-x0={:#018x}",
                vector.name(),
                reported_esr,
                return_x0
            );
        }
        SYSCALL_SMOKE_UNKNOWN_NUMBER => {
            let ok = return_x0 == SYSCALL_SMOKE_EXPECTED_ENOSYS_X0 && after_len == before_len;
            DESCRIPTOR_WRITE_UNKNOWN_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-descriptor-write-smoke: syscall case=unknown vector={} esr={:#018x} svc=0x0000 number=17 return-x0={:#018x} expected=-ENOSYS",
                vector.name(),
                reported_esr,
                return_x0
            );
        }
        DESCRIPTOR_WRITE_COPY_PROBE_NUMBER => {
            let ok = return_x0 == SYSCALL_SMOKE_EXPECTED_ENOSYS_X0 && after_len == before_len;
            DESCRIPTOR_WRITE_COPY_PROBE_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-descriptor-write-smoke: syscall case=copy_probe_quarantine vector={} esr={:#018x} svc=0x0000 number={:#018x} return-x0={:#018x} expected=-ENOSYS dispatched=false",
                vector.name(),
                reported_esr,
                raw_number,
                return_x0
            );
        }
        _ => {
            DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            crate::println!(
                "qemu-descriptor-write-smoke: syscall case=unexpected vector={} esr={:#018x} svc=0x0000 number={:#018x} return-x0={:#018x}",
                vector.name(),
                reported_esr,
                raw_number,
                return_x0
            );
        }
    }

    true
}

#[cfg(talos_boot_scenario = "qemu_process_descriptor_stdio_smoke")]
pub fn handle_process_descriptor_stdio_smoke_exception(
    esr: u64,
    _elr: u64,
    far: u64,
    vector: ExceptionVector,
    _spsr: u64,
    saved_frame: *mut ExceptionFrame,
) -> bool {
    let marker = crate::arch::aarch64::exceptions::svc_immediate(esr);
    let reported_esr = esr & !(1 << 25);
    let Some(frame) = (unsafe { saved_frame.as_mut() }) else {
        DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
        return false;
    };

    if marker == syscall::DIAGNOSTIC_EL0_TRAP_SVC_IMMEDIATE {
        let stable = syscall::is_stable_syscall_svc_immediate(marker);
        crate::println!(
            "qemu-process-descriptor-stdio-smoke: diagnostic-marker marker=0x7a10 stable-syscall={} dispatched=false",
            stable
        );
        finish_process_descriptor_stdio_smoke(
            reported_esr == SYSCALL_SMOKE_EXPECTED_MARKER_ESR && far == 0,
        );
    }

    if reported_esr != SYSCALL_SMOKE_EXPECTED_SVC_ESR {
        DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
        return false;
    }

    let current_owner = crate::scheduler::ProcessOwnerId::new(PROCESS_DESCRIPTOR_STDIO_OWNER_RAW)
        .expect("process descriptor stdio owner id is nonzero");
    let store = unsafe { &*core::ptr::addr_of!(PROCESS_DESCRIPTOR_STDIO_STORE) };
    let Ok(descriptor_table) = store.current_descriptor_table(Some(current_owner)) else {
        DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
        return false;
    };
    let arguments = syscall::SyscallArguments::new([
        frame.reg(0),
        frame.reg(1),
        frame.reg(2),
        frame.reg(3),
        frame.reg(4),
        frame.reg(5),
    ]);
    let args = arguments.values();
    let raw_number = frame.reg(8);
    let before_len = DESCRIPTOR_WRITE_CONSOLE_LEN.load(Ordering::Relaxed) as usize;
    let mappings = [UserMapping::new(
        DESCRIPTOR_WRITE_USER_DATA_START,
        DESCRIPTOR_WRITE_USER_DATA_LEN,
        UserMappingPermissions::USER_DATA,
    )
    .expect("fixed process-descriptor stdio smoke data mapping is valid")];
    let mut scratch = [0u8; 64];
    let mut console = DescriptorWriteCaptureConsole;
    let result = syscall::dispatch_descriptor_write(
        raw_number,
        arguments,
        descriptor_table,
        &mappings,
        DESCRIPTOR_WRITE_USER_DATA_START,
        unsafe { &*core::ptr::addr_of!(DESCRIPTOR_WRITE_USER_DATA) },
        &mut scratch,
        &mut console,
    );
    let return_x0 = result.return_value().x0();
    frame.set_reg(0, return_x0);
    let after_len = DESCRIPTOR_WRITE_CONSOLE_LEN.load(Ordering::Relaxed) as usize;

    match raw_number {
        syscall::TALOS_WRITE_SYSCALL
            if args[0] == 1 && args[1] == DESCRIPTOR_WRITE_USER_DATA_START && args[3] == 0 =>
        {
            let console_ok = return_x0 == 18
                && after_len == before_len + DESCRIPTOR_WRITE_STDOUT.len()
                && descriptor_write_console_matches(before_len, DESCRIPTOR_WRITE_STDOUT);
            DESCRIPTOR_WRITE_STDOUT_OBSERVED.store(u64::from(console_ok), Ordering::Relaxed);
            if !console_ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-process-descriptor-stdio-smoke: syscall case=write_stdout vector={} esr={:#018x} svc=0x0000 number=1 args=[x0={:#018x} x1={:#018x} x2={:#018x} x3={:#018x} x4={:#018x} x5={:#018x}] descriptor-owner={:#018x} return-x0={:#018x}",
                vector.name(),
                reported_esr,
                args[0],
                args[1],
                args[2],
                args[3],
                args[4],
                args[5],
                current_owner.raw(),
                return_x0
            );
            crate::println!(
                "qemu-process-descriptor-stdio-smoke: runtime-console case=write_stdout device=runtime-console0 bytes=18 hex=74616c6f732d7374646f75742d71656d750a ok={}",
                console_ok
            );
            crate::println!(
                "qemu-process-descriptor-stdio-smoke: user-observed case=write_stdout x0={:#018x} ok={}",
                return_x0,
                console_ok
            );
        }
        syscall::TALOS_WRITE_SYSCALL
            if args[0] == 2
                && args[1]
                    == DESCRIPTOR_WRITE_USER_DATA_START + DESCRIPTOR_WRITE_STDERR_OFFSET as u64 =>
        {
            let console_ok = return_x0 == 18
                && after_len == before_len + DESCRIPTOR_WRITE_STDERR.len()
                && descriptor_write_console_matches(before_len, DESCRIPTOR_WRITE_STDERR);
            DESCRIPTOR_WRITE_STDERR_OBSERVED.store(u64::from(console_ok), Ordering::Relaxed);
            if !console_ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-process-descriptor-stdio-smoke: syscall case=write_stderr vector={} esr={:#018x} svc=0x0000 number=1 args=[x0={:#018x} x1={:#018x} x2={:#018x} x3={:#018x} x4={:#018x} x5={:#018x}] descriptor-owner={:#018x} return-x0={:#018x}",
                vector.name(),
                reported_esr,
                args[0],
                args[1],
                args[2],
                args[3],
                args[4],
                args[5],
                current_owner.raw(),
                return_x0
            );
            crate::println!(
                "qemu-process-descriptor-stdio-smoke: runtime-console case=write_stderr device=runtime-console0 bytes=18 hex=74616c6f732d7374646572722d71656d750a ok={}",
                console_ok
            );
            crate::println!(
                "qemu-process-descriptor-stdio-smoke: user-observed case=write_stderr x0={:#018x} ok={}",
                return_x0,
                console_ok
            );
        }
        syscall::TALOS_WRITE_SYSCALL if args[0] == 0 => {
            let ok = return_x0 == DESCRIPTOR_WRITE_EXPECTED_EBADF_X0 && after_len == before_len;
            DESCRIPTOR_WRITE_FD0_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-process-descriptor-stdio-smoke: syscall case=write_fd0 vector={} esr={:#018x} svc=0x0000 number=1 return-x0={:#018x} expected=-EBADF console-unchanged={}",
                vector.name(),
                reported_esr,
                return_x0,
                after_len == before_len
            );
        }
        syscall::TALOS_WRITE_SYSCALL if args[0] == 99 => {
            let ok = return_x0 == DESCRIPTOR_WRITE_EXPECTED_EBADF_X0 && after_len == before_len;
            DESCRIPTOR_WRITE_BADFD_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-process-descriptor-stdio-smoke: syscall case=write_badfd vector={} esr={:#018x} svc=0x0000 number=1 return-x0={:#018x} expected=-EBADF console-unchanged={}",
                vector.name(),
                reported_esr,
                return_x0,
                after_len == before_len
            );
        }
        syscall::TALOS_WRITE_SYSCALL if args[1] == EL0_TRAP_USER_GUARD_START => {
            let ok = return_x0 == DESCRIPTOR_WRITE_EXPECTED_EFAULT_X0 && after_len == before_len;
            DESCRIPTOR_WRITE_EFAULT_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-process-descriptor-stdio-smoke: syscall case=write_efault vector={} esr={:#018x} svc=0x0000 number=1 return-x0={:#018x} expected=-EFAULT console-unchanged={}",
                vector.name(),
                reported_esr,
                return_x0,
                after_len == before_len
            );
        }
        syscall::TALOS_WRITE_SYSCALL if args[3] != 0 => {
            let ok = return_x0 == DESCRIPTOR_WRITE_EXPECTED_EINVAL_X0 && after_len == before_len;
            DESCRIPTOR_WRITE_RESERVED_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-process-descriptor-stdio-smoke: syscall case=write_reserved vector={} esr={:#018x} svc=0x0000 number=1 return-x0={:#018x} expected=-EINVAL console-unchanged={}",
                vector.name(),
                reported_esr,
                return_x0,
                after_len == before_len
            );
        }
        syscall::TALOS_NOP_SYSCALL => {
            let ok = return_x0 == 0 && after_len == before_len;
            DESCRIPTOR_WRITE_TALOS_NOP_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-process-descriptor-stdio-smoke: syscall case=talos_nop vector={} esr={:#018x} svc=0x0000 number=0 return-x0={:#018x}",
                vector.name(),
                reported_esr,
                return_x0
            );
        }
        SYSCALL_SMOKE_UNKNOWN_NUMBER => {
            let ok = return_x0 == SYSCALL_SMOKE_EXPECTED_ENOSYS_X0 && after_len == before_len;
            DESCRIPTOR_WRITE_UNKNOWN_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-process-descriptor-stdio-smoke: syscall case=unknown vector={} esr={:#018x} svc=0x0000 number=17 return-x0={:#018x} expected=-ENOSYS",
                vector.name(),
                reported_esr,
                return_x0
            );
        }
        DESCRIPTOR_WRITE_COPY_PROBE_NUMBER => {
            let ok = return_x0 == SYSCALL_SMOKE_EXPECTED_ENOSYS_X0 && after_len == before_len;
            DESCRIPTOR_WRITE_COPY_PROBE_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-process-descriptor-stdio-smoke: syscall case=copy_probe_quarantine vector={} esr={:#018x} svc=0x0000 number={:#018x} return-x0={:#018x} expected=-ENOSYS dispatched=false",
                vector.name(),
                reported_esr,
                raw_number,
                return_x0
            );
        }
        _ => {
            DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            crate::println!(
                "qemu-process-descriptor-stdio-smoke: syscall case=unexpected vector={} esr={:#018x} svc=0x0000 number={:#018x} return-x0={:#018x}",
                vector.name(),
                reported_esr,
                raw_number,
                return_x0
            );
        }
    }

    true
}

#[cfg(talos_boot_scenario = "qemu_close_syscall_smoke")]
pub fn handle_close_syscall_smoke_exception(
    esr: u64,
    _elr: u64,
    far: u64,
    vector: ExceptionVector,
    _spsr: u64,
    saved_frame: *mut ExceptionFrame,
) -> bool {
    let marker = crate::arch::aarch64::exceptions::svc_immediate(esr);
    let reported_esr = esr & !(1 << 25);
    let Some(frame) = (unsafe { saved_frame.as_mut() }) else {
        DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
        return false;
    };

    if marker == syscall::DIAGNOSTIC_EL0_TRAP_SVC_IMMEDIATE {
        let stable = syscall::is_stable_syscall_svc_immediate(marker);
        crate::println!(
            "qemu-close-syscall-smoke: diagnostic-marker marker=0x7a10 stable-syscall={} dispatched=false",
            stable
        );
        finish_close_syscall_smoke(reported_esr == SYSCALL_SMOKE_EXPECTED_MARKER_ESR && far == 0);
    }

    if reported_esr != SYSCALL_SMOKE_EXPECTED_SVC_ESR {
        DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
        return false;
    }

    let current_owner = crate::scheduler::ProcessOwnerId::new(PROCESS_DESCRIPTOR_STDIO_OWNER_RAW)
        .expect("close syscall smoke owner id is nonzero");
    let arguments = syscall::SyscallArguments::new([
        frame.reg(0),
        frame.reg(1),
        frame.reg(2),
        frame.reg(3),
        frame.reg(4),
        frame.reg(5),
    ]);
    let args = arguments.values();
    let raw_number = frame.reg(8);
    let before_len = DESCRIPTOR_WRITE_CONSOLE_LEN.load(Ordering::Relaxed) as usize;
    let mappings = [UserMapping::new(
        DESCRIPTOR_WRITE_USER_DATA_START,
        DESCRIPTOR_WRITE_USER_DATA_LEN,
        UserMappingPermissions::USER_DATA,
    )
    .expect("fixed close syscall smoke data mapping is valid")];
    let mut scratch = [0u8; 64];
    let mut console = DescriptorWriteCaptureConsole;
    let result = {
        let store = unsafe { &mut *core::ptr::addr_of_mut!(PROCESS_DESCRIPTOR_STDIO_STORE) };
        syscall::dispatch_process_descriptor(
            raw_number,
            arguments,
            Some(current_owner),
            store,
            &mappings,
            DESCRIPTOR_WRITE_USER_DATA_START,
            unsafe { &*core::ptr::addr_of!(DESCRIPTOR_WRITE_USER_DATA) },
            &mut scratch,
            &mut console,
        )
    };
    let return_x0 = result.return_value().x0();
    frame.set_reg(0, return_x0);
    let after_len = DESCRIPTOR_WRITE_CONSOLE_LEN.load(Ordering::Relaxed) as usize;

    let store = unsafe { &*core::ptr::addr_of!(PROCESS_DESCRIPTOR_STDIO_STORE) };
    let stdout_closed = store
        .current_descriptor_table(Some(current_owner))
        .map(|table| table.get(crate::posix::STDOUT_FD).is_err())
        .unwrap_or(false);
    let stderr_open = store
        .current_descriptor_table(Some(current_owner))
        .map(|table| table.get(crate::posix::STDERR_FD).is_ok())
        .unwrap_or(false);
    let stderr_closed = store
        .current_descriptor_table(Some(current_owner))
        .map(|table| table.get(crate::posix::STDERR_FD).is_err())
        .unwrap_or(false);

    match raw_number {
        syscall::TALOS_CLOSE_SYSCALL
            if args[0] == 1
                && args[1] == 0
                && args[2] == 0
                && args[3] == 0
                && args[4] == 0
                && args[5] == 0
                && CLOSE_SYSCALL_CLOSE_STDOUT_OBSERVED.load(Ordering::Relaxed) == 0 =>
        {
            let ok = return_x0 == 0 && after_len == before_len && stdout_closed && stderr_open;
            CLOSE_SYSCALL_CLOSE_STDOUT_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-close-syscall-smoke: syscall case=close_stdout vector={} esr={:#018x} svc=0x0000 number=2 args=[x0={:#018x} x1={:#018x} x2={:#018x} x3={:#018x} x4={:#018x} x5={:#018x}] descriptor-owner={:#018x} return-x0={:#018x}",
                vector.name(),
                reported_esr,
                args[0],
                args[1],
                args[2],
                args[3],
                args[4],
                args[5],
                current_owner.raw(),
                return_x0
            );
        }
        syscall::TALOS_WRITE_SYSCALL if args[0] == 1 => {
            let ok = return_x0 == DESCRIPTOR_WRITE_EXPECTED_EBADF_X0 && after_len == before_len;
            CLOSE_SYSCALL_WRITE_CLOSED_STDOUT_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-close-syscall-smoke: syscall case=write_closed_stdout vector={} esr={:#018x} svc=0x0000 number=1 return-x0={:#018x} expected=-EBADF console-unchanged={}",
                vector.name(),
                reported_esr,
                return_x0,
                after_len == before_len
            );
        }
        syscall::TALOS_CLOSE_SYSCALL if args[0] == 2 && args[1] != 0 => {
            let table_unchanged = stderr_open;
            let ok = return_x0 == DESCRIPTOR_WRITE_EXPECTED_EINVAL_X0
                && after_len == before_len
                && table_unchanged;
            CLOSE_SYSCALL_CLOSE_STDERR_RESERVED_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-close-syscall-smoke: syscall case=close_stderr_reserved vector={} esr={:#018x} svc=0x0000 number=2 return-x0={:#018x} expected=-EINVAL descriptor={:#018x} table-unchanged={}",
                vector.name(),
                reported_esr,
                return_x0,
                args[0],
                table_unchanged
            );
        }
        syscall::TALOS_WRITE_SYSCALL
            if args[0] == 2 && CLOSE_SYSCALL_CLOSE_STDERR_OBSERVED.load(Ordering::Relaxed) == 0 =>
        {
            let console_ok = return_x0 == 18
                && after_len == before_len + DESCRIPTOR_WRITE_STDERR.len()
                && descriptor_write_console_matches(before_len, DESCRIPTOR_WRITE_STDERR);
            CLOSE_SYSCALL_WRITE_STDERR_OBSERVED.store(u64::from(console_ok), Ordering::Relaxed);
            if !console_ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-close-syscall-smoke: syscall case=write_stderr_after_stdout_close vector={} esr={:#018x} svc=0x0000 number=1 args=[x0={:#018x} x1={:#018x} x2={:#018x} x3={:#018x} x4={:#018x} x5={:#018x}] descriptor-owner={:#018x} return-x0={:#018x}",
                vector.name(),
                reported_esr,
                args[0],
                args[1],
                args[2],
                args[3],
                args[4],
                args[5],
                current_owner.raw(),
                return_x0
            );
            crate::println!(
                "qemu-close-syscall-smoke: runtime-console case=write_stderr_after_stdout_close device=runtime-console0 bytes=18 hex=74616c6f732d7374646572722d71656d750a ok={}",
                console_ok
            );
            crate::println!(
                "qemu-close-syscall-smoke: user-observed case=write_stderr_after_stdout_close x0={:#018x} ok={}",
                return_x0,
                console_ok
            );
        }
        syscall::TALOS_CLOSE_SYSCALL
            if args[0] == 2
                && args[1] == 0
                && args[2] == 0
                && args[3] == 0
                && args[4] == 0
                && args[5] == 0 =>
        {
            let ok = return_x0 == 0 && after_len == before_len && stderr_closed;
            CLOSE_SYSCALL_CLOSE_STDERR_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-close-syscall-smoke: syscall case=close_stderr vector={} esr={:#018x} svc=0x0000 number=2 args=[x0={:#018x} x1={:#018x} x2={:#018x} x3={:#018x} x4={:#018x} x5={:#018x}] descriptor-owner={:#018x} return-x0={:#018x}",
                vector.name(),
                reported_esr,
                args[0],
                args[1],
                args[2],
                args[3],
                args[4],
                args[5],
                current_owner.raw(),
                return_x0
            );
        }
        syscall::TALOS_WRITE_SYSCALL if args[0] == 2 => {
            let ok = return_x0 == DESCRIPTOR_WRITE_EXPECTED_EBADF_X0 && after_len == before_len;
            CLOSE_SYSCALL_WRITE_CLOSED_STDERR_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-close-syscall-smoke: syscall case=write_closed_stderr vector={} esr={:#018x} svc=0x0000 number=1 return-x0={:#018x} expected=-EBADF console-unchanged={}",
                vector.name(),
                reported_esr,
                return_x0,
                after_len == before_len
            );
        }
        syscall::TALOS_CLOSE_SYSCALL if args[0] == 1 => {
            let table_unchanged = stdout_closed && stderr_closed;
            let ok = return_x0 == DESCRIPTOR_WRITE_EXPECTED_EBADF_X0
                && after_len == before_len
                && table_unchanged;
            CLOSE_SYSCALL_CLOSE_STDOUT_AGAIN_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-close-syscall-smoke: syscall case=close_stdout_again vector={} esr={:#018x} svc=0x0000 number=2 return-x0={:#018x} expected=-EBADF table-unchanged={}",
                vector.name(),
                reported_esr,
                return_x0,
                table_unchanged
            );
        }
        syscall::TALOS_CLOSE_SYSCALL if args[0] == 99 => {
            let table_unchanged = stdout_closed && stderr_closed;
            let ok = return_x0 == DESCRIPTOR_WRITE_EXPECTED_EBADF_X0
                && after_len == before_len
                && table_unchanged;
            CLOSE_SYSCALL_CLOSE_BADFD_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-close-syscall-smoke: syscall case=close_badfd vector={} esr={:#018x} svc=0x0000 number=2 return-x0={:#018x} expected=-EBADF table-unchanged={}",
                vector.name(),
                reported_esr,
                return_x0,
                table_unchanged
            );
        }
        syscall::TALOS_NOP_SYSCALL => {
            let ok = return_x0 == 0 && after_len == before_len;
            CLOSE_SYSCALL_TALOS_NOP_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-close-syscall-smoke: syscall case=talos_nop vector={} esr={:#018x} svc=0x0000 number=0 return-x0={:#018x}",
                vector.name(),
                reported_esr,
                return_x0
            );
        }
        SYSCALL_SMOKE_UNKNOWN_NUMBER => {
            let ok = return_x0 == SYSCALL_SMOKE_EXPECTED_ENOSYS_X0 && after_len == before_len;
            CLOSE_SYSCALL_UNKNOWN_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-close-syscall-smoke: syscall case=unknown vector={} esr={:#018x} svc=0x0000 number=17 return-x0={:#018x} expected=-ENOSYS",
                vector.name(),
                reported_esr,
                return_x0
            );
        }
        DESCRIPTOR_WRITE_COPY_PROBE_NUMBER => {
            let ok = return_x0 == SYSCALL_SMOKE_EXPECTED_ENOSYS_X0 && after_len == before_len;
            CLOSE_SYSCALL_COPY_PROBE_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-close-syscall-smoke: syscall case=copy_probe_quarantine vector={} esr={:#018x} svc=0x0000 number={:#018x} return-x0={:#018x} expected=-ENOSYS dispatched=false",
                vector.name(),
                reported_esr,
                raw_number,
                return_x0
            );
        }
        _ => {
            DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            crate::println!(
                "qemu-close-syscall-smoke: syscall case=unexpected vector={} esr={:#018x} svc=0x0000 number={:#018x} return-x0={:#018x}",
                vector.name(),
                reported_esr,
                raw_number,
                return_x0
            );
        }
    }

    true
}

#[cfg(talos_boot_scenario = "qemu_dup_syscall_smoke")]
pub fn handle_dup_syscall_smoke_exception(
    esr: u64,
    _elr: u64,
    far: u64,
    vector: ExceptionVector,
    _spsr: u64,
    saved_frame: *mut ExceptionFrame,
) -> bool {
    let marker = crate::arch::aarch64::exceptions::svc_immediate(esr);
    let reported_esr = esr & !(1 << 25);
    let Some(frame) = (unsafe { saved_frame.as_mut() }) else {
        DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
        return false;
    };

    if marker == syscall::DIAGNOSTIC_EL0_TRAP_SVC_IMMEDIATE {
        let stable = syscall::is_stable_syscall_svc_immediate(marker);
        crate::println!(
            "qemu-dup-syscall-smoke: diagnostic-marker marker=0x7a10 stable-syscall={} dispatched=false",
            stable
        );
        finish_dup_syscall_smoke(reported_esr == SYSCALL_SMOKE_EXPECTED_MARKER_ESR && far == 0);
    }

    if reported_esr != SYSCALL_SMOKE_EXPECTED_SVC_ESR {
        DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
        return false;
    }

    let current_owner = crate::scheduler::ProcessOwnerId::new(PROCESS_DESCRIPTOR_STDIO_OWNER_RAW)
        .expect("dup syscall smoke owner id is nonzero");
    let arguments = syscall::SyscallArguments::new([
        frame.reg(0),
        frame.reg(1),
        frame.reg(2),
        frame.reg(3),
        frame.reg(4),
        frame.reg(5),
    ]);
    let args = arguments.values();
    let raw_number = frame.reg(8);
    let before_len = DESCRIPTOR_WRITE_CONSOLE_LEN.load(Ordering::Relaxed) as usize;
    let mappings = [UserMapping::new(
        DESCRIPTOR_WRITE_USER_DATA_START,
        DESCRIPTOR_WRITE_USER_DATA_LEN,
        UserMappingPermissions::USER_DATA,
    )
    .expect("fixed dup syscall smoke data mapping is valid")];
    let mut scratch = [0u8; 64];
    let mut console = DescriptorWriteCaptureConsole;
    let result = {
        let store = unsafe { &mut *core::ptr::addr_of_mut!(PROCESS_DESCRIPTOR_STDIO_STORE) };
        syscall::dispatch_process_descriptor(
            raw_number,
            arguments,
            Some(current_owner),
            store,
            &mappings,
            DESCRIPTOR_WRITE_USER_DATA_START,
            unsafe { &*core::ptr::addr_of!(DESCRIPTOR_WRITE_USER_DATA) },
            &mut scratch,
            &mut console,
        )
    };
    let return_x0 = result.return_value().x0();
    frame.set_reg(0, return_x0);
    let after_len = DESCRIPTOR_WRITE_CONSOLE_LEN.load(Ordering::Relaxed) as usize;

    let store = unsafe { &*core::ptr::addr_of!(PROCESS_DESCRIPTOR_STDIO_STORE) };
    let table = store
        .current_descriptor_table(Some(current_owner))
        .expect("dup syscall smoke current table remains present");
    let stdin_open = table.get(crate::posix::STDIN_FD).is_ok();
    let stdout_open = table.get(crate::posix::STDOUT_FD).is_ok();
    let stderr_open = table.get(crate::posix::STDERR_FD).is_ok();
    let duplicate_open = table.get(3).is_ok();
    let table_full = stdin_open && stdout_open && stderr_open && duplicate_open;
    let source_closed = !stdout_open;
    let duplicate_closed = !duplicate_open;

    match raw_number {
        syscall::TALOS_DUP_SYSCALL
            if args[0] == 1
                && args[1] == 0
                && args[2] == 0
                && args[3] == 0
                && args[4] == 0
                && args[5] == 0
                && DUP_SYSCALL_DUP_STDOUT_OBSERVED.load(Ordering::Relaxed) == 0 =>
        {
            let ok = return_x0 == 3 && after_len == before_len && stdout_open && table_full;
            DUP_SYSCALL_DUP_STDOUT_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-dup-syscall-smoke: syscall case=dup_stdout vector={} esr={:#018x} svc=0x0000 number=3 args=[x0={:#018x} x1={:#018x} x2={:#018x} x3={:#018x} x4={:#018x} x5={:#018x}] descriptor-owner={:#018x} return-x0={:#018x} lowest-free={} source-open={}",
                vector.name(),
                reported_esr,
                args[0],
                args[1],
                args[2],
                args[3],
                args[4],
                args[5],
                current_owner.raw(),
                return_x0,
                return_x0 == 3,
                stdout_open
            );
        }
        syscall::TALOS_DUP_SYSCALL
            if args[0] == 2
                && args[1] == 0
                && args[2] == 0
                && args[3] == 0
                && args[4] == 0
                && args[5] == 0 =>
        {
            let ok = return_x0 == (syscall::EMFILE as u64).wrapping_neg()
                && after_len == before_len
                && table_full;
            DUP_SYSCALL_DUP_STDERR_FULL_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-dup-syscall-smoke: syscall case=dup_stderr_full vector={} esr={:#018x} svc=0x0000 number=3 return-x0={:#018x} expected=-EMFILE table-unchanged={}",
                vector.name(),
                reported_esr,
                return_x0,
                table_full
            );
        }
        syscall::TALOS_DUP_SYSCALL if args[0] == 1 && args[1] != 0 => {
            let ok = return_x0 == DESCRIPTOR_WRITE_EXPECTED_EINVAL_X0
                && after_len == before_len
                && table_full;
            DUP_SYSCALL_DUP_STDOUT_RESERVED_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-dup-syscall-smoke: syscall case=dup_stdout_reserved vector={} esr={:#018x} svc=0x0000 number=3 return-x0={:#018x} expected=-EINVAL table-unchanged={}",
                vector.name(),
                reported_esr,
                return_x0,
                table_full
            );
        }
        syscall::TALOS_WRITE_SYSCALL
            if args[0] == 1
                && args[1] == DESCRIPTOR_WRITE_USER_DATA_START
                && DUP_SYSCALL_CLOSE_SOURCE_OBSERVED.load(Ordering::Relaxed) == 0 =>
        {
            let console_ok = return_x0 == 19
                && after_len == before_len + DUP_SYSCALL_SOURCE.len()
                && descriptor_write_console_matches(before_len, DUP_SYSCALL_SOURCE);
            DUP_SYSCALL_WRITE_SOURCE_OBSERVED.store(u64::from(console_ok), Ordering::Relaxed);
            if !console_ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-dup-syscall-smoke: syscall case=write_stdout_source vector={} esr={:#018x} svc=0x0000 number=1 args=[x0={:#018x} x1={:#018x} x2={:#018x} x3={:#018x} x4={:#018x} x5={:#018x}] descriptor-owner={:#018x} return-x0={:#018x}",
                vector.name(),
                reported_esr,
                args[0],
                args[1],
                args[2],
                args[3],
                args[4],
                args[5],
                current_owner.raw(),
                return_x0
            );
            crate::println!(
                "qemu-dup-syscall-smoke: runtime-console case=write_stdout_source device=runtime-console0 bytes=19 hex=74616c6f732d6475702d7372632d71656d750a ok={}",
                console_ok
            );
        }
        syscall::TALOS_WRITE_SYSCALL
            if args[0] == 3
                && args[1]
                    == DESCRIPTOR_WRITE_USER_DATA_START + DESCRIPTOR_WRITE_STDERR_OFFSET as u64
                && DUP_SYSCALL_CLOSE_SOURCE_OBSERVED.load(Ordering::Relaxed) == 0 =>
        {
            let console_ok = return_x0 == 19
                && after_len == before_len + DUP_SYSCALL_DUPLICATE.len()
                && descriptor_write_console_matches(before_len, DUP_SYSCALL_DUPLICATE);
            DUP_SYSCALL_WRITE_DUPLICATE_OBSERVED.store(u64::from(console_ok), Ordering::Relaxed);
            if !console_ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-dup-syscall-smoke: syscall case=write_stdout_duplicate vector={} esr={:#018x} svc=0x0000 number=1 args=[x0={:#018x} x1={:#018x} x2={:#018x} x3={:#018x} x4={:#018x} x5={:#018x}] descriptor-owner={:#018x} return-x0={:#018x}",
                vector.name(),
                reported_esr,
                args[0],
                args[1],
                args[2],
                args[3],
                args[4],
                args[5],
                current_owner.raw(),
                return_x0
            );
            crate::println!(
                "qemu-dup-syscall-smoke: runtime-console case=write_stdout_duplicate device=runtime-console0 bytes=19 hex=74616c6f732d6475702d6e65772d71656d750a ok={}",
                console_ok
            );
        }
        syscall::TALOS_CLOSE_SYSCALL if args[0] == 1 => {
            let ok = return_x0 == 0 && after_len == before_len && source_closed && duplicate_open;
            DUP_SYSCALL_CLOSE_SOURCE_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-dup-syscall-smoke: syscall case=close_stdout_source vector={} esr={:#018x} svc=0x0000 number=2 args=[x0={:#018x} x1={:#018x} x2={:#018x} x3={:#018x} x4={:#018x} x5={:#018x}] descriptor-owner={:#018x} return-x0={:#018x}",
                vector.name(),
                reported_esr,
                args[0],
                args[1],
                args[2],
                args[3],
                args[4],
                args[5],
                current_owner.raw(),
                return_x0
            );
        }
        syscall::TALOS_WRITE_SYSCALL if args[0] == 1 => {
            let ok = return_x0 == DESCRIPTOR_WRITE_EXPECTED_EBADF_X0 && after_len == before_len;
            DUP_SYSCALL_WRITE_SOURCE_CLOSED_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-dup-syscall-smoke: syscall case=write_stdout_source_after_close vector={} esr={:#018x} svc=0x0000 number=1 return-x0={:#018x} expected=-EBADF console-unchanged={}",
                vector.name(),
                reported_esr,
                return_x0,
                after_len == before_len
            );
        }
        syscall::TALOS_WRITE_SYSCALL
            if args[0] == 3
                && args[1]
                    == DESCRIPTOR_WRITE_USER_DATA_START + DESCRIPTOR_WRITE_STDERR_OFFSET as u64
                && DUP_SYSCALL_CLOSE_DUPLICATE_OBSERVED.load(Ordering::Relaxed) == 0 =>
        {
            let console_ok = return_x0 == 19
                && after_len == before_len + DUP_SYSCALL_DUPLICATE.len()
                && descriptor_write_console_matches(before_len, DUP_SYSCALL_DUPLICATE);
            DUP_SYSCALL_WRITE_DUPLICATE_AFTER_SOURCE_CLOSE_OBSERVED
                .store(u64::from(console_ok), Ordering::Relaxed);
            if !console_ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-dup-syscall-smoke: syscall case=write_duplicate_after_source_close vector={} esr={:#018x} svc=0x0000 number=1 args=[x0={:#018x} x1={:#018x} x2={:#018x} x3={:#018x} x4={:#018x} x5={:#018x}] descriptor-owner={:#018x} return-x0={:#018x}",
                vector.name(),
                reported_esr,
                args[0],
                args[1],
                args[2],
                args[3],
                args[4],
                args[5],
                current_owner.raw(),
                return_x0
            );
            crate::println!(
                "qemu-dup-syscall-smoke: runtime-console case=write_duplicate_after_source_close device=runtime-console0 bytes=19 hex=74616c6f732d6475702d6e65772d71656d750a ok={}",
                console_ok
            );
        }
        syscall::TALOS_CLOSE_SYSCALL if args[0] == 3 => {
            let ok = return_x0 == 0 && after_len == before_len && duplicate_closed;
            DUP_SYSCALL_CLOSE_DUPLICATE_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-dup-syscall-smoke: syscall case=close_stdout_duplicate vector={} esr={:#018x} svc=0x0000 number=2 args=[x0={:#018x} x1={:#018x} x2={:#018x} x3={:#018x} x4={:#018x} x5={:#018x}] descriptor-owner={:#018x} return-x0={:#018x}",
                vector.name(),
                reported_esr,
                args[0],
                args[1],
                args[2],
                args[3],
                args[4],
                args[5],
                current_owner.raw(),
                return_x0
            );
        }
        syscall::TALOS_WRITE_SYSCALL if args[0] == 3 => {
            let ok = return_x0 == DESCRIPTOR_WRITE_EXPECTED_EBADF_X0 && after_len == before_len;
            DUP_SYSCALL_WRITE_DUPLICATE_CLOSED_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-dup-syscall-smoke: syscall case=write_duplicate_after_duplicate_close vector={} esr={:#018x} svc=0x0000 number=1 return-x0={:#018x} expected=-EBADF console-unchanged={}",
                vector.name(),
                reported_esr,
                return_x0,
                after_len == before_len
            );
        }
        syscall::TALOS_DUP_SYSCALL if args[0] == 1 => {
            let table_unchanged = source_closed && duplicate_closed;
            let ok = return_x0 == DESCRIPTOR_WRITE_EXPECTED_EBADF_X0
                && after_len == before_len
                && table_unchanged;
            DUP_SYSCALL_DUP_CLOSED_SOURCE_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-dup-syscall-smoke: syscall case=dup_closed_source vector={} esr={:#018x} svc=0x0000 number=3 return-x0={:#018x} expected=-EBADF table-unchanged={}",
                vector.name(),
                reported_esr,
                return_x0,
                table_unchanged
            );
        }
        syscall::TALOS_NOP_SYSCALL => {
            let ok = return_x0 == 0 && after_len == before_len;
            DUP_SYSCALL_TALOS_NOP_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-dup-syscall-smoke: syscall case=talos_nop vector={} esr={:#018x} svc=0x0000 number=0 return-x0={:#018x}",
                vector.name(),
                reported_esr,
                return_x0
            );
        }
        SYSCALL_SMOKE_UNKNOWN_NUMBER => {
            let ok = return_x0 == SYSCALL_SMOKE_EXPECTED_ENOSYS_X0 && after_len == before_len;
            DUP_SYSCALL_UNKNOWN_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-dup-syscall-smoke: syscall case=unknown vector={} esr={:#018x} svc=0x0000 number=17 return-x0={:#018x} expected=-ENOSYS",
                vector.name(),
                reported_esr,
                return_x0
            );
        }
        DESCRIPTOR_WRITE_COPY_PROBE_NUMBER => {
            let ok = return_x0 == SYSCALL_SMOKE_EXPECTED_ENOSYS_X0 && after_len == before_len;
            DUP_SYSCALL_COPY_PROBE_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-dup-syscall-smoke: syscall case=copy_probe_quarantine vector={} esr={:#018x} svc=0x0000 number={:#018x} return-x0={:#018x} expected=-ENOSYS dispatched=false",
                vector.name(),
                reported_esr,
                raw_number,
                return_x0
            );
        }
        _ => {
            DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            crate::println!(
                "qemu-dup-syscall-smoke: syscall case=unexpected vector={} esr={:#018x} svc=0x0000 number={:#018x} return-x0={:#018x}",
                vector.name(),
                reported_esr,
                raw_number,
                return_x0
            );
        }
    }

    true
}

#[cfg(talos_boot_scenario = "qemu_read_stdin_smoke")]
fn read_stdin_user_matches(start: usize, expected: &[u8]) -> bool {
    let Some(end) = start.checked_add(expected.len()) else {
        return false;
    };
    if end > DESCRIPTOR_WRITE_USER_DATA_LEN {
        return false;
    }
    let data = unsafe { &*core::ptr::addr_of!(DESCRIPTOR_WRITE_USER_DATA) };
    &data[start..end] == expected
}

#[cfg(talos_boot_scenario = "qemu_read_stdin_smoke")]
fn read_stdin_user_zero(start: usize, len: usize) -> bool {
    let Some(end) = start.checked_add(len) else {
        return false;
    };
    if end > DESCRIPTOR_WRITE_USER_DATA_LEN {
        return false;
    }
    let data = unsafe { &*core::ptr::addr_of!(DESCRIPTOR_WRITE_USER_DATA) };
    data[start..end].iter().all(|byte| *byte == 0)
}

#[cfg(talos_boot_scenario = "qemu_read_stdin_smoke")]
fn read_stdin_user_all_zero() -> bool {
    let data = unsafe { &*core::ptr::addr_of!(DESCRIPTOR_WRITE_USER_DATA) };
    data.iter().all(|byte| *byte == 0)
}

#[cfg(talos_boot_scenario = "qemu_read_stdin_smoke")]
pub fn handle_read_stdin_smoke_exception(
    esr: u64,
    _elr: u64,
    far: u64,
    vector: ExceptionVector,
    _spsr: u64,
    saved_frame: *mut ExceptionFrame,
) -> bool {
    let marker = crate::arch::aarch64::exceptions::svc_immediate(esr);
    let reported_esr = esr & !(1 << 25);
    let Some(frame) = (unsafe { saved_frame.as_mut() }) else {
        READ_STDIN_ERRORS.fetch_add(1, Ordering::Relaxed);
        return false;
    };

    if marker == syscall::DIAGNOSTIC_EL0_TRAP_SVC_IMMEDIATE {
        let stable = syscall::is_stable_syscall_svc_immediate(marker);
        crate::println!(
            "qemu-read-stdin-smoke: diagnostic-marker marker=0x7a10 stable-syscall={} dispatched=false",
            stable
        );
        finish_read_stdin_smoke(reported_esr == SYSCALL_SMOKE_EXPECTED_MARKER_ESR && far == 0);
    }

    if reported_esr != SYSCALL_SMOKE_EXPECTED_SVC_ESR {
        READ_STDIN_ERRORS.fetch_add(1, Ordering::Relaxed);
        return false;
    }

    let current_owner = crate::scheduler::ProcessOwnerId::new(PROCESS_DESCRIPTOR_STDIO_OWNER_RAW)
        .expect("read stdin smoke owner id is nonzero");
    let arguments = syscall::SyscallArguments::new([
        frame.reg(0),
        frame.reg(1),
        frame.reg(2),
        frame.reg(3),
        frame.reg(4),
        frame.reg(5),
    ]);
    let args = arguments.values();
    let raw_number = frame.reg(8);
    let mappings = [UserMapping::new(
        DESCRIPTOR_WRITE_USER_DATA_START,
        DESCRIPTOR_WRITE_USER_DATA_LEN,
        UserMappingPermissions::USER_DATA,
    )
    .expect("fixed read stdin smoke data mapping is valid")];
    let mut scratch = [0u8; 64];
    let mut console = DescriptorWriteCaptureConsole;
    let result = {
        let store = unsafe { &mut *core::ptr::addr_of_mut!(PROCESS_DESCRIPTOR_STDIO_STORE) };
        let user_memory = unsafe { &mut *core::ptr::addr_of_mut!(DESCRIPTOR_WRITE_USER_DATA) };
        let fixed_stdin = unsafe { &mut *core::ptr::addr_of_mut!(READ_STDIN_FIXED_STATE) };
        syscall::dispatch_process_descriptor_with_fixed_stdin(
            raw_number,
            arguments,
            Some(current_owner),
            store,
            &mappings,
            DESCRIPTOR_WRITE_USER_DATA_START,
            user_memory,
            &mut scratch,
            &mut console,
            Some(fixed_stdin),
        )
    };
    let return_x0 = result.return_value().x0();
    frame.set_reg(0, return_x0);
    let fixed_stdin_cursor = unsafe { &*core::ptr::addr_of!(READ_STDIN_FIXED_STATE) }.cursor();
    let store = unsafe { &*core::ptr::addr_of!(PROCESS_DESCRIPTOR_STDIO_STORE) };
    let table = store
        .current_descriptor_table(Some(current_owner))
        .expect("read stdin smoke current table remains present");
    let stdin_open = table.get(crate::posix::STDIN_FD).is_ok();
    let duplicate_open = table.get(3).is_ok();

    match raw_number {
        syscall::TALOS_DUP_SYSCALL
            if args == [0, 0, 0, 0, 0, 0]
                && READ_STDIN_DUP_STDIN_OBSERVED.load(Ordering::Relaxed) == 0 =>
        {
            let ok = return_x0 == 3 && stdin_open && duplicate_open;
            READ_STDIN_DUP_STDIN_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                READ_STDIN_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-read-stdin-smoke: syscall case=dup_stdin vector={} esr={:#018x} svc=0x0000 number=3 args=[x0={:#018x} x1={:#018x} x2={:#018x} x3={:#018x} x4={:#018x} x5={:#018x}] descriptor-owner={:#018x} return-x0={:#018x} lowest-free={} source-open={}",
                vector.name(),
                reported_esr,
                args[0],
                args[1],
                args[2],
                args[3],
                args[4],
                args[5],
                current_owner.raw(),
                return_x0,
                return_x0 == 3,
                stdin_open
            );
        }
        syscall::TALOS_READ_SYSCALL
            if args[0] == 0
                && args[1] == EL0_TRAP_USER_GUARD_START
                && args[2] == 5
                && args[3] == 0
                && args[4] == 0
                && args[5] == 0 =>
        {
            let user_unchanged = read_stdin_user_all_zero();
            let ok = return_x0 == DESCRIPTOR_WRITE_EXPECTED_EFAULT_X0
                && fixed_stdin_cursor == 0
                && user_unchanged;
            READ_STDIN_GUARD_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                READ_STDIN_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-read-stdin-smoke: syscall case=read_guard vector={} esr={:#018x} svc=0x0000 number=4 return-x0={:#018x} expected=-EFAULT fixed-stdin-cursor={} user-unchanged={}",
                vector.name(),
                reported_esr,
                return_x0,
                fixed_stdin_cursor,
                user_unchanged
            );
        }
        syscall::TALOS_READ_SYSCALL
            if args[0] == 0
                && args[1] == DESCRIPTOR_WRITE_USER_DATA_START + 0x80
                && args[2] == 5
                && args[3] == 1
                && args[4] == 0
                && args[5] == 0 =>
        {
            let user_unchanged = read_stdin_user_all_zero();
            let ok = return_x0 == DESCRIPTOR_WRITE_EXPECTED_EINVAL_X0
                && fixed_stdin_cursor == 0
                && user_unchanged;
            READ_STDIN_RESERVED_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                READ_STDIN_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-read-stdin-smoke: syscall case=read_reserved vector={} esr={:#018x} svc=0x0000 number=4 return-x0={:#018x} expected=-EINVAL fixed-stdin-cursor={} user-unchanged={}",
                vector.name(),
                reported_esr,
                return_x0,
                fixed_stdin_cursor,
                user_unchanged
            );
        }
        syscall::TALOS_READ_SYSCALL
            if args[0] == 1
                && args[1] == DESCRIPTOR_WRITE_USER_DATA_START + 0x80
                && args[2] == 5
                && args[3] == 0
                && args[4] == 0
                && args[5] == 0 =>
        {
            let user_unchanged = read_stdin_user_all_zero();
            let ok = return_x0 == DESCRIPTOR_WRITE_EXPECTED_EBADF_X0
                && fixed_stdin_cursor == 0
                && user_unchanged;
            READ_STDIN_FD1_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                READ_STDIN_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-read-stdin-smoke: syscall case=read_fd1 vector={} esr={:#018x} svc=0x0000 number=4 return-x0={:#018x} expected=-EBADF fixed-stdin-cursor={} user-unchanged={}",
                vector.name(),
                reported_esr,
                return_x0,
                fixed_stdin_cursor,
                user_unchanged
            );
        }
        syscall::TALOS_READ_SYSCALL
            if args[0] == 99
                && args[1] == DESCRIPTOR_WRITE_USER_DATA_START + 0x80
                && args[2] == 5
                && args[3] == 0
                && args[4] == 0
                && args[5] == 0 =>
        {
            let user_unchanged = read_stdin_user_all_zero();
            let ok = return_x0 == DESCRIPTOR_WRITE_EXPECTED_EBADF_X0
                && fixed_stdin_cursor == 0
                && user_unchanged;
            READ_STDIN_BADFD_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                READ_STDIN_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-read-stdin-smoke: syscall case=read_badfd vector={} esr={:#018x} svc=0x0000 number=4 return-x0={:#018x} expected=-EBADF fixed-stdin-cursor={} user-unchanged={}",
                vector.name(),
                reported_esr,
                return_x0,
                fixed_stdin_cursor,
                user_unchanged
            );
        }
        syscall::TALOS_READ_SYSCALL
            if args[0] == 0
                && args[1] == DESCRIPTOR_WRITE_USER_DATA_START + 0x80
                && args[2] == 5
                && args[3] == 0
                && args[4] == 0
                && args[5] == 0 =>
        {
            let ok = return_x0 == 5
                && fixed_stdin_cursor == 5
                && read_stdin_user_matches(0x80, b"talos");
            READ_STDIN_FIRST_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                READ_STDIN_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-read-stdin-smoke: syscall case=read_stdin_first vector={} esr={:#018x} svc=0x0000 number=4 args=[x0={:#018x} x1={:#018x} x2={:#018x} x3={:#018x} x4={:#018x} x5={:#018x}] descriptor-owner={:#018x} return-x0={:#018x} fixed-stdin-cursor={}",
                vector.name(),
                reported_esr,
                args[0],
                args[1],
                args[2],
                args[3],
                args[4],
                args[5],
                current_owner.raw(),
                return_x0,
                fixed_stdin_cursor
            );
            crate::println!(
                "qemu-read-stdin-smoke: user-buffer case=read_stdin_first addr={:#018x} bytes=5 hex=74616c6f73 ok={}",
                args[1],
                read_stdin_user_matches(0x80, b"talos")
            );
            crate::println!(
                "qemu-read-stdin-smoke: user-observed case=read_stdin_first x0={:#018x} ok={}",
                return_x0,
                ok
            );
        }
        syscall::TALOS_READ_SYSCALL
            if args[0] == 3
                && args[1] == DESCRIPTOR_WRITE_USER_DATA_START + 0xa0
                && args[2] == 32
                && args[3] == 0
                && args[4] == 0
                && args[5] == 0 =>
        {
            let ok = return_x0 == 12
                && fixed_stdin_cursor == READ_STDIN_FIXED_BYTES.len()
                && read_stdin_user_matches(0xa0, b"-stdin-qemu\n");
            READ_STDIN_DUPLICATE_REMAINING_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                READ_STDIN_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-read-stdin-smoke: syscall case=read_stdin_duplicate_remaining vector={} esr={:#018x} svc=0x0000 number=4 args=[x0={:#018x} x1={:#018x} x2={:#018x} x3={:#018x} x4={:#018x} x5={:#018x}] descriptor-owner={:#018x} return-x0={:#018x} fixed-stdin-cursor={} short-read={}",
                vector.name(),
                reported_esr,
                args[0],
                args[1],
                args[2],
                args[3],
                args[4],
                args[5],
                current_owner.raw(),
                return_x0,
                fixed_stdin_cursor,
                return_x0 < args[2]
            );
            crate::println!(
                "qemu-read-stdin-smoke: user-buffer case=read_stdin_duplicate_remaining addr={:#018x} bytes=12 hex=2d737464696e2d71656d750a ok={}",
                args[1],
                read_stdin_user_matches(0xa0, b"-stdin-qemu\n")
            );
            crate::println!(
                "qemu-read-stdin-smoke: user-observed case=read_stdin_duplicate_remaining x0={:#018x} ok={}",
                return_x0,
                ok
            );
        }
        syscall::TALOS_READ_SYSCALL
            if args[0] == 0
                && args[1] == DESCRIPTOR_WRITE_USER_DATA_START + 0xc0
                && args[2] == 1
                && args[3] == 0
                && args[4] == 0
                && args[5] == 0 =>
        {
            let user_unchanged = read_stdin_user_zero(0xc0, 1);
            let ok = return_x0 == 0
                && fixed_stdin_cursor == READ_STDIN_FIXED_BYTES.len()
                && user_unchanged;
            READ_STDIN_EOF_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                READ_STDIN_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-read-stdin-smoke: syscall case=read_stdin_eof vector={} esr={:#018x} svc=0x0000 number=4 return-x0={:#018x} fixed-stdin-cursor={} user-unchanged={} eof=true",
                vector.name(),
                reported_esr,
                return_x0,
                fixed_stdin_cursor,
                user_unchanged
            );
        }
        syscall::TALOS_NOP_SYSCALL => {
            let ok = return_x0 == 0;
            READ_STDIN_TALOS_NOP_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                READ_STDIN_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-read-stdin-smoke: syscall case=talos_nop vector={} esr={:#018x} svc=0x0000 number=0 return-x0={:#018x}",
                vector.name(),
                reported_esr,
                return_x0
            );
        }
        SYSCALL_SMOKE_UNKNOWN_NUMBER => {
            let ok = return_x0 == SYSCALL_SMOKE_EXPECTED_ENOSYS_X0;
            READ_STDIN_UNKNOWN_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                READ_STDIN_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-read-stdin-smoke: syscall case=unknown vector={} esr={:#018x} svc=0x0000 number=17 return-x0={:#018x} expected=-ENOSYS",
                vector.name(),
                reported_esr,
                return_x0
            );
        }
        DESCRIPTOR_WRITE_COPY_PROBE_NUMBER => {
            let ok = return_x0 == SYSCALL_SMOKE_EXPECTED_ENOSYS_X0;
            READ_STDIN_COPY_PROBE_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                READ_STDIN_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "qemu-read-stdin-smoke: syscall case=copy_probe_quarantine vector={} esr={:#018x} svc=0x0000 number={:#018x} return-x0={:#018x} expected=-ENOSYS dispatched=false",
                vector.name(),
                reported_esr,
                raw_number,
                return_x0
            );
        }
        _ => {
            READ_STDIN_ERRORS.fetch_add(1, Ordering::Relaxed);
            crate::println!(
                "qemu-read-stdin-smoke: syscall case=unexpected vector={} esr={:#018x} svc=0x0000 number={:#018x} return-x0={:#018x}",
                vector.name(),
                reported_esr,
                raw_number,
                return_x0
            );
        }
    }

    true
}

#[cfg(talos_boot_scenario = "qemu_pointer_copy_smoke")]
fn pointer_copy_user_data_replaced() -> bool {
    let data = unsafe { &*core::ptr::addr_of!(POINTER_COPY_USER_DATA) };
    data[..16]
        .iter()
        .all(|byte| *byte == POINTER_COPY_USER_DATA_REPLACEMENT)
}

#[cfg(talos_boot_scenario = "qemu_descriptor_write_smoke")]
fn descriptor_write_console_matches(start: usize, expected: &[u8]) -> bool {
    let len = DESCRIPTOR_WRITE_CONSOLE_LEN.load(Ordering::Relaxed) as usize;
    let Some(end) = start.checked_add(expected.len()) else {
        return false;
    };
    if end > len || end > 64 {
        return false;
    }
    let capture = unsafe { &*core::ptr::addr_of!(DESCRIPTOR_WRITE_CONSOLE_CAPTURE) };
    &capture[start..end] == expected
}

#[cfg(talos_boot_scenario = "qemu_descriptor_write_smoke")]
fn finish_descriptor_write_smoke(marker_ok: bool) -> ! {
    if !marker_ok {
        DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
    }
    let invalid_descriptor_observed = DESCRIPTOR_WRITE_FD0_OBSERVED.load(Ordering::Relaxed) == 1
        && DESCRIPTOR_WRITE_BADFD_OBSERVED.load(Ordering::Relaxed) == 1;
    let participants = DESCRIPTOR_WRITE_STDOUT_OBSERVED.load(Ordering::Relaxed)
        + DESCRIPTOR_WRITE_STDERR_OBSERVED.load(Ordering::Relaxed)
        + u64::from(invalid_descriptor_observed)
        + DESCRIPTOR_WRITE_EFAULT_OBSERVED.load(Ordering::Relaxed)
        + DESCRIPTOR_WRITE_RESERVED_OBSERVED.load(Ordering::Relaxed)
        + DESCRIPTOR_WRITE_TALOS_NOP_OBSERVED.load(Ordering::Relaxed)
        + DESCRIPTOR_WRITE_UNKNOWN_OBSERVED.load(Ordering::Relaxed)
        + DESCRIPTOR_WRITE_COPY_PROBE_OBSERVED.load(Ordering::Relaxed);
    let errors = DESCRIPTOR_WRITE_ERRORS.load(Ordering::Relaxed);
    let complete = participants == 8 && errors == 0;
    let classification = if complete {
        "qemu-descriptor-write-smoke-complete"
    } else {
        "qemu-descriptor-write-smoke-failed"
    };

    crate::println!(
        "qemu-descriptor-write-smoke: final participants={} expected=8 errors={} classification={}",
        participants,
        errors,
        classification
    );
    if complete {
        crate::println!("qemu-descriptor-write-smoke: PASS");
        crate::target::qemu::exit_success();
    }
    crate::target::qemu::exit_failure();
}

#[cfg(talos_boot_scenario = "qemu_process_descriptor_stdio_smoke")]
fn finish_process_descriptor_stdio_smoke(marker_ok: bool) -> ! {
    if !marker_ok {
        DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
    }
    let invalid_descriptor_observed = DESCRIPTOR_WRITE_FD0_OBSERVED.load(Ordering::Relaxed) == 1
        && DESCRIPTOR_WRITE_BADFD_OBSERVED.load(Ordering::Relaxed) == 1;
    let participants = DESCRIPTOR_WRITE_STDOUT_OBSERVED.load(Ordering::Relaxed)
        + DESCRIPTOR_WRITE_STDERR_OBSERVED.load(Ordering::Relaxed)
        + u64::from(invalid_descriptor_observed)
        + DESCRIPTOR_WRITE_EFAULT_OBSERVED.load(Ordering::Relaxed)
        + DESCRIPTOR_WRITE_RESERVED_OBSERVED.load(Ordering::Relaxed)
        + DESCRIPTOR_WRITE_TALOS_NOP_OBSERVED.load(Ordering::Relaxed)
        + DESCRIPTOR_WRITE_UNKNOWN_OBSERVED.load(Ordering::Relaxed)
        + DESCRIPTOR_WRITE_COPY_PROBE_OBSERVED.load(Ordering::Relaxed);
    let errors = DESCRIPTOR_WRITE_ERRORS.load(Ordering::Relaxed);
    let complete = participants == 8 && errors == 0;
    let classification = if complete {
        "qemu-process-descriptor-stdio-smoke-complete"
    } else {
        "qemu-process-descriptor-stdio-smoke-failed"
    };

    crate::println!(
        "qemu-process-descriptor-stdio-smoke: final participants={} expected=8 errors={} classification={}",
        participants,
        errors,
        classification
    );
    if complete {
        crate::println!("qemu-process-descriptor-stdio-smoke: PASS");
        crate::target::qemu::exit_success();
    }
    crate::target::qemu::exit_failure();
}

#[cfg(talos_boot_scenario = "qemu_close_syscall_smoke")]
fn finish_close_syscall_smoke(marker_ok: bool) -> ! {
    if !marker_ok {
        DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
    }
    let participants = CLOSE_SYSCALL_CLOSE_STDOUT_OBSERVED.load(Ordering::Relaxed)
        + CLOSE_SYSCALL_WRITE_CLOSED_STDOUT_OBSERVED.load(Ordering::Relaxed)
        + CLOSE_SYSCALL_CLOSE_STDERR_RESERVED_OBSERVED.load(Ordering::Relaxed)
        + CLOSE_SYSCALL_WRITE_STDERR_OBSERVED.load(Ordering::Relaxed)
        + CLOSE_SYSCALL_CLOSE_STDERR_OBSERVED.load(Ordering::Relaxed)
        + CLOSE_SYSCALL_WRITE_CLOSED_STDERR_OBSERVED.load(Ordering::Relaxed)
        + CLOSE_SYSCALL_CLOSE_STDOUT_AGAIN_OBSERVED.load(Ordering::Relaxed)
        + CLOSE_SYSCALL_CLOSE_BADFD_OBSERVED.load(Ordering::Relaxed)
        + CLOSE_SYSCALL_TALOS_NOP_OBSERVED.load(Ordering::Relaxed)
        + CLOSE_SYSCALL_UNKNOWN_OBSERVED.load(Ordering::Relaxed)
        + CLOSE_SYSCALL_COPY_PROBE_OBSERVED.load(Ordering::Relaxed);
    let errors = DESCRIPTOR_WRITE_ERRORS.load(Ordering::Relaxed);
    let complete = participants == 11 && errors == 0;
    let classification = if complete {
        "qemu-close-syscall-smoke-complete"
    } else {
        "qemu-close-syscall-smoke-failed"
    };

    crate::println!(
        "qemu-close-syscall-smoke: final participants={} expected=11 errors={} classification={}",
        participants,
        errors,
        classification
    );
    if complete {
        crate::println!("qemu-close-syscall-smoke: PASS");
        crate::target::qemu::exit_success();
    }
    crate::target::qemu::exit_failure();
}

#[cfg(talos_boot_scenario = "qemu_dup_syscall_smoke")]
fn finish_dup_syscall_smoke(marker_ok: bool) -> ! {
    if !marker_ok {
        DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
    }
    let participants = DUP_SYSCALL_DUP_STDOUT_OBSERVED.load(Ordering::Relaxed)
        + DUP_SYSCALL_DUP_STDERR_FULL_OBSERVED.load(Ordering::Relaxed)
        + DUP_SYSCALL_DUP_STDOUT_RESERVED_OBSERVED.load(Ordering::Relaxed)
        + DUP_SYSCALL_WRITE_SOURCE_OBSERVED.load(Ordering::Relaxed)
        + DUP_SYSCALL_WRITE_DUPLICATE_OBSERVED.load(Ordering::Relaxed)
        + DUP_SYSCALL_CLOSE_SOURCE_OBSERVED.load(Ordering::Relaxed)
        + DUP_SYSCALL_WRITE_SOURCE_CLOSED_OBSERVED.load(Ordering::Relaxed)
        + DUP_SYSCALL_WRITE_DUPLICATE_AFTER_SOURCE_CLOSE_OBSERVED.load(Ordering::Relaxed)
        + DUP_SYSCALL_CLOSE_DUPLICATE_OBSERVED.load(Ordering::Relaxed)
        + DUP_SYSCALL_WRITE_DUPLICATE_CLOSED_OBSERVED.load(Ordering::Relaxed)
        + DUP_SYSCALL_DUP_CLOSED_SOURCE_OBSERVED.load(Ordering::Relaxed)
        + DUP_SYSCALL_TALOS_NOP_OBSERVED.load(Ordering::Relaxed)
        + DUP_SYSCALL_UNKNOWN_OBSERVED.load(Ordering::Relaxed)
        + DUP_SYSCALL_COPY_PROBE_OBSERVED.load(Ordering::Relaxed);
    let errors = DESCRIPTOR_WRITE_ERRORS.load(Ordering::Relaxed);
    let complete = participants == 14 && errors == 0;
    let classification = if complete {
        "qemu-dup-syscall-smoke-complete"
    } else {
        "qemu-dup-syscall-smoke-failed"
    };

    crate::println!(
        "qemu-dup-syscall-smoke: final participants={} expected=14 errors={} classification={}",
        participants,
        errors,
        classification
    );
    if complete {
        crate::println!("qemu-dup-syscall-smoke: PASS");
        crate::target::qemu::exit_success();
    }
    crate::target::qemu::exit_failure();
}

#[cfg(talos_boot_scenario = "qemu_read_stdin_smoke")]
fn finish_read_stdin_smoke(marker_ok: bool) -> ! {
    if !marker_ok {
        READ_STDIN_ERRORS.fetch_add(1, Ordering::Relaxed);
    }
    let participants = READ_STDIN_DUP_STDIN_OBSERVED.load(Ordering::Relaxed)
        + READ_STDIN_GUARD_OBSERVED.load(Ordering::Relaxed)
        + READ_STDIN_RESERVED_OBSERVED.load(Ordering::Relaxed)
        + READ_STDIN_FD1_OBSERVED.load(Ordering::Relaxed)
        + READ_STDIN_BADFD_OBSERVED.load(Ordering::Relaxed)
        + READ_STDIN_FIRST_OBSERVED.load(Ordering::Relaxed)
        + READ_STDIN_DUPLICATE_REMAINING_OBSERVED.load(Ordering::Relaxed)
        + READ_STDIN_EOF_OBSERVED.load(Ordering::Relaxed)
        + READ_STDIN_TALOS_NOP_OBSERVED.load(Ordering::Relaxed)
        + READ_STDIN_UNKNOWN_OBSERVED.load(Ordering::Relaxed)
        + READ_STDIN_COPY_PROBE_OBSERVED.load(Ordering::Relaxed);
    let errors = READ_STDIN_ERRORS.load(Ordering::Relaxed);
    let complete = participants == 11 && errors == 0;
    let classification = if complete {
        "qemu-read-stdin-smoke-complete"
    } else {
        "qemu-read-stdin-smoke-failed"
    };

    crate::println!(
        "qemu-read-stdin-smoke: final participants={} expected=11 errors={} classification={}",
        participants,
        errors,
        classification
    );
    if complete {
        crate::println!("qemu-read-stdin-smoke: PASS");
        crate::target::qemu::exit_success();
    }
    crate::target::qemu::exit_failure();
}

#[cfg(talos_boot_scenario = "qemu_pointer_copy_smoke")]
fn finish_pointer_copy_smoke(marker_ok: bool) -> ! {
    if !marker_ok {
        POINTER_COPY_ERRORS.fetch_add(1, Ordering::Relaxed);
    }
    let participants = POINTER_COPY_SUCCESS_OBSERVED.load(Ordering::Relaxed)
        + POINTER_COPY_EFAULT_OBSERVED.load(Ordering::Relaxed)
        + POINTER_COPY_UNKNOWN_OBSERVED.load(Ordering::Relaxed);
    let errors = POINTER_COPY_ERRORS.load(Ordering::Relaxed);
    let complete = participants == 3 && errors == 0;
    let classification = if complete {
        "qemu-pointer-copy-smoke-complete"
    } else {
        "qemu-pointer-copy-smoke-failed"
    };

    crate::println!(
        "qemu-pointer-copy-smoke: final participants={} expected=3 errors={} classification={}",
        participants,
        errors,
        classification
    );
    if complete {
        crate::println!("qemu-pointer-copy-smoke: PASS");
        crate::target::qemu::exit_success();
    }
    crate::target::qemu::exit_failure();
}

#[cfg(talos_boot_scenario = "qemu_syscall_smoke")]
fn finish_syscall_smoke(marker_ok: bool) -> ! {
    if !marker_ok {
        SYSCALL_SMOKE_ERRORS.fetch_add(1, Ordering::Relaxed);
    }
    let participants = SYSCALL_SMOKE_TALOS_NOP_OBSERVED.load(Ordering::Relaxed)
        + SYSCALL_SMOKE_UNKNOWN_OBSERVED.load(Ordering::Relaxed);
    let errors = SYSCALL_SMOKE_ERRORS.load(Ordering::Relaxed);
    let complete = participants == 2 && errors == 0;
    let classification = if complete {
        "qemu-syscall-smoke-complete"
    } else {
        "qemu-syscall-smoke-failed"
    };

    crate::println!(
        "qemu-syscall-smoke: final participants={} expected=2 errors={} classification={}",
        participants,
        errors,
        classification
    );
    if complete {
        crate::println!("qemu-syscall-smoke: PASS");
        crate::target::qemu::exit_success();
    }
    crate::target::qemu::exit_failure();
}

#[cfg(talos_boot_scenario = "qemu_el0_trap_smoke")]
pub fn handle_el0_trap_smoke_exception(
    esr: u64,
    elr: u64,
    far: u64,
    vector: ExceptionVector,
    spsr: u64,
    saved_frame: *const ExceptionFrame,
) -> ! {
    let marker = esr & 0xffff;
    let reported_esr = esr & !(1 << 25);
    let user_sp = unsafe { read_sp_el0() };
    let frame_available = !saved_frame.is_null();
    let ok = vector == ExceptionVector::LowerAarch64Sync
        && reported_esr == EL0_TRAP_EXPECTED_ESR
        && marker == EL0_TRAP_SVC_MARKER
        && EL0_TRAP_USER_TEXT_START <= elr
        && elr < EL0_TRAP_USER_TEXT_START + EL0_TRAP_USER_TEXT_LEN as u64
        && EL0_TRAP_USER_STACK_START <= user_sp
        && user_sp <= EL0_TRAP_USER_STACK_START + EL0_TRAP_USER_STACK_LEN as u64;

    crate::println!(
        "qemu-el0-trap-smoke: trap vector={} esr={:#018x} far={:#018x} elr={:#018x} sp={:#018x} spsr={:#018x} marker={:#x}",
        vector.name(),
        reported_esr,
        far,
        elr,
        user_sp,
        spsr,
        marker
    );
    crate::println!("qemu-el0-trap-smoke: raw-esr={:#018x}", esr);
    crate::println!(
        "qemu-el0-trap-smoke: frame available={} x0={:#018x} x1={:#018x}",
        frame_available,
        unsafe { saved_frame.as_ref().map(|frame| frame.reg(0)).unwrap_or(0) },
        unsafe { saved_frame.as_ref().map(|frame| frame.reg(1)).unwrap_or(0) }
    );

    if ok {
        crate::println!(
            "qemu-el0-trap-smoke: final participants=1 expected=1 errors=0 classification=qemu-el0-trap-smoke-complete"
        );
        crate::println!("qemu-el0-trap-smoke: PASS");
        crate::target::qemu::exit_success();
    }

    crate::println!(
        "qemu-el0-trap-smoke: final participants=0 expected=1 errors=1 classification=qemu-el0-trap-smoke-failed"
    );
    crate::target::qemu::exit_failure();
}

#[cfg(any(
    talos_boot_scenario = "qemu_el0_trap_smoke",
    talos_boot_scenario = "qemu_syscall_smoke",
    talos_boot_scenario = "qemu_pointer_copy_smoke",
    talos_boot_scenario = "qemu_descriptor_write_smoke"
))]
unsafe fn install_el0_trap_smoke_tables() {
    const TABLE_DESC: u64 = 0b11;
    const PAGE_DESC: u64 = 0b11;
    const BLOCK_DESC: u64 = 0b01;
    const ATTR_NORMAL: u64 = 0;
    const ATTR_DEVICE: u64 = 1;
    const ATTR_SHIFT: u64 = 2;
    const AP_EL0_RW: u64 = 0b01 << 6;
    const AP_EL0_RO: u64 = 0b11 << 6;
    const AF: u64 = 1 << 10;
    const SH_INNER: u64 = 0b11 << 8;
    const PXN: u64 = 1 << 53;
    const UXN: u64 = 1 << 54;
    const ADDR_MASK_4K: u64 = 0x0000_ffff_ffff_f000;
    const ADDR_MASK_2M: u64 = 0x0000_ffff_ffe0_0000;
    const ADDR_MASK_1G: u64 = 0x0000_ffff_c000_0000;

    let root = unsafe { core::ptr::addr_of_mut!(EL0_TRAP_ROOT_TABLE.0) };
    let l1 = unsafe { core::ptr::addr_of_mut!(EL0_TRAP_L1_TABLE.0) };
    let low_l2 = unsafe { core::ptr::addr_of_mut!(EL0_TRAP_LOW_L2_TABLE.0) };
    let low_l3 = unsafe { core::ptr::addr_of_mut!(EL0_TRAP_LOW_L3_TABLE.0) };
    let payload_pa = core::ptr::addr_of!(EL0_TRAP_PAYLOAD.0) as u64;
    #[cfg(talos_boot_scenario = "qemu_pointer_copy_smoke")]
    let user_data_pa = core::ptr::addr_of!(POINTER_COPY_USER_DATA) as u64;
    #[cfg(talos_boot_scenario = "qemu_descriptor_write_smoke")]
    let descriptor_user_data_pa = core::ptr::addr_of!(DESCRIPTOR_WRITE_USER_DATA) as u64;
    let stack_pa = unsafe { core::ptr::addr_of!(EL0_TRAP_STACK.0) as u64 };

    unsafe {
        core::ptr::write_bytes(root.cast::<u8>(), 0, 4096);
        core::ptr::write_bytes(l1.cast::<u8>(), 0, 4096);
        core::ptr::write_bytes(low_l2.cast::<u8>(), 0, 4096);
        core::ptr::write_bytes(low_l3.cast::<u8>(), 0, 4096);

        (*root)[0] = (l1 as u64 & ADDR_MASK_4K) | TABLE_DESC;
        (*l1)[0] = (low_l2 as u64 & ADDR_MASK_4K) | TABLE_DESC;
        (*l1)[1] =
            (0x4000_0000 & ADDR_MASK_1G) | (ATTR_NORMAL << ATTR_SHIFT) | SH_INNER | AF | BLOCK_DESC;

        let mut index = 1usize;
        while index < 512 {
            let base = (index as u64) << 21;
            (*low_l2)[index] =
                (base & ADDR_MASK_2M) | (ATTR_DEVICE << ATTR_SHIFT) | AF | PXN | UXN | BLOCK_DESC;
            index += 1;
        }
        (*low_l2)[0] = (low_l3 as u64 & ADDR_MASK_4K) | TABLE_DESC;

        (*low_l3)[(EL0_TRAP_USER_TEXT_START as usize) >> 12] = (payload_pa & ADDR_MASK_4K)
            | (ATTR_NORMAL << ATTR_SHIFT)
            | AP_EL0_RO
            | SH_INNER
            | AF
            | PAGE_DESC;

        #[cfg(talos_boot_scenario = "qemu_pointer_copy_smoke")]
        {
            (*low_l3)[(POINTER_COPY_USER_DATA_START as usize) >> 12] = (user_data_pa
                & ADDR_MASK_4K)
                | (ATTR_NORMAL << ATTR_SHIFT)
                | AP_EL0_RW
                | SH_INNER
                | AF
                | UXN
                | PAGE_DESC;
        }

        #[cfg(talos_boot_scenario = "qemu_descriptor_write_smoke")]
        {
            (*low_l3)[(DESCRIPTOR_WRITE_USER_DATA_START as usize) >> 12] = (descriptor_user_data_pa
                & ADDR_MASK_4K)
                | (ATTR_NORMAL << ATTR_SHIFT)
                | AP_EL0_RW
                | SH_INNER
                | AF
                | UXN
                | PAGE_DESC;
        }

        let mut page = 0usize;
        while page < EL0_TRAP_USER_STACK_LEN / 4096 {
            let va = EL0_TRAP_USER_STACK_START as usize + page * 4096;
            let pa = stack_pa + (page * 4096) as u64;
            (*low_l3)[va >> 12] = (pa & ADDR_MASK_4K)
                | (ATTR_NORMAL << ATTR_SHIFT)
                | AP_EL0_RW
                | SH_INNER
                | AF
                | UXN
                | PAGE_DESC;
            page += 1;
        }
    }

    unsafe {
        core::arch::asm!("dsb sy", "isb", options(nostack, preserves_flags));
    }
}

#[cfg(any(
    talos_boot_scenario = "qemu_el0_trap_smoke",
    talos_boot_scenario = "qemu_syscall_smoke",
    talos_boot_scenario = "qemu_pointer_copy_smoke",
    talos_boot_scenario = "qemu_descriptor_write_smoke"
))]
unsafe fn enable_el2_and_el0_translation() {
    const MAIR_NORMAL_WBWA: u64 = 0xff;
    const MAIR_DEVICE_NGNRE: u64 = 0x04;
    const TCR_T0SZ_SHIFT: u64 = 0;
    const TCR_IRGN0_SHIFT: u64 = 8;
    const TCR_ORGN0_SHIFT: u64 = 10;
    const TCR_SH0_SHIFT: u64 = 12;
    const TCR_TG0_4K: u64 = 0b00 << 14;
    const TCR_PS_SHIFT: u64 = 16;
    const TCR_CACHE_WBWA: u64 = 0b01;
    const TCR_SH_INNER: u64 = 0b11;
    const TCR_PS_48BIT: u64 = 0b101;
    const SCTLR_M: u64 = 1 << 0;
    const SCTLR_C: u64 = 1 << 2;
    const SCTLR_I: u64 = 1 << 12;
    const HCR_RW: u64 = 1 << 31;

    let mair = MAIR_NORMAL_WBWA | (MAIR_DEVICE_NGNRE << 8);
    let tcr = ((64 - 48) << TCR_T0SZ_SHIFT)
        | (TCR_CACHE_WBWA << TCR_IRGN0_SHIFT)
        | (TCR_CACHE_WBWA << TCR_ORGN0_SHIFT)
        | (TCR_SH_INNER << TCR_SH0_SHIFT)
        | TCR_TG0_4K
        | (TCR_PS_48BIT << TCR_PS_SHIFT);
    let ttbr0 = unsafe { core::ptr::addr_of!(EL0_TRAP_ROOT_TABLE.0) as u64 };
    let hcr: u64;
    let mut sctlr: u64;

    unsafe {
        core::arch::asm!(
            "mrs {hcr}, HCR_EL2",
            hcr = out(reg) hcr,
            options(nostack, preserves_flags)
        );
        let hcr = hcr | HCR_RW;
        core::arch::asm!(
            "msr HCR_EL2, {hcr}",
            "isb",
            hcr = in(reg) hcr,
            options(nostack, preserves_flags)
        );
        core::arch::asm!(
            "msr MAIR_EL2, {mair}",
            "msr TCR_EL2, {tcr}",
            "msr TTBR0_EL2, {ttbr0}",
            "isb",
            "tlbi alle2",
            "dsb sy",
            "isb",
            mair = in(reg) mair,
            tcr = in(reg) tcr,
            ttbr0 = in(reg) ttbr0,
            options(nostack, preserves_flags)
        );
        core::arch::asm!(
            "mrs {sctlr}, SCTLR_EL2",
            sctlr = out(reg) sctlr,
            options(nostack, preserves_flags)
        );
        sctlr |= SCTLR_M | SCTLR_C | SCTLR_I;
        core::arch::asm!(
            "msr SCTLR_EL2, {sctlr}",
            "dsb sy",
            "isb",
            sctlr = in(reg) sctlr,
            options(nostack, preserves_flags)
        );
    }
}

#[cfg(any(
    talos_boot_scenario = "qemu_el0_trap_smoke",
    talos_boot_scenario = "qemu_syscall_smoke",
    talos_boot_scenario = "qemu_pointer_copy_smoke",
    talos_boot_scenario = "qemu_descriptor_write_smoke"
))]
unsafe fn enable_el1_and_el0_translation() {
    const MAIR_NORMAL_WBWA: u64 = 0xff;
    const MAIR_DEVICE_NGNRE: u64 = 0x04;
    const TCR_T0SZ_SHIFT: u64 = 0;
    const TCR_IRGN0_SHIFT: u64 = 8;
    const TCR_ORGN0_SHIFT: u64 = 10;
    const TCR_SH0_SHIFT: u64 = 12;
    const TCR_TG0_4K: u64 = 0b00 << 14;
    const TCR_IPS_SHIFT: u64 = 32;
    const TCR_CACHE_WBWA: u64 = 0b01;
    const TCR_SH_INNER: u64 = 0b11;
    const TCR_IPS_48BIT: u64 = 0b101;
    const SCTLR_M: u64 = 1 << 0;
    const SCTLR_C: u64 = 1 << 2;
    const SCTLR_I: u64 = 1 << 12;

    let mair = MAIR_NORMAL_WBWA | (MAIR_DEVICE_NGNRE << 8);
    let tcr = ((64 - 48) << TCR_T0SZ_SHIFT)
        | (TCR_CACHE_WBWA << TCR_IRGN0_SHIFT)
        | (TCR_CACHE_WBWA << TCR_ORGN0_SHIFT)
        | (TCR_SH_INNER << TCR_SH0_SHIFT)
        | TCR_TG0_4K
        | (TCR_IPS_48BIT << TCR_IPS_SHIFT);
    let ttbr0 = unsafe { core::ptr::addr_of!(EL0_TRAP_ROOT_TABLE.0) as u64 };
    let vbar = aarch64::current_vbar();
    let mut sctlr: u64;

    unsafe {
        core::arch::asm!(
            "msr MAIR_EL1, {mair}",
            "msr TCR_EL1, {tcr}",
            "msr TTBR0_EL1, {ttbr0}",
            "msr VBAR_EL1, {vbar}",
            "isb",
            "tlbi vmalle1",
            "dsb sy",
            "isb",
            mair = in(reg) mair,
            tcr = in(reg) tcr,
            ttbr0 = in(reg) ttbr0,
            vbar = in(reg) vbar,
            options(nostack, preserves_flags)
        );
        core::arch::asm!(
            "mrs {sctlr}, SCTLR_EL1",
            sctlr = out(reg) sctlr,
            options(nostack, preserves_flags)
        );
        sctlr |= SCTLR_M | SCTLR_C | SCTLR_I;
        core::arch::asm!(
            "msr SCTLR_EL1, {sctlr}",
            "dsb sy",
            "isb",
            sctlr = in(reg) sctlr,
            options(nostack, preserves_flags)
        );
    }
}

#[cfg(talos_boot_scenario = "qemu_el0_trap_smoke")]
unsafe fn read_sp_el0() -> u64 {
    let value: u64;
    unsafe {
        core::arch::asm!(
            "mrs {value}, SP_EL0",
            value = out(reg) value,
            options(nomem, nostack, preserves_flags)
        );
    }
    value
}

#[cfg(talos_boot_scenario = "qemu_smp_lock_contention")]
impl SmpLockContentionState {
    const fn new() -> Self {
        Self {
            shared_counter: 0,
            per_core_counts: [0; MAX_CORES],
            error_count: 0,
        }
    }
}

#[cfg(talos_boot_scenario = "qemu_smp_lock_contention")]
static SMP_LOCK_CONTENTION_STATE: SpinLock<SmpLockContentionState> =
    SpinLock::new(SmpLockContentionState::new());

#[cfg(talos_boot_scenario = "qemu_smp_lock_contention")]
fn reset_smp_lock_contention_state() {
    let mut state = SMP_LOCK_CONTENTION_STATE.lock();
    *state = SmpLockContentionState::new();
}

#[cfg(talos_boot_scenario = "qemu_smp_lock_contention")]
fn run_smp_lock_contention_secondary(core_state: &smp::PerCoreState, logical_cpu: usize) {
    core_state.mark_workload_running();
    core_state.clean_to_poc();

    let mut progress = 0;
    while progress < SMP_LOCK_CONTENTION_TARGET_PER_CORE {
        let expected_after = {
            let mut state = SMP_LOCK_CONTENTION_STATE.lock();
            let before = state.shared_counter;
            state.shared_counter = before + 1;
            state.per_core_counts[logical_cpu] += 1;
            if state.shared_counter != before + 1 {
                state.error_count += 1;
            }
            state.per_core_counts[logical_cpu]
        };
        progress += 1;
        if expected_after != progress {
            let mut state = SMP_LOCK_CONTENTION_STATE.lock();
            state.error_count += 1;
        }
        core_state.record_workload_progress(progress);
        if progress == SMP_LOCK_CONTENTION_TARGET_PER_CORE || progress & 0xf == 0 {
            core_state.clean_to_poc();
        }
        smp_full_barrier();
        core::hint::spin_loop();
    }

    core_state.mark_workload_complete(progress);
    core_state.clean_to_poc();
}

#[cfg(talos_boot_scenario = "qemu_secondary_core_workload")]
pub fn run_secondary_core_workload_smoke() -> bool {
    smp::reset_secondary_core_states();

    let boot_mpidr = aarch64::mpidr_el1();
    let boot_affinity = aarch64::mpidr_affinity(boot_mpidr);
    let boot_logical = qemu_logical_cpu_from_mpidr_affinity(boot_affinity);
    let entry = talos_aarch64_qemu_secondary_entry as *const () as usize;
    let stack_layout = secondary_stack_layout();
    let stack_base = core::ptr::addr_of!(talos_secondary_core_stacks) as usize;
    let stack_end = core::ptr::addr_of!(talos_secondary_core_stacks_end) as usize;

    crate::println!(
        "qemu-secondary-core-workload: start conduit=smc cores={} target={} boot-mpidr={:#018x} boot-affinity={:#x} boot-logical={:?} entry={:#018x} stack-range=[{:#018x},{:#018x})",
        MAX_CORES,
        SECONDARY_CORE_WORKLOAD_TARGET,
        boot_mpidr,
        boot_affinity,
        boot_logical,
        entry,
        stack_base,
        stack_end
    );

    let mut cpu_on_ok = true;
    for logical_cpu in 1..MAX_CORES {
        let target_affinity = logical_cpu as u64;
        let result = unsafe { psci_cpu_on_smc(target_affinity, entry, logical_cpu) };
        crate::println!(
            "qemu-secondary-core-workload: cpu-on logical={} target-affinity={:#x} result={}",
            logical_cpu,
            target_affinity,
            result
        );
        cpu_on_ok &= result == 0;
    }

    let mut remaining = QEMU_SECONDARY_WAIT_LIMIT;
    while remaining > 0 {
        let all_complete = (1..MAX_CORES).all(|logical_cpu| {
            SECONDARY_CORE_STATES[logical_cpu]
                .snapshot(logical_cpu)
                .lifecycle
                >= CoreLifecycle::WorkloadComplete
        });
        if all_complete {
            break;
        }
        core::hint::spin_loop();
        remaining -= 1;
    }

    let mut reports_ok = cpu_on_ok && boot_logical == Some(0);
    for logical_cpu in 1..MAX_CORES {
        let report = SECONDARY_CORE_STATES[logical_cpu].snapshot(logical_cpu);
        let logical_from_mpidr = qemu_logical_cpu_from_mpidr_affinity(report.affinity);
        let stack_slot = stack_layout
            .slot(logical_cpu)
            .expect("stack slot for possible QEMU core");
        let stack_owned = stack_slot.contains_stack_pointer(report.stack_pointer);
        let report_ok = report.lifecycle >= CoreLifecycle::WorkloadComplete
            && report.context == logical_cpu
            && logical_from_mpidr == Some(logical_cpu)
            && stack_owned
            && report.workload_progress == SECONDARY_CORE_WORKLOAD_TARGET;
        reports_ok &= report_ok;

        crate::println!(
            "qemu-secondary-core-workload: report logical={} state={} context={} mpidr={:#018x} affinity={:#x} mapped={:?} sp={:#018x} stack=[{:#018x},{:#018x}) progress={} target={} ok={}",
            logical_cpu,
            secondary_state_name(report.lifecycle.raw()),
            report.context,
            report.mpidr,
            report.affinity,
            logical_from_mpidr,
            report.stack_pointer,
            stack_slot.bottom,
            stack_slot.top,
            report.workload_progress,
            SECONDARY_CORE_WORKLOAD_TARGET,
            report_ok
        );
    }

    let classification = if reports_ok {
        "qemu-secondary-core-controlled-workload-complete"
    } else if cpu_on_ok {
        "qemu-secondary-core-workload-incomplete"
    } else {
        "qemu-psci-smc-cpu-on-failed"
    };
    crate::println!(
        "qemu-secondary-core-workload: wait-remaining={} classification={}",
        remaining,
        classification
    );

    if reports_ok {
        crate::println!("qemu-secondary-core-workload: PASS");
    } else {
        crate::println!("qemu-secondary-core-workload: FAIL");
    }

    reports_ok
}

#[cfg(talos_boot_scenario = "qemu_smp_lock_contention")]
pub fn run_smp_lock_contention_smoke() -> bool {
    smp::reset_secondary_core_states();
    reset_smp_lock_contention_state();

    let boot_mpidr = aarch64::mpidr_el1();
    let boot_affinity = aarch64::mpidr_affinity(boot_mpidr);
    let boot_logical = qemu_logical_cpu_from_mpidr_affinity(boot_affinity);
    let entry = talos_aarch64_qemu_secondary_entry as *const () as usize;
    let stack_layout = secondary_stack_layout();
    let stack_base = core::ptr::addr_of!(talos_secondary_core_stacks) as usize;
    let stack_end = core::ptr::addr_of!(talos_secondary_core_stacks_end) as usize;
    let expected_total = SMP_LOCK_CONTENTION_TARGET_PER_CORE * (MAX_CORES as u64 - 1);

    crate::println!(
        "qemu-smp-lock-contention: start conduit=smc cores={} target-per-core={} expected-total={} boot-mpidr={:#018x} boot-affinity={:#x} boot-logical={:?} entry={:#018x} stack-range=[{:#018x},{:#018x})",
        MAX_CORES,
        SMP_LOCK_CONTENTION_TARGET_PER_CORE,
        expected_total,
        boot_mpidr,
        boot_affinity,
        boot_logical,
        entry,
        stack_base,
        stack_end
    );

    let mut cpu_on_ok = true;
    for logical_cpu in 1..MAX_CORES {
        let target_affinity = logical_cpu as u64;
        let result = unsafe { psci_cpu_on_smc(target_affinity, entry, logical_cpu) };
        crate::println!(
            "qemu-smp-lock-contention: cpu-on logical={} target-affinity={:#x} result={}",
            logical_cpu,
            target_affinity,
            result
        );
        cpu_on_ok &= result == 0;
    }

    let mut remaining = QEMU_SECONDARY_WAIT_LIMIT;
    while remaining > 0 {
        let all_complete = (1..MAX_CORES).all(|logical_cpu| {
            SECONDARY_CORE_STATES[logical_cpu]
                .snapshot(logical_cpu)
                .lifecycle
                >= CoreLifecycle::WorkloadComplete
        });
        if all_complete {
            break;
        }
        core::hint::spin_loop();
        remaining -= 1;
    }

    let final_state = SMP_LOCK_CONTENTION_STATE.try_lock().map(|state| *state);
    let lock_available = final_state.is_some();
    let final_state = final_state.unwrap_or_else(SmpLockContentionState::new);
    let mut participants = 0;
    let mut reports_ok = cpu_on_ok
        && boot_logical == Some(0)
        && lock_available
        && final_state.shared_counter == expected_total
        && final_state.error_count == 0;

    for logical_cpu in 1..MAX_CORES {
        let report = SECONDARY_CORE_STATES[logical_cpu].snapshot(logical_cpu);
        let logical_from_mpidr = qemu_logical_cpu_from_mpidr_affinity(report.affinity);
        let stack_slot = stack_layout
            .slot(logical_cpu)
            .expect("stack slot for possible QEMU core");
        let stack_owned = stack_slot.contains_stack_pointer(report.stack_pointer);
        let locked_count = final_state.per_core_counts[logical_cpu];
        let report_ok = report.lifecycle >= CoreLifecycle::WorkloadComplete
            && report.context == logical_cpu
            && logical_from_mpidr == Some(logical_cpu)
            && stack_owned
            && report.workload_progress == SMP_LOCK_CONTENTION_TARGET_PER_CORE
            && locked_count == SMP_LOCK_CONTENTION_TARGET_PER_CORE;
        if locked_count == SMP_LOCK_CONTENTION_TARGET_PER_CORE {
            participants += 1;
        }
        reports_ok &= report_ok;

        crate::println!(
            "qemu-smp-lock-contention: report logical={} state={} context={} mpidr={:#018x} affinity={:#x} mapped={:?} sp={:#018x} stack=[{:#018x},{:#018x}) lock-count={} progress={} target={} ok={}",
            logical_cpu,
            secondary_state_name(report.lifecycle.raw()),
            report.context,
            report.mpidr,
            report.affinity,
            logical_from_mpidr,
            report.stack_pointer,
            stack_slot.bottom,
            stack_slot.top,
            locked_count,
            report.workload_progress,
            SMP_LOCK_CONTENTION_TARGET_PER_CORE,
            report_ok
        );
    }

    let classification = if reports_ok {
        "qemu-smp-lock-contention-complete"
    } else if !lock_available {
        "qemu-smp-lock-contention-lock-still-held"
    } else if cpu_on_ok {
        "qemu-smp-lock-contention-invariant-failed"
    } else {
        "qemu-psci-smc-cpu-on-failed"
    };
    crate::println!(
        "qemu-smp-lock-contention: final counter={} expected={} participants={} errors={} lock-available={} wait-remaining={} classification={}",
        final_state.shared_counter,
        expected_total,
        participants,
        final_state.error_count,
        lock_available,
        remaining,
        classification
    );

    if reports_ok {
        crate::println!("qemu-smp-lock-contention: PASS");
    } else {
        crate::println!("qemu-smp-lock-contention: FAIL");
    }

    reports_ok
}

#[cfg(talos_boot_scenario = "qemu_per_core_scheduler_ownership")]
pub fn run_per_core_scheduler_ownership_smoke() -> bool {
    smp::reset_secondary_core_states();
    reset_per_core_scheduler_ownership_state();

    let boot_mpidr = aarch64::mpidr_el1();
    let boot_affinity = aarch64::mpidr_affinity(boot_mpidr);
    let boot_logical = qemu_logical_cpu_from_mpidr_affinity(boot_affinity);
    let entry = talos_aarch64_qemu_secondary_entry as *const () as usize;
    let stack_layout = secondary_stack_layout();
    let stack_base = core::ptr::addr_of!(talos_secondary_core_stacks) as usize;
    let stack_end = core::ptr::addr_of!(talos_secondary_core_stacks_end) as usize;

    crate::println!(
        "qemu-per-core-scheduler-ownership: start conduit=smc cores={} target={} boot-mpidr={:#018x} boot-affinity={:#x} boot-logical={:?} entry={:#018x} stack-range=[{:#018x},{:#018x})",
        MAX_CORES,
        PER_CORE_SCHEDULER_PROGRESS_TARGET,
        boot_mpidr,
        boot_affinity,
        boot_logical,
        entry,
        stack_base,
        stack_end
    );

    let irq_mask_probe = run_single_core_irq_mask_probe();
    publish_per_core_scheduler_report(0, build_boot_scheduler_report());

    let mut cpu_on_ok = true;
    for logical_cpu in 1..MAX_CORES {
        let target_affinity = logical_cpu as u64;
        let result = unsafe { psci_cpu_on_smc(target_affinity, entry, logical_cpu) };
        crate::println!(
            "qemu-per-core-scheduler-ownership: cpu-on logical={} target-affinity={:#x} result={}",
            logical_cpu,
            target_affinity,
            result
        );
        cpu_on_ok &= result == 0;
    }

    let mut remaining = QEMU_SECONDARY_WAIT_LIMIT;
    while remaining > 0 {
        let all_complete = (1..MAX_CORES).all(|logical_cpu| {
            SECONDARY_CORE_STATES[logical_cpu]
                .snapshot(logical_cpu)
                .lifecycle
                >= CoreLifecycle::WorkloadComplete
        });
        if all_complete {
            break;
        }
        core::hint::spin_loop();
        remaining -= 1;
    }

    let final_state = PER_CORE_SCHEDULER_OWNERSHIP_STATE
        .try_lock()
        .map(|state| *state);
    let lock_available = final_state.is_some();
    let final_state = final_state.unwrap_or_else(PerCoreSchedulerOwnershipState::new);
    let mut participants = 0;
    let mut errors = 0;
    let mut reports_ok = cpu_on_ok && boot_logical == Some(0) && lock_available;

    for logical_cpu in 0..MAX_CORES {
        let report = final_state.reports[logical_cpu];
        let (lifecycle, context, mapped, stack_owned) = if logical_cpu == 0 {
            (CoreLifecycle::WorkloadComplete, 0, boot_logical, true)
        } else {
            let core_report = SECONDARY_CORE_STATES[logical_cpu].snapshot(logical_cpu);
            let logical_from_mpidr = qemu_logical_cpu_from_mpidr_affinity(core_report.affinity);
            let stack_slot = stack_layout
                .slot(logical_cpu)
                .expect("stack slot for possible QEMU core");
            (
                core_report.lifecycle,
                core_report.context,
                logical_from_mpidr,
                stack_slot.contains_stack_pointer(core_report.stack_pointer),
            )
        };

        let role_ok = if logical_cpu == 0 {
            report.role == SchedulerCoreRole::BootCpuProduction
                && report.production_dispatch_enabled
                && !report.dispatch_deferred
                && report.current_task == 1
        } else {
            report.role == SchedulerCoreRole::SecondaryDeferred
                && !report.production_dispatch_enabled
                && report.dispatch_deferred
                && report.current_task == 0
        };
        let report_ok = lifecycle >= CoreLifecycle::WorkloadComplete
            && context == logical_cpu
            && mapped == Some(logical_cpu)
            && stack_owned
            && report.owner == logical_cpu as u64
            && role_ok
            && report.queue_len == 0
            && report.front_task == 0
            && report.progress == PER_CORE_SCHEDULER_PROGRESS_TARGET
            && report.state_transitions == PER_CORE_SCHEDULER_PROGRESS_TARGET
            && final_state.lock_progress[logical_cpu] == PER_CORE_SCHEDULER_PROGRESS_TARGET
            && report.errors == 0;
        if report_ok {
            participants += 1;
        }
        errors += report.errors;
        reports_ok &= report_ok;

        crate::println!(
            "qemu-per-core-scheduler-ownership: report logical={} state={} context={} mapped={:?} owner={} role={} production={} current={} queue-len={} front={} progress={} transitions={} dispatch-deferred={} lock-progress={} irq-ok={} errors={} ok={}",
            logical_cpu,
            secondary_state_name(lifecycle.raw()),
            context,
            mapped,
            report.owner,
            scheduler_role_name(report.role),
            report.production_dispatch_enabled,
            report.current_task,
            report.queue_len,
            report.front_task,
            report.progress,
            report.state_transitions,
            report.dispatch_deferred,
            final_state.lock_progress[logical_cpu],
            irq_mask_probe.passed(),
            report.errors,
            report_ok
        );
    }

    let classification = if reports_ok && irq_mask_probe.passed() {
        "qemu-per-core-scheduler-ownership-complete"
    } else if !lock_available {
        "qemu-per-core-scheduler-ownership-lock-still-held"
    } else if cpu_on_ok {
        "qemu-per-core-scheduler-ownership-invariant-failed"
    } else {
        "qemu-psci-smc-cpu-on-failed"
    };
    crate::println!(
        "qemu-per-core-scheduler-ownership: final participants={} expected={} errors={} lock-available={} irq-ok={} wait-remaining={} classification={}",
        participants,
        MAX_CORES,
        errors,
        lock_available,
        irq_mask_probe.passed(),
        remaining,
        classification
    );

    if reports_ok && irq_mask_probe.passed() {
        crate::println!("qemu-per-core-scheduler-ownership: PASS");
    } else {
        crate::println!("qemu-per-core-scheduler-ownership: FAIL");
    }

    reports_ok && irq_mask_probe.passed()
}

#[cfg(talos_boot_scenario = "qemu_production_secondary_dispatch")]
pub fn run_production_secondary_dispatch_smoke() -> bool {
    smp::reset_secondary_core_states();
    reset_production_secondary_dispatch_state();

    let boot_mpidr = aarch64::mpidr_el1();
    let boot_affinity = aarch64::mpidr_affinity(boot_mpidr);
    let boot_logical = qemu_logical_cpu_from_mpidr_affinity(boot_affinity);
    let entry = talos_aarch64_qemu_secondary_entry as *const () as usize;
    let stack_layout = secondary_stack_layout();
    let stack_base = core::ptr::addr_of!(talos_secondary_core_stacks) as usize;
    let stack_end = core::ptr::addr_of!(talos_secondary_core_stacks_end) as usize;

    crate::println!(
        "qemu-production-secondary-dispatch: start conduit=smc cores={} target={} boot-mpidr={:#018x} boot-affinity={:#x} boot-logical={:?} entry={:#018x} stack-range=[{:#018x},{:#018x})",
        MAX_CORES,
        PRODUCTION_SECONDARY_DISPATCH_PROGRESS_TARGET,
        boot_mpidr,
        boot_affinity,
        boot_logical,
        entry,
        stack_base,
        stack_end
    );

    let mut cpu_on_ok = true;
    for logical_cpu in 1..MAX_CORES {
        let target_affinity = logical_cpu as u64;
        let result = unsafe { psci_cpu_on_smc(target_affinity, entry, logical_cpu) };
        crate::println!(
            "qemu-production-secondary-dispatch: cpu-on logical={} target-affinity={:#x} result={}",
            logical_cpu,
            target_affinity,
            result
        );
        cpu_on_ok &= result == 0;
    }

    let mut remaining = QEMU_SECONDARY_WAIT_LIMIT;
    while remaining > 0 {
        let all_complete = (1..MAX_CORES).all(|logical_cpu| {
            SECONDARY_CORE_STATES[logical_cpu]
                .snapshot(logical_cpu)
                .lifecycle
                >= CoreLifecycle::WorkloadComplete
        });
        if all_complete {
            break;
        }
        core::hint::spin_loop();
        remaining -= 1;
    }

    let final_state = PRODUCTION_SECONDARY_DISPATCH_STATE
        .try_lock()
        .map(|state| *state);
    let lock_available = final_state.is_some();
    let final_state = final_state.unwrap_or_else(ProductionSecondaryDispatchState::new);
    let mut participants = 0;
    let mut errors = 0;
    let mut reports_ok = cpu_on_ok && boot_logical == Some(0) && lock_available;

    for logical_cpu in 1..MAX_CORES {
        let report = final_state.reports[logical_cpu];
        let core_report = SECONDARY_CORE_STATES[logical_cpu].snapshot(logical_cpu);
        let logical_from_mpidr = qemu_logical_cpu_from_mpidr_affinity(core_report.affinity);
        let stack_slot = stack_layout
            .slot(logical_cpu)
            .expect("stack slot for possible QEMU core");
        let stack_owned = stack_slot.contains_stack_pointer(core_report.stack_pointer);
        let expected_current =
            (logical_cpu as u64 + 1) * 100 + PRODUCTION_SECONDARY_DISPATCH_PROGRESS_TARGET;
        let role_ok = report.role == SchedulerCoreRole::SecondaryProductionDiagnostic
            && report.production_dispatch_enabled
            && report.current_task == expected_current;
        let report_ok = core_report.lifecycle >= CoreLifecycle::WorkloadComplete
            && core_report.context == logical_cpu
            && logical_from_mpidr == Some(logical_cpu)
            && stack_owned
            && report.owner == logical_cpu as u64
            && role_ok
            && report.queue_len == 0
            && report.front_task == 0
            && report.progress == PRODUCTION_SECONDARY_DISPATCH_PROGRESS_TARGET
            && report.state_transitions == PRODUCTION_SECONDARY_DISPATCH_PROGRESS_TARGET * 2
            && report.production_dispatches == PRODUCTION_SECONDARY_DISPATCH_PROGRESS_TARGET
            && report.context_switches == PRODUCTION_SECONDARY_DISPATCH_PROGRESS_TARGET
            && final_state.lock_progress[logical_cpu]
                == PRODUCTION_SECONDARY_DISPATCH_PROGRESS_TARGET
            && report.cross_owner_rejected
            && report.cross_owner_dispatch_rejected
            && report.errors == 0;
        if report_ok {
            participants += 1;
        }
        errors += report.errors;
        reports_ok &= report_ok;

        crate::println!(
            "qemu-production-secondary-dispatch: report logical={} state={} context={} mapped={:?} owner={} role={} production={} current={} queue-len={} front={} progress={} transitions={} production-dispatches={} context-switches={} cross-owner-rejected={} cross-owner-dispatch-rejected={} lock-progress={} errors={} ok={}",
            logical_cpu,
            secondary_state_name(core_report.lifecycle.raw()),
            core_report.context,
            logical_from_mpidr,
            report.owner,
            scheduler_role_name(report.role),
            report.production_dispatch_enabled,
            report.current_task,
            report.queue_len,
            report.front_task,
            report.progress,
            report.state_transitions,
            report.production_dispatches,
            report.context_switches,
            report.cross_owner_rejected,
            report.cross_owner_dispatch_rejected,
            final_state.lock_progress[logical_cpu],
            report.errors,
            report_ok
        );
    }

    let classification = if reports_ok {
        "qemu-production-secondary-dispatch-complete"
    } else if !lock_available {
        "qemu-production-secondary-dispatch-lock-still-held"
    } else if cpu_on_ok {
        "qemu-production-secondary-dispatch-invariant-failed"
    } else {
        "qemu-psci-smc-cpu-on-failed"
    };
    crate::println!(
        "qemu-production-secondary-dispatch: final participants={} expected={} errors={} lock-available={} wait-remaining={} classification={}",
        participants,
        MAX_CORES - 1,
        errors,
        lock_available,
        remaining,
        classification
    );

    if reports_ok {
        crate::println!("qemu-production-secondary-dispatch: PASS");
    } else {
        crate::println!("qemu-production-secondary-dispatch: FAIL");
    }

    reports_ok
}

#[cfg(talos_boot_scenario = "qemu_shared_scheduler_metadata")]
pub fn run_shared_scheduler_metadata_smoke() -> bool {
    smp::reset_secondary_core_states();
    reset_shared_scheduler_metadata_smoke_state();

    let boot_mpidr = aarch64::mpidr_el1();
    let boot_affinity = aarch64::mpidr_affinity(boot_mpidr);
    let boot_logical = qemu_logical_cpu_from_mpidr_affinity(boot_affinity);
    let entry = talos_aarch64_qemu_secondary_entry as *const () as usize;
    let stack_layout = secondary_stack_layout();
    let stack_base = core::ptr::addr_of!(talos_secondary_core_stacks) as usize;
    let stack_end = core::ptr::addr_of!(talos_secondary_core_stacks_end) as usize;

    crate::println!(
        "qemu-shared-scheduler-metadata: start conduit=smc cores={} task-capacity={} boot-mpidr={:#018x} boot-affinity={:#x} boot-logical={:?} entry={:#018x} stack-range=[{:#018x},{:#018x})",
        MAX_CORES,
        SHARED_SCHEDULER_METADATA_TASK_CAPACITY,
        boot_mpidr,
        boot_affinity,
        boot_logical,
        entry,
        stack_base,
        stack_end
    );

    let mut boot_scheduler = PerCoreScheduler::<2>::boot_cpu();
    let boot_report = build_shared_scheduler_metadata_report(0, &mut boot_scheduler);
    publish_shared_scheduler_metadata_report(0, boot_report);

    let mut cpu_on_ok = true;
    for logical_cpu in 1..MAX_CORES {
        let target_affinity = logical_cpu as u64;
        let result = unsafe { psci_cpu_on_smc(target_affinity, entry, logical_cpu) };
        crate::println!(
            "qemu-shared-scheduler-metadata: cpu-on logical={} target-affinity={:#x} result={}",
            logical_cpu,
            target_affinity,
            result
        );
        cpu_on_ok &= result == 0;
    }

    let mut remaining = SHARED_SCHEDULER_METADATA_WAIT_LIMIT;
    while remaining > 0 {
        let all_complete = (1..MAX_CORES).all(|logical_cpu| {
            SECONDARY_CORE_STATES[logical_cpu]
                .snapshot(logical_cpu)
                .lifecycle
                >= CoreLifecycle::WorkloadComplete
        });
        if all_complete {
            break;
        }
        core::hint::spin_loop();
        remaining -= 1;
    }

    let final_state = SHARED_SCHEDULER_METADATA_SMOKE_STATE
        .try_lock()
        .map(|state| *state);
    let state_lock_available = final_state.is_some();
    let final_state = final_state.unwrap_or_else(SharedSchedulerMetadataSmokeState::new);
    let final_metadata = SHARED_SCHEDULER_METADATA_SMOKE_TABLE
        .try_lock()
        .map(|metadata| (metadata.len(), metadata.generation()));
    let metadata_lock_available = final_metadata.is_some();
    let (final_metadata_len, final_metadata_generation) = final_metadata.unwrap_or((0, 0));

    let mut participants = 0;
    let mut errors = 0;
    let mut reports_ok = cpu_on_ok
        && boot_logical == Some(0)
        && state_lock_available
        && metadata_lock_available
        && final_metadata_len == MAX_CORES;

    for logical_cpu in 0..MAX_CORES {
        let report = final_state.reports[logical_cpu];
        let (lifecycle, context, mapped, stack_owned) = if logical_cpu == 0 {
            (CoreLifecycle::WorkloadComplete, 0, boot_logical, true)
        } else {
            let core_report = SECONDARY_CORE_STATES[logical_cpu].snapshot(logical_cpu);
            let logical_from_mpidr = qemu_logical_cpu_from_mpidr_affinity(core_report.affinity);
            let stack_slot = stack_layout
                .slot(logical_cpu)
                .expect("stack slot for possible QEMU core");
            (
                core_report.lifecycle,
                core_report.context,
                logical_from_mpidr,
                stack_slot.contains_stack_pointer(core_report.stack_pointer),
            )
        };
        let expected_task = (logical_cpu as u64 + 1) * 100 + 1;
        let expected_role = if logical_cpu == 0 {
            SchedulerCoreRole::BootCpuProduction
        } else {
            SchedulerCoreRole::SecondaryProductionDiagnostic
        };
        let report_ok = lifecycle >= CoreLifecycle::WorkloadComplete
            && context == logical_cpu
            && mapped == Some(logical_cpu)
            && stack_owned
            && report.owner == logical_cpu as u64
            && report.role == expected_role
            && report.production_dispatch_enabled
            && report.task_id == expected_task
            && report.task_state == task_state_code(TaskState::Running)
            && report.current_task == expected_task
            && report.queue_len == 0
            && report.front_task == 0
            && report.lookup_owner == logical_cpu as u64
            && report.lookup_task == expected_task
            && report.lookup_generation > 0
            && report.boot_lookup_owner == 0
            && report.boot_lookup_task == 101
            && report.boot_lookup_generation > 0
            && report.cross_owner_rejected
            && report.metadata_cross_owner_rejected
            && report.local_queue_preserved
            && final_state.lock_progress[logical_cpu] == 1
            && report.errors == 0;
        if report_ok {
            participants += 1;
        }
        errors += report.errors;
        reports_ok &= report_ok;

        crate::println!(
            "qemu-shared-scheduler-metadata: report logical={} state={} context={} mapped={:?} owner={} role={} production={} task={} task-state={} current={} queue-len={} front={} metadata-len={} metadata-generation={} lookup-owner={} lookup-task={} lookup-generation={} boot-lookup-owner={} boot-lookup-task={} boot-lookup-generation={} cross-owner-rejected={} metadata-cross-owner-rejected={} local-queue-preserved={} lock-progress={} errors={} ok={}",
            logical_cpu,
            secondary_state_name(lifecycle.raw()),
            context,
            mapped,
            report.owner,
            scheduler_role_name(report.role),
            report.production_dispatch_enabled,
            report.task_id,
            task_state_name(report.task_state),
            report.current_task,
            report.queue_len,
            report.front_task,
            report.metadata_len,
            report.metadata_generation,
            report.lookup_owner,
            report.lookup_task,
            report.lookup_generation,
            report.boot_lookup_owner,
            report.boot_lookup_task,
            report.boot_lookup_generation,
            report.cross_owner_rejected,
            report.metadata_cross_owner_rejected,
            report.local_queue_preserved,
            final_state.lock_progress[logical_cpu],
            report.errors,
            report_ok
        );
    }

    let classification = if reports_ok {
        "qemu-shared-scheduler-metadata-complete"
    } else if !state_lock_available || !metadata_lock_available {
        "qemu-shared-scheduler-metadata-lock-still-held"
    } else if cpu_on_ok {
        "qemu-shared-scheduler-metadata-invariant-failed"
    } else {
        "qemu-psci-smc-cpu-on-failed"
    };
    crate::println!(
        "qemu-shared-scheduler-metadata: final participants={} expected={} errors={} state-lock-available={} metadata-lock-available={} final-metadata-len={} final-metadata-generation={} wait-remaining={} classification={}",
        participants,
        MAX_CORES,
        errors,
        state_lock_available,
        metadata_lock_available,
        final_metadata_len,
        final_metadata_generation,
        remaining,
        classification
    );

    if reports_ok {
        crate::println!("qemu-shared-scheduler-metadata: PASS");
    } else {
        crate::println!("qemu-shared-scheduler-metadata: FAIL");
    }

    reports_ok
}

#[cfg(talos_boot_scenario = "qemu_shared_runqueue_migration")]
#[derive(Clone, Copy)]
struct SharedRunQueueMigrationReport {
    source_owner: u64,
    destination_owner: u64,
    task_id: u64,
    task_state: u64,
    registered_generation: u64,
    publish_reserved_state: MigrationState,
    publish_queued_state: MigrationState,
    consume_queued_state: MigrationState,
    consume_destination_state: MigrationState,
    source_queue_before: u64,
    source_queue_after_publish: u64,
    shared_len_after_publish: u64,
    shared_len_after_consume: u64,
    destination_queue_len: u64,
    destination_front: u64,
    metadata_owner_after_consume: u64,
    metadata_generation_after_consume: u64,
    source_removed: bool,
    destination_enqueued: bool,
    metadata_migrated: bool,
    errors: u64,
}

#[cfg(any(
    talos_boot_scenario = "qemu_shared_runqueue_migration",
    talos_boot_scenario = "qemu_load_balancing_smoke"
))]
fn migration_state_name(state: MigrationState) -> &'static str {
    match state {
        MigrationState::OwnerLocal => "owner-local",
        MigrationState::MigrationReserved => "migration-reserved",
        MigrationState::SharedQueued => "shared-queued",
        MigrationState::DestinationEnqueued => "destination-enqueued",
        MigrationState::MigrationRejected => "migration-rejected",
    }
}

#[cfg(talos_boot_scenario = "qemu_shared_runqueue_migration")]
fn build_shared_runqueue_migration_report() -> SharedRunQueueMigrationReport {
    let source_owner = LogicalCpuId::BOOT;
    let destination_owner = LogicalCpuId::new(1);
    let mut source_scheduler = PerCoreScheduler::<2>::boot_cpu();
    let mut destination_scheduler =
        PerCoreScheduler::<2>::production_secondary_diagnostic(destination_owner);
    let mut metadata =
        SharedSchedulerMetadata::<SHARED_RUNQUEUE_MIGRATION_TASK_CAPACITY, MAX_CORES>::new();
    let mut shared = SharedRunQueue::<SHARED_RUNQUEUE_MIGRATION_QUEUE_CAPACITY, MAX_CORES>::new();
    let mut task = scheduler_task(0, 7);
    let mut errors = 0;

    match source_scheduler.local_scheduler_mut(source_owner) {
        Ok(scheduler) => {
            if scheduler.make_runnable(&mut task).is_err() {
                errors += 1;
            }
        }
        Err(_) => errors += 1,
    }
    let source_queue_before = source_scheduler.scheduler().runnable().len() as u64;

    let registered_generation =
        match metadata.register_local_task(source_owner, &source_scheduler, &task) {
            Ok(snapshot) => snapshot.generation(),
            Err(_) => {
                errors += 1;
                0
            }
        };

    let publish_report = shared.publish_migration(
        source_owner,
        &mut source_scheduler,
        destination_owner,
        &metadata,
        &task,
        registered_generation,
    );
    let (publish_reserved_state, publish_queued_state) = match publish_report {
        Ok(report) => (report.reserved().state(), report.queued().state()),
        Err(_) => {
            errors += 1;
            (
                MigrationState::MigrationRejected,
                MigrationState::MigrationRejected,
            )
        }
    };

    let source_queue_after_publish = source_scheduler.scheduler().runnable().len() as u64;
    let source_removed = source_queue_before == 1
        && source_queue_after_publish == 0
        && !source_scheduler.scheduler().runnable().contains(task.id());
    if !source_removed {
        errors += 1;
    }

    let shared_len_after_publish = shared.len() as u64;
    let consume_report = shared.consume_for_destination(
        destination_owner,
        &mut destination_scheduler,
        &mut metadata,
        &mut task,
    );
    let (consume_queued_state, consume_destination_state) = match consume_report {
        Ok(Some(report)) => (
            report.queued().state(),
            report.destination_enqueued().state(),
        ),
        _ => {
            errors += 1;
            (
                MigrationState::MigrationRejected,
                MigrationState::MigrationRejected,
            )
        }
    };

    let shared_len_after_consume = shared.len() as u64;
    let destination_queue_len = destination_scheduler.scheduler().runnable().len() as u64;
    let destination_front = destination_scheduler
        .scheduler()
        .runnable()
        .front()
        .map_or(0, TaskId::raw);
    let final_metadata = metadata.lookup_task(task.id());
    let (metadata_owner_after_consume, metadata_generation_after_consume) = match final_metadata {
        Ok(snapshot) => (snapshot.owner().raw() as u64, snapshot.generation()),
        Err(_) => {
            errors += 1;
            (u64::MAX, 0)
        }
    };

    let destination_enqueued = destination_queue_len == 1
        && destination_front == task.id().raw()
        && task.state() == TaskState::Runnable
        && shared_len_after_consume == 0;
    if !destination_enqueued {
        errors += 1;
    }

    let metadata_migrated = metadata_owner_after_consume == destination_owner.raw() as u64
        && metadata_generation_after_consume > registered_generation;
    if !metadata_migrated {
        errors += 1;
    }

    SharedRunQueueMigrationReport {
        source_owner: source_owner.raw() as u64,
        destination_owner: destination_owner.raw() as u64,
        task_id: task.id().raw(),
        task_state: task_state_code(task.state()),
        registered_generation,
        publish_reserved_state,
        publish_queued_state,
        consume_queued_state,
        consume_destination_state,
        source_queue_before,
        source_queue_after_publish,
        shared_len_after_publish,
        shared_len_after_consume,
        destination_queue_len,
        destination_front,
        metadata_owner_after_consume,
        metadata_generation_after_consume,
        source_removed,
        destination_enqueued,
        metadata_migrated,
        errors,
    }
}

#[cfg(talos_boot_scenario = "qemu_shared_runqueue_migration")]
pub fn run_shared_runqueue_migration_smoke() -> bool {
    let boot_mpidr = aarch64::mpidr_el1();
    let boot_affinity = aarch64::mpidr_affinity(boot_mpidr);
    let boot_logical = qemu_logical_cpu_from_mpidr_affinity(boot_affinity);
    crate::println!(
        "qemu-shared-runqueue-migration: start task-capacity={} queue-capacity={} boot-mpidr={:#018x} boot-affinity={:#x} boot-logical={:?}",
        SHARED_RUNQUEUE_MIGRATION_TASK_CAPACITY,
        SHARED_RUNQUEUE_MIGRATION_QUEUE_CAPACITY,
        boot_mpidr,
        boot_affinity,
        boot_logical,
    );

    let report = build_shared_runqueue_migration_report();
    let report_ok = boot_logical == Some(0)
        && report.source_owner == 0
        && report.destination_owner == 1
        && report.task_id == 107
        && report.task_state == task_state_code(TaskState::Runnable)
        && report.registered_generation > 0
        && report.publish_reserved_state == MigrationState::MigrationReserved
        && report.publish_queued_state == MigrationState::SharedQueued
        && report.consume_queued_state == MigrationState::SharedQueued
        && report.consume_destination_state == MigrationState::DestinationEnqueued
        && report.source_removed
        && report.destination_enqueued
        && report.metadata_migrated
        && report.errors == 0;

    crate::println!(
        "qemu-shared-runqueue-migration: report source-owner={} destination-owner={} task={} task-state={} registered-generation={} publish-reserved-state={} publish-queued-state={} consume-queued-state={} consume-destination-state={} source-queue-before={} source-queue-after-publish={} shared-len-after-publish={} shared-len-after-consume={} destination-queue-len={} destination-front={} metadata-owner-after-consume={} metadata-generation-after-consume={} source-removed={} destination-enqueued={} metadata-migrated={} errors={} ok={}",
        report.source_owner,
        report.destination_owner,
        report.task_id,
        task_state_name(report.task_state),
        report.registered_generation,
        migration_state_name(report.publish_reserved_state),
        migration_state_name(report.publish_queued_state),
        migration_state_name(report.consume_queued_state),
        migration_state_name(report.consume_destination_state),
        report.source_queue_before,
        report.source_queue_after_publish,
        report.shared_len_after_publish,
        report.shared_len_after_consume,
        report.destination_queue_len,
        report.destination_front,
        report.metadata_owner_after_consume,
        report.metadata_generation_after_consume,
        report.source_removed,
        report.destination_enqueued,
        report.metadata_migrated,
        report.errors,
        report_ok
    );

    let classification = if report_ok {
        "qemu-shared-runqueue-migration-complete"
    } else {
        "qemu-shared-runqueue-migration-invariant-failed"
    };
    crate::println!(
        "qemu-shared-runqueue-migration: final participants=1 expected=1 errors={} classification={}",
        report.errors,
        classification
    );

    if report_ok {
        crate::println!("qemu-shared-runqueue-migration: PASS");
    } else {
        crate::println!("qemu-shared-runqueue-migration: FAIL");
    }

    report_ok
}

#[cfg(talos_boot_scenario = "qemu_load_balancing_smoke")]
#[derive(Clone, Copy)]
struct LoadBalancingSmokeReport {
    source_owner: u64,
    destination_owner: u64,
    task_id: u64,
    task_state: u64,
    registered_generation: u64,
    plan_generation: u64,
    publish_reserved_state: MigrationState,
    publish_queued_state: MigrationState,
    consume_queued_state: MigrationState,
    consume_destination_state: MigrationState,
    source_queue_before: u64,
    source_queue_after_publish: u64,
    shared_len_after_publish: u64,
    shared_len_after_consume: u64,
    destination_queue_len: u64,
    destination_front: u64,
    metadata_owner_after_consume: u64,
    metadata_generation_after_consume: u64,
    selected_front: bool,
    source_removed: bool,
    destination_enqueued: bool,
    metadata_migrated: bool,
    errors: u64,
}

#[cfg(talos_boot_scenario = "qemu_load_balancing_smoke")]
fn build_load_balancing_smoke_report() -> LoadBalancingSmokeReport {
    let source_owner = LogicalCpuId::BOOT;
    let destination_owner = LogicalCpuId::new(1);
    let mut source_scheduler = PerCoreScheduler::<2>::boot_cpu();
    let mut destination_scheduler =
        PerCoreScheduler::<2>::production_secondary_diagnostic(destination_owner);
    let mut metadata =
        SharedSchedulerMetadata::<LOAD_BALANCING_SMOKE_TASK_CAPACITY, MAX_CORES>::new();
    let mut shared = SharedRunQueue::<LOAD_BALANCING_SMOKE_QUEUE_CAPACITY, MAX_CORES>::new();
    let mut task = scheduler_task(0, 9);
    let mut errors = 0;

    match source_scheduler.local_scheduler_mut(source_owner) {
        Ok(scheduler) => {
            if scheduler.make_runnable(&mut task).is_err() {
                errors += 1;
            }
        }
        Err(_) => errors += 1,
    }
    let source_queue_before = source_scheduler.scheduler().runnable().len() as u64;

    let registered_generation =
        match metadata.register_local_task(source_owner, &source_scheduler, &task) {
            Ok(snapshot) => snapshot.generation(),
            Err(_) => {
                errors += 1;
                0
            }
        };

    let plan = LoadBalancingPolicy::plan_front_runnable(
        source_owner,
        &source_scheduler,
        &destination_scheduler,
        &metadata,
        &shared,
    );
    let (plan_task, plan_generation) = match plan {
        Ok(plan) => (plan.task_id().raw(), plan.metadata_generation()),
        Err(_) => {
            errors += 1;
            (0, 0)
        }
    };

    let publish_report = LoadBalancingPolicy::publish_front_runnable(
        source_owner,
        &mut source_scheduler,
        &destination_scheduler,
        &metadata,
        &mut shared,
        &task,
    );
    let (publish_reserved_state, publish_queued_state) = match publish_report {
        Ok(report) => (
            report.migration().reserved().state(),
            report.migration().queued().state(),
        ),
        Err(_) => {
            errors += 1;
            (
                MigrationState::MigrationRejected,
                MigrationState::MigrationRejected,
            )
        }
    };

    let source_queue_after_publish = source_scheduler.scheduler().runnable().len() as u64;
    let source_removed = source_queue_before == 1
        && source_queue_after_publish == 0
        && !source_scheduler.scheduler().runnable().contains(task.id());
    if !source_removed {
        errors += 1;
    }

    let shared_len_after_publish = shared.len() as u64;
    let consume_report = shared.consume_for_destination(
        destination_owner,
        &mut destination_scheduler,
        &mut metadata,
        &mut task,
    );
    let (consume_queued_state, consume_destination_state) = match consume_report {
        Ok(Some(report)) => (
            report.queued().state(),
            report.destination_enqueued().state(),
        ),
        _ => {
            errors += 1;
            (
                MigrationState::MigrationRejected,
                MigrationState::MigrationRejected,
            )
        }
    };

    let shared_len_after_consume = shared.len() as u64;
    let destination_queue_len = destination_scheduler.scheduler().runnable().len() as u64;
    let destination_front = destination_scheduler
        .scheduler()
        .runnable()
        .front()
        .map_or(0, TaskId::raw);
    let final_metadata = metadata.lookup_task(task.id());
    let (metadata_owner_after_consume, metadata_generation_after_consume) = match final_metadata {
        Ok(snapshot) => (snapshot.owner().raw() as u64, snapshot.generation()),
        Err(_) => {
            errors += 1;
            (u64::MAX, 0)
        }
    };

    let selected_front = plan_task == task.id().raw() && plan_generation == registered_generation;
    if !selected_front {
        errors += 1;
    }

    let destination_enqueued = destination_queue_len == 1
        && destination_front == task.id().raw()
        && task.state() == TaskState::Runnable
        && shared_len_after_consume == 0;
    if !destination_enqueued {
        errors += 1;
    }

    let metadata_migrated = metadata_owner_after_consume == destination_owner.raw() as u64
        && metadata_generation_after_consume > registered_generation;
    if !metadata_migrated {
        errors += 1;
    }

    LoadBalancingSmokeReport {
        source_owner: source_owner.raw() as u64,
        destination_owner: destination_owner.raw() as u64,
        task_id: task.id().raw(),
        task_state: task_state_code(task.state()),
        registered_generation,
        plan_generation,
        publish_reserved_state,
        publish_queued_state,
        consume_queued_state,
        consume_destination_state,
        source_queue_before,
        source_queue_after_publish,
        shared_len_after_publish,
        shared_len_after_consume,
        destination_queue_len,
        destination_front,
        metadata_owner_after_consume,
        metadata_generation_after_consume,
        selected_front,
        source_removed,
        destination_enqueued,
        metadata_migrated,
        errors,
    }
}

#[cfg(talos_boot_scenario = "qemu_load_balancing_smoke")]
pub fn run_load_balancing_smoke() -> bool {
    let boot_mpidr = aarch64::mpidr_el1();
    let boot_affinity = aarch64::mpidr_affinity(boot_mpidr);
    let boot_logical = qemu_logical_cpu_from_mpidr_affinity(boot_affinity);
    crate::println!(
        "qemu-load-balancing-smoke: start task-capacity={} queue-capacity={} boot-mpidr={:#018x} boot-affinity={:#x} boot-logical={:?}",
        LOAD_BALANCING_SMOKE_TASK_CAPACITY,
        LOAD_BALANCING_SMOKE_QUEUE_CAPACITY,
        boot_mpidr,
        boot_affinity,
        boot_logical,
    );

    let report = build_load_balancing_smoke_report();
    let report_ok = boot_logical == Some(0)
        && report.source_owner == 0
        && report.destination_owner == 1
        && report.task_id == 109
        && report.task_state == task_state_code(TaskState::Runnable)
        && report.registered_generation > 0
        && report.plan_generation == report.registered_generation
        && report.publish_reserved_state == MigrationState::MigrationReserved
        && report.publish_queued_state == MigrationState::SharedQueued
        && report.consume_queued_state == MigrationState::SharedQueued
        && report.consume_destination_state == MigrationState::DestinationEnqueued
        && report.selected_front
        && report.source_removed
        && report.destination_enqueued
        && report.metadata_migrated
        && report.errors == 0;

    crate::println!(
        "qemu-load-balancing-smoke: report source-owner={} destination-owner={} task={} task-state={} registered-generation={} plan-generation={} publish-reserved-state={} publish-queued-state={} consume-queued-state={} consume-destination-state={} source-queue-before={} source-queue-after-publish={} shared-len-after-publish={} shared-len-after-consume={} destination-queue-len={} destination-front={} metadata-owner-after-consume={} metadata-generation-after-consume={} selected-front={} source-removed={} destination-enqueued={} metadata-migrated={} errors={} ok={}",
        report.source_owner,
        report.destination_owner,
        report.task_id,
        task_state_name(report.task_state),
        report.registered_generation,
        report.plan_generation,
        migration_state_name(report.publish_reserved_state),
        migration_state_name(report.publish_queued_state),
        migration_state_name(report.consume_queued_state),
        migration_state_name(report.consume_destination_state),
        report.source_queue_before,
        report.source_queue_after_publish,
        report.shared_len_after_publish,
        report.shared_len_after_consume,
        report.destination_queue_len,
        report.destination_front,
        report.metadata_owner_after_consume,
        report.metadata_generation_after_consume,
        report.selected_front,
        report.source_removed,
        report.destination_enqueued,
        report.metadata_migrated,
        report.errors,
        report_ok
    );

    let classification = if report_ok {
        "qemu-load-balancing-smoke-complete"
    } else {
        "qemu-load-balancing-smoke-invariant-failed"
    };
    crate::println!(
        "qemu-load-balancing-smoke: final participants=1 expected=1 errors={} classification={}",
        report.errors,
        classification
    );

    if report_ok {
        crate::println!("qemu-load-balancing-smoke: PASS");
    } else {
        crate::println!("qemu-load-balancing-smoke: FAIL");
    }

    report_ok
}

#[cfg(talos_boot_scenario = "qemu_secondary_scheduler_service_loop")]
pub fn run_secondary_scheduler_service_loop_smoke() -> bool {
    smp::reset_secondary_core_states();
    reset_secondary_scheduler_service_loop_state();

    let boot_mpidr = aarch64::mpidr_el1();
    let boot_affinity = aarch64::mpidr_affinity(boot_mpidr);
    let boot_logical = qemu_logical_cpu_from_mpidr_affinity(boot_affinity);
    let entry = talos_aarch64_qemu_secondary_entry as *const () as usize;
    let stack_layout = secondary_stack_layout();
    let stack_base = core::ptr::addr_of!(talos_secondary_core_stacks) as usize;
    let stack_end = core::ptr::addr_of!(talos_secondary_core_stacks_end) as usize;

    crate::println!(
        "qemu-secondary-scheduler-service-loop: start conduit=smc cores={} task-capacity={} boot-mpidr={:#018x} boot-affinity={:#x} boot-logical={:?} entry={:#018x} stack-range=[{:#018x},{:#018x})",
        MAX_CORES,
        SECONDARY_SCHEDULER_SERVICE_LOOP_TASK_CAPACITY,
        boot_mpidr,
        boot_affinity,
        boot_logical,
        entry,
        stack_base,
        stack_end
    );

    let mut cpu_on_ok = true;
    for logical_cpu in 1..MAX_CORES {
        let target_affinity = logical_cpu as u64;
        let result = unsafe { psci_cpu_on_smc(target_affinity, entry, logical_cpu) };
        crate::println!(
            "qemu-secondary-scheduler-service-loop: cpu-on logical={} target-affinity={:#x} result={}",
            logical_cpu,
            target_affinity,
            result
        );
        cpu_on_ok &= result == 0;
    }

    let mut remaining = SECONDARY_SCHEDULER_SERVICE_LOOP_WAIT_LIMIT;
    while remaining > 0 {
        let all_complete = (1..MAX_CORES).all(|logical_cpu| {
            SECONDARY_CORE_STATES[logical_cpu]
                .snapshot(logical_cpu)
                .lifecycle
                >= CoreLifecycle::WorkloadComplete
        });
        if all_complete {
            break;
        }
        core::hint::spin_loop();
        remaining -= 1;
    }

    let state_lock_available = true;
    let metadata_lock_available = true;
    let mut final_metadata_len = 0;
    let mut final_metadata_generation = 0;
    for logical_cpu in 1..MAX_CORES {
        let report = load_secondary_scheduler_service_loop_report(logical_cpu);
        final_metadata_len += report.metadata_len;
        final_metadata_generation += report.metadata_generation;
    }

    let mut participants = 0;
    let mut errors = 0;
    let mut reports_ok = cpu_on_ok
        && boot_logical == Some(0)
        && state_lock_available
        && final_metadata_len == (MAX_CORES - 1) as u64;

    for logical_cpu in 1..MAX_CORES {
        let report = load_secondary_scheduler_service_loop_report(logical_cpu);
        let core_report = SECONDARY_CORE_STATES[logical_cpu].snapshot(logical_cpu);
        let logical_from_mpidr = qemu_logical_cpu_from_mpidr_affinity(core_report.affinity);
        let stack_slot = stack_layout
            .slot(logical_cpu)
            .expect("stack slot for possible QEMU core");
        let stack_owned = stack_slot.contains_stack_pointer(core_report.stack_pointer);
        let expected_task = (logical_cpu as u64 + 1) * 100 + 1;
        let report_ok = core_report.lifecycle >= CoreLifecycle::WorkloadComplete
            && core_report.context == logical_cpu
            && logical_from_mpidr == Some(logical_cpu)
            && stack_owned
            && report.owner == logical_cpu as u64
            && report.role == SchedulerCoreRole::SecondaryProductionDiagnostic
            && report.task_id == expected_task
            && report.task_state == task_state_code(TaskState::Running)
            && report.current_task == expected_task
            && report.queue_len == 0
            && report.front_task == 0
            && report.remote_wake_task == expected_task
            && report.dispatch_task == expected_task
            && !report.no_work_did_work
            && report.metadata_generation > 0
            && report.observed_remote_wake
            && !report.pending_timer_preemption
            && report.dispatch_requested
            && report.cross_owner_rejected
            && report.deferred_role_rejected
            && report.local_queue_preserved
            && SECONDARY_SCHEDULER_SERVICE_LOOP_STATE.progress[logical_cpu].load(Ordering::Acquire)
                == 1
            && report.errors == 0;
        if report_ok {
            participants += 1;
        }
        errors += report.errors;
        reports_ok &= report_ok;

        crate::println!(
            "qemu-secondary-scheduler-service-loop: report logical={} state={} context={} mapped={:?} owner={} role={} task={} task-state={} current={} queue-len={} front={} remote-wake={} dispatch={} no-work-did-work={} metadata-len={} metadata-generation={} observed-remote-wake={} pending-timer-preemption={} dispatch-requested={} cross-owner-rejected={} deferred-role-rejected={} local-queue-preserved={} lock-progress={} errors={} ok={}",
            logical_cpu,
            secondary_state_name(core_report.lifecycle.raw()),
            core_report.context,
            logical_from_mpidr,
            report.owner,
            scheduler_role_name(report.role),
            report.task_id,
            task_state_name(report.task_state),
            report.current_task,
            report.queue_len,
            report.front_task,
            report.remote_wake_task,
            report.dispatch_task,
            report.no_work_did_work,
            report.metadata_len,
            report.metadata_generation,
            report.observed_remote_wake,
            report.pending_timer_preemption,
            report.dispatch_requested,
            report.cross_owner_rejected,
            report.deferred_role_rejected,
            report.local_queue_preserved,
            SECONDARY_SCHEDULER_SERVICE_LOOP_STATE.progress[logical_cpu].load(Ordering::Acquire),
            report.errors,
            report_ok
        );
    }

    let classification = if reports_ok {
        "qemu-secondary-scheduler-service-loop-complete"
    } else if !state_lock_available || !metadata_lock_available {
        "qemu-secondary-scheduler-service-loop-lock-still-held"
    } else if cpu_on_ok {
        "qemu-secondary-scheduler-service-loop-invariant-failed"
    } else {
        "qemu-psci-smc-cpu-on-failed"
    };
    crate::println!(
        "qemu-secondary-scheduler-service-loop: final participants={} expected={} errors={} state-lock-available={} metadata-lock-available={} final-metadata-len={} final-metadata-generation={} wait-remaining={} classification={}",
        participants,
        MAX_CORES - 1,
        errors,
        state_lock_available,
        metadata_lock_available,
        final_metadata_len,
        final_metadata_generation,
        remaining,
        classification
    );

    if reports_ok {
        crate::println!("qemu-secondary-scheduler-service-loop: PASS");
    } else {
        crate::println!("qemu-secondary-scheduler-service-loop: FAIL");
    }

    reports_ok
}

#[cfg(talos_boot_scenario = "qemu_multicore_preemption_smoke")]
pub fn run_multicore_preemption_smoke() -> bool {
    smp::reset_secondary_core_states();
    reset_multicore_preemption_smoke_state();

    let boot_mpidr = aarch64::mpidr_el1();
    let boot_affinity = aarch64::mpidr_affinity(boot_mpidr);
    let boot_logical = qemu_logical_cpu_from_mpidr_affinity(boot_affinity);
    let entry = talos_aarch64_qemu_secondary_entry as *const () as usize;
    let stack_layout = secondary_stack_layout();
    let stack_base = core::ptr::addr_of!(talos_secondary_core_stacks) as usize;
    let stack_end = core::ptr::addr_of!(talos_secondary_core_stacks_end) as usize;

    crate::println!(
        "qemu-multicore-preemption-smoke: start conduit=smc cores={} task-capacity={} boot-mpidr={:#018x} boot-affinity={:#x} boot-logical={:?} entry={:#018x} stack-range=[{:#018x},{:#018x})",
        MAX_CORES,
        MULTICORE_PREEMPTION_SMOKE_TASK_CAPACITY,
        boot_mpidr,
        boot_affinity,
        boot_logical,
        entry,
        stack_base,
        stack_end
    );

    let mut cpu_on_ok = true;
    for logical_cpu in 1..MAX_CORES {
        let target_affinity = logical_cpu as u64;
        let result = unsafe { psci_cpu_on_smc(target_affinity, entry, logical_cpu) };
        crate::println!(
            "qemu-multicore-preemption-smoke: cpu-on logical={} target-affinity={:#x} result={}",
            logical_cpu,
            target_affinity,
            result
        );
        cpu_on_ok &= result == 0;
    }

    let mut remaining = MULTICORE_PREEMPTION_SMOKE_WAIT_LIMIT;
    while remaining > 0 {
        let all_complete = (1..MAX_CORES).all(|logical_cpu| {
            SECONDARY_CORE_STATES[logical_cpu]
                .snapshot(logical_cpu)
                .lifecycle
                >= CoreLifecycle::WorkloadComplete
        });
        if all_complete {
            break;
        }
        core::hint::spin_loop();
        remaining -= 1;
    }

    let state_lock_available = true;
    let metadata_lock_available = true;
    let mut participants = 0;
    let mut errors = 0;
    let mut reports_ok = cpu_on_ok && boot_logical == Some(0) && state_lock_available;

    for logical_cpu in 1..MAX_CORES {
        let report = load_multicore_preemption_smoke_report(logical_cpu);
        let core_report = SECONDARY_CORE_STATES[logical_cpu].snapshot(logical_cpu);
        let logical_from_mpidr = qemu_logical_cpu_from_mpidr_affinity(core_report.affinity);
        let stack_slot = stack_layout
            .slot(logical_cpu)
            .expect("stack slot for possible QEMU core");
        let stack_owned = stack_slot.contains_stack_pointer(core_report.stack_pointer);
        let expected_current = (logical_cpu as u64 + 1) * 100 + 1;
        let expected_next = (logical_cpu as u64 + 1) * 100 + 2;
        let report_ok = core_report.lifecycle >= CoreLifecycle::WorkloadComplete
            && core_report.context == logical_cpu
            && logical_from_mpidr == Some(logical_cpu)
            && stack_owned
            && report.owner == logical_cpu as u64
            && report.role == SchedulerCoreRole::SecondaryProductionDiagnostic
            && report.current_before_record == expected_current
            && report.next_task == expected_next
            && report.queue_len_before_record == 1
            && report.metadata_generation_before_record > 0
            && report.record_inserted
            && report.duplicate_coalesced
            && report.cross_owner_rejected
            && report.current_after_record == expected_current
            && report.queue_len_after_record == 1
            && report.metadata_generation_after_record == report.metadata_generation_before_record
            && !report.scheduler_mutated_during_record
            && report.pending_after_record
            && report.service_timer_preemption == expected_next
            && report.current_after_service == expected_next
            && report.queue_len_after_service == 1
            && report.front_after_service == expected_current
            && report.previous_task_state == task_state_code(TaskState::Runnable)
            && report.selected_task_state == task_state_code(TaskState::Running)
            && !report.pending_after_service
            && report.recorded_requests == 1
            && report.coalesced_requests == 1
            && report.serviced_requests == 1
            && report.metadata_owner_after_service == logical_cpu as u64
            && report.metadata_task_after_service == expected_next
            && report.metadata_generation_after_service > report.metadata_generation_after_record
            && report.errors == 0
            && report.progress() == 1;
        if report_ok {
            participants += 1;
        }
        errors += report.errors;
        reports_ok &= report_ok;

        crate::println!(
            "qemu-multicore-preemption-smoke: report logical={} state={} context={} mapped={:?} owner={} role={} current-before-record={} next={} queue-len-before-record={} metadata-generation-before-record={} record-outcome={} duplicate-outcome={} cross-owner-rejected={} current-after-record={} queue-len-after-record={} metadata-generation-after-record={} irq-record-scheduler-mutated={} pending-after-record={} service-timer-preemption={} current-after-service={} queue-len-after-service={} front-after-service={} previous-task-state={} selected-task-state={} pending-after-service={} recorded={} coalesced={} serviced={} metadata-owner-after-service={} metadata-task-after-service={} metadata-generation-after-service={} lock-progress={} errors={} ok={}",
            logical_cpu,
            secondary_state_name(core_report.lifecycle.raw()),
            core_report.context,
            logical_from_mpidr,
            report.owner,
            scheduler_role_name(report.role),
            report.current_before_record,
            report.next_task,
            report.queue_len_before_record,
            report.metadata_generation_before_record,
            if report.record_inserted {
                "inserted"
            } else {
                "error"
            },
            if report.duplicate_coalesced {
                "coalesced"
            } else {
                "error"
            },
            report.cross_owner_rejected,
            report.current_after_record,
            report.queue_len_after_record,
            report.metadata_generation_after_record,
            report.scheduler_mutated_during_record,
            report.pending_after_record,
            report.service_timer_preemption,
            report.current_after_service,
            report.queue_len_after_service,
            report.front_after_service,
            task_state_name(report.previous_task_state),
            task_state_name(report.selected_task_state),
            report.pending_after_service,
            report.recorded_requests,
            report.coalesced_requests,
            report.serviced_requests,
            report.metadata_owner_after_service,
            report.metadata_task_after_service,
            report.metadata_generation_after_service,
            report.progress(),
            report.errors,
            report_ok
        );
    }

    let classification = if reports_ok {
        "qemu-multicore-preemption-smoke-complete"
    } else if !state_lock_available || !metadata_lock_available {
        "qemu-multicore-preemption-smoke-lock-still-held"
    } else if cpu_on_ok {
        "qemu-multicore-preemption-smoke-invariant-failed"
    } else {
        "qemu-psci-smc-cpu-on-failed"
    };
    crate::println!(
        "qemu-multicore-preemption-smoke: final participants={} expected={} errors={} state-lock-available={} metadata-lock-available={} wait-remaining={} classification={}",
        participants,
        MAX_CORES - 1,
        errors,
        state_lock_available,
        metadata_lock_available,
        remaining,
        classification
    );

    if reports_ok {
        crate::println!("qemu-multicore-preemption-smoke: PASS");
    } else {
        crate::println!("qemu-multicore-preemption-smoke: FAIL");
    }

    reports_ok
}

#[cfg(talos_boot_scenario = "qemu_production_timer_preemption_smoke")]
pub fn run_production_timer_preemption_smoke() -> bool {
    smp::reset_secondary_core_states();
    reset_production_timer_preemption_smoke_state();

    let boot_mpidr = aarch64::mpidr_el1();
    let boot_affinity = aarch64::mpidr_affinity(boot_mpidr);
    let boot_logical = qemu_logical_cpu_from_mpidr_affinity(boot_affinity);
    let entry = talos_aarch64_qemu_secondary_entry as *const () as usize;
    let stack_layout = secondary_stack_layout();
    let stack_base = core::ptr::addr_of!(talos_secondary_core_stacks) as usize;
    let stack_end = core::ptr::addr_of!(talos_secondary_core_stacks_end) as usize;

    crate::println!(
        "qemu-production-timer-preemption-smoke: start conduit=smc cores={} task-capacity={} entry-path=production-timer-irq-adapter boot-mpidr={:#018x} boot-affinity={:#x} boot-logical={:?} entry={:#018x} stack-range=[{:#018x},{:#018x})",
        MAX_CORES,
        PRODUCTION_TIMER_PREEMPTION_SMOKE_TASK_CAPACITY,
        boot_mpidr,
        boot_affinity,
        boot_logical,
        entry,
        stack_base,
        stack_end
    );

    let mut cpu_on_ok = true;
    for logical_cpu in 1..MAX_CORES {
        let target_affinity = logical_cpu as u64;
        let result = unsafe { psci_cpu_on_smc(target_affinity, entry, logical_cpu) };
        crate::println!(
            "qemu-production-timer-preemption-smoke: cpu-on logical={} target-affinity={:#x} result={}",
            logical_cpu,
            target_affinity,
            result
        );
        cpu_on_ok &= result == 0;
    }

    let mut remaining = PRODUCTION_TIMER_PREEMPTION_SMOKE_WAIT_LIMIT;
    while remaining > 0 {
        let all_complete = (1..MAX_CORES).all(|logical_cpu| {
            SECONDARY_CORE_STATES[logical_cpu]
                .snapshot(logical_cpu)
                .lifecycle
                >= CoreLifecycle::WorkloadComplete
        });
        if all_complete {
            break;
        }
        core::hint::spin_loop();
        remaining -= 1;
    }

    let state_lock_available = true;
    let metadata_lock_available = true;
    let mut participants = 0;
    let mut errors = 0;
    let mut reports_ok = cpu_on_ok && boot_logical == Some(0) && state_lock_available;

    for logical_cpu in 1..MAX_CORES {
        let report = load_production_timer_preemption_smoke_report(logical_cpu);
        let core_report = SECONDARY_CORE_STATES[logical_cpu].snapshot(logical_cpu);
        let logical_from_mpidr = qemu_logical_cpu_from_mpidr_affinity(core_report.affinity);
        let stack_slot = stack_layout
            .slot(logical_cpu)
            .expect("stack slot for possible QEMU core");
        let stack_owned = stack_slot.contains_stack_pointer(core_report.stack_pointer);
        let expected_current = (logical_cpu as u64 + 1) * 100 + 1;
        let expected_next = (logical_cpu as u64 + 1) * 100 + 2;
        let report_ok = core_report.lifecycle >= CoreLifecycle::WorkloadComplete
            && core_report.context == logical_cpu
            && logical_from_mpidr == Some(logical_cpu)
            && stack_owned
            && report.owner == logical_cpu as u64
            && report.role == SchedulerCoreRole::SecondaryProductionDiagnostic
            && report.current_before_record == expected_current
            && report.next_task == expected_next
            && report.queue_len_before_record == 1
            && report.metadata_generation_before_record > 0
            && report.production_irq_record_inserted
            && report.production_irq_duplicate_coalesced
            && report.cross_owner_rejected
            && report.record_misses == 0
            && report.timer_record_rejections == 1
            && report.current_after_record == expected_current
            && report.queue_len_after_record == 1
            && report.metadata_generation_after_record == report.metadata_generation_before_record
            && !report.irq_record_scheduler_mutated
            && report.pending_after_record
            && report.service_timer_preemption == expected_next
            && report.current_after_service == expected_next
            && report.queue_len_after_service == 1
            && report.front_after_service == expected_current
            && report.previous_task_state == task_state_code(TaskState::Runnable)
            && report.selected_task_state == task_state_code(TaskState::Running)
            && !report.pending_after_service
            && report.recorded_requests == 1
            && report.coalesced_requests == 1
            && report.serviced_requests == 1
            && report.metadata_owner_after_service == logical_cpu as u64
            && report.metadata_task_after_service == expected_next
            && report.metadata_generation_after_service > report.metadata_generation_after_record
            && report.errors == 0
            && report.progress() == 1;
        if report_ok {
            participants += 1;
        }
        errors += report.errors;
        reports_ok &= report_ok;

        crate::println!(
            "qemu-production-timer-preemption-smoke: report logical={} state={} context={} mapped={:?} owner={} role={} current-before-record={} next={} queue-len-before-record={} metadata-generation-before-record={} entry-path=production-timer-irq-adapter record-outcome={} duplicate-outcome={} cross-owner-rejected={} record-misses={} timer-record-rejections={} current-after-record={} queue-len-after-record={} metadata-generation-after-record={} irq-record-scheduler-mutated={} pending-after-record={} service-timer-preemption={} current-after-service={} queue-len-after-service={} front-after-service={} previous-task-state={} selected-task-state={} pending-after-service={} recorded={} coalesced={} serviced={} metadata-owner-after-service={} metadata-task-after-service={} metadata-generation-after-service={} lock-progress={} errors={} ok={}",
            logical_cpu,
            secondary_state_name(core_report.lifecycle.raw()),
            core_report.context,
            logical_from_mpidr,
            report.owner,
            scheduler_role_name(report.role),
            report.current_before_record,
            report.next_task,
            report.queue_len_before_record,
            report.metadata_generation_before_record,
            if report.production_irq_record_inserted {
                "inserted"
            } else {
                "error"
            },
            if report.production_irq_duplicate_coalesced {
                "coalesced"
            } else {
                "error"
            },
            report.cross_owner_rejected,
            report.record_misses,
            report.timer_record_rejections,
            report.current_after_record,
            report.queue_len_after_record,
            report.metadata_generation_after_record,
            report.irq_record_scheduler_mutated,
            report.pending_after_record,
            report.service_timer_preemption,
            report.current_after_service,
            report.queue_len_after_service,
            report.front_after_service,
            task_state_name(report.previous_task_state),
            task_state_name(report.selected_task_state),
            report.pending_after_service,
            report.recorded_requests,
            report.coalesced_requests,
            report.serviced_requests,
            report.metadata_owner_after_service,
            report.metadata_task_after_service,
            report.metadata_generation_after_service,
            report.progress(),
            report.errors,
            report_ok
        );
    }

    let classification = if reports_ok {
        "qemu-production-timer-preemption-smoke-complete"
    } else if !state_lock_available || !metadata_lock_available {
        "qemu-production-timer-preemption-smoke-lock-still-held"
    } else if cpu_on_ok {
        "qemu-production-timer-preemption-smoke-invariant-failed"
    } else {
        "qemu-psci-smc-cpu-on-failed"
    };
    crate::println!(
        "qemu-production-timer-preemption-smoke: final participants={} expected={} errors={} state-lock-available={} metadata-lock-available={} wait-remaining={} classification={}",
        participants,
        MAX_CORES - 1,
        errors,
        state_lock_available,
        metadata_lock_available,
        remaining,
        classification
    );

    if reports_ok {
        crate::println!("qemu-production-timer-preemption-smoke: PASS");
    } else {
        crate::println!("qemu-production-timer-preemption-smoke: FAIL");
    }

    reports_ok
}

#[cfg(talos_boot_scenario = "qemu_cross_core_ipi_delivery")]
pub fn run_cross_core_ipi_delivery_smoke() -> bool {
    smp::reset_secondary_core_states();
    CROSS_CORE_IPI_DELIVERY_STATE.reset();

    crate::arch::aarch64::exceptions::init();
    unsafe {
        aarch64::disable_irq();
        aarch64::route_physical_irqs_to_el2();
        let gic = GicV2::new(GICD_BASE, GICC_BASE);
        gic.configure_sgi_priority(QEMU_CROSS_CORE_IPI_SGI_INTID, 0x80);
        gic.enable_cpu_interface();
        gic.enable_distributor();
    }

    let boot_mpidr = aarch64::mpidr_el1();
    let boot_affinity = aarch64::mpidr_affinity(boot_mpidr);
    let boot_logical = qemu_logical_cpu_from_mpidr_affinity(boot_affinity);
    let entry = talos_aarch64_qemu_secondary_entry as *const () as usize;
    let stack_layout = secondary_stack_layout();
    let stack_base = core::ptr::addr_of!(talos_secondary_core_stacks) as usize;
    let stack_end = core::ptr::addr_of!(talos_secondary_core_stacks_end) as usize;
    let expected_mask = ((1u64 << MAX_CORES) - 1) & !1;

    crate::println!(
        "qemu-cross-core-ipi-delivery: start conduit=smc cores={} sgi-intid={} expected-mask={:#x} boot-mpidr={:#018x} boot-affinity={:#x} boot-logical={:?} entry={:#018x} stack-range=[{:#018x},{:#018x})",
        MAX_CORES,
        QEMU_CROSS_CORE_IPI_SGI_INTID,
        expected_mask,
        boot_mpidr,
        boot_affinity,
        boot_logical,
        entry,
        stack_base,
        stack_end
    );

    let mut cpu_on_ok = true;
    for logical_cpu in 1..MAX_CORES {
        let target_affinity = logical_cpu as u64;
        let result = unsafe { psci_cpu_on_smc(target_affinity, entry, logical_cpu) };
        crate::println!(
            "qemu-cross-core-ipi-delivery: cpu-on logical={} target-affinity={:#x} result={}",
            logical_cpu,
            target_affinity,
            result
        );
        cpu_on_ok &= result == 0;
    }

    let mut ready_remaining = QEMU_SECONDARY_WAIT_LIMIT;
    while ready_remaining > 0
        && (CROSS_CORE_IPI_DELIVERY_STATE
            .ready_mask
            .load(Ordering::Acquire)
            & expected_mask)
            != expected_mask
    {
        core::hint::spin_loop();
        ready_remaining -= 1;
    }

    let gic = GicV2::new(GICD_BASE, GICC_BASE);
    for logical_cpu in 1..MAX_CORES {
        let target_bit = 1u8 << logical_cpu;
        let sgir_value =
            unsafe { gic.send_sgi_to_target_list(QEMU_CROSS_CORE_IPI_SGI_INTID, target_bit) };
        CROSS_CORE_IPI_DELIVERY_STATE.record_send(logical_cpu, target_bit, sgir_value);
        crate::println!(
            "qemu-cross-core-ipi-delivery: send sender=0 target-logical={} target-list-bit={:#04x} sgi-intid={} sgir={:#010x}",
            logical_cpu,
            target_bit,
            QEMU_CROSS_CORE_IPI_SGI_INTID,
            sgir_value
        );
    }

    let mut complete_remaining = QEMU_SECONDARY_WAIT_LIMIT;
    while complete_remaining > 0
        && (CROSS_CORE_IPI_DELIVERY_STATE
            .complete_mask
            .load(Ordering::Acquire)
            & expected_mask)
            != expected_mask
    {
        core::hint::spin_loop();
        complete_remaining -= 1;
    }

    unsafe {
        aarch64::disable_irq();
    }

    let ready_mask = CROSS_CORE_IPI_DELIVERY_STATE
        .ready_mask
        .load(Ordering::Acquire);
    let complete_mask = CROSS_CORE_IPI_DELIVERY_STATE
        .complete_mask
        .load(Ordering::Acquire);
    let mut participants = 0;
    let mut reports_ok =
        cpu_on_ok && boot_logical == Some(0) && (ready_mask & expected_mask) == expected_mask;

    for logical_cpu in 1..MAX_CORES {
        let core_report = SECONDARY_CORE_STATES[logical_cpu].snapshot(logical_cpu);
        let logical_from_mpidr = qemu_logical_cpu_from_mpidr_affinity(core_report.affinity);
        let stack_slot = stack_layout
            .slot(logical_cpu)
            .expect("stack slot for possible QEMU core");
        let stack_owned = stack_slot.contains_stack_pointer(core_report.stack_pointer);
        let target_bit =
            CROSS_CORE_IPI_DELIVERY_STATE.target_bits[logical_cpu].load(Ordering::Acquire);
        let sgir_value =
            CROSS_CORE_IPI_DELIVERY_STATE.sent_values[logical_cpu].load(Ordering::Acquire);
        let receive_count =
            CROSS_CORE_IPI_DELIVERY_STATE.receive_counts[logical_cpu].load(Ordering::Acquire);
        let eoi_count =
            CROSS_CORE_IPI_DELIVERY_STATE.eoi_counts[logical_cpu].load(Ordering::Acquire);
        let last_vector =
            CROSS_CORE_IPI_DELIVERY_STATE.last_vectors[logical_cpu].load(Ordering::Acquire);
        let last_iar = CROSS_CORE_IPI_DELIVERY_STATE.last_iars[logical_cpu].load(Ordering::Acquire);
        let last_intid =
            CROSS_CORE_IPI_DELIVERY_STATE.last_intids[logical_cpu].load(Ordering::Acquire);
        let report_ok = core_report.lifecycle >= CoreLifecycle::WorkloadComplete
            && core_report.context == logical_cpu
            && logical_from_mpidr == Some(logical_cpu)
            && stack_owned
            && target_bit == (1u64 << logical_cpu)
            && receive_count == 1
            && eoi_count == 1
            && last_intid == QEMU_CROSS_CORE_IPI_SGI_INTID as u64;
        if report_ok {
            participants += 1;
        }
        reports_ok &= report_ok;

        crate::println!(
            "qemu-cross-core-ipi-delivery: report sender=0 receiver={} state={} context={} mapped={:?} target-list-bit={:#04x} sgir={:#010x} vector={} iar={:#010x} intid={} receive-count={} eoi-count={} errors={} ok={}",
            logical_cpu,
            secondary_state_name(core_report.lifecycle.raw()),
            core_report.context,
            logical_from_mpidr,
            target_bit,
            sgir_value,
            last_vector,
            last_iar,
            last_intid,
            receive_count,
            eoi_count,
            CROSS_CORE_IPI_DELIVERY_STATE.errors.load(Ordering::Acquire),
            report_ok
        );
    }

    let errors = CROSS_CORE_IPI_DELIVERY_STATE.errors.load(Ordering::Acquire);
    let classification = if reports_ok && errors == 0 {
        "qemu-cross-core-ipi-delivery-complete"
    } else if (ready_mask & expected_mask) != expected_mask {
        "qemu-cross-core-ipi-delivery-secondaries-not-ready"
    } else if cpu_on_ok {
        "qemu-cross-core-ipi-delivery-invariant-failed"
    } else {
        "qemu-psci-smc-cpu-on-failed"
    };
    crate::println!(
        "qemu-cross-core-ipi-delivery: final participants={} expected={} errors={} ready-mask={:#x} complete-mask={:#x} ready-wait-remaining={} complete-wait-remaining={} classification={}",
        participants,
        MAX_CORES - 1,
        errors,
        ready_mask,
        complete_mask,
        ready_remaining,
        complete_remaining,
        classification
    );

    if reports_ok && errors == 0 {
        crate::println!("qemu-cross-core-ipi-delivery: PASS");
    } else {
        crate::println!("qemu-cross-core-ipi-delivery: FAIL");
    }

    reports_ok && errors == 0
}

#[cfg(talos_boot_scenario = "qemu_remote_wakeup_request")]
pub fn run_remote_wakeup_request_smoke() -> bool {
    smp::reset_secondary_core_states();
    reset_remote_wakeup_request_state();

    crate::arch::aarch64::exceptions::init();
    unsafe {
        aarch64::disable_irq();
        aarch64::route_physical_irqs_to_el2();
        let gic = GicV2::new(GICD_BASE, GICC_BASE);
        gic.configure_sgi_priority(QEMU_CROSS_CORE_IPI_SGI_INTID, 0x80);
        gic.enable_cpu_interface();
        gic.enable_distributor();
    }

    let boot_mpidr = aarch64::mpidr_el1();
    let boot_affinity = aarch64::mpidr_affinity(boot_mpidr);
    let boot_logical = qemu_logical_cpu_from_mpidr_affinity(boot_affinity);
    let entry = talos_aarch64_qemu_secondary_entry as *const () as usize;
    let stack_layout = secondary_stack_layout();
    let stack_base = core::ptr::addr_of!(talos_secondary_core_stacks) as usize;
    let stack_end = core::ptr::addr_of!(talos_secondary_core_stacks_end) as usize;
    let expected_mask = ((1u64 << MAX_CORES) - 1) & !1;

    crate::println!(
        "qemu-remote-wakeup-request: start conduit=smc cores={} sgi-intid={} queue-capacity={} expected-mask={:#x} boot-mpidr={:#018x} boot-affinity={:#x} boot-logical={:?} entry={:#018x} stack-range=[{:#018x},{:#018x})",
        MAX_CORES,
        QEMU_CROSS_CORE_IPI_SGI_INTID,
        REMOTE_WAKE_QUEUE_CAPACITY,
        expected_mask,
        boot_mpidr,
        boot_affinity,
        boot_logical,
        entry,
        stack_base,
        stack_end
    );

    let mut cpu_on_ok = true;
    for logical_cpu in 1..MAX_CORES {
        let target_affinity = logical_cpu as u64;
        let result = unsafe { psci_cpu_on_smc(target_affinity, entry, logical_cpu) };
        crate::println!(
            "qemu-remote-wakeup-request: cpu-on logical={} target-affinity={:#x} result={}",
            logical_cpu,
            target_affinity,
            result
        );
        cpu_on_ok &= result == 0;
    }

    let mut ready_remaining = QEMU_SECONDARY_WAIT_LIMIT;
    while ready_remaining > 0
        && (REMOTE_WAKE_REQUEST_SMOKE_STATE
            .ready_mask
            .load(Ordering::Acquire)
            & expected_mask)
            != expected_mask
    {
        core::hint::spin_loop();
        ready_remaining -= 1;
    }

    let mut publish_ok = true;
    for logical_cpu in 1..MAX_CORES {
        let task_id = TaskId::new(200 + logical_cpu as u64).expect("diagnostic task ID is nonzero");
        publish_ok &= publish_remote_wake_request(logical_cpu, task_id);
        if logical_cpu == 1 {
            publish_ok &= publish_remote_wake_request(logical_cpu, task_id);
        }
    }

    let gic = GicV2::new(GICD_BASE, GICC_BASE);
    for logical_cpu in 1..MAX_CORES {
        let target_bit = 1u8 << logical_cpu;
        let sgir_value =
            unsafe { gic.send_sgi_to_target_list(QEMU_CROSS_CORE_IPI_SGI_INTID, target_bit) };
        REMOTE_WAKE_REQUEST_SMOKE_STATE.record_send(logical_cpu, target_bit, sgir_value);
        crate::println!(
            "qemu-remote-wakeup-request: send sender=0 target-logical={} target-list-bit={:#04x} sgi-intid={} sgir={:#010x}",
            logical_cpu,
            target_bit,
            QEMU_CROSS_CORE_IPI_SGI_INTID,
            sgir_value
        );
    }

    let mut complete_remaining = QEMU_SECONDARY_WAIT_LIMIT;
    while complete_remaining > 0
        && (REMOTE_WAKE_REQUEST_SMOKE_STATE
            .complete_mask
            .load(Ordering::Acquire)
            & expected_mask)
            != expected_mask
    {
        core::hint::spin_loop();
        complete_remaining -= 1;
    }

    unsafe {
        aarch64::disable_irq();
    }

    let ready_mask = REMOTE_WAKE_REQUEST_SMOKE_STATE
        .ready_mask
        .load(Ordering::Acquire);
    let complete_mask = REMOTE_WAKE_REQUEST_SMOKE_STATE
        .complete_mask
        .load(Ordering::Acquire);
    let mut participants = 0;
    let mut reports_ok = cpu_on_ok
        && publish_ok
        && boot_logical == Some(0)
        && (ready_mask & expected_mask) == expected_mask;

    for logical_cpu in 1..MAX_CORES {
        let core_report = SECONDARY_CORE_STATES[logical_cpu].snapshot(logical_cpu);
        let logical_from_mpidr = qemu_logical_cpu_from_mpidr_affinity(core_report.affinity);
        let stack_slot = stack_layout
            .slot(logical_cpu)
            .expect("stack slot for possible QEMU core");
        let stack_owned = stack_slot.contains_stack_pointer(core_report.stack_pointer);
        let expected_task = 200 + logical_cpu as u64;
        let target_bit =
            REMOTE_WAKE_REQUEST_SMOKE_STATE.target_bits[logical_cpu].load(Ordering::Acquire);
        let sgir_value =
            REMOTE_WAKE_REQUEST_SMOKE_STATE.sent_values[logical_cpu].load(Ordering::Acquire);
        let receive_count =
            REMOTE_WAKE_REQUEST_SMOKE_STATE.receive_counts[logical_cpu].load(Ordering::Acquire);
        let eoi_count =
            REMOTE_WAKE_REQUEST_SMOKE_STATE.eoi_counts[logical_cpu].load(Ordering::Acquire);
        let pending_count =
            REMOTE_WAKE_REQUEST_SMOKE_STATE.pending_counts[logical_cpu].load(Ordering::Acquire);
        let consumed_task =
            REMOTE_WAKE_REQUEST_SMOKE_STATE.consumed_task_ids[logical_cpu].load(Ordering::Acquire);
        let duplicate_count =
            REMOTE_WAKE_REQUEST_SMOKE_STATE.duplicate_counts[logical_cpu].load(Ordering::Acquire);
        let queue_len_after =
            REMOTE_WAKE_REQUEST_SMOKE_STATE.queue_lens_after[logical_cpu].load(Ordering::Acquire);
        let cross_owner_rejected = REMOTE_WAKE_REQUEST_SMOKE_STATE.cross_owner_rejections
            [logical_cpu]
            .load(Ordering::Acquire)
            == 1;
        let production_deferred = REMOTE_WAKE_REQUEST_SMOKE_STATE.production_deferrals[logical_cpu]
            .load(Ordering::Acquire)
            == 1;
        let last_vector =
            REMOTE_WAKE_REQUEST_SMOKE_STATE.last_vectors[logical_cpu].load(Ordering::Acquire);
        let last_iar =
            REMOTE_WAKE_REQUEST_SMOKE_STATE.last_iars[logical_cpu].load(Ordering::Acquire);
        let last_intid =
            REMOTE_WAKE_REQUEST_SMOKE_STATE.last_intids[logical_cpu].load(Ordering::Acquire);
        let report_ok = core_report.lifecycle >= CoreLifecycle::WorkloadComplete
            && core_report.context == logical_cpu
            && logical_from_mpidr == Some(logical_cpu)
            && stack_owned
            && target_bit == (1u64 << logical_cpu)
            && receive_count == 1
            && eoi_count == 1
            && pending_count == 1
            && last_intid == QEMU_CROSS_CORE_IPI_SGI_INTID as u64
            && consumed_task == expected_task
            && queue_len_after == 0
            && cross_owner_rejected
            && production_deferred
            && (logical_cpu != 1 || duplicate_count == 1)
            && (logical_cpu == 1 || duplicate_count == 0);
        #[cfg(talos_boot_scenario = "qemu_remote_wake_to_local_runnable")]
        let report_ok = {
            let mut report_ok = report_ok;
            let local_wake_task = REMOTE_WAKE_REQUEST_SMOKE_STATE.local_wake_task_ids[logical_cpu]
                .load(Ordering::Acquire);
            let local_runnable_len = REMOTE_WAKE_REQUEST_SMOKE_STATE.local_runnable_lens
                [logical_cpu]
                .load(Ordering::Acquire);
            let local_state_before = REMOTE_WAKE_REQUEST_SMOKE_STATE.local_state_before
                [logical_cpu]
                .load(Ordering::Acquire);
            let local_state_after = REMOTE_WAKE_REQUEST_SMOKE_STATE.local_state_after[logical_cpu]
                .load(Ordering::Acquire);
            let duplicate_local_rejected = REMOTE_WAKE_REQUEST_SMOKE_STATE
                .duplicate_local_rejections[logical_cpu]
                .load(Ordering::Acquire)
                == 1;
            report_ok &= local_wake_task == expected_task
                && local_runnable_len == 1
                && task_state_name(local_state_before) == "blocked"
                && task_state_name(local_state_after) == "runnable"
                && duplicate_local_rejected;
            crate::println!(
                "qemu-remote-wake-to-local-runnable: local receiver={} state-before={} state-after={} woke-task={} local-runnable-len={} duplicate-local-rejected={} ok={}",
                logical_cpu,
                task_state_name(local_state_before),
                task_state_name(local_state_after),
                local_wake_task,
                local_runnable_len,
                duplicate_local_rejected,
                report_ok
            );
            report_ok
        };
        if report_ok {
            participants += 1;
        }
        reports_ok &= report_ok;

        crate::println!(
            "qemu-remote-wakeup-request: report sender=0 receiver={} state={} context={} mapped={:?} target-list-bit={:#04x} sgir={:#010x} vector={} iar={:#010x} intid={} receive-count={} eoi-count={} pending-count={} consumed-task={} duplicate-count={} queue-len-after={} cross-owner-rejected={} production-deferred={} errors={} ok={}",
            logical_cpu,
            secondary_state_name(core_report.lifecycle.raw()),
            core_report.context,
            logical_from_mpidr,
            target_bit,
            sgir_value,
            last_vector,
            last_iar,
            last_intid,
            receive_count,
            eoi_count,
            pending_count,
            consumed_task,
            duplicate_count,
            queue_len_after,
            cross_owner_rejected,
            production_deferred,
            REMOTE_WAKE_REQUEST_SMOKE_STATE
                .errors
                .load(Ordering::Acquire),
            report_ok
        );
    }

    let errors = REMOTE_WAKE_REQUEST_SMOKE_STATE
        .errors
        .load(Ordering::Acquire);
    let classification = if reports_ok && errors == 0 {
        if cfg!(talos_boot_scenario = "qemu_remote_wake_to_local_runnable") {
            "qemu-remote-wake-to-local-runnable-complete"
        } else {
            "qemu-remote-wakeup-request-complete"
        }
    } else if (ready_mask & expected_mask) != expected_mask {
        "qemu-remote-wakeup-request-secondaries-not-ready"
    } else if cpu_on_ok {
        "qemu-remote-wakeup-request-invariant-failed"
    } else {
        "qemu-psci-smc-cpu-on-failed"
    };
    crate::println!(
        "qemu-remote-wakeup-request: final participants={} expected={} errors={} ready-mask={:#x} complete-mask={:#x} ready-wait-remaining={} complete-wait-remaining={} classification={}",
        participants,
        MAX_CORES - 1,
        errors,
        ready_mask,
        complete_mask,
        ready_remaining,
        complete_remaining,
        classification
    );

    if reports_ok && errors == 0 {
        crate::println!("qemu-remote-wakeup-request: PASS");
    } else {
        crate::println!("qemu-remote-wakeup-request: FAIL");
    }

    reports_ok && errors == 0
}

#[cfg(talos_boot_scenario = "qemu_polling_tty_rx")]
pub fn run_polling_tty_rx_diagnostic() -> bool {
    crate::println!(
        "qemu-tty-rx-diagnostic: ready capacity={} wait-limit={} backend=runtime-console0/qemu-virt-pl011",
        crate::tty::CANONICAL_LINE_CAPACITY,
        crate::tty::POLLING_RX_WAIT_LIMIT
    );

    let result = crate::tty::run_polling_rx_diagnostic(console());
    crate::println!();
    crate::println!(
        "qemu-tty-rx-diagnostic: raw-len={} line-len={} terminated={} timeout={} outcome={} truncated={} backspaces={} deletes={} controls={}",
        result.raw_bytes(),
        result.line().len(),
        result.terminated(),
        result.timed_out(),
        result.outcome_name(),
        result.truncated(),
        result.backspaces(),
        result.deletes(),
        result.controls().len()
    );
    crate::print!("qemu-tty-rx-diagnostic: line-hex=");
    print_hex_bytes(result.line());
    crate::println!();
    crate::print!("qemu-tty-rx-diagnostic: echo-hex=");
    print_hex_bytes(result.echo());
    crate::println!();
    crate::print!("qemu-tty-rx-diagnostic: control-events=");
    print_control_events(result.controls());
    crate::println!();

    if result.passed() && result.truncated() && !result.controls().is_empty() {
        crate::println!("qemu-tty-rx-diagnostic: PASS");
        true
    } else {
        crate::println!("qemu-tty-rx-diagnostic: FAIL");
        false
    }
}

#[cfg(talos_boot_scenario = "qemu_diagnostic_command_channel")]
pub fn run_diagnostic_command_channel_smoke() -> bool {
    crate::println!(
        "qemu-diagnostic-command-channel-smoke: start command-count=4 backend=runtime-console0/qemu-virt-pl011 input=tty-canonical-lite"
    );

    let mut passed = true;

    for command_index in 0..4 {
        crate::println!(
            "qemu-diagnostic-command-channel-smoke: ready command={}",
            command_index
        );

        let result = crate::tty::run_polling_rx_diagnostic(console());
        crate::println!();
        crate::print!(
            "qemu-diagnostic-command-channel-smoke: line command={} hex=",
            command_index
        );
        print_hex_bytes(result.line());
        crate::println!();

        if !result.passed() || result.truncated() || !result.controls().is_empty() {
            crate::println!(
                "qemu-diagnostic-command-channel-smoke: input-fail command={} outcome={} truncated={} controls={}",
                command_index,
                result.outcome_name(),
                result.truncated(),
                result.controls().len()
            );
            passed = false;
            continue;
        }

        let mut sink = crate::runtime_console::RuntimeConsole::new(console());
        let dispatch = crate::diagnostic_command::dispatch_default_diagnostic_command(
            result.line(),
            &mut sink,
        );
        let dispatch = match dispatch {
            Ok(dispatch) => dispatch,
            Err(_) => {
                crate::println!(
                    "qemu-diagnostic-command-channel-smoke: dispatch-fail command={} response-write-failed",
                    command_index
                );
                passed = false;
                continue;
            }
        };

        let status_name = diagnostic_dispatch_status_name(dispatch.status);
        crate::println!(
            "qemu-diagnostic-command-channel-smoke: dispatch command={} status={} responses={}",
            command_index,
            status_name,
            dispatch.response_lines
        );

        if !expected_diagnostic_dispatch(
            command_index,
            result.line(),
            dispatch.status,
            dispatch.response_lines,
        ) {
            passed = false;
        }
    }

    if passed {
        crate::println!("qemu-diagnostic-command-channel-smoke: PASS");
    } else {
        crate::println!("qemu-diagnostic-command-channel-smoke: FAIL");
    }

    passed
}

#[cfg(talos_boot_scenario = "qemu_diagnostic_command_channel")]
fn diagnostic_dispatch_status_name(
    status: crate::diagnostic_command::DiagnosticDispatchStatus,
) -> &'static str {
    match status {
        crate::diagnostic_command::DiagnosticDispatchStatus::Handled => "handled",
        crate::diagnostic_command::DiagnosticDispatchStatus::UnknownCommand => "unknown-command",
        crate::diagnostic_command::DiagnosticDispatchStatus::UnexpectedArgument => {
            "unexpected-argument"
        }
        crate::diagnostic_command::DiagnosticDispatchStatus::ParseError(_) => "parse-error",
    }
}

#[cfg(talos_boot_scenario = "qemu_diagnostic_command_channel")]
fn expected_diagnostic_dispatch(
    command_index: usize,
    line: &[u8],
    status: crate::diagnostic_command::DiagnosticDispatchStatus,
    response_lines: usize,
) -> bool {
    use crate::diagnostic_command::DiagnosticDispatchStatus::{Handled, UnknownCommand};

    match command_index {
        0 => line == b"help" && status == Handled && response_lines == 2,
        1 => line == b"list" && status == Handled && response_lines == 2,
        2 => line == b"bogus" && status == UnknownCommand && response_lines == 1,
        3 => line == b"status" && status == Handled && response_lines == 6,
        _ => false,
    }
}

#[cfg(any(
    talos_boot_scenario = "qemu_polling_tty_rx",
    talos_boot_scenario = "qemu_diagnostic_command_channel"
))]
fn print_hex_bytes(bytes: &[u8]) {
    for (index, byte) in bytes.iter().enumerate() {
        if index != 0 {
            crate::print!(" ");
        }
        crate::print!("{:02x}", byte);
    }
}

#[cfg(talos_boot_scenario = "qemu_polling_tty_rx")]
fn print_control_events(events: &[Option<crate::tty::TtyControlEvent>]) {
    if events.is_empty() {
        crate::print!("none");
        return;
    }

    for (index, event) in events.iter().enumerate() {
        if index != 0 {
            crate::print!(",");
        }
        match event {
            Some(event) => {
                crate::print!("{}", event.name());
            }
            None => {
                crate::print!("empty");
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimerIrqSnapshot {
    pub timer_count: u64,
    pub last_vector: u64,
    pub last_iar: u64,
    pub last_intid: u64,
    pub unexpected_gic_count: u64,
}

pub fn timer_irq_snapshot() -> TimerIrqSnapshot {
    TimerIrqSnapshot {
        timer_count: generic_timer::monotonic_ticks(),
        last_vector: LAST_IRQ_VECTOR.load(Ordering::Relaxed),
        last_iar: LAST_IAR.load(Ordering::Relaxed),
        last_intid: LAST_INTID.load(Ordering::Relaxed),
        unexpected_gic_count: UNEXPECTED_GIC_IRQ_COUNT.load(Ordering::Relaxed),
    }
}

pub fn handle_irq(vector: u64) -> bool {
    let gic = GicV2::new(GICD_BASE, GICC_BASE);
    let iar = unsafe { gic.acknowledge() };
    let intid = iar & 0x03ff;

    LAST_IRQ_VECTOR.store(vector, Ordering::Relaxed);
    LAST_IAR.store(iar as u64, Ordering::Relaxed);
    LAST_INTID.store(intid as u64, Ordering::Relaxed);

    #[cfg(talos_boot_scenario = "qemu_cross_core_ipi_delivery")]
    if intid == QEMU_CROSS_CORE_IPI_SGI_INTID {
        let logical_cpu = current_qemu_logical_cpu();
        CROSS_CORE_IPI_DELIVERY_STATE.record_receive(logical_cpu, vector, iar, intid);
        unsafe {
            gic.end_interrupt(iar);
        }
        CROSS_CORE_IPI_DELIVERY_STATE.record_eoi(logical_cpu);
        return true;
    }

    #[cfg(talos_boot_scenario = "qemu_remote_wakeup_request")]
    if intid == QEMU_CROSS_CORE_IPI_SGI_INTID {
        let logical_cpu = current_qemu_logical_cpu();
        REMOTE_WAKE_REQUEST_SMOKE_STATE.record_receive(logical_cpu, vector, iar, intid);
        unsafe {
            gic.end_interrupt(iar);
        }
        REMOTE_WAKE_REQUEST_SMOKE_STATE.record_eoi(logical_cpu);
        return true;
    }

    if intid == EL2_PHYSICAL_TIMER_INTID {
        unsafe { generic_timer::record_el2_physical_tick_and_rearm() };
        record_production_timer_preemption_irq(current_qemu_logical_cpu());
        #[cfg(talos_boot_scenario = "qemu_timer_preemption")]
        TIMER_PREEMPTION_REQUESTS.fetch_add(1, Ordering::Relaxed);
        unsafe {
            gic.end_interrupt(iar);
        }
        return true;
    }

    UNEXPECTED_GIC_IRQ_COUNT.fetch_add(1, Ordering::Relaxed);
    if intid != SPURIOUS_INTID {
        unsafe {
            gic.end_interrupt(iar);
        }
    }
    true
}

#[cfg(talos_boot_scenario = "qemu_timer_preemption")]
extern "C" fn qemu_timer_preemption_thread(raw_task_index: usize) -> ! {
    let task_index = raw_task_index & 1;
    loop {
        unsafe {
            let state = TIMER_PREEMPTION_SMOKE.get();
            (*state).current_task = task_index as u64 + 1;

            if (*state).proof_complete() {
                (*state).runnable_task = (*state)
                    .scheduler
                    .runnable()
                    .front()
                    .map_or(0, |task_id| task_id.raw());
                aarch64::cooperative_context_switch(
                    core::ptr::addr_of_mut!((*state).worker_contexts[task_index]),
                    core::ptr::addr_of!((*state).main_context),
                );
            }

            let request_count = TIMER_PREEMPTION_REQUESTS.load(Ordering::Relaxed);
            if request_count != (*state).handled_requests {
                (*state).progress[task_index] += 1;
                let irq_state = aarch64::single_core_irq_mask_save();
                let next_task_index =
                    (*state).dispatch_timer_preemption_from(task_index, request_count);
                aarch64::single_core_irq_restore(irq_state);
                aarch64::cooperative_context_switch(
                    core::ptr::addr_of_mut!((*state).worker_contexts[task_index]),
                    core::ptr::addr_of!((*state).worker_contexts[next_task_index]),
                );
            }
        }

        unsafe {
            core::arch::asm!("wfe", options(nomem, nostack, preserves_flags));
        }
    }
}

#[cfg(talos_boot_scenario = "qemu_timer_preemption")]
pub fn run_el2_timer_preemption_smoke() -> bool {
    let _keep_timer_smoke_reachable: fn() -> bool = run_el2_timer_irq_smoke;

    unsafe {
        aarch64::disable_irq();
        aarch64::route_physical_irqs_to_el2();
        generic_timer::mask_el2_physical_timer();
    }
    unsafe {
        GicV2::new(GICD_BASE, GICC_BASE).enable_ppi_or_spi(EL2_PHYSICAL_TIMER_INTID);
    }
    LAST_IRQ_VECTOR.store(0, Ordering::Relaxed);
    LAST_IAR.store(0, Ordering::Relaxed);
    LAST_INTID.store(0, Ordering::Relaxed);
    UNEXPECTED_GIC_IRQ_COUNT.store(0, Ordering::Relaxed);
    TIMER_PREEMPTION_REQUESTS.store(0, Ordering::Relaxed);
    generic_timer::reset_monotonic_ticks();

    let freq = generic_timer::counter_frequency_hz();
    let start = generic_timer::physical_count();
    let delta = generic_timer::periodic_tick_delta_ticks(freq);
    let compare = start.wrapping_add(delta);
    generic_timer::configure_periodic_tick_delta(delta);

    unsafe {
        let state = TIMER_PREEMPTION_SMOKE.get();
        (*state).reset();
        crate::println!(
            "qemu-timer-preemption-smoke: stack0={:#018x} stack1={:#018x} trampoline={:#018x}",
            (*state).worker_stacks[0].top(),
            (*state).worker_stacks[1].top(),
            aarch64::kernel_thread_trampoline_address()
        );
        crate::println!(
            "qemu-timer-preemption-smoke: gicd={:#010x} gicc={:#010x} intid={} cntfrq={} start={} cval={} delta={}",
            GICD_BASE,
            GICC_BASE,
            EL2_PHYSICAL_TIMER_INTID,
            freq,
            start,
            compare,
            delta
        );
        crate::println!(
            "qemu-timer-preemption-smoke: start current={} runnable={} preempted={} requests={}",
            (*state).current_task,
            (*state).runnable_task,
            (*state).preempted_task,
            TIMER_PREEMPTION_REQUESTS.load(Ordering::Relaxed)
        );

        generic_timer::program_el2_physical_compare(compare);
        aarch64::enable_irq();
        aarch64::cooperative_context_switch(
            core::ptr::addr_of_mut!((*state).main_context),
            core::ptr::addr_of!((*state).worker_contexts[0]),
        );
    }

    unsafe {
        aarch64::disable_irq();
    }

    let (
        progress0,
        progress1,
        state_transitions,
        voluntary_yields,
        timer_preemptions,
        dispatch_switches,
        handled_requests,
        current_task,
        runnable_task,
        preempted_task,
    ) = unsafe {
        let state = TIMER_PREEMPTION_SMOKE.get();
        let counters = (*state).scheduler.counters();
        (
            (*state).progress[0],
            (*state).progress[1],
            counters.state_transitions(),
            counters.voluntary_yields(),
            counters.timer_preemptions(),
            counters.context_switches(),
            (*state).handled_requests,
            (*state).current_task,
            (*state).runnable_task,
            (*state).preempted_task,
        )
    };
    let snapshot = timer_irq_snapshot();
    let daif = aarch64::daif();
    let control = generic_timer::el2_physical_control();

    crate::println!(
        "qemu-timer-preemption-smoke: progress task1={} task2={} ticks={} requests={} handled={} timer-preemptions={} dispatch-switches={} voluntary-yields={} transitions={} current={} runnable={} preempted={}",
        progress0,
        progress1,
        snapshot.timer_count,
        TIMER_PREEMPTION_REQUESTS.load(Ordering::Relaxed),
        handled_requests,
        timer_preemptions,
        dispatch_switches,
        voluntary_yields,
        state_transitions,
        current_task,
        runnable_task,
        preempted_task
    );
    crate::println!(
        "qemu-timer-preemption-smoke: irq vector={} iar={:#010x} intid={} unexpected={} ctl={:#x} daif={:#x}",
        snapshot.last_vector,
        snapshot.last_iar,
        snapshot.last_intid,
        snapshot.unexpected_gic_count,
        control,
        daif
    );

    let passed = progress0 >= TIMER_PREEMPTION_TARGET_PROGRESS
        && progress1 >= TIMER_PREEMPTION_TARGET_PROGRESS
        && snapshot.timer_count >= TIMER_PREEMPTION_TARGET_SWITCHES
        && handled_requests >= TIMER_PREEMPTION_TARGET_SWITCHES
        && timer_preemptions >= TIMER_PREEMPTION_TARGET_SWITCHES
        && dispatch_switches == timer_preemptions
        && voluntary_yields == 0
        && snapshot.last_intid == EL2_PHYSICAL_TIMER_INTID as u64
        && snapshot.unexpected_gic_count == 0
        && current_task != 0
        && runnable_task != 0
        && preempted_task != 0;

    if passed {
        crate::println!("qemu-timer-preemption-smoke: PASS");
    } else {
        crate::println!("qemu-timer-preemption-smoke: FAIL");
    }

    passed
}

#[cfg(talos_boot_scenario = "qemu_scheduler_yield")]
extern "C" fn qemu_scheduler_yield_thread(raw_task_index: usize) -> ! {
    let task_index = raw_task_index & 1;
    loop {
        unsafe {
            let state = SCHEDULER_YIELD_SMOKE.get();
            (*state).current_task = task_index as u64 + 1;
            (*state).progress[task_index] += 1;

            if (*state).all_workers_made_progress() {
                (*state).runnable_task = (*state)
                    .scheduler
                    .runnable()
                    .front()
                    .map_or(0, |task_id| task_id.raw());
                aarch64::cooperative_context_switch(
                    core::ptr::addr_of_mut!((*state).worker_contexts[task_index]),
                    core::ptr::addr_of!((*state).main_context),
                );
            } else {
                // This is the accepted single-core critical section: scheduler-owned
                // queue/current/yielded state is mutated with IRQs masked, and the
                // section performs no allocation, formatting, printing, or callbacks.
                let irq_state = aarch64::single_core_irq_mask_save();
                let next_task_index = (*state).dispatch_voluntary_yield_from(task_index);
                aarch64::single_core_irq_restore(irq_state);
                aarch64::cooperative_context_switch(
                    core::ptr::addr_of_mut!((*state).worker_contexts[task_index]),
                    core::ptr::addr_of!((*state).worker_contexts[next_task_index]),
                );
            }
        }
    }
}

#[cfg(talos_boot_scenario = "qemu_scheduler_yield")]
pub fn run_el2_scheduler_yield_smoke() -> bool {
    let _keep_timer_smoke_reachable: fn() -> bool = run_el2_timer_irq_smoke;

    unsafe {
        aarch64::disable_irq();
    }

    unsafe {
        let state = SCHEDULER_YIELD_SMOKE.get();
        (*state).reset();
        crate::println!(
            "qemu-scheduler-yield-smoke: stack0={:#018x} stack1={:#018x} trampoline={:#018x}",
            (*state).worker_stacks[0].top(),
            (*state).worker_stacks[1].top(),
            aarch64::kernel_thread_trampoline_address()
        );
        crate::println!(
            "qemu-scheduler-yield-smoke: start current={} runnable={} yielded={}",
            (*state).current_task,
            (*state).runnable_task,
            (*state).yielded_task
        );

        aarch64::cooperative_context_switch(
            core::ptr::addr_of_mut!((*state).main_context),
            core::ptr::addr_of!((*state).worker_contexts[0]),
        );
    }

    unsafe {
        aarch64::disable_irq();
    }

    let (
        progress0,
        progress1,
        state_transitions,
        voluntary_yields,
        dispatch_switches,
        current_task,
        runnable_task,
        yielded_task,
    ) = unsafe {
        let state = SCHEDULER_YIELD_SMOKE.get();
        let counters = (*state).scheduler.counters();
        (
            (*state).progress[0],
            (*state).progress[1],
            counters.state_transitions(),
            counters.voluntary_yields(),
            counters.context_switches(),
            (*state).current_task,
            (*state).runnable_task,
            (*state).yielded_task,
        )
    };

    crate::println!(
        "qemu-scheduler-yield-smoke: progress task1={} task2={} yields={} dispatch-switches={} transitions={} current={} runnable={} yielded={}",
        progress0,
        progress1,
        voluntary_yields,
        dispatch_switches,
        state_transitions,
        current_task,
        runnable_task,
        yielded_task
    );

    let passed = progress0 >= SCHEDULER_YIELD_TARGET_PROGRESS
        && progress1 >= SCHEDULER_YIELD_TARGET_PROGRESS
        && voluntary_yields >= 5
        && dispatch_switches == voluntary_yields
        && state_transitions >= voluntary_yields
        && current_task != 0
        && runnable_task != 0
        && yielded_task != 0;

    if passed {
        crate::println!("qemu-scheduler-yield-smoke: PASS");
    } else {
        crate::println!("qemu-scheduler-yield-smoke: FAIL");
    }

    passed
}

#[cfg(talos_boot_scenario = "qemu_context_switch")]
extern "C" fn qemu_context_switch_thread(raw_task_index: usize) -> ! {
    let task_index = raw_task_index & 1;
    loop {
        unsafe {
            let state = CONTEXT_SWITCH_SMOKE.get();
            (*state).current_task = task_index as u64 + 1;
            (*state).progress[task_index] += 1;

            if (*state).all_workers_made_progress() {
                (*state).runnable_task = 0;
                (*state).switch_count += 1;
                aarch64::cooperative_context_switch(
                    core::ptr::addr_of_mut!((*state).worker_contexts[task_index]),
                    core::ptr::addr_of!((*state).main_context),
                );
            } else {
                let next_task_index = 1 - task_index;
                (*state).runnable_task = next_task_index as u64 + 1;
                (*state).switch_count += 1;
                aarch64::cooperative_context_switch(
                    core::ptr::addr_of_mut!((*state).worker_contexts[task_index]),
                    core::ptr::addr_of!((*state).worker_contexts[next_task_index]),
                );
            }
        }
    }
}

#[cfg(talos_boot_scenario = "qemu_context_switch")]
pub fn run_el2_context_switch_smoke() -> bool {
    let _keep_timer_smoke_reachable: fn() -> bool = run_el2_timer_irq_smoke;

    unsafe {
        aarch64::disable_irq();
    }

    unsafe {
        let state = CONTEXT_SWITCH_SMOKE.get();
        (*state).reset();
        crate::println!(
            "qemu-context-switch-smoke: stack0={:#018x} stack1={:#018x} trampoline={:#018x}",
            (*state).worker_stacks[0].top(),
            (*state).worker_stacks[1].top(),
            aarch64::kernel_thread_trampoline_address()
        );
        crate::println!(
            "qemu-context-switch-smoke: start current={} runnable={}",
            (*state).current_task,
            (*state).runnable_task
        );

        (*state).current_task = 0;
        (*state).runnable_task = 1;
        (*state).switch_count += 1;
        aarch64::cooperative_context_switch(
            core::ptr::addr_of_mut!((*state).main_context),
            core::ptr::addr_of!((*state).worker_contexts[0]),
        );
    }

    unsafe {
        aarch64::disable_irq();
    }

    let (progress0, progress1, switch_count, current_task, runnable_task) = unsafe {
        let state = CONTEXT_SWITCH_SMOKE.get();
        (
            (*state).progress[0],
            (*state).progress[1],
            (*state).switch_count,
            (*state).current_task,
            (*state).runnable_task,
        )
    };

    crate::println!(
        "qemu-context-switch-smoke: progress task1={} task2={} switches={} current={} runnable={}",
        progress0,
        progress1,
        switch_count,
        current_task,
        runnable_task
    );

    let passed = progress0 >= CONTEXT_SWITCH_TARGET_PROGRESS
        && progress1 >= CONTEXT_SWITCH_TARGET_PROGRESS
        && switch_count >= 5
        && current_task != 0
        && runnable_task == 0;

    if passed {
        crate::println!("qemu-context-switch-smoke: PASS");
    } else {
        crate::println!("qemu-context-switch-smoke: FAIL");
    }

    passed
}

fn run_single_core_irq_mask_probe() -> SingleCoreIrqMaskProbe {
    unsafe {
        aarch64::disable_irq();
    }
    let nested_start_masked = aarch64::irq_masked();
    let outer = unsafe { aarch64::single_core_irq_mask_save() };
    let inner = unsafe { aarch64::single_core_irq_mask_save() };
    unsafe {
        aarch64::single_core_irq_restore(inner);
    }
    let inner_restored_masked = aarch64::irq_masked();
    unsafe {
        aarch64::single_core_irq_restore(outer);
    }
    let outer_restored_masked = aarch64::irq_masked();

    unsafe {
        aarch64::enable_irq();
    }
    let unmasked_start = !aarch64::irq_masked();
    let unmasked = unsafe { aarch64::single_core_irq_mask_save() };
    let saved_unmasked_masked = !unmasked.was_irq_masked() && aarch64::irq_masked();
    unsafe {
        aarch64::single_core_irq_restore(unmasked);
    }
    let restored_unmasked = !aarch64::irq_masked();
    unsafe {
        aarch64::disable_irq();
    }

    SingleCoreIrqMaskProbe {
        nested_start_masked,
        inner_restored_masked,
        outer_restored_masked,
        unmasked_start,
        saved_unmasked_masked,
        restored_unmasked,
    }
}

#[cfg(test)]
mod tests {
    use super::qemu_logical_cpu_from_mpidr_affinity;

    #[test_case]
    fn qemu_mpidr_affinity_maps_four_virt_cpus() {
        assert_eq!(qemu_logical_cpu_from_mpidr_affinity(0), Some(0));
        assert_eq!(qemu_logical_cpu_from_mpidr_affinity(1), Some(1));
        assert_eq!(qemu_logical_cpu_from_mpidr_affinity(2), Some(2));
        assert_eq!(qemu_logical_cpu_from_mpidr_affinity(3), Some(3));
        assert_eq!(qemu_logical_cpu_from_mpidr_affinity(0x100), None);
    }
}

pub fn run_el2_timer_irq_smoke() -> bool {
    unsafe {
        aarch64::disable_irq();
        aarch64::route_physical_irqs_to_el2();
        generic_timer::mask_el2_physical_timer();
    }
    let irq_mask_probe = run_single_core_irq_mask_probe();
    unsafe {
        GicV2::new(GICD_BASE, GICC_BASE).enable_ppi_or_spi(EL2_PHYSICAL_TIMER_INTID);
    }
    LAST_IRQ_VECTOR.store(0, Ordering::Relaxed);
    LAST_IAR.store(0, Ordering::Relaxed);
    LAST_INTID.store(0, Ordering::Relaxed);
    UNEXPECTED_GIC_IRQ_COUNT.store(0, Ordering::Relaxed);
    generic_timer::reset_monotonic_ticks();

    let freq = generic_timer::counter_frequency_hz();
    let start = generic_timer::physical_count();
    let delta = generic_timer::periodic_tick_delta_ticks(freq);
    let compare = start.wrapping_add(delta);
    let target_ticks = generic_timer::periodic_tick_proof_count();
    generic_timer::configure_periodic_tick_delta(delta);

    crate::println!(
        "qemu-timer-irq-smoke: gicd={:#010x} gicc={:#010x} intid={}",
        GICD_BASE,
        GICC_BASE,
        EL2_PHYSICAL_TIMER_INTID
    );
    crate::println!(
        "qemu-timer-irq-smoke: cntfrq={} start={} cval={} delta={} target-ticks={}",
        freq,
        start,
        compare,
        delta,
        target_ticks
    );
    crate::println!(
        "qemu-timer-irq-smoke: irq-mask nested-start={} inner-restored={} outer-restored={} unmasked-start={} saved-mask={} restored-unmasked={}",
        irq_mask_probe.nested_start_masked,
        irq_mask_probe.inner_restored_masked,
        irq_mask_probe.outer_restored_masked,
        irq_mask_probe.unmasked_start,
        irq_mask_probe.saved_unmasked_masked,
        irq_mask_probe.restored_unmasked
    );

    let mut workload = 0x1234_5678_9abc_def0u64;
    unsafe {
        generic_timer::program_el2_physical_compare(compare);
        aarch64::enable_irq();
    }

    let mut remaining = TIMER_IRQ_WAIT_LIMIT;
    let mut critical_sections = 0usize;
    while timer_irq_snapshot().timer_count < target_ticks && remaining > 0 {
        let saved_irq_state = unsafe { aarch64::single_core_irq_mask_save() };
        workload = workload.rotate_left(7) ^ 0x0f0e_0d0c_0b0a_0908;
        unsafe {
            aarch64::single_core_irq_restore(saved_irq_state);
        }
        critical_sections += 1;
        unsafe {
            core::arch::asm!("wfe", options(nomem, nostack, preserves_flags));
        }
        remaining -= 1;
    }

    unsafe {
        aarch64::disable_irq();
    }

    let snapshot = timer_irq_snapshot();
    let gic = GicV2::new(GICD_BASE, GICC_BASE);
    let (enable_bits, pending_bits, active_bits, highest_pending) = unsafe {
        (
            gic.enable_bits(EL2_PHYSICAL_TIMER_INTID),
            gic.pending_bits(EL2_PHYSICAL_TIMER_INTID),
            gic.active_bits(EL2_PHYSICAL_TIMER_INTID),
            gic.highest_pending(),
        )
    };
    let daif = aarch64::daif();
    let control = generic_timer::el2_physical_control();
    crate::println!(
        "qemu-timer-irq-smoke: tick-count={} target={} vector={} iar={:#010x} intid={} unexpected={} ctl={:#x}",
        snapshot.timer_count,
        target_ticks,
        snapshot.last_vector,
        snapshot.last_iar,
        snapshot.last_intid,
        snapshot.unexpected_gic_count,
        control
    );
    crate::println!(
        "qemu-timer-irq-smoke: gic enable={:#010x} pending={:#010x} active={:#010x} hppir={:#010x} daif={:#x}",
        enable_bits,
        pending_bits,
        active_bits,
        highest_pending,
        daif
    );
    crate::println!(
        "qemu-timer-irq-smoke: post-irq workload={:#018x} remaining={} critical-sections={}",
        workload,
        remaining,
        critical_sections
    );

    let passed = snapshot.timer_count > 0
        && snapshot.timer_count >= target_ticks
        && snapshot.last_intid == EL2_PHYSICAL_TIMER_INTID as u64
        && snapshot.unexpected_gic_count == 0
        && irq_mask_probe.passed()
        && critical_sections > 0;

    if passed {
        crate::println!("qemu-timer-irq-smoke: PASS");
    } else {
        crate::println!("qemu-timer-irq-smoke: FAIL");
    }

    passed
}

#[cfg(talos_boot_scenario = "qemu_live_address_space_activation_smoke")]
pub fn run_live_address_space_activation_smoke() -> bool {
    crate::println!("qemu-live-address-space-activation-smoke: start");

    let (
        success_ok,
        ttbr_ok,
        compatibility_ok,
        blocked_ok,
        reachability_ok,
        binding_ok,
        effects_ok,
    ) = live_address_space_activation_report_success();
    let teardown_ok = live_address_space_activation_report_teardown();
    let identity_ok = live_address_space_activation_report_error(
        "identity-mismatch",
        live_address_space_activation_identity_mismatch_fixture(),
        LiveAddressSpaceActivationRequest::PreflightOnly,
        LiveAddressSpaceActivationLeaseSource::for_single_plan(),
        PosixError::InvalidArgument,
    );
    let descriptor_ok = live_address_space_activation_report_error(
        "entry-stack-descriptor-disagreement",
        live_address_space_activation_missing_descriptor_fixture(),
        LiveAddressSpaceActivationRequest::PreflightOnly,
        LiveAddressSpaceActivationLeaseSource::for_single_plan(),
        PosixError::NotExecutable,
    );
    let forbidden_ok = live_address_space_activation_report_error(
        "forbidden-range",
        live_address_space_activation_valid_fixture(),
        LiveAddressSpaceActivationRequest::ForbiddenRangeProbe,
        LiveAddressSpaceActivationLeaseSource::for_single_plan(),
        PosixError::AccessDenied,
    );
    let reachability_error_ok = live_address_space_activation_report_error(
        "missing-kernel-reachability",
        live_address_space_activation_valid_fixture(),
        LiveAddressSpaceActivationRequest::MissingKernelReachabilityProbe,
        LiveAddressSpaceActivationLeaseSource::for_single_plan(),
        PosixError::InvalidArgument,
    );
    let live_register_ok = live_address_space_activation_report_error(
        "live-register-request",
        live_address_space_activation_valid_fixture(),
        LiveAddressSpaceActivationRequest::LiveRegisterSequence,
        LiveAddressSpaceActivationLeaseSource::for_single_plan(),
        PosixError::NotImplemented,
    );
    let scheduler_publication_ok = live_address_space_activation_report_error(
        "scheduler-publication-request",
        live_address_space_activation_valid_fixture(),
        LiveAddressSpaceActivationRequest::PublishSchedulerRunnable,
        LiveAddressSpaceActivationLeaseSource::for_single_plan(),
        PosixError::NotImplemented,
    );
    let lower_el_ok = live_address_space_activation_report_error(
        "lower-el-launch-request",
        live_address_space_activation_valid_fixture(),
        LiveAddressSpaceActivationRequest::LowerElLaunch,
        LiveAddressSpaceActivationLeaseSource::for_single_plan(),
        PosixError::NotImplemented,
    );
    let resource_ok = live_address_space_activation_report_error(
        "resource-exhaustion",
        live_address_space_activation_valid_fixture(),
        LiveAddressSpaceActivationRequest::PreflightOnly,
        LiveAddressSpaceActivationLeaseSource::with_plan_record_capacity(0),
        PosixError::NoMemory,
    );

    let participants = u64::from(success_ok)
        + u64::from(ttbr_ok)
        + u64::from(compatibility_ok)
        + u64::from(blocked_ok)
        + u64::from(reachability_ok)
        + u64::from(binding_ok)
        + u64::from(effects_ok)
        + u64::from(teardown_ok)
        + u64::from(identity_ok)
        + u64::from(descriptor_ok)
        + u64::from(forbidden_ok)
        + u64::from(reachability_error_ok)
        + u64::from(live_register_ok && scheduler_publication_ok && lower_el_ok)
        + u64::from(resource_ok)
        + u64::from(effects_ok && !live_address_space_activation_runnable_publication_visible());
    let errors = 15 - participants;
    let classification = if participants == 15 && errors == 0 {
        "qemu-live-address-space-activation-smoke-complete"
    } else {
        "qemu-live-address-space-activation-smoke-failed"
    };

    crate::println!(
        "qemu-live-address-space-activation-smoke: final participants={} expected=15 errors={} classification={}",
        participants,
        errors,
        classification
    );
    if participants == 15 && errors == 0 {
        crate::println!("qemu-live-address-space-activation-smoke: PASS");
        true
    } else {
        crate::println!("qemu-live-address-space-activation-smoke: FAIL");
        false
    }
}

#[cfg(talos_boot_scenario = "qemu_live_address_space_activation_smoke")]
fn live_address_space_activation_report_success() -> (bool, bool, bool, bool, bool, bool, bool) {
    let Ok((image, install_plan, address_space, materialization, launch_plan, stack_plan)) =
        live_address_space_activation_valid_fixture()
    else {
        live_address_space_activation_report_empty_success();
        return (false, false, false, false, false, false, false);
    };
    let mut activation_source = LiveAddressSpaceActivationLeaseSource::for_single_plan();
    let Ok(plan) = preflight_live_address_space_activation(
        image,
        install_plan,
        address_space,
        materialization,
        launch_plan,
        stack_plan,
        LiveAddressSpaceActivationRequest::PreflightOnly,
        &mut activation_source,
    ) else {
        live_address_space_activation_report_empty_success();
        return (false, false, false, false, false, false, false);
    };

    crate::println!(
        "qemu-live-address-space-activation-smoke: fixture name={} path=/bin/init source-digest={:#x} install-boundary={} address-space-boundary={} materialization-boundary={} launch-boundary={} stack-boundary={} activation-boundary={} activation-policy={}",
        PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY,
        plan.source_digest(),
        PROCESS_INSTALL_BOUNDARY_IDENTITY,
        PROCESS_ADDRESS_SPACE_BOUNDARY_IDENTITY,
        PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY,
        INITIAL_PROCESS_LAUNCH_BOUNDARY_IDENTITY,
        INITIAL_USER_STACK_BOUNDARY_IDENTITY,
        LIVE_ADDRESS_SPACE_ACTIVATION_BOUNDARY_IDENTITY,
        LIVE_ADDRESS_SPACE_ACTIVATION_POLICY
    );

    let success_ok = plan.published()
        && plan.boundary_identity() == LIVE_ADDRESS_SPACE_ACTIVATION_BOUNDARY_IDENTITY
        && plan.activation_policy() == LIVE_ADDRESS_SPACE_ACTIVATION_POLICY
        && plan.image_fixture_identity() == PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY
        && plan.install_boundary_identity() == PROCESS_INSTALL_BOUNDARY_IDENTITY
        && plan.address_space_boundary_identity() == PROCESS_ADDRESS_SPACE_BOUNDARY_IDENTITY
        && plan.materialization_boundary_identity()
            == PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY
        && plan.launch_boundary_identity() == INITIAL_PROCESS_LAUNCH_BOUNDARY_IDENTITY
        && plan.stack_boundary_identity() == INITIAL_USER_STACK_BOUNDARY_IDENTITY
        && plan.source_path() == PHASE8_INIT_PATH
        && plan.source_digest() == image.source_digest()
        && plan.address_space_id() == address_space.id().raw()
        && plan.materialization_id() == materialization.id()
        && plan.entry_pc() == image.entry()
        && plan.initial_sp() == stack_plan.layout().initial_sp();
    crate::println!(
        "qemu-live-address-space-activation-smoke: success output=LiveAddressSpaceActivationPlan published={} copied-identities={} activation-boundary={} activation-policy={} ok={}",
        plan.published(),
        success_ok,
        plan.boundary_identity(),
        plan.activation_policy(),
        success_ok
    );

    let root = plan.root_provenance();
    let ttbr_ok = root.state() == TTBR0_ROOT_PROVENANCE
        && root.materialization_id() == materialization.id()
        && root.root_token() == materialization.root().token().raw()
        && !root.ttbr0_written()
        && plan.ttbr1_kernel_policy() == TTBR1_KERNEL_POLICY_BLOCKED;
    crate::println!(
        "qemu-live-address-space-activation-smoke: ttbr-provenance ttbr0-root={} ttbr0-written={} ttbr1-policy={} ok={}",
        root.state(),
        root.ttbr0_written(),
        plan.ttbr1_kernel_policy(),
        ttbr_ok
    );

    let compatibility_ok = plan.tcr_state() == TCR_COMPATIBILITY_RECORD_ONLY
        && plan.mair_state() == MAIR_COMPATIBILITY_RECORD_ONLY
        && plan.sctlr_state() == SCTLR_MUTATION_BLOCKED;
    crate::println!(
        "qemu-live-address-space-activation-smoke: compatibility tcr-state={} mair-state={} sctlr-state={} ok={}",
        plan.tcr_state(),
        plan.mair_state(),
        plan.sctlr_state(),
        compatibility_ok
    );

    let blocked_ok = plan.asid_state() == ASID_ALLOCATION_BLOCKED
        && plan.tlb_state() == TLB_INVALIDATION_BLOCKED
        && plan.barrier_state() == BARRIER_SEQUENCE_PLANNED_ONLY
        && plan.live_register_sequence_state() == LIVE_REGISTER_SEQUENCE_BLOCKED;
    crate::println!(
        "qemu-live-address-space-activation-smoke: blocked-states asid={} tlb={} barriers={} live-register-sequence={} ok={}",
        plan.asid_state(),
        plan.tlb_state(),
        plan.barrier_state(),
        plan.live_register_sequence_state(),
        blocked_ok
    );

    let reachability = plan.kernel_reachability();
    let reachability_ok = reachability.vbar_el1()
        && reachability.exception_vectors()
        && reachability.active_kernel_stack()
        && reachability.kernel_text_data()
        && reachability.allocator()
        && reachability.uart_mmio_diagnostics()
        && reachability.scheduler_code_data()
        && reachability.panic_fault_reporting();
    crate::println!(
        "qemu-live-address-space-activation-smoke: kernel-reachability vbar={} vectors={} active-stack={} kernel-text-data={} allocator={} uart-mmio-diagnostics={} scheduler-code-data={} panic-fault-reporting={} ok={}",
        reachability.vbar_el1(),
        reachability.exception_vectors(),
        reachability.active_kernel_stack(),
        reachability.kernel_text_data(),
        reachability.allocator(),
        reachability.uart_mmio_diagnostics(),
        reachability.scheduler_code_data(),
        reachability.panic_fault_reporting(),
        reachability_ok
    );

    let binding_ok = launch_plan.activation_state() == INITIAL_ACTIVATION_BLOCKED
        && plan.launch_activation_state() == ACTIVATION_PREFLIGHT_READY
        && !plan.side_effects().lower_el_eret()
        && !plan.side_effects().scheduler_published();
    crate::println!(
        "qemu-live-address-space-activation-smoke: launch-binding previous={} next={} lower-el-eret={} scheduler-published={} ok={}",
        launch_plan.activation_state(),
        plan.launch_activation_state(),
        plan.side_effects().lower_el_eret(),
        plan.side_effects().scheduler_published(),
        binding_ok
    );

    let effects = plan.side_effects();
    let effects_ok = !effects.ttbr_mutated()
        && !effects.tcr_mutated()
        && !effects.mair_mutated()
        && !effects.sctlr_mutated()
        && !effects.asid_allocated()
        && !effects.tlb_mutated()
        && !effects.live_dsb_isb()
        && !effects.lower_el_eret()
        && !effects.scheduler_published()
        && !effects.process_table_mutated()
        && !effects.descriptor_table_mutated();
    crate::println!(
        "qemu-live-address-space-activation-smoke: side-effects ttbr-mutated={} tcr-mutated={} mair-mutated={} sctlr-mutated={} asid-allocated={} tlb-mutated={} live-dsb-isb={} lower-el-eret={} scheduler-published={} process-table-mutated={} descriptor-table-mutated={} ok={}",
        effects.ttbr_mutated(),
        effects.tcr_mutated(),
        effects.mair_mutated(),
        effects.sctlr_mutated(),
        effects.asid_allocated(),
        effects.tlb_mutated(),
        effects.live_dsb_isb(),
        effects.lower_el_eret(),
        effects.scheduler_published(),
        effects.process_table_mutated(),
        effects.descriptor_table_mutated(),
        effects_ok
    );

    (
        success_ok,
        ttbr_ok,
        compatibility_ok,
        blocked_ok,
        reachability_ok,
        binding_ok,
        effects_ok,
    )
}

#[cfg(talos_boot_scenario = "qemu_live_address_space_activation_smoke")]
fn live_address_space_activation_report_empty_success() {
    crate::println!(
        "qemu-live-address-space-activation-smoke: fixture name={} path=/bin/init source-digest=0x0 install-boundary={} address-space-boundary={} materialization-boundary={} launch-boundary={} stack-boundary={} activation-boundary={} activation-policy={}",
        PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY,
        PROCESS_INSTALL_BOUNDARY_IDENTITY,
        PROCESS_ADDRESS_SPACE_BOUNDARY_IDENTITY,
        PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY,
        INITIAL_PROCESS_LAUNCH_BOUNDARY_IDENTITY,
        INITIAL_USER_STACK_BOUNDARY_IDENTITY,
        LIVE_ADDRESS_SPACE_ACTIVATION_BOUNDARY_IDENTITY,
        LIVE_ADDRESS_SPACE_ACTIVATION_POLICY
    );
    crate::println!(
        "qemu-live-address-space-activation-smoke: success output=LiveAddressSpaceActivationPlan published=false copied-identities=false activation-boundary={} activation-policy={} ok=false",
        LIVE_ADDRESS_SPACE_ACTIVATION_BOUNDARY_IDENTITY,
        LIVE_ADDRESS_SPACE_ACTIVATION_POLICY
    );
}

#[cfg(talos_boot_scenario = "qemu_live_address_space_activation_smoke")]
fn live_address_space_activation_report_teardown() -> bool {
    let Ok((image, install_plan, address_space, materialization, launch_plan, stack_plan)) =
        live_address_space_activation_valid_fixture()
    else {
        crate::println!(
            "qemu-live-address-space-activation-smoke: teardown plan-local-released=false materialization-owned=false launch-owned=false stack-owned=false image-owned=false idempotent=false ok=false"
        );
        return false;
    };
    let mut activation_source = LiveAddressSpaceActivationLeaseSource::for_single_plan();
    let Ok(mut plan) = preflight_live_address_space_activation(
        image,
        install_plan,
        address_space,
        materialization,
        launch_plan,
        stack_plan,
        LiveAddressSpaceActivationRequest::PreflightOnly,
        &mut activation_source,
    ) else {
        crate::println!(
            "qemu-live-address-space-activation-smoke: teardown plan-local-released=false materialization-owned=false launch-owned=false stack-owned=false image-owned=false idempotent=false ok=false"
        );
        return false;
    };

    let first = plan.destroy(&mut activation_source);
    let second = plan.destroy(&mut activation_source);
    let plan_local_released =
        first.plan_record_released() && activation_source.outstanding_leases() == 0;
    let materialization_owned = first.materialization_owned() && second.materialization_owned();
    let launch_owned = first.launch_owned() && second.launch_owned();
    let stack_owned = first.stack_owned() && second.stack_owned();
    let image_owned = first.image_owned() && second.image_owned();
    let idempotent = !first.already_destroyed()
        && second.already_destroyed()
        && !second.plan_record_released()
        && !plan.published()
        && plan.destroyed();
    let ok = plan_local_released
        && materialization_owned
        && launch_owned
        && stack_owned
        && image_owned
        && idempotent;
    crate::println!(
        "qemu-live-address-space-activation-smoke: teardown plan-local-released={} materialization-owned={} launch-owned={} stack-owned={} image-owned={} idempotent={} ok={}",
        plan_local_released,
        materialization_owned,
        launch_owned,
        stack_owned,
        image_owned,
        idempotent,
        ok
    );
    ok
}

#[cfg(talos_boot_scenario = "qemu_live_address_space_activation_smoke")]
fn live_address_space_activation_report_error(
    case: &str,
    fixture: Result<
        (
            ProgramImagePlan,
            ProcessImageInstallPlan,
            ProcessAddressSpace,
            ProcessPageTableMaterialization,
            crate::initial_process_launch::InitialProcessLaunchPlan,
            crate::initial_user_stack::InitialUserStackPlan,
        ),
        PosixError,
    >,
    request: LiveAddressSpaceActivationRequest,
    mut activation_source: LiveAddressSpaceActivationLeaseSource,
    expected: PosixError,
) -> bool {
    let result = fixture.and_then(
        |(image, install_plan, address_space, materialization, launch_plan, stack_plan)| {
            preflight_live_address_space_activation(
                image,
                install_plan,
                address_space,
                materialization,
                launch_plan,
                stack_plan,
                request,
                &mut activation_source,
            )
        },
    );
    let (errno, ok) = match result {
        Ok(_) => (expected, false),
        Err(error) => (error, error == expected),
    };
    let partial_activation = activation_source.outstanding_leases() != 0;
    crate::println!(
        "qemu-live-address-space-activation-smoke: error case={} errno=-{} partial-activation={} runnable-published=false ok={}",
        case,
        errno.name(),
        partial_activation,
        ok && !partial_activation
    );
    ok && !partial_activation
}

#[cfg(talos_boot_scenario = "qemu_live_address_space_activation_smoke")]
fn live_address_space_activation_runnable_publication_visible() -> bool {
    let Ok((image, install_plan, address_space, materialization, launch_plan, stack_plan)) =
        live_address_space_activation_valid_fixture()
    else {
        return true;
    };
    let mut activation_source = LiveAddressSpaceActivationLeaseSource::for_single_plan();
    let Ok(plan) = preflight_live_address_space_activation(
        image,
        install_plan,
        address_space,
        materialization,
        launch_plan,
        stack_plan,
        LiveAddressSpaceActivationRequest::PreflightOnly,
        &mut activation_source,
    ) else {
        return true;
    };
    plan.commit_request(LiveAddressSpaceActivationCommitTarget::Runnable)
        .is_ok()
        || plan.side_effects().scheduler_published()
        || plan.side_effects().process_table_mutated()
}

#[cfg(talos_boot_scenario = "qemu_live_address_space_activation_smoke")]
fn live_address_space_activation_valid_fixture() -> Result<
    (
        ProgramImagePlan,
        ProcessImageInstallPlan,
        ProcessAddressSpace,
        ProcessPageTableMaterialization,
        crate::initial_process_launch::InitialProcessLaunchPlan,
        crate::initial_user_stack::InitialUserStackPlan,
    ),
    PosixError,
> {
    let image = plan_phase8_init_image(phase8_readonly_initramfs_fixture())
        .map_err(|error| error.posix_error())?;
    let install_plan = plan_process_image_install(image)?;
    let mut address_source = ProcessAddressSpaceLeaseSource::for_plan(install_plan);
    let address_space = install_process_address_space(
        install_plan,
        ProcessAddressSpaceId::new(0x8800_4001).expect("address-space id"),
        Some(ProcessOwnerId::new(0x8800_4002).expect("owner id")),
        &mut address_source,
    )?;
    let mut materialization_source =
        ProcessPageTableMaterializationLeaseSource::for_address_space(address_space);
    let materialization = materialize_process_page_tables(
        image,
        install_plan,
        address_space,
        ProcessMaterializationRequest::DescriptorImageOnly,
        &mut materialization_source,
    )?;
    let launch_plan = prepare_initial_process_launch(
        image,
        install_plan,
        address_space,
        materialization,
        InitialProcessLaunchRequest::PreparePlanOnly,
    )?;
    let mut stack_source = InitialUserStackLeaseSource::for_initial_stack();
    let stack_plan = plan_initial_user_stack(
        image,
        install_plan,
        address_space,
        materialization,
        launch_plan,
        InitialUserStackRequest::PlanOnly,
        &mut stack_source,
    )?;
    Ok((
        image,
        install_plan,
        address_space,
        materialization,
        launch_plan,
        stack_plan,
    ))
}

#[cfg(talos_boot_scenario = "qemu_live_address_space_activation_smoke")]
fn live_address_space_activation_identity_mismatch_fixture() -> Result<
    (
        ProgramImagePlan,
        ProcessImageInstallPlan,
        ProcessAddressSpace,
        ProcessPageTableMaterialization,
        crate::initial_process_launch::InitialProcessLaunchPlan,
        crate::initial_user_stack::InitialUserStackPlan,
    ),
    PosixError,
> {
    let (image, install_plan, address_space, materialization, launch_plan, stack_plan) =
        live_address_space_activation_valid_fixture()?;
    Ok((
        live_address_space_activation_image_with_identity(image, "wrong-fixture"),
        install_plan,
        address_space,
        materialization,
        launch_plan,
        stack_plan,
    ))
}

#[cfg(talos_boot_scenario = "qemu_live_address_space_activation_smoke")]
fn live_address_space_activation_missing_descriptor_fixture() -> Result<
    (
        ProgramImagePlan,
        ProcessImageInstallPlan,
        ProcessAddressSpace,
        ProcessPageTableMaterialization,
        crate::initial_process_launch::InitialProcessLaunchPlan,
        crate::initial_user_stack::InitialUserStackPlan,
    ),
    PosixError,
> {
    let (image, install_plan, address_space, materialization, launch_plan, stack_plan) =
        live_address_space_activation_valid_fixture()?;
    Ok((
        image,
        install_plan,
        address_space,
        materialization.for_test_missing_descriptor(0),
        launch_plan,
        stack_plan,
    ))
}

#[cfg(talos_boot_scenario = "qemu_live_address_space_activation_smoke")]
fn live_address_space_activation_image_with_identity(
    image: ProgramImagePlan,
    identity: &'static str,
) -> ProgramImagePlan {
    let mut segments = [None; MAX_LOAD_SEGMENTS];
    let mut index = 0;
    while index < image.segment_count() {
        segments[index] = image.segment(index);
        index += 1;
    }
    ProgramImagePlan::for_test_unchecked(
        image.source_path(),
        identity,
        image.source_len(),
        image.source_digest(),
        image.entry(),
        image.segment_count(),
        segments,
        image.memory_start(),
        image.memory_end(),
        image.memory_footprint(),
    )
}

#[cfg(talos_boot_scenario = "qemu_kernel_half_reachability_smoke")]
pub fn run_kernel_half_reachability_smoke() -> bool {
    crate::println!("qemu-kernel-half-reachability-smoke: start");

    let (
        success_ok,
        root_ok,
        reachability_ok,
        permissions_ok,
        compatibility_ok,
        blocked_ok,
        effects_ok,
    ) = kernel_half_reachability_report_success();
    let teardown_ok = kernel_half_reachability_report_teardown();
    let identity_ok = kernel_half_reachability_report_error(
        "identity-mismatch",
        kernel_half_reachability_identity_mismatch_fixture(),
        KernelHalfReachabilityRequest::PreflightOnly,
        KernelHalfReachabilityLeaseSource::for_single_plan(),
        PosixError::InvalidArgument,
    );
    let missing_range_ok = kernel_half_reachability_report_error(
        "missing-kernel-range",
        kernel_half_reachability_valid_fixture(),
        KernelHalfReachabilityRequest::MissingKernelRange,
        KernelHalfReachabilityLeaseSource::for_single_plan(),
        PosixError::AccessDenied,
    );
    let diagnostic_ok = kernel_half_reachability_report_error(
        "missing-diagnostic-fault-reporting",
        kernel_half_reachability_valid_fixture(),
        KernelHalfReachabilityRequest::MissingDiagnosticFaultReporting,
        KernelHalfReachabilityLeaseSource::for_single_plan(),
        PosixError::NotImplemented,
    );
    let forbidden_ok = kernel_half_reachability_report_error(
        "forbidden-el0-access",
        kernel_half_reachability_valid_fixture(),
        KernelHalfReachabilityRequest::ForbiddenEl0Access,
        KernelHalfReachabilityLeaseSource::for_single_plan(),
        PosixError::AccessDenied,
    );
    let device_ok = kernel_half_reachability_report_error(
        "bad-device-attribute-intent",
        kernel_half_reachability_valid_fixture(),
        KernelHalfReachabilityRequest::BadDeviceAttributeIntent,
        KernelHalfReachabilityLeaseSource::for_single_plan(),
        PosixError::AccessDenied,
    );
    let live_register_ok = kernel_half_reachability_report_error(
        "live-register-request",
        kernel_half_reachability_valid_fixture(),
        KernelHalfReachabilityRequest::LiveRegisterSequence,
        KernelHalfReachabilityLeaseSource::for_single_plan(),
        PosixError::NotImplemented,
    );
    let descriptor_ok = kernel_half_reachability_report_error(
        "descriptor-image-request",
        kernel_half_reachability_valid_fixture(),
        KernelHalfReachabilityRequest::DescriptorImage,
        KernelHalfReachabilityLeaseSource::for_single_plan(),
        PosixError::NotImplemented,
    );
    let scheduler_ok = kernel_half_reachability_report_error(
        "scheduler-publication-request",
        kernel_half_reachability_valid_fixture(),
        KernelHalfReachabilityRequest::PublishSchedulerRunnable,
        KernelHalfReachabilityLeaseSource::for_single_plan(),
        PosixError::NotImplemented,
    );
    let lower_el_ok = kernel_half_reachability_report_error(
        "lower-el-launch-request",
        kernel_half_reachability_valid_fixture(),
        KernelHalfReachabilityRequest::LowerElLaunch,
        KernelHalfReachabilityLeaseSource::for_single_plan(),
        PosixError::NotImplemented,
    );
    let resource_ok = kernel_half_reachability_report_error(
        "resource-exhaustion",
        kernel_half_reachability_valid_fixture(),
        KernelHalfReachabilityRequest::PreflightOnly,
        KernelHalfReachabilityLeaseSource::with_plan_record_capacity(0),
        PosixError::NoMemory,
    );
    let no_partial_ok = kernel_half_reachability_no_partial_state_visible();

    let participants = u64::from(success_ok)
        + u64::from(root_ok)
        + u64::from(reachability_ok)
        + u64::from(permissions_ok)
        + u64::from(compatibility_ok)
        + u64::from(blocked_ok)
        + u64::from(effects_ok)
        + u64::from(teardown_ok)
        + u64::from(identity_ok)
        + u64::from(missing_range_ok)
        + u64::from(diagnostic_ok)
        + u64::from(forbidden_ok)
        + u64::from(device_ok)
        + u64::from(live_register_ok && descriptor_ok && scheduler_ok && lower_el_ok)
        + u64::from(resource_ok)
        + u64::from(no_partial_ok);
    let errors = 16 - participants;
    let classification = if participants == 16 && errors == 0 {
        "qemu-kernel-half-reachability-smoke-complete"
    } else {
        "qemu-kernel-half-reachability-smoke-failed"
    };

    crate::println!(
        "qemu-kernel-half-reachability-smoke: final participants={} expected=16 errors={} classification={}",
        participants,
        errors,
        classification
    );
    if participants == 16 && errors == 0 {
        crate::println!("qemu-kernel-half-reachability-smoke: PASS");
        true
    } else {
        crate::println!("qemu-kernel-half-reachability-smoke: FAIL");
        false
    }
}

#[cfg(talos_boot_scenario = "qemu_kernel_half_reachability_smoke")]
fn kernel_half_reachability_report_success() -> (bool, bool, bool, bool, bool, bool, bool) {
    let Ok((
        image,
        install_plan,
        address_space,
        materialization,
        launch_plan,
        stack_plan,
        activation_plan,
    )) = kernel_half_reachability_valid_fixture()
    else {
        kernel_half_reachability_report_empty_success();
        return (false, false, false, false, false, false, false);
    };
    let mut reachability_source = KernelHalfReachabilityLeaseSource::for_single_plan();
    let Ok(plan) = preflight_kernel_half_reachability(
        image,
        install_plan,
        address_space,
        materialization,
        launch_plan,
        stack_plan,
        activation_plan,
        KernelHalfReachabilityRequest::PreflightOnly,
        &mut reachability_source,
    ) else {
        kernel_half_reachability_report_empty_success();
        return (false, false, false, false, false, false, false);
    };

    crate::println!(
        "qemu-kernel-half-reachability-smoke: fixture name={} path=/bin/init source-digest={:#x} install-boundary={} address-space-boundary={} materialization-boundary={} launch-boundary={} stack-boundary={} activation-boundary={} kernel-half-boundary={} kernel-half-policy={}",
        PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY,
        plan.source_digest(),
        PROCESS_INSTALL_BOUNDARY_IDENTITY,
        PROCESS_ADDRESS_SPACE_BOUNDARY_IDENTITY,
        PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY,
        INITIAL_PROCESS_LAUNCH_BOUNDARY_IDENTITY,
        INITIAL_USER_STACK_BOUNDARY_IDENTITY,
        LIVE_ADDRESS_SPACE_ACTIVATION_BOUNDARY_IDENTITY,
        KERNEL_HALF_REACHABILITY_BOUNDARY_IDENTITY,
        KERNEL_HALF_REACHABILITY_POLICY
    );

    let success_ok = plan.published()
        && plan.boundary_identity() == KERNEL_HALF_REACHABILITY_BOUNDARY_IDENTITY
        && plan.policy_identity() == KERNEL_HALF_REACHABILITY_POLICY
        && plan.image_fixture_identity() == PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY
        && plan.install_boundary_identity() == PROCESS_INSTALL_BOUNDARY_IDENTITY
        && plan.address_space_boundary_identity() == PROCESS_ADDRESS_SPACE_BOUNDARY_IDENTITY
        && plan.materialization_boundary_identity()
            == PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY
        && plan.launch_boundary_identity() == INITIAL_PROCESS_LAUNCH_BOUNDARY_IDENTITY
        && plan.stack_boundary_identity() == INITIAL_USER_STACK_BOUNDARY_IDENTITY
        && plan.activation_boundary_identity() == LIVE_ADDRESS_SPACE_ACTIVATION_BOUNDARY_IDENTITY
        && plan.source_path() == PHASE8_INIT_PATH
        && plan.source_digest() == image.source_digest()
        && plan.address_space_id() == address_space.id().raw()
        && plan.materialization_id() == materialization.id()
        && plan.entry_pc() == image.entry()
        && plan.initial_sp() == stack_plan.layout().initial_sp();
    crate::println!(
        "qemu-kernel-half-reachability-smoke: success output=KernelHalfReachabilityPlan published={} copied-identities={} kernel-half-boundary={} kernel-half-policy={} ok={}",
        plan.published(),
        success_ok,
        plan.boundary_identity(),
        plan.policy_identity(),
        success_ok
    );

    let root = plan.root_policy();
    let root_ok = root.ttbr0_root() == TTBR0_ROOT_PROVENANCE
        && root.ttbr0_root_token() == activation_plan.root_provenance().root_token()
        && root.ttbr0_root_physical_frame()
            == activation_plan.root_provenance().root_physical_frame()
        && !root.ttbr0_written()
        && root.ttbr1_policy() == TTBR1_SHARED_KERNEL_ROOT_POLICY
        && !root.ttbr1_written()
        && root.descriptor_image() == KERNEL_HALF_DESCRIPTOR_IMAGE_BLOCKED;
    crate::println!(
        "qemu-kernel-half-reachability-smoke: root-policy ttbr0-root={} ttbr0-written={} ttbr1-policy={} ttbr1-written={} descriptor-image={} ok={}",
        root.ttbr0_root(),
        root.ttbr0_written(),
        root.ttbr1_policy(),
        root.ttbr1_written(),
        root.descriptor_image(),
        root_ok
    );

    let reachability = plan.reachability();
    let reachability_ok = reachability.kernel_text()
        && reachability.rodata()
        && reachability.data()
        && reachability.bss()
        && reachability.vectors()
        && reachability.active_stack()
        && reachability.heap()
        && reachability.page_frames()
        && reachability.uart_mmio_diagnostics()
        && reachability.scheduler_code_data()
        && reachability.panic_fault_reporting();
    crate::println!(
        "qemu-kernel-half-reachability-smoke: reachability kernel-text={} rodata={} data={} bss={} vectors={} active-stack={} heap={} page-frames={} uart-mmio-diagnostics={} scheduler-code-data={} panic-fault-reporting={} ok={}",
        reachability.kernel_text(),
        reachability.rodata(),
        reachability.data(),
        reachability.bss(),
        reachability.vectors(),
        reachability.active_stack(),
        reachability.heap(),
        reachability.page_frames(),
        reachability.uart_mmio_diagnostics(),
        reachability.scheduler_code_data(),
        reachability.panic_fault_reporting(),
        reachability_ok
    );

    let permissions = plan.permission_policy();
    let permissions_ok = permissions.text_exec_privileged_only()
        && !permissions.data_exec()
        && !permissions.device_normal_memory()
        && !permissions.el0_kernel_access();
    crate::println!(
        "qemu-kernel-half-reachability-smoke: permissions text-exec=privileged-only data-exec={} device-normal-memory={} el0-kernel-access={} ok={}",
        permissions.data_exec(),
        permissions.device_normal_memory(),
        permissions.el0_kernel_access(),
        permissions_ok
    );

    let compatibility_ok = plan.tcr_state() == SPLIT_TCR_COMPATIBILITY_RECORD_ONLY
        && plan.mair_state() == NORMAL_DEVICE_MAIR_COMPATIBILITY_RECORD_ONLY
        && plan.sctlr_state() == SCTLR_MUTATION_BLOCKED;
    crate::println!(
        "qemu-kernel-half-reachability-smoke: compatibility tcr-state={} mair-state={} sctlr-state={} ok={}",
        plan.tcr_state(),
        plan.mair_state(),
        plan.sctlr_state(),
        compatibility_ok
    );

    let blocked_ok = plan.asid_state() == ASID_ALLOCATION_BLOCKED
        && plan.tlb_state() == TLB_INVALIDATION_BLOCKED
        && plan.barrier_state() == BARRIER_SEQUENCE_PLANNED_ONLY
        && plan.live_register_sequence_state() == LIVE_REGISTER_SEQUENCE_BLOCKED;
    crate::println!(
        "qemu-kernel-half-reachability-smoke: blocked-states asid={} tlb={} barriers={} live-register-sequence={} ok={}",
        plan.asid_state(),
        plan.tlb_state(),
        plan.barrier_state(),
        plan.live_register_sequence_state(),
        blocked_ok
    );

    let effects = plan.side_effects();
    let effects_ok = !effects.ttbr_mutated()
        && !effects.tcr_mutated()
        && !effects.mair_mutated()
        && !effects.sctlr_mutated()
        && !effects.descriptor_image_installed()
        && !effects.asid_allocated()
        && !effects.tlb_mutated()
        && !effects.live_dsb_isb()
        && !effects.lower_el_eret()
        && !effects.scheduler_published()
        && !effects.process_table_mutated()
        && !effects.descriptor_table_mutated();
    crate::println!(
        "qemu-kernel-half-reachability-smoke: side-effects ttbr-mutated={} tcr-mutated={} mair-mutated={} sctlr-mutated={} descriptor-image-installed={} asid-allocated={} tlb-mutated={} live-dsb-isb={} lower-el-eret={} scheduler-published={} process-table-mutated={} descriptor-table-mutated={} ok={}",
        effects.ttbr_mutated(),
        effects.tcr_mutated(),
        effects.mair_mutated(),
        effects.sctlr_mutated(),
        effects.descriptor_image_installed(),
        effects.asid_allocated(),
        effects.tlb_mutated(),
        effects.live_dsb_isb(),
        effects.lower_el_eret(),
        effects.scheduler_published(),
        effects.process_table_mutated(),
        effects.descriptor_table_mutated(),
        effects_ok
    );

    (
        success_ok,
        root_ok,
        reachability_ok,
        permissions_ok,
        compatibility_ok,
        blocked_ok,
        effects_ok,
    )
}

#[cfg(talos_boot_scenario = "qemu_kernel_half_reachability_smoke")]
fn kernel_half_reachability_report_empty_success() {
    crate::println!(
        "qemu-kernel-half-reachability-smoke: fixture name={} path=/bin/init source-digest=0x0 install-boundary={} address-space-boundary={} materialization-boundary={} launch-boundary={} stack-boundary={} activation-boundary={} kernel-half-boundary={} kernel-half-policy={}",
        PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY,
        PROCESS_INSTALL_BOUNDARY_IDENTITY,
        PROCESS_ADDRESS_SPACE_BOUNDARY_IDENTITY,
        PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY,
        INITIAL_PROCESS_LAUNCH_BOUNDARY_IDENTITY,
        INITIAL_USER_STACK_BOUNDARY_IDENTITY,
        LIVE_ADDRESS_SPACE_ACTIVATION_BOUNDARY_IDENTITY,
        KERNEL_HALF_REACHABILITY_BOUNDARY_IDENTITY,
        KERNEL_HALF_REACHABILITY_POLICY
    );
    crate::println!(
        "qemu-kernel-half-reachability-smoke: success output=KernelHalfReachabilityPlan published=false copied-identities=false kernel-half-boundary={} kernel-half-policy={} ok=false",
        KERNEL_HALF_REACHABILITY_BOUNDARY_IDENTITY,
        KERNEL_HALF_REACHABILITY_POLICY
    );
}

#[cfg(talos_boot_scenario = "qemu_kernel_half_reachability_smoke")]
fn kernel_half_reachability_report_teardown() -> bool {
    let Ok((
        image,
        install_plan,
        address_space,
        materialization,
        launch_plan,
        stack_plan,
        activation_plan,
    )) = kernel_half_reachability_valid_fixture()
    else {
        crate::println!(
            "qemu-kernel-half-reachability-smoke: teardown plan-local-released=false input-records-owned=false descriptor-image-installed=false idempotent=false ok=false"
        );
        return false;
    };
    let mut reachability_source = KernelHalfReachabilityLeaseSource::for_single_plan();
    let Ok(mut plan) = preflight_kernel_half_reachability(
        image,
        install_plan,
        address_space,
        materialization,
        launch_plan,
        stack_plan,
        activation_plan,
        KernelHalfReachabilityRequest::PreflightOnly,
        &mut reachability_source,
    ) else {
        crate::println!(
            "qemu-kernel-half-reachability-smoke: teardown plan-local-released=false input-records-owned=false descriptor-image-installed=false idempotent=false ok=false"
        );
        return false;
    };

    let first = plan.destroy(&mut reachability_source);
    let second = plan.destroy(&mut reachability_source);
    let plan_local_released =
        first.plan_record_released() && reachability_source.outstanding_leases() == 0;
    let input_records_owned = first.input_records_owned() && second.input_records_owned();
    let descriptor_image_installed =
        first.descriptor_image_installed() || second.descriptor_image_installed();
    let idempotent = !first.already_destroyed()
        && second.already_destroyed()
        && !second.plan_record_released()
        && !plan.published()
        && plan.destroyed();
    let ok =
        plan_local_released && input_records_owned && !descriptor_image_installed && idempotent;
    crate::println!(
        "qemu-kernel-half-reachability-smoke: teardown plan-local-released={} input-records-owned={} descriptor-image-installed={} idempotent={} ok={}",
        plan_local_released,
        input_records_owned,
        descriptor_image_installed,
        idempotent,
        ok
    );
    ok
}

#[cfg(talos_boot_scenario = "qemu_kernel_half_reachability_smoke")]
fn kernel_half_reachability_report_error(
    case: &str,
    fixture: Result<
        (
            ProgramImagePlan,
            ProcessImageInstallPlan,
            ProcessAddressSpace,
            ProcessPageTableMaterialization,
            InitialProcessLaunchPlan,
            InitialUserStackPlan,
            LiveAddressSpaceActivationPlan,
        ),
        PosixError,
    >,
    request: KernelHalfReachabilityRequest,
    mut reachability_source: KernelHalfReachabilityLeaseSource,
    expected: PosixError,
) -> bool {
    let result = fixture.and_then(
        |(
            image,
            install_plan,
            address_space,
            materialization,
            launch_plan,
            stack_plan,
            activation_plan,
        )| {
            preflight_kernel_half_reachability(
                image,
                install_plan,
                address_space,
                materialization,
                launch_plan,
                stack_plan,
                activation_plan,
                request,
                &mut reachability_source,
            )
        },
    );
    let (errno, ok) = match result {
        Ok(_) => (expected, false),
        Err(error) => (error, error == expected),
    };
    let partial_plan = reachability_source.outstanding_leases() != 0;
    crate::println!(
        "qemu-kernel-half-reachability-smoke: error case={} errno=-{} partial-plan={} ok={}",
        case,
        errno.name(),
        partial_plan,
        ok && !partial_plan
    );
    ok && !partial_plan
}

#[cfg(talos_boot_scenario = "qemu_kernel_half_reachability_smoke")]
fn kernel_half_reachability_no_partial_state_visible() -> bool {
    let Ok((
        image,
        install_plan,
        address_space,
        materialization,
        launch_plan,
        stack_plan,
        activation_plan,
    )) = kernel_half_reachability_valid_fixture()
    else {
        return false;
    };
    let mut reachability_source = KernelHalfReachabilityLeaseSource::with_plan_record_capacity(0);
    let result = preflight_kernel_half_reachability(
        image,
        install_plan,
        address_space,
        materialization,
        launch_plan,
        stack_plan,
        activation_plan,
        KernelHalfReachabilityRequest::PreflightOnly,
        &mut reachability_source,
    );
    result.is_err()
        && reachability_source.outstanding_leases() == 0
        && materialization.activation_blocked()
        && activation_plan.published()
        && !activation_plan.destroyed()
        && !activation_plan.side_effects().ttbr_mutated()
        && !activation_plan.side_effects().scheduler_published()
}

#[cfg(talos_boot_scenario = "qemu_kernel_half_reachability_smoke")]
fn kernel_half_reachability_valid_fixture() -> Result<
    (
        ProgramImagePlan,
        ProcessImageInstallPlan,
        ProcessAddressSpace,
        ProcessPageTableMaterialization,
        InitialProcessLaunchPlan,
        InitialUserStackPlan,
        LiveAddressSpaceActivationPlan,
    ),
    PosixError,
> {
    let image = plan_phase8_init_image(phase8_readonly_initramfs_fixture())
        .map_err(|error| error.posix_error())?;
    let install_plan = plan_process_image_install(image)?;
    let mut address_source = ProcessAddressSpaceLeaseSource::for_plan(install_plan);
    let address_space = install_process_address_space(
        install_plan,
        ProcessAddressSpaceId::new(0x8800_6001).expect("address-space id"),
        Some(ProcessOwnerId::new(0x8800_6002).expect("owner id")),
        &mut address_source,
    )?;
    let mut materialization_source =
        ProcessPageTableMaterializationLeaseSource::for_address_space(address_space);
    let materialization = materialize_process_page_tables(
        image,
        install_plan,
        address_space,
        ProcessMaterializationRequest::DescriptorImageOnly,
        &mut materialization_source,
    )?;
    let launch_plan = prepare_initial_process_launch(
        image,
        install_plan,
        address_space,
        materialization,
        InitialProcessLaunchRequest::PreparePlanOnly,
    )?;
    let mut stack_source = InitialUserStackLeaseSource::for_initial_stack();
    let stack_plan = plan_initial_user_stack(
        image,
        install_plan,
        address_space,
        materialization,
        launch_plan,
        InitialUserStackRequest::PlanOnly,
        &mut stack_source,
    )?;
    let mut activation_source = LiveAddressSpaceActivationLeaseSource::for_single_plan();
    let activation_plan = preflight_live_address_space_activation(
        image,
        install_plan,
        address_space,
        materialization,
        launch_plan,
        stack_plan,
        LiveAddressSpaceActivationRequest::PreflightOnly,
        &mut activation_source,
    )?;

    Ok((
        image,
        install_plan,
        address_space,
        materialization,
        launch_plan,
        stack_plan,
        activation_plan,
    ))
}

#[cfg(talos_boot_scenario = "qemu_kernel_half_reachability_smoke")]
fn kernel_half_reachability_identity_mismatch_fixture() -> Result<
    (
        ProgramImagePlan,
        ProcessImageInstallPlan,
        ProcessAddressSpace,
        ProcessPageTableMaterialization,
        InitialProcessLaunchPlan,
        InitialUserStackPlan,
        LiveAddressSpaceActivationPlan,
    ),
    PosixError,
> {
    let (
        image,
        install_plan,
        address_space,
        materialization,
        launch_plan,
        stack_plan,
        activation_plan,
    ) = kernel_half_reachability_valid_fixture()?;
    Ok((
        kernel_half_reachability_image_with_identity(image, "wrong-fixture"),
        install_plan,
        address_space,
        materialization,
        launch_plan,
        stack_plan,
        activation_plan,
    ))
}

#[cfg(talos_boot_scenario = "qemu_kernel_half_reachability_smoke")]
fn kernel_half_reachability_image_with_identity(
    image: ProgramImagePlan,
    identity: &'static str,
) -> ProgramImagePlan {
    let mut segments = [None; MAX_LOAD_SEGMENTS];
    let mut index = 0;
    while index < image.segment_count() {
        segments[index] = image.segment(index);
        index += 1;
    }
    ProgramImagePlan::for_test_unchecked(
        image.source_path(),
        identity,
        image.source_len(),
        image.source_digest(),
        image.entry(),
        image.segment_count(),
        segments,
        image.memory_start(),
        image.memory_end(),
        image.memory_footprint(),
    )
}

#[cfg(talos_boot_scenario = "qemu_kernel_half_descriptor_image_smoke")]
pub fn run_kernel_half_descriptor_image_smoke() -> bool {
    crate::println!("qemu-kernel-half-descriptor-image-smoke: start");

    let (
        success_ok,
        root_ok,
        coverage_ok,
        permissions_ok,
        attributes_ok,
        ownership_ok,
        compatibility_ok,
        blocked_ok,
        effects_ok,
    ) = kernel_half_descriptor_image_report_success();
    let teardown_ok = kernel_half_descriptor_image_report_teardown();
    let bad_reachability_ok = kernel_half_descriptor_image_report_error(
        "bad-reachability-plan",
        KernelHalfDescriptorImageRequest::BadReachabilityPlan,
        KernelHalfDescriptorImageLeaseSource::for_descriptor_image(),
        PosixError::InvalidArgument,
    );
    let lineage_ok = kernel_half_descriptor_image_report_error(
        "lineage-mismatch",
        KernelHalfDescriptorImageRequest::LineageMismatch,
        KernelHalfDescriptorImageLeaseSource::for_descriptor_image(),
        PosixError::InvalidArgument,
    );
    let missing_coverage_ok = kernel_half_descriptor_image_report_error(
        "missing-kernel-coverage",
        KernelHalfDescriptorImageRequest::MissingKernelCoverage,
        KernelHalfDescriptorImageLeaseSource::for_descriptor_image(),
        PosixError::InvalidArgument,
    );
    let forbidden_ok = kernel_half_descriptor_image_report_error(
        "forbidden-el0-access",
        KernelHalfDescriptorImageRequest::ForbiddenEl0Access,
        KernelHalfDescriptorImageLeaseSource::for_descriptor_image(),
        PosixError::AccessDenied,
    );
    let writable_text_ok = kernel_half_descriptor_image_report_error(
        "writable-text",
        KernelHalfDescriptorImageRequest::WritableText,
        KernelHalfDescriptorImageLeaseSource::for_descriptor_image(),
        PosixError::AccessDenied,
    );
    let executable_data_ok = kernel_half_descriptor_image_report_error(
        "executable-data",
        KernelHalfDescriptorImageRequest::ExecutableData,
        KernelHalfDescriptorImageLeaseSource::for_descriptor_image(),
        PosixError::AccessDenied,
    );
    let device_ok = kernel_half_descriptor_image_report_error(
        "bad-device-attribute-intent",
        KernelHalfDescriptorImageRequest::BadDeviceAttributeIntent,
        KernelHalfDescriptorImageLeaseSource::for_descriptor_image(),
        PosixError::AccessDenied,
    );
    let overlapping_ok = kernel_half_descriptor_image_report_error(
        "overlapping-range",
        KernelHalfDescriptorImageRequest::OverlappingRange,
        KernelHalfDescriptorImageLeaseSource::for_descriptor_image(),
        PosixError::InvalidArgument,
    );
    let resource_ok = kernel_half_descriptor_image_report_error(
        "resource-exhaustion",
        KernelHalfDescriptorImageRequest::ConstructOnly,
        KernelHalfDescriptorImageLeaseSource::with_limits(1, 1, 1),
        PosixError::NoMemory,
    );
    let unsupported_ok = kernel_half_descriptor_image_report_error(
        "unsupported-topology",
        KernelHalfDescriptorImageRequest::UnsupportedTopology,
        KernelHalfDescriptorImageLeaseSource::for_descriptor_image(),
        PosixError::NotSupported,
    );
    let live_activation_ok = kernel_half_descriptor_image_report_error(
        "live-activation-request",
        KernelHalfDescriptorImageRequest::LiveActivationRequest,
        KernelHalfDescriptorImageLeaseSource::for_descriptor_image(),
        PosixError::NotImplemented,
    );
    let no_partial_ok = kernel_half_descriptor_image_no_partial_state_visible();

    let invalid_request_ok = lineage_ok && missing_coverage_ok && overlapping_ok;
    let access_request_ok = forbidden_ok && writable_text_ok && executable_data_ok && device_ok;
    let participants = u64::from(success_ok)
        + u64::from(root_ok)
        + u64::from(coverage_ok)
        + u64::from(permissions_ok)
        + u64::from(attributes_ok)
        + u64::from(ownership_ok)
        + u64::from(compatibility_ok)
        + u64::from(blocked_ok)
        + u64::from(effects_ok)
        + u64::from(teardown_ok)
        + u64::from(bad_reachability_ok)
        + u64::from(invalid_request_ok)
        + u64::from(access_request_ok)
        + u64::from(resource_ok)
        + u64::from(unsupported_ok)
        + u64::from(live_activation_ok)
        + u64::from(no_partial_ok);
    let errors = 17 - participants;
    let classification = if participants == 17 && errors == 0 {
        "qemu-kernel-half-descriptor-image-smoke-complete"
    } else {
        "qemu-kernel-half-descriptor-image-smoke-failed"
    };

    crate::println!(
        "qemu-kernel-half-descriptor-image-smoke: final participants={} expected=17 errors={} classification={}",
        participants,
        errors,
        classification
    );
    if participants == 17 && errors == 0 {
        crate::println!("qemu-kernel-half-descriptor-image-smoke: PASS");
        true
    } else {
        crate::println!("qemu-kernel-half-descriptor-image-smoke: FAIL");
        false
    }
}

#[cfg(talos_boot_scenario = "qemu_kernel_half_descriptor_image_smoke")]
fn kernel_half_descriptor_image_report_success()
-> (bool, bool, bool, bool, bool, bool, bool, bool, bool) {
    let Ok((
        image,
        _install_plan,
        address_space,
        materialization,
        _activation_plan,
        reachability_plan,
    )) = kernel_half_descriptor_image_valid_fixture()
    else {
        kernel_half_descriptor_image_report_empty_success();
        return (
            false, false, false, false, false, false, false, false, false,
        );
    };
    let mut lease_source = KernelHalfDescriptorImageLeaseSource::for_descriptor_image();
    let Ok(descriptor_image) = construct_kernel_half_descriptor_image(
        reachability_plan,
        materialization,
        KernelHalfDescriptorImageRequest::ConstructOnly,
        &mut lease_source,
    ) else {
        kernel_half_descriptor_image_report_empty_success();
        return (
            false, false, false, false, false, false, false, false, false,
        );
    };

    crate::println!(
        "qemu-kernel-half-descriptor-image-smoke: fixture name={} path=/bin/init source-digest={:#x} install-boundary={} address-space-boundary={} materialization-boundary={} launch-boundary={} stack-boundary={} activation-boundary={} reachability-boundary={} descriptor-image-boundary={} descriptor-image-policy={}",
        PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY,
        descriptor_image.source_digest(),
        PROCESS_INSTALL_BOUNDARY_IDENTITY,
        PROCESS_ADDRESS_SPACE_BOUNDARY_IDENTITY,
        PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY,
        INITIAL_PROCESS_LAUNCH_BOUNDARY_IDENTITY,
        INITIAL_USER_STACK_BOUNDARY_IDENTITY,
        LIVE_ADDRESS_SPACE_ACTIVATION_BOUNDARY_IDENTITY,
        KERNEL_HALF_REACHABILITY_BOUNDARY_IDENTITY,
        KERNEL_HALF_DESCRIPTOR_IMAGE_BOUNDARY_IDENTITY,
        KERNEL_HALF_DESCRIPTOR_IMAGE_POLICY
    );

    let success_ok = descriptor_image.published()
        && descriptor_image.boundary_identity() == KERNEL_HALF_DESCRIPTOR_IMAGE_BOUNDARY_IDENTITY
        && descriptor_image.policy_identity() == KERNEL_HALF_DESCRIPTOR_IMAGE_POLICY
        && descriptor_image.reachability_boundary_identity()
            == KERNEL_HALF_REACHABILITY_BOUNDARY_IDENTITY
        && descriptor_image.reachability_policy_identity() == KERNEL_HALF_REACHABILITY_POLICY
        && descriptor_image.image_fixture_identity() == PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY
        && descriptor_image.materialization_boundary_identity()
            == PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY
        && descriptor_image.source_path() == PHASE8_INIT_PATH
        && descriptor_image.source_digest() == image.source_digest()
        && descriptor_image.address_space_id() == address_space.id().raw()
        && descriptor_image.materialization_id() == materialization.id();
    crate::println!(
        "qemu-kernel-half-descriptor-image-smoke: success output=KernelHalfDescriptorImage published={} installed={} copied-identities={} descriptor-image-boundary={} descriptor-image-policy={} ok={}",
        descriptor_image.published(),
        descriptor_image.side_effects().descriptor_image_installed(),
        success_ok,
        descriptor_image.boundary_identity(),
        descriptor_image.policy_identity(),
        success_ok
    );

    let root_ok = descriptor_image.ttbr0_root() == TTBR0_ROOT_MATERIALIZED_PROVENANCE_ONLY
        && descriptor_image.ttbr0_root_token() == materialization.root().token().raw()
        && descriptor_image.ttbr0_root_physical_frame() == materialization.root().physical_frame()
        && !descriptor_image.ttbr0_written()
        && descriptor_image.ttbr1_root() == TTBR1_OWNED_KERNEL_ROOT_IMAGE
        && !descriptor_image.ttbr1_written()
        && !descriptor_image.side_effects().descriptor_image_installed();
    crate::println!(
        "qemu-kernel-half-descriptor-image-smoke: root-policy ttbr0-root={} ttbr0-written={} ttbr1-root={} ttbr1-written={} descriptor-image-installed={} ok={}",
        descriptor_image.ttbr0_root(),
        descriptor_image.ttbr0_written(),
        descriptor_image.ttbr1_root(),
        descriptor_image.ttbr1_written(),
        descriptor_image.side_effects().descriptor_image_installed(),
        root_ok
    );

    let coverage = descriptor_image.coverage();
    let coverage_ok = coverage.kernel_text()
        && coverage.rodata()
        && coverage.data()
        && coverage.bss()
        && coverage.vectors()
        && coverage.active_stack()
        && coverage.heap()
        && coverage.page_frames()
        && coverage.uart_mmio_diagnostics()
        && coverage.scheduler_code_data()
        && coverage.runtime_console()
        && coverage.panic_fault_reporting();
    crate::println!(
        "qemu-kernel-half-descriptor-image-smoke: coverage kernel-text={} rodata={} data={} bss={} vectors={} active-stack={} heap={} page-frames={} uart-mmio-diagnostics={} scheduler-code-data={} runtime-console={} panic-fault-reporting={} ok={}",
        coverage.kernel_text(),
        coverage.rodata(),
        coverage.data(),
        coverage.bss(),
        coverage.vectors(),
        coverage.active_stack(),
        coverage.heap(),
        coverage.page_frames(),
        coverage.uart_mmio_diagnostics(),
        coverage.scheduler_code_data(),
        coverage.runtime_console(),
        coverage.panic_fault_reporting(),
        coverage_ok
    );

    let permissions = descriptor_image.permissions();
    let permissions_ok = permissions.text_exec_privileged_only()
        && !permissions.rodata_write()
        && !permissions.data_exec()
        && !permissions.device_normal_memory()
        && !permissions.el0_kernel_access()
        && !permissions.wx_normal_memory();
    crate::println!(
        "qemu-kernel-half-descriptor-image-smoke: permissions text-exec=privileged-only rodata-write={} data-exec={} device-normal-memory={} el0-kernel-access={} wx-normal-memory={} ok={}",
        permissions.rodata_write(),
        permissions.data_exec(),
        permissions.device_normal_memory(),
        permissions.el0_kernel_access(),
        permissions.wx_normal_memory(),
        permissions_ok
    );

    let mut normal_inner_shareable = false;
    let mut device_ngnre = false;
    let mut af = true;
    let mut user_denied = true;
    let mut descriptor_index = 0;
    while descriptor_index < descriptor_image.descriptor_record_count() {
        let Some(record) = descriptor_image.descriptor_record(descriptor_index) else {
            af = false;
            user_denied = false;
            break;
        };
        normal_inner_shareable |= record.normal_memory() && record.inner_shareable();
        device_ngnre |= record.device_memory() && !record.normal_memory();
        af &= record.access_flag();
        user_denied &= !record.user_access() && record.privileged_only();
        descriptor_index += 1;
    }
    let attributes_ok = normal_inner_shareable
        && device_ngnre
        && af
        && user_denied
        && descriptor_image.descriptor_record_count() == 12;
    crate::println!(
        "qemu-kernel-half-descriptor-image-smoke: attributes normal-memory={} device-memory={} af={} user-access={} exact-coverage={} ok={}",
        if normal_inner_shareable {
            "inner-shareable"
        } else {
            "missing"
        },
        if device_ngnre {
            "device-nGnRE"
        } else {
            "missing"
        },
        af,
        if user_denied { "denied" } else { "allowed" },
        descriptor_image.descriptor_record_count() == 12,
        attributes_ok
    );

    let ownership_ok = descriptor_image.root_lease().owner() == "model-owned"
        && descriptor_image.table_lease_count() == 3
        && !descriptor_image.root_lease().released()
        && lease_source.outstanding_leases() == 16
        && descriptor_image.descriptor_image_state() == KERNEL_HALF_DESCRIPTOR_IMAGE_READY;
    crate::println!(
        "qemu-kernel-half-descriptor-image-smoke: ownership root-lease={} table-leases={} live-table-borrowed=false input-records-owned=true rollback-ready=true ok={}",
        descriptor_image.root_lease().owner(),
        if descriptor_image.table_lease_count() == 3 {
            "model-owned"
        } else {
            "incomplete"
        },
        ownership_ok
    );

    let compatibility_ok = descriptor_image.tcr_state() == reachability_plan.tcr_state()
        && descriptor_image.mair_state() == reachability_plan.mair_state()
        && descriptor_image.sctlr_state() == SCTLR_MUTATION_BLOCKED;
    crate::println!(
        "qemu-kernel-half-descriptor-image-smoke: compatibility tcr-state={} mair-state={} sctlr-state={} ok={}",
        descriptor_image.tcr_state(),
        descriptor_image.mair_state(),
        descriptor_image.sctlr_state(),
        compatibility_ok
    );

    let blocked_ok = descriptor_image.asid_state() == ASID_ALLOCATION_BLOCKED
        && descriptor_image.tlb_state() == TLB_INVALIDATION_BLOCKED
        && descriptor_image.barrier_state() == BARRIER_SEQUENCE_PLANNED_ONLY
        && descriptor_image.live_register_sequence_state() == LIVE_REGISTER_SEQUENCE_BLOCKED
        && !descriptor_image.lower_el_eret_state().is_empty()
        && !descriptor_image.scheduler_publication_state().is_empty();
    crate::println!(
        "qemu-kernel-half-descriptor-image-smoke: blocked-states asid={} tlb={} barriers={} live-register-sequence={} lower-el-eret={} scheduler-publication={} ok={}",
        descriptor_image.asid_state(),
        descriptor_image.tlb_state(),
        descriptor_image.barrier_state(),
        descriptor_image.live_register_sequence_state(),
        false,
        false,
        blocked_ok
    );

    let effects = descriptor_image.side_effects();
    let effects_ok = !effects.ttbr_mutated()
        && !effects.tcr_mutated()
        && !effects.mair_mutated()
        && !effects.sctlr_mutated()
        && !effects.descriptor_image_installed()
        && !effects.asid_allocated()
        && !effects.tlb_mutated()
        && !effects.live_dsb_isb()
        && !effects.lower_el_eret()
        && !effects.scheduler_published()
        && !effects.process_table_mutated()
        && !effects.descriptor_table_mutated();
    crate::println!(
        "qemu-kernel-half-descriptor-image-smoke: side-effects ttbr-mutated={} tcr-mutated={} mair-mutated={} sctlr-mutated={} descriptor-image-installed={} asid-allocated={} tlb-mutated={} live-dsb-isb={} lower-el-eret={} scheduler-published={} process-table-mutated={} descriptor-table-mutated={} ok={}",
        effects.ttbr_mutated(),
        effects.tcr_mutated(),
        effects.mair_mutated(),
        effects.sctlr_mutated(),
        effects.descriptor_image_installed(),
        effects.asid_allocated(),
        effects.tlb_mutated(),
        effects.live_dsb_isb(),
        effects.lower_el_eret(),
        effects.scheduler_published(),
        effects.process_table_mutated(),
        effects.descriptor_table_mutated(),
        effects_ok
    );

    (
        success_ok,
        root_ok,
        coverage_ok,
        permissions_ok,
        attributes_ok,
        ownership_ok,
        compatibility_ok,
        blocked_ok,
        effects_ok,
    )
}

#[cfg(talos_boot_scenario = "qemu_kernel_half_descriptor_image_smoke")]
fn kernel_half_descriptor_image_report_empty_success() {
    crate::println!(
        "qemu-kernel-half-descriptor-image-smoke: fixture name={} path=/bin/init source-digest=0x0 install-boundary={} address-space-boundary={} materialization-boundary={} launch-boundary={} stack-boundary={} activation-boundary={} reachability-boundary={} descriptor-image-boundary={} descriptor-image-policy={}",
        PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY,
        PROCESS_INSTALL_BOUNDARY_IDENTITY,
        PROCESS_ADDRESS_SPACE_BOUNDARY_IDENTITY,
        PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY,
        INITIAL_PROCESS_LAUNCH_BOUNDARY_IDENTITY,
        INITIAL_USER_STACK_BOUNDARY_IDENTITY,
        LIVE_ADDRESS_SPACE_ACTIVATION_BOUNDARY_IDENTITY,
        KERNEL_HALF_REACHABILITY_BOUNDARY_IDENTITY,
        KERNEL_HALF_DESCRIPTOR_IMAGE_BOUNDARY_IDENTITY,
        KERNEL_HALF_DESCRIPTOR_IMAGE_POLICY
    );
    crate::println!(
        "qemu-kernel-half-descriptor-image-smoke: success output=KernelHalfDescriptorImage published=false installed=false copied-identities=false descriptor-image-boundary={} descriptor-image-policy={} ok=false",
        KERNEL_HALF_DESCRIPTOR_IMAGE_BOUNDARY_IDENTITY,
        KERNEL_HALF_DESCRIPTOR_IMAGE_POLICY
    );
}

#[cfg(talos_boot_scenario = "qemu_kernel_half_descriptor_image_smoke")]
fn kernel_half_descriptor_image_report_teardown() -> bool {
    let Ok((
        _image,
        _install_plan,
        _address_space,
        materialization,
        _activation_plan,
        reachability_plan,
    )) = kernel_half_descriptor_image_valid_fixture()
    else {
        crate::println!(
            "qemu-kernel-half-descriptor-image-smoke: teardown phase=first descriptors-cleared=false root-released=false tables-released=false published=false input-records-owned=false already-destroyed=false ok=false"
        );
        crate::println!(
            "qemu-kernel-half-descriptor-image-smoke: teardown phase=second descriptors-cleared=false root-released=false tables-released=false published=false already-destroyed=false ok=false"
        );
        return false;
    };
    let mut lease_source = KernelHalfDescriptorImageLeaseSource::for_descriptor_image();
    let Ok(mut descriptor_image) = construct_kernel_half_descriptor_image(
        reachability_plan,
        materialization,
        KernelHalfDescriptorImageRequest::ConstructOnly,
        &mut lease_source,
    ) else {
        crate::println!(
            "qemu-kernel-half-descriptor-image-smoke: teardown phase=first descriptors-cleared=false root-released=false tables-released=false published=false input-records-owned=false already-destroyed=false ok=false"
        );
        crate::println!(
            "qemu-kernel-half-descriptor-image-smoke: teardown phase=second descriptors-cleared=false root-released=false tables-released=false published=false already-destroyed=false ok=false"
        );
        return false;
    };

    let first = descriptor_image.destroy(&mut lease_source);
    let first_ok = first.descriptors_cleared() == 12
        && first.root_released()
        && first.tables_released() == 3
        && first.input_records_owned()
        && !first.already_destroyed()
        && !descriptor_image.published()
        && lease_source.outstanding_leases() == 0;
    crate::println!(
        "qemu-kernel-half-descriptor-image-smoke: teardown phase=first descriptors-cleared={} root-released={} tables-released={} published={} input-records-owned={} already-destroyed={} ok={}",
        first.descriptors_cleared() == 12,
        first.root_released(),
        first.tables_released() == 3,
        descriptor_image.published(),
        first.input_records_owned(),
        first.already_destroyed(),
        first_ok
    );

    let second = descriptor_image.destroy(&mut lease_source);
    let second_ok = second.descriptors_cleared() == 0
        && !second.root_released()
        && second.tables_released() == 0
        && !descriptor_image.published()
        && second.already_destroyed()
        && lease_source.outstanding_leases() == 0;
    crate::println!(
        "qemu-kernel-half-descriptor-image-smoke: teardown phase=second descriptors-cleared={} root-released={} tables-released={} published={} already-destroyed={} ok={}",
        second.descriptors_cleared() != 0,
        second.root_released(),
        second.tables_released() != 0,
        descriptor_image.published(),
        second.already_destroyed(),
        second_ok
    );

    first_ok && second_ok
}

#[cfg(talos_boot_scenario = "qemu_kernel_half_descriptor_image_smoke")]
fn kernel_half_descriptor_image_report_error(
    case: &str,
    request: KernelHalfDescriptorImageRequest,
    mut lease_source: KernelHalfDescriptorImageLeaseSource,
    expected: PosixError,
) -> bool {
    let result = kernel_half_descriptor_image_valid_fixture().and_then(
        |(_image, _install_plan, _address_space, materialization, _activation_plan, plan)| {
            construct_kernel_half_descriptor_image(
                plan,
                materialization,
                request,
                &mut lease_source,
            )
        },
    );
    let (errno, ok) = match result {
        Ok(_) => (expected, false),
        Err(error) => (error, error == expected),
    };
    let leaked_leases = lease_source.outstanding_leases() != 0;
    crate::println!(
        "qemu-kernel-half-descriptor-image-smoke: error case={} errno=-{} partial-image=false leaked-leases={} ok={}",
        case,
        errno.name(),
        leaked_leases,
        ok && !leaked_leases
    );
    ok && !leaked_leases
}

#[cfg(talos_boot_scenario = "qemu_kernel_half_descriptor_image_smoke")]
fn kernel_half_descriptor_image_no_partial_state_visible() -> bool {
    let mut lease_source = KernelHalfDescriptorImageLeaseSource::with_limits(1, 1, 1);
    let result = kernel_half_descriptor_image_valid_fixture().and_then(
        |(_image, _install_plan, _address_space, materialization, activation_plan, plan)| {
            let result = construct_kernel_half_descriptor_image(
                plan,
                materialization,
                KernelHalfDescriptorImageRequest::ConstructOnly,
                &mut lease_source,
            );
            if result.is_err()
                && lease_source.outstanding_leases() == 0
                && materialization.activation_blocked()
                && activation_plan.published()
                && !activation_plan.destroyed()
                && !activation_plan.side_effects().ttbr_mutated()
                && !activation_plan.side_effects().scheduler_published()
            {
                Err(PosixError::NoMemory)
            } else {
                result.map(|_| ())
            }
        },
    );
    result == Err(PosixError::NoMemory) && lease_source.outstanding_leases() == 0
}

#[cfg(any(
    talos_boot_scenario = "qemu_kernel_half_descriptor_image_smoke",
    talos_boot_scenario = "qemu_live_descriptor_image_installation_smoke"
))]
fn kernel_half_descriptor_image_valid_fixture() -> Result<
    (
        ProgramImagePlan,
        ProcessImageInstallPlan,
        ProcessAddressSpace,
        ProcessPageTableMaterialization,
        LiveAddressSpaceActivationPlan,
        KernelHalfReachabilityPlan,
    ),
    PosixError,
> {
    let image = plan_phase8_init_image(phase8_readonly_initramfs_fixture())
        .map_err(|error| error.posix_error())?;
    let install_plan = plan_process_image_install(image)?;
    let mut address_source = ProcessAddressSpaceLeaseSource::for_plan(install_plan);
    let address_space = install_process_address_space(
        install_plan,
        ProcessAddressSpaceId::new(0x8800_7001).expect("address-space id"),
        Some(ProcessOwnerId::new(0x8800_7002).expect("owner id")),
        &mut address_source,
    )?;
    let mut materialization_source =
        ProcessPageTableMaterializationLeaseSource::for_address_space(address_space);
    let materialization = materialize_process_page_tables(
        image,
        install_plan,
        address_space,
        ProcessMaterializationRequest::DescriptorImageOnly,
        &mut materialization_source,
    )?;
    let launch_plan = prepare_initial_process_launch(
        image,
        install_plan,
        address_space,
        materialization,
        InitialProcessLaunchRequest::PreparePlanOnly,
    )?;
    let mut stack_source = InitialUserStackLeaseSource::for_initial_stack();
    let stack_plan = plan_initial_user_stack(
        image,
        install_plan,
        address_space,
        materialization,
        launch_plan,
        InitialUserStackRequest::PlanOnly,
        &mut stack_source,
    )?;
    let mut activation_source = LiveAddressSpaceActivationLeaseSource::for_single_plan();
    let activation_plan = preflight_live_address_space_activation(
        image,
        install_plan,
        address_space,
        materialization,
        launch_plan,
        stack_plan,
        LiveAddressSpaceActivationRequest::PreflightOnly,
        &mut activation_source,
    )?;
    let mut reachability_source = KernelHalfReachabilityLeaseSource::for_single_plan();
    let reachability_plan = preflight_kernel_half_reachability(
        image,
        install_plan,
        address_space,
        materialization,
        launch_plan,
        stack_plan,
        activation_plan,
        KernelHalfReachabilityRequest::PreflightOnly,
        &mut reachability_source,
    )?;

    Ok((
        image,
        install_plan,
        address_space,
        materialization,
        activation_plan,
        reachability_plan,
    ))
}

#[cfg(talos_boot_scenario = "qemu_live_descriptor_image_installation_smoke")]
pub fn run_live_descriptor_image_installation_smoke() -> bool {
    crate::println!("qemu-live-descriptor-image-installation-smoke: start");

    let (
        success_ok,
        input_ok,
        ttbr_ok,
        coverage_ok,
        permissions_ok,
        installation_ok,
        compatibility_ok,
        blocked_ok,
        effects_ok,
    ) = live_descriptor_image_installation_report_success();
    let teardown_ok = live_descriptor_image_installation_report_teardown();
    let missing_ok = live_descriptor_image_installation_report_error(
        "missing-input",
        None,
        LiveDescriptorImageInstallationRequest::InstallModelBinding,
        LiveDescriptorImageInstallationLeaseSource::for_single_installation(),
        PosixError::InvalidArgument,
        false,
    );
    let destroyed_ok = live_descriptor_image_installation_report_destroyed_input_error();
    let identity_ok = live_descriptor_image_installation_report_error(
        "identity-mismatch",
        Some(LiveDescriptorImageInstallationRequest::IdentityMismatch),
        LiveDescriptorImageInstallationRequest::IdentityMismatch,
        LiveDescriptorImageInstallationLeaseSource::for_single_installation(),
        PosixError::InvalidArgument,
        false,
    );
    let lineage_ok = live_descriptor_image_installation_report_error(
        "lineage-mismatch",
        Some(LiveDescriptorImageInstallationRequest::LineageMismatch),
        LiveDescriptorImageInstallationRequest::LineageMismatch,
        LiveDescriptorImageInstallationLeaseSource::for_single_installation(),
        PosixError::NotExecutable,
        false,
    );
    let already_installed_ok = live_descriptor_image_installation_report_error(
        "already-installed-input",
        Some(LiveDescriptorImageInstallationRequest::AlreadyInstalledInput),
        LiveDescriptorImageInstallationRequest::AlreadyInstalledInput,
        LiveDescriptorImageInstallationLeaseSource::for_single_installation(),
        PosixError::Busy,
        true,
    );
    let forbidden_ok = live_descriptor_image_installation_report_error(
        "forbidden-el0-access",
        Some(LiveDescriptorImageInstallationRequest::ForbiddenEl0Access),
        LiveDescriptorImageInstallationRequest::ForbiddenEl0Access,
        LiveDescriptorImageInstallationLeaseSource::for_single_installation(),
        PosixError::AccessDenied,
        false,
    );
    let diagnostic_ok = live_descriptor_image_installation_report_error(
        "diagnostic-reachability-loss",
        Some(LiveDescriptorImageInstallationRequest::DiagnosticReachabilityLoss),
        LiveDescriptorImageInstallationRequest::DiagnosticReachabilityLoss,
        LiveDescriptorImageInstallationLeaseSource::for_single_installation(),
        PosixError::AccessDenied,
        false,
    );
    let resource_ok = live_descriptor_image_installation_report_error(
        "resource-exhaustion",
        Some(LiveDescriptorImageInstallationRequest::InstallModelBinding),
        LiveDescriptorImageInstallationRequest::InstallModelBinding,
        LiveDescriptorImageInstallationLeaseSource::with_installation_record_capacity(0),
        PosixError::NoMemory,
        false,
    );
    let live_register_ok = live_descriptor_image_installation_report_error(
        "live-register-request",
        Some(LiveDescriptorImageInstallationRequest::LiveRegisterRequest),
        LiveDescriptorImageInstallationRequest::LiveRegisterRequest,
        LiveDescriptorImageInstallationLeaseSource::for_single_installation(),
        PosixError::NotImplemented,
        false,
    );

    let invalid_input_ok = missing_ok && destroyed_ok && identity_ok;
    let access_ok = forbidden_ok && diagnostic_ok;
    let blocked_request_ok = resource_ok && live_register_ok;
    let participants = u64::from(success_ok)
        + u64::from(input_ok)
        + u64::from(ttbr_ok)
        + u64::from(coverage_ok)
        + u64::from(permissions_ok)
        + u64::from(installation_ok)
        + u64::from(compatibility_ok)
        + u64::from(blocked_ok)
        + u64::from(effects_ok)
        + u64::from(teardown_ok)
        + u64::from(invalid_input_ok)
        + u64::from(lineage_ok)
        + u64::from(already_installed_ok)
        + u64::from(access_ok)
        + u64::from(blocked_request_ok);
    let errors = 15 - participants;
    let classification = if participants == 15 && errors == 0 {
        "qemu-live-descriptor-image-installation-smoke-complete"
    } else {
        "qemu-live-descriptor-image-installation-smoke-failed"
    };

    crate::println!(
        "qemu-live-descriptor-image-installation-smoke: final participants={} expected=15 errors={} classification={}",
        participants,
        errors,
        classification
    );
    if participants == 15 && errors == 0 {
        crate::println!("qemu-live-descriptor-image-installation-smoke: PASS");
        true
    } else {
        crate::println!("qemu-live-descriptor-image-installation-smoke: FAIL");
        false
    }
}

#[cfg(talos_boot_scenario = "qemu_live_descriptor_image_installation_smoke")]
fn live_descriptor_image_installation_report_success()
-> (bool, bool, bool, bool, bool, bool, bool, bool, bool) {
    let Ok((_image, _install_plan, address_space, materialization, activation_plan, reachability_plan)) =
        kernel_half_descriptor_image_valid_fixture()
    else {
        live_descriptor_image_installation_report_empty_success();
        return (false, false, false, false, false, false, false, false, false);
    };
    let mut descriptor_source = KernelHalfDescriptorImageLeaseSource::for_descriptor_image();
    let Ok(descriptor_image) = construct_kernel_half_descriptor_image(
        reachability_plan,
        materialization,
        KernelHalfDescriptorImageRequest::ConstructOnly,
        &mut descriptor_source,
    ) else {
        live_descriptor_image_installation_report_empty_success();
        return (false, false, false, false, false, false, false, false, false);
    };
    let mut installation_source =
        LiveDescriptorImageInstallationLeaseSource::for_single_installation();
    let Ok(installation) = install_live_descriptor_image_binding(
        Some(descriptor_image),
        Some(activation_plan),
        LiveDescriptorImageInstallationRequest::InstallModelBinding,
        &mut installation_source,
    ) else {
        live_descriptor_image_installation_report_empty_success();
        return (false, false, false, false, false, false, false, false, false);
    };

    crate::println!(
        "qemu-live-descriptor-image-installation-smoke: fixture name={} path=/bin/init source-digest={:#x} install-boundary={} address-space-boundary={} materialization-boundary={} launch-boundary={} stack-boundary={} activation-boundary={} reachability-boundary={} descriptor-image-boundary={} installation-boundary={} installation-policy={}",
        PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY,
        installation.source_digest(),
        PROCESS_INSTALL_BOUNDARY_IDENTITY,
        PROCESS_ADDRESS_SPACE_BOUNDARY_IDENTITY,
        PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY,
        INITIAL_PROCESS_LAUNCH_BOUNDARY_IDENTITY,
        INITIAL_USER_STACK_BOUNDARY_IDENTITY,
        LIVE_ADDRESS_SPACE_ACTIVATION_BOUNDARY_IDENTITY,
        KERNEL_HALF_REACHABILITY_BOUNDARY_IDENTITY,
        KERNEL_HALF_DESCRIPTOR_IMAGE_BOUNDARY_IDENTITY,
        LIVE_DESCRIPTOR_IMAGE_INSTALLATION_BOUNDARY_IDENTITY,
        LIVE_DESCRIPTOR_IMAGE_INSTALLATION_POLICY
    );

    let success_ok = installation.published()
        && installation.boundary_identity() == LIVE_DESCRIPTOR_IMAGE_INSTALLATION_BOUNDARY_IDENTITY
        && installation.policy_identity() == LIVE_DESCRIPTOR_IMAGE_INSTALLATION_POLICY
        && installation.image_fixture_identity() == PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY
        && installation.activation_boundary_identity() == LIVE_ADDRESS_SPACE_ACTIVATION_BOUNDARY_IDENTITY
        && installation.reachability_boundary_identity() == KERNEL_HALF_REACHABILITY_BOUNDARY_IDENTITY
        && installation.descriptor_image_boundary_identity() == KERNEL_HALF_DESCRIPTOR_IMAGE_BOUNDARY_IDENTITY
        && installation.materialization_boundary_identity() == PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY
        && installation.source_path() == PHASE8_INIT_PATH
        && installation.address_space_id() == address_space.id().raw()
        && installation.materialization_id() == materialization.id();
    crate::println!(
        "qemu-live-descriptor-image-installation-smoke: success output=KernelHalfDescriptorImageInstallation published={} copied-identities={} installation-boundary={} installation-policy={} ok={}",
        installation.published(),
        success_ok,
        installation.boundary_identity(),
        installation.policy_identity(),
        success_ok
    );

    let input = installation.input_state();
    let input_ok = input.descriptor_published()
        && !input.descriptor_installed()
        && !input.descriptor_image_installed()
        && !input.ttbr1_written()
        && input.activation_published()
        && input.activation_model_only();
    crate::println!(
        "qemu-live-descriptor-image-installation-smoke: input-state descriptor-published={} descriptor-installed={} descriptor-image-installed={} ttbr1-written={} activation-published={} activation-model-only={} ok={}",
        input.descriptor_published(),
        input.descriptor_installed(),
        input.descriptor_image_installed(),
        input.ttbr1_written(),
        input.activation_published(),
        input.activation_model_only(),
        input_ok
    );

    let ttbr = installation.ttbr_provenance();
    let ttbr_ok = ttbr.ttbr0_root() == TTBR0_ROOT_MATERIALIZED_PROVENANCE_ONLY
        && !ttbr.ttbr0_written()
        && ttbr.ttbr1_root() == TTBR1_DESCRIPTOR_IMAGE_ROOT_PROVENANCE
        && !ttbr.ttbr1_written()
        && !ttbr.active_root_copied();
    crate::println!(
        "qemu-live-descriptor-image-installation-smoke: ttbr-provenance ttbr0-root={} ttbr0-written={} ttbr1-root={} ttbr1-written={} active-root-copied={} ok={}",
        ttbr.ttbr0_root(),
        ttbr.ttbr0_written(),
        ttbr.ttbr1_root(),
        ttbr.ttbr1_written(),
        ttbr.active_root_copied(),
        ttbr_ok
    );

    let coverage = installation.coverage();
    let coverage_ok = coverage.kernel_text()
        && coverage.rodata()
        && coverage.data()
        && coverage.bss()
        && coverage.vectors()
        && coverage.active_stack()
        && coverage.heap()
        && coverage.page_frames()
        && coverage.uart_mmio_diagnostics()
        && coverage.scheduler_code_data()
        && coverage.runtime_console()
        && coverage.panic_fault_reporting();
    crate::println!(
        "qemu-live-descriptor-image-installation-smoke: coverage kernel-text={} rodata={} data={} bss={} vectors={} active-stack={} heap={} page-frames={} uart-mmio-diagnostics={} scheduler-code-data={} runtime-console={} panic-fault-reporting={} ok={}",
        coverage.kernel_text(),
        coverage.rodata(),
        coverage.data(),
        coverage.bss(),
        coverage.vectors(),
        coverage.active_stack(),
        coverage.heap(),
        coverage.page_frames(),
        coverage.uart_mmio_diagnostics(),
        coverage.scheduler_code_data(),
        coverage.runtime_console(),
        coverage.panic_fault_reporting(),
        coverage_ok
    );

    let permissions = installation.permissions();
    let permissions_ok = permissions.text_exec_privileged_only()
        && !permissions.rodata_write()
        && !permissions.data_exec()
        && !permissions.device_normal_memory()
        && !permissions.el0_kernel_access()
        && !permissions.wx_normal_memory();
    crate::println!(
        "qemu-live-descriptor-image-installation-smoke: permissions text-exec=privileged-only rodata-write={} data-exec={} device-normal-memory={} el0-kernel-access={} wx-normal-memory={} ok={}",
        permissions.rodata_write(),
        permissions.data_exec(),
        permissions.device_normal_memory(),
        permissions.el0_kernel_access(),
        permissions.wx_normal_memory(),
        permissions_ok
    );

    let installation_ok = installation.previous_state() == DESCRIPTOR_IMAGE_NOT_INSTALLED_STATE
        && installation.next_state() == DESCRIPTOR_IMAGE_INSTALLATION_READY_STATE
        && installation.live_register_state() == LIVE_REGISTER_SEQUENCE_BLOCKED;
    crate::println!(
        "qemu-live-descriptor-image-installation-smoke: installation-state previous={} next={} live-register-state={} ok={}",
        installation.previous_state(),
        installation.next_state(),
        installation.live_register_state(),
        installation_ok
    );

    let compatibility_ok = installation.tcr_state() == TCR_COMPATIBILITY_RECORD_ONLY
        && installation.mair_state() == activation_plan.mair_state()
        && installation.sctlr_state() == SCTLR_MUTATION_BLOCKED;
    crate::println!(
        "qemu-live-descriptor-image-installation-smoke: compatibility tcr-state={} mair-state={} sctlr-state={} ok={}",
        installation.tcr_state(),
        installation.mair_state(),
        installation.sctlr_state(),
        compatibility_ok
    );

    let blocked_ok = installation.asid_state() == ASID_ALLOCATION_BLOCKED
        && installation.tlb_state() == TLB_INVALIDATION_BLOCKED
        && installation.barrier_state() == BARRIER_SEQUENCE_PLANNED_ONLY
        && !installation.lower_el_eret()
        && !installation.scheduler_publication()
        && !installation.filesystem_syscalls();
    crate::println!(
        "qemu-live-descriptor-image-installation-smoke: blocked-states asid={} tlb={} barriers={} lower-el-eret={} scheduler-publication={} filesystem-syscalls={} ok={}",
        installation.asid_state(),
        installation.tlb_state(),
        installation.barrier_state(),
        installation.lower_el_eret(),
        installation.scheduler_publication(),
        installation.filesystem_syscalls(),
        blocked_ok
    );

    let effects = installation.side_effects();
    let effects_ok = !effects.ttbr_mutated()
        && !effects.tcr_mutated()
        && !effects.mair_mutated()
        && !effects.sctlr_mutated()
        && !effects.active_root_copied()
        && !effects.descriptor_table_published()
        && !effects.asid_allocated()
        && !effects.tlb_mutated()
        && !effects.live_dsb_isb()
        && !effects.lower_el_eret()
        && !effects.scheduler_published()
        && !effects.process_table_mutated()
        && !effects.filesystem_mutated()
        && !effects.hardware_action();
    crate::println!(
        "qemu-live-descriptor-image-installation-smoke: side-effects ttbr-mutated={} tcr-mutated={} mair-mutated={} sctlr-mutated={} active-root-copied={} descriptor-table-published={} asid-allocated={} tlb-mutated={} live-dsb-isb={} lower-el-eret={} scheduler-published={} process-table-mutated={} filesystem-mutated={} hardware-action={} ok={}",
        effects.ttbr_mutated(),
        effects.tcr_mutated(),
        effects.mair_mutated(),
        effects.sctlr_mutated(),
        effects.active_root_copied(),
        effects.descriptor_table_published(),
        effects.asid_allocated(),
        effects.tlb_mutated(),
        effects.live_dsb_isb(),
        effects.lower_el_eret(),
        effects.scheduler_published(),
        effects.process_table_mutated(),
        effects.filesystem_mutated(),
        effects.hardware_action(),
        effects_ok
    );

    (
        success_ok,
        input_ok,
        ttbr_ok,
        coverage_ok,
        permissions_ok,
        installation_ok,
        compatibility_ok,
        blocked_ok,
        effects_ok,
    )
}

#[cfg(talos_boot_scenario = "qemu_live_descriptor_image_installation_smoke")]
fn live_descriptor_image_installation_report_empty_success() {
    crate::println!(
        "qemu-live-descriptor-image-installation-smoke: fixture name={} path=/bin/init source-digest=0x0 install-boundary={} address-space-boundary={} materialization-boundary={} launch-boundary={} stack-boundary={} activation-boundary={} reachability-boundary={} descriptor-image-boundary={} installation-boundary={} installation-policy={}",
        PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY,
        PROCESS_INSTALL_BOUNDARY_IDENTITY,
        PROCESS_ADDRESS_SPACE_BOUNDARY_IDENTITY,
        PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY,
        INITIAL_PROCESS_LAUNCH_BOUNDARY_IDENTITY,
        INITIAL_USER_STACK_BOUNDARY_IDENTITY,
        LIVE_ADDRESS_SPACE_ACTIVATION_BOUNDARY_IDENTITY,
        KERNEL_HALF_REACHABILITY_BOUNDARY_IDENTITY,
        KERNEL_HALF_DESCRIPTOR_IMAGE_BOUNDARY_IDENTITY,
        LIVE_DESCRIPTOR_IMAGE_INSTALLATION_BOUNDARY_IDENTITY,
        LIVE_DESCRIPTOR_IMAGE_INSTALLATION_POLICY
    );
    crate::println!(
        "qemu-live-descriptor-image-installation-smoke: success output=KernelHalfDescriptorImageInstallation published=false copied-identities=false installation-boundary={} installation-policy={} ok=false",
        LIVE_DESCRIPTOR_IMAGE_INSTALLATION_BOUNDARY_IDENTITY,
        LIVE_DESCRIPTOR_IMAGE_INSTALLATION_POLICY
    );
}

#[cfg(talos_boot_scenario = "qemu_live_descriptor_image_installation_smoke")]
fn live_descriptor_image_installation_report_teardown() -> bool {
    let Ok((_image, _install_plan, _address_space, materialization, activation_plan, reachability_plan)) =
        kernel_half_descriptor_image_valid_fixture()
    else {
        live_descriptor_image_installation_print_empty_teardown();
        return false;
    };
    let mut descriptor_source = KernelHalfDescriptorImageLeaseSource::for_descriptor_image();
    let Ok(descriptor_image) = construct_kernel_half_descriptor_image(
        reachability_plan,
        materialization,
        KernelHalfDescriptorImageRequest::ConstructOnly,
        &mut descriptor_source,
    ) else {
        live_descriptor_image_installation_print_empty_teardown();
        return false;
    };
    let mut installation_source =
        LiveDescriptorImageInstallationLeaseSource::for_single_installation();
    let Ok(mut installation) = install_live_descriptor_image_binding(
        Some(descriptor_image),
        Some(activation_plan),
        LiveDescriptorImageInstallationRequest::InstallModelBinding,
        &mut installation_source,
    ) else {
        live_descriptor_image_installation_print_empty_teardown();
        return false;
    };

    let first = installation.destroy(&mut installation_source);
    let first_ok = first.installation_cleared()
        && first.descriptor_input_owned()
        && first.activation_input_owned()
        && !first.live_state_mutated()
        && !first.already_destroyed()
        && !installation.published()
        && installation.destroyed()
        && installation_source.outstanding_leases() == 0;
    crate::println!(
        "qemu-live-descriptor-image-installation-smoke: teardown phase=first installation-cleared={} descriptor-input-owned={} activation-input-owned={} live-state-mutated={} already-destroyed={} ok={}",
        first.installation_cleared(),
        first.descriptor_input_owned(),
        first.activation_input_owned(),
        first.live_state_mutated(),
        first.already_destroyed(),
        first_ok
    );

    let second = installation.destroy(&mut installation_source);
    let second_ok = !second.installation_cleared()
        && second.descriptor_input_owned()
        && second.activation_input_owned()
        && second.already_destroyed()
        && !second.live_state_mutated()
        && installation_source.outstanding_leases() == 0;
    crate::println!(
        "qemu-live-descriptor-image-installation-smoke: teardown phase=second installation-cleared={} descriptor-input-owned={} activation-input-owned={} already-destroyed={} ok={}",
        second.installation_cleared(),
        second.descriptor_input_owned(),
        second.activation_input_owned(),
        second.already_destroyed(),
        second_ok
    );

    first_ok && second_ok
}

#[cfg(talos_boot_scenario = "qemu_live_descriptor_image_installation_smoke")]
fn live_descriptor_image_installation_print_empty_teardown() {
    crate::println!(
        "qemu-live-descriptor-image-installation-smoke: teardown phase=first installation-cleared=false descriptor-input-owned=false activation-input-owned=false live-state-mutated=false already-destroyed=false ok=false"
    );
    crate::println!(
        "qemu-live-descriptor-image-installation-smoke: teardown phase=second installation-cleared=false descriptor-input-owned=false activation-input-owned=false already-destroyed=false ok=false"
    );
}

#[cfg(talos_boot_scenario = "qemu_live_descriptor_image_installation_smoke")]
fn live_descriptor_image_installation_report_error(
    case: &str,
    fixture_request: Option<LiveDescriptorImageInstallationRequest>,
    request: LiveDescriptorImageInstallationRequest,
    mut lease_source: LiveDescriptorImageInstallationLeaseSource,
    expected: PosixError,
    descriptor_installed: bool,
) -> bool {
    let result = if fixture_request.is_some() {
        live_descriptor_image_installation_fixture().and_then(|(descriptor_image, activation_plan)| {
            install_live_descriptor_image_binding(
                Some(descriptor_image),
                Some(activation_plan),
                request,
                &mut lease_source,
            )
        })
    } else {
        live_descriptor_image_installation_fixture().and_then(|(_descriptor_image, activation_plan)| {
            install_live_descriptor_image_binding(
                None,
                Some(activation_plan),
                request,
                &mut lease_source,
            )
        })
    };
    live_descriptor_image_installation_print_error_result(
        case,
        result,
        lease_source,
        expected,
        descriptor_installed,
    )
}

#[cfg(talos_boot_scenario = "qemu_live_descriptor_image_installation_smoke")]
fn live_descriptor_image_installation_report_destroyed_input_error() -> bool {
    let mut lease_source = LiveDescriptorImageInstallationLeaseSource::for_single_installation();
    let result = live_descriptor_image_installation_fixture_with_descriptor_source().and_then(
        |(mut descriptor_image, activation_plan, mut descriptor_source)| {
            let _ = descriptor_image.destroy(&mut descriptor_source);
            install_live_descriptor_image_binding(
                Some(descriptor_image),
                Some(activation_plan),
                LiveDescriptorImageInstallationRequest::InstallModelBinding,
                &mut lease_source,
            )
        },
    );
    live_descriptor_image_installation_print_error_result(
        "destroyed-input",
        result,
        lease_source,
        PosixError::InvalidArgument,
        false,
    )
}

#[cfg(talos_boot_scenario = "qemu_live_descriptor_image_installation_smoke")]
fn live_descriptor_image_installation_print_error_result(
    case: &str,
    result: Result<crate::live_descriptor_image_installation::KernelHalfDescriptorImageInstallation, PosixError>,
    lease_source: LiveDescriptorImageInstallationLeaseSource,
    expected: PosixError,
    descriptor_installed: bool,
) -> bool {
    let (errno, expected_error) = match result {
        Ok(_) => (expected, false),
        Err(error) => (error, error == expected),
    };
    let partial_installation = lease_source.outstanding_leases() != 0;
    let ok = expected_error && !partial_installation;
    crate::println!(
        "qemu-live-descriptor-image-installation-smoke: error case={} errno=-{} partial-installation={} descriptor-installed={} live-state-mutated=false ok={}",
        case,
        errno.name(),
        partial_installation,
        descriptor_installed,
        ok
    );
    ok
}

#[cfg(talos_boot_scenario = "qemu_live_descriptor_image_installation_smoke")]
fn live_descriptor_image_installation_fixture() -> Result<
    (
        crate::kernel_half_descriptor_image::KernelHalfDescriptorImage,
        LiveAddressSpaceActivationPlan,
    ),
    PosixError,
> {
    live_descriptor_image_installation_fixture_with_descriptor_source()
        .map(|(descriptor_image, activation_plan, _descriptor_source)| {
            (descriptor_image, activation_plan)
        })
}

#[cfg(talos_boot_scenario = "qemu_live_descriptor_image_installation_smoke")]
fn live_descriptor_image_installation_fixture_with_descriptor_source() -> Result<
    (
        crate::kernel_half_descriptor_image::KernelHalfDescriptorImage,
        LiveAddressSpaceActivationPlan,
        KernelHalfDescriptorImageLeaseSource,
    ),
    PosixError,
> {
    let Ok((_image, _install_plan, _address_space, materialization, activation_plan, reachability_plan)) =
        kernel_half_descriptor_image_valid_fixture()
    else {
        return Err(PosixError::InvalidArgument);
    };
    let mut descriptor_source = KernelHalfDescriptorImageLeaseSource::for_descriptor_image();
    let descriptor_image = construct_kernel_half_descriptor_image(
        reachability_plan,
        materialization,
        KernelHalfDescriptorImageRequest::ConstructOnly,
        &mut descriptor_source,
    )?;
    Ok((descriptor_image, activation_plan, descriptor_source))
}
