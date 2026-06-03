use crate::{
    initramfs, posix,
    runtime_console::{self, ConsoleBackend, ConsoleInputBackend, DEFAULT_RUNTIME_CONSOLE},
    scheduler::ProcessOwnerId,
    tty::{self, CANONICAL_LINE_CAPACITY, PollingTtyRxOutcome, PollingTtyRxResult},
};

pub const LOCAL_COMMAND_LOOP_VERSION: &str = "phase10.1-kernel-builtins-v1";
pub const LOCAL_COMMAND_LOOP_PROMPT: &str = "talos> ";
pub const DEFAULT_LOCAL_COMMAND_COUNT: usize = 7;

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
const LOCAL_COMMAND_BIN_LISTING: [(&[u8], &str); 1] = [(initramfs::PHASE8_INIT_PATH, "init")];

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

pub struct DescriptorBackedLocalCommandSink<
    'a,
    B,
    const OWNER_CAPACITY: usize,
    const DESCRIPTOR_CAPACITY: usize,
> where
    B: ConsoleBackend,
{
    descriptor_store: posix::ProcessDescriptorStore<OWNER_CAPACITY, DESCRIPTOR_CAPACITY>,
    current_owner: Option<ProcessOwnerId>,
    output_descriptor: usize,
    console_backend: &'a mut B,
    current_directory: LocalCommandDirectory,
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
}

impl<'a, B> DescriptorBackedLocalCommandSink<'a, B, 1, 4>
where
    B: ConsoleBackend,
{
    pub fn new_inherited_stdio(console_backend: &'a mut B) -> Result<Self, posix::PosixError> {
        let current_owner = ProcessOwnerId::new(1).expect("local command owner id is nonzero");
        let mut descriptor_store = posix::ProcessDescriptorStore::<1, 4>::new_empty();
        descriptor_store.create_owner_with_inherited_stdio(current_owner)?;
        Ok(Self {
            descriptor_store,
            current_owner: Some(current_owner),
            output_descriptor: posix::STDOUT_FD,
            console_backend,
            current_directory: LocalCommandDirectory::Root,
        })
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

impl<B, const OWNER_CAPACITY: usize, const DESCRIPTOR_CAPACITY: usize> LocalCommandSink
    for DescriptorBackedLocalCommandSink<'_, B, OWNER_CAPACITY, DESCRIPTOR_CAPACITY>
where
    B: ConsoleBackend,
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
            self.console_backend,
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

    fn current_directory(&self) -> LocalCommandDirectory {
        self.current_directory
    }

    fn set_current_directory(&mut self, directory: LocalCommandDirectory) -> bool {
        self.current_directory = directory;
        true
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
                "talos: commands help status stdio pwd echo ls cat cd",
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
            write_line(sink, responses, "talos: builtins kernel-backed")?;
            write_line(
                sink,
                responses,
                "talos: commands help status stdio pwd echo ls cat cd",
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
    let fs = initramfs::phase8_readonly_initramfs_fixture();
    let bytes = match fs.regular_file_bytes(initramfs::PHASE8_BANNER_PATH) {
        Ok(bytes) => bytes,
        Err(_) => {
            write_line(sink, responses, "talos: filesystem-error")?;
            return Ok(());
        }
    };
    let text = match core::str::from_utf8(bytes) {
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

fn copy_line(line: &[u8]) -> [u8; CANONICAL_LINE_CAPACITY] {
    let mut copy = [0; CANONICAL_LINE_CAPACITY];
    copy[..line.len()].copy_from_slice(line);
    copy
}

#[cfg(test)]
mod tests {
    use crate::tty::TtyControlEvent;

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
        bytes: [u8; 1024],
        len: usize,
        fail_after: usize,
        writes: usize,
    }

    impl CaptureSink {
        const fn new() -> Self {
            Self {
                bytes: [0; 1024],
                len: 0,
                fail_after: usize::MAX,
                writes: 0,
            }
        }

        const fn failing_after(fail_after: usize) -> Self {
            Self {
                bytes: [0; 1024],
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
        assert_eq!(DEFAULT_LOCAL_COMMAND_COUNT, 7);
        assert_eq!(
            sink.as_str(),
            "talos> talos: ok help\n\
	talos: commands help status stdio pwd echo ls cat cd\n\
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
        assert_eq!(result.response_lines(), 1);
        assert_eq!(backend.as_str(), "talos> init\n");
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
        assert_eq!(bin_ls.response_lines(), 1);
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
