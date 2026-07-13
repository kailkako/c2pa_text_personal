use core::fmt::*;

/// A formatting aid for pretty-printing a byte buffer,
/// not unlike the `hexdump` utility.
///
/// # Examples
/// ```
/// use extfmt::*;
///
/// let dump = Hexdump::new(&[1,2,3,4,5,6,255]);
/// assert_eq!(format!("{}", dump), "00000000\t01 02 03 04 05 06 ff");
/// ```
pub struct Hexdump<'a> {
    data: &'a [u8],
    show_index: bool,
    items_per_row: usize,
}

impl<'a> Hexdump<'a> {
    /// Creates a new Hexdump instance from a byte slice.
    pub fn new(data: &'a [u8]) -> Self {
        Hexdump {
            data,
            show_index: true,
            items_per_row: 16,
        }
    }

    /// Controls whether or not to show the current index at the beginning of each row.
    ///
    /// Default: true
    pub fn show_index(&mut self, value: bool) -> &mut Self {
        self.show_index = value;
        self
    }

    /// Controls the amount of bytes to print in each row.
    ///
    /// Default: 16
    pub fn items_per_row(&mut self, value: usize) -> &mut Self {
        self.items_per_row = value;
        self
    }
}

impl<'a> Display for Hexdump<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for i in 0..self.data.len() {
            if self.show_index && i % self.items_per_row == 0 {
                write!(f, "{:08x}\t", i)?;
            }

            write!(f, "{:02x}", self.data[i])?;

            // Items separator
            if i != self.data.len() - 1 {
                // Start a new row when appropriate
                if i % self.items_per_row == self.items_per_row - 1 {
                    f.write_char('\n')?;
                } else {
                    f.write_char(' ')?;
                }
            }
        }

        Ok(())
    }
}

/// A utility trait used to create Hexdump objects.
pub trait AsHexdump {
    fn as_hexdump(&self) -> Hexdump<'_>;
}

impl<T: AsRef<[u8]>> AsHexdump for T {
    fn as_hexdump(&self) -> Hexdump<'_> {
        Hexdump::new(self.as_ref())
    }
}

/// Create a hexdump of the given value using the AsHexdump trait.
/// It is implemented by default for Sized types and for [u8].
///
/// `key: value` can be added to control formatting options, where `key` is a method of the `Hexdump` struct.
///
/// # Examples
/// ```
/// use extfmt::*;
/// let data = &[1u8,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16];
/// let expected = "00000000\t01 02 03 04 05 06 07 08 09 0a 0b 0c\n0000000c\t0d 0e 0f 10";
/// assert_eq!(format!("{}", hexdump!(data, items_per_row: 12)), expected);
/// ```
#[macro_export]
macro_rules! hexdump {
    ($value:expr) => (
    	$value.as_hexdump()
    );
    ($value:expr, $($setting:ident: $setting_value: expr)*) => (
    	$value.as_hexdump() $(
    		.$setting($setting_value)
    		)*
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readme() {
        assert_eq!(
            format!("{}", hexdump!(&[1u8, 2, 255, 64])),
            "00000000	01 02 ff 40"
        );
        assert_eq!(
            format!("{}", hexdump!(64i32.to_le_bytes())),
            "00000000	40 00 00 00"
        );
        assert_eq!(
            format!("{}", hexdump!(64i32.to_le_bytes(), show_index: false)),
            "40 00 00 00"
        );
    }
}
