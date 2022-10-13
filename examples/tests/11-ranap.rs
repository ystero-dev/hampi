#![allow(dead_code, unreachable_patterns, non_camel_case_types)]
use bitvec::order::Msb0;
use bitvec::vec::BitVec;

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "OCTET-STRING",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "255"
)]
pub struct APN(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct AccuracyFulfilmentIndicator(pub u8);
impl AccuracyFulfilmentIndicator {
    pub const REQUESTED_ACCURACY_FULFILLED: u8 = 0u8;
    pub const REQUESTED_ACCURACY_NOT_FULFILLED: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct Additional_CSPS_coordination_information {
    #[asn(optional_idx = 0)]
    pub old_lai: Option<LAI>,
    #[asn(optional_idx = 1)]
    pub old_rac: Option<RAC>,
    #[asn(optional_idx = 2)]
    pub nri: Option<BIT_STRING_2>,
    #[asn(optional_idx = 3)]
    pub ue_is_attaching: Option<NULL_3>,
    #[asn(optional_idx = 4)]
    pub ie_extensions: Option<Additional_CSPS_coordination_informationIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct Additional_PositioningDataSet(pub Vec<Additional_PositioningMethodAndUsage>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "1", sz_ub = "1")]
pub struct Additional_PositioningMethodAndUsage(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AllocationOrRetentionPriority {
    pub priority_level: PriorityLevel,
    pub pre_emption_capability: Pre_emptionCapability,
    pub pre_emption_vulnerability: Pre_emptionVulnerability,
    pub queuing_allowed: QueuingAllowed,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AllocationOrRetentionPriorityIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Alt_RAB_Parameter_ExtendedGuaranteedBitrateInf {
    pub alt_extended_guaranteed_bitrate_type: Alt_RAB_Parameter_GuaranteedBitrateType,
    #[asn(optional_idx = 0)]
    pub alt_extended_guaranteed_bitrates: Option<Alt_RAB_Parameter_ExtendedGuaranteedBitrates>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct Alt_RAB_Parameter_ExtendedGuaranteedBitrateList(pub Vec<ExtendedGuaranteedBitrate>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct Alt_RAB_Parameter_ExtendedGuaranteedBitrates(
    pub Vec<Alt_RAB_Parameter_ExtendedGuaranteedBitrateList>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Alt_RAB_Parameter_ExtendedMaxBitrateInf {
    pub alt_extended_max_bitrate_type: Alt_RAB_Parameter_MaxBitrateType,
    #[asn(optional_idx = 0)]
    pub alt_extended_max_bitrates: Option<Alt_RAB_Parameter_ExtendedMaxBitrates>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct Alt_RAB_Parameter_ExtendedMaxBitrateList(pub Vec<ExtendedMaxBitrate>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct Alt_RAB_Parameter_ExtendedMaxBitrates(pub Vec<Alt_RAB_Parameter_ExtendedMaxBitrateList>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Alt_RAB_Parameter_GuaranteedBitrateInf {
    pub alt_guaranteed_bitrate_type: Alt_RAB_Parameter_GuaranteedBitrateType,
    #[asn(optional_idx = 0)]
    pub alt_guaranteed_bitrates: Option<Alt_RAB_Parameter_GuaranteedBitrates>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct Alt_RAB_Parameter_GuaranteedBitrateList(pub Vec<GuaranteedBitrate>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct Alt_RAB_Parameter_GuaranteedBitrateType(pub u8);
impl Alt_RAB_Parameter_GuaranteedBitrateType {
    pub const UNSPECIFIED: u8 = 0u8;
    pub const VALUE_RANGE: u8 = 1u8;
    pub const DISCRETE_VALUES: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct Alt_RAB_Parameter_GuaranteedBitrates(pub Vec<Alt_RAB_Parameter_GuaranteedBitrateList>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Alt_RAB_Parameter_MaxBitrateInf {
    pub alt_max_bitrate_type: Alt_RAB_Parameter_MaxBitrateType,
    #[asn(optional_idx = 0)]
    pub alt_max_bitrates: Option<Alt_RAB_Parameter_MaxBitrates>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct Alt_RAB_Parameter_MaxBitrateList(pub Vec<MaxBitrate>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct Alt_RAB_Parameter_MaxBitrateType(pub u8);
impl Alt_RAB_Parameter_MaxBitrateType {
    pub const UNSPECIFIED: u8 = 0u8;
    pub const VALUE_RANGE: u8 = 1u8;
    pub const DISCRETE_VALUES: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct Alt_RAB_Parameter_MaxBitrates(pub Vec<Alt_RAB_Parameter_MaxBitrateList>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct Alt_RAB_Parameter_SupportedGuaranteedBitrateInf {
    pub alt_supported_guaranteed_bitrate_type: Alt_RAB_Parameter_GuaranteedBitrateType,
    #[asn(optional_idx = 0)]
    pub alt_supported_guaranteed_bitrates: Option<Alt_RAB_Parameter_SupportedGuaranteedBitrates>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<Alt_RAB_Parameter_SupportedGuaranteedBitrateInfIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct Alt_RAB_Parameter_SupportedGuaranteedBitrates(
    pub Vec<SupportedRAB_ParameterBitrateList>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct Alt_RAB_Parameter_SupportedMaxBitrateInf {
    pub alt_supported_max_bitrate_type: Alt_RAB_Parameter_MaxBitrateType,
    #[asn(optional_idx = 0)]
    pub alt_supported_max_bitrates: Option<Alt_RAB_Parameter_SupportedMaxBitrates>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<Alt_RAB_Parameter_SupportedMaxBitrateInfIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct Alt_RAB_Parameter_SupportedMaxBitrates(pub Vec<SupportedRAB_ParameterBitrateList>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct Alt_RAB_Parameters {
    #[asn(optional_idx = 0)]
    pub alt_max_bitrate_inf: Option<Alt_RAB_Parameter_MaxBitrateInf>,
    #[asn(optional_idx = 1)]
    pub alt_guaranteed_bit_rate_inf: Option<Alt_RAB_Parameter_GuaranteedBitrateInf>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<Alt_RAB_ParametersIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct AlternativeRABConfigurationRequest(pub u8);
impl AlternativeRABConfigurationRequest {
    pub const ALTERNATIVE_RAB_CONFIGURATION_REQUESTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum AreaIdentity {
    #[asn(key = 0, extended = false)]
    SAI(SAI),
    #[asn(key = 1, extended = false)]
    GeographicalArea(GeographicalArea),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = true)]
pub enum AreaScopeForUEApplicationLayerMeasurementConfiguration {
    #[asn(key = 0, extended = false)]
    Cellbased(CellBased),
    #[asn(key = 1, extended = false)]
    Labased(LABased),
    #[asn(key = 2, extended = false)]
    Rabased(RABased),
    #[asn(key = 3, extended = false)]
    Plmn_area_based(PLMNBased),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct Ass_RAB_Parameter_ExtendedGuaranteedBitrateList(pub Vec<ExtendedGuaranteedBitrate>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct Ass_RAB_Parameter_ExtendedMaxBitrateList(pub Vec<ExtendedMaxBitrate>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct Ass_RAB_Parameter_GuaranteedBitrateList(pub Vec<GuaranteedBitrate>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct Ass_RAB_Parameter_MaxBitrateList(pub Vec<MaxBitrate>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct Ass_RAB_Parameters {
    #[asn(optional_idx = 0)]
    pub ass_max_bitrate_inf: Option<Ass_RAB_Parameter_MaxBitrateList>,
    #[asn(optional_idx = 1)]
    pub ass_guaranteed_bit_rate_inf: Option<Ass_RAB_Parameter_GuaranteedBitrateList>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<Ass_RAB_ParametersIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct AuthorisedPLMNs(pub Vec<AuthorisedPLMNs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65536"
)]
pub struct AuthorisedSNAs(pub Vec<SNAC>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "30000", ub = "115000")]
pub struct BarometricPressure(pub u32);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct BindingID(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct BroadcastAssistanceDataDecipheringKeys {
    pub ciphering_key_flag: BIT_STRING_4,
    pub current_deciphering_key: BIT_STRING_5,
    pub next_deciphering_key: BIT_STRING_6,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct CGI {
    pub plm_nidentity: PLMNidentity,
    pub lac: LAC,
    pub ci: CI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CGIIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct CI(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CN_DeactivateTrace {
    pub protocol_i_es: CN_DeactivateTraceProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<CN_DeactivateTraceProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct CN_DomainIndicator(pub u8);
impl CN_DomainIndicator {
    pub const CS_DOMAIN: u8 = 0u8;
    pub const PS_DOMAIN: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct CN_ID(pub u16);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CN_InvokeTrace {
    pub protocol_i_es: CN_InvokeTraceProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<CN_InvokeTraceProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CNMBMSLinkingInformation {
    pub joined_mbms_bearer_service_i_es: JoinedMBMSBearerService_IEs,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CNMBMSLinkingInformationIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct CSFB_Information(pub u8);
impl CSFB_Information {
    pub const CSFB: u8 = 0u8;
    pub const CSFB_HIGH_PRIORITY: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "27", sz_ub = "27")]
pub struct CSG_Id(pub BitVec<u8, Msb0>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct CSG_Id_List(pub Vec<CSG_Id>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct CSG_Membership_Status(pub u8);
impl CSG_Membership_Status {
    pub const MEMBER: u8 = 0u8;
    pub const NON_MEMBER: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "5", extensible = true)]
pub enum Cause {
    #[asn(key = 0, extended = false)]
    RadioNetwork(CauseRadioNetwork),
    #[asn(key = 1, extended = false)]
    TransmissionNetwork(CauseTransmissionNetwork),
    #[asn(key = 2, extended = false)]
    NAS(CauseNAS),
    #[asn(key = 3, extended = false)]
    Protocol(CauseProtocol),
    #[asn(key = 4, extended = false)]
    Misc(CauseMisc),
    #[asn(key = 5, extended = false)]
    Non_Standard(CauseNon_Standard),
    #[asn(key = 0, extended = true)]
    RadioNetworkExtension(CauseRadioNetworkExtension),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "113", ub = "128")]
pub struct CauseMisc(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "81", ub = "96")]
pub struct CauseNAS(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "129", ub = "256")]
pub struct CauseNon_Standard(pub u16);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "97", ub = "112")]
pub struct CauseProtocol(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "64")]
pub struct CauseRadioNetwork(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "257", ub = "512")]
pub struct CauseRadioNetworkExtension(pub u16);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "65", ub = "80")]
pub struct CauseTransmissionNetwork(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Cell_Access_Mode(pub u8);
impl Cell_Access_Mode {
    pub const HYBRID: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "100", extensible = true)]
pub struct Cell_Capacity_Class_Value(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "268435455")]
pub struct Cell_Id(pub u32);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellBased {
    pub cell_id_list: CellIdList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellBasedIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct CellIdList(pub Vec<Cell_Id>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct CellLoadInformation {
    pub cell_capacity_class_value: Cell_Capacity_Class_Value,
    pub load_value: LoadValue,
    #[asn(optional_idx = 0)]
    pub rt_load_value: Option<RTLoadValue>,
    #[asn(optional_idx = 1)]
    pub nrt_load_information_value: Option<NRTLoadInformationValue>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<CellLoadInformationIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct CellLoadInformationGroup {
    pub source_cell_id: SourceCellID,
    #[asn(optional_idx = 0)]
    pub uplink_cell_load_information: Option<CellLoadInformation>,
    #[asn(optional_idx = 1)]
    pub downlink_cell_load_information: Option<CellLoadInformation>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<CellLoadInformationGroupIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct CellType(pub u8);
impl CellType {
    pub const MACRO: u8 = 0u8;
    pub const MICRO: u8 = 1u8;
    pub const PICO: u8 = 2u8;
    pub const FEMTO: u8 = 3u8;
}

pub type ChosenEncryptionAlgorithm = EncryptionAlgorithm;

pub type ChosenIntegrityProtectionAlgorithm = IntegrityProtectionAlgorithm;

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING")]
pub struct CivicAddress(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING")]
pub struct ClassmarkInformation2(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING")]
pub struct ClassmarkInformation3(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "7")]
pub struct ClientType(pub u8);
impl ClientType {
    pub const EMERGENCY_SERVICES: u8 = 0u8;
    pub const VALUE_ADDED_SERVICES: u8 = 1u8;
    pub const P_LMN_OPERATOR_SERVICES: u8 = 2u8;
    pub const LAWFUL_INTERCEPT_SERVICES: u8 = 3u8;
    pub const P_LMN_OPERATOR_BROADCAST_SERVICES: u8 = 4u8;
    pub const P_LMN_OPERATOR_O_ET_M: u8 = 5u8;
    pub const P_LMN_OPERATOR_ANONYMOUS_STATISTICS: u8 = 6u8;
    pub const P_LMN_OPERATOR_TARGET_MS_SERVICE_SUPPORT: u8 = 7u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CommonID {
    pub protocol_i_es: CommonIDProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<CommonIDProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct Correlation_ID(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct Criticality(pub u8);
impl Criticality {
    pub const REJECT: u8 = 0u8;
    pub const IGNORE: u8 = 1u8;
    pub const NOTIFY: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct CriticalityDiagnostics {
    #[asn(optional_idx = 0)]
    pub procedure_code: Option<ProcedureCode>,
    #[asn(optional_idx = 1)]
    pub triggering_message: Option<TriggeringMessage>,
    #[asn(optional_idx = 2)]
    pub procedure_criticality: Option<Criticality>,
    #[asn(optional_idx = 3)]
    pub i_es_criticality_diagnostics: Option<CriticalityDiagnostics_IE_List>,
    #[asn(optional_idx = 4)]
    pub ie_extensions: Option<CriticalityDiagnosticsIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct CriticalityDiagnostics_IE_List(pub Vec<CriticalityDiagnostics_IE_List_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1048575")]
pub struct D_RNTI(pub u32);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct DCH_ID(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct DCN_ID(pub u16);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct DL_GTP_PDU_SequenceNumber(pub u16);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct DL_N_PDU_SequenceNumber(pub u16);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "6", ub = "9")]
pub struct DRX_CycleLengthCoefficient(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct DSCH_ID(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct DataPDUType(pub u8);
impl DataPDUType {
    pub const P_D_UTYPE0: u8 = 0u8;
    pub const P_D_UTYPE1: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct DataVolumeList(pub Vec<DataVolumeList_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct DataVolumeReference(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DataVolumeReport {
    pub protocol_i_es: DataVolumeReportProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<DataVolumeReportProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DataVolumeReportRequest {
    pub protocol_i_es: DataVolumeReportRequestProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<DataVolumeReportRequestProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct DataVolumeReportingIndication(pub u8);
impl DataVolumeReportingIndication {
    pub const DO_REPORT: u8 = 0u8;
    pub const DO_NOT_REPORT: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct DeliveryOfErroneousSDU(pub u8);
impl DeliveryOfErroneousSDU {
    pub const YES: u8 = 0u8;
    pub const NO: u8 = 1u8;
    pub const NO_ERROR_DETECTION_CONSIDERATION: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct DeliveryOrder(pub u8);
impl DeliveryOrder {
    pub const DELIVERY_ORDER_REQUESTED: u8 = 0u8;
    pub const DELIVERY_ORDER_NOT_REQUESTED: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 3)]
pub struct DeltaRAListofIdleModeUEs {
    #[asn(optional_idx = 0)]
    pub new_ra_listof_idle_mode_u_es: Option<NewRAListofIdleModeUEs>,
    #[asn(optional_idx = 1)]
    pub ra_listwith_no_idle_mode_u_es_any_more: Option<RAListwithNoIdleModeUEsAnyMore>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<DeltaRAListofIdleModeUEsIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DirectInformationTransfer {
    pub protocol_i_es: DirectInformationTransferProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<DirectInformationTransferProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct DirectReportingIndicator(pub u8);
impl DirectReportingIndicator {
    pub const DIRECT_SAI: u8 = 0u8;
    pub const DIRECT_GEO: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DirectTransfer {
    pub protocol_i_es: DirectTransferProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<DirectTransferProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DirectTransferInformationItem_RANAP_RelocInf {
    pub nas_pdu: NAS_PDU,
    pub sapi: SAPI,
    pub cn_domain_indicator: CN_DomainIndicator,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<DirectTransferInformationItem_RANAP_RelocInfIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "15")]
pub struct DirectTransferInformationList_RANAP_RelocInf(
    pub Vec<DirectTransferInformationList_RANAP_RelocInf_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "7")]
pub struct E_DCH_MAC_d_Flow_ID(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct E_UTRAN_Service_Handover(pub u8);
impl E_UTRAN_Service_Handover {
    pub const HANDOVER_TO_E_UTRAN_SHALL_NOT_BE_PERFORMED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "65536", ub = "262143", extensible = true)]
pub struct EARFCN_Extended(pub u32);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum ENB_ID {
    #[asn(key = 0, extended = false)]
    MacroENB_ID(BIT_STRING_7),
    #[asn(key = 1, extended = false)]
    HomeENB_ID(BIT_STRING_8),
    #[asn(key = 0, extended = true)]
    Short_macroENB_ID(BIT_STRING_9),
    #[asn(key = 1, extended = true)]
    Long_macroENB_ID(BIT_STRING_10),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct EUTRANFrequencies(pub Vec<EUTRANFrequencies_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct EncryptionAlgorithm(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct EncryptionInformation {
    pub permitted_algorithms: PermittedEncryptionAlgorithms,
    pub key: EncryptionKey,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EncryptionInformationIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "BITSTRING",
    sz_extensible = false,
    sz_lb = "128",
    sz_ub = "128"
)]
pub struct EncryptionKey(pub BitVec<u8, Msb0>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct End_Of_CSFB(pub u8);
impl End_Of_CSFB {
    pub const END_OF_CSFB: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EnhancedRelocationCompleteConfirm {
    pub protocol_i_es: EnhancedRelocationCompleteConfirmProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<EnhancedRelocationCompleteConfirmProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EnhancedRelocationCompleteFailure {
    pub protocol_i_es: EnhancedRelocationCompleteFailureProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<EnhancedRelocationCompleteFailureProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EnhancedRelocationCompleteRequest {
    pub protocol_i_es: EnhancedRelocationCompleteRequestProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<EnhancedRelocationCompleteRequestProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EnhancedRelocationCompleteResponse {
    pub protocol_i_es: EnhancedRelocationCompleteResponseProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<EnhancedRelocationCompleteResponseProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = true)]
pub enum EquipmentsToBeTraced {
    #[asn(key = 0, extended = false)]
    IMEIlist(IMEIList),
    #[asn(key = 1, extended = false)]
    IMEISVlist(IMEISVList),
    #[asn(key = 2, extended = false)]
    IMEIgroup(IMEIGroup),
    #[asn(key = 3, extended = false)]
    IMEISVgroup(IMEISVGroup),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ErrorIndication {
    pub protocol_i_es: ErrorIndicationProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<ErrorIndicationProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct Event(pub u8);
impl Event {
    pub const STOP_CHANGE_OF_SERVICE_AREA: u8 = 0u8;
    pub const DIRECT: u8 = 1u8;
    pub const CHANGE_OF_SERVICEAREA: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct Event1F_Parameters {
    pub measurement_quantity: MeasurementQuantity,
    pub threshold: INTEGER_12,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct Event1I_Parameters {
    pub threshold: INTEGER_13,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "16000001", ub = "256000000")]
pub struct ExtendedGuaranteedBitrate(pub u32);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "16000001", ub = "256000000")]
pub struct ExtendedMaxBitrate(pub u32);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "4096", ub = "65535")]
pub struct ExtendedRNC_ID(pub u16);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ForwardSRNS_Context {
    pub protocol_i_es: ForwardSRNS_ContextProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<ForwardSRNS_ContextProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ForwardingIndication(pub u8);
impl ForwardingIndication {
    pub const FORWARDING_ADMITTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct FrameSequenceNumber(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct FrequenceLayerConvergenceFlag(pub u8);
impl FrequenceLayerConvergenceFlag {
    pub const NO_FLC_FLAG: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GA_AltitudeAndDirection {
    pub direction_of_altitude: ENUMERATED_14,
    pub altitude: INTEGER_15,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GA_EllipsoidArc {
    pub geographical_coordinates: GeographicalCoordinates,
    pub inner_radius: INTEGER_16,
    pub uncertainty_radius: INTEGER_17,
    pub offset_angle: INTEGER_18,
    pub included_angle: INTEGER_19,
    pub confidence: INTEGER_20,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GA_EllipsoidArcIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GA_Point {
    pub geographical_coordinates: GeographicalCoordinates,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GA_PointIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GA_PointWithAltitude {
    pub geographical_coordinates: GeographicalCoordinates,
    pub altitude_and_direction: GA_AltitudeAndDirection,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GA_PointWithAltitudeIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GA_PointWithAltitudeAndUncertaintyEllipsoid {
    pub geographical_coordinates: GeographicalCoordinates,
    pub altitude_and_direction: GA_AltitudeAndDirection,
    pub uncertainty_ellipse: GA_UncertaintyEllipse,
    pub uncertainty_altitude: INTEGER_21,
    pub confidence: INTEGER_22,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GA_PointWithAltitudeAndUncertaintyEllipsoidIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct GA_PointWithUnCertainty {
    pub geographical_coordinates: GeographicalCoordinates,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GA_PointWithUnCertaintyIE_Extensions>,
    pub uncertainty_code: INTEGER_23,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GA_PointWithUnCertaintyEllipse {
    pub geographical_coordinates: GeographicalCoordinates,
    pub uncertainty_ellipse: GA_UncertaintyEllipse,
    pub confidence: INTEGER_24,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GA_PointWithUnCertaintyEllipseIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "15")]
pub struct GA_Polygon(pub Vec<GA_Polygon_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GA_UncertaintyEllipse {
    pub uncertainty_semi_major: INTEGER_25,
    pub uncertainty_semi_minor: INTEGER_26,
    pub orientation_of_major_axis: INTEGER_27,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "9")]
pub struct GANSS_PositioningDataSet(pub Vec<GANSS_PositioningMethodAndUsage>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "1", sz_ub = "1")]
pub struct GANSS_PositioningMethodAndUsage(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING")]
pub struct GERAN_BSC_Container(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct GERAN_Cell_ID {
    pub lai: LAI,
    pub rac: RAC,
    pub ci: CI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GERAN_Cell_IDIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING")]
pub struct GERAN_Classmark(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct GERAN_Iumode_RAB_Failed_RABAssgntResponse_Item {
    pub rab_id: RAB_ID,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub geran_classmark: Option<GERAN_Classmark>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<GERAN_Iumode_RAB_Failed_RABAssgntResponse_ItemIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct GERAN_Iumode_RAB_FailedList_RABAssgntResponse(
    pub Vec<GERAN_Iumode_RAB_FailedList_RABAssgntResponse_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct GTP_TEI(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = true)]
pub enum GeographicalArea {
    #[asn(key = 0, extended = false)]
    Point(GA_Point),
    #[asn(key = 1, extended = false)]
    PointWithUnCertainty(GA_PointWithUnCertainty),
    #[asn(key = 2, extended = false)]
    Polygon(GA_Polygon),
    #[asn(key = 0, extended = true)]
    PointWithUncertaintyEllipse(GA_PointWithUnCertaintyEllipse),
    #[asn(key = 1, extended = true)]
    PointWithAltitude(GA_PointWithAltitude),
    #[asn(key = 2, extended = true)]
    PointWithAltitudeAndUncertaintyEllipsoid(GA_PointWithAltitudeAndUncertaintyEllipsoid),
    #[asn(key = 3, extended = true)]
    EllipsoidArc(GA_EllipsoidArc),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GeographicalCoordinates {
    pub latitude_sign: ENUMERATED_28,
    pub latitude: INTEGER_29,
    pub longitude: INTEGER_30,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GeographicalCoordinatesIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalCN_ID {
    pub plm_nidentity: PLMNidentity,
    pub cn_id: CN_ID,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalRNC_ID {
    pub plm_nidentity: PLMNidentity,
    pub rnc_id: RNC_ID,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "16000000")]
pub struct GuaranteedBitrate(pub u32);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "7")]
pub struct HS_DSCH_MAC_d_Flow_ID(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct HigherBitratesThan16MbpsFlag(pub u8);
impl HigherBitratesThan16MbpsFlag {
    pub const ALLOWED: u8 = 0u8;
    pub const NOT_ALLOWED: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HorizontalSpeedAndBearing {
    pub bearing: INTEGER_31,
    pub horizontal_speed: INTEGER_32,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct HorizontalVelocity {
    pub horizontal_speed_and_bearing: HorizontalSpeedAndBearing,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<HorizontalVelocityIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct HorizontalVelocityWithUncertainty {
    pub horizontal_speed_and_bearing: HorizontalSpeedAndBearing,
    pub uncertainty_speed: INTEGER_33,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<HorizontalVelocityWithUncertaintyIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct HorizontalWithVerticalVelocity {
    pub horizontal_speed_and_bearing: HorizontalSpeedAndBearing,
    pub veritcal_velocity: VerticalVelocity,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<HorizontalWithVerticalVelocityIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct HorizontalWithVerticalVelocityAndUncertainty {
    pub horizontal_speed_and_bearing: HorizontalSpeedAndBearing,
    pub veritcal_velocity: VerticalVelocity,
    pub horizontal_uncertainty_speed: INTEGER_34,
    pub vertical_uncertainty_speed: INTEGER_35,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<HorizontalWithVerticalVelocityAndUncertaintyIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct IMEI(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct IMEIGroup {
    pub imei: IMEI,
    pub imei_mask: BIT_STRING_36,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<IMEIGroupIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct IMEIList(pub Vec<IMEI>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct IMEISV(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct IMEISVGroup {
    pub imeisv: IMEISV,
    pub imeisv_mask: BIT_STRING_37,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<IMEISVGroupIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct IMEISVList(pub Vec<IMEISV>);

pub type IMSI = TBCD_STRING;

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "OCTET-STRING",
    sz_extensible = false,
    sz_lb = "4",
    sz_ub = "16"
)]
pub struct IPMulticastAddress(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 3)]
pub struct IRAT_Measurement_Configuration {
    #[asn(optional_idx = 0)]
    pub rsrp: Option<INTEGER_38>,
    #[asn(optional_idx = 1)]
    pub rsrq: Option<INTEGER_39>,
    pub ira_tmeasurement_parameters: IRATmeasurementParameters,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<IRAT_Measurement_ConfigurationIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct IRATmeasurementParameters {
    pub measurement_duration: INTEGER_40,
    #[asn(optional_idx = 0)]
    pub eutran_frequencies: Option<EUTRANFrequencies>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<IRATmeasurementParametersIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct ImmediateMDT {
    pub measurements_to_activate: MeasurementsToActivate,
    #[asn(optional_idx = 0)]
    pub m1report: Option<M1Report>,
    #[asn(optional_idx = 1)]
    pub m2report: Option<M2Report>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<ImmediateMDTIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct IncludeVelocity(pub u8);
impl IncludeVelocity {
    pub const REQUESTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1048575")]
pub struct InformationExchangeID(pub u32);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct InformationExchangeType(pub u8);
impl InformationExchangeType {
    pub const TRANSFER: u8 = 0u8;
    pub const REQUEST: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum InformationRequestType {
    #[asn(key = 0, extended = false)]
    MBMSIPMulticastAddressandAPNRequest(MBMSIPMulticastAddressandAPNRequest),
    #[asn(key = 1, extended = false)]
    PermanentNAS_UE_ID(PermanentNAS_UE_ID),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum InformationRequested {
    #[asn(key = 0, extended = false)]
    RequestedMBMSIPMulticastAddressandAPNRequest(RequestedMBMSIPMulticastAddressandAPNRequest),
    #[asn(key = 1, extended = false)]
    RequestedMulticastServiceList(RequestedMulticastServiceList),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct InformationTransferConfirmation {
    pub protocol_i_es: InformationTransferConfirmationProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<InformationTransferConfirmationProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct InformationTransferFailure {
    pub protocol_i_es: InformationTransferFailureProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<InformationTransferFailureProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1048575")]
pub struct InformationTransferID(pub u32);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct InformationTransferIndication {
    pub protocol_i_es: InformationTransferIndicationProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<InformationTransferIndicationProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "0", extensible = true)]
pub enum InformationTransferType {
    #[asn(key = 0, extended = false)]
    RNCTraceInformation(RNCTraceInformation),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct InitialUE_Message {
    pub protocol_i_es: InitialUE_MessageProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<InitialUE_MessageProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InitiatingMessage {
    #[asn(key_field = true)]
    pub procedure_code: ProcedureCode,
    pub criticality: Criticality,
    pub value: InitiatingMessageValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct IntegrityProtectionAlgorithm(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct IntegrityProtectionInformation {
    pub permitted_algorithms: PermittedIntegrityProtectionAlgorithms,
    pub key: IntegrityProtectionKey,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<IntegrityProtectionInformationIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "BITSTRING",
    sz_extensible = false,
    sz_lb = "128",
    sz_ub = "128"
)]
pub struct IntegrityProtectionKey(pub BitVec<u8, Msb0>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct InterSystemInformation_TransparentContainer {
    #[asn(optional_idx = 0)]
    pub downlink_cell_load_information: Option<CellLoadInformation>,
    #[asn(optional_idx = 1)]
    pub uplink_cell_load_information: Option<CellLoadInformation>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<InterSystemInformation_TransparentContainerIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "0", extensible = true)]
pub enum InterSystemInformationTransferType {
    #[asn(key = 0, extended = false)]
    RIM_Transfer(RIM_Transfer),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct InterfacesToTraceItem {
    pub interface: ENUMERATED_41,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<InterfacesToTraceItemIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Iu_ReleaseCommand {
    pub protocol_i_es: Iu_ReleaseCommandProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<Iu_ReleaseCommandProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Iu_ReleaseComplete {
    pub protocol_i_es: Iu_ReleaseCompleteProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<Iu_ReleaseCompleteProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Iu_ReleaseRequest {
    pub protocol_i_es: Iu_ReleaseRequestProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<Iu_ReleaseRequestProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "24", sz_ub = "24")]
pub struct IuSignallingConnectionIdentifier(pub BitVec<u8, Msb0>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum IuTransportAssociation {
    #[asn(key = 0, extended = false)]
    GTP_TEI(GTP_TEI),
    #[asn(key = 1, extended = false)]
    BindingID(BindingID),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "128"
)]
pub struct JoinedMBMSBearerService_IEs(pub Vec<JoinedMBMSBearerService_IEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct KeyStatus(pub u8);
impl KeyStatus {
    pub const OLD: u8 = 0u8;
    pub const NEW: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING")]
pub struct L3_Information(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65536"
)]
pub struct LA_LIST(pub Vec<LA_LIST_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LABased {
    pub lai_list: LAI_List,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<LABasedIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct LAC(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct LAI {
    pub plm_nidentity: PLMNidentity,
    pub lac: LAC,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<LAIIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct LAI_List(pub Vec<LAI>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65536"
)]
pub struct LAListofIdleModeUEs(pub Vec<LAI>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "OCTET-STRING",
    sz_extensible = false,
    sz_lb = "32",
    sz_ub = "256"
)]
pub struct LHN_ID(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LastKnownServiceArea {
    pub sai: SAI,
    pub age_of_sai: INTEGER_42,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<LastKnownServiceAreaIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LastVisitedUTRANCell_Item {
    pub utran_cell_id: UTRAN_CellID,
    pub cell_type: CellType,
    pub time_ue_stayed_in_cell: Time_UE_StayedInCell,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<LastVisitedUTRANCell_ItemIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "128"
)]
pub struct LeftMBMSBearerService_IEs(pub Vec<LeftMBMSBearerService_IEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct Links_to_log(pub u8);
impl Links_to_log {
    pub const UPLINK: u8 = 0u8;
    pub const DOWNLINK: u8 = 1u8;
    pub const BOTH_UPLINK_AND_DOWNLINK: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65536"
)]
pub struct ListOF_SNAs(pub Vec<SNAC>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct ListOfInterfacesToTrace(pub Vec<InterfacesToTraceItem>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct LoadValue(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LocationRelatedDataFailure {
    pub protocol_i_es: LocationRelatedDataFailureProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<LocationRelatedDataFailureProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LocationRelatedDataRequest {
    pub protocol_i_es: LocationRelatedDataRequestProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<LocationRelatedDataRequestProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LocationRelatedDataRequestType {
    pub requested_location_related_data_type: RequestedLocationRelatedDataType,
    #[asn(optional_idx = 0)]
    pub requested_gps_assistance_data: Option<RequestedGPSAssistanceData>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct LocationRelatedDataRequestTypeSpecificToGERANIuMode(pub u8);
impl LocationRelatedDataRequestTypeSpecificToGERANIuMode {
    pub const DECIPHERING_KEYS_EOTD: u8 = 0u8;
    pub const DEDICATED_MOBILE_ASSISTED_EOTD_ASSISTANCE_DATA: u8 = 1u8;
    pub const DEDICATED_MOBILE_BASED_EOTD_ASSISTANCE_DATA: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LocationRelatedDataResponse {
    pub protocol_i_es: LocationRelatedDataResponseProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<LocationRelatedDataResponseProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LocationReport {
    pub protocol_i_es: LocationReportProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<LocationReportProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LocationReportingControl {
    pub protocol_i_es: LocationReportingControlProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<LocationReportingControlProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 12)]
pub struct LocationReportingTransferInformation {
    #[asn(optional_idx = 0)]
    pub report_change_of_sai: Option<ReportChangeOfSAI>,
    #[asn(optional_idx = 1)]
    pub periodic_reporting_indicator: Option<PeriodicReportingIndicator>,
    #[asn(optional_idx = 2)]
    pub direct_reporting_indicator: Option<DirectReportingIndicator>,
    #[asn(optional_idx = 3)]
    pub vertical_accuracy_code: Option<VerticalAccuracyCode>,
    #[asn(optional_idx = 4)]
    pub positioning_priority_change_sai: Option<PositioningPriority>,
    #[asn(optional_idx = 5)]
    pub positioning_priority_direct: Option<PositioningPriority>,
    #[asn(optional_idx = 6)]
    pub client_type_periodic: Option<ClientType>,
    #[asn(optional_idx = 7)]
    pub client_type_direct: Option<ClientType>,
    #[asn(optional_idx = 8)]
    pub response_time: Option<ResponseTime>,
    #[asn(optional_idx = 9)]
    pub include_velocity: Option<IncludeVelocity>,
    #[asn(optional_idx = 10)]
    pub periodic_location_info: Option<PeriodicLocationInfo>,
    #[asn(optional_idx = 11)]
    pub ie_extensions: Option<LocationReportingTransferInformationIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LoggedMDT {
    pub logging_interval: LoggingInterval,
    pub logging_duration: LoggingDuration,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<LoggedMDTIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "5")]
pub struct LoggingDuration(pub u8);
impl LoggingDuration {
    pub const MIN10: u8 = 0u8;
    pub const MIN20: u8 = 1u8;
    pub const MIN40: u8 = 2u8;
    pub const MIN60: u8 = 3u8;
    pub const MIN90: u8 = 4u8;
    pub const MIN120: u8 = 5u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "7")]
pub struct LoggingInterval(pub u8);
impl LoggingInterval {
    pub const S1D28: u8 = 0u8;
    pub const S2D56: u8 = 1u8;
    pub const S5D12: u8 = 2u8;
    pub const S10D24: u8 = 3u8;
    pub const S20D48: u8 = 4u8;
    pub const S30D72: u8 = 5u8;
    pub const S40D96: u8 = 6u8;
    pub const S61D44: u8 = 7u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum M1Report {
    #[asn(key = 0, extended = false)]
    Periodic(MDT_Report_Parameters),
    #[asn(key = 1, extended = false)]
    Event1F(Event1F_Parameters),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum M2Report {
    #[asn(key = 0, extended = false)]
    Periodic(MDT_Report_Parameters),
    #[asn(key = 1, extended = false)]
    Event1I(Event1I_Parameters),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct M4_Collection_Parameters {
    pub m4_period: M4_Period,
    #[asn(optional_idx = 0)]
    pub m4_threshold: Option<M4_Threshold>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<M4_Collection_ParametersIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "7")]
pub struct M4_Period(pub u8);
impl M4_Period {
    pub const MS100: u8 = 0u8;
    pub const MS250: u8 = 1u8;
    pub const MS500: u8 = 2u8;
    pub const MS1000: u8 = 3u8;
    pub const MS2000: u8 = 4u8;
    pub const MS3000: u8 = 5u8;
    pub const MS4000: u8 = 6u8;
    pub const MS6000: u8 = 7u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "31")]
pub struct M4_Threshold(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum M4Report {
    #[asn(key = 0, extended = false)]
    All(NULL_43),
    #[asn(key = 1, extended = false)]
    M4_collection_parameters(M4_Collection_Parameters),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "7")]
pub struct M5_Period(pub u8);
impl M5_Period {
    pub const MS100: u8 = 0u8;
    pub const MS250: u8 = 1u8;
    pub const MS500: u8 = 2u8;
    pub const MS1000: u8 = 3u8;
    pub const MS2000: u8 = 4u8;
    pub const MS3000: u8 = 5u8;
    pub const MS4000: u8 = 6u8;
    pub const MS6000: u8 = 7u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum M5Report {
    #[asn(key = 0, extended = false)]
    When_available(NULL_44),
    #[asn(key = 1, extended = false)]
    M5_period(M5_Period),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "12")]
pub struct M6_Period(pub u8);
impl M6_Period {
    pub const MS1000: u8 = 0u8;
    pub const MS2000: u8 = 1u8;
    pub const MS3000: u8 = 2u8;
    pub const MS4000: u8 = 3u8;
    pub const MS6000: u8 = 4u8;
    pub const MS8000: u8 = 5u8;
    pub const MS12000: u8 = 6u8;
    pub const MS16000: u8 = 7u8;
    pub const MS20000: u8 = 8u8;
    pub const MS24000: u8 = 9u8;
    pub const MS28000: u8 = 10u8;
    pub const MS32000: u8 = 11u8;
    pub const MS64000: u8 = 12u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct M6Report {
    pub m6_period: M6_Period,
    pub m6_links_to_log: Links_to_log,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<M6ReportIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "12")]
pub struct M7_Period(pub u8);
impl M7_Period {
    pub const MS1000: u8 = 0u8;
    pub const MS2000: u8 = 1u8;
    pub const MS3000: u8 = 2u8;
    pub const MS4000: u8 = 3u8;
    pub const MS6000: u8 = 4u8;
    pub const MS8000: u8 = 5u8;
    pub const MS12000: u8 = 6u8;
    pub const MS16000: u8 = 7u8;
    pub const MS20000: u8 = 8u8;
    pub const MS24000: u8 = 9u8;
    pub const MS28000: u8 = 10u8;
    pub const MS32000: u8 = 11u8;
    pub const MS64000: u8 = 12u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct M7Report {
    pub m7_period: M7_Period,
    pub m7_links_to_log: Links_to_log,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<M7ReportIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct MBMS_PTP_RAB_ID(pub BitVec<u8, Msb0>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct MBMSBearerServiceType(pub u8);
impl MBMSBearerServiceType {
    pub const MULTICAST: u8 = 0u8;
    pub const BROADCAST: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct MBMSCNDe_Registration(pub u8);
impl MBMSCNDe_Registration {
    pub const NORMALSESSIONSTOP: u8 = 0u8;
    pub const DEREGISTER: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSCNDe_RegistrationRequest {
    pub protocol_i_es: MBMSCNDe_RegistrationRequestProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSCNDe_RegistrationRequestProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSCNDe_RegistrationResponse {
    pub protocol_i_es: MBMSCNDe_RegistrationResponseProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSCNDe_RegistrationResponseProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct MBMSCountingInformation(pub u8);
impl MBMSCountingInformation {
    pub const COUNTING: u8 = 0u8;
    pub const NOTCOUNTING: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct MBMSHCIndicator(pub u8);
impl MBMSHCIndicator {
    pub const UNCOMPRESSED_HEADER: u8 = 0u8;
    pub const COMPRESSED_HEADER: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "512"
)]
pub struct MBMSIPMulticastAddressandAPNRequest(pub Vec<TMGI>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSIPMulticastAddressandAPNlist {
    pub tmgi: TMGI,
    pub ip_multicast_address: IPMulticastAddress,
    pub apn: APN,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<MBMSIPMulticastAddressandAPNlistIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct MBMSLinkingInformation(pub u8);
impl MBMSLinkingInformation {
    pub const U_E_HAS_JOINED_MULTICAST_SERVICES: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSRABEstablishmentIndication {
    pub protocol_i_es: MBMSRABEstablishmentIndicationProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSRABEstablishmentIndicationProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSRABRelease {
    pub protocol_i_es: MBMSRABReleaseProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSRABReleaseProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSRABReleaseFailure {
    pub protocol_i_es: MBMSRABReleaseFailureProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSRABReleaseFailureProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSRABReleaseRequest {
    pub protocol_i_es: MBMSRABReleaseRequestProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSRABReleaseRequestProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSRegistrationFailure {
    pub protocol_i_es: MBMSRegistrationFailureProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSRegistrationFailureProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSRegistrationRequest {
    pub protocol_i_es: MBMSRegistrationRequestProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSRegistrationRequestProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct MBMSRegistrationRequestType(pub u8);
impl MBMSRegistrationRequestType {
    pub const REGISTER: u8 = 0u8;
    pub const DEREGISTER: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSRegistrationResponse {
    pub protocol_i_es: MBMSRegistrationResponseProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSRegistrationResponseProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING")]
pub struct MBMSServiceArea(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "3", sz_ub = "3")]
pub struct MBMSSessionDuration(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "1", sz_ub = "1")]
pub struct MBMSSessionIdentity(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "1", sz_ub = "1")]
pub struct MBMSSessionRepetitionNumber(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSSessionStart {
    pub protocol_i_es: MBMSSessionStartProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSSessionStartProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSSessionStartFailure {
    pub protocol_i_es: MBMSSessionStartFailureProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSSessionStartFailureProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSSessionStartResponse {
    pub protocol_i_es: MBMSSessionStartResponseProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSSessionStartResponseProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSSessionStop {
    pub protocol_i_es: MBMSSessionStopProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSSessionStopProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSSessionStopResponse {
    pub protocol_i_es: MBMSSessionStopResponseProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSSessionStopResponseProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSSessionUpdate {
    pub protocol_i_es: MBMSSessionUpdateProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSSessionUpdateProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSSessionUpdateFailure {
    pub protocol_i_es: MBMSSessionUpdateFailureProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSSessionUpdateFailureProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSSessionUpdateResponse {
    pub protocol_i_es: MBMSSessionUpdateResponseProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSSessionUpdateResponseProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSSynchronisationInformation {
    pub mbmshc_indicator: MBMSHCIndicator,
    pub ip_multicast_address: IPMulticastAddress,
    pub gtpdlteid: GTP_TEI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<MBMSSynchronisationInformationIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSUELinkingRequest {
    pub protocol_i_es: MBMSUELinkingRequestProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSUELinkingRequestProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSUELinkingResponse {
    pub protocol_i_es: MBMSUELinkingResponseProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSUELinkingResponseProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct MDT_Activation(pub u8);
impl MDT_Activation {
    pub const IMMEDIATE_MD_TONLY: u8 = 0u8;
    pub const LOGGED_MD_TONLY: u8 = 1u8;
    pub const IMMEDIATE_MD_TAND_TRACE: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MDT_Configuration {
    pub mdt_activation: MDT_Activation,
    pub mdt_area_scope: MDTAreaScope,
    pub mdt_mode: MDTMode,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<MDT_ConfigurationIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct MDT_PLMN_List(pub Vec<PLMNidentity>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct MDT_Report_Parameters {
    pub report_interval: ReportInterval,
    pub report_amount: ReportAmount,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = true)]
pub enum MDTAreaScope {
    #[asn(key = 0, extended = false)]
    Cellbased(CellBased),
    #[asn(key = 1, extended = false)]
    Labased(LABased),
    #[asn(key = 2, extended = false)]
    Rabased(RABased),
    #[asn(key = 3, extended = false)]
    Plmn_area_based(NULL_45),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum MDTMode {
    #[asn(key = 0, extended = false)]
    ImmediateMDT(ImmediateMDT),
    #[asn(key = 1, extended = false)]
    LoggedMDT(LoggedMDT),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "1", sz_ub = "9")]
pub struct MSISDN(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Management_Based_MDT_Allowed(pub u8);
impl Management_Based_MDT_Allowed {
    pub const ALLOWED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "16000000")]
pub struct MaxBitrate(pub u32);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "32768")]
pub struct MaxSDU_Size(pub u16);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "5")]
pub struct MeasBand(pub u8);
impl MeasBand {
    pub const V6: u8 = 0u8;
    pub const V15: u8 = 1u8;
    pub const V25: u8 = 2u8;
    pub const V50: u8 = 3u8;
    pub const V75: u8 = 4u8;
    pub const V100: u8 = 5u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct MeasurementQuantity(pub u8);
impl MeasurementQuantity {
    pub const CPICH_EC_NO: u8 = 0u8;
    pub const CPICH_RSCP: u8 = 1u8;
    pub const PATHLOSS: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct MeasurementsToActivate(pub BitVec<u8, Msb0>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct MessageStructure(pub Vec<MessageStructure_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING")]
pub struct NAS_PDU(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct NAS_SequenceNumber(pub BitVec<u8, Msb0>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct NAS_SynchronisationIndicator(pub BitVec<u8, Msb0>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "3")]
pub struct NRTLoadInformationValue(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING")]
pub struct NewBSS_To_OldBSS_Information(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65536"
)]
pub struct NewRAListofIdleModeUEs(pub Vec<RAC>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct NonSearchingIndication(pub u8);
impl NonSearchingIndication {
    pub const NON_SEARCHING: u8 = 0u8;
    pub const SEARCHING: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct NotEmptyRAListofIdleModeUEs {
    pub r_aof_idle_mode_u_es: RAofIdleModeUEs,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<NotEmptyRAListofIdleModeUEsIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "10", sz_ub = "10")]
pub struct Null_NRI(pub BitVec<u8, Msb0>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "2")]
pub struct NumberOfIuInstances(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "16")]
pub struct NumberOfSteps(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "OCTET-STRING",
    sz_extensible = false,
    sz_lb = "3",
    sz_ub = "22"
)]
pub struct OMC_ID(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Offload_RAB_Parameters {
    pub access_point_name: Offload_RAB_Parameters_APN,
    pub charging_characteristics: Offload_RAB_Parameters_ChargingCharacteristics,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<Offload_RAB_ParametersIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "OCTET-STRING",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "255"
)]
pub struct Offload_RAB_Parameters_APN(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct Offload_RAB_Parameters_ChargingCharacteristics(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING")]
pub struct OldBSS_ToNewBSS_Information(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Out_Of_UTRAN(pub u8);
impl Out_Of_UTRAN {
    pub const CELL_RESELECTION_TO_EUTRAN: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Outcome {
    #[asn(key_field = true)]
    pub procedure_code: ProcedureCode,
    pub criticality: Criticality,
    pub value: OutcomeValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Overload {
    pub protocol_i_es: OverloadProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<OverloadProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct P_TMSI(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "4")]
pub struct PDP_Type(pub u8);
impl PDP_Type {
    pub const EMPTY: u8 = 0u8;
    pub const PPP: u8 = 1u8;
    pub const OSP_IHOSS: u8 = 2u8;
    pub const IPV4: u8 = 3u8;
    pub const IPV6: u8 = 4u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct PDP_Type_extension(pub u8);
impl PDP_Type_extension {
    pub const IPV4_AND_IPV6: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct PDP_TypeInformation(pub Vec<PDP_Type>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct PDP_TypeInformation_extension(pub Vec<PDP_Type_extension>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "3")]
pub struct PDUType14FrameSequenceNumber(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PLMNBased {
    pub plmn_list: PLMNList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PLMNBasedIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct PLMNList(pub Vec<PLMNidentity>);

pub type PLMNidentity = TBCD_STRING;

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct PLMNs_in_shared_network(pub Vec<PLMNs_in_shared_network_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Paging {
    pub protocol_i_es: PagingProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<PagingProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum PagingAreaID {
    #[asn(key = 0, extended = false)]
    LAI(LAI),
    #[asn(key = 1, extended = false)]
    RAI(RAI),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "4")]
pub struct PagingCause(pub u8);
impl PagingCause {
    pub const TERMINATING_CONVERSATIONAL_CALL: u8 = 0u8;
    pub const TERMINATING_STREAMING_CALL: u8 = 1u8;
    pub const TERMINATING_INTERACTIVE_CALL: u8 = 2u8;
    pub const TERMINATING_BACKGROUND_CALL: u8 = 3u8;
    pub const TERMINATING_LOW_PRIORITY_SIGNALLING: u8 = 4u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PeriodicLocationInfo {
    pub reporting_amount: INTEGER_46,
    pub reporting_interval: INTEGER_47,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PeriodicLocationInfoIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct PeriodicReportingIndicator(pub u8);
impl PeriodicReportingIndicator {
    pub const PERIODIC_SAI: u8 = 0u8;
    pub const PERIODIC_GEO: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "0", extensible = true)]
pub enum PermanentNAS_UE_ID {
    #[asn(key = 0, extended = false)]
    IMSI(IMSI),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct PermittedEncryptionAlgorithms(pub Vec<EncryptionAlgorithm>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct PermittedIntegrityProtectionAlgorithms(pub Vec<IntegrityProtectionAlgorithm>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct Port_Number(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PositionData {
    pub positioning_data_discriminator: PositioningDataDiscriminator,
    #[asn(optional_idx = 0)]
    pub positioning_data_set: Option<PositioningDataSet>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PositionDataIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING")]
pub struct PositionDataSpecificToGERANIuMode(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct PositioningDataDiscriminator(pub BitVec<u8, Msb0>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "9")]
pub struct PositioningDataSet(pub Vec<PositioningMethodAndUsage>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "1", sz_ub = "1")]
pub struct PositioningMethodAndUsage(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct PositioningPriority(pub u8);
impl PositioningPriority {
    pub const HIGH_PRIORITY: u8 = 0u8;
    pub const NORMAL_PRIORITY: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct PowerSavingIndicator(pub u8);
impl PowerSavingIndicator {
    pub const PSM_CONFIGURED: u8 = 0u8;
    pub const E_DRX_CONFIGURED: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct Pre_emptionCapability(pub u8);
impl Pre_emptionCapability {
    pub const SHALL_NOT_TRIGGER_PRE_EMPTION: u8 = 0u8;
    pub const MAY_TRIGGER_PRE_EMPTION: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct Pre_emptionVulnerability(pub u8);
impl Pre_emptionVulnerability {
    pub const NOT_PRE_EMPTABLE: u8 = 0u8;
    pub const PRE_EMPTABLE: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct Presence(pub u8);
impl Presence {
    pub const OPTIONAL: u8 = 0u8;
    pub const CONDITIONAL: u8 = 1u8;
    pub const MANDATORY: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct Priority_Class_Indicator(pub BitVec<u8, Msb0>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct PriorityLevel(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum PrivateIE_ID {
    #[asn(key = 0, extended = false)]
    Local(INTEGER_48),
    #[asn(key = 1, extended = false)]
    Global(OBJECT_IDENTIFIER_49),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PrivateMessage {
    pub private_i_es: PrivateMessagePrivateIEs,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct ProcedureCode(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct ProtocolExtensionID(pub u16);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct ProtocolIE_ID(pub u16);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "0", extensible = true)]
pub enum ProvidedData {
    #[asn(key = 0, extended = false)]
    Shared_network_information(Shared_Network_Information),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct QueuingAllowed(pub u8);
impl QueuingAllowed {
    pub const QUEUEING_NOT_ALLOWED: u8 = 0u8;
    pub const QUEUEING_ALLOWED: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_AssignmentRequest {
    pub protocol_i_es: RAB_AssignmentRequestProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RAB_AssignmentRequestProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_AssignmentResponse {
    pub protocol_i_es: RAB_AssignmentResponseProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RAB_AssignmentResponseProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct RAB_AsymmetryIndicator(pub u8);
impl RAB_AsymmetryIndicator {
    pub const SYMMETRIC_BIDIRECTIONAL: u8 = 0u8;
    pub const ASYMMETRIC_UNIDIRECTIONAL_DOWNLINK: u8 = 1u8;
    pub const ASYMMETRIC_UNIDIRECTIONAL_UPLINK: u8 = 2u8;
    pub const ASYMMETRIC_BIDIRECTIONAL: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_ContextFailedtoTransferList(pub Vec<RAB_ContextFailedtoTransferList_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct RAB_ContextItem {
    pub rab_id: RAB_ID,
    #[asn(optional_idx = 0)]
    pub dl_gtp_pdu_sequence_number: Option<DL_GTP_PDU_SequenceNumber>,
    #[asn(optional_idx = 1)]
    pub ul_gtp_pdu_sequence_number: Option<UL_GTP_PDU_SequenceNumber>,
    #[asn(optional_idx = 2)]
    pub dl_n_pdu_sequence_number: Option<DL_N_PDU_SequenceNumber>,
    #[asn(optional_idx = 3)]
    pub ul_n_pdu_sequence_number: Option<UL_N_PDU_SequenceNumber>,
    #[asn(optional_idx = 4)]
    pub ie_extensions: Option<RAB_ContextItemIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct RAB_ContextItem_RANAP_RelocInf {
    pub rab_id: RAB_ID,
    #[asn(optional_idx = 0)]
    pub dl_gtp_pdu_sequence_number: Option<DL_GTP_PDU_SequenceNumber>,
    #[asn(optional_idx = 1)]
    pub ul_gtp_pdu_sequence_number: Option<UL_GTP_PDU_SequenceNumber>,
    #[asn(optional_idx = 2)]
    pub dl_n_pdu_sequence_number: Option<DL_N_PDU_SequenceNumber>,
    #[asn(optional_idx = 3)]
    pub ul_n_pdu_sequence_number: Option<UL_N_PDU_SequenceNumber>,
    #[asn(optional_idx = 4)]
    pub ie_extensions: Option<RAB_ContextItem_RANAP_RelocInfIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_ContextList(pub Vec<RAB_ContextList_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_ContextList_RANAP_RelocInf(pub Vec<RAB_ContextList_RANAP_RelocInf_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_DataForwardingItem {
    pub rab_id: RAB_ID,
    pub transport_layer_address: TransportLayerAddress,
    pub iu_transport_association: IuTransportAssociation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RAB_DataForwardingItemIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_DataForwardingItem_SRNS_CtxReq {
    pub rab_id: RAB_ID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RAB_DataForwardingItem_SRNS_CtxReqIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_DataForwardingList(pub Vec<RAB_DataForwardingList_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_DataForwardingList_SRNS_CtxReq(pub Vec<RAB_DataForwardingList_SRNS_CtxReq_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct RAB_DataVolumeReportItem {
    pub rab_id: RAB_ID,
    #[asn(optional_idx = 0)]
    pub dl_unsuccessfully_transmitted_data_volume: Option<DataVolumeList>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<RAB_DataVolumeReportItemIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_DataVolumeReportList(pub Vec<RAB_DataVolumeReportList_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_DataVolumeReportRequestItem {
    pub rab_id: RAB_ID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RAB_DataVolumeReportRequestItemIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_DataVolumeReportRequestList(pub Vec<RAB_DataVolumeReportRequestList_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_FailedItem {
    pub rab_id: RAB_ID,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RAB_FailedItemIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_FailedItem_EnhRelocInfoRes {
    pub cn_domain_indicator: CN_DomainIndicator,
    pub rab_id: RAB_ID,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RAB_FailedItem_EnhRelocInfoResIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_FailedList(pub Vec<RAB_FailedList_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_FailedList_EnhRelocInfoRes(pub Vec<RAB_FailedList_EnhRelocInfoRes_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_FailedtoReportList(pub Vec<RAB_FailedtoReportList_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct RAB_ID(pub BitVec<u8, Msb0>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_ModifyItem {
    pub rab_id: RAB_ID,
    pub requested_rab_parameter_values: Requested_RAB_Parameter_Values,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RAB_ModifyItemIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_ModifyList(pub Vec<RAB_ModifyList_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_ModifyRequest {
    pub protocol_i_es: RAB_ModifyRequestProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RAB_ModifyRequestProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct RAB_Parameter_ExtendedGuaranteedBitrateList(pub Vec<ExtendedGuaranteedBitrate>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct RAB_Parameter_ExtendedMaxBitrateList(pub Vec<ExtendedMaxBitrate>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct RAB_Parameter_GuaranteedBitrateList(pub Vec<GuaranteedBitrate>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct RAB_Parameter_MaxBitrateList(pub Vec<MaxBitrate>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 7)]
pub struct RAB_Parameters {
    pub traffic_class: TrafficClass,
    pub rab_asymmetry_indicator: RAB_AsymmetryIndicator,
    pub max_bitrate: RAB_Parameter_MaxBitrateList,
    #[asn(optional_idx = 0)]
    pub guaranteed_bit_rate: Option<RAB_Parameter_GuaranteedBitrateList>,
    pub delivery_order: DeliveryOrder,
    pub max_sdu_size: MaxSDU_Size,
    pub sdu_parameters: SDU_Parameters,
    #[asn(optional_idx = 1)]
    pub transfer_delay: Option<TransferDelay>,
    #[asn(optional_idx = 2)]
    pub traffic_handling_priority: Option<TrafficHandlingPriority>,
    #[asn(optional_idx = 3)]
    pub allocation_or_retention_priority: Option<AllocationOrRetentionPriority>,
    #[asn(optional_idx = 4)]
    pub source_statistics_descriptor: Option<SourceStatisticsDescriptor>,
    #[asn(optional_idx = 5)]
    pub relocation_requirement: Option<RelocationRequirement>,
    #[asn(optional_idx = 6)]
    pub ie_extensions: Option<RAB_ParametersIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_QueuedItem {
    pub rab_id: RAB_ID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RAB_QueuedItemIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_QueuedList(pub Vec<RAB_QueuedList_Entry>);

pub type RAB_ReleaseFailedList = RAB_FailedList;

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_ReleaseItem {
    pub rab_id: RAB_ID,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RAB_ReleaseItemIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_ReleaseList(pub Vec<RAB_ReleaseList_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_ReleaseRequest {
    pub protocol_i_es: RAB_ReleaseRequestProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RAB_ReleaseRequestProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct RAB_ReleasedItem {
    pub rab_id: RAB_ID,
    #[asn(optional_idx = 0)]
    pub dl_data_volumes: Option<DataVolumeList>,
    #[asn(optional_idx = 1)]
    pub dl_gtp_pdu_sequence_number: Option<DL_GTP_PDU_SequenceNumber>,
    #[asn(optional_idx = 2)]
    pub ul_gtp_pdu_sequence_number: Option<UL_GTP_PDU_SequenceNumber>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<RAB_ReleasedItemIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct RAB_ReleasedItem_IuRelComp {
    pub rab_id: RAB_ID,
    #[asn(optional_idx = 0)]
    pub dl_gtp_pdu_sequence_number: Option<DL_GTP_PDU_SequenceNumber>,
    #[asn(optional_idx = 1)]
    pub ul_gtp_pdu_sequence_number: Option<UL_GTP_PDU_SequenceNumber>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<RAB_ReleasedItem_IuRelCompIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_ReleasedList(pub Vec<RAB_ReleasedList_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_ReleasedList_IuRelComp(pub Vec<RAB_ReleasedList_IuRelComp_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_RelocationReleaseItem {
    pub rab_id: RAB_ID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RAB_RelocationReleaseItemIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_RelocationReleaseList(pub Vec<RAB_RelocationReleaseList_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 7)]
pub struct RAB_SetupItem_EnhRelocInfoReq {
    pub rab_id: RAB_ID,
    pub cn_domain_indicator: CN_DomainIndicator,
    pub rab_parameters: RAB_Parameters,
    #[asn(optional_idx = 0)]
    pub data_volume_reporting_indication: Option<DataVolumeReportingIndication>,
    #[asn(optional_idx = 1)]
    pub pdp_type_information: Option<PDP_TypeInformation>,
    pub user_plane_information: UserPlaneInformation,
    #[asn(optional_idx = 2)]
    pub data_forwarding_information: Option<TNLInformationEnhRelInfoReq>,
    #[asn(optional_idx = 3)]
    pub source_side_iu_ultnl_info: Option<TNLInformationEnhRelInfoReq>,
    #[asn(optional_idx = 4)]
    pub service_handover: Option<Service_Handover>,
    #[asn(optional_idx = 5)]
    pub alt_rab_parameters: Option<Alt_RAB_Parameters>,
    #[asn(optional_idx = 6)]
    pub ie_extensions: Option<RAB_SetupItem_EnhRelocInfoReqIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct RAB_SetupItem_EnhRelocInfoRes {
    pub cn_domain_indicator: CN_DomainIndicator,
    pub rab_id: RAB_ID,
    #[asn(optional_idx = 0)]
    pub data_forwarding_information: Option<TNLInformationEnhRelInfoRes>,
    #[asn(optional_idx = 1)]
    pub ass_rab_parameters: Option<Ass_RAB_Parameters>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<RAB_SetupItem_EnhRelocInfoResIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct RAB_SetupItem_EnhancedRelocCompleteReq {
    pub rab_id: RAB_ID,
    #[asn(optional_idx = 0)]
    pub transport_layer_address_req1: Option<TransportLayerAddress>,
    #[asn(optional_idx = 1)]
    pub iu_transport_association_req1: Option<IuTransportAssociation>,
    #[asn(optional_idx = 2)]
    pub ass_rab_parameters: Option<Ass_RAB_Parameters>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<RAB_SetupItem_EnhancedRelocCompleteReqIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct RAB_SetupItem_EnhancedRelocCompleteRes {
    pub rab_id: RAB_ID,
    #[asn(optional_idx = 0)]
    pub rab_parameters: Option<RAB_Parameters>,
    pub user_plane_information: UserPlaneInformation,
    #[asn(optional_idx = 1)]
    pub transport_layer_address_res1: Option<TransportLayerAddress>,
    #[asn(optional_idx = 2)]
    pub iu_transport_association_res1: Option<IuTransportAssociation>,
    #[asn(optional_idx = 3)]
    pub rab2be_released_list: Option<RAB_ToBeReleasedList_EnhancedRelocCompleteRes>,
    #[asn(optional_idx = 4)]
    pub ie_extensions: Option<RAB_SetupItem_EnhancedRelocCompleteResIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct RAB_SetupItem_RelocReq {
    pub rab_id: RAB_ID,
    #[asn(optional_idx = 0)]
    pub nas_synchronisation_indicator: Option<NAS_SynchronisationIndicator>,
    pub rab_parameters: RAB_Parameters,
    #[asn(optional_idx = 1)]
    pub data_volume_reporting_indication: Option<DataVolumeReportingIndication>,
    #[asn(optional_idx = 2)]
    pub pdp_type_information: Option<PDP_TypeInformation>,
    pub user_plane_information: UserPlaneInformation,
    pub transport_layer_address: TransportLayerAddress,
    pub iu_transport_association: IuTransportAssociation,
    #[asn(optional_idx = 3)]
    pub service_handover: Option<Service_Handover>,
    #[asn(optional_idx = 4)]
    pub ie_extensions: Option<RAB_SetupItem_RelocReqIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct RAB_SetupItem_RelocReqAck {
    pub rab_id: RAB_ID,
    #[asn(optional_idx = 0)]
    pub transport_layer_address: Option<TransportLayerAddress>,
    #[asn(optional_idx = 1)]
    pub iu_transport_association: Option<IuTransportAssociation>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<RAB_SetupItem_RelocReqAckIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_SetupList_EnhRelocInfoReq(pub Vec<RAB_SetupList_EnhRelocInfoReq_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_SetupList_EnhRelocInfoRes(pub Vec<RAB_SetupList_EnhRelocInfoRes_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_SetupList_EnhancedRelocCompleteReq(
    pub Vec<RAB_SetupList_EnhancedRelocCompleteReq_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_SetupList_EnhancedRelocCompleteRes(
    pub Vec<RAB_SetupList_EnhancedRelocCompleteRes_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_SetupList_RelocReq(pub Vec<RAB_SetupList_RelocReq_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_SetupList_RelocReqAck(pub Vec<RAB_SetupList_RelocReqAck_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct RAB_SetupOrModifiedItem {
    pub rab_id: RAB_ID,
    #[asn(optional_idx = 0)]
    pub transport_layer_address: Option<TransportLayerAddress>,
    #[asn(optional_idx = 1)]
    pub iu_transport_association: Option<IuTransportAssociation>,
    #[asn(optional_idx = 2)]
    pub dl_data_volumes: Option<DataVolumeList>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<RAB_SetupOrModifiedItemIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_SetupOrModifiedList(pub Vec<RAB_SetupOrModifiedList_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 6)]
pub struct RAB_SetupOrModifyItemFirst {
    pub rab_id: RAB_ID,
    #[asn(optional_idx = 0)]
    pub nas_synchronisation_indicator: Option<NAS_SynchronisationIndicator>,
    #[asn(optional_idx = 1)]
    pub rab_parameters: Option<RAB_Parameters>,
    #[asn(optional_idx = 2)]
    pub user_plane_information: Option<UserPlaneInformation>,
    #[asn(optional_idx = 3)]
    pub transport_layer_information: Option<TransportLayerInformation>,
    #[asn(optional_idx = 4)]
    pub service_handover: Option<Service_Handover>,
    #[asn(optional_idx = 5)]
    pub ie_extensions: Option<RAB_SetupOrModifyItemFirstIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 7)]
pub struct RAB_SetupOrModifyItemSecond {
    #[asn(optional_idx = 0)]
    pub pdp_type_information: Option<PDP_TypeInformation>,
    #[asn(optional_idx = 1)]
    pub data_volume_reporting_indication: Option<DataVolumeReportingIndication>,
    #[asn(optional_idx = 2)]
    pub dl_gtp_pdu_sequence_number: Option<DL_GTP_PDU_SequenceNumber>,
    #[asn(optional_idx = 3)]
    pub ul_gtp_pdu_sequence_number: Option<UL_GTP_PDU_SequenceNumber>,
    #[asn(optional_idx = 4)]
    pub dl_n_pdu_sequence_number: Option<DL_N_PDU_SequenceNumber>,
    #[asn(optional_idx = 5)]
    pub ul_n_pdu_sequence_number: Option<UL_N_PDU_SequenceNumber>,
    #[asn(optional_idx = 6)]
    pub ie_extensions: Option<RAB_SetupOrModifyItemSecondIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_SetupOrModifyList(pub Vec<RAB_SetupOrModifyList_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "16000000")]
pub struct RAB_SubflowCombinationBitRate(pub u32);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_ToBeReleasedItem_EnhancedRelocCompleteRes {
    pub rab_id: RAB_ID,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RAB_ToBeReleasedItem_EnhancedRelocCompleteResIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_ToBeReleasedList_EnhancedRelocCompleteRes(
    pub Vec<RAB_ToBeReleasedList_EnhancedRelocCompleteRes_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_TrCH_Mapping(pub Vec<RAB_TrCH_MappingItem>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_TrCH_MappingItem {
    pub rab_id: RAB_ID,
    pub tr_ch_id_list: TrCH_ID_List,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RAB_TrCH_MappingItemIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct RABDataVolumeReport(pub Vec<RABDataVolumeReport_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RABParametersList(pub Vec<RABParametersList_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RABased {
    pub rai_list: RAI_List,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RABasedIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RABs_ContextFailedtoTransferItem {
    pub rab_id: RAB_ID,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RABs_ContextFailedtoTransferItemIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RABs_failed_to_reportItem {
    pub rab_id: RAB_ID,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RABs_failed_to_reportItemIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "1", sz_ub = "1")]
pub struct RAC(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAI {
    pub lai: LAI,
    pub rac: RAC,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RAIIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct RAI_List(pub Vec<RAI>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum RAListofIdleModeUEs {
    #[asn(key = 0, extended = false)]
    NotEmptyRAListofIdleModeUEs(NotEmptyRAListofIdleModeUEs),
    #[asn(key = 1, extended = false)]
    EmptyFullRAListofIdleModeUEs(ENUMERATED_50),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65536"
)]
pub struct RAListwithNoIdleModeUEsAnyMore(pub Vec<RAC>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RANAP_EnhancedRelocationInformationRequest {
    pub protocol_i_es: RANAP_EnhancedRelocationInformationRequestProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RANAP_EnhancedRelocationInformationRequestProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RANAP_EnhancedRelocationInformationResponse {
    pub protocol_i_es: RANAP_EnhancedRelocationInformationResponseProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RANAP_EnhancedRelocationInformationResponseProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = true)]
pub enum RANAP_PDU {
    #[asn(key = 0, extended = false)]
    InitiatingMessage(InitiatingMessage),
    #[asn(key = 1, extended = false)]
    SuccessfulOutcome(SuccessfulOutcome),
    #[asn(key = 2, extended = false)]
    UnsuccessfulOutcome(UnsuccessfulOutcome),
    #[asn(key = 3, extended = false)]
    Outcome(Outcome),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RANAP_RelocationInformation {
    pub protocol_i_es: RANAP_RelocationInformationProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RANAP_RelocationInformationProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct RAT_Type(pub u8);
impl RAT_Type {
    pub const UTRAN: u8 = 0u8;
    pub const GERAN: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65536"
)]
pub struct RAofIdleModeUEs(pub Vec<RAC>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct RIM_Transfer {
    pub rim_information: RIMInformation,
    #[asn(optional_idx = 0)]
    pub rim_routing_address: Option<RIMRoutingAddress>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<RIM_TransferIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING")]
pub struct RIMInformation(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum RIMRoutingAddress {
    #[asn(key = 0, extended = false)]
    TargetRNC_ID(TargetRNC_ID),
    #[asn(key = 1, extended = false)]
    GERAN_Cell_ID(GERAN_Cell_ID),
    #[asn(key = 0, extended = true)]
    TargeteNB_ID(TargetENB_ID),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct RNC_ID(pub u16);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct RNCTraceInformation {
    pub trace_reference: TraceReference,
    pub trace_activation_indicator: ENUMERATED_51,
    #[asn(optional_idx = 0)]
    pub equipments_to_be_traced: Option<EquipmentsToBeTraced>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<RNCTraceInformationIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct RNSAPRelocationParameters {
    #[asn(optional_idx = 0)]
    pub rab_parmeters_list: Option<RABParametersList>,
    #[asn(optional_idx = 1)]
    pub location_reporting: Option<LocationReportingTransferInformation>,
    #[asn(optional_idx = 2)]
    pub trace_information: Option<TraceInformation>,
    #[asn(optional_idx = 3)]
    pub source_sai: Option<SAI>,
    #[asn(optional_idx = 4)]
    pub ie_extensions: Option<RNSAPRelocationParametersIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING")]
pub struct RRC_Container(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "-30", ub = "46", extensible = true)]
pub struct RSRQ_Extension(pub i8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RSRQ_Type {
    pub all_symbols: BOOLEAN_52,
    pub wide_band: BOOLEAN_53,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct RSRVCC_HO_Indication(pub u8);
impl RSRVCC_HO_Indication {
    pub const PS_ONLY: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RSRVCC_Information {
    pub nonce: BIT_STRING_54,
    pub ims_information: OCTET_STRING_55,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RSRVCC_InformationIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct RSRVCC_Operation_Possible(pub u8);
impl RSRVCC_Operation_Possible {
    pub const RSRVCC_POSSIBLE: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct RTLoadValue(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct RateControlAllowed(pub u8);
impl RateControlAllowed {
    pub const NOT_ALLOWED: u8 = 0u8;
    pub const ALLOWED: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "NULL")]
pub struct RedirectAttemptFlag;

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct RedirectionCompleted(pub u8);
impl RedirectionCompleted {
    pub const REDIRECTION_COMPLETED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RedirectionIndication(pub Vec<RedirectionIndication_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "5")]
pub struct RejectCauseValue(pub u8);
impl RejectCauseValue {
    pub const P_LMN_NOT_ALLOWED: u8 = 0u8;
    pub const LOCATION_AREA_NOT_ALLOWED: u8 = 1u8;
    pub const ROAMING_NOT_ALLOWED_IN_THIS_LOCATION_AREA: u8 = 2u8;
    pub const NO_SUITABLE_CELL_IN_LOCATION_AREA: u8 = 3u8;
    pub const G_PRS_SERVICES_NOT_ALLOWED_IN_THIS_PLMN: u8 = 4u8;
    pub const C_S_PS_COORDINATION_REQUIRED: u8 = 5u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RelocationCancel {
    pub protocol_i_es: RelocationCancelProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RelocationCancelProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RelocationCancelAcknowledge {
    pub protocol_i_es: RelocationCancelAcknowledgeProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RelocationCancelAcknowledgeProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RelocationCommand {
    pub protocol_i_es: RelocationCommandProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RelocationCommandProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RelocationComplete {
    pub protocol_i_es: RelocationCompleteProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RelocationCompleteProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RelocationDetect {
    pub protocol_i_es: RelocationDetectProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RelocationDetectProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RelocationFailure {
    pub protocol_i_es: RelocationFailureProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RelocationFailureProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RelocationPreparationFailure {
    pub protocol_i_es: RelocationPreparationFailureProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RelocationPreparationFailureProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RelocationRequest {
    pub protocol_i_es: RelocationRequestProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RelocationRequestProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RelocationRequestAcknowledge {
    pub protocol_i_es: RelocationRequestAcknowledgeProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RelocationRequestAcknowledgeProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RelocationRequired {
    pub protocol_i_es: RelocationRequiredProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RelocationRequiredProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct RelocationRequirement(pub u8);
impl RelocationRequirement {
    pub const LOSSLESS: u8 = 0u8;
    pub const NONE: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct RelocationType(pub u8);
impl RelocationType {
    pub const UE_NOT_INVOLVED: u8 = 0u8;
    pub const UE_INVOLVED: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct RepetitionNumber0(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "256")]
pub struct RepetitionNumber1(pub u16);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "7")]
pub struct ReportAmount(pub u8);
impl ReportAmount {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N8: u8 = 3u8;
    pub const N16: u8 = 4u8;
    pub const N32: u8 = 5u8;
    pub const N64: u8 = 6u8;
    pub const INFINITY: u8 = 7u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ReportArea(pub u8);
impl ReportArea {
    pub const SERVICE_AREA: u8 = 0u8;
    pub const GEOGRAPHICAL_AREA: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ReportChangeOfSAI(pub u8);
impl ReportChangeOfSAI {
    pub const REQUESTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "12")]
pub struct ReportInterval(pub u8);
impl ReportInterval {
    pub const MS250: u8 = 0u8;
    pub const MS500: u8 = 1u8;
    pub const MS1000: u8 = 2u8;
    pub const MS2000: u8 = 3u8;
    pub const MS3000: u8 = 4u8;
    pub const MS4000: u8 = 5u8;
    pub const MS6000: u8 = 6u8;
    pub const MS12000: u8 = 7u8;
    pub const MS16000: u8 = 8u8;
    pub const MS20000: u8 = 9u8;
    pub const MS24000: u8 = 10u8;
    pub const MS32000: u8 = 11u8;
    pub const MS64000: u8 = 12u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RequestType {
    pub event: Event,
    pub report_area: ReportArea,
    #[asn(optional_idx = 0)]
    pub accuracy_code: Option<INTEGER_56>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct Requested_RAB_Parameter_ExtendedGuaranteedBitrateList(
    pub Vec<ExtendedGuaranteedBitrate>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct Requested_RAB_Parameter_ExtendedMaxBitrateList(pub Vec<ExtendedMaxBitrate>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct Requested_RAB_Parameter_GuaranteedBitrateList(pub Vec<GuaranteedBitrate>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct Requested_RAB_Parameter_MaxBitrateList(pub Vec<MaxBitrate>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct Requested_RAB_Parameter_Values {
    #[asn(optional_idx = 0)]
    pub requested_max_bitrates: Option<Requested_RAB_Parameter_MaxBitrateList>,
    #[asn(optional_idx = 1)]
    pub requested_guaranteed_bitrates: Option<Requested_RAB_Parameter_GuaranteedBitrateList>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<Requested_RAB_Parameter_ValuesIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "OCTET-STRING",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "201"
)]
pub struct RequestedGANSSAssistanceData(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "OCTET-STRING",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "38"
)]
pub struct RequestedGPSAssistanceData(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct RequestedLocationRelatedDataType(pub u8);
impl RequestedLocationRelatedDataType {
    pub const DECIPHERING_KEYS_UE_BASED_OTDOA: u8 = 0u8;
    pub const DECIPHERING_KEYS_ASSISTED_GPS: u8 = 1u8;
    pub const DEDICATED_ASSISTANCE_DATA_UE_BASED_OTDOA: u8 = 2u8;
    pub const DEDICATED_ASSISTANCE_DATA_ASSISTED_GPS: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "512"
)]
pub struct RequestedMBMSIPMulticastAddressandAPNRequest(pub Vec<MBMSIPMulticastAddressandAPNlist>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "128"
)]
pub struct RequestedMulticastServiceList(pub Vec<TMGI>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RerouteNASRequest {
    pub protocol_i_es: RerouteNASRequestProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RerouteNASRequestProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Reset {
    pub protocol_i_es: ResetProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<ResetProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ResetAcknowledge {
    pub protocol_i_es: ResetAcknowledgeProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<ResetAcknowledgeProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ResetResource {
    pub protocol_i_es: ResetResourceProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<ResetResourceProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ResetResourceAckItem {
    pub iu_sig_con_id: IuSignallingConnectionIdentifier,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ResetResourceAckItemIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "250"
)]
pub struct ResetResourceAckList(pub Vec<ResetResourceAckList_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ResetResourceAcknowledge {
    pub protocol_i_es: ResetResourceAcknowledgeProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<ResetResourceAcknowledgeProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ResetResourceItem {
    pub iu_sig_con_id: IuSignallingConnectionIdentifier,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ResetResourceItemIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "250"
)]
pub struct ResetResourceList(pub Vec<ResetResourceList_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct ResidualBitErrorRatio {
    pub mantissa: INTEGER_57,
    pub exponent: INTEGER_58,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ResidualBitErrorRatioIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ResponseTime(pub u8);
impl ResponseTime {
    pub const LOWDELAY: u8 = 0u8;
    pub const DELAYTOLERANT: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct SAC(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct SAI {
    pub plm_nidentity: PLMNidentity,
    pub lac: LAC,
    pub sac: SAC,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SAIIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct SAPI(pub u8);
impl SAPI {
    pub const SAPI_0: u8 = 0u8;
    pub const SAPI_3: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct SDU_ErrorRatio {
    pub mantissa: INTEGER_59,
    pub exponent: INTEGER_60,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SDU_ErrorRatioIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct SDU_FormatInformationParameters(pub Vec<SDU_FormatInformationParameters_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "7")]
pub struct SDU_Parameters(pub Vec<SDU_Parameters_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct SGSN_Group_ID(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum SGSN_Group_Identity {
    #[asn(key = 0, extended = false)]
    Null_NRI(Null_NRI),
    #[asn(key = 1, extended = false)]
    SGSN_Group_ID(SGSN_Group_ID),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SNA_Access_Information {
    pub authorised_plm_ns: AuthorisedPLMNs,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SNA_Access_InformationIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct SNAC(pub u16);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "32")]
pub struct SRB_ID(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct SRB_TrCH_Mapping(pub Vec<SRB_TrCH_MappingItem>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SRB_TrCH_MappingItem {
    pub srb_id: SRB_ID,
    pub tr_ch_id: TrCH_ID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SRB_TrCH_MappingItemIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SRNS_ContextRequest {
    pub protocol_i_es: SRNS_ContextRequestProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<SRNS_ContextRequestProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SRNS_ContextResponse {
    pub protocol_i_es: SRNS_ContextResponseProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<SRNS_ContextResponseProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SRNS_DataForwardCommand {
    pub protocol_i_es: SRNS_DataForwardCommandProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<SRNS_DataForwardCommandProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SRVCC_CSKeysRequest {
    pub protocol_i_es: SRVCC_CSKeysRequestProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<SRVCC_CSKeysRequestProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SRVCC_CSKeysResponse {
    pub protocol_i_es: SRVCC_CSKeysResponseProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<SRVCC_CSKeysResponseProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct SRVCC_HO_Indication(pub u8);
impl SRVCC_HO_Indication {
    pub const PS_AND_CS: u8 = 0u8;
    pub const CS_ONLY: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SRVCC_Information {
    pub nonce: BIT_STRING_61,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SRVCC_InformationIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct SRVCC_Operation_Possible(pub u8);
impl SRVCC_Operation_Possible {
    pub const SRVCC_POSSIBLE: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct SRVCCSource(pub u8);
impl SRVCCSource {
    pub const V5_G: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SecurityModeCommand {
    pub protocol_i_es: SecurityModeCommandProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<SecurityModeCommandProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SecurityModeComplete {
    pub protocol_i_es: SecurityModeCompleteProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<SecurityModeCompleteProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SecurityModeReject {
    pub protocol_i_es: SecurityModeRejectProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<SecurityModeRejectProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct Service_Handover(pub u8);
impl Service_Handover {
    pub const HANDOVER_TO_GSM_SHOULD_BE_PERFORMED: u8 = 0u8;
    pub const HANDOVER_TO_GSM_SHOULD_NOT_BE_PERFORMED: u8 = 1u8;
    pub const HANDOVER_TO_GSM_SHALL_NOT_BE_PERFORMED: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ServiceType(pub u8);
impl ServiceType {
    pub const Q_MC_FOR_STREAMING_SERVICE: u8 = 0u8;
    pub const Q_MC_FOR_MSTI_SERVICE: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Session_Re_establishment_Indicator(pub u8);
impl Session_Re_establishment_Indicator {
    pub const TRUE: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1048575")]
pub struct SessionUpdateID(pub u32);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Shared_Network_Information {
    pub plm_ns_in_shared_network: PLMNs_in_shared_network,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<Shared_Network_InformationIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct SignallingIndication(pub u8);
impl SignallingIndication {
    pub const SIGNALLING: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING")]
pub struct Source_ToTarget_TransparentContainer(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING")]
pub struct SourceBSS_ToTargetBSS_TransparentContainer(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum SourceCellID {
    #[asn(key = 0, extended = false)]
    SourceUTRANCellID(SourceUTRANCellID),
    #[asn(key = 1, extended = false)]
    SourceGERANCellID(CGI),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum SourceID {
    #[asn(key = 0, extended = false)]
    SourceRNC_ID(SourceRNC_ID),
    #[asn(key = 1, extended = false)]
    SAI(SAI),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct SourceRNC_ID {
    pub plm_nidentity: PLMNidentity,
    pub rnc_id: RNC_ID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SourceRNC_IDIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 10)]
pub struct SourceRNC_ToTargetRNC_TransparentContainer {
    pub rrc_container: RRC_Container,
    pub number_of_iu_instances: NumberOfIuInstances,
    pub relocation_type: RelocationType,
    #[asn(optional_idx = 0)]
    pub chosen_integrity_protection_algorithm: Option<ChosenIntegrityProtectionAlgorithm>,
    #[asn(optional_idx = 1)]
    pub integrity_protection_key: Option<IntegrityProtectionKey>,
    #[asn(optional_idx = 2)]
    pub chosen_encryption_algorith_for_signalling: Option<ChosenEncryptionAlgorithm>,
    #[asn(optional_idx = 3)]
    pub ciphering_key: Option<EncryptionKey>,
    #[asn(optional_idx = 4)]
    pub chosen_encryption_algorith_for_cs: Option<ChosenEncryptionAlgorithm>,
    #[asn(optional_idx = 5)]
    pub chosen_encryption_algorith_for_ps: Option<ChosenEncryptionAlgorithm>,
    #[asn(optional_idx = 6)]
    pub d_rnti: Option<D_RNTI>,
    #[asn(optional_idx = 7)]
    pub target_cell_id: Option<TargetCellId>,
    #[asn(optional_idx = 8)]
    pub rab_tr_ch_mapping: Option<RAB_TrCH_Mapping>,
    #[asn(optional_idx = 9)]
    pub ie_extensions: Option<SourceRNC_ToTargetRNC_TransparentContainerIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct SourceStatisticsDescriptor(pub u8);
impl SourceStatisticsDescriptor {
    pub const SPEECH: u8 = 0u8;
    pub const UNKNOWN: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct SourceUTRANCellID {
    pub plm_nidentity: PLMNidentity,
    pub utra_ncell_id: TargetCellId,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SourceUTRANCellIDIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING")]
pub struct SourceeNodeB_ToTargeteNodeB_TransparentContainer(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct SubflowSDU_Size(pub u16);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "256")]
pub struct SubscriberProfileIDforRFP(pub u16);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SuccessfulOutcome {
    #[asn(key_field = true)]
    pub procedure_code: ProcedureCode,
    pub criticality: Criticality,
    pub value: SuccessfulOutcomeValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "1000000000", extensible = true)]
pub struct SupportedBitrate(pub u32);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct SupportedRAB_ParameterBitrateList(pub Vec<SupportedBitrate>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct TAC(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct TAI {
    pub plm_nidentity: PLMNidentity,
    pub tac: TAC,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TAIIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING")]
pub struct TBCD_STRING(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct TMGI {
    pub plm_nidentity: PLMNidentity,
    pub service_id: OCTET_STRING_62,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TMGIIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct TMSI(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TNLInformationEnhRelInfoReq {
    pub transport_layer_address: TransportLayerAddress,
    pub iu_transport_association: IuTransportAssociation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TNLInformationEnhRelInfoReqIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TNLInformationEnhRelInfoRes {
    pub dl_forwarding_transport_layer_address: TransportLayerAddress,
    pub dl_forwarding_transport_association: IuTransportAssociation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TNLInformationEnhRelInfoResIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING")]
pub struct Target_ToSource_TransparentContainer(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING")]
pub struct TargetBSS_ToSourceBSS_TransparentContainer(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "268435455")]
pub struct TargetCellId(pub u32);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TargetENB_ID {
    pub plm_nidentity: PLMNidentity,
    pub enb_id: ENB_ID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TargetENB_IDIE_Extensions>,
    pub selected_tai: TAI,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum TargetID {
    #[asn(key = 0, extended = false)]
    TargetRNC_ID(TargetRNC_ID),
    #[asn(key = 1, extended = false)]
    CGI(CGI),
    #[asn(key = 0, extended = true)]
    TargeteNB_ID(TargetENB_ID),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct TargetRNC_ID {
    pub lai: LAI,
    #[asn(optional_idx = 0)]
    pub rac: Option<RAC>,
    pub rnc_id: RNC_ID,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<TargetRNC_IDIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct TargetRNC_ToSourceRNC_TransparentContainer {
    pub rrc_container: RRC_Container,
    #[asn(optional_idx = 0)]
    pub d_rnti: Option<D_RNTI>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<TargetRNC_ToSourceRNC_TransparentContainerIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING")]
pub struct TargeteNodeB_ToSourceeNodeB_TransparentContainer(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum TemporaryUE_ID {
    #[asn(key = 0, extended = false)]
    TMSI(TMSI),
    #[asn(key = 1, extended = false)]
    P_TMSI(P_TMSI),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct Time_UE_StayedInCell(pub u16);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "40950")]
pub struct Time_UE_StayedInCell_EnhancedGranularity(pub u16);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "1", sz_ub = "1")]
pub struct TimeToMBMSDataTransfer(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "1", sz_ub = "1")]
pub struct TimingDifferenceULDL(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct TrCH_ID {
    #[asn(optional_idx = 0)]
    pub dch_id: Option<DCH_ID>,
    #[asn(optional_idx = 1)]
    pub dsch_id: Option<DSCH_ID>,
    #[asn(optional_idx = 2)]
    pub usch_id: Option<USCH_ID>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<TrCH_IDIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "7")]
pub struct TrCH_ID_List(pub Vec<TrCH_ID>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct TraceDepth(pub u8);
impl TraceDepth {
    pub const MINIMUM: u8 = 0u8;
    pub const MEDIUM: u8 = 1u8;
    pub const MAXIMUM: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct TraceInformation {
    pub trace_reference: TraceReference,
    pub ue_identity: UE_ID,
    #[asn(optional_idx = 0)]
    pub trace_propagation_parameters: Option<TracePropagationParameters>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<TraceInformationIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct TracePropagationParameters {
    pub trace_recording_session_reference: TraceRecordingSessionReference,
    pub trace_depth: TraceDepth,
    #[asn(optional_idx = 0)]
    pub list_of_interfaces_to_trace: Option<ListOfInterfacesToTrace>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<TracePropagationParametersIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TraceRecordingSessionInformation {
    pub trace_reference: TraceReference,
    pub trace_recording_session_reference: TraceRecordingSessionReference,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TraceRecordingSessionInformationIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct TraceRecordingSessionReference(pub u16);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "2", sz_ub = "3")]
pub struct TraceReference(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "1", sz_ub = "1")]
pub struct TraceType(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct TrafficClass(pub u8);
impl TrafficClass {
    pub const CONVERSATIONAL: u8 = 0u8;
    pub const STREAMING: u8 = 1u8;
    pub const INTERACTIVE: u8 = 2u8;
    pub const BACKGROUND: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct TrafficHandlingPriority(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct TransferDelay(pub u16);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "1", sz_ub = "160")]
pub struct TransportLayerAddress(pub BitVec<u8, Msb0>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TransportLayerInformation {
    pub transport_layer_address: TransportLayerAddress,
    pub iu_transport_association: IuTransportAssociation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TransportLayerInformationIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "OCTET-STRING",
    sz_extensible = false,
    sz_lb = "3",
    sz_ub = "22"
)]
pub struct TriggerID(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "3")]
pub struct TriggeringMessage(pub u8);
impl TriggeringMessage {
    pub const INITIATING_MESSAGE: u8 = 0u8;
    pub const SUCCESSFUL_OUTCOME: u8 = 1u8;
    pub const UNSUCCESSFULL_OUTCOME: u8 = 2u8;
    pub const OUTCOME: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct TunnelInformation {
    pub transport_layer_address: TransportLayerAddress,
    #[asn(optional_idx = 0)]
    pub udp_port_number: Option<Port_Number>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<TunnelInformationIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct TypeOfError(pub u8);
impl TypeOfError {
    pub const NOT_UNDERSTOOD: u8 = 0u8;
    pub const MISSING: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct UE_AggregateMaximumBitRate {
    #[asn(optional_idx = 0)]
    pub ue_aggregate_maximum_bit_rate_downlink: Option<UE_AggregateMaximumBitRateDownlink>,
    #[asn(optional_idx = 1)]
    pub ue_aggregate_maximum_bit_rate_uplink: Option<UE_AggregateMaximumBitRateUplink>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "1000000000")]
pub struct UE_AggregateMaximumBitRateDownlink(pub u32);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "1000000000")]
pub struct UE_AggregateMaximumBitRateUplink(pub u32);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct UE_Application_Layer_Measurement_Capability(pub BitVec<u8, Msb0>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UE_Application_Layer_Measurement_Configuration {
    pub application_layer_container_for_measurement_configuration: OCTET_STRING_63,
    pub area_scope_for_ue_application_layer_measurement_configuration:
        AreaScopeForUEApplicationLayerMeasurementConfiguration,
    pub service_type: ServiceType,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct UE_Application_Layer_Measurement_Configuration_For_Relocation {
    pub area_scope_for_ue_application_layer_measurement_configuration:
        AreaScopeForUEApplicationLayerMeasurementConfiguration,
    pub trace_reference: TraceReference,
    #[asn(optional_idx = 0)]
    pub trace_propagation_parameters: Option<TracePropagationParameters>,
    #[asn(optional_idx = 1)]
    pub trace_collection_entity_ip_address: Option<TransportLayerAddress>,
    pub service_type: ServiceType,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING")]
pub struct UE_History_Information(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum UE_ID {
    #[asn(key = 0, extended = false)]
    Imsi(IMSI),
    #[asn(key = 1, extended = false)]
    Imei(IMEI),
    #[asn(key = 0, extended = true)]
    Imeisv(IMEISV),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UE_IsNotServed {
    pub permanent_nas_ue_id: PermanentNAS_UE_ID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UE_IsNotServedIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UE_IsServed {
    pub permanent_nas_ue_id: PermanentNAS_UE_ID,
    pub plm_nidentity: PLMNidentity,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UE_IsServedIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct UE_Usage_Type(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum UERegistrationQueryResult {
    #[asn(key = 0, extended = false)]
    UE_IsServed(UE_IsServed),
    #[asn(key = 1, extended = false)]
    UE_IsNotServed(UE_IsNotServed),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct UESBI_Iu {
    #[asn(optional_idx = 0)]
    pub uesbi_iu_a: Option<UESBI_IuA>,
    #[asn(optional_idx = 1)]
    pub uesbi_iu_b: Option<UESBI_IuB>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<UESBI_IuIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "128")]
pub struct UESBI_IuA(pub BitVec<u8, Msb0>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "128")]
pub struct UESBI_IuB(pub BitVec<u8, Msb0>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UESpecificInformationIndication {
    pub protocol_i_es: UESpecificInformationIndicationProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<UESpecificInformationIndicationProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct UL_GTP_PDU_SequenceNumber(pub u16);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct UL_N_PDU_SequenceNumber(pub u16);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct UP_ModeVersions(pub BitVec<u8, Msb0>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UPInformation {
    pub frame_seq_no_ul: FrameSequenceNumber,
    pub frame_seq_no_dl: FrameSequenceNumber,
    pub pdu14_frame_seq_no_ul: PDUType14FrameSequenceNumber,
    pub pdu14_frame_seq_no_dl: PDUType14FrameSequenceNumber,
    pub data_pdu_type: DataPDUType,
    pub upinitialisation_frame: UPInitialisationFrame,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UPInformationIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING")]
pub struct UPInitialisationFrame(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct USCH_ID(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct UTRAN_CellID {
    pub plm_nidentity: PLMNidentity,
    pub cell_id: TargetCellId,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UTRAN_CellIDIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct UeApplicationLayerMeasurementSupportIndication(pub BitVec<u8, Msb0>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UeRadioCapabilityMatchRequest {
    pub protocol_i_es: UeRadioCapabilityMatchRequestProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<UeRadioCapabilityMatchRequestProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UeRadioCapabilityMatchResponse {
    pub protocol_i_es: UeRadioCapabilityMatchResponseProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<UeRadioCapabilityMatchResponseProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UeRegistrationQueryRequest {
    pub protocol_i_es: UeRegistrationQueryRequestProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<UeRegistrationQueryRequestProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UeRegistrationQueryResponse {
    pub protocol_i_es: UeRegistrationQueryResponseProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<UeRegistrationQueryResponseProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "128"
)]
pub struct UnsuccessfulLinking_IEs(pub Vec<UnsuccessfulLinking_IEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UnsuccessfulOutcome {
    #[asn(key_field = true)]
    pub procedure_code: ProcedureCode,
    pub criticality: Criticality,
    pub value: UnsuccessfulOutcomeValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "4294967295")]
pub struct UnsuccessfullyTransmittedDataVolume(pub u32);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UplinkInformationExchangeFailure {
    pub protocol_i_es: UplinkInformationExchangeFailureProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<UplinkInformationExchangeFailureProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UplinkInformationExchangeRequest {
    pub protocol_i_es: UplinkInformationExchangeRequestProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<UplinkInformationExchangeRequestProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UplinkInformationExchangeResponse {
    pub protocol_i_es: UplinkInformationExchangeResponseProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<UplinkInformationExchangeResponseProtocolExtensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UserPlaneInformation {
    pub user_plane_mode: UserPlaneMode,
    pub up_mode_versions: UP_ModeVersions,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UserPlaneInformationIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct UserPlaneMode(pub u8);
impl UserPlaneMode {
    pub const TRANSPARENT_MODE: u8 = 0u8;
    pub const SUPPORT_MODE_FOR_PREDEFINED_SDU_SIZES: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = true)]
pub enum VelocityEstimate {
    #[asn(key = 0, extended = false)]
    HorizontalVelocity(HorizontalVelocity),
    #[asn(key = 1, extended = false)]
    HorizontalWithVerticalVelocity(HorizontalWithVerticalVelocity),
    #[asn(key = 2, extended = false)]
    HorizontalVelocityWithUncertainty(HorizontalVelocityWithUncertainty),
    #[asn(key = 3, extended = false)]
    HorizontalWithVeritcalVelocityAndUncertainty(HorizontalWithVerticalVelocityAndUncertainty),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct VerticalAccuracyCode(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct VerticalSpeedDirection(pub u8);
impl VerticalSpeedDirection {
    pub const UPWARD: u8 = 0u8;
    pub const DOWNWARD: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct VerticalVelocity {
    pub veritcal_speed: INTEGER_64,
    pub veritcal_speed_direction: VerticalSpeedDirection,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct VoiceSupportMatchIndicator(pub u8);
impl VoiceSupportMatchIndicator {
    pub const SUPPORTED: u8 = 0u8;
    pub const NOT_SUPPORTED: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "10", sz_ub = "10")]
pub struct BIT_STRING_2(pub BitVec<u8, Msb0>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "NULL")]
pub struct NULL_3;

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Additional_CSPS_coordination_informationIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Additional_CSPS_coordination_informationIE_Extensions(
    pub Vec<Additional_CSPS_coordination_informationIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AllocationOrRetentionPriorityIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AllocationOrRetentionPriorityIE_Extensions(
    pub Vec<AllocationOrRetentionPriorityIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Alt_RAB_Parameter_SupportedGuaranteedBitrateInfIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Alt_RAB_Parameter_SupportedGuaranteedBitrateInfIE_Extensions(
    pub Vec<Alt_RAB_Parameter_SupportedGuaranteedBitrateInfIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Alt_RAB_Parameter_SupportedMaxBitrateInfIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Alt_RAB_Parameter_SupportedMaxBitrateInfIE_Extensions(
    pub Vec<Alt_RAB_Parameter_SupportedMaxBitrateInfIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum Alt_RAB_ParametersIE_Extensions_EntryExtensionValue {
    #[asn(key = 172)]
    Id_Alt_RAB_Parameter_ExtendedGuaranteedBitrateInf(
        Alt_RAB_Parameter_ExtendedGuaranteedBitrateInf,
    ),
    #[asn(key = 173)]
    Id_Alt_RAB_Parameter_ExtendedMaxBitrateInf(Alt_RAB_Parameter_ExtendedMaxBitrateInf),
    #[asn(key = 214)]
    Id_Alt_RAB_Parameter_SupportedGuaranteedBitrateInf(
        Alt_RAB_Parameter_SupportedGuaranteedBitrateInf,
    ),
    #[asn(key = 215)]
    Id_Alt_RAB_Parameter_SupportedMaxBitrateInf(Alt_RAB_Parameter_SupportedMaxBitrateInf),
    #[asn(key = 158)]
    Id_AlternativeRABConfiguration(RAB_Parameters),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Alt_RAB_ParametersIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: Alt_RAB_ParametersIE_Extensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Alt_RAB_ParametersIE_Extensions(pub Vec<Alt_RAB_ParametersIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum Ass_RAB_ParametersIE_Extensions_EntryExtensionValue {
    #[asn(key = 174)]
    Id_Ass_RAB_Parameter_ExtendedGuaranteedBitrateList(
        Ass_RAB_Parameter_ExtendedGuaranteedBitrateList,
    ),
    #[asn(key = 175)]
    Id_Ass_RAB_Parameter_ExtendedMaxBitrateList(Ass_RAB_Parameter_ExtendedMaxBitrateList),
    #[asn(key = 216)]
    Id_Ass_RAB_Parameter_SupportedGuaranteedBitrateList(SupportedRAB_ParameterBitrateList),
    #[asn(key = 217)]
    Id_Ass_RAB_Parameter_SupportedMaxBitrateList(SupportedRAB_ParameterBitrateList),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Ass_RAB_ParametersIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: Ass_RAB_ParametersIE_Extensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Ass_RAB_ParametersIE_Extensions(pub Vec<Ass_RAB_ParametersIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AuthorisedPLMNs_EntryIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AuthorisedPLMNs_EntryIE_Extensions(pub Vec<AuthorisedPLMNs_EntryIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct AuthorisedPLMNs_Entry {
    pub plm_nidentity: PLMNidentity,
    #[asn(optional_idx = 0)]
    pub authorised_sn_as_list: Option<AuthorisedSNAs>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<AuthorisedPLMNs_EntryIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "1")]
pub struct BIT_STRING_4(pub BitVec<u8, Msb0>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "56", sz_ub = "56")]
pub struct BIT_STRING_5(pub BitVec<u8, Msb0>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "56", sz_ub = "56")]
pub struct BIT_STRING_6(pub BitVec<u8, Msb0>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum CGIIE_Extensions_EntryExtensionValue {
    #[asn(key = 55)]
    Id_RAC(RAC),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CGIIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: CGIIE_Extensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CGIIE_Extensions(pub Vec<CGIIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum CN_DeactivateTraceProtocolIEs_EntryValue {
    #[asn(key = 65)]
    Id_TraceReference(TraceReference),
    #[asn(key = 68)]
    Id_TriggerID(TriggerID),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CN_DeactivateTraceProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: CN_DeactivateTraceProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct CN_DeactivateTraceProtocolIEs(pub Vec<CN_DeactivateTraceProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CN_DeactivateTraceProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CN_DeactivateTraceProtocolExtensions(
    pub Vec<CN_DeactivateTraceProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum CN_InvokeTraceProtocolIEs_EntryValue {
    #[asn(key = 19)]
    Id_OMC_ID(OMC_ID),
    #[asn(key = 65)]
    Id_TraceReference(TraceReference),
    #[asn(key = 66)]
    Id_TraceType(TraceType),
    #[asn(key = 68)]
    Id_TriggerID(TriggerID),
    #[asn(key = 69)]
    Id_UE_ID(UE_ID),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CN_InvokeTraceProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: CN_InvokeTraceProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct CN_InvokeTraceProtocolIEs(pub Vec<CN_InvokeTraceProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum CN_InvokeTraceProtocolExtensions_EntryExtensionValue {
    #[asn(key = 244)]
    Id_MDT_Configuration(MDT_Configuration),
    #[asn(key = 251)]
    Id_Trace_Collection_Entity_IP_Addess(TransportLayerAddress),
    #[asn(key = 125)]
    Id_TracePropagationParameters(TracePropagationParameters),
    #[asn(key = 292)]
    Id_UE_Application_Layer_Measurement_Configuration(
        UE_Application_Layer_Measurement_Configuration,
    ),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CN_InvokeTraceProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: CN_InvokeTraceProtocolExtensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CN_InvokeTraceProtocolExtensions(pub Vec<CN_InvokeTraceProtocolExtensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CNMBMSLinkingInformationIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CNMBMSLinkingInformationIE_Extensions(
    pub Vec<CNMBMSLinkingInformationIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellBasedIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellBasedIE_Extensions(pub Vec<CellBasedIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellLoadInformationIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellLoadInformationIE_Extensions(pub Vec<CellLoadInformationIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellLoadInformationGroupIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellLoadInformationGroupIE_Extensions(
    pub Vec<CellLoadInformationGroupIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum CommonIDProtocolIEs_EntryValue {
    #[asn(key = 23)]
    Id_PermanentNAS_UE_ID(PermanentNAS_UE_ID),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CommonIDProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: CommonIDProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct CommonIDProtocolIEs(pub Vec<CommonIDProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum CommonIDProtocolExtensions_EntryExtensionValue {
    #[asn(key = 234)]
    Id_CSG_Membership_Status(CSG_Membership_Status),
    #[asn(key = 277)]
    Id_LastE_UTRANPLMNIdentity(PLMNidentity),
    #[asn(key = 249)]
    Id_Management_Based_MDT_Allowed(Management_Based_MDT_Allowed),
    #[asn(key = 263)]
    Id_Management_Based_MDT_PLMN_List(MDT_PLMN_List),
    #[asn(key = 289)]
    Id_PowerSavingIndicator(PowerSavingIndicator),
    #[asn(key = 272)]
    Id_RSRVCC_Operation_Possible(RSRVCC_Operation_Possible),
    #[asn(key = 105)]
    Id_SNA_Access_Information(SNA_Access_Information),
    #[asn(key = 228)]
    Id_SRVCC_Operation_Possible(SRVCC_Operation_Possible),
    #[asn(key = 127)]
    Id_SelectedPLMN_ID(PLMNidentity),
    #[asn(key = 202)]
    Id_SubscriberProfileIDforRFP(SubscriberProfileIDforRFP),
    #[asn(key = 118)]
    Id_UESBI_Iu(UESBI_Iu),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CommonIDProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: CommonIDProtocolExtensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CommonIDProtocolExtensions(pub Vec<CommonIDProtocolExtensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CriticalityDiagnosticsIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CriticalityDiagnosticsIE_Extensions(pub Vec<CriticalityDiagnosticsIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum CriticalityDiagnostics_IE_List_EntryIE_Extensions_EntryExtensionValue {
    #[asn(key = 88)]
    Id_MessageStructure(MessageStructure),
    #[asn(key = 93)]
    Id_TypeOfError(TypeOfError),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CriticalityDiagnostics_IE_List_EntryIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: CriticalityDiagnostics_IE_List_EntryIE_Extensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CriticalityDiagnostics_IE_List_EntryIE_Extensions(
    pub Vec<CriticalityDiagnostics_IE_List_EntryIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct CriticalityDiagnostics_IE_List_Entry {
    pub ie_criticality: Criticality,
    pub ie_id: ProtocolIE_ID,
    #[asn(optional_idx = 0)]
    pub repetition_number: Option<RepetitionNumber0>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<CriticalityDiagnostics_IE_List_EntryIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DataVolumeList_EntryIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DataVolumeList_EntryIE_Extensions(pub Vec<DataVolumeList_EntryIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct DataVolumeList_Entry {
    pub dl_unsuccessfully_transmitted_data_volume: UnsuccessfullyTransmittedDataVolume,
    #[asn(optional_idx = 0)]
    pub data_volume_reference: Option<DataVolumeReference>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<DataVolumeList_EntryIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum DataVolumeReportProtocolIEs_EntryValue {
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 31)]
    Id_RAB_DataVolumeReportList(RAB_DataVolumeReportList),
    #[asn(key = 72)]
    Id_RAB_FailedtoReportList(RAB_FailedtoReportList),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DataVolumeReportProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: DataVolumeReportProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct DataVolumeReportProtocolIEs(pub Vec<DataVolumeReportProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DataVolumeReportProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DataVolumeReportProtocolExtensions(pub Vec<DataVolumeReportProtocolExtensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum DataVolumeReportRequestProtocolIEs_EntryValue {
    #[asn(key = 33)]
    Id_RAB_DataVolumeReportRequestList(RAB_DataVolumeReportRequestList),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DataVolumeReportRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: DataVolumeReportRequestProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct DataVolumeReportRequestProtocolIEs(pub Vec<DataVolumeReportRequestProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DataVolumeReportRequestProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DataVolumeReportRequestProtocolExtensions(
    pub Vec<DataVolumeReportRequestProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum DeltaRAListofIdleModeUEsIE_Extensions_EntryExtensionValue {
    #[asn(key = 182)]
    Id_LAListwithNoIdleModeUEsAnyMore(LAListofIdleModeUEs),
    #[asn(key = 181)]
    Id_newLAListofIdleModeUEs(LAListofIdleModeUEs),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DeltaRAListofIdleModeUEsIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: DeltaRAListofIdleModeUEsIE_Extensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DeltaRAListofIdleModeUEsIE_Extensions(
    pub Vec<DeltaRAListofIdleModeUEsIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum DirectInformationTransferProtocolIEs_EntryValue {
    #[asn(key = 3)]
    Id_CN_DomainIndicator(CN_DomainIndicator),
    #[asn(key = 96)]
    Id_GlobalCN_ID(GlobalCN_ID),
    #[asn(key = 86)]
    Id_GlobalRNC_ID(GlobalRNC_ID),
    #[asn(key = 126)]
    Id_InterSystemInformationTransferType(InterSystemInformationTransferType),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DirectInformationTransferProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: DirectInformationTransferProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct DirectInformationTransferProtocolIEs(
    pub Vec<DirectInformationTransferProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum DirectInformationTransferProtocolExtensions_EntryExtensionValue {
    #[asn(key = 171)]
    Id_ExtendedRNC_ID(ExtendedRNC_ID),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DirectInformationTransferProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: DirectInformationTransferProtocolExtensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DirectInformationTransferProtocolExtensions(
    pub Vec<DirectInformationTransferProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum DirectTransferProtocolIEs_EntryValue {
    #[asn(key = 15)]
    Id_LAI(LAI),
    #[asn(key = 16)]
    Id_NAS_PDU(NAS_PDU),
    #[asn(key = 55)]
    Id_RAC(RAC),
    #[asn(key = 58)]
    Id_SAI(SAI),
    #[asn(key = 59)]
    Id_SAPI(SAPI),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DirectTransferProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: DirectTransferProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct DirectTransferProtocolIEs(pub Vec<DirectTransferProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum DirectTransferProtocolExtensions_EntryExtensionValue {
    #[asn(key = 241)]
    Id_LGW_TransportLayerAddress(TransportLayerAddress),
    #[asn(key = 275)]
    Id_LHN_ID(LHN_ID),
    #[asn(key = 128)]
    Id_RedirectionCompleted(RedirectionCompleted),
    #[asn(key = 129)]
    Id_RedirectionIndication(RedirectionIndication),
    #[asn(key = 273)]
    Id_SIPTO_LGW_TransportLayerAddress(TransportLayerAddress),
    #[asn(key = 202)]
    Id_SubscriberProfileIDforRFP(SubscriberProfileIDforRFP),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DirectTransferProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: DirectTransferProtocolExtensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DirectTransferProtocolExtensions(pub Vec<DirectTransferProtocolExtensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DirectTransferInformationItem_RANAP_RelocInfIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DirectTransferInformationItem_RANAP_RelocInfIE_Extensions(
    pub Vec<DirectTransferInformationItem_RANAP_RelocInfIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum DirectTransferInformationList_RANAP_RelocInf_Entry_EntryValue {
    #[asn(key = 80)]
    Id_DirectTransferInformationItem_RANAP_RelocInf(DirectTransferInformationItem_RANAP_RelocInf),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DirectTransferInformationList_RANAP_RelocInf_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: DirectTransferInformationList_RANAP_RelocInf_Entry_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct DirectTransferInformationList_RANAP_RelocInf_Entry(
    pub Vec<DirectTransferInformationList_RANAP_RelocInf_Entry_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "20", sz_ub = "20")]
pub struct BIT_STRING_7(pub BitVec<u8, Msb0>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "28", sz_ub = "28")]
pub struct BIT_STRING_8(pub BitVec<u8, Msb0>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "18", sz_ub = "18")]
pub struct BIT_STRING_9(pub BitVec<u8, Msb0>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "21", sz_ub = "21")]
pub struct BIT_STRING_10(pub BitVec<u8, Msb0>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct INTEGER_11(pub u16);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum EUTRANFrequencies_EntryIE_Extensions_EntryExtensionValue {
    #[asn(key = 271)]
    Id_EARFCN_Extended(EARFCN_Extended),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EUTRANFrequencies_EntryIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: EUTRANFrequencies_EntryIE_Extensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EUTRANFrequencies_EntryIE_Extensions(
    pub Vec<EUTRANFrequencies_EntryIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct EUTRANFrequencies_Entry {
    pub earfcn: INTEGER_11,
    #[asn(optional_idx = 0)]
    pub meas_band: Option<MeasBand>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<EUTRANFrequencies_EntryIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EncryptionInformationIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EncryptionInformationIE_Extensions(pub Vec<EncryptionInformationIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum EnhancedRelocationCompleteConfirmProtocolIEs_EntryValue {
    #[asn(key = 35)]
    Id_RAB_FailedList(RAB_FailedList),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EnhancedRelocationCompleteConfirmProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: EnhancedRelocationCompleteConfirmProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct EnhancedRelocationCompleteConfirmProtocolIEs(
    pub Vec<EnhancedRelocationCompleteConfirmProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EnhancedRelocationCompleteConfirmProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EnhancedRelocationCompleteConfirmProtocolExtensions(
    pub Vec<EnhancedRelocationCompleteConfirmProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum EnhancedRelocationCompleteFailureProtocolIEs_EntryValue {
    #[asn(key = 4)]
    Id_Cause(Cause),
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EnhancedRelocationCompleteFailureProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: EnhancedRelocationCompleteFailureProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct EnhancedRelocationCompleteFailureProtocolIEs(
    pub Vec<EnhancedRelocationCompleteFailureProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EnhancedRelocationCompleteFailureProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EnhancedRelocationCompleteFailureProtocolExtensions(
    pub Vec<EnhancedRelocationCompleteFailureProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum EnhancedRelocationCompleteRequestProtocolIEs_EntryValue {
    #[asn(key = 79)]
    Id_IuSigConId(IuSignallingConnectionIdentifier),
    #[asn(key = 196)]
    Id_OldIuSigConId(IuSignallingConnectionIdentifier),
    #[asn(key = 188)]
    Id_RAB_SetupList_EnhancedRelocCompleteReq(RAB_SetupList_EnhancedRelocCompleteReq),
    #[asn(key = 223)]
    Id_Relocation_SourceExtendedRNC_ID(ExtendedRNC_ID),
    #[asn(key = 222)]
    Id_Relocation_SourceRNC_ID(GlobalRNC_ID),
    #[asn(key = 213)]
    Id_Relocation_TargetExtendedRNC_ID(ExtendedRNC_ID),
    #[asn(key = 212)]
    Id_Relocation_TargetRNC_ID(GlobalRNC_ID),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EnhancedRelocationCompleteRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: EnhancedRelocationCompleteRequestProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct EnhancedRelocationCompleteRequestProtocolIEs(
    pub Vec<EnhancedRelocationCompleteRequestProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum EnhancedRelocationCompleteRequestProtocolExtensions_EntryExtensionValue {
    #[asn(key = 203)]
    Id_CSG_Id(CSG_Id),
    #[asn(key = 235)]
    Id_Cell_Access_Mode(Cell_Access_Mode),
    #[asn(key = 5)]
    Id_ChosenEncryptionAlgorithm(ChosenEncryptionAlgorithm),
    #[asn(key = 6)]
    Id_ChosenIntegrityProtectionAlgorithm(ChosenIntegrityProtectionAlgorithm),
    #[asn(key = 250)]
    Id_HigherBitratesThan16MbpsFlag(HigherBitratesThan16MbpsFlag),
    #[asn(key = 275)]
    Id_LHN_ID(LHN_ID),
    #[asn(key = 262)]
    Id_Tunnel_Information_for_BBF(TunnelInformation),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EnhancedRelocationCompleteRequestProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: EnhancedRelocationCompleteRequestProtocolExtensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EnhancedRelocationCompleteRequestProtocolExtensions(
    pub Vec<EnhancedRelocationCompleteRequestProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum EnhancedRelocationCompleteResponseProtocolIEs_EntryValue {
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 190)]
    Id_RAB_SetupList_EnhancedRelocCompleteRes(RAB_SetupList_EnhancedRelocCompleteRes),
    #[asn(key = 210)]
    Id_RAB_ToBeReleasedList_EnhancedRelocCompleteRes(RAB_ToBeReleasedList_EnhancedRelocCompleteRes),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EnhancedRelocationCompleteResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: EnhancedRelocationCompleteResponseProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct EnhancedRelocationCompleteResponseProtocolIEs(
    pub Vec<EnhancedRelocationCompleteResponseProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum EnhancedRelocationCompleteResponseProtocolExtensions_EntryExtensionValue {
    #[asn(key = 234)]
    Id_CSG_Membership_Status(CSG_Membership_Status),
    #[asn(key = 239)]
    Id_MSISDN(MSISDN),
    #[asn(key = 233)]
    Id_UE_AggregateMaximumBitRate(UE_AggregateMaximumBitRate),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EnhancedRelocationCompleteResponseProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: EnhancedRelocationCompleteResponseProtocolExtensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EnhancedRelocationCompleteResponseProtocolExtensions(
    pub Vec<EnhancedRelocationCompleteResponseProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum ErrorIndicationProtocolIEs_EntryValue {
    #[asn(key = 3)]
    Id_CN_DomainIndicator(CN_DomainIndicator),
    #[asn(key = 4)]
    Id_Cause(Cause),
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 86)]
    Id_GlobalRNC_ID(GlobalRNC_ID),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ErrorIndicationProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ErrorIndicationProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ErrorIndicationProtocolIEs(pub Vec<ErrorIndicationProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum ErrorIndicationProtocolExtensions_EntryExtensionValue {
    #[asn(key = 171)]
    Id_ExtendedRNC_ID(ExtendedRNC_ID),
    #[asn(key = 96)]
    Id_GlobalCN_ID(GlobalCN_ID),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ErrorIndicationProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: ErrorIndicationProtocolExtensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ErrorIndicationProtocolExtensions(pub Vec<ErrorIndicationProtocolExtensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "-120", ub = "165")]
pub struct INTEGER_12(pub i8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "-120", ub = "-25")]
pub struct INTEGER_13(pub i8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum ForwardSRNS_ContextProtocolIEs_EntryValue {
    #[asn(key = 25)]
    Id_RAB_ContextList(RAB_ContextList),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ForwardSRNS_ContextProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ForwardSRNS_ContextProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ForwardSRNS_ContextProtocolIEs(pub Vec<ForwardSRNS_ContextProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum ForwardSRNS_ContextProtocolExtensions_EntryExtensionValue {
    #[asn(key = 103)]
    Id_SourceRNC_PDCP_context_info(RRC_Container),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ForwardSRNS_ContextProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: ForwardSRNS_ContextProtocolExtensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ForwardSRNS_ContextProtocolExtensions(
    pub Vec<ForwardSRNS_ContextProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct ENUMERATED_14(pub u8);
impl ENUMERATED_14 {
    pub const HEIGHT: u8 = 0u8;
    pub const DEPTH: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "32767")]
pub struct INTEGER_15(pub u16);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct INTEGER_16(pub u16);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct INTEGER_17(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "179")]
pub struct INTEGER_18(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "179")]
pub struct INTEGER_19(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct INTEGER_20(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GA_EllipsoidArcIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GA_EllipsoidArcIE_Extensions(pub Vec<GA_EllipsoidArcIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GA_PointIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GA_PointIE_Extensions(pub Vec<GA_PointIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GA_PointWithAltitudeIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GA_PointWithAltitudeIE_Extensions(pub Vec<GA_PointWithAltitudeIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct INTEGER_21(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct INTEGER_22(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GA_PointWithAltitudeAndUncertaintyEllipsoidIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GA_PointWithAltitudeAndUncertaintyEllipsoidIE_Extensions(
    pub Vec<GA_PointWithAltitudeAndUncertaintyEllipsoidIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GA_PointWithUnCertaintyIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GA_PointWithUnCertaintyIE_Extensions(
    pub Vec<GA_PointWithUnCertaintyIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct INTEGER_23(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct INTEGER_24(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GA_PointWithUnCertaintyEllipseIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GA_PointWithUnCertaintyEllipseIE_Extensions(
    pub Vec<GA_PointWithUnCertaintyEllipseIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GA_Polygon_EntryIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GA_Polygon_EntryIE_Extensions(pub Vec<GA_Polygon_EntryIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GA_Polygon_Entry {
    pub geographical_coordinates: GeographicalCoordinates,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GA_Polygon_EntryIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct INTEGER_25(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct INTEGER_26(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "179")]
pub struct INTEGER_27(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GERAN_Cell_IDIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GERAN_Cell_IDIE_Extensions(pub Vec<GERAN_Cell_IDIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GERAN_Iumode_RAB_Failed_RABAssgntResponse_ItemIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GERAN_Iumode_RAB_Failed_RABAssgntResponse_ItemIE_Extensions(
    pub Vec<GERAN_Iumode_RAB_Failed_RABAssgntResponse_ItemIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum GERAN_Iumode_RAB_FailedList_RABAssgntResponse_Entry_EntryValue {
    #[asn(key = 109)]
    Id_GERAN_Iumode_RAB_Failed_RABAssgntResponse_Item(
        GERAN_Iumode_RAB_Failed_RABAssgntResponse_Item,
    ),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GERAN_Iumode_RAB_FailedList_RABAssgntResponse_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: GERAN_Iumode_RAB_FailedList_RABAssgntResponse_Entry_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct GERAN_Iumode_RAB_FailedList_RABAssgntResponse_Entry(
    pub Vec<GERAN_Iumode_RAB_FailedList_RABAssgntResponse_Entry_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct ENUMERATED_28(pub u8);
impl ENUMERATED_28 {
    pub const NORTH: u8 = 0u8;
    pub const SOUTH: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "8388607")]
pub struct INTEGER_29(pub u32);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "-8388608", ub = "8388607")]
pub struct INTEGER_30(pub i32);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GeographicalCoordinatesIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GeographicalCoordinatesIE_Extensions(
    pub Vec<GeographicalCoordinatesIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "359")]
pub struct INTEGER_31(pub u16);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "2047")]
pub struct INTEGER_32(pub u16);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HorizontalVelocityIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct HorizontalVelocityIE_Extensions(pub Vec<HorizontalVelocityIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct INTEGER_33(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HorizontalVelocityWithUncertaintyIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct HorizontalVelocityWithUncertaintyIE_Extensions(
    pub Vec<HorizontalVelocityWithUncertaintyIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HorizontalWithVerticalVelocityIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct HorizontalWithVerticalVelocityIE_Extensions(
    pub Vec<HorizontalWithVerticalVelocityIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct INTEGER_34(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct INTEGER_35(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HorizontalWithVerticalVelocityAndUncertaintyIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct HorizontalWithVerticalVelocityAndUncertaintyIE_Extensions(
    pub Vec<HorizontalWithVerticalVelocityAndUncertaintyIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "7", sz_ub = "7")]
pub struct BIT_STRING_36(pub BitVec<u8, Msb0>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IMEIGroupIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct IMEIGroupIE_Extensions(pub Vec<IMEIGroupIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "7", sz_ub = "7")]
pub struct BIT_STRING_37(pub BitVec<u8, Msb0>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IMEISVGroupIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct IMEISVGroupIE_Extensions(pub Vec<IMEISVGroupIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "97")]
pub struct INTEGER_38(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "34")]
pub struct INTEGER_39(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum IRAT_Measurement_ConfigurationIE_Extensions_EntryExtensionValue {
    #[asn(key = 279)]
    Id_RSRQ_Extension(RSRQ_Extension),
    #[asn(key = 278)]
    Id_RSRQ_Type(RSRQ_Type),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IRAT_Measurement_ConfigurationIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: IRAT_Measurement_ConfigurationIE_Extensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct IRAT_Measurement_ConfigurationIE_Extensions(
    pub Vec<IRAT_Measurement_ConfigurationIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "100")]
pub struct INTEGER_40(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IRATmeasurementParametersIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct IRATmeasurementParametersIE_Extensions(
    pub Vec<IRATmeasurementParametersIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum ImmediateMDTIE_Extensions_EntryExtensionValue {
    #[asn(key = 265)]
    Id_M4Report(M4Report),
    #[asn(key = 266)]
    Id_M5Report(M5Report),
    #[asn(key = 267)]
    Id_M6Report(M6Report),
    #[asn(key = 268)]
    Id_M7Report(M7Report),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ImmediateMDTIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: ImmediateMDTIE_Extensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ImmediateMDTIE_Extensions(pub Vec<ImmediateMDTIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum InformationTransferConfirmationProtocolIEs_EntryValue {
    #[asn(key = 3)]
    Id_CN_DomainIndicator(CN_DomainIndicator),
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 86)]
    Id_GlobalRNC_ID(GlobalRNC_ID),
    #[asn(key = 104)]
    Id_InformationTransferID(InformationTransferID),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InformationTransferConfirmationProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: InformationTransferConfirmationProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct InformationTransferConfirmationProtocolIEs(
    pub Vec<InformationTransferConfirmationProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum InformationTransferConfirmationProtocolExtensions_EntryExtensionValue {
    #[asn(key = 171)]
    Id_ExtendedRNC_ID(ExtendedRNC_ID),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InformationTransferConfirmationProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: InformationTransferConfirmationProtocolExtensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct InformationTransferConfirmationProtocolExtensions(
    pub Vec<InformationTransferConfirmationProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum InformationTransferFailureProtocolIEs_EntryValue {
    #[asn(key = 3)]
    Id_CN_DomainIndicator(CN_DomainIndicator),
    #[asn(key = 4)]
    Id_Cause(Cause),
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 86)]
    Id_GlobalRNC_ID(GlobalRNC_ID),
    #[asn(key = 104)]
    Id_InformationTransferID(InformationTransferID),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InformationTransferFailureProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: InformationTransferFailureProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct InformationTransferFailureProtocolIEs(
    pub Vec<InformationTransferFailureProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum InformationTransferFailureProtocolExtensions_EntryExtensionValue {
    #[asn(key = 171)]
    Id_ExtendedRNC_ID(ExtendedRNC_ID),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InformationTransferFailureProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: InformationTransferFailureProtocolExtensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct InformationTransferFailureProtocolExtensions(
    pub Vec<InformationTransferFailureProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum InformationTransferIndicationProtocolIEs_EntryValue {
    #[asn(key = 3)]
    Id_CN_DomainIndicator(CN_DomainIndicator),
    #[asn(key = 96)]
    Id_GlobalCN_ID(GlobalCN_ID),
    #[asn(key = 104)]
    Id_InformationTransferID(InformationTransferID),
    #[asn(key = 106)]
    Id_ProvidedData(ProvidedData),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InformationTransferIndicationProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: InformationTransferIndicationProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct InformationTransferIndicationProtocolIEs(
    pub Vec<InformationTransferIndicationProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InformationTransferIndicationProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct InformationTransferIndicationProtocolExtensions(
    pub Vec<InformationTransferIndicationProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum InitialUE_MessageProtocolIEs_EntryValue {
    #[asn(key = 3)]
    Id_CN_DomainIndicator(CN_DomainIndicator),
    #[asn(key = 86)]
    Id_GlobalRNC_ID(GlobalRNC_ID),
    #[asn(key = 79)]
    Id_IuSigConId(IuSignallingConnectionIdentifier),
    #[asn(key = 15)]
    Id_LAI(LAI),
    #[asn(key = 16)]
    Id_NAS_PDU(NAS_PDU),
    #[asn(key = 55)]
    Id_RAC(RAC),
    #[asn(key = 58)]
    Id_SAI(SAI),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InitialUE_MessageProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: InitialUE_MessageProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct InitialUE_MessageProtocolIEs(pub Vec<InitialUE_MessageProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum InitialUE_MessageProtocolExtensions_EntryExtensionValue {
    #[asn(key = 203)]
    Id_CSG_Id(CSG_Id),
    #[asn(key = 235)]
    Id_Cell_Access_Mode(Cell_Access_Mode),
    #[asn(key = 291)]
    Id_DCN_ID(DCN_ID),
    #[asn(key = 171)]
    Id_ExtendedRNC_ID(ExtendedRNC_ID),
    #[asn(key = 108)]
    Id_GERAN_Classmark(GERAN_Classmark),
    #[asn(key = 250)]
    Id_HigherBitratesThan16MbpsFlag(HigherBitratesThan16MbpsFlag),
    #[asn(key = 241)]
    Id_LGW_TransportLayerAddress(TransportLayerAddress),
    #[asn(key = 275)]
    Id_LHN_ID(LHN_ID),
    #[asn(key = 130)]
    Id_NAS_SequenceNumber(NAS_SequenceNumber),
    #[asn(key = 23)]
    Id_PermanentNAS_UE_ID(PermanentNAS_UE_ID),
    #[asn(key = 166)]
    Id_RedirectAttemptFlag(RedirectAttemptFlag),
    #[asn(key = 286)]
    Id_SGSN_Group_Identity(SGSN_Group_Identity),
    #[asn(key = 273)]
    Id_SIPTO_LGW_TransportLayerAddress(TransportLayerAddress),
    #[asn(key = 127)]
    Id_SelectedPLMN_ID(PLMNidentity),
    #[asn(key = 262)]
    Id_Tunnel_Information_for_BBF(TunnelInformation),
    #[asn(key = 294)]
    Id_UE_Application_Layer_Measurement_Capability(UE_Application_Layer_Measurement_Capability),
    #[asn(key = 290)]
    Id_UE_Usage_Type(UE_Usage_Type),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InitialUE_MessageProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: InitialUE_MessageProtocolExtensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct InitialUE_MessageProtocolExtensions(pub Vec<InitialUE_MessageProtocolExtensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum InitiatingMessageValue {
    #[asn(key = 26)]
    Id_CN_DeactivateTrace(CN_DeactivateTrace),
    #[asn(key = 16)]
    Id_CN_InvokeTrace(CN_InvokeTrace),
    #[asn(key = 15)]
    Id_CommonID(CommonID),
    #[asn(key = 7)]
    Id_DataVolumeReport(DataVolumeReportRequest),
    #[asn(key = 34)]
    Id_DirectInformationTransfer(DirectInformationTransfer),
    #[asn(key = 20)]
    Id_DirectTransfer(DirectTransfer),
    #[asn(key = 22)]
    Id_ErrorIndication(ErrorIndication),
    #[asn(key = 24)]
    Id_ForwardSRNS_Context(ForwardSRNS_Context),
    #[asn(key = 31)]
    Id_InformationTransfer(InformationTransferIndication),
    #[asn(key = 19)]
    Id_InitialUE_Message(InitialUE_Message),
    #[asn(key = 1)]
    Id_Iu_Release(Iu_ReleaseCommand),
    #[asn(key = 11)]
    Id_Iu_ReleaseRequest(Iu_ReleaseRequest),
    #[asn(key = 30)]
    Id_LocationRelatedData(LocationRelatedDataRequest),
    #[asn(key = 18)]
    Id_LocationReport(LocationReport),
    #[asn(key = 17)]
    Id_LocationReportingControl(LocationReportingControl),
    #[asn(key = 40)]
    Id_MBMSCNDe_Registration_Procedure(MBMSCNDe_RegistrationRequest),
    #[asn(key = 41)]
    Id_MBMSRABEstablishmentIndication(MBMSRABEstablishmentIndication),
    #[asn(key = 42)]
    Id_MBMSRABRelease(MBMSRABReleaseRequest),
    #[asn(key = 39)]
    Id_MBMSRegistration(MBMSRegistrationRequest),
    #[asn(key = 35)]
    Id_MBMSSessionStart(MBMSSessionStart),
    #[asn(key = 37)]
    Id_MBMSSessionStop(MBMSSessionStop),
    #[asn(key = 36)]
    Id_MBMSSessionUpdate(MBMSSessionUpdate),
    #[asn(key = 38)]
    Id_MBMSUELinking(MBMSUELinkingRequest),
    #[asn(key = 21)]
    Id_OverloadControl(Overload),
    #[asn(key = 14)]
    Id_Paging(Paging),
    #[asn(key = 0)]
    Id_RAB_Assignment(RAB_AssignmentRequest),
    #[asn(key = 29)]
    Id_RAB_ModifyRequest(RAB_ModifyRequest),
    #[asn(key = 10)]
    Id_RAB_ReleaseRequest(RAB_ReleaseRequest),
    #[asn(key = 28)]
    Id_RANAP_Relocation(RANAP_RelocationInformation),
    #[asn(key = 45)]
    Id_RANAPenhancedRelocation(RANAP_EnhancedRelocationInformationRequest),
    #[asn(key = 4)]
    Id_RelocationCancel(RelocationCancel),
    #[asn(key = 13)]
    Id_RelocationComplete(RelocationComplete),
    #[asn(key = 12)]
    Id_RelocationDetect(RelocationDetect),
    #[asn(key = 2)]
    Id_RelocationPreparation(RelocationRequired),
    #[asn(key = 3)]
    Id_RelocationResourceAllocation(RelocationRequest),
    #[asn(key = 49)]
    Id_RerouteNASRequest(RerouteNASRequest),
    #[asn(key = 9)]
    Id_Reset(Reset),
    #[asn(key = 27)]
    Id_ResetResource(ResetResource),
    #[asn(key = 5)]
    Id_SRNS_ContextTransfer(SRNS_ContextRequest),
    #[asn(key = 23)]
    Id_SRNS_DataForward(SRNS_DataForwardCommand),
    #[asn(key = 46)]
    Id_SRVCCPreparation(SRVCC_CSKeysRequest),
    #[asn(key = 6)]
    Id_SecurityModeControl(SecurityModeCommand),
    #[asn(key = 32)]
    Id_UESpecificInformation(UESpecificInformationIndication),
    #[asn(key = 47)]
    Id_UeRadioCapabilityMatch(UeRadioCapabilityMatchRequest),
    #[asn(key = 48)]
    Id_UeRegistrationQuery(UeRegistrationQueryRequest),
    #[asn(key = 33)]
    Id_UplinkInformationExchange(UplinkInformationExchangeRequest),
    #[asn(key = 43)]
    Id_enhancedRelocationComplete(EnhancedRelocationCompleteRequest),
    #[asn(key = 44)]
    Id_enhancedRelocationCompleteConfirm(EnhancedRelocationCompleteConfirm),
    #[asn(key = 25)]
    Id_privateMessage(PrivateMessage),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IntegrityProtectionInformationIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct IntegrityProtectionInformationIE_Extensions(
    pub Vec<IntegrityProtectionInformationIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InterSystemInformation_TransparentContainerIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct InterSystemInformation_TransparentContainerIE_Extensions(
    pub Vec<InterSystemInformation_TransparentContainerIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "4")]
pub struct ENUMERATED_41(pub u8);
impl ENUMERATED_41 {
    pub const IU_CS: u8 = 0u8;
    pub const IU_PS: u8 = 1u8;
    pub const IUR: u8 = 2u8;
    pub const IUB: u8 = 3u8;
    pub const UU: u8 = 4u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InterfacesToTraceItemIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct InterfacesToTraceItemIE_Extensions(pub Vec<InterfacesToTraceItemIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum Iu_ReleaseCommandProtocolIEs_EntryValue {
    #[asn(key = 4)]
    Id_Cause(Cause),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Iu_ReleaseCommandProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: Iu_ReleaseCommandProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct Iu_ReleaseCommandProtocolIEs(pub Vec<Iu_ReleaseCommandProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum Iu_ReleaseCommandProtocolExtensions_EntryExtensionValue {
    #[asn(key = 252)]
    Id_End_Of_CSFB(End_Of_CSFB),
    #[asn(key = 277)]
    Id_LastE_UTRANPLMNIdentity(PLMNidentity),
    #[asn(key = 254)]
    Id_Out_Of_UTRAN(Out_Of_UTRAN),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Iu_ReleaseCommandProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: Iu_ReleaseCommandProtocolExtensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Iu_ReleaseCommandProtocolExtensions(pub Vec<Iu_ReleaseCommandProtocolExtensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum Iu_ReleaseCompleteProtocolIEs_EntryValue {
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 31)]
    Id_RAB_DataVolumeReportList(RAB_DataVolumeReportList),
    #[asn(key = 44)]
    Id_RAB_ReleasedList_IuRelComp(RAB_ReleasedList_IuRelComp),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Iu_ReleaseCompleteProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: Iu_ReleaseCompleteProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct Iu_ReleaseCompleteProtocolIEs(pub Vec<Iu_ReleaseCompleteProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Iu_ReleaseCompleteProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Iu_ReleaseCompleteProtocolExtensions(
    pub Vec<Iu_ReleaseCompleteProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum Iu_ReleaseRequestProtocolIEs_EntryValue {
    #[asn(key = 4)]
    Id_Cause(Cause),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Iu_ReleaseRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: Iu_ReleaseRequestProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct Iu_ReleaseRequestProtocolIEs(pub Vec<Iu_ReleaseRequestProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Iu_ReleaseRequestProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Iu_ReleaseRequestProtocolExtensions(pub Vec<Iu_ReleaseRequestProtocolExtensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct JoinedMBMSBearerService_IEs_EntryIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct JoinedMBMSBearerService_IEs_EntryIE_Extensions(
    pub Vec<JoinedMBMSBearerService_IEs_EntryIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct JoinedMBMSBearerService_IEs_Entry {
    pub tmgi: TMGI,
    pub mbms_ptp_rab_id: MBMS_PTP_RAB_ID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<JoinedMBMSBearerService_IEs_EntryIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LA_LIST_EntryIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LA_LIST_EntryIE_Extensions(pub Vec<LA_LIST_EntryIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LA_LIST_Entry {
    pub lac: LAC,
    pub list_of_sn_as: ListOF_SNAs,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<LA_LIST_EntryIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LABasedIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LABasedIE_Extensions(pub Vec<LABasedIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LAIIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LAIIE_Extensions(pub Vec<LAIIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "32767")]
pub struct INTEGER_42(pub u16);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LastKnownServiceAreaIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LastKnownServiceAreaIE_Extensions(pub Vec<LastKnownServiceAreaIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum LastVisitedUTRANCell_ItemIE_Extensions_EntryExtensionValue {
    #[asn(key = 257)]
    Id_HO_Cause(Cause),
    #[asn(key = 253)]
    Id_Time_UE_StayedInCell_EnhancedGranularity(Time_UE_StayedInCell_EnhancedGranularity),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LastVisitedUTRANCell_ItemIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: LastVisitedUTRANCell_ItemIE_Extensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LastVisitedUTRANCell_ItemIE_Extensions(
    pub Vec<LastVisitedUTRANCell_ItemIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LeftMBMSBearerService_IEs_EntryIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LeftMBMSBearerService_IEs_EntryIE_Extensions(
    pub Vec<LeftMBMSBearerService_IEs_EntryIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LeftMBMSBearerService_IEs_Entry {
    pub tmgi: TMGI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<LeftMBMSBearerService_IEs_EntryIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum LocationRelatedDataFailureProtocolIEs_EntryValue {
    #[asn(key = 4)]
    Id_Cause(Cause),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationRelatedDataFailureProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: LocationRelatedDataFailureProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct LocationRelatedDataFailureProtocolIEs(
    pub Vec<LocationRelatedDataFailureProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum LocationRelatedDataFailureProtocolExtensions_EntryExtensionValue {
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationRelatedDataFailureProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: LocationRelatedDataFailureProtocolExtensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LocationRelatedDataFailureProtocolExtensions(
    pub Vec<LocationRelatedDataFailureProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum LocationRelatedDataRequestProtocolIEs_EntryValue {
    #[asn(key = 95)]
    Id_LocationRelatedDataRequestType(LocationRelatedDataRequestType),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationRelatedDataRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: LocationRelatedDataRequestProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct LocationRelatedDataRequestProtocolIEs(
    pub Vec<LocationRelatedDataRequestProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum LocationRelatedDataRequestProtocolExtensions_EntryExtensionValue {
    #[asn(key = 115)]
    Id_LocationRelatedDataRequestTypeSpecificToGERANIuMode(
        LocationRelatedDataRequestTypeSpecificToGERANIuMode,
    ),
    #[asn(key = 185)]
    Id_RequestedGANSSAssistanceData(RequestedGANSSAssistanceData),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationRelatedDataRequestProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: LocationRelatedDataRequestProtocolExtensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LocationRelatedDataRequestProtocolExtensions(
    pub Vec<LocationRelatedDataRequestProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum LocationRelatedDataResponseProtocolIEs_EntryValue {
    #[asn(key = 94)]
    Id_BroadcastAssistanceDataDecipheringKeys(BroadcastAssistanceDataDecipheringKeys),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationRelatedDataResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: LocationRelatedDataResponseProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct LocationRelatedDataResponseProtocolIEs(
    pub Vec<LocationRelatedDataResponseProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum LocationRelatedDataResponseProtocolExtensions_EntryExtensionValue {
    #[asn(key = 186)]
    Id_BroadcastGANSSAssistanceDataDecipheringKeys(BroadcastAssistanceDataDecipheringKeys),
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationRelatedDataResponseProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: LocationRelatedDataResponseProtocolExtensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LocationRelatedDataResponseProtocolExtensions(
    pub Vec<LocationRelatedDataResponseProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum LocationReportProtocolIEs_EntryValue {
    #[asn(key = 0)]
    Id_AreaIdentity(AreaIdentity),
    #[asn(key = 4)]
    Id_Cause(Cause),
    #[asn(key = 57)]
    Id_RequestType(RequestType),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationReportProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: LocationReportProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct LocationReportProtocolIEs(pub Vec<LocationReportProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum LocationReportProtocolExtensions_EntryExtensionValue {
    #[asn(key = 122)]
    Id_AccuracyFulfilmentIndicator(AccuracyFulfilmentIndicator),
    #[asn(key = 283)]
    Id_BarometricPressure(BarometricPressure),
    #[asn(key = 285)]
    Id_CivicAddress(CivicAddress),
    #[asn(key = 97)]
    Id_LastKnownServiceArea(LastKnownServiceArea),
    #[asn(key = 119)]
    Id_PositionData(PositionData),
    #[asn(key = 120)]
    Id_PositionDataSpecificToGERANIuMode(PositionDataSpecificToGERANIuMode),
    #[asn(key = 165)]
    Id_VelocityEstimate(VelocityEstimate),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationReportProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: LocationReportProtocolExtensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LocationReportProtocolExtensions(pub Vec<LocationReportProtocolExtensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum LocationReportingControlProtocolIEs_EntryValue {
    #[asn(key = 57)]
    Id_RequestType(RequestType),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationReportingControlProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: LocationReportingControlProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct LocationReportingControlProtocolIEs(pub Vec<LocationReportingControlProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum LocationReportingControlProtocolExtensions_EntryExtensionValue {
    #[asn(key = 114)]
    Id_ClientType(ClientType),
    #[asn(key = 164)]
    Id_IncludeVelocity(IncludeVelocity),
    #[asn(key = 168)]
    Id_PeriodicLocationInfo(PeriodicLocationInfo),
    #[asn(key = 113)]
    Id_PositioningPriority(PositioningPriority),
    #[asn(key = 112)]
    Id_ResponseTime(ResponseTime),
    #[asn(key = 111)]
    Id_VerticalAccuracyCode(VerticalAccuracyCode),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationReportingControlProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: LocationReportingControlProtocolExtensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LocationReportingControlProtocolExtensions(
    pub Vec<LocationReportingControlProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationReportingTransferInformationIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LocationReportingTransferInformationIE_Extensions(
    pub Vec<LocationReportingTransferInformationIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LoggedMDTIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LoggedMDTIE_Extensions(pub Vec<LoggedMDTIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M4_Collection_ParametersIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct M4_Collection_ParametersIE_Extensions(
    pub Vec<M4_Collection_ParametersIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "NULL")]
pub struct NULL_43;

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "NULL")]
pub struct NULL_44;

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M6ReportIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct M6ReportIE_Extensions(pub Vec<M6ReportIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M7ReportIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct M7ReportIE_Extensions(pub Vec<M7ReportIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MBMSCNDe_RegistrationRequestProtocolIEs_EntryValue {
    #[asn(key = 96)]
    Id_GlobalCN_ID(GlobalCN_ID),
    #[asn(key = 153)]
    Id_TMGI(TMGI),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSCNDe_RegistrationRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSCNDe_RegistrationRequestProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSCNDe_RegistrationRequestProtocolIEs(
    pub Vec<MBMSCNDe_RegistrationRequestProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSCNDe_RegistrationRequestProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSCNDe_RegistrationRequestProtocolExtensions(
    pub Vec<MBMSCNDe_RegistrationRequestProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MBMSCNDe_RegistrationResponseProtocolIEs_EntryValue {
    #[asn(key = 4)]
    Id_Cause(Cause),
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 86)]
    Id_GlobalRNC_ID(GlobalRNC_ID),
    #[asn(key = 153)]
    Id_TMGI(TMGI),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSCNDe_RegistrationResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSCNDe_RegistrationResponseProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSCNDe_RegistrationResponseProtocolIEs(
    pub Vec<MBMSCNDe_RegistrationResponseProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MBMSCNDe_RegistrationResponseProtocolExtensions_EntryExtensionValue {
    #[asn(key = 171)]
    Id_ExtendedRNC_ID(ExtendedRNC_ID),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSCNDe_RegistrationResponseProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: MBMSCNDe_RegistrationResponseProtocolExtensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSCNDe_RegistrationResponseProtocolExtensions(
    pub Vec<MBMSCNDe_RegistrationResponseProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSIPMulticastAddressandAPNlistIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSIPMulticastAddressandAPNlistIE_Extensions(
    pub Vec<MBMSIPMulticastAddressandAPNlistIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MBMSRABEstablishmentIndicationProtocolIEs_EntryValue {
    #[asn(key = 154)]
    Id_TransportLayerInformation(TransportLayerInformation),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSRABEstablishmentIndicationProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSRABEstablishmentIndicationProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSRABEstablishmentIndicationProtocolIEs(
    pub Vec<MBMSRABEstablishmentIndicationProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSRABEstablishmentIndicationProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSRABEstablishmentIndicationProtocolExtensions(
    pub Vec<MBMSRABEstablishmentIndicationProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MBMSRABReleaseProtocolIEs_EntryValue {
    #[asn(key = 4)]
    Id_Cause(Cause),
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSRABReleaseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSRABReleaseProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSRABReleaseProtocolIEs(pub Vec<MBMSRABReleaseProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSRABReleaseProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSRABReleaseProtocolExtensions(pub Vec<MBMSRABReleaseProtocolExtensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MBMSRABReleaseFailureProtocolIEs_EntryValue {
    #[asn(key = 4)]
    Id_Cause(Cause),
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSRABReleaseFailureProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSRABReleaseFailureProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSRABReleaseFailureProtocolIEs(pub Vec<MBMSRABReleaseFailureProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSRABReleaseFailureProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSRABReleaseFailureProtocolExtensions(
    pub Vec<MBMSRABReleaseFailureProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MBMSRABReleaseRequestProtocolIEs_EntryValue {
    #[asn(key = 4)]
    Id_Cause(Cause),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSRABReleaseRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSRABReleaseRequestProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSRABReleaseRequestProtocolIEs(pub Vec<MBMSRABReleaseRequestProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSRABReleaseRequestProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSRABReleaseRequestProtocolExtensions(
    pub Vec<MBMSRABReleaseRequestProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MBMSRegistrationFailureProtocolIEs_EntryValue {
    #[asn(key = 4)]
    Id_Cause(Cause),
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 96)]
    Id_GlobalCN_ID(GlobalCN_ID),
    #[asn(key = 153)]
    Id_TMGI(TMGI),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSRegistrationFailureProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSRegistrationFailureProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSRegistrationFailureProtocolIEs(pub Vec<MBMSRegistrationFailureProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSRegistrationFailureProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSRegistrationFailureProtocolExtensions(
    pub Vec<MBMSRegistrationFailureProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MBMSRegistrationRequestProtocolIEs_EntryValue {
    #[asn(key = 132)]
    Id_APN(APN),
    #[asn(key = 86)]
    Id_GlobalRNC_ID(GlobalRNC_ID),
    #[asn(key = 140)]
    Id_IPMulticastAddress(IPMulticastAddress),
    #[asn(key = 151)]
    Id_MBMSRegistrationRequestType(MBMSRegistrationRequestType),
    #[asn(key = 153)]
    Id_TMGI(TMGI),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSRegistrationRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSRegistrationRequestProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSRegistrationRequestProtocolIEs(pub Vec<MBMSRegistrationRequestProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MBMSRegistrationRequestProtocolExtensions_EntryExtensionValue {
    #[asn(key = 171)]
    Id_ExtendedRNC_ID(ExtendedRNC_ID),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSRegistrationRequestProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: MBMSRegistrationRequestProtocolExtensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSRegistrationRequestProtocolExtensions(
    pub Vec<MBMSRegistrationRequestProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MBMSRegistrationResponseProtocolIEs_EntryValue {
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 96)]
    Id_GlobalCN_ID(GlobalCN_ID),
    #[asn(key = 153)]
    Id_TMGI(TMGI),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSRegistrationResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSRegistrationResponseProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSRegistrationResponseProtocolIEs(pub Vec<MBMSRegistrationResponseProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSRegistrationResponseProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSRegistrationResponseProtocolExtensions(
    pub Vec<MBMSRegistrationResponseProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MBMSSessionStartProtocolIEs_EntryValue {
    #[asn(key = 135)]
    Id_FrequenceLayerConvergenceFlag(FrequenceLayerConvergenceFlag),
    #[asn(key = 96)]
    Id_GlobalCN_ID(GlobalCN_ID),
    #[asn(key = 79)]
    Id_IuSigConId(IuSignallingConnectionIdentifier),
    #[asn(key = 143)]
    Id_MBMSBearerServiceType(MBMSBearerServiceType),
    #[asn(key = 145)]
    Id_MBMSServiceArea(MBMSServiceArea),
    #[asn(key = 146)]
    Id_MBMSSessionDuration(MBMSSessionDuration),
    #[asn(key = 147)]
    Id_MBMSSessionIdentity(MBMSSessionIdentity),
    #[asn(key = 157)]
    Id_MBMSSessionRepetitionNumber(MBMSSessionRepetitionNumber),
    #[asn(key = 148)]
    Id_PDP_TypeInformation(PDP_TypeInformation),
    #[asn(key = 149)]
    Id_RAB_Parameters(RAB_Parameters),
    #[asn(key = 150)]
    Id_RAListofIdleModeUEs(RAListofIdleModeUEs),
    #[asn(key = 153)]
    Id_TMGI(TMGI),
    #[asn(key = 163)]
    Id_TimeToMBMSDataTransfer(TimeToMBMSDataTransfer),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionStartProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSSessionStartProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSSessionStartProtocolIEs(pub Vec<MBMSSessionStartProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MBMSSessionStartProtocolExtensions_EntryExtensionValue {
    #[asn(key = 169)]
    Id_MBMSCountingInformation(MBMSCountingInformation),
    #[asn(key = 201)]
    Id_MBMSSynchronisationInformation(MBMSSynchronisationInformation),
    #[asn(key = 238)]
    Id_PDP_TypeInformation_extension(PDP_TypeInformation_extension),
    #[asn(key = 276)]
    Id_Session_Re_establishment_Indicator(Session_Re_establishment_Indicator),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionStartProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: MBMSSessionStartProtocolExtensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSSessionStartProtocolExtensions(pub Vec<MBMSSessionStartProtocolExtensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MBMSSessionStartFailureProtocolIEs_EntryValue {
    #[asn(key = 4)]
    Id_Cause(Cause),
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionStartFailureProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSSessionStartFailureProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSSessionStartFailureProtocolIEs(pub Vec<MBMSSessionStartFailureProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionStartFailureProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSSessionStartFailureProtocolExtensions(
    pub Vec<MBMSSessionStartFailureProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MBMSSessionStartResponseProtocolIEs_EntryValue {
    #[asn(key = 4)]
    Id_Cause(Cause),
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 154)]
    Id_TransportLayerInformation(TransportLayerInformation),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionStartResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSSessionStartResponseProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSSessionStartResponseProtocolIEs(pub Vec<MBMSSessionStartResponseProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionStartResponseProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSSessionStartResponseProtocolExtensions(
    pub Vec<MBMSSessionStartResponseProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MBMSSessionStopProtocolIEs_EntryValue {
    #[asn(key = 144)]
    Id_MBMSCNDe_Registration(MBMSCNDe_Registration),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionStopProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSSessionStopProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSSessionStopProtocolIEs(pub Vec<MBMSSessionStopProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionStopProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSSessionStopProtocolExtensions(pub Vec<MBMSSessionStopProtocolExtensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MBMSSessionStopResponseProtocolIEs_EntryValue {
    #[asn(key = 4)]
    Id_Cause(Cause),
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionStopResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSSessionStopResponseProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSSessionStopResponseProtocolIEs(pub Vec<MBMSSessionStopResponseProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionStopResponseProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSSessionStopResponseProtocolExtensions(
    pub Vec<MBMSSessionStopResponseProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MBMSSessionUpdateProtocolIEs_EntryValue {
    #[asn(key = 134)]
    Id_DeltaRAListofIdleModeUEs(DeltaRAListofIdleModeUEs),
    #[asn(key = 152)]
    Id_SessionUpdateID(SessionUpdateID),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionUpdateProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSSessionUpdateProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSSessionUpdateProtocolIEs(pub Vec<MBMSSessionUpdateProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionUpdateProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSSessionUpdateProtocolExtensions(pub Vec<MBMSSessionUpdateProtocolExtensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MBMSSessionUpdateFailureProtocolIEs_EntryValue {
    #[asn(key = 4)]
    Id_Cause(Cause),
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 152)]
    Id_SessionUpdateID(SessionUpdateID),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionUpdateFailureProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSSessionUpdateFailureProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSSessionUpdateFailureProtocolIEs(pub Vec<MBMSSessionUpdateFailureProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionUpdateFailureProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSSessionUpdateFailureProtocolExtensions(
    pub Vec<MBMSSessionUpdateFailureProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MBMSSessionUpdateResponseProtocolIEs_EntryValue {
    #[asn(key = 4)]
    Id_Cause(Cause),
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 152)]
    Id_SessionUpdateID(SessionUpdateID),
    #[asn(key = 154)]
    Id_TransportLayerInformation(TransportLayerInformation),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionUpdateResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSSessionUpdateResponseProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSSessionUpdateResponseProtocolIEs(
    pub Vec<MBMSSessionUpdateResponseProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionUpdateResponseProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSSessionUpdateResponseProtocolExtensions(
    pub Vec<MBMSSessionUpdateResponseProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MBMSSynchronisationInformationIE_Extensions_EntryExtensionValue {
    #[asn(key = 236)]
    Id_IP_Source_Address(IPMulticastAddress),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSynchronisationInformationIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: MBMSSynchronisationInformationIE_Extensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSSynchronisationInformationIE_Extensions(
    pub Vec<MBMSSynchronisationInformationIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MBMSUELinkingRequestProtocolIEs_EntryValue {
    #[asn(key = 141)]
    Id_JoinedMBMSBearerServicesList(JoinedMBMSBearerService_IEs),
    #[asn(key = 142)]
    Id_LeftMBMSBearerServicesList(LeftMBMSBearerService_IEs),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSUELinkingRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSUELinkingRequestProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSUELinkingRequestProtocolIEs(pub Vec<MBMSUELinkingRequestProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSUELinkingRequestProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSUELinkingRequestProtocolExtensions(
    pub Vec<MBMSUELinkingRequestProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MBMSUELinkingResponseProtocolIEs_EntryValue {
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 155)]
    Id_UnsuccessfulLinkingList(UnsuccessfulLinking_IEs),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSUELinkingResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSUELinkingResponseProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSUELinkingResponseProtocolIEs(pub Vec<MBMSUELinkingResponseProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSUELinkingResponseProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSUELinkingResponseProtocolExtensions(
    pub Vec<MBMSUELinkingResponseProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MDT_ConfigurationIE_Extensions_EntryExtensionValue {
    #[asn(key = 264)]
    Id_SignallingBasedMDTPLMNList(MDT_PLMN_List),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MDT_ConfigurationIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: MDT_ConfigurationIE_Extensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MDT_ConfigurationIE_Extensions(pub Vec<MDT_ConfigurationIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "NULL")]
pub struct NULL_45;

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MessageStructure_EntryIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MessageStructure_EntryIE_Extensions(pub Vec<MessageStructure_EntryIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct MessageStructure_Entry {
    pub ie_id: ProtocolIE_ID,
    #[asn(optional_idx = 0)]
    pub repetition_number: Option<RepetitionNumber1>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<MessageStructure_EntryIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum NotEmptyRAListofIdleModeUEsIE_Extensions_EntryExtensionValue {
    #[asn(key = 180)]
    Id_LAofIdleModeUEs(LAListofIdleModeUEs),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NotEmptyRAListofIdleModeUEsIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: NotEmptyRAListofIdleModeUEsIE_Extensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NotEmptyRAListofIdleModeUEsIE_Extensions(
    pub Vec<NotEmptyRAListofIdleModeUEsIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Offload_RAB_ParametersIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Offload_RAB_ParametersIE_Extensions(pub Vec<Offload_RAB_ParametersIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum OutcomeValue {
    #[asn(key = 38)]
    Id_MBMSUELinking(MBMSUELinkingResponse),
    #[asn(key = 0)]
    Id_RAB_Assignment(RAB_AssignmentResponse),
    #[asn(key = 46)]
    Id_SRVCCPreparation(SRVCC_CSKeysResponse),
    #[asn(key = 47)]
    Id_UeRadioCapabilityMatch(UeRadioCapabilityMatchResponse),
    #[asn(key = 48)]
    Id_UeRegistrationQuery(UeRegistrationQueryResponse),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum OverloadProtocolIEs_EntryValue {
    #[asn(key = 86)]
    Id_GlobalRNC_ID(GlobalRNC_ID),
    #[asn(key = 18)]
    Id_NumberOfSteps(NumberOfSteps),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct OverloadProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: OverloadProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct OverloadProtocolIEs(pub Vec<OverloadProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum OverloadProtocolExtensions_EntryExtensionValue {
    #[asn(key = 3)]
    Id_CN_DomainIndicator(CN_DomainIndicator),
    #[asn(key = 171)]
    Id_ExtendedRNC_ID(ExtendedRNC_ID),
    #[asn(key = 96)]
    Id_GlobalCN_ID(GlobalCN_ID),
    #[asn(key = 245)]
    Id_Priority_Class_Indicator(Priority_Class_Indicator),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct OverloadProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: OverloadProtocolExtensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct OverloadProtocolExtensions(pub Vec<OverloadProtocolExtensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PLMNBasedIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PLMNBasedIE_Extensions(pub Vec<PLMNBasedIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PLMNs_in_shared_network_EntryIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PLMNs_in_shared_network_EntryIE_Extensions(
    pub Vec<PLMNs_in_shared_network_EntryIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PLMNs_in_shared_network_Entry {
    pub plm_nidentity: PLMNidentity,
    pub la_list: LA_LIST,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PLMNs_in_shared_network_EntryIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum PagingProtocolIEs_EntryValue {
    #[asn(key = 3)]
    Id_CN_DomainIndicator(CN_DomainIndicator),
    #[asn(key = 76)]
    Id_DRX_CycleLengthCoefficient(DRX_CycleLengthCoefficient),
    #[asn(key = 17)]
    Id_NonSearchingIndication(NonSearchingIndication),
    #[asn(key = 21)]
    Id_PagingAreaID(PagingAreaID),
    #[asn(key = 22)]
    Id_PagingCause(PagingCause),
    #[asn(key = 23)]
    Id_PermanentNAS_UE_ID(PermanentNAS_UE_ID),
    #[asn(key = 64)]
    Id_TemporaryUE_ID(TemporaryUE_ID),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PagingProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: PagingProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PagingProtocolIEs(pub Vec<PagingProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum PagingProtocolExtensions_EntryExtensionValue {
    #[asn(key = 229)]
    Id_CSG_Id_List(CSG_Id_List),
    #[asn(key = 96)]
    Id_GlobalCN_ID(GlobalCN_ID),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PagingProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: PagingProtocolExtensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PagingProtocolExtensions(pub Vec<PagingProtocolExtensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "8639999", extensible = true)]
pub struct INTEGER_46(pub u32);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "8639999", extensible = true)]
pub struct INTEGER_47(pub u32);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PeriodicLocationInfoIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PeriodicLocationInfoIE_Extensions(pub Vec<PeriodicLocationInfoIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum PositionDataIE_Extensions_EntryExtensionValue {
    #[asn(key = 284)]
    Id_Additional_PositioningDataSet(Additional_PositioningDataSet),
    #[asn(key = 184)]
    Id_GANSS_PositioningDataSet(GANSS_PositioningDataSet),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PositionDataIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: PositionDataIE_Extensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PositionDataIE_Extensions(pub Vec<PositionDataIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct INTEGER_48(pub u16);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OBJECT-IDENTIFIER")]
pub struct OBJECT_IDENTIFIER_49;

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PrivateMessagePrivateIEs_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PrivateMessagePrivateIEs(pub Vec<PrivateMessagePrivateIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RAB_AssignmentRequestProtocolIEs_EntryValue {
    #[asn(key = 41)]
    Id_RAB_ReleaseList(RAB_ReleaseList),
    #[asn(key = 54)]
    Id_RAB_SetupOrModifyList(RAB_SetupOrModifyList),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_AssignmentRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_AssignmentRequestProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_AssignmentRequestProtocolIEs(pub Vec<RAB_AssignmentRequestProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RAB_AssignmentRequestProtocolExtensions_EntryExtensionValue {
    #[asn(key = 239)]
    Id_MSISDN(MSISDN),
    #[asn(key = 233)]
    Id_UE_AggregateMaximumBitRate(UE_AggregateMaximumBitRate),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_AssignmentRequestProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RAB_AssignmentRequestProtocolExtensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_AssignmentRequestProtocolExtensions(
    pub Vec<RAB_AssignmentRequestProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RAB_AssignmentResponseProtocolIEs_EntryValue {
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 35)]
    Id_RAB_FailedList(RAB_FailedList),
    #[asn(key = 38)]
    Id_RAB_QueuedList(RAB_QueuedList),
    #[asn(key = 39)]
    Id_RAB_ReleaseFailedList(RAB_ReleaseFailedList),
    #[asn(key = 43)]
    Id_RAB_ReleasedList(RAB_ReleasedList),
    #[asn(key = 52)]
    Id_RAB_SetupOrModifiedList(RAB_SetupOrModifiedList),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_AssignmentResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_AssignmentResponseProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_AssignmentResponseProtocolIEs(pub Vec<RAB_AssignmentResponseProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RAB_AssignmentResponseProtocolExtensions_EntryExtensionValue {
    #[asn(key = 110)]
    Id_GERAN_Iumode_RAB_FailedList_RABAssgntResponse(GERAN_Iumode_RAB_FailedList_RABAssgntResponse),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_AssignmentResponseProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RAB_AssignmentResponseProtocolExtensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_AssignmentResponseProtocolExtensions(
    pub Vec<RAB_AssignmentResponseProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RAB_ContextFailedtoTransferList_Entry_EntryValue {
    #[asn(key = 84)]
    Id_RAB_ContextFailedtoTransferItem(RABs_ContextFailedtoTransferItem),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ContextFailedtoTransferList_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_ContextFailedtoTransferList_Entry_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_ContextFailedtoTransferList_Entry(
    pub Vec<RAB_ContextFailedtoTransferList_Entry_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ContextItemIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_ContextItemIE_Extensions(pub Vec<RAB_ContextItemIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ContextItem_RANAP_RelocInfIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_ContextItem_RANAP_RelocInfIE_Extensions(
    pub Vec<RAB_ContextItem_RANAP_RelocInfIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RAB_ContextList_Entry_EntryValue {
    #[asn(key = 24)]
    Id_RAB_ContextItem(RAB_ContextItem),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ContextList_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_ContextList_Entry_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_ContextList_Entry(pub Vec<RAB_ContextList_Entry_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RAB_ContextList_RANAP_RelocInf_Entry_EntryValue {
    #[asn(key = 82)]
    Id_RAB_ContextItem_RANAP_RelocInf(RAB_ContextItem_RANAP_RelocInf),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ContextList_RANAP_RelocInf_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_ContextList_RANAP_RelocInf_Entry_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_ContextList_RANAP_RelocInf_Entry(
    pub Vec<RAB_ContextList_RANAP_RelocInf_Entry_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RAB_DataForwardingItemIE_Extensions_EntryExtensionValue {
    #[asn(key = 13)]
    Id_IuTransportAssociation(IuTransportAssociation),
    #[asn(key = 67)]
    Id_TransportLayerAddress(TransportLayerAddress),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_DataForwardingItemIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RAB_DataForwardingItemIE_Extensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_DataForwardingItemIE_Extensions(pub Vec<RAB_DataForwardingItemIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_DataForwardingItem_SRNS_CtxReqIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_DataForwardingItem_SRNS_CtxReqIE_Extensions(
    pub Vec<RAB_DataForwardingItem_SRNS_CtxReqIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RAB_DataForwardingList_Entry_EntryValue {
    #[asn(key = 26)]
    Id_RAB_DataForwardingItem(RAB_DataForwardingItem),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_DataForwardingList_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_DataForwardingList_Entry_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_DataForwardingList_Entry(pub Vec<RAB_DataForwardingList_Entry_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RAB_DataForwardingList_SRNS_CtxReq_Entry_EntryValue {
    #[asn(key = 27)]
    Id_RAB_DataForwardingItem_SRNS_CtxReq(RAB_DataForwardingItem_SRNS_CtxReq),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_DataForwardingList_SRNS_CtxReq_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_DataForwardingList_SRNS_CtxReq_Entry_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_DataForwardingList_SRNS_CtxReq_Entry(
    pub Vec<RAB_DataForwardingList_SRNS_CtxReq_Entry_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_DataVolumeReportItemIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_DataVolumeReportItemIE_Extensions(
    pub Vec<RAB_DataVolumeReportItemIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RAB_DataVolumeReportList_Entry_EntryValue {
    #[asn(key = 30)]
    Id_RAB_DataVolumeReportItem(RAB_DataVolumeReportItem),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_DataVolumeReportList_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_DataVolumeReportList_Entry_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_DataVolumeReportList_Entry(pub Vec<RAB_DataVolumeReportList_Entry_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_DataVolumeReportRequestItemIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_DataVolumeReportRequestItemIE_Extensions(
    pub Vec<RAB_DataVolumeReportRequestItemIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RAB_DataVolumeReportRequestList_Entry_EntryValue {
    #[asn(key = 32)]
    Id_RAB_DataVolumeReportRequestItem(RAB_DataVolumeReportRequestItem),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_DataVolumeReportRequestList_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_DataVolumeReportRequestList_Entry_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_DataVolumeReportRequestList_Entry(
    pub Vec<RAB_DataVolumeReportRequestList_Entry_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_FailedItemIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_FailedItemIE_Extensions(pub Vec<RAB_FailedItemIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_FailedItem_EnhRelocInfoResIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_FailedItem_EnhRelocInfoResIE_Extensions(
    pub Vec<RAB_FailedItem_EnhRelocInfoResIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RAB_FailedList_Entry_EntryValue {
    #[asn(key = 34)]
    Id_RAB_FailedItem(RAB_FailedItem),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_FailedList_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_FailedList_Entry_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_FailedList_Entry(pub Vec<RAB_FailedList_Entry_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RAB_FailedList_EnhRelocInfoRes_Entry_EntryValue {
    #[asn(key = 198)]
    Id_RAB_FailedItem_EnhRelocInfoRes(RAB_FailedItem_EnhRelocInfoRes),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_FailedList_EnhRelocInfoRes_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_FailedList_EnhRelocInfoRes_Entry_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_FailedList_EnhRelocInfoRes_Entry(
    pub Vec<RAB_FailedList_EnhRelocInfoRes_Entry_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RAB_FailedtoReportList_Entry_EntryValue {
    #[asn(key = 71)]
    Id_RAB_FailedtoReportItem(RABs_failed_to_reportItem),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_FailedtoReportList_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_FailedtoReportList_Entry_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_FailedtoReportList_Entry(pub Vec<RAB_FailedtoReportList_Entry_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ModifyItemIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_ModifyItemIE_Extensions(pub Vec<RAB_ModifyItemIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RAB_ModifyList_Entry_EntryValue {
    #[asn(key = 92)]
    Id_RAB_ModifyItem(RAB_ModifyItem),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ModifyList_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_ModifyList_Entry_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_ModifyList_Entry(pub Vec<RAB_ModifyList_Entry_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RAB_ModifyRequestProtocolIEs_EntryValue {
    #[asn(key = 91)]
    Id_RAB_ModifyList(RAB_ModifyList),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ModifyRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_ModifyRequestProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_ModifyRequestProtocolIEs(pub Vec<RAB_ModifyRequestProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ModifyRequestProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_ModifyRequestProtocolExtensions(pub Vec<RAB_ModifyRequestProtocolExtensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RAB_ParametersIE_Extensions_EntryExtensionValue {
    #[asn(key = 176)]
    Id_RAB_Parameter_ExtendedGuaranteedBitrateList(RAB_Parameter_ExtendedGuaranteedBitrateList),
    #[asn(key = 177)]
    Id_RAB_Parameter_ExtendedMaxBitrateList(RAB_Parameter_ExtendedMaxBitrateList),
    #[asn(key = 218)]
    Id_RAB_Parameter_SupportedGuaranteedBitrateList(SupportedRAB_ParameterBitrateList),
    #[asn(key = 219)]
    Id_RAB_Parameter_SupportedMaxBitrateList(SupportedRAB_ParameterBitrateList),
    #[asn(key = 116)]
    Id_SignallingIndication(SignallingIndication),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ParametersIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RAB_ParametersIE_Extensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_ParametersIE_Extensions(pub Vec<RAB_ParametersIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_QueuedItemIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_QueuedItemIE_Extensions(pub Vec<RAB_QueuedItemIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RAB_QueuedList_Entry_EntryValue {
    #[asn(key = 37)]
    Id_RAB_QueuedItem(RAB_QueuedItem),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_QueuedList_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_QueuedList_Entry_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_QueuedList_Entry(pub Vec<RAB_QueuedList_Entry_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ReleaseItemIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_ReleaseItemIE_Extensions(pub Vec<RAB_ReleaseItemIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RAB_ReleaseList_Entry_EntryValue {
    #[asn(key = 40)]
    Id_RAB_ReleaseItem(RAB_ReleaseItem),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ReleaseList_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_ReleaseList_Entry_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_ReleaseList_Entry(pub Vec<RAB_ReleaseList_Entry_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RAB_ReleaseRequestProtocolIEs_EntryValue {
    #[asn(key = 41)]
    Id_RAB_ReleaseList(RAB_ReleaseList),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ReleaseRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_ReleaseRequestProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_ReleaseRequestProtocolIEs(pub Vec<RAB_ReleaseRequestProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ReleaseRequestProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_ReleaseRequestProtocolExtensions(
    pub Vec<RAB_ReleaseRequestProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ReleasedItemIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_ReleasedItemIE_Extensions(pub Vec<RAB_ReleasedItemIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ReleasedItem_IuRelCompIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_ReleasedItem_IuRelCompIE_Extensions(
    pub Vec<RAB_ReleasedItem_IuRelCompIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RAB_ReleasedList_Entry_EntryValue {
    #[asn(key = 42)]
    Id_RAB_ReleasedItem(RAB_ReleasedItem),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ReleasedList_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_ReleasedList_Entry_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_ReleasedList_Entry(pub Vec<RAB_ReleasedList_Entry_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RAB_ReleasedList_IuRelComp_Entry_EntryValue {
    #[asn(key = 87)]
    Id_RAB_ReleasedItem_IuRelComp(RAB_ReleasedItem_IuRelComp),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ReleasedList_IuRelComp_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_ReleasedList_IuRelComp_Entry_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_ReleasedList_IuRelComp_Entry(pub Vec<RAB_ReleasedList_IuRelComp_Entry_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_RelocationReleaseItemIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_RelocationReleaseItemIE_Extensions(
    pub Vec<RAB_RelocationReleaseItemIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RAB_RelocationReleaseList_Entry_EntryValue {
    #[asn(key = 45)]
    Id_RAB_RelocationReleaseItem(RAB_RelocationReleaseItem),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_RelocationReleaseList_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_RelocationReleaseList_Entry_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_RelocationReleaseList_Entry(pub Vec<RAB_RelocationReleaseList_Entry_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RAB_SetupItem_EnhRelocInfoReqIE_Extensions_EntryExtensionValue {
    #[asn(key = 231)]
    Id_E_UTRAN_Service_Handover(E_UTRAN_Service_Handover),
    #[asn(key = 238)]
    Id_PDP_TypeInformation_extension(PDP_TypeInformation_extension),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupItem_EnhRelocInfoReqIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RAB_SetupItem_EnhRelocInfoReqIE_Extensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_SetupItem_EnhRelocInfoReqIE_Extensions(
    pub Vec<RAB_SetupItem_EnhRelocInfoReqIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupItem_EnhRelocInfoResIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_SetupItem_EnhRelocInfoResIE_Extensions(
    pub Vec<RAB_SetupItem_EnhRelocInfoResIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupItem_EnhancedRelocCompleteReqIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_SetupItem_EnhancedRelocCompleteReqIE_Extensions(
    pub Vec<RAB_SetupItem_EnhancedRelocCompleteReqIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RAB_SetupItem_EnhancedRelocCompleteResIE_Extensions_EntryExtensionValue {
    #[asn(key = 240)]
    Id_Offload_RAB_Parameters(Offload_RAB_Parameters),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupItem_EnhancedRelocCompleteResIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RAB_SetupItem_EnhancedRelocCompleteResIE_Extensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_SetupItem_EnhancedRelocCompleteResIE_Extensions(
    pub Vec<RAB_SetupItem_EnhancedRelocCompleteResIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RAB_SetupItem_RelocReqIE_Extensions_EntryExtensionValue {
    #[asn(key = 89)]
    Id_Alt_RAB_Parameters(Alt_RAB_Parameters),
    #[asn(key = 231)]
    Id_E_UTRAN_Service_Handover(E_UTRAN_Service_Handover),
    #[asn(key = 107)]
    Id_GERAN_BSC_Container(GERAN_BSC_Container),
    #[asn(key = 240)]
    Id_Offload_RAB_Parameters(Offload_RAB_Parameters),
    #[asn(key = 238)]
    Id_PDP_TypeInformation_extension(PDP_TypeInformation_extension),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupItem_RelocReqIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RAB_SetupItem_RelocReqIE_Extensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_SetupItem_RelocReqIE_Extensions(pub Vec<RAB_SetupItem_RelocReqIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RAB_SetupItem_RelocReqAckIE_Extensions_EntryExtensionValue {
    #[asn(key = 90)]
    Id_Ass_RAB_Parameters(Ass_RAB_Parameters),
    #[asn(key = 13)]
    Id_IuTransportAssociation(IuTransportAssociation),
    #[asn(key = 67)]
    Id_TransportLayerAddress(TransportLayerAddress),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupItem_RelocReqAckIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RAB_SetupItem_RelocReqAckIE_Extensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_SetupItem_RelocReqAckIE_Extensions(
    pub Vec<RAB_SetupItem_RelocReqAckIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RAB_SetupList_EnhRelocInfoReq_Entry_EntryValue {
    #[asn(key = 193)]
    Id_RAB_SetupItem_EnhRelocInfoReq(RAB_SetupItem_EnhRelocInfoReq),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupList_EnhRelocInfoReq_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_SetupList_EnhRelocInfoReq_Entry_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_SetupList_EnhRelocInfoReq_Entry(pub Vec<RAB_SetupList_EnhRelocInfoReq_Entry_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RAB_SetupList_EnhRelocInfoRes_Entry_EntryValue {
    #[asn(key = 195)]
    Id_RAB_SetupItem_EnhRelocInfoRes(RAB_SetupItem_EnhRelocInfoRes),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupList_EnhRelocInfoRes_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_SetupList_EnhRelocInfoRes_Entry_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_SetupList_EnhRelocInfoRes_Entry(pub Vec<RAB_SetupList_EnhRelocInfoRes_Entry_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RAB_SetupList_EnhancedRelocCompleteReq_Entry_EntryValue {
    #[asn(key = 189)]
    Id_RAB_SetupItem_EnhancedRelocCompleteReq(RAB_SetupItem_EnhancedRelocCompleteReq),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupList_EnhancedRelocCompleteReq_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_SetupList_EnhancedRelocCompleteReq_Entry_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_SetupList_EnhancedRelocCompleteReq_Entry(
    pub Vec<RAB_SetupList_EnhancedRelocCompleteReq_Entry_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RAB_SetupList_EnhancedRelocCompleteRes_Entry_EntryValue {
    #[asn(key = 191)]
    Id_RAB_SetupItem_EnhancedRelocCompleteRes(RAB_SetupItem_EnhancedRelocCompleteRes),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupList_EnhancedRelocCompleteRes_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_SetupList_EnhancedRelocCompleteRes_Entry_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_SetupList_EnhancedRelocCompleteRes_Entry(
    pub Vec<RAB_SetupList_EnhancedRelocCompleteRes_Entry_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RAB_SetupList_RelocReq_Entry_EntryValue {
    #[asn(key = 47)]
    Id_RAB_SetupItem_RelocReq(RAB_SetupItem_RelocReq),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupList_RelocReq_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_SetupList_RelocReq_Entry_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_SetupList_RelocReq_Entry(pub Vec<RAB_SetupList_RelocReq_Entry_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RAB_SetupList_RelocReqAck_Entry_EntryValue {
    #[asn(key = 48)]
    Id_RAB_SetupItem_RelocReqAck(RAB_SetupItem_RelocReqAck),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupList_RelocReqAck_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_SetupList_RelocReqAck_Entry_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_SetupList_RelocReqAck_Entry(pub Vec<RAB_SetupList_RelocReqAck_Entry_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RAB_SetupOrModifiedItemIE_Extensions_EntryExtensionValue {
    #[asn(key = 90)]
    Id_Ass_RAB_Parameters(Ass_RAB_Parameters),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupOrModifiedItemIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RAB_SetupOrModifiedItemIE_Extensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_SetupOrModifiedItemIE_Extensions(
    pub Vec<RAB_SetupOrModifiedItemIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RAB_SetupOrModifiedList_Entry_EntryValue {
    #[asn(key = 51)]
    Id_RAB_SetupOrModifiedItem(RAB_SetupOrModifiedItem),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupOrModifiedList_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_SetupOrModifiedList_Entry_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_SetupOrModifiedList_Entry(pub Vec<RAB_SetupOrModifiedList_Entry_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RAB_SetupOrModifyItemFirstIE_Extensions_EntryExtensionValue {
    #[asn(key = 242)]
    Id_Correlation_ID(Correlation_ID),
    #[asn(key = 231)]
    Id_E_UTRAN_Service_Handover(E_UTRAN_Service_Handover),
    #[asn(key = 274)]
    Id_SIPTO_Correlation_ID(Correlation_ID),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupOrModifyItemFirstIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RAB_SetupOrModifyItemFirstIE_Extensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_SetupOrModifyItemFirstIE_Extensions(
    pub Vec<RAB_SetupOrModifyItemFirstIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RAB_SetupOrModifyItemSecondIE_Extensions_EntryExtensionValue {
    #[asn(key = 89)]
    Id_Alt_RAB_Parameters(Alt_RAB_Parameters),
    #[asn(key = 107)]
    Id_GERAN_BSC_Container(GERAN_BSC_Container),
    #[asn(key = 240)]
    Id_Offload_RAB_Parameters(Offload_RAB_Parameters),
    #[asn(key = 238)]
    Id_PDP_TypeInformation_extension(PDP_TypeInformation_extension),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupOrModifyItemSecondIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RAB_SetupOrModifyItemSecondIE_Extensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_SetupOrModifyItemSecondIE_Extensions(
    pub Vec<RAB_SetupOrModifyItemSecondIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RAB_SetupOrModifyList_Entry_EntryFirstValue {
    #[asn(key = 53)]
    Id_RAB_SetupOrModifyItem(RAB_SetupOrModifyItemFirst),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RAB_SetupOrModifyList_Entry_EntrySecondValue {
    #[asn(key = 53)]
    Id_RAB_SetupOrModifyItem(RAB_SetupOrModifyItemSecond),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupOrModifyList_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub first_criticality: Criticality,
    pub first_value: RAB_SetupOrModifyList_Entry_EntryFirstValue,
    pub second_criticality: Criticality,
    pub second_value: RAB_SetupOrModifyList_Entry_EntrySecondValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_SetupOrModifyList_Entry(pub Vec<RAB_SetupOrModifyList_Entry_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ToBeReleasedItem_EnhancedRelocCompleteResIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_ToBeReleasedItem_EnhancedRelocCompleteResIE_Extensions(
    pub Vec<RAB_ToBeReleasedItem_EnhancedRelocCompleteResIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RAB_ToBeReleasedList_EnhancedRelocCompleteRes_Entry_EntryValue {
    #[asn(key = 209)]
    Id_RAB_ToBeReleasedItem_EnhancedRelocCompleteRes(RAB_ToBeReleasedItem_EnhancedRelocCompleteRes),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ToBeReleasedList_EnhancedRelocCompleteRes_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_ToBeReleasedList_EnhancedRelocCompleteRes_Entry_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_ToBeReleasedList_EnhancedRelocCompleteRes_Entry(
    pub Vec<RAB_ToBeReleasedList_EnhancedRelocCompleteRes_Entry_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RAB_TrCH_MappingItemIE_Extensions_EntryExtensionValue {
    #[asn(key = 3)]
    Id_CN_DomainIndicator(CN_DomainIndicator),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_TrCH_MappingItemIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RAB_TrCH_MappingItemIE_Extensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_TrCH_MappingItemIE_Extensions(pub Vec<RAB_TrCH_MappingItemIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RABDataVolumeReport_EntryIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RABDataVolumeReport_EntryIE_Extensions(
    pub Vec<RABDataVolumeReport_EntryIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct RABDataVolumeReport_Entry {
    pub dl_unsuccessfully_transmitted_data_volume: UnsuccessfullyTransmittedDataVolume,
    #[asn(optional_idx = 0)]
    pub data_volume_reference: Option<DataVolumeReference>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<RABDataVolumeReport_EntryIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RABParametersList_EntryIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RABParametersList_EntryIE_Extensions(
    pub Vec<RABParametersList_EntryIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct RABParametersList_Entry {
    pub rab_id: RAB_ID,
    pub cn_domain: CN_DomainIndicator,
    #[asn(optional_idx = 0)]
    pub rab_data_volume_report: Option<RABDataVolumeReport>,
    #[asn(optional_idx = 1)]
    pub up_information: Option<UPInformation>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<RABParametersList_EntryIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RABasedIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RABasedIE_Extensions(pub Vec<RABasedIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RABs_ContextFailedtoTransferItemIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RABs_ContextFailedtoTransferItemIE_Extensions(
    pub Vec<RABs_ContextFailedtoTransferItemIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RABs_failed_to_reportItemIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RABs_failed_to_reportItemIE_Extensions(
    pub Vec<RABs_failed_to_reportItemIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAIIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAIIE_Extensions(pub Vec<RAIIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ENUMERATED_50(pub u8);
impl ENUMERATED_50 {
    pub const EMPTYLIST: u8 = 0u8;
    pub const FULLLIST: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RANAP_EnhancedRelocationInformationRequestProtocolIEs_EntryValue {
    #[asn(key = 133)]
    Id_CNMBMSLinkingInformation(CNMBMSLinkingInformation),
    #[asn(key = 206)]
    Id_GlobalCN_IDCS(GlobalCN_ID),
    #[asn(key = 207)]
    Id_GlobalCN_IDPS(GlobalCN_ID),
    #[asn(key = 204)]
    Id_OldIuSigConIdCS(IuSignallingConnectionIdentifier),
    #[asn(key = 205)]
    Id_OldIuSigConIdPS(IuSignallingConnectionIdentifier),
    #[asn(key = 192)]
    Id_RAB_SetupList_EnhRelocInfoReq(RAB_SetupList_EnhRelocInfoReq),
    #[asn(key = 105)]
    Id_SNA_Access_Information(SNA_Access_Information),
    #[asn(key = 127)]
    Id_SelectedPLMN_ID(PLMNidentity),
    #[asn(key = 61)]
    Id_Source_ToTarget_TransparentContainer(SourceRNC_ToTargetRNC_TransparentContainer),
    #[asn(key = 118)]
    Id_UESBI_Iu(UESBI_Iu),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RANAP_EnhancedRelocationInformationRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RANAP_EnhancedRelocationInformationRequestProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RANAP_EnhancedRelocationInformationRequestProtocolIEs(
    pub Vec<RANAP_EnhancedRelocationInformationRequestProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RANAP_EnhancedRelocationInformationRequestProtocolExtensions_EntryExtensionValue {
    #[asn(key = 261)]
    Id_AnchorPLMN_ID(PLMNidentity),
    #[asn(key = 203)]
    Id_CSG_Id(CSG_Id),
    #[asn(key = 234)]
    Id_CSG_Membership_Status(CSG_Membership_Status),
    #[asn(key = 11)]
    Id_EncryptionInformation(EncryptionInformation),
    #[asn(key = 12)]
    Id_IntegrityProtectionInformation(IntegrityProtectionInformation),
    #[asn(key = 248)]
    Id_RABParametersList(RABParametersList),
    #[asn(key = 233)]
    Id_UE_AggregateMaximumBitRate(UE_AggregateMaximumBitRate),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RANAP_EnhancedRelocationInformationRequestProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value:
        RANAP_EnhancedRelocationInformationRequestProtocolExtensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RANAP_EnhancedRelocationInformationRequestProtocolExtensions(
    pub Vec<RANAP_EnhancedRelocationInformationRequestProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RANAP_EnhancedRelocationInformationResponseProtocolIEs_EntryValue {
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 197)]
    Id_RAB_FailedList_EnhRelocInfoRes(RAB_FailedList_EnhRelocInfoRes),
    #[asn(key = 194)]
    Id_RAB_SetupList_EnhRelocInfoRes(RAB_SetupList_EnhRelocInfoRes),
    #[asn(key = 63)]
    Id_Target_ToSource_TransparentContainer(TargetRNC_ToSourceRNC_TransparentContainer),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RANAP_EnhancedRelocationInformationResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RANAP_EnhancedRelocationInformationResponseProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RANAP_EnhancedRelocationInformationResponseProtocolIEs(
    pub Vec<RANAP_EnhancedRelocationInformationResponseProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RANAP_EnhancedRelocationInformationResponseProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RANAP_EnhancedRelocationInformationResponseProtocolExtensions(
    pub Vec<RANAP_EnhancedRelocationInformationResponseProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RANAP_RelocationInformationProtocolIEs_EntryValue {
    #[asn(key = 81)]
    Id_DirectTransferInformationList_RANAP_RelocInf(DirectTransferInformationList_RANAP_RelocInf),
    #[asn(key = 83)]
    Id_RAB_ContextList_RANAP_RelocInf(RAB_ContextList_RANAP_RelocInf),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RANAP_RelocationInformationProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RANAP_RelocationInformationProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RANAP_RelocationInformationProtocolIEs(
    pub Vec<RANAP_RelocationInformationProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RANAP_RelocationInformationProtocolExtensions_EntryExtensionValue {
    #[asn(key = 247)]
    Id_RNSAPRelocationParameters(RNSAPRelocationParameters),
    #[asn(key = 103)]
    Id_SourceRNC_PDCP_context_info(RRC_Container),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RANAP_RelocationInformationProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RANAP_RelocationInformationProtocolExtensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RANAP_RelocationInformationProtocolExtensions(
    pub Vec<RANAP_RelocationInformationProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RIM_TransferIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RIM_TransferIE_Extensions(pub Vec<RIM_TransferIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct ENUMERATED_51(pub u8);
impl ENUMERATED_51 {
    pub const ACTIVATED: u8 = 0u8;
    pub const DEACTIVATED: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RNCTraceInformationIE_Extensions_EntryExtensionValue {
    #[asn(key = 256)]
    Id_IMSI(IMSI),
    #[asn(key = 270)]
    Id_Serving_Cell_Identifier(UTRAN_CellID),
    #[asn(key = 251)]
    Id_Trace_Collection_Entity_IP_Addess(TransportLayerAddress),
    #[asn(key = 255)]
    Id_TraceRecordingSessionReference(TraceRecordingSessionReference),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RNCTraceInformationIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RNCTraceInformationIE_Extensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RNCTraceInformationIE_Extensions(pub Vec<RNCTraceInformationIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RNSAPRelocationParametersIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RNSAPRelocationParametersIE_Extensions(
    pub Vec<RNSAPRelocationParametersIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BOOLEAN")]
pub struct BOOLEAN_52(pub bool);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BOOLEAN")]
pub struct BOOLEAN_53(pub bool);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "BITSTRING",
    sz_extensible = false,
    sz_lb = "128",
    sz_ub = "128"
)]
pub struct BIT_STRING_54(pub BitVec<u8, Msb0>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "OCTET-STRING",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "32"
)]
pub struct OCTET_STRING_55(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RSRVCC_InformationIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RSRVCC_InformationIE_Extensions(pub Vec<RSRVCC_InformationIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RedirectionIndication_EntryValue {
    #[asn(key = 280)]
    Id_Additional_CSPS_coordination_information(Additional_CSPS_coordination_information),
    #[asn(key = 16)]
    Id_NAS_PDU(NAS_PDU),
    #[asn(key = 130)]
    Id_NAS_SequenceNumber(NAS_SequenceNumber),
    #[asn(key = 23)]
    Id_PermanentNAS_UE_ID(PermanentNAS_UE_ID),
    #[asn(key = 131)]
    Id_RejectCauseValue(RejectCauseValue),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RedirectionIndication_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RedirectionIndication_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RelocationCancelProtocolIEs_EntryValue {
    #[asn(key = 4)]
    Id_Cause(Cause),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationCancelProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RelocationCancelProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RelocationCancelProtocolIEs(pub Vec<RelocationCancelProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationCancelProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RelocationCancelProtocolExtensions(pub Vec<RelocationCancelProtocolExtensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RelocationCancelAcknowledgeProtocolIEs_EntryValue {
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationCancelAcknowledgeProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RelocationCancelAcknowledgeProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RelocationCancelAcknowledgeProtocolIEs(
    pub Vec<RelocationCancelAcknowledgeProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationCancelAcknowledgeProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RelocationCancelAcknowledgeProtocolExtensions(
    pub Vec<RelocationCancelAcknowledgeProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RelocationCommandProtocolIEs_EntryValue {
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 14)]
    Id_L3_Information(L3_Information),
    #[asn(key = 28)]
    Id_RAB_DataForwardingList(RAB_DataForwardingList),
    #[asn(key = 46)]
    Id_RAB_RelocationReleaseList(RAB_RelocationReleaseList),
    #[asn(key = 63)]
    Id_Target_ToSource_TransparentContainer(Target_ToSource_TransparentContainer),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationCommandProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RelocationCommandProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RelocationCommandProtocolIEs(pub Vec<RelocationCommandProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RelocationCommandProtocolExtensions_EntryExtensionValue {
    #[asn(key = 99)]
    Id_InterSystemInformation_TransparentContainer(InterSystemInformation_TransparentContainer),
    #[asn(key = 260)]
    Id_RSRVCC_Information(RSRVCC_Information),
    #[asn(key = 227)]
    Id_SRVCC_Information(SRVCC_Information),
    #[asn(key = 162)]
    Id_TargetBSS_ToSourceBSS_TransparentContainer(TargetBSS_ToSourceBSS_TransparentContainer),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationCommandProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RelocationCommandProtocolExtensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RelocationCommandProtocolExtensions(pub Vec<RelocationCommandProtocolExtensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationCompleteProtocolIEs_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RelocationCompleteProtocolIEs(pub Vec<RelocationCompleteProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RelocationCompleteProtocolExtensions_EntryExtensionValue {
    #[asn(key = 250)]
    Id_HigherBitratesThan16MbpsFlag(HigherBitratesThan16MbpsFlag),
    #[asn(key = 275)]
    Id_LHN_ID(LHN_ID),
    #[asn(key = 262)]
    Id_Tunnel_Information_for_BBF(TunnelInformation),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationCompleteProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RelocationCompleteProtocolExtensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RelocationCompleteProtocolExtensions(
    pub Vec<RelocationCompleteProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationDetectProtocolIEs_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RelocationDetectProtocolIEs(pub Vec<RelocationDetectProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationDetectProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RelocationDetectProtocolExtensions(pub Vec<RelocationDetectProtocolExtensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RelocationFailureProtocolIEs_EntryValue {
    #[asn(key = 4)]
    Id_Cause(Cause),
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationFailureProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RelocationFailureProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RelocationFailureProtocolIEs(pub Vec<RelocationFailureProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RelocationFailureProtocolExtensions_EntryExtensionValue {
    #[asn(key = 108)]
    Id_GERAN_Classmark(GERAN_Classmark),
    #[asn(key = 100)]
    Id_NewBSS_To_OldBSS_Information(NewBSS_To_OldBSS_Information),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationFailureProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RelocationFailureProtocolExtensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RelocationFailureProtocolExtensions(pub Vec<RelocationFailureProtocolExtensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RelocationPreparationFailureProtocolIEs_EntryValue {
    #[asn(key = 4)]
    Id_Cause(Cause),
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationPreparationFailureProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RelocationPreparationFailureProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RelocationPreparationFailureProtocolIEs(
    pub Vec<RelocationPreparationFailureProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RelocationPreparationFailureProtocolExtensions_EntryExtensionValue {
    #[asn(key = 99)]
    Id_InterSystemInformation_TransparentContainer(InterSystemInformation_TransparentContainer),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationPreparationFailureProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RelocationPreparationFailureProtocolExtensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RelocationPreparationFailureProtocolExtensions(
    pub Vec<RelocationPreparationFailureProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RelocationRequestProtocolIEs_EntryValue {
    #[asn(key = 3)]
    Id_CN_DomainIndicator(CN_DomainIndicator),
    #[asn(key = 4)]
    Id_Cause(Cause),
    #[asn(key = 11)]
    Id_EncryptionInformation(EncryptionInformation),
    #[asn(key = 12)]
    Id_IntegrityProtectionInformation(IntegrityProtectionInformation),
    #[asn(key = 79)]
    Id_IuSigConId(IuSignallingConnectionIdentifier),
    #[asn(key = 23)]
    Id_PermanentNAS_UE_ID(PermanentNAS_UE_ID),
    #[asn(key = 49)]
    Id_RAB_SetupList_RelocReq(RAB_SetupList_RelocReq),
    #[asn(key = 61)]
    Id_Source_ToTarget_TransparentContainer(SourceRNC_ToTargetRNC_TransparentContainer),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RelocationRequestProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RelocationRequestProtocolIEs(pub Vec<RelocationRequestProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RelocationRequestProtocolExtensions_EntryExtensionValue {
    #[asn(key = 261)]
    Id_AnchorPLMN_ID(PLMNidentity),
    #[asn(key = 133)]
    Id_CNMBMSLinkingInformation(CNMBMSLinkingInformation),
    #[asn(key = 203)]
    Id_CSG_Id(CSG_Id),
    #[asn(key = 234)]
    Id_CSG_Membership_Status(CSG_Membership_Status),
    #[asn(key = 96)]
    Id_GlobalCN_ID(GlobalCN_ID),
    #[asn(key = 239)]
    Id_MSISDN(MSISDN),
    #[asn(key = 289)]
    Id_PowerSavingIndicator(PowerSavingIndicator),
    #[asn(key = 105)]
    Id_SNA_Access_Information(SNA_Access_Information),
    #[asn(key = 127)]
    Id_SelectedPLMN_ID(PLMNidentity),
    #[asn(key = 233)]
    Id_UE_AggregateMaximumBitRate(UE_AggregateMaximumBitRate),
    #[asn(key = 293)]
    Id_UE_Application_Layer_Measurement_Configuration_For_Relocation(
        UE_Application_Layer_Measurement_Configuration_For_Relocation,
    ),
    #[asn(key = 118)]
    Id_UESBI_Iu(UESBI_Iu),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationRequestProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RelocationRequestProtocolExtensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RelocationRequestProtocolExtensions(pub Vec<RelocationRequestProtocolExtensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RelocationRequestAcknowledgeProtocolIEs_EntryValue {
    #[asn(key = 5)]
    Id_ChosenEncryptionAlgorithm(ChosenEncryptionAlgorithm),
    #[asn(key = 6)]
    Id_ChosenIntegrityProtectionAlgorithm(ChosenIntegrityProtectionAlgorithm),
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 35)]
    Id_RAB_FailedList(RAB_FailedList),
    #[asn(key = 50)]
    Id_RAB_SetupList_RelocReqAck(RAB_SetupList_RelocReqAck),
    #[asn(key = 63)]
    Id_Target_ToSource_TransparentContainer(TargetRNC_ToSourceRNC_TransparentContainer),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationRequestAcknowledgeProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RelocationRequestAcknowledgeProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RelocationRequestAcknowledgeProtocolIEs(
    pub Vec<RelocationRequestAcknowledgeProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RelocationRequestAcknowledgeProtocolExtensions_EntryExtensionValue {
    #[asn(key = 203)]
    Id_CSG_Id(CSG_Id),
    #[asn(key = 100)]
    Id_NewBSS_To_OldBSS_Information(NewBSS_To_OldBSS_Information),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationRequestAcknowledgeProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RelocationRequestAcknowledgeProtocolExtensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RelocationRequestAcknowledgeProtocolExtensions(
    pub Vec<RelocationRequestAcknowledgeProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RelocationRequiredProtocolIEs_EntryValue {
    #[asn(key = 4)]
    Id_Cause(Cause),
    #[asn(key = 7)]
    Id_ClassmarkInformation2(ClassmarkInformation2),
    #[asn(key = 8)]
    Id_ClassmarkInformation3(ClassmarkInformation3),
    #[asn(key = 20)]
    Id_OldBSS_ToNewBSS_Information(OldBSS_ToNewBSS_Information),
    #[asn(key = 56)]
    Id_RelocationType(RelocationType),
    #[asn(key = 61)]
    Id_Source_ToTarget_TransparentContainer(Source_ToTarget_TransparentContainer),
    #[asn(key = 60)]
    Id_SourceID(SourceID),
    #[asn(key = 62)]
    Id_TargetID(TargetID),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationRequiredProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RelocationRequiredProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RelocationRequiredProtocolIEs(pub Vec<RelocationRequiredProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RelocationRequiredProtocolExtensions_EntryExtensionValue {
    #[asn(key = 203)]
    Id_CSG_Id(CSG_Id),
    #[asn(key = 235)]
    Id_Cell_Access_Mode(Cell_Access_Mode),
    #[asn(key = 108)]
    Id_GERAN_Classmark(GERAN_Classmark),
    #[asn(key = 259)]
    Id_RSRVCC_HO_Indication(RSRVCC_HO_Indication),
    #[asn(key = 226)]
    Id_SRVCC_HO_Indication(SRVCC_HO_Indication),
    #[asn(key = 161)]
    Id_SourceBSS_ToTargetBSS_TransparentContainer(SourceBSS_ToTargetBSS_TransparentContainer),
    #[asn(key = 293)]
    Id_UE_Application_Layer_Measurement_Configuration_For_Relocation(
        UE_Application_Layer_Measurement_Configuration_For_Relocation,
    ),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationRequiredProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RelocationRequiredProtocolExtensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RelocationRequiredProtocolExtensions(
    pub Vec<RelocationRequiredProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct INTEGER_56(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum Requested_RAB_Parameter_ValuesIE_Extensions_EntryExtensionValue {
    #[asn(key = 159)]
    Id_AlternativeRABConfigurationRequest(AlternativeRABConfigurationRequest),
    #[asn(key = 179)]
    Id_Requested_RAB_Parameter_ExtendedGuaranteedBitrateList(
        Requested_RAB_Parameter_ExtendedGuaranteedBitrateList,
    ),
    #[asn(key = 178)]
    Id_Requested_RAB_Parameter_ExtendedMaxBitrateList(
        Requested_RAB_Parameter_ExtendedMaxBitrateList,
    ),
    #[asn(key = 221)]
    Id_Requested_RAB_Parameter_SupportedGuaranteedBitrateList(SupportedRAB_ParameterBitrateList),
    #[asn(key = 220)]
    Id_Requested_RAB_Parameter_SupportedMaxBitrateList(SupportedRAB_ParameterBitrateList),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Requested_RAB_Parameter_ValuesIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: Requested_RAB_Parameter_ValuesIE_Extensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Requested_RAB_Parameter_ValuesIE_Extensions(
    pub Vec<Requested_RAB_Parameter_ValuesIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum RerouteNASRequestProtocolIEs_EntryValue {
    #[asn(key = 287)]
    Id_P_TMSI(P_TMSI),
    #[asn(key = 286)]
    Id_SGSN_Group_Identity(SGSN_Group_Identity),
    #[asn(key = 290)]
    Id_UE_Usage_Type(UE_Usage_Type),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RerouteNASRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RerouteNASRequestProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RerouteNASRequestProtocolIEs(pub Vec<RerouteNASRequestProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RerouteNASRequestProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RerouteNASRequestProtocolExtensions(pub Vec<RerouteNASRequestProtocolExtensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum ResetProtocolIEs_EntryValue {
    #[asn(key = 3)]
    Id_CN_DomainIndicator(CN_DomainIndicator),
    #[asn(key = 4)]
    Id_Cause(Cause),
    #[asn(key = 86)]
    Id_GlobalRNC_ID(GlobalRNC_ID),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ResetProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ResetProtocolIEs(pub Vec<ResetProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum ResetProtocolExtensions_EntryExtensionValue {
    #[asn(key = 171)]
    Id_ExtendedRNC_ID(ExtendedRNC_ID),
    #[asn(key = 96)]
    Id_GlobalCN_ID(GlobalCN_ID),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: ResetProtocolExtensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ResetProtocolExtensions(pub Vec<ResetProtocolExtensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum ResetAcknowledgeProtocolIEs_EntryValue {
    #[asn(key = 3)]
    Id_CN_DomainIndicator(CN_DomainIndicator),
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 86)]
    Id_GlobalRNC_ID(GlobalRNC_ID),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetAcknowledgeProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ResetAcknowledgeProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ResetAcknowledgeProtocolIEs(pub Vec<ResetAcknowledgeProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum ResetAcknowledgeProtocolExtensions_EntryExtensionValue {
    #[asn(key = 171)]
    Id_ExtendedRNC_ID(ExtendedRNC_ID),
    #[asn(key = 96)]
    Id_GlobalCN_ID(GlobalCN_ID),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetAcknowledgeProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: ResetAcknowledgeProtocolExtensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ResetAcknowledgeProtocolExtensions(pub Vec<ResetAcknowledgeProtocolExtensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum ResetResourceProtocolIEs_EntryValue {
    #[asn(key = 3)]
    Id_CN_DomainIndicator(CN_DomainIndicator),
    #[asn(key = 4)]
    Id_Cause(Cause),
    #[asn(key = 86)]
    Id_GlobalRNC_ID(GlobalRNC_ID),
    #[asn(key = 77)]
    Id_IuSigConIdList(ResetResourceList),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetResourceProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ResetResourceProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ResetResourceProtocolIEs(pub Vec<ResetResourceProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum ResetResourceProtocolExtensions_EntryExtensionValue {
    #[asn(key = 171)]
    Id_ExtendedRNC_ID(ExtendedRNC_ID),
    #[asn(key = 96)]
    Id_GlobalCN_ID(GlobalCN_ID),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetResourceProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: ResetResourceProtocolExtensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ResetResourceProtocolExtensions(pub Vec<ResetResourceProtocolExtensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum ResetResourceAckItemIE_Extensions_EntryExtensionValue {
    #[asn(key = 282)]
    Id_IuSigConIdRangeEnd(IuSignallingConnectionIdentifier),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetResourceAckItemIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: ResetResourceAckItemIE_Extensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ResetResourceAckItemIE_Extensions(pub Vec<ResetResourceAckItemIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum ResetResourceAckList_Entry_EntryValue {
    #[asn(key = 78)]
    Id_IuSigConIdItem(ResetResourceAckItem),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetResourceAckList_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ResetResourceAckList_Entry_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ResetResourceAckList_Entry(pub Vec<ResetResourceAckList_Entry_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum ResetResourceAcknowledgeProtocolIEs_EntryValue {
    #[asn(key = 3)]
    Id_CN_DomainIndicator(CN_DomainIndicator),
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 86)]
    Id_GlobalRNC_ID(GlobalRNC_ID),
    #[asn(key = 77)]
    Id_IuSigConIdList(ResetResourceAckList),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetResourceAcknowledgeProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ResetResourceAcknowledgeProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ResetResourceAcknowledgeProtocolIEs(pub Vec<ResetResourceAcknowledgeProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum ResetResourceAcknowledgeProtocolExtensions_EntryExtensionValue {
    #[asn(key = 171)]
    Id_ExtendedRNC_ID(ExtendedRNC_ID),
    #[asn(key = 96)]
    Id_GlobalCN_ID(GlobalCN_ID),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetResourceAcknowledgeProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: ResetResourceAcknowledgeProtocolExtensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ResetResourceAcknowledgeProtocolExtensions(
    pub Vec<ResetResourceAcknowledgeProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum ResetResourceItemIE_Extensions_EntryExtensionValue {
    #[asn(key = 282)]
    Id_IuSigConIdRangeEnd(IuSignallingConnectionIdentifier),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetResourceItemIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: ResetResourceItemIE_Extensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ResetResourceItemIE_Extensions(pub Vec<ResetResourceItemIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum ResetResourceList_Entry_EntryValue {
    #[asn(key = 78)]
    Id_IuSigConIdItem(ResetResourceItem),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetResourceList_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ResetResourceList_Entry_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ResetResourceList_Entry(pub Vec<ResetResourceList_Entry_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "9")]
pub struct INTEGER_57(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "8")]
pub struct INTEGER_58(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResidualBitErrorRatioIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ResidualBitErrorRatioIE_Extensions(pub Vec<ResidualBitErrorRatioIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SAIIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SAIIE_Extensions(pub Vec<SAIIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "9")]
pub struct INTEGER_59(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "6")]
pub struct INTEGER_60(pub u8);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SDU_ErrorRatioIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SDU_ErrorRatioIE_Extensions(pub Vec<SDU_ErrorRatioIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SDU_FormatInformationParameters_EntryIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SDU_FormatInformationParameters_EntryIE_Extensions(
    pub Vec<SDU_FormatInformationParameters_EntryIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct SDU_FormatInformationParameters_Entry {
    #[asn(optional_idx = 0)]
    pub subflow_sdu_size: Option<SubflowSDU_Size>,
    #[asn(optional_idx = 1)]
    pub rab_subflow_combination_bit_rate: Option<RAB_SubflowCombinationBitRate>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<SDU_FormatInformationParameters_EntryIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SDU_Parameters_EntryIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SDU_Parameters_EntryIE_Extensions(pub Vec<SDU_Parameters_EntryIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct SDU_Parameters_Entry {
    #[asn(optional_idx = 0)]
    pub sdu_error_ratio: Option<SDU_ErrorRatio>,
    pub residual_bit_error_ratio: ResidualBitErrorRatio,
    pub delivery_of_erroneous_sdu: DeliveryOfErroneousSDU,
    #[asn(optional_idx = 1)]
    pub sdu_format_information_parameters: Option<SDU_FormatInformationParameters>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<SDU_Parameters_EntryIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SNA_Access_InformationIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SNA_Access_InformationIE_Extensions(pub Vec<SNA_Access_InformationIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SRB_TrCH_MappingItemIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SRB_TrCH_MappingItemIE_Extensions(pub Vec<SRB_TrCH_MappingItemIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum SRNS_ContextRequestProtocolIEs_EntryValue {
    #[asn(key = 29)]
    Id_RAB_DataForwardingList_SRNS_CtxReq(RAB_DataForwardingList_SRNS_CtxReq),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SRNS_ContextRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: SRNS_ContextRequestProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct SRNS_ContextRequestProtocolIEs(pub Vec<SRNS_ContextRequestProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum SRNS_ContextRequestProtocolExtensions_EntryExtensionValue {
    #[asn(key = 167)]
    Id_RAT_Type(RAT_Type),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SRNS_ContextRequestProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: SRNS_ContextRequestProtocolExtensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SRNS_ContextRequestProtocolExtensions(
    pub Vec<SRNS_ContextRequestProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum SRNS_ContextResponseProtocolIEs_EntryValue {
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 85)]
    Id_RAB_ContextFailedtoTransferList(RAB_ContextFailedtoTransferList),
    #[asn(key = 25)]
    Id_RAB_ContextList(RAB_ContextList),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SRNS_ContextResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: SRNS_ContextResponseProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct SRNS_ContextResponseProtocolIEs(pub Vec<SRNS_ContextResponseProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SRNS_ContextResponseProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SRNS_ContextResponseProtocolExtensions(
    pub Vec<SRNS_ContextResponseProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum SRNS_DataForwardCommandProtocolIEs_EntryValue {
    #[asn(key = 28)]
    Id_RAB_DataForwardingList(RAB_DataForwardingList),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SRNS_DataForwardCommandProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: SRNS_DataForwardCommandProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct SRNS_DataForwardCommandProtocolIEs(pub Vec<SRNS_DataForwardCommandProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SRNS_DataForwardCommandProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SRNS_DataForwardCommandProtocolExtensions(
    pub Vec<SRNS_DataForwardCommandProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SRVCC_CSKeysRequestProtocolIEs_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct SRVCC_CSKeysRequestProtocolIEs(pub Vec<SRVCC_CSKeysRequestProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SRVCC_CSKeysRequestProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SRVCC_CSKeysRequestProtocolExtensions(
    pub Vec<SRVCC_CSKeysRequestProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum SRVCC_CSKeysResponseProtocolIEs_EntryValue {
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 224)]
    Id_EncryptionKey(EncryptionKey),
    #[asn(key = 225)]
    Id_IntegrityProtectionKey(IntegrityProtectionKey),
    #[asn(key = 227)]
    Id_SRVCC_Information(SRVCC_Information),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SRVCC_CSKeysResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: SRVCC_CSKeysResponseProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct SRVCC_CSKeysResponseProtocolIEs(pub Vec<SRVCC_CSKeysResponseProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SRVCC_CSKeysResponseProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SRVCC_CSKeysResponseProtocolExtensions(
    pub Vec<SRVCC_CSKeysResponseProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "BITSTRING",
    sz_extensible = false,
    sz_lb = "128",
    sz_ub = "128"
)]
pub struct BIT_STRING_61(pub BitVec<u8, Msb0>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SRVCC_InformationIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SRVCC_InformationIE_Extensions(pub Vec<SRVCC_InformationIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum SecurityModeCommandProtocolIEs_EntryValue {
    #[asn(key = 11)]
    Id_EncryptionInformation(EncryptionInformation),
    #[asn(key = 12)]
    Id_IntegrityProtectionInformation(IntegrityProtectionInformation),
    #[asn(key = 75)]
    Id_KeyStatus(KeyStatus),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecurityModeCommandProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: SecurityModeCommandProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct SecurityModeCommandProtocolIEs(pub Vec<SecurityModeCommandProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecurityModeCommandProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SecurityModeCommandProtocolExtensions(
    pub Vec<SecurityModeCommandProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum SecurityModeCompleteProtocolIEs_EntryValue {
    #[asn(key = 5)]
    Id_ChosenEncryptionAlgorithm(ChosenEncryptionAlgorithm),
    #[asn(key = 6)]
    Id_ChosenIntegrityProtectionAlgorithm(ChosenIntegrityProtectionAlgorithm),
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecurityModeCompleteProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: SecurityModeCompleteProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct SecurityModeCompleteProtocolIEs(pub Vec<SecurityModeCompleteProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecurityModeCompleteProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SecurityModeCompleteProtocolExtensions(
    pub Vec<SecurityModeCompleteProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum SecurityModeRejectProtocolIEs_EntryValue {
    #[asn(key = 4)]
    Id_Cause(Cause),
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecurityModeRejectProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: SecurityModeRejectProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct SecurityModeRejectProtocolIEs(pub Vec<SecurityModeRejectProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecurityModeRejectProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SecurityModeRejectProtocolExtensions(
    pub Vec<SecurityModeRejectProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Shared_Network_InformationIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Shared_Network_InformationIE_Extensions(
    pub Vec<Shared_Network_InformationIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum SourceRNC_IDIE_Extensions_EntryExtensionValue {
    #[asn(key = 171)]
    Id_ExtendedRNC_ID(ExtendedRNC_ID),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SourceRNC_IDIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: SourceRNC_IDIE_Extensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SourceRNC_IDIE_Extensions(pub Vec<SourceRNC_IDIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum SourceRNC_ToTargetRNC_TransparentContainerIE_Extensions_EntryExtensionValue {
    #[asn(key = 237)]
    Id_CSFB_Information(CSFB_Information),
    #[asn(key = 121)]
    Id_CellLoadInformationGroup(CellLoadInformationGroup),
    #[asn(key = 243)]
    Id_IRAT_Measurement_Configuration(IRAT_Measurement_Configuration),
    #[asn(key = 277)]
    Id_LastE_UTRANPLMNIdentity(PLMNidentity),
    #[asn(key = 156)]
    Id_MBMSLinkingInformation(MBMSLinkingInformation),
    #[asn(key = 249)]
    Id_Management_Based_MDT_Allowed(Management_Based_MDT_Allowed),
    #[asn(key = 263)]
    Id_Management_Based_MDT_PLMN_List(MDT_PLMN_List),
    #[asn(key = 230)]
    Id_PSRABtobeReplaced(RAB_ID),
    #[asn(key = 98)]
    Id_SRB_TrCH_Mapping(SRB_TrCH_Mapping),
    #[asn(key = 227)]
    Id_SRVCC_Information(SRVCC_Information),
    #[asn(key = 296)]
    Id_SRVCCSource(SRVCCSource),
    #[asn(key = 202)]
    Id_SubscriberProfileIDforRFP(SubscriberProfileIDforRFP),
    #[asn(key = 124)]
    Id_TraceRecordingSessionInformation(TraceRecordingSessionInformation),
    #[asn(key = 200)]
    Id_UE_History_Information(UE_History_Information),
    #[asn(key = 187)]
    Id_d_RNTI_for_NoIuCSUP(D_RNTI),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SourceRNC_ToTargetRNC_TransparentContainerIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value:
        SourceRNC_ToTargetRNC_TransparentContainerIE_Extensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SourceRNC_ToTargetRNC_TransparentContainerIE_Extensions(
    pub Vec<SourceRNC_ToTargetRNC_TransparentContainerIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SourceUTRANCellIDIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SourceUTRANCellIDIE_Extensions(pub Vec<SourceUTRANCellIDIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum SuccessfulOutcomeValue {
    #[asn(key = 7)]
    Id_DataVolumeReport(DataVolumeReport),
    #[asn(key = 31)]
    Id_InformationTransfer(InformationTransferConfirmation),
    #[asn(key = 1)]
    Id_Iu_Release(Iu_ReleaseComplete),
    #[asn(key = 30)]
    Id_LocationRelatedData(LocationRelatedDataResponse),
    #[asn(key = 40)]
    Id_MBMSCNDe_Registration_Procedure(MBMSCNDe_RegistrationResponse),
    #[asn(key = 42)]
    Id_MBMSRABRelease(MBMSRABRelease),
    #[asn(key = 39)]
    Id_MBMSRegistration(MBMSRegistrationResponse),
    #[asn(key = 35)]
    Id_MBMSSessionStart(MBMSSessionStartResponse),
    #[asn(key = 37)]
    Id_MBMSSessionStop(MBMSSessionStopResponse),
    #[asn(key = 36)]
    Id_MBMSSessionUpdate(MBMSSessionUpdateResponse),
    #[asn(key = 45)]
    Id_RANAPenhancedRelocation(RANAP_EnhancedRelocationInformationResponse),
    #[asn(key = 4)]
    Id_RelocationCancel(RelocationCancelAcknowledge),
    #[asn(key = 2)]
    Id_RelocationPreparation(RelocationCommand),
    #[asn(key = 3)]
    Id_RelocationResourceAllocation(RelocationRequestAcknowledge),
    #[asn(key = 9)]
    Id_Reset(ResetAcknowledge),
    #[asn(key = 27)]
    Id_ResetResource(ResetResourceAcknowledge),
    #[asn(key = 5)]
    Id_SRNS_ContextTransfer(SRNS_ContextResponse),
    #[asn(key = 6)]
    Id_SecurityModeControl(SecurityModeComplete),
    #[asn(key = 33)]
    Id_UplinkInformationExchange(UplinkInformationExchangeResponse),
    #[asn(key = 43)]
    Id_enhancedRelocationComplete(EnhancedRelocationCompleteResponse),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TAIIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TAIIE_Extensions(pub Vec<TAIIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "3", sz_ub = "3")]
pub struct OCTET_STRING_62(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TMGIIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TMGIIE_Extensions(pub Vec<TMGIIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TNLInformationEnhRelInfoReqIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TNLInformationEnhRelInfoReqIE_Extensions(
    pub Vec<TNLInformationEnhRelInfoReqIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TNLInformationEnhRelInfoResIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TNLInformationEnhRelInfoResIE_Extensions(
    pub Vec<TNLInformationEnhRelInfoResIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargetENB_IDIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TargetENB_IDIE_Extensions(pub Vec<TargetENB_IDIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum TargetRNC_IDIE_Extensions_EntryExtensionValue {
    #[asn(key = 171)]
    Id_ExtendedRNC_ID(ExtendedRNC_ID),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargetRNC_IDIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: TargetRNC_IDIE_Extensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TargetRNC_IDIE_Extensions(pub Vec<TargetRNC_IDIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum TargetRNC_ToSourceRNC_TransparentContainerIE_Extensions_EntryExtensionValue {
    #[asn(key = 295)]
    Id_UeApplicationLayerMeasurementSupportIndication(
        UeApplicationLayerMeasurementSupportIndication,
    ),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargetRNC_ToSourceRNC_TransparentContainerIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value:
        TargetRNC_ToSourceRNC_TransparentContainerIE_Extensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TargetRNC_ToSourceRNC_TransparentContainerIE_Extensions(
    pub Vec<TargetRNC_ToSourceRNC_TransparentContainerIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum TrCH_IDIE_Extensions_EntryExtensionValue {
    #[asn(key = 160)]
    Id_E_DCH_MAC_d_Flow_ID(E_DCH_MAC_d_Flow_ID),
    #[asn(key = 117)]
    Id_hS_DSCH_MAC_d_Flow_ID(HS_DSCH_MAC_d_Flow_ID),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TrCH_IDIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: TrCH_IDIE_Extensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TrCH_IDIE_Extensions(pub Vec<TrCH_IDIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TraceInformationIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TraceInformationIE_Extensions(pub Vec<TraceInformationIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TracePropagationParametersIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TracePropagationParametersIE_Extensions(
    pub Vec<TracePropagationParametersIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TraceRecordingSessionInformationIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TraceRecordingSessionInformationIE_Extensions(
    pub Vec<TraceRecordingSessionInformationIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TransportLayerInformationIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TransportLayerInformationIE_Extensions(
    pub Vec<TransportLayerInformationIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TunnelInformationIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TunnelInformationIE_Extensions(pub Vec<TunnelInformationIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "OCTET-STRING",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "1000"
)]
pub struct OCTET_STRING_63(pub Vec<u8>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UE_IsNotServedIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UE_IsNotServedIE_Extensions(pub Vec<UE_IsNotServedIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UE_IsServedIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UE_IsServedIE_Extensions(pub Vec<UE_IsServedIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UESBI_IuIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UESBI_IuIE_Extensions(pub Vec<UESBI_IuIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum UESpecificInformationIndicationProtocolIEs_EntryValue {
    #[asn(key = 118)]
    Id_UESBI_Iu(UESBI_Iu),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UESpecificInformationIndicationProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UESpecificInformationIndicationProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UESpecificInformationIndicationProtocolIEs(
    pub Vec<UESpecificInformationIndicationProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UESpecificInformationIndicationProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UESpecificInformationIndicationProtocolExtensions(
    pub Vec<UESpecificInformationIndicationProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum UPInformationIE_Extensions_EntryExtensionValue {
    #[asn(key = 269)]
    Id_TimingDifferenceULDL(TimingDifferenceULDL),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UPInformationIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: UPInformationIE_Extensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UPInformationIE_Extensions(pub Vec<UPInformationIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UTRAN_CellIDIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UTRAN_CellIDIE_Extensions(pub Vec<UTRAN_CellIDIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeRadioCapabilityMatchRequestProtocolIEs_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UeRadioCapabilityMatchRequestProtocolIEs(
    pub Vec<UeRadioCapabilityMatchRequestProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeRadioCapabilityMatchRequestProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UeRadioCapabilityMatchRequestProtocolExtensions(
    pub Vec<UeRadioCapabilityMatchRequestProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum UeRadioCapabilityMatchResponseProtocolIEs_EntryValue {
    #[asn(key = 258)]
    Id_VoiceSupportMatchIndicator(VoiceSupportMatchIndicator),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeRadioCapabilityMatchResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UeRadioCapabilityMatchResponseProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UeRadioCapabilityMatchResponseProtocolIEs(
    pub Vec<UeRadioCapabilityMatchResponseProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeRadioCapabilityMatchResponseProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UeRadioCapabilityMatchResponseProtocolExtensions(
    pub Vec<UeRadioCapabilityMatchResponseProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum UeRegistrationQueryRequestProtocolIEs_EntryValue {
    #[asn(key = 79)]
    Id_IuSigConId(IuSignallingConnectionIdentifier),
    #[asn(key = 23)]
    Id_PermanentNAS_UE_ID(PermanentNAS_UE_ID),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeRegistrationQueryRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UeRegistrationQueryRequestProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UeRegistrationQueryRequestProtocolIEs(
    pub Vec<UeRegistrationQueryRequestProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeRegistrationQueryRequestProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UeRegistrationQueryRequestProtocolExtensions(
    pub Vec<UeRegistrationQueryRequestProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum UeRegistrationQueryResponseProtocolIEs_EntryValue {
    #[asn(key = 281)]
    Id_UERegistrationQueryResult(UERegistrationQueryResult),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeRegistrationQueryResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UeRegistrationQueryResponseProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UeRegistrationQueryResponseProtocolIEs(
    pub Vec<UeRegistrationQueryResponseProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeRegistrationQueryResponseProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UeRegistrationQueryResponseProtocolExtensions(
    pub Vec<UeRegistrationQueryResponseProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UnsuccessfulLinking_IEs_EntryIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UnsuccessfulLinking_IEs_EntryIE_Extensions(
    pub Vec<UnsuccessfulLinking_IEs_EntryIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UnsuccessfulLinking_IEs_Entry {
    pub tmgi: TMGI,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UnsuccessfulLinking_IEs_EntryIE_Extensions>,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum UnsuccessfulOutcomeValue {
    #[asn(key = 31)]
    Id_InformationTransfer(InformationTransferFailure),
    #[asn(key = 30)]
    Id_LocationRelatedData(LocationRelatedDataFailure),
    #[asn(key = 42)]
    Id_MBMSRABRelease(MBMSRABReleaseFailure),
    #[asn(key = 39)]
    Id_MBMSRegistration(MBMSRegistrationFailure),
    #[asn(key = 35)]
    Id_MBMSSessionStart(MBMSSessionStartFailure),
    #[asn(key = 36)]
    Id_MBMSSessionUpdate(MBMSSessionUpdateFailure),
    #[asn(key = 2)]
    Id_RelocationPreparation(RelocationPreparationFailure),
    #[asn(key = 3)]
    Id_RelocationResourceAllocation(RelocationFailure),
    #[asn(key = 6)]
    Id_SecurityModeControl(SecurityModeReject),
    #[asn(key = 33)]
    Id_UplinkInformationExchange(UplinkInformationExchangeFailure),
    #[asn(key = 43)]
    Id_enhancedRelocationComplete(EnhancedRelocationCompleteFailure),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum UplinkInformationExchangeFailureProtocolIEs_EntryValue {
    #[asn(key = 3)]
    Id_CN_DomainIndicator(CN_DomainIndicator),
    #[asn(key = 4)]
    Id_Cause(Cause),
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 96)]
    Id_GlobalCN_ID(GlobalCN_ID),
    #[asn(key = 136)]
    Id_InformationExchangeID(InformationExchangeID),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkInformationExchangeFailureProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UplinkInformationExchangeFailureProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UplinkInformationExchangeFailureProtocolIEs(
    pub Vec<UplinkInformationExchangeFailureProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkInformationExchangeFailureProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UplinkInformationExchangeFailureProtocolExtensions(
    pub Vec<UplinkInformationExchangeFailureProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum UplinkInformationExchangeRequestProtocolIEs_EntryValue {
    #[asn(key = 3)]
    Id_CN_DomainIndicator(CN_DomainIndicator),
    #[asn(key = 86)]
    Id_GlobalRNC_ID(GlobalRNC_ID),
    #[asn(key = 136)]
    Id_InformationExchangeID(InformationExchangeID),
    #[asn(key = 137)]
    Id_InformationExchangeType(InformationExchangeType),
    #[asn(key = 139)]
    Id_InformationRequestType(InformationRequestType),
    #[asn(key = 123)]
    Id_InformationTransferType(InformationTransferType),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkInformationExchangeRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UplinkInformationExchangeRequestProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UplinkInformationExchangeRequestProtocolIEs(
    pub Vec<UplinkInformationExchangeRequestProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum UplinkInformationExchangeRequestProtocolExtensions_EntryExtensionValue {
    #[asn(key = 171)]
    Id_ExtendedRNC_ID(ExtendedRNC_ID),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkInformationExchangeRequestProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: UplinkInformationExchangeRequestProtocolExtensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UplinkInformationExchangeRequestProtocolExtensions(
    pub Vec<UplinkInformationExchangeRequestProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum UplinkInformationExchangeResponseProtocolIEs_EntryValue {
    #[asn(key = 3)]
    Id_CN_DomainIndicator(CN_DomainIndicator),
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 96)]
    Id_GlobalCN_ID(GlobalCN_ID),
    #[asn(key = 136)]
    Id_InformationExchangeID(InformationExchangeID),
    #[asn(key = 138)]
    Id_InformationRequested(InformationRequested),
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkInformationExchangeResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UplinkInformationExchangeResponseProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UplinkInformationExchangeResponseProtocolIEs(
    pub Vec<UplinkInformationExchangeResponseProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkInformationExchangeResponseProtocolExtensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UplinkInformationExchangeResponseProtocolExtensions(
    pub Vec<UplinkInformationExchangeResponseProtocolExtensions_Entry>,
);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserPlaneInformationIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UserPlaneInformationIE_Extensions(pub Vec<UserPlaneInformationIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: AperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct INTEGER_64(pub u8);

fn main() {
    use asn1_codecs::aper::*;
    eprintln!("RANAP");

    let ranap_data = hex::decode("000100080000010004400122").unwrap();
    let mut codec_data = AperCodecData::from_slice(&ranap_data);
    let ranap_pdu = RANAP_PDU::decode(&mut codec_data).unwrap();
    eprintln!("ranap_pdu: {:#?}", ranap_pdu);

    let ranap_data = hex::decode("0014400f0000020010400302832a003b400100").unwrap();
    let mut codec_data = AperCodecData::from_slice(&ranap_data);
    let ranap_pdu = RANAP_PDU::decode(&mut codec_data).unwrap();
    eprintln!("ranap_pdu: {:#?}", ranap_pdu);
}
