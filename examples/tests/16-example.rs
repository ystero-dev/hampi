#![allow(dead_code, unreachable_patterns, non_camel_case_types)]

use asn1_codecs::aper::{AperCodec, AperCodecData};

#[derive(asn1_codecs_derive :: AperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ChildInformation {
    pub name: Name,
    pub date_of_birth: Date,
}

#[derive(asn1_codecs_derive :: AperCodec, Debug)]
#[asn(type = "VisibleString")]
pub struct Date(pub String);

#[derive(asn1_codecs_derive :: AperCodec, Debug)]
#[asn(type = "INTEGER")]
pub struct EmployeeNumber(pub i128);

#[derive(asn1_codecs_derive :: AperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Name {
    pub given_name: CharacterString_2,
    pub initial: CharacterString_3,
    pub family_name: CharacterString_4,
}

#[derive(asn1_codecs_derive :: AperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PersonnelRecord {
    pub name: Name,
    pub number: EmployeeNumber,
    pub title: CharacterString_5,
    pub date_of_hire: Date,
    pub name_of_spouse: Name,
    pub children: PersonnelRecordChildren,
}

#[derive(asn1_codecs_derive :: AperCodec, Debug)]
#[asn(type = "VisibleString")]
pub struct CharacterString_2(pub String);

#[derive(asn1_codecs_derive :: AperCodec, Debug)]
#[asn(type = "VisibleString")]
pub struct CharacterString_3(pub String);

#[derive(asn1_codecs_derive :: AperCodec, Debug)]
#[asn(type = "VisibleString")]
pub struct CharacterString_4(pub String);

#[derive(asn1_codecs_derive :: AperCodec, Debug)]
#[asn(type = "VisibleString")]
pub struct CharacterString_5(pub String);

#[derive(asn1_codecs_derive :: AperCodec, Debug)]
#[asn(type = "SEQUENCE-OF")]
pub struct PersonnelRecordChildren(pub Vec<ChildInformation>);

fn main() {
    let example_byte_str = "80044A6F686E015005536D6974680133084469726563746F72083139373130393137044D617279015405536D697468020552616C7068015405536D69746808313935373131313105537573616E0142054A6F6E6573083139353930373137";
    let example_data = hex::decode(example_byte_str).unwrap();
    let mut codec_data = AperCodecData::from_slice(&example_data);

    let personnel_record = PersonnelRecord::decode(&mut codec_data);

    eprintln!("personnel_record: {:#?}", personnel_record);
}
