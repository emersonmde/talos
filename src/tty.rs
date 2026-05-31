use crate::runtime_console::{self, ConsoleInputBackend, ConsoleInputPollOutcome};

pub const CANONICAL_LINE_CAPACITY: usize = 16;
pub const CANONICAL_ECHO_CAPACITY: usize = 32;
pub const CONTROL_EVENT_CAPACITY: usize = 8;
pub const POLLING_RX_WAIT_LIMIT: usize = 2_000_000;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TtyControlEvent {
    Interrupt,
    EndOfInput,
    Suspend,
    ClearLine,
    Unsupported(u8),
}

impl TtyControlEvent {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Interrupt => "ctrl-c",
            Self::EndOfInput => "ctrl-d",
            Self::Suspend => "ctrl-z",
            Self::ClearLine => "ctrl-u",
            Self::Unsupported(_) => "unsupported-control",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TtyMode {
    Raw,
    CanonicalLite,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TtyInputOutcome {
    Pending,
    LineComplete,
    RawByte(u8),
    BufferLimit,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PollingTtyRxOutcome {
    Pending,
    LineComplete,
    Timeout,
    InputUnavailable,
    BackendError,
}

impl PollingTtyRxOutcome {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::LineComplete => "line-complete",
            Self::Timeout => "timeout",
            Self::InputUnavailable => "input-unavailable",
            Self::BackendError => "backend-error",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TtyLineDiscipline {
    mode: TtyMode,
    line: [u8; CANONICAL_LINE_CAPACITY],
    line_len: usize,
    echo: [u8; CANONICAL_ECHO_CAPACITY],
    echo_len: usize,
    controls: [Option<TtyControlEvent>; CONTROL_EVENT_CAPACITY],
    control_len: usize,
    raw_bytes: usize,
    backspaces: usize,
    deletes: usize,
    truncated: bool,
    terminated: bool,
}

impl TtyLineDiscipline {
    pub const fn canonical_lite() -> Self {
        Self::new(TtyMode::CanonicalLite)
    }

    pub const fn raw() -> Self {
        Self::new(TtyMode::Raw)
    }

    const fn new(mode: TtyMode) -> Self {
        Self {
            mode,
            line: [0; CANONICAL_LINE_CAPACITY],
            line_len: 0,
            echo: [0; CANONICAL_ECHO_CAPACITY],
            echo_len: 0,
            controls: [None; CONTROL_EVENT_CAPACITY],
            control_len: 0,
            raw_bytes: 0,
            backspaces: 0,
            deletes: 0,
            truncated: false,
            terminated: false,
        }
    }

    pub const fn mode(&self) -> TtyMode {
        self.mode
    }

    pub fn line(&self) -> &[u8] {
        &self.line[..self.line_len]
    }

    pub fn echo(&self) -> &[u8] {
        &self.echo[..self.echo_len]
    }

    pub fn controls(&self) -> &[Option<TtyControlEvent>] {
        &self.controls[..self.control_len]
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

    pub const fn terminated(&self) -> bool {
        self.terminated
    }

    pub fn process_byte(&mut self, byte: u8) -> TtyInputOutcome {
        if self.terminated {
            return TtyInputOutcome::LineComplete;
        }

        self.raw_bytes += 1;

        match self.mode {
            TtyMode::Raw => self.accept_raw_byte(byte),
            TtyMode::CanonicalLite => self.accept_canonical_byte(byte),
        }
    }

    fn accept_raw_byte(&mut self, byte: u8) -> TtyInputOutcome {
        if self.push_line_byte(byte) {
            TtyInputOutcome::RawByte(byte)
        } else {
            TtyInputOutcome::BufferLimit
        }
    }

    fn accept_canonical_byte(&mut self, byte: u8) -> TtyInputOutcome {
        match byte {
            b'\r' | b'\n' => {
                self.push_echo(b'\r');
                self.push_echo(b'\n');
                self.terminated = true;
                TtyInputOutcome::LineComplete
            }
            0x08 => {
                self.backspaces += 1;
                if self.line_len > 0 {
                    self.line_len -= 1;
                    self.push_erase_echo();
                }
                TtyInputOutcome::Pending
            }
            0x7f => {
                self.deletes += 1;
                if self.line_len > 0 {
                    self.line_len -= 1;
                    self.push_erase_echo();
                }
                TtyInputOutcome::Pending
            }
            0x03 => self.push_control(TtyControlEvent::Interrupt),
            0x04 => self.push_control(TtyControlEvent::EndOfInput),
            0x1a => self.push_control(TtyControlEvent::Suspend),
            0x15 => {
                self.line_len = 0;
                self.push_control(TtyControlEvent::ClearLine)
            }
            0x1b => {
                if self.push_line_byte(byte) {
                    TtyInputOutcome::Pending
                } else {
                    TtyInputOutcome::BufferLimit
                }
            }
            b'\t' | 0x20..=0x7e => {
                if self.push_printable(byte) {
                    TtyInputOutcome::Pending
                } else {
                    TtyInputOutcome::BufferLimit
                }
            }
            0x00..=0x1f => self.push_control(TtyControlEvent::Unsupported(byte)),
            _ => self.push_control(TtyControlEvent::Unsupported(byte)),
        }
    }

    fn push_printable(&mut self, byte: u8) -> bool {
        if self.push_line_byte(byte) {
            self.push_echo(byte);
            true
        } else {
            false
        }
    }

    fn push_line_byte(&mut self, byte: u8) -> bool {
        if self.line_len < self.line.len() {
            self.line[self.line_len] = byte;
            self.line_len += 1;
            true
        } else {
            self.truncated = true;
            false
        }
    }

    fn push_control(&mut self, event: TtyControlEvent) -> TtyInputOutcome {
        if self.control_len < self.controls.len() {
            self.controls[self.control_len] = Some(event);
            self.control_len += 1;
        }
        TtyInputOutcome::Pending
    }

    fn push_erase_echo(&mut self) {
        self.push_echo(0x08);
        self.push_echo(b' ');
        self.push_echo(0x08);
    }

    fn push_echo(&mut self, byte: u8) {
        if self.echo_len < self.echo.len() {
            self.echo[self.echo_len] = byte;
            self.echo_len += 1;
        }

        #[cfg(not(test))]
        write_echo_byte(byte);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PollingTtyRxResult {
    discipline: TtyLineDiscipline,
    outcome: PollingTtyRxOutcome,
}

impl PollingTtyRxResult {
    const fn new() -> Self {
        Self {
            discipline: TtyLineDiscipline::canonical_lite(),
            outcome: PollingTtyRxOutcome::Pending,
        }
    }

    pub const fn mode(&self) -> TtyMode {
        self.discipline.mode()
    }

    pub fn line(&self) -> &[u8] {
        self.discipline.line()
    }

    pub fn echo(&self) -> &[u8] {
        self.discipline.echo()
    }

    pub fn controls(&self) -> &[Option<TtyControlEvent>] {
        self.discipline.controls()
    }

    pub const fn raw_bytes(&self) -> usize {
        self.discipline.raw_bytes()
    }

    pub const fn backspaces(&self) -> usize {
        self.discipline.backspaces()
    }

    pub const fn deletes(&self) -> usize {
        self.discipline.deletes()
    }

    pub const fn truncated(&self) -> bool {
        self.discipline.truncated()
    }

    pub const fn terminated(&self) -> bool {
        self.discipline.terminated()
    }

    pub const fn timed_out(&self) -> bool {
        matches!(self.outcome, PollingTtyRxOutcome::Timeout)
    }

    pub const fn outcome(&self) -> PollingTtyRxOutcome {
        self.outcome
    }

    pub const fn outcome_name(&self) -> &'static str {
        self.outcome.name()
    }

    pub const fn passed(&self) -> bool {
        matches!(self.outcome, PollingTtyRxOutcome::LineComplete) && self.discipline.raw_bytes() > 0
    }

    fn mark_timeout(&mut self) {
        self.outcome = PollingTtyRxOutcome::Timeout;
    }

    fn mark_input_unavailable(&mut self) {
        self.outcome = PollingTtyRxOutcome::InputUnavailable;
    }

    fn mark_backend_error(&mut self) {
        self.outcome = PollingTtyRxOutcome::BackendError;
    }

    fn accept_byte(&mut self, byte: u8) {
        if self.discipline.process_byte(byte) == TtyInputOutcome::LineComplete {
            self.outcome = PollingTtyRxOutcome::LineComplete;
        }
    }
}

pub fn run_polling_rx_diagnostic<B>(backend: B) -> PollingTtyRxResult
where
    B: ConsoleInputBackend,
{
    run_polling_rx_diagnostic_with_limit(backend, POLLING_RX_WAIT_LIMIT)
}

pub fn run_polling_rx_diagnostic_with_limit<B>(
    mut backend: B,
    wait_limit: usize,
) -> PollingTtyRxResult
where
    B: ConsoleInputBackend,
{
    let mut result = PollingTtyRxResult::new();
    let mut idle_polls = 0usize;

    while !result.terminated() {
        match runtime_console::poll_default_console_input(&mut backend) {
            ConsoleInputPollOutcome::ByteAvailable { byte, .. } => {
                idle_polls = 0;
                result.accept_byte(byte);
            }
            ConsoleInputPollOutcome::NoData { .. } => {
                if idle_polls >= wait_limit {
                    result.mark_timeout();
                    break;
                }
                idle_polls += 1;
                core::hint::spin_loop();
            }
            ConsoleInputPollOutcome::BackendUnavailable { .. } => {
                result.mark_input_unavailable();
                break;
            }
            ConsoleInputPollOutcome::BackendError { .. } => {
                result.mark_backend_error();
                break;
            }
        }
    }

    result
}

#[cfg(not(test))]
fn write_echo_byte(byte: u8) {
    let bytes = [byte];
    if let Ok(s) = core::str::from_utf8(&bytes) {
        crate::target::console::write_static(s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ScriptedInput {
        bytes: [u8; 32],
        len: usize,
        pos: usize,
    }

    impl ScriptedInput {
        const fn new(bytes: [u8; 32], len: usize) -> Self {
            Self { bytes, len, pos: 0 }
        }
    }

    impl ConsoleInputBackend for ScriptedInput {
        fn poll_read_byte(&mut self) -> Option<u8> {
            if self.pos == self.len {
                return None;
            }
            let byte = self.bytes[self.pos];
            self.pos += 1;
            Some(byte)
        }
    }

    #[test_case]
    fn line_discipline_collapses_newline_forms_to_crlf_echo() {
        let mut cr_line = TtyLineDiscipline::canonical_lite();
        let mut lf_line = TtyLineDiscipline::canonical_lite();

        assert_eq!(cr_line.process_byte(b'a'), TtyInputOutcome::Pending);
        assert_eq!(cr_line.process_byte(b'\r'), TtyInputOutcome::LineComplete);
        assert_eq!(lf_line.process_byte(b'a'), TtyInputOutcome::Pending);
        assert_eq!(lf_line.process_byte(b'\n'), TtyInputOutcome::LineComplete);

        assert_eq!(cr_line.line(), b"a");
        assert_eq!(lf_line.line(), b"a");
        assert_eq!(cr_line.echo(), b"a\r\n");
        assert_eq!(lf_line.echo(), b"a\r\n");
        assert!(cr_line.terminated());
        assert!(lf_line.terminated());
    }

    #[test_case]
    fn line_discipline_handles_tab_printable_and_editing_bytes() {
        let mut tty = TtyLineDiscipline::canonical_lite();

        for byte in [b'a', b'\t', 0x1b, b'X', 0x08, b'Y', 0x7f, b'b'] {
            tty.process_byte(byte);
        }

        assert_eq!(tty.line(), &[b'a', b'\t', 0x1b, b'b']);
        assert_eq!(tty.echo(), b"a\tX\x08 \x08Y\x08 \x08b");
        assert_eq!(tty.backspaces(), 1);
        assert_eq!(tty.deletes(), 1);
        assert_eq!(tty.controls(), &[]);
    }

    #[test_case]
    fn line_discipline_records_deferred_control_event_names() {
        let mut tty = TtyLineDiscipline::canonical_lite();

        for byte in [0x03, 0x04, 0x1a, 0x15, 0x01] {
            tty.process_byte(byte);
        }

        assert_eq!(
            tty.controls(),
            &[
                Some(TtyControlEvent::Interrupt),
                Some(TtyControlEvent::EndOfInput),
                Some(TtyControlEvent::Suspend),
                Some(TtyControlEvent::ClearLine),
                Some(TtyControlEvent::Unsupported(0x01)),
            ]
        );
        assert_eq!(tty.controls()[0].unwrap().name(), "ctrl-c");
        assert_eq!(tty.controls()[1].unwrap().name(), "ctrl-d");
        assert_eq!(tty.controls()[2].unwrap().name(), "ctrl-z");
        assert_eq!(tty.controls()[3].unwrap().name(), "ctrl-u");
        assert_eq!(tty.controls()[4].unwrap().name(), "unsupported-control");
        assert_eq!(tty.line(), b"");
        assert_eq!(tty.echo(), b"");
    }

    #[test_case]
    fn line_discipline_reports_buffer_limit_without_implicit_timeout() {
        let mut tty = TtyLineDiscipline::canonical_lite();

        for byte in b"abcdefghijklmnop" {
            assert_eq!(tty.process_byte(*byte), TtyInputOutcome::Pending);
        }
        assert_eq!(tty.process_byte(b'q'), TtyInputOutcome::BufferLimit);

        assert_eq!(tty.line(), b"abcdefghijklmnop");
        assert!(tty.truncated());
        assert!(!tty.terminated());
    }

    #[test_case]
    fn raw_mode_passes_bytes_without_canonical_translation_or_echo() {
        let mut tty = TtyLineDiscipline::raw();

        assert_eq!(tty.process_byte(b'a'), TtyInputOutcome::RawByte(b'a'));
        assert_eq!(tty.process_byte(0x03), TtyInputOutcome::RawByte(0x03));
        assert_eq!(tty.process_byte(b'\n'), TtyInputOutcome::RawByte(b'\n'));

        assert_eq!(tty.mode(), TtyMode::Raw);
        assert_eq!(tty.line(), &[b'a', 0x03, b'\n']);
        assert_eq!(tty.echo(), b"");
        assert_eq!(tty.controls(), &[]);
        assert_eq!(tty.raw_bytes(), 3);
        assert!(!tty.terminated());
    }

    #[test_case]
    fn canonical_lite_applies_backspace_delete_control_truncation_and_newline() {
        let result = run_polling_rx_diagnostic_with_limit(
            ScriptedInput::new(
                [
                    b'a', b'b', b'X', 0x08, b'c', b'Y', 0x7f, b'd', 0x03, b'e', b'f', b'g', b'h',
                    b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'\r', 0, 0, 0, 0, 0, 0,
                    0, 0, 0,
                ],
                23,
            ),
            8,
        );

        assert!(result.passed());
        assert_eq!(result.outcome(), PollingTtyRxOutcome::LineComplete);
        assert_eq!(result.outcome_name(), "line-complete");
        assert_eq!(result.line(), b"abcdefghijklmnop");
        assert_eq!(result.echo(), b"abX\x08 \x08cY\x08 \x08defghijklmnop\r\n");
        assert_eq!(result.raw_bytes(), 23);
        assert_eq!(result.backspaces(), 1);
        assert_eq!(result.deletes(), 1);
        assert!(result.truncated());
        assert_eq!(result.controls(), &[Some(TtyControlEvent::Interrupt)]);
    }

    #[test_case]
    fn polling_rx_diagnostic_reports_bounded_timeout_without_input() {
        let result = run_polling_rx_diagnostic_with_limit(ScriptedInput::new([0; 32], 0), 2);

        assert!(!result.passed());
        assert!(result.timed_out());
        assert_eq!(result.outcome(), PollingTtyRxOutcome::Timeout);
        assert_eq!(result.outcome_name(), "timeout");
        assert_eq!(result.raw_bytes(), 0);
        assert_eq!(result.line(), b"");
    }
}
