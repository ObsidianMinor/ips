//! Contains types for VLAN ethernet header extensions

use crate::link::ethernet::EtherType;
use crate::payload;
use crate::physical::Size;

use core::convert::TryFrom;

/// Defines a VLAN extension extension value.
/// This may be no header, a standard vlan header, a stacked header, or any unparsed header value.
pub trait Extension { }

/// A VLAN extension header containing a tag
pub trait Header: Extension {
    /// Gets the header tag
    fn tag(&self) -> &Tag;
}

/// No extension header
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct Empty;

impl Extension for Empty { }
impl Size for Empty {
    fn size(&self) -> usize { 0 }
}

/// Represents the priority level of an ethernet packet.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PriorityLevel {
    Background = 0,
    BestEffort = 1,
    ExcellentEffort = 2,
    CriticalApplications = 3,
    Video = 4,
    Voice = 5,
    InternetworkControl = 6,
    NetworkControl = 7,
}

impl Default for PriorityLevel {
    fn default() -> Self {
        PriorityLevel::BestEffort
    }
}

/// A VLAN extension tag
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Tag(u16);

impl Extension for Tag { }
impl Header for Tag {
    fn tag(&self) -> &Tag { self }
}
impl Size for Tag {
    fn size(&self) -> usize { 4 }
}

impl Tag {
    const PCP_MASK: u16 = 0xE000;
    const PCP_SHIFT: u32 = 13;

    const DEI_MASK: u16 = 0x1000;
    const DEI_SHIFT: u32 = 12;

    const ID_MASK: u16 = 0x0FFF;

    /// Creates a new VLAN extension tag using the specified ID, priority level, and drop eligibility
    pub const fn new(id: Identifier, priority: PriorityLevel, can_drop: bool) -> Tag {
        Tag(id.get() | ((can_drop as u16) << Self::DEI_SHIFT) | ((priority as u16) << Self::PCP_SHIFT))
    }

    /// Returns a new tag from the specified raw value
    pub const fn raw(value: u16) -> Tag {
        Tag(value)
    }

    /// Gets the priority level of this packet on the VLAN.
    #[inline]
    pub fn priority(self) -> PriorityLevel {
        match (self.0 & Self::PCP_MASK) >> Self::PCP_SHIFT {
            0 => PriorityLevel::Background,
            1 => PriorityLevel::BestEffort,
            2 => PriorityLevel::ExcellentEffort,
            3 => PriorityLevel::CriticalApplications,
            4 => PriorityLevel::Video,
            5 => PriorityLevel::Voice,
            6 => PriorityLevel::InternetworkControl,
            7 => PriorityLevel::NetworkControl,
            _ => unreachable!()
        }
    }

    /// Gets whether this packet is eligible for being dropped if the network is congested.
    pub const fn drop_eligible(self) -> bool {
        (self.0 & Self::DEI_MASK) != 0
    }

    /// Gets the 12-bit VLAN identifier for this tag
    pub const fn identifier(self) -> Identifier {
        Identifier(self.0 & Self::ID_MASK)
    }

    /// Gets the raw underlying value of this Tag
    pub const fn get(self) -> u16 {
        self.0
    }
}

/// A 12-bit VLAN extension identifier
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Identifier(u16);

impl Identifier {
    /// A reserved value used to indicate that no VLAN identifier is present
    pub const NONE: Identifier = Identifier(0x000);
    /// The reserved VLAN identifier
    pub const RESERVED: Identifier = Identifier(0xFFF);

    /// Creates a new identifier instance from the specified value. If the value is out of range, this returns None.
    pub fn new(v: u16) -> Option<Identifier> {
        if v > 0xFFF {
            None
        } else {
            Some(Identifier(v))
        }
    }
    /// Creates a new identifier value without checking that it's in the 12-bit value range
    pub const unsafe fn new_unchecked(v: u16) -> Identifier {
        Identifier(v)
    }
    /// Gets the underlying value for this identifier
    pub const fn get(self) -> u16 {
        self.0
    }
}

/// A stacked VLAN tag
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Stacked<V> {
    pub tag: Tag,
    pub remainder: V
}

impl<V: Extension> Stacked<V> {
    pub fn map_remainder<T, F: FnOnce(V) -> T>(self, f: F) -> Stacked<T> {
        let Stacked { tag, remainder } = self;
        let remainder = f(remainder);
        Stacked { tag, remainder }
    }
}

impl<V: Extension> Extension for Stacked<V> { }
impl<V: Extension> Header for Stacked<V> {
    fn tag(&self) -> &Tag {
        &self.tag
    }
}
impl<V: Size> Size for Stacked<V> {
    fn size(&self) -> usize {
        self.tag.size() + self.remainder.size()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct AnyHeader<'a> {
    first: EtherType,
    data: payload::Unknown<'a>
}

impl<'a> AnyHeader<'a> {
    fn parse(ethertype: EtherType, payload: payload::Unknown<'a>) -> (u16, Self, payload::Unknown<'a>) {
        let mut etypes_iter = 
            payload
                .chunks(2) // split the payload into 2 value chunks
                .skip(1) // skip this chunk (since it's the tag of the first)
                .step_by(2) // skip every other chunk, we're just looking at the ethertypes
                .map(|c| <[u8; 2]>::try_from(c).map(|slice| EtherType(u16::from_be_bytes(slice)))) // turn each slice into an ethertype
                .take_while(|result| result.map(|tp| VLAN_EXTENSIONS.contains(&tp)).unwrap_or(false)); // take ethertypes while they're VLAN extensions

        let qinq_headers = // count all QinQ headers
            etypes_iter
                .by_ref()
                .take_while(|result| result.map(|tp| tp == EtherType::QINQ).unwrap_or(false))
                .count();

        // make sure the next one after QinQ is the single header
        let trailing_headers = 
            if qinq_headers != 0 {
                match etypes_iter.next() {
                    Some(Ok(EtherType::DOT1Q)) => { },
                    _ => panic!("bad VLAN extension; expected DOT1Q after all QinQ headers"),
                }

                qinq_headers + 1
            } else {
                0
            };

        let headers = trailing_headers + 1; // count the first one we were given

        let last = 
            match etypes_iter.next() {
                Some(Ok(EtherType::QINQ)) | Some(Ok(EtherType::DOT1Q)) => {
                    panic!("bad VLAN extension; expected ethertype or length after VLAN DOT1Q header")
                },
                Some(Ok(other)) => other,
                _ => panic!("bad ethernet payload; expected ethertype or length after VLAN headers, but ran out of data") // we never got our final ethertype or length
            };

        let read_len = (headers * 4) - 2;

        (last.0, AnyHeader { first: ethertype, data: payload::Unknown(&payload.0[..read_len]) }, payload.consume(read_len))
    }
    /// Gets the EtherType of the first VLAN extension header. This is the header furthest to the left in an ethernet header
    pub fn first(&self) -> EtherType {
        self.first
    }
}

/// Any unparsed VLAN header value.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Any<'a> {
    Some(AnyHeader<'a>),
    None
}

const VLAN_EXTENSIONS: [EtherType; 2] = [EtherType::QINQ, EtherType::DOT1Q];

impl<'a> Any<'a> {
    /// Parses any VLAN extension, returning the new ethertype or length, the value, and a new unknown payload that starts after the new type or value field.
    pub fn parse<P: Into<payload::Unknown<'a>>>(ethertype: EtherType, payload: P) -> (u16, Self, payload::Unknown<'a>) {
        let payload = payload.into();
        match ethertype {
            EtherType::DOT1Q | EtherType::QINQ => {
                let (last, hdr, pld) = AnyHeader::parse(ethertype, payload);
                (last, Any::Some(hdr), pld)
            },
            _ => (ethertype.0, Any::None, payload)
        }
    }

    /// Consumes the value, returning a new Stacked tag where the remainder is the rest of the Any value.
    pub fn unwrap_stack(self) -> Stacked<Any<'a>> {
        match self {
            Any::Some(AnyHeader { first: EtherType::QINQ, data: payload::Unknown(data) }) => {
                if data.len() < 4 {
                    panic!("expected at least 4 bytes of stacked VLAN header data")
                }
                unsafe {
                    let tag = Tag::raw(u16::from_be_bytes(<[u8; 2]>::try_from(data.get_unchecked(0..2)).unwrap()));
                    let next_type = EtherType(u16::from_be_bytes(<[u8; 2]>::try_from(data.get_unchecked(2..4)).unwrap()));
                    let remainder = data.get_unchecked(4..);

                    Stacked {
                        tag,
                        remainder: Any::Some(AnyHeader {
                            first: next_type,
                            data: payload::Unknown(remainder)
                        })
                    }
                }
            },
            _ => panic!("expected stacked VLAN header")
        }
    }

    /// Consumes the value, returning a new single tag
    pub fn unwrap_tag(self) -> Tag {
        match self {
            Any::Some(AnyHeader { first: EtherType::DOT1Q, data: payload::Unknown(data) }) => {
                if data.len() != 2 {
                    panic!("expected exactly 2 bytes of VLAN header data")
                }

                Tag::raw(u16::from_be_bytes(<[u8; 2]>::try_from(data).unwrap()))
            },
            _ => panic!("expected VLAN tag")
        }
    }

    /// Consumes the value, returning an empty header value.
    /// 
    /// This will panic if a header exists. If you want to remove a VLAN header, drop it and return an Empty header.
    pub fn unwrap_empty(self) -> Empty {
        match self {
            Any::Some(_) => panic!("unexpected vlan header"),
            Any::None => Empty
        }
    }
}

impl Extension for Any<'_> { }
impl Size for Any<'_> {
    fn size(&self) -> usize {
        match self {
            Any::Some(AnyHeader { data, .. }) => data.len() + 2,
            Any::None => 0
        }
    }
}

/// A type used to indicate if it's unknown whether an ethernet frame contains a VLAN header.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Unknown(pub(super) ());