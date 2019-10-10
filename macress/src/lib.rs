//! An idiomatic, easy to use, no-std compatible implementation of EUI-48 and EUI-64 addresses.

#![no_std]

use core::borrow::{Borrow, BorrowMut};
use core::fmt::{self as corefmt, Debug, Display, Formatter};
use core::str::FromStr;

mod internal;
pub mod fmt;

/// The transmission type of the packet.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Transmission {
    /// A multicast transmission
    Multicast,
    /// A unicast transmission
    Unicast
}

/// The administer type that assigned this MAC address
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Admin {
    /// The address is universally administered, meaning it's assigned to the device by it's manufacturer
    Universal,
    /// The address is locally administered, meaning it's assigned by the network admin
    Local
}

/// An error that occurs while parsing an address from a string
pub struct AddressParseError(());

/// The type used to represent a 6-octet MAC address value
pub type Eui48 = [u8; 6];
/// The type used to represent an 8-octet MAC address value
pub type Eui64 = [u8; 8];

/// An address value.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Address<T>(T);

impl<T> Address<T> {
    /// Creates a new address value using the specified inner value
    pub const fn new(inner: T) -> Self {
        Self(inner)
    }

    /// Gets the inner value of the address
    pub fn get(self) -> T {
        self.0
    }
}

impl<T> AsRef<T> for Address<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for Address<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> Borrow<T> for Address<T> {
    fn borrow(&self) -> &T {
        &self.0
    }
}

impl<T> BorrowMut<T> for Address<T> {
    fn borrow_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> From<T> for Address<T> {
    fn from(t: T) -> Address<T> {
        Address::new(t)
    }
}

impl<T: internal::Eui> Address<T> {
    /// Gets the kind of transmission this address is for. This can be a multicast transmission meant for multiple controllers 
    /// or a unicast transmission meant for one controller.
    pub fn transmission(&self) -> Transmission {
        self.0.transmission()
    }
    /// Gets the administer of this address. This can be a universal or local address, 
    /// assigned by the manufacturer or the network respectively.
    pub fn admin(&self) -> Admin {
        self.0.admin()
    }

    /// Returns whether this is a multicast address.
    pub fn is_multicast(&self) -> bool {
        self.transmission() == Transmission::Multicast
    }
    /// Returns whether this is a unicast address.
    pub fn is_unicast(&self) -> bool {
        self.transmission() == Transmission::Unicast
    }

    /// Returns whether this is a universal address assigned to the deviced by the manufacturer.
    pub fn is_universal(&self) -> bool {
        self.admin() == Admin::Universal
    }
    /// Returns whether this is a local address assigned to the deviced by the network.
    pub fn is_local(&self) -> bool {
        self.admin() == Admin::Local
    }

    /// Reverses the bits of this address in place. This is useful for transmitting addresses in bit-reversed order.
    pub fn reverse_bits(&mut self) {
        self.0.reverse_bits()
    }
}

impl Address<Eui48> {
    /// A MAC address with all zero bytes
    pub const ZERO: Self = Address::new([0x00; 6]);
    /// An address with all 255 bytes used to indicate that a packet is a broadcast that should be received by all network interfaces
    pub const BROADCAST: Self = Address::new([0xFF; 6]);

    pub fn to_interface(&self) -> Address<Eui64> {
        let arr = self.as_ref();
        Address::new([arr[0] ^ 0x02, arr[1], arr[2], 0xFF, 0xFE, arr[3], arr[4], arr[5]])
    }
}

impl Address<Eui64> {
    /// A MAC address with all zero bytes
    pub const ZERO: Self = Address::new([0x00; 8]);
    /// An address used to indicate that a packet is a broadcast that should be received by all network interfaces
    pub const BROADCAST: Self = Address::new([0xFF; 8]);
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Address<Eui48> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de>
    {
        deserializer.deserialize_str(internal::serde::AddressVisitor::<Eui48>::default())
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Address<Eui48> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer
    {
        serializer.collect_str(self)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Address<Eui64> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de>
    {
        deserializer.deserialize_str(internal::serde::AddressVisitor::<Eui64>::default())
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Address<Eui64> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer
    {
        serializer.collect_str(self)
    }
}

impl Default for Address<Eui48> {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Default for Address<Eui64> {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Debug for Address<Eui48> {
    fn fmt(&self, f: &mut Formatter) -> corefmt::Result {
        write!(f, "{:X}", fmt::Colon(self))
    }
}

impl Display for Address<Eui48> {
    fn fmt(&self, f: &mut Formatter) -> corefmt::Result {
        write!(f, "{:X}", fmt::Hyphen(self))
    }
}

impl Debug for Address<Eui64> {
    fn fmt(&self, f: &mut Formatter) -> corefmt::Result {
        write!(f, "{:X}", fmt::Colon(self))
    }
}

impl Display for Address<Eui64> {
    fn fmt(&self, f: &mut Formatter) -> corefmt::Result {
        write!(f, "{:X}", fmt::Hyphen(self))
    }
}

impl FromStr for Address<Eui48> {
    type Err = AddressParseError;

    fn from_str(s: &str) -> Result<Self, AddressParseError> {
        unimplemented!()
    }
}

impl FromStr for Address<Eui64> {
    type Err = AddressParseError;

    fn from_str(s: &str) -> Result<Self, AddressParseError> {
        unimplemented!()
    }
}