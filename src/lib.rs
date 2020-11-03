/*!
IntPtr
======

This crate's purpose is to model 32-bit and 64-bit 'pointers' to memory outside of your address space.

Eg. when interacting with other processes' memory address space.

*/

#![cfg_attr(feature = "nightly", feature(structural_match))]
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

#[cfg(feature = "nightly")]
#[test]
fn test_match() {
	const TEST_PTR: IntPtr = IntPtr::from_raw(0x1000);
	match TEST_PTR {
		TEST_PTR => (),
		_ => panic!(),
	}
}

#[test]
fn raw_ptr() {
	fn c_api(_: *const ()) {}
	fn c_mut(_: *mut ()) {}

	let ptr = IntPtr::<i32>::from_usize(0x1000);
	c_api(ptr.cast().as_ptr());
	c_mut(ptr.cast().as_mut_ptr());
}
