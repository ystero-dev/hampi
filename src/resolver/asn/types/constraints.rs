//! Constraint Resolution Implementation

use crate::error::Error;

use crate::parser::asn::structs::types::constraints::*;

impl Asn1Constraint {
    /// Returns 'value' for the constraint
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
            let value = e.get_single_value();
            if value.is_some() {
                Ok(value.unwrap())
            } else {
                Err(constraint_error!("Not Single Value!"))
            }
        } else if let Asn1Constraint::Table(ref t) = self {
            let value = t.get_single_value();
            if value.is_some() {
                Ok(value.unwrap())
            } else {
                Err(constraint_error!("Not Single Value!"))
            }
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

    fn get_single_value(&self) -> Option<String> {
        let inner_elements = self.get_inner_elements();
        if inner_elements.len() == 1 {
            let e = &inner_elements[0];

            e.get_single_value()
        } else {
            None
        }
    }
}

impl IntersectionSet {
    fn get_single_value(&self) -> Option<String> {
        if self.elements.len() == 1 {
            let element = &self.elements[0];
            if let Elements::Subtype(SubtypeElements::SingleValue { value }) = element {
                Some(value.clone())
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl TableConstraint {
    fn get_single_value(&self) -> Option<String> {
        if let TableConstraint::Simple(ObjectSet::DefinedObjectSet(ref s)) = self {
            Some(s.clone())
        } else {
            None
        }
    }
}
