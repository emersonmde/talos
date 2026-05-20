use core::fmt::{self, Write};

const HEX_DIGITS: &[u8; 16] = b"0123456789abcdef";

pub fn write_hex_usize(mut writer: impl Write, value: usize) -> fmt::Result {
    write_hex_u64(&mut writer, value as u64)
}

pub fn write_hex_u64(mut writer: impl Write, value: u64) -> fmt::Result {
    writer.write_str("0x")?;

    let mut started = false;
    for shift in (0..u64::BITS).step_by(4).rev() {
        let nibble = ((value >> shift) & 0xf) as usize;
        if nibble != 0 || started || shift == 0 {
            started = true;
            writer.write_char(HEX_DIGITS[nibble] as char)?;
        }
    }

    Ok(())
}

pub fn write_dec_usize(mut writer: impl Write, mut value: usize) -> fmt::Result {
    if value == 0 {
        return writer.write_str("0");
    }

    let mut digits = [0u8; 20];
    let mut len = 0;
    while value != 0 {
        digits[len] = b'0' + (value % 10) as u8;
        value /= 10;
        len += 1;
    }

    while len != 0 {
        len -= 1;
        writer.write_char(digits[len] as char)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{write_dec_usize, write_hex_usize};
    use core::fmt;

    struct Buffer {
        bytes: [u8; 32],
        len: usize,
    }

    impl Buffer {
        const fn new() -> Self {
            Self {
                bytes: [0; 32],
                len: 0,
            }
        }

        fn as_str(&self) -> &str {
            core::str::from_utf8(&self.bytes[..self.len]).expect("valid ascii")
        }
    }

    impl fmt::Write for Buffer {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            let end = self.len + s.len();
            self.bytes[self.len..end].copy_from_slice(s.as_bytes());
            self.len = end;
            Ok(())
        }
    }

    #[test_case]
    fn hex_formatter_uses_minimal_digits() {
        let mut buffer = Buffer::new();
        write_hex_usize(&mut buffer, 0x10_7d00_1000).expect("format hex");
        assert_eq!(buffer.as_str(), "0x107d001000");
    }

    #[test_case]
    fn hex_formatter_writes_full_width_u64_values() {
        let mut buffer = Buffer::new();
        super::write_hex_u64(&mut buffer, 0xfedc_ba98_7654_3210).expect("format hex");
        assert_eq!(buffer.as_str(), "0xfedcba9876543210");
    }

    #[test_case]
    fn decimal_formatter_writes_zero_and_multi_digit_values() {
        let mut zero = Buffer::new();
        write_dec_usize(&mut zero, 0).expect("format zero");
        assert_eq!(zero.as_str(), "0");

        let mut value = Buffer::new();
        write_dec_usize(&mut value, 12345).expect("format decimal");
        assert_eq!(value.as_str(), "12345");
    }
}
