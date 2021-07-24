use std::collections::HashMap;

use crate::error::Error;

use crate::structs::parser::types::Asn1ConstructedType;
use crate::structs::resolver::{
    defs::Asn1ResolvedDefinition,
    types::constructed::{
        Asn1ResolvedChoice, Asn1ResolvedSequence, Asn1ResolvedSequenceOf, ResolvedConstructedType,
    },
};

pub(crate) fn resolve_constructed_type(
    ty: &Asn1ConstructedType,
    _table: &HashMap<String, Asn1ResolvedDefinition>,
) -> Result<ResolvedConstructedType, Error> {
    match ty {
        Asn1ConstructedType::Choice(ref _i) => {
            let resolved = ResolvedConstructedType::Choice(Asn1ResolvedChoice::default());
            Ok(resolved)
        }
        Asn1ConstructedType::Sequence(ref _i) => {
            let resolved = ResolvedConstructedType::Sequence(Asn1ResolvedSequence::default());
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
