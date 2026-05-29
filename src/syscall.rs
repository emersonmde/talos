//! Target-independent syscall ABI dispatch primitives.
//!
//! This module owns only the first stable scalar syscall vocabulary and return
//! encoding. It does not route exception vectors, enter EL0, copy user memory,
//! mutate descriptor tables, load programs, or provide VFS/filesystem behavior.

use crate::posix::PosixError;

pub(crate) const STABLE_SVC_IMMEDIATE: u16 = 0;
pub(crate) const DIAGNOSTIC_EL0_TRAP_SVC_IMMEDIATE: u16 = 0x7a10;
pub(crate) const TALOS_NOP_SYSCALL: u64 = 0;
#[cfg(any(test, talos_boot_scenario = "qemu_pointer_copy_smoke"))]
pub(crate) const TALOS_COPY_PROBE_SYSCALL: u64 = 0x7001;
#[cfg(any(test, talos_boot_scenario = "qemu_pointer_copy_smoke"))]
pub(crate) const TALOS_COPY_PROBE_MAX_LEN: usize = 32;
pub(crate) const MAX_SCALAR_ARGUMENTS: usize = 6;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SyscallNumber {
    TalosNop,
    Unknown(u64),
}

impl SyscallNumber {
    pub(crate) const fn from_raw(raw: u64) -> Self {
        match raw {
            TALOS_NOP_SYSCALL => Self::TalosNop,
            unknown => Self::Unknown(unknown),
        }
    }

    pub(crate) const fn raw(self) -> u64 {
        match self {
            Self::TalosNop => TALOS_NOP_SYSCALL,
            Self::Unknown(raw) => raw,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SyscallArguments {
    values: [u64; MAX_SCALAR_ARGUMENTS],
}

impl SyscallArguments {
    pub(crate) const fn new(values: [u64; MAX_SCALAR_ARGUMENTS]) -> Self {
        Self { values }
    }

    pub(crate) const fn empty() -> Self {
        Self {
            values: [0; MAX_SCALAR_ARGUMENTS],
        }
    }

    pub(crate) const fn get(self, index: usize) -> Option<u64> {
        if index < MAX_SCALAR_ARGUMENTS {
            Some(self.values[index])
        } else {
            None
        }
    }

    pub(crate) const fn values(self) -> [u64; MAX_SCALAR_ARGUMENTS] {
        self.values
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SyscallReturn {
    x0: u64,
}

impl SyscallReturn {
    pub(crate) const fn success(value: u64) -> Self {
        Self { x0: value }
    }

    pub(crate) const fn error(error: PosixError) -> Self {
        match errno_number(error) {
            Some(errno) => Self {
                x0: (errno as u64).wrapping_neg(),
            },
            None => Self {
                x0: (ENOSYS as u64).wrapping_neg(),
            },
        }
    }

    pub(crate) const fn x0(self) -> u64 {
        self.x0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SyscallDispatchResult {
    number: SyscallNumber,
    arguments: SyscallArguments,
    return_value: SyscallReturn,
}

impl SyscallDispatchResult {
    pub(crate) const fn number(self) -> SyscallNumber {
        self.number
    }

    pub(crate) const fn arguments(self) -> SyscallArguments {
        self.arguments
    }

    pub(crate) const fn return_value(self) -> SyscallReturn {
        self.return_value
    }
}

pub(crate) const EINVAL: u16 = 22;
pub(crate) const EBADF: u16 = 9;
pub(crate) const EFAULT: u16 = 14;
pub(crate) const ENOSYS: u16 = 38;
pub(crate) const ENOTSUP: u16 = 95;

pub(crate) const fn errno_number(error: PosixError) -> Option<u16> {
    match error {
        PosixError::InvalidArgument => Some(EINVAL),
        PosixError::BadDescriptor => Some(EBADF),
        PosixError::Fault => Some(EFAULT),
        PosixError::NotImplemented => Some(ENOSYS),
        PosixError::NotSupported => Some(ENOTSUP),
        _ => None,
    }
}

pub(crate) const fn is_stable_syscall_svc_immediate(immediate: u16) -> bool {
    immediate == STABLE_SVC_IMMEDIATE
}

pub(crate) const fn dispatch(
    raw_number: u64,
    arguments: SyscallArguments,
) -> SyscallDispatchResult {
    let number = SyscallNumber::from_raw(raw_number);
    let return_value = match number {
        SyscallNumber::TalosNop => SyscallReturn::success(0),
        SyscallNumber::Unknown(_) => SyscallReturn::error(PosixError::NotImplemented),
    };

    SyscallDispatchResult {
        number,
        arguments,
        return_value,
    }
}

#[cfg(any(test, talos_boot_scenario = "qemu_pointer_copy_smoke"))]
pub(crate) fn dispatch_copy_probe(
    arguments: SyscallArguments,
    mappings: &[crate::posix::UserMapping],
    user_memory_start: u64,
    user_memory: &mut [u8],
) -> SyscallReturn {
    let [
        user_start,
        len,
        expected,
        replacement,
        scratch_selector,
        flags,
    ] = arguments.values();
    if scratch_selector != 0 || flags != 0 || len as usize > TALOS_COPY_PROBE_MAX_LEN {
        return SyscallReturn::error(PosixError::InvalidArgument);
    }

    let len = len as usize;
    let mut scratch = [0u8; TALOS_COPY_PROBE_MAX_LEN];
    let copied = match crate::posix::copy_from_user(
        mappings,
        user_memory_start,
        user_memory,
        user_start,
        len,
        &mut scratch,
    ) {
        Ok(copied) => copied,
        Err(error) => return SyscallReturn::error(error),
    };

    let expected = expected as u8;
    if scratch[..copied].iter().any(|byte| *byte != expected) {
        return SyscallReturn::error(PosixError::InvalidArgument);
    }

    scratch[..copied].fill(replacement as u8);
    match crate::posix::copy_to_user(
        mappings,
        user_memory_start,
        user_memory,
        user_start,
        copied,
        &scratch,
    ) {
        Ok(written) => SyscallReturn::success(written as u64),
        Err(error) => SyscallReturn::error(error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn stable_svc_immediate_is_zero_and_diagnostic_marker_is_not_stable() {
        assert!(is_stable_syscall_svc_immediate(STABLE_SVC_IMMEDIATE));
        assert_eq!(STABLE_SVC_IMMEDIATE, 0);
        assert_eq!(DIAGNOSTIC_EL0_TRAP_SVC_IMMEDIATE, 0x7a10);
        assert!(!is_stable_syscall_svc_immediate(
            DIAGNOSTIC_EL0_TRAP_SVC_IMMEDIATE
        ));
    }

    #[test_case]
    fn syscall_number_zero_dispatches_to_talos_nop_success() {
        let result = dispatch(TALOS_NOP_SYSCALL, SyscallArguments::empty());

        assert_eq!(result.number(), SyscallNumber::TalosNop);
        assert_eq!(result.number().raw(), TALOS_NOP_SYSCALL);
        assert_eq!(result.return_value().x0(), 0);
    }

    #[test_case]
    fn unknown_syscall_dispatches_to_negative_enosys() {
        let result = dispatch(17, SyscallArguments::empty());

        assert_eq!(result.number(), SyscallNumber::Unknown(17));
        assert_eq!(result.number().raw(), 17);
        assert_eq!(result.return_value().x0(), (ENOSYS as u64).wrapping_neg());
    }

    #[test_case]
    fn copy_probe_number_is_unknown_outside_proof_dispatch() {
        let result = dispatch(TALOS_COPY_PROBE_SYSCALL, SyscallArguments::empty());

        assert_eq!(
            result.number(),
            SyscallNumber::Unknown(TALOS_COPY_PROBE_SYSCALL)
        );
        assert_eq!(result.return_value().x0(), (ENOSYS as u64).wrapping_neg());
    }

    #[test_case]
    fn copy_probe_dispatch_uses_explicit_mapping_and_backing_storage() {
        let mapping = crate::posix::UserMapping::new(
            0x0000_0000_0011_0000,
            0x1000,
            crate::posix::UserMappingPermissions::USER_DATA,
        )
        .expect("fixed user data mapping is valid");
        let mut user_memory = [0x2au8; 0x1000];
        let arguments = SyscallArguments::new([0x0000_0000_0011_0000, 16, 0x2a, 0xa5, 0, 0]);

        let result = dispatch_copy_probe(
            arguments,
            &[mapping],
            0x0000_0000_0011_0000,
            &mut user_memory,
        );

        assert_eq!(result.x0(), 16);
        assert_eq!(&user_memory[..16], &[0xa5; 16]);
    }

    #[test_case]
    fn copy_probe_dispatch_reports_fault_for_unmapped_user_range() {
        let mapping = crate::posix::UserMapping::new(
            0x0000_0000_0011_0000,
            0x1000,
            crate::posix::UserMappingPermissions::USER_DATA,
        )
        .expect("fixed user data mapping is valid");
        let mut user_memory = [0x2au8; 0x1000];
        let arguments = SyscallArguments::new([0x0000_0000_001e_0000, 16, 0x2a, 0xa5, 0, 0]);

        let result = dispatch_copy_probe(
            arguments,
            &[mapping],
            0x0000_0000_0011_0000,
            &mut user_memory,
        );

        assert_eq!(result.x0(), (EFAULT as u64).wrapping_neg());
    }

    #[test_case]
    fn scalar_argument_view_preserves_x0_through_x5_values() {
        let arguments = SyscallArguments::new([10, 11, 12, 13, 14, 15]);
        let result = dispatch(TALOS_NOP_SYSCALL, arguments);

        assert_eq!(result.arguments().values(), [10, 11, 12, 13, 14, 15]);
        assert_eq!(result.arguments().get(0), Some(10));
        assert_eq!(result.arguments().get(5), Some(15));
        assert_eq!(result.arguments().get(6), None);
    }

    #[test_case]
    fn accepted_errno_subset_encodes_as_negative_x0_values() {
        let accepted = [
            (PosixError::InvalidArgument, EINVAL),
            (PosixError::BadDescriptor, EBADF),
            (PosixError::Fault, EFAULT),
            (PosixError::NotImplemented, ENOSYS),
            (PosixError::NotSupported, ENOTSUP),
        ];

        let mut index = 0;
        while index < accepted.len() {
            let (error, errno) = accepted[index];
            assert_eq!(errno_number(error), Some(errno));
            assert_eq!(
                SyscallReturn::error(error).x0(),
                (errno as u64).wrapping_neg()
            );
            index += 1;
        }
    }

    #[test_case]
    fn unaccepted_posix_errors_fall_back_to_enosys_encoding() {
        assert_eq!(errno_number(PosixError::NoEntry), None);
        assert_eq!(
            SyscallReturn::error(PosixError::NoEntry).x0(),
            (ENOSYS as u64).wrapping_neg()
        );
    }
}
