//! Code generation related utilities for `Asn1ResolvedValue`

use proc_macro2::{Literal, TokenStream};
use quote::quote;

use crate::error::Error;
use crate::generator::Generator;
use crate::resolver::asn::structs::{
    types::{base::ResolvedBaseType, Asn1ResolvedType},
    values::{Asn1ResolvedValue, ResolvedBaseValue},
};

impl Asn1ResolvedValue {
    // This function generates constants for values that are of base types or references to base
    // types (with possible constraints). These values can be used by the application code as
    // constants. For example something like -
    //
    // ```asn1
    // id-NGSetup ProcedureCode ::= 21
    // ```
    // Would become
    // ```rust
    // const ID_NGSETUP: u8 = 21;
    // ```
    // Where `ProcedureCode` is a type Integer with constraints min value 0 max value 255 - defined
    // like `ProcedureCode ::= INTEGER (0..255)` in ASN.1 specification.
    pub(crate) fn generate_const_for_base_value(
        name: &str,
        value: &Asn1ResolvedValue,
        gen: &mut Generator,
    ) -> Result<Option<TokenStream>, Error> {
        match value {
            Asn1ResolvedValue::Base(v) => Self::tokens_from_resolved_base_value(name, v, gen),
            Asn1ResolvedValue::ReferencedType { value, .. } => {
                Self::generate_const_for_base_value(name, value, gen)
            }
            _ => Ok(None),
        }
    }

    fn tokens_from_resolved_base_value(
        name: &str,
        base: &ResolvedBaseValue,
        gen: &mut Generator,
    ) -> Result<Option<TokenStream>, Error> {
        if let ResolvedBaseValue::Integer(i) = base {
            if let Asn1ResolvedType::Base(ResolvedBaseType::Integer(ref typ)) = i.typeref {
                let const_type = gen.to_inner_type(typ.bits, typ.signed);
                let const_id = gen.to_const_ident(name);
                let vis = gen.get_visibility_tokens();
                let val = Literal::i128_unsuffixed(i.value);
                let tokens = quote! {
                    #vis const #const_id : #const_type = #val;
                };

                return Ok(Some(tokens));
            };
        };
        Ok(None)
    }
}
