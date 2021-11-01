//! Utilities related to genrating Type/Field attributes based on Type Constraints.

use proc_macro2::TokenStream;
use quote::quote;

use crate::resolver::asn::structs::types::constraints::Asn1ConstraintValueSet;

impl Asn1ConstraintValueSet {
    pub(crate) fn get_ty_size_constraints_attrs(&self) -> TokenStream {
        let mut ty_attributes = TokenStream::new();

        let ext = self.has_extension();
        let sz_extensible = quote! { #ext };

        ty_attributes.extend(quote! { , sz_extensible = #sz_extensible });

        if self.root_values.min().is_some() {
            let lb = self.root_values.min().unwrap();
            ty_attributes.extend(quote! { , sz_lb = #lb });
        }
        if self.root_values.max().is_some() {
            let ub = self.root_values.max().unwrap();
            ty_attributes.extend(quote! { , sz_ub = #ub });
        }

        ty_attributes
    }
}
