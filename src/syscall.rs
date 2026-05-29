//! Target-independent syscall ABI dispatch primitives.
//!
//! This module owns only the first stable syscall vocabulary and return
//! encoding. It does not route exception vectors, enter EL0, load programs, or
//! provide VFS/filesystem behavior.

use crate::posix::PosixError;

pub(crate) const STABLE_SVC_IMMEDIATE: u16 = 0;
pub(crate) const DIAGNOSTIC_EL0_TRAP_SVC_IMMEDIATE: u16 = 0x7a10;
pub(crate) const TALOS_NOP_SYSCALL: u64 = 0;
pub(crate) const TALOS_WRITE_SYSCALL: u64 = 1;
pub(crate) const TALOS_CLOSE_SYSCALL: u64 = 2;
#[cfg(any(
    test,
    talos_boot_scenario = "qemu_pointer_copy_smoke",
    talos_boot_scenario = "rpi5_pointer_copy_proof"
))]
pub(crate) const TALOS_COPY_PROBE_SYSCALL: u64 = 0x7001;
#[cfg(any(
    test,
    talos_boot_scenario = "qemu_pointer_copy_smoke",
    talos_boot_scenario = "rpi5_pointer_copy_proof"
))]
pub(crate) const TALOS_COPY_PROBE_MAX_LEN: usize = 32;
pub(crate) const MAX_SCALAR_ARGUMENTS: usize = 6;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SyscallNumber {
    TalosNop,
    TalosWrite,
    TalosClose,
    Unknown(u64),
}

impl SyscallNumber {
    pub(crate) const fn from_raw(raw: u64) -> Self {
        match raw {
            TALOS_NOP_SYSCALL => Self::TalosNop,
            TALOS_WRITE_SYSCALL => Self::TalosWrite,
            TALOS_CLOSE_SYSCALL => Self::TalosClose,
            unknown => Self::Unknown(unknown),
        }
    }

    pub(crate) const fn raw(self) -> u64 {
        match self {
            Self::TalosNop => TALOS_NOP_SYSCALL,
            Self::TalosWrite => TALOS_WRITE_SYSCALL,
            Self::TalosClose => TALOS_CLOSE_SYSCALL,
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
pub(crate) const EIO: u16 = 5;
pub(crate) const EFAULT: u16 = 14;
pub(crate) const ENOSYS: u16 = 38;
pub(crate) const ENOTSUP: u16 = 95;

pub(crate) const fn errno_number(error: PosixError) -> Option<u16> {
    match error {
        PosixError::InvalidArgument => Some(EINVAL),
        PosixError::BadDescriptor => Some(EBADF),
        PosixError::Io => Some(EIO),
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
        SyscallNumber::TalosWrite | SyscallNumber::TalosClose => {
            SyscallReturn::error(PosixError::NotSupported)
        }
        SyscallNumber::Unknown(_) => SyscallReturn::error(PosixError::NotImplemented),
    };

    SyscallDispatchResult {
        number,
        arguments,
        return_value,
    }
}

pub(crate) fn dispatch_descriptor_write<const CAPACITY: usize, B>(
    raw_number: u64,
    arguments: SyscallArguments,
    descriptor_table: &crate::posix::DescriptorTable<CAPACITY>,
    mappings: &[crate::posix::UserMapping],
    user_memory_start: u64,
    user_memory: &[u8],
    kernel_scratch: &mut [u8],
    console_backend: &mut B,
) -> SyscallDispatchResult
where
    B: crate::runtime_console::ConsoleBackend,
{
    let number = SyscallNumber::from_raw(raw_number);
    let return_value = match number {
        SyscallNumber::TalosWrite => dispatch_talos_write(
            arguments,
            descriptor_table,
            mappings,
            user_memory_start,
            user_memory,
            kernel_scratch,
            console_backend,
        ),
        SyscallNumber::TalosNop | SyscallNumber::TalosClose | SyscallNumber::Unknown(_) => {
            dispatch(raw_number, arguments).return_value()
        }
    };

    SyscallDispatchResult {
        number,
        arguments,
        return_value,
    }
}

pub(crate) fn dispatch_process_descriptor<
    const OWNER_CAPACITY: usize,
    const DESCRIPTOR_CAPACITY: usize,
    B,
>(
    raw_number: u64,
    arguments: SyscallArguments,
    current_owner: Option<crate::scheduler::ProcessOwnerId>,
    descriptor_store: &mut crate::posix::ProcessDescriptorStore<
        OWNER_CAPACITY,
        DESCRIPTOR_CAPACITY,
    >,
    mappings: &[crate::posix::UserMapping],
    user_memory_start: u64,
    user_memory: &[u8],
    kernel_scratch: &mut [u8],
    console_backend: &mut B,
) -> SyscallDispatchResult
where
    B: crate::runtime_console::ConsoleBackend,
{
    let number = SyscallNumber::from_raw(raw_number);
    let return_value = match number {
        SyscallNumber::TalosWrite => match descriptor_store.current_descriptor_table(current_owner)
        {
            Ok(descriptor_table) => dispatch_talos_write(
                arguments,
                descriptor_table,
                mappings,
                user_memory_start,
                user_memory,
                kernel_scratch,
                console_backend,
            ),
            Err(error) => SyscallReturn::error(error),
        },
        SyscallNumber::TalosClose => {
            dispatch_talos_close(arguments, current_owner, descriptor_store)
        }
        SyscallNumber::TalosNop | SyscallNumber::Unknown(_) => {
            dispatch(raw_number, arguments).return_value()
        }
    };

    SyscallDispatchResult {
        number,
        arguments,
        return_value,
    }
}

fn dispatch_talos_write<const CAPACITY: usize, B>(
    arguments: SyscallArguments,
    descriptor_table: &crate::posix::DescriptorTable<CAPACITY>,
    mappings: &[crate::posix::UserMapping],
    user_memory_start: u64,
    user_memory: &[u8],
    kernel_scratch: &mut [u8],
    console_backend: &mut B,
) -> SyscallReturn
where
    B: crate::runtime_console::ConsoleBackend,
{
    let [descriptor, user_start, len, reserved0, reserved1, reserved2] = arguments.values();
    if reserved0 != 0
        || reserved1 != 0
        || reserved2 != 0
        || len > crate::posix::DEFAULT_USER_COPY_LIMIT as u64
    {
        return SyscallReturn::error(PosixError::InvalidArgument);
    }

    let Ok(descriptor) = usize::try_from(descriptor) else {
        return SyscallReturn::error(PosixError::BadDescriptor);
    };
    let Ok(len) = usize::try_from(len) else {
        return SyscallReturn::error(PosixError::InvalidArgument);
    };

    match crate::posix::write_descriptor_to_runtime_console(
        descriptor_table,
        descriptor,
        mappings,
        user_memory_start,
        user_memory,
        user_start,
        len,
        kernel_scratch,
        console_backend,
    ) {
        Ok(bytes_written) => SyscallReturn::success(bytes_written as u64),
        Err(error) => SyscallReturn::error(error),
    }
}

fn dispatch_talos_close<const OWNER_CAPACITY: usize, const DESCRIPTOR_CAPACITY: usize>(
    arguments: SyscallArguments,
    current_owner: Option<crate::scheduler::ProcessOwnerId>,
    descriptor_store: &mut crate::posix::ProcessDescriptorStore<
        OWNER_CAPACITY,
        DESCRIPTOR_CAPACITY,
    >,
) -> SyscallReturn {
    let [
        descriptor,
        reserved0,
        reserved1,
        reserved2,
        reserved3,
        reserved4,
    ] = arguments.values();
    if reserved0 != 0 || reserved1 != 0 || reserved2 != 0 || reserved3 != 0 || reserved4 != 0 {
        return SyscallReturn::error(PosixError::InvalidArgument);
    }

    let Ok(descriptor) = usize::try_from(descriptor) else {
        return SyscallReturn::error(PosixError::BadDescriptor);
    };

    match descriptor_store.close_current_descriptor(current_owner, descriptor) {
        Ok(_) => SyscallReturn::success(0),
        Err(error) => SyscallReturn::error(error),
    }
}

#[cfg(any(
    test,
    talos_boot_scenario = "qemu_pointer_copy_smoke",
    talos_boot_scenario = "rpi5_pointer_copy_proof"
))]
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
    fn descriptor_write_number_requires_context_in_scalar_dispatch() {
        let result = dispatch(TALOS_WRITE_SYSCALL, SyscallArguments::empty());

        assert_eq!(result.number(), SyscallNumber::TalosWrite);
        assert_eq!(result.number().raw(), TALOS_WRITE_SYSCALL);
        assert_eq!(result.return_value().x0(), (ENOTSUP as u64).wrapping_neg());
    }

    #[test_case]
    fn descriptor_close_number_requires_context_in_scalar_dispatch() {
        let result = dispatch(TALOS_CLOSE_SYSCALL, SyscallArguments::empty());

        assert_eq!(result.number(), SyscallNumber::TalosClose);
        assert_eq!(result.number().raw(), TALOS_CLOSE_SYSCALL);
        assert_eq!(result.return_value().x0(), (ENOTSUP as u64).wrapping_neg());
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
            (PosixError::Io, EIO),
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

    struct CaptureConsole {
        bytes: [u8; 64],
        len: usize,
        fail_writes: bool,
    }

    impl CaptureConsole {
        const fn new() -> Self {
            Self {
                bytes: [0; 64],
                len: 0,
                fail_writes: false,
            }
        }

        const fn failing() -> Self {
            Self {
                bytes: [0; 64],
                len: 0,
                fail_writes: true,
            }
        }

        fn as_bytes(&self) -> &[u8] {
            &self.bytes[..self.len]
        }
    }

    impl crate::runtime_console::ConsoleBackend for CaptureConsole {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            self.write_bytes(s.as_bytes())
        }

        fn write_bytes(&mut self, bytes: &[u8]) -> core::fmt::Result {
            if self.fail_writes {
                return Err(core::fmt::Error);
            }
            let Some(end) = self.len.checked_add(bytes.len()) else {
                return Err(core::fmt::Error);
            };
            if end > self.bytes.len() {
                return Err(core::fmt::Error);
            }
            self.bytes[self.len..end].copy_from_slice(bytes);
            self.len = end;
            Ok(())
        }
    }

    fn descriptor_write_fixture() -> (
        crate::posix::DescriptorTable<4>,
        [crate::posix::UserMapping; 1],
        [u8; 128],
    ) {
        let table =
            crate::posix::DescriptorTable::<4>::with_inherited_stdio().expect("stdio table");
        let mappings = [crate::posix::UserMapping::new(
            0x0000_0000_0011_0000,
            0x80,
            crate::posix::UserMappingPermissions::USER_DATA,
        )
        .expect("user data mapping")];
        let mut user_memory = [0u8; 128];
        user_memory[..18].copy_from_slice(b"talos-stdout-qemu\n");
        user_memory[0x40..0x52].copy_from_slice(b"talos-stderr-qemu\n");
        (table, mappings, user_memory)
    }

    fn process_descriptor_fixture() -> (
        crate::scheduler::ProcessOwnerId,
        crate::posix::ProcessDescriptorStore<2, 4>,
        [crate::posix::UserMapping; 1],
        [u8; 128],
    ) {
        let owner = crate::scheduler::ProcessOwnerId::new(31).expect("owner id");
        let mut store = crate::posix::ProcessDescriptorStore::<2, 4>::new_empty();
        store
            .create_owner_with_inherited_stdio(owner)
            .expect("create owner");
        let mappings = [crate::posix::UserMapping::new(
            0x0000_0000_0011_0000,
            0x80,
            crate::posix::UserMappingPermissions::USER_DATA,
        )
        .expect("user data mapping")];
        let mut user_memory = [0u8; 128];
        user_memory[..18].copy_from_slice(b"talos-stdout-qemu\n");
        user_memory[0x40..0x52].copy_from_slice(b"talos-stderr-qemu\n");
        (owner, store, mappings, user_memory)
    }

    fn dispatch_write_case(
        descriptor: u64,
        user_start: u64,
        len: u64,
        reserved0: u64,
        console: &mut CaptureConsole,
    ) -> SyscallDispatchResult {
        let (table, mappings, user_memory) = descriptor_write_fixture();
        let mut scratch = [0u8; 64];
        dispatch_descriptor_write(
            TALOS_WRITE_SYSCALL,
            SyscallArguments::new([descriptor, user_start, len, reserved0, 0, 0]),
            &table,
            &mappings,
            0x0000_0000_0011_0000,
            &user_memory,
            &mut scratch,
            console,
        )
    }

    #[test_case]
    fn talos_write_stdout_copies_user_bytes_before_runtime_console_output() {
        let mut console = CaptureConsole::new();

        let result = dispatch_write_case(1, 0x0000_0000_0011_0000, 18, 0, &mut console);

        assert_eq!(result.number(), SyscallNumber::TalosWrite);
        assert_eq!(result.return_value().x0(), 18);
        assert_eq!(console.as_bytes(), b"talos-stdout-qemu\n");
    }

    #[test_case]
    fn talos_write_stderr_uses_the_same_runtime_console_slice() {
        let mut console = CaptureConsole::new();

        let result = dispatch_write_case(2, 0x0000_0000_0011_0040, 18, 0, &mut console);

        assert_eq!(result.return_value().x0(), 18);
        assert_eq!(console.as_bytes(), b"talos-stderr-qemu\n");
    }

    #[test_case]
    fn talos_write_zero_length_validates_descriptor_without_console_output() {
        let mut console = CaptureConsole::new();

        let result = dispatch_write_case(1, 0, 0, 0, &mut console);

        assert_eq!(result.return_value().x0(), 0);
        assert_eq!(console.as_bytes(), b"");
    }

    #[test_case]
    fn talos_write_rejects_read_only_and_invalid_descriptors_without_output() {
        let mut console = CaptureConsole::new();
        let fd0 = dispatch_write_case(0, 0x0000_0000_0011_0000, 18, 0, &mut console);
        let bad_fd = dispatch_write_case(99, 0x0000_0000_0011_0000, 18, 0, &mut console);

        assert_eq!(fd0.return_value().x0(), (EBADF as u64).wrapping_neg());
        assert_eq!(bad_fd.return_value().x0(), (EBADF as u64).wrapping_neg());
        assert_eq!(console.as_bytes(), b"");
    }

    #[test_case]
    fn talos_write_rejects_unmapped_user_ranges_without_output() {
        let mut console = CaptureConsole::new();

        let result = dispatch_write_case(1, 0x0000_0000_001e_0000, 18, 0, &mut console);

        assert_eq!(result.return_value().x0(), (EFAULT as u64).wrapping_neg());
        assert_eq!(console.as_bytes(), b"");
    }

    #[test_case]
    fn talos_write_rejects_reserved_registers_and_oversize_lengths_as_einval() {
        let mut console = CaptureConsole::new();
        let reserved = dispatch_write_case(1, 0x0000_0000_0011_0000, 18, 1, &mut console);
        let oversize = dispatch_write_case(
            1,
            0x0000_0000_0011_0000,
            crate::posix::DEFAULT_USER_COPY_LIMIT as u64 + 1,
            0,
            &mut console,
        );

        assert_eq!(reserved.return_value().x0(), (EINVAL as u64).wrapping_neg());
        assert_eq!(oversize.return_value().x0(), (EINVAL as u64).wrapping_neg());
        assert_eq!(console.as_bytes(), b"");
    }

    #[test_case]
    fn talos_write_backend_failure_is_encoded_as_eio_not_success() {
        let mut console = CaptureConsole::failing();

        let result = dispatch_write_case(1, 0x0000_0000_0011_0000, 18, 0, &mut console);

        assert_eq!(result.return_value().x0(), (EIO as u64).wrapping_neg());
        assert_eq!(console.as_bytes(), b"");
    }

    #[test_case]
    fn talos_close_stdout_blocks_later_process_descriptor_write() {
        let (owner, mut store, mappings, user_memory) = process_descriptor_fixture();
        let mut scratch = [0u8; 64];
        let mut console = CaptureConsole::new();

        let close = dispatch_process_descriptor(
            TALOS_CLOSE_SYSCALL,
            SyscallArguments::new([1, 0, 0, 0, 0, 0]),
            Some(owner),
            &mut store,
            &mappings,
            0x0000_0000_0011_0000,
            &user_memory,
            &mut scratch,
            &mut console,
        );
        let write = dispatch_process_descriptor(
            TALOS_WRITE_SYSCALL,
            SyscallArguments::new([1, 0x0000_0000_0011_0000, 18, 0, 0, 0]),
            Some(owner),
            &mut store,
            &mappings,
            0x0000_0000_0011_0000,
            &user_memory,
            &mut scratch,
            &mut console,
        );

        assert_eq!(close.number(), SyscallNumber::TalosClose);
        assert_eq!(close.return_value().x0(), 0);
        assert_eq!(write.number(), SyscallNumber::TalosWrite);
        assert_eq!(write.return_value().x0(), (EBADF as u64).wrapping_neg());
        assert_eq!(console.as_bytes(), b"");
    }

    #[test_case]
    fn talos_close_stderr_uses_the_same_table_local_rule() {
        let (owner, mut store, mappings, user_memory) = process_descriptor_fixture();
        let mut scratch = [0u8; 64];
        let mut console = CaptureConsole::new();

        let close = dispatch_process_descriptor(
            TALOS_CLOSE_SYSCALL,
            SyscallArguments::new([2, 0, 0, 0, 0, 0]),
            Some(owner),
            &mut store,
            &mappings,
            0x0000_0000_0011_0000,
            &user_memory,
            &mut scratch,
            &mut console,
        );

        assert_eq!(close.return_value().x0(), 0);
        assert_eq!(
            store
                .current_descriptor_table(Some(owner))
                .expect("current table")
                .get(crate::posix::STDERR_FD),
            Err(PosixError::BadDescriptor)
        );
        assert_eq!(console.as_bytes(), b"");
    }

    #[test_case]
    fn talos_close_failures_are_deterministic_and_do_not_mutate_on_einval() {
        let (owner, mut store, mappings, user_memory) = process_descriptor_fixture();
        let missing = crate::scheduler::ProcessOwnerId::new(32).expect("missing owner id");
        let mut scratch = [0u8; 64];
        let mut console = CaptureConsole::new();

        let missing_owner = dispatch_process_descriptor(
            TALOS_CLOSE_SYSCALL,
            SyscallArguments::new([1, 0, 0, 0, 0, 0]),
            Some(missing),
            &mut store,
            &mappings,
            0x0000_0000_0011_0000,
            &user_memory,
            &mut scratch,
            &mut console,
        );
        let no_owner = dispatch_process_descriptor(
            TALOS_CLOSE_SYSCALL,
            SyscallArguments::new([1, 0, 0, 0, 0, 0]),
            None,
            &mut store,
            &mappings,
            0x0000_0000_0011_0000,
            &user_memory,
            &mut scratch,
            &mut console,
        );
        let invalid_descriptor = dispatch_process_descriptor(
            TALOS_CLOSE_SYSCALL,
            SyscallArguments::new([99, 0, 0, 0, 0, 0]),
            Some(owner),
            &mut store,
            &mappings,
            0x0000_0000_0011_0000,
            &user_memory,
            &mut scratch,
            &mut console,
        );
        let reserved = dispatch_process_descriptor(
            TALOS_CLOSE_SYSCALL,
            SyscallArguments::new([1, 1, 0, 0, 0, 0]),
            Some(owner),
            &mut store,
            &mappings,
            0x0000_0000_0011_0000,
            &user_memory,
            &mut scratch,
            &mut console,
        );
        let close = dispatch_process_descriptor(
            TALOS_CLOSE_SYSCALL,
            SyscallArguments::new([1, 0, 0, 0, 0, 0]),
            Some(owner),
            &mut store,
            &mappings,
            0x0000_0000_0011_0000,
            &user_memory,
            &mut scratch,
            &mut console,
        );
        let double_close = dispatch_process_descriptor(
            TALOS_CLOSE_SYSCALL,
            SyscallArguments::new([1, 0, 0, 0, 0, 0]),
            Some(owner),
            &mut store,
            &mappings,
            0x0000_0000_0011_0000,
            &user_memory,
            &mut scratch,
            &mut console,
        );

        assert_eq!(
            missing_owner.return_value().x0(),
            (EBADF as u64).wrapping_neg()
        );
        assert_eq!(no_owner.return_value().x0(), (EBADF as u64).wrapping_neg());
        assert_eq!(
            invalid_descriptor.return_value().x0(),
            (EBADF as u64).wrapping_neg()
        );
        assert_eq!(reserved.return_value().x0(), (EINVAL as u64).wrapping_neg());
        assert_eq!(close.return_value().x0(), 0);
        assert_eq!(
            double_close.return_value().x0(),
            (EBADF as u64).wrapping_neg()
        );
        assert_eq!(console.as_bytes(), b"");
    }

    #[test_case]
    fn talos_close_preserves_duplicate_descriptor_lifetime() {
        let (owner, mut store, mappings, user_memory) = process_descriptor_fixture();
        let duplicate = store
            .current_descriptor_table_mut(Some(owner))
            .expect("current table")
            .dup(crate::posix::STDOUT_FD)
            .expect("dup stdout");
        let mut scratch = [0u8; 64];
        let mut console = CaptureConsole::new();

        let close_original = dispatch_process_descriptor(
            TALOS_CLOSE_SYSCALL,
            SyscallArguments::new([crate::posix::STDOUT_FD as u64, 0, 0, 0, 0, 0]),
            Some(owner),
            &mut store,
            &mappings,
            0x0000_0000_0011_0000,
            &user_memory,
            &mut scratch,
            &mut console,
        );
        let write_duplicate = dispatch_process_descriptor(
            TALOS_WRITE_SYSCALL,
            SyscallArguments::new([duplicate as u64, 0x0000_0000_0011_0000, 18, 0, 0, 0]),
            Some(owner),
            &mut store,
            &mappings,
            0x0000_0000_0011_0000,
            &user_memory,
            &mut scratch,
            &mut console,
        );

        assert_eq!(close_original.return_value().x0(), 0);
        assert_eq!(
            write_duplicate.return_value().x0(),
            (EBADF as u64).wrapping_neg()
        );
        assert_eq!(console.as_bytes(), b"");
        assert_eq!(
            store
                .current_descriptor_table(Some(owner))
                .expect("current table")
                .get(duplicate)
                .expect("duplicate remains")
                .object()
                .reference(),
            crate::posix::STDOUT_FD
        );
    }

    #[test_case]
    fn descriptor_write_dispatch_preserves_scalar_syscall_regressions() {
        let mut console = CaptureConsole::new();
        let (table, mappings, user_memory) = descriptor_write_fixture();
        let mut scratch = [0u8; 64];

        let nop = dispatch_descriptor_write(
            TALOS_NOP_SYSCALL,
            SyscallArguments::empty(),
            &table,
            &mappings,
            0x0000_0000_0011_0000,
            &user_memory,
            &mut scratch,
            &mut console,
        );
        let unknown = dispatch_descriptor_write(
            17,
            SyscallArguments::empty(),
            &table,
            &mappings,
            0x0000_0000_0011_0000,
            &user_memory,
            &mut scratch,
            &mut console,
        );
        let copy_probe = dispatch_descriptor_write(
            TALOS_COPY_PROBE_SYSCALL,
            SyscallArguments::new([0x0000_0000_0011_0000, 16, 0x2a, 0xa5, 0, 0]),
            &table,
            &mappings,
            0x0000_0000_0011_0000,
            &user_memory,
            &mut scratch,
            &mut console,
        );

        assert_eq!(nop.number(), SyscallNumber::TalosNop);
        assert_eq!(nop.return_value().x0(), 0);
        assert_eq!(unknown.number(), SyscallNumber::Unknown(17));
        assert_eq!(unknown.return_value().x0(), (ENOSYS as u64).wrapping_neg());
        assert_eq!(
            copy_probe.number(),
            SyscallNumber::Unknown(TALOS_COPY_PROBE_SYSCALL)
        );
        assert_eq!(
            copy_probe.return_value().x0(),
            (ENOSYS as u64).wrapping_neg()
        );
        assert_eq!(console.as_bytes(), b"");
    }
}
