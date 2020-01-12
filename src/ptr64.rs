use core::{cmp, fmt, hash, mem, ops, str};
use core::marker::PhantomData;

#[inline(always)]
fn nibbles(word: u64) -> [u8; 16] {
	let b = word.to_be_bytes();
	[
		b[0] >> 4, b[0] & 0xf,
		b[1] >> 4, b[1] & 0xf,
		b[2] >> 4, b[2] & 0xf,
		b[3] >> 4, b[3] & 0xf,
		b[4] >> 4, b[4] & 0xf,
		b[5] >> 4, b[5] & 0xf,
		b[6] >> 4, b[6] & 0xf,
		b[7] >> 4, b[7] & 0xf,
	]
}
#[inline(always)]
fn digit(nibble: u8) -> u8 {
	if nibble < 10 { b'0' + nibble } else { b'a' - 10 + nibble }
}

/// Unmanaged 64-bit typed pointer.
#[repr(transparent)]
pub struct IntPtr64<T: ?Sized = ()> {
	address: u64,
	phantom_data: PhantomData<fn() -> T>,
}

impl<T: ?Sized> IntPtr64<T> {
	// Work around unstable const fn features
	const PHANTOM_DATA: PhantomData<fn() -> T> = PhantomData;

	/// Null pointer constant.
	pub const NULL: IntPtr64<T> = IntPtr64 { address: 0, phantom_data: PhantomData };
	/// Creates a null pointer.
	pub const fn new() -> IntPtr64<T> {
		IntPtr64::NULL
	}
	/// Constructs a pointer with an offset.
	pub const fn member(address: u64, offset: u32) -> IntPtr64<T> {
		let address = address + offset as u64;
		IntPtr64 { address, phantom_data: Self::PHANTOM_DATA }
	}
	/// Returns true if the pointer is null.
	pub const fn is_null(self) -> bool {
		self.address == 0
	}
	/// Casts the pointer to a different type keeping the pointer address fixed.
	pub const fn cast<U: ?Sized>(self) -> IntPtr64<U> {
		IntPtr64 { address: self.address, phantom_data: IntPtr64::<U>::PHANTOM_DATA }
	}
	/// Offsets the pointer to a field.
	pub const fn field<U: ?Sized>(self, offset: u32) -> IntPtr64<U> {
		let address = self.address + offset as u64;
		IntPtr64 { address, phantom_data: IntPtr64::<U>::PHANTOM_DATA }
	}
	/// Offsets the pointer and cast.
	pub const fn offset<U: ?Sized>(self, offset: i64) -> IntPtr64<U> {
		let address = self.address.wrapping_add(offset as u64);
		IntPtr64 { address, phantom_data: IntPtr64::<U>::PHANTOM_DATA }
	}
	/// Returns the raw integer, type ascription helper.
	pub const fn into_raw(self) -> u64 {
		self.address
	}
	/// Formats the pointer.
	pub fn fmt(self) -> [u8; 18] {
		let n = nibbles(self.address);
		[
			b'0', b'x',
			digit(n[0]),
			digit(n[1]),
			digit(n[2]),
			digit(n[3]),
			digit(n[4]),
			digit(n[5]),
			digit(n[6]),
			digit(n[7]),
			digit(n[8]),
			digit(n[9]),
			digit(n[10]),
			digit(n[11]),
			digit(n[12]),
			digit(n[13]),
			digit(n[14]),
			digit(n[15]),
		]
	}
}
impl<T> IntPtr64<[T]> {
	/// Decays the pointee from `[T]` to `T`.
	pub const fn decay(self) -> IntPtr64<T> {
		IntPtr64 { address: self.address, phantom_data: IntPtr64::<T>::PHANTOM_DATA }
	}
	/// Pointer arithmetic, gets the pointer of an element at the specified index.
	pub const fn at(self, i: usize) -> IntPtr64<T> {
		let address = self.address + (i * mem::size_of::<T>()) as u64;
		IntPtr64 { address, phantom_data: IntPtr64::<T>::PHANTOM_DATA }
	}
}

impl<T: ?Sized> Copy for IntPtr64<T> {}
impl<T: ?Sized> Clone for IntPtr64<T> {
	#[inline(always)]
	fn clone(&self) -> IntPtr64<T> {
		*self
	}
}
impl<T: ?Sized> Default for IntPtr64<T> {
	#[inline(always)]
	fn default() -> IntPtr64<T> {
		IntPtr64::NULL
	}
}
impl<T: ?Sized> Eq for IntPtr64<T> {}
impl<T: ?Sized> PartialEq for IntPtr64<T> {
	#[inline(always)]
	fn eq(&self, rhs: &IntPtr64<T>) -> bool {
		self.address == rhs.address
	}
}
impl<T: ?Sized> PartialOrd for IntPtr64<T> {
	#[inline(always)]
	fn partial_cmp(&self, rhs: &IntPtr64<T>) -> Option<cmp::Ordering> {
		self.address.partial_cmp(&rhs.address)
	}
}
impl<T: ?Sized> Ord for IntPtr64<T> {
	#[inline(always)]
	fn cmp(&self, rhs: &IntPtr64<T>) -> cmp::Ordering {
		self.address.cmp(&rhs.address)
	}
}
impl<T: ?Sized> hash::Hash for IntPtr64<T> {
	#[inline(always)]
	fn hash<H: hash::Hasher>(&self, state: &mut H) {
		self.address.hash(state)
	}
}
impl<T: ?Sized> AsRef<u64> for IntPtr64<T> {
	#[inline(always)]
	fn as_ref(&self) -> &u64 {
		&self.address
	}
}
impl<T: ?Sized> AsMut<u64> for IntPtr64<T> {
	#[inline(always)]
	fn as_mut(&mut self) -> &mut u64 {
		&mut self.address
	}
}

impl<T: ?Sized> From<u64> for IntPtr64<T> {
	#[inline(always)]
	fn from(address: u64) -> IntPtr64<T> {
		IntPtr64 { address, phantom_data: PhantomData }
	}
}
impl<T: ?Sized> From<IntPtr64<T>> for u64 {
	#[inline(always)]
	fn from(ptr: IntPtr64<T>) -> u64 {
		ptr.address
	}
}

impl<T> ops::Add<usize> for IntPtr64<T> {
	type Output = IntPtr64<T>;
	#[inline(always)]
	fn add(self, other: usize) -> IntPtr64<T> {
		let address = self.address + (other * mem::size_of::<T>()) as u64;
		IntPtr64 { address, phantom_data: self.phantom_data }
	}
}
impl<T> ops::Sub<usize> for IntPtr64<T> {
	type Output = IntPtr64<T>;
	#[inline(always)]
	fn sub(self, other: usize) -> IntPtr64<T> {
		let address = self.address - (other * mem::size_of::<T>()) as u64;
		IntPtr64 { address, phantom_data: self.phantom_data }
	}
}

impl<T: ?Sized> fmt::Debug for IntPtr64<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let buf = IntPtr64::fmt(*self);
		f.pad(unsafe { str::from_utf8_unchecked(&buf) })
	}
}
impl<T: ?Sized> fmt::UpperHex for IntPtr64<T> {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.address.fmt(f)
	}
}
impl<T: ?Sized> fmt::LowerHex for IntPtr64<T> {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.address.fmt(f)
	}
}
impl<T: ?Sized> fmt::Display for IntPtr64<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let buf = IntPtr64::fmt(*self);
		f.pad(unsafe { str::from_utf8_unchecked(&buf) })
	}
}

#[cfg(feature = "dataview")]
unsafe impl<T: ?Sized + 'static> dataview::Pod for IntPtr64<T> {}

#[cfg(feature = "serde")]
impl<T: ?Sized> serde::Serialize for IntPtr64<T> {
	#[inline(always)]
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		serializer.serialize_u64(self.address)
	}
}

#[test]
fn units() {
	let a = IntPtr64::<f64>::from(0x2000);
	let b = a + 0x40;
	let c = a - 0x40;
	assert_eq!(mem::size_of_val(&a), 8);
	assert_eq!(b.into_raw(), 0x2200);
	assert_eq!(format!("{}", a), "0x0000000000002000");
	assert_eq!(c.into_raw(), 0x1E00);
}
