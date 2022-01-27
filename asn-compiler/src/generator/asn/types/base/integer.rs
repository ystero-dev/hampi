//! Mainly 'generator' code for `Asn1ResolvedInteger`

use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::error::Error;

use crate::generator::Generator;
use crate::resolver::asn::structs::types::base::Asn1ResolvedInteger;

impl Asn1ResolvedInteger {
    pub(crate) fn generate(
        &self,
        name: &str,
        generator: &mut Generator,
    ) -> Result<TokenStream, Error> {
        let struct_name = generator.to_type_ident(name);
        let inner_type = generator.to_inner_type(self.bits, self.signed);
        let (min, max) = self.get_min_max_constraints();
        let extensible = self.resolved_constraints.is_some()
            && self.resolved_constraints.as_ref().unwrap().has_extension();

        let lb = if min.is_some() {
            let min = format!("{}", min.unwrap());
            Some(quote! { #min })
        } else {
            None
        };

        let ub = if max.is_some() {
            let max = format!("{}", max.unwrap());
            Some(quote! { #max })
        } else {
            None
        };

        let mut ty_tokens = quote! { type = "INTEGER" };
        if lb.is_some() {
            ty_tokens.extend(quote! {
                , lb = #lb
            });
        }
        if ub.is_some() {
            ty_tokens.extend(quote! {
                , ub = #ub
            });
        }

        if extensible {
            ty_tokens.extend(quote! {
                , extensible = true
            });
        }

        let struct_tokens = quote! {
            #[derive(Debug, AperCodec)]
            #[asn(#ty_tokens)]
            pub struct #struct_name(#inner_type);
        };

        Ok(struct_tokens)
    }

    pub(crate) fn generate_ident_and_aux_type(
        &self,
        generator: &mut Generator,
    ) -> Result<Ident, Error> {
        let unique_name = generator.get_unique_name("INTEGER");

        let item = self.generate(&unique_name, generator)?;
        generator.aux_items.push(item);

        Ok(generator.to_type_ident(&unique_name))
    }

    fn get_min_max_constraints(&self) -> (Option<i128>, Option<i128>) {
        if self.resolved_constraints.is_none() {
            (None, None)
        } else {
            let constraints = self.resolved_constraints.as_ref().unwrap();

            (constraints.root_values.min(), constraints.root_values.max())
        }
    }
}
