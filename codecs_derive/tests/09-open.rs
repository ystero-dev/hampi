#![allow(non_camel_case_types, dead_code)]

use asn_codecs_derive::AperCodec;

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = 0i128, ub = 65535i128)]
pub struct ProtocolIE_ID(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = 0i128, ub = 2i128)]
pub struct Criticality(u8);
impl Criticality {
    const REJECT: u8 = 0u8;
    const IGNORE: u8 = 1u8;
    const NOTIFY: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = 0i128, ub = 255i128)]
pub struct Routing_ID(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = 0i128, ub = 4294967295i128)]
pub struct MME_UE_S1AP_ID(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct LPPa_PDU(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = 0i128, ub = 16777215i128)]
pub struct ENB_UE_S1AP_ID(u32);

#[derive(Debug, AperCodec)]
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
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkUEAssociatedLPPaTransportprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UplinkUEAssociatedLPPaTransportprotocolIEsItemvalue,
}

fn main() {
    eprintln!("Open");
}
