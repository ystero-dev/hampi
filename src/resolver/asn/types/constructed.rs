use crate::error::Error;

use crate::parser::asn::structs::types::{
    constructed::{Asn1TypeChoice, Asn1TypeSequence, Asn1TypeSequenceOf},
    Asn1ConstructedType, Asn1Type, Asn1TypeKind,
};

use crate::resolver::{
    asn::{
        structs::types::constructed::{
            ResolvedComponent, ResolvedConstructedType, ResolvedSeqComponent,
        },
        types::resolve_type,
    },
    Resolver,
};

pub(crate) fn resolve_constructed_type(
    ty: &Asn1Type,
    resolver: &Resolver,
) -> Result<ResolvedConstructedType, Error> {
    if let Asn1TypeKind::Constructed(ref kind) = ty.kind {
        match kind {
            Asn1ConstructedType::Choice(ref c) => resolve_choice_type(c, resolver),
            Asn1ConstructedType::Sequence(ref s) => resolve_sequence_type(s, resolver),
            Asn1ConstructedType::SequenceOf(ref so) => resolve_sequence_of_type(so, resolver),
            _ => {
                eprintln!("ConstructedType: {:#?}", ty);
                Err(resolve_error!("resolve_constructed_Type: Not Implemented!"))
            }
        }
    } else {
        Err(resolve_error!(
            "Expected Constructed Type. Found '{:#?}'",
            ty
        ))
    }
}

fn resolve_choice_type(
    choice: &Asn1TypeChoice,
    resolver: &Resolver,
) -> Result<ResolvedConstructedType, Error> {
    let mut components = vec![];
    for c in &choice.components {
        let ty = resolve_type(&c.ty, resolver)?;
        let component = ResolvedComponent {
            id: c.id.clone(),
            ty,
        };
        components.push(component);
    }

    Ok(ResolvedConstructedType::Choice { components })
}

fn resolve_sequence_type(
    sequence: &Asn1TypeSequence,
    resolver: &Resolver,
) -> Result<ResolvedConstructedType, Error> {
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

    Ok(ResolvedConstructedType::Sequence { components })
}

fn resolve_sequence_of_type(
    sequence_of: &Asn1TypeSequenceOf,
    resolver: &Resolver,
) -> Result<ResolvedConstructedType, Error> {
    let resolved = resolve_type(&sequence_of.ty, resolver)?;
    Ok(ResolvedConstructedType::SequenceOf {
        ty: Box::new(resolved),
    })
}
