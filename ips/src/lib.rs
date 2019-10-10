//! A library for reading and writing type-safe net packets

#![no_std]

extern crate alloc;

mod internal {
    pub trait Sealed { }
}

pub mod physical;
pub mod link;
pub mod payload;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
