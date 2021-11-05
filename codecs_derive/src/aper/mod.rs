//! Actual Implementation for Codec `impl` generation for different ASN Types.

use super::attrs::TyCodecParams;

mod bitstring;
mod boolean;
mod charstring;
mod choice;
mod enumerated;
mod integer;
mod null;
mod octetstring;
mod oid;
mod open;
mod seq;
mod seqof;

pub(crate) fn generate_decode(
    ast: &syn::DeriveInput,
    params: &TyCodecParams,
) -> proc_macro::TokenStream {
    let ty = params.ty.as_ref().unwrap();
    match ty.value().as_str() {
        "BOOLEAN" => boolean::generate_aper_decode_for_asn_boolean(ast, params),
        "CHOICE" => choice::generate_aper_decode_for_asn_choice(ast, params),
        "INTEGER" => integer::generate_aper_decode_for_asn_integer(ast, params),
        "ENUMERATED" => enumerated::generate_aper_decode_for_asn_enumerated(ast, params),
        "BITSTRING" => bitstring::generate_aper_decode_for_asn_bitstring(ast, params),
        "OCTET-STRING" => octetstring::generate_aper_decode_for_asn_octetstring(ast, params),
        "UTF8String" | "PrintableString" | "VisibleString" => {
            charstring::generate_aper_decode_for_asn_charstring(ast, params)
        }
        "NULL" => null::generate_aper_decode_for_asn_null(ast, params),
        "SEQUENCE" => seq::generate_aper_decode_for_asn_sequence(ast, params),
        "OPEN" => open::generate_aper_decode_for_asn_open_type(ast, params),
        "SEQUENCE-OF" => seqof::generate_aper_decode_for_asn_sequence_of(ast, params),
        "OBJECT-IDENTIFIER" => oid::generate_aper_decode_for_asn_object_identifier(ast, params),
        _ => syn::Error::new_spanned(ty.clone(), "This ASN.1 Type is not supported.")
            .to_compile_error()
            .into(),
    }
}
