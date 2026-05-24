use crate::runtime_console::{self, ConsoleInputBackend};

pub const CANONICAL_LINE_CAPACITY: usize = 8;
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
pub struct PollingTtyRxResult {
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
    timed_out: bool,
}

impl PollingTtyRxResult {
    const fn new() -> Self {
        Self {
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
            timed_out: false,
        }
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

    pub const fn timed_out(&self) -> bool {
        self.timed_out
    }

    pub const fn passed(&self) -> bool {
        self.terminated && !self.timed_out && self.raw_bytes > 0
    }

    fn mark_timeout(&mut self) {
        self.timed_out = true;
    }

    fn accept_byte(&mut self, byte: u8) {
        self.raw_bytes += 1;

        match byte {
            b'\r' | b'\n' => {
                self.push_echo(b'\r');
                self.push_echo(b'\n');
                self.terminated = true;
            }
            0x08 => {
                self.backspaces += 1;
                if self.line_len > 0 {
                    self.line_len -= 1;
                    self.push_erase_echo();
                }
            }
            0x7f => {
                self.deletes += 1;
                if self.line_len > 0 {
                    self.line_len -= 1;
                    self.push_erase_echo();
                }
            }
            0x03 => self.push_control(TtyControlEvent::Interrupt),
            0x04 => self.push_control(TtyControlEvent::EndOfInput),
            0x1a => self.push_control(TtyControlEvent::Suspend),
            0x15 => {
                self.line_len = 0;
                self.push_control(TtyControlEvent::ClearLine);
            }
            b'\t' | 0x20..=0x7e => self.push_printable(byte),
            0x00..=0x1f => self.push_control(TtyControlEvent::Unsupported(byte)),
            _ => self.push_control(TtyControlEvent::Unsupported(byte)),
        }
    }

    fn push_printable(&mut self, byte: u8) {
        if self.line_len < self.line.len() {
            self.line[self.line_len] = byte;
            self.line_len += 1;
            self.push_echo(byte);
        } else {
            self.truncated = true;
        }
    }

    fn push_control(&mut self, event: TtyControlEvent) {
        if self.control_len < self.controls.len() {
            self.controls[self.control_len] = Some(event);
            self.control_len += 1;
        }
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
        if let Some(byte) = runtime_console::poll_default_console_input(&mut backend) {
            idle_polls = 0;
            result.accept_byte(byte);
        } else {
            if idle_polls >= wait_limit {
                result.mark_timeout();
                break;
            }
            idle_polls += 1;
            core::hint::spin_loop();
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
        bytes: [u8; 16],
        len: usize,
        pos: usize,
    }

    impl ScriptedInput {
        const fn new(bytes: [u8; 16], len: usize) -> Self {
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
    fn canonical_lite_applies_backspace_delete_control_truncation_and_newline() {
        let result = run_polling_rx_diagnostic_with_limit(
            ScriptedInput::new(
                [
                    b'a', b'b', b'X', 0x08, b'c', b'Y', 0x7f, b'd', 0x03, b'e', b'f', b'g', b'h',
                    b'i', b'\r', 0,
                ],
                15,
            ),
            8,
        );

        assert!(result.passed());
        assert_eq!(result.line(), b"abcdefgh");
        assert_eq!(result.echo(), b"abX\x08 \x08cY\x08 \x08defgh\r\n");
        assert_eq!(result.raw_bytes(), 15);
        assert_eq!(result.backspaces(), 1);
        assert_eq!(result.deletes(), 1);
        assert!(result.truncated());
        assert_eq!(result.controls(), &[Some(TtyControlEvent::Interrupt)]);
    }

    #[test_case]
    fn polling_rx_diagnostic_reports_bounded_timeout_without_input() {
        let result = run_polling_rx_diagnostic_with_limit(ScriptedInput::new([0; 16], 0), 2);

        assert!(!result.passed());
        assert!(result.timed_out());
        assert_eq!(result.raw_bytes(), 0);
        assert_eq!(result.line(), b"");
    }
}
