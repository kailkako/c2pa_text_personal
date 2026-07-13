//! Formatting options for slices.

use core::fmt::*;

/// Format a slice in a Rust syntax.
/// Supports formatting using the following traits:
///		- Display
///		- Debug
///		- Binary
///		- Octal
///		- LowerHex, UpperHex
///		- LowerExp, UpperExp
///		- Pointer
///
/// # Examples
/// ```
/// use extfmt::*;
///
/// assert_eq!(format!("{}", CommaSeparated(&[1, 2, 3])), "[1, 2, 3]");
/// assert_eq!(format!("{:x}", CommaSeparated(&[122, 123, 134])), "[7a, 7b, 86]");
/// assert_eq!(format!("{:#o}", CommaSeparated(&[122, 123, 134])), "[0o172, 0o173, 0o206]");
/// ```
#[derive(Clone)]
pub struct CommaSeparated<'a, T: 'a>(pub &'a [T]);

macro_rules! additional_slice_formatting {
    ($($tr:ident),*) => {
        $(
        impl<'a, T: 'a + $tr> $tr for CommaSeparated<'a, T> {
            fn fmt(&self, f: &mut Formatter) -> Result {
                let mut has_items = false;

                f.write_str("[")?;

                for i in self.0.iter() {
                    if has_items {
                        f.write_str(", ")?;
                    }

                    <T as $tr>::fmt(i, f)?;
                    has_items = true;
                }

                f.write_str("]")
            }
        }
        )*
    }
}

additional_slice_formatting!(
    Display, Debug, Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp, Pointer
);

/// Formats a byte-buffer as a series of concatanated hex-pairs.
///
/// Implements the following formatting traits:
///		- Display, Debug, LowerHex - Will format in all letters in lowercase.
///		- UpperHex - Will format in all letters in uppercase
///
/// # Examples
/// ```
/// use extfmt::*;
///
/// assert_eq!(format!("{}", Hexlify(&[122, 123, 134])), "7a7b86");
/// assert_eq!(format!("{:x}", Hexlify(&[122, 123, 134])), "7a7b86");
/// assert_eq!(format!("{:X}", Hexlify(&[122, 123, 134])), "7A7B86");
/// ```
pub struct Hexlify<'a>(pub &'a [u8]);

impl<'a> LowerHex for Hexlify<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for byte in self.0.iter() {
            write!(f, "{:02x}", byte)?;
        }

        Ok(())
    }
}

impl<'a> UpperHex for Hexlify<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for byte in self.0.iter() {
            write!(f, "{:02X}", byte)?;
        }

        Ok(())
    }
}

impl<'a> Debug for Hexlify<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        <Self as LowerHex>::fmt(self, f)
    }
}

impl<'a> Display for Hexlify<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        <Self as LowerHex>::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readme() {
        assert_eq!(
            format!("{:02x}", CommaSeparated(&[1, 2, 255, 64])),
            "[01, 02, ff, 40]"
        );
        assert_eq!(format!("{}", Hexlify(&[1, 2, 255, 64])), "0102ff40");
    }
}
