//! Contains types and traits for writing and reading data from byte slices

/// An error struct used to communicate that an error occured while reading our writing a packet value.
/// This is mostly used to communicate that the output or input is too small to contain a value of a specified type
pub struct Error;

/// A trait used to determine the size of structs when serialized to an output
pub trait Size {
    /// Gets the size of the value when serialized to an output
    fn size(&self) -> usize;
}