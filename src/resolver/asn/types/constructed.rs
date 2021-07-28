use crate::error::Error;

use crate::parser::asn::structs::types::{
    constructed::{Asn1TypeChoice, Asn1TypeSequence},
    Asn1ConstructedType,
};

use crate::resolver::{
    asn::{
        structs::types::constructed::{
            Asn1ResolvedChoice, Asn1ResolvedSequence, Asn1ResolvedSequenceOf, ResolvedComponent,
            ResolvedConstructedType, ResolvedSeqComponent,
        },
        types::resolve_type,
    },
    Resolver,
};

pub(crate) fn resolve_constructed_type(
    ty: &Asn1ConstructedType,
    resolver: &Resolver,
) -> Result<ResolvedConstructedType, Error> {
    match ty {
        Asn1ConstructedType::Choice(ref c) => {
            let resolved = ResolvedConstructedType::Choice(resolved_choice_type(c, resolver)?);
            Ok(resolved)
        }
        Asn1ConstructedType::Sequence(ref s) => {
            let resolved = ResolvedConstructedType::Sequence(resolved_sequence_type(s, resolver)?);
            Ok(resolved)
        }
        Asn1ConstructedType::SequenceOf(ref _i) => {
            let resolved = ResolvedConstructedType::SequenceOf(Asn1ResolvedSequenceOf::default());
            Ok(resolved)
        }
        _ => {
            eprintln!("ConstructedType: {:#?}", ty);
            Err(resolve_error!("resolve_constructed_Type: Not Implemented!"))
        }
    }
}

fn resolved_choice_type(
    choice: &Asn1TypeChoice,
    resolver: &Resolver,
) -> Result<Asn1ResolvedChoice, Error> {
    let mut components = vec![];
    for c in &choice.components {
        let ty = resolve_type(&c.ty, resolver)?;
        let component = ResolvedComponent {
            id: c.id.clone(),
            ty,
        };
        components.push(component);
    }

    Ok(Asn1ResolvedChoice { components })
}

fn resolved_sequence_type(
    sequence: &Asn1TypeSequence,
    resolver: &Resolver,
) -> Result<Asn1ResolvedSequence, Error> {
    let mut components = vec![];
    for c in &sequence.root_components {
        let ty = resolve_type(&c.component.ty, resolver)?;
        let component = ResolvedComponent {
            id: c.component.id.clone(),
            ty,
        };
        let seq_component = ResolvedSeqComponent {
            component,
            optional: c.optional,
        };
        components.push(seq_component);
    }

    Ok(Asn1ResolvedSequence { components })
}
