//! Structs Related to Resolved Constraints

use std::ops::Range;

#[derive(Debug, Clone)]
pub(crate) struct ConstraintValues {
    pub(crate) ranges: Vec<Range<i64>>,
    pub(crate) values: Vec<i64>,
}

impl ConstraintValues {
    pub(crate) fn new() -> Self {
        ConstraintValues {
            ranges: vec![],
            values: vec![],
        }
    }

    pub(crate) fn append(&mut self, other: &Self) -> () {
        self.ranges.extend(other.ranges.clone());
        self.values.extend(other.values.clone());
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.values.is_empty() && self.ranges.is_empty()
    }

    pub(crate) fn min(&self) -> Option<i64> {
        let values_min = self.values.iter().min();
        let ranges_min = self.ranges.iter().map(|r| r.start).min();
        if values_min.is_none() {
            if ranges_min.is_none() {
                None
            } else {
                ranges_min
            }
        } else {
            if ranges_min.is_none() {
                Some(*values_min.unwrap())
            } else {
                Some(std::cmp::min(ranges_min.unwrap(), *values_min.unwrap()))
            }
        }
    }

    pub(crate) fn max(&self) -> Option<i64> {
        let values_max = self.values.iter().max();
        let ranges_max = self.ranges.iter().map(|r| r.end - 1).max();
        if values_max.is_none() {
            if ranges_max.is_none() {
                None
            } else {
                ranges_max
            }
        } else {
            if ranges_max.is_none() {
                Some(*values_max.unwrap())
            } else {
                Some(std::cmp::min(ranges_max.unwrap(), *values_max.unwrap()))
            }
        }
    }
}
#[derive(Debug, Clone)]
pub(crate) struct Asn1ConstraintValueSet {
    pub(crate) root_values: ConstraintValues,
    pub(crate) additional_values: Option<ConstraintValues>,
}

impl Asn1ConstraintValueSet {
    pub(crate) fn has_extension(&self) -> bool {
        self.additional_values.is_some()
    }
}
