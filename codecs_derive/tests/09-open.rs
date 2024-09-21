#![allow(non_camel_case_types, dead_code)]

use asn1_codecs_derive::{AperCodec, UperCodec};

#[derive(Debug, AperCodec, UperCodec, Eq, PartialEq)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct ProtocolIE_ID(u16);

#[derive(Debug, AperCodec, UperCodec, Eq, PartialEq)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct Criticality(u8);
impl Criticality {
    const REJECT: u8 = 0u8;
    const IGNORE: u8 = 1u8;
    const NOTIFY: u8 = 2u8;
}

#[derive(Debug, AperCodec, UperCodec, Eq, PartialEq)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct Routing_ID(u8);

#[derive(Debug, AperCodec, UperCodec, Eq, PartialEq)]
#[asn(type = "INTEGER", lb = "0", ub = "4294967295")]
pub struct MME_UE_S1AP_ID(u32);

#[derive(Debug, AperCodec, UperCodec, Eq, PartialEq)]
#[asn(type = "OCTET-STRING")]
pub struct LPPa_PDU(Vec<u8>);

#[derive(Debug, AperCodec, UperCodec, Eq, PartialEq)]
#[asn(type = "INTEGER", lb = "0", ub = "16777215")]
pub struct ENB_UE_S1AP_ID(u32);

#[derive(Debug, AperCodec, UperCodec, Eq, PartialEq)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct GUAMIType(pub u8);
impl GUAMIType {
    pub const NATIVE: u8 = 0u8;
    pub const MAPPED: u8 = 1u8;
}

#[derive(Debug, AperCodec, UperCodec, Eq, PartialEq)]
#[asn(type = "OPEN")]
pub enum UplinkUEAssociatedLPPaTransportprotocolIEsItemvalue {
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 147)]
    LPPa_PDU(LPPa_PDU),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 148)]
    Routing_ID(Routing_ID),
    #[asn(key = 176)]
    Id_GUAMIType(GUAMIType),
}

#[derive(Debug, AperCodec, UperCodec, Eq, PartialEq)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkUEAssociatedLPPaTransportprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UplinkUEAssociatedLPPaTransportprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec, UperCodec, Eq, PartialEq)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct UplinkUEAssociatedLPPaTransportprotocolList(pub Vec<UplinkUEAssociatedLPPaTransportprotocolItem>);

#[derive(Debug, AperCodec, UperCodec, Eq, PartialEq)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UplinkUEAssociatedLPPaTransportprotocolItem {
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UplinkUEAssociatedLPPaTransportprotocolIEsItem>,
}

fn main() {
    eprintln!("Open");
}

#[cfg(test)]
mod tests {
    use asn1_codecs::aper::AperCodec;
    use asn1_codecs::PerCodecData;
    use super::*;

    /// Test whether encoding and then decoding a list of optional extensions that include an open
    /// enumeration type results in the same object.
    ///
    /// This function was added because of a bug that appeared in
    /// #[126](https://github.com/ystero-dev/hampi/issues/126).
    #[test]
    fn test_encode_decode_open_type() {
        let original_test_value = UplinkUEAssociatedLPPaTransportprotocolList(vec![UplinkUEAssociatedLPPaTransportprotocolItem {
            ie_extensions: Some(
                UplinkUEAssociatedLPPaTransportprotocolIEsItem {
                    id: ProtocolIE_ID(176),
                    criticality: Criticality(0),
                    value: UplinkUEAssociatedLPPaTransportprotocolIEsItemvalue::Id_GUAMIType(
                        GUAMIType(0)
                    ),
                }
            ),
        }, UplinkUEAssociatedLPPaTransportprotocolItem {
            ie_extensions: Some(
                UplinkUEAssociatedLPPaTransportprotocolIEsItem {
                    id: ProtocolIE_ID(176),
                    criticality: Criticality(0),
                    value: UplinkUEAssociatedLPPaTransportprotocolIEsItemvalue::Id_GUAMIType(
                        GUAMIType(0)
                    ),
                }
            ),
        }]);
        let mut test_value_encoded = PerCodecData::new_aper();
        original_test_value.aper_encode(&mut test_value_encoded).unwrap();

        let test_value_decoded = UplinkUEAssociatedLPPaTransportprotocolList::aper_decode(&mut test_value_encoded).unwrap();
        assert_eq!(original_test_value, test_value_decoded);
    }
}