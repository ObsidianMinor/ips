use super::{Address, Eui48, Eui64};

use core::fmt::{self, Formatter, UpperHex, LowerHex, Debug, Display};

/// A helper type for formatting address values using hypen seperators in the format "01-23-45-67-89-AB".
pub struct Hyphen<'a, T: 'a>(pub &'a T);

impl<'a> UpperHex for Hyphen<'a, Address<Eui48>> {
    /// Formats the address using hyphens and upper case hex values.
    /// 
    /// The alternate flag '#' has no effect on this implementation.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let value = self.0.as_ref();
        write!(f, "{:X}-{:X}-{:X}-{:X}-{:X}-{:X}", value[0], value[1], value[2], value[3], value[4], value[5])
    }
}

impl<'a> UpperHex for Hyphen<'a, Address<Eui64>> {
    /// Formats the address using hyphens and upper case hex values.
    /// 
    /// The alternate flag '#' has no effect on this implementation.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let value = self.0.as_ref();
        write!(f, "{:X}-{:X}-{:X}-{:X}-{:X}-{:X}-{:X}-{:X}", value[0], value[1], value[2], value[3], value[4], value[5], value[6], value[7])
    }
}

impl<'a> LowerHex for Hyphen<'a, Address<Eui48>> {
    /// Formats the address using hyphens and lower case hex values.
    /// 
    /// The alternate flag '#' has no effect on this implementation.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let value = self.0.as_ref();
        write!(f, "{:x}-{:x}-{:x}-{:x}-{:x}-{:x}", value[0], value[1], value[2], value[3], value[4], value[5])
    }
}

impl<'a> LowerHex for Hyphen<'a, Address<Eui64>> {
    /// Formats the address using hyphens and lower case hex values.
    /// 
    /// The alternate flag '#' has no effect on this implementation.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let value = self.0.as_ref();
        write!(f, "{:x}-{:x}-{:x}-{:x}-{:x}-{:x}-{:x}-{:x}", value[0], value[1], value[2], value[3], value[4], value[5], value[6], value[7])
    }
}

impl<'a> Debug for Hyphen<'a, Address<Eui48>> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:X}", self)
    }
}

impl<'a> Debug for Hyphen<'a, Address<Eui64>> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:X}", self)
    }
}

impl<'a> Display for Hyphen<'a, Address<Eui48>> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:X}", self)
    }
}

impl<'a> Display for Hyphen<'a, Address<Eui64>> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:X}", self)
    }
}

/// A helper type for formatting address values using colon seperators in the format "01:23:45:67:89:AB".
pub struct Colon<'a, T: 'a>(pub &'a T);

impl<'a> UpperHex for Colon<'a, Address<Eui48>> {
    /// Formats the address using hyphens and lower case hex values.
    /// 
    /// The alternate flag '#' has no effect on this implementation.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let value = self.0.as_ref();
        write!(f, "{:X}:{:X}:{:X}:{:X}:{:X}:{:X}", value[0], value[1], value[2], value[3], value[4], value[5])
    }
}

impl<'a> UpperHex for Colon<'a, Address<Eui64>> {
    /// Formats the address using hyphens and lower case hex values.
    /// 
    /// The alternate flag '#' has no effect on this implementation.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let value = self.0.as_ref();
        write!(f, "{:X}:{:X}:{:X}:{:X}:{:X}:{:X}:{:X}:{:X}", value[0], value[1], value[2], value[3], value[4], value[5], value[6], value[7])
    }
}

impl<'a> LowerHex for Colon<'a, Address<Eui48>> {
    /// Formats the address using hyphens and lower case hex values.
    /// 
    /// The alternate flag '#' has no effect on this implementation.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let value = self.0.as_ref();
        write!(f, "{:x}:{:x}:{:x}:{:x}:{:x}:{:x}", value[0], value[1], value[2], value[3], value[4], value[5])
    }
}

impl<'a> LowerHex for Colon<'a, Address<Eui64>> {
    /// Formats the address using hyphens and lower case hex values.
    /// 
    /// The alternate flag '#' has no effect on this implementation.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let value = self.0.as_ref();
        write!(f, "{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}", value[0], value[1], value[2], value[3], value[4], value[5], value[6], value[7])
    }
}

impl<'a> Debug for Colon<'a, Address<Eui48>> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:X}", self)
    }
}

impl<'a> Debug for Colon<'a, Address<Eui64>> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:X}", self)
    }
}

impl<'a> Display for Colon<'a, Address<Eui48>> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:X}", self)
    }
}

impl<'a> Display for Colon<'a, Address<Eui64>> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:X}", self)
    }
}

/// A helper type for formatting address value using dot seperators in the format "0123.4567.89AB".
pub struct Dot<'a, T: 'a>(pub &'a T);

impl<'a> UpperHex for Dot<'a, Address<Eui48>> {
    /// Formats the address using hyphens and lower case hex values.
    /// 
    /// The alternate flag '#' has no effect on this implementation.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let value = self.0.as_ref();
        write!(f, "{:X}{:X}.{:X}{:X}.{:X}{:X}", value[0], value[1], value[2], value[3], value[4], value[5])
    }
}

impl<'a> UpperHex for Dot<'a, Address<Eui64>> {
    /// Formats the address using hyphens and lower case hex values.
    /// 
    /// The alternate flag '#' has no effect on this implementation.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let value = self.0.as_ref();
        write!(f, "{:X}{:X}.{:X}{:X}.{:X}{:X}.{:X}{:X}", value[0], value[1], value[2], value[3], value[4], value[5], value[6], value[7])
    }
}

impl<'a> LowerHex for Dot<'a, Address<Eui48>> {
    /// Formats the address using hyphens and lower case hex values.
    /// 
    /// The alternate flag '#' has no effect on this implementation.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let value = self.0.as_ref();
        write!(f, "{:x}{:x}.{:x}{:x}.{:x}{:x}", value[0], value[1], value[2], value[3], value[4], value[5])
    }
}

impl<'a> LowerHex for Dot<'a, Address<Eui64>> {
    /// Formats the address using hyphens and lower case hex values.
    /// 
    /// The alternate flag '#' has no effect on this implementation.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let value = self.0.as_ref();
        write!(f, "{:x}{:x}.{:x}{:x}.{:x}{:x}.{:x}{:x}", value[0], value[1], value[2], value[3], value[4], value[5], value[6], value[7])
    }
}

impl<'a> Debug for Dot<'a, Address<Eui48>> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:X}", self)
    }
}

impl<'a> Debug for Dot<'a, Address<Eui64>> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:X}", self)
    }
}

impl<'a> Display for Dot<'a, Address<Eui48>> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:X}", self)
    }
}

impl<'a> Display for Dot<'a, Address<Eui64>> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:X}", self)
    }
}