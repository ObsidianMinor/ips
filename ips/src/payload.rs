//! Types for managing and representing byte payloads

use crate::physical::Size;

use core::ops::Deref;

/// A structure used to indicate that the structure contains no payload
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct Empty;

impl Size for Empty {
    fn size(&self) -> usize { 0 }
}

/// A structure used to signal that it's undetermined where the start or end of the payload is. This may have padded data, unparsed header data, or any other data.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Unknown<'a>(pub &'a [u8]);

impl<'a> Unknown<'a> {
    /// Advances the buffer by the specified length.
    pub fn consume(self, amnt: usize) -> Self {
        if amnt > self.len() {
            panic!("attempted to advance past the end of the buffer")
        } else {
            unsafe { self.consume_unchecked(amnt) }
        }
    }

    pub unsafe fn consume_unchecked(self, amnt: usize) -> Self {
        Unknown(self.0.get_unchecked(..amnt))
    }

    /// Converts this [`Unknown`] payload into a [`Padded`] payload with [`Any`] unparsed value where the payload is of the specified length, or
    /// [`None`] if the length extends past the end of the payload.
    /// 
    /// [`Unknown`]: struct.Unknown.html
    /// [`Padded`]: struct.Padded.html
    /// [`Any`]: struct.Any.html
    /// [`None`]: https://doc.rust-lang.org/core/option/enum.Option.html#variant.None
    pub fn try_as_padded_any(self, length: usize) -> Option<Padded<&'a [u8], Any<'a>>> {
        if length > self.0.len() {
            None
        } else {
            unsafe { Some(self.as_padded_any_unchecked(length)) }
        }
    }
    /// Converts this [`Unknown`] payload into a [`Padded`] payload with [`Any`] unparsed value where the payload is of the specified length.
    /// 
    /// [`Unknown`]: struct.Unknown.html
    /// [`Padded`]: struct.Padded.html
    /// [`Any`]: struct.Any.html
    /// 
    /// # Panics
    /// 
    /// This function will panic if `length > len`
    pub fn as_padded_any(self, length: usize) -> Padded<&'a [u8], Any<'a>> {
        let (bytes, padding) = self.0.split_at(length);
        Padded {
            payload: Any(bytes),
            padding
        }
    }
    /// Converts this [`Unknown`] payload into a [`Padded`] payload with [`Any`] unparsed value where the payload is of the specified length without performing length checks.
    /// 
    /// [`Unknown`]: struct.Unknown.html
    /// [`Padded`]: struct.Padded.html
    /// [`Any`]: struct.Any.html
    /// 
    /// # Safety
    /// 
    /// This will cause undefined behavior if `length > len`
    pub unsafe fn as_padded_any_unchecked(self, length: usize) -> Padded<&'a [u8], Any<'a>> {
        let (bytes, padding) = (self.0.get_unchecked(..length), self.0.get_unchecked(length..));
        Padded {
            payload: Any(bytes),
            padding
        }
    }
}

impl<'a> From<Any<'a>> for Unknown<'a> {
    fn from(a: Any<'a>) -> Unknown<'a> {
        Unknown(a.0)
    }
}
impl<'a> From<&'a [u8]> for Unknown<'a> {
    fn from(s: &'a [u8]) -> Unknown<'a> {
        Unknown(s)
    }
}
impl<'a> Deref for Unknown<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

/// A structure used to contain an unparsed payload value
pub struct Any<'a>(pub &'a [u8]);

impl<'a> From<Unknown<'a>> for Any<'a> {
    fn from(u: Unknown<'a>) -> Any<'a> {
        Any(u.0)
    }
}
impl Size for Any<'_> {
    fn size(&self) -> usize { self.0.len() }
}

/// A padding value that can be written to an output
pub struct ValuePadding<T> {
    pub value: T,
    pub length: usize
}

impl ValuePadding<u8> {
    pub fn zero(length: usize) -> ValuePadding<u8> {
        ValuePadding { value: 0, length }
    }
}
impl<T: Size> Size for ValuePadding<T> {
    fn size(&self) -> usize {
        self.value.size() * self.length
    }
}

/// Represents a possibly padded value
pub struct Padded<P, T> {
    pub payload: T,
    pub padding: P
}