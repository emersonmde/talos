use crate::{
    entropy::{self, EntropyDiagnosticSnapshot},
    initramfs::ReadOnlyInitramfs,
    runtime_console::{self, ConsoleBackend, DEFAULT_RUNTIME_CONSOLE},
    ssh_key_readiness::{self, SshKeyReadinessSnapshot},
    ssh_service_readiness,
    tty::CANONICAL_LINE_CAPACITY,
};

pub const DIAGNOSTIC_COMMAND_CHANNEL_VERSION: &str = "phase5.3-contract-v1";
pub const MAX_COMMAND_TOKEN_BYTES: usize = 16;
pub const MAX_COMMAND_ARGUMENTS: usize = 2;
pub const DEFAULT_COMMAND_COUNT: usize = 6;
const DIAGNOSTIC_COMMAND_LIST: &str = "entropy help list sshkeydiag sshservicediag status";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DiagnosticParseError {
    EmptyLine,
    InvalidUtf8,
    UnsupportedTokenByte(u8),
    TokenTooLong,
    TooManyArguments,
}

impl DiagnosticParseError {
    pub const fn name(self) -> &'static str {
        match self {
            Self::EmptyLine => "empty-command",
            Self::InvalidUtf8 => "invalid-utf8",
            Self::UnsupportedTokenByte(_) => "unsupported-token-byte",
            Self::TokenTooLong => "token-too-long",
            Self::TooManyArguments => "too-many-arguments",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ParsedDiagnosticCommand<'a> {
    name: &'a str,
    arguments: [&'a str; MAX_COMMAND_ARGUMENTS],
    argument_count: usize,
}

impl<'a> ParsedDiagnosticCommand<'a> {
    pub const fn name(&self) -> &'a str {
        self.name
    }

    pub fn arguments(&self) -> &[&'a str] {
        &self.arguments[..self.argument_count]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DiagnosticDispatchStatus {
    Handled,
    UnknownCommand,
    UnexpectedArgument,
    ParseError(DiagnosticParseError),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DiagnosticDispatchResult {
    pub status: DiagnosticDispatchStatus,
    pub response_lines: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DiagnosticResponseError {
    BackendWriteFailed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DiagnosticDispatchError {
    ResponseWriteFailed,
}

pub trait DiagnosticResponseSink {
    fn write_response_str(&mut self, text: &str) -> Result<(), DiagnosticResponseError>;
}

impl<B> DiagnosticResponseSink for runtime_console::RuntimeConsole<B>
where
    B: ConsoleBackend,
{
    fn write_response_str(&mut self, text: &str) -> Result<(), DiagnosticResponseError> {
        self.write_kernel_args(format_args!("{}", text))
            .map(|_| ())
            .map_err(|_| DiagnosticResponseError::BackendWriteFailed)
    }
}

pub fn parse_diagnostic_command_line(
    line: &[u8],
) -> Result<ParsedDiagnosticCommand<'_>, DiagnosticParseError> {
    let line = core::str::from_utf8(line).map_err(|_| DiagnosticParseError::InvalidUtf8)?;
    let bytes = line.as_bytes();
    let mut index = 0usize;

    skip_token_separators(bytes, &mut index);
    if index == bytes.len() {
        return Err(DiagnosticParseError::EmptyLine);
    }

    let name = parse_token(line, bytes, &mut index)?;
    let mut command = ParsedDiagnosticCommand {
        name,
        arguments: [""; MAX_COMMAND_ARGUMENTS],
        argument_count: 0,
    };

    loop {
        skip_token_separators(bytes, &mut index);
        if index == bytes.len() {
            break;
        }
        if command.argument_count == MAX_COMMAND_ARGUMENTS {
            return Err(DiagnosticParseError::TooManyArguments);
        }
        command.arguments[command.argument_count] = parse_token(line, bytes, &mut index)?;
        command.argument_count += 1;
    }

    Ok(command)
}

pub fn dispatch_default_diagnostic_command(
    line: &[u8],
    sink: &mut impl DiagnosticResponseSink,
) -> Result<DiagnosticDispatchResult, DiagnosticDispatchError> {
    dispatch_diagnostic_command(line, DiagnosticContext::FailClosedDefault, sink)
}

pub(crate) fn dispatch_diagnostic_command_with_operator_seed_material(
    line: &[u8],
    initramfs: ReadOnlyInitramfs,
    sink: &mut impl DiagnosticResponseSink,
) -> Result<DiagnosticDispatchResult, DiagnosticDispatchError> {
    dispatch_diagnostic_command(
        line,
        DiagnosticContext::ReadOnlyVfsMetadata(initramfs),
        sink,
    )
}

#[derive(Clone, Copy, Debug)]
enum DiagnosticContext {
    FailClosedDefault,
    ReadOnlyVfsMetadata(ReadOnlyInitramfs),
}

fn dispatch_diagnostic_command(
    line: &[u8],
    context: DiagnosticContext,
    sink: &mut impl DiagnosticResponseSink,
) -> Result<DiagnosticDispatchResult, DiagnosticDispatchError> {
    let command = match parse_diagnostic_command_line(line) {
        Ok(command) => command,
        Err(error) => {
            let mut responses = 0usize;
            write_parts_line(sink, &mut responses, &["diag: error ", error.name()])?;
            return Ok(DiagnosticDispatchResult {
                status: DiagnosticDispatchStatus::ParseError(error),
                response_lines: responses,
            });
        }
    };

    if !command.arguments().is_empty() {
        let mut responses = 0usize;
        write_line(sink, &mut responses, "diag: error unexpected-argument")?;
        return Ok(DiagnosticDispatchResult {
            status: DiagnosticDispatchStatus::UnexpectedArgument,
            response_lines: responses,
        });
    }

    match command.name() {
        "entropy" => write_entropy_response(context, sink),
        "help" => write_help_response(sink),
        "list" => write_list_response(sink),
        "sshkeydiag" => write_ssh_key_readiness_response(context, sink),
        "sshservicediag" => write_ssh_service_readiness_response(context, sink),
        "status" => write_status_response(sink),
        _ => {
            let mut responses = 0usize;
            write_line(sink, &mut responses, "diag: error unknown-command")?;
            Ok(DiagnosticDispatchResult {
                status: DiagnosticDispatchStatus::UnknownCommand,
                response_lines: responses,
            })
        }
    }
}

fn write_help_response(
    sink: &mut impl DiagnosticResponseSink,
) -> Result<DiagnosticDispatchResult, DiagnosticDispatchError> {
    let mut responses = 0usize;
    write_line(sink, &mut responses, "diag: ok help")?;
    write_parts_line(
        sink,
        &mut responses,
        &["diag: commands ", DIAGNOSTIC_COMMAND_LIST],
    )?;
    Ok(DiagnosticDispatchResult {
        status: DiagnosticDispatchStatus::Handled,
        response_lines: responses,
    })
}

fn write_list_response(
    sink: &mut impl DiagnosticResponseSink,
) -> Result<DiagnosticDispatchResult, DiagnosticDispatchError> {
    let mut responses = 0usize;
    write_line(sink, &mut responses, "diag: ok list")?;
    write_parts_line(
        sink,
        &mut responses,
        &["diag: commands ", DIAGNOSTIC_COMMAND_LIST],
    )?;
    Ok(DiagnosticDispatchResult {
        status: DiagnosticDispatchStatus::Handled,
        response_lines: responses,
    })
}

fn write_status_response(
    sink: &mut impl DiagnosticResponseSink,
) -> Result<DiagnosticDispatchResult, DiagnosticDispatchError> {
    let mut responses = 0usize;
    write_line(sink, &mut responses, "diag: ok status")?;
    write_parts_line(
        sink,
        &mut responses,
        &["diag: version ", DIAGNOSTIC_COMMAND_CHANNEL_VERSION],
    )?;
    write_parts_line(
        sink,
        &mut responses,
        &["diag: runtime-console ", DEFAULT_RUNTIME_CONSOLE.name],
    )?;
    write_usize_line(
        sink,
        &mut responses,
        "diag: tty canonical-lite line-capacity ",
        CANONICAL_LINE_CAPACITY,
    )?;
    write_usize_line(
        sink,
        &mut responses,
        "diag: command-count ",
        DEFAULT_COMMAND_COUNT,
    )?;
    write_parts_line(
        sink,
        &mut responses,
        &["diag: commands ", DIAGNOSTIC_COMMAND_LIST],
    )?;
    Ok(DiagnosticDispatchResult {
        status: DiagnosticDispatchStatus::Handled,
        response_lines: responses,
    })
}

fn write_entropy_response(
    context: DiagnosticContext,
    sink: &mut impl DiagnosticResponseSink,
) -> Result<DiagnosticDispatchResult, DiagnosticDispatchError> {
    let snapshot = match context {
        DiagnosticContext::FailClosedDefault => EntropyDiagnosticSnapshot::empty(),
        DiagnosticContext::ReadOnlyVfsMetadata(initramfs) => {
            entropy::entropy_snapshot_with_operator_seed_material(initramfs)
        }
    };
    let report = entropy::classify_entropy_snapshot(snapshot);
    let mut responses = 0usize;
    write_line(sink, &mut responses, "diag: ok entropy")?;
    write_parts_line(
        sink,
        &mut responses,
        &["diag: entropy-label ", report.input_label().name()],
    )?;
    write_parts_line(
        sink,
        &mut responses,
        &["diag: hardware-rng ", report.hardware_rng_label().name()],
    )?;
    if let Some(seed_label) = report.operator_seed_label() {
        write_parts_line(
            sink,
            &mut responses,
            &["diag: operator-seed ", seed_label.name()],
        )?;
    }
    write_bool_line(
        sink,
        &mut responses,
        "diag: cryptographic-strength ",
        report.cryptographic_strength(),
    )?;
    write_bool_line(sink, &mut responses, "diag: ssh-ready ", report.ssh_ready())?;
    Ok(DiagnosticDispatchResult {
        status: DiagnosticDispatchStatus::Handled,
        response_lines: responses,
    })
}

fn write_ssh_key_readiness_response(
    context: DiagnosticContext,
    sink: &mut impl DiagnosticResponseSink,
) -> Result<DiagnosticDispatchResult, DiagnosticDispatchError> {
    let snapshot = ssh_key_readiness_snapshot(context);
    let report = ssh_key_readiness::classify_ssh_key_readiness(snapshot);
    let mut responses = 0usize;
    write_line(sink, &mut responses, "diag: ok sshkeydiag")?;
    write_parts_line(
        sink,
        &mut responses,
        &["diag: sshkey-readiness ", report.primary_label().name()],
    )?;
    for label in report.labels() {
        write_parts_line(sink, &mut responses, &["diag: sshkey-label ", label.name()])?;
    }
    write_bool_line(sink, &mut responses, "diag: ssh-ready ", report.ssh_ready())?;
    Ok(DiagnosticDispatchResult {
        status: DiagnosticDispatchStatus::Handled,
        response_lines: responses,
    })
}

fn write_ssh_service_readiness_response(
    context: DiagnosticContext,
    sink: &mut impl DiagnosticResponseSink,
) -> Result<DiagnosticDispatchResult, DiagnosticDispatchError> {
    let key_report =
        ssh_key_readiness::classify_ssh_key_readiness(ssh_key_readiness_snapshot(context));
    let report = ssh_service_readiness::classify_ssh_service_readiness(key_report);
    let mut responses = 0usize;
    write_line(sink, &mut responses, "diag: ok sshservicediag")?;
    write_parts_line(
        sink,
        &mut responses,
        &["diag: sshservice-readiness ", report.primary_label().name()],
    )?;
    write_parts_line(
        sink,
        &mut responses,
        &["diag: sshservice-lifecycle ", report.lifecycle().name()],
    )?;
    for label in report.labels() {
        write_parts_line(
            sink,
            &mut responses,
            &["diag: sshservice-label ", label.name()],
        )?;
    }
    write_usize_line(
        sink,
        &mut responses,
        "diag: listener-count ",
        report.listener_count(),
    )?;
    write_bool_line(
        sink,
        &mut responses,
        "diag: transport-enabled ",
        report.transport_enabled(),
    )?;
    write_usize_line(
        sink,
        &mut responses,
        "diag: accepted-connection-count ",
        report.accepted_connection_count(),
    )?;
    write_usize_line(
        sink,
        &mut responses,
        "diag: session-count ",
        report.session_count(),
    )?;
    write_usize_line(
        sink,
        &mut responses,
        "diag: channel-count ",
        report.channel_count(),
    )?;
    write_bool_line(
        sink,
        &mut responses,
        "diag: authentication-success ",
        report.authentication_success(),
    )?;
    write_bool_line(
        sink,
        &mut responses,
        "diag: shell-attached ",
        report.shell_attached(),
    )?;
    write_bool_line(
        sink,
        &mut responses,
        "diag: reachability-accepted ",
        report.reachability_accepted(),
    )?;
    write_bool_line(sink, &mut responses, "diag: ssh-ready ", report.ssh_ready())?;
    Ok(DiagnosticDispatchResult {
        status: DiagnosticDispatchStatus::Handled,
        response_lines: responses,
    })
}

fn ssh_key_readiness_snapshot(context: DiagnosticContext) -> SshKeyReadinessSnapshot {
    match context {
        DiagnosticContext::FailClosedDefault => SshKeyReadinessSnapshot::fail_closed_default(),
        DiagnosticContext::ReadOnlyVfsMetadata(initramfs) => {
            let seed_metadata = entropy::classify_operator_seed_material(initramfs);
            let host_key_metadata = ssh_key_readiness::classify_host_key_material(initramfs);
            let authorized_key_metadata =
                ssh_key_readiness::classify_authorized_key_material(initramfs);
            let persistence = ssh_key_readiness::classify_persistence_metadata(
                seed_metadata,
                host_key_metadata,
                authorized_key_metadata,
            );
            let exposure = ssh_key_readiness::classify_exposure_marker(initramfs);
            let entropy = entropy::classify_entropy_snapshot(
                entropy::entropy_snapshot_with_operator_seed_material(initramfs),
            );
            SshKeyReadinessSnapshot::fail_closed_default()
                .with_host_key_material(host_key_metadata)
                .with_authorized_key_material(authorized_key_metadata)
                .with_operator_seed_material(seed_metadata)
                .with_persistence_state(persistence)
                .with_exposure_state(exposure)
                .with_entropy_report(entropy)
        }
    }
}

fn skip_token_separators(bytes: &[u8], index: &mut usize) {
    while *index < bytes.len() && matches!(bytes[*index], b' ' | b'\t') {
        *index += 1;
    }
}

fn parse_token<'a>(
    line: &'a str,
    bytes: &[u8],
    index: &mut usize,
) -> Result<&'a str, DiagnosticParseError> {
    let start = *index;
    while *index < bytes.len() && !matches!(bytes[*index], b' ' | b'\t') {
        let byte = bytes[*index];
        if !is_supported_token_byte(byte) {
            return Err(DiagnosticParseError::UnsupportedTokenByte(byte));
        }
        if *index - start == MAX_COMMAND_TOKEN_BYTES {
            return Err(DiagnosticParseError::TokenTooLong);
        }
        *index += 1;
    }
    Ok(&line[start..*index])
}

const fn is_supported_token_byte(byte: u8) -> bool {
    matches!(byte, b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_')
}

fn write_line(
    sink: &mut impl DiagnosticResponseSink,
    response_lines: &mut usize,
    text: &str,
) -> Result<(), DiagnosticDispatchError> {
    write_parts_line(sink, response_lines, &[text])
}

fn write_parts_line(
    sink: &mut impl DiagnosticResponseSink,
    response_lines: &mut usize,
    parts: &[&str],
) -> Result<(), DiagnosticDispatchError> {
    for part in parts {
        sink.write_response_str(part)
            .map_err(|_| DiagnosticDispatchError::ResponseWriteFailed)?;
    }
    write_newline(sink, response_lines)
}

fn write_newline(
    sink: &mut impl DiagnosticResponseSink,
    response_lines: &mut usize,
) -> Result<(), DiagnosticDispatchError> {
    sink.write_response_str("\n")
        .map_err(|_| DiagnosticDispatchError::ResponseWriteFailed)?;
    *response_lines += 1;
    Ok(())
}

fn write_usize_line(
    sink: &mut impl DiagnosticResponseSink,
    response_lines: &mut usize,
    prefix: &str,
    mut value: usize,
) -> Result<(), DiagnosticDispatchError> {
    let mut digits = [0u8; 20];
    let mut len = 0usize;

    if value == 0 {
        digits[0] = b'0';
        len = 1;
    } else {
        while value != 0 {
            digits[len] = b'0' + (value % 10) as u8;
            len += 1;
            value /= 10;
        }
        digits[..len].reverse();
    }

    let text = core::str::from_utf8(&digits[..len]).expect("decimal digits are utf8");
    sink.write_response_str(prefix)
        .map_err(|_| DiagnosticDispatchError::ResponseWriteFailed)?;
    sink.write_response_str(text)
        .map_err(|_| DiagnosticDispatchError::ResponseWriteFailed)?;
    write_newline(sink, response_lines)
}

fn write_bool_line(
    sink: &mut impl DiagnosticResponseSink,
    response_lines: &mut usize,
    prefix: &str,
    value: bool,
) -> Result<(), DiagnosticDispatchError> {
    write_parts_line(
        sink,
        response_lines,
        &[prefix, if value { "true" } else { "false" }],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        entropy::OPERATOR_SEED_MIN_SUFFICIENT_BYTES,
        initramfs::{DirectoryEntry, InitramfsNode, phase8_readonly_initramfs_fixture},
        ssh_key_readiness::{AUTHORIZED_KEY_MAX_METADATA_BYTES, AUTHORIZED_KEY_MIN_METADATA_BYTES},
        ssh_key_readiness::{
            EXPOSURE_MARKER_MAX_METADATA_BYTES, HOST_KEY_MAX_METADATA_BYTES,
            HOST_KEY_MIN_METADATA_BYTES,
        },
    };

    struct CaptureSink {
        bytes: [u8; 2048],
        len: usize,
        fail_after: usize,
        writes: usize,
    }

    impl CaptureSink {
        const fn new() -> Self {
            Self {
                bytes: [0; 2048],
                len: 0,
                fail_after: usize::MAX,
                writes: 0,
            }
        }

        const fn failing_after(fail_after: usize) -> Self {
            Self {
                bytes: [0; 2048],
                len: 0,
                fail_after,
                writes: 0,
            }
        }

        fn as_str(&self) -> &str {
            core::str::from_utf8(&self.bytes[..self.len]).expect("capture is utf8")
        }
    }

    impl DiagnosticResponseSink for CaptureSink {
        fn write_response_str(&mut self, text: &str) -> Result<(), DiagnosticResponseError> {
            if self.writes >= self.fail_after {
                return Err(DiagnosticResponseError::BackendWriteFailed);
            }

            let end = self.len + text.len();
            self.bytes[self.len..end].copy_from_slice(text.as_bytes());
            self.len = end;
            self.writes += 1;
            Ok(())
        }
    }

    #[test_case]
    fn parser_accepts_completed_tty_line_tokens() {
        let parsed = parse_diagnostic_command_line(b"  status\targ_one arg-2  ").unwrap();

        assert_eq!(parsed.name(), "status");
        assert_eq!(parsed.arguments(), &["arg_one", "arg-2"]);
    }

    #[test_case]
    fn parser_rejects_non_command_shell_syntax_and_bounds() {
        assert_eq!(
            parse_diagnostic_command_line(b"").unwrap_err(),
            DiagnosticParseError::EmptyLine
        );
        assert_eq!(
            parse_diagnostic_command_line(b"status;reboot").unwrap_err(),
            DiagnosticParseError::UnsupportedTokenByte(b';')
        );
        assert_eq!(
            parse_diagnostic_command_line(b"abcdefghijklmnopq").unwrap_err(),
            DiagnosticParseError::TokenTooLong
        );
        assert_eq!(
            parse_diagnostic_command_line(b"status a b c").unwrap_err(),
            DiagnosticParseError::TooManyArguments
        );
    }

    #[test_case]
    fn dispatcher_lists_bounded_default_commands() {
        let mut sink = CaptureSink::new();
        let result = dispatch_default_diagnostic_command(b"help", &mut sink).unwrap();

        assert_eq!(result.status, DiagnosticDispatchStatus::Handled);
        assert_eq!(result.response_lines, 2);
        assert_eq!(
            sink.as_str(),
            "diag: ok help\ndiag: commands entropy help list sshkeydiag sshservicediag status\n"
        );
    }

    #[test_case]
    fn dispatcher_reports_deterministic_status_without_posix_state() {
        let mut sink = CaptureSink::new();
        let result = dispatch_default_diagnostic_command(b"status", &mut sink).unwrap();

        assert_eq!(result.status, DiagnosticDispatchStatus::Handled);
        assert_eq!(result.response_lines, 6);
        assert_eq!(
            sink.as_str(),
            "diag: ok status\ndiag: version phase5.3-contract-v1\ndiag: runtime-console runtime-console0\ndiag: tty canonical-lite line-capacity 64\ndiag: command-count 6\ndiag: commands entropy help list sshkeydiag sshservicediag status\n"
        );
    }

    #[test_case]
    fn dispatcher_reports_entropy_diagnostic_fail_closed_without_crypto_claim() {
        let mut sink = CaptureSink::new();
        let result = dispatch_default_diagnostic_command(b"entropy", &mut sink).unwrap();

        assert_eq!(result.status, DiagnosticDispatchStatus::Handled);
        assert_eq!(result.response_lines, 6);
        assert_eq!(
            sink.as_str(),
            "diag: ok entropy\ndiag: entropy-label entropydiag-fail-closed-no-input\ndiag: hardware-rng entropydiag-hardware-rng-unaccepted\ndiag: operator-seed entropydiag-operator-seed-required\ndiag: cryptographic-strength false\ndiag: ssh-ready false\n"
        );
    }

    #[test_case]
    fn dispatcher_reports_ssh_key_readiness_fail_closed_without_secret_material() {
        let mut sink = CaptureSink::new();
        let result = dispatch_default_diagnostic_command(b"sshkeydiag", &mut sink).unwrap();

        assert_eq!(result.status, DiagnosticDispatchStatus::Handled);
        assert_eq!(result.response_lines, 10);
        assert_eq!(
            sink.as_str(),
            "diag: ok sshkeydiag\ndiag: sshkey-readiness sshkeydiag-not-ready\ndiag: sshkey-label sshkeydiag-missing-host-key\ndiag: sshkey-label sshkeydiag-missing-authorized-key\ndiag: sshkey-label sshkeydiag-entropy-unready\ndiag: sshkey-label sshkeydiag-seed-material-missing\ndiag: sshkey-label sshkeydiag-persistence-unavailable\ndiag: sshkey-label sshkeydiag-exposure-disabled\ndiag: sshkey-label sshkeydiag-not-ready\ndiag: ssh-ready false\n"
        );
    }

    #[test_case]
    fn dispatcher_reports_ssh_service_readiness_fail_closed_without_live_service() {
        let mut sink = CaptureSink::new();
        let result = dispatch_default_diagnostic_command(b"sshservicediag", &mut sink).unwrap();

        assert_eq!(result.status, DiagnosticDispatchStatus::Handled);
        assert_eq!(result.response_lines, 20);
        assert_eq!(
            sink.as_str(),
            "diag: ok sshservicediag\ndiag: sshservice-readiness sshservicediag-not-ready\ndiag: sshservice-lifecycle disabled\ndiag: sshservice-label sshservicediag-exposure-disabled\ndiag: sshservice-label sshservicediag-prerequisites-missing\ndiag: sshservice-label sshservicediag-transport-unaccepted\ndiag: sshservice-label sshservicediag-dependency-unaccepted\ndiag: sshservice-label sshservicediag-crypto-backend-unaccepted\ndiag: sshservice-label sshservicediag-authentication-unimplemented\ndiag: sshservice-label sshservicediag-session-unimplemented\ndiag: sshservice-label sshservicediag-not-ready\ndiag: listener-count 0\ndiag: transport-enabled false\ndiag: accepted-connection-count 0\ndiag: session-count 0\ndiag: channel-count 0\ndiag: authentication-success false\ndiag: shell-attached false\ndiag: reachability-accepted false\ndiag: ssh-ready false\n"
        );
    }

    #[test_case]
    fn dispatcher_reports_operator_seed_missing_from_vfs_without_secret_material() {
        let mut entropy_sink = CaptureSink::new();
        let entropy_result = dispatch_diagnostic_command_with_operator_seed_material(
            b"entropy",
            phase8_readonly_initramfs_fixture(),
            &mut entropy_sink,
        )
        .unwrap();

        assert_eq!(entropy_result.status, DiagnosticDispatchStatus::Handled);
        assert_eq!(entropy_result.response_lines, 6);
        assert_eq!(
            entropy_sink.as_str(),
            "diag: ok entropy\ndiag: entropy-label entropydiag-fail-closed-no-input\ndiag: hardware-rng entropydiag-hardware-rng-unaccepted\ndiag: operator-seed entropydiag-operator-seed-required\ndiag: cryptographic-strength false\ndiag: ssh-ready false\n"
        );

        let mut ssh_sink = CaptureSink::new();
        let ssh_result = dispatch_diagnostic_command_with_operator_seed_material(
            b"sshkeydiag",
            phase8_readonly_initramfs_fixture(),
            &mut ssh_sink,
        )
        .unwrap();

        assert_eq!(ssh_result.status, DiagnosticDispatchStatus::Handled);
        assert_eq!(ssh_result.response_lines, 10);
        assert_eq!(
            ssh_sink.as_str(),
            "diag: ok sshkeydiag\ndiag: sshkey-readiness sshkeydiag-not-ready\ndiag: sshkey-label sshkeydiag-missing-host-key\ndiag: sshkey-label sshkeydiag-missing-authorized-key\ndiag: sshkey-label sshkeydiag-entropy-unready\ndiag: sshkey-label sshkeydiag-seed-material-missing\ndiag: sshkey-label sshkeydiag-persistence-unavailable\ndiag: sshkey-label sshkeydiag-exposure-disabled\ndiag: sshkey-label sshkeydiag-not-ready\ndiag: ssh-ready false\n"
        );
    }

    #[test_case]
    fn dispatcher_reports_operator_seed_insufficient_from_vfs_without_secret_material() {
        let mut entropy_sink = CaptureSink::new();
        let entropy_result = dispatch_diagnostic_command_with_operator_seed_material(
            b"entropy",
            insufficient_seed_initramfs(),
            &mut entropy_sink,
        )
        .unwrap();

        assert_eq!(entropy_result.status, DiagnosticDispatchStatus::Handled);
        assert_eq!(entropy_result.response_lines, 5);
        assert_eq!(
            entropy_sink.as_str(),
            "diag: ok entropy\ndiag: entropy-label entropydiag-untrusted-local-mix\ndiag: hardware-rng entropydiag-hardware-rng-unaccepted\ndiag: cryptographic-strength false\ndiag: ssh-ready false\n"
        );

        let mut ssh_sink = CaptureSink::new();
        let ssh_result = dispatch_diagnostic_command_with_operator_seed_material(
            b"sshkeydiag",
            insufficient_seed_initramfs(),
            &mut ssh_sink,
        )
        .unwrap();

        assert_eq!(ssh_result.status, DiagnosticDispatchStatus::Handled);
        assert_eq!(ssh_result.response_lines, 10);
        assert_eq!(
            ssh_sink.as_str(),
            "diag: ok sshkeydiag\ndiag: sshkey-readiness sshkeydiag-not-ready\ndiag: sshkey-label sshkeydiag-missing-host-key\ndiag: sshkey-label sshkeydiag-missing-authorized-key\ndiag: sshkey-label sshkeydiag-entropy-unready\ndiag: sshkey-label sshkeydiag-seed-material-insufficient\ndiag: sshkey-label sshkeydiag-persistence-unavailable\ndiag: sshkey-label sshkeydiag-exposure-disabled\ndiag: sshkey-label sshkeydiag-not-ready\ndiag: ssh-ready false\n"
        );
    }

    #[test_case]
    fn dispatcher_reports_operator_seed_sufficient_from_vfs_without_secret_material() {
        let mut entropy_sink = CaptureSink::new();
        let entropy_result = dispatch_diagnostic_command_with_operator_seed_material(
            b"entropy",
            sufficient_seed_initramfs(),
            &mut entropy_sink,
        )
        .unwrap();

        assert_eq!(entropy_result.status, DiagnosticDispatchStatus::Handled);
        assert_eq!(entropy_result.response_lines, 5);
        assert_eq!(
            entropy_sink.as_str(),
            "diag: ok entropy\ndiag: entropy-label entropydiag-untrusted-local-mix\ndiag: hardware-rng entropydiag-hardware-rng-unaccepted\ndiag: cryptographic-strength false\ndiag: ssh-ready false\n"
        );

        let mut ssh_sink = CaptureSink::new();
        let ssh_result = dispatch_diagnostic_command_with_operator_seed_material(
            b"sshkeydiag",
            sufficient_seed_initramfs(),
            &mut ssh_sink,
        )
        .unwrap();

        assert_eq!(ssh_result.status, DiagnosticDispatchStatus::Handled);
        assert_eq!(ssh_result.response_lines, 9);
        assert_eq!(
            ssh_sink.as_str(),
            "diag: ok sshkeydiag\ndiag: sshkey-readiness sshkeydiag-not-ready\ndiag: sshkey-label sshkeydiag-missing-host-key\ndiag: sshkey-label sshkeydiag-missing-authorized-key\ndiag: sshkey-label sshkeydiag-entropy-unready\ndiag: sshkey-label sshkeydiag-persistence-unavailable\ndiag: sshkey-label sshkeydiag-exposure-disabled\ndiag: sshkey-label sshkeydiag-not-ready\ndiag: ssh-ready false\n"
        );
    }

    #[test_case]
    fn dispatcher_reports_host_key_metadata_invalid_insufficient_and_sufficient_from_vfs() {
        let mut invalid_sink = CaptureSink::new();
        let invalid_result = dispatch_diagnostic_command_with_operator_seed_material(
            b"sshkeydiag",
            empty_host_key_initramfs(),
            &mut invalid_sink,
        )
        .unwrap();

        assert_eq!(invalid_result.status, DiagnosticDispatchStatus::Handled);
        assert_eq!(invalid_result.response_lines, 10);
        assert_eq!(
            invalid_sink.as_str(),
            "diag: ok sshkeydiag\ndiag: sshkey-readiness sshkeydiag-not-ready\ndiag: sshkey-label sshkeydiag-host-key-invalid\ndiag: sshkey-label sshkeydiag-missing-authorized-key\ndiag: sshkey-label sshkeydiag-entropy-unready\ndiag: sshkey-label sshkeydiag-seed-material-missing\ndiag: sshkey-label sshkeydiag-persistence-unavailable\ndiag: sshkey-label sshkeydiag-exposure-disabled\ndiag: sshkey-label sshkeydiag-not-ready\ndiag: ssh-ready false\n"
        );

        let mut oversized_sink = CaptureSink::new();
        let oversized_result = dispatch_diagnostic_command_with_operator_seed_material(
            b"sshkeydiag",
            oversized_host_key_initramfs(),
            &mut oversized_sink,
        )
        .unwrap();

        assert_eq!(oversized_result.status, DiagnosticDispatchStatus::Handled);
        assert_eq!(oversized_result.response_lines, 10);
        assert_eq!(
            oversized_sink.as_str(),
            "diag: ok sshkeydiag\ndiag: sshkey-readiness sshkeydiag-not-ready\ndiag: sshkey-label sshkeydiag-host-key-invalid\ndiag: sshkey-label sshkeydiag-missing-authorized-key\ndiag: sshkey-label sshkeydiag-entropy-unready\ndiag: sshkey-label sshkeydiag-seed-material-missing\ndiag: sshkey-label sshkeydiag-persistence-unavailable\ndiag: sshkey-label sshkeydiag-exposure-disabled\ndiag: sshkey-label sshkeydiag-not-ready\ndiag: ssh-ready false\n"
        );

        let mut insufficient_sink = CaptureSink::new();
        let insufficient_result = dispatch_diagnostic_command_with_operator_seed_material(
            b"sshkeydiag",
            insufficient_host_key_initramfs(),
            &mut insufficient_sink,
        )
        .unwrap();

        assert_eq!(
            insufficient_result.status,
            DiagnosticDispatchStatus::Handled
        );
        assert_eq!(insufficient_result.response_lines, 10);
        assert_eq!(
            insufficient_sink.as_str(),
            "diag: ok sshkeydiag\ndiag: sshkey-readiness sshkeydiag-not-ready\ndiag: sshkey-label sshkeydiag-host-key-insufficient\ndiag: sshkey-label sshkeydiag-missing-authorized-key\ndiag: sshkey-label sshkeydiag-entropy-unready\ndiag: sshkey-label sshkeydiag-seed-material-missing\ndiag: sshkey-label sshkeydiag-persistence-unavailable\ndiag: sshkey-label sshkeydiag-exposure-disabled\ndiag: sshkey-label sshkeydiag-not-ready\ndiag: ssh-ready false\n"
        );

        let mut sufficient_sink = CaptureSink::new();
        let sufficient_result = dispatch_diagnostic_command_with_operator_seed_material(
            b"sshkeydiag",
            sufficient_host_key_and_seed_initramfs(),
            &mut sufficient_sink,
        )
        .unwrap();

        assert_eq!(sufficient_result.status, DiagnosticDispatchStatus::Handled);
        assert_eq!(sufficient_result.response_lines, 8);
        assert_eq!(
            sufficient_sink.as_str(),
            "diag: ok sshkeydiag\ndiag: sshkey-readiness sshkeydiag-not-ready\ndiag: sshkey-label sshkeydiag-missing-authorized-key\ndiag: sshkey-label sshkeydiag-entropy-unready\ndiag: sshkey-label sshkeydiag-persistence-unavailable\ndiag: sshkey-label sshkeydiag-exposure-disabled\ndiag: sshkey-label sshkeydiag-not-ready\ndiag: ssh-ready false\n"
        );
    }

    #[test_case]
    fn dispatcher_reports_authorized_key_metadata_invalid_insufficient_and_sufficient_from_vfs() {
        let mut invalid_sink = CaptureSink::new();
        let invalid_result = dispatch_diagnostic_command_with_operator_seed_material(
            b"sshkeydiag",
            empty_authorized_key_initramfs(),
            &mut invalid_sink,
        )
        .unwrap();

        assert_eq!(invalid_result.status, DiagnosticDispatchStatus::Handled);
        assert_eq!(invalid_result.response_lines, 8);
        assert_eq!(
            invalid_sink.as_str(),
            "diag: ok sshkeydiag\ndiag: sshkey-readiness sshkeydiag-not-ready\ndiag: sshkey-label sshkeydiag-authorized-key-invalid\ndiag: sshkey-label sshkeydiag-entropy-unready\ndiag: sshkey-label sshkeydiag-persistence-unavailable\ndiag: sshkey-label sshkeydiag-exposure-disabled\ndiag: sshkey-label sshkeydiag-not-ready\ndiag: ssh-ready false\n"
        );

        let mut oversized_sink = CaptureSink::new();
        let oversized_result = dispatch_diagnostic_command_with_operator_seed_material(
            b"sshkeydiag",
            oversized_authorized_key_initramfs(),
            &mut oversized_sink,
        )
        .unwrap();

        assert_eq!(oversized_result.status, DiagnosticDispatchStatus::Handled);
        assert_eq!(oversized_result.response_lines, 8);
        assert_eq!(
            oversized_sink.as_str(),
            "diag: ok sshkeydiag\ndiag: sshkey-readiness sshkeydiag-not-ready\ndiag: sshkey-label sshkeydiag-authorized-key-invalid\ndiag: sshkey-label sshkeydiag-entropy-unready\ndiag: sshkey-label sshkeydiag-persistence-unavailable\ndiag: sshkey-label sshkeydiag-exposure-disabled\ndiag: sshkey-label sshkeydiag-not-ready\ndiag: ssh-ready false\n"
        );

        let mut insufficient_sink = CaptureSink::new();
        let insufficient_result = dispatch_diagnostic_command_with_operator_seed_material(
            b"sshkeydiag",
            insufficient_authorized_key_initramfs(),
            &mut insufficient_sink,
        )
        .unwrap();

        assert_eq!(
            insufficient_result.status,
            DiagnosticDispatchStatus::Handled
        );
        assert_eq!(insufficient_result.response_lines, 8);
        assert_eq!(
            insufficient_sink.as_str(),
            "diag: ok sshkeydiag\ndiag: sshkey-readiness sshkeydiag-not-ready\ndiag: sshkey-label sshkeydiag-authorized-key-insufficient\ndiag: sshkey-label sshkeydiag-entropy-unready\ndiag: sshkey-label sshkeydiag-persistence-unavailable\ndiag: sshkey-label sshkeydiag-exposure-disabled\ndiag: sshkey-label sshkeydiag-not-ready\ndiag: ssh-ready false\n"
        );

        let mut sufficient_sink = CaptureSink::new();
        let sufficient_result = dispatch_diagnostic_command_with_operator_seed_material(
            b"sshkeydiag",
            sufficient_host_key_seed_and_authorized_key_initramfs(),
            &mut sufficient_sink,
        )
        .unwrap();

        assert_eq!(sufficient_result.status, DiagnosticDispatchStatus::Handled);
        assert_eq!(sufficient_result.response_lines, 6);
        assert_eq!(
            sufficient_sink.as_str(),
            "diag: ok sshkeydiag\ndiag: sshkey-readiness sshkeydiag-not-ready\ndiag: sshkey-label sshkeydiag-entropy-unready\ndiag: sshkey-label sshkeydiag-exposure-disabled\ndiag: sshkey-label sshkeydiag-not-ready\ndiag: ssh-ready false\n"
        );
    }

    #[test_case]
    fn dispatcher_reports_persistence_exposure_metadata_without_secret_material() {
        let mut invalid_exposure_sink = CaptureSink::new();
        let invalid_exposure_result = dispatch_diagnostic_command_with_operator_seed_material(
            b"sshkeydiag",
            invalid_exposure_marker_initramfs(),
            &mut invalid_exposure_sink,
        )
        .unwrap();

        assert_eq!(
            invalid_exposure_result.status,
            DiagnosticDispatchStatus::Handled
        );
        assert_eq!(invalid_exposure_result.response_lines, 6);
        assert_eq!(
            invalid_exposure_sink.as_str(),
            "diag: ok sshkeydiag\ndiag: sshkey-readiness sshkeydiag-not-ready\ndiag: sshkey-label sshkeydiag-entropy-unready\ndiag: sshkey-label sshkeydiag-exposure-disabled\ndiag: sshkey-label sshkeydiag-not-ready\ndiag: ssh-ready false\n"
        );

        let mut enabled_exposure_sink = CaptureSink::new();
        let enabled_exposure_result = dispatch_diagnostic_command_with_operator_seed_material(
            b"sshkeydiag",
            enabled_exposure_marker_initramfs(),
            &mut enabled_exposure_sink,
        )
        .unwrap();

        assert_eq!(
            enabled_exposure_result.status,
            DiagnosticDispatchStatus::Handled
        );
        assert_eq!(enabled_exposure_result.response_lines, 5);
        assert_eq!(
            enabled_exposure_sink.as_str(),
            "diag: ok sshkeydiag\ndiag: sshkey-readiness sshkeydiag-not-ready\ndiag: sshkey-label sshkeydiag-entropy-unready\ndiag: sshkey-label sshkeydiag-not-ready\ndiag: ssh-ready false\n"
        );

        let mut service_sink = CaptureSink::new();
        let service_result = dispatch_diagnostic_command_with_operator_seed_material(
            b"sshservicediag",
            enabled_exposure_marker_initramfs(),
            &mut service_sink,
        )
        .unwrap();

        assert_eq!(service_result.status, DiagnosticDispatchStatus::Handled);
        assert_eq!(service_result.response_lines, 19);
        assert_eq!(
            service_sink.as_str(),
            "diag: ok sshservicediag\ndiag: sshservice-readiness sshservicediag-not-ready\ndiag: sshservice-lifecycle prerequisites-missing\ndiag: sshservice-label sshservicediag-prerequisites-missing\ndiag: sshservice-label sshservicediag-transport-unaccepted\ndiag: sshservice-label sshservicediag-dependency-unaccepted\ndiag: sshservice-label sshservicediag-crypto-backend-unaccepted\ndiag: sshservice-label sshservicediag-authentication-unimplemented\ndiag: sshservice-label sshservicediag-session-unimplemented\ndiag: sshservice-label sshservicediag-not-ready\ndiag: listener-count 0\ndiag: transport-enabled false\ndiag: accepted-connection-count 0\ndiag: session-count 0\ndiag: channel-count 0\ndiag: authentication-success false\ndiag: shell-attached false\ndiag: reachability-accepted false\ndiag: ssh-ready false\n"
        );
    }

    #[test_case]
    fn dispatcher_reports_unknown_parse_and_argument_errors() {
        let mut unknown = CaptureSink::new();
        let unknown_result = dispatch_default_diagnostic_command(b"ticks", &mut unknown).unwrap();
        assert_eq!(
            unknown_result.status,
            DiagnosticDispatchStatus::UnknownCommand
        );
        assert_eq!(unknown.as_str(), "diag: error unknown-command\n");

        let mut bad_parse = CaptureSink::new();
        let bad_parse_result =
            dispatch_default_diagnostic_command(b"status|pipes", &mut bad_parse).unwrap();
        assert_eq!(
            bad_parse_result.status,
            DiagnosticDispatchStatus::ParseError(DiagnosticParseError::UnsupportedTokenByte(b'|'))
        );
        assert_eq!(bad_parse.as_str(), "diag: error unsupported-token-byte\n");

        let mut argument = CaptureSink::new();
        let argument_result =
            dispatch_default_diagnostic_command(b"status now", &mut argument).unwrap();
        assert_eq!(
            argument_result.status,
            DiagnosticDispatchStatus::UnexpectedArgument
        );
        assert_eq!(argument.as_str(), "diag: error unexpected-argument\n");
    }

    #[test_case]
    fn dispatcher_propagates_response_sink_failure() {
        let mut sink = CaptureSink::failing_after(1);

        assert_eq!(
            dispatch_default_diagnostic_command(b"list", &mut sink).unwrap_err(),
            DiagnosticDispatchError::ResponseWriteFailed
        );
    }

    const ROOT_INDEX: usize = 0;
    const ETC_INDEX: usize = 1;
    const TALOS_INDEX: usize = 2;
    const SEED_INDEX: usize = 3;
    const SSH_INDEX: usize = 4;
    const HOST_KEY_INDEX: usize = 5;
    const HOST_KEY_AND_SEED_INDEX: usize = 6;
    const AUTHORIZED_KEY_INDEX: usize = 7;
    const EXPOSURE_MARKER_INDEX: usize = 8;

    static ROOT_ENTRIES: [DirectoryEntry; 1] = [DirectoryEntry::new(b"etc", ETC_INDEX)];
    static ETC_ENTRIES: [DirectoryEntry; 1] = [DirectoryEntry::new(b"talos", TALOS_INDEX)];
    static TALOS_ENTRIES: [DirectoryEntry; 1] =
        [DirectoryEntry::new(b"operator-seed.bin", SEED_INDEX)];
    static TALOS_WITH_SSH_ENTRIES: [DirectoryEntry; 1] = [DirectoryEntry::new(b"ssh", SSH_INDEX)];
    static TALOS_WITH_SEED_AND_SSH_ENTRIES: [DirectoryEntry; 2] = [
        DirectoryEntry::new(b"operator-seed.bin", SEED_INDEX),
        DirectoryEntry::new(b"ssh", SSH_INDEX),
    ];
    static SSH_ENTRIES: [DirectoryEntry; 1] =
        [DirectoryEntry::new(b"ssh_host_ed25519_key", HOST_KEY_INDEX)];
    static SSH_WITH_SEED_ENTRIES: [DirectoryEntry; 1] = [DirectoryEntry::new(
        b"ssh_host_ed25519_key",
        HOST_KEY_AND_SEED_INDEX,
    )];
    static SSH_WITH_AUTHORIZED_KEY_ENTRIES: [DirectoryEntry; 2] = [
        DirectoryEntry::new(b"ssh_host_ed25519_key", HOST_KEY_AND_SEED_INDEX),
        DirectoryEntry::new(b"authorized_keys", AUTHORIZED_KEY_INDEX),
    ];
    static SSH_WITH_AUTHORIZED_KEY_AND_EXPOSURE_ENTRIES: [DirectoryEntry; 3] = [
        DirectoryEntry::new(b"ssh_host_ed25519_key", HOST_KEY_AND_SEED_INDEX),
        DirectoryEntry::new(b"authorized_keys", AUTHORIZED_KEY_INDEX),
        DirectoryEntry::new(b"exposure-enabled", EXPOSURE_MARKER_INDEX),
    ];
    static INSUFFICIENT_SEED_BYTES: [u8; OPERATOR_SEED_MIN_SUFFICIENT_BYTES - 1] =
        [0; OPERATOR_SEED_MIN_SUFFICIENT_BYTES - 1];
    static SUFFICIENT_SEED_BYTES: [u8; OPERATOR_SEED_MIN_SUFFICIENT_BYTES] =
        [0; OPERATOR_SEED_MIN_SUFFICIENT_BYTES];
    static INSUFFICIENT_HOST_KEY_BYTES: [u8; HOST_KEY_MIN_METADATA_BYTES - 1] =
        [0; HOST_KEY_MIN_METADATA_BYTES - 1];
    static SUFFICIENT_HOST_KEY_BYTES: &[u8] = br#"
-----BEGIN OPENSSH PRIVATE KEY-----
b3BlbnNzaC1rZXktdjEAAAAABG5vbmUAAAAEbm9uZQAAAAAAAAABAAAAMwAAAAtzc2gtZW
QyNTUxOQAAACCzPq7zfqLffKoBDe/eo04kH2XxtSmk9D7RQyf1xUqrYgAAAJgAIAxdACAM
XQAAAAtzc2gtZWQyNTUxOQAAACCzPq7zfqLffKoBDe/eo04kH2XxtSmk9D7RQyf1xUqrYg
AAAEC2BsIi0QwW2uFscKTUUXNHLsYX4FxlaSDSblbAj7WR7bM+rvN+ot98qgEN796jTiQf
ZfG1KaT0PtFDJ/XFSqtiAAAAEHVzZXJAZXhhbXBsZS5jb20BAgMEBQ==
-----END OPENSSH PRIVATE KEY-----
"#;
    static OVERSIZED_HOST_KEY_BYTES: [u8; HOST_KEY_MAX_METADATA_BYTES + 1] =
        [0; HOST_KEY_MAX_METADATA_BYTES + 1];
    static INSUFFICIENT_AUTHORIZED_KEY_BYTES: [u8; AUTHORIZED_KEY_MIN_METADATA_BYTES - 1] =
        [0; AUTHORIZED_KEY_MIN_METADATA_BYTES - 1];
    static SUFFICIENT_AUTHORIZED_KEY_BYTES: [u8; AUTHORIZED_KEY_MIN_METADATA_BYTES] =
        [0; AUTHORIZED_KEY_MIN_METADATA_BYTES];
    static OVERSIZED_AUTHORIZED_KEY_BYTES: [u8; AUTHORIZED_KEY_MAX_METADATA_BYTES + 1] =
        [0; AUTHORIZED_KEY_MAX_METADATA_BYTES + 1];
    static OVERSIZED_EXPOSURE_MARKER_BYTES: [u8; EXPOSURE_MARKER_MAX_METADATA_BYTES + 1] =
        [0; EXPOSURE_MARKER_MAX_METADATA_BYTES + 1];

    static INSUFFICIENT_SEED_NODES: [InitramfsNode; 4] = [
        InitramfsNode::directory(ROOT_INDEX, &ROOT_ENTRIES),
        InitramfsNode::directory(ETC_INDEX, &ETC_ENTRIES),
        InitramfsNode::directory(TALOS_INDEX, &TALOS_ENTRIES),
        InitramfsNode::regular_file(SEED_INDEX, &INSUFFICIENT_SEED_BYTES),
    ];
    static SUFFICIENT_SEED_NODES: [InitramfsNode; 4] = [
        InitramfsNode::directory(ROOT_INDEX, &ROOT_ENTRIES),
        InitramfsNode::directory(ETC_INDEX, &ETC_ENTRIES),
        InitramfsNode::directory(TALOS_INDEX, &TALOS_ENTRIES),
        InitramfsNode::regular_file(SEED_INDEX, &SUFFICIENT_SEED_BYTES),
    ];
    static EMPTY_HOST_KEY_NODES: [InitramfsNode; 6] = [
        InitramfsNode::directory(ROOT_INDEX, &ROOT_ENTRIES),
        InitramfsNode::directory(ETC_INDEX, &ETC_ENTRIES),
        InitramfsNode::directory(TALOS_INDEX, &TALOS_WITH_SSH_ENTRIES),
        InitramfsNode::regular_file(SEED_INDEX, b"unused"),
        InitramfsNode::directory(SSH_INDEX, &SSH_ENTRIES),
        InitramfsNode::regular_file(HOST_KEY_INDEX, b""),
    ];
    static INSUFFICIENT_HOST_KEY_NODES: [InitramfsNode; 6] = [
        InitramfsNode::directory(ROOT_INDEX, &ROOT_ENTRIES),
        InitramfsNode::directory(ETC_INDEX, &ETC_ENTRIES),
        InitramfsNode::directory(TALOS_INDEX, &TALOS_WITH_SSH_ENTRIES),
        InitramfsNode::regular_file(SEED_INDEX, b"unused"),
        InitramfsNode::directory(SSH_INDEX, &SSH_ENTRIES),
        InitramfsNode::regular_file(HOST_KEY_INDEX, &INSUFFICIENT_HOST_KEY_BYTES),
    ];
    static OVERSIZED_HOST_KEY_NODES: [InitramfsNode; 6] = [
        InitramfsNode::directory(ROOT_INDEX, &ROOT_ENTRIES),
        InitramfsNode::directory(ETC_INDEX, &ETC_ENTRIES),
        InitramfsNode::directory(TALOS_INDEX, &TALOS_WITH_SSH_ENTRIES),
        InitramfsNode::regular_file(SEED_INDEX, b"unused"),
        InitramfsNode::directory(SSH_INDEX, &SSH_ENTRIES),
        InitramfsNode::regular_file(HOST_KEY_INDEX, &OVERSIZED_HOST_KEY_BYTES),
    ];
    static SUFFICIENT_HOST_KEY_AND_SEED_NODES: [InitramfsNode; 7] = [
        InitramfsNode::directory(ROOT_INDEX, &ROOT_ENTRIES),
        InitramfsNode::directory(ETC_INDEX, &ETC_ENTRIES),
        InitramfsNode::directory(TALOS_INDEX, &TALOS_WITH_SEED_AND_SSH_ENTRIES),
        InitramfsNode::regular_file(SEED_INDEX, &SUFFICIENT_SEED_BYTES),
        InitramfsNode::directory(SSH_INDEX, &SSH_WITH_SEED_ENTRIES),
        InitramfsNode::regular_file(HOST_KEY_INDEX, b"unused"),
        InitramfsNode::regular_file(HOST_KEY_AND_SEED_INDEX, SUFFICIENT_HOST_KEY_BYTES),
    ];
    static EMPTY_AUTHORIZED_KEY_NODES: [InitramfsNode; 8] = [
        InitramfsNode::directory(ROOT_INDEX, &ROOT_ENTRIES),
        InitramfsNode::directory(ETC_INDEX, &ETC_ENTRIES),
        InitramfsNode::directory(TALOS_INDEX, &TALOS_WITH_SEED_AND_SSH_ENTRIES),
        InitramfsNode::regular_file(SEED_INDEX, &SUFFICIENT_SEED_BYTES),
        InitramfsNode::directory(SSH_INDEX, &SSH_WITH_AUTHORIZED_KEY_ENTRIES),
        InitramfsNode::regular_file(HOST_KEY_INDEX, b"unused"),
        InitramfsNode::regular_file(HOST_KEY_AND_SEED_INDEX, SUFFICIENT_HOST_KEY_BYTES),
        InitramfsNode::regular_file(AUTHORIZED_KEY_INDEX, b""),
    ];
    static INSUFFICIENT_AUTHORIZED_KEY_NODES: [InitramfsNode; 8] = [
        InitramfsNode::directory(ROOT_INDEX, &ROOT_ENTRIES),
        InitramfsNode::directory(ETC_INDEX, &ETC_ENTRIES),
        InitramfsNode::directory(TALOS_INDEX, &TALOS_WITH_SEED_AND_SSH_ENTRIES),
        InitramfsNode::regular_file(SEED_INDEX, &SUFFICIENT_SEED_BYTES),
        InitramfsNode::directory(SSH_INDEX, &SSH_WITH_AUTHORIZED_KEY_ENTRIES),
        InitramfsNode::regular_file(HOST_KEY_INDEX, b"unused"),
        InitramfsNode::regular_file(HOST_KEY_AND_SEED_INDEX, SUFFICIENT_HOST_KEY_BYTES),
        InitramfsNode::regular_file(AUTHORIZED_KEY_INDEX, &INSUFFICIENT_AUTHORIZED_KEY_BYTES),
    ];
    static SUFFICIENT_HOST_KEY_SEED_AND_AUTHORIZED_KEY_NODES: [InitramfsNode; 8] = [
        InitramfsNode::directory(ROOT_INDEX, &ROOT_ENTRIES),
        InitramfsNode::directory(ETC_INDEX, &ETC_ENTRIES),
        InitramfsNode::directory(TALOS_INDEX, &TALOS_WITH_SEED_AND_SSH_ENTRIES),
        InitramfsNode::regular_file(SEED_INDEX, &SUFFICIENT_SEED_BYTES),
        InitramfsNode::directory(SSH_INDEX, &SSH_WITH_AUTHORIZED_KEY_ENTRIES),
        InitramfsNode::regular_file(HOST_KEY_INDEX, b"unused"),
        InitramfsNode::regular_file(HOST_KEY_AND_SEED_INDEX, SUFFICIENT_HOST_KEY_BYTES),
        InitramfsNode::regular_file(AUTHORIZED_KEY_INDEX, &SUFFICIENT_AUTHORIZED_KEY_BYTES),
    ];
    static OVERSIZED_AUTHORIZED_KEY_NODES: [InitramfsNode; 8] = [
        InitramfsNode::directory(ROOT_INDEX, &ROOT_ENTRIES),
        InitramfsNode::directory(ETC_INDEX, &ETC_ENTRIES),
        InitramfsNode::directory(TALOS_INDEX, &TALOS_WITH_SEED_AND_SSH_ENTRIES),
        InitramfsNode::regular_file(SEED_INDEX, &SUFFICIENT_SEED_BYTES),
        InitramfsNode::directory(SSH_INDEX, &SSH_WITH_AUTHORIZED_KEY_ENTRIES),
        InitramfsNode::regular_file(HOST_KEY_INDEX, b"unused"),
        InitramfsNode::regular_file(HOST_KEY_AND_SEED_INDEX, SUFFICIENT_HOST_KEY_BYTES),
        InitramfsNode::regular_file(AUTHORIZED_KEY_INDEX, &OVERSIZED_AUTHORIZED_KEY_BYTES),
    ];
    static INVALID_EXPOSURE_MARKER_NODES: [InitramfsNode; 9] = [
        InitramfsNode::directory(ROOT_INDEX, &ROOT_ENTRIES),
        InitramfsNode::directory(ETC_INDEX, &ETC_ENTRIES),
        InitramfsNode::directory(TALOS_INDEX, &TALOS_WITH_SEED_AND_SSH_ENTRIES),
        InitramfsNode::regular_file(SEED_INDEX, &SUFFICIENT_SEED_BYTES),
        InitramfsNode::directory(SSH_INDEX, &SSH_WITH_AUTHORIZED_KEY_AND_EXPOSURE_ENTRIES),
        InitramfsNode::regular_file(HOST_KEY_INDEX, b"unused"),
        InitramfsNode::regular_file(HOST_KEY_AND_SEED_INDEX, SUFFICIENT_HOST_KEY_BYTES),
        InitramfsNode::regular_file(AUTHORIZED_KEY_INDEX, &SUFFICIENT_AUTHORIZED_KEY_BYTES),
        InitramfsNode::regular_file(EXPOSURE_MARKER_INDEX, &OVERSIZED_EXPOSURE_MARKER_BYTES),
    ];
    static ENABLED_EXPOSURE_MARKER_NODES: [InitramfsNode; 9] = [
        InitramfsNode::directory(ROOT_INDEX, &ROOT_ENTRIES),
        InitramfsNode::directory(ETC_INDEX, &ETC_ENTRIES),
        InitramfsNode::directory(TALOS_INDEX, &TALOS_WITH_SEED_AND_SSH_ENTRIES),
        InitramfsNode::regular_file(SEED_INDEX, &SUFFICIENT_SEED_BYTES),
        InitramfsNode::directory(SSH_INDEX, &SSH_WITH_AUTHORIZED_KEY_AND_EXPOSURE_ENTRIES),
        InitramfsNode::regular_file(HOST_KEY_INDEX, b"unused"),
        InitramfsNode::regular_file(HOST_KEY_AND_SEED_INDEX, SUFFICIENT_HOST_KEY_BYTES),
        InitramfsNode::regular_file(AUTHORIZED_KEY_INDEX, &SUFFICIENT_AUTHORIZED_KEY_BYTES),
        InitramfsNode::regular_file(EXPOSURE_MARKER_INDEX, b""),
    ];

    const fn insufficient_seed_initramfs() -> ReadOnlyInitramfs {
        ReadOnlyInitramfs::new(&INSUFFICIENT_SEED_NODES, ROOT_INDEX)
    }

    const fn sufficient_seed_initramfs() -> ReadOnlyInitramfs {
        ReadOnlyInitramfs::new(&SUFFICIENT_SEED_NODES, ROOT_INDEX)
    }

    const fn empty_host_key_initramfs() -> ReadOnlyInitramfs {
        ReadOnlyInitramfs::new(&EMPTY_HOST_KEY_NODES, ROOT_INDEX)
    }

    const fn insufficient_host_key_initramfs() -> ReadOnlyInitramfs {
        ReadOnlyInitramfs::new(&INSUFFICIENT_HOST_KEY_NODES, ROOT_INDEX)
    }

    const fn oversized_host_key_initramfs() -> ReadOnlyInitramfs {
        ReadOnlyInitramfs::new(&OVERSIZED_HOST_KEY_NODES, ROOT_INDEX)
    }

    const fn sufficient_host_key_and_seed_initramfs() -> ReadOnlyInitramfs {
        ReadOnlyInitramfs::new(&SUFFICIENT_HOST_KEY_AND_SEED_NODES, ROOT_INDEX)
    }

    const fn empty_authorized_key_initramfs() -> ReadOnlyInitramfs {
        ReadOnlyInitramfs::new(&EMPTY_AUTHORIZED_KEY_NODES, ROOT_INDEX)
    }

    const fn insufficient_authorized_key_initramfs() -> ReadOnlyInitramfs {
        ReadOnlyInitramfs::new(&INSUFFICIENT_AUTHORIZED_KEY_NODES, ROOT_INDEX)
    }

    const fn sufficient_host_key_seed_and_authorized_key_initramfs() -> ReadOnlyInitramfs {
        ReadOnlyInitramfs::new(
            &SUFFICIENT_HOST_KEY_SEED_AND_AUTHORIZED_KEY_NODES,
            ROOT_INDEX,
        )
    }

    const fn oversized_authorized_key_initramfs() -> ReadOnlyInitramfs {
        ReadOnlyInitramfs::new(&OVERSIZED_AUTHORIZED_KEY_NODES, ROOT_INDEX)
    }

    const fn invalid_exposure_marker_initramfs() -> ReadOnlyInitramfs {
        ReadOnlyInitramfs::new(&INVALID_EXPOSURE_MARKER_NODES, ROOT_INDEX)
    }

    const fn enabled_exposure_marker_initramfs() -> ReadOnlyInitramfs {
        ReadOnlyInitramfs::new(&ENABLED_EXPOSURE_MARKER_NODES, ROOT_INDEX)
    }
}
