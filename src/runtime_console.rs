use core::fmt::{self, Write};

pub trait ConsoleBackend {
    fn write_str(&mut self, s: &str) -> fmt::Result;
}

#[cfg_attr(not(any(test, talos_qemu_polling_tty_rx_diagnostic)), allow(dead_code))]
pub trait ConsoleInputBackend {
    fn poll_read_byte(&mut self) -> Option<u8>;
}

impl<T> ConsoleBackend for T
where
    T: Write,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        Write::write_str(self, s)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeConsoleDevice {
    pub name: &'static str,
}

pub const DEFAULT_RUNTIME_CONSOLE: RuntimeConsoleDevice = RuntimeConsoleDevice {
    name: "runtime-console0",
};

pub struct RuntimeConsole<B> {
    device: RuntimeConsoleDevice,
    backend: B,
    bytes_written: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConsoleWriteResult {
    pub device: RuntimeConsoleDevice,
    pub bytes_written: usize,
}

impl ConsoleWriteResult {
    pub const fn complete(device: RuntimeConsoleDevice, bytes_written: usize) -> Self {
        Self {
            device,
            bytes_written,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConsoleWriteError {
    BackendWriteFailed {
        device: RuntimeConsoleDevice,
        bytes_accepted: usize,
    },
}

pub type ConsoleWriteOutcome = Result<ConsoleWriteResult, ConsoleWriteError>;

impl<B> RuntimeConsole<B>
where
    B: ConsoleBackend,
{
    pub const fn new(backend: B) -> Self {
        Self::with_device(DEFAULT_RUNTIME_CONSOLE, backend)
    }

    pub const fn with_device(device: RuntimeConsoleDevice, backend: B) -> Self {
        Self {
            device,
            backend,
            bytes_written: 0,
        }
    }

    pub fn write_kernel_args(&mut self, args: fmt::Arguments<'_>) -> ConsoleWriteOutcome {
        self.bytes_written = 0;

        let write_result = if let Some(s) = args.as_str() {
            Write::write_str(self, s)
        } else {
            self.write_fmt(args)
        };

        if write_result.is_ok() {
            Ok(ConsoleWriteResult::complete(
                self.device,
                self.bytes_written,
            ))
        } else {
            Err(ConsoleWriteError::BackendWriteFailed {
                device: self.device,
                bytes_accepted: self.bytes_written,
            })
        }
    }

    #[cfg(test)]
    pub fn into_backend(self) -> B {
        self.backend
    }
}

impl<B> Write for RuntimeConsole<B>
where
    B: ConsoleBackend,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let Some(bytes_written) = self.bytes_written.checked_add(s.len()) else {
            return Err(fmt::Error);
        };
        self.backend.write_str(s)?;
        self.bytes_written = bytes_written;
        Ok(())
    }
}

pub fn write_default_console_output<B>(backend: B, args: fmt::Arguments<'_>) -> ConsoleWriteOutcome
where
    B: ConsoleBackend,
{
    RuntimeConsole::new(backend).write_kernel_args(args)
}

#[cfg_attr(not(any(test, talos_qemu_polling_tty_rx_diagnostic)), allow(dead_code))]
pub fn poll_default_console_input<B>(backend: &mut B) -> Option<u8>
where
    B: ConsoleInputBackend,
{
    backend.poll_read_byte()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Capture {
        bytes: [u8; 64],
        len: usize,
        capacity: usize,
        writes: usize,
    }

    impl Capture {
        const fn new() -> Self {
            Self {
                bytes: [0; 64],
                len: 0,
                capacity: 64,
                writes: 0,
            }
        }

        const fn with_capacity(capacity: usize) -> Self {
            Self {
                bytes: [0; 64],
                len: 0,
                capacity,
                writes: 0,
            }
        }

        fn as_str(&self) -> &str {
            core::str::from_utf8(&self.bytes[..self.len]).expect("capture is utf8")
        }
    }

    impl ConsoleBackend for Capture {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            let Some(end) = self.len.checked_add(s.len()) else {
                return Err(fmt::Error);
            };
            if end > self.capacity || end > self.bytes.len() {
                return Err(fmt::Error);
            }

            self.bytes[self.len..end].copy_from_slice(s.as_bytes());
            self.len = end;
            self.writes += 1;
            Ok(())
        }
    }

    #[test_case]
    fn runtime_console_routes_static_kernel_text_to_backend() {
        let mut console = RuntimeConsole::new(Capture::new());

        let result = console.write_kernel_args(format_args!("talos")).unwrap();
        let backend = console.into_backend();

        assert_eq!(backend.as_str(), "talos");
        assert_eq!(result.device, DEFAULT_RUNTIME_CONSOLE);
        assert_eq!(result.bytes_written, 5);
        assert_eq!(backend.writes, 1);
    }

    #[test_case]
    fn runtime_console_names_the_default_kernel_console() {
        let result =
            write_default_console_output(Capture::new(), format_args!("identity")).unwrap();

        assert_eq!(result.device, DEFAULT_RUNTIME_CONSOLE);
        assert_eq!(result.device.name, "runtime-console0");
    }

    #[test_case]
    fn default_console_output_uses_named_runtime_console_boundary() {
        let result = write_default_console_output(Capture::new(), format_args!("console")).unwrap();

        assert_eq!(result.device.name, "runtime-console0");
        assert_eq!(result.bytes_written, 7);
    }

    #[test_case]
    fn runtime_console_routes_formatted_kernel_text_to_backend() {
        let mut console = RuntimeConsole::new(Capture::new());

        let result = console
            .write_kernel_args(format_args!("tick={}", 7))
            .unwrap();
        let backend = console.into_backend();

        assert_eq!(backend.as_str(), "tick=7");
        assert_eq!(result.device, DEFAULT_RUNTIME_CONSOLE);
        assert_eq!(result.bytes_written, 6);
        assert_eq!(backend.writes, 1);
    }

    #[test_case]
    fn runtime_console_reports_backend_write_failure() {
        let mut console = RuntimeConsole::new(Capture::with_capacity(4));

        let err = console
            .write_kernel_args(format_args!("too-long"))
            .unwrap_err();
        let backend = console.into_backend();

        assert_eq!(
            err,
            ConsoleWriteError::BackendWriteFailed {
                device: DEFAULT_RUNTIME_CONSOLE,
                bytes_accepted: 0,
            }
        );
        assert_eq!(backend.as_str(), "");
        assert_eq!(backend.writes, 0);
    }
}
