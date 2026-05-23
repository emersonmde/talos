use core::fmt::{self, Write};

#[allow(dead_code)]
pub fn write_hex_usize(mut writer: impl Write, value: usize) -> fmt::Result {
    write_hex_u64(&mut writer, value as u64)
}

#[allow(dead_code)]
pub fn write_dec_usize(mut writer: impl Write, value: usize) -> fmt::Result {
    write_dec_u64(&mut writer, value as u64)
}

#[allow(dead_code)]
pub fn write_dec_i32(mut writer: impl Write, value: i32) -> fmt::Result {
    write_dec_i64(&mut writer, value as i64)
}

#[allow(dead_code)]
pub fn write_dec_u32(mut writer: impl Write, value: u32) -> fmt::Result {
    write_dec_u64(&mut writer, value as u64)
}

#[allow(dead_code)]
pub fn write_hex_u64(mut writer: impl Write, value: u64) -> fmt::Result {
    writer.write_str("0x")?;

    let mut started = false;
    for shift in (0..u64::BITS).step_by(4).rev() {
        let nibble = ((value >> shift) & 0xf) as usize;
        if nibble != 0 || started || shift == 0 {
            started = true;
            write_ascii_byte(&mut writer, hex_digit(nibble))?;
        }
    }

    Ok(())
}

#[allow(dead_code)]
fn hex_digit(nibble: usize) -> u8 {
    match nibble {
        0..=9 => b'0' + nibble as u8,
        10..=15 => b'a' + (nibble as u8 - 10),
        _ => b'?',
    }
}

#[allow(dead_code)]
pub fn write_dec_i64(mut writer: impl Write, value: i64) -> fmt::Result {
    if value < 0 {
        writer.write_str("-")?;
        write_dec_u64(&mut writer, value.wrapping_neg() as u64)
    } else {
        write_dec_u64(&mut writer, value as u64)
    }
}

#[allow(dead_code)]
pub fn write_dec_u64(mut writer: impl Write, mut value: u64) -> fmt::Result {
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
        write_ascii_byte(&mut writer, digits[len])?;
    }

    Ok(())
}

fn write_ascii_byte(writer: &mut impl Write, byte: u8) -> fmt::Result {
    let bytes = [byte];
    // Keep generated digits stack-backed so diagnostic formatting does not
    // depend on a static digit table in early boot.
    let digit = unsafe { core::str::from_utf8_unchecked(&bytes) };
    writer.write_str(digit)
}

#[cfg(test)]
mod tests {
    use super::{
        write_dec_i32, write_dec_i64, write_dec_u32, write_dec_u64, write_dec_usize,
        write_hex_usize,
    };
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

        let mut wide = Buffer::new();
        write_dec_u64(&mut wide, 18_446_744_073_709_551_615).expect("format u64");
        assert_eq!(wide.as_str(), "18446744073709551615");

        let mut u32_value = Buffer::new();
        write_dec_u32(&mut u32_value, 123_456_789).expect("format u32");
        assert_eq!(u32_value.as_str(), "123456789");

        let mut negative = Buffer::new();
        write_dec_i32(&mut negative, -12345).expect("format i32");
        assert_eq!(negative.as_str(), "-12345");

        let mut min_i64 = Buffer::new();
        write_dec_i64(&mut min_i64, i64::MIN).expect("format i64 min");
        assert_eq!(min_i64.as_str(), "-9223372036854775808");
    }
}
