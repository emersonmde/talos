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
pub(crate) const TALOS_DUP_SYSCALL: u64 = 3;
pub(crate) const TALOS_READ_SYSCALL: u64 = 4;
pub(crate) const TALOS_OPEN_SYSCALL: u64 = 5;
pub(crate) const TALOS_SOCKET_SYSCALL: u64 = 6;
pub(crate) const TALOS_BIND_SYSCALL: u64 = 7;
pub(crate) const TALOS_LISTEN_SYSCALL: u64 = 8;
pub(crate) const TALOS_CONNECT_SYSCALL: u64 = 9;
pub(crate) const TALOS_ACCEPT_SYSCALL: u64 = 10;
pub(crate) const TALOS_SEND_SYSCALL: u64 = 11;
pub(crate) const TALOS_RECV_SYSCALL: u64 = 12;
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
    TalosDup,
    TalosRead,
    TalosOpen,
    TalosSocket,
    TalosBind,
    TalosListen,
    TalosConnect,
    TalosAccept,
    TalosSend,
    TalosRecv,
    Unknown(u64),
}

impl SyscallNumber {
    pub(crate) const fn from_raw(raw: u64) -> Self {
        match raw {
            TALOS_NOP_SYSCALL => Self::TalosNop,
            TALOS_WRITE_SYSCALL => Self::TalosWrite,
            TALOS_CLOSE_SYSCALL => Self::TalosClose,
            TALOS_DUP_SYSCALL => Self::TalosDup,
            TALOS_READ_SYSCALL => Self::TalosRead,
            TALOS_OPEN_SYSCALL => Self::TalosOpen,
            TALOS_SOCKET_SYSCALL => Self::TalosSocket,
            TALOS_BIND_SYSCALL => Self::TalosBind,
            TALOS_LISTEN_SYSCALL => Self::TalosListen,
            TALOS_CONNECT_SYSCALL => Self::TalosConnect,
            TALOS_ACCEPT_SYSCALL => Self::TalosAccept,
            TALOS_SEND_SYSCALL => Self::TalosSend,
            TALOS_RECV_SYSCALL => Self::TalosRecv,
            unknown => Self::Unknown(unknown),
        }
    }

    pub(crate) const fn raw(self) -> u64 {
        match self {
            Self::TalosNop => TALOS_NOP_SYSCALL,
            Self::TalosWrite => TALOS_WRITE_SYSCALL,
            Self::TalosClose => TALOS_CLOSE_SYSCALL,
            Self::TalosDup => TALOS_DUP_SYSCALL,
            Self::TalosRead => TALOS_READ_SYSCALL,
            Self::TalosOpen => TALOS_OPEN_SYSCALL,
            Self::TalosSocket => TALOS_SOCKET_SYSCALL,
            Self::TalosBind => TALOS_BIND_SYSCALL,
            Self::TalosListen => TALOS_LISTEN_SYSCALL,
            Self::TalosConnect => TALOS_CONNECT_SYSCALL,
            Self::TalosAccept => TALOS_ACCEPT_SYSCALL,
            Self::TalosSend => TALOS_SEND_SYSCALL,
            Self::TalosRecv => TALOS_RECV_SYSCALL,
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
        let errno = errno_number(error);
        Self {
            x0: (errno as u64).wrapping_neg(),
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

pub(crate) const EPERM: u16 = 1;
pub(crate) const ENOENT: u16 = 2;
pub(crate) const EINTR: u16 = 4;
pub(crate) const EIO: u16 = 5;
pub(crate) const ENOEXEC: u16 = 8;
pub(crate) const EBADF: u16 = 9;
pub(crate) const ECHILD: u16 = 10;
pub(crate) const EAGAIN: u16 = 11;
pub(crate) const ENOMEM: u16 = 12;
pub(crate) const EACCES: u16 = 13;
pub(crate) const EFAULT: u16 = 14;
pub(crate) const EBUSY: u16 = 16;
pub(crate) const EEXIST: u16 = 17;
pub(crate) const ENODEV: u16 = 19;
pub(crate) const ENOTDIR: u16 = 20;
pub(crate) const EISDIR: u16 = 21;
pub(crate) const EINVAL: u16 = 22;
pub(crate) const EMFILE: u16 = 24;
pub(crate) const ENOTTY: u16 = 25;
pub(crate) const ENOSPC: u16 = 28;
pub(crate) const EPIPE: u16 = 32;
pub(crate) const ERANGE: u16 = 34;
pub(crate) const ENAMETOOLONG: u16 = 36;
pub(crate) const ENOSYS: u16 = 38;
pub(crate) const ENOTEMPTY: u16 = 39;
pub(crate) const ENOTSUP: u16 = 95;

pub(crate) const fn errno_number(error: PosixError) -> u16 {
    match error {
        PosixError::OperationNotPermitted => EPERM,
        PosixError::NoEntry => ENOENT,
        PosixError::Interrupted => EINTR,
        PosixError::Io => EIO,
        PosixError::NotExecutable => ENOEXEC,
        PosixError::BadDescriptor => EBADF,
        PosixError::NoChild => ECHILD,
        PosixError::Again => EAGAIN,
        PosixError::NoMemory => ENOMEM,
        PosixError::AccessDenied => EACCES,
        PosixError::Fault => EFAULT,
        PosixError::Busy => EBUSY,
        PosixError::Exists => EEXIST,
        PosixError::NoDevice => ENODEV,
        PosixError::NotDirectory => ENOTDIR,
        PosixError::IsDirectory => EISDIR,
        PosixError::InvalidArgument => EINVAL,
        PosixError::TooManyOpenFiles => EMFILE,
        PosixError::NotTty => ENOTTY,
        PosixError::NoSpace => ENOSPC,
        PosixError::Pipe => EPIPE,
        PosixError::Range => ERANGE,
        PosixError::NameTooLong => ENAMETOOLONG,
        PosixError::NotImplemented => ENOSYS,
        PosixError::NotEmpty => ENOTEMPTY,
        PosixError::NotSupported => ENOTSUP,
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
        SyscallNumber::TalosWrite
        | SyscallNumber::TalosClose
        | SyscallNumber::TalosDup
        | SyscallNumber::TalosRead
        | SyscallNumber::TalosOpen
        | SyscallNumber::TalosSocket
        | SyscallNumber::TalosBind
        | SyscallNumber::TalosListen
        | SyscallNumber::TalosConnect
        | SyscallNumber::TalosAccept
        | SyscallNumber::TalosSend
        | SyscallNumber::TalosRecv => SyscallReturn::error(PosixError::NotSupported),
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
        SyscallNumber::TalosNop
        | SyscallNumber::TalosClose
        | SyscallNumber::TalosDup
        | SyscallNumber::TalosRead
        | SyscallNumber::TalosOpen
        | SyscallNumber::TalosSocket
        | SyscallNumber::TalosBind
        | SyscallNumber::TalosListen
        | SyscallNumber::TalosConnect
        | SyscallNumber::TalosAccept
        | SyscallNumber::TalosSend
        | SyscallNumber::TalosRecv
        | SyscallNumber::Unknown(_) => dispatch(raw_number, arguments).return_value(),
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
        SyscallNumber::TalosDup => dispatch_talos_dup(arguments, current_owner, descriptor_store),
        SyscallNumber::TalosRead
        | SyscallNumber::TalosOpen
        | SyscallNumber::TalosSocket
        | SyscallNumber::TalosBind
        | SyscallNumber::TalosListen
        | SyscallNumber::TalosConnect
        | SyscallNumber::TalosAccept
        | SyscallNumber::TalosSend
        | SyscallNumber::TalosRecv
        | SyscallNumber::TalosNop
        | SyscallNumber::Unknown(_) => dispatch(raw_number, arguments).return_value(),
    };

    SyscallDispatchResult {
        number,
        arguments,
        return_value,
    }
}

pub(crate) fn dispatch_process_descriptor_with_fixed_stdin<
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
    user_memory: &mut [u8],
    kernel_scratch: &mut [u8],
    console_backend: &mut B,
    fixed_stdin: Option<&mut crate::posix::FixedStdin<'_>>,
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
        SyscallNumber::TalosDup => dispatch_talos_dup(arguments, current_owner, descriptor_store),
        SyscallNumber::TalosRead => dispatch_talos_read(
            arguments,
            current_owner,
            descriptor_store,
            mappings,
            user_memory_start,
            user_memory,
            kernel_scratch,
            fixed_stdin,
            None::<&mut NoConsoleInput>,
            None::<crate::initramfs::ReadOnlyInitramfs>,
            None::<&mut crate::initramfs::ReadOnlyFileDescriptions<0>>,
        ),
        SyscallNumber::TalosOpen
        | SyscallNumber::TalosSocket
        | SyscallNumber::TalosBind
        | SyscallNumber::TalosListen
        | SyscallNumber::TalosConnect
        | SyscallNumber::TalosAccept
        | SyscallNumber::TalosSend
        | SyscallNumber::TalosRecv
        | SyscallNumber::TalosNop
        | SyscallNumber::Unknown(_) => dispatch(raw_number, arguments).return_value(),
    };

    SyscallDispatchResult {
        number,
        arguments,
        return_value,
    }
}

pub(crate) fn dispatch_process_descriptor_with_initramfs<
    const OWNER_CAPACITY: usize,
    const DESCRIPTOR_CAPACITY: usize,
    const FILE_CAPACITY: usize,
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
    user_memory: &mut [u8],
    kernel_scratch: &mut [u8],
    console_backend: &mut B,
    initramfs: crate::initramfs::ReadOnlyInitramfs,
    file_descriptions: &mut crate::initramfs::ReadOnlyFileDescriptions<FILE_CAPACITY>,
    fixed_stdin: Option<&mut crate::posix::FixedStdin<'_>>,
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
        SyscallNumber::TalosDup => dispatch_talos_dup(arguments, current_owner, descriptor_store),
        SyscallNumber::TalosRead => dispatch_talos_read(
            arguments,
            current_owner,
            descriptor_store,
            mappings,
            user_memory_start,
            user_memory,
            kernel_scratch,
            fixed_stdin,
            None::<&mut NoConsoleInput>,
            Some(initramfs),
            Some(file_descriptions),
        ),
        SyscallNumber::TalosOpen => dispatch_talos_open_initramfs(
            arguments,
            current_owner,
            descriptor_store,
            mappings,
            user_memory_start,
            user_memory,
            kernel_scratch,
            initramfs,
            file_descriptions,
        ),
        SyscallNumber::TalosSocket
        | SyscallNumber::TalosBind
        | SyscallNumber::TalosListen
        | SyscallNumber::TalosConnect
        | SyscallNumber::TalosAccept
        | SyscallNumber::TalosSend
        | SyscallNumber::TalosRecv
        | SyscallNumber::TalosNop
        | SyscallNumber::Unknown(_) => dispatch(raw_number, arguments).return_value(),
    };

    SyscallDispatchResult {
        number,
        arguments,
        return_value,
    }
}

pub(crate) fn dispatch_process_descriptor_with_socket_table<
    const OWNER_CAPACITY: usize,
    const DESCRIPTOR_CAPACITY: usize,
    const SOCKET_CAPACITY: usize,
    B,
>(
    raw_number: u64,
    arguments: SyscallArguments,
    current_owner: Option<crate::scheduler::ProcessOwnerId>,
    descriptor_store: &mut crate::posix::ProcessDescriptorStore<
        OWNER_CAPACITY,
        DESCRIPTOR_CAPACITY,
    >,
    socket_table: &mut crate::network::NetworkSocketDescriptorTable<SOCKET_CAPACITY>,
    mappings: &[crate::posix::UserMapping],
    user_memory_start: u64,
    user_memory: &mut [u8],
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
        SyscallNumber::TalosClose => dispatch_talos_close_socket_aware(
            arguments,
            current_owner,
            descriptor_store,
            socket_table,
        ),
        SyscallNumber::TalosDup => dispatch_talos_dup(arguments, current_owner, descriptor_store),
        SyscallNumber::TalosSocket => {
            dispatch_talos_socket(arguments, current_owner, descriptor_store, socket_table)
        }
        SyscallNumber::TalosBind => {
            dispatch_talos_bind(arguments, current_owner, descriptor_store, socket_table)
        }
        SyscallNumber::TalosListen => {
            dispatch_talos_listen(arguments, current_owner, descriptor_store, socket_table)
        }
        SyscallNumber::TalosConnect => {
            dispatch_talos_connect(arguments, current_owner, descriptor_store, socket_table)
        }
        SyscallNumber::TalosAccept => {
            dispatch_talos_accept(arguments, current_owner, descriptor_store, socket_table)
        }
        SyscallNumber::TalosSend => dispatch_talos_send(
            arguments,
            current_owner,
            descriptor_store,
            socket_table,
            mappings,
            user_memory_start,
            user_memory,
            kernel_scratch,
        ),
        SyscallNumber::TalosRecv => dispatch_talos_recv(
            arguments,
            current_owner,
            descriptor_store,
            socket_table,
            mappings,
            user_memory_start,
            user_memory,
            kernel_scratch,
        ),
        SyscallNumber::TalosRead
        | SyscallNumber::TalosOpen
        | SyscallNumber::TalosNop
        | SyscallNumber::Unknown(_) => dispatch(raw_number, arguments).return_value(),
    };

    SyscallDispatchResult {
        number,
        arguments,
        return_value,
    }
}

struct NoConsoleInput;

impl crate::runtime_console::ConsoleInputBackend for NoConsoleInput {
    fn poll_read_byte(&mut self) -> Option<u8> {
        None
    }
}

pub(crate) fn dispatch_process_descriptor_with_initramfs_and_console_stdin<
    const OWNER_CAPACITY: usize,
    const DESCRIPTOR_CAPACITY: usize,
    const FILE_CAPACITY: usize,
    B,
    I,
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
    user_memory: &mut [u8],
    kernel_scratch: &mut [u8],
    console_backend: &mut B,
    initramfs: crate::initramfs::ReadOnlyInitramfs,
    file_descriptions: &mut crate::initramfs::ReadOnlyFileDescriptions<FILE_CAPACITY>,
    fixed_stdin: Option<&mut crate::posix::FixedStdin<'_>>,
    console_stdin: Option<&mut I>,
) -> SyscallDispatchResult
where
    B: crate::runtime_console::ConsoleBackend,
    I: crate::runtime_console::ConsoleInputBackend,
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
        SyscallNumber::TalosDup => dispatch_talos_dup(arguments, current_owner, descriptor_store),
        SyscallNumber::TalosRead => dispatch_talos_read(
            arguments,
            current_owner,
            descriptor_store,
            mappings,
            user_memory_start,
            user_memory,
            kernel_scratch,
            fixed_stdin,
            console_stdin,
            Some(initramfs),
            Some(file_descriptions),
        ),
        SyscallNumber::TalosOpen => dispatch_talos_open_initramfs(
            arguments,
            current_owner,
            descriptor_store,
            mappings,
            user_memory_start,
            user_memory,
            kernel_scratch,
            initramfs,
            file_descriptions,
        ),
        SyscallNumber::TalosSocket
        | SyscallNumber::TalosBind
        | SyscallNumber::TalosListen
        | SyscallNumber::TalosConnect
        | SyscallNumber::TalosAccept
        | SyscallNumber::TalosSend
        | SyscallNumber::TalosRecv
        | SyscallNumber::TalosNop
        | SyscallNumber::Unknown(_) => dispatch(raw_number, arguments).return_value(),
    };

    SyscallDispatchResult {
        number,
        arguments,
        return_value,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PingOperationSyscallSubstituteStatusKind {
    Idle,
    PendingArp,
    Inflight,
    Completed,
    TimedOut,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PingOperationSyscallSubstituteStatus {
    kind: PingOperationSyscallSubstituteStatusKind,
    destination_ipv4: [u8; 4],
    next_hop_ipv4: [u8; 4],
    payload_len: usize,
    arp_retries_remaining: usize,
}

impl PingOperationSyscallSubstituteStatus {
    pub(crate) const fn idle() -> Self {
        Self {
            kind: PingOperationSyscallSubstituteStatusKind::Idle,
            destination_ipv4: [0; 4],
            next_hop_ipv4: [0; 4],
            payload_len: 0,
            arp_retries_remaining: 0,
        }
    }

    pub(crate) const fn kind(self) -> PingOperationSyscallSubstituteStatusKind {
        self.kind
    }

    pub(crate) const fn destination_ipv4(self) -> [u8; 4] {
        self.destination_ipv4
    }

    pub(crate) const fn next_hop_ipv4(self) -> [u8; 4] {
        self.next_hop_ipv4
    }

    pub(crate) const fn payload_len(self) -> usize {
        self.payload_len
    }

    pub(crate) const fn arp_retries_remaining(self) -> usize {
        self.arp_retries_remaining
    }

    const fn from_userspace(status: crate::network::UserspacePingOperationStatus) -> Self {
        match status {
            crate::network::UserspacePingOperationStatus::Idle => Self::idle(),
            crate::network::UserspacePingOperationStatus::PendingArp {
                destination_ipv4,
                next_hop_ipv4,
                arp_retries_remaining,
            } => Self {
                kind: PingOperationSyscallSubstituteStatusKind::PendingArp,
                destination_ipv4,
                next_hop_ipv4,
                payload_len: 0,
                arp_retries_remaining,
            },
            crate::network::UserspacePingOperationStatus::Inflight { destination_ipv4 } => Self {
                kind: PingOperationSyscallSubstituteStatusKind::Inflight,
                destination_ipv4,
                next_hop_ipv4: [0; 4],
                payload_len: 0,
                arp_retries_remaining: 0,
            },
            crate::network::UserspacePingOperationStatus::Completed {
                destination_ipv4,
                payload_len,
            } => Self {
                kind: PingOperationSyscallSubstituteStatusKind::Completed,
                destination_ipv4,
                next_hop_ipv4: [0; 4],
                payload_len,
                arp_retries_remaining: 0,
            },
            crate::network::UserspacePingOperationStatus::TimedOut { destination_ipv4 } => Self {
                kind: PingOperationSyscallSubstituteStatusKind::TimedOut,
                destination_ipv4,
                next_hop_ipv4: [0; 4],
                payload_len: 0,
                arp_retries_remaining: 0,
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PingOperationSyscallSubstituteStepKind {
    StartedPendingArp,
    StartedInflight,
    NoFrame,
    AdvancedToInflight,
    RetryTransmitted,
    Completed,
    TimedOut,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PingOperationSyscallSubstituteStep {
    kind: PingOperationSyscallSubstituteStepKind,
    frame_len: usize,
    payload_len: usize,
    destination_ipv4: [u8; 4],
}

impl PingOperationSyscallSubstituteStep {
    pub(crate) const fn kind(self) -> PingOperationSyscallSubstituteStepKind {
        self.kind
    }

    pub(crate) const fn frame_len(self) -> usize {
        self.frame_len
    }

    pub(crate) const fn payload_len(self) -> usize {
        self.payload_len
    }

    pub(crate) const fn destination_ipv4(self) -> [u8; 4] {
        self.destination_ipv4
    }

    pub(crate) const fn from_userspace(step: crate::network::UserspacePingOperationStep) -> Self {
        match step {
            crate::network::UserspacePingOperationStep::StartedPendingArp { frame_len } => Self {
                kind: PingOperationSyscallSubstituteStepKind::StartedPendingArp,
                frame_len,
                payload_len: 0,
                destination_ipv4: [0; 4],
            },
            crate::network::UserspacePingOperationStep::StartedInflight { frame_len } => Self {
                kind: PingOperationSyscallSubstituteStepKind::StartedInflight,
                frame_len,
                payload_len: 0,
                destination_ipv4: [0; 4],
            },
            crate::network::UserspacePingOperationStep::NoFrame => Self {
                kind: PingOperationSyscallSubstituteStepKind::NoFrame,
                frame_len: 0,
                payload_len: 0,
                destination_ipv4: [0; 4],
            },
            crate::network::UserspacePingOperationStep::AdvancedToInflight { frame_len } => Self {
                kind: PingOperationSyscallSubstituteStepKind::AdvancedToInflight,
                frame_len,
                payload_len: 0,
                destination_ipv4: [0; 4],
            },
            crate::network::UserspacePingOperationStep::RetryTransmitted { frame_len } => Self {
                kind: PingOperationSyscallSubstituteStepKind::RetryTransmitted,
                frame_len,
                payload_len: 0,
                destination_ipv4: [0; 4],
            },
            crate::network::UserspacePingOperationStep::Completed { payload_len } => Self {
                kind: PingOperationSyscallSubstituteStepKind::Completed,
                frame_len: 0,
                payload_len,
                destination_ipv4: [0; 4],
            },
            crate::network::UserspacePingOperationStep::TimedOut { destination_ipv4 } => Self {
                kind: PingOperationSyscallSubstituteStepKind::TimedOut,
                frame_len: 0,
                payload_len: 0,
                destination_ipv4,
            },
        }
    }
}

pub(crate) struct PingOperationSyscallSubstitute<
    'a,
    const DESCRIPTOR_CAPACITY: usize,
    const ARP_CAPACITY: usize,
    const PAYLOAD_CAPACITY: usize,
> {
    descriptor_table: &'a mut crate::network::NetworkPingOperationDescriptorTable<
        DESCRIPTOR_CAPACITY,
        ARP_CAPACITY,
        PAYLOAD_CAPACITY,
    >,
    receive_buffer: &'a mut [u8],
    transmit_buffer: &'a mut [u8],
}

impl<'a, const DESCRIPTOR_CAPACITY: usize, const ARP_CAPACITY: usize, const PAYLOAD_CAPACITY: usize>
    PingOperationSyscallSubstitute<'a, DESCRIPTOR_CAPACITY, ARP_CAPACITY, PAYLOAD_CAPACITY>
{
    pub(crate) fn new(
        descriptor_table: &'a mut crate::network::NetworkPingOperationDescriptorTable<
            DESCRIPTOR_CAPACITY,
            ARP_CAPACITY,
            PAYLOAD_CAPACITY,
        >,
        receive_buffer: &'a mut [u8],
        transmit_buffer: &'a mut [u8],
    ) -> Self {
        Self {
            descriptor_table,
            receive_buffer,
            transmit_buffer,
        }
    }

    pub(crate) fn open(&mut self) -> Result<usize, PosixError> {
        self.descriptor_table
            .open()
            .map(crate::network::NetworkPingOperationDescriptor::raw)
    }

    pub(crate) fn close(&mut self, descriptor: usize) -> Result<(), PosixError> {
        self.descriptor_table
            .close(Self::operation_descriptor(descriptor))
    }

    pub(crate) fn status(
        &self,
        descriptor: usize,
        status: &mut PingOperationSyscallSubstituteStatus,
    ) -> Result<(), PosixError> {
        *status = PingOperationSyscallSubstituteStatus::from_userspace(
            self.descriptor_table
                .status(Self::operation_descriptor(descriptor))?,
        );
        Ok(())
    }

    pub(crate) fn start<D>(
        &mut self,
        descriptor: usize,
        device: &mut D,
        endpoint: crate::network::LocalNetworkEndpoint,
        route_policy: crate::network::Ipv4EgressRoutePolicy,
        destination_ipv4: [u8; 4],
        identifier: u16,
        sequence_number: u16,
        ttl: u8,
        payload: &[u8],
        arp_retry_budget: usize,
    ) -> Result<PingOperationSyscallSubstituteStep, PosixError>
    where
        D: crate::network::NetworkDevice,
    {
        self.descriptor_table
            .start(
                Self::operation_descriptor(descriptor),
                device,
                endpoint,
                route_policy,
                destination_ipv4,
                identifier,
                sequence_number,
                ttl,
                payload,
                self.transmit_buffer,
                arp_retry_budget,
            )
            .map(PingOperationSyscallSubstituteStep::from_userspace)
    }

    pub(crate) fn pump<D>(
        &mut self,
        descriptor: usize,
        device: &mut D,
    ) -> Result<PingOperationSyscallSubstituteStep, PosixError>
    where
        D: crate::network::NetworkDevice,
    {
        self.descriptor_table
            .pump(
                Self::operation_descriptor(descriptor),
                device,
                self.receive_buffer,
                self.transmit_buffer,
            )
            .map(PingOperationSyscallSubstituteStep::from_userspace)
    }

    pub(crate) fn retry_arp<D>(
        &mut self,
        descriptor: usize,
        device: &mut D,
    ) -> Result<PingOperationSyscallSubstituteStep, PosixError>
    where
        D: crate::network::NetworkDevice,
    {
        self.descriptor_table
            .retry_arp(
                Self::operation_descriptor(descriptor),
                device,
                self.transmit_buffer,
            )
            .map(PingOperationSyscallSubstituteStep::from_userspace)
    }

    pub(crate) fn timeout(
        &mut self,
        descriptor: usize,
    ) -> Result<PingOperationSyscallSubstituteStep, PosixError> {
        self.descriptor_table
            .timeout(Self::operation_descriptor(descriptor))
            .map(PingOperationSyscallSubstituteStep::from_userspace)
    }

    const fn operation_descriptor(
        descriptor: usize,
    ) -> crate::network::NetworkPingOperationDescriptor {
        crate::network::NetworkPingOperationDescriptor::from_raw(descriptor)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RuntimePingOperationSyscallSubstitutePumpKind {
    NoFrame,
    LocalNoReply,
    LocalReply,
    ActivePing,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct RuntimePingOperationSyscallSubstitutePumpStep {
    kind: RuntimePingOperationSyscallSubstitutePumpKind,
    descriptor: usize,
    local_reply_kind: Option<crate::network::PacketReplyKind>,
    local_reply_frame_len: usize,
    active_ping_step: PingOperationSyscallSubstituteStep,
}

impl RuntimePingOperationSyscallSubstitutePumpStep {
    pub(crate) const fn kind(self) -> RuntimePingOperationSyscallSubstitutePumpKind {
        self.kind
    }

    pub(crate) const fn descriptor(self) -> usize {
        self.descriptor
    }

    pub(crate) const fn local_reply_kind(self) -> Option<crate::network::PacketReplyKind> {
        self.local_reply_kind
    }

    pub(crate) const fn local_reply_frame_len(self) -> usize {
        self.local_reply_frame_len
    }

    pub(crate) const fn active_ping_step(self) -> PingOperationSyscallSubstituteStep {
        self.active_ping_step
    }

    pub(crate) const fn no_frame() -> Self {
        Self {
            kind: RuntimePingOperationSyscallSubstitutePumpKind::NoFrame,
            descriptor: usize::MAX,
            local_reply_kind: None,
            local_reply_frame_len: 0,
            active_ping_step: PingOperationSyscallSubstituteStep::from_userspace(
                crate::network::UserspacePingOperationStep::NoFrame,
            ),
        }
    }

    const fn local_no_reply() -> Self {
        Self {
            kind: RuntimePingOperationSyscallSubstitutePumpKind::LocalNoReply,
            descriptor: usize::MAX,
            local_reply_kind: None,
            local_reply_frame_len: 0,
            active_ping_step: PingOperationSyscallSubstituteStep::from_userspace(
                crate::network::UserspacePingOperationStep::NoFrame,
            ),
        }
    }

    const fn local_reply(reply: crate::network::PacketDispatchResult) -> Self {
        Self {
            kind: RuntimePingOperationSyscallSubstitutePumpKind::LocalReply,
            descriptor: usize::MAX,
            local_reply_kind: Some(reply.reply_kind()),
            local_reply_frame_len: reply.frame_len(),
            active_ping_step: PingOperationSyscallSubstituteStep::from_userspace(
                crate::network::UserspacePingOperationStep::NoFrame,
            ),
        }
    }

    const fn active_ping(
        descriptor: crate::network::NetworkPingOperationDescriptor,
        step: crate::network::UserspacePingOperationStep,
    ) -> Self {
        Self {
            kind: RuntimePingOperationSyscallSubstitutePumpKind::ActivePing,
            descriptor: descriptor.raw(),
            local_reply_kind: None,
            local_reply_frame_len: 0,
            active_ping_step: PingOperationSyscallSubstituteStep::from_userspace(step),
        }
    }
}

pub(crate) struct RuntimePingOperationSyscallSubstitute<
    'a,
    const LOCAL_ARP_CAPACITY: usize,
    const DESCRIPTOR_CAPACITY: usize,
    const OPERATION_ARP_CAPACITY: usize,
    const PAYLOAD_CAPACITY: usize,
> {
    runtime_pump: &'a mut crate::network::NetworkRuntimeDevicePump<
        LOCAL_ARP_CAPACITY,
        DESCRIPTOR_CAPACITY,
        OPERATION_ARP_CAPACITY,
        PAYLOAD_CAPACITY,
    >,
    receive_buffer: &'a mut [u8],
    transmit_buffer: &'a mut [u8],
}

impl<
    'a,
    const LOCAL_ARP_CAPACITY: usize,
    const DESCRIPTOR_CAPACITY: usize,
    const OPERATION_ARP_CAPACITY: usize,
    const PAYLOAD_CAPACITY: usize,
>
    RuntimePingOperationSyscallSubstitute<
        'a,
        LOCAL_ARP_CAPACITY,
        DESCRIPTOR_CAPACITY,
        OPERATION_ARP_CAPACITY,
        PAYLOAD_CAPACITY,
    >
{
    pub(crate) fn new(
        runtime_pump: &'a mut crate::network::NetworkRuntimeDevicePump<
            LOCAL_ARP_CAPACITY,
            DESCRIPTOR_CAPACITY,
            OPERATION_ARP_CAPACITY,
            PAYLOAD_CAPACITY,
        >,
        receive_buffer: &'a mut [u8],
        transmit_buffer: &'a mut [u8],
    ) -> Self {
        Self {
            runtime_pump,
            receive_buffer,
            transmit_buffer,
        }
    }

    pub(crate) fn open(&mut self) -> Result<usize, PosixError> {
        self.runtime_pump
            .open_ping_operation()
            .map(crate::network::NetworkPingOperationDescriptor::raw)
    }

    pub(crate) fn close(&mut self, descriptor: usize) -> Result<(), PosixError> {
        self.runtime_pump
            .close_ping_operation(Self::operation_descriptor(descriptor))
    }

    pub(crate) fn status(
        &self,
        descriptor: usize,
        status: &mut PingOperationSyscallSubstituteStatus,
    ) -> Result<(), PosixError> {
        *status = PingOperationSyscallSubstituteStatus::from_userspace(
            self.runtime_pump
                .ping_status(Self::operation_descriptor(descriptor))?,
        );
        Ok(())
    }

    pub(crate) fn start<D>(
        &mut self,
        descriptor: usize,
        device: &mut D,
        route_policy: crate::network::Ipv4EgressRoutePolicy,
        destination_ipv4: [u8; 4],
        identifier: u16,
        sequence_number: u16,
        ttl: u8,
        payload: &[u8],
        arp_retry_budget: usize,
    ) -> Result<PingOperationSyscallSubstituteStep, PosixError>
    where
        D: crate::network::NetworkDevice,
    {
        self.runtime_pump
            .start_ping(
                Self::operation_descriptor(descriptor),
                device,
                route_policy,
                destination_ipv4,
                identifier,
                sequence_number,
                ttl,
                payload,
                self.transmit_buffer,
                arp_retry_budget,
            )
            .map(PingOperationSyscallSubstituteStep::from_userspace)
    }

    pub(crate) fn pump<D>(
        &mut self,
        descriptor: usize,
        device: &mut D,
    ) -> Result<RuntimePingOperationSyscallSubstitutePumpStep, PosixError>
    where
        D: crate::network::NetworkDevice,
    {
        let active_ping = Self::operation_descriptor(descriptor);
        match self.runtime_pump.pump(
            device,
            Some(active_ping),
            self.receive_buffer,
            self.transmit_buffer,
        ) {
            crate::network::NetworkRuntimeDevicePumpStepResult::NoFrame => {
                Ok(RuntimePingOperationSyscallSubstitutePumpStep::no_frame())
            }
            crate::network::NetworkRuntimeDevicePumpStepResult::ReceiveBufferTooSmall => {
                Err(PosixError::NoSpace)
            }
            crate::network::NetworkRuntimeDevicePumpStepResult::ReceiveError(error) => {
                Err(crate::network::posix_error_from_device_error(error))
            }
            crate::network::NetworkRuntimeDevicePumpStepResult::LocalNoReply => {
                Ok(RuntimePingOperationSyscallSubstitutePumpStep::local_no_reply())
            }
            crate::network::NetworkRuntimeDevicePumpStepResult::LocalDispatchError(error) => {
                Err(crate::network::posix_error_from_packet_error(error))
            }
            crate::network::NetworkRuntimeDevicePumpStepResult::LocalTransmitError(error) => {
                Err(crate::network::posix_error_from_device_error(error))
            }
            crate::network::NetworkRuntimeDevicePumpStepResult::LocalReply(reply) => Ok(
                RuntimePingOperationSyscallSubstitutePumpStep::local_reply(reply),
            ),
            crate::network::NetworkRuntimeDevicePumpStepResult::ActivePingStep {
                descriptor,
                step,
            } => Ok(RuntimePingOperationSyscallSubstitutePumpStep::active_ping(
                descriptor, step,
            )),
            crate::network::NetworkRuntimeDevicePumpStepResult::ActivePingError {
                error, ..
            } => Err(error),
        }
    }

    pub(crate) fn retry_arp<D>(
        &mut self,
        descriptor: usize,
        device: &mut D,
    ) -> Result<PingOperationSyscallSubstituteStep, PosixError>
    where
        D: crate::network::NetworkDevice,
    {
        self.runtime_pump
            .retry_ping_arp(
                Self::operation_descriptor(descriptor),
                device,
                self.transmit_buffer,
            )
            .map(PingOperationSyscallSubstituteStep::from_userspace)
    }

    pub(crate) fn timeout(
        &mut self,
        descriptor: usize,
    ) -> Result<PingOperationSyscallSubstituteStep, PosixError> {
        self.runtime_pump
            .timeout_ping(Self::operation_descriptor(descriptor))
            .map(PingOperationSyscallSubstituteStep::from_userspace)
    }

    const fn operation_descriptor(
        descriptor: usize,
    ) -> crate::network::NetworkPingOperationDescriptor {
        crate::network::NetworkPingOperationDescriptor::from_raw(descriptor)
    }
}

pub(crate) struct DescriptorShapedPingControl<
    'a,
    const LOCAL_ARP_CAPACITY: usize,
    const DESCRIPTOR_CAPACITY: usize,
    const OPERATION_ARP_CAPACITY: usize,
    const PAYLOAD_CAPACITY: usize,
> {
    runtime_substitute: RuntimePingOperationSyscallSubstitute<
        'a,
        LOCAL_ARP_CAPACITY,
        DESCRIPTOR_CAPACITY,
        OPERATION_ARP_CAPACITY,
        PAYLOAD_CAPACITY,
    >,
}

impl<
    'a,
    const LOCAL_ARP_CAPACITY: usize,
    const DESCRIPTOR_CAPACITY: usize,
    const OPERATION_ARP_CAPACITY: usize,
    const PAYLOAD_CAPACITY: usize,
>
    DescriptorShapedPingControl<
        'a,
        LOCAL_ARP_CAPACITY,
        DESCRIPTOR_CAPACITY,
        OPERATION_ARP_CAPACITY,
        PAYLOAD_CAPACITY,
    >
{
    pub(crate) fn new(
        runtime_pump: &'a mut crate::network::NetworkRuntimeDevicePump<
            LOCAL_ARP_CAPACITY,
            DESCRIPTOR_CAPACITY,
            OPERATION_ARP_CAPACITY,
            PAYLOAD_CAPACITY,
        >,
        receive_buffer: &'a mut [u8],
        transmit_buffer: &'a mut [u8],
    ) -> Self {
        Self {
            runtime_substitute: RuntimePingOperationSyscallSubstitute::new(
                runtime_pump,
                receive_buffer,
                transmit_buffer,
            ),
        }
    }

    pub(crate) fn open(&mut self) -> Result<usize, PosixError> {
        self.runtime_substitute.open()
    }

    pub(crate) fn close(&mut self, descriptor: usize) -> Result<(), PosixError> {
        self.runtime_substitute.close(descriptor)
    }

    pub(crate) fn status(
        &self,
        descriptor: usize,
        status: &mut PingOperationSyscallSubstituteStatus,
    ) -> Result<(), PosixError> {
        self.runtime_substitute.status(descriptor, status)
    }

    pub(crate) fn start<D>(
        &mut self,
        descriptor: usize,
        device: &mut D,
        route_policy: crate::network::Ipv4EgressRoutePolicy,
        destination_ipv4: [u8; 4],
        identifier: u16,
        sequence_number: u16,
        ttl: u8,
        payload: &[u8],
        arp_retry_budget: usize,
    ) -> Result<PingOperationSyscallSubstituteStep, PosixError>
    where
        D: crate::network::NetworkDevice,
    {
        self.runtime_substitute.start(
            descriptor,
            device,
            route_policy,
            destination_ipv4,
            identifier,
            sequence_number,
            ttl,
            payload,
            arp_retry_budget,
        )
    }

    pub(crate) fn pump_or_read_result<D>(
        &mut self,
        descriptor: usize,
        device: &mut D,
    ) -> Result<RuntimePingOperationSyscallSubstitutePumpStep, PosixError>
    where
        D: crate::network::NetworkDevice,
    {
        self.runtime_substitute.pump(descriptor, device)
    }

    pub(crate) fn retry_arp<D>(
        &mut self,
        descriptor: usize,
        device: &mut D,
    ) -> Result<PingOperationSyscallSubstituteStep, PosixError>
    where
        D: crate::network::NetworkDevice,
    {
        self.runtime_substitute.retry_arp(descriptor, device)
    }

    pub(crate) fn timeout(
        &mut self,
        descriptor: usize,
    ) -> Result<PingOperationSyscallSubstituteStep, PosixError> {
        self.runtime_substitute.timeout(descriptor)
    }
}

pub(crate) struct ProcessLocalPingDescriptorControl<
    'a,
    const OWNER_CAPACITY: usize,
    const PROCESS_DESCRIPTOR_CAPACITY: usize,
    const LOCAL_ARP_CAPACITY: usize,
    const PING_DESCRIPTOR_CAPACITY: usize,
    const OPERATION_ARP_CAPACITY: usize,
    const PAYLOAD_CAPACITY: usize,
> {
    current_owner: Option<crate::scheduler::ProcessOwnerId>,
    descriptor_store:
        &'a mut crate::posix::ProcessDescriptorStore<OWNER_CAPACITY, PROCESS_DESCRIPTOR_CAPACITY>,
    ping_control: DescriptorShapedPingControl<
        'a,
        LOCAL_ARP_CAPACITY,
        PING_DESCRIPTOR_CAPACITY,
        OPERATION_ARP_CAPACITY,
        PAYLOAD_CAPACITY,
    >,
}

impl<
    'a,
    const OWNER_CAPACITY: usize,
    const PROCESS_DESCRIPTOR_CAPACITY: usize,
    const LOCAL_ARP_CAPACITY: usize,
    const PING_DESCRIPTOR_CAPACITY: usize,
    const OPERATION_ARP_CAPACITY: usize,
    const PAYLOAD_CAPACITY: usize,
>
    ProcessLocalPingDescriptorControl<
        'a,
        OWNER_CAPACITY,
        PROCESS_DESCRIPTOR_CAPACITY,
        LOCAL_ARP_CAPACITY,
        PING_DESCRIPTOR_CAPACITY,
        OPERATION_ARP_CAPACITY,
        PAYLOAD_CAPACITY,
    >
{
    pub(crate) fn new(
        current_owner: Option<crate::scheduler::ProcessOwnerId>,
        descriptor_store: &'a mut crate::posix::ProcessDescriptorStore<
            OWNER_CAPACITY,
            PROCESS_DESCRIPTOR_CAPACITY,
        >,
        runtime_pump: &'a mut crate::network::NetworkRuntimeDevicePump<
            LOCAL_ARP_CAPACITY,
            PING_DESCRIPTOR_CAPACITY,
            OPERATION_ARP_CAPACITY,
            PAYLOAD_CAPACITY,
        >,
        receive_buffer: &'a mut [u8],
        transmit_buffer: &'a mut [u8],
    ) -> Self {
        Self {
            current_owner,
            descriptor_store,
            ping_control: DescriptorShapedPingControl::new(
                runtime_pump,
                receive_buffer,
                transmit_buffer,
            ),
        }
    }

    pub(crate) fn open(&mut self) -> Result<usize, PosixError> {
        self.descriptor_store
            .current_descriptor_table(self.current_owner)?;

        let ping_descriptor = self.ping_control.open()?;
        let entry = crate::posix::DescriptorEntry::new(
            crate::posix::DescriptorAccess::ReadWrite,
            crate::posix::DescriptorFlags::EMPTY,
            crate::posix::DescriptorObject::new(
                crate::posix::DescriptorObjectKind::OtherKernelObject,
                ping_descriptor,
            ),
        );

        match self
            .descriptor_store
            .current_descriptor_table_mut(self.current_owner)?
            .allocate(entry)
        {
            Ok(process_descriptor) => Ok(process_descriptor),
            Err(error) => {
                let _ = self.ping_control.close(ping_descriptor);
                Err(error)
            }
        }
    }

    pub(crate) fn close(&mut self, process_descriptor: usize) -> Result<(), PosixError> {
        let ping_descriptor = self.ping_descriptor(process_descriptor)?;
        let mut status = PingOperationSyscallSubstituteStatus::idle();
        self.ping_control.status(ping_descriptor, &mut status)?;
        self.descriptor_store
            .close_current_descriptor(self.current_owner, process_descriptor)?;
        self.ping_control.close(ping_descriptor)
    }

    pub(crate) fn status(
        &self,
        process_descriptor: usize,
        status: &mut PingOperationSyscallSubstituteStatus,
    ) -> Result<(), PosixError> {
        let ping_descriptor = self.ping_descriptor(process_descriptor)?;
        self.ping_control.status(ping_descriptor, status)
    }

    pub(crate) fn start<D>(
        &mut self,
        process_descriptor: usize,
        device: &mut D,
        route_policy: crate::network::Ipv4EgressRoutePolicy,
        destination_ipv4: [u8; 4],
        identifier: u16,
        sequence_number: u16,
        ttl: u8,
        payload: &[u8],
        arp_retry_budget: usize,
    ) -> Result<PingOperationSyscallSubstituteStep, PosixError>
    where
        D: crate::network::NetworkDevice,
    {
        let ping_descriptor = self.ping_descriptor(process_descriptor)?;
        self.ping_control.start(
            ping_descriptor,
            device,
            route_policy,
            destination_ipv4,
            identifier,
            sequence_number,
            ttl,
            payload,
            arp_retry_budget,
        )
    }

    pub(crate) fn pump_or_read_result<D>(
        &mut self,
        process_descriptor: usize,
        device: &mut D,
    ) -> Result<RuntimePingOperationSyscallSubstitutePumpStep, PosixError>
    where
        D: crate::network::NetworkDevice,
    {
        let ping_descriptor = self.ping_descriptor(process_descriptor)?;
        self.ping_control
            .pump_or_read_result(ping_descriptor, device)
    }

    pub(crate) fn retry_arp<D>(
        &mut self,
        process_descriptor: usize,
        device: &mut D,
    ) -> Result<PingOperationSyscallSubstituteStep, PosixError>
    where
        D: crate::network::NetworkDevice,
    {
        let ping_descriptor = self.ping_descriptor(process_descriptor)?;
        self.ping_control.retry_arp(ping_descriptor, device)
    }

    pub(crate) fn timeout(
        &mut self,
        process_descriptor: usize,
    ) -> Result<PingOperationSyscallSubstituteStep, PosixError> {
        let ping_descriptor = self.ping_descriptor(process_descriptor)?;
        self.ping_control.timeout(ping_descriptor)
    }

    fn ping_descriptor(&self, process_descriptor: usize) -> Result<usize, PosixError> {
        let entry = self
            .descriptor_store
            .current_descriptor_table(self.current_owner)?
            .get(process_descriptor)?;
        if entry.object().kind() != crate::posix::DescriptorObjectKind::OtherKernelObject {
            return Err(PosixError::BadDescriptor);
        }
        Ok(entry.object().reference())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ProcessLocalPingDispatchOperation<'a> {
    Open,
    Start {
        process_descriptor: usize,
        route_policy: crate::network::Ipv4EgressRoutePolicy,
        destination_ipv4: [u8; 4],
        identifier: u16,
        sequence_number: u16,
        ttl: u8,
        payload: &'a [u8],
        arp_retry_budget: usize,
    },
    PumpOrReadResult {
        process_descriptor: usize,
    },
    Status {
        process_descriptor: usize,
    },
    RetryArp {
        process_descriptor: usize,
    },
    Timeout {
        process_descriptor: usize,
    },
    Close {
        process_descriptor: usize,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ProcessLocalPingDispatchOutcome {
    Opened { process_descriptor: usize },
    Started,
    PumpedOrReadResult,
    Status,
    RetriedArp,
    TimedOut,
    Closed,
}

pub(crate) struct ProcessLocalPingDispatchOutputs<'a> {
    step: &'a mut PingOperationSyscallSubstituteStep,
    pump_step: &'a mut RuntimePingOperationSyscallSubstitutePumpStep,
    status: &'a mut PingOperationSyscallSubstituteStatus,
}

impl<'a> ProcessLocalPingDispatchOutputs<'a> {
    pub(crate) fn new(
        step: &'a mut PingOperationSyscallSubstituteStep,
        pump_step: &'a mut RuntimePingOperationSyscallSubstitutePumpStep,
        status: &'a mut PingOperationSyscallSubstituteStatus,
    ) -> Self {
        Self {
            step,
            pump_step,
            status,
        }
    }
}

pub(crate) fn dispatch_process_local_ping_descriptor_operation<
    const OWNER_CAPACITY: usize,
    const PROCESS_DESCRIPTOR_CAPACITY: usize,
    const LOCAL_ARP_CAPACITY: usize,
    const PING_DESCRIPTOR_CAPACITY: usize,
    const OPERATION_ARP_CAPACITY: usize,
    const PAYLOAD_CAPACITY: usize,
    D,
>(
    operation: ProcessLocalPingDispatchOperation<'_>,
    current_owner: Option<crate::scheduler::ProcessOwnerId>,
    descriptor_store: &mut crate::posix::ProcessDescriptorStore<
        OWNER_CAPACITY,
        PROCESS_DESCRIPTOR_CAPACITY,
    >,
    runtime_pump: &mut crate::network::NetworkRuntimeDevicePump<
        LOCAL_ARP_CAPACITY,
        PING_DESCRIPTOR_CAPACITY,
        OPERATION_ARP_CAPACITY,
        PAYLOAD_CAPACITY,
    >,
    receive_buffer: &mut [u8],
    transmit_buffer: &mut [u8],
    device: &mut D,
    outputs: &mut ProcessLocalPingDispatchOutputs<'_>,
) -> Result<ProcessLocalPingDispatchOutcome, PosixError>
where
    D: crate::network::NetworkDevice,
{
    let mut control = ProcessLocalPingDescriptorControl::new(
        current_owner,
        descriptor_store,
        runtime_pump,
        receive_buffer,
        transmit_buffer,
    );

    match operation {
        ProcessLocalPingDispatchOperation::Open => {
            control.open().map(
                |process_descriptor| ProcessLocalPingDispatchOutcome::Opened { process_descriptor },
            )
        }
        ProcessLocalPingDispatchOperation::Start {
            process_descriptor,
            route_policy,
            destination_ipv4,
            identifier,
            sequence_number,
            ttl,
            payload,
            arp_retry_budget,
        } => {
            *outputs.step = control.start(
                process_descriptor,
                device,
                route_policy,
                destination_ipv4,
                identifier,
                sequence_number,
                ttl,
                payload,
                arp_retry_budget,
            )?;
            Ok(ProcessLocalPingDispatchOutcome::Started)
        }
        ProcessLocalPingDispatchOperation::PumpOrReadResult { process_descriptor } => {
            *outputs.pump_step = control.pump_or_read_result(process_descriptor, device)?;
            Ok(ProcessLocalPingDispatchOutcome::PumpedOrReadResult)
        }
        ProcessLocalPingDispatchOperation::Status { process_descriptor } => {
            control.status(process_descriptor, outputs.status)?;
            Ok(ProcessLocalPingDispatchOutcome::Status)
        }
        ProcessLocalPingDispatchOperation::RetryArp { process_descriptor } => {
            *outputs.step = control.retry_arp(process_descriptor, device)?;
            Ok(ProcessLocalPingDispatchOutcome::RetriedArp)
        }
        ProcessLocalPingDispatchOperation::Timeout { process_descriptor } => {
            *outputs.step = control.timeout(process_descriptor)?;
            Ok(ProcessLocalPingDispatchOutcome::TimedOut)
        }
        ProcessLocalPingDispatchOperation::Close { process_descriptor } => {
            control.close(process_descriptor)?;
            Ok(ProcessLocalPingDispatchOutcome::Closed)
        }
    }
}

pub(crate) const PROCESS_LOCAL_PING_USER_SELECTOR_OPEN: u64 = 0;
pub(crate) const PROCESS_LOCAL_PING_USER_SELECTOR_START: u64 = 1;
pub(crate) const PROCESS_LOCAL_PING_USER_SELECTOR_PUMP_OR_READ_RESULT: u64 = 2;
pub(crate) const PROCESS_LOCAL_PING_USER_SELECTOR_STATUS: u64 = 3;
pub(crate) const PROCESS_LOCAL_PING_USER_SELECTOR_RETRY_ARP: u64 = 4;
pub(crate) const PROCESS_LOCAL_PING_USER_SELECTOR_TIMEOUT: u64 = 5;
pub(crate) const PROCESS_LOCAL_PING_USER_SELECTOR_CLOSE: u64 = 6;
pub(crate) const PROCESS_LOCAL_PING_USER_STEP_RECORD_LEN: usize = 32;
pub(crate) const PROCESS_LOCAL_PING_USER_PUMP_RECORD_LEN: usize = 64;
pub(crate) const PROCESS_LOCAL_PING_USER_STATUS_RECORD_LEN: usize = 40;

const PROCESS_LOCAL_PING_USER_ROUTE_SAME_SUBNET: u8 = 0;
const PROCESS_LOCAL_PING_USER_MAX_ARP_RETRY_BUDGET: u64 = u32::MAX as u64;

pub(crate) fn dispatch_process_local_ping_descriptor_user_arguments<
    const OWNER_CAPACITY: usize,
    const PROCESS_DESCRIPTOR_CAPACITY: usize,
    const LOCAL_ARP_CAPACITY: usize,
    const PING_DESCRIPTOR_CAPACITY: usize,
    const OPERATION_ARP_CAPACITY: usize,
    const PAYLOAD_CAPACITY: usize,
    D,
>(
    arguments: SyscallArguments,
    current_owner: Option<crate::scheduler::ProcessOwnerId>,
    descriptor_store: &mut crate::posix::ProcessDescriptorStore<
        OWNER_CAPACITY,
        PROCESS_DESCRIPTOR_CAPACITY,
    >,
    mappings: &[crate::posix::UserMapping],
    user_memory_start: u64,
    user_memory: &mut [u8],
    kernel_scratch: &mut [u8],
    runtime_pump: &mut crate::network::NetworkRuntimeDevicePump<
        LOCAL_ARP_CAPACITY,
        PING_DESCRIPTOR_CAPACITY,
        OPERATION_ARP_CAPACITY,
        PAYLOAD_CAPACITY,
    >,
    receive_buffer: &mut [u8],
    transmit_buffer: &mut [u8],
    device: &mut D,
    outputs: &mut ProcessLocalPingDispatchOutputs<'_>,
) -> Result<ProcessLocalPingDispatchOutcome, PosixError>
where
    D: crate::network::NetworkDevice,
{
    match arguments.get(0).ok_or(PosixError::InvalidArgument)? {
        PROCESS_LOCAL_PING_USER_SELECTOR_OPEN => {
            require_process_local_ping_reserved(arguments, 1)?;
            dispatch_process_local_ping_descriptor_operation(
                ProcessLocalPingDispatchOperation::Open,
                current_owner,
                descriptor_store,
                runtime_pump,
                receive_buffer,
                transmit_buffer,
                device,
                outputs,
            )
        }
        PROCESS_LOCAL_PING_USER_SELECTOR_START => {
            let process_descriptor = process_local_ping_user_usize_arg(arguments, 1)?;
            let payload_user_start = arguments.get(2).ok_or(PosixError::InvalidArgument)?;
            let payload_len = process_local_ping_user_usize_arg(arguments, 3)?;
            let packed_route = arguments.get(4).ok_or(PosixError::InvalidArgument)?;
            let packed_control = arguments.get(5).ok_or(PosixError::InvalidArgument)?;
            let route_policy = process_local_ping_user_route_policy(packed_route)?;
            let destination_ipv4 = process_local_ping_user_destination_ipv4(packed_route);
            let ttl = process_local_ping_user_ttl(packed_route)?;
            let identifier = process_local_ping_user_u16_arg(packed_control & 0xffff)?;
            let sequence_number = process_local_ping_user_u16_arg((packed_control >> 16) & 0xffff)?;
            let arp_retry_budget_raw = packed_control >> 32;
            if arp_retry_budget_raw > PROCESS_LOCAL_PING_USER_MAX_ARP_RETRY_BUDGET {
                return Err(PosixError::InvalidArgument);
            }
            let arp_retry_budget = process_local_ping_user_usize(arp_retry_budget_raw)?;
            if payload_len > kernel_scratch.len() {
                return Err(PosixError::NoSpace);
            }
            crate::posix::copy_from_user(
                mappings,
                user_memory_start,
                user_memory,
                payload_user_start,
                payload_len,
                &mut kernel_scratch[..payload_len],
            )?;
            dispatch_process_local_ping_descriptor_operation(
                ProcessLocalPingDispatchOperation::Start {
                    process_descriptor,
                    route_policy,
                    destination_ipv4,
                    identifier,
                    sequence_number,
                    ttl,
                    payload: &kernel_scratch[..payload_len],
                    arp_retry_budget,
                },
                current_owner,
                descriptor_store,
                runtime_pump,
                receive_buffer,
                transmit_buffer,
                device,
                outputs,
            )
        }
        PROCESS_LOCAL_PING_USER_SELECTOR_PUMP_OR_READ_RESULT => {
            let process_descriptor = process_local_ping_user_usize_arg(arguments, 1)?;
            let output_user_start = arguments.get(2).ok_or(PosixError::InvalidArgument)?;
            let output_len = process_local_ping_user_usize_arg(arguments, 3)?;
            require_process_local_ping_reserved_from(arguments, 4)?;
            if output_len < PROCESS_LOCAL_PING_USER_PUMP_RECORD_LEN {
                return Err(PosixError::NoSpace);
            }
            let outcome = dispatch_process_local_ping_descriptor_operation(
                ProcessLocalPingDispatchOperation::PumpOrReadResult { process_descriptor },
                current_owner,
                descriptor_store,
                runtime_pump,
                receive_buffer,
                transmit_buffer,
                device,
                outputs,
            )?;
            let mut record = [0u8; PROCESS_LOCAL_PING_USER_PUMP_RECORD_LEN];
            process_local_ping_user_encode_pump_step(outputs.pump_step, &mut record);
            crate::posix::copy_to_user(
                mappings,
                user_memory_start,
                user_memory,
                output_user_start,
                PROCESS_LOCAL_PING_USER_PUMP_RECORD_LEN,
                &record,
            )?;
            Ok(outcome)
        }
        PROCESS_LOCAL_PING_USER_SELECTOR_STATUS => {
            let process_descriptor = process_local_ping_user_usize_arg(arguments, 1)?;
            let output_user_start = arguments.get(2).ok_or(PosixError::InvalidArgument)?;
            let output_len = process_local_ping_user_usize_arg(arguments, 3)?;
            require_process_local_ping_reserved_from(arguments, 4)?;
            if output_len < PROCESS_LOCAL_PING_USER_STATUS_RECORD_LEN {
                return Err(PosixError::NoSpace);
            }
            let outcome = dispatch_process_local_ping_descriptor_operation(
                ProcessLocalPingDispatchOperation::Status { process_descriptor },
                current_owner,
                descriptor_store,
                runtime_pump,
                receive_buffer,
                transmit_buffer,
                device,
                outputs,
            )?;
            let mut record = [0u8; PROCESS_LOCAL_PING_USER_STATUS_RECORD_LEN];
            process_local_ping_user_encode_status(outputs.status, &mut record);
            crate::posix::copy_to_user(
                mappings,
                user_memory_start,
                user_memory,
                output_user_start,
                PROCESS_LOCAL_PING_USER_STATUS_RECORD_LEN,
                &record,
            )?;
            Ok(outcome)
        }
        PROCESS_LOCAL_PING_USER_SELECTOR_RETRY_ARP => {
            let process_descriptor = process_local_ping_user_usize_arg(arguments, 1)?;
            require_process_local_ping_reserved_from(arguments, 2)?;
            dispatch_process_local_ping_descriptor_operation(
                ProcessLocalPingDispatchOperation::RetryArp { process_descriptor },
                current_owner,
                descriptor_store,
                runtime_pump,
                receive_buffer,
                transmit_buffer,
                device,
                outputs,
            )
        }
        PROCESS_LOCAL_PING_USER_SELECTOR_TIMEOUT => {
            let process_descriptor = process_local_ping_user_usize_arg(arguments, 1)?;
            require_process_local_ping_reserved_from(arguments, 2)?;
            dispatch_process_local_ping_descriptor_operation(
                ProcessLocalPingDispatchOperation::Timeout { process_descriptor },
                current_owner,
                descriptor_store,
                runtime_pump,
                receive_buffer,
                transmit_buffer,
                device,
                outputs,
            )
        }
        PROCESS_LOCAL_PING_USER_SELECTOR_CLOSE => {
            let process_descriptor = process_local_ping_user_usize_arg(arguments, 1)?;
            require_process_local_ping_reserved_from(arguments, 2)?;
            dispatch_process_local_ping_descriptor_operation(
                ProcessLocalPingDispatchOperation::Close { process_descriptor },
                current_owner,
                descriptor_store,
                runtime_pump,
                receive_buffer,
                transmit_buffer,
                device,
                outputs,
            )
        }
        _ => Err(PosixError::InvalidArgument),
    }
}

fn process_local_ping_user_usize_arg(
    arguments: SyscallArguments,
    index: usize,
) -> Result<usize, PosixError> {
    process_local_ping_user_usize(arguments.get(index).ok_or(PosixError::InvalidArgument)?)
}

fn process_local_ping_user_usize(value: u64) -> Result<usize, PosixError> {
    usize::try_from(value).map_err(|_| PosixError::InvalidArgument)
}

fn process_local_ping_user_u16_arg(value: u64) -> Result<u16, PosixError> {
    u16::try_from(value).map_err(|_| PosixError::InvalidArgument)
}

fn require_process_local_ping_reserved(
    arguments: SyscallArguments,
    start: usize,
) -> Result<(), PosixError> {
    require_process_local_ping_reserved_from(arguments, start)
}

fn require_process_local_ping_reserved_from(
    arguments: SyscallArguments,
    start: usize,
) -> Result<(), PosixError> {
    let mut index = start;
    while index < MAX_SCALAR_ARGUMENTS {
        if arguments.get(index).ok_or(PosixError::InvalidArgument)? != 0 {
            return Err(PosixError::InvalidArgument);
        }
        index += 1;
    }
    Ok(())
}

fn process_local_ping_user_route_policy(
    packed_route: u64,
) -> Result<crate::network::Ipv4EgressRoutePolicy, PosixError> {
    let route_kind = ((packed_route >> 48) & 0xff) as u8;
    if route_kind != PROCESS_LOCAL_PING_USER_ROUTE_SAME_SUBNET {
        return Err(PosixError::InvalidArgument);
    }
    let prefix_len = ((packed_route >> 40) & 0xff) as u8;
    Ok(crate::network::Ipv4EgressRoutePolicy::new(
        process_local_ping_user_ipv4_mask(prefix_len)?,
        None,
    ))
}

fn process_local_ping_user_destination_ipv4(packed_route: u64) -> [u8; 4] {
    (packed_route as u32).to_be_bytes()
}

fn process_local_ping_user_ttl(packed_route: u64) -> Result<u8, PosixError> {
    let ttl = ((packed_route >> 32) & 0xff) as u8;
    if ttl == 0 {
        return Err(PosixError::InvalidArgument);
    }
    Ok(ttl)
}

fn process_local_ping_user_ipv4_mask(prefix_len: u8) -> Result<[u8; 4], PosixError> {
    if prefix_len > 32 {
        return Err(PosixError::InvalidArgument);
    }
    let mask = if prefix_len == 0 {
        0
    } else {
        u32::MAX << (32 - prefix_len)
    };
    Ok(mask.to_be_bytes())
}

fn process_local_ping_user_encode_step(
    step: &PingOperationSyscallSubstituteStep,
    dst: &mut [u8; PROCESS_LOCAL_PING_USER_STEP_RECORD_LEN],
) {
    process_local_ping_user_put_u64(dst, 0, process_local_ping_user_step_kind_code(step.kind()));
    process_local_ping_user_put_u64(dst, 8, step.frame_len() as u64);
    process_local_ping_user_put_u64(dst, 16, step.payload_len() as u64);
    dst[24..28].copy_from_slice(&step.destination_ipv4());
}

fn process_local_ping_user_encode_pump_step(
    pump_step: &RuntimePingOperationSyscallSubstitutePumpStep,
    dst: &mut [u8; PROCESS_LOCAL_PING_USER_PUMP_RECORD_LEN],
) {
    process_local_ping_user_put_u64(
        dst,
        0,
        process_local_ping_user_pump_kind_code(pump_step.kind()),
    );
    process_local_ping_user_put_u64(dst, 8, pump_step.descriptor() as u64);
    process_local_ping_user_put_u64(
        dst,
        16,
        process_local_ping_user_reply_kind_code(pump_step.local_reply_kind()),
    );
    process_local_ping_user_put_u64(dst, 24, pump_step.local_reply_frame_len() as u64);
    let mut active_step = [0u8; PROCESS_LOCAL_PING_USER_STEP_RECORD_LEN];
    process_local_ping_user_encode_step(&pump_step.active_ping_step(), &mut active_step);
    dst[32..].copy_from_slice(&active_step);
}

fn process_local_ping_user_encode_status(
    status: &PingOperationSyscallSubstituteStatus,
    dst: &mut [u8; PROCESS_LOCAL_PING_USER_STATUS_RECORD_LEN],
) {
    process_local_ping_user_put_u64(
        dst,
        0,
        process_local_ping_user_status_kind_code(status.kind()),
    );
    dst[8..12].copy_from_slice(&status.destination_ipv4());
    dst[16..20].copy_from_slice(&status.next_hop_ipv4());
    process_local_ping_user_put_u64(dst, 24, status.payload_len() as u64);
    process_local_ping_user_put_u64(dst, 32, status.arp_retries_remaining() as u64);
}

fn process_local_ping_user_step_kind_code(kind: PingOperationSyscallSubstituteStepKind) -> u64 {
    match kind {
        PingOperationSyscallSubstituteStepKind::StartedPendingArp => 1,
        PingOperationSyscallSubstituteStepKind::StartedInflight => 2,
        PingOperationSyscallSubstituteStepKind::NoFrame => 3,
        PingOperationSyscallSubstituteStepKind::AdvancedToInflight => 4,
        PingOperationSyscallSubstituteStepKind::RetryTransmitted => 5,
        PingOperationSyscallSubstituteStepKind::Completed => 6,
        PingOperationSyscallSubstituteStepKind::TimedOut => 7,
    }
}

fn process_local_ping_user_pump_kind_code(
    kind: RuntimePingOperationSyscallSubstitutePumpKind,
) -> u64 {
    match kind {
        RuntimePingOperationSyscallSubstitutePumpKind::NoFrame => 0,
        RuntimePingOperationSyscallSubstitutePumpKind::LocalNoReply => 1,
        RuntimePingOperationSyscallSubstitutePumpKind::LocalReply => 2,
        RuntimePingOperationSyscallSubstitutePumpKind::ActivePing => 3,
    }
}

fn process_local_ping_user_reply_kind_code(kind: Option<crate::network::PacketReplyKind>) -> u64 {
    match kind {
        None => 0,
        Some(crate::network::PacketReplyKind::Arp) => 1,
        Some(crate::network::PacketReplyKind::IcmpEcho) => 2,
    }
}

fn process_local_ping_user_status_kind_code(kind: PingOperationSyscallSubstituteStatusKind) -> u64 {
    match kind {
        PingOperationSyscallSubstituteStatusKind::Idle => 0,
        PingOperationSyscallSubstituteStatusKind::PendingArp => 1,
        PingOperationSyscallSubstituteStatusKind::Inflight => 2,
        PingOperationSyscallSubstituteStatusKind::Completed => 3,
        PingOperationSyscallSubstituteStatusKind::TimedOut => 4,
    }
}

fn process_local_ping_user_put_u64(dst: &mut [u8], offset: usize, value: u64) {
    dst[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
}

pub(crate) struct VfsPingDiagnosticSvcFixture<'a> {
    executable_path: &'static [u8],
    executable_len: usize,
    mappings: &'a [crate::posix::UserMapping],
    user_memory_start: u64,
    user_memory: &'a mut [u8],
    kernel_scratch: &'a mut [u8],
    payload_user_start: u64,
    pump_user_start: u64,
    status_user_start: u64,
}

impl<'a> VfsPingDiagnosticSvcFixture<'a> {
    pub(crate) fn new(
        initramfs: crate::initramfs::ReadOnlyInitramfs,
        executable_path: &'static [u8],
        mappings: &'a [crate::posix::UserMapping],
        user_memory_start: u64,
        user_memory: &'a mut [u8],
        kernel_scratch: &'a mut [u8],
        payload_user_start: u64,
        pump_user_start: u64,
        status_user_start: u64,
    ) -> Result<Self, PosixError> {
        let executable_len = initramfs.regular_file_bytes(executable_path)?.len();
        Ok(Self {
            executable_path,
            executable_len,
            mappings,
            user_memory_start,
            user_memory,
            kernel_scratch,
            payload_user_start,
            pump_user_start,
            status_user_start,
        })
    }

    pub(crate) const fn executable_path(&self) -> &'static [u8] {
        self.executable_path
    }

    pub(crate) const fn executable_len(&self) -> usize {
        self.executable_len
    }

    pub(crate) fn write_payload(&mut self, payload: &[u8]) -> Result<(), PosixError> {
        let range = self.user_range(self.payload_user_start, payload.len())?;
        self.user_memory[range].copy_from_slice(payload);
        Ok(())
    }

    pub(crate) fn read_user_u64(&self, user_start: u64) -> Result<u64, PosixError> {
        let range = self.user_range(user_start, 8)?;
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&self.user_memory[range]);
        Ok(u64::from_le_bytes(raw))
    }

    pub(crate) fn read_user_bytes(&self, user_start: u64, len: usize) -> Result<&[u8], PosixError> {
        let range = self.user_range(user_start, len)?;
        Ok(&self.user_memory[range])
    }

    pub(crate) fn open_arguments(&self) -> SyscallArguments {
        SyscallArguments::new([PROCESS_LOCAL_PING_USER_SELECTOR_OPEN, 0, 0, 0, 0, 0])
    }

    pub(crate) fn status_arguments(
        &self,
        process_descriptor: usize,
        output_len: usize,
    ) -> SyscallArguments {
        SyscallArguments::new([
            PROCESS_LOCAL_PING_USER_SELECTOR_STATUS,
            process_descriptor as u64,
            self.status_user_start,
            output_len as u64,
            0,
            0,
        ])
    }

    pub(crate) fn start_arguments(
        &self,
        process_descriptor: usize,
        payload_len: usize,
        destination_ipv4: [u8; 4],
        ttl: u8,
        prefix_len: u8,
        identifier: u16,
        sequence_number: u16,
        arp_retry_budget: u32,
    ) -> SyscallArguments {
        SyscallArguments::new([
            PROCESS_LOCAL_PING_USER_SELECTOR_START,
            process_descriptor as u64,
            self.payload_user_start,
            payload_len as u64,
            vfs_ping_diagnostic_user_route(destination_ipv4, ttl, prefix_len),
            vfs_ping_diagnostic_user_start_control(identifier, sequence_number, arp_retry_budget),
        ])
    }

    pub(crate) fn pump_or_read_result_arguments(
        &self,
        process_descriptor: usize,
        output_len: usize,
    ) -> SyscallArguments {
        SyscallArguments::new([
            PROCESS_LOCAL_PING_USER_SELECTOR_PUMP_OR_READ_RESULT,
            process_descriptor as u64,
            self.pump_user_start,
            output_len as u64,
            0,
            0,
        ])
    }

    pub(crate) fn retry_arp_arguments(&self, process_descriptor: usize) -> SyscallArguments {
        SyscallArguments::new([
            PROCESS_LOCAL_PING_USER_SELECTOR_RETRY_ARP,
            process_descriptor as u64,
            0,
            0,
            0,
            0,
        ])
    }

    pub(crate) fn timeout_arguments(&self, process_descriptor: usize) -> SyscallArguments {
        SyscallArguments::new([
            PROCESS_LOCAL_PING_USER_SELECTOR_TIMEOUT,
            process_descriptor as u64,
            0,
            0,
            0,
            0,
        ])
    }

    pub(crate) fn close_arguments(&self, process_descriptor: usize) -> SyscallArguments {
        SyscallArguments::new([
            PROCESS_LOCAL_PING_USER_SELECTOR_CLOSE,
            process_descriptor as u64,
            0,
            0,
            0,
            0,
        ])
    }

    pub(crate) fn dispatch<
        const OWNER_CAPACITY: usize,
        const PROCESS_DESCRIPTOR_CAPACITY: usize,
        const LOCAL_ARP_CAPACITY: usize,
        const PING_DESCRIPTOR_CAPACITY: usize,
        const OPERATION_ARP_CAPACITY: usize,
        const PAYLOAD_CAPACITY: usize,
        D,
    >(
        &mut self,
        arguments: SyscallArguments,
        current_owner: Option<crate::scheduler::ProcessOwnerId>,
        descriptor_store: &mut crate::posix::ProcessDescriptorStore<
            OWNER_CAPACITY,
            PROCESS_DESCRIPTOR_CAPACITY,
        >,
        runtime_pump: &mut crate::network::NetworkRuntimeDevicePump<
            LOCAL_ARP_CAPACITY,
            PING_DESCRIPTOR_CAPACITY,
            OPERATION_ARP_CAPACITY,
            PAYLOAD_CAPACITY,
        >,
        receive_buffer: &mut [u8],
        transmit_buffer: &mut [u8],
        device: &mut D,
        outputs: &mut ProcessLocalPingDispatchOutputs<'_>,
    ) -> Result<ProcessLocalPingDispatchOutcome, PosixError>
    where
        D: crate::network::NetworkDevice,
    {
        dispatch_process_local_ping_descriptor_user_arguments(
            arguments,
            current_owner,
            descriptor_store,
            self.mappings,
            self.user_memory_start,
            self.user_memory,
            self.kernel_scratch,
            runtime_pump,
            receive_buffer,
            transmit_buffer,
            device,
            outputs,
        )
    }

    fn user_range(
        &self,
        user_start: u64,
        len: usize,
    ) -> Result<core::ops::Range<usize>, PosixError> {
        let offset = user_start
            .checked_sub(self.user_memory_start)
            .ok_or(PosixError::Fault)?;
        let offset = usize::try_from(offset).map_err(|_| PosixError::Fault)?;
        let end = offset.checked_add(len).ok_or(PosixError::Fault)?;
        if end > self.user_memory.len() {
            return Err(PosixError::Fault);
        }
        Ok(offset..end)
    }
}

const fn vfs_ping_diagnostic_user_route(destination_ipv4: [u8; 4], ttl: u8, prefix_len: u8) -> u64 {
    u32::from_be_bytes(destination_ipv4) as u64 | ((ttl as u64) << 32) | ((prefix_len as u64) << 40)
}

const fn vfs_ping_diagnostic_user_start_control(
    identifier: u16,
    sequence_number: u16,
    arp_retry_budget: u32,
) -> u64 {
    identifier as u64 | ((sequence_number as u64) << 16) | ((arp_retry_budget as u64) << 32)
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

fn dispatch_talos_read<
    const OWNER_CAPACITY: usize,
    const DESCRIPTOR_CAPACITY: usize,
    const FILE_CAPACITY: usize,
    I,
>(
    arguments: SyscallArguments,
    current_owner: Option<crate::scheduler::ProcessOwnerId>,
    descriptor_store: &mut crate::posix::ProcessDescriptorStore<
        OWNER_CAPACITY,
        DESCRIPTOR_CAPACITY,
    >,
    mappings: &[crate::posix::UserMapping],
    user_memory_start: u64,
    user_memory: &mut [u8],
    kernel_scratch: &mut [u8],
    fixed_stdin: Option<&mut crate::posix::FixedStdin<'_>>,
    console_stdin: Option<&mut I>,
    initramfs: Option<crate::initramfs::ReadOnlyInitramfs>,
    file_descriptions: Option<&mut crate::initramfs::ReadOnlyFileDescriptions<FILE_CAPACITY>>,
) -> SyscallReturn
where
    I: crate::runtime_console::ConsoleInputBackend,
{
    let [descriptor, user_start, len, reserved0, reserved1, reserved2] = arguments.values();
    if reserved0 != 0 || reserved1 != 0 || reserved2 != 0 {
        return SyscallReturn::error(PosixError::InvalidArgument);
    }
    if len > crate::posix::DEFAULT_USER_COPY_LIMIT as u64 {
        return SyscallReturn::error(PosixError::Fault);
    }

    let Ok(descriptor) = usize::try_from(descriptor) else {
        return SyscallReturn::error(PosixError::BadDescriptor);
    };
    let Ok(len) = usize::try_from(len) else {
        return SyscallReturn::error(PosixError::Fault);
    };
    let descriptor_table = match descriptor_store.current_descriptor_table(current_owner) {
        Ok(descriptor_table) => descriptor_table,
        Err(error) => return SyscallReturn::error(error),
    };

    let entry = match descriptor_table.get(descriptor) {
        Ok(entry) => entry,
        Err(error) => return SyscallReturn::error(error),
    };
    if let Err(error) = entry.require_readable() {
        return SyscallReturn::error(error);
    }

    let read_result = match entry.object().kind() {
        crate::posix::DescriptorObjectKind::StdioInput => match fixed_stdin {
            Some(stdin) => crate::posix::read_descriptor_from_fixed_stdin(
                descriptor_table,
                descriptor,
                mappings,
                user_memory_start,
                user_memory,
                user_start,
                len,
                kernel_scratch,
                Some(stdin),
            ),
            None => crate::posix::read_descriptor_from_console_input(
                descriptor_table,
                descriptor,
                mappings,
                user_memory_start,
                user_memory,
                user_start,
                len,
                kernel_scratch,
                console_stdin,
            ),
        },
        crate::posix::DescriptorObjectKind::RegularFile => match (initramfs, file_descriptions) {
            (Some(fs), Some(file_descriptions)) => fs.read_descriptor(
                descriptor_table,
                file_descriptions,
                descriptor,
                mappings,
                user_memory_start,
                user_memory,
                user_start,
                len,
                kernel_scratch,
            ),
            _ => Err(PosixError::NotSupported),
        },
        crate::posix::DescriptorObjectKind::Device if entry.object().is_dev_null() => Ok(0),
        crate::posix::DescriptorObjectKind::Directory => Err(PosixError::IsDirectory),
        _ => Err(PosixError::NotSupported),
    };

    match read_result {
        Ok(bytes_read) => SyscallReturn::success(bytes_read as u64),
        Err(error) => SyscallReturn::error(error),
    }
}

fn dispatch_talos_open_initramfs<
    const OWNER_CAPACITY: usize,
    const DESCRIPTOR_CAPACITY: usize,
    const FILE_CAPACITY: usize,
>(
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
    initramfs: crate::initramfs::ReadOnlyInitramfs,
    file_descriptions: &mut crate::initramfs::ReadOnlyFileDescriptions<FILE_CAPACITY>,
) -> SyscallReturn {
    let [path_start, path_len, flags, reserved0, reserved1, reserved2] = arguments.values();
    if flags != 0 || reserved0 != 0 || reserved1 != 0 || reserved2 != 0 {
        return SyscallReturn::error(PosixError::InvalidArgument);
    }
    if path_len > crate::posix::DEFAULT_PATH_LIMITS.max_path_len as u64 {
        return SyscallReturn::error(PosixError::NameTooLong);
    }

    let Ok(path_len) = usize::try_from(path_len) else {
        return SyscallReturn::error(PosixError::NameTooLong);
    };
    if kernel_scratch.len() < path_len {
        return SyscallReturn::error(PosixError::InvalidArgument);
    }

    let descriptor_table = match descriptor_store.current_descriptor_table_mut(current_owner) {
        Ok(descriptor_table) => descriptor_table,
        Err(error) => return SyscallReturn::error(error),
    };
    let path = &mut kernel_scratch[..path_len];
    let open_result = crate::posix::copy_from_user(
        mappings,
        user_memory_start,
        user_memory,
        path_start,
        path_len,
        path,
    )
    .and_then(|_| initramfs.open_regular_descriptor(descriptor_table, file_descriptions, path));

    match open_result {
        Ok(descriptor) => SyscallReturn::success(descriptor as u64),
        Err(error) => SyscallReturn::error(error),
    }
}

fn dispatch_talos_socket<
    const OWNER_CAPACITY: usize,
    const DESCRIPTOR_CAPACITY: usize,
    const SOCKET_CAPACITY: usize,
>(
    arguments: SyscallArguments,
    current_owner: Option<crate::scheduler::ProcessOwnerId>,
    descriptor_store: &mut crate::posix::ProcessDescriptorStore<
        OWNER_CAPACITY,
        DESCRIPTOR_CAPACITY,
    >,
    socket_table: &mut crate::network::NetworkSocketDescriptorTable<SOCKET_CAPACITY>,
) -> SyscallReturn {
    let [
        domain,
        socket_type,
        protocol,
        reserved0,
        reserved1,
        reserved2,
    ] = arguments.values();
    if reserved0 != 0 || reserved1 != 0 || reserved2 != 0 {
        return SyscallReturn::error(PosixError::InvalidArgument);
    }

    let owner = match current_owner {
        Some(owner) => owner,
        None => return SyscallReturn::error(PosixError::BadDescriptor),
    };
    let descriptor_table = match descriptor_store.current_descriptor_table_mut(current_owner) {
        Ok(descriptor_table) => descriptor_table,
        Err(error) => return SyscallReturn::error(error),
    };
    if !descriptor_table.has_free_slot() {
        return SyscallReturn::error(PosixError::TooManyOpenFiles);
    }

    let socket_descriptor = match socket_table.open(owner, domain, socket_type, protocol) {
        Ok(descriptor) => descriptor,
        Err(error) => return SyscallReturn::error(error),
    };
    let entry = crate::posix::DescriptorEntry::new(
        crate::posix::DescriptorAccess::ReadWrite,
        crate::posix::DescriptorFlags::EMPTY,
        crate::posix::DescriptorObject::new(
            crate::posix::DescriptorObjectKind::Socket,
            socket_descriptor.raw(),
        ),
    );

    match descriptor_table.allocate(entry) {
        Ok(process_descriptor) => SyscallReturn::success(process_descriptor as u64),
        Err(error) => {
            let _ = socket_table.close(owner, socket_descriptor);
            SyscallReturn::error(error)
        }
    }
}

fn current_socket_descriptor<
    const OWNER_CAPACITY: usize,
    const DESCRIPTOR_CAPACITY: usize,
    const SOCKET_CAPACITY: usize,
>(
    current_owner: Option<crate::scheduler::ProcessOwnerId>,
    descriptor_store: &crate::posix::ProcessDescriptorStore<OWNER_CAPACITY, DESCRIPTOR_CAPACITY>,
    socket_table: &crate::network::NetworkSocketDescriptorTable<SOCKET_CAPACITY>,
    process_descriptor: usize,
) -> Result<
    (
        crate::scheduler::ProcessOwnerId,
        crate::network::NetworkSocketDescriptor,
    ),
    PosixError,
> {
    let owner = current_owner.ok_or(PosixError::BadDescriptor)?;
    let entry = descriptor_store
        .current_descriptor_table(current_owner)
        .and_then(|table| table.get(process_descriptor))?;
    if entry.object().kind() != crate::posix::DescriptorObjectKind::Socket {
        return Err(PosixError::BadDescriptor);
    }

    let socket_descriptor =
        crate::network::NetworkSocketDescriptor::from_raw(entry.object().reference());
    socket_table.require_owner(owner, socket_descriptor)?;
    Ok((owner, socket_descriptor))
}

fn dispatch_talos_bind<
    const OWNER_CAPACITY: usize,
    const DESCRIPTOR_CAPACITY: usize,
    const SOCKET_CAPACITY: usize,
>(
    arguments: SyscallArguments,
    current_owner: Option<crate::scheduler::ProcessOwnerId>,
    descriptor_store: &mut crate::posix::ProcessDescriptorStore<
        OWNER_CAPACITY,
        DESCRIPTOR_CAPACITY,
    >,
    socket_table: &mut crate::network::NetworkSocketDescriptorTable<SOCKET_CAPACITY>,
) -> SyscallReturn {
    let [descriptor, ipv4_be, port, reserved0, reserved1, reserved2] = arguments.values();
    if reserved0 != 0 || reserved1 != 0 || reserved2 != 0 {
        return SyscallReturn::error(PosixError::InvalidArgument);
    }
    if ipv4_be > u32::MAX as u64 || port == 0 || port > u16::MAX as u64 {
        return SyscallReturn::error(PosixError::InvalidArgument);
    }
    let Ok(process_descriptor) = usize::try_from(descriptor) else {
        return SyscallReturn::error(PosixError::BadDescriptor);
    };
    let (owner, socket_descriptor) = match current_socket_descriptor(
        current_owner,
        descriptor_store,
        socket_table,
        process_descriptor,
    ) {
        Ok(socket) => socket,
        Err(error) => return SyscallReturn::error(error),
    };

    match socket_table.bind(
        owner,
        socket_descriptor,
        crate::network::Ipv4Endpoint::new(ipv4_be as u32, port as u16),
    ) {
        Ok(()) => SyscallReturn::success(0),
        Err(error) => SyscallReturn::error(error),
    }
}

fn dispatch_talos_listen<
    const OWNER_CAPACITY: usize,
    const DESCRIPTOR_CAPACITY: usize,
    const SOCKET_CAPACITY: usize,
>(
    arguments: SyscallArguments,
    current_owner: Option<crate::scheduler::ProcessOwnerId>,
    descriptor_store: &mut crate::posix::ProcessDescriptorStore<
        OWNER_CAPACITY,
        DESCRIPTOR_CAPACITY,
    >,
    socket_table: &mut crate::network::NetworkSocketDescriptorTable<SOCKET_CAPACITY>,
) -> SyscallReturn {
    let [
        descriptor,
        backlog,
        reserved0,
        reserved1,
        reserved2,
        reserved3,
    ] = arguments.values();
    if reserved0 != 0 || reserved1 != 0 || reserved2 != 0 || reserved3 != 0 {
        return SyscallReturn::error(PosixError::InvalidArgument);
    }
    if !(crate::network::SOCKET_LISTEN_BACKLOG_MIN..=crate::network::SOCKET_LISTEN_BACKLOG_MAX)
        .contains(&backlog)
    {
        return SyscallReturn::error(PosixError::InvalidArgument);
    }
    let Ok(process_descriptor) = usize::try_from(descriptor) else {
        return SyscallReturn::error(PosixError::BadDescriptor);
    };
    let (owner, socket_descriptor) = match current_socket_descriptor(
        current_owner,
        descriptor_store,
        socket_table,
        process_descriptor,
    ) {
        Ok(socket) => socket,
        Err(error) => return SyscallReturn::error(error),
    };

    match socket_table.listen(owner, socket_descriptor, backlog as u8) {
        Ok(()) => SyscallReturn::success(0),
        Err(error) => SyscallReturn::error(error),
    }
}

fn dispatch_talos_connect<
    const OWNER_CAPACITY: usize,
    const DESCRIPTOR_CAPACITY: usize,
    const SOCKET_CAPACITY: usize,
>(
    arguments: SyscallArguments,
    current_owner: Option<crate::scheduler::ProcessOwnerId>,
    descriptor_store: &mut crate::posix::ProcessDescriptorStore<
        OWNER_CAPACITY,
        DESCRIPTOR_CAPACITY,
    >,
    socket_table: &mut crate::network::NetworkSocketDescriptorTable<SOCKET_CAPACITY>,
) -> SyscallReturn {
    let [descriptor, ipv4_be, port, reserved0, reserved1, reserved2] = arguments.values();
    if reserved0 != 0 || reserved1 != 0 || reserved2 != 0 {
        return SyscallReturn::error(PosixError::InvalidArgument);
    }
    if ipv4_be > u32::MAX as u64 || port == 0 || port > u16::MAX as u64 {
        return SyscallReturn::error(PosixError::InvalidArgument);
    }
    let Ok(process_descriptor) = usize::try_from(descriptor) else {
        return SyscallReturn::error(PosixError::BadDescriptor);
    };
    let (owner, socket_descriptor) = match current_socket_descriptor(
        current_owner,
        descriptor_store,
        socket_table,
        process_descriptor,
    ) {
        Ok(socket) => socket,
        Err(error) => return SyscallReturn::error(error),
    };

    match socket_table.connect(
        owner,
        socket_descriptor,
        crate::network::Ipv4Endpoint::new(ipv4_be as u32, port as u16),
    ) {
        Ok(()) => SyscallReturn::success(0),
        Err(error) => SyscallReturn::error(error),
    }
}

fn dispatch_talos_accept<
    const OWNER_CAPACITY: usize,
    const DESCRIPTOR_CAPACITY: usize,
    const SOCKET_CAPACITY: usize,
>(
    arguments: SyscallArguments,
    current_owner: Option<crate::scheduler::ProcessOwnerId>,
    descriptor_store: &mut crate::posix::ProcessDescriptorStore<
        OWNER_CAPACITY,
        DESCRIPTOR_CAPACITY,
    >,
    socket_table: &mut crate::network::NetworkSocketDescriptorTable<SOCKET_CAPACITY>,
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
    let Ok(process_descriptor) = usize::try_from(descriptor) else {
        return SyscallReturn::error(PosixError::BadDescriptor);
    };
    let (owner, listener_descriptor) = match current_socket_descriptor(
        current_owner,
        descriptor_store,
        socket_table,
        process_descriptor,
    ) {
        Ok(socket) => socket,
        Err(error) => return SyscallReturn::error(error),
    };
    let descriptor_table = match descriptor_store.current_descriptor_table(current_owner) {
        Ok(descriptor_table) => descriptor_table,
        Err(error) => return SyscallReturn::error(error),
    };
    if !descriptor_table.has_free_slot() {
        return SyscallReturn::error(PosixError::TooManyOpenFiles);
    }

    let accepted_socket_descriptor = match socket_table.accept(owner, listener_descriptor) {
        Ok(descriptor) => descriptor,
        Err(error) => return SyscallReturn::error(error),
    };
    let entry = crate::posix::DescriptorEntry::new(
        crate::posix::DescriptorAccess::ReadWrite,
        crate::posix::DescriptorFlags::EMPTY,
        crate::posix::DescriptorObject::new(
            crate::posix::DescriptorObjectKind::Socket,
            accepted_socket_descriptor.raw(),
        ),
    );
    match descriptor_store
        .current_descriptor_table_mut(current_owner)
        .and_then(|descriptor_table| descriptor_table.allocate(entry))
    {
        Ok(process_descriptor) => SyscallReturn::success(process_descriptor as u64),
        Err(error) => {
            let _ = socket_table.close(owner, accepted_socket_descriptor);
            SyscallReturn::error(error)
        }
    }
}

fn dispatch_talos_send<
    const OWNER_CAPACITY: usize,
    const DESCRIPTOR_CAPACITY: usize,
    const SOCKET_CAPACITY: usize,
>(
    arguments: SyscallArguments,
    current_owner: Option<crate::scheduler::ProcessOwnerId>,
    descriptor_store: &mut crate::posix::ProcessDescriptorStore<
        OWNER_CAPACITY,
        DESCRIPTOR_CAPACITY,
    >,
    socket_table: &mut crate::network::NetworkSocketDescriptorTable<SOCKET_CAPACITY>,
    mappings: &[crate::posix::UserMapping],
    user_memory_start: u64,
    user_memory: &[u8],
    kernel_scratch: &mut [u8],
) -> SyscallReturn {
    let [descriptor, buffer_start, len, flags, reserved0, reserved1] = arguments.values();
    if flags != 0 || reserved0 != 0 || reserved1 != 0 {
        return SyscallReturn::error(PosixError::InvalidArgument);
    }
    let Ok(len) = usize::try_from(len) else {
        return SyscallReturn::error(PosixError::NoSpace);
    };
    if len > crate::network::SOCKET_PAYLOAD_QUEUE_CAPACITY {
        return SyscallReturn::error(PosixError::NoSpace);
    }
    if kernel_scratch.len() < len {
        return SyscallReturn::error(PosixError::InvalidArgument);
    }
    let Ok(process_descriptor) = usize::try_from(descriptor) else {
        return SyscallReturn::error(PosixError::BadDescriptor);
    };
    let (owner, socket_descriptor) = match current_socket_descriptor(
        current_owner,
        descriptor_store,
        socket_table,
        process_descriptor,
    ) {
        Ok(socket) => socket,
        Err(error) => return SyscallReturn::error(error),
    };
    if let Err(error) = socket_table.send_ready(owner, socket_descriptor, len) {
        return SyscallReturn::error(error);
    }
    if len == 0 {
        return SyscallReturn::success(0);
    }
    if let Err(error) = crate::posix::copy_from_user(
        mappings,
        user_memory_start,
        user_memory,
        buffer_start,
        len,
        &mut kernel_scratch[..len],
    ) {
        return SyscallReturn::error(error);
    }

    match socket_table.send(owner, socket_descriptor, &kernel_scratch[..len]) {
        Ok(sent) => SyscallReturn::success(sent as u64),
        Err(error) => SyscallReturn::error(error),
    }
}

fn dispatch_talos_recv<
    const OWNER_CAPACITY: usize,
    const DESCRIPTOR_CAPACITY: usize,
    const SOCKET_CAPACITY: usize,
>(
    arguments: SyscallArguments,
    current_owner: Option<crate::scheduler::ProcessOwnerId>,
    descriptor_store: &mut crate::posix::ProcessDescriptorStore<
        OWNER_CAPACITY,
        DESCRIPTOR_CAPACITY,
    >,
    socket_table: &mut crate::network::NetworkSocketDescriptorTable<SOCKET_CAPACITY>,
    mappings: &[crate::posix::UserMapping],
    user_memory_start: u64,
    user_memory: &mut [u8],
    kernel_scratch: &mut [u8],
) -> SyscallReturn {
    let [descriptor, buffer_start, len, flags, reserved0, reserved1] = arguments.values();
    if flags != 0 || reserved0 != 0 || reserved1 != 0 {
        return SyscallReturn::error(PosixError::InvalidArgument);
    }
    let Ok(len) = usize::try_from(len) else {
        return SyscallReturn::error(PosixError::InvalidArgument);
    };
    let Ok(process_descriptor) = usize::try_from(descriptor) else {
        return SyscallReturn::error(PosixError::BadDescriptor);
    };
    let (owner, socket_descriptor) = match current_socket_descriptor(
        current_owner,
        descriptor_store,
        socket_table,
        process_descriptor,
    ) {
        Ok(socket) => socket,
        Err(error) => return SyscallReturn::error(error),
    };
    let peek_len = core::cmp::min(len, kernel_scratch.len());
    let received =
        match socket_table.recv_peek(owner, socket_descriptor, &mut kernel_scratch[..peek_len]) {
            Ok(received) => received,
            Err(error) => return SyscallReturn::error(error),
        };
    if received == 0 {
        return SyscallReturn::success(0);
    }
    if let Err(error) = crate::posix::copy_to_user(
        mappings,
        user_memory_start,
        user_memory,
        buffer_start,
        received,
        &kernel_scratch[..received],
    ) {
        return SyscallReturn::error(error);
    }
    match socket_table.recv_commit(owner, socket_descriptor, received) {
        Ok(()) => SyscallReturn::success(received as u64),
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

    #[cfg(talos_boot_scenario = "rpi5_close_syscall_proof")]
    {
        let owner_raw = current_owner.map(|owner| owner.raw()).unwrap_or(0);
        match descriptor_store.current_descriptor_table(current_owner) {
            Ok(table) => {
                crate::println!(
                    "rpi5-close-syscall-proof: talos-close-entry owner={:#018x} descriptor={} owner-present=true fd-open={} stdout-open={} stderr-open={}",
                    owner_raw,
                    descriptor,
                    table.get(descriptor).is_ok(),
                    table.get(crate::posix::STDOUT_FD).is_ok(),
                    table.get(crate::posix::STDERR_FD).is_ok()
                );
            }
            Err(error) => {
                crate::println!(
                    "rpi5-close-syscall-proof: talos-close-entry owner={:#018x} descriptor={} owner-present=false table-error={}",
                    owner_raw,
                    descriptor,
                    error.name()
                );
            }
        }
    }

    let close_result = descriptor_store.close_current_descriptor(current_owner, descriptor);

    #[cfg(talos_boot_scenario = "rpi5_close_syscall_proof")]
    {
        let owner_raw = current_owner.map(|owner| owner.raw()).unwrap_or(0);
        let result_name = match close_result {
            Ok(_) => "OK",
            Err(error) => error.name(),
        };
        match descriptor_store.current_descriptor_table(current_owner) {
            Ok(table) => {
                crate::println!(
                    "rpi5-close-syscall-proof: talos-close-result owner={:#018x} descriptor={} result={} fd-open-after={} stdout-open-after={} stderr-open-after={}",
                    owner_raw,
                    descriptor,
                    result_name,
                    table.get(descriptor).is_ok(),
                    table.get(crate::posix::STDOUT_FD).is_ok(),
                    table.get(crate::posix::STDERR_FD).is_ok()
                );
            }
            Err(error) => {
                crate::println!(
                    "rpi5-close-syscall-proof: talos-close-result owner={:#018x} descriptor={} result={} owner-present-after=false table-error-after={}",
                    owner_raw,
                    descriptor,
                    result_name,
                    error.name()
                );
            }
        }
    }

    match close_result {
        Ok(_) => SyscallReturn::success(0),
        Err(error) => SyscallReturn::error(error),
    }
}

fn dispatch_talos_close_socket_aware<
    const OWNER_CAPACITY: usize,
    const DESCRIPTOR_CAPACITY: usize,
    const SOCKET_CAPACITY: usize,
>(
    arguments: SyscallArguments,
    current_owner: Option<crate::scheduler::ProcessOwnerId>,
    descriptor_store: &mut crate::posix::ProcessDescriptorStore<
        OWNER_CAPACITY,
        DESCRIPTOR_CAPACITY,
    >,
    socket_table: &mut crate::network::NetworkSocketDescriptorTable<SOCKET_CAPACITY>,
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
    let owner = match current_owner {
        Some(owner) => owner,
        None => return SyscallReturn::error(PosixError::BadDescriptor),
    };
    let entry = match descriptor_store
        .current_descriptor_table(current_owner)
        .and_then(|table| table.get(descriptor))
    {
        Ok(entry) => entry,
        Err(error) => return SyscallReturn::error(error),
    };

    if entry.object().kind() != crate::posix::DescriptorObjectKind::Socket {
        return dispatch_talos_close(arguments, current_owner, descriptor_store);
    }

    let socket_descriptor =
        crate::network::NetworkSocketDescriptor::from_raw(entry.object().reference());
    if let Err(error) = socket_table.require_owner(owner, socket_descriptor) {
        return SyscallReturn::error(error);
    }
    match descriptor_store.close_current_descriptor(current_owner, descriptor) {
        Ok(_) => match socket_table.close(owner, socket_descriptor) {
            Ok(()) => SyscallReturn::success(0),
            Err(error) => SyscallReturn::error(error),
        },
        Err(error) => SyscallReturn::error(error),
    }
}

fn dispatch_talos_dup<const OWNER_CAPACITY: usize, const DESCRIPTOR_CAPACITY: usize>(
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

    match descriptor_store.dup_current_descriptor(current_owner, descriptor) {
        Ok(duplicate) => SyscallReturn::success(duplicate as u64),
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
    fn descriptor_read_number_requires_context_in_scalar_dispatch() {
        let result = dispatch(TALOS_READ_SYSCALL, SyscallArguments::empty());

        assert_eq!(result.number(), SyscallNumber::TalosRead);
        assert_eq!(result.number().raw(), TALOS_READ_SYSCALL);
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
    fn descriptor_dup_number_requires_context_in_scalar_dispatch() {
        let result = dispatch(TALOS_DUP_SYSCALL, SyscallArguments::empty());

        assert_eq!(result.number(), SyscallNumber::TalosDup);
        assert_eq!(result.number().raw(), TALOS_DUP_SYSCALL);
        assert_eq!(result.return_value().x0(), (ENOTSUP as u64).wrapping_neg());
    }

    #[test_case]
    fn descriptor_open_number_requires_initramfs_context_in_scalar_dispatch() {
        let result = dispatch(TALOS_OPEN_SYSCALL, SyscallArguments::empty());

        assert_eq!(result.number(), SyscallNumber::TalosOpen);
        assert_eq!(result.number().raw(), TALOS_OPEN_SYSCALL);
        assert_eq!(result.return_value().x0(), (ENOTSUP as u64).wrapping_neg());
    }

    #[test_case]
    fn socket_number_requires_socket_table_context_in_scalar_dispatch() {
        let result = dispatch(TALOS_SOCKET_SYSCALL, SyscallArguments::empty());

        assert_eq!(result.number(), SyscallNumber::TalosSocket);
        assert_eq!(result.number().raw(), TALOS_SOCKET_SYSCALL);
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
    fn posix_error_vocabulary_encodes_as_negative_errno_x0_values() {
        let accepted = [
            (PosixError::OperationNotPermitted, EPERM),
            (PosixError::NoEntry, ENOENT),
            (PosixError::Interrupted, EINTR),
            (PosixError::Io, EIO),
            (PosixError::NotExecutable, ENOEXEC),
            (PosixError::BadDescriptor, EBADF),
            (PosixError::NoChild, ECHILD),
            (PosixError::Again, EAGAIN),
            (PosixError::NoMemory, ENOMEM),
            (PosixError::AccessDenied, EACCES),
            (PosixError::Fault, EFAULT),
            (PosixError::Busy, EBUSY),
            (PosixError::Exists, EEXIST),
            (PosixError::NoDevice, ENODEV),
            (PosixError::NotDirectory, ENOTDIR),
            (PosixError::IsDirectory, EISDIR),
            (PosixError::InvalidArgument, EINVAL),
            (PosixError::TooManyOpenFiles, EMFILE),
            (PosixError::NotTty, ENOTTY),
            (PosixError::NoSpace, ENOSPC),
            (PosixError::Pipe, EPIPE),
            (PosixError::Range, ERANGE),
            (PosixError::NameTooLong, ENAMETOOLONG),
            (PosixError::NotImplemented, ENOSYS),
            (PosixError::NotEmpty, ENOTEMPTY),
            (PosixError::NotSupported, ENOTSUP),
        ];

        let mut index = 0;
        while index < accepted.len() {
            let (error, errno) = accepted[index];
            assert_eq!(errno_number(error), errno);
            assert_eq!(
                SyscallReturn::error(error).x0(),
                (errno as u64).wrapping_neg()
            );
            index += 1;
        }
    }

    #[test_case]
    fn vfs_path_errors_do_not_collapse_to_enosys() {
        assert_eq!(
            SyscallReturn::error(PosixError::NoEntry).x0(),
            (ENOENT as u64).wrapping_neg()
        );
        assert_eq!(
            SyscallReturn::error(PosixError::IsDirectory).x0(),
            (EISDIR as u64).wrapping_neg()
        );
        assert_eq!(
            SyscallReturn::error(PosixError::NotDirectory).x0(),
            (ENOTDIR as u64).wrapping_neg()
        );
        assert_eq!(
            SyscallReturn::error(PosixError::NameTooLong).x0(),
            (ENAMETOOLONG as u64).wrapping_neg()
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

    fn dispatch_read_case(
        descriptor: u64,
        user_start: u64,
        len: u64,
        reserved0: u64,
        current_owner: Option<crate::scheduler::ProcessOwnerId>,
        store: &mut crate::posix::ProcessDescriptorStore<2, 4>,
        user_memory: &mut [u8; 128],
        fixed_stdin: Option<&mut crate::posix::FixedStdin<'_>>,
    ) -> SyscallDispatchResult {
        let mappings = [crate::posix::UserMapping::new(
            0x0000_0000_0011_0000,
            0x80,
            crate::posix::UserMappingPermissions::USER_DATA,
        )
        .expect("user data mapping")];
        let mut scratch = [0u8; 64];
        let mut console = CaptureConsole::new();
        dispatch_process_descriptor_with_fixed_stdin(
            TALOS_READ_SYSCALL,
            SyscallArguments::new([descriptor, user_start, len, reserved0, 0, 0]),
            current_owner,
            store,
            &mappings,
            0x0000_0000_0011_0000,
            user_memory,
            &mut scratch,
            &mut console,
            fixed_stdin,
        )
    }

    fn dispatch_initramfs_case<const FILE_CAPACITY: usize>(
        raw_number: u64,
        arguments: SyscallArguments,
        store: &mut crate::posix::ProcessDescriptorStore<2, 5>,
        files: &mut crate::initramfs::ReadOnlyFileDescriptions<FILE_CAPACITY>,
        user_memory: &mut [u8; 128],
    ) -> SyscallDispatchResult {
        let owner = crate::scheduler::ProcessOwnerId::new(31).expect("owner id");
        let mappings = [crate::posix::UserMapping::new(
            0x0000_0000_0011_0000,
            0x80,
            crate::posix::UserMappingPermissions::USER_DATA,
        )
        .expect("user data mapping")];
        let mut scratch = [0u8; 128];
        let mut console = CaptureConsole::new();
        dispatch_process_descriptor_with_initramfs(
            raw_number,
            arguments,
            Some(owner),
            store,
            &mappings,
            0x0000_0000_0011_0000,
            user_memory,
            &mut scratch,
            &mut console,
            crate::initramfs::phase8_readonly_initramfs_fixture(),
            files,
            None,
        )
    }

    fn dispatch_socket_case<
        const OWNER_CAPACITY: usize,
        const DESCRIPTOR_CAPACITY: usize,
        const SOCKET_CAPACITY: usize,
    >(
        raw_number: u64,
        arguments: SyscallArguments,
        current_owner: Option<crate::scheduler::ProcessOwnerId>,
        store: &mut crate::posix::ProcessDescriptorStore<OWNER_CAPACITY, DESCRIPTOR_CAPACITY>,
        sockets: &mut crate::network::NetworkSocketDescriptorTable<SOCKET_CAPACITY>,
        user_memory: &mut [u8; 128],
    ) -> SyscallDispatchResult {
        let mappings = [crate::posix::UserMapping::new(
            0x0000_0000_0011_0000,
            0x80,
            crate::posix::UserMappingPermissions::USER_DATA,
        )
        .expect("user data mapping")];
        let mut scratch = [0u8; 64];
        let mut console = CaptureConsole::new();
        dispatch_process_descriptor_with_socket_table(
            raw_number,
            arguments,
            current_owner,
            store,
            sockets,
            &mappings,
            0x0000_0000_0011_0000,
            user_memory,
            &mut scratch,
            &mut console,
        )
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

    struct PingOutboundTransmitDevice {
        transmit_error: Option<crate::network::DeviceError>,
        transmitted: [u8; 128],
        transmitted_len: usize,
    }

    impl PingOutboundTransmitDevice {
        const fn new() -> Self {
            Self {
                transmit_error: None,
                transmitted: [0; 128],
                transmitted_len: 0,
            }
        }

        const fn with_transmit_error(error: crate::network::DeviceError) -> Self {
            Self {
                transmit_error: Some(error),
                transmitted: [0; 128],
                transmitted_len: 0,
            }
        }
    }

    impl crate::network::NetworkDevice for PingOutboundTransmitDevice {
        fn receive_frame<'a>(
            &mut self,
            _buffer: &'a mut [u8],
        ) -> Result<&'a [u8], crate::network::DeviceError> {
            Err(crate::network::DeviceError::WouldBlock)
        }

        fn transmit_frame(&mut self, frame: &[u8]) -> Result<(), crate::network::DeviceError> {
            if let Some(error) = self.transmit_error {
                return Err(error);
            }

            self.transmitted[..frame.len()].copy_from_slice(frame);
            self.transmitted_len = frame.len();
            Ok(())
        }
    }

    struct PingPollDevice<'a> {
        frame: Option<&'a [u8]>,
        receive_error: Option<crate::network::DeviceError>,
        transmit_error: Option<crate::network::DeviceError>,
        transmitted: [u8; 128],
        transmitted_len: usize,
    }

    impl<'a> PingPollDevice<'a> {
        fn with_frame(frame: &'a [u8]) -> Self {
            Self {
                frame: Some(frame),
                receive_error: None,
                transmit_error: None,
                transmitted: [0; 128],
                transmitted_len: 0,
            }
        }

        fn with_receive_error(error: crate::network::DeviceError) -> Self {
            Self {
                frame: None,
                receive_error: Some(error),
                transmit_error: None,
                transmitted: [0; 128],
                transmitted_len: 0,
            }
        }

        fn with_transmit_error(frame: &'a [u8], error: crate::network::DeviceError) -> Self {
            Self {
                frame: Some(frame),
                receive_error: None,
                transmit_error: Some(error),
                transmitted: [0; 128],
                transmitted_len: 0,
            }
        }
    }

    impl<'a> crate::network::NetworkDevice for PingPollDevice<'a> {
        fn receive_frame<'b>(
            &mut self,
            buffer: &'b mut [u8],
        ) -> Result<&'b [u8], crate::network::DeviceError> {
            if let Some(error) = self.receive_error {
                return Err(error);
            }

            let frame = self.frame.expect("test poll frame configured");
            if buffer.len() < frame.len() {
                return Err(crate::network::DeviceError::BufferTooSmall);
            }

            buffer[..frame.len()].copy_from_slice(frame);
            Ok(&buffer[..frame.len()])
        }

        fn transmit_frame(&mut self, frame: &[u8]) -> Result<(), crate::network::DeviceError> {
            if let Some(error) = self.transmit_error {
                return Err(error);
            }

            self.transmitted[..frame.len()].copy_from_slice(frame);
            self.transmitted_len = frame.len();
            Ok(())
        }
    }

    const fn ping_local_endpoint() -> crate::network::LocalNetworkEndpoint {
        crate::network::LocalNetworkEndpoint::new(
            crate::network::MacAddress::new([0x02, 0, 0, 0, 0, 99]),
            [192, 0, 2, 1],
        )
    }

    fn ping_arp_reply_frame()
    -> [u8; crate::network::ETHERNET_HEADER_LEN + crate::network::ARP_ETHERNET_IPV4_LEN] {
        let mut frame =
            [0u8; crate::network::ETHERNET_HEADER_LEN + crate::network::ARP_ETHERNET_IPV4_LEN];
        frame[..6].copy_from_slice(&[0x02, 0, 0, 0, 0, 1]);
        frame[6..12].copy_from_slice(&[0x02, 0, 0, 0, 0, 20]);
        write_test_be_u16(&mut frame, 12, crate::network::ETHERTYPE_ARP);

        let arp = &mut frame[crate::network::ETHERNET_HEADER_LEN..];
        write_test_be_u16(arp, 0, 1);
        write_test_be_u16(arp, 2, crate::network::ETHERTYPE_IPV4);
        arp[4] = crate::network::ETHERNET_ADDR_LEN as u8;
        arp[5] = 4;
        write_test_be_u16(arp, 6, 2);
        arp[8..14].copy_from_slice(&[0x02, 0, 0, 0, 0, 20]);
        arp[14..18].copy_from_slice(&[192, 0, 2, 20]);
        arp[18..24].copy_from_slice(&[0x02, 0, 0, 0, 0, 1]);
        arp[24..28].copy_from_slice(&[192, 0, 2, 10]);
        frame
    }

    fn ping_icmp_echo_reply_frame()
    -> [u8; crate::network::ETHERNET_HEADER_LEN + crate::network::IPV4_MIN_HEADER_LEN + 12] {
        let mut frame =
            [0u8; crate::network::ETHERNET_HEADER_LEN + crate::network::IPV4_MIN_HEADER_LEN + 12];
        frame[..6].copy_from_slice(&[0x02, 0, 0, 0, 0, 99]);
        frame[6..12].copy_from_slice(&[0x02, 0, 0, 0, 0, 20]);
        write_test_be_u16(&mut frame, 12, crate::network::ETHERTYPE_IPV4);

        let ipv4 = &mut frame[crate::network::ETHERNET_HEADER_LEN
            ..crate::network::ETHERNET_HEADER_LEN + crate::network::IPV4_MIN_HEADER_LEN];
        ipv4[0] = 0x45;
        write_test_be_u16(ipv4, 2, (crate::network::IPV4_MIN_HEADER_LEN + 12) as u16);
        write_test_be_u16(ipv4, 4, 0x4444);
        ipv4[8] = 64;
        ipv4[9] = crate::network::IPV4_PROTOCOL_ICMP;
        ipv4[12..16].copy_from_slice(&[192, 0, 2, 20]);
        ipv4[16..20].copy_from_slice(&[192, 0, 2, 1]);
        let checksum = test_internet_checksum(ipv4);
        write_test_be_u16(ipv4, 10, checksum);

        let icmp =
            &mut frame[crate::network::ETHERNET_HEADER_LEN + crate::network::IPV4_MIN_HEADER_LEN..];
        icmp[0] = 0;
        icmp[4..].copy_from_slice(&[0x12, 0x34, 0, 7, 1, 2, 3, 4]);
        let checksum = test_internet_checksum(icmp);
        write_test_be_u16(icmp, 2, checksum);
        frame
    }

    fn ping_local_arp_request_frame()
    -> [u8; crate::network::ETHERNET_HEADER_LEN + crate::network::ARP_ETHERNET_IPV4_LEN] {
        let mut frame =
            [0u8; crate::network::ETHERNET_HEADER_LEN + crate::network::ARP_ETHERNET_IPV4_LEN];
        frame[..6].copy_from_slice(&[0xff; crate::network::ETHERNET_ADDR_LEN]);
        frame[6..12].copy_from_slice(&[0x02, 0, 0, 0, 0, 20]);
        write_test_be_u16(&mut frame, 12, crate::network::ETHERTYPE_ARP);

        let arp = &mut frame[crate::network::ETHERNET_HEADER_LEN..];
        write_test_be_u16(arp, 0, 1);
        write_test_be_u16(arp, 2, crate::network::ETHERTYPE_IPV4);
        arp[4] = crate::network::ETHERNET_ADDR_LEN as u8;
        arp[5] = 4;
        write_test_be_u16(arp, 6, 1);
        arp[8..14].copy_from_slice(&[0x02, 0, 0, 0, 0, 20]);
        arp[14..18].copy_from_slice(&[192, 0, 2, 20]);
        arp[18..24].copy_from_slice(&[0; crate::network::ETHERNET_ADDR_LEN]);
        arp[24..28].copy_from_slice(&[192, 0, 2, 1]);
        frame
    }

    fn ping_local_icmp_echo_request_frame()
    -> [u8; crate::network::ETHERNET_HEADER_LEN + crate::network::IPV4_MIN_HEADER_LEN + 12] {
        let mut frame =
            [0u8; crate::network::ETHERNET_HEADER_LEN + crate::network::IPV4_MIN_HEADER_LEN + 12];
        frame[..6].copy_from_slice(&[0x02, 0, 0, 0, 0, 99]);
        frame[6..12].copy_from_slice(&[0x02, 0, 0, 0, 0, 20]);
        write_test_be_u16(&mut frame, 12, crate::network::ETHERTYPE_IPV4);

        let ipv4 = &mut frame[crate::network::ETHERNET_HEADER_LEN
            ..crate::network::ETHERNET_HEADER_LEN + crate::network::IPV4_MIN_HEADER_LEN];
        ipv4[0] = 0x45;
        write_test_be_u16(ipv4, 2, (crate::network::IPV4_MIN_HEADER_LEN + 12) as u16);
        write_test_be_u16(ipv4, 4, 0x2222);
        ipv4[8] = 64;
        ipv4[9] = crate::network::IPV4_PROTOCOL_ICMP;
        ipv4[12..16].copy_from_slice(&[192, 0, 2, 20]);
        ipv4[16..20].copy_from_slice(&[192, 0, 2, 1]);
        let checksum = test_internet_checksum(ipv4);
        write_test_be_u16(ipv4, 10, checksum);

        let icmp =
            &mut frame[crate::network::ETHERNET_HEADER_LEN + crate::network::IPV4_MIN_HEADER_LEN..];
        icmp[0] = 8;
        icmp[4..].copy_from_slice(&[0x56, 0x78, 0, 9, 5, 6, 7, 8]);
        let checksum = test_internet_checksum(icmp);
        write_test_be_u16(icmp, 2, checksum);
        frame
    }

    fn write_test_be_u16(bytes: &mut [u8], offset: usize, value: u16) {
        let raw = value.to_be_bytes();
        bytes[offset] = raw[0];
        bytes[offset + 1] = raw[1];
    }

    fn test_internet_checksum(bytes: &[u8]) -> u16 {
        let mut sum = 0u32;
        let mut index = 0;
        while index + 1 < bytes.len() {
            sum += u16::from_be_bytes([bytes[index], bytes[index + 1]]) as u32;
            index += 2;
        }
        if index < bytes.len() {
            sum += (bytes[index] as u32) << 8;
        }
        while sum >> 16 != 0 {
            sum = (sum & 0xffff) + (sum >> 16);
        }
        !(sum as u16)
    }

    #[test_case]
    fn ping_operation_syscall_substitute_completes_unresolved_arp_to_echo_reply() {
        let destination = [192, 0, 2, 20];
        let policy = crate::network::Ipv4EgressRoutePolicy::new([255, 255, 255, 0], None);
        let payload = [1, 2, 3, 4];
        let mut table = crate::network::NetworkPingOperationDescriptorTable::<1, 2, 4>::new();
        let mut receive_buffer = [0u8; 128];
        let mut transmit_buffer = [0u8; 128];
        let mut adapter = PingOperationSyscallSubstitute::new(
            &mut table,
            &mut receive_buffer,
            &mut transmit_buffer,
        );
        let mut status = PingOperationSyscallSubstituteStatus::idle();

        let descriptor = adapter.open().expect("open descriptor");
        assert_eq!(descriptor, 0);
        assert_eq!(adapter.status(descriptor, &mut status), Ok(()));
        assert_eq!(
            status.kind(),
            PingOperationSyscallSubstituteStatusKind::Idle
        );

        let start = adapter
            .start(
                descriptor,
                &mut PingOutboundTransmitDevice::new(),
                ping_local_endpoint(),
                policy,
                destination,
                0x1234,
                7,
                61,
                &payload,
                1,
            )
            .expect("start pending arp");
        assert_eq!(
            start.kind(),
            PingOperationSyscallSubstituteStepKind::StartedPendingArp
        );
        assert_eq!(
            start.frame_len(),
            crate::network::ETHERNET_HEADER_LEN + crate::network::ARP_ETHERNET_IPV4_LEN
        );
        assert_eq!(adapter.status(descriptor, &mut status), Ok(()));
        assert_eq!(
            status.kind(),
            PingOperationSyscallSubstituteStatusKind::PendingArp
        );
        assert_eq!(status.destination_ipv4(), destination);
        assert_eq!(status.next_hop_ipv4(), destination);
        assert_eq!(status.arp_retries_remaining(), 1);

        let arp_reply = ping_arp_reply_frame();
        let advanced = adapter
            .pump(descriptor, &mut PingPollDevice::with_frame(&arp_reply))
            .expect("advance to inflight");
        assert_eq!(
            advanced.kind(),
            PingOperationSyscallSubstituteStepKind::AdvancedToInflight
        );
        assert_eq!(
            advanced.frame_len(),
            crate::network::ETHERNET_HEADER_LEN
                + crate::network::IPV4_MIN_HEADER_LEN
                + crate::network::ICMP_ECHO_HEADER_LEN
                + payload.len()
        );
        assert_eq!(adapter.status(descriptor, &mut status), Ok(()));
        assert_eq!(
            status.kind(),
            PingOperationSyscallSubstituteStatusKind::Inflight
        );

        let echo_reply = ping_icmp_echo_reply_frame();
        let completed = adapter
            .pump(descriptor, &mut PingPollDevice::with_frame(&echo_reply))
            .expect("complete echo reply");
        assert_eq!(
            completed.kind(),
            PingOperationSyscallSubstituteStepKind::Completed
        );
        assert_eq!(completed.payload_len(), payload.len());
        assert_eq!(adapter.status(descriptor, &mut status), Ok(()));
        assert_eq!(
            status.kind(),
            PingOperationSyscallSubstituteStatusKind::Completed
        );
        assert_eq!(status.destination_ipv4(), destination);
        assert_eq!(status.payload_len(), payload.len());

        assert_eq!(adapter.close(descriptor), Ok(()));
        assert_eq!(
            adapter.status(descriptor, &mut status),
            Err(PosixError::BadDescriptor)
        );
    }

    #[test_case]
    fn ping_operation_syscall_substitute_maps_descriptor_and_device_errors() {
        let destination = [192, 0, 2, 20];
        let policy = crate::network::Ipv4EgressRoutePolicy::new([255, 255, 255, 0], None);
        let payload = [1, 2, 3, 4];
        let mut empty_table = crate::network::NetworkPingOperationDescriptorTable::<0, 2, 4>::new();
        let mut empty_receive = [0u8; 128];
        let mut empty_transmit = [0u8; 128];
        let mut empty_adapter = PingOperationSyscallSubstitute::new(
            &mut empty_table,
            &mut empty_receive,
            &mut empty_transmit,
        );

        assert_eq!(empty_adapter.open(), Err(PosixError::TooManyOpenFiles));

        let mut table = crate::network::NetworkPingOperationDescriptorTable::<1, 2, 4>::new();
        let mut receive_buffer = [0u8; 128];
        let mut transmit_buffer = [0u8; 128];
        let mut adapter = PingOperationSyscallSubstitute::new(
            &mut table,
            &mut receive_buffer,
            &mut transmit_buffer,
        );
        let mut status = PingOperationSyscallSubstituteStatus::idle();
        let descriptor = adapter.open().expect("open descriptor");

        assert_eq!(adapter.open(), Err(PosixError::Busy));
        assert_eq!(
            adapter.status(7, &mut status),
            Err(PosixError::BadDescriptor)
        );
        assert_eq!(adapter.close(descriptor), Ok(()));
        assert_eq!(adapter.close(descriptor), Err(PosixError::BadDescriptor));
        assert_eq!(
            adapter.start(
                descriptor,
                &mut PingOutboundTransmitDevice::new(),
                ping_local_endpoint(),
                policy,
                destination,
                0x1234,
                7,
                61,
                &payload,
                0,
            ),
            Err(PosixError::BadDescriptor)
        );

        let descriptor = adapter.open().expect("reopen descriptor");
        assert_eq!(
            adapter
                .start(
                    descriptor,
                    &mut PingOutboundTransmitDevice::new(),
                    ping_local_endpoint(),
                    policy,
                    destination,
                    0x1234,
                    7,
                    61,
                    &payload,
                    0,
                )
                .expect("start pending without retry budget")
                .kind(),
            PingOperationSyscallSubstituteStepKind::StartedPendingArp
        );
        assert_eq!(
            adapter.retry_arp(descriptor, &mut PingOutboundTransmitDevice::new()),
            Err(PosixError::Again)
        );
        let timeout = adapter.timeout(descriptor).expect("explicit timeout");
        assert_eq!(
            timeout.kind(),
            PingOperationSyscallSubstituteStepKind::TimedOut
        );
        assert_eq!(timeout.destination_ipv4(), destination);
        assert_eq!(adapter.status(descriptor, &mut status), Ok(()));
        assert_eq!(
            status.kind(),
            PingOperationSyscallSubstituteStatusKind::TimedOut
        );
        assert_eq!(adapter.close(descriptor), Ok(()));

        let descriptor = adapter.open().expect("reopen for tx error");
        assert_eq!(
            adapter.start(
                descriptor,
                &mut PingOutboundTransmitDevice::with_transmit_error(
                    crate::network::DeviceError::Io,
                ),
                ping_local_endpoint(),
                policy,
                destination,
                0x1234,
                7,
                61,
                &payload,
                0,
            ),
            Err(PosixError::Io)
        );
        assert_eq!(adapter.status(descriptor, &mut status), Ok(()));
        assert_eq!(
            status.kind(),
            PingOperationSyscallSubstituteStatusKind::Idle
        );
        assert_eq!(adapter.close(descriptor), Ok(()));

        let descriptor = adapter.open().expect("reopen for rx and pump tx errors");
        assert_eq!(
            adapter
                .start(
                    descriptor,
                    &mut PingOutboundTransmitDevice::new(),
                    ping_local_endpoint(),
                    policy,
                    destination,
                    0x1234,
                    7,
                    61,
                    &payload,
                    1,
                )
                .expect("start pending")
                .kind(),
            PingOperationSyscallSubstituteStepKind::StartedPendingArp
        );
        assert_eq!(
            adapter.pump(
                descriptor,
                &mut PingPollDevice::with_receive_error(crate::network::DeviceError::Io),
            ),
            Err(PosixError::Io)
        );
        let arp_reply = ping_arp_reply_frame();
        assert_eq!(
            adapter.pump(
                descriptor,
                &mut PingPollDevice::with_transmit_error(
                    &arp_reply,
                    crate::network::DeviceError::Io
                ),
            ),
            Err(PosixError::Io)
        );
    }

    #[test_case]
    fn runtime_ping_syscall_substitute_completes_unresolved_arp_to_echo_reply_through_runtime_pump()
    {
        let destination = [192, 0, 2, 20];
        let policy = crate::network::Ipv4EgressRoutePolicy::new([255, 255, 255, 0], None);
        let payload = [1, 2, 3, 4];
        let mut runtime =
            crate::network::NetworkRuntimeDevicePump::<2, 1, 2, 4>::new(ping_local_endpoint());
        let mut receive_buffer = [0u8; 128];
        let mut transmit_buffer = [0u8; 128];
        let mut adapter = RuntimePingOperationSyscallSubstitute::new(
            &mut runtime,
            &mut receive_buffer,
            &mut transmit_buffer,
        );
        let mut status = PingOperationSyscallSubstituteStatus::idle();

        let descriptor = adapter.open().expect("open runtime ping descriptor");
        assert_eq!(descriptor, 0);
        assert_eq!(adapter.status(descriptor, &mut status), Ok(()));
        assert_eq!(
            status.kind(),
            PingOperationSyscallSubstituteStatusKind::Idle
        );

        let start = adapter
            .start(
                descriptor,
                &mut PingOutboundTransmitDevice::new(),
                policy,
                destination,
                0x1234,
                7,
                61,
                &payload,
                1,
            )
            .expect("start through runtime pump");
        assert_eq!(
            start.kind(),
            PingOperationSyscallSubstituteStepKind::StartedPendingArp
        );
        assert_eq!(adapter.status(descriptor, &mut status), Ok(()));
        assert_eq!(
            status.kind(),
            PingOperationSyscallSubstituteStatusKind::PendingArp
        );
        assert_eq!(status.destination_ipv4(), destination);

        let arp_reply = ping_arp_reply_frame();
        let advanced = adapter
            .pump(descriptor, &mut PingPollDevice::with_frame(&arp_reply))
            .expect("runtime pump advances arp reply");
        assert_eq!(
            advanced.kind(),
            RuntimePingOperationSyscallSubstitutePumpKind::ActivePing
        );
        assert_eq!(advanced.descriptor(), descriptor);
        assert_eq!(
            advanced.active_ping_step().kind(),
            PingOperationSyscallSubstituteStepKind::AdvancedToInflight
        );
        assert_eq!(adapter.status(descriptor, &mut status), Ok(()));
        assert_eq!(
            status.kind(),
            PingOperationSyscallSubstituteStatusKind::Inflight
        );

        let echo_reply = ping_icmp_echo_reply_frame();
        let completed = adapter
            .pump(descriptor, &mut PingPollDevice::with_frame(&echo_reply))
            .expect("runtime pump completes echo reply");
        assert_eq!(
            completed.kind(),
            RuntimePingOperationSyscallSubstitutePumpKind::ActivePing
        );
        assert_eq!(
            completed.active_ping_step().kind(),
            PingOperationSyscallSubstituteStepKind::Completed
        );
        assert_eq!(completed.active_ping_step().payload_len(), payload.len());
        assert_eq!(adapter.status(descriptor, &mut status), Ok(()));
        assert_eq!(
            status.kind(),
            PingOperationSyscallSubstituteStatusKind::Completed
        );
        assert_eq!(status.payload_len(), payload.len());
    }

    #[test_case]
    fn runtime_ping_syscall_substitute_keeps_local_responder_working_with_open_descriptor() {
        let mut runtime =
            crate::network::NetworkRuntimeDevicePump::<2, 1, 2, 4>::new(ping_local_endpoint());
        let mut receive_buffer = [0u8; 128];
        let mut transmit_buffer = [0u8; 128];
        let mut adapter = RuntimePingOperationSyscallSubstitute::new(
            &mut runtime,
            &mut receive_buffer,
            &mut transmit_buffer,
        );
        let descriptor = adapter.open().expect("open descriptor");

        let arp_request = ping_local_arp_request_frame();
        let arp_reply = adapter
            .pump(descriptor, &mut PingPollDevice::with_frame(&arp_request))
            .expect("local arp reply");
        assert_eq!(
            arp_reply.kind(),
            RuntimePingOperationSyscallSubstitutePumpKind::LocalReply
        );
        assert_eq!(
            arp_reply.local_reply_kind(),
            Some(crate::network::PacketReplyKind::Arp)
        );
        assert_eq!(
            arp_reply.local_reply_frame_len(),
            crate::network::ETHERNET_HEADER_LEN + crate::network::ARP_ETHERNET_IPV4_LEN
        );

        let icmp_request = ping_local_icmp_echo_request_frame();
        let icmp_reply = adapter
            .pump(descriptor, &mut PingPollDevice::with_frame(&icmp_request))
            .expect("local icmp reply");
        assert_eq!(
            icmp_reply.kind(),
            RuntimePingOperationSyscallSubstitutePumpKind::LocalReply
        );
        assert_eq!(
            icmp_reply.local_reply_kind(),
            Some(crate::network::PacketReplyKind::IcmpEcho)
        );
        assert_eq!(
            icmp_reply.local_reply_frame_len(),
            crate::network::ETHERNET_HEADER_LEN
                + crate::network::IPV4_MIN_HEADER_LEN
                + crate::network::ICMP_ECHO_HEADER_LEN
                + 4
        );

        assert_eq!(adapter.close(descriptor), Ok(()));
    }

    #[test_case]
    fn runtime_ping_syscall_substitute_maps_descriptor_capacity_timeout_and_device_errors() {
        let destination = [192, 0, 2, 20];
        let policy = crate::network::Ipv4EgressRoutePolicy::new([255, 255, 255, 0], None);
        let payload = [1, 2, 3, 4];
        let mut empty_runtime =
            crate::network::NetworkRuntimeDevicePump::<0, 0, 2, 4>::new(ping_local_endpoint());
        let mut empty_receive = [0u8; 128];
        let mut empty_transmit = [0u8; 128];
        let mut empty_adapter = RuntimePingOperationSyscallSubstitute::new(
            &mut empty_runtime,
            &mut empty_receive,
            &mut empty_transmit,
        );

        assert_eq!(empty_adapter.open(), Err(PosixError::TooManyOpenFiles));

        let mut runtime =
            crate::network::NetworkRuntimeDevicePump::<1, 1, 2, 4>::new(ping_local_endpoint());
        let mut receive_buffer = [0u8; 128];
        let mut transmit_buffer = [0u8; 128];
        let mut adapter = RuntimePingOperationSyscallSubstitute::new(
            &mut runtime,
            &mut receive_buffer,
            &mut transmit_buffer,
        );
        let mut status = PingOperationSyscallSubstituteStatus::idle();
        let descriptor = adapter.open().expect("open descriptor");

        assert_eq!(adapter.open(), Err(PosixError::Busy));
        assert_eq!(
            adapter.status(7, &mut status),
            Err(PosixError::BadDescriptor)
        );
        assert_eq!(adapter.close(descriptor), Ok(()));
        assert_eq!(adapter.close(descriptor), Err(PosixError::BadDescriptor));

        let descriptor = adapter.open().expect("reopen descriptor");
        assert_eq!(
            adapter
                .start(
                    descriptor,
                    &mut PingOutboundTransmitDevice::new(),
                    policy,
                    destination,
                    0x1234,
                    7,
                    61,
                    &payload,
                    0,
                )
                .expect("start pending without retry budget")
                .kind(),
            PingOperationSyscallSubstituteStepKind::StartedPendingArp
        );
        assert_eq!(
            adapter.retry_arp(descriptor, &mut PingOutboundTransmitDevice::new()),
            Err(PosixError::Again)
        );
        let timeout = adapter.timeout(descriptor).expect("timeout pending ping");
        assert_eq!(
            timeout.kind(),
            PingOperationSyscallSubstituteStepKind::TimedOut
        );
        assert_eq!(timeout.destination_ipv4(), destination);
        assert_eq!(adapter.status(descriptor, &mut status), Ok(()));
        assert_eq!(
            status.kind(),
            PingOperationSyscallSubstituteStatusKind::TimedOut
        );
        assert_eq!(adapter.close(descriptor), Ok(()));

        let descriptor = adapter.open().expect("reopen for runtime pump errors");
        assert_eq!(
            adapter.pump(
                descriptor,
                &mut PingPollDevice::with_receive_error(crate::network::DeviceError::Io),
            ),
            Err(PosixError::Io)
        );

        let arp_request = ping_local_arp_request_frame();
        assert_eq!(
            adapter.pump(
                descriptor,
                &mut PingPollDevice::with_transmit_error(
                    &arp_request,
                    crate::network::DeviceError::Io
                ),
            ),
            Err(PosixError::Io)
        );

        assert_eq!(
            adapter
                .start(
                    descriptor,
                    &mut PingOutboundTransmitDevice::new(),
                    policy,
                    destination,
                    0x1234,
                    7,
                    61,
                    &payload,
                    1,
                )
                .expect("start active-error case")
                .kind(),
            PingOperationSyscallSubstituteStepKind::StartedPendingArp
        );
        let arp_reply = ping_arp_reply_frame();
        assert_eq!(
            adapter.pump(
                descriptor,
                &mut PingPollDevice::with_transmit_error(
                    &arp_reply,
                    crate::network::DeviceError::Io
                ),
            ),
            Err(PosixError::Io)
        );
    }

    #[test_case]
    fn descriptor_shaped_ping_control_completes_single_runtime_pump_lifecycle() {
        let destination = [192, 0, 2, 20];
        let policy = crate::network::Ipv4EgressRoutePolicy::new([255, 255, 255, 0], None);
        let payload = [1, 2, 3, 4];
        let mut runtime =
            crate::network::NetworkRuntimeDevicePump::<2, 1, 2, 4>::new(ping_local_endpoint());
        let mut receive_buffer = [0u8; 128];
        let mut transmit_buffer = [0u8; 128];
        let mut control = DescriptorShapedPingControl::new(
            &mut runtime,
            &mut receive_buffer,
            &mut transmit_buffer,
        );
        let mut status = PingOperationSyscallSubstituteStatus::idle();

        let descriptor = control
            .open()
            .expect("open descriptor-shaped control handle");
        assert_eq!(descriptor, 0);
        assert_eq!(control.status(descriptor, &mut status), Ok(()));
        assert_eq!(
            status.kind(),
            PingOperationSyscallSubstituteStatusKind::Idle
        );

        let start = control
            .start(
                descriptor,
                &mut PingOutboundTransmitDevice::new(),
                policy,
                destination,
                0x1234,
                7,
                61,
                &payload,
                1,
            )
            .expect("start through descriptor-shaped control");
        assert_eq!(
            start.kind(),
            PingOperationSyscallSubstituteStepKind::StartedPendingArp
        );
        assert_eq!(control.status(descriptor, &mut status), Ok(()));
        assert_eq!(
            status.kind(),
            PingOperationSyscallSubstituteStatusKind::PendingArp
        );
        assert_eq!(status.destination_ipv4(), destination);
        assert_eq!(status.arp_retries_remaining(), 1);

        let arp_reply = ping_arp_reply_frame();
        let advanced = control
            .pump_or_read_result(descriptor, &mut PingPollDevice::with_frame(&arp_reply))
            .expect("descriptor-shaped control advances arp reply");
        assert_eq!(
            advanced.kind(),
            RuntimePingOperationSyscallSubstitutePumpKind::ActivePing
        );
        assert_eq!(advanced.descriptor(), descriptor);
        assert_eq!(
            advanced.active_ping_step().kind(),
            PingOperationSyscallSubstituteStepKind::AdvancedToInflight
        );
        assert_eq!(control.status(descriptor, &mut status), Ok(()));
        assert_eq!(
            status.kind(),
            PingOperationSyscallSubstituteStatusKind::Inflight
        );

        let echo_reply = ping_icmp_echo_reply_frame();
        let completed = control
            .pump_or_read_result(descriptor, &mut PingPollDevice::with_frame(&echo_reply))
            .expect("descriptor-shaped control completes echo reply");
        assert_eq!(
            completed.kind(),
            RuntimePingOperationSyscallSubstitutePumpKind::ActivePing
        );
        assert_eq!(
            completed.active_ping_step().kind(),
            PingOperationSyscallSubstituteStepKind::Completed
        );
        assert_eq!(completed.active_ping_step().payload_len(), payload.len());
        assert_eq!(control.status(descriptor, &mut status), Ok(()));
        assert_eq!(
            status.kind(),
            PingOperationSyscallSubstituteStatusKind::Completed
        );
        assert_eq!(status.payload_len(), payload.len());

        assert_eq!(control.close(descriptor), Ok(()));
        assert_eq!(
            control.status(descriptor, &mut status),
            Err(PosixError::BadDescriptor)
        );
    }

    #[test_case]
    fn descriptor_shaped_ping_control_maps_descriptor_capacity_retry_timeout_and_io_errors() {
        let destination = [192, 0, 2, 20];
        let policy = crate::network::Ipv4EgressRoutePolicy::new([255, 255, 255, 0], None);
        let payload = [1, 2, 3, 4];
        let mut empty_runtime =
            crate::network::NetworkRuntimeDevicePump::<0, 0, 2, 4>::new(ping_local_endpoint());
        let mut empty_receive = [0u8; 128];
        let mut empty_transmit = [0u8; 128];
        let mut empty_control = DescriptorShapedPingControl::new(
            &mut empty_runtime,
            &mut empty_receive,
            &mut empty_transmit,
        );

        assert_eq!(empty_control.open(), Err(PosixError::TooManyOpenFiles));

        let mut runtime =
            crate::network::NetworkRuntimeDevicePump::<1, 1, 2, 4>::new(ping_local_endpoint());
        let mut receive_buffer = [0u8; 128];
        let mut transmit_buffer = [0u8; 128];
        let mut control = DescriptorShapedPingControl::new(
            &mut runtime,
            &mut receive_buffer,
            &mut transmit_buffer,
        );
        let mut status = PingOperationSyscallSubstituteStatus::idle();
        let descriptor = control
            .open()
            .expect("open descriptor-shaped control handle");

        assert_eq!(control.open(), Err(PosixError::Busy));
        assert_eq!(
            control.status(7, &mut status),
            Err(PosixError::BadDescriptor)
        );
        assert_eq!(control.close(descriptor), Ok(()));
        assert_eq!(control.close(descriptor), Err(PosixError::BadDescriptor));
        assert_eq!(
            control.start(
                descriptor,
                &mut PingOutboundTransmitDevice::new(),
                policy,
                destination,
                0x1234,
                7,
                61,
                &payload,
                0,
            ),
            Err(PosixError::BadDescriptor)
        );

        let descriptor = control
            .open()
            .expect("reopen descriptor-shaped control handle");
        assert_eq!(
            control
                .start(
                    descriptor,
                    &mut PingOutboundTransmitDevice::new(),
                    policy,
                    destination,
                    0x1234,
                    7,
                    61,
                    &payload,
                    0,
                )
                .expect("start pending without retry budget")
                .kind(),
            PingOperationSyscallSubstituteStepKind::StartedPendingArp
        );
        assert_eq!(
            control.retry_arp(descriptor, &mut PingOutboundTransmitDevice::new()),
            Err(PosixError::Again)
        );
        let timeout = control.timeout(descriptor).expect("timeout pending ping");
        assert_eq!(
            timeout.kind(),
            PingOperationSyscallSubstituteStepKind::TimedOut
        );
        assert_eq!(timeout.destination_ipv4(), destination);
        assert_eq!(control.status(descriptor, &mut status), Ok(()));
        assert_eq!(
            status.kind(),
            PingOperationSyscallSubstituteStatusKind::TimedOut
        );
        assert_eq!(control.close(descriptor), Ok(()));

        let descriptor = control
            .open()
            .expect("reopen for receive and transmit errors");
        assert_eq!(
            control.pump_or_read_result(
                descriptor,
                &mut PingPollDevice::with_receive_error(crate::network::DeviceError::Io),
            ),
            Err(PosixError::Io)
        );

        let arp_request = ping_local_arp_request_frame();
        assert_eq!(
            control.pump_or_read_result(
                descriptor,
                &mut PingPollDevice::with_transmit_error(
                    &arp_request,
                    crate::network::DeviceError::Io
                ),
            ),
            Err(PosixError::Io)
        );

        assert_eq!(
            control
                .start(
                    descriptor,
                    &mut PingOutboundTransmitDevice::new(),
                    policy,
                    destination,
                    0x1234,
                    7,
                    61,
                    &payload,
                    1,
                )
                .expect("start active transmit error case")
                .kind(),
            PingOperationSyscallSubstituteStepKind::StartedPendingArp
        );
        let arp_reply = ping_arp_reply_frame();
        assert_eq!(
            control.pump_or_read_result(
                descriptor,
                &mut PingPollDevice::with_transmit_error(
                    &arp_reply,
                    crate::network::DeviceError::Io
                ),
            ),
            Err(PosixError::Io)
        );
    }

    #[test_case]
    fn descriptor_shaped_ping_control_maps_caller_receive_buffer_pressure_to_enospc() {
        let mut runtime =
            crate::network::NetworkRuntimeDevicePump::<1, 1, 2, 4>::new(ping_local_endpoint());
        let mut receive_buffer = [0u8; 8];
        let mut transmit_buffer = [0u8; 128];
        let mut control = DescriptorShapedPingControl::new(
            &mut runtime,
            &mut receive_buffer,
            &mut transmit_buffer,
        );
        let descriptor = control
            .open()
            .expect("open descriptor-shaped control handle");
        let arp_request = ping_local_arp_request_frame();

        assert_eq!(
            control.pump_or_read_result(descriptor, &mut PingPollDevice::with_frame(&arp_request)),
            Err(PosixError::NoSpace)
        );
    }

    #[test_case]
    fn process_local_ping_descriptor_control_completes_lifecycle_through_process_descriptor() {
        let owner = crate::scheduler::ProcessOwnerId::new(31).expect("owner id");
        let destination = [192, 0, 2, 20];
        let policy = crate::network::Ipv4EgressRoutePolicy::new([255, 255, 255, 0], None);
        let payload = [1, 2, 3, 4];
        let mut store = crate::posix::ProcessDescriptorStore::<1, 5>::new_empty();
        store
            .create_owner_with_inherited_stdio(owner)
            .expect("create owner");
        let mut runtime =
            crate::network::NetworkRuntimeDevicePump::<2, 1, 2, 4>::new(ping_local_endpoint());
        let mut receive_buffer = [0u8; 128];
        let mut transmit_buffer = [0u8; 128];
        let mut control = ProcessLocalPingDescriptorControl::new(
            Some(owner),
            &mut store,
            &mut runtime,
            &mut receive_buffer,
            &mut transmit_buffer,
        );
        let mut status = PingOperationSyscallSubstituteStatus::idle();

        let process_descriptor = control.open().expect("open process-local ping descriptor");
        assert_eq!(process_descriptor, 3);
        assert_eq!(control.status(process_descriptor, &mut status), Ok(()));
        assert_eq!(
            status.kind(),
            PingOperationSyscallSubstituteStatusKind::Idle
        );

        let start = control
            .start(
                process_descriptor,
                &mut PingOutboundTransmitDevice::new(),
                policy,
                destination,
                0x1234,
                7,
                61,
                &payload,
                1,
            )
            .expect("start through process descriptor");
        assert_eq!(
            start.kind(),
            PingOperationSyscallSubstituteStepKind::StartedPendingArp
        );
        assert_eq!(control.status(process_descriptor, &mut status), Ok(()));
        assert_eq!(
            status.kind(),
            PingOperationSyscallSubstituteStatusKind::PendingArp
        );
        assert_eq!(status.destination_ipv4(), destination);

        let arp_reply = ping_arp_reply_frame();
        let advanced = control
            .pump_or_read_result(
                process_descriptor,
                &mut PingPollDevice::with_frame(&arp_reply),
            )
            .expect("advance through process descriptor");
        assert_eq!(
            advanced.kind(),
            RuntimePingOperationSyscallSubstitutePumpKind::ActivePing
        );
        assert_eq!(advanced.descriptor(), 0);
        assert_eq!(
            advanced.active_ping_step().kind(),
            PingOperationSyscallSubstituteStepKind::AdvancedToInflight
        );

        let echo_reply = ping_icmp_echo_reply_frame();
        let completed = control
            .pump_or_read_result(
                process_descriptor,
                &mut PingPollDevice::with_frame(&echo_reply),
            )
            .expect("complete through process descriptor");
        assert_eq!(
            completed.active_ping_step().kind(),
            PingOperationSyscallSubstituteStepKind::Completed
        );
        assert_eq!(completed.active_ping_step().payload_len(), payload.len());
        assert_eq!(control.status(process_descriptor, &mut status), Ok(()));
        assert_eq!(
            status.kind(),
            PingOperationSyscallSubstituteStatusKind::Completed
        );

        assert_eq!(control.close(process_descriptor), Ok(()));
        assert_eq!(
            control.status(process_descriptor, &mut status),
            Err(PosixError::BadDescriptor)
        );
    }

    #[test_case]
    fn process_local_ping_descriptor_control_maps_capacity_busy_closed_retry_timeout_and_io_errors()
    {
        let owner = crate::scheduler::ProcessOwnerId::new(31).expect("owner id");
        let destination = [192, 0, 2, 20];
        let policy = crate::network::Ipv4EgressRoutePolicy::new([255, 255, 255, 0], None);
        let payload = [1, 2, 3, 4];

        let mut missing_store = crate::posix::ProcessDescriptorStore::<1, 5>::new_empty();
        let mut missing_runtime =
            crate::network::NetworkRuntimeDevicePump::<1, 1, 2, 4>::new(ping_local_endpoint());
        let mut missing_receive = [0u8; 128];
        let mut missing_transmit = [0u8; 128];
        let mut missing_control = ProcessLocalPingDescriptorControl::new(
            None,
            &mut missing_store,
            &mut missing_runtime,
            &mut missing_receive,
            &mut missing_transmit,
        );
        assert_eq!(missing_control.open(), Err(PosixError::BadDescriptor));

        let mut full_store = crate::posix::ProcessDescriptorStore::<1, 3>::new_empty();
        full_store
            .create_owner_with_inherited_stdio(owner)
            .expect("create full owner");
        let mut full_runtime =
            crate::network::NetworkRuntimeDevicePump::<1, 1, 2, 4>::new(ping_local_endpoint());
        let mut full_receive = [0u8; 128];
        let mut full_transmit = [0u8; 128];
        let mut full_control = ProcessLocalPingDescriptorControl::new(
            Some(owner),
            &mut full_store,
            &mut full_runtime,
            &mut full_receive,
            &mut full_transmit,
        );
        assert_eq!(full_control.open(), Err(PosixError::TooManyOpenFiles));
        assert_eq!(full_control.open(), Err(PosixError::TooManyOpenFiles));

        let mut store = crate::posix::ProcessDescriptorStore::<1, 5>::new_empty();
        store
            .create_owner_with_inherited_stdio(owner)
            .expect("create owner");
        let mut runtime =
            crate::network::NetworkRuntimeDevicePump::<1, 1, 2, 4>::new(ping_local_endpoint());
        let mut receive_buffer = [0u8; 128];
        let mut transmit_buffer = [0u8; 128];
        let mut control = ProcessLocalPingDescriptorControl::new(
            Some(owner),
            &mut store,
            &mut runtime,
            &mut receive_buffer,
            &mut transmit_buffer,
        );
        let mut status = PingOperationSyscallSubstituteStatus::idle();
        let process_descriptor = control.open().expect("open descriptor");

        assert_eq!(control.open(), Err(PosixError::Busy));
        assert_eq!(
            control.status(crate::posix::STDOUT_FD, &mut status),
            Err(PosixError::BadDescriptor)
        );
        assert_eq!(control.close(process_descriptor), Ok(()));
        assert_eq!(
            control.close(process_descriptor),
            Err(PosixError::BadDescriptor)
        );
        assert_eq!(
            control.start(
                process_descriptor,
                &mut PingOutboundTransmitDevice::new(),
                policy,
                destination,
                0x1234,
                7,
                61,
                &payload,
                0,
            ),
            Err(PosixError::BadDescriptor)
        );

        let process_descriptor = control.open().expect("reopen descriptor");
        assert_eq!(
            control
                .start(
                    process_descriptor,
                    &mut PingOutboundTransmitDevice::new(),
                    policy,
                    destination,
                    0x1234,
                    7,
                    61,
                    &payload,
                    0,
                )
                .expect("start pending without retry budget")
                .kind(),
            PingOperationSyscallSubstituteStepKind::StartedPendingArp
        );
        assert_eq!(
            control.retry_arp(process_descriptor, &mut PingOutboundTransmitDevice::new()),
            Err(PosixError::Again)
        );
        let timeout = control
            .timeout(process_descriptor)
            .expect("timeout pending ping");
        assert_eq!(
            timeout.kind(),
            PingOperationSyscallSubstituteStepKind::TimedOut
        );
        assert_eq!(timeout.destination_ipv4(), destination);
        assert_eq!(control.close(process_descriptor), Ok(()));

        let process_descriptor = control.open().expect("reopen for io errors");
        assert_eq!(
            control.pump_or_read_result(
                process_descriptor,
                &mut PingPollDevice::with_receive_error(crate::network::DeviceError::Io),
            ),
            Err(PosixError::Io)
        );

        let arp_request = ping_local_arp_request_frame();
        assert_eq!(
            control.pump_or_read_result(
                process_descriptor,
                &mut PingPollDevice::with_transmit_error(
                    &arp_request,
                    crate::network::DeviceError::Io
                ),
            ),
            Err(PosixError::Io)
        );
    }

    #[test_case]
    fn process_local_ping_dispatch_completes_lifecycle_through_dispatch_shape() {
        let owner = crate::scheduler::ProcessOwnerId::new(31).expect("owner id");
        let destination = [192, 0, 2, 20];
        let policy = crate::network::Ipv4EgressRoutePolicy::new([255, 255, 255, 0], None);
        let payload = [1, 2, 3, 4];
        let mut store = crate::posix::ProcessDescriptorStore::<1, 5>::new_empty();
        store
            .create_owner_with_inherited_stdio(owner)
            .expect("create owner");
        let mut runtime =
            crate::network::NetworkRuntimeDevicePump::<2, 1, 2, 4>::new(ping_local_endpoint());
        let mut receive_buffer = [0u8; 128];
        let mut transmit_buffer = [0u8; 128];
        let mut step = PingOperationSyscallSubstituteStep::from_userspace(
            crate::network::UserspacePingOperationStep::NoFrame,
        );
        let mut pump_step = RuntimePingOperationSyscallSubstitutePumpStep::no_frame();
        let mut status = PingOperationSyscallSubstituteStatus::idle();

        let process_descriptor = {
            let mut device = PingOutboundTransmitDevice::new();
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            match dispatch_process_local_ping_descriptor_operation(
                ProcessLocalPingDispatchOperation::Open,
                Some(owner),
                &mut store,
                &mut runtime,
                &mut receive_buffer,
                &mut transmit_buffer,
                &mut device,
                &mut outputs,
            )
            .expect("open process-local ping descriptor through dispatch")
            {
                ProcessLocalPingDispatchOutcome::Opened { process_descriptor } => {
                    process_descriptor
                }
                outcome => panic!("unexpected open outcome {outcome:?}"),
            }
        };
        assert_eq!(process_descriptor, 3);

        {
            let mut device = PingOutboundTransmitDevice::new();
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                dispatch_process_local_ping_descriptor_operation(
                    ProcessLocalPingDispatchOperation::Status { process_descriptor },
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                ),
                Ok(ProcessLocalPingDispatchOutcome::Status)
            );
        }
        assert_eq!(
            status.kind(),
            PingOperationSyscallSubstituteStatusKind::Idle
        );

        {
            let mut device = PingOutboundTransmitDevice::new();
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                dispatch_process_local_ping_descriptor_operation(
                    ProcessLocalPingDispatchOperation::Start {
                        process_descriptor,
                        route_policy: policy,
                        destination_ipv4: destination,
                        identifier: 0x1234,
                        sequence_number: 7,
                        ttl: 61,
                        payload: &payload,
                        arp_retry_budget: 1,
                    },
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                ),
                Ok(ProcessLocalPingDispatchOutcome::Started)
            );
        }
        assert_eq!(
            step.kind(),
            PingOperationSyscallSubstituteStepKind::StartedPendingArp
        );

        let arp_reply = ping_arp_reply_frame();
        {
            let mut device = PingPollDevice::with_frame(&arp_reply);
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                dispatch_process_local_ping_descriptor_operation(
                    ProcessLocalPingDispatchOperation::PumpOrReadResult { process_descriptor },
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                ),
                Ok(ProcessLocalPingDispatchOutcome::PumpedOrReadResult)
            );
        }
        assert_eq!(
            pump_step.kind(),
            RuntimePingOperationSyscallSubstitutePumpKind::ActivePing
        );
        assert_eq!(
            pump_step.active_ping_step().kind(),
            PingOperationSyscallSubstituteStepKind::AdvancedToInflight
        );

        let echo_reply = ping_icmp_echo_reply_frame();
        {
            let mut device = PingPollDevice::with_frame(&echo_reply);
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                dispatch_process_local_ping_descriptor_operation(
                    ProcessLocalPingDispatchOperation::PumpOrReadResult { process_descriptor },
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                ),
                Ok(ProcessLocalPingDispatchOutcome::PumpedOrReadResult)
            );
        }
        assert_eq!(
            pump_step.active_ping_step().kind(),
            PingOperationSyscallSubstituteStepKind::Completed
        );
        assert_eq!(pump_step.active_ping_step().payload_len(), payload.len());

        {
            let mut device = PingOutboundTransmitDevice::new();
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                dispatch_process_local_ping_descriptor_operation(
                    ProcessLocalPingDispatchOperation::Status { process_descriptor },
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                ),
                Ok(ProcessLocalPingDispatchOutcome::Status)
            );
        }
        assert_eq!(
            status.kind(),
            PingOperationSyscallSubstituteStatusKind::Completed
        );
        assert_eq!(status.payload_len(), payload.len());

        {
            let mut device = PingOutboundTransmitDevice::new();
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                dispatch_process_local_ping_descriptor_operation(
                    ProcessLocalPingDispatchOperation::Close { process_descriptor },
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                ),
                Ok(ProcessLocalPingDispatchOutcome::Closed)
            );
        }

        {
            let mut device = PingOutboundTransmitDevice::new();
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                dispatch_process_local_ping_descriptor_operation(
                    ProcessLocalPingDispatchOperation::Status { process_descriptor },
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                ),
                Err(PosixError::BadDescriptor)
            );
        }
    }

    #[test_case]
    fn process_local_ping_dispatch_maps_descriptor_capacity_and_runtime_errors() {
        let owner = crate::scheduler::ProcessOwnerId::new(31).expect("owner id");
        let destination = [192, 0, 2, 20];
        let policy = crate::network::Ipv4EgressRoutePolicy::new([255, 255, 255, 0], None);
        let payload = [1, 2, 3, 4];
        let mut step = PingOperationSyscallSubstituteStep::from_userspace(
            crate::network::UserspacePingOperationStep::NoFrame,
        );
        let mut pump_step = RuntimePingOperationSyscallSubstitutePumpStep::no_frame();
        let mut status = PingOperationSyscallSubstituteStatus::idle();

        let mut missing_store = crate::posix::ProcessDescriptorStore::<1, 5>::new_empty();
        let mut missing_runtime =
            crate::network::NetworkRuntimeDevicePump::<1, 1, 2, 4>::new(ping_local_endpoint());
        let mut missing_receive = [0u8; 128];
        let mut missing_transmit = [0u8; 128];
        let mut missing_device = PingOutboundTransmitDevice::new();
        let mut missing_outputs =
            ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
        assert_eq!(
            dispatch_process_local_ping_descriptor_operation(
                ProcessLocalPingDispatchOperation::Open,
                None,
                &mut missing_store,
                &mut missing_runtime,
                &mut missing_receive,
                &mut missing_transmit,
                &mut missing_device,
                &mut missing_outputs,
            ),
            Err(PosixError::BadDescriptor)
        );

        let mut full_store = crate::posix::ProcessDescriptorStore::<1, 3>::new_empty();
        full_store
            .create_owner_with_inherited_stdio(owner)
            .expect("create full owner");
        let mut full_runtime =
            crate::network::NetworkRuntimeDevicePump::<1, 1, 2, 4>::new(ping_local_endpoint());
        let mut full_receive = [0u8; 128];
        let mut full_transmit = [0u8; 128];
        let mut full_device = PingOutboundTransmitDevice::new();
        let mut full_outputs =
            ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
        assert_eq!(
            dispatch_process_local_ping_descriptor_operation(
                ProcessLocalPingDispatchOperation::Open,
                Some(owner),
                &mut full_store,
                &mut full_runtime,
                &mut full_receive,
                &mut full_transmit,
                &mut full_device,
                &mut full_outputs,
            ),
            Err(PosixError::TooManyOpenFiles)
        );
        assert_eq!(
            dispatch_process_local_ping_descriptor_operation(
                ProcessLocalPingDispatchOperation::Open,
                Some(owner),
                &mut full_store,
                &mut full_runtime,
                &mut full_receive,
                &mut full_transmit,
                &mut full_device,
                &mut full_outputs,
            ),
            Err(PosixError::TooManyOpenFiles)
        );

        let mut store = crate::posix::ProcessDescriptorStore::<1, 5>::new_empty();
        store
            .create_owner_with_inherited_stdio(owner)
            .expect("create owner");
        let mut runtime =
            crate::network::NetworkRuntimeDevicePump::<1, 1, 2, 4>::new(ping_local_endpoint());
        let mut receive_buffer = [0u8; 128];
        let mut transmit_buffer = [0u8; 128];
        let mut device = PingOutboundTransmitDevice::new();
        let process_descriptor = {
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            match dispatch_process_local_ping_descriptor_operation(
                ProcessLocalPingDispatchOperation::Open,
                Some(owner),
                &mut store,
                &mut runtime,
                &mut receive_buffer,
                &mut transmit_buffer,
                &mut device,
                &mut outputs,
            )
            .expect("open descriptor")
            {
                ProcessLocalPingDispatchOutcome::Opened { process_descriptor } => {
                    process_descriptor
                }
                outcome => panic!("unexpected open outcome {outcome:?}"),
            }
        };

        {
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                dispatch_process_local_ping_descriptor_operation(
                    ProcessLocalPingDispatchOperation::Status {
                        process_descriptor: 7,
                    },
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                ),
                Err(PosixError::BadDescriptor)
            );
            assert_eq!(
                dispatch_process_local_ping_descriptor_operation(
                    ProcessLocalPingDispatchOperation::Status {
                        process_descriptor: crate::posix::STDOUT_FD,
                    },
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                ),
                Err(PosixError::BadDescriptor)
            );
            assert_eq!(
                dispatch_process_local_ping_descriptor_operation(
                    ProcessLocalPingDispatchOperation::Start {
                        process_descriptor,
                        route_policy: policy,
                        destination_ipv4: destination,
                        identifier: 0x1234,
                        sequence_number: 7,
                        ttl: 61,
                        payload: &payload,
                        arp_retry_budget: 0,
                    },
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                ),
                Ok(ProcessLocalPingDispatchOutcome::Started)
            );
            assert_eq!(
                dispatch_process_local_ping_descriptor_operation(
                    ProcessLocalPingDispatchOperation::Start {
                        process_descriptor,
                        route_policy: policy,
                        destination_ipv4: destination,
                        identifier: 0x1234,
                        sequence_number: 7,
                        ttl: 61,
                        payload: &payload,
                        arp_retry_budget: 0,
                    },
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                ),
                Err(PosixError::Busy)
            );
            assert_eq!(
                dispatch_process_local_ping_descriptor_operation(
                    ProcessLocalPingDispatchOperation::RetryArp { process_descriptor },
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                ),
                Err(PosixError::Again)
            );
            assert_eq!(
                dispatch_process_local_ping_descriptor_operation(
                    ProcessLocalPingDispatchOperation::Timeout { process_descriptor },
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                ),
                Ok(ProcessLocalPingDispatchOutcome::TimedOut)
            );
            assert_eq!(
                dispatch_process_local_ping_descriptor_operation(
                    ProcessLocalPingDispatchOperation::Status { process_descriptor },
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                ),
                Ok(ProcessLocalPingDispatchOutcome::Status)
            );
        }
        assert_eq!(
            status.kind(),
            PingOperationSyscallSubstituteStatusKind::TimedOut
        );

        let mut small_receive = [0u8; 4];
        let arp_reply = ping_arp_reply_frame();
        {
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                dispatch_process_local_ping_descriptor_operation(
                    ProcessLocalPingDispatchOperation::PumpOrReadResult { process_descriptor },
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut small_receive,
                    &mut transmit_buffer,
                    &mut PingPollDevice::with_frame(&arp_reply),
                    &mut outputs,
                ),
                Err(PosixError::NoSpace)
            );
            assert_eq!(
                dispatch_process_local_ping_descriptor_operation(
                    ProcessLocalPingDispatchOperation::PumpOrReadResult { process_descriptor },
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut PingPollDevice::with_receive_error(crate::network::DeviceError::Io),
                    &mut outputs,
                ),
                Err(PosixError::Io)
            );
        }

        let mut local_store = crate::posix::ProcessDescriptorStore::<1, 5>::new_empty();
        local_store
            .create_owner_with_inherited_stdio(owner)
            .expect("create local owner");
        let mut local_runtime =
            crate::network::NetworkRuntimeDevicePump::<1, 1, 2, 4>::new(ping_local_endpoint());
        let mut local_receive = [0u8; 128];
        let mut local_transmit = [0u8; 128];
        let mut local_device = PingOutboundTransmitDevice::new();
        let local_descriptor = {
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            match dispatch_process_local_ping_descriptor_operation(
                ProcessLocalPingDispatchOperation::Open,
                Some(owner),
                &mut local_store,
                &mut local_runtime,
                &mut local_receive,
                &mut local_transmit,
                &mut local_device,
                &mut outputs,
            )
            .expect("open local descriptor")
            {
                ProcessLocalPingDispatchOutcome::Opened { process_descriptor } => {
                    process_descriptor
                }
                outcome => panic!("unexpected open outcome {outcome:?}"),
            }
        };

        let local_arp_request = ping_local_arp_request_frame();
        {
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                dispatch_process_local_ping_descriptor_operation(
                    ProcessLocalPingDispatchOperation::PumpOrReadResult {
                        process_descriptor: local_descriptor,
                    },
                    Some(owner),
                    &mut local_store,
                    &mut local_runtime,
                    &mut local_receive,
                    &mut local_transmit,
                    &mut PingPollDevice::with_transmit_error(
                        &local_arp_request,
                        crate::network::DeviceError::Io
                    ),
                    &mut outputs,
                ),
                Err(PosixError::Io)
            );
        }

        let mut active_store = crate::posix::ProcessDescriptorStore::<1, 5>::new_empty();
        active_store
            .create_owner_with_inherited_stdio(owner)
            .expect("create active owner");
        let mut active_runtime =
            crate::network::NetworkRuntimeDevicePump::<1, 1, 2, 4>::new(ping_local_endpoint());
        let mut active_receive = [0u8; 128];
        let mut active_transmit = [0u8; 128];
        let mut active_device = PingOutboundTransmitDevice::new();
        let active_descriptor = {
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            match dispatch_process_local_ping_descriptor_operation(
                ProcessLocalPingDispatchOperation::Open,
                Some(owner),
                &mut active_store,
                &mut active_runtime,
                &mut active_receive,
                &mut active_transmit,
                &mut active_device,
                &mut outputs,
            )
            .expect("open active descriptor")
            {
                ProcessLocalPingDispatchOutcome::Opened { process_descriptor } => {
                    process_descriptor
                }
                outcome => panic!("unexpected open outcome {outcome:?}"),
            }
        };
        {
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                dispatch_process_local_ping_descriptor_operation(
                    ProcessLocalPingDispatchOperation::Start {
                        process_descriptor: active_descriptor,
                        route_policy: policy,
                        destination_ipv4: destination,
                        identifier: 0x1234,
                        sequence_number: 7,
                        ttl: 61,
                        payload: &payload,
                        arp_retry_budget: 1,
                    },
                    Some(owner),
                    &mut active_store,
                    &mut active_runtime,
                    &mut active_receive,
                    &mut active_transmit,
                    &mut active_device,
                    &mut outputs,
                ),
                Ok(ProcessLocalPingDispatchOutcome::Started)
            );
            assert_eq!(
                dispatch_process_local_ping_descriptor_operation(
                    ProcessLocalPingDispatchOperation::PumpOrReadResult {
                        process_descriptor: active_descriptor,
                    },
                    Some(owner),
                    &mut active_store,
                    &mut active_runtime,
                    &mut active_receive,
                    &mut active_transmit,
                    &mut PingPollDevice::with_transmit_error(
                        &arp_reply,
                        crate::network::DeviceError::Io
                    ),
                    &mut outputs,
                ),
                Err(PosixError::Io)
            );
        }
    }

    #[test_case]
    fn process_local_ping_user_arguments_complete_lifecycle_and_copy_outputs() {
        let owner = crate::scheduler::ProcessOwnerId::new(31).expect("owner id");
        let user_start = 0x0000_0000_0011_0000;
        let payload_user = user_start;
        let pump_user = user_start + 0x40;
        let status_user = user_start + 0x90;
        let destination = [192, 0, 2, 20];
        let mut store = crate::posix::ProcessDescriptorStore::<1, 5>::new_empty();
        store
            .create_owner_with_inherited_stdio(owner)
            .expect("create owner");
        let mappings = [crate::posix::UserMapping::new(
            user_start,
            0x100,
            crate::posix::UserMappingPermissions::USER_DATA,
        )
        .expect("user data mapping")];
        let mut user_memory = [0u8; 0x100];
        user_memory[..4].copy_from_slice(&[1, 2, 3, 4]);
        let mut kernel_scratch = [0u8; 64];
        let mut runtime =
            crate::network::NetworkRuntimeDevicePump::<2, 1, 2, 4>::new(ping_local_endpoint());
        let mut receive_buffer = [0u8; 128];
        let mut transmit_buffer = [0u8; 128];
        let mut step = PingOperationSyscallSubstituteStep::from_userspace(
            crate::network::UserspacePingOperationStep::NoFrame,
        );
        let mut pump_step = RuntimePingOperationSyscallSubstitutePumpStep::no_frame();
        let mut status = PingOperationSyscallSubstituteStatus::idle();

        let process_descriptor = {
            let mut device = PingOutboundTransmitDevice::new();
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            match dispatch_process_local_ping_descriptor_user_arguments(
                SyscallArguments::new([PROCESS_LOCAL_PING_USER_SELECTOR_OPEN, 0, 0, 0, 0, 0]),
                Some(owner),
                &mut store,
                &mappings,
                user_start,
                &mut user_memory,
                &mut kernel_scratch,
                &mut runtime,
                &mut receive_buffer,
                &mut transmit_buffer,
                &mut device,
                &mut outputs,
            )
            .expect("open through user arguments")
            {
                ProcessLocalPingDispatchOutcome::Opened { process_descriptor } => {
                    process_descriptor
                }
                outcome => panic!("unexpected open outcome {outcome:?}"),
            }
        };

        {
            let mut device = PingOutboundTransmitDevice::new();
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                dispatch_process_local_ping_descriptor_user_arguments(
                    SyscallArguments::new([
                        PROCESS_LOCAL_PING_USER_SELECTOR_STATUS,
                        process_descriptor as u64,
                        status_user,
                        PROCESS_LOCAL_PING_USER_STATUS_RECORD_LEN as u64,
                        0,
                        0,
                    ]),
                    Some(owner),
                    &mut store,
                    &mappings,
                    user_start,
                    &mut user_memory,
                    &mut kernel_scratch,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                ),
                Ok(ProcessLocalPingDispatchOutcome::Status)
            );
        }
        assert_eq!(
            read_test_le_u64(&user_memory, 0x90),
            process_local_ping_user_status_kind_code(
                PingOperationSyscallSubstituteStatusKind::Idle
            )
        );

        {
            let mut device = PingOutboundTransmitDevice::new();
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                dispatch_process_local_ping_descriptor_user_arguments(
                    SyscallArguments::new([
                        PROCESS_LOCAL_PING_USER_SELECTOR_START,
                        process_descriptor as u64,
                        payload_user,
                        4,
                        pack_process_local_ping_user_route(destination, 61, 24),
                        pack_process_local_ping_user_start_control(0x1234, 7, 1),
                    ]),
                    Some(owner),
                    &mut store,
                    &mappings,
                    user_start,
                    &mut user_memory,
                    &mut kernel_scratch,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                ),
                Ok(ProcessLocalPingDispatchOutcome::Started)
            );
            assert!(device.transmitted_len > 0);
        }
        assert_eq!(
            step.kind(),
            PingOperationSyscallSubstituteStepKind::StartedPendingArp
        );

        let arp_reply = ping_arp_reply_frame();
        {
            let mut device = PingPollDevice::with_frame(&arp_reply);
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                dispatch_process_local_ping_descriptor_user_arguments(
                    SyscallArguments::new([
                        PROCESS_LOCAL_PING_USER_SELECTOR_PUMP_OR_READ_RESULT,
                        process_descriptor as u64,
                        pump_user,
                        PROCESS_LOCAL_PING_USER_PUMP_RECORD_LEN as u64,
                        0,
                        0,
                    ]),
                    Some(owner),
                    &mut store,
                    &mappings,
                    user_start,
                    &mut user_memory,
                    &mut kernel_scratch,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                ),
                Ok(ProcessLocalPingDispatchOutcome::PumpedOrReadResult)
            );
        }
        assert_eq!(
            read_test_le_u64(&user_memory, 0x40),
            process_local_ping_user_pump_kind_code(
                RuntimePingOperationSyscallSubstitutePumpKind::ActivePing
            )
        );
        assert_eq!(
            read_test_le_u64(&user_memory, 0x60),
            process_local_ping_user_step_kind_code(
                PingOperationSyscallSubstituteStepKind::AdvancedToInflight
            )
        );

        let echo_reply = ping_icmp_echo_reply_frame();
        {
            let mut device = PingPollDevice::with_frame(&echo_reply);
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                dispatch_process_local_ping_descriptor_user_arguments(
                    SyscallArguments::new([
                        PROCESS_LOCAL_PING_USER_SELECTOR_PUMP_OR_READ_RESULT,
                        process_descriptor as u64,
                        pump_user,
                        PROCESS_LOCAL_PING_USER_PUMP_RECORD_LEN as u64,
                        0,
                        0,
                    ]),
                    Some(owner),
                    &mut store,
                    &mappings,
                    user_start,
                    &mut user_memory,
                    &mut kernel_scratch,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                ),
                Ok(ProcessLocalPingDispatchOutcome::PumpedOrReadResult)
            );
        }
        assert_eq!(
            read_test_le_u64(&user_memory, 0x60),
            process_local_ping_user_step_kind_code(
                PingOperationSyscallSubstituteStepKind::Completed
            )
        );

        {
            let mut device = PingOutboundTransmitDevice::new();
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                dispatch_process_local_ping_descriptor_user_arguments(
                    SyscallArguments::new([
                        PROCESS_LOCAL_PING_USER_SELECTOR_STATUS,
                        process_descriptor as u64,
                        status_user,
                        PROCESS_LOCAL_PING_USER_STATUS_RECORD_LEN as u64,
                        0,
                        0,
                    ]),
                    Some(owner),
                    &mut store,
                    &mappings,
                    user_start,
                    &mut user_memory,
                    &mut kernel_scratch,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                ),
                Ok(ProcessLocalPingDispatchOutcome::Status)
            );
        }
        assert_eq!(
            read_test_le_u64(&user_memory, 0x90),
            process_local_ping_user_status_kind_code(
                PingOperationSyscallSubstituteStatusKind::Completed
            )
        );
        assert_eq!(&user_memory[0x98..0x9c], &destination);
        assert_eq!(read_test_le_u64(&user_memory, 0xa8), 4);

        {
            let mut device = PingOutboundTransmitDevice::new();
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                dispatch_process_local_ping_descriptor_user_arguments(
                    SyscallArguments::new([
                        PROCESS_LOCAL_PING_USER_SELECTOR_CLOSE,
                        process_descriptor as u64,
                        0,
                        0,
                        0,
                        0,
                    ]),
                    Some(owner),
                    &mut store,
                    &mappings,
                    user_start,
                    &mut user_memory,
                    &mut kernel_scratch,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                ),
                Ok(ProcessLocalPingDispatchOutcome::Closed)
            );
        }
    }

    #[test_case]
    fn process_local_ping_user_arguments_reject_malformed_owner_descriptor_capacity_and_buffers() {
        let owner = crate::scheduler::ProcessOwnerId::new(31).expect("owner id");
        let user_start = 0x0000_0000_0011_0000;
        let destination = [192, 0, 2, 20];
        let mappings = [crate::posix::UserMapping::new(
            user_start,
            0x80,
            crate::posix::UserMappingPermissions::USER_DATA,
        )
        .expect("user data mapping")];
        let mut user_memory = [0u8; 0x80];
        user_memory[..4].copy_from_slice(&[1, 2, 3, 4]);
        let mut scratch = [0u8; 4];
        let mut receive = [0u8; 128];
        let mut transmit = [0u8; 128];
        let mut step = PingOperationSyscallSubstituteStep::from_userspace(
            crate::network::UserspacePingOperationStep::NoFrame,
        );
        let mut pump_step = RuntimePingOperationSyscallSubstitutePumpStep::no_frame();
        let mut status = PingOperationSyscallSubstituteStatus::idle();

        assert_eq!(SyscallNumber::from_raw(6), SyscallNumber::TalosSocket);
        assert_eq!(TALOS_OPEN_SYSCALL, 5);

        let mut no_owner_store = crate::posix::ProcessDescriptorStore::<1, 5>::new_empty();
        let mut no_owner_runtime =
            crate::network::NetworkRuntimeDevicePump::<1, 1, 2, 4>::new(ping_local_endpoint());
        {
            let mut device = PingOutboundTransmitDevice::new();
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                dispatch_process_local_ping_descriptor_user_arguments(
                    SyscallArguments::new([PROCESS_LOCAL_PING_USER_SELECTOR_OPEN, 0, 0, 0, 0, 0]),
                    None,
                    &mut no_owner_store,
                    &mappings,
                    user_start,
                    &mut user_memory,
                    &mut scratch,
                    &mut no_owner_runtime,
                    &mut receive,
                    &mut transmit,
                    &mut device,
                    &mut outputs,
                ),
                Err(PosixError::BadDescriptor)
            );
        }

        let mut full_store = crate::posix::ProcessDescriptorStore::<1, 3>::new_empty();
        full_store
            .create_owner_with_inherited_stdio(owner)
            .expect("create full owner");
        let mut full_runtime =
            crate::network::NetworkRuntimeDevicePump::<1, 1, 2, 4>::new(ping_local_endpoint());
        {
            let mut device = PingOutboundTransmitDevice::new();
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                dispatch_process_local_ping_descriptor_user_arguments(
                    SyscallArguments::new([PROCESS_LOCAL_PING_USER_SELECTOR_OPEN, 0, 0, 0, 0, 0]),
                    Some(owner),
                    &mut full_store,
                    &mappings,
                    user_start,
                    &mut user_memory,
                    &mut scratch,
                    &mut full_runtime,
                    &mut receive,
                    &mut transmit,
                    &mut device,
                    &mut outputs,
                ),
                Err(PosixError::TooManyOpenFiles)
            );
        }

        let mut store = crate::posix::ProcessDescriptorStore::<1, 5>::new_empty();
        store
            .create_owner_with_inherited_stdio(owner)
            .expect("create owner");
        let mut runtime =
            crate::network::NetworkRuntimeDevicePump::<1, 1, 2, 4>::new(ping_local_endpoint());
        {
            let mut device = PingOutboundTransmitDevice::new();
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                dispatch_process_local_ping_descriptor_user_arguments(
                    SyscallArguments::new([99, 0, 0, 0, 0, 0]),
                    Some(owner),
                    &mut store,
                    &mappings,
                    user_start,
                    &mut user_memory,
                    &mut scratch,
                    &mut runtime,
                    &mut receive,
                    &mut transmit,
                    &mut device,
                    &mut outputs,
                ),
                Err(PosixError::InvalidArgument)
            );
            assert_eq!(
                dispatch_process_local_ping_descriptor_user_arguments(
                    SyscallArguments::new([PROCESS_LOCAL_PING_USER_SELECTOR_OPEN, 1, 0, 0, 0, 0]),
                    Some(owner),
                    &mut store,
                    &mappings,
                    user_start,
                    &mut user_memory,
                    &mut scratch,
                    &mut runtime,
                    &mut receive,
                    &mut transmit,
                    &mut device,
                    &mut outputs,
                ),
                Err(PosixError::InvalidArgument)
            );
            assert_eq!(
                dispatch_process_local_ping_descriptor_user_arguments(
                    SyscallArguments::new([
                        PROCESS_LOCAL_PING_USER_SELECTOR_STATUS,
                        99,
                        user_start + 0x20,
                        PROCESS_LOCAL_PING_USER_STATUS_RECORD_LEN as u64,
                        0,
                        0,
                    ]),
                    Some(owner),
                    &mut store,
                    &mappings,
                    user_start,
                    &mut user_memory,
                    &mut scratch,
                    &mut runtime,
                    &mut receive,
                    &mut transmit,
                    &mut device,
                    &mut outputs,
                ),
                Err(PosixError::BadDescriptor)
            );
            assert_eq!(
                dispatch_process_local_ping_descriptor_user_arguments(
                    SyscallArguments::new([
                        PROCESS_LOCAL_PING_USER_SELECTOR_STATUS,
                        99,
                        user_start + 0x20,
                        (PROCESS_LOCAL_PING_USER_STATUS_RECORD_LEN - 1) as u64,
                        0,
                        0,
                    ]),
                    Some(owner),
                    &mut store,
                    &mappings,
                    user_start,
                    &mut user_memory,
                    &mut scratch,
                    &mut runtime,
                    &mut receive,
                    &mut transmit,
                    &mut device,
                    &mut outputs,
                ),
                Err(PosixError::NoSpace)
            );
            assert_eq!(
                dispatch_process_local_ping_descriptor_user_arguments(
                    SyscallArguments::new([
                        PROCESS_LOCAL_PING_USER_SELECTOR_START,
                        3,
                        user_start + 0x1000,
                        4,
                        pack_process_local_ping_user_route(destination, 61, 24),
                        pack_process_local_ping_user_start_control(0x1234, 7, 1),
                    ]),
                    Some(owner),
                    &mut store,
                    &mappings,
                    user_start,
                    &mut user_memory,
                    &mut scratch,
                    &mut runtime,
                    &mut receive,
                    &mut transmit,
                    &mut device,
                    &mut outputs,
                ),
                Err(PosixError::Fault)
            );
            assert_eq!(
                dispatch_process_local_ping_descriptor_user_arguments(
                    SyscallArguments::new([
                        PROCESS_LOCAL_PING_USER_SELECTOR_START,
                        3,
                        user_start,
                        5,
                        pack_process_local_ping_user_route(destination, 61, 24),
                        pack_process_local_ping_user_start_control(0x1234, 7, 1),
                    ]),
                    Some(owner),
                    &mut store,
                    &mappings,
                    user_start,
                    &mut user_memory,
                    &mut scratch,
                    &mut runtime,
                    &mut receive,
                    &mut transmit,
                    &mut device,
                    &mut outputs,
                ),
                Err(PosixError::NoSpace)
            );
            assert_eq!(
                dispatch_process_local_ping_descriptor_user_arguments(
                    SyscallArguments::new([
                        PROCESS_LOCAL_PING_USER_SELECTOR_START,
                        3,
                        user_start,
                        4,
                        pack_process_local_ping_user_route(destination, 0, 24),
                        pack_process_local_ping_user_start_control(0x1234, 7, 1),
                    ]),
                    Some(owner),
                    &mut store,
                    &mappings,
                    user_start,
                    &mut user_memory,
                    &mut scratch,
                    &mut runtime,
                    &mut receive,
                    &mut transmit,
                    &mut device,
                    &mut outputs,
                ),
                Err(PosixError::InvalidArgument)
            );
            assert_eq!(
                dispatch_process_local_ping_descriptor_user_arguments(
                    SyscallArguments::new([
                        PROCESS_LOCAL_PING_USER_SELECTOR_START,
                        3,
                        user_start,
                        4,
                        pack_process_local_ping_user_route(destination, 61, 33),
                        pack_process_local_ping_user_start_control(0x1234, 7, 1),
                    ]),
                    Some(owner),
                    &mut store,
                    &mappings,
                    user_start,
                    &mut user_memory,
                    &mut scratch,
                    &mut runtime,
                    &mut receive,
                    &mut transmit,
                    &mut device,
                    &mut outputs,
                ),
                Err(PosixError::InvalidArgument)
            );
        }
    }

    const VFS_PING_DIAGNOSTIC_PATH: &[u8] = b"/bin/pingdiag";
    const VFS_PING_DIAGNOSTIC_BYTES: &[u8] = b"talos-vfs-ping-diagnostic-svc-fixture-v1";
    static VFS_PING_DIAGNOSTIC_ROOT_ENTRIES: [crate::initramfs::DirectoryEntry; 1] =
        [crate::initramfs::DirectoryEntry::new(b"bin", 1)];
    static VFS_PING_DIAGNOSTIC_BIN_ENTRIES: [crate::initramfs::DirectoryEntry; 1] =
        [crate::initramfs::DirectoryEntry::new(b"pingdiag", 2)];
    static VFS_PING_DIAGNOSTIC_NODES: [crate::initramfs::InitramfsNode; 3] = [
        crate::initramfs::InitramfsNode::directory(0, &VFS_PING_DIAGNOSTIC_ROOT_ENTRIES),
        crate::initramfs::InitramfsNode::directory(1, &VFS_PING_DIAGNOSTIC_BIN_ENTRIES),
        crate::initramfs::InitramfsNode::regular_file(2, VFS_PING_DIAGNOSTIC_BYTES),
    ];

    fn vfs_ping_diagnostic_initramfs() -> crate::initramfs::ReadOnlyInitramfs {
        crate::initramfs::ReadOnlyInitramfs::new(&VFS_PING_DIAGNOSTIC_NODES, 0)
    }

    #[test_case]
    fn vfs_ping_diagnostic_svc_fixture_completes_vfs_backed_user_lifecycle() {
        let owner = crate::scheduler::ProcessOwnerId::new(31).expect("owner id");
        let user_start = 0x0000_0000_0021_0000;
        let payload_user = user_start;
        let pump_user = user_start + 0x40;
        let status_user = user_start + 0x90;
        let destination = [192, 0, 2, 20];
        let payload = [1, 2, 3, 4];
        let mut store = crate::posix::ProcessDescriptorStore::<1, 5>::new_empty();
        store
            .create_owner_with_inherited_stdio(owner)
            .expect("create owner");
        let mappings = [crate::posix::UserMapping::new(
            user_start,
            0x100,
            crate::posix::UserMappingPermissions::USER_DATA,
        )
        .expect("user data mapping")];
        let mut user_memory = [0u8; 0x100];
        let mut kernel_scratch = [0u8; 64];
        let mut runtime =
            crate::network::NetworkRuntimeDevicePump::<2, 1, 2, 4>::new(ping_local_endpoint());
        let mut receive_buffer = [0u8; 128];
        let mut transmit_buffer = [0u8; 128];
        let mut step = PingOperationSyscallSubstituteStep::from_userspace(
            crate::network::UserspacePingOperationStep::NoFrame,
        );
        let mut pump_step = RuntimePingOperationSyscallSubstitutePumpStep::no_frame();
        let mut status = PingOperationSyscallSubstituteStatus::idle();
        let mut fixture = VfsPingDiagnosticSvcFixture::new(
            vfs_ping_diagnostic_initramfs(),
            VFS_PING_DIAGNOSTIC_PATH,
            &mappings,
            user_start,
            &mut user_memory,
            &mut kernel_scratch,
            payload_user,
            pump_user,
            status_user,
        )
        .expect("VFS-backed diagnostic fixture exists");

        assert_eq!(fixture.executable_path(), VFS_PING_DIAGNOSTIC_PATH);
        assert_eq!(fixture.executable_len(), VFS_PING_DIAGNOSTIC_BYTES.len());
        assert_eq!(SyscallNumber::from_raw(6), SyscallNumber::TalosSocket);
        assert_eq!(TALOS_OPEN_SYSCALL, 5);
        fixture.write_payload(&payload).expect("write payload");

        let process_descriptor = {
            let mut device = PingOutboundTransmitDevice::new();
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            match fixture
                .dispatch(
                    fixture.open_arguments(),
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                )
                .expect("open through VFS diagnostic fixture")
            {
                ProcessLocalPingDispatchOutcome::Opened { process_descriptor } => {
                    process_descriptor
                }
                outcome => panic!("unexpected open outcome {outcome:?}"),
            }
        };

        {
            let mut device = PingOutboundTransmitDevice::new();
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                fixture.dispatch(
                    fixture.status_arguments(
                        process_descriptor,
                        PROCESS_LOCAL_PING_USER_STATUS_RECORD_LEN,
                    ),
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                ),
                Ok(ProcessLocalPingDispatchOutcome::Status)
            );
        }
        assert_eq!(
            fixture
                .read_user_u64(status_user)
                .expect("idle status kind copied"),
            process_local_ping_user_status_kind_code(
                PingOperationSyscallSubstituteStatusKind::Idle
            )
        );

        {
            let mut device = PingOutboundTransmitDevice::new();
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                fixture.dispatch(
                    fixture.start_arguments(
                        process_descriptor,
                        payload.len(),
                        destination,
                        61,
                        24,
                        0x1234,
                        7,
                        1,
                    ),
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                ),
                Ok(ProcessLocalPingDispatchOutcome::Started)
            );
            assert!(device.transmitted_len > 0);
        }
        assert_eq!(
            step.kind(),
            PingOperationSyscallSubstituteStepKind::StartedPendingArp
        );

        let arp_reply = ping_arp_reply_frame();
        {
            let mut device = PingPollDevice::with_frame(&arp_reply);
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                fixture.dispatch(
                    fixture.pump_or_read_result_arguments(
                        process_descriptor,
                        PROCESS_LOCAL_PING_USER_PUMP_RECORD_LEN,
                    ),
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                ),
                Ok(ProcessLocalPingDispatchOutcome::PumpedOrReadResult)
            );
        }
        assert_eq!(
            fixture.read_user_u64(pump_user).expect("pump kind copied"),
            process_local_ping_user_pump_kind_code(
                RuntimePingOperationSyscallSubstitutePumpKind::ActivePing
            )
        );
        assert_eq!(
            fixture
                .read_user_u64(pump_user + 32)
                .expect("active step copied"),
            process_local_ping_user_step_kind_code(
                PingOperationSyscallSubstituteStepKind::AdvancedToInflight
            )
        );

        let echo_reply = ping_icmp_echo_reply_frame();
        {
            let mut device = PingPollDevice::with_frame(&echo_reply);
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                fixture.dispatch(
                    fixture.pump_or_read_result_arguments(
                        process_descriptor,
                        PROCESS_LOCAL_PING_USER_PUMP_RECORD_LEN,
                    ),
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                ),
                Ok(ProcessLocalPingDispatchOutcome::PumpedOrReadResult)
            );
        }
        assert_eq!(
            fixture
                .read_user_u64(pump_user + 32)
                .expect("completed step copied"),
            process_local_ping_user_step_kind_code(
                PingOperationSyscallSubstituteStepKind::Completed
            )
        );

        {
            let mut device = PingOutboundTransmitDevice::new();
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                fixture.dispatch(
                    fixture.status_arguments(
                        process_descriptor,
                        PROCESS_LOCAL_PING_USER_STATUS_RECORD_LEN,
                    ),
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                ),
                Ok(ProcessLocalPingDispatchOutcome::Status)
            );
        }
        assert_eq!(
            fixture
                .read_user_u64(status_user)
                .expect("completed status kind copied"),
            process_local_ping_user_status_kind_code(
                PingOperationSyscallSubstituteStatusKind::Completed
            )
        );
        assert_eq!(
            fixture
                .read_user_bytes(status_user + 8, 4)
                .expect("destination copied"),
            destination.as_slice()
        );
        assert_eq!(
            fixture
                .read_user_u64(status_user + 24)
                .expect("payload length copied"),
            payload.len() as u64
        );

        {
            let mut device = PingOutboundTransmitDevice::new();
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                fixture.dispatch(
                    fixture.close_arguments(process_descriptor),
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                ),
                Ok(ProcessLocalPingDispatchOutcome::Closed)
            );
        }
    }

    #[test_case]
    fn vfs_ping_diagnostic_svc_fixture_records_packet_queue_lifecycle() {
        let owner = crate::scheduler::ProcessOwnerId::new(31).expect("owner id");
        let user_start = 0x0000_0000_0023_0000;
        let payload_user = user_start;
        let pump_user = user_start + 0x40;
        let status_user = user_start + 0x90;
        let destination = [192, 0, 2, 20];
        let payload = [1, 2, 3, 4];
        let mut store = crate::posix::ProcessDescriptorStore::<1, 5>::new_empty();
        store
            .create_owner_with_inherited_stdio(owner)
            .expect("create owner");
        let mappings = [crate::posix::UserMapping::new(
            user_start,
            0x100,
            crate::posix::UserMappingPermissions::USER_DATA,
        )
        .expect("user data mapping")];
        let mut user_memory = [0u8; 0x100];
        let mut kernel_scratch = [0u8; 64];
        let mut runtime =
            crate::network::NetworkRuntimeDevicePump::<2, 1, 2, 4>::new(ping_local_endpoint());
        let mut receive_buffer = [0u8; 128];
        let mut transmit_buffer = [0u8; 128];
        let mut step = PingOperationSyscallSubstituteStep::from_userspace(
            crate::network::UserspacePingOperationStep::NoFrame,
        );
        let mut pump_step = RuntimePingOperationSyscallSubstitutePumpStep::no_frame();
        let mut status = PingOperationSyscallSubstituteStatus::idle();
        let mut queue = crate::network::PacketQueueNetworkDevice::<2, 2, 128>::new();
        let mut driver = crate::network::PacketQueueNetworkDevice::<2, 2, 128>::new();
        let mut driver_receive_buffer = [0u8; 128];
        let mut fixture = VfsPingDiagnosticSvcFixture::new(
            vfs_ping_diagnostic_initramfs(),
            VFS_PING_DIAGNOSTIC_PATH,
            &mappings,
            user_start,
            &mut user_memory,
            &mut kernel_scratch,
            payload_user,
            pump_user,
            status_user,
        )
        .expect("VFS-backed diagnostic fixture exists");

        assert_eq!(SyscallNumber::from_raw(6), SyscallNumber::TalosSocket);
        assert_eq!(TALOS_OPEN_SYSCALL, 5);
        fixture.write_payload(&payload).expect("write payload");

        let process_descriptor = {
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            match fixture
                .dispatch(
                    fixture.open_arguments(),
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut queue,
                    &mut outputs,
                )
                .expect("open through VFS diagnostic fixture")
            {
                ProcessLocalPingDispatchOutcome::Opened { process_descriptor } => {
                    process_descriptor
                }
                outcome => panic!("unexpected open outcome {outcome:?}"),
            }
        };

        {
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                fixture.dispatch(
                    fixture.start_arguments(
                        process_descriptor,
                        payload.len(),
                        destination,
                        61,
                        24,
                        0x1234,
                        7,
                        1,
                    ),
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut queue,
                    &mut outputs,
                ),
                Ok(ProcessLocalPingDispatchOutcome::Started)
            );
        }
        assert_eq!(
            step.kind(),
            PingOperationSyscallSubstituteStepKind::StartedPendingArp
        );
        assert_eq!(queue.transmitted_len(), 1);
        assert_eq!(
            queue.pump_driver(&mut driver, &mut driver_receive_buffer),
            crate::network::PacketQueueDriverPumpStep::Transmitted {
                frame_len: crate::network::ETHERNET_HEADER_LEN
                    + crate::network::ARP_ETHERNET_IPV4_LEN,
            }
        );
        let outbound_arp = driver.pop_transmitted().expect("outbound ARP request");
        let outbound_arp =
            crate::network::EthernetFrame::parse(outbound_arp.as_bytes()).expect("parse ARP");
        assert_eq!(outbound_arp.ether_type(), crate::network::EtherType::Arp);

        driver
            .inject_received(&ping_arp_reply_frame())
            .expect("driver receives ARP reply");
        assert_eq!(
            queue.pump_driver(&mut driver, &mut driver_receive_buffer),
            crate::network::PacketQueueDriverPumpStep::Received {
                frame_len: crate::network::ETHERNET_HEADER_LEN
                    + crate::network::ARP_ETHERNET_IPV4_LEN,
            }
        );
        {
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                fixture.dispatch(
                    fixture.pump_or_read_result_arguments(
                        process_descriptor,
                        PROCESS_LOCAL_PING_USER_PUMP_RECORD_LEN,
                    ),
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut queue,
                    &mut outputs,
                ),
                Ok(ProcessLocalPingDispatchOutcome::PumpedOrReadResult)
            );
        }
        assert_eq!(
            pump_step.active_ping_step().kind(),
            PingOperationSyscallSubstituteStepKind::AdvancedToInflight
        );
        assert_eq!(
            queue.pump_driver(&mut driver, &mut driver_receive_buffer),
            crate::network::PacketQueueDriverPumpStep::Transmitted {
                frame_len: crate::network::ETHERNET_HEADER_LEN
                    + crate::network::IPV4_MIN_HEADER_LEN
                    + crate::network::ICMP_ECHO_HEADER_LEN
                    + payload.len(),
            }
        );
        let outbound_icmp = driver
            .pop_transmitted()
            .expect("outbound ICMP echo request");
        let outbound_icmp =
            crate::network::EthernetFrame::parse(outbound_icmp.as_bytes()).expect("parse ICMP");
        assert_eq!(outbound_icmp.ether_type(), crate::network::EtherType::Ipv4);
        assert_eq!(
            crate::network::Ipv4Packet::parse(outbound_icmp.payload())
                .expect("parse IPv4")
                .protocol(),
            crate::network::IPV4_PROTOCOL_ICMP
        );

        driver
            .inject_received(&ping_icmp_echo_reply_frame())
            .expect("driver receives ICMP echo reply");
        assert_eq!(
            queue.pump_driver(&mut driver, &mut driver_receive_buffer),
            crate::network::PacketQueueDriverPumpStep::Received {
                frame_len: crate::network::ETHERNET_HEADER_LEN
                    + crate::network::IPV4_MIN_HEADER_LEN
                    + crate::network::ICMP_ECHO_HEADER_LEN
                    + payload.len(),
            }
        );
        {
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                fixture.dispatch(
                    fixture.pump_or_read_result_arguments(
                        process_descriptor,
                        PROCESS_LOCAL_PING_USER_PUMP_RECORD_LEN,
                    ),
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut queue,
                    &mut outputs,
                ),
                Ok(ProcessLocalPingDispatchOutcome::PumpedOrReadResult)
            );
        }
        assert_eq!(
            pump_step.active_ping_step().kind(),
            PingOperationSyscallSubstituteStepKind::Completed
        );
        assert_eq!(queue.received_len(), 0);

        {
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                fixture.dispatch(
                    fixture.status_arguments(
                        process_descriptor,
                        PROCESS_LOCAL_PING_USER_STATUS_RECORD_LEN,
                    ),
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut queue,
                    &mut outputs,
                ),
                Ok(ProcessLocalPingDispatchOutcome::Status)
            );
        }
        assert_eq!(
            fixture
                .read_user_u64(status_user)
                .expect("completed status kind copied"),
            process_local_ping_user_status_kind_code(
                PingOperationSyscallSubstituteStatusKind::Completed
            )
        );

        {
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                fixture.dispatch(
                    fixture.close_arguments(process_descriptor),
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut queue,
                    &mut outputs,
                ),
                Ok(ProcessLocalPingDispatchOutcome::Closed)
            );
        }
    }

    #[test_case]
    fn vfs_ping_diagnostic_svc_fixture_maps_packet_queue_controls() {
        let owner = crate::scheduler::ProcessOwnerId::new(31).expect("owner id");
        let user_start = 0x0000_0000_0024_0000;
        let payload_user = user_start;
        let pump_user = user_start + 0x40;
        let status_user = user_start + 0x90;
        let destination = [192, 0, 2, 20];
        let mappings = [crate::posix::UserMapping::new(
            user_start,
            0x100,
            crate::posix::UserMappingPermissions::USER_DATA,
        )
        .expect("user data mapping")];
        let mut user_memory = [0u8; 0x100];
        let mut kernel_scratch = [0u8; 64];
        let mut runtime =
            crate::network::NetworkRuntimeDevicePump::<2, 1, 2, 4>::new(ping_local_endpoint());
        let mut receive_buffer = [0u8; 128];
        let mut transmit_buffer = [0u8; 128];
        let mut step = PingOperationSyscallSubstituteStep::from_userspace(
            crate::network::UserspacePingOperationStep::NoFrame,
        );
        let mut pump_step = RuntimePingOperationSyscallSubstitutePumpStep::no_frame();
        let mut status = PingOperationSyscallSubstituteStatus::idle();
        let mut store = crate::posix::ProcessDescriptorStore::<1, 5>::new_empty();
        store
            .create_owner_with_inherited_stdio(owner)
            .expect("create owner");
        let mut fixture = VfsPingDiagnosticSvcFixture::new(
            vfs_ping_diagnostic_initramfs(),
            VFS_PING_DIAGNOSTIC_PATH,
            &mappings,
            user_start,
            &mut user_memory,
            &mut kernel_scratch,
            payload_user,
            pump_user,
            status_user,
        )
        .expect("VFS-backed diagnostic fixture exists");
        fixture.write_payload(&[1, 2, 3, 4]).expect("write payload");

        let mut tiny_queue = crate::network::PacketQueueNetworkDevice::<1, 1, 2>::new();
        assert_eq!(
            tiny_queue.inject_received(&[0xde, 0xad, 0xbe]),
            Err(crate::network::PacketQueueError::FrameTooLarge {
                required_len: 3,
                max_len: 2,
            })
        );

        let process_descriptor = {
            let mut queue = crate::network::PacketQueueNetworkDevice::<1, 1, 128>::new();
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            match fixture
                .dispatch(
                    fixture.open_arguments(),
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut queue,
                    &mut outputs,
                )
                .expect("open through VFS diagnostic fixture")
            {
                ProcessLocalPingDispatchOutcome::Opened { process_descriptor } => {
                    process_descriptor
                }
                outcome => panic!("unexpected open outcome {outcome:?}"),
            }
        };

        {
            let mut full_queue = crate::network::PacketQueueNetworkDevice::<1, 0, 128>::new();
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                fixture.dispatch(
                    fixture.start_arguments(
                        process_descriptor,
                        4,
                        destination,
                        61,
                        24,
                        0x1234,
                        7,
                        1,
                    ),
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut full_queue,
                    &mut outputs,
                ),
                Err(PosixError::NoSpace)
            );
        }

        {
            let mut error_queue = crate::network::PacketQueueNetworkDevice::<1, 1, 128>::new();
            error_queue.set_transmit_error(Some(crate::network::DeviceError::Io));
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                fixture.dispatch(
                    fixture.start_arguments(
                        process_descriptor,
                        4,
                        destination,
                        61,
                        24,
                        0x1234,
                        7,
                        1,
                    ),
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut error_queue,
                    &mut outputs,
                ),
                Err(PosixError::Io)
            );
            error_queue.set_transmit_error(None);
        }

        {
            let mut small_transmit = [0u8; 8];
            let mut queue = crate::network::PacketQueueNetworkDevice::<1, 1, 128>::new();
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                fixture.dispatch(
                    fixture.start_arguments(
                        process_descriptor,
                        4,
                        destination,
                        61,
                        24,
                        0x1234,
                        7,
                        1,
                    ),
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut small_transmit,
                    &mut queue,
                    &mut outputs,
                ),
                Err(PosixError::NoSpace)
            );
        }

        {
            let mut queue = crate::network::PacketQueueNetworkDevice::<2, 2, 128>::new();
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                fixture.dispatch(
                    fixture.start_arguments(
                        process_descriptor,
                        4,
                        destination,
                        61,
                        24,
                        0x1234,
                        7,
                        1,
                    ),
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut queue,
                    &mut outputs,
                ),
                Ok(ProcessLocalPingDispatchOutcome::Started)
            );
            let _ = queue.pop_transmitted().expect("queued outbound ARP");
            queue
                .inject_received(&[0xde, 0xad])
                .expect("inject malformed");
            assert_eq!(
                fixture.dispatch(
                    fixture.pump_or_read_result_arguments(
                        process_descriptor,
                        PROCESS_LOCAL_PING_USER_PUMP_RECORD_LEN,
                    ),
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut queue,
                    &mut outputs,
                ),
                Err(PosixError::InvalidArgument)
            );
            assert_eq!(
                fixture.dispatch(
                    fixture.retry_arp_arguments(process_descriptor),
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut queue,
                    &mut outputs,
                ),
                Ok(ProcessLocalPingDispatchOutcome::RetriedArp)
            );
            let retry_frame = queue.pop_transmitted().expect("retry ARP frame");
            assert_eq!(
                crate::network::EthernetFrame::parse(retry_frame.as_bytes())
                    .expect("retry ARP parses")
                    .ether_type(),
                crate::network::EtherType::Arp
            );
            queue.set_receive_error(Some(crate::network::DeviceError::Io));
            assert_eq!(
                fixture.dispatch(
                    fixture.pump_or_read_result_arguments(
                        process_descriptor,
                        PROCESS_LOCAL_PING_USER_PUMP_RECORD_LEN,
                    ),
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut queue,
                    &mut outputs,
                ),
                Err(PosixError::Io)
            );
            queue.set_receive_error(None);
            assert_eq!(
                fixture.dispatch(
                    fixture.timeout_arguments(process_descriptor),
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut queue,
                    &mut outputs,
                ),
                Ok(ProcessLocalPingDispatchOutcome::TimedOut)
            );
        }

        {
            let mut queue = crate::network::PacketQueueNetworkDevice::<1, 1, 128>::new();
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                fixture.dispatch(
                    fixture.status_arguments(99, PROCESS_LOCAL_PING_USER_STATUS_RECORD_LEN),
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut queue,
                    &mut outputs,
                ),
                Err(PosixError::BadDescriptor)
            );
        }
    }

    #[test_case]
    fn vfs_ping_diagnostic_svc_fixture_maps_contract_error_controls() {
        let owner = crate::scheduler::ProcessOwnerId::new(31).expect("owner id");
        let user_start = 0x0000_0000_0022_0000;
        let payload_user = user_start;
        let pump_user = user_start + 0x40;
        let status_user = user_start + 0x90;
        let destination = [192, 0, 2, 20];
        let mappings = [crate::posix::UserMapping::new(
            user_start,
            0x100,
            crate::posix::UserMappingPermissions::USER_DATA,
        )
        .expect("user data mapping")];
        let mut user_memory = [0u8; 0x100];
        let mut kernel_scratch = [0u8; 4];
        let mut receive_buffer = [0u8; 128];
        let mut transmit_buffer = [0u8; 128];
        let mut step = PingOperationSyscallSubstituteStep::from_userspace(
            crate::network::UserspacePingOperationStep::NoFrame,
        );
        let mut pump_step = RuntimePingOperationSyscallSubstitutePumpStep::no_frame();
        let mut status = PingOperationSyscallSubstituteStatus::idle();

        let mut missing_user_memory = [0u8; 0x100];
        let mut missing_kernel_scratch = [0u8; 4];
        assert_eq!(
            VfsPingDiagnosticSvcFixture::new(
                vfs_ping_diagnostic_initramfs(),
                b"/bin/missing",
                &mappings,
                user_start,
                &mut missing_user_memory,
                &mut missing_kernel_scratch,
                payload_user,
                pump_user,
                status_user,
            )
            .map(|_| ()),
            Err(PosixError::NoEntry)
        );

        let mut fixture = VfsPingDiagnosticSvcFixture::new(
            vfs_ping_diagnostic_initramfs(),
            VFS_PING_DIAGNOSTIC_PATH,
            &mappings,
            user_start,
            &mut user_memory,
            &mut kernel_scratch,
            payload_user,
            pump_user,
            status_user,
        )
        .expect("VFS-backed diagnostic fixture exists");
        fixture.write_payload(&[1, 2, 3, 4]).expect("payload");

        let mut no_owner_store = crate::posix::ProcessDescriptorStore::<1, 5>::new_empty();
        let mut no_owner_runtime =
            crate::network::NetworkRuntimeDevicePump::<1, 1, 2, 4>::new(ping_local_endpoint());
        {
            let mut device = PingOutboundTransmitDevice::new();
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                fixture.dispatch(
                    fixture.open_arguments(),
                    None,
                    &mut no_owner_store,
                    &mut no_owner_runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                ),
                Err(PosixError::BadDescriptor)
            );
        }

        let mut full_store = crate::posix::ProcessDescriptorStore::<1, 3>::new_empty();
        full_store
            .create_owner_with_inherited_stdio(owner)
            .expect("create full owner");
        let mut full_runtime =
            crate::network::NetworkRuntimeDevicePump::<1, 1, 2, 4>::new(ping_local_endpoint());
        {
            let mut device = PingOutboundTransmitDevice::new();
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                fixture.dispatch(
                    fixture.open_arguments(),
                    Some(owner),
                    &mut full_store,
                    &mut full_runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                ),
                Err(PosixError::TooManyOpenFiles)
            );
        }

        let mut store = crate::posix::ProcessDescriptorStore::<1, 5>::new_empty();
        store
            .create_owner_with_inherited_stdio(owner)
            .expect("create owner");
        let mut runtime =
            crate::network::NetworkRuntimeDevicePump::<1, 1, 2, 4>::new(ping_local_endpoint());
        {
            let mut device = PingOutboundTransmitDevice::new();
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                fixture.dispatch(
                    SyscallArguments::new([99, 0, 0, 0, 0, 0]),
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                ),
                Err(PosixError::InvalidArgument)
            );
        }

        let process_descriptor = {
            let mut device = PingOutboundTransmitDevice::new();
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            match fixture
                .dispatch(
                    fixture.open_arguments(),
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                )
                .expect("open descriptor")
            {
                ProcessLocalPingDispatchOutcome::Opened { process_descriptor } => {
                    process_descriptor
                }
                outcome => panic!("unexpected open outcome {outcome:?}"),
            }
        };

        {
            let mut device = PingOutboundTransmitDevice::new();
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                fixture.dispatch(
                    fixture.open_arguments(),
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                ),
                Err(PosixError::Busy)
            );
            assert_eq!(
                fixture.dispatch(
                    fixture.status_arguments(
                        process_descriptor,
                        PROCESS_LOCAL_PING_USER_STATUS_RECORD_LEN - 1,
                    ),
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                ),
                Err(PosixError::NoSpace)
            );
            assert_eq!(
                fixture.dispatch(
                    fixture.status_arguments(99, PROCESS_LOCAL_PING_USER_STATUS_RECORD_LEN),
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                ),
                Err(PosixError::BadDescriptor)
            );
            assert_eq!(
                fixture.dispatch(
                    SyscallArguments::new([
                        PROCESS_LOCAL_PING_USER_SELECTOR_START,
                        process_descriptor as u64,
                        user_start + 0x1000,
                        4,
                        pack_process_local_ping_user_route(destination, 61, 24),
                        pack_process_local_ping_user_start_control(0x1234, 7, 1),
                    ]),
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                ),
                Err(PosixError::Fault)
            );
            assert_eq!(
                fixture.dispatch(
                    fixture.start_arguments(
                        process_descriptor,
                        5,
                        destination,
                        61,
                        24,
                        0x1234,
                        7,
                        1
                    ),
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                ),
                Err(PosixError::NoSpace)
            );
            assert_eq!(
                fixture.dispatch(
                    fixture.start_arguments(
                        process_descriptor,
                        4,
                        destination,
                        0,
                        24,
                        0x1234,
                        7,
                        1
                    ),
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                ),
                Err(PosixError::InvalidArgument)
            );
        }

        {
            let mut device = PingOutboundTransmitDevice::new();
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                fixture.dispatch(
                    fixture.start_arguments(
                        process_descriptor,
                        4,
                        destination,
                        61,
                        24,
                        0x1234,
                        7,
                        0
                    ),
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                ),
                Ok(ProcessLocalPingDispatchOutcome::Started)
            );
            assert_eq!(
                fixture.dispatch(
                    fixture.retry_arp_arguments(process_descriptor),
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                ),
                Err(PosixError::Again)
            );
            assert_eq!(
                fixture.dispatch(
                    fixture.timeout_arguments(process_descriptor),
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                ),
                Ok(ProcessLocalPingDispatchOutcome::TimedOut)
            );
            assert_eq!(
                fixture.dispatch(
                    fixture.close_arguments(process_descriptor),
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                ),
                Ok(ProcessLocalPingDispatchOutcome::Closed)
            );
            assert_eq!(
                fixture.dispatch(
                    fixture.close_arguments(process_descriptor),
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                ),
                Err(PosixError::BadDescriptor)
            );
        }
        assert_eq!(
            step.kind(),
            PingOperationSyscallSubstituteStepKind::TimedOut
        );

        let process_descriptor = {
            let mut device = PingOutboundTransmitDevice::new();
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            match fixture
                .dispatch(
                    fixture.open_arguments(),
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                )
                .expect("reopen descriptor")
            {
                ProcessLocalPingDispatchOutcome::Opened { process_descriptor } => {
                    process_descriptor
                }
                outcome => panic!("unexpected open outcome {outcome:?}"),
            }
        };

        let local_arp_request = ping_local_arp_request_frame();
        {
            let mut small_receive = [0u8; 8];
            let mut device = PingPollDevice::with_frame(&local_arp_request);
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                fixture.dispatch(
                    fixture.pump_or_read_result_arguments(
                        process_descriptor,
                        PROCESS_LOCAL_PING_USER_PUMP_RECORD_LEN,
                    ),
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut small_receive,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                ),
                Err(PosixError::NoSpace)
            );
        }
        {
            let mut device = PingPollDevice::with_receive_error(crate::network::DeviceError::Io);
            let mut outputs =
                ProcessLocalPingDispatchOutputs::new(&mut step, &mut pump_step, &mut status);
            assert_eq!(
                fixture.dispatch(
                    fixture.pump_or_read_result_arguments(
                        process_descriptor,
                        PROCESS_LOCAL_PING_USER_PUMP_RECORD_LEN,
                    ),
                    Some(owner),
                    &mut store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut device,
                    &mut outputs,
                ),
                Err(PosixError::Io)
            );
        }
    }

    fn pack_process_local_ping_user_route(destination: [u8; 4], ttl: u8, prefix_len: u8) -> u64 {
        u32::from_be_bytes(destination) as u64 | ((ttl as u64) << 32) | ((prefix_len as u64) << 40)
    }

    fn pack_process_local_ping_user_start_control(
        identifier: u16,
        sequence_number: u16,
        arp_retry_budget: u32,
    ) -> u64 {
        identifier as u64 | ((sequence_number as u64) << 16) | ((arp_retry_budget as u64) << 32)
    }

    fn read_test_le_u64(bytes: &[u8], offset: usize) -> u64 {
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&bytes[offset..offset + 8]);
        u64::from_le_bytes(raw)
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
    fn talos_socket_opens_af_inet_stream_descriptor_and_close_drops_backing() {
        let owner = crate::scheduler::ProcessOwnerId::new(31).expect("owner id");
        let mut store = crate::posix::ProcessDescriptorStore::<1, 5>::new_empty();
        store
            .create_owner_with_inherited_stdio(owner)
            .expect("create owner");
        let mut sockets = crate::network::NetworkSocketDescriptorTable::<1>::new();
        let mut user_memory = [0u8; 128];

        let open = dispatch_socket_case(
            TALOS_SOCKET_SYSCALL,
            SyscallArguments::new([
                crate::network::SOCKET_DOMAIN_AF_INET,
                crate::network::SOCKET_TYPE_STREAM,
                crate::network::SOCKET_PROTOCOL_DEFAULT,
                0,
                0,
                0,
            ]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );

        assert_eq!(open.number(), SyscallNumber::TalosSocket);
        assert_eq!(open.return_value().x0(), 3);
        let entry = store
            .current_descriptor_table(Some(owner))
            .expect("current table")
            .get(3)
            .expect("socket fd");
        assert_eq!(entry.access(), crate::posix::DescriptorAccess::ReadWrite);
        assert_eq!(
            entry.object().kind(),
            crate::posix::DescriptorObjectKind::Socket
        );
        assert_eq!(entry.object().reference(), 0);
        let socket = sockets
            .socket(crate::network::NetworkSocketDescriptor::from_raw(0))
            .expect("socket backing");
        assert_eq!(socket.owner(), owner);
        assert_eq!(socket.domain(), crate::network::SOCKET_DOMAIN_AF_INET);
        assert_eq!(socket.socket_type(), crate::network::SOCKET_TYPE_STREAM);
        assert_eq!(socket.protocol(), crate::network::SOCKET_PROTOCOL_DEFAULT);

        let close = dispatch_socket_case(
            TALOS_CLOSE_SYSCALL,
            SyscallArguments::new([3, 0, 0, 0, 0, 0]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        let double_close = dispatch_socket_case(
            TALOS_CLOSE_SYSCALL,
            SyscallArguments::new([3, 0, 0, 0, 0, 0]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );

        assert_eq!(close.return_value().x0(), 0);
        assert_eq!(
            store
                .current_descriptor_table(Some(owner))
                .expect("current table")
                .get(3),
            Err(PosixError::BadDescriptor)
        );
        assert_eq!(
            sockets.socket(crate::network::NetworkSocketDescriptor::from_raw(0)),
            Err(PosixError::BadDescriptor)
        );
        assert_eq!(
            double_close.return_value().x0(),
            (EBADF as u64).wrapping_neg()
        );
    }

    #[test_case]
    fn talos_socket_errors_are_deterministic_and_do_not_allocate_on_failure() {
        let owner = crate::scheduler::ProcessOwnerId::new(31).expect("owner id");
        let mut no_owner_store = crate::posix::ProcessDescriptorStore::<1, 5>::new_empty();
        let mut no_owner_sockets = crate::network::NetworkSocketDescriptorTable::<1>::new();
        let mut no_owner_memory = [0u8; 128];
        let no_owner = dispatch_socket_case(
            TALOS_SOCKET_SYSCALL,
            SyscallArguments::new([
                crate::network::SOCKET_DOMAIN_AF_INET,
                crate::network::SOCKET_TYPE_STREAM,
                crate::network::SOCKET_PROTOCOL_DEFAULT,
                0,
                0,
                0,
            ]),
            None,
            &mut no_owner_store,
            &mut no_owner_sockets,
            &mut no_owner_memory,
        );
        assert_eq!(no_owner.return_value().x0(), (EBADF as u64).wrapping_neg());

        let mut full_store = crate::posix::ProcessDescriptorStore::<1, 3>::new_empty();
        full_store
            .create_owner_with_inherited_stdio(owner)
            .expect("create full owner");
        let mut full_sockets = crate::network::NetworkSocketDescriptorTable::<1>::new();
        let mut full_memory = [0u8; 128];
        let full = dispatch_socket_case(
            TALOS_SOCKET_SYSCALL,
            SyscallArguments::new([
                crate::network::SOCKET_DOMAIN_AF_INET,
                crate::network::SOCKET_TYPE_STREAM,
                crate::network::SOCKET_PROTOCOL_DEFAULT,
                0,
                0,
                0,
            ]),
            Some(owner),
            &mut full_store,
            &mut full_sockets,
            &mut full_memory,
        );
        assert_eq!(full.return_value().x0(), (EMFILE as u64).wrapping_neg());
        assert_eq!(
            full_sockets.socket(crate::network::NetworkSocketDescriptor::from_raw(0)),
            Err(PosixError::BadDescriptor)
        );

        let mut store = crate::posix::ProcessDescriptorStore::<1, 5>::new_empty();
        store
            .create_owner_with_inherited_stdio(owner)
            .expect("create owner");
        let mut sockets = crate::network::NetworkSocketDescriptorTable::<1>::new();
        let mut user_memory = [0u8; 128];
        let reserved = dispatch_socket_case(
            TALOS_SOCKET_SYSCALL,
            SyscallArguments::new([
                crate::network::SOCKET_DOMAIN_AF_INET,
                crate::network::SOCKET_TYPE_STREAM,
                crate::network::SOCKET_PROTOCOL_DEFAULT,
                1,
                0,
                0,
            ]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        let unsupported_domain = dispatch_socket_case(
            TALOS_SOCKET_SYSCALL,
            SyscallArguments::new([
                10,
                crate::network::SOCKET_TYPE_STREAM,
                crate::network::SOCKET_PROTOCOL_DEFAULT,
                0,
                0,
                0,
            ]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        let unsupported_type = dispatch_socket_case(
            TALOS_SOCKET_SYSCALL,
            SyscallArguments::new([
                crate::network::SOCKET_DOMAIN_AF_INET,
                2,
                crate::network::SOCKET_PROTOCOL_DEFAULT,
                0,
                0,
                0,
            ]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        let unsupported_protocol = dispatch_socket_case(
            TALOS_SOCKET_SYSCALL,
            SyscallArguments::new([
                crate::network::SOCKET_DOMAIN_AF_INET,
                crate::network::SOCKET_TYPE_STREAM,
                6,
                0,
                0,
                0,
            ]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        let first = dispatch_socket_case(
            TALOS_SOCKET_SYSCALL,
            SyscallArguments::new([
                crate::network::SOCKET_DOMAIN_AF_INET,
                crate::network::SOCKET_TYPE_STREAM,
                crate::network::SOCKET_PROTOCOL_DEFAULT,
                0,
                0,
                0,
            ]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        let backing_full = dispatch_socket_case(
            TALOS_SOCKET_SYSCALL,
            SyscallArguments::new([
                crate::network::SOCKET_DOMAIN_AF_INET,
                crate::network::SOCKET_TYPE_STREAM,
                crate::network::SOCKET_PROTOCOL_DEFAULT,
                0,
                0,
                0,
            ]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );

        assert_eq!(reserved.return_value().x0(), (EINVAL as u64).wrapping_neg());
        assert_eq!(
            unsupported_domain.return_value().x0(),
            (ENOTSUP as u64).wrapping_neg()
        );
        assert_eq!(
            unsupported_type.return_value().x0(),
            (ENOTSUP as u64).wrapping_neg()
        );
        assert_eq!(
            unsupported_protocol.return_value().x0(),
            (ENOTSUP as u64).wrapping_neg()
        );
        assert_eq!(first.return_value().x0(), 3);
        assert_eq!(
            backing_full.return_value().x0(),
            (ENOSPC as u64).wrapping_neg()
        );
        assert_eq!(
            store
                .current_descriptor_table(Some(owner))
                .expect("current table")
                .get(4),
            Err(PosixError::BadDescriptor)
        );
    }

    #[test_case]
    fn talos_bind_listen_records_socket_state_and_close_drops_backing() {
        let owner = crate::scheduler::ProcessOwnerId::new(31).expect("owner id");
        let mut store = crate::posix::ProcessDescriptorStore::<1, 5>::new_empty();
        store
            .create_owner_with_inherited_stdio(owner)
            .expect("create owner");
        let mut sockets = crate::network::NetworkSocketDescriptorTable::<1>::new();
        let mut user_memory = [0u8; 128];
        let endpoint = crate::network::Ipv4Endpoint::new(0x7f00_0001, 8080);

        let open = dispatch_socket_case(
            TALOS_SOCKET_SYSCALL,
            SyscallArguments::new([
                crate::network::SOCKET_DOMAIN_AF_INET,
                crate::network::SOCKET_TYPE_STREAM,
                crate::network::SOCKET_PROTOCOL_DEFAULT,
                0,
                0,
                0,
            ]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        assert_eq!(open.return_value().x0(), 3);
        assert_eq!(
            sockets
                .socket(crate::network::NetworkSocketDescriptor::from_raw(0))
                .expect("socket backing")
                .state(),
            crate::network::NetworkSocketState::OpenUnbound
        );

        let bind = dispatch_socket_case(
            TALOS_BIND_SYSCALL,
            SyscallArguments::new([
                3,
                endpoint.ipv4_be() as u64,
                endpoint.port() as u64,
                0,
                0,
                0,
            ]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        let repeated_bind = dispatch_socket_case(
            TALOS_BIND_SYSCALL,
            SyscallArguments::new([
                3,
                endpoint.ipv4_be() as u64,
                endpoint.port() as u64,
                0,
                0,
                0,
            ]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        let listen = dispatch_socket_case(
            TALOS_LISTEN_SYSCALL,
            SyscallArguments::new([3, 2, 0, 0, 0, 0]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        let repeated_listen = dispatch_socket_case(
            TALOS_LISTEN_SYSCALL,
            SyscallArguments::new([3, 4, 0, 0, 0, 0]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );

        assert_eq!(bind.number(), SyscallNumber::TalosBind);
        assert_eq!(bind.return_value().x0(), 0);
        assert_eq!(
            repeated_bind.return_value().x0(),
            (EINVAL as u64).wrapping_neg()
        );
        assert_eq!(listen.number(), SyscallNumber::TalosListen);
        assert_eq!(listen.return_value().x0(), 0);
        assert_eq!(repeated_listen.return_value().x0(), 0);
        assert_eq!(
            sockets
                .socket(crate::network::NetworkSocketDescriptor::from_raw(0))
                .expect("socket backing")
                .state(),
            crate::network::NetworkSocketState::Listening {
                local_endpoint: endpoint,
                backlog: 4,
                pending: crate::network::NetworkSocketPendingQueue::new(),
            }
        );

        let close = dispatch_socket_case(
            TALOS_CLOSE_SYSCALL,
            SyscallArguments::new([3, 0, 0, 0, 0, 0]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        assert_eq!(close.return_value().x0(), 0);
        assert_eq!(
            sockets.socket(crate::network::NetworkSocketDescriptor::from_raw(0)),
            Err(PosixError::BadDescriptor)
        );
    }

    #[test_case]
    fn talos_connect_accept_records_local_handshake_state() {
        let owner = crate::scheduler::ProcessOwnerId::new(31).expect("owner id");
        let mut store = crate::posix::ProcessDescriptorStore::<1, 7>::new_empty();
        store
            .create_owner_with_inherited_stdio(owner)
            .expect("create owner");
        let mut sockets = crate::network::NetworkSocketDescriptorTable::<4>::new();
        let mut user_memory = [0u8; 128];
        let listener_endpoint = crate::network::Ipv4Endpoint::new(0x7f00_0001, 8080);
        let client_endpoint = crate::network::Ipv4Endpoint::new(
            crate::network::SOCKET_SYNTHETIC_LOCAL_IPV4_BE,
            crate::network::SOCKET_SYNTHETIC_CLIENT_PORT_BASE + 1,
        );

        let listener_fd = dispatch_socket_case(
            TALOS_SOCKET_SYSCALL,
            SyscallArguments::new([
                crate::network::SOCKET_DOMAIN_AF_INET,
                crate::network::SOCKET_TYPE_STREAM,
                crate::network::SOCKET_PROTOCOL_DEFAULT,
                0,
                0,
                0,
            ]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        assert_eq!(listener_fd.return_value().x0(), 3);
        let bind = dispatch_socket_case(
            TALOS_BIND_SYSCALL,
            SyscallArguments::new([
                3,
                listener_endpoint.ipv4_be() as u64,
                listener_endpoint.port() as u64,
                0,
                0,
                0,
            ]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        let listen = dispatch_socket_case(
            TALOS_LISTEN_SYSCALL,
            SyscallArguments::new([3, 2, 0, 0, 0, 0]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        let client_fd = dispatch_socket_case(
            TALOS_SOCKET_SYSCALL,
            SyscallArguments::new([
                crate::network::SOCKET_DOMAIN_AF_INET,
                crate::network::SOCKET_TYPE_STREAM,
                crate::network::SOCKET_PROTOCOL_DEFAULT,
                0,
                0,
                0,
            ]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        let connect = dispatch_socket_case(
            TALOS_CONNECT_SYSCALL,
            SyscallArguments::new([
                4,
                listener_endpoint.ipv4_be() as u64,
                listener_endpoint.port() as u64,
                0,
                0,
                0,
            ]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        let accept = dispatch_socket_case(
            TALOS_ACCEPT_SYSCALL,
            SyscallArguments::new([3, 0, 0, 0, 0, 0]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );

        assert_eq!(bind.return_value().x0(), 0);
        assert_eq!(listen.return_value().x0(), 0);
        assert_eq!(client_fd.return_value().x0(), 4);
        assert_eq!(connect.number(), SyscallNumber::TalosConnect);
        assert_eq!(connect.return_value().x0(), 0);
        assert_eq!(accept.number(), SyscallNumber::TalosAccept);
        assert_eq!(accept.return_value().x0(), 5);
        assert_eq!(
            sockets
                .socket(crate::network::NetworkSocketDescriptor::from_raw(0))
                .expect("listener backing")
                .state(),
            crate::network::NetworkSocketState::Listening {
                local_endpoint: listener_endpoint,
                backlog: 2,
                pending: crate::network::NetworkSocketPendingQueue::new(),
            }
        );
        assert_eq!(
            sockets
                .socket(crate::network::NetworkSocketDescriptor::from_raw(1))
                .expect("client backing")
                .state(),
            crate::network::NetworkSocketState::Connected {
                local_endpoint: client_endpoint,
                remote_endpoint: listener_endpoint,
                recv_queue: crate::network::NetworkSocketPayloadQueue::new(),
            }
        );
        assert_eq!(
            sockets
                .socket(crate::network::NetworkSocketDescriptor::from_raw(2))
                .expect("accepted backing")
                .state(),
            crate::network::NetworkSocketState::Accepted {
                local_endpoint: listener_endpoint,
                remote_endpoint: client_endpoint,
                recv_queue: crate::network::NetworkSocketPayloadQueue::new(),
            }
        );

        let close_client = dispatch_socket_case(
            TALOS_CLOSE_SYSCALL,
            SyscallArguments::new([4, 0, 0, 0, 0, 0]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        let close_accepted = dispatch_socket_case(
            TALOS_CLOSE_SYSCALL,
            SyscallArguments::new([5, 0, 0, 0, 0, 0]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        assert_eq!(close_client.return_value().x0(), 0);
        assert_eq!(close_accepted.return_value().x0(), 0);
        assert_eq!(
            sockets.socket(crate::network::NetworkSocketDescriptor::from_raw(1)),
            Err(PosixError::BadDescriptor)
        );
        assert_eq!(
            sockets.socket(crate::network::NetworkSocketDescriptor::from_raw(2)),
            Err(PosixError::BadDescriptor)
        );
    }

    #[test_case]
    fn talos_connect_accept_errors_are_all_or_nothing() {
        let owner = crate::scheduler::ProcessOwnerId::new(31).expect("owner id");
        let mut store = crate::posix::ProcessDescriptorStore::<1, 7>::new_empty();
        store
            .create_owner_with_inherited_stdio(owner)
            .expect("create owner");
        let mut sockets = crate::network::NetworkSocketDescriptorTable::<4>::new();
        let mut user_memory = [0u8; 128];
        let endpoint = crate::network::Ipv4Endpoint::new(0x7f00_0001, 8080);

        let scalar_connect = dispatch(TALOS_CONNECT_SYSCALL, SyscallArguments::empty());
        let scalar_accept = dispatch(TALOS_ACCEPT_SYSCALL, SyscallArguments::empty());
        assert_eq!(
            scalar_connect.return_value().x0(),
            (ENOTSUP as u64).wrapping_neg()
        );
        assert_eq!(
            scalar_accept.return_value().x0(),
            (ENOTSUP as u64).wrapping_neg()
        );

        let listener = dispatch_socket_case(
            TALOS_SOCKET_SYSCALL,
            SyscallArguments::new([
                crate::network::SOCKET_DOMAIN_AF_INET,
                crate::network::SOCKET_TYPE_STREAM,
                crate::network::SOCKET_PROTOCOL_DEFAULT,
                0,
                0,
                0,
            ]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        assert_eq!(listener.return_value().x0(), 3);
        assert_eq!(
            dispatch_socket_case(
                TALOS_BIND_SYSCALL,
                SyscallArguments::new([
                    3,
                    endpoint.ipv4_be() as u64,
                    endpoint.port() as u64,
                    0,
                    0,
                    0,
                ]),
                Some(owner),
                &mut store,
                &mut sockets,
                &mut user_memory,
            )
            .return_value()
            .x0(),
            0
        );
        assert_eq!(
            dispatch_socket_case(
                TALOS_LISTEN_SYSCALL,
                SyscallArguments::new([3, 1, 0, 0, 0, 0]),
                Some(owner),
                &mut store,
                &mut sockets,
                &mut user_memory,
            )
            .return_value()
            .x0(),
            0
        );
        let client = dispatch_socket_case(
            TALOS_SOCKET_SYSCALL,
            SyscallArguments::new([
                crate::network::SOCKET_DOMAIN_AF_INET,
                crate::network::SOCKET_TYPE_STREAM,
                crate::network::SOCKET_PROTOCOL_DEFAULT,
                0,
                0,
                0,
            ]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        assert_eq!(client.return_value().x0(), 4);

        let reserved_connect = dispatch_socket_case(
            TALOS_CONNECT_SYSCALL,
            SyscallArguments::new([
                4,
                endpoint.ipv4_be() as u64,
                endpoint.port() as u64,
                1,
                0,
                0,
            ]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        let oversized_ipv4 = dispatch_socket_case(
            TALOS_CONNECT_SYSCALL,
            SyscallArguments::new([4, u32::MAX as u64 + 1, endpoint.port() as u64, 0, 0, 0]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        let zero_port = dispatch_socket_case(
            TALOS_CONNECT_SYSCALL,
            SyscallArguments::new([4, endpoint.ipv4_be() as u64, 0, 0, 0, 0]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        let stdout_connect = dispatch_socket_case(
            TALOS_CONNECT_SYSCALL,
            SyscallArguments::new([
                1,
                endpoint.ipv4_be() as u64,
                endpoint.port() as u64,
                0,
                0,
                0,
            ]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        let empty_accept = dispatch_socket_case(
            TALOS_ACCEPT_SYSCALL,
            SyscallArguments::new([3, 0, 0, 0, 0, 0]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        let no_listener_connect = dispatch_socket_case(
            TALOS_CONNECT_SYSCALL,
            SyscallArguments::new([
                4,
                endpoint.ipv4_be() as u64,
                endpoint.port() as u64 + 1,
                0,
                0,
                0,
            ]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );

        assert_eq!(
            reserved_connect.return_value().x0(),
            (EINVAL as u64).wrapping_neg()
        );
        assert_eq!(
            oversized_ipv4.return_value().x0(),
            (EINVAL as u64).wrapping_neg()
        );
        assert_eq!(
            zero_port.return_value().x0(),
            (EINVAL as u64).wrapping_neg()
        );
        assert_eq!(
            stdout_connect.return_value().x0(),
            (EBADF as u64).wrapping_neg()
        );
        assert_eq!(
            empty_accept.return_value().x0(),
            (EAGAIN as u64).wrapping_neg()
        );
        assert_eq!(
            no_listener_connect.return_value().x0(),
            (EINVAL as u64).wrapping_neg()
        );
        assert_eq!(
            sockets
                .socket(crate::network::NetworkSocketDescriptor::from_raw(1))
                .expect("client unchanged")
                .state(),
            crate::network::NetworkSocketState::OpenUnbound
        );

        let connect = dispatch_socket_case(
            TALOS_CONNECT_SYSCALL,
            SyscallArguments::new([
                4,
                endpoint.ipv4_be() as u64,
                endpoint.port() as u64,
                0,
                0,
                0,
            ]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        assert_eq!(connect.return_value().x0(), 0);
        let second_client = dispatch_socket_case(
            TALOS_SOCKET_SYSCALL,
            SyscallArguments::new([
                crate::network::SOCKET_DOMAIN_AF_INET,
                crate::network::SOCKET_TYPE_STREAM,
                crate::network::SOCKET_PROTOCOL_DEFAULT,
                0,
                0,
                0,
            ]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        let full_queue_connect = dispatch_socket_case(
            TALOS_CONNECT_SYSCALL,
            SyscallArguments::new([
                5,
                endpoint.ipv4_be() as u64,
                endpoint.port() as u64,
                0,
                0,
                0,
            ]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        assert_eq!(second_client.return_value().x0(), 5);
        assert_eq!(
            full_queue_connect.return_value().x0(),
            (ENOSPC as u64).wrapping_neg()
        );
        assert_eq!(
            sockets
                .socket(crate::network::NetworkSocketDescriptor::from_raw(2))
                .expect("second client unchanged")
                .state(),
            crate::network::NetworkSocketState::OpenUnbound
        );
    }

    #[test_case]
    fn talos_accept_rejects_capacity_failures_without_dequeueing_peer() {
        let owner = crate::scheduler::ProcessOwnerId::new(31).expect("owner id");
        let mut store = crate::posix::ProcessDescriptorStore::<1, 5>::new_empty();
        store
            .create_owner_with_inherited_stdio(owner)
            .expect("create owner");
        let mut sockets = crate::network::NetworkSocketDescriptorTable::<2>::new();
        let mut user_memory = [0u8; 128];
        let endpoint = crate::network::Ipv4Endpoint::new(0x7f00_0001, 8080);

        for expected_fd in 3..=4 {
            let open = dispatch_socket_case(
                TALOS_SOCKET_SYSCALL,
                SyscallArguments::new([
                    crate::network::SOCKET_DOMAIN_AF_INET,
                    crate::network::SOCKET_TYPE_STREAM,
                    crate::network::SOCKET_PROTOCOL_DEFAULT,
                    0,
                    0,
                    0,
                ]),
                Some(owner),
                &mut store,
                &mut sockets,
                &mut user_memory,
            );
            assert_eq!(open.return_value().x0(), expected_fd);
        }
        assert_eq!(
            dispatch_socket_case(
                TALOS_BIND_SYSCALL,
                SyscallArguments::new([
                    3,
                    endpoint.ipv4_be() as u64,
                    endpoint.port() as u64,
                    0,
                    0,
                    0,
                ]),
                Some(owner),
                &mut store,
                &mut sockets,
                &mut user_memory,
            )
            .return_value()
            .x0(),
            0
        );
        assert_eq!(
            dispatch_socket_case(
                TALOS_LISTEN_SYSCALL,
                SyscallArguments::new([3, 1, 0, 0, 0, 0]),
                Some(owner),
                &mut store,
                &mut sockets,
                &mut user_memory,
            )
            .return_value()
            .x0(),
            0
        );
        assert_eq!(
            dispatch_socket_case(
                TALOS_CONNECT_SYSCALL,
                SyscallArguments::new([
                    4,
                    endpoint.ipv4_be() as u64,
                    endpoint.port() as u64,
                    0,
                    0,
                    0
                ]),
                Some(owner),
                &mut store,
                &mut sockets,
                &mut user_memory,
            )
            .return_value()
            .x0(),
            0
        );

        let no_process_descriptor = dispatch_socket_case(
            TALOS_ACCEPT_SYSCALL,
            SyscallArguments::new([3, 0, 0, 0, 0, 0]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        assert_eq!(
            no_process_descriptor.return_value().x0(),
            (EMFILE as u64).wrapping_neg()
        );
        assert!(matches!(
            sockets
                .socket(crate::network::NetworkSocketDescriptor::from_raw(0))
                .expect("listener still queued")
                .state(),
            crate::network::NetworkSocketState::Listening { pending, .. } if pending.len() == 1
        ));

        assert_eq!(
            dispatch_socket_case(
                TALOS_CLOSE_SYSCALL,
                SyscallArguments::new([1, 0, 0, 0, 0, 0]),
                Some(owner),
                &mut store,
                &mut sockets,
                &mut user_memory,
            )
            .return_value()
            .x0(),
            0
        );
        let no_socket_backing = dispatch_socket_case(
            TALOS_ACCEPT_SYSCALL,
            SyscallArguments::new([3, 0, 0, 0, 0, 0]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        assert_eq!(
            no_socket_backing.return_value().x0(),
            (ENOSPC as u64).wrapping_neg()
        );
        assert!(matches!(
            sockets
                .socket(crate::network::NetworkSocketDescriptor::from_raw(0))
                .expect("listener still queued after backing failure")
                .state(),
            crate::network::NetworkSocketState::Listening { pending, .. } if pending.len() == 1
        ));
    }

    fn create_socket_pair(
        owner: crate::scheduler::ProcessOwnerId,
        store: &mut crate::posix::ProcessDescriptorStore<2, 8>,
        sockets: &mut crate::network::NetworkSocketDescriptorTable<4>,
        user_memory: &mut [u8; 128],
    ) -> (u64, u64) {
        let endpoint = crate::network::Ipv4Endpoint::new(0x7f00_0001, 8080);
        let listener = dispatch_socket_case(
            TALOS_SOCKET_SYSCALL,
            SyscallArguments::new([
                crate::network::SOCKET_DOMAIN_AF_INET,
                crate::network::SOCKET_TYPE_STREAM,
                crate::network::SOCKET_PROTOCOL_DEFAULT,
                0,
                0,
                0,
            ]),
            Some(owner),
            store,
            sockets,
            user_memory,
        );
        assert_eq!(listener.return_value().x0(), 3);
        assert_eq!(
            dispatch_socket_case(
                TALOS_BIND_SYSCALL,
                SyscallArguments::new([
                    3,
                    endpoint.ipv4_be() as u64,
                    endpoint.port() as u64,
                    0,
                    0,
                    0,
                ]),
                Some(owner),
                store,
                sockets,
                user_memory,
            )
            .return_value()
            .x0(),
            0
        );
        assert_eq!(
            dispatch_socket_case(
                TALOS_LISTEN_SYSCALL,
                SyscallArguments::new([3, 2, 0, 0, 0, 0]),
                Some(owner),
                store,
                sockets,
                user_memory,
            )
            .return_value()
            .x0(),
            0
        );
        let client = dispatch_socket_case(
            TALOS_SOCKET_SYSCALL,
            SyscallArguments::new([
                crate::network::SOCKET_DOMAIN_AF_INET,
                crate::network::SOCKET_TYPE_STREAM,
                crate::network::SOCKET_PROTOCOL_DEFAULT,
                0,
                0,
                0,
            ]),
            Some(owner),
            store,
            sockets,
            user_memory,
        );
        assert_eq!(client.return_value().x0(), 4);
        assert_eq!(
            dispatch_socket_case(
                TALOS_CONNECT_SYSCALL,
                SyscallArguments::new([
                    4,
                    endpoint.ipv4_be() as u64,
                    endpoint.port() as u64,
                    0,
                    0,
                    0,
                ]),
                Some(owner),
                store,
                sockets,
                user_memory,
            )
            .return_value()
            .x0(),
            0
        );
        let accepted = dispatch_socket_case(
            TALOS_ACCEPT_SYSCALL,
            SyscallArguments::new([3, 0, 0, 0, 0, 0]),
            Some(owner),
            store,
            sockets,
            user_memory,
        );
        assert_eq!(accepted.return_value().x0(), 5);
        (4, 5)
    }

    #[test_case]
    fn talos_send_recv_moves_local_payload_bytes_bidirectionally() {
        let owner = crate::scheduler::ProcessOwnerId::new(31).expect("owner id");
        let mut store = crate::posix::ProcessDescriptorStore::<2, 8>::new_empty();
        store
            .create_owner_with_inherited_stdio(owner)
            .expect("create owner");
        let mut sockets = crate::network::NetworkSocketDescriptorTable::<4>::new();
        let mut user_memory = [0u8; 128];
        let (client_fd, accepted_fd) =
            create_socket_pair(owner, &mut store, &mut sockets, &mut user_memory);

        user_memory[..15].copy_from_slice(b"client->server");
        let send = dispatch_socket_case(
            TALOS_SEND_SYSCALL,
            SyscallArguments::new([client_fd, 0x0000_0000_0011_0000, 15, 0, 0, 0]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        let recv = dispatch_socket_case(
            TALOS_RECV_SYSCALL,
            SyscallArguments::new([accepted_fd, 0x0000_0000_0011_0040, 64, 0, 0, 0]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        assert_eq!(send.number(), SyscallNumber::TalosSend);
        assert_eq!(send.return_value().x0(), 15);
        assert_eq!(recv.number(), SyscallNumber::TalosRecv);
        assert_eq!(recv.return_value().x0(), 15);
        assert_eq!(&user_memory[0x40..0x4f], b"client->server");

        user_memory[0x20..0x23].copy_from_slice(b"ack");
        assert_eq!(
            dispatch_socket_case(
                TALOS_SEND_SYSCALL,
                SyscallArguments::new([accepted_fd, 0x0000_0000_0011_0020, 3, 0, 0, 0]),
                Some(owner),
                &mut store,
                &mut sockets,
                &mut user_memory,
            )
            .return_value()
            .x0(),
            3
        );
        assert_eq!(
            dispatch_socket_case(
                TALOS_RECV_SYSCALL,
                SyscallArguments::new([client_fd, 0x0000_0000_0011_0060, 2, 0, 0, 0]),
                Some(owner),
                &mut store,
                &mut sockets,
                &mut user_memory,
            )
            .return_value()
            .x0(),
            2
        );
        assert_eq!(&user_memory[0x60..0x62], b"ac");
        assert_eq!(
            dispatch_socket_case(
                TALOS_RECV_SYSCALL,
                SyscallArguments::new([client_fd, 0x0000_0000_0011_0062, 8, 0, 0, 0]),
                Some(owner),
                &mut store,
                &mut sockets,
                &mut user_memory,
            )
            .return_value()
            .x0(),
            1
        );
        assert_eq!(&user_memory[0x62..0x63], b"k");
    }

    #[test_case]
    fn talos_send_recv_errors_are_deterministic_and_all_or_nothing() {
        let owner = crate::scheduler::ProcessOwnerId::new(31).expect("owner id");
        let mut store = crate::posix::ProcessDescriptorStore::<2, 8>::new_empty();
        store
            .create_owner_with_inherited_stdio(owner)
            .expect("create owner");
        let mut sockets = crate::network::NetworkSocketDescriptorTable::<4>::new();
        let mut user_memory = [0u8; 128];
        let (client_fd, accepted_fd) =
            create_socket_pair(owner, &mut store, &mut sockets, &mut user_memory);

        assert_eq!(
            dispatch(TALOS_SEND_SYSCALL, SyscallArguments::empty())
                .return_value()
                .x0(),
            (ENOTSUP as u64).wrapping_neg()
        );
        assert_eq!(
            dispatch(TALOS_RECV_SYSCALL, SyscallArguments::empty())
                .return_value()
                .x0(),
            (ENOTSUP as u64).wrapping_neg()
        );
        assert_eq!(
            dispatch_socket_case(
                TALOS_RECV_SYSCALL,
                SyscallArguments::new([accepted_fd, 0x0000_0000_0011_0040, 8, 0, 0, 0]),
                Some(owner),
                &mut store,
                &mut sockets,
                &mut user_memory,
            )
            .return_value()
            .x0(),
            (EAGAIN as u64).wrapping_neg()
        );
        assert_eq!(
            dispatch_socket_case(
                TALOS_SEND_SYSCALL,
                SyscallArguments::new([client_fd, 0x0000_0000_0011_0000, 1, 1, 0, 0]),
                Some(owner),
                &mut store,
                &mut sockets,
                &mut user_memory,
            )
            .return_value()
            .x0(),
            (EINVAL as u64).wrapping_neg()
        );
        assert_eq!(
            dispatch_socket_case(
                TALOS_SEND_SYSCALL,
                SyscallArguments::new([1, 0x0000_0000_0011_0000, 1, 0, 0, 0]),
                Some(owner),
                &mut store,
                &mut sockets,
                &mut user_memory,
            )
            .return_value()
            .x0(),
            (EBADF as u64).wrapping_neg()
        );
        assert_eq!(
            dispatch_socket_case(
                TALOS_SEND_SYSCALL,
                SyscallArguments::new([3, 0x0000_0000_0011_0000, 1, 0, 0, 0]),
                Some(owner),
                &mut store,
                &mut sockets,
                &mut user_memory,
            )
            .return_value()
            .x0(),
            (EINVAL as u64).wrapping_neg()
        );
        assert_eq!(
            dispatch_socket_case(
                TALOS_SEND_SYSCALL,
                SyscallArguments::new([client_fd, 0x0000_0000_0011_0000, 65, 0, 0, 0]),
                Some(owner),
                &mut store,
                &mut sockets,
                &mut user_memory,
            )
            .return_value()
            .x0(),
            (ENOSPC as u64).wrapping_neg()
        );
        assert_eq!(
            dispatch_socket_case(
                TALOS_SEND_SYSCALL,
                SyscallArguments::new([client_fd, 0, 1, 0, 0, 0]),
                Some(owner),
                &mut store,
                &mut sockets,
                &mut user_memory,
            )
            .return_value()
            .x0(),
            (EFAULT as u64).wrapping_neg()
        );
        assert_eq!(
            dispatch_socket_case(
                TALOS_RECV_SYSCALL,
                SyscallArguments::new([accepted_fd, 0x0000_0000_0011_0040, 8, 0, 0, 0]),
                Some(owner),
                &mut store,
                &mut sockets,
                &mut user_memory,
            )
            .return_value()
            .x0(),
            (EAGAIN as u64).wrapping_neg()
        );

        let mut index = 0usize;
        while index < crate::network::SOCKET_PAYLOAD_QUEUE_CAPACITY {
            user_memory[index] = index as u8;
            index += 1;
        }
        assert_eq!(
            dispatch_socket_case(
                TALOS_SEND_SYSCALL,
                SyscallArguments::new([
                    client_fd,
                    0x0000_0000_0011_0000,
                    crate::network::SOCKET_PAYLOAD_QUEUE_CAPACITY as u64,
                    0,
                    0,
                    0,
                ]),
                Some(owner),
                &mut store,
                &mut sockets,
                &mut user_memory,
            )
            .return_value()
            .x0(),
            crate::network::SOCKET_PAYLOAD_QUEUE_CAPACITY as u64
        );
        assert_eq!(
            dispatch_socket_case(
                TALOS_SEND_SYSCALL,
                SyscallArguments::new([client_fd, 0x0000_0000_0011_0000, 1, 0, 0, 0]),
                Some(owner),
                &mut store,
                &mut sockets,
                &mut user_memory,
            )
            .return_value()
            .x0(),
            (ENOSPC as u64).wrapping_neg()
        );
        assert_eq!(
            dispatch_socket_case(
                TALOS_RECV_SYSCALL,
                SyscallArguments::new([accepted_fd, 0x0000_0000_0011_0040, 64, 0, 0, 0]),
                Some(owner),
                &mut store,
                &mut sockets,
                &mut user_memory,
            )
            .return_value()
            .x0(),
            crate::network::SOCKET_PAYLOAD_QUEUE_CAPACITY as u64
        );
        assert_eq!(&user_memory[0x40..0x80], &user_memory[..0x40]);
    }

    #[test_case]
    fn talos_send_recv_reports_disconnected_peer_after_queue_drain() {
        let owner = crate::scheduler::ProcessOwnerId::new(31).expect("owner id");
        let mut store = crate::posix::ProcessDescriptorStore::<2, 8>::new_empty();
        store
            .create_owner_with_inherited_stdio(owner)
            .expect("create owner");
        let mut sockets = crate::network::NetworkSocketDescriptorTable::<4>::new();
        let mut user_memory = [0u8; 128];
        let (client_fd, accepted_fd) =
            create_socket_pair(owner, &mut store, &mut sockets, &mut user_memory);
        user_memory[..4].copy_from_slice(b"last");
        assert_eq!(
            dispatch_socket_case(
                TALOS_SEND_SYSCALL,
                SyscallArguments::new([client_fd, 0x0000_0000_0011_0000, 4, 0, 0, 0]),
                Some(owner),
                &mut store,
                &mut sockets,
                &mut user_memory,
            )
            .return_value()
            .x0(),
            4
        );
        assert_eq!(
            dispatch_socket_case(
                TALOS_CLOSE_SYSCALL,
                SyscallArguments::new([client_fd, 0, 0, 0, 0, 0]),
                Some(owner),
                &mut store,
                &mut sockets,
                &mut user_memory,
            )
            .return_value()
            .x0(),
            0
        );
        assert_eq!(
            dispatch_socket_case(
                TALOS_RECV_SYSCALL,
                SyscallArguments::new([accepted_fd, 0x0000_0000_0011_0040, 8, 0, 0, 0]),
                Some(owner),
                &mut store,
                &mut sockets,
                &mut user_memory,
            )
            .return_value()
            .x0(),
            4
        );
        assert_eq!(&user_memory[0x40..0x44], b"last");
        assert_eq!(
            dispatch_socket_case(
                TALOS_RECV_SYSCALL,
                SyscallArguments::new([accepted_fd, 0x0000_0000_0011_0040, 8, 0, 0, 0]),
                Some(owner),
                &mut store,
                &mut sockets,
                &mut user_memory,
            )
            .return_value()
            .x0(),
            (EPIPE as u64).wrapping_neg()
        );
        assert_eq!(
            dispatch_socket_case(
                TALOS_SEND_SYSCALL,
                SyscallArguments::new([accepted_fd, 0x0000_0000_0011_0000, 1, 0, 0, 0]),
                Some(owner),
                &mut store,
                &mut sockets,
                &mut user_memory,
            )
            .return_value()
            .x0(),
            (EPIPE as u64).wrapping_neg()
        );
    }

    #[test_case]
    fn talos_bind_listen_errors_are_deterministic_and_do_not_mutate_state() {
        let owner = crate::scheduler::ProcessOwnerId::new(31).expect("owner id");
        let second = crate::scheduler::ProcessOwnerId::new(32).expect("second owner id");
        let mut store = crate::posix::ProcessDescriptorStore::<2, 5>::new_empty();
        store
            .create_owner_with_inherited_stdio(owner)
            .expect("create owner");
        store
            .create_owner_with_inherited_stdio(second)
            .expect("create second owner");
        let mut sockets = crate::network::NetworkSocketDescriptorTable::<1>::new();
        let mut user_memory = [0u8; 128];
        let endpoint = crate::network::Ipv4Endpoint::new(0x0a00_0001, 80);

        let scalar_bind = dispatch(TALOS_BIND_SYSCALL, SyscallArguments::empty());
        let scalar_listen = dispatch(TALOS_LISTEN_SYSCALL, SyscallArguments::empty());
        assert_eq!(
            scalar_bind.return_value().x0(),
            (ENOTSUP as u64).wrapping_neg()
        );
        assert_eq!(
            scalar_listen.return_value().x0(),
            (ENOTSUP as u64).wrapping_neg()
        );

        let open = dispatch_socket_case(
            TALOS_SOCKET_SYSCALL,
            SyscallArguments::new([
                crate::network::SOCKET_DOMAIN_AF_INET,
                crate::network::SOCKET_TYPE_STREAM,
                crate::network::SOCKET_PROTOCOL_DEFAULT,
                0,
                0,
                0,
            ]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        assert_eq!(open.return_value().x0(), 3);

        let listen_before_bind = dispatch_socket_case(
            TALOS_LISTEN_SYSCALL,
            SyscallArguments::new([3, 1, 0, 0, 0, 0]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        let bind_reserved = dispatch_socket_case(
            TALOS_BIND_SYSCALL,
            SyscallArguments::new([
                3,
                endpoint.ipv4_be() as u64,
                endpoint.port() as u64,
                1,
                0,
                0,
            ]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        let bind_oversize_ipv4 = dispatch_socket_case(
            TALOS_BIND_SYSCALL,
            SyscallArguments::new([3, u32::MAX as u64 + 1, endpoint.port() as u64, 0, 0, 0]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        let bind_zero_port = dispatch_socket_case(
            TALOS_BIND_SYSCALL,
            SyscallArguments::new([3, endpoint.ipv4_be() as u64, 0, 0, 0, 0]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        let listen_reserved = dispatch_socket_case(
            TALOS_LISTEN_SYSCALL,
            SyscallArguments::new([3, 1, 1, 0, 0, 0]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        let listen_backlog_zero = dispatch_socket_case(
            TALOS_LISTEN_SYSCALL,
            SyscallArguments::new([3, 0, 0, 0, 0, 0]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        let listen_backlog_too_large = dispatch_socket_case(
            TALOS_LISTEN_SYSCALL,
            SyscallArguments::new([3, 5, 0, 0, 0, 0]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        let bind_stdout = dispatch_socket_case(
            TALOS_BIND_SYSCALL,
            SyscallArguments::new([
                1,
                endpoint.ipv4_be() as u64,
                endpoint.port() as u64,
                0,
                0,
                0,
            ]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        let listen_stdout = dispatch_socket_case(
            TALOS_LISTEN_SYSCALL,
            SyscallArguments::new([1, 1, 0, 0, 0, 0]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );

        assert_eq!(
            listen_before_bind.return_value().x0(),
            (EINVAL as u64).wrapping_neg()
        );
        assert_eq!(
            bind_reserved.return_value().x0(),
            (EINVAL as u64).wrapping_neg()
        );
        assert_eq!(
            bind_oversize_ipv4.return_value().x0(),
            (EINVAL as u64).wrapping_neg()
        );
        assert_eq!(
            bind_zero_port.return_value().x0(),
            (EINVAL as u64).wrapping_neg()
        );
        assert_eq!(
            listen_reserved.return_value().x0(),
            (EINVAL as u64).wrapping_neg()
        );
        assert_eq!(
            listen_backlog_zero.return_value().x0(),
            (EINVAL as u64).wrapping_neg()
        );
        assert_eq!(
            listen_backlog_too_large.return_value().x0(),
            (EINVAL as u64).wrapping_neg()
        );
        assert_eq!(
            bind_stdout.return_value().x0(),
            (EBADF as u64).wrapping_neg()
        );
        assert_eq!(
            listen_stdout.return_value().x0(),
            (EBADF as u64).wrapping_neg()
        );
        assert_eq!(
            sockets
                .socket(crate::network::NetworkSocketDescriptor::from_raw(0))
                .expect("socket backing")
                .state(),
            crate::network::NetworkSocketState::OpenUnbound
        );

        let forged = crate::posix::DescriptorEntry::new(
            crate::posix::DescriptorAccess::ReadWrite,
            crate::posix::DescriptorFlags::EMPTY,
            crate::posix::DescriptorObject::new(crate::posix::DescriptorObjectKind::Socket, 0),
        );
        assert_eq!(
            store
                .current_descriptor_table_mut(Some(second))
                .expect("second table")
                .allocate(forged),
            Ok(3)
        );
        let wrong_owner_bind = dispatch_socket_case(
            TALOS_BIND_SYSCALL,
            SyscallArguments::new([
                3,
                endpoint.ipv4_be() as u64,
                endpoint.port() as u64,
                0,
                0,
                0,
            ]),
            Some(second),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        assert_eq!(
            wrong_owner_bind.return_value().x0(),
            (EBADF as u64).wrapping_neg()
        );
        assert_eq!(
            sockets
                .socket(crate::network::NetworkSocketDescriptor::from_raw(0))
                .expect("socket backing")
                .state(),
            crate::network::NetworkSocketState::OpenUnbound
        );
    }

    #[test_case]
    fn talos_socket_close_rejects_wrong_owner_socket_backing() {
        let owner = crate::scheduler::ProcessOwnerId::new(31).expect("owner id");
        let second = crate::scheduler::ProcessOwnerId::new(32).expect("second owner id");
        let mut store = crate::posix::ProcessDescriptorStore::<2, 5>::new_empty();
        store
            .create_owner_with_inherited_stdio(owner)
            .expect("create owner");
        store
            .create_owner_with_inherited_stdio(second)
            .expect("create second owner");
        let mut sockets = crate::network::NetworkSocketDescriptorTable::<1>::new();
        let mut user_memory = [0u8; 128];

        let open = dispatch_socket_case(
            TALOS_SOCKET_SYSCALL,
            SyscallArguments::new([
                crate::network::SOCKET_DOMAIN_AF_INET,
                crate::network::SOCKET_TYPE_STREAM,
                crate::network::SOCKET_PROTOCOL_DEFAULT,
                0,
                0,
                0,
            ]),
            Some(owner),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );
        assert_eq!(open.return_value().x0(), 3);
        let forged = crate::posix::DescriptorEntry::new(
            crate::posix::DescriptorAccess::ReadWrite,
            crate::posix::DescriptorFlags::EMPTY,
            crate::posix::DescriptorObject::new(crate::posix::DescriptorObjectKind::Socket, 0),
        );
        assert_eq!(
            store
                .current_descriptor_table_mut(Some(second))
                .expect("second table")
                .allocate(forged),
            Ok(3)
        );

        let wrong_owner_close = dispatch_socket_case(
            TALOS_CLOSE_SYSCALL,
            SyscallArguments::new([3, 0, 0, 0, 0, 0]),
            Some(second),
            &mut store,
            &mut sockets,
            &mut user_memory,
        );

        assert_eq!(
            wrong_owner_close.return_value().x0(),
            (EBADF as u64).wrapping_neg()
        );
        assert!(
            store
                .current_descriptor_table(Some(second))
                .expect("second table")
                .get(3)
                .is_ok()
        );
        assert!(
            sockets
                .socket(crate::network::NetworkSocketDescriptor::from_raw(0))
                .is_ok()
        );
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
        assert_eq!(write_duplicate.return_value().x0(), 18);
        assert_eq!(console.as_bytes(), b"talos-stdout-qemu\n");
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
    fn talos_dup_stdout_returns_lowest_free_descriptor_and_preserves_source() {
        let (owner, mut store, mappings, user_memory) = process_descriptor_fixture();
        let mut scratch = [0u8; 64];
        let mut console = CaptureConsole::new();

        let duplicate = dispatch_process_descriptor(
            TALOS_DUP_SYSCALL,
            SyscallArguments::new([crate::posix::STDOUT_FD as u64, 0, 0, 0, 0, 0]),
            Some(owner),
            &mut store,
            &mappings,
            0x0000_0000_0011_0000,
            &user_memory,
            &mut scratch,
            &mut console,
        );
        let write_source = dispatch_process_descriptor(
            TALOS_WRITE_SYSCALL,
            SyscallArguments::new([
                crate::posix::STDOUT_FD as u64,
                0x0000_0000_0011_0000,
                18,
                0,
                0,
                0,
            ]),
            Some(owner),
            &mut store,
            &mappings,
            0x0000_0000_0011_0000,
            &user_memory,
            &mut scratch,
            &mut console,
        );

        assert_eq!(duplicate.number(), SyscallNumber::TalosDup);
        assert_eq!(duplicate.return_value().x0(), 3);
        assert_eq!(write_source.return_value().x0(), 18);
        assert_eq!(console.as_bytes(), b"talos-stdout-qemu\n");
    }

    #[test_case]
    fn talos_dup_duplicate_remains_writable_after_source_close() {
        let (owner, mut store, mappings, user_memory) = process_descriptor_fixture();
        let mut scratch = [0u8; 64];
        let mut console = CaptureConsole::new();

        let duplicate = dispatch_process_descriptor(
            TALOS_DUP_SYSCALL,
            SyscallArguments::new([crate::posix::STDOUT_FD as u64, 0, 0, 0, 0, 0]),
            Some(owner),
            &mut store,
            &mappings,
            0x0000_0000_0011_0000,
            &user_memory,
            &mut scratch,
            &mut console,
        );
        let close_source = dispatch_process_descriptor(
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
            SyscallArguments::new([
                duplicate.return_value().x0(),
                0x0000_0000_0011_0000,
                18,
                0,
                0,
                0,
            ]),
            Some(owner),
            &mut store,
            &mappings,
            0x0000_0000_0011_0000,
            &user_memory,
            &mut scratch,
            &mut console,
        );

        assert_eq!(duplicate.return_value().x0(), 3);
        assert_eq!(close_source.return_value().x0(), 0);
        assert_eq!(write_duplicate.return_value().x0(), 18);
        assert_eq!(console.as_bytes(), b"talos-stdout-qemu\n");
    }

    #[test_case]
    fn talos_dup_stderr_and_stdin_follow_table_local_descriptor_rules() {
        let (owner, mut store, mappings, user_memory) = process_descriptor_fixture();
        let mut scratch = [0u8; 64];
        let mut console = CaptureConsole::new();

        let stderr_duplicate = dispatch_process_descriptor(
            TALOS_DUP_SYSCALL,
            SyscallArguments::new([crate::posix::STDERR_FD as u64, 0, 0, 0, 0, 0]),
            Some(owner),
            &mut store,
            &mappings,
            0x0000_0000_0011_0000,
            &user_memory,
            &mut scratch,
            &mut console,
        );
        let mut stdin_store = crate::posix::ProcessDescriptorStore::<1, 4>::new_empty();
        stdin_store
            .create_owner_with_inherited_stdio(owner)
            .expect("create owner");
        let stdin_duplicate = dispatch_process_descriptor(
            TALOS_DUP_SYSCALL,
            SyscallArguments::new([crate::posix::STDIN_FD as u64, 0, 0, 0, 0, 0]),
            Some(owner),
            &mut stdin_store,
            &mappings,
            0x0000_0000_0011_0000,
            &user_memory,
            &mut scratch,
            &mut console,
        );
        let write_stdin_duplicate = dispatch_process_descriptor(
            TALOS_WRITE_SYSCALL,
            SyscallArguments::new([
                stdin_duplicate.return_value().x0(),
                0x0000_0000_0011_0000,
                18,
                0,
                0,
                0,
            ]),
            Some(owner),
            &mut stdin_store,
            &mappings,
            0x0000_0000_0011_0000,
            &user_memory,
            &mut scratch,
            &mut console,
        );

        assert_eq!(stderr_duplicate.return_value().x0(), 3);
        assert_eq!(stdin_duplicate.return_value().x0(), 3);
        assert_eq!(
            write_stdin_duplicate.return_value().x0(),
            (EBADF as u64).wrapping_neg()
        );
        assert_eq!(console.as_bytes(), b"");
    }

    #[test_case]
    fn talos_dup_failures_are_deterministic_and_do_not_mutate_on_einval() {
        let (owner, mut store, mappings, user_memory) = process_descriptor_fixture();
        let missing = crate::scheduler::ProcessOwnerId::new(32).expect("missing owner id");
        let mut scratch = [0u8; 64];
        let mut console = CaptureConsole::new();

        let missing_owner = dispatch_process_descriptor(
            TALOS_DUP_SYSCALL,
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
            TALOS_DUP_SYSCALL,
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
            TALOS_DUP_SYSCALL,
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
            TALOS_DUP_SYSCALL,
            SyscallArguments::new([1, 1, 0, 0, 0, 0]),
            Some(owner),
            &mut store,
            &mappings,
            0x0000_0000_0011_0000,
            &user_memory,
            &mut scratch,
            &mut console,
        );
        let after_reserved = dispatch_process_descriptor(
            TALOS_DUP_SYSCALL,
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
        assert_eq!(after_reserved.return_value().x0(), 3);
        assert_eq!(console.as_bytes(), b"");
    }

    #[test_case]
    fn talos_dup_full_table_returns_emfile_without_mutation() {
        let owner = crate::scheduler::ProcessOwnerId::new(33).expect("owner id");
        let mut store = crate::posix::ProcessDescriptorStore::<1, 3>::new_empty();
        let mappings = [crate::posix::UserMapping::new(
            0x0000_0000_0011_0000,
            0x80,
            crate::posix::UserMappingPermissions::USER_DATA,
        )
        .expect("user data mapping")];
        let user_memory = [0u8; 128];
        let mut scratch = [0u8; 64];
        let mut console = CaptureConsole::new();

        store
            .create_owner_with_inherited_stdio(owner)
            .expect("create owner");
        let duplicate = dispatch_process_descriptor(
            TALOS_DUP_SYSCALL,
            SyscallArguments::new([crate::posix::STDOUT_FD as u64, 0, 0, 0, 0, 0]),
            Some(owner),
            &mut store,
            &mappings,
            0x0000_0000_0011_0000,
            &user_memory,
            &mut scratch,
            &mut console,
        );

        assert_eq!(
            duplicate.return_value().x0(),
            (EMFILE as u64).wrapping_neg()
        );
        assert!(
            store
                .current_descriptor_table(Some(owner))
                .expect("current table")
                .get(crate::posix::STDOUT_FD)
                .is_ok()
        );
        assert_eq!(console.as_bytes(), b"");
    }

    #[test_case]
    fn talos_read_stdin_copies_fixed_input_and_advances_after_copy() {
        let (owner, mut store, _, _) = process_descriptor_fixture();
        let mut user_memory = [0u8; 128];
        let mut stdin = crate::posix::FixedStdin::new(b"talos-stdin");

        let result = dispatch_read_case(
            crate::posix::STDIN_FD as u64,
            0x0000_0000_0011_0020,
            5,
            0,
            Some(owner),
            &mut store,
            &mut user_memory,
            Some(&mut stdin),
        );

        assert_eq!(result.number(), SyscallNumber::TalosRead);
        assert_eq!(result.return_value().x0(), 5);
        assert_eq!(&user_memory[0x20..0x25], b"talos");
        assert_eq!(stdin.cursor(), 5);
    }

    #[test_case]
    fn talos_read_short_count_and_eof_are_bounded_to_fixed_input() {
        let (owner, mut store, _, _) = process_descriptor_fixture();
        let mut user_memory = [0u8; 128];
        let mut stdin = crate::posix::FixedStdin::new(b"abc");

        let short = dispatch_read_case(
            crate::posix::STDIN_FD as u64,
            0x0000_0000_0011_0000,
            8,
            0,
            Some(owner),
            &mut store,
            &mut user_memory,
            Some(&mut stdin),
        );
        let eof = dispatch_read_case(
            crate::posix::STDIN_FD as u64,
            0x0000_0000_0011_0010,
            8,
            0,
            Some(owner),
            &mut store,
            &mut user_memory,
            Some(&mut stdin),
        );

        assert_eq!(short.return_value().x0(), 3);
        assert_eq!(eof.return_value().x0(), 0);
        assert_eq!(&user_memory[..3], b"abc");
        assert_eq!(&user_memory[0x10..0x18], &[0; 8]);
        assert_eq!(stdin.cursor(), 3);
    }

    #[test_case]
    fn talos_read_duplicate_of_stdin_shares_fixed_input_cursor() {
        let (owner, mut store, mappings, _) = process_descriptor_fixture();
        let mut user_memory = [0u8; 128];
        let mut scratch = [0u8; 64];
        let mut console = CaptureConsole::new();
        let mut stdin = crate::posix::FixedStdin::new(b"stdin-dupe");
        let duplicate = dispatch_process_descriptor_with_fixed_stdin(
            TALOS_DUP_SYSCALL,
            SyscallArguments::new([crate::posix::STDIN_FD as u64, 0, 0, 0, 0, 0]),
            Some(owner),
            &mut store,
            &mappings,
            0x0000_0000_0011_0000,
            &mut user_memory,
            &mut scratch,
            &mut console,
            None,
        );

        let read_duplicate = dispatch_read_case(
            duplicate.return_value().x0(),
            0x0000_0000_0011_0000,
            5,
            0,
            Some(owner),
            &mut store,
            &mut user_memory,
            Some(&mut stdin),
        );
        let read_original = dispatch_read_case(
            crate::posix::STDIN_FD as u64,
            0x0000_0000_0011_0010,
            4,
            0,
            Some(owner),
            &mut store,
            &mut user_memory,
            Some(&mut stdin),
        );

        assert_eq!(duplicate.return_value().x0(), 3);
        assert_eq!(read_duplicate.return_value().x0(), 5);
        assert_eq!(read_original.return_value().x0(), 4);
        assert_eq!(&user_memory[..5], b"stdin");
        assert_eq!(&user_memory[0x10..0x14], b"-dup");
        assert_eq!(stdin.cursor(), 9);
    }

    #[test_case]
    fn talos_read_zero_length_does_not_consume_or_use_destination() {
        let (owner, mut store, _, _) = process_descriptor_fixture();
        let mut user_memory = [0u8; 128];
        let mut stdin = crate::posix::FixedStdin::new(b"abc");

        let result = dispatch_read_case(
            crate::posix::STDIN_FD as u64,
            0,
            0,
            0,
            Some(owner),
            &mut store,
            &mut user_memory,
            Some(&mut stdin),
        );

        assert_eq!(result.return_value().x0(), 0);
        assert_eq!(stdin.cursor(), 0);
        assert_eq!(user_memory, [0; 128]);
    }

    #[test_case]
    fn talos_read_fd_errors_do_not_copy_or_consume_input() {
        let (owner, mut store, _, _) = process_descriptor_fixture();
        let missing = crate::scheduler::ProcessOwnerId::new(32).expect("missing owner id");
        let mut user_memory = [0u8; 128];
        let mut stdin = crate::posix::FixedStdin::new(b"abc");

        let stdout = dispatch_read_case(
            crate::posix::STDOUT_FD as u64,
            0x0000_0000_0011_0000,
            3,
            0,
            Some(owner),
            &mut store,
            &mut user_memory,
            Some(&mut stdin),
        );
        let bad_fd = dispatch_read_case(
            99,
            0x0000_0000_0011_0000,
            3,
            0,
            Some(owner),
            &mut store,
            &mut user_memory,
            Some(&mut stdin),
        );
        store
            .close_current_descriptor(Some(owner), crate::posix::STDIN_FD)
            .expect("close stdin");
        let closed = dispatch_read_case(
            crate::posix::STDIN_FD as u64,
            0x0000_0000_0011_0000,
            3,
            0,
            Some(owner),
            &mut store,
            &mut user_memory,
            Some(&mut stdin),
        );
        let no_owner = dispatch_read_case(
            crate::posix::STDIN_FD as u64,
            0x0000_0000_0011_0000,
            3,
            0,
            None,
            &mut store,
            &mut user_memory,
            Some(&mut stdin),
        );
        let unknown_owner = dispatch_read_case(
            crate::posix::STDIN_FD as u64,
            0x0000_0000_0011_0000,
            3,
            0,
            Some(missing),
            &mut store,
            &mut user_memory,
            Some(&mut stdin),
        );

        assert_eq!(stdout.return_value().x0(), (EBADF as u64).wrapping_neg());
        assert_eq!(bad_fd.return_value().x0(), (EBADF as u64).wrapping_neg());
        assert_eq!(closed.return_value().x0(), (EBADF as u64).wrapping_neg());
        assert_eq!(no_owner.return_value().x0(), (EBADF as u64).wrapping_neg());
        assert_eq!(
            unknown_owner.return_value().x0(),
            (EBADF as u64).wrapping_neg()
        );
        assert_eq!(stdin.cursor(), 0);
        assert_eq!(user_memory, [0; 128]);
    }

    #[test_case]
    fn talos_read_reserved_registers_reject_without_mutation() {
        let (owner, mut store, _, _) = process_descriptor_fixture();
        let mut user_memory = [0u8; 128];
        let mut stdin = crate::posix::FixedStdin::new(b"abc");

        let result = dispatch_read_case(
            crate::posix::STDIN_FD as u64,
            0x0000_0000_0011_0000,
            3,
            1,
            Some(owner),
            &mut store,
            &mut user_memory,
            Some(&mut stdin),
        );

        assert_eq!(result.return_value().x0(), (EINVAL as u64).wrapping_neg());
        assert_eq!(stdin.cursor(), 0);
        assert_eq!(user_memory, [0; 128]);
    }

    #[test_case]
    fn talos_read_copy_faults_do_not_consume_fixed_input() {
        let (owner, mut store, _, _) = process_descriptor_fixture();
        let mut user_memory = [0u8; 128];
        let mut stdin = crate::posix::FixedStdin::new(b"abcdef");

        let unmapped = dispatch_read_case(
            crate::posix::STDIN_FD as u64,
            0x0000_0000_001e_0000,
            3,
            0,
            Some(owner),
            &mut store,
            &mut user_memory,
            Some(&mut stdin),
        );
        let oversize = dispatch_read_case(
            crate::posix::STDIN_FD as u64,
            0x0000_0000_0011_0000,
            crate::posix::DEFAULT_USER_COPY_LIMIT as u64 + 1,
            0,
            Some(owner),
            &mut store,
            &mut user_memory,
            Some(&mut stdin),
        );

        assert_eq!(unmapped.return_value().x0(), (EFAULT as u64).wrapping_neg());
        assert_eq!(oversize.return_value().x0(), (EFAULT as u64).wrapping_neg());
        assert_eq!(stdin.cursor(), 0);
        assert_eq!(user_memory, [0; 128]);
    }

    #[test_case]
    fn talos_read_reports_enotsup_for_non_stdin_readable_objects_or_missing_source() {
        let (owner, mut store, _, _) = process_descriptor_fixture();
        let mut user_memory = [0u8; 128];
        let mut stdin = crate::posix::FixedStdin::new(b"abc");
        let regular = crate::posix::DescriptorEntry::new(
            crate::posix::DescriptorAccess::ReadWrite,
            crate::posix::DescriptorFlags::EMPTY,
            crate::posix::DescriptorObject::new(crate::posix::DescriptorObjectKind::RegularFile, 7),
        );
        let regular_fd = store
            .current_descriptor_table_mut(Some(owner))
            .expect("current table")
            .allocate(regular)
            .expect("regular fixture fd");

        let non_stdin = dispatch_read_case(
            regular_fd as u64,
            0x0000_0000_0011_0000,
            3,
            0,
            Some(owner),
            &mut store,
            &mut user_memory,
            Some(&mut stdin),
        );
        let missing_source = dispatch_read_case(
            crate::posix::STDIN_FD as u64,
            0x0000_0000_0011_0000,
            3,
            0,
            Some(owner),
            &mut store,
            &mut user_memory,
            None,
        );

        assert_eq!(
            non_stdin.return_value().x0(),
            (ENOTSUP as u64).wrapping_neg()
        );
        assert_eq!(
            missing_source.return_value().x0(),
            (ENOTSUP as u64).wrapping_neg()
        );
        assert_eq!(stdin.cursor(), 0);
        assert_eq!(user_memory, [0; 128]);
    }

    #[test_case]
    fn talos_open_initramfs_then_read_exposes_regular_file_contents() {
        let owner = crate::scheduler::ProcessOwnerId::new(31).expect("owner id");
        let mut store = crate::posix::ProcessDescriptorStore::<2, 5>::new_empty();
        store
            .create_owner_with_inherited_stdio(owner)
            .expect("create owner");
        let mut files = crate::initramfs::ReadOnlyFileDescriptions::<2>::new_empty();
        let mut user_memory = [0u8; 128];
        user_memory[..crate::initramfs::PHASE8_BANNER_PATH.len()]
            .copy_from_slice(crate::initramfs::PHASE8_BANNER_PATH);
        user_memory[0x40..0x40 + crate::initramfs::PHASE8_INIT_PATH.len()]
            .copy_from_slice(crate::initramfs::PHASE8_INIT_PATH);

        let open_banner = dispatch_initramfs_case(
            TALOS_OPEN_SYSCALL,
            SyscallArguments::new([
                0x0000_0000_0011_0000,
                crate::initramfs::PHASE8_BANNER_PATH.len() as u64,
                0,
                0,
                0,
                0,
            ]),
            &mut store,
            &mut files,
            &mut user_memory,
        );
        let read_banner = dispatch_initramfs_case(
            TALOS_READ_SYSCALL,
            SyscallArguments::new([
                open_banner.return_value().x0(),
                0x0000_0000_0011_0020,
                64,
                0,
                0,
                0,
            ]),
            &mut store,
            &mut files,
            &mut user_memory,
        );
        let eof_banner = dispatch_initramfs_case(
            TALOS_READ_SYSCALL,
            SyscallArguments::new([
                open_banner.return_value().x0(),
                0x0000_0000_0011_0020,
                64,
                0,
                0,
                0,
            ]),
            &mut store,
            &mut files,
            &mut user_memory,
        );
        let open_init = dispatch_initramfs_case(
            TALOS_OPEN_SYSCALL,
            SyscallArguments::new([
                0x0000_0000_0011_0040,
                crate::initramfs::PHASE8_INIT_PATH.len() as u64,
                0,
                0,
                0,
                0,
            ]),
            &mut store,
            &mut files,
            &mut user_memory,
        );
        let read_init = dispatch_initramfs_case(
            TALOS_READ_SYSCALL,
            SyscallArguments::new([
                open_init.return_value().x0(),
                0x0000_0000_0011_0060,
                4,
                0,
                0,
                0,
            ]),
            &mut store,
            &mut files,
            &mut user_memory,
        );

        assert_eq!(open_banner.number(), SyscallNumber::TalosOpen);
        assert_eq!(open_banner.return_value().x0(), 3);
        assert_eq!(
            read_banner.return_value().x0(),
            crate::initramfs::PHASE8_BANNER_BYTES.len() as u64
        );
        assert_eq!(
            &user_memory[0x20..0x20 + crate::initramfs::PHASE8_BANNER_BYTES.len()],
            crate::initramfs::PHASE8_BANNER_BYTES
        );
        assert_eq!(eof_banner.return_value().x0(), 0);
        assert_eq!(open_init.return_value().x0(), 4);
        assert_eq!(read_init.return_value().x0(), 4);
        assert_eq!(&user_memory[0x60..0x64], b"\x7fELF");
    }

    #[test_case]
    fn talos_open_initramfs_errors_do_not_allocate_descriptors_or_files() {
        let owner = crate::scheduler::ProcessOwnerId::new(31).expect("owner id");
        let mut store = crate::posix::ProcessDescriptorStore::<2, 5>::new_empty();
        store
            .create_owner_with_inherited_stdio(owner)
            .expect("create owner");
        let mut files = crate::initramfs::ReadOnlyFileDescriptions::<1>::new_empty();
        let mut user_memory = [0u8; 128];
        user_memory[..4].copy_from_slice(b"/etc");
        user_memory[0x20..0x20 + crate::initramfs::PHASE8_BANNER_PATH.len()]
            .copy_from_slice(crate::initramfs::PHASE8_BANNER_PATH);

        let invalid_flags = dispatch_initramfs_case(
            TALOS_OPEN_SYSCALL,
            SyscallArguments::new([
                0x0000_0000_0011_0020,
                crate::initramfs::PHASE8_BANNER_PATH.len() as u64,
                1,
                0,
                0,
                0,
            ]),
            &mut store,
            &mut files,
            &mut user_memory,
        );
        let user_fault = dispatch_initramfs_case(
            TALOS_OPEN_SYSCALL,
            SyscallArguments::new([
                0x0000_0000_001e_0000,
                crate::initramfs::PHASE8_BANNER_PATH.len() as u64,
                0,
                0,
                0,
                0,
            ]),
            &mut store,
            &mut files,
            &mut user_memory,
        );
        let directory = dispatch_initramfs_case(
            TALOS_OPEN_SYSCALL,
            SyscallArguments::new([0x0000_0000_0011_0000, 4, 0, 0, 0, 0]),
            &mut store,
            &mut files,
            &mut user_memory,
        );
        let valid = dispatch_initramfs_case(
            TALOS_OPEN_SYSCALL,
            SyscallArguments::new([
                0x0000_0000_0011_0020,
                crate::initramfs::PHASE8_BANNER_PATH.len() as u64,
                0,
                0,
                0,
                0,
            ]),
            &mut store,
            &mut files,
            &mut user_memory,
        );

        assert_eq!(
            invalid_flags.return_value().x0(),
            (EINVAL as u64).wrapping_neg()
        );
        assert_eq!(
            user_fault.return_value().x0(),
            (EFAULT as u64).wrapping_neg()
        );
        assert_eq!(
            directory.return_value().x0(),
            (EISDIR as u64).wrapping_neg()
        );
        assert_eq!(valid.return_value().x0(), 3);
    }

    #[test_case]
    fn talos_read_initramfs_errors_preserve_file_offset_and_user_memory() {
        let owner = crate::scheduler::ProcessOwnerId::new(31).expect("owner id");
        let mut store = crate::posix::ProcessDescriptorStore::<2, 5>::new_empty();
        store
            .create_owner_with_inherited_stdio(owner)
            .expect("create owner");
        let mut files = crate::initramfs::ReadOnlyFileDescriptions::<1>::new_empty();
        let mut user_memory = [0u8; 128];
        user_memory[..crate::initramfs::PHASE8_BANNER_PATH.len()]
            .copy_from_slice(crate::initramfs::PHASE8_BANNER_PATH);

        let open = dispatch_initramfs_case(
            TALOS_OPEN_SYSCALL,
            SyscallArguments::new([
                0x0000_0000_0011_0000,
                crate::initramfs::PHASE8_BANNER_PATH.len() as u64,
                0,
                0,
                0,
                0,
            ]),
            &mut store,
            &mut files,
            &mut user_memory,
        );
        let fault = dispatch_initramfs_case(
            TALOS_READ_SYSCALL,
            SyscallArguments::new([open.return_value().x0(), 0x0000_0000_001e_0000, 6, 0, 0, 0]),
            &mut store,
            &mut files,
            &mut user_memory,
        );
        assert_eq!(fault.return_value().x0(), (EFAULT as u64).wrapping_neg());
        assert_eq!(&user_memory[0x20..0x26], &[0; 6]);

        let ok = dispatch_initramfs_case(
            TALOS_READ_SYSCALL,
            SyscallArguments::new([open.return_value().x0(), 0x0000_0000_0011_0020, 6, 0, 0, 0]),
            &mut store,
            &mut files,
            &mut user_memory,
        );

        assert_eq!(ok.return_value().x0(), 6);
        assert_eq!(&user_memory[0x20..0x26], b"Talos ");
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
