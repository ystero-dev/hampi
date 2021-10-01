use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(AperCodec, attributes(asn))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;

    let tokens = quote! {
        use asn_codecs::{Asn1CodecParams, aper::{AperCodec, AperCodecData, AperCodecError}};

        impl AperCodec for #name {

            fn decode(&mut self, _data: &mut AperCodecData) -> Result<(), AperCodecError> {

                let params = Asn1CodecParams::from_ast(&ast);

                Ok(())
            }
        }
    };

    TokenStream::from(tokens)
}
