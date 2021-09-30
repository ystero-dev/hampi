use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, DeriveInput};

#[proc_macro_derive(AperCodec)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse(input).unwrap();

    let name = &ast.ident;

    let tokens = quote! {
        use asn_codecs::aper::{AperCodec, AperCodecData, Asn1CodecParams, AperCodecError};

        impl AperCodec for #name {

            fn decode(&mut self, _data: &mut AperCodecData, _params: &Asn1CodecParams) -> Result<(), AperCodecError> {
                Ok(())
            }
        }
    };

    TokenStream::from(tokens)
}
