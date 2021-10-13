use proc_macro::TokenStream;
use quote::quote;
use syn::Meta::List;
use syn::{parse_macro_input, DeriveInput};
use syn::{Ident, Lit, Path};

#[proc_macro_derive(AperCodec, attributes(asn))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;

    let meta_items_iter = ast
        .attrs
        .iter()
        .flat_map(|attr| get_asn_meta_items(attr))
        .flatten()
        .collect::<Vec<syn::NestedMeta>>();

    let codec_params = get_codec_params_from_meta_items(&meta_items_iter).unwrap();
    eprintln!("codec_params : {:#?}", codec_params);
    let lb = codec_params.lb;
    let ub = codec_params.ub;

    let tokens = quote! {

        impl asn_codecs::aper::AperCodec for #name {
            type Output = Self;

            fn decode(data: &mut asn_codecs::aper::AperCodecData) -> Result<Self::Output, asn_codecs::aper::AperCodecError> {

                asn_codecs::aper::decode_choice_idx(#lb, #ub);

                Ok(Self{})
            }
        }
    };

    TokenStream::from(tokens)
}

#[derive(Copy, Clone)]
struct Symbol(&'static str);

const ASN: Symbol = Symbol("asn");
const LB: Symbol = Symbol("lb");
const UB: Symbol = Symbol("ub");

impl PartialEq<Symbol> for Ident {
    fn eq(&self, word: &Symbol) -> bool {
        self == word.0
    }
}

impl PartialEq<Symbol> for Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word)
    }
}

#[derive(Debug, Default)]
struct CodecParams {
    lb: Option<syn::LitInt>,
    ub: Option<syn::LitInt>,
    //extensible: Option<Literal>,
}

pub(crate) fn get_asn_meta_items(attr: &syn::Attribute) -> Result<Vec<syn::NestedMeta>, ()> {
    if attr.path != ASN {
        Ok(Vec::new())
    } else {
        match attr.parse_meta() {
            Ok(List(meta)) => Ok(meta.nested.into_iter().collect()),
            _ => Err(()),
        }
    }
}

pub(crate) fn get_codec_params_from_meta_items(
    items: &Vec<syn::NestedMeta>,
) -> Result<CodecParams, ()> {
    let mut codec_params = CodecParams::default();

    for item in items {
        match item {
            syn::NestedMeta::Meta(syn::Meta::NameValue(m)) if m.path == LB => match m.lit {
                syn::Lit::Int(ref i) => {
                    let lb = i.clone();
                    codec_params.lb.replace(lb);
                }
                _ => (),
            },
            syn::NestedMeta::Meta(syn::Meta::NameValue(m)) if m.path == UB => match m.lit {
                syn::Lit::Int(ref i) => {
                    let ub = i.clone();
                    codec_params.ub.replace(ub);
                }
                _ => (),
            },
            _ => (),
        }
    }
    Ok(codec_params)
}
