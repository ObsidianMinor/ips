//! An Ethernet frame types module

pub mod vlan;

use crate::payload;
use crate::physical;

use core::convert::TryFrom;
use macress::{Address, Eui48};

type MacAddr = Address<Eui48>;

/// The minimum length of a ethernet payload
pub const MIN_PAYLOAD_LEN: usize = 46;
/// The maximum length of a ethernet frame payload
pub const MAX_PAYLOAD_LEN: usize = 1500;
/// The maximum length of a jumbo ethernet frame payload
pub const MAX_JUMBO_PAYLOAD_LEN: usize = 9000;

pub struct InvalidLengthError(());

/// A base ethernet packet that can be used to differentiate
/// between ethernet frames with a length and frames with an ethertype
#[derive(Clone, Debug)]
pub struct EthernetBase<V, P> {
    /// The destination MAC address for the packet
    pub destination: MacAddr,
    /// The source MAC address for the packet
    pub source: MacAddr,
    /// The VLAN extension for this ethernet frame
    pub vlan: V,
    /// The type or length of the packet
    pub type_or_length: u16,
    payload: P
}

impl<V, P> EthernetBase<V, P> {
    /// Returns the [`type_or_length`](#structfield.type_or_length) field as a payload length
    pub fn length(&self) -> u16 {
        self.type_or_length
    }
    /// Returns the [`type_or_length`](#structfield.type_or_length) field as payload ethertype
    pub fn ethertype(&self) -> EtherType {
        EtherType(self.type_or_length)
    }
    /// Creates a VLAN header from the payload and the old header.
    pub fn map_vlan<W, Q, F: FnOnce(u16, V, P) -> (u16, W, Q)>(self, f: F) -> EthernetBase<W, Q> {
        let EthernetBase {
            destination,
            source,
            vlan,
            type_or_length,
            payload,
        } = self;

        let (type_or_length, vlan, payload) = f(type_or_length, vlan, payload);

        EthernetBase { destination, source, type_or_length, vlan, payload }
    }
    /// Creates a VLAN header from the payload and the old header, where the conversion function may fail with the specified error type.
    pub fn try_map_vlan<W, Q, E, F: FnOnce(u16, V, P) -> Result<(u16, W, Q), E>>(self, f: F) -> Result<EthernetBase<W, Q>, E> {
        let EthernetBase {
            destination,
            source,
            vlan,
            type_or_length,
            payload,
        } = self;

        let (type_or_length, vlan, payload) = f(type_or_length, vlan, payload)?;

        Ok(EthernetBase { destination, source, type_or_length, vlan, payload })
    }
    /// Creates a new VLAN header to a new type without using the payload to map the header.
    pub fn map_vlan_no_payload<W, F: FnOnce(u16, V) -> (u16, W)>(self, f: F) -> EthernetBase<W, P> {
        let EthernetBase {
            destination,
            source,
            vlan,
            type_or_length,
            payload,
        } = self;

        let (type_or_length, vlan) = f(type_or_length, vlan);

        EthernetBase { destination, source, type_or_length, vlan, payload }
    }
    /// Creates a new VLAN header to a new type without using the payload to map the header, where the conversion function may fail with the specified error type.
    pub fn try_map_vlan_no_payload<W, E, F: FnOnce(u16, V) -> Result<(u16, W), E>>(self, f: F) -> Result<EthernetBase<W, P>, E> {
        let EthernetBase {
            destination,
            source,
            vlan,
            type_or_length,
            payload,
        } = self;

        let (type_or_length, vlan) = f(type_or_length, vlan)?;

        Ok(EthernetBase { destination, source, type_or_length, vlan, payload })
    }
}

impl<'a> EthernetBase<vlan::Unknown, payload::Unknown<'a>> {
    /// Parses a base ethernet packet from a slice of bytes, returning an error of the data is less than 14 bytes.
    pub fn parse<P: Into<payload::Unknown<'a>>>(payload: P) -> Result<Self, physical::Error> {
        let bytes = payload.into();
        if bytes.len() < 14 {
            Err(physical::Error)
        } else {
            unsafe { Ok(Self::consume_unknown(bytes)) }
        }
    }

    /// Reads a base ethernet packet from a slice of bytes.
    /// 
    /// # Safety
    /// 
    /// The buffer must be at least 14 bytes (the minimum length of a ethernet header).
    /// Using this function with a slice where `len < 14` is undefined behavior.
    pub unsafe fn parse_unchecked<P: Into<payload::Unknown<'a>>>(payload: P) -> Self {
        Self::consume_unknown(payload.into())
    }

    unsafe fn consume_unknown(bytes: payload::Unknown<'a>) -> Self {
        let dest = MacAddr::new(<[u8; 6]>::try_from(bytes.get_unchecked(0..6)).unwrap());
        let src = MacAddr::new(<[u8; 6]>::try_from(bytes.get_unchecked(6..12)).unwrap());
        let type_or_length = u16::from_be_bytes(<[u8; 2]>::try_from(bytes.get_unchecked(12..14)).unwrap());

        Self {
            destination: dest,
            source: src,
            vlan: vlan::Unknown(()),
            type_or_length,
            payload: bytes.consume(14)
        }
    }
}

impl<'a, V: vlan::Extension> EthernetBase<V, payload::Unknown<'a>> {
    /// Consumes this base packet, turning it into an Ethernet packet.
    /// This interprets the type or length field as a length, making the payload any padded payload value.
    pub fn unwrap_ethernet(self) -> Ethernet<V, payload::Padded<&'a [u8], payload::Any<'a>>> {
        let EthernetBase { destination, source, vlan, type_or_length, payload } = self;
        Ethernet {
            destination,
            source,
            vlan,
            payload: payload.as_padded_any(type_or_length as usize)
        }
    }

    /// Consumes this base packet, turning it into an Ethernet packet.
    /// This interprets the type or length field as a length, making the payload any padded payload value.
    /// 
    /// If the length is not in bounds, this returns an InvalidLengthError.
    pub fn try_unrwap_ethernet(self) -> Result<Ethernet<V, payload::Padded<&'a [u8], payload::Any<'a>>>, InvalidLengthError> {
        let EthernetBase { destination, source, vlan, type_or_length, payload } = self;
        Ok(Ethernet {
            destination,
            source,
            vlan,
            payload: payload.try_as_padded_any(type_or_length as usize).ok_or(InvalidLengthError(()))?
        })
    }

    /// Consumes this base packet, turning it into an Ethernet2 packet.
    /// This interprets the type or length field as a ethertype, leaving the payload in-place as unknown.
    pub fn unwrap_ethernet2(self) -> Ethernet2<V, payload::Unknown<'a>> {
        let EthernetBase { destination, source, vlan, type_or_length, payload } = self;
        Ethernet2 {
            destination,
            source,
            vlan,
            etype: EtherType(type_or_length),
            payload
        }
    }
}

/// An ethernet frame with a payload length field.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Ethernet<V, P> {
    /// The destination MAC address for the packet
    pub destination: MacAddr,
    /// The source MAC address for the packet
    pub source: MacAddr,
    /// The VLAN extension for this ethernet frame
    pub vlan: V,
    payload: P,
}

/// A double octet EtherType value
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct EtherType(pub u16);

impl EtherType {
    /// The ethertype used for IPv4 protocol payloads
    pub const IPV4: EtherType = EtherType(0x8000);
    /// An ethertype used to signal that this ethernet frame is using a single VLAN extension field.
    pub const DOT1Q: EtherType = EtherType(0x8100);
    /// An ethertype used to signal that this ethernet frame is using a stacked VLAN extension field.
    pub const QINQ: EtherType = EtherType(0x88a8);
}

/// An ethernet frame with a payload ethertype field.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Ethernet2<V, P> {
    /// The destination MAC address for this ethernet frame
    pub destination: MacAddr,
    /// The source MAC address for this ethernet frame
    pub source: MacAddr,
    /// A VLAN extension for this ethernet frame
    pub vlan: V,
    /// A double octet in the ethertype position.
    /// For instances with an UnknownVlan header, this may be the the start of the VLAN header.
    pub etype: EtherType,
    payload: P
}