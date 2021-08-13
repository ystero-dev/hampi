#![allow(dead_code)]
//! Code Generator for the SEQUENCE Type.

use proc_macro2::{Ident, TokenStream};

use crate::error::Error;
use crate::generator::Generator;
use crate::resolver::asn::structs::types::constructed::ResolvedConstructedType;

impl ResolvedConstructedType {
    pub(crate) fn generate_ident_and_aux_type_for_constucted(
        &self,
        _gen: &mut Generator,
    ) -> Result<Ident, Error> {
        Err(resolve_error!("Not Implemented!"))
    }

    pub(crate) fn generate_sequence(
        &self,
        _name: &str,
        _generator: &mut Generator,
    ) -> Result<TokenStream, Error> {
        if let ResolvedConstructedType::Sequence { .. } = self {
            Ok(TokenStream::new())
        } else {
            Ok(TokenStream::new())
        }
    }
}
