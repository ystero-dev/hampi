//! Functionality for decoding character strings
use bitvec::field::BitField;

use crate::per::PerCodecData;

use super::AperCodecError;

use super::decode_internal::decode_length_determinent_common;
