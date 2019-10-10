use crate::{Eui48, Eui64, Transmission, Admin};

pub trait Sealed { }
pub trait Eui: Sealed {
    fn transmission(&self) -> Transmission;
    fn admin(&self) -> Admin;
    fn reverse_bits(&mut self);
}

impl Sealed for Eui48 { }
impl Eui for Eui48 {
    fn transmission(&self) -> Transmission {
        match self[0] & 0x01 {
            0 => Transmission::Unicast,
            1 => Transmission::Multicast,
            _ => unreachable!()
        }
    }
    fn admin(&self) -> Admin {
        match (self[0] >> 1) & 0x01 {
            0 => Admin::Universal,
            1 => Admin::Local,
            _ => unreachable!()
        }
    }
    fn reverse_bits(&mut self) {
        self.reverse();
        self.iter_mut().for_each(|v| *v = u8::reverse_bits(*v))
    }
}
impl Sealed for Eui64 { }
impl Eui for Eui64 {
    fn transmission(&self) -> Transmission {
        match self[0] & 0x01 {
            0 => Transmission::Unicast,
            1 => Transmission::Multicast,
            _ => unreachable!()
        }
    }
    fn admin(&self) -> Admin {
        match (self[0] >> 1) & 0x01 {
            0 => Admin::Universal,
            1 => Admin::Local,
            _ => unreachable!()
        }
    }
    fn reverse_bits(&mut self) {
        self.reverse();
        self.iter_mut().for_each(|v| *v = u8::reverse_bits(*v))
    }
}

#[cfg(feature = "serde")]
pub mod serde {
    use crate::{Address, Eui48, Eui64};

    use core::fmt::{self, Formatter};
    use serde::de::Visitor;

    #[derive(Default)]
    pub struct AddressVisitor<T> {
        t: core::marker::PhantomData<T>
    }

    impl<'de> Visitor<'de> for AddressVisitor<Eui48> {
        type Value = Address<Eui48>;

        fn expecting(&self, f: &mut Formatter) -> fmt::Result {
            unimplemented!()
        }
    }

    impl<'de> Visitor<'de> for AddressVisitor<Eui64> {
        type Value = Address<Eui48>;

        fn expecting(&self, f: &mut Formatter) -> fmt::Result {
            unimplemented!()
        }
    }
}