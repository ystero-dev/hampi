//! Parsing related to constructed types SEQUENCE, SEQUENCE OF, SET, SET OF , CHOICE

mod utils;

mod choice;
pub(crate) use choice::parse_choice_type;

mod seq;
pub(crate) use seq::parse_seq_or_seq_of_type;
