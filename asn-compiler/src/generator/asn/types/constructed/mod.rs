#![allow(dead_code)]
//! Code Generator for Constructed Types.

mod choice;

mod seqof;

mod seq;

use proc_macro2::{Ident, TokenStream};

use crate::error::Error;
use crate::generator::Generator;
use crate::resolver::asn::structs::types::constructed::ResolvedConstructedType;

impl ResolvedConstructedType {
    // The main `generate` function for the constucted types
    //
    // This function simply calls the appropriate `generate_{sequence|choice|sequence_of}` function
    // for therespective types.
    pub(crate) fn generate(
        &self,
        name: &str,
        generator: &mut Generator,
    ) -> Result<TokenStream, Error> {
        match self {
            ResolvedConstructedType::Sequence { .. } => self.generate_sequence(name, generator),
            ResolvedConstructedType::Choice { .. } => self.generate_choice(name, generator),
            ResolvedConstructedType::SequenceOf { .. } => {
                self.generate_sequence_of(name, generator)
            }
        }
    }

    // This function is called for types 'within' constructed types.
    //
    // Usually whenever a type is defined 'inside a field of a `SEQUENCE` or a variant of a
    // `CHOICE`, this function is called for generating an appropriate 'auxillary type' and then
    // getting the identifier for that type which will be used by the caller inside it's field as
    // it's type. This is typically required because we cannot define a `struct` or an `enum`
    // within a `struct` or else it'd have just been an 'internal' type.
    //
    // For example, this function is called for definitions such as following
    // ```
    // Foo := SEQUENCE {
    //      a SEQUENCE {
    //          b INTEGER
    //      }
    //  }
    //  ```
    //
    //  Often the caller determines the name to be used, if the name is not passed, we simply
    //  generate a 'unique' name for the given type and go ahead and `generate` the 'internal'
    //  type. This can happen to any depth.
    pub(crate) fn generate_ident_and_aux_type_for_constucted(
        &self,
        generator: &mut Generator,
        input: Option<&String>,
    ) -> Result<Ident, Error> {
        let unique_name = match self {
            ResolvedConstructedType::Sequence { name, .. } => match input {
                Some(ref inp) => inp.to_string(),
                None => match name {
                    Some(ref n) => n.to_string(),
                    None => generator.get_unique_name("Sequence"),
                },
            },
            ResolvedConstructedType::Choice { .. } => match input {
                Some(ref inp) => inp.to_string(),
                None => generator.get_unique_name("Choice"),
            },
            ResolvedConstructedType::SequenceOf { .. } => match input {
                Some(ref inp) => inp.to_string(),
                None => generator.get_unique_name("SeqOf"),
            },
        };

        let generated_type = self.generate(&unique_name, generator)?;
        generator.aux_items.push(generated_type);
        Ok(generator.to_type_ident(&unique_name))
    }
}
