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
                        "Constraint is not a component Relation Constraint!"
                    ))
                }
            }
        }
    }

    // FIXME: only gets the dependent Set for now. Types in other constraints not yet supported.
    /// Returns Dependent Components for the constraint
    pub(crate) fn dependent_references(&self) -> Vec<String> {
        match self.get_set_reference() {
            Ok(ref s) => vec![s.clone()],
            Err(_) => vec![],
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
            let mut element_values = element.get_integer_valueset(resolver)?;
            root_values.append(&mut element_values);
        }

        let additional_values = if self.additional_elements.is_some() {
            let additional_elements = self.additional_elements.as_ref().unwrap();
            let mut additional_values = ConstraintValues::new();
            for element in &additional_elements.elements {
                let mut element_values = element.get_integer_valueset(resolver)?;
                additional_values.append(&mut element_values);
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
            let mut element_set = element.get_integer_valueset(resolver)?;
            if value_set.is_empty() {
                // If Empty Set, it's okay to union, but not otherwise
                value_set.append(&mut element_set);
            } else {
                value_set.values = value_set
                    .values
                    .into_iter()
                    .filter(|e| element_set.values.contains(e))
                    .collect::<Vec<i64>>();
                value_set.ranges = value_set
                    .ranges
                    .into_iter()
                    .filter(|e| element_set.ranges.contains(e))
                    .collect::<Vec<Range<i64>>>();
            }
        }
        Ok(value_set)
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
                let mut all_values_set = ty.get_integer_valueset_from_constraint(resolver)?;
                value_set.append(&mut all_values_set.root_values); // We only care about Root Elements
            }
            Self::ValueRange {
                lower,
                lower_inclusive,
                upper,
                upper_inclusive,
            } => {
                eprintln!("Range: {:#?} ", self);
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
            _ => {
                return Err(constraint_error!(
                    "Unexpected Constraint Type '{:#?}'",
                    self
                ));
            }
        }
        Ok(value_set)
    }

    fn parse_or_resolve_value(value: &String, resolver: &Resolver) -> Result<i64, Error> {
        // FIXME : do the 'resolve part'
        let parsed = value.parse::<i64>();
        match parsed {
            Ok(x) => Ok(x),
            Err(_) => {
                let resolved = resolver.resolved_defs.get(value);
                if resolved.is_none() {
                    Err(constraint_error!(
                        "Unable To Resolve '{}'. Not Found!",
                        value
                    ))
                } else {
                    let resolved = resolved.unwrap();
                    if let Asn1ResolvedDefinition::Value(Asn1ResolvedValue::Base(
                        ResolvedBaseValue::Integer(ref i),
                    )) = resolved
                    {
                        Ok(i.value)
                    } else {
                        Err(constraint_error!(
                            "Resolved Value {:#?} of different type!",
                            resolved
                        ))
                    }
                }
            }
        }
    }
}
