//! Classifications of primitive types

use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor,
    BitXorAssign, Div, DivAssign, Mul, MulAssign, Not, Rem, RemAssign, Shl,
    ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

use traitful::seal;

/// Trait implemented for integer primitives
#[seal(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize)]
pub trait Int: IntRequirements {}

#[doc(hidden)]
pub trait IntRequirements:
    Add<Output = Self>
    + AddAssign
    + BitAnd<Output = Self>
    + BitAndAssign
    + BitOr<Output = Self>
    + BitOrAssign
    + BitXor<Output = Self>
    + BitXorAssign
    + Div<Output = Self>
    + DivAssign
    + Mul<Output = Self>
    + MulAssign
    + Not<Output = Self>
    + Rem<Output = Self>
    + RemAssign
    + Shl<u8, Output = Self>
    + ShlAssign<u8>
    + Shr<u8, Output = Self>
    + ShrAssign<u8>
    + Sub<Output = Self>
    + SubAssign
    + Eq
    + PartialEq
    + Sized
    + Copy
    + Clone
    + 'static
{
}

impl<T> IntRequirements for T where
    T: Add<Output = Self>
        + AddAssign
        + BitAnd<Output = Self>
        + BitAndAssign
        + BitOr<Output = Self>
        + BitOrAssign
        + BitXor<Output = Self>
        + BitXorAssign
        + Div<Output = Self>
        + DivAssign
        + Mul<Output = Self>
        + MulAssign
        + Not<Output = Self>
        + Rem<Output = Self>
        + RemAssign
        + Shl<u8, Output = Self>
        + ShlAssign<u8>
        + Shr<u8, Output = Self>
        + ShrAssign<u8>
        + Sub<Output = Self>
        + SubAssign
        + Eq
        + PartialEq
        + Sized
        + Copy
        + Clone
        + 'static
{
}

impl Int for u8 {}
impl Int for i8 {}
impl Int for u16 {}
impl Int for i16 {}
impl Int for u32 {}
impl Int for i32 {}
impl Int for u64 {}
impl Int for i64 {}
impl Int for u128 {}
impl Int for i128 {}
impl Int for usize {}
impl Int for isize {}

/// Trait implemented for unsigned integer primitives
#[seal(u8, u16, u32, u64, u128, usize)]
pub trait UInt: UIntRequirements {
    /// The minimum value of an unsigned integer, 0
    #[doc(hidden)]
    const ZERO: Self;
    /// Size of the primitive, in bits
    #[doc(hidden)]
    const BITS: u8;

    /// Grab the little byte.
    #[doc(hidden)]
    fn little(self) -> u8;
}

#[doc(hidden)]
pub trait UIntRequirements: Int + From<u8> + TryInto<u128> {}

impl<T> UIntRequirements for T where T: Int + From<u8> + TryInto<u128> {}

impl UInt for u8 {
    const BITS: u8 = 8;
    const ZERO: u8 = u8::MIN;

    fn little(self) -> u8 {
        self
    }
}

impl UInt for u16 {
    const BITS: u8 = 16;
    const ZERO: u16 = u16::MIN;

    fn little(self) -> u8 {
        let [byte, _] = self.to_le_bytes();

        byte
    }
}

impl UInt for u32 {
    const BITS: u8 = 32;
    const ZERO: u32 = u32::MIN;

    fn little(self) -> u8 {
        let [byte, _, _, _] = self.to_le_bytes();

        byte
    }
}

impl UInt for u64 {
    const BITS: u8 = 64;
    const ZERO: u64 = u64::MIN;

    fn little(self) -> u8 {
        let [byte, _, _, _, _, _, _, _] = self.to_le_bytes();

        byte
    }
}

impl UInt for u128 {
    const BITS: u8 = 128;
    const ZERO: u128 = u128::MIN;

    fn little(self) -> u8 {
        let [byte, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _] =
            self.to_le_bytes();

        byte
    }
}

#[cfg(target_pointer_width = "16")]
impl UInt for usize {
    const BITS: u8 = 16;
    const ZERO: usize = usize::MIN;

    fn little(self) -> u8 {
        let [byte, _] = self.to_le_bytes();

        byte
    }
}

#[cfg(target_pointer_width = "32")]
impl UInt for usize {
    const BITS: u8 = 32;
    const ZERO: usize = usize::MIN;

    fn little(self) -> u8 {
        let [byte, _, _, _] = self.to_le_bytes();

        byte
    }
}

#[cfg(target_pointer_width = "64")]
impl UInt for usize {
    const BITS: u8 = 64;
    const ZERO: usize = usize::MIN;

    fn little(self) -> u8 {
        let [byte, _, _, _, _, _, _, _] = self.to_le_bytes();

        byte
    }
}
