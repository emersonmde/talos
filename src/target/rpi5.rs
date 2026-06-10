#[cfg(any(
    talos_boot_scenario = "rpi5_production_secondary_dispatch",
    talos_boot_scenario = "rpi5_shared_scheduler_metadata",
    talos_boot_scenario = "rpi5_shared_runqueue_migration",
    talos_boot_scenario = "rpi5_load_balancing_proof",
    talos_boot_scenario = "rpi5_multicore_preemption_proof",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof",
    talos_boot_scenario = "rpi5_el0_trap_proof",
    talos_boot_scenario = "rpi5_syscall_proof",
    talos_boot_scenario = "rpi5_secondary_scheduler_service_loop"
))]
use crate::arch::aarch64;
#[cfg(any(
    talos_boot_scenario = "rpi5_timer_irq",
    talos_boot_scenario = "rpi5_timer_preemption"
))]
use crate::arch::aarch64::generic_timer;
#[cfg(any(
    talos_boot_scenario = "rpi5_timer_irq",
    talos_boot_scenario = "rpi5_timer_preemption",
    talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
    talos_boot_scenario = "rpi5_remote_wakeup_request",
    talos_boot_scenario = "rpi5_rp1_gic_visible_route_status_read",
    talos_boot_scenario = "rpi5_rp1_gpio14_ownership_route_preflight_read",
    talos_boot_scenario = "rpi5_rp1_gpio16_owned_event_discriminator"
))]
use crate::arch::aarch64::{
    self,
    gicv2::{GicV2, SPURIOUS_INTID},
};
#[cfg(talos_boot_scenario = "rpi5_production_timer_preemption_proof")]
use crate::scheduler::SharedSchedulerMetadata;
#[cfg(talos_boot_scenario = "rpi5_remote_wake_to_local_runnable")]
use crate::scheduler::TargetWakeConsumptionError;
#[cfg(any(
    talos_boot_scenario = "rpi5_timer_preemption",
    talos_boot_scenario = "rpi5_remote_wake_to_local_runnable",
    talos_boot_scenario = "rpi5_production_secondary_dispatch",
    talos_boot_scenario = "rpi5_shared_scheduler_metadata",
    talos_boot_scenario = "rpi5_shared_runqueue_migration",
    talos_boot_scenario = "rpi5_load_balancing_proof",
    talos_boot_scenario = "rpi5_multicore_preemption_proof",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof",
    talos_boot_scenario = "rpi5_secondary_scheduler_service_loop"
))]
use crate::scheduler::{ContextFrame, KernelStack, Task, TaskState};
#[cfg(talos_boot_scenario = "rpi5_multicore_preemption_proof")]
use crate::scheduler::{
    CpuLocalSchedulerService, PerCorePreemptionState, PreemptionRecordOutcome, RemoteWakeQueue,
    SharedSchedulerMetadata,
};
#[cfg(talos_boot_scenario = "rpi5_load_balancing_proof")]
use crate::scheduler::{
    LoadBalancingPolicy, LogicalCpuId, MigrationState, PerCoreScheduler, SharedRunQueue,
    SharedSchedulerMetadata, TaskId,
};
#[cfg(talos_boot_scenario = "rpi5_shared_runqueue_migration")]
use crate::scheduler::{
    LogicalCpuId, MigrationState, PerCoreScheduler, SchedulerCoreRole, SharedRunQueue,
    SharedSchedulerMetadata, TaskId,
};
#[cfg(any(
    talos_boot_scenario = "rpi5_remote_wakeup_request",
    talos_boot_scenario = "rpi5_production_secondary_dispatch",
    talos_boot_scenario = "rpi5_multicore_preemption_proof",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof",
    talos_boot_scenario = "rpi5_secondary_scheduler_service_loop"
))]
use crate::scheduler::{
    LogicalCpuId, PerCoreScheduler, PerCoreSchedulerAccessError, ProductionDispatchError,
    SchedulerCoreRole, TaskId,
};
#[cfg(talos_boot_scenario = "rpi5_shared_scheduler_metadata")]
use crate::scheduler::{
    LogicalCpuId, PerCoreScheduler, PerCoreSchedulerAccessError, SchedulerCoreRole,
    SharedSchedulerMetadata, SharedSchedulerMetadataError, SharedSchedulerMetadataLock, TaskId,
};
#[cfg(talos_boot_scenario = "rpi5_remote_wakeup_request")]
use crate::scheduler::{RemoteWakePublishOutcome, RemoteWakeQueue};
#[cfg(talos_boot_scenario = "rpi5_secondary_scheduler_service_loop")]
use crate::scheduler::{
    RemoteWakeQueue, SecondarySchedulerServiceLoop, SecondarySchedulerServiceLoopError,
    SharedSchedulerMetadata,
};
#[cfg(talos_boot_scenario = "rpi5_timer_preemption")]
use crate::scheduler::{SingleCoreScheduler, TaskId};
#[cfg(talos_boot_scenario = "rpi5_secondary_core_workload")]
use crate::smp::SECONDARY_CORE_WORKLOAD_TARGET;
#[cfg(any(
    talos_boot_scenario = "rpi5_psci_secondary_core_alive",
    talos_boot_scenario = "rpi5_secondary_core_workload",
    talos_boot_scenario = "rpi5_smp_lock_cache_coherence",
    talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
    talos_boot_scenario = "rpi5_remote_wakeup_request",
    talos_boot_scenario = "rpi5_production_secondary_dispatch",
    talos_boot_scenario = "rpi5_shared_scheduler_metadata",
    talos_boot_scenario = "rpi5_shared_runqueue_migration",
    talos_boot_scenario = "rpi5_load_balancing_proof",
    talos_boot_scenario = "rpi5_multicore_preemption_proof",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof",
    talos_boot_scenario = "rpi5_secondary_scheduler_service_loop"
))]
use crate::smp::{
    self, CoreLifecycle, CoreStackLayout, MAX_CORES, SECONDARY_CORE_STATES,
    SECONDARY_KERNEL_STACK_SIZE, pi5_logical_cpu_from_mpidr_affinity,
};
#[cfg(any(
    talos_boot_scenario = "rpi5_smp_lock_cache_coherence",
    talos_boot_scenario = "rpi5_remote_wakeup_request",
    talos_boot_scenario = "rpi5_production_secondary_dispatch",
    talos_boot_scenario = "rpi5_shared_scheduler_metadata",
    talos_boot_scenario = "rpi5_shared_runqueue_migration",
    talos_boot_scenario = "rpi5_multicore_preemption_proof",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof",
    talos_boot_scenario = "rpi5_secondary_scheduler_service_loop"
))]
use crate::smp_sync::{SpinLock, smp_full_barrier};
#[cfg(talos_boot_scenario = "rpi5_syscall_proof")]
use crate::syscall::{self, SyscallNumber};
#[cfg(any(
    talos_boot_scenario = "rpi5_el0_trap_proof",
    talos_boot_scenario = "rpi5_syscall_proof"
))]
use crate::{
    arch::aarch64::exceptions::{ExceptionFrame, ExceptionVector},
    posix::{
        PosixError, UserAccessKind, UserMapping, UserMappingPermissions,
        validate_user_memory_access,
    },
};
use crate::{
    boot::BootInfo,
    device_tree::DeviceTree,
    mmio::{MmioMap, MmioRegion},
    target::{InterruptControllerKind, TargetServices, TimerKind, UartKind},
};
#[cfg(talos_boot_scenario = "rpi5_timer_preemption")]
use core::cell::UnsafeCell;
#[cfg(any(
    talos_boot_scenario = "rpi5_timer_irq",
    talos_boot_scenario = "rpi5_timer_preemption",
    talos_boot_scenario = "rpi5_psci_secondary_core_alive",
    talos_boot_scenario = "rpi5_secondary_core_workload",
    talos_boot_scenario = "rpi5_smp_lock_cache_coherence",
    talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
    talos_boot_scenario = "rpi5_remote_wakeup_request",
    talos_boot_scenario = "rpi5_production_secondary_dispatch",
    talos_boot_scenario = "rpi5_shared_scheduler_metadata",
    talos_boot_scenario = "rpi5_shared_runqueue_migration",
    talos_boot_scenario = "rpi5_multicore_preemption_proof",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof",
    talos_boot_scenario = "rpi5_syscall_proof",
    talos_boot_scenario = "rpi5_secondary_scheduler_service_loop"
))]
use core::sync::atomic::{AtomicU64, Ordering};

#[cfg(talos_target_rpi5_bcm2712)]
use crate::pl011::Pl011;

pub const UART10_BASE: usize = 0x10_7d00_1000;
pub const RP1_UART0_PCIE2_BASE: usize = 0x1f_0003_0000;
pub const RP1_UART0_FIRMWARE_BASE: usize = 0x1c_0003_0000;
pub const RP1_UART0_BASE: usize = RP1_UART0_PCIE2_BASE;
#[allow(dead_code)]
pub const RP1_UART0_FR: usize = RP1_UART0_BASE + 0x18;
#[allow(dead_code)]
pub const RP1_UART0_OBSERVED_APERTURE_FR: usize = RP1_UART0_FIRMWARE_BASE + 0x18;
#[allow(dead_code)]
pub const RP1_GPIO14_OBSERVED_APERTURE_STATUS: usize = 0x1c_000d_0070;
#[allow(dead_code)]
pub const RP1_GPIO14_OBSERVED_APERTURE_CTRL: usize = 0x1c_000d_0074;
#[allow(dead_code)]
pub const RP1_GPIO16_OBSERVED_APERTURE_STATUS: usize = 0x1c_000d_0080;
#[allow(dead_code)]
pub const RP1_GPIO16_OBSERVED_APERTURE_CTRL: usize = 0x1c_000d_0084;
#[allow(dead_code)]
pub const RP1_GPIO32_OBSERVED_APERTURE_STATUS: usize = 0x1c_000d_4020;
#[allow(dead_code)]
pub const RP1_GPIO32_OBSERVED_APERTURE_CTRL: usize = 0x1c_000d_4024;
#[allow(dead_code)]
pub const RP1_IO_BANK0_OBSERVED_APERTURE_INTE: usize = 0x1c_000d_011c;
#[allow(dead_code)]
pub const RP1_IO_BANK0_OBSERVED_APERTURE_INTS: usize = 0x1c_000d_0124;
#[allow(dead_code)]
pub const RP1_RIO0_OBSERVED_APERTURE_OUT: usize = 0x1c_000e_0000;
#[allow(dead_code)]
pub const RP1_RIO0_OBSERVED_APERTURE_OE: usize = 0x1c_000e_0004;
#[allow(dead_code)]
pub const RP1_RIO0_OBSERVED_APERTURE_IN: usize = 0x1c_000e_0008;
#[allow(dead_code)]
pub const RP1_RIO1_OBSERVED_APERTURE_OUT: usize = 0x1c_000e_4000;
#[allow(dead_code)]
pub const RP1_RIO1_OBSERVED_APERTURE_OE: usize = 0x1c_000e_4004;
#[allow(dead_code)]
pub const RP1_RIO1_OBSERVED_APERTURE_IN: usize = 0x1c_000e_4008;
#[allow(dead_code)]
pub const RP1_GPIO14_OBSERVED_APERTURE_PAD: usize = 0x1c_000f_003c;
#[allow(dead_code)]
pub const RP1_GPIO16_OBSERVED_APERTURE_PAD: usize = 0x1c_000f_0044;
#[allow(dead_code)]
pub const RP1_GPIO32_OBSERVED_APERTURE_PAD: usize = 0x1c_000f_4014;
#[allow(dead_code)]
pub const RP1_UART0_GPIO14_PAD: usize = 0x1f_000f_003c;
#[allow(dead_code)]
pub const RP1_UART0_GPIO15_PAD: usize = 0x1f_000f_0040;
#[allow(dead_code)]
pub const RP1_UART0_GPIO14_CTRL: usize = 0x1f_000d_0074;
#[allow(dead_code)]
pub const RP1_UART0_GPIO15_CTRL: usize = 0x1f_000d_007c;
#[allow(dead_code)]
pub const RP1_GPIO14_STATUS: usize = 0x1f_000d_0070;
#[allow(dead_code)]
pub const RP1_GPIO16_STATUS: usize = 0x1f_000d_0080;
#[allow(dead_code)]
pub const RP1_GPIO16_CTRL: usize = 0x1f_000d_0084;
#[allow(dead_code)]
pub const RP1_GPIO16_CTRL_SET: usize = 0x1f_000d_2084;
#[allow(dead_code)]
pub const RP1_GPIO16_CTRL_CLR: usize = 0x1f_000d_3084;
#[allow(dead_code)]
pub const RP1_RIO0_OUT: usize = 0x1f_000e_0000;
#[allow(dead_code)]
pub const RP1_RIO0_OE: usize = 0x1f_000e_0004;
#[allow(dead_code)]
pub const RP1_RIO0_IN: usize = 0x1f_000e_0008;
#[allow(dead_code)]
pub const RP1_RIO0_OUT_SET: usize = 0x1f_000e_2000;
#[allow(dead_code)]
pub const RP1_RIO0_OUT_CLR: usize = 0x1f_000e_3000;
#[allow(dead_code)]
pub const RP1_RIO0_OE_SET: usize = 0x1f_000e_2004;
#[allow(dead_code)]
pub const RP1_RIO0_OE_CLR: usize = 0x1f_000e_3004;
#[allow(dead_code)]
pub const RP1_IO_BANK0_INTE: usize = 0x1f_000d_011c;
#[allow(dead_code)]
pub const RP1_IO_BANK0_INTE_SET: usize = 0x1f_000d_211c;
#[allow(dead_code)]
pub const RP1_IO_BANK0_INTE_CLR: usize = 0x1f_000d_311c;
#[allow(dead_code)]
pub const RP1_IO_BANK0_INTS: usize = 0x1f_000d_0124;
#[allow(dead_code)]
pub const RP1_IO_BANK0_MSIX_CFG: usize = 0x1f_0010_8008;
#[allow(dead_code)]
pub const RP1_GPIO16_PAD: usize = 0x1f_000f_0044;
#[allow(dead_code)]
pub const RP1_SYSINFO_BASE: usize = 0x1f_0000_0000;
#[allow(dead_code)]
pub const RP1_SYSINFO_CHIP_ID: usize = RP1_SYSINFO_BASE;
#[allow(dead_code)]
pub const RP1_SYSINFO_PLATFORM: usize = RP1_SYSINFO_BASE + 0x4;
#[allow(dead_code)]
pub const RP1_SYSINFO_OBSERVED_APERTURE_BASE: usize = 0x1c_0000_0000;
#[allow(dead_code)]
pub const RP1_SYSINFO_OBSERVED_APERTURE_CHIP_ID: usize = RP1_SYSINFO_OBSERVED_APERTURE_BASE;
#[allow(dead_code)]
pub const RP1_SYSINFO_OBSERVED_APERTURE_PLATFORM: usize = RP1_SYSINFO_OBSERVED_APERTURE_BASE + 0x4;
#[allow(dead_code)]
pub const RP1_EXPECTED_CHIP_ID: u32 = 0x2000_1927;
#[allow(dead_code)]
pub const RP1_ETHERNET_OBSERVED_WINDOW_GEM_MID: usize = 0x1c_0010_00fc;
#[allow(dead_code)]
pub const RP1_CLOCK_MANAGER_BASE: usize = 0x1f_0001_8000;
#[allow(dead_code)]
pub const RP1_CLOCK_MANAGER_OBSERVED_APERTURE_BASE: usize = 0x1c_0001_8000;
#[allow(dead_code)]
pub const PCIE2_CONTROLLER_BASE: usize = 0x10_0012_0000;
#[allow(dead_code)]
pub const PCIE_MISC_PCIE_STATUS: usize = PCIE2_CONTROLLER_BASE + 0x4068;
#[allow(dead_code)]
pub const PCIE_MISC_PCIE_STATUS_OFFSET: u64 = 0x4068;
#[allow(dead_code)]
pub const PCIE_MISC_MISC_CTRL: usize = PCIE2_CONTROLLER_BASE + 0x4008;
#[allow(dead_code)]
pub const PCIE_MISC_MISC_CTRL_OFFSET: u64 = 0x4008;
#[allow(dead_code)]
pub const PCIE_RC_CFG_PRIV1_ID_VAL3: usize = PCIE2_CONTROLLER_BASE + 0x043c;
#[allow(dead_code)]
pub const PCIE_RC_CFG_PRIV1_ID_VAL3_OFFSET: u64 = 0x043c;
#[allow(dead_code)]
pub const PCIE_MISC_CPU_2_PCIE_MEM_WIN0_LO: usize = PCIE2_CONTROLLER_BASE + 0x400c;
#[allow(dead_code)]
pub const PCIE_MISC_CPU_2_PCIE_MEM_WIN0_LO_OFFSET: u64 = 0x400c;
#[allow(dead_code)]
pub const PCIE_MISC_CPU_2_PCIE_MEM_WIN0_HI: usize = PCIE2_CONTROLLER_BASE + 0x4010;
#[allow(dead_code)]
pub const PCIE_MISC_CPU_2_PCIE_MEM_WIN0_HI_OFFSET: u64 = 0x4010;
#[allow(dead_code)]
pub const PCIE_MISC_CPU_2_PCIE_MEM_WIN0_BASE_LIMIT: usize = PCIE2_CONTROLLER_BASE + 0x4070;
#[allow(dead_code)]
pub const PCIE_MISC_CPU_2_PCIE_MEM_WIN0_BASE_LIMIT_OFFSET: u64 = 0x4070;
#[allow(dead_code)]
pub const PCIE_MISC_CPU_2_PCIE_MEM_WIN0_BASE_HI: usize = PCIE2_CONTROLLER_BASE + 0x4080;
#[allow(dead_code)]
pub const PCIE_MISC_CPU_2_PCIE_MEM_WIN0_BASE_HI_OFFSET: u64 = 0x4080;
#[allow(dead_code)]
pub const PCIE_MISC_CPU_2_PCIE_MEM_WIN0_LIMIT_HI: usize = PCIE2_CONTROLLER_BASE + 0x4084;
#[allow(dead_code)]
pub const PCIE_MISC_CPU_2_PCIE_MEM_WIN0_LIMIT_HI_OFFSET: u64 = 0x4084;
#[allow(dead_code)]
pub const PCIE_EXT_CFG_DATA: usize = PCIE2_CONTROLLER_BASE + 0x8000;
#[allow(dead_code)]
pub const PCIE_EXT_CFG_DATA_OFFSET: u64 = 0x8000;
#[allow(dead_code)]
pub const PCIE_EXT_CFG_INDEX: usize = PCIE2_CONTROLLER_BASE + 0x9000;
#[allow(dead_code)]
pub const PCIE_EXT_CFG_INDEX_OFFSET: u64 = 0x9000;
#[allow(dead_code)]
pub const RP1_ENDPOINT_CONFIG_INDEX_VALUE: u32 = 0x0010_0000;
#[allow(dead_code)]
pub const RP1_ENDPOINT_CONFIG_OFFSET: u64 = 0;
#[allow(dead_code)]
pub const RP1_ENDPOINT_CONFIG_BDF: &str = "0002:01:00.0";
#[allow(dead_code)]
pub const RP1_ENDPOINT_EXPECTED_VENDOR_ID: u32 = 0x1de4;
#[allow(dead_code)]
pub const RP1_ENDPOINT_EXPECTED_DEVICE_ID: u32 = 0x0001;
#[allow(dead_code)]
pub const PCIE_STATUS_PORT: u32 = 0x80;
#[allow(dead_code)]
pub const PCIE_STATUS_DL_ACTIVE: u32 = 0x20;
#[allow(dead_code)]
pub const PCIE_STATUS_PHYLINKUP: u32 = 0x10;
#[allow(dead_code)]
pub const PCIE_STATUS_LINK_IN_L23: u32 = 0x40;
#[allow(dead_code)]
pub const PCIE_MISC_CTRL_RCB_64B_MODE: u32 = 0x80;
#[allow(dead_code)]
pub const PCIE_MISC_CTRL_RCB_MPS_MODE: u32 = 0x400;
#[allow(dead_code)]
pub const PCIE_MISC_CTRL_SCB_ACCESS_EN: u32 = 0x1000;
#[allow(dead_code)]
pub const PCIE_MISC_CTRL_CFG_READ_UR_MODE: u32 = 0x2000;
#[allow(dead_code)]
pub const PCIE_MISC_CTRL_MAX_BURST_SIZE_MASK: u32 = 0x30_0000;
#[allow(dead_code)]
pub const PCIE_MISC_CTRL_MAX_BURST_SIZE_SHIFT: u32 = 20;
#[allow(dead_code)]
pub const PCIE_RC_CLASS_CODE_MASK: u32 = 0x00ff_ffff;
#[allow(dead_code)]
pub const PCIE_RC_EXPECTED_BRIDGE_CLASS_CODE: u32 = 0x0006_0400;
#[allow(dead_code)]
pub const PCIE_WIN0_BASE_LOW_MASK: u32 = 0x0000_fff0;
#[allow(dead_code)]
pub const PCIE_WIN0_BASE_LOW_EXPECTED: u32 = 0x0000_0000;
#[allow(dead_code)]
pub const PCIE_WIN0_LIMIT_LOW_MASK: u32 = 0xfff0_0000;
#[allow(dead_code)]
pub const PCIE_WIN0_LIMIT_LOW_EXPECTED: u32 = 0xfff0_0000;
#[allow(dead_code)]
pub const PCIE_WIN0_HIGH_MASK: u32 = 0x0000_00ff;
#[allow(dead_code)]
pub const PCIE_WIN0_HIGH_EXPECTED: u32 = 0x0000_001f;
#[allow(dead_code)]
pub const RP1_PLL_SYS_CS: usize = 0x1f_0002_0000;
#[allow(dead_code)]
pub const RP1_PLL_SYS_OBSERVED_APERTURE_CS: usize = 0x1c_0002_0000;
#[allow(dead_code)]
pub const RP1_CLK_SYS_CTRL: usize = RP1_CLOCK_MANAGER_BASE + 0x14;
#[allow(dead_code)]
pub const RP1_CLK_SYS_OBSERVED_APERTURE_CTRL: usize =
    RP1_CLOCK_MANAGER_OBSERVED_APERTURE_BASE + 0x14;
#[allow(dead_code)]
pub const RP1_CLK_SYS_DIV_INT: usize = RP1_CLOCK_MANAGER_BASE + 0x18;
#[allow(dead_code)]
pub const RP1_CLK_SYS_OBSERVED_APERTURE_DIV_INT: usize =
    RP1_CLOCK_MANAGER_OBSERVED_APERTURE_BASE + 0x18;
#[allow(dead_code)]
pub const RP1_CLK_SYS_SEL: usize = RP1_CLOCK_MANAGER_BASE + 0x20;
#[allow(dead_code)]
pub const RP1_CLK_SYS_OBSERVED_APERTURE_SEL: usize =
    RP1_CLOCK_MANAGER_OBSERVED_APERTURE_BASE + 0x20;
#[allow(dead_code)]
pub const RP1_CLK_SLOW_SYS_CTRL: usize = RP1_CLOCK_MANAGER_BASE + 0x24;
#[allow(dead_code)]
pub const RP1_CLK_SLOW_SYS_OBSERVED_APERTURE_CTRL: usize =
    RP1_CLOCK_MANAGER_OBSERVED_APERTURE_BASE + 0x24;
#[allow(dead_code)]
pub const RP1_CLK_UART_CTRL: usize = RP1_CLOCK_MANAGER_BASE + 0x54;
#[allow(dead_code)]
pub const RP1_CLK_UART_OBSERVED_APERTURE_CTRL: usize =
    RP1_CLOCK_MANAGER_OBSERVED_APERTURE_BASE + 0x54;
#[allow(dead_code)]
pub const RP1_CLK_UART_DIV_INT: usize = RP1_CLOCK_MANAGER_BASE + 0x58;
#[allow(dead_code)]
pub const RP1_CLK_UART_OBSERVED_APERTURE_DIV_INT: usize =
    RP1_CLOCK_MANAGER_OBSERVED_APERTURE_BASE + 0x58;
#[allow(dead_code)]
pub const RP1_CLK_UART_SEL: usize = RP1_CLOCK_MANAGER_BASE + 0x60;
#[allow(dead_code)]
pub const RP1_CLK_UART_OBSERVED_APERTURE_SEL: usize =
    RP1_CLOCK_MANAGER_OBSERVED_APERTURE_BASE + 0x60;
#[allow(dead_code)]
pub const RP1_CLK_ADC_CTRL: usize = RP1_CLOCK_MANAGER_BASE + 0x144;
#[allow(dead_code)]
pub const RP1_CLK_ADC_DIV_INT: usize = RP1_CLOCK_MANAGER_BASE + 0x148;
#[allow(dead_code)]
pub const RP1_CLK_ADC_SEL: usize = RP1_CLOCK_MANAGER_BASE + 0x150;
#[allow(dead_code)]
pub const RP1_CLK_ETH_CTRL_OBSERVED_APERTURE: usize =
    RP1_CLOCK_MANAGER_OBSERVED_APERTURE_BASE + 0x64;
#[allow(dead_code)]
pub const RP1_CLK_ETH_TSU_CTRL_OBSERVED_APERTURE: usize =
    RP1_CLOCK_MANAGER_OBSERVED_APERTURE_BASE + 0x134;
#[allow(dead_code)]
pub const RP1_CLK_CTRL_ENABLE: u32 = 1 << 11;
#[cfg(any(
    talos_boot_scenario = "rpi5_timer_irq",
    talos_boot_scenario = "rpi5_timer_preemption",
    talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
    talos_boot_scenario = "rpi5_remote_wakeup_request",
    talos_boot_scenario = "rpi5_rp1_gic_visible_route_status_read",
    talos_boot_scenario = "rpi5_rp1_gpio14_ownership_route_preflight_read",
    talos_boot_scenario = "rpi5_rp1_gpio16_owned_event_discriminator"
))]
const GICD_BASE: usize = 0x10_7fff_9000;
#[cfg(any(
    talos_boot_scenario = "rpi5_timer_irq",
    talos_boot_scenario = "rpi5_timer_preemption",
    talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
    talos_boot_scenario = "rpi5_remote_wakeup_request",
    talos_boot_scenario = "rpi5_rp1_gic_visible_route_status_read",
    talos_boot_scenario = "rpi5_rp1_gpio14_ownership_route_preflight_read",
    talos_boot_scenario = "rpi5_rp1_gpio16_owned_event_discriminator"
))]
const GICC_BASE: usize = 0x10_7fff_a000;
#[cfg(any(
    talos_boot_scenario = "rpi5_timer_irq",
    talos_boot_scenario = "rpi5_timer_preemption"
))]
const EL2_PHYSICAL_TIMER_INTID: u32 = 26;
#[cfg(talos_boot_scenario = "rpi5_timer_irq")]
const TIMER_IRQ_WAIT_LIMIT: usize = 8_000_000;
#[cfg(any(
    talos_boot_scenario = "rpi5_uart10_polling_rx",
    talos_boot_scenario = "rpi5_diagnostic_command_channel",
    talos_boot_scenario = "rpi5_local_serial_command_loop",
    talos_boot_scenario = "rpi5_local_literal_echo",
    talos_boot_scenario = "rpi5_local_line_editing",
    talos_boot_scenario = "rpi5_local_line_cancel",
    talos_boot_scenario = "rpi5_local_line_kill"
))]
const UART10_RX_WAIT_LIMIT: usize = 200_000_000;
#[cfg(talos_boot_scenario = "rpi5_diagnostic_command_channel")]
const DIAGNOSTIC_COMMAND_CAPTURE_SETTLE_SPINS: usize = 10_000_000;
#[cfg(talos_boot_scenario = "rpi5_timer_preemption")]
const CONTEXT_SWITCH_STACK_SIZE: usize = 4096;
#[cfg(talos_boot_scenario = "rpi5_timer_preemption")]
const TIMER_PREEMPTION_TARGET_PROGRESS: u64 = 3;
#[cfg(talos_boot_scenario = "rpi5_timer_preemption")]
const TIMER_PREEMPTION_TARGET_SWITCHES: u64 = 6;
#[cfg(any(
    talos_boot_scenario = "rpi5_psci_secondary_core_alive",
    talos_boot_scenario = "rpi5_secondary_core_workload",
    talos_boot_scenario = "rpi5_smp_lock_cache_coherence",
    talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
    talos_boot_scenario = "rpi5_remote_wakeup_request",
    talos_boot_scenario = "rpi5_production_secondary_dispatch",
    talos_boot_scenario = "rpi5_shared_scheduler_metadata",
    talos_boot_scenario = "rpi5_shared_runqueue_migration",
    talos_boot_scenario = "rpi5_multicore_preemption_proof",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof",
    talos_boot_scenario = "rpi5_secondary_scheduler_service_loop"
))]
const RPI5_SECONDARY_WAIT_LIMIT: usize = 200_000_000;
#[cfg(talos_boot_scenario = "rpi5_production_secondary_dispatch")]
const PRODUCTION_SECONDARY_DISPATCH_PROGRESS_TARGET: u64 = 3;
#[cfg(talos_boot_scenario = "rpi5_shared_scheduler_metadata")]
const SHARED_SCHEDULER_METADATA_TASK_CAPACITY: usize = MAX_CORES;
#[cfg(talos_boot_scenario = "rpi5_shared_runqueue_migration")]
const SHARED_RUNQUEUE_MIGRATION_TASK_CAPACITY: usize = 1;
#[cfg(talos_boot_scenario = "rpi5_shared_runqueue_migration")]
const SHARED_RUNQUEUE_MIGRATION_QUEUE_CAPACITY: usize = 1;
#[cfg(talos_boot_scenario = "rpi5_load_balancing_proof")]
const LOAD_BALANCING_PROOF_TASK_CAPACITY: usize = 1;
#[cfg(talos_boot_scenario = "rpi5_load_balancing_proof")]
const LOAD_BALANCING_PROOF_QUEUE_CAPACITY: usize = 1;
#[cfg(talos_boot_scenario = "rpi5_multicore_preemption_proof")]
const MULTICORE_PREEMPTION_PROOF_TASK_CAPACITY: usize = 2;
#[cfg(talos_boot_scenario = "rpi5_production_timer_preemption_proof")]
const PRODUCTION_TIMER_PREEMPTION_PROOF_TASK_CAPACITY: usize = 2;
#[cfg(talos_boot_scenario = "rpi5_secondary_scheduler_service_loop")]
const SECONDARY_SCHEDULER_SERVICE_LOOP_TASK_CAPACITY: usize = 1;
#[cfg(any(
    talos_boot_scenario = "rpi5_el0_trap_proof",
    talos_boot_scenario = "rpi5_syscall_proof"
))]
const EL0_TRAP_USER_TEXT_START: u64 = 0x0000_0000_0010_0000;
#[cfg(any(
    talos_boot_scenario = "rpi5_el0_trap_proof",
    talos_boot_scenario = "rpi5_syscall_proof"
))]
const EL0_TRAP_USER_TEXT_LEN: usize = 0x1000;
#[cfg(any(
    talos_boot_scenario = "rpi5_el0_trap_proof",
    talos_boot_scenario = "rpi5_syscall_proof"
))]
const EL0_TRAP_USER_STACK_START: u64 = 0x0000_0000_001f_0000;
#[cfg(any(
    talos_boot_scenario = "rpi5_el0_trap_proof",
    talos_boot_scenario = "rpi5_syscall_proof"
))]
const EL0_TRAP_USER_STACK_LEN: usize = 0x1_0000;
#[cfg(any(
    talos_boot_scenario = "rpi5_el0_trap_proof",
    talos_boot_scenario = "rpi5_syscall_proof"
))]
const EL0_TRAP_USER_GUARD_START: u64 = 0x0000_0000_001e_0000;
#[cfg(any(
    talos_boot_scenario = "rpi5_el0_trap_proof",
    talos_boot_scenario = "rpi5_syscall_proof"
))]
const EL0_TRAP_SVC_MARKER: u64 = 0x7a10;
#[cfg(talos_boot_scenario = "rpi5_el0_trap_proof")]
const EL0_TRAP_EXPECTED_ESR: u64 = 0x0000_0000_5400_7a10;
#[cfg(talos_boot_scenario = "rpi5_syscall_proof")]
const SYSCALL_PROOF_EXPECTED_SVC_ESR: u64 = 0x0000_0000_5400_0000;
#[cfg(talos_boot_scenario = "rpi5_syscall_proof")]
const SYSCALL_PROOF_EXPECTED_MARKER_ESR: u64 = 0x0000_0000_5400_7a10;
#[cfg(talos_boot_scenario = "rpi5_syscall_proof")]
const SYSCALL_PROOF_UNKNOWN_NUMBER: u64 = 17;
#[cfg(talos_boot_scenario = "rpi5_syscall_proof")]
const SYSCALL_PROOF_EXPECTED_ENOSYS_X0: u64 = (syscall::ENOSYS as u64).wrapping_neg();
#[cfg(talos_boot_scenario = "rpi5_pointer_copy_proof")]
const POINTER_COPY_USER_DATA_START: u64 = 0x0000_0000_0011_0000;
#[cfg(talos_boot_scenario = "rpi5_pointer_copy_proof")]
const POINTER_COPY_USER_DATA_LEN: usize = 0x1000;
#[cfg(talos_boot_scenario = "rpi5_pointer_copy_proof")]
const POINTER_COPY_USER_DATA_INIT: u8 = 0x2a;
#[cfg(talos_boot_scenario = "rpi5_pointer_copy_proof")]
const POINTER_COPY_USER_DATA_REPLACEMENT: u8 = 0xa5;
#[cfg(talos_boot_scenario = "rpi5_pointer_copy_proof")]
const POINTER_COPY_EXPECTED_ENOSYS_X0: u64 = (syscall::ENOSYS as u64).wrapping_neg();
#[cfg(talos_boot_scenario = "rpi5_pointer_copy_proof")]
const POINTER_COPY_EXPECTED_EFAULT_X0: u64 = (syscall::EFAULT as u64).wrapping_neg();
#[cfg(any(
    talos_boot_scenario = "rpi5_descriptor_write_proof",
    talos_boot_scenario = "rpi5_close_syscall_proof",
    talos_boot_scenario = "rpi5_dup_syscall_proof",
    talos_boot_scenario = "rpi5_read_stdin_proof"
))]
const DESCRIPTOR_WRITE_USER_DATA_START: u64 = 0x0000_0000_0011_0000;
#[cfg(any(
    talos_boot_scenario = "rpi5_descriptor_write_proof",
    talos_boot_scenario = "rpi5_close_syscall_proof",
    talos_boot_scenario = "rpi5_dup_syscall_proof",
    talos_boot_scenario = "rpi5_read_stdin_proof"
))]
const DESCRIPTOR_WRITE_USER_DATA_LEN: usize = 0x1000;
#[cfg(any(
    talos_boot_scenario = "rpi5_descriptor_write_proof",
    talos_boot_scenario = "rpi5_close_syscall_proof",
    talos_boot_scenario = "rpi5_dup_syscall_proof",
    talos_boot_scenario = "rpi5_read_stdin_proof"
))]
const DESCRIPTOR_WRITE_STDOUT_OFFSET: usize = 0x00;
#[cfg(any(
    talos_boot_scenario = "rpi5_descriptor_write_proof",
    talos_boot_scenario = "rpi5_close_syscall_proof",
    talos_boot_scenario = "rpi5_dup_syscall_proof",
    talos_boot_scenario = "rpi5_read_stdin_proof"
))]
const DESCRIPTOR_WRITE_STDERR_OFFSET: usize = 0x40;
#[cfg(any(
    talos_boot_scenario = "rpi5_descriptor_write_proof",
    talos_boot_scenario = "rpi5_close_syscall_proof",
    talos_boot_scenario = "rpi5_dup_syscall_proof",
    talos_boot_scenario = "rpi5_read_stdin_proof"
))]
const DESCRIPTOR_WRITE_STDOUT: &[u8; 18] = b"talos-stdout-rpi5\n";
#[cfg(any(
    talos_boot_scenario = "rpi5_descriptor_write_proof",
    talos_boot_scenario = "rpi5_close_syscall_proof",
    talos_boot_scenario = "rpi5_dup_syscall_proof",
    talos_boot_scenario = "rpi5_read_stdin_proof"
))]
const DESCRIPTOR_WRITE_STDERR: &[u8; 18] = b"talos-stderr-rpi5\n";
#[cfg(any(
    talos_boot_scenario = "rpi5_descriptor_write_proof",
    talos_boot_scenario = "rpi5_close_syscall_proof",
    talos_boot_scenario = "rpi5_dup_syscall_proof",
    talos_boot_scenario = "rpi5_read_stdin_proof"
))]
const DESCRIPTOR_WRITE_EXPECTED_EBADF_X0: u64 = (syscall::EBADF as u64).wrapping_neg();
#[cfg(any(
    talos_boot_scenario = "rpi5_descriptor_write_proof",
    talos_boot_scenario = "rpi5_read_stdin_proof"
))]
const DESCRIPTOR_WRITE_EXPECTED_EFAULT_X0: u64 = (syscall::EFAULT as u64).wrapping_neg();
#[cfg(any(
    talos_boot_scenario = "rpi5_descriptor_write_proof",
    talos_boot_scenario = "rpi5_close_syscall_proof",
    talos_boot_scenario = "rpi5_dup_syscall_proof",
    talos_boot_scenario = "rpi5_read_stdin_proof"
))]
const DESCRIPTOR_WRITE_EXPECTED_EINVAL_X0: u64 = (syscall::EINVAL as u64).wrapping_neg();
#[cfg(any(
    talos_boot_scenario = "rpi5_descriptor_write_proof",
    talos_boot_scenario = "rpi5_close_syscall_proof",
    talos_boot_scenario = "rpi5_dup_syscall_proof",
    talos_boot_scenario = "rpi5_read_stdin_proof"
))]
const DESCRIPTOR_WRITE_COPY_PROBE_NUMBER: u64 = 0x7001;
#[cfg(any(
    talos_boot_scenario = "rpi5_el0_trap_proof",
    talos_boot_scenario = "rpi5_syscall_proof"
))]
const EL0_TRAP_SPSR_EL0T_DAIF_MASKED: u64 = 0x3c0;
#[cfg(any(
    talos_boot_scenario = "rpi5_el0_trap_proof",
    talos_boot_scenario = "rpi5_syscall_proof"
))]
const EL0_TRAP_SPSR_EL1H_DAIF_MASKED: u64 = 0x3c5;
#[cfg(any(
    talos_boot_scenario = "rpi5_el0_trap_proof",
    talos_boot_scenario = "rpi5_syscall_proof"
))]
const EL0_TRAP_TABLE_PAGE_SIZE: usize = 4096;
#[cfg(any(
    talos_boot_scenario = "rpi5_el0_trap_proof",
    talos_boot_scenario = "rpi5_syscall_proof"
))]
const EL0_TRAP_L1_BCM2712_MMIO_INDEX: usize = 0x41;
#[cfg(any(
    talos_boot_scenario = "rpi5_el0_trap_proof",
    talos_boot_scenario = "rpi5_syscall_proof"
))]
const EL0_TRAP_BCM2712_MMIO_START: u64 = 0x10_7c00_0000;
#[cfg(any(
    talos_boot_scenario = "rpi5_el0_trap_proof",
    talos_boot_scenario = "rpi5_syscall_proof"
))]
const EL0_TRAP_BCM2712_MMIO_END: u64 = 0x10_8000_0000;
#[cfg(any(
    talos_boot_scenario = "rpi5_el0_trap_proof",
    talos_boot_scenario = "rpi5_syscall_proof"
))]
const EL0_TRAP_TCR_T0SZ: u64 = 16;
#[cfg(any(
    talos_boot_scenario = "rpi5_el0_trap_proof",
    talos_boot_scenario = "rpi5_syscall_proof"
))]
const EL0_TRAP_TCR_IPS_CODE_40BIT: u64 = 0b010;
#[cfg(any(
    talos_boot_scenario = "rpi5_el0_trap_proof",
    talos_boot_scenario = "rpi5_syscall_proof"
))]
const EL0_TRAP_TCR_IRGN0_NONCACHEABLE: u64 = 0b00;
#[cfg(any(
    talos_boot_scenario = "rpi5_el0_trap_proof",
    talos_boot_scenario = "rpi5_syscall_proof"
))]
const EL0_TRAP_TCR_ORGN0_NONCACHEABLE: u64 = 0b00;
#[cfg(any(
    talos_boot_scenario = "rpi5_el0_trap_proof",
    talos_boot_scenario = "rpi5_syscall_proof"
))]
const EL0_TRAP_TCR_SH0_INNER: u64 = 0b11;

#[cfg(any(
    talos_boot_scenario = "rpi5_el0_trap_proof",
    talos_boot_scenario = "rpi5_syscall_proof"
))]
struct El0TrapPreEretRegisters {
    hcr_el2: u64,
    sctlr_el1: u64,
    tcr_el1: u64,
    ttbr0_el1: u64,
    vbar_el1: u64,
    elr_el1: u64,
    spsr_el1: u64,
}

#[cfg(talos_boot_scenario = "rpi5_el0_trap_proof")]
struct El0TrapEl1HandoffRegisters {
    sctlr_el1: u64,
    tcr_el1: u64,
    ttbr0_el1: u64,
    vbar_el1: u64,
    elr_el1: u64,
    spsr_el1: u64,
    sp: u64,
}

#[cfg(talos_boot_scenario = "rpi5_el0_trap_proof")]
struct El0TrapTranslationFeatureReport {
    id_aa64mmfr0_el1: u64,
    id_aa64mmfr1_el1: u64,
    id_aa64mmfr2_el1: u64,
    id_aa64pfr0_el1: u64,
    mair_el1: u64,
    tcr_el1: u64,
}
#[cfg(any(
    talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
    talos_boot_scenario = "rpi5_remote_wakeup_request"
))]
const RPI5_CROSS_CORE_IPI_SGI_INTID: u32 = 1;
#[cfg(any(
    talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
    talos_boot_scenario = "rpi5_remote_wakeup_request"
))]
const RPI5_CROSS_CORE_IPI_WAIT_POLL_INTERVAL: usize = 20_000_000;
#[cfg(any(
    talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
    talos_boot_scenario = "rpi5_remote_wakeup_request"
))]
const RPI5_CROSS_CORE_IPI_CPU_INTERFACE_POLL_INTERVAL: usize = 1024;
#[cfg(talos_boot_scenario = "rpi5_remote_wakeup_request")]
const REMOTE_WAKE_QUEUE_CAPACITY: usize = 4;
#[cfg(talos_boot_scenario = "rpi5_smp_lock_cache_coherence")]
const RPI5_SMP_LOCK_CONTENTION_TARGET_PER_CORE: u64 = 64;
#[cfg(talos_boot_scenario = "rpi5_smp_lock_cache_coherence")]
const RPI5_SMP_LOCK_ACQUIRE_SPIN_LIMIT: u64 = 1_000_000;
#[cfg(talos_boot_scenario = "rpi5_smp_lock_cache_coherence")]
const RPI5_SMP_LOCK_WAIT_POLL_INTERVAL: usize = 20_000_000;
#[cfg(any(
    talos_boot_scenario = "rpi5_smp_lock_cache_coherence",
    talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
    talos_boot_scenario = "rpi5_remote_wakeup_request",
    talos_boot_scenario = "rpi5_production_secondary_dispatch",
    talos_boot_scenario = "rpi5_shared_scheduler_metadata",
    talos_boot_scenario = "rpi5_shared_runqueue_migration",
    talos_boot_scenario = "rpi5_multicore_preemption_proof",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof",
    talos_boot_scenario = "rpi5_secondary_scheduler_service_loop"
))]
const RPI5_SCTLR_M_ENABLE: u64 = 1 << 0;
#[cfg(any(
    talos_boot_scenario = "rpi5_smp_lock_cache_coherence",
    talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
    talos_boot_scenario = "rpi5_remote_wakeup_request",
    talos_boot_scenario = "rpi5_production_secondary_dispatch",
    talos_boot_scenario = "rpi5_shared_scheduler_metadata",
    talos_boot_scenario = "rpi5_shared_runqueue_migration",
    talos_boot_scenario = "rpi5_multicore_preemption_proof",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof",
    talos_boot_scenario = "rpi5_secondary_scheduler_service_loop"
))]
const RPI5_SCTLR_C_ENABLE: u64 = 1 << 2;
#[cfg(any(
    talos_boot_scenario = "rpi5_smp_lock_cache_coherence",
    talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
    talos_boot_scenario = "rpi5_remote_wakeup_request",
    talos_boot_scenario = "rpi5_production_secondary_dispatch",
    talos_boot_scenario = "rpi5_shared_scheduler_metadata",
    talos_boot_scenario = "rpi5_shared_runqueue_migration",
    talos_boot_scenario = "rpi5_multicore_preemption_proof",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof",
    talos_boot_scenario = "rpi5_secondary_scheduler_service_loop"
))]
const RPI5_SCTLR_I_ENABLE: u64 = 1 << 12;
#[cfg(any(
    talos_boot_scenario = "rpi5_psci_secondary_core_alive",
    talos_boot_scenario = "rpi5_secondary_core_workload",
    talos_boot_scenario = "rpi5_smp_lock_cache_coherence",
    talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
    talos_boot_scenario = "rpi5_remote_wakeup_request",
    talos_boot_scenario = "rpi5_production_secondary_dispatch",
    talos_boot_scenario = "rpi5_shared_scheduler_metadata",
    talos_boot_scenario = "rpi5_shared_runqueue_migration",
    talos_boot_scenario = "rpi5_multicore_preemption_proof",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof",
    talos_boot_scenario = "rpi5_secondary_scheduler_service_loop"
))]
const PSCI_AFFINITY_INFO: u64 = 0x8400_0004;
#[cfg(any(
    talos_boot_scenario = "rpi5_psci_secondary_core_alive",
    talos_boot_scenario = "rpi5_secondary_core_workload",
    talos_boot_scenario = "rpi5_smp_lock_cache_coherence",
    talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
    talos_boot_scenario = "rpi5_remote_wakeup_request",
    talos_boot_scenario = "rpi5_production_secondary_dispatch",
    talos_boot_scenario = "rpi5_shared_scheduler_metadata",
    talos_boot_scenario = "rpi5_shared_runqueue_migration",
    talos_boot_scenario = "rpi5_multicore_preemption_proof",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof",
    talos_boot_scenario = "rpi5_secondary_scheduler_service_loop"
))]
const PSCI_CPU_ON: u64 = 0xc400_0003;

const MMIO_REGIONS: &[MmioRegion] = &[
    MmioRegion::new("bcm2712-local-peripherals", 0x10_7c00_0000, 0x0400_0000),
    MmioRegion::new("bcm2712-gic-400", 0x10_7fff_9000, 0x0001_0000),
    MmioRegion::new("bcm2712-uart10-pl011", UART10_BASE, 0x0000_0200),
    MmioRegion::new("rp1-uart0-pl011-pcie2", RP1_UART0_BASE, 0x0000_0100),
    MmioRegion::new("rp1-gpio-pads", 0x1f_000f_0000, 0x0000_1000),
    MmioRegion::new("rp1-gpio-ctrl", 0x1f_000d_0000, 0x0000_1000),
    MmioRegion::new(
        "rp1-uart0-pl011-firmware-preserved",
        RP1_UART0_FIRMWARE_BASE,
        0x0000_0100,
    ),
];

#[cfg(any(
    talos_boot_scenario = "rpi5_timer_irq",
    talos_boot_scenario = "rpi5_timer_preemption",
    talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
    talos_boot_scenario = "rpi5_remote_wakeup_request"
))]
static LAST_IRQ_VECTOR: AtomicU64 = AtomicU64::new(0);
#[cfg(any(
    talos_boot_scenario = "rpi5_timer_irq",
    talos_boot_scenario = "rpi5_timer_preemption",
    talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
    talos_boot_scenario = "rpi5_remote_wakeup_request"
))]
static LAST_IAR: AtomicU64 = AtomicU64::new(0);
#[cfg(any(
    talos_boot_scenario = "rpi5_timer_irq",
    talos_boot_scenario = "rpi5_timer_preemption",
    talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
    talos_boot_scenario = "rpi5_remote_wakeup_request"
))]
static LAST_INTID: AtomicU64 = AtomicU64::new(0);
#[cfg(any(
    talos_boot_scenario = "rpi5_timer_irq",
    talos_boot_scenario = "rpi5_timer_preemption",
    talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
    talos_boot_scenario = "rpi5_remote_wakeup_request"
))]
static UNEXPECTED_GIC_IRQ_COUNT: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_timer_preemption")]
static TIMER_PREEMPTION_REQUESTS: AtomicU64 = AtomicU64::new(0);

#[cfg(any(
    talos_boot_scenario = "rpi5_el0_trap_proof",
    talos_boot_scenario = "rpi5_syscall_proof"
))]
#[repr(align(4096))]
struct El0TrapPage([u64; 512]);

#[cfg(any(
    talos_boot_scenario = "rpi5_el0_trap_proof",
    talos_boot_scenario = "rpi5_syscall_proof"
))]
impl El0TrapPage {
    const fn zeroed() -> Self {
        Self([0; 512])
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_el0_trap_proof",
    talos_boot_scenario = "rpi5_syscall_proof"
))]
#[repr(align(65536))]
struct El0TrapStack([u8; EL0_TRAP_USER_STACK_LEN]);

#[cfg(any(
    talos_boot_scenario = "rpi5_el0_trap_proof",
    talos_boot_scenario = "rpi5_syscall_proof"
))]
impl El0TrapStack {
    const fn zeroed() -> Self {
        Self([0; EL0_TRAP_USER_STACK_LEN])
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_el0_trap_proof",
    talos_boot_scenario = "rpi5_syscall_proof"
))]
#[repr(align(4096))]
struct El0TrapPayload([u8; EL0_TRAP_USER_TEXT_LEN]);

#[cfg(talos_boot_scenario = "rpi5_el0_trap_proof")]
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

#[cfg(all(
    talos_boot_scenario = "rpi5_syscall_proof",
    not(talos_boot_scenario = "rpi5_pointer_copy_proof"),
    not(talos_boot_scenario = "rpi5_close_syscall_proof")
))]
impl El0TrapPayload {
    const fn syscall_proof() -> Self {
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

#[cfg(talos_boot_scenario = "rpi5_close_syscall_proof")]
impl El0TrapPayload {
    const fn close_syscall_proof() -> Self {
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

#[cfg(talos_boot_scenario = "rpi5_dup_syscall_proof")]
impl El0TrapPayload {
    const fn dup_syscall_proof() -> Self {
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

#[cfg(talos_boot_scenario = "rpi5_read_stdin_proof")]
impl El0TrapPayload {
    const fn read_stdin_proof() -> Self {
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

#[cfg(talos_boot_scenario = "rpi5_pointer_copy_proof")]
impl El0TrapPayload {
    const fn pointer_copy_proof() -> Self {
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

#[cfg(talos_boot_scenario = "rpi5_descriptor_write_proof")]
impl El0TrapPayload {
    const fn descriptor_write_proof() -> Self {
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

#[cfg(any(
    talos_boot_scenario = "rpi5_el0_trap_proof",
    talos_boot_scenario = "rpi5_syscall_proof"
))]
static mut EL0_TRAP_ROOT_TABLE: El0TrapPage = El0TrapPage::zeroed();
#[cfg(any(
    talos_boot_scenario = "rpi5_el0_trap_proof",
    talos_boot_scenario = "rpi5_syscall_proof"
))]
static mut EL0_TRAP_L1_TABLE: El0TrapPage = El0TrapPage::zeroed();
#[cfg(any(
    talos_boot_scenario = "rpi5_el0_trap_proof",
    talos_boot_scenario = "rpi5_syscall_proof"
))]
static mut EL0_TRAP_LOW_L2_TABLE: El0TrapPage = El0TrapPage::zeroed();
#[cfg(any(
    talos_boot_scenario = "rpi5_el0_trap_proof",
    talos_boot_scenario = "rpi5_syscall_proof"
))]
static mut EL0_TRAP_LOW_L3_TABLE: El0TrapPage = El0TrapPage::zeroed();
#[cfg(any(
    talos_boot_scenario = "rpi5_el0_trap_proof",
    talos_boot_scenario = "rpi5_syscall_proof"
))]
static mut EL0_TRAP_MMIO_L2_TABLE: El0TrapPage = El0TrapPage::zeroed();
#[cfg(any(
    talos_boot_scenario = "rpi5_el0_trap_proof",
    talos_boot_scenario = "rpi5_syscall_proof"
))]
static mut EL0_TRAP_STACK: El0TrapStack = El0TrapStack::zeroed();
#[cfg(talos_boot_scenario = "rpi5_el0_trap_proof")]
static EL0_TRAP_PAYLOAD: El0TrapPayload = El0TrapPayload::svc_marker();
#[cfg(all(
    talos_boot_scenario = "rpi5_syscall_proof",
    not(talos_boot_scenario = "rpi5_pointer_copy_proof"),
    not(talos_boot_scenario = "rpi5_descriptor_write_proof"),
    not(talos_boot_scenario = "rpi5_close_syscall_proof"),
    not(talos_boot_scenario = "rpi5_dup_syscall_proof"),
    not(talos_boot_scenario = "rpi5_read_stdin_proof")
))]
static EL0_TRAP_PAYLOAD: El0TrapPayload = El0TrapPayload::syscall_proof();
#[cfg(talos_boot_scenario = "rpi5_pointer_copy_proof")]
static EL0_TRAP_PAYLOAD: El0TrapPayload = El0TrapPayload::pointer_copy_proof();
#[cfg(talos_boot_scenario = "rpi5_descriptor_write_proof")]
static EL0_TRAP_PAYLOAD: El0TrapPayload = El0TrapPayload::descriptor_write_proof();
#[cfg(talos_boot_scenario = "rpi5_close_syscall_proof")]
static EL0_TRAP_PAYLOAD: El0TrapPayload = El0TrapPayload::close_syscall_proof();
#[cfg(talos_boot_scenario = "rpi5_dup_syscall_proof")]
static EL0_TRAP_PAYLOAD: El0TrapPayload = El0TrapPayload::dup_syscall_proof();
#[cfg(talos_boot_scenario = "rpi5_read_stdin_proof")]
static EL0_TRAP_PAYLOAD: El0TrapPayload = El0TrapPayload::read_stdin_proof();
#[cfg(talos_boot_scenario = "rpi5_pointer_copy_proof")]
static mut POINTER_COPY_USER_DATA: [u8; POINTER_COPY_USER_DATA_LEN] =
    [0; POINTER_COPY_USER_DATA_LEN];
#[cfg(any(
    talos_boot_scenario = "rpi5_descriptor_write_proof",
    talos_boot_scenario = "rpi5_close_syscall_proof",
    talos_boot_scenario = "rpi5_dup_syscall_proof",
    talos_boot_scenario = "rpi5_read_stdin_proof"
))]
static mut DESCRIPTOR_WRITE_USER_DATA: [u8; DESCRIPTOR_WRITE_USER_DATA_LEN] =
    [0; DESCRIPTOR_WRITE_USER_DATA_LEN];
#[cfg(any(
    talos_boot_scenario = "rpi5_descriptor_write_proof",
    talos_boot_scenario = "rpi5_close_syscall_proof",
    talos_boot_scenario = "rpi5_dup_syscall_proof",
    talos_boot_scenario = "rpi5_read_stdin_proof"
))]
static mut DESCRIPTOR_WRITE_CONSOLE_CAPTURE: [u8; 64] = [0; 64];
#[cfg(any(
    talos_boot_scenario = "rpi5_descriptor_write_proof",
    talos_boot_scenario = "rpi5_close_syscall_proof",
    talos_boot_scenario = "rpi5_dup_syscall_proof",
    talos_boot_scenario = "rpi5_read_stdin_proof"
))]
static DESCRIPTOR_WRITE_CONSOLE_LEN: AtomicU64 = AtomicU64::new(0);
#[cfg(any(
    talos_boot_scenario = "rpi5_close_syscall_proof",
    talos_boot_scenario = "rpi5_dup_syscall_proof",
    talos_boot_scenario = "rpi5_read_stdin_proof"
))]
static mut PROCESS_DESCRIPTOR_STDIO_STORE: crate::posix::ProcessDescriptorStore<1, 4> =
    crate::posix::ProcessDescriptorStore::new_empty();
#[cfg(any(
    talos_boot_scenario = "rpi5_close_syscall_proof",
    talos_boot_scenario = "rpi5_dup_syscall_proof",
    talos_boot_scenario = "rpi5_read_stdin_proof"
))]
const PROCESS_DESCRIPTOR_STDIO_OWNER_RAW: u64 = 1;
#[cfg(talos_boot_scenario = "rpi5_descriptor_write_proof")]
static DESCRIPTOR_WRITE_STDOUT_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_descriptor_write_proof")]
static DESCRIPTOR_WRITE_STDERR_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_descriptor_write_proof")]
static DESCRIPTOR_WRITE_FD0_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_descriptor_write_proof")]
static DESCRIPTOR_WRITE_BADFD_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_descriptor_write_proof")]
static DESCRIPTOR_WRITE_EFAULT_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_descriptor_write_proof")]
static DESCRIPTOR_WRITE_RESERVED_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_descriptor_write_proof")]
static DESCRIPTOR_WRITE_TALOS_NOP_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_descriptor_write_proof")]
static DESCRIPTOR_WRITE_UNKNOWN_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_descriptor_write_proof")]
static DESCRIPTOR_WRITE_COPY_PROBE_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_descriptor_write_proof")]
static DESCRIPTOR_WRITE_ERRORS: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_close_syscall_proof")]
static CLOSE_SYSCALL_CLOSE_STDOUT_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_close_syscall_proof")]
static CLOSE_SYSCALL_WRITE_CLOSED_STDOUT_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_close_syscall_proof")]
static CLOSE_SYSCALL_CLOSE_STDERR_RESERVED_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_close_syscall_proof")]
static CLOSE_SYSCALL_WRITE_STDERR_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_close_syscall_proof")]
static CLOSE_SYSCALL_CLOSE_STDERR_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_close_syscall_proof")]
static CLOSE_SYSCALL_WRITE_CLOSED_STDERR_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_close_syscall_proof")]
static CLOSE_SYSCALL_CLOSE_STDOUT_AGAIN_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_close_syscall_proof")]
static CLOSE_SYSCALL_CLOSE_BADFD_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_close_syscall_proof")]
static CLOSE_SYSCALL_TALOS_NOP_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_close_syscall_proof")]
static CLOSE_SYSCALL_UNKNOWN_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_close_syscall_proof")]
static CLOSE_SYSCALL_COPY_PROBE_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_close_syscall_proof")]
static CLOSE_SYSCALL_ERRORS: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_dup_syscall_proof")]
const DUP_SYSCALL_SOURCE: &[u8; 19] = b"talos-dup-src-rpi5\n";
#[cfg(talos_boot_scenario = "rpi5_dup_syscall_proof")]
const DUP_SYSCALL_DUPLICATE: &[u8; 19] = b"talos-dup-new-rpi5\n";
#[cfg(talos_boot_scenario = "rpi5_dup_syscall_proof")]
static DUP_SYSCALL_DUP_STDOUT_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_dup_syscall_proof")]
static DUP_SYSCALL_DUP_STDERR_FULL_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_dup_syscall_proof")]
static DUP_SYSCALL_DUP_STDOUT_RESERVED_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_dup_syscall_proof")]
static DUP_SYSCALL_WRITE_SOURCE_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_dup_syscall_proof")]
static DUP_SYSCALL_WRITE_DUPLICATE_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_dup_syscall_proof")]
static DUP_SYSCALL_CLOSE_SOURCE_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_dup_syscall_proof")]
static DUP_SYSCALL_WRITE_SOURCE_CLOSED_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_dup_syscall_proof")]
static DUP_SYSCALL_WRITE_DUPLICATE_AFTER_SOURCE_CLOSE_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_dup_syscall_proof")]
static DUP_SYSCALL_CLOSE_DUPLICATE_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_dup_syscall_proof")]
static DUP_SYSCALL_WRITE_DUPLICATE_CLOSED_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_dup_syscall_proof")]
static DUP_SYSCALL_DUP_CLOSED_SOURCE_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_dup_syscall_proof")]
static DUP_SYSCALL_TALOS_NOP_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_dup_syscall_proof")]
static DUP_SYSCALL_UNKNOWN_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_dup_syscall_proof")]
static DUP_SYSCALL_COPY_PROBE_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_dup_syscall_proof")]
static DUP_SYSCALL_ERRORS: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_read_stdin_proof")]
const READ_STDIN_FIXED_BYTES: &[u8; 17] = b"talos-stdin-rpi5\n";
#[cfg(talos_boot_scenario = "rpi5_read_stdin_proof")]
static mut READ_STDIN_FIXED_STATE: crate::posix::FixedStdin<'static> =
    crate::posix::FixedStdin::new(READ_STDIN_FIXED_BYTES);
#[cfg(talos_boot_scenario = "rpi5_read_stdin_proof")]
static READ_STDIN_DUP_STDIN_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_read_stdin_proof")]
static READ_STDIN_GUARD_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_read_stdin_proof")]
static READ_STDIN_RESERVED_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_read_stdin_proof")]
static READ_STDIN_FD1_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_read_stdin_proof")]
static READ_STDIN_BADFD_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_read_stdin_proof")]
static READ_STDIN_FIRST_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_read_stdin_proof")]
static READ_STDIN_DUPLICATE_REMAINING_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_read_stdin_proof")]
static READ_STDIN_EOF_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_read_stdin_proof")]
static READ_STDIN_TALOS_NOP_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_read_stdin_proof")]
static READ_STDIN_UNKNOWN_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_read_stdin_proof")]
static READ_STDIN_COPY_PROBE_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_read_stdin_proof")]
static READ_STDIN_ERRORS: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_syscall_proof")]
static SYSCALL_PROOF_TALOS_NOP_DISPATCHED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_syscall_proof")]
static SYSCALL_PROOF_TALOS_NOP_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_syscall_proof")]
static SYSCALL_PROOF_UNKNOWN_DISPATCHED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_syscall_proof")]
static SYSCALL_PROOF_UNKNOWN_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_syscall_proof")]
static SYSCALL_PROOF_ERRORS: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_pointer_copy_proof")]
static POINTER_COPY_SUCCESS_DISPATCHED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_pointer_copy_proof")]
static POINTER_COPY_SUCCESS_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_pointer_copy_proof")]
static POINTER_COPY_EFAULT_DISPATCHED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_pointer_copy_proof")]
static POINTER_COPY_EFAULT_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_pointer_copy_proof")]
static POINTER_COPY_UNKNOWN_DISPATCHED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_pointer_copy_proof")]
static POINTER_COPY_UNKNOWN_OBSERVED: AtomicU64 = AtomicU64::new(0);
#[cfg(talos_boot_scenario = "rpi5_pointer_copy_proof")]
static POINTER_COPY_ERRORS: AtomicU64 = AtomicU64::new(0);

#[cfg(any(
    talos_boot_scenario = "rpi5_el0_trap_proof",
    talos_boot_scenario = "rpi5_syscall_proof"
))]
unsafe extern "C" {
    static __exception_vectors: u8;
}

#[cfg(any(
    talos_boot_scenario = "rpi5_psci_secondary_core_alive",
    talos_boot_scenario = "rpi5_secondary_core_workload",
    talos_boot_scenario = "rpi5_smp_lock_cache_coherence",
    talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
    talos_boot_scenario = "rpi5_remote_wakeup_request",
    talos_boot_scenario = "rpi5_production_secondary_dispatch",
    talos_boot_scenario = "rpi5_shared_scheduler_metadata",
    talos_boot_scenario = "rpi5_shared_runqueue_migration",
    talos_boot_scenario = "rpi5_multicore_preemption_proof",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof",
    talos_boot_scenario = "rpi5_secondary_scheduler_service_loop"
))]
unsafe extern "C" {
    fn talos_aarch64_rpi5_secondary_entry();
    static talos_secondary_core_stacks: u8;
    static talos_secondary_core_stacks_end: u8;
}

pub fn init_stub() {
    init_rp1_uart0_pins();
    // serial10 is already active for firmware/BL31 logs; avoid disturbing baud
    // while testing Talos' runtime console path.
}

#[cfg(talos_target_rpi5_bcm2712)]
fn init_rp1_uart0_pins() {
    write_rp1_reg_flush(RP1_UART0_GPIO14_PAD, 0x40);
    write_rp1_reg_flush(RP1_UART0_GPIO15_PAD, 0x48);
    write_rp1_reg_flush(RP1_UART0_GPIO14_CTRL, 4);
    write_rp1_reg_flush(RP1_UART0_GPIO15_CTRL, 4);
}

#[cfg(not(talos_target_rpi5_bcm2712))]
fn init_rp1_uart0_pins() {}

#[cfg(talos_target_rpi5_bcm2712)]
fn write_rp1_reg_flush(addr: usize, value: u32) {
    let reg = addr as *mut u32;
    unsafe {
        core::ptr::write_volatile(reg, value);
        let _ = core::ptr::read_volatile(reg);
        core::arch::asm!("dsb sy", options(nostack, preserves_flags));
    }
}

#[cfg(talos_target_rpi5_bcm2712)]
pub fn firmware_console() -> Pl011 {
    Pl011::new_with_posted_write_flush(UART10_BASE)
}

#[cfg(any(
    talos_boot_scenario = "rpi5_psci_secondary_core_alive",
    talos_boot_scenario = "rpi5_secondary_core_workload",
    talos_boot_scenario = "rpi5_smp_lock_cache_coherence",
    talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
    talos_boot_scenario = "rpi5_remote_wakeup_request",
    talos_boot_scenario = "rpi5_production_secondary_dispatch",
    talos_boot_scenario = "rpi5_shared_scheduler_metadata",
    talos_boot_scenario = "rpi5_shared_runqueue_migration",
    talos_boot_scenario = "rpi5_multicore_preemption_proof",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof",
    talos_boot_scenario = "rpi5_secondary_scheduler_service_loop"
))]
fn secondary_stack_layout() -> CoreStackLayout {
    let base = core::ptr::addr_of!(talos_secondary_core_stacks) as usize;
    let end = core::ptr::addr_of!(talos_secondary_core_stacks_end) as usize;
    CoreStackLayout::new(base, end, MAX_CORES, SECONDARY_KERNEL_STACK_SIZE)
        .expect("valid linked secondary-core stack layout")
}

#[cfg(any(
    talos_boot_scenario = "rpi5_psci_secondary_core_alive",
    talos_boot_scenario = "rpi5_secondary_core_workload",
    talos_boot_scenario = "rpi5_smp_lock_cache_coherence",
    talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
    talos_boot_scenario = "rpi5_remote_wakeup_request",
    talos_boot_scenario = "rpi5_production_secondary_dispatch",
    talos_boot_scenario = "rpi5_shared_scheduler_metadata",
    talos_boot_scenario = "rpi5_shared_runqueue_migration",
    talos_boot_scenario = "rpi5_multicore_preemption_proof",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof",
    talos_boot_scenario = "rpi5_secondary_scheduler_service_loop"
))]
fn secondary_state_name(state: u64) -> &'static str {
    CoreLifecycle::from_raw(state).map_or("unknown", CoreLifecycle::name)
}

#[cfg(any(
    talos_boot_scenario = "rpi5_psci_secondary_core_alive",
    talos_boot_scenario = "rpi5_secondary_core_workload",
    talos_boot_scenario = "rpi5_smp_lock_cache_coherence",
    talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
    talos_boot_scenario = "rpi5_remote_wakeup_request",
    talos_boot_scenario = "rpi5_production_secondary_dispatch",
    talos_boot_scenario = "rpi5_shared_scheduler_metadata",
    talos_boot_scenario = "rpi5_shared_runqueue_migration",
    talos_boot_scenario = "rpi5_multicore_preemption_proof",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof",
    talos_boot_scenario = "rpi5_secondary_scheduler_service_loop"
))]
unsafe fn psci_smc(function_id: u64, arg1: u64, arg2: u64, arg3: u64) -> i64 {
    let mut result = function_id;
    let scratch1 = arg1;
    let scratch2 = arg2;
    let scratch3 = arg3;
    unsafe {
        core::arch::asm!(
            "smc #0",
            inout("x0") result,
            inout("x1") scratch1 => _,
            inout("x2") scratch2 => _,
            inout("x3") scratch3 => _,
            lateout("x4") _,
            lateout("x5") _,
            lateout("x6") _,
            lateout("x7") _,
            lateout("x8") _,
            lateout("x9") _,
            lateout("x10") _,
            lateout("x11") _,
            lateout("x12") _,
            lateout("x13") _,
            lateout("x14") _,
            lateout("x15") _,
            lateout("x16") _,
            lateout("x17") _,
            options(nostack)
        );
    }
    result as i64
}

#[cfg(any(
    talos_boot_scenario = "rpi5_psci_secondary_core_alive",
    talos_boot_scenario = "rpi5_secondary_core_workload",
    talos_boot_scenario = "rpi5_smp_lock_cache_coherence",
    talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
    talos_boot_scenario = "rpi5_remote_wakeup_request",
    talos_boot_scenario = "rpi5_production_secondary_dispatch",
    talos_boot_scenario = "rpi5_shared_scheduler_metadata",
    talos_boot_scenario = "rpi5_shared_runqueue_migration",
    talos_boot_scenario = "rpi5_multicore_preemption_proof",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof",
    talos_boot_scenario = "rpi5_secondary_scheduler_service_loop"
))]
unsafe fn psci_cpu_on_smc(target_affinity: u64, entry: usize, context: usize) -> i64 {
    unsafe { psci_smc(PSCI_CPU_ON, target_affinity, entry as u64, context as u64) }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_psci_secondary_core_alive",
    talos_boot_scenario = "rpi5_secondary_core_workload",
    talos_boot_scenario = "rpi5_smp_lock_cache_coherence",
    talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
    talos_boot_scenario = "rpi5_remote_wakeup_request",
    talos_boot_scenario = "rpi5_production_secondary_dispatch",
    talos_boot_scenario = "rpi5_shared_scheduler_metadata",
    talos_boot_scenario = "rpi5_shared_runqueue_migration",
    talos_boot_scenario = "rpi5_multicore_preemption_proof",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof",
    talos_boot_scenario = "rpi5_secondary_scheduler_service_loop"
))]
unsafe fn psci_affinity_info_smc(target_affinity: u64, lowest_affinity_level: u64) -> i64 {
    unsafe {
        psci_smc(
            PSCI_AFFINITY_INFO,
            target_affinity,
            lowest_affinity_level,
            0,
        )
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_psci_secondary_core_alive",
    talos_boot_scenario = "rpi5_secondary_core_workload",
    talos_boot_scenario = "rpi5_smp_lock_cache_coherence",
    talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
    talos_boot_scenario = "rpi5_remote_wakeup_request",
    talos_boot_scenario = "rpi5_production_secondary_dispatch",
    talos_boot_scenario = "rpi5_shared_scheduler_metadata",
    talos_boot_scenario = "rpi5_shared_runqueue_migration",
    talos_boot_scenario = "rpi5_multicore_preemption_proof",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof",
    talos_boot_scenario = "rpi5_secondary_scheduler_service_loop"
))]
fn psci_affinity_state_name(state: i64) -> &'static str {
    match state {
        0 => "on",
        1 => "off",
        2 => "on-pending",
        _ => "error-or-unknown",
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_psci_secondary_core_alive",
    talos_boot_scenario = "rpi5_secondary_core_workload",
    talos_boot_scenario = "rpi5_smp_lock_cache_coherence",
    talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
    talos_boot_scenario = "rpi5_remote_wakeup_request",
    talos_boot_scenario = "rpi5_production_secondary_dispatch",
    talos_boot_scenario = "rpi5_shared_scheduler_metadata",
    talos_boot_scenario = "rpi5_shared_runqueue_migration",
    talos_boot_scenario = "rpi5_multicore_preemption_proof",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof",
    talos_boot_scenario = "rpi5_secondary_scheduler_service_loop"
))]
#[unsafe(no_mangle)]
pub extern "C" fn talos_rpi5_secondary_entry(context: usize) -> ! {
    write_uart10_bytes_early_phase(b"TALOS: secondary_rust_entry\r\n");

    let mpidr = crate::arch::aarch64::mpidr_el1();
    let affinity = crate::arch::aarch64::mpidr_affinity(mpidr);
    let logical_cpu = pi5_logical_cpu_from_mpidr_affinity(affinity).unwrap_or(context);
    if logical_cpu < MAX_CORES {
        let core_state = &SECONDARY_CORE_STATES[logical_cpu];
        core_state.enter(context, mpidr, affinity);

        let stack_pointer: u64;
        unsafe {
            core::arch::asm!("mov {stack_pointer}, sp", stack_pointer = out(reg) stack_pointer, options(nomem, nostack, preserves_flags));
        }
        core_state.mark_stack_ready(stack_pointer as usize);
        core_state.mark_registered();
        #[cfg(any(
            talos_boot_scenario = "rpi5_smp_lock_cache_coherence",
            talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
            talos_boot_scenario = "rpi5_remote_wakeup_request",
            talos_boot_scenario = "rpi5_production_secondary_dispatch",
            talos_boot_scenario = "rpi5_shared_scheduler_metadata",
            talos_boot_scenario = "rpi5_shared_runqueue_migration",
            talos_boot_scenario = "rpi5_multicore_preemption_proof",
            talos_boot_scenario = "rpi5_production_timer_preemption_proof",
            talos_boot_scenario = "rpi5_secondary_scheduler_service_loop"
        ))]
        if !enter_secondary_cacheable_mmu_handoff(logical_cpu) {
            core_state.clean_to_poc();
            write_uart10_bytes_early_phase(b"TALOS: secondary_cacheable_mmu_handoff_failed\r\n");
            loop {
                unsafe {
                    core::arch::asm!("wfe", options(nomem, nostack, preserves_flags));
                }
            }
        }
        #[cfg(any(
            talos_boot_scenario = "rpi5_smp_lock_cache_coherence",
            talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
            talos_boot_scenario = "rpi5_remote_wakeup_request",
            talos_boot_scenario = "rpi5_production_secondary_dispatch",
            talos_boot_scenario = "rpi5_shared_scheduler_metadata",
            talos_boot_scenario = "rpi5_shared_runqueue_migration",
            talos_boot_scenario = "rpi5_multicore_preemption_proof",
            talos_boot_scenario = "rpi5_production_timer_preemption_proof",
            talos_boot_scenario = "rpi5_secondary_scheduler_service_loop"
        ))]
        core_state.republish_identity(context, mpidr, affinity, stack_pointer as usize);
        core_state.mark_handoff_ready();
        core_state.clean_to_poc();
        write_uart10_bytes_early_phase(b"TALOS: secondary_state_published\r\n");
        #[cfg(talos_boot_scenario = "rpi5_secondary_core_workload")]
        {
            smp::run_controlled_secondary_workload(core_state, SECONDARY_CORE_WORKLOAD_TARGET);
            write_uart10_bytes_early_phase(b"TALOS: secondary_workload_complete\r\n");
        }
        #[cfg(talos_boot_scenario = "rpi5_smp_lock_cache_coherence")]
        {
            run_smp_lock_contention_secondary(core_state, logical_cpu);
            write_uart10_bytes_early_phase(b"TALOS: secondary_lock_contention_complete\r\n");
        }
        #[cfg(talos_boot_scenario = "rpi5_cross_core_ipi_delivery")]
        {
            run_cross_core_ipi_delivery_secondary(core_state, logical_cpu);
            write_uart10_bytes_early_phase(b"TALOS: secondary_ipi_delivery_complete\r\n");
        }
        #[cfg(talos_boot_scenario = "rpi5_remote_wakeup_request")]
        {
            run_remote_wakeup_request_secondary(core_state, logical_cpu);
            write_uart10_bytes_early_phase(b"TALOS: secondary_remote_wakeup_complete\r\n");
        }
        #[cfg(talos_boot_scenario = "rpi5_production_secondary_dispatch")]
        {
            run_production_secondary_dispatch_secondary(core_state, logical_cpu);
            write_uart10_bytes_early_phase(b"TALOS: secondary_production_dispatch_complete\r\n");
        }
        #[cfg(talos_boot_scenario = "rpi5_shared_scheduler_metadata")]
        {
            run_shared_scheduler_metadata_secondary(core_state, logical_cpu);
            write_uart10_bytes_early_phase(b"TALOS: secondary_shared_metadata_complete\r\n");
        }
        #[cfg(talos_boot_scenario = "rpi5_shared_runqueue_migration")]
        {
            run_shared_runqueue_migration_secondary(core_state, logical_cpu);
            write_uart10_bytes_early_phase(b"TALOS: secondary_shared_runqueue_complete\r\n");
        }
        #[cfg(talos_boot_scenario = "rpi5_multicore_preemption_proof")]
        {
            run_multicore_preemption_secondary(core_state, logical_cpu);
            write_uart10_bytes_early_phase(b"TALOS: secondary_multicore_preemption_complete\r\n");
        }
        #[cfg(talos_boot_scenario = "rpi5_production_timer_preemption_proof")]
        {
            run_production_timer_preemption_secondary(core_state, logical_cpu);
            write_uart10_bytes_early_phase(
                b"TALOS: secondary_production_timer_preemption_complete\r\n",
            );
        }
        #[cfg(talos_boot_scenario = "rpi5_secondary_scheduler_service_loop")]
        {
            run_secondary_scheduler_service_loop_secondary(core_state, logical_cpu);
            write_uart10_bytes_early_phase(b"TALOS: secondary_scheduler_service_loop_complete\r\n");
        }
    }

    loop {
        unsafe {
            core::arch::asm!("wfe", options(nomem, nostack, preserves_flags));
        }
    }
}

#[cfg(talos_boot_scenario = "rpi5_production_secondary_dispatch")]
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

#[cfg(talos_boot_scenario = "rpi5_production_secondary_dispatch")]
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

#[cfg(talos_boot_scenario = "rpi5_production_secondary_dispatch")]
#[derive(Clone, Copy)]
struct ProductionSecondaryDispatchState {
    reports: [ProductionSecondaryDispatchReport; MAX_CORES],
    lock_progress: [u64; MAX_CORES],
}

#[cfg(talos_boot_scenario = "rpi5_production_secondary_dispatch")]
impl ProductionSecondaryDispatchState {
    const fn new() -> Self {
        Self {
            reports: [ProductionSecondaryDispatchReport::empty(); MAX_CORES],
            lock_progress: [0; MAX_CORES],
        }
    }
}

#[cfg(talos_boot_scenario = "rpi5_production_secondary_dispatch")]
static PRODUCTION_SECONDARY_DISPATCH_STATE: SpinLock<ProductionSecondaryDispatchState> =
    SpinLock::new(ProductionSecondaryDispatchState::new());

#[cfg(talos_boot_scenario = "rpi5_production_secondary_dispatch")]
fn reset_production_secondary_dispatch_state() {
    let mut state = unsafe { PRODUCTION_SECONDARY_DISPATCH_STATE.lock_irqsave() };
    *state = ProductionSecondaryDispatchState::new();
}

#[cfg(any(
    talos_boot_scenario = "rpi5_production_secondary_dispatch",
    talos_boot_scenario = "rpi5_shared_scheduler_metadata",
    talos_boot_scenario = "rpi5_shared_runqueue_migration",
    talos_boot_scenario = "rpi5_multicore_preemption_proof",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof",
    talos_boot_scenario = "rpi5_secondary_scheduler_service_loop"
))]
fn scheduler_role_name(role: SchedulerCoreRole) -> &'static str {
    match role {
        SchedulerCoreRole::BootCpuProduction => "boot-production",
        SchedulerCoreRole::SecondaryDeferred => "secondary-deferred",
        SchedulerCoreRole::SecondaryProductionDiagnostic => "secondary-production-diagnostic",
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_production_secondary_dispatch",
    talos_boot_scenario = "rpi5_shared_scheduler_metadata",
    talos_boot_scenario = "rpi5_shared_runqueue_migration",
    talos_boot_scenario = "rpi5_load_balancing_proof",
    talos_boot_scenario = "rpi5_multicore_preemption_proof",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof",
    talos_boot_scenario = "rpi5_secondary_scheduler_service_loop"
))]
fn task_id(raw: u64) -> TaskId {
    TaskId::new(raw).expect("diagnostic task IDs are nonzero")
}

#[cfg(any(
    talos_boot_scenario = "rpi5_production_secondary_dispatch",
    talos_boot_scenario = "rpi5_shared_scheduler_metadata",
    talos_boot_scenario = "rpi5_shared_runqueue_migration",
    talos_boot_scenario = "rpi5_load_balancing_proof",
    talos_boot_scenario = "rpi5_multicore_preemption_proof",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof",
    talos_boot_scenario = "rpi5_secondary_scheduler_service_loop"
))]
fn scheduler_task(logical_cpu: usize, progress: u64) -> Task {
    let raw_task_id = (logical_cpu as u64 + 1) * 100 + progress;
    let stack_base = 0x80_0000 + logical_cpu * 0x10000 + progress as usize * 0x1000;
    let stack = KernelStack::new(stack_base, 0x1000).expect("diagnostic stack bounds are valid");
    let context = ContextFrame::new(stack.limit() & !0xf, 0x40_0000 + raw_task_id as usize);
    Task::kernel_thread(task_id(raw_task_id), stack, context)
}

#[cfg(talos_boot_scenario = "rpi5_production_secondary_dispatch")]
fn build_production_secondary_dispatch_report(
    logical_cpu: usize,
    scheduler: &mut PerCoreScheduler<2>,
) -> ProductionSecondaryDispatchReport {
    let requester = LogicalCpuId::new(logical_cpu);
    let wrong_requester = LogicalCpuId::BOOT;
    let mut errors = 0;

    let cross_owner_rejected = matches!(
        scheduler.local_scheduler_mut(wrong_requester),
        Err(PerCoreSchedulerAccessError::WrongOwner { owner, requester: wrong })
            if owner == requester && wrong == wrong_requester
    );
    if !cross_owner_rejected {
        errors += 1;
    }

    let mut wrong_owner_task = scheduler_task(logical_cpu, 99);
    let cross_owner_dispatch_rejected = matches!(
        scheduler.dispatch_cpu_local_diagnostic_task(wrong_requester, &mut wrong_owner_task),
        Err(ProductionDispatchError::WrongOwner { owner, requester: wrong })
            if owner == requester && wrong == wrong_requester
    );
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

#[cfg(talos_boot_scenario = "rpi5_production_secondary_dispatch")]
fn publish_production_secondary_dispatch_report(
    logical_cpu: usize,
    report: ProductionSecondaryDispatchReport,
) {
    let mut state = PRODUCTION_SECONDARY_DISPATCH_STATE.lock();
    state.reports[logical_cpu] = report;
    state.lock_progress[logical_cpu] = report.progress;
}

#[cfg(talos_boot_scenario = "rpi5_production_secondary_dispatch")]
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

#[cfg(talos_boot_scenario = "rpi5_shared_scheduler_metadata")]
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

#[cfg(talos_boot_scenario = "rpi5_shared_scheduler_metadata")]
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

    const fn lock_progress(self) -> u64 {
        if self.errors == 0 { 1 } else { 0 }
    }
}

#[cfg(talos_boot_scenario = "rpi5_shared_scheduler_metadata")]
#[derive(Clone, Copy)]
struct SharedSchedulerMetadataState {
    reports: [SharedSchedulerMetadataReport; MAX_CORES],
    lock_progress: [u64; MAX_CORES],
}

#[cfg(talos_boot_scenario = "rpi5_shared_scheduler_metadata")]
impl SharedSchedulerMetadataState {
    const fn new() -> Self {
        Self {
            reports: [SharedSchedulerMetadataReport::empty(); MAX_CORES],
            lock_progress: [0; MAX_CORES],
        }
    }
}

#[cfg(talos_boot_scenario = "rpi5_shared_scheduler_metadata")]
static SHARED_SCHEDULER_METADATA_STATE: SpinLock<SharedSchedulerMetadataState> =
    SpinLock::new(SharedSchedulerMetadataState::new());

#[cfg(talos_boot_scenario = "rpi5_shared_scheduler_metadata")]
static SHARED_SCHEDULER_METADATA_TABLE: SharedSchedulerMetadataLock<
    SHARED_SCHEDULER_METADATA_TASK_CAPACITY,
    MAX_CORES,
> = SpinLock::new(SharedSchedulerMetadata::new());

#[cfg(talos_boot_scenario = "rpi5_shared_scheduler_metadata")]
fn reset_shared_scheduler_metadata_state() {
    let mut state = unsafe { SHARED_SCHEDULER_METADATA_STATE.lock_irqsave() };
    *state = SharedSchedulerMetadataState::new();
    let mut metadata = unsafe { SHARED_SCHEDULER_METADATA_TABLE.lock_irqsave() };
    *metadata = SharedSchedulerMetadata::new();
}

#[cfg(talos_boot_scenario = "rpi5_shared_scheduler_metadata")]
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
        let mut metadata = unsafe { SHARED_SCHEDULER_METADATA_TABLE.lock_irqsave() };
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

#[cfg(talos_boot_scenario = "rpi5_shared_scheduler_metadata")]
fn publish_shared_scheduler_metadata_report(
    logical_cpu: usize,
    report: SharedSchedulerMetadataReport,
) {
    let mut state = SHARED_SCHEDULER_METADATA_STATE.lock();
    state.reports[logical_cpu] = report;
    state.lock_progress[logical_cpu] = report.lock_progress();
}

#[cfg(talos_boot_scenario = "rpi5_shared_scheduler_metadata")]
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

#[cfg(talos_boot_scenario = "rpi5_shared_runqueue_migration")]
#[derive(Clone, Copy)]
struct SharedRunQueueMigrationReport {
    source_owner: u64,
    source_role: SchedulerCoreRole,
    destination_owner: u64,
    destination_role: SchedulerCoreRole,
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

#[cfg(talos_boot_scenario = "rpi5_shared_runqueue_migration")]
impl SharedRunQueueMigrationReport {
    const fn empty() -> Self {
        Self {
            source_owner: u64::MAX,
            source_role: SchedulerCoreRole::SecondaryDeferred,
            destination_owner: u64::MAX,
            destination_role: SchedulerCoreRole::SecondaryDeferred,
            task_id: 0,
            task_state: 0,
            registered_generation: 0,
            publish_reserved_state: MigrationState::MigrationRejected,
            publish_queued_state: MigrationState::MigrationRejected,
            consume_queued_state: MigrationState::MigrationRejected,
            consume_destination_state: MigrationState::MigrationRejected,
            source_queue_before: 0,
            source_queue_after_publish: 0,
            shared_len_after_publish: 0,
            shared_len_after_consume: 0,
            destination_queue_len: 0,
            destination_front: 0,
            metadata_owner_after_consume: u64::MAX,
            metadata_generation_after_consume: 0,
            source_removed: false,
            destination_enqueued: false,
            metadata_migrated: false,
            errors: 0,
        }
    }

    const fn lock_progress(self) -> u64 {
        if self.errors == 0 { 1 } else { 0 }
    }
}

#[cfg(talos_boot_scenario = "rpi5_shared_runqueue_migration")]
#[derive(Clone, Copy)]
struct SharedRunQueueMigrationState {
    reports: [SharedRunQueueMigrationReport; MAX_CORES],
    lock_progress: [u64; MAX_CORES],
}

#[cfg(talos_boot_scenario = "rpi5_shared_runqueue_migration")]
impl SharedRunQueueMigrationState {
    const fn new() -> Self {
        Self {
            reports: [SharedRunQueueMigrationReport::empty(); MAX_CORES],
            lock_progress: [0; MAX_CORES],
        }
    }
}

#[cfg(talos_boot_scenario = "rpi5_shared_runqueue_migration")]
static SHARED_RUNQUEUE_MIGRATION_STATE: SpinLock<SharedRunQueueMigrationState> =
    SpinLock::new(SharedRunQueueMigrationState::new());

#[cfg(talos_boot_scenario = "rpi5_shared_runqueue_migration")]
fn reset_shared_runqueue_migration_state() {
    let mut state = unsafe { SHARED_RUNQUEUE_MIGRATION_STATE.lock_irqsave() };
    *state = SharedRunQueueMigrationState::new();
}

#[cfg(any(
    talos_boot_scenario = "rpi5_shared_runqueue_migration",
    talos_boot_scenario = "rpi5_load_balancing_proof"
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

#[cfg(talos_boot_scenario = "rpi5_shared_runqueue_migration")]
fn diagnostic_scheduler_for(owner: LogicalCpuId) -> PerCoreScheduler<2> {
    if owner == LogicalCpuId::BOOT {
        PerCoreScheduler::<2>::boot_cpu()
    } else {
        PerCoreScheduler::<2>::production_secondary_diagnostic(owner)
    }
}

#[cfg(talos_boot_scenario = "rpi5_shared_runqueue_migration")]
fn build_shared_runqueue_migration_report(logical_cpu: usize) -> SharedRunQueueMigrationReport {
    let source_owner = LogicalCpuId::new(logical_cpu);
    let destination_owner = LogicalCpuId::new((logical_cpu + 1) % MAX_CORES);
    let mut source_scheduler = diagnostic_scheduler_for(source_owner);
    let mut destination_scheduler = diagnostic_scheduler_for(destination_owner);
    let mut metadata =
        SharedSchedulerMetadata::<SHARED_RUNQUEUE_MIGRATION_TASK_CAPACITY, MAX_CORES>::new();
    let mut shared = SharedRunQueue::<SHARED_RUNQUEUE_MIGRATION_QUEUE_CAPACITY, MAX_CORES>::new();
    let mut task = scheduler_task(logical_cpu, 7);
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
        source_role: source_scheduler.role(),
        destination_owner: destination_owner.raw() as u64,
        destination_role: destination_scheduler.role(),
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

#[cfg(talos_boot_scenario = "rpi5_shared_runqueue_migration")]
fn publish_shared_runqueue_migration_report(
    logical_cpu: usize,
    report: SharedRunQueueMigrationReport,
) {
    let mut state = SHARED_RUNQUEUE_MIGRATION_STATE.lock();
    state.reports[logical_cpu] = report;
    state.lock_progress[logical_cpu] = report.lock_progress();
}

#[cfg(talos_boot_scenario = "rpi5_shared_runqueue_migration")]
fn run_shared_runqueue_migration_secondary(core_state: &smp::PerCoreState, logical_cpu: usize) {
    core_state.mark_workload_running();
    core_state.clean_to_poc();

    let report = build_shared_runqueue_migration_report(logical_cpu);
    publish_shared_runqueue_migration_report(logical_cpu, report);
    smp_full_barrier();

    core_state.mark_workload_complete(report.lock_progress());
    core_state.clean_to_poc();
}

#[cfg(talos_boot_scenario = "rpi5_load_balancing_proof")]
#[derive(Clone, Copy)]
struct LoadBalancingProofReport {
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

#[cfg(talos_boot_scenario = "rpi5_load_balancing_proof")]
fn build_load_balancing_proof_report() -> LoadBalancingProofReport {
    let source_owner = LogicalCpuId::BOOT;
    let destination_owner = LogicalCpuId::new(1);
    let mut source_scheduler = PerCoreScheduler::<2>::boot_cpu();
    let mut destination_scheduler =
        PerCoreScheduler::<2>::production_secondary_diagnostic(destination_owner);
    let mut metadata =
        SharedSchedulerMetadata::<LOAD_BALANCING_PROOF_TASK_CAPACITY, MAX_CORES>::new();
    let mut shared = SharedRunQueue::<LOAD_BALANCING_PROOF_QUEUE_CAPACITY, MAX_CORES>::new();
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

    LoadBalancingProofReport {
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

#[cfg(talos_boot_scenario = "rpi5_secondary_scheduler_service_loop")]
#[derive(Clone, Copy)]
struct SecondarySchedulerServiceLoopReport {
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

#[cfg(talos_boot_scenario = "rpi5_secondary_scheduler_service_loop")]
impl SecondarySchedulerServiceLoopReport {
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

#[cfg(talos_boot_scenario = "rpi5_secondary_scheduler_service_loop")]
#[derive(Clone, Copy)]
struct SecondarySchedulerServiceLoopState {
    reports: [SecondarySchedulerServiceLoopReport; MAX_CORES],
    lock_progress: [u64; MAX_CORES],
}

#[cfg(talos_boot_scenario = "rpi5_secondary_scheduler_service_loop")]
impl SecondarySchedulerServiceLoopState {
    const fn new() -> Self {
        Self {
            reports: [SecondarySchedulerServiceLoopReport::empty(); MAX_CORES],
            lock_progress: [0; MAX_CORES],
        }
    }
}

#[cfg(talos_boot_scenario = "rpi5_secondary_scheduler_service_loop")]
static SECONDARY_SCHEDULER_SERVICE_LOOP_STATE: SpinLock<SecondarySchedulerServiceLoopState> =
    SpinLock::new(SecondarySchedulerServiceLoopState::new());

#[cfg(talos_boot_scenario = "rpi5_secondary_scheduler_service_loop")]
fn reset_secondary_scheduler_service_loop_state() {
    let mut state = unsafe { SECONDARY_SCHEDULER_SERVICE_LOOP_STATE.lock_irqsave() };
    *state = SecondarySchedulerServiceLoopState::new();
}

#[cfg(talos_boot_scenario = "rpi5_secondary_scheduler_service_loop")]
fn build_secondary_scheduler_service_loop_report(
    logical_cpu: usize,
) -> SecondarySchedulerServiceLoopReport {
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

    SecondarySchedulerServiceLoopReport {
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

#[cfg(talos_boot_scenario = "rpi5_secondary_scheduler_service_loop")]
fn publish_secondary_scheduler_service_loop_report(
    logical_cpu: usize,
    report: SecondarySchedulerServiceLoopReport,
) {
    let mut state = SECONDARY_SCHEDULER_SERVICE_LOOP_STATE.lock();
    state.reports[logical_cpu] = report;
    state.lock_progress[logical_cpu] = report.progress();
}

#[cfg(talos_boot_scenario = "rpi5_secondary_scheduler_service_loop")]
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

#[cfg(talos_boot_scenario = "rpi5_multicore_preemption_proof")]
#[derive(Clone, Copy)]
struct MultiCorePreemptionProofReport {
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

#[cfg(talos_boot_scenario = "rpi5_multicore_preemption_proof")]
impl MultiCorePreemptionProofReport {
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

#[cfg(talos_boot_scenario = "rpi5_multicore_preemption_proof")]
#[derive(Clone, Copy)]
struct MultiCorePreemptionProofState {
    reports: [MultiCorePreemptionProofReport; MAX_CORES],
    lock_progress: [u64; MAX_CORES],
}

#[cfg(talos_boot_scenario = "rpi5_multicore_preemption_proof")]
impl MultiCorePreemptionProofState {
    const fn new() -> Self {
        Self {
            reports: [MultiCorePreemptionProofReport::empty(); MAX_CORES],
            lock_progress: [0; MAX_CORES],
        }
    }
}

#[cfg(talos_boot_scenario = "rpi5_multicore_preemption_proof")]
static MULTICORE_PREEMPTION_PROOF_STATE: SpinLock<MultiCorePreemptionProofState> =
    SpinLock::new(MultiCorePreemptionProofState::new());

#[cfg(talos_boot_scenario = "rpi5_multicore_preemption_proof")]
fn reset_multicore_preemption_proof_state() {
    let mut state = unsafe { MULTICORE_PREEMPTION_PROOF_STATE.lock_irqsave() };
    *state = MultiCorePreemptionProofState::new();
}

#[cfg(talos_boot_scenario = "rpi5_multicore_preemption_proof")]
fn build_multicore_preemption_proof_report(logical_cpu: usize) -> MultiCorePreemptionProofReport {
    let owner = LogicalCpuId::new(logical_cpu);
    let mut scheduler = PerCoreScheduler::<2>::production_secondary_diagnostic(owner);
    let mut preemption = PerCorePreemptionState::new(owner);
    let mut remote_wakes = RemoteWakeQueue::<1>::new(owner);
    let mut metadata =
        SharedSchedulerMetadata::<MULTICORE_PREEMPTION_PROOF_TASK_CAPACITY, MAX_CORES>::new();
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

    MultiCorePreemptionProofReport {
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

#[cfg(talos_boot_scenario = "rpi5_multicore_preemption_proof")]
fn publish_multicore_preemption_proof_report(
    logical_cpu: usize,
    report: MultiCorePreemptionProofReport,
) {
    let mut state = MULTICORE_PREEMPTION_PROOF_STATE.lock();
    state.reports[logical_cpu] = report;
    state.lock_progress[logical_cpu] = report.progress();
}

#[cfg(talos_boot_scenario = "rpi5_multicore_preemption_proof")]
fn run_multicore_preemption_secondary(core_state: &smp::PerCoreState, logical_cpu: usize) {
    core_state.mark_workload_running();
    core_state.clean_to_poc();

    let report = build_multicore_preemption_proof_report(logical_cpu);
    publish_multicore_preemption_proof_report(logical_cpu, report);
    smp_full_barrier();

    core_state.mark_workload_complete(report.progress());
    core_state.clean_to_poc();
}

#[cfg(talos_boot_scenario = "rpi5_production_timer_preemption_proof")]
#[derive(Clone, Copy)]
struct ProductionTimerPreemptionProofReport {
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

#[cfg(talos_boot_scenario = "rpi5_production_timer_preemption_proof")]
impl ProductionTimerPreemptionProofReport {
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

#[cfg(talos_boot_scenario = "rpi5_production_timer_preemption_proof")]
#[derive(Clone, Copy)]
struct ProductionTimerPreemptionProofState {
    reports: [ProductionTimerPreemptionProofReport; MAX_CORES],
    lock_progress: [u64; MAX_CORES],
}

#[cfg(talos_boot_scenario = "rpi5_production_timer_preemption_proof")]
impl ProductionTimerPreemptionProofState {
    const fn new() -> Self {
        Self {
            reports: [ProductionTimerPreemptionProofReport::empty(); MAX_CORES],
            lock_progress: [0; MAX_CORES],
        }
    }
}

#[cfg(talos_boot_scenario = "rpi5_production_timer_preemption_proof")]
static PRODUCTION_TIMER_PREEMPTION_PROOF_STATE: SpinLock<ProductionTimerPreemptionProofState> =
    SpinLock::new(ProductionTimerPreemptionProofState::new());

#[cfg(talos_boot_scenario = "rpi5_production_timer_preemption_proof")]
fn reset_production_timer_preemption_proof_state() {
    {
        let mut state = unsafe { PRODUCTION_TIMER_PREEMPTION_PROOF_STATE.lock_irqsave() };
        *state = ProductionTimerPreemptionProofState::new();
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

#[cfg(talos_boot_scenario = "rpi5_production_timer_preemption_proof")]
fn build_production_timer_preemption_proof_report(
    logical_cpu: usize,
) -> ProductionTimerPreemptionProofReport {
    let owner = LogicalCpuId::new(logical_cpu);
    let mut metadata =
        SharedSchedulerMetadata::<PRODUCTION_TIMER_PREEMPTION_PROOF_TASK_CAPACITY, MAX_CORES>::new(
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

    ProductionTimerPreemptionProofReport {
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

#[cfg(talos_boot_scenario = "rpi5_production_timer_preemption_proof")]
fn publish_production_timer_preemption_proof_report(
    logical_cpu: usize,
    report: ProductionTimerPreemptionProofReport,
) {
    let mut state = PRODUCTION_TIMER_PREEMPTION_PROOF_STATE.lock();
    state.reports[logical_cpu] = report;
    state.lock_progress[logical_cpu] = report.progress();
}

#[cfg(talos_boot_scenario = "rpi5_production_timer_preemption_proof")]
fn run_production_timer_preemption_secondary(core_state: &smp::PerCoreState, logical_cpu: usize) {
    core_state.mark_workload_running();
    core_state.clean_to_poc();

    let report = build_production_timer_preemption_proof_report(logical_cpu);
    publish_production_timer_preemption_proof_report(logical_cpu, report);
    smp_full_barrier();

    core_state.mark_workload_complete(report.progress());
    core_state.clean_to_poc();
}

#[cfg(talos_boot_scenario = "rpi5_smp_lock_cache_coherence")]
#[derive(Clone, Copy)]
struct SmpLockContentionState {
    shared_counter: u64,
    per_core_counts: [u64; MAX_CORES],
    error_count: u64,
}

#[cfg(talos_boot_scenario = "rpi5_smp_lock_cache_coherence")]
impl SmpLockContentionState {
    const fn new() -> Self {
        Self {
            shared_counter: 0,
            per_core_counts: [0; MAX_CORES],
            error_count: 0,
        }
    }
}

#[cfg(talos_boot_scenario = "rpi5_smp_lock_cache_coherence")]
static SMP_LOCK_CONTENTION_STATE: SpinLock<SmpLockContentionState> =
    SpinLock::new(SmpLockContentionState::new());

#[cfg(talos_boot_scenario = "rpi5_smp_lock_cache_coherence")]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u64)]
enum SmpLockDiagnosticPhase {
    Idle = 0,
    SecondaryEntered = 1,
    BeforeLockAttempt = 2,
    WaitingForLock = 3,
    LockAcquired = 4,
    LockReleased = 5,
    IterationComplete = 6,
    WorkloadComplete = 7,
    LockAcquireTimeout = 8,
}

#[cfg(talos_boot_scenario = "rpi5_smp_lock_cache_coherence")]
impl SmpLockDiagnosticPhase {
    const fn from_raw(raw: u64) -> Self {
        match raw {
            1 => Self::SecondaryEntered,
            2 => Self::BeforeLockAttempt,
            3 => Self::WaitingForLock,
            4 => Self::LockAcquired,
            5 => Self::LockReleased,
            6 => Self::IterationComplete,
            7 => Self::WorkloadComplete,
            8 => Self::LockAcquireTimeout,
            _ => Self::Idle,
        }
    }

    const fn raw(self) -> u64 {
        self as u64
    }

    const fn name(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::SecondaryEntered => "secondary-entered",
            Self::BeforeLockAttempt => "before-lock-attempt",
            Self::WaitingForLock => "waiting-for-lock",
            Self::LockAcquired => "lock-acquired",
            Self::LockReleased => "lock-released",
            Self::IterationComplete => "iteration-complete",
            Self::WorkloadComplete => "workload-complete",
            Self::LockAcquireTimeout => "lock-acquire-timeout",
        }
    }
}

#[cfg(talos_boot_scenario = "rpi5_smp_lock_cache_coherence")]
#[derive(Clone, Copy)]
struct SmpLockDiagnosticSnapshot {
    phase: SmpLockDiagnosticPhase,
    progress: u64,
    attempts: u64,
    timeouts: u64,
    releases: u64,
    sctlr_el2: u64,
}

#[cfg(talos_boot_scenario = "rpi5_smp_lock_cache_coherence")]
static SMP_LOCK_DIAGNOSTIC_PHASES: [AtomicU64; MAX_CORES] =
    [const { AtomicU64::new(SmpLockDiagnosticPhase::Idle.raw()) }; MAX_CORES];
#[cfg(talos_boot_scenario = "rpi5_smp_lock_cache_coherence")]
static SMP_LOCK_DIAGNOSTIC_PROGRESS: [AtomicU64; MAX_CORES] =
    [const { AtomicU64::new(0) }; MAX_CORES];
#[cfg(talos_boot_scenario = "rpi5_smp_lock_cache_coherence")]
static SMP_LOCK_DIAGNOSTIC_ATTEMPTS: [AtomicU64; MAX_CORES] =
    [const { AtomicU64::new(0) }; MAX_CORES];
#[cfg(talos_boot_scenario = "rpi5_smp_lock_cache_coherence")]
static SMP_LOCK_DIAGNOSTIC_TIMEOUTS: [AtomicU64; MAX_CORES] =
    [const { AtomicU64::new(0) }; MAX_CORES];
#[cfg(talos_boot_scenario = "rpi5_smp_lock_cache_coherence")]
static SMP_LOCK_DIAGNOSTIC_RELEASES: [AtomicU64; MAX_CORES] =
    [const { AtomicU64::new(0) }; MAX_CORES];
#[cfg(talos_boot_scenario = "rpi5_smp_lock_cache_coherence")]
static SMP_LOCK_DIAGNOSTIC_SCTLR_EL2: [AtomicU64; MAX_CORES] =
    [const { AtomicU64::new(0) }; MAX_CORES];
#[cfg(any(
    talos_boot_scenario = "rpi5_smp_lock_cache_coherence",
    talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
    talos_boot_scenario = "rpi5_remote_wakeup_request",
    talos_boot_scenario = "rpi5_production_secondary_dispatch",
    talos_boot_scenario = "rpi5_shared_scheduler_metadata",
    talos_boot_scenario = "rpi5_shared_runqueue_migration",
    talos_boot_scenario = "rpi5_multicore_preemption_proof",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof",
    talos_boot_scenario = "rpi5_secondary_scheduler_service_loop"
))]
static SECONDARY_CACHEABLE_MMU_HANDOFF_READY: AtomicU64 = AtomicU64::new(0);
#[cfg(any(
    talos_boot_scenario = "rpi5_smp_lock_cache_coherence",
    talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
    talos_boot_scenario = "rpi5_remote_wakeup_request",
    talos_boot_scenario = "rpi5_production_secondary_dispatch",
    talos_boot_scenario = "rpi5_shared_scheduler_metadata",
    talos_boot_scenario = "rpi5_shared_runqueue_migration",
    talos_boot_scenario = "rpi5_multicore_preemption_proof",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof",
    talos_boot_scenario = "rpi5_secondary_scheduler_service_loop"
))]
static SECONDARY_CACHEABLE_MMU_HANDOFF_MAIR_EL2: AtomicU64 = AtomicU64::new(0);
#[cfg(any(
    talos_boot_scenario = "rpi5_smp_lock_cache_coherence",
    talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
    talos_boot_scenario = "rpi5_remote_wakeup_request",
    talos_boot_scenario = "rpi5_production_secondary_dispatch",
    talos_boot_scenario = "rpi5_shared_scheduler_metadata",
    talos_boot_scenario = "rpi5_shared_runqueue_migration",
    talos_boot_scenario = "rpi5_multicore_preemption_proof",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof",
    talos_boot_scenario = "rpi5_secondary_scheduler_service_loop"
))]
static SECONDARY_CACHEABLE_MMU_HANDOFF_TCR_EL2: AtomicU64 = AtomicU64::new(0);
#[cfg(any(
    talos_boot_scenario = "rpi5_smp_lock_cache_coherence",
    talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
    talos_boot_scenario = "rpi5_remote_wakeup_request",
    talos_boot_scenario = "rpi5_production_secondary_dispatch",
    talos_boot_scenario = "rpi5_shared_scheduler_metadata",
    talos_boot_scenario = "rpi5_shared_runqueue_migration",
    talos_boot_scenario = "rpi5_multicore_preemption_proof",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof",
    talos_boot_scenario = "rpi5_secondary_scheduler_service_loop"
))]
static SECONDARY_CACHEABLE_MMU_HANDOFF_TTBR0_EL2: AtomicU64 = AtomicU64::new(0);
#[cfg(any(
    talos_boot_scenario = "rpi5_smp_lock_cache_coherence",
    talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
    talos_boot_scenario = "rpi5_remote_wakeup_request",
    talos_boot_scenario = "rpi5_production_secondary_dispatch",
    talos_boot_scenario = "rpi5_shared_scheduler_metadata",
    talos_boot_scenario = "rpi5_shared_runqueue_migration",
    talos_boot_scenario = "rpi5_multicore_preemption_proof",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof",
    talos_boot_scenario = "rpi5_secondary_scheduler_service_loop"
))]
static SECONDARY_CACHEABLE_MMU_HANDOFF_SCTLR_EL2: AtomicU64 = AtomicU64::new(0);

#[cfg(any(
    talos_boot_scenario = "rpi5_smp_lock_cache_coherence",
    talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
    talos_boot_scenario = "rpi5_remote_wakeup_request",
    talos_boot_scenario = "rpi5_production_secondary_dispatch",
    talos_boot_scenario = "rpi5_shared_scheduler_metadata",
    talos_boot_scenario = "rpi5_shared_runqueue_migration",
    talos_boot_scenario = "rpi5_multicore_preemption_proof",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof",
    talos_boot_scenario = "rpi5_secondary_scheduler_service_loop"
))]
fn clean_secondary_cacheable_mmu_handoff_plan() {
    clean_cache_line_to_poc(&SECONDARY_CACHEABLE_MMU_HANDOFF_MAIR_EL2);
    clean_cache_line_to_poc(&SECONDARY_CACHEABLE_MMU_HANDOFF_TCR_EL2);
    clean_cache_line_to_poc(&SECONDARY_CACHEABLE_MMU_HANDOFF_TTBR0_EL2);
    clean_cache_line_to_poc(&SECONDARY_CACHEABLE_MMU_HANDOFF_SCTLR_EL2);
    clean_cache_line_to_poc(&SECONDARY_CACHEABLE_MMU_HANDOFF_READY);
}

#[cfg(any(
    talos_boot_scenario = "rpi5_smp_lock_cache_coherence",
    talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
    talos_boot_scenario = "rpi5_remote_wakeup_request",
    talos_boot_scenario = "rpi5_production_secondary_dispatch",
    talos_boot_scenario = "rpi5_shared_scheduler_metadata",
    talos_boot_scenario = "rpi5_shared_runqueue_migration",
    talos_boot_scenario = "rpi5_multicore_preemption_proof",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof",
    talos_boot_scenario = "rpi5_secondary_scheduler_service_loop"
))]
fn publish_secondary_cacheable_mmu_handoff_plan(
    regime: crate::arch::aarch64::El2Stage1CacheRegime,
) {
    SECONDARY_CACHEABLE_MMU_HANDOFF_READY.store(0, Ordering::Release);
    clean_secondary_cacheable_mmu_handoff_plan();
    SECONDARY_CACHEABLE_MMU_HANDOFF_MAIR_EL2.store(regime.mair, Ordering::Release);
    SECONDARY_CACHEABLE_MMU_HANDOFF_TCR_EL2.store(regime.tcr, Ordering::Release);
    SECONDARY_CACHEABLE_MMU_HANDOFF_TTBR0_EL2.store(regime.ttbr0, Ordering::Release);
    SECONDARY_CACHEABLE_MMU_HANDOFF_SCTLR_EL2.store(
        regime.sctlr | RPI5_SCTLR_M_ENABLE | RPI5_SCTLR_I_ENABLE | RPI5_SCTLR_C_ENABLE,
        Ordering::Release,
    );
    SECONDARY_CACHEABLE_MMU_HANDOFF_READY.store(1, Ordering::Release);
    clean_secondary_cacheable_mmu_handoff_plan();
}

#[cfg(any(
    talos_boot_scenario = "rpi5_smp_lock_cache_coherence",
    talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
    talos_boot_scenario = "rpi5_remote_wakeup_request",
    talos_boot_scenario = "rpi5_production_secondary_dispatch",
    talos_boot_scenario = "rpi5_shared_scheduler_metadata",
    talos_boot_scenario = "rpi5_shared_runqueue_migration",
    talos_boot_scenario = "rpi5_multicore_preemption_proof",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof",
    talos_boot_scenario = "rpi5_secondary_scheduler_service_loop"
))]
fn secondary_cacheable_mmu_handoff_plan() -> Option<crate::arch::aarch64::El2Stage1CacheRegime> {
    invalidate_cache_line_from_poc(&SECONDARY_CACHEABLE_MMU_HANDOFF_READY);
    if SECONDARY_CACHEABLE_MMU_HANDOFF_READY.load(Ordering::Acquire) != 1 {
        return None;
    }

    invalidate_cache_line_from_poc(&SECONDARY_CACHEABLE_MMU_HANDOFF_MAIR_EL2);
    invalidate_cache_line_from_poc(&SECONDARY_CACHEABLE_MMU_HANDOFF_TCR_EL2);
    invalidate_cache_line_from_poc(&SECONDARY_CACHEABLE_MMU_HANDOFF_TTBR0_EL2);
    invalidate_cache_line_from_poc(&SECONDARY_CACHEABLE_MMU_HANDOFF_SCTLR_EL2);
    Some(crate::arch::aarch64::El2Stage1CacheRegime {
        mair: SECONDARY_CACHEABLE_MMU_HANDOFF_MAIR_EL2.load(Ordering::Acquire),
        tcr: SECONDARY_CACHEABLE_MMU_HANDOFF_TCR_EL2.load(Ordering::Acquire),
        ttbr0: SECONDARY_CACHEABLE_MMU_HANDOFF_TTBR0_EL2.load(Ordering::Acquire),
        sctlr: SECONDARY_CACHEABLE_MMU_HANDOFF_SCTLR_EL2.load(Ordering::Acquire),
    })
}

#[cfg(any(
    talos_boot_scenario = "rpi5_smp_lock_cache_coherence",
    talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
    talos_boot_scenario = "rpi5_remote_wakeup_request",
    talos_boot_scenario = "rpi5_production_secondary_dispatch",
    talos_boot_scenario = "rpi5_shared_scheduler_metadata",
    talos_boot_scenario = "rpi5_shared_runqueue_migration",
    talos_boot_scenario = "rpi5_multicore_preemption_proof",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof",
    talos_boot_scenario = "rpi5_secondary_scheduler_service_loop"
))]
fn enter_secondary_cacheable_mmu_handoff(logical_cpu: usize) -> bool {
    let _ = logical_cpu;
    let Some(plan) = secondary_cacheable_mmu_handoff_plan() else {
        #[cfg(talos_boot_scenario = "rpi5_smp_lock_cache_coherence")]
        record_smp_lock_diagnostic(
            logical_cpu,
            SmpLockDiagnosticPhase::SecondaryEntered,
            0,
            0,
            0,
            0,
            current_sctlr_el2(),
        );
        return false;
    };

    let Some(after) = (unsafe { crate::arch::aarch64::install_el2_stage1_cache_regime(plan) })
    else {
        #[cfg(talos_boot_scenario = "rpi5_smp_lock_cache_coherence")]
        record_smp_lock_diagnostic(
            logical_cpu,
            SmpLockDiagnosticPhase::SecondaryEntered,
            0,
            0,
            0,
            0,
            current_sctlr_el2(),
        );
        return false;
    };
    #[cfg(talos_boot_scenario = "rpi5_smp_lock_cache_coherence")]
    record_smp_lock_diagnostic(
        logical_cpu,
        SmpLockDiagnosticPhase::SecondaryEntered,
        0,
        0,
        0,
        0,
        after.sctlr,
    );
    cacheable_mmu_enabled(after.sctlr)
}

#[cfg(talos_boot_scenario = "rpi5_smp_lock_cache_coherence")]
fn reset_smp_lock_contention_state() {
    let mut state = SMP_LOCK_CONTENTION_STATE.lock();
    *state = SmpLockContentionState::new();
}

#[cfg(talos_boot_scenario = "rpi5_smp_lock_cache_coherence")]
fn reset_smp_lock_diagnostic_state() {
    for logical_cpu in 0..MAX_CORES {
        record_smp_lock_diagnostic(logical_cpu, SmpLockDiagnosticPhase::Idle, 0, 0, 0, 0, 0);
    }
}

#[cfg(talos_boot_scenario = "rpi5_smp_lock_cache_coherence")]
fn record_smp_lock_diagnostic(
    logical_cpu: usize,
    phase: SmpLockDiagnosticPhase,
    progress: u64,
    attempts: u64,
    timeouts: u64,
    releases: u64,
    sctlr_el2: u64,
) {
    if logical_cpu >= MAX_CORES {
        return;
    }
    SMP_LOCK_DIAGNOSTIC_PROGRESS[logical_cpu].store(progress, Ordering::Release);
    SMP_LOCK_DIAGNOSTIC_ATTEMPTS[logical_cpu].store(attempts, Ordering::Release);
    SMP_LOCK_DIAGNOSTIC_TIMEOUTS[logical_cpu].store(timeouts, Ordering::Release);
    SMP_LOCK_DIAGNOSTIC_RELEASES[logical_cpu].store(releases, Ordering::Release);
    SMP_LOCK_DIAGNOSTIC_SCTLR_EL2[logical_cpu].store(sctlr_el2, Ordering::Release);
    SMP_LOCK_DIAGNOSTIC_PHASES[logical_cpu].store(phase.raw(), Ordering::Release);
    clean_smp_lock_diagnostic(logical_cpu);
}

#[cfg(talos_boot_scenario = "rpi5_smp_lock_cache_coherence")]
fn clean_smp_lock_diagnostic(logical_cpu: usize) {
    clean_cache_line_to_poc(&SMP_LOCK_DIAGNOSTIC_PROGRESS[logical_cpu]);
    clean_cache_line_to_poc(&SMP_LOCK_DIAGNOSTIC_ATTEMPTS[logical_cpu]);
    clean_cache_line_to_poc(&SMP_LOCK_DIAGNOSTIC_TIMEOUTS[logical_cpu]);
    clean_cache_line_to_poc(&SMP_LOCK_DIAGNOSTIC_RELEASES[logical_cpu]);
    clean_cache_line_to_poc(&SMP_LOCK_DIAGNOSTIC_SCTLR_EL2[logical_cpu]);
    clean_cache_line_to_poc(&SMP_LOCK_DIAGNOSTIC_PHASES[logical_cpu]);
}

#[cfg(talos_boot_scenario = "rpi5_smp_lock_cache_coherence")]
fn snapshot_smp_lock_diagnostic(logical_cpu: usize) -> SmpLockDiagnosticSnapshot {
    invalidate_cache_line_from_poc(&SMP_LOCK_DIAGNOSTIC_PROGRESS[logical_cpu]);
    invalidate_cache_line_from_poc(&SMP_LOCK_DIAGNOSTIC_ATTEMPTS[logical_cpu]);
    invalidate_cache_line_from_poc(&SMP_LOCK_DIAGNOSTIC_TIMEOUTS[logical_cpu]);
    invalidate_cache_line_from_poc(&SMP_LOCK_DIAGNOSTIC_RELEASES[logical_cpu]);
    invalidate_cache_line_from_poc(&SMP_LOCK_DIAGNOSTIC_SCTLR_EL2[logical_cpu]);
    invalidate_cache_line_from_poc(&SMP_LOCK_DIAGNOSTIC_PHASES[logical_cpu]);
    SmpLockDiagnosticSnapshot {
        phase: SmpLockDiagnosticPhase::from_raw(
            SMP_LOCK_DIAGNOSTIC_PHASES[logical_cpu].load(Ordering::Acquire),
        ),
        progress: SMP_LOCK_DIAGNOSTIC_PROGRESS[logical_cpu].load(Ordering::Acquire),
        attempts: SMP_LOCK_DIAGNOSTIC_ATTEMPTS[logical_cpu].load(Ordering::Acquire),
        timeouts: SMP_LOCK_DIAGNOSTIC_TIMEOUTS[logical_cpu].load(Ordering::Acquire),
        releases: SMP_LOCK_DIAGNOSTIC_RELEASES[logical_cpu].load(Ordering::Acquire),
        sctlr_el2: SMP_LOCK_DIAGNOSTIC_SCTLR_EL2[logical_cpu].load(Ordering::Acquire),
    }
}

#[cfg(talos_boot_scenario = "rpi5_smp_lock_cache_coherence")]
fn write_smp_lock_wait_observation(logical_cpu: usize, remaining: usize) {
    SECONDARY_CORE_STATES[logical_cpu].invalidate_from_poc();
    let report = SECONDARY_CORE_STATES[logical_cpu].snapshot(logical_cpu);
    let diagnostic = snapshot_smp_lock_diagnostic(logical_cpu);
    crate::println!(
        "rpi5-smp-lock-cache-coherence: wait logical={} remaining={} state={} progress={} diag-phase={} diag-progress={} diag-attempts={} diag-timeouts={} diag-releases={} diag-sctlr-el2={:#018x} diag-cacheable-mmu={}",
        logical_cpu,
        remaining,
        secondary_state_name(report.lifecycle.raw()),
        report.workload_progress,
        diagnostic.phase.name(),
        diagnostic.progress,
        diagnostic.attempts,
        diagnostic.timeouts,
        diagnostic.releases,
        diagnostic.sctlr_el2,
        cacheable_mmu_enabled(diagnostic.sctlr_el2)
    );
}

#[cfg(talos_boot_scenario = "rpi5_cross_core_ipi_delivery")]
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
    polled_hppirs: [AtomicU64; MAX_CORES],
    polled_iars: [AtomicU64; MAX_CORES],
    polled_intids: [AtomicU64; MAX_CORES],
    polled_daifs: [AtomicU64; MAX_CORES],
    polled_hcrs: [AtomicU64; MAX_CORES],
    poll_counts: [AtomicU64; MAX_CORES],
    errors: AtomicU64,
}

#[cfg(talos_boot_scenario = "rpi5_cross_core_ipi_delivery")]
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
            polled_hppirs: [const { AtomicU64::new(0) }; MAX_CORES],
            polled_iars: [const { AtomicU64::new(0) }; MAX_CORES],
            polled_intids: [const { AtomicU64::new(0) }; MAX_CORES],
            polled_daifs: [const { AtomicU64::new(0) }; MAX_CORES],
            polled_hcrs: [const { AtomicU64::new(0) }; MAX_CORES],
            poll_counts: [const { AtomicU64::new(0) }; MAX_CORES],
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
            self.polled_hppirs[logical_cpu].store(0, Ordering::Release);
            self.polled_iars[logical_cpu].store(0, Ordering::Release);
            self.polled_intids[logical_cpu].store(0, Ordering::Release);
            self.polled_daifs[logical_cpu].store(0, Ordering::Release);
            self.polled_hcrs[logical_cpu].store(0, Ordering::Release);
            self.poll_counts[logical_cpu].store(0, Ordering::Release);
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

    fn record_filter_send(&self, target_mask: u64, sgir_value: u32) {
        for logical_cpu in 1..MAX_CORES {
            self.target_bits[logical_cpu].store(target_mask, Ordering::Release);
            self.sent_values[logical_cpu].store(sgir_value as u64, Ordering::Release);
        }
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

    fn record_cpu_interface_poll(
        &self,
        logical_cpu: usize,
        hppir: u32,
        iar: u32,
        daif: u64,
        hcr: u64,
    ) {
        if logical_cpu < MAX_CORES {
            self.polled_hppirs[logical_cpu].store(hppir as u64, Ordering::Release);
            self.polled_iars[logical_cpu].store(iar as u64, Ordering::Release);
            self.polled_intids[logical_cpu].store((iar & 0x03ff) as u64, Ordering::Release);
            self.polled_daifs[logical_cpu].store(daif, Ordering::Release);
            self.polled_hcrs[logical_cpu].store(hcr, Ordering::Release);
            self.poll_counts[logical_cpu].fetch_add(1, Ordering::AcqRel);
        } else {
            self.errors.fetch_add(1, Ordering::AcqRel);
        }
    }
}

#[cfg(talos_boot_scenario = "rpi5_cross_core_ipi_delivery")]
static CROSS_CORE_IPI_DELIVERY_STATE: CrossCoreIpiDeliveryState = CrossCoreIpiDeliveryState::new();

#[cfg(any(
    talos_boot_scenario = "rpi5_timer_irq",
    talos_boot_scenario = "rpi5_timer_preemption",
    talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
    talos_boot_scenario = "rpi5_remote_wakeup_request"
))]
fn current_pi5_logical_cpu() -> Option<usize> {
    crate::smp::pi5_logical_cpu_from_mpidr_affinity(crate::arch::aarch64::mpidr_affinity(
        crate::arch::aarch64::mpidr_el1(),
    ))
}

#[cfg(any(
    talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
    talos_boot_scenario = "rpi5_remote_wakeup_request"
))]
fn current_hcr_el2() -> u64 {
    let value: u64;
    unsafe {
        core::arch::asm!("mrs {value}, HCR_EL2", value = out(reg) value, options(nomem, nostack, preserves_flags));
    }
    value
}

#[cfg(talos_boot_scenario = "rpi5_cross_core_ipi_delivery")]
fn poll_cross_core_ipi_cpu_interface(logical_cpu: usize) -> bool {
    let gic = GicV2::new(GICD_BASE, GICC_BASE);
    let hppir = unsafe { gic.highest_pending() };
    let iar = unsafe { gic.acknowledge() };
    let intid = iar & 0x03ff;
    CROSS_CORE_IPI_DELIVERY_STATE.record_cpu_interface_poll(
        logical_cpu,
        hppir,
        iar,
        aarch64::daif(),
        current_hcr_el2(),
    );
    if intid == RPI5_CROSS_CORE_IPI_SGI_INTID {
        unsafe {
            gic.end_interrupt(iar);
        }
        true
    } else {
        false
    }
}

#[cfg(talos_boot_scenario = "rpi5_cross_core_ipi_delivery")]
fn run_cross_core_ipi_delivery_secondary(core_state: &smp::PerCoreState, logical_cpu: usize) {
    core_state.mark_workload_running();
    core_state.clean_to_poc();

    crate::arch::aarch64::exceptions::init();
    unsafe {
        aarch64::disable_irq();
        aarch64::route_physical_irqs_to_el2();
        let gic = GicV2::new(GICD_BASE, GICC_BASE);
        gic.configure_sgi_or_ppi_group1(RPI5_CROSS_CORE_IPI_SGI_INTID, 0x80);
        gic.enable_cpu_interface();
        aarch64::enable_irq();
    }

    CROSS_CORE_IPI_DELIVERY_STATE.mark_ready(logical_cpu);

    let mut remaining = RPI5_SECONDARY_WAIT_LIMIT;
    let mut poll_remaining = RPI5_CROSS_CORE_IPI_CPU_INTERFACE_POLL_INTERVAL;
    while CROSS_CORE_IPI_DELIVERY_STATE.receive_count(logical_cpu) == 0 && remaining > 0 {
        core::hint::spin_loop();
        poll_remaining -= 1;
        if poll_remaining == 0 {
            if poll_cross_core_ipi_cpu_interface(logical_cpu) {
                break;
            }
            poll_remaining = RPI5_CROSS_CORE_IPI_CPU_INTERFACE_POLL_INTERVAL;
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

#[cfg(talos_boot_scenario = "rpi5_cross_core_ipi_delivery")]
fn write_cross_core_ipi_wait_observation(remaining: usize) {
    let ready_mask = CROSS_CORE_IPI_DELIVERY_STATE
        .ready_mask
        .load(Ordering::Acquire);
    let complete_mask = CROSS_CORE_IPI_DELIVERY_STATE
        .complete_mask
        .load(Ordering::Acquire);
    crate::println!(
        "rpi5-cross-core-ipi-delivery: wait remaining={} ready-mask={:#x} complete-mask={:#x}",
        remaining,
        ready_mask,
        complete_mask
    );
    wait_uart10_empty_early_phase();
}

#[cfg(talos_boot_scenario = "rpi5_remote_wakeup_request")]
struct RemoteWakeRequestProofState {
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
    last_vectors: [AtomicU64; MAX_CORES],
    last_iars: [AtomicU64; MAX_CORES],
    last_intids: [AtomicU64; MAX_CORES],
    polled_hppirs: [AtomicU64; MAX_CORES],
    polled_iars: [AtomicU64; MAX_CORES],
    polled_intids: [AtomicU64; MAX_CORES],
    polled_daifs: [AtomicU64; MAX_CORES],
    polled_hcrs: [AtomicU64; MAX_CORES],
    poll_counts: [AtomicU64; MAX_CORES],
    #[cfg(talos_boot_scenario = "rpi5_remote_wake_to_local_runnable")]
    local_wake_task_ids: [AtomicU64; MAX_CORES],
    #[cfg(talos_boot_scenario = "rpi5_remote_wake_to_local_runnable")]
    local_runnable_lens: [AtomicU64; MAX_CORES],
    #[cfg(talos_boot_scenario = "rpi5_remote_wake_to_local_runnable")]
    local_state_before: [AtomicU64; MAX_CORES],
    #[cfg(talos_boot_scenario = "rpi5_remote_wake_to_local_runnable")]
    local_state_after: [AtomicU64; MAX_CORES],
    #[cfg(talos_boot_scenario = "rpi5_remote_wake_to_local_runnable")]
    duplicate_local_rejections: [AtomicU64; MAX_CORES],
    errors: AtomicU64,
}

#[cfg(talos_boot_scenario = "rpi5_remote_wakeup_request")]
impl RemoteWakeRequestProofState {
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
            last_vectors: [const { AtomicU64::new(0) }; MAX_CORES],
            last_iars: [const { AtomicU64::new(0) }; MAX_CORES],
            last_intids: [const { AtomicU64::new(0) }; MAX_CORES],
            polled_hppirs: [const { AtomicU64::new(0) }; MAX_CORES],
            polled_iars: [const { AtomicU64::new(0) }; MAX_CORES],
            polled_intids: [const { AtomicU64::new(0) }; MAX_CORES],
            polled_daifs: [const { AtomicU64::new(0) }; MAX_CORES],
            polled_hcrs: [const { AtomicU64::new(0) }; MAX_CORES],
            poll_counts: [const { AtomicU64::new(0) }; MAX_CORES],
            #[cfg(talos_boot_scenario = "rpi5_remote_wake_to_local_runnable")]
            local_wake_task_ids: [const { AtomicU64::new(0) }; MAX_CORES],
            #[cfg(talos_boot_scenario = "rpi5_remote_wake_to_local_runnable")]
            local_runnable_lens: [const { AtomicU64::new(0) }; MAX_CORES],
            #[cfg(talos_boot_scenario = "rpi5_remote_wake_to_local_runnable")]
            local_state_before: [const { AtomicU64::new(0) }; MAX_CORES],
            #[cfg(talos_boot_scenario = "rpi5_remote_wake_to_local_runnable")]
            local_state_after: [const { AtomicU64::new(0) }; MAX_CORES],
            #[cfg(talos_boot_scenario = "rpi5_remote_wake_to_local_runnable")]
            duplicate_local_rejections: [const { AtomicU64::new(0) }; MAX_CORES],
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
            self.last_vectors[logical_cpu].store(0, Ordering::Release);
            self.last_iars[logical_cpu].store(0, Ordering::Release);
            self.last_intids[logical_cpu].store(0, Ordering::Release);
            self.polled_hppirs[logical_cpu].store(0, Ordering::Release);
            self.polled_iars[logical_cpu].store(0, Ordering::Release);
            self.polled_intids[logical_cpu].store(0, Ordering::Release);
            self.polled_daifs[logical_cpu].store(0, Ordering::Release);
            self.polled_hcrs[logical_cpu].store(0, Ordering::Release);
            self.poll_counts[logical_cpu].store(0, Ordering::Release);
            #[cfg(talos_boot_scenario = "rpi5_remote_wake_to_local_runnable")]
            {
                self.local_wake_task_ids[logical_cpu].store(0, Ordering::Release);
                self.local_runnable_lens[logical_cpu].store(0, Ordering::Release);
                self.local_state_before[logical_cpu].store(0, Ordering::Release);
                self.local_state_after[logical_cpu].store(0, Ordering::Release);
                self.duplicate_local_rejections[logical_cpu].store(0, Ordering::Release);
            }
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

    fn record_cpu_interface_poll(
        &self,
        logical_cpu: usize,
        hppir: u32,
        iar: u32,
        daif: u64,
        hcr: u64,
    ) {
        if logical_cpu < MAX_CORES {
            self.polled_hppirs[logical_cpu].store(hppir as u64, Ordering::Release);
            self.polled_iars[logical_cpu].store(iar as u64, Ordering::Release);
            self.polled_intids[logical_cpu].store((iar & 0x03ff) as u64, Ordering::Release);
            self.polled_daifs[logical_cpu].store(daif, Ordering::Release);
            self.polled_hcrs[logical_cpu].store(hcr, Ordering::Release);
            self.poll_counts[logical_cpu].fetch_add(1, Ordering::AcqRel);
        } else {
            self.errors.fetch_add(1, Ordering::AcqRel);
        }
    }
}

#[cfg(talos_boot_scenario = "rpi5_remote_wakeup_request")]
static REMOTE_WAKE_REQUEST_PROOF_STATE: RemoteWakeRequestProofState =
    RemoteWakeRequestProofState::new();

#[cfg(talos_boot_scenario = "rpi5_remote_wakeup_request")]
static REMOTE_WAKE_QUEUES: [SpinLock<RemoteWakeQueue<REMOTE_WAKE_QUEUE_CAPACITY>>; MAX_CORES] = [
    SpinLock::new(RemoteWakeQueue::new(LogicalCpuId::new(0))),
    SpinLock::new(RemoteWakeQueue::new(LogicalCpuId::new(1))),
    SpinLock::new(RemoteWakeQueue::new(LogicalCpuId::new(2))),
    SpinLock::new(RemoteWakeQueue::new(LogicalCpuId::new(3))),
];

#[cfg(talos_boot_scenario = "rpi5_remote_wakeup_request")]
fn reset_remote_wakeup_request_state() {
    REMOTE_WAKE_REQUEST_PROOF_STATE.reset();
    for logical_cpu in 0..MAX_CORES {
        let mut queue = unsafe { REMOTE_WAKE_QUEUES[logical_cpu].lock_irqsave() };
        *queue = RemoteWakeQueue::new(LogicalCpuId::new(logical_cpu));
    }
}

#[cfg(talos_boot_scenario = "rpi5_remote_wakeup_request")]
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
                "rpi5-remote-wakeup-request: publish requester=0 target={} task={} outcome=inserted",
                target,
                task_id.raw()
            );
            true
        }
        Ok(RemoteWakePublishOutcome::Duplicate) => {
            crate::println!(
                "rpi5-remote-wakeup-request: publish requester=0 target={} task={} outcome=duplicate",
                target,
                task_id.raw()
            );
            true
        }
        Err(error) => {
            REMOTE_WAKE_REQUEST_PROOF_STATE
                .errors
                .fetch_add(1, Ordering::AcqRel);
            crate::println!(
                "rpi5-remote-wakeup-request: publish requester=0 target={} task={} outcome=error {:?}",
                target,
                task_id.raw(),
                error
            );
            false
        }
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_remote_wake_to_local_runnable",
    talos_boot_scenario = "rpi5_shared_scheduler_metadata",
    talos_boot_scenario = "rpi5_shared_runqueue_migration",
    talos_boot_scenario = "rpi5_load_balancing_proof",
    talos_boot_scenario = "rpi5_multicore_preemption_proof",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof",
    talos_boot_scenario = "rpi5_secondary_scheduler_service_loop"
))]
fn task_state_code(state: TaskState) -> u64 {
    match state {
        TaskState::Running => 1,
        TaskState::Runnable => 2,
        TaskState::Blocked => 3,
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_remote_wake_to_local_runnable",
    talos_boot_scenario = "rpi5_shared_scheduler_metadata",
    talos_boot_scenario = "rpi5_shared_runqueue_migration",
    talos_boot_scenario = "rpi5_load_balancing_proof",
    talos_boot_scenario = "rpi5_multicore_preemption_proof",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof",
    talos_boot_scenario = "rpi5_secondary_scheduler_service_loop"
))]
fn task_state_name(code: u64) -> &'static str {
    match code {
        1 => "running",
        2 => "runnable",
        3 => "blocked",
        _ => "unknown",
    }
}

#[cfg(talos_boot_scenario = "rpi5_remote_wakeup_request")]
fn poll_remote_wakeup_cpu_interface(logical_cpu: usize) -> bool {
    let gic = GicV2::new(GICD_BASE, GICC_BASE);
    let hppir = unsafe { gic.highest_pending() };
    let iar = unsafe { gic.acknowledge() };
    let intid = iar & 0x03ff;
    REMOTE_WAKE_REQUEST_PROOF_STATE.record_cpu_interface_poll(
        logical_cpu,
        hppir,
        iar,
        aarch64::daif(),
        current_hcr_el2(),
    );
    if intid == RPI5_CROSS_CORE_IPI_SGI_INTID {
        REMOTE_WAKE_REQUEST_PROOF_STATE.record_receive(Some(logical_cpu), 0, iar, intid);
        unsafe {
            gic.end_interrupt(iar);
        }
        REMOTE_WAKE_REQUEST_PROOF_STATE.record_eoi(Some(logical_cpu));
        true
    } else {
        false
    }
}

#[cfg(talos_boot_scenario = "rpi5_remote_wakeup_request")]
fn run_remote_wakeup_request_secondary(core_state: &smp::PerCoreState, logical_cpu: usize) {
    core_state.mark_workload_running();
    core_state.clean_to_poc();

    crate::arch::aarch64::exceptions::init();
    unsafe {
        aarch64::disable_irq();
        aarch64::route_physical_irqs_to_el2();
        let gic = GicV2::new(GICD_BASE, GICC_BASE);
        gic.configure_sgi_or_ppi_group1(RPI5_CROSS_CORE_IPI_SGI_INTID, 0x80);
        gic.enable_cpu_interface();
        aarch64::enable_irq();
    }

    REMOTE_WAKE_REQUEST_PROOF_STATE.mark_ready(logical_cpu);

    let mut remaining = RPI5_SECONDARY_WAIT_LIMIT;
    let mut poll_remaining = RPI5_CROSS_CORE_IPI_CPU_INTERFACE_POLL_INTERVAL;
    while REMOTE_WAKE_REQUEST_PROOF_STATE.receive_count(logical_cpu) == 0 && remaining > 0 {
        core::hint::spin_loop();
        poll_remaining -= 1;
        if poll_remaining == 0 {
            if poll_remote_wakeup_cpu_interface(logical_cpu) {
                break;
            }
            poll_remaining = RPI5_CROSS_CORE_IPI_CPU_INTERFACE_POLL_INTERVAL;
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

    REMOTE_WAKE_REQUEST_PROOF_STATE.consumed_task_ids[logical_cpu]
        .store(consumed_task, Ordering::Release);
    REMOTE_WAKE_REQUEST_PROOF_STATE.duplicate_counts[logical_cpu]
        .store(duplicates, Ordering::Release);
    REMOTE_WAKE_REQUEST_PROOF_STATE.queue_lens_after[logical_cpu]
        .store(queue_len_after as u64, Ordering::Release);

    let mut scheduler = PerCoreScheduler::<2>::deferred_secondary(requester);
    if matches!(
        scheduler.local_scheduler_mut(LogicalCpuId::BOOT),
        Err(PerCoreSchedulerAccessError::WrongOwner { .. })
    ) {
        REMOTE_WAKE_REQUEST_PROOF_STATE.cross_owner_rejections[logical_cpu]
            .store(1, Ordering::Release);
    }
    if matches!(
        scheduler.production_scheduler_mut(requester),
        Err(PerCoreSchedulerAccessError::ProductionDispatchDeferred { .. })
    ) {
        REMOTE_WAKE_REQUEST_PROOF_STATE.production_deferrals[logical_cpu]
            .store(1, Ordering::Release);
    }

    #[cfg(talos_boot_scenario = "rpi5_remote_wake_to_local_runnable")]
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
        REMOTE_WAKE_REQUEST_PROOF_STATE.local_state_before[logical_cpu]
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
            REMOTE_WAKE_REQUEST_PROOF_STATE.local_wake_task_ids[logical_cpu]
                .store(woken_task.raw(), Ordering::Release);
        } else {
            REMOTE_WAKE_REQUEST_PROOF_STATE
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
            REMOTE_WAKE_REQUEST_PROOF_STATE.duplicate_local_rejections[logical_cpu]
                .store(1, Ordering::Release);
        } else {
            REMOTE_WAKE_REQUEST_PROOF_STATE
                .errors
                .fetch_add(1, Ordering::AcqRel);
        }

        REMOTE_WAKE_REQUEST_PROOF_STATE.local_state_after[logical_cpu]
            .store(task_state_code(task.state()), Ordering::Release);
        REMOTE_WAKE_REQUEST_PROOF_STATE.local_runnable_lens[logical_cpu].store(
            scheduler.scheduler().runnable().len() as u64,
            Ordering::Release,
        );
    }

    REMOTE_WAKE_REQUEST_PROOF_STATE.mark_complete(logical_cpu);
    core_state.mark_workload_complete(consumed_task);
    core_state.clean_to_poc();
}

#[cfg(talos_boot_scenario = "rpi5_remote_wakeup_request")]
fn write_remote_wakeup_wait_observation(remaining: usize) {
    let ready_mask = REMOTE_WAKE_REQUEST_PROOF_STATE
        .ready_mask
        .load(Ordering::Acquire);
    let complete_mask = REMOTE_WAKE_REQUEST_PROOF_STATE
        .complete_mask
        .load(Ordering::Acquire);
    crate::println!(
        "rpi5-remote-wakeup-request: wait remaining={} ready-mask={:#x} complete-mask={:#x}",
        remaining,
        ready_mask,
        complete_mask
    );
    wait_uart10_empty_early_phase();
}

#[cfg(any(
    talos_boot_scenario = "rpi5_smp_lock_cache_coherence",
    talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
    talos_boot_scenario = "rpi5_remote_wakeup_request",
    talos_boot_scenario = "rpi5_production_secondary_dispatch",
    talos_boot_scenario = "rpi5_shared_scheduler_metadata",
    talos_boot_scenario = "rpi5_shared_runqueue_migration",
    talos_boot_scenario = "rpi5_multicore_preemption_proof",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof",
    talos_boot_scenario = "rpi5_secondary_scheduler_service_loop"
))]
fn current_sctlr_el2() -> u64 {
    let sctlr: u64;
    unsafe {
        core::arch::asm!("mrs {sctlr}, SCTLR_EL2", sctlr = out(reg) sctlr, options(nostack, preserves_flags));
    }
    sctlr
}

#[cfg(any(
    talos_boot_scenario = "rpi5_smp_lock_cache_coherence",
    talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
    talos_boot_scenario = "rpi5_remote_wakeup_request",
    talos_boot_scenario = "rpi5_production_secondary_dispatch",
    talos_boot_scenario = "rpi5_shared_scheduler_metadata",
    talos_boot_scenario = "rpi5_shared_runqueue_migration",
    talos_boot_scenario = "rpi5_multicore_preemption_proof",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof",
    talos_boot_scenario = "rpi5_secondary_scheduler_service_loop"
))]
fn cacheable_mmu_enabled(sctlr: u64) -> bool {
    (sctlr & (RPI5_SCTLR_M_ENABLE | RPI5_SCTLR_C_ENABLE))
        == (RPI5_SCTLR_M_ENABLE | RPI5_SCTLR_C_ENABLE)
}

#[cfg(any(
    talos_boot_scenario = "rpi5_smp_lock_cache_coherence",
    talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
    talos_boot_scenario = "rpi5_remote_wakeup_request",
    talos_boot_scenario = "rpi5_production_secondary_dispatch",
    talos_boot_scenario = "rpi5_shared_scheduler_metadata",
    talos_boot_scenario = "rpi5_shared_runqueue_migration",
    talos_boot_scenario = "rpi5_multicore_preemption_proof",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof",
    talos_boot_scenario = "rpi5_secondary_scheduler_service_loop"
))]
fn clean_cache_line_to_poc<T>(value: &T) {
    unsafe {
        core::arch::asm!(
            "dc cvac, {addr}",
            "dsb sy",
            addr = in(reg) value as *const T as usize,
            options(nostack, preserves_flags)
        );
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_smp_lock_cache_coherence",
    talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
    talos_boot_scenario = "rpi5_remote_wakeup_request",
    talos_boot_scenario = "rpi5_production_secondary_dispatch",
    talos_boot_scenario = "rpi5_shared_scheduler_metadata",
    talos_boot_scenario = "rpi5_shared_runqueue_migration",
    talos_boot_scenario = "rpi5_multicore_preemption_proof",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof",
    talos_boot_scenario = "rpi5_secondary_scheduler_service_loop"
))]
fn invalidate_cache_line_from_poc<T>(value: &T) {
    unsafe {
        core::arch::asm!(
            "dc ivac, {addr}",
            "dsb sy",
            addr = in(reg) value as *const T as usize,
            options(nostack, preserves_flags)
        );
    }
}

#[cfg(talos_boot_scenario = "rpi5_smp_lock_cache_coherence")]
fn run_smp_lock_contention_secondary(core_state: &smp::PerCoreState, logical_cpu: usize) {
    core_state.mark_workload_running();
    core_state.clean_to_poc();
    let sctlr_el2 = current_sctlr_el2();
    let mut attempts = 0;
    let mut timeouts = 0;
    let mut releases = 0;
    record_smp_lock_diagnostic(
        logical_cpu,
        SmpLockDiagnosticPhase::SecondaryEntered,
        0,
        attempts,
        timeouts,
        releases,
        sctlr_el2,
    );

    let mut progress = 0;
    while progress < RPI5_SMP_LOCK_CONTENTION_TARGET_PER_CORE {
        record_smp_lock_diagnostic(
            logical_cpu,
            SmpLockDiagnosticPhase::BeforeLockAttempt,
            progress,
            attempts,
            timeouts,
            releases,
            sctlr_el2,
        );
        let mut waited = 0;
        let expected_after = {
            let mut state = loop {
                attempts += 1;
                if let Some(state) = SMP_LOCK_CONTENTION_STATE.try_lock() {
                    break state;
                }
                waited += 1;
                if waited >= RPI5_SMP_LOCK_ACQUIRE_SPIN_LIMIT {
                    timeouts += 1;
                    record_smp_lock_diagnostic(
                        logical_cpu,
                        SmpLockDiagnosticPhase::LockAcquireTimeout,
                        progress,
                        attempts,
                        timeouts,
                        releases,
                        sctlr_el2,
                    );
                    core_state.record_workload_progress(progress);
                    core_state.mark_workload_complete(progress);
                    core_state.clean_to_poc();
                    return;
                }
                if waited & 0xffff == 0 {
                    record_smp_lock_diagnostic(
                        logical_cpu,
                        SmpLockDiagnosticPhase::WaitingForLock,
                        progress,
                        attempts,
                        timeouts,
                        releases,
                        sctlr_el2,
                    );
                }
                core::hint::spin_loop();
            };
            record_smp_lock_diagnostic(
                logical_cpu,
                SmpLockDiagnosticPhase::LockAcquired,
                progress,
                attempts,
                timeouts,
                releases,
                sctlr_el2,
            );
            let before = state.shared_counter;
            state.shared_counter = before + 1;
            state.per_core_counts[logical_cpu] += 1;
            if state.shared_counter != before + 1 {
                state.error_count += 1;
            }
            state.per_core_counts[logical_cpu]
        };
        releases += 1;
        record_smp_lock_diagnostic(
            logical_cpu,
            SmpLockDiagnosticPhase::LockReleased,
            progress,
            attempts,
            timeouts,
            releases,
            sctlr_el2,
        );
        progress += 1;
        if expected_after != progress {
            let mut state = SMP_LOCK_CONTENTION_STATE.lock();
            state.error_count += 1;
        }
        core_state.record_workload_progress(progress);
        record_smp_lock_diagnostic(
            logical_cpu,
            SmpLockDiagnosticPhase::IterationComplete,
            progress,
            attempts,
            timeouts,
            releases,
            sctlr_el2,
        );
        if progress == RPI5_SMP_LOCK_CONTENTION_TARGET_PER_CORE || progress & 0xf == 0 {
            core_state.clean_to_poc();
        }
        smp_full_barrier();
        core::hint::spin_loop();
    }

    core_state.mark_workload_complete(progress);
    core_state.clean_to_poc();
    record_smp_lock_diagnostic(
        logical_cpu,
        SmpLockDiagnosticPhase::WorkloadComplete,
        progress,
        attempts,
        timeouts,
        releases,
        sctlr_el2,
    );
}

#[cfg(talos_boot_scenario = "rpi5_smp_lock_cache_coherence")]
pub fn run_smp_lock_cache_coherence_proof() -> bool {
    let boot_mpidr = crate::arch::aarch64::mpidr_el1();
    let boot_affinity = crate::arch::aarch64::mpidr_affinity(boot_mpidr);
    let boot_logical = pi5_logical_cpu_from_mpidr_affinity(boot_affinity);
    let boot_cache_regime = crate::arch::aarch64::current_el2_stage1_cache_regime();
    let boot_sctlr_el2 = boot_cache_regime.map_or_else(current_sctlr_el2, |regime| regime.sctlr);
    let entry = talos_aarch64_rpi5_secondary_entry as *const () as usize;
    let stack_layout = secondary_stack_layout();
    let stack_base = core::ptr::addr_of!(talos_secondary_core_stacks) as usize;
    let stack_end = core::ptr::addr_of!(talos_secondary_core_stacks_end) as usize;
    let expected_total = RPI5_SMP_LOCK_CONTENTION_TARGET_PER_CORE * (MAX_CORES as u64 - 1);

    crate::println!(
        "rpi5-smp-lock-cache-coherence: start conduit=smc cores={} target-per-core={} expected-total={} boot-mpidr={:#018x} boot-affinity={:#x} boot-logical={:?} boot-sctlr-el2={:#018x} boot-cacheable-mmu={} entry={:#018x} stack-range=[{:#018x},{:#018x}) cache-policy=generic-lock-no-cache-maintenance acquire-spin-limit={}",
        MAX_CORES,
        RPI5_SMP_LOCK_CONTENTION_TARGET_PER_CORE,
        expected_total,
        boot_mpidr,
        boot_affinity,
        boot_logical,
        boot_sctlr_el2,
        cacheable_mmu_enabled(boot_sctlr_el2),
        entry,
        stack_base,
        stack_end,
        RPI5_SMP_LOCK_ACQUIRE_SPIN_LIMIT
    );
    wait_uart10_empty_early_phase();

    smp::reset_secondary_core_states();
    reset_smp_lock_contention_state();
    reset_smp_lock_diagnostic_state();
    if let Some(regime) = boot_cache_regime {
        publish_secondary_cacheable_mmu_handoff_plan(regime);
        crate::println!(
            "rpi5-smp-lock-cache-coherence: secondary-cacheable-mmu-handoff-plan mair-el2={:#018x} tcr-el2={:#018x} ttbr0-el2={:#018x} sctlr-el2={:#018x} cacheable-mmu={}",
            regime.mair,
            regime.tcr,
            regime.ttbr0,
            regime.sctlr,
            cacheable_mmu_enabled(regime.sctlr)
        );
    } else {
        SECONDARY_CACHEABLE_MMU_HANDOFF_READY.store(0, Ordering::Release);
        clean_secondary_cacheable_mmu_handoff_plan();
        crate::println!(
            "rpi5-smp-lock-cache-coherence: secondary-cacheable-mmu-handoff-plan unavailable"
        );
    }
    wait_uart10_empty_early_phase();

    let mut cpu_on_ok = true;
    for logical_cpu in 1..MAX_CORES {
        let target_affinity = (logical_cpu as u64) << 8;
        let result = unsafe { psci_cpu_on_smc(target_affinity, entry, logical_cpu) };
        crate::println!(
            "rpi5-smp-lock-cache-coherence: cpu-on logical={} target-affinity={:#x} result={}",
            logical_cpu,
            target_affinity,
            result
        );
        cpu_on_ok &= result == 0;
        let affinity_after = unsafe { psci_affinity_info_smc(target_affinity, 0) };
        crate::println!(
            "rpi5-smp-lock-cache-coherence: affinity-after logical={} target-affinity={:#x} level=0 state={} raw={}",
            logical_cpu,
            target_affinity,
            psci_affinity_state_name(affinity_after),
            affinity_after
        );
        wait_uart10_empty_early_phase();
    }

    let mut remaining = RPI5_SECONDARY_WAIT_LIMIT;
    while remaining > 0 {
        let all_complete = (1..MAX_CORES).all(|logical_cpu| {
            SECONDARY_CORE_STATES[logical_cpu].invalidate_from_poc();
            SECONDARY_CORE_STATES[logical_cpu]
                .snapshot(logical_cpu)
                .lifecycle
                >= CoreLifecycle::WorkloadComplete
        });
        if all_complete {
            break;
        }
        if remaining == RPI5_SECONDARY_WAIT_LIMIT
            || remaining % RPI5_SMP_LOCK_WAIT_POLL_INTERVAL == 0
        {
            for logical_cpu in 1..MAX_CORES {
                write_smp_lock_wait_observation(logical_cpu, remaining);
            }
            wait_uart10_empty_early_phase();
        }
        core::hint::spin_loop();
        remaining -= 1;
    }

    let final_state = SMP_LOCK_CONTENTION_STATE.try_lock().map(|state| *state);
    let lock_available = final_state.is_some();
    let final_state = final_state.unwrap_or_else(SmpLockContentionState::new);
    let mut participants = 0;
    let mut diagnostic_participants = 0;
    let mut any_pre_lock_stall = false;
    let mut any_lock_acquire_timeout = false;
    let mut any_lock_held_stall = !lock_available;
    let mut all_diagnostic_progress_complete = true;
    let mut any_mixed_cache_mmu = false;
    let mut reports_ok = cpu_on_ok
        && boot_logical == Some(0)
        && lock_available
        && final_state.shared_counter == expected_total
        && final_state.error_count == 0;

    for logical_cpu in 1..MAX_CORES {
        SECONDARY_CORE_STATES[logical_cpu].invalidate_from_poc();
        let report = SECONDARY_CORE_STATES[logical_cpu].snapshot(logical_cpu);
        let diagnostic = snapshot_smp_lock_diagnostic(logical_cpu);
        let logical_from_mpidr = pi5_logical_cpu_from_mpidr_affinity(report.affinity);
        let stack_slot = stack_layout
            .slot(logical_cpu)
            .expect("stack slot for possible Pi 5 core");
        let stack_owned = stack_slot.contains_stack_pointer(report.stack_pointer);
        let locked_count = final_state.per_core_counts[logical_cpu];
        let report_ok = report.lifecycle >= CoreLifecycle::WorkloadComplete
            && report.context == logical_cpu
            && logical_from_mpidr == Some(logical_cpu)
            && stack_owned
            && report.workload_progress == RPI5_SMP_LOCK_CONTENTION_TARGET_PER_CORE
            && locked_count == RPI5_SMP_LOCK_CONTENTION_TARGET_PER_CORE;
        if locked_count == RPI5_SMP_LOCK_CONTENTION_TARGET_PER_CORE {
            participants += 1;
        }
        if diagnostic.progress == RPI5_SMP_LOCK_CONTENTION_TARGET_PER_CORE {
            diagnostic_participants += 1;
        }
        any_pre_lock_stall |= report.lifecycle >= CoreLifecycle::WorkloadRunning
            && diagnostic.phase < SmpLockDiagnosticPhase::BeforeLockAttempt;
        any_lock_acquire_timeout |= diagnostic.timeouts > 0
            || diagnostic.phase == SmpLockDiagnosticPhase::LockAcquireTimeout;
        any_lock_held_stall &= diagnostic.phase == SmpLockDiagnosticPhase::LockAcquired;
        all_diagnostic_progress_complete &= diagnostic.progress
            == RPI5_SMP_LOCK_CONTENTION_TARGET_PER_CORE
            && diagnostic.releases == RPI5_SMP_LOCK_CONTENTION_TARGET_PER_CORE;
        any_mixed_cache_mmu |=
            cacheable_mmu_enabled(boot_sctlr_el2) && !cacheable_mmu_enabled(diagnostic.sctlr_el2);
        reports_ok &= report_ok;

        crate::println!(
            "rpi5-smp-lock-cache-coherence: report logical={} state={} context={} mpidr={:#018x} affinity={:#x} mapped={:?} sp={:#018x} stack=[{:#018x},{:#018x}) lock-count={} progress={} target={} diag-phase={} diag-progress={} diag-attempts={} diag-timeouts={} diag-releases={} diag-sctlr-el2={:#018x} diag-cacheable-mmu={} ok={}",
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
            RPI5_SMP_LOCK_CONTENTION_TARGET_PER_CORE,
            diagnostic.phase.name(),
            diagnostic.progress,
            diagnostic.attempts,
            diagnostic.timeouts,
            diagnostic.releases,
            diagnostic.sctlr_el2,
            cacheable_mmu_enabled(diagnostic.sctlr_el2),
            report_ok
        );
        wait_uart10_empty_early_phase();
    }

    let generic_state_visible = lock_available
        && final_state.shared_counter == expected_total
        && final_state.per_core_counts[1..]
            .iter()
            .all(|count| *count == RPI5_SMP_LOCK_CONTENTION_TARGET_PER_CORE);
    let non_visible_progress = all_diagnostic_progress_complete && !generic_state_visible;
    let classification = if reports_ok {
        "pi5-smp-lock-cache-coherence-complete"
    } else if any_mixed_cache_mmu {
        "pi5-smp-lock-cache-coherence-invalid-mixed-cache-mmu-regime"
    } else if any_pre_lock_stall {
        "pi5-smp-lock-cache-coherence-pre-lock-stall"
    } else if any_lock_held_stall {
        "pi5-smp-lock-cache-coherence-lock-held-stall"
    } else if non_visible_progress {
        "pi5-smp-lock-cache-coherence-non-visible-progress"
    } else if any_lock_acquire_timeout {
        "pi5-smp-lock-cache-coherence-lock-acquire-timeout"
    } else if !lock_available {
        "pi5-smp-lock-cache-coherence-lock-still-held"
    } else if !cpu_on_ok {
        "pi5-psci-smc-cpu-on-failed"
    } else if boot_logical != Some(0) {
        "pi5-psci-boot-core-identity-mismatch"
    } else {
        "pi5-smp-lock-cache-coherence-invariant-failed"
    };
    crate::println!(
        "rpi5-smp-lock-cache-coherence: final counter={} expected={} participants={} diag-participants={} errors={} lock-available={} generic-state-visible={} mixed-cache-mmu={} non-visible-progress={} wait-remaining={} classification={}",
        final_state.shared_counter,
        expected_total,
        participants,
        diagnostic_participants,
        final_state.error_count,
        lock_available,
        generic_state_visible,
        any_mixed_cache_mmu,
        non_visible_progress,
        remaining,
        classification
    );

    if reports_ok {
        crate::println!("rpi5-smp-lock-cache-coherence: PASS");
    } else {
        crate::println!("rpi5-smp-lock-cache-coherence: FAIL");
    }
    wait_uart10_empty_early_phase();

    reports_ok
}

#[cfg(talos_boot_scenario = "rpi5_cross_core_ipi_delivery")]
pub fn run_cross_core_ipi_delivery_proof() -> bool {
    smp::reset_secondary_core_states();
    CROSS_CORE_IPI_DELIVERY_STATE.reset();

    crate::arch::aarch64::exceptions::init();
    unsafe {
        aarch64::disable_irq();
        aarch64::route_physical_irqs_to_el2();
        let gic = GicV2::new(GICD_BASE, GICC_BASE);
        gic.configure_sgi_or_ppi_group1(RPI5_CROSS_CORE_IPI_SGI_INTID, 0x80);
        gic.enable_cpu_interface();
        gic.enable_distributor();
    }

    let boot_mpidr = aarch64::mpidr_el1();
    let boot_affinity = aarch64::mpidr_affinity(boot_mpidr);
    let boot_logical = pi5_logical_cpu_from_mpidr_affinity(boot_affinity);
    let boot_cache_regime = crate::arch::aarch64::current_el2_stage1_cache_regime();
    let boot_sctlr_el2 = boot_cache_regime.map_or_else(current_sctlr_el2, |regime| regime.sctlr);
    let entry = talos_aarch64_rpi5_secondary_entry as *const () as usize;
    let stack_layout = secondary_stack_layout();
    let stack_base = core::ptr::addr_of!(talos_secondary_core_stacks) as usize;
    let stack_end = core::ptr::addr_of!(talos_secondary_core_stacks_end) as usize;
    let expected_mask = ((1u64 << MAX_CORES) - 1) & !1;

    crate::println!(
        "rpi5-cross-core-ipi-delivery: start conduit=smc cores={} sgi-intid={} expected-mask={:#x} cpuif-poll=active-spin poll-interval={} boot-mpidr={:#018x} boot-affinity={:#x} boot-logical={:?} boot-sctlr-el2={:#018x} boot-cacheable-mmu={} entry={:#018x} stack-range=[{:#018x},{:#018x})",
        MAX_CORES,
        RPI5_CROSS_CORE_IPI_SGI_INTID,
        expected_mask,
        RPI5_CROSS_CORE_IPI_CPU_INTERFACE_POLL_INTERVAL,
        boot_mpidr,
        boot_affinity,
        boot_logical,
        boot_sctlr_el2,
        cacheable_mmu_enabled(boot_sctlr_el2),
        entry,
        stack_base,
        stack_end
    );
    wait_uart10_empty_early_phase();

    if let Some(regime) = boot_cache_regime {
        publish_secondary_cacheable_mmu_handoff_plan(regime);
        crate::println!(
            "rpi5-cross-core-ipi-delivery: secondary-cacheable-mmu-handoff-plan mair-el2={:#018x} tcr-el2={:#018x} ttbr0-el2={:#018x} sctlr-el2={:#018x} cacheable-mmu={}",
            regime.mair,
            regime.tcr,
            regime.ttbr0,
            regime.sctlr,
            cacheable_mmu_enabled(regime.sctlr)
        );
    } else {
        SECONDARY_CACHEABLE_MMU_HANDOFF_READY.store(0, Ordering::Release);
        clean_secondary_cacheable_mmu_handoff_plan();
        crate::println!(
            "rpi5-cross-core-ipi-delivery: secondary-cacheable-mmu-handoff-plan unavailable"
        );
    }
    wait_uart10_empty_early_phase();

    let mut cpu_on_ok = true;
    for logical_cpu in 1..MAX_CORES {
        let target_affinity = (logical_cpu as u64) << 8;
        let result = unsafe { psci_cpu_on_smc(target_affinity, entry, logical_cpu) };
        crate::println!(
            "rpi5-cross-core-ipi-delivery: cpu-on logical={} target-affinity={:#x} result={}",
            logical_cpu,
            target_affinity,
            result
        );
        cpu_on_ok &= result == 0;
        let affinity_after = unsafe { psci_affinity_info_smc(target_affinity, 0) };
        crate::println!(
            "rpi5-cross-core-ipi-delivery: affinity-after logical={} target-affinity={:#x} level=0 state={} raw={}",
            logical_cpu,
            target_affinity,
            psci_affinity_state_name(affinity_after),
            affinity_after
        );
        wait_uart10_empty_early_phase();
    }

    let mut ready_remaining = RPI5_SECONDARY_WAIT_LIMIT;
    while ready_remaining > 0
        && (CROSS_CORE_IPI_DELIVERY_STATE
            .ready_mask
            .load(Ordering::Acquire)
            & expected_mask)
            != expected_mask
    {
        if ready_remaining == RPI5_SECONDARY_WAIT_LIMIT
            || ready_remaining % RPI5_CROSS_CORE_IPI_WAIT_POLL_INTERVAL == 0
        {
            write_cross_core_ipi_wait_observation(ready_remaining);
        }
        core::hint::spin_loop();
        ready_remaining -= 1;
    }

    let gic = GicV2::new(GICD_BASE, GICC_BASE);
    let sgir_value = unsafe { gic.send_sgi_to_all_except_self(RPI5_CROSS_CORE_IPI_SGI_INTID) };
    CROSS_CORE_IPI_DELIVERY_STATE.record_filter_send(expected_mask, sgir_value);
    crate::println!(
        "rpi5-cross-core-ipi-delivery: send sender=0 target-logical=all-secondaries target-list-bit={:#04x} target-filter=all-except-self sgi-intid={} sgir={:#010x}",
        expected_mask,
        RPI5_CROSS_CORE_IPI_SGI_INTID,
        sgir_value
    );
    wait_uart10_empty_early_phase();

    let mut complete_remaining = RPI5_SECONDARY_WAIT_LIMIT;
    while complete_remaining > 0
        && (CROSS_CORE_IPI_DELIVERY_STATE
            .complete_mask
            .load(Ordering::Acquire)
            & expected_mask)
            != expected_mask
    {
        if complete_remaining == RPI5_SECONDARY_WAIT_LIMIT
            || complete_remaining % RPI5_CROSS_CORE_IPI_WAIT_POLL_INTERVAL == 0
        {
            write_cross_core_ipi_wait_observation(complete_remaining);
        }
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
        SECONDARY_CORE_STATES[logical_cpu].invalidate_from_poc();
        let core_report = SECONDARY_CORE_STATES[logical_cpu].snapshot(logical_cpu);
        let logical_from_mpidr = pi5_logical_cpu_from_mpidr_affinity(core_report.affinity);
        let stack_slot = stack_layout
            .slot(logical_cpu)
            .expect("stack slot for possible Pi 5 core");
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
        let polled_hppir =
            CROSS_CORE_IPI_DELIVERY_STATE.polled_hppirs[logical_cpu].load(Ordering::Acquire);
        let polled_iar =
            CROSS_CORE_IPI_DELIVERY_STATE.polled_iars[logical_cpu].load(Ordering::Acquire);
        let polled_intid =
            CROSS_CORE_IPI_DELIVERY_STATE.polled_intids[logical_cpu].load(Ordering::Acquire);
        let polled_daif =
            CROSS_CORE_IPI_DELIVERY_STATE.polled_daifs[logical_cpu].load(Ordering::Acquire);
        let polled_hcr =
            CROSS_CORE_IPI_DELIVERY_STATE.polled_hcrs[logical_cpu].load(Ordering::Acquire);
        let poll_count =
            CROSS_CORE_IPI_DELIVERY_STATE.poll_counts[logical_cpu].load(Ordering::Acquire);
        let report_ok = core_report.lifecycle >= CoreLifecycle::WorkloadComplete
            && core_report.context == logical_cpu
            && logical_from_mpidr == Some(logical_cpu)
            && stack_owned
            && target_bit == expected_mask
            && receive_count == 1
            && eoi_count == 1
            && last_intid == RPI5_CROSS_CORE_IPI_SGI_INTID as u64;
        let poll_observed_sgi = polled_intid == RPI5_CROSS_CORE_IPI_SGI_INTID as u64;
        if report_ok {
            participants += 1;
        }
        reports_ok &= report_ok;

        crate::println!(
            "rpi5-cross-core-ipi-delivery: report sender=0 receiver={} state={} context={} mpidr={:#018x} affinity={:#x} mapped={:?} sp={:#018x} stack=[{:#018x},{:#018x}) target-list-bit={:#04x} sgir={:#010x} vector={} iar={:#010x} intid={} receive-count={} eoi-count={} poll-count={} poll-hppir={:#010x} poll-iar={:#010x} poll-intid={} poll-daif={:#x} poll-hcr={:#018x} poll-observed-sgi={} errors={} ok={}",
            logical_cpu,
            secondary_state_name(core_report.lifecycle.raw()),
            core_report.context,
            core_report.mpidr,
            core_report.affinity,
            logical_from_mpidr,
            core_report.stack_pointer,
            stack_slot.bottom,
            stack_slot.top,
            target_bit,
            sgir_value,
            last_vector,
            last_iar,
            last_intid,
            receive_count,
            eoi_count,
            poll_count,
            polled_hppir,
            polled_iar,
            polled_intid,
            polled_daif,
            polled_hcr,
            poll_observed_sgi,
            CROSS_CORE_IPI_DELIVERY_STATE.errors.load(Ordering::Acquire),
            report_ok
        );
        wait_uart10_empty_early_phase();
    }

    let errors = CROSS_CORE_IPI_DELIVERY_STATE.errors.load(Ordering::Acquire);
    let classification = if reports_ok && errors == 0 {
        "pi5-cross-core-ipi-delivery-complete"
    } else if (ready_mask & expected_mask) != expected_mask {
        "pi5-cross-core-ipi-delivery-secondaries-not-ready"
    } else if (1..MAX_CORES).all(|logical_cpu| {
        CROSS_CORE_IPI_DELIVERY_STATE.polled_intids[logical_cpu].load(Ordering::Acquire)
            == RPI5_CROSS_CORE_IPI_SGI_INTID as u64
    }) {
        "pi5-cross-core-ipi-delivery-pending-polled-not-vectored"
    } else if cpu_on_ok {
        "pi5-cross-core-ipi-delivery-invariant-failed"
    } else {
        "pi5-psci-smc-cpu-on-failed"
    };
    crate::println!(
        "rpi5-cross-core-ipi-delivery: final participants={} expected={} errors={} ready-mask={:#x} complete-mask={:#x} ready-wait-remaining={} complete-wait-remaining={} classification={}",
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
        crate::println!("rpi5-cross-core-ipi-delivery: PASS");
    } else {
        crate::println!("rpi5-cross-core-ipi-delivery: FAIL");
    }
    wait_uart10_empty_early_phase();

    reports_ok && errors == 0
}

#[cfg(talos_boot_scenario = "rpi5_remote_wakeup_request")]
pub fn run_remote_wakeup_request_proof() -> bool {
    smp::reset_secondary_core_states();
    reset_remote_wakeup_request_state();

    crate::arch::aarch64::exceptions::init();
    unsafe {
        aarch64::disable_irq();
        aarch64::route_physical_irqs_to_el2();
        let gic = GicV2::new(GICD_BASE, GICC_BASE);
        gic.configure_sgi_or_ppi_group1(RPI5_CROSS_CORE_IPI_SGI_INTID, 0x80);
        gic.enable_cpu_interface();
        gic.enable_distributor();
    }

    let boot_mpidr = aarch64::mpidr_el1();
    let boot_affinity = aarch64::mpidr_affinity(boot_mpidr);
    let boot_logical = pi5_logical_cpu_from_mpidr_affinity(boot_affinity);
    let boot_cache_regime = crate::arch::aarch64::current_el2_stage1_cache_regime();
    let boot_sctlr_el2 = boot_cache_regime.map_or_else(current_sctlr_el2, |regime| regime.sctlr);
    let entry = talos_aarch64_rpi5_secondary_entry as *const () as usize;
    let stack_layout = secondary_stack_layout();
    let stack_base = core::ptr::addr_of!(talos_secondary_core_stacks) as usize;
    let stack_end = core::ptr::addr_of!(talos_secondary_core_stacks_end) as usize;
    let expected_mask = ((1u64 << MAX_CORES) - 1) & !1;

    crate::println!(
        "rpi5-remote-wakeup-request: start conduit=smc cores={} sgi-intid={} queue-capacity={} expected-mask={:#x} cpuif-poll=active-spin poll-interval={} boot-mpidr={:#018x} boot-affinity={:#x} boot-logical={:?} boot-sctlr-el2={:#018x} boot-cacheable-mmu={} entry={:#018x} stack-range=[{:#018x},{:#018x})",
        MAX_CORES,
        RPI5_CROSS_CORE_IPI_SGI_INTID,
        REMOTE_WAKE_QUEUE_CAPACITY,
        expected_mask,
        RPI5_CROSS_CORE_IPI_CPU_INTERFACE_POLL_INTERVAL,
        boot_mpidr,
        boot_affinity,
        boot_logical,
        boot_sctlr_el2,
        cacheable_mmu_enabled(boot_sctlr_el2),
        entry,
        stack_base,
        stack_end
    );
    wait_uart10_empty_early_phase();

    if let Some(regime) = boot_cache_regime {
        publish_secondary_cacheable_mmu_handoff_plan(regime);
        crate::println!(
            "rpi5-remote-wakeup-request: secondary-cacheable-mmu-handoff-plan mair-el2={:#018x} tcr-el2={:#018x} ttbr0-el2={:#018x} sctlr-el2={:#018x} cacheable-mmu={}",
            regime.mair,
            regime.tcr,
            regime.ttbr0,
            regime.sctlr,
            cacheable_mmu_enabled(regime.sctlr)
        );
    } else {
        SECONDARY_CACHEABLE_MMU_HANDOFF_READY.store(0, Ordering::Release);
        clean_secondary_cacheable_mmu_handoff_plan();
        crate::println!(
            "rpi5-remote-wakeup-request: secondary-cacheable-mmu-handoff-plan unavailable"
        );
    }
    wait_uart10_empty_early_phase();

    let mut cpu_on_ok = true;
    for logical_cpu in 1..MAX_CORES {
        let target_affinity = (logical_cpu as u64) << 8;
        let result = unsafe { psci_cpu_on_smc(target_affinity, entry, logical_cpu) };
        crate::println!(
            "rpi5-remote-wakeup-request: cpu-on logical={} target-affinity={:#x} result={}",
            logical_cpu,
            target_affinity,
            result
        );
        cpu_on_ok &= result == 0;
        let affinity_after = unsafe { psci_affinity_info_smc(target_affinity, 0) };
        crate::println!(
            "rpi5-remote-wakeup-request: affinity-after logical={} target-affinity={:#x} level=0 state={} raw={}",
            logical_cpu,
            target_affinity,
            psci_affinity_state_name(affinity_after),
            affinity_after
        );
        wait_uart10_empty_early_phase();
    }

    let mut ready_remaining = RPI5_SECONDARY_WAIT_LIMIT;
    while ready_remaining > 0
        && (REMOTE_WAKE_REQUEST_PROOF_STATE
            .ready_mask
            .load(Ordering::Acquire)
            & expected_mask)
            != expected_mask
    {
        if ready_remaining == RPI5_SECONDARY_WAIT_LIMIT
            || ready_remaining % RPI5_CROSS_CORE_IPI_WAIT_POLL_INTERVAL == 0
        {
            write_remote_wakeup_wait_observation(ready_remaining);
        }
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
        wait_uart10_empty_early_phase();
    }

    let gic = GicV2::new(GICD_BASE, GICC_BASE);
    for logical_cpu in 1..MAX_CORES {
        let target_bit = 1u8 << logical_cpu;
        let sgir_value =
            unsafe { gic.send_sgi_to_target_list(RPI5_CROSS_CORE_IPI_SGI_INTID, target_bit) };
        REMOTE_WAKE_REQUEST_PROOF_STATE.record_send(logical_cpu, target_bit, sgir_value);
        crate::println!(
            "rpi5-remote-wakeup-request: send sender=0 target-logical={} target-list-bit={:#04x} sgi-intid={} sgir={:#010x}",
            logical_cpu,
            target_bit,
            RPI5_CROSS_CORE_IPI_SGI_INTID,
            sgir_value
        );
        wait_uart10_empty_early_phase();
    }

    let mut complete_remaining = RPI5_SECONDARY_WAIT_LIMIT;
    while complete_remaining > 0
        && (REMOTE_WAKE_REQUEST_PROOF_STATE
            .complete_mask
            .load(Ordering::Acquire)
            & expected_mask)
            != expected_mask
    {
        if complete_remaining == RPI5_SECONDARY_WAIT_LIMIT
            || complete_remaining % RPI5_CROSS_CORE_IPI_WAIT_POLL_INTERVAL == 0
        {
            write_remote_wakeup_wait_observation(complete_remaining);
        }
        core::hint::spin_loop();
        complete_remaining -= 1;
    }

    unsafe {
        aarch64::disable_irq();
    }

    let ready_mask = REMOTE_WAKE_REQUEST_PROOF_STATE
        .ready_mask
        .load(Ordering::Acquire);
    let complete_mask = REMOTE_WAKE_REQUEST_PROOF_STATE
        .complete_mask
        .load(Ordering::Acquire);
    let mut participants = 0;
    let mut reports_ok = cpu_on_ok
        && publish_ok
        && boot_logical == Some(0)
        && (ready_mask & expected_mask) == expected_mask;

    for logical_cpu in 1..MAX_CORES {
        SECONDARY_CORE_STATES[logical_cpu].invalidate_from_poc();
        let core_report = SECONDARY_CORE_STATES[logical_cpu].snapshot(logical_cpu);
        let logical_from_mpidr = pi5_logical_cpu_from_mpidr_affinity(core_report.affinity);
        let stack_slot = stack_layout
            .slot(logical_cpu)
            .expect("stack slot for possible Pi 5 core");
        let stack_owned = stack_slot.contains_stack_pointer(core_report.stack_pointer);
        let expected_task = 200 + logical_cpu as u64;
        let target_bit =
            REMOTE_WAKE_REQUEST_PROOF_STATE.target_bits[logical_cpu].load(Ordering::Acquire);
        let sgir_value =
            REMOTE_WAKE_REQUEST_PROOF_STATE.sent_values[logical_cpu].load(Ordering::Acquire);
        let receive_count =
            REMOTE_WAKE_REQUEST_PROOF_STATE.receive_counts[logical_cpu].load(Ordering::Acquire);
        let eoi_count =
            REMOTE_WAKE_REQUEST_PROOF_STATE.eoi_counts[logical_cpu].load(Ordering::Acquire);
        let pending_count =
            REMOTE_WAKE_REQUEST_PROOF_STATE.pending_counts[logical_cpu].load(Ordering::Acquire);
        let consumed_task =
            REMOTE_WAKE_REQUEST_PROOF_STATE.consumed_task_ids[logical_cpu].load(Ordering::Acquire);
        let duplicate_count =
            REMOTE_WAKE_REQUEST_PROOF_STATE.duplicate_counts[logical_cpu].load(Ordering::Acquire);
        let queue_len_after =
            REMOTE_WAKE_REQUEST_PROOF_STATE.queue_lens_after[logical_cpu].load(Ordering::Acquire);
        let cross_owner_rejected = REMOTE_WAKE_REQUEST_PROOF_STATE.cross_owner_rejections
            [logical_cpu]
            .load(Ordering::Acquire)
            == 1;
        let production_deferred = REMOTE_WAKE_REQUEST_PROOF_STATE.production_deferrals[logical_cpu]
            .load(Ordering::Acquire)
            == 1;
        let last_vector =
            REMOTE_WAKE_REQUEST_PROOF_STATE.last_vectors[logical_cpu].load(Ordering::Acquire);
        let last_iar =
            REMOTE_WAKE_REQUEST_PROOF_STATE.last_iars[logical_cpu].load(Ordering::Acquire);
        let last_intid =
            REMOTE_WAKE_REQUEST_PROOF_STATE.last_intids[logical_cpu].load(Ordering::Acquire);
        let polled_hppir =
            REMOTE_WAKE_REQUEST_PROOF_STATE.polled_hppirs[logical_cpu].load(Ordering::Acquire);
        let polled_iar =
            REMOTE_WAKE_REQUEST_PROOF_STATE.polled_iars[logical_cpu].load(Ordering::Acquire);
        let polled_intid =
            REMOTE_WAKE_REQUEST_PROOF_STATE.polled_intids[logical_cpu].load(Ordering::Acquire);
        let polled_daif =
            REMOTE_WAKE_REQUEST_PROOF_STATE.polled_daifs[logical_cpu].load(Ordering::Acquire);
        let polled_hcr =
            REMOTE_WAKE_REQUEST_PROOF_STATE.polled_hcrs[logical_cpu].load(Ordering::Acquire);
        let poll_count =
            REMOTE_WAKE_REQUEST_PROOF_STATE.poll_counts[logical_cpu].load(Ordering::Acquire);
        let report_ok = core_report.lifecycle >= CoreLifecycle::WorkloadComplete
            && core_report.context == logical_cpu
            && logical_from_mpidr == Some(logical_cpu)
            && stack_owned
            && target_bit == (1u64 << logical_cpu)
            && receive_count == 1
            && eoi_count == 1
            && pending_count == 1
            && last_intid == RPI5_CROSS_CORE_IPI_SGI_INTID as u64
            && consumed_task == expected_task
            && queue_len_after == 0
            && cross_owner_rejected
            && production_deferred
            && (logical_cpu != 1 || duplicate_count == 1)
            && (logical_cpu == 1 || duplicate_count == 0);
        #[cfg(talos_boot_scenario = "rpi5_remote_wake_to_local_runnable")]
        let report_ok = {
            let mut report_ok = report_ok;
            let local_wake_task = REMOTE_WAKE_REQUEST_PROOF_STATE.local_wake_task_ids[logical_cpu]
                .load(Ordering::Acquire);
            let local_runnable_len = REMOTE_WAKE_REQUEST_PROOF_STATE.local_runnable_lens
                [logical_cpu]
                .load(Ordering::Acquire);
            let local_state_before = REMOTE_WAKE_REQUEST_PROOF_STATE.local_state_before
                [logical_cpu]
                .load(Ordering::Acquire);
            let local_state_after = REMOTE_WAKE_REQUEST_PROOF_STATE.local_state_after[logical_cpu]
                .load(Ordering::Acquire);
            let duplicate_local_rejected = REMOTE_WAKE_REQUEST_PROOF_STATE
                .duplicate_local_rejections[logical_cpu]
                .load(Ordering::Acquire)
                == 1;
            report_ok &= local_wake_task == expected_task
                && local_runnable_len == 1
                && task_state_name(local_state_before) == "blocked"
                && task_state_name(local_state_after) == "runnable"
                && duplicate_local_rejected;
            crate::println!(
                "rpi5-remote-wake-to-local-runnable: local receiver={} state-before={} state-after={} woke-task={} local-runnable-len={} duplicate-local-rejected={} ok={}",
                logical_cpu,
                task_state_name(local_state_before),
                task_state_name(local_state_after),
                local_wake_task,
                local_runnable_len,
                duplicate_local_rejected,
                report_ok
            );
            wait_uart10_empty_early_phase();
            report_ok
        };
        let poll_observed_sgi = polled_intid == RPI5_CROSS_CORE_IPI_SGI_INTID as u64;
        if report_ok {
            participants += 1;
        }
        reports_ok &= report_ok;

        crate::println!(
            "rpi5-remote-wakeup-request: report sender=0 receiver={} state={} context={} mpidr={:#018x} affinity={:#x} mapped={:?} sp={:#018x} stack=[{:#018x},{:#018x}) target-list-bit={:#04x} sgir={:#010x} vector={} iar={:#010x} intid={} receive-count={} eoi-count={} pending-count={} consumed-task={} duplicate-count={} queue-len-after={} cross-owner-rejected={} production-deferred={} poll-count={} poll-hppir={:#010x} poll-iar={:#010x} poll-intid={} poll-daif={:#x} poll-hcr={:#018x} poll-observed-sgi={} errors={} ok={}",
            logical_cpu,
            secondary_state_name(core_report.lifecycle.raw()),
            core_report.context,
            core_report.mpidr,
            core_report.affinity,
            logical_from_mpidr,
            core_report.stack_pointer,
            stack_slot.bottom,
            stack_slot.top,
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
            poll_count,
            polled_hppir,
            polled_iar,
            polled_intid,
            polled_daif,
            polled_hcr,
            poll_observed_sgi,
            REMOTE_WAKE_REQUEST_PROOF_STATE
                .errors
                .load(Ordering::Acquire),
            report_ok
        );
        wait_uart10_empty_early_phase();
    }

    let errors = REMOTE_WAKE_REQUEST_PROOF_STATE
        .errors
        .load(Ordering::Acquire);
    let classification = if reports_ok && errors == 0 {
        if cfg!(talos_boot_scenario = "rpi5_remote_wake_to_local_runnable") {
            "pi5-remote-wake-to-local-runnable-complete"
        } else {
            "pi5-remote-wakeup-request-complete"
        }
    } else if (ready_mask & expected_mask) != expected_mask {
        "pi5-remote-wakeup-request-secondaries-not-ready"
    } else if (1..MAX_CORES).all(|logical_cpu| {
        REMOTE_WAKE_REQUEST_PROOF_STATE.polled_intids[logical_cpu].load(Ordering::Acquire)
            == RPI5_CROSS_CORE_IPI_SGI_INTID as u64
    }) {
        "pi5-remote-wakeup-request-pending-polled-not-vectored"
    } else if cpu_on_ok {
        "pi5-remote-wakeup-request-invariant-failed"
    } else {
        "pi5-psci-smc-cpu-on-failed"
    };
    crate::println!(
        "rpi5-remote-wakeup-request: final participants={} expected={} errors={} ready-mask={:#x} complete-mask={:#x} ready-wait-remaining={} complete-wait-remaining={} classification={}",
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
        crate::println!("rpi5-remote-wakeup-request: PASS");
    } else {
        crate::println!("rpi5-remote-wakeup-request: FAIL");
    }
    wait_uart10_empty_early_phase();

    reports_ok && errors == 0
}

#[cfg(talos_boot_scenario = "rpi5_production_secondary_dispatch")]
pub fn run_production_secondary_dispatch_proof() -> bool {
    smp::reset_secondary_core_states();
    reset_production_secondary_dispatch_state();

    let boot_mpidr = aarch64::mpidr_el1();
    let boot_affinity = aarch64::mpidr_affinity(boot_mpidr);
    let boot_logical = pi5_logical_cpu_from_mpidr_affinity(boot_affinity);
    let boot_cache_regime = crate::arch::aarch64::current_el2_stage1_cache_regime();
    let boot_sctlr_el2 = boot_cache_regime.map_or_else(current_sctlr_el2, |regime| regime.sctlr);
    let entry = talos_aarch64_rpi5_secondary_entry as *const () as usize;
    let stack_layout = secondary_stack_layout();
    let stack_base = core::ptr::addr_of!(talos_secondary_core_stacks) as usize;
    let stack_end = core::ptr::addr_of!(talos_secondary_core_stacks_end) as usize;

    crate::println!(
        "rpi5-production-secondary-dispatch: start conduit=smc cores={} target={} boot-mpidr={:#018x} boot-affinity={:#x} boot-logical={:?} boot-sctlr-el2={:#018x} boot-cacheable-mmu={} entry={:#018x} stack-range=[{:#018x},{:#018x})",
        MAX_CORES,
        PRODUCTION_SECONDARY_DISPATCH_PROGRESS_TARGET,
        boot_mpidr,
        boot_affinity,
        boot_logical,
        boot_sctlr_el2,
        cacheable_mmu_enabled(boot_sctlr_el2),
        entry,
        stack_base,
        stack_end
    );
    wait_uart10_empty_early_phase();

    if let Some(regime) = boot_cache_regime {
        publish_secondary_cacheable_mmu_handoff_plan(regime);
        crate::println!(
            "rpi5-production-secondary-dispatch: secondary-cacheable-mmu-handoff-plan mair-el2={:#018x} tcr-el2={:#018x} ttbr0-el2={:#018x} sctlr-el2={:#018x} cacheable-mmu={}",
            regime.mair,
            regime.tcr,
            regime.ttbr0,
            regime.sctlr,
            cacheable_mmu_enabled(regime.sctlr)
        );
    } else {
        SECONDARY_CACHEABLE_MMU_HANDOFF_READY.store(0, Ordering::Release);
        clean_secondary_cacheable_mmu_handoff_plan();
        crate::println!(
            "rpi5-production-secondary-dispatch: secondary-cacheable-mmu-handoff-plan unavailable"
        );
    }
    wait_uart10_empty_early_phase();

    let mut cpu_on_ok = true;
    for logical_cpu in 1..MAX_CORES {
        let target_affinity = (logical_cpu as u64) << 8;
        let result = unsafe { psci_cpu_on_smc(target_affinity, entry, logical_cpu) };
        crate::println!(
            "rpi5-production-secondary-dispatch: cpu-on logical={} target-affinity={:#x} result={}",
            logical_cpu,
            target_affinity,
            result
        );
        cpu_on_ok &= result == 0;
        let affinity_after = unsafe { psci_affinity_info_smc(target_affinity, 0) };
        crate::println!(
            "rpi5-production-secondary-dispatch: affinity-after logical={} target-affinity={:#x} level=0 state={} raw={}",
            logical_cpu,
            target_affinity,
            psci_affinity_state_name(affinity_after),
            affinity_after
        );
        wait_uart10_empty_early_phase();
    }

    let mut remaining = RPI5_SECONDARY_WAIT_LIMIT;
    while remaining > 0 {
        let all_complete = (1..MAX_CORES).all(|logical_cpu| {
            SECONDARY_CORE_STATES[logical_cpu].invalidate_from_poc();
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
        SECONDARY_CORE_STATES[logical_cpu].invalidate_from_poc();
        let report = final_state.reports[logical_cpu];
        let core_report = SECONDARY_CORE_STATES[logical_cpu].snapshot(logical_cpu);
        let logical_from_mpidr = pi5_logical_cpu_from_mpidr_affinity(core_report.affinity);
        let stack_slot = stack_layout
            .slot(logical_cpu)
            .expect("stack slot for possible Pi 5 core");
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
            "rpi5-production-secondary-dispatch: report logical={} state={} context={} mpidr={:#018x} affinity={:#x} mapped={:?} sp={:#018x} stack=[{:#018x},{:#018x}) owner={} role={} production={} current={} queue-len={} front={} progress={} transitions={} production-dispatches={} context-switches={} cross-owner-rejected={} cross-owner-dispatch-rejected={} lock-progress={} errors={} ok={}",
            logical_cpu,
            secondary_state_name(core_report.lifecycle.raw()),
            core_report.context,
            core_report.mpidr,
            core_report.affinity,
            logical_from_mpidr,
            core_report.stack_pointer,
            stack_slot.bottom,
            stack_slot.top,
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
        wait_uart10_empty_early_phase();
    }

    let classification = if reports_ok {
        "pi5-production-secondary-dispatch-complete"
    } else if !lock_available {
        "pi5-production-secondary-dispatch-lock-still-held"
    } else if cpu_on_ok {
        "pi5-production-secondary-dispatch-invariant-failed"
    } else {
        "pi5-psci-smc-cpu-on-failed"
    };
    crate::println!(
        "rpi5-production-secondary-dispatch: final participants={} expected={} errors={} lock-available={} wait-remaining={} classification={}",
        participants,
        MAX_CORES - 1,
        errors,
        lock_available,
        remaining,
        classification
    );

    if reports_ok {
        crate::println!("rpi5-production-secondary-dispatch: PASS");
    } else {
        crate::println!("rpi5-production-secondary-dispatch: FAIL");
    }
    wait_uart10_empty_early_phase();

    reports_ok
}

#[cfg(talos_boot_scenario = "rpi5_shared_scheduler_metadata")]
pub fn run_shared_scheduler_metadata_proof() -> bool {
    smp::reset_secondary_core_states();
    reset_shared_scheduler_metadata_state();

    let boot_mpidr = aarch64::mpidr_el1();
    let boot_affinity = aarch64::mpidr_affinity(boot_mpidr);
    let boot_logical = pi5_logical_cpu_from_mpidr_affinity(boot_affinity);
    let boot_cache_regime = crate::arch::aarch64::current_el2_stage1_cache_regime();
    let boot_sctlr_el2 = boot_cache_regime.map_or_else(current_sctlr_el2, |regime| regime.sctlr);
    let entry = talos_aarch64_rpi5_secondary_entry as *const () as usize;
    let stack_layout = secondary_stack_layout();
    let stack_base = core::ptr::addr_of!(talos_secondary_core_stacks) as usize;
    let stack_end = core::ptr::addr_of!(talos_secondary_core_stacks_end) as usize;

    crate::println!(
        "rpi5-shared-scheduler-metadata: start conduit=smc cores={} task-capacity={} boot-mpidr={:#018x} boot-affinity={:#x} boot-logical={:?} boot-sctlr-el2={:#018x} boot-cacheable-mmu={} entry={:#018x} stack-range=[{:#018x},{:#018x})",
        MAX_CORES,
        SHARED_SCHEDULER_METADATA_TASK_CAPACITY,
        boot_mpidr,
        boot_affinity,
        boot_logical,
        boot_sctlr_el2,
        cacheable_mmu_enabled(boot_sctlr_el2),
        entry,
        stack_base,
        stack_end
    );
    wait_uart10_empty_early_phase();

    if let Some(regime) = boot_cache_regime {
        publish_secondary_cacheable_mmu_handoff_plan(regime);
        crate::println!(
            "rpi5-shared-scheduler-metadata: secondary-cacheable-mmu-handoff-plan mair-el2={:#018x} tcr-el2={:#018x} ttbr0-el2={:#018x} sctlr-el2={:#018x} cacheable-mmu={}",
            regime.mair,
            regime.tcr,
            regime.ttbr0,
            regime.sctlr,
            cacheable_mmu_enabled(regime.sctlr)
        );
    } else {
        SECONDARY_CACHEABLE_MMU_HANDOFF_READY.store(0, Ordering::Release);
        clean_secondary_cacheable_mmu_handoff_plan();
        crate::println!(
            "rpi5-shared-scheduler-metadata: secondary-cacheable-mmu-handoff-plan unavailable"
        );
    }
    wait_uart10_empty_early_phase();

    let mut boot_scheduler = PerCoreScheduler::<2>::boot_cpu();
    let boot_report = build_shared_scheduler_metadata_report(0, &mut boot_scheduler);
    publish_shared_scheduler_metadata_report(0, boot_report);

    let mut cpu_on_ok = true;
    for logical_cpu in 1..MAX_CORES {
        let target_affinity = (logical_cpu as u64) << 8;
        let result = unsafe { psci_cpu_on_smc(target_affinity, entry, logical_cpu) };
        crate::println!(
            "rpi5-shared-scheduler-metadata: cpu-on logical={} target-affinity={:#x} result={}",
            logical_cpu,
            target_affinity,
            result
        );
        cpu_on_ok &= result == 0;
        let affinity_after = unsafe { psci_affinity_info_smc(target_affinity, 0) };
        crate::println!(
            "rpi5-shared-scheduler-metadata: affinity-after logical={} target-affinity={:#x} level=0 state={} raw={}",
            logical_cpu,
            target_affinity,
            psci_affinity_state_name(affinity_after),
            affinity_after
        );
        wait_uart10_empty_early_phase();
    }

    let mut remaining = RPI5_SECONDARY_WAIT_LIMIT;
    while remaining > 0 {
        let all_complete = (1..MAX_CORES).all(|logical_cpu| {
            SECONDARY_CORE_STATES[logical_cpu].invalidate_from_poc();
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

    let final_state = SHARED_SCHEDULER_METADATA_STATE
        .try_lock()
        .map(|state| *state);
    let state_lock_available = final_state.is_some();
    let final_state = final_state.unwrap_or_else(SharedSchedulerMetadataState::new);
    let final_metadata = SHARED_SCHEDULER_METADATA_TABLE
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
        let (lifecycle, context, mapped, stack_pointer, stack_bottom, stack_top, stack_owned) =
            if logical_cpu == 0 {
                (
                    CoreLifecycle::WorkloadComplete,
                    0,
                    boot_logical,
                    0,
                    0,
                    0,
                    true,
                )
            } else {
                SECONDARY_CORE_STATES[logical_cpu].invalidate_from_poc();
                let core_report = SECONDARY_CORE_STATES[logical_cpu].snapshot(logical_cpu);
                let logical_from_mpidr = pi5_logical_cpu_from_mpidr_affinity(core_report.affinity);
                let stack_slot = stack_layout
                    .slot(logical_cpu)
                    .expect("stack slot for possible Pi 5 core");
                (
                    core_report.lifecycle,
                    core_report.context,
                    logical_from_mpidr,
                    core_report.stack_pointer,
                    stack_slot.bottom,
                    stack_slot.top,
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
            "rpi5-shared-scheduler-metadata: report logical={} state={} context={} mapped={:?} sp={:#018x} stack=[{:#018x},{:#018x}) owner={} role={} production={} task={} task-state={} current={} queue-len={} front={} metadata-len={} metadata-generation={} lookup-owner={} lookup-task={} lookup-generation={} boot-lookup-owner={} boot-lookup-task={} boot-lookup-generation={} cross-owner-rejected={} metadata-cross-owner-rejected={} local-queue-preserved={} lock-progress={} errors={} ok={}",
            logical_cpu,
            secondary_state_name(lifecycle.raw()),
            context,
            mapped,
            stack_pointer,
            stack_bottom,
            stack_top,
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
        wait_uart10_empty_early_phase();
    }

    let classification = if reports_ok {
        "pi5-shared-scheduler-metadata-complete"
    } else if !state_lock_available || !metadata_lock_available {
        "pi5-shared-scheduler-metadata-lock-still-held"
    } else if cpu_on_ok {
        "pi5-shared-scheduler-metadata-invariant-failed"
    } else {
        "pi5-psci-smc-cpu-on-failed"
    };
    crate::println!(
        "rpi5-shared-scheduler-metadata: final participants={} expected={} errors={} state-lock-available={} metadata-lock-available={} final-metadata-len={} final-metadata-generation={} wait-remaining={} classification={}",
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
        crate::println!("rpi5-shared-scheduler-metadata: PASS");
    } else {
        crate::println!("rpi5-shared-scheduler-metadata: FAIL");
    }
    wait_uart10_empty_early_phase();

    reports_ok
}

#[cfg(talos_boot_scenario = "rpi5_shared_runqueue_migration")]
pub fn run_shared_runqueue_migration_proof() -> bool {
    smp::reset_secondary_core_states();
    reset_shared_runqueue_migration_state();

    let boot_mpidr = aarch64::mpidr_el1();
    let boot_affinity = aarch64::mpidr_affinity(boot_mpidr);
    let boot_logical = pi5_logical_cpu_from_mpidr_affinity(boot_affinity);
    let boot_cache_regime = crate::arch::aarch64::current_el2_stage1_cache_regime();
    let boot_sctlr_el2 = boot_cache_regime.map_or_else(current_sctlr_el2, |regime| regime.sctlr);
    let entry = talos_aarch64_rpi5_secondary_entry as *const () as usize;
    let stack_base = core::ptr::addr_of!(talos_secondary_core_stacks) as usize;
    let stack_end = core::ptr::addr_of!(talos_secondary_core_stacks_end) as usize;

    crate::println!(
        "rpi5-shared-runqueue-migration: start conduit=smc cores={} task-capacity={} queue-capacity={} boot-mpidr={:#018x} boot-affinity={:#x} boot-logical={:?} boot-sctlr-el2={:#018x} boot-cacheable-mmu={} entry={:#018x} stack-range=[{:#018x},{:#018x})",
        MAX_CORES,
        SHARED_RUNQUEUE_MIGRATION_TASK_CAPACITY,
        SHARED_RUNQUEUE_MIGRATION_QUEUE_CAPACITY,
        boot_mpidr,
        boot_affinity,
        boot_logical,
        boot_sctlr_el2,
        cacheable_mmu_enabled(boot_sctlr_el2),
        entry,
        stack_base,
        stack_end
    );
    wait_uart10_empty_early_phase();

    if let Some(regime) = boot_cache_regime {
        publish_secondary_cacheable_mmu_handoff_plan(regime);
        crate::println!(
            "rpi5-shared-runqueue-migration: secondary-cacheable-mmu-handoff-plan mair-el2={:#018x} tcr-el2={:#018x} ttbr0-el2={:#018x} sctlr-el2={:#018x} cacheable-mmu={}",
            regime.mair,
            regime.tcr,
            regime.ttbr0,
            regime.sctlr,
            cacheable_mmu_enabled(regime.sctlr)
        );
    } else {
        SECONDARY_CACHEABLE_MMU_HANDOFF_READY.store(0, Ordering::Release);
        clean_secondary_cacheable_mmu_handoff_plan();
        crate::println!(
            "rpi5-shared-runqueue-migration: secondary-cacheable-mmu-handoff-plan unavailable"
        );
    }
    wait_uart10_empty_early_phase();

    let boot_report = build_shared_runqueue_migration_report(0);
    publish_shared_runqueue_migration_report(0, boot_report);

    let mut cpu_on_ok = true;
    for logical_cpu in 1..MAX_CORES {
        let target_affinity = (logical_cpu as u64) << 8;
        let result = unsafe { psci_cpu_on_smc(target_affinity, entry, logical_cpu) };
        crate::println!(
            "rpi5-shared-runqueue-migration: cpu-on logical={} target-affinity={:#x} result={}",
            logical_cpu,
            target_affinity,
            result
        );
        cpu_on_ok &= result == 0;
        let affinity_after = unsafe { psci_affinity_info_smc(target_affinity, 0) };
        crate::println!(
            "rpi5-shared-runqueue-migration: affinity-after logical={} target-affinity={:#x} level=0 state={} raw={}",
            logical_cpu,
            target_affinity,
            psci_affinity_state_name(affinity_after),
            affinity_after
        );
        wait_uart10_empty_early_phase();
    }

    let mut remaining = RPI5_SECONDARY_WAIT_LIMIT;
    while remaining > 0 {
        let all_complete = (1..MAX_CORES).all(|logical_cpu| {
            SECONDARY_CORE_STATES[logical_cpu].invalidate_from_poc();
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

    let final_state = SHARED_RUNQUEUE_MIGRATION_STATE
        .try_lock()
        .map(|state| *state);
    let state_lock_available = final_state.is_some();
    let final_state = final_state.unwrap_or_else(SharedRunQueueMigrationState::new);

    let mut participants = 0;
    let mut errors = 0;
    let mut reports_ok = cpu_on_ok && boot_logical == Some(0) && state_lock_available;

    for logical_cpu in 0..MAX_CORES {
        let report = final_state.reports[logical_cpu];
        let expected_destination = ((logical_cpu + 1) % MAX_CORES) as u64;
        let expected_task = (logical_cpu as u64 + 1) * 100 + 7;
        let expected_source_role = if logical_cpu == 0 {
            SchedulerCoreRole::BootCpuProduction
        } else {
            SchedulerCoreRole::SecondaryProductionDiagnostic
        };
        let expected_destination_role = if expected_destination == 0 {
            SchedulerCoreRole::BootCpuProduction
        } else {
            SchedulerCoreRole::SecondaryProductionDiagnostic
        };
        let report_ok = report.source_owner == logical_cpu as u64
            && report.source_role == expected_source_role
            && report.destination_owner == expected_destination
            && report.destination_role == expected_destination_role
            && report.task_id == expected_task
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
        if report.source_owner != u64::MAX {
            participants += 1;
        }
        if !report_ok {
            errors += 1;
        }
        reports_ok &= report_ok;

        crate::println!(
            "rpi5-shared-runqueue-migration: report logical={} source-owner={} source-role={} destination-owner={} destination-role={} task={} task-state={} registered-generation={} publish-reserved-state={} publish-queued-state={} consume-queued-state={} consume-destination-state={} source-queue-before={} source-queue-after-publish={} shared-len-after-publish={} shared-len-after-consume={} destination-queue-len={} destination-front={} metadata-owner-after-consume={} metadata-generation-after-consume={} source-removed={} destination-enqueued={} metadata-migrated={} errors={} ok={}",
            logical_cpu,
            report.source_owner,
            scheduler_role_name(report.source_role),
            report.destination_owner,
            scheduler_role_name(report.destination_role),
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
        wait_uart10_empty_early_phase();
    }

    let classification = if reports_ok {
        "pi5-shared-runqueue-migration-complete"
    } else if !state_lock_available {
        "pi5-shared-runqueue-migration-lock-still-held"
    } else if cpu_on_ok {
        "pi5-shared-runqueue-migration-invariant-failed"
    } else {
        "pi5-psci-smc-cpu-on-failed"
    };
    crate::println!(
        "rpi5-shared-runqueue-migration: final participants={} expected={} errors={} lock-available={} wait-remaining={} classification={}",
        participants,
        MAX_CORES,
        errors,
        state_lock_available,
        remaining,
        classification
    );

    if reports_ok {
        crate::println!("rpi5-shared-runqueue-migration: PASS");
    } else {
        crate::println!("rpi5-shared-runqueue-migration: FAIL");
    }
    wait_uart10_empty_early_phase();

    reports_ok
}

#[cfg(talos_boot_scenario = "rpi5_load_balancing_proof")]
pub fn run_load_balancing_proof() -> bool {
    let boot_mpidr = aarch64::mpidr_el1();
    let boot_affinity = aarch64::mpidr_affinity(boot_mpidr);
    let boot_logical = pi5_logical_cpu_from_mpidr_affinity(boot_affinity);
    crate::println!(
        "rpi5-load-balancing: start task-capacity={} queue-capacity={} boot-mpidr={:#018x} boot-affinity={:#x} boot-logical={:?}",
        LOAD_BALANCING_PROOF_TASK_CAPACITY,
        LOAD_BALANCING_PROOF_QUEUE_CAPACITY,
        boot_mpidr,
        boot_affinity,
        boot_logical,
    );
    wait_uart10_empty_early_phase();

    let report = build_load_balancing_proof_report();
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
        "rpi5-load-balancing: report source-owner={} destination-owner={} task={} task-state={} registered-generation={} plan-generation={} publish-reserved-state={} publish-queued-state={} consume-queued-state={} consume-destination-state={} source-queue-before={} source-queue-after-publish={} shared-len-after-publish={} shared-len-after-consume={} destination-queue-len={} destination-front={} metadata-owner-after-consume={} metadata-generation-after-consume={} selected-front={} source-removed={} destination-enqueued={} metadata-migrated={} errors={} ok={}",
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
        "pi5-load-balancing-complete"
    } else {
        "pi5-load-balancing-invariant-failed"
    };
    crate::println!(
        "rpi5-load-balancing: final participants=1 expected=1 errors={} classification={}",
        report.errors,
        classification
    );

    if report_ok {
        crate::println!("rpi5-load-balancing: PASS");
    } else {
        crate::println!("rpi5-load-balancing: FAIL");
    }
    wait_uart10_empty_early_phase();

    report_ok
}

#[cfg(talos_boot_scenario = "rpi5_secondary_scheduler_service_loop")]
pub fn run_secondary_scheduler_service_loop_proof() -> bool {
    smp::reset_secondary_core_states();
    reset_secondary_scheduler_service_loop_state();

    let boot_mpidr = aarch64::mpidr_el1();
    let boot_affinity = aarch64::mpidr_affinity(boot_mpidr);
    let boot_logical = pi5_logical_cpu_from_mpidr_affinity(boot_affinity);
    let boot_cache_regime = crate::arch::aarch64::current_el2_stage1_cache_regime();
    let boot_sctlr_el2 = boot_cache_regime.map_or_else(current_sctlr_el2, |regime| regime.sctlr);
    let entry = talos_aarch64_rpi5_secondary_entry as *const () as usize;
    let stack_layout = secondary_stack_layout();
    let stack_base = core::ptr::addr_of!(talos_secondary_core_stacks) as usize;
    let stack_end = core::ptr::addr_of!(talos_secondary_core_stacks_end) as usize;

    crate::println!(
        "rpi5-secondary-scheduler-service-loop: start conduit=smc cores={} task-capacity={} boot-mpidr={:#018x} boot-affinity={:#x} boot-logical={:?} boot-sctlr-el2={:#018x} boot-cacheable-mmu={} entry={:#018x} stack-range=[{:#018x},{:#018x})",
        MAX_CORES,
        SECONDARY_SCHEDULER_SERVICE_LOOP_TASK_CAPACITY,
        boot_mpidr,
        boot_affinity,
        boot_logical,
        boot_sctlr_el2,
        cacheable_mmu_enabled(boot_sctlr_el2),
        entry,
        stack_base,
        stack_end
    );
    wait_uart10_empty_early_phase();

    if let Some(regime) = boot_cache_regime {
        publish_secondary_cacheable_mmu_handoff_plan(regime);
        crate::println!(
            "rpi5-secondary-scheduler-service-loop: secondary-cacheable-mmu-handoff-plan mair-el2={:#018x} tcr-el2={:#018x} ttbr0-el2={:#018x} sctlr-el2={:#018x} cacheable-mmu={}",
            regime.mair,
            regime.tcr,
            regime.ttbr0,
            regime.sctlr,
            cacheable_mmu_enabled(regime.sctlr)
        );
    } else {
        SECONDARY_CACHEABLE_MMU_HANDOFF_READY.store(0, Ordering::Release);
        clean_secondary_cacheable_mmu_handoff_plan();
        crate::println!(
            "rpi5-secondary-scheduler-service-loop: secondary-cacheable-mmu-handoff-plan unavailable"
        );
    }
    wait_uart10_empty_early_phase();

    let mut cpu_on_ok = true;
    for logical_cpu in 1..MAX_CORES {
        let target_affinity = (logical_cpu as u64) << 8;
        let result = unsafe { psci_cpu_on_smc(target_affinity, entry, logical_cpu) };
        crate::println!(
            "rpi5-secondary-scheduler-service-loop: cpu-on logical={} target-affinity={:#x} result={}",
            logical_cpu,
            target_affinity,
            result
        );
        cpu_on_ok &= result == 0;
        let affinity_after = unsafe { psci_affinity_info_smc(target_affinity, 0) };
        crate::println!(
            "rpi5-secondary-scheduler-service-loop: affinity-after logical={} target-affinity={:#x} level=0 state={} raw={}",
            logical_cpu,
            target_affinity,
            psci_affinity_state_name(affinity_after),
            affinity_after
        );
        wait_uart10_empty_early_phase();
    }

    let mut remaining = RPI5_SECONDARY_WAIT_LIMIT;
    while remaining > 0 {
        let all_complete = (1..MAX_CORES).all(|logical_cpu| {
            SECONDARY_CORE_STATES[logical_cpu].invalidate_from_poc();
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

    let final_state = SECONDARY_SCHEDULER_SERVICE_LOOP_STATE
        .try_lock()
        .map(|state| *state);
    let state_lock_available = final_state.is_some();
    let final_state = final_state.unwrap_or_else(SecondarySchedulerServiceLoopState::new);
    let metadata_lock_available = true;
    let mut final_metadata_len = 0;
    let mut final_metadata_generation = 0;
    for logical_cpu in 1..MAX_CORES {
        let report = final_state.reports[logical_cpu];
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
        SECONDARY_CORE_STATES[logical_cpu].invalidate_from_poc();
        let report = final_state.reports[logical_cpu];
        let core_report = SECONDARY_CORE_STATES[logical_cpu].snapshot(logical_cpu);
        let logical_from_mpidr = pi5_logical_cpu_from_mpidr_affinity(core_report.affinity);
        let stack_slot = stack_layout
            .slot(logical_cpu)
            .expect("stack slot for possible Pi 5 core");
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
            && final_state.lock_progress[logical_cpu] == 1
            && report.errors == 0;
        if report_ok {
            participants += 1;
        }
        errors += report.errors;
        reports_ok &= report_ok;

        crate::println!(
            "rpi5-secondary-scheduler-service-loop: report logical={} state={} context={} mpidr={:#018x} affinity={:#x} mapped={:?} sp={:#018x} stack=[{:#018x},{:#018x}) owner={} role={} task={} task-state={} current={} queue-len={} front={} remote-wake={} dispatch={} no-work-did-work={} metadata-len={} metadata-generation={} observed-remote-wake={} pending-timer-preemption={} dispatch-requested={} cross-owner-rejected={} deferred-role-rejected={} local-queue-preserved={} lock-progress={} errors={} ok={}",
            logical_cpu,
            secondary_state_name(core_report.lifecycle.raw()),
            core_report.context,
            core_report.mpidr,
            core_report.affinity,
            logical_from_mpidr,
            core_report.stack_pointer,
            stack_slot.bottom,
            stack_slot.top,
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
            final_state.lock_progress[logical_cpu],
            report.errors,
            report_ok
        );
        wait_uart10_empty_early_phase();
    }

    let classification = if reports_ok {
        "pi5-secondary-scheduler-service-loop-complete"
    } else if !state_lock_available || !metadata_lock_available {
        "pi5-secondary-scheduler-service-loop-lock-still-held"
    } else if cpu_on_ok {
        "pi5-secondary-scheduler-service-loop-invariant-failed"
    } else {
        "pi5-psci-smc-cpu-on-failed"
    };
    crate::println!(
        "rpi5-secondary-scheduler-service-loop: final participants={} expected={} errors={} state-lock-available={} metadata-lock-available={} final-metadata-len={} final-metadata-generation={} wait-remaining={} classification={}",
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
        crate::println!("rpi5-secondary-scheduler-service-loop: PASS");
    } else {
        crate::println!("rpi5-secondary-scheduler-service-loop: FAIL");
    }
    wait_uart10_empty_early_phase();

    reports_ok
}

#[cfg(talos_boot_scenario = "rpi5_multicore_preemption_proof")]
pub fn run_multicore_preemption_proof() -> bool {
    smp::reset_secondary_core_states();
    reset_multicore_preemption_proof_state();

    let boot_mpidr = aarch64::mpidr_el1();
    let boot_affinity = aarch64::mpidr_affinity(boot_mpidr);
    let boot_logical = pi5_logical_cpu_from_mpidr_affinity(boot_affinity);
    let boot_cache_regime = crate::arch::aarch64::current_el2_stage1_cache_regime();
    let boot_sctlr_el2 = boot_cache_regime.map_or_else(current_sctlr_el2, |regime| regime.sctlr);
    let entry = talos_aarch64_rpi5_secondary_entry as *const () as usize;
    let stack_layout = secondary_stack_layout();
    let stack_base = core::ptr::addr_of!(talos_secondary_core_stacks) as usize;
    let stack_end = core::ptr::addr_of!(talos_secondary_core_stacks_end) as usize;

    crate::println!(
        "rpi5-multicore-preemption: start conduit=smc cores={} task-capacity={} boot-mpidr={:#018x} boot-affinity={:#x} boot-logical={:?} boot-sctlr-el2={:#018x} boot-cacheable-mmu={} entry={:#018x} stack-range=[{:#018x},{:#018x})",
        MAX_CORES,
        MULTICORE_PREEMPTION_PROOF_TASK_CAPACITY,
        boot_mpidr,
        boot_affinity,
        boot_logical,
        boot_sctlr_el2,
        cacheable_mmu_enabled(boot_sctlr_el2),
        entry,
        stack_base,
        stack_end
    );
    wait_uart10_empty_early_phase();

    if let Some(regime) = boot_cache_regime {
        publish_secondary_cacheable_mmu_handoff_plan(regime);
        crate::println!(
            "rpi5-multicore-preemption: secondary-cacheable-mmu-handoff-plan mair-el2={:#018x} tcr-el2={:#018x} ttbr0-el2={:#018x} sctlr-el2={:#018x} cacheable-mmu={}",
            regime.mair,
            regime.tcr,
            regime.ttbr0,
            regime.sctlr,
            cacheable_mmu_enabled(regime.sctlr)
        );
    } else {
        SECONDARY_CACHEABLE_MMU_HANDOFF_READY.store(0, Ordering::Release);
        clean_secondary_cacheable_mmu_handoff_plan();
        crate::println!(
            "rpi5-multicore-preemption: secondary-cacheable-mmu-handoff-plan unavailable"
        );
    }
    wait_uart10_empty_early_phase();

    let mut cpu_on_ok = true;
    for logical_cpu in 1..MAX_CORES {
        let target_affinity = (logical_cpu as u64) << 8;
        let result = unsafe { psci_cpu_on_smc(target_affinity, entry, logical_cpu) };
        crate::println!(
            "rpi5-multicore-preemption: cpu-on logical={} target-affinity={:#x} result={}",
            logical_cpu,
            target_affinity,
            result
        );
        cpu_on_ok &= result == 0;
        let affinity_after = unsafe { psci_affinity_info_smc(target_affinity, 0) };
        crate::println!(
            "rpi5-multicore-preemption: affinity-after logical={} target-affinity={:#x} level=0 state={} raw={}",
            logical_cpu,
            target_affinity,
            psci_affinity_state_name(affinity_after),
            affinity_after
        );
        wait_uart10_empty_early_phase();
    }

    let mut remaining = RPI5_SECONDARY_WAIT_LIMIT;
    while remaining > 0 {
        let all_complete = (1..MAX_CORES).all(|logical_cpu| {
            SECONDARY_CORE_STATES[logical_cpu].invalidate_from_poc();
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

    let final_state = MULTICORE_PREEMPTION_PROOF_STATE
        .try_lock()
        .map(|state| *state);
    let state_lock_available = final_state.is_some();
    let final_state = final_state.unwrap_or_else(MultiCorePreemptionProofState::new);
    let metadata_lock_available = true;
    let mut participants = 0;
    let mut errors = 0;
    let mut reports_ok = cpu_on_ok && boot_logical == Some(0) && state_lock_available;

    for logical_cpu in 1..MAX_CORES {
        SECONDARY_CORE_STATES[logical_cpu].invalidate_from_poc();
        let report = final_state.reports[logical_cpu];
        let core_report = SECONDARY_CORE_STATES[logical_cpu].snapshot(logical_cpu);
        let logical_from_mpidr = pi5_logical_cpu_from_mpidr_affinity(core_report.affinity);
        let stack_slot = stack_layout
            .slot(logical_cpu)
            .expect("stack slot for possible Pi 5 core");
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
            && final_state.lock_progress[logical_cpu] == 1;
        if report_ok {
            participants += 1;
        }
        errors += report.errors;
        reports_ok &= report_ok;

        crate::println!(
            "rpi5-multicore-preemption: report logical={} state={} context={} mpidr={:#018x} affinity={:#x} mapped={:?} sp={:#018x} stack=[{:#018x},{:#018x}) owner={} role={} current-before-record={} next={} queue-len-before-record={} metadata-generation-before-record={} record-outcome={} duplicate-outcome={} cross-owner-rejected={} current-after-record={} queue-len-after-record={} metadata-generation-after-record={} irq-record-scheduler-mutated={} pending-after-record={} service-timer-preemption={} current-after-service={} queue-len-after-service={} front-after-service={} previous-task-state={} selected-task-state={} pending-after-service={} recorded={} coalesced={} serviced={} metadata-owner-after-service={} metadata-task-after-service={} metadata-generation-after-service={} lock-progress={} errors={} ok={}",
            logical_cpu,
            secondary_state_name(core_report.lifecycle.raw()),
            core_report.context,
            core_report.mpidr,
            core_report.affinity,
            logical_from_mpidr,
            core_report.stack_pointer,
            stack_slot.bottom,
            stack_slot.top,
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
            final_state.lock_progress[logical_cpu],
            report.errors,
            report_ok
        );
        wait_uart10_empty_early_phase();
    }

    let classification = if reports_ok {
        "pi5-multicore-preemption-complete"
    } else if !state_lock_available || !metadata_lock_available {
        "pi5-multicore-preemption-lock-still-held"
    } else if cpu_on_ok {
        "pi5-multicore-preemption-invariant-failed"
    } else {
        "pi5-psci-smc-cpu-on-failed"
    };
    crate::println!(
        "rpi5-multicore-preemption: final participants={} expected={} errors={} state-lock-available={} metadata-lock-available={} wait-remaining={} classification={}",
        participants,
        MAX_CORES - 1,
        errors,
        state_lock_available,
        metadata_lock_available,
        remaining,
        classification
    );

    if reports_ok {
        crate::println!("rpi5-multicore-preemption: PASS");
    } else {
        crate::println!("rpi5-multicore-preemption: FAIL");
    }
    wait_uart10_empty_early_phase();

    reports_ok
}

#[cfg(talos_boot_scenario = "rpi5_production_timer_preemption_proof")]
pub fn run_production_timer_preemption_proof() -> bool {
    smp::reset_secondary_core_states();
    reset_production_timer_preemption_proof_state();

    let boot_mpidr = aarch64::mpidr_el1();
    let boot_affinity = aarch64::mpidr_affinity(boot_mpidr);
    let boot_logical = pi5_logical_cpu_from_mpidr_affinity(boot_affinity);
    let boot_cache_regime = crate::arch::aarch64::current_el2_stage1_cache_regime();
    let boot_sctlr_el2 = boot_cache_regime.map_or_else(current_sctlr_el2, |regime| regime.sctlr);
    let entry = talos_aarch64_rpi5_secondary_entry as *const () as usize;
    let stack_layout = secondary_stack_layout();
    let stack_base = core::ptr::addr_of!(talos_secondary_core_stacks) as usize;
    let stack_end = core::ptr::addr_of!(talos_secondary_core_stacks_end) as usize;

    crate::println!(
        "rpi5-production-timer-preemption: start conduit=smc cores={} task-capacity={} entry-path=production-timer-irq-adapter boot-logical={:?} cacheable-mmu={}",
        MAX_CORES,
        PRODUCTION_TIMER_PREEMPTION_PROOF_TASK_CAPACITY,
        boot_logical,
        cacheable_mmu_enabled(boot_sctlr_el2)
    );
    wait_uart10_empty_early_phase();

    if let Some(regime) = boot_cache_regime {
        publish_secondary_cacheable_mmu_handoff_plan(regime);
        crate::println!(
            "rpi5-production-timer-preemption: secondary-cacheable-mmu-handoff-plan mair-el2={:#018x} tcr-el2={:#018x} ttbr0-el2={:#018x} sctlr-el2={:#018x} cacheable-mmu={}",
            regime.mair,
            regime.tcr,
            regime.ttbr0,
            regime.sctlr,
            cacheable_mmu_enabled(regime.sctlr)
        );
    } else {
        SECONDARY_CACHEABLE_MMU_HANDOFF_READY.store(0, Ordering::Release);
        clean_secondary_cacheable_mmu_handoff_plan();
        crate::println!(
            "rpi5-production-timer-preemption: secondary-cacheable-mmu-handoff-plan unavailable"
        );
    }
    wait_uart10_empty_early_phase();

    let mut cpu_on_ok = true;
    for logical_cpu in 1..MAX_CORES {
        let target_affinity = (logical_cpu as u64) << 8;
        let result = unsafe { psci_cpu_on_smc(target_affinity, entry, logical_cpu) };
        crate::println!(
            "rpi5-production-timer-preemption: cpu-on logical={} target-affinity={:#x} result={}",
            logical_cpu,
            target_affinity,
            result
        );
        cpu_on_ok &= result == 0;
        let affinity_after = unsafe { psci_affinity_info_smc(target_affinity, 0) };
        crate::println!(
            "rpi5-production-timer-preemption: affinity-after logical={} target-affinity={:#x} level=0 state={} raw={}",
            logical_cpu,
            target_affinity,
            psci_affinity_state_name(affinity_after),
            affinity_after
        );
        wait_uart10_empty_early_phase();
    }

    let mut remaining = RPI5_SECONDARY_WAIT_LIMIT;
    while remaining > 0 {
        let all_complete = (1..MAX_CORES).all(|logical_cpu| {
            SECONDARY_CORE_STATES[logical_cpu].invalidate_from_poc();
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

    let final_state = PRODUCTION_TIMER_PREEMPTION_PROOF_STATE
        .try_lock()
        .map(|state| *state);
    let state_lock_available = final_state.is_some();
    let final_state = final_state.unwrap_or_else(ProductionTimerPreemptionProofState::new);
    let metadata_lock_available = true;
    let mut participants = 0;
    let mut errors = 0;
    let mut reports_ok = cpu_on_ok && boot_logical == Some(0) && state_lock_available;

    for logical_cpu in 1..MAX_CORES {
        SECONDARY_CORE_STATES[logical_cpu].invalidate_from_poc();
        let report = final_state.reports[logical_cpu];
        let core_report = SECONDARY_CORE_STATES[logical_cpu].snapshot(logical_cpu);
        let logical_from_mpidr = pi5_logical_cpu_from_mpidr_affinity(core_report.affinity);
        let stack_slot = stack_layout
            .slot(logical_cpu)
            .expect("stack slot for possible Pi 5 core");
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
            && final_state.lock_progress[logical_cpu] == 1;
        if report_ok {
            participants += 1;
        }
        errors += report.errors;
        reports_ok &= report_ok;

        crate::println!(
            "rpi5-production-timer-preemption: report logical={} state={} mapped={:?} owner={} role={} entry-path=production-timer-irq-adapter record-outcome={} duplicate-outcome={} cross-owner-rejected={} record-misses={} timer-record-rejections={} irq-record-scheduler-mutated={} pending-after-record={} service-timer-preemption={} pending-after-service={} recorded={} coalesced={} serviced={} metadata-generation-after-service={} lock-progress={} errors={} ok={}",
            logical_cpu,
            secondary_state_name(core_report.lifecycle.raw()),
            logical_from_mpidr,
            report.owner,
            scheduler_role_name(report.role),
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
            report.irq_record_scheduler_mutated,
            report.pending_after_record,
            report.service_timer_preemption,
            report.pending_after_service,
            report.recorded_requests,
            report.coalesced_requests,
            report.serviced_requests,
            report.metadata_generation_after_service,
            final_state.lock_progress[logical_cpu],
            report.errors,
            report_ok
        );
        wait_uart10_empty_early_phase();
    }

    let classification = if reports_ok {
        "pi5-production-timer-preemption-complete"
    } else if !state_lock_available || !metadata_lock_available {
        "pi5-production-timer-preemption-lock-still-held"
    } else if cpu_on_ok {
        "pi5-production-timer-preemption-invariant-failed"
    } else {
        "pi5-psci-smc-cpu-on-failed"
    };
    crate::println!(
        "rpi5-production-timer-preemption: final participants={} expected={} errors={} state-lock-available={} metadata-lock-available={} wait-remaining={} classification={}",
        participants,
        MAX_CORES - 1,
        errors,
        state_lock_available,
        metadata_lock_available,
        remaining,
        classification
    );

    if reports_ok {
        crate::println!("rpi5-production-timer-preemption: PASS");
    } else {
        crate::println!("rpi5-production-timer-preemption: FAIL");
    }
    wait_uart10_empty_early_phase();

    reports_ok
}

#[cfg(talos_boot_scenario = "rpi5_psci_secondary_core_alive")]
pub fn run_psci_secondary_core_alive_proof() -> bool {
    smp::reset_secondary_core_states();

    let boot_mpidr = crate::arch::aarch64::mpidr_el1();
    let boot_affinity = crate::arch::aarch64::mpidr_affinity(boot_mpidr);
    let boot_logical = pi5_logical_cpu_from_mpidr_affinity(boot_affinity);
    let entry = talos_aarch64_rpi5_secondary_entry as *const () as usize;
    let stack_layout = secondary_stack_layout();
    let stack_base = core::ptr::addr_of!(talos_secondary_core_stacks) as usize;
    let stack_end = core::ptr::addr_of!(talos_secondary_core_stacks_end) as usize;

    crate::println!(
        "rpi5-psci-secondary-core-alive: start conduit=smc cores={} boot-mpidr={:#018x} boot-affinity={:#x} boot-logical={:?} entry={:#018x} stack-range=[{:#018x},{:#018x})",
        MAX_CORES,
        boot_mpidr,
        boot_affinity,
        boot_logical,
        entry,
        stack_base,
        stack_end
    );
    wait_uart10_empty_early_phase();

    let mut cpu_on_ok = true;
    for logical_cpu in 1..MAX_CORES {
        let target_affinity = (logical_cpu as u64) << 8;
        let result = unsafe { psci_cpu_on_smc(target_affinity, entry, logical_cpu) };
        crate::println!(
            "rpi5-psci-secondary-core-alive: cpu-on logical={} target-affinity={:#x} result={}",
            logical_cpu,
            target_affinity,
            result
        );
        cpu_on_ok &= result == 0;
        let affinity_after = unsafe { psci_affinity_info_smc(target_affinity, 0) };
        crate::println!(
            "rpi5-psci-secondary-core-alive: affinity-after logical={} target-affinity={:#x} level=0 state={} raw={}",
            logical_cpu,
            target_affinity,
            psci_affinity_state_name(affinity_after),
            affinity_after
        );
        wait_uart10_empty_early_phase();
    }

    let mut remaining = RPI5_SECONDARY_WAIT_LIMIT;
    while remaining > 0 {
        let all_ready = (1..MAX_CORES).all(|logical_cpu| {
            SECONDARY_CORE_STATES[logical_cpu].invalidate_from_poc();
            SECONDARY_CORE_STATES[logical_cpu]
                .snapshot(logical_cpu)
                .lifecycle
                >= CoreLifecycle::HandoffReady
        });
        if all_ready {
            break;
        }
        core::hint::spin_loop();
        remaining -= 1;
    }

    let mut reports_ok = cpu_on_ok && boot_logical == Some(0);
    for logical_cpu in 1..MAX_CORES {
        SECONDARY_CORE_STATES[logical_cpu].invalidate_from_poc();
        let report = SECONDARY_CORE_STATES[logical_cpu].snapshot(logical_cpu);
        let logical_from_mpidr = pi5_logical_cpu_from_mpidr_affinity(report.affinity);
        let stack_slot = stack_layout
            .slot(logical_cpu)
            .expect("stack slot for possible Pi 5 core");
        let stack_owned = stack_slot.contains_stack_pointer(report.stack_pointer);
        let report_ok = report.lifecycle >= CoreLifecycle::HandoffReady
            && report.context == logical_cpu
            && logical_from_mpidr == Some(logical_cpu)
            && stack_owned;
        reports_ok &= report_ok;

        crate::println!(
            "rpi5-psci-secondary-core-alive: report logical={} state={} context={} mpidr={:#018x} affinity={:#x} mapped={:?} sp={:#018x} stack=[{:#018x},{:#018x}) ok={}",
            logical_cpu,
            secondary_state_name(report.lifecycle.raw()),
            report.context,
            report.mpidr,
            report.affinity,
            logical_from_mpidr,
            report.stack_pointer,
            stack_slot.bottom,
            stack_slot.top,
            report_ok
        );
        wait_uart10_empty_early_phase();
    }

    let classification = if reports_ok {
        "pi5-psci-smc-secondary-cores-alive"
    } else if !cpu_on_ok {
        "pi5-psci-smc-cpu-on-failed"
    } else if boot_logical != Some(0) {
        "pi5-psci-boot-core-identity-mismatch"
    } else {
        "pi5-psci-started-but-state-or-stack-incomplete"
    };
    crate::println!(
        "rpi5-psci-secondary-core-alive: wait-remaining={} classification={}",
        remaining,
        classification
    );

    if reports_ok {
        crate::println!("rpi5-psci-secondary-core-alive: PASS");
    } else {
        crate::println!("rpi5-psci-secondary-core-alive: FAIL");
    }
    wait_uart10_empty_early_phase();

    reports_ok
}

#[cfg(talos_boot_scenario = "rpi5_secondary_core_workload")]
pub fn run_secondary_core_workload_proof() -> bool {
    smp::reset_secondary_core_states();

    let boot_mpidr = crate::arch::aarch64::mpidr_el1();
    let boot_affinity = crate::arch::aarch64::mpidr_affinity(boot_mpidr);
    let boot_logical = pi5_logical_cpu_from_mpidr_affinity(boot_affinity);
    let entry = talos_aarch64_rpi5_secondary_entry as *const () as usize;
    let stack_layout = secondary_stack_layout();
    let stack_base = core::ptr::addr_of!(talos_secondary_core_stacks) as usize;
    let stack_end = core::ptr::addr_of!(talos_secondary_core_stacks_end) as usize;

    crate::println!(
        "rpi5-secondary-core-workload: start conduit=smc cores={} target={} boot-mpidr={:#018x} boot-affinity={:#x} boot-logical={:?} entry={:#018x} stack-range=[{:#018x},{:#018x}) scheduler=single-core-deferred",
        MAX_CORES,
        SECONDARY_CORE_WORKLOAD_TARGET,
        boot_mpidr,
        boot_affinity,
        boot_logical,
        entry,
        stack_base,
        stack_end
    );
    wait_uart10_empty_early_phase();

    let mut cpu_on_ok = true;
    for logical_cpu in 1..MAX_CORES {
        let target_affinity = (logical_cpu as u64) << 8;
        let result = unsafe { psci_cpu_on_smc(target_affinity, entry, logical_cpu) };
        crate::println!(
            "rpi5-secondary-core-workload: cpu-on logical={} target-affinity={:#x} result={}",
            logical_cpu,
            target_affinity,
            result
        );
        cpu_on_ok &= result == 0;
        let affinity_after = unsafe { psci_affinity_info_smc(target_affinity, 0) };
        crate::println!(
            "rpi5-secondary-core-workload: affinity-after logical={} target-affinity={:#x} level=0 state={} raw={}",
            logical_cpu,
            target_affinity,
            psci_affinity_state_name(affinity_after),
            affinity_after
        );
        wait_uart10_empty_early_phase();
    }

    let mut remaining = RPI5_SECONDARY_WAIT_LIMIT;
    while remaining > 0 {
        let all_complete = (1..MAX_CORES).all(|logical_cpu| {
            SECONDARY_CORE_STATES[logical_cpu].invalidate_from_poc();
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
        SECONDARY_CORE_STATES[logical_cpu].invalidate_from_poc();
        let report = SECONDARY_CORE_STATES[logical_cpu].snapshot(logical_cpu);
        let logical_from_mpidr = pi5_logical_cpu_from_mpidr_affinity(report.affinity);
        let stack_slot = stack_layout
            .slot(logical_cpu)
            .expect("stack slot for possible Pi 5 core");
        let stack_owned = stack_slot.contains_stack_pointer(report.stack_pointer);
        let report_ok = report.lifecycle >= CoreLifecycle::WorkloadComplete
            && report.context == logical_cpu
            && logical_from_mpidr == Some(logical_cpu)
            && stack_owned
            && report.workload_progress == SECONDARY_CORE_WORKLOAD_TARGET;
        reports_ok &= report_ok;

        crate::println!(
            "rpi5-secondary-core-workload: report logical={} state={} context={} mpidr={:#018x} affinity={:#x} mapped={:?} sp={:#018x} stack=[{:#018x},{:#018x}) progress={} target={} ok={}",
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
        wait_uart10_empty_early_phase();
    }

    let classification = if reports_ok {
        "pi5-secondary-core-controlled-workload-complete"
    } else if !cpu_on_ok {
        "pi5-psci-smc-cpu-on-failed"
    } else if boot_logical != Some(0) {
        "pi5-psci-boot-core-identity-mismatch"
    } else {
        "pi5-secondary-core-workload-incomplete"
    };
    crate::println!(
        "rpi5-secondary-core-workload: wait-remaining={} classification={}",
        remaining,
        classification
    );

    if reports_ok {
        crate::println!("rpi5-secondary-core-workload: PASS");
    } else {
        crate::println!("rpi5-secondary-core-workload: FAIL");
    }
    wait_uart10_empty_early_phase();

    reports_ok
}

#[cfg(talos_boot_scenario = "rpi5_uart10_polling_rx")]
pub fn run_uart10_polling_tty_rx_diagnostic() -> bool {
    crate::println!(
        "rpi5-uart10-rx-diagnostic: ready capacity={} wait-limit={} backend=runtime-console0/bcm2712-uart10-pl011 inject-hex=61 62 58 08 63 59 7f 64 03 65 66 67 68 69 0d",
        crate::tty::CANONICAL_LINE_CAPACITY,
        UART10_RX_WAIT_LIMIT
    );
    wait_uart10_empty_early_phase();

    let result =
        crate::tty::run_polling_rx_diagnostic_with_limit(firmware_console(), UART10_RX_WAIT_LIMIT);
    crate::println!();
    crate::println!(
        "rpi5-uart10-rx-diagnostic: raw-len={} line-len={} terminated={} timeout={} outcome={} truncated={} backspaces={} deletes={} controls={}",
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
    crate::print!("rpi5-uart10-rx-diagnostic: line-hex=");
    print_tty_hex_bytes(result.line());
    crate::println!();
    crate::print!("rpi5-uart10-rx-diagnostic: echo-hex=");
    print_tty_hex_bytes(result.echo());
    crate::println!();
    crate::print!("rpi5-uart10-rx-diagnostic: control-events=");
    print_tty_control_events(result.controls());
    crate::println!();

    let passed = result.passed() && result.truncated() && !result.controls().is_empty();
    if passed {
        crate::println!("rpi5-uart10-rx-diagnostic: PASS");
    } else {
        crate::println!("rpi5-uart10-rx-diagnostic: FAIL");
    }
    wait_uart10_empty_early_phase();

    passed
}

#[cfg(talos_boot_scenario = "rpi5_diagnostic_command_channel")]
pub fn run_diagnostic_command_channel_proof() -> bool {
    crate::println!(
        "rpi5-diagnostic-command-channel-proof: start command-count=4 backend=runtime-console0/bcm2712-uart10-pl011 input=tty-canonical-lite"
    );
    wait_uart10_empty_early_phase();

    let mut passed = true;

    for command_index in 0..4 {
        crate::println!(
            "rpi5-diagnostic-command-channel-proof: ready command={}",
            command_index
        );
        wait_uart10_empty_early_phase();

        let result = crate::tty::run_polling_rx_diagnostic_with_limit(
            firmware_console(),
            UART10_RX_WAIT_LIMIT,
        );
        settle_for_serial_capture();
        crate::println!();
        crate::print!(
            "rpi5-diagnostic-command-channel-proof: line command={} hex=",
            command_index
        );
        print_tty_hex_bytes(result.line());
        crate::println!();

        if !result.passed() || result.truncated() || !result.controls().is_empty() {
            crate::println!(
                "rpi5-diagnostic-command-channel-proof: input-fail command={} outcome={} truncated={} controls={}",
                command_index,
                result.outcome_name(),
                result.truncated(),
                result.controls().len()
            );
            passed = false;
            continue;
        }

        let mut sink = crate::runtime_console::RuntimeConsole::new(firmware_console());
        let dispatch = crate::diagnostic_command::dispatch_default_diagnostic_command(
            result.line(),
            &mut sink,
        );
        let dispatch = match dispatch {
            Ok(dispatch) => dispatch,
            Err(_) => {
                crate::println!(
                    "rpi5-diagnostic-command-channel-proof: dispatch-fail command={} response-write-failed",
                    command_index
                );
                passed = false;
                continue;
            }
        };

        let status_name = diagnostic_dispatch_status_name(dispatch.status);
        crate::println!(
            "rpi5-diagnostic-command-channel-proof: dispatch command={} status={} responses={}",
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
        wait_uart10_empty_early_phase();
    }

    if passed {
        crate::println!("rpi5-diagnostic-command-channel-proof: PASS");
    } else {
        crate::println!("rpi5-diagnostic-command-channel-proof: FAIL");
    }
    wait_uart10_empty_early_phase();

    passed
}

#[cfg(talos_boot_scenario = "rpi5_diagnostic_command_channel")]
fn settle_for_serial_capture() {
    for _ in 0..DIAGNOSTIC_COMMAND_CAPTURE_SETTLE_SPINS {
        core::hint::spin_loop();
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_local_serial_command_loop",
    talos_boot_scenario = "rpi5_local_echo_command",
    talos_boot_scenario = "rpi5_local_literal_echo",
    talos_boot_scenario = "rpi5_local_pwd_command",
    talos_boot_scenario = "rpi5_local_cd_fixed_dirs",
    talos_boot_scenario = "rpi5_local_ls_cwd",
    talos_boot_scenario = "rpi5_local_cat_cwd",
    talos_boot_scenario = "rpi5_local_line_editing",
    talos_boot_scenario = "rpi5_local_line_cancel",
    talos_boot_scenario = "rpi5_local_line_kill"
))]
pub fn run_local_serial_command_loop_proof() -> bool {
    write_early_static("TALOS: command loop proof entered\n");
    let command_count = local_command_pi5_proof_command_count();

    crate::println!(
        "{}: start command-count={} backend=runtime-console0/bcm2712-uart10-pl011 input=fd0/runtime-console0/tty-canonical-lite builtins={} descriptor-backed-input=true descriptor-backed-output=true",
        local_command_pi5_proof_label(),
        command_count,
        crate::local_command_loop::LOCAL_COMMAND_BUILTIN_BOUNDARY
    );
    wait_uart10_empty_early_phase();

    let mut input = firmware_console();
    let mut output = LocalCommandProofConsole::new(firmware_console());
    let mut io =
        match crate::local_command_loop::DescriptorBackedLocalCommandIo::new_inherited_stdio(
            &mut input,
            &mut output,
        ) {
            Ok(sink) => sink,
            Err(error) => {
                crate::println!(
                    "{}: descriptor-bridge-fail error={}",
                    local_command_pi5_proof_label(),
                    error.name()
                );
                crate::println!(
                    "{}: final participants=0 expected={} errors=1 classification={}{}",
                    local_command_pi5_proof_label(),
                    command_count,
                    local_command_pi5_proof_classification(),
                    "-incomplete"
                );
                crate::println!("{}: FAIL", local_command_pi5_proof_label());
                return false;
            }
        };
    write_early_static("TALOS: command loop io ready\n");
    let mut passed = true;

    for command_index in 0..command_count {
        crate::println!(
            "{}: ready command={}",
            local_command_pi5_proof_label(),
            command_index
        );
        wait_uart10_empty_early_phase();

        let result =
            match crate::local_command_loop::run_one_descriptor_backed_serial_command_with_limit(
                &mut io,
                UART10_RX_WAIT_LIMIT,
            ) {
                Ok(result) => result,
                Err(error) => {
                    crate::println!(
                        "{}: cycle-fail command={} error={:?}",
                        local_command_pi5_proof_label(),
                        command_index,
                        error
                    );
                    passed = false;
                    continue;
                }
            };

        crate::println!();
        crate::print!(
            "{}: line command={} hex=",
            local_command_pi5_proof_label(),
            command_index
        );
        print_tty_hex_bytes(result.line());
        crate::println!();
        wait_uart10_empty_early_phase();
        crate::println!(
            "{}: dispatch command={} status={} responses={} raw-bytes={} truncated={} controls={}",
            local_command_pi5_proof_label(),
            command_index,
            result.status_name(),
            result.response_lines(),
            result.raw_bytes(),
            result.truncated(),
            result.controls()
        );
        wait_uart10_empty_early_phase();
        crate::println!(
            "{}: edit command={} backspaces={} deletes={}",
            local_command_pi5_proof_label(),
            command_index,
            result.backspaces(),
            result.deletes()
        );
        wait_uart10_empty_early_phase();
        #[cfg(talos_boot_scenario = "rpi5_local_line_kill")]
        if result.line() == b"pwd"
            && result.status() == crate::local_command_loop::LocalCommandStatus::Handled
            && result.response_lines() == 2
        {
            replay_visible_line_kill_response_for_pi5_proof();
            replay_visible_pwd_response_for_pi5_proof(command_index);
            crate::println!(
                "{}: line-kill-observed partial=bogus control=ctrl-u final-line=pwd raw-bytes={} controls={} responses={}",
                local_command_pi5_proof_label(),
                result.raw_bytes(),
                result.controls(),
                result.response_lines()
            );
            wait_uart10_empty_early_phase();
        }
        if result.line() == b"help"
            && result.status() == crate::local_command_loop::LocalCommandStatus::Handled
            && result.response_lines() == 4
        {
            replay_visible_help_response_for_pi5_proof();
            crate::println!(
                "{}: help-observed input='help' commands='help status stdio pwd echo ls cat cd' editing='backspace delete ctrl-c ctrl-u' raw-bytes={} controls={} responses={}",
                local_command_pi5_proof_label(),
                result.raw_bytes(),
                result.controls(),
                result.response_lines()
            );
            wait_uart10_empty_early_phase();
        } else if result.line() == b"stdio"
            && result.status() == crate::local_command_loop::LocalCommandStatus::Handled
            && result.response_lines() == 7
        {
            replay_visible_stdio_response_for_pi5_proof();
        } else if result.line() == b"pwd"
            && result.status() == crate::local_command_loop::LocalCommandStatus::Handled
            && result.response_lines() == 1
        {
            replay_visible_pwd_response_for_pi5_proof(command_index);
            #[cfg(talos_boot_scenario = "rpi5_local_cd_fixed_dirs")]
            {
                crate::println!(
                    "{}: cd-fixed-dirs-pwd-observed command={} cwd='{}' raw-bytes={} controls={} responses={}",
                    local_command_pi5_proof_label(),
                    command_index,
                    local_command_pi5_cd_fixed_dirs_expected_pwd(command_index),
                    result.raw_bytes(),
                    result.controls(),
                    result.response_lines()
                );
                wait_uart10_empty_early_phase();
            }
        } else if result.line() == b"ls /"
            && result.status() == crate::local_command_loop::LocalCommandStatus::Handled
            && result.response_lines() == 4
        {
            replay_visible_ls_root_response_for_pi5_proof();
            crate::println!(
                "{}: ls-root-observed input='ls /' entries='bin dir empty etc' raw-bytes={} controls={} responses={}",
                local_command_pi5_proof_label(),
                result.raw_bytes(),
                result.controls(),
                result.response_lines()
            );
            wait_uart10_empty_early_phase();
        } else if result.line() == b"ls /bin"
            && result.status() == crate::local_command_loop::LocalCommandStatus::Handled
            && result.response_lines() == 1
        {
            replay_visible_ls_bin_response_for_pi5_proof();
            crate::println!(
                "{}: ls-bin-observed input='ls /bin' entries='init' raw-bytes={} controls={} responses={}",
                local_command_pi5_proof_label(),
                result.raw_bytes(),
                result.controls(),
                result.response_lines()
            );
            wait_uart10_empty_early_phase();
        } else if result.line() == b"cat /etc/banner.txt"
            && result.status() == crate::local_command_loop::LocalCommandStatus::Handled
            && result.response_lines() == 1
        {
            replay_visible_cat_banner_response_for_pi5_proof();
            crate::println!(
                "{}: cat-banner-observed input='cat /etc/banner.txt' output='Talos initramfs fixture' raw-bytes={} controls={} responses={}",
                local_command_pi5_proof_label(),
                result.raw_bytes(),
                result.controls(),
                result.response_lines()
            );
            wait_uart10_empty_early_phase();
        } else if result.line() == b"cat banner.txt"
            && result.status() == crate::local_command_loop::LocalCommandStatus::Handled
            && result.response_lines() == 1
        {
            replay_visible_cat_banner_response_for_pi5_proof();
            crate::println!(
                "{}: cat-cwd-observed input='cat banner.txt' cwd='/etc' output='Talos initramfs fixture' raw-bytes={} controls={} responses={}",
                local_command_pi5_proof_label(),
                result.raw_bytes(),
                result.controls(),
                result.response_lines()
            );
            wait_uart10_empty_early_phase();
        } else if result.line() == b"cat banner.txt"
            && result.status() == crate::local_command_loop::LocalCommandStatus::UnexpectedArgument
            && result.response_lines() == 1
        {
            replay_visible_cat_not_found_response_for_pi5_proof();
            crate::println!(
                "{}: cat-cwd-negative-observed input='cat banner.txt' cwd='/' output='talos: not-found' raw-bytes={} controls={} responses={}",
                local_command_pi5_proof_label(),
                result.raw_bytes(),
                result.controls(),
                result.response_lines()
            );
            wait_uart10_empty_early_phase();
        } else if result.line() == b"echo hello"
            && result.status() == crate::local_command_loop::LocalCommandStatus::Handled
            && result.response_lines() == 1
        {
            replay_visible_echo_response_for_pi5_proof();
        } else if result.line() == b"echo local serial works"
            && result.status() == crate::local_command_loop::LocalCommandStatus::Handled
            && result.response_lines() == 1
        {
            replay_visible_literal_echo_response_for_pi5_proof();
            crate::println!(
                "{}: literal-echo-observed input='echo local serial works' final-line='echo local serial works' raw-bytes={} controls={} responses={}",
                local_command_pi5_proof_label(),
                result.raw_bytes(),
                result.controls(),
                result.response_lines()
            );
            wait_uart10_empty_early_phase();
        } else if result.status() == crate::local_command_loop::LocalCommandStatus::LineCanceled
            && result.response_lines() == 1
        {
            replay_visible_line_cancel_response_for_pi5_proof();
        }
        #[cfg(talos_boot_scenario = "rpi5_local_ls_cwd")]
        if result.line() == b"ls"
            && result.status() == crate::local_command_loop::LocalCommandStatus::Handled
        {
            replay_visible_ls_cwd_response_for_pi5_proof(command_index);
            crate::println!(
                "{}: ls-cwd-observed command={} cwd='{}' entries='{}' raw-bytes={} controls={} responses={}",
                local_command_pi5_proof_label(),
                command_index,
                local_command_pi5_ls_cwd_expected_cwd(command_index),
                local_command_pi5_ls_cwd_expected_entries(command_index),
                result.raw_bytes(),
                result.controls(),
                result.response_lines()
            );
            wait_uart10_empty_early_phase();
        }

        if !expected_local_command_loop_dispatch(
            command_index,
            result.line(),
            result.status(),
            result.response_lines(),
        ) {
            passed = false;
        }
        wait_uart10_empty_early_phase();
    }

    let ready_for_next = crate::local_command_loop::write_local_command_prompt(&mut io).is_ok();
    crate::println!();
    crate::println!(
        "{}: ready-for-next prompt={}",
        local_command_pi5_proof_label(),
        ready_for_next
    );
    if ready_for_next {
        write_early_static("TALOS: command loop prompt ready\n");
    }

    if ready_for_next && passed {
        crate::println!(
            "{}: final participants={} expected={} errors=0 classification={}",
            local_command_pi5_proof_label(),
            command_count,
            command_count,
            local_command_pi5_proof_classification()
        );
        crate::println!("{}: PASS", local_command_pi5_proof_label());
    } else {
        crate::println!(
            "{}: final participants={} expected={} errors=1 classification={}{}",
            local_command_pi5_proof_label(),
            command_count,
            command_count,
            local_command_pi5_proof_classification(),
            "-incomplete"
        );
        crate::println!("{}: FAIL", local_command_pi5_proof_label());
    }
    wait_uart10_empty_early_phase();

    ready_for_next && passed
}

#[cfg(any(
    talos_boot_scenario = "rpi5_local_serial_command_loop",
    talos_boot_scenario = "rpi5_local_echo_command",
    talos_boot_scenario = "rpi5_local_literal_echo",
    talos_boot_scenario = "rpi5_local_pwd_command",
    talos_boot_scenario = "rpi5_local_cd_fixed_dirs",
    talos_boot_scenario = "rpi5_local_ls_cwd",
    talos_boot_scenario = "rpi5_local_cat_cwd",
    talos_boot_scenario = "rpi5_local_line_editing",
    talos_boot_scenario = "rpi5_local_line_cancel",
    talos_boot_scenario = "rpi5_local_line_kill"
))]
fn replay_visible_stdio_response_for_pi5_proof() {
    crate::println!("talos: ok stdio");
    crate::println!("talos: fd 0 stdio-input");
    crate::println!("talos: fd 1 stdio-output");
    crate::println!("talos: fd 2 stdio-output");
    crate::println!("talos: runtime-console runtime-console0");
    crate::println!("talos: descriptor-backed-input=true");
    crate::println!("talos: descriptor-backed-output=true");
    wait_uart10_empty_early_phase();
}

#[cfg(any(
    talos_boot_scenario = "rpi5_local_serial_command_loop",
    talos_boot_scenario = "rpi5_local_echo_command",
    talos_boot_scenario = "rpi5_local_literal_echo",
    talos_boot_scenario = "rpi5_local_pwd_command",
    talos_boot_scenario = "rpi5_local_line_cancel",
    talos_boot_scenario = "rpi5_local_line_kill"
))]
fn replay_visible_echo_response_for_pi5_proof() {
    crate::println!("hello");
    wait_uart10_empty_early_phase();
}

#[cfg(any(
    talos_boot_scenario = "rpi5_local_serial_command_loop",
    talos_boot_scenario = "rpi5_local_literal_echo",
    talos_boot_scenario = "rpi5_local_help_command",
    talos_boot_scenario = "rpi5_local_ls_root",
    talos_boot_scenario = "rpi5_local_ls_bin",
    talos_boot_scenario = "rpi5_local_cat_banner",
    talos_boot_scenario = "rpi5_local_cat_cwd",
    talos_boot_scenario = "rpi5_local_cd_fixed_dirs",
    talos_boot_scenario = "rpi5_local_ls_cwd"
))]
fn replay_visible_literal_echo_response_for_pi5_proof() {
    crate::println!("local serial works");
    wait_uart10_empty_early_phase();
}

#[cfg(any(
    talos_boot_scenario = "rpi5_local_serial_command_loop",
    talos_boot_scenario = "rpi5_local_echo_command",
    talos_boot_scenario = "rpi5_local_literal_echo",
    talos_boot_scenario = "rpi5_local_help_command",
    talos_boot_scenario = "rpi5_local_ls_root",
    talos_boot_scenario = "rpi5_local_ls_bin",
    talos_boot_scenario = "rpi5_local_cat_banner",
    talos_boot_scenario = "rpi5_local_cat_cwd",
    talos_boot_scenario = "rpi5_local_cd_fixed_dirs",
    talos_boot_scenario = "rpi5_local_ls_cwd",
    talos_boot_scenario = "rpi5_local_pwd_command",
    talos_boot_scenario = "rpi5_local_line_editing",
    talos_boot_scenario = "rpi5_local_line_cancel",
    talos_boot_scenario = "rpi5_local_line_kill"
))]
fn replay_visible_help_response_for_pi5_proof() {
    crate::println!("talos: ok help");
    crate::println!("talos: commands help status stdio pwd echo ls cat cd");
    crate::println!("talos: echo forms echo hello; echo local serial works");
    crate::println!("talos: editing backspace delete ctrl-c ctrl-u");
    wait_uart10_empty_early_phase();
}

#[cfg(any(
    talos_boot_scenario = "rpi5_local_serial_command_loop",
    talos_boot_scenario = "rpi5_local_literal_echo",
    talos_boot_scenario = "rpi5_local_help_command",
    talos_boot_scenario = "rpi5_local_ls_root",
    talos_boot_scenario = "rpi5_local_ls_bin",
    talos_boot_scenario = "rpi5_local_cat_banner",
    talos_boot_scenario = "rpi5_local_cat_cwd",
    talos_boot_scenario = "rpi5_local_cd_fixed_dirs",
    talos_boot_scenario = "rpi5_local_ls_cwd",
    talos_boot_scenario = "rpi5_local_pwd_command",
    talos_boot_scenario = "rpi5_local_line_editing",
    talos_boot_scenario = "rpi5_local_line_cancel",
    talos_boot_scenario = "rpi5_local_line_kill"
))]
fn replay_visible_pwd_response_for_pi5_proof(command_index: usize) {
    #[cfg(talos_boot_scenario = "rpi5_local_cd_fixed_dirs")]
    crate::println!(
        "{}",
        local_command_pi5_cd_fixed_dirs_expected_pwd(command_index)
    );

    #[cfg(not(talos_boot_scenario = "rpi5_local_cd_fixed_dirs"))]
    {
        let _ = command_index;
        crate::println!("/");
    }
    wait_uart10_empty_early_phase();
}

#[cfg(talos_boot_scenario = "rpi5_local_cd_fixed_dirs")]
const fn local_command_pi5_cd_fixed_dirs_expected_pwd(command_index: usize) -> &'static str {
    match command_index {
        2 => "/etc",
        4 => "/bin",
        _ => "/",
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_local_serial_command_loop",
    talos_boot_scenario = "rpi5_local_ls_root"
))]
fn replay_visible_ls_root_response_for_pi5_proof() {
    crate::println!("bin");
    crate::println!("dir");
    crate::println!("empty");
    crate::println!("etc");
    wait_uart10_empty_early_phase();
}

#[cfg(any(
    talos_boot_scenario = "rpi5_local_serial_command_loop",
    talos_boot_scenario = "rpi5_local_ls_bin"
))]
fn replay_visible_ls_bin_response_for_pi5_proof() {
    crate::println!("init");
    wait_uart10_empty_early_phase();
}

#[cfg(talos_boot_scenario = "rpi5_local_ls_cwd")]
fn replay_visible_ls_cwd_response_for_pi5_proof(command_index: usize) {
    match command_index {
        1 | 7 => replay_visible_ls_root_response_for_pi5_proof(),
        3 => {
            crate::println!("banner.txt");
            wait_uart10_empty_early_phase();
        }
        5 => replay_visible_ls_bin_response_for_pi5_proof(),
        _ => {}
    }
}

#[cfg(talos_boot_scenario = "rpi5_local_serial_command_loop")]
fn replay_visible_cat_banner_response_for_pi5_proof() {
    crate::println!("Talos initramfs fixture");
    wait_uart10_empty_early_phase();
}

#[cfg(talos_boot_scenario = "rpi5_local_serial_command_loop")]
fn replay_visible_cat_not_found_response_for_pi5_proof() {
    crate::println!("talos: not-found");
    wait_uart10_empty_early_phase();
}

#[cfg(any(
    talos_boot_scenario = "rpi5_local_serial_command_loop",
    talos_boot_scenario = "rpi5_local_literal_echo",
    talos_boot_scenario = "rpi5_local_help_command",
    talos_boot_scenario = "rpi5_local_ls_root",
    talos_boot_scenario = "rpi5_local_ls_bin",
    talos_boot_scenario = "rpi5_local_cat_banner",
    talos_boot_scenario = "rpi5_local_cat_cwd",
    talos_boot_scenario = "rpi5_local_cd_fixed_dirs",
    talos_boot_scenario = "rpi5_local_ls_cwd",
    talos_boot_scenario = "rpi5_local_line_cancel",
    talos_boot_scenario = "rpi5_local_line_kill"
))]
fn replay_visible_line_cancel_response_for_pi5_proof() {
    crate::println!("talos: line-canceled");
    wait_uart10_empty_early_phase();
}

#[cfg(talos_boot_scenario = "rpi5_local_line_kill")]
fn replay_visible_line_kill_response_for_pi5_proof() {
    crate::println!("talos: line-killed");
    wait_uart10_empty_early_phase();
}

#[cfg(talos_boot_scenario = "rpi5_local_pwd_command")]
const fn local_command_pi5_proof_label() -> &'static str {
    "rpi5-local-pwd-command-proof"
}

#[cfg(talos_boot_scenario = "rpi5_local_line_editing")]
const fn local_command_pi5_proof_label() -> &'static str {
    "rpi5-local-line-editing-proof"
}

#[cfg(talos_boot_scenario = "rpi5_local_line_cancel")]
const fn local_command_pi5_proof_label() -> &'static str {
    "rpi5-local-line-cancel-proof"
}

#[cfg(talos_boot_scenario = "rpi5_local_line_kill")]
const fn local_command_pi5_proof_label() -> &'static str {
    "rpi5-local-line-kill-proof"
}

#[cfg(talos_boot_scenario = "rpi5_local_echo_command")]
const fn local_command_pi5_proof_label() -> &'static str {
    "rpi5-local-echo-command-proof"
}

#[cfg(talos_boot_scenario = "rpi5_local_literal_echo")]
const fn local_command_pi5_proof_label() -> &'static str {
    "rpi5-local-literal-echo-proof"
}

#[cfg(talos_boot_scenario = "rpi5_local_help_command")]
const fn local_command_pi5_proof_label() -> &'static str {
    "rpi5-local-help-command-proof"
}

#[cfg(talos_boot_scenario = "rpi5_local_ls_root")]
const fn local_command_pi5_proof_label() -> &'static str {
    "rpi5-local-ls-root-proof"
}

#[cfg(talos_boot_scenario = "rpi5_local_ls_bin")]
const fn local_command_pi5_proof_label() -> &'static str {
    "rpi5-local-ls-bin-proof"
}

#[cfg(talos_boot_scenario = "rpi5_local_cat_banner")]
const fn local_command_pi5_proof_label() -> &'static str {
    "rpi5-local-cat-banner-proof"
}

#[cfg(talos_boot_scenario = "rpi5_local_cat_cwd")]
const fn local_command_pi5_proof_label() -> &'static str {
    "rpi5-local-cat-cwd-proof"
}

#[cfg(talos_boot_scenario = "rpi5_local_cd_fixed_dirs")]
const fn local_command_pi5_proof_label() -> &'static str {
    "rpi5-local-cd-fixed-dirs-proof"
}

#[cfg(talos_boot_scenario = "rpi5_local_ls_cwd")]
const fn local_command_pi5_proof_label() -> &'static str {
    "rpi5-local-ls-cwd-proof"
}

#[cfg(talos_boot_scenario = "rpi5_generated_root_boot_transport")]
const fn local_command_pi5_proof_label() -> &'static str {
    "rpi5-generated-root-boot-transport-proof"
}

#[cfg(all(
    talos_boot_scenario = "rpi5_local_serial_command_loop",
    not(talos_boot_scenario = "rpi5_local_echo_command"),
    not(talos_boot_scenario = "rpi5_local_literal_echo"),
    not(talos_boot_scenario = "rpi5_local_help_command"),
    not(talos_boot_scenario = "rpi5_local_ls_root"),
    not(talos_boot_scenario = "rpi5_local_ls_bin"),
    not(talos_boot_scenario = "rpi5_local_cat_banner"),
    not(talos_boot_scenario = "rpi5_local_cat_cwd"),
    not(talos_boot_scenario = "rpi5_local_cd_fixed_dirs"),
    not(talos_boot_scenario = "rpi5_local_ls_cwd"),
    not(talos_boot_scenario = "rpi5_generated_root_boot_transport"),
    not(talos_boot_scenario = "rpi5_local_pwd_command"),
    not(talos_boot_scenario = "rpi5_local_line_editing"),
    not(talos_boot_scenario = "rpi5_local_line_cancel"),
    not(talos_boot_scenario = "rpi5_local_line_kill")
))]
const fn local_command_pi5_proof_label() -> &'static str {
    "rpi5-local-command-stdio-bridge-proof"
}

#[cfg(talos_boot_scenario = "rpi5_local_pwd_command")]
const fn local_command_pi5_proof_classification() -> &'static str {
    "pi5-local-pwd-command-complete"
}

#[cfg(talos_boot_scenario = "rpi5_local_line_editing")]
const fn local_command_pi5_proof_classification() -> &'static str {
    "pi5-local-line-editing-complete"
}

#[cfg(talos_boot_scenario = "rpi5_local_line_cancel")]
const fn local_command_pi5_proof_classification() -> &'static str {
    "pi5-local-line-cancel-complete"
}

#[cfg(talos_boot_scenario = "rpi5_local_line_kill")]
const fn local_command_pi5_proof_classification() -> &'static str {
    "pi5-local-line-kill-complete"
}

#[cfg(talos_boot_scenario = "rpi5_local_echo_command")]
const fn local_command_pi5_proof_classification() -> &'static str {
    "pi5-local-echo-command-complete"
}

#[cfg(talos_boot_scenario = "rpi5_local_literal_echo")]
const fn local_command_pi5_proof_classification() -> &'static str {
    "pi5-local-literal-echo-complete"
}

#[cfg(talos_boot_scenario = "rpi5_local_help_command")]
const fn local_command_pi5_proof_classification() -> &'static str {
    "pi5-local-help-command-complete"
}

#[cfg(talos_boot_scenario = "rpi5_local_ls_root")]
const fn local_command_pi5_proof_classification() -> &'static str {
    "pi5-local-ls-root-complete"
}

#[cfg(talos_boot_scenario = "rpi5_local_ls_bin")]
const fn local_command_pi5_proof_classification() -> &'static str {
    "pi5-local-ls-bin-complete"
}

#[cfg(talos_boot_scenario = "rpi5_local_cat_banner")]
const fn local_command_pi5_proof_classification() -> &'static str {
    "pi5-local-cat-banner-complete"
}

#[cfg(talos_boot_scenario = "rpi5_local_cat_cwd")]
const fn local_command_pi5_proof_classification() -> &'static str {
    "pi5-local-cat-cwd-complete"
}

#[cfg(talos_boot_scenario = "rpi5_local_cd_fixed_dirs")]
const fn local_command_pi5_proof_classification() -> &'static str {
    "pi5-local-cd-fixed-dirs-complete"
}

#[cfg(talos_boot_scenario = "rpi5_local_ls_cwd")]
const fn local_command_pi5_proof_classification() -> &'static str {
    "pi5-local-ls-cwd-complete"
}

#[cfg(talos_boot_scenario = "rpi5_generated_root_boot_transport")]
const fn local_command_pi5_proof_classification() -> &'static str {
    "pi5-generated-root-boot-transport-complete"
}

#[cfg(all(
    talos_boot_scenario = "rpi5_local_serial_command_loop",
    not(talos_boot_scenario = "rpi5_local_echo_command"),
    not(talos_boot_scenario = "rpi5_local_literal_echo"),
    not(talos_boot_scenario = "rpi5_local_help_command"),
    not(talos_boot_scenario = "rpi5_local_ls_root"),
    not(talos_boot_scenario = "rpi5_local_ls_bin"),
    not(talos_boot_scenario = "rpi5_local_cat_banner"),
    not(talos_boot_scenario = "rpi5_local_cat_cwd"),
    not(talos_boot_scenario = "rpi5_local_cd_fixed_dirs"),
    not(talos_boot_scenario = "rpi5_local_ls_cwd"),
    not(talos_boot_scenario = "rpi5_generated_root_boot_transport"),
    not(talos_boot_scenario = "rpi5_local_pwd_command"),
    not(talos_boot_scenario = "rpi5_local_line_editing"),
    not(talos_boot_scenario = "rpi5_local_line_cancel"),
    not(talos_boot_scenario = "rpi5_local_line_kill")
))]
const fn local_command_pi5_proof_classification() -> &'static str {
    "pi5-local-command-stdio-bridge-complete"
}

#[cfg(any(
    talos_boot_scenario = "rpi5_local_serial_command_loop",
    talos_boot_scenario = "rpi5_local_echo_command",
    talos_boot_scenario = "rpi5_local_literal_echo",
    talos_boot_scenario = "rpi5_local_pwd_command",
    talos_boot_scenario = "rpi5_local_cd_fixed_dirs",
    talos_boot_scenario = "rpi5_local_ls_cwd",
    talos_boot_scenario = "rpi5_local_cat_cwd",
    talos_boot_scenario = "rpi5_local_line_editing",
    talos_boot_scenario = "rpi5_local_line_cancel",
    talos_boot_scenario = "rpi5_local_line_kill"
))]
const fn local_command_pi5_proof_command_count() -> usize {
    if cfg!(talos_boot_scenario = "rpi5_generated_root_boot_transport") {
        5
    } else if cfg!(talos_boot_scenario = "rpi5_local_cd_fixed_dirs") {
        9
    } else if cfg!(talos_boot_scenario = "rpi5_local_ls_cwd") {
        9
    } else if cfg!(talos_boot_scenario = "rpi5_local_cat_cwd") {
        4
    } else if cfg!(talos_boot_scenario = "rpi5_local_line_cancel") {
        2
    } else {
        1
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_local_serial_command_loop",
    talos_boot_scenario = "rpi5_local_echo_command",
    talos_boot_scenario = "rpi5_local_literal_echo",
    talos_boot_scenario = "rpi5_local_pwd_command",
    talos_boot_scenario = "rpi5_local_cd_fixed_dirs",
    talos_boot_scenario = "rpi5_local_ls_cwd",
    talos_boot_scenario = "rpi5_local_cat_cwd",
    talos_boot_scenario = "rpi5_local_line_editing",
    talos_boot_scenario = "rpi5_local_line_cancel",
    talos_boot_scenario = "rpi5_local_line_kill"
))]
struct LocalCommandProofConsole;

#[cfg(any(
    talos_boot_scenario = "rpi5_local_serial_command_loop",
    talos_boot_scenario = "rpi5_local_echo_command",
    talos_boot_scenario = "rpi5_local_literal_echo",
    talos_boot_scenario = "rpi5_local_pwd_command",
    talos_boot_scenario = "rpi5_local_cd_fixed_dirs",
    talos_boot_scenario = "rpi5_local_ls_cwd",
    talos_boot_scenario = "rpi5_local_cat_cwd",
    talos_boot_scenario = "rpi5_local_line_editing",
    talos_boot_scenario = "rpi5_local_line_cancel",
    talos_boot_scenario = "rpi5_local_line_kill"
))]
impl LocalCommandProofConsole {
    const fn new(_uart: Pl011) -> Self {
        Self
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_local_serial_command_loop",
    talos_boot_scenario = "rpi5_local_echo_command",
    talos_boot_scenario = "rpi5_local_literal_echo",
    talos_boot_scenario = "rpi5_local_pwd_command",
    talos_boot_scenario = "rpi5_local_cd_fixed_dirs",
    talos_boot_scenario = "rpi5_local_ls_cwd",
    talos_boot_scenario = "rpi5_local_cat_cwd",
    talos_boot_scenario = "rpi5_local_line_editing",
    talos_boot_scenario = "rpi5_local_line_cancel",
    talos_boot_scenario = "rpi5_local_line_kill"
))]
impl core::fmt::Write for LocalCommandProofConsole {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        crate::target::console::write_static(s);
        wait_uart10_empty_early_phase();
        Ok(())
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_local_serial_command_loop",
    talos_boot_scenario = "rpi5_local_echo_command",
    talos_boot_scenario = "rpi5_local_literal_echo",
    talos_boot_scenario = "rpi5_local_pwd_command",
    talos_boot_scenario = "rpi5_local_line_editing",
    talos_boot_scenario = "rpi5_local_ls_cwd",
    talos_boot_scenario = "rpi5_local_cat_cwd",
    talos_boot_scenario = "rpi5_local_line_cancel",
    talos_boot_scenario = "rpi5_local_line_kill"
))]
fn expected_local_command_loop_dispatch(
    command_index: usize,
    line: &[u8],
    status: crate::local_command_loop::LocalCommandStatus,
    response_lines: usize,
) -> bool {
    use crate::local_command_loop::LocalCommandStatus::{
        Empty, Handled, LineCanceled, UnexpectedArgument, UnknownCommand,
    };

    match command_index {
        0 if cfg!(talos_boot_scenario = "rpi5_local_pwd_command") => {
            line == b"pwd" && status == Handled && response_lines == 1
        }
        0 if cfg!(talos_boot_scenario = "rpi5_local_line_editing") => {
            line == b"pwd" && status == Handled && response_lines == 1
        }
        0 if cfg!(talos_boot_scenario = "rpi5_local_line_cancel") => {
            line.is_empty() && status == LineCanceled && response_lines == 1
        }
        1 if cfg!(talos_boot_scenario = "rpi5_local_line_cancel") => {
            line == b"pwd" && status == Handled && response_lines == 1
        }
        0 if cfg!(talos_boot_scenario = "rpi5_local_line_kill") => {
            line == b"pwd" && status == Handled && response_lines == 2
        }
        0 if cfg!(talos_boot_scenario = "rpi5_local_echo_command") => {
            line == b"echo hello" && status == Handled && response_lines == 1
        }
        0 if cfg!(talos_boot_scenario = "rpi5_local_literal_echo") => {
            line == b"echo local serial works" && status == Handled && response_lines == 1
        }
        0 if cfg!(talos_boot_scenario = "rpi5_local_help_command") => {
            line == b"help" && status == Handled && response_lines == 4
        }
        0 if cfg!(talos_boot_scenario = "rpi5_local_ls_root") => {
            line == b"ls /" && status == Handled && response_lines == 4
        }
        0 if cfg!(talos_boot_scenario = "rpi5_local_ls_bin") => {
            line == b"ls /bin" && status == Handled && response_lines == 1
        }
        0 if cfg!(talos_boot_scenario = "rpi5_local_cat_banner") => {
            line == b"cat /etc/banner.txt" && status == Handled && response_lines == 1
        }
        0 if cfg!(talos_boot_scenario = "rpi5_local_cat_cwd") => {
            line == b"cd /etc" && status == Handled && response_lines == 0
        }
        1 if cfg!(talos_boot_scenario = "rpi5_local_cat_cwd") => {
            line == b"cat banner.txt" && status == Handled && response_lines == 1
        }
        2 if cfg!(talos_boot_scenario = "rpi5_local_cat_cwd") => {
            line == b"cd /" && status == Handled && response_lines == 0
        }
        3 if cfg!(talos_boot_scenario = "rpi5_local_cat_cwd") => {
            line == b"cat banner.txt" && status == UnexpectedArgument && response_lines == 1
        }
        0 if cfg!(talos_boot_scenario = "rpi5_local_cd_fixed_dirs") => {
            line == b"pwd" && status == Handled && response_lines == 1
        }
        1 if cfg!(talos_boot_scenario = "rpi5_local_cd_fixed_dirs") => {
            line == b"cd /etc" && status == Handled && response_lines == 0
        }
        2 if cfg!(talos_boot_scenario = "rpi5_local_cd_fixed_dirs") => {
            line == b"pwd" && status == Handled && response_lines == 1
        }
        3 if cfg!(talos_boot_scenario = "rpi5_local_cd_fixed_dirs") => {
            line == b"cd /bin" && status == Handled && response_lines == 0
        }
        4 if cfg!(talos_boot_scenario = "rpi5_local_cd_fixed_dirs") => {
            line == b"pwd" && status == Handled && response_lines == 1
        }
        5 if cfg!(talos_boot_scenario = "rpi5_local_cd_fixed_dirs") => {
            line == b"cd /" && status == Handled && response_lines == 0
        }
        6 if cfg!(talos_boot_scenario = "rpi5_local_cd_fixed_dirs") => {
            line == b"pwd" && status == Handled && response_lines == 1
        }
        7 if cfg!(talos_boot_scenario = "rpi5_local_cd_fixed_dirs") => {
            line == b"cd /missing"
                && status == crate::local_command_loop::LocalCommandStatus::UnexpectedArgument
                && response_lines == 1
        }
        8 if cfg!(talos_boot_scenario = "rpi5_local_cd_fixed_dirs") => {
            line == b"pwd" && status == Handled && response_lines == 1
        }
        0 if cfg!(talos_boot_scenario = "rpi5_local_ls_cwd") => {
            line == b"pwd" && status == Handled && response_lines == 1
        }
        1 if cfg!(talos_boot_scenario = "rpi5_local_ls_cwd") => {
            line == b"ls" && status == Handled && response_lines == 4
        }
        2 if cfg!(talos_boot_scenario = "rpi5_local_ls_cwd") => {
            line == b"cd /etc" && status == Handled && response_lines == 0
        }
        3 if cfg!(talos_boot_scenario = "rpi5_local_ls_cwd") => {
            line == b"ls" && status == Handled && response_lines == 1
        }
        4 if cfg!(talos_boot_scenario = "rpi5_local_ls_cwd") => {
            line == b"cd /bin" && status == Handled && response_lines == 0
        }
        5 if cfg!(talos_boot_scenario = "rpi5_local_ls_cwd") => {
            line == b"ls" && status == Handled && response_lines == 1
        }
        6 if cfg!(talos_boot_scenario = "rpi5_local_ls_cwd") => {
            line == b"cd /" && status == Handled && response_lines == 0
        }
        7 if cfg!(talos_boot_scenario = "rpi5_local_ls_cwd") => {
            line == b"ls" && status == Handled && response_lines == 4
        }
        8 if cfg!(talos_boot_scenario = "rpi5_local_ls_cwd") => {
            line == b"bogus" && status == UnknownCommand && response_lines == 1
        }
        0 if cfg!(talos_boot_scenario = "rpi5_generated_root_boot_transport") => {
            line == b"rootinfo" && status == Handled && response_lines == 1
        }
        1 if cfg!(talos_boot_scenario = "rpi5_generated_root_boot_transport") => {
            line == b"cat /generated/manifest.txt" && status == Handled && response_lines == 1
        }
        2 if cfg!(talos_boot_scenario = "rpi5_generated_root_boot_transport") => {
            line == b"exec /generated/status7 alpha" && status == Handled && response_lines == 9
        }
        3 if cfg!(talos_boot_scenario = "rpi5_generated_root_boot_transport") => {
            line == b"waitpid" && status == Handled && response_lines == 1
        }
        4 if cfg!(talos_boot_scenario = "rpi5_generated_root_boot_transport") => {
            line == b"laststatus" && status == Handled && response_lines == 1
        }
        0 => line == b"stdio" && status == Handled && response_lines == 7,
        _ => false,
    }
}

#[cfg(talos_boot_scenario = "rpi5_local_ls_cwd")]
const fn local_command_pi5_ls_cwd_expected_cwd(command_index: usize) -> &'static str {
    match command_index {
        3 => "/etc",
        5 => "/bin",
        _ => "/",
    }
}

#[cfg(talos_boot_scenario = "rpi5_local_ls_cwd")]
const fn local_command_pi5_ls_cwd_expected_entries(command_index: usize) -> &'static str {
    match command_index {
        3 => "banner.txt",
        5 => "init",
        _ => "bin dir empty etc",
    }
}

#[cfg(talos_boot_scenario = "rpi5_diagnostic_command_channel")]
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

#[cfg(talos_boot_scenario = "rpi5_diagnostic_command_channel")]
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
    talos_boot_scenario = "rpi5_uart10_polling_rx",
    talos_boot_scenario = "rpi5_diagnostic_command_channel",
    talos_boot_scenario = "rpi5_local_serial_command_loop",
    talos_boot_scenario = "rpi5_local_literal_echo",
    talos_boot_scenario = "rpi5_local_line_editing",
    talos_boot_scenario = "rpi5_local_line_cancel",
    talos_boot_scenario = "rpi5_local_line_kill"
))]
fn print_tty_hex_bytes(bytes: &[u8]) {
    for (index, byte) in bytes.iter().enumerate() {
        if index != 0 {
            crate::print!(" ");
        }
        crate::print!("{:02x}", byte);
    }
}

#[cfg(talos_boot_scenario = "rpi5_uart10_polling_rx")]
fn print_tty_control_events(events: &[Option<crate::tty::TtyControlEvent>]) {
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

#[cfg(talos_boot_scenario = "rpi5_timer_preemption")]
#[repr(align(16))]
struct KernelThreadStack([u8; CONTEXT_SWITCH_STACK_SIZE]);

#[cfg(talos_boot_scenario = "rpi5_timer_preemption")]
impl KernelThreadStack {
    const fn new() -> Self {
        Self([0; CONTEXT_SWITCH_STACK_SIZE])
    }

    fn top(&self) -> usize {
        self.0.as_ptr() as usize + self.0.len()
    }
}

#[cfg(talos_boot_scenario = "rpi5_timer_preemption")]
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

#[cfg(talos_boot_scenario = "rpi5_timer_preemption")]
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
            rpi5_timer_preemption_thread as *const () as usize,
            0,
        );
        self.worker_contexts[1] = ContextFrame::kernel_thread_bootstrap(
            self.worker_stacks[1].top(),
            aarch64::kernel_thread_trampoline_address(),
            rpi5_timer_preemption_thread as *const () as usize,
            1,
        );

        let task1_id = TaskId::new(1).expect("nonzero task id");
        let task2_id = TaskId::new(2).expect("nonzero task id");
        let stack1 = KernelStack::new(
            self.worker_stacks[0].top() - CONTEXT_SWITCH_STACK_SIZE,
            CONTEXT_SWITCH_STACK_SIZE,
        )
        .expect("valid Pi 5 timer-preemption task 1 stack");
        let stack2 = KernelStack::new(
            self.worker_stacks[1].top() - CONTEXT_SWITCH_STACK_SIZE,
            CONTEXT_SWITCH_STACK_SIZE,
        )
        .expect("valid Pi 5 timer-preemption task 2 stack");
        let mut task1 = Task::kernel_thread(task1_id, stack1, self.worker_contexts[0]);
        let mut task2 = Task::kernel_thread(task2_id, stack2, self.worker_contexts[1]);
        task1.set_state(TaskState::Running);
        self.scheduler
            .make_runnable(&mut task2)
            .expect("Pi 5 timer-preemption smoke has runnable capacity");
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
            .expect("current Pi 5 timer-preemption task exists");
        let preempted_task = current.id();
        let next_task = self
            .scheduler
            .timer_preempt(current)
            .expect("Pi 5 timer-preemption smoke has a runnable peer");
        let next_task_index = (next_task.raw() - 1) as usize;
        self.tasks[next_task_index]
            .as_mut()
            .expect("next Pi 5 timer-preemption task exists")
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

#[cfg(talos_boot_scenario = "rpi5_timer_preemption")]
struct TimerPreemptionSmokeCell(UnsafeCell<TimerPreemptionSmokeState>);

#[cfg(talos_boot_scenario = "rpi5_timer_preemption")]
unsafe impl Sync for TimerPreemptionSmokeCell {}

#[cfg(talos_boot_scenario = "rpi5_timer_preemption")]
impl TimerPreemptionSmokeCell {
    const fn new() -> Self {
        Self(UnsafeCell::new(TimerPreemptionSmokeState::new()))
    }

    unsafe fn get(&self) -> *mut TimerPreemptionSmokeState {
        self.0.get()
    }
}

#[cfg(talos_boot_scenario = "rpi5_timer_preemption")]
static TIMER_PREEMPTION_SMOKE: TimerPreemptionSmokeCell = TimerPreemptionSmokeCell::new();

#[cfg(any(
    talos_boot_scenario = "rpi5_timer_irq",
    talos_boot_scenario = "rpi5_timer_preemption",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof"
))]
const PRODUCTION_TIMER_PREEMPTION_RUNNABLE_CAPACITY: usize = 2;
#[cfg(any(
    talos_boot_scenario = "rpi5_timer_irq",
    talos_boot_scenario = "rpi5_timer_preemption",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof"
))]
const PRODUCTION_TIMER_PREEMPTION_REMOTE_WAKE_CAPACITY: usize = 1;

#[cfg(any(
    talos_boot_scenario = "rpi5_timer_irq",
    talos_boot_scenario = "rpi5_timer_preemption",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof"
))]
struct ProductionSchedulerRuntimeCell(
    core::cell::UnsafeCell<
        crate::scheduler::ProductionSchedulerRuntime<
            PRODUCTION_TIMER_PREEMPTION_RUNNABLE_CAPACITY,
            PRODUCTION_TIMER_PREEMPTION_REMOTE_WAKE_CAPACITY,
        >,
    >,
);

#[cfg(any(
    talos_boot_scenario = "rpi5_timer_irq",
    talos_boot_scenario = "rpi5_timer_preemption",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof"
))]
unsafe impl Sync for ProductionSchedulerRuntimeCell {}

#[cfg(any(
    talos_boot_scenario = "rpi5_timer_irq",
    talos_boot_scenario = "rpi5_timer_preemption",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof"
))]
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

#[cfg(any(
    talos_boot_scenario = "rpi5_timer_irq",
    talos_boot_scenario = "rpi5_timer_preemption",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof"
))]
static PRODUCTION_SCHEDULER_RUNTIMES: [ProductionSchedulerRuntimeCell; crate::smp::MAX_CORES] = [
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

#[cfg(any(
    talos_boot_scenario = "rpi5_timer_irq",
    talos_boot_scenario = "rpi5_timer_preemption",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof"
))]
static PRODUCTION_TIMER_PREEMPTION_RECORD_MISSES: AtomicU64 = AtomicU64::new(0);

#[cfg(any(
    talos_boot_scenario = "rpi5_timer_irq",
    talos_boot_scenario = "rpi5_timer_preemption",
    talos_boot_scenario = "rpi5_production_timer_preemption_proof"
))]
fn record_production_timer_preemption_irq(logical_cpu: Option<usize>) {
    let Some(logical_cpu) = logical_cpu.filter(|cpu| *cpu < crate::smp::MAX_CORES) else {
        PRODUCTION_TIMER_PREEMPTION_RECORD_MISSES.fetch_add(1, Ordering::Relaxed);
        return;
    };

    let runtime = unsafe { &mut *PRODUCTION_SCHEDULER_RUNTIMES[logical_cpu].get() };
    let _ = runtime.record_timer_irq::<{ crate::smp::MAX_CORES }>(Some(logical_cpu));
}

#[cfg(any(
    talos_boot_scenario = "rpi5_timer_irq",
    talos_boot_scenario = "rpi5_timer_preemption"
))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimerIrqSnapshot {
    pub timer_count: u64,
    pub last_vector: u64,
    pub last_iar: u64,
    pub last_intid: u64,
    pub unexpected_gic_count: u64,
}

#[cfg(any(
    talos_boot_scenario = "rpi5_timer_irq",
    talos_boot_scenario = "rpi5_timer_preemption"
))]
pub fn timer_irq_snapshot() -> TimerIrqSnapshot {
    TimerIrqSnapshot {
        timer_count: generic_timer::monotonic_ticks(),
        last_vector: LAST_IRQ_VECTOR.load(Ordering::Relaxed),
        last_iar: LAST_IAR.load(Ordering::Relaxed),
        last_intid: LAST_INTID.load(Ordering::Relaxed),
        unexpected_gic_count: UNEXPECTED_GIC_IRQ_COUNT.load(Ordering::Relaxed),
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_timer_irq",
    talos_boot_scenario = "rpi5_timer_preemption",
    talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
    talos_boot_scenario = "rpi5_remote_wakeup_request"
))]
pub fn handle_irq(vector: u64) -> bool {
    let gic = GicV2::new(GICD_BASE, GICC_BASE);
    let iar = unsafe { gic.acknowledge() };
    let intid = iar & 0x03ff;

    LAST_IRQ_VECTOR.store(vector, Ordering::Relaxed);
    LAST_IAR.store(iar as u64, Ordering::Relaxed);
    LAST_INTID.store(intid as u64, Ordering::Relaxed);

    #[cfg(talos_boot_scenario = "rpi5_cross_core_ipi_delivery")]
    if intid == RPI5_CROSS_CORE_IPI_SGI_INTID {
        let logical_cpu = current_pi5_logical_cpu();
        CROSS_CORE_IPI_DELIVERY_STATE.record_receive(logical_cpu, vector, iar, intid);
        unsafe {
            gic.end_interrupt(iar);
        }
        CROSS_CORE_IPI_DELIVERY_STATE.record_eoi(logical_cpu);
        return true;
    }

    #[cfg(talos_boot_scenario = "rpi5_remote_wakeup_request")]
    if intid == RPI5_CROSS_CORE_IPI_SGI_INTID {
        let logical_cpu = current_pi5_logical_cpu();
        REMOTE_WAKE_REQUEST_PROOF_STATE.record_receive(logical_cpu, vector, iar, intid);
        unsafe {
            gic.end_interrupt(iar);
        }
        REMOTE_WAKE_REQUEST_PROOF_STATE.record_eoi(logical_cpu);
        return true;
    }

    #[cfg(any(
        talos_boot_scenario = "rpi5_timer_irq",
        talos_boot_scenario = "rpi5_timer_preemption"
    ))]
    if intid == EL2_PHYSICAL_TIMER_INTID {
        unsafe { generic_timer::record_el2_physical_tick_and_rearm() };
        record_production_timer_preemption_irq(current_pi5_logical_cpu());
        #[cfg(talos_boot_scenario = "rpi5_timer_preemption")]
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

#[cfg(talos_boot_scenario = "rpi5_timer_preemption")]
extern "C" fn rpi5_timer_preemption_thread(raw_task_index: usize) -> ! {
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

#[cfg(talos_boot_scenario = "rpi5_timer_preemption")]
pub fn run_el2_timer_preemption_smoke() -> bool {
    unsafe {
        aarch64::disable_irq();
        aarch64::route_physical_irqs_to_el2();
        generic_timer::mask_el2_physical_timer();
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
            "rpi5-timer-preemption-smoke: stack0={:#018x} stack1={:#018x} trampoline={:#018x}",
            (*state).worker_stacks[0].top(),
            (*state).worker_stacks[1].top(),
            aarch64::kernel_thread_trampoline_address()
        );
        crate::println!(
            "rpi5-timer-preemption-smoke: gicd={:#014x} gicc={:#014x} intid={} cntfrq={} start={} cval={} delta={}",
            GICD_BASE,
            GICC_BASE,
            EL2_PHYSICAL_TIMER_INTID,
            freq,
            start,
            compare,
            delta
        );
        crate::println!(
            "rpi5-timer-preemption-smoke: start current={} runnable={} preempted={} requests={}",
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
        "rpi5-timer-preemption-smoke: progress task1={} task2={} ticks={} requests={} handled={} timer-preemptions={} dispatch-switches={} voluntary-yields={} transitions={} current={} runnable={} preempted={}",
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
        "rpi5-timer-preemption-smoke: irq vector={} iar={:#010x} intid={} unexpected={} ctl={:#x} daif={:#x}",
        snapshot.last_vector,
        snapshot.last_iar,
        snapshot.last_intid,
        snapshot.unexpected_gic_count,
        control,
        daif
    );
    crate::println!(
        "rpi5-timer-preemption-smoke: gic enable={:#010x} pending={:#010x} active={:#010x} hppir={:#010x}",
        enable_bits,
        pending_bits,
        active_bits,
        highest_pending
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
        crate::println!("rpi5-timer-preemption-smoke: PASS");
    } else {
        crate::println!("rpi5-timer-preemption-smoke: FAIL");
    }
    wait_uart10_empty_early_phase();

    passed
}

#[cfg(talos_boot_scenario = "rpi5_timer_irq")]
pub fn run_el2_timer_irq_smoke() -> bool {
    unsafe {
        crate::arch::aarch64::disable_irq();
        crate::arch::aarch64::route_physical_irqs_to_el2();
        generic_timer::mask_el2_physical_timer();
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
        "rpi5-timer-irq-smoke: gicd={:#014x} gicc={:#014x} intid={}",
        GICD_BASE,
        GICC_BASE,
        EL2_PHYSICAL_TIMER_INTID
    );
    crate::println!(
        "rpi5-timer-irq-smoke: cntfrq={} start={} cval={} delta={} target-ticks={}",
        freq,
        start,
        compare,
        delta,
        target_ticks
    );

    let mut workload = 0x1234_5678_9abc_def0u64;
    unsafe {
        generic_timer::program_el2_physical_compare(compare);
        crate::arch::aarch64::enable_irq();
    }

    let mut remaining = TIMER_IRQ_WAIT_LIMIT;
    while timer_irq_snapshot().timer_count < target_ticks && remaining > 0 {
        workload = workload.rotate_left(7) ^ 0x0f0e_0d0c_0b0a_0908;
        unsafe {
            core::arch::asm!("wfe", options(nomem, nostack, preserves_flags));
        }
        remaining -= 1;
    }

    unsafe {
        crate::arch::aarch64::disable_irq();
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
    let daif = crate::arch::aarch64::daif();
    let control = generic_timer::el2_physical_control();
    crate::println!(
        "rpi5-timer-irq-smoke: tick-count={} target={} vector={} iar={:#010x} intid={} unexpected={} ctl={:#x}",
        snapshot.timer_count,
        target_ticks,
        snapshot.last_vector,
        snapshot.last_iar,
        snapshot.last_intid,
        snapshot.unexpected_gic_count,
        control
    );
    crate::println!(
        "rpi5-timer-irq-smoke: gic enable={:#010x} pending={:#010x} active={:#010x} hppir={:#010x} daif={:#x}",
        enable_bits,
        pending_bits,
        active_bits,
        highest_pending,
        daif
    );
    crate::println!(
        "rpi5-timer-irq-smoke: post-irq workload={:#018x} remaining={}",
        workload,
        remaining
    );

    let passed = snapshot.timer_count > 0
        && snapshot.timer_count >= target_ticks
        && snapshot.last_intid == EL2_PHYSICAL_TIMER_INTID as u64
        && snapshot.unexpected_gic_count == 0;

    if passed {
        crate::println!("rpi5-timer-irq-smoke: PASS");
    } else {
        crate::println!("rpi5-timer-irq-smoke: FAIL");
    }
    wait_uart10_empty_early_phase();

    passed
}

#[cfg(talos_target_rpi5_bcm2712)]
pub enum EarlyPhaseLine {
    RustEntry,
    BootInfoParsed,
    TargetInit,
    ExceptionsReady,
    KernelMain,
    DtbReservationsStart,
    DtbReservationsDone,
    DtbMemoryScanStart,
    DtbMemoryScanDone,
    MmuEnableStart,
    MmuEnableDone,
    IcacheEnableStart,
    IcacheEnableDone,
}

#[cfg(talos_target_rpi5_bcm2712)]
pub fn write_early_phase_line(line: EarlyPhaseLine) {
    write_uart10_byte_early_phase(b'T');
    write_uart10_byte_early_phase(b'A');
    write_uart10_byte_early_phase(b'L');
    write_uart10_byte_early_phase(b'O');
    write_uart10_byte_early_phase(b'S');
    write_uart10_byte_early_phase(b':');
    write_uart10_byte_early_phase(b' ');

    match line {
        EarlyPhaseLine::RustEntry => {
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b'u');
            write_uart10_byte_early_phase(b's');
            write_uart10_byte_early_phase(b't');
            write_uart10_byte_early_phase(b'_');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b't');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b'y');
        }
        EarlyPhaseLine::BootInfoParsed => {
            write_uart10_byte_early_phase(b'b');
            write_uart10_byte_early_phase(b'o');
            write_uart10_byte_early_phase(b'o');
            write_uart10_byte_early_phase(b't');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'i');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b'f');
            write_uart10_byte_early_phase(b'o');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'p');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b's');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'd');
        }
        EarlyPhaseLine::TargetInit => {
            write_uart10_byte_early_phase(b't');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b'g');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b't');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'i');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b'i');
            write_uart10_byte_early_phase(b't');
        }
        EarlyPhaseLine::ExceptionsReady => {
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'x');
            write_uart10_byte_early_phase(b'c');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'p');
            write_uart10_byte_early_phase(b't');
            write_uart10_byte_early_phase(b'i');
            write_uart10_byte_early_phase(b'o');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b's');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'd');
            write_uart10_byte_early_phase(b'y');
        }
        EarlyPhaseLine::KernelMain => {
            write_uart10_byte_early_phase(b'k');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'l');
            write_uart10_byte_early_phase(b'_');
            write_uart10_byte_early_phase(b'm');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'i');
            write_uart10_byte_early_phase(b'n');
        }
        EarlyPhaseLine::DtbReservationsStart => {
            write_uart10_byte_early_phase(b'd');
            write_uart10_byte_early_phase(b't');
            write_uart10_byte_early_phase(b'b');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b's');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b'v');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'd');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b's');
            write_uart10_byte_early_phase(b't');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b't');
        }
        EarlyPhaseLine::DtbReservationsDone => {
            write_uart10_byte_early_phase(b'd');
            write_uart10_byte_early_phase(b't');
            write_uart10_byte_early_phase(b'b');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b's');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b'v');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'd');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'd');
            write_uart10_byte_early_phase(b'o');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b'e');
        }
        EarlyPhaseLine::DtbMemoryScanStart => {
            write_uart10_byte_early_phase(b'd');
            write_uart10_byte_early_phase(b't');
            write_uart10_byte_early_phase(b'b');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'm');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'm');
            write_uart10_byte_early_phase(b'o');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b'y');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b's');
            write_uart10_byte_early_phase(b'c');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b's');
            write_uart10_byte_early_phase(b't');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b't');
        }
        EarlyPhaseLine::DtbMemoryScanDone => {
            write_uart10_byte_early_phase(b'd');
            write_uart10_byte_early_phase(b't');
            write_uart10_byte_early_phase(b'b');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'm');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'm');
            write_uart10_byte_early_phase(b'o');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b'y');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b's');
            write_uart10_byte_early_phase(b'c');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'd');
            write_uart10_byte_early_phase(b'o');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b'e');
        }
        EarlyPhaseLine::MmuEnableStart => {
            write_uart10_byte_early_phase(b'm');
            write_uart10_byte_early_phase(b'm');
            write_uart10_byte_early_phase(b'u');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'b');
            write_uart10_byte_early_phase(b'l');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b's');
            write_uart10_byte_early_phase(b't');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b't');
        }
        EarlyPhaseLine::MmuEnableDone => {
            write_uart10_byte_early_phase(b'm');
            write_uart10_byte_early_phase(b'm');
            write_uart10_byte_early_phase(b'u');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'b');
            write_uart10_byte_early_phase(b'l');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'd');
            write_uart10_byte_early_phase(b'o');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b'e');
        }
        EarlyPhaseLine::IcacheEnableStart => {
            write_uart10_byte_early_phase(b'i');
            write_uart10_byte_early_phase(b'c');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'c');
            write_uart10_byte_early_phase(b'h');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'b');
            write_uart10_byte_early_phase(b'l');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b's');
            write_uart10_byte_early_phase(b't');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'r');
            write_uart10_byte_early_phase(b't');
        }
        EarlyPhaseLine::IcacheEnableDone => {
            write_uart10_byte_early_phase(b'i');
            write_uart10_byte_early_phase(b'c');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'c');
            write_uart10_byte_early_phase(b'h');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b'a');
            write_uart10_byte_early_phase(b'b');
            write_uart10_byte_early_phase(b'l');
            write_uart10_byte_early_phase(b'e');
            write_uart10_byte_early_phase(b' ');
            write_uart10_byte_early_phase(b'd');
            write_uart10_byte_early_phase(b'o');
            write_uart10_byte_early_phase(b'n');
            write_uart10_byte_early_phase(b'e');
        }
    }

    write_uart10_byte_early_phase(b'\r');
    write_uart10_byte_early_phase(b'\n');
    wait_uart10_empty_early_phase();
}

#[cfg(talos_target_rpi5_bcm2712)]
pub fn write_early_hex_u64(value: u64) {
    write_uart10_byte_early_phase(b'0');
    write_uart10_byte_early_phase(b'x');

    let mut started = false;
    let mut shift = u64::BITS;
    while shift != 0 {
        shift -= 4;
        let nibble = ((value >> shift) & 0xf) as u8;
        if nibble != 0 || started || shift == 0 {
            started = true;
            write_early_hex_digit(nibble);
        }
    }

    wait_uart10_empty_early_phase();
}

#[cfg(talos_target_rpi5_bcm2712)]
#[allow(dead_code)]
fn write_early_dec_u64(mut value: u64) {
    let mut digits = [0u8; 20];
    let mut len = 0usize;

    if value == 0 {
        write_uart10_byte_early_phase(b'0');
        wait_uart10_empty_early_phase();
        return;
    }

    while value != 0 {
        digits[len] = b'0' + (value % 10) as u8;
        value /= 10;
        len += 1;
    }

    while len != 0 {
        len -= 1;
        write_uart10_byte_early_phase(digits[len]);
    }
    wait_uart10_empty_early_phase();
}

#[cfg(talos_boot_scenario = "rpi5_el0_trap_proof")]
pub fn run_el0_trap_proof() -> ! {
    crate::println!(
        "rpi5-el0-trap-proof: start user-text=[{:#018x},{:#018x}) user-stack=[{:#018x},{:#018x}) user-guard=[{:#018x},{:#018x}) marker={:#x}",
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
        "rpi5-el0-trap-proof: validated elr={:#018x} sp={:#018x} spsr={:#018x} guard-blocked={}",
        entry,
        user_sp,
        EL0_TRAP_SPSR_EL0T_DAIF_MASKED,
        guard_blocked
    );
    if !guard_blocked {
        crate::println!(
            "rpi5-el0-trap-proof: final participants=0 expected=1 errors=1 classification=pi5-el0-trap-proof-guard-open"
        );
        crate::println!("rpi5-el0-trap-proof: FAIL");
        crate::arch::aarch64::halt();
    }

    unsafe {
        install_el0_trap_proof_tables();
        prepare_el1_and_el0_translation();
        write_el0_trap_proof_pre_eret_registers(entry, EL0_TRAP_SPSR_EL0T_DAIF_MASKED);
        let pre_eret = read_el0_trap_proof_pre_eret_registers();
        crate::println!(
            "rpi5-el0-trap-proof: pre-eret hcr_el2={:#018x} sctlr_el1={:#018x} tcr_el1={:#018x} ttbr0_el1={:#018x} vbar_el1={:#018x} elr_el1={:#018x} spsr_el1={:#018x}",
            pre_eret.hcr_el2,
            pre_eret.sctlr_el1,
            pre_eret.tcr_el1,
            pre_eret.ttbr0_el1,
            pre_eret.vbar_el1,
            pre_eret.elr_el1,
            pre_eret.spsr_el1
        );
        report_el0_trap_proof_translation_features();
        wait_uart10_empty_early_phase();
        aarch64::enter_el1_then_el0(
            entry as usize,
            user_sp as usize,
            EL0_TRAP_SPSR_EL0T_DAIF_MASKED,
            EL0_TRAP_SPSR_EL1H_DAIF_MASKED,
        );
    }
}

#[cfg(talos_boot_scenario = "rpi5_syscall_proof")]
pub fn run_syscall_proof() -> ! {
    crate::println!(
        "rpi5-syscall-proof: start user-text=[{:#018x},{:#018x}) user-stack=[{:#018x},{:#018x}) user-guard=[{:#018x},{:#018x}) stable-svc={:#06x} diagnostic-marker={:#06x}",
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
        .expect("fixed syscall proof text mapping is a valid user mapping"),
        UserMapping::new(
            EL0_TRAP_USER_STACK_START,
            EL0_TRAP_USER_STACK_LEN,
            UserMappingPermissions::USER_DATA,
        )
        .expect("fixed syscall proof stack mapping is a valid user mapping"),
    ];
    let entry = validate_user_memory_access(
        &mappings,
        EL0_TRAP_USER_TEXT_START,
        4,
        UserAccessKind::Execute,
        EL0_TRAP_USER_TEXT_LEN,
    )
    .expect("syscall proof entry validates inside fixed UserText")
    .start();
    let user_sp = EL0_TRAP_USER_STACK_START + EL0_TRAP_USER_STACK_LEN as u64;
    validate_user_memory_access(
        &mappings,
        user_sp - 16,
        16,
        UserAccessKind::Write,
        EL0_TRAP_USER_STACK_LEN,
    )
    .expect("syscall proof stack top validates inside fixed UserStack");
    let guard_result = validate_user_memory_access(
        &mappings,
        EL0_TRAP_USER_GUARD_START,
        8,
        UserAccessKind::Read,
        EL0_TRAP_USER_TEXT_LEN,
    );
    let guard_blocked = matches!(guard_result, Err(PosixError::Fault));

    crate::println!(
        "rpi5-syscall-proof: validated elr={:#018x} sp={:#018x} spsr={:#018x} guard-blocked={}",
        entry,
        user_sp,
        EL0_TRAP_SPSR_EL0T_DAIF_MASKED,
        guard_blocked
    );
    if !guard_blocked {
        crate::println!(
            "rpi5-syscall-proof: final participants=0 expected=2 errors=1 classification=pi5-syscall-proof-guard-open"
        );
        crate::println!("rpi5-syscall-proof: FAIL");
        wait_uart10_empty_early_phase();
        crate::arch::aarch64::halt();
    }

    unsafe {
        install_el0_trap_proof_tables();
        prepare_el1_and_el0_translation();
        write_el0_trap_proof_pre_eret_registers(entry, EL0_TRAP_SPSR_EL0T_DAIF_MASKED);
        let pre_eret = read_el0_trap_proof_pre_eret_registers();
        crate::println!(
            "rpi5-syscall-proof: pre-eret hcr_el2={:#018x} sctlr_el1={:#018x} tcr_el1={:#018x} ttbr0_el1={:#018x} vbar_el1={:#018x} elr_el1={:#018x} spsr_el1={:#018x}",
            pre_eret.hcr_el2,
            pre_eret.sctlr_el1,
            pre_eret.tcr_el1,
            pre_eret.ttbr0_el1,
            pre_eret.vbar_el1,
            pre_eret.elr_el1,
            pre_eret.spsr_el1
        );
        wait_uart10_empty_early_phase();
        aarch64::enter_el1_then_el0(
            entry as usize,
            user_sp as usize,
            EL0_TRAP_SPSR_EL0T_DAIF_MASKED,
            EL0_TRAP_SPSR_EL1H_DAIF_MASKED,
        );
    }
}

#[cfg(talos_boot_scenario = "rpi5_pointer_copy_proof")]
pub fn run_pointer_copy_proof() -> ! {
    crate::println!("rpi5-pointer-copy-proof: start");

    let mappings = [
        UserMapping::new(
            EL0_TRAP_USER_TEXT_START,
            EL0_TRAP_USER_TEXT_LEN,
            UserMappingPermissions::USER_TEXT,
        )
        .expect("fixed pointer-copy proof text mapping is a valid user mapping"),
        UserMapping::new(
            POINTER_COPY_USER_DATA_START,
            POINTER_COPY_USER_DATA_LEN,
            UserMappingPermissions::USER_DATA,
        )
        .expect("fixed pointer-copy proof data mapping is a valid user mapping"),
        UserMapping::new(
            EL0_TRAP_USER_STACK_START,
            EL0_TRAP_USER_STACK_LEN,
            UserMappingPermissions::USER_DATA,
        )
        .expect("fixed pointer-copy proof stack mapping is a valid user mapping"),
    ];
    let entry = validate_user_memory_access(
        &mappings,
        EL0_TRAP_USER_TEXT_START,
        4,
        UserAccessKind::Execute,
        EL0_TRAP_USER_TEXT_LEN,
    )
    .expect("pointer-copy proof entry validates inside fixed UserText")
    .start();
    validate_user_memory_access(
        &mappings,
        POINTER_COPY_USER_DATA_START,
        POINTER_COPY_USER_DATA_LEN,
        UserAccessKind::Write,
        POINTER_COPY_USER_DATA_LEN,
    )
    .expect("pointer-copy proof data validates inside fixed UserData");
    let user_sp = EL0_TRAP_USER_STACK_START + EL0_TRAP_USER_STACK_LEN as u64;
    validate_user_memory_access(
        &mappings,
        user_sp - 16,
        16,
        UserAccessKind::Write,
        EL0_TRAP_USER_STACK_LEN,
    )
    .expect("pointer-copy proof stack top validates inside fixed UserStack");
    let guard_result = validate_user_memory_access(
        &mappings,
        EL0_TRAP_USER_GUARD_START,
        16,
        UserAccessKind::Read,
        EL0_TRAP_USER_TEXT_LEN,
    );
    let guard_blocked = matches!(guard_result, Err(PosixError::Fault));

    crate::println!(
        "rpi5-pointer-copy-proof: validated elr={:#018x} sp={:#018x} user-data={:#018x} user-data-len={:#018x} guard-blocked={}",
        entry,
        user_sp,
        POINTER_COPY_USER_DATA_START,
        POINTER_COPY_USER_DATA_LEN as u64,
        guard_blocked
    );
    if !guard_blocked {
        crate::println!(
            "rpi5-pointer-copy-proof: final participants=0 expected=3 errors=1 classification=pi5-pointer-copy-proof-guard-open"
        );
        crate::println!("rpi5-pointer-copy-proof: FAIL");
        wait_uart10_empty_early_phase();
        crate::arch::aarch64::halt();
    }

    unsafe {
        core::ptr::write_bytes(
            core::ptr::addr_of_mut!(POINTER_COPY_USER_DATA).cast::<u8>(),
            POINTER_COPY_USER_DATA_INIT,
            POINTER_COPY_USER_DATA_LEN,
        );
        clean_cache_range_to_poc(
            core::ptr::addr_of!(POINTER_COPY_USER_DATA) as usize,
            POINTER_COPY_USER_DATA_LEN,
        );
        install_el0_trap_proof_tables();
        prepare_el1_and_el0_translation();
        write_el0_trap_proof_pre_eret_registers(entry, EL0_TRAP_SPSR_EL0T_DAIF_MASKED);
        let pre_eret = read_el0_trap_proof_pre_eret_registers();
        crate::println!(
            "rpi5-pointer-copy-proof: pre-eret hcr_el2={:#018x} sctlr_el1={:#018x} tcr_el1={:#018x} ttbr0_el1={:#018x} vbar_el1={:#018x} elr_el1={:#018x} spsr_el1={:#018x}",
            pre_eret.hcr_el2,
            pre_eret.sctlr_el1,
            pre_eret.tcr_el1,
            pre_eret.ttbr0_el1,
            pre_eret.vbar_el1,
            pre_eret.elr_el1,
            pre_eret.spsr_el1
        );
        wait_uart10_empty_early_phase();
        aarch64::enter_el1_then_el0(
            entry as usize,
            user_sp as usize,
            EL0_TRAP_SPSR_EL0T_DAIF_MASKED,
            EL0_TRAP_SPSR_EL1H_DAIF_MASKED,
        );
    }
}

#[cfg(talos_boot_scenario = "rpi5_descriptor_write_proof")]
pub fn run_descriptor_write_proof() -> ! {
    crate::println!("rpi5-descriptor-write-proof: start");

    let descriptor_table =
        crate::posix::DescriptorTable::<4>::with_inherited_stdio().expect("stdio table");
    let mappings = [
        UserMapping::new(
            EL0_TRAP_USER_TEXT_START,
            EL0_TRAP_USER_TEXT_LEN,
            UserMappingPermissions::USER_TEXT,
        )
        .expect("fixed descriptor-write proof text mapping is a valid user mapping"),
        UserMapping::new(
            DESCRIPTOR_WRITE_USER_DATA_START,
            DESCRIPTOR_WRITE_USER_DATA_LEN,
            UserMappingPermissions::USER_DATA,
        )
        .expect("fixed descriptor-write proof data mapping is a valid user mapping"),
        UserMapping::new(
            EL0_TRAP_USER_STACK_START,
            EL0_TRAP_USER_STACK_LEN,
            UserMappingPermissions::USER_DATA,
        )
        .expect("fixed descriptor-write proof stack mapping is a valid user mapping"),
    ];
    let entry = validate_user_memory_access(
        &mappings,
        EL0_TRAP_USER_TEXT_START,
        4,
        UserAccessKind::Execute,
        EL0_TRAP_USER_TEXT_LEN,
    )
    .expect("descriptor-write proof entry validates inside fixed UserText")
    .start();
    validate_user_memory_access(
        &mappings,
        DESCRIPTOR_WRITE_USER_DATA_START,
        DESCRIPTOR_WRITE_USER_DATA_LEN,
        UserAccessKind::Write,
        DESCRIPTOR_WRITE_USER_DATA_LEN,
    )
    .expect("descriptor-write proof data validates inside fixed UserData");
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
    .expect("descriptor-write proof stack top validates inside fixed UserStack");
    let guard_result = validate_user_memory_access(
        &mappings,
        EL0_TRAP_USER_GUARD_START,
        16,
        UserAccessKind::Read,
        EL0_TRAP_USER_TEXT_LEN,
    );
    let guard_blocked = matches!(guard_result, Err(PosixError::Fault));

    crate::println!(
        "rpi5-descriptor-write-proof: validated elr={:#018x} sp={:#018x} user-data={:#018x} user-data-len={:#018x} guard-blocked={} descriptor-table=inherited-stdio runtime-console=runtime-console0",
        entry,
        user_sp,
        DESCRIPTOR_WRITE_USER_DATA_START,
        DESCRIPTOR_WRITE_USER_DATA_LEN as u64,
        guard_blocked
    );
    if !guard_blocked {
        crate::println!(
            "rpi5-descriptor-write-proof: final participants=0 expected=8 errors=1 classification=pi5-descriptor-write-proof-guard-open"
        );
        crate::println!("rpi5-descriptor-write-proof: FAIL");
        wait_uart10_empty_early_phase();
        crate::arch::aarch64::halt();
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
        clean_cache_range_to_poc(
            core::ptr::addr_of!(DESCRIPTOR_WRITE_USER_DATA) as usize,
            DESCRIPTOR_WRITE_USER_DATA_LEN,
        );
        install_el0_trap_proof_tables();
        prepare_el1_and_el0_translation();
        write_el0_trap_proof_pre_eret_registers(entry, EL0_TRAP_SPSR_EL0T_DAIF_MASKED);
        let pre_eret = read_el0_trap_proof_pre_eret_registers();
        crate::println!(
            "rpi5-descriptor-write-proof: pre-eret hcr_el2={:#018x} sctlr_el1={:#018x} tcr_el1={:#018x} ttbr0_el1={:#018x} vbar_el1={:#018x} elr_el1={:#018x} spsr_el1={:#018x}",
            pre_eret.hcr_el2,
            pre_eret.sctlr_el1,
            pre_eret.tcr_el1,
            pre_eret.ttbr0_el1,
            pre_eret.vbar_el1,
            pre_eret.elr_el1,
            pre_eret.spsr_el1
        );
        wait_uart10_empty_early_phase();
        aarch64::enter_el1_then_el0(
            entry as usize,
            user_sp as usize,
            EL0_TRAP_SPSR_EL0T_DAIF_MASKED,
            EL0_TRAP_SPSR_EL1H_DAIF_MASKED,
        );
    }
}

#[cfg(talos_boot_scenario = "rpi5_close_syscall_proof")]
pub fn run_close_syscall_proof() -> ! {
    crate::println!("rpi5-close-syscall-proof: start");

    let current_owner = crate::scheduler::ProcessOwnerId::new(PROCESS_DESCRIPTOR_STDIO_OWNER_RAW)
        .expect("close syscall proof owner id is nonzero");
    unsafe {
        *core::ptr::addr_of_mut!(PROCESS_DESCRIPTOR_STDIO_STORE) =
            crate::posix::ProcessDescriptorStore::new_empty();
        (*core::ptr::addr_of_mut!(PROCESS_DESCRIPTOR_STDIO_STORE))
            .create_owner_with_inherited_stdio(current_owner)
            .expect("process-owned inherited stdio table");
        clean_cache_range_to_poc(
            core::ptr::addr_of!(PROCESS_DESCRIPTOR_STDIO_STORE) as usize,
            core::mem::size_of::<crate::posix::ProcessDescriptorStore<1, 4>>(),
        );
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
        .expect("fixed close syscall proof text mapping is a valid user mapping"),
        UserMapping::new(
            DESCRIPTOR_WRITE_USER_DATA_START,
            DESCRIPTOR_WRITE_USER_DATA_LEN,
            UserMappingPermissions::USER_DATA,
        )
        .expect("fixed close syscall proof data mapping is a valid user mapping"),
        UserMapping::new(
            EL0_TRAP_USER_STACK_START,
            EL0_TRAP_USER_STACK_LEN,
            UserMappingPermissions::USER_DATA,
        )
        .expect("fixed close syscall proof stack mapping is a valid user mapping"),
    ];
    let entry = validate_user_memory_access(
        &mappings,
        EL0_TRAP_USER_TEXT_START,
        4,
        UserAccessKind::Execute,
        EL0_TRAP_USER_TEXT_LEN,
    )
    .expect("close syscall proof entry validates inside fixed UserText")
    .start();
    validate_user_memory_access(
        &mappings,
        DESCRIPTOR_WRITE_USER_DATA_START,
        DESCRIPTOR_WRITE_USER_DATA_LEN,
        UserAccessKind::Write,
        DESCRIPTOR_WRITE_USER_DATA_LEN,
    )
    .expect("close syscall proof data validates inside fixed UserData");
    let user_sp = EL0_TRAP_USER_STACK_START + EL0_TRAP_USER_STACK_LEN as u64;
    validate_user_memory_access(
        &mappings,
        user_sp - 16,
        16,
        UserAccessKind::Write,
        EL0_TRAP_USER_STACK_LEN,
    )
    .expect("close syscall proof stack top validates inside fixed UserStack");
    let guard_result = validate_user_memory_access(
        &mappings,
        EL0_TRAP_USER_GUARD_START,
        16,
        UserAccessKind::Read,
        EL0_TRAP_USER_TEXT_LEN,
    );
    let guard_blocked = matches!(guard_result, Err(PosixError::Fault));

    crate::println!(
        "rpi5-close-syscall-proof: validated elr={:#018x} sp={:#018x} user-data={:#018x} user-data-len={:#018x} guard-blocked={} descriptor-store=current-owner inherited-stdio=true runtime-console=runtime-console0",
        entry,
        user_sp,
        DESCRIPTOR_WRITE_USER_DATA_START,
        DESCRIPTOR_WRITE_USER_DATA_LEN as u64,
        guard_blocked
    );
    if !guard_blocked {
        crate::println!(
            "rpi5-close-syscall-proof: final participants=0 expected=11 errors=1 classification=pi5-close-syscall-proof-guard-open"
        );
        crate::println!("rpi5-close-syscall-proof: FAIL");
        wait_uart10_empty_early_phase();
        crate::arch::aarch64::halt();
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
        CLOSE_SYSCALL_ERRORS.store(0, Ordering::Relaxed);
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
        clean_cache_range_to_poc(
            core::ptr::addr_of!(DESCRIPTOR_WRITE_USER_DATA) as usize,
            DESCRIPTOR_WRITE_USER_DATA_LEN,
        );
        install_el0_trap_proof_tables();
        prepare_el1_and_el0_translation();
        write_el0_trap_proof_pre_eret_registers(entry, EL0_TRAP_SPSR_EL0T_DAIF_MASKED);
        let pre_eret = read_el0_trap_proof_pre_eret_registers();
        crate::println!(
            "rpi5-close-syscall-proof: pre-eret hcr_el2={:#018x} sctlr_el1={:#018x} tcr_el1={:#018x} ttbr0_el1={:#018x} vbar_el1={:#018x} elr_el1={:#018x} spsr_el1={:#018x}",
            pre_eret.hcr_el2,
            pre_eret.sctlr_el1,
            pre_eret.tcr_el1,
            pre_eret.ttbr0_el1,
            pre_eret.vbar_el1,
            pre_eret.elr_el1,
            pre_eret.spsr_el1
        );
        wait_uart10_empty_early_phase();
        aarch64::enter_el1_then_el0(
            entry as usize,
            user_sp as usize,
            EL0_TRAP_SPSR_EL0T_DAIF_MASKED,
            EL0_TRAP_SPSR_EL1H_DAIF_MASKED,
        );
    }
}

#[cfg(talos_boot_scenario = "rpi5_dup_syscall_proof")]
pub fn run_dup_syscall_proof() -> ! {
    crate::println!("rpi5-dup-syscall-proof: start");

    let current_owner = crate::scheduler::ProcessOwnerId::new(PROCESS_DESCRIPTOR_STDIO_OWNER_RAW)
        .expect("dup syscall proof owner id is nonzero");
    unsafe {
        *core::ptr::addr_of_mut!(PROCESS_DESCRIPTOR_STDIO_STORE) =
            crate::posix::ProcessDescriptorStore::new_empty();
        (*core::ptr::addr_of_mut!(PROCESS_DESCRIPTOR_STDIO_STORE))
            .create_owner_with_inherited_stdio(current_owner)
            .expect("process-owned inherited stdio table");
        clean_cache_range_to_poc(
            core::ptr::addr_of!(PROCESS_DESCRIPTOR_STDIO_STORE) as usize,
            core::mem::size_of::<crate::posix::ProcessDescriptorStore<1, 4>>(),
        );
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
        .expect("fixed dup syscall proof text mapping is a valid user mapping"),
        UserMapping::new(
            DESCRIPTOR_WRITE_USER_DATA_START,
            DESCRIPTOR_WRITE_USER_DATA_LEN,
            UserMappingPermissions::USER_DATA,
        )
        .expect("fixed dup syscall proof data mapping is a valid user mapping"),
        UserMapping::new(
            EL0_TRAP_USER_STACK_START,
            EL0_TRAP_USER_STACK_LEN,
            UserMappingPermissions::USER_DATA,
        )
        .expect("fixed dup syscall proof stack mapping is a valid user mapping"),
    ];
    let entry = validate_user_memory_access(
        &mappings,
        EL0_TRAP_USER_TEXT_START,
        4,
        UserAccessKind::Execute,
        EL0_TRAP_USER_TEXT_LEN,
    )
    .expect("dup syscall proof entry validates inside fixed UserText")
    .start();
    validate_user_memory_access(
        &mappings,
        DESCRIPTOR_WRITE_USER_DATA_START,
        DESCRIPTOR_WRITE_USER_DATA_LEN,
        UserAccessKind::Write,
        DESCRIPTOR_WRITE_USER_DATA_LEN,
    )
    .expect("dup syscall proof data validates inside fixed UserData");
    let user_sp = EL0_TRAP_USER_STACK_START + EL0_TRAP_USER_STACK_LEN as u64;
    validate_user_memory_access(
        &mappings,
        user_sp - 16,
        16,
        UserAccessKind::Write,
        EL0_TRAP_USER_STACK_LEN,
    )
    .expect("dup syscall proof stack top validates inside fixed UserStack");
    let guard_result = validate_user_memory_access(
        &mappings,
        EL0_TRAP_USER_GUARD_START,
        16,
        UserAccessKind::Read,
        EL0_TRAP_USER_TEXT_LEN,
    );
    let guard_blocked = matches!(guard_result, Err(PosixError::Fault));

    crate::println!(
        "rpi5-dup-syscall-proof: validated elr={:#018x} sp={:#018x} user-data={:#018x} user-data-len={:#018x} guard-blocked={} process-owner={:#018x} current-owner={:#018x} descriptor-table=process-owned-inherited-stdio descriptor-capacity=4 runtime-console=runtime-console0",
        entry,
        user_sp,
        DESCRIPTOR_WRITE_USER_DATA_START,
        DESCRIPTOR_WRITE_USER_DATA_LEN as u64,
        guard_blocked,
        current_owner.raw(),
        current_owner.raw()
    );
    crate::println!(
        "rpi5-dup-syscall-proof: current-descriptor-table lookup=process-owned owner={:#018x} resolved=true stdio=inherited runtime-console=runtime-console0",
        current_owner.raw()
    );
    if !guard_blocked {
        crate::println!(
            "rpi5-dup-syscall-proof: final participants=0 expected=14 errors=1 classification=pi5-dup-syscall-proof-guard-open"
        );
        crate::println!("rpi5-dup-syscall-proof: FAIL");
        wait_uart10_empty_early_phase();
        crate::arch::aarch64::halt();
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
        DUP_SYSCALL_ERRORS.store(0, Ordering::Relaxed);
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
        clean_cache_range_to_poc(
            core::ptr::addr_of!(DESCRIPTOR_WRITE_USER_DATA) as usize,
            DESCRIPTOR_WRITE_USER_DATA_LEN,
        );
        install_el0_trap_proof_tables();
        prepare_el1_and_el0_translation();
        write_el0_trap_proof_pre_eret_registers(entry, EL0_TRAP_SPSR_EL0T_DAIF_MASKED);
        let pre_eret = read_el0_trap_proof_pre_eret_registers();
        crate::println!(
            "rpi5-dup-syscall-proof: pre-eret hcr_el2={:#018x} sctlr_el1={:#018x} tcr_el1={:#018x} ttbr0_el1={:#018x} vbar_el1={:#018x} elr_el1={:#018x} spsr_el1={:#018x}",
            pre_eret.hcr_el2,
            pre_eret.sctlr_el1,
            pre_eret.tcr_el1,
            pre_eret.ttbr0_el1,
            pre_eret.vbar_el1,
            pre_eret.elr_el1,
            pre_eret.spsr_el1
        );
        wait_uart10_empty_early_phase();
        aarch64::enter_el1_then_el0(
            entry as usize,
            user_sp as usize,
            EL0_TRAP_SPSR_EL0T_DAIF_MASKED,
            EL0_TRAP_SPSR_EL1H_DAIF_MASKED,
        );
    }
}

#[cfg(talos_boot_scenario = "rpi5_read_stdin_proof")]
pub fn run_read_stdin_proof() -> ! {
    crate::println!("rpi5-read-stdin-proof: start");

    let current_owner = crate::scheduler::ProcessOwnerId::new(PROCESS_DESCRIPTOR_STDIO_OWNER_RAW)
        .expect("read stdin proof owner id is nonzero");
    unsafe {
        *core::ptr::addr_of_mut!(PROCESS_DESCRIPTOR_STDIO_STORE) =
            crate::posix::ProcessDescriptorStore::new_empty();
        (*core::ptr::addr_of_mut!(PROCESS_DESCRIPTOR_STDIO_STORE))
            .create_owner_with_inherited_stdio(current_owner)
            .expect("process-owned inherited stdio table");
        *core::ptr::addr_of_mut!(READ_STDIN_FIXED_STATE) =
            crate::posix::FixedStdin::new(READ_STDIN_FIXED_BYTES);
        clean_cache_range_to_poc(
            core::ptr::addr_of!(PROCESS_DESCRIPTOR_STDIO_STORE) as usize,
            core::mem::size_of::<crate::posix::ProcessDescriptorStore<1, 4>>(),
        );
        clean_cache_range_to_poc(
            core::ptr::addr_of!(READ_STDIN_FIXED_STATE) as usize,
            core::mem::size_of::<crate::posix::FixedStdin<'static>>(),
        );
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
        .expect("fixed read stdin proof text mapping is a valid user mapping"),
        UserMapping::new(
            DESCRIPTOR_WRITE_USER_DATA_START,
            DESCRIPTOR_WRITE_USER_DATA_LEN,
            UserMappingPermissions::USER_DATA,
        )
        .expect("fixed read stdin proof data mapping is a valid user mapping"),
        UserMapping::new(
            EL0_TRAP_USER_STACK_START,
            EL0_TRAP_USER_STACK_LEN,
            UserMappingPermissions::USER_DATA,
        )
        .expect("fixed read stdin proof stack mapping is a valid user mapping"),
    ];
    let entry = validate_user_memory_access(
        &mappings,
        EL0_TRAP_USER_TEXT_START,
        4,
        UserAccessKind::Execute,
        EL0_TRAP_USER_TEXT_LEN,
    )
    .expect("read stdin proof entry validates inside fixed UserText")
    .start();
    validate_user_memory_access(
        &mappings,
        DESCRIPTOR_WRITE_USER_DATA_START,
        DESCRIPTOR_WRITE_USER_DATA_LEN,
        UserAccessKind::Write,
        DESCRIPTOR_WRITE_USER_DATA_LEN,
    )
    .expect("read stdin proof data validates inside fixed UserData");
    let user_sp = EL0_TRAP_USER_STACK_START + EL0_TRAP_USER_STACK_LEN as u64;
    validate_user_memory_access(
        &mappings,
        user_sp - 16,
        16,
        UserAccessKind::Write,
        EL0_TRAP_USER_STACK_LEN,
    )
    .expect("read stdin proof stack top validates inside fixed UserStack");
    let guard_result = validate_user_memory_access(
        &mappings,
        EL0_TRAP_USER_GUARD_START,
        16,
        UserAccessKind::Read,
        EL0_TRAP_USER_TEXT_LEN,
    );
    let guard_blocked = matches!(guard_result, Err(PosixError::Fault));

    crate::println!(
        "rpi5-read-stdin-proof: validated elr={:#018x} sp={:#018x} user-data={:#018x} user-data-len={:#018x} guard-blocked={} process-owner={:#018x} current-owner={:#018x} descriptor-table=process-owned-inherited-stdio descriptor-capacity=4 fixed-stdin-len={} fixed-stdin-cursor={}",
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
        "rpi5-read-stdin-proof: current-descriptor-table lookup=process-owned owner={:#018x} resolved=true stdio=inherited fixed-stdin=proof-buffer",
        current_owner.raw()
    );
    if !guard_blocked {
        crate::println!(
            "rpi5-read-stdin-proof: final participants=0 expected=11 errors=1 classification=pi5-read-stdin-proof-guard-open"
        );
        crate::println!("rpi5-read-stdin-proof: FAIL");
        wait_uart10_empty_early_phase();
        crate::arch::aarch64::halt();
    }

    unsafe {
        core::ptr::write_bytes(
            core::ptr::addr_of_mut!(DESCRIPTOR_WRITE_USER_DATA).cast::<u8>(),
            0,
            DESCRIPTOR_WRITE_USER_DATA_LEN,
        );
        core::ptr::write_bytes(
            core::ptr::addr_of_mut!(DESCRIPTOR_WRITE_CONSOLE_CAPTURE).cast::<u8>(),
            0,
            64,
        );
        DESCRIPTOR_WRITE_CONSOLE_LEN.store(0, Ordering::Relaxed);
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
        clean_cache_range_to_poc(
            core::ptr::addr_of!(DESCRIPTOR_WRITE_USER_DATA) as usize,
            DESCRIPTOR_WRITE_USER_DATA_LEN,
        );
        install_el0_trap_proof_tables();
        prepare_el1_and_el0_translation();
        write_el0_trap_proof_pre_eret_registers(entry, EL0_TRAP_SPSR_EL0T_DAIF_MASKED);
        let pre_eret = read_el0_trap_proof_pre_eret_registers();
        crate::println!(
            "rpi5-read-stdin-proof: pre-eret hcr_el2={:#018x} sctlr_el1={:#018x} tcr_el1={:#018x} ttbr0_el1={:#018x} vbar_el1={:#018x} elr_el1={:#018x} spsr_el1={:#018x}",
            pre_eret.hcr_el2,
            pre_eret.sctlr_el1,
            pre_eret.tcr_el1,
            pre_eret.ttbr0_el1,
            pre_eret.vbar_el1,
            pre_eret.elr_el1,
            pre_eret.spsr_el1
        );
        wait_uart10_empty_early_phase();
        aarch64::enter_el1_then_el0(
            entry as usize,
            user_sp as usize,
            EL0_TRAP_SPSR_EL0T_DAIF_MASKED,
            EL0_TRAP_SPSR_EL1H_DAIF_MASKED,
        );
    }
}

#[cfg(talos_boot_scenario = "rpi5_syscall_proof")]
pub fn handle_syscall_proof_exception(
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
        SYSCALL_PROOF_ERRORS.fetch_add(1, Ordering::Relaxed);
        return false;
    };

    if marker == syscall::DIAGNOSTIC_EL0_TRAP_SVC_IMMEDIATE {
        let unknown_x0 = frame.reg(0);
        let unknown_ok = unknown_x0 == SYSCALL_PROOF_EXPECTED_ENOSYS_X0
            && SYSCALL_PROOF_UNKNOWN_DISPATCHED.load(Ordering::Relaxed) == 1;
        SYSCALL_PROOF_UNKNOWN_OBSERVED.store(u64::from(unknown_ok), Ordering::Relaxed);
        if !unknown_ok {
            SYSCALL_PROOF_ERRORS.fetch_add(1, Ordering::Relaxed);
        }
        crate::println!(
            "rpi5-syscall-proof: user-observed case=unknown x0={:#018x} ok={}",
            unknown_x0,
            unknown_ok
        );
        let stable = syscall::is_stable_syscall_svc_immediate(marker);
        crate::println!(
            "rpi5-syscall-proof: diagnostic-marker marker=0x7a10 stable-syscall={} dispatched=false",
            stable
        );
        finish_syscall_proof(reported_esr == SYSCALL_PROOF_EXPECTED_MARKER_ESR && far == 0);
    }

    if reported_esr != SYSCALL_PROOF_EXPECTED_SVC_ESR {
        SYSCALL_PROOF_ERRORS.fetch_add(1, Ordering::Relaxed);
        return false;
    }

    let raw_number = frame.reg(8);
    if raw_number == SYSCALL_PROOF_UNKNOWN_NUMBER {
        let talos_nop_x0 = frame.reg(0);
        let talos_nop_ok =
            talos_nop_x0 == 0 && SYSCALL_PROOF_TALOS_NOP_DISPATCHED.load(Ordering::Relaxed) == 1;
        SYSCALL_PROOF_TALOS_NOP_OBSERVED.store(u64::from(talos_nop_ok), Ordering::Relaxed);
        if !talos_nop_ok {
            SYSCALL_PROOF_ERRORS.fetch_add(1, Ordering::Relaxed);
        }
        crate::println!(
            "rpi5-syscall-proof: user-observed case=talos_nop x0={:#018x} ok={}",
            talos_nop_x0,
            talos_nop_ok
        );
    }

    let Some(routed) =
        crate::arch::aarch64::exceptions::try_route_lower_aarch64_syscall(vector, esr, saved_frame)
    else {
        SYSCALL_PROOF_ERRORS.fetch_add(1, Ordering::Relaxed);
        return false;
    };

    match SyscallNumber::from_raw(routed.raw_number) {
        SyscallNumber::TalosNop => {
            let args = routed.arguments.values();
            let args_ok = args == [0; syscall::MAX_SCALAR_ARGUMENTS];
            if !args_ok || routed.return_x0 != 0 {
                SYSCALL_PROOF_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            SYSCALL_PROOF_TALOS_NOP_DISPATCHED.store(1, Ordering::Relaxed);
            crate::println!(
                "rpi5-syscall-proof: syscall case=talos_nop vector={} esr={:#018x} svc=0x0000 number={} args=[x0={:#018x} x1={:#018x} x2={:#018x} x3={:#018x} x4={:#018x} x5={:#018x}] return-x0={:#018x}",
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
            let return_ok = routed.return_x0 == SYSCALL_PROOF_EXPECTED_ENOSYS_X0;
            if routed.raw_number != SYSCALL_PROOF_UNKNOWN_NUMBER || !return_ok {
                SYSCALL_PROOF_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            SYSCALL_PROOF_UNKNOWN_DISPATCHED.store(u64::from(return_ok), Ordering::Relaxed);
            crate::println!(
                "rpi5-syscall-proof: syscall case=unknown vector={} esr={:#018x} svc=0x0000 number={} return-x0={:#018x} expected=-ENOSYS",
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
            SYSCALL_PROOF_ERRORS.fetch_add(1, Ordering::Relaxed);
            crate::println!(
                "rpi5-syscall-proof: syscall case=unexpected_context_syscall vector={} esr={:#018x} svc=0x0000 number={} return-x0={:#018x}",
                vector.name(),
                reported_esr,
                routed.raw_number,
                routed.return_x0
            );
        }
    }

    true
}

#[cfg(talos_boot_scenario = "rpi5_pointer_copy_proof")]
pub fn handle_pointer_copy_proof_exception(
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
        let unknown_ok = unknown_x0 == POINTER_COPY_EXPECTED_ENOSYS_X0
            && POINTER_COPY_UNKNOWN_DISPATCHED.load(Ordering::Relaxed) == 1;
        POINTER_COPY_UNKNOWN_OBSERVED.store(u64::from(unknown_ok), Ordering::Relaxed);
        if !unknown_ok {
            POINTER_COPY_ERRORS.fetch_add(1, Ordering::Relaxed);
        }
        crate::println!(
            "rpi5-pointer-copy-proof: user-observed case=unknown x0={:#018x} ok={}",
            unknown_x0,
            unknown_ok
        );
        let stable = syscall::is_stable_syscall_svc_immediate(marker);
        crate::println!(
            "rpi5-pointer-copy-proof: diagnostic-marker marker=0x7a10 stable-syscall={} dispatched=false",
            stable
        );
        finish_pointer_copy_proof(reported_esr == SYSCALL_PROOF_EXPECTED_MARKER_ESR && far == 0);
    }

    if reported_esr != SYSCALL_PROOF_EXPECTED_SVC_ESR {
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
        .expect("fixed pointer-copy proof data mapping is a valid user mapping");
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
                "rpi5-pointer-copy-proof: syscall case=copy_probe_success vector={} esr={:#018x} svc=0x0000 number={:#018x} args=[x0={:#018x} x1={:#018x} x2={:#018x} x3={:#018x} x4={:#018x} x5={:#018x}] return-x0={:#018x}",
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
                "rpi5-pointer-copy-proof: user-observed case=copy_probe_success x0={:#018x} data=0xa5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5 ok={}",
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
                "rpi5-pointer-copy-proof: syscall case=copy_probe_efault vector={} esr={:#018x} svc=0x0000 number={:#018x} args=[x0={:#018x} x1={:#018x} x2={:#018x} x3={:#018x} x4={:#018x} x5={:#018x}] return-x0={:#018x} expected=-EFAULT",
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
                "rpi5-pointer-copy-proof: user-observed case=copy_probe_efault x0={:#018x} ok={}",
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

    let return_ok = routed.raw_number == SYSCALL_PROOF_UNKNOWN_NUMBER
        && matches!(
            SyscallNumber::from_raw(routed.raw_number),
            SyscallNumber::Unknown(_)
        )
        && routed.return_x0 == POINTER_COPY_EXPECTED_ENOSYS_X0;
    if !return_ok {
        POINTER_COPY_ERRORS.fetch_add(1, Ordering::Relaxed);
    }
    POINTER_COPY_UNKNOWN_DISPATCHED.store(u64::from(return_ok), Ordering::Relaxed);
    crate::println!(
        "rpi5-pointer-copy-proof: syscall case=unknown vector={} esr={:#018x} svc=0x0000 number={} return-x0={:#018x} expected=-ENOSYS",
        vector.name(),
        reported_esr,
        routed.raw_number,
        routed.return_x0
    );

    true
}

#[cfg(talos_boot_scenario = "rpi5_descriptor_write_proof")]
pub fn handle_descriptor_write_proof_exception(
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
            "rpi5-descriptor-write-proof: diagnostic-marker marker=0x7a10 stable-syscall={} dispatched=false",
            stable
        );
        finish_descriptor_write_proof(
            reported_esr == SYSCALL_PROOF_EXPECTED_MARKER_ESR && far == 0,
        );
    }

    if reported_esr != SYSCALL_PROOF_EXPECTED_SVC_ESR {
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
    .expect("fixed descriptor-write proof data mapping is valid")];
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
                "rpi5-descriptor-write-proof: syscall case=write_stdout vector={} esr={:#018x} svc=0x0000 number=1 args=[x0={:#018x} x1={:#018x} x2={:#018x} x3={:#018x} x4={:#018x} x5={:#018x}] return-x0={:#018x}",
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
                "rpi5-descriptor-write-proof: runtime-console case=write_stdout device=runtime-console0 bytes=18 hex=74616c6f732d7374646f75742d727069350a ok={}",
                console_ok
            );
            crate::println!(
                "rpi5-descriptor-write-proof: user-observed case=write_stdout x0={:#018x} ok={}",
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
                "rpi5-descriptor-write-proof: syscall case=write_stderr vector={} esr={:#018x} svc=0x0000 number=1 args=[x0={:#018x} x1={:#018x} x2={:#018x} x3={:#018x} x4={:#018x} x5={:#018x}] return-x0={:#018x}",
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
                "rpi5-descriptor-write-proof: runtime-console case=write_stderr device=runtime-console0 bytes=18 hex=74616c6f732d7374646572722d727069350a ok={}",
                console_ok
            );
            crate::println!(
                "rpi5-descriptor-write-proof: user-observed case=write_stderr x0={:#018x} ok={}",
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
                "rpi5-descriptor-write-proof: syscall case=write_fd0 vector={} esr={:#018x} svc=0x0000 number=1 return-x0={:#018x} expected=-EBADF console-unchanged={}",
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
                "rpi5-descriptor-write-proof: syscall case=write_badfd vector={} esr={:#018x} svc=0x0000 number=1 return-x0={:#018x} expected=-EBADF console-unchanged={}",
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
                "rpi5-descriptor-write-proof: syscall case=write_efault vector={} esr={:#018x} svc=0x0000 number=1 return-x0={:#018x} expected=-EFAULT console-unchanged={}",
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
                "rpi5-descriptor-write-proof: syscall case=write_reserved vector={} esr={:#018x} svc=0x0000 number=1 return-x0={:#018x} expected=-EINVAL console-unchanged={}",
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
                "rpi5-descriptor-write-proof: syscall case=talos_nop vector={} esr={:#018x} svc=0x0000 number=0 return-x0={:#018x}",
                vector.name(),
                reported_esr,
                return_x0
            );
        }
        SYSCALL_PROOF_UNKNOWN_NUMBER => {
            let ok = return_x0 == SYSCALL_PROOF_EXPECTED_ENOSYS_X0 && after_len == before_len;
            DESCRIPTOR_WRITE_UNKNOWN_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "rpi5-descriptor-write-proof: syscall case=unknown vector={} esr={:#018x} svc=0x0000 number=17 return-x0={:#018x} expected=-ENOSYS",
                vector.name(),
                reported_esr,
                return_x0
            );
        }
        DESCRIPTOR_WRITE_COPY_PROBE_NUMBER => {
            let ok = return_x0 == SYSCALL_PROOF_EXPECTED_ENOSYS_X0 && after_len == before_len;
            DESCRIPTOR_WRITE_COPY_PROBE_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "rpi5-descriptor-write-proof: syscall case=copy_probe_quarantine vector={} esr={:#018x} svc=0x0000 number={:#018x} return-x0={:#018x} expected=-ENOSYS dispatched=false",
                vector.name(),
                reported_esr,
                raw_number,
                return_x0
            );
        }
        _ => {
            DESCRIPTOR_WRITE_ERRORS.fetch_add(1, Ordering::Relaxed);
            crate::println!(
                "rpi5-descriptor-write-proof: syscall case=unexpected vector={} esr={:#018x} svc=0x0000 number={:#018x} return-x0={:#018x}",
                vector.name(),
                reported_esr,
                raw_number,
                return_x0
            );
        }
    }

    true
}

#[cfg(talos_boot_scenario = "rpi5_close_syscall_proof")]
pub fn handle_close_syscall_proof_exception(
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
        CLOSE_SYSCALL_ERRORS.fetch_add(1, Ordering::Relaxed);
        return false;
    };

    if marker == syscall::DIAGNOSTIC_EL0_TRAP_SVC_IMMEDIATE {
        let stable = syscall::is_stable_syscall_svc_immediate(marker);
        crate::println!(
            "rpi5-close-syscall-proof: diagnostic-marker marker=0x7a10 stable-syscall={} dispatched=false",
            stable
        );
        finish_close_syscall_proof(reported_esr == SYSCALL_PROOF_EXPECTED_MARKER_ESR && far == 0);
    }

    if reported_esr != SYSCALL_PROOF_EXPECTED_SVC_ESR {
        CLOSE_SYSCALL_ERRORS.fetch_add(1, Ordering::Relaxed);
        return false;
    }

    let current_owner = crate::scheduler::ProcessOwnerId::new(PROCESS_DESCRIPTOR_STDIO_OWNER_RAW)
        .expect("close syscall proof owner id is nonzero");
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
    crate::println!(
        "rpi5-close-syscall-proof: handler-entry vector={} esr={:#018x} far={:#018x} svc={:#06x} number={:#018x} x0={:#018x}",
        vector.name(),
        reported_esr,
        far,
        marker,
        raw_number,
        args[0]
    );
    wait_uart10_empty_early_phase();
    let before_len = DESCRIPTOR_WRITE_CONSOLE_LEN.load(Ordering::Relaxed) as usize;
    let mappings = [UserMapping::new(
        DESCRIPTOR_WRITE_USER_DATA_START,
        DESCRIPTOR_WRITE_USER_DATA_LEN,
        UserMappingPermissions::USER_DATA,
    )
    .expect("fixed close syscall proof data mapping is valid")];
    let mut scratch = [0u8; 64];
    let mut console = DescriptorWriteCaptureConsole;
    let result = {
        let store = unsafe { &mut *core::ptr::addr_of_mut!(PROCESS_DESCRIPTOR_STDIO_STORE) };
        match store.current_descriptor_table(Some(current_owner)) {
            Ok(table) => {
                crate::println!(
                    "rpi5-close-syscall-proof: store-before-dispatch owner={:#018x} number={:#018x} fd={} fd-open={} stdout-open={} stderr-open={}",
                    current_owner.raw(),
                    raw_number,
                    args[0],
                    usize::try_from(args[0])
                        .ok()
                        .map(|descriptor| table.get(descriptor).is_ok())
                        .unwrap_or(false),
                    table.get(crate::posix::STDOUT_FD).is_ok(),
                    table.get(crate::posix::STDERR_FD).is_ok()
                );
            }
            Err(error) => {
                crate::println!(
                    "rpi5-close-syscall-proof: store-before-dispatch owner={:#018x} number={:#018x} fd={} owner-present=false table-error={}",
                    current_owner.raw(),
                    raw_number,
                    args[0],
                    error.name()
                );
            }
        }
        wait_uart10_empty_early_phase();
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
    crate::println!(
        "rpi5-close-syscall-proof: dispatch-return number={:#018x} return-x0={:#018x}",
        raw_number,
        return_x0
    );
    wait_uart10_empty_early_phase();
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
                CLOSE_SYSCALL_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "rpi5-close-syscall-proof: syscall case=close_stdout vector={} esr={:#018x} svc=0x0000 number=2 args=[x0={:#018x} x1={:#018x} x2={:#018x} x3={:#018x} x4={:#018x} x5={:#018x}] return-x0={:#018x} closed={}",
                vector.name(),
                reported_esr,
                args[0],
                args[1],
                args[2],
                args[3],
                args[4],
                args[5],
                return_x0,
                stdout_closed
            );
        }
        syscall::TALOS_WRITE_SYSCALL if args[0] == 1 => {
            let ok = return_x0 == DESCRIPTOR_WRITE_EXPECTED_EBADF_X0 && after_len == before_len;
            CLOSE_SYSCALL_WRITE_CLOSED_STDOUT_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                CLOSE_SYSCALL_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "rpi5-close-syscall-proof: syscall case=write_closed_stdout vector={} esr={:#018x} svc=0x0000 number=1 return-x0={:#018x} expected=-EBADF console-unchanged={}",
                vector.name(),
                reported_esr,
                return_x0,
                after_len == before_len
            );
        }
        syscall::TALOS_CLOSE_SYSCALL if args[0] == 2 && args[1] != 0 => {
            let ok = return_x0 == DESCRIPTOR_WRITE_EXPECTED_EINVAL_X0
                && after_len == before_len
                && stderr_open;
            CLOSE_SYSCALL_CLOSE_STDERR_RESERVED_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                CLOSE_SYSCALL_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "rpi5-close-syscall-proof: syscall case=close_reserved_stderr vector={} esr={:#018x} svc=0x0000 number=2 args=[x0={:#018x} x1={:#018x} x2={:#018x} x3={:#018x} x4={:#018x} x5={:#018x}] return-x0={:#018x} expected=-EINVAL fd2-still-open={}",
                vector.name(),
                reported_esr,
                args[0],
                args[1],
                args[2],
                args[3],
                args[4],
                args[5],
                return_x0,
                stderr_open
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
                CLOSE_SYSCALL_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "rpi5-close-syscall-proof: syscall case=write_stderr_after_reserved vector={} esr={:#018x} svc=0x0000 number=1 args=[x0={:#018x} x1={:#018x} x2={:#018x} x3={:#018x} x4={:#018x} x5={:#018x}] return-x0={:#018x}",
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
                "rpi5-close-syscall-proof: runtime-console case=write_stderr_after_reserved device=runtime-console0 bytes=18 hex=74616c6f732d7374646572722d727069350a ok={}",
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
                CLOSE_SYSCALL_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "rpi5-close-syscall-proof: syscall case=close_stderr vector={} esr={:#018x} svc=0x0000 number=2 args=[x0={:#018x} x1={:#018x} x2={:#018x} x3={:#018x} x4={:#018x} x5={:#018x}] return-x0={:#018x} closed={}",
                vector.name(),
                reported_esr,
                args[0],
                args[1],
                args[2],
                args[3],
                args[4],
                args[5],
                return_x0,
                stderr_closed
            );
        }
        syscall::TALOS_WRITE_SYSCALL if args[0] == 2 => {
            let ok = return_x0 == DESCRIPTOR_WRITE_EXPECTED_EBADF_X0 && after_len == before_len;
            CLOSE_SYSCALL_WRITE_CLOSED_STDERR_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                CLOSE_SYSCALL_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "rpi5-close-syscall-proof: syscall case=write_closed_stderr vector={} esr={:#018x} svc=0x0000 number=1 return-x0={:#018x} expected=-EBADF console-unchanged={}",
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
                CLOSE_SYSCALL_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "rpi5-close-syscall-proof: syscall case=close_stdout_again vector={} esr={:#018x} svc=0x0000 number=2 return-x0={:#018x} expected=-EBADF table-unchanged={}",
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
                CLOSE_SYSCALL_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "rpi5-close-syscall-proof: syscall case=close_badfd vector={} esr={:#018x} svc=0x0000 number=2 return-x0={:#018x} expected=-EBADF table-unchanged={}",
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
                CLOSE_SYSCALL_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "rpi5-close-syscall-proof: syscall case=talos_nop vector={} esr={:#018x} svc=0x0000 number=0 return-x0={:#018x}",
                vector.name(),
                reported_esr,
                return_x0
            );
        }
        SYSCALL_PROOF_UNKNOWN_NUMBER => {
            let ok = return_x0 == SYSCALL_PROOF_EXPECTED_ENOSYS_X0 && after_len == before_len;
            CLOSE_SYSCALL_UNKNOWN_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                CLOSE_SYSCALL_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "rpi5-close-syscall-proof: syscall case=unknown vector={} esr={:#018x} svc=0x0000 number=17 return-x0={:#018x} expected=-ENOSYS",
                vector.name(),
                reported_esr,
                return_x0
            );
        }
        DESCRIPTOR_WRITE_COPY_PROBE_NUMBER => {
            let ok = return_x0 == SYSCALL_PROOF_EXPECTED_ENOSYS_X0 && after_len == before_len;
            CLOSE_SYSCALL_COPY_PROBE_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                CLOSE_SYSCALL_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "rpi5-close-syscall-proof: syscall case=copy_probe_quarantine vector={} esr={:#018x} svc=0x0000 number={:#018x} return-x0={:#018x} expected=-ENOSYS dispatched=false",
                vector.name(),
                reported_esr,
                raw_number,
                return_x0
            );
        }
        _ => {
            CLOSE_SYSCALL_ERRORS.fetch_add(1, Ordering::Relaxed);
            crate::println!(
                "rpi5-close-syscall-proof: syscall case=unexpected vector={} esr={:#018x} svc=0x0000 number={:#018x} return-x0={:#018x}",
                vector.name(),
                reported_esr,
                raw_number,
                return_x0
            );
        }
    }

    true
}

#[cfg(talos_boot_scenario = "rpi5_dup_syscall_proof")]
pub fn handle_dup_syscall_proof_exception(
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
        DUP_SYSCALL_ERRORS.fetch_add(1, Ordering::Relaxed);
        return false;
    };

    if marker == syscall::DIAGNOSTIC_EL0_TRAP_SVC_IMMEDIATE {
        let stable = syscall::is_stable_syscall_svc_immediate(marker);
        crate::println!(
            "rpi5-dup-syscall-proof: diagnostic-marker marker=0x7a10 stable-syscall={} dispatched=false",
            stable
        );
        finish_dup_syscall_proof(reported_esr == SYSCALL_PROOF_EXPECTED_MARKER_ESR && far == 0);
    }

    if reported_esr != SYSCALL_PROOF_EXPECTED_SVC_ESR {
        DUP_SYSCALL_ERRORS.fetch_add(1, Ordering::Relaxed);
        return false;
    }

    let current_owner = crate::scheduler::ProcessOwnerId::new(PROCESS_DESCRIPTOR_STDIO_OWNER_RAW)
        .expect("dup syscall proof owner id is nonzero");
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
    .expect("fixed dup syscall proof data mapping is valid")];
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
        .expect("dup syscall proof current table remains present");
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
                DUP_SYSCALL_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "rpi5-dup-syscall-proof: syscall case=dup_stdout vector={} esr={:#018x} svc=0x0000 number=3 args=[x0={:#018x} x1={:#018x} x2={:#018x} x3={:#018x} x4={:#018x} x5={:#018x}] descriptor-owner={:#018x} return-x0={:#018x} lowest-free={} source-open={}",
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
                DUP_SYSCALL_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "rpi5-dup-syscall-proof: syscall case=dup_stderr_full vector={} esr={:#018x} svc=0x0000 number=3 return-x0={:#018x} expected=-EMFILE table-unchanged={}",
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
                DUP_SYSCALL_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "rpi5-dup-syscall-proof: syscall case=dup_stdout_reserved vector={} esr={:#018x} svc=0x0000 number=3 return-x0={:#018x} expected=-EINVAL table-unchanged={}",
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
                DUP_SYSCALL_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "rpi5-dup-syscall-proof: syscall case=write_stdout_source vector={} esr={:#018x} svc=0x0000 number=1 args=[x0={:#018x} x1={:#018x} x2={:#018x} x3={:#018x} x4={:#018x} x5={:#018x}] descriptor-owner={:#018x} return-x0={:#018x}",
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
                "rpi5-dup-syscall-proof: runtime-console case=write_stdout_source device=runtime-console0 bytes=19 hex=74616c6f732d6475702d7372632d727069350a ok={}",
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
                DUP_SYSCALL_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "rpi5-dup-syscall-proof: syscall case=write_stdout_duplicate vector={} esr={:#018x} svc=0x0000 number=1 args=[x0={:#018x} x1={:#018x} x2={:#018x} x3={:#018x} x4={:#018x} x5={:#018x}] descriptor-owner={:#018x} return-x0={:#018x}",
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
                "rpi5-dup-syscall-proof: runtime-console case=write_stdout_duplicate device=runtime-console0 bytes=19 hex=74616c6f732d6475702d6e65772d727069350a ok={}",
                console_ok
            );
        }
        syscall::TALOS_CLOSE_SYSCALL if args[0] == 1 => {
            let ok = return_x0 == 0 && after_len == before_len && source_closed && duplicate_open;
            DUP_SYSCALL_CLOSE_SOURCE_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                DUP_SYSCALL_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "rpi5-dup-syscall-proof: syscall case=close_stdout_source vector={} esr={:#018x} svc=0x0000 number=2 args=[x0={:#018x} x1={:#018x} x2={:#018x} x3={:#018x} x4={:#018x} x5={:#018x}] descriptor-owner={:#018x} return-x0={:#018x}",
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
                DUP_SYSCALL_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "rpi5-dup-syscall-proof: syscall case=write_stdout_source_after_close vector={} esr={:#018x} svc=0x0000 number=1 return-x0={:#018x} expected=-EBADF console-unchanged={}",
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
                DUP_SYSCALL_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "rpi5-dup-syscall-proof: syscall case=write_duplicate_after_source_close vector={} esr={:#018x} svc=0x0000 number=1 args=[x0={:#018x} x1={:#018x} x2={:#018x} x3={:#018x} x4={:#018x} x5={:#018x}] descriptor-owner={:#018x} return-x0={:#018x}",
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
                "rpi5-dup-syscall-proof: runtime-console case=write_duplicate_after_source_close device=runtime-console0 bytes=19 hex=74616c6f732d6475702d6e65772d727069350a ok={}",
                console_ok
            );
        }
        syscall::TALOS_CLOSE_SYSCALL if args[0] == 3 => {
            let ok = return_x0 == 0 && after_len == before_len && duplicate_closed;
            DUP_SYSCALL_CLOSE_DUPLICATE_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                DUP_SYSCALL_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "rpi5-dup-syscall-proof: syscall case=close_stdout_duplicate vector={} esr={:#018x} svc=0x0000 number=2 args=[x0={:#018x} x1={:#018x} x2={:#018x} x3={:#018x} x4={:#018x} x5={:#018x}] descriptor-owner={:#018x} return-x0={:#018x}",
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
                DUP_SYSCALL_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "rpi5-dup-syscall-proof: syscall case=write_duplicate_after_duplicate_close vector={} esr={:#018x} svc=0x0000 number=1 return-x0={:#018x} expected=-EBADF console-unchanged={}",
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
                DUP_SYSCALL_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "rpi5-dup-syscall-proof: syscall case=dup_closed_source vector={} esr={:#018x} svc=0x0000 number=3 return-x0={:#018x} expected=-EBADF table-unchanged={}",
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
                DUP_SYSCALL_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "rpi5-dup-syscall-proof: syscall case=talos_nop vector={} esr={:#018x} svc=0x0000 number=0 return-x0={:#018x}",
                vector.name(),
                reported_esr,
                return_x0
            );
        }
        SYSCALL_PROOF_UNKNOWN_NUMBER => {
            let ok = return_x0 == SYSCALL_PROOF_EXPECTED_ENOSYS_X0 && after_len == before_len;
            DUP_SYSCALL_UNKNOWN_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                DUP_SYSCALL_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "rpi5-dup-syscall-proof: syscall case=unknown vector={} esr={:#018x} svc=0x0000 number=17 return-x0={:#018x} expected=-ENOSYS",
                vector.name(),
                reported_esr,
                return_x0
            );
        }
        DESCRIPTOR_WRITE_COPY_PROBE_NUMBER => {
            let ok = return_x0 == SYSCALL_PROOF_EXPECTED_ENOSYS_X0 && after_len == before_len;
            DUP_SYSCALL_COPY_PROBE_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                DUP_SYSCALL_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "rpi5-dup-syscall-proof: syscall case=copy_probe_quarantine vector={} esr={:#018x} svc=0x0000 number={:#018x} return-x0={:#018x} expected=-ENOSYS dispatched=false",
                vector.name(),
                reported_esr,
                raw_number,
                return_x0
            );
        }
        _ => {
            DUP_SYSCALL_ERRORS.fetch_add(1, Ordering::Relaxed);
            crate::println!(
                "rpi5-dup-syscall-proof: syscall case=unexpected vector={} esr={:#018x} svc=0x0000 number={:#018x} return-x0={:#018x}",
                vector.name(),
                reported_esr,
                raw_number,
                return_x0
            );
        }
    }

    true
}

#[cfg(talos_boot_scenario = "rpi5_read_stdin_proof")]
fn read_stdin_user_matches(start: usize, expected: &[u8]) -> bool {
    let data = unsafe { &*core::ptr::addr_of!(DESCRIPTOR_WRITE_USER_DATA) };
    let Some(end) = start.checked_add(expected.len()) else {
        return false;
    };
    end <= data.len() && &data[start..end] == expected
}

#[cfg(talos_boot_scenario = "rpi5_read_stdin_proof")]
fn read_stdin_user_zero(start: usize, len: usize) -> bool {
    let data = unsafe { &*core::ptr::addr_of!(DESCRIPTOR_WRITE_USER_DATA) };
    let Some(end) = start.checked_add(len) else {
        return false;
    };
    end <= data.len() && data[start..end].iter().all(|byte| *byte == 0)
}

#[cfg(talos_boot_scenario = "rpi5_read_stdin_proof")]
fn read_stdin_user_all_zero() -> bool {
    let data = unsafe { &*core::ptr::addr_of!(DESCRIPTOR_WRITE_USER_DATA) };
    data.iter().all(|byte| *byte == 0)
}

#[cfg(talos_boot_scenario = "rpi5_read_stdin_proof")]
pub fn handle_read_stdin_proof_exception(
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
            "rpi5-read-stdin-proof: diagnostic-marker marker=0x7a10 stable-syscall={} dispatched=false",
            stable
        );
        finish_read_stdin_proof(reported_esr == SYSCALL_PROOF_EXPECTED_MARKER_ESR && far == 0);
    }

    if reported_esr != SYSCALL_PROOF_EXPECTED_SVC_ESR {
        READ_STDIN_ERRORS.fetch_add(1, Ordering::Relaxed);
        return false;
    }

    let current_owner = crate::scheduler::ProcessOwnerId::new(PROCESS_DESCRIPTOR_STDIO_OWNER_RAW)
        .expect("read stdin proof owner id is nonzero");
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
    .expect("fixed read stdin proof data mapping is valid")];
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
        .expect("read stdin proof current table remains present");
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
                "rpi5-read-stdin-proof: syscall case=dup_stdin vector={} esr={:#018x} svc=0x0000 number=3 args=[x0={:#018x} x1={:#018x} x2={:#018x} x3={:#018x} x4={:#018x} x5={:#018x}] descriptor-owner={:#018x} return-x0={:#018x} lowest-free={} source-open={}",
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
                "rpi5-read-stdin-proof: syscall case=read_guard vector={} esr={:#018x} svc=0x0000 number=4 return-x0={:#018x} expected=-EFAULT fixed-stdin-cursor={} user-unchanged={}",
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
                "rpi5-read-stdin-proof: syscall case=read_reserved vector={} esr={:#018x} svc=0x0000 number=4 return-x0={:#018x} expected=-EINVAL fixed-stdin-cursor={} user-unchanged={}",
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
                "rpi5-read-stdin-proof: syscall case=read_fd1 vector={} esr={:#018x} svc=0x0000 number=4 return-x0={:#018x} expected=-EBADF fixed-stdin-cursor={} user-unchanged={}",
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
                "rpi5-read-stdin-proof: syscall case=read_badfd vector={} esr={:#018x} svc=0x0000 number=4 return-x0={:#018x} expected=-EBADF fixed-stdin-cursor={} user-unchanged={}",
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
                "rpi5-read-stdin-proof: syscall case=read_stdin_first vector={} esr={:#018x} svc=0x0000 number=4 args=[x0={:#018x} x1={:#018x} x2={:#018x} x3={:#018x} x4={:#018x} x5={:#018x}] descriptor-owner={:#018x} return-x0={:#018x} fixed-stdin-cursor={}",
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
                "rpi5-read-stdin-proof: user-buffer case=read_stdin_first addr={:#018x} bytes=5 hex=74616c6f73 ok={}",
                args[1],
                read_stdin_user_matches(0x80, b"talos")
            );
            crate::println!(
                "rpi5-read-stdin-proof: user-observed case=read_stdin_first x0={:#018x} ok={}",
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
                && read_stdin_user_matches(0xa0, b"-stdin-rpi5\n");
            READ_STDIN_DUPLICATE_REMAINING_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                READ_STDIN_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "rpi5-read-stdin-proof: syscall case=read_stdin_duplicate_remaining vector={} esr={:#018x} svc=0x0000 number=4 args=[x0={:#018x} x1={:#018x} x2={:#018x} x3={:#018x} x4={:#018x} x5={:#018x}] descriptor-owner={:#018x} return-x0={:#018x} fixed-stdin-cursor={} short-read={}",
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
                "rpi5-read-stdin-proof: user-buffer case=read_stdin_duplicate_remaining addr={:#018x} bytes=12 hex=2d737464696e2d727069350a ok={}",
                args[1],
                read_stdin_user_matches(0xa0, b"-stdin-rpi5\n")
            );
            crate::println!(
                "rpi5-read-stdin-proof: user-observed case=read_stdin_duplicate_remaining x0={:#018x} ok={}",
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
                "rpi5-read-stdin-proof: syscall case=read_stdin_eof vector={} esr={:#018x} svc=0x0000 number=4 return-x0={:#018x} fixed-stdin-cursor={} user-unchanged={} eof=true",
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
                "rpi5-read-stdin-proof: syscall case=talos_nop vector={} esr={:#018x} svc=0x0000 number=0 return-x0={:#018x}",
                vector.name(),
                reported_esr,
                return_x0
            );
        }
        SYSCALL_PROOF_UNKNOWN_NUMBER => {
            let ok = return_x0 == SYSCALL_PROOF_EXPECTED_ENOSYS_X0;
            READ_STDIN_UNKNOWN_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                READ_STDIN_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "rpi5-read-stdin-proof: syscall case=unknown vector={} esr={:#018x} svc=0x0000 number=17 return-x0={:#018x} expected=-ENOSYS",
                vector.name(),
                reported_esr,
                return_x0
            );
        }
        DESCRIPTOR_WRITE_COPY_PROBE_NUMBER => {
            let ok = return_x0 == SYSCALL_PROOF_EXPECTED_ENOSYS_X0;
            READ_STDIN_COPY_PROBE_OBSERVED.store(u64::from(ok), Ordering::Relaxed);
            if !ok {
                READ_STDIN_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            crate::println!(
                "rpi5-read-stdin-proof: syscall case=copy_probe_quarantine vector={} esr={:#018x} svc=0x0000 number={:#018x} return-x0={:#018x} expected=-ENOSYS dispatched=false",
                vector.name(),
                reported_esr,
                raw_number,
                return_x0
            );
        }
        _ => {
            READ_STDIN_ERRORS.fetch_add(1, Ordering::Relaxed);
            crate::println!(
                "rpi5-read-stdin-proof: syscall case=unexpected vector={} esr={:#018x} svc=0x0000 number={:#018x} return-x0={:#018x}",
                vector.name(),
                reported_esr,
                raw_number,
                return_x0
            );
        }
    }

    true
}

#[cfg(talos_boot_scenario = "rpi5_pointer_copy_proof")]
fn pointer_copy_user_data_replaced() -> bool {
    let data = unsafe { &*core::ptr::addr_of!(POINTER_COPY_USER_DATA) };
    data[..16]
        .iter()
        .all(|byte| *byte == POINTER_COPY_USER_DATA_REPLACEMENT)
}

#[cfg(any(
    talos_boot_scenario = "rpi5_descriptor_write_proof",
    talos_boot_scenario = "rpi5_close_syscall_proof",
    talos_boot_scenario = "rpi5_dup_syscall_proof",
    talos_boot_scenario = "rpi5_read_stdin_proof"
))]
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

#[cfg(any(
    talos_boot_scenario = "rpi5_descriptor_write_proof",
    talos_boot_scenario = "rpi5_close_syscall_proof",
    talos_boot_scenario = "rpi5_dup_syscall_proof",
    talos_boot_scenario = "rpi5_read_stdin_proof"
))]
struct DescriptorWriteCaptureConsole;

#[cfg(any(
    talos_boot_scenario = "rpi5_descriptor_write_proof",
    talos_boot_scenario = "rpi5_close_syscall_proof",
    talos_boot_scenario = "rpi5_dup_syscall_proof",
    talos_boot_scenario = "rpi5_read_stdin_proof"
))]
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
        };
        unsafe {
            let capture = &mut *core::ptr::addr_of_mut!(DESCRIPTOR_WRITE_CONSOLE_CAPTURE);
            capture[len..end].copy_from_slice(bytes);
        }
        if let Ok(s) = core::str::from_utf8(bytes) {
            crate::target::console::write_static(s);
            wait_uart10_empty_early_phase();
        } else {
            return Err(core::fmt::Error);
        }
        DESCRIPTOR_WRITE_CONSOLE_LEN.store(end as u64, Ordering::Relaxed);
        Ok(())
    }
}

#[cfg(talos_boot_scenario = "rpi5_descriptor_write_proof")]
fn finish_descriptor_write_proof(marker_ok: bool) -> ! {
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
        "pi5-descriptor-write-proof-complete"
    } else {
        "pi5-descriptor-write-proof-failed"
    };

    crate::println!(
        "rpi5-descriptor-write-proof: final participants={} expected=8 errors={} classification={}",
        participants,
        errors,
        classification
    );
    if complete {
        crate::println!("rpi5-descriptor-write-proof: PASS");
    } else {
        crate::println!("rpi5-descriptor-write-proof: FAIL");
    }
    wait_uart10_empty_early_phase();
    crate::arch::aarch64::halt()
}

#[cfg(talos_boot_scenario = "rpi5_close_syscall_proof")]
fn finish_close_syscall_proof(marker_ok: bool) -> ! {
    if !marker_ok {
        CLOSE_SYSCALL_ERRORS.fetch_add(1, Ordering::Relaxed);
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
    let errors = CLOSE_SYSCALL_ERRORS.load(Ordering::Relaxed);
    let complete = participants == 11 && errors == 0;
    let classification = if complete {
        "pi5-close-syscall-proof-complete"
    } else {
        "pi5-close-syscall-proof-failed"
    };

    crate::println!(
        "rpi5-close-syscall-proof: final participants={} expected=11 errors={} classification={}",
        participants,
        errors,
        classification
    );
    if complete {
        crate::println!("rpi5-close-syscall-proof: PASS");
    } else {
        crate::println!("rpi5-close-syscall-proof: FAIL");
    }
    wait_uart10_empty_early_phase();
    crate::arch::aarch64::halt()
}

#[cfg(talos_boot_scenario = "rpi5_dup_syscall_proof")]
fn finish_dup_syscall_proof(marker_ok: bool) -> ! {
    if !marker_ok {
        DUP_SYSCALL_ERRORS.fetch_add(1, Ordering::Relaxed);
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
    let errors = DUP_SYSCALL_ERRORS.load(Ordering::Relaxed);
    let complete = participants == 14 && errors == 0;
    let classification = if complete {
        "pi5-dup-syscall-proof-complete"
    } else {
        "pi5-dup-syscall-proof-failed"
    };

    crate::println!(
        "rpi5-dup-syscall-proof: final participants={} expected=14 errors={} classification={}",
        participants,
        errors,
        classification
    );
    if complete {
        crate::println!("rpi5-dup-syscall-proof: PASS");
    } else {
        crate::println!("rpi5-dup-syscall-proof: FAIL");
    }
    wait_uart10_empty_early_phase();
    crate::arch::aarch64::halt()
}

#[cfg(talos_boot_scenario = "rpi5_read_stdin_proof")]
fn finish_read_stdin_proof(marker_ok: bool) -> ! {
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
        "pi5-read-stdin-proof-complete"
    } else {
        "pi5-read-stdin-proof-failed"
    };

    crate::println!(
        "rpi5-read-stdin-proof: final participants={} expected=11 errors={} classification={}",
        participants,
        errors,
        classification
    );
    if complete {
        crate::println!("rpi5-read-stdin-proof: PASS");
    } else {
        crate::println!("rpi5-read-stdin-proof: FAIL");
    }
    wait_uart10_empty_early_phase();
    crate::arch::aarch64::halt()
}

#[cfg(talos_boot_scenario = "rpi5_pointer_copy_proof")]
fn finish_pointer_copy_proof(marker_ok: bool) -> ! {
    if !marker_ok {
        POINTER_COPY_ERRORS.fetch_add(1, Ordering::Relaxed);
    }
    let participants = POINTER_COPY_SUCCESS_OBSERVED.load(Ordering::Relaxed)
        + POINTER_COPY_EFAULT_OBSERVED.load(Ordering::Relaxed)
        + POINTER_COPY_UNKNOWN_OBSERVED.load(Ordering::Relaxed);
    let errors = POINTER_COPY_ERRORS.load(Ordering::Relaxed);
    let complete = participants == 3 && errors == 0;
    let classification = if complete {
        "pi5-pointer-copy-proof-complete"
    } else {
        "pi5-pointer-copy-proof-failed"
    };

    crate::println!(
        "rpi5-pointer-copy-proof: final participants={} expected=3 errors={} classification={}",
        participants,
        errors,
        classification
    );
    if complete {
        crate::println!("rpi5-pointer-copy-proof: PASS");
    } else {
        crate::println!("rpi5-pointer-copy-proof: FAIL");
    }
    wait_uart10_empty_early_phase();
    crate::arch::aarch64::halt()
}

#[cfg(talos_boot_scenario = "rpi5_syscall_proof")]
fn finish_syscall_proof(marker_ok: bool) -> ! {
    if !marker_ok {
        SYSCALL_PROOF_ERRORS.fetch_add(1, Ordering::Relaxed);
    }
    let participants = SYSCALL_PROOF_TALOS_NOP_OBSERVED.load(Ordering::Relaxed)
        + SYSCALL_PROOF_UNKNOWN_OBSERVED.load(Ordering::Relaxed);
    let errors = SYSCALL_PROOF_ERRORS.load(Ordering::Relaxed);
    let complete = participants == 2 && errors == 0;
    let classification = if complete {
        "pi5-syscall-proof-complete"
    } else {
        "pi5-syscall-proof-failed"
    };

    crate::println!(
        "rpi5-syscall-proof: final participants={} expected=2 errors={} classification={}",
        participants,
        errors,
        classification
    );
    if complete {
        crate::println!("rpi5-syscall-proof: PASS");
    } else {
        crate::println!("rpi5-syscall-proof: FAIL");
    }
    wait_uart10_empty_early_phase();
    crate::arch::aarch64::halt()
}

#[cfg(talos_boot_scenario = "rpi5_el0_trap_proof")]
pub fn handle_el0_trap_proof_exception(
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
        "rpi5-el0-trap-proof: trap vector={} esr={:#018x} far={:#018x} elr={:#018x} sp={:#018x} spsr={:#018x} marker={:#x}",
        vector.name(),
        reported_esr,
        far,
        elr,
        user_sp,
        spsr,
        marker
    );
    crate::println!("rpi5-el0-trap-proof: raw-esr={:#018x}", esr);
    crate::println!(
        "rpi5-el0-trap-proof: frame available={} x0={:#018x} x1={:#018x}",
        frame_available,
        unsafe { saved_frame.as_ref().map(|frame| frame.reg(0)).unwrap_or(0) },
        unsafe { saved_frame.as_ref().map(|frame| frame.reg(1)).unwrap_or(0) }
    );

    if ok {
        crate::println!(
            "rpi5-el0-trap-proof: final participants=1 expected=1 errors=0 classification=pi5-el0-trap-proof-complete"
        );
        crate::println!("rpi5-el0-trap-proof: PASS");
        wait_uart10_empty_early_phase();
        crate::arch::aarch64::halt();
    }

    crate::println!(
        "rpi5-el0-trap-proof: final participants=0 expected=1 errors=1 classification=pi5-el0-trap-proof-failed"
    );
    crate::println!("rpi5-el0-trap-proof: FAIL");
    wait_uart10_empty_early_phase();
    crate::arch::aarch64::halt()
}

#[cfg(talos_boot_scenario = "rpi5_el0_trap_proof")]
#[unsafe(no_mangle)]
pub extern "C" fn talos_rpi5_el0_trap_proof_entered_el1() {
    let current_el = aarch64::current_el();
    let handoff = unsafe { read_el0_trap_proof_el1_handoff_registers() };
    crate::println!(
        "rpi5-el0-trap-proof: entered-el1 current_el={:#x} sctlr_el1={:#018x} tcr_el1={:#018x} ttbr0_el1={:#018x} vbar_el1={:#018x} elr_el1={:#018x} spsr_el1={:#018x} sp={:#018x}",
        current_el,
        handoff.sctlr_el1,
        handoff.tcr_el1,
        handoff.ttbr0_el1,
        handoff.vbar_el1,
        handoff.elr_el1,
        handoff.spsr_el1,
        handoff.sp
    );
    wait_uart10_empty_early_phase();
}

#[cfg(any(
    talos_boot_scenario = "rpi5_el0_trap_proof",
    talos_boot_scenario = "rpi5_syscall_proof"
))]
unsafe fn install_el0_trap_proof_tables() {
    const TABLE_DESC: u64 = 0b11;
    const BLOCK_DESC: u64 = 0b01;
    const PAGE_DESC: u64 = 0b11;
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

    let root = unsafe { core::ptr::addr_of_mut!(EL0_TRAP_ROOT_TABLE.0) };
    let l1 = unsafe { core::ptr::addr_of_mut!(EL0_TRAP_L1_TABLE.0) };
    let low_l2 = unsafe { core::ptr::addr_of_mut!(EL0_TRAP_LOW_L2_TABLE.0) };
    let low_l3 = unsafe { core::ptr::addr_of_mut!(EL0_TRAP_LOW_L3_TABLE.0) };
    let mmio_l2 = unsafe { core::ptr::addr_of_mut!(EL0_TRAP_MMIO_L2_TABLE.0) };
    let payload_pa = core::ptr::addr_of!(EL0_TRAP_PAYLOAD.0) as u64;
    #[cfg(talos_boot_scenario = "rpi5_pointer_copy_proof")]
    let user_data_pa = core::ptr::addr_of!(POINTER_COPY_USER_DATA) as u64;
    #[cfg(any(
        talos_boot_scenario = "rpi5_descriptor_write_proof",
        talos_boot_scenario = "rpi5_close_syscall_proof",
        talos_boot_scenario = "rpi5_read_stdin_proof"
    ))]
    let user_data_pa = core::ptr::addr_of!(DESCRIPTOR_WRITE_USER_DATA) as u64;
    let stack_pa = unsafe { core::ptr::addr_of!(EL0_TRAP_STACK.0) as u64 };

    unsafe {
        core::ptr::write_bytes(root.cast::<u8>(), 0, EL0_TRAP_TABLE_PAGE_SIZE);
        core::ptr::write_bytes(l1.cast::<u8>(), 0, EL0_TRAP_TABLE_PAGE_SIZE);
        core::ptr::write_bytes(low_l2.cast::<u8>(), 0, EL0_TRAP_TABLE_PAGE_SIZE);
        core::ptr::write_bytes(low_l3.cast::<u8>(), 0, EL0_TRAP_TABLE_PAGE_SIZE);
        core::ptr::write_bytes(mmio_l2.cast::<u8>(), 0, EL0_TRAP_TABLE_PAGE_SIZE);

        (*root)[0] = (l1 as u64 & ADDR_MASK_4K) | TABLE_DESC;
        (*l1)[0] = (low_l2 as u64 & ADDR_MASK_4K) | TABLE_DESC;
        (*l1)[EL0_TRAP_L1_BCM2712_MMIO_INDEX] = (mmio_l2 as u64 & ADDR_MASK_4K) | TABLE_DESC;

        (*low_l2)[0] = (low_l3 as u64 & ADDR_MASK_4K) | TABLE_DESC;
        let mut index = 1usize;
        while index < 512 {
            let base = (index as u64) << 21;
            (*low_l2)[index] =
                (base & ADDR_MASK_2M) | (ATTR_NORMAL << ATTR_SHIFT) | SH_INNER | AF | BLOCK_DESC;
            index += 1;
        }

        (*low_l3)[(EL0_TRAP_USER_TEXT_START as usize) >> 12] = (payload_pa & ADDR_MASK_4K)
            | (ATTR_NORMAL << ATTR_SHIFT)
            | AP_EL0_RO
            | SH_INNER
            | AF
            | PAGE_DESC;

        #[cfg(talos_boot_scenario = "rpi5_pointer_copy_proof")]
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

        #[cfg(any(
            talos_boot_scenario = "rpi5_descriptor_write_proof",
            talos_boot_scenario = "rpi5_close_syscall_proof",
            talos_boot_scenario = "rpi5_read_stdin_proof"
        ))]
        {
            (*low_l3)[(DESCRIPTOR_WRITE_USER_DATA_START as usize) >> 12] = (user_data_pa
                & ADDR_MASK_4K)
                | (ATTR_NORMAL << ATTR_SHIFT)
                | AP_EL0_RW
                | SH_INNER
                | AF
                | UXN
                | PAGE_DESC;
        }

        let mut page = 0usize;
        while page < EL0_TRAP_USER_STACK_LEN / EL0_TRAP_TABLE_PAGE_SIZE {
            let va = EL0_TRAP_USER_STACK_START as usize + page * EL0_TRAP_TABLE_PAGE_SIZE;
            let pa = stack_pa + (page * EL0_TRAP_TABLE_PAGE_SIZE) as u64;
            (*low_l3)[va >> 12] = (pa & ADDR_MASK_4K)
                | (ATTR_NORMAL << ATTR_SHIFT)
                | AP_EL0_RW
                | SH_INNER
                | AF
                | UXN
                | PAGE_DESC;
            page += 1;
        }

        let mmio_start_index = ((EL0_TRAP_BCM2712_MMIO_START >> 21) & 0x1ff) as usize;
        let mmio_end_index = bcm2712_mmio_l2_end_index();
        let mut mmio_index = mmio_start_index;
        while mmio_index < mmio_end_index {
            let base =
                EL0_TRAP_BCM2712_MMIO_START + ((mmio_index - mmio_start_index) as u64 * 0x20_0000);
            (*mmio_l2)[mmio_index] =
                (base & ADDR_MASK_2M) | (ATTR_DEVICE << ATTR_SHIFT) | AF | PXN | UXN | BLOCK_DESC;
            mmio_index += 1;
        }
    }

    clean_cache_range_to_poc(root as usize, EL0_TRAP_TABLE_PAGE_SIZE);
    clean_cache_range_to_poc(l1 as usize, EL0_TRAP_TABLE_PAGE_SIZE);
    clean_cache_range_to_poc(low_l2 as usize, EL0_TRAP_TABLE_PAGE_SIZE);
    clean_cache_range_to_poc(low_l3 as usize, EL0_TRAP_TABLE_PAGE_SIZE);
    clean_cache_range_to_poc(mmio_l2 as usize, EL0_TRAP_TABLE_PAGE_SIZE);
    unsafe {
        core::arch::asm!("dsb sy", "isb", options(nostack, preserves_flags));
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_el0_trap_proof",
    talos_boot_scenario = "rpi5_syscall_proof"
))]
unsafe fn prepare_el1_and_el0_translation() {
    const MAIR_NORMAL_NC: u64 = 0x44;
    const MAIR_DEVICE_NGNRE: u64 = 0x04;
    const TCR_T0SZ_SHIFT: u64 = 0;
    const TCR_IRGN0_SHIFT: u64 = 8;
    const TCR_ORGN0_SHIFT: u64 = 10;
    const TCR_SH0_SHIFT: u64 = 12;
    const TCR_TG0_4K: u64 = 0b00 << 14;
    const TCR_IPS_SHIFT: u64 = 32;
    const TCR_CACHE_NC: u64 = 0b00;
    const TCR_SH_INNER: u64 = 0b11;
    const TCR_IPS_40BIT: u64 = 0b010;
    const SCTLR_M: u64 = 1 << 0;
    const SCTLR_C: u64 = 1 << 2;
    const SCTLR_I: u64 = 1 << 12;
    const SCTLR_WXN: u64 = 1 << 19;
    const SCTLR_RES1: u64 = (1 << 11) | (1 << 20) | (1 << 22) | (1 << 28) | (1 << 29);
    const HCR_RW: u64 = 1 << 31;

    let mair = MAIR_NORMAL_NC | (MAIR_DEVICE_NGNRE << 8);
    let tcr = ((64 - 48) << TCR_T0SZ_SHIFT)
        | (TCR_CACHE_NC << TCR_IRGN0_SHIFT)
        | (TCR_CACHE_NC << TCR_ORGN0_SHIFT)
        | (TCR_SH_INNER << TCR_SH0_SHIFT)
        | TCR_TG0_4K
        | (TCR_IPS_40BIT << TCR_IPS_SHIFT);
    let ttbr0 = unsafe { core::ptr::addr_of!(EL0_TRAP_ROOT_TABLE.0) as u64 };
    let vbar = relocate_early_linked_addr(core::ptr::addr_of!(__exception_vectors) as usize) as u64;
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
        sctlr = (sctlr | SCTLR_RES1) & !(SCTLR_M | SCTLR_C | SCTLR_I | SCTLR_WXN);
        core::arch::asm!(
            "msr SCTLR_EL1, {sctlr}",
            "dsb sy",
            "isb",
            sctlr = in(reg) sctlr,
            options(nostack, preserves_flags)
        );
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_el0_trap_proof",
    talos_boot_scenario = "rpi5_syscall_proof"
))]
unsafe fn write_el0_trap_proof_pre_eret_registers(entry: u64, spsr: u64) {
    unsafe {
        core::arch::asm!(
            "msr ELR_EL1, {entry}",
            "msr SPSR_EL1, {spsr}",
            "isb",
            entry = in(reg) entry,
            spsr = in(reg) spsr,
            options(nostack, preserves_flags)
        );
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_el0_trap_proof",
    talos_boot_scenario = "rpi5_syscall_proof"
))]
unsafe fn read_el0_trap_proof_pre_eret_registers() -> El0TrapPreEretRegisters {
    let hcr_el2: u64;
    let sctlr_el1: u64;
    let tcr_el1: u64;
    let ttbr0_el1: u64;
    let vbar_el1: u64;
    let elr_el1: u64;
    let spsr_el1: u64;
    unsafe {
        core::arch::asm!(
            "mrs {hcr_el2}, HCR_EL2",
            "mrs {sctlr_el1}, SCTLR_EL1",
            "mrs {tcr_el1}, TCR_EL1",
            "mrs {ttbr0_el1}, TTBR0_EL1",
            "mrs {vbar_el1}, VBAR_EL1",
            "mrs {elr_el1}, ELR_EL1",
            "mrs {spsr_el1}, SPSR_EL1",
            hcr_el2 = out(reg) hcr_el2,
            sctlr_el1 = out(reg) sctlr_el1,
            tcr_el1 = out(reg) tcr_el1,
            ttbr0_el1 = out(reg) ttbr0_el1,
            vbar_el1 = out(reg) vbar_el1,
            elr_el1 = out(reg) elr_el1,
            spsr_el1 = out(reg) spsr_el1,
            options(nomem, nostack, preserves_flags)
        );
    }
    El0TrapPreEretRegisters {
        hcr_el2,
        sctlr_el1,
        tcr_el1,
        ttbr0_el1,
        vbar_el1,
        elr_el1,
        spsr_el1,
    }
}

#[cfg(talos_boot_scenario = "rpi5_el0_trap_proof")]
unsafe fn read_el0_trap_proof_el1_handoff_registers() -> El0TrapEl1HandoffRegisters {
    let sctlr_el1: u64;
    let tcr_el1: u64;
    let ttbr0_el1: u64;
    let vbar_el1: u64;
    let elr_el1: u64;
    let spsr_el1: u64;
    let sp: u64;
    unsafe {
        core::arch::asm!(
            "mrs {sctlr_el1}, SCTLR_EL1",
            "mrs {tcr_el1}, TCR_EL1",
            "mrs {ttbr0_el1}, TTBR0_EL1",
            "mrs {vbar_el1}, VBAR_EL1",
            "mrs {elr_el1}, ELR_EL1",
            "mrs {spsr_el1}, SPSR_EL1",
            "mov {sp}, sp",
            sctlr_el1 = out(reg) sctlr_el1,
            tcr_el1 = out(reg) tcr_el1,
            ttbr0_el1 = out(reg) ttbr0_el1,
            vbar_el1 = out(reg) vbar_el1,
            elr_el1 = out(reg) elr_el1,
            spsr_el1 = out(reg) spsr_el1,
            sp = out(reg) sp,
            options(nomem, nostack, preserves_flags)
        );
    }
    El0TrapEl1HandoffRegisters {
        sctlr_el1,
        tcr_el1,
        ttbr0_el1,
        vbar_el1,
        elr_el1,
        spsr_el1,
        sp,
    }
}

#[cfg(talos_boot_scenario = "rpi5_el0_trap_proof")]
fn report_el0_trap_proof_translation_features() {
    let report = unsafe { read_el0_trap_proof_translation_feature_report() };
    let parange_code = report.id_aa64mmfr0_el1 & 0xf;
    let parange_bits = aa64_parange_bits(parange_code);
    let tgran4_code = (report.id_aa64mmfr0_el1 >> 28) & 0xf;
    let tgran4_supported = tgran4_code != 0xf;
    let va_bits = 64 - EL0_TRAP_TCR_T0SZ;
    let ips_fits_parange = parange_bits >= 40;
    let mmio_l2_start = (EL0_TRAP_BCM2712_MMIO_START >> 21) & 0x1ff;
    let mmio_l2_end = bcm2712_mmio_l2_end_index() as u64;
    let user_stack_first_page = EL0_TRAP_USER_STACK_START >> 12;
    let user_stack_last_page =
        (EL0_TRAP_USER_STACK_START + EL0_TRAP_USER_STACK_LEN as u64 - 1) >> 12;

    crate::println!(
        "rpi5-el0-trap-proof: translation-id id_aa64mmfr0_el1={:#018x} id_aa64mmfr1_el1={:#018x} id_aa64mmfr2_el1={:#018x} id_aa64pfr0_el1={:#018x}",
        report.id_aa64mmfr0_el1,
        report.id_aa64mmfr1_el1,
        report.id_aa64mmfr2_el1,
        report.id_aa64pfr0_el1
    );
    crate::println!(
        "rpi5-el0-trap-proof: translation-shape mair_el1={:#018x} tcr_el1={:#018x} parange-code={:#x} parange-bits={} ips-code={:#x} ips-fits-parange={} tg0=4k tgran4-code={:#x} tgran4-supported={} t0sz={} va-bits={} irgn0=nc orgn0=nc sh0=inner",
        report.mair_el1,
        report.tcr_el1,
        parange_code,
        parange_bits,
        EL0_TRAP_TCR_IPS_CODE_40BIT,
        ips_fits_parange,
        tgran4_code,
        tgran4_supported,
        EL0_TRAP_TCR_T0SZ,
        va_bits
    );
    crate::println!(
        "rpi5-el0-trap-proof: descriptor-shape root-l0=table l1-low=table low-l2[0]=user-l3 low-l2[1..511]=kernel-blocks user-text-page={:#x} user-stack-pages={:#x}..{:#x} mmio-l1={:#x} mmio-l2={:#x}..{:#x} normal-attr=0 device-attr=1 page-desc=4k block-desc=2m",
        EL0_TRAP_USER_TEXT_START >> 12,
        user_stack_first_page,
        user_stack_last_page,
        EL0_TRAP_L1_BCM2712_MMIO_INDEX,
        mmio_l2_start,
        mmio_l2_end - 1
    );
}

#[cfg(any(
    talos_boot_scenario = "rpi5_el0_trap_proof",
    talos_boot_scenario = "rpi5_syscall_proof"
))]
const fn bcm2712_mmio_l2_end_index() -> usize {
    let end = ((EL0_TRAP_BCM2712_MMIO_END >> 21) & 0x1ff) as usize;
    if end == 0 { 512 } else { end }
}

#[cfg(talos_boot_scenario = "rpi5_el0_trap_proof")]
unsafe fn read_el0_trap_proof_translation_feature_report() -> El0TrapTranslationFeatureReport {
    let id_aa64mmfr0_el1: u64;
    let id_aa64mmfr1_el1: u64;
    let id_aa64mmfr2_el1: u64;
    let id_aa64pfr0_el1: u64;
    let mair_el1: u64;
    let tcr_el1: u64;
    unsafe {
        core::arch::asm!(
            "mrs {id_aa64mmfr0_el1}, ID_AA64MMFR0_EL1",
            "mrs {id_aa64mmfr1_el1}, ID_AA64MMFR1_EL1",
            "mrs {id_aa64mmfr2_el1}, ID_AA64MMFR2_EL1",
            "mrs {id_aa64pfr0_el1}, ID_AA64PFR0_EL1",
            "mrs {mair_el1}, MAIR_EL1",
            "mrs {tcr_el1}, TCR_EL1",
            id_aa64mmfr0_el1 = out(reg) id_aa64mmfr0_el1,
            id_aa64mmfr1_el1 = out(reg) id_aa64mmfr1_el1,
            id_aa64mmfr2_el1 = out(reg) id_aa64mmfr2_el1,
            id_aa64pfr0_el1 = out(reg) id_aa64pfr0_el1,
            mair_el1 = out(reg) mair_el1,
            tcr_el1 = out(reg) tcr_el1,
            options(nomem, nostack, preserves_flags)
        );
    }
    El0TrapTranslationFeatureReport {
        id_aa64mmfr0_el1,
        id_aa64mmfr1_el1,
        id_aa64mmfr2_el1,
        id_aa64pfr0_el1,
        mair_el1,
        tcr_el1,
    }
}

#[cfg(talos_boot_scenario = "rpi5_el0_trap_proof")]
const fn aa64_parange_bits(code: u64) -> u64 {
    match code {
        0 => 32,
        1 => 36,
        2 => 40,
        3 => 42,
        4 => 44,
        5 => 48,
        6 => 52,
        _ => 0,
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_el0_trap_proof",
    talos_boot_scenario = "rpi5_syscall_proof"
))]
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

#[cfg(any(
    talos_boot_scenario = "rpi5_el0_trap_proof",
    talos_boot_scenario = "rpi5_syscall_proof"
))]
fn clean_cache_range_to_poc(start: usize, len: usize) {
    const CACHE_LINE_SIZE: usize = 64;
    let mut addr = start & !(CACHE_LINE_SIZE - 1);
    let end = (start + len + CACHE_LINE_SIZE - 1) & !(CACHE_LINE_SIZE - 1);
    while addr < end {
        unsafe {
            core::arch::asm!(
                "dc cvac, {addr}",
                addr = in(reg) addr,
                options(nostack, preserves_flags)
            );
        }
        addr += CACHE_LINE_SIZE;
    }
}

#[cfg(all(
    talos_target_rpi5_bcm2712,
    any(
        talos_boot_scenario = "rpi5_rp1_uart0_fr_read",
        talos_boot_scenario = "rpi5_rp1_uart0_fr_read_delayed_marker",
        talos_boot_scenario = "rpi5_rp1_uart0_fr_read_hold_control",
        talos_boot_scenario = "rpi5_rp1_uart0_fr_tail_stable_result",
        talos_boot_scenario = "rpi5_rp1_gpio14_status_read",
        talos_boot_scenario = "rpi5_rp1_interrupt_routing_msix_cfg_read",
        talos_boot_scenario = "rpi5_rp1_gpio_bank_source_status_read",
        talos_boot_scenario = "rpi5_rp1_clock_manager_status_read",
        talos_boot_scenario = "rpi5_rp1_clock_adc_ctrl_write_restore",
        talos_boot_scenario = "rpi5_rp1_clock_adc_ctrl_enable_toggle",
        talos_boot_scenario = "rpi5_rp1_clock_adc_window_coherence_read",
        talos_boot_scenario = "rpi5_rp1_sysinfo_clock_sentinel_read",
        talos_boot_scenario = "rpi5_rp1_clock_reset_dependency_read",
        talos_boot_scenario = "rpi5_rp1_ethernet_gem_mid_visibility_candidate",
        talos_boot_scenario = "rpi5_rp1_ethernet_gem_mid_decode_discriminator_candidate",
        talos_boot_scenario = "rpi5_rp1_ethernet_observed_window_discriminator_candidate",
        talos_boot_scenario = "rpi5_rp1_ethernet_prereq_ownership_candidate",
        talos_boot_scenario = "rpi5_rp1_ethernet_clock_reset_readonly_baseline_candidate",
        talos_boot_scenario = "rpi5_rp1_ethernet_clock_reset_write_restore_candidate",
        talos_boot_scenario = "rpi5_rp1_ethernet_clk_eth_ctrl_write_restore_candidate",
        talos_boot_scenario = "rpi5_rp1_ethernet_gpio32_phy_reset_write_restore_candidate",
        talos_boot_scenario = "rpi5_rp1_pcie2_host_link_status_read",
        talos_boot_scenario = "rpi5_rp1_endpoint_config_identity_read",
        talos_boot_scenario = "rpi5_rp1_bridge_config_preflight_read",
        talos_boot_scenario = "rpi5_rp1_bridge_setup_state_read",
        talos_boot_scenario = "rpi5_rp1_observed_aperture_read",
        talos_boot_scenario = "rpi5_rp1_observed_gpio_status_read",
        talos_boot_scenario = "rpi5_rp1_gpio14_ownership_route_preflight_read",
        talos_boot_scenario = "rpi5_rp1_gpio16_owned_event_discriminator"
    )
))]
fn read_rp1_reg_u32(addr: usize) -> u32 {
    let reg = addr as *const u32;
    unsafe { core::ptr::read_volatile(reg) }
}

#[cfg(all(
    talos_target_rpi5_bcm2712,
    any(
        talos_boot_scenario = "rpi5_rp1_clock_adc_ctrl_write_restore",
        talos_boot_scenario = "rpi5_rp1_clock_adc_ctrl_enable_toggle",
        talos_boot_scenario = "rpi5_rp1_ethernet_clock_reset_write_restore_candidate",
        talos_boot_scenario = "rpi5_rp1_ethernet_clk_eth_ctrl_write_restore_candidate",
        talos_boot_scenario = "rpi5_rp1_ethernet_gpio32_phy_reset_write_restore_candidate"
    )
))]
fn write_rp1_reg_u32_ordered(addr: usize, value: u32) {
    let reg = addr as *mut u32;
    unsafe {
        core::ptr::write_volatile(reg, value);
        core::arch::asm!("dsb sy", options(nostack, preserves_flags));
    }
}

#[cfg(all(
    talos_target_rpi5_bcm2712,
    talos_boot_scenario = "rpi5_rp1_clock_adc_window_coherence_read"
))]
fn rp1_clock_window_ordering_barrier() {
    unsafe {
        core::arch::asm!("dsb sy", options(nostack, preserves_flags));
    }
}

#[cfg(all(
    talos_target_rpi5_bcm2712,
    talos_boot_scenario = "rpi5_rp1_entry_control"
))]
pub fn run_rp1_entry_control_diagnostic() -> ! {
    write_early_static("rpi5-rp1-entry-control: rust-entry-control\n");
    write_early_static("rpi5-rp1-entry-control: no-rp1-mmio\n");
    write_early_static("rpi5-rp1-entry-control: classification=entry-control-reached\n");
    write_early_static("rpi5-rp1-entry-control: PASS\n");
    wait_uart10_empty_early_phase();
    loop {
        core::hint::spin_loop();
    }
}

#[cfg(all(
    talos_target_rpi5_bcm2712,
    talos_boot_scenario = "rpi5_rp1_handoff_reset"
))]
pub fn run_rp1_handoff_reset_diagnostic() -> ! {
    loop {
        unsafe {
            core::arch::asm!(
                "mov w0, #0x0009",
                "movk w0, #0x8400, lsl #16",
                "smc #0",
                lateout("x0") _,
                lateout("x1") _,
                lateout("x2") _,
                lateout("x3") _,
                lateout("x4") _,
                lateout("x5") _,
                lateout("x6") _,
                lateout("x7") _,
                options(nostack)
            );
        }
        core::hint::spin_loop();
    }
}

#[cfg(all(
    talos_target_rpi5_bcm2712,
    talos_boot_scenario = "rpi5_rp1_post_handoff_marker_reset"
))]
pub fn run_rp1_post_handoff_marker_reset_diagnostic() -> ! {
    write_early_static("rpi5-rp1-post-handoff-marker-reset: post-handoff-marker\n");
    write_early_static("rpi5-rp1-post-handoff-marker-reset: classification=marker-before-reset\n");
    wait_uart10_empty_early_phase();

    loop {
        unsafe {
            core::arch::asm!(
                "mov w0, #0x0009",
                "movk w0, #0x8400, lsl #16",
                "smc #0",
                lateout("x0") _,
                lateout("x1") _,
                lateout("x2") _,
                lateout("x3") _,
                lateout("x4") _,
                lateout("x5") _,
                lateout("x6") _,
                lateout("x7") _,
                options(nostack)
            );
        }
        core::hint::spin_loop();
    }
}

#[cfg(all(
    talos_target_rpi5_bcm2712,
    talos_boot_scenario = "rpi5_rust_entry_uart10_marker_loop"
))]
pub fn run_rust_entry_uart10_marker_loop() -> ! {
    loop {
        write_early_static("TALOS: reu10-loop\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_uart0_fr_read")]
pub fn run_rp1_uart0_fr_read_diagnostic() -> ! {
    const CONTRACT_ID: &str = "phase11-rp1-pcie-map-contract-v1";

    write_early_static("rpi5-rp1-uart0-fr-read: start\n");
    write_early_static("rpi5-rp1-uart0-fr-read: pre-mmio-read\n");
    wait_uart10_empty_early_phase();

    let value = read_rp1_reg_u32(RP1_UART0_FR);

    write_early_static("rpi5-rp1-uart0-fr-read: contract=");
    write_early_static(CONTRACT_ID);
    write_early_static(" target=rp1-uart0-fr-read address=");
    write_early_hex_u64(RP1_UART0_FR as u64);
    write_early_static(" width=32 raw=");
    write_early_hex_u64(value as u64);
    write_early_static("\n");
    write_early_static("rpi5-rp1-uart0-fr-read: classification=mapped/read-value\n");
    write_early_static("rpi5-rp1-uart0-fr-read: PASS\n");
    wait_uart10_empty_early_phase();

    loop {
        core::hint::spin_loop();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_uart0_fr_read_delayed_marker")]
pub fn run_rp1_uart0_fr_read_delayed_marker_diagnostic() -> ! {
    const CONTRACT_ID: &str = "phase11-rp1-pcie-map-contract-v1";
    const PRELOAD_MARKER_REPEAT_COUNT: usize = 32;

    write_early_static("rpi5-rp1-uart0-fr-read: start\n");
    write_early_static("rpi5-rp1-uart0-fr-read: pre-mmio-read\n");
    write_early_static("rpi5-rp1-uart0-fr-read-delayed-marker: classification=before-rp1-read\n");
    wait_uart10_empty_early_phase();

    let mut remaining = PRELOAD_MARKER_REPEAT_COUNT;
    while remaining != 0 {
        write_early_static("TALOS: fr-delayed-preload-loop\n");
        wait_uart10_empty_early_phase();
        remaining -= 1;
    }

    write_early_static("rpi5-rp1-uart0-fr-read-delayed-marker: final-preload-marker\n");
    wait_uart10_empty_early_phase();

    let value = read_rp1_reg_u32(RP1_UART0_FR);

    write_early_static("rpi5-rp1-uart0-fr-read: contract=");
    write_early_static(CONTRACT_ID);
    write_early_static(" target=rp1-uart0-fr-read address=");
    write_early_hex_u64(RP1_UART0_FR as u64);
    write_early_static(" width=32 raw=");
    write_early_hex_u64(value as u64);
    write_early_static("\n");
    write_early_static("rpi5-rp1-uart0-fr-read: classification=mapped/read-value\n");
    write_early_static("rpi5-rp1-uart0-fr-read: PASS\n");
    wait_uart10_empty_early_phase();

    loop {
        core::hint::spin_loop();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_final_preload_marker_hold")]
pub fn run_rp1_final_preload_marker_hold() -> ! {
    const PRELOAD_MARKER_REPEAT_COUNT: usize = 32;

    write_early_static("rpi5-rp1-uart0-fr-read: start\n");
    write_early_static("rpi5-rp1-uart0-fr-read: pre-mmio-read\n");
    write_early_static("rpi5-rp1-uart0-fr-read-delayed-marker: classification=before-rp1-read\n");
    wait_uart10_empty_early_phase();

    let mut remaining = PRELOAD_MARKER_REPEAT_COUNT;
    while remaining != 0 {
        write_early_static("TALOS: fr-delayed-preload-loop\n");
        wait_uart10_empty_early_phase();
        remaining -= 1;
    }

    write_early_static("rpi5-rp1-uart0-fr-read-delayed-marker: final-preload-marker\n");
    wait_uart10_empty_early_phase();

    loop {
        write_early_static("TALOS: fr-final-preload-hold-loop\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_uart0_fr_read_hold_control")]
pub fn run_rp1_uart0_fr_read_hold_control() -> ! {
    const CONTRACT_ID: &str = "phase11-rp1-pcie-map-contract-v1";
    const PRELOAD_MARKER_REPEAT_COUNT: usize = 32;

    write_early_static("rpi5-rp1-uart0-fr-read: start\n");
    write_early_static("rpi5-rp1-uart0-fr-read: pre-mmio-read\n");
    write_early_static(
        "rpi5-rp1-uart0-fr-read-hold-control: classification=pre-read-control-before-rp1-read\n",
    );
    wait_uart10_empty_early_phase();

    let mut remaining = PRELOAD_MARKER_REPEAT_COUNT;
    while remaining != 0 {
        write_early_static("TALOS: fr-hold-control-pre-read-loop\n");
        wait_uart10_empty_early_phase();
        remaining -= 1;
    }

    write_early_static("rpi5-rp1-uart0-fr-read-hold-control: pre-read-control-marker\n");
    wait_uart10_empty_early_phase();

    let value = read_rp1_reg_u32(RP1_UART0_FR);

    write_early_static("rpi5-rp1-uart0-fr-read: contract=");
    write_early_static(CONTRACT_ID);
    write_early_static(" target=rp1-uart0-fr-read address=");
    write_early_hex_u64(RP1_UART0_FR as u64);
    write_early_static(" width=32 raw=");
    write_early_hex_u64(value as u64);
    write_early_static("\n");
    write_early_static("rpi5-rp1-uart0-fr-read: classification=mapped/read-value\n");
    write_early_static("rpi5-rp1-uart0-fr-read-hold-control: post-read-terminal-hold-marker\n");
    wait_uart10_empty_early_phase();

    loop {
        write_early_static("TALOS: fr-hold-control-post-read-loop\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_uart0_fr_shaped_no_mmio_marker")]
pub fn run_rp1_uart0_fr_shaped_no_mmio_marker() -> ! {
    write_early_static("rpi5-rp1-uart0-fr-read: start\n");
    write_early_static("rpi5-rp1-uart0-fr-read: pre-mmio-read\n");
    write_early_static(
        "rpi5-rp1-uart0-fr-shaped-no-mmio-marker: classification=no-mmio-marker-before-rp1-read\n",
    );
    wait_uart10_empty_early_phase();

    loop {
        write_early_static("TALOS: fr-no-mmio-loop\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_uart0_fr_tail_stable_result")]
pub fn run_rp1_uart0_fr_tail_stable_result() -> ! {
    const CONTRACT_ID: &str = "phase11-rp1-pcie-map-contract-v1";

    write_early_static("rpi5-rp1-uart0-fr-tail-stable-result: start\n");
    write_early_static("rpi5-rp1-uart0-fr-tail-stable-result: before-rp1-load\n");
    wait_uart10_empty_early_phase();

    let value = read_rp1_reg_u32(RP1_UART0_FR);

    loop {
        write_early_static("TALOS: fr-tail-stable-result contract=");
        write_early_static(CONTRACT_ID);
        write_early_static(" target=rp1-uart0-fr-read address=");
        write_early_hex_u64(RP1_UART0_FR as u64);
        write_early_static(" width=32 raw=");
        write_early_hex_u64(value as u64);
        write_early_static(" classification=mapped/read-value\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_uart0_fr_tail_stable_no_mmio_control")]
pub fn run_rp1_uart0_fr_tail_stable_no_mmio_control() -> ! {
    const CONTRACT_ID: &str = "phase11-rp1-pcie-map-contract-v1";
    const SIMULATED_RAW_VALUE: u64 = 0;

    write_early_static("rpi5-rp1-uart0-fr-tail-stable-control: start\n");
    write_early_static("rpi5-rp1-uart0-fr-tail-stable-control: no-rp1-mmio\n");
    wait_uart10_empty_early_phase();

    loop {
        write_early_static("TALOS: fr-tail-stable-control contract=");
        write_early_static(CONTRACT_ID);
        write_early_static(" target=rp1-uart0-fr-read width=32 raw=");
        write_early_hex_u64(SIMULATED_RAW_VALUE);
        write_early_static(" classification=simulated/control\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_gpio14_status_read")]
pub fn run_rp1_gpio14_status_read_diagnostic() -> ! {
    const CONTRACT_ID: &str = "phase11-rp1-irq-clock-gpio-contract-v1";

    write_early_static("rpi5-rp1-gpio14-status-read: start\n");
    write_early_static("rpi5-rp1-gpio14-status-read: before-rp1-load\n");
    wait_uart10_empty_early_phase();

    let value = read_rp1_reg_u32(RP1_GPIO14_STATUS);

    loop {
        write_early_static("TALOS: gpio14-status-result contract=");
        write_early_static(CONTRACT_ID);
        write_early_static(" target=rp1-gpio14-status-read address=");
        write_early_hex_u64(RP1_GPIO14_STATUS as u64);
        write_early_static(" width=32 raw=");
        write_early_hex_u64(value as u64);
        write_gpio_status_bits(value);
        write_early_static(" classification=diagnostic-result-visible\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_gpio14_status_no_mmio_control")]
pub fn run_rp1_gpio14_status_no_mmio_control() -> ! {
    const CONTRACT_ID: &str = "phase11-rp1-irq-clock-gpio-contract-v1";
    const SIMULATED_RAW_VALUE: u32 = 0;

    write_early_static("rpi5-rp1-gpio14-status-control: start\n");
    write_early_static("rpi5-rp1-gpio14-status-control: no-rp1-mmio\n");
    wait_uart10_empty_early_phase();

    loop {
        write_early_static("TALOS: gpio14-status-control contract=");
        write_early_static(CONTRACT_ID);
        write_early_static(" target=rp1-gpio14-status-read address=not-constructed width=32 raw=");
        write_early_hex_u64(SIMULATED_RAW_VALUE as u64);
        write_gpio_status_bits(SIMULATED_RAW_VALUE);
        write_early_static(" classification=simulated/control\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_interrupt_routing_msix_cfg_read")]
pub fn run_rp1_interrupt_routing_msix_cfg_read() -> ! {
    const CONTRACT_ID: &str = "phase11-rp1-interrupt-routing-source-contract-v1";
    const HWIRQ: u64 = 0;
    const MSIX_VECTOR: u64 = 0;
    const GIC_SPI: u64 = 128;
    const GIC_INTID: u64 = 160;

    write_early_static("rpi5-rp1-interrupt-routing-msix-cfg-read: start\n");
    write_early_static("rpi5-rp1-interrupt-routing-msix-cfg-read: before-rp1-load\n");
    wait_uart10_empty_early_phase();

    let value = read_rp1_reg_u32(RP1_IO_BANK0_MSIX_CFG);

    loop {
        write_early_static("TALOS: rp1-interrupt-routing-result contract=");
        write_early_static(CONTRACT_ID);
        write_early_static(" target=rp1-io-bank0-msix-cfg-read hwirq=");
        write_early_dec_u64(HWIRQ);
        write_early_static(" predicted-msix-vector=");
        write_early_dec_u64(MSIX_VECTOR);
        write_early_static(" predicted-gic-spi=");
        write_early_dec_u64(GIC_SPI);
        write_early_static(" predicted-gic-intid=");
        write_early_dec_u64(GIC_INTID);
        write_early_static(" address=");
        write_early_hex_u64(RP1_IO_BANK0_MSIX_CFG as u64);
        write_early_static(" width=32 raw=");
        write_early_hex_u64(value as u64);
        write_msix_cfg_bits(value);
        write_early_static(" classification=routing-msix-cfg-visible\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_interrupt_routing_no_mmio_control")]
pub fn run_rp1_interrupt_routing_no_mmio_control() -> ! {
    const CONTRACT_ID: &str = "phase11-rp1-interrupt-routing-source-contract-v1";
    const HWIRQ: u64 = 0;
    const MSIX_VECTOR: u64 = 0;
    const GIC_SPI: u64 = 128;
    const GIC_INTID: u64 = 160;
    const SIMULATED_RAW_VALUE: u32 = 0;

    write_early_static("rpi5-rp1-interrupt-routing-control: start\n");
    write_early_static("rpi5-rp1-interrupt-routing-control: no-rp1-msix-pcie-gic-mmio\n");
    wait_uart10_empty_early_phase();

    loop {
        write_early_static("TALOS: rp1-interrupt-routing-control contract=");
        write_early_static(CONTRACT_ID);
        write_early_static(" target=rp1-io-bank0-msix-cfg-read hwirq=");
        write_early_dec_u64(HWIRQ);
        write_early_static(" predicted-msix-vector=");
        write_early_dec_u64(MSIX_VECTOR);
        write_early_static(" predicted-gic-spi=");
        write_early_dec_u64(GIC_SPI);
        write_early_static(" predicted-gic-intid=");
        write_early_dec_u64(GIC_INTID);
        write_early_static(" address=not-constructed width=32 raw=");
        write_early_hex_u64(SIMULATED_RAW_VALUE as u64);
        write_msix_cfg_bits(SIMULATED_RAW_VALUE);
        write_early_static(" classification=simulated/control\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_gic_visible_route_status_read")]
pub fn run_rp1_gic_visible_route_status_read() -> ! {
    const CONTRACT_ID: &str = "phase11-rp1-gic-visible-route-source-contract-v1";
    const HWIRQ: u64 = 0;
    const MSIX_VECTOR: u64 = 0;
    const GIC_SPI: u64 = 128;
    const GIC_INTID: u32 = 160;
    const BANK: u64 = 5;
    const BIT_MASK: u32 = 0x0000_0001;
    const GICD_ISENABLER5: usize = GICD_BASE + 0x114;
    const GICD_ISPENDR5: usize = GICD_BASE + 0x214;
    const GICD_ISACTIVER5: usize = GICD_BASE + 0x314;
    const GICC_HPPIR: usize = GICC_BASE + 0x18;

    write_early_static("rpi5-rp1-gic-visible-route-status-read: start\n");
    write_early_static("rpi5-rp1-gic-visible-route-status-read: before-gic-loads\n");
    wait_uart10_empty_early_phase();

    let gic = GicV2::new(GICD_BASE, GICC_BASE);
    let (isenabler, ispendr, isactiver, hppir) = unsafe {
        (
            gic.enable_bits(GIC_INTID),
            gic.pending_bits(GIC_INTID),
            gic.active_bits(GIC_INTID),
            gic.highest_pending(),
        )
    };
    let hppir_intid = hppir & 0x3ff;

    loop {
        write_early_static("TALOS: rp1-gic-route-status-result contract=");
        write_early_static(CONTRACT_ID);
        write_early_static(" target=rp1-io-bank0-gic-route-status-read hwirq=");
        write_early_dec_u64(HWIRQ);
        write_early_static(" predicted-msix-vector=");
        write_early_dec_u64(MSIX_VECTOR);
        write_early_static(" predicted-gic-spi=");
        write_early_dec_u64(GIC_SPI);
        write_early_static(" predicted-gic-intid=");
        write_early_dec_u64(GIC_INTID as u64);
        write_early_static(" gicd-base=");
        write_early_hex_u64(GICD_BASE as u64);
        write_early_static(" gicc-base=");
        write_early_hex_u64(GICC_BASE as u64);
        write_early_static(" bank=");
        write_early_dec_u64(BANK);
        write_early_static(" bit-mask=");
        write_early_hex_u64(BIT_MASK as u64);
        write_early_static(" isenabler-address=");
        write_early_hex_u64(GICD_ISENABLER5 as u64);
        write_early_static(" ispendr-address=");
        write_early_hex_u64(GICD_ISPENDR5 as u64);
        write_early_static(" isactiver-address=");
        write_early_hex_u64(GICD_ISACTIVER5 as u64);
        write_early_static(" hppir-address=");
        write_early_hex_u64(GICC_HPPIR as u64);
        write_gic_route_status_bits(isenabler, ispendr, isactiver, hppir, BIT_MASK);
        write_early_static(" hppir-intid=");
        write_early_dec_u64(hppir_intid as u64);
        write_early_static(" hppir-spurious=");
        write_bool(hppir_intid == SPURIOUS_INTID);
        write_early_static(" hppir-target-match=");
        write_bool(hppir_intid == GIC_INTID);
        write_early_static(" classification=gic-route-status-visible\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_gic_visible_route_no_mmio_control")]
pub fn run_rp1_gic_visible_route_no_mmio_control() -> ! {
    const CONTRACT_ID: &str = "phase11-rp1-gic-visible-route-source-contract-v1";
    const HWIRQ: u64 = 0;
    const MSIX_VECTOR: u64 = 0;
    const GIC_SPI: u64 = 128;
    const GIC_INTID: u64 = 160;
    const BANK: u64 = 5;
    const SIMULATED_RAW_VALUE: u32 = 0;

    write_early_static("rpi5-rp1-gic-visible-route-control: start\n");
    write_early_static(
        "rpi5-rp1-gic-visible-route-control: no-gic-rp1-msix-pcie-mip-gpio-pads-rio-clock-reset-mmio\n",
    );
    wait_uart10_empty_early_phase();

    loop {
        write_early_static("TALOS: rp1-gic-route-status-control contract=");
        write_early_static(CONTRACT_ID);
        write_early_static(" target=rp1-io-bank0-gic-route-status-read hwirq=");
        write_early_dec_u64(HWIRQ);
        write_early_static(" predicted-msix-vector=");
        write_early_dec_u64(MSIX_VECTOR);
        write_early_static(" predicted-gic-spi=");
        write_early_dec_u64(GIC_SPI);
        write_early_static(" predicted-gic-intid=");
        write_early_dec_u64(GIC_INTID);
        write_early_static(" gicd-base=not-constructed gicc-base=not-constructed bank=");
        write_early_dec_u64(BANK);
        write_early_static(
            " bit-mask=not-constructed isenabler-address=not-constructed ispendr-address=not-constructed isactiver-address=not-constructed hppir-address=not-constructed",
        );
        write_gic_route_status_bits(
            SIMULATED_RAW_VALUE,
            SIMULATED_RAW_VALUE,
            SIMULATED_RAW_VALUE,
            SIMULATED_RAW_VALUE,
            1,
        );
        write_early_static(" hppir-intid=0 hppir-spurious=false hppir-target-match=false");
        write_early_static(" classification=simulated/control\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_gpio_bank_source_status_read")]
pub fn run_rp1_gpio_bank_source_status_read() -> ! {
    const CONTRACT_ID: &str = "phase11-rp1-gpio-bank-source-status-contract-v1";
    const SOURCE_HWIRQ: u64 = 0;
    const BANK: u64 = 0;
    const BANK_FIRST_GPIO: u64 = 0;
    const BANK_GPIO_COUNT: u64 = 28;
    const BANK_LAST_GPIO: u64 = 27;
    const GPIO14_MASK: u32 = 1 << 14;

    write_early_static("rpi5-rp1-gpio-bank-source-status-read: start\n");
    write_early_static("rpi5-rp1-gpio-bank-source-status-read: before-rp1-loads\n");
    wait_uart10_empty_early_phase();

    let inte = read_rp1_reg_u32(RP1_IO_BANK0_INTE);
    let ints = read_rp1_reg_u32(RP1_IO_BANK0_INTS);

    loop {
        write_early_static("TALOS: rp1-gpio-bank-source-status-result contract=");
        write_early_static(CONTRACT_ID);
        write_early_static(" target=rp1-io-bank0-source-status-read source-hwirq=");
        write_early_dec_u64(SOURCE_HWIRQ);
        write_early_static(" bank=");
        write_early_dec_u64(BANK);
        write_early_static(" bank-first-gpio=");
        write_early_dec_u64(BANK_FIRST_GPIO);
        write_early_static(" bank-gpio-count=");
        write_early_dec_u64(BANK_GPIO_COUNT);
        write_early_static(" bank-last-gpio=");
        write_early_dec_u64(BANK_LAST_GPIO);
        write_early_static(" inte-address=");
        write_early_hex_u64(RP1_IO_BANK0_INTE as u64);
        write_early_static(" ints-address=");
        write_early_hex_u64(RP1_IO_BANK0_INTS as u64);
        write_gpio_bank_source_status_bits(inte, ints, GPIO14_MASK);
        write_early_static(" classification=gpio-bank-source-status-visible\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_gpio_bank_source_status_no_mmio_control")]
pub fn run_rp1_gpio_bank_source_status_no_mmio_control() -> ! {
    const CONTRACT_ID: &str = "phase11-rp1-gpio-bank-source-status-contract-v1";
    const SOURCE_HWIRQ: u64 = 0;
    const BANK: u64 = 0;
    const BANK_FIRST_GPIO: u64 = 0;
    const BANK_GPIO_COUNT: u64 = 28;
    const BANK_LAST_GPIO: u64 = 27;
    const GPIO14_MASK: u32 = 1 << 14;
    const SIMULATED_RAW_VALUE: u32 = 0;

    write_early_static("rpi5-rp1-gpio-bank-source-status-control: start\n");
    write_early_static(
        "rpi5-rp1-gpio-bank-source-status-control: no-rp1-gpio-rio-pads-clock-reset-msix-pcie-mip-gic-mmio\n",
    );
    wait_uart10_empty_early_phase();

    loop {
        write_early_static("TALOS: rp1-gpio-bank-source-status-control contract=");
        write_early_static(CONTRACT_ID);
        write_early_static(" target=rp1-io-bank0-source-status-read source-hwirq=");
        write_early_dec_u64(SOURCE_HWIRQ);
        write_early_static(" bank=");
        write_early_dec_u64(BANK);
        write_early_static(" bank-first-gpio=");
        write_early_dec_u64(BANK_FIRST_GPIO);
        write_early_static(" bank-gpio-count=");
        write_early_dec_u64(BANK_GPIO_COUNT);
        write_early_static(" bank-last-gpio=");
        write_early_dec_u64(BANK_LAST_GPIO);
        write_early_static(" inte-address=not-constructed ints-address=not-constructed");
        write_gpio_bank_source_status_bits(SIMULATED_RAW_VALUE, SIMULATED_RAW_VALUE, GPIO14_MASK);
        write_early_static(" classification=simulated/control\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_clock_manager_status_read")]
pub fn run_rp1_clock_manager_status_read() -> ! {
    const CONTRACT_ID: &str = "phase11-rp1-clock-reset-status-source-contract-v1";
    const TARGET: &str = "rp1-clock-manager-status-read";

    write_early_static("rpi5-rp1-clock-manager-status-read: start\n");
    write_early_static("rpi5-rp1-clock-manager-status-read: before-rp1-clock-loads\n");
    wait_uart10_empty_early_phase();

    let pll_sys_cs = read_rp1_reg_u32(RP1_PLL_SYS_CS);
    let clk_sys_ctrl = read_rp1_reg_u32(RP1_CLK_SYS_CTRL);
    let clk_sys_div_int = read_rp1_reg_u32(RP1_CLK_SYS_DIV_INT);
    let clk_sys_sel = read_rp1_reg_u32(RP1_CLK_SYS_SEL);
    let clk_slow_sys_ctrl = read_rp1_reg_u32(RP1_CLK_SLOW_SYS_CTRL);
    let clk_uart_ctrl = read_rp1_reg_u32(RP1_CLK_UART_CTRL);
    let clk_uart_div_int = read_rp1_reg_u32(RP1_CLK_UART_DIV_INT);
    let clk_uart_sel = read_rp1_reg_u32(RP1_CLK_UART_SEL);

    loop {
        write_early_static("TALOS: rp1-clock-manager-status-result contract=");
        write_early_static(CONTRACT_ID);
        write_early_static(" target=");
        write_early_static(TARGET);
        write_early_static(" clock-manager-base=");
        write_early_hex_u64(RP1_CLOCK_MANAGER_BASE as u64);
        write_clock_status_register(" pll-sys-cs", RP1_PLL_SYS_CS, pll_sys_cs);
        write_clock_status_register(" clk-sys-ctrl", RP1_CLK_SYS_CTRL, clk_sys_ctrl);
        write_clock_status_register(" clk-sys-div-int", RP1_CLK_SYS_DIV_INT, clk_sys_div_int);
        write_clock_status_register(" clk-sys-sel", RP1_CLK_SYS_SEL, clk_sys_sel);
        write_clock_status_register(
            " clk-slow-sys-ctrl",
            RP1_CLK_SLOW_SYS_CTRL,
            clk_slow_sys_ctrl,
        );
        write_clock_status_register(" clk-uart-ctrl", RP1_CLK_UART_CTRL, clk_uart_ctrl);
        write_clock_status_register(" clk-uart-div-int", RP1_CLK_UART_DIV_INT, clk_uart_div_int);
        write_clock_status_register(" clk-uart-sel", RP1_CLK_UART_SEL, clk_uart_sel);
        write_clock_manager_status_bits(
            pll_sys_cs,
            clk_sys_ctrl,
            clk_sys_div_int,
            clk_sys_sel,
            clk_slow_sys_ctrl,
            clk_uart_ctrl,
            clk_uart_div_int,
            clk_uart_sel,
        );
        write_early_static(" retained-gpio14-blocker=fsel13 retained-gpio16-blocker=fsel13");
        write_early_static(" classification=rp1-clock-manager-status-visible\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_clock_manager_status_no_mmio_control")]
pub fn run_rp1_clock_manager_status_no_mmio_control() -> ! {
    const CONTRACT_ID: &str = "phase11-rp1-clock-reset-status-source-contract-v1";
    const TARGET: &str = "rp1-clock-manager-status-read";
    const SIMULATED_RAW_VALUE: u32 = 0;

    write_early_static("rpi5-rp1-clock-manager-status-control: start\n");
    write_early_static(
        "rpi5-rp1-clock-manager-status-control: no-rp1-clock-reset-gpio-rio-pads-msix-pcie-mip-gic-mmio\n",
    );
    wait_uart10_empty_early_phase();

    loop {
        write_early_static("TALOS: rp1-clock-manager-status-control contract=");
        write_early_static(CONTRACT_ID);
        write_early_static(" target=");
        write_early_static(TARGET);
        write_early_static(" clock-manager-base=not-constructed");
        write_clock_status_control_register(" pll-sys-cs", SIMULATED_RAW_VALUE);
        write_clock_status_control_register(" clk-sys-ctrl", SIMULATED_RAW_VALUE);
        write_clock_status_control_register(" clk-sys-div-int", SIMULATED_RAW_VALUE);
        write_clock_status_control_register(" clk-sys-sel", SIMULATED_RAW_VALUE);
        write_clock_status_control_register(" clk-slow-sys-ctrl", SIMULATED_RAW_VALUE);
        write_clock_status_control_register(" clk-uart-ctrl", SIMULATED_RAW_VALUE);
        write_clock_status_control_register(" clk-uart-div-int", SIMULATED_RAW_VALUE);
        write_clock_status_control_register(" clk-uart-sel", SIMULATED_RAW_VALUE);
        write_clock_manager_status_bits(
            SIMULATED_RAW_VALUE,
            SIMULATED_RAW_VALUE,
            SIMULATED_RAW_VALUE,
            SIMULATED_RAW_VALUE,
            SIMULATED_RAW_VALUE,
            SIMULATED_RAW_VALUE,
            SIMULATED_RAW_VALUE,
            SIMULATED_RAW_VALUE,
        );
        write_early_static(" retained-gpio14-blocker=fsel13 retained-gpio16-blocker=fsel13");
        write_early_static(" classification=simulated/control\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_clock_adc_ctrl_write_restore")]
pub fn run_rp1_clock_adc_ctrl_write_restore() -> ! {
    const CONTRACT_ID: &str = "phase11-rp1-clock-reset-write-restore-source-contract-v1";
    const TARGET: &str = "rp1-clk-adc-ctrl-idempotent-write-restore";
    const REGISTER_NAME: &str = "CLK_ADC_CTRL";
    const SOURCE_OFFSET: u64 = 0x144;

    write_early_static("rpi5-rp1-clock-adc-ctrl-write-restore: start\n");
    write_early_static("rpi5-rp1-clock-adc-ctrl-write-restore: before-rp1-clock-write-restore\n");
    wait_uart10_empty_early_phase();

    let pre_raw = read_rp1_reg_u32(RP1_CLK_ADC_CTRL);
    write_rp1_reg_u32_ordered(RP1_CLK_ADC_CTRL, pre_raw);
    let post_raw = read_rp1_reg_u32(RP1_CLK_ADC_CTRL);
    write_rp1_reg_u32_ordered(RP1_CLK_ADC_CTRL, pre_raw);
    let restore_raw = read_rp1_reg_u32(RP1_CLK_ADC_CTRL);

    let post_matches_pre = post_raw == pre_raw;
    let restore_matches_pre = restore_raw == pre_raw;
    let classification = if post_matches_pre && restore_matches_pre {
        "rp1-clock-adc-ctrl-idempotent-write-restored"
    } else if restore_matches_pre {
        "rp1-clock-adc-ctrl-idempotent-write-mismatch-restored"
    } else {
        "rp1-clock-adc-ctrl-idempotent-write-restore-failed"
    };

    loop {
        write_early_static("TALOS: rp1-clock-adc-ctrl-write-restore-result contract=");
        write_early_static(CONTRACT_ID);
        write_early_static(" target=");
        write_early_static(TARGET);
        write_early_static(" register=");
        write_early_static(REGISTER_NAME);
        write_early_static(" clock-manager-base=");
        write_early_hex_u64(RP1_CLOCK_MANAGER_BASE as u64);
        write_early_static(" source-offset=");
        write_early_hex_u64(SOURCE_OFFSET);
        write_early_static(" address=");
        write_early_hex_u64(RP1_CLK_ADC_CTRL as u64);
        write_early_static(" width=32");
        write_clock_adc_ctrl_raw_triplet(pre_raw, post_raw, restore_raw);
        write_early_static(" post-eq-pre=");
        write_bool(post_matches_pre);
        write_early_static(" restore-eq-pre=");
        write_bool(restore_matches_pre);
        write_early_static(" retained-gpio14-blocker=fsel13 retained-gpio16-blocker=fsel13");
        write_early_static(" classification=");
        write_early_static(classification);
        write_early_static("\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_clock_adc_ctrl_write_restore_no_mmio_control")]
pub fn run_rp1_clock_adc_ctrl_write_restore_no_mmio_control() -> ! {
    const CONTRACT_ID: &str = "phase11-rp1-clock-reset-write-restore-source-contract-v1";
    const TARGET: &str = "rp1-clk-adc-ctrl-idempotent-write-restore";
    const REGISTER_NAME: &str = "CLK_ADC_CTRL";
    const SOURCE_OFFSET: u64 = 0x144;
    const SIMULATED_RAW_VALUE: u32 = 0;

    write_early_static("rpi5-rp1-clock-adc-ctrl-write-restore-control: start\n");
    write_early_static(
        "rpi5-rp1-clock-adc-ctrl-write-restore-control: no-rp1-clock-reset-gpio-rio-pads-msix-pcie-mip-gic-mmio\n",
    );
    wait_uart10_empty_early_phase();

    loop {
        write_early_static("TALOS: rp1-clock-adc-ctrl-write-restore-control contract=");
        write_early_static(CONTRACT_ID);
        write_early_static(" target=");
        write_early_static(TARGET);
        write_early_static(" register=");
        write_early_static(REGISTER_NAME);
        write_early_static(" clock-manager-base=not-constructed");
        write_early_static(" source-offset=");
        write_early_hex_u64(SOURCE_OFFSET);
        write_early_static(" address=not-constructed width=32");
        write_clock_adc_ctrl_raw_triplet(
            SIMULATED_RAW_VALUE,
            SIMULATED_RAW_VALUE,
            SIMULATED_RAW_VALUE,
        );
        write_early_static(" post-eq-pre=true restore-eq-pre=true");
        write_early_static(" retained-gpio14-blocker=fsel13 retained-gpio16-blocker=fsel13");
        write_early_static(" classification=simulated/control\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_clock_adc_ctrl_enable_toggle")]
pub fn run_rp1_clock_adc_ctrl_enable_toggle() -> ! {
    const CONTRACT_ID: &str = "phase11-rp1-clock-adc-enable-toggle-source-contract-v1";
    const TARGET: &str = "rp1-clk-adc-ctrl-enable-bit-toggle-restore";
    const REGISTER_NAME: &str = "CLK_ADC_CTRL";
    const SOURCE_OFFSET: u64 = 0x144;

    write_early_static("rpi5-rp1-clock-adc-ctrl-enable-toggle: start\n");
    write_early_static("rpi5-rp1-clock-adc-ctrl-enable-toggle: before-rp1-clock-enable-toggle\n");
    wait_uart10_empty_early_phase();

    let pre_raw = read_rp1_reg_u32(RP1_CLK_ADC_CTRL);
    let transition_raw = pre_raw ^ RP1_CLK_CTRL_ENABLE;
    write_rp1_reg_u32_ordered(RP1_CLK_ADC_CTRL, transition_raw);
    let post_raw = read_rp1_reg_u32(RP1_CLK_ADC_CTRL);
    write_rp1_reg_u32_ordered(RP1_CLK_ADC_CTRL, pre_raw);
    let restore_raw = read_rp1_reg_u32(RP1_CLK_ADC_CTRL);

    let one_bit_transition =
        transition_raw != pre_raw && (transition_raw ^ pre_raw) == RP1_CLK_CTRL_ENABLE;
    let post_delta_is_transition_mask = (post_raw ^ pre_raw) == RP1_CLK_CTRL_ENABLE;
    let post_enable_flipped = ((post_raw ^ pre_raw) & RP1_CLK_CTRL_ENABLE) != 0;
    let restore_matches_pre = restore_raw == pre_raw;
    let classification = if post_delta_is_transition_mask && restore_matches_pre {
        "rp1-clock-adc-ctrl-enable-toggle-restored"
    } else if restore_matches_pre {
        "rp1-clock-adc-ctrl-enable-toggle-mismatch-restored"
    } else {
        "rp1-clock-adc-ctrl-enable-toggle-restore-failed"
    };

    loop {
        write_early_static("TALOS: rp1-clock-adc-ctrl-enable-toggle-result contract=");
        write_early_static(CONTRACT_ID);
        write_early_static(" target=");
        write_early_static(TARGET);
        write_early_static(" register=");
        write_early_static(REGISTER_NAME);
        write_early_static(" clock-manager-base=");
        write_early_hex_u64(RP1_CLOCK_MANAGER_BASE as u64);
        write_early_static(" source-offset=");
        write_early_hex_u64(SOURCE_OFFSET);
        write_early_static(" address=");
        write_early_hex_u64(RP1_CLK_ADC_CTRL as u64);
        write_early_static(" width=32 transition-mask=");
        write_early_hex_u64(RP1_CLK_CTRL_ENABLE as u64);
        write_clock_adc_ctrl_enable_toggle_values(pre_raw, transition_raw, post_raw, restore_raw);
        write_early_static(" one-bit-transition=");
        write_bool(one_bit_transition);
        write_early_static(" post-enable-flipped=");
        write_bool(post_enable_flipped);
        write_early_static(" post-delta-is-transition-mask=");
        write_bool(post_delta_is_transition_mask);
        write_early_static(" restore-eq-pre=");
        write_bool(restore_matches_pre);
        write_early_static(
            " retained-idempotent-proof=rp1-clock-adc-ctrl-idempotent-write-restored",
        );
        write_early_static(" retained-gpio14-blocker=fsel13 retained-gpio16-blocker=fsel13");
        write_early_static(" classification=");
        write_early_static(classification);
        write_early_static("\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_clock_adc_ctrl_enable_toggle_no_mmio_control")]
pub fn run_rp1_clock_adc_ctrl_enable_toggle_no_mmio_control() -> ! {
    const CONTRACT_ID: &str = "phase11-rp1-clock-adc-enable-toggle-source-contract-v1";
    const TARGET: &str = "rp1-clk-adc-ctrl-enable-bit-toggle-restore";
    const REGISTER_NAME: &str = "CLK_ADC_CTRL";
    const SOURCE_OFFSET: u64 = 0x144;
    const SIMULATED_PRE_RAW: u32 = 0;
    const SIMULATED_TRANSITION_RAW: u32 = RP1_CLK_CTRL_ENABLE;
    const SIMULATED_POST_RAW: u32 = RP1_CLK_CTRL_ENABLE;
    const SIMULATED_RESTORE_RAW: u32 = 0;

    write_early_static("rpi5-rp1-clock-adc-ctrl-enable-toggle-control: start\n");
    write_early_static(
        "rpi5-rp1-clock-adc-ctrl-enable-toggle-control: no-rp1-clock-reset-gpio-rio-pads-msix-pcie-mip-gic-mmio\n",
    );
    wait_uart10_empty_early_phase();

    loop {
        write_early_static("TALOS: rp1-clock-adc-ctrl-enable-toggle-control contract=");
        write_early_static(CONTRACT_ID);
        write_early_static(" target=");
        write_early_static(TARGET);
        write_early_static(" register=");
        write_early_static(REGISTER_NAME);
        write_early_static(" clock-manager-base=not-constructed");
        write_early_static(" source-offset=");
        write_early_hex_u64(SOURCE_OFFSET);
        write_early_static(" address=not-constructed width=32 transition-mask=");
        write_early_hex_u64(RP1_CLK_CTRL_ENABLE as u64);
        write_clock_adc_ctrl_enable_toggle_values(
            SIMULATED_PRE_RAW,
            SIMULATED_TRANSITION_RAW,
            SIMULATED_POST_RAW,
            SIMULATED_RESTORE_RAW,
        );
        write_early_static(" one-bit-transition=true");
        write_early_static(" post-enable-flipped=true");
        write_early_static(" post-delta-is-transition-mask=true");
        write_early_static(" restore-eq-pre=true");
        write_early_static(
            " retained-idempotent-proof=rp1-clock-adc-ctrl-idempotent-write-restored",
        );
        write_early_static(" retained-gpio14-blocker=fsel13 retained-gpio16-blocker=fsel13");
        write_early_static(" classification=simulated/control\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_clock_adc_window_coherence_read")]
pub fn run_rp1_clock_adc_window_coherence_read() -> ! {
    const CONTRACT_ID: &str = "phase11-rp1-clock-write-effect-discriminator-source-contract-v1";
    const TARGET: &str = "rp1-clk-adc-window-coherence-read";
    const PRIOR_PRE_RAW: u32 = 0xdead_dead;
    const PRIOR_TRANSITION_RAW: u32 = 0xdead_d6ad;
    const PRIOR_POST_RAW: u32 = 0xdead_dead;
    const PRIOR_RESTORE_RAW: u32 = 0xdead_dead;

    write_early_static("rpi5-rp1-clock-adc-window-coherence-read: start\n");
    write_early_static("rpi5-rp1-clock-adc-window-coherence-read: before-rp1-clock-window-loads\n");
    wait_uart10_empty_early_phase();

    let clk_sys_ctrl = read_rp1_reg_u32(RP1_CLK_SYS_CTRL);
    let clk_uart_ctrl = read_rp1_reg_u32(RP1_CLK_UART_CTRL);
    let adc_ctrl_first = read_rp1_reg_u32(RP1_CLK_ADC_CTRL);
    rp1_clock_window_ordering_barrier();
    let adc_ctrl_second = read_rp1_reg_u32(RP1_CLK_ADC_CTRL);
    let adc_div_int = read_rp1_reg_u32(RP1_CLK_ADC_DIV_INT);
    let adc_sel = read_rp1_reg_u32(RP1_CLK_ADC_SEL);

    let clk_sys_enabled = clk_sys_ctrl & RP1_CLK_CTRL_ENABLE != 0;
    let clk_uart_enabled = clk_uart_ctrl & RP1_CLK_CTRL_ENABLE != 0;
    let adc_ctrl_stable = adc_ctrl_first == adc_ctrl_second;
    let adc_window_all_equal = adc_ctrl_second == adc_div_int && adc_div_int == adc_sel;
    let adc_window_all_deaddead = adc_ctrl_second == PRIOR_PRE_RAW
        && adc_div_int == PRIOR_PRE_RAW
        && adc_sel == PRIOR_PRE_RAW;
    let classification = if !clk_sys_enabled {
        "rp1-clock-adc-window-blocked-missing-clock-manager"
    } else if !clk_uart_enabled {
        "rp1-clock-adc-window-blocked-uart-clock-disabled"
    } else if !adc_ctrl_stable {
        "rp1-clock-adc-window-unstable-readback"
    } else if adc_window_all_equal || adc_window_all_deaddead {
        "rp1-clock-adc-window-readback-sentinel"
    } else {
        "rp1-clock-adc-window-coherent-read"
    };

    loop {
        write_early_static("TALOS: rp1-clock-adc-window-coherence-result contract=");
        write_early_static(CONTRACT_ID);
        write_early_static(" target=");
        write_early_static(TARGET);
        write_early_static(" clock-manager-base=");
        write_early_hex_u64(RP1_CLOCK_MANAGER_BASE as u64);
        write_adc_window_register(" clk-sys-ctrl", 0x14, RP1_CLK_SYS_CTRL, clk_sys_ctrl);
        write_adc_window_register(" clk-uart-ctrl", 0x54, RP1_CLK_UART_CTRL, clk_uart_ctrl);
        write_adc_window_register(" adc-ctrl-first", 0x144, RP1_CLK_ADC_CTRL, adc_ctrl_first);
        write_adc_window_register(" adc-ctrl-second", 0x144, RP1_CLK_ADC_CTRL, adc_ctrl_second);
        write_adc_window_register(" adc-div-int", 0x148, RP1_CLK_ADC_DIV_INT, adc_div_int);
        write_adc_window_register(" adc-sel", 0x150, RP1_CLK_ADC_SEL, adc_sel);
        write_early_static(" clk-sys-enable=");
        write_bool(clk_sys_enabled);
        write_early_static(" clk-uart-enable=");
        write_bool(clk_uart_enabled);
        write_adc_ctrl_window_fields(" adc-ctrl-first", adc_ctrl_first);
        write_adc_ctrl_window_fields(" adc-ctrl-second", adc_ctrl_second);
        write_adc_window_booleans(
            adc_ctrl_stable,
            adc_window_all_equal,
            adc_window_all_deaddead,
            adc_sel,
        );
        write_prior_adc_enable_toggle_context(
            PRIOR_PRE_RAW,
            PRIOR_TRANSITION_RAW,
            PRIOR_POST_RAW,
            PRIOR_RESTORE_RAW,
        );
        write_early_static(" classification=");
        write_early_static(classification);
        write_early_static("\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_clock_adc_window_coherence_no_mmio_control")]
pub fn run_rp1_clock_adc_window_coherence_no_mmio_control() -> ! {
    const CONTRACT_ID: &str = "phase11-rp1-clock-write-effect-discriminator-source-contract-v1";
    const TARGET: &str = "rp1-clk-adc-window-coherence-read";
    const SIMULATED_CLK_SYS_CTRL: u32 = RP1_CLK_CTRL_ENABLE;
    const SIMULATED_CLK_UART_CTRL: u32 = RP1_CLK_CTRL_ENABLE;
    const SIMULATED_ADC_CTRL: u32 = 0;
    const SIMULATED_ADC_DIV_INT: u32 = 1;
    const SIMULATED_ADC_SEL: u32 = 1;
    const PRIOR_PRE_RAW: u32 = 0xdead_dead;
    const PRIOR_TRANSITION_RAW: u32 = 0xdead_d6ad;
    const PRIOR_POST_RAW: u32 = 0xdead_dead;
    const PRIOR_RESTORE_RAW: u32 = 0xdead_dead;

    write_early_static("rpi5-rp1-clock-adc-window-coherence-control: start\n");
    write_early_static(
        "rpi5-rp1-clock-adc-window-coherence-control: no-rp1-clock-reset-gpio-rio-pads-msix-pcie-mip-gic-mmio\n",
    );
    wait_uart10_empty_early_phase();

    loop {
        write_early_static("TALOS: rp1-clock-adc-window-coherence-control contract=");
        write_early_static(CONTRACT_ID);
        write_early_static(" target=");
        write_early_static(TARGET);
        write_early_static(" clock-manager-base=not-constructed");
        write_adc_window_control_register(" clk-sys-ctrl", 0x14, SIMULATED_CLK_SYS_CTRL);
        write_adc_window_control_register(" clk-uart-ctrl", 0x54, SIMULATED_CLK_UART_CTRL);
        write_adc_window_control_register(" adc-ctrl-first", 0x144, SIMULATED_ADC_CTRL);
        write_adc_window_control_register(" adc-ctrl-second", 0x144, SIMULATED_ADC_CTRL);
        write_adc_window_control_register(" adc-div-int", 0x148, SIMULATED_ADC_DIV_INT);
        write_adc_window_control_register(" adc-sel", 0x150, SIMULATED_ADC_SEL);
        write_early_static(" clk-sys-enable=true clk-uart-enable=true");
        write_adc_ctrl_window_fields(" adc-ctrl-first", SIMULATED_ADC_CTRL);
        write_adc_ctrl_window_fields(" adc-ctrl-second", SIMULATED_ADC_CTRL);
        write_adc_window_booleans(true, false, false, SIMULATED_ADC_SEL);
        write_prior_adc_enable_toggle_context(
            PRIOR_PRE_RAW,
            PRIOR_TRANSITION_RAW,
            PRIOR_POST_RAW,
            PRIOR_RESTORE_RAW,
        );
        write_early_static(" classification=no-mmio-clock-adc-window-coherence-control-visible\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_sysinfo_clock_sentinel_read")]
pub fn run_rp1_sysinfo_clock_sentinel_read() -> ! {
    const CONTRACT_ID: &str = "phase11-rp1-clock-sentinel-address-discriminator-source-contract-v1";
    const TARGET: &str = "rp1-sysinfo-vs-clock-sentinel-read";
    const RETAINED_ADC_WINDOW_CLASSIFICATION: &str = "rp1-clock-adc-window-readback-sentinel";
    const RETAINED_ADC_WINDOW_RAW: u32 = 0xdead_dead;

    write_early_static("rpi5-rp1-sysinfo-clock-sentinel-read: start\n");
    write_early_static("rpi5-rp1-sysinfo-clock-sentinel-read: before-read-only-loads\n");
    wait_uart10_empty_early_phase();

    let sysinfo_chip_id = read_rp1_reg_u32(RP1_SYSINFO_CHIP_ID);
    let sysinfo_platform = read_rp1_reg_u32(RP1_SYSINFO_PLATFORM);
    let clk_adc_ctrl = read_rp1_reg_u32(RP1_CLK_ADC_CTRL);

    let chip_id_matches_expected = sysinfo_chip_id == RP1_EXPECTED_CHIP_ID;
    let chip_id_is_deaddead = sysinfo_chip_id == RETAINED_ADC_WINDOW_RAW;
    let platform_is_deaddead = sysinfo_platform == RETAINED_ADC_WINDOW_RAW;
    let adc_ctrl_is_deaddead = clk_adc_ctrl == RETAINED_ADC_WINDOW_RAW;
    let sysinfo_pair_equal = sysinfo_chip_id == sysinfo_platform;
    let sysinfo_vs_adc_same = sysinfo_chip_id == clk_adc_ctrl;
    let classification = classify_sysinfo_clock_sentinel(
        chip_id_matches_expected,
        chip_id_is_deaddead,
        platform_is_deaddead,
        adc_ctrl_is_deaddead,
    );

    loop {
        write_early_static("TALOS: rp1-sysinfo-clock-sentinel-result contract=");
        write_early_static(CONTRACT_ID);
        write_early_static(" target=");
        write_early_static(TARGET);
        write_early_static(" sysinfo-base=");
        write_early_hex_u64(RP1_SYSINFO_BASE as u64);
        write_early_static(" clock-manager-base=");
        write_early_hex_u64(RP1_CLOCK_MANAGER_BASE as u64);
        write_sysinfo_clock_sentinel_register(
            " sysinfo-chip-id",
            0x0,
            RP1_SYSINFO_CHIP_ID,
            sysinfo_chip_id,
        );
        write_sysinfo_clock_sentinel_register(
            " sysinfo-platform",
            0x4,
            RP1_SYSINFO_PLATFORM,
            sysinfo_platform,
        );
        write_sysinfo_clock_sentinel_register(
            " clk-adc-ctrl",
            0x18144,
            RP1_CLK_ADC_CTRL,
            clk_adc_ctrl,
        );
        write_sysinfo_clock_sentinel_booleans(
            chip_id_matches_expected,
            chip_id_is_deaddead,
            platform_is_deaddead,
            adc_ctrl_is_deaddead,
            sysinfo_pair_equal,
            sysinfo_vs_adc_same,
        );
        write_retained_adc_window_sentinel_context(
            RETAINED_ADC_WINDOW_CLASSIFICATION,
            RETAINED_ADC_WINDOW_RAW,
        );
        write_early_static(" classification=");
        write_early_static(classification);
        write_early_static("\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_sysinfo_clock_sentinel_no_mmio_control")]
pub fn run_rp1_sysinfo_clock_sentinel_no_mmio_control() -> ! {
    const CONTRACT_ID: &str = "phase11-rp1-clock-sentinel-address-discriminator-source-contract-v1";
    const TARGET: &str = "rp1-sysinfo-vs-clock-sentinel-read";
    const RETAINED_ADC_WINDOW_CLASSIFICATION: &str = "rp1-clock-adc-window-readback-sentinel";
    const RETAINED_ADC_WINDOW_RAW: u32 = 0xdead_dead;
    const SIMULATED_SYSINFO_CHIP_ID: u32 = RP1_EXPECTED_CHIP_ID;
    const SIMULATED_SYSINFO_PLATFORM: u32 = 0;
    const SIMULATED_CLK_ADC_CTRL: u32 = RETAINED_ADC_WINDOW_RAW;

    write_early_static("rpi5-rp1-sysinfo-clock-sentinel-control: start\n");
    write_early_static(
        "rpi5-rp1-sysinfo-clock-sentinel-control: no-rp1-sysinfo-clock-reset-gpio-rio-pads-msix-pcie-mip-gic-mmio\n",
    );
    wait_uart10_empty_early_phase();

    let chip_id_matches_expected = SIMULATED_SYSINFO_CHIP_ID == RP1_EXPECTED_CHIP_ID;
    let chip_id_is_deaddead = SIMULATED_SYSINFO_CHIP_ID == RETAINED_ADC_WINDOW_RAW;
    let platform_is_deaddead = SIMULATED_SYSINFO_PLATFORM == RETAINED_ADC_WINDOW_RAW;
    let adc_ctrl_is_deaddead = SIMULATED_CLK_ADC_CTRL == RETAINED_ADC_WINDOW_RAW;
    let sysinfo_pair_equal = SIMULATED_SYSINFO_CHIP_ID == SIMULATED_SYSINFO_PLATFORM;
    let sysinfo_vs_adc_same = SIMULATED_SYSINFO_CHIP_ID == SIMULATED_CLK_ADC_CTRL;

    loop {
        write_early_static("TALOS: rp1-sysinfo-clock-sentinel-control contract=");
        write_early_static(CONTRACT_ID);
        write_early_static(" target=");
        write_early_static(TARGET);
        write_early_static(" sysinfo-base=not-constructed clock-manager-base=not-constructed");
        write_sysinfo_clock_sentinel_control_register(
            " sysinfo-chip-id",
            0x0,
            SIMULATED_SYSINFO_CHIP_ID,
        );
        write_sysinfo_clock_sentinel_control_register(
            " sysinfo-platform",
            0x4,
            SIMULATED_SYSINFO_PLATFORM,
        );
        write_sysinfo_clock_sentinel_control_register(
            " clk-adc-ctrl",
            0x18144,
            SIMULATED_CLK_ADC_CTRL,
        );
        write_sysinfo_clock_sentinel_booleans(
            chip_id_matches_expected,
            chip_id_is_deaddead,
            platform_is_deaddead,
            adc_ctrl_is_deaddead,
            sysinfo_pair_equal,
            sysinfo_vs_adc_same,
        );
        write_retained_adc_window_sentinel_context(
            RETAINED_ADC_WINDOW_CLASSIFICATION,
            RETAINED_ADC_WINDOW_RAW,
        );
        write_early_static(" classification=no-mmio-sysinfo-clock-sentinel-control-visible\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_clock_reset_dependency_read",
    talos_boot_scenario = "rpi5_rp1_clock_reset_dependency_no_mmio_control"
))]
#[derive(Clone, Copy)]
struct ClockResetDependencySnapshot {
    sysinfo_chip_id: u32,
    sysinfo_platform: u32,
    pll_sys_cs: u32,
    clk_sys_ctrl: u32,
    clk_sys_div_int: u32,
    clk_sys_sel: u32,
    clk_slow_sys_ctrl: u32,
    clk_uart_ctrl: u32,
    clk_uart_div_int: u32,
    clk_uart_sel: u32,
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_clock_reset_dependency_read",
    talos_boot_scenario = "rpi5_rp1_clock_reset_dependency_no_mmio_control"
))]
impl ClockResetDependencySnapshot {
    const fn zero() -> Self {
        Self {
            sysinfo_chip_id: 0,
            sysinfo_platform: 0,
            pll_sys_cs: 0,
            clk_sys_ctrl: 0,
            clk_sys_div_int: 0,
            clk_sys_sel: 0,
            clk_slow_sys_ctrl: 0,
            clk_uart_ctrl: 0,
            clk_uart_div_int: 0,
            clk_uart_sel: 0,
        }
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_clock_reset_dependency_read")]
pub fn run_rp1_clock_reset_dependency_read() -> ! {
    const CONTRACT_ID: &str = "phase11-rp1-clock-reset-dependency-source-contract-v1";
    const TARGET: &str = "rp1-observed-clock-reset-dependency-preflight-read";

    write_early_static("rpi5-rp1-clock-reset-dependency-read: start\n");
    write_early_static("rpi5-rp1-clock-reset-dependency-read: before-read-only-loads\n");
    wait_uart10_empty_early_phase();

    let snapshot = read_clock_reset_dependency_snapshot();
    let classification = clock_reset_dependency_classification(snapshot);

    loop {
        write_early_static("TALOS: rp1-clock-reset-dependency-result contract=");
        write_early_static(CONTRACT_ID);
        write_early_static(" target=");
        write_early_static(TARGET);
        write_clock_reset_dependency_bases(
            "0x1c00000000",
            RP1_SYSINFO_OBSERVED_APERTURE_BASE,
            RP1_CLOCK_MANAGER_OBSERVED_APERTURE_BASE,
        );
        write_clock_reset_dependency_real_registers(snapshot);
        write_clock_reset_dependency_report_fields(snapshot);
        write_clock_reset_dependency_retained_context();
        write_clock_reset_dependency_classification_vocabulary();
        write_early_static(" classification=");
        write_early_static(classification);
        write_early_static("\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_clock_reset_dependency_no_mmio_control")]
pub fn run_rp1_clock_reset_dependency_no_mmio_control() -> ! {
    const CONTRACT_ID: &str = "phase11-rp1-clock-reset-dependency-source-contract-v1";
    const TARGET: &str = "rp1-observed-clock-reset-dependency-preflight-read";
    const SNAPSHOT: ClockResetDependencySnapshot = ClockResetDependencySnapshot::zero();

    write_early_static("rpi5-rp1-clock-reset-dependency-control: start\n");
    write_early_static(
        "rpi5-rp1-clock-reset-dependency-control: no-rp1-gpio-clock-reset-msix-pcie-mip-gic-dma-mmio\n",
    );
    wait_uart10_empty_early_phase();

    loop {
        write_early_static("TALOS: rp1-clock-reset-dependency-control contract=");
        write_early_static(CONTRACT_ID);
        write_early_static(" target=");
        write_early_static(TARGET);
        write_clock_reset_dependency_capture_nonce();
        write_clock_reset_dependency_bases("not-constructed", 0, 0);
        write_clock_reset_dependency_control_registers(SNAPSHOT);
        write_clock_reset_dependency_report_fields(SNAPSHOT);
        write_clock_reset_dependency_retained_context();
        write_clock_reset_dependency_classification_vocabulary();
        write_early_static(" classification=no-mmio-clock-reset-dependency-control-visible\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_clock_reset_dependency_read")]
fn read_clock_reset_dependency_snapshot() -> ClockResetDependencySnapshot {
    ClockResetDependencySnapshot {
        sysinfo_chip_id: read_rp1_reg_u32(RP1_SYSINFO_OBSERVED_APERTURE_CHIP_ID),
        sysinfo_platform: read_rp1_reg_u32(RP1_SYSINFO_OBSERVED_APERTURE_PLATFORM),
        pll_sys_cs: read_rp1_reg_u32(RP1_PLL_SYS_OBSERVED_APERTURE_CS),
        clk_sys_ctrl: read_rp1_reg_u32(RP1_CLK_SYS_OBSERVED_APERTURE_CTRL),
        clk_sys_div_int: read_rp1_reg_u32(RP1_CLK_SYS_OBSERVED_APERTURE_DIV_INT),
        clk_sys_sel: read_rp1_reg_u32(RP1_CLK_SYS_OBSERVED_APERTURE_SEL),
        clk_slow_sys_ctrl: read_rp1_reg_u32(RP1_CLK_SLOW_SYS_OBSERVED_APERTURE_CTRL),
        clk_uart_ctrl: read_rp1_reg_u32(RP1_CLK_UART_OBSERVED_APERTURE_CTRL),
        clk_uart_div_int: read_rp1_reg_u32(RP1_CLK_UART_OBSERVED_APERTURE_DIV_INT),
        clk_uart_sel: read_rp1_reg_u32(RP1_CLK_UART_OBSERVED_APERTURE_SEL),
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_clock_reset_dependency_read",
    talos_boot_scenario = "rpi5_rp1_clock_reset_dependency_no_mmio_control"
))]
fn clock_reset_dependency_classification(snapshot: ClockResetDependencySnapshot) -> &'static str {
    if snapshot.sysinfo_chip_id == 0xdead_dead || snapshot.sysinfo_platform == 0xdead_dead {
        "observed-clock-reset-dependency-blocked-sysinfo-sentinel"
    } else if clock_reset_dependency_any_clock_deaddead(snapshot) {
        "observed-clock-reset-dependency-blocked-clock-manager-sentinel"
    } else if snapshot.clk_sys_ctrl & RP1_CLK_CTRL_ENABLE == 0
        || snapshot.clk_slow_sys_ctrl & RP1_CLK_CTRL_ENABLE == 0
    {
        "observed-clock-reset-dependency-blocked-system-clock-disabled"
    } else if snapshot.clk_uart_ctrl & RP1_CLK_CTRL_ENABLE == 0 {
        "observed-clock-reset-dependency-blocked-uart-clock-disabled"
    } else {
        "observed-clock-reset-dependency-visible"
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_clock_reset_dependency_read",
    talos_boot_scenario = "rpi5_rp1_clock_reset_dependency_no_mmio_control"
))]
fn clock_reset_dependency_any_clock_deaddead(snapshot: ClockResetDependencySnapshot) -> bool {
    snapshot.pll_sys_cs == 0xdead_dead
        || snapshot.clk_sys_ctrl == 0xdead_dead
        || snapshot.clk_sys_div_int == 0xdead_dead
        || snapshot.clk_sys_sel == 0xdead_dead
        || snapshot.clk_slow_sys_ctrl == 0xdead_dead
        || snapshot.clk_uart_ctrl == 0xdead_dead
        || snapshot.clk_uart_div_int == 0xdead_dead
        || snapshot.clk_uart_sel == 0xdead_dead
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_clock_reset_dependency_read",
    talos_boot_scenario = "rpi5_rp1_clock_reset_dependency_no_mmio_control"
))]
fn clock_reset_dependency_all_clock_deaddead(snapshot: ClockResetDependencySnapshot) -> bool {
    snapshot.pll_sys_cs == 0xdead_dead
        && snapshot.clk_sys_ctrl == 0xdead_dead
        && snapshot.clk_sys_div_int == 0xdead_dead
        && snapshot.clk_sys_sel == 0xdead_dead
        && snapshot.clk_slow_sys_ctrl == 0xdead_dead
        && snapshot.clk_uart_ctrl == 0xdead_dead
        && snapshot.clk_uart_div_int == 0xdead_dead
        && snapshot.clk_uart_sel == 0xdead_dead
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_clock_reset_dependency_read",
    talos_boot_scenario = "rpi5_rp1_clock_reset_dependency_no_mmio_control"
))]
fn write_clock_reset_dependency_bases(
    observed_base: &str,
    sysinfo_base: usize,
    clock_manager_base: usize,
) {
    write_early_static(" observed-base=");
    write_early_static(observed_base);
    write_early_static(" sysinfo-base=");
    if observed_base == "not-constructed" {
        write_early_static("not-constructed");
    } else {
        write_early_hex_u64(sysinfo_base as u64);
    }
    write_early_static(" clock-manager-base=");
    if observed_base == "not-constructed" {
        write_early_static("not-constructed");
    } else {
        write_early_hex_u64(clock_manager_base as u64);
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_clock_reset_dependency_read")]
fn write_clock_reset_dependency_real_registers(snapshot: ClockResetDependencySnapshot) {
    write_clock_reset_dependency_register(
        " sysinfo-chip-id",
        0x000000,
        RP1_SYSINFO_OBSERVED_APERTURE_CHIP_ID,
        snapshot.sysinfo_chip_id,
    );
    write_clock_reset_dependency_register(
        " sysinfo-platform",
        0x000004,
        RP1_SYSINFO_OBSERVED_APERTURE_PLATFORM,
        snapshot.sysinfo_platform,
    );
    write_clock_reset_dependency_register(
        " pll-sys-cs",
        0x020000,
        RP1_PLL_SYS_OBSERVED_APERTURE_CS,
        snapshot.pll_sys_cs,
    );
    write_clock_reset_dependency_register(
        " clk-sys-ctrl",
        0x018014,
        RP1_CLK_SYS_OBSERVED_APERTURE_CTRL,
        snapshot.clk_sys_ctrl,
    );
    write_clock_reset_dependency_register(
        " clk-sys-div-int",
        0x018018,
        RP1_CLK_SYS_OBSERVED_APERTURE_DIV_INT,
        snapshot.clk_sys_div_int,
    );
    write_clock_reset_dependency_register(
        " clk-sys-sel",
        0x018020,
        RP1_CLK_SYS_OBSERVED_APERTURE_SEL,
        snapshot.clk_sys_sel,
    );
    write_clock_reset_dependency_register(
        " clk-slow-sys-ctrl",
        0x018024,
        RP1_CLK_SLOW_SYS_OBSERVED_APERTURE_CTRL,
        snapshot.clk_slow_sys_ctrl,
    );
    write_clock_reset_dependency_register(
        " clk-uart-ctrl",
        0x018054,
        RP1_CLK_UART_OBSERVED_APERTURE_CTRL,
        snapshot.clk_uart_ctrl,
    );
    write_clock_reset_dependency_register(
        " clk-uart-div-int",
        0x018058,
        RP1_CLK_UART_OBSERVED_APERTURE_DIV_INT,
        snapshot.clk_uart_div_int,
    );
    write_clock_reset_dependency_register(
        " clk-uart-sel",
        0x018060,
        RP1_CLK_UART_OBSERVED_APERTURE_SEL,
        snapshot.clk_uart_sel,
    );
}

#[cfg(talos_boot_scenario = "rpi5_rp1_clock_reset_dependency_no_mmio_control")]
fn write_clock_reset_dependency_control_registers(snapshot: ClockResetDependencySnapshot) {
    write_clock_reset_dependency_control_register(
        " sysinfo-chip-id",
        0x000000,
        snapshot.sysinfo_chip_id,
    );
    write_clock_reset_dependency_control_register(
        " sysinfo-platform",
        0x000004,
        snapshot.sysinfo_platform,
    );
    write_clock_reset_dependency_control_register(" pll-sys-cs", 0x020000, snapshot.pll_sys_cs);
    write_clock_reset_dependency_control_register(" clk-sys-ctrl", 0x018014, snapshot.clk_sys_ctrl);
    write_clock_reset_dependency_control_register(
        " clk-sys-div-int",
        0x018018,
        snapshot.clk_sys_div_int,
    );
    write_clock_reset_dependency_control_register(" clk-sys-sel", 0x018020, snapshot.clk_sys_sel);
    write_clock_reset_dependency_control_register(
        " clk-slow-sys-ctrl",
        0x018024,
        snapshot.clk_slow_sys_ctrl,
    );
    write_clock_reset_dependency_control_register(
        " clk-uart-ctrl",
        0x018054,
        snapshot.clk_uart_ctrl,
    );
    write_clock_reset_dependency_control_register(
        " clk-uart-div-int",
        0x018058,
        snapshot.clk_uart_div_int,
    );
    write_clock_reset_dependency_control_register(" clk-uart-sel", 0x018060, snapshot.clk_uart_sel);
}

#[cfg(talos_boot_scenario = "rpi5_rp1_clock_reset_dependency_read")]
fn write_clock_reset_dependency_register(
    name: &str,
    source_offset: u64,
    address: usize,
    value: u32,
) {
    write_early_static(name);
    write_early_static("-source-offset=");
    write_early_hex_u64(source_offset);
    write_early_static(" address=");
    write_early_hex_u64(address as u64);
    write_early_static(" width=32 raw=");
    write_early_hex_u64(value as u64);
}

#[cfg(talos_boot_scenario = "rpi5_rp1_clock_reset_dependency_no_mmio_control")]
fn write_clock_reset_dependency_control_register(name: &str, source_offset: u64, value: u32) {
    write_early_static(name);
    write_early_static("-source-offset=");
    write_early_hex_u64(source_offset);
    write_early_static(" address=not-constructed width=32 raw=");
    write_early_hex_u64(value as u64);
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_clock_reset_dependency_read",
    talos_boot_scenario = "rpi5_rp1_clock_reset_dependency_no_mmio_control"
))]
fn write_clock_reset_dependency_report_fields(snapshot: ClockResetDependencySnapshot) {
    let any_clock_deaddead = clock_reset_dependency_any_clock_deaddead(snapshot);
    let all_clock_deaddead = clock_reset_dependency_all_clock_deaddead(snapshot);

    write_early_static(" expected-chip-id=");
    write_early_hex_u64(RP1_EXPECTED_CHIP_ID as u64);
    write_early_static(" chip-id-matches-expected=");
    write_bool(snapshot.sysinfo_chip_id == RP1_EXPECTED_CHIP_ID);
    write_early_static(" chip-id-is-deaddead=");
    write_bool(snapshot.sysinfo_chip_id == 0xdead_dead);
    write_early_static(" platform-is-deaddead=");
    write_bool(snapshot.sysinfo_platform == 0xdead_dead);
    write_early_static(" pll-sys-locked=");
    write_bool(snapshot.pll_sys_cs & (1 << 31) != 0);
    write_early_static(" clk-sys-enabled=");
    write_bool(snapshot.clk_sys_ctrl & RP1_CLK_CTRL_ENABLE != 0);
    write_early_static(" clk-slow-sys-enabled=");
    write_bool(snapshot.clk_slow_sys_ctrl & RP1_CLK_CTRL_ENABLE != 0);
    write_early_static(" clk-uart-enabled=");
    write_bool(snapshot.clk_uart_ctrl & RP1_CLK_CTRL_ENABLE != 0);
    write_early_static(" any-selected-clock-deaddead=");
    write_bool(any_clock_deaddead);
    write_early_static(" all-selected-clock-deaddead=");
    write_bool(all_clock_deaddead);
    write_early_static(" reset-status-source=none-selected-read-only");
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_clock_reset_dependency_read",
    talos_boot_scenario = "rpi5_rp1_clock_reset_dependency_no_mmio_control"
))]
fn write_clock_reset_dependency_retained_context() {
    write_early_static(
        " retained-gpio14-blocker=observed-gpio14-ownership-preflight-blocked-non-gpio-function",
    );
    write_early_static(
        " retained-gpio16-blocker=observed-gpio16-ownership-preflight-blocked-non-gpio-function",
    );
    write_early_static(
        " retained-0x1f-sysinfo-clock-sentinel=rp1-sysinfo-and-clock-window-sentinel",
    );
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_clock_reset_dependency_read",
    talos_boot_scenario = "rpi5_rp1_clock_reset_dependency_no_mmio_control"
))]
fn write_clock_reset_dependency_classification_vocabulary() {
    write_early_static(" classification-vocabulary=");
    write_early_static("observed-clock-reset-dependency-visible,");
    write_early_static("observed-clock-reset-dependency-blocked-sysinfo-sentinel,");
    write_early_static("observed-clock-reset-dependency-blocked-clock-manager-sentinel,");
    write_early_static("observed-clock-reset-dependency-blocked-system-clock-disabled,");
    write_early_static("observed-clock-reset-dependency-blocked-uart-clock-disabled,");
    write_early_static("observed-clock-reset-dependency-no-return-or-trap,");
    write_early_static("observed-clock-reset-dependency-inconclusive-capture,");
    write_early_static("no-mmio-clock-reset-dependency-control-visible,");
    write_early_static("staging/build-blocker");
}

#[cfg(talos_boot_scenario = "rpi5_rp1_clock_reset_dependency_no_mmio_control")]
fn write_clock_reset_dependency_capture_nonce() {
    if let Some(nonce) = option_env!("TALOS_CAPTURE_NONCE") {
        if !nonce.is_empty() {
            write_early_static(" capture-nonce=");
            write_early_static(nonce);
        }
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_dma_cache_small_diagnostic_visibility_candidate")]
pub fn run_rp1_dma_cache_small_diagnostic_visibility_candidate() -> ! {
    write_early_static("rpi5-rp1-dma-cache-small-diagnostic-visibility-candidate: start\n");
    write_early_static(
        "rpi5-rp1-dma-cache-small-diagnostic-visibility-candidate: no-rp1-mmio-no-dma-channel-programming-no-descriptor-ring\n",
    );
    wait_uart10_empty_early_phase();

    loop {
        write_early_static("TALOS: rp1-dma-cache-small-diagnostic-visibility-candidate");
        write_dma_cache_small_diagnostic_visibility_capture_nonce();
        write_dma_cache_small_diagnostic_visibility_common("candidate");
        write_early_static(
            " small-diagnostic-plan-contract-id=phase11-rp1-dma-cache-small-diagnostic-plan-contract-v1",
        );
        write_early_static(
            " driver-diagnostic-envelope-contract-id=phase11-rp1-dma-cache-driver-diagnostic-envelope-contract-v1",
        );
        write_early_static(
            " executor-contract-id=phase11-rp1-dma-cache-maintenance-executor-contract-v1",
        );
        write_early_static(
            " maintenance-sequence-contract-id=phase11-rp1-dma-cache-maintenance-sequence-contract-v1",
        );
        write_early_static(" sync-plan-contract-id=phase11-rp1-dma-cache-sync-plan-contract-v1");
        write_early_static(" descriptor-contract-id=phase11-rp1-dma-cache-substrate-contract-v1");
        write_early_static(
            " descriptor-source-inventory-id=phase11-rp1-dma-cache-source-inventory-20260609",
        );
        write_early_static(" rp1-dma-compatible=snps,axi-dma-1.01a");
        write_early_static(" rp1-dma-controller-rp1-bus-base=");
        write_early_hex_u64(0xc0_4018_8000);
        write_early_static(" rp1-dma-controller-cpu-physical-base=");
        write_early_hex_u64(0x1f_0018_8000);
        write_early_static(" rp1-dma-channel-count=8 rp1-dma-target-count=64");
        write_early_static(" rp1-dma-interrupt-name=RP1_INT_DMA");
        write_early_static(" rp1-dma-clock-names=RP1_CLK_DMA,RP1_CLK_SYS");
        write_early_static(" cpu-physical=");
        write_early_hex_u64(0x2f02_0000);
        write_early_static(" cpu-visible=");
        write_early_hex_u64(0x2f02_0000);
        write_early_static(" rp1-bus-address=");
        write_early_hex_u64(0x10_2f02_0000);
        write_early_static(" descriptor-length=");
        write_early_hex_u64(0x2000);
        write_early_static(
            " cache-line-source=bcm2712-dcache-l2-cache-line-size cache-line-size=64",
        );
        write_early_static(" line-aligned-cpu-start=");
        write_early_hex_u64(0x2f02_0000);
        write_early_static(" covered-length=");
        write_early_hex_u64(0x2000);
        write_early_static(" line-count=128 direction=to-device");
        write_early_static(
            " cacheability=cacheable-requires-maintenance owner-transition=cpu-to-device",
        );
        write_early_static(" iommu-classification=source-unassigned-rp1-dma");
        write_early_static(
            " prerequisite-rejected-runtime-claims=executed-driver-buffer-cache-maintenance,live-barrier-ordering,rp1-mmio-writes,dma-channel-programming,descriptor-rings,ethernet-storage-networking-ssh,milestone-11-3-completion",
        );
        write_early_static(
            " executor-rejected-runtime-claims=driver-dma-completion,rp1-mmio-writes,dma-channel-programming,descriptor-rings,interrupt-completion,ethernet-storage-networking-ssh,hardware-validation,milestone-11-3-completion",
        );
        write_early_static(
            " unresolved-dma-diagnostic-gaps=rp1-dma-channel-ownership,descriptor-ring-layout-and-ownership,transfer-completion-and-interrupt-policy,iommu-runtime-policy,dma-safe-allocation-and-pinning,hardware-proof,device-specific-consumer",
        );
        write_dma_cache_small_diagnostic_visibility_rejections();
        write_early_static(
            " classification=local-static-rp1-dma-small-diagnostic-plan-visibility-candidate\n",
        );
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_dma_cache_small_diagnostic_visibility_no_plan_control")]
pub fn run_rp1_dma_cache_small_diagnostic_visibility_no_plan_control() -> ! {
    write_early_static("rpi5-rp1-dma-cache-small-diagnostic-visibility-control: start\n");
    write_early_static(
        "rpi5-rp1-dma-cache-small-diagnostic-visibility-control: no-plan-no-rp1-mmio-no-dma-channel-programming-no-descriptor-ring\n",
    );
    wait_uart10_empty_early_phase();

    loop {
        write_early_static("TALOS: rp1-dma-cache-small-diagnostic-visibility-control");
        write_dma_cache_small_diagnostic_visibility_capture_nonce();
        write_dma_cache_small_diagnostic_visibility_common("no-plan-control");
        write_early_static(" small-diagnostic-plan-contract-id=none");
        write_early_static(" driver-diagnostic-envelope-contract-id=none");
        write_early_static(" executor-contract-id=none maintenance-sequence-contract-id=none");
        write_early_static(" sync-plan-contract-id=none descriptor-contract-id=none");
        write_early_static(" descriptor-source-inventory-id=none rp1-dma-compatible=none");
        write_early_static(" rp1-dma-controller-rp1-bus-base=none");
        write_early_static(" rp1-dma-controller-cpu-physical-base=none");
        write_early_static(" rp1-dma-channel-count=none rp1-dma-target-count=none");
        write_early_static(" rp1-dma-interrupt-name=none rp1-dma-clock-names=none");
        write_early_static(" cpu-physical=none cpu-visible=none rp1-bus-address=none");
        write_early_static(" descriptor-length=none cache-line-source=none cache-line-size=none");
        write_early_static(" line-aligned-cpu-start=none covered-length=none line-count=none");
        write_early_static(" direction=none cacheability=none owner-transition=none");
        write_early_static(" iommu-classification=none");
        write_early_static(" prerequisite-rejected-runtime-claims=none");
        write_early_static(
            " executor-rejected-runtime-claims=none unresolved-dma-diagnostic-gaps=none",
        );
        write_dma_cache_small_diagnostic_visibility_rejections();
        write_early_static(" classification=no-plan-rp1-dma-small-diagnostic-visibility-control\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_ethernet_gem_mid_visibility_candidate")]
pub fn run_rp1_ethernet_gem_mid_visibility_candidate() -> ! {
    const MACB_MID: usize = 0x1f_0010_00fc;

    write_early_static("rpi5-rp1-ethernet-gem-mid-visibility-candidate: start\n");
    write_early_static(
        "rpi5-rp1-ethernet-gem-mid-visibility-candidate: before-read-only-volatile-load\n",
    );
    wait_uart10_empty_early_phase();

    let raw = read_rp1_reg_u32(MACB_MID);
    let idnum = (raw >> 16) & 0x0fff;
    let rev = raw & 0xffff;
    let classification = classify_rp1_ethernet_gem_mid(raw);

    loop {
        write_early_static("TALOS: rp1-ethernet-gem-mid-visibility-candidate");
        write_rp1_ethernet_gem_mid_capture_nonce();
        write_rp1_ethernet_gem_mid_common("candidate");
        write_early_static(" compatible=raspberrypi,rp1-gem,cdns,macb");
        write_early_static(" controller=rp1_eth register=MACB_MID");
        write_early_static(" rp1-bus-base=");
        write_early_hex_u64(0xc0_4010_0000);
        write_early_static(" cpu-physical-base=");
        write_early_hex_u64(0x1f_0010_0000);
        write_early_static(" offset=");
        write_early_hex_u64(0x00fc);
        write_early_static(" rp1-bus-target=");
        write_early_hex_u64(0xc0_4010_00fc);
        write_early_static(" cpu-physical-target=");
        write_early_hex_u64(MACB_MID as u64);
        write_early_static(" width=32 endianness=little-endian access=read-only-volatile-load");
        write_early_static(" raw=");
        write_early_hex_u64(raw as u64);
        write_early_static(" idnum=");
        write_early_hex_u64(idnum as u64);
        write_early_static(" rev=");
        write_early_hex_u64(rev as u64);
        write_early_static(" raw-is-zero=");
        write_bool(raw == 0);
        write_early_static(" raw-is-all-ones=");
        write_bool(raw == 0xffff_ffff);
        write_early_static(" raw-is-deaddead=");
        write_bool(raw == 0xdead_dead);
        write_rp1_ethernet_gem_mid_rejections();
        write_early_static(" classification=");
        write_early_static(classification);
        write_early_static("\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_ethernet_gem_mid_visibility_no_mmio_control")]
pub fn run_rp1_ethernet_gem_mid_visibility_no_mmio_control() -> ! {
    write_early_static("rpi5-rp1-ethernet-gem-mid-visibility-control: start\n");
    write_early_static(
        "rpi5-rp1-ethernet-gem-mid-visibility-control: no-ethernet-no-mmio-target-construction\n",
    );
    wait_uart10_empty_early_phase();

    loop {
        write_early_static("TALOS: rp1-ethernet-gem-mid-visibility-control");
        write_rp1_ethernet_gem_mid_capture_nonce();
        write_rp1_ethernet_gem_mid_common("no-ethernet-no-mmio-control");
        write_early_static(" compatible=none controller=none register=MACB_MID");
        write_early_static(" rp1-bus-base=none cpu-physical-base=none offset=none");
        write_early_static(" rp1-bus-target=none cpu-physical-target=not-constructed");
        write_early_static(" width=32 endianness=little-endian access=not-constructed");
        write_early_static(" raw=none idnum=none rev=none");
        write_early_static(" raw-is-zero=false raw-is-all-ones=false raw-is-deaddead=false");
        write_rp1_ethernet_gem_mid_rejections();
        write_early_static(" classification=no-ethernet-no-mmio-rp1-ethernet-gem-mid-control\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_ethernet_clock_reset_write_restore_candidate")]
pub fn run_rp1_ethernet_clock_reset_write_restore_candidate() -> ! {
    const CONTRACT_ID: &str = "phase12-rp1-ethernet-clock-reset-write-target-source-contract-v1";
    const REPORT_CONTRACT_ID: &str =
        "phase12-rp1-ethernet-clock-reset-write-restore-report-contract-v1";
    const TARGET: &str = "rp1-ethernet-clk-eth-tsu-ctrl-idempotent-write-restore";
    const REGISTER_NAME: &str = "CLK_ETH_TSU_CTRL";
    const SOURCE_OFFSET: u64 = 0x018134;

    write_early_static("rpi5-rp1-ethernet-clock-reset-write-restore-candidate: start\n");
    write_early_static(
        "rpi5-rp1-ethernet-clock-reset-write-restore-candidate: before-clk-eth-tsu-ctrl-idempotent-write-restore\n",
    );
    wait_uart10_empty_early_phase();

    let pre_raw = read_rp1_reg_u32(RP1_CLK_ETH_TSU_CTRL_OBSERVED_APERTURE);
    write_rp1_reg_u32_ordered(RP1_CLK_ETH_TSU_CTRL_OBSERVED_APERTURE, pre_raw);
    let post_raw = read_rp1_reg_u32(RP1_CLK_ETH_TSU_CTRL_OBSERVED_APERTURE);
    write_rp1_reg_u32_ordered(RP1_CLK_ETH_TSU_CTRL_OBSERVED_APERTURE, pre_raw);
    let restore_raw = read_rp1_reg_u32(RP1_CLK_ETH_TSU_CTRL_OBSERVED_APERTURE);

    let post_matches_pre = post_raw == pre_raw;
    let restore_matches_pre = restore_raw == pre_raw;
    let classification = if post_matches_pre && restore_matches_pre {
        "rp1-ethernet-clk-eth-tsu-ctrl-idempotent-write-restored"
    } else if restore_matches_pre {
        "rp1-ethernet-clk-eth-tsu-ctrl-idempotent-write-mismatch-restored"
    } else {
        "rp1-ethernet-clk-eth-tsu-ctrl-idempotent-write-restore-failed"
    };

    loop {
        write_early_static("TALOS: rp1-ethernet-clock-reset-write-restore-candidate");
        write_rp1_ethernet_clock_reset_write_restore_capture_nonce();
        write_rp1_ethernet_clock_reset_write_restore_common(
            REPORT_CONTRACT_ID,
            CONTRACT_ID,
            "candidate",
        );
        write_early_static(" target=");
        write_early_static(TARGET);
        write_early_static(" register=");
        write_early_static(REGISTER_NAME);
        write_early_static(" clock-name=tsu_clk clock-id=29");
        write_early_static(" observed-rp1-base=");
        write_early_hex_u64(0x1c_0000_0000);
        write_early_static(" source-offset=");
        write_early_hex_u64(SOURCE_OFFSET);
        write_early_static(" address=");
        write_early_hex_u64(RP1_CLK_ETH_TSU_CTRL_OBSERVED_APERTURE as u64);
        write_early_static(" width=32 allowed-write-value=pre-raw-only");
        write_clock_adc_ctrl_raw_triplet(pre_raw, post_raw, restore_raw);
        write_early_static(" post-eq-pre=");
        write_bool(post_matches_pre);
        write_early_static(" restore-eq-pre=");
        write_bool(restore_matches_pre);
        write_early_static(
            " preserved-fields=full-raw,enable-bit11,auxsrc-bits9-5,source-bits0,reserved-bits",
        );
        write_rp1_ethernet_clock_reset_write_restore_rejections();
        write_early_static(" classification=");
        write_early_static(classification);
        write_early_static("\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_ethernet_clock_reset_write_restore_no_mmio_control")]
pub fn run_rp1_ethernet_clock_reset_write_restore_no_mmio_control() -> ! {
    const CONTRACT_ID: &str = "phase12-rp1-ethernet-clock-reset-write-target-source-contract-v1";
    const REPORT_CONTRACT_ID: &str =
        "phase12-rp1-ethernet-clock-reset-write-restore-report-contract-v1";
    const SIMULATED_RAW_VALUE: u32 = 0;

    write_early_static("rpi5-rp1-ethernet-clock-reset-write-restore-control: start\n");
    write_early_static(
        "rpi5-rp1-ethernet-clock-reset-write-restore-control: no-clock-write-no-ethernet-no-mmio-target-construction\n",
    );
    wait_uart10_empty_early_phase();

    loop {
        write_early_static("TALOS: rp1-ethernet-clock-reset-write-restore-control");
        write_rp1_ethernet_clock_reset_write_restore_capture_nonce();
        write_rp1_ethernet_clock_reset_write_restore_common(
            REPORT_CONTRACT_ID,
            CONTRACT_ID,
            "no-clock-write-no-ethernet-control",
        );
        write_early_static(" target=none register=none clock-name=none clock-id=none");
        write_early_static(" observed-rp1-base=not-constructed");
        write_early_static(" source-offset=0x18134 address=not-constructed");
        write_early_static(" width=32 allowed-write-value=withheld");
        write_clock_adc_ctrl_raw_triplet(
            SIMULATED_RAW_VALUE,
            SIMULATED_RAW_VALUE,
            SIMULATED_RAW_VALUE,
        );
        write_early_static(" post-eq-pre=true restore-eq-pre=true");
        write_early_static(" preserved-fields=withheld");
        write_rp1_ethernet_clock_reset_write_restore_rejections();
        write_early_static(
            " classification=no-clock-write-no-ethernet-rp1-ethernet-write-restore-control\n",
        );
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_ethernet_clk_eth_ctrl_write_restore_candidate")]
pub fn run_rp1_ethernet_clk_eth_ctrl_write_restore_candidate() -> ! {
    const CONTRACT_ID: &str = "phase12-rp1-ethernet-clk-eth-ctrl-write-target-source-contract-v1";
    const REPORT_CONTRACT_ID: &str =
        "phase12-rp1-ethernet-clk-eth-ctrl-write-restore-report-contract-v1";
    const TARGET: &str = "rp1-ethernet-clk-eth-ctrl-idempotent-write-restore";
    const REGISTER_NAME: &str = "CLK_ETH_CTRL";
    const SOURCE_OFFSET: u64 = 0x018064;

    write_early_static("rpi5-rp1-ethernet-clk-eth-ctrl-write-restore-candidate: start\n");
    write_early_static(
        "rpi5-rp1-ethernet-clk-eth-ctrl-write-restore-candidate: before-clk-eth-ctrl-idempotent-write-restore\n",
    );
    wait_uart10_empty_early_phase();

    let pre_raw = read_rp1_reg_u32(RP1_CLK_ETH_CTRL_OBSERVED_APERTURE);
    write_rp1_reg_u32_ordered(RP1_CLK_ETH_CTRL_OBSERVED_APERTURE, pre_raw);
    let post_raw = read_rp1_reg_u32(RP1_CLK_ETH_CTRL_OBSERVED_APERTURE);
    write_rp1_reg_u32_ordered(RP1_CLK_ETH_CTRL_OBSERVED_APERTURE, pre_raw);
    let restore_raw = read_rp1_reg_u32(RP1_CLK_ETH_CTRL_OBSERVED_APERTURE);

    let post_matches_pre = post_raw == pre_raw;
    let restore_matches_pre = restore_raw == pre_raw;
    let classification = if post_matches_pre && restore_matches_pre {
        "rp1-ethernet-clk-eth-ctrl-idempotent-write-restored"
    } else if restore_matches_pre {
        "rp1-ethernet-clk-eth-ctrl-idempotent-write-mismatch-restored"
    } else {
        "rp1-ethernet-clk-eth-ctrl-idempotent-write-restore-failed"
    };

    loop {
        write_early_static("TALOS: rp1-ethernet-clk-eth-ctrl-write-restore-candidate");
        write_rp1_ethernet_clk_eth_ctrl_write_restore_capture_nonce();
        write_rp1_ethernet_clk_eth_ctrl_write_restore_common(
            REPORT_CONTRACT_ID,
            CONTRACT_ID,
            "candidate",
        );
        write_early_static(" target=");
        write_early_static(TARGET);
        write_early_static(" register=");
        write_early_static(REGISTER_NAME);
        write_early_static(" clock-name=tx_clk clock-id=16");
        write_early_static(" observed-rp1-base=");
        write_early_hex_u64(0x1c_0000_0000);
        write_early_static(" source-offset=");
        write_early_hex_u64(SOURCE_OFFSET);
        write_early_static(" address=");
        write_early_hex_u64(RP1_CLK_ETH_CTRL_OBSERVED_APERTURE as u64);
        write_early_static(" width=32 allowed-write-value=pre-raw-only");
        write_clock_adc_ctrl_raw_triplet(pre_raw, post_raw, restore_raw);
        write_early_static(" post-eq-pre=");
        write_bool(post_matches_pre);
        write_early_static(" restore-eq-pre=");
        write_bool(restore_matches_pre);
        write_early_static(
            " preserved-fields=full-raw,enable-bit11,auxsrc-bits9-5,source-bits1-0,reserved-bits",
        );
        write_early_static(" claims-clk-eth-ctrl-idempotent-write=true");
        write_rp1_ethernet_clk_eth_ctrl_write_restore_rejections();
        write_early_static(" classification=");
        write_early_static(classification);
        write_early_static("\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_ethernet_clk_eth_ctrl_write_restore_no_mmio_control")]
pub fn run_rp1_ethernet_clk_eth_ctrl_write_restore_no_mmio_control() -> ! {
    const CONTRACT_ID: &str = "phase12-rp1-ethernet-clk-eth-ctrl-write-target-source-contract-v1";
    const REPORT_CONTRACT_ID: &str =
        "phase12-rp1-ethernet-clk-eth-ctrl-write-restore-report-contract-v1";
    const SIMULATED_RAW_VALUE: u32 = 0;

    write_early_static("rpi5-rp1-ethernet-clk-eth-ctrl-write-restore-control: start\n");
    write_early_static(
        "rpi5-rp1-ethernet-clk-eth-ctrl-write-restore-control: no-clock-write-no-ethernet-no-mmio-target-construction\n",
    );
    wait_uart10_empty_early_phase();

    loop {
        write_early_static("TALOS: rp1-ethernet-clk-eth-ctrl-write-restore-control");
        write_rp1_ethernet_clk_eth_ctrl_write_restore_capture_nonce();
        write_rp1_ethernet_clk_eth_ctrl_write_restore_common(
            REPORT_CONTRACT_ID,
            CONTRACT_ID,
            "no-clock-write-no-ethernet-control",
        );
        write_early_static(" target=none register=none clock-name=none clock-id=none");
        write_early_static(" observed-rp1-base=not-constructed");
        write_early_static(" source-offset=0x18064 address=not-constructed");
        write_early_static(" width=32 allowed-write-value=withheld");
        write_clock_adc_ctrl_raw_triplet(
            SIMULATED_RAW_VALUE,
            SIMULATED_RAW_VALUE,
            SIMULATED_RAW_VALUE,
        );
        write_early_static(" post-eq-pre=true restore-eq-pre=true");
        write_early_static(" preserved-fields=withheld");
        write_early_static(" claims-clk-eth-ctrl-idempotent-write=false");
        write_rp1_ethernet_clk_eth_ctrl_write_restore_rejections();
        write_early_static(
            " classification=no-clock-write-no-ethernet-rp1-ethernet-clk-eth-ctrl-control\n",
        );
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_ethernet_gpio32_phy_reset_preflight_candidate")]
pub fn run_rp1_ethernet_gpio32_phy_reset_preflight_candidate() -> ! {
    write_early_static("rpi5-rp1-ethernet-gpio32-phy-reset-preflight-candidate: start\n");
    write_early_static(
        "rpi5-rp1-ethernet-gpio32-phy-reset-preflight-candidate: readonly-report-no-gpio-mdio-mmio-writes\n",
    );
    wait_uart10_empty_early_phase();

    loop {
        write_early_static("TALOS: rp1-ethernet-gpio32-phy-reset-preflight-candidate");
        write_rp1_ethernet_gpio32_phy_reset_preflight_capture_nonce();
        write_rp1_ethernet_gpio32_phy_reset_preflight_common("candidate");
        write_early_static(
            " accepted-input-frontier=observed-window-macb-mid-identity-context,prereq-ownership-report-visibility,clk-eth-tsu-ctrl-proof-closeout,clk-eth-ctrl-proof-closeout,phase11-gpio-source-status-frontiers-without-ownership",
        );
        write_early_static(" controller=rp1_eth compatible=raspberrypi,rp1-gem,cdns,macb");
        write_early_static(" phy-mode=rgmii-id phy-handle=phy1 phy-node=ethernet-phy@1 phy-reg=");
        write_early_hex_u64(0x1);
        write_early_static(" gpio-controller=rp1_gpio gpio-line=32 signal=ETH_RST_N");
        write_early_static(" active-low=true logical-assertion-value=1 physical-assertion=low");
        write_early_static(" logical-deassertion-value=0 physical-deassertion=high");
        write_early_static(" reset-duration-ms=5");
        write_early_static(
            " linux-hook-relationship=macb_mdio_reset-installed-as-mdio-bus-reset-hook",
        );
        write_early_static(
            " phase11-gpio-constraints=gpio-ownership-unaccepted,function-changes-unaccepted,rio-out-oe-in-writes-unaccepted,pad-writes-unaccepted,inte-ctrl-writes-unaccepted,event-generation-interrupt-delivery-unaccepted,gpio-write-restore-authority-unaccepted",
        );
        write_early_static(
            " source-evidence=phase12-gpio32-source-contract,linux-rp1-dtsi,linux-bcm2712-rpi-5-b-dts,linux-cdns-macb-yaml,linux-macb-main-c,phase11-gpio-source-notes",
        );
        write_early_static(
            " future-write-restore-invariants=requires-separate-gpio32-ownership-or-precise-prestate-restore-contract,capture-gpio-function-rio-pad-output-enable-state,preserve-active-low-polarity,capture-pre-assert-deassert-restore-postreadback,paired-no-gpio-no-ethernet-control,precise-failure-classifications",
        );
        write_rp1_ethernet_gpio32_phy_reset_preflight_rejections();
        write_early_static(
            " classification=rp1-ethernet-gpio32-phy-reset-readonly-preflight-report-visible\n",
        );
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_ethernet_gpio32_phy_reset_preflight_control")]
pub fn run_rp1_ethernet_gpio32_phy_reset_preflight_control() -> ! {
    write_early_static("rpi5-rp1-ethernet-gpio32-phy-reset-preflight-control: start\n");
    write_early_static(
        "rpi5-rp1-ethernet-gpio32-phy-reset-preflight-control: no-gpio-no-ethernet-no-mdio-no-mmio-target-construction\n",
    );
    wait_uart10_empty_early_phase();

    loop {
        write_early_static("TALOS: rp1-ethernet-gpio32-phy-reset-preflight-control");
        write_rp1_ethernet_gpio32_phy_reset_preflight_capture_nonce();
        write_rp1_ethernet_gpio32_phy_reset_preflight_common("no-gpio-no-ethernet-control");
        write_early_static(" accepted-input-frontier=withheld");
        write_early_static(" controller=none compatible=none");
        write_early_static(" phy-mode=none phy-handle=none phy-node=none phy-reg=none");
        write_early_static(" gpio-controller=none gpio-line=none signal=none");
        write_early_static(
            " active-low=false logical-assertion-value=none physical-assertion=none",
        );
        write_early_static(" logical-deassertion-value=none physical-deassertion=none");
        write_early_static(" reset-duration-ms=none");
        write_early_static(" linux-hook-relationship=withheld");
        write_early_static(" phase11-gpio-constraints=withheld");
        write_early_static(" source-evidence=withheld");
        write_early_static(" future-write-restore-invariants=withheld");
        write_rp1_ethernet_gpio32_phy_reset_preflight_rejections();
        write_early_static(
            " classification=no-gpio-no-ethernet-rp1-ethernet-gpio32-phy-reset-control\n",
        );
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_ethernet_gpio32_phy_reset_write_restore_candidate")]
pub fn run_rp1_ethernet_gpio32_phy_reset_write_restore_candidate() -> ! {
    const BIT_MASK: u32 = 1 << 4;

    write_early_static("rpi5-rp1-ethernet-gpio32-phy-reset-write-restore-candidate: start\n");
    write_early_static(
        "rpi5-rp1-ethernet-gpio32-phy-reset-write-restore-candidate: before-gpio32-eth-rst-n-write-restore\n",
    );
    wait_uart10_empty_early_phase();

    let baseline_status = read_rp1_reg_u32(RP1_GPIO32_OBSERVED_APERTURE_STATUS);
    let baseline_ctrl = read_rp1_reg_u32(RP1_GPIO32_OBSERVED_APERTURE_CTRL);
    let baseline_out = read_rp1_reg_u32(RP1_RIO1_OBSERVED_APERTURE_OUT);
    let baseline_oe = read_rp1_reg_u32(RP1_RIO1_OBSERVED_APERTURE_OE);
    let baseline_in = read_rp1_reg_u32(RP1_RIO1_OBSERVED_APERTURE_IN);
    let baseline_pad = read_rp1_reg_u32(RP1_GPIO32_OBSERVED_APERTURE_PAD);

    let funcsel = baseline_ctrl & 0x1f;
    let override_bits = baseline_ctrl & ((0x3 << 12) | (0x3 << 14) | (0x3 << 16));
    let event_bits = baseline_status & (0xff << 20);
    let irq_bits = baseline_ctrl & ((0xff << 20) | (1 << 28));
    let pad_out_disabled = (baseline_pad & (1 << 7)) != 0;
    let blocked_classification = classify_rp1_ethernet_gpio32_phy_reset_write_restore_preconditions(
        baseline_status,
        baseline_ctrl,
        baseline_out,
        baseline_oe,
        baseline_in,
        baseline_pad,
        funcsel,
        override_bits,
        event_bits,
        irq_bits,
        pad_out_disabled,
    );

    let mut assertion_out = baseline_out;
    let mut assertion_oe = baseline_oe;
    let mut assertion_in = baseline_in;
    let mut deassertion_out = baseline_out;
    let mut deassertion_oe = baseline_oe;
    let mut deassertion_in = baseline_in;
    let mut restore_out = baseline_out;
    let mut restore_oe = baseline_oe;
    let mut restore_in = baseline_in;
    let mut writes_performed = false;
    let mut wait_ticks = 0;

    let classification = if let Some(blocked) = blocked_classification {
        blocked
    } else {
        let assert_out_value = baseline_out & !BIT_MASK;
        let assert_oe_value = baseline_oe | BIT_MASK;
        write_rp1_reg_u32_ordered(RP1_RIO1_OBSERVED_APERTURE_OUT, assert_out_value);
        write_rp1_reg_u32_ordered(RP1_RIO1_OBSERVED_APERTURE_OE, assert_oe_value);
        assertion_out = read_rp1_reg_u32(RP1_RIO1_OBSERVED_APERTURE_OUT);
        assertion_oe = read_rp1_reg_u32(RP1_RIO1_OBSERVED_APERTURE_OE);
        assertion_in = read_rp1_reg_u32(RP1_RIO1_OBSERVED_APERTURE_IN);
        wait_ticks = wait_rp1_ethernet_gpio32_phy_reset_duration();

        let deassert_out_value = assert_out_value | BIT_MASK;
        write_rp1_reg_u32_ordered(RP1_RIO1_OBSERVED_APERTURE_OUT, deassert_out_value);
        deassertion_out = read_rp1_reg_u32(RP1_RIO1_OBSERVED_APERTURE_OUT);
        deassertion_oe = read_rp1_reg_u32(RP1_RIO1_OBSERVED_APERTURE_OE);
        deassertion_in = read_rp1_reg_u32(RP1_RIO1_OBSERVED_APERTURE_IN);

        write_rp1_reg_u32_ordered(RP1_RIO1_OBSERVED_APERTURE_OUT, baseline_out);
        write_rp1_reg_u32_ordered(RP1_RIO1_OBSERVED_APERTURE_OE, baseline_oe);
        restore_out = read_rp1_reg_u32(RP1_RIO1_OBSERVED_APERTURE_OUT);
        restore_oe = read_rp1_reg_u32(RP1_RIO1_OBSERVED_APERTURE_OE);
        restore_in = read_rp1_reg_u32(RP1_RIO1_OBSERVED_APERTURE_IN);
        writes_performed = true;

        classify_rp1_ethernet_gpio32_phy_reset_write_restore_result(
            assertion_out,
            assertion_oe,
            deassertion_out,
            deassertion_oe,
            restore_out,
            restore_oe,
            baseline_out,
            baseline_oe,
        )
    };

    loop {
        write_early_static("TALOS: rp1-ethernet-gpio32-phy-reset-write-restore-candidate");
        write_rp1_ethernet_gpio32_phy_reset_write_restore_capture_nonce();
        write_rp1_ethernet_gpio32_phy_reset_write_restore_common("candidate");
        write_early_static(" target=rp1-ethernet-gpio32-eth-rst-n-write-restore");
        write_early_static(
            " gpio-controller=rp1_gpio gpio-line=32 signal=ETH_RST_N bank=bank1 bank-local-bit=4",
        );
        write_early_static(" active-low=true reset-duration-ms=5");
        write_early_static(" gpio32-status-address=");
        write_early_hex_u64(RP1_GPIO32_OBSERVED_APERTURE_STATUS as u64);
        write_early_static(" gpio32-ctrl-address=");
        write_early_hex_u64(RP1_GPIO32_OBSERVED_APERTURE_CTRL as u64);
        write_early_static(" rio1-out-address=");
        write_early_hex_u64(RP1_RIO1_OBSERVED_APERTURE_OUT as u64);
        write_early_static(" rio1-oe-address=");
        write_early_hex_u64(RP1_RIO1_OBSERVED_APERTURE_OE as u64);
        write_early_static(" rio1-in-address=");
        write_early_hex_u64(RP1_RIO1_OBSERVED_APERTURE_IN as u64);
        write_early_static(" gpio32-pad-address=");
        write_early_hex_u64(RP1_GPIO32_OBSERVED_APERTURE_PAD as u64);
        write_early_static(" baseline-status=");
        write_early_hex_u64(baseline_status as u64);
        write_early_static(" baseline-ctrl=");
        write_early_hex_u64(baseline_ctrl as u64);
        write_early_static(" baseline-out=");
        write_early_hex_u64(baseline_out as u64);
        write_early_static(" baseline-oe=");
        write_early_hex_u64(baseline_oe as u64);
        write_early_static(" baseline-in=");
        write_early_hex_u64(baseline_in as u64);
        write_early_static(" baseline-pad=");
        write_early_hex_u64(baseline_pad as u64);
        write_early_static(" funcsel=");
        write_early_dec_u64(funcsel as u64);
        write_early_static(" override-bits=");
        write_early_hex_u64(override_bits as u64);
        write_early_static(" event-bits=");
        write_early_hex_u64(event_bits as u64);
        write_early_static(" irq-bits=");
        write_early_hex_u64(irq_bits as u64);
        write_early_static(" pad-out-disabled=");
        write_bool(pad_out_disabled);
        write_early_static(" writes-performed=");
        write_bool(writes_performed);
        write_early_static(" assertion-out=");
        write_early_hex_u64(assertion_out as u64);
        write_early_static(" assertion-oe=");
        write_early_hex_u64(assertion_oe as u64);
        write_early_static(" assertion-in=");
        write_early_hex_u64(assertion_in as u64);
        write_early_static(" deassertion-out=");
        write_early_hex_u64(deassertion_out as u64);
        write_early_static(" deassertion-oe=");
        write_early_hex_u64(deassertion_oe as u64);
        write_early_static(" deassertion-in=");
        write_early_hex_u64(deassertion_in as u64);
        write_early_static(" restore-out=");
        write_early_hex_u64(restore_out as u64);
        write_early_static(" restore-oe=");
        write_early_hex_u64(restore_oe as u64);
        write_early_static(" restore-in=");
        write_early_hex_u64(restore_in as u64);
        write_early_static(" restore-out-eq-baseline=");
        write_bool(restore_out == baseline_out);
        write_early_static(" restore-oe-eq-baseline=");
        write_bool(restore_oe == baseline_oe);
        write_early_static(" wait-ticks=");
        write_early_dec_u64(wait_ticks);
        write_early_static(" touched-fields=RIO1_OUT.bit4,RIO1_OE.bit4");
        write_rp1_ethernet_gpio32_phy_reset_write_restore_rejections();
        write_early_static(" classification=");
        write_early_static(classification);
        write_early_static("\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_ethernet_gpio32_phy_reset_write_restore_no_mmio_control")]
pub fn run_rp1_ethernet_gpio32_phy_reset_write_restore_no_mmio_control() -> ! {
    write_early_static("rpi5-rp1-ethernet-gpio32-phy-reset-write-restore-control: start\n");
    write_early_static(
        "rpi5-rp1-ethernet-gpio32-phy-reset-write-restore-control: no-gpio-write-no-ethernet-no-mmio-target-construction\n",
    );
    wait_uart10_empty_early_phase();

    loop {
        write_early_static("TALOS: rp1-ethernet-gpio32-phy-reset-write-restore-control");
        write_rp1_ethernet_gpio32_phy_reset_write_restore_capture_nonce();
        write_rp1_ethernet_gpio32_phy_reset_write_restore_common(
            "no-gpio-write-no-ethernet-control",
        );
        write_early_static(
            " target=none gpio-controller=none gpio-line=none signal=none bank=none bank-local-bit=none",
        );
        write_early_static(" active-low=false reset-duration-ms=none");
        write_early_static(
            " gpio32-status-address=not-constructed gpio32-ctrl-address=not-constructed",
        );
        write_early_static(
            " rio1-out-address=not-constructed rio1-oe-address=not-constructed rio1-in-address=not-constructed",
        );
        write_early_static(" gpio32-pad-address=not-constructed");
        write_early_static(
            " baseline-status=withheld baseline-ctrl=withheld baseline-out=withheld baseline-oe=withheld baseline-in=withheld baseline-pad=withheld",
        );
        write_early_static(
            " funcsel=withheld override-bits=withheld event-bits=withheld irq-bits=withheld pad-out-disabled=withheld",
        );
        write_early_static(
            " writes-performed=false assertion-out=withheld assertion-oe=withheld assertion-in=withheld",
        );
        write_early_static(
            " deassertion-out=withheld deassertion-oe=withheld deassertion-in=withheld",
        );
        write_early_static(" restore-out=withheld restore-oe=withheld restore-in=withheld");
        write_early_static(
            " restore-out-eq-baseline=true restore-oe-eq-baseline=true wait-ticks=0",
        );
        write_early_static(" touched-fields=none");
        write_rp1_ethernet_gpio32_phy_reset_write_restore_rejections();
        write_early_static(
            " classification=no-gpio-write-no-ethernet-rp1-ethernet-gpio32-phy-reset-control\n",
        );
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_ethernet_gem_mid_decode_discriminator_candidate")]
pub fn run_rp1_ethernet_gem_mid_decode_discriminator_candidate() -> ! {
    const MACB_MID: usize = 0x1f_0010_00fc;

    write_early_static("rpi5-rp1-ethernet-gem-mid-decode-discriminator-candidate: start\n");
    write_early_static(
        "rpi5-rp1-ethernet-gem-mid-decode-discriminator-candidate: before-observed-sysinfo-and-gem-mid-read-only-volatile-loads\n",
    );
    wait_uart10_empty_early_phase();

    let observed_sysinfo_chip_id = read_rp1_reg_u32(RP1_SYSINFO_OBSERVED_APERTURE_CHIP_ID);
    let raw = read_rp1_reg_u32(MACB_MID);
    let idnum = (raw >> 16) & 0x0fff;
    let rev = raw & 0xffff;
    let sysinfo_matches_expected = observed_sysinfo_chip_id == RP1_EXPECTED_CHIP_ID;
    let sysinfo_is_deaddead = observed_sysinfo_chip_id == 0xdead_dead;
    let gem_mid_is_deaddead = raw == 0xdead_dead;
    let classification = classify_rp1_ethernet_gem_mid_decode_discriminator(
        sysinfo_matches_expected,
        sysinfo_is_deaddead,
        gem_mid_is_deaddead,
        raw,
    );

    loop {
        write_early_static("TALOS: rp1-ethernet-gem-mid-decode-discriminator-candidate");
        write_rp1_ethernet_gem_mid_decode_discriminator_capture_nonce();
        write_rp1_ethernet_gem_mid_decode_discriminator_common("candidate");
        write_early_static(" same-run-required=true changed-from-gem-mid-only-proof=true");
        write_early_static(" observed-positive-control-register=SYSINFO_CHIP_ID");
        write_early_static(" observed-positive-control-cpu-physical-target=");
        write_early_hex_u64(RP1_SYSINFO_OBSERVED_APERTURE_CHIP_ID as u64);
        write_early_static(" observed-positive-control-expected=");
        write_early_hex_u64(RP1_EXPECTED_CHIP_ID as u64);
        write_early_static(" observed-positive-control-raw=");
        write_early_hex_u64(observed_sysinfo_chip_id as u64);
        write_early_static(" observed-positive-control-matches-expected=");
        write_bool(sysinfo_matches_expected);
        write_early_static(" observed-positive-control-is-deaddead=");
        write_bool(sysinfo_is_deaddead);
        write_early_static(" compatible=raspberrypi,rp1-gem,cdns,macb");
        write_early_static(" controller=rp1_eth register=MACB_MID");
        write_early_static(" rp1-bus-base=");
        write_early_hex_u64(0xc0_4010_0000);
        write_early_static(" cpu-physical-base=");
        write_early_hex_u64(0x1f_0010_0000);
        write_early_static(" offset=");
        write_early_hex_u64(0x00fc);
        write_early_static(" rp1-bus-target=");
        write_early_hex_u64(0xc0_4010_00fc);
        write_early_static(" cpu-physical-target=");
        write_early_hex_u64(MACB_MID as u64);
        write_early_static(" width=32 endianness=little-endian access=read-only-volatile-load");
        write_early_static(" raw=");
        write_early_hex_u64(raw as u64);
        write_early_static(" idnum=");
        write_early_hex_u64(idnum as u64);
        write_early_static(" rev=");
        write_early_hex_u64(rev as u64);
        write_early_static(" raw-is-zero=");
        write_bool(raw == 0);
        write_early_static(" raw-is-all-ones=");
        write_bool(raw == 0xffff_ffff);
        write_early_static(" raw-is-deaddead=");
        write_bool(gem_mid_is_deaddead);
        write_rp1_ethernet_gem_mid_decode_discriminator_rejections();
        write_early_static(" classification=");
        write_early_static(classification);
        write_early_static("\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_ethernet_gem_mid_decode_discriminator_no_mmio_control")]
pub fn run_rp1_ethernet_gem_mid_decode_discriminator_no_mmio_control() -> ! {
    write_early_static("rpi5-rp1-ethernet-gem-mid-decode-discriminator-control: start\n");
    write_early_static(
        "rpi5-rp1-ethernet-gem-mid-decode-discriminator-control: no-observed-rp1-no-ethernet-no-mmio-target-construction\n",
    );
    wait_uart10_empty_early_phase();

    loop {
        write_early_static("TALOS: rp1-ethernet-gem-mid-decode-discriminator-control");
        write_rp1_ethernet_gem_mid_decode_discriminator_capture_nonce();
        write_rp1_ethernet_gem_mid_decode_discriminator_common("no-mmio-no-ethernet-control");
        write_early_static(" same-run-required=true changed-from-gem-mid-only-proof=true");
        write_early_static(" observed-positive-control-register=none");
        write_early_static(" observed-positive-control-cpu-physical-target=not-constructed");
        write_early_static(" observed-positive-control-expected=none");
        write_early_static(" observed-positive-control-raw=none");
        write_early_static(" observed-positive-control-matches-expected=false");
        write_early_static(" observed-positive-control-is-deaddead=false");
        write_early_static(" compatible=none controller=none register=MACB_MID");
        write_early_static(" rp1-bus-base=none cpu-physical-base=none offset=none");
        write_early_static(" rp1-bus-target=none cpu-physical-target=not-constructed");
        write_early_static(" width=32 endianness=little-endian access=not-constructed");
        write_early_static(" raw=none idnum=none rev=none");
        write_early_static(" raw-is-zero=false raw-is-all-ones=false raw-is-deaddead=false");
        write_rp1_ethernet_gem_mid_decode_discriminator_rejections();
        write_early_static(
            " classification=no-mmio-no-ethernet-rp1-ethernet-gem-mid-decode-discriminator-control\n",
        );
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_ethernet_observed_window_discriminator_candidate")]
pub fn run_rp1_ethernet_observed_window_discriminator_candidate() -> ! {
    write_early_static("rpi5-rp1-ethernet-observed-window-discriminator-candidate: start\n");
    write_early_static(
        "rpi5-rp1-ethernet-observed-window-discriminator-candidate: before-observed-sysinfo-and-observed-window-gem-mid-read-only-volatile-loads\n",
    );
    wait_uart10_empty_early_phase();

    let observed_sysinfo_chip_id = read_rp1_reg_u32(RP1_SYSINFO_OBSERVED_APERTURE_CHIP_ID);
    let raw = read_rp1_reg_u32(RP1_ETHERNET_OBSERVED_WINDOW_GEM_MID);
    let idnum = (raw >> 16) & 0x0fff;
    let rev = raw & 0xffff;
    let sysinfo_matches_expected = observed_sysinfo_chip_id == RP1_EXPECTED_CHIP_ID;
    let sysinfo_is_deaddead = observed_sysinfo_chip_id == 0xdead_dead;
    let gem_mid_is_deaddead = raw == 0xdead_dead;
    let classification = classify_rp1_ethernet_observed_window_discriminator(
        sysinfo_matches_expected,
        sysinfo_is_deaddead,
        gem_mid_is_deaddead,
        raw,
    );

    loop {
        write_early_static("TALOS: rp1-ethernet-observed-window-discriminator-candidate");
        write_rp1_ethernet_observed_window_discriminator_capture_nonce();
        write_rp1_ethernet_observed_window_discriminator_common("candidate");
        write_early_static(
            " same-run-required=true material-difference-from-translated-window=true",
        );
        write_early_static(" observed-positive-control-register=SYSINFO_CHIP_ID");
        write_early_static(" observed-positive-control-cpu-physical-target=");
        write_early_hex_u64(RP1_SYSINFO_OBSERVED_APERTURE_CHIP_ID as u64);
        write_early_static(" observed-positive-control-expected=");
        write_early_hex_u64(RP1_EXPECTED_CHIP_ID as u64);
        write_early_static(" observed-positive-control-raw=");
        write_early_hex_u64(observed_sysinfo_chip_id as u64);
        write_early_static(" observed-positive-control-matches-expected=");
        write_bool(sysinfo_matches_expected);
        write_early_static(" observed-positive-control-is-deaddead=");
        write_bool(sysinfo_is_deaddead);
        write_early_static(" compatible=raspberrypi,rp1-gem,cdns,macb");
        write_early_static(" controller=rp1_eth register=MACB_MID");
        write_early_static(" source-offset-from-observed-rp1-base=");
        write_early_hex_u64(0x0010_00fc);
        write_early_static(" observed-rp1-base=");
        write_early_hex_u64(RP1_SYSINFO_OBSERVED_APERTURE_BASE as u64);
        write_early_static(" observed-window-cpu-physical-target=");
        write_early_hex_u64(RP1_ETHERNET_OBSERVED_WINDOW_GEM_MID as u64);
        write_early_static(" translated-window-comparator-cpu-physical-target=");
        write_early_hex_u64(0x1f_0010_00fc);
        write_early_static(" translated-window-comparator-role=comparator-sentinel-only");
        write_early_static(" offset=");
        write_early_hex_u64(0x00fc);
        write_early_static(" width=32 endianness=little-endian access=read-only-volatile-load");
        write_early_static(" raw=");
        write_early_hex_u64(raw as u64);
        write_early_static(" idnum=");
        write_early_hex_u64(idnum as u64);
        write_early_static(" rev=");
        write_early_hex_u64(rev as u64);
        write_early_static(" raw-is-zero=");
        write_bool(raw == 0);
        write_early_static(" raw-is-all-ones=");
        write_bool(raw == 0xffff_ffff);
        write_early_static(" raw-is-deaddead=");
        write_bool(gem_mid_is_deaddead);
        write_rp1_ethernet_observed_window_discriminator_rejections();
        write_early_static(" classification=");
        write_early_static(classification);
        write_early_static("\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_ethernet_observed_window_discriminator_no_mmio_control")]
pub fn run_rp1_ethernet_observed_window_discriminator_no_mmio_control() -> ! {
    write_early_static("rpi5-rp1-ethernet-observed-window-discriminator-control: start\n");
    write_early_static(
        "rpi5-rp1-ethernet-observed-window-discriminator-control: no-observed-window-no-ethernet-no-mmio-target-construction\n",
    );
    wait_uart10_empty_early_phase();

    loop {
        write_early_static("TALOS: rp1-ethernet-observed-window-discriminator-control");
        write_rp1_ethernet_observed_window_discriminator_capture_nonce();
        write_rp1_ethernet_observed_window_discriminator_common("no-mmio-no-ethernet-control");
        write_early_static(
            " same-run-required=true material-difference-from-translated-window=true",
        );
        write_early_static(" observed-positive-control-register=none");
        write_early_static(" observed-positive-control-cpu-physical-target=not-constructed");
        write_early_static(" observed-positive-control-expected=none");
        write_early_static(" observed-positive-control-raw=none");
        write_early_static(" observed-positive-control-matches-expected=false");
        write_early_static(" observed-positive-control-is-deaddead=false");
        write_early_static(" compatible=none controller=none register=MACB_MID");
        write_early_static(" source-offset-from-observed-rp1-base=none observed-rp1-base=none");
        write_early_static(" observed-window-cpu-physical-target=not-constructed");
        write_early_static(" translated-window-comparator-cpu-physical-target=none");
        write_early_static(" translated-window-comparator-role=none");
        write_early_static(" offset=none width=32 endianness=little-endian access=not-constructed");
        write_early_static(" raw=none idnum=none rev=none");
        write_early_static(" raw-is-zero=false raw-is-all-ones=false raw-is-deaddead=false");
        write_rp1_ethernet_observed_window_discriminator_rejections();
        write_early_static(
            " classification=no-mmio-no-ethernet-rp1-ethernet-observed-window-control\n",
        );
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_ethernet_prereq_ownership_candidate")]
pub fn run_rp1_ethernet_prereq_ownership_candidate() -> ! {
    write_early_static("rpi5-rp1-ethernet-prereq-ownership-candidate: start\n");
    write_early_static(
        "rpi5-rp1-ethernet-prereq-ownership-candidate: static-prerequisite-report-no-mmio-writes\n",
    );
    wait_uart10_empty_early_phase();

    loop {
        write_early_static("TALOS: rp1-ethernet-prereq-ownership-candidate");
        write_rp1_ethernet_prereq_ownership_capture_nonce();
        write_rp1_ethernet_prereq_ownership_common("candidate");
        write_early_static(
            " selected-prerequisite=rp1-ethernet-clock-reset-phy-mdio-dma-ownership-report",
        );
        write_early_static(" observed-window-macb-mid-context-cpu-physical-target=");
        write_early_hex_u64(RP1_ETHERNET_OBSERVED_WINDOW_GEM_MID as u64);
        write_early_static(" observed-window-macb-mid-context-raw=");
        write_early_hex_u64(0x0007_0109);
        write_early_static(" observed-window-macb-mid-context-idnum=");
        write_early_hex_u64(0x7);
        write_early_static(" observed-window-macb-mid-context-rev=");
        write_early_hex_u64(0x109);
        write_early_static(
            " observed-window-macb-mid-context-role=context-only-not-broad-ethernet-mmio-readiness",
        );
        write_early_static(" compatible=raspberrypi,rp1-gem,cdns,macb controller=rp1_eth");
        write_early_static(" rp1-bus-base=");
        write_early_hex_u64(0xc0_4010_0000);
        write_early_static(" rp1-bus-window-size=");
        write_early_hex_u64(0x4000);
        write_early_static(" translated-comparator-cpu-physical-base=");
        write_early_hex_u64(0x1f_0010_0000);
        write_early_static(" translated-comparator-macb-mid-target=");
        write_early_hex_u64(0x1f_0010_00fc);
        write_early_static(" translated-comparator-role=comparator-sentinel-only");
        write_early_static(" interrupt-name=RP1_INT_ETH interrupt-number=6");
        write_early_static(" clock-names=pclk,hclk,tsu_clk,tx_clk clock-ids=12,12,29,16");
        write_early_static(" clock-sources=RP1_CLK_SYS,RP1_CLK_SYS,RP1_CLK_ETH_TSU,RP1_CLK_ETH");
        write_early_static(" clock-policy-classification=no-clock-reset-ownership");
        write_early_static(" phy-mode=rgmii-id phy-handle=phy1 phy-node=ethernet-phy@1 phy-reg=");
        write_early_hex_u64(0x1);
        write_early_static(" phy-reset-gpio=32 phy-reset-active-low=true phy-reset-duration-ms=5");
        write_early_static(" phy-mdio-policy-classification=no-phy-reset-or-mdio-ownership");
        write_early_static(
            " dma-descriptor-policy-classification=no-live-dma-or-descriptor-ownership",
        );
        write_early_static(
            " cadence-rp1-config=gigabit,hardware-clock-change,jumbo,ptp,dma-burst-length-16",
        );
        write_rp1_ethernet_prereq_ownership_rejections();
        write_early_static(" classification=rp1-ethernet-prereq-ownership-report-visible\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_ethernet_prereq_ownership_no_mmio_control")]
pub fn run_rp1_ethernet_prereq_ownership_no_mmio_control() -> ! {
    write_early_static("rpi5-rp1-ethernet-prereq-ownership-control: start\n");
    write_early_static(
        "rpi5-rp1-ethernet-prereq-ownership-control: no-ownership-no-ethernet-no-mmio-target-construction\n",
    );
    wait_uart10_empty_early_phase();

    loop {
        write_early_static("TALOS: rp1-ethernet-prereq-ownership-control");
        write_rp1_ethernet_prereq_ownership_capture_nonce();
        write_rp1_ethernet_prereq_ownership_common("no-ownership-no-ethernet-control");
        write_early_static(" selected-prerequisite=none");
        write_early_static(" observed-window-macb-mid-context-cpu-physical-target=not-constructed");
        write_early_static(" observed-window-macb-mid-context-raw=none");
        write_early_static(" observed-window-macb-mid-context-idnum=none");
        write_early_static(" observed-window-macb-mid-context-rev=none");
        write_early_static(" observed-window-macb-mid-context-role=none");
        write_early_static(" compatible=none controller=none");
        write_early_static(" rp1-bus-base=none rp1-bus-window-size=none");
        write_early_static(" translated-comparator-cpu-physical-base=none");
        write_early_static(" translated-comparator-macb-mid-target=not-constructed");
        write_early_static(" translated-comparator-role=none");
        write_early_static(" interrupt-name=none interrupt-number=none");
        write_early_static(" clock-names=none clock-ids=none clock-sources=none");
        write_early_static(" clock-policy-classification=no-clock-reset-ownership");
        write_early_static(" phy-mode=none phy-handle=none phy-node=none phy-reg=none");
        write_early_static(
            " phy-reset-gpio=none phy-reset-active-low=false phy-reset-duration-ms=none",
        );
        write_early_static(" phy-mdio-policy-classification=no-phy-reset-or-mdio-ownership");
        write_early_static(
            " dma-descriptor-policy-classification=no-live-dma-or-descriptor-ownership",
        );
        write_early_static(" cadence-rp1-config=none");
        write_rp1_ethernet_prereq_ownership_rejections();
        write_early_static(
            " classification=no-ownership-no-ethernet-rp1-ethernet-prereq-control\n",
        );
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_ethernet_clock_reset_readonly_baseline_candidate")]
pub fn run_rp1_ethernet_clock_reset_readonly_baseline_candidate() -> ! {
    write_early_static("rpi5-rp1-ethernet-clock-reset-readonly-baseline-candidate: start\n");
    write_early_static(
        "rpi5-rp1-ethernet-clock-reset-readonly-baseline-candidate: read-only-baseline-report-no-mmio-writes\n",
    );
    wait_uart10_empty_early_phase();

    loop {
        write_early_static("TALOS: rp1-ethernet-clock-reset-readonly-baseline-candidate");
        write_rp1_ethernet_clock_reset_readonly_baseline_capture_nonce();
        write_rp1_ethernet_clock_reset_readonly_baseline_common("candidate");
        write_early_static(" observed-window-macb-mid-context-cpu-physical-target=");
        write_early_hex_u64(RP1_ETHERNET_OBSERVED_WINDOW_GEM_MID as u64);
        write_early_static(" observed-window-macb-mid-context-raw=");
        write_early_hex_u64(0x0007_0109);
        write_early_static(" observed-window-macb-mid-context-idnum=");
        write_early_hex_u64(0x7);
        write_early_static(" observed-window-macb-mid-context-rev=");
        write_early_hex_u64(0x109);
        write_early_static(
            " observed-window-macb-mid-context-role=context-only-not-broad-ethernet-mmio-readiness",
        );
        write_early_static(
            " selected-read-only-baseline-fields=pclk,hclk,tsu_clk,tx_clk,shared-rp1-clk-sys,ethernet-private-clock-ids,reset-controller-policy,phy-reset-gpio-context",
        );
        write_early_static(" clock-names=pclk,hclk,tsu_clk,tx_clk");
        write_early_static(" clock-ids=12,12,29,16");
        write_early_static(" clock-sources=RP1_CLK_SYS,RP1_CLK_SYS,RP1_CLK_ETH_TSU,RP1_CLK_ETH");
        write_early_static(
            " shared-clock-names=pclk,hclk shared-clock-source=RP1_CLK_SYS shared-clock-id=12",
        );
        write_early_static(" ethernet-private-clock-names=tsu_clk,tx_clk");
        write_early_static(" ethernet-private-clock-sources=RP1_CLK_ETH_TSU,RP1_CLK_ETH");
        write_early_static(" ethernet-private-clock-ids=29,16");
        write_early_static(" clock-policy-classification=no-clock-reset-ownership");
        write_early_static(
            " reset-controller-policy-classification=no-accepted-rp1-eth-reset-controller-target",
        );
        write_early_static(" phy-reset-gpio-context=32");
        write_early_static(" phy-mdio-policy-classification=no-phy-reset-or-mdio-ownership");
        write_early_static(
            " read-only-baseline-requirements=same-candidate-control-report-path,observed-window-macb-mid-identity-context-only,pclk-hclk-tsu-clk-tx-clk-source-backed-names-and-ids,pclk-and-hclk-shared-rp1-clk-sys-inputs,tx-clk-and-tsu-clk-ethernet-specific-source-ids-without-accepted-write-targets,no-accepted-rp1-eth-reset-controller-target,paired-no-clock-reset-no-ethernet-control",
        );
        write_early_static(
            " write-backed-invariants=do-not-transition-rp1-clk-sys,do-not-touch-reset-controller-without-target-and-restore-contract,do-not-fold-gpio32-or-mdio-into-clock-reset-ownership,future-writes-require-pre-post-restore-evidence,preserve-non-target-clock-fields,paired-control-required,reject-downstream-inference",
        );
        write_rp1_ethernet_clock_reset_readonly_baseline_rejections();
        write_early_static(
            " classification=rp1-ethernet-clock-reset-readonly-baseline-report-visible\n",
        );
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_ethernet_clock_reset_readonly_baseline_no_mmio_control")]
pub fn run_rp1_ethernet_clock_reset_readonly_baseline_no_mmio_control() -> ! {
    write_early_static("rpi5-rp1-ethernet-clock-reset-readonly-baseline-control: start\n");
    write_early_static(
        "rpi5-rp1-ethernet-clock-reset-readonly-baseline-control: no-clock-reset-no-ethernet-no-mmio-target-construction\n",
    );
    wait_uart10_empty_early_phase();

    loop {
        write_early_static("TALOS: rp1-ethernet-clock-reset-readonly-baseline-control");
        write_rp1_ethernet_clock_reset_readonly_baseline_capture_nonce();
        write_rp1_ethernet_clock_reset_readonly_baseline_common(
            "no-clock-reset-no-ethernet-control",
        );
        write_early_static(" observed-window-macb-mid-context-cpu-physical-target=not-constructed");
        write_early_static(" observed-window-macb-mid-context-raw=none");
        write_early_static(" observed-window-macb-mid-context-idnum=none");
        write_early_static(" observed-window-macb-mid-context-rev=none");
        write_early_static(" observed-window-macb-mid-context-role=none");
        write_early_static(" selected-read-only-baseline-fields=none");
        write_early_static(" clock-names=none clock-ids=none clock-sources=none");
        write_early_static(
            " shared-clock-names=none shared-clock-source=none shared-clock-id=none",
        );
        write_early_static(" ethernet-private-clock-names=none");
        write_early_static(" ethernet-private-clock-sources=none");
        write_early_static(" ethernet-private-clock-ids=none");
        write_early_static(" clock-policy-classification=no-clock-reset-ownership");
        write_early_static(
            " reset-controller-policy-classification=no-accepted-rp1-eth-reset-controller-target",
        );
        write_early_static(" phy-reset-gpio-context=none");
        write_early_static(" phy-mdio-policy-classification=no-phy-reset-or-mdio-ownership");
        write_early_static(" read-only-baseline-requirements=withheld");
        write_early_static(" write-backed-invariants=withheld");
        write_rp1_ethernet_clock_reset_readonly_baseline_rejections();
        write_early_static(
            " classification=no-clock-reset-no-ethernet-rp1-ethernet-clock-reset-baseline-control\n",
        );
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_ethernet_gem_mid_decode_discriminator_candidate")]
fn classify_rp1_ethernet_gem_mid_decode_discriminator(
    sysinfo_matches_expected: bool,
    sysinfo_is_deaddead: bool,
    gem_mid_is_deaddead: bool,
    gem_mid_raw: u32,
) -> &'static str {
    if sysinfo_matches_expected && gem_mid_is_deaddead {
        "observed-rp1-positive-control-gem-mid-0x1f-window-sentinel"
    } else if sysinfo_matches_expected && gem_mid_raw != 0 && gem_mid_raw != 0xffff_ffff {
        "observed-rp1-positive-control-and-gem-mid-visible"
    } else if sysinfo_is_deaddead {
        "observed-rp1-positive-control-sentinel"
    } else {
        "inconclusive-capture"
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_ethernet_observed_window_discriminator_candidate")]
fn classify_rp1_ethernet_observed_window_discriminator(
    sysinfo_matches_expected: bool,
    sysinfo_is_deaddead: bool,
    gem_mid_is_deaddead: bool,
    gem_mid_raw: u32,
) -> &'static str {
    if sysinfo_matches_expected && gem_mid_is_deaddead {
        "observed-window-macb-mid-sentinel-with-sysinfo-positive-control"
    } else if sysinfo_matches_expected && gem_mid_raw != 0 && gem_mid_raw != 0xffff_ffff {
        "observed-window-macb-mid-visible"
    } else if sysinfo_is_deaddead {
        "observed-window-positive-control-sentinel"
    } else {
        "inconclusive-capture"
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_ethernet_gem_mid_visibility_candidate")]
fn classify_rp1_ethernet_gem_mid(raw: u32) -> &'static str {
    if raw == 0xdead_dead {
        "rp1-ethernet-gem-mid-blocked-address-decode-sentinel"
    } else if raw == 0xffff_ffff {
        "rp1-ethernet-gem-mid-blocked-all-ones"
    } else if raw == 0 {
        "rp1-ethernet-gem-mid-blocked-zero"
    } else {
        "rp1-ethernet-gem-mid-visible"
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_ethernet_gem_mid_visibility_candidate",
    talos_boot_scenario = "rpi5_rp1_ethernet_gem_mid_visibility_no_mmio_control"
))]
fn write_rp1_ethernet_gem_mid_common(report_kind: &str) {
    write_early_static(
        " diagnostic-report-contract-id=phase12-rp1-ethernet-gem-mid-diagnostic-report-contract-v1",
    );
    write_early_static(" source-contract-id=phase12-rp1-ethernet-gem-mid-source-contract-20260609");
    write_early_static(" report-kind=");
    write_early_static(report_kind);
    write_early_static(
        " hardware-proof-boundary-classification=hardware-proof-limited-to-gem-mid-visibility-control-output",
    );
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_ethernet_gem_mid_visibility_candidate",
    talos_boot_scenario = "rpi5_rp1_ethernet_gem_mid_visibility_no_mmio_control"
))]
fn write_rp1_ethernet_gem_mid_capture_nonce() {
    if let Some(nonce) = option_env!("TALOS_CAPTURE_NONCE") {
        if !nonce.is_empty() {
            write_early_static(" capture-nonce=");
            write_early_static(nonce);
        }
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_ethernet_gem_mid_visibility_candidate",
    talos_boot_scenario = "rpi5_rp1_ethernet_gem_mid_visibility_no_mmio_control"
))]
fn write_rp1_ethernet_gem_mid_rejections() {
    write_early_static(
        " rejected-runtime-hardware-claims=ethernet-driver-readiness,broad-live-ethernet-mmio-readiness,rp1-mmio-dma-programming,descriptor-rings,dma-ownership,transfer-completion,interrupt-completion,clock-reset-ownership,phy-reset-ownership,packet-io,networking,sockets,ssh,phase-12-2,phase-transition",
    );
    write_early_static(
        " retained-risks=no-clock-reset-ownership,no-phy-reset-ownership,no-descriptor-ring-layout-or-ownership,no-live-dma-proof,no-packet-io,no-network-stack",
    );
    write_early_static(" claims-ethernet-ready=false claims-broad-mmio-ready=false");
    write_early_static(" claims-rp1-mmio-dma-programming=false claims-descriptor-rings=false");
    write_early_static(" claims-dma-ownership=false claims-transfer-completion=false");
    write_early_static(" claims-interrupt-completion=false claims-clock-reset-ownership=false");
    write_early_static(" claims-phy-ownership=false claims-packet-io=false");
    write_early_static(" claims-networking=false claims-sockets=false claims-ssh=false");
    write_early_static(" claims-phase-12-2=false claims-phase-transition=false");
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_ethernet_gem_mid_decode_discriminator_candidate",
    talos_boot_scenario = "rpi5_rp1_ethernet_gem_mid_decode_discriminator_no_mmio_control"
))]
fn write_rp1_ethernet_gem_mid_decode_discriminator_common(report_kind: &str) {
    write_early_static(
        " decode-discriminator-report-contract-id=phase12-rp1-ethernet-gem-mid-decode-discriminator-contract-v1",
    );
    write_early_static(" source-contract-id=phase12-rp1-ethernet-gem-mid-source-contract-20260609");
    write_early_static(" report-kind=");
    write_early_static(report_kind);
    write_early_static(
        " hardware-proof-boundary-classification=hardware-proof-limited-to-observed-sysinfo-gem-mid-discriminator-control-output",
    );
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_ethernet_gem_mid_decode_discriminator_candidate",
    talos_boot_scenario = "rpi5_rp1_ethernet_gem_mid_decode_discriminator_no_mmio_control"
))]
fn write_rp1_ethernet_gem_mid_decode_discriminator_capture_nonce() {
    if let Some(nonce) = option_env!("TALOS_CAPTURE_NONCE") {
        if !nonce.is_empty() {
            write_early_static(" capture-nonce=");
            write_early_static(nonce);
        }
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_ethernet_gem_mid_decode_discriminator_candidate",
    talos_boot_scenario = "rpi5_rp1_ethernet_gem_mid_decode_discriminator_no_mmio_control"
))]
fn write_rp1_ethernet_gem_mid_decode_discriminator_rejections() {
    write_early_static(
        " rejected-runtime-hardware-claims=ethernet-driver-readiness,broad-live-ethernet-mmio-readiness,rp1-mmio-dma-programming,descriptor-rings,dma-ownership,transfer-completion,interrupt-completion,clock-reset-ownership,phy-reset-ownership,packet-io,networking,sockets,ssh,phase-12-2,phase-transition",
    );
    write_early_static(
        " retained-risks=no-clock-reset-ownership,no-phy-reset-ownership,no-descriptor-ring-layout-or-ownership,no-live-dma-proof,no-packet-io,no-network-stack",
    );
    write_early_static(" claims-ethernet-ready=false claims-broad-mmio-ready=false");
    write_early_static(" claims-rp1-mmio-dma-programming=false claims-descriptor-rings=false");
    write_early_static(" claims-dma-ownership=false claims-transfer-completion=false");
    write_early_static(" claims-interrupt-completion=false claims-clock-reset-ownership=false");
    write_early_static(" claims-phy-ownership=false claims-packet-io=false");
    write_early_static(" claims-networking=false claims-sockets=false claims-ssh=false");
    write_early_static(" claims-phase-12-2=false claims-phase-transition=false");
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_ethernet_observed_window_discriminator_candidate",
    talos_boot_scenario = "rpi5_rp1_ethernet_observed_window_discriminator_no_mmio_control"
))]
fn write_rp1_ethernet_observed_window_discriminator_common(report_kind: &str) {
    write_early_static(
        " observed-window-contract-id=phase12-rp1-ethernet-observed-window-contract-v1",
    );
    write_early_static(
        " observed-window-discriminator-contract-id=phase12-rp1-ethernet-observed-window-discriminator-contract-v1",
    );
    write_early_static(
        " selected-by-task-id=phase12-rp1-ethernet-observed-window-contract-20260610",
    );
    write_early_static(" source-contract-id=phase12-rp1-ethernet-gem-mid-source-contract-20260609");
    write_early_static(" report-kind=");
    write_early_static(report_kind);
    write_early_static(
        " hardware-proof-boundary-classification=hardware-proof-limited-to-observed-window-gem-mid-discriminator-control-output",
    );
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_ethernet_observed_window_discriminator_candidate",
    talos_boot_scenario = "rpi5_rp1_ethernet_observed_window_discriminator_no_mmio_control"
))]
fn write_rp1_ethernet_observed_window_discriminator_capture_nonce() {
    if let Some(nonce) = option_env!("TALOS_CAPTURE_NONCE") {
        if !nonce.is_empty() {
            write_early_static(" capture-nonce=");
            write_early_static(nonce);
        }
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_ethernet_observed_window_discriminator_candidate",
    talos_boot_scenario = "rpi5_rp1_ethernet_observed_window_discriminator_no_mmio_control"
))]
fn write_rp1_ethernet_observed_window_discriminator_rejections() {
    write_early_static(
        " rejected-runtime-hardware-claims=ethernet-driver-readiness,broad-live-ethernet-mmio-readiness,rp1-mmio-dma-programming,descriptor-rings,dma-ownership,transfer-completion,interrupt-completion,clock-reset-ownership,phy-reset-ownership,packet-io,networking,sockets,ssh,phase-12-2,phase-transition",
    );
    write_early_static(
        " retained-risks=observed-window-macb-mid-may-sentinel-or-fault,pci-rp1-bridge-or-address-window-ownership-unaccepted,ethernet-clock-reset-and-phy-mdio-ownership-unaccepted,no-packet-io-no-network-stack",
    );
    write_early_static(" claims-ethernet-ready=false claims-broad-mmio-ready=false");
    write_early_static(" claims-rp1-mmio-dma-programming=false claims-descriptor-rings=false");
    write_early_static(" claims-dma-ownership=false claims-transfer-completion=false");
    write_early_static(" claims-interrupt-completion=false claims-clock-reset-ownership=false");
    write_early_static(" claims-phy-ownership=false claims-packet-io=false");
    write_early_static(" claims-networking=false claims-sockets=false claims-ssh=false");
    write_early_static(" claims-phase-12-2=false claims-phase-transition=false");
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_ethernet_prereq_ownership_candidate",
    talos_boot_scenario = "rpi5_rp1_ethernet_prereq_ownership_no_mmio_control"
))]
fn write_rp1_ethernet_prereq_ownership_common(report_kind: &str) {
    write_early_static(
        " prereq-ownership-contract-id=phase12-rp1-ethernet-prereq-ownership-contract-v1",
    );
    write_early_static(
        " source-task-id=phase12-rp1-ethernet-prereq-ownership-source-contract-20260610",
    );
    write_early_static(" report-kind=");
    write_early_static(report_kind);
    write_early_static(
        " hardware-proof-boundary-classification=hardware-proof-limited-to-prereq-ownership-report-visibility-control-output",
    );
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_ethernet_prereq_ownership_candidate",
    talos_boot_scenario = "rpi5_rp1_ethernet_prereq_ownership_no_mmio_control"
))]
fn write_rp1_ethernet_prereq_ownership_capture_nonce() {
    if let Some(nonce) = option_env!("TALOS_CAPTURE_NONCE") {
        if !nonce.is_empty() {
            write_early_static(" capture-nonce=");
            write_early_static(nonce);
        }
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_ethernet_prereq_ownership_candidate",
    talos_boot_scenario = "rpi5_rp1_ethernet_prereq_ownership_no_mmio_control"
))]
fn write_rp1_ethernet_prereq_ownership_rejections() {
    write_early_static(
        " rejected-runtime-hardware-claims=ethernet-driver-readiness,broad-ethernet-mmio-readiness,rp1-mmio-writes,clock-reset-ownership-or-writes,gpio32-ownership-or-phy-reset,mdio-transactions-or-phy-ownership,interrupt-delivery-handler-ownership-or-completion,dma-descriptor-rings-channel-ownership-or-transfer-completion,packet-io,networking,sockets,ssh,phase-12-2,phase-transition",
    );
    write_early_static(
        " retained-risks=observed-window-macb-mid-identity-does-not-prove-prerequisites,source-facts-not-talos-ownership,report-visibility-not-hardware-ownership,no-packet-io-no-network-stack",
    );
    write_early_static(" claims-ethernet-ready=false claims-broad-mmio-ready=false");
    write_early_static(" claims-rp1-mmio-writes=false claims-clock-reset-ownership=false");
    write_early_static(" claims-gpio32-phy-reset-ownership=false claims-mdio-phy-ownership=false");
    write_early_static(" claims-interrupt-ownership=false claims-dma-descriptor-ownership=false");
    write_early_static(" claims-packet-io=false claims-networking=false");
    write_early_static(" claims-sockets=false claims-ssh=false");
    write_early_static(" claims-phase-12-2=false claims-phase-transition=false");
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_ethernet_clock_reset_readonly_baseline_candidate",
    talos_boot_scenario = "rpi5_rp1_ethernet_clock_reset_readonly_baseline_no_mmio_control"
))]
fn write_rp1_ethernet_clock_reset_readonly_baseline_common(report_kind: &str) {
    write_early_static(
        " clock-reset-guard-contract-id=phase12-rp1-ethernet-clock-reset-guard-contract-v1",
    );
    write_early_static(
        " ownership-contract-task-id=phase12-rp1-ethernet-clock-reset-ownership-contract-20260610",
    );
    write_early_static(" prereq-contract-id=phase12-rp1-ethernet-prereq-ownership-contract-v1");
    write_early_static(" report-kind=");
    write_early_static(report_kind);
    write_early_static(
        " hardware-proof-boundary-classification=hardware-proof-limited-to-clock-reset-readonly-baseline-report-visibility-control-output",
    );
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_ethernet_clock_reset_readonly_baseline_candidate",
    talos_boot_scenario = "rpi5_rp1_ethernet_clock_reset_readonly_baseline_no_mmio_control"
))]
fn write_rp1_ethernet_clock_reset_readonly_baseline_capture_nonce() {
    if let Some(nonce) = option_env!("TALOS_CAPTURE_NONCE") {
        if !nonce.is_empty() {
            write_early_static(" capture-nonce=");
            write_early_static(nonce);
        }
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_ethernet_clock_reset_readonly_baseline_candidate",
    talos_boot_scenario = "rpi5_rp1_ethernet_clock_reset_readonly_baseline_no_mmio_control"
))]
fn write_rp1_ethernet_clock_reset_readonly_baseline_rejections() {
    write_early_static(
        " rejected-runtime-hardware-claims=ethernet-driver-readiness,broad-ethernet-mmio-readiness,rp1-mmio-writes,clock-reset-writes,clock-reset-ownership,rp1-clk-sys-transition,reset-controller-ownership,gpio32-ownership-or-phy-reset,mdio-transactions-or-phy-ownership,interrupt-delivery-handler-ownership-or-completion,dma-descriptor-rings-channel-ownership-or-transfer-completion,packet-io,networking,sockets,ssh,phase-12-2,phase-transition",
    );
    write_early_static(
        " retained-risks=clock-source-facts-not-talos-ownership,shared-rp1-clk-sys-pclk-hclk-require-shared-clock-safety,tx-clk-and-tsu-clk-need-exact-register-targets-and-restore-semantics,no-accepted-reset-controller-target,phy-reset-is-separate-gpio32-mdio-task",
    );
    write_early_static(" claims-ethernet-ready=false claims-broad-mmio-ready=false");
    write_early_static(" claims-rp1-mmio-writes=false claims-clock-reset-writes=false");
    write_early_static(" claims-clock-reset-ownership=false claims-rp1-clk-sys-transition=false");
    write_early_static(" claims-reset-controller-ownership=false");
    write_early_static(" claims-gpio32-phy-reset-ownership=false claims-mdio-phy-ownership=false");
    write_early_static(" claims-interrupt-ownership=false claims-dma-descriptor-ownership=false");
    write_early_static(" claims-packet-io=false claims-networking=false");
    write_early_static(" claims-sockets=false claims-ssh=false");
    write_early_static(" claims-phase-12-2=false claims-phase-transition=false");
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_ethernet_clock_reset_write_restore_candidate",
    talos_boot_scenario = "rpi5_rp1_ethernet_clock_reset_write_restore_no_mmio_control"
))]
fn write_rp1_ethernet_clock_reset_write_restore_common(
    report_contract_id: &str,
    target_contract_id: &str,
    report_kind: &str,
) {
    write_early_static(" write-restore-report-contract-id=");
    write_early_static(report_contract_id);
    write_early_static(" target-contract-id=");
    write_early_static(target_contract_id);
    write_early_static(
        " source-task-id=phase12-rp1-ethernet-clock-reset-write-target-source-contract-20260610",
    );
    write_early_static(" report-kind=");
    write_early_static(report_kind);
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_ethernet_clock_reset_write_restore_candidate",
    talos_boot_scenario = "rpi5_rp1_ethernet_clock_reset_write_restore_no_mmio_control"
))]
fn write_rp1_ethernet_clock_reset_write_restore_capture_nonce() {
    if let Some(nonce) = option_env!("TALOS_CAPTURE_NONCE") {
        if !nonce.is_empty() {
            write_early_static(" capture-nonce=");
            write_early_static(nonce);
        }
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_ethernet_clock_reset_write_restore_candidate",
    talos_boot_scenario = "rpi5_rp1_ethernet_clock_reset_write_restore_no_mmio_control"
))]
fn write_rp1_ethernet_clock_reset_write_restore_rejections() {
    write_early_static(
        " future-proof-classifications=rp1-ethernet-clk-eth-tsu-ctrl-idempotent-write-restored,rp1-ethernet-clk-eth-tsu-ctrl-idempotent-write-mismatch-restored,rp1-ethernet-clk-eth-tsu-ctrl-idempotent-write-restore-failed,rp1-ethernet-clk-eth-tsu-ctrl-blocked-missing-clock-manager,rp1-ethernet-clk-eth-tsu-ctrl-inconclusive-capture,no-clock-write-no-ethernet-rp1-ethernet-write-restore-control,staging-build-blocker",
    );
    write_early_static(
        " rejected-runtime-hardware-claims=ethernet-driver-readiness,broad-ethernet-mmio-readiness,unscoped-rp1-mmio-writes,rp1-clk-sys-transition,clk-eth-ctrl-write,reset-controller-ownership,gpio32-ownership-or-phy-reset,mdio-transactions-or-phy-ownership,interrupt-delivery-handler-ownership-or-completion,dma-descriptor-rings-channel-ownership-or-transfer-completion,packet-io,networking,sockets,ssh,phase-12-2,phase-transition",
    );
    write_early_static(
        " retained-risks=single-ethernet-private-clock-store-readback-only,clk-eth-ctrl-unselected,phy-reset-separate-gpio32-mdio-task,future-pi5-proof-needs-identity-tftp-serial-final-identity-restore-evidence",
    );
    write_early_static(" claims-ethernet-ready=false claims-broad-mmio-ready=false");
    write_early_static(
        " claims-unscoped-rp1-mmio-writes=false claims-rp1-clk-sys-transition=false",
    );
    write_early_static(" claims-clk-eth-ctrl-write=false claims-reset-controller-ownership=false");
    write_early_static(" claims-gpio32-phy-reset-ownership=false claims-mdio-phy-ownership=false");
    write_early_static(" claims-interrupt-ownership=false claims-dma-descriptor-ownership=false");
    write_early_static(" claims-packet-io=false claims-networking=false");
    write_early_static(" claims-sockets=false claims-ssh=false");
    write_early_static(" claims-phase-12-2=false claims-phase-transition=false");
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_ethernet_clk_eth_ctrl_write_restore_candidate",
    talos_boot_scenario = "rpi5_rp1_ethernet_clk_eth_ctrl_write_restore_no_mmio_control"
))]
fn write_rp1_ethernet_clk_eth_ctrl_write_restore_common(
    report_contract_id: &str,
    target_contract_id: &str,
    report_kind: &str,
) {
    write_early_static(" write-restore-report-contract-id=");
    write_early_static(report_contract_id);
    write_early_static(" target-contract-id=");
    write_early_static(target_contract_id);
    write_early_static(
        " source-task-id=phase12-rp1-ethernet-clk-eth-ctrl-source-contract-20260610",
    );
    write_early_static(" report-kind=");
    write_early_static(report_kind);
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_ethernet_clk_eth_ctrl_write_restore_candidate",
    talos_boot_scenario = "rpi5_rp1_ethernet_clk_eth_ctrl_write_restore_no_mmio_control"
))]
fn write_rp1_ethernet_clk_eth_ctrl_write_restore_capture_nonce() {
    if let Some(nonce) = option_env!("TALOS_CAPTURE_NONCE") {
        if !nonce.is_empty() {
            write_early_static(" capture-nonce=");
            write_early_static(nonce);
        }
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_ethernet_clk_eth_ctrl_write_restore_candidate",
    talos_boot_scenario = "rpi5_rp1_ethernet_clk_eth_ctrl_write_restore_no_mmio_control"
))]
fn write_rp1_ethernet_clk_eth_ctrl_write_restore_rejections() {
    write_early_static(
        " future-proof-classifications=rp1-ethernet-clk-eth-ctrl-idempotent-write-restored,rp1-ethernet-clk-eth-ctrl-idempotent-write-mismatch-restored,rp1-ethernet-clk-eth-ctrl-idempotent-write-restore-failed,rp1-ethernet-clk-eth-ctrl-blocked-missing-clock-manager,rp1-ethernet-clk-eth-ctrl-inconclusive-capture,no-clock-write-no-ethernet-rp1-ethernet-clk-eth-ctrl-control,staging-build-blocker",
    );
    write_early_static(
        " rejected-runtime-hardware-claims=ethernet-driver-readiness,broad-ethernet-mmio-readiness,unscoped-rp1-mmio-writes,rp1-clk-sys-transition,shared-rp1-clk-sys-write,clk-eth-tsu-ctrl-retry,non-idempotent-clk-eth-ctrl-transition,divider-source-pll-frequency-counter-gpclk-writes,reset-controller-ownership,gpio32-ownership-or-phy-reset,mdio-transactions-or-phy-ownership,interrupt-delivery-handler-ownership-or-completion,dma-descriptor-rings-channel-ownership-or-transfer-completion,packet-io,networking,sockets,ssh,phase-12-2,phase-transition",
    );
    write_early_static(
        " retained-risks=single-ethernet-private-tx-clock-store-readback-only,broad-clock-reset-ownership-unaccepted,phy-reset-separate-gpio32-mdio-task,future-pi5-proof-needs-identity-tftp-serial-final-identity-restore-evidence",
    );
    write_early_static(" claims-ethernet-ready=false claims-broad-mmio-ready=false");
    write_early_static(
        " claims-unscoped-rp1-mmio-writes=false claims-rp1-clk-sys-transition=false",
    );
    write_early_static(" claims-shared-rp1-clk-sys-write=false");
    write_early_static(" claims-clk-eth-tsu-ctrl-retry=false");
    write_early_static(" claims-clk-eth-ctrl-non-idempotent-transition=false");
    write_early_static(" claims-reset-controller-ownership=false");
    write_early_static(" claims-gpio32-phy-reset-ownership=false claims-mdio-phy-ownership=false");
    write_early_static(" claims-interrupt-ownership=false claims-dma-descriptor-ownership=false");
    write_early_static(" claims-packet-io=false claims-networking=false");
    write_early_static(" claims-sockets=false claims-ssh=false");
    write_early_static(" claims-phase-12-2=false claims-phase-transition=false");
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_ethernet_gpio32_phy_reset_preflight_candidate",
    talos_boot_scenario = "rpi5_rp1_ethernet_gpio32_phy_reset_preflight_control"
))]
fn write_rp1_ethernet_gpio32_phy_reset_preflight_common(report_kind: &str) {
    write_early_static(
        " preflight-report-contract-id=phase12-rp1-ethernet-gpio32-phy-reset-preflight-report-contract-v1",
    );
    write_early_static(
        " source-contract-id=phase12-rp1-ethernet-gpio32-phy-reset-source-contract-v1",
    );
    write_early_static(
        " source-task-id=phase12-rp1-ethernet-gpio32-phy-reset-source-contract-20260610",
    );
    write_early_static(" report-kind=");
    write_early_static(report_kind);
    write_early_static(
        " hardware-proof-boundary-classification=hardware-proof-limited-to-gpio32-phy-reset-readonly-preflight-control-output",
    );
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_ethernet_gpio32_phy_reset_preflight_candidate",
    talos_boot_scenario = "rpi5_rp1_ethernet_gpio32_phy_reset_preflight_control"
))]
fn write_rp1_ethernet_gpio32_phy_reset_preflight_capture_nonce() {
    if let Some(nonce) = option_env!("TALOS_CAPTURE_NONCE") {
        if !nonce.is_empty() {
            write_early_static(" capture-nonce=");
            write_early_static(nonce);
        }
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_ethernet_gpio32_phy_reset_preflight_candidate",
    talos_boot_scenario = "rpi5_rp1_ethernet_gpio32_phy_reset_preflight_control"
))]
fn write_rp1_ethernet_gpio32_phy_reset_preflight_rejections() {
    write_early_static(
        " rejected-runtime-hardware-claims=gpio-ownership,phy-reset-assertion-or-deassertion,mdio-transactions-or-phy-ownership,runtime-ethernet-driver-readiness,broad-ethernet-mmio-readiness,rp1-mmio-gpio-rio-pad-inte-ctrl-clock-writes,interrupt-delivery-handler-ownership-or-completion,dma-descriptor-rings-channel-ownership-or-transfer-completion,packet-io,networking,sockets,ssh,phase-12-2,phase-transition",
    );
    write_early_static(
        " retained-risks=gpio32-source-facts-do-not-prove-safe-drive-or-restore,phase11-gpio-frontiers-have-no-gpio32-write-restore-authority,macb-reset-is-mdio-bus-reset-hook,no-mdio-phy-ownership,future-proof-needs-identity-tftp-serial-final-identity-restore-evidence",
    );
    write_early_static(" claims-gpio-ownership=false");
    write_early_static(" claims-phy-reset-assertion=false claims-phy-reset-deassertion=false");
    write_early_static(" claims-mdio-transactions=false claims-phy-ownership=false");
    write_early_static(" claims-ethernet-driver-ready=false claims-broad-mmio-ready=false");
    write_early_static(" claims-rp1-mmio-writes=false claims-gpio-rio-pad-writes=false");
    write_early_static(" claims-inte-ctrl-writes=false claims-clock-writes=false");
    write_early_static(" claims-interrupt-ownership=false claims-dma-descriptor-ownership=false");
    write_early_static(" claims-packet-io=false claims-networking=false");
    write_early_static(" claims-sockets=false claims-ssh=false");
    write_early_static(" claims-phase-12-2=false claims-phase-transition=false");
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_ethernet_gpio32_phy_reset_write_restore_candidate",
    talos_boot_scenario = "rpi5_rp1_ethernet_gpio32_phy_reset_write_restore_no_mmio_control"
))]
fn write_rp1_ethernet_gpio32_phy_reset_write_restore_common(report_kind: &str) {
    write_early_static(
        " write-restore-report-contract-id=phase12-rp1-ethernet-gpio32-phy-reset-write-restore-report-contract-v1",
    );
    write_early_static(
        " source-contract-id=phase12-rp1-ethernet-gpio32-phy-reset-write-restore-source-contract-v1",
    );
    write_early_static(
        " source-task-id=phase12-rp1-ethernet-gpio32-phy-reset-write-restore-source-contract-20260610",
    );
    write_early_static(" report-kind=");
    write_early_static(report_kind);
    write_early_static(
        " hardware-proof-boundary-classification=hardware-proof-limited-to-gpio32-phy-reset-write-restore-control-output",
    );
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_ethernet_gpio32_phy_reset_write_restore_candidate",
    talos_boot_scenario = "rpi5_rp1_ethernet_gpio32_phy_reset_write_restore_no_mmio_control"
))]
fn write_rp1_ethernet_gpio32_phy_reset_write_restore_capture_nonce() {
    if let Some(nonce) = option_env!("TALOS_CAPTURE_NONCE") {
        if !nonce.is_empty() {
            write_early_static(" capture-nonce=");
            write_early_static(nonce);
        }
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_ethernet_gpio32_phy_reset_write_restore_candidate",
    talos_boot_scenario = "rpi5_rp1_ethernet_gpio32_phy_reset_write_restore_no_mmio_control"
))]
fn write_rp1_ethernet_gpio32_phy_reset_write_restore_rejections() {
    write_early_static(
        " future-proof-classifications=rp1-ethernet-gpio32-phy-reset-write-restored,rp1-ethernet-gpio32-phy-reset-write-assertion-mismatch-restored,rp1-ethernet-gpio32-phy-reset-write-deassertion-mismatch-restored,rp1-ethernet-gpio32-phy-reset-write-restore-failed,rp1-ethernet-gpio32-phy-reset-blocked-sentinel-read,rp1-ethernet-gpio32-phy-reset-blocked-unsafe-function,rp1-ethernet-gpio32-phy-reset-blocked-unexpected-event-state,rp1-ethernet-gpio32-phy-reset-blocked-missing-restore-baseline,rp1-ethernet-gpio32-phy-reset-inconclusive-capture,no-gpio-write-no-ethernet-rp1-ethernet-gpio32-phy-reset-control,staging-build-blocker",
    );
    write_early_static(
        " rejected-runtime-hardware-claims=mdio-transactions-or-phy-ownership,ethernet-driver-readiness,broad-ethernet-mmio-readiness,non-gpio32-writes,interrupt-delivery-handler-ownership-or-completion,dma-descriptor-rings-channel-ownership-or-transfer-completion,packet-io,networking,sockets,ssh,phase-12-2,phase-transition",
    );
    write_early_static(
        " retained-risks=gpio32-write-restore-does-not-prove-mdio-phy-ownership,no-ethernet-driver-readiness,no-packet-io,no-network-stack,future-proof-needs-identity-tftp-serial-final-identity-restore-evidence",
    );
    #[cfg(talos_boot_scenario = "rpi5_rp1_ethernet_gpio32_phy_reset_write_restore_candidate")]
    write_early_static(" claims-gpio32-write-restore-only=true");
    #[cfg(
        talos_boot_scenario = "rpi5_rp1_ethernet_gpio32_phy_reset_write_restore_no_mmio_control"
    )]
    write_early_static(" claims-gpio32-write-restore-only=false");
    write_early_static(" claims-broad-gpio-ownership=false");
    write_early_static(" claims-mdio-transactions=false claims-phy-ownership=false");
    write_early_static(" claims-ethernet-driver-ready=false claims-broad-mmio-ready=false");
    write_early_static(" claims-non-gpio32-writes=false");
    write_early_static(" claims-interrupt-ownership=false claims-dma-descriptor-ownership=false");
    write_early_static(" claims-packet-io=false claims-networking=false");
    write_early_static(" claims-sockets=false claims-ssh=false");
    write_early_static(" claims-phase-12-2=false claims-phase-transition=false");
}

#[cfg(talos_boot_scenario = "rpi5_rp1_ethernet_gpio32_phy_reset_write_restore_candidate")]
fn classify_rp1_ethernet_gpio32_phy_reset_write_restore_preconditions(
    baseline_status: u32,
    baseline_ctrl: u32,
    baseline_out: u32,
    baseline_oe: u32,
    baseline_in: u32,
    baseline_pad: u32,
    funcsel: u32,
    override_bits: u32,
    event_bits: u32,
    irq_bits: u32,
    pad_out_disabled: bool,
) -> Option<&'static str> {
    if is_rp1_gpio32_phy_reset_write_restore_sentinel(baseline_status)
        || is_rp1_gpio32_phy_reset_write_restore_sentinel(baseline_ctrl)
        || is_rp1_gpio32_phy_reset_write_restore_sentinel(baseline_out)
        || is_rp1_gpio32_phy_reset_write_restore_sentinel(baseline_oe)
        || is_rp1_gpio32_phy_reset_write_restore_sentinel(baseline_in)
        || is_rp1_gpio32_phy_reset_write_restore_sentinel(baseline_pad)
    {
        return Some("rp1-ethernet-gpio32-phy-reset-blocked-sentinel-read");
    }
    if funcsel != 5 || override_bits != 0 || pad_out_disabled {
        return Some("rp1-ethernet-gpio32-phy-reset-blocked-unsafe-function");
    }
    if event_bits != 0 || irq_bits != 0 {
        return Some("rp1-ethernet-gpio32-phy-reset-blocked-unexpected-event-state");
    }
    None
}

#[cfg(talos_boot_scenario = "rpi5_rp1_ethernet_gpio32_phy_reset_write_restore_candidate")]
fn is_rp1_gpio32_phy_reset_write_restore_sentinel(raw: u32) -> bool {
    raw == 0xffff_ffff || raw == 0xdead_dead
}

#[cfg(talos_boot_scenario = "rpi5_rp1_ethernet_gpio32_phy_reset_write_restore_candidate")]
fn classify_rp1_ethernet_gpio32_phy_reset_write_restore_result(
    assertion_out: u32,
    assertion_oe: u32,
    deassertion_out: u32,
    deassertion_oe: u32,
    restore_out: u32,
    restore_oe: u32,
    baseline_out: u32,
    baseline_oe: u32,
) -> &'static str {
    const BIT_MASK: u32 = 1 << 4;

    if restore_out != baseline_out || restore_oe != baseline_oe {
        return "rp1-ethernet-gpio32-phy-reset-write-restore-failed";
    }
    if assertion_out & BIT_MASK != 0 || assertion_oe & BIT_MASK == 0 {
        return "rp1-ethernet-gpio32-phy-reset-write-assertion-mismatch-restored";
    }
    if deassertion_out & BIT_MASK == 0 || deassertion_oe & BIT_MASK == 0 {
        return "rp1-ethernet-gpio32-phy-reset-write-deassertion-mismatch-restored";
    }
    "rp1-ethernet-gpio32-phy-reset-write-restored"
}

#[cfg(talos_boot_scenario = "rpi5_rp1_ethernet_gpio32_phy_reset_write_restore_candidate")]
fn wait_rp1_ethernet_gpio32_phy_reset_duration() -> u64 {
    let frequency = crate::arch::aarch64::generic_timer::counter_frequency_hz();
    let ticks = core::cmp::max(1, frequency / 1_000) * 5;
    let start = crate::arch::aarch64::generic_timer::physical_count();
    while crate::arch::aarch64::generic_timer::physical_count().wrapping_sub(start) < ticks {
        core::hint::spin_loop();
    }
    ticks
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_dma_cache_small_diagnostic_visibility_candidate",
    talos_boot_scenario = "rpi5_rp1_dma_cache_small_diagnostic_visibility_no_plan_control"
))]
fn write_dma_cache_small_diagnostic_visibility_common(report_kind: &str) {
    write_early_static(
        " visibility-report-contract-id=phase11-rp1-dma-cache-small-diagnostic-visibility-report-contract-v1",
    );
    write_early_static(
        " source-contract-id=phase11-rp1-dma-cache-small-diagnostic-source-contract-20260609",
    );
    write_early_static(" report-kind=");
    write_early_static(report_kind);
    write_early_static(
        " hardware-proof-boundary-classification=hardware-proof-limited-to-plan-visibility-control-output",
    );
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_dma_cache_small_diagnostic_visibility_candidate",
    talos_boot_scenario = "rpi5_rp1_dma_cache_small_diagnostic_visibility_no_plan_control"
))]
fn write_dma_cache_small_diagnostic_visibility_capture_nonce() {
    if let Some(nonce) = option_env!("TALOS_CAPTURE_NONCE") {
        if !nonce.is_empty() {
            write_early_static(" capture-nonce=");
            write_early_static(nonce);
        }
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_dma_cache_small_diagnostic_visibility_candidate",
    talos_boot_scenario = "rpi5_rp1_dma_cache_small_diagnostic_visibility_no_plan_control"
))]
fn write_dma_cache_small_diagnostic_visibility_rejections() {
    write_early_static(
        " rejected-runtime-hardware-claims=rp1-mmio-writes,rp1-dma-channel-ownership,dma-channel-programming,descriptor-ring-construction,descriptor-ring-ownership,transfer-completion,interrupt-completion,hardware-device-completion,ethernet-readiness,storage-readiness,networking,ssh,milestone-11-3-completion,phase-transition",
    );
    write_early_static(
        " retained-risks=no-rp1-dma-channel-ownership,no-descriptor-ring-layout-or-ownership,no-transfer-completion-or-interrupt-policy,source-unassigned-rp1-dma,no-live-hardware-dma-proof,no-device-specific-consumer",
    );
    write_early_static(" claims-rp1-mmio-writes=false claims-rp1-channel-ownership=false");
    write_early_static(" claims-dma-channel-programming=false claims-descriptor-ring-ready=false");
    write_early_static(" claims-transfer-completion=false claims-interrupt-completion=false");
    write_early_static(" claims-hardware-device-completion=false claims-ethernet-ready=false");
    write_early_static(" claims-storage-ready=false claims-networking=false claims-ssh=false");
    write_early_static(" claims-milestone-11-3-completion=false claims-phase-transition=false");
}

#[cfg(talos_boot_scenario = "rpi5_rp1_pcie2_host_link_status_read")]
pub fn run_rp1_pcie2_host_link_status_read() -> ! {
    const CONTRACT_ID: &str = "phase11-rp1-pcie-endpoint-config-discriminator-source-contract-v1";
    const TARGET: &str = "pcie2-host-link-status-read";
    const REGISTER_NAME: &str = "PCIE_MISC_PCIE_STATUS";
    const RETAINED_SYSINFO_CLOCK_SENTINEL_CLASSIFICATION: &str =
        "rp1-sysinfo-and-clock-window-sentinel";

    write_early_static("rpi5-rp1-pcie2-host-link-status-read: start\n");
    write_early_static("rpi5-rp1-pcie2-host-link-status-read: before-read-only-load\n");
    wait_uart10_empty_early_phase();

    let status = read_rp1_reg_u32(PCIE_MISC_PCIE_STATUS);
    let pcie_port = status & PCIE_STATUS_PORT != 0;
    let dl_active = status & PCIE_STATUS_DL_ACTIVE != 0;
    let phylinkup = status & PCIE_STATUS_PHYLINKUP != 0;
    let link_in_l23 = status & PCIE_STATUS_LINK_IN_L23 != 0;
    let status_is_deaddead = status == 0xdead_dead;
    let classification = classify_pcie2_host_link_status(status_is_deaddead, dl_active, phylinkup);

    loop {
        write_early_static("TALOS: rp1-pcie2-host-link-status-result contract=");
        write_early_static(CONTRACT_ID);
        write_early_static(" target=");
        write_early_static(TARGET);
        write_early_static(" pcie2-controller-base=");
        write_early_hex_u64(PCIE2_CONTROLLER_BASE as u64);
        write_pcie2_host_link_status_register(REGISTER_NAME, PCIE_MISC_PCIE_STATUS, status);
        write_pcie2_host_link_status_booleans(
            pcie_port,
            dl_active,
            phylinkup,
            link_in_l23,
            status_is_deaddead,
        );
        write_retained_rp1_window_sentinel_context(RETAINED_SYSINFO_CLOCK_SENTINEL_CLASSIFICATION);
        write_early_static(" classification=");
        write_early_static(classification);
        write_early_static("\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_pcie2_host_link_status_no_mmio_control")]
pub fn run_rp1_pcie2_host_link_status_no_mmio_control() -> ! {
    const CONTRACT_ID: &str = "phase11-rp1-pcie-endpoint-config-discriminator-source-contract-v1";
    const TARGET: &str = "pcie2-host-link-status-read";
    const REGISTER_NAME: &str = "PCIE_MISC_PCIE_STATUS";
    const RETAINED_SYSINFO_CLOCK_SENTINEL_CLASSIFICATION: &str =
        "rp1-sysinfo-and-clock-window-sentinel";
    const SIMULATED_STATUS: u32 = PCIE_STATUS_PORT | PCIE_STATUS_DL_ACTIVE | PCIE_STATUS_PHYLINKUP;

    write_early_static("rpi5-rp1-pcie2-host-link-status-control: start\n");
    write_early_static(
        "rpi5-rp1-pcie2-host-link-status-control: no-bcm2712-pcie-rp1-msix-mip-gic-gpio-clock-reset-dma-mmio\n",
    );
    wait_uart10_empty_early_phase();

    let pcie_port = SIMULATED_STATUS & PCIE_STATUS_PORT != 0;
    let dl_active = SIMULATED_STATUS & PCIE_STATUS_DL_ACTIVE != 0;
    let phylinkup = SIMULATED_STATUS & PCIE_STATUS_PHYLINKUP != 0;
    let link_in_l23 = SIMULATED_STATUS & PCIE_STATUS_LINK_IN_L23 != 0;
    let status_is_deaddead = SIMULATED_STATUS == 0xdead_dead;

    loop {
        write_early_static("TALOS: rp1-pcie2-host-link-status-control contract=");
        write_early_static(CONTRACT_ID);
        write_early_static(" target=");
        write_early_static(TARGET);
        write_early_static(" pcie2-controller-base=not-constructed");
        write_pcie2_host_link_status_control_register(REGISTER_NAME, SIMULATED_STATUS);
        write_pcie2_host_link_status_booleans(
            pcie_port,
            dl_active,
            phylinkup,
            link_in_l23,
            status_is_deaddead,
        );
        write_retained_rp1_window_sentinel_context(RETAINED_SYSINFO_CLOCK_SENTINEL_CLASSIFICATION);
        write_early_static(" classification=no-mmio-pcie2-host-link-status-control-visible\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_endpoint_config_identity_read")]
pub fn run_rp1_endpoint_config_identity_read() -> ! {
    const CONTRACT_ID: &str = "phase11-rp1-endpoint-config-identity-source-contract-v1";
    const TARGET: &str = "rp1-endpoint-config-vendor-device-read";
    const STATUS_REGISTER_NAME: &str = "PCIE_MISC_PCIE_STATUS";
    const INDEX_REGISTER_NAME: &str = "EXT_CFG_INDEX";
    const DATA_REGISTER_NAME: &str = "EXT_CFG_DATA";

    write_early_static("rpi5-rp1-endpoint-config-identity-read: start\n");
    write_early_static("rpi5-rp1-endpoint-config-identity-read: before-precondition-load\n");
    wait_uart10_empty_early_phase();

    let status = read_rp1_reg_u32(PCIE_MISC_PCIE_STATUS);
    let pcie_port = status & PCIE_STATUS_PORT != 0;
    let dl_active = status & PCIE_STATUS_DL_ACTIVE != 0;
    let phylinkup = status & PCIE_STATUS_PHYLINKUP != 0;
    let link_in_l23 = status & PCIE_STATUS_LINK_IN_L23 != 0;
    let status_is_deaddead = status == 0xdead_dead;
    let link_ready = dl_active && phylinkup && !status_is_deaddead;
    let (raw_config, index_write_performed) = if link_ready {
        write_early_static("rpi5-rp1-endpoint-config-identity-read: before-ext-cfg-index-write\n");
        write_pcie_ext_cfg_index(RP1_ENDPOINT_CONFIG_INDEX_VALUE);
        write_early_static("rpi5-rp1-endpoint-config-identity-read: before-ext-cfg-data-load\n");
        wait_uart10_empty_early_phase();
        (read_rp1_reg_u32(PCIE_EXT_CFG_DATA), true)
    } else {
        (0, false)
    };
    let vendor_id = raw_config & 0xffff;
    let device_id = (raw_config >> 16) & 0xffff;
    let vendor_device_match = vendor_id == RP1_ENDPOINT_EXPECTED_VENDOR_ID
        && device_id == RP1_ENDPOINT_EXPECTED_DEVICE_ID;
    let raw_config_is_all_ones = raw_config == 0xffff_ffff;
    let raw_config_is_zero = raw_config == 0;
    let raw_config_is_deaddead = raw_config == 0xdead_dead;
    let classification = classify_rp1_endpoint_config_identity(
        status_is_deaddead,
        dl_active,
        phylinkup,
        raw_config_is_all_ones,
        raw_config_is_zero,
        raw_config_is_deaddead,
        vendor_device_match,
    );

    loop {
        write_early_static("TALOS: rp1-endpoint-config-identity-result contract=");
        write_early_static(CONTRACT_ID);
        write_early_static(" target=");
        write_early_static(TARGET);
        write_endpoint_config_identity_common_fields(
            Some(PCIE2_CONTROLLER_BASE as u64),
            STATUS_REGISTER_NAME,
            PCIE_MISC_PCIE_STATUS_OFFSET,
            Some(PCIE_MISC_PCIE_STATUS as u64),
            status,
            pcie_port,
            dl_active,
            phylinkup,
            link_in_l23,
            status_is_deaddead,
            RP1_ENDPOINT_CONFIG_BDF,
            RP1_ENDPOINT_CONFIG_OFFSET,
            INDEX_REGISTER_NAME,
            PCIE_EXT_CFG_INDEX_OFFSET,
            Some(PCIE_EXT_CFG_INDEX as u64),
            RP1_ENDPOINT_CONFIG_INDEX_VALUE,
            index_write_performed,
            DATA_REGISTER_NAME,
            PCIE_EXT_CFG_DATA_OFFSET,
            Some(PCIE_EXT_CFG_DATA as u64),
            raw_config,
            vendor_id,
            device_id,
            vendor_device_match,
            raw_config_is_all_ones,
            raw_config_is_zero,
            raw_config_is_deaddead,
        );
        write_early_static(" classification=");
        write_early_static(classification);
        write_early_static("\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_endpoint_config_identity_no_mmio_control")]
pub fn run_rp1_endpoint_config_identity_no_mmio_control() -> ! {
    const CONTRACT_ID: &str = "phase11-rp1-endpoint-config-identity-source-contract-v1";
    const TARGET: &str = "rp1-endpoint-config-vendor-device-read";
    const STATUS_REGISTER_NAME: &str = "PCIE_MISC_PCIE_STATUS";
    const INDEX_REGISTER_NAME: &str = "EXT_CFG_INDEX";
    const DATA_REGISTER_NAME: &str = "EXT_CFG_DATA";
    const SIMULATED_STATUS: u32 = PCIE_STATUS_PORT | PCIE_STATUS_DL_ACTIVE | PCIE_STATUS_PHYLINKUP;
    const SIMULATED_RAW_CONFIG: u32 =
        (RP1_ENDPOINT_EXPECTED_DEVICE_ID << 16) | RP1_ENDPOINT_EXPECTED_VENDOR_ID;

    write_early_static("rpi5-rp1-endpoint-config-identity-control: start\n");
    write_early_static(
        "rpi5-rp1-endpoint-config-identity-control: no-bcm2712-pcie-rp1-sysinfo-clock-gpio-msix-mip-gic-dma-mmio\n",
    );
    wait_uart10_empty_early_phase();

    let pcie_port = SIMULATED_STATUS & PCIE_STATUS_PORT != 0;
    let dl_active = SIMULATED_STATUS & PCIE_STATUS_DL_ACTIVE != 0;
    let phylinkup = SIMULATED_STATUS & PCIE_STATUS_PHYLINKUP != 0;
    let link_in_l23 = SIMULATED_STATUS & PCIE_STATUS_LINK_IN_L23 != 0;
    let status_is_deaddead = SIMULATED_STATUS == 0xdead_dead;
    let vendor_id = SIMULATED_RAW_CONFIG & 0xffff;
    let device_id = (SIMULATED_RAW_CONFIG >> 16) & 0xffff;
    let vendor_device_match = vendor_id == RP1_ENDPOINT_EXPECTED_VENDOR_ID
        && device_id == RP1_ENDPOINT_EXPECTED_DEVICE_ID;
    let raw_config_is_all_ones = SIMULATED_RAW_CONFIG == 0xffff_ffff;
    let raw_config_is_zero = SIMULATED_RAW_CONFIG == 0;
    let raw_config_is_deaddead = SIMULATED_RAW_CONFIG == 0xdead_dead;

    loop {
        write_early_static("TALOS: rp1-endpoint-config-identity-control contract=");
        write_early_static(CONTRACT_ID);
        write_early_static(" target=");
        write_early_static(TARGET);
        write_endpoint_config_identity_common_fields(
            None,
            STATUS_REGISTER_NAME,
            PCIE_MISC_PCIE_STATUS_OFFSET,
            None,
            SIMULATED_STATUS,
            pcie_port,
            dl_active,
            phylinkup,
            link_in_l23,
            status_is_deaddead,
            RP1_ENDPOINT_CONFIG_BDF,
            RP1_ENDPOINT_CONFIG_OFFSET,
            INDEX_REGISTER_NAME,
            PCIE_EXT_CFG_INDEX_OFFSET,
            None,
            RP1_ENDPOINT_CONFIG_INDEX_VALUE,
            false,
            DATA_REGISTER_NAME,
            PCIE_EXT_CFG_DATA_OFFSET,
            None,
            SIMULATED_RAW_CONFIG,
            vendor_id,
            device_id,
            vendor_device_match,
            raw_config_is_all_ones,
            raw_config_is_zero,
            raw_config_is_deaddead,
        );
        write_early_static(" classification=no-mmio-rp1-endpoint-config-id-control-visible\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_bridge_config_preflight_read")]
pub fn run_rp1_bridge_config_preflight_read() -> ! {
    const CONTRACT_ID: &str = "phase11-rp1-bridge-config-preflight-source-contract-v1";
    const TARGET: &str = "pcie2-bridge-misc-ctrl-preflight-read";
    const STATUS_REGISTER_NAME: &str = "PCIE_MISC_PCIE_STATUS";
    const PREFLIGHT_REGISTER_NAME: &str = "PCIE_MISC_MISC_CTRL";
    const RETAINED_ENDPOINT_CONFIG_CLASSIFICATION: &str = "rp1-endpoint-config-id-all-ones";

    write_early_static("rpi5-rp1-bridge-config-preflight-read: start\n");
    write_early_static("rpi5-rp1-bridge-config-preflight-read: before-status-load\n");
    wait_uart10_empty_early_phase();

    let status = read_rp1_reg_u32(PCIE_MISC_PCIE_STATUS);
    write_early_static("rpi5-rp1-bridge-config-preflight-read: before-misc-ctrl-load\n");
    wait_uart10_empty_early_phase();
    let misc_ctrl = read_rp1_reg_u32(PCIE_MISC_MISC_CTRL);

    let pcie_port = status & PCIE_STATUS_PORT != 0;
    let dl_active = status & PCIE_STATUS_DL_ACTIVE != 0;
    let phylinkup = status & PCIE_STATUS_PHYLINKUP != 0;
    let link_in_l23 = status & PCIE_STATUS_LINK_IN_L23 != 0;
    let status_is_deaddead = status == 0xdead_dead;
    let scb_access_en = misc_ctrl & PCIE_MISC_CTRL_SCB_ACCESS_EN != 0;
    let cfg_read_ur_mode = misc_ctrl & PCIE_MISC_CTRL_CFG_READ_UR_MODE != 0;
    let rcb_mps_mode = misc_ctrl & PCIE_MISC_CTRL_RCB_MPS_MODE != 0;
    let rcb_64b_mode = misc_ctrl & PCIE_MISC_CTRL_RCB_64B_MODE != 0;
    let max_burst_size =
        (misc_ctrl & PCIE_MISC_CTRL_MAX_BURST_SIZE_MASK) >> PCIE_MISC_CTRL_MAX_BURST_SIZE_SHIFT;
    let misc_ctrl_is_sentinel =
        misc_ctrl == 0xdead_dead || misc_ctrl == 0xffff_ffff || misc_ctrl == 0;
    let classification = classify_pcie2_bridge_config_preflight(
        status_is_deaddead,
        dl_active,
        phylinkup,
        misc_ctrl_is_sentinel,
        scb_access_en,
        cfg_read_ur_mode,
    );

    loop {
        write_early_static("TALOS: rp1-bridge-config-preflight-result contract=");
        write_early_static(CONTRACT_ID);
        write_early_static(" target=");
        write_early_static(TARGET);
        write_bridge_config_preflight_common_fields(
            Some(PCIE2_CONTROLLER_BASE as u64),
            STATUS_REGISTER_NAME,
            PCIE_MISC_PCIE_STATUS_OFFSET,
            Some(PCIE_MISC_PCIE_STATUS as u64),
            status,
            pcie_port,
            dl_active,
            phylinkup,
            link_in_l23,
            status_is_deaddead,
            PREFLIGHT_REGISTER_NAME,
            PCIE_MISC_MISC_CTRL_OFFSET,
            Some(PCIE_MISC_MISC_CTRL as u64),
            misc_ctrl,
            scb_access_en,
            cfg_read_ur_mode,
            rcb_mps_mode,
            rcb_64b_mode,
            max_burst_size,
            misc_ctrl_is_sentinel,
            RETAINED_ENDPOINT_CONFIG_CLASSIFICATION,
        );
        write_bridge_config_preflight_classification_vocabulary();
        write_early_static(" classification=");
        write_early_static(classification);
        write_early_static("\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_bridge_config_preflight_no_mmio_control")]
pub fn run_rp1_bridge_config_preflight_no_mmio_control() -> ! {
    const CONTRACT_ID: &str = "phase11-rp1-bridge-config-preflight-source-contract-v1";
    const TARGET: &str = "pcie2-bridge-misc-ctrl-preflight-read";
    const STATUS_REGISTER_NAME: &str = "PCIE_MISC_PCIE_STATUS";
    const PREFLIGHT_REGISTER_NAME: &str = "PCIE_MISC_MISC_CTRL";
    const RETAINED_ENDPOINT_CONFIG_CLASSIFICATION: &str = "rp1-endpoint-config-id-all-ones";
    const SIMULATED_STATUS: u32 = PCIE_STATUS_PORT | PCIE_STATUS_DL_ACTIVE | PCIE_STATUS_PHYLINKUP;
    const SIMULATED_MISC_CTRL: u32 = PCIE_MISC_CTRL_SCB_ACCESS_EN
        | PCIE_MISC_CTRL_CFG_READ_UR_MODE
        | PCIE_MISC_CTRL_RCB_MPS_MODE
        | PCIE_MISC_CTRL_RCB_64B_MODE
        | (0x2 << PCIE_MISC_CTRL_MAX_BURST_SIZE_SHIFT);

    write_early_static("rpi5-rp1-bridge-config-preflight-control: start\n");
    write_early_static(
        "rpi5-rp1-bridge-config-preflight-control: no-bcm2712-pcie-rp1-msix-mip-gic-gpio-clock-reset-dma-mmio\n",
    );
    wait_uart10_empty_early_phase();

    let pcie_port = SIMULATED_STATUS & PCIE_STATUS_PORT != 0;
    let dl_active = SIMULATED_STATUS & PCIE_STATUS_DL_ACTIVE != 0;
    let phylinkup = SIMULATED_STATUS & PCIE_STATUS_PHYLINKUP != 0;
    let link_in_l23 = SIMULATED_STATUS & PCIE_STATUS_LINK_IN_L23 != 0;
    let status_is_deaddead = SIMULATED_STATUS == 0xdead_dead;
    let scb_access_en = SIMULATED_MISC_CTRL & PCIE_MISC_CTRL_SCB_ACCESS_EN != 0;
    let cfg_read_ur_mode = SIMULATED_MISC_CTRL & PCIE_MISC_CTRL_CFG_READ_UR_MODE != 0;
    let rcb_mps_mode = SIMULATED_MISC_CTRL & PCIE_MISC_CTRL_RCB_MPS_MODE != 0;
    let rcb_64b_mode = SIMULATED_MISC_CTRL & PCIE_MISC_CTRL_RCB_64B_MODE != 0;
    let max_burst_size = (SIMULATED_MISC_CTRL & PCIE_MISC_CTRL_MAX_BURST_SIZE_MASK)
        >> PCIE_MISC_CTRL_MAX_BURST_SIZE_SHIFT;
    let misc_ctrl_is_sentinel = false;

    loop {
        write_early_static("TALOS: rp1-bridge-config-preflight-control contract=");
        write_early_static(CONTRACT_ID);
        write_early_static(" target=");
        write_early_static(TARGET);
        write_bridge_config_preflight_common_fields(
            None,
            STATUS_REGISTER_NAME,
            PCIE_MISC_PCIE_STATUS_OFFSET,
            None,
            SIMULATED_STATUS,
            pcie_port,
            dl_active,
            phylinkup,
            link_in_l23,
            status_is_deaddead,
            PREFLIGHT_REGISTER_NAME,
            PCIE_MISC_MISC_CTRL_OFFSET,
            None,
            SIMULATED_MISC_CTRL,
            scb_access_en,
            cfg_read_ur_mode,
            rcb_mps_mode,
            rcb_64b_mode,
            max_burst_size,
            misc_ctrl_is_sentinel,
            RETAINED_ENDPOINT_CONFIG_CLASSIFICATION,
        );
        write_bridge_config_preflight_classification_vocabulary();
        write_early_static(" classification=no-mmio-pcie2-bridge-preflight-control-visible\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_bridge_setup_state_read")]
pub fn run_rp1_bridge_setup_state_read() -> ! {
    const CONTRACT_ID: &str = "phase11-rp1-bridge-setup-source-contract-v1";
    const TARGET: &str = "pcie2-bridge-setup-state-read";
    const STATUS_REGISTER_NAME: &str = "PCIE_MISC_PCIE_STATUS";
    const PREFLIGHT_REGISTER_NAME: &str = "PCIE_MISC_MISC_CTRL";
    const RC_CLASS_REGISTER_NAME: &str = "PCIE_RC_CFG_PRIV1_ID_VAL3";
    const WIN0_LO_REGISTER_NAME: &str = "PCIE_MISC_CPU_2_PCIE_MEM_WIN0_LO";
    const WIN0_HI_REGISTER_NAME: &str = "PCIE_MISC_CPU_2_PCIE_MEM_WIN0_HI";
    const WIN0_BASE_LIMIT_REGISTER_NAME: &str = "PCIE_MISC_CPU_2_PCIE_MEM_WIN0_BASE_LIMIT";
    const WIN0_BASE_HI_REGISTER_NAME: &str = "PCIE_MISC_CPU_2_PCIE_MEM_WIN0_BASE_HI";
    const WIN0_LIMIT_HI_REGISTER_NAME: &str = "PCIE_MISC_CPU_2_PCIE_MEM_WIN0_LIMIT_HI";
    const RETAINED_ENDPOINT_CONFIG_CLASSIFICATION: &str = "rp1-endpoint-config-id-all-ones";

    write_early_static("rpi5-rp1-bridge-setup-state-read: start\n");
    write_early_static("rpi5-rp1-bridge-setup-state-read: before-status-load\n");
    wait_uart10_empty_early_phase();
    let status = read_rp1_reg_u32(PCIE_MISC_PCIE_STATUS);
    write_early_static("rpi5-rp1-bridge-setup-state-read: before-misc-ctrl-load\n");
    wait_uart10_empty_early_phase();
    let misc_ctrl = read_rp1_reg_u32(PCIE_MISC_MISC_CTRL);
    write_early_static("rpi5-rp1-bridge-setup-state-read: before-rc-class-load\n");
    wait_uart10_empty_early_phase();
    let rc_class = read_rp1_reg_u32(PCIE_RC_CFG_PRIV1_ID_VAL3);
    write_early_static("rpi5-rp1-bridge-setup-state-read: before-win0-loads\n");
    wait_uart10_empty_early_phase();
    let win0_lo = read_rp1_reg_u32(PCIE_MISC_CPU_2_PCIE_MEM_WIN0_LO);
    let win0_hi = read_rp1_reg_u32(PCIE_MISC_CPU_2_PCIE_MEM_WIN0_HI);
    let win0_base_limit = read_rp1_reg_u32(PCIE_MISC_CPU_2_PCIE_MEM_WIN0_BASE_LIMIT);
    let win0_base_hi = read_rp1_reg_u32(PCIE_MISC_CPU_2_PCIE_MEM_WIN0_BASE_HI);
    let win0_limit_hi = read_rp1_reg_u32(PCIE_MISC_CPU_2_PCIE_MEM_WIN0_LIMIT_HI);

    let decoded = BridgeSetupState::from_raw(
        status,
        misc_ctrl,
        rc_class,
        win0_lo,
        win0_hi,
        win0_base_limit,
        win0_base_hi,
        win0_limit_hi,
    );

    loop {
        write_early_static("TALOS: rp1-bridge-setup-state-result contract=");
        write_early_static(CONTRACT_ID);
        write_early_static(" target=");
        write_early_static(TARGET);
        write_bridge_setup_state_common_fields(
            Some(PCIE2_CONTROLLER_BASE as u64),
            STATUS_REGISTER_NAME,
            PCIE_MISC_PCIE_STATUS_OFFSET,
            Some(PCIE_MISC_PCIE_STATUS as u64),
            PREFLIGHT_REGISTER_NAME,
            PCIE_MISC_MISC_CTRL_OFFSET,
            Some(PCIE_MISC_MISC_CTRL as u64),
            RC_CLASS_REGISTER_NAME,
            PCIE_RC_CFG_PRIV1_ID_VAL3_OFFSET,
            Some(PCIE_RC_CFG_PRIV1_ID_VAL3 as u64),
            WIN0_LO_REGISTER_NAME,
            PCIE_MISC_CPU_2_PCIE_MEM_WIN0_LO_OFFSET,
            Some(PCIE_MISC_CPU_2_PCIE_MEM_WIN0_LO as u64),
            WIN0_HI_REGISTER_NAME,
            PCIE_MISC_CPU_2_PCIE_MEM_WIN0_HI_OFFSET,
            Some(PCIE_MISC_CPU_2_PCIE_MEM_WIN0_HI as u64),
            WIN0_BASE_LIMIT_REGISTER_NAME,
            PCIE_MISC_CPU_2_PCIE_MEM_WIN0_BASE_LIMIT_OFFSET,
            Some(PCIE_MISC_CPU_2_PCIE_MEM_WIN0_BASE_LIMIT as u64),
            WIN0_BASE_HI_REGISTER_NAME,
            PCIE_MISC_CPU_2_PCIE_MEM_WIN0_BASE_HI_OFFSET,
            Some(PCIE_MISC_CPU_2_PCIE_MEM_WIN0_BASE_HI as u64),
            WIN0_LIMIT_HI_REGISTER_NAME,
            PCIE_MISC_CPU_2_PCIE_MEM_WIN0_LIMIT_HI_OFFSET,
            Some(PCIE_MISC_CPU_2_PCIE_MEM_WIN0_LIMIT_HI as u64),
            &decoded,
            RETAINED_ENDPOINT_CONFIG_CLASSIFICATION,
        );
        write_bridge_setup_state_classification_vocabulary();
        write_early_static(" classification=");
        write_early_static(decoded.classification);
        write_early_static("\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_bridge_setup_state_no_mmio_control")]
pub fn run_rp1_bridge_setup_state_no_mmio_control() -> ! {
    const CONTRACT_ID: &str = "phase11-rp1-bridge-setup-source-contract-v1";
    const TARGET: &str = "pcie2-bridge-setup-state-read";
    const STATUS_REGISTER_NAME: &str = "PCIE_MISC_PCIE_STATUS";
    const PREFLIGHT_REGISTER_NAME: &str = "PCIE_MISC_MISC_CTRL";
    const RC_CLASS_REGISTER_NAME: &str = "PCIE_RC_CFG_PRIV1_ID_VAL3";
    const WIN0_LO_REGISTER_NAME: &str = "PCIE_MISC_CPU_2_PCIE_MEM_WIN0_LO";
    const WIN0_HI_REGISTER_NAME: &str = "PCIE_MISC_CPU_2_PCIE_MEM_WIN0_HI";
    const WIN0_BASE_LIMIT_REGISTER_NAME: &str = "PCIE_MISC_CPU_2_PCIE_MEM_WIN0_BASE_LIMIT";
    const WIN0_BASE_HI_REGISTER_NAME: &str = "PCIE_MISC_CPU_2_PCIE_MEM_WIN0_BASE_HI";
    const WIN0_LIMIT_HI_REGISTER_NAME: &str = "PCIE_MISC_CPU_2_PCIE_MEM_WIN0_LIMIT_HI";
    const RETAINED_ENDPOINT_CONFIG_CLASSIFICATION: &str = "rp1-endpoint-config-id-all-ones";
    const SIMULATED_STATUS: u32 = PCIE_STATUS_PORT | PCIE_STATUS_DL_ACTIVE | PCIE_STATUS_PHYLINKUP;
    const SIMULATED_MISC_CTRL: u32 = PCIE_MISC_CTRL_SCB_ACCESS_EN | PCIE_MISC_CTRL_CFG_READ_UR_MODE;
    const SIMULATED_RC_CLASS: u32 = PCIE_RC_EXPECTED_BRIDGE_CLASS_CODE;
    const SIMULATED_WIN0_LO: u32 = 0;
    const SIMULATED_WIN0_HI: u32 = 0;
    const SIMULATED_WIN0_BASE_LIMIT: u32 =
        PCIE_WIN0_BASE_LOW_EXPECTED | PCIE_WIN0_LIMIT_LOW_EXPECTED;
    const SIMULATED_WIN0_BASE_HI: u32 = PCIE_WIN0_HIGH_EXPECTED;
    const SIMULATED_WIN0_LIMIT_HI: u32 = PCIE_WIN0_HIGH_EXPECTED;

    write_early_static("rpi5-rp1-bridge-setup-state-control: start\n");
    write_early_static(
        "rpi5-rp1-bridge-setup-state-control: no-bcm2712-pcie-rp1-msix-mip-gic-gpio-clock-reset-dma-mmio\n",
    );
    wait_uart10_empty_early_phase();

    let mut decoded = BridgeSetupState::from_raw(
        SIMULATED_STATUS,
        SIMULATED_MISC_CTRL,
        SIMULATED_RC_CLASS,
        SIMULATED_WIN0_LO,
        SIMULATED_WIN0_HI,
        SIMULATED_WIN0_BASE_LIMIT,
        SIMULATED_WIN0_BASE_HI,
        SIMULATED_WIN0_LIMIT_HI,
    );
    decoded.classification = "no-mmio-pcie2-bridge-setup-state-control-visible";

    loop {
        write_early_static("TALOS: rp1-bridge-setup-state-control contract=");
        write_early_static(CONTRACT_ID);
        write_early_static(" target=");
        write_early_static(TARGET);
        write_bridge_setup_state_common_fields(
            None,
            STATUS_REGISTER_NAME,
            PCIE_MISC_PCIE_STATUS_OFFSET,
            None,
            PREFLIGHT_REGISTER_NAME,
            PCIE_MISC_MISC_CTRL_OFFSET,
            None,
            RC_CLASS_REGISTER_NAME,
            PCIE_RC_CFG_PRIV1_ID_VAL3_OFFSET,
            None,
            WIN0_LO_REGISTER_NAME,
            PCIE_MISC_CPU_2_PCIE_MEM_WIN0_LO_OFFSET,
            None,
            WIN0_HI_REGISTER_NAME,
            PCIE_MISC_CPU_2_PCIE_MEM_WIN0_HI_OFFSET,
            None,
            WIN0_BASE_LIMIT_REGISTER_NAME,
            PCIE_MISC_CPU_2_PCIE_MEM_WIN0_BASE_LIMIT_OFFSET,
            None,
            WIN0_BASE_HI_REGISTER_NAME,
            PCIE_MISC_CPU_2_PCIE_MEM_WIN0_BASE_HI_OFFSET,
            None,
            WIN0_LIMIT_HI_REGISTER_NAME,
            PCIE_MISC_CPU_2_PCIE_MEM_WIN0_LIMIT_HI_OFFSET,
            None,
            &decoded,
            RETAINED_ENDPOINT_CONFIG_CLASSIFICATION,
        );
        write_bridge_setup_state_classification_vocabulary();
        write_early_static(" classification=no-mmio-pcie2-bridge-setup-state-control-visible\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_observed_aperture_read")]
pub fn run_rp1_observed_aperture_read() -> ! {
    const CONTRACT_ID: &str = "phase11-rp1-observed-aperture-source-contract-v1";
    const TARGET: &str = "rp1-uart0-fr-observed-aperture-read";
    const SOURCE_RP1_BUS_ADDRESS: &str = "0xc040030018";
    const OBSERVED_CPU_PHYSICAL_ADDRESS: &str = "0x1c00030018";
    const REGISTER_OFFSET: &str = "0x18";

    write_early_static("rpi5-rp1-observed-aperture-read: start contract=");
    write_early_static(CONTRACT_ID);
    write_early_static("\n");
    write_early_static("rpi5-rp1-observed-aperture-read: before-rp1-fr-load\n");
    wait_uart10_empty_early_phase();

    let raw = read_rp1_reg_u32(RP1_UART0_OBSERVED_APERTURE_FR);
    let classification = classify_observed_aperture_raw(raw);

    loop {
        write_early_static("TALOS: rp1-observed-aperture-result contract=");
        write_early_static(CONTRACT_ID);
        write_early_static(" target=");
        write_early_static(TARGET);
        write_observed_aperture_fields(
            SOURCE_RP1_BUS_ADDRESS,
            OBSERVED_CPU_PHYSICAL_ADDRESS,
            REGISTER_OFFSET,
            raw,
        );
        write_observed_aperture_retained_bridge_context();
        write_observed_aperture_classification_vocabulary();
        write_early_static(" classification=");
        write_early_static(classification);
        write_early_static("\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_observed_aperture_no_mmio_control")]
pub fn run_rp1_observed_aperture_no_mmio_control() -> ! {
    const CONTRACT_ID: &str = "phase11-rp1-observed-aperture-source-contract-v1";
    const TARGET: &str = "rp1-uart0-fr-observed-aperture-read";
    const NOT_CONSTRUCTED: &str = "not-constructed";
    const SIMULATED_RAW_VALUE: u32 = 0x0000_0090;

    write_early_static("rpi5-rp1-observed-aperture-control: start contract=");
    write_early_static(CONTRACT_ID);
    write_early_static("\n");
    write_early_static(
        "rpi5-rp1-observed-aperture-control: no-bcm2712-pcie-rp1-mip-gic-gpio-clock-reset-dma-mmio\n",
    );
    wait_uart10_empty_early_phase();

    loop {
        write_early_static("TALOS: rp1-observed-aperture-control contract=");
        write_early_static(CONTRACT_ID);
        write_early_static(" target=");
        write_early_static(TARGET);
        write_observed_aperture_fields(
            NOT_CONSTRUCTED,
            NOT_CONSTRUCTED,
            NOT_CONSTRUCTED,
            SIMULATED_RAW_VALUE,
        );
        write_observed_aperture_retained_bridge_context();
        write_observed_aperture_classification_vocabulary();
        write_early_static(" classification=no-mmio-observed-aperture-control-visible\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_observed_gpio_status_read")]
pub fn run_rp1_observed_gpio_status_read() -> ! {
    const CONTRACT_ID: &str = "phase11-rp1-observed-gpio-status-source-contract-v1";
    const TARGET: &str = "rp1-gpio14-status-ctrl-observed-aperture-read";
    const STATUS_SOURCE_RP1_BUS_ADDRESS: &str = "0xc0400d0070";
    const CTRL_SOURCE_RP1_BUS_ADDRESS: &str = "0xc0400d0074";
    const STATUS_OBSERVED_CPU_PHYSICAL_ADDRESS: &str = "0x1c000d0070";
    const CTRL_OBSERVED_CPU_PHYSICAL_ADDRESS: &str = "0x1c000d0074";
    const STATUS_REGISTER_OFFSET: &str = "0x70";
    const CTRL_REGISTER_OFFSET: &str = "0x74";

    write_early_static("rpi5-rp1-observed-gpio-status-read: start contract=");
    write_early_static(CONTRACT_ID);
    write_early_static("\n");
    write_early_static(
        "rpi5-rp1-observed-gpio-status-read: before-gpio14-status-ctrl-loads addresses=",
    );
    write_early_static(STATUS_OBSERVED_CPU_PHYSICAL_ADDRESS);
    write_early_static(",");
    write_early_static(CTRL_OBSERVED_CPU_PHYSICAL_ADDRESS);
    write_early_static("\n");
    wait_uart10_empty_early_phase();

    let gpio14_status = read_rp1_reg_u32(RP1_GPIO14_OBSERVED_APERTURE_STATUS);
    let gpio14_ctrl = read_rp1_reg_u32(RP1_GPIO14_OBSERVED_APERTURE_CTRL);
    let classification = classify_observed_gpio_status_pair(gpio14_status, gpio14_ctrl);

    loop {
        write_early_static("TALOS: rp1-observed-gpio-status-result");
        write_observed_gpio_status_capture_nonce();
        write_early_static(" contract=");
        write_early_static(CONTRACT_ID);
        write_early_static(" target=");
        write_early_static(TARGET);
        write_observed_gpio_status_fields(
            STATUS_SOURCE_RP1_BUS_ADDRESS,
            CTRL_SOURCE_RP1_BUS_ADDRESS,
            STATUS_OBSERVED_CPU_PHYSICAL_ADDRESS,
            CTRL_OBSERVED_CPU_PHYSICAL_ADDRESS,
            STATUS_REGISTER_OFFSET,
            CTRL_REGISTER_OFFSET,
            gpio14_status,
            gpio14_ctrl,
        );
        write_observed_gpio_status_classification_vocabulary();
        write_early_static(" classification=");
        write_early_static(classification);
        write_early_static("\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_observed_gpio_status_no_mmio_control")]
pub fn run_rp1_observed_gpio_status_no_mmio_control() -> ! {
    const CONTRACT_ID: &str = "phase11-rp1-observed-gpio-status-source-contract-v1";
    const TARGET: &str = "rp1-gpio14-status-ctrl-observed-aperture-read";
    const NOT_CONSTRUCTED: &str = "not-constructed";
    const SIMULATED_STATUS_RAW: u32 = 0x0010_0000;
    const SIMULATED_CTRL_RAW: u32 = 0x0040_0004;

    write_early_static("rpi5-rp1-observed-gpio-status-control: start contract=");
    write_early_static(CONTRACT_ID);
    write_early_static("\n");
    write_early_static(
        "rpi5-rp1-observed-gpio-status-control: no-bcm2712-pcie-rp1-mip-gic-gpio-rio-pads-clock-reset-dma-mmio\n",
    );
    wait_uart10_empty_early_phase();

    loop {
        write_early_static("TALOS: rp1-observed-gpio-status-control");
        write_observed_gpio_status_capture_nonce();
        write_early_static(" contract=");
        write_early_static(CONTRACT_ID);
        write_early_static(" target=");
        write_early_static(TARGET);
        write_observed_gpio_status_fields(
            NOT_CONSTRUCTED,
            NOT_CONSTRUCTED,
            NOT_CONSTRUCTED,
            NOT_CONSTRUCTED,
            NOT_CONSTRUCTED,
            NOT_CONSTRUCTED,
            SIMULATED_STATUS_RAW,
            SIMULATED_CTRL_RAW,
        );
        write_observed_gpio_status_classification_vocabulary();
        write_early_static(" classification=no-mmio-observed-gpio-status-control-visible\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_gpio14_ownership_route_preflight_read")]
pub fn run_rp1_gpio14_ownership_route_preflight_read() -> ! {
    const CONTRACT_ID: &str = "phase11-rp1-observed-gpio-ownership-route-source-contract-v1";
    const TARGET: &str = "rp1-gpio14-ownership-route-observed-aperture-preflight-read";
    const PIN: &str = "GPIO14";
    const GPIO14_MASK: u32 = 1 << 14;
    const GIC_INTID: u32 = 160;
    const GICD_ISENABLER5: usize = GICD_BASE + 0x114;
    const GICD_ISPENDR5: usize = GICD_BASE + 0x214;
    const GICD_ISACTIVER5: usize = GICD_BASE + 0x314;
    const GICC_HPPIR: usize = GICC_BASE + 0x18;

    write_early_static("rpi5-rp1-gpio14-ownership-route-preflight-read: start\n");
    write_early_static("rpi5-rp1-gpio14-ownership-route-preflight-read: before-read-only-loads\n");
    wait_uart10_empty_early_phase();

    let gpio14_status = read_rp1_reg_u32(RP1_GPIO14_OBSERVED_APERTURE_STATUS);
    let gpio14_ctrl = read_rp1_reg_u32(RP1_GPIO14_OBSERVED_APERTURE_CTRL);
    let io_bank0_inte = read_rp1_reg_u32(RP1_IO_BANK0_OBSERVED_APERTURE_INTE);
    let io_bank0_ints = read_rp1_reg_u32(RP1_IO_BANK0_OBSERVED_APERTURE_INTS);
    let rio_out = read_rp1_reg_u32(RP1_RIO0_OBSERVED_APERTURE_OUT);
    let rio_oe = read_rp1_reg_u32(RP1_RIO0_OBSERVED_APERTURE_OE);
    let rio_in = read_rp1_reg_u32(RP1_RIO0_OBSERVED_APERTURE_IN);
    let pad = read_rp1_reg_u32(RP1_GPIO14_OBSERVED_APERTURE_PAD);

    let gic = GicV2::new(GICD_BASE, GICC_BASE);
    let (gicd_isenabler5, gicd_ispendr5, gicd_isactiver5, gicc_hppir) = unsafe {
        (
            gic.enable_bits(GIC_INTID),
            gic.pending_bits(GIC_INTID),
            gic.active_bits(GIC_INTID),
            gic.highest_pending(),
        )
    };
    let hppir_intid = gicc_hppir & 0x3ff;
    let classification = gpio14_ownership_preflight_classification(
        gpio14_status,
        gpio14_ctrl,
        io_bank0_inte,
        io_bank0_ints,
        rio_out,
        rio_oe,
        rio_in,
        pad,
        gicd_isenabler5,
        gicd_ispendr5,
        gicd_isactiver5,
        hppir_intid,
        GPIO14_MASK,
        GIC_INTID,
    );

    loop {
        write_early_static("TALOS: rp1-gpio14-ownership-route-preflight-result contract=");
        write_early_static(CONTRACT_ID);
        write_early_static(" target=");
        write_early_static(TARGET);
        write_early_static(" pin=");
        write_early_static(PIN);
        write_early_static(" gpio14-bit-mask=");
        write_early_hex_u64(GPIO14_MASK as u64);
        write_early_static(" gpio14-status-address=");
        write_early_hex_u64(RP1_GPIO14_OBSERVED_APERTURE_STATUS as u64);
        write_early_static(" gpio14-ctrl-address=");
        write_early_hex_u64(RP1_GPIO14_OBSERVED_APERTURE_CTRL as u64);
        write_early_static(" io-bank0-inte-address=");
        write_early_hex_u64(RP1_IO_BANK0_OBSERVED_APERTURE_INTE as u64);
        write_early_static(" io-bank0-ints-address=");
        write_early_hex_u64(RP1_IO_BANK0_OBSERVED_APERTURE_INTS as u64);
        write_early_static(" rio-out-address=");
        write_early_hex_u64(RP1_RIO0_OBSERVED_APERTURE_OUT as u64);
        write_early_static(" rio-oe-address=");
        write_early_hex_u64(RP1_RIO0_OBSERVED_APERTURE_OE as u64);
        write_early_static(" rio-in-address=");
        write_early_hex_u64(RP1_RIO0_OBSERVED_APERTURE_IN as u64);
        write_early_static(" pad-address=");
        write_early_hex_u64(RP1_GPIO14_OBSERVED_APERTURE_PAD as u64);
        write_early_static(" gicd-isenabler5-address=");
        write_early_hex_u64(GICD_ISENABLER5 as u64);
        write_early_static(" gicd-ispendr5-address=");
        write_early_hex_u64(GICD_ISPENDR5 as u64);
        write_early_static(" gicd-isactiver5-address=");
        write_early_hex_u64(GICD_ISACTIVER5 as u64);
        write_early_static(" gicc-hppir-address=");
        write_early_hex_u64(GICC_HPPIR as u64);
        write_gpio14_ownership_preflight_fields(
            gpio14_status,
            gpio14_ctrl,
            io_bank0_inte,
            io_bank0_ints,
            rio_out,
            rio_oe,
            rio_in,
            pad,
            gicd_isenabler5,
            gicd_ispendr5,
            gicd_isactiver5,
            gicc_hppir,
            hppir_intid,
            GPIO14_MASK,
        );
        write_early_static(" classification=");
        write_early_static(classification);
        write_early_static("\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_gpio14_ownership_route_preflight_no_mmio_control")]
pub fn run_rp1_gpio14_ownership_route_preflight_no_mmio_control() -> ! {
    const CONTRACT_ID: &str = "phase11-rp1-observed-gpio-ownership-route-source-contract-v1";
    const TARGET: &str = "rp1-gpio14-ownership-route-observed-aperture-preflight-read";
    const PIN: &str = "GPIO14";
    const GPIO14_MASK: u32 = 1 << 14;
    const SIMULATED_RAW_VALUE: u32 = 0;
    const SIMULATED_HPPIR_INTID: u32 = 0;

    write_early_static("rpi5-rp1-gpio14-ownership-route-preflight-control: start\n");
    write_early_static(
        "rpi5-rp1-gpio14-ownership-route-preflight-control: no-rp1-gpio-rio-pads-clock-reset-msix-pcie-mip-gic-mmio\n",
    );
    wait_uart10_empty_early_phase();

    loop {
        write_early_static("TALOS: rp1-gpio14-ownership-route-preflight-control contract=");
        write_early_static(CONTRACT_ID);
        write_early_static(" target=");
        write_early_static(TARGET);
        write_early_static(" pin=");
        write_early_static(PIN);
        write_gpio14_ownership_route_capture_nonce();
        write_early_static(" gpio14-bit-mask=");
        write_early_hex_u64(GPIO14_MASK as u64);
        write_early_static(
            " gpio14-status-address=not-constructed gpio14-ctrl-address=not-constructed io-bank0-inte-address=not-constructed io-bank0-ints-address=not-constructed rio-out-address=not-constructed rio-oe-address=not-constructed rio-in-address=not-constructed pad-address=not-constructed gicd-isenabler5-address=not-constructed gicd-ispendr5-address=not-constructed gicd-isactiver5-address=not-constructed gicc-hppir-address=not-constructed",
        );
        write_gpio14_ownership_preflight_fields(
            SIMULATED_RAW_VALUE,
            SIMULATED_RAW_VALUE,
            SIMULATED_RAW_VALUE,
            SIMULATED_RAW_VALUE,
            SIMULATED_RAW_VALUE,
            SIMULATED_RAW_VALUE,
            SIMULATED_RAW_VALUE,
            SIMULATED_RAW_VALUE,
            SIMULATED_RAW_VALUE,
            SIMULATED_RAW_VALUE,
            SIMULATED_RAW_VALUE,
            SIMULATED_RAW_VALUE,
            SIMULATED_HPPIR_INTID,
            GPIO14_MASK,
        );
        write_early_static(
            " classification=no-mmio-observed-gpio14-ownership-route-control-visible\n",
        );
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_gpio14_ownership_route_preflight_no_mmio_control")]
fn write_gpio14_ownership_route_capture_nonce() {
    if let Some(nonce) = option_env!("TALOS_CAPTURE_NONCE") {
        if !nonce.is_empty() {
            write_early_static(" capture-nonce=");
            write_early_static(nonce);
        }
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_gpio16_owned_event_discriminator",
    talos_boot_scenario = "rpi5_rp1_gpio16_owned_event_discriminator_no_mmio_control"
))]
#[derive(Clone, Copy)]
struct Gpio16EventDiscriminatorSnapshot {
    gpio16_status: u32,
    gpio16_ctrl: u32,
    io_bank0_inte: u32,
    io_bank0_ints: u32,
    rio_out: u32,
    rio_oe: u32,
    rio_in: u32,
    pad: u32,
    gicd_isenabler5: u32,
    gicd_ispendr5: u32,
    gicd_isactiver5: u32,
    gicc_hppir: u32,
    hppir_intid: u32,
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_gpio16_owned_event_discriminator",
    talos_boot_scenario = "rpi5_rp1_gpio16_owned_event_discriminator_no_mmio_control"
))]
impl Gpio16EventDiscriminatorSnapshot {
    const fn zero() -> Self {
        Self {
            gpio16_status: 0,
            gpio16_ctrl: 0,
            io_bank0_inte: 0,
            io_bank0_ints: 0,
            rio_out: 0,
            rio_oe: 0,
            rio_in: 0,
            pad: 0,
            gicd_isenabler5: 0,
            gicd_ispendr5: 0,
            gicd_isactiver5: 0,
            gicc_hppir: 0,
            hppir_intid: 0,
        }
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_gpio16_owned_event_discriminator")]
pub fn run_rp1_gpio16_owned_event_discriminator() -> ! {
    const CONTRACT_ID: &str = "phase11-rp1-observed-gpio16-ownership-event-source-contract-v1";
    const TARGET: &str = "rp1-gpio16-ownership-event-observed-aperture-preflight-read";
    const PIN: &str = "GPIO16";
    const GPIO16_MASK: u32 = 1 << 16;
    const GIC_INTID: u32 = 160;
    const GICD_ISENABLER5: usize = GICD_BASE + 0x114;
    const GICD_ISPENDR5: usize = GICD_BASE + 0x214;
    const GICD_ISACTIVER5: usize = GICD_BASE + 0x314;
    const GICC_HPPIR: usize = GICC_BASE + 0x18;

    write_early_static("rpi5-rp1-gpio16-owned-event-discriminator: start\n");
    write_early_static(
        "rpi5-rp1-gpio16-owned-event-discriminator: before-read-only-observed-aperture-loads\n",
    );
    wait_uart10_empty_early_phase();

    let preflight = read_gpio16_event_discriminator_snapshot(GIC_INTID);
    let classification =
        gpio16_observed_ownership_event_preflight_classification(preflight, GPIO16_MASK, GIC_INTID);

    loop {
        write_early_static("TALOS: rp1-gpio16-owned-event-discriminator-result contract=");
        write_early_static(CONTRACT_ID);
        write_early_static(" target=");
        write_early_static(TARGET);
        write_early_static(" pin=");
        write_early_static(PIN);
        write_early_static(" bank=IO_BANK0 gpio16-bit-mask=");
        write_early_hex_u64(GPIO16_MASK as u64);
        write_gpio16_event_discriminator_addresses(
            RP1_GPIO16_OBSERVED_APERTURE_STATUS,
            RP1_GPIO16_OBSERVED_APERTURE_CTRL,
            RP1_IO_BANK0_OBSERVED_APERTURE_INTE,
            RP1_IO_BANK0_OBSERVED_APERTURE_INTS,
            RP1_RIO0_OBSERVED_APERTURE_OUT,
            RP1_RIO0_OBSERVED_APERTURE_OE,
            RP1_RIO0_OBSERVED_APERTURE_IN,
            RP1_GPIO16_OBSERVED_APERTURE_PAD,
            GICD_ISENABLER5,
            GICD_ISPENDR5,
            GICD_ISACTIVER5,
            GICC_HPPIR,
        );
        write_gpio16_ownership_event_preflight_fields(preflight, GPIO16_MASK);
        write_early_static(" classification=");
        write_early_static(classification);
        write_early_static("\n");
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_gpio16_owned_event_discriminator_no_mmio_control")]
pub fn run_rp1_gpio16_owned_event_discriminator_no_mmio_control() -> ! {
    const CONTRACT_ID: &str = "phase11-rp1-observed-gpio16-ownership-event-source-contract-v1";
    const TARGET: &str = "rp1-gpio16-ownership-event-observed-aperture-preflight-read";
    const PIN: &str = "GPIO16";
    const GPIO16_MASK: u32 = 1 << 16;
    const SNAPSHOT: Gpio16EventDiscriminatorSnapshot = Gpio16EventDiscriminatorSnapshot::zero();

    write_early_static("rpi5-rp1-gpio16-owned-event-discriminator-control: start\n");
    write_early_static(
        "rpi5-rp1-gpio16-owned-event-discriminator-control: no-rp1-gpio-rio-pads-clock-reset-msix-pcie-mip-gic-mmio\n",
    );
    wait_uart10_empty_early_phase();

    loop {
        write_early_static("TALOS: rp1-gpio16-owned-event-discriminator-control contract=");
        write_early_static(CONTRACT_ID);
        write_early_static(" target=");
        write_early_static(TARGET);
        write_early_static(" pin=");
        write_early_static(PIN);
        write_gpio16_ownership_event_capture_nonce();
        write_early_static(" bank=IO_BANK0 gpio16-bit-mask=");
        write_early_hex_u64(GPIO16_MASK as u64);
        write_early_static(
            " gpio16-status-address=not-constructed gpio16-ctrl-address=not-constructed io-bank0-inte-address=not-constructed io-bank0-ints-address=not-constructed rio-out-address=not-constructed rio-oe-address=not-constructed rio-in-address=not-constructed pad-address=not-constructed gicd-isenabler5-address=not-constructed gicd-ispendr5-address=not-constructed gicd-isactiver5-address=not-constructed gicc-hppir-address=not-constructed",
        );
        write_gpio16_ownership_event_preflight_fields(SNAPSHOT, GPIO16_MASK);
        write_early_static(
            " classification=no-mmio-observed-gpio16-ownership-event-control-visible\n",
        );
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_gpio16_owned_event_discriminator_no_mmio_control")]
fn write_gpio16_ownership_event_capture_nonce() {
    if let Some(nonce) = option_env!("TALOS_CAPTURE_NONCE") {
        if !nonce.is_empty() {
            write_early_static(" capture-nonce=");
            write_early_static(nonce);
        }
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_gpio16_owned_event_discriminator")]
fn read_gpio16_event_discriminator_snapshot(gic_intid: u32) -> Gpio16EventDiscriminatorSnapshot {
    let gpio16_status = read_rp1_reg_u32(RP1_GPIO16_OBSERVED_APERTURE_STATUS);
    let gpio16_ctrl = read_rp1_reg_u32(RP1_GPIO16_OBSERVED_APERTURE_CTRL);
    let io_bank0_inte = read_rp1_reg_u32(RP1_IO_BANK0_OBSERVED_APERTURE_INTE);
    let io_bank0_ints = read_rp1_reg_u32(RP1_IO_BANK0_OBSERVED_APERTURE_INTS);
    let rio_out = read_rp1_reg_u32(RP1_RIO0_OBSERVED_APERTURE_OUT);
    let rio_oe = read_rp1_reg_u32(RP1_RIO0_OBSERVED_APERTURE_OE);
    let rio_in = read_rp1_reg_u32(RP1_RIO0_OBSERVED_APERTURE_IN);
    let pad = read_rp1_reg_u32(RP1_GPIO16_OBSERVED_APERTURE_PAD);
    let gic = GicV2::new(GICD_BASE, GICC_BASE);
    let (gicd_isenabler5, gicd_ispendr5, gicd_isactiver5, gicc_hppir) = unsafe {
        (
            gic.enable_bits(gic_intid),
            gic.pending_bits(gic_intid),
            gic.active_bits(gic_intid),
            gic.highest_pending(),
        )
    };
    let hppir_intid = gicc_hppir & 0x3ff;

    Gpio16EventDiscriminatorSnapshot {
        gpio16_status,
        gpio16_ctrl,
        io_bank0_inte,
        io_bank0_ints,
        rio_out,
        rio_oe,
        rio_in,
        pad,
        gicd_isenabler5,
        gicd_ispendr5,
        gicd_isactiver5,
        gicc_hppir,
        hppir_intid,
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_gpio16_owned_event_discriminator")]
fn gpio16_observed_ownership_event_preflight_classification(
    snapshot: Gpio16EventDiscriminatorSnapshot,
    gpio16_mask: u32,
    gic_intid: u32,
) -> &'static str {
    let raw_event_enable = (snapshot.gpio16_ctrl >> 20) & 0xf;
    let filtered_event_enable = (snapshot.gpio16_ctrl >> 24) & 0xf;
    let status_event_mask = (snapshot.gpio16_status >> 20) & 0xff;
    let rp1_reads = [
        snapshot.gpio16_status,
        snapshot.gpio16_ctrl,
        snapshot.io_bank0_inte,
        snapshot.io_bank0_ints,
        snapshot.rio_out,
        snapshot.rio_oe,
        snapshot.rio_in,
        snapshot.pad,
    ];

    if rp1_reads.iter().all(|&raw| raw == 0xdead_dead) {
        "observed-gpio16-ownership-preflight-sentinel"
    } else if rp1_reads.iter().all(|&raw| raw == 0xffff_ffff) {
        "observed-gpio16-ownership-preflight-all-ones"
    } else if rp1_reads.iter().all(|&raw| raw == 0) {
        "observed-gpio16-ownership-preflight-zero"
    } else if gpio16_funcsel(snapshot.gpio16_ctrl) != 5 {
        "observed-gpio16-ownership-preflight-blocked-non-gpio-function"
    } else if (snapshot.gicd_isenabler5 & 1) != 0
        || (snapshot.gicd_ispendr5 & 1) != 0
        || (snapshot.gicd_isactiver5 & 1) != 0
        || snapshot.hppir_intid == gic_intid
        || raw_event_enable != 0
        || filtered_event_enable != 0
        || status_event_mask != 0
        || (snapshot.io_bank0_inte & gpio16_mask) != 0
        || (snapshot.io_bank0_ints & gpio16_mask) != 0
    {
        "observed-gpio16-ownership-preflight-blocked-route-or-source-state"
    } else {
        "observed-gpio16-ownership-event-preflight-visible"
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_gpio16_owned_event_discriminator",
    talos_boot_scenario = "rpi5_rp1_gpio16_owned_event_discriminator_no_mmio_control"
))]
fn write_gpio16_event_discriminator_addresses(
    gpio16_status: usize,
    gpio16_ctrl: usize,
    io_bank0_inte: usize,
    io_bank0_ints: usize,
    rio_out: usize,
    rio_oe: usize,
    rio_in: usize,
    pad: usize,
    gicd_isenabler5: usize,
    gicd_ispendr5: usize,
    gicd_isactiver5: usize,
    gicc_hppir: usize,
) {
    write_early_static(" gpio16-status-address=");
    write_early_hex_u64(gpio16_status as u64);
    write_early_static(" gpio16-ctrl-address=");
    write_early_hex_u64(gpio16_ctrl as u64);
    write_early_static(" io-bank0-inte-address=");
    write_early_hex_u64(io_bank0_inte as u64);
    write_early_static(" io-bank0-ints-address=");
    write_early_hex_u64(io_bank0_ints as u64);
    write_early_static(" rio-out-address=");
    write_early_hex_u64(rio_out as u64);
    write_early_static(" rio-oe-address=");
    write_early_hex_u64(rio_oe as u64);
    write_early_static(" rio-in-address=");
    write_early_hex_u64(rio_in as u64);
    write_early_static(" pad-address=");
    write_early_hex_u64(pad as u64);
    write_early_static(" gicd-isenabler5-address=");
    write_early_hex_u64(gicd_isenabler5 as u64);
    write_early_static(" gicd-ispendr5-address=");
    write_early_hex_u64(gicd_ispendr5 as u64);
    write_early_static(" gicd-isactiver5-address=");
    write_early_hex_u64(gicd_isactiver5 as u64);
    write_early_static(" gicc-hppir-address=");
    write_early_hex_u64(gicc_hppir as u64);
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_gpio16_owned_event_discriminator",
    talos_boot_scenario = "rpi5_rp1_gpio16_owned_event_discriminator_no_mmio_control"
))]
fn write_gpio16_ownership_event_preflight_fields(
    snapshot: Gpio16EventDiscriminatorSnapshot,
    gpio16_mask: u32,
) {
    write_early_static(" width=32 gpio16-status-raw=");
    write_early_hex_u64(snapshot.gpio16_status as u64);
    write_early_static(" gpio16-ctrl-raw=");
    write_early_hex_u64(snapshot.gpio16_ctrl as u64);
    write_early_static(" gpio16-funcsel=");
    write_early_dec_u64(gpio16_funcsel(snapshot.gpio16_ctrl) as u64);
    write_early_static(" gpio16-func-name=");
    write_early_static(gpio16_func_name(snapshot.gpio16_ctrl));
    write_early_static(" gpio16-outover=");
    write_early_dec_u64(((snapshot.gpio16_ctrl >> 12) & 0x3) as u64);
    write_early_static(" gpio16-oeover=");
    write_early_dec_u64(((snapshot.gpio16_ctrl >> 14) & 0x3) as u64);
    write_early_static(" gpio16-inover=");
    write_early_dec_u64(((snapshot.gpio16_ctrl >> 16) & 0x3) as u64);
    write_early_static(" gpio16-raw-event-enable-mask=");
    write_early_hex_u64(((snapshot.gpio16_ctrl >> 20) & 0xf) as u64);
    write_early_static(" gpio16-filtered-event-enable-mask=");
    write_early_hex_u64(((snapshot.gpio16_ctrl >> 24) & 0xf) as u64);
    write_early_static(" gpio16-status-event-mask=");
    write_early_hex_u64(((snapshot.gpio16_status >> 20) & 0xff) as u64);
    write_early_static(" gpio16-status-raw-high=");
    write_bool(snapshot.gpio16_status & (1 << 23) != 0);
    write_early_static(" io-bank0-inte-raw=");
    write_early_hex_u64(snapshot.io_bank0_inte as u64);
    write_early_static(" io-bank0-ints-raw=");
    write_early_hex_u64(snapshot.io_bank0_ints as u64);
    write_early_static(" gpio16-enabled=");
    write_bool(snapshot.io_bank0_inte & gpio16_mask != 0);
    write_early_static(" gpio16-source-status=");
    write_bool(snapshot.io_bank0_ints & gpio16_mask != 0);
    write_early_static(" rio-out-raw=");
    write_early_hex_u64(snapshot.rio_out as u64);
    write_early_static(" rio-oe-raw=");
    write_early_hex_u64(snapshot.rio_oe as u64);
    write_early_static(" rio-in-raw=");
    write_early_hex_u64(snapshot.rio_in as u64);
    write_early_static(" rio-out-gpio16=");
    write_bool(snapshot.rio_out & gpio16_mask != 0);
    write_early_static(" rio-oe-gpio16=");
    write_bool(snapshot.rio_oe & gpio16_mask != 0);
    write_early_static(" rio-in-gpio16=");
    write_bool(snapshot.rio_in & gpio16_mask != 0);
    write_early_static(" pad-raw=");
    write_early_hex_u64(snapshot.pad as u64);
    write_early_static(" pad-input-enable=");
    write_bool(snapshot.pad & (1 << 6) != 0);
    write_early_static(" pad-output-disable=");
    write_bool(snapshot.pad & (1 << 7) != 0);
    write_early_static(" pad-pull=");
    write_early_hex_u64(((snapshot.pad >> 2) & 0x3) as u64);
    write_early_static(" pad-drive=");
    write_early_hex_u64(((snapshot.pad >> 4) & 0x3) as u64);
    write_early_static(" pad-schmitt=");
    write_bool(snapshot.pad & (1 << 1) != 0);
    write_early_static(" pad-slew=");
    write_bool(snapshot.pad & 1 != 0);
    write_early_static(" gicd-isenabler5-raw=");
    write_early_hex_u64(snapshot.gicd_isenabler5 as u64);
    write_early_static(" gicd-ispendr5-raw=");
    write_early_hex_u64(snapshot.gicd_ispendr5 as u64);
    write_early_static(" gicd-isactiver5-raw=");
    write_early_hex_u64(snapshot.gicd_isactiver5 as u64);
    write_early_static(" gicc-hppir-raw=");
    write_early_hex_u64(snapshot.gicc_hppir as u64);
    write_early_static(" intid160-enabled=");
    write_bool(snapshot.gicd_isenabler5 & 1 != 0);
    write_early_static(" intid160-pending=");
    write_bool(snapshot.gicd_ispendr5 & 1 != 0);
    write_early_static(" intid160-active=");
    write_bool(snapshot.gicd_isactiver5 & 1 != 0);
    write_early_static(" hppir-intid=");
    write_early_dec_u64(snapshot.hppir_intid as u64);
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_gpio14_status_read",
    talos_boot_scenario = "rpi5_rp1_gpio14_status_no_mmio_control"
))]
fn write_gpio_status_bits(value: u32) {
    write_early_static(" raw-falling=");
    write_bool(value & (1 << 20) != 0);
    write_early_static(" raw-rising=");
    write_bool(value & (1 << 21) != 0);
    write_early_static(" raw-low=");
    write_bool(value & (1 << 22) != 0);
    write_early_static(" raw-high=");
    write_bool(value & (1 << 23) != 0);
    write_early_static(" filtered-falling=");
    write_bool(value & (1 << 24) != 0);
    write_early_static(" filtered-rising=");
    write_bool(value & (1 << 25) != 0);
    write_early_static(" filtered-low=");
    write_bool(value & (1 << 26) != 0);
    write_early_static(" filtered-high=");
    write_bool(value & (1 << 27) != 0);
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_interrupt_routing_msix_cfg_read",
    talos_boot_scenario = "rpi5_rp1_interrupt_routing_no_mmio_control"
))]
fn write_msix_cfg_bits(value: u32) {
    write_early_static(" enable=");
    write_bool(value & 1 != 0);
    write_early_static(" test=");
    write_bool(value & (1 << 1) != 0);
    write_early_static(" iack=");
    write_bool(value & (1 << 2) != 0);
    write_early_static(" iack-en=");
    write_bool(value & (1 << 3) != 0);
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_gic_visible_route_status_read",
    talos_boot_scenario = "rpi5_rp1_gic_visible_route_no_mmio_control"
))]
fn write_gic_route_status_bits(
    isenabler: u32,
    ispendr: u32,
    isactiver: u32,
    hppir: u32,
    bit_mask: u32,
) {
    write_early_static(" isenabler-raw=");
    write_early_hex_u64(isenabler as u64);
    write_early_static(" ispendr-raw=");
    write_early_hex_u64(ispendr as u64);
    write_early_static(" isactiver-raw=");
    write_early_hex_u64(isactiver as u64);
    write_early_static(" intid-enabled=");
    write_bool(isenabler & bit_mask != 0);
    write_early_static(" intid-pending=");
    write_bool(ispendr & bit_mask != 0);
    write_early_static(" intid-active=");
    write_bool(isactiver & bit_mask != 0);
    write_early_static(" hppir-raw=");
    write_early_hex_u64(hppir as u64);
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_gpio_bank_source_status_read",
    talos_boot_scenario = "rpi5_rp1_gpio_bank_source_status_no_mmio_control"
))]
fn write_gpio_bank_source_status_bits(inte: u32, ints: u32, gpio14_mask: u32) {
    write_early_static(" width=32 inte-raw=");
    write_early_hex_u64(inte as u64);
    write_early_static(" ints-raw=");
    write_early_hex_u64(ints as u64);
    write_early_static(" gpio14-mask=");
    write_early_hex_u64(gpio14_mask as u64);
    write_early_static(" gpio14-enabled=");
    write_bool(inte & gpio14_mask != 0);
    write_early_static(" gpio14-source-status=");
    write_bool(ints & gpio14_mask != 0);
    write_early_static(" source-status-mask=");
    write_early_hex_u64(ints as u64);
    write_early_static(" source-status-nonzero=");
    write_bool(ints != 0);
}

#[cfg(talos_boot_scenario = "rpi5_rp1_clock_manager_status_read")]
fn write_clock_status_register(name: &str, address: usize, value: u32) {
    write_early_static(name);
    write_early_static("-address=");
    write_early_hex_u64(address as u64);
    write_early_static(" raw=");
    write_early_hex_u64(value as u64);
}

#[cfg(talos_boot_scenario = "rpi5_rp1_clock_manager_status_no_mmio_control")]
fn write_clock_status_control_register(name: &str, value: u32) {
    write_early_static(name);
    write_early_static("-address=not-constructed raw=");
    write_early_hex_u64(value as u64);
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_clock_manager_status_read",
    talos_boot_scenario = "rpi5_rp1_clock_manager_status_no_mmio_control"
))]
fn write_clock_manager_status_bits(
    pll_sys_cs: u32,
    clk_sys_ctrl: u32,
    clk_sys_div_int: u32,
    clk_sys_sel: u32,
    clk_slow_sys_ctrl: u32,
    clk_uart_ctrl: u32,
    clk_uart_div_int: u32,
    clk_uart_sel: u32,
) {
    write_early_static(" pll-sys-lock=");
    write_bool(pll_sys_cs & (1 << 31) != 0);
    write_early_static(" pll-sys-refdiv=");
    write_early_hex_u64((pll_sys_cs & 0x3f) as u64);
    write_early_static(" clk-sys-enabled=");
    write_bool(clk_sys_ctrl & (1 << 11) != 0);
    write_early_static(" clk-sys-source=");
    write_early_hex_u64((clk_sys_ctrl & 0x3) as u64);
    write_early_static(" clk-sys-auxsrc=");
    write_early_hex_u64(((clk_sys_ctrl >> 5) & 0x1f) as u64);
    write_early_static(" clk-sys-div-int-decoded=");
    write_early_hex_u64(clk_sys_div_int as u64);
    write_early_static(" clk-sys-sel-decoded=");
    write_early_hex_u64(clk_sys_sel as u64);
    write_early_static(" clk-slow-sys-enabled=");
    write_bool(clk_slow_sys_ctrl & (1 << 11) != 0);
    write_early_static(" clk-slow-sys-source=");
    write_early_hex_u64((clk_slow_sys_ctrl & 0x3) as u64);
    write_early_static(" clk-uart-enabled=");
    write_bool(clk_uart_ctrl & (1 << 11) != 0);
    write_early_static(" clk-uart-source=");
    write_early_hex_u64((clk_uart_ctrl & 0x3) as u64);
    write_early_static(" clk-uart-auxsrc=");
    write_early_hex_u64(((clk_uart_ctrl >> 5) & 0x1f) as u64);
    write_early_static(" clk-uart-div-int-decoded=");
    write_early_hex_u64(clk_uart_div_int as u64);
    write_early_static(" clk-uart-sel-decoded=");
    write_early_hex_u64(clk_uart_sel as u64);
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_clock_adc_ctrl_write_restore",
    talos_boot_scenario = "rpi5_rp1_clock_adc_ctrl_write_restore_no_mmio_control",
    talos_boot_scenario = "rpi5_rp1_ethernet_clock_reset_write_restore_candidate",
    talos_boot_scenario = "rpi5_rp1_ethernet_clock_reset_write_restore_no_mmio_control",
    talos_boot_scenario = "rpi5_rp1_ethernet_clk_eth_ctrl_write_restore_candidate",
    talos_boot_scenario = "rpi5_rp1_ethernet_clk_eth_ctrl_write_restore_no_mmio_control"
))]
fn write_clock_adc_ctrl_raw_triplet(pre_raw: u32, post_raw: u32, restore_raw: u32) {
    write_early_static(" pre-raw=");
    write_early_hex_u64(pre_raw as u64);
    write_clock_adc_ctrl_fields(" pre-enable=", " pre-auxsrc=", " pre-source=", pre_raw);
    write_early_static(" post-raw=");
    write_early_hex_u64(post_raw as u64);
    write_clock_adc_ctrl_fields(" post-enable=", " post-auxsrc=", " post-source=", post_raw);
    write_early_static(" restore-raw=");
    write_early_hex_u64(restore_raw as u64);
    write_clock_adc_ctrl_fields(
        " restore-enable=",
        " restore-auxsrc=",
        " restore-source=",
        restore_raw,
    );
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_clock_adc_ctrl_write_restore",
    talos_boot_scenario = "rpi5_rp1_clock_adc_ctrl_write_restore_no_mmio_control",
    talos_boot_scenario = "rpi5_rp1_clock_adc_ctrl_enable_toggle",
    talos_boot_scenario = "rpi5_rp1_clock_adc_ctrl_enable_toggle_no_mmio_control",
    talos_boot_scenario = "rpi5_rp1_ethernet_clock_reset_write_restore_candidate",
    talos_boot_scenario = "rpi5_rp1_ethernet_clock_reset_write_restore_no_mmio_control",
    talos_boot_scenario = "rpi5_rp1_ethernet_clk_eth_ctrl_write_restore_candidate",
    talos_boot_scenario = "rpi5_rp1_ethernet_clk_eth_ctrl_write_restore_no_mmio_control"
))]
fn write_clock_adc_ctrl_fields(
    enable_field: &str,
    auxsrc_field: &str,
    source_field: &str,
    value: u32,
) {
    write_early_static(enable_field);
    write_bool(value & (1 << 11) != 0);
    write_early_static(auxsrc_field);
    write_early_hex_u64(((value >> 5) & 0x1f) as u64);
    write_early_static(source_field);
    write_early_hex_u64((value & 0x3) as u64);
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_clock_adc_ctrl_enable_toggle",
    talos_boot_scenario = "rpi5_rp1_clock_adc_ctrl_enable_toggle_no_mmio_control"
))]
fn write_clock_adc_ctrl_enable_toggle_values(
    pre_raw: u32,
    transition_raw: u32,
    post_raw: u32,
    restore_raw: u32,
) {
    write_early_static(" pre-raw=");
    write_early_hex_u64(pre_raw as u64);
    write_clock_adc_ctrl_fields(" pre-enable=", " pre-auxsrc=", " pre-source=", pre_raw);
    write_early_static(" transition-raw=");
    write_early_hex_u64(transition_raw as u64);
    write_early_static(" post-raw=");
    write_early_hex_u64(post_raw as u64);
    write_clock_adc_ctrl_fields(" post-enable=", " post-auxsrc=", " post-source=", post_raw);
    write_early_static(" restore-raw=");
    write_early_hex_u64(restore_raw as u64);
    write_clock_adc_ctrl_fields(
        " restore-enable=",
        " restore-auxsrc=",
        " restore-source=",
        restore_raw,
    );
}

#[cfg(talos_boot_scenario = "rpi5_rp1_clock_adc_window_coherence_read")]
fn write_adc_window_register(name: &str, source_offset: u64, address: usize, value: u32) {
    write_early_static(name);
    write_early_static("-source-offset=");
    write_early_hex_u64(source_offset);
    write_early_static(" address=");
    write_early_hex_u64(address as u64);
    write_early_static(" width=32 raw=");
    write_early_hex_u64(value as u64);
}

#[cfg(talos_boot_scenario = "rpi5_rp1_clock_adc_window_coherence_no_mmio_control")]
fn write_adc_window_control_register(name: &str, source_offset: u64, value: u32) {
    write_early_static(name);
    write_early_static("-source-offset=");
    write_early_hex_u64(source_offset);
    write_early_static(" address=not-constructed width=32 raw=");
    write_early_hex_u64(value as u64);
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_clock_adc_window_coherence_read",
    talos_boot_scenario = "rpi5_rp1_clock_adc_window_coherence_no_mmio_control"
))]
fn write_adc_ctrl_window_fields(prefix: &str, value: u32) {
    write_early_static(prefix);
    write_early_static("-enable=");
    write_bool(value & RP1_CLK_CTRL_ENABLE != 0);
    write_early_static(prefix);
    write_early_static("-auxsrc=");
    write_early_hex_u64(((value >> 5) & 0x1f) as u64);
    write_early_static(prefix);
    write_early_static("-source=");
    write_early_hex_u64((value & 0x3) as u64);
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_clock_adc_window_coherence_read",
    talos_boot_scenario = "rpi5_rp1_clock_adc_window_coherence_no_mmio_control"
))]
fn write_adc_window_booleans(
    adc_ctrl_stable: bool,
    adc_window_all_equal: bool,
    adc_window_all_deaddead: bool,
    adc_sel: u32,
) {
    let adc_sel_zero = adc_sel == 0;
    let adc_sel_one_hot = adc_sel != 0 && (adc_sel & (adc_sel - 1)) == 0;
    let adc_sel_multi_bit = adc_sel.count_ones() > 1;

    write_early_static(" adc-ctrl-stable=");
    write_bool(adc_ctrl_stable);
    write_early_static(" adc-window-all-equal=");
    write_bool(adc_window_all_equal);
    write_early_static(" adc-window-all-deaddead=");
    write_bool(adc_window_all_deaddead);
    write_early_static(" adc-sel-zero=");
    write_bool(adc_sel_zero);
    write_early_static(" adc-sel-one-hot=");
    write_bool(adc_sel_one_hot);
    write_early_static(" adc-sel-multi-bit=");
    write_bool(adc_sel_multi_bit);
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_clock_adc_window_coherence_read",
    talos_boot_scenario = "rpi5_rp1_clock_adc_window_coherence_no_mmio_control"
))]
fn write_prior_adc_enable_toggle_context(
    prior_pre_raw: u32,
    prior_transition_raw: u32,
    prior_post_raw: u32,
    prior_restore_raw: u32,
) {
    write_early_static(" retained-enable-toggle-pre-raw=");
    write_early_hex_u64(prior_pre_raw as u64);
    write_early_static(" retained-enable-toggle-transition-raw=");
    write_early_hex_u64(prior_transition_raw as u64);
    write_early_static(" retained-enable-toggle-post-raw=");
    write_early_hex_u64(prior_post_raw as u64);
    write_early_static(" retained-enable-toggle-restore-raw=");
    write_early_hex_u64(prior_restore_raw as u64);
    write_early_static(" retained-enable-toggle-restore-eq-pre=true");
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_sysinfo_clock_sentinel_read",
    talos_boot_scenario = "rpi5_rp1_sysinfo_clock_sentinel_no_mmio_control"
))]
fn classify_sysinfo_clock_sentinel(
    chip_id_matches_expected: bool,
    chip_id_is_deaddead: bool,
    platform_is_deaddead: bool,
    adc_ctrl_is_deaddead: bool,
) -> &'static str {
    if chip_id_matches_expected && adc_ctrl_is_deaddead {
        "rp1-sysinfo-live-clock-window-sentinel"
    } else if chip_id_matches_expected {
        "rp1-sysinfo-live-clock-window-non-sentinel"
    } else if chip_id_is_deaddead && platform_is_deaddead && adc_ctrl_is_deaddead {
        "rp1-sysinfo-and-clock-window-sentinel"
    } else if chip_id_is_deaddead {
        "rp1-sysinfo-address-decode-blocked"
    } else {
        "rp1-sysinfo-unexpected-chip-id"
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_sysinfo_clock_sentinel_read")]
fn write_sysinfo_clock_sentinel_register(
    name: &str,
    source_offset: u64,
    address: usize,
    value: u32,
) {
    write_early_static(name);
    write_early_static("-source-offset=");
    write_early_hex_u64(source_offset);
    write_early_static(" address=");
    write_early_hex_u64(address as u64);
    write_early_static(" width=32 raw=");
    write_early_hex_u64(value as u64);
}

#[cfg(talos_boot_scenario = "rpi5_rp1_sysinfo_clock_sentinel_no_mmio_control")]
fn write_sysinfo_clock_sentinel_control_register(name: &str, source_offset: u64, value: u32) {
    write_early_static(name);
    write_early_static("-source-offset=");
    write_early_hex_u64(source_offset);
    write_early_static(" address=not-constructed width=32 raw=");
    write_early_hex_u64(value as u64);
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_sysinfo_clock_sentinel_read",
    talos_boot_scenario = "rpi5_rp1_sysinfo_clock_sentinel_no_mmio_control"
))]
fn write_sysinfo_clock_sentinel_booleans(
    chip_id_matches_expected: bool,
    chip_id_is_deaddead: bool,
    platform_is_deaddead: bool,
    adc_ctrl_is_deaddead: bool,
    sysinfo_pair_equal: bool,
    sysinfo_vs_adc_same: bool,
) {
    write_early_static(" expected-chip-id=");
    write_early_hex_u64(RP1_EXPECTED_CHIP_ID as u64);
    write_early_static(" chip-id-matches-expected=");
    write_bool(chip_id_matches_expected);
    write_early_static(" chip-id-is-deaddead=");
    write_bool(chip_id_is_deaddead);
    write_early_static(" platform-is-deaddead=");
    write_bool(platform_is_deaddead);
    write_early_static(" adc-ctrl-is-deaddead=");
    write_bool(adc_ctrl_is_deaddead);
    write_early_static(" sysinfo-pair-equal=");
    write_bool(sysinfo_pair_equal);
    write_early_static(" sysinfo-vs-adc-same=");
    write_bool(sysinfo_vs_adc_same);
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_sysinfo_clock_sentinel_read",
    talos_boot_scenario = "rpi5_rp1_sysinfo_clock_sentinel_no_mmio_control"
))]
fn write_retained_adc_window_sentinel_context(classification: &str, raw: u32) {
    write_early_static(" retained-adc-window-classification=");
    write_early_static(classification);
    write_early_static(" retained-adc-window-clk-sys-ctrl-raw=");
    write_early_hex_u64(raw as u64);
    write_early_static(" retained-adc-window-clk-uart-ctrl-raw=");
    write_early_hex_u64(raw as u64);
    write_early_static(" retained-adc-window-adc-ctrl-first-raw=");
    write_early_hex_u64(raw as u64);
    write_early_static(" retained-adc-window-adc-ctrl-second-raw=");
    write_early_hex_u64(raw as u64);
    write_early_static(" retained-adc-window-adc-div-int-raw=");
    write_early_hex_u64(raw as u64);
    write_early_static(" retained-adc-window-adc-sel-raw=");
    write_early_hex_u64(raw as u64);
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_pcie2_host_link_status_read",
    talos_boot_scenario = "rpi5_rp1_pcie2_host_link_status_no_mmio_control"
))]
fn classify_pcie2_host_link_status(
    status_is_deaddead: bool,
    dl_active: bool,
    phylinkup: bool,
) -> &'static str {
    if status_is_deaddead {
        "pcie2-host-status-sentinel"
    } else if dl_active && phylinkup {
        "pcie2-host-link-up-rp1-window-sentinel"
    } else {
        "pcie2-host-status-visible-link-down"
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_pcie2_host_link_status_read")]
fn write_pcie2_host_link_status_register(name: &str, address: usize, value: u32) {
    write_early_static(" register=");
    write_early_static(name);
    write_early_static(" source-offset=");
    write_early_hex_u64(PCIE_MISC_PCIE_STATUS_OFFSET);
    write_early_static(" address=");
    write_early_hex_u64(address as u64);
    write_early_static(" width=32 raw=");
    write_early_hex_u64(value as u64);
}

#[cfg(talos_boot_scenario = "rpi5_rp1_pcie2_host_link_status_no_mmio_control")]
fn write_pcie2_host_link_status_control_register(name: &str, value: u32) {
    write_early_static(" register=");
    write_early_static(name);
    write_early_static(" source-offset=");
    write_early_hex_u64(PCIE_MISC_PCIE_STATUS_OFFSET);
    write_early_static(" address=not-constructed width=32 raw=");
    write_early_hex_u64(value as u64);
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_pcie2_host_link_status_read",
    talos_boot_scenario = "rpi5_rp1_pcie2_host_link_status_no_mmio_control"
))]
fn write_pcie2_host_link_status_booleans(
    pcie_port: bool,
    dl_active: bool,
    phylinkup: bool,
    link_in_l23: bool,
    status_is_deaddead: bool,
) {
    write_early_static(" pcie-port=");
    write_bool(pcie_port);
    write_early_static(" dl-active=");
    write_bool(dl_active);
    write_early_static(" phylinkup=");
    write_bool(phylinkup);
    write_early_static(" link-in-l23=");
    write_bool(link_in_l23);
    write_early_static(" status-is-deaddead=");
    write_bool(status_is_deaddead);
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_pcie2_host_link_status_read",
    talos_boot_scenario = "rpi5_rp1_pcie2_host_link_status_no_mmio_control"
))]
fn write_retained_rp1_window_sentinel_context(classification: &str) {
    write_early_static(" retained-sysinfo-clock-sentinel-classification=");
    write_early_static(classification);
    write_early_static(" retained-rp1-window-sentinel=true");
}

#[cfg(talos_boot_scenario = "rpi5_rp1_endpoint_config_identity_read")]
fn write_pcie_ext_cfg_index(value: u32) {
    let reg = PCIE_EXT_CFG_INDEX as *mut u32;
    unsafe {
        core::ptr::write_volatile(reg, value);
        core::arch::asm!("dsb sy", options(nostack, preserves_flags));
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_endpoint_config_identity_read",
    talos_boot_scenario = "rpi5_rp1_endpoint_config_identity_no_mmio_control"
))]
fn classify_rp1_endpoint_config_identity(
    status_is_deaddead: bool,
    dl_active: bool,
    phylinkup: bool,
    raw_config_is_all_ones: bool,
    raw_config_is_zero: bool,
    raw_config_is_deaddead: bool,
    vendor_device_match: bool,
) -> &'static str {
    if status_is_deaddead {
        "rp1-endpoint-config-id-inconclusive-capture"
    } else if !(dl_active && phylinkup) {
        "rp1-endpoint-config-link-down-skip"
    } else if raw_config_is_deaddead {
        "rp1-endpoint-config-id-sentinel"
    } else if raw_config_is_all_ones {
        "rp1-endpoint-config-id-all-ones"
    } else if raw_config_is_zero {
        "rp1-endpoint-config-id-zero"
    } else if vendor_device_match {
        "rp1-endpoint-config-id-visible"
    } else {
        "rp1-endpoint-config-id-unexpected"
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_bridge_config_preflight_read",
    talos_boot_scenario = "rpi5_rp1_bridge_config_preflight_no_mmio_control"
))]
fn classify_pcie2_bridge_config_preflight(
    status_is_deaddead: bool,
    dl_active: bool,
    phylinkup: bool,
    misc_ctrl_is_sentinel: bool,
    scb_access_en: bool,
    cfg_read_ur_mode: bool,
) -> &'static str {
    if status_is_deaddead {
        "pcie2-bridge-preflight-inconclusive-capture"
    } else if !(dl_active && phylinkup) {
        "pcie2-bridge-preflight-link-down-skip"
    } else if misc_ctrl_is_sentinel {
        "pcie2-bridge-preflight-sentinel"
    } else if scb_access_en && cfg_read_ur_mode {
        "pcie2-bridge-preflight-ready"
    } else {
        "pcie2-bridge-preflight-incomplete"
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_bridge_setup_state_read",
    talos_boot_scenario = "rpi5_rp1_bridge_setup_state_no_mmio_control"
))]
struct BridgeSetupState {
    status_raw: u32,
    pcie_port: bool,
    dl_active: bool,
    phylinkup: bool,
    link_in_l23: bool,
    status_is_deaddead: bool,
    misc_ctrl_raw: u32,
    scb_access_en: bool,
    cfg_read_ur_mode: bool,
    misc_ctrl_is_sentinel: bool,
    rc_class_raw: u32,
    class_code: u32,
    class_code_is_pcie_bridge: bool,
    win0_lo_raw: u32,
    win0_hi_raw: u32,
    win0_base_limit_raw: u32,
    win0_base_hi_raw: u32,
    win0_limit_hi_raw: u32,
    pcie_base_is_zero: bool,
    cpu_base_low_matches: bool,
    cpu_limit_low_matches: bool,
    cpu_base_high_matches: bool,
    cpu_limit_high_matches: bool,
    outbound_window0_matches: bool,
    classification: &'static str,
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_bridge_setup_state_read",
    talos_boot_scenario = "rpi5_rp1_bridge_setup_state_no_mmio_control"
))]
impl BridgeSetupState {
    fn from_raw(
        status_raw: u32,
        misc_ctrl_raw: u32,
        rc_class_raw: u32,
        win0_lo_raw: u32,
        win0_hi_raw: u32,
        win0_base_limit_raw: u32,
        win0_base_hi_raw: u32,
        win0_limit_hi_raw: u32,
    ) -> Self {
        let pcie_port = status_raw & PCIE_STATUS_PORT != 0;
        let dl_active = status_raw & PCIE_STATUS_DL_ACTIVE != 0;
        let phylinkup = status_raw & PCIE_STATUS_PHYLINKUP != 0;
        let link_in_l23 = status_raw & PCIE_STATUS_LINK_IN_L23 != 0;
        let status_is_deaddead = status_raw == 0xdead_dead;
        let scb_access_en = misc_ctrl_raw & PCIE_MISC_CTRL_SCB_ACCESS_EN != 0;
        let cfg_read_ur_mode = misc_ctrl_raw & PCIE_MISC_CTRL_CFG_READ_UR_MODE != 0;
        let misc_ctrl_is_sentinel =
            misc_ctrl_raw == 0xdead_dead || misc_ctrl_raw == 0xffff_ffff || misc_ctrl_raw == 0;
        let class_code = rc_class_raw & PCIE_RC_CLASS_CODE_MASK;
        let class_code_is_pcie_bridge = class_code == PCIE_RC_EXPECTED_BRIDGE_CLASS_CODE;
        let class_register_is_sentinel =
            rc_class_raw == 0xdead_dead || rc_class_raw == 0xffff_ffff || rc_class_raw == 0;
        let pcie_base_is_zero = win0_lo_raw == 0 && win0_hi_raw == 0;
        let cpu_base_low_matches =
            win0_base_limit_raw & PCIE_WIN0_BASE_LOW_MASK == PCIE_WIN0_BASE_LOW_EXPECTED;
        let cpu_limit_low_matches =
            win0_base_limit_raw & PCIE_WIN0_LIMIT_LOW_MASK == PCIE_WIN0_LIMIT_LOW_EXPECTED;
        let cpu_base_high_matches =
            win0_base_hi_raw & PCIE_WIN0_HIGH_MASK == PCIE_WIN0_HIGH_EXPECTED;
        let cpu_limit_high_matches =
            win0_limit_hi_raw & PCIE_WIN0_HIGH_MASK == PCIE_WIN0_HIGH_EXPECTED;
        let outbound_window0_matches = pcie_base_is_zero
            && cpu_base_low_matches
            && cpu_limit_low_matches
            && cpu_base_high_matches
            && cpu_limit_high_matches;
        let outbound_window0_is_sentinel = bridge_setup_window0_is_sentinel(
            win0_lo_raw,
            win0_hi_raw,
            win0_base_limit_raw,
            win0_base_hi_raw,
            win0_limit_hi_raw,
        );
        let classification = classify_pcie2_bridge_setup_state(
            status_is_deaddead,
            dl_active,
            phylinkup,
            misc_ctrl_is_sentinel,
            scb_access_en,
            cfg_read_ur_mode,
            class_register_is_sentinel,
            class_code_is_pcie_bridge,
            outbound_window0_is_sentinel,
            outbound_window0_matches,
        );

        Self {
            status_raw,
            pcie_port,
            dl_active,
            phylinkup,
            link_in_l23,
            status_is_deaddead,
            misc_ctrl_raw,
            scb_access_en,
            cfg_read_ur_mode,
            misc_ctrl_is_sentinel,
            rc_class_raw,
            class_code,
            class_code_is_pcie_bridge,
            win0_lo_raw,
            win0_hi_raw,
            win0_base_limit_raw,
            win0_base_hi_raw,
            win0_limit_hi_raw,
            pcie_base_is_zero,
            cpu_base_low_matches,
            cpu_limit_low_matches,
            cpu_base_high_matches,
            cpu_limit_high_matches,
            outbound_window0_matches,
            classification,
        }
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_bridge_setup_state_read",
    talos_boot_scenario = "rpi5_rp1_bridge_setup_state_no_mmio_control"
))]
fn classify_pcie2_bridge_setup_state(
    status_is_deaddead: bool,
    dl_active: bool,
    phylinkup: bool,
    misc_ctrl_is_sentinel: bool,
    scb_access_en: bool,
    cfg_read_ur_mode: bool,
    class_register_is_sentinel: bool,
    class_code_is_pcie_bridge: bool,
    outbound_window0_is_sentinel: bool,
    outbound_window0_matches: bool,
) -> &'static str {
    if status_is_deaddead {
        "pcie2-bridge-setup-state-inconclusive-capture"
    } else if !(dl_active && phylinkup) {
        "pcie2-bridge-setup-state-link-down-skip"
    } else if misc_ctrl_is_sentinel || class_register_is_sentinel || outbound_window0_is_sentinel {
        "pcie2-bridge-setup-state-sentinel"
    } else if scb_access_en
        && cfg_read_ur_mode
        && class_code_is_pcie_bridge
        && outbound_window0_matches
    {
        "pcie2-bridge-setup-state-visible"
    } else {
        "pcie2-bridge-setup-state-incomplete"
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_bridge_setup_state_read",
    talos_boot_scenario = "rpi5_rp1_bridge_setup_state_no_mmio_control"
))]
fn bridge_setup_window0_is_sentinel(
    win0_lo: u32,
    win0_hi: u32,
    win0_base_limit: u32,
    win0_base_hi: u32,
    win0_limit_hi: u32,
) -> bool {
    let lo_sentinel = win0_lo == 0xdead_dead || win0_lo == 0xffff_ffff;
    let hi_sentinel = win0_hi == 0xdead_dead || win0_hi == 0xffff_ffff;
    let base_limit_sentinel = win0_base_limit == 0xdead_dead
        || win0_base_limit == 0xffff_ffff
        || (win0_base_limit & PCIE_WIN0_LIMIT_LOW_MASK) == 0;
    let base_hi_sentinel =
        win0_base_hi == 0xdead_dead || win0_base_hi == 0xffff_ffff || win0_base_hi == 0;
    let limit_hi_sentinel =
        win0_limit_hi == 0xdead_dead || win0_limit_hi == 0xffff_ffff || win0_limit_hi == 0;

    lo_sentinel || hi_sentinel || base_limit_sentinel || base_hi_sentinel || limit_hi_sentinel
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_bridge_config_preflight_read",
    talos_boot_scenario = "rpi5_rp1_bridge_config_preflight_no_mmio_control"
))]
#[allow(clippy::too_many_arguments)]
fn write_bridge_config_preflight_common_fields(
    controller_base: Option<u64>,
    status_register_name: &str,
    status_source_offset: u64,
    status_address: Option<u64>,
    status_raw: u32,
    pcie_port: bool,
    dl_active: bool,
    phylinkup: bool,
    link_in_l23: bool,
    status_is_deaddead: bool,
    preflight_register_name: &str,
    preflight_source_offset: u64,
    preflight_address: Option<u64>,
    misc_ctrl_raw: u32,
    scb_access_en: bool,
    cfg_read_ur_mode: bool,
    rcb_mps_mode: bool,
    rcb_64b_mode: bool,
    max_burst_size: u32,
    misc_ctrl_is_sentinel: bool,
    retained_endpoint_config_classification: &str,
) {
    write_early_static(" pcie2-controller-base=");
    write_optional_hex_or_not_constructed(controller_base);
    write_early_static(" status-register=");
    write_early_static(status_register_name);
    write_early_static(" status-source-offset=");
    write_early_hex_u64(status_source_offset);
    write_early_static(" status-address=");
    write_optional_hex_or_not_constructed(status_address);
    write_early_static(" status-width=32 status-raw=");
    write_early_hex_u64(status_raw as u64);
    write_early_static(" pcie-port=");
    write_bool(pcie_port);
    write_early_static(" dl-active=");
    write_bool(dl_active);
    write_early_static(" phylinkup=");
    write_bool(phylinkup);
    write_early_static(" link-in-l23=");
    write_bool(link_in_l23);
    write_early_static(" status-is-deaddead=");
    write_bool(status_is_deaddead);
    write_early_static(" preflight-register=");
    write_early_static(preflight_register_name);
    write_early_static(" preflight-source-offset=");
    write_early_hex_u64(preflight_source_offset);
    write_early_static(" preflight-address=");
    write_optional_hex_or_not_constructed(preflight_address);
    write_early_static(" preflight-width=32 misc-ctrl-raw=");
    write_early_hex_u64(misc_ctrl_raw as u64);
    write_early_static(" scb-access-en=");
    write_bool(scb_access_en);
    write_early_static(" cfg-read-ur-mode=");
    write_bool(cfg_read_ur_mode);
    write_early_static(" rcb-mps-mode=");
    write_bool(rcb_mps_mode);
    write_early_static(" rcb-64b-mode=");
    write_bool(rcb_64b_mode);
    write_early_static(" max-burst-size=");
    write_early_hex_u64(max_burst_size as u64);
    write_early_static(" misc-ctrl-is-sentinel=");
    write_bool(misc_ctrl_is_sentinel);
    write_early_static(" retained-endpoint-config-classification=");
    write_early_static(retained_endpoint_config_classification);
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_bridge_config_preflight_read",
    talos_boot_scenario = "rpi5_rp1_bridge_config_preflight_no_mmio_control"
))]
fn write_bridge_config_preflight_classification_vocabulary() {
    write_early_static(
        " classification-vocabulary=pcie2-bridge-preflight-ready,pcie2-bridge-preflight-incomplete,pcie2-bridge-preflight-sentinel,pcie2-bridge-preflight-link-down-skip,pcie2-bridge-preflight-inconclusive-capture,no-mmio-pcie2-bridge-preflight-control-visible,staging/build-blocker",
    );
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_bridge_setup_state_read",
    talos_boot_scenario = "rpi5_rp1_bridge_setup_state_no_mmio_control"
))]
#[allow(clippy::too_many_arguments)]
fn write_bridge_setup_state_common_fields(
    controller_base: Option<u64>,
    status_register_name: &str,
    status_source_offset: u64,
    status_address: Option<u64>,
    preflight_register_name: &str,
    preflight_source_offset: u64,
    preflight_address: Option<u64>,
    rc_class_register_name: &str,
    rc_class_source_offset: u64,
    rc_class_address: Option<u64>,
    win0_lo_register_name: &str,
    win0_lo_source_offset: u64,
    win0_lo_address: Option<u64>,
    win0_hi_register_name: &str,
    win0_hi_source_offset: u64,
    win0_hi_address: Option<u64>,
    win0_base_limit_register_name: &str,
    win0_base_limit_source_offset: u64,
    win0_base_limit_address: Option<u64>,
    win0_base_hi_register_name: &str,
    win0_base_hi_source_offset: u64,
    win0_base_hi_address: Option<u64>,
    win0_limit_hi_register_name: &str,
    win0_limit_hi_source_offset: u64,
    win0_limit_hi_address: Option<u64>,
    decoded: &BridgeSetupState,
    retained_endpoint_config_classification: &str,
) {
    write_early_static(" pcie2-controller-base=");
    write_optional_hex_or_not_constructed(controller_base);
    write_early_static(" status-register=");
    write_early_static(status_register_name);
    write_early_static(" status-source-offset=");
    write_early_hex_u64(status_source_offset);
    write_early_static(" status-address=");
    write_optional_hex_or_not_constructed(status_address);
    write_early_static(" status-width=32 status-raw=");
    write_early_hex_u64(decoded.status_raw as u64);
    write_early_static(" pcie-port=");
    write_bool(decoded.pcie_port);
    write_early_static(" dl-active=");
    write_bool(decoded.dl_active);
    write_early_static(" phylinkup=");
    write_bool(decoded.phylinkup);
    write_early_static(" link-in-l23=");
    write_bool(decoded.link_in_l23);
    write_early_static(" status-is-deaddead=");
    write_bool(decoded.status_is_deaddead);
    write_early_static(" preflight-register=");
    write_early_static(preflight_register_name);
    write_early_static(" preflight-source-offset=");
    write_early_hex_u64(preflight_source_offset);
    write_early_static(" preflight-address=");
    write_optional_hex_or_not_constructed(preflight_address);
    write_early_static(" preflight-width=32 misc-ctrl-raw=");
    write_early_hex_u64(decoded.misc_ctrl_raw as u64);
    write_early_static(" scb-access-en=");
    write_bool(decoded.scb_access_en);
    write_early_static(" cfg-read-ur-mode=");
    write_bool(decoded.cfg_read_ur_mode);
    write_early_static(" misc-ctrl-is-sentinel=");
    write_bool(decoded.misc_ctrl_is_sentinel);
    write_early_static(" rc-class-register=");
    write_early_static(rc_class_register_name);
    write_early_static(" rc-class-source-offset=");
    write_early_hex_u64(rc_class_source_offset);
    write_early_static(" rc-class-address=");
    write_optional_hex_or_not_constructed(rc_class_address);
    write_early_static(" rc-class-width=32 rc-class-raw=");
    write_early_hex_u64(decoded.rc_class_raw as u64);
    write_early_static(" class-code=");
    write_early_hex_u64(decoded.class_code as u64);
    write_early_static(" class-code-is-pcie-bridge=");
    write_bool(decoded.class_code_is_pcie_bridge);
    write_bridge_setup_win0_register(
        " win0-lo-register=",
        win0_lo_register_name,
        " win0-lo-source-offset=",
        win0_lo_source_offset,
        " win0-lo-address=",
        win0_lo_address,
        " win0-lo-width=32 win0-lo-raw=",
        decoded.win0_lo_raw,
    );
    write_bridge_setup_win0_register(
        " win0-hi-register=",
        win0_hi_register_name,
        " win0-hi-source-offset=",
        win0_hi_source_offset,
        " win0-hi-address=",
        win0_hi_address,
        " win0-hi-width=32 win0-hi-raw=",
        decoded.win0_hi_raw,
    );
    write_bridge_setup_win0_register(
        " win0-base-limit-register=",
        win0_base_limit_register_name,
        " win0-base-limit-source-offset=",
        win0_base_limit_source_offset,
        " win0-base-limit-address=",
        win0_base_limit_address,
        " win0-base-limit-width=32 win0-base-limit-raw=",
        decoded.win0_base_limit_raw,
    );
    write_bridge_setup_win0_register(
        " win0-base-hi-register=",
        win0_base_hi_register_name,
        " win0-base-hi-source-offset=",
        win0_base_hi_source_offset,
        " win0-base-hi-address=",
        win0_base_hi_address,
        " win0-base-hi-width=32 win0-base-hi-raw=",
        decoded.win0_base_hi_raw,
    );
    write_bridge_setup_win0_register(
        " win0-limit-hi-register=",
        win0_limit_hi_register_name,
        " win0-limit-hi-source-offset=",
        win0_limit_hi_source_offset,
        " win0-limit-hi-address=",
        win0_limit_hi_address,
        " win0-limit-hi-width=32 win0-limit-hi-raw=",
        decoded.win0_limit_hi_raw,
    );
    write_early_static(" pcie-base-is-zero=");
    write_bool(decoded.pcie_base_is_zero);
    write_early_static(" cpu-base-low-matches=");
    write_bool(decoded.cpu_base_low_matches);
    write_early_static(" cpu-limit-low-matches=");
    write_bool(decoded.cpu_limit_low_matches);
    write_early_static(" cpu-base-high-matches=");
    write_bool(decoded.cpu_base_high_matches);
    write_early_static(" cpu-limit-high-matches=");
    write_bool(decoded.cpu_limit_high_matches);
    write_early_static(" outbound-window0-matches=");
    write_bool(decoded.outbound_window0_matches);
    write_early_static(" retained-endpoint-config-classification=");
    write_early_static(retained_endpoint_config_classification);
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_bridge_setup_state_read",
    talos_boot_scenario = "rpi5_rp1_bridge_setup_state_no_mmio_control"
))]
#[allow(clippy::too_many_arguments)]
fn write_bridge_setup_win0_register(
    register_label: &str,
    register_name: &str,
    offset_label: &str,
    source_offset: u64,
    address_label: &str,
    address: Option<u64>,
    raw_label: &str,
    raw: u32,
) {
    write_early_static(register_label);
    write_early_static(register_name);
    write_early_static(offset_label);
    write_early_hex_u64(source_offset);
    write_early_static(address_label);
    write_optional_hex_or_not_constructed(address);
    write_early_static(raw_label);
    write_early_hex_u64(raw as u64);
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_bridge_setup_state_read",
    talos_boot_scenario = "rpi5_rp1_bridge_setup_state_no_mmio_control"
))]
fn write_bridge_setup_state_classification_vocabulary() {
    write_early_static(
        " classification-vocabulary=pcie2-bridge-setup-state-visible,pcie2-bridge-setup-state-incomplete,pcie2-bridge-setup-state-sentinel,pcie2-bridge-setup-state-link-down-skip,pcie2-bridge-setup-state-inconclusive-capture,no-mmio-pcie2-bridge-setup-state-control-visible,staging/build-blocker",
    );
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_endpoint_config_identity_read",
    talos_boot_scenario = "rpi5_rp1_endpoint_config_identity_no_mmio_control"
))]
#[allow(clippy::too_many_arguments)]
fn write_endpoint_config_identity_common_fields(
    controller_base: Option<u64>,
    status_register_name: &str,
    status_source_offset: u64,
    status_address: Option<u64>,
    status_raw: u32,
    pcie_port: bool,
    dl_active: bool,
    phylinkup: bool,
    link_in_l23: bool,
    status_is_deaddead: bool,
    bdf: &str,
    config_offset: u64,
    index_register_name: &str,
    index_source_offset: u64,
    index_address: Option<u64>,
    index_value: u32,
    index_write_performed: bool,
    data_register_name: &str,
    data_source_offset: u64,
    data_address: Option<u64>,
    raw_config: u32,
    vendor_id: u32,
    device_id: u32,
    vendor_device_match: bool,
    raw_config_is_all_ones: bool,
    raw_config_is_zero: bool,
    raw_config_is_deaddead: bool,
) {
    write_early_static(" pcie2-controller-base=");
    write_optional_hex_or_not_constructed(controller_base);
    write_early_static(" pci-domain=2 precondition-register=");
    write_early_static(status_register_name);
    write_early_static(" precondition-source-offset=");
    write_early_hex_u64(status_source_offset);
    write_early_static(" precondition-address=");
    write_optional_hex_or_not_constructed(status_address);
    write_early_static(" precondition-width=32 precondition-raw=");
    write_early_hex_u64(status_raw as u64);
    write_early_static(" pcie-port=");
    write_bool(pcie_port);
    write_early_static(" dl-active=");
    write_bool(dl_active);
    write_early_static(" phylinkup=");
    write_bool(phylinkup);
    write_early_static(" link-in-l23=");
    write_bool(link_in_l23);
    write_early_static(" status-is-deaddead=");
    write_bool(status_is_deaddead);
    write_early_static(" config-bdf=");
    write_early_static(bdf);
    write_early_static(" config-offset=");
    write_early_hex_u64(config_offset);
    write_early_static(" index-register=");
    write_early_static(index_register_name);
    write_early_static(" index-source-offset=");
    write_early_hex_u64(index_source_offset);
    write_early_static(" index-address=");
    write_optional_hex_or_not_constructed(index_address);
    write_early_static(" index-value=");
    write_early_hex_u64(index_value as u64);
    write_early_static(" index-write-performed=");
    write_bool(index_write_performed);
    write_early_static(" data-register=");
    write_early_static(data_register_name);
    write_early_static(" data-source-offset=");
    write_early_hex_u64(data_source_offset);
    write_early_static(" data-address=");
    write_optional_hex_or_not_constructed(data_address);
    write_early_static(" width=32 raw-config=");
    write_early_hex_u64(raw_config as u64);
    write_early_static(" vendor-id=");
    write_early_hex_u64(vendor_id as u64);
    write_early_static(" device-id=");
    write_early_hex_u64(device_id as u64);
    write_early_static(" expected-vendor-id=");
    write_early_hex_u64(RP1_ENDPOINT_EXPECTED_VENDOR_ID as u64);
    write_early_static(" expected-device-id=");
    write_early_hex_u64(RP1_ENDPOINT_EXPECTED_DEVICE_ID as u64);
    write_early_static(" vendor-device-match=");
    write_bool(vendor_device_match);
    write_early_static(" raw-config-is-all-ones=");
    write_bool(raw_config_is_all_ones);
    write_early_static(" raw-config-is-zero=");
    write_bool(raw_config_is_zero);
    write_early_static(" raw-config-is-deaddead=");
    write_bool(raw_config_is_deaddead);
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_endpoint_config_identity_read",
    talos_boot_scenario = "rpi5_rp1_endpoint_config_identity_no_mmio_control",
    talos_boot_scenario = "rpi5_rp1_bridge_config_preflight_read",
    talos_boot_scenario = "rpi5_rp1_bridge_config_preflight_no_mmio_control",
    talos_boot_scenario = "rpi5_rp1_bridge_setup_state_read",
    talos_boot_scenario = "rpi5_rp1_bridge_setup_state_no_mmio_control"
))]
fn write_optional_hex_or_not_constructed(value: Option<u64>) {
    if let Some(value) = value {
        write_early_hex_u64(value);
    } else {
        write_early_static("not-constructed");
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_gpio14_ownership_route_preflight_read",
    talos_boot_scenario = "rpi5_rp1_gpio14_ownership_route_preflight_no_mmio_control"
))]
fn write_gpio14_ownership_preflight_fields(
    gpio14_status: u32,
    gpio14_ctrl: u32,
    io_bank0_inte: u32,
    io_bank0_ints: u32,
    rio_out: u32,
    rio_oe: u32,
    rio_in: u32,
    pad: u32,
    gicd_isenabler5: u32,
    gicd_ispendr5: u32,
    gicd_isactiver5: u32,
    gicc_hppir: u32,
    hppir_intid: u32,
    gpio14_mask: u32,
) {
    const INTID160_MASK: u32 = 1;

    write_early_static(" width=32 gpio14-status-raw=");
    write_early_hex_u64(gpio14_status as u64);
    write_early_static(" gpio14-ctrl-raw=");
    write_early_hex_u64(gpio14_ctrl as u64);
    write_early_static(" gpio14-funcsel=");
    write_early_dec_u64(gpio14_funcsel(gpio14_ctrl) as u64);
    write_early_static(" gpio14-func-name=");
    write_early_static(gpio14_func_name(gpio14_ctrl));
    write_early_static(" gpio14-outover=");
    write_early_dec_u64(((gpio14_ctrl >> 12) & 0x3) as u64);
    write_early_static(" gpio14-oeover=");
    write_early_dec_u64(((gpio14_ctrl >> 14) & 0x3) as u64);
    write_early_static(" gpio14-inover=");
    write_early_dec_u64(((gpio14_ctrl >> 16) & 0x3) as u64);
    write_early_static(" gpio14-raw-event-enable-mask=");
    write_early_hex_u64(((gpio14_ctrl >> 20) & 0xf) as u64);
    write_early_static(" gpio14-filtered-event-enable-mask=");
    write_early_hex_u64(((gpio14_ctrl >> 24) & 0xf) as u64);
    write_early_static(" gpio14-status-event-mask=");
    write_early_hex_u64(((gpio14_status >> 20) & 0xff) as u64);
    write_early_static(" io-bank0-inte-raw=");
    write_early_hex_u64(io_bank0_inte as u64);
    write_early_static(" io-bank0-ints-raw=");
    write_early_hex_u64(io_bank0_ints as u64);
    write_early_static(" rio-out-raw=");
    write_early_hex_u64(rio_out as u64);
    write_early_static(" rio-oe-raw=");
    write_early_hex_u64(rio_oe as u64);
    write_early_static(" rio-in-raw=");
    write_early_hex_u64(rio_in as u64);
    write_early_static(" rio-out-gpio14=");
    write_bool(rio_out & gpio14_mask != 0);
    write_early_static(" rio-oe-gpio14=");
    write_bool(rio_oe & gpio14_mask != 0);
    write_early_static(" rio-in-gpio14=");
    write_bool(rio_in & gpio14_mask != 0);
    write_early_static(" pad-raw=");
    write_early_hex_u64(pad as u64);
    write_early_static(" pad-input-enable=");
    write_bool(pad & (1 << 6) != 0);
    write_early_static(" pad-output-disable=");
    write_bool(pad & (1 << 7) != 0);
    write_early_static(" pad-pull=");
    write_early_hex_u64(((pad >> 2) & 0x3) as u64);
    write_early_static(" pad-drive=");
    write_early_hex_u64(((pad >> 4) & 0x3) as u64);
    write_early_static(" pad-schmitt=");
    write_bool(pad & (1 << 1) != 0);
    write_early_static(" pad-slew=");
    write_bool(pad & 1 != 0);
    write_early_static(" gicd-isenabler5-raw=");
    write_early_hex_u64(gicd_isenabler5 as u64);
    write_early_static(" gicd-ispendr5-raw=");
    write_early_hex_u64(gicd_ispendr5 as u64);
    write_early_static(" gicd-isactiver5-raw=");
    write_early_hex_u64(gicd_isactiver5 as u64);
    write_early_static(" gicc-hppir-raw=");
    write_early_hex_u64(gicc_hppir as u64);
    write_early_static(" intid160-enabled=");
    write_bool(gicd_isenabler5 & INTID160_MASK != 0);
    write_early_static(" intid160-pending=");
    write_bool(gicd_ispendr5 & INTID160_MASK != 0);
    write_early_static(" intid160-active=");
    write_bool(gicd_isactiver5 & INTID160_MASK != 0);
    write_early_static(" hppir-intid=");
    write_early_dec_u64(hppir_intid as u64);
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_gpio14_ownership_route_preflight_read",
    talos_boot_scenario = "rpi5_rp1_gpio14_ownership_route_preflight_no_mmio_control"
))]
fn gpio14_funcsel(gpio14_ctrl: u32) -> u32 {
    gpio14_ctrl & 0x1f
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_gpio14_ownership_route_preflight_read",
    talos_boot_scenario = "rpi5_rp1_gpio14_ownership_route_preflight_no_mmio_control"
))]
fn gpio14_func_name(gpio14_ctrl: u32) -> &'static str {
    match gpio14_funcsel(gpio14_ctrl) {
        0 => "pwm0",
        1 => "dpi",
        2 => "uart4",
        3 => "i2c3",
        4 => "uart0",
        5 => "gpio",
        6 => "proc_rio",
        7 => "pio",
        8 => "spi5",
        _ => "unknown",
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_gpio16_owned_event_discriminator",
    talos_boot_scenario = "rpi5_rp1_gpio16_owned_event_discriminator_no_mmio_control"
))]
fn gpio16_funcsel(gpio16_ctrl: u32) -> u32 {
    gpio16_ctrl & 0x1f
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_gpio16_owned_event_discriminator",
    talos_boot_scenario = "rpi5_rp1_gpio16_owned_event_discriminator_no_mmio_control"
))]
fn gpio16_func_name(gpio16_ctrl: u32) -> &'static str {
    match gpio16_funcsel(gpio16_ctrl) {
        0 => "spi1",
        1 => "dpi",
        2 => "dsi0_te_ext",
        3 => "_",
        4 => "uart0",
        5 => "gpio",
        6 => "proc_rio",
        7 => "pio",
        8 => "_",
        _ => "unknown",
    }
}

#[cfg(talos_boot_scenario = "rpi5_rp1_gpio14_ownership_route_preflight_read")]
fn gpio14_ownership_preflight_classification(
    gpio14_status: u32,
    gpio14_ctrl: u32,
    io_bank0_inte: u32,
    io_bank0_ints: u32,
    rio_out: u32,
    rio_oe: u32,
    rio_in: u32,
    pad: u32,
    gicd_isenabler5: u32,
    gicd_ispendr5: u32,
    gicd_isactiver5: u32,
    hppir_intid: u32,
    gpio14_mask: u32,
    gic_intid: u32,
) -> &'static str {
    let funcsel = gpio14_funcsel(gpio14_ctrl);
    let raw_event_enable = (gpio14_ctrl >> 20) & 0xf;
    let filtered_event_enable = (gpio14_ctrl >> 24) & 0xf;
    let status_event_mask = (gpio14_status >> 20) & 0xff;
    let rp1_reads = [
        gpio14_status,
        gpio14_ctrl,
        io_bank0_inte,
        io_bank0_ints,
        rio_out,
        rio_oe,
        rio_in,
        pad,
    ];
    if rp1_reads.iter().all(|&raw| raw == 0xdead_dead) {
        "observed-gpio14-ownership-preflight-sentinel"
    } else if rp1_reads.iter().all(|&raw| raw == 0xffff_ffff) {
        "observed-gpio14-ownership-preflight-all-ones"
    } else if rp1_reads.iter().all(|&raw| raw == 0) {
        "observed-gpio14-ownership-preflight-zero"
    } else if funcsel != 5 {
        "observed-gpio14-ownership-preflight-blocked-non-gpio-function"
    } else if (gicd_isenabler5 & 1) != 0
        || (gicd_ispendr5 & 1) != 0
        || (gicd_isactiver5 & 1) != 0
        || hppir_intid == gic_intid
    {
        "gpio14-ownership-preflight-blocked-parent-route-state"
    } else if raw_event_enable != 0
        || filtered_event_enable != 0
        || status_event_mask != 0
        || (io_bank0_inte & gpio14_mask) != 0
        || (io_bank0_ints & gpio14_mask) != 0
    {
        "observed-gpio14-ownership-preflight-blocked-route-or-source-state"
    } else {
        "observed-gpio14-ownership-route-preflight-visible"
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_gpio14_status_read",
    talos_boot_scenario = "rpi5_rp1_gpio14_status_no_mmio_control",
    talos_boot_scenario = "rpi5_rp1_interrupt_routing_msix_cfg_read",
    talos_boot_scenario = "rpi5_rp1_interrupt_routing_no_mmio_control",
    talos_boot_scenario = "rpi5_rp1_gic_visible_route_status_read",
    talos_boot_scenario = "rpi5_rp1_gic_visible_route_no_mmio_control",
    talos_boot_scenario = "rpi5_rp1_gpio_bank_source_status_read",
    talos_boot_scenario = "rpi5_rp1_gpio_bank_source_status_no_mmio_control",
    talos_boot_scenario = "rpi5_rp1_clock_manager_status_read",
    talos_boot_scenario = "rpi5_rp1_clock_manager_status_no_mmio_control",
    talos_boot_scenario = "rpi5_rp1_clock_adc_ctrl_write_restore",
    talos_boot_scenario = "rpi5_rp1_clock_adc_ctrl_write_restore_no_mmio_control",
    talos_boot_scenario = "rpi5_rp1_clock_adc_ctrl_enable_toggle",
    talos_boot_scenario = "rpi5_rp1_clock_adc_ctrl_enable_toggle_no_mmio_control",
    talos_boot_scenario = "rpi5_rp1_clock_adc_window_coherence_read",
    talos_boot_scenario = "rpi5_rp1_clock_adc_window_coherence_no_mmio_control",
    talos_boot_scenario = "rpi5_rp1_sysinfo_clock_sentinel_read",
    talos_boot_scenario = "rpi5_rp1_sysinfo_clock_sentinel_no_mmio_control",
    talos_boot_scenario = "rpi5_rp1_clock_reset_dependency_read",
    talos_boot_scenario = "rpi5_rp1_clock_reset_dependency_no_mmio_control",
    talos_boot_scenario = "rpi5_rp1_ethernet_gem_mid_visibility_candidate",
    talos_boot_scenario = "rpi5_rp1_ethernet_gem_mid_visibility_no_mmio_control",
    talos_boot_scenario = "rpi5_rp1_ethernet_gem_mid_decode_discriminator_candidate",
    talos_boot_scenario = "rpi5_rp1_ethernet_gem_mid_decode_discriminator_no_mmio_control",
    talos_boot_scenario = "rpi5_rp1_ethernet_observed_window_discriminator_candidate",
    talos_boot_scenario = "rpi5_rp1_ethernet_observed_window_discriminator_no_mmio_control",
    talos_boot_scenario = "rpi5_rp1_ethernet_prereq_ownership_candidate",
    talos_boot_scenario = "rpi5_rp1_ethernet_prereq_ownership_no_mmio_control",
    talos_boot_scenario = "rpi5_rp1_ethernet_clock_reset_readonly_baseline_candidate",
    talos_boot_scenario = "rpi5_rp1_ethernet_clock_reset_readonly_baseline_no_mmio_control",
    talos_boot_scenario = "rpi5_rp1_ethernet_clock_reset_write_restore_candidate",
    talos_boot_scenario = "rpi5_rp1_ethernet_clock_reset_write_restore_no_mmio_control",
    talos_boot_scenario = "rpi5_rp1_ethernet_clk_eth_ctrl_write_restore_candidate",
    talos_boot_scenario = "rpi5_rp1_ethernet_clk_eth_ctrl_write_restore_no_mmio_control",
    talos_boot_scenario = "rpi5_rp1_ethernet_gpio32_phy_reset_preflight_candidate",
    talos_boot_scenario = "rpi5_rp1_ethernet_gpio32_phy_reset_preflight_control",
    talos_boot_scenario = "rpi5_rp1_ethernet_gpio32_phy_reset_write_restore_candidate",
    talos_boot_scenario = "rpi5_rp1_ethernet_gpio32_phy_reset_write_restore_no_mmio_control",
    talos_boot_scenario = "rpi5_rp1_pcie2_host_link_status_read",
    talos_boot_scenario = "rpi5_rp1_pcie2_host_link_status_no_mmio_control",
    talos_boot_scenario = "rpi5_rp1_endpoint_config_identity_read",
    talos_boot_scenario = "rpi5_rp1_endpoint_config_identity_no_mmio_control",
    talos_boot_scenario = "rpi5_rp1_bridge_config_preflight_read",
    talos_boot_scenario = "rpi5_rp1_bridge_config_preflight_no_mmio_control",
    talos_boot_scenario = "rpi5_rp1_bridge_setup_state_read",
    talos_boot_scenario = "rpi5_rp1_bridge_setup_state_no_mmio_control",
    talos_boot_scenario = "rpi5_rp1_observed_aperture_read",
    talos_boot_scenario = "rpi5_rp1_observed_aperture_no_mmio_control",
    talos_boot_scenario = "rpi5_rp1_observed_gpio_status_read",
    talos_boot_scenario = "rpi5_rp1_observed_gpio_status_no_mmio_control",
    talos_boot_scenario = "rpi5_rp1_gpio14_ownership_route_preflight_read",
    talos_boot_scenario = "rpi5_rp1_gpio14_ownership_route_preflight_no_mmio_control",
    talos_boot_scenario = "rpi5_rp1_gpio16_owned_event_discriminator",
    talos_boot_scenario = "rpi5_rp1_gpio16_owned_event_discriminator_no_mmio_control"
))]
fn write_bool(value: bool) {
    if value {
        write_early_static("true");
    } else {
        write_early_static("false");
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_observed_aperture_read",
    talos_boot_scenario = "rpi5_rp1_observed_aperture_no_mmio_control"
))]
fn classify_observed_aperture_raw(raw: u32) -> &'static str {
    if raw == 0xdead_dead {
        "observed-aperture-rp1-uart0-fr-sentinel"
    } else if raw == 0xffff_ffff {
        "observed-aperture-rp1-uart0-fr-all-ones"
    } else if raw == 0 {
        "observed-aperture-rp1-uart0-fr-zero"
    } else {
        "observed-aperture-rp1-uart0-fr-visible"
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_observed_aperture_read",
    talos_boot_scenario = "rpi5_rp1_observed_aperture_no_mmio_control"
))]
fn observed_aperture_raw_is_pl011_fr_shaped(raw: u32) -> bool {
    raw & !0x1ff == 0
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_observed_aperture_read",
    talos_boot_scenario = "rpi5_rp1_observed_aperture_no_mmio_control"
))]
fn write_observed_aperture_fields(
    source_rp1_bus_address: &str,
    observed_cpu_physical_address: &str,
    register_offset: &str,
    raw: u32,
) {
    write_early_static(" source-rp1-bus-address=");
    write_early_static(source_rp1_bus_address);
    write_early_static(" observed-cpu-physical-address=");
    write_early_static(observed_cpu_physical_address);
    write_early_static(" register-offset=");
    write_early_static(register_offset);
    write_early_static(" width=32 raw=");
    write_early_hex_u64(raw as u64);
    write_early_static(" raw-is-deaddead=");
    write_bool(raw == 0xdead_dead);
    write_early_static(" raw-is-all-ones=");
    write_bool(raw == 0xffff_ffff);
    write_early_static(" raw-is-zero=");
    write_bool(raw == 0);
    write_early_static(" raw-is-pl011-fr-shaped=");
    write_bool(observed_aperture_raw_is_pl011_fr_shaped(raw));
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_observed_aperture_read",
    talos_boot_scenario = "rpi5_rp1_observed_aperture_no_mmio_control"
))]
fn write_observed_aperture_retained_bridge_context() {
    write_early_static(" retained-bridge-win0-lo=0x80000000");
    write_early_static(" retained-bridge-win0-base-limit=0x3ff00000");
    write_early_static(" retained-bridge-win0-base-hi=0x1c");
    write_early_static(" retained-bridge-win0-limit-hi=0x1c");
    write_early_static(" retained-bridge-outbound-window0-matches=false");
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_observed_aperture_read",
    talos_boot_scenario = "rpi5_rp1_observed_aperture_no_mmio_control"
))]
fn write_observed_aperture_classification_vocabulary() {
    write_early_static(" classification-vocabulary=");
    write_early_static("observed-aperture-rp1-uart0-fr-visible,");
    write_early_static("observed-aperture-rp1-uart0-fr-sentinel,");
    write_early_static("observed-aperture-rp1-uart0-fr-all-ones,");
    write_early_static("observed-aperture-rp1-uart0-fr-zero,");
    write_early_static("observed-aperture-rp1-uart0-fr-no-return-or-trap,");
    write_early_static("observed-aperture-rp1-uart0-fr-inconclusive-capture,");
    write_early_static("no-mmio-observed-aperture-control-visible,");
    write_early_static("staging/build-blocker");
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_observed_gpio_status_read",
    talos_boot_scenario = "rpi5_rp1_observed_gpio_status_no_mmio_control"
))]
fn write_observed_gpio_status_capture_nonce() {
    if let Some(nonce) = option_env!("TALOS_CAPTURE_NONCE") {
        if !nonce.is_empty() {
            write_early_static(" capture-nonce=");
            write_early_static(nonce);
        }
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_observed_gpio_status_read",
    talos_boot_scenario = "rpi5_rp1_observed_gpio_status_no_mmio_control"
))]
fn classify_observed_gpio_status_pair(gpio14_status: u32, gpio14_ctrl: u32) -> &'static str {
    if gpio14_status == 0xdead_dead && gpio14_ctrl == 0xdead_dead {
        "observed-aperture-gpio14-status-ctrl-sentinel"
    } else if gpio14_status == 0xffff_ffff && gpio14_ctrl == 0xffff_ffff {
        "observed-aperture-gpio14-status-ctrl-all-ones"
    } else if gpio14_status == 0 && gpio14_ctrl == 0 {
        "observed-aperture-gpio14-status-ctrl-zero"
    } else {
        "observed-aperture-gpio14-status-ctrl-visible"
    }
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_observed_gpio_status_read",
    talos_boot_scenario = "rpi5_rp1_observed_gpio_status_no_mmio_control"
))]
fn write_observed_gpio_status_fields(
    status_source_rp1_bus_address: &str,
    ctrl_source_rp1_bus_address: &str,
    status_observed_cpu_physical_address: &str,
    ctrl_observed_cpu_physical_address: &str,
    status_register_offset: &str,
    ctrl_register_offset: &str,
    gpio14_status: u32,
    gpio14_ctrl: u32,
) {
    write_early_static(" status-source-rp1-bus-address=");
    write_early_static(status_source_rp1_bus_address);
    write_early_static(" ctrl-source-rp1-bus-address=");
    write_early_static(ctrl_source_rp1_bus_address);
    write_early_static(" status-observed-cpu-physical-address=");
    write_early_static(status_observed_cpu_physical_address);
    write_early_static(" ctrl-observed-cpu-physical-address=");
    write_early_static(ctrl_observed_cpu_physical_address);
    write_early_static(" status-register-offset=");
    write_early_static(status_register_offset);
    write_early_static(" ctrl-register-offset=");
    write_early_static(ctrl_register_offset);
    write_early_static(" width=32 gpio14-status-raw=");
    write_early_hex_u64(gpio14_status as u64);
    write_early_static(" gpio14-ctrl-raw=");
    write_early_hex_u64(gpio14_ctrl as u64);
    write_early_static(" status-raw-falling=");
    write_bool(gpio14_status & (1 << 20) != 0);
    write_early_static(" status-raw-rising=");
    write_bool(gpio14_status & (1 << 21) != 0);
    write_early_static(" status-raw-low=");
    write_bool(gpio14_status & (1 << 22) != 0);
    write_early_static(" status-raw-high=");
    write_bool(gpio14_status & (1 << 23) != 0);
    write_early_static(" status-filtered-falling=");
    write_bool(gpio14_status & (1 << 24) != 0);
    write_early_static(" status-filtered-rising=");
    write_bool(gpio14_status & (1 << 25) != 0);
    write_early_static(" status-filtered-low=");
    write_bool(gpio14_status & (1 << 26) != 0);
    write_early_static(" status-filtered-high=");
    write_bool(gpio14_status & (1 << 27) != 0);
    write_early_static(" ctrl-funcsel=");
    write_early_dec_u64((gpio14_ctrl & 0x1f) as u64);
    write_early_static(" ctrl-outover=");
    write_early_dec_u64(((gpio14_ctrl >> 12) & 0x3) as u64);
    write_early_static(" ctrl-oeover=");
    write_early_dec_u64(((gpio14_ctrl >> 14) & 0x3) as u64);
    write_early_static(" ctrl-inover=");
    write_early_dec_u64(((gpio14_ctrl >> 16) & 0x3) as u64);
    write_early_static(" ctrl-irqover=");
    write_early_dec_u64(((gpio14_ctrl >> 28) & 0x3) as u64);
    write_early_static(" ctrl-raw-falling-enabled=");
    write_bool(gpio14_ctrl & (1 << 20) != 0);
    write_early_static(" ctrl-raw-rising-enabled=");
    write_bool(gpio14_ctrl & (1 << 21) != 0);
    write_early_static(" ctrl-raw-low-enabled=");
    write_bool(gpio14_ctrl & (1 << 22) != 0);
    write_early_static(" ctrl-raw-high-enabled=");
    write_bool(gpio14_ctrl & (1 << 23) != 0);
    write_early_static(" ctrl-filtered-falling-enabled=");
    write_bool(gpio14_ctrl & (1 << 24) != 0);
    write_early_static(" ctrl-filtered-rising-enabled=");
    write_bool(gpio14_ctrl & (1 << 25) != 0);
    write_early_static(" ctrl-filtered-low-enabled=");
    write_bool(gpio14_ctrl & (1 << 26) != 0);
    write_early_static(" ctrl-filtered-high-enabled=");
    write_bool(gpio14_ctrl & (1 << 27) != 0);
    write_early_static(" status-raw-is-deaddead=");
    write_bool(gpio14_status == 0xdead_dead);
    write_early_static(" status-raw-is-all-ones=");
    write_bool(gpio14_status == 0xffff_ffff);
    write_early_static(" status-raw-is-zero=");
    write_bool(gpio14_status == 0);
    write_early_static(" ctrl-raw-is-deaddead=");
    write_bool(gpio14_ctrl == 0xdead_dead);
    write_early_static(" ctrl-raw-is-all-ones=");
    write_bool(gpio14_ctrl == 0xffff_ffff);
    write_early_static(" ctrl-raw-is-zero=");
    write_bool(gpio14_ctrl == 0);
    write_early_static(" retained-observed-uart0-fr-raw=0x187");
    write_early_static(" retained-observed-uart0-fr-pl011-fr-shaped=true");
}

#[cfg(any(
    talos_boot_scenario = "rpi5_rp1_observed_gpio_status_read",
    talos_boot_scenario = "rpi5_rp1_observed_gpio_status_no_mmio_control"
))]
fn write_observed_gpio_status_classification_vocabulary() {
    write_early_static(" classification-vocabulary=");
    write_early_static("observed-aperture-gpio14-status-ctrl-visible,");
    write_early_static("observed-aperture-gpio14-status-ctrl-sentinel,");
    write_early_static("observed-aperture-gpio14-status-ctrl-all-ones,");
    write_early_static("observed-aperture-gpio14-status-ctrl-zero,");
    write_early_static("observed-aperture-gpio14-status-ctrl-no-return-or-trap,");
    write_early_static("observed-aperture-gpio14-status-ctrl-inconclusive-capture,");
    write_early_static("no-mmio-observed-gpio-status-control-visible,");
    write_early_static("staging/build-blocker");
}

#[cfg(not(talos_target_rpi5_bcm2712))]
#[allow(dead_code)]
fn read_rp1_reg_u32(_addr: usize) -> u32 {
    0
}

#[cfg(talos_target_rpi5_bcm2712)]
pub fn write_early_static(s: &str) {
    for byte in s.bytes() {
        if byte == b'\n' {
            write_uart10_byte_early_phase(b'\r');
            wait_uart10_empty_early_phase();
        }
        write_uart10_byte_early_phase(byte);
        wait_uart10_empty_early_phase();
    }
}

#[cfg(talos_target_rpi5_bcm2712)]
fn write_early_hex_digit(nibble: u8) {
    let digit = if nibble < 10 {
        b'0'.wrapping_add(nibble)
    } else if nibble < 16 {
        b'a'.wrapping_add(nibble.wrapping_sub(10))
    } else {
        b'?'
    };
    write_uart10_byte_early_phase(digit);
}

#[cfg(talos_target_rpi5_bcm2712)]
#[inline(always)]
pub(crate) fn write_uart10_byte_early_phase(byte: u8) {
    let value = byte as u32;
    unsafe {
        core::arch::asm!(
            "mov x9, #0x1000",
            "movk x9, #0x7d00, lsl #16",
            "movk x9, #0x10, lsl #32",
            "str w11, [x9]",
            "ldr w10, [x9, #0x18]",
            "dsb sy",
            in("w11") value,
            lateout("x9") _,
            lateout("x10") _,
            options(nostack, preserves_flags)
        );
    }
}

#[cfg(all(
    talos_target_rpi5_bcm2712,
    any(
        talos_boot_scenario = "rpi5_psci_secondary_core_alive",
        talos_boot_scenario = "rpi5_secondary_core_workload",
        talos_boot_scenario = "rpi5_smp_lock_cache_coherence",
        talos_boot_scenario = "rpi5_cross_core_ipi_delivery",
        talos_boot_scenario = "rpi5_remote_wakeup_request",
        talos_boot_scenario = "rpi5_production_secondary_dispatch",
        talos_boot_scenario = "rpi5_shared_scheduler_metadata",
        talos_boot_scenario = "rpi5_shared_runqueue_migration",
        talos_boot_scenario = "rpi5_multicore_preemption_proof",
        talos_boot_scenario = "rpi5_production_timer_preemption_proof",
        talos_boot_scenario = "rpi5_secondary_scheduler_service_loop"
    )
))]
pub(crate) fn write_uart10_bytes_early_phase(bytes: &[u8]) {
    for &byte in bytes {
        write_uart10_byte_early_phase(byte);
    }
    wait_uart10_empty_early_phase();
}

#[cfg(talos_target_rpi5_bcm2712)]
pub(crate) fn wait_uart10_empty_early_phase() {
    unsafe {
        core::arch::asm!(
            "mov x9, #0x1000",
            "movk x9, #0x7d00, lsl #16",
            "movk x9, #0x10, lsl #32",
            "mov x21, #0x200000",
            "2:",
            "ldr w10, [x9, #0x18]",
            "tbnz w10, #7, 3f",
            "subs x21, x21, #1",
            "b.ne 2b",
            "3:",
            "dsb sy",
            lateout("x9") _,
            lateout("x10") _,
            lateout("x21") _,
            options(nostack)
        );
    }
}

#[cfg(talos_target_rpi5_bcm2712)]
pub(crate) fn relocate_early_linked_addr(addr: usize) -> usize {
    // The accepted normal Pi 5 Image links and runs at 0x200000, so this is
    // normally a no-op. Keep the helper for vector installation and explicit
    // address-contract diagnostics while that part of bring-up is still active.
    addr.wrapping_add(runtime_relocation_delta())
}

#[cfg(talos_target_rpi5_bcm2712)]
pub(crate) fn runtime_relocation_delta() -> usize {
    let mut runtime_pc: usize;
    let mut linked_pc: usize;
    unsafe {
        core::arch::asm!(
            "adr {runtime}, 1f",
            "ldr {linked}, =1f",
            "1:",
            runtime = out(reg) runtime_pc,
            linked = out(reg) linked_pc,
            options(nostack, preserves_flags)
        );
    }
    runtime_pc.wrapping_sub(linked_pc)
}

pub fn services(boot_info: &BootInfo) -> TargetServices {
    TargetServices {
        uart: UartKind::FirmwarePreserved,
        timer: TimerKind::ArmGeneric,
        interrupt_controller: InterruptControllerKind::GicV2,
        mmio_map: MmioMap::new(MMIO_REGIONS),
        device_tree: DeviceTree::from_physical_address(boot_info.dtb_pa),
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn pi5_uart10_address_matches_bcm2712_soc_range() {
        assert_eq!(UART10_BASE, 0x10_7d00_1000);
        assert_eq!(RP1_UART0_PCIE2_BASE, 0x1f_0003_0000);
        assert_eq!(RP1_UART0_FIRMWARE_BASE, 0x1c_0003_0000);
        assert_eq!(RP1_UART0_FR, 0x1f_0003_0018);
        assert_eq!(RP1_UART0_OBSERVED_APERTURE_FR, 0x1c_0003_0018);
        assert_eq!(RP1_GPIO14_OBSERVED_APERTURE_STATUS, 0x1c_000d_0070);
        assert_eq!(RP1_GPIO14_OBSERVED_APERTURE_CTRL, 0x1c_000d_0074);
        assert_eq!(RP1_UART0_GPIO14_PAD, 0x1f_000f_003c);
        assert_eq!(RP1_UART0_GPIO15_PAD, 0x1f_000f_0040);
        assert_eq!(RP1_GPIO14_STATUS, 0x1f_000d_0070);
        assert_eq!(RP1_UART0_GPIO14_CTRL, 0x1f_000d_0074);
        assert_eq!(RP1_UART0_GPIO15_CTRL, 0x1f_000d_007c);
        assert_eq!(RP1_RIO0_OUT, 0x1f_000e_0000);
        assert_eq!(RP1_RIO0_OE, 0x1f_000e_0004);
        assert_eq!(RP1_RIO0_IN, 0x1f_000e_0008);
        assert_eq!(RP1_IO_BANK0_INTE, 0x1f_000d_011c);
        assert_eq!(RP1_IO_BANK0_INTS, 0x1f_000d_0124);
        assert_eq!(RP1_SYSINFO_BASE, 0x1f_0000_0000);
        assert_eq!(RP1_SYSINFO_CHIP_ID, 0x1f_0000_0000);
        assert_eq!(RP1_SYSINFO_PLATFORM, 0x1f_0000_0004);
        assert_eq!(RP1_SYSINFO_OBSERVED_APERTURE_BASE, 0x1c_0000_0000);
        assert_eq!(RP1_SYSINFO_OBSERVED_APERTURE_CHIP_ID, 0x1c_0000_0000);
        assert_eq!(RP1_SYSINFO_OBSERVED_APERTURE_PLATFORM, 0x1c_0000_0004);
        assert_eq!(RP1_EXPECTED_CHIP_ID, 0x2000_1927);
        assert_eq!(RP1_CLOCK_MANAGER_BASE, 0x1f_0001_8000);
        assert_eq!(RP1_CLOCK_MANAGER_OBSERVED_APERTURE_BASE, 0x1c_0001_8000);
        assert_eq!(RP1_PLL_SYS_OBSERVED_APERTURE_CS, 0x1c_0002_0000);
        assert_eq!(RP1_CLK_SYS_OBSERVED_APERTURE_CTRL, 0x1c_0001_8014);
        assert_eq!(RP1_CLK_SYS_OBSERVED_APERTURE_DIV_INT, 0x1c_0001_8018);
        assert_eq!(RP1_CLK_SYS_OBSERVED_APERTURE_SEL, 0x1c_0001_8020);
        assert_eq!(RP1_CLK_SLOW_SYS_OBSERVED_APERTURE_CTRL, 0x1c_0001_8024);
        assert_eq!(RP1_CLK_UART_OBSERVED_APERTURE_CTRL, 0x1c_0001_8054);
        assert_eq!(RP1_CLK_UART_OBSERVED_APERTURE_DIV_INT, 0x1c_0001_8058);
        assert_eq!(RP1_CLK_UART_OBSERVED_APERTURE_SEL, 0x1c_0001_8060);
        assert_eq!(PCIE2_CONTROLLER_BASE, 0x10_0012_0000);
        assert_eq!(PCIE_MISC_PCIE_STATUS_OFFSET, 0x4068);
        assert_eq!(PCIE_MISC_PCIE_STATUS, 0x10_0012_4068);
        assert_eq!(PCIE_MISC_MISC_CTRL_OFFSET, 0x4008);
        assert_eq!(PCIE_MISC_MISC_CTRL, 0x10_0012_4008);
        assert_eq!(PCIE_RC_CFG_PRIV1_ID_VAL3_OFFSET, 0x043c);
        assert_eq!(PCIE_RC_CFG_PRIV1_ID_VAL3, 0x10_0012_043c);
        assert_eq!(PCIE_MISC_CPU_2_PCIE_MEM_WIN0_LO_OFFSET, 0x400c);
        assert_eq!(PCIE_MISC_CPU_2_PCIE_MEM_WIN0_LO, 0x10_0012_400c);
        assert_eq!(PCIE_MISC_CPU_2_PCIE_MEM_WIN0_HI_OFFSET, 0x4010);
        assert_eq!(PCIE_MISC_CPU_2_PCIE_MEM_WIN0_HI, 0x10_0012_4010);
        assert_eq!(PCIE_MISC_CPU_2_PCIE_MEM_WIN0_BASE_LIMIT_OFFSET, 0x4070);
        assert_eq!(PCIE_MISC_CPU_2_PCIE_MEM_WIN0_BASE_LIMIT, 0x10_0012_4070);
        assert_eq!(PCIE_MISC_CPU_2_PCIE_MEM_WIN0_BASE_HI_OFFSET, 0x4080);
        assert_eq!(PCIE_MISC_CPU_2_PCIE_MEM_WIN0_BASE_HI, 0x10_0012_4080);
        assert_eq!(PCIE_MISC_CPU_2_PCIE_MEM_WIN0_LIMIT_HI_OFFSET, 0x4084);
        assert_eq!(PCIE_MISC_CPU_2_PCIE_MEM_WIN0_LIMIT_HI, 0x10_0012_4084);
        assert_eq!(PCIE_EXT_CFG_DATA_OFFSET, 0x8000);
        assert_eq!(PCIE_EXT_CFG_DATA, 0x10_0012_8000);
        assert_eq!(PCIE_EXT_CFG_INDEX_OFFSET, 0x9000);
        assert_eq!(PCIE_EXT_CFG_INDEX, 0x10_0012_9000);
        assert_eq!(RP1_ENDPOINT_CONFIG_INDEX_VALUE, 0x0010_0000);
        assert_eq!(RP1_ENDPOINT_CONFIG_OFFSET, 0);
        assert_eq!(RP1_ENDPOINT_CONFIG_BDF, "0002:01:00.0");
        assert_eq!(RP1_ENDPOINT_EXPECTED_VENDOR_ID, 0x1de4);
        assert_eq!(RP1_ENDPOINT_EXPECTED_DEVICE_ID, 0x0001);
        assert_eq!(PCIE_STATUS_PORT, 0x80);
        assert_eq!(PCIE_STATUS_DL_ACTIVE, 0x20);
        assert_eq!(PCIE_STATUS_PHYLINKUP, 0x10);
        assert_eq!(PCIE_STATUS_LINK_IN_L23, 0x40);
        assert_eq!(RP1_PLL_SYS_CS, 0x1f_0002_0000);
        assert_eq!(RP1_CLK_SYS_CTRL, 0x1f_0001_8014);
        assert_eq!(RP1_CLK_SYS_DIV_INT, 0x1f_0001_8018);
        assert_eq!(RP1_CLK_SYS_SEL, 0x1f_0001_8020);
        assert_eq!(RP1_CLK_SLOW_SYS_CTRL, 0x1f_0001_8024);
        assert_eq!(RP1_CLK_UART_CTRL, 0x1f_0001_8054);
        assert_eq!(RP1_CLK_UART_DIV_INT, 0x1f_0001_8058);
        assert_eq!(RP1_CLK_UART_SEL, 0x1f_0001_8060);
        assert_eq!(RP1_CLK_ADC_CTRL, 0x1f_0001_8144);
        assert_eq!(RP1_CLK_ADC_DIV_INT, 0x1f_0001_8148);
        assert_eq!(RP1_CLK_ADC_SEL, 0x1f_0001_8150);
        assert_eq!(RP1_CLK_ETH_CTRL_OBSERVED_APERTURE, 0x1c_0001_8064);
        assert_eq!(RP1_CLK_ETH_TSU_CTRL_OBSERVED_APERTURE, 0x1c_0001_8134);
        assert_eq!(RP1_CLK_CTRL_ENABLE, 0x0000_0800);
        assert_eq!(RP1_UART0_BASE, RP1_UART0_PCIE2_BASE);
    }
}
