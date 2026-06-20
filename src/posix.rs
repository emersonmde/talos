//! Target-independent POSIX baseline primitives.
//!
//! This module owns only the target-independent POSIX contract surface. It
//! does not perform VFS lookup, syscall ABI translation, process
//! current-working-directory storage, EL0 entry, translation-table switching,
//! or target I/O. The first descriptor-write slice owns only the
//! target-independent stdio-to-runtime-console boundary.

use crate::{
    runtime_console::{self, ConsoleBackend, ConsoleInputBackend},
    scheduler::ProcessOwnerId,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PosixError {
    OperationNotPermitted,
    NoEntry,
    Interrupted,
    Io,
    NotExecutable,
    BadDescriptor,
    NoChild,
    Again,
    NoMemory,
    AccessDenied,
    Fault,
    Busy,
    Exists,
    NoDevice,
    NotDirectory,
    IsDirectory,
    InvalidArgument,
    TooManyOpenFiles,
    NotTty,
    NoSpace,
    Pipe,
    Range,
    NameTooLong,
    NotImplemented,
    NotEmpty,
    NotSupported,
}

impl PosixError {
    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::OperationNotPermitted => "EPERM",
            Self::NoEntry => "ENOENT",
            Self::Interrupted => "EINTR",
            Self::Io => "EIO",
            Self::NotExecutable => "ENOEXEC",
            Self::BadDescriptor => "EBADF",
            Self::NoChild => "ECHILD",
            Self::Again => "EAGAIN",
            Self::NoMemory => "ENOMEM",
            Self::AccessDenied => "EACCES",
            Self::Fault => "EFAULT",
            Self::Busy => "EBUSY",
            Self::Exists => "EEXIST",
            Self::NoDevice => "ENODEV",
            Self::NotDirectory => "ENOTDIR",
            Self::IsDirectory => "EISDIR",
            Self::InvalidArgument => "EINVAL",
            Self::TooManyOpenFiles => "EMFILE",
            Self::NotTty => "ENOTTY",
            Self::NoSpace => "ENOSPC",
            Self::Pipe => "EPIPE",
            Self::Range => "ERANGE",
            Self::NameTooLong => "ENAMETOOLONG",
            Self::NotImplemented => "ENOSYS",
            Self::NotEmpty => "ENOTEMPTY",
            Self::NotSupported => "ENOTSUP",
        }
    }
}

pub(crate) const USER_ADDRESS_SPACE_END: u64 = 0x0000_8000_0000_0000;
pub(crate) const USER_NULL_GUARD_END: u64 = 0x0000_0000_0001_0000;
pub(crate) const DEFAULT_USER_COPY_LIMIT: usize = 1024 * 1024;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UserAccessKind {
    Read,
    Write,
    Execute,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UserMappingPermissions {
    bits: u8,
}

impl UserMappingPermissions {
    pub(crate) const NONE: Self = Self { bits: 0 };
    pub(crate) const READ: Self = Self { bits: 1 << 0 };
    pub(crate) const WRITE: Self = Self { bits: 1 << 1 };
    pub(crate) const EXECUTE: Self = Self { bits: 1 << 2 };
    pub(crate) const USER_TEXT: Self = Self {
        bits: Self::READ.bits | Self::EXECUTE.bits,
    };
    pub(crate) const USER_DATA: Self = Self {
        bits: Self::READ.bits | Self::WRITE.bits,
    };

    pub(crate) const fn contains(self, permissions: Self) -> bool {
        self.bits & permissions.bits == permissions.bits
    }

    pub(crate) const fn allows(self, access: UserAccessKind) -> bool {
        match access {
            UserAccessKind::Read => self.contains(Self::READ),
            UserAccessKind::Write => self.contains(Self::WRITE),
            UserAccessKind::Execute => self.contains(Self::EXECUTE),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UserRange {
    start: u64,
    len: usize,
    end: u64,
}

impl UserRange {
    pub(crate) fn new(start: u64, len: usize, max_len: usize) -> Result<Self, PosixError> {
        if len > max_len {
            return Err(PosixError::Fault);
        }

        let end = start.checked_add(len as u64).ok_or(PosixError::Fault)?;
        if len == 0 {
            if end > USER_ADDRESS_SPACE_END {
                return Err(PosixError::Fault);
            }
            return Ok(Self { start, len, end });
        }

        if !is_non_guard_user_address(start) {
            return Err(PosixError::Fault);
        }
        if end > USER_ADDRESS_SPACE_END {
            return Err(PosixError::Fault);
        }

        Ok(Self { start, len, end })
    }

    pub(crate) const fn start(self) -> u64 {
        self.start
    }

    pub(crate) const fn len(self) -> usize {
        self.len
    }

    pub(crate) const fn end(self) -> u64 {
        self.end
    }

    pub(crate) const fn is_empty(self) -> bool {
        self.len == 0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UserMapping {
    range: UserRange,
    permissions: UserMappingPermissions,
}

impl UserMapping {
    pub(crate) fn new(
        start: u64,
        len: usize,
        permissions: UserMappingPermissions,
    ) -> Result<Self, PosixError> {
        if len == 0 {
            return Err(PosixError::Fault);
        }
        Ok(Self {
            range: UserRange::new(start, len, usize::MAX)?,
            permissions,
        })
    }

    pub(crate) const fn start(self) -> u64 {
        self.range.start()
    }

    pub(crate) const fn end(self) -> u64 {
        self.range.end()
    }

    pub(crate) const fn permissions(self) -> UserMappingPermissions {
        self.permissions
    }

    const fn contains_address(self, address: u64) -> bool {
        self.start() <= address && address < self.end()
    }
}

pub(crate) fn validate_user_memory_access(
    mappings: &[UserMapping],
    start: u64,
    len: usize,
    access: UserAccessKind,
    max_len: usize,
) -> Result<UserRange, PosixError> {
    let range = UserRange::new(start, len, max_len)?;
    if range.is_empty() {
        return Ok(range);
    }

    let mut cursor = range.start();
    while cursor < range.end() {
        let mut next = None;
        let mut index = 0;
        while index < mappings.len() {
            let mapping = mappings[index];
            if mapping.contains_address(cursor) && mapping.permissions().allows(access) {
                next = Some(core::cmp::min(mapping.end(), range.end()));
                break;
            }
            index += 1;
        }

        match next {
            Some(next_cursor) if next_cursor > cursor => cursor = next_cursor,
            _ => return Err(PosixError::Fault),
        }
    }

    Ok(range)
}

pub(crate) fn copy_from_user(
    mappings: &[UserMapping],
    user_memory_start: u64,
    user_memory: &[u8],
    user_start: u64,
    len: usize,
    kernel_dst: &mut [u8],
) -> Result<usize, PosixError> {
    let (offset, end) = validate_user_copy_request(
        mappings,
        user_memory_start,
        user_memory.len(),
        user_start,
        len,
        kernel_dst.len(),
        UserAccessKind::Read,
    )?;
    if len != 0 {
        kernel_dst[..len].copy_from_slice(&user_memory[offset..end]);
    }
    Ok(len)
}

pub(crate) fn copy_to_user(
    mappings: &[UserMapping],
    user_memory_start: u64,
    user_memory: &mut [u8],
    user_start: u64,
    len: usize,
    kernel_src: &[u8],
) -> Result<usize, PosixError> {
    let (offset, end) = validate_user_copy_request(
        mappings,
        user_memory_start,
        user_memory.len(),
        user_start,
        len,
        kernel_src.len(),
        UserAccessKind::Write,
    )?;
    if len != 0 {
        user_memory[offset..end].copy_from_slice(&kernel_src[..len]);
    }
    Ok(len)
}

fn validate_user_copy_request(
    mappings: &[UserMapping],
    user_memory_start: u64,
    user_memory_len: usize,
    user_start: u64,
    len: usize,
    kernel_buffer_len: usize,
    access: UserAccessKind,
) -> Result<(usize, usize), PosixError> {
    if len > DEFAULT_USER_COPY_LIMIT {
        return Err(PosixError::Fault);
    }
    if kernel_buffer_len < len {
        return Err(PosixError::InvalidArgument);
    }

    let range =
        validate_user_memory_access(mappings, user_start, len, access, DEFAULT_USER_COPY_LIMIT)?;
    if range.is_empty() {
        return Ok((0, 0));
    }

    let offset = range
        .start()
        .checked_sub(user_memory_start)
        .ok_or(PosixError::Fault)?;
    let end = range
        .end()
        .checked_sub(user_memory_start)
        .ok_or(PosixError::Fault)?;
    if end > user_memory_len as u64 {
        return Err(PosixError::Fault);
    }

    Ok((offset as usize, end as usize))
}

const fn is_non_guard_user_address(address: u64) -> bool {
    USER_NULL_GUARD_END <= address && address < USER_ADDRESS_SPACE_END
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PathStart {
    Root,
    CurrentWorkingDirectory,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PathLimits {
    pub(crate) max_path_len: usize,
    pub(crate) max_component_len: usize,
    pub(crate) max_components: usize,
}

impl PathLimits {
    pub(crate) const fn new(
        max_path_len: usize,
        max_component_len: usize,
        max_components: usize,
    ) -> Self {
        Self {
            max_path_len,
            max_component_len,
            max_components,
        }
    }
}

pub(crate) const DEFAULT_PATH_LIMITS: PathLimits = PathLimits::new(4096, 255, 64);

pub(crate) const STDIN_FD: usize = 0;
pub(crate) const STDOUT_FD: usize = 1;
pub(crate) const STDERR_FD: usize = 2;
pub(crate) const TERMINAL_EOF_BYTE: u8 = 0x04;
pub(crate) const DEV_NULL_REFERENCE: usize = 0;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct DescriptorFlags {
    bits: u8,
}

impl DescriptorFlags {
    pub(crate) const EMPTY: Self = Self { bits: 0 };
    pub(crate) const CLOSE_ON_EXEC: Self = Self { bits: 1 };

    const KNOWN_BITS: u8 = Self::CLOSE_ON_EXEC.bits;

    pub(crate) const fn from_bits(bits: u8) -> Result<Self, PosixError> {
        if bits & !Self::KNOWN_BITS == 0 {
            Ok(Self { bits })
        } else {
            Err(PosixError::InvalidArgument)
        }
    }

    pub(crate) const fn bits(self) -> u8 {
        self.bits
    }

    pub(crate) const fn contains(self, flag: Self) -> bool {
        self.bits & flag.bits == flag.bits
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum DescriptorAccess {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

impl DescriptorAccess {
    const fn allows_read(self) -> bool {
        matches!(self, Self::ReadOnly | Self::ReadWrite)
    }

    const fn allows_write(self) -> bool {
        matches!(self, Self::WriteOnly | Self::ReadWrite)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum DescriptorObjectKind {
    StdioInput,
    StdioOutput,
    RegularFile,
    Directory,
    PipeEndpoint,
    Socket,
    Device,
    OtherKernelObject,
}

impl DescriptorObjectKind {
    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::StdioInput => "stdio-input",
            Self::StdioOutput => "stdio-output",
            Self::RegularFile => "regular-file",
            Self::Directory => "directory",
            Self::PipeEndpoint => "pipe-endpoint",
            Self::Socket => "socket",
            Self::Device => "device",
            Self::OtherKernelObject => "other-kernel-object",
        }
    }

    const fn supports_tty_operation(self) -> bool {
        matches!(self, Self::StdioInput | Self::StdioOutput)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct DescriptorObject {
    kind: DescriptorObjectKind,
    reference: usize,
}

impl DescriptorObject {
    pub(crate) const fn new(kind: DescriptorObjectKind, reference: usize) -> Self {
        Self { kind, reference }
    }

    pub(crate) const fn kind(self) -> DescriptorObjectKind {
        self.kind
    }

    pub(crate) const fn reference(self) -> usize {
        self.reference
    }

    pub(crate) const fn stdio_stream_name(self) -> &'static str {
        match (self.kind, self.reference) {
            (DescriptorObjectKind::StdioInput, STDIN_FD) => "stdin",
            (DescriptorObjectKind::StdioOutput, STDOUT_FD) => "stdout",
            (DescriptorObjectKind::StdioOutput, STDERR_FD) => "stderr",
            (DescriptorObjectKind::StdioInput, _) => "stdio-input",
            (DescriptorObjectKind::StdioOutput, _) => "stdio-output",
            _ => self.kind.name(),
        }
    }

    pub(crate) const fn runtime_console_route_name(self) -> &'static str {
        match (self.kind, self.reference) {
            (DescriptorObjectKind::StdioInput, STDIN_FD) => "runtime-console0/stdin",
            (DescriptorObjectKind::StdioOutput, STDOUT_FD) => "runtime-console0/stdout",
            (DescriptorObjectKind::StdioOutput, STDERR_FD) => "runtime-console0/stderr",
            (DescriptorObjectKind::Device, DEV_NULL_REFERENCE) => "device:/dev/null",
            (DescriptorObjectKind::StdioInput, _) => "runtime-console0/stdio-input",
            (DescriptorObjectKind::StdioOutput, _) => "runtime-console0/stdio-output",
            _ => "runtime-console0/unsupported",
        }
    }

    pub(crate) const fn is_dev_null(self) -> bool {
        matches!(
            (self.kind, self.reference),
            (DescriptorObjectKind::Device, DEV_NULL_REFERENCE)
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct DescriptorEntry {
    access: DescriptorAccess,
    flags: DescriptorFlags,
    object: DescriptorObject,
}

impl DescriptorEntry {
    pub(crate) const fn new(
        access: DescriptorAccess,
        flags: DescriptorFlags,
        object: DescriptorObject,
    ) -> Self {
        Self {
            access,
            flags,
            object,
        }
    }

    pub(crate) const fn access(self) -> DescriptorAccess {
        self.access
    }

    pub(crate) const fn flags(self) -> DescriptorFlags {
        self.flags
    }

    pub(crate) const fn object(self) -> DescriptorObject {
        self.object
    }

    pub(crate) fn require_readable(self) -> Result<(), PosixError> {
        if self.access.allows_read() {
            Ok(())
        } else {
            Err(PosixError::BadDescriptor)
        }
    }

    pub(crate) fn require_writable(self) -> Result<(), PosixError> {
        if self.access.allows_write() {
            Ok(())
        } else {
            Err(PosixError::BadDescriptor)
        }
    }

    pub(crate) fn require_tty(self) -> Result<(), PosixError> {
        if self.object.kind().supports_tty_operation() {
            Ok(())
        } else {
            Err(PosixError::NotTty)
        }
    }

    pub(crate) const fn unsupported_operation(self) -> Result<(), PosixError> {
        Err(PosixError::NotImplemented)
    }

    pub(crate) const fn unsupported_kind_operation(self) -> Result<(), PosixError> {
        Err(PosixError::NotSupported)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct DescriptorTable<const CAPACITY: usize> {
    entries: [Option<DescriptorEntry>; CAPACITY],
}

impl<const CAPACITY: usize> DescriptorTable<CAPACITY> {
    pub(crate) const fn new_empty() -> Self {
        Self {
            entries: [None; CAPACITY],
        }
    }

    pub(crate) fn with_inherited_stdio() -> Result<Self, PosixError> {
        let mut table = Self::new_empty();
        if CAPACITY <= STDERR_FD {
            return Err(PosixError::TooManyOpenFiles);
        }

        table.entries[STDIN_FD] = Some(DescriptorEntry::new(
            DescriptorAccess::ReadOnly,
            DescriptorFlags::EMPTY,
            DescriptorObject::new(DescriptorObjectKind::StdioInput, STDIN_FD),
        ));
        table.entries[STDOUT_FD] = Some(DescriptorEntry::new(
            DescriptorAccess::WriteOnly,
            DescriptorFlags::EMPTY,
            DescriptorObject::new(DescriptorObjectKind::StdioOutput, STDOUT_FD),
        ));
        table.entries[STDERR_FD] = Some(DescriptorEntry::new(
            DescriptorAccess::WriteOnly,
            DescriptorFlags::EMPTY,
            DescriptorObject::new(DescriptorObjectKind::StdioOutput, STDERR_FD),
        ));
        Ok(table)
    }

    pub(crate) fn get(&self, descriptor: usize) -> Result<DescriptorEntry, PosixError> {
        self.entries
            .get(descriptor)
            .and_then(|entry| *entry)
            .ok_or(PosixError::BadDescriptor)
    }

    pub(crate) fn allocate(&mut self, entry: DescriptorEntry) -> Result<usize, PosixError> {
        let mut descriptor = 0;
        while descriptor < CAPACITY {
            if self.entries[descriptor].is_none() {
                self.entries[descriptor] = Some(entry);
                return Ok(descriptor);
            }
            descriptor += 1;
        }
        Err(PosixError::TooManyOpenFiles)
    }

    pub(crate) fn has_free_slot(&self) -> bool {
        self.entries.iter().any(Option::is_none)
    }

    pub(crate) fn allocate_at(
        &mut self,
        descriptor: usize,
        entry: DescriptorEntry,
    ) -> Result<usize, PosixError> {
        if descriptor >= CAPACITY {
            return Err(PosixError::InvalidArgument);
        }
        if self.entries[descriptor].is_some() {
            return Err(PosixError::InvalidArgument);
        }
        self.entries[descriptor] = Some(entry);
        Ok(descriptor)
    }

    pub(crate) fn close(&mut self, descriptor: usize) -> Result<DescriptorEntry, PosixError> {
        if descriptor >= CAPACITY {
            return Err(PosixError::BadDescriptor);
        }
        self.entries[descriptor]
            .take()
            .ok_or(PosixError::BadDescriptor)
    }

    pub(crate) fn dup(&mut self, descriptor: usize) -> Result<usize, PosixError> {
        let entry = self.get(descriptor)?;
        self.allocate(entry)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ProcessDescriptorOwner<const CAPACITY: usize> {
    owner: ProcessOwnerId,
    table: DescriptorTable<CAPACITY>,
}

impl<const CAPACITY: usize> ProcessDescriptorOwner<CAPACITY> {
    pub(crate) fn with_inherited_stdio(owner: ProcessOwnerId) -> Result<Self, PosixError> {
        Ok(Self {
            owner,
            table: DescriptorTable::with_inherited_stdio()?,
        })
    }

    pub(crate) const fn owner(self) -> ProcessOwnerId {
        self.owner
    }

    pub(crate) const fn descriptor_table(&self) -> &DescriptorTable<CAPACITY> {
        &self.table
    }

    pub(crate) fn descriptor_table_mut(&mut self) -> &mut DescriptorTable<CAPACITY> {
        &mut self.table
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ProcessDescriptorStore<
    const OWNER_CAPACITY: usize,
    const DESCRIPTOR_CAPACITY: usize,
> {
    owners: [Option<ProcessDescriptorOwner<DESCRIPTOR_CAPACITY>>; OWNER_CAPACITY],
}

impl<const OWNER_CAPACITY: usize, const DESCRIPTOR_CAPACITY: usize>
    ProcessDescriptorStore<OWNER_CAPACITY, DESCRIPTOR_CAPACITY>
{
    pub(crate) const fn new_empty() -> Self {
        Self {
            owners: [None; OWNER_CAPACITY],
        }
    }

    pub(crate) fn create_owner_with_inherited_stdio(
        &mut self,
        owner: ProcessOwnerId,
    ) -> Result<(), PosixError> {
        if self.owner_index(owner).is_some() {
            return Err(PosixError::InvalidArgument);
        }

        let owner_record = ProcessDescriptorOwner::with_inherited_stdio(owner)?;
        let mut index = 0;
        while index < OWNER_CAPACITY {
            if self.owners[index].is_none() {
                self.owners[index] = Some(owner_record);
                return Ok(());
            }
            index += 1;
        }

        Err(PosixError::TooManyOpenFiles)
    }

    pub(crate) fn descriptor_table(
        &self,
        owner: ProcessOwnerId,
    ) -> Result<&DescriptorTable<DESCRIPTOR_CAPACITY>, PosixError> {
        let index = self.owner_index(owner).ok_or(PosixError::BadDescriptor)?;
        Ok(self.owners[index]
            .as_ref()
            .expect("owner index only returns occupied slots")
            .descriptor_table())
    }

    pub(crate) fn descriptor_table_mut(
        &mut self,
        owner: ProcessOwnerId,
    ) -> Result<&mut DescriptorTable<DESCRIPTOR_CAPACITY>, PosixError> {
        let index = self.owner_index(owner).ok_or(PosixError::BadDescriptor)?;
        Ok(self.owners[index]
            .as_mut()
            .expect("owner index only returns occupied slots")
            .descriptor_table_mut())
    }

    pub(crate) fn current_descriptor_table(
        &self,
        current_owner: Option<ProcessOwnerId>,
    ) -> Result<&DescriptorTable<DESCRIPTOR_CAPACITY>, PosixError> {
        let owner = current_owner.ok_or(PosixError::BadDescriptor)?;
        self.descriptor_table(owner)
    }

    pub(crate) fn current_descriptor_table_mut(
        &mut self,
        current_owner: Option<ProcessOwnerId>,
    ) -> Result<&mut DescriptorTable<DESCRIPTOR_CAPACITY>, PosixError> {
        let owner = current_owner.ok_or(PosixError::BadDescriptor)?;
        self.descriptor_table_mut(owner)
    }

    pub(crate) fn close_current_descriptor(
        &mut self,
        current_owner: Option<ProcessOwnerId>,
        descriptor: usize,
    ) -> Result<DescriptorEntry, PosixError> {
        self.current_descriptor_table_mut(current_owner)?
            .close(descriptor)
    }

    pub(crate) fn dup_current_descriptor(
        &mut self,
        current_owner: Option<ProcessOwnerId>,
        descriptor: usize,
    ) -> Result<usize, PosixError> {
        self.current_descriptor_table_mut(current_owner)?
            .dup(descriptor)
    }

    fn owner_index(&self, owner: ProcessOwnerId) -> Option<usize> {
        let mut index = 0;
        while index < OWNER_CAPACITY {
            if let Some(record) = self.owners[index] {
                if record.owner() == owner {
                    return Some(index);
                }
            }
            index += 1;
        }
        None
    }
}

pub(crate) fn write_descriptor_to_runtime_console<const CAPACITY: usize, B>(
    table: &DescriptorTable<CAPACITY>,
    descriptor: usize,
    mappings: &[UserMapping],
    user_memory_start: u64,
    user_memory: &[u8],
    user_start: u64,
    len: usize,
    kernel_scratch: &mut [u8],
    console_backend: &mut B,
) -> Result<usize, PosixError>
where
    B: ConsoleBackend,
{
    let entry = table.get(descriptor)?;
    entry.require_writable()?;

    if entry.object().kind() != DescriptorObjectKind::StdioOutput && !entry.object().is_dev_null() {
        return Err(PosixError::NotSupported);
    }
    if len == 0 {
        return Ok(0);
    }
    if len > DEFAULT_USER_COPY_LIMIT {
        return Err(PosixError::InvalidArgument);
    }
    if kernel_scratch.len() < len {
        return Err(PosixError::InvalidArgument);
    }

    let copied = copy_from_user(
        mappings,
        user_memory_start,
        user_memory,
        user_start,
        len,
        &mut kernel_scratch[..len],
    )?;

    if entry.object().is_dev_null() {
        return Ok(copied);
    }

    match runtime_console::write_default_console_bytes(console_backend, &kernel_scratch[..copied]) {
        Ok(result) if result.bytes_written == copied => Ok(copied),
        Ok(_) | Err(_) => Err(PosixError::Io),
    }
}

pub(crate) fn write_kernel_bytes_to_descriptor_console<const CAPACITY: usize, B>(
    table: &DescriptorTable<CAPACITY>,
    descriptor: usize,
    bytes: &[u8],
    console_backend: &mut B,
) -> Result<usize, PosixError>
where
    B: ConsoleBackend,
{
    let entry = table.get(descriptor)?;
    entry.require_writable()?;

    if entry.object().kind() != DescriptorObjectKind::StdioOutput {
        return Err(PosixError::NotSupported);
    }
    if bytes.is_empty() {
        return Ok(0);
    }

    match runtime_console::write_default_console_bytes(console_backend, bytes) {
        Ok(result) if result.bytes_written == bytes.len() => Ok(bytes.len()),
        Ok(_) | Err(_) => Err(PosixError::Io),
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct FixedStdin<'a> {
    bytes: &'a [u8],
    cursor: usize,
}

impl<'a> FixedStdin<'a> {
    pub(crate) const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, cursor: 0 }
    }

    pub(crate) const fn cursor(&self) -> usize {
        self.cursor
    }

    const fn remaining(&self) -> usize {
        self.bytes.len() - self.cursor
    }
}

pub(crate) fn read_descriptor_from_fixed_stdin<const CAPACITY: usize>(
    table: &DescriptorTable<CAPACITY>,
    descriptor: usize,
    mappings: &[UserMapping],
    user_memory_start: u64,
    user_memory: &mut [u8],
    user_start: u64,
    len: usize,
    kernel_scratch: &mut [u8],
    fixed_stdin: Option<&mut FixedStdin<'_>>,
) -> Result<usize, PosixError> {
    let entry = table.get(descriptor)?;
    entry.require_readable()?;

    if entry.object().kind() != DescriptorObjectKind::StdioInput {
        return Err(PosixError::NotSupported);
    }
    if len == 0 {
        return Ok(0);
    }
    if len > DEFAULT_USER_COPY_LIMIT {
        return Err(PosixError::Fault);
    }

    let Some(stdin) = fixed_stdin else {
        return Err(PosixError::NotSupported);
    };
    let selected_len = core::cmp::min(len, stdin.remaining());
    if selected_len == 0 {
        return Ok(0);
    }
    if kernel_scratch.len() < selected_len {
        return Err(PosixError::InvalidArgument);
    }

    let end = stdin.cursor + selected_len;
    kernel_scratch[..selected_len].copy_from_slice(&stdin.bytes[stdin.cursor..end]);
    copy_to_user(
        mappings,
        user_memory_start,
        user_memory,
        user_start,
        selected_len,
        &kernel_scratch[..selected_len],
    )?;
    stdin.cursor = end;
    Ok(selected_len)
}

pub(crate) fn read_descriptor_from_console_input<const CAPACITY: usize, I>(
    table: &DescriptorTable<CAPACITY>,
    descriptor: usize,
    mappings: &[UserMapping],
    user_memory_start: u64,
    user_memory: &mut [u8],
    user_start: u64,
    len: usize,
    kernel_scratch: &mut [u8],
    input_backend: Option<&mut I>,
) -> Result<usize, PosixError>
where
    I: ConsoleInputBackend,
{
    let entry = table.get(descriptor)?;
    entry.require_readable()?;

    if entry.object().kind() != DescriptorObjectKind::StdioInput {
        return Err(PosixError::NotSupported);
    }
    if len == 0 {
        return Ok(0);
    }
    if len > DEFAULT_USER_COPY_LIMIT {
        return Err(PosixError::Fault);
    }
    if kernel_scratch.len() < len {
        return Err(PosixError::InvalidArgument);
    }

    let Some(input_backend) = input_backend else {
        return Err(PosixError::NotSupported);
    };
    let mut selected_len = 0;
    while selected_len < len {
        let Some(byte) = input_backend.poll_read_byte() else {
            break;
        };
        if selected_len == 0 && byte == TERMINAL_EOF_BYTE {
            return Ok(0);
        }
        kernel_scratch[selected_len] = byte;
        selected_len += 1;
    }
    if selected_len == 0 {
        return Err(PosixError::Again);
    }

    copy_to_user(
        mappings,
        user_memory_start,
        user_memory,
        user_start,
        selected_len,
        &kernel_scratch[..selected_len],
    )?;
    Ok(selected_len)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PathComponent<'a> {
    bytes: &'a [u8],
}

impl<'a> PathComponent<'a> {
    const EMPTY: Self = Self { bytes: b"" };

    pub(crate) const fn bytes(self) -> &'a [u8] {
        self.bytes
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct NormalizedPath<'a, const MAX_COMPONENTS: usize> {
    start: PathStart,
    components: [PathComponent<'a>; MAX_COMPONENTS],
    component_count: usize,
    requires_directory: bool,
}

impl<'a, const MAX_COMPONENTS: usize> NormalizedPath<'a, MAX_COMPONENTS> {
    pub(crate) const fn start(self) -> PathStart {
        self.start
    }

    pub(crate) const fn component_count(self) -> usize {
        self.component_count
    }

    pub(crate) fn components(&self) -> &[PathComponent<'a>] {
        &self.components[..self.component_count]
    }

    pub(crate) const fn requires_directory(self) -> bool {
        self.requires_directory
    }
}

pub(crate) fn normalize_path<'a, const MAX_COMPONENTS: usize>(
    path: &'a [u8],
    limits: PathLimits,
) -> Result<NormalizedPath<'a, MAX_COMPONENTS>, PosixError> {
    if path.is_empty() {
        return Err(PosixError::NoEntry);
    }
    if path.len() > limits.max_path_len {
        return Err(PosixError::NameTooLong);
    }
    if contains_nul(path) {
        return Err(PosixError::InvalidArgument);
    }

    let start = if path[0] == b'/' {
        PathStart::Root
    } else {
        PathStart::CurrentWorkingDirectory
    };
    let mut normalized = NormalizedPath {
        start,
        components: [PathComponent::EMPTY; MAX_COMPONENTS],
        component_count: 0,
        requires_directory: false,
    };
    let component_capacity = core::cmp::min(MAX_COMPONENTS, limits.max_components);

    let mut offset = 0;
    while offset < path.len() {
        while offset < path.len() && path[offset] == b'/' {
            offset += 1;
        }
        if offset == path.len() {
            break;
        }

        let component_start = offset;
        while offset < path.len() && path[offset] != b'/' {
            offset += 1;
        }
        let component = &path[component_start..offset];
        if component.len() > limits.max_component_len {
            return Err(PosixError::NameTooLong);
        }

        if component == b"." {
            continue;
        }
        if component == b".." {
            match start {
                PathStart::Root => {
                    if normalized.component_count != 0 {
                        normalized.component_count -= 1;
                    }
                }
                PathStart::CurrentWorkingDirectory => {
                    if normalized.component_count != 0
                        && normalized.components[normalized.component_count - 1].bytes != b".."
                    {
                        normalized.component_count -= 1;
                    } else {
                        push_component(&mut normalized, component, component_capacity)?;
                    }
                }
            }
            continue;
        }

        push_component(&mut normalized, component, component_capacity)?;
    }

    normalized.requires_directory = path.len() > 1
        && path[path.len() - 1] == b'/'
        && !(normalized.start == PathStart::Root && normalized.component_count == 0);

    Ok(normalized)
}

fn contains_nul(path: &[u8]) -> bool {
    let mut index = 0;
    while index < path.len() {
        if path[index] == 0 {
            return true;
        }
        index += 1;
    }
    false
}

fn push_component<'a, const MAX_COMPONENTS: usize>(
    normalized: &mut NormalizedPath<'a, MAX_COMPONENTS>,
    component: &'a [u8],
    component_capacity: usize,
) -> Result<(), PosixError> {
    if normalized.component_count >= component_capacity {
        return Err(PosixError::NameTooLong);
    }
    normalized.components[normalized.component_count] = PathComponent { bytes: component };
    normalized.component_count += 1;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_LIMITS: PathLimits = PathLimits::new(64, 8, 4);

    fn normalize(path: &[u8]) -> Result<NormalizedPath<'_, 4>, PosixError> {
        normalize_path(path, TEST_LIMITS)
    }

    fn regular_file(reference: usize) -> DescriptorEntry {
        DescriptorEntry::new(
            DescriptorAccess::ReadWrite,
            DescriptorFlags::EMPTY,
            DescriptorObject::new(DescriptorObjectKind::RegularFile, reference),
        )
    }

    fn user_mapping(start: u64, len: usize, permissions: UserMappingPermissions) -> UserMapping {
        UserMapping::new(start, len, permissions).expect("user mapping")
    }

    struct CaptureConsole {
        bytes: [u8; 32],
        len: usize,
    }

    impl CaptureConsole {
        const fn new() -> Self {
            Self {
                bytes: [0; 32],
                len: 0,
            }
        }

        fn as_bytes(&self) -> &[u8] {
            &self.bytes[..self.len]
        }
    }

    impl ConsoleBackend for CaptureConsole {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            self.write_bytes(s.as_bytes())
        }

        fn write_bytes(&mut self, bytes: &[u8]) -> core::fmt::Result {
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

    fn assert_components(path: &NormalizedPath<'_, 4>, expected: &[&[u8]]) {
        assert_eq!(path.component_count(), expected.len());
        let components = path.components();
        let mut index = 0;
        while index < expected.len() {
            assert_eq!(components[index].bytes(), expected[index]);
            index += 1;
        }
    }

    #[test_case]
    fn posix_error_names_match_baseline_contract() {
        let errors = [
            (PosixError::OperationNotPermitted, "EPERM"),
            (PosixError::NoEntry, "ENOENT"),
            (PosixError::Interrupted, "EINTR"),
            (PosixError::Io, "EIO"),
            (PosixError::NotExecutable, "ENOEXEC"),
            (PosixError::BadDescriptor, "EBADF"),
            (PosixError::NoChild, "ECHILD"),
            (PosixError::Again, "EAGAIN"),
            (PosixError::NoMemory, "ENOMEM"),
            (PosixError::AccessDenied, "EACCES"),
            (PosixError::Fault, "EFAULT"),
            (PosixError::Busy, "EBUSY"),
            (PosixError::Exists, "EEXIST"),
            (PosixError::NoDevice, "ENODEV"),
            (PosixError::NotDirectory, "ENOTDIR"),
            (PosixError::IsDirectory, "EISDIR"),
            (PosixError::InvalidArgument, "EINVAL"),
            (PosixError::TooManyOpenFiles, "EMFILE"),
            (PosixError::NotTty, "ENOTTY"),
            (PosixError::NoSpace, "ENOSPC"),
            (PosixError::Pipe, "EPIPE"),
            (PosixError::Range, "ERANGE"),
            (PosixError::NameTooLong, "ENAMETOOLONG"),
            (PosixError::NotImplemented, "ENOSYS"),
            (PosixError::NotEmpty, "ENOTEMPTY"),
            (PosixError::NotSupported, "ENOTSUP"),
        ];

        let mut index = 0;
        while index < errors.len() {
            let (error, name) = errors[index];
            assert_eq!(error.name(), name);
            index += 1;
        }
    }

    #[test_case]
    fn descriptor_flags_accept_known_bits_and_reject_unknown_bits() {
        let flags =
            DescriptorFlags::from_bits(DescriptorFlags::CLOSE_ON_EXEC.bits()).expect("known flag");

        assert!(flags.contains(DescriptorFlags::CLOSE_ON_EXEC));
        assert_eq!(flags.bits(), 1);
        assert_eq!(
            DescriptorFlags::from_bits(0b10),
            Err(PosixError::InvalidArgument)
        );
    }

    #[test_case]
    fn descriptor_object_kind_names_cover_reserved_future_kinds() {
        let kinds = [
            (DescriptorObjectKind::StdioInput, "stdio-input"),
            (DescriptorObjectKind::StdioOutput, "stdio-output"),
            (DescriptorObjectKind::RegularFile, "regular-file"),
            (DescriptorObjectKind::Directory, "directory"),
            (DescriptorObjectKind::PipeEndpoint, "pipe-endpoint"),
            (DescriptorObjectKind::Socket, "socket"),
            (DescriptorObjectKind::Device, "device"),
            (
                DescriptorObjectKind::OtherKernelObject,
                "other-kernel-object",
            ),
        ];

        let mut index = 0;
        while index < kinds.len() {
            let (kind, name) = kinds[index];
            assert_eq!(kind.name(), name);
            index += 1;
        }
    }

    #[test_case]
    fn inherited_stdio_populates_process_local_reserved_descriptors() {
        let table = DescriptorTable::<4>::with_inherited_stdio().expect("stdio table");
        let stdin = table.get(STDIN_FD).expect("stdin");
        let stdout = table.get(STDOUT_FD).expect("stdout");
        let stderr = table.get(STDERR_FD).expect("stderr");

        assert_eq!(stdin.access(), DescriptorAccess::ReadOnly);
        assert_eq!(stdin.flags(), DescriptorFlags::EMPTY);
        assert_eq!(stdin.object().kind(), DescriptorObjectKind::StdioInput);
        assert_eq!(stdin.object().reference(), STDIN_FD);
        assert_eq!(stdout.access(), DescriptorAccess::WriteOnly);
        assert_eq!(stdout.object().kind(), DescriptorObjectKind::StdioOutput);
        assert_eq!(stderr.access(), DescriptorAccess::WriteOnly);
        assert_eq!(stderr.object().kind(), DescriptorObjectKind::StdioOutput);
        assert_ne!(stdout.object().reference(), stderr.object().reference());
        assert_eq!(stdout.object().stdio_stream_name(), "stdout");
        assert_eq!(
            stdout.object().runtime_console_route_name(),
            "runtime-console0/stdout"
        );
        assert_eq!(stderr.object().stdio_stream_name(), "stderr");
        assert_eq!(
            stderr.object().runtime_console_route_name(),
            "runtime-console0/stderr"
        );
        assert_eq!(stdin.require_readable(), Ok(()));
        assert_eq!(stdout.require_writable(), Ok(()));
        assert_eq!(stderr.require_tty(), Ok(()));
    }

    #[test_case]
    fn inherited_stdio_requires_room_for_descriptors_zero_one_and_two() {
        assert_eq!(
            DescriptorTable::<2>::with_inherited_stdio(),
            Err(PosixError::TooManyOpenFiles)
        );
    }

    #[test_case]
    fn descriptor_allocate_uses_lowest_available_slot() {
        let mut table = DescriptorTable::<4>::with_inherited_stdio().expect("stdio table");

        table.close(STDOUT_FD).expect("close stdout");

        assert_eq!(table.allocate(regular_file(10)), Ok(STDOUT_FD));
        assert_eq!(
            table
                .get(STDOUT_FD)
                .expect("allocated")
                .object()
                .reference(),
            10
        );
    }

    #[test_case]
    fn descriptor_allocate_at_rejects_invalid_or_occupied_target() {
        let mut table = DescriptorTable::<3>::with_inherited_stdio().expect("stdio table");

        assert_eq!(
            table.allocate_at(3, regular_file(10)),
            Err(PosixError::InvalidArgument)
        );
        assert_eq!(
            table.allocate_at(STDIN_FD, regular_file(10)),
            Err(PosixError::InvalidArgument)
        );
    }

    #[test_case]
    fn descriptor_get_close_and_double_close_use_ebadf() {
        let mut table = DescriptorTable::<4>::with_inherited_stdio().expect("stdio table");

        assert_eq!(table.get(4), Err(PosixError::BadDescriptor));
        assert_eq!(
            table
                .close(STDERR_FD)
                .expect("close stderr")
                .object()
                .reference(),
            STDERR_FD
        );
        assert_eq!(table.get(STDERR_FD), Err(PosixError::BadDescriptor));
        assert_eq!(table.close(STDERR_FD), Err(PosixError::BadDescriptor));
    }

    #[test_case]
    fn descriptor_dup_preserves_object_reference_and_separate_lifetime() {
        let mut table = DescriptorTable::<4>::with_inherited_stdio().expect("stdio table");

        let duplicate = table.dup(STDOUT_FD).expect("dup stdout");

        assert_eq!(duplicate, 3);
        assert_eq!(
            table.get(duplicate).expect("duplicate").object(),
            table.get(STDOUT_FD).expect("stdout").object()
        );
        table.close(STDOUT_FD).expect("close original");
        assert_eq!(
            table
                .get(duplicate)
                .expect("duplicate remains")
                .object()
                .kind(),
            DescriptorObjectKind::StdioOutput
        );
        assert_eq!(table.dup(STDOUT_FD), Err(PosixError::BadDescriptor));
    }

    #[test_case]
    fn descriptor_full_table_maps_allocate_and_dup_to_emfile() {
        let mut table = DescriptorTable::<3>::with_inherited_stdio().expect("stdio table");

        assert_eq!(
            table.allocate(regular_file(10)),
            Err(PosixError::TooManyOpenFiles)
        );
        assert_eq!(table.dup(STDOUT_FD), Err(PosixError::TooManyOpenFiles));
    }

    #[test_case]
    fn descriptor_access_mismatch_maps_to_ebadf() {
        let table = DescriptorTable::<3>::with_inherited_stdio().expect("stdio table");
        let stdin = table.get(STDIN_FD).expect("stdin");
        let stdout = table.get(STDOUT_FD).expect("stdout");
        let read_write = regular_file(10);

        assert_eq!(stdin.require_writable(), Err(PosixError::BadDescriptor));
        assert_eq!(stdout.require_readable(), Err(PosixError::BadDescriptor));
        assert_eq!(read_write.require_readable(), Ok(()));
        assert_eq!(read_write.require_writable(), Ok(()));
    }

    #[test_case]
    fn descriptor_reserved_operation_errors_are_deterministic() {
        let file = regular_file(10);

        assert_eq!(file.require_tty(), Err(PosixError::NotTty));
        assert_eq!(
            file.unsupported_operation(),
            Err(PosixError::NotImplemented)
        );
        assert_eq!(
            file.unsupported_kind_operation(),
            Err(PosixError::NotSupported)
        );
    }

    #[test_case]
    fn process_descriptor_owner_initializes_inherited_stdio_for_owner() {
        let owner = ProcessOwnerId::new(7).expect("owner id");
        let process =
            ProcessDescriptorOwner::<4>::with_inherited_stdio(owner).expect("process owner");
        let table = process.descriptor_table();

        assert_eq!(process.owner(), owner);
        assert_eq!(
            table.get(STDIN_FD).expect("stdin").object().kind(),
            DescriptorObjectKind::StdioInput
        );
        assert_eq!(
            table.get(STDOUT_FD).expect("stdout").object().kind(),
            DescriptorObjectKind::StdioOutput
        );
        assert_eq!(
            table.get(STDERR_FD).expect("stderr").object().kind(),
            DescriptorObjectKind::StdioOutput
        );
    }

    #[test_case]
    fn process_descriptor_store_resolves_current_owner_table() {
        let owner = ProcessOwnerId::new(11).expect("owner id");
        let mut store = ProcessDescriptorStore::<2, 4>::new_empty();

        store
            .create_owner_with_inherited_stdio(owner)
            .expect("create owner");
        assert_eq!(
            store
                .current_descriptor_table(Some(owner))
                .expect("current table")
                .get(STDOUT_FD)
                .expect("stdout")
                .object()
                .reference(),
            STDOUT_FD
        );
        store
            .current_descriptor_table_mut(Some(owner))
            .expect("current table")
            .close(STDOUT_FD)
            .expect("close stdout");
        assert_eq!(
            store
                .descriptor_table(owner)
                .expect("owner table")
                .get(STDOUT_FD),
            Err(PosixError::BadDescriptor)
        );
    }

    #[test_case]
    fn process_descriptor_close_stdout_blocks_descriptor_write_lookup() {
        let owner = ProcessOwnerId::new(23).expect("owner id");
        let mut store = ProcessDescriptorStore::<1, 4>::new_empty();
        let mappings = [user_mapping(
            USER_NULL_GUARD_END,
            0x20,
            UserMappingPermissions::USER_DATA,
        )];
        let user_memory = *b"closed-stdout\n\0\0";
        let mut scratch = [0u8; 32];
        let mut console = CaptureConsole::new();

        store
            .create_owner_with_inherited_stdio(owner)
            .expect("create owner");
        let closed = store
            .close_current_descriptor(Some(owner), STDOUT_FD)
            .expect("close stdout");

        assert_eq!(closed.object().kind(), DescriptorObjectKind::StdioOutput);
        assert_eq!(closed.object().reference(), STDOUT_FD);
        assert_eq!(
            write_descriptor_to_runtime_console(
                store
                    .current_descriptor_table(Some(owner))
                    .expect("current table"),
                STDOUT_FD,
                &mappings,
                USER_NULL_GUARD_END,
                &user_memory,
                USER_NULL_GUARD_END,
                14,
                &mut scratch,
                &mut console,
            ),
            Err(PosixError::BadDescriptor)
        );
        assert_eq!(console.as_bytes(), b"");
    }

    #[test_case]
    fn process_descriptor_close_stderr_follows_table_local_rule() {
        let owner = ProcessOwnerId::new(24).expect("owner id");
        let mut store = ProcessDescriptorStore::<1, 4>::new_empty();

        store
            .create_owner_with_inherited_stdio(owner)
            .expect("create owner");
        let closed = store
            .close_current_descriptor(Some(owner), STDERR_FD)
            .expect("close stderr");

        assert_eq!(closed.object().kind(), DescriptorObjectKind::StdioOutput);
        assert_eq!(closed.object().reference(), STDERR_FD);
        assert_eq!(
            store
                .current_descriptor_table(Some(owner))
                .expect("current table")
                .get(STDERR_FD),
            Err(PosixError::BadDescriptor)
        );
    }

    #[test_case]
    fn process_descriptor_close_failures_map_to_ebadf() {
        let owner = ProcessOwnerId::new(25).expect("owner id");
        let missing = ProcessOwnerId::new(26).expect("missing owner id");
        let mut store = ProcessDescriptorStore::<1, 4>::new_empty();

        store
            .create_owner_with_inherited_stdio(owner)
            .expect("create owner");

        assert_eq!(
            store.close_current_descriptor(None, STDOUT_FD),
            Err(PosixError::BadDescriptor)
        );
        assert_eq!(
            store.close_current_descriptor(Some(missing), STDOUT_FD),
            Err(PosixError::BadDescriptor)
        );
        assert_eq!(
            store.close_current_descriptor(Some(owner), 4),
            Err(PosixError::BadDescriptor)
        );
        assert!(
            store
                .close_current_descriptor(Some(owner), STDOUT_FD)
                .is_ok()
        );
        assert_eq!(
            store.close_current_descriptor(Some(owner), STDOUT_FD),
            Err(PosixError::BadDescriptor)
        );
    }

    #[test_case]
    fn process_descriptor_close_reuses_lowest_slot_and_preserves_duplicates() {
        let owner = ProcessOwnerId::new(27).expect("owner id");
        let mut store = ProcessDescriptorStore::<1, 4>::new_empty();

        store
            .create_owner_with_inherited_stdio(owner)
            .expect("create owner");
        let duplicate = store
            .current_descriptor_table_mut(Some(owner))
            .expect("current table")
            .dup(STDOUT_FD)
            .expect("dup stdout");

        assert_eq!(duplicate, 3);
        store
            .close_current_descriptor(Some(owner), STDOUT_FD)
            .expect("close original");
        assert_eq!(
            store
                .current_descriptor_table(Some(owner))
                .expect("current table")
                .get(duplicate)
                .expect("duplicate remains")
                .object()
                .reference(),
            STDOUT_FD
        );
        assert_eq!(
            store
                .current_descriptor_table_mut(Some(owner))
                .expect("current table")
                .allocate(regular_file(99)),
            Ok(STDOUT_FD)
        );
        assert_eq!(
            store
                .current_descriptor_table_mut(Some(owner))
                .expect("current table")
                .dup(4),
            Err(PosixError::BadDescriptor)
        );
        assert_eq!(
            store
                .close_current_descriptor(Some(owner), duplicate)
                .expect("close duplicate")
                .object()
                .reference(),
            STDOUT_FD
        );
        assert_eq!(
            store
                .close_current_descriptor(Some(owner), STDOUT_FD)
                .expect("close reused slot")
                .object()
                .reference(),
            99
        );
    }

    #[test_case]
    fn process_descriptor_store_dups_current_owner_descriptors() {
        let owner = ProcessOwnerId::new(28).expect("owner id");
        let mut store = ProcessDescriptorStore::<1, 5>::new_empty();

        store
            .create_owner_with_inherited_stdio(owner)
            .expect("create owner");
        let stdout_duplicate = store
            .dup_current_descriptor(Some(owner), STDOUT_FD)
            .expect("dup stdout");
        let stderr_duplicate = store
            .dup_current_descriptor(Some(owner), STDERR_FD)
            .expect("dup stderr");

        assert_eq!(stdout_duplicate, 3);
        assert_eq!(stderr_duplicate, 4);
        assert_eq!(
            store
                .current_descriptor_table(Some(owner))
                .expect("current table")
                .get(stdout_duplicate)
                .expect("stdout duplicate")
                .object(),
            store
                .current_descriptor_table(Some(owner))
                .expect("current table")
                .get(STDOUT_FD)
                .expect("stdout")
                .object()
        );
        store
            .close_current_descriptor(Some(owner), STDOUT_FD)
            .expect("close stdout");
        assert_eq!(
            store
                .current_descriptor_table(Some(owner))
                .expect("current table")
                .get(stdout_duplicate)
                .expect("duplicate remains")
                .object()
                .reference(),
            STDOUT_FD
        );
    }

    #[test_case]
    fn process_descriptor_dup_failures_map_to_ebadf_or_emfile() {
        let owner = ProcessOwnerId::new(29).expect("owner id");
        let missing = ProcessOwnerId::new(30).expect("missing owner id");
        let mut full_store = ProcessDescriptorStore::<1, 3>::new_empty();
        let mut store = ProcessDescriptorStore::<1, 4>::new_empty();

        full_store
            .create_owner_with_inherited_stdio(owner)
            .expect("create owner");
        store
            .create_owner_with_inherited_stdio(owner)
            .expect("create owner");

        assert_eq!(
            store.dup_current_descriptor(None, STDOUT_FD),
            Err(PosixError::BadDescriptor)
        );
        assert_eq!(
            store.dup_current_descriptor(Some(missing), STDOUT_FD),
            Err(PosixError::BadDescriptor)
        );
        assert_eq!(
            store.dup_current_descriptor(Some(owner), 4),
            Err(PosixError::BadDescriptor)
        );
        store
            .close_current_descriptor(Some(owner), STDOUT_FD)
            .expect("close stdout");
        assert_eq!(
            store.dup_current_descriptor(Some(owner), STDOUT_FD),
            Err(PosixError::BadDescriptor)
        );
        assert_eq!(
            full_store.dup_current_descriptor(Some(owner), STDOUT_FD),
            Err(PosixError::TooManyOpenFiles)
        );
    }

    #[test_case]
    fn process_descriptor_lookup_failures_map_to_ebadf() {
        let owner = ProcessOwnerId::new(17).expect("owner id");
        let missing = ProcessOwnerId::new(18).expect("missing owner id");
        let mut store = ProcessDescriptorStore::<1, 4>::new_empty();

        store
            .create_owner_with_inherited_stdio(owner)
            .expect("create owner");

        assert_eq!(
            store.current_descriptor_table(None),
            Err(PosixError::BadDescriptor)
        );
        assert_eq!(
            store.current_descriptor_table(Some(missing)),
            Err(PosixError::BadDescriptor)
        );
        assert_eq!(
            store.current_descriptor_table_mut(Some(missing)),
            Err(PosixError::BadDescriptor)
        );
    }

    #[test_case]
    fn process_descriptor_store_preserves_owner_and_table_errors() {
        let owner = ProcessOwnerId::new(21).expect("owner id");
        let second = ProcessOwnerId::new(22).expect("second owner id");
        let mut store = ProcessDescriptorStore::<1, 4>::new_empty();

        store
            .create_owner_with_inherited_stdio(owner)
            .expect("create owner");

        assert_eq!(
            store.create_owner_with_inherited_stdio(owner),
            Err(PosixError::InvalidArgument)
        );
        assert_eq!(
            store.create_owner_with_inherited_stdio(second),
            Err(PosixError::TooManyOpenFiles)
        );
        assert_eq!(
            ProcessDescriptorOwner::<2>::with_inherited_stdio(owner),
            Err(PosixError::TooManyOpenFiles)
        );
    }

    #[test_case]
    fn user_range_rejects_null_guard_and_kernel_addresses_for_nonempty_ranges_as_efault() {
        assert_eq!(
            UserRange::new(0, 1, DEFAULT_USER_COPY_LIMIT),
            Err(PosixError::Fault)
        );
        assert_eq!(
            UserRange::new(USER_NULL_GUARD_END - 1, 1, DEFAULT_USER_COPY_LIMIT),
            Err(PosixError::Fault)
        );
        assert_eq!(
            UserRange::new(USER_ADDRESS_SPACE_END, 0, DEFAULT_USER_COPY_LIMIT),
            Ok(UserRange {
                start: USER_ADDRESS_SPACE_END,
                len: 0,
                end: USER_ADDRESS_SPACE_END
            })
        );
        assert_eq!(
            UserRange::new(USER_ADDRESS_SPACE_END, 1, DEFAULT_USER_COPY_LIMIT),
            Err(PosixError::Fault)
        );
    }

    #[test_case]
    fn user_range_rejects_wraparound_and_length_limit_as_efault() {
        assert_eq!(
            UserRange::new(u64::MAX - 4, 8, DEFAULT_USER_COPY_LIMIT),
            Err(PosixError::Fault)
        );
        assert_eq!(
            UserRange::new(
                USER_NULL_GUARD_END,
                DEFAULT_USER_COPY_LIMIT + 1,
                DEFAULT_USER_COPY_LIMIT
            ),
            Err(PosixError::Fault)
        );
    }

    #[test_case]
    fn user_mapping_rejects_zero_length_and_null_guard_overlap() {
        assert_eq!(
            UserMapping::new(USER_NULL_GUARD_END, 0, UserMappingPermissions::USER_DATA),
            Err(PosixError::Fault)
        );
        assert_eq!(
            UserMapping::new(
                USER_NULL_GUARD_END - 0x1000,
                0x2000,
                UserMappingPermissions::USER_DATA
            ),
            Err(PosixError::Fault)
        );
    }

    #[test_case]
    fn user_access_accepts_contiguous_readable_ranges() {
        let mappings = [
            user_mapping(
                USER_NULL_GUARD_END,
                0x1000,
                UserMappingPermissions::USER_TEXT,
            ),
            user_mapping(
                USER_NULL_GUARD_END + 0x1000,
                0x2000,
                UserMappingPermissions::USER_DATA,
            ),
        ];

        let range = validate_user_memory_access(
            &mappings,
            USER_NULL_GUARD_END + 0x800,
            0x1800,
            UserAccessKind::Read,
            DEFAULT_USER_COPY_LIMIT,
        )
        .expect("readable range");

        assert_eq!(range.start(), USER_NULL_GUARD_END + 0x800);
        assert_eq!(range.len(), 0x1800);
        assert_eq!(range.end(), USER_NULL_GUARD_END + 0x2000);
    }

    #[test_case]
    fn user_access_requires_matching_read_write_and_execute_permissions() {
        let mappings = [
            user_mapping(
                USER_NULL_GUARD_END,
                0x1000,
                UserMappingPermissions::USER_TEXT,
            ),
            user_mapping(
                USER_NULL_GUARD_END + 0x1000,
                0x1000,
                UserMappingPermissions::USER_DATA,
            ),
        ];

        assert_eq!(
            validate_user_memory_access(
                &mappings,
                USER_NULL_GUARD_END,
                0x100,
                UserAccessKind::Execute,
                DEFAULT_USER_COPY_LIMIT
            )
            .map(|range| range.len()),
            Ok(0x100)
        );
        assert_eq!(
            validate_user_memory_access(
                &mappings,
                USER_NULL_GUARD_END,
                0x100,
                UserAccessKind::Write,
                DEFAULT_USER_COPY_LIMIT
            ),
            Err(PosixError::Fault)
        );
        assert_eq!(
            validate_user_memory_access(
                &mappings,
                USER_NULL_GUARD_END + 0x1000,
                0x100,
                UserAccessKind::Execute,
                DEFAULT_USER_COPY_LIMIT
            ),
            Err(PosixError::Fault)
        );
    }

    #[test_case]
    fn user_access_rejects_unmapped_and_guard_gaps_as_efault() {
        let mappings = [
            user_mapping(
                USER_NULL_GUARD_END,
                0x1000,
                UserMappingPermissions::USER_DATA,
            ),
            user_mapping(
                USER_NULL_GUARD_END + 0x2000,
                0x1000,
                UserMappingPermissions::NONE,
            ),
        ];

        assert_eq!(
            validate_user_memory_access(
                &mappings,
                USER_NULL_GUARD_END + 0x800,
                0x1000,
                UserAccessKind::Read,
                DEFAULT_USER_COPY_LIMIT
            ),
            Err(PosixError::Fault)
        );
        assert_eq!(
            validate_user_memory_access(
                &mappings,
                USER_NULL_GUARD_END + 0x2000,
                0x100,
                UserAccessKind::Read,
                DEFAULT_USER_COPY_LIMIT
            ),
            Err(PosixError::Fault)
        );
    }

    #[test_case]
    fn copy_from_user_reads_complete_valid_range() {
        let mappings = [user_mapping(
            USER_NULL_GUARD_END,
            0x100,
            UserMappingPermissions::USER_DATA,
        )];
        let user_memory = [1, 2, 3, 4, 5, 6];
        let mut kernel_dst = [0xaa; 6];

        assert_eq!(
            copy_from_user(
                &mappings,
                USER_NULL_GUARD_END,
                &user_memory,
                USER_NULL_GUARD_END + 1,
                4,
                &mut kernel_dst
            ),
            Ok(4)
        );
        assert_eq!(kernel_dst, [2, 3, 4, 5, 0xaa, 0xaa]);
    }

    #[test_case]
    fn copy_to_user_writes_complete_valid_range() {
        let mappings = [user_mapping(
            USER_NULL_GUARD_END,
            0x100,
            UserMappingPermissions::USER_DATA,
        )];
        let mut user_memory = [0xaa; 6];
        let kernel_src = [1, 2, 3, 4, 5, 6];

        assert_eq!(
            copy_to_user(
                &mappings,
                USER_NULL_GUARD_END,
                &mut user_memory,
                USER_NULL_GUARD_END + 2,
                3,
                &kernel_src
            ),
            Ok(3)
        );
        assert_eq!(user_memory, [0xaa, 0xaa, 1, 2, 3, 0xaa]);
    }

    #[test_case]
    fn copy_user_zero_length_succeeds_without_dereferencing_user_start() {
        let mappings: [UserMapping; 0] = [];
        let user_memory: [u8; 0] = [];
        let mut kernel_dst = [0xaa; 2];
        let mut user_dst = [0xbbu8; 2];

        assert_eq!(
            copy_from_user(
                &mappings,
                USER_NULL_GUARD_END,
                &user_memory,
                0,
                0,
                &mut kernel_dst
            ),
            Ok(0)
        );
        assert_eq!(
            copy_to_user(
                &mappings,
                USER_NULL_GUARD_END,
                &mut user_dst,
                USER_ADDRESS_SPACE_END,
                0,
                &[]
            ),
            Ok(0)
        );
        assert_eq!(kernel_dst, [0xaa; 2]);
        assert_eq!(user_dst, [0xbb; 2]);
    }

    #[test_case]
    fn copy_user_rejects_invalid_user_ranges_as_efault() {
        let mappings = [user_mapping(
            USER_NULL_GUARD_END,
            0x100,
            UserMappingPermissions::USER_DATA,
        )];
        let user_memory = [0xaa; 0x100];
        let mut kernel_dst = [0xbb; 8];

        assert_eq!(
            copy_from_user(
                &mappings,
                USER_NULL_GUARD_END,
                &user_memory,
                0,
                1,
                &mut kernel_dst
            ),
            Err(PosixError::Fault)
        );
        assert_eq!(
            copy_from_user(
                &mappings,
                USER_NULL_GUARD_END,
                &user_memory,
                u64::MAX - 1,
                2,
                &mut kernel_dst
            ),
            Err(PosixError::Fault)
        );
        assert_eq!(
            copy_from_user(
                &mappings,
                USER_NULL_GUARD_END,
                &user_memory,
                USER_NULL_GUARD_END,
                DEFAULT_USER_COPY_LIMIT + 1,
                &mut kernel_dst
            ),
            Err(PosixError::Fault)
        );
        assert_eq!(kernel_dst, [0xbb; 8]);
    }

    #[test_case]
    fn copy_user_rejects_unmapped_no_access_and_permission_mismatch_as_efault() {
        let mappings = [
            user_mapping(USER_NULL_GUARD_END, 0x10, UserMappingPermissions::USER_TEXT),
            user_mapping(
                USER_NULL_GUARD_END + 0x20,
                0x10,
                UserMappingPermissions::NONE,
            ),
        ];
        let mut user_memory = [0xaa; 0x40];
        let user_before = user_memory;
        let mut kernel_dst = [0xbb; 0x20];

        assert_eq!(
            copy_from_user(
                &mappings,
                USER_NULL_GUARD_END,
                &user_memory,
                USER_NULL_GUARD_END + 0x8,
                0x10,
                &mut kernel_dst
            ),
            Err(PosixError::Fault)
        );
        assert_eq!(
            copy_from_user(
                &mappings,
                USER_NULL_GUARD_END,
                &user_memory,
                USER_NULL_GUARD_END + 0x20,
                1,
                &mut kernel_dst
            ),
            Err(PosixError::Fault)
        );
        assert_eq!(
            copy_to_user(
                &mappings,
                USER_NULL_GUARD_END,
                &mut user_memory,
                USER_NULL_GUARD_END,
                4,
                &[1, 2, 3, 4]
            ),
            Err(PosixError::Fault)
        );
        assert_eq!(kernel_dst, [0xbb; 0x20]);
        assert_eq!(user_memory, user_before);
    }

    #[test_case]
    fn copy_user_rejects_backing_storage_gaps_as_efault() {
        let mappings = [user_mapping(
            USER_NULL_GUARD_END,
            0x100,
            UserMappingPermissions::USER_DATA,
        )];
        let user_memory = [0xaa; 4];
        let mut kernel_dst = [0xbb; 8];

        assert_eq!(
            copy_from_user(
                &mappings,
                USER_NULL_GUARD_END,
                &user_memory,
                USER_NULL_GUARD_END + 2,
                4,
                &mut kernel_dst
            ),
            Err(PosixError::Fault)
        );
        assert_eq!(kernel_dst, [0xbb; 8]);
    }

    #[test_case]
    fn copy_user_rejects_short_kernel_buffers_as_einval_without_side_effects() {
        let mappings = [user_mapping(
            USER_NULL_GUARD_END,
            0x100,
            UserMappingPermissions::USER_DATA,
        )];
        let user_memory = [0xaa; 4];
        let mut user_dst = [0xbb; 4];
        let mut kernel_dst = [0xcc; 2];

        assert_eq!(
            copy_from_user(
                &mappings,
                USER_NULL_GUARD_END,
                &user_memory,
                USER_NULL_GUARD_END,
                3,
                &mut kernel_dst
            ),
            Err(PosixError::InvalidArgument)
        );
        assert_eq!(
            copy_to_user(
                &mappings,
                USER_NULL_GUARD_END,
                &mut user_dst,
                USER_NULL_GUARD_END,
                3,
                &[1, 2]
            ),
            Err(PosixError::InvalidArgument)
        );
        assert_eq!(kernel_dst, [0xcc; 2]);
        assert_eq!(user_dst, [0xbb; 4]);
    }

    #[test_case]
    fn empty_path_maps_to_enoent() {
        assert_eq!(normalize(b""), Err(PosixError::NoEntry));
    }

    #[test_case]
    fn root_path_has_no_components() {
        let path = normalize(b"////").expect("root path");

        assert_eq!(path.start(), PathStart::Root);
        assert_components(&path, &[]);
        assert!(!path.requires_directory());
    }

    #[test_case]
    fn default_limits_cover_normal_contract_paths() {
        let path =
            normalize_path::<64>(b"/usr/bin/talos", DEFAULT_PATH_LIMITS).expect("default limits");

        assert_eq!(path.start(), PathStart::Root);
        assert_eq!(path.component_count(), 3);
        assert!(!path.requires_directory());
    }

    #[test_case]
    fn repeated_separators_and_dot_components_are_removed() {
        let path = normalize(b"/alpha//./beta").expect("normalized path");

        assert_eq!(path.start(), PathStart::Root);
        assert_components(&path, &[&b"alpha"[..], &b"beta"[..]]);
        assert!(!path.requires_directory());
    }

    #[test_case]
    fn absolute_dot_dot_clamps_above_root() {
        let path = normalize(b"/../alpha/../../").expect("absolute parent clamp");

        assert_eq!(path.start(), PathStart::Root);
        assert_components(&path, &[]);
        assert!(!path.requires_directory());
    }

    #[test_case]
    fn relative_leading_dot_dot_components_are_retained() {
        let path = normalize(b"../../alpha").expect("relative parents");

        assert_eq!(path.start(), PathStart::CurrentWorkingDirectory);
        assert_components(&path, &[&b".."[..], &b".."[..], &b"alpha"[..]]);
        assert!(!path.requires_directory());
    }

    #[test_case]
    fn relative_dot_dot_cancels_previous_normal_component() {
        let path = normalize(b"alpha/beta/../gamma").expect("relative cancellation");

        assert_eq!(path.start(), PathStart::CurrentWorkingDirectory);
        assert_components(&path, &[&b"alpha"[..], &b"gamma"[..]]);
        assert!(!path.requires_directory());
    }

    #[test_case]
    fn dot_path_is_relative_current_directory() {
        let path = normalize(b".").expect("dot path");

        assert_eq!(path.start(), PathStart::CurrentWorkingDirectory);
        assert_components(&path, &[]);
        assert!(!path.requires_directory());
    }

    #[test_case]
    fn trailing_slash_requires_directory_for_non_root_path() {
        let path = normalize(b"alpha/beta/").expect("trailing slash");

        assert_eq!(path.start(), PathStart::CurrentWorkingDirectory);
        assert_components(&path, &[&b"alpha"[..], &b"beta"[..]]);
        assert!(path.requires_directory());
    }

    #[test_case]
    fn relative_current_directory_with_trailing_slash_requires_directory() {
        let path = normalize(b"./").expect("dot slash");

        assert_eq!(path.start(), PathStart::CurrentWorkingDirectory);
        assert_components(&path, &[]);
        assert!(path.requires_directory());
    }

    #[test_case]
    fn embedded_nul_maps_to_einval() {
        assert_eq!(normalize(b"alpha\0beta"), Err(PosixError::InvalidArgument));
    }

    #[test_case]
    fn path_longer_than_limit_maps_to_enametoolong() {
        assert_eq!(
            normalize_path::<4>(b"alpha", PathLimits::new(4, 8, 4)),
            Err(PosixError::NameTooLong)
        );
    }

    #[test_case]
    fn component_longer_than_limit_maps_to_enametoolong() {
        assert_eq!(
            normalize_path::<4>(b"toolong", PathLimits::new(64, 3, 4)),
            Err(PosixError::NameTooLong)
        );
    }

    #[test_case]
    fn component_count_limit_maps_to_enametoolong() {
        assert_eq!(
            normalize_path::<4>(b"a/b/c", PathLimits::new(64, 8, 2)),
            Err(PosixError::NameTooLong)
        );
    }

    #[test_case]
    fn storage_capacity_limit_maps_to_enametoolong() {
        assert_eq!(
            normalize_path::<2>(b"a/b/c", PathLimits::new(64, 8, 4)),
            Err(PosixError::NameTooLong)
        );
    }
}
