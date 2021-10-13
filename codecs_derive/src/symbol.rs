use syn::{Ident, Path};

#[derive(Copy, Clone)]
pub(crate) struct Symbol(pub(crate) &'static str);

pub(crate) const ASN: Symbol = Symbol("asn");
pub(crate) const LB: Symbol = Symbol("lb");
pub(crate) const UB: Symbol = Symbol("ub");
pub(crate) const TYPE: Symbol = Symbol("type");

impl PartialEq<Symbol> for Ident {
    fn eq(&self, word: &Symbol) -> bool {
        self == word.0
    }
}

impl PartialEq<Symbol> for Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word)
    }
}
