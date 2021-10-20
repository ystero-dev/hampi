use syn::{Ident, Path};

#[derive(Copy, Clone)]
pub(crate) struct Symbol(pub(crate) &'static str);

pub(crate) const ASN: Symbol = Symbol("asn");
pub(crate) const LB: Symbol = Symbol("lb");
pub(crate) const UB: Symbol = Symbol("ub");
pub(crate) const TYPE: Symbol = Symbol("type");
pub(crate) const EXTENSIBLE: Symbol = Symbol("extensible");
pub(crate) const SZ_EXTENSIBLE: Symbol = Symbol("sz_extensible");
pub(crate) const SZ_LB: Symbol = Symbol("sz_lb");
pub(crate) const SZ_UB: Symbol = Symbol("sz_ub");
pub(crate) const KEY: Symbol = Symbol("key");
pub(crate) const EXTENDED: Symbol = Symbol("extended");

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
