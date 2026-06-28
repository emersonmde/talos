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
    scheduler::{self, ProcessOwnerId, TaskId, TaskState},
    syscall,
    tty::{self, CANONICAL_LINE_CAPACITY, PollingTtyRxOutcome, PollingTtyRxResult},
    userspace_socket_abi,
};

pub const LOCAL_COMMAND_LOOP_VERSION: &str = "phase10.2-kernel-builtins-v2";
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
    "+pipeline-producer-file-redirection-away+background-vfs-exec-lifecycle",
    "+jobs-accounting-list+multiple-background-vfs-exec-records",
    "+background-jobs-stale-entry-policy+generated-root-manifest-read",
    "+generated-root-executable-vfs-exec+shell-pingdiag-vfs-userspace-diagnostic",
    "+shell-sockdiag-vfs-userspace-socket-open-close",
    "+shell-sockdiag-vfs-userspace-socket-bind-listen",
    "+shell-sockdiag-vfs-userspace-socket-connect-accept",
    "+shell-sockdiag-vfs-userspace-socket-send-recv",
    "+shell-sockdiag-vfs-userspace-socket-readiness-poll",
    "+shell-sockdiag-vfs-userspace-socket-blocking-poll-wait",
    "+shell-sockdiag-vfs-userspace-cross-process-local-socket",
    "+shell-sockdiag-vfs-userspace-smoltcp-tcp",
    "+pipeline-distinct-serialized-process-identities",
    "+explicit-pid-wait-status-observation+waitpid-completed-child-observation",
    "+bounded-process-table-direct-vfs-exec-lifecycle",
    "+bounded-process-table-pipeline-background-lifecycle",
    "+proc-talos-processes-descriptor-backed-status-vfs+ps-command-vfs-backed-process-status",
    "+multistage-pipeline-bounded-process-table+bounded-pipeline-status-observation",
    "+direct-absolute-path-vfs-command+bounded-absolute-path-vfs-pipeline",
    "+bounded-bare-name-bin-vfs-command+bounded-bare-name-bin-vfs-pipeline",
    "+direct-absolute-path-vfs-command-literal-argv",
    "+bounded-bare-name-bin-vfs-command-literal-argv",
    "+direct-absolute-path-vfs-pipeline-stage-literal-argv",
    "+bounded-bare-name-bin-vfs-pipeline-stage-literal-argv",
    "+direct-absolute-path-vfs-command-readonly-stdin-redirection",
    "+bounded-bare-name-bin-vfs-command-readonly-stdin-redirection",
    "+direct-absolute-path-vfs-pipeline-producer-readonly-stdin-redirection",
    "+bounded-bare-name-bin-vfs-pipeline-producer-readonly-stdin-redirection",
    "+direct-absolute-path-vfs-pipeline-consumer-readonly-stdin-redirection",
    "+bounded-bare-name-bin-vfs-pipeline-consumer-readonly-stdin-redirection",
    "+bounded-dual-stage-vfs-pipeline-readonly-stdin-redirection",
    "+bounded-bare-name-bin-vfs-command-volatile-stdout-regular-file-redirection",
    "+bounded-bare-name-bin-vfs-command-volatile-stderr-regular-file-redirection",
    "+bounded-bare-name-bin-vfs-command-volatile-stdout-append-redirection",
    "+direct-absolute-path-vfs-command-combined-stdin-stdout-regular-file-redirection",
    "+bounded-bare-name-bin-vfs-command-combined-stdin-stdout-regular-file-redirection",
    "+bounded-bare-name-bin-vfs-pipeline-consumer-volatile-stdout-regular-file-redirection",
    "+direct-absolute-path-vfs-pipeline-combined-stdin-stdout-regular-file-redirection",
    "+bounded-bare-name-bin-vfs-pipeline-combined-stdin-stdout-regular-file-redirection",
    "+direct-absolute-path-vfs-pipeline-consumer-volatile-stdout-append-redirection",
    "+bounded-bare-name-bin-vfs-pipeline-consumer-volatile-stdout-append-redirection",
    "+direct-absolute-path-vfs-pipeline-consumer-volatile-stderr-redirection",
    "+bounded-bare-name-bin-vfs-pipeline-consumer-volatile-stderr-redirection",
    "+direct-absolute-path-vfs-pipeline-consumer-volatile-stderr-append-redirection",
    "+bounded-bare-name-bin-vfs-pipeline-consumer-volatile-stderr-append-redirection"
);
pub const LOCAL_COMMAND_LOOP_PROMPT: &str = "talos> ";
pub const DEFAULT_LOCAL_COMMAND_COUNT: usize = 8;
const LOCAL_COMMAND_LITERAL_ARGV_CAPACITY: usize = 4;
const LOCAL_COMMAND_LITERAL_ARG_BYTES: usize = 32;
const LOCAL_COMMAND_EXEC_PATH_BYTES: usize = LOCAL_COMMAND_LITERAL_ARG_BYTES;
const LOCAL_COMMAND_FILE_USER_BASE: u64 = 0x0000_0000_0011_0000;
const LOCAL_COMMAND_FILE_READ_OFFSET: usize = 0x40;
const LOCAL_COMMAND_FILE_USER_MEMORY_LEN: usize = 2048;
const LOCAL_COMMAND_INITRAMFS_CAT_BUFFER_LEN: usize = 1280;
const LOCAL_COMMAND_READ_ONLY_FILE_CAPACITY: usize = 2;
const LOCAL_COMMAND_PROC_TALOS_PROCESSES_PATH: &[u8] = b"/proc/talos/processes";
const LOCAL_COMMAND_PROCESS_STATUS_SCHEMA: &str = "talos-processes-v1";
const LOCAL_COMMAND_PROCESS_STATUS_FILE_REFERENCE: usize = 0x200;
const LOCAL_COMMAND_PROCESS_STATUS_FILE_BYTES: usize = 1280;
const LOCAL_COMMAND_STDOUT_VOLATILE_FILE_REFERENCE: usize = 0x100;
const LOCAL_COMMAND_STDERR_VOLATILE_FILE_REFERENCE: usize = 0x101;
const LOCAL_COMMAND_VOLATILE_FILE_BYTES: usize = 256;
const LOCAL_COMMAND_VOLATILE_PATH_BYTES: usize = 40;
const LOCAL_COMMAND_VOLATILE_ROUTE_BYTES: usize =
    LOCAL_COMMAND_VOLATILE_PATH_BYTES + b"volatile-vfs:".len();
const LOCAL_COMMAND_EXEC_READ_OFFSET: usize = 0x80;
const LOCAL_COMMAND_EXEC_USER_MEMORY_LEN: usize = 1024;
const LOCAL_COMMAND_STDIN_USER_BASE: u64 = 0x0000_0000_0013_0000;
const LOCAL_COMMAND_STDIN_USER_MEMORY_LEN: usize = 192;
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
const LOCAL_COMMAND_PIPELINE_PRODUCER_PROCESS_ID: u64 = LOCAL_COMMAND_EXEC_PROCESS_ID;
const LOCAL_COMMAND_PIPELINE_CONSUMER_PROCESS_ID: u64 = LOCAL_COMMAND_EXEC_PROCESS_ID + 1;
const LOCAL_COMMAND_PIPELINE_MIDDLE_PROCESS_ID: u64 = LOCAL_COMMAND_EXEC_PROCESS_ID + 1;
const LOCAL_COMMAND_PIPELINE_FINAL_PROCESS_ID: u64 = LOCAL_COMMAND_EXEC_PROCESS_ID + 2;
const LOCAL_COMMAND_EXPLICIT_WAIT_RECORD_CAPACITY: usize = 3;
const LOCAL_COMMAND_PROCESS_TABLE_CAPACITY: usize = 3;
const LOCAL_COMMAND_BACKGROUND_JOB_CAPACITY: usize = 2;
const LOCAL_COMMAND_BACKGROUND_JOB_FIRST_ID: u64 = 0x0000_0001;
const LOCAL_COMMAND_EXEC_TEMP_DESCRIPTOR: usize = posix::STDERR_FD + 1;
const LOCAL_COMMAND_PIPE_BUFFER_LEN: usize = 128;
const LOCAL_COMMAND_PIPE_ENDPOINT_REFERENCE: usize = 1;
const LOCAL_COMMAND_SECOND_PIPE_ENDPOINT_REFERENCE: usize = 2;
const LOCAL_COMMAND_SOCKET_CAPACITY: usize = 4;
const LOCAL_COMMAND_SOCKDIAG_USER_BASE: u64 = 0x0000_0000_0026_0000;
const LOCAL_COMMAND_SOCKDIAG_POLL_WAIT_TASK_BASE: u64 = 0x0000_0000_0000_1200;

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

const LOCAL_COMMAND_ROOT_LISTING: [(&[u8], &str); 5] = [
    (b"/bin", "bin"),
    (b"/dir", "dir"),
    (b"/empty", "empty"),
    (b"/etc", "etc"),
    (b"/generated", "generated"),
];
const LOCAL_COMMAND_ETC_LISTING: [(&[u8], &str); 1] =
    [(initramfs::PHASE8_BANNER_PATH, "banner.txt")];
const LOCAL_COMMAND_BIN_LISTING: [(&[u8], &str); 8] = [
    (initramfs::PHASE8_INIT_PATH, "init"),
    (initramfs::PHASE10_ZERO_PATH, "zero"),
    (initramfs::PHASE10_STATUS42_PATH, "status42"),
    (initramfs::PHASE10_STDOUT_PATH, "stdout"),
    (initramfs::PHASE10_STDIN_PATH, "stdin"),
    (initramfs::PHASE10_STDERR_PATH, "stderr"),
    (initramfs::PHASE12_PINGDIAG_PATH, "pingdiag"),
    (initramfs::PHASE12_SOCKDIAG_PATH, "sockdiag"),
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
    middle: Option<LocalCommandExecRequest>,
    consumer: LocalCommandExecRequest,
}

fn is_direct_pipeline_combined_stdin_stdout_redirection(
    request: &LocalCommandPipelineRequest,
) -> bool {
    request.middle.is_none()
        && request.producer.path() == initramfs::PHASE10_STDIN_PATH
        && request.producer.argv.argc() == 1
        && matches!(
            request.producer.stdin_redirection,
            Some(LocalCommandExecRedirection::StdinFromEtcBanner)
        )
        && request.producer.redirection.is_none()
        && request.consumer.path() == initramfs::PHASE10_STDIN_PATH
        && request.consumer.argv.argc() == 1
        && request.consumer.stdin_redirection.is_none()
        && (matches!(
            request.consumer.redirection,
            Some(LocalCommandExecRedirection::StdoutToTmpStdout(path))
                if path.is_exact_pipeline_combined_path()
                    || path.is_exact_pipeline_combined_append_path()
        ) || matches!(
            request.consumer.redirection,
            Some(LocalCommandExecRedirection::StdoutAppendTmpStdout(path))
                if path.is_exact_pipeline_combined_append_path()
        ))
}

fn is_direct_pipeline_combined_stdin_stderr_redirection(
    request: &LocalCommandPipelineRequest,
) -> bool {
    request.middle.is_none()
        && request.producer.path() == initramfs::PHASE10_STDIN_PATH
        && request.producer.argv.argc() == 1
        && matches!(
            request.producer.stdin_redirection,
            Some(LocalCommandExecRedirection::StdinFromEtcBanner)
        )
        && request.producer.redirection.is_none()
        && request.consumer.path() == initramfs::PHASE10_STDERR_PATH
        && request.consumer.argv.argc() == 1
        && request.consumer.stdin_redirection.is_none()
        && matches!(
            request.consumer.redirection,
            Some(LocalCommandExecRedirection::StderrToTmpStderr(path))
                if path.is_exact_pipeline_combined_stderr_path()
        )
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalCommandPipelineSummary {
    pipe: LocalCommandPipeRecord,
    second_pipe: Option<LocalCommandPipeRecord>,
    lifecycle_status: LocalCommandPipelineLifecycleStatusRecord,
    producer: LocalCommandExecSummary,
    middle: Option<LocalCommandExecSummary>,
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
    fn from_exact_stdout_path(path: &[u8]) -> Option<Self> {
        if path != b"/tmp/stdout.txt" {
            return None;
        }
        Self::from_supported_stdout_path(path)
    }

    fn from_exact_stdin_report_path(path: &[u8]) -> Option<Self> {
        if path != b"/tmp/stdin-report.txt" {
            return None;
        }
        Self::from_supported_stdout_path(path)
    }

    fn from_exact_pipeline_report_path(path: &[u8]) -> Option<Self> {
        if path != b"/tmp/pipeline-report.txt" {
            return None;
        }
        Self::from_supported_stdout_path(path)
    }

    fn from_exact_pipeline_combined_path(path: &[u8]) -> Option<Self> {
        if path != b"/tmp/pipeline-combined.txt" {
            return None;
        }
        Self::from_supported_stdout_path(path)
    }

    fn from_exact_pipeline_combined_append_path(path: &[u8]) -> Option<Self> {
        if path != b"/tmp/pipeline-combined-append.txt" {
            return None;
        }
        Self::from_supported_stdout_path(path)
    }

    fn from_exact_pipeline_combined_stderr_path(path: &[u8]) -> Option<Self> {
        if path != b"/tmp/pipeline-combined-stderr.txt" {
            return None;
        }
        Self::from_supported_stderr_path(path)
    }

    fn from_exact_pipeline_stderr_path(path: &[u8]) -> Option<Self> {
        if path != b"/tmp/pipeline-stderr.txt" {
            return None;
        }
        Self::from_supported_stderr_path(path)
    }

    fn from_exact_stderr_path(path: &[u8]) -> Option<Self> {
        if path != b"/tmp/stderr.txt" {
            return None;
        }
        Self::from_supported_stderr_path(path)
    }

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

    fn is_exact_pipeline_combined_path(self) -> bool {
        self.as_bytes() == b"/tmp/pipeline-combined.txt"
    }

    fn is_exact_pipeline_combined_append_path(self) -> bool {
        self.as_bytes() == b"/tmp/pipeline-combined-append.txt"
    }

    fn is_exact_pipeline_combined_stderr_path(self) -> bool {
        self.as_bytes() == b"/tmp/pipeline-combined-stderr.txt"
    }

    fn is_exact_pipeline_stderr_path(self) -> bool {
        self.as_bytes() == b"/tmp/pipeline-stderr.txt"
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
    pingdiag: Option<LocalCommandPingdiagRecord>,
    pingdiag_controls: Option<LocalCommandPingdiagControlRecord>,
    sockdiag: Option<LocalCommandSockdiagRecord>,
    sockdiag_controls: Option<LocalCommandSockdiagControlRecord>,
    lifecycle: LocalCommandProcessLifecycleRecord,
    init_lifecycle_status: Option<LocalCommandInitLifecycleStatusRecord>,
    vfs_exec_lifecycle_status: Option<LocalCommandVfsExecLifecycleStatusRecord>,
    process_table_record: Option<LocalCommandProcessTableRecord>,
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
pub struct LocalCommandPipelineLifecycleStatusRecord {
    identity: &'static str,
    pipe_id: usize,
    producer: LocalCommandProcessLifecycleRecord,
    middle: Option<LocalCommandProcessLifecycleRecord>,
    consumer: LocalCommandProcessLifecycleRecord,
}

impl LocalCommandPipelineLifecycleStatusRecord {
    const IDENTITY: &'static str =
        "phase12-local-pipeline-distinct-process-lifecycle-status-record-v1";
    const MULTISTAGE_IDENTITY: &'static str =
        "phase12-local-multistage-pipeline-lifecycle-status-record-v1";

    const fn from_pipeline(
        pipe_id: usize,
        producer: LocalCommandProcessLifecycleRecord,
        consumer: LocalCommandProcessLifecycleRecord,
    ) -> Self {
        Self {
            identity: Self::IDENTITY,
            pipe_id,
            producer,
            middle: None,
            consumer,
        }
    }

    const fn from_three_stage_pipeline(
        pipe_id: usize,
        producer: LocalCommandProcessLifecycleRecord,
        middle: LocalCommandProcessLifecycleRecord,
        consumer: LocalCommandProcessLifecycleRecord,
    ) -> Self {
        Self {
            identity: Self::MULTISTAGE_IDENTITY,
            pipe_id,
            producer,
            middle: Some(middle),
            consumer,
        }
    }
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
pub struct LocalCommandPingdiagRecord {
    process_descriptor: usize,
    destination_ipv4: [u8; 4],
    payload_len: usize,
    start_step: syscall::PingOperationSyscallSubstituteStepKind,
    arp_driver_step: &'static str,
    arp_frame_len: usize,
    arp_pump_step: syscall::PingOperationSyscallSubstituteStepKind,
    icmp_driver_step: &'static str,
    icmp_frame_len: usize,
    result_step: syscall::PingOperationSyscallSubstituteStepKind,
    status: syscall::PingOperationSyscallSubstituteStatusKind,
    status_payload_len: usize,
    closed: bool,
    source: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalCommandPingdiagControlRecord {
    malformed_arguments: &'static str,
    missing_executable_identity: &'static str,
    owner_descriptor_failure: &'static str,
    invalid_closed_descriptor: &'static str,
    queue_backpressure: &'static str,
    timeout_retry: &'static str,
    device_errors: &'static str,
    syscall_vocabulary: &'static str,
    source: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalCommandSockdiagRecord {
    process_descriptor: usize,
    client_descriptor: usize,
    accepted_descriptor: usize,
    empty_listener_revents: u32,
    pending_listener_revents: u32,
    empty_recv_revents: u32,
    payload_recv_revents: u32,
    write_ready_revents: u32,
    write_backpressure_revents: u32,
    peer_hangup_revents: u32,
    invalid_descriptor_revents: u32,
    non_socket_descriptor_revents: u32,
    poll_wait_immediate_revents: u32,
    poll_wait_pending_listener_revents: u32,
    poll_wait_payload_revents: u32,
    poll_wait_timeout_revents: u32,
    poll_wait_peer_hangup_revents: u32,
    poll_wait_blocked_state: &'static str,
    poll_wait_ready_state: &'static str,
    poll_wait_timeout_state: &'static str,
    poll_wait_ready_count: u64,
    poll_wait_timeout_tick: u64,
    client_send_bytes: usize,
    server_recv_bytes: usize,
    server_send_bytes: usize,
    client_recv_bytes: usize,
    client_payload: &'static str,
    server_payload: &'static str,
    domain: u64,
    socket_type: u64,
    protocol: u64,
    local_ipv4_be: u32,
    local_port: u16,
    bind_return: u64,
    listen_backlog: u8,
    listen_return: u64,
    connect_return: u64,
    accept_return: u64,
    socket_state: &'static str,
    client_state: &'static str,
    accepted_state: &'static str,
    descriptor_kind: &'static str,
    descriptor_access: &'static str,
    close_return: u64,
    backing_closed: bool,
    cross_process_server_owner: u64,
    cross_process_client_owner: u64,
    cross_process_server_descriptor: usize,
    cross_process_client_descriptor: usize,
    cross_process_accepted_descriptor: usize,
    cross_process_listener_revents: u32,
    cross_process_payload_revents: u32,
    cross_process_hangup_revents: u32,
    cross_process_accept_wait_revents: u32,
    cross_process_payload_wait_revents: u32,
    cross_process_cleanup_close_return: u64,
    cross_process_payload: &'static str,
    cross_process_reply: &'static str,
    cross_process_descriptor_ownership: &'static str,
    cross_process_backing_closed: bool,
    smoltcp_connection_id: u64,
    smoltcp_handshake_client_state: &'static str,
    smoltcp_handshake_server_state: &'static str,
    smoltcp_handshake_steps: usize,
    smoltcp_handshake_client_to_server_frames: usize,
    smoltcp_handshake_server_to_client_frames: usize,
    smoltcp_accepted_attached: bool,
    smoltcp_payload_transfers: u64,
    smoltcp_payload_len: usize,
    smoltcp_payload_client_state: &'static str,
    smoltcp_payload_server_state: &'static str,
    driver_packet_rx_step: &'static str,
    driver_packet_rx_frame_len: usize,
    driver_packet_tx_step: &'static str,
    driver_packet_tx_frame_len: usize,
    driver_packet_tx_observed_len: usize,
    driver_packet_tx_queued_after_pop: usize,
    driver_packet_backpressure_step: &'static str,
    driver_packet_backpressure_rx_queued: usize,
    driver_packet_backpressure_tx_queued: usize,
    driver_packet_evidence: &'static str,
    source: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalCommandSockdiagControlRecord {
    malformed_arguments: &'static str,
    missing_executable_identity: &'static str,
    unsupported_domain: &'static str,
    unsupported_type: &'static str,
    unsupported_protocol: &'static str,
    listen_before_bind: &'static str,
    invalid_bind_endpoint: &'static str,
    invalid_backlog: &'static str,
    repeated_bind: &'static str,
    repeated_listen: &'static str,
    accept_before_connect: &'static str,
    missing_listener: &'static str,
    queue_backpressure: &'static str,
    non_socket_descriptor: &'static str,
    empty_recv: &'static str,
    send_invalid_flags: &'static str,
    recv_invalid_flags: &'static str,
    payload_queue_backpressure: &'static str,
    send_after_peer_close: &'static str,
    poll_unsupported_events: &'static str,
    poll_invalid_descriptor: &'static str,
    poll_non_socket_descriptor: &'static str,
    poll_wait_scalar_dispatch: &'static str,
    poll_wait_invalid_timeout: &'static str,
    poll_wait_unsupported_events: &'static str,
    invalid_closed_descriptor: &'static str,
    syscall_vocabulary: &'static str,
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

    const fn with_process_id(self, process_id: u64) -> Self {
        Self { process_id, ..self }
    }

    pub(crate) const fn completed_wait_exit_status_u32(self) -> Option<u32> {
        if !matches!(self.state, LocalCommandProcessState::Exited)
            || !self.reaped
            || self.status != self.observed_status
            || self.status > u32::MAX as u64
        {
            return None;
        }
        Some(self.status as u32)
    }

    #[cfg(test)]
    pub(crate) const fn ssh_model_test_exit_record(status: u32) -> Self {
        Self::exited(
            LOCAL_COMMAND_EXEC_PROCESS_ID,
            1,
            b"/bin/status42",
            status as u64,
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalCommandInitLifecycleStatusRecord {
    identity: &'static str,
    lifecycle: LocalCommandProcessLifecycleRecord,
}

impl LocalCommandInitLifecycleStatusRecord {
    const IDENTITY: &'static str = "phase12-local-process-lifecycle-status-record-v1";

    fn from_lifecycle(lifecycle: LocalCommandProcessLifecycleRecord) -> Option<Self> {
        if lifecycle.source_path != initramfs::PHASE8_INIT_PATH {
            return None;
        }
        Some(Self {
            identity: Self::IDENTITY,
            lifecycle,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalCommandVfsExecLifecycleStatusRecord {
    identity: &'static str,
    lifecycle: LocalCommandProcessLifecycleRecord,
}

impl LocalCommandVfsExecLifecycleStatusRecord {
    const IDENTITY: &'static str = "phase12-local-vfs-exec-lifecycle-status-record-v2";

    fn from_lifecycle(lifecycle: LocalCommandProcessLifecycleRecord) -> Option<Self> {
        if !matches!(
            lifecycle.source_path,
            initramfs::PHASE8_INIT_PATH
                | initramfs::PHASE10_ZERO_PATH
                | initramfs::PHASE10_STATUS42_PATH
        ) {
            return None;
        }
        Some(Self {
            identity: Self::IDENTITY,
            lifecycle,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalCommandProcessTableRecord {
    identity: &'static str,
    slot: usize,
    capacity: usize,
    lifecycle: LocalCommandProcessLifecycleRecord,
}

impl LocalCommandProcessTableRecord {
    const IDENTITY: &'static str = "phase12-local-bounded-process-table-lifecycle-record-v1";

    const fn from_lifecycle(
        slot: usize,
        capacity: usize,
        lifecycle: LocalCommandProcessLifecycleRecord,
    ) -> Option<Self> {
        if !matches!(
            lifecycle.source_path,
            initramfs::PHASE8_INIT_PATH
                | initramfs::PHASE10_ZERO_PATH
                | initramfs::PHASE10_STATUS42_PATH
                | initramfs::PHASE10_STDOUT_PATH
                | initramfs::PHASE10_STDIN_PATH
                | initramfs::PHASE10_STDERR_PATH
        ) {
            return None;
        }
        Some(Self {
            identity: Self::IDENTITY,
            slot,
            capacity,
            lifecycle,
        })
    }
}

struct LocalCommandProcessStatusFile {
    bytes: [u8; LOCAL_COMMAND_PROCESS_STATUS_FILE_BYTES],
    len: usize,
}

impl LocalCommandProcessStatusFile {
    const fn new_empty() -> Self {
        Self {
            bytes: [0; LOCAL_COMMAND_PROCESS_STATUS_FILE_BYTES],
            len: 0,
        }
    }

    fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}

impl core::fmt::Write for LocalCommandProcessStatusFile {
    fn write_str(&mut self, text: &str) -> core::fmt::Result {
        let end = self.len.checked_add(text.len()).ok_or(core::fmt::Error)?;
        if end > self.bytes.len() {
            return Err(core::fmt::Error);
        }
        self.bytes[self.len..end].copy_from_slice(text.as_bytes());
        self.len = end;
        Ok(())
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
pub enum LocalCommandExplicitWaitResult {
    Record(LocalCommandProcessLifecycleRecord),
    RecordWithSource(LocalCommandProcessLifecycleRecord, &'static str),
    NoChild,
    UnsupportedPid,
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

    fn exec_three_stage_vfs_pipeline(
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

    fn exec_shell_pingdiag_diagnostic(
        &mut self,
    ) -> Result<LocalCommandPingdiagRecord, LocalCommandExecError> {
        Err(LocalCommandExecError::NotSupported)
    }

    fn exec_shell_sockdiag_diagnostic(
        &mut self,
    ) -> Result<LocalCommandSockdiagRecord, LocalCommandExecError> {
        Err(LocalCommandExecError::NotSupported)
    }

    fn poll_background_job_completion(&mut self) -> Option<LocalCommandBackgroundJobRecord> {
        None
    }

    fn clear_completed_background_job_records(&mut self) -> usize {
        0
    }

    fn background_job_records(
        &self,
    ) -> [Option<LocalCommandBackgroundJobRecord>; LOCAL_COMMAND_BACKGROUND_JOB_CAPACITY] {
        [None; LOCAL_COMMAND_BACKGROUND_JOB_CAPACITY]
    }

    fn process_table_records(
        &self,
    ) -> [Option<LocalCommandProcessTableRecord>; LOCAL_COMMAND_PROCESS_TABLE_CAPACITY] {
        [None; LOCAL_COMMAND_PROCESS_TABLE_CAPACITY]
    }

    fn last_process_lifecycle_record(&self) -> Option<LocalCommandProcessLifecycleRecord> {
        None
    }

    fn wait_process_lifecycle_record(&mut self) -> Option<LocalCommandProcessLifecycleRecord> {
        None
    }

    fn wait_process_lifecycle_record_with_source(
        &mut self,
    ) -> Option<(LocalCommandProcessLifecycleRecord, &'static str)> {
        self.wait_process_lifecycle_record()
            .map(|record| (record, "lifecycle-record"))
    }

    fn wait_process_lifecycle_record_by_pid(
        &mut self,
        _process_id: u64,
    ) -> LocalCommandExplicitWaitResult {
        LocalCommandExplicitWaitResult::UnsupportedPid
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
    explicit_wait_records:
        [Option<LocalCommandProcessLifecycleRecord>; LOCAL_COMMAND_EXPLICIT_WAIT_RECORD_CAPACITY],
    process_table_records:
        [Option<LocalCommandProcessTableRecord>; LOCAL_COMMAND_PROCESS_TABLE_CAPACITY],
    background_jobs:
        [Option<LocalCommandBackgroundJobRecord>; LOCAL_COMMAND_BACKGROUND_JOB_CAPACITY],
    next_background_job_id: u64,
    pipe: LocalCommandPipeState,
    second_pipe: LocalCommandPipeState,
    stdout_scratch_file: LocalCommandVolatileFileState,
    stderr_scratch_file: LocalCommandVolatileFileState,
    socket_descriptors: crate::network::NetworkSocketDescriptorTable<LOCAL_COMMAND_SOCKET_CAPACITY>,
}

struct LocalCommandDiscardConsole;

impl core::fmt::Write for LocalCommandDiscardConsole {
    fn write_str(&mut self, _s: &str) -> core::fmt::Result {
        Ok(())
    }
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

impl<I, O> DescriptorBackedLocalCommandIo<I, O, 2, 7>
where
    I: ConsoleInputBackend,
    O: ConsoleBackend,
{
    pub fn new_inherited_stdio(
        input_backend: I,
        output_backend: O,
    ) -> Result<Self, posix::PosixError> {
        let current_owner = ProcessOwnerId::new(1).expect("local command owner id is nonzero");
        let mut descriptor_store = posix::ProcessDescriptorStore::<2, 7>::new_empty();
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
            explicit_wait_records: [None; LOCAL_COMMAND_EXPLICIT_WAIT_RECORD_CAPACITY],
            process_table_records: [None; LOCAL_COMMAND_PROCESS_TABLE_CAPACITY],
            background_jobs: [None; LOCAL_COMMAND_BACKGROUND_JOB_CAPACITY],
            next_background_job_id: LOCAL_COMMAND_BACKGROUND_JOB_FIRST_ID,
            pipe: LocalCommandPipeState::new_empty(),
            second_pipe: LocalCommandPipeState::new_empty(),
            stdout_scratch_file: LocalCommandVolatileFileState::new_empty(),
            stderr_scratch_file: LocalCommandVolatileFileState::new_empty(),
            socket_descriptors: crate::network::NetworkSocketDescriptorTable::new(),
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

impl<I, O, const OWNER_CAPACITY: usize, const DESCRIPTOR_CAPACITY: usize>
    DescriptorBackedLocalCommandIo<I, O, OWNER_CAPACITY, DESCRIPTOR_CAPACITY>
where
    I: ConsoleInputBackend,
    O: ConsoleBackend,
{
    fn record_direct_process_table_record(
        &mut self,
        lifecycle: LocalCommandProcessLifecycleRecord,
    ) -> Option<LocalCommandProcessTableRecord> {
        self.process_table_records = [None; LOCAL_COMMAND_PROCESS_TABLE_CAPACITY];
        self.record_process_table_record(0, lifecycle)
    }

    fn record_process_table_record(
        &mut self,
        slot: usize,
        lifecycle: LocalCommandProcessLifecycleRecord,
    ) -> Option<LocalCommandProcessTableRecord> {
        if slot >= LOCAL_COMMAND_PROCESS_TABLE_CAPACITY {
            return None;
        }
        let record = LocalCommandProcessTableRecord::from_lifecycle(
            slot,
            LOCAL_COMMAND_PROCESS_TABLE_CAPACITY,
            lifecycle,
        )?;
        self.process_table_records[slot] = Some(record);
        Some(record)
    }

    #[cfg(test)]
    fn process_table_records(
        &self,
    ) -> [Option<LocalCommandProcessTableRecord>; LOCAL_COMMAND_PROCESS_TABLE_CAPACITY] {
        self.process_table_records
    }

    fn process_status_file_bytes(&self) -> Result<LocalCommandProcessStatusFile, core::fmt::Error> {
        use core::fmt::Write as _;

        let mut file = LocalCommandProcessStatusFile::new_empty();
        writeln!(file, "{}", LOCAL_COMMAND_PROCESS_STATUS_SCHEMA)?;
        for record in self.process_table_records.into_iter().flatten() {
            let lifecycle = record.lifecycle;
            let wait_consumed = !self.process_has_pending_wait_observation(lifecycle.process_id);
            let job_state = self
                .background_jobs
                .iter()
                .flatten()
                .find(|job| job.lifecycle.process_id == lifecycle.process_id)
                .map(|job| job.state.name())
                .unwrap_or("foreground");
            write!(
                file,
                "slot={} capacity={} pid={:#018x} parent=shell owner={:#018x} path=",
                record.slot, record.capacity, lifecycle.process_id, lifecycle.parent_owner_id
            )?;
            core::fmt::Write::write_str(
                &mut file,
                core::str::from_utf8(lifecycle.source_path).map_err(|_| core::fmt::Error)?,
            )?;
            writeln!(
                file,
                " state={} status={:#018x} observed-status={:#018x} reaped={} wait-consumed={} job-state={} source=bounded-process-table",
                lifecycle.state.name(),
                lifecycle.status,
                lifecycle.observed_status,
                lifecycle.reaped,
                wait_consumed,
                job_state
            )?;
        }
        Ok(file)
    }

    fn process_has_pending_wait_observation(&self, process_id: u64) -> bool {
        self.waitable_process
            .map(|record| record.process_id == process_id)
            .unwrap_or(false)
            || self
                .explicit_wait_records
                .iter()
                .flatten()
                .any(|record| record.process_id == process_id)
            || self
                .background_jobs
                .iter()
                .flatten()
                .any(|job| job.lifecycle.process_id == process_id)
    }

    fn pipe_route(reference: usize) -> Result<&'static str, LocalCommandExecError> {
        match reference {
            LOCAL_COMMAND_PIPE_ENDPOINT_REFERENCE => Ok("pipe:stdout-to-stdin"),
            LOCAL_COMMAND_SECOND_PIPE_ENDPOINT_REFERENCE => Ok("pipe:middle-to-stdin"),
            _ => Err(LocalCommandExecError::LaunchPipelineFailed),
        }
    }

    fn pipe_state(
        &self,
        reference: usize,
    ) -> Result<&LocalCommandPipeState, LocalCommandExecError> {
        match reference {
            LOCAL_COMMAND_PIPE_ENDPOINT_REFERENCE => Ok(&self.pipe),
            LOCAL_COMMAND_SECOND_PIPE_ENDPOINT_REFERENCE => Ok(&self.second_pipe),
            _ => Err(LocalCommandExecError::LaunchPipelineFailed),
        }
    }

    fn pipe_state_mut(
        &mut self,
        reference: usize,
    ) -> Result<&mut LocalCommandPipeState, LocalCommandExecError> {
        match reference {
            LOCAL_COMMAND_PIPE_ENDPOINT_REFERENCE => Ok(&mut self.pipe),
            LOCAL_COMMAND_SECOND_PIPE_ENDPOINT_REFERENCE => Ok(&mut self.second_pipe),
            _ => Err(LocalCommandExecError::LaunchPipelineFailed),
        }
    }

    fn pipe_remaining(&self, reference: usize) -> Result<usize, LocalCommandExecError> {
        let pipe = self.pipe_state(reference)?;
        Ok(pipe.len.saturating_sub(pipe.cursor))
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
        if path == LOCAL_COMMAND_PROC_TALOS_PROCESSES_PATH {
            return self.read_process_status_file_via_descriptor(output);
        }
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
            initramfs::PHASE12_PINGDIAG_PATH => initramfs::PHASE12_PINGDIAG_PATH,
            initramfs::PHASE12_SOCKDIAG_PATH => initramfs::PHASE12_SOCKDIAG_PATH,
            initramfs::GENERATED_ROOT_EXEC_PATH => initramfs::GENERATED_ROOT_EXEC_PATH,
            initramfs::PHASE8_BANNER_PATH => initramfs::PHASE8_BANNER_PATH,
            initramfs::PHASE8_EMPTY_PATH => initramfs::PHASE8_EMPTY_PATH,
            initramfs::PHASE8_NESTED_PATH => initramfs::PHASE8_NESTED_PATH,
            _ => return Err(LocalCommandExecError::NotExecutable),
        };
        if source_path == initramfs::PHASE12_PINGDIAG_PATH
            && (request.argv.argc() != 1
                || request.stdin_redirection.is_some()
                || request.redirection.is_some())
        {
            return Err(LocalCommandExecError::InvalidPath);
        }
        if source_path == initramfs::PHASE12_SOCKDIAG_PATH
            && (request.argv.argc() != 1
                || request.stdin_redirection.is_some()
                || request.redirection.is_some())
        {
            return Err(LocalCommandExecError::InvalidPath);
        }
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
                Some(
                    LocalCommandExecRedirection::StdoutToTmpStdout(_)
                        | LocalCommandExecRedirection::StdoutAppendTmpStdout(_)
                )
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
        let (pingdiag, pingdiag_controls) = if source_path == initramfs::PHASE12_PINGDIAG_PATH {
            (
                Some(self.exec_shell_pingdiag_diagnostic()?),
                Some(LocalCommandPingdiagControlRecord {
                    malformed_arguments: "exec-invalid-path",
                    missing_executable_identity: "exec-not-found",
                    owner_descriptor_failure: "EBADF",
                    invalid_closed_descriptor: "EBADF",
                    queue_backpressure: "ENOSPC",
                    timeout_retry: "timed-out-after-retry",
                    device_errors: "EIO",
                    syscall_vocabulary: "SyscallNumber/STABLE_SVC_IMMEDIATE/TALOS_* unchanged",
                    source: "shell-pingdiag-controls",
                }),
            )
        } else {
            (None, None)
        };
        let (sockdiag, sockdiag_controls) = if source_path == initramfs::PHASE12_SOCKDIAG_PATH {
            (
                Some(self.exec_shell_sockdiag_diagnostic()?),
                Some(LocalCommandSockdiagControlRecord {
                    malformed_arguments: "exec-invalid-path",
                    missing_executable_identity: "exec-not-found",
                    unsupported_domain: "ENOTSUP",
                    unsupported_type: "ENOTSUP",
                    unsupported_protocol: "ENOTSUP",
                    listen_before_bind: "EINVAL",
                    invalid_bind_endpoint: "EINVAL",
                    invalid_backlog: "EINVAL",
                    repeated_bind: "EINVAL",
                    repeated_listen: "ok-updates-backlog",
                    accept_before_connect: "EAGAIN",
                    missing_listener: "EINVAL",
                    queue_backpressure: "ENOSPC",
                    non_socket_descriptor: "EBADF",
                    empty_recv: "EAGAIN",
                    send_invalid_flags: "EINVAL",
                    recv_invalid_flags: "EINVAL",
                    payload_queue_backpressure: "ENOSPC",
                    send_after_peer_close: "EPIPE",
                    poll_unsupported_events: "EINVAL",
                    poll_invalid_descriptor: "ERROR",
                    poll_non_socket_descriptor: "ERROR",
                    poll_wait_scalar_dispatch: "ENOTSUP",
                    poll_wait_invalid_timeout: "EINVAL",
                    poll_wait_unsupported_events: "EINVAL",
                    invalid_closed_descriptor: "EBADF",
                    syscall_vocabulary: "SyscallNumber/STABLE_SVC_IMMEDIATE/TALOS_SOCKET/TALOS_BIND/TALOS_LISTEN/TALOS_CONNECT/TALOS_ACCEPT/TALOS_SEND/TALOS_RECV/TALOS_POLL/TALOS_POLL_WAIT/TALOS_CLOSE bounded",
                    source: "shell-sockdiag-controls",
                }),
            )
        } else {
            (None, None)
        };
        let redirections = self.restore_exec_redirections(applied_redirections)?;
        let lifecycle = LocalCommandProcessLifecycleRecord::exited(
            LOCAL_COMMAND_EXEC_PROCESS_ID,
            owner.raw(),
            source_path,
            completion_status,
        );
        let init_lifecycle_status =
            LocalCommandInitLifecycleStatusRecord::from_lifecycle(lifecycle);
        let vfs_exec_lifecycle_status =
            LocalCommandVfsExecLifecycleStatusRecord::from_lifecycle(lifecycle);
        let process_table_record = self.record_direct_process_table_record(lifecycle);
        self.last_process = Some(lifecycle);
        self.waitable_process = Some(lifecycle);
        self.explicit_wait_records = [Some(lifecycle), None, None];

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
            pingdiag,
            pingdiag_controls,
            sockdiag,
            sockdiag_controls,
            lifecycle,
            init_lifecycle_status,
            vfs_exec_lifecycle_status,
            process_table_record,
        })
    }

    fn exec_shell_sockdiag_diagnostic(
        &mut self,
    ) -> Result<LocalCommandSockdiagRecord, LocalCommandExecError> {
        let owner = self
            .current_owner
            .ok_or(LocalCommandExecError::LaunchPipelineFailed)?;
        let mut user_memory = [0u8; 128];
        let mut kernel_scratch = [0u8; 64];
        let mappings = [posix::UserMapping::new(
            LOCAL_COMMAND_SOCKDIAG_USER_BASE,
            user_memory.len(),
            posix::UserMappingPermissions::USER_DATA,
        )
        .map_err(local_exec_error_from_posix)?];
        let mut socket_dispatch =
            |raw_number: u64,
             arguments: syscall::SyscallArguments,
             descriptor_store: &mut posix::ProcessDescriptorStore<
                OWNER_CAPACITY,
                DESCRIPTOR_CAPACITY,
            >,
             socket_descriptors: &mut crate::network::NetworkSocketDescriptorTable<
                LOCAL_COMMAND_SOCKET_CAPACITY,
            >,
             user_memory: &mut [u8; 128],
             kernel_scratch: &mut [u8; 64]| {
                syscall::dispatch_process_descriptor_with_socket_table(
                    raw_number,
                    arguments,
                    Some(owner),
                    descriptor_store,
                    socket_descriptors,
                    &mappings,
                    LOCAL_COMMAND_SOCKDIAG_USER_BASE,
                    user_memory,
                    kernel_scratch,
                    &mut self.output_backend,
                )
            };
        macro_rules! socket_abi_dispatch {
            ($call:expr) => {{
                let call = $call;
                socket_dispatch(
                    call.number(),
                    call.syscall_arguments(),
                    &mut self.descriptor_store,
                    &mut self.socket_descriptors,
                    &mut user_memory,
                    &mut kernel_scratch,
                )
            }};
        }

        let unsupported_domain = socket_dispatch(
            syscall::TALOS_SOCKET_SYSCALL,
            syscall::SyscallArguments::new([
                10,
                crate::network::SOCKET_TYPE_STREAM,
                crate::network::SOCKET_PROTOCOL_DEFAULT,
                0,
                0,
                0,
            ]),
            &mut self.descriptor_store,
            &mut self.socket_descriptors,
            &mut user_memory,
            &mut kernel_scratch,
        );
        if syscall::errno_number(posix::PosixError::NotSupported) as u64
            != unsupported_domain.return_value().x0().wrapping_neg()
        {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        let unsupported_type = socket_dispatch(
            syscall::TALOS_SOCKET_SYSCALL,
            syscall::SyscallArguments::new([
                crate::network::SOCKET_DOMAIN_AF_INET,
                2,
                crate::network::SOCKET_PROTOCOL_DEFAULT,
                0,
                0,
                0,
            ]),
            &mut self.descriptor_store,
            &mut self.socket_descriptors,
            &mut user_memory,
            &mut kernel_scratch,
        );
        if syscall::errno_number(posix::PosixError::NotSupported) as u64
            != unsupported_type.return_value().x0().wrapping_neg()
        {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        let unsupported_protocol = socket_dispatch(
            syscall::TALOS_SOCKET_SYSCALL,
            syscall::SyscallArguments::new([
                crate::network::SOCKET_DOMAIN_AF_INET,
                crate::network::SOCKET_TYPE_STREAM,
                17,
                0,
                0,
                0,
            ]),
            &mut self.descriptor_store,
            &mut self.socket_descriptors,
            &mut user_memory,
            &mut kernel_scratch,
        );
        if syscall::errno_number(posix::PosixError::NotSupported) as u64
            != unsupported_protocol.return_value().x0().wrapping_neg()
        {
            return Err(LocalCommandExecError::SyscallFailed);
        }

        let invalid_closed_bind = socket_dispatch(
            syscall::TALOS_BIND_SYSCALL,
            syscall::SyscallArguments::new([3, 0x7f00_0001, 8080, 0, 0, 0]),
            &mut self.descriptor_store,
            &mut self.socket_descriptors,
            &mut user_memory,
            &mut kernel_scratch,
        );
        if syscall::errno_number(posix::PosixError::BadDescriptor) as u64
            != invalid_closed_bind.return_value().x0().wrapping_neg()
        {
            return Err(LocalCommandExecError::SyscallFailed);
        }

        let open = socket_abi_dispatch!(userspace_socket_abi::inet_stream_socket());
        let process_descriptor = syscall_success_usize(open.return_value().x0())
            .map_err(|_| LocalCommandExecError::SyscallFailed)?;
        let entry = self
            .descriptor_store
            .current_descriptor_table(self.current_owner)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?
            .get(process_descriptor)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        let socket_reference =
            crate::network::NetworkSocketDescriptor::from_raw(entry.object().reference());
        let socket = self
            .socket_descriptors
            .socket(socket_reference)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        if socket.owner() != owner
            || socket.domain() != crate::network::SOCKET_DOMAIN_AF_INET
            || socket.socket_type() != crate::network::SOCKET_TYPE_STREAM
            || socket.protocol() != crate::network::SOCKET_PROTOCOL_DEFAULT
        {
            return Err(LocalCommandExecError::LaunchPipelineFailed);
        }

        let listen_before_bind = socket_dispatch(
            syscall::TALOS_LISTEN_SYSCALL,
            syscall::SyscallArguments::new([process_descriptor as u64, 1, 0, 0, 0, 0]),
            &mut self.descriptor_store,
            &mut self.socket_descriptors,
            &mut user_memory,
            &mut kernel_scratch,
        );
        if syscall::errno_number(posix::PosixError::InvalidArgument) as u64
            != listen_before_bind.return_value().x0().wrapping_neg()
        {
            return Err(LocalCommandExecError::SyscallFailed);
        }

        let invalid_endpoint = socket_dispatch(
            syscall::TALOS_BIND_SYSCALL,
            syscall::SyscallArguments::new([process_descriptor as u64, 0x7f00_0001, 0, 0, 0, 0]),
            &mut self.descriptor_store,
            &mut self.socket_descriptors,
            &mut user_memory,
            &mut kernel_scratch,
        );
        if syscall::errno_number(posix::PosixError::InvalidArgument) as u64
            != invalid_endpoint.return_value().x0().wrapping_neg()
        {
            return Err(LocalCommandExecError::SyscallFailed);
        }

        let invalid_backlog = socket_dispatch(
            syscall::TALOS_LISTEN_SYSCALL,
            syscall::SyscallArguments::new([process_descriptor as u64, 5, 0, 0, 0, 0]),
            &mut self.descriptor_store,
            &mut self.socket_descriptors,
            &mut user_memory,
            &mut kernel_scratch,
        );
        if syscall::errno_number(posix::PosixError::InvalidArgument) as u64
            != invalid_backlog.return_value().x0().wrapping_neg()
        {
            return Err(LocalCommandExecError::SyscallFailed);
        }

        const SOCKDIAG_LOCAL_IPV4_BE: u32 = 0x7f00_0001;
        const SOCKDIAG_LOCAL_PORT: u16 = 8080;
        const SOCKDIAG_LISTEN_BACKLOG: u8 = 2;
        const SOCKDIAG_UPDATED_BACKLOG: u8 = 1;

        let bind = socket_abi_dispatch!(userspace_socket_abi::bind(
            process_descriptor as u64,
            SOCKDIAG_LOCAL_IPV4_BE,
            SOCKDIAG_LOCAL_PORT,
        ));
        let bind_return = bind.return_value().x0();
        if bind_return != 0 {
            return Err(LocalCommandExecError::SyscallFailed);
        }

        let repeated_bind = socket_dispatch(
            syscall::TALOS_BIND_SYSCALL,
            syscall::SyscallArguments::new([
                process_descriptor as u64,
                SOCKDIAG_LOCAL_IPV4_BE as u64,
                SOCKDIAG_LOCAL_PORT as u64,
                0,
                0,
                0,
            ]),
            &mut self.descriptor_store,
            &mut self.socket_descriptors,
            &mut user_memory,
            &mut kernel_scratch,
        );
        if syscall::errno_number(posix::PosixError::InvalidArgument) as u64
            != repeated_bind.return_value().x0().wrapping_neg()
        {
            return Err(LocalCommandExecError::SyscallFailed);
        }

        let listen = socket_abi_dispatch!(userspace_socket_abi::listen(
            process_descriptor as u64,
            SOCKDIAG_LISTEN_BACKLOG as u64,
        ));
        let listen_return = listen.return_value().x0();
        if listen_return != 0 {
            return Err(LocalCommandExecError::SyscallFailed);
        }

        let repeated_listen = socket_dispatch(
            syscall::TALOS_LISTEN_SYSCALL,
            syscall::SyscallArguments::new([
                process_descriptor as u64,
                SOCKDIAG_UPDATED_BACKLOG as u64,
                0,
                0,
                0,
                0,
            ]),
            &mut self.descriptor_store,
            &mut self.socket_descriptors,
            &mut user_memory,
            &mut kernel_scratch,
        );
        if repeated_listen.return_value().x0() != 0 {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        let listening_socket = self
            .socket_descriptors
            .socket(socket_reference)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        if listening_socket.state()
            != (crate::network::NetworkSocketState::Listening {
                local_endpoint: crate::network::Ipv4Endpoint::new(
                    SOCKDIAG_LOCAL_IPV4_BE,
                    SOCKDIAG_LOCAL_PORT,
                ),
                backlog: SOCKDIAG_UPDATED_BACKLOG,
                pending: crate::network::NetworkSocketPendingQueue::new(),
            })
        {
            return Err(LocalCommandExecError::LaunchPipelineFailed);
        }

        let accept_before_connect = socket_dispatch(
            syscall::TALOS_ACCEPT_SYSCALL,
            syscall::SyscallArguments::new([process_descriptor as u64, 0, 0, 0, 0, 0]),
            &mut self.descriptor_store,
            &mut self.socket_descriptors,
            &mut user_memory,
            &mut kernel_scratch,
        );
        if syscall::errno_number(posix::PosixError::Again) as u64
            != accept_before_connect.return_value().x0().wrapping_neg()
        {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        local_write_poll_entry(
            &mut user_memory,
            0,
            process_descriptor as u64,
            syscall::TALOS_POLL_READ,
        );
        let empty_listener_poll = socket_dispatch(
            syscall::TALOS_POLL_SYSCALL,
            syscall::SyscallArguments::new([LOCAL_COMMAND_SOCKDIAG_USER_BASE, 1, 0, 0, 0, 0]),
            &mut self.descriptor_store,
            &mut self.socket_descriptors,
            &mut user_memory,
            &mut kernel_scratch,
        );
        if empty_listener_poll.return_value().x0() != 0 {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        let empty_listener_revents = local_read_poll_revents(&user_memory, 0);

        let client_open = socket_abi_dispatch!(userspace_socket_abi::inet_stream_socket());
        let client_descriptor = syscall_success_usize(client_open.return_value().x0())
            .map_err(|_| LocalCommandExecError::SyscallFailed)?;
        let client_entry = self
            .descriptor_store
            .current_descriptor_table(self.current_owner)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?
            .get(client_descriptor)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        let client_socket_reference =
            crate::network::NetworkSocketDescriptor::from_raw(client_entry.object().reference());

        let mut poll_waits = syscall::SocketPollWaitTable::<2>::new();
        let mut poll_wait_scheduler = scheduler::SingleCoreScheduler::<8>::new();
        let mut poll_wait_console = LocalCommandDiscardConsole;
        macro_rules! socket_poll_wait_dispatch {
            ($arguments:expr, $task:expr, $now_tick:expr) => {
                syscall::dispatch_process_descriptor_with_socket_table_and_poll_wait(
                    syscall::TALOS_POLL_WAIT_SYSCALL,
                    $arguments,
                    Some(owner),
                    $task,
                    $now_tick,
                    &mut self.descriptor_store,
                    &mut self.socket_descriptors,
                    &mut poll_waits,
                    &mappings,
                    LOCAL_COMMAND_SOCKDIAG_USER_BASE,
                    &mut user_memory,
                    &mut kernel_scratch,
                    &mut poll_wait_console,
                )
            };
        }

        let poll_wait_scalar = syscall::dispatch(
            syscall::TALOS_POLL_WAIT_SYSCALL,
            syscall::SyscallArguments::empty(),
        );
        if syscall::errno_number(posix::PosixError::NotSupported) as u64
            != poll_wait_scalar.return_value().x0().wrapping_neg()
        {
            return Err(LocalCommandExecError::SyscallFailed);
        }

        let mut invalid_timeout_task = local_sockdiag_poll_wait_task(0)?;
        local_write_poll_entry(
            &mut user_memory,
            0,
            process_descriptor as u64,
            syscall::TALOS_POLL_READ,
        );
        let invalid_timeout_wait = socket_poll_wait_dispatch!(
            userspace_socket_abi::poll_wait(LOCAL_COMMAND_SOCKDIAG_USER_BASE, 1, 0)
                .syscall_arguments(),
            &mut invalid_timeout_task,
            7
        );
        if invalid_timeout_wait.number() != syscall::SyscallNumber::TalosPollWait
            || invalid_timeout_wait.outcome()
                != syscall::SocketPollWaitOutcome::Completed(syscall::SyscallReturn::error(
                    posix::PosixError::InvalidArgument,
                ))
        {
            return Err(LocalCommandExecError::SyscallFailed);
        }

        let mut unsupported_wait_task = local_sockdiag_poll_wait_task(1)?;
        local_write_poll_entry(
            &mut user_memory,
            0,
            process_descriptor as u64,
            syscall::TALOS_POLL_READ | 0x10,
        );
        let unsupported_poll_wait = socket_poll_wait_dispatch!(
            userspace_socket_abi::SocketAbiCall::new(
                userspace_socket_abi::POLL_WAIT,
                [LOCAL_COMMAND_SOCKDIAG_USER_BASE, 1, 5, 0, 0, 0],
            )
            .syscall_arguments(),
            &mut unsupported_wait_task,
            8
        );
        if unsupported_poll_wait.outcome()
            != syscall::SocketPollWaitOutcome::Completed(syscall::SyscallReturn::error(
                posix::PosixError::InvalidArgument,
            ))
        {
            return Err(LocalCommandExecError::SyscallFailed);
        }

        let mut listener_wait_task = local_sockdiag_poll_wait_task(2)?;
        local_write_poll_entry(
            &mut user_memory,
            0,
            process_descriptor as u64,
            syscall::TALOS_POLL_READ,
        );
        let pending_listener_wait = socket_poll_wait_dispatch!(
            userspace_socket_abi::poll_wait(LOCAL_COMMAND_SOCKDIAG_USER_BASE, 1, 5)
                .syscall_arguments(),
            &mut listener_wait_task,
            10
        );
        if pending_listener_wait.outcome()
            != (syscall::SocketPollWaitOutcome::Blocked {
                task_id: listener_wait_task.id(),
                deadline_tick: 15,
            })
            || listener_wait_task.state() != TaskState::Blocked
            || !poll_waits.has_wait_for_task(listener_wait_task.id())
        {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        let poll_wait_blocked_state = local_task_state_name(listener_wait_task.state());

        let missing_listener = socket_dispatch(
            syscall::TALOS_CONNECT_SYSCALL,
            syscall::SyscallArguments::new([
                client_descriptor as u64,
                SOCKDIAG_LOCAL_IPV4_BE as u64,
                (SOCKDIAG_LOCAL_PORT + 1) as u64,
                0,
                0,
                0,
            ]),
            &mut self.descriptor_store,
            &mut self.socket_descriptors,
            &mut user_memory,
            &mut kernel_scratch,
        );
        if syscall::errno_number(posix::PosixError::InvalidArgument) as u64
            != missing_listener.return_value().x0().wrapping_neg()
        {
            return Err(LocalCommandExecError::SyscallFailed);
        }

        let non_socket_connect = socket_dispatch(
            syscall::TALOS_CONNECT_SYSCALL,
            syscall::SyscallArguments::new([
                posix::STDOUT_FD as u64,
                SOCKDIAG_LOCAL_IPV4_BE as u64,
                SOCKDIAG_LOCAL_PORT as u64,
                0,
                0,
                0,
            ]),
            &mut self.descriptor_store,
            &mut self.socket_descriptors,
            &mut user_memory,
            &mut kernel_scratch,
        );
        if syscall::errno_number(posix::PosixError::BadDescriptor) as u64
            != non_socket_connect.return_value().x0().wrapping_neg()
        {
            return Err(LocalCommandExecError::SyscallFailed);
        }

        let connect = socket_abi_dispatch!(userspace_socket_abi::connect(
            client_descriptor as u64,
            SOCKDIAG_LOCAL_IPV4_BE,
            SOCKDIAG_LOCAL_PORT,
        ));
        let connect_return = connect.return_value().x0();
        if connect_return != 0 {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        let smoltcp_connection_id = match self
            .socket_descriptors
            .socket(client_socket_reference)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?
            .state()
        {
            crate::network::NetworkSocketState::Connected { connection_id, .. } => connection_id,
            _ => return Err(LocalCommandExecError::LaunchPipelineFailed),
        };
        let smoltcp_handshake_record = self
            .socket_descriptors
            .smoltcp_bridge_record(smoltcp_connection_id)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        let smoltcp_handshake = smoltcp_handshake_record.handshake();
        if smoltcp_handshake_record.connection_id() != smoltcp_connection_id
            || smoltcp_handshake.client_state() != smoltcp::socket::tcp::State::Established
            || smoltcp_handshake.server_state() != smoltcp::socket::tcp::State::Established
            || smoltcp_handshake_record.payload_transfers() != 0
            || smoltcp_handshake_record.accepted_descriptor().is_some()
        {
            return Err(LocalCommandExecError::LaunchPipelineFailed);
        }
        let pending_listener_resume = poll_waits
            .resume_ready_or_expired(
                &mut listener_wait_task,
                &mut poll_wait_scheduler,
                &self.socket_descriptors,
                11,
                &mappings,
                LOCAL_COMMAND_SOCKDIAG_USER_BASE,
                &mut user_memory,
                &mut kernel_scratch,
            )
            .map_err(local_exec_error_from_posix)?;
        let poll_wait_ready_count = match pending_listener_resume {
            Some(syscall::SocketPollWaitResume::Ready {
                task_id,
                ready_count,
            }) if task_id == listener_wait_task.id() => ready_count,
            _ => return Err(LocalCommandExecError::SyscallFailed),
        };
        let poll_wait_pending_listener_revents = local_read_poll_revents(&user_memory, 0);
        let poll_wait_ready_state = local_task_state_name(listener_wait_task.state());
        local_write_poll_entry(
            &mut user_memory,
            0,
            process_descriptor as u64,
            syscall::TALOS_POLL_READ,
        );
        let pending_listener_poll = socket_abi_dispatch!(userspace_socket_abi::poll(
            LOCAL_COMMAND_SOCKDIAG_USER_BASE,
            1,
        ));
        if pending_listener_poll.return_value().x0() != 1 {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        let pending_listener_revents = local_read_poll_revents(&user_memory, 0);
        let mut immediate_wait_task = local_sockdiag_poll_wait_task(3)?;
        local_write_poll_entry(
            &mut user_memory,
            0,
            process_descriptor as u64,
            syscall::TALOS_POLL_READ,
        );
        let immediate_wait = socket_poll_wait_dispatch!(
            userspace_socket_abi::poll_wait(LOCAL_COMMAND_SOCKDIAG_USER_BASE, 1, 5)
                .syscall_arguments(),
            &mut immediate_wait_task,
            12
        );
        if immediate_wait.outcome()
            != syscall::SocketPollWaitOutcome::Completed(syscall::SyscallReturn::success(1))
            || immediate_wait_task.state() != TaskState::Runnable
        {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        let poll_wait_immediate_revents = local_read_poll_revents(&user_memory, 0);

        let second_client_open = socket_abi_dispatch!(userspace_socket_abi::inet_stream_socket());
        let second_client_descriptor =
            syscall_success_usize(second_client_open.return_value().x0())
                .map_err(|_| LocalCommandExecError::SyscallFailed)?;
        let second_client_entry = self
            .descriptor_store
            .current_descriptor_table(self.current_owner)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?
            .get(second_client_descriptor)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        let second_client_socket_reference = crate::network::NetworkSocketDescriptor::from_raw(
            second_client_entry.object().reference(),
        );

        let queue_backpressure = socket_dispatch(
            syscall::TALOS_CONNECT_SYSCALL,
            syscall::SyscallArguments::new([
                second_client_descriptor as u64,
                SOCKDIAG_LOCAL_IPV4_BE as u64,
                SOCKDIAG_LOCAL_PORT as u64,
                0,
                0,
                0,
            ]),
            &mut self.descriptor_store,
            &mut self.socket_descriptors,
            &mut user_memory,
            &mut kernel_scratch,
        );
        if syscall::errno_number(posix::PosixError::NoSpace) as u64
            != queue_backpressure.return_value().x0().wrapping_neg()
        {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        if self
            .socket_descriptors
            .socket(second_client_socket_reference)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?
            .state()
            != crate::network::NetworkSocketState::OpenUnbound
        {
            return Err(LocalCommandExecError::LaunchPipelineFailed);
        }

        let accept = socket_abi_dispatch!(userspace_socket_abi::accept(process_descriptor as u64));
        let accepted_descriptor = syscall_success_usize(accept.return_value().x0())
            .map_err(|_| LocalCommandExecError::SyscallFailed)?;
        let accepted_entry = self
            .descriptor_store
            .current_descriptor_table(self.current_owner)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?
            .get(accepted_descriptor)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        let accepted_socket_reference =
            crate::network::NetworkSocketDescriptor::from_raw(accepted_entry.object().reference());
        let smoltcp_accepted_record = self
            .socket_descriptors
            .smoltcp_bridge_record(smoltcp_connection_id)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        let smoltcp_accepted_attached =
            smoltcp_accepted_record.accepted_descriptor() == Some(accepted_socket_reference);
        if !smoltcp_accepted_attached {
            return Err(LocalCommandExecError::LaunchPipelineFailed);
        }
        let client_endpoint = crate::network::Ipv4Endpoint::new(
            crate::network::SOCKET_SYNTHETIC_LOCAL_IPV4_BE,
            crate::network::SOCKET_SYNTHETIC_CLIENT_PORT_BASE
                + client_socket_reference.raw() as u16,
        );
        let local_endpoint =
            crate::network::Ipv4Endpoint::new(SOCKDIAG_LOCAL_IPV4_BE, SOCKDIAG_LOCAL_PORT);
        if self
            .socket_descriptors
            .socket(socket_reference)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?
            .state()
            != (crate::network::NetworkSocketState::Listening {
                local_endpoint,
                backlog: SOCKDIAG_UPDATED_BACKLOG,
                pending: crate::network::NetworkSocketPendingQueue::new(),
            })
        {
            return Err(LocalCommandExecError::LaunchPipelineFailed);
        }
        if !matches!(
            self.socket_descriptors
                .socket(client_socket_reference)
                .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?
                .state(),
            crate::network::NetworkSocketState::Connected {
                local_endpoint: connected_local_endpoint,
                remote_endpoint: connected_remote_endpoint,
                recv_queue,
                ..
            } if connected_local_endpoint == client_endpoint
                && connected_remote_endpoint == local_endpoint
                && recv_queue == crate::network::NetworkSocketPayloadQueue::new()
        ) {
            return Err(LocalCommandExecError::LaunchPipelineFailed);
        }
        if !matches!(
            self.socket_descriptors
                .socket(accepted_socket_reference)
                .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?
                .state(),
            crate::network::NetworkSocketState::Accepted {
                local_endpoint: accepted_local_endpoint,
                remote_endpoint: accepted_remote_endpoint,
                recv_queue,
                ..
            } if accepted_local_endpoint == local_endpoint
                && accepted_remote_endpoint == client_endpoint
                && recv_queue == crate::network::NetworkSocketPayloadQueue::new()
        ) {
            return Err(LocalCommandExecError::LaunchPipelineFailed);
        }
        local_write_poll_entry(
            &mut user_memory,
            0,
            accepted_descriptor as u64,
            syscall::TALOS_POLL_READ,
        );
        let empty_recv_poll = socket_abi_dispatch!(userspace_socket_abi::poll(
            LOCAL_COMMAND_SOCKDIAG_USER_BASE,
            1,
        ));
        if empty_recv_poll.return_value().x0() != 0 {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        let empty_recv_revents = local_read_poll_revents(&user_memory, 0);
        local_write_poll_entry(
            &mut user_memory,
            0,
            accepted_descriptor as u64,
            syscall::TALOS_POLL_READ | 0x10,
        );
        let unsupported_poll_events = socket_dispatch(
            syscall::TALOS_POLL_SYSCALL,
            syscall::SyscallArguments::new([LOCAL_COMMAND_SOCKDIAG_USER_BASE, 1, 0, 0, 0, 0]),
            &mut self.descriptor_store,
            &mut self.socket_descriptors,
            &mut user_memory,
            &mut kernel_scratch,
        );
        if syscall::errno_number(posix::PosixError::InvalidArgument) as u64
            != unsupported_poll_events.return_value().x0().wrapping_neg()
        {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        local_write_poll_entry(
            &mut user_memory,
            0,
            client_descriptor as u64,
            syscall::TALOS_POLL_WRITE,
        );
        let write_ready_poll = socket_abi_dispatch!(userspace_socket_abi::poll(
            LOCAL_COMMAND_SOCKDIAG_USER_BASE,
            1,
        ));
        if write_ready_poll.return_value().x0() != 1 {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        let write_ready_revents = local_read_poll_revents(&user_memory, 0);

        let empty_recv = socket_abi_dispatch!(userspace_socket_abi::recv(
            accepted_descriptor as u64,
            LOCAL_COMMAND_SOCKDIAG_USER_BASE + 0x40,
            16,
        ));
        if syscall::errno_number(posix::PosixError::Again) as u64
            != empty_recv.return_value().x0().wrapping_neg()
        {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        let mut timeout_wait_task = local_sockdiag_poll_wait_task(4)?;
        local_write_poll_entry(
            &mut user_memory,
            0,
            accepted_descriptor as u64,
            syscall::TALOS_POLL_READ,
        );
        let timeout_wait = socket_poll_wait_dispatch!(
            userspace_socket_abi::poll_wait(LOCAL_COMMAND_SOCKDIAG_USER_BASE, 1, 2)
                .syscall_arguments(),
            &mut timeout_wait_task,
            20
        );
        if timeout_wait.outcome()
            != (syscall::SocketPollWaitOutcome::Blocked {
                task_id: timeout_wait_task.id(),
                deadline_tick: 22,
            })
        {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        if poll_waits
            .resume_ready_or_expired(
                &mut timeout_wait_task,
                &mut poll_wait_scheduler,
                &self.socket_descriptors,
                21,
                &mappings,
                LOCAL_COMMAND_SOCKDIAG_USER_BASE,
                &mut user_memory,
                &mut kernel_scratch,
            )
            .map_err(local_exec_error_from_posix)?
            .is_some()
        {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        let timeout_resume = poll_waits
            .resume_ready_or_expired(
                &mut timeout_wait_task,
                &mut poll_wait_scheduler,
                &self.socket_descriptors,
                22,
                &mappings,
                LOCAL_COMMAND_SOCKDIAG_USER_BASE,
                &mut user_memory,
                &mut kernel_scratch,
            )
            .map_err(local_exec_error_from_posix)?;
        if timeout_resume
            != Some(syscall::SocketPollWaitResume::Timeout {
                task_id: timeout_wait_task.id(),
            })
        {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        let poll_wait_timeout_revents = local_read_poll_revents(&user_memory, 0);
        let poll_wait_timeout_state = local_task_state_name(timeout_wait_task.state());
        let poll_wait_timeout_tick = 22;

        let send_invalid_flags = socket_dispatch(
            syscall::TALOS_SEND_SYSCALL,
            syscall::SyscallArguments::new([
                client_descriptor as u64,
                LOCAL_COMMAND_SOCKDIAG_USER_BASE,
                1,
                1,
                0,
                0,
            ]),
            &mut self.descriptor_store,
            &mut self.socket_descriptors,
            &mut user_memory,
            &mut kernel_scratch,
        );
        if syscall::errno_number(posix::PosixError::InvalidArgument) as u64
            != send_invalid_flags.return_value().x0().wrapping_neg()
        {
            return Err(LocalCommandExecError::SyscallFailed);
        }

        let recv_invalid_flags = socket_dispatch(
            syscall::TALOS_RECV_SYSCALL,
            syscall::SyscallArguments::new([
                accepted_descriptor as u64,
                LOCAL_COMMAND_SOCKDIAG_USER_BASE + 0x40,
                1,
                1,
                0,
                0,
            ]),
            &mut self.descriptor_store,
            &mut self.socket_descriptors,
            &mut user_memory,
            &mut kernel_scratch,
        );
        if syscall::errno_number(posix::PosixError::InvalidArgument) as u64
            != recv_invalid_flags.return_value().x0().wrapping_neg()
        {
            return Err(LocalCommandExecError::SyscallFailed);
        }

        const SOCKDIAG_CLIENT_PAYLOAD: &[u8] = b"client->server";
        const SOCKDIAG_SERVER_PAYLOAD: &[u8] = b"server->client";
        let mut payload_wait_task = local_sockdiag_poll_wait_task(5)?;
        local_write_poll_entry(
            &mut user_memory,
            0,
            accepted_descriptor as u64,
            syscall::TALOS_POLL_READ,
        );
        let payload_wait = socket_poll_wait_dispatch!(
            userspace_socket_abi::poll_wait(LOCAL_COMMAND_SOCKDIAG_USER_BASE, 1, 9)
                .syscall_arguments(),
            &mut payload_wait_task,
            30
        );
        if !matches!(
            payload_wait.outcome(),
            syscall::SocketPollWaitOutcome::Blocked {
                deadline_tick: 39,
                ..
            }
        ) {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        user_memory[..SOCKDIAG_CLIENT_PAYLOAD.len()].copy_from_slice(SOCKDIAG_CLIENT_PAYLOAD);
        let client_send = socket_abi_dispatch!(userspace_socket_abi::send(
            client_descriptor as u64,
            LOCAL_COMMAND_SOCKDIAG_USER_BASE,
            SOCKDIAG_CLIENT_PAYLOAD.len() as u64,
        ));
        if client_send.return_value().x0() != SOCKDIAG_CLIENT_PAYLOAD.len() as u64 {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        let smoltcp_payload_record = self
            .socket_descriptors
            .smoltcp_bridge_record(smoltcp_connection_id)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        let smoltcp_payload = smoltcp_payload_record.last_payload();
        if smoltcp_payload_record.payload_transfers() != 1
            || smoltcp_payload.payload_len() != SOCKDIAG_CLIENT_PAYLOAD.len()
            || smoltcp_payload.client_state() != smoltcp::socket::tcp::State::Established
            || smoltcp_payload.server_state() != smoltcp::socket::tcp::State::Established
        {
            return Err(LocalCommandExecError::LaunchPipelineFailed);
        }
        if !matches!(
            poll_waits
                .resume_ready_or_expired(
                    &mut payload_wait_task,
                    &mut poll_wait_scheduler,
                    &self.socket_descriptors,
                    31,
                    &mappings,
                    LOCAL_COMMAND_SOCKDIAG_USER_BASE,
                    &mut user_memory,
                    &mut kernel_scratch,
                )
                .map_err(local_exec_error_from_posix)?,
            Some(syscall::SocketPollWaitResume::Ready { ready_count: 1, .. })
        ) {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        let poll_wait_payload_revents = local_read_poll_revents(&user_memory, 0);
        local_write_poll_entry(
            &mut user_memory,
            0,
            accepted_descriptor as u64,
            syscall::TALOS_POLL_READ,
        );
        let payload_recv_poll = socket_abi_dispatch!(userspace_socket_abi::poll(
            LOCAL_COMMAND_SOCKDIAG_USER_BASE,
            1,
        ));
        if payload_recv_poll.return_value().x0() != 1 {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        let payload_recv_revents = local_read_poll_revents(&user_memory, 0);
        let server_recv = socket_abi_dispatch!(userspace_socket_abi::recv(
            accepted_descriptor as u64,
            LOCAL_COMMAND_SOCKDIAG_USER_BASE + 0x40,
            32,
        ));
        if server_recv.return_value().x0() != SOCKDIAG_CLIENT_PAYLOAD.len() as u64
            || &user_memory[0x40..0x40 + SOCKDIAG_CLIENT_PAYLOAD.len()] != SOCKDIAG_CLIENT_PAYLOAD
        {
            return Err(LocalCommandExecError::SyscallFailed);
        }

        user_memory[0x20..0x20 + SOCKDIAG_SERVER_PAYLOAD.len()]
            .copy_from_slice(SOCKDIAG_SERVER_PAYLOAD);
        let server_send = socket_abi_dispatch!(userspace_socket_abi::send(
            accepted_descriptor as u64,
            LOCAL_COMMAND_SOCKDIAG_USER_BASE + 0x20,
            SOCKDIAG_SERVER_PAYLOAD.len() as u64,
        ));
        if server_send.return_value().x0() != SOCKDIAG_SERVER_PAYLOAD.len() as u64 {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        let client_recv = socket_abi_dispatch!(userspace_socket_abi::recv(
            client_descriptor as u64,
            LOCAL_COMMAND_SOCKDIAG_USER_BASE + 0x60,
            32,
        ));
        if client_recv.return_value().x0() != SOCKDIAG_SERVER_PAYLOAD.len() as u64
            || &user_memory[0x60..0x60 + SOCKDIAG_SERVER_PAYLOAD.len()] != SOCKDIAG_SERVER_PAYLOAD
        {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        let mut driver_packet_adapter = crate::network::DriverPacketAdapter::<1, 1, 64>::new();
        driver_packet_adapter
            .inject_driver_rx(&[0xde, 0xad, 0xbe, 0xef])
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        let driver_packet_rx =
            driver_packet_adapter.receive_one_for_smoltcp(smoltcp::time::Instant::from_micros(1));
        let driver_packet_tx = driver_packet_adapter
            .transmit_one_from_smoltcp(smoltcp::time::Instant::from_micros(2), &[0xca, 0xfe, 0xba]);
        let driver_packet_tx_observed = driver_packet_adapter
            .pop_driver_tx()
            .ok_or(LocalCommandExecError::LaunchPipelineFailed)?;
        let driver_packet_tx_observed_len = driver_packet_tx_observed.len();
        if driver_packet_tx_observed.as_bytes() != [0xca, 0xfe, 0xba] {
            return Err(LocalCommandExecError::LaunchPipelineFailed);
        }
        let driver_packet_tx_queued_after_pop = driver_packet_adapter.driver_tx_len();
        let mut backpressure_adapter = crate::network::DriverPacketAdapter::<1, 1, 64>::new();
        if backpressure_adapter
            .transmit_one_from_smoltcp(smoltcp::time::Instant::from_micros(3), &[0x01, 0x02])
            != (crate::network::DriverPacketAdapterTransmitStep::Transmitted { frame_len: 2 })
        {
            return Err(LocalCommandExecError::LaunchPipelineFailed);
        }
        backpressure_adapter
            .inject_driver_rx(&[0x03, 0x04, 0x05])
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        let driver_packet_backpressure =
            backpressure_adapter.receive_one_for_smoltcp(smoltcp::time::Instant::from_micros(4));
        let driver_packet_backpressure_rx_queued = backpressure_adapter.driver_rx_len();
        let driver_packet_backpressure_tx_queued = backpressure_adapter.driver_tx_len();

        let mut index = 0usize;
        while index < crate::network::SOCKET_PAYLOAD_QUEUE_CAPACITY {
            user_memory[index] = index as u8;
            index += 1;
        }
        let fill_queue = socket_abi_dispatch!(userspace_socket_abi::send(
            client_descriptor as u64,
            LOCAL_COMMAND_SOCKDIAG_USER_BASE,
            crate::network::SOCKET_PAYLOAD_QUEUE_CAPACITY as u64,
        ));
        if fill_queue.return_value().x0() != crate::network::SOCKET_PAYLOAD_QUEUE_CAPACITY as u64 {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        let full_queue_send = socket_abi_dispatch!(userspace_socket_abi::send(
            client_descriptor as u64,
            LOCAL_COMMAND_SOCKDIAG_USER_BASE,
            1,
        ));
        if syscall::errno_number(posix::PosixError::NoSpace) as u64
            != full_queue_send.return_value().x0().wrapping_neg()
        {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        local_write_poll_entry(
            &mut user_memory,
            0,
            client_descriptor as u64,
            syscall::TALOS_POLL_WRITE,
        );
        let write_backpressure_poll = socket_abi_dispatch!(userspace_socket_abi::poll(
            LOCAL_COMMAND_SOCKDIAG_USER_BASE,
            1,
        ));
        if write_backpressure_poll.return_value().x0() != 0 {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        let write_backpressure_revents = local_read_poll_revents(&user_memory, 0);
        let drain_queue = socket_abi_dispatch!(userspace_socket_abi::recv(
            accepted_descriptor as u64,
            LOCAL_COMMAND_SOCKDIAG_USER_BASE + 0x40,
            crate::network::SOCKET_PAYLOAD_QUEUE_CAPACITY as u64,
        ));
        if drain_queue.return_value().x0() != crate::network::SOCKET_PAYLOAD_QUEUE_CAPACITY as u64 {
            return Err(LocalCommandExecError::SyscallFailed);
        }

        let mut hangup_wait_task = local_sockdiag_poll_wait_task(6)?;
        local_write_poll_entry(
            &mut user_memory,
            0,
            client_descriptor as u64,
            syscall::TALOS_POLL_READ,
        );
        let hangup_wait = socket_poll_wait_dispatch!(
            userspace_socket_abi::poll_wait(LOCAL_COMMAND_SOCKDIAG_USER_BASE, 1, 8)
                .syscall_arguments(),
            &mut hangup_wait_task,
            40
        );
        if !matches!(
            hangup_wait.outcome(),
            syscall::SocketPollWaitOutcome::Blocked {
                deadline_tick: 48,
                ..
            }
        ) {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        let accepted_close =
            socket_abi_dispatch!(userspace_socket_abi::close(accepted_descriptor as u64));
        if accepted_close.return_value().x0() != 0 {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        if !matches!(
            poll_waits
                .resume_ready_or_expired(
                    &mut hangup_wait_task,
                    &mut poll_wait_scheduler,
                    &self.socket_descriptors,
                    41,
                    &mappings,
                    LOCAL_COMMAND_SOCKDIAG_USER_BASE,
                    &mut user_memory,
                    &mut kernel_scratch,
                )
                .map_err(local_exec_error_from_posix)?,
            Some(syscall::SocketPollWaitResume::Ready { ready_count: 1, .. })
        ) {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        let poll_wait_peer_hangup_revents = local_read_poll_revents(&user_memory, 0);
        local_write_poll_entry(
            &mut user_memory,
            0,
            client_descriptor as u64,
            syscall::TALOS_POLL_READ | syscall::TALOS_POLL_WRITE,
        );
        local_write_poll_entry(&mut user_memory, 1, 99, syscall::TALOS_POLL_READ);
        local_write_poll_entry(
            &mut user_memory,
            2,
            posix::STDOUT_FD as u64,
            syscall::TALOS_POLL_READ,
        );
        let hangup_and_errors_poll = socket_abi_dispatch!(userspace_socket_abi::poll(
            LOCAL_COMMAND_SOCKDIAG_USER_BASE,
            3,
        ));
        if hangup_and_errors_poll.return_value().x0() != 3 {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        let peer_hangup_revents = local_read_poll_revents(&user_memory, 0);
        let invalid_descriptor_revents = local_read_poll_revents(&user_memory, 1);
        let non_socket_descriptor_revents = local_read_poll_revents(&user_memory, 2);
        let send_after_peer_close = socket_abi_dispatch!(userspace_socket_abi::send(
            client_descriptor as u64,
            LOCAL_COMMAND_SOCKDIAG_USER_BASE,
            1,
        ));
        if syscall::errno_number(posix::PosixError::Pipe) as u64
            != send_after_peer_close.return_value().x0().wrapping_neg()
        {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        let second_client_close =
            socket_abi_dispatch!(userspace_socket_abi::close(second_client_descriptor as u64,));
        if second_client_close.return_value().x0() != 0 {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        let client_close =
            socket_abi_dispatch!(userspace_socket_abi::close(client_descriptor as u64));
        if client_close.return_value().x0() != 0 {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        let close = socket_abi_dispatch!(userspace_socket_abi::close(process_descriptor as u64));
        let close_return = close.return_value().x0();
        if close_return != 0 {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        let closed = socket_dispatch(
            syscall::TALOS_CLOSE_SYSCALL,
            syscall::SyscallArguments::new([process_descriptor as u64, 0, 0, 0, 0, 0]),
            &mut self.descriptor_store,
            &mut self.socket_descriptors,
            &mut user_memory,
            &mut kernel_scratch,
        );
        if syscall::errno_number(posix::PosixError::BadDescriptor) as u64
            != closed.return_value().x0().wrapping_neg()
        {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        drop(socket_dispatch);

        let cross_process_server_owner = owner;
        let cross_process_client_owner =
            ProcessOwnerId::new(2).ok_or(LocalCommandExecError::LaunchPipelineFailed)?;
        match self
            .descriptor_store
            .create_owner_with_inherited_stdio(cross_process_client_owner)
        {
            Ok(()) | Err(posix::PosixError::InvalidArgument) => {}
            Err(_) => return Err(LocalCommandExecError::LaunchPipelineFailed),
        }
        let mut cross_process_console = LocalCommandDiscardConsole;
        macro_rules! cross_process_socket_dispatch {
            ($raw_number:expr, $arguments:expr, $owner:expr) => {
                syscall::dispatch_process_descriptor_with_socket_table(
                    $raw_number,
                    $arguments,
                    Some($owner),
                    &mut self.descriptor_store,
                    &mut self.socket_descriptors,
                    &mappings,
                    LOCAL_COMMAND_SOCKDIAG_USER_BASE,
                    &mut user_memory,
                    &mut kernel_scratch,
                    &mut cross_process_console,
                )
            };
        }

        const CROSS_PROCESS_LOCAL_IPV4_BE: u32 = 0x7f00_0002;
        const CROSS_PROCESS_LOCAL_PORT: u16 = 9090;
        const CROSS_PROCESS_CLIENT_PAYLOAD: &[u8] = b"cross-client";
        const CROSS_PROCESS_SERVER_PAYLOAD: &[u8] = b"cross-server";
        let cross_server_open = cross_process_socket_dispatch!(
            syscall::TALOS_SOCKET_SYSCALL,
            syscall::SyscallArguments::new([
                crate::network::SOCKET_DOMAIN_AF_INET,
                crate::network::SOCKET_TYPE_STREAM,
                crate::network::SOCKET_PROTOCOL_DEFAULT,
                0,
                0,
                0,
            ]),
            cross_process_server_owner
        );
        let cross_process_server_descriptor =
            syscall_success_usize(cross_server_open.return_value().x0())
                .map_err(|_| LocalCommandExecError::SyscallFailed)?;
        let cross_server_entry = self
            .descriptor_store
            .descriptor_table(cross_process_server_owner)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?
            .get(cross_process_server_descriptor)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        let cross_server_socket_reference = crate::network::NetworkSocketDescriptor::from_raw(
            cross_server_entry.object().reference(),
        );
        if cross_server_entry.object().kind() != posix::DescriptorObjectKind::Socket {
            return Err(LocalCommandExecError::LaunchPipelineFailed);
        }
        if cross_process_socket_dispatch!(
            syscall::TALOS_BIND_SYSCALL,
            syscall::SyscallArguments::new([
                cross_process_server_descriptor as u64,
                CROSS_PROCESS_LOCAL_IPV4_BE as u64,
                CROSS_PROCESS_LOCAL_PORT as u64,
                0,
                0,
                0,
            ]),
            cross_process_server_owner
        )
        .return_value()
        .x0()
            != 0
        {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        if cross_process_socket_dispatch!(
            syscall::TALOS_LISTEN_SYSCALL,
            syscall::SyscallArguments::new([cross_process_server_descriptor as u64, 1, 0, 0, 0, 0]),
            cross_process_server_owner
        )
        .return_value()
        .x0()
            != 0
        {
            return Err(LocalCommandExecError::SyscallFailed);
        }

        let mut cross_process_accept_wait_task = local_sockdiag_poll_wait_task(7)?;
        local_write_poll_entry(
            &mut user_memory,
            0,
            cross_process_server_descriptor as u64,
            syscall::TALOS_POLL_READ,
        );
        let cross_process_accept_wait =
            syscall::dispatch_process_descriptor_with_socket_table_and_poll_wait(
                syscall::TALOS_POLL_WAIT_SYSCALL,
                syscall::SyscallArguments::new([LOCAL_COMMAND_SOCKDIAG_USER_BASE, 1, 5, 0, 0, 0]),
                Some(cross_process_server_owner),
                &mut cross_process_accept_wait_task,
                50,
                &mut self.descriptor_store,
                &mut self.socket_descriptors,
                &mut poll_waits,
                &mappings,
                LOCAL_COMMAND_SOCKDIAG_USER_BASE,
                &mut user_memory,
                &mut kernel_scratch,
                &mut cross_process_console,
            );
        if !matches!(
            cross_process_accept_wait.outcome(),
            syscall::SocketPollWaitOutcome::Blocked {
                deadline_tick: 55,
                ..
            }
        ) {
            return Err(LocalCommandExecError::SyscallFailed);
        }

        let cross_client_open = cross_process_socket_dispatch!(
            syscall::TALOS_SOCKET_SYSCALL,
            syscall::SyscallArguments::new([
                crate::network::SOCKET_DOMAIN_AF_INET,
                crate::network::SOCKET_TYPE_STREAM,
                crate::network::SOCKET_PROTOCOL_DEFAULT,
                0,
                0,
                0,
            ]),
            cross_process_client_owner
        );
        let cross_process_client_descriptor =
            syscall_success_usize(cross_client_open.return_value().x0())
                .map_err(|_| LocalCommandExecError::SyscallFailed)?;
        let cross_client_entry = self
            .descriptor_store
            .descriptor_table(cross_process_client_owner)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?
            .get(cross_process_client_descriptor)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        let cross_client_socket_reference = crate::network::NetworkSocketDescriptor::from_raw(
            cross_client_entry.object().reference(),
        );
        if cross_process_socket_dispatch!(
            syscall::TALOS_CONNECT_SYSCALL,
            syscall::SyscallArguments::new([
                cross_process_client_descriptor as u64,
                CROSS_PROCESS_LOCAL_IPV4_BE as u64,
                CROSS_PROCESS_LOCAL_PORT as u64,
                0,
                0,
                0,
            ]),
            cross_process_client_owner
        )
        .return_value()
        .x0()
            != 0
        {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        if !matches!(
            poll_waits
                .resume_ready_or_expired(
                    &mut cross_process_accept_wait_task,
                    &mut poll_wait_scheduler,
                    &self.socket_descriptors,
                    51,
                    &mappings,
                    LOCAL_COMMAND_SOCKDIAG_USER_BASE,
                    &mut user_memory,
                    &mut kernel_scratch,
                )
                .map_err(local_exec_error_from_posix)?,
            Some(syscall::SocketPollWaitResume::Ready { ready_count: 1, .. })
        ) {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        let cross_process_accept_wait_revents = local_read_poll_revents(&user_memory, 0);

        local_write_poll_entry(
            &mut user_memory,
            0,
            cross_process_server_descriptor as u64,
            syscall::TALOS_POLL_READ,
        );
        if cross_process_socket_dispatch!(
            syscall::TALOS_POLL_SYSCALL,
            syscall::SyscallArguments::new([LOCAL_COMMAND_SOCKDIAG_USER_BASE, 1, 0, 0, 0, 0]),
            cross_process_server_owner
        )
        .return_value()
        .x0()
            != 1
        {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        let cross_process_listener_revents = local_read_poll_revents(&user_memory, 0);
        let cross_accept = cross_process_socket_dispatch!(
            syscall::TALOS_ACCEPT_SYSCALL,
            syscall::SyscallArguments::new([cross_process_server_descriptor as u64, 0, 0, 0, 0, 0]),
            cross_process_server_owner
        );
        let cross_process_accepted_descriptor =
            syscall_success_usize(cross_accept.return_value().x0())
                .map_err(|_| LocalCommandExecError::SyscallFailed)?;
        if self
            .descriptor_store
            .descriptor_table(cross_process_client_owner)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?
            .get(cross_process_accepted_descriptor)
            .is_ok()
        {
            return Err(LocalCommandExecError::LaunchPipelineFailed);
        }
        let cross_accepted_entry = self
            .descriptor_store
            .descriptor_table(cross_process_server_owner)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?
            .get(cross_process_accepted_descriptor)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        let cross_accepted_socket_reference = crate::network::NetworkSocketDescriptor::from_raw(
            cross_accepted_entry.object().reference(),
        );

        let mut cross_process_payload_wait_task = local_sockdiag_poll_wait_task(8)?;
        local_write_poll_entry(
            &mut user_memory,
            0,
            cross_process_accepted_descriptor as u64,
            syscall::TALOS_POLL_READ,
        );
        let cross_process_payload_wait =
            syscall::dispatch_process_descriptor_with_socket_table_and_poll_wait(
                syscall::TALOS_POLL_WAIT_SYSCALL,
                syscall::SyscallArguments::new([LOCAL_COMMAND_SOCKDIAG_USER_BASE, 1, 5, 0, 0, 0]),
                Some(cross_process_server_owner),
                &mut cross_process_payload_wait_task,
                60,
                &mut self.descriptor_store,
                &mut self.socket_descriptors,
                &mut poll_waits,
                &mappings,
                LOCAL_COMMAND_SOCKDIAG_USER_BASE,
                &mut user_memory,
                &mut kernel_scratch,
                &mut cross_process_console,
            );
        if !matches!(
            cross_process_payload_wait.outcome(),
            syscall::SocketPollWaitOutcome::Blocked {
                deadline_tick: 65,
                ..
            }
        ) {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        user_memory[..CROSS_PROCESS_CLIENT_PAYLOAD.len()]
            .copy_from_slice(CROSS_PROCESS_CLIENT_PAYLOAD);
        if cross_process_socket_dispatch!(
            syscall::TALOS_SEND_SYSCALL,
            syscall::SyscallArguments::new([
                cross_process_client_descriptor as u64,
                LOCAL_COMMAND_SOCKDIAG_USER_BASE,
                CROSS_PROCESS_CLIENT_PAYLOAD.len() as u64,
                0,
                0,
                0,
            ]),
            cross_process_client_owner
        )
        .return_value()
        .x0()
            != CROSS_PROCESS_CLIENT_PAYLOAD.len() as u64
        {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        if !matches!(
            poll_waits
                .resume_ready_or_expired(
                    &mut cross_process_payload_wait_task,
                    &mut poll_wait_scheduler,
                    &self.socket_descriptors,
                    61,
                    &mappings,
                    LOCAL_COMMAND_SOCKDIAG_USER_BASE,
                    &mut user_memory,
                    &mut kernel_scratch,
                )
                .map_err(local_exec_error_from_posix)?,
            Some(syscall::SocketPollWaitResume::Ready { ready_count: 1, .. })
        ) {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        let cross_process_payload_wait_revents = local_read_poll_revents(&user_memory, 0);
        local_write_poll_entry(
            &mut user_memory,
            0,
            cross_process_accepted_descriptor as u64,
            syscall::TALOS_POLL_READ,
        );
        if cross_process_socket_dispatch!(
            syscall::TALOS_POLL_SYSCALL,
            syscall::SyscallArguments::new([LOCAL_COMMAND_SOCKDIAG_USER_BASE, 1, 0, 0, 0, 0]),
            cross_process_server_owner
        )
        .return_value()
        .x0()
            != 1
        {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        let cross_process_payload_revents = local_read_poll_revents(&user_memory, 0);
        let cross_server_recv = cross_process_socket_dispatch!(
            syscall::TALOS_RECV_SYSCALL,
            syscall::SyscallArguments::new([
                cross_process_accepted_descriptor as u64,
                LOCAL_COMMAND_SOCKDIAG_USER_BASE + 0x40,
                32,
                0,
                0,
                0,
            ]),
            cross_process_server_owner
        );
        if cross_server_recv.return_value().x0() != CROSS_PROCESS_CLIENT_PAYLOAD.len() as u64
            || &user_memory[0x40..0x40 + CROSS_PROCESS_CLIENT_PAYLOAD.len()]
                != CROSS_PROCESS_CLIENT_PAYLOAD
        {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        user_memory[0x20..0x20 + CROSS_PROCESS_SERVER_PAYLOAD.len()]
            .copy_from_slice(CROSS_PROCESS_SERVER_PAYLOAD);
        if cross_process_socket_dispatch!(
            syscall::TALOS_SEND_SYSCALL,
            syscall::SyscallArguments::new([
                cross_process_accepted_descriptor as u64,
                LOCAL_COMMAND_SOCKDIAG_USER_BASE + 0x20,
                CROSS_PROCESS_SERVER_PAYLOAD.len() as u64,
                0,
                0,
                0,
            ]),
            cross_process_server_owner
        )
        .return_value()
        .x0()
            != CROSS_PROCESS_SERVER_PAYLOAD.len() as u64
        {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        let cross_client_recv = cross_process_socket_dispatch!(
            syscall::TALOS_RECV_SYSCALL,
            syscall::SyscallArguments::new([
                cross_process_client_descriptor as u64,
                LOCAL_COMMAND_SOCKDIAG_USER_BASE + 0x60,
                32,
                0,
                0,
                0,
            ]),
            cross_process_client_owner
        );
        if cross_client_recv.return_value().x0() != CROSS_PROCESS_SERVER_PAYLOAD.len() as u64
            || &user_memory[0x60..0x60 + CROSS_PROCESS_SERVER_PAYLOAD.len()]
                != CROSS_PROCESS_SERVER_PAYLOAD
        {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        if cross_process_socket_dispatch!(
            syscall::TALOS_CLOSE_SYSCALL,
            syscall::SyscallArguments::new([cross_process_client_descriptor as u64, 0, 0, 0, 0, 0]),
            cross_process_client_owner
        )
        .return_value()
        .x0()
            != 0
        {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        local_write_poll_entry(
            &mut user_memory,
            0,
            cross_process_accepted_descriptor as u64,
            syscall::TALOS_POLL_READ | syscall::TALOS_POLL_WRITE,
        );
        if cross_process_socket_dispatch!(
            syscall::TALOS_POLL_SYSCALL,
            syscall::SyscallArguments::new([LOCAL_COMMAND_SOCKDIAG_USER_BASE, 1, 0, 0, 0, 0]),
            cross_process_server_owner
        )
        .return_value()
        .x0()
            != 1
        {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        let cross_process_hangup_revents = local_read_poll_revents(&user_memory, 0);
        let cross_process_cleanup_close = cross_process_socket_dispatch!(
            syscall::TALOS_CLOSE_SYSCALL,
            syscall::SyscallArguments::new([
                cross_process_accepted_descriptor as u64,
                0,
                0,
                0,
                0,
                0
            ]),
            cross_process_server_owner
        );
        let cross_process_cleanup_close_return = cross_process_cleanup_close.return_value().x0();
        if cross_process_cleanup_close_return != 0 {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        if cross_process_socket_dispatch!(
            syscall::TALOS_CLOSE_SYSCALL,
            syscall::SyscallArguments::new([cross_process_server_descriptor as u64, 0, 0, 0, 0, 0]),
            cross_process_server_owner
        )
        .return_value()
        .x0()
            != 0
        {
            return Err(LocalCommandExecError::SyscallFailed);
        }
        let cross_process_backing_closed = self
            .socket_descriptors
            .socket(cross_server_socket_reference)
            .is_err()
            && self
                .socket_descriptors
                .socket(cross_client_socket_reference)
                .is_err()
            && self
                .socket_descriptors
                .socket(cross_accepted_socket_reference)
                .is_err();

        Ok(LocalCommandSockdiagRecord {
            process_descriptor,
            client_descriptor,
            accepted_descriptor,
            empty_listener_revents,
            pending_listener_revents,
            empty_recv_revents,
            payload_recv_revents,
            write_ready_revents,
            write_backpressure_revents,
            peer_hangup_revents,
            invalid_descriptor_revents,
            non_socket_descriptor_revents,
            poll_wait_immediate_revents,
            poll_wait_pending_listener_revents,
            poll_wait_payload_revents,
            poll_wait_timeout_revents,
            poll_wait_peer_hangup_revents,
            poll_wait_blocked_state,
            poll_wait_ready_state,
            poll_wait_timeout_state,
            poll_wait_ready_count,
            poll_wait_timeout_tick,
            client_send_bytes: SOCKDIAG_CLIENT_PAYLOAD.len(),
            server_recv_bytes: SOCKDIAG_CLIENT_PAYLOAD.len(),
            server_send_bytes: SOCKDIAG_SERVER_PAYLOAD.len(),
            client_recv_bytes: SOCKDIAG_SERVER_PAYLOAD.len(),
            client_payload: "client->server",
            server_payload: "server->client",
            domain: crate::network::SOCKET_DOMAIN_AF_INET,
            socket_type: crate::network::SOCKET_TYPE_STREAM,
            protocol: crate::network::SOCKET_PROTOCOL_DEFAULT,
            local_ipv4_be: SOCKDIAG_LOCAL_IPV4_BE,
            local_port: SOCKDIAG_LOCAL_PORT,
            bind_return,
            listen_backlog: SOCKDIAG_UPDATED_BACKLOG,
            listen_return,
            connect_return,
            accept_return: accepted_descriptor as u64,
            socket_state: "listening",
            client_state: "connected",
            accepted_state: "accepted",
            descriptor_kind: entry.object().kind().name(),
            descriptor_access: entry.access().name(),
            close_return,
            backing_closed: self.socket_descriptors.socket(socket_reference).is_err()
                && self
                    .socket_descriptors
                    .socket(client_socket_reference)
                    .is_err()
                && self
                    .socket_descriptors
                    .socket(second_client_socket_reference)
                    .is_err()
                && self
                    .socket_descriptors
                    .socket(accepted_socket_reference)
                    .is_err(),
            cross_process_server_owner: cross_process_server_owner.raw(),
            cross_process_client_owner: cross_process_client_owner.raw(),
            cross_process_server_descriptor,
            cross_process_client_descriptor,
            cross_process_accepted_descriptor,
            cross_process_listener_revents,
            cross_process_payload_revents,
            cross_process_hangup_revents,
            cross_process_accept_wait_revents,
            cross_process_payload_wait_revents,
            cross_process_cleanup_close_return,
            cross_process_payload: "cross-client",
            cross_process_reply: "cross-server",
            cross_process_descriptor_ownership: "server-owner-listener-accepted+client-owner-connected",
            cross_process_backing_closed,
            smoltcp_connection_id,
            smoltcp_handshake_client_state: local_smoltcp_tcp_state_name(
                smoltcp_handshake.client_state(),
            ),
            smoltcp_handshake_server_state: local_smoltcp_tcp_state_name(
                smoltcp_handshake.server_state(),
            ),
            smoltcp_handshake_steps: smoltcp_handshake.steps(),
            smoltcp_handshake_client_to_server_frames: smoltcp_handshake.client_to_server_frames(),
            smoltcp_handshake_server_to_client_frames: smoltcp_handshake.server_to_client_frames(),
            smoltcp_accepted_attached,
            smoltcp_payload_transfers: smoltcp_payload_record.payload_transfers(),
            smoltcp_payload_len: smoltcp_payload.payload_len(),
            smoltcp_payload_client_state: local_smoltcp_tcp_state_name(
                smoltcp_payload.client_state(),
            ),
            smoltcp_payload_server_state: local_smoltcp_tcp_state_name(
                smoltcp_payload.server_state(),
            ),
            driver_packet_rx_step: local_driver_packet_rx_step_name(driver_packet_rx),
            driver_packet_rx_frame_len: local_driver_packet_rx_frame_len(driver_packet_rx),
            driver_packet_tx_step: local_driver_packet_tx_step_name(driver_packet_tx),
            driver_packet_tx_frame_len: local_driver_packet_tx_frame_len(driver_packet_tx),
            driver_packet_tx_observed_len,
            driver_packet_tx_queued_after_pop,
            driver_packet_backpressure_step: local_driver_packet_rx_step_name(
                driver_packet_backpressure,
            ),
            driver_packet_backpressure_rx_queued,
            driver_packet_backpressure_tx_queued,
            driver_packet_evidence: "host-qemu-substitute-not-live-packet-io",
            source: "vfs-userspace-sockdiag+userspace-socket-abi-v1+talos-socket-bind-listen-connect-accept-send-recv-poll-wait-close+process-descriptor+cross-process-local-rendezvous+private-smoltcp-tcp-bridge+driver-packet-adapter-substrate",
        })
    }

    fn exec_shell_pingdiag_diagnostic(
        &mut self,
    ) -> Result<LocalCommandPingdiagRecord, LocalCommandExecError> {
        const USER_START: u64 = 0x0000_0000_0025_0000;
        const PAYLOAD_USER: u64 = USER_START;
        const PUMP_USER: u64 = USER_START + 0x40;
        const STATUS_USER: u64 = USER_START + 0x90;

        let owner = self
            .current_owner
            .ok_or(LocalCommandExecError::LaunchPipelineFailed)?;
        let mappings =
            [
                posix::UserMapping::new(
                    USER_START,
                    0x100,
                    posix::UserMappingPermissions::USER_DATA,
                )
                .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?,
            ];
        let mut user_memory = [0u8; 0x100];
        let mut kernel_scratch = [0u8; 64];
        let mut runtime =
            crate::network::NetworkRuntimeDevicePump::<2, 1, 2, 4>::new(local_ping_endpoint());
        let mut receive_buffer = [0u8; 128];
        let mut transmit_buffer = [0u8; 128];
        let mut driver_receive_buffer = [0u8; 128];
        let mut queue = crate::network::PacketQueueNetworkDevice::<2, 2, 128>::new();
        let mut driver = crate::network::PacketQueueNetworkDevice::<2, 2, 128>::new();
        let mut step = syscall::PingOperationSyscallSubstituteStep::from_userspace(
            crate::network::UserspacePingOperationStep::NoFrame,
        );
        let mut pump_step = syscall::RuntimePingOperationSyscallSubstitutePumpStep::no_frame();
        let mut status = syscall::PingOperationSyscallSubstituteStatus::idle();
        let payload = [1, 2, 3, 4];
        let destination = [192, 0, 2, 20];
        let mut fixture = syscall::VfsPingDiagnosticSvcFixture::new(
            initramfs::phase8_readonly_initramfs_fixture(),
            initramfs::PHASE12_PINGDIAG_PATH,
            &mappings,
            USER_START,
            &mut user_memory,
            &mut kernel_scratch,
            PAYLOAD_USER,
            PUMP_USER,
            STATUS_USER,
        )
        .map_err(local_exec_error_from_posix)?;
        fixture
            .write_payload(&payload)
            .map_err(local_exec_error_from_posix)?;

        let process_descriptor = {
            let mut outputs = syscall::ProcessLocalPingDispatchOutputs::new(
                &mut step,
                &mut pump_step,
                &mut status,
            );
            match fixture
                .dispatch(
                    fixture.open_arguments(),
                    Some(owner),
                    &mut self.descriptor_store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut queue,
                    &mut outputs,
                )
                .map_err(local_exec_error_from_posix)?
            {
                syscall::ProcessLocalPingDispatchOutcome::Opened { process_descriptor } => {
                    process_descriptor
                }
                _ => return Err(LocalCommandExecError::LaunchPipelineFailed),
            }
        };

        {
            let mut outputs = syscall::ProcessLocalPingDispatchOutputs::new(
                &mut step,
                &mut pump_step,
                &mut status,
            );
            fixture
                .dispatch(
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
                    &mut self.descriptor_store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut queue,
                    &mut outputs,
                )
                .map_err(local_exec_error_from_posix)?;
        }
        let start_step = step.kind();
        let (arp_driver_step, arp_frame_len) =
            local_packet_pump_transmit(queue.pump_driver(&mut driver, &mut driver_receive_buffer))?;
        driver
            .inject_received(&local_ping_arp_reply_frame())
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        local_packet_pump_receive(queue.pump_driver(&mut driver, &mut driver_receive_buffer))?;
        {
            let mut outputs = syscall::ProcessLocalPingDispatchOutputs::new(
                &mut step,
                &mut pump_step,
                &mut status,
            );
            fixture
                .dispatch(
                    fixture.pump_or_read_result_arguments(
                        process_descriptor,
                        syscall::PROCESS_LOCAL_PING_USER_PUMP_RECORD_LEN,
                    ),
                    Some(owner),
                    &mut self.descriptor_store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut queue,
                    &mut outputs,
                )
                .map_err(local_exec_error_from_posix)?;
        }
        let arp_pump_step = pump_step.active_ping_step().kind();
        let (icmp_driver_step, icmp_frame_len) =
            local_packet_pump_transmit(queue.pump_driver(&mut driver, &mut driver_receive_buffer))?;
        driver
            .inject_received(&local_ping_icmp_echo_reply_frame())
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        local_packet_pump_receive(queue.pump_driver(&mut driver, &mut driver_receive_buffer))?;
        {
            let mut outputs = syscall::ProcessLocalPingDispatchOutputs::new(
                &mut step,
                &mut pump_step,
                &mut status,
            );
            fixture
                .dispatch(
                    fixture.pump_or_read_result_arguments(
                        process_descriptor,
                        syscall::PROCESS_LOCAL_PING_USER_PUMP_RECORD_LEN,
                    ),
                    Some(owner),
                    &mut self.descriptor_store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut queue,
                    &mut outputs,
                )
                .map_err(local_exec_error_from_posix)?;
        }
        let result_step = pump_step.active_ping_step().kind();
        {
            let mut outputs = syscall::ProcessLocalPingDispatchOutputs::new(
                &mut step,
                &mut pump_step,
                &mut status,
            );
            fixture
                .dispatch(
                    fixture.status_arguments(
                        process_descriptor,
                        syscall::PROCESS_LOCAL_PING_USER_STATUS_RECORD_LEN,
                    ),
                    Some(owner),
                    &mut self.descriptor_store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut queue,
                    &mut outputs,
                )
                .map_err(local_exec_error_from_posix)?;
        }
        let status_kind = status.kind();
        let status_payload_len = fixture
            .read_user_u64(STATUS_USER + 24)
            .map_err(local_exec_error_from_posix)? as usize;
        {
            let mut outputs = syscall::ProcessLocalPingDispatchOutputs::new(
                &mut step,
                &mut pump_step,
                &mut status,
            );
            fixture
                .dispatch(
                    fixture.close_arguments(process_descriptor),
                    Some(owner),
                    &mut self.descriptor_store,
                    &mut runtime,
                    &mut receive_buffer,
                    &mut transmit_buffer,
                    &mut queue,
                    &mut outputs,
                )
                .map_err(local_exec_error_from_posix)?;
        }

        Ok(LocalCommandPingdiagRecord {
            process_descriptor,
            destination_ipv4: destination,
            payload_len: payload.len(),
            start_step,
            arp_driver_step,
            arp_frame_len,
            arp_pump_step,
            icmp_driver_step,
            icmp_frame_len,
            result_step,
            status: status_kind,
            status_payload_len,
            closed: true,
            source: "vfs-userspace-diagnostic-svc+process-local-descriptor+packet-queue-pump",
        })
    }

    fn last_process_lifecycle_record(&self) -> Option<LocalCommandProcessLifecycleRecord> {
        self.last_process
    }

    fn wait_process_lifecycle_record(&mut self) -> Option<LocalCommandProcessLifecycleRecord> {
        self.waitable_process.take()
    }

    fn wait_process_lifecycle_record_with_source(
        &mut self,
    ) -> Option<(LocalCommandProcessLifecycleRecord, &'static str)> {
        if let Some(record) = self.waitable_process.take() {
            for explicit in &mut self.explicit_wait_records {
                if explicit
                    .map(|candidate| candidate.process_id == record.process_id)
                    .unwrap_or(false)
                {
                    *explicit = None;
                }
            }
            return Some((record, "lifecycle-record"));
        }
        for slot in &mut self.background_jobs {
            let Some(job) = *slot else {
                continue;
            };
            if job.state == LocalCommandBackgroundJobState::Completed && job.reaped {
                *slot = None;
                return Some((job.lifecycle, "background-job-lifecycle-record"));
            }
        }
        None
    }

    fn wait_process_lifecycle_record_by_pid(
        &mut self,
        process_id: u64,
    ) -> LocalCommandExplicitWaitResult {
        if process_id == 0 {
            return LocalCommandExplicitWaitResult::UnsupportedPid;
        }
        for record in &mut self.explicit_wait_records {
            if let Some(lifecycle) = record {
                if lifecycle.process_id == process_id {
                    let lifecycle = *lifecycle;
                    *record = None;
                    if self
                        .waitable_process
                        .map(|waitable| waitable.process_id == process_id)
                        .unwrap_or(false)
                    {
                        self.waitable_process = None;
                    }
                    return LocalCommandExplicitWaitResult::Record(lifecycle);
                }
            }
        }
        for slot in &mut self.background_jobs {
            let Some(job) = *slot else {
                continue;
            };
            if job.lifecycle.process_id == process_id
                && job.state == LocalCommandBackgroundJobState::Completed
                && job.reaped
            {
                *slot = None;
                return LocalCommandExplicitWaitResult::RecordWithSource(
                    job.lifecycle,
                    "background-job-lifecycle-record",
                );
            }
        }
        LocalCommandExplicitWaitResult::NoChild
    }

    fn exec_vfs_pipeline(
        &mut self,
        request: LocalCommandPipelineRequest,
    ) -> Result<LocalCommandPipelineSummary, LocalCommandExecError> {
        if request.middle.is_some() {
            return self.exec_three_stage_vfs_pipeline(request);
        }

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
            (initramfs::PHASE10_STATUS42_PATH, None, None) => true,
            (
                initramfs::PHASE10_STDIN_PATH,
                Some(LocalCommandExecRedirection::StdinFromEtcBanner),
                None,
            ) => true,
            _ => false,
        };
        let consumer_stdin_redirection_supported = request.producer.path()
            == initramfs::PHASE10_STDIN_PATH
            && request.producer.argv.argc() == 1
            && request.producer.redirection.is_none()
            && request.consumer.path() == initramfs::PHASE10_STDIN_PATH
            && request.consumer.argv.argc() == 1
            && matches!(
                request.consumer.stdin_redirection,
                Some(LocalCommandExecRedirection::StdinFromEtcBanner)
            )
            && request.consumer.redirection.is_none()
            && matches!(
                request.producer.stdin_redirection,
                None | Some(LocalCommandExecRedirection::StdinFromEtcBanner)
            );
        let consumer_output_redirection_supported = request.producer.path()
            == initramfs::PHASE10_STDOUT_PATH
            && request.producer.stdin_redirection.is_none()
            && request.producer.redirection.is_none()
            && request.consumer.path() == initramfs::PHASE10_STDIN_PATH
            && request.consumer.stdin_redirection.is_none()
            && (matches!(
                request.consumer.redirection,
                Some(LocalCommandExecRedirection::StdoutToTmpStdout(path))
                    if !path.is_exact_pipeline_combined_path()
                        && !path.is_exact_pipeline_combined_append_path()
            ) || matches!(
                request.consumer.redirection,
                Some(LocalCommandExecRedirection::StdoutAppendTmpStdout(path))
                    if !path.is_exact_pipeline_combined_append_path()
            ));
        let combined_stdin_stdout_redirection_supported =
            is_direct_pipeline_combined_stdin_stdout_redirection(&request);
        let combined_stdin_stderr_redirection_supported =
            is_direct_pipeline_combined_stdin_stderr_redirection(&request);
        let consumer_stderr_redirection_supported = request.producer.path()
            == initramfs::PHASE10_STDOUT_PATH
            && request.producer.argv.argc() == 1
            && request.producer.stdin_redirection.is_none()
            && request.producer.redirection.is_none()
            && request.consumer.path() == initramfs::PHASE10_STDERR_PATH
            && request.consumer.argv.argc() == 1
            && request.consumer.stdin_redirection.is_none()
            && matches!(
                request.consumer.redirection,
                Some(
                    LocalCommandExecRedirection::StderrToTmpStderr(path)
                        | LocalCommandExecRedirection::StderrAppendTmpStderr(path)
                ) if path.is_exact_pipeline_stderr_path()
            );
        if request.producer.stdin_redirection.is_some()
            && !combined_stdin_stdout_redirection_supported
            && !combined_stdin_stderr_redirection_supported
            && (request.producer.argv.argc() != 1
                || request.consumer.argv.argc() != 1
                || request.consumer.redirection.is_some())
        {
            return Err(LocalCommandExecError::InvalidPath);
        }
        if request.producer.stdin_redirection.is_some()
            && combined_stdin_stderr_redirection_supported
            && !matches!(
                request.consumer.redirection,
                Some(LocalCommandExecRedirection::StderrToTmpStderr(path))
                    if path.is_exact_pipeline_combined_stderr_path()
            )
        {
            return Err(LocalCommandExecError::InvalidPath);
        }
        if !producer_redirection_supported
            && !consumer_stdin_redirection_supported
            && !combined_stdin_stdout_redirection_supported
            && !combined_stdin_stderr_redirection_supported
        {
            return Err(LocalCommandExecError::InvalidPath);
        }
        if !consumer_stdin_redirection_supported
            && !combined_stdin_stdout_redirection_supported
            && !combined_stdin_stderr_redirection_supported
            && (request.consumer.path() != initramfs::PHASE10_STDIN_PATH
                || request.consumer.stdin_redirection.is_some()
                || (request.consumer.redirection.is_some()
                    && !consumer_output_redirection_supported))
            && !consumer_stderr_redirection_supported
        {
            return Err(LocalCommandExecError::InvalidPath);
        }

        self.pipe.reset();
        let stdout_restore = self.install_pipe_endpoint(
            posix::STDOUT_FD,
            posix::DescriptorAccess::WriteOnly,
            LOCAL_COMMAND_PIPE_ENDPOINT_REFERENCE,
        )?;
        self.pipe.open_writer();
        let producer = self.exec_vfs_program(request.producer);
        self.pipe.close_writer();
        self.restore_pipe_endpoint(posix::STDOUT_FD, stdout_restore)?;
        let mut producer = producer?;
        producer.lifecycle = producer
            .lifecycle
            .with_process_id(LOCAL_COMMAND_PIPELINE_PRODUCER_PROCESS_ID);

        let stdin_restore = self.install_pipe_endpoint(
            posix::STDIN_FD,
            posix::DescriptorAccess::ReadOnly,
            LOCAL_COMMAND_PIPE_ENDPOINT_REFERENCE,
        )?;
        self.pipe.open_reader();
        let consumer = self.exec_vfs_program_with_policy(
            request.consumer,
            consumer_output_redirection_supported || combined_stdin_stdout_redirection_supported,
        );
        self.pipe.close_reader();
        self.restore_pipe_endpoint(posix::STDIN_FD, stdin_restore)?;
        let mut consumer = consumer?;
        consumer.lifecycle = consumer
            .lifecycle
            .with_process_id(LOCAL_COMMAND_PIPELINE_CONSUMER_PROCESS_ID);
        self.process_table_records = [None; LOCAL_COMMAND_PROCESS_TABLE_CAPACITY];
        producer.process_table_record = self.record_process_table_record(0, producer.lifecycle);
        consumer.process_table_record = self.record_process_table_record(1, consumer.lifecycle);
        self.last_process = Some(consumer.lifecycle);
        self.waitable_process = Some(consumer.lifecycle);
        self.explicit_wait_records = [Some(producer.lifecycle), Some(consumer.lifecycle), None];
        let pipe_source = match (
            request.producer.redirection,
            request.consumer.redirection,
            request.producer.stdin_redirection,
            request.consumer.stdin_redirection,
        ) {
            (None, Some(LocalCommandExecRedirection::StdoutToTmpStdout(_)), None, None) => {
                "shell-pipe-consumer-stdout-redirection"
            }
            (None, Some(LocalCommandExecRedirection::StdoutAppendTmpStdout(_)), None, None) => {
                "shell-pipe-consumer-stdout-append-redirection"
            }
            (None, Some(LocalCommandExecRedirection::StderrToTmpStderr(_)), None, None) => {
                "shell-pipe-consumer-stderr-redirection"
            }
            (None, Some(LocalCommandExecRedirection::StderrAppendTmpStderr(_)), None, None) => {
                "shell-pipe-consumer-stderr-append-redirection"
            }
            (Some(LocalCommandExecRedirection::StdoutToTmpStdout(_)), None, None, None) => {
                "shell-pipe-producer-file-redirection-away"
            }
            (Some(LocalCommandExecRedirection::StderrToStdout), None, None, None) => {
                "shell-pipe-stderr-dup-to-stdout"
            }
            (Some(LocalCommandExecRedirection::StdoutToStderr), None, None, None) => {
                "shell-pipe-stdout-redirect-away"
            }
            (None, None, None, None) if producer.source_path == initramfs::PHASE10_STDERR_PATH => {
                "shell-pipe-stdout-only-stderr-not-piped"
            }
            (None, None, None, None)
                if producer.source_path == initramfs::PHASE10_STATUS42_PATH =>
            {
                "shell-pipe-status42-to-stdin"
            }
            (None, None, Some(LocalCommandExecRedirection::StdinFromEtcBanner), None) => {
                "shell-pipe-producer-stdin-redirection-to-stdin"
            }
            (
                None,
                Some(LocalCommandExecRedirection::StdoutToTmpStdout(_)),
                Some(LocalCommandExecRedirection::StdinFromEtcBanner),
                None,
            ) => "shell-pipe-producer-stdin-consumer-stdout-redirection",
            (
                None,
                Some(LocalCommandExecRedirection::StdoutAppendTmpStdout(_)),
                Some(LocalCommandExecRedirection::StdinFromEtcBanner),
                None,
            ) => "shell-pipe-producer-stdin-consumer-stdout-append-redirection",
            (
                None,
                Some(LocalCommandExecRedirection::StderrToTmpStderr(_)),
                Some(LocalCommandExecRedirection::StdinFromEtcBanner),
                None,
            ) => "shell-pipe-producer-stdin-consumer-stderr-redirection",
            (
                None,
                None,
                Some(LocalCommandExecRedirection::StdinFromEtcBanner),
                Some(LocalCommandExecRedirection::StdinFromEtcBanner),
            ) => "shell-pipe-dual-stdin-redirection-from-file",
            (None, None, None, Some(LocalCommandExecRedirection::StdinFromEtcBanner)) => {
                "shell-pipe-consumer-stdin-redirection-from-file"
            }
            (None, None, None, None) => "shell-pipe-stdout-to-stdin",
            _ => return Err(LocalCommandExecError::InvalidPath),
        };

        let pipe = LocalCommandPipeRecord {
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
        };

        Ok(LocalCommandPipelineSummary {
            pipe,
            second_pipe: None,
            lifecycle_status: LocalCommandPipelineLifecycleStatusRecord::from_pipeline(
                pipe.id,
                producer.lifecycle,
                consumer.lifecycle,
            ),
            producer,
            middle: None,
            consumer,
        })
    }

    fn exec_three_stage_vfs_pipeline(
        &mut self,
        request: LocalCommandPipelineRequest,
    ) -> Result<LocalCommandPipelineSummary, LocalCommandExecError> {
        let middle = request.middle.ok_or(LocalCommandExecError::InvalidPath)?;
        if request.producer.path() != initramfs::PHASE10_STDOUT_PATH
            || request.producer.stdin_redirection.is_some()
            || request.producer.redirection.is_some()
            || middle.path() != initramfs::PHASE10_STDIN_PATH
            || middle.stdin_redirection.is_some()
            || middle.redirection.is_some()
            || request.consumer.path() != initramfs::PHASE10_STDIN_PATH
            || request.consumer.stdin_redirection.is_some()
            || request.consumer.redirection.is_some()
        {
            return Err(LocalCommandExecError::InvalidPath);
        }

        self.pipe.reset();
        self.second_pipe.reset();

        let producer_stdout_restore = self.install_pipe_endpoint(
            posix::STDOUT_FD,
            posix::DescriptorAccess::WriteOnly,
            LOCAL_COMMAND_PIPE_ENDPOINT_REFERENCE,
        )?;
        self.pipe.open_writer();
        let producer = self.exec_vfs_program(request.producer);
        self.pipe.close_writer();
        self.restore_pipe_endpoint(posix::STDOUT_FD, producer_stdout_restore)?;
        let mut producer = producer?;
        producer.lifecycle = producer
            .lifecycle
            .with_process_id(LOCAL_COMMAND_PIPELINE_PRODUCER_PROCESS_ID);

        let middle_stdin_restore = self.install_pipe_endpoint(
            posix::STDIN_FD,
            posix::DescriptorAccess::ReadOnly,
            LOCAL_COMMAND_PIPE_ENDPOINT_REFERENCE,
        )?;
        let middle_stdout_restore = self.install_pipe_endpoint(
            posix::STDOUT_FD,
            posix::DescriptorAccess::WriteOnly,
            LOCAL_COMMAND_SECOND_PIPE_ENDPOINT_REFERENCE,
        )?;
        self.pipe.open_reader();
        self.second_pipe.open_writer();
        let middle_exec = self.exec_vfs_program(middle);
        self.second_pipe.close_writer();
        self.pipe.close_reader();
        self.restore_pipe_endpoint(posix::STDOUT_FD, middle_stdout_restore)?;
        self.restore_pipe_endpoint(posix::STDIN_FD, middle_stdin_restore)?;
        let mut middle_exec = middle_exec?;
        middle_exec.lifecycle = middle_exec
            .lifecycle
            .with_process_id(LOCAL_COMMAND_PIPELINE_MIDDLE_PROCESS_ID);

        let consumer_stdin_restore = self.install_pipe_endpoint(
            posix::STDIN_FD,
            posix::DescriptorAccess::ReadOnly,
            LOCAL_COMMAND_SECOND_PIPE_ENDPOINT_REFERENCE,
        )?;
        self.second_pipe.open_reader();
        let consumer = self.exec_vfs_program(request.consumer);
        self.second_pipe.close_reader();
        self.restore_pipe_endpoint(posix::STDIN_FD, consumer_stdin_restore)?;
        let mut consumer = consumer?;
        consumer.lifecycle = consumer
            .lifecycle
            .with_process_id(LOCAL_COMMAND_PIPELINE_FINAL_PROCESS_ID);

        self.process_table_records = [None; LOCAL_COMMAND_PROCESS_TABLE_CAPACITY];
        producer.process_table_record = self.record_process_table_record(0, producer.lifecycle);
        middle_exec.process_table_record =
            self.record_process_table_record(1, middle_exec.lifecycle);
        consumer.process_table_record = self.record_process_table_record(2, consumer.lifecycle);
        self.last_process = Some(consumer.lifecycle);
        self.waitable_process = Some(consumer.lifecycle);
        self.explicit_wait_records = [
            Some(producer.lifecycle),
            Some(middle_exec.lifecycle),
            Some(consumer.lifecycle),
        ];

        let first_pipe = LocalCommandPipeRecord {
            id: LOCAL_COMMAND_PIPE_ENDPOINT_REFERENCE,
            producer_fd: posix::STDOUT_FD,
            producer_path: producer.source_path,
            consumer_fd: posix::STDIN_FD,
            consumer_path: middle_exec.source_path,
            bytes_written: self.pipe.len,
            bytes_read: self.pipe.cursor,
            writer_closed: !self.pipe.writer_open,
            reader_eof: self.pipe.eof_observed,
            shell_restored: self.shell_standard_descriptors_restored()?,
            source: "shell-pipe-multistage-first-stdout-to-stdin",
        };
        let second_pipe = LocalCommandPipeRecord {
            id: LOCAL_COMMAND_SECOND_PIPE_ENDPOINT_REFERENCE,
            producer_fd: posix::STDOUT_FD,
            producer_path: middle_exec.source_path,
            consumer_fd: posix::STDIN_FD,
            consumer_path: consumer.source_path,
            bytes_written: self.second_pipe.len,
            bytes_read: self.second_pipe.cursor,
            writer_closed: !self.second_pipe.writer_open,
            reader_eof: self.second_pipe.eof_observed,
            shell_restored: self.shell_standard_descriptors_restored()?,
            source: "shell-pipe-multistage-middle-to-stdin",
        };

        Ok(LocalCommandPipelineSummary {
            pipe: first_pipe,
            second_pipe: Some(second_pipe),
            lifecycle_status: LocalCommandPipelineLifecycleStatusRecord::from_three_stage_pipeline(
                first_pipe.id,
                producer.lifecycle,
                middle_exec.lifecycle,
                consumer.lifecycle,
            ),
            producer,
            middle: Some(middle_exec),
            consumer,
        })
    }

    fn exec_background_vfs_program(
        &mut self,
        request: LocalCommandExecRequest,
    ) -> Result<LocalCommandBackgroundExecSummary, LocalCommandExecError> {
        if !matches!(
            request.path(),
            initramfs::PHASE10_STATUS42_PATH | initramfs::PHASE10_ZERO_PATH
        ) || request.redirection.is_some()
            || request.stdin_redirection.is_some()
            || self
                .background_jobs
                .iter()
                .flatten()
                .any(|job| job.state == LocalCommandBackgroundJobState::Running && !job.reaped)
        {
            return Err(LocalCommandExecError::InvalidPath);
        }
        let slot = self
            .background_jobs
            .iter()
            .position(Option::is_none)
            .ok_or(LocalCommandExecError::InvalidPath)?;

        let previous_last = self.last_process;
        let previous_waitable = self.waitable_process;
        let previous_explicit = self.explicit_wait_records;
        let previous_process_table = self.process_table_records;
        let mut exec = self.exec_vfs_program(request)?;
        self.last_process = previous_last;
        self.waitable_process = previous_waitable;
        self.explicit_wait_records = previous_explicit;
        self.process_table_records = previous_process_table;
        let job_id = self.next_background_job_id;
        self.next_background_job_id = self
            .next_background_job_id
            .checked_add(1)
            .ok_or(LocalCommandExecError::LaunchPipelineFailed)?;
        let process_id = LOCAL_COMMAND_EXEC_PROCESS_ID
            .checked_add(job_id.saturating_sub(LOCAL_COMMAND_BACKGROUND_JOB_FIRST_ID))
            .ok_or(LocalCommandExecError::LaunchPipelineFailed)?;
        let job = LocalCommandBackgroundJobRecord::running(
            job_id,
            exec.lifecycle.with_process_id(process_id),
            exec.source_path,
        );
        exec.process_table_record = self.record_process_table_record(slot, job.lifecycle);
        self.background_jobs[slot] = Some(job);
        Ok(LocalCommandBackgroundExecSummary { exec, job })
    }

    fn poll_background_job_completion(&mut self) -> Option<LocalCommandBackgroundJobRecord> {
        for slot in &mut self.background_jobs {
            let Some(job) = *slot else {
                continue;
            };
            if job.state != LocalCommandBackgroundJobState::Running || job.reaped {
                continue;
            }
            let completed = job.completed_reaped();
            *slot = Some(completed);
            return Some(completed);
        }
        None
    }

    fn clear_completed_background_job_records(&mut self) -> usize {
        let mut cleared = 0;
        for slot in &mut self.background_jobs {
            let Some(job) = *slot else {
                continue;
            };
            if job.state == LocalCommandBackgroundJobState::Completed && job.reaped {
                *slot = None;
                cleared += 1;
            }
        }
        cleared
    }

    fn background_job_records(
        &self,
    ) -> [Option<LocalCommandBackgroundJobRecord>; LOCAL_COMMAND_BACKGROUND_JOB_CAPACITY] {
        self.background_jobs
    }

    fn process_table_records(
        &self,
    ) -> [Option<LocalCommandProcessTableRecord>; LOCAL_COMMAND_PROCESS_TABLE_CAPACITY] {
        self.process_table_records
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

    fn read_process_status_file_via_descriptor(
        &mut self,
        output: &mut [u8],
    ) -> Result<usize, LocalCommandFileReadError> {
        let file = self
            .process_status_file_bytes()
            .map_err(|_| LocalCommandFileReadError::SyscallFailed)?;
        let table = self
            .descriptor_store
            .current_descriptor_table_mut(self.current_owner)
            .map_err(|_| LocalCommandFileReadError::SyscallFailed)?;
        let entry = posix::DescriptorEntry::new(
            posix::DescriptorAccess::ReadOnly,
            posix::DescriptorFlags::EMPTY,
            posix::DescriptorObject::new(
                posix::DescriptorObjectKind::OtherKernelObject,
                LOCAL_COMMAND_PROCESS_STATUS_FILE_REFERENCE,
            ),
        );
        let descriptor = table
            .allocate(entry)
            .map_err(|_| LocalCommandFileReadError::SyscallFailed)?;
        let bytes = self.read_process_status_descriptor(descriptor, file.as_bytes(), output);
        let cleanup = self.close_regular_file_descriptor(descriptor);
        match (bytes, cleanup) {
            (Ok(bytes), Ok(())) => Ok(bytes),
            _ => Err(LocalCommandFileReadError::SyscallFailed),
        }
    }

    fn read_process_status_descriptor(
        &mut self,
        descriptor: usize,
        file_bytes: &[u8],
        output: &mut [u8],
    ) -> Result<usize, LocalCommandFileReadError> {
        let table = self
            .descriptor_store
            .current_descriptor_table(self.current_owner)
            .map_err(|_| LocalCommandFileReadError::SyscallFailed)?;
        let entry = table
            .get(descriptor)
            .map_err(|_| LocalCommandFileReadError::SyscallFailed)?;
        if entry.require_readable().is_err()
            || entry.object().kind() != posix::DescriptorObjectKind::OtherKernelObject
            || entry.object().reference() != LOCAL_COMMAND_PROCESS_STATUS_FILE_REFERENCE
        {
            return Err(LocalCommandFileReadError::SyscallFailed);
        }
        let selected = core::cmp::min(output.len(), file_bytes.len());
        output[..selected].copy_from_slice(&file_bytes[..selected]);
        Ok(selected)
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
        reference: usize,
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
            posix::DescriptorObject::new(posix::DescriptorObjectKind::PipeEndpoint, reference),
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
            let reference = entry.object().reference();
            let remaining = match self.pipe_remaining(reference) {
                Ok(remaining) => remaining,
                Err(_) => return (syscall::EBADF as u64).wrapping_neg(),
            };
            let read_len =
                core::cmp::min(core::cmp::max(remaining, 1), user_memory.len() - read_start);
            let bytes_read = match self.pipe_state_mut(reference) {
                Ok(pipe) => pipe.read(&mut user_memory[read_start..read_start + read_len]),
                Err(_) => return (syscall::EBADF as u64).wrapping_neg(),
            };
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

    fn stdin_pipe_route(&self) -> Result<&'static str, LocalCommandExecError> {
        let table = self
            .descriptor_store
            .current_descriptor_table(self.current_owner)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        let entry = table
            .get(posix::STDIN_FD)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        if entry.object().kind() != posix::DescriptorObjectKind::PipeEndpoint {
            return Err(LocalCommandExecError::LaunchPipelineFailed);
        }
        Self::pipe_route(entry.object().reference())
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

    fn command_output_descriptor_is_pipe(&self) -> Result<bool, LocalCommandExecError> {
        let table = self
            .descriptor_store
            .current_descriptor_table(self.current_owner)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        let entry = table
            .get(self.output_descriptor)
            .map_err(|_| LocalCommandExecError::LaunchPipelineFailed)?;
        Ok(entry.object().kind() == posix::DescriptorObjectKind::PipeEndpoint)
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
        if self.command_output_descriptor_is_pipe()? {
            return Ok(());
        }
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
        if read_return_value != (syscall::EAGAIN as u64).wrapping_neg()
            && read_return_value != 0
            && stdin_is_pipe
        {
            expected_read_bytes = read_return_value as usize;
            read_source = self.stdin_pipe_route()?;
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
                let stdin_prefix = initramfs::PHASE10_STDIN_STDOUT_PREFIX;
                let banner = initramfs::PHASE8_BANNER_BYTES;
                let pipe_from_stdin_banner = read_input.len()
                    == stdin_prefix.len() + banner.len() + 1
                    && &read_input[..stdin_prefix.len()] == stdin_prefix
                    && &read_input[stdin_prefix.len()..stdin_prefix.len() + banner.len()] == banner
                    && read_input[read_input.len() - 1] == b'\n';
                if read_input != initramfs::PHASE10_STDOUT_PAYLOAD
                    && read_input != initramfs::PHASE10_STDERR_PAYLOAD
                    && !pipe_from_stdin_banner
                {
                    return Err(LocalCommandExecError::SyscallFailed);
                }
            } else if read_source == "pipe:middle-to-stdin" {
                let prefix = initramfs::PHASE10_STDIN_STDOUT_PREFIX;
                let payload = initramfs::PHASE10_STDOUT_PAYLOAD;
                if read_input.len() != prefix.len() + payload.len() + 1
                    || &read_input[..prefix.len()] != prefix
                    || &read_input[prefix.len()..prefix.len() + payload.len()] != payload
                    || read_input[read_input.len() - 1] != b'\n'
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
            let observed_stdout_bytes = prefix_write
                .checked_add(read_write)
                .and_then(|len| len.checked_add(newline_write))
                .ok_or(LocalCommandExecError::SyscallFailed)?;
            if observed_stdout_bytes != read_stdout_bytes as u64 {
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
            if read_source == "pipe:stdout-to-stdin" || read_source == "pipe:middle-to-stdin" {
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
            read_source = self.stdin_pipe_route()?;
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
                LocalCommandFieldText::from_static(Self::pipe_route(entry.object().reference())?),
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
            let reference = entry.object().reference();
            return match self
                .pipe_state_mut(reference)
                .and_then(|pipe| pipe.write(&user_memory[..len]))
            {
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

fn local_exec_error_from_posix(error: posix::PosixError) -> LocalCommandExecError {
    match error {
        posix::PosixError::NoEntry => LocalCommandExecError::NotFound,
        posix::PosixError::NotExecutable => LocalCommandExecError::NotExecutable,
        posix::PosixError::InvalidArgument
        | posix::PosixError::NameTooLong
        | posix::PosixError::NoSpace => LocalCommandExecError::InvalidPath,
        posix::PosixError::NotSupported => LocalCommandExecError::NotSupported,
        _ => LocalCommandExecError::SyscallFailed,
    }
}

fn local_sockdiag_poll_wait_task(offset: u64) -> Result<scheduler::Task, LocalCommandExecError> {
    let raw_task_id = LOCAL_COMMAND_SOCKDIAG_POLL_WAIT_TASK_BASE
        .checked_add(offset)
        .ok_or(LocalCommandExecError::LaunchPipelineFailed)?;
    let task_id =
        scheduler::TaskId::new(raw_task_id).ok_or(LocalCommandExecError::LaunchPipelineFailed)?;
    let kernel_stack = scheduler::KernelStack::new(0x8000 + offset as usize * 0x1000, 0x1000)
        .ok_or(LocalCommandExecError::LaunchPipelineFailed)?;
    Ok(scheduler::Task::kernel_thread(
        task_id,
        kernel_stack,
        scheduler::ContextFrame::new(0x9000 + offset as usize * 0x1000, 0x1000),
    ))
}

const fn local_task_state_name(state: TaskState) -> &'static str {
    match state {
        TaskState::Running => "running",
        TaskState::Runnable => "runnable",
        TaskState::Blocked => "blocked",
    }
}

const fn local_smoltcp_tcp_state_name(state: smoltcp::socket::tcp::State) -> &'static str {
    match state {
        smoltcp::socket::tcp::State::Established => "established",
        smoltcp::socket::tcp::State::Closed => "closed",
        smoltcp::socket::tcp::State::Listen => "listen",
        smoltcp::socket::tcp::State::SynSent => "syn-sent",
        smoltcp::socket::tcp::State::SynReceived => "syn-received",
        smoltcp::socket::tcp::State::FinWait1 => "fin-wait-1",
        smoltcp::socket::tcp::State::FinWait2 => "fin-wait-2",
        smoltcp::socket::tcp::State::CloseWait => "close-wait",
        smoltcp::socket::tcp::State::Closing => "closing",
        smoltcp::socket::tcp::State::LastAck => "last-ack",
        smoltcp::socket::tcp::State::TimeWait => "time-wait",
    }
}

const fn local_driver_packet_rx_step_name(
    step: crate::network::DriverPacketAdapterReceiveStep,
) -> &'static str {
    match step {
        crate::network::DriverPacketAdapterReceiveStep::NoFrame => "no-frame",
        crate::network::DriverPacketAdapterReceiveStep::Received { .. } => "received",
        crate::network::DriverPacketAdapterReceiveStep::TransmitQueueFull => "tx-queue-full",
        crate::network::DriverPacketAdapterReceiveStep::ReceiveBufferTooSmall => {
            "rx-buffer-too-small"
        }
        crate::network::DriverPacketAdapterReceiveStep::ReceiveError(_) => "rx-error",
    }
}

const fn local_driver_packet_rx_frame_len(
    step: crate::network::DriverPacketAdapterReceiveStep,
) -> usize {
    match step {
        crate::network::DriverPacketAdapterReceiveStep::Received { frame_len } => frame_len,
        _ => 0,
    }
}

const fn local_driver_packet_tx_step_name(
    step: crate::network::DriverPacketAdapterTransmitStep,
) -> &'static str {
    match step {
        crate::network::DriverPacketAdapterTransmitStep::Transmitted { .. } => "transmitted",
        crate::network::DriverPacketAdapterTransmitStep::TransmitQueueFull => "tx-queue-full",
        crate::network::DriverPacketAdapterTransmitStep::FrameTooLarge { .. } => "frame-too-large",
        crate::network::DriverPacketAdapterTransmitStep::TransmitError(_) => "tx-error",
    }
}

const fn local_driver_packet_tx_frame_len(
    step: crate::network::DriverPacketAdapterTransmitStep,
) -> usize {
    match step {
        crate::network::DriverPacketAdapterTransmitStep::Transmitted { frame_len } => frame_len,
        _ => 0,
    }
}

fn local_packet_pump_transmit(
    step: crate::network::PacketQueueDriverPumpStep,
) -> Result<(&'static str, usize), LocalCommandExecError> {
    match step {
        crate::network::PacketQueueDriverPumpStep::Transmitted { frame_len } => {
            Ok(("transmitted", frame_len))
        }
        _ => Err(LocalCommandExecError::LaunchPipelineFailed),
    }
}

fn local_packet_pump_receive(
    step: crate::network::PacketQueueDriverPumpStep,
) -> Result<(), LocalCommandExecError> {
    match step {
        crate::network::PacketQueueDriverPumpStep::Received { .. } => Ok(()),
        _ => Err(LocalCommandExecError::LaunchPipelineFailed),
    }
}

const fn local_ping_endpoint() -> crate::network::LocalNetworkEndpoint {
    crate::network::LocalNetworkEndpoint::new(
        crate::network::MacAddress::new([0x02, 0, 0, 0, 0, 99]),
        [192, 0, 2, 1],
    )
}

fn local_ping_arp_reply_frame()
-> [u8; crate::network::ETHERNET_HEADER_LEN + crate::network::ARP_ETHERNET_IPV4_LEN] {
    let mut frame =
        [0u8; crate::network::ETHERNET_HEADER_LEN + crate::network::ARP_ETHERNET_IPV4_LEN];
    frame[..6].copy_from_slice(&[0x02, 0, 0, 0, 0, 1]);
    frame[6..12].copy_from_slice(&[0x02, 0, 0, 0, 0, 20]);
    local_write_be_u16(&mut frame, 12, crate::network::ETHERTYPE_ARP);

    let arp = &mut frame[crate::network::ETHERNET_HEADER_LEN..];
    local_write_be_u16(arp, 0, 1);
    local_write_be_u16(arp, 2, crate::network::ETHERTYPE_IPV4);
    arp[4] = crate::network::ETHERNET_ADDR_LEN as u8;
    arp[5] = 4;
    local_write_be_u16(arp, 6, 2);
    arp[8..14].copy_from_slice(&[0x02, 0, 0, 0, 0, 20]);
    arp[14..18].copy_from_slice(&[192, 0, 2, 20]);
    arp[18..24].copy_from_slice(&[0x02, 0, 0, 0, 0, 1]);
    arp[24..28].copy_from_slice(&[192, 0, 2, 10]);
    frame
}

fn local_ping_icmp_echo_reply_frame()
-> [u8; crate::network::ETHERNET_HEADER_LEN + crate::network::IPV4_MIN_HEADER_LEN + 12] {
    let mut frame =
        [0u8; crate::network::ETHERNET_HEADER_LEN + crate::network::IPV4_MIN_HEADER_LEN + 12];
    frame[..6].copy_from_slice(&[0x02, 0, 0, 0, 0, 99]);
    frame[6..12].copy_from_slice(&[0x02, 0, 0, 0, 0, 20]);
    local_write_be_u16(&mut frame, 12, crate::network::ETHERTYPE_IPV4);

    let ipv4 = &mut frame[crate::network::ETHERNET_HEADER_LEN
        ..crate::network::ETHERNET_HEADER_LEN + crate::network::IPV4_MIN_HEADER_LEN];
    ipv4[0] = 0x45;
    local_write_be_u16(ipv4, 2, (crate::network::IPV4_MIN_HEADER_LEN + 12) as u16);
    local_write_be_u16(ipv4, 4, 0x4444);
    ipv4[8] = 64;
    ipv4[9] = crate::network::IPV4_PROTOCOL_ICMP;
    ipv4[12..16].copy_from_slice(&[192, 0, 2, 20]);
    ipv4[16..20].copy_from_slice(&[192, 0, 2, 1]);
    let checksum = local_internet_checksum(ipv4);
    local_write_be_u16(ipv4, 10, checksum);

    let icmp =
        &mut frame[crate::network::ETHERNET_HEADER_LEN + crate::network::IPV4_MIN_HEADER_LEN..];
    icmp[0] = 0;
    icmp[4..].copy_from_slice(&[0x12, 0x34, 0, 7, 1, 2, 3, 4]);
    let checksum = local_internet_checksum(icmp);
    local_write_be_u16(icmp, 2, checksum);
    frame
}

fn local_write_be_u16(bytes: &mut [u8], offset: usize, value: u16) {
    let raw = value.to_be_bytes();
    bytes[offset] = raw[0];
    bytes[offset + 1] = raw[1];
}

fn local_internet_checksum(bytes: &[u8]) -> u16 {
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

fn syscall_success_usize(value: u64) -> Result<usize, LocalCommandFileReadError> {
    if value > isize::MAX as u64 {
        return Err(LocalCommandFileReadError::SyscallFailed);
    }
    usize::try_from(value).map_err(|_| LocalCommandFileReadError::SyscallFailed)
}

fn local_write_poll_entry(memory: &mut [u8], index: usize, fd: u64, events: u32) {
    let offset = index * userspace_socket_abi::POLL_ENTRY_SIZE;
    match userspace_socket_abi::PollEntry::new(fd, events)
        .encode(&mut memory[offset..offset + userspace_socket_abi::POLL_ENTRY_SIZE])
    {
        Ok(()) => {}
        Err(userspace_socket_abi::PollEntryEncodeError::BufferTooSmall) => unreachable!(),
    }
}

fn local_read_poll_revents(memory: &[u8], index: usize) -> u32 {
    let offset = index * userspace_socket_abi::POLL_ENTRY_SIZE;
    match userspace_socket_abi::PollEntry::decode(
        &memory[offset..offset + userspace_socket_abi::POLL_ENTRY_SIZE],
    ) {
        Ok(entry) => entry.revents(),
        Err(userspace_socket_abi::PollEntryEncodeError::BufferTooSmall) => unreachable!(),
    }
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

    pub fn is_no_data_timeout(&self) -> bool {
        self.line_len == 0
            && self.raw_bytes == 0
            && self.status == LocalCommandStatus::InputError(PollingTtyRxOutcome::Timeout)
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

    if command.name != "jobs" {
        while let Some(job) = sink.poll_background_job_completion() {
            write_background_job_completion_line(sink, responses, job)?;
        }
    }

    if command.name.starts_with('/') {
        return dispatch_absolute_path_command(command, sink, responses);
    }

    if is_bounded_bare_bin_command_name(command.name) {
        return dispatch_bare_bin_command(command, sink, responses);
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
                "talos: commands help status stdio pwd echo ls cat cd exec laststatus waitpid jobs ps pipestatus",
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
                "talos: commands help status stdio pwd echo ls cat cd exec laststatus waitpid jobs ps pipestatus",
            )?;
            Ok(LocalCommandStatus::Handled)
        }
        "jobs" => {
            if command.arguments.is_some() {
                write_line(sink, responses, "talos: unexpected-argument")?;
                return Ok(LocalCommandStatus::UnexpectedArgument);
            }
            let mut found_job = false;
            for job in sink.background_job_records().into_iter().flatten() {
                found_job = true;
                write_jobs_accounting_line(sink, responses, job)?;
            }
            if found_job {
                let _ = sink.clear_completed_background_job_records();
                let _ = sink.poll_background_job_completion();
            } else {
                write_line(
                    sink,
                    responses,
                    "talos: jobs none source=background-vfs-exec-accounting",
                )?;
            }
            Ok(LocalCommandStatus::Handled)
        }
        "pipestatus" => {
            if command.arguments.is_some() {
                write_line(sink, responses, "talos: unexpected-argument")?;
                return Ok(LocalCommandStatus::UnexpectedArgument);
            }
            write_pipeline_status_observation(sink, responses, sink.process_table_records())?;
            Ok(LocalCommandStatus::Handled)
        }
        "rootinfo" => {
            if command.arguments.is_some() {
                write_line(sink, responses, "talos: unexpected-argument")?;
                return Ok(LocalCommandStatus::UnexpectedArgument);
            }
            write_generated_root_selection_line(sink, responses)?;
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
                Some("/generated/manifest.txt") => {
                    write_initramfs_text_file(sink, responses, initramfs::GENERATED_ROOT_FILE_PATH)?
                }
                Some("/proc/talos/processes") => write_initramfs_text_file(
                    sink,
                    responses,
                    LOCAL_COMMAND_PROC_TALOS_PROCESSES_PATH,
                )?,
                Some(path) if path.starts_with("/proc") => {
                    write_line(sink, responses, "talos: not-found")?;
                    return Ok(LocalCommandStatus::UnexpectedArgument);
                }
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
        "ps" => {
            if command.arguments.is_some() {
                write_line(sink, responses, "talos: unexpected-argument")?;
                return Ok(LocalCommandStatus::UnexpectedArgument);
            }
            write_initramfs_text_file(sink, responses, LOCAL_COMMAND_PROC_TALOS_PROCESSES_PATH)?;
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
            if let Some(arguments) = command.arguments {
                let Some(process_id) = parse_waitpid_process_id(arguments) else {
                    write_line(
                        sink,
                        responses,
                        "talos: waitpid invalid-pid source=explicit-pid-lifecycle-record",
                    )?;
                    return Ok(LocalCommandStatus::UnexpectedArgument);
                };
                match sink.wait_process_lifecycle_record_by_pid(process_id) {
                    LocalCommandExplicitWaitResult::Record(record) => {
                        write_waitpid_status_line_with_source(
                            sink,
                            responses,
                            record,
                            "explicit-pid-lifecycle-record",
                        )?;
                        return Ok(LocalCommandStatus::Handled);
                    }
                    LocalCommandExplicitWaitResult::RecordWithSource(record, source) => {
                        write_waitpid_status_line_with_source(sink, responses, record, source)?;
                        return Ok(LocalCommandStatus::Handled);
                    }
                    LocalCommandExplicitWaitResult::NoChild => {
                        write_waitpid_no_child_pid_line(sink, responses, process_id)?;
                        return Ok(LocalCommandStatus::Handled);
                    }
                    LocalCommandExplicitWaitResult::UnsupportedPid => {
                        write_waitpid_unsupported_pid_line(sink, responses, process_id)?;
                        return Ok(LocalCommandStatus::UnexpectedArgument);
                    }
                }
            }
            match sink.wait_process_lifecycle_record_with_source() {
                Some((record, source)) => {
                    write_waitpid_status_line_with_source(sink, responses, record, source)?;
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

fn dispatch_bare_bin_command(
    command: ParsedLocalCommand<'_>,
    sink: &mut impl LocalCommandSink,
    responses: &mut usize,
) -> Result<LocalCommandStatus, LocalCommandCycleError> {
    if command
        .arguments
        .map(|arguments| arguments.as_bytes().contains(&b'|'))
        .unwrap_or(false)
    {
        return match parse_bare_bin_pipeline_request(command)
            .and_then(|request| sink.exec_vfs_pipeline(request))
        {
            Ok(summary) => {
                write_pipeline_summary(sink, responses, summary)?;
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
            Err(LocalCommandExecError::SyscallFailed) => {
                write_line(sink, responses, "talos: exec-syscall-failed")?;
                Ok(LocalCommandStatus::UnexpectedArgument)
            }
            Err(LocalCommandExecError::LaunchPipelineFailed) => {
                write_line(sink, responses, "talos: exec-launch-failed")?;
                Ok(LocalCommandStatus::UnexpectedArgument)
            }
            Err(_) => {
                write_line(sink, responses, "talos: exec-error")?;
                Ok(LocalCommandStatus::UnexpectedArgument)
            }
        };
    }

    match parse_bare_bin_command_request(command).and_then(|request| sink.exec_vfs_program(request))
    {
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

fn dispatch_absolute_path_command(
    command: ParsedLocalCommand<'_>,
    sink: &mut impl LocalCommandSink,
    responses: &mut usize,
) -> Result<LocalCommandStatus, LocalCommandCycleError> {
    if command
        .arguments
        .map(|arguments| arguments.as_bytes().contains(&b'|'))
        .unwrap_or(false)
    {
        return match parse_absolute_path_pipeline_request(command)
            .and_then(|request| sink.exec_vfs_pipeline(request))
        {
            Ok(summary) => {
                write_pipeline_summary(sink, responses, summary)?;
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
            Err(LocalCommandExecError::SyscallFailed) => {
                write_line(sink, responses, "talos: exec-syscall-failed")?;
                Ok(LocalCommandStatus::UnexpectedArgument)
            }
            Err(LocalCommandExecError::LaunchPipelineFailed) => {
                write_line(sink, responses, "talos: exec-launch-failed")?;
                Ok(LocalCommandStatus::UnexpectedArgument)
            }
            Err(_) => {
                write_line(sink, responses, "talos: exec-error")?;
                Ok(LocalCommandStatus::UnexpectedArgument)
            }
        };
    }

    match parse_absolute_path_command_request(command)
        .and_then(|request| sink.exec_vfs_program(request))
    {
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

fn parse_waitpid_process_id(arguments: &str) -> Option<u64> {
    if arguments.as_bytes().contains(&b' ') {
        return None;
    }
    if arguments.is_empty() {
        return None;
    }
    let digits = arguments.strip_prefix("0x").unwrap_or(arguments);
    if digits.is_empty() {
        return None;
    }
    let mut value = 0u64;
    for byte in digits.as_bytes() {
        let digit = match *byte {
            b'0'..=b'9' => byte - b'0',
            b'a'..=b'f' => byte - b'a' + 10,
            b'A'..=b'F' => byte - b'A' + 10,
            _ => return None,
        };
        value = value.checked_mul(16)?;
        value = value.checked_add(digit as u64)?;
    }
    Some(value)
}

fn parse_pipeline_request(
    arguments: &str,
) -> Result<LocalCommandPipelineRequest, LocalCommandExecError> {
    let bytes = arguments.as_bytes();
    let mut pipes = [None; 2];
    let mut pipe_count = 0usize;
    let mut index = 0usize;
    while index < bytes.len() {
        if bytes[index] == b'|' {
            if pipe_count >= pipes.len() {
                return Err(LocalCommandExecError::InvalidPath);
            }
            pipes[pipe_count] = Some(index);
            pipe_count += 1;
        }
        index += 1;
    }
    let first_pipe = pipes[0].ok_or(LocalCommandExecError::InvalidPath)?;
    let producer = trim_ascii_space(&arguments[..first_pipe]);
    if producer.is_empty() {
        return Err(LocalCommandExecError::InvalidPath);
    }
    let second_pipe = pipes[1];
    let middle_or_consumer_end = second_pipe.unwrap_or(arguments.len());
    let middle_or_consumer = trim_ascii_space(&arguments[first_pipe + 1..middle_or_consumer_end]);
    let middle_or_consumer = middle_or_consumer
        .strip_prefix("exec ")
        .ok_or(LocalCommandExecError::InvalidPath)?;
    if middle_or_consumer.is_empty() {
        return Err(LocalCommandExecError::InvalidPath);
    }
    let (middle, consumer) = if let Some(second_pipe) = second_pipe {
        let consumer = trim_ascii_space(&arguments[second_pipe + 1..]);
        let consumer = consumer
            .strip_prefix("exec ")
            .ok_or(LocalCommandExecError::InvalidPath)?;
        if consumer.is_empty() {
            return Err(LocalCommandExecError::InvalidPath);
        }
        (
            Some(parse_exec_request(middle_or_consumer)?),
            parse_exec_request(consumer)?,
        )
    } else {
        (None, parse_exec_request(middle_or_consumer)?)
    };
    let request = LocalCommandPipelineRequest {
        producer: parse_exec_request(producer)?,
        middle,
        consumer,
    };
    if matches!(
        request.consumer.redirection,
        Some(LocalCommandExecRedirection::StdoutAppendTmpStdout(_))
    ) {
        return Err(LocalCommandExecError::InvalidPath);
    }
    Ok(request)
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

fn parse_absolute_path_command_request(
    command: ParsedLocalCommand<'_>,
) -> Result<LocalCommandExecRequest, LocalCommandExecError> {
    parse_absolute_path_exec_request_with_arguments(command.name, command.arguments)
}

fn parse_bare_bin_command_request(
    command: ParsedLocalCommand<'_>,
) -> Result<LocalCommandExecRequest, LocalCommandExecError> {
    parse_bare_bin_exec_request_with_arguments(command.name, command.arguments)
}

fn parse_bare_bin_pipeline_request(
    command: ParsedLocalCommand<'_>,
) -> Result<LocalCommandPipelineRequest, LocalCommandExecError> {
    let arguments = command
        .arguments
        .ok_or(LocalCommandExecError::InvalidPath)?;
    let arguments = trim_ascii_space(arguments);
    let pipe = arguments
        .as_bytes()
        .iter()
        .position(|byte| *byte == b'|')
        .ok_or(LocalCommandExecError::InvalidPath)?;
    let producer_arguments = trim_ascii_space(&arguments[..pipe]);
    let consumer = trim_ascii_space(&arguments[pipe + 1..]);
    if consumer.is_empty() || consumer.as_bytes().contains(&b'|') {
        return Err(LocalCommandExecError::InvalidPath);
    }
    let (consumer_name, consumer_arguments) = split_pipeline_stage_path_and_arguments(consumer)?;
    reject_unbounded_pipeline_producer_arguments(producer_arguments)?;
    reject_unbounded_pipeline_consumer_arguments(consumer_arguments.unwrap_or(""))?;
    Ok(LocalCommandPipelineRequest {
        producer: parse_bare_bin_exec_request_with_arguments(
            command.name,
            optional_trimmed_arguments(producer_arguments),
        )?,
        middle: None,
        consumer: parse_bare_bin_pipeline_consumer_request(consumer_name, consumer_arguments)?,
    })
}

fn parse_bare_bin_pipeline_consumer_request(
    name: &str,
    arguments: Option<&str>,
) -> Result<LocalCommandExecRequest, LocalCommandExecError> {
    let Some(arguments) = arguments else {
        return parse_bare_bin_exec_request_with_arguments(name, None);
    };
    let arguments = trim_ascii_space(arguments);
    if name.as_bytes() == b"stdin" {
        if let Some(target) = arguments.strip_prefix(">>") {
            if let Some(path) =
                LocalCommandVolatilePath::from_exact_pipeline_report_path(target.as_bytes())
                    .or_else(|| {
                        LocalCommandVolatilePath::from_exact_pipeline_combined_append_path(
                            target.as_bytes(),
                        )
                    })
            {
                let exec_path = LocalCommandExecPath::from_fixed_bin_name(name.as_bytes())?;
                let argv = LocalCommandLiteralArgv::from_tokens(&[name.as_bytes()])?
                    .with_resolved_argv0(exec_path.as_bytes())?;
                return Ok(LocalCommandExecRequest {
                    path: exec_path,
                    argv,
                    redirection: Some(LocalCommandExecRedirection::StdoutAppendTmpStdout(path)),
                    stdin_redirection: None,
                });
            }
        }
        if let Some(target) = arguments.strip_prefix('>') {
            if let Some(path) =
                LocalCommandVolatilePath::from_exact_pipeline_report_path(target.as_bytes())
                    .or_else(|| {
                        LocalCommandVolatilePath::from_exact_pipeline_combined_path(
                            target.as_bytes(),
                        )
                    })
                    .or_else(|| {
                        LocalCommandVolatilePath::from_exact_pipeline_combined_append_path(
                            target.as_bytes(),
                        )
                    })
            {
                let exec_path = LocalCommandExecPath::from_fixed_bin_name(name.as_bytes())?;
                let argv = LocalCommandLiteralArgv::from_tokens(&[name.as_bytes()])?
                    .with_resolved_argv0(exec_path.as_bytes())?;
                return Ok(LocalCommandExecRequest {
                    path: exec_path,
                    argv,
                    redirection: Some(LocalCommandExecRedirection::StdoutToTmpStdout(path)),
                    stdin_redirection: None,
                });
            }
        }
    }
    if name.as_bytes() == b"stderr" {
        if let Some(target) = arguments.strip_prefix("2>>") {
            if let Some(path) =
                LocalCommandVolatilePath::from_exact_pipeline_stderr_path(target.as_bytes())
            {
                let exec_path = LocalCommandExecPath::from_fixed_bin_name(name.as_bytes())?;
                let argv = LocalCommandLiteralArgv::from_tokens(&[name.as_bytes()])?
                    .with_resolved_argv0(exec_path.as_bytes())?;
                return Ok(LocalCommandExecRequest {
                    path: exec_path,
                    argv,
                    redirection: Some(LocalCommandExecRedirection::StderrAppendTmpStderr(path)),
                    stdin_redirection: None,
                });
            }
        }
        if let Some(target) = arguments.strip_prefix("2>") {
            if let Some(path) =
                LocalCommandVolatilePath::from_exact_pipeline_stderr_path(target.as_bytes())
            {
                let exec_path = LocalCommandExecPath::from_fixed_bin_name(name.as_bytes())?;
                let argv = LocalCommandLiteralArgv::from_tokens(&[name.as_bytes()])?
                    .with_resolved_argv0(exec_path.as_bytes())?;
                return Ok(LocalCommandExecRequest {
                    path: exec_path,
                    argv,
                    redirection: Some(LocalCommandExecRedirection::StderrToTmpStderr(path)),
                    stdin_redirection: None,
                });
            }
        }
    }
    parse_bare_bin_exec_request_with_arguments(name, Some(arguments))
}

fn parse_bare_bin_exec_request_with_arguments(
    name: &str,
    arguments: Option<&str>,
) -> Result<LocalCommandExecRequest, LocalCommandExecError> {
    if !is_bounded_bare_bin_command_name(name) {
        return Err(LocalCommandExecError::InvalidPath);
    }
    let path = LocalCommandExecPath::from_fixed_bin_name(name.as_bytes())?;
    let mut tokens: [&[u8]; LOCAL_COMMAND_LITERAL_ARGV_CAPACITY] =
        [&[]; LOCAL_COMMAND_LITERAL_ARGV_CAPACITY];
    tokens[0] = name.as_bytes();
    let mut count = 1usize;
    let mut redirection = None;
    let mut stdin_redirection = None;
    let mut redirection_started = false;
    if let Some(arguments) = arguments {
        for token in arguments.as_bytes().split(|byte| is_space(*byte)) {
            if token.is_empty() {
                continue;
            }
            if token == b"</etc/banner.txt" {
                if redirection_started || count != 1 {
                    return Err(LocalCommandExecError::InvalidPath);
                }
                redirection_started = true;
                stdin_redirection = Some(LocalCommandExecRedirection::StdinFromEtcBanner);
                continue;
            }
            if let Some(path) = token.strip_prefix(b">").and_then(|path| {
                if name.as_bytes() == b"stdin"
                    && stdin_redirection == Some(LocalCommandExecRedirection::StdinFromEtcBanner)
                {
                    LocalCommandVolatilePath::from_exact_stdin_report_path(path)
                } else {
                    LocalCommandVolatilePath::from_exact_stdout_path(path)
                }
            }) {
                let bare_name_combined_stdin_stdout = name.as_bytes() == b"stdin"
                    && stdin_redirection == Some(LocalCommandExecRedirection::StdinFromEtcBanner)
                    && redirection.is_none()
                    && count == 1;
                if !(name.as_bytes() == b"stdout" && !redirection_started && count == 1)
                    && !bare_name_combined_stdin_stdout
                {
                    return Err(LocalCommandExecError::InvalidPath);
                }
                redirection_started = true;
                redirection = Some(LocalCommandExecRedirection::StdoutToTmpStdout(path));
                continue;
            }
            if let Some(path) = token
                .strip_prefix(b">>")
                .and_then(LocalCommandVolatilePath::from_exact_stdout_path)
            {
                if name.as_bytes() != b"stdout" || redirection_started || count != 1 {
                    return Err(LocalCommandExecError::InvalidPath);
                }
                redirection_started = true;
                redirection = Some(LocalCommandExecRedirection::StdoutAppendTmpStdout(path));
                continue;
            }
            if let Some(path) = token
                .strip_prefix(b"2>")
                .and_then(LocalCommandVolatilePath::from_exact_stderr_path)
            {
                if name.as_bytes() != b"stderr" || redirection_started || count != 1 {
                    return Err(LocalCommandExecError::InvalidPath);
                }
                redirection_started = true;
                redirection = Some(LocalCommandExecRedirection::StderrToTmpStderr(path));
                continue;
            }
            if let Some(path) = token
                .strip_prefix(b"2>>")
                .and_then(LocalCommandVolatilePath::from_exact_stderr_path)
            {
                if name.as_bytes() != b"stderr" || redirection_started || count != 1 {
                    return Err(LocalCommandExecError::InvalidPath);
                }
                redirection_started = true;
                redirection = Some(LocalCommandExecRedirection::StderrAppendTmpStderr(path));
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
    }
    let argv = LocalCommandLiteralArgv::from_tokens(&tokens[..count])?
        .with_resolved_argv0(path.as_bytes())?;
    Ok(LocalCommandExecRequest {
        path,
        argv,
        redirection,
        stdin_redirection,
    })
}

fn parse_absolute_path_pipeline_request(
    command: ParsedLocalCommand<'_>,
) -> Result<LocalCommandPipelineRequest, LocalCommandExecError> {
    let arguments = command
        .arguments
        .ok_or(LocalCommandExecError::InvalidPath)?;
    let arguments = trim_ascii_space(arguments);
    let pipe = arguments
        .as_bytes()
        .iter()
        .position(|byte| *byte == b'|')
        .ok_or(LocalCommandExecError::InvalidPath)?;
    let producer_arguments = trim_ascii_space(&arguments[..pipe]);
    let consumer = trim_ascii_space(&arguments[pipe + 1..]);
    if consumer.is_empty() || consumer.as_bytes().contains(&b'|') {
        return Err(LocalCommandExecError::InvalidPath);
    }
    let (consumer_path, consumer_arguments) = split_pipeline_stage_path_and_arguments(consumer)?;
    reject_unbounded_pipeline_producer_arguments(producer_arguments)?;
    reject_unbounded_pipeline_consumer_arguments(consumer_arguments.unwrap_or(""))?;
    Ok(LocalCommandPipelineRequest {
        producer: parse_absolute_path_exec_request_with_arguments(
            command.name,
            optional_trimmed_arguments(producer_arguments),
        )?,
        middle: None,
        consumer: parse_absolute_path_pipeline_consumer_request(consumer_path, consumer_arguments)?,
    })
}

fn parse_absolute_path_pipeline_consumer_request(
    path_text: &str,
    arguments: Option<&str>,
) -> Result<LocalCommandExecRequest, LocalCommandExecError> {
    let Some(arguments) = arguments else {
        return parse_absolute_path_exec_request_with_arguments(path_text, None);
    };
    let arguments = trim_ascii_space(arguments);
    if path_text.as_bytes() == initramfs::PHASE10_STDIN_PATH {
        if let Some(target) = arguments.strip_prefix(">>") {
            if let Some(path) =
                LocalCommandVolatilePath::from_exact_pipeline_report_path(target.as_bytes())
                    .or_else(|| {
                        LocalCommandVolatilePath::from_exact_pipeline_combined_append_path(
                            target.as_bytes(),
                        )
                    })
            {
                let exec_path = LocalCommandExecPath::from_absolute(path_text.as_bytes())?;
                let argv = LocalCommandLiteralArgv::from_tokens(&[path_text.as_bytes()])?;
                return Ok(LocalCommandExecRequest {
                    path: exec_path,
                    argv,
                    redirection: Some(LocalCommandExecRedirection::StdoutAppendTmpStdout(path)),
                    stdin_redirection: None,
                });
            }
        }
        if let Some(target) = arguments.strip_prefix('>') {
            if let Some(path) =
                LocalCommandVolatilePath::from_exact_pipeline_report_path(target.as_bytes())
                    .or_else(|| {
                        LocalCommandVolatilePath::from_exact_pipeline_combined_path(
                            target.as_bytes(),
                        )
                    })
                    .or_else(|| {
                        LocalCommandVolatilePath::from_exact_pipeline_combined_append_path(
                            target.as_bytes(),
                        )
                    })
            {
                let exec_path = LocalCommandExecPath::from_absolute(path_text.as_bytes())?;
                let argv = LocalCommandLiteralArgv::from_tokens(&[path_text.as_bytes()])?;
                return Ok(LocalCommandExecRequest {
                    path: exec_path,
                    argv,
                    redirection: Some(LocalCommandExecRedirection::StdoutToTmpStdout(path)),
                    stdin_redirection: None,
                });
            }
        }
    }
    if path_text.as_bytes() == initramfs::PHASE10_STDERR_PATH {
        if let Some(target) = arguments.strip_prefix("2>>") {
            if let Some(path) =
                LocalCommandVolatilePath::from_exact_pipeline_stderr_path(target.as_bytes())
            {
                let exec_path = LocalCommandExecPath::from_absolute(path_text.as_bytes())?;
                let argv = LocalCommandLiteralArgv::from_tokens(&[path_text.as_bytes()])?;
                return Ok(LocalCommandExecRequest {
                    path: exec_path,
                    argv,
                    redirection: Some(LocalCommandExecRedirection::StderrAppendTmpStderr(path)),
                    stdin_redirection: None,
                });
            }
        }
        if let Some(target) = arguments.strip_prefix("2>") {
            if let Some(path) =
                LocalCommandVolatilePath::from_exact_pipeline_stderr_path(target.as_bytes())
                    .or_else(|| {
                        LocalCommandVolatilePath::from_exact_pipeline_combined_stderr_path(
                            target.as_bytes(),
                        )
                    })
            {
                let exec_path = LocalCommandExecPath::from_absolute(path_text.as_bytes())?;
                let argv = LocalCommandLiteralArgv::from_tokens(&[path_text.as_bytes()])?;
                return Ok(LocalCommandExecRequest {
                    path: exec_path,
                    argv,
                    redirection: Some(LocalCommandExecRedirection::StderrToTmpStderr(path)),
                    stdin_redirection: None,
                });
            }
        }
    }
    parse_absolute_path_exec_request_with_arguments(path_text, Some(arguments))
}

fn parse_absolute_path_exec_request_with_arguments(
    path_text: &str,
    arguments: Option<&str>,
) -> Result<LocalCommandExecRequest, LocalCommandExecError> {
    if !is_absolute_exec_path(path_text.as_bytes()) {
        return Err(LocalCommandExecError::InvalidPath);
    }
    let path = LocalCommandExecPath::from_absolute(path_text.as_bytes())?;
    let mut tokens: [&[u8]; LOCAL_COMMAND_LITERAL_ARGV_CAPACITY] =
        [&[]; LOCAL_COMMAND_LITERAL_ARGV_CAPACITY];
    tokens[0] = path_text.as_bytes();
    let mut count = 1usize;
    let mut redirection = None;
    let mut stdin_redirection = None;
    let mut redirection_started = false;
    if let Some(arguments) = arguments {
        for token in arguments.as_bytes().split(|byte| is_space(*byte)) {
            if token.is_empty() {
                continue;
            }
            if token == b"</etc/banner.txt" {
                if redirection_started || count != 1 {
                    return Err(LocalCommandExecError::InvalidPath);
                }
                redirection_started = true;
                stdin_redirection = Some(LocalCommandExecRedirection::StdinFromEtcBanner);
                continue;
            }
            if let Some(path) = token.strip_prefix(b">").and_then(|path| {
                if path_text.as_bytes() == initramfs::PHASE10_STDIN_PATH
                    && stdin_redirection == Some(LocalCommandExecRedirection::StdinFromEtcBanner)
                {
                    LocalCommandVolatilePath::from_exact_stdin_report_path(path)
                } else {
                    LocalCommandVolatilePath::from_exact_stdout_path(path)
                }
            }) {
                let direct_combined_stdin_stdout = path_text.as_bytes()
                    == initramfs::PHASE10_STDIN_PATH
                    && stdin_redirection == Some(LocalCommandExecRedirection::StdinFromEtcBanner)
                    && redirection.is_none()
                    && count == 1;
                if !(path_text.as_bytes() == initramfs::PHASE10_STDOUT_PATH
                    && !redirection_started
                    && count == 1)
                    && !direct_combined_stdin_stdout
                {
                    return Err(LocalCommandExecError::InvalidPath);
                }
                redirection_started = true;
                redirection = Some(LocalCommandExecRedirection::StdoutToTmpStdout(path));
                continue;
            }
            if let Some(path) = token
                .strip_prefix(b">>")
                .and_then(LocalCommandVolatilePath::from_exact_stdout_path)
            {
                if path_text.as_bytes() != initramfs::PHASE10_STDOUT_PATH
                    || redirection_started
                    || count != 1
                {
                    return Err(LocalCommandExecError::InvalidPath);
                }
                redirection_started = true;
                redirection = Some(LocalCommandExecRedirection::StdoutAppendTmpStdout(path));
                continue;
            }
            if let Some(path) = token
                .strip_prefix(b"2>")
                .and_then(LocalCommandVolatilePath::from_exact_stderr_path)
            {
                if path_text.as_bytes() != initramfs::PHASE10_STDERR_PATH
                    || redirection_started
                    || count != 1
                {
                    return Err(LocalCommandExecError::InvalidPath);
                }
                redirection_started = true;
                redirection = Some(LocalCommandExecRedirection::StderrToTmpStderr(path));
                continue;
            }
            if let Some(path) = token
                .strip_prefix(b"2>>")
                .and_then(LocalCommandVolatilePath::from_exact_stderr_path)
            {
                if path_text.as_bytes() != initramfs::PHASE10_STDERR_PATH
                    || redirection_started
                    || count != 1
                {
                    return Err(LocalCommandExecError::InvalidPath);
                }
                redirection_started = true;
                redirection = Some(LocalCommandExecRedirection::StderrAppendTmpStderr(path));
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
    }
    let argv = LocalCommandLiteralArgv::from_tokens(&tokens[..count])?
        .with_resolved_argv0(path.as_bytes())?;
    Ok(LocalCommandExecRequest {
        path,
        argv,
        redirection,
        stdin_redirection,
    })
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

fn optional_trimmed_arguments(text: &str) -> Option<&str> {
    let text = trim_ascii_space(text);
    if text.is_empty() { None } else { Some(text) }
}

fn split_pipeline_stage_path_and_arguments(
    stage: &str,
) -> Result<(&str, Option<&str>), LocalCommandExecError> {
    let stage = trim_ascii_space(stage);
    if stage.is_empty() {
        return Err(LocalCommandExecError::InvalidPath);
    }
    let bytes = stage.as_bytes();
    let mut split = 0usize;
    while split < bytes.len() && !is_space(bytes[split]) {
        split += 1;
    }
    let path = &stage[..split];
    let arguments = optional_trimmed_arguments(&stage[split..]);
    Ok((path, arguments))
}

fn reject_unbounded_pipeline_consumer_arguments(
    arguments: &str,
) -> Result<(), LocalCommandExecError> {
    let mut count = 0usize;
    let mut saw_stdin_redirection = false;
    for token in arguments.as_bytes().split(|byte| is_space(*byte)) {
        if token.is_empty() {
            continue;
        }
        if token == b"</etc/banner.txt" {
            if count != 0 || saw_stdin_redirection {
                return Err(LocalCommandExecError::InvalidPath);
            }
            saw_stdin_redirection = true;
            count += 1;
            continue;
        }
        if token == b">/tmp/pipeline-report.txt"
            || token == b">>/tmp/pipeline-report.txt"
            || token == b">/tmp/pipeline-combined.txt"
            || token == b">/tmp/pipeline-combined-append.txt"
            || token == b">>/tmp/pipeline-combined-append.txt"
            || token == b"2>/tmp/pipeline-combined-stderr.txt"
            || token == b"2>/tmp/pipeline-stderr.txt"
            || token == b"2>>/tmp/pipeline-stderr.txt"
        {
            if count != 0 || saw_stdin_redirection {
                return Err(LocalCommandExecError::InvalidPath);
            }
            count += 1;
            continue;
        }
        if saw_stdin_redirection || !is_supported_literal_exec_token(token) {
            return Err(LocalCommandExecError::InvalidPath);
        }
        count += 1;
        if count > 1 {
            return Err(LocalCommandExecError::InvalidPath);
        }
    }
    Ok(())
}

fn reject_unbounded_pipeline_producer_arguments(
    arguments: &str,
) -> Result<(), LocalCommandExecError> {
    let mut count = 0usize;
    let mut saw_stdin_redirection = false;
    for token in arguments.as_bytes().split(|byte| is_space(*byte)) {
        if token.is_empty() {
            continue;
        }
        if token == b"</etc/banner.txt" {
            if count != 0 || saw_stdin_redirection {
                return Err(LocalCommandExecError::InvalidPath);
            }
            saw_stdin_redirection = true;
            count += 1;
            continue;
        }
        if saw_stdin_redirection || !is_supported_literal_exec_token(token) {
            return Err(LocalCommandExecError::InvalidPath);
        }
        count += 1;
        if count > 1 {
            return Err(LocalCommandExecError::InvalidPath);
        }
    }
    Ok(())
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

fn is_bounded_bare_bin_command_name(name: &str) -> bool {
    !name.is_empty()
        && !name.as_bytes().iter().any(|byte| *byte == b'/')
        && LOCAL_COMMAND_BIN_LISTING
            .iter()
            .any(|(_, entry_name)| entry_name.as_bytes() == name.as_bytes())
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
    write_initramfs_text_file(sink, responses, initramfs::PHASE8_BANNER_PATH)
}

fn write_initramfs_text_file(
    sink: &mut impl LocalCommandSink,
    responses: &mut usize,
    path: &[u8],
) -> Result<(), LocalCommandCycleError> {
    let mut bytes = [0u8; LOCAL_COMMAND_INITRAMFS_CAT_BUFFER_LEN];
    let bytes_read = match sink.read_initramfs_file_via_syscall(path, &mut bytes) {
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

fn write_generated_root_selection_line(
    sink: &mut impl LocalCommandSink,
    response_lines: &mut usize,
) -> Result<(), LocalCommandCycleError> {
    let report = initramfs::generated_root_selection_report();
    write_str_part(sink, "talos: generated-root digest=")?;
    write_hex_u64_part(sink, report.digest)?;
    write_str_part(sink, " total-len=")?;
    write_hex_usize_part(sink, report.total_len)?;
    write_str_part(sink, " file-len=")?;
    write_hex_usize_part(sink, report.file_len)?;
    write_str_part(sink, " exec-len=")?;
    write_hex_usize_part(sink, report.exec_len)?;
    write_str_part(sink, " path=")?;
    write_byte_path_part(sink, initramfs::GENERATED_ROOT_FILE_PATH)?;
    write_str_part(sink, " exec-path=")?;
    write_byte_path_part(sink, initramfs::GENERATED_ROOT_EXEC_PATH)?;
    write_str_part(sink, " source=")?;
    write_str_part(sink, report.source)?;
    write_str_part(sink, " reason=")?;
    write_str_part(sink, report.reason)?;
    finish_dynamic_line(sink, response_lines)
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
    matches!(
        byte,
        b'a'..=b'z'
            | b'0'..=b'9'
            | b'/'
            | b'.'
            | b'-'
            | b'_'
    )
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
    if let Some(record) = summary.pingdiag {
        write_exec_pingdiag_line(sink, response_lines, record)?;
    }
    if let Some(record) = summary.pingdiag_controls {
        write_exec_pingdiag_controls_line(sink, response_lines, record)?;
    }
    if let Some(record) = summary.sockdiag {
        write_exec_sockdiag_line(sink, response_lines, record)?;
    }
    if let Some(record) = summary.sockdiag_controls {
        write_exec_sockdiag_controls_line(sink, response_lines, record)?;
    }
    write_exec_lifecycle_line(sink, response_lines, summary)?;
    if let Some(record) = summary.init_lifecycle_status {
        write_exec_init_lifecycle_status_line(sink, response_lines, record)?;
    }
    if let Some(record) = summary.vfs_exec_lifecycle_status {
        write_exec_vfs_exec_lifecycle_status_line(sink, response_lines, record)?;
    }
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

fn write_jobs_accounting_line(
    sink: &mut impl LocalCommandSink,
    response_lines: &mut usize,
    job: LocalCommandBackgroundJobRecord,
) -> Result<(), LocalCommandCycleError> {
    write_str_part(sink, "talos: jobs id=")?;
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
    } else {
        write_str_part(sink, " status=pending")?;
    }
    write_str_part(sink, " reaped=")?;
    write_str_part(sink, if job.reaped { "true" } else { "false" })?;
    write_str_part(sink, " source=background-vfs-exec-accounting")?;
    finish_dynamic_line(sink, response_lines)
}

fn write_pipeline_status_observation(
    sink: &mut impl LocalCommandSink,
    response_lines: &mut usize,
    records: [Option<LocalCommandProcessTableRecord>; LOCAL_COMMAND_PROCESS_TABLE_CAPACITY],
) -> Result<(), LocalCommandCycleError> {
    let mut participant_count = 0usize;
    let mut default_status = 0u64;
    let mut pipefail_status = 0u64;
    let mut first_nonzero_seen = false;

    for record in records.into_iter().flatten() {
        participant_count += 1;
        default_status = record.lifecycle.status;
        if !first_nonzero_seen && record.lifecycle.status != 0 {
            pipefail_status = record.lifecycle.status;
            first_nonzero_seen = true;
        }
    }

    if participant_count < 2 {
        write_line(
            sink,
            response_lines,
            "talos: pipestatus none source=bounded-process-table-pipeline-status",
        )?;
        return Ok(());
    }

    if !first_nonzero_seen {
        pipefail_status = default_status;
    }

    write_str_part(sink, "talos: pipestatus participants=")?;
    write_hex_usize_part(sink, participant_count)?;
    write_str_part(sink, " default-status=")?;
    write_hex_u64_part(sink, default_status)?;
    write_str_part(sink, " pipefail-status=")?;
    write_hex_u64_part(sink, pipefail_status)?;
    write_str_part(
        sink,
        " semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status",
    )?;
    finish_dynamic_line(sink, response_lines)?;

    for record in records.into_iter().flatten() {
        let lifecycle = record.lifecycle;
        write_str_part(sink, "talos: pipestatus-participant slot=")?;
        write_decimal_usize_part(sink, record.slot)?;
        write_str_part(sink, " pid=")?;
        write_hex_u64_part(sink, lifecycle.process_id)?;
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
        write_str_part(sink, " source=bounded-process-table-pipeline-status")?;
        finish_dynamic_line(sink, response_lines)?;
    }

    Ok(())
}

fn write_pipeline_summary(
    sink: &mut impl LocalCommandSink,
    response_lines: &mut usize,
    summary: LocalCommandPipelineSummary,
) -> Result<(), LocalCommandCycleError> {
    write_pipeline_record_line(sink, response_lines, summary.pipe)?;
    if let Some(record) = summary.second_pipe {
        write_pipeline_record_line(sink, response_lines, record)?;
    }
    write_pipeline_lifecycle_status_line(sink, response_lines, summary.lifecycle_status)?;
    write_exec_summary(sink, response_lines, summary.producer)?;
    if let Some(middle) = summary.middle {
        write_exec_summary(sink, response_lines, middle)?;
    }
    write_exec_summary(sink, response_lines, summary.consumer)
}

fn write_pipeline_record_line(
    sink: &mut impl LocalCommandSink,
    response_lines: &mut usize,
    record: LocalCommandPipeRecord,
) -> Result<(), LocalCommandCycleError> {
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
    finish_dynamic_line(sink, response_lines)
}

fn write_pipeline_lifecycle_status_line(
    sink: &mut impl LocalCommandSink,
    response_lines: &mut usize,
    record: LocalCommandPipelineLifecycleStatusRecord,
) -> Result<(), LocalCommandCycleError> {
    let producer = record.producer;
    let consumer = record.consumer;
    write_str_part(sink, "talos: pipeline-lifecycle-status record=")?;
    write_str_part(sink, record.identity)?;
    write_str_part(sink, " pipeline=")?;
    write_hex_usize_part(sink, record.pipe_id)?;
    write_str_part(sink, " producer-pid=")?;
    write_hex_u64_part(sink, producer.process_id)?;
    write_str_part(sink, " producer-path=")?;
    write_byte_path_part(sink, producer.source_path)?;
    write_str_part(sink, " producer-state=")?;
    write_str_part(sink, producer.state.name())?;
    write_str_part(sink, " producer-status=")?;
    write_hex_u64_part(sink, producer.status)?;
    write_str_part(sink, " producer-observed-status=")?;
    write_hex_u64_part(sink, producer.observed_status)?;
    write_str_part(sink, " producer-reaped=")?;
    write_str_part(sink, if producer.reaped { "true" } else { "false" })?;
    if let Some(middle) = record.middle {
        write_str_part(sink, " middle-pid=")?;
        write_hex_u64_part(sink, middle.process_id)?;
        write_str_part(sink, " middle-path=")?;
        write_byte_path_part(sink, middle.source_path)?;
        write_str_part(sink, " middle-state=")?;
        write_str_part(sink, middle.state.name())?;
        write_str_part(sink, " middle-status=")?;
        write_hex_u64_part(sink, middle.status)?;
        write_str_part(sink, " middle-observed-status=")?;
        write_hex_u64_part(sink, middle.observed_status)?;
        write_str_part(sink, " middle-reaped=")?;
        write_str_part(sink, if middle.reaped { "true" } else { "false" })?;
    }
    write_str_part(sink, " consumer-pid=")?;
    write_hex_u64_part(sink, consumer.process_id)?;
    write_str_part(sink, " consumer-path=")?;
    write_byte_path_part(sink, consumer.source_path)?;
    write_str_part(sink, " consumer-state=")?;
    write_str_part(sink, consumer.state.name())?;
    write_str_part(sink, " consumer-status=")?;
    write_hex_u64_part(sink, consumer.status)?;
    write_str_part(sink, " consumer-observed-status=")?;
    write_hex_u64_part(sink, consumer.observed_status)?;
    write_str_part(sink, " consumer-reaped=")?;
    write_str_part(sink, if consumer.reaped { "true" } else { "false" })?;
    write_str_part(
        sink,
        " source=kernel-owned-pipeline-lifecycle-status-record",
    )?;
    finish_dynamic_line(sink, response_lines)
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

fn write_exec_init_lifecycle_status_line(
    sink: &mut impl LocalCommandSink,
    response_lines: &mut usize,
    record: LocalCommandInitLifecycleStatusRecord,
) -> Result<(), LocalCommandCycleError> {
    let lifecycle = record.lifecycle;
    write_str_part(sink, "talos: init-lifecycle-status record=")?;
    write_str_part(sink, record.identity)?;
    write_str_part(sink, " pid=")?;
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
    write_str_part(sink, " source=kernel-owned-lifecycle-status-record")?;
    finish_dynamic_line(sink, response_lines)
}

fn write_exec_vfs_exec_lifecycle_status_line(
    sink: &mut impl LocalCommandSink,
    response_lines: &mut usize,
    record: LocalCommandVfsExecLifecycleStatusRecord,
) -> Result<(), LocalCommandCycleError> {
    let lifecycle = record.lifecycle;
    write_str_part(sink, "talos: vfs-exec-lifecycle-status record=")?;
    write_str_part(sink, record.identity)?;
    write_str_part(sink, " pid=")?;
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
    write_str_part(
        sink,
        " source=kernel-owned-vfs-exec-lifecycle-status-record",
    )?;
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

fn write_exec_pingdiag_line(
    sink: &mut impl LocalCommandSink,
    response_lines: &mut usize,
    record: LocalCommandPingdiagRecord,
) -> Result<(), LocalCommandCycleError> {
    write_str_part(sink, "talos: pingdiag fd=")?;
    write_hex_usize_part(sink, record.process_descriptor)?;
    write_str_part(sink, " destination=")?;
    write_ipv4_part(sink, record.destination_ipv4)?;
    write_str_part(sink, " payload-bytes=")?;
    write_hex_usize_part(sink, record.payload_len)?;
    write_str_part(sink, " start=")?;
    write_str_part(sink, ping_step_name(record.start_step))?;
    write_str_part(sink, " arp-driver=")?;
    write_str_part(sink, record.arp_driver_step)?;
    write_str_part(sink, " arp-frame-bytes=")?;
    write_hex_usize_part(sink, record.arp_frame_len)?;
    write_str_part(sink, " arp-pump=")?;
    write_str_part(sink, ping_step_name(record.arp_pump_step))?;
    write_str_part(sink, " icmp-driver=")?;
    write_str_part(sink, record.icmp_driver_step)?;
    write_str_part(sink, " icmp-frame-bytes=")?;
    write_hex_usize_part(sink, record.icmp_frame_len)?;
    write_str_part(sink, " result=")?;
    write_str_part(sink, ping_step_name(record.result_step))?;
    write_str_part(sink, " status=")?;
    write_str_part(sink, ping_status_name(record.status))?;
    write_str_part(sink, " status-payload-bytes=")?;
    write_hex_usize_part(sink, record.status_payload_len)?;
    write_str_part(sink, " closed=")?;
    write_str_part(sink, if record.closed { "true" } else { "false" })?;
    write_str_part(sink, " source=")?;
    write_str_part(sink, record.source)?;
    finish_dynamic_line(sink, response_lines)
}

fn write_exec_pingdiag_controls_line(
    sink: &mut impl LocalCommandSink,
    response_lines: &mut usize,
    record: LocalCommandPingdiagControlRecord,
) -> Result<(), LocalCommandCycleError> {
    write_str_part(sink, "talos: pingdiag-controls malformed-arguments=")?;
    write_str_part(sink, record.malformed_arguments)?;
    write_str_part(sink, " missing-executable=")?;
    write_str_part(sink, record.missing_executable_identity)?;
    write_str_part(sink, " owner-descriptor=")?;
    write_str_part(sink, record.owner_descriptor_failure)?;
    write_str_part(sink, " invalid-closed-descriptor=")?;
    write_str_part(sink, record.invalid_closed_descriptor)?;
    write_str_part(sink, " queue-backpressure=")?;
    write_str_part(sink, record.queue_backpressure)?;
    write_str_part(sink, " timeout-retry=")?;
    write_str_part(sink, record.timeout_retry)?;
    write_str_part(sink, " device-errors=")?;
    write_str_part(sink, record.device_errors)?;
    write_str_part(sink, " syscall-vocabulary=")?;
    write_str_part(sink, record.syscall_vocabulary)?;
    write_str_part(sink, " source=")?;
    write_str_part(sink, record.source)?;
    finish_dynamic_line(sink, response_lines)
}

fn write_exec_sockdiag_line(
    sink: &mut impl LocalCommandSink,
    response_lines: &mut usize,
    record: LocalCommandSockdiagRecord,
) -> Result<(), LocalCommandCycleError> {
    write_str_part(sink, "talos: sockdiag fd=")?;
    write_hex_usize_part(sink, record.process_descriptor)?;
    write_str_part(sink, " client-fd=")?;
    write_hex_usize_part(sink, record.client_descriptor)?;
    write_str_part(sink, " accepted-fd=")?;
    write_hex_usize_part(sink, record.accepted_descriptor)?;
    write_str_part(sink, " poll-empty-listener=")?;
    write_hex_u64_part(sink, record.empty_listener_revents as u64)?;
    write_str_part(sink, " poll-pending-listener=")?;
    write_hex_u64_part(sink, record.pending_listener_revents as u64)?;
    write_str_part(sink, " poll-empty-recv=")?;
    write_hex_u64_part(sink, record.empty_recv_revents as u64)?;
    write_str_part(sink, " poll-payload-recv=")?;
    write_hex_u64_part(sink, record.payload_recv_revents as u64)?;
    write_str_part(sink, " poll-write-ready=")?;
    write_hex_u64_part(sink, record.write_ready_revents as u64)?;
    write_str_part(sink, " poll-write-backpressure=")?;
    write_hex_u64_part(sink, record.write_backpressure_revents as u64)?;
    write_str_part(sink, " poll-peer-hangup=")?;
    write_hex_u64_part(sink, record.peer_hangup_revents as u64)?;
    write_str_part(sink, " poll-invalid-descriptor=")?;
    write_hex_u64_part(sink, record.invalid_descriptor_revents as u64)?;
    write_str_part(sink, " poll-non-socket-descriptor=")?;
    write_hex_u64_part(sink, record.non_socket_descriptor_revents as u64)?;
    write_str_part(sink, " poll-wait-immediate=")?;
    write_hex_u64_part(sink, record.poll_wait_immediate_revents as u64)?;
    write_str_part(sink, " poll-wait-pending-listener=")?;
    write_hex_u64_part(sink, record.poll_wait_pending_listener_revents as u64)?;
    write_str_part(sink, " poll-wait-payload-recv=")?;
    write_hex_u64_part(sink, record.poll_wait_payload_revents as u64)?;
    write_str_part(sink, " poll-wait-timeout=")?;
    write_hex_u64_part(sink, record.poll_wait_timeout_revents as u64)?;
    write_str_part(sink, " poll-wait-peer-hangup=")?;
    write_hex_u64_part(sink, record.poll_wait_peer_hangup_revents as u64)?;
    write_str_part(sink, " poll-wait-blocked-state=")?;
    write_str_part(sink, record.poll_wait_blocked_state)?;
    write_str_part(sink, " poll-wait-ready-state=")?;
    write_str_part(sink, record.poll_wait_ready_state)?;
    write_str_part(sink, " poll-wait-timeout-state=")?;
    write_str_part(sink, record.poll_wait_timeout_state)?;
    write_str_part(sink, " poll-wait-ready-count=")?;
    write_hex_u64_part(sink, record.poll_wait_ready_count)?;
    write_str_part(sink, " poll-wait-timeout-tick=")?;
    write_hex_u64_part(sink, record.poll_wait_timeout_tick)?;
    write_str_part(sink, " client-send=")?;
    write_hex_usize_part(sink, record.client_send_bytes)?;
    write_str_part(sink, " server-recv=")?;
    write_hex_usize_part(sink, record.server_recv_bytes)?;
    write_str_part(sink, " server-send=")?;
    write_hex_usize_part(sink, record.server_send_bytes)?;
    write_str_part(sink, " client-recv=")?;
    write_hex_usize_part(sink, record.client_recv_bytes)?;
    write_str_part(sink, " payload=")?;
    write_str_part(sink, record.client_payload)?;
    write_str_part(sink, " reply=")?;
    write_str_part(sink, record.server_payload)?;
    write_str_part(sink, " domain=")?;
    write_hex_u64_part(sink, record.domain)?;
    write_str_part(sink, " type=")?;
    write_hex_u64_part(sink, record.socket_type)?;
    write_str_part(sink, " protocol=")?;
    write_hex_u64_part(sink, record.protocol)?;
    write_str_part(sink, " bind-ipv4=")?;
    write_hex_u64_part(sink, record.local_ipv4_be as u64)?;
    write_str_part(sink, " bind-port=")?;
    write_hex_u64_part(sink, record.local_port as u64)?;
    write_str_part(sink, " bind-return=")?;
    write_hex_u64_part(sink, record.bind_return)?;
    write_str_part(sink, " listen-backlog=")?;
    write_hex_u64_part(sink, record.listen_backlog as u64)?;
    write_str_part(sink, " listen-return=")?;
    write_hex_u64_part(sink, record.listen_return)?;
    write_str_part(sink, " connect-return=")?;
    write_hex_u64_part(sink, record.connect_return)?;
    write_str_part(sink, " accept-return=")?;
    write_hex_u64_part(sink, record.accept_return)?;
    write_str_part(sink, " socket-state=")?;
    write_str_part(sink, record.socket_state)?;
    write_str_part(sink, " client-state=")?;
    write_str_part(sink, record.client_state)?;
    write_str_part(sink, " accepted-state=")?;
    write_str_part(sink, record.accepted_state)?;
    write_str_part(sink, " descriptor-kind=")?;
    write_str_part(sink, record.descriptor_kind)?;
    write_str_part(sink, " descriptor-access=")?;
    write_str_part(sink, record.descriptor_access)?;
    write_str_part(sink, " close-return=")?;
    write_hex_u64_part(sink, record.close_return)?;
    write_str_part(sink, " backing-closed=")?;
    write_str_part(
        sink,
        if record.backing_closed {
            "true"
        } else {
            "false"
        },
    )?;
    write_str_part(sink, " cross-process-server-owner=")?;
    write_hex_u64_part(sink, record.cross_process_server_owner)?;
    write_str_part(sink, " cross-process-client-owner=")?;
    write_hex_u64_part(sink, record.cross_process_client_owner)?;
    write_str_part(sink, " cross-process-server-fd=")?;
    write_hex_usize_part(sink, record.cross_process_server_descriptor)?;
    write_str_part(sink, " cross-process-client-fd=")?;
    write_hex_usize_part(sink, record.cross_process_client_descriptor)?;
    write_str_part(sink, " cross-process-accepted-fd=")?;
    write_hex_usize_part(sink, record.cross_process_accepted_descriptor)?;
    write_str_part(sink, " cross-process-listener=")?;
    write_hex_u64_part(sink, record.cross_process_listener_revents as u64)?;
    write_str_part(sink, " cross-process-payload=")?;
    write_hex_u64_part(sink, record.cross_process_payload_revents as u64)?;
    write_str_part(sink, " cross-process-hangup=")?;
    write_hex_u64_part(sink, record.cross_process_hangup_revents as u64)?;
    write_str_part(sink, " cross-process-accept-wait=")?;
    write_hex_u64_part(sink, record.cross_process_accept_wait_revents as u64)?;
    write_str_part(sink, " cross-process-payload-wait=")?;
    write_hex_u64_part(sink, record.cross_process_payload_wait_revents as u64)?;
    write_str_part(sink, " cross-process-cleanup-close=")?;
    write_hex_u64_part(sink, record.cross_process_cleanup_close_return)?;
    write_str_part(sink, " cross-process-payload-text=")?;
    write_str_part(sink, record.cross_process_payload)?;
    write_str_part(sink, " cross-process-reply=")?;
    write_str_part(sink, record.cross_process_reply)?;
    write_str_part(sink, " cross-process-ownership=")?;
    write_str_part(sink, record.cross_process_descriptor_ownership)?;
    write_str_part(sink, " cross-process-backing-closed=")?;
    write_str_part(
        sink,
        if record.cross_process_backing_closed {
            "true"
        } else {
            "false"
        },
    )?;
    write_str_part(sink, " smoltcp-connection-id=")?;
    write_hex_u64_part(sink, record.smoltcp_connection_id)?;
    write_str_part(sink, " smoltcp-handshake-client=")?;
    write_str_part(sink, record.smoltcp_handshake_client_state)?;
    write_str_part(sink, " smoltcp-handshake-server=")?;
    write_str_part(sink, record.smoltcp_handshake_server_state)?;
    write_str_part(sink, " smoltcp-handshake-steps=")?;
    write_hex_usize_part(sink, record.smoltcp_handshake_steps)?;
    write_str_part(sink, " smoltcp-handshake-c2s-frames=")?;
    write_hex_usize_part(sink, record.smoltcp_handshake_client_to_server_frames)?;
    write_str_part(sink, " smoltcp-handshake-s2c-frames=")?;
    write_hex_usize_part(sink, record.smoltcp_handshake_server_to_client_frames)?;
    write_str_part(sink, " smoltcp-accepted-attached=")?;
    write_str_part(
        sink,
        if record.smoltcp_accepted_attached {
            "true"
        } else {
            "false"
        },
    )?;
    write_str_part(sink, " smoltcp-payload-transfers=")?;
    write_hex_u64_part(sink, record.smoltcp_payload_transfers)?;
    write_str_part(sink, " smoltcp-payload-len=")?;
    write_hex_usize_part(sink, record.smoltcp_payload_len)?;
    write_str_part(sink, " smoltcp-payload-client=")?;
    write_str_part(sink, record.smoltcp_payload_client_state)?;
    write_str_part(sink, " smoltcp-payload-server=")?;
    write_str_part(sink, record.smoltcp_payload_server_state)?;
    write_str_part(sink, " driver-packet-rx=")?;
    write_str_part(sink, record.driver_packet_rx_step)?;
    write_str_part(sink, " driver-packet-rx-len=")?;
    write_hex_usize_part(sink, record.driver_packet_rx_frame_len)?;
    write_str_part(sink, " driver-packet-tx=")?;
    write_str_part(sink, record.driver_packet_tx_step)?;
    write_str_part(sink, " driver-packet-tx-len=")?;
    write_hex_usize_part(sink, record.driver_packet_tx_frame_len)?;
    write_str_part(sink, " driver-packet-tx-observed-len=")?;
    write_hex_usize_part(sink, record.driver_packet_tx_observed_len)?;
    write_str_part(sink, " driver-packet-tx-queued-after-pop=")?;
    write_hex_usize_part(sink, record.driver_packet_tx_queued_after_pop)?;
    write_str_part(sink, " driver-packet-backpressure=")?;
    write_str_part(sink, record.driver_packet_backpressure_step)?;
    write_str_part(sink, " driver-packet-backpressure-rx-queued=")?;
    write_hex_usize_part(sink, record.driver_packet_backpressure_rx_queued)?;
    write_str_part(sink, " driver-packet-backpressure-tx-queued=")?;
    write_hex_usize_part(sink, record.driver_packet_backpressure_tx_queued)?;
    write_str_part(sink, " driver-packet-evidence=")?;
    write_str_part(sink, record.driver_packet_evidence)?;
    write_str_part(sink, " source=")?;
    write_str_part(sink, record.source)?;
    finish_dynamic_line(sink, response_lines)
}

fn write_exec_sockdiag_controls_line(
    sink: &mut impl LocalCommandSink,
    response_lines: &mut usize,
    record: LocalCommandSockdiagControlRecord,
) -> Result<(), LocalCommandCycleError> {
    write_str_part(sink, "talos: sockdiag-controls malformed-arguments=")?;
    write_str_part(sink, record.malformed_arguments)?;
    write_str_part(sink, " missing-executable=")?;
    write_str_part(sink, record.missing_executable_identity)?;
    write_str_part(sink, " unsupported-domain=")?;
    write_str_part(sink, record.unsupported_domain)?;
    write_str_part(sink, " unsupported-type=")?;
    write_str_part(sink, record.unsupported_type)?;
    write_str_part(sink, " unsupported-protocol=")?;
    write_str_part(sink, record.unsupported_protocol)?;
    write_str_part(sink, " listen-before-bind=")?;
    write_str_part(sink, record.listen_before_bind)?;
    write_str_part(sink, " invalid-bind-endpoint=")?;
    write_str_part(sink, record.invalid_bind_endpoint)?;
    write_str_part(sink, " invalid-backlog=")?;
    write_str_part(sink, record.invalid_backlog)?;
    write_str_part(sink, " repeated-bind=")?;
    write_str_part(sink, record.repeated_bind)?;
    write_str_part(sink, " repeated-listen=")?;
    write_str_part(sink, record.repeated_listen)?;
    write_str_part(sink, " accept-before-connect=")?;
    write_str_part(sink, record.accept_before_connect)?;
    write_str_part(sink, " missing-listener=")?;
    write_str_part(sink, record.missing_listener)?;
    write_str_part(sink, " queue-backpressure=")?;
    write_str_part(sink, record.queue_backpressure)?;
    write_str_part(sink, " non-socket-descriptor=")?;
    write_str_part(sink, record.non_socket_descriptor)?;
    write_str_part(sink, " empty-recv=")?;
    write_str_part(sink, record.empty_recv)?;
    write_str_part(sink, " send-invalid-flags=")?;
    write_str_part(sink, record.send_invalid_flags)?;
    write_str_part(sink, " recv-invalid-flags=")?;
    write_str_part(sink, record.recv_invalid_flags)?;
    write_str_part(sink, " payload-queue-backpressure=")?;
    write_str_part(sink, record.payload_queue_backpressure)?;
    write_str_part(sink, " send-after-peer-close=")?;
    write_str_part(sink, record.send_after_peer_close)?;
    write_str_part(sink, " poll-unsupported-events=")?;
    write_str_part(sink, record.poll_unsupported_events)?;
    write_str_part(sink, " poll-invalid-descriptor=")?;
    write_str_part(sink, record.poll_invalid_descriptor)?;
    write_str_part(sink, " poll-non-socket-descriptor=")?;
    write_str_part(sink, record.poll_non_socket_descriptor)?;
    write_str_part(sink, " poll-wait-scalar-dispatch=")?;
    write_str_part(sink, record.poll_wait_scalar_dispatch)?;
    write_str_part(sink, " poll-wait-invalid-timeout=")?;
    write_str_part(sink, record.poll_wait_invalid_timeout)?;
    write_str_part(sink, " poll-wait-unsupported-events=")?;
    write_str_part(sink, record.poll_wait_unsupported_events)?;
    write_str_part(sink, " invalid-closed-descriptor=")?;
    write_str_part(sink, record.invalid_closed_descriptor)?;
    write_str_part(sink, " syscall-vocabulary=")?;
    write_str_part(sink, record.syscall_vocabulary)?;
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

fn write_waitpid_status_line_with_source(
    sink: &mut impl LocalCommandSink,
    response_lines: &mut usize,
    lifecycle: LocalCommandProcessLifecycleRecord,
    source: &str,
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
    write_str_part(sink, " reaped=true source=")?;
    write_str_part(sink, source)?;
    finish_dynamic_line(sink, response_lines)
}

fn write_waitpid_no_child_pid_line(
    sink: &mut impl LocalCommandSink,
    response_lines: &mut usize,
    process_id: u64,
) -> Result<(), LocalCommandCycleError> {
    write_str_part(sink, "talos: waitpid no-child pid=")?;
    write_hex_u64_part(sink, process_id)?;
    write_str_part(sink, " source=explicit-pid-lifecycle-record")?;
    finish_dynamic_line(sink, response_lines)
}

fn write_waitpid_unsupported_pid_line(
    sink: &mut impl LocalCommandSink,
    response_lines: &mut usize,
    process_id: u64,
) -> Result<(), LocalCommandCycleError> {
    write_str_part(sink, "talos: waitpid unsupported-pid pid=")?;
    write_hex_u64_part(sink, process_id)?;
    write_str_part(sink, " source=explicit-pid-lifecycle-record")?;
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

fn write_ipv4_part(
    sink: &mut impl LocalCommandSink,
    address: [u8; 4],
) -> Result<(), LocalCommandCycleError> {
    write_decimal_usize_part(sink, address[0] as usize)?;
    write_str_part(sink, ".")?;
    write_decimal_usize_part(sink, address[1] as usize)?;
    write_str_part(sink, ".")?;
    write_decimal_usize_part(sink, address[2] as usize)?;
    write_str_part(sink, ".")?;
    write_decimal_usize_part(sink, address[3] as usize)
}

fn ping_step_name(kind: syscall::PingOperationSyscallSubstituteStepKind) -> &'static str {
    match kind {
        syscall::PingOperationSyscallSubstituteStepKind::StartedPendingArp => "started-pending-arp",
        syscall::PingOperationSyscallSubstituteStepKind::StartedInflight => "started-inflight",
        syscall::PingOperationSyscallSubstituteStepKind::NoFrame => "no-frame",
        syscall::PingOperationSyscallSubstituteStepKind::AdvancedToInflight => {
            "advanced-to-inflight"
        }
        syscall::PingOperationSyscallSubstituteStepKind::RetryTransmitted => "retry-transmitted",
        syscall::PingOperationSyscallSubstituteStepKind::Completed => "completed",
        syscall::PingOperationSyscallSubstituteStepKind::TimedOut => "timed-out",
    }
}

fn ping_status_name(kind: syscall::PingOperationSyscallSubstituteStatusKind) -> &'static str {
    match kind {
        syscall::PingOperationSyscallSubstituteStatusKind::Idle => "idle",
        syscall::PingOperationSyscallSubstituteStatusKind::PendingArp => "pending-arp",
        syscall::PingOperationSyscallSubstituteStatusKind::Inflight => "inflight",
        syscall::PingOperationSyscallSubstituteStatusKind::Completed => "completed",
        syscall::PingOperationSyscallSubstituteStatusKind::TimedOut => "timed-out",
    }
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
        bytes: [u8; 65536],
        len: usize,
        fail_after: usize,
        writes: usize,
    }

    impl CaptureSink {
        const fn new() -> Self {
            Self {
                bytes: [0; 65536],
                len: 0,
                fail_after: usize::MAX,
                writes: 0,
            }
        }

        const fn failing_after(fail_after: usize) -> Self {
            Self {
                bytes: [0; 65536],
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
	talos: commands help status stdio pwd echo ls cat cd exec laststatus waitpid jobs ps pipestatus\n\
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
        assert_eq!(result.response_lines(), 5);
        assert_eq!(backend.as_str(), "talos> bin\ndir\nempty\netc\ngenerated\n");
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
        assert_eq!(result.response_lines(), 8);
        assert_eq!(
            backend.as_str(),
            "talos> init\nzero\nstatus42\nstdout\nstdin\nstderr\npingdiag\nsockdiag\n"
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
        assert_eq!(root_ls.response_lines(), 5);
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
        assert_eq!(bin_ls.response_lines(), 8);
        assert_eq!(cd_root.line(), b"cd /");
        assert_eq!(cd_root.status(), LocalCommandStatus::Handled);
        assert_eq!(cd_root.response_lines(), 0);
        assert_eq!(final_root_ls.line(), b"ls");
        assert_eq!(final_root_ls.status(), LocalCommandStatus::Handled);
        assert_eq!(final_root_ls.response_lines(), 5);
        assert_eq!(
            backend.as_str(),
            "talos> bin\n\
dir\n\
empty\n\
etc\n\
generated\n\
talos> talos> banner.txt\n\
	talos> talos> init\n\
	zero\n\
		status42\n\
		stdout\n\
			stdin\n\
			stderr\n\
			pingdiag\n\
			sockdiag\n\
			talos> talos> bin\n\
dir\n\
empty\n\
etc\n\
generated\n"
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
    fn local_command_loop_cats_generated_manifest_through_vfs_syscall_descriptor() {
        let input = ScriptedInput::new(*b"cat /generated/manifest.txt\r", 28);
        let mut backend = CaptureSink::new();
        let result = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            run_one_descriptor_backed_serial_command(&mut io).unwrap()
        };

        assert_eq!(result.line(), b"cat /generated/manifest.txt");
        assert_eq!(result.status(), LocalCommandStatus::Handled);
        assert_eq!(result.response_lines(), 1);
        assert_eq!(
            backend.as_str(),
            "talos> Talos generated-root manifest fixture\n"
        );
    }

    #[test_case]
    fn local_command_loop_reports_generated_root_source_reason_in_tail_stable_position() {
        initramfs::install_external_generated_root_fallback("tail-stable-unit-test");
        let input = ScriptedInput::new(*b"rootinfo\r", 9);
        let mut backend = CaptureSink::new();
        let result = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            run_one_descriptor_backed_serial_command(&mut io).unwrap()
        };
        let output = backend.as_str();

        assert_eq!(result.line(), b"rootinfo");
        assert_eq!(result.status(), LocalCommandStatus::Handled);
        assert_eq!(result.response_lines(), 1);
        assert!(output.starts_with("talos> talos: generated-root digest=0x"));
        assert!(output.contains(" path=/generated/manifest.txt exec-path=/generated/status7"));
        assert!(output.ends_with(" source=compiled-fallback reason=tail-stable-unit-test\n"));
        let path_pos = output.find(" path=").expect("path marker");
        let source_pos = output.find(" source=").expect("source marker");
        assert!(path_pos < source_pos);
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
        assert_eq!(result.response_lines(), 11);
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
            "talos: init-lifecycle-status record=phase12-local-process-lifecycle-status-record-v1 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/init state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=kernel-owned-lifecycle-status-record\n"
        ));
        assert!(output.contains(
            "talos: vfs-exec-lifecycle-status record=phase12-local-vfs-exec-lifecycle-status-record-v2 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/init state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=kernel-owned-vfs-exec-lifecycle-status-record\n"
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
        assert_eq!(result.response_lines(), 10);
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
            "talos: vfs-exec-lifecycle-status record=phase12-local-vfs-exec-lifecycle-status-record-v2 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/zero state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=kernel-owned-vfs-exec-lifecycle-status-record\n"
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
        assert_eq!(exec.response_lines(), 10);
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
            "talos: vfs-exec-lifecycle-status record=phase12-local-vfs-exec-lifecycle-status-record-v2 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=kernel-owned-vfs-exec-lifecycle-status-record\n"
        ));
        assert!(output.contains(
            "talos: exec-status boundary=lower-aarch64-svc-status-equivalent marker=0x0000000000007a10 status=0x000000000000002a complete=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=lifecycle-record\n"
        ));
    }

    #[test_case]
    fn local_command_loop_execs_shell_visible_pingdiag_through_vfs_diagnostic_layers() {
        let input = ScriptedInput::new(
            *b"exec /bin/pingdiag\rwaitpid\rlaststatus\rexec /bin/pingdiag extra\rexec /bin/missingdiag\r",
            85,
        );
        let mut backend = CaptureSink::new();
        let (exec, waited, observed, malformed, missing) = {
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

        assert_eq!(exec.line(), b"exec /bin/pingdiag");
        assert_eq!(exec.status(), LocalCommandStatus::Handled);
        assert_eq!(exec.response_lines(), 11);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.line(), b"laststatus");
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert_eq!(malformed.line(), b"exec /bin/pingdiag extra");
        assert_eq!(malformed.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(missing.line(), b"exec /bin/missingdiag");
        assert_eq!(missing.status(), LocalCommandStatus::UnexpectedArgument);
        assert!(output.contains("talos> talos: exec path=/bin/pingdiag source=vfs-open-read\n"));
        assert!(output.contains(
            "talos: pingdiag fd=0x0000000000000003 destination=192.0.2.20 payload-bytes=0x0000000000000004 start=started-pending-arp arp-driver=transmitted"
        ));
        assert!(output.contains("arp-pump=advanced-to-inflight icmp-driver=transmitted"));
        assert!(output.contains(
            "result=completed status=completed status-payload-bytes=0x0000000000000004 closed=true source=vfs-userspace-diagnostic-svc+process-local-descriptor+packet-queue-pump\n"
        ));
        assert!(output.contains(
            "talos: pingdiag-controls malformed-arguments=exec-invalid-path missing-executable=exec-not-found owner-descriptor=EBADF invalid-closed-descriptor=EBADF queue-backpressure=ENOSPC timeout-retry=timed-out-after-retry device-errors=EIO syscall-vocabulary=SyscallNumber/STABLE_SVC_IMMEDIATE/TALOS_* unchanged source=shell-pingdiag-controls\n"
        ));
        assert!(output.contains(
            "talos: exec-lifecycle pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/pingdiag state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/pingdiag state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/pingdiag state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains("talos> talos: exec-invalid-path\n"));
        assert!(output.contains("talos> talos: exec-not-found\n"));
    }

    #[test_case]
    fn local_command_loop_execs_shell_visible_sockdiag_through_userspace_socket_abi() {
        let bytes = *b"exec /bin/sockdiag\rwaitpid\rlaststatus\rexec /bin/sockdiag extra\rexec /bin/missingsock\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (exec, waited, observed, malformed, missing) = {
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

        assert_eq!(exec.line(), b"exec /bin/sockdiag");
        assert_eq!(exec.status(), LocalCommandStatus::Handled);
        assert_eq!(exec.response_lines(), 11);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.line(), b"laststatus");
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert_eq!(malformed.line(), b"exec /bin/sockdiag extra");
        assert_eq!(malformed.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(missing.line(), b"exec /bin/missingsock");
        assert_eq!(missing.status(), LocalCommandStatus::UnexpectedArgument);
        assert!(output.contains("talos> talos: exec path=/bin/sockdiag source=vfs-open-read\n"));
        assert!(output.contains(concat!(
            "talos: sockdiag fd=0x0000000000000003 client-fd=0x0000000000000004 accepted-fd=0x0000000000000006 ",
            "poll-empty-listener=0x0000000000000000 poll-pending-listener=0x0000000000000001 ",
            "poll-empty-recv=0x0000000000000000 poll-payload-recv=0x0000000000000001 ",
            "poll-write-ready=0x0000000000000002 poll-write-backpressure=0x0000000000000000 ",
            "poll-peer-hangup=0x0000000000000005 poll-invalid-descriptor=0x0000000000000008 ",
            "poll-non-socket-descriptor=0x0000000000000008 ",
            "poll-wait-immediate=0x0000000000000001 poll-wait-pending-listener=0x0000000000000001 ",
            "poll-wait-payload-recv=0x0000000000000001 poll-wait-timeout=0x0000000000000000 ",
            "poll-wait-peer-hangup=0x0000000000000005 poll-wait-blocked-state=blocked ",
            "poll-wait-ready-state=runnable poll-wait-timeout-state=runnable ",
            "poll-wait-ready-count=0x0000000000000001 poll-wait-timeout-tick=0x0000000000000016 ",
            "client-send=0x000000000000000e ",
            "server-recv=0x000000000000000e server-send=0x000000000000000e client-recv=0x000000000000000e ",
            "payload=client->server reply=server->client domain=0x0000000000000002 ",
            "type=0x0000000000000001 protocol=0x0000000000000000 bind-ipv4=0x000000007f000001 ",
            "bind-port=0x0000000000001f90 bind-return=0x0000000000000000 listen-backlog=0x0000000000000001 ",
            "listen-return=0x0000000000000000 connect-return=0x0000000000000000 ",
            "accept-return=0x0000000000000006 socket-state=listening client-state=connected ",
            "accepted-state=accepted descriptor-kind=socket descriptor-access=read-write ",
            "close-return=0x0000000000000000 backing-closed=true ",
            "cross-process-server-owner=0x0000000000000001 cross-process-client-owner=0x0000000000000002 ",
            "cross-process-server-fd=0x0000000000000003 cross-process-client-fd=0x0000000000000003 ",
            "cross-process-accepted-fd=0x0000000000000004 cross-process-listener=0x0000000000000001 ",
            "cross-process-payload=0x0000000000000001 cross-process-hangup=0x0000000000000005 ",
            "cross-process-accept-wait=0x0000000000000001 cross-process-payload-wait=0x0000000000000001 ",
            "cross-process-cleanup-close=0x0000000000000000 cross-process-payload-text=cross-client ",
            "cross-process-reply=cross-server ",
            "cross-process-ownership=server-owner-listener-accepted+client-owner-connected ",
            "cross-process-backing-closed=true "
        )));
        assert!(output.contains(
            "smoltcp-connection-id=0x0000000000000001 smoltcp-handshake-client=established smoltcp-handshake-server=established "
        ));
        assert!(output.contains(
            "smoltcp-accepted-attached=true smoltcp-payload-transfers=0x0000000000000001 smoltcp-payload-len=0x000000000000000e smoltcp-payload-client=established smoltcp-payload-server=established "
        ));
        assert!(output.contains(
            "driver-packet-rx=received driver-packet-rx-len=0x0000000000000004 driver-packet-tx=transmitted driver-packet-tx-len=0x0000000000000003 driver-packet-tx-observed-len=0x0000000000000003 driver-packet-tx-queued-after-pop=0x0000000000000000 driver-packet-backpressure=tx-queue-full driver-packet-backpressure-rx-queued=0x0000000000000001 driver-packet-backpressure-tx-queued=0x0000000000000001 driver-packet-evidence=host-qemu-substitute-not-live-packet-io "
        ));
        assert!(output.contains(
            "source=vfs-userspace-sockdiag+userspace-socket-abi-v1+talos-socket-bind-listen-connect-accept-send-recv-poll-wait-close+process-descriptor+cross-process-local-rendezvous+private-smoltcp-tcp-bridge+driver-packet-adapter-substrate\n"
        ));
        assert!(output.contains(concat!(
            "talos: sockdiag-controls malformed-arguments=exec-invalid-path missing-executable=exec-not-found ",
            "unsupported-domain=ENOTSUP unsupported-type=ENOTSUP unsupported-protocol=ENOTSUP ",
            "listen-before-bind=EINVAL invalid-bind-endpoint=EINVAL invalid-backlog=EINVAL ",
            "repeated-bind=EINVAL repeated-listen=ok-updates-backlog accept-before-connect=EAGAIN ",
            "missing-listener=EINVAL queue-backpressure=ENOSPC non-socket-descriptor=EBADF ",
            "empty-recv=EAGAIN send-invalid-flags=EINVAL recv-invalid-flags=EINVAL ",
            "payload-queue-backpressure=ENOSPC send-after-peer-close=EPIPE ",
            "poll-unsupported-events=EINVAL poll-invalid-descriptor=ERROR poll-non-socket-descriptor=ERROR ",
            "poll-wait-scalar-dispatch=ENOTSUP poll-wait-invalid-timeout=EINVAL ",
            "poll-wait-unsupported-events=EINVAL ",
            "invalid-closed-descriptor=EBADF syscall-vocabulary=SyscallNumber/STABLE_SVC_IMMEDIATE/",
            "TALOS_SOCKET/TALOS_BIND/TALOS_LISTEN/TALOS_CONNECT/TALOS_ACCEPT/TALOS_SEND/",
            "TALOS_RECV/TALOS_POLL/TALOS_POLL_WAIT/TALOS_CLOSE bounded source=shell-sockdiag-controls\n"
        )));
        assert!(output.contains(
            "talos: exec-lifecycle pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/sockdiag state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/sockdiag state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/sockdiag state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains("talos> talos: exec-invalid-path\n"));
        assert!(output.contains("talos> talos: exec-not-found\n"));
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
    fn local_command_loop_redirects_direct_absolute_stdout_to_volatile_regular_file() {
        let bytes = *b"/bin/stdout >/tmp/stdout.txt\rwaitpid\rlaststatus\rcat /tmp/stdout.txt\r/bin/stdout\r";
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

        assert_eq!(redirected.line(), b"/bin/stdout >/tmp/stdout.txt");
        assert_eq!(redirected.status(), LocalCommandStatus::Handled);
        assert_eq!(redirected.response_lines(), 11);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.line(), b"laststatus");
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert_eq!(readback.line(), b"cat /tmp/stdout.txt");
        assert_eq!(readback.status(), LocalCommandStatus::Handled);
        assert_eq!(readback.response_lines(), 2);
        assert_eq!(normal.line(), b"/bin/stdout");
        assert_eq!(normal.status(), LocalCommandStatus::Handled);
        assert_eq!(normal.response_lines(), 10);
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
    fn local_command_loop_redirects_bare_name_stdout_to_volatile_regular_file() {
        let bytes = *b"stdout >/tmp/stdout.txt\rwaitpid\rlaststatus\rcat /tmp/stdout.txt\rstdout\r";
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

        assert_eq!(redirected.line(), b"stdout >/tmp/stdout.txt");
        assert_eq!(redirected.status(), LocalCommandStatus::Handled);
        assert_eq!(redirected.response_lines(), 11);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.line(), b"laststatus");
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert_eq!(readback.line(), b"cat /tmp/stdout.txt");
        assert_eq!(readback.status(), LocalCommandStatus::Handled);
        assert_eq!(readback.response_lines(), 2);
        assert_eq!(normal.line(), b"stdout");
        assert_eq!(normal.status(), LocalCommandStatus::Handled);
        assert_eq!(normal.response_lines(), 10);
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
    fn local_command_loop_appends_bare_name_stdout_to_existing_volatile_regular_file() {
        let bytes = *b"stdout >/tmp/stdout.txt\rstdout >>/tmp/stdout.txt\rwaitpid\rlaststatus\rcat /tmp/stdout.txt\rstdout\r";
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

        assert_eq!(created.line(), b"stdout >/tmp/stdout.txt");
        assert_eq!(created.status(), LocalCommandStatus::Handled);
        assert_eq!(appended.line(), b"stdout >>/tmp/stdout.txt");
        assert_eq!(appended.status(), LocalCommandStatus::Handled);
        assert_eq!(appended.response_lines(), 11);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.line(), b"laststatus");
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert_eq!(readback.line(), b"cat /tmp/stdout.txt");
        assert_eq!(readback.status(), LocalCommandStatus::Handled);
        assert_eq!(normal.line(), b"stdout");
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
    fn local_command_loop_rejects_unaccepted_direct_stdout_output_redirection_forms() {
        let bytes = *b"/bin/stderr 2>/tmp/stdout.txt\r/bin/stdout >>/var/other.txt\r/bin/stderr 2>>/tmp/other.txt\r/bin/stdout | /bin/stdin >/tmp/stdout.txt\r/bin/stdin </etc/banner.txt >/tmp/stdout.txt\rcat /etc/banner.txt >/tmp/stdout.txt\rwaitpid\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (
            stderr_file,
            append,
            unsupported_path,
            pipeline_output,
            combined_io,
            kernel_backed_cat,
            waited,
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
            )
        };
        let output = backend.as_str();

        for rejected in [
            stderr_file,
            append,
            unsupported_path,
            pipeline_output,
            combined_io,
            kernel_backed_cat,
        ] {
            assert_eq!(rejected.status(), LocalCommandStatus::UnexpectedArgument);
            assert_eq!(rejected.response_lines(), 1);
        }
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert!(output.contains("talos: waitpid no-child source=lifecycle-record\n"));
        assert_eq!(output.matches("talos: exec-invalid-path").count(), 5);
        assert_eq!(output.matches("talos: unexpected-argument").count(), 1);
        assert_eq!(output.matches("path=/bin/stdout state=exited").count(), 0);
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
    fn local_command_loop_appends_direct_absolute_stdout_to_existing_volatile_regular_file() {
        let bytes = *b"/bin/stdout >/tmp/stdout.txt\r/bin/stdout >>/tmp/stdout.txt\rwaitpid\rlaststatus\rcat /tmp/stdout.txt\r/bin/stdout\r";
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

        assert_eq!(created.line(), b"/bin/stdout >/tmp/stdout.txt");
        assert_eq!(created.status(), LocalCommandStatus::Handled);
        assert_eq!(appended.line(), b"/bin/stdout >>/tmp/stdout.txt");
        assert_eq!(appended.status(), LocalCommandStatus::Handled);
        assert_eq!(appended.response_lines(), 11);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.line(), b"laststatus");
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert_eq!(readback.line(), b"cat /tmp/stdout.txt");
        assert_eq!(readback.status(), LocalCommandStatus::Handled);
        assert_eq!(normal.line(), b"/bin/stdout");
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
        let bytes = *b"/bin/stderr 2>/tmp/stderr.txt\rwaitpid\rlaststatus\rcat /tmp/stderr.txt\r/bin/stderr\rexec stdout\r";
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

        assert_eq!(redirected.line(), b"/bin/stderr 2>/tmp/stderr.txt");
        assert_eq!(redirected.status(), LocalCommandStatus::Handled);
        assert_eq!(redirected.response_lines(), 11);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.line(), b"laststatus");
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert_eq!(readback.line(), b"cat /tmp/stderr.txt");
        assert_eq!(readback.status(), LocalCommandStatus::Handled);
        assert_eq!(readback.response_lines(), 2);
        assert_eq!(normal_stderr.line(), b"/bin/stderr");
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
    fn local_command_loop_redirects_bare_name_stderr_to_volatile_regular_file() {
        let bytes = *b"stderr 2>/tmp/stderr.txt\rwaitpid\rlaststatus\rcat /tmp/stderr.txt\rstderr\rexec stdout\r";
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

        assert_eq!(redirected.line(), b"stderr 2>/tmp/stderr.txt");
        assert_eq!(redirected.status(), LocalCommandStatus::Handled);
        assert_eq!(redirected.response_lines(), 11);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.line(), b"laststatus");
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert_eq!(readback.line(), b"cat /tmp/stderr.txt");
        assert_eq!(readback.status(), LocalCommandStatus::Handled);
        assert_eq!(readback.response_lines(), 2);
        assert_eq!(normal_stderr.line(), b"stderr");
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
            "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stderr"
        ));
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
    fn local_command_loop_rejects_unaccepted_direct_stderr_output_redirection_forms() {
        let bytes = *b"/bin/stderr 2> /tmp/stderr.txt\r/bin/stderr 2>>/tmp/other.txt\rstderr 2>>/tmp/other.txt\r/bin/stderr 2>/var/err.txt\r/bin/stderr | /bin/stdin 2>/tmp/stderr.txt\r/bin/stdin </etc/banner.txt 2>/tmp/stderr.txt\rcat /etc/banner.txt 2>/tmp/stderr.txt\rwaitpid\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (
            separated,
            arbitrary_append,
            bare_name_append,
            unsupported_path,
            pipeline_output,
            combined_io,
            kernel_backed_cat,
            waited,
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

        for rejected in [
            separated,
            arbitrary_append,
            bare_name_append,
            unsupported_path,
            pipeline_output,
            combined_io,
            kernel_backed_cat,
        ] {
            assert_eq!(rejected.status(), LocalCommandStatus::UnexpectedArgument);
            assert_eq!(rejected.response_lines(), 1);
        }
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert!(output.contains("talos: waitpid no-child source=lifecycle-record\n"));
        assert_eq!(output.matches("talos: exec-invalid-path").count(), 6);
        assert_eq!(output.matches("talos: unexpected-argument").count(), 1);
        assert_eq!(output.matches("path=/bin/stderr state=exited").count(), 0);
    }

    #[test_case]
    fn local_command_loop_appends_direct_child_stderr_to_existing_volatile_regular_file() {
        let bytes = *b"/bin/stderr 2>/tmp/stderr.txt\r/bin/stderr 2>>/tmp/stderr.txt\rwaitpid\rlaststatus\rcat /tmp/stderr.txt\r/bin/stderr\rexec stdout\r";
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

        assert_eq!(created.line(), b"/bin/stderr 2>/tmp/stderr.txt");
        assert_eq!(created.status(), LocalCommandStatus::Handled);
        assert_eq!(appended.line(), b"/bin/stderr 2>>/tmp/stderr.txt");
        assert_eq!(appended.status(), LocalCommandStatus::Handled);
        assert_eq!(appended.response_lines(), 11);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.line(), b"laststatus");
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert_eq!(readback.line(), b"cat /tmp/stderr.txt");
        assert_eq!(readback.status(), LocalCommandStatus::Handled);
        assert_eq!(normal_stderr.line(), b"/bin/stderr");
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
    fn local_command_loop_appends_bare_name_child_stderr_to_existing_volatile_regular_file() {
        let bytes = *b"stderr 2>/tmp/stderr.txt\rstderr 2>>/tmp/stderr.txt\rwaitpid\rlaststatus\rcat /tmp/stderr.txt\rstderr\rexec stdout\r";
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

        assert_eq!(created.line(), b"stderr 2>/tmp/stderr.txt");
        assert_eq!(created.status(), LocalCommandStatus::Handled);
        assert_eq!(appended.line(), b"stderr 2>>/tmp/stderr.txt");
        assert_eq!(appended.status(), LocalCommandStatus::Handled);
        assert_eq!(appended.response_lines(), 11);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.line(), b"laststatus");
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert_eq!(readback.line(), b"cat /tmp/stderr.txt");
        assert_eq!(readback.status(), LocalCommandStatus::Handled);
        assert_eq!(normal_stderr.line(), b"stderr");
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
        assert!(output.contains("talos: exec path=/bin/stderr source=vfs-open-read\n"));
        assert!(output.contains(
            "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stderr"
        ));
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
        let (pipeline, waited, observed, process_table_records) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            let pipeline = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let process_table_records = io.process_table_records();
            (
                pipeline,
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                process_table_records,
            )
        };
        let output = backend.as_str();

        assert_eq!(pipeline.line(), b"exec stdout | exec stdin");
        assert_eq!(pipeline.status(), LocalCommandStatus::Handled);
        assert_eq!(pipeline.response_lines(), 22);
        let producer_record = process_table_records[0].unwrap();
        let consumer_record = process_table_records[1].unwrap();
        assert_eq!(
            producer_record.identity,
            LocalCommandProcessTableRecord::IDENTITY
        );
        assert_eq!(producer_record.slot, 0);
        assert_eq!(
            producer_record.capacity,
            LOCAL_COMMAND_PROCESS_TABLE_CAPACITY
        );
        assert_eq!(
            producer_record.lifecycle.process_id,
            LOCAL_COMMAND_PIPELINE_PRODUCER_PROCESS_ID
        );
        assert_eq!(
            producer_record.lifecycle.source_path,
            initramfs::PHASE10_STDOUT_PATH
        );
        assert_eq!(
            consumer_record.identity,
            LocalCommandProcessTableRecord::IDENTITY
        );
        assert_eq!(consumer_record.slot, 1);
        assert_eq!(
            consumer_record.capacity,
            LOCAL_COMMAND_PROCESS_TABLE_CAPACITY
        );
        assert_eq!(
            consumer_record.lifecycle.process_id,
            LOCAL_COMMAND_PIPELINE_CONSUMER_PROCESS_ID
        );
        assert_eq!(
            consumer_record.lifecycle.source_path,
            initramfs::PHASE10_STDIN_PATH
        );
        assert_eq!(process_table_records[2], None);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.line(), b"laststatus");
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert!(output.contains(
            "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdout consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000001f bytes-read=0x000000000000001f writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-stdout-to-stdin\n"
        ));
        assert!(output.contains(
            "talos: pipeline-lifecycle-status record=phase12-local-pipeline-distinct-process-lifecycle-status-record-v1 pipeline=0x0000000000000001 producer-pid=0x0000000000100001 producer-path=/bin/stdout producer-state=exited producer-status=0x0000000000000000 producer-observed-status=0x0000000000000000 producer-reaped=true consumer-pid=0x0000000000100002 consumer-path=/bin/stdin consumer-state=exited consumer-status=0x0000000000000000 consumer-observed-status=0x0000000000000000 consumer-reaped=true source=kernel-owned-pipeline-lifecycle-status-record\n"
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
            "talos> talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
    }

    #[test_case]
    fn local_command_loop_waitpid_observes_pipeline_records_by_explicit_pid() {
        let bytes = *b"waitpid 0x100001\rwaitpid bogus\rwaitpid 0x0\rexec stdout | exec stdin\rwaitpid 0x100001\rwaitpid 0x100002\rwaitpid 0x100001\rwaitpid\rlaststatus\rcat /etc/banner.txt\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (
            initial_no_child,
            malformed,
            unsupported,
            pipeline,
            producer_wait,
            consumer_wait,
            stale_wait,
            consumed_wait,
            observed,
            cat,
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

        assert_eq!(initial_no_child.line(), b"waitpid 0x100001");
        assert_eq!(initial_no_child.status(), LocalCommandStatus::Handled);
        assert_eq!(malformed.line(), b"waitpid bogus");
        assert_eq!(malformed.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(unsupported.line(), b"waitpid 0x0");
        assert_eq!(unsupported.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(pipeline.line(), b"exec stdout | exec stdin");
        assert_eq!(pipeline.status(), LocalCommandStatus::Handled);
        assert_eq!(producer_wait.line(), b"waitpid 0x100001");
        assert_eq!(producer_wait.status(), LocalCommandStatus::Handled);
        assert_eq!(consumer_wait.line(), b"waitpid 0x100002");
        assert_eq!(consumer_wait.status(), LocalCommandStatus::Handled);
        assert_eq!(stale_wait.line(), b"waitpid 0x100001");
        assert_eq!(stale_wait.status(), LocalCommandStatus::Handled);
        assert_eq!(consumed_wait.line(), b"waitpid");
        assert_eq!(consumed_wait.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.line(), b"laststatus");
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert_eq!(cat.line(), b"cat /etc/banner.txt");
        assert_eq!(cat.status(), LocalCommandStatus::Handled);
        assert!(output.contains(
            "talos> talos: waitpid no-child pid=0x0000000000100001 source=explicit-pid-lifecycle-record\n"
        ));
        assert!(
            output.contains(
                "talos> talos: waitpid invalid-pid source=explicit-pid-lifecycle-record\n"
            )
        );
        assert!(output.contains(
            "talos> talos: waitpid unsupported-pid pid=0x0000000000000000 source=explicit-pid-lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record\n"
        ));
        assert!(output.contains("talos> talos: waitpid no-child source=lifecycle-record\n"));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains("Talos initramfs fixture\n"));
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
        assert_eq!(pipeline.response_lines(), 22);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.line(), b"laststatus");
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert!(output.contains(
            "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stderr consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x0000000000000000 bytes-read=0x0000000000000000 writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-stdout-only-stderr-not-piped\n"
        ));
        assert!(output.contains(
            "talos: pipeline-lifecycle-status record=phase12-local-pipeline-distinct-process-lifecycle-status-record-v1 pipeline=0x0000000000000001 producer-pid=0x0000000000100001 producer-path=/bin/stderr producer-state=exited producer-status=0x0000000000000000 producer-observed-status=0x0000000000000000 producer-reaped=true consumer-pid=0x0000000000100002 consumer-path=/bin/stdin consumer-state=exited consumer-status=0x0000000000000000 consumer-observed-status=0x0000000000000000 consumer-reaped=true source=kernel-owned-pipeline-lifecycle-status-record\n"
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
            "talos> talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
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
        assert_eq!(mixed.response_lines(), 23);
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
            "talos: pipeline-lifecycle-status record=phase12-local-pipeline-distinct-process-lifecycle-status-record-v1 pipeline=0x0000000000000001 producer-pid=0x0000000000100001 producer-path=/bin/stderr producer-state=exited producer-status=0x0000000000000000 producer-observed-status=0x0000000000000000 producer-reaped=true consumer-pid=0x0000000000100002 consumer-path=/bin/stdin consumer-state=exited consumer-status=0x0000000000000000 consumer-observed-status=0x0000000000000000 consumer-reaped=true source=kernel-owned-pipeline-lifecycle-status-record\n"
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
            "talos> talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
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
        assert_eq!(mixed.response_lines(), 23);
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
            "talos: pipeline-lifecycle-status record=phase12-local-pipeline-distinct-process-lifecycle-status-record-v1 pipeline=0x0000000000000001 producer-pid=0x0000000000100001 producer-path=/bin/stdout producer-state=exited producer-status=0x0000000000000000 producer-observed-status=0x0000000000000000 producer-reaped=true consumer-pid=0x0000000000100002 consumer-path=/bin/stdin consumer-state=exited consumer-status=0x0000000000000000 consumer-observed-status=0x0000000000000000 consumer-reaped=true source=kernel-owned-pipeline-lifecycle-status-record\n"
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
            "talos> talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
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
        assert_eq!(redirected.response_lines(), 23);
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
            "talos: pipeline-lifecycle-status record=phase12-local-pipeline-distinct-process-lifecycle-status-record-v1 pipeline=0x0000000000000001 producer-pid=0x0000000000100001 producer-path=/bin/stdout producer-state=exited producer-status=0x0000000000000000 producer-observed-status=0x0000000000000000 producer-reaped=true consumer-pid=0x0000000000100002 consumer-path=/bin/stdin consumer-state=exited consumer-status=0x0000000000000000 consumer-observed-status=0x0000000000000000 consumer-reaped=true source=kernel-owned-pipeline-lifecycle-status-record\n"
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
            "talos> talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
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
        assert_eq!(redirected.response_lines(), 23);
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
            "talos: pipeline-lifecycle-status record=phase12-local-pipeline-distinct-process-lifecycle-status-record-v1 pipeline=0x0000000000000001 producer-pid=0x0000000000100001 producer-path=/bin/stdout producer-state=exited producer-status=0x0000000000000000 producer-observed-status=0x0000000000000000 producer-reaped=true consumer-pid=0x0000000000100002 consumer-path=/bin/stdin consumer-state=exited consumer-status=0x0000000000000000 consumer-observed-status=0x0000000000000000 consumer-reaped=true source=kernel-owned-pipeline-lifecycle-status-record\n"
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
            "talos> talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
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
        assert_eq!(exec.response_lines(), 10);
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
            "talos: vfs-exec-lifecycle-status record=phase12-local-vfs-exec-lifecycle-status-record-v2 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=kernel-owned-vfs-exec-lifecycle-status-record\n"
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
        assert_eq!(exec.response_lines(), 10);
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
            "talos: vfs-exec-lifecycle-status record=phase12-local-vfs-exec-lifecycle-status-record-v2 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=kernel-owned-vfs-exec-lifecycle-status-record\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=lifecycle-record\n"
        ));
    }

    #[test_case]
    fn local_command_loop_execs_generated_root_executable_with_literal_argv() {
        let input =
            ScriptedInput::new(*b"exec /generated/status7 alpha\rwaitpid\rlaststatus\r", 49);
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

        assert_eq!(exec.line(), b"exec /generated/status7 alpha");
        assert_eq!(exec.status(), LocalCommandStatus::Handled);
        assert_eq!(exec.response_lines(), 9);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.line(), b"laststatus");
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert!(
            output.contains("talos> talos: exec path=/generated/status7 source=vfs-open-read\n")
        );
        assert!(output.contains(
            "talos: exec-startup-abi state=literal-argv-absolute-empty-envp argc=0x0000000000000002 argv0=/generated/status7 argv1=alpha"
        ));
        assert!(output.contains(
            "talos: exec-lifecycle pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/generated/status7 state=exited status=0x0000000000000007 observed-status=0x0000000000000007 reaped=true\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/generated/status7 state=exited status=0x0000000000000007 observed-status=0x0000000000000007 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/generated/status7 state=exited status=0x0000000000000007 observed-status=0x0000000000000007 reaped=true source=lifecycle-record\n"
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
        assert_eq!(exec.response_lines(), 10);
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
            "talos: vfs-exec-lifecycle-status record=phase12-local-vfs-exec-lifecycle-status-record-v2 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=kernel-owned-vfs-exec-lifecycle-status-record\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=lifecycle-record\n"
        ));
    }

    #[test_case]
    fn local_command_loop_jobs_reports_background_accounting_record() {
        let bytes = *b"jobs\rexec /bin/status42 &\rjobs\rjobs\rwaitpid\rlaststatus\rfg\rbg\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (
            empty,
            background,
            background_process_table_records,
            running,
            completed,
            waited,
            observed,
            fg,
            bg,
        ) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            let empty = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let background = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let background_process_table_records = io.process_table_records();
            (
                empty,
                background,
                background_process_table_records,
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
            )
        };
        let output = backend.as_str();

        assert_eq!(empty.line(), b"jobs");
        assert_eq!(empty.status(), LocalCommandStatus::Handled);
        assert_eq!(empty.response_lines(), 1);
        assert_eq!(background.line(), b"exec /bin/status42 &");
        assert_eq!(background.status(), LocalCommandStatus::Handled);
        assert_eq!(background.response_lines(), 8);
        let background_process_record = background_process_table_records[0].unwrap();
        assert_eq!(
            background_process_record.identity,
            LocalCommandProcessTableRecord::IDENTITY
        );
        assert_eq!(background_process_record.slot, 0);
        assert_eq!(
            background_process_record.lifecycle.process_id,
            LOCAL_COMMAND_EXEC_PROCESS_ID
        );
        assert_eq!(
            background_process_record.lifecycle.source_path,
            initramfs::PHASE10_STATUS42_PATH
        );
        assert_eq!(background_process_record.lifecycle.status, 0x2a);
        assert_eq!(background_process_table_records[1], None);
        assert_eq!(running.line(), b"jobs");
        assert_eq!(running.status(), LocalCommandStatus::Handled);
        assert_eq!(running.response_lines(), 1);
        assert_eq!(completed.line(), b"jobs");
        assert_eq!(completed.status(), LocalCommandStatus::Handled);
        assert_eq!(completed.response_lines(), 1);
        assert_eq!(waited.line(), b"waitpid");
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.line(), b"laststatus");
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert_eq!(fg.status(), LocalCommandStatus::UnknownCommand);
        assert_eq!(bg.status(), LocalCommandStatus::UnknownCommand);
        assert!(output.contains("talos> talos: jobs none source=background-vfs-exec-accounting\n"));
        assert!(output.contains(
            "talos: background-job id=0x0000000000000001 pid=0x0000000000100001 command=/bin/status42 state=running reaped=false status=pending shell-responsive=true source=background-vfs-exec-accounting\n"
        ));
        assert!(output.contains(
            "talos> talos: jobs id=0x0000000000000001 pid=0x0000000000100001 command=/bin/status42 state=running status=pending reaped=false source=background-vfs-exec-accounting\n"
        ));
        assert!(output.contains(
            "talos> talos: jobs id=0x0000000000000001 pid=0x0000000000100001 command=/bin/status42 state=completed status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=background-vfs-exec-accounting\n"
        ));
        assert!(output.contains("talos> talos: waitpid no-child source=lifecycle-record\n"));
        assert!(output.contains("talos> talos: last-process none\n"));
        assert_eq!(output.matches("talos: jobs id=").count(), 2);
        assert_eq!(output.matches("talos: unknown-command\n").count(), 2);
    }

    #[test_case]
    fn local_command_loop_records_multiple_background_vfs_exec_jobs() {
        let bytes = *b"jobs\rexec /bin/status42 &\rexec /bin/zero &\rjobs\rjobs\rjobs\rwaitpid\rlaststatus\rexec /bin/zero\rwaitpid\rlaststatus\rexec /bin/status42&\rexec stdout &\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (
            empty,
            first_background,
            second_background,
            two_background_process_table_records,
            mixed_jobs,
            completed_jobs,
            stale_cleared_jobs,
            empty_wait,
            empty_last,
            foreground,
            foreground_wait,
            foreground_last,
            malformed_background,
            unsupported_background,
        ) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            let empty = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let first_background = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let second_background = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let two_background_process_table_records = io.process_table_records();
            (
                empty,
                first_background,
                second_background,
                two_background_process_table_records,
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

        assert_eq!(empty.line(), b"jobs");
        assert_eq!(empty.status(), LocalCommandStatus::Handled);
        assert_eq!(empty.response_lines(), 1);
        assert_eq!(first_background.line(), b"exec /bin/status42 &");
        assert_eq!(first_background.status(), LocalCommandStatus::Handled);
        assert_eq!(first_background.response_lines(), 8);
        assert_eq!(second_background.line(), b"exec /bin/zero &");
        assert_eq!(second_background.status(), LocalCommandStatus::Handled);
        assert_eq!(second_background.response_lines(), 9);
        let first_background_record = two_background_process_table_records[0].unwrap();
        let second_background_record = two_background_process_table_records[1].unwrap();
        assert_eq!(
            first_background_record.identity,
            LocalCommandProcessTableRecord::IDENTITY
        );
        assert_eq!(first_background_record.slot, 0);
        assert_eq!(
            first_background_record.lifecycle.source_path,
            initramfs::PHASE10_STATUS42_PATH
        );
        assert_eq!(first_background_record.lifecycle.status, 0x2a);
        assert_eq!(
            second_background_record.identity,
            LocalCommandProcessTableRecord::IDENTITY
        );
        assert_eq!(second_background_record.slot, 1);
        assert_eq!(
            second_background_record.lifecycle.process_id,
            LOCAL_COMMAND_EXEC_PROCESS_ID + 1
        );
        assert_eq!(
            second_background_record.lifecycle.source_path,
            initramfs::PHASE10_ZERO_PATH
        );
        assert_eq!(second_background_record.lifecycle.status, 0);
        assert_eq!(two_background_process_table_records[2], None);
        assert_eq!(mixed_jobs.line(), b"jobs");
        assert_eq!(mixed_jobs.status(), LocalCommandStatus::Handled);
        assert_eq!(mixed_jobs.response_lines(), 2);
        assert_eq!(completed_jobs.line(), b"jobs");
        assert_eq!(completed_jobs.status(), LocalCommandStatus::Handled);
        assert_eq!(completed_jobs.response_lines(), 1);
        assert_eq!(stale_cleared_jobs.line(), b"jobs");
        assert_eq!(stale_cleared_jobs.status(), LocalCommandStatus::Handled);
        assert_eq!(stale_cleared_jobs.response_lines(), 1);
        assert_eq!(empty_wait.line(), b"waitpid");
        assert_eq!(empty_wait.status(), LocalCommandStatus::Handled);
        assert_eq!(empty_last.line(), b"laststatus");
        assert_eq!(empty_last.status(), LocalCommandStatus::Handled);
        assert_eq!(foreground.line(), b"exec /bin/zero");
        assert_eq!(foreground.status(), LocalCommandStatus::Handled);
        assert_eq!(foreground.response_lines(), 10);
        assert_eq!(foreground_wait.line(), b"waitpid");
        assert_eq!(foreground_wait.status(), LocalCommandStatus::Handled);
        assert_eq!(foreground_last.line(), b"laststatus");
        assert_eq!(foreground_last.status(), LocalCommandStatus::Handled);
        assert_eq!(
            malformed_background.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert_eq!(
            unsupported_background.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert!(output.contains("talos> talos: jobs none source=background-vfs-exec-accounting\n"));
        assert!(output.contains(
            "talos: background-job id=0x0000000000000001 pid=0x0000000000100001 command=/bin/status42 state=running reaped=false status=pending shell-responsive=true source=background-vfs-exec-accounting\n"
        ));
        assert!(output.contains(
            "talos> talos: background-job id=0x0000000000000001 pid=0x0000000000100001 command=/bin/status42 state=completed status=0x000000000000002a observed-status=0x000000000000002a reaped=true shell-responsive=observed source=background-vfs-exec-accounting\n"
        ));
        assert!(output.contains(
            "talos: background-job id=0x0000000000000002 pid=0x0000000000100002 command=/bin/zero state=running reaped=false status=pending shell-responsive=true source=background-vfs-exec-accounting\n"
        ));
        assert!(output.contains(
            "talos> talos: jobs id=0x0000000000000001 pid=0x0000000000100001 command=/bin/status42 state=completed status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=background-vfs-exec-accounting\n"
        ));
        assert!(output.contains(
            "talos: jobs id=0x0000000000000002 pid=0x0000000000100002 command=/bin/zero state=running status=pending reaped=false source=background-vfs-exec-accounting\n"
        ));
        assert!(output.contains(
            "talos: jobs id=0x0000000000000002 pid=0x0000000000100002 command=/bin/zero state=completed status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=background-vfs-exec-accounting\n"
        ));
        assert_eq!(
            output
                .matches("talos> talos: jobs none source=background-vfs-exec-accounting\n")
                .count(),
            2
        );
        assert!(output.contains("talos> talos: waitpid no-child source=lifecycle-record\n"));
        assert!(output.contains("talos> talos: last-process none\n"));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/zero state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/zero state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert_eq!(output.matches("talos: jobs id=").count(), 3);
        assert_eq!(output.matches("talos: exec-invalid-path\n").count(), 2);
    }

    #[test_case]
    fn local_command_loop_waitpid_consumes_completed_background_job_by_pid() {
        let bytes = *b"exec /bin/status42 &\rwaitpid 0x100001\rwaitpid 0x100001\rjobs\rexec /bin/zero &\rwaitpid 0x100002\rwaitpid 0x100002\rjobs\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (
            status_background,
            status_wait,
            status_stale,
            status_jobs,
            zero_background,
            zero_wait,
            zero_stale,
            zero_jobs,
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

        assert_eq!(status_background.line(), b"exec /bin/status42 &");
        assert_eq!(status_background.status(), LocalCommandStatus::Handled);
        assert_eq!(status_wait.line(), b"waitpid 0x100001");
        assert_eq!(status_wait.status(), LocalCommandStatus::Handled);
        assert_eq!(status_wait.response_lines(), 2);
        assert_eq!(status_stale.line(), b"waitpid 0x100001");
        assert_eq!(status_stale.status(), LocalCommandStatus::Handled);
        assert_eq!(status_jobs.line(), b"jobs");
        assert_eq!(status_jobs.status(), LocalCommandStatus::Handled);
        assert_eq!(zero_background.line(), b"exec /bin/zero &");
        assert_eq!(zero_background.status(), LocalCommandStatus::Handled);
        assert_eq!(zero_wait.line(), b"waitpid 0x100002");
        assert_eq!(zero_wait.status(), LocalCommandStatus::Handled);
        assert_eq!(zero_wait.response_lines(), 2);
        assert_eq!(zero_stale.line(), b"waitpid 0x100002");
        assert_eq!(zero_stale.status(), LocalCommandStatus::Handled);
        assert_eq!(zero_jobs.line(), b"jobs");
        assert_eq!(zero_jobs.status(), LocalCommandStatus::Handled);
        assert!(output.contains(
            "talos> talos: background-job id=0x0000000000000001 pid=0x0000000000100001 command=/bin/status42 state=completed status=0x000000000000002a observed-status=0x000000000000002a reaped=true shell-responsive=observed source=background-vfs-exec-accounting\n"
        ));
        assert!(output.contains(
            "talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=background-job-lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid no-child pid=0x0000000000100001 source=explicit-pid-lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: background-job id=0x0000000000000002 pid=0x0000000000100002 command=/bin/zero state=completed status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true shell-responsive=observed source=background-vfs-exec-accounting\n"
        ));
        assert!(output.contains(
            "talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/zero state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=background-job-lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid no-child pid=0x0000000000100002 source=explicit-pid-lifecycle-record\n"
        ));
        assert_eq!(
            output
                .matches("talos> talos: jobs none source=background-vfs-exec-accounting\n")
                .count(),
            2
        );
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
        assert_eq!(exec.response_lines(), 11);
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
            "talos: init-lifecycle-status record=phase12-local-process-lifecycle-status-record-v1 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/init state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=kernel-owned-lifecycle-status-record\n"
        ));
        assert!(output.contains(
            "talos: vfs-exec-lifecycle-status record=phase12-local-vfs-exec-lifecycle-status-record-v2 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/init state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=kernel-owned-vfs-exec-lifecycle-status-record\n"
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
    fn local_command_loop_records_direct_vfs_exec_process_table_entries() {
        let bytes = *b"exec /bin/init\rwaitpid\rlaststatus\rexec /bin/zero\rwaitpid\rlaststatus\rexec /bin/status42\rwaitpid\rlaststatus\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (
            init_exec,
            init_wait,
            init_last,
            zero_exec,
            zero_wait,
            zero_last,
            status_exec,
            status_wait,
            status_last,
        ) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            let init_exec = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let init_record = io.process_table_records()[0].unwrap();
            assert_eq!(
                init_record.identity,
                LocalCommandProcessTableRecord::IDENTITY
            );
            assert_eq!(init_record.slot, 0);
            assert_eq!(init_record.capacity, LOCAL_COMMAND_PROCESS_TABLE_CAPACITY);
            assert_eq!(
                init_record.lifecycle.process_id,
                LOCAL_COMMAND_EXEC_PROCESS_ID
            );
            assert_eq!(init_record.lifecycle.parent_owner_id, 1);
            assert_eq!(
                init_record.lifecycle.source_path,
                initramfs::PHASE8_INIT_PATH
            );
            assert_eq!(init_record.lifecycle.status, 0);
            assert_eq!(init_record.lifecycle.observed_status, 0);
            assert_eq!(
                init_record.lifecycle.state,
                LocalCommandProcessState::Exited
            );
            assert!(init_record.lifecycle.reaped);

            let init_wait = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert!(io.waitable_process.is_none());
            let init_last = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records()[0], Some(init_record));

            let zero_exec = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let zero_record = io.process_table_records()[0].unwrap();
            assert_eq!(
                zero_record.lifecycle.process_id,
                LOCAL_COMMAND_EXEC_PROCESS_ID
            );
            assert_eq!(zero_record.lifecycle.parent_owner_id, 1);
            assert_eq!(
                zero_record.lifecycle.source_path,
                initramfs::PHASE10_ZERO_PATH
            );
            assert_eq!(zero_record.lifecycle.status, 0);
            assert_eq!(zero_record.lifecycle.observed_status, 0);
            assert_eq!(
                zero_record.lifecycle.state,
                LocalCommandProcessState::Exited
            );
            assert!(zero_record.lifecycle.reaped);
            let zero_wait = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert!(io.waitable_process.is_none());
            let zero_last = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records()[0], Some(zero_record));

            let status_exec = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let status_record = io.process_table_records()[0].unwrap();
            assert_eq!(
                status_record.lifecycle.process_id,
                LOCAL_COMMAND_EXEC_PROCESS_ID
            );
            assert_eq!(status_record.lifecycle.parent_owner_id, 1);
            assert_eq!(
                status_record.lifecycle.source_path,
                initramfs::PHASE10_STATUS42_PATH
            );
            assert_eq!(status_record.lifecycle.status, 0x2a);
            assert_eq!(status_record.lifecycle.observed_status, 0x2a);
            assert_eq!(
                status_record.lifecycle.state,
                LocalCommandProcessState::Exited
            );
            assert!(status_record.lifecycle.reaped);
            let status_wait = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert!(io.waitable_process.is_none());
            let status_last = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records()[0], Some(status_record));

            (
                init_exec,
                init_wait,
                init_last,
                zero_exec,
                zero_wait,
                zero_last,
                status_exec,
                status_wait,
                status_last,
            )
        };
        let output = backend.as_str();

        assert_eq!(init_exec.status(), LocalCommandStatus::Handled);
        assert_eq!(init_wait.status(), LocalCommandStatus::Handled);
        assert_eq!(init_last.status(), LocalCommandStatus::Handled);
        assert_eq!(zero_exec.status(), LocalCommandStatus::Handled);
        assert_eq!(zero_wait.status(), LocalCommandStatus::Handled);
        assert_eq!(zero_last.status(), LocalCommandStatus::Handled);
        assert_eq!(status_exec.status(), LocalCommandStatus::Handled);
        assert_eq!(status_wait.status(), LocalCommandStatus::Handled);
        assert_eq!(status_last.status(), LocalCommandStatus::Handled);
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/init state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/zero state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=lifecycle-record\n"
        ));
    }

    #[test_case]
    fn local_command_loop_does_not_record_process_table_for_rejected_exec() {
        let bytes = *b"exec /missing\rexec /bin/status42 *\rexec /bin\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (missing, invalid_argv, directory) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            let missing = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(
                io.process_table_records(),
                [None; LOCAL_COMMAND_PROCESS_TABLE_CAPACITY]
            );
            let invalid_argv = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(
                io.process_table_records(),
                [None; LOCAL_COMMAND_PROCESS_TABLE_CAPACITY]
            );
            let directory = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(
                io.process_table_records(),
                [None; LOCAL_COMMAND_PROCESS_TABLE_CAPACITY]
            );
            (missing, invalid_argv, directory)
        };

        assert_eq!(missing.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(
            invalid_argv.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert_eq!(directory.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(
            backend.as_str(),
            "talos> talos: exec-not-found\n\
talos> talos: exec-invalid-path\n\
talos> talos: exec-not-executable\n"
        );
    }

    #[test_case]
    fn local_command_loop_runs_direct_absolute_path_vfs_command() {
        let bytes = *b"/bin/status42\rwaitpid\rlaststatus\rcat /proc/talos/processes\rps\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (exec, waited, observed, cat, ps) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            let exec = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let record = io.process_table_records()[0].unwrap();
            assert_eq!(
                record.lifecycle.source_path,
                initramfs::PHASE10_STATUS42_PATH
            );
            assert_eq!(record.lifecycle.status, 0x2a);
            assert_eq!(record.lifecycle.observed_status, 0x2a);
            let waited = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let observed = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let cat = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let ps = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records()[0], Some(record));
            (exec, waited, observed, cat, ps)
        };
        let output = backend.as_str();

        assert_eq!(exec.line(), b"/bin/status42");
        assert_eq!(exec.status(), LocalCommandStatus::Handled);
        assert_eq!(exec.response_lines(), 10);
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert_eq!(cat.status(), LocalCommandStatus::Handled);
        assert_eq!(ps.status(), LocalCommandStatus::Handled);
        assert!(output.contains("talos> talos: exec path=/bin/status42 source=vfs-open-read\n"));
        assert!(output.contains(
            "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/status42"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "slot=0 capacity=3 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true wait-consumed=true job-state=foreground source=bounded-process-table\n"
        ));
    }

    #[test_case]
    fn local_command_loop_runs_direct_absolute_path_vfs_command_with_literal_argv() {
        let bytes =
            *b"/bin/status42 alpha beta\rwaitpid\rlaststatus\rcat /proc/talos/processes\rps\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (exec, waited, observed, cat, ps) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            let exec = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let record = io.process_table_records()[0].unwrap();
            assert_eq!(
                record.lifecycle.source_path,
                initramfs::PHASE10_STATUS42_PATH
            );
            assert_eq!(record.lifecycle.status, 0x2a);
            assert_eq!(record.lifecycle.observed_status, 0x2a);
            let waited = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let observed = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let cat = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let ps = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records()[0], Some(record));
            (exec, waited, observed, cat, ps)
        };
        let output = backend.as_str();

        assert_eq!(exec.line(), b"/bin/status42 alpha beta");
        assert_eq!(exec.status(), LocalCommandStatus::Handled);
        assert_eq!(exec.response_lines(), 10);
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert_eq!(cat.status(), LocalCommandStatus::Handled);
        assert_eq!(ps.status(), LocalCommandStatus::Handled);
        assert!(output.contains("talos> talos: exec path=/bin/status42 source=vfs-open-read\n"));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-startup-abi state=literal-argv-absolute-empty-envp argc=0x0000000000000003 argv0=/bin/status42 argv1=alpha argv2=beta argv0-ptr=0x00007fffffffffe0 argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x00007fffffffffd8 copied-startup-bytes=0x0000000000000049 source=initial-user-stack-record\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "slot=0 capacity=3 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true wait-consumed=true job-state=foreground source=bounded-process-table\n"
        ));
    }

    #[test_case]
    fn local_command_loop_runs_direct_absolute_path_vfs_command_with_readonly_stdin_redirection() {
        let bytes = *b"/bin/stdin </etc/banner.txt\rwaitpid\rlaststatus\rpipestatus\rcat /proc/talos/processes\rps\r/bin/stdout </etc/banner.txt\r/bin/stdin </dev/null\r/bin/stdin < /etc/banner.txt\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (exec, waited, observed, status, cat, ps, non_stdin, dev_null, spaced) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            let exec = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let record = io.process_table_records()[0].unwrap();
            assert_eq!(record.lifecycle.source_path, initramfs::PHASE10_STDIN_PATH);
            assert_eq!(record.lifecycle.status, 0);
            assert_eq!(record.lifecycle.observed_status, 0);
            let waited = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let observed = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let status = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let cat = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let ps = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let non_stdin = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let dev_null = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let spaced = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records()[0], Some(record));
            assert_eq!(io.process_table_records()[1], None);
            (
                exec, waited, observed, status, cat, ps, non_stdin, dev_null, spaced,
            )
        };
        let output = backend.as_str();

        assert_eq!(exec.line(), b"/bin/stdin </etc/banner.txt");
        assert_eq!(exec.status(), LocalCommandStatus::Handled);
        assert_eq!(exec.response_lines(), 11);
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert_eq!(status.status(), LocalCommandStatus::Handled);
        assert_eq!(cat.status(), LocalCommandStatus::Handled);
        assert_eq!(ps.status(), LocalCommandStatus::Handled);
        assert_eq!(non_stdin.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(dev_null.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(spaced.status(), LocalCommandStatus::UnexpectedArgument);
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
            "talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(
            output
                .contains("talos: pipestatus none source=bounded-process-table-pipeline-status\n")
        );
        assert!(output.contains(
            "slot=0 capacity=3 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table\n"
        ));
        assert_eq!(output.matches("talos: exec-invalid-path\n").count(), 3);
    }

    #[test_case]
    fn local_command_loop_runs_direct_absolute_path_vfs_command_with_combined_stdin_stdout_redirection()
     {
        let bytes = *b"/bin/stdin </etc/banner.txt >/tmp/stdin-report.txt\rwaitpid\rlaststatus\rcat /tmp/stdin-report.txt\r/bin/stdin >/tmp/stdin-report.txt </etc/banner.txt\r/bin/stdin </dev/null >/tmp/stdin-report.txt\r/bin/stdin </etc/banner.txt 1>/tmp/stdin-report.txt\r/bin/stdin < /etc/banner.txt >/tmp/stdin-report.txt\r/bin/stdin </etc/banner.txt >>/tmp/stdin-report.txt\r/bin/stdin </etc/banner.txt 2>/tmp/stdin-report.txt\r/bin/stdin </etc/banner.txt >/tmp/other.txt\r/bin/stdin\rtalos-console0";
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
            append_output,
            stderr_output,
            arbitrary_output,
            normal,
        ) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            let redirected = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let record = io.process_table_records()[0].unwrap();
            assert_eq!(record.lifecycle.source_path, initramfs::PHASE10_STDIN_PATH);
            assert_eq!(record.lifecycle.status, 0);
            assert_eq!(record.lifecycle.observed_status, 0);
            let waited = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let observed = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let readback = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let output_first = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let dev_null_input = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let explicit_fd1 = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let spaced_input = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let append_output = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let stderr_output = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let arbitrary_output = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let normal = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records()[0], Some(record));
            assert_eq!(io.process_table_records()[1], None);
            (
                redirected,
                waited,
                observed,
                readback,
                output_first,
                dev_null_input,
                explicit_fd1,
                spaced_input,
                append_output,
                stderr_output,
                arbitrary_output,
                normal,
            )
        };
        let output = backend.as_str();

        assert_eq!(
            redirected.line(),
            b"/bin/stdin </etc/banner.txt >/tmp/stdin-report.txt"
        );
        assert_eq!(redirected.status(), LocalCommandStatus::Handled);
        assert_eq!(redirected.response_lines(), 12);
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert_eq!(readback.line(), b"cat /tmp/stdin-report.txt");
        assert_eq!(readback.status(), LocalCommandStatus::Handled);
        assert_eq!(readback.response_lines(), 2);
        assert_eq!(normal.line(), b"/bin/stdin");
        assert_eq!(normal.status(), LocalCommandStatus::Handled);
        assert_eq!(normal.response_lines(), 10);
        for rejected in [
            output_first,
            dev_null_input,
            explicit_fd1,
            spaced_input,
            append_output,
            stderr_output,
            arbitrary_output,
        ] {
            assert_eq!(rejected.status(), LocalCommandStatus::UnexpectedArgument);
            assert_eq!(rejected.response_lines(), 1);
        }
        assert!(output.contains("talos: exec path=/bin/stdin source=vfs-open-read\n"));
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
            "talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains("Talos userspace stdin fixture read: talos-console0\n"));
        assert!(output.contains(
            "talos: exec-stdin fd=0x0000000000000000 bytes=0x000000000000000e return=0x000000000000000e read-source=runtime-console0/local-input stdout-fd=0x0000000000000001 stdout-bytes=0x0000000000000033 stdout-return=0x0000000000000033 source=userspace-talos-read+userspace-talos-write\n"
        ));
        assert_eq!(output.matches("talos: exec-invalid-path\n").count(), 7);
    }

    #[test_case]
    fn local_command_loop_runs_bare_name_vfs_command_with_combined_stdin_stdout_redirection() {
        let bytes = *b"stdin </etc/banner.txt >/tmp/stdin-report.txt\rwaitpid\rlaststatus\rcat /tmp/stdin-report.txt\rstdin >/tmp/stdin-report.txt </etc/banner.txt\rstdin </dev/null >/tmp/stdin-report.txt\rstdin </etc/banner.txt 1>/tmp/stdin-report.txt\rstdin < /etc/banner.txt >/tmp/stdin-report.txt\rstdin </etc/banner.txt >>/tmp/stdin-report.txt\rstdin </etc/banner.txt 2>/tmp/stdin-report.txt\rstdout </etc/banner.txt >/tmp/stdin-report.txt\rstdin </etc/banner.txt >/tmp/other.txt\rstdin\rtalos-console0";
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
            append_output,
            stderr_output,
            unsupported_command,
            arbitrary_output,
            normal,
        ) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            let redirected = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let record = io.process_table_records()[0].unwrap();
            assert_eq!(record.lifecycle.source_path, initramfs::PHASE10_STDIN_PATH);
            assert_eq!(record.lifecycle.status, 0);
            assert_eq!(record.lifecycle.observed_status, 0);
            let waited = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let observed = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let readback = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let output_first = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let dev_null_input = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let explicit_fd1 = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let spaced_input = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let append_output = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let stderr_output = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let unsupported_command = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let arbitrary_output = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let normal = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records()[0], Some(record));
            assert_eq!(io.process_table_records()[1], None);
            (
                redirected,
                waited,
                observed,
                readback,
                output_first,
                dev_null_input,
                explicit_fd1,
                spaced_input,
                append_output,
                stderr_output,
                unsupported_command,
                arbitrary_output,
                normal,
            )
        };
        let output = backend.as_str();

        assert_eq!(
            redirected.line(),
            b"stdin </etc/banner.txt >/tmp/stdin-report.txt"
        );
        assert_eq!(redirected.status(), LocalCommandStatus::Handled);
        assert_eq!(redirected.response_lines(), 12);
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert_eq!(readback.line(), b"cat /tmp/stdin-report.txt");
        assert_eq!(readback.status(), LocalCommandStatus::Handled);
        assert_eq!(readback.response_lines(), 2);
        assert_eq!(normal.line(), b"stdin");
        assert_eq!(normal.status(), LocalCommandStatus::Handled);
        assert_eq!(normal.response_lines(), 10);
        for rejected in [
            output_first,
            dev_null_input,
            explicit_fd1,
            spaced_input,
            append_output,
            stderr_output,
            unsupported_command,
            arbitrary_output,
        ] {
            assert_eq!(rejected.status(), LocalCommandStatus::UnexpectedArgument);
            assert_eq!(rejected.response_lines(), 1);
        }
        assert!(output.contains("talos: exec path=/bin/stdin source=vfs-open-read\n"));
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
            "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stdin"
        ));
        assert!(output.contains(
            "talos: exec-stdin fd=0x0000000000000000 bytes=0x0000000000000018 return=0x0000000000000018 read-source=initramfs:/etc/banner.txt stdout-fd=0x0000000000000001 stdout-bytes=0x000000000000003d stdout-return=0x000000000000003d source=userspace-talos-read+userspace-talos-write read-result=regular-file-eof-after-read\n"
        ));
        assert!(output.contains(
            "talos: cat path=/tmp/stdin-report.txt bytes=0x000000000000003d source=volatile-vfs-descriptor-read\n"
        ));
        assert!(output.contains("Talos userspace stdin fixture read: Talos initramfs fixture\n"));
        assert!(output.contains(
            "talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains("Talos userspace stdin fixture read: talos-console0\n"));
        assert!(output.contains(
            "talos: exec-stdin fd=0x0000000000000000 bytes=0x000000000000000e return=0x000000000000000e read-source=runtime-console0/local-input stdout-fd=0x0000000000000001 stdout-bytes=0x0000000000000033 stdout-return=0x0000000000000033 source=userspace-talos-read+userspace-talos-write\n"
        ));
        assert_eq!(output.matches("talos: exec-invalid-path\n").count(), 8);
    }

    #[test_case]
    fn local_command_loop_runs_direct_path_pipeline_producer_stdin_redirection() {
        let bytes = *b"/bin/stdin </etc/banner.txt | /bin/stdin\rwaitpid 0x100001\rwaitpid 0x100002\rlaststatus\rpipestatus\rcat /proc/talos/processes\rps\r/bin/stdin </etc/banner.txt | /bin/stdout\r/bin/stdin alpha </etc/banner.txt | /bin/stdin\r/bin/stdin </etc/banner.txt | /bin/stdin beta\r/bin/stdin </dev/null | /bin/stdin\r/bin/stdin < /etc/banner.txt | /bin/stdin\r/bin/stdin </etc/banner.txt | /bin/stdin | /bin/stdin\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (
            pipeline,
            producer_wait,
            consumer_wait,
            last,
            status,
            cat,
            ps,
            unsupported_consumer,
            producer_arg,
            consumer_arg,
            dev_null,
            spaced,
            multistage,
        ) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            let pipeline = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(pipeline.line(), b"/bin/stdin </etc/banner.txt | /bin/stdin");
            assert_eq!(pipeline.status(), LocalCommandStatus::Handled);
            (
                pipeline,
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
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
            )
        };
        let output = backend.as_str();

        assert_eq!(pipeline.line(), b"/bin/stdin </etc/banner.txt | /bin/stdin");
        assert_eq!(pipeline.status(), LocalCommandStatus::Handled);
        assert_eq!(pipeline.response_lines(), 23);
        assert_eq!(producer_wait.status(), LocalCommandStatus::Handled);
        assert_eq!(consumer_wait.status(), LocalCommandStatus::Handled);
        assert_eq!(last.status(), LocalCommandStatus::Handled);
        assert_eq!(status.status(), LocalCommandStatus::Handled);
        assert_eq!(cat.status(), LocalCommandStatus::Handled);
        assert_eq!(ps.status(), LocalCommandStatus::Handled);
        assert_eq!(
            unsupported_consumer.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert_eq!(
            producer_arg.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert_eq!(
            consumer_arg.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert_eq!(dev_null.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(spaced.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(multistage.status(), LocalCommandStatus::UnexpectedArgument);
        assert!(output.contains("talos: exec path=/bin/stdin source=vfs-open-read\n"));
        assert!(output.contains(
            "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stdin"
        ));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=regular-file fd1=pipe-endpoint fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=pipe-endpoint fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-redirection op=source source-fd=0x0000000000000000 source-path=/etc/banner.txt source-stream=regular-file source-route=initramfs:/etc/banner.txt child-only=true shell-restored=true source=shell-redirection-stdin-etc-banner\n"
        ));
        assert!(output.contains("Talos userspace stdin fixture read: Talos initramfs fixture\n"));
        assert!(output.contains(
            "talos: exec-stdin fd=0x0000000000000000 bytes=0x0000000000000018 return=0x0000000000000018 read-source=initramfs:/etc/banner.txt stdout-fd=0x0000000000000001 stdout-bytes=0x000000000000003d stdout-return=0x000000000000003d source=userspace-talos-read+userspace-talos-write read-result=regular-file-eof-after-read\n"
        ));
        assert!(output.contains(
            "talos: exec-stdin fd=0x0000000000000000 bytes=0x000000000000003d return=0x000000000000003d read-source=pipe:stdout-to-stdin stdout-fd=0x0000000000000001 stdout-bytes=0x0000000000000062 stdout-return=0x0000000000000062 source=userspace-talos-read+userspace-talos-write read-result=pipe-eof-after-writer-close\n"
        ));
        assert!(output.contains(
            "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdin consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000003d bytes-read=0x000000000000003d writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-producer-stdin-redirection-to-stdin\n"
        ));
        assert!(output.contains(
            "talos: pipeline-lifecycle-status record=phase12-local-pipeline-distinct-process-lifecycle-status-record-v1 pipeline=0x0000000000000001 producer-pid=0x0000000000100001 producer-path=/bin/stdin producer-state=exited producer-status=0x0000000000000000 producer-observed-status=0x0000000000000000 producer-reaped=true consumer-pid=0x0000000000100002 consumer-path=/bin/stdin consumer-state=exited consumer-status=0x0000000000000000 consumer-observed-status=0x0000000000000000 consumer-reaped=true source=kernel-owned-pipeline-lifecycle-status-record\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: pipestatus participants=0x0000000000000002 default-status=0x0000000000000000 pipefail-status=0x0000000000000000 semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status\n"
        ));
        assert!(output.contains(
            "slot=0 capacity=3 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table\n"
        ));
        assert!(output.contains(
            "slot=1 capacity=3 pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table\n"
        ));
        assert_eq!(output.matches("talos-processes-v1\n").count(), 2);
        assert_eq!(output.matches("talos: exec-invalid-path\n").count(), 6);
    }

    #[test_case]
    fn local_command_loop_runs_direct_path_pipeline_combined_stdin_stdout_redirection() {
        let command = parse_local_command(
            b"/bin/stdin </etc/banner.txt | /bin/stdin >/tmp/pipeline-combined.txt",
        )
        .unwrap();
        let request = parse_absolute_path_pipeline_request(command).unwrap();
        assert_eq!(request.producer.path(), initramfs::PHASE10_STDIN_PATH);
        assert_eq!(request.producer.argv.argc(), 1);
        assert_eq!(
            request.producer.stdin_redirection,
            Some(LocalCommandExecRedirection::StdinFromEtcBanner)
        );
        assert_eq!(request.producer.redirection, None);
        assert_eq!(request.consumer.path(), initramfs::PHASE10_STDIN_PATH);
        assert_eq!(request.consumer.argv.argc(), 1);
        assert_eq!(request.consumer.stdin_redirection, None);
        assert!(matches!(
            request.consumer.redirection,
            Some(LocalCommandExecRedirection::StdoutToTmpStdout(path))
                if path.is_exact_pipeline_combined_path()
        ));
        assert!(is_direct_pipeline_combined_stdin_stdout_redirection(
            &request
        ));
        let mut pipeline_probe_backend = CaptureSink::new();
        let mut pipeline_probe_io = DescriptorBackedLocalCommandIo::new_inherited_stdio(
            ScriptedInput::new([], 0),
            &mut pipeline_probe_backend,
        )
        .unwrap();
        assert_eq!(pipeline_probe_io.exec_vfs_pipeline(request).err(), None);
        let bytes = *b"/bin/stdin </etc/banner.txt | /bin/stdin >/tmp/pipeline-combined.txt\rwaitpid 0x100001\rwaitpid 0x100002\rlaststatus\rpipestatus\rcat /proc/talos/processes\rps\rcat /tmp/pipeline-combined.txt\r/bin/stdin </etc/banner.txt | /bin/stdin >>/tmp/pipeline-combined.txt\r/bin/stdin </etc/banner.txt | /bin/stdin >/tmp/pipeline-report.txt\r/bin/stdout | /bin/stdin >/tmp/pipeline-combined.txt\r/bin/stdin </etc/banner.txt | /bin/stdin 1>/tmp/pipeline-combined.txt\r/bin/stdin </etc/banner.txt | /bin/stdin > /tmp/pipeline-combined.txt\r/bin/stdin </etc/banner.txt | /bin/stdin >/var/x\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (
            pipeline,
            producer_wait,
            consumer_wait,
            last,
            status,
            cat_processes,
            ps,
            readback,
            append,
            wrong_output_path,
            stdout_producer,
            explicit_fd,
            spaced_output,
            persistent_path,
        ) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            let pipeline = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(
                pipeline.line(),
                b"/bin/stdin </etc/banner.txt | /bin/stdin >/tmp/pipeline-combined.txt"
            );
            assert_eq!(pipeline.status(), LocalCommandStatus::Handled);
            let accepted_records = io.process_table_records();
            let producer_wait = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let consumer_wait = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let last = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let status = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let cat_processes = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let ps = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let readback = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let append = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records(), accepted_records);
            let wrong_output_path = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records(), accepted_records);
            let stdout_producer = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records(), accepted_records);
            let explicit_fd = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records(), accepted_records);
            let spaced_output = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records(), accepted_records);
            let persistent_path = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records(), accepted_records);
            (
                pipeline,
                producer_wait,
                consumer_wait,
                last,
                status,
                cat_processes,
                ps,
                readback,
                append,
                wrong_output_path,
                stdout_producer,
                explicit_fd,
                spaced_output,
                persistent_path,
            )
        };
        let output = backend.as_str();

        assert_eq!(pipeline.status(), LocalCommandStatus::Handled);
        assert_eq!(producer_wait.status(), LocalCommandStatus::Handled);
        assert_eq!(consumer_wait.status(), LocalCommandStatus::Handled);
        assert_eq!(last.status(), LocalCommandStatus::Handled);
        assert_eq!(status.status(), LocalCommandStatus::Handled);
        assert_eq!(cat_processes.status(), LocalCommandStatus::Handled);
        assert_eq!(ps.status(), LocalCommandStatus::Handled);
        assert_eq!(readback.status(), LocalCommandStatus::Handled);
        for rejected in [
            append,
            wrong_output_path,
            stdout_producer,
            explicit_fd,
            spaced_output,
            persistent_path,
        ] {
            assert_eq!(rejected.status(), LocalCommandStatus::UnexpectedArgument);
            assert_eq!(rejected.response_lines(), 1);
        }
        assert!(output.contains(
            "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdin consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000003d bytes-read=0x000000000000003d writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-producer-stdin-consumer-stdout-redirection\n"
        ));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=regular-file fd1=pipe-endpoint fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=pipe-endpoint fd1=regular-file fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-redirection op=source source-fd=0x0000000000000000 source-path=/etc/banner.txt source-stream=regular-file source-route=initramfs:/etc/banner.txt child-only=true shell-restored=true source=shell-redirection-stdin-etc-banner\n"
        ));
        assert!(output.contains(
            "talos: exec-redirection op=sink source-fd=0x0000000000000001 target-path=/tmp/pipeline-combined.txt target-stream=regular-file target-route=volatile-vfs:/tmp/pipeline-combined.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout\n"
        ));
        assert!(output.contains(
            "talos: exec-stdin fd=0x0000000000000000 bytes=0x0000000000000018 return=0x0000000000000018 read-source=initramfs:/etc/banner.txt stdout-fd=0x0000000000000001 stdout-bytes=0x000000000000003d stdout-return=0x000000000000003d source=userspace-talos-read+userspace-talos-write read-result=regular-file-eof-after-read\n"
        ));
        assert!(output.contains(
            "talos: exec-stdin fd=0x0000000000000000 bytes=0x000000000000003d return=0x000000000000003d read-source=pipe:stdout-to-stdin stdout-fd=0x0000000000000001 stdout-bytes=0x0000000000000062 stdout-return=0x0000000000000062 source=userspace-talos-read+userspace-talos-write read-result=pipe-eof-after-writer-close\n"
        ));
        assert!(output.contains(
            "talos: cat path=/tmp/pipeline-combined.txt bytes=0x0000000000000062 source=volatile-vfs-descriptor-read\n"
        ));
        assert!(output.contains(
            "talos> Talos userspace stdin fixture read: Talos userspace stdin fixture read: Talos initramfs fixture\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: pipestatus participants=0x0000000000000002 default-status=0x0000000000000000 pipefail-status=0x0000000000000000 semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status\n"
        ));
        assert_eq!(output.matches("talos-processes-v1\n").count(), 2);
        assert_eq!(output.matches("talos: exec-invalid-path\n").count(), 6);
    }

    #[test_case]
    fn local_command_loop_appends_direct_path_pipeline_combined_stdin_stdout_redirection() {
        let command = parse_local_command(
            b"/bin/stdin </etc/banner.txt | /bin/stdin >>/tmp/pipeline-combined-append.txt",
        )
        .unwrap();
        let request = parse_absolute_path_pipeline_request(command).unwrap();
        assert_eq!(request.producer.path(), initramfs::PHASE10_STDIN_PATH);
        assert_eq!(
            request.producer.stdin_redirection,
            Some(LocalCommandExecRedirection::StdinFromEtcBanner)
        );
        assert_eq!(request.consumer.path(), initramfs::PHASE10_STDIN_PATH);
        assert!(matches!(
            request.consumer.redirection,
            Some(LocalCommandExecRedirection::StdoutAppendTmpStdout(path))
                if path.is_exact_pipeline_combined_append_path()
        ));
        assert!(is_direct_pipeline_combined_stdin_stdout_redirection(
            &request
        ));

        let bytes = *b"/bin/stdin </etc/banner.txt | /bin/stdin >/tmp/pipeline-combined-append.txt\r/bin/stdin </etc/banner.txt | /bin/stdin >>/tmp/pipeline-combined-append.txt\rwaitpid 0x100001\rwaitpid 0x100002\rlaststatus\rpipestatus\rcat /proc/talos/processes\rps\rcat /tmp/pipeline-combined-append.txt\r/bin/stdin </etc/banner.txt\rstdin </etc/banner.txt | /bin/stdin >>/tmp/pipeline-combined-append.txt\r/bin/stdin </etc/banner.txt | /bin/stdin 2>>/tmp/pipeline-combined-append.txt\r/bin/stdin </etc/banner.txt | /bin/stdin >/tmp/pipeline-report.txt\r/bin/stdout | /bin/stdin >>/tmp/pipeline-combined-append.txt\r/bin/stdin </etc/banner.txt | /bin/stdin 1>/tmp/pipeline-combined-append.txt\r/bin/stdin </etc/banner.txt | /bin/stdin > /tmp/pipeline-combined-append.txt\r/bin/stdin </etc/banner.txt | /bin/stdin >>/var/x\r/bin/stdin </etc/banner.txt | /bin/stdin >>/tmp/pipeline-combined-append.txt | /bin/stdin\rmissing </etc/banner.txt | /bin/stdin >>/tmp/pipeline-combined-append.txt\r/bin/stdin </etc/banner.txt | missing >>/tmp/pipeline-combined-append.txt\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (
            pipeline,
            append_pipeline,
            producer_wait,
            consumer_wait,
            last,
            status,
            cat_processes,
            ps,
            readback,
            normal,
            mixed_bare_absolute_append,
            stderr_append,
            wrong_output_path,
            stdout_producer,
            explicit_fd,
            spaced_output,
            persistent_path,
            multistage,
            missing_producer,
            missing_consumer,
        ) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            let pipeline = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(
                pipeline.line(),
                b"/bin/stdin </etc/banner.txt | /bin/stdin >/tmp/pipeline-combined-append.txt"
            );
            assert_eq!(pipeline.status(), LocalCommandStatus::Handled);
            let append_pipeline = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(
                append_pipeline.line(),
                b"/bin/stdin </etc/banner.txt | /bin/stdin >>/tmp/pipeline-combined-append.txt"
            );
            assert_eq!(append_pipeline.status(), LocalCommandStatus::Handled);
            let producer_wait = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let consumer_wait = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let last = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let status = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let cat_processes = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let ps = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let readback = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let normal = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let retained_records = io.process_table_records();
            let mixed_bare_absolute_append =
                run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records(), retained_records);
            let stderr_append = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records(), retained_records);
            let wrong_output_path = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records(), retained_records);
            let stdout_producer = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records(), retained_records);
            let explicit_fd = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records(), retained_records);
            let spaced_output = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records(), retained_records);
            let persistent_path = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records(), retained_records);
            let multistage = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records(), retained_records);
            let missing_producer = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records(), retained_records);
            let missing_consumer = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records(), retained_records);
            (
                pipeline,
                append_pipeline,
                producer_wait,
                consumer_wait,
                last,
                status,
                cat_processes,
                ps,
                readback,
                normal,
                mixed_bare_absolute_append,
                stderr_append,
                wrong_output_path,
                stdout_producer,
                explicit_fd,
                spaced_output,
                persistent_path,
                multistage,
                missing_producer,
                missing_consumer,
            )
        };
        let output = backend.as_str();

        for handled in [
            pipeline,
            append_pipeline,
            producer_wait,
            consumer_wait,
            last,
            status,
            cat_processes,
            ps,
            readback,
            normal,
        ] {
            assert_eq!(handled.status(), LocalCommandStatus::Handled);
        }
        for rejected in [
            mixed_bare_absolute_append,
            stderr_append,
            wrong_output_path,
            stdout_producer,
            explicit_fd,
            spaced_output,
            persistent_path,
            multistage,
        ] {
            assert_eq!(rejected.status(), LocalCommandStatus::UnexpectedArgument);
            assert_eq!(rejected.response_lines(), 1);
        }
        assert_eq!(
            missing_producer.status(),
            LocalCommandStatus::UnknownCommand
        );
        assert_eq!(
            missing_consumer.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert!(output.contains(
            "talos: exec-redirection op=sink source-fd=0x0000000000000001 target-path=/tmp/pipeline-combined-append.txt target-stream=regular-file target-route=volatile-vfs:/tmp/pipeline-combined-append.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout\n"
        ));
        assert!(output.contains(
            "talos: exec-redirection op=append source-fd=0x0000000000000001 target-path=/tmp/pipeline-combined-append.txt target-stream=regular-file target-route=volatile-vfs:/tmp/pipeline-combined-append.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout-append\n"
        ));
        assert!(output.contains(
            "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdin consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000003d bytes-read=0x000000000000003d writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-producer-stdin-consumer-stdout-redirection\n"
        ));
        assert!(output.contains(
            "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdin consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000003d bytes-read=0x000000000000003d writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-producer-stdin-consumer-stdout-append-redirection\n"
        ));
        assert!(output.contains(
            "talos: cat path=/tmp/pipeline-combined-append.txt bytes=0x00000000000000c4 source=volatile-vfs-descriptor-read\n"
        ));
        assert_eq!(
            output
                .matches(
                    "Talos userspace stdin fixture read: Talos userspace stdin fixture read: Talos initramfs fixture\n"
                )
                .count(),
            2
        );
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: pipestatus participants=0x0000000000000002 default-status=0x0000000000000000 pipefail-status=0x0000000000000000 semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status\n"
        ));
        assert_eq!(output.matches("talos-processes-v1\n").count(), 2);
        assert_eq!(output.matches("talos: exec-invalid-path\n").count(), 9);
    }

    #[test_case]
    fn local_command_loop_runs_direct_path_pipeline_combined_stdin_stderr_redirection() {
        let command = parse_local_command(
            b"/bin/stdin </etc/banner.txt | /bin/stderr 2>/tmp/pipeline-combined-stderr.txt",
        )
        .unwrap();
        let request = parse_absolute_path_pipeline_request(command).unwrap();
        assert!(is_direct_pipeline_combined_stdin_stderr_redirection(
            &request
        ));

        let bytes = *b"/bin/stdin </etc/banner.txt | /bin/stderr 2>/tmp/pipeline-combined-stderr.txt\rwaitpid 0x100001\rwaitpid 0x100002\rlaststatus\rpipestatus\rcat /proc/talos/processes\rps\rcat /tmp/pipeline-combined-stderr.txt\r/bin/stdin </etc/banner.txt\r/bin/stdout\r/bin/stderr\rstdin </etc/banner.txt | /bin/stderr 2>/tmp/pipeline-combined-stderr.txt\r/bin/stdin </etc/banner.txt | /bin/stderr 2>>/tmp/pipeline-combined-stderr.txt\r/bin/stdin </etc/banner.txt | /bin/stderr 2>/tmp/pipeline-stderr.txt\r/bin/stdout | /bin/stderr 2>/tmp/pipeline-combined-stderr.txt\r/bin/stdin </etc/banner.txt | /bin/stderr 2> /tmp/pipeline-combined-stderr.txt\r/bin/stdin </etc/banner.txt | /bin/stderr 2>/var/x\r/bin/stdin </etc/banner.txt | /bin/stderr 2>/tmp/pipeline-combined-stderr.txt | /bin/stdin\rmissing </etc/banner.txt | /bin/stderr 2>/tmp/pipeline-combined-stderr.txt\r/bin/stdin </etc/banner.txt | missing 2>/tmp/pipeline-combined-stderr.txt\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let statuses = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            let mut statuses = [LocalCommandStatus::Handled; 20];
            let mut index = 0usize;
            while index < statuses.len() {
                let record = run_one_descriptor_backed_serial_command(&mut io).unwrap();
                statuses[index] = record.status();
                if index == 0 {
                    assert_eq!(
                        io.process_table_records()
                            .iter()
                            .filter(|record| record.is_some())
                            .count(),
                        2
                    );
                }
                index += 1;
            }
            statuses
        };
        let output = backend.as_str();

        for status in &statuses[..11] {
            assert_eq!(*status, LocalCommandStatus::Handled);
        }
        for status in &statuses[11..18] {
            assert_eq!(*status, LocalCommandStatus::UnexpectedArgument);
        }
        assert_eq!(statuses[18], LocalCommandStatus::UnknownCommand);
        assert_eq!(statuses[19], LocalCommandStatus::UnexpectedArgument);
        assert!(output.contains("talos: exec path=/bin/stdin source=vfs-open-read\n"));
        assert!(output.contains("talos: exec path=/bin/stderr source=vfs-open-read\n"));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=regular-file fd1=pipe-endpoint fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=pipe-endpoint fd1=stdio-output fd2=regular-file loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-redirection op=source source-fd=0x0000000000000000 source-path=/etc/banner.txt source-stream=regular-file source-route=initramfs:/etc/banner.txt child-only=true shell-restored=true source=shell-redirection-stdin-etc-banner\n"
        ));
        assert!(output.contains(
            "talos: exec-redirection op=sink source-fd=0x0000000000000002 target-path=/tmp/pipeline-combined-stderr.txt target-stream=regular-file target-route=volatile-vfs:/tmp/pipeline-combined-stderr.txt child-only=true shell-restored=true source=shell-redirection-stderr-tmp-stderr\n"
        ));
        assert!(output.contains(
            "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdin consumer-fd=0x0000000000000000 consumer-path=/bin/stderr bytes-written=0x000000000000003d bytes-read=0x0000000000000000 writer-closed=true reader-eof=false shell-restored=true source=shell-pipe-producer-stdin-consumer-stderr-redirection\n"
        ));
        assert!(output.contains(
            "talos: cat path=/tmp/pipeline-combined-stderr.txt bytes=0x000000000000001f source=volatile-vfs-descriptor-read\n"
        ));
        assert!(output.contains("Talos userspace stdin fixture read: Talos initramfs fixture\n"));
        assert!(output.contains("Talos userspace stdout fixture\n"));
        assert!(output.contains("Talos userspace stderr fixture\n"));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: pipestatus participants=0x0000000000000002 default-status=0x0000000000000000 pipefail-status=0x0000000000000000 semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status\n"
        ));
        assert_eq!(output.matches("talos-processes-v1\n").count(), 2);
        assert_eq!(output.matches("talos: exec-invalid-path\n").count(), 8);
        assert_eq!(output.matches("talos: unknown-command").count(), 1);
    }

    #[test_case]
    fn local_command_loop_appends_bare_name_pipeline_combined_stdin_stdout_redirection() {
        let command = parse_local_command(
            b"stdin </etc/banner.txt | stdin >>/tmp/pipeline-combined-append.txt",
        )
        .unwrap();
        let request = parse_bare_bin_pipeline_request(command).unwrap();
        assert_eq!(request.producer.path(), initramfs::PHASE10_STDIN_PATH);
        assert_eq!(request.producer.argv.argc(), 1);
        assert_eq!(
            request.producer.stdin_redirection,
            Some(LocalCommandExecRedirection::StdinFromEtcBanner)
        );
        assert_eq!(request.consumer.path(), initramfs::PHASE10_STDIN_PATH);
        assert_eq!(request.consumer.argv.argc(), 1);
        assert!(matches!(
            request.consumer.redirection,
            Some(LocalCommandExecRedirection::StdoutAppendTmpStdout(path))
                if path.is_exact_pipeline_combined_append_path()
        ));
        assert!(is_direct_pipeline_combined_stdin_stdout_redirection(
            &request
        ));

        let bytes = *b"stdin </etc/banner.txt | stdin >/tmp/pipeline-combined-append.txt\rstdin </etc/banner.txt | stdin >>/tmp/pipeline-combined-append.txt\rwaitpid 0x100001\rwaitpid 0x100002\rlaststatus\rpipestatus\rcat /proc/talos/processes\rps\rcat /tmp/pipeline-combined-append.txt\rstdin </etc/banner.txt\r/bin/stdin </etc/banner.txt | /bin/stdin >/tmp/pipeline-combined-append.txt\rmissing </etc/banner.txt | stdin >>/tmp/pipeline-combined-append.txt\rstdin </etc/banner.txt | missing >>/tmp/pipeline-combined-append.txt\rstdin </etc/banner.txt | stdin 2>>/tmp/pipeline-combined-append.txt\rstdin </etc/banner.txt | stdin >/tmp/pipeline-report.txt\rstdin </etc/banner.txt | stdin 1>/tmp/pipeline-combined-append.txt\rstdin </etc/banner.txt | stdin > /tmp/pipeline-combined-append.txt\rstdin </etc/banner.txt | stdin >>/var/x\rstdin </etc/banner.txt | /bin/stdin >>/tmp/pipeline-combined-append.txt\r/bin/stdin </etc/banner.txt | stdin >>/tmp/pipeline-combined-append.txt\rstdin </etc/banner.txt | stdin >>/tmp/pipeline-combined-append.txt | stdin\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (
            pipeline,
            append_pipeline,
            producer_wait,
            consumer_wait,
            last,
            status,
            cat_processes,
            ps,
            readback,
            normal,
            direct_path_form,
            missing_producer,
            missing_consumer,
            stderr_append,
            wrong_output_path,
            explicit_fd,
            spaced_output,
            persistent_path,
            slash_consumer,
            path_producer,
            multistage,
        ) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            let pipeline = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(
                pipeline.line(),
                b"stdin </etc/banner.txt | stdin >/tmp/pipeline-combined-append.txt"
            );
            assert_eq!(pipeline.status(), LocalCommandStatus::Handled);
            let append_pipeline = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(
                append_pipeline.line(),
                b"stdin </etc/banner.txt | stdin >>/tmp/pipeline-combined-append.txt"
            );
            assert_eq!(append_pipeline.status(), LocalCommandStatus::Handled);
            let producer_wait = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let consumer_wait = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let last = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let status = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let cat_processes = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let ps = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let readback = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let normal = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let direct_path_form = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(direct_path_form.status(), LocalCommandStatus::Handled);
            let retained_records = io.process_table_records();
            let missing_producer = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records(), retained_records);
            let missing_consumer = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records(), retained_records);
            let stderr_append = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records(), retained_records);
            let wrong_output_path = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records(), retained_records);
            let explicit_fd = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records(), retained_records);
            let spaced_output = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records(), retained_records);
            let persistent_path = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records(), retained_records);
            let slash_consumer = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records(), retained_records);
            let path_producer = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records(), retained_records);
            let multistage = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records(), retained_records);
            (
                pipeline,
                append_pipeline,
                producer_wait,
                consumer_wait,
                last,
                status,
                cat_processes,
                ps,
                readback,
                normal,
                direct_path_form,
                missing_producer,
                missing_consumer,
                stderr_append,
                wrong_output_path,
                explicit_fd,
                spaced_output,
                persistent_path,
                slash_consumer,
                path_producer,
                multistage,
            )
        };
        let output = backend.as_str();

        for handled in [
            pipeline,
            append_pipeline,
            producer_wait,
            consumer_wait,
            last,
            status,
            cat_processes,
            ps,
            readback,
            normal,
            direct_path_form,
        ] {
            assert_eq!(handled.status(), LocalCommandStatus::Handled);
        }
        assert_eq!(
            missing_producer.status(),
            LocalCommandStatus::UnknownCommand
        );
        for rejected in [
            missing_consumer,
            stderr_append,
            wrong_output_path,
            explicit_fd,
            spaced_output,
            persistent_path,
            slash_consumer,
            path_producer,
            multistage,
        ] {
            assert_eq!(rejected.status(), LocalCommandStatus::UnexpectedArgument);
            assert_eq!(rejected.response_lines(), 1);
        }
        assert!(output.contains(
            "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stdin"
        ));
        assert!(output.contains(
            "talos: exec-redirection op=sink source-fd=0x0000000000000001 target-path=/tmp/pipeline-combined-append.txt target-stream=regular-file target-route=volatile-vfs:/tmp/pipeline-combined-append.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout\n"
        ));
        assert!(output.contains(
            "talos: exec-redirection op=append source-fd=0x0000000000000001 target-path=/tmp/pipeline-combined-append.txt target-stream=regular-file target-route=volatile-vfs:/tmp/pipeline-combined-append.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout-append\n"
        ));
        assert!(output.contains(
            "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdin consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000003d bytes-read=0x000000000000003d writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-producer-stdin-consumer-stdout-redirection\n"
        ));
        assert!(output.contains(
            "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdin consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000003d bytes-read=0x000000000000003d writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-producer-stdin-consumer-stdout-append-redirection\n"
        ));
        assert!(output.contains(
            "talos: cat path=/tmp/pipeline-combined-append.txt bytes=0x00000000000000c4 source=volatile-vfs-descriptor-read\n"
        ));
        assert_eq!(
            output
                .matches(
                    "Talos userspace stdin fixture read: Talos userspace stdin fixture read: Talos initramfs fixture\n"
                )
                .count(),
            2
        );
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: pipestatus participants=0x0000000000000002 default-status=0x0000000000000000 pipefail-status=0x0000000000000000 semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status\n"
        ));
        assert_eq!(output.matches("talos-processes-v1\n").count(), 2);
        assert_eq!(output.matches("talos: exec-invalid-path\n").count(), 9);
    }

    #[test_case]
    fn local_command_loop_runs_bare_name_pipeline_combined_stdin_stdout_redirection() {
        let command =
            parse_local_command(b"stdin </etc/banner.txt | stdin >/tmp/pipeline-combined.txt")
                .unwrap();
        let request = parse_bare_bin_pipeline_request(command).unwrap();
        assert_eq!(request.producer.path(), initramfs::PHASE10_STDIN_PATH);
        assert_eq!(request.producer.argv.argc(), 1);
        assert_eq!(
            request.producer.stdin_redirection,
            Some(LocalCommandExecRedirection::StdinFromEtcBanner)
        );
        assert_eq!(request.producer.redirection, None);
        assert_eq!(request.consumer.path(), initramfs::PHASE10_STDIN_PATH);
        assert_eq!(request.consumer.argv.argc(), 1);
        assert_eq!(request.consumer.stdin_redirection, None);
        assert!(matches!(
            request.consumer.redirection,
            Some(LocalCommandExecRedirection::StdoutToTmpStdout(path))
                if path.is_exact_pipeline_combined_path()
        ));
        assert!(is_direct_pipeline_combined_stdin_stdout_redirection(
            &request
        ));
        let bytes = *b"stdin </etc/banner.txt | stdin >/tmp/pipeline-combined.txt\rwaitpid 0x100001\rwaitpid 0x100002\rlaststatus\rpipestatus\rcat /proc/talos/processes\rps\rcat /tmp/pipeline-combined.txt\rstdin </etc/banner.txt\r/bin/stdin </etc/banner.txt | /bin/stdin >/tmp/pipeline-combined.txt\rstdin </etc/banner.txt | stdin >>/tmp/pipeline-combined.txt\rstdin </etc/banner.txt | stdin >/tmp/pipeline-report.txt\rstdout | stdin >/tmp/pipeline-combined.txt\rstdin </etc/banner.txt | stdin 1>/tmp/pipeline-combined.txt\rstdin </etc/banner.txt | stdin > /tmp/pipeline-combined.txt\rstdin </etc/banner.txt | stdin >/var/x\rstdin </etc/banner.txt | /bin/stdin >/tmp/pipeline-combined.txt\r/bin/stdin </etc/banner.txt | stdin >/tmp/pipeline-combined.txt\rstdin </etc/banner.txt | stdin >/tmp/pipeline-combined.txt | stdin\rmissing </etc/banner.txt | stdin >/tmp/pipeline-combined.txt\rstdin </etc/banner.txt | missing >/tmp/pipeline-combined.txt\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (
            pipeline,
            producer_wait,
            consumer_wait,
            last,
            status,
            cat_processes,
            ps,
            readback,
            normal,
            direct_path_form,
            append,
            wrong_output_path,
            stdout_producer,
            explicit_fd,
            spaced_output,
            persistent_path,
            slash_consumer,
            path_producer,
            multistage,
            missing_producer,
            missing_consumer,
        ) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            let pipeline = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(
                pipeline.line(),
                b"stdin </etc/banner.txt | stdin >/tmp/pipeline-combined.txt"
            );
            assert_eq!(pipeline.status(), LocalCommandStatus::Handled);
            let producer_wait = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let consumer_wait = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let last = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let status = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let cat_processes = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let ps = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let readback = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let normal = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let direct_path_form = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let append = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let retained_records = io.process_table_records();
            let wrong_output_path = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records(), retained_records);
            let stdout_producer = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records(), retained_records);
            let explicit_fd = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records(), retained_records);
            let spaced_output = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records(), retained_records);
            let persistent_path = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records(), retained_records);
            let slash_consumer = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records(), retained_records);
            let path_producer = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records(), retained_records);
            let multistage = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records(), retained_records);
            let missing_producer = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records(), retained_records);
            let missing_consumer = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records(), retained_records);
            (
                pipeline,
                producer_wait,
                consumer_wait,
                last,
                status,
                cat_processes,
                ps,
                readback,
                normal,
                direct_path_form,
                append,
                wrong_output_path,
                stdout_producer,
                explicit_fd,
                spaced_output,
                persistent_path,
                slash_consumer,
                path_producer,
                multistage,
                missing_producer,
                missing_consumer,
            )
        };
        let output = backend.as_str();

        assert_eq!(pipeline.status(), LocalCommandStatus::Handled);
        assert_eq!(producer_wait.status(), LocalCommandStatus::Handled);
        assert_eq!(consumer_wait.status(), LocalCommandStatus::Handled);
        assert_eq!(last.status(), LocalCommandStatus::Handled);
        assert_eq!(status.status(), LocalCommandStatus::Handled);
        assert_eq!(cat_processes.status(), LocalCommandStatus::Handled);
        assert_eq!(ps.status(), LocalCommandStatus::Handled);
        assert_eq!(readback.status(), LocalCommandStatus::Handled);
        assert_eq!(normal.status(), LocalCommandStatus::Handled);
        assert_eq!(direct_path_form.status(), LocalCommandStatus::Handled);
        for rejected in [
            append,
            wrong_output_path,
            stdout_producer,
            explicit_fd,
            spaced_output,
            persistent_path,
            slash_consumer,
            path_producer,
            multistage,
        ] {
            assert_eq!(rejected.status(), LocalCommandStatus::UnexpectedArgument);
            assert_eq!(rejected.response_lines(), 1);
        }
        assert_eq!(
            missing_producer.status(),
            LocalCommandStatus::UnknownCommand
        );
        assert_eq!(
            missing_consumer.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert!(output.contains(
            "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stdin"
        ));
        assert!(output.contains(
            "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdin consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000003d bytes-read=0x000000000000003d writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-producer-stdin-consumer-stdout-redirection\n"
        ));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=regular-file fd1=pipe-endpoint fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=pipe-endpoint fd1=regular-file fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-redirection op=source source-fd=0x0000000000000000 source-path=/etc/banner.txt source-stream=regular-file source-route=initramfs:/etc/banner.txt child-only=true shell-restored=true source=shell-redirection-stdin-etc-banner\n"
        ));
        assert!(output.contains(
            "talos: exec-redirection op=sink source-fd=0x0000000000000001 target-path=/tmp/pipeline-combined.txt target-stream=regular-file target-route=volatile-vfs:/tmp/pipeline-combined.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout\n"
        ));
        assert!(output.contains(
            "talos: exec-stdin fd=0x0000000000000000 bytes=0x0000000000000018 return=0x0000000000000018 read-source=initramfs:/etc/banner.txt stdout-fd=0x0000000000000001 stdout-bytes=0x000000000000003d stdout-return=0x000000000000003d source=userspace-talos-read+userspace-talos-write read-result=regular-file-eof-after-read\n"
        ));
        assert!(output.contains(
            "talos: exec-stdin fd=0x0000000000000000 bytes=0x000000000000003d return=0x000000000000003d read-source=pipe:stdout-to-stdin stdout-fd=0x0000000000000001 stdout-bytes=0x0000000000000062 stdout-return=0x0000000000000062 source=userspace-talos-read+userspace-talos-write read-result=pipe-eof-after-writer-close\n"
        ));
        assert!(output.contains(
            "talos: cat path=/tmp/pipeline-combined.txt bytes=0x0000000000000062 source=volatile-vfs-descriptor-read\n"
        ));
        assert!(output.contains(
            "talos> Talos userspace stdin fixture read: Talos userspace stdin fixture read: Talos initramfs fixture\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: pipestatus participants=0x0000000000000002 default-status=0x0000000000000000 pipefail-status=0x0000000000000000 semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status\n"
        ));
        assert_eq!(output.matches("talos-processes-v1\n").count(), 2);
        assert_eq!(output.matches("talos: exec-invalid-path\n").count(), 10);
    }

    #[test_case]
    fn local_command_loop_runs_direct_path_pipeline_consumer_stdin_redirection() {
        let bytes = *b"/bin/stdin </etc/banner.txt | /bin/stdin </etc/banner.txt\rwaitpid 0x100001\rwaitpid 0x100002\rlaststatus\rpipestatus\rcat /proc/talos/processes\rps\r/bin/stdout | /bin/stdin </etc/banner.txt\r/bin/stdin alpha | /bin/stdin </etc/banner.txt\r/bin/stdin | /bin/stdin beta </etc/banner.txt\r/bin/stdin | /bin/stdin </dev/null\r/bin/stdin | /bin/stdin < /etc/banner.txt\r/bin/stdin | /bin/stdin </etc/banner.txt | /bin/stdin\r/bin/stdin </etc/banner.txt | /bin/stdin </etc/banner.txt\r/bin/stdin </etc/banner.txt | stdin </etc/banner.txt\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (
            pipeline,
            producer_wait,
            consumer_wait,
            last,
            status,
            cat,
            ps,
            unsupported_producer,
            producer_arg,
            consumer_arg,
            dev_null,
            spaced,
            multistage,
            both_stages,
            mixed_form,
        ) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            let pipeline = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(pipeline.status(), LocalCommandStatus::Handled);
            (
                pipeline,
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
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
            )
        };
        let output = backend.as_str();

        assert_eq!(
            pipeline.line(),
            b"/bin/stdin </etc/banner.txt | /bin/stdin </etc/banner.txt"
        );
        assert_eq!(pipeline.status(), LocalCommandStatus::Handled);
        assert_eq!(producer_wait.status(), LocalCommandStatus::Handled);
        assert_eq!(consumer_wait.status(), LocalCommandStatus::Handled);
        assert_eq!(last.status(), LocalCommandStatus::Handled);
        assert_eq!(status.status(), LocalCommandStatus::Handled);
        assert_eq!(cat.status(), LocalCommandStatus::Handled);
        assert_eq!(ps.status(), LocalCommandStatus::Handled);
        assert_eq!(
            unsupported_producer.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert_eq!(
            producer_arg.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert_eq!(
            consumer_arg.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert_eq!(dev_null.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(spaced.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(multistage.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(both_stages.status(), LocalCommandStatus::Handled);
        assert_eq!(mixed_form.status(), LocalCommandStatus::UnexpectedArgument);
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=regular-file fd1=pipe-endpoint fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=regular-file fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-redirection op=source source-fd=0x0000000000000000 source-path=/etc/banner.txt source-stream=regular-file source-route=initramfs:/etc/banner.txt child-only=true shell-restored=true source=shell-redirection-stdin-etc-banner\n"
        ));
        assert!(output.contains(
            "talos: exec-stdin fd=0x0000000000000000 bytes=0x0000000000000018 return=0x0000000000000018 read-source=initramfs:/etc/banner.txt stdout-fd=0x0000000000000001 stdout-bytes=0x000000000000003d stdout-return=0x000000000000003d source=userspace-talos-read+userspace-talos-write read-result=regular-file-eof-after-read\n"
        ));
        assert!(output.contains(
            "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdin consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000003d bytes-read=0x0000000000000000 writer-closed=true reader-eof=false shell-restored=true source=shell-pipe-dual-stdin-redirection-from-file\n"
        ));
        assert!(output.contains(
            "talos> talos: pipestatus participants=0x0000000000000002 default-status=0x0000000000000000 pipefail-status=0x0000000000000000 semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status\n"
        ));
        assert!(output.contains(
            "slot=0 capacity=3 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table\n"
        ));
        assert!(output.contains(
            "slot=1 capacity=3 pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table\n"
        ));
        assert_eq!(output.matches("talos-processes-v1\n").count(), 2);
        assert_eq!(output.matches("talos: exec-invalid-path\n").count(), 7);
    }

    #[test_case]
    fn local_command_loop_runs_bare_name_pipeline_consumer_stdin_redirection() {
        let bytes = *b"stdin </etc/banner.txt | stdin </etc/banner.txt\rwaitpid 0x100001\rwaitpid 0x100002\rlaststatus\rpipestatus\rcat /proc/talos/processes\rps\rstdout | stdin </etc/banner.txt\rstdin alpha | stdin </etc/banner.txt\rstdin | stdin beta </etc/banner.txt\rstdin | stdin </dev/null\rstdin | stdin < /etc/banner.txt\rstdin | stdin </etc/banner.txt | stdin\rstdin </etc/banner.txt | stdin </etc/banner.txt\rstdin </etc/banner.txt | /bin/stdin </etc/banner.txt\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (
            pipeline,
            producer_wait,
            consumer_wait,
            last,
            status,
            cat,
            ps,
            unsupported_producer,
            producer_arg,
            consumer_arg,
            dev_null,
            spaced,
            multistage,
            both_stages,
            mixed_form,
        ) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            let pipeline = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(
                pipeline.line(),
                b"stdin </etc/banner.txt | stdin </etc/banner.txt"
            );
            assert_eq!(pipeline.status(), LocalCommandStatus::Handled);
            (
                pipeline,
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
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
            )
        };
        let output = backend.as_str();

        assert_eq!(
            pipeline.line(),
            b"stdin </etc/banner.txt | stdin </etc/banner.txt"
        );
        assert_eq!(pipeline.status(), LocalCommandStatus::Handled);
        assert_eq!(producer_wait.status(), LocalCommandStatus::Handled);
        assert_eq!(consumer_wait.status(), LocalCommandStatus::Handled);
        assert_eq!(last.status(), LocalCommandStatus::Handled);
        assert_eq!(status.status(), LocalCommandStatus::Handled);
        assert_eq!(cat.status(), LocalCommandStatus::Handled);
        assert_eq!(ps.status(), LocalCommandStatus::Handled);
        assert_eq!(
            unsupported_producer.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert_eq!(
            producer_arg.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert_eq!(
            consumer_arg.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert_eq!(dev_null.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(spaced.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(multistage.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(both_stages.status(), LocalCommandStatus::Handled);
        assert_eq!(mixed_form.status(), LocalCommandStatus::UnexpectedArgument);
        assert!(output.contains("talos: exec path=/bin/stdin source=vfs-open-read\n"));
        assert!(output.contains(
            "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stdin"
        ));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=regular-file fd1=pipe-endpoint fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=regular-file fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-redirection op=source source-fd=0x0000000000000000 source-path=/etc/banner.txt source-stream=regular-file source-route=initramfs:/etc/banner.txt child-only=true shell-restored=true source=shell-redirection-stdin-etc-banner\n"
        ));
        assert!(output.contains(
            "talos: exec-stdin fd=0x0000000000000000 bytes=0x0000000000000018 return=0x0000000000000018 read-source=initramfs:/etc/banner.txt stdout-fd=0x0000000000000001 stdout-bytes=0x000000000000003d stdout-return=0x000000000000003d source=userspace-talos-read+userspace-talos-write read-result=regular-file-eof-after-read\n"
        ));
        assert!(output.contains(
            "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdin consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000003d bytes-read=0x0000000000000000 writer-closed=true reader-eof=false shell-restored=true source=shell-pipe-dual-stdin-redirection-from-file\n"
        ));
        assert!(output.contains(
            "talos: pipeline-lifecycle-status record=phase12-local-pipeline-distinct-process-lifecycle-status-record-v1 pipeline=0x0000000000000001 producer-pid=0x0000000000100001 producer-path=/bin/stdin producer-state=exited producer-status=0x0000000000000000 producer-observed-status=0x0000000000000000 producer-reaped=true consumer-pid=0x0000000000100002 consumer-path=/bin/stdin consumer-state=exited consumer-status=0x0000000000000000 consumer-observed-status=0x0000000000000000 consumer-reaped=true source=kernel-owned-pipeline-lifecycle-status-record\n"
        ));
        assert!(output.contains(
            "talos> talos: pipestatus participants=0x0000000000000002 default-status=0x0000000000000000 pipefail-status=0x0000000000000000 semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status\n"
        ));
        assert!(output.contains(
            "slot=0 capacity=3 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table\n"
        ));
        assert!(output.contains(
            "slot=1 capacity=3 pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table\n"
        ));
        assert_eq!(output.matches("talos-processes-v1\n").count(), 2);
        assert_eq!(output.matches("talos: exec-invalid-path\n").count(), 7);
    }

    #[test_case]
    fn local_command_loop_runs_bare_name_vfs_command_through_bounded_bin_lookup() {
        let bytes = *b"status42\rwaitpid\rlaststatus\rcat /proc/talos/processes\rps\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (exec, waited, observed, cat, ps) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            let exec = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let record = io.process_table_records()[0].unwrap();
            assert_eq!(
                record.lifecycle.source_path,
                initramfs::PHASE10_STATUS42_PATH
            );
            assert_eq!(record.lifecycle.status, 0x2a);
            assert_eq!(record.lifecycle.observed_status, 0x2a);
            let waited = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let observed = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let cat = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let ps = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records()[0], Some(record));
            (exec, waited, observed, cat, ps)
        };
        let output = backend.as_str();

        assert_eq!(exec.line(), b"status42");
        assert_eq!(exec.status(), LocalCommandStatus::Handled);
        assert_eq!(exec.response_lines(), 10);
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert_eq!(cat.status(), LocalCommandStatus::Handled);
        assert_eq!(ps.status(), LocalCommandStatus::Handled);
        assert!(output.contains("talos> talos: exec path=/bin/status42 source=vfs-open-read\n"));
        assert!(output.contains(
            "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/status42"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "slot=0 capacity=3 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true wait-consumed=true job-state=foreground source=bounded-process-table\n"
        ));
    }

    #[test_case]
    fn local_command_loop_runs_bare_name_vfs_command_with_literal_argv() {
        let bytes = *b"status42 alpha beta\rwaitpid\rlaststatus\rcat /proc/talos/processes\rps\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (exec, waited, observed, cat, ps) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            let exec = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let record = io.process_table_records()[0].unwrap();
            assert_eq!(
                record.lifecycle.source_path,
                initramfs::PHASE10_STATUS42_PATH
            );
            assert_eq!(record.lifecycle.status, 0x2a);
            assert_eq!(record.lifecycle.observed_status, 0x2a);
            let waited = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let observed = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let cat = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let ps = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records()[0], Some(record));
            (exec, waited, observed, cat, ps)
        };
        let output = backend.as_str();

        assert_eq!(exec.line(), b"status42 alpha beta");
        assert_eq!(exec.status(), LocalCommandStatus::Handled);
        assert_eq!(exec.response_lines(), 10);
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert_eq!(cat.status(), LocalCommandStatus::Handled);
        assert_eq!(ps.status(), LocalCommandStatus::Handled);
        assert!(output.contains("talos> talos: exec path=/bin/status42 source=vfs-open-read\n"));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-startup-abi state=literal-argv-absolute-empty-envp argc=0x0000000000000003 argv0=/bin/status42 argv1=alpha argv2=beta argv0-ptr=0x00007fffffffffe0 argv-null=false envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 envp0-ptr=0x00007fffffffffd8 copied-startup-bytes=0x0000000000000049 source=initial-user-stack-record\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "slot=0 capacity=3 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true wait-consumed=true job-state=foreground source=bounded-process-table\n"
        ));
    }

    #[test_case]
    fn local_command_loop_runs_bare_name_vfs_command_with_readonly_stdin_redirection() {
        let bytes = *b"stdin </etc/banner.txt\rwaitpid\rlaststatus\rpipestatus\rcat /proc/talos/processes\rps\rstdout </etc/banner.txt\rstdin </dev/null\rstdin < /etc/banner.txt\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (exec, waited, observed, status, cat, ps, non_stdin, dev_null, spaced) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            let exec = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let record = io.process_table_records()[0].unwrap();
            assert_eq!(record.lifecycle.source_path, initramfs::PHASE10_STDIN_PATH);
            assert_eq!(record.lifecycle.status, 0);
            assert_eq!(record.lifecycle.observed_status, 0);
            let waited = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let observed = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let status = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let cat = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let ps = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let non_stdin = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let dev_null = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let spaced = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(io.process_table_records()[0], Some(record));
            assert_eq!(io.process_table_records()[1], None);
            (
                exec, waited, observed, status, cat, ps, non_stdin, dev_null, spaced,
            )
        };
        let output = backend.as_str();

        assert_eq!(exec.line(), b"stdin </etc/banner.txt");
        assert_eq!(exec.status(), LocalCommandStatus::Handled);
        assert_eq!(exec.response_lines(), 11);
        assert_eq!(waited.status(), LocalCommandStatus::Handled);
        assert_eq!(observed.status(), LocalCommandStatus::Handled);
        assert_eq!(status.status(), LocalCommandStatus::Handled);
        assert_eq!(cat.status(), LocalCommandStatus::Handled);
        assert_eq!(ps.status(), LocalCommandStatus::Handled);
        assert_eq!(non_stdin.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(dev_null.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(spaced.status(), LocalCommandStatus::UnexpectedArgument);
        assert!(output.contains("talos: exec path=/bin/stdin source=vfs-open-read\n"));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=regular-file fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-redirection op=source source-fd=0x0000000000000000 source-path=/etc/banner.txt source-stream=regular-file source-route=initramfs:/etc/banner.txt child-only=true shell-restored=true source=shell-redirection-stdin-etc-banner\n"
        ));
        assert!(output.contains(
            "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stdin"
        ));
        assert!(output.contains("Talos userspace stdin fixture read: Talos initramfs fixture\n"));
        assert!(output.contains(
            "talos: exec-stdin fd=0x0000000000000000 bytes=0x0000000000000018 return=0x0000000000000018 read-source=initramfs:/etc/banner.txt stdout-fd=0x0000000000000001 stdout-bytes=0x000000000000003d stdout-return=0x000000000000003d source=userspace-talos-read+userspace-talos-write read-result=regular-file-eof-after-read\n"
        ));
        assert!(output.contains(
            "talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(
            output
                .contains("talos: pipestatus none source=bounded-process-table-pipeline-status\n")
        );
        assert!(output.contains(
            "slot=0 capacity=3 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table\n"
        ));
        assert_eq!(output.matches("talos: exec-invalid-path\n").count(), 3);
    }

    #[test_case]
    fn local_command_loop_runs_bare_name_pipeline_producer_stdin_redirection() {
        let bytes = *b"stdin </etc/banner.txt | stdin\rwaitpid 0x100001\rwaitpid 0x100002\rlaststatus\rpipestatus\rcat /proc/talos/processes\rps\rstdin </etc/banner.txt | stdout\rstdin alpha </etc/banner.txt | stdin\rstdin </etc/banner.txt | stdin beta\rstdin </dev/null | stdin\rstdin < /etc/banner.txt | stdin\rstdin </etc/banner.txt | stdin | stdin\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (
            pipeline,
            producer_wait,
            consumer_wait,
            last,
            status,
            cat,
            ps,
            unsupported_consumer,
            producer_arg,
            consumer_arg,
            dev_null,
            spaced,
            multistage,
        ) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            let pipeline = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let producer_record = io.process_table_records()[0].unwrap();
            let consumer_record = io.process_table_records()[1].unwrap();
            assert_eq!(
                producer_record.lifecycle.process_id,
                LOCAL_COMMAND_PIPELINE_PRODUCER_PROCESS_ID
            );
            assert_eq!(
                producer_record.lifecycle.source_path,
                initramfs::PHASE10_STDIN_PATH
            );
            assert_eq!(
                consumer_record.lifecycle.process_id,
                LOCAL_COMMAND_PIPELINE_CONSUMER_PROCESS_ID
            );
            assert_eq!(
                consumer_record.lifecycle.source_path,
                initramfs::PHASE10_STDIN_PATH
            );
            assert_eq!(io.process_table_records()[2], None);
            (
                pipeline,
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
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
            )
        };
        let output = backend.as_str();

        assert_eq!(pipeline.line(), b"stdin </etc/banner.txt | stdin");
        assert_eq!(pipeline.status(), LocalCommandStatus::Handled);
        assert_eq!(pipeline.response_lines(), 23);
        assert_eq!(producer_wait.status(), LocalCommandStatus::Handled);
        assert_eq!(consumer_wait.status(), LocalCommandStatus::Handled);
        assert_eq!(last.status(), LocalCommandStatus::Handled);
        assert_eq!(status.status(), LocalCommandStatus::Handled);
        assert_eq!(cat.status(), LocalCommandStatus::Handled);
        assert_eq!(ps.status(), LocalCommandStatus::Handled);
        assert_eq!(
            unsupported_consumer.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert_eq!(
            producer_arg.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert_eq!(
            consumer_arg.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert_eq!(dev_null.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(spaced.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(multistage.status(), LocalCommandStatus::UnexpectedArgument);
        assert!(output.contains("talos: exec path=/bin/stdin source=vfs-open-read\n"));
        assert!(output.contains(
            "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stdin"
        ));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=regular-file fd1=pipe-endpoint fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=pipe-endpoint fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-redirection op=source source-fd=0x0000000000000000 source-path=/etc/banner.txt source-stream=regular-file source-route=initramfs:/etc/banner.txt child-only=true shell-restored=true source=shell-redirection-stdin-etc-banner\n"
        ));
        assert!(output.contains("Talos userspace stdin fixture read: Talos initramfs fixture\n"));
        assert!(output.contains(
            "talos: exec-stdin fd=0x0000000000000000 bytes=0x0000000000000018 return=0x0000000000000018 read-source=initramfs:/etc/banner.txt stdout-fd=0x0000000000000001 stdout-bytes=0x000000000000003d stdout-return=0x000000000000003d source=userspace-talos-read+userspace-talos-write read-result=regular-file-eof-after-read\n"
        ));
        assert!(output.contains(
            "talos: exec-stdin fd=0x0000000000000000 bytes=0x000000000000003d return=0x000000000000003d read-source=pipe:stdout-to-stdin stdout-fd=0x0000000000000001 stdout-bytes=0x0000000000000062 stdout-return=0x0000000000000062 source=userspace-talos-read+userspace-talos-write read-result=pipe-eof-after-writer-close\n"
        ));
        assert!(output.contains(
            "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdin consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000003d bytes-read=0x000000000000003d writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-producer-stdin-redirection-to-stdin\n"
        ));
        assert!(output.contains(
            "talos: pipeline-lifecycle-status record=phase12-local-pipeline-distinct-process-lifecycle-status-record-v1 pipeline=0x0000000000000001 producer-pid=0x0000000000100001 producer-path=/bin/stdin producer-state=exited producer-status=0x0000000000000000 producer-observed-status=0x0000000000000000 producer-reaped=true consumer-pid=0x0000000000100002 consumer-path=/bin/stdin consumer-state=exited consumer-status=0x0000000000000000 consumer-observed-status=0x0000000000000000 consumer-reaped=true source=kernel-owned-pipeline-lifecycle-status-record\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: pipestatus participants=0x0000000000000002 default-status=0x0000000000000000 pipefail-status=0x0000000000000000 semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status\n"
        ));
        assert!(output.contains(
            "slot=0 capacity=3 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table\n"
        ));
        assert!(output.contains(
            "slot=1 capacity=3 pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table\n"
        ));
        assert_eq!(output.matches("talos-processes-v1\n").count(), 2);
        assert_eq!(output.matches("talos: exec-invalid-path\n").count(), 6);
    }

    #[test_case]
    fn local_command_loop_runs_path_form_absolute_vfs_pipeline() {
        let bytes = *b"/bin/stdout | /bin/stdin\rwaitpid 0x100001\rwaitpid 0x100002\rlaststatus\rpipestatus\rcat /proc/talos/processes\rps\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (pipeline, producer_wait, consumer_wait, last, status, cat, ps) = {
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

        assert_eq!(pipeline.line(), b"/bin/stdout | /bin/stdin");
        assert_eq!(pipeline.status(), LocalCommandStatus::Handled);
        assert_eq!(producer_wait.status(), LocalCommandStatus::Handled);
        assert_eq!(consumer_wait.status(), LocalCommandStatus::Handled);
        assert_eq!(last.status(), LocalCommandStatus::Handled);
        assert_eq!(status.status(), LocalCommandStatus::Handled);
        assert_eq!(cat.status(), LocalCommandStatus::Handled);
        assert_eq!(ps.status(), LocalCommandStatus::Handled);
        assert!(output.contains("talos: exec path=/bin/stdout source=vfs-open-read\n"));
        assert!(output.contains("talos: exec path=/bin/stdin source=vfs-open-read\n"));
        assert!(output.contains(
            "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdout consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000001f bytes-read=0x000000000000001f writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-stdout-to-stdin\n"
        ));
        assert!(output.contains(
            "talos: pipeline-lifecycle-status record=phase12-local-pipeline-distinct-process-lifecycle-status-record-v1 pipeline=0x0000000000000001 producer-pid=0x0000000000100001 producer-path=/bin/stdout producer-state=exited producer-status=0x0000000000000000 producer-observed-status=0x0000000000000000 producer-reaped=true consumer-pid=0x0000000000100002 consumer-path=/bin/stdin consumer-state=exited consumer-status=0x0000000000000000 consumer-observed-status=0x0000000000000000 consumer-reaped=true source=kernel-owned-pipeline-lifecycle-status-record\n"
        ));
        assert!(
            output.contains("Talos userspace stdin fixture read: Talos userspace stdout fixture\n")
        );
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: pipestatus participants=0x0000000000000002 default-status=0x0000000000000000 pipefail-status=0x0000000000000000 semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status\n"
        ));
        assert!(output.contains(
            "slot=0 capacity=3 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table\n"
        ));
        assert!(output.contains(
            "slot=1 capacity=3 pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table\n"
        ));
        assert_eq!(output.matches("talos-processes-v1\n").count(), 2);
    }

    #[test_case]
    fn local_command_loop_redirects_direct_path_pipeline_consumer_stdout_to_volatile_regular_file()
    {
        let bytes = *b"/bin/stdout | /bin/stdin >/tmp/pipeline-report.txt\r/bin/stdout | /bin/stdin >>/tmp/pipeline-report.txt\rwaitpid 0x100001\rwaitpid 0x100002\rlaststatus\rpipestatus\rcat /proc/talos/processes\rps\rcat /tmp/pipeline-report.txt\r/bin/stdout\r/bin/stdout | /bin/stdin >/tmp/stdout.txt\rstdout | stdin >>/tmp/pipeline-report.txt\r/bin/stdout | /bin/stderr >>/tmp/pipeline-report.txt\r/bin/stdout | /bin/stdin 2>>/tmp/x\r/bin/stdout | /bin/stdin </etc/banner.txt\r/bin/stdout | /bin/stdin >> /tmp/pipeline-report.txt\r/bin/stdout | /bin/stdin >>/var/x\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (
            pipeline,
            append_pipeline,
            producer_wait,
            consumer_wait,
            last,
            status,
            cat_processes,
            ps,
            readback,
            normal,
            unsupported_path,
            bare_name_append,
            wrong_consumer,
            stderr_form,
            input_redirection,
            malformed_append,
            persistent_path,
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
            pipeline.line(),
            b"/bin/stdout | /bin/stdin >/tmp/pipeline-report.txt"
        );
        assert_eq!(
            append_pipeline.line(),
            b"/bin/stdout | /bin/stdin >>/tmp/pipeline-report.txt"
        );
        assert_eq!(pipeline.status(), LocalCommandStatus::Handled);
        assert_eq!(append_pipeline.status(), LocalCommandStatus::Handled);
        assert_eq!(producer_wait.status(), LocalCommandStatus::Handled);
        assert_eq!(consumer_wait.status(), LocalCommandStatus::Handled);
        assert_eq!(last.status(), LocalCommandStatus::Handled);
        assert_eq!(status.status(), LocalCommandStatus::Handled);
        assert_eq!(cat_processes.status(), LocalCommandStatus::Handled);
        assert_eq!(ps.status(), LocalCommandStatus::Handled);
        assert_eq!(readback.status(), LocalCommandStatus::Handled);
        assert_eq!(normal.status(), LocalCommandStatus::Handled);
        assert_eq!(bare_name_append.status(), LocalCommandStatus::Handled);
        for rejected in [
            unsupported_path,
            wrong_consumer,
            stderr_form,
            input_redirection,
            malformed_append,
            persistent_path,
        ] {
            assert_eq!(rejected.status(), LocalCommandStatus::UnexpectedArgument);
        }
        assert!(output.contains("talos: exec path=/bin/stdout source=vfs-open-read\n"));
        assert!(output.contains("talos: exec path=/bin/stdin source=vfs-open-read\n"));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=pipe-endpoint fd1=regular-file fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-redirection op=sink source-fd=0x0000000000000001 target-path=/tmp/pipeline-report.txt target-stream=regular-file target-route=volatile-vfs:/tmp/pipeline-report.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout\n"
        ));
        assert!(output.contains(
            "talos: exec-redirection op=append source-fd=0x0000000000000001 target-path=/tmp/pipeline-report.txt target-stream=regular-file target-route=volatile-vfs:/tmp/pipeline-report.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout-append\n"
        ));
        assert!(output.contains(
            "talos: exec-stdin fd=0x0000000000000000 bytes=0x000000000000001f return=0x000000000000001f read-source=pipe:stdout-to-stdin stdout-fd=0x0000000000000001"
        ));
        assert!(output.contains(
            "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdout consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000001f bytes-read=0x000000000000001f writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-consumer-stdout-redirection\n"
        ));
        assert!(output.contains(
            "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdout consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000001f bytes-read=0x000000000000001f writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-consumer-stdout-append-redirection\n"
        ));
        assert!(output.contains(
            "talos: cat path=/tmp/pipeline-report.txt bytes=0x0000000000000088 source=volatile-vfs-descriptor-read\n"
        ));
        assert_eq!(
            output
                .matches("Talos userspace stdin fixture read: Talos userspace stdout fixture\n")
                .count(),
            2
        );
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: pipestatus participants=0x0000000000000002 default-status=0x0000000000000000 pipefail-status=0x0000000000000000 semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status\n"
        ));
        assert!(output.contains(
            "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=stdout route=runtime-console0/stdout source=userspace-talos-write\n"
        ));
        assert_eq!(output.matches("talos-processes-v1\n").count(), 2);
        assert_eq!(output.matches("talos: exec-invalid-path").count(), 6);
    }

    #[test_case]
    fn local_command_loop_redirects_direct_path_pipeline_consumer_stderr_to_volatile_regular_file()
    {
        let bytes = *b"/bin/stdout | /bin/stderr 2>/tmp/pipeline-stderr.txt\r/bin/stdout | /bin/stderr 2>>/tmp/pipeline-stderr.txt\rwaitpid 0x100001\rwaitpid 0x100002\rlaststatus\rpipestatus\rcat /proc/talos/processes\rps\rcat /tmp/pipeline-stderr.txt\r/bin/stderr\r/bin/stdout | /bin/stderr >/tmp/pipeline-stderr.txt\r/bin/stdout | /bin/stderr </etc/banner.txt\r/bin/stdout | /bin/stderr 2>>/tmp/stderr.txt\r/bin/stdout | /bin/stderr 2>/tmp/stderr.txt\r/bin/stdout | /bin/stderr 2> /tmp/pipeline-stderr.txt\r/bin/stdout | /bin/stderr 2>>/var/x\rstdout | stderr 2>>/tmp/stderr.txt\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (
            pipeline,
            append_pipeline,
            producer_wait,
            consumer_wait,
            last,
            status,
            cat_processes,
            ps,
            readback,
            normal,
            stdout_form,
            input_redirection,
            append_command_stderr_path,
            command_stderr_path,
            malformed_spacing,
            persistent_path,
            bare_name_append,
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
            pipeline.line(),
            b"/bin/stdout | /bin/stderr 2>/tmp/pipeline-stderr.txt"
        );
        assert_eq!(
            append_pipeline.line(),
            b"/bin/stdout | /bin/stderr 2>>/tmp/pipeline-stderr.txt"
        );
        assert_eq!(pipeline.status(), LocalCommandStatus::Handled);
        assert_eq!(append_pipeline.status(), LocalCommandStatus::Handled);
        assert_eq!(producer_wait.status(), LocalCommandStatus::Handled);
        assert_eq!(consumer_wait.status(), LocalCommandStatus::Handled);
        assert_eq!(last.status(), LocalCommandStatus::Handled);
        assert_eq!(status.status(), LocalCommandStatus::Handled);
        assert_eq!(cat_processes.status(), LocalCommandStatus::Handled);
        assert_eq!(ps.status(), LocalCommandStatus::Handled);
        assert_eq!(readback.status(), LocalCommandStatus::Handled);
        assert_eq!(normal.status(), LocalCommandStatus::Handled);
        for rejected in [
            stdout_form,
            input_redirection,
            append_command_stderr_path,
            command_stderr_path,
            malformed_spacing,
            persistent_path,
            bare_name_append,
        ] {
            assert_eq!(rejected.status(), LocalCommandStatus::UnexpectedArgument);
        }
        assert!(output.contains("talos: exec path=/bin/stdout source=vfs-open-read\n"));
        assert!(output.contains("talos: exec path=/bin/stderr source=vfs-open-read\n"));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=pipe-endpoint fd1=stdio-output fd2=regular-file loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-redirection op=sink source-fd=0x0000000000000002 target-path=/tmp/pipeline-stderr.txt target-stream=regular-file target-route=volatile-vfs:/tmp/pipeline-stderr.txt child-only=true shell-restored=true source=shell-redirection-stderr-tmp-stderr\n"
        ));
        assert!(output.contains(
            "talos: exec-redirection op=append source-fd=0x0000000000000002 target-path=/tmp/pipeline-stderr.txt target-stream=regular-file target-route=volatile-vfs:/tmp/pipeline-stderr.txt child-only=true shell-restored=true source=shell-redirection-stderr-tmp-stderr-append\n"
        ));
        assert!(output.contains(
            "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=regular-file route=volatile-vfs:/tmp/pipeline-stderr.txt source=userspace-talos-write\n"
        ));
        assert!(output.contains(
            "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdout consumer-fd=0x0000000000000000 consumer-path=/bin/stderr bytes-written=0x000000000000001f bytes-read=0x0000000000000000 writer-closed=true reader-eof=false shell-restored=true source=shell-pipe-consumer-stderr-redirection\n"
        ));
        assert!(output.contains(
            "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdout consumer-fd=0x0000000000000000 consumer-path=/bin/stderr bytes-written=0x000000000000001f bytes-read=0x0000000000000000 writer-closed=true reader-eof=false shell-restored=true source=shell-pipe-consumer-stderr-append-redirection\n"
        ));
        assert!(output.contains(
            "talos: cat path=/tmp/pipeline-stderr.txt bytes=0x000000000000003e source=volatile-vfs-descriptor-read\n"
        ));
        assert_eq!(
            output.matches("Talos userspace stderr fixture\n").count(),
            3
        );
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: pipestatus participants=0x0000000000000002 default-status=0x0000000000000000 pipefail-status=0x0000000000000000 semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status\n"
        ));
        assert!(output.contains(
            "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=stderr route=runtime-console0/stderr source=userspace-talos-write\n"
        ));
        assert_eq!(output.matches("talos-processes-v1\n").count(), 2);
        assert_eq!(output.matches("talos: exec-invalid-path").count(), 7);
    }

    #[test_case]
    fn local_command_loop_redirects_bare_name_pipeline_consumer_stderr_to_volatile_regular_file() {
        let bytes = *b"stdout | stderr 2>/tmp/pipeline-stderr.txt\rstdout | stderr 2>>/tmp/pipeline-stderr.txt\rwaitpid 0x100001\rwaitpid 0x100002\rlaststatus\rpipestatus\rcat /proc/talos/processes\rps\rcat /tmp/pipeline-stderr.txt\rstderr\r/bin/stdout | /bin/stderr 2>>/tmp/pipeline-stderr.txt\rstdout | stderr >/tmp/pipeline-stderr.txt\rstdout | stderr </etc/banner.txt\rstdout | stderr 2>>/tmp/stderr.txt\rstdout | stderr 2> /tmp/pipeline-stderr.txt\rstdout | stderr 2>>/var/x\rstdout | bin/stderr 2>/tmp/pipeline-stderr.txt\r/bin/stdout | stderr 2>/tmp/pipeline-stderr.txt\rnosuch | stderr 2>/tmp/pipeline-stderr.txt\rstdout | nosuch 2>/tmp/pipeline-stderr.txt\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (
            pipeline,
            append_pipeline,
            producer_wait,
            consumer_wait,
            last,
            status,
            cat_processes,
            ps,
            readback,
            normal,
            direct_path_form,
            stdout_form,
            input_redirection,
            append_command_stderr_path,
            malformed_spacing,
            persistent_path,
            slash_consumer,
            path_producer,
            unsupported_producer,
            unsupported_consumer,
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
            pipeline.line(),
            b"stdout | stderr 2>/tmp/pipeline-stderr.txt"
        );
        assert_eq!(
            append_pipeline.line(),
            b"stdout | stderr 2>>/tmp/pipeline-stderr.txt"
        );
        assert_eq!(pipeline.status(), LocalCommandStatus::Handled);
        assert_eq!(append_pipeline.status(), LocalCommandStatus::Handled);
        assert_eq!(producer_wait.status(), LocalCommandStatus::Handled);
        assert_eq!(consumer_wait.status(), LocalCommandStatus::Handled);
        assert_eq!(last.status(), LocalCommandStatus::Handled);
        assert_eq!(status.status(), LocalCommandStatus::Handled);
        assert_eq!(cat_processes.status(), LocalCommandStatus::Handled);
        assert_eq!(ps.status(), LocalCommandStatus::Handled);
        assert_eq!(readback.status(), LocalCommandStatus::Handled);
        assert_eq!(normal.status(), LocalCommandStatus::Handled);
        assert_eq!(direct_path_form.status(), LocalCommandStatus::Handled);
        for rejected in [
            stdout_form,
            input_redirection,
            append_command_stderr_path,
            malformed_spacing,
            persistent_path,
            slash_consumer,
            path_producer,
        ] {
            assert_eq!(rejected.status(), LocalCommandStatus::UnexpectedArgument);
        }
        assert_eq!(
            unsupported_producer.status(),
            LocalCommandStatus::UnknownCommand
        );
        assert_eq!(
            unsupported_consumer.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert!(output.contains("talos: exec path=/bin/stdout source=vfs-open-read\n"));
        assert!(output.contains("talos: exec path=/bin/stderr source=vfs-open-read\n"));
        assert!(output.contains(
            "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stdout"
        ));
        assert!(output.contains(
            "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stderr"
        ));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=pipe-endpoint fd1=stdio-output fd2=regular-file loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-redirection op=sink source-fd=0x0000000000000002 target-path=/tmp/pipeline-stderr.txt target-stream=regular-file target-route=volatile-vfs:/tmp/pipeline-stderr.txt child-only=true shell-restored=true source=shell-redirection-stderr-tmp-stderr\n"
        ));
        assert!(output.contains(
            "talos: exec-redirection op=append source-fd=0x0000000000000002 target-path=/tmp/pipeline-stderr.txt target-stream=regular-file target-route=volatile-vfs:/tmp/pipeline-stderr.txt child-only=true shell-restored=true source=shell-redirection-stderr-tmp-stderr-append\n"
        ));
        assert!(output.contains(
            "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=regular-file route=volatile-vfs:/tmp/pipeline-stderr.txt source=userspace-talos-write\n"
        ));
        assert!(output.contains(
            "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdout consumer-fd=0x0000000000000000 consumer-path=/bin/stderr bytes-written=0x000000000000001f bytes-read=0x0000000000000000 writer-closed=true reader-eof=false shell-restored=true source=shell-pipe-consumer-stderr-redirection\n"
        ));
        assert!(output.contains(
            "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdout consumer-fd=0x0000000000000000 consumer-path=/bin/stderr bytes-written=0x000000000000001f bytes-read=0x0000000000000000 writer-closed=true reader-eof=false shell-restored=true source=shell-pipe-consumer-stderr-append-redirection\n"
        ));
        assert!(output.contains(
            "talos: cat path=/tmp/pipeline-stderr.txt bytes=0x000000000000003e source=volatile-vfs-descriptor-read\n"
        ));
        assert_eq!(
            output.matches("Talos userspace stderr fixture\n").count(),
            3
        );
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stderr state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: pipestatus participants=0x0000000000000002 default-status=0x0000000000000000 pipefail-status=0x0000000000000000 semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status\n"
        ));
        assert!(output.contains(
            "talos: exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f stream=stderr route=runtime-console0/stderr source=userspace-talos-write\n"
        ));
        assert_eq!(output.matches("talos-processes-v1\n").count(), 2);
        assert_eq!(output.matches("talos: exec-invalid-path").count(), 8);
    }

    #[test_case]
    fn local_command_loop_redirects_bare_name_pipeline_consumer_stdout_to_volatile_regular_file() {
        let bytes = *b"stdout | stdin >/tmp/pipeline-report.txt\rstdout | stdin >>/tmp/pipeline-report.txt\rwaitpid 0x100001\rwaitpid 0x100002\rlaststatus\rpipestatus\rcat /proc/talos/processes\rps\rcat /tmp/pipeline-report.txt\rstdout\rstdout | stdin >/tmp/stdout.txt\rstdout | stderr >/tmp/pipeline-report.txt\rstdout | stdin 1>/tmp/pipeline-report.txt\rstdout | stdin > /tmp/pipeline-report.txt\rstdout | bin/stdin >/tmp/pipeline-report.txt\rstdout | stdin 2>>/tmp/x\rstdout | stdin </etc/banner.txt\rstdout | stdin >> /tmp/pipeline-report.txt\rstdout | stdin >>/var/x\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (
            pipeline,
            append_pipeline,
            producer_wait,
            consumer_wait,
            last,
            status,
            cat_processes,
            ps,
            readback,
            normal,
            unsupported_path,
            wrong_consumer,
            explicit_fd,
            spaced_output,
            slash_consumer,
            stderr_form,
            input_redirection,
            malformed_append,
            persistent_path,
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

        assert_eq!(pipeline.line(), b"stdout | stdin >/tmp/pipeline-report.txt");
        assert_eq!(
            append_pipeline.line(),
            b"stdout | stdin >>/tmp/pipeline-report.txt"
        );
        assert_eq!(pipeline.status(), LocalCommandStatus::Handled);
        assert_eq!(append_pipeline.status(), LocalCommandStatus::Handled);
        assert_eq!(producer_wait.status(), LocalCommandStatus::Handled);
        assert_eq!(consumer_wait.status(), LocalCommandStatus::Handled);
        assert_eq!(last.status(), LocalCommandStatus::Handled);
        assert_eq!(status.status(), LocalCommandStatus::Handled);
        assert_eq!(cat_processes.status(), LocalCommandStatus::Handled);
        assert_eq!(ps.status(), LocalCommandStatus::Handled);
        assert_eq!(readback.status(), LocalCommandStatus::Handled);
        assert_eq!(normal.status(), LocalCommandStatus::Handled);
        for rejected in [
            unsupported_path,
            wrong_consumer,
            explicit_fd,
            spaced_output,
            slash_consumer,
            stderr_form,
            input_redirection,
            malformed_append,
            persistent_path,
        ] {
            assert_eq!(rejected.status(), LocalCommandStatus::UnexpectedArgument);
        }
        assert!(output.contains("talos: exec path=/bin/stdout source=vfs-open-read\n"));
        assert!(output.contains("talos: exec path=/bin/stdin source=vfs-open-read\n"));
        assert!(output.contains(
            "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stdout"
        ));
        assert!(output.contains(
            "talos: exec-startup-abi state=minimal-argc1-argv0-absolute-empty-envp argc=0x0000000000000001 argv0=/bin/stdin"
        ));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=pipe-endpoint fd1=regular-file fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-redirection op=sink source-fd=0x0000000000000001 target-path=/tmp/pipeline-report.txt target-stream=regular-file target-route=volatile-vfs:/tmp/pipeline-report.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout\n"
        ));
        assert!(output.contains(
            "talos: exec-redirection op=append source-fd=0x0000000000000001 target-path=/tmp/pipeline-report.txt target-stream=regular-file target-route=volatile-vfs:/tmp/pipeline-report.txt child-only=true shell-restored=true source=shell-redirection-stdout-tmp-stdout-append\n"
        ));
        assert!(output.contains(
            "talos: exec-stdin fd=0x0000000000000000 bytes=0x000000000000001f return=0x000000000000001f read-source=pipe:stdout-to-stdin stdout-fd=0x0000000000000001"
        ));
        assert!(output.contains(
            "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdout consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000001f bytes-read=0x000000000000001f writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-consumer-stdout-redirection\n"
        ));
        assert!(output.contains(
            "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdout consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000001f bytes-read=0x000000000000001f writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-consumer-stdout-append-redirection\n"
        ));
        assert!(output.contains(
            "talos: cat path=/tmp/pipeline-report.txt bytes=0x0000000000000088 source=volatile-vfs-descriptor-read\n"
        ));
        assert_eq!(
            output
                .matches("Talos userspace stdin fixture read: Talos userspace stdout fixture\n")
                .count(),
            2
        );
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: pipestatus participants=0x0000000000000002 default-status=0x0000000000000000 pipefail-status=0x0000000000000000 semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status\n"
        ));
        assert!(output.contains(
            "talos: exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f stream=stdout route=runtime-console0/stdout source=userspace-talos-write\n"
        ));
        assert_eq!(output.matches("talos-processes-v1\n").count(), 2);
        assert_eq!(output.matches("talos: exec-invalid-path").count(), 9);
    }

    #[test_case]
    fn local_command_loop_runs_path_form_absolute_vfs_pipeline_with_literal_stage_argv() {
        let bytes = *b"/bin/stdout alpha | /bin/stdin beta\rwaitpid 0x100001\rwaitpid 0x100002\rlaststatus\rpipestatus\rcat /proc/talos/processes\rps\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (pipeline, producer_wait, consumer_wait, last, status, cat, ps) = {
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

        assert_eq!(pipeline.line(), b"/bin/stdout alpha | /bin/stdin beta");
        assert_eq!(pipeline.status(), LocalCommandStatus::Handled);
        assert_eq!(producer_wait.status(), LocalCommandStatus::Handled);
        assert_eq!(consumer_wait.status(), LocalCommandStatus::Handled);
        assert_eq!(last.status(), LocalCommandStatus::Handled);
        assert_eq!(status.status(), LocalCommandStatus::Handled);
        assert_eq!(cat.status(), LocalCommandStatus::Handled);
        assert_eq!(ps.status(), LocalCommandStatus::Handled);
        assert!(output.contains("talos: exec path=/bin/stdout source=vfs-open-read\n"));
        assert!(output.contains("talos: exec path=/bin/stdin source=vfs-open-read\n"));
        assert!(output.contains(
            "talos: exec-startup-abi state=literal-argv-absolute-empty-envp argc=0x0000000000000002 argv0=/bin/stdout argv1=alpha"
        ));
        assert!(output.contains(
            "talos: exec-startup-abi state=literal-argv-absolute-empty-envp argc=0x0000000000000002 argv0=/bin/stdin argv1=beta"
        ));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=pipe-endpoint fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=pipe-endpoint fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdout consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000001f bytes-read=0x000000000000001f writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-stdout-to-stdin\n"
        ));
        assert!(output.contains(
            "talos: pipeline-lifecycle-status record=phase12-local-pipeline-distinct-process-lifecycle-status-record-v1 pipeline=0x0000000000000001 producer-pid=0x0000000000100001 producer-path=/bin/stdout producer-state=exited producer-status=0x0000000000000000 producer-observed-status=0x0000000000000000 producer-reaped=true consumer-pid=0x0000000000100002 consumer-path=/bin/stdin consumer-state=exited consumer-status=0x0000000000000000 consumer-observed-status=0x0000000000000000 consumer-reaped=true source=kernel-owned-pipeline-lifecycle-status-record\n"
        ));
        assert!(
            output.contains("Talos userspace stdin fixture read: Talos userspace stdout fixture\n")
        );
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: pipestatus participants=0x0000000000000002 default-status=0x0000000000000000 pipefail-status=0x0000000000000000 semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status\n"
        ));
        assert!(output.contains(
            "slot=0 capacity=3 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table\n"
        ));
        assert!(output.contains(
            "slot=1 capacity=3 pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table\n"
        ));
        assert_eq!(output.matches("talos-processes-v1\n").count(), 2);
    }

    #[test_case]
    fn local_command_loop_runs_bare_name_vfs_pipeline_through_bounded_bin_lookup() {
        let bytes = *b"stdout | stdin\rwaitpid 0x100001\rwaitpid 0x100002\rlaststatus\rpipestatus\rcat /proc/talos/processes\rps\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (pipeline, producer_wait, consumer_wait, last, status, cat, ps) = {
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

        assert_eq!(pipeline.line(), b"stdout | stdin");
        assert_eq!(pipeline.status(), LocalCommandStatus::Handled);
        assert_eq!(producer_wait.status(), LocalCommandStatus::Handled);
        assert_eq!(consumer_wait.status(), LocalCommandStatus::Handled);
        assert_eq!(last.status(), LocalCommandStatus::Handled);
        assert_eq!(status.status(), LocalCommandStatus::Handled);
        assert_eq!(cat.status(), LocalCommandStatus::Handled);
        assert_eq!(ps.status(), LocalCommandStatus::Handled);
        assert!(output.contains("talos: exec path=/bin/stdout source=vfs-open-read\n"));
        assert!(output.contains("talos: exec path=/bin/stdin source=vfs-open-read\n"));
        assert!(output.contains(
            "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdout consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000001f bytes-read=0x000000000000001f writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-stdout-to-stdin\n"
        ));
        assert!(output.contains(
            "talos: pipeline-lifecycle-status record=phase12-local-pipeline-distinct-process-lifecycle-status-record-v1 pipeline=0x0000000000000001 producer-pid=0x0000000000100001 producer-path=/bin/stdout producer-state=exited producer-status=0x0000000000000000 producer-observed-status=0x0000000000000000 producer-reaped=true consumer-pid=0x0000000000100002 consumer-path=/bin/stdin consumer-state=exited consumer-status=0x0000000000000000 consumer-observed-status=0x0000000000000000 consumer-reaped=true source=kernel-owned-pipeline-lifecycle-status-record\n"
        ));
        assert!(
            output.contains("Talos userspace stdin fixture read: Talos userspace stdout fixture\n")
        );
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: pipestatus participants=0x0000000000000002 default-status=0x0000000000000000 pipefail-status=0x0000000000000000 semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status\n"
        ));
        assert!(output.contains(
            "slot=0 capacity=3 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table\n"
        ));
        assert!(output.contains(
            "slot=1 capacity=3 pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table\n"
        ));
        assert_eq!(output.matches("talos-processes-v1\n").count(), 2);
    }

    #[test_case]
    fn local_command_loop_runs_bare_name_vfs_pipeline_with_literal_stage_argv() {
        let bytes = *b"stdout alpha | stdin beta\rwaitpid 0x100001\rwaitpid 0x100002\rlaststatus\rpipestatus\rcat /proc/talos/processes\rps\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (pipeline, producer_wait, consumer_wait, last, status, cat, ps) = {
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

        assert_eq!(pipeline.line(), b"stdout alpha | stdin beta");
        assert_eq!(pipeline.status(), LocalCommandStatus::Handled);
        assert_eq!(producer_wait.status(), LocalCommandStatus::Handled);
        assert_eq!(consumer_wait.status(), LocalCommandStatus::Handled);
        assert_eq!(last.status(), LocalCommandStatus::Handled);
        assert_eq!(status.status(), LocalCommandStatus::Handled);
        assert_eq!(cat.status(), LocalCommandStatus::Handled);
        assert_eq!(ps.status(), LocalCommandStatus::Handled);
        assert!(output.contains("talos: exec path=/bin/stdout source=vfs-open-read\n"));
        assert!(output.contains("talos: exec path=/bin/stdin source=vfs-open-read\n"));
        assert!(output.contains(
            "talos: exec-startup-abi state=literal-argv-absolute-empty-envp argc=0x0000000000000002 argv0=/bin/stdout argv1=alpha"
        ));
        assert!(output.contains(
            "talos: exec-startup-abi state=literal-argv-absolute-empty-envp argc=0x0000000000000002 argv0=/bin/stdin argv1=beta"
        ));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=stdio-input fd1=pipe-endpoint fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=pipe-endpoint fd1=stdio-output fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdout consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000001f bytes-read=0x000000000000001f writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-stdout-to-stdin\n"
        ));
        assert!(output.contains(
            "talos: pipeline-lifecycle-status record=phase12-local-pipeline-distinct-process-lifecycle-status-record-v1 pipeline=0x0000000000000001 producer-pid=0x0000000000100001 producer-path=/bin/stdout producer-state=exited producer-status=0x0000000000000000 producer-observed-status=0x0000000000000000 producer-reaped=true consumer-pid=0x0000000000100002 consumer-path=/bin/stdin consumer-state=exited consumer-status=0x0000000000000000 consumer-observed-status=0x0000000000000000 consumer-reaped=true source=kernel-owned-pipeline-lifecycle-status-record\n"
        ));
        assert!(
            output.contains("Talos userspace stdin fixture read: Talos userspace stdout fixture\n")
        );
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: pipestatus participants=0x0000000000000002 default-status=0x0000000000000000 pipefail-status=0x0000000000000000 semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status\n"
        ));
        assert!(output.contains(
            "slot=0 capacity=3 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table\n"
        ));
        assert!(output.contains(
            "slot=1 capacity=3 pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table\n"
        ));
        assert_eq!(output.matches("talos-processes-v1\n").count(), 2);
    }

    #[test_case]
    fn local_command_loop_rejects_unsupported_path_form_pipeline_forms() {
        let bytes = *b"/bin/stdout | exec stdin\rexec stdout | /bin/stdin\rstatus42 | /bin/stdin\r/bin/stdout | /missing\r/bin/stdout | /bin/stdin | /bin/stdin\r/bin/stdout alpha beta | /bin/stdin\r/bin/stdout * | /bin/stdin\r/bin/stdout alpha | /bin/stdin beta gamma\r/bin/stdout alpha | /bin/stdin *\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (
            mixed_consumer,
            mixed_producer,
            bare_producer,
            unsupported_consumer,
            multistage,
            producer_too_many_args,
            producer_invalid_arg,
            consumer_too_many_args,
            consumer_invalid_arg,
        ) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            let mixed_consumer = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(
                io.process_table_records(),
                [None; LOCAL_COMMAND_PROCESS_TABLE_CAPACITY]
            );
            let mixed_producer = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(
                io.process_table_records(),
                [None; LOCAL_COMMAND_PROCESS_TABLE_CAPACITY]
            );
            let bare_producer = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(
                io.process_table_records(),
                [None; LOCAL_COMMAND_PROCESS_TABLE_CAPACITY]
            );
            let unsupported_consumer = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(
                io.process_table_records(),
                [None; LOCAL_COMMAND_PROCESS_TABLE_CAPACITY]
            );
            let multistage = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(
                io.process_table_records(),
                [None; LOCAL_COMMAND_PROCESS_TABLE_CAPACITY]
            );
            let producer_too_many_args = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(
                io.process_table_records(),
                [None; LOCAL_COMMAND_PROCESS_TABLE_CAPACITY]
            );
            let producer_invalid_arg = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(
                io.process_table_records(),
                [None; LOCAL_COMMAND_PROCESS_TABLE_CAPACITY]
            );
            let consumer_too_many_args = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(
                io.process_table_records(),
                [None; LOCAL_COMMAND_PROCESS_TABLE_CAPACITY]
            );
            let consumer_invalid_arg = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(
                io.process_table_records(),
                [None; LOCAL_COMMAND_PROCESS_TABLE_CAPACITY]
            );
            (
                mixed_consumer,
                mixed_producer,
                bare_producer,
                unsupported_consumer,
                multistage,
                producer_too_many_args,
                producer_invalid_arg,
                consumer_too_many_args,
                consumer_invalid_arg,
            )
        };
        let output = backend.as_str();

        assert_eq!(mixed_consumer.line(), b"/bin/stdout | exec stdin");
        assert_eq!(
            mixed_consumer.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert_eq!(mixed_producer.line(), b"exec stdout | /bin/stdin");
        assert_eq!(
            mixed_producer.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert_eq!(bare_producer.line(), b"status42 | /bin/stdin");
        assert_eq!(
            bare_producer.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert_eq!(unsupported_consumer.line(), b"/bin/stdout | /missing");
        assert_eq!(
            unsupported_consumer.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert_eq!(multistage.line(), b"/bin/stdout | /bin/stdin | /bin/stdin");
        assert_eq!(multistage.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(
            producer_too_many_args.line(),
            b"/bin/stdout alpha beta | /bin/stdin"
        );
        assert_eq!(
            producer_too_many_args.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert_eq!(producer_invalid_arg.line(), b"/bin/stdout * | /bin/stdin");
        assert_eq!(
            producer_invalid_arg.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert_eq!(
            consumer_too_many_args.line(),
            b"/bin/stdout alpha | /bin/stdin beta gamma"
        );
        assert_eq!(
            consumer_too_many_args.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert_eq!(
            consumer_invalid_arg.line(),
            b"/bin/stdout alpha | /bin/stdin *"
        );
        assert_eq!(
            consumer_invalid_arg.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert_eq!(output.matches("talos: exec-invalid-path").count(), 9);
        assert_eq!(output.matches("talos: unknown-command").count(), 0);
    }

    #[test_case]
    fn local_command_loop_rejects_unsupported_bare_name_pipeline_forms() {
        let bytes = *b"stdout | /bin/stdin\r/bin/stdout | stdin\rmissing | stdin\rstdout | missing\rstdout 1>&2 | stdin\rstdout | stdin 1>/tmp/stdout.txt\rstdout | stdin | stdin\rstdout alpha beta | stdin\rstdout * | stdin\rstdout alpha | stdin beta gamma\rstdout alpha | stdin *\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (
            path_consumer,
            path_producer,
            missing_producer,
            missing_consumer,
            producer_redirection,
            consumer_redirection,
            multistage,
            producer_too_many_args,
            producer_invalid_arg,
            consumer_too_many_args,
            consumer_invalid_arg,
        ) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            let path_consumer = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(
                io.process_table_records(),
                [None; LOCAL_COMMAND_PROCESS_TABLE_CAPACITY]
            );
            let path_producer = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(
                io.process_table_records(),
                [None; LOCAL_COMMAND_PROCESS_TABLE_CAPACITY]
            );
            let missing_producer = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(
                io.process_table_records(),
                [None; LOCAL_COMMAND_PROCESS_TABLE_CAPACITY]
            );
            let missing_consumer = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(
                io.process_table_records(),
                [None; LOCAL_COMMAND_PROCESS_TABLE_CAPACITY]
            );
            let producer_redirection = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(
                io.process_table_records(),
                [None; LOCAL_COMMAND_PROCESS_TABLE_CAPACITY]
            );
            let consumer_redirection = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(
                io.process_table_records(),
                [None; LOCAL_COMMAND_PROCESS_TABLE_CAPACITY]
            );
            let multistage = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(
                io.process_table_records(),
                [None; LOCAL_COMMAND_PROCESS_TABLE_CAPACITY]
            );
            let producer_too_many_args = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(
                io.process_table_records(),
                [None; LOCAL_COMMAND_PROCESS_TABLE_CAPACITY]
            );
            let producer_invalid_arg = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(
                io.process_table_records(),
                [None; LOCAL_COMMAND_PROCESS_TABLE_CAPACITY]
            );
            let consumer_too_many_args = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(
                io.process_table_records(),
                [None; LOCAL_COMMAND_PROCESS_TABLE_CAPACITY]
            );
            let consumer_invalid_arg = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(
                io.process_table_records(),
                [None; LOCAL_COMMAND_PROCESS_TABLE_CAPACITY]
            );
            (
                path_consumer,
                path_producer,
                missing_producer,
                missing_consumer,
                producer_redirection,
                consumer_redirection,
                multistage,
                producer_too_many_args,
                producer_invalid_arg,
                consumer_too_many_args,
                consumer_invalid_arg,
            )
        };
        let output = backend.as_str();

        assert_eq!(path_consumer.line(), b"stdout | /bin/stdin");
        assert_eq!(
            path_consumer.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert_eq!(path_producer.line(), b"/bin/stdout | stdin");
        assert_eq!(
            path_producer.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert_eq!(missing_producer.line(), b"missing | stdin");
        assert_eq!(
            missing_producer.status(),
            LocalCommandStatus::UnknownCommand
        );
        assert_eq!(missing_consumer.line(), b"stdout | missing");
        assert_eq!(
            missing_consumer.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert_eq!(producer_redirection.line(), b"stdout 1>&2 | stdin");
        assert_eq!(
            producer_redirection.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert_eq!(
            consumer_redirection.line(),
            b"stdout | stdin 1>/tmp/stdout.txt"
        );
        assert_eq!(
            consumer_redirection.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert_eq!(multistage.line(), b"stdout | stdin | stdin");
        assert_eq!(multistage.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(producer_too_many_args.line(), b"stdout alpha beta | stdin");
        assert_eq!(
            producer_too_many_args.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert_eq!(producer_invalid_arg.line(), b"stdout * | stdin");
        assert_eq!(
            producer_invalid_arg.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert_eq!(
            consumer_too_many_args.line(),
            b"stdout alpha | stdin beta gamma"
        );
        assert_eq!(
            consumer_too_many_args.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert_eq!(consumer_invalid_arg.line(), b"stdout alpha | stdin *");
        assert_eq!(
            consumer_invalid_arg.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert_eq!(output.matches("talos: exec-invalid-path").count(), 10);
        assert_eq!(output.matches("talos: unknown-command").count(), 1);
    }

    #[test_case]
    fn local_command_loop_rejects_unsupported_direct_path_commands_without_process_records() {
        let bytes =
            *b"/missing\rbin/status42\r/bin\r/etc/banner.txt\rstatus42 alpha beta gamma delta\rstatus42 *\r/bin/status42 alpha beta gamma delta\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (missing, relative, directory, banner, bare_too_many, bare_invalid, path_extra) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            let missing = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(
                io.process_table_records(),
                [None; LOCAL_COMMAND_PROCESS_TABLE_CAPACITY]
            );
            let relative = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(
                io.process_table_records(),
                [None; LOCAL_COMMAND_PROCESS_TABLE_CAPACITY]
            );
            let directory = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(
                io.process_table_records(),
                [None; LOCAL_COMMAND_PROCESS_TABLE_CAPACITY]
            );
            let banner = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(
                io.process_table_records(),
                [None; LOCAL_COMMAND_PROCESS_TABLE_CAPACITY]
            );
            let bare_too_many = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(
                io.process_table_records(),
                [None; LOCAL_COMMAND_PROCESS_TABLE_CAPACITY]
            );
            let bare_invalid = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(
                io.process_table_records(),
                [None; LOCAL_COMMAND_PROCESS_TABLE_CAPACITY]
            );
            let path_extra = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            assert_eq!(
                io.process_table_records(),
                [None; LOCAL_COMMAND_PROCESS_TABLE_CAPACITY]
            );
            (
                missing,
                relative,
                directory,
                banner,
                bare_too_many,
                bare_invalid,
                path_extra,
            )
        };

        assert_eq!(missing.line(), b"/missing");
        assert_eq!(missing.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(relative.line(), b"bin/status42");
        assert_eq!(relative.status(), LocalCommandStatus::UnknownCommand);
        assert_eq!(directory.line(), b"/bin");
        assert_eq!(directory.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(banner.line(), b"/etc/banner.txt");
        assert_eq!(banner.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(bare_too_many.line(), b"status42 alpha beta gamma delta");
        assert_eq!(
            bare_too_many.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert_eq!(bare_invalid.line(), b"status42 *");
        assert_eq!(
            bare_invalid.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert_eq!(path_extra.line(), b"/bin/status42 alpha beta gamma delta");
        assert_eq!(path_extra.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(
            backend.as_str(),
            "talos> talos: exec-not-found\n\
talos> talos: unknown-command\n\
talos> talos: exec-not-executable\n\
talos> talos: exec-not-executable\n\
talos> talos: exec-invalid-path\n\
talos> talos: exec-invalid-path\n\
talos> talos: exec-invalid-path\n"
        );
    }

    #[test_case]
    fn local_command_loop_cats_proc_talos_processes_after_direct_vfs_exec() {
        let bytes = *b"exec /bin/status42\rcat /proc/talos/processes\rwaitpid\rcat /proc/talos/processes\rcat /proc/talos\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (exec, first_cat, wait, second_cat, missing_proc) = {
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

        assert_eq!(exec.line(), b"exec /bin/status42");
        assert_eq!(exec.status(), LocalCommandStatus::Handled);
        assert_eq!(first_cat.line(), b"cat /proc/talos/processes");
        assert_eq!(first_cat.status(), LocalCommandStatus::Handled);
        assert_eq!(first_cat.response_lines(), 1);
        assert_eq!(wait.status(), LocalCommandStatus::Handled);
        assert_eq!(second_cat.status(), LocalCommandStatus::Handled);
        assert_eq!(missing_proc.line(), b"cat /proc/talos");
        assert_eq!(
            missing_proc.status(),
            LocalCommandStatus::UnexpectedArgument
        );
        assert!(output.contains("talos-processes-v1\n"));
        assert!(output.contains(
            "slot=0 capacity=3 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true wait-consumed=false job-state=foreground source=bounded-process-table\n"
        ));
        assert!(output.contains(
            "slot=0 capacity=3 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true wait-consumed=true job-state=foreground source=bounded-process-table\n"
        ));
        assert!(output.contains("talos> talos: not-found\n"));
    }

    #[test_case]
    fn local_command_loop_cats_proc_talos_processes_after_pipeline_vfs_exec() {
        let bytes = *b"exec stdout | exec stdin\rcat /proc/talos/processes\rwaitpid 0x100001\rcat /proc/talos/processes\rwaitpid 0x100002\rcat /proc/talos/processes\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (pipeline, first_cat, producer_wait, second_cat, consumer_wait, third_cat) = {
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

        assert_eq!(pipeline.status(), LocalCommandStatus::Handled);
        assert_eq!(first_cat.status(), LocalCommandStatus::Handled);
        assert_eq!(producer_wait.status(), LocalCommandStatus::Handled);
        assert_eq!(second_cat.status(), LocalCommandStatus::Handled);
        assert_eq!(consumer_wait.status(), LocalCommandStatus::Handled);
        assert_eq!(third_cat.status(), LocalCommandStatus::Handled);
        assert!(output.contains(
            "slot=0 capacity=3 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=false job-state=foreground source=bounded-process-table\n"
        ));
        assert!(output.contains(
            "slot=1 capacity=3 pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=false job-state=foreground source=bounded-process-table\n"
        ));
        assert!(output.contains(
            "slot=0 capacity=3 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table\n"
        ));
        assert!(output.contains(
            "slot=1 capacity=3 pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table\n"
        ));
    }

    #[test_case]
    fn local_command_loop_runs_three_stage_pipeline_through_bounded_vfs_processes() {
        let bytes = *b"exec stdout | exec stdin | exec stdin\rwaitpid 0x100001\rwaitpid 0x100002\rwaitpid 0x100003\rwaitpid\rlaststatus\rcat /proc/talos/processes\rps\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (pipeline, records, wait_producer, wait_middle, wait_final, wait_none, last, cat, ps) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            let pipeline = run_one_descriptor_backed_serial_command(&mut io).unwrap();
            let records = io.process_table_records();
            (
                pipeline,
                records,
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

        assert_eq!(pipeline.line(), b"exec stdout | exec stdin | exec stdin");
        assert_eq!(pipeline.status(), LocalCommandStatus::Handled);
        assert_eq!(wait_producer.status(), LocalCommandStatus::Handled);
        assert_eq!(wait_middle.status(), LocalCommandStatus::Handled);
        assert_eq!(wait_final.status(), LocalCommandStatus::Handled);
        assert_eq!(wait_none.status(), LocalCommandStatus::Handled);
        assert_eq!(last.status(), LocalCommandStatus::Handled);
        assert_eq!(cat.status(), LocalCommandStatus::Handled);
        assert_eq!(ps.status(), LocalCommandStatus::Handled);
        assert_eq!(cat.response_lines(), 1);
        assert_eq!(ps.response_lines(), 1);
        let [producer, middle, final_stage] = records;
        let producer = producer.unwrap();
        let middle = middle.unwrap();
        let final_stage = final_stage.unwrap();
        assert_eq!(producer.lifecycle.process_id, 0x100001);
        assert_eq!(
            producer.lifecycle.source_path,
            initramfs::PHASE10_STDOUT_PATH
        );
        assert_eq!(middle.lifecycle.process_id, 0x100002);
        assert_eq!(middle.lifecycle.source_path, initramfs::PHASE10_STDIN_PATH);
        assert_eq!(final_stage.lifecycle.process_id, 0x100003);
        assert_eq!(
            final_stage.lifecycle.source_path,
            initramfs::PHASE10_STDIN_PATH
        );
        assert!(output.contains(
            "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/stdout consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x000000000000001f bytes-read=0x000000000000001f writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-multistage-first-stdout-to-stdin\n"
        ));
        assert!(output.contains(
            "talos: pipeline id=0x0000000000000002 producer-fd=0x0000000000000001 producer-path=/bin/stdin consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x0000000000000044 bytes-read=0x0000000000000044 writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-multistage-middle-to-stdin\n"
        ));
        assert!(output.contains(
            "talos: pipeline-lifecycle-status record=phase12-local-multistage-pipeline-lifecycle-status-record-v1 pipeline=0x0000000000000001 producer-pid=0x0000000000100001 producer-path=/bin/stdout producer-state=exited producer-status=0x0000000000000000 producer-observed-status=0x0000000000000000 producer-reaped=true middle-pid=0x0000000000100002 middle-path=/bin/stdin middle-state=exited middle-status=0x0000000000000000 middle-observed-status=0x0000000000000000 middle-reaped=true consumer-pid=0x0000000000100003 consumer-path=/bin/stdin consumer-state=exited consumer-status=0x0000000000000000 consumer-observed-status=0x0000000000000000 consumer-reaped=true source=kernel-owned-pipeline-lifecycle-status-record\n"
        ));
        assert!(output.contains(
            "talos: exec-descriptors owner=0x0000000000000001 inherited-count=0x0000000000000003 fd0=pipe-endpoint fd1=pipe-endpoint fd2=stdio-output loader-temp-fd=0x0000000000000003 loader-temp-open=false source=shell-process-descriptor-table\n"
        ));
        assert!(output.contains(
            "talos: exec-stdin fd=0x0000000000000000 bytes=0x000000000000001f return=0x000000000000001f read-source=pipe:stdout-to-stdin stdout-fd=0x0000000000000001 stdout-bytes=0x0000000000000044 stdout-return=0x0000000000000044 source=userspace-talos-read+userspace-talos-write read-result=pipe-eof-after-writer-close\n"
        ));
        assert!(output.contains(
            "talos: exec-stdin fd=0x0000000000000000 bytes=0x0000000000000044 return=0x0000000000000044 read-source=pipe:middle-to-stdin stdout-fd=0x0000000000000001 stdout-bytes=0x0000000000000069 stdout-return=0x0000000000000069 source=userspace-talos-read+userspace-talos-write read-result=pipe-eof-after-writer-close\n"
        ));
        assert!(output.contains(
            "Talos userspace stdin fixture read: Talos userspace stdin fixture read: Talos userspace stdout fixture\n\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record\n"
        ));
        assert!(output.contains(
            "talos> talos: waitpid pid=0x0000000000100003 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=explicit-pid-lifecycle-record\n"
        ));
        assert!(output.contains("talos> talos: waitpid no-child source=lifecycle-record\n"));
        assert!(output.contains(
            "slot=0 capacity=3 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/stdout state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table\n"
        ));
        assert!(output.contains(
            "slot=1 capacity=3 pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table\n"
        ));
        assert!(output.contains(
            "slot=2 capacity=3 pid=0x0000000000100003 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true wait-consumed=true job-state=foreground source=bounded-process-table\n"
        ));
        assert_eq!(output.matches("talos-processes-v1\n").count(), 2);
    }

    #[test_case]
    fn local_command_loop_reports_bounded_pipeline_status_from_process_table() {
        let bytes = *b"pipestatus\rexec stdout | exec stdin\rpipestatus\rlaststatus\rexec status42 | exec stdin\rpipestatus\rlaststatus\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (
            empty,
            zero_pipeline,
            zero_status,
            zero_last,
            nonzero_pipeline,
            nonzero_status,
            nonzero_last,
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
            )
        };
        let output = backend.as_str();

        assert_eq!(empty.line(), b"pipestatus");
        assert_eq!(empty.status(), LocalCommandStatus::Handled);
        assert_eq!(zero_pipeline.line(), b"exec stdout | exec stdin");
        assert_eq!(zero_pipeline.status(), LocalCommandStatus::Handled);
        assert_eq!(zero_status.line(), b"pipestatus");
        assert_eq!(zero_status.status(), LocalCommandStatus::Handled);
        assert_eq!(zero_last.status(), LocalCommandStatus::Handled);
        assert_eq!(nonzero_pipeline.line(), b"exec status42 | exec stdin");
        assert_eq!(nonzero_pipeline.status(), LocalCommandStatus::Handled);
        assert_eq!(nonzero_status.line(), b"pipestatus");
        assert_eq!(nonzero_status.status(), LocalCommandStatus::Handled);
        assert_eq!(nonzero_last.status(), LocalCommandStatus::Handled);
        assert!(output.contains(
            "talos> talos: pipestatus none source=bounded-process-table-pipeline-status\n"
        ));
        assert!(output.contains(
            "talos> talos: pipestatus participants=0x0000000000000002 default-status=0x0000000000000000 pipefail-status=0x0000000000000000 semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status\n"
        ));
        assert!(output.contains(
            "talos: pipeline id=0x0000000000000001 producer-fd=0x0000000000000001 producer-path=/bin/status42 consumer-fd=0x0000000000000000 consumer-path=/bin/stdin bytes-written=0x0000000000000000 bytes-read=0x0000000000000000 writer-closed=true reader-eof=true shell-restored=true source=shell-pipe-status42-to-stdin\n"
        ));
        assert!(output.contains(
            "talos> talos: pipestatus participants=0x0000000000000002 default-status=0x0000000000000000 pipefail-status=0x000000000000002a semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status\n"
        ));
        assert!(output.contains(
            "talos: pipestatus-participant slot=0 pid=0x0000000000100001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=bounded-process-table-pipeline-status\n"
        ));
        assert!(output.contains(
            "talos> talos: last-process pid=0x0000000000100002 parent=shell owner=0x0000000000000001 path=/bin/stdin state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record\n"
        ));
    }

    #[test_case]
    fn local_command_loop_reports_three_stage_pipeline_status_from_process_table() {
        let bytes = *b"exec stdout | exec stdin | exec stdin\rpipestatus\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (pipeline, status) = {
            let mut io =
                DescriptorBackedLocalCommandIo::new_inherited_stdio(input, &mut backend).unwrap();
            (
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
                run_one_descriptor_backed_serial_command(&mut io).unwrap(),
            )
        };
        let output = backend.as_str();

        assert_eq!(pipeline.line(), b"exec stdout | exec stdin | exec stdin");
        assert_eq!(pipeline.status(), LocalCommandStatus::Handled);
        assert_eq!(status.line(), b"pipestatus");
        assert_eq!(status.status(), LocalCommandStatus::Handled);
        assert!(output.contains(
            "talos> talos: pipestatus participants=0x0000000000000003 default-status=0x0000000000000000 pipefail-status=0x0000000000000000 semantics=bounded-observation-not-posix-shell source=bounded-process-table-pipeline-status\n"
        ));
    }

    #[test_case]
    fn local_command_loop_cats_proc_talos_processes_after_background_vfs_exec() {
        let bytes = *b"exec /bin/status42 &\rcat /proc/talos/processes\rwaitpid 0x100001\rcat /proc/talos/processes\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (background, first_cat, wait, second_cat) = {
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

        assert_eq!(background.status(), LocalCommandStatus::Handled);
        assert_eq!(first_cat.status(), LocalCommandStatus::Handled);
        assert_eq!(wait.status(), LocalCommandStatus::Handled);
        assert_eq!(second_cat.status(), LocalCommandStatus::Handled);
        assert!(output.contains(
            "slot=0 capacity=3 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true wait-consumed=false job-state=completed source=bounded-process-table\n"
        ));
        assert!(output.contains(
            "slot=0 capacity=3 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true wait-consumed=true job-state=foreground source=bounded-process-table\n"
        ));
    }

    #[test_case]
    fn local_command_loop_ps_reads_proc_talos_processes_status_file() {
        let bytes = *b"exec /bin/status42\rps\rwaitpid\rps\rps -a\rps extra\r";
        let input = ScriptedInput::new(bytes, bytes.len());
        let mut backend = CaptureSink::new();
        let (exec, first_ps, wait, second_ps, option, extra) = {
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

        assert_eq!(exec.status(), LocalCommandStatus::Handled);
        assert_eq!(first_ps.line(), b"ps");
        assert_eq!(first_ps.status(), LocalCommandStatus::Handled);
        assert_eq!(first_ps.response_lines(), 1);
        assert_eq!(wait.status(), LocalCommandStatus::Handled);
        assert_eq!(second_ps.line(), b"ps");
        assert_eq!(second_ps.status(), LocalCommandStatus::Handled);
        assert_eq!(second_ps.response_lines(), 1);
        assert_eq!(option.line(), b"ps -a");
        assert_eq!(option.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(extra.line(), b"ps extra");
        assert_eq!(extra.status(), LocalCommandStatus::UnexpectedArgument);
        assert_eq!(output.matches("talos-processes-v1\n").count(), 2);
        assert!(output.contains(
            "slot=0 capacity=3 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true wait-consumed=false job-state=foreground source=bounded-process-table\n"
        ));
        assert!(output.contains(
            "slot=0 capacity=3 pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/status42 state=exited status=0x000000000000002a observed-status=0x000000000000002a reaped=true wait-consumed=true job-state=foreground source=bounded-process-table\n"
        ));
        assert_eq!(
            output
                .matches("talos> talos: unexpected-argument\n")
                .count(),
            2
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
        let mut no_data_input = ScriptedInput::new([0; 32], 0);
        let mut no_data_sink = CaptureSink::new();
        let no_data =
            run_one_serial_command_with_limit(&mut no_data_input, &mut no_data_sink, 2).unwrap();

        assert!(no_data.is_no_data_timeout());
        assert_eq!(
            no_data.status(),
            LocalCommandStatus::InputError(PollingTtyRxOutcome::Timeout)
        );
        assert_eq!(no_data.line(), b"");
        assert_eq!(no_data.raw_bytes(), 0);
        assert_eq!(no_data_sink.as_str(), "talos> talos: input-error timeout\n");

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
