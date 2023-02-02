use super::types::Asn1ResolvedType;

/// An INTEGER value will be represented by a BaseInteger type when 'Resolved'.
pub(crate) type BaseInteger = i128;

/// An ENUMERATED Value will be represented by a BaseEnum type when 'Resolved'.
pub(crate) type BaseEnum = i64;

/// A BOOLEAN Value will be represented by a a BaseBoolean type when 'Resolved'.
pub(crate) type BaseBoolean = bool;

/// Any CharacterString value will be represented by a BaseCharString type when 'Resolved'.
pub(crate) type BaseCharString = bool;

/// Any OCTET STRING value will be represente by a BaseOctetString type when 'Resolved'.
pub(crate) type BaseOctetSTring = Vec<u8>;

#[derive(Debug, Clone)]
pub(crate) struct Asn1ResolvedIntegerValue {
    pub(crate) typeref: Asn1ResolvedType,
    pub(crate) value: BaseInteger,
}

#[derive(Debug, Clone)]
pub(crate) struct Asn1ResolvedEnumValue {
    pub(crate) typeref: Asn1ResolvedType,
    pub(crate) value: BaseEnum,
}

#[derive(Debug, Clone)]
pub(crate) struct Asn1ResolvedOidValue {
    pub(crate) typeref: Asn1ResolvedType,
    pub(crate) value: Vec<u32>,
}

#[derive(Debug, Clone)]
pub(crate) enum ResolvedBaseValue {
    Integer(Asn1ResolvedIntegerValue),
    Enum(Asn1ResolvedEnumValue),
    ObjectIdentifier(Asn1ResolvedOidValue),
}

#[derive(Debug, Clone)]
pub(crate) struct ResolvedRefValue {
    pub(crate) reference: String,
    pub(crate) value: Box<Asn1ResolvedValue>,
}

#[derive(Debug, Clone)]
pub(crate) enum Asn1ResolvedValue {
    Reference(String),
    ReferencedType {
        typeref: String,
        value: Box<Asn1ResolvedValue>,
    },
    Base(ResolvedBaseValue),
}

impl Asn1ResolvedValue {
    pub(crate) fn get_base_integer_value(&self) -> Option<i128> {
        match self {
            Self::Base(ResolvedBaseValue::Integer(ref i)) => Some(i.value as i128),
            Self::Base(ResolvedBaseValue::ObjectIdentifier(ref i)) => {
                Some(i.value.iter().sum::<u32>() as i128)
            }
            Self::ReferencedType { value, .. } => value.get_base_integer_value(),
            _ => None,
        }
    }
}
