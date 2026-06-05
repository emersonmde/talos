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
    scheduler::{ProcessOwnerId, TaskId, TaskState},
    syscall,
    tty::{self, CANONICAL_LINE_CAPACITY, PollingTtyRxOutcome, PollingTtyRxResult},
};

pub const LOCAL_COMMAND_LOOP_VERSION: &str = "phase10.2-kernel-builtins-v1";
pub const LOCAL_COMMAND_BUILTIN_BOUNDARY: &str = concat!(
    "kernel-backed-regression-control+vfs-syscall-cat+vfs-userspace-exec-boundary",
    "+lifecycle-laststatus+waitpid-lifecycle-observation+standard-descriptor-inheritance",
    "+userspace-stdout-through-inherited-fd1+userspace-stdin-through-inherited-fd0",
    "+userspace-stderr-through-inherited-fd2+descriptor-dup-redirection-1-to-2",
    "+descriptor-dup-redirection-2-to-1+descriptor-close-redirection-1",
    "+descriptor-close-redirection-2+minimal-stdout-to-stdin-pipeline",
    "+stdout-only-pipeline-stderr-not-piped+pipeline-stderr-dup-to-stdout",
    "+pipeline-stdout-redirect-away+stdout-dev-null-redirection",
    "+stderr-dev-null-redirection+stdin-dev-null-redirection",
    "+readonly-regular-file-stdin-redirection+volatile-stdout-regular-file-redirection",
    "+volatile-stderr-regular-file-redirection+explicit-fd1-regular-file-redirection",
    "+stdout-arbitrary-tmp-output-redirection+stderr-arbitrary-tmp-output-redirection",
    "+combined-stdin-stdout-redirection+pipeline-consumer-output-redirection",
    "+pipeline-producer-file-redirection-away+background-vfs-exec-lifecycle"
);
pub const LOCAL_COMMAND_LOOP_PROMPT: &str = "talos> ";
pub const DEFAULT_LOCAL_COMMAND_COUNT: usize = 8;
const LOCAL_COMMAND_LITERAL_ARGV_CAPACITY: usize = 4;
const LOCAL_COMMAND_LITERAL_ARG_BYTES: usize = 32;
const LOCAL_COMMAND_EXEC_PATH_BYTES: usize = LOCAL_COMMAND_LITERAL_ARG_BYTES;
const LOCAL_COMMAND_FILE_USER_BASE: u64 = 0x0000_0000_0011_0000;
const LOCAL_COMMAND_FILE_READ_OFFSET: usize = 0x40;
const LOCAL_COMMAND_FILE_USER_MEMORY_LEN: usize = 128;
const LOCAL_COMMAND_READ_ONLY_FILE_CAPACITY: usize = 2;
const LOCAL_COMMAND_STDOUT_VOLATILE_FILE_REFERENCE: usize = 0x100;
const LOCAL_COMMAND_STDERR_VOLATILE_FILE_REFERENCE: usize = 0x101;
const LOCAL_COMMAND_VOLATILE_FILE_BYTES: usize = 128;
const LOCAL_COMMAND_VOLATILE_PATH_BYTES: usize = 32;
const LOCAL_COMMAND_VOLATILE_ROUTE_BYTES: usize =
    LOCAL_COMMAND_VOLATILE_PATH_BYTES + b"volatile-vfs:".len();
const LOCAL_COMMAND_EXEC_READ_OFFSET: usize = 0x80;
const LOCAL_COMMAND_EXEC_USER_MEMORY_LEN: usize = 1024;
const LOCAL_COMMAND_STDIN_USER_BASE: u64 = 0x0000_0000_0013_0000;
const LOCAL_COMMAND_STDIN_USER_MEMORY_LEN: usize = 128;
const LOCAL_COMMAND_STDIN_READ_OFFSET: usize = 0x40;
const LOCAL_COMMAND_RUNTIME_STDIN_INPUT_BYTES: &[u8] = b"talos-console0";
#[cfg(not(test))]
const LOCAL_COMMAND_RUNTIME_STDIN_WAIT_CYCLES: usize = 1_000_000;
#[cfg(test)]
const LOCAL_COMMAND_RUNTIME_STDIN_WAIT_CYCLES: usize = 4;
const LOCAL_COMMAND_STDERR_USER_BASE: u64 = 0x0000_0000_0014_0000;
const LOCAL_COMMAND_STDERR_USER_MEMORY_LEN: usize = 128;
const LOCAL_COMMAND_EXEC_ADDRESS_SPACE_ID: u64 = 0x0010_0001;
const LOCAL_COMMAND_EXEC_PROCESS_ID: u64 = 0x0010_0001;
const LOCAL_COMMAND_BACKGROUND_JOB_ID: u64 = 0x0000_0001;
const LOCAL_COMMAND_EXEC_TEMP_DESCRIPTOR: usize = posix::STDERR_FD + 1;
const LOCAL_COMMAND_PIPE_BUFFER_LEN: usize = 128;
const LOCAL_COMMAND_PIPE_ENDPOINT_REFERENCE: usize = 1;

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
const LOCAL_COMMAND_BIN_LISTING: [(&[u8], &str); 6] = [
    (initramfs::PHASE8_INIT_PATH, "init"),
    (initramfs::PHASE10_ZERO_PATH, "zero"),
    (initramfs::PHASE10_STATUS42_PATH, "status42"),
    (initramfs::PHASE10_STDOUT_PATH, "stdout"),
    (initramfs::PHASE10_STDIN_PATH, "stdin"),
    (initramfs::PHASE10_STDERR_PATH, "stderr"),
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
    NotFound,
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
pub struct LocalCommandExecRequest {
    path: LocalCommandExecPath,
    argv: LocalCommandLiteralArgv,
    redirection: Option<LocalCommandExecRedirection>,
    stdin_redirection: Option<LocalCommandExecRedirection>,
}

impl LocalCommandExecRequest {
    fn path(&self) -> &[u8] {
        self.path.as_bytes()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalCommandPipelineRequest {
    producer: LocalCommandExecRequest,
    consumer: LocalCommandExecRequest,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalCommandPipelineSummary {
    pipe: LocalCommandPipeRecord,
    producer: LocalCommandExecSummary,
    consumer: LocalCommandExecSummary,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalCommandBackgroundExecSummary {
    exec: LocalCommandExecSummary,
    job: LocalCommandBackgroundJobRecord,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalCommandText<const N: usize> {
    bytes: [u8; N],
    len: usize,
}

impl<const N: usize> LocalCommandText<N> {
    const fn new_empty() -> Self {
        Self {
            bytes: [0; N],
            len: 0,
        }
    }

    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() > N {
            return None;
        }
        let mut text = Self::new_empty();
        text.bytes[..bytes.len()].copy_from_slice(bytes);
        text.len = bytes.len();
        Some(text)
    }

    fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocalCommandFieldText<const N: usize> {
    Static(&'static str),
    Inline(LocalCommandText<N>),
}

impl<const N: usize> LocalCommandFieldText<N> {
    const fn from_static(text: &'static str) -> Self {
        Self::Static(text)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalCommandVolatilePath {
    path: LocalCommandText<LOCAL_COMMAND_VOLATILE_PATH_BYTES>,
}

impl LocalCommandVolatilePath {
    fn from_supported_stdout_path(path: &[u8]) -> Option<Self> {
        Self::from_supported_output_path(path, b"stderr.txt")
    }

    fn from_supported_stderr_path(path: &[u8]) -> Option<Self> {
        Self::from_supported_output_path(path, b"stdout.txt")
    }

    fn from_supported_output_path(path: &[u8], reserved_basename: &[u8]) -> Option<Self> {
        const PREFIX: &[u8] = b"/tmp/";
        if !path.starts_with(PREFIX) {
            return None;
        }
        let basename = &path[PREFIX.len()..];
        if basename.is_empty()
            || basename == b"."
            || basename == b".."
            || basename == reserved_basename
            || path.len() > LOCAL_COMMAND_VOLATILE_PATH_BYTES
        {
            return None;
        }
        if !basename.iter().all(|byte| {
            matches!(
                byte,
                b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'.' | b'-' | b'_'
            )
        }) {
            return None;
        }
        Some(Self {
            path: LocalCommandText::from_bytes(path)?,
        })
    }

    fn as_bytes(&self) -> &[u8] {
        self.path.as_bytes()
    }

    fn path_text(&self) -> LocalCommandFieldText<LOCAL_COMMAND_VOLATILE_PATH_BYTES> {
        LocalCommandFieldText::Inline(self.path)
    }

    fn route_text(&self) -> LocalCommandFieldText<LOCAL_COMMAND_VOLATILE_ROUTE_BYTES> {
        let mut route = LocalCommandText::new_empty();
        const PREFIX: &[u8] = b"volatile-vfs:";
        route.bytes[..PREFIX.len()].copy_from_slice(PREFIX);
        let path = self.as_bytes();
        route.bytes[PREFIX.len()..PREFIX.len() + path.len()].copy_from_slice(path);
        route.len = PREFIX.len() + path.len();
        LocalCommandFieldText::Inline(route)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocalCommandExecRedirection {
    StdoutToStderr,
    StderrToStdout,
    CloseStdout,
    CloseStderr,
    StdoutToDevNull,
    StderrToDevNull,
    StdinFromDevNull,
    StdinFromEtcBanner,
    StdoutToTmpStdout(LocalCommandVolatilePath),
    StdoutAppendTmpStdout(LocalCommandVolatilePath),
    StderrToTmpStderr(LocalCommandVolatilePath),
    StderrAppendTmpStderr(LocalCommandVolatilePath),
}

impl LocalCommandExecRedirection {
    fn source_descriptor(self) -> usize {
        match self {
            Self::StdoutToStderr => posix::STDOUT_FD,
            Self::StderrToStdout => posix::STDERR_FD,
            Self::CloseStdout => posix::STDOUT_FD,
            Self::CloseStderr => posix::STDERR_FD,
            Self::StdoutToDevNull => posix::STDOUT_FD,
            Self::StderrToDevNull => posix::STDERR_FD,
            Self::StdinFromDevNull | Self::StdinFromEtcBanner => posix::STDIN_FD,
            Self::StdoutToTmpStdout(_) | Self::StdoutAppendTmpStdout(_) => posix::STDOUT_FD,
            Self::StderrToTmpStderr(_) | Self::StderrAppendTmpStderr(_) => posix::STDERR_FD,
        }
    }

    fn target_descriptor(self) -> Option<usize> {
        match self {
            Self::StdoutToStderr => Some(posix::STDERR_FD),
            Self::StderrToStdout => Some(posix::STDOUT_FD),
            Self::CloseStdout
            | Self::CloseStderr
            | Self::StdoutToDevNull
            | Self::StderrToDevNull
            | Self::StdinFromDevNull
            | Self::StdinFromEtcBanner
            | Self::StdoutToTmpStdout(_)
            | Self::StdoutAppendTmpStdout(_)
            | Self::StderrToTmpStderr(_)
            | Self::StderrAppendTmpStderr(_) => None,
        }
    }

    fn operation(self) -> &'static str {
        match self {
            Self::StdoutToStderr | Self::StderrToStdout => "dup",
            Self::CloseStdout | Self::CloseStderr => "close",
            Self::StdoutToDevNull
            | Self::StderrToDevNull
            | Self::StdoutToTmpStdout(_)
            | Self::StderrToTmpStderr(_) => "sink",
            Self::StdoutAppendTmpStdout(_) | Self::StderrAppendTmpStderr(_) => "append",
            Self::StdinFromDevNull | Self::StdinFromEtcBanner => "source",
        }
    }

    fn source(self) -> &'static str {
        match self {
            Self::StdoutToStderr => "shell-redirection-1-to-2",
            Self::StderrToStdout => "shell-redirection-2-to-1",
            Self::CloseStdout => "shell-redirection-1-close",
            Self::CloseStderr => "shell-redirection-2-close",
            Self::StdoutToDevNull => "shell-redirection-stdout-dev-null",
            Self::StderrToDevNull => "shell-redirection-stderr-dev-null",
            Self::StdinFromDevNull => "shell-redirection-stdin-dev-null",
            Self::StdinFromEtcBanner => "shell-redirection-stdin-etc-banner",
            Self::StdoutToTmpStdout(_) => "shell-redirection-stdout-tmp-stdout",
            Self::StdoutAppendTmpStdout(_) => "shell-redirection-stdout-tmp-stdout-append",
            Self::StderrToTmpStderr(_) => "shell-redirection-stderr-tmp-stderr",
            Self::StderrAppendTmpStderr(_) => "shell-redirection-stderr-tmp-stderr-append",
        }
    }

    fn installs_replacement_descriptor(self) -> bool {
        matches!(
            self,
            Self::StdoutToStderr
                | Self::StderrToStdout
                | Self::StdoutToDevNull
                | Self::StderrToDevNull
                | Self::StdinFromDevNull
                | Self::StdinFromEtcBanner
                | Self::StdoutToTmpStdout(_)
                | Self::StdoutAppendTmpStdout(_)
                | Self::StderrToTmpStderr(_)
                | Self::StderrAppendTmpStderr(_)
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalCommandExecPath {
    bytes: [u8; LOCAL_COMMAND_EXEC_PATH_BYTES],
    len: usize,
}

impl LocalCommandExecPath {
    const BIN_PREFIX: &'static [u8] = b"/bin/";

    const EMPTY: Self = Self {
        bytes: [0; LOCAL_COMMAND_EXEC_PATH_BYTES],
        len: 0,
    };

    fn from_absolute(path: &[u8]) -> Result<Self, LocalCommandExecError> {
        Self::from_bytes(path)
    }

    fn from_fixed_bin_name(name: &[u8]) -> Result<Self, LocalCommandExecError> {
        if name.is_empty() || name.iter().any(|byte| *byte == b'/') {
            return Err(LocalCommandExecError::InvalidPath);
        }
        let len = Self::BIN_PREFIX
            .len()
            .checked_add(name.len())
            .ok_or(LocalCommandExecError::InvalidPath)?;
        if len > LOCAL_COMMAND_EXEC_PATH_BYTES {
            return Err(LocalCommandExecError::InvalidPath);
        }
        let mut path = Self::EMPTY;
        path.bytes[..Self::BIN_PREFIX.len()].copy_from_slice(Self::BIN_PREFIX);
        path.bytes[Self::BIN_PREFIX.len()..len].copy_from_slice(name);
        path.len = len;
        Ok(path)
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, LocalCommandExecError> {
        if bytes.is_empty() || bytes.len() > LOCAL_COMMAND_EXEC_PATH_BYTES {
            return Err(LocalCommandExecError::InvalidPath);
        }
        let mut path = Self::EMPTY;
        path.bytes[..bytes.len()].copy_from_slice(bytes);
        path.len = bytes.len();
        Ok(path)
    }

    fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalCommandLiteralArg {
    bytes: [u8; LOCAL_COMMAND_LITERAL_ARG_BYTES],
    len: usize,
}

impl LocalCommandLiteralArg {
    const EMPTY: Self = Self {
        bytes: [0; LOCAL_COMMAND_LITERAL_ARG_BYTES],
        len: 0,
    };

    fn from_bytes(bytes: &[u8]) -> Result<Self, LocalCommandExecError> {
        if bytes.is_empty() || bytes.len() > LOCAL_COMMAND_LITERAL_ARG_BYTES {
            return Err(LocalCommandExecError::InvalidPath);
        }
        let mut arg = Self::EMPTY;
        arg.bytes[..bytes.len()].copy_from_slice(bytes);
        arg.len = bytes.len();
        Ok(arg)
    }

    fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }

    const fn len(self) -> usize {
        self.len
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalCommandLiteralArgv {
    args: [LocalCommandLiteralArg; LOCAL_COMMAND_LITERAL_ARGV_CAPACITY],
    argc: usize,
}

impl LocalCommandLiteralArgv {
    const fn empty() -> Self {
        Self {
            args: [LocalCommandLiteralArg::EMPTY; LOCAL_COMMAND_LITERAL_ARGV_CAPACITY],
            argc: 0,
        }
    }

    fn from_tokens(tokens: &[&[u8]]) -> Result<Self, LocalCommandExecError> {
        if tokens.is_empty() || tokens.len() > LOCAL_COMMAND_LITERAL_ARGV_CAPACITY {
            return Err(LocalCommandExecError::InvalidPath);
        }
        let mut argv = Self::empty();
        let mut index = 0;
        while index < tokens.len() {
            argv.args[index] = LocalCommandLiteralArg::from_bytes(tokens[index])?;
            index += 1;
        }
        argv.argc = tokens.len();
        Ok(argv)
    }

    const fn argc(self) -> usize {
        self.argc
    }

    fn arg(self, index: usize) -> Option<LocalCommandLiteralArg> {
        if index < self.argc {
            Some(self.args[index])
        } else {
            None
        }
    }

    fn with_resolved_argv0(mut self, argv0: &[u8]) -> Result<Self, LocalCommandExecError> {
        if self.argc == 0 {
            return Err(LocalCommandExecError::InvalidPath);
        }
        self.args[0] = LocalCommandLiteralArg::from_bytes(argv0)?;
        Ok(self)
    }

    fn copied_startup_bytes(self) -> Result<u64, LocalCommandExecError> {
        let argc =
            u64::try_from(self.argc).map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        let mut string_bytes = 0u64;
        let mut index = 0;
        while index < self.argc {
            string_bytes = string_bytes
                .checked_add(self.args[index].len() as u64)
                .and_then(|bytes| bytes.checked_add(1))
                .ok_or(LocalCommandExecError::LaunchPipelineFailed)?;
            index += 1;
        }
        8u64.checked_add(8 * argc)
            .and_then(|bytes| bytes.checked_add(8))
            .and_then(|bytes| bytes.checked_add(8))
            .and_then(|bytes| bytes.checked_add(string_bytes))
            .ok_or(LocalCommandExecError::LaunchPipelineFailed)
    }
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
    startup_argv: LocalCommandLiteralArgv,
    copied_startup_bytes: u64,
    completion_status: u64,
    completion_marker: u64,
    completion_boundary: &'static str,
    descriptor_inheritance: LocalCommandExecDescriptorInheritanceRecord,
    redirections: [Option<LocalCommandExecRedirectionRecord>; 2],
    userspace_stdout: Option<LocalCommandUserspaceStdoutRecord>,
    userspace_stdin: Option<LocalCommandUserspaceStdinRecord>,
    userspace_stderr: Option<LocalCommandUserspaceStderrRecord>,
    lifecycle: LocalCommandProcessLifecycleRecord,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalCommandExecRedirectionRecord {
    operation: &'static str,
    source_descriptor: usize,
    target_descriptor: Option<usize>,
    target_path: Option<LocalCommandFieldText<LOCAL_COMMAND_VOLATILE_PATH_BYTES>>,
    target_stream: &'static str,
    target_route: LocalCommandFieldText<LOCAL_COMMAND_VOLATILE_ROUTE_BYTES>,
    child_only: bool,
    shell_restored: bool,
    source: &'static str,
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
pub struct LocalCommandUserspaceStdoutRecord {
    descriptor: usize,
    bytes: usize,
    return_value: u64,
    stream: &'static str,
    route: LocalCommandFieldText<LOCAL_COMMAND_VOLATILE_ROUTE_BYTES>,
    source: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalCommandPipeRecord {
    id: usize,
    producer_fd: usize,
    producer_path: &'static [u8],
    consumer_fd: usize,
    consumer_path: &'static [u8],
    bytes_written: usize,
    bytes_read: usize,
    writer_closed: bool,
    reader_eof: bool,
    shell_restored: bool,
    source: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalCommandUserspaceStdinRecord {
    read_descriptor: usize,
    read_bytes: usize,
    read_return_value: u64,
    read_source: &'static str,
    stdout_descriptor: usize,
    stdout_bytes: usize,
    stdout_return_value: u64,
    source: &'static str,
    read_result: Option<&'static str>,
    readiness_observations: usize,
    scheduler_wait: Option<LocalCommandSchedulerStdinWaitRecord>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalCommandSchedulerStdinWaitRecord {
    task_id: u64,
    descriptor: usize,
    sleep_state: TaskState,
    wake_state: TaskState,
    wait_cycles: usize,
    result: &'static str,
    source: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalCommandUserspaceStderrRecord {
    descriptor: usize,
    bytes: usize,
    return_value: u64,
    stream: &'static str,
    route: LocalCommandFieldText<LOCAL_COMMAND_VOLATILE_ROUTE_BYTES>,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalCommandBackgroundJobRecord {
    job_id: u64,
    lifecycle: LocalCommandProcessLifecycleRecord,
    command_label: &'static [u8],
    state: LocalCommandBackgroundJobState,
    reaped: bool,
}

impl LocalCommandBackgroundJobRecord {
    const fn running(
        job_id: u64,
        lifecycle: LocalCommandProcessLifecycleRecord,
        command_label: &'static [u8],
    ) -> Self {
        Self {
            job_id,
            lifecycle,
            command_label,
            state: LocalCommandBackgroundJobState::Running,
            reaped: false,
        }
    }

    const fn completed_reaped(self) -> Self {
        Self {
            state: LocalCommandBackgroundJobState::Completed,
            reaped: true,
            ..self
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LocalCommandBackgroundJobState {
    Running,
    Completed,
}

impl LocalCommandBackgroundJobState {
    const fn name(self) -> &'static str {
        match self {
            Self::Running => "running",
            Self::Completed => "completed",
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

    fn read_stdout_tmp_file_via_descriptor(
        &mut self,
        _path: &[u8],
        _output: &mut [u8],
    ) -> Result<usize, LocalCommandFileReadError> {
        Err(LocalCommandFileReadError::NotSupported)
    }

    fn read_stderr_tmp_file_via_descriptor(
        &mut self,
        _path: &[u8],
        _output: &mut [u8],
    ) -> Result<usize, LocalCommandFileReadError> {
        Err(LocalCommandFileReadError::NotSupported)
    }

    fn exec_vfs_program(
        &mut self,
        _request: LocalCommandExecRequest,
    ) -> Result<LocalCommandExecSummary, LocalCommandExecError> {
        Err(LocalCommandExecError::NotSupported)
    }

    fn exec_vfs_program_with_policy(
        &mut self,
        request: LocalCommandExecRequest,
        _allow_stdin_stdout_file_redirection: bool,
    ) -> Result<LocalCommandExecSummary, LocalCommandExecError> {
        self.exec_vfs_program(request)
    }

    fn exec_vfs_pipeline(
        &mut self,
        _request: LocalCommandPipelineRequest,
    ) -> Result<LocalCommandPipelineSummary, LocalCommandExecError> {
        Err(LocalCommandExecError::NotSupported)
    }

    fn exec_background_vfs_program(
        &mut self,
        _request: LocalCommandExecRequest,
    ) -> Result<LocalCommandBackgroundExecSummary, LocalCommandExecError> {
        Err(LocalCommandExecError::NotSupported)
    }

    fn poll_background_job_completion(&mut self) -> Option<LocalCommandBackgroundJobRecord> {
        None
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
    read_only_files: initramfs::ReadOnlyFileDescriptions<LOCAL_COMMAND_READ_ONLY_FILE_CAPACITY>,
    last_process: Option<LocalCommandProcessLifecycleRecord>,
    waitable_process: Option<LocalCommandProcessLifecycleRecord>,
    background_job: Option<LocalCommandBackgroundJobRecord>,
    pipe: LocalCommandPipeState,
    stdout_scratch_file: LocalCommandVolatileFileState,
    stderr_scratch_file: LocalCommandVolatileFileState,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct LocalCommandPipeState {
    bytes: [u8; LOCAL_COMMAND_PIPE_BUFFER_LEN],
    len: usize,
    cursor: usize,
    writer_open: bool,
    reader_open: bool,
    eof_observed: bool,
}

impl LocalCommandPipeState {
    const fn new_empty() -> Self {
        Self {
            bytes: [0; LOCAL_COMMAND_PIPE_BUFFER_LEN],
            len: 0,
            cursor: 0,
            writer_open: false,
            reader_open: false,
            eof_observed: false,
        }
    }

    fn reset(&mut self) {
        *self = Self::new_empty();
    }

    fn open_writer(&mut self) {
        self.writer_open = true;
        self.eof_observed = false;
    }

    fn close_writer(&mut self) {
        self.writer_open = false;
    }

    fn open_reader(&mut self) {
        self.reader_open = true;
    }

    fn close_reader(&mut self) {
        self.reader_open = false;
    }

    fn write(&mut self, bytes: &[u8]) -> Result<usize, LocalCommandExecError> {
        if !self.writer_open {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        let end = self
            .len
            .checked_add(bytes.len())
            .ok_or(LocalCommandExecError::LaunchPipelineFailed)?;
        if end > self.bytes.len() {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        self.bytes[self.len..end].copy_from_slice(bytes);
        self.len = end;
        Ok(bytes.len())
    }

    fn read(&mut self, output: &mut [u8]) -> usize {
        if !self.reader_open || output.is_empty() {
            return 0;
        }
        let remaining = self.len - self.cursor;
        let selected = core::cmp::min(output.len(), remaining);
        if selected == 0 {
            if !self.writer_open {
                self.eof_observed = true;
            }
            return 0;
        }
        let end = self.cursor + selected;
        output[..selected].copy_from_slice(&self.bytes[self.cursor..end]);
        self.cursor = end;
        selected
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct LocalCommandVolatileFileState {
    exists: bool,
    path: LocalCommandVolatilePath,
    bytes: [u8; LOCAL_COMMAND_VOLATILE_FILE_BYTES],
    len: usize,
}

impl LocalCommandVolatileFileState {
    const fn new_empty() -> Self {
        Self {
            exists: false,
            path: LocalCommandVolatilePath {
                path: LocalCommandText::new_empty(),
            },
            bytes: [0; LOCAL_COMMAND_VOLATILE_FILE_BYTES],
            len: 0,
        }
    }

    fn truncate_create(&mut self, path: LocalCommandVolatilePath) {
        self.exists = true;
        self.path = path;
        self.len = 0;
    }

    fn create_if_missing(&mut self, path: LocalCommandVolatilePath) {
        if self.path != path {
            self.exists = false;
            self.len = 0;
            self.path = path;
        }
        self.exists = true;
    }

    fn write(&mut self, bytes: &[u8]) -> Result<usize, LocalCommandExecError> {
        let end = self
            .len
            .checked_add(bytes.len())
            .ok_or(LocalCommandExecError::LaunchPipelineFailed)?;
        if end > self.bytes.len() {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        self.bytes[self.len..end].copy_from_slice(bytes);
        self.len = end;
        Ok(bytes.len())
    }

    fn read(
        &self,
        path: LocalCommandVolatilePath,
        output: &mut [u8],
    ) -> Result<usize, LocalCommandFileReadError> {
        if !self.exists || self.path != path {
            return Err(LocalCommandFileReadError::NotFound);
        }
        let selected = core::cmp::min(output.len(), self.len);
        output[..selected].copy_from_slice(&self.bytes[..selected]);
        Ok(selected)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LocalCommandVolatileFileTarget {
    Stdout(LocalCommandVolatilePath),
    Stderr(LocalCommandVolatilePath),
}

impl LocalCommandVolatileFileTarget {
    fn reference(self) -> usize {
        match self {
            Self::Stdout(_) => LOCAL_COMMAND_STDOUT_VOLATILE_FILE_REFERENCE,
            Self::Stderr(_) => LOCAL_COMMAND_STDERR_VOLATILE_FILE_REFERENCE,
        }
    }

    fn path_text(self) -> LocalCommandFieldText<LOCAL_COMMAND_VOLATILE_PATH_BYTES> {
        match self {
            Self::Stdout(path) => path.path_text(),
            Self::Stderr(path) => path.path_text(),
        }
    }

    fn route_text(self) -> LocalCommandFieldText<LOCAL_COMMAND_VOLATILE_ROUTE_BYTES> {
        match self {
            Self::Stdout(path) => path.route_text(),
            Self::Stderr(path) => path.route_text(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct AppliedLocalCommandExecRedirection {
    request: LocalCommandExecRedirection,
    restored_entry: posix::DescriptorEntry,
    record: LocalCommandExecRedirectionRecord,
}

impl AppliedLocalCommandExecRedirection {
    const fn restored_record(self) -> LocalCommandExecRedirectionRecord {
        LocalCommandExecRedirectionRecord {
            shell_restored: true,
            ..self.record
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct AppliedLocalCommandExecRedirections {
    entries: [Option<AppliedLocalCommandExecRedirection>; 2],
}

impl AppliedLocalCommandExecRedirections {
    fn records(self) -> [Option<LocalCommandExecRedirectionRecord>; 2] {
        [
            self.entries[0].map(|applied| applied.restored_record()),
            self.entries[1].map(|applied| applied.restored_record()),
        ]
    }
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
            background_job: None,
            pipe: LocalCommandPipeState::new_empty(),
            stdout_scratch_file: LocalCommandVolatileFileState::new_empty(),
            stderr_scratch_file: LocalCommandVolatileFileState::new_empty(),
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

struct RuntimeStdinReadBackend<'a, I> {
    ready_byte: &'a mut Option<u8>,
    backend: &'a mut I,
}

impl<I> ConsoleInputBackend for RuntimeStdinReadBackend<'_, I>
where
    I: ConsoleInputBackend,
{
    fn poll_read_byte(&mut self) -> Option<u8> {
        self.ready_byte
            .take()
            .or_else(|| self.backend.poll_read_byte())
    }
}

struct LocalCommandRuntimeStdinWait {
    task_id: TaskId,
    descriptor: usize,
    state: TaskState,
    wait_cycles: usize,
}

impl LocalCommandRuntimeStdinWait {
    const SOURCE: &'static str = "scheduler-runtime-console-readiness";

    const fn new(task_id: TaskId, descriptor: usize) -> Self {
        Self {
            task_id,
            descriptor,
            state: TaskState::Running,
            wait_cycles: 0,
        }
    }

    fn sleep(&mut self) -> LocalCommandSchedulerStdinWaitRecord {
        self.state = TaskState::Blocked;
        self.record("sleep")
    }

    fn observe_wait_cycle(&mut self) -> Result<(), LocalCommandExecError> {
        self.wait_cycles = self
            .wait_cycles
            .checked_add(1)
            .ok_or(LocalCommandExecError::LaunchPipelineFailed)?;
        Ok(())
    }

    fn wake(&mut self) -> LocalCommandSchedulerStdinWaitRecord {
        self.state = TaskState::Runnable;
        self.record("wakeup/resume")
    }

    fn cancel_no_data(&mut self) -> LocalCommandSchedulerStdinWaitRecord {
        self.state = TaskState::Runnable;
        self.record("timeout/no-false-eof")
    }

    const fn record(&self, result: &'static str) -> LocalCommandSchedulerStdinWaitRecord {
        LocalCommandSchedulerStdinWaitRecord {
            task_id: self.task_id.raw(),
            descriptor: self.descriptor,
            sleep_state: TaskState::Blocked,
            wake_state: self.state,
            wait_cycles: self.wait_cycles,
            result,
            source: Self::SOURCE,
        }
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

    fn read_stdout_tmp_file_via_descriptor(
        &mut self,
        path: &[u8],
        output: &mut [u8],
    ) -> Result<usize, LocalCommandFileReadError> {
        let path = LocalCommandVolatilePath::from_supported_stdout_path(path)
            .ok_or(LocalCommandFileReadError::NotSupported)?;
        self.read_scratch_file_descriptor(
            LOCAL_COMMAND_STDOUT_VOLATILE_FILE_REFERENCE,
            LocalCommandVolatileFileTarget::Stdout(path),
            output,
        )
    }

    fn read_stderr_tmp_file_via_descriptor(
        &mut self,
        path: &[u8],
        output: &mut [u8],
    ) -> Result<usize, LocalCommandFileReadError> {
        let path = LocalCommandVolatilePath::from_supported_stderr_path(path)
            .ok_or(LocalCommandFileReadError::NotSupported)?;
        self.read_scratch_file_descriptor(
            LOCAL_COMMAND_STDERR_VOLATILE_FILE_REFERENCE,
            LocalCommandVolatileFileTarget::Stderr(path),
            output,
        )
    }

    fn exec_vfs_program(
        &mut self,
        request: LocalCommandExecRequest,
    ) -> Result<LocalCommandExecSummary, LocalCommandExecError> {
        self.exec_vfs_program_with_policy(request, false)
    }

    fn exec_vfs_program_with_policy(
        &mut self,
        request: LocalCommandExecRequest,
        allow_stdin_stdout_file_redirection: bool,
    ) -> Result<LocalCommandExecSummary, LocalCommandExecError> {
        let path = request.path();
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
            initramfs::PHASE10_STDOUT_PATH => initramfs::PHASE10_STDOUT_PATH,
            initramfs::PHASE10_STDIN_PATH => initramfs::PHASE10_STDIN_PATH,
            initramfs::PHASE10_STDERR_PATH => initramfs::PHASE10_STDERR_PATH,
            initramfs::PHASE8_BANNER_PATH => initramfs::PHASE8_BANNER_PATH,
            initramfs::PHASE8_EMPTY_PATH => initramfs::PHASE8_EMPTY_PATH,
            initramfs::PHASE8_NESTED_PATH => initramfs::PHASE8_NESTED_PATH,
            _ => return Err(LocalCommandExecError::NotExecutable),
        };
        if matches!(
            request.stdin_redirection,
            Some(
                LocalCommandExecRedirection::StdinFromDevNull
                    | LocalCommandExecRedirection::StdinFromEtcBanner
            )
        ) && source_path != initramfs::PHASE10_STDIN_PATH
        {
            return Err(LocalCommandExecError::InvalidPath);
        }
        let allowed_stdin_stdout_file_redirection = allow_stdin_stdout_file_redirection
            && source_path == initramfs::PHASE10_STDIN_PATH
            && request.stdin_redirection.is_none()
            && matches!(
                request.redirection,
                Some(LocalCommandExecRedirection::StdoutToTmpStdout(_))
            );
        if matches!(
            request.redirection,
            Some(
                LocalCommandExecRedirection::StdoutToTmpStdout(_)
                    | LocalCommandExecRedirection::StdoutAppendTmpStdout(_)
            )
        ) && source_path != initramfs::PHASE10_STDOUT_PATH
            && !(source_path == initramfs::PHASE10_STDIN_PATH
                && matches!(
                    request.stdin_redirection,
                    Some(LocalCommandExecRedirection::StdinFromEtcBanner)
                )
                && matches!(
                    request.redirection,
                    Some(LocalCommandExecRedirection::StdoutToTmpStdout(_))
                ))
            && !allowed_stdin_stdout_file_redirection
        {
            return Err(LocalCommandExecError::InvalidPath);
        }
        if matches!(
            request.redirection,
            Some(
                LocalCommandExecRedirection::StderrToTmpStderr(_)
                    | LocalCommandExecRedirection::StderrAppendTmpStderr(_)
            )
        ) && source_path != initramfs::PHASE10_STDERR_PATH
        {
            return Err(LocalCommandExecError::InvalidPath);
        }
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
        let copied_startup_bytes = request.argv.copied_startup_bytes()?;
        let stack_plan = initial_user_stack::plan_initial_user_stack_with_startup_payload(
            image,
            install_plan,
            address_space,
            materialization,
            launch_plan,
            InitialUserStackRequest::PlanOnly,
            &mut stack_source,
            request.argv.argc(),
            copied_startup_bytes,
        )
        .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        let applied_redirections =
            self.apply_exec_redirections(request.stdin_redirection, request.redirection)?;
        let descriptor_inheritance = self
            .standard_descriptor_inheritance_record(owner, request.redirection)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        let userspace_stdout = self.emit_userspace_stdout_fixture(source_path)?;
        let userspace_stdin = self.emit_userspace_stdin_fixture(source_path)?;
        let userspace_stderr = self.emit_userspace_stderr_fixture(source_path)?;
        let redirections = self.restore_exec_redirections(applied_redirections)?;
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
            startup_argv: request.argv,
            copied_startup_bytes: stack_plan.startup_payload().copied_startup_bytes(),
            completion_status,
            completion_marker: initramfs::PHASE8_INIT_SVC_MARKER,
            completion_boundary: "lower-aarch64-svc-status-equivalent",
            descriptor_inheritance,
            redirections,
            userspace_stdout,
            userspace_stdin,
            userspace_stderr,
            lifecycle,
        })
    }

    fn last_process_lifecycle_record(&self) -> Option<LocalCommandProcessLifecycleRecord> {
        self.last_process
    }

    fn wait_process_lifecycle_record(&mut self) -> Option<LocalCommandProcessLifecycleRecord> {
        self.waitable_process.take()
    }

    fn exec_vfs_pipeline(
        &mut self,
        request: LocalCommandPipelineRequest,
    ) -> Result<LocalCommandPipelineSummary, LocalCommandExecError> {
        let producer_redirection_supported = match (
            request.producer.path(),
            request.producer.stdin_redirection,
            request.producer.redirection,
        ) {
            (
                initramfs::PHASE10_STDOUT_PATH,
                None,
                None
                | Some(LocalCommandExecRedirection::StdoutToStderr)
                | Some(LocalCommandExecRedirection::StdoutToTmpStdout(_)),
            ) => true,
            (
                initramfs::PHASE10_STDERR_PATH,
                None,
                None | Some(LocalCommandExecRedirection::StderrToStdout),
            ) => true,
            _ => false,
        };
        let consumer_output_redirection_supported = request.producer.path()
            == initramfs::PHASE10_STDOUT_PATH
            && request.producer.stdin_redirection.is_none()
            && request.producer.redirection.is_none()
            && request.consumer.path() == initramfs::PHASE10_STDIN_PATH
            && request.consumer.stdin_redirection.is_none()
            && matches!(
                request.consumer.redirection,
                Some(LocalCommandExecRedirection::StdoutToTmpStdout(_))
            );
        if !producer_redirection_supported
            || request.consumer.path() != initramfs::PHASE10_STDIN_PATH
            || request.consumer.stdin_redirection.is_some()
            || (request.consumer.redirection.is_some() && !consumer_output_redirection_supported)
        {
            return Err(LocalCommandExecError::InvalidPath);
        }

        self.pipe.reset();
        let stdout_restore =
            self.install_pipe_endpoint(posix::STDOUT_FD, posix::DescriptorAccess::WriteOnly)?;
        self.pipe.open_writer();
        let producer = self.exec_vfs_program(request.producer);
        self.pipe.close_writer();
        self.restore_pipe_endpoint(posix::STDOUT_FD, stdout_restore)?;
        let producer = producer?;

        let stdin_restore =
            self.install_pipe_endpoint(posix::STDIN_FD, posix::DescriptorAccess::ReadOnly)?;
        self.pipe.open_reader();
        let consumer = self
            .exec_vfs_program_with_policy(request.consumer, consumer_output_redirection_supported);
        self.pipe.close_reader();
        self.restore_pipe_endpoint(posix::STDIN_FD, stdin_restore)?;
        let consumer = consumer?;
        let pipe_source = match (request.producer.redirection, request.consumer.redirection) {
            (None, Some(LocalCommandExecRedirection::StdoutToTmpStdout(_))) => {
                "shell-pipe-consumer-stdout-redirection"
            }
            (Some(LocalCommandExecRedirection::StdoutToTmpStdout(_)), None) => {
                "shell-pipe-producer-file-redirection-away"
            }
            (Some(LocalCommandExecRedirection::StderrToStdout), None) => {
                "shell-pipe-stderr-dup-to-stdout"
            }
            (Some(LocalCommandExecRedirection::StdoutToStderr), None) => {
                "shell-pipe-stdout-redirect-away"
            }
            (None, None) if producer.source_path == initramfs::PHASE10_STDERR_PATH => {
                "shell-pipe-stdout-only-stderr-not-piped"
            }
            (None, None) => "shell-pipe-stdout-to-stdin",
            _ => return Err(LocalCommandExecError::InvalidPath),
        };

        Ok(LocalCommandPipelineSummary {
            pipe: LocalCommandPipeRecord {
                id: LOCAL_COMMAND_PIPE_ENDPOINT_REFERENCE,
                producer_fd: posix::STDOUT_FD,
                producer_path: producer.source_path,
                consumer_fd: posix::STDIN_FD,
                consumer_path: consumer.source_path,
                bytes_written: self.pipe.len,
                bytes_read: self.pipe.cursor,
                writer_closed: !self.pipe.writer_open,
                reader_eof: self.pipe.eof_observed,
                shell_restored: self.shell_standard_descriptors_restored()?,
                source: pipe_source,
            },
            producer,
            consumer,
        })
    }

    fn exec_background_vfs_program(
        &mut self,
        request: LocalCommandExecRequest,
    ) -> Result<LocalCommandBackgroundExecSummary, LocalCommandExecError> {
        if request.path() != initramfs::PHASE10_STATUS42_PATH
            || request.redirection.is_some()
            || request.stdin_redirection.is_some()
            || self.background_job.is_some_and(|job| {
                job.state == LocalCommandBackgroundJobState::Running && !job.reaped
            })
        {
            return Err(LocalCommandExecError::InvalidPath);
        }

        let previous_last = self.last_process;
        let previous_waitable = self.waitable_process;
        let exec = self.exec_vfs_program(request)?;
        self.last_process = previous_last;
        self.waitable_process = previous_waitable;
        let job = LocalCommandBackgroundJobRecord::running(
            LOCAL_COMMAND_BACKGROUND_JOB_ID,
            exec.lifecycle,
            exec.source_path,
        );
        self.background_job = Some(job);
        Ok(LocalCommandBackgroundExecSummary { exec, job })
    }

    fn poll_background_job_completion(&mut self) -> Option<LocalCommandBackgroundJobRecord> {
        let job = self.background_job?;
        if job.state != LocalCommandBackgroundJobState::Running || job.reaped {
            return None;
        }
        let completed = job.completed_reaped();
        self.background_job = Some(completed);
        Some(completed)
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
        if entry.object().kind() == posix::DescriptorObjectKind::RegularFile
            && entry.object().reference() != LOCAL_COMMAND_STDOUT_VOLATILE_FILE_REFERENCE
            && entry.object().reference() != LOCAL_COMMAND_STDERR_VOLATILE_FILE_REFERENCE
        {
            self.read_only_files
                .remove(entry.object().reference())
                .map_err(|_| LocalCommandFileReadError::SyscallFailed)?;
        }
        Ok(())
    }

    fn open_stdout_scratch_redirection(
        &mut self,
        path: LocalCommandVolatilePath,
    ) -> Result<(), LocalCommandExecError> {
        self.open_scratch_redirection(
            posix::STDOUT_FD,
            LocalCommandVolatileFileTarget::Stdout(path),
            true,
        )
    }

    fn open_stdout_scratch_append_redirection(
        &mut self,
        path: LocalCommandVolatilePath,
    ) -> Result<(), LocalCommandExecError> {
        self.open_scratch_redirection(
            posix::STDOUT_FD,
            LocalCommandVolatileFileTarget::Stdout(path),
            false,
        )
    }

    fn open_stderr_scratch_redirection(
        &mut self,
        path: LocalCommandVolatilePath,
    ) -> Result<(), LocalCommandExecError> {
        self.open_scratch_redirection(
            posix::STDERR_FD,
            LocalCommandVolatileFileTarget::Stderr(path),
            true,
        )
    }

    fn open_stderr_scratch_append_redirection(
        &mut self,
        path: LocalCommandVolatilePath,
    ) -> Result<(), LocalCommandExecError> {
        self.open_scratch_redirection(
            posix::STDERR_FD,
            LocalCommandVolatileFileTarget::Stderr(path),
            false,
        )
    }

    fn open_scratch_redirection(
        &mut self,
        descriptor: usize,
        target: LocalCommandVolatileFileTarget,
        truncate: bool,
    ) -> Result<(), LocalCommandExecError> {
        let table = self
            .descriptor_store
            .current_descriptor_table_mut(self.current_owner)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        let entry = posix::DescriptorEntry::new(
            posix::DescriptorAccess::WriteOnly,
            posix::DescriptorFlags::EMPTY,
            posix::DescriptorObject::new(
                posix::DescriptorObjectKind::RegularFile,
                target.reference(),
            ),
        );
        table
            .allocate_at(descriptor, entry)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        if truncate {
            match target {
                LocalCommandVolatileFileTarget::Stdout(path) => {
                    self.stdout_scratch_file.truncate_create(path)
                }
                LocalCommandVolatileFileTarget::Stderr(path) => {
                    self.stderr_scratch_file.truncate_create(path)
                }
            }
        } else {
            match target {
                LocalCommandVolatileFileTarget::Stdout(path) => {
                    self.stdout_scratch_file.create_if_missing(path)
                }
                LocalCommandVolatileFileTarget::Stderr(path) => {
                    self.stderr_scratch_file.create_if_missing(path)
                }
            }
        }
        Ok(())
    }

    fn read_scratch_file_descriptor(
        &mut self,
        reference: usize,
        target: LocalCommandVolatileFileTarget,
        output: &mut [u8],
    ) -> Result<usize, LocalCommandFileReadError> {
        let exists = match target {
            LocalCommandVolatileFileTarget::Stdout(path) => {
                self.stdout_scratch_file.exists && self.stdout_scratch_file.path == path
            }
            LocalCommandVolatileFileTarget::Stderr(path) => {
                self.stderr_scratch_file.exists && self.stderr_scratch_file.path == path
            }
        };
        if !exists {
            return Err(LocalCommandFileReadError::NotFound);
        }
        let table = self
            .descriptor_store
            .current_descriptor_table_mut(self.current_owner)
            .map_err(|_| LocalCommandFileReadError::SyscallFailed)?;
        let entry = posix::DescriptorEntry::new(
            posix::DescriptorAccess::ReadOnly,
            posix::DescriptorFlags::EMPTY,
            posix::DescriptorObject::new(posix::DescriptorObjectKind::RegularFile, reference),
        );
        let descriptor = table
            .allocate(entry)
            .map_err(|_| LocalCommandFileReadError::SyscallFailed)?;
        let bytes = match target {
            LocalCommandVolatileFileTarget::Stdout(path) => {
                self.stdout_scratch_file.read(path, output)
            }
            LocalCommandVolatileFileTarget::Stderr(path) => {
                self.stderr_scratch_file.read(path, output)
            }
        };
        let cleanup = self.close_regular_file_descriptor(descriptor);
        match (bytes, cleanup) {
            (Ok(bytes), Ok(())) => Ok(bytes),
            _ => Err(LocalCommandFileReadError::SyscallFailed),
        }
    }

    fn open_readonly_stdin_redirection(
        &mut self,
        path: &[u8],
    ) -> Result<(), LocalCommandExecError> {
        if path.len() > LOCAL_COMMAND_FILE_READ_OFFSET {
            return Err(LocalCommandExecError::LaunchPipelineFailed);
        }

        let mut user_memory = [0u8; LOCAL_COMMAND_FILE_USER_MEMORY_LEN];
        let mut scratch = [0u8; LOCAL_COMMAND_FILE_USER_MEMORY_LEN];
        user_memory[..path.len()].copy_from_slice(path);
        let mappings = [posix::UserMapping::new(
            LOCAL_COMMAND_FILE_USER_BASE,
            user_memory.len(),
            posix::UserMappingPermissions::USER_DATA,
        )
        .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?];
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
        let descriptor = syscall_success_usize(open.return_value().x0())
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        if descriptor != posix::STDIN_FD {
            let _ = self.close_regular_file_descriptor(descriptor);
            return Err(LocalCommandExecError::LaunchPipelineFailed);
        }

        Ok(())
    }

    fn close_replacement_descriptor(
        &mut self,
        descriptor: usize,
    ) -> Result<(), LocalCommandExecError> {
        let table = self
            .descriptor_store
            .current_descriptor_table(self.current_owner)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        let entry = table
            .get(descriptor)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        if entry.object().kind() == posix::DescriptorObjectKind::RegularFile {
            return self
                .close_regular_file_descriptor(descriptor)
                .map_err(|_| LocalCommandExecError::LaunchPipelineFailed);
        }

        self.descriptor_store
            .close_current_descriptor(self.current_owner, descriptor)
            .map(|_| ())
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)
    }

    fn apply_exec_redirection(
        &mut self,
        redirection: Option<LocalCommandExecRedirection>,
    ) -> Result<Option<AppliedLocalCommandExecRedirection>, LocalCommandExecError> {
        let Some(redirection) = redirection else {
            return Ok(None);
        };
        let table = self
            .descriptor_store
            .current_descriptor_table_mut(self.current_owner)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        let source_descriptor = redirection.source_descriptor();
        let target_descriptor = redirection.target_descriptor();
        let restored_entry = table
            .get(source_descriptor)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        match redirection {
            LocalCommandExecRedirection::StdinFromDevNull
            | LocalCommandExecRedirection::StdinFromEtcBanner => {
                if restored_entry.require_readable().is_err()
                    || !matches!(
                        restored_entry.object().kind(),
                        posix::DescriptorObjectKind::StdioInput
                            | posix::DescriptorObjectKind::PipeEndpoint
                    )
                {
                    return Err(LocalCommandExecError::LaunchPipelineFailed);
                }
            }
            _ => {
                if restored_entry.require_writable().is_err()
                    || !matches!(
                        restored_entry.object().kind(),
                        posix::DescriptorObjectKind::StdioOutput
                            | posix::DescriptorObjectKind::PipeEndpoint
                    )
                {
                    return Err(LocalCommandExecError::LaunchPipelineFailed);
                }
            }
        }
        let (target_entry, target_path, target_stream, target_route) = if matches!(
            redirection,
            LocalCommandExecRedirection::StdoutToDevNull
                | LocalCommandExecRedirection::StderrToDevNull
                | LocalCommandExecRedirection::StdinFromDevNull
        ) {
            (
                Some(posix::DescriptorEntry::new(
                    match redirection {
                        LocalCommandExecRedirection::StdinFromDevNull => {
                            posix::DescriptorAccess::ReadOnly
                        }
                        _ => posix::DescriptorAccess::WriteOnly,
                    },
                    posix::DescriptorFlags::EMPTY,
                    posix::DescriptorObject::new(
                        posix::DescriptorObjectKind::Device,
                        posix::DEV_NULL_REFERENCE,
                    ),
                )),
                Some(LocalCommandFieldText::from_static("/dev/null")),
                match redirection {
                    LocalCommandExecRedirection::StdinFromDevNull => "null-source",
                    _ => "null-sink",
                },
                LocalCommandFieldText::from_static("device:/dev/null"),
            )
        } else if let LocalCommandExecRedirection::StdoutToTmpStdout(path)
        | LocalCommandExecRedirection::StdoutAppendTmpStdout(path) = redirection
        {
            (
                None,
                Some(LocalCommandVolatileFileTarget::Stdout(path).path_text()),
                "regular-file",
                LocalCommandVolatileFileTarget::Stdout(path).route_text(),
            )
        } else if let LocalCommandExecRedirection::StderrToTmpStderr(path)
        | LocalCommandExecRedirection::StderrAppendTmpStderr(path) = redirection
        {
            (
                None,
                Some(LocalCommandVolatileFileTarget::Stderr(path).path_text()),
                "regular-file",
                LocalCommandVolatileFileTarget::Stderr(path).route_text(),
            )
        } else if redirection == LocalCommandExecRedirection::StdinFromEtcBanner {
            (
                None,
                Some(LocalCommandFieldText::from_static("/etc/banner.txt")),
                "regular-file",
                LocalCommandFieldText::from_static("initramfs:/etc/banner.txt"),
            )
        } else if let Some(target_descriptor) = target_descriptor {
            let target_entry = table
                .get(target_descriptor)
                .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
            if target_entry.require_writable().is_err()
                || !matches!(
                    target_entry.object().kind(),
                    posix::DescriptorObjectKind::StdioOutput
                        | posix::DescriptorObjectKind::PipeEndpoint
                )
            {
                return Err(LocalCommandExecError::LaunchPipelineFailed);
            }
            let (target_stream, target_route) = match target_entry.object().kind() {
                posix::DescriptorObjectKind::StdioOutput => (
                    target_entry.object().stdio_stream_name(),
                    target_entry.object().runtime_console_route_name(),
                ),
                posix::DescriptorObjectKind::PipeEndpoint => {
                    ("pipe-writer", "pipe:stdout-to-stdin")
                }
                _ => return Err(LocalCommandExecError::LaunchPipelineFailed),
            };
            (
                Some(target_entry),
                None,
                target_stream,
                LocalCommandFieldText::from_static(target_route),
            )
        } else {
            (
                None,
                None,
                "closed",
                LocalCommandFieldText::from_static("closed-descriptor"),
            )
        };

        table
            .close(source_descriptor)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        let install_regular_stdin = redirection == LocalCommandExecRedirection::StdinFromEtcBanner;
        let install_stdout_scratch = match redirection {
            LocalCommandExecRedirection::StdoutToTmpStdout(path) => Some(path),
            _ => None,
        };
        let install_stdout_scratch_append = match redirection {
            LocalCommandExecRedirection::StdoutAppendTmpStdout(path) => Some(path),
            _ => None,
        };
        let install_stderr_scratch = match redirection {
            LocalCommandExecRedirection::StderrToTmpStderr(path) => Some(path),
            _ => None,
        };
        let install_stderr_scratch_append = match redirection {
            LocalCommandExecRedirection::StderrAppendTmpStderr(path) => Some(path),
            _ => None,
        };
        if let Some(target_entry) = target_entry {
            if table.allocate_at(source_descriptor, target_entry).is_err() {
                let _ = table.allocate_at(source_descriptor, restored_entry);
                return Err(LocalCommandExecError::LaunchPipelineFailed);
            }
        } else if install_regular_stdin {
            let _ = table;
            if self
                .open_readonly_stdin_redirection(initramfs::PHASE8_BANNER_PATH)
                .is_err()
            {
                let restore_table = self
                    .descriptor_store
                    .current_descriptor_table_mut(self.current_owner)
                    .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
                let _ = restore_table.allocate_at(source_descriptor, restored_entry);
                return Err(LocalCommandExecError::LaunchPipelineFailed);
            }
        } else if let Some(path) = install_stdout_scratch {
            let _ = table;
            if self.open_stdout_scratch_redirection(path).is_err() {
                let restore_table = self
                    .descriptor_store
                    .current_descriptor_table_mut(self.current_owner)
                    .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
                let _ = restore_table.allocate_at(source_descriptor, restored_entry);
                return Err(LocalCommandExecError::LaunchPipelineFailed);
            }
        } else if let Some(path) = install_stdout_scratch_append {
            let _ = table;
            if self.open_stdout_scratch_append_redirection(path).is_err() {
                let restore_table = self
                    .descriptor_store
                    .current_descriptor_table_mut(self.current_owner)
                    .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
                let _ = restore_table.allocate_at(source_descriptor, restored_entry);
                return Err(LocalCommandExecError::LaunchPipelineFailed);
            }
        } else if let Some(path) = install_stderr_scratch {
            let _ = table;
            if self.open_stderr_scratch_redirection(path).is_err() {
                let restore_table = self
                    .descriptor_store
                    .current_descriptor_table_mut(self.current_owner)
                    .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
                let _ = restore_table.allocate_at(source_descriptor, restored_entry);
                return Err(LocalCommandExecError::LaunchPipelineFailed);
            }
        } else if let Some(path) = install_stderr_scratch_append {
            let _ = table;
            if self.open_stderr_scratch_append_redirection(path).is_err() {
                let restore_table = self
                    .descriptor_store
                    .current_descriptor_table_mut(self.current_owner)
                    .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
                let _ = restore_table.allocate_at(source_descriptor, restored_entry);
                return Err(LocalCommandExecError::LaunchPipelineFailed);
            }
        }

        Ok(Some(AppliedLocalCommandExecRedirection {
            request: redirection,
            restored_entry,
            record: LocalCommandExecRedirectionRecord {
                operation: redirection.operation(),
                source_descriptor,
                target_descriptor,
                target_path,
                target_stream,
                target_route,
                child_only: true,
                shell_restored: false,
                source: redirection.source(),
            },
        }))
    }

    fn restore_exec_redirection(
        &mut self,
        applied: AppliedLocalCommandExecRedirection,
    ) -> Result<(), LocalCommandExecError> {
        let descriptor = applied.request.source_descriptor();
        if applied.request.installs_replacement_descriptor() {
            self.close_replacement_descriptor(descriptor)?;
        }
        let table = self
            .descriptor_store
            .current_descriptor_table_mut(self.current_owner)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        table
            .allocate_at(descriptor, applied.restored_entry)
            .map(|_| ())
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)
    }

    fn apply_exec_redirections(
        &mut self,
        stdin_redirection: Option<LocalCommandExecRedirection>,
        redirection: Option<LocalCommandExecRedirection>,
    ) -> Result<AppliedLocalCommandExecRedirections, LocalCommandExecError> {
        let first = self.apply_exec_redirection(stdin_redirection)?;
        let second = match self.apply_exec_redirection(redirection) {
            Ok(applied) => applied,
            Err(error) => {
                if let Some(applied) = first {
                    let _ = self.restore_exec_redirection(applied);
                }
                return Err(error);
            }
        };
        Ok(AppliedLocalCommandExecRedirections {
            entries: [first, second],
        })
    }

    fn restore_exec_redirections(
        &mut self,
        applied: AppliedLocalCommandExecRedirections,
    ) -> Result<[Option<LocalCommandExecRedirectionRecord>; 2], LocalCommandExecError> {
        if let Some(second) = applied.entries[1] {
            self.restore_exec_redirection(second)?;
        }
        if let Some(first) = applied.entries[0] {
            self.restore_exec_redirection(first)?;
        }
        Ok(applied.records())
    }

    fn install_pipe_endpoint(
        &mut self,
        descriptor: usize,
        access: posix::DescriptorAccess,
    ) -> Result<posix::DescriptorEntry, LocalCommandExecError> {
        let table = self
            .descriptor_store
            .current_descriptor_table_mut(self.current_owner)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        let restored = table
            .close(descriptor)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        let pipe = posix::DescriptorEntry::new(
            access,
            posix::DescriptorFlags::EMPTY,
            posix::DescriptorObject::new(
                posix::DescriptorObjectKind::PipeEndpoint,
                LOCAL_COMMAND_PIPE_ENDPOINT_REFERENCE,
            ),
        );
        if table.allocate_at(descriptor, pipe).is_err() {
            let _ = table.allocate_at(descriptor, restored);
            return Err(LocalCommandExecError::LaunchPipelineFailed);
        }
        Ok(restored)
    }

    fn restore_pipe_endpoint(
        &mut self,
        descriptor: usize,
        restored: posix::DescriptorEntry,
    ) -> Result<(), LocalCommandExecError> {
        let table = self
            .descriptor_store
            .current_descriptor_table_mut(self.current_owner)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        table
            .close(descriptor)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        table
            .allocate_at(descriptor, restored)
            .map(|_| ())
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)
    }

    fn shell_standard_descriptors_restored(&self) -> Result<bool, LocalCommandExecError> {
        let table = self
            .descriptor_store
            .current_descriptor_table(self.current_owner)
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
        Ok(
            stdin.object().kind() == posix::DescriptorObjectKind::StdioInput
                && stdout.object().kind() == posix::DescriptorObjectKind::StdioOutput
                && stderr.object().kind() == posix::DescriptorObjectKind::StdioOutput,
        )
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
        redirection: Option<LocalCommandExecRedirection>,
    ) -> Result<LocalCommandExecDescriptorInheritanceRecord, LocalCommandExecError> {
        let table = self
            .descriptor_store
            .descriptor_table(owner)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        let stdin = table
            .get(posix::STDIN_FD)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        let stdout = table.get(posix::STDOUT_FD);
        let stderr = table.get(posix::STDERR_FD);

        if stdin.require_readable().is_err()
            || !matches!(
                stdin.object().kind(),
                posix::DescriptorObjectKind::StdioInput
                    | posix::DescriptorObjectKind::PipeEndpoint
                    | posix::DescriptorObjectKind::RegularFile
                    | posix::DescriptorObjectKind::Device
            )
            || (stdin.object().kind() == posix::DescriptorObjectKind::Device
                && !stdin.object().is_dev_null())
        {
            return Err(LocalCommandExecError::LaunchPipelineFailed);
        }

        let mut inherited_count = 1usize;
        let stdout_kind = match stdout {
            Ok(stdout) => {
                if stdout.require_writable().is_err()
                    || !matches!(
                        stdout.object().kind(),
                        posix::DescriptorObjectKind::StdioOutput
                            | posix::DescriptorObjectKind::PipeEndpoint
                            | posix::DescriptorObjectKind::Device
                            | posix::DescriptorObjectKind::RegularFile
                    )
                {
                    return Err(LocalCommandExecError::LaunchPipelineFailed);
                }
                inherited_count += 1;
                stdout.object().kind().name()
            }
            Err(posix::PosixError::BadDescriptor)
                if redirection == Some(LocalCommandExecRedirection::CloseStdout) =>
            {
                "closed"
            }
            Err(_) => return Err(LocalCommandExecError::LaunchPipelineFailed),
        };
        let stderr_kind = match stderr {
            Ok(stderr) => {
                if stderr.require_writable().is_err()
                    || !matches!(
                        stderr.object().kind(),
                        posix::DescriptorObjectKind::StdioOutput
                            | posix::DescriptorObjectKind::PipeEndpoint
                            | posix::DescriptorObjectKind::Device
                            | posix::DescriptorObjectKind::RegularFile
                    )
                {
                    return Err(LocalCommandExecError::LaunchPipelineFailed);
                }
                inherited_count += 1;
                stderr.object().kind().name()
            }
            Err(posix::PosixError::BadDescriptor)
                if redirection == Some(LocalCommandExecRedirection::CloseStderr) =>
            {
                "closed"
            }
            Err(_) => return Err(LocalCommandExecError::LaunchPipelineFailed),
        };

        Ok(LocalCommandExecDescriptorInheritanceRecord {
            owner_id: owner.raw(),
            stdin_kind: stdin.object().kind().name(),
            stdout_kind,
            stderr_kind,
            inherited_count,
            loader_temporary_descriptor: LOCAL_COMMAND_EXEC_TEMP_DESCRIPTOR,
            loader_temporary_descriptor_open: table.get(LOCAL_COMMAND_EXEC_TEMP_DESCRIPTOR).is_ok(),
            source: "shell-process-descriptor-table",
        })
    }

    fn dispatch_stdin_fixture_read(
        &mut self,
        mappings: &[posix::UserMapping],
        user_memory: &mut [u8; LOCAL_COMMAND_STDIN_USER_MEMORY_LEN],
        scratch: &mut [u8; LOCAL_COMMAND_STDIN_USER_MEMORY_LEN],
        ready_byte: &mut Option<u8>,
    ) -> u64 {
        let descriptor_table = match self
            .descriptor_store
            .current_descriptor_table(self.current_owner)
        {
            Ok(table) => table,
            Err(_) => return (syscall::EBADF as u64).wrapping_neg(),
        };
        let entry = match descriptor_table.get(posix::STDIN_FD) {
            Ok(entry) => entry,
            Err(_) => return (syscall::EBADF as u64).wrapping_neg(),
        };
        if entry.object().kind() == posix::DescriptorObjectKind::PipeEndpoint {
            if entry.require_readable().is_err() {
                return (syscall::EBADF as u64).wrapping_neg();
            }
            let read_start = LOCAL_COMMAND_STDIN_READ_OFFSET;
            let read_len = initramfs::PHASE10_STDOUT_PAYLOAD.len();
            let bytes_read = self
                .pipe
                .read(&mut user_memory[read_start..read_start + read_len]);
            return bytes_read as u64;
        }

        let read_len = if entry.object().kind() == posix::DescriptorObjectKind::RegularFile {
            initramfs::PHASE8_BANNER_BYTES.len()
        } else {
            LOCAL_COMMAND_RUNTIME_STDIN_INPUT_BYTES.len()
        };
        let mut stdin_backend = RuntimeStdinReadBackend {
            ready_byte,
            backend: &mut self.input_backend,
        };
        syscall::dispatch_process_descriptor_with_initramfs_and_console_stdin(
            syscall::TALOS_READ_SYSCALL,
            syscall::SyscallArguments::new([
                posix::STDIN_FD as u64,
                LOCAL_COMMAND_STDIN_USER_BASE + LOCAL_COMMAND_STDIN_READ_OFFSET as u64,
                read_len as u64,
                0,
                0,
                0,
            ]),
            self.current_owner,
            &mut self.descriptor_store,
            mappings,
            LOCAL_COMMAND_STDIN_USER_BASE,
            user_memory,
            scratch,
            &mut self.output_backend,
            initramfs::phase8_readonly_initramfs_fixture(),
            &mut self.read_only_files,
            None,
            Some(&mut stdin_backend),
        )
        .return_value()
        .x0()
    }

    fn stdin_descriptor_is_pipe(&self) -> Result<bool, LocalCommandExecError> {
        let table = self
            .descriptor_store
            .current_descriptor_table(self.current_owner)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        let entry = table
            .get(posix::STDIN_FD)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        Ok(entry.object().kind() == posix::DescriptorObjectKind::PipeEndpoint)
    }

    fn stdin_descriptor_is_dev_null(&self) -> Result<bool, LocalCommandExecError> {
        let table = self
            .descriptor_store
            .current_descriptor_table(self.current_owner)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        let entry = table
            .get(posix::STDIN_FD)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        Ok(entry.object().is_dev_null())
    }

    fn stdin_descriptor_is_regular_file(&self) -> Result<bool, LocalCommandExecError> {
        let table = self
            .descriptor_store
            .current_descriptor_table(self.current_owner)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        let entry = table
            .get(posix::STDIN_FD)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        Ok(entry.object().kind() == posix::DescriptorObjectKind::RegularFile)
    }

    fn wait_for_runtime_stdin_readiness(
        &mut self,
        ready_byte: &mut Option<u8>,
    ) -> Result<LocalCommandSchedulerStdinWaitRecord, LocalCommandExecError> {
        let task_id = TaskId::new(LOCAL_COMMAND_EXEC_PROCESS_ID)
            .ok_or(LocalCommandExecError::LaunchPipelineFailed)?;
        let mut wait = LocalCommandRuntimeStdinWait::new(task_id, posix::STDIN_FD);
        self.write_scheduler_stdin_wait_line(wait.sleep())?;

        while wait.wait_cycles < LOCAL_COMMAND_RUNTIME_STDIN_WAIT_CYCLES {
            wait.observe_wait_cycle()?;
            if let Some(byte) = self.input_backend.poll_read_byte() {
                *ready_byte = Some(byte);
                let record = wait.wake();
                self.write_scheduler_stdin_wait_line(record)?;
                return Ok(record);
            }
        }

        let record = wait.cancel_no_data();
        self.write_scheduler_stdin_wait_line(record)?;
        Ok(record)
    }

    fn write_scheduler_stdin_wait_line(
        &mut self,
        record: LocalCommandSchedulerStdinWaitRecord,
    ) -> Result<(), LocalCommandExecError> {
        self.write_command_str("talos: stdin-wait task=")
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        write_hex_u64_to_command_sink(self, record.task_id)?;
        self.write_command_str(" fd=")
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        write_hex_usize_to_command_sink(self, record.descriptor)?;
        self.write_command_str(" sleep-state=")
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        self.write_command_str(task_state_name(record.sleep_state))
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        self.write_command_str(" wake-state=")
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        self.write_command_str(task_state_name(record.wake_state))
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        self.write_command_str(" wait-cycles=")
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        write_hex_usize_to_command_sink(self, record.wait_cycles)?;
        self.write_command_str(" result=")
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        self.write_command_str(record.result)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        self.write_command_str(" source=")
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        self.write_command_str(record.source)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        self.write_command_str("\n")
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)
    }

    fn emit_userspace_stdout_fixture(
        &mut self,
        source_path: &'static [u8],
    ) -> Result<Option<LocalCommandUserspaceStdoutRecord>, LocalCommandExecError> {
        if source_path != initramfs::PHASE10_STDOUT_PATH {
            return Ok(None);
        }

        const USER_MEMORY_LEN: usize = 128;
        const USER_MEMORY_BASE: u64 = 0x0000_0000_0012_0000;

        let payload = initramfs::PHASE10_STDOUT_PAYLOAD;
        let mut user_memory = [0u8; USER_MEMORY_LEN];
        let mut scratch = [0u8; USER_MEMORY_LEN];
        user_memory[..payload.len()].copy_from_slice(payload);
        let mappings = [posix::UserMapping::new(
            USER_MEMORY_BASE,
            user_memory.len(),
            posix::UserMappingPermissions::USER_DATA,
        )
        .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?];

        let return_value = self.write_userspace_fd_bytes(
            posix::STDOUT_FD,
            USER_MEMORY_BASE,
            &mappings,
            &user_memory,
            &mut scratch,
            payload.len(),
        );
        let bad_descriptor = (syscall::EBADF as u64).wrapping_neg();
        let (stream, route) = if return_value == payload.len() as u64 {
            self.output_route_metadata(posix::STDOUT_FD)?
        } else if return_value == bad_descriptor {
            (
                "closed",
                LocalCommandFieldText::from_static("closed-descriptor"),
            )
        } else {
            return Err(LocalCommandExecError::SyscallFailed);
        };

        Ok(Some(LocalCommandUserspaceStdoutRecord {
            descriptor: posix::STDOUT_FD,
            bytes: payload.len(),
            return_value,
            stream,
            route,
            source: "userspace-talos-write",
        }))
    }

    fn emit_userspace_stdin_fixture(
        &mut self,
        source_path: &'static [u8],
    ) -> Result<Option<LocalCommandUserspaceStdinRecord>, LocalCommandExecError> {
        if source_path != initramfs::PHASE10_STDIN_PATH {
            return Ok(None);
        }

        let mut user_memory = [0u8; LOCAL_COMMAND_STDIN_USER_MEMORY_LEN];
        let mut scratch = [0u8; LOCAL_COMMAND_STDIN_USER_MEMORY_LEN];
        let mappings = [posix::UserMapping::new(
            LOCAL_COMMAND_STDIN_USER_BASE,
            user_memory.len(),
            posix::UserMappingPermissions::USER_DATA,
        )
        .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?];
        let read_start = LOCAL_COMMAND_STDIN_READ_OFFSET;
        let mut expected_read_bytes = LOCAL_COMMAND_RUNTIME_STDIN_INPUT_BYTES.len();
        let mut readiness_observations = 0usize;
        let mut stdout_bytes = 0usize;
        let mut ready_byte = None;
        let mut scheduler_wait = None;
        let read_bytes;
        let mut read_result;
        let mut read_source = "runtime-console0/local-input";
        let mut read_return_value = self.dispatch_stdin_fixture_read(
            &mappings,
            &mut user_memory,
            &mut scratch,
            &mut ready_byte,
        );
        let stdin_is_pipe = self.stdin_descriptor_is_pipe()?;
        let stdin_is_dev_null = self.stdin_descriptor_is_dev_null()?;
        let stdin_is_regular_file = self.stdin_descriptor_is_regular_file()?;
        if stdin_is_dev_null {
            read_source = "device:/dev/null";
        }
        if read_return_value != (syscall::EAGAIN as u64).wrapping_neg() && stdin_is_pipe {
            expected_read_bytes = initramfs::PHASE10_STDOUT_PAYLOAD.len();
            read_source = "pipe:stdout-to-stdin";
        }
        if read_return_value != (syscall::EAGAIN as u64).wrapping_neg() && stdin_is_regular_file {
            expected_read_bytes = initramfs::PHASE8_BANNER_BYTES.len();
            read_source = "initramfs:/etc/banner.txt";
        }

        if read_return_value == (syscall::EAGAIN as u64).wrapping_neg() {
            let payload = initramfs::PHASE10_STDIN_READINESS_STDOUT;
            user_memory[..payload.len()].copy_from_slice(payload);
            let stdout_return =
                self.write_userspace_stdout_bytes(&mappings, &user_memory, payload.len())?;
            if stdout_return != payload.len() as u64 {
                return Err(LocalCommandExecError::SyscallFailed);
            }
            stdout_bytes = stdout_bytes
                .checked_add(payload.len())
                .ok_or(LocalCommandExecError::LaunchPipelineFailed)?;

            let wait_record = self.wait_for_runtime_stdin_readiness(&mut ready_byte)?;
            readiness_observations = wait_record.wait_cycles;
            scheduler_wait = Some(wait_record);
            if ready_byte.is_some() {
                read_return_value = self.dispatch_stdin_fixture_read(
                    &mappings,
                    &mut user_memory,
                    &mut scratch,
                    &mut ready_byte,
                );
            }
        }

        if read_return_value == expected_read_bytes as u64 {
            let read_end = read_start + expected_read_bytes;
            let read_input = &user_memory[read_start..read_end];
            if read_source == "pipe:stdout-to-stdin" {
                if read_input != initramfs::PHASE10_STDOUT_PAYLOAD
                    && read_input != initramfs::PHASE10_STDERR_PAYLOAD
                {
                    return Err(LocalCommandExecError::SyscallFailed);
                }
            } else if read_source == "initramfs:/etc/banner.txt" {
                if read_input != initramfs::PHASE8_BANNER_BYTES {
                    return Err(LocalCommandExecError::SyscallFailed);
                }
            } else if read_input != LOCAL_COMMAND_RUNTIME_STDIN_INPUT_BYTES {
                return Err(LocalCommandExecError::SyscallFailed);
            }

            let prefix = initramfs::PHASE10_STDIN_STDOUT_PREFIX;
            user_memory[..prefix.len()].copy_from_slice(prefix);
            let prefix_write =
                self.write_userspace_stdout_bytes(&mappings, &user_memory, prefix.len())?;
            user_memory.copy_within(read_start..read_end, 0);
            let read_write =
                self.write_userspace_stdout_bytes(&mappings, &user_memory, expected_read_bytes)?;
            user_memory[0] = b'\n';
            let newline_write = self.write_userspace_stdout_bytes(&mappings, &user_memory, 1)?;
            let read_stdout_bytes = prefix
                .len()
                .checked_add(expected_read_bytes)
                .and_then(|len| len.checked_add(1))
                .ok_or(LocalCommandExecError::LaunchPipelineFailed)?;
            if prefix_write + read_write + newline_write != read_stdout_bytes as u64 {
                return Err(LocalCommandExecError::SyscallFailed);
            }
            stdout_bytes = stdout_bytes
                .checked_add(read_stdout_bytes)
                .ok_or(LocalCommandExecError::LaunchPipelineFailed)?;
            read_bytes = expected_read_bytes;
            read_result = if readiness_observations == 0 {
                None
            } else {
                Some("scheduler-wait/delayed-input")
            };
            if read_source == "pipe:stdout-to-stdin" {
                let eof = self.dispatch_stdin_fixture_read(
                    &mappings,
                    &mut user_memory,
                    &mut scratch,
                    &mut ready_byte,
                );
                if eof != 0 {
                    return Err(LocalCommandExecError::SyscallFailed);
                }
                read_result = Some("pipe-eof-after-writer-close");
            } else if read_source == "initramfs:/etc/banner.txt" {
                let eof = self.dispatch_stdin_fixture_read(
                    &mappings,
                    &mut user_memory,
                    &mut scratch,
                    &mut ready_byte,
                );
                if eof != 0 {
                    return Err(LocalCommandExecError::SyscallFailed);
                }
                read_result = Some("regular-file-eof-after-read");
            }
        } else if read_return_value == (syscall::EAGAIN as u64).wrapping_neg()
            && scheduler_wait.is_some()
        {
            read_bytes = 0;
            read_result = Some("readiness/no-data");
        } else if read_return_value == 0 && stdin_is_pipe {
            let payload = initramfs::PHASE10_STDIN_PIPE_EOF_STDOUT;
            user_memory[..payload.len()].copy_from_slice(payload);
            let stdout_return =
                self.write_userspace_stdout_bytes(&mappings, &user_memory, payload.len())?;
            if stdout_return != payload.len() as u64 {
                return Err(LocalCommandExecError::SyscallFailed);
            }
            stdout_bytes = stdout_bytes
                .checked_add(payload.len())
                .ok_or(LocalCommandExecError::LaunchPipelineFailed)?;
            read_bytes = 0;
            read_result = Some("pipe-eof/no-data");
        } else if read_return_value == 0 && stdin_is_dev_null {
            let payload = initramfs::PHASE10_STDIN_DEV_NULL_EOF_STDOUT;
            user_memory[..payload.len()].copy_from_slice(payload);
            let stdout_return =
                self.write_userspace_stdout_bytes(&mappings, &user_memory, payload.len())?;
            if stdout_return != payload.len() as u64 {
                return Err(LocalCommandExecError::SyscallFailed);
            }
            stdout_bytes = stdout_bytes
                .checked_add(payload.len())
                .ok_or(LocalCommandExecError::LaunchPipelineFailed)?;
            read_bytes = 0;
            read_result = Some("null-source-eof/no-data");
        } else if read_return_value == 0 {
            let payload = initramfs::PHASE10_STDIN_TERMINAL_EOF_STDOUT;
            user_memory[..payload.len()].copy_from_slice(payload);
            let stdout_return =
                self.write_userspace_stdout_bytes(&mappings, &user_memory, payload.len())?;
            if stdout_return != payload.len() as u64 {
                return Err(LocalCommandExecError::SyscallFailed);
            }
            stdout_bytes = stdout_bytes
                .checked_add(payload.len())
                .ok_or(LocalCommandExecError::LaunchPipelineFailed)?;
            read_bytes = 0;
            read_result = Some("terminal-eof");
        } else {
            return Err(LocalCommandExecError::SyscallFailed);
        }

        Ok(Some(LocalCommandUserspaceStdinRecord {
            read_descriptor: posix::STDIN_FD,
            read_bytes,
            read_return_value,
            read_source,
            stdout_descriptor: posix::STDOUT_FD,
            stdout_bytes,
            stdout_return_value: stdout_bytes as u64,
            source: "userspace-talos-read+userspace-talos-write",
            read_result,
            readiness_observations,
            scheduler_wait,
        }))
    }

    fn emit_userspace_stderr_fixture(
        &mut self,
        source_path: &'static [u8],
    ) -> Result<Option<LocalCommandUserspaceStderrRecord>, LocalCommandExecError> {
        if source_path != initramfs::PHASE10_STDERR_PATH {
            return Ok(None);
        }

        let payload = initramfs::PHASE10_STDERR_PAYLOAD;
        let mut user_memory = [0u8; LOCAL_COMMAND_STDERR_USER_MEMORY_LEN];
        user_memory[..payload.len()].copy_from_slice(payload);
        let mappings = [posix::UserMapping::new(
            LOCAL_COMMAND_STDERR_USER_BASE,
            user_memory.len(),
            posix::UserMappingPermissions::USER_DATA,
        )
        .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?];
        let mut scratch = [0u8; LOCAL_COMMAND_STDERR_USER_MEMORY_LEN];
        let return_value = self.write_userspace_fd_bytes(
            posix::STDERR_FD,
            LOCAL_COMMAND_STDERR_USER_BASE,
            &mappings,
            &user_memory,
            &mut scratch,
            payload.len(),
        );
        let bad_descriptor = (syscall::EBADF as u64).wrapping_neg();
        let (stream, route) = if return_value == payload.len() as u64 {
            self.output_route_metadata(posix::STDERR_FD)?
        } else if return_value == bad_descriptor {
            (
                "closed",
                LocalCommandFieldText::from_static("closed-descriptor"),
            )
        } else {
            return Err(LocalCommandExecError::SyscallFailed);
        };

        Ok(Some(LocalCommandUserspaceStderrRecord {
            descriptor: posix::STDERR_FD,
            bytes: payload.len(),
            return_value,
            stream,
            route,
            source: "userspace-talos-write",
        }))
    }

    fn output_route_metadata(
        &self,
        descriptor: usize,
    ) -> Result<
        (
            &'static str,
            LocalCommandFieldText<LOCAL_COMMAND_VOLATILE_ROUTE_BYTES>,
        ),
        LocalCommandExecError,
    > {
        let table = self
            .descriptor_store
            .current_descriptor_table(self.current_owner)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        let entry = table
            .get(descriptor)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        if entry.require_writable().is_err() {
            return Err(LocalCommandExecError::LaunchPipelineFailed);
        }
        match entry.object().kind() {
            posix::DescriptorObjectKind::StdioOutput => Ok((
                entry.object().stdio_stream_name(),
                LocalCommandFieldText::from_static(entry.object().runtime_console_route_name()),
            )),
            posix::DescriptorObjectKind::PipeEndpoint => Ok((
                "pipe-writer",
                LocalCommandFieldText::from_static("pipe:stdout-to-stdin"),
            )),
            posix::DescriptorObjectKind::Device if entry.object().is_dev_null() => Ok((
                "null-sink",
                LocalCommandFieldText::from_static("device:/dev/null"),
            )),
            posix::DescriptorObjectKind::RegularFile => match entry.object().reference() {
                LOCAL_COMMAND_STDOUT_VOLATILE_FILE_REFERENCE => Ok((
                    "regular-file",
                    LocalCommandVolatileFileTarget::Stdout(self.stdout_scratch_file.path)
                        .route_text(),
                )),
                LOCAL_COMMAND_STDERR_VOLATILE_FILE_REFERENCE => Ok((
                    "regular-file",
                    LocalCommandVolatileFileTarget::Stderr(self.stderr_scratch_file.path)
                        .route_text(),
                )),
                _ => Err(LocalCommandExecError::LaunchPipelineFailed),
            },
            _ => Err(LocalCommandExecError::LaunchPipelineFailed),
        }
    }

    fn write_userspace_stdout_bytes(
        &mut self,
        mappings: &[posix::UserMapping],
        user_memory: &[u8],
        len: usize,
    ) -> Result<u64, LocalCommandExecError> {
        self.write_userspace_stdio_bytes(
            posix::STDOUT_FD,
            LOCAL_COMMAND_STDIN_USER_BASE,
            mappings,
            user_memory,
            len,
        )
    }

    fn write_userspace_stdio_bytes(
        &mut self,
        descriptor: usize,
        user_memory_base: u64,
        mappings: &[posix::UserMapping],
        user_memory: &[u8],
        len: usize,
    ) -> Result<u64, LocalCommandExecError> {
        let mut scratch = [0u8; LOCAL_COMMAND_STDIN_USER_MEMORY_LEN];
        Ok(self.write_userspace_fd_bytes(
            descriptor,
            user_memory_base,
            mappings,
            user_memory,
            &mut scratch,
            len,
        ))
    }

    fn write_userspace_fd_bytes(
        &mut self,
        descriptor: usize,
        user_memory_base: u64,
        mappings: &[posix::UserMapping],
        user_memory: &[u8],
        scratch: &mut [u8],
        len: usize,
    ) -> u64 {
        let descriptor_table = match self
            .descriptor_store
            .current_descriptor_table(self.current_owner)
        {
            Ok(table) => table,
            Err(_) => return (syscall::EBADF as u64).wrapping_neg(),
        };
        let entry = match descriptor_table.get(descriptor) {
            Ok(entry) => entry,
            Err(_) => return (syscall::EBADF as u64).wrapping_neg(),
        };
        if entry.object().kind() == posix::DescriptorObjectKind::PipeEndpoint {
            if entry.require_writable().is_err() || len > user_memory.len() {
                return (syscall::EBADF as u64).wrapping_neg();
            }
            return match self.pipe.write(&user_memory[..len]) {
                Ok(bytes) => bytes as u64,
                Err(_) => (syscall::EPIPE as u64).wrapping_neg(),
            };
        }
        if entry.object().kind() == posix::DescriptorObjectKind::RegularFile {
            let target = match entry.object().reference() {
                LOCAL_COMMAND_STDOUT_VOLATILE_FILE_REFERENCE => {
                    LocalCommandVolatileFileTarget::Stdout(self.stdout_scratch_file.path)
                }
                LOCAL_COMMAND_STDERR_VOLATILE_FILE_REFERENCE => {
                    LocalCommandVolatileFileTarget::Stderr(self.stderr_scratch_file.path)
                }
                _ => return (syscall::EBADF as u64).wrapping_neg(),
            };
            if entry.require_writable().is_err() {
                return (syscall::EBADF as u64).wrapping_neg();
            }
            if len > user_memory.len() || scratch.len() < len {
                return (syscall::EINVAL as u64).wrapping_neg();
            }
            if posix::copy_from_user(
                mappings,
                user_memory_base,
                user_memory,
                user_memory_base,
                len,
                &mut scratch[..len],
            )
            .is_err()
            {
                return (syscall::EFAULT as u64).wrapping_neg();
            }
            let write = match target {
                LocalCommandVolatileFileTarget::Stdout(_) => {
                    self.stdout_scratch_file.write(&scratch[..len])
                }
                LocalCommandVolatileFileTarget::Stderr(_) => {
                    self.stderr_scratch_file.write(&scratch[..len])
                }
            };
            return match write {
                Ok(bytes) => bytes as u64,
                Err(_) => (syscall::ENOSPC as u64).wrapping_neg(),
            };
        }

        syscall::dispatch_process_descriptor(
            syscall::TALOS_WRITE_SYSCALL,
            syscall::SyscallArguments::new([
                descriptor as u64,
                user_memory_base,
                len as u64,
                0,
                0,
                0,
            ]),
            self.current_owner,
            &mut self.descriptor_store,
            mappings,
            user_memory_base,
            user_memory,
            scratch,
            &mut self.output_backend,
        )
        .return_value()
        .x0()
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

    if let Some(job) = sink.poll_background_job_completion() {
        write_background_job_completion_line(sink, responses, job)?;
    }

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
                Some(path)
                    if LocalCommandVolatilePath::from_supported_stdout_path(path.as_bytes())
                        .is_some()
                        || LocalCommandVolatilePath::from_supported_stderr_path(
                            path.as_bytes(),
                        )
                        .is_some() =>
                {
                    write_tmp_scratch_file(sink, responses, path.as_bytes())?
                }
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
            if has_background_exec_suffix(arguments) {
                match parse_background_exec_request(arguments)
                    .and_then(|request| sink.exec_background_vfs_program(request))
                {
                    Ok(summary) => {
                        write_background_exec_summary(sink, responses, summary)?;
                        return Ok(LocalCommandStatus::Handled);
                    }
                    Err(LocalCommandExecError::InvalidPath) => {
                        write_line(sink, responses, "talos: exec-invalid-path")?;
                        return Ok(LocalCommandStatus::UnexpectedArgument);
                    }
                    Err(LocalCommandExecError::NotExecutable) => {
                        write_line(sink, responses, "talos: exec-not-executable")?;
                        return Ok(LocalCommandStatus::UnexpectedArgument);
                    }
                    Err(LocalCommandExecError::NotFound) => {
                        write_line(sink, responses, "talos: exec-not-found")?;
                        return Ok(LocalCommandStatus::UnexpectedArgument);
                    }
                    Err(_) => {
                        write_line(sink, responses, "talos: exec-error")?;
                        return Ok(LocalCommandStatus::UnexpectedArgument);
                    }
                }
            }
            if arguments.as_bytes().contains(&b'|') {
                match parse_pipeline_request(arguments)
                    .and_then(|request| sink.exec_vfs_pipeline(request))
                {
                    Ok(summary) => {
                        write_pipeline_summary(sink, responses, summary)?;
                        return Ok(LocalCommandStatus::Handled);
                    }
                    Err(LocalCommandExecError::InvalidPath) => {
                        write_line(sink, responses, "talos: exec-invalid-path")?;
                        return Ok(LocalCommandStatus::UnexpectedArgument);
                    }
                    Err(LocalCommandExecError::NotExecutable) => {
                        write_line(sink, responses, "talos: exec-not-executable")?;
                        return Ok(LocalCommandStatus::UnexpectedArgument);
                    }
                    Err(LocalCommandExecError::NotFound) => {
                        write_line(sink, responses, "talos: exec-not-found")?;
                        return Ok(LocalCommandStatus::UnexpectedArgument);
                    }
                    Err(LocalCommandExecError::SyscallFailed) => {
                        write_line(sink, responses, "talos: exec-syscall-failed")?;
                        return Ok(LocalCommandStatus::UnexpectedArgument);
                    }
                    Err(LocalCommandExecError::LaunchPipelineFailed) => {
                        write_line(sink, responses, "talos: exec-launch-failed")?;
                        return Ok(LocalCommandStatus::UnexpectedArgument);
                    }
                    Err(_) => {
                        write_line(sink, responses, "talos: exec-error")?;
                        return Ok(LocalCommandStatus::UnexpectedArgument);
                    }
                }
            }
            match parse_exec_request(arguments).and_then(|request| sink.exec_vfs_program(request)) {
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

fn parse_pipeline_request(
    arguments: &str,
) -> Result<LocalCommandPipelineRequest, LocalCommandExecError> {
    let bytes = arguments.as_bytes();
    let mut pipe = None;
    let mut index = 0usize;
    while index < bytes.len() {
        if bytes[index] == b'|' {
            if pipe.is_some() {
                return Err(LocalCommandExecError::InvalidPath);
            }
            pipe = Some(index);
        }
        index += 1;
    }
    let pipe = pipe.ok_or(LocalCommandExecError::InvalidPath)?;
    let producer = trim_ascii_space(&arguments[..pipe]);
    let consumer = trim_ascii_space(&arguments[pipe + 1..]);
    let consumer = consumer
        .strip_prefix("exec ")
        .ok_or(LocalCommandExecError::InvalidPath)?;
    if producer.is_empty() || consumer.is_empty() {
        return Err(LocalCommandExecError::InvalidPath);
    }
    Ok(LocalCommandPipelineRequest {
        producer: parse_exec_request(producer)?,
        consumer: parse_exec_request(consumer)?,
    })
}

fn parse_background_exec_request(
    arguments: &str,
) -> Result<LocalCommandExecRequest, LocalCommandExecError> {
    let trimmed = trim_ascii_space(arguments);
    let foreground = trimmed
        .strip_suffix(" &")
        .ok_or(LocalCommandExecError::InvalidPath)?;
    if foreground.as_bytes().contains(&b'&') {
        return Err(LocalCommandExecError::InvalidPath);
    }
    parse_exec_request(trim_ascii_space(foreground))
}

fn has_background_exec_suffix(arguments: &str) -> bool {
    trim_ascii_space(arguments).ends_with(" &")
}

fn trim_ascii_space(text: &str) -> &str {
    let bytes = text.as_bytes();
    let mut start = 0usize;
    while start < bytes.len() && is_space(bytes[start]) {
        start += 1;
    }
    let mut end = bytes.len();
    while end > start && is_space(bytes[end - 1]) {
        end -= 1;
    }
    &text[start..end]
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

fn parse_exec_request(arguments: &str) -> Result<LocalCommandExecRequest, LocalCommandExecError> {
    let mut tokens: [&[u8]; LOCAL_COMMAND_LITERAL_ARGV_CAPACITY] =
        [&[]; LOCAL_COMMAND_LITERAL_ARGV_CAPACITY];
    let mut count = 0usize;
    let mut redirection = None;
    let mut stdin_redirection = None;
    let mut redirection_started = false;
    for token in arguments.as_bytes().split(|byte| is_space(*byte)) {
        if token.is_empty() {
            continue;
        }
        let parsed_redirection = if token == b"1>&2" {
            Some(LocalCommandExecRedirection::StdoutToStderr)
        } else if token == b"2>&1" {
            Some(LocalCommandExecRedirection::StderrToStdout)
        } else if token == b"1>&-" {
            Some(LocalCommandExecRedirection::CloseStdout)
        } else if token == b"2>&-" {
            Some(LocalCommandExecRedirection::CloseStderr)
        } else if token == b">/dev/null" {
            Some(LocalCommandExecRedirection::StdoutToDevNull)
        } else if token == b"2>/dev/null" {
            Some(LocalCommandExecRedirection::StderrToDevNull)
        } else if token == b"</dev/null" {
            Some(LocalCommandExecRedirection::StdinFromDevNull)
        } else if token == b"</etc/banner.txt" {
            Some(LocalCommandExecRedirection::StdinFromEtcBanner)
        } else if let Some(path) = token
            .strip_prefix(b">")
            .and_then(LocalCommandVolatilePath::from_supported_stdout_path)
        {
            Some(LocalCommandExecRedirection::StdoutToTmpStdout(path))
        } else if let Some(path) = token
            .strip_prefix(b"1>")
            .and_then(LocalCommandVolatilePath::from_supported_stdout_path)
        {
            Some(LocalCommandExecRedirection::StdoutToTmpStdout(path))
        } else if let Some(path) = token
            .strip_prefix(b">>")
            .and_then(LocalCommandVolatilePath::from_supported_stdout_path)
        {
            Some(LocalCommandExecRedirection::StdoutAppendTmpStdout(path))
        } else if let Some(path) = token
            .strip_prefix(b"1>>")
            .and_then(LocalCommandVolatilePath::from_supported_stdout_path)
        {
            Some(LocalCommandExecRedirection::StdoutAppendTmpStdout(path))
        } else if let Some(path) = token
            .strip_prefix(b"2>")
            .and_then(LocalCommandVolatilePath::from_supported_stderr_path)
        {
            Some(LocalCommandExecRedirection::StderrToTmpStderr(path))
        } else if let Some(path) = token
            .strip_prefix(b"2>>")
            .and_then(LocalCommandVolatilePath::from_supported_stderr_path)
        {
            Some(LocalCommandExecRedirection::StderrAppendTmpStderr(path))
        } else {
            None
        };
        if let Some(parsed_redirection) = parsed_redirection {
            redirection_started = true;
            if matches!(
                parsed_redirection,
                LocalCommandExecRedirection::StdinFromDevNull
                    | LocalCommandExecRedirection::StdinFromEtcBanner
            ) {
                if stdin_redirection.is_some() || redirection.is_some() {
                    return Err(LocalCommandExecError::InvalidPath);
                }
                stdin_redirection = Some(parsed_redirection);
            } else {
                if redirection.is_some() {
                    return Err(LocalCommandExecError::InvalidPath);
                }
                if stdin_redirection.is_some()
                    && (!matches!(
                        stdin_redirection,
                        Some(LocalCommandExecRedirection::StdinFromEtcBanner)
                    ) || !matches!(
                        parsed_redirection,
                        LocalCommandExecRedirection::StdoutToTmpStdout(_)
                    ) || !token.starts_with(b">/tmp/"))
                {
                    return Err(LocalCommandExecError::InvalidPath);
                }
                redirection = Some(parsed_redirection);
            }
            continue;
        }
        if redirection_started {
            return Err(LocalCommandExecError::InvalidPath);
        }
        if count == tokens.len() || !is_supported_literal_exec_token(token) {
            return Err(LocalCommandExecError::InvalidPath);
        }
        tokens[count] = token;
        count += 1;
    }
    if count == 0 {
        return Err(LocalCommandExecError::InvalidPath);
    }
    let path = if is_absolute_exec_path(tokens[0]) {
        LocalCommandExecPath::from_absolute(tokens[0])?
    } else if tokens[0].iter().any(|byte| *byte == b'/') {
        return Err(LocalCommandExecError::InvalidPath);
    } else {
        LocalCommandExecPath::from_fixed_bin_name(tokens[0])?
    };
    let argv = LocalCommandLiteralArgv::from_tokens(&tokens[..count])?
        .with_resolved_argv0(path.as_bytes())?;
    Ok(LocalCommandExecRequest {
        path,
        argv,
        redirection,
        stdin_redirection,
    })
}

fn is_supported_literal_exec_token(token: &[u8]) -> bool {
    token.iter().all(|byte| {
        matches!(
            byte,
            b'a'..=b'z'
                | b'A'..=b'Z'
                | b'0'..=b'9'
                | b'/'
                | b'.'
                | b'_'
                | b'-'
                | b'+'
                | b':'
        )
    })
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

fn write_tmp_scratch_file(
    sink: &mut impl LocalCommandSink,
    responses: &mut usize,
    path: &[u8],
) -> Result<(), LocalCommandCycleError> {
    let stdout_path = LocalCommandVolatilePath::from_supported_stdout_path(path);
    let stderr_path = LocalCommandVolatilePath::from_supported_stderr_path(path);
    if let Some(path) = stdout_path {
        let mut bytes = [0u8; LOCAL_COMMAND_VOLATILE_FILE_BYTES];
        match sink.read_stdout_tmp_file_via_descriptor(path.as_bytes(), &mut bytes) {
            Ok(bytes_read) => {
                return write_volatile_scratch_file_contents(
                    sink,
                    responses,
                    LocalCommandVolatileFileTarget::Stdout(path),
                    &bytes[..bytes_read],
                );
            }
            Err(LocalCommandFileReadError::NotFound) => {}
            Err(_) => {
                write_line(sink, responses, "talos: filesystem-error")?;
                return Ok(());
            }
        }
    }
    if let Some(path) = stderr_path {
        let mut bytes = [0u8; LOCAL_COMMAND_VOLATILE_FILE_BYTES];
        match sink.read_stderr_tmp_file_via_descriptor(path.as_bytes(), &mut bytes) {
            Ok(bytes_read) => {
                return write_volatile_scratch_file_contents(
                    sink,
                    responses,
                    LocalCommandVolatileFileTarget::Stderr(path),
                    &bytes[..bytes_read],
                );
            }
            Err(LocalCommandFileReadError::NotFound) => {}
            Err(_) => {
                write_line(sink, responses, "talos: filesystem-error")?;
                return Ok(());
            }
        }
    }
    write_line(sink, responses, "talos: not-found")
}

fn write_volatile_scratch_file_contents(
    sink: &mut impl LocalCommandSink,
    responses: &mut usize,
    target: LocalCommandVolatileFileTarget,
    bytes: &[u8],
) -> Result<(), LocalCommandCycleError> {
    let text = match core::str::from_utf8(bytes) {
        Ok(text) => text,
        Err(_) => {
            write_line(sink, responses, "talos: filesystem-error")?;
            return Ok(());
        }
    };
    write_file_contents(sink, responses, text)?;
    write_str_part(sink, "talos: cat path=")?;
    write_field_text_part(sink, target.path_text())?;
    write_str_part(sink, " bytes=")?;
    write_hex_usize_part(sink, bytes.len())?;
    write_str_part(sink, " source=volatile-vfs-descriptor-read")?;
    finish_dynamic_line(sink, responses)
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
    for record in summary.redirections {
        if let Some(record) = record {
            write_exec_redirection_line(sink, response_lines, record)?;
        }
    }
    if let Some(record) = summary.userspace_stdout {
        write_exec_userspace_stdout_line(sink, response_lines, record)?;
    }
    if let Some(record) = summary.userspace_stdin {
        write_exec_userspace_stdin_line(sink, response_lines, record)?;
    }
    if let Some(record) = summary.userspace_stderr {
        write_exec_userspace_stderr_line(sink, response_lines, record)?;
    }
    write_exec_lifecycle_line(sink, response_lines, summary)?;
    write_exec_status_line(sink, response_lines, summary)?;
    write_line(
        sink,
        response_lines,
        "talos: exec-signal lower-aarch64-svc-launch-boundary-equivalent",
    )
}

fn write_background_exec_summary(
    sink: &mut impl LocalCommandSink,
    response_lines: &mut usize,
    summary: LocalCommandBackgroundExecSummary,
) -> Result<(), LocalCommandCycleError> {
    write_str_part(sink, "talos: exec path=")?;
    write_byte_path_part(sink, summary.exec.source_path)?;
    write_str_part(sink, " source=vfs-open-read mode=background")?;
    finish_dynamic_line(sink, response_lines)?;
    write_exec_source_line(sink, response_lines, summary.exec)?;
    write_exec_entry_line(sink, response_lines, summary.exec)?;
    write_exec_launch_line(sink, response_lines, summary.exec)?;
    write_exec_descriptor_inheritance_line(sink, response_lines, summary.exec)?;
    write_exec_startup_abi_line(sink, response_lines, summary.exec)?;
    for record in summary.exec.redirections {
        if let Some(record) = record {
            write_exec_redirection_line(sink, response_lines, record)?;
        }
    }
    if let Some(record) = summary.exec.userspace_stdout {
        write_exec_userspace_stdout_line(sink, response_lines, record)?;
    }
    if let Some(record) = summary.exec.userspace_stdin {
        write_exec_userspace_stdin_line(sink, response_lines, record)?;
    }
    if let Some(record) = summary.exec.userspace_stderr {
        write_exec_userspace_stderr_line(sink, response_lines, record)?;
    }
    write_background_job_running_line(sink, response_lines, summary.job)?;
    write_line(
        sink,
        response_lines,
        "talos: background-signal lower-aarch64-svc-launch-boundary-equivalent",
    )
}

fn write_background_job_running_line(
    sink: &mut impl LocalCommandSink,
    response_lines: &mut usize,
    job: LocalCommandBackgroundJobRecord,
) -> Result<(), LocalCommandCycleError> {
    write_background_job_line(
        sink,
        response_lines,
        job,
        "status=pending shell-responsive=true",
    )
}

fn write_background_job_completion_line(
    sink: &mut impl LocalCommandSink,
    response_lines: &mut usize,
    job: LocalCommandBackgroundJobRecord,
) -> Result<(), LocalCommandCycleError> {
    write_background_job_line(sink, response_lines, job, "shell-responsive=observed")
}

fn write_background_job_line(
    sink: &mut impl LocalCommandSink,
    response_lines: &mut usize,
    job: LocalCommandBackgroundJobRecord,
    suffix: &str,
) -> Result<(), LocalCommandCycleError> {
    write_str_part(sink, "talos: background-job id=")?;
    write_hex_u64_part(sink, job.job_id)?;
    write_str_part(sink, " pid=")?;
    write_hex_u64_part(sink, job.lifecycle.process_id)?;
    write_str_part(sink, " command=")?;
    write_byte_path_part(sink, job.command_label)?;
    write_str_part(sink, " state=")?;
    write_str_part(sink, job.state.name())?;
    if job.state == LocalCommandBackgroundJobState::Completed {
        write_str_part(sink, " status=")?;
        write_hex_u64_part(sink, job.lifecycle.status)?;
        write_str_part(sink, " observed-status=")?;
        write_hex_u64_part(sink, job.lifecycle.observed_status)?;
    }
    write_str_part(sink, " reaped=")?;
    write_str_part(sink, if job.reaped { "true" } else { "false" })?;
    write_str_part(sink, " ")?;
    write_str_part(sink, suffix)?;
    write_str_part(sink, " source=background-vfs-exec-accounting")?;
    finish_dynamic_line(sink, response_lines)
}

fn write_pipeline_summary(
    sink: &mut impl LocalCommandSink,
    response_lines: &mut usize,
    summary: LocalCommandPipelineSummary,
) -> Result<(), LocalCommandCycleError> {
    let record = summary.pipe;
    write_str_part(sink, "talos: pipeline id=")?;
    write_hex_usize_part(sink, record.id)?;
    write_str_part(sink, " producer-fd=")?;
    write_hex_usize_part(sink, record.producer_fd)?;
    write_str_part(sink, " producer-path=")?;
    write_byte_path_part(sink, record.producer_path)?;
    write_str_part(sink, " consumer-fd=")?;
    write_hex_usize_part(sink, record.consumer_fd)?;
    write_str_part(sink, " consumer-path=")?;
    write_byte_path_part(sink, record.consumer_path)?;
    write_str_part(sink, " bytes-written=")?;
    write_hex_usize_part(sink, record.bytes_written)?;
    write_str_part(sink, " bytes-read=")?;
    write_hex_usize_part(sink, record.bytes_read)?;
    write_str_part(sink, " writer-closed=")?;
    write_str_part(
        sink,
        if record.writer_closed {
            "true"
        } else {
            "false"
        },
    )?;
    write_str_part(sink, " reader-eof=")?;
    write_str_part(sink, if record.reader_eof { "true" } else { "false" })?;
    write_str_part(sink, " shell-restored=")?;
    write_str_part(
        sink,
        if record.shell_restored {
            "true"
        } else {
            "false"
        },
    )?;
    write_str_part(sink, " source=")?;
    write_str_part(sink, record.source)?;
    finish_dynamic_line(sink, response_lines)?;
    write_exec_summary(sink, response_lines, summary.producer)?;
    write_exec_summary(sink, response_lines, summary.consumer)
}

fn write_exec_redirection_line(
    sink: &mut impl LocalCommandSink,
    response_lines: &mut usize,
    record: LocalCommandExecRedirectionRecord,
) -> Result<(), LocalCommandCycleError> {
    write_str_part(sink, "talos: exec-redirection op=")?;
    write_str_part(sink, record.operation)?;
    write_str_part(sink, " source-fd=")?;
    write_hex_usize_part(sink, record.source_descriptor)?;
    if let Some(target_descriptor) = record.target_descriptor {
        write_str_part(sink, " target-fd=")?;
        write_hex_usize_part(sink, target_descriptor)?;
        write_str_part(sink, " target-stream=")?;
        write_str_part(sink, record.target_stream)?;
        write_str_part(sink, " target-route=")?;
        write_field_text_part(sink, record.target_route)?;
    } else if let Some(target_path) = record.target_path {
        if record.operation == "source" {
            write_str_part(sink, " source-path=")?;
        } else {
            write_str_part(sink, " target-path=")?;
        }
        write_field_text_part(sink, target_path)?;
        if record.operation == "source" {
            write_str_part(sink, " source-stream=")?;
        } else {
            write_str_part(sink, " target-stream=")?;
        }
        write_str_part(sink, record.target_stream)?;
        if record.operation == "source" {
            write_str_part(sink, " source-route=")?;
        } else {
            write_str_part(sink, " target-route=")?;
        }
        write_field_text_part(sink, record.target_route)?;
    } else {
        write_str_part(sink, " result=closed-descriptor")?;
    }
    write_str_part(sink, " child-only=")?;
    write_str_part(sink, if record.child_only { "true" } else { "false" })?;
    write_str_part(sink, " shell-restored=")?;
    write_str_part(
        sink,
        if record.shell_restored {
            "true"
        } else {
            "false"
        },
    )?;
    write_str_part(sink, " source=")?;
    write_str_part(sink, record.source)?;
    finish_dynamic_line(sink, response_lines)
}

fn write_field_text_part<const N: usize>(
    sink: &mut impl LocalCommandSink,
    text: LocalCommandFieldText<N>,
) -> Result<(), LocalCommandCycleError> {
    match text {
        LocalCommandFieldText::Static(text) => write_str_part(sink, text),
        LocalCommandFieldText::Inline(text) => write_byte_path_part(sink, text.as_bytes()),
    }
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
    let mut index = 1usize;
    while let Some(arg) = summary.startup_argv.arg(index) {
        write_str_part(sink, " argv")?;
        write_decimal_usize_part(sink, index)?;
        write_str_part(sink, "=")?;
        write_byte_path_part(sink, arg.as_bytes())?;
        index += 1;
    }
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

fn write_exec_userspace_stdout_line(
    sink: &mut impl LocalCommandSink,
    response_lines: &mut usize,
    record: LocalCommandUserspaceStdoutRecord,
) -> Result<(), LocalCommandCycleError> {
    write_str_part(sink, "talos: exec-stdout fd=")?;
    write_hex_usize_part(sink, record.descriptor)?;
    write_str_part(sink, " bytes=")?;
    write_hex_usize_part(sink, record.bytes)?;
    write_str_part(sink, " return=")?;
    write_hex_u64_part(sink, record.return_value)?;
    write_str_part(sink, " stream=")?;
    write_str_part(sink, record.stream)?;
    write_str_part(sink, " route=")?;
    write_field_text_part(sink, record.route)?;
    write_str_part(sink, " source=")?;
    write_str_part(sink, record.source)?;
    finish_dynamic_line(sink, response_lines)
}

fn write_exec_userspace_stdin_line(
    sink: &mut impl LocalCommandSink,
    response_lines: &mut usize,
    record: LocalCommandUserspaceStdinRecord,
) -> Result<(), LocalCommandCycleError> {
    write_str_part(sink, "talos: exec-stdin fd=")?;
    write_hex_usize_part(sink, record.read_descriptor)?;
    write_str_part(sink, " bytes=")?;
    write_hex_usize_part(sink, record.read_bytes)?;
    write_str_part(sink, " return=")?;
    write_hex_u64_part(sink, record.read_return_value)?;
    write_str_part(sink, " read-source=")?;
    write_str_part(sink, record.read_source)?;
    write_str_part(sink, " stdout-fd=")?;
    write_hex_usize_part(sink, record.stdout_descriptor)?;
    write_str_part(sink, " stdout-bytes=")?;
    write_hex_usize_part(sink, record.stdout_bytes)?;
    write_str_part(sink, " stdout-return=")?;
    write_hex_u64_part(sink, record.stdout_return_value)?;
    write_str_part(sink, " source=")?;
    write_str_part(sink, record.source)?;
    if let Some(result) = record.read_result {
        write_str_part(sink, " read-result=")?;
        write_str_part(sink, result)?;
    }
    if record.readiness_observations != 0 {
        write_str_part(sink, " readiness-observations=")?;
        write_hex_usize_part(sink, record.readiness_observations)?;
    }
    if let Some(wait) = record.scheduler_wait {
        write_str_part(sink, " scheduler-wait-result=")?;
        write_str_part(sink, wait.result)?;
        write_str_part(sink, " scheduler-wait-cycles=")?;
        write_hex_usize_part(sink, wait.wait_cycles)?;
        write_str_part(sink, " scheduler-wait-source=")?;
        write_str_part(sink, wait.source)?;
    }
    finish_dynamic_line(sink, response_lines)
}

fn write_exec_userspace_stderr_line(
    sink: &mut impl LocalCommandSink,
    response_lines: &mut usize,
    record: LocalCommandUserspaceStderrRecord,
) -> Result<(), LocalCommandCycleError> {
    write_str_part(sink, "talos: exec-stderr fd=")?;
    write_hex_usize_part(sink, record.descriptor)?;
    write_str_part(sink, " bytes=")?;
    write_hex_usize_part(sink, record.bytes)?;
    write_str_part(sink, " return=")?;
    write_hex_u64_part(sink, record.return_value)?;
    write_str_part(sink, " stream=")?;
    write_str_part(sink, record.stream)?;
    write_str_part(sink, " route=")?;
    write_field_text_part(sink, record.route)?;
    write_str_part(sink, " source=")?;
    write_str_part(sink, record.source)?;
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

fn write_hex_usize_to_command_sink(
    sink: &mut impl LocalCommandSink,
    value: usize,
) -> Result<(), LocalCommandExecError> {
    write_hex_u64_to_command_sink(sink, value as u64)
}

fn write_hex_u64_to_command_sink(
    sink: &mut impl LocalCommandSink,
    value: u64,
) -> Result<(), LocalCommandExecError> {
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
        core::str::from_utf8(&bytes).map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
    sink.write_command_str(text)
        .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)
}

const fn task_state_name(state: TaskState) -> &'static str {
    match state {
        TaskState::Running => "running",
        TaskState::Runnable => "runnable",
        TaskState::Blocked => "blocked",
    }
}

fn write_decimal_usize_part(
    sink: &mut impl LocalCommandSink,
    value: usize,
) -> Result<(), LocalCommandCycleError> {
    let mut digits = [0u8; 20];
    let mut value = value;
    let mut index = digits.len();
    loop {
        index -= 1;
        digits[index] = b'0' + (value % 10) as u8;
        value /= 10;
        if value == 0 {
            break;
        }
    }
    let text = core::str::from_utf8(&digits[index..])
        .map_err(|_| LocalCommandCycleError::ResponseWriteFailed)?;
    write_str_part(sink, text)
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

    struct DelayedScriptedInput<const N: usize> {
        bytes: [u8; N],
        len: usize,
        pos: usize,
        delay_after: usize,
        delay_remaining: usize,
    }

    impl<const N: usize> DelayedScriptedInput<N> {
        const fn new(
            bytes: [u8; N],
            len: usize,
            delay_after: usize,
            delay_remaining: usize,
        ) -> Self {
            Self {
                bytes,
                len,
                pos: 0,
                delay_after,
                delay_remaining,
            }
        }
    }

    impl<const N: usize> ConsoleInputBackend for DelayedScriptedInput<N> {
        fn poll_read_byte(&mut self) -> Option<u8> {
            if self.pos >= self.delay_after && self.delay_remaining != 0 {
                self.delay_remaining -= 1;
                return None;
            }
            if self.pos == self.len {
                return None;
            }
            let byte = self.bytes[self.pos];
            self.pos += 1;
            Some(byte)
        }
    }

    struct CaptureSink {
        bytes: [u8; 16384],
        len: usize,
        fail_after: usize,
        writes: usize,
    }

    impl CaptureSink {
        const fn new() -> Self {
            Self {
                bytes: [0; 16384],
                len: 0,
                fail_after: usize::MAX,
                writes: 0,
            }
        }

        const fn failing_after(fail_after: usize) -> Self {
            Self {
                bytes: [0; 16384],
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
        assert_eq!(result.response_lines(), 6);
        assert_eq!(
            backend.as_str(),
            "talos> init\nzero\nstatus42\nstdout\nstdin\nstderr\n"
        );
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
        assert_eq!(bin_ls.response_lines(), 6);
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
		stdout\n\
		stdin\n\
		stderr\n\
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
    fn local_command_loop_execs_userspace_stdout_fixture_through_fd1() {
        let input = ScriptedInput::new(*b"exec stdout\rwaitpid\rlaststatus\r", 31);
        let mut backend = CaptureSink::new();
        let (exec, waited, observed) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            (
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
            )
        };
        let output = backend.as_str();

        assert_eq!(exec.line(), b"exec stdout");
        assert_eq!(exec.status(), LocalCommandStatus::Handled);
        assert_eq!(exec.response_lines(), 10);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(waited.response_lines(), 1);
        assert_eq!(observed.line(), b"laststatus");
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.response_lines(), 1);
        assert!(output.contains("talos> Talos userspace stdout fixture\n"));
        assert!(output.contains("talos: exec path=/bin/stdout source=vfs-open-read\n"));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stdout"
        ));
        assert!(output.contains(
            "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=stdout route=runtime-console0/stdout source=userspace-talos-write\n"
        ));
        assert!(output.contains(
            "talos: exec-lifecycle pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
    }

    #[test_case]
    fn local_command_loop_redirects_child_stdout_to_inherited_stderr_fd() {
        let input = ScriptedInput::new(*b"exec stdout 1>&2\rwaitpid\rexec stdout\r", 37);
        let mut backend = CaptureSink::new();
        let (redirected, waited, normal) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            (
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
            )
        };
        let output = backend.as_str();

        assert_eq!(redirected.line(), b"exec stdout 1>&2");
        assert_eq!(redirected.status(), LocalCommandStatus::Handled);
        assert_eq!(redirected.response_lines(), 11);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(normal.line(), b"exec stdout");
        assert_eq!(normal.status(), LocalCommandStatus::Handled);
        assert_eq!(normal.response_lines(), 10);
        assert!(output.contains("talos> Talos userspace stdout fixture\n"));
        assert!(output.contains("talos: exec path=/bin/stdout source=vfs-open-read\n"));
        assert!(output.contains(
            "talos: exec-redirection op=dup source-fd=0x0000000000000001 target-fd=0x0000000000000002 target-stream=stderr target-route=runtime-console0/stderr child-only=true shell-restored=true source=shell-redirection-1-to-2\n"
        ));
        assert!(output.contains(
            "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=stderr route=runtime-console0/stderr source=userspace-talos-write\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=stdout route=runtime-console0/stdout source=userspace-talos-write\n"
        ));
    }

    #[test_case]
    fn local_command_loop_redirects_child_stdout_to_dev_null_only_for_one_exec() {
        let input = ScriptedInput::new(*b"exec stdout >/dev/null\rwaitpid\rexec stdout\r", 43);
        let mut backend = CaptureSink::new();
        let (redirected, waited, normal) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            (
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
            )
        };
        let output = backend.as_str();

        assert_eq!(redirected.line(), b"exec stdout >/dev/null");
        assert_eq!(redirected.status(), LocalCommandStatus::Handled);
        assert_eq!(redirected.response_lines(), 11);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(normal.line(), b"exec stdout");
        assert_eq!(normal.status(), LocalCommandStatus::Handled);
        assert_eq!(normal.response_lines(), 10);
        assert_eq!(
            output
                .matches("talos> Talos userspace stdout fixture\n")
                .count(),
            1
        );
        assert!(output.contains("talos: exec path=/bin/stdout source=vfs-open-read\n"));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=device fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-redirection op=sink source-fd=0x0000000000000001 target-path=/dev/null target-stream=null-sink target-route=device:/dev/null child-only=true shell-restored=true source=shell-redirection-stdout-dev-null\n"
        ));
        assert!(output.contains(
            "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=null-sink route=device:/dev/null source=userspace-talos-write\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=stdout route=runtime-console0/stdout source=userspace-talos-write\n"
        ));
    }

    #[test_case]
    fn local_command_loop_redirects_child_stdout_to_volatile_regular_file() {
        let bytes =
            *b"exec stdout >/tmp/stdout.txt\rwaitpid\rlaststatus\rcat /tmp/stdout.txt\rexec stdout\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (redirected, waited, observed, readback, normal) = {
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

        assert_eq!(redirected.line(), b"exec stdout >/tmp/stdout.txt");
        assert_eq!(redirected.status(), LocalCommandStatus::Handled);
        assert_eq!(redirected.response_lines(), 11);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.line(), b"laststatus");
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert_eq!(readback.line(), b"cat /tmp/stdout.txt");
        assert_eq!(readback.status(), LocalCommandStatus::Handled);
        assert_eq!(readback.response_lines(), 2);
        assert_eq!(normal.line(), b"exec stdout");
        assert_eq!(normal.status(), LocalCommandStatus::Handled);
        assert_eq!(normal.response_lines(), 10);
        assert_eq!(
            output
                .matches("talos> Talos userspace stdout fixture\n")
                .count(),
            2
        );
        assert!(output.contains("talos: exec path=/bin/stdout source=vfs-open-read\n"));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=regular-file fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-redirection op=sink source-fd=0x0000000000000001 target-path=/tmp/stdout.txt target-stream=regular-file target-route=volatile-vfs:/tmp/stdout.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout\n"
        ));
        assert!(output.contains(
            "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=regular-file route=volatile-vfs:/tmp/stdout.txt source=userspace-talos-write\n"
        ));
        assert!(output.contains(
            "talos: cat path=/tmp/stdout.txt bytes=0x000000000000001f source=volatile-vfs-descriptor-read\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=stdout route=runtime-console0/stdout source=userspace-talos-write\n"
        ));
    }

    #[test_case]
    fn local_command_loop_appends_child_stdout_to_existing_volatile_regular_file() {
        let bytes = *b"exec stdout >/tmp/stdout.txt\rexec stdout >>/tmp/stdout.txt\rwaitpid\rlaststatus\rcat /tmp/stdout.txt\rexec stdout\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (created, appended, waited, observed, readback, normal) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            (
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
            )
        };
        let output = backend.as_str();

        assert_eq!(created.line(), b"exec stdout >/tmp/stdout.txt");
        assert_eq!(created.status(), LocalCommandStatus::Handled);
        assert_eq!(appended.line(), b"exec stdout >>/tmp/stdout.txt");
        assert_eq!(appended.status(), LocalCommandStatus::Handled);
        assert_eq!(appended.response_lines(), 11);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.line(), b"laststatus");
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert_eq!(readback.line(), b"cat /tmp/stdout.txt");
        assert_eq!(readback.status(), LocalCommandStatus::Handled);
        assert_eq!(normal.line(), b"exec stdout");
        assert_eq!(normal.status(), LocalCommandStatus::Handled);
        assert_eq!(
            output.matches("Talos userspace stdout fixture\n").count(),
            3
        );
        assert!(output.contains(
            "talos: exec-redirection op=sink source-fd=0x0000000000000001 target-path=/tmp/stdout.txt target-stream=regular-file target-route=volatile-vfs:/tmp/stdout.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout\n"
        ));
        assert!(output.contains(
            "talos: exec-redirection op=append source-fd=0x0000000000000001 target-path=/tmp/stdout.txt target-stream=regular-file target-route=volatile-vfs:/tmp/stdout.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout-append\n"
        ));
        assert!(output.contains(
            "talos: cat path=/tmp/stdout.txt bytes=0x000000000000003e source=volatile-vfs-descriptor-read\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=stdout route=runtime-console0/stdout source=userspace-talos-write\n"
        ));
    }

    #[test_case]
    fn local_command_loop_append_creates_missing_stdout_volatile_regular_file() {
        let bytes = *b"exec stdout >>/tmp/stdout.txt\rwaitpid\rlaststatus\rcat /tmp/stdout.txt\rexec stdout\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (appended, waited, observed, readback, normal) = {
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

        assert_eq!(appended.line(), b"exec stdout >>/tmp/stdout.txt");
        assert_eq!(appended.status(), LocalCommandStatus::Handled);
        assert_eq!(appended.response_lines(), 11);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.line(), b"laststatus");
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert_eq!(readback.line(), b"cat /tmp/stdout.txt");
        assert_eq!(readback.status(), LocalCommandStatus::Handled);
        assert_eq!(readback.response_lines(), 2);
        assert_eq!(normal.line(), b"exec stdout");
        assert_eq!(normal.status(), LocalCommandStatus::Handled);
        assert_eq!(
            output.matches("Talos userspace stdout fixture\n").count(),
            2
        );
        assert!(output.contains(
            "talos: exec-redirection op=append source-fd=0x0000000000000001 target-path=/tmp/stdout.txt target-stream=regular-file target-route=volatile-vfs:/tmp/stdout.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout-append\n"
        ));
        assert!(output.contains(
            "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=regular-file route=volatile-vfs:/tmp/stdout.txt source=userspace-talos-write\n"
        ));
        assert!(output.contains(
            "talos: cat path=/tmp/stdout.txt bytes=0x000000000000001f source=volatile-vfs-descriptor-read\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=stdout route=runtime-console0/stdout source=userspace-talos-write\n"
        ));
    }

    #[test_case]
    fn local_command_loop_accepts_explicit_fd1_stdout_regular_file_aliases() {
        let bytes = *b"exec stdout 1>/tmp/stdout.txt\rwaitpid\rlaststatus\rcat /tmp/stdout.txt\rexec stdout\rexec stdout 1>>/tmp/stdout.txt\rwaitpid\rlaststatus\rcat /tmp/stdout.txt\rexec stdout\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (
            truncated,
            waited,
            observed,
            first_readback,
            normal,
            appended,
            append_waited,
            append_observed,
            second_readback,
            restored,
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
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
            )
        };
        let output = backend.as_str();

        assert_eq!(truncated.line(), b"exec stdout 1>/tmp/stdout.txt");
        assert_eq!(truncated.status(), LocalCommandStatus::Handled);
        assert_eq!(truncated.response_lines(), 11);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.line(), b"laststatus");
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert_eq!(first_readback.line(), b"cat /tmp/stdout.txt");
        assert_eq!(first_readback.status(), LocalCommandStatus::Handled);
        assert_eq!(normal.line(), b"exec stdout");
        assert_eq!(normal.status(), LocalCommandStatus::Handled);
        assert_eq!(appended.line(), b"exec stdout 1>>/tmp/stdout.txt");
        assert_eq!(appended.status(), LocalCommandStatus::Handled);
        assert_eq!(appended.response_lines(), 11);
        assert_eq!(append_waited.line(), b"waitpid");
        assert_eq!(append_waited.status(), LocalCommandStatus::Handled);
        assert_eq!(append_observed.line(), b"laststatus");
        assert_eq!(append_observed.status(), LocalCommandStatus::Handled);
        assert_eq!(second_readback.line(), b"cat /tmp/stdout.txt");
        assert_eq!(second_readback.status(), LocalCommandStatus::Handled);
        assert_eq!(restored.line(), b"exec stdout");
        assert_eq!(restored.status(), LocalCommandStatus::Handled);
        assert!(output.contains(
            "talos: exec-redirection op=sink source-fd=0x0000000000000001 target-path=/tmp/stdout.txt target-stream=regular-file target-route=volatile-vfs:/tmp/stdout.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout\n"
        ));
        assert!(output.contains(
            "talos: exec-redirection op=append source-fd=0x0000000000000001 target-path=/tmp/stdout.txt target-stream=regular-file target-route=volatile-vfs:/tmp/stdout.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout-append\n"
        ));
        assert!(output.contains(
            "talos: cat path=/tmp/stdout.txt bytes=0x000000000000001f source=volatile-vfs-descriptor-read\n"
        ));
        assert!(output.contains(
            "talos: cat path=/tmp/stdout.txt bytes=0x000000000000003e source=volatile-vfs-descriptor-read\n"
        ));
        assert!(output.contains(
            "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=stdout route=runtime-console0/stdout source=userspace-talos-write\n"
        ));
    }

    #[test_case]
    fn local_command_loop_redirects_stdout_to_arbitrary_tmp_basenames() {
        let bytes = *b"exec stdout >/tmp/alpha.log\rwaitpid\rlaststatus\rcat /tmp/alpha.log\rexec stdout 1>>/tmp/beta.out\rwaitpid\rlaststatus\rcat /tmp/beta.out\rexec stdout\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (
            truncated,
            first_waited,
            first_observed,
            first_readback,
            appended,
            second_waited,
            second_observed,
            second_readback,
            normal,
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
        let output = backend.as_str();

        assert_eq!(truncated.line(), b"exec stdout >/tmp/alpha.log");
        assert_eq!(truncated.status(), LocalCommandStatus::Handled);
        assert_eq!(first_waited.line(), b"waitpid");
        assert_eq!(first_waited.status(), LocalCommandStatus::Handled);
        assert_eq!(first_observed.line(), b"laststatus");
        assert_eq!(first_observed.status(), LocalCommandStatus::Handled);
        assert_eq!(first_readback.line(), b"cat /tmp/alpha.log");
        assert_eq!(first_readback.status(), LocalCommandStatus::Handled);
        assert_eq!(appended.line(), b"exec stdout 1>>/tmp/beta.out");
        assert_eq!(appended.status(), LocalCommandStatus::Handled);
        assert_eq!(second_waited.line(), b"waitpid");
        assert_eq!(second_waited.status(), LocalCommandStatus::Handled);
        assert_eq!(second_observed.line(), b"laststatus");
        assert_eq!(second_observed.status(), LocalCommandStatus::Handled);
        assert_eq!(second_readback.line(), b"cat /tmp/beta.out");
        assert_eq!(second_readback.status(), LocalCommandStatus::Handled);
        assert_eq!(normal.line(), b"exec stdout");
        assert_eq!(normal.status(), LocalCommandStatus::Handled);
        assert!(output.contains(
            "talos: exec-redirection op=sink source-fd=0x0000000000000001 target-path=/tmp/alpha.log target-stream=regular-file target-route=volatile-vfs:/tmp/alpha.log child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout\n"
        ));
        assert!(output.contains(
            "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=regular-file route=volatile-vfs:/tmp/alpha.log source=userspace-talos-write\n"
        ));
        assert!(output.contains(
            "talos: cat path=/tmp/alpha.log bytes=0x000000000000001f source=volatile-vfs-descriptor-read\n"
        ));
        assert!(output.contains(
            "talos: exec-redirection op=append source-fd=0x0000000000000001 target-path=/tmp/beta.out target-stream=regular-file target-route=volatile-vfs:/tmp/beta.out child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout-append\n"
        ));
        assert!(output.contains(
            "talos: cat path=/tmp/beta.out bytes=0x000000000000001f source=volatile-vfs-descriptor-read\n"
        ));
        assert!(output.contains(
            "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=stdout route=runtime-console0/stdout source=userspace-talos-write\n"
        ));
    }

    #[test_case]
    fn local_command_loop_redirects_stderr_to_arbitrary_tmp_basenames() {
        let bytes = *b"exec stderr 2>/tmp/omega.err\rwaitpid\rlaststatus\rcat /tmp/omega.err\rexec stderr 2>>/tmp/theta.log\rwaitpid\rlaststatus\rcat /tmp/theta.log\rexec stderr\rexec stdout\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (
            truncated,
            first_waited,
            first_observed,
            first_readback,
            appended,
            second_waited,
            second_observed,
            second_readback,
            normal_stderr,
            normal_stdout,
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
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
            )
        };
        let output = backend.as_str();

        assert_eq!(truncated.line(), b"exec stderr 2>/tmp/omega.err");
        assert_eq!(truncated.status(), LocalCommandStatus::Handled);
        assert_eq!(first_waited.line(), b"waitpid");
        assert_eq!(first_waited.status(), LocalCommandStatus::Handled);
        assert_eq!(first_observed.line(), b"laststatus");
        assert_eq!(first_observed.status(), LocalCommandStatus::Handled);
        assert_eq!(first_readback.line(), b"cat /tmp/omega.err");
        assert_eq!(first_readback.status(), LocalCommandStatus::Handled);
        assert_eq!(appended.line(), b"exec stderr 2>>/tmp/theta.log");
        assert_eq!(appended.status(), LocalCommandStatus::Handled);
        assert_eq!(second_waited.line(), b"waitpid");
        assert_eq!(second_waited.status(), LocalCommandStatus::Handled);
        assert_eq!(second_observed.line(), b"laststatus");
        assert_eq!(second_observed.status(), LocalCommandStatus::Handled);
        assert_eq!(second_readback.line(), b"cat /tmp/theta.log");
        assert_eq!(second_readback.status(), LocalCommandStatus::Handled);
        assert_eq!(normal_stderr.line(), b"exec stderr");
        assert_eq!(normal_stderr.status(), LocalCommandStatus::Handled);
        assert_eq!(normal_stdout.line(), b"exec stdout");
        assert_eq!(normal_stdout.status(), LocalCommandStatus::Handled);
        assert!(output.contains(
            "talos: exec-redirection op=sink source-fd=0x0000000000000002 target-path=/tmp/omega.err target-stream=regular-file target-route=volatile-vfs:/tmp/omega.err child-only=true shell-restored=true source=shell-redirection-stderr-tmp-stderr\n"
        ));
        assert!(output.contains(
            "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=regular-file route=volatile-vfs:/tmp/omega.err source=userspace-talos-write\n"
        ));
        assert!(output.contains(
            "talos: cat path=/tmp/omega.err bytes=0x000000000000001f source=volatile-vfs-descriptor-read\n"
        ));
        assert!(output.contains(
            "talos: exec-redirection op=append source-fd=0x0000000000000002 target-path=/tmp/theta.log target-stream=regular-file target-route=volatile-vfs:/tmp/theta.log child-only=true shell-restored=true source=shell-redirection-stderr-tmp-stderr-append\n"
        ));
        assert!(output.contains(
            "talos: cat path=/tmp/theta.log bytes=0x000000000000001f source=volatile-vfs-descriptor-read\n"
        ));
        assert!(output.contains(
            "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=stderr route=runtime-console0/stderr source=userspace-talos-write\n"
        ));
        assert!(output.contains(
            "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=stdout route=runtime-console0/stdout source=userspace-talos-write\n"
        ));
    }

    #[test_case]
    fn local_command_loop_rejects_unsupported_explicit_file_output_aliases() {
        let bytes = *b"exec stdout 3>/tmp/stdout.txt\rexec stdout 1>/var/other.txt\rexec stdout 1>>/tmp/nested/x.txt\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (unsupported_fd, unsupported_truncate_path, unsupported_append_path) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            (
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
            )
        };
        let output = backend.as_str();

        assert_eq!(unsupported_fd.line(), b"exec stdout 3>/tmp/stdout.txt");
        assert_eq!(
            unsupported_fd.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert_eq!(
            unsupported_truncate_path.line(),
            b"exec stdout 1>/var/other.txt"
        );
        assert_eq!(
            unsupported_truncate_path.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert_eq!(
            unsupported_append_path.line(),
            b"exec stdout 1>>/tmp/nested/x.txt"
        );
        assert_eq!(
            unsupported_append_path.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert_eq!(output.matches("talos: exec-invalid-path\n").count(), 3);
    }

    #[test_case]
    fn local_command_loop_redirects_child_stderr_to_volatile_regular_file() {
        let bytes = *b"exec stderr 2>/tmp/stderr.txt\rwaitpid\rlaststatus\rcat /tmp/stderr.txt\rexec stderr\rexec stdout\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (redirected, waited, observed, readback, normal_stderr, normal_stdout) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            (
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
            )
        };
        let output = backend.as_str();

        assert_eq!(redirected.line(), b"exec stderr 2>/tmp/stderr.txt");
        assert_eq!(redirected.status(), LocalCommandStatus::Handled);
        assert_eq!(redirected.response_lines(), 11);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.line(), b"laststatus");
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert_eq!(readback.line(), b"cat /tmp/stderr.txt");
        assert_eq!(readback.status(), LocalCommandStatus::Handled);
        assert_eq!(readback.response_lines(), 2);
        assert_eq!(normal_stderr.line(), b"exec stderr");
        assert_eq!(normal_stderr.status(), LocalCommandStatus::Handled);
        assert_eq!(normal_stderr.response_lines(), 10);
        assert_eq!(normal_stdout.line(), b"exec stdout");
        assert_eq!(normal_stdout.status(), LocalCommandStatus::Handled);
        assert_eq!(normal_stdout.response_lines(), 10);
        assert_eq!(
            output
                .matches("talos> Talos userspace stderr fixture\n")
                .count(),
            2
        );
        assert_eq!(
            output
                .matches("talos> Talos userspace stdout fixture\n")
                .count(),
            1
        );
        assert!(output.contains("talos: exec path=/bin/stderr source=vfs-open-read\n"));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=stdio-output fd2=regular-file loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-redirection op=sink source-fd=0x0000000000000002 target-path=/tmp/stderr.txt target-stream=regular-file target-route=volatile-vfs:/tmp/stderr.txt child-only=true shell-restored=true source=shell-redirection-stderr-tmp-stderr\n"
        ));
        assert!(output.contains(
            "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=regular-file route=volatile-vfs:/tmp/stderr.txt source=userspace-talos-write\n"
        ));
        assert!(output.contains(
            "talos: cat path=/tmp/stderr.txt bytes=0x000000000000001f source=volatile-vfs-descriptor-read\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=stderr route=runtime-console0/stderr source=userspace-talos-write\n"
        ));
        assert!(output.contains(
            "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=stdout route=runtime-console0/stdout source=userspace-talos-write\n"
        ));
    }

    #[test_case]
    fn local_command_loop_appends_child_stderr_to_existing_volatile_regular_file() {
        let bytes = *b"exec stderr 2>/tmp/stderr.txt\rexec stderr 2>>/tmp/stderr.txt\rwaitpid\rlaststatus\rcat /tmp/stderr.txt\rexec stderr\rexec stdout\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (created, appended, waited, observed, readback, normal_stderr, normal_stdout) = {
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
        let output = backend.as_str();

        assert_eq!(created.line(), b"exec stderr 2>/tmp/stderr.txt");
        assert_eq!(created.status(), LocalCommandStatus::Handled);
        assert_eq!(appended.line(), b"exec stderr 2>>/tmp/stderr.txt");
        assert_eq!(appended.status(), LocalCommandStatus::Handled);
        assert_eq!(appended.response_lines(), 11);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.line(), b"laststatus");
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert_eq!(readback.line(), b"cat /tmp/stderr.txt");
        assert_eq!(readback.status(), LocalCommandStatus::Handled);
        assert_eq!(normal_stderr.line(), b"exec stderr");
        assert_eq!(normal_stderr.status(), LocalCommandStatus::Handled);
        assert_eq!(normal_stdout.line(), b"exec stdout");
        assert_eq!(normal_stdout.status(), LocalCommandStatus::Handled);
        assert_eq!(
            output.matches("Talos userspace stderr fixture\n").count(),
            3
        );
        assert_eq!(
            output
                .matches("talos> Talos userspace stdout fixture\n")
                .count(),
            1
        );
        assert!(output.contains(
            "talos: exec-redirection op=sink source-fd=0x0000000000000002 target-path=/tmp/stderr.txt target-stream=regular-file target-route=volatile-vfs:/tmp/stderr.txt child-only=true shell-restored=true source=shell-redirection-stderr-tmp-stderr\n"
        ));
        assert!(output.contains(
            "talos: exec-redirection op=append source-fd=0x0000000000000002 target-path=/tmp/stderr.txt target-stream=regular-file target-route=volatile-vfs:/tmp/stderr.txt child-only=true shell-restored=true source=shell-redirection-stderr-tmp-stderr-append\n"
        ));
        assert!(output.contains(
            "talos: cat path=/tmp/stderr.txt bytes=0x000000000000003e source=volatile-vfs-descriptor-read\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=stderr route=runtime-console0/stderr source=userspace-talos-write\n"
        ));
        assert!(output.contains(
            "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=stdout route=runtime-console0/stdout source=userspace-talos-write\n"
        ));
    }

    #[test_case]
    fn local_command_loop_append_creates_missing_stderr_volatile_regular_file() {
        let bytes = *b"exec stderr 2>>/tmp/stderr.txt\rwaitpid\rlaststatus\rcat /tmp/stderr.txt\rexec stderr\rexec stdout\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (appended, waited, observed, readback, normal_stderr, normal_stdout) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            (
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
            )
        };
        let output = backend.as_str();

        assert_eq!(appended.line(), b"exec stderr 2>>/tmp/stderr.txt");
        assert_eq!(appended.status(), LocalCommandStatus::Handled);
        assert_eq!(appended.response_lines(), 11);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.line(), b"laststatus");
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert_eq!(readback.line(), b"cat /tmp/stderr.txt");
        assert_eq!(readback.status(), LocalCommandStatus::Handled);
        assert_eq!(readback.response_lines(), 2);
        assert_eq!(normal_stderr.line(), b"exec stderr");
        assert_eq!(normal_stderr.status(), LocalCommandStatus::Handled);
        assert_eq!(normal_stdout.line(), b"exec stdout");
        assert_eq!(normal_stdout.status(), LocalCommandStatus::Handled);
        assert_eq!(
            output.matches("Talos userspace stderr fixture\n").count(),
            2
        );
        assert_eq!(
            output
                .matches("talos> Talos userspace stdout fixture\n")
                .count(),
            1
        );
        assert!(output.contains(
            "talos: exec-redirection op=append source-fd=0x0000000000000002 target-path=/tmp/stderr.txt target-stream=regular-file target-route=volatile-vfs:/tmp/stderr.txt child-only=true shell-restored=true source=shell-redirection-stderr-tmp-stderr-append\n"
        ));
        assert!(output.contains(
            "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=regular-file route=volatile-vfs:/tmp/stderr.txt source=userspace-talos-write\n"
        ));
        assert!(output.contains(
            "talos: cat path=/tmp/stderr.txt bytes=0x000000000000001f source=volatile-vfs-descriptor-read\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=stderr route=runtime-console0/stderr source=userspace-talos-write\n"
        ));
        assert!(output.contains(
            "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=stdout route=runtime-console0/stdout source=userspace-talos-write\n"
        ));
    }

    #[test_case]
    fn local_command_loop_redirects_child_stderr_to_dev_null_only_for_one_exec() {
        let bytes = *b"exec stderr 2>/dev/null\rwaitpid\rexec stderr\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (redirected, waited, normal) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            (
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
            )
        };
        let output = backend.as_str();

        assert_eq!(redirected.line(), b"exec stderr 2>/dev/null");
        assert_eq!(redirected.status(), LocalCommandStatus::Handled);
        assert_eq!(redirected.response_lines(), 11);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(normal.line(), b"exec stderr");
        assert_eq!(normal.status(), LocalCommandStatus::Handled);
        assert_eq!(normal.response_lines(), 10);
        assert_eq!(
            output
                .matches("talos> Talos userspace stderr fixture\n")
                .count(),
            1
        );
        assert!(output.contains("talos: exec path=/bin/stderr source=vfs-open-read\n"));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=stdio-output fd2=device loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-redirection op=sink source-fd=0x0000000000000002 target-path=/dev/null target-stream=null-sink target-route=device:/dev/null child-only=true shell-restored=true source=shell-redirection-stderr-dev-null\n"
        ));
        assert!(output.contains(
            "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=null-sink route=device:/dev/null source=userspace-talos-write\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=stderr route=runtime-console0/stderr source=userspace-talos-write\n"
        ));
    }

    #[test_case]
    fn local_command_loop_redirects_child_stdin_from_dev_null_only_for_one_exec() {
        let bytes = *b"exec stdin </dev/null\rwaitpid\rexec stdin\rtalos-console0";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (redirected, waited, normal) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            (
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
            )
        };
        let output = backend.as_str();

        assert_eq!(redirected.line(), b"exec stdin </dev/null");
        assert_eq!(redirected.status(), LocalCommandStatus::Handled);
        assert_eq!(redirected.response_lines(), 11);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(normal.line(), b"exec stdin");
        assert_eq!(normal.status(), LocalCommandStatus::Handled);
        assert_eq!(normal.response_lines(), 10);
        assert!(output.contains("talos: exec path=/bin/stdin source=vfs-open-read\n"));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=device fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-redirection op=source source-fd=0x0000000000000000 source-path=/dev/null source-stream=null-source source-route=device:/dev/null child-only=true shell-restored=true source=shell-redirection-stdin-dev-null\n"
        ));
        assert!(
            output.contains("Talos userspace stdin fixture read-result: null-source-eof/no-data\n")
        );
        assert!(output.contains(
            "talos: exec-stdin fd=0x0000000000000000 bytes=0x0000000000000000 return=0x0000000000000000 read-source=device:/dev/null stdout-fd=0x0000000000000001 stdout-bytes=0x0000000000000043 stdout-return=0x0000000000000043 source=userspace-talos-read+userspace-talos-write read-result=null-source-eof/no-data\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains("Talos userspace stdin fixture read: talos-console0\n"));
        assert!(output.contains(
            "talos: exec-stdin fd=0x0000000000000000 bytes=0x000000000000000e return=0x000000000000000e read-source=runtime-console0/local-input stdout-fd=0x0000000000000001 stdout-bytes=0x0000000000000033 stdout-return=0x0000000000000033 source=userspace-talos-read+userspace-talos-write\n"
        ));
    }

    #[test_case]
    fn local_command_loop_redirects_child_stdin_from_readonly_regular_file_only_for_one_exec() {
        let bytes = *b"exec stdin </etc/banner.txt\rwaitpid\rexec stdin\rtalos-console0";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (redirected, waited, normal) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            (
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
            )
        };
        let output = backend.as_str();

        assert_eq!(redirected.line(), b"exec stdin </etc/banner.txt");
        assert_eq!(redirected.status(), LocalCommandStatus::Handled);
        assert_eq!(redirected.response_lines(), 11);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(normal.line(), b"exec stdin");
        assert_eq!(normal.status(), LocalCommandStatus::Handled);
        assert_eq!(normal.response_lines(), 10);
        assert!(output.contains("talos: exec path=/bin/stdin source=vfs-open-read\n"));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=regular-file fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-redirection op=source source-fd=0x0000000000000000 source-path=/etc/banner.txt source-stream=regular-file source-route=initramfs:/etc/banner.txt child-only=true shell-restored=true source=shell-redirection-stdin-etc-banner\n"
        ));
        assert!(output.contains("Talos userspace stdin fixture read: Talos initramfs fixture\n"));
        assert!(output.contains(
            "talos: exec-stdin fd=0x0000000000000000 bytes=0x0000000000000018 return=0x0000000000000018 read-source=initramfs:/etc/banner.txt stdout-fd=0x0000000000000001 stdout-bytes=0x000000000000003d stdout-return=0x000000000000003d source=userspace-talos-read+userspace-talos-write read-result=regular-file-eof-after-read\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains("Talos userspace stdin fixture read: talos-console0\n"));
        assert!(output.contains(
            "talos: exec-stdin fd=0x0000000000000000 bytes=0x000000000000000e return=0x000000000000000e read-source=runtime-console0/local-input stdout-fd=0x0000000000000001 stdout-bytes=0x0000000000000033 stdout-return=0x0000000000000033 source=userspace-talos-read+userspace-talos-write\n"
        ));
    }

    #[test_case]
    fn local_command_loop_combines_readonly_stdin_and_volatile_stdout_redirection() {
        let bytes = *b"exec stdin </etc/banner.txt >/tmp/stdin-report.txt\rwaitpid\rlaststatus\rcat /tmp/stdin-report.txt\rexec stdin >/tmp/stdin-report.txt </etc/banner.txt\rexec stdin </dev/null >/tmp/stdin-report.txt\rexec stdin </etc/banner.txt 1>/tmp/stdin-report.txt\rexec stdin < /etc/banner.txt >/tmp/stdin-report.txt\rexec stdin </etc/banner.txt >/tmp/nested/out.txt\rexec stdin\rtalos-console0";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (
            redirected,
            waited,
            observed,
            readback,
            output_first,
            dev_null_input,
            explicit_fd1,
            spaced_input,
            nested_output,
            normal,
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
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
            )
        };
        let output = backend.as_str();

        assert_eq!(
            redirected.line(),
            b"exec stdin </etc/banner.txt >/tmp/stdin-report.txt"
        );
        assert_eq!(redirected.status(), LocalCommandStatus::Handled);
        assert_eq!(redirected.response_lines(), 12);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.line(), b"laststatus");
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert_eq!(readback.line(), b"cat /tmp/stdin-report.txt");
        assert_eq!(readback.status(), LocalCommandStatus::Handled);
        assert_eq!(readback.response_lines(), 2);
        assert_eq!(normal.line(), b"exec stdin");
        assert_eq!(normal.status(), LocalCommandStatus::Handled);
        assert_eq!(normal.response_lines(), 10);
        for rejected in [
            output_first,
            dev_null_input,
            explicit_fd1,
            spaced_input,
            nested_output,
        ] {
            assert_eq!(rejected.status(), LocalCommandStatus::UnexpectedArgument);
            assert_eq!(rejected.response_lines(), 1);
        }
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=regular-file fd1=regular-file fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-redirection op=source source-fd=0x0000000000000000 source-path=/etc/banner.txt source-stream=regular-file source-route=initramfs:/etc/banner.txt child-only=true shell-restored=true source=shell-redirection-stdin-etc-banner\n"
        ));
        assert!(output.contains(
            "talos: exec-redirection op=sink source-fd=0x0000000000000001 target-path=/tmp/stdin-report.txt target-stream=regular-file target-route=volatile-vfs:/tmp/stdin-report.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout\n"
        ));
        assert!(output.contains(
            "talos: exec-stdin fd=0x0000000000000000 bytes=0x0000000000000018 return=0x0000000000000018 read-source=initramfs:/etc/banner.txt stdout-fd=0x0000000000000001 stdout-bytes=0x000000000000003d stdout-return=0x000000000000003d source=userspace-talos-read+userspace-talos-write read-result=regular-file-eof-after-read\n"
        ));
        assert!(output.contains(
            "talos: cat path=/tmp/stdin-report.txt bytes=0x000000000000003d source=volatile-vfs-descriptor-read\n"
        ));
        assert!(output.contains("Talos userspace stdin fixture read: Talos initramfs fixture\n"));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains("Talos userspace stdin fixture read: talos-console0\n"));
        assert!(output.contains(
            "talos: exec-stdin fd=0x0000000000000000 bytes=0x000000000000000e return=0x000000000000000e read-source=runtime-console0/local-input stdout-fd=0x0000000000000001 stdout-bytes=0x0000000000000033 stdout-return=0x0000000000000033 source=userspace-talos-read+userspace-talos-write\n"
        ));
    }

    #[test_case]
    fn local_command_loop_redirects_child_stderr_to_inherited_stdout_fd() {
        let input = ScriptedInput::new(*b"exec stderr 2>&1\rwaitpid\rexec stderr\r", 37);
        let mut backend = CaptureSink::new();
        let (redirected, waited, normal) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            (
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
            )
        };
        let output = backend.as_str();

        assert_eq!(redirected.line(), b"exec stderr 2>&1");
        assert_eq!(redirected.status(), LocalCommandStatus::Handled);
        assert_eq!(redirected.response_lines(), 11);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(normal.line(), b"exec stderr");
        assert_eq!(normal.status(), LocalCommandStatus::Handled);
        assert_eq!(normal.response_lines(), 10);
        assert!(output.contains("talos> Talos userspace stderr fixture\n"));
        assert!(output.contains("talos: exec path=/bin/stderr source=vfs-open-read\n"));
        assert!(output.contains(
            "talos: exec-redirection op=dup source-fd=0x0000000000000002 target-fd=0x0000000000000001 target-stream=stdout target-route=runtime-console0/stdout child-only=true shell-restored=true source=shell-redirection-2-to-1\n"
        ));
        assert!(output.contains(
            "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=stdout route=runtime-console0/stdout source=userspace-talos-write\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=stderr route=runtime-console0/stderr source=userspace-talos-write\n"
        ));
    }

    #[test_case]
    fn local_command_loop_closes_child_stdout_only_for_one_exec() {
        let input = ScriptedInput::new(*b"exec stdout 1>&-\rwaitpid\rexec stdout\r", 37);
        let mut backend = CaptureSink::new();
        let (redirected, waited, normal) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            (
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
            )
        };
        let output = backend.as_str();

        assert_eq!(redirected.line(), b"exec stdout 1>&-");
        assert_eq!(redirected.status(), LocalCommandStatus::Handled);
        assert_eq!(redirected.response_lines(), 11);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(normal.line(), b"exec stdout");
        assert_eq!(normal.status(), LocalCommandStatus::Handled);
        assert_eq!(normal.response_lines(), 10);
        assert!(output.contains("talos: exec path=/bin/stdout source=vfs-open-read\n"));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000002 fd0=stdio-input fd1=closed fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-redirection op=close source-fd=0x0000000000000001 result=closed-descriptor child-only=true shell-restored=true source=shell-redirection-1-close\n"
        ));
        assert!(output.contains(
            "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0xfffffffffffffff7 stream=closed route=closed-descriptor source=userspace-talos-write\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=stdout route=runtime-console0/stdout source=userspace-talos-write\n"
        ));
    }

    #[test_case]
    fn local_command_loop_closes_child_stderr_only_for_one_exec() {
        let input = ScriptedInput::new(*b"exec stderr 2>&-\rwaitpid\rexec stderr\r", 37);
        let mut backend = CaptureSink::new();
        let (redirected, waited, normal) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            (
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
            )
        };
        let output = backend.as_str();

        assert_eq!(redirected.line(), b"exec stderr 2>&-");
        assert_eq!(redirected.status(), LocalCommandStatus::Handled);
        assert_eq!(redirected.response_lines(), 11);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(normal.line(), b"exec stderr");
        assert_eq!(normal.status(), LocalCommandStatus::Handled);
        assert_eq!(normal.response_lines(), 10);
        assert!(output.contains("talos: exec path=/bin/stderr source=vfs-open-read\n"));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000002 fd0=stdio-input fd1=stdio-output fd2=closed loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-redirection op=close source-fd=0x0000000000000002 result=closed-descriptor child-only=true shell-restored=true source=shell-redirection-2-close\n"
        ));
        assert!(output.contains(
            "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0xfffffffffffffff7 stream=closed route=closed-descriptor source=userspace-talos-write\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=stderr route=runtime-console0/stderr source=userspace-talos-write\n"
        ));
    }

    #[test_case]
    fn local_command_loop_rejects_unsupported_redirection_forms() {
        let bytes = *b"exec stdout 2>&3\rexec stdout 1>file\rexec stdout | stderr\rexec stderr 2>file\rexec stdout >>/tmp/stderr.txt\rexec stderr 2>>/tmp/stdout.txt\rexec stdout >/var/other.txt\rexec stderr 2>/tmp/stdout.txt\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (
            bad_descriptor,
            file,
            pipe,
            stderr_file,
            append,
            stderr_append,
            other_path,
            stderr_regular_file,
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
            )
        };
        let output = backend.as_str();

        assert_eq!(bad_descriptor.line(), b"exec stdout 2>&3");
        assert_eq!(
            bad_descriptor.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert_eq!(file.line(), b"exec stdout 1>file");
        assert_eq!(file.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(pipe.line(), b"exec stdout | stderr");
        assert_eq!(pipe.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(stderr_file.line(), b"exec stderr 2>file");
        assert_eq!(stderr_file.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(append.line(), b"exec stdout >>/tmp/stderr.txt");
        assert_eq!(append.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(stderr_append.line(), b"exec stderr 2>>/tmp/stdout.txt");
        assert_eq!(
            stderr_append.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert_eq!(other_path.line(), b"exec stdout >/var/other.txt");
        assert_eq!(other_path.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(stderr_regular_file.line(), b"exec stderr 2>/tmp/stdout.txt");
        assert_eq!(
            stderr_regular_file.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert_eq!(
            output,
            "talos> talos: exec-invalid-path\n\
talos> talos: exec-invalid-path\n\
talos> talos: exec-invalid-path\n\
talos> talos: exec-invalid-path\n\
talos> talos: exec-invalid-path\n\
talos> talos: exec-invalid-path\n\
talos> talos: exec-invalid-path\n\
talos> talos: exec-invalid-path\n"
        );
    }

    #[test_case]
    fn local_command_loop_execs_minimal_stdout_to_stdin_pipeline() {
        let input = ScriptedInput::new(*b"exec stdout | exec stdin\rwaitpid\rlaststatus\r", 44);
        let mut backend = CaptureSink::new();
        let (pipeline, waited, observed) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            (
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
            )
        };
        let output = backend.as_str();

        assert_eq!(pipeline.line(), b"exec stdout | exec stdin");
        assert_eq!(pipeline.status(), LocalCommandStatus::Handled);
        assert_eq!(pipeline.response_lines(), 21);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.line(), b"laststatus");
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert!(output.contains(
            "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdout consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000001f bytes-read=0x000000000000001f writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-stdout-to-stdin\n"
        ));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=pipe-endpoint fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=pipe-writer route=pipe:stdout-to-stdin source=userspace-talos-write\n"
        ));
        assert!(output.contains(
            "talos> Talos userspace stdin fixture read: Talos userspace stdout fixture\n"
        ));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=pipe-endpoint fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-stdin fd=0x0000000000000000 bytes=0x000000000000001f return=0x000000000000001f read-source=pipe:stdout-to-stdin stdout-fd=0x0000000000000001 stdout-bytes=0x0000000000000044 stdout-return=0x0000000000000044 source=userspace-talos-read+userspace-talos-write read-result=pipe-eof-after-writer-close\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
    }

    #[test_case]
    fn local_command_loop_keeps_stderr_out_of_stdout_pipeline() {
        let input = ScriptedInput::new(*b"exec stderr | exec stdin\rwaitpid\rlaststatus\r", 44);
        let mut backend = CaptureSink::new();
        let (pipeline, waited, observed) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            (
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
            )
        };
        let output = backend.as_str();

        assert_eq!(pipeline.line(), b"exec stderr | exec stdin");
        assert_eq!(pipeline.status(), LocalCommandStatus::Handled);
        assert_eq!(pipeline.response_lines(), 21);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.line(), b"laststatus");
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert!(output.contains(
            "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stderr consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x0000000000000000 bytes-read=0x0000000000000000 writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-stdout-only-stderr-not-piped\n"
        ));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=pipe-endpoint fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=stderr route=runtime-console0/stderr source=userspace-talos-write\n"
        ));
        assert!(output.contains("talos> Talos userspace stderr fixture\n"));
        assert!(output.contains("Talos userspace stdin fixture read-result: pipe-eof/no-data\n"));
        assert!(output.contains(
            "talos: exec-stdin fd=0x0000000000000000 bytes=0x0000000000000000 return=0x0000000000000000 read-source=pipe:stdout-to-stdin stdout-fd=0x0000000000000001 stdout-bytes=0x000000000000003c stdout-return=0x000000000000003c source=userspace-talos-read+userspace-talos-write read-result=pipe-eof/no-data\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
    }

    #[test_case]
    fn local_command_loop_pipes_child_stderr_after_dup_to_stdout() {
        let bytes = *b"exec stderr 2>&1 | exec stdin\rwaitpid\rlaststatus\rexec stderr | exec stdin\rexec stdout | exec stdin\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (mixed, waited, observed, plain_stderr, plain_stdout) = {
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

        assert_eq!(mixed.line(), b"exec stderr 2>&1 | exec stdin");
        assert_eq!(mixed.status(), LocalCommandStatus::Handled);
        assert_eq!(mixed.response_lines(), 22);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.line(), b"laststatus");
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert_eq!(plain_stderr.line(), b"exec stderr | exec stdin");
        assert_eq!(plain_stderr.status(), LocalCommandStatus::Handled);
        assert_eq!(plain_stdout.line(), b"exec stdout | exec stdin");
        assert_eq!(plain_stdout.status(), LocalCommandStatus::Handled);
        assert!(output.contains(
            "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stderr consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000001f bytes-read=0x000000000000001f writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-stderr-dup-to-stdout\n"
        ));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=pipe-endpoint fd2=pipe-endpoint loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-redirection op=dup source-fd=0x0000000000000002 target-fd=0x0000000000000001 target-stream=pipe-writer target-route=pipe:stdout-to-stdin child-only=true shell-restored=true source=shell-redirection-2-to-1\n"
        ));
        assert!(output.contains(
            "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=pipe-writer route=pipe:stdout-to-stdin source=userspace-talos-write\n"
        ));
        assert!(output.contains(
            "talos> Talos userspace stdin fixture read: Talos userspace stderr fixture\n"
        ));
        assert!(output.contains(
            "talos: exec-stdin fd=0x0000000000000000 bytes=0x000000000000001f return=0x000000000000001f read-source=pipe:stdout-to-stdin stdout-fd=0x0000000000000001 stdout-bytes=0x0000000000000044 stdout-return=0x0000000000000044 source=userspace-talos-read+userspace-talos-write read-result=pipe-eof-after-writer-close\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stderr consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x0000000000000000 bytes-read=0x0000000000000000 writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-stdout-only-stderr-not-piped\n"
        ));
        assert!(output.contains(
            "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdout consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000001f bytes-read=0x000000000000001f writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-stdout-to-stdin\n"
        ));
    }

    #[test_case]
    fn local_command_loop_redirects_pipeline_stdout_away_to_stderr() {
        let bytes = *b"exec stdout 1>&2 | exec stdin\rwaitpid\rlaststatus\rexec stderr 2>&1 | exec stdin\rexec stdout | exec stdin\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (mixed, waited, observed, stderr_to_pipe, plain_stdout) = {
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

        assert_eq!(mixed.line(), b"exec stdout 1>&2 | exec stdin");
        assert_eq!(mixed.status(), LocalCommandStatus::Handled);
        assert_eq!(mixed.response_lines(), 22);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.line(), b"laststatus");
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert_eq!(stderr_to_pipe.line(), b"exec stderr 2>&1 | exec stdin");
        assert_eq!(stderr_to_pipe.status(), LocalCommandStatus::Handled);
        assert_eq!(plain_stdout.line(), b"exec stdout | exec stdin");
        assert_eq!(plain_stdout.status(), LocalCommandStatus::Handled);
        assert!(output.contains(
            "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdout consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x0000000000000000 bytes-read=0x0000000000000000 writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-stdout-redirect-away\n"
        ));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-redirection op=dup source-fd=0x0000000000000001 target-fd=0x0000000000000002 target-stream=stderr target-route=runtime-console0/stderr child-only=true shell-restored=true source=shell-redirection-1-to-2\n"
        ));
        assert!(output.contains(
            "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=stderr route=runtime-console0/stderr source=userspace-talos-write\n"
        ));
        assert!(output.contains("Talos userspace stdin fixture read-result: pipe-eof/no-data\n"));
        assert!(output.contains(
            "talos: exec-stdin fd=0x0000000000000000 bytes=0x0000000000000000 return=0x0000000000000000 read-source=pipe:stdout-to-stdin stdout-fd=0x0000000000000001 stdout-bytes=0x000000000000003c stdout-return=0x000000000000003c source=userspace-talos-read+userspace-talos-write read-result=pipe-eof/no-data\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stderr consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000001f bytes-read=0x000000000000001f writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-stderr-dup-to-stdout\n"
        ));
        assert!(output.contains(
            "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdout consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000001f bytes-read=0x000000000000001f writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-stdout-to-stdin\n"
        ));
    }

    #[test_case]
    fn local_command_loop_redirects_pipeline_consumer_stdout_to_volatile_file() {
        let bytes = *b"exec stdout | exec stdin >/tmp/pipe-consumer.txt\rwaitpid\rlaststatus\rcat /tmp/pipe-consumer.txt\rexec stdout | exec stdin\rexec stdout | exec stdin >>/tmp/pipe-consumer.txt\rexec stderr | exec stdin >/tmp/pipe-consumer.txt\rexec stdout >/tmp/src.txt | exec stdin >/tmp/out.txt\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (
            redirected,
            waited,
            observed,
            readback,
            plain_pipeline,
            append_consumer,
            stderr_producer,
            redirected_producer,
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
            )
        };
        let output = backend.as_str();

        assert_eq!(
            redirected.line(),
            b"exec stdout | exec stdin >/tmp/pipe-consumer.txt"
        );
        assert_eq!(redirected.status(), LocalCommandStatus::Handled);
        assert_eq!(redirected.response_lines(), 22);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.line(), b"laststatus");
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert_eq!(readback.line(), b"cat /tmp/pipe-consumer.txt");
        assert_eq!(readback.status(), LocalCommandStatus::Handled);
        assert_eq!(readback.response_lines(), 2);
        assert_eq!(plain_pipeline.line(), b"exec stdout | exec stdin");
        assert_eq!(plain_pipeline.status(), LocalCommandStatus::Handled);
        for rejected in [append_consumer, stderr_producer, redirected_producer] {
            assert_eq!(rejected.status(), LocalCommandStatus::UnexpectedArgument);
            assert_eq!(rejected.response_lines(), 1);
        }
        assert!(output.contains(
            "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdout consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000001f bytes-read=0x000000000000001f writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-consumer-stdout-redirection\n"
        ));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=pipe-endpoint fd1=regular-file fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-redirection op=sink source-fd=0x0000000000000001 target-path=/tmp/pipe-consumer.txt target-stream=regular-file target-route=volatile-vfs:/tmp/pipe-consumer.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout\n"
        ));
        assert!(output.contains(
            "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=pipe-writer route=pipe:stdout-to-stdin source=userspace-talos-write\n"
        ));
        assert!(output.contains(
            "talos: exec-stdin fd=0x0000000000000000 bytes=0x000000000000001f return=0x000000000000001f read-source=pipe:stdout-to-stdin stdout-fd=0x0000000000000001 stdout-bytes=0x0000000000000044 stdout-return=0x0000000000000044 source=userspace-talos-read+userspace-talos-write read-result=pipe-eof-after-writer-close\n"
        ));
        assert!(output.contains(
            "talos: cat path=/tmp/pipe-consumer.txt bytes=0x0000000000000044 source=volatile-vfs-descriptor-read\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdout consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000001f bytes-read=0x000000000000001f writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-stdout-to-stdin\n"
        ));
    }

    #[test_case]
    fn local_command_loop_redirects_pipeline_producer_stdout_to_volatile_file() {
        let bytes = *b"exec stdout >/tmp/pipe-source.txt | exec stdin\rwaitpid\rlaststatus\rcat /tmp/pipe-source.txt\rexec stdout | exec stdin\rexec stdout >>/tmp/pipe-source.txt | exec stdin\rexec stderr >/tmp/pipe-source.txt | exec stdin\rexec stdout >/tmp/src.txt | exec stdin >/tmp/out.txt\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (
            redirected,
            waited,
            observed,
            readback,
            plain_pipeline,
            append_producer,
            stderr_producer,
            both_redirected,
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
            )
        };
        let output = backend.as_str();

        assert_eq!(
            redirected.line(),
            b"exec stdout >/tmp/pipe-source.txt | exec stdin"
        );
        assert_eq!(redirected.status(), LocalCommandStatus::Handled);
        assert_eq!(redirected.response_lines(), 22);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.line(), b"laststatus");
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert_eq!(readback.line(), b"cat /tmp/pipe-source.txt");
        assert_eq!(readback.status(), LocalCommandStatus::Handled);
        assert_eq!(readback.response_lines(), 2);
        assert_eq!(plain_pipeline.line(), b"exec stdout | exec stdin");
        assert_eq!(plain_pipeline.status(), LocalCommandStatus::Handled);
        for rejected in [append_producer, stderr_producer, both_redirected] {
            assert_eq!(rejected.status(), LocalCommandStatus::UnexpectedArgument);
            assert_eq!(rejected.response_lines(), 1);
        }
        assert!(output.contains(
            "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdout consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x0000000000000000 bytes-read=0x0000000000000000 writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-producer-file-redirection-away\n"
        ));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=regular-file fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-redirection op=sink source-fd=0x0000000000000001 target-path=/tmp/pipe-source.txt target-stream=regular-file target-route=volatile-vfs:/tmp/pipe-source.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout\n"
        ));
        assert!(output.contains(
            "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=regular-file route=volatile-vfs:/tmp/pipe-source.txt source=userspace-talos-write\n"
        ));
        assert!(output.contains(
            "talos: exec-stdin fd=0x0000000000000000 bytes=0x0000000000000000 return=0x0000000000000000 read-source=pipe:stdout-to-stdin stdout-fd=0x0000000000000001 stdout-bytes=0x000000000000003c stdout-return=0x000000000000003c source=userspace-talos-read+userspace-talos-write read-result=pipe-eof/no-data\n"
        ));
        assert!(output.contains(
            "talos: cat path=/tmp/pipe-source.txt bytes=0x000000000000001f source=volatile-vfs-descriptor-read\n"
        ));
        assert!(output.contains("talos> Talos userspace stdout fixture\n"));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdout consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000001f bytes-read=0x000000000000001f writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-stdout-to-stdin\n"
        ));
    }

    #[test_case]
    fn local_command_loop_rejects_unsupported_pipeline_forms() {
        let bytes =
            *b"| exec stdin\rexec stdout |\rexec stdout | exec stdin | x\rexec missing | exec stdin\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (leading, trailing, multi, missing) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            (
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
            )
        };
        let output = backend.as_str();

        assert_eq!(leading.line(), b"| exec stdin");
        assert_eq!(leading.status(), LocalCommandStatus::ParseError);
        assert_eq!(trailing.line(), b"exec stdout |");
        assert_eq!(trailing.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(multi.line(), b"exec stdout | exec stdin | x");
        assert_eq!(multi.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(missing.line(), b"exec missing | exec stdin");
        assert_eq!(missing.status(), LocalCommandStatus::UnexpectedArgument);
        assert!(output.contains("talos> talos: parse-error\n"));
        assert_eq!(output.matches("talos: exec-invalid-path").count(), 3);
    }

    #[test_case]
    fn local_command_loop_execs_userspace_stdin_fixture_through_fd0() {
        let input = ScriptedInput::new(*b"exec stdin\rtalos-console0waitpid\rlaststatus\r", 44);
        let mut backend = CaptureSink::new();
        let (exec, waited, observed) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            (
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
            )
        };
        let output = backend.as_str();

        assert_eq!(exec.line(), b"exec stdin");
        assert_eq!(exec.status(), LocalCommandStatus::Handled);
        assert_eq!(exec.response_lines(), 10);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(waited.response_lines(), 1);
        assert_eq!(observed.line(), b"laststatus");
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.response_lines(), 1);
        assert!(output.contains("talos> Talos userspace stdin fixture read: talos-console0\n"));
        assert!(output.contains("talos: exec path=/bin/stdin source=vfs-open-read\n"));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stdin"
        ));
        assert!(output.contains(
            "talos: exec-stdin fd=0x0000000000000000 bytes=0x000000000000000e return=0x000000000000000e read-source=runtime-console0/local-input stdout-fd=0x0000000000000001 stdout-bytes=0x0000000000000033 stdout-return=0x0000000000000033 source=userspace-talos-read+userspace-talos-write\n"
        ));
        assert!(output.contains(
            "talos: exec-lifecycle pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
    }

    #[test_case]
    fn local_command_loop_reports_userspace_stdin_no_data_through_fd0() {
        let input = ScriptedInput::new(*b"exec stdin\r", 11);
        let mut backend = CaptureSink::new();
        let exec = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            run_one_descriptor_backed_serial_command(&mut io).unwrap()
        };
        let output = backend.as_str();

        assert_eq!(exec.line(), b"exec stdin");
        assert_eq!(exec.status(), LocalCommandStatus::Handled);
        assert_eq!(exec.response_lines(), 10);
        assert!(output.contains("talos> Talos userspace stdin fixture no-data: readiness\n"));
        assert!(output.contains(
            "talos: stdin-wait task=0x0000000000100001 fd=0x0000000000000000 sleep-state=blocked wake-state=blocked wait-cycles=0x0000000000000000 result=sleep source=scheduler-runtime-console-readiness\n"
        ));
        assert!(output.contains(
            "talos: stdin-wait task=0x0000000000100001 fd=0x0000000000000000 sleep-state=blocked wake-state=runnable wait-cycles=0x0000000000000004 result=timeout/no-false-eof source=scheduler-runtime-console-readiness\n"
        ));
        assert!(output.contains("talos: exec path=/bin/stdin source=vfs-open-read\n"));
        assert!(output.contains(
            "talos: exec-stdin fd=0x0000000000000000 bytes=0x0000000000000000 return=0xfffffffffffffff5 read-source=runtime-console0/local-input stdout-fd=0x0000000000000001 stdout-bytes=0x0000000000000031 stdout-return=0x0000000000000031 source=userspace-talos-read+userspace-talos-write read-result=readiness/no-data readiness-observations=0x0000000000000004 scheduler-wait-result=timeout/no-false-eof scheduler-wait-cycles=0x0000000000000004 scheduler-wait-source=scheduler-runtime-console-readiness\n"
        ));
        assert!(output.contains(
            "talos: exec-lifecycle pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true\n"
        ));
    }

    #[test_case]
    fn local_command_loop_reports_userspace_stdin_ctrl_d_terminal_eof() {
        let input = ScriptedInput::new(*b"exec stdin\r\x04waitpid\rlaststatus\r", 31);
        let mut backend = CaptureSink::new();
        let (exec, waited, observed) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            (
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
            )
        };
        let output = backend.as_str();

        assert_eq!(exec.line(), b"exec stdin");
        assert_eq!(exec.status(), LocalCommandStatus::Handled);
        assert_eq!(exec.response_lines(), 10);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.line(), b"laststatus");
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert!(
            output.contains("talos> Talos userspace stdin fixture read-result: terminal-eof\n")
        );
        assert!(!output.contains("read-result=readiness/no-data"));
        assert!(!output.contains("talos: stdin-wait task="));
        assert!(output.contains("talos: exec path=/bin/stdin source=vfs-open-read\n"));
        assert!(output.contains(
            "talos: exec-stdin fd=0x0000000000000000 bytes=0x0000000000000000 return=0x0000000000000000 read-source=runtime-console0/local-input stdout-fd=0x0000000000000001 stdout-bytes=0x0000000000000038 stdout-return=0x0000000000000038 source=userspace-talos-read+userspace-talos-write read-result=terminal-eof\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
    }

    #[test_case]
    fn local_command_loop_bounded_wait_consumes_delayed_userspace_stdin() {
        let input = DelayedScriptedInput::new(
            *b"exec stdin\rtalos-console0waitpid\rlaststatus\r",
            44,
            11,
            2,
        );
        let mut backend = CaptureSink::new();
        let (exec, waited, observed) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            (
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
            )
        };
        let output = backend.as_str();

        assert_eq!(exec.line(), b"exec stdin");
        assert_eq!(exec.status(), LocalCommandStatus::Handled);
        assert_eq!(exec.response_lines(), 10);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.line(), b"laststatus");
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert!(output.contains("talos> Talos userspace stdin fixture no-data: readiness\n"));
        assert!(output.contains(
            "talos: stdin-wait task=0x0000000000100001 fd=0x0000000000000000 sleep-state=blocked wake-state=blocked wait-cycles=0x0000000000000000 result=sleep source=scheduler-runtime-console-readiness\n"
        ));
        assert!(output.contains(
            "talos: stdin-wait task=0x0000000000100001 fd=0x0000000000000000 sleep-state=blocked wake-state=runnable wait-cycles=0x0000000000000002 result=wakeup/resume source=scheduler-runtime-console-readiness\n"
        ));
        assert!(output.contains("Talos userspace stdin fixture read: talos-console0\n"));
        assert!(output.contains(
            "talos: exec-stdin fd=0x0000000000000000 bytes=0x000000000000000e return=0x000000000000000e read-source=runtime-console0/local-input stdout-fd=0x0000000000000001 stdout-bytes=0x0000000000000064 stdout-return=0x0000000000000064 source=userspace-talos-read+userspace-talos-write read-result=scheduler-wait/delayed-input readiness-observations=0x0000000000000002 scheduler-wait-result=wakeup/resume scheduler-wait-cycles=0x0000000000000002 scheduler-wait-source=scheduler-runtime-console-readiness\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
    }

    #[test_case]
    fn local_command_loop_execs_userspace_stderr_fixture_through_fd2() {
        let input = ScriptedInput::new(*b"exec stderr\rwaitpid\rlaststatus\r", 31);
        let mut backend = CaptureSink::new();
        let (exec, waited, observed) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            (
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
            )
        };
        let output = backend.as_str();

        assert_eq!(exec.line(), b"exec stderr");
        assert_eq!(exec.status(), LocalCommandStatus::Handled);
        assert_eq!(exec.response_lines(), 10);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(waited.response_lines(), 1);
        assert_eq!(observed.line(), b"laststatus");
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.response_lines(), 1);
        assert!(output.contains("talos> Talos userspace stderr fixture\n"));
        assert!(output.contains("talos: exec path=/bin/stderr source=vfs-open-read\n"));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stderr"
        ));
        assert!(output.contains(
            "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=stderr route=runtime-console0/stderr source=userspace-talos-write\n"
        ));
        assert!(output.contains(
            "talos: exec-lifecycle pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
    }

    #[test_case]
    fn local_command_loop_execs_absolute_vfs_program_with_literal_argv() {
        let input = ScriptedInput::new(*b"exec /bin/status42 alpha beta\rwaitpid\r", 38);
        let mut backend = CaptureSink::new();
        let (exec, waited) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            (
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
            )
        };
        let output = backend.as_str();

        assert_eq!(exec.line(), b"exec /bin/status42 alpha beta");
        assert_eq!(exec.status(), LocalCommandStatus::Handled);
        assert_eq!(exec.response_lines(), 9);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(waited.response_lines(), 1);
        assert!(output.contains("talos> talos: exec path=/bin/status42 source=vfs-open-read\n"));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-startup-abi state=literal-argv-absolute-empty-envp argc=0x0000000000000003 argv0=/bin/status42 argv1=alpha argv2=beta argv0-ptr=0x00007fffffffffe0 argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x00007fffffffffd8 copied-startup-bytes=0x0000000000000049 source=initial-user-stack-record\n"
        ));
        assert!(output.contains(
            "talos: exec-lifecycle pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=lifecycle-record\n"
        ));
    }

    #[test_case]
    fn local_command_loop_resolves_bare_exec_name_through_fixed_bin_lookup() {
        let input = ScriptedInput::new(*b"exec status42 alpha beta\rwaitpid\rlaststatus\r", 48);
        let mut backend = CaptureSink::new();
        let (exec, waited, observed) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            (
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
            )
        };
        let output = backend.as_str();

        assert_eq!(exec.line(), b"exec status42 alpha beta");
        assert_eq!(exec.status(), LocalCommandStatus::Handled);
        assert_eq!(exec.response_lines(), 9);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(waited.response_lines(), 1);
        assert_eq!(observed.line(), b"laststatus");
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.response_lines(), 1);
        assert!(output.contains("talos> talos: exec path=/bin/status42 source=vfs-open-read\n"));
        assert!(output.contains(
            "talos: exec-startup-abi state=literal-argv-absolute-empty-envp argc=0x0000000000000003 argv0=/bin/status42 argv1=alpha argv2=beta argv0-ptr=0x00007fffffffffe0 argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x00007fffffffffd8 copied-startup-bytes=0x0000000000000049 source=initial-user-stack-record\n"
        ));
        assert!(output.contains(
            "talos: exec-lifecycle pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=lifecycle-record\n"
        ));
    }

    #[test_case]
    fn local_command_loop_rejects_unsupported_literal_exec_grammar() {
        let input = ScriptedInput::new(
            *b"exec /bin/status42 *\rexec /bin/status42 quoted\\arg\r",
            52,
        );
        let mut backend = CaptureSink::new();
        let (glob, escaped) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            (
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
            )
        };

        assert_eq!(glob.line(), b"exec /bin/status42 *");
        assert_eq!(glob.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(glob.response_lines(), 1);
        assert_eq!(escaped.line(), b"exec /bin/status42 quoted\\arg");
        assert_eq!(escaped.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(escaped.response_lines(), 1);
        assert_eq!(
            backend.as_str(),
            "talos> talos: exec-invalid-path\n\
talos> talos: exec-invalid-path\n"
        );
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
            *b"exec /missing\rexec bin/init\rexec /bin\rexec /etc/banner.txt\rexec /empty\r",
            71,
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
        assert_eq!(relative.line(), b"exec bin/init");
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
        let mut truncated_bytes = [b'a'; CANONICAL_LINE_CAPACITY + 2];
        truncated_bytes[CANONICAL_LINE_CAPACITY + 1] = b'\r';
        let mut truncated_input = ScriptedInput::new(truncated_bytes, truncated_bytes.len());
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
