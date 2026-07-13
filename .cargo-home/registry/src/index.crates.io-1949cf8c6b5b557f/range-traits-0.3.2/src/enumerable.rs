use crate::MaybeBounded;

/// Type for which instances can be partially enumerated.
pub trait PartialEnum: PartialOrd + MaybeBounded {
	fn pred(&self) -> Option<Self>;

	fn succ(&self) -> Option<Self>;
}

/// Type for which instances can be entirely enumerated.
pub trait Enum: PartialOrd + MaybeBounded {
	fn pred(&self) -> Option<Self>;

	fn succ(&self) -> Option<Self>;
}

impl<T: Enum> PartialEnum for T {
	fn pred(&self) -> Option<Self> {
		self.pred()
	}

	fn succ(&self) -> Option<Self> {
		self.succ()
	}
}

impl Enum for char {
	fn pred(&self) -> Option<Self> {
		('\u{0000}'..*self).next_back()
	}

	fn succ(&self) -> Option<Self> {
		// we skip one element since that's the current char, not its successor
		(*self..='\u{10ffff}').nth(1)
	}
}

macro_rules! impl_int {
	($($ty:ident),*) => {
		$(
			impl Enum for $ty {
				fn pred(&self) -> Option<Self> {
					self.checked_sub(1)
				}

				fn succ(&self) -> Option<Self> {
					self.checked_add(1)
				}
			}
		)*
	};
}

impl_int!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

macro_rules! impl_float {
	($($ty:ident),*) => {
		$(
			impl PartialEnum for $ty {
				fn pred(&self) -> Option<Self> {
					None
				}

				fn succ(&self) -> Option<Self> {
					None
				}
			}
		)*
	};
}

impl_float!(f32, f64);
