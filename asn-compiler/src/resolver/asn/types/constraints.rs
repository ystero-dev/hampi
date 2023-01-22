//! Constraint Resolution Implementation
use std::ops::Range;

use crate::error::Error;

use crate::parser::asn::structs::types::constraints::*;

use crate::resolver::asn::structs::{
    defs::Asn1ResolvedDefinition,
    types::constraints::{Asn1ConstraintValueSet, ConstraintValues},
    values::{Asn1ResolvedValue, ResolvedBaseValue},
};
use crate::resolver::Resolver;

impl Asn1Constraint {
    /// Returns 'value' `String` for the constraint
    ///
    /// This function should be called for Single Value Subtype Constraints or Simple Table
    /// constraints only.
    pub(crate) fn get_single_string_value(&self) -> Result<String, Error> {
        if !self.is_subtype() || !self.is_single_value() || !self.is_simple_table_constraint() {
            Err(constraint_error!(
                "Require a Single Value Subtype or Table Constraint. Found '{:#?}'.",
                self
            ))
        } else if let Asn1Constraint::Subtype(ref e) = self {
            e.get_single_string_value()
        } else if let Asn1Constraint::Table(ref t) = self {
            t.get_single_string_value()
        } else {
            Err(constraint_error!(
                "Single Value not supported for '{:#?}'",
                self
            ))
        }
    }

    /// Returns the 'Set Reference' trimming the leading '{' and trailing '}'
    pub(crate) fn get_set_reference(&self) -> Result<String, Error> {
        match self.get_single_string_value() {
            Ok(v) => Ok(v
                .trim_matches(|c| matches!(c, '{' | '}'))
                .trim()
                .to_string()),
            Err(_) => {
                if let Self::Table(TableConstraint::ComponentRelation { table, .. }) = self {
                    Ok(table
                        .trim_matches(|c| matches!(c, '{' | '}'))
                        .trim()
                        .to_string())
                } else {
                    Err(resolve_error!(
                        "Constraint is not a component Relation Constraint! '{:#?}'",
                        self
                    ))
                }
            }
        }
    }
    /// Returns the 'Set Reference' trimming the leading '{' and trailing '}'
    pub(crate) fn get_comp_reference(&self) -> Option<String> {
        if let Self::Table(TableConstraint::ComponentRelation { component, .. }) = self {
            Some(
                component
                    .trim_matches(|c| matches!(c, '@' | '.'))
                    .to_string(),
            )
        } else {
            None
        }
    }

    // FIXME: only gets the dependent Set for now. Types in other constraints not yet supported.
    /// Returns Dependent Components for the constraint
    pub(crate) fn dependent_references(&self) -> Vec<String> {
        match self {
            Self::Table(ref _t) => vec![self.get_set_reference().unwrap()],
            Self::Subtype(ref s) => s.clone().dependent_references(), // FIXME: Need to get reference
            Self::Contents { .. } => vec![], // FIXME: Not sure but perhaps this causes lot of circular dependencies
        }
    }

    pub(crate) fn get_integer_valueset(
        &self,
        resolver: &Resolver,
    ) -> Result<Asn1ConstraintValueSet, Error> {
        match self {
            Self::Subtype(ref e) => e.get_integer_valueset(resolver),
            _ => Err(constraint_error!(
                "Integer Values not supported for the constraint '{:#?}'",
                self
            )),
        }
    }
    // Returns whether this constraint is a Subtype Constraint
    fn is_subtype(&self) -> bool {
        true
    }

    // Returns whether this constraint is a Single Value Constraint
    fn is_single_value(&self) -> bool {
        true
    }

    // Returns whether this constraint is a simple table Constraint
    fn is_simple_table_constraint(&self) -> bool {
        true
    }

    // Returns whether the constraint is a 'size' constraint.
    pub(crate) fn is_size_constraint(&self) -> bool {
        if let Self::Subtype(ref e) = self {
            let iset = e.get_inner_elements();
            if iset.len() != 1 {
                false
            } else {
                matches!(
                    iset[0].elements[0],
                    Elements::Subtype(SubtypeElements::SizeConstraint(..))
                )
            }
        } else {
            false
        }
    }

    pub(crate) fn get_size_valueset(
        &self,
        resolver: &Resolver,
    ) -> Result<Asn1ConstraintValueSet, Error> {
        if let Self::Subtype(ref e) = self {
            let iset = e.get_inner_elements();
            if iset.len() == 1 {
                match iset[0].elements[0] {
                    Elements::Subtype(ref s) => match s {
                        SubtypeElements::SizeConstraint(ref elems) => {
                            elems.get_integer_valueset(resolver)
                        }
                        _ => Err(constraint_error!(
                            "The Constraint for the Type is not a Size Constraint."
                        )),
                    },
                    _ => Err(constraint_error!(
                        "The Constraint for the type is not a size Constraint."
                    )),
                }
            } else {
                Err(constraint_error!(
                    "The Constraint for the type is not a size Constraint."
                ))
            }
        } else {
            Err(constraint_error!(
                "The Constraint for the type is not a size Constraint."
            ))
        }
    }
}

impl ElementSet {
    fn get_inner_elements(&self) -> &Vec<IntersectionSet> {
        &self.root_elements.elements
    }

    fn get_single_string_value(&self) -> Result<String, Error> {
        let inner_elements = self.get_inner_elements();
        if inner_elements.len() == 1 {
            let e = &inner_elements[0];

            e.get_single_string_value()
        } else {
            Err(constraint_error!(
                "The Length of the element set is {}, while expected length is 1.",
                inner_elements.len()
            ))
        }
    }

    fn get_integer_valueset(&self, resolver: &Resolver) -> Result<Asn1ConstraintValueSet, Error> {
        let mut root_values = ConstraintValues::new();
        for element in self.get_inner_elements() {
            let element_values = element.get_integer_valueset(resolver)?;
            root_values.append(&element_values);
        }

        let additional_values = if self.additional_elements.is_some() {
            let additional_elements = self.additional_elements.as_ref().unwrap();
            let mut additional_values = ConstraintValues::new();
            for element in &additional_elements.elements {
                let element_values = element.get_integer_valueset(resolver)?;
                additional_values.append(&element_values);
            }
            Some(additional_values)
        } else {
            None
        };

        Ok(Asn1ConstraintValueSet {
            root_values,
            additional_values,
        })
    }

    fn dependent_references(self) -> Vec<String> {
        let mut output = vec![];
        output.extend(self.root_elements.dependent_references());

        if self.additional_elements.is_some() {
            output.extend(
                self.additional_elements
                    .as_ref()
                    .unwrap()
                    .dependent_references(),
            );
        }
        output
    }
}

impl IntersectionSet {
    fn get_single_string_value(&self) -> Result<String, Error> {
        if self.elements.len() == 1 {
            let element = &self.elements[0];
            if let Elements::Subtype(SubtypeElements::SingleValue { value }) = element {
                Ok(value.clone())
            } else {
                Err(constraint_error!(
                    "The Element is not a SingleValue Subtype Element!"
                ))
            }
        } else {
            Err(constraint_error!(
                "The Length of the element set is {}, while expected length is 1.",
                self.elements.len()
            ))
        }
    }

    fn get_integer_valueset(&self, resolver: &Resolver) -> Result<ConstraintValues, Error> {
        let mut value_set = ConstraintValues::new();
        for element in &self.elements {
            let element_set = element.get_integer_valueset(resolver)?;
            if value_set.is_empty() {
                // If Empty Set, it's okay to union, but not otherwise
                value_set.append(&element_set);
            } else {
                value_set.values = value_set
                    .values
                    .into_iter()
                    .filter(|e| element_set.values.contains(e))
                    .collect::<Vec<i128>>();
                value_set.ranges = value_set
                    .ranges
                    .into_iter()
                    .filter(|e| element_set.ranges.contains(e))
                    .collect::<Vec<Range<i128>>>();
            }
        }
        Ok(value_set)
    }

    fn dependent_references(&self) -> Vec<String> {
        let mut output = vec![];
        for element in &self.elements {
            output.extend(element.dependent_references())
        }
        output
    }
}

impl TableConstraint {
    fn get_single_string_value(&self) -> Result<String, Error> {
        if let TableConstraint::Simple(ObjectSet::DefinedObjectSet(ref s)) = self {
            Ok(s.clone())
        } else {
            Err(constraint_error!("Shouldn't Reach here!"))
        }
    }
}

impl Elements {
    fn get_integer_valueset(&self, resolver: &Resolver) -> Result<ConstraintValues, Error> {
        match self {
            Self::Subtype(ref s) => s.get_integer_valueset(resolver),
            Self::Set(ref _s) => Err(constraint_error!(
                "get_integer_valueset: Set Variant: Not Supported!"
            )),
        }
    }

    fn dependent_references(&self) -> Vec<String> {
        match self {
            Self::Subtype(ref s) => s.dependent_references(),
            Self::Set(ref e) => e.clone().dependent_references(),
        }
    }
}

impl SubtypeElements {
    fn get_integer_valueset(&self, resolver: &Resolver) -> Result<ConstraintValues, Error> {
        let mut value_set = ConstraintValues::new();
        match self {
            Self::SingleValue { value } => {
                value_set
                    .values
                    .push(Self::parse_or_resolve_value(value, resolver)?);
            }
            Self::ConstrainedSubtype(ref ty) => {
                let all_values_set = ty.get_integer_valueset_from_constraint(resolver)?;
                value_set.append(&all_values_set.root_values); // We only care about Root Elements
            }
            Self::ValueRange {
                lower,
                lower_inclusive,
                upper,
                upper_inclusive,
            } => {
                let mut lower_value = Self::parse_or_resolve_value(lower, resolver)?;
                if !lower_inclusive {
                    lower_value += 1;
                }
                let mut upper_value = Self::parse_or_resolve_value(upper, resolver)?;
                if !upper_inclusive {
                    upper_value -= 1;
                }
                value_set.ranges.push(Range {
                    start: lower_value,
                    end: upper_value + 1,
                });
            }
            Self::SizeConstraint(ref elems) => {
                let size_elements = elems.get_integer_valueset(resolver)?;
                value_set.append(&size_elements.root_values);
                if size_elements.additional_values.is_some() {
                    value_set.append(&size_elements.additional_values.unwrap());
                }
            }
            _ => {
                return Err(constraint_error!(
                    "Unexpected Constraint Type '{:#?}'",
                    self
                ));
            }
        }
        Ok(value_set)
    }

    fn parse_or_resolve_value(value: &str, resolver: &Resolver) -> Result<i128, Error> {
        // FIXME : do the 'resolve part'
        let parsed = value.parse::<i128>();
        match parsed {
            Ok(x) => Ok(x),
            Err(_) => {
                let resolved = resolver.resolved_defs.get(value);
                match resolved {
                    None => Err(constraint_error!(
                        "Unable To Resolve '{}'. Not Found!",
                        value
                    )),
                    Some(res) => {
                        if let Asn1ResolvedDefinition::Value(Asn1ResolvedValue::Base(
                            ResolvedBaseValue::Integer(ref i),
                        )) = res
                        {
                            Ok(i.value)
                        } else {
                            Err(constraint_error!(
                                "Resolved Value {:#?} of different type!",
                                res
                            ))
                        }
                    }
                }
            }
        }
    }

    fn dependent_references(&self) -> Vec<String> {
        match self {
            Self::SingleValue { value } => match value.parse::<i128>() {
                Ok(_) => vec![],
                Err(_) => vec![value
                    .trim_matches(|c| matches!(c, '{' | '}'))
                    .trim()
                    .to_string()],
            },
            Self::ConstrainedSubtype(ref t) => t.dependent_references(),
            Self::ValueRange { lower, upper, .. } => {
                let mut output = vec![];
                match lower.parse::<i128>() {
                    Ok(_) => {}
                    Err(_) => output.extend(vec![lower.trim().to_string()]),
                };
                match upper.parse::<i128>() {
                    Ok(_) => {}
                    Err(_) => output.extend(vec![upper.trim().to_string()]),
                };
                output
            }
            Self::SizeConstraint(ref s) => s.clone().dependent_references(),
            Self::PermittedAlphabet(ref _p) => vec![], // FIXME: Should we?
            Self::PermittedAlphabetExcept(ref _p) => vec![], // FIXME: Should we?
        }
    }
}

impl UnionSet {
    fn dependent_references(&self) -> Vec<String> {
        let mut output = vec![];
        for element in &self.elements {
            output.extend(element.dependent_references())
        }
        output
    }
}
