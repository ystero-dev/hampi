//! Handling top level Generator code for a Resolved Type. Based on the individual type variant,
//! the respective functions are called.

use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::error::Error;
use crate::generator::Generator;
use crate::resolver::asn::structs::types::{Asn1ResolvedType, ResolvedSetType};

impl Asn1ResolvedType {
    pub(crate) fn generate_for_type(
        name: &str,
        ty: &Asn1ResolvedType,
        gen: &mut Generator,
    ) -> Result<Option<TokenStream>, Error> {
        match ty {
            Asn1ResolvedType::Base(ref b) => Ok(Some(b.generate_for_base_type(name, gen)?)),
            Asn1ResolvedType::Constructed(ref c) => Ok(Some(c.generate(name, gen)?)),
            Asn1ResolvedType::Set(ref s) => Ok(Some(s.generate(name, gen)?)),
            Asn1ResolvedType::Reference(ref reference) => Ok(Some(
                Asn1ResolvedType::generate_type_alias_for_reference(name, gen, reference)?,
            )),
        }
    }

    pub(crate) fn generate_name_maybe_aux_type(
        ty: &Asn1ResolvedType,
        generator: &mut Generator,
        input: Option<&String>,
    ) -> Result<Ident, Error> {
        match ty {
            Asn1ResolvedType::Base(ref b) => b.generate_ident_and_aux_type_for_base(generator),
            Asn1ResolvedType::Reference(ref r) => {
                Asn1ResolvedType::generate_ident_for_reference(r, generator)
            }
            Asn1ResolvedType::Constructed(ref c) => {
                c.generate_ident_and_aux_type_for_constucted(generator, input)
            }
            Asn1ResolvedType::Set(ref s) => {
                s.generate_ident_and_aux_types_for_set(generator, input)
            }
        }
    }

    pub(crate) fn generate_ident_for_reference(
        reference: &str,
        gen: &mut Generator,
    ) -> Result<Ident, Error> {
        Ok(gen.to_type_ident(reference))
    }

    fn generate_type_alias_for_reference(
        name: &str,
        gen: &mut Generator,
        reference: &str,
    ) -> Result<TokenStream, Error> {
        let referring = gen.to_type_ident(name);
        let reference = gen.to_type_ident(reference);

        Ok(quote! {
            pub type #referring = #reference;
        })
    }
}

impl ResolvedSetType {
    pub(crate) fn generate(
        &self,
        name: &str,
        generator: &mut Generator,
    ) -> Result<TokenStream, Error> {
        let ty_ident = generator.to_type_ident(name);
        let ty_elements = self.generate_aux_types(generator)?;

        Ok(quote! {
            #[derive(Debug, AperCodec)]
            pub enum #ty_ident {
                #ty_elements
            }
        })
    }

    pub(crate) fn generate_ident_and_aux_types_for_set(
        &self,
        generator: &mut Generator,
        input: Option<&String>,
    ) -> Result<Ident, Error> {
        // FIXME: This is perhaps not right
        let ty_ident = if input.is_none() {
            generator.to_type_ident(&self.setref)
        } else {
            generator.to_type_ident(input.unwrap())
        };
        let ty_elements = self.generate_aux_types(generator)?;

        let set_ty = quote! {
            #[derive(Debug, AperCodec)]
            #[asn(type = "OPEN")]
            pub enum #ty_ident {
                #ty_elements
            }
        };

        generator.aux_items.push(set_ty);

        Ok(ty_ident)
    }

    fn generate_aux_types(&self, generator: &mut Generator) -> Result<TokenStream, Error> {
        let mut variant_tokens = TokenStream::new();
        for (name, ty) in &self.types {
            let variant_ident = generator.to_type_ident(&name.0);
            let ty_ident = Asn1ResolvedType::generate_name_maybe_aux_type(&ty.1, generator, None)?;
            let key: proc_macro2::TokenStream = format!("{}", ty.0).parse().unwrap();
            let key_tokens = quote! {
                #[asn(key = #key)]
            };

            let variant_token = quote! {
                #key_tokens
                #variant_ident(#ty_ident),
            };
            variant_tokens.extend(variant_token);
        }
        Ok(variant_tokens)
    }
}
