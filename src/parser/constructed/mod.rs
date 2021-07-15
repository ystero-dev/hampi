//! Parsing related to constructed types SEQUENCE, SEQUENCE OF, SET, SET OF , CHOICE

mod choice;
pub(crate) use choice::parse_choice_type;

mod utils;
