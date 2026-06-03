use crate::{
    initial_process_launch::{self, InitialProcessLaunchRequest},
    initial_user_stack::{self, InitialUserStackLeaseSource, InitialUserStackRequest},
    initramfs, posix,
    process_address_space::{self, ProcessAddressSpaceId, ProcessAddressSpaceLeaseSource},
    process_install,
    process_page_table_materialization::{
        self, ProcessMaterializationRequest, ProcessPageTableMaterializationLeaseSource,
    },
    program_loader,
    runtime_console::{self, ConsoleBackend, ConsoleInputBackend, DEFAULT_RUNTIME_CONSOLE},
    scheduler::ProcessOwnerId,
    syscall,
    tty::{self, CANONICAL_LINE_CAPACITY, PollingTtyRxOutcome, PollingTtyRxResult},
};

pub const LOCAL_COMMAND_LOOP_VERSION: &str = "phase10.2-kernel-builtins-v1";
pub const LOCAL_COMMAND_BUILTIN_BOUNDARY: &str = concat!(
    "kernel-backed-regression-control+vfs-syscall-cat+vfs-userspace-exec-boundary",
    "+lifecycle-laststatus+waitpid-lifecycle-observation+standard-descriptor-inheritance"
);
pub const LOCAL_COMMAND_LOOP_PROMPT: &str = "talos> ";
pub const DEFAULT_LOCAL_COMMAND_COUNT: usize = 8;
const LOCAL_COMMAND_FILE_USER_BASE: u64 = 0x0000_0000_0011_0000;
const LOCAL_COMMAND_FILE_READ_OFFSET: usize = 0x40;
const LOCAL_COMMAND_FILE_USER_MEMORY_LEN: usize = 128;
const LOCAL_COMMAND_EXEC_READ_OFFSET: usize = 0x80;
const LOCAL_COMMAND_EXEC_USER_MEMORY_LEN: usize = 1024;
const LOCAL_COMMAND_EXEC_ADDRESS_SPACE_ID: u64 = 0x0010_0001;
const LOCAL_COMMAND_EXEC_PROCESS_ID: u64 = 0x0010_0001;
const LOCAL_COMMAND_EXEC_TEMP_DESCRIPTOR: usize = posix::STDERR_FD + 1;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocalCommandDirectory {
    Root,
    Etc,
    Bin,
}

impl LocalCommandDirectory {
    pub const fn path(self) -> &'static str {
        match self {
            Self::Root => "/",
            Self::Etc => "/etc",
            Self::Bin => "/bin",
        }
    }

    const fn initramfs_path(self) -> &'static [u8] {
        match self {
            Self::Root => b"/",
            Self::Etc => b"/etc",
            Self::Bin => b"/bin",
        }
    }
}

const LOCAL_COMMAND_ROOT_LISTING: [(&[u8], &str); 4] = [
    (b"/bin", "bin"),
    (b"/dir", "dir"),
    (b"/empty", "empty"),
    (b"/etc", "etc"),
];
const LOCAL_COMMAND_ETC_LISTING: [(&[u8], &str); 1] =
    [(initramfs::PHASE8_BANNER_PATH, "banner.txt")];
const LOCAL_COMMAND_BIN_LISTING: [(&[u8], &str); 3] = [
    (initramfs::PHASE8_INIT_PATH, "init"),
    (initramfs::PHASE10_ZERO_PATH, "zero"),
    (initramfs::PHASE10_STATUS42_PATH, "status42"),
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocalCommandStatus {
    Handled,
    LineCanceled,
    Empty,
    UnknownCommand,
    UnexpectedArgument,
    ParseError,
    InputError(PollingTtyRxOutcome),
}

impl LocalCommandStatus {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Handled => "handled",
            Self::LineCanceled => "line-canceled",
            Self::Empty => "empty-command",
            Self::UnknownCommand => "unknown-command",
            Self::UnexpectedArgument => "unexpected-argument",
            Self::ParseError => "parse-error",
            Self::InputError(_) => "input-error",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocalCommandWriteError {
    BackendWriteFailed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocalCommandFileReadError {
    NotSupported,
    SyscallFailed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocalCommandExecError {
    InvalidPath,
    NotFound,
    NotExecutable,
    NotSupported,
    SyscallFailed,
    LaunchPipelineFailed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalCommandExecSummary {
    source_path: &'static [u8],
    source_len: usize,
    source_digest: u64,
    entry: u64,
    segments: usize,
    address_space_id: u64,
    materialization_id: u64,
    initial_sp: u64,
    launch_boundary: &'static str,
    stack_boundary: &'static str,
    startup_state: &'static str,
    startup_argc: usize,
    startup_argv0_path: &'static [u8],
    startup_argv0_user_address: u64,
    startup_argv_null: bool,
    startup_envp_state: &'static str,
    startup_envp_entry_count: usize,
    startup_envp0_user_address: u64,
    startup_envp_null: bool,
    copied_startup_bytes: u64,
    completion_status: u64,
    completion_marker: u64,
    completion_boundary: &'static str,
    descriptor_inheritance: LocalCommandExecDescriptorInheritanceRecord,
    lifecycle: LocalCommandProcessLifecycleRecord,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalCommandExecDescriptorInheritanceRecord {
    owner_id: u64,
    stdin_kind: &'static str,
    stdout_kind: &'static str,
    stderr_kind: &'static str,
    inherited_count: usize,
    loader_temporary_descriptor: usize,
    loader_temporary_descriptor_open: bool,
    source: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalCommandProcessLifecycleRecord {
    process_id: u64,
    parent_owner_id: u64,
    source_path: &'static [u8],
    state: LocalCommandProcessState,
    status: u64,
    observed_status: u64,
    reaped: bool,
}

impl LocalCommandProcessLifecycleRecord {
    const fn exited(
        process_id: u64,
        parent_owner_id: u64,
        source_path: &'static [u8],
        status: u64,
    ) -> Self {
        Self {
            process_id,
            parent_owner_id,
            source_path,
            state: LocalCommandProcessState::Exited,
            status,
            observed_status: status,
            reaped: true,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LocalCommandProcessState {
    Exited,
}

impl LocalCommandProcessState {
    const fn name(self) -> &'static str {
        match self {
            Self::Exited => "exited",
        }
    }
}

pub trait LocalCommandSink {
    fn write_command_str(&mut self, text: &str) -> Result<(), LocalCommandWriteError>;

    fn stdio_descriptor_kind(&self, _descriptor: usize) -> Option<&'static str> {
        None
    }

    fn runtime_console_name(&self) -> &'static str {
        DEFAULT_RUNTIME_CONSOLE.name
    }

    fn descriptor_backed_output(&self) -> bool {
        false
    }

    fn descriptor_backed_input(&self) -> bool {
        false
    }

    fn current_directory(&self) -> LocalCommandDirectory {
        LocalCommandDirectory::Root
    }

    fn set_current_directory(&mut self, directory: LocalCommandDirectory) -> bool {
        directory == LocalCommandDirectory::Root
    }

    fn read_initramfs_file_via_syscall(
        &mut self,
        _path: &[u8],
        _output: &mut [u8],
    ) -> Result<usize, LocalCommandFileReadError> {
        Err(LocalCommandFileReadError::NotSupported)
    }

    fn exec_vfs_program(
        &mut self,
        _path: &[u8],
    ) -> Result<LocalCommandExecSummary, LocalCommandExecError> {
        Err(LocalCommandExecError::NotSupported)
    }

    fn last_process_lifecycle_record(&self) -> Option<LocalCommandProcessLifecycleRecord> {
        None
    }

    fn wait_process_lifecycle_record(&mut self) -> Option<LocalCommandProcessLifecycleRecord> {
        None
    }
}

impl<B> LocalCommandSink for runtime_console::RuntimeConsole<B>
where
    B: ConsoleBackend,
{
    fn write_command_str(&mut self, text: &str) -> Result<(), LocalCommandWriteError> {
        self.write_kernel_args(format_args!("{}", text))
            .map(|_| ())
            .map_err(|_| LocalCommandWriteError::BackendWriteFailed)
    }
}

pub struct DescriptorBackedLocalCommandIo<
    I,
    O,
    const OWNER_CAPACITY: usize,
    const DESCRIPTOR_CAPACITY: usize,
> where
    I: ConsoleInputBackend,
    O: ConsoleBackend,
{
    descriptor_store: posix::ProcessDescriptorStore<OWNER_CAPACITY, DESCRIPTOR_CAPACITY>,
    current_owner: Option<ProcessOwnerId>,
    input_descriptor: usize,
    output_descriptor: usize,
    input_backend: I,
    output_backend: O,
    current_directory: LocalCommandDirectory,
    read_only_files: initramfs::ReadOnlyFileDescriptions<1>,
    last_process: Option<LocalCommandProcessLifecycleRecord>,
    waitable_process: Option<LocalCommandProcessLifecycleRecord>,
}

impl<I, O> DescriptorBackedLocalCommandIo<I, O, 1, 4>
where
    I: ConsoleInputBackend,
    O: ConsoleBackend,
{
    pub fn new_inherited_stdio(
        input_backend: I,
        output_backend: O,
    ) -> Result<Self, posix::PosixError> {
        let current_owner = ProcessOwnerId::new(1).expect("local command owner id is nonzero");
        let mut descriptor_store = posix::ProcessDescriptorStore::<1, 4>::new_empty();
        descriptor_store.create_owner_with_inherited_stdio(current_owner)?;
        Ok(Self {
            descriptor_store,
            current_owner: Some(current_owner),
            input_descriptor: posix::STDIN_FD,
            output_descriptor: posix::STDOUT_FD,
            input_backend,
            output_backend,
            current_directory: LocalCommandDirectory::Root,
            read_only_files: initramfs::ReadOnlyFileDescriptions::new_empty(),
            last_process: None,
            waitable_process: None,
        })
    }
}

impl<I, O, const OWNER_CAPACITY: usize, const DESCRIPTOR_CAPACITY: usize> ConsoleInputBackend
    for DescriptorBackedLocalCommandIo<I, O, OWNER_CAPACITY, DESCRIPTOR_CAPACITY>
where
    I: ConsoleInputBackend,
    O: ConsoleBackend,
{
    fn poll_read_byte(&mut self) -> Option<u8> {
        let descriptor_table = self
            .descriptor_store
            .current_descriptor_table(self.current_owner)
            .ok()?;
        let entry = descriptor_table.get(self.input_descriptor).ok()?;
        if entry.require_readable().is_err()
            || entry.object().kind() != posix::DescriptorObjectKind::StdioInput
        {
            return None;
        }
        self.input_backend.poll_read_byte()
    }
}

impl<I, O, const OWNER_CAPACITY: usize, const DESCRIPTOR_CAPACITY: usize> LocalCommandSink
    for DescriptorBackedLocalCommandIo<I, O, OWNER_CAPACITY, DESCRIPTOR_CAPACITY>
where
    I: ConsoleInputBackend,
    O: ConsoleBackend,
{
    fn write_command_str(&mut self, text: &str) -> Result<(), LocalCommandWriteError> {
        let descriptor_table = self
            .descriptor_store
            .current_descriptor_table(self.current_owner)
            .map_err(|_| LocalCommandWriteError::BackendWriteFailed)?;
        posix::write_kernel_bytes_to_descriptor_console(
            descriptor_table,
            self.output_descriptor,
            text.as_bytes(),
            &mut self.output_backend,
        )
        .map(|_| ())
        .map_err(|_| LocalCommandWriteError::BackendWriteFailed)
    }

    fn stdio_descriptor_kind(&self, descriptor: usize) -> Option<&'static str> {
        self.descriptor_store
            .current_descriptor_table(self.current_owner)
            .ok()
            .and_then(|table| table.get(descriptor).ok())
            .map(|entry| entry.object().kind().name())
    }

    fn descriptor_backed_output(&self) -> bool {
        true
    }

    fn descriptor_backed_input(&self) -> bool {
        self.descriptor_store
            .current_descriptor_table(self.current_owner)
            .ok()
            .and_then(|table| table.get(self.input_descriptor).ok())
            .is_some_and(|entry| {
                entry.require_readable().is_ok()
                    && entry.object().kind() == posix::DescriptorObjectKind::StdioInput
            })
    }

    fn current_directory(&self) -> LocalCommandDirectory {
        self.current_directory
    }

    fn set_current_directory(&mut self, directory: LocalCommandDirectory) -> bool {
        self.current_directory = directory;
        true
    }

    fn read_initramfs_file_via_syscall(
        &mut self,
        path: &[u8],
        output: &mut [u8],
    ) -> Result<usize, LocalCommandFileReadError> {
        self.read_initramfs_file_via_syscall_with_memory::<LOCAL_COMMAND_FILE_USER_MEMORY_LEN>(
            path,
            output,
            LOCAL_COMMAND_FILE_READ_OFFSET,
        )
    }

    fn exec_vfs_program(
        &mut self,
        path: &[u8],
    ) -> Result<LocalCommandExecSummary, LocalCommandExecError> {
        if !is_absolute_exec_path(path) {
            return Err(LocalCommandExecError::InvalidPath);
        }

        let fs = initramfs::phase8_readonly_initramfs_fixture();
        let node = fs
            .lookup_default(path)
            .map_err(|_| LocalCommandExecError::NotFound)?;
        if node.metadata().kind() != initramfs::VfsNodeKind::RegularFile {
            return Err(LocalCommandExecError::NotExecutable);
        }
        let source_path = match path {
            initramfs::PHASE8_INIT_PATH => initramfs::PHASE8_INIT_PATH,
            initramfs::PHASE10_ZERO_PATH => initramfs::PHASE10_ZERO_PATH,
            initramfs::PHASE10_STATUS42_PATH => initramfs::PHASE10_STATUS42_PATH,
            initramfs::PHASE8_BANNER_PATH => initramfs::PHASE8_BANNER_PATH,
            initramfs::PHASE8_EMPTY_PATH => initramfs::PHASE8_EMPTY_PATH,
            initramfs::PHASE8_NESTED_PATH => initramfs::PHASE8_NESTED_PATH,
            _ => return Err(LocalCommandExecError::NotExecutable),
        };

        let owner = self
            .current_owner
            .ok_or(LocalCommandExecError::LaunchPipelineFailed)?;
        let mut program_bytes = [0u8; initramfs::PHASE8_INIT_ELF_LEN];
        let bytes_read = self
            .read_initramfs_file_via_syscall_with_memory::<LOCAL_COMMAND_EXEC_USER_MEMORY_LEN>(
                path,
                &mut program_bytes,
                LOCAL_COMMAND_EXEC_READ_OFFSET,
            )
            .map_err(|_| LocalCommandExecError::SyscallFailed)?;
        let descriptor_inheritance = self
            .standard_descriptor_inheritance_record(owner)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        let image = program_loader::plan_elf64_aarch64_image(
            source_path,
            program_loader::PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY,
            &program_bytes[..bytes_read],
        )
        .map_err(|_| LocalCommandExecError::NotExecutable)?;
        let completion_status = decode_phase8_init_completion_status(&program_bytes[..bytes_read])
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        let install_plan = process_install::plan_process_image_install(image)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        let mut address_source = ProcessAddressSpaceLeaseSource::for_plan(install_plan);
        let address_space = process_address_space::install_process_address_space(
            install_plan,
            ProcessAddressSpaceId::new(LOCAL_COMMAND_EXEC_ADDRESS_SPACE_ID)
                .ok_or(LocalCommandExecError::LaunchPipelineFailed)?,
            Some(owner),
            &mut address_source,
        )
        .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        let mut materialization_source =
            ProcessPageTableMaterializationLeaseSource::for_address_space(address_space);
        let materialization = process_page_table_materialization::materialize_process_page_tables(
            image,
            install_plan,
            address_space,
            ProcessMaterializationRequest::DescriptorImageOnly,
            &mut materialization_source,
        )
        .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        let launch_plan = initial_process_launch::prepare_initial_process_launch(
            image,
            install_plan,
            address_space,
            materialization,
            InitialProcessLaunchRequest::PreparePlanOnly,
        )
        .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        let mut stack_source = InitialUserStackLeaseSource::for_initial_stack();
        let stack_plan = initial_user_stack::plan_initial_user_stack(
            image,
            install_plan,
            address_space,
            materialization,
            launch_plan,
            InitialUserStackRequest::PlanOnly,
            &mut stack_source,
        )
        .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        let lifecycle = LocalCommandProcessLifecycleRecord::exited(
            LOCAL_COMMAND_EXEC_PROCESS_ID,
            owner.raw(),
            source_path,
            completion_status,
        );
        self.last_process = Some(lifecycle);
        self.waitable_process = Some(lifecycle);

        Ok(LocalCommandExecSummary {
            source_path: image.source_path(),
            source_len: image.source_len(),
            source_digest: image.source_digest(),
            entry: image.entry(),
            segments: image.segment_count(),
            address_space_id: address_space.id().raw(),
            materialization_id: materialization.id(),
            initial_sp: stack_plan.layout().initial_sp(),
            launch_boundary: launch_plan.boundary_identity(),
            stack_boundary: stack_plan.boundary_identity(),
            startup_state: stack_plan.startup_payload().state(),
            startup_argc: stack_plan.startup_payload().argc(),
            startup_argv0_path: stack_plan.startup_payload().argv0_path(),
            startup_argv0_user_address: stack_plan.startup_payload().argv0_user_address(),
            startup_argv_null: stack_plan.startup_payload().argv_null(),
            startup_envp_state: stack_plan.startup_payload().envp_state(),
            startup_envp_entry_count: stack_plan.startup_payload().envp_entry_count(),
            startup_envp0_user_address: stack_plan.startup_payload().envp0_user_address(),
            startup_envp_null: stack_plan.startup_payload().envp_null(),
            copied_startup_bytes: stack_plan.startup_payload().copied_startup_bytes(),
            completion_status,
            completion_marker: initramfs::PHASE8_INIT_SVC_MARKER,
            completion_boundary: "lower-aarch64-svc-status-equivalent",
            descriptor_inheritance,
            lifecycle,
        })
    }

    fn last_process_lifecycle_record(&self) -> Option<LocalCommandProcessLifecycleRecord> {
        self.last_process
    }

    fn wait_process_lifecycle_record(&mut self) -> Option<LocalCommandProcessLifecycleRecord> {
        self.waitable_process.take()
    }
}

impl<I, O, const OWNER_CAPACITY: usize, const DESCRIPTOR_CAPACITY: usize>
    DescriptorBackedLocalCommandIo<I, O, OWNER_CAPACITY, DESCRIPTOR_CAPACITY>
where
    I: ConsoleInputBackend,
    O: ConsoleBackend,
{
    fn close_regular_file_descriptor(
        &mut self,
        descriptor: usize,
    ) -> Result<(), LocalCommandFileReadError> {
        let entry = self
            .descriptor_store
            .close_current_descriptor(self.current_owner, descriptor)
            .map_err(|_| LocalCommandFileReadError::SyscallFailed)?;
        if entry.object().kind() == posix::DescriptorObjectKind::RegularFile {
            self.read_only_files
                .remove(entry.object().reference())
                .map_err(|_| LocalCommandFileReadError::SyscallFailed)?;
        }
        Ok(())
    }

    fn read_initramfs_file_via_syscall_with_memory<const USER_MEMORY_LEN: usize>(
        &mut self,
        path: &[u8],
        output: &mut [u8],
        read_offset: usize,
    ) -> Result<usize, LocalCommandFileReadError> {
        if path.len() > read_offset || read_offset + output.len() > USER_MEMORY_LEN {
            return Err(LocalCommandFileReadError::SyscallFailed);
        }

        let mut user_memory = [0u8; USER_MEMORY_LEN];
        let mut scratch = [0u8; USER_MEMORY_LEN];
        user_memory[..path.len()].copy_from_slice(path);
        let mappings = [posix::UserMapping::new(
            LOCAL_COMMAND_FILE_USER_BASE,
            user_memory.len(),
            posix::UserMappingPermissions::USER_DATA,
        )
        .map_err(|_| LocalCommandFileReadError::SyscallFailed)?];
        let fs = initramfs::phase8_readonly_initramfs_fixture();

        let open = syscall::dispatch_process_descriptor_with_initramfs(
            syscall::TALOS_OPEN_SYSCALL,
            syscall::SyscallArguments::new([
                LOCAL_COMMAND_FILE_USER_BASE,
                path.len() as u64,
                0,
                0,
                0,
                0,
            ]),
            self.current_owner,
            &mut self.descriptor_store,
            &mappings,
            LOCAL_COMMAND_FILE_USER_BASE,
            &mut user_memory,
            &mut scratch,
            &mut self.output_backend,
            fs,
            &mut self.read_only_files,
            None,
        );
        let descriptor = syscall_success_usize(open.return_value().x0())?;

        let read = syscall::dispatch_process_descriptor_with_initramfs(
            syscall::TALOS_READ_SYSCALL,
            syscall::SyscallArguments::new([
                descriptor as u64,
                LOCAL_COMMAND_FILE_USER_BASE + read_offset as u64,
                output.len() as u64,
                0,
                0,
                0,
            ]),
            self.current_owner,
            &mut self.descriptor_store,
            &mappings,
            LOCAL_COMMAND_FILE_USER_BASE,
            &mut user_memory,
            &mut scratch,
            &mut self.output_backend,
            fs,
            &mut self.read_only_files,
            None,
        );
        let bytes_read = syscall_success_usize(read.return_value().x0());
        let cleanup = self.close_regular_file_descriptor(descriptor);

        match (bytes_read, cleanup) {
            (Ok(bytes_read), Ok(())) => {
                output[..bytes_read]
                    .copy_from_slice(&user_memory[read_offset..read_offset + bytes_read]);
                Ok(bytes_read)
            }
            _ => Err(LocalCommandFileReadError::SyscallFailed),
        }
    }

    fn standard_descriptor_inheritance_record(
        &self,
        owner: ProcessOwnerId,
    ) -> Result<LocalCommandExecDescriptorInheritanceRecord, LocalCommandExecError> {
        let table = self
            .descriptor_store
            .descriptor_table(owner)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        let stdin = table
            .get(posix::STDIN_FD)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        let stdout = table
            .get(posix::STDOUT_FD)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        let stderr = table
            .get(posix::STDERR_FD)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;

        if stdin.require_readable().is_err()
            || stdout.require_writable().is_err()
            || stderr.require_writable().is_err()
            || stdin.object().kind() != posix::DescriptorObjectKind::StdioInput
            || stdout.object().kind() != posix::DescriptorObjectKind::StdioOutput
            || stderr.object().kind() != posix::DescriptorObjectKind::StdioOutput
        {
            return Err(LocalCommandExecError::LaunchPipelineFailed);
        }

        Ok(LocalCommandExecDescriptorInheritanceRecord {
            owner_id: owner.raw(),
            stdin_kind: stdin.object().kind().name(),
            stdout_kind: stdout.object().kind().name(),
            stderr_kind: stderr.object().kind().name(),
            inherited_count: 3,
            loader_temporary_descriptor: LOCAL_COMMAND_EXEC_TEMP_DESCRIPTOR,
            loader_temporary_descriptor_open: table.get(LOCAL_COMMAND_EXEC_TEMP_DESCRIPTOR).is_ok(),
            source: "shell-process-descriptor-table",
        })
    }
}

fn syscall_success_usize(value: u64) -> Result<usize, LocalCommandFileReadError> {
    if value > isize::MAX as u64 {
        return Err(LocalCommandFileReadError::SyscallFailed);
    }
    usize::try_from(value).map_err(|_| LocalCommandFileReadError::SyscallFailed)
}

fn decode_phase8_init_completion_status(
    program_bytes: &[u8],
) -> Result<u64, LocalCommandExecError> {
    let text = program_bytes
        .get(initramfs::PHASE8_INIT_TEXT_OFFSET..initramfs::PHASE8_INIT_TEXT_OFFSET + 8)
        .ok_or(LocalCommandExecError::LaunchPipelineFailed)?;
    let movz_x0 = u32::from_le_bytes([text[0], text[1], text[2], text[3]]);
    let svc = u32::from_le_bytes([text[4], text[5], text[6], text[7]]);
    let status = (movz_x0 >> 5) & 0xffff;
    let marker = (svc >> 5) & 0xffff;
    if movz_x0 & 0xffe0_001f != 0xd280_0000
        || svc & 0xffe0_001f != 0xd400_0001
        || marker as u64 != initramfs::PHASE8_INIT_SVC_MARKER
    {
        return Err(LocalCommandExecError::LaunchPipelineFailed);
    }
    Ok(status as u64)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocalCommandCycleError {
    PromptWriteFailed,
    ResponseWriteFailed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalCommandCycleResult {
    line: [u8; CANONICAL_LINE_CAPACITY],
    line_len: usize,
    status: LocalCommandStatus,
    response_lines: usize,
    raw_bytes: usize,
    backspaces: usize,
    deletes: usize,
    truncated: bool,
    controls: usize,
    controls_truncated: bool,
}

impl LocalCommandCycleResult {
    pub fn line(&self) -> &[u8] {
        &self.line[..self.line_len]
    }

    pub const fn status(&self) -> LocalCommandStatus {
        self.status
    }

    pub const fn status_name(&self) -> &'static str {
        self.status.name()
    }

    pub const fn response_lines(&self) -> usize {
        self.response_lines
    }

    pub const fn raw_bytes(&self) -> usize {
        self.raw_bytes
    }

    pub const fn backspaces(&self) -> usize {
        self.backspaces
    }

    pub const fn deletes(&self) -> usize {
        self.deletes
    }

    pub const fn truncated(&self) -> bool {
        self.truncated
    }

    pub const fn controls(&self) -> usize {
        self.controls
    }

    pub const fn controls_truncated(&self) -> bool {
        self.controls_truncated
    }
}

pub fn write_local_command_prompt(
    sink: &mut impl LocalCommandSink,
) -> Result<(), LocalCommandWriteError> {
    sink.write_command_str(LOCAL_COMMAND_LOOP_PROMPT)
}

pub fn run_one_serial_command<I, S>(
    input: &mut I,
    sink: &mut S,
) -> Result<LocalCommandCycleResult, LocalCommandCycleError>
where
    I: ConsoleInputBackend,
    S: LocalCommandSink,
{
    run_one_serial_command_with_limit(input, sink, tty::POLLING_RX_WAIT_LIMIT)
}

pub fn run_one_serial_command_with_limit<I, S>(
    input: &mut I,
    sink: &mut S,
    wait_limit: usize,
) -> Result<LocalCommandCycleResult, LocalCommandCycleError>
where
    I: ConsoleInputBackend,
    S: LocalCommandSink,
{
    write_local_command_prompt(sink).map_err(|_| LocalCommandCycleError::PromptWriteFailed)?;
    let input_result = tty::run_polling_rx_diagnostic_with_limit(input, wait_limit);
    dispatch_completed_line(input_result, sink)
}

pub fn run_one_descriptor_backed_serial_command_with_limit<T>(
    io: &mut T,
    wait_limit: usize,
) -> Result<LocalCommandCycleResult, LocalCommandCycleError>
where
    T: ConsoleInputBackend + LocalCommandSink,
{
    write_local_command_prompt(io).map_err(|_| LocalCommandCycleError::PromptWriteFailed)?;
    let input_result = tty::run_polling_rx_diagnostic_with_limit(&mut *io, wait_limit);
    dispatch_completed_line(input_result, io)
}

pub fn run_one_descriptor_backed_serial_command<T>(
    io: &mut T,
) -> Result<LocalCommandCycleResult, LocalCommandCycleError>
where
    T: ConsoleInputBackend + LocalCommandSink,
{
    run_one_descriptor_backed_serial_command_with_limit(io, tty::POLLING_RX_WAIT_LIMIT)
}

fn dispatch_completed_line(
    input_result: PollingTtyRxResult,
    sink: &mut impl LocalCommandSink,
) -> Result<LocalCommandCycleResult, LocalCommandCycleError> {
    let status;
    let response_lines;

    if input_result.outcome() == PollingTtyRxOutcome::LineCanceled
        && input_result.controls() == &[Some(tty::TtyControlEvent::Interrupt)]
    {
        status = LocalCommandStatus::LineCanceled;
        let mut responses = 0usize;
        write_line(sink, &mut responses, "talos: line-canceled")?;
        response_lines = responses;
    } else if !input_result.passed()
        || input_result.truncated()
        || input_result.controls_truncated()
        || has_unsupported_controls(input_result.controls())
    {
        status = LocalCommandStatus::InputError(input_result.outcome());
        let mut responses = 0usize;
        write_parts_line(
            sink,
            &mut responses,
            &["talos: input-error ", input_result.outcome_name()],
        )?;
        response_lines = responses;
    } else {
        let mut responses = 0usize;
        if has_line_kill(input_result.controls()) {
            write_line(sink, &mut responses, "talos: line-killed")?;
        }
        status = dispatch_local_command(input_result.line(), sink, &mut responses)?;
        response_lines = responses;
    }

    Ok(LocalCommandCycleResult {
        line: copy_line(input_result.line()),
        line_len: input_result.line().len(),
        status,
        response_lines,
        raw_bytes: input_result.raw_bytes(),
        backspaces: input_result.backspaces(),
        deletes: input_result.deletes(),
        truncated: input_result.truncated(),
        controls: input_result.controls().len(),
        controls_truncated: input_result.controls_truncated(),
    })
}

fn has_line_kill(controls: &[Option<tty::TtyControlEvent>]) -> bool {
    controls
        .iter()
        .any(|event| *event == Some(tty::TtyControlEvent::ClearLine))
}

fn has_unsupported_controls(controls: &[Option<tty::TtyControlEvent>]) -> bool {
    controls
        .iter()
        .any(|event| *event != Some(tty::TtyControlEvent::ClearLine))
}

fn dispatch_local_command(
    line: &[u8],
    sink: &mut impl LocalCommandSink,
    responses: &mut usize,
) -> Result<LocalCommandStatus, LocalCommandCycleError> {
    let command = match parse_local_command(line) {
        Ok(command) => command,
        Err(ParseLocalCommandError::Empty) => {
            write_line(sink, responses, "talos: empty-command")?;
            return Ok(LocalCommandStatus::Empty);
        }
        Err(ParseLocalCommandError::Invalid) => {
            write_line(sink, responses, "talos: parse-error")?;
            return Ok(LocalCommandStatus::ParseError);
        }
    };

    match command.name {
        "help" => {
            if command.arguments.is_some() {
                write_line(sink, responses, "talos: unexpected-argument")?;
                return Ok(LocalCommandStatus::UnexpectedArgument);
            }
            write_line(sink, responses, "talos: ok help")?;
            write_line(
                sink,
                responses,
                "talos: commands help status stdio pwd echo ls cat cd exec laststatus waitpid",
            )?;
            write_line(
                sink,
                responses,
                "talos: echo forms echo hello; echo local serial works",
            )?;
            write_line(
                sink,
                responses,
                "talos: editing backspace delete ctrl-c ctrl-u",
            )?;
            Ok(LocalCommandStatus::Handled)
        }
        "status" => {
            if command.arguments.is_some() {
                write_line(sink, responses, "talos: unexpected-argument")?;
                return Ok(LocalCommandStatus::UnexpectedArgument);
            }
            write_line(sink, responses, "talos: ok status")?;
            write_parts_line(
                sink,
                responses,
                &["talos: version ", LOCAL_COMMAND_LOOP_VERSION],
            )?;
            write_parts_line(
                sink,
                responses,
                &["talos: runtime-console ", DEFAULT_RUNTIME_CONSOLE.name],
            )?;
            write_parts_line(
                sink,
                responses,
                &["talos: builtins ", LOCAL_COMMAND_BUILTIN_BOUNDARY],
            )?;
            write_line(
                sink,
                responses,
                "talos: commands help status stdio pwd echo ls cat cd exec laststatus waitpid",
            )?;
            Ok(LocalCommandStatus::Handled)
        }
        "stdio" => {
            if command.arguments.is_some() {
                write_line(sink, responses, "talos: unexpected-argument")?;
                return Ok(LocalCommandStatus::UnexpectedArgument);
            }
            write_line(sink, responses, "talos: ok stdio")?;
            write_stdio_descriptor_line(sink, responses, posix::STDIN_FD)?;
            write_stdio_descriptor_line(sink, responses, posix::STDOUT_FD)?;
            write_stdio_descriptor_line(sink, responses, posix::STDERR_FD)?;
            write_parts_line(
                sink,
                responses,
                &["talos: runtime-console ", sink.runtime_console_name()],
            )?;
            let marker = if sink.descriptor_backed_input() {
                "true"
            } else {
                "false"
            };
            write_parts_line(
                sink,
                responses,
                &["talos: descriptor-backed-input=", marker],
            )?;
            let marker = if sink.descriptor_backed_output() {
                "true"
            } else {
                "false"
            };
            write_parts_line(
                sink,
                responses,
                &["talos: descriptor-backed-output=", marker],
            )?;
            Ok(LocalCommandStatus::Handled)
        }
        "echo" => {
            write_line(sink, responses, command.arguments.unwrap_or(""))?;
            Ok(LocalCommandStatus::Handled)
        }
        "pwd" => {
            if command.arguments.is_some() {
                write_line(sink, responses, "talos: unexpected-argument")?;
                return Ok(LocalCommandStatus::UnexpectedArgument);
            }
            write_line(sink, responses, sink.current_directory().path())?;
            Ok(LocalCommandStatus::Handled)
        }
        "cd" => {
            let Some(arguments) = command.arguments else {
                write_line(sink, responses, "talos: unexpected-argument")?;
                return Ok(LocalCommandStatus::UnexpectedArgument);
            };
            let Some(directory) = parse_bounded_directory(arguments) else {
                write_line(sink, responses, "talos: not-directory")?;
                return Ok(LocalCommandStatus::UnexpectedArgument);
            };
            if !directory_exists(directory) || !sink.set_current_directory(directory) {
                write_line(sink, responses, "talos: not-directory")?;
                return Ok(LocalCommandStatus::UnexpectedArgument);
            }
            Ok(LocalCommandStatus::Handled)
        }
        "ls" => {
            match command.arguments {
                None => {
                    let directory = sink.current_directory();
                    write_directory_listing(sink, responses, directory)?;
                }
                Some("/") => write_root_listing(sink, responses)?,
                Some("/bin") => write_bin_listing(sink, responses)?,
                _ => {
                    write_line(sink, responses, "talos: unexpected-argument")?;
                    return Ok(LocalCommandStatus::UnexpectedArgument);
                }
            }
            Ok(LocalCommandStatus::Handled)
        }
        "cat" => {
            match command.arguments {
                Some("/etc/banner.txt") => write_banner_file(sink, responses)?,
                Some("banner.txt") => {
                    if sink.current_directory() == LocalCommandDirectory::Etc {
                        write_banner_file(sink, responses)?;
                    } else {
                        write_line(sink, responses, "talos: not-found")?;
                        return Ok(LocalCommandStatus::UnexpectedArgument);
                    }
                }
                _ => {
                    write_line(sink, responses, "talos: unexpected-argument")?;
                    return Ok(LocalCommandStatus::UnexpectedArgument);
                }
            }
            Ok(LocalCommandStatus::Handled)
        }
        "exec" => {
            let Some(arguments) = command.arguments else {
                write_line(sink, responses, "talos: unexpected-argument")?;
                return Ok(LocalCommandStatus::UnexpectedArgument);
            };
            match sink.exec_vfs_program(arguments.as_bytes()) {
                Ok(summary) => {
                    write_exec_summary(sink, responses, summary)?;
                    Ok(LocalCommandStatus::Handled)
                }
                Err(LocalCommandExecError::InvalidPath) => {
                    write_line(sink, responses, "talos: exec-invalid-path")?;
                    Ok(LocalCommandStatus::UnexpectedArgument)
                }
                Err(LocalCommandExecError::NotExecutable) => {
                    write_line(sink, responses, "talos: exec-not-executable")?;
                    Ok(LocalCommandStatus::UnexpectedArgument)
                }
                Err(LocalCommandExecError::NotFound) => {
                    write_line(sink, responses, "talos: exec-not-found")?;
                    Ok(LocalCommandStatus::UnexpectedArgument)
                }
                Err(_) => {
                    write_line(sink, responses, "talos: exec-error")?;
                    Ok(LocalCommandStatus::UnexpectedArgument)
                }
            }
        }
        "laststatus" => {
            if command.arguments.is_some() {
                write_line(sink, responses, "talos: unexpected-argument")?;
                return Ok(LocalCommandStatus::UnexpectedArgument);
            }
            match sink.last_process_lifecycle_record() {
                Some(record) => {
                    write_last_process_status_line(sink, responses, record)?;
                    Ok(LocalCommandStatus::Handled)
                }
                None => {
                    write_line(sink, responses, "talos: last-process none")?;
                    Ok(LocalCommandStatus::Handled)
                }
            }
        }
        "waitpid" => {
            if command.arguments.is_some() {
                write_line(sink, responses, "talos: unexpected-argument")?;
                return Ok(LocalCommandStatus::UnexpectedArgument);
            }
            match sink.wait_process_lifecycle_record() {
                Some(record) => {
                    write_waitpid_status_line(sink, responses, record)?;
                    Ok(LocalCommandStatus::Handled)
                }
                None => {
                    write_line(
                        sink,
                        responses,
                        "talos: waitpid no-child source=lifecycle-record",
                    )?;
                    Ok(LocalCommandStatus::Handled)
                }
            }
        }
        _ => {
            write_line(sink, responses, "talos: unknown-command")?;
            Ok(LocalCommandStatus::UnknownCommand)
        }
    }
}

fn parse_bounded_directory(path: &str) -> Option<LocalCommandDirectory> {
    match path {
        "/" => Some(LocalCommandDirectory::Root),
        "/etc" => Some(LocalCommandDirectory::Etc),
        "/bin" => Some(LocalCommandDirectory::Bin),
        _ => None,
    }
}

fn directory_exists(directory: LocalCommandDirectory) -> bool {
    initramfs::phase8_readonly_initramfs_fixture()
        .lookup_default(directory.initramfs_path())
        .is_ok()
}

fn is_absolute_exec_path(path: &[u8]) -> bool {
    if path.first() != Some(&b'/') {
        return false;
    }
    !path.iter().any(|byte| is_space(*byte))
}

fn write_root_listing(
    sink: &mut impl LocalCommandSink,
    responses: &mut usize,
) -> Result<(), LocalCommandCycleError> {
    let fs = initramfs::phase8_readonly_initramfs_fixture();
    for (path, name) in LOCAL_COMMAND_ROOT_LISTING {
        if fs.lookup_default(path).is_err() {
            write_line(sink, responses, "talos: filesystem-error")?;
            return Ok(());
        }
        write_line(sink, responses, name)?;
    }
    Ok(())
}

fn write_etc_listing(
    sink: &mut impl LocalCommandSink,
    responses: &mut usize,
) -> Result<(), LocalCommandCycleError> {
    let fs = initramfs::phase8_readonly_initramfs_fixture();
    if fs.lookup_default(b"/etc").is_err() {
        write_line(sink, responses, "talos: filesystem-error")?;
        return Ok(());
    }
    for (path, name) in LOCAL_COMMAND_ETC_LISTING {
        if fs.lookup_default(path).is_err() {
            write_line(sink, responses, "talos: filesystem-error")?;
            return Ok(());
        }
        write_line(sink, responses, name)?;
    }
    Ok(())
}

fn write_bin_listing(
    sink: &mut impl LocalCommandSink,
    responses: &mut usize,
) -> Result<(), LocalCommandCycleError> {
    let fs = initramfs::phase8_readonly_initramfs_fixture();
    if fs.lookup_default(b"/bin").is_err() {
        write_line(sink, responses, "talos: filesystem-error")?;
        return Ok(());
    }
    for (path, name) in LOCAL_COMMAND_BIN_LISTING {
        if fs.lookup_default(path).is_err() {
            write_line(sink, responses, "talos: filesystem-error")?;
            return Ok(());
        }
        write_line(sink, responses, name)?;
    }
    Ok(())
}

fn write_directory_listing(
    sink: &mut impl LocalCommandSink,
    responses: &mut usize,
    directory: LocalCommandDirectory,
) -> Result<(), LocalCommandCycleError> {
    match directory {
        LocalCommandDirectory::Root => write_root_listing(sink, responses),
        LocalCommandDirectory::Etc => write_etc_listing(sink, responses),
        LocalCommandDirectory::Bin => write_bin_listing(sink, responses),
    }
}

fn write_banner_file(
    sink: &mut impl LocalCommandSink,
    responses: &mut usize,
) -> Result<(), LocalCommandCycleError> {
    let mut bytes = [0u8; initramfs::PHASE8_BANNER_BYTES.len()];
    let bytes_read =
        match sink.read_initramfs_file_via_syscall(initramfs::PHASE8_BANNER_PATH, &mut bytes) {
            Ok(bytes_read) => bytes_read,
            Err(_) => {
                write_line(sink, responses, "talos: filesystem-error")?;
                return Ok(());
            }
        };
    let text = match core::str::from_utf8(&bytes[..bytes_read]) {
        Ok(text) => text,
        Err(_) => {
            write_line(sink, responses, "talos: filesystem-error")?;
            return Ok(());
        }
    };
    write_file_contents(sink, responses, text)
}

fn write_stdio_descriptor_line(
    sink: &mut impl LocalCommandSink,
    response_lines: &mut usize,
    descriptor: usize,
) -> Result<(), LocalCommandCycleError> {
    let kind = sink
        .stdio_descriptor_kind(descriptor)
        .unwrap_or("unavailable");
    let descriptor_name = match descriptor {
        posix::STDIN_FD => "0",
        posix::STDOUT_FD => "1",
        posix::STDERR_FD => "2",
        _ => "?",
    };
    write_parts_line(
        sink,
        response_lines,
        &["talos: fd ", descriptor_name, " ", kind],
    )
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ParseLocalCommandError {
    Empty,
    Invalid,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ParsedLocalCommand<'a> {
    name: &'a str,
    arguments: Option<&'a str>,
}

fn parse_local_command(line: &[u8]) -> Result<ParsedLocalCommand<'_>, ParseLocalCommandError> {
    let line = core::str::from_utf8(line).map_err(|_| ParseLocalCommandError::Invalid)?;
    let bytes = line.as_bytes();
    let mut start = 0usize;
    while start < bytes.len() && is_space(bytes[start]) {
        start += 1;
    }
    if start == bytes.len() {
        return Err(ParseLocalCommandError::Empty);
    }

    let mut end = start;
    while end < bytes.len() && !is_space(bytes[end]) {
        if !is_command_byte(bytes[end]) {
            return Err(ParseLocalCommandError::Invalid);
        }
        end += 1;
    }

    let mut next = end;
    while next < bytes.len() && is_space(bytes[next]) {
        next += 1;
    }

    let mut argument_end = bytes.len();
    while argument_end > next && is_space(bytes[argument_end - 1]) {
        argument_end -= 1;
    }
    let arguments = if next == argument_end {
        None
    } else {
        Some(&line[next..argument_end])
    };

    Ok(ParsedLocalCommand {
        name: &line[start..end],
        arguments,
    })
}

const fn is_space(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t')
}

const fn is_command_byte(byte: u8) -> bool {
    matches!(byte, b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_')
}

fn write_line(
    sink: &mut impl LocalCommandSink,
    response_lines: &mut usize,
    text: &str,
) -> Result<(), LocalCommandCycleError> {
    write_parts_line(sink, response_lines, &[text])
}

fn write_parts_line(
    sink: &mut impl LocalCommandSink,
    response_lines: &mut usize,
    parts: &[&str],
) -> Result<(), LocalCommandCycleError> {
    for part in parts {
        sink.write_command_str(part)
            .map_err(|_| LocalCommandCycleError::ResponseWriteFailed)?;
    }
    sink.write_command_str("\n")
        .map_err(|_| LocalCommandCycleError::ResponseWriteFailed)?;
    *response_lines += 1;
    Ok(())
}

fn write_file_contents(
    sink: &mut impl LocalCommandSink,
    response_lines: &mut usize,
    text: &str,
) -> Result<(), LocalCommandCycleError> {
    sink.write_command_str(text)
        .map_err(|_| LocalCommandCycleError::ResponseWriteFailed)?;
    if !text.ends_with('\n') {
        sink.write_command_str("\n")
            .map_err(|_| LocalCommandCycleError::ResponseWriteFailed)?;
    }
    *response_lines += 1;
    Ok(())
}

fn write_exec_summary(
    sink: &mut impl LocalCommandSink,
    response_lines: &mut usize,
    summary: LocalCommandExecSummary,
) -> Result<(), LocalCommandCycleError> {
    write_str_part(sink, "talos: exec path=")?;
    write_byte_path_part(sink, summary.source_path)?;
    write_str_part(sink, " source=vfs-open-read")?;
    finish_dynamic_line(sink, response_lines)?;
    write_exec_source_line(sink, response_lines, summary)?;
    write_exec_entry_line(sink, response_lines, summary)?;
    write_exec_launch_line(sink, response_lines, summary)?;
    write_exec_descriptor_inheritance_line(sink, response_lines, summary)?;
    write_exec_startup_abi_line(sink, response_lines, summary)?;
    write_exec_lifecycle_line(sink, response_lines, summary)?;
    write_exec_status_line(sink, response_lines, summary)?;
    write_line(
        sink,
        response_lines,
        "talos: exec-signal lower-aarch64-svc-launch-boundary-equivalent",
    )
}

fn write_exec_source_line(
    sink: &mut impl LocalCommandSink,
    response_lines: &mut usize,
    summary: LocalCommandExecSummary,
) -> Result<(), LocalCommandCycleError> {
    write_str_part(sink, "talos: exec-source bytes=")?;
    write_hex_usize_part(sink, summary.source_len)?;
    write_str_part(sink, " digest=")?;
    write_hex_u64_part(sink, summary.source_digest)?;
    finish_dynamic_line(sink, response_lines)
}

fn write_exec_entry_line(
    sink: &mut impl LocalCommandSink,
    response_lines: &mut usize,
    summary: LocalCommandExecSummary,
) -> Result<(), LocalCommandCycleError> {
    write_str_part(sink, "talos: exec-loader fixture=")?;
    write_str_part(sink, program_loader::PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY)?;
    write_str_part(sink, " entry=")?;
    write_hex_u64_part(sink, summary.entry)?;
    write_str_part(sink, " segments=")?;
    write_hex_usize_part(sink, summary.segments)?;
    finish_dynamic_line(sink, response_lines)
}

fn write_exec_launch_line(
    sink: &mut impl LocalCommandSink,
    response_lines: &mut usize,
    summary: LocalCommandExecSummary,
) -> Result<(), LocalCommandCycleError> {
    write_str_part(sink, "talos: exec-launch launch-boundary=")?;
    write_str_part(sink, summary.launch_boundary)?;
    write_str_part(sink, " stack-boundary=")?;
    write_str_part(sink, summary.stack_boundary)?;
    write_str_part(sink, " address-space=")?;
    write_hex_u64_part(sink, summary.address_space_id)?;
    write_str_part(sink, " materialization=")?;
    write_hex_u64_part(sink, summary.materialization_id)?;
    write_str_part(sink, " initial-sp=")?;
    write_hex_u64_part(sink, summary.initial_sp)?;
    finish_dynamic_line(sink, response_lines)
}

fn write_exec_descriptor_inheritance_line(
    sink: &mut impl LocalCommandSink,
    response_lines: &mut usize,
    summary: LocalCommandExecSummary,
) -> Result<(), LocalCommandCycleError> {
    let record = summary.descriptor_inheritance;
    write_str_part(sink, "talos: exec-descriptors owner=")?;
    write_hex_u64_part(sink, record.owner_id)?;
    write_str_part(sink, " inherited-count=")?;
    write_hex_usize_part(sink, record.inherited_count)?;
    write_str_part(sink, " fd0=")?;
    write_str_part(sink, record.stdin_kind)?;
    write_str_part(sink, " fd1=")?;
    write_str_part(sink, record.stdout_kind)?;
    write_str_part(sink, " fd2=")?;
    write_str_part(sink, record.stderr_kind)?;
    write_str_part(sink, " loader-temp-fd=")?;
    write_hex_usize_part(sink, record.loader_temporary_descriptor)?;
    write_str_part(sink, " loader-temp-open=")?;
    write_str_part(
        sink,
        if record.loader_temporary_descriptor_open {
            "true"
        } else {
            "false"
        },
    )?;
    write_str_part(sink, " source=")?;
    write_str_part(sink, record.source)?;
    finish_dynamic_line(sink, response_lines)
}

fn write_exec_startup_abi_line(
    sink: &mut impl LocalCommandSink,
    response_lines: &mut usize,
    summary: LocalCommandExecSummary,
) -> Result<(), LocalCommandCycleError> {
    write_str_part(sink, "talos: exec-startup-abi state=")?;
    write_str_part(sink, summary.startup_state)?;
    write_str_part(sink, " argc=")?;
    write_hex_usize_part(sink, summary.startup_argc)?;
    write_str_part(sink, " argv0=")?;
    write_byte_path_part(sink, summary.startup_argv0_path)?;
    write_str_part(sink, " argv0-ptr=")?;
    write_hex_u64_part(sink, summary.startup_argv0_user_address)?;
    write_str_part(sink, " argv-null=")?;
    write_str_part(
        sink,
        if summary.startup_argv_null {
            "true"
        } else {
            "false"
        },
    )?;
    write_str_part(sink, " envp-null=")?;
    write_str_part(
        sink,
        if summary.startup_envp_null {
            "true"
        } else {
            "false"
        },
    )?;
    write_str_part(sink, " envp-state=")?;
    write_str_part(sink, summary.startup_envp_state)?;
    write_str_part(sink, " envp-entries=")?;
    write_hex_usize_part(sink, summary.startup_envp_entry_count)?;
    write_str_part(sink, " envp0-ptr=")?;
    write_hex_u64_part(sink, summary.startup_envp0_user_address)?;
    write_str_part(sink, " copied-startup-bytes=")?;
    write_hex_u64_part(sink, summary.copied_startup_bytes)?;
    write_str_part(sink, " source=initial-user-stack-record")?;
    finish_dynamic_line(sink, response_lines)
}

fn write_exec_lifecycle_line(
    sink: &mut impl LocalCommandSink,
    response_lines: &mut usize,
    summary: LocalCommandExecSummary,
) -> Result<(), LocalCommandCycleError> {
    let lifecycle = summary.lifecycle;
    write_str_part(sink, "talos: exec-lifecycle pid=")?;
    write_hex_u64_part(sink, lifecycle.process_id)?;
    write_str_part(sink, " parent=shell owner=")?;
    write_hex_u64_part(sink, lifecycle.parent_owner_id)?;
    write_str_part(sink, " path=")?;
    write_byte_path_part(sink, lifecycle.source_path)?;
    write_str_part(sink, " state=")?;
    write_str_part(sink, lifecycle.state.name())?;
    write_str_part(sink, " status=")?;
    write_hex_u64_part(sink, lifecycle.status)?;
    write_str_part(sink, " observed-status=")?;
    write_hex_u64_part(sink, lifecycle.observed_status)?;
    write_str_part(sink, " reaped=")?;
    write_str_part(sink, if lifecycle.reaped { "true" } else { "false" })?;
    finish_dynamic_line(sink, response_lines)
}

fn write_exec_status_line(
    sink: &mut impl LocalCommandSink,
    response_lines: &mut usize,
    summary: LocalCommandExecSummary,
) -> Result<(), LocalCommandCycleError> {
    write_str_part(sink, "talos: exec-status boundary=")?;
    write_str_part(sink, summary.completion_boundary)?;
    write_str_part(sink, " marker=")?;
    write_hex_u64_part(sink, summary.completion_marker)?;
    write_str_part(sink, " status=")?;
    write_hex_u64_part(sink, summary.completion_status)?;
    write_str_part(sink, " complete=true source=lifecycle-record")?;
    finish_dynamic_line(sink, response_lines)
}

fn write_last_process_status_line(
    sink: &mut impl LocalCommandSink,
    response_lines: &mut usize,
    lifecycle: LocalCommandProcessLifecycleRecord,
) -> Result<(), LocalCommandCycleError> {
    write_str_part(sink, "talos: last-process pid=")?;
    write_hex_u64_part(sink, lifecycle.process_id)?;
    write_str_part(sink, " parent=shell owner=")?;
    write_hex_u64_part(sink, lifecycle.parent_owner_id)?;
    write_str_part(sink, " path=")?;
    write_byte_path_part(sink, lifecycle.source_path)?;
    write_str_part(sink, " state=")?;
    write_str_part(sink, lifecycle.state.name())?;
    write_str_part(sink, " status=")?;
    write_hex_u64_part(sink, lifecycle.status)?;
    write_str_part(sink, " observed-status=")?;
    write_hex_u64_part(sink, lifecycle.observed_status)?;
    write_str_part(sink, " reaped=")?;
    write_str_part(sink, if lifecycle.reaped { "true" } else { "false" })?;
    write_str_part(sink, " source=lifecycle-record")?;
    finish_dynamic_line(sink, response_lines)
}

fn write_waitpid_status_line(
    sink: &mut impl LocalCommandSink,
    response_lines: &mut usize,
    lifecycle: LocalCommandProcessLifecycleRecord,
) -> Result<(), LocalCommandCycleError> {
    write_str_part(sink, "talos: waitpid pid=")?;
    write_hex_u64_part(sink, lifecycle.process_id)?;
    write_str_part(sink, " parent=shell owner=")?;
    write_hex_u64_part(sink, lifecycle.parent_owner_id)?;
    write_str_part(sink, " path=")?;
    write_byte_path_part(sink, lifecycle.source_path)?;
    write_str_part(sink, " state=")?;
    write_str_part(sink, lifecycle.state.name())?;
    write_str_part(sink, " status=")?;
    write_hex_u64_part(sink, lifecycle.status)?;
    write_str_part(sink, " observed-status=")?;
    write_hex_u64_part(sink, lifecycle.observed_status)?;
    write_str_part(sink, " reaped=true source=lifecycle-record")?;
    finish_dynamic_line(sink, response_lines)
}

fn write_str_part(
    sink: &mut impl LocalCommandSink,
    text: &str,
) -> Result<(), LocalCommandCycleError> {
    sink.write_command_str(text)
        .map_err(|_| LocalCommandCycleError::ResponseWriteFailed)
}

fn write_byte_path_part(
    sink: &mut impl LocalCommandSink,
    path: &[u8],
) -> Result<(), LocalCommandCycleError> {
    let text =
        core::str::from_utf8(path).map_err(|_| LocalCommandCycleError::ResponseWriteFailed)?;
    write_str_part(sink, text)
}

fn write_hex_usize_part(
    sink: &mut impl LocalCommandSink,
    value: usize,
) -> Result<(), LocalCommandCycleError> {
    write_hex_u64_part(sink, value as u64)
}

fn write_hex_u64_part(
    sink: &mut impl LocalCommandSink,
    value: u64,
) -> Result<(), LocalCommandCycleError> {
    let mut bytes = [0u8; 18];
    bytes[0] = b'0';
    bytes[1] = b'x';
    let mut shift = 60usize;
    let mut index = 2usize;
    while index < bytes.len() {
        let digit = ((value >> shift) & 0xf) as u8;
        bytes[index] = match digit {
            0..=9 => b'0' + digit,
            _ => b'a' + (digit - 10),
        };
        if shift == 0 {
            break;
        }
        shift -= 4;
        index += 1;
    }
    let text =
        core::str::from_utf8(&bytes).map_err(|_| LocalCommandCycleError::ResponseWriteFailed)?;
    write_str_part(sink, text)
}

fn finish_dynamic_line(
    sink: &mut impl LocalCommandSink,
    response_lines: &mut usize,
) -> Result<(), LocalCommandCycleError> {
    write_str_part(sink, "\n")?;
    *response_lines += 1;
    Ok(())
}

fn copy_line(line: &[u8]) -> [u8; CANONICAL_LINE_CAPACITY] {
    let mut copy = [0; CANONICAL_LINE_CAPACITY];
    copy[..line.len()].copy_from_slice(line);
    copy
}

#[cfg(test)]
mod tests {
    use crate::tty::{CONTROL_EVENT_CAPACITY, TtyControlEvent};

    use super::*;

    struct ScriptedInput<const N: usize> {
        bytes: [u8; N],
        len: usize,
        pos: usize,
    }

    impl<const N: usize> ScriptedInput<N> {
        const fn new(bytes: [u8; N], len: usize) -> Self {
            Self { bytes, len, pos: 0 }
        }
    }

    impl<const N: usize> ConsoleInputBackend for ScriptedInput<N> {
        fn poll_read_byte(&mut self) -> Option<u8> {
            if self.pos == self.len {
                return None;
            }
            let byte = self.bytes[self.pos];
            self.pos += 1;
            Some(byte)
        }
    }

    struct CaptureSink {
        bytes: [u8; 4096],
        len: usize,
        fail_after: usize,
        writes: usize,
    }

    impl CaptureSink {
        const fn new() -> Self {
            Self {
                bytes: [0; 4096],
                len: 0,
                fail_after: usize::MAX,
                writes: 0,
            }
        }

        const fn failing_after(fail_after: usize) -> Self {
            Self {
                bytes: [0; 4096],
                len: 0,
                fail_after,
                writes: 0,
            }
        }

        fn as_str(&self) -> &str {
            core::str::from_utf8(&self.bytes[..self.len]).expect("capture is utf8")
        }
    }

    impl LocalCommandSink for CaptureSink {
        fn write_command_str(&mut self, text: &str) -> Result<(), LocalCommandWriteError> {
            if self.writes >= self.fail_after {
                return Err(LocalCommandWriteError::BackendWriteFailed);
            }
            let end = self.len + text.len();
            self.bytes[self.len..end].copy_from_slice(text.as_bytes());
            self.len = end;
            self.writes += 1;
            Ok(())
        }
    }

    impl core::fmt::Write for CaptureSink {
        fn write_str(&mut self, text: &str) -> core::fmt::Result {
            if self.writes >= self.fail_after {
                return Err(core::fmt::Error);
            }
            let end = self.len + text.len();
            self.bytes[self.len..end].copy_from_slice(text.as_bytes());
            self.len = end;
            self.writes += 1;
            Ok(())
        }
    }

    #[test_case]
    fn local_command_loop_dispatches_kernel_backed_help() {
        let mut input = ScriptedInput::new(
            [
                b'h', b'e', b'l', b'p', b'\r', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            5,
        );
        let mut sink = CaptureSink::new();

        let result = run_one_serial_command(&mut input, &mut sink).unwrap();

        assert_eq!(result.line(), b"help");
        assert_eq!(result.status(), LocalCommandStatus::Handled);
        assert_eq!(result.status_name(), "handled");
        assert_eq!(result.response_lines(), 4);
        assert_eq!(result.raw_bytes(), 5);
        assert_eq!(result.controls(), 0);
        assert_eq!(DEFAULT_LOCAL_COMMAND_COUNT, 8);
        assert_eq!(
            sink.as_str(),
            "talos> talos: ok help\n\
	talos: commands help status stdio pwd echo ls cat cd exec laststatus waitpid\n\
	talos: echo forms echo hello; echo local serial works\n\
	talos: editing backspace delete ctrl-c ctrl-u\n"
        );
    }

    #[test_case]
    fn local_command_loop_dispatches_kernel_backed_echo_argument() {
        let input = ScriptedInput::new(
            [
                b'e', b'c', b'h', b'o', b' ', b'h', b'e', b'l', b'l', b'o', b'\r', 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            11,
        );
        let mut backend = CaptureSink::new();
        let result = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            run_one_descriptor_backed_serial_command(&mut io).unwrap()
        };

        assert_eq!(result.line(), b"echo hello");
        assert_eq!(result.status(), LocalCommandStatus::Handled);
        assert_eq!(result.response_lines(), 1);
        assert_eq!(backend.as_str(), "talos> hello\n");
    }

    #[test_case]
    fn local_command_loop_dispatches_bounded_literal_echo_tail() {
        let input = ScriptedInput::new(
            [
                b'e', b'c', b'h', b'o', b' ', b'l', b'o', b'c', b'a', b'l', b' ', b's', b'e', b'r',
                b'i', b'a', b'l', b' ', b'w', b'o', b'r', b'k', b's', b'\r', 0, 0, 0, 0, 0, 0, 0,
                0,
            ],
            24,
        );
        let mut backend = CaptureSink::new();
        let result = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            run_one_descriptor_backed_serial_command(&mut io).unwrap()
        };

        assert_eq!(result.line(), b"echo local serial works");
        assert_eq!(result.status(), LocalCommandStatus::Handled);
        assert_eq!(result.response_lines(), 1);
        assert_eq!(backend.as_str(), "talos> local serial works\n");
    }

    #[test_case]
    fn local_command_loop_dispatches_root_only_pwd() {
        let input = ScriptedInput::new(
            [
                b'p', b'w', b'd', b'\r', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            4,
        );
        let mut backend = CaptureSink::new();
        let result = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            run_one_descriptor_backed_serial_command(&mut io).unwrap()
        };

        assert_eq!(result.line(), b"pwd");
        assert_eq!(result.status(), LocalCommandStatus::Handled);
        assert_eq!(result.response_lines(), 1);
        assert_eq!(backend.as_str(), "talos> /\n");
    }

    #[test_case]
    fn local_command_loop_tracks_bounded_current_directory() {
        let input = ScriptedInput::new(
            [
                b'p', b'w', b'd', b'\r', b'c', b'd', b' ', b'/', b'e', b't', b'c', b'\r', b'p',
                b'w', b'd', b'\r', b'c', b'd', b' ', b'/', b'b', b'i', b'n', b'\r', b'p', b'w',
                b'd', b'\r', b'c', b'd', b' ', b'/', b'\r', b'p', b'w', b'd', b'\r', b'c', b'd',
                b' ', b'/', b'm', b'i', b's', b's', b'i', b'n', b'g', b'\r', b'p', b'w', b'd',
                b'\r', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            53,
        );
        let mut backend = CaptureSink::new();
        let (
            initial_pwd,
            cd_etc,
            etc_pwd,
            cd_bin,
            bin_pwd,
            cd_root,
            root_pwd,
            missing_cd,
            final_pwd,
        ) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            (
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
            )
        };

        assert_eq!(initial_pwd.line(), b"pwd");
        assert_eq!(initial_pwd.status(), LocalCommandStatus::Handled);
        assert_eq!(initial_pwd.response_lines(), 1);
        assert_eq!(cd_etc.line(), b"cd /etc");
        assert_eq!(cd_etc.status(), LocalCommandStatus::Handled);
        assert_eq!(cd_etc.response_lines(), 0);
        assert_eq!(etc_pwd.line(), b"pwd");
        assert_eq!(etc_pwd.response_lines(), 1);
        assert_eq!(cd_bin.line(), b"cd /bin");
        assert_eq!(cd_bin.status(), LocalCommandStatus::Handled);
        assert_eq!(cd_bin.response_lines(), 0);
        assert_eq!(bin_pwd.line(), b"pwd");
        assert_eq!(bin_pwd.response_lines(), 1);
        assert_eq!(cd_root.line(), b"cd /");
        assert_eq!(cd_root.status(), LocalCommandStatus::Handled);
        assert_eq!(cd_root.response_lines(), 0);
        assert_eq!(root_pwd.line(), b"pwd");
        assert_eq!(root_pwd.response_lines(), 1);
        assert_eq!(missing_cd.line(), b"cd /missing");
        assert_eq!(missing_cd.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(missing_cd.response_lines(), 1);
        assert_eq!(final_pwd.line(), b"pwd");
        assert_eq!(final_pwd.response_lines(), 1);
        assert_eq!(
            backend.as_str(),
            "talos> /\n\
talos> talos> /etc\n\
talos> talos> /bin\n\
talos> talos> /\n\
talos> talos: not-directory\n\
talos> /\n"
        );
    }

    #[test_case]
    fn local_command_loop_dispatches_bounded_ls_root() {
        let input = ScriptedInput::new(
            [
                b'l', b's', b' ', b'/', b'\r', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            5,
        );
        let mut backend = CaptureSink::new();
        let result = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            run_one_descriptor_backed_serial_command(&mut io).unwrap()
        };

        assert_eq!(result.line(), b"ls /");
        assert_eq!(result.status(), LocalCommandStatus::Handled);
        assert_eq!(result.response_lines(), 4);
        assert_eq!(backend.as_str(), "talos> bin\ndir\nempty\netc\n");
    }

    #[test_case]
    fn local_command_loop_dispatches_bounded_ls_bin() {
        let input = ScriptedInput::new(
            [
                b'l', b's', b' ', b'/', b'b', b'i', b'n', b'\r', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            8,
        );
        let mut backend = CaptureSink::new();
        let result = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            run_one_descriptor_backed_serial_command(&mut io).unwrap()
        };

        assert_eq!(result.line(), b"ls /bin");
        assert_eq!(result.status(), LocalCommandStatus::Handled);
        assert_eq!(result.response_lines(), 3);
        assert_eq!(backend.as_str(), "talos> init\nzero\nstatus42\n");
    }

    #[test_case]
    fn local_command_loop_dispatches_bare_ls_against_current_directory() {
        let input = ScriptedInput::new(
            [
                b'l', b's', b'\r', b'c', b'd', b' ', b'/', b'e', b't', b'c', b'\r', b'l', b's',
                b'\r', b'c', b'd', b' ', b'/', b'b', b'i', b'n', b'\r', b'l', b's', b'\r', b'c',
                b'd', b' ', b'/', b'\r', b'l', b's', b'\r', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            34,
        );
        let mut backend = CaptureSink::new();
        let (root_ls, cd_etc, etc_ls, cd_bin, bin_ls, cd_root, final_root_ls) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            (
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
            )
        };

        assert_eq!(root_ls.line(), b"ls");
        assert_eq!(root_ls.status(), LocalCommandStatus::Handled);
        assert_eq!(root_ls.response_lines(), 4);
        assert_eq!(cd_etc.line(), b"cd /etc");
        assert_eq!(cd_etc.status(), LocalCommandStatus::Handled);
        assert_eq!(cd_etc.response_lines(), 0);
        assert_eq!(etc_ls.line(), b"ls");
        assert_eq!(etc_ls.status(), LocalCommandStatus::Handled);
        assert_eq!(etc_ls.response_lines(), 1);
        assert_eq!(cd_bin.line(), b"cd /bin");
        assert_eq!(cd_bin.status(), LocalCommandStatus::Handled);
        assert_eq!(cd_bin.response_lines(), 0);
        assert_eq!(bin_ls.line(), b"ls");
        assert_eq!(bin_ls.status(), LocalCommandStatus::Handled);
        assert_eq!(bin_ls.response_lines(), 3);
        assert_eq!(cd_root.line(), b"cd /");
        assert_eq!(cd_root.status(), LocalCommandStatus::Handled);
        assert_eq!(cd_root.response_lines(), 0);
        assert_eq!(final_root_ls.line(), b"ls");
        assert_eq!(final_root_ls.status(), LocalCommandStatus::Handled);
        assert_eq!(final_root_ls.response_lines(), 4);
        assert_eq!(
            backend.as_str(),
            "talos> bin\n\
dir\n\
empty\n\
etc\n\
talos> talos> banner.txt\n\
	talos> talos> init\n\
	zero\n\
	status42\n\
	talos> talos> bin\n\
dir\n\
empty\n\
etc\n"
        );
    }

    #[test_case]
    fn local_command_loop_dispatches_bounded_cat_banner() {
        let input = ScriptedInput::new(
            [
                b'c', b'a', b't', b' ', b'/', b'e', b't', b'c', b'/', b'b', b'a', b'n', b'n', b'e',
                b'r', b'.', b't', b'x', b't', b'\r', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            20,
        );
        let mut backend = CaptureSink::new();
        let result = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            run_one_descriptor_backed_serial_command(&mut io).unwrap()
        };

        assert_eq!(result.line(), b"cat /etc/banner.txt");
        assert_eq!(result.status(), LocalCommandStatus::Handled);
        assert_eq!(result.response_lines(), 1);
        assert_eq!(backend.as_str(), "talos> Talos initramfs fixture\n");
    }

    #[test_case]
    fn local_command_loop_cats_banner_through_reusable_vfs_syscall_descriptor() {
        let input = ScriptedInput::new(*b"cat /etc/banner.txt\rcat /etc/banner.txt\r", 40);
        let mut backend = CaptureSink::new();
        let (first, second) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            (
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
            )
        };

        assert_eq!(first.line(), b"cat /etc/banner.txt");
        assert_eq!(first.status(), LocalCommandStatus::Handled);
        assert_eq!(first.response_lines(), 1);
        assert_eq!(second.line(), b"cat /etc/banner.txt");
        assert_eq!(second.status(), LocalCommandStatus::Handled);
        assert_eq!(second.response_lines(), 1);
        assert_eq!(
            backend.as_str(),
            "talos> Talos initramfs fixture\n\
talos> Talos initramfs fixture\n"
        );
    }

    #[test_case]
    fn local_command_loop_execs_init_through_vfs_launch_boundary() {
        let input = ScriptedInput::new(*b"exec /bin/init\r", 15);
        let mut backend = CaptureSink::new();
        let result = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            run_one_descriptor_backed_serial_command(&mut io).unwrap()
        };
        let output = backend.as_str();

        assert_eq!(result.line(), b"exec /bin/init");
        assert_eq!(result.status(), LocalCommandStatus::Handled);
        assert_eq!(result.response_lines(), 9);
        assert!(output.contains("talos> talos: exec path=/bin/init source=vfs-open-read\n"));
        assert!(output.contains("talos: exec-source bytes=0x0000000000000204 digest=0x"));
        assert!(output.contains(
            "talos: exec-loader fixture=phase8-program-loader-elf64-aarch64-v1 entry=0x"
        ));
        assert!(output.contains(
            "talos: exec-launch launch-boundary=phase8-initial-process-launch-plan-v1 stack-boundary=phase8-initial-user-stack-plan-v1"
        ));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-startup-abi state=minimal-argc1-argv0-init-empty-envp argc=0x0000000000000001 argv0=/bin/init"
        ));
        assert!(output.contains(
            "argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x00007fffffffffe8 copied-startup-bytes=0x000000000000002a source=initial-user-stack-record\n"
        ));
        assert!(output.contains(
            "talos: exec-lifecycle pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/init state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true\n"
        ));
        assert!(output.contains(
            "talos: exec-status boundary=lower-aarch64-svc-status-equivalent marker=0x0000000000007a10 status=0x0000000000000000 complete=true source=lifecycle-record\n"
        ));
        assert!(
            output.contains("talos: exec-signal lower-aarch64-svc-launch-boundary-equivalent\n")
        );
    }

    #[test_case]
    fn local_command_loop_execs_non_init_absolute_vfs_program() {
        let input = ScriptedInput::new(*b"exec /bin/zero\r", 15);
        let mut backend = CaptureSink::new();
        let result = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            run_one_descriptor_backed_serial_command(&mut io).unwrap()
        };
        let output = backend.as_str();

        assert_eq!(result.line(), b"exec /bin/zero");
        assert_eq!(result.status(), LocalCommandStatus::Handled);
        assert_eq!(result.response_lines(), 9);
        assert!(output.contains("talos> talos: exec path=/bin/zero source=vfs-open-read\n"));
        assert!(output.contains("talos: exec-source bytes=0x0000000000000204 digest=0x"));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/zero"
        ));
        assert!(output.contains(
            "argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x00007fffffffffe8 copied-startup-bytes=0x000000000000002a source=initial-user-stack-record\n"
        ));
        assert!(output.contains(
            "talos: exec-lifecycle pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/zero state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true\n"
        ));
        assert!(output.contains(
            "talos: exec-status boundary=lower-aarch64-svc-status-equivalent marker=0x0000000000007a10 status=0x0000000000000000 complete=true source=lifecycle-record\n"
        ));
    }

    #[test_case]
    fn local_command_loop_execs_nonzero_status_vfs_program() {
        let input = ScriptedInput::new(*b"exec /bin/status42\rlaststatus\r", 30);
        let mut backend = CaptureSink::new();
        let (exec, observed) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            (
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
            )
        };
        let output = backend.as_str();

        assert_eq!(exec.line(), b"exec /bin/status42");
        assert_eq!(exec.status(), LocalCommandStatus::Handled);
        assert_eq!(exec.response_lines(), 9);
        assert_eq!(observed.line(), b"laststatus");
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.response_lines(), 1);
        assert!(output.contains("talos> talos: exec path=/bin/status42 source=vfs-open-read\n"));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/status42"
        ));
        assert!(output.contains(
            "talos: exec-lifecycle pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true\n"
        ));
        assert!(output.contains(
            "talos: exec-status boundary=lower-aarch64-svc-status-equivalent marker=0x0000000000007a10 status=0x000000000000002a complete=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=lifecycle-record\n"
        ));
    }

    #[test_case]
    fn local_command_loop_waitpid_consumes_vfs_exec_lifecycle_record() {
        let input = ScriptedInput::new(
            *b"waitpid\rexec /bin/status42\rwaitpid\rwaitpid\rlaststatus\r",
            54,
        );
        let mut backend = CaptureSink::new();
        let (empty, exec, waited, consumed, observed) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            (
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
            )
        };
        let output = backend.as_str();

        assert_eq!(empty.line(), b"waitpid");
        assert_eq!(empty.status(), LocalCommandStatus::Handled);
        assert_eq!(empty.response_lines(), 1);
        assert_eq!(exec.line(), b"exec /bin/status42");
        assert_eq!(exec.status(), LocalCommandStatus::Handled);
        assert_eq!(exec.response_lines(), 9);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(waited.response_lines(), 1);
        assert_eq!(consumed.line(), b"waitpid");
        assert_eq!(consumed.status(), LocalCommandStatus::Handled);
        assert_eq!(consumed.response_lines(), 1);
        assert_eq!(observed.line(), b"laststatus");
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.response_lines(), 1);
        assert!(output.contains("talos> talos: waitpid no-child source=lifecycle-record\n"));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=lifecycle-record\n"
        ));
    }

    #[test_case]
    fn local_command_loop_observes_last_process_status_from_lifecycle_record() {
        let input = ScriptedInput::new(*b"laststatus\rexec /bin/init\rlaststatus\r", 37);
        let mut backend = CaptureSink::new();
        let (none, exec, observed) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            (
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
            )
        };
        let output = backend.as_str();

        assert_eq!(none.line(), b"laststatus");
        assert_eq!(none.status(), LocalCommandStatus::Handled);
        assert_eq!(none.response_lines(), 1);
        assert_eq!(exec.line(), b"exec /bin/init");
        assert_eq!(exec.status(), LocalCommandStatus::Handled);
        assert_eq!(exec.response_lines(), 9);
        assert_eq!(observed.line(), b"laststatus");
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.response_lines(), 1);
        assert!(output.contains("talos> talos: last-process none\n"));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-lifecycle pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/init state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/init state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
    }

    #[test_case]
    fn local_command_loop_rejects_missing_and_non_executable_exec_targets() {
        let input = ScriptedInput::new(
            *b"exec /missing\rexec init\rexec /bin\rexec /etc/banner.txt\rexec /empty\r",
            67,
        );
        let mut backend = CaptureSink::new();
        let (missing, relative, directory, banner, empty) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            (
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
            )
        };

        assert_eq!(missing.line(), b"exec /missing");
        assert_eq!(missing.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(missing.response_lines(), 1);
        assert_eq!(relative.line(), b"exec init");
        assert_eq!(relative.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(relative.response_lines(), 1);
        assert_eq!(directory.line(), b"exec /bin");
        assert_eq!(directory.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(directory.response_lines(), 1);
        assert_eq!(banner.line(), b"exec /etc/banner.txt");
        assert_eq!(banner.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(banner.response_lines(), 1);
        assert_eq!(empty.line(), b"exec /empty");
        assert_eq!(empty.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(empty.response_lines(), 1);
        assert_eq!(
            backend.as_str(),
            "talos> talos: exec-not-found\n\
talos> talos: exec-invalid-path\n\
talos> talos: exec-not-executable\n\
talos> talos: exec-not-executable\n\
talos> talos: exec-not-executable\n"
        );
    }

    #[test_case]
    fn local_command_loop_dispatches_cat_banner_against_current_directory() {
        let input = ScriptedInput::new(*b"cd /etc\rcat banner.txt\rcd /\rcat banner.txt\r", 43);
        let mut backend = CaptureSink::new();
        let (cd_etc, cat_etc, cd_root, cat_root) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            (
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
            )
        };

        assert_eq!(cd_etc.line(), b"cd /etc");
        assert_eq!(cd_etc.status(), LocalCommandStatus::Handled);
        assert_eq!(cd_etc.response_lines(), 0);
        assert_eq!(cat_etc.line(), b"cat banner.txt");
        assert_eq!(cat_etc.status(), LocalCommandStatus::Handled);
        assert_eq!(cat_etc.response_lines(), 1);
        assert_eq!(cd_root.line(), b"cd /");
        assert_eq!(cd_root.status(), LocalCommandStatus::Handled);
        assert_eq!(cd_root.response_lines(), 0);
        assert_eq!(cat_root.line(), b"cat banner.txt");
        assert_eq!(cat_root.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(cat_root.response_lines(), 1);
        assert_eq!(
            backend.as_str(),
            "talos> talos> Talos initramfs fixture\n\
talos> talos> talos: not-found\n"
        );
    }

    #[test_case]
    fn local_command_loop_applies_backspace_and_delete_before_dispatch() {
        let backspace_input = ScriptedInput::new(
            [
                b'p', b'w', b'x', 0x08, b'd', b'\r', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            6,
        );
        let mut backspace_backend = CaptureSink::new();
        let backspace_result = {
            let mut io = DescriptorBackedLocalCommandIo::new_inherited_stdio(
                backspace_input,
                &mut backspace_backend,
            )
            .unwrap();
            run_one_descriptor_backed_serial_command(&mut io).unwrap()
        };

        assert_eq!(backspace_result.line(), b"pwd");
        assert_eq!(backspace_result.status(), LocalCommandStatus::Handled);
        assert_eq!(backspace_result.response_lines(), 1);
        assert_eq!(backspace_result.backspaces(), 1);
        assert_eq!(backspace_result.deletes(), 0);
        assert_eq!(backspace_backend.as_str(), "talos> /\n");

        let delete_input = ScriptedInput::new(
            [
                b'p', b'w', b'x', 0x7f, b'd', b'\r', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            6,
        );
        let mut delete_backend = CaptureSink::new();
        let delete_result = {
            let mut io = DescriptorBackedLocalCommandIo::new_inherited_stdio(
                delete_input,
                &mut delete_backend,
            )
            .unwrap();
            run_one_descriptor_backed_serial_command(&mut io).unwrap()
        };

        assert_eq!(delete_result.line(), b"pwd");
        assert_eq!(delete_result.status(), LocalCommandStatus::Handled);
        assert_eq!(delete_result.response_lines(), 1);
        assert_eq!(delete_result.backspaces(), 0);
        assert_eq!(delete_result.deletes(), 1);
        assert_eq!(delete_backend.as_str(), "talos> /\n");
    }

    #[test_case]
    fn local_command_loop_cancels_partial_line_on_ctrl_c_before_next_dispatch() {
        let input = ScriptedInput::new(
            [
                b'b', b'o', b'g', b'u', b's', 0x03, b'p', b'w', b'd', b'\r', 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            10,
        );
        let mut backend = CaptureSink::new();
        let (cancel_result, pwd_result) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            let cancel_result = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let pwd_result = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            (cancel_result, pwd_result)
        };

        assert_eq!(cancel_result.line(), b"");
        assert_eq!(cancel_result.status(), LocalCommandStatus::LineCanceled);
        assert_eq!(cancel_result.response_lines(), 1);
        assert_eq!(cancel_result.raw_bytes(), 6);
        assert_eq!(cancel_result.controls(), 1);
        assert_eq!(pwd_result.line(), b"pwd");
        assert_eq!(pwd_result.status(), LocalCommandStatus::Handled);
        assert_eq!(pwd_result.response_lines(), 1);
        assert_eq!(backend.as_str(), "talos> talos: line-canceled\ntalos> /\n");
    }

    #[test_case]
    fn local_command_loop_kills_partial_line_on_ctrl_u_before_dispatch() {
        let input = ScriptedInput::new(
            [
                b'b', b'o', b'g', b'u', b's', 0x15, b'p', b'w', b'd', b'\r', 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            10,
        );
        let mut backend = CaptureSink::new();
        let result = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            run_one_descriptor_backed_serial_command(&mut io).unwrap()
        };

        assert_eq!(result.line(), b"pwd");
        assert_eq!(result.status(), LocalCommandStatus::Handled);
        assert_eq!(result.response_lines(), 2);
        assert_eq!(result.raw_bytes(), 10);
        assert_eq!(result.controls(), 1);
        assert_eq!(backend.as_str(), "talos> talos: line-killed\n/\n");
    }

    #[test_case]
    fn local_command_loop_rejects_truncated_control_history() {
        let input = ScriptedInput::new(
            [
                0x15, 0x15, 0x15, 0x15, 0x15, 0x15, 0x15, 0x15, 0x15, b'p', b'w', b'd', b'\r', 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            13,
        );
        let mut backend = CaptureSink::new();
        let result = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            run_one_descriptor_backed_serial_command(&mut io).unwrap()
        };

        assert_eq!(result.line(), b"pwd");
        assert_eq!(
            result.status(),
            LocalCommandStatus::InputError(PollingTtyRxOutcome::LineComplete)
        );
        assert_eq!(result.controls(), CONTROL_EVENT_CAPACITY);
        assert!(result.controls_truncated());
        assert_eq!(
            backend.as_str(),
            "talos> talos: input-error line-complete\n"
        );
    }

    #[test_case]
    fn local_command_loop_rejects_arguments_for_non_echo_builtins() {
        let mut input = ScriptedInput::new(
            [
                b's', b't', b'a', b't', b'u', b's', b' ', b'n', b'o', b'w', b'\r', 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            11,
        );
        let mut sink = CaptureSink::new();
        let result = run_one_serial_command(&mut input, &mut sink).unwrap();

        assert_eq!(result.line(), b"status now");
        assert_eq!(result.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(result.response_lines(), 1);
        assert_eq!(sink.as_str(), "talos> talos: unexpected-argument\n");
    }

    #[test_case]
    fn local_command_loop_reports_descriptor_backed_stdio() {
        let input = ScriptedInput::new(
            [
                b's', b't', b'd', b'i', b'o', b'\r', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            6,
        );
        let mut backend = CaptureSink::new();
        let result = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            run_one_descriptor_backed_serial_command(&mut io).unwrap()
        };

        assert_eq!(result.line(), b"stdio");
        assert_eq!(result.status(), LocalCommandStatus::Handled);
        assert_eq!(result.response_lines(), 7);
        assert_eq!(
            backend.as_str(),
            "talos> talos: ok stdio\n\
talos: fd 0 stdio-input\n\
talos: fd 1 stdio-output\n\
talos: fd 2 stdio-output\n\
talos: runtime-console runtime-console0\n\
talos: descriptor-backed-input=true\n\
talos: descriptor-backed-output=true\n"
        );
    }

    #[test_case]
    fn local_command_loop_handles_empty_and_unknown_input_visibly() {
        let mut empty_input = ScriptedInput::new([b'\r'; 32], 1);
        let mut empty_sink = CaptureSink::new();
        let empty = run_one_serial_command(&mut empty_input, &mut empty_sink).unwrap();

        assert_eq!(empty.line(), b"");
        assert_eq!(empty.status(), LocalCommandStatus::Empty);
        assert_eq!(empty_sink.as_str(), "talos> talos: empty-command\n");

        let mut unknown_input = ScriptedInput::new(
            [
                b'b', b'o', b'g', b'u', b's', b'\n', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            6,
        );
        let mut unknown_sink = CaptureSink::new();
        let unknown = run_one_serial_command(&mut unknown_input, &mut unknown_sink).unwrap();

        assert_eq!(unknown.line(), b"bogus");
        assert_eq!(unknown.status(), LocalCommandStatus::UnknownCommand);
        assert_eq!(unknown.response_lines(), 1);
        assert_eq!(unknown_sink.as_str(), "talos> talos: unknown-command\n");
    }

    #[test_case]
    fn local_command_loop_reports_input_and_response_failures() {
        let mut truncated_input = ScriptedInput::new(
            [
                b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n',
                b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'1', b'2',
                b'3', b'4', b'5', b'6', b'7', b'\r', 0, 0, 0, 0, 0, 0,
            ],
            34,
        );
        let mut sink = CaptureSink::new();
        let result = run_one_serial_command(&mut truncated_input, &mut sink).unwrap();

        assert_eq!(
            result.status(),
            LocalCommandStatus::InputError(PollingTtyRxOutcome::LineComplete)
        );
        assert!(result.truncated());
        assert_eq!(sink.as_str(), "talos> talos: input-error line-complete\n");

        let mut input = ScriptedInput::new([b'\r'; 32], 1);
        let mut failing_sink = CaptureSink::failing_after(0);
        assert_eq!(
            run_one_serial_command(&mut input, &mut failing_sink).unwrap_err(),
            LocalCommandCycleError::PromptWriteFailed
        );
    }

    #[test_case]
    fn local_command_status_names_are_stable_for_transcripts() {
        assert_eq!(LocalCommandStatus::Empty.name(), "empty-command");
        assert_eq!(LocalCommandStatus::LineCanceled.name(), "line-canceled");
        assert_eq!(LocalCommandStatus::UnknownCommand.name(), "unknown-command");
        assert_eq!(
            LocalCommandStatus::InputError(PollingTtyRxOutcome::Timeout).name(),
            "input-error"
        );
        assert_eq!(TtyControlEvent::Interrupt.name(), "ctrl-c");
        assert_eq!(TtyControlEvent::ClearLine.name(), "ctrl-u");
    }
}
