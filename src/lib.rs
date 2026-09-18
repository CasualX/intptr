#![doc = include_str!("../readme.md")]
#![cfg_attr(not(test), no_std)]

mod ptr32;
mod ptr64;

pub use self::ptr32::*;
pub use self::ptr64::*;

#[cfg(target_pointer_width = "32")]
#[doc(no_inline)]
pub use IntPtr32 as IntPtr;

#[cfg(target_pointer_width = "64")]
#[doc(no_inline)]
pub use IntPtr64 as IntPtr;

impl<T: ?Sized> From<IntPtr32<T>> for IntPtr64<T> {
	#[inline]
	fn from(ptr: IntPtr32<T>) -> IntPtr64<T> {
		IntPtr64::from(ptr.into_raw() as u64)
	}
}

impl<T: ?Sized> TryFrom<IntPtr64<T>> for IntPtr32<T> {
	type Error = core::num::TryFromIntError;

	#[inline]
	fn try_from(ptr: IntPtr64<T>) -> Result<IntPtr32<T>, core::num::TryFromIntError> {
		u32::try_from(ptr.into_raw()).map(IntPtr32::from_raw)
	}
}

#[test]
fn pointer_width_conversions() {
	assert_eq!(IntPtr32::try_from(IntPtr64::<u8>::NULL), Ok(IntPtr32::<u8>::NULL));

	let ptr32 = IntPtr32::<u8>::from_raw(u32::MAX);
	assert_eq!(IntPtr64::from(ptr32), IntPtr64::<u8>::from_raw(u32::MAX as u64));
	assert_eq!(IntPtr32::try_from(IntPtr64::<u8>::from_raw(u32::MAX as u64)), Ok(ptr32));

	assert!(IntPtr32::try_from(IntPtr64::<u8>::from_raw(u32::MAX as u64 + 1)).is_err());
	assert!(IntPtr32::try_from(IntPtr64::<u8>::from_raw(u64::MAX)).is_err());
}
