use crate::{
    entropy::{self, EntropyDiagnosticSnapshot},
    runtime_console::{self, ConsoleBackend, DEFAULT_RUNTIME_CONSOLE},
    tty::CANONICAL_LINE_CAPACITY,
};

pub const DIAGNOSTIC_COMMAND_CHANNEL_VERSION: &str = "phase5.3-contract-v1";
pub const MAX_COMMAND_TOKEN_BYTES: usize = 16;
pub const MAX_COMMAND_ARGUMENTS: usize = 2;
pub const DEFAULT_COMMAND_COUNT: usize = 4;

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
        "entropy" => write_entropy_response(sink),
        "help" => write_help_response(sink),
        "list" => write_list_response(sink),
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
    write_line(
        sink,
        &mut responses,
        "diag: commands entropy help list status",
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
    write_line(
        sink,
        &mut responses,
        "diag: commands entropy help list status",
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
    write_line(
        sink,
        &mut responses,
        "diag: commands entropy help list status",
    )?;
    Ok(DiagnosticDispatchResult {
        status: DiagnosticDispatchStatus::Handled,
        response_lines: responses,
    })
}

fn write_entropy_response(
    sink: &mut impl DiagnosticResponseSink,
) -> Result<DiagnosticDispatchResult, DiagnosticDispatchError> {
    let report = entropy::classify_entropy_snapshot(EntropyDiagnosticSnapshot::empty());
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

    struct CaptureSink {
        bytes: [u8; 384],
        len: usize,
        fail_after: usize,
        writes: usize,
    }

    impl CaptureSink {
        const fn new() -> Self {
            Self {
                bytes: [0; 384],
                len: 0,
                fail_after: usize::MAX,
                writes: 0,
            }
        }

        const fn failing_after(fail_after: usize) -> Self {
            Self {
                bytes: [0; 384],
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
            "diag: ok help\ndiag: commands entropy help list status\n"
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
            "diag: ok status\ndiag: version phase5.3-contract-v1\ndiag: runtime-console runtime-console0\ndiag: tty canonical-lite line-capacity 64\ndiag: command-count 4\ndiag: commands entropy help list status\n"
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
}
