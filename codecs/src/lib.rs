//! Codec support for ASN.1 Types.

#![allow(dead_code)]
mod per;

#[doc(inline)]
pub use per::PerCodecData;

#[doc(inline)]
pub use per::PerCodecError;

#[doc(inline)]
pub use per::PerCodecErrorCause;

#[doc(inline)]
pub use per::aper;

#[doc(inline)]
pub use per::uper;

// For now making this public, eventually when we have a proper sequence extensions support, this
// will not be required anyways.
pub use per::common::decode::decode_sequence_extensions_skip_bits;
