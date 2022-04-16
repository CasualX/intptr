use core::{cmp, fmt, hash, marker, mem, ops, str};

#[inline]
const fn nibbles(word: u32) -> [u8; 8] {
	let b = word.to_le_bytes();
	[
		b[3] >> 4, b[3] & 0xf,
		b[2] >> 4, b[2] & 0xf,
		b[1] >> 4, b[1] & 0xf,
		b[0] >> 4, b[0] & 0xf,
	]
}
#[inline]
const fn digit(nibble: u8) -> u8 {
	if nibble < 10 { b'0' + nibble } else { b'a' + (nibble - 10) }
}

/// Unmanaged 32-bit typed pointer.
#[repr(transparent)]
pub struct IntPtr32<T: ?Sized = ()> {
	address: u32,
	phantom_data: marker::PhantomData<fn() -> T>,
}

impl<T: ?Sized> IntPtr32<T> {
	// Work around unstable const fn features
	const PHANTOM_DATA: marker::PhantomData<fn() -> T> = marker::PhantomData;
	/// Null pointer constant.
	pub const NULL: IntPtr32<T> = IntPtr32 { address: 0, phantom_data: marker::PhantomData };
	/// Creates a null pointer.
	#[inline]
	pub const fn new() -> IntPtr32<T> {
		IntPtr32::NULL
	}
	/// Creates a pointer from its address.
	#[inline]
	pub const fn from_raw(address: u32) -> IntPtr32<T> {
		IntPtr32 { address, phantom_data: Self::PHANTOM_DATA }
	}
	/// Constructs a pointer with an offset.
	#[inline]
	pub const fn member(address: u32, offset: u32) -> IntPtr32<T> {
		let address = address + offset;
		IntPtr32 { address, phantom_data: Self::PHANTOM_DATA }
	}
	/// Returns true if the pointer is null.
	#[inline]
	pub const fn is_null(self) -> bool {
		self.address == 0
	}
	/// Casts the pointer to a different type keeping the pointer address fixed.
	#[inline]
	pub const fn cast<U: ?Sized>(self) -> IntPtr32<U> {
		IntPtr32 { address: self.address, phantom_data: IntPtr32::<U>::PHANTOM_DATA }
	}
	/// Offsets the pointer to a field.
	#[inline]
	pub const fn field<U: ?Sized>(self, offset: u32) -> IntPtr32<U> {
		let address = self.address + offset;
		IntPtr32 { address, phantom_data: IntPtr32::<U>::PHANTOM_DATA }
	}
	/// Offsets the pointer and cast.
	#[inline]
	pub const fn offset<U: ?Sized>(self, offset: i32) -> IntPtr32<U> {
		let address = self.address.wrapping_add(offset as u32);
		IntPtr32 { address, phantom_data: IntPtr32::<U>::PHANTOM_DATA }
	}
	/// Returns the raw integer, type ascription helper.
	#[inline]
	pub const fn into_raw(self) -> u32 {
		self.address
	}
	/// Formats the pointer.
	#[inline]
	pub const fn fmt(self) -> [u8; 10] {
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
		]
	}
}
#[cfg(target_pointer_width = "32")]
impl<T: ?Sized> IntPtr32<T> {
	/// Creates a pointer from `usize`.
	#[inline]
	pub const fn from_usize(address: usize) -> IntPtr32<T> {
		Self::from_raw(address as u32)
	}
	/// Returns the address as `usize`.
	#[inline]
	pub const fn into_usize(self) -> usize {
		self.address as usize
	}
}
#[cfg(all(feature = "int2ptr", target_pointer_width = "32"))]
impl<T> IntPtr32<T> {
	#[inline]
	pub fn from_ptr(ptr: *const T) -> IntPtr32<T> {
		Self::from_raw(ptr as usize as u32)
	}
	#[inline]
	pub fn from_mut_ptr(ptr: *mut T) -> IntPtr32<T> {
		Self::from_raw(ptr as usize as u32)
	}
	#[inline]
	pub const fn as_ptr(self) -> *const T {
		self.address as *const T
	}
	#[inline]
	pub const fn as_mut_ptr(self) -> *mut T {
		self.address as *mut T
	}
}
impl<T> IntPtr32<[T]> {
	/// Decays the pointee from `[T]` to `T`.
	#[inline]
	pub const fn decay(self) -> IntPtr32<T> {
		IntPtr32 { address: self.address, phantom_data: IntPtr32::<T>::PHANTOM_DATA }
	}
	/// Pointer arithmetic, gets the pointer of an element at the specified index.
	#[inline]
	pub const fn at(self, i: usize) -> IntPtr32<T> {
		let address = self.address + (i * mem::size_of::<T>()) as u32;
		IntPtr32 { address, phantom_data: IntPtr32::<T>::PHANTOM_DATA }
	}
}
impl<T, const N: usize> IntPtr32<[T; N]> {
	/// Decays the pointee from `[T; N]` to `T`.
	#[inline]
	pub const fn decay(self) -> IntPtr32<T> {
		IntPtr32 { address: self.address, phantom_data: IntPtr32::<T>::PHANTOM_DATA }
	}
	/// Pointer arithmetic, gets the pointer of an element at the specified index.
	#[inline]
	pub const fn at(self, i: usize) -> IntPtr32<T> {
		let address = self.address + (i * mem::size_of::<T>()) as u32;
		IntPtr32 { address, phantom_data: IntPtr32::<T>::PHANTOM_DATA }
	}
}

#[cfg(feature = "nightly")]
impl<T: ?Sized> marker::StructuralEq for IntPtr32<T> {}
#[cfg(feature = "nightly")]
impl<T: ?Sized> marker::StructuralPartialEq for IntPtr32<T> {}

impl<T: ?Sized> Copy for IntPtr32<T> {}
impl<T: ?Sized> Clone for IntPtr32<T> {
	#[inline]
	fn clone(&self) -> IntPtr32<T> {
		*self
	}
}
impl<T: ?Sized> Default for IntPtr32<T> {
	#[inline]
	fn default() -> IntPtr32<T> {
		IntPtr32::NULL
	}
}
impl<T: ?Sized> Eq for IntPtr32<T> {}
impl<T: ?Sized> PartialEq for IntPtr32<T> {
	#[inline]
	fn eq(&self, rhs: &IntPtr32<T>) -> bool {
		self.address == rhs.address
	}
}
impl<T: ?Sized> PartialOrd for IntPtr32<T> {
	#[inline]
	fn partial_cmp(&self, rhs: &IntPtr32<T>) -> Option<cmp::Ordering> {
		self.address.partial_cmp(&rhs.address)
	}
}
impl<T: ?Sized> Ord for IntPtr32<T> {
	#[inline]
	fn cmp(&self, rhs: &IntPtr32<T>) -> cmp::Ordering {
		self.address.cmp(&rhs.address)
	}
}
impl<T: ?Sized> hash::Hash for IntPtr32<T> {
	#[inline]
	fn hash<H: hash::Hasher>(&self, state: &mut H) {
		self.address.hash(state)
	}
}
impl<T: ?Sized> AsRef<u32> for IntPtr32<T> {
	#[inline]
	fn as_ref(&self) -> &u32 {
		&self.address
	}
}
impl<T: ?Sized> AsMut<u32> for IntPtr32<T> {
	#[inline]
	fn as_mut(&mut self) -> &mut u32 {
		&mut self.address
	}
}

impl<T: ?Sized> From<u32> for IntPtr32<T> {
	#[inline]
	fn from(address: u32) -> IntPtr32<T> {
		IntPtr32 { address, phantom_data: marker::PhantomData }
	}
}
impl<T: ?Sized> From<IntPtr32<T>> for u32 {
	#[inline]
	fn from(ptr: IntPtr32<T>) -> u32 {
		ptr.address
	}
}

impl<T> ops::Add<usize> for IntPtr32<T> {
	type Output = IntPtr32<T>;
	#[inline]
	fn add(self, other: usize) -> IntPtr32<T> {
		let address = self.address + (other * mem::size_of::<T>()) as u32;
		IntPtr32 { address, phantom_data: self.phantom_data }
	}
}
impl<T> ops::Sub<usize> for IntPtr32<T> {
	type Output = IntPtr32<T>;
	#[inline]
	fn sub(self, other: usize) -> IntPtr32<T> {
		let address = self.address - (other * mem::size_of::<T>()) as u32;
		IntPtr32 { address, phantom_data: self.phantom_data }
	}
}

impl<T: ?Sized> fmt::Debug for IntPtr32<T> {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.is_null() {
			f.pad("0x0")
		}
		else {
			let buf = IntPtr32::fmt(*self);
			f.pad(unsafe { str::from_utf8_unchecked(&buf) })
		}
	}
}
impl<T: ?Sized> fmt::UpperHex for IntPtr32<T> {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.address.fmt(f)
	}
}
impl<T: ?Sized> fmt::LowerHex for IntPtr32<T> {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.address.fmt(f)
	}
}
impl<T: ?Sized> fmt::Display for IntPtr32<T> {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.is_null() {
			f.pad("0x0")
		}
		else {
			let buf = IntPtr32::fmt(*self);
			f.pad(unsafe { str::from_utf8_unchecked(&buf) })
		}
	}
}

#[cfg(feature = "dataview")]
unsafe impl<T: ?Sized + 'static> dataview::Pod for IntPtr32<T> {}

#[cfg(feature = "serde")]
impl<T: ?Sized> serde::Serialize for IntPtr32<T> {
	#[inline]
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		serializer.serialize_u32(self.address)
	}
}

#[test]
fn units() {
	let a = IntPtr32::<f32>::from(0x2000);
	let b = a + 0x40;
	let c = a - 0x40;
	assert_eq!(mem::size_of_val(&a), 4);
	assert_eq!(b.into_raw(), 0x2100);
	assert_eq!(format!("{}", a), "0x00002000");
	assert_eq!(format!("{}", IntPtr32::<()>::NULL), "0x0");
	assert_eq!(c.into_raw(), 0x1F00);
	assert_eq!(IntPtr32::<[u32]>::from_raw(0x1000).at(1), IntPtr32::<u32>::from_raw(0x1004));
	assert_eq!(IntPtr32::<[u32; 2]>::from_raw(0x1000).at(1), IntPtr32::<u32>::from_raw(0x1004));
}
