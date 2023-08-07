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
mod real;
mod seq;
mod seqof;

pub(crate) fn generate_codec(
    ast: &syn::DeriveInput,
    params: &TyCodecParams,
    aligned: bool,
) -> proc_macro::TokenStream {
    let ty = params.ty.as_ref().unwrap();
    match ty.value().as_str() {
        "BOOLEAN" => boolean::generate_aper_codec_for_asn_boolean(ast, params, aligned),
        "CHOICE" => choice::generate_aper_codec_for_asn_choice(ast, params, aligned),
        "INTEGER" => integer::generate_aper_codec_for_asn_integer(ast, params, aligned),
        "ENUMERATED" => enumerated::generate_aper_codec_for_asn_enumerated(ast, params, aligned),
        "BITSTRING" => bitstring::generate_aper_codec_for_asn_bitstring(ast, params, aligned),
        "OCTET-STRING" => {
            octetstring::generate_aper_codec_for_asn_octetstring(ast, params, aligned)
        }
        "UTF8String" | "PrintableString" | "VisibleString" => {
            charstring::generate_aper_codec_for_asn_charstring(ast, params, aligned)
        }
        "NULL" => null::generate_aper_codec_for_asn_null(ast, params, aligned),
        "REAL" => real::generate_aper_codec_for_asn_real(ast, params, aligned),
        "SEQUENCE" => seq::generate_aper_codec_for_asn_sequence(ast, params, aligned),
        "OPEN" => open::generate_aper_codec_for_asn_open_type(ast, params, aligned),
        "SEQUENCE-OF" => seqof::generate_aper_codec_for_asn_sequence_of(ast, params, aligned),
        "OBJECT-IDENTIFIER" => {
            oid::generate_aper_codec_for_asn_object_identifier(ast, params, aligned)
        }
        _ => syn::Error::new_spanned(ty.clone(), "This ASN.1 Type is not supported.")
            .to_compile_error()
            .into(),
    }
}
