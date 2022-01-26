//! Structs Related to Resolved Constraints

use std::ops::Range;

#[derive(Debug, Clone)]
pub(crate) struct ConstraintValues {
    pub(crate) ranges: Vec<Range<i128>>,
    pub(crate) values: Vec<i128>,
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

    pub(crate) fn min(&self) -> Option<i128> {
        let values_min = self.values.iter().min();
        let ranges_min = self.ranges.iter().map(|r| r.start).min();
        match values_min {
            None => ranges_min,
            Some(vals_min) => match ranges_min {
                None => Some(*vals_min),
                Some(rnges_min) => Some(std::cmp::min(rnges_min, *vals_min)),
            },
        }
    }

    pub(crate) fn max(&self) -> Option<i128> {
        let values_max = self.values.iter().max();
        let ranges_max = self.ranges.iter().map(|r| r.end - 1).max();
        match values_max {
            None => ranges_max,
            Some(vals_max) => match ranges_max {
                None => Some(*vals_max),
                Some(rnges_max) => Some(std::cmp::min(rnges_max, *vals_max)), // FIXME: This should be std::cmp::max?
            },
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
