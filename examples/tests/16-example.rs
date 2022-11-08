#![allow(dead_code, unreachable_patterns, non_camel_case_types)]

use asn1_codecs::{aper::AperCodec, uper::UperCodec, PerCodecData};

use bitvec::order::Msb0;
use bitvec::vec::BitVec;

#[derive(asn1_codecs_derive :: AperCodec, asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ChildInformation {
    pub name: Name,
    pub date_of_birth: Date,
}

#[derive(asn1_codecs_derive :: AperCodec, asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "VisibleString")]
pub struct Date(pub String);

#[derive(asn1_codecs_derive :: AperCodec, asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER")]
pub struct EmployeeNumber(pub i64);

#[derive(asn1_codecs_derive :: AperCodec, asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Name {
    pub given_name: CharacterString_2,
    pub initial: CharacterString_3,
    pub family_name: CharacterString_4,
}

#[derive(asn1_codecs_derive :: AperCodec, asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct PersonnelRecord {
    pub name: Name,
    pub number: EmployeeNumber,
    pub title: CharacterString_5,
    pub date_of_hire: Date,
    pub name_of_spouse: Name,
    #[asn(optional_idx = 0)]
    pub children: Option<PersonnelRecordChildren>,
}

#[derive(asn1_codecs_derive :: AperCodec, asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "VisibleString")]
pub struct CharacterString_2(pub String);

#[derive(asn1_codecs_derive :: AperCodec, asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "VisibleString")]
pub struct CharacterString_3(pub String);

#[derive(asn1_codecs_derive :: AperCodec, asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "VisibleString")]
pub struct CharacterString_4(pub String);

#[derive(asn1_codecs_derive :: AperCodec, asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "VisibleString")]
pub struct CharacterString_5(pub String);

#[derive(asn1_codecs_derive :: AperCodec, asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF")]
pub struct PersonnelRecordChildren(pub Vec<ChildInformation>);

fn main() {
    let example_bytes_aper = "80044A6F686E015005536D6974680133084469726563746F72083139373130393137044D617279015405536D697468020552616C7068015405536D69746808313935373131313105537573616E0142054A6F6E6573083139353930373137";
    let example_data_aper = hex::decode(example_bytes_aper).unwrap();
    let mut codec_data = PerCodecData::from_slice_aper(&example_data_aper);

    let personnel_record = PersonnelRecord::aper_decode(&mut codec_data);
    eprintln!("personnel_record: {:#?}", personnel_record);

    let example_bytes_uper = "824ADFA3700D005A7B74F4D0026611134F2CB8FA6FE410C5CB762C1CB16E09370F2F20350169EDD3D340102D2C3B386801A80B4F6E9E9A0218B96ADD8B162C4169F5E787700C20595BF765E610C5CB572C1BB16E";
    let example_data_uper = hex::decode(example_bytes_uper).unwrap();
    let mut codec_data = PerCodecData::from_slice_uper(&example_data_uper);

    let personnel_record = PersonnelRecord::uper_decode(&mut codec_data);
    if personnel_record.is_ok() {
        let personnel_record = personnel_record.unwrap();
        eprintln!("personnel_record: {:#?}", personnel_record);

        let mut codec_data = PerCodecData::new_uper();
        let result = personnel_record.uper_encode(&mut codec_data);
        eprintln!("result: {:?}", result);
        if result.is_ok() {
            let encoded_data = codec_data.get_inner().unwrap();
            eprintln!("encoded_data: {}", hex::encode(encoded_data));
        }
    }
}
