//! ## **Falcon Packet Core**
//! This is the main component of [FalconMC](https://github.com/FalconMC-Dev/FalconMC)'s
//! networking system. It defines how types should be read from and written to a
//! minecraft network connection.
//!
//! The design philosophy of this crate is inspired by [serde](https://serde.rs).
//!
//! ## **Traits**
//! Six traits are introduced:
//! - [`PacketRead`]: How to read a type from the network.
//! - [`PacketWrite`]: How to write a type to the network.
//! - [`PacketSize`]: Memory-efficient size computation of the data
//!     when it would be written to the netork.
//! - [`PacketReadSeed`]: How to read a type from the network.
//!     This trait is used to pass data to the read implemenetation.
//! - [`PacketWriteSeed`]: How to write a type to the network.
//!     This trait is used to pass data to the write implemenetation.
//! - [`PacketSizeSeed`]: Memory-efficient size computation of the data
//!     when it would be written to the netork.
//!     This trait is used to pass data to the size implemenetation.
//!
//! Because [Minecraft's protocol](https://wiki.vg/) doesn't
//! translate one-to-one to Rust types, this crate offers some
//! convenient wrapper structs to correctly serialize
//! data such as strings or length-prefixed lists to the network.
//!
//! Some of these wrappers also help in reading
//! from the network while maintaining high memory efficiency
//! by leveraging the use of [`bytes::Bytes`].
//!
// //! ## **How to implement**
// //! For user implementations, it is highly encouraged to use the following
// //! derive macros:
// //! - [`PacketRead`](falcon_packet_core_derive::PacketRead)
// //! - [`PacketWrite`](falcon_packet_core_derive::PacketWrite)
// //! - [`PacketSize`](falcon_packet_core_derive::PacketSize)

use bytes::{Buf, BufMut};

pub use error::{ReadError, WriteError};

pub mod primitives;
mod error;

/// A data structure that can be read from a minecraft connection.
///
/// Users should aim to avoid implementing this trait directly, use the provided
/// [derive macros].
///
/// [derive macros]: falcon_packet_core#derives
pub trait PacketRead {
    /// This function extracts the type from the given buffer.
    ///
    /// # Important
    /// Implementations that read directly from the buffer
    /// (no redirection of this function/trait) **must ensure**
    /// that the remaining length of the buffer is always
    /// checked first before reading bytes from it.
    /// This is to eliminate panics.
    fn read<B>(buffer: &mut B) -> Result<Self, ReadError>
    where
        B: Buf + ?Sized,
        Self: Sized;
}

/// A data structure that can be written to a minecraft connection.
///
/// Users should aim to avoid implementing this trait directly, use the provided
/// [derive macros].
///
/// [derive macros]: falcon_packet_core#derives
pub trait PacketWrite: PacketSize {
    /// This function serializes the type to the given buffer.
    fn write<B>(&self, buffer: &mut B) -> Result<(), WriteError>
    where
        B: BufMut + ?Sized;
}

/// A data structure that can efficiently compute
/// its serialized size on the network buffer.
///
/// Users should aim to avoid implementing this trait directly, use the provided
/// [derive macros].
///
/// [derive macros]: falcon_packet_core#derives
pub trait PacketSize {
    /// This function computes the exact network
    /// size of the type.
    ///
    /// # Implementors
    /// It is highly encouraged to optimize this function.
    /// Avoid writing the type to a buffer and returning
    /// that buffer's change in length at all cost.
    fn size(&self) -> usize;
}

/// A data structure that can read another data type from a minecraft
/// connection, see [`DeserializeSeed`](https://docs.rs/serde/latest/serde/de/trait.DeserializeSeed.html).
///
/// This trait should rarely be implemented manually, if you implement this for
/// a general type, please contribute it to this project.
pub trait PacketReadSeed {
    /// The type produced by using this seed.
    type Value;

    /// This function extracts the type from the given buffer.
    ///
    /// # Important
    /// Implementations that read directly from the buffer
    /// (no redirection of this function/trait) **must ensure**
    /// that the remaining length of the buffer is always
    /// checked first before reading bytes from it.
    /// This is to eliminate panics.
    fn read<B>(self, buffer: &mut B) -> Result<Self::Value, ReadError>
    where
        B: Buf + ?Sized;
}

/// A data structure that can write another data type from a minecraft
/// connection, see [`SeralizeSeed`](https://docs.rs/serde/latest/serde/ser/trait.SerializeSeed.html).
///
/// This trait should rarely be implemented manually, if you implement this for
/// a general type, please contribute it to this project.
pub trait PacketWriteSeed<'a> {
    /// The type written by using this seed.
    type Value;

    /// This function serializes the type to the given buffer.
    fn write<B>(self, value: &'a Self::Value, buffer: &'a mut B) -> Result<(), WriteError>
    where
        B: BufMut + ?Sized;
}

/// A data structure that can efficiently compute
/// its serialized size on the network buffer.
/// It is a stateful variant of [`PacketSize`], see
/// [`SerializeSeed`](https://docs.rs/serde/latest/serde/de/trait.DeserializeSeed.html).
///
/// This trait should rarely be implemented manually, if you implement this for
/// a general type, please contribute it to this project.
pub trait PacketSizeSeed<'a> {
    /// The type measured by using this seed.
    type Value;

    /// This function computes the exact network
    /// size of the type.
    ///
    /// # Implementors
    /// It is highly encouraged to optimize this function.
    /// Avoid writing the type to a buffer and returning
    /// that buffer's change in length at all cost.
    fn size(self, value: &'a Self::Value) -> usize;
}
