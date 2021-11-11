#![allow(dead_code, unreachable_patterns, non_camel_case_types)]
use asn1_codecs_derive::AperCodec;
use bitvec::order::Msb0;
use bitvec::vec::BitVec;

#[derive(Debug, AperCodec)]
#[asn(
    type = "OCTET-STRING",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "255"
)]
pub struct APN(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct AccuracyFulfilmentIndicator(u8);
impl AccuracyFulfilmentIndicator {
    const REQUESTED_ACCURACY_FULFILLED: u8 = 0u8;
    const REQUESTED_ACCURACY_NOT_FULFILLED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct Additional_CSPS_coordination_information {
    #[asn(optional_idx = 0)]
    pub old_lai: Option<LAI>,
    #[asn(optional_idx = 1)]
    pub old_rac: Option<RAC>,
    #[asn(optional_idx = 2)]
    pub n_ri: Option<BIT_STRING_2>,
    #[asn(optional_idx = 3)]
    pub u_e_is_attaching: Option<NULL_3>,
    #[asn(optional_idx = 4)]
    pub i_e_extensions: Option<Additional_CSPS_coordination_informationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct Additional_PositioningDataSet(Vec<Additional_PositioningMethodAndUsage>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "1", sz_ub = "1")]
pub struct Additional_PositioningMethodAndUsage(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AllocationOrRetentionPriority {
    pub priority_level: PriorityLevel,
    pub pre_emption_capability: Pre_emptionCapability,
    pub pre_emption_vulnerability: Pre_emptionVulnerability,
    pub queuing_allowed: QueuingAllowed,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<AllocationOrRetentionPriorityiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Alt_RAB_Parameter_ExtendedGuaranteedBitrateInf {
    pub alt_extended_guaranteed_bitrate_type: Alt_RAB_Parameter_GuaranteedBitrateType,
    #[asn(optional_idx = 0)]
    pub alt_extended_guaranteed_bitrates: Option<Alt_RAB_Parameter_ExtendedGuaranteedBitrates>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct Alt_RAB_Parameter_ExtendedGuaranteedBitrateList(Vec<ExtendedGuaranteedBitrate>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct Alt_RAB_Parameter_ExtendedGuaranteedBitrates(
    Vec<Alt_RAB_Parameter_ExtendedGuaranteedBitrateList>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Alt_RAB_Parameter_ExtendedMaxBitrateInf {
    pub alt_extended_max_bitrate_type: Alt_RAB_Parameter_MaxBitrateType,
    #[asn(optional_idx = 0)]
    pub alt_extended_max_bitrates: Option<Alt_RAB_Parameter_ExtendedMaxBitrates>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct Alt_RAB_Parameter_ExtendedMaxBitrateList(Vec<ExtendedMaxBitrate>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct Alt_RAB_Parameter_ExtendedMaxBitrates(Vec<Alt_RAB_Parameter_ExtendedMaxBitrateList>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Alt_RAB_Parameter_GuaranteedBitrateInf {
    pub alt_guaranteed_bitrate_type: Alt_RAB_Parameter_GuaranteedBitrateType,
    #[asn(optional_idx = 0)]
    pub alt_guaranteed_bitrates: Option<Alt_RAB_Parameter_GuaranteedBitrates>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct Alt_RAB_Parameter_GuaranteedBitrateList(Vec<GuaranteedBitrate>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct Alt_RAB_Parameter_GuaranteedBitrateType(u8);
impl Alt_RAB_Parameter_GuaranteedBitrateType {
    const UNSPECIFIED: u8 = 0u8;
    const VALUE_RANGE: u8 = 1u8;
    const DISCRETE_VALUES: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct Alt_RAB_Parameter_GuaranteedBitrates(Vec<Alt_RAB_Parameter_GuaranteedBitrateList>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Alt_RAB_Parameter_MaxBitrateInf {
    pub alt_max_bitrate_type: Alt_RAB_Parameter_MaxBitrateType,
    #[asn(optional_idx = 0)]
    pub alt_max_bitrates: Option<Alt_RAB_Parameter_MaxBitrates>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct Alt_RAB_Parameter_MaxBitrateList(Vec<MaxBitrate>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct Alt_RAB_Parameter_MaxBitrateType(u8);
impl Alt_RAB_Parameter_MaxBitrateType {
    const UNSPECIFIED: u8 = 0u8;
    const VALUE_RANGE: u8 = 1u8;
    const DISCRETE_VALUES: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct Alt_RAB_Parameter_MaxBitrates(Vec<Alt_RAB_Parameter_MaxBitrateList>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct Alt_RAB_Parameter_SupportedGuaranteedBitrateInf {
    pub alt_supported_guaranteed_bitrate_type: Alt_RAB_Parameter_GuaranteedBitrateType,
    #[asn(optional_idx = 0)]
    pub alt_supported_guaranteed_bitrates: Option<Alt_RAB_Parameter_SupportedGuaranteedBitrates>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<Alt_RAB_Parameter_SupportedGuaranteedBitrateInfiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct Alt_RAB_Parameter_SupportedGuaranteedBitrates(Vec<SupportedRAB_ParameterBitrateList>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct Alt_RAB_Parameter_SupportedMaxBitrateInf {
    pub alt_supported_max_bitrate_type: Alt_RAB_Parameter_MaxBitrateType,
    #[asn(optional_idx = 0)]
    pub alt_supported_max_bitrates: Option<Alt_RAB_Parameter_SupportedMaxBitrates>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<Alt_RAB_Parameter_SupportedMaxBitrateInfiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct Alt_RAB_Parameter_SupportedMaxBitrates(Vec<SupportedRAB_ParameterBitrateList>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct Alt_RAB_Parameters {
    #[asn(optional_idx = 0)]
    pub alt_max_bitrate_inf: Option<Alt_RAB_Parameter_MaxBitrateInf>,
    #[asn(optional_idx = 1)]
    pub alt_guaranteed_bit_rate_inf: Option<Alt_RAB_Parameter_GuaranteedBitrateInf>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<Alt_RAB_ParametersiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct AlternativeRABConfigurationRequest(u8);
impl AlternativeRABConfigurationRequest {
    const ALTERNATIVE_RAB_CONFIGURATION_REQUESTED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum AreaIdentity {
    #[asn(key = 0, extended = false)]
    sAI(SAI),
    #[asn(key = 1, extended = false)]
    geographicalArea(GeographicalArea),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = true)]
pub enum AreaScopeForUEApplicationLayerMeasurementConfiguration {
    #[asn(key = 0, extended = false)]
    cellbased(CellBased),
    #[asn(key = 1, extended = false)]
    labased(LABased),
    #[asn(key = 2, extended = false)]
    rabased(RABased),
    #[asn(key = 3, extended = false)]
    plmn_area_based(PLMNBased),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct Ass_RAB_Parameter_ExtendedGuaranteedBitrateList(Vec<ExtendedGuaranteedBitrate>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct Ass_RAB_Parameter_ExtendedMaxBitrateList(Vec<ExtendedMaxBitrate>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct Ass_RAB_Parameter_GuaranteedBitrateList(Vec<GuaranteedBitrate>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct Ass_RAB_Parameter_MaxBitrateList(Vec<MaxBitrate>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct Ass_RAB_Parameters {
    #[asn(optional_idx = 0)]
    pub ass_max_bitrate_inf: Option<Ass_RAB_Parameter_MaxBitrateList>,
    #[asn(optional_idx = 1)]
    pub ass_guaranteed_bit_rate_inf: Option<Ass_RAB_Parameter_GuaranteedBitrateList>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<Ass_RAB_ParametersiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct AuthorisedPLMNs(Vec<AuthorisedPLMNs_Item>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65536"
)]
pub struct AuthorisedSNAs(Vec<SNAC>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "30000", ub = "115000")]
pub struct BarometricPressure(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct BindingID(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct BroadcastAssistanceDataDecipheringKeys {
    pub ciphering_key_flag: BIT_STRING_4,
    pub current_deciphering_key: BIT_STRING_5,
    pub next_deciphering_key: BIT_STRING_6,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct CGI {
    pub p_lm_nidentity: PLMNidentity,
    pub l_ac: LAC,
    pub c_i: CI,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<CGIiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct CI(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CN_DeactivateTrace {
    pub protocol_i_es: CN_DeactivateTraceprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<CN_DeactivateTraceprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct CN_DomainIndicator(u8);
impl CN_DomainIndicator {
    const CS_DOMAIN: u8 = 0u8;
    const PS_DOMAIN: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct CN_ID(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CN_InvokeTrace {
    pub protocol_i_es: CN_InvokeTraceprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<CN_InvokeTraceprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CNMBMSLinkingInformation {
    pub joined_mbms_bearer_service_i_es: JoinedMBMSBearerService_IEs,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<CNMBMSLinkingInformationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct CSFB_Information(u8);
impl CSFB_Information {
    const CSFB: u8 = 0u8;
    const CSFB_HIGH_PRIORITY: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "27", sz_ub = "27")]
pub struct CSG_Id(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct CSG_Id_List(Vec<CSG_Id>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct CSG_Membership_Status(u8);
impl CSG_Membership_Status {
    const MEMBER: u8 = 0u8;
    const NON_MEMBER: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "5", extensible = true)]
pub enum Cause {
    #[asn(key = 0, extended = false)]
    radioNetwork(CauseRadioNetwork),
    #[asn(key = 1, extended = false)]
    transmissionNetwork(CauseTransmissionNetwork),
    #[asn(key = 2, extended = false)]
    nAS(CauseNAS),
    #[asn(key = 3, extended = false)]
    protocol(CauseProtocol),
    #[asn(key = 4, extended = false)]
    misc(CauseMisc),
    #[asn(key = 5, extended = false)]
    non_Standard(CauseNon_Standard),
    #[asn(key = 0, extended = true)]
    radioNetworkExtension(CauseRadioNetworkExtension),
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "113", ub = "128")]
pub struct CauseMisc(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "81", ub = "96")]
pub struct CauseNAS(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "129", ub = "256")]
pub struct CauseNon_Standard(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "97", ub = "112")]
pub struct CauseProtocol(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "64")]
pub struct CauseRadioNetwork(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "257", ub = "512")]
pub struct CauseRadioNetworkExtension(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "65", ub = "80")]
pub struct CauseTransmissionNetwork(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Cell_Access_Mode(u8);
impl Cell_Access_Mode {
    const HYBRID: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "100", extensible = true)]
pub struct Cell_Capacity_Class_Value(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "268435455")]
pub struct Cell_Id(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellBased {
    pub cell_id_list: CellIdList,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<CellBasediE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct CellIdList(Vec<Cell_Id>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct CellLoadInformation {
    pub cell_capacity_class_value: Cell_Capacity_Class_Value,
    pub load_value: LoadValue,
    #[asn(optional_idx = 0)]
    pub r_t_load_value: Option<RTLoadValue>,
    #[asn(optional_idx = 1)]
    pub n_rt_load_information_value: Option<NRTLoadInformationValue>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<CellLoadInformationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct CellLoadInformationGroup {
    pub source_cell_id: SourceCellID,
    #[asn(optional_idx = 0)]
    pub uplink_cell_load_information: Option<CellLoadInformation>,
    #[asn(optional_idx = 1)]
    pub downlink_cell_load_information: Option<CellLoadInformation>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<CellLoadInformationGroupiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct CellType(u8);
impl CellType {
    const MACRO: u8 = 0u8;
    const MICRO: u8 = 1u8;
    const PICO: u8 = 2u8;
    const FEMTO: u8 = 3u8;
}

pub type ChosenEncryptionAlgorithm = EncryptionAlgorithm;

pub type ChosenIntegrityProtectionAlgorithm = IntegrityProtectionAlgorithm;

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct CivicAddress(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct ClassmarkInformation2(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct ClassmarkInformation3(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "7")]
pub struct ClientType(u8);
impl ClientType {
    const EMERGENCY_SERVICES: u8 = 0u8;
    const VALUE_ADDED_SERVICES: u8 = 1u8;
    const P_LMN_OPERATOR_SERVICES: u8 = 2u8;
    const LAWFUL_INTERCEPT_SERVICES: u8 = 3u8;
    const P_LMN_OPERATOR_BROADCAST_SERVICES: u8 = 4u8;
    const P_LMN_OPERATOR_O_ET_M: u8 = 5u8;
    const P_LMN_OPERATOR_ANONYMOUS_STATISTICS: u8 = 6u8;
    const P_LMN_OPERATOR_TARGET_MS_SERVICE_SUPPORT: u8 = 7u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CommonID {
    pub protocol_i_es: CommonIDprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<CommonIDprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct Correlation_ID(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct Criticality(u8);
impl Criticality {
    const REJECT: u8 = 0u8;
    const IGNORE: u8 = 1u8;
    const NOTIFY: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
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
    pub i_e_extensions: Option<CriticalityDiagnosticsiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct CriticalityDiagnostics_IE_List(Vec<CriticalityDiagnostics_IE_List_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1048575")]
pub struct D_RNTI(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct DCH_ID(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct DCN_ID(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct DL_GTP_PDU_SequenceNumber(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct DL_N_PDU_SequenceNumber(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "6", ub = "9")]
pub struct DRX_CycleLengthCoefficient(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct DSCH_ID(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct DataPDUType(u8);
impl DataPDUType {
    const P_D_UTYPE0: u8 = 0u8;
    const P_D_UTYPE1: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct DataVolumeList(Vec<DataVolumeList_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct DataVolumeReference(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DataVolumeReport {
    pub protocol_i_es: DataVolumeReportprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<DataVolumeReportprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DataVolumeReportRequest {
    pub protocol_i_es: DataVolumeReportRequestprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<DataVolumeReportRequestprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct DataVolumeReportingIndication(u8);
impl DataVolumeReportingIndication {
    const DO_REPORT: u8 = 0u8;
    const DO_NOT_REPORT: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct DeliveryOfErroneousSDU(u8);
impl DeliveryOfErroneousSDU {
    const YES: u8 = 0u8;
    const NO: u8 = 1u8;
    const NO_ERROR_DETECTION_CONSIDERATION: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct DeliveryOrder(u8);
impl DeliveryOrder {
    const DELIVERY_ORDER_REQUESTED: u8 = 0u8;
    const DELIVERY_ORDER_NOT_REQUESTED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 3)]
pub struct DeltaRAListofIdleModeUEs {
    #[asn(optional_idx = 0)]
    pub new_ra_listof_idle_mode_u_es: Option<NewRAListofIdleModeUEs>,
    #[asn(optional_idx = 1)]
    pub r_a_listwith_no_idle_mode_u_es_any_more: Option<RAListwithNoIdleModeUEsAnyMore>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<DeltaRAListofIdleModeUEsiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DirectInformationTransfer {
    pub protocol_i_es: DirectInformationTransferprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<DirectInformationTransferprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct DirectReportingIndicator(u8);
impl DirectReportingIndicator {
    const DIRECT_SAI: u8 = 0u8;
    const DIRECT_GEO: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DirectTransfer {
    pub protocol_i_es: DirectTransferprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<DirectTransferprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DirectTransferInformationItem_RANAP_RelocInf {
    pub n_as_pdu: NAS_PDU,
    pub s_api: SAPI,
    pub c_n_domain_indicator: CN_DomainIndicator,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<DirectTransferInformationItem_RANAP_RelocInfiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "15")]
pub struct DirectTransferInformationList_RANAP_RelocInf(
    Vec<DirectTransferInformationList_RANAP_RelocInf_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "7")]
pub struct E_DCH_MAC_d_Flow_ID(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct E_UTRAN_Service_Handover(u8);
impl E_UTRAN_Service_Handover {
    const HANDOVER_TO_E_UTRAN_SHALL_NOT_BE_PERFORMED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "65536", ub = "262143", extensible = true)]
pub struct EARFCN_Extended(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum ENB_ID {
    #[asn(key = 0, extended = false)]
    macroENB_ID(BIT_STRING_7),
    #[asn(key = 1, extended = false)]
    homeENB_ID(BIT_STRING_8),
    #[asn(key = 0, extended = true)]
    short_macroENB_ID(BIT_STRING_9),
    #[asn(key = 1, extended = true)]
    long_macroENB_ID(BIT_STRING_10),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct EUTRANFrequencies(Vec<EUTRANFrequencies_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct EncryptionAlgorithm(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct EncryptionInformation {
    pub permitted_algorithms: PermittedEncryptionAlgorithms,
    pub key: EncryptionKey,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<EncryptionInformationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "BITSTRING",
    sz_extensible = false,
    sz_lb = "128",
    sz_ub = "128"
)]
pub struct EncryptionKey(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct End_Of_CSFB(u8);
impl End_Of_CSFB {
    const END_OF_CSFB: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EnhancedRelocationCompleteConfirm {
    pub protocol_i_es: EnhancedRelocationCompleteConfirmprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<EnhancedRelocationCompleteConfirmprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EnhancedRelocationCompleteFailure {
    pub protocol_i_es: EnhancedRelocationCompleteFailureprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<EnhancedRelocationCompleteFailureprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EnhancedRelocationCompleteRequest {
    pub protocol_i_es: EnhancedRelocationCompleteRequestprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<EnhancedRelocationCompleteRequestprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EnhancedRelocationCompleteResponse {
    pub protocol_i_es: EnhancedRelocationCompleteResponseprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<EnhancedRelocationCompleteResponseprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = true)]
pub enum EquipmentsToBeTraced {
    #[asn(key = 0, extended = false)]
    iMEIlist(IMEIList),
    #[asn(key = 1, extended = false)]
    iMEISVlist(IMEISVList),
    #[asn(key = 2, extended = false)]
    iMEIgroup(IMEIGroup),
    #[asn(key = 3, extended = false)]
    iMEISVgroup(IMEISVGroup),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ErrorIndication {
    pub protocol_i_es: ErrorIndicationprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<ErrorIndicationprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct Event(u8);
impl Event {
    const STOP_CHANGE_OF_SERVICE_AREA: u8 = 0u8;
    const DIRECT: u8 = 1u8;
    const CHANGE_OF_SERVICEAREA: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct Event1F_Parameters {
    pub measurement_quantity: MeasurementQuantity,
    pub threshold: INTEGER_12,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct Event1I_Parameters {
    pub threshold: INTEGER_13,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "16000001", ub = "256000000")]
pub struct ExtendedGuaranteedBitrate(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "16000001", ub = "256000000")]
pub struct ExtendedMaxBitrate(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "4096", ub = "65535")]
pub struct ExtendedRNC_ID(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ForwardSRNS_Context {
    pub protocol_i_es: ForwardSRNS_ContextprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<ForwardSRNS_ContextprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ForwardingIndication(u8);
impl ForwardingIndication {
    const FORWARDING_ADMITTED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct FrameSequenceNumber(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct FrequenceLayerConvergenceFlag(u8);
impl FrequenceLayerConvergenceFlag {
    const NO_FLC_FLAG: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GA_AltitudeAndDirection {
    pub direction_of_altitude: ENUMERATED_14,
    pub altitude: INTEGER_15,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GA_EllipsoidArc {
    pub geographical_coordinates: GeographicalCoordinates,
    pub inner_radius: INTEGER_16,
    pub uncertainty_radius: INTEGER_17,
    pub offset_angle: INTEGER_18,
    pub included_angle: INTEGER_19,
    pub confidence: INTEGER_20,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<GA_EllipsoidArciE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GA_Point {
    pub geographical_coordinates: GeographicalCoordinates,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<GA_PointiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GA_PointWithAltitude {
    pub geographical_coordinates: GeographicalCoordinates,
    pub altitude_and_direction: GA_AltitudeAndDirection,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<GA_PointWithAltitudeiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GA_PointWithAltitudeAndUncertaintyEllipsoid {
    pub geographical_coordinates: GeographicalCoordinates,
    pub altitude_and_direction: GA_AltitudeAndDirection,
    pub uncertainty_ellipse: GA_UncertaintyEllipse,
    pub uncertainty_altitude: INTEGER_21,
    pub confidence: INTEGER_22,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<GA_PointWithAltitudeAndUncertaintyEllipsoidiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct GA_PointWithUnCertainty {
    pub geographical_coordinates: GeographicalCoordinates,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<GA_PointWithUnCertaintyiE_Extensions>,
    pub uncertainty_code: INTEGER_23,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GA_PointWithUnCertaintyEllipse {
    pub geographical_coordinates: GeographicalCoordinates,
    pub uncertainty_ellipse: GA_UncertaintyEllipse,
    pub confidence: INTEGER_24,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<GA_PointWithUnCertaintyEllipseiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "15")]
pub struct GA_Polygon(Vec<GA_Polygon_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GA_UncertaintyEllipse {
    pub uncertainty_semi_major: INTEGER_25,
    pub uncertainty_semi_minor: INTEGER_26,
    pub orientation_of_major_axis: INTEGER_27,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "9")]
pub struct GANSS_PositioningDataSet(Vec<GANSS_PositioningMethodAndUsage>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "1", sz_ub = "1")]
pub struct GANSS_PositioningMethodAndUsage(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct GERAN_BSC_Container(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct GERAN_Cell_ID {
    pub l_ai: LAI,
    pub r_ac: RAC,
    pub c_i: CI,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<GERAN_Cell_IDiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct GERAN_Classmark(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct GERAN_Iumode_RAB_Failed_RABAssgntResponse_Item {
    pub r_ab_id: RAB_ID,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub g_eran_classmark: Option<GERAN_Classmark>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<GERAN_Iumode_RAB_Failed_RABAssgntResponse_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct GERAN_Iumode_RAB_FailedList_RABAssgntResponse(
    Vec<GERAN_Iumode_RAB_FailedList_RABAssgntResponse_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct GTP_TEI(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = true)]
pub enum GeographicalArea {
    #[asn(key = 0, extended = false)]
    point(GA_Point),
    #[asn(key = 1, extended = false)]
    pointWithUnCertainty(GA_PointWithUnCertainty),
    #[asn(key = 2, extended = false)]
    polygon(GA_Polygon),
    #[asn(key = 0, extended = true)]
    pointWithUncertaintyEllipse(GA_PointWithUnCertaintyEllipse),
    #[asn(key = 1, extended = true)]
    pointWithAltitude(GA_PointWithAltitude),
    #[asn(key = 2, extended = true)]
    pointWithAltitudeAndUncertaintyEllipsoid(GA_PointWithAltitudeAndUncertaintyEllipsoid),
    #[asn(key = 3, extended = true)]
    ellipsoidArc(GA_EllipsoidArc),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GeographicalCoordinates {
    pub latitude_sign: ENUMERATED_28,
    pub latitude: INTEGER_29,
    pub longitude: INTEGER_30,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<GeographicalCoordinatesiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalCN_ID {
    pub p_lm_nidentity: PLMNidentity,
    pub c_n_id: CN_ID,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalRNC_ID {
    pub p_lm_nidentity: PLMNidentity,
    pub r_nc_id: RNC_ID,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "16000000")]
pub struct GuaranteedBitrate(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "7")]
pub struct HS_DSCH_MAC_d_Flow_ID(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct HigherBitratesThan16MbpsFlag(u8);
impl HigherBitratesThan16MbpsFlag {
    const ALLOWED: u8 = 0u8;
    const NOT_ALLOWED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HorizontalSpeedAndBearing {
    pub bearing: INTEGER_31,
    pub horizontal_speed: INTEGER_32,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct HorizontalVelocity {
    pub horizontal_speed_and_bearing: HorizontalSpeedAndBearing,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<HorizontalVelocityiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct HorizontalVelocityWithUncertainty {
    pub horizontal_speed_and_bearing: HorizontalSpeedAndBearing,
    pub uncertainty_speed: INTEGER_33,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<HorizontalVelocityWithUncertaintyiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct HorizontalWithVerticalVelocity {
    pub horizontal_speed_and_bearing: HorizontalSpeedAndBearing,
    pub veritcal_velocity: VerticalVelocity,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<HorizontalWithVerticalVelocityiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct HorizontalWithVerticalVelocityAndUncertainty {
    pub horizontal_speed_and_bearing: HorizontalSpeedAndBearing,
    pub veritcal_velocity: VerticalVelocity,
    pub horizontal_uncertainty_speed: INTEGER_34,
    pub vertical_uncertainty_speed: INTEGER_35,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<HorizontalWithVerticalVelocityAndUncertaintyiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct IMEI(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct IMEIGroup {
    pub i_mei: IMEI,
    pub i_mei_mask: BIT_STRING_36,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<IMEIGroupiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct IMEIList(Vec<IMEI>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct IMEISV(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct IMEISVGroup {
    pub i_meisv: IMEISV,
    pub i_meisv_mask: BIT_STRING_37,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<IMEISVGroupiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct IMEISVList(Vec<IMEISV>);

pub type IMSI = TBCD_STRING;

#[derive(Debug, AperCodec)]
#[asn(
    type = "OCTET-STRING",
    sz_extensible = false,
    sz_lb = "4",
    sz_ub = "16"
)]
pub struct IPMulticastAddress(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 3)]
pub struct IRAT_Measurement_Configuration {
    #[asn(optional_idx = 0)]
    pub r_srp: Option<INTEGER_38>,
    #[asn(optional_idx = 1)]
    pub r_srq: Option<INTEGER_39>,
    pub i_ra_tmeasurement_parameters: IRATmeasurementParameters,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<IRAT_Measurement_ConfigurationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct IRATmeasurementParameters {
    pub measurement_duration: INTEGER_40,
    #[asn(optional_idx = 0)]
    pub e_utran_frequencies: Option<EUTRANFrequencies>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<IRATmeasurementParametersiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct ImmediateMDT {
    pub measurements_to_activate: MeasurementsToActivate,
    #[asn(optional_idx = 0)]
    pub m1report: Option<M1Report>,
    #[asn(optional_idx = 1)]
    pub m2report: Option<M2Report>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<ImmediateMDTiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct IncludeVelocity(u8);
impl IncludeVelocity {
    const REQUESTED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1048575")]
pub struct InformationExchangeID(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct InformationExchangeType(u8);
impl InformationExchangeType {
    const TRANSFER: u8 = 0u8;
    const REQUEST: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum InformationRequestType {
    #[asn(key = 0, extended = false)]
    mBMSIPMulticastAddressandAPNRequest(MBMSIPMulticastAddressandAPNRequest),
    #[asn(key = 1, extended = false)]
    permanentNAS_UE_ID(PermanentNAS_UE_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum InformationRequested {
    #[asn(key = 0, extended = false)]
    requestedMBMSIPMulticastAddressandAPNRequest(RequestedMBMSIPMulticastAddressandAPNRequest),
    #[asn(key = 1, extended = false)]
    requestedMulticastServiceList(RequestedMulticastServiceList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct InformationTransferConfirmation {
    pub protocol_i_es: InformationTransferConfirmationprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<InformationTransferConfirmationprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct InformationTransferFailure {
    pub protocol_i_es: InformationTransferFailureprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<InformationTransferFailureprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1048575")]
pub struct InformationTransferID(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct InformationTransferIndication {
    pub protocol_i_es: InformationTransferIndicationprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<InformationTransferIndicationprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "0", extensible = true)]
pub enum InformationTransferType {
    #[asn(key = 0, extended = false)]
    rNCTraceInformation(RNCTraceInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct InitialUE_Message {
    pub protocol_i_es: InitialUE_MessageprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<InitialUE_MessageprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InitiatingMessage {
    #[asn(key_field = true)]
    pub procedure_code: ProcedureCode,
    pub criticality: Criticality,
    pub value: InitiatingMessagevalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct IntegrityProtectionAlgorithm(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct IntegrityProtectionInformation {
    pub permitted_algorithms: PermittedIntegrityProtectionAlgorithms,
    pub key: IntegrityProtectionKey,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<IntegrityProtectionInformationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "BITSTRING",
    sz_extensible = false,
    sz_lb = "128",
    sz_ub = "128"
)]
pub struct IntegrityProtectionKey(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct InterSystemInformation_TransparentContainer {
    #[asn(optional_idx = 0)]
    pub downlink_cell_load_information: Option<CellLoadInformation>,
    #[asn(optional_idx = 1)]
    pub uplink_cell_load_information: Option<CellLoadInformation>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<InterSystemInformation_TransparentContaineriE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "0", extensible = true)]
pub enum InterSystemInformationTransferType {
    #[asn(key = 0, extended = false)]
    rIM_Transfer(RIM_Transfer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct InterfacesToTraceItem {
    pub interface: ENUMERATED_41,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<InterfacesToTraceItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Iu_ReleaseCommand {
    pub protocol_i_es: Iu_ReleaseCommandprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<Iu_ReleaseCommandprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Iu_ReleaseComplete {
    pub protocol_i_es: Iu_ReleaseCompleteprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<Iu_ReleaseCompleteprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Iu_ReleaseRequest {
    pub protocol_i_es: Iu_ReleaseRequestprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<Iu_ReleaseRequestprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "24", sz_ub = "24")]
pub struct IuSignallingConnectionIdentifier(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum IuTransportAssociation {
    #[asn(key = 0, extended = false)]
    gTP_TEI(GTP_TEI),
    #[asn(key = 1, extended = false)]
    bindingID(BindingID),
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "128"
)]
pub struct JoinedMBMSBearerService_IEs(Vec<JoinedMBMSBearerService_IEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct KeyStatus(u8);
impl KeyStatus {
    const OLD: u8 = 0u8;
    const NEW: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct L3_Information(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65536"
)]
pub struct LA_LIST(Vec<LA_LIST_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LABased {
    pub lai_list: LAI_List,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<LABasediE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct LAC(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct LAI {
    pub p_lm_nidentity: PLMNidentity,
    pub l_ac: LAC,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<LAIiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct LAI_List(Vec<LAI>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65536"
)]
pub struct LAListofIdleModeUEs(Vec<LAI>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "OCTET-STRING",
    sz_extensible = false,
    sz_lb = "32",
    sz_ub = "256"
)]
pub struct LHN_ID(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LastKnownServiceArea {
    pub s_ai: SAI,
    pub age_of_sai: INTEGER_42,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<LastKnownServiceAreaiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LastVisitedUTRANCell_Item {
    pub u_tran_cell_id: UTRAN_CellID,
    pub cell_type: CellType,
    pub time_ue_stayed_in_cell: Time_UE_StayedInCell,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<LastVisitedUTRANCell_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "128"
)]
pub struct LeftMBMSBearerService_IEs(Vec<LeftMBMSBearerService_IEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct Links_to_log(u8);
impl Links_to_log {
    const UPLINK: u8 = 0u8;
    const DOWNLINK: u8 = 1u8;
    const BOTH_UPLINK_AND_DOWNLINK: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65536"
)]
pub struct ListOF_SNAs(Vec<SNAC>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct ListOfInterfacesToTrace(Vec<InterfacesToTraceItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct LoadValue(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LocationRelatedDataFailure {
    pub protocol_i_es: LocationRelatedDataFailureprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<LocationRelatedDataFailureprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LocationRelatedDataRequest {
    pub protocol_i_es: LocationRelatedDataRequestprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<LocationRelatedDataRequestprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LocationRelatedDataRequestType {
    pub requested_location_related_data_type: RequestedLocationRelatedDataType,
    #[asn(optional_idx = 0)]
    pub requested_gps_assistance_data: Option<RequestedGPSAssistanceData>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct LocationRelatedDataRequestTypeSpecificToGERANIuMode(u8);
impl LocationRelatedDataRequestTypeSpecificToGERANIuMode {
    const DECIPHERING_KEYS_EOTD: u8 = 0u8;
    const DEDICATED_MOBILE_ASSISTED_EOTD_ASSISTANCE_DATA: u8 = 1u8;
    const DEDICATED_MOBILE_BASED_EOTD_ASSISTANCE_DATA: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LocationRelatedDataResponse {
    pub protocol_i_es: LocationRelatedDataResponseprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<LocationRelatedDataResponseprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LocationReport {
    pub protocol_i_es: LocationReportprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<LocationReportprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LocationReportingControl {
    pub protocol_i_es: LocationReportingControlprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<LocationReportingControlprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
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
    pub i_e_extensions: Option<LocationReportingTransferInformationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LoggedMDT {
    pub logging_interval: LoggingInterval,
    pub logging_duration: LoggingDuration,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<LoggedMDTiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "5")]
pub struct LoggingDuration(u8);
impl LoggingDuration {
    const MIN10: u8 = 0u8;
    const MIN20: u8 = 1u8;
    const MIN40: u8 = 2u8;
    const MIN60: u8 = 3u8;
    const MIN90: u8 = 4u8;
    const MIN120: u8 = 5u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "7")]
pub struct LoggingInterval(u8);
impl LoggingInterval {
    const S1D28: u8 = 0u8;
    const S2D56: u8 = 1u8;
    const S5D12: u8 = 2u8;
    const S10D24: u8 = 3u8;
    const S20D48: u8 = 4u8;
    const S30D72: u8 = 5u8;
    const S40D96: u8 = 6u8;
    const S61D44: u8 = 7u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum M1Report {
    #[asn(key = 0, extended = false)]
    periodic(MDT_Report_Parameters),
    #[asn(key = 1, extended = false)]
    event1F(Event1F_Parameters),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum M2Report {
    #[asn(key = 0, extended = false)]
    periodic(MDT_Report_Parameters),
    #[asn(key = 1, extended = false)]
    event1I(Event1I_Parameters),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct M4_Collection_Parameters {
    pub m4_period: M4_Period,
    #[asn(optional_idx = 0)]
    pub m4_threshold: Option<M4_Threshold>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<M4_Collection_ParametersiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "7")]
pub struct M4_Period(u8);
impl M4_Period {
    const MS100: u8 = 0u8;
    const MS250: u8 = 1u8;
    const MS500: u8 = 2u8;
    const MS1000: u8 = 3u8;
    const MS2000: u8 = 4u8;
    const MS3000: u8 = 5u8;
    const MS4000: u8 = 6u8;
    const MS6000: u8 = 7u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "31")]
pub struct M4_Threshold(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum M4Report {
    #[asn(key = 0, extended = false)]
    all(NULL_43),
    #[asn(key = 1, extended = false)]
    m4_collection_parameters(M4_Collection_Parameters),
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "7")]
pub struct M5_Period(u8);
impl M5_Period {
    const MS100: u8 = 0u8;
    const MS250: u8 = 1u8;
    const MS500: u8 = 2u8;
    const MS1000: u8 = 3u8;
    const MS2000: u8 = 4u8;
    const MS3000: u8 = 5u8;
    const MS4000: u8 = 6u8;
    const MS6000: u8 = 7u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum M5Report {
    #[asn(key = 0, extended = false)]
    when_available(NULL_44),
    #[asn(key = 1, extended = false)]
    m5_period(M5_Period),
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "12")]
pub struct M6_Period(u8);
impl M6_Period {
    const MS1000: u8 = 0u8;
    const MS2000: u8 = 1u8;
    const MS3000: u8 = 2u8;
    const MS4000: u8 = 3u8;
    const MS6000: u8 = 4u8;
    const MS8000: u8 = 5u8;
    const MS12000: u8 = 6u8;
    const MS16000: u8 = 7u8;
    const MS20000: u8 = 8u8;
    const MS24000: u8 = 9u8;
    const MS28000: u8 = 10u8;
    const MS32000: u8 = 11u8;
    const MS64000: u8 = 12u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct M6Report {
    pub m6_period: M6_Period,
    pub m6_links_to_log: Links_to_log,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<M6ReportiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "12")]
pub struct M7_Period(u8);
impl M7_Period {
    const MS1000: u8 = 0u8;
    const MS2000: u8 = 1u8;
    const MS3000: u8 = 2u8;
    const MS4000: u8 = 3u8;
    const MS6000: u8 = 4u8;
    const MS8000: u8 = 5u8;
    const MS12000: u8 = 6u8;
    const MS16000: u8 = 7u8;
    const MS20000: u8 = 8u8;
    const MS24000: u8 = 9u8;
    const MS28000: u8 = 10u8;
    const MS32000: u8 = 11u8;
    const MS64000: u8 = 12u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct M7Report {
    pub m7_period: M7_Period,
    pub m7_links_to_log: Links_to_log,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<M7ReportiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct MBMS_PTP_RAB_ID(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct MBMSBearerServiceType(u8);
impl MBMSBearerServiceType {
    const MULTICAST: u8 = 0u8;
    const BROADCAST: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct MBMSCNDe_Registration(u8);
impl MBMSCNDe_Registration {
    const NORMALSESSIONSTOP: u8 = 0u8;
    const DEREGISTER: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSCNDe_RegistrationRequest {
    pub protocol_i_es: MBMSCNDe_RegistrationRequestprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSCNDe_RegistrationRequestprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSCNDe_RegistrationResponse {
    pub protocol_i_es: MBMSCNDe_RegistrationResponseprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSCNDe_RegistrationResponseprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct MBMSCountingInformation(u8);
impl MBMSCountingInformation {
    const COUNTING: u8 = 0u8;
    const NOTCOUNTING: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct MBMSHCIndicator(u8);
impl MBMSHCIndicator {
    const UNCOMPRESSED_HEADER: u8 = 0u8;
    const COMPRESSED_HEADER: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "512"
)]
pub struct MBMSIPMulticastAddressandAPNRequest(Vec<TMGI>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSIPMulticastAddressandAPNlist {
    pub t_mgi: TMGI,
    pub i_p_multicast_address: IPMulticastAddress,
    pub a_pn: APN,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<MBMSIPMulticastAddressandAPNlistiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct MBMSLinkingInformation(u8);
impl MBMSLinkingInformation {
    const U_E_HAS_JOINED_MULTICAST_SERVICES: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSRABEstablishmentIndication {
    pub protocol_i_es: MBMSRABEstablishmentIndicationprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSRABEstablishmentIndicationprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSRABRelease {
    pub protocol_i_es: MBMSRABReleaseprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSRABReleaseprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSRABReleaseFailure {
    pub protocol_i_es: MBMSRABReleaseFailureprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSRABReleaseFailureprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSRABReleaseRequest {
    pub protocol_i_es: MBMSRABReleaseRequestprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSRABReleaseRequestprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSRegistrationFailure {
    pub protocol_i_es: MBMSRegistrationFailureprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSRegistrationFailureprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSRegistrationRequest {
    pub protocol_i_es: MBMSRegistrationRequestprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSRegistrationRequestprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct MBMSRegistrationRequestType(u8);
impl MBMSRegistrationRequestType {
    const REGISTER: u8 = 0u8;
    const DEREGISTER: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSRegistrationResponse {
    pub protocol_i_es: MBMSRegistrationResponseprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSRegistrationResponseprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct MBMSServiceArea(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "3", sz_ub = "3")]
pub struct MBMSSessionDuration(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "1", sz_ub = "1")]
pub struct MBMSSessionIdentity(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "1", sz_ub = "1")]
pub struct MBMSSessionRepetitionNumber(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSSessionStart {
    pub protocol_i_es: MBMSSessionStartprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSSessionStartprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSSessionStartFailure {
    pub protocol_i_es: MBMSSessionStartFailureprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSSessionStartFailureprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSSessionStartResponse {
    pub protocol_i_es: MBMSSessionStartResponseprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSSessionStartResponseprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSSessionStop {
    pub protocol_i_es: MBMSSessionStopprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSSessionStopprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSSessionStopResponse {
    pub protocol_i_es: MBMSSessionStopResponseprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSSessionStopResponseprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSSessionUpdate {
    pub protocol_i_es: MBMSSessionUpdateprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSSessionUpdateprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSSessionUpdateFailure {
    pub protocol_i_es: MBMSSessionUpdateFailureprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSSessionUpdateFailureprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSSessionUpdateResponse {
    pub protocol_i_es: MBMSSessionUpdateResponseprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSSessionUpdateResponseprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSSynchronisationInformation {
    pub m_bmshc_indicator: MBMSHCIndicator,
    pub i_p_multicast_address: IPMulticastAddress,
    pub g_tpdlteid: GTP_TEI,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<MBMSSynchronisationInformationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSUELinkingRequest {
    pub protocol_i_es: MBMSUELinkingRequestprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSUELinkingRequestprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSUELinkingResponse {
    pub protocol_i_es: MBMSUELinkingResponseprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSUELinkingResponseprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct MDT_Activation(u8);
impl MDT_Activation {
    const IMMEDIATE_MD_TONLY: u8 = 0u8;
    const LOGGED_MD_TONLY: u8 = 1u8;
    const IMMEDIATE_MD_TAND_TRACE: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MDT_Configuration {
    pub mdt_activation: MDT_Activation,
    pub mdt_area_scope: MDTAreaScope,
    pub mdt_mode: MDTMode,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<MDT_ConfigurationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct MDT_PLMN_List(Vec<PLMNidentity>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct MDT_Report_Parameters {
    pub report_interval: ReportInterval,
    pub report_amount: ReportAmount,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = true)]
pub enum MDTAreaScope {
    #[asn(key = 0, extended = false)]
    cellbased(CellBased),
    #[asn(key = 1, extended = false)]
    labased(LABased),
    #[asn(key = 2, extended = false)]
    rabased(RABased),
    #[asn(key = 3, extended = false)]
    plmn_area_based(NULL_45),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum MDTMode {
    #[asn(key = 0, extended = false)]
    immediateMDT(ImmediateMDT),
    #[asn(key = 1, extended = false)]
    loggedMDT(LoggedMDT),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "1", sz_ub = "9")]
pub struct MSISDN(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Management_Based_MDT_Allowed(u8);
impl Management_Based_MDT_Allowed {
    const ALLOWED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "16000000")]
pub struct MaxBitrate(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "32768")]
pub struct MaxSDU_Size(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "5")]
pub struct MeasBand(u8);
impl MeasBand {
    const V6: u8 = 0u8;
    const V15: u8 = 1u8;
    const V25: u8 = 2u8;
    const V50: u8 = 3u8;
    const V75: u8 = 4u8;
    const V100: u8 = 5u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct MeasurementQuantity(u8);
impl MeasurementQuantity {
    const CPICH_EC_NO: u8 = 0u8;
    const CPICH_RSCP: u8 = 1u8;
    const PATHLOSS: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct MeasurementsToActivate(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct MessageStructure(Vec<MessageStructure_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct NAS_PDU(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct NAS_SequenceNumber(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct NAS_SynchronisationIndicator(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "3")]
pub struct NRTLoadInformationValue(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct NewBSS_To_OldBSS_Information(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65536"
)]
pub struct NewRAListofIdleModeUEs(Vec<RAC>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct NonSearchingIndication(u8);
impl NonSearchingIndication {
    const NON_SEARCHING: u8 = 0u8;
    const SEARCHING: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct NotEmptyRAListofIdleModeUEs {
    pub r_aof_idle_mode_u_es: RAofIdleModeUEs,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<NotEmptyRAListofIdleModeUEsiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "10", sz_ub = "10")]
pub struct Null_NRI(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "2")]
pub struct NumberOfIuInstances(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "16")]
pub struct NumberOfSteps(u8);

#[derive(Debug, AperCodec)]
#[asn(
    type = "OCTET-STRING",
    sz_extensible = false,
    sz_lb = "3",
    sz_ub = "22"
)]
pub struct OMC_ID(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Offload_RAB_Parameters {
    pub access_point_name: Offload_RAB_Parameters_APN,
    pub charging_characteristics: Offload_RAB_Parameters_ChargingCharacteristics,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<Offload_RAB_ParametersiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "OCTET-STRING",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "255"
)]
pub struct Offload_RAB_Parameters_APN(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct Offload_RAB_Parameters_ChargingCharacteristics(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OldBSS_ToNewBSS_Information(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Out_Of_UTRAN(u8);
impl Out_Of_UTRAN {
    const CELL_RESELECTION_TO_EUTRAN: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Outcome {
    #[asn(key_field = true)]
    pub procedure_code: ProcedureCode,
    pub criticality: Criticality,
    pub value: Outcomevalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Overload {
    pub protocol_i_es: OverloadprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<OverloadprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct P_TMSI(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "4")]
pub struct PDP_Type(u8);
impl PDP_Type {
    const EMPTY: u8 = 0u8;
    const PPP: u8 = 1u8;
    const OSP_IHOSS: u8 = 2u8;
    const IPV4: u8 = 3u8;
    const IPV6: u8 = 4u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct PDP_Type_extension(u8);
impl PDP_Type_extension {
    const IPV4_AND_IPV6: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct PDP_TypeInformation(Vec<PDP_Type>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct PDP_TypeInformation_extension(Vec<PDP_Type_extension>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "3")]
pub struct PDUType14FrameSequenceNumber(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PLMNBased {
    pub plmn_list: PLMNList,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PLMNBasediE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct PLMNList(Vec<PLMNidentity>);

pub type PLMNidentity = TBCD_STRING;

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct PLMNs_in_shared_network(Vec<PLMNs_in_shared_network_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Paging {
    pub protocol_i_es: PagingprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<PagingprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum PagingAreaID {
    #[asn(key = 0, extended = false)]
    lAI(LAI),
    #[asn(key = 1, extended = false)]
    rAI(RAI),
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "4")]
pub struct PagingCause(u8);
impl PagingCause {
    const TERMINATING_CONVERSATIONAL_CALL: u8 = 0u8;
    const TERMINATING_STREAMING_CALL: u8 = 1u8;
    const TERMINATING_INTERACTIVE_CALL: u8 = 2u8;
    const TERMINATING_BACKGROUND_CALL: u8 = 3u8;
    const TERMINATING_LOW_PRIORITY_SIGNALLING: u8 = 4u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PeriodicLocationInfo {
    pub reporting_amount: INTEGER_46,
    pub reporting_interval: INTEGER_47,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PeriodicLocationInfoiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct PeriodicReportingIndicator(u8);
impl PeriodicReportingIndicator {
    const PERIODIC_SAI: u8 = 0u8;
    const PERIODIC_GEO: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "0", extensible = true)]
pub enum PermanentNAS_UE_ID {
    #[asn(key = 0, extended = false)]
    iMSI(IMSI),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct PermittedEncryptionAlgorithms(Vec<EncryptionAlgorithm>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct PermittedIntegrityProtectionAlgorithms(Vec<IntegrityProtectionAlgorithm>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct Port_Number(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PositionData {
    pub positioning_data_discriminator: PositioningDataDiscriminator,
    #[asn(optional_idx = 0)]
    pub positioning_data_set: Option<PositioningDataSet>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<PositionDataiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct PositionDataSpecificToGERANIuMode(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct PositioningDataDiscriminator(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "9")]
pub struct PositioningDataSet(Vec<PositioningMethodAndUsage>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "1", sz_ub = "1")]
pub struct PositioningMethodAndUsage(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct PositioningPriority(u8);
impl PositioningPriority {
    const HIGH_PRIORITY: u8 = 0u8;
    const NORMAL_PRIORITY: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct PowerSavingIndicator(u8);
impl PowerSavingIndicator {
    const PSM_CONFIGURED: u8 = 0u8;
    const E_DRX_CONFIGURED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct Pre_emptionCapability(u8);
impl Pre_emptionCapability {
    const SHALL_NOT_TRIGGER_PRE_EMPTION: u8 = 0u8;
    const MAY_TRIGGER_PRE_EMPTION: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct Pre_emptionVulnerability(u8);
impl Pre_emptionVulnerability {
    const NOT_PRE_EMPTABLE: u8 = 0u8;
    const PRE_EMPTABLE: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct Presence(u8);
impl Presence {
    const OPTIONAL: u8 = 0u8;
    const CONDITIONAL: u8 = 1u8;
    const MANDATORY: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct Priority_Class_Indicator(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct PriorityLevel(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum PrivateIE_ID {
    #[asn(key = 0, extended = false)]
    local(INTEGER_48),
    #[asn(key = 1, extended = false)]
    global(OBJECT_IDENTIFIER_49),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PrivateMessage {
    pub private_i_es: PrivateMessageprivateIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct ProcedureCode(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct ProtocolExtensionID(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct ProtocolIE_ID(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "0", extensible = true)]
pub enum ProvidedData {
    #[asn(key = 0, extended = false)]
    shared_network_information(Shared_Network_Information),
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct QueuingAllowed(u8);
impl QueuingAllowed {
    const QUEUEING_NOT_ALLOWED: u8 = 0u8;
    const QUEUEING_ALLOWED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_AssignmentRequest {
    pub protocol_i_es: RAB_AssignmentRequestprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RAB_AssignmentRequestprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_AssignmentResponse {
    pub protocol_i_es: RAB_AssignmentResponseprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RAB_AssignmentResponseprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct RAB_AsymmetryIndicator(u8);
impl RAB_AsymmetryIndicator {
    const SYMMETRIC_BIDIRECTIONAL: u8 = 0u8;
    const ASYMMETRIC_UNIDIRECTIONAL_DOWNLINK: u8 = 1u8;
    const ASYMMETRIC_UNIDIRECTIONAL_UPLINK: u8 = 2u8;
    const ASYMMETRIC_BIDIRECTIONAL: u8 = 3u8;
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_ContextFailedtoTransferList(Vec<RAB_ContextFailedtoTransferList_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct RAB_ContextItem {
    pub r_ab_id: RAB_ID,
    #[asn(optional_idx = 0)]
    pub dl_gtp_pdu_sequence_number: Option<DL_GTP_PDU_SequenceNumber>,
    #[asn(optional_idx = 1)]
    pub ul_gtp_pdu_sequence_number: Option<UL_GTP_PDU_SequenceNumber>,
    #[asn(optional_idx = 2)]
    pub dl_n_pdu_sequence_number: Option<DL_N_PDU_SequenceNumber>,
    #[asn(optional_idx = 3)]
    pub ul_n_pdu_sequence_number: Option<UL_N_PDU_SequenceNumber>,
    #[asn(optional_idx = 4)]
    pub i_e_extensions: Option<RAB_ContextItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct RAB_ContextItem_RANAP_RelocInf {
    pub r_ab_id: RAB_ID,
    #[asn(optional_idx = 0)]
    pub dl_gtp_pdu_sequence_number: Option<DL_GTP_PDU_SequenceNumber>,
    #[asn(optional_idx = 1)]
    pub ul_gtp_pdu_sequence_number: Option<UL_GTP_PDU_SequenceNumber>,
    #[asn(optional_idx = 2)]
    pub dl_n_pdu_sequence_number: Option<DL_N_PDU_SequenceNumber>,
    #[asn(optional_idx = 3)]
    pub ul_n_pdu_sequence_number: Option<UL_N_PDU_SequenceNumber>,
    #[asn(optional_idx = 4)]
    pub i_e_extensions: Option<RAB_ContextItem_RANAP_RelocInfiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_ContextList(Vec<RAB_ContextList_Item>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_ContextList_RANAP_RelocInf(Vec<RAB_ContextList_RANAP_RelocInf_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_DataForwardingItem {
    pub r_ab_id: RAB_ID,
    pub transport_layer_address: TransportLayerAddress,
    pub iu_transport_association: IuTransportAssociation,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<RAB_DataForwardingItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_DataForwardingItem_SRNS_CtxReq {
    pub r_ab_id: RAB_ID,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<RAB_DataForwardingItem_SRNS_CtxReqiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_DataForwardingList(Vec<RAB_DataForwardingList_Item>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_DataForwardingList_SRNS_CtxReq(Vec<RAB_DataForwardingList_SRNS_CtxReq_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct RAB_DataVolumeReportItem {
    pub r_ab_id: RAB_ID,
    #[asn(optional_idx = 0)]
    pub dl_unsuccessfully_transmitted_data_volume: Option<DataVolumeList>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<RAB_DataVolumeReportItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_DataVolumeReportList(Vec<RAB_DataVolumeReportList_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_DataVolumeReportRequestItem {
    pub r_ab_id: RAB_ID,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<RAB_DataVolumeReportRequestItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_DataVolumeReportRequestList(Vec<RAB_DataVolumeReportRequestList_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_FailedItem {
    pub r_ab_id: RAB_ID,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<RAB_FailedItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_FailedItem_EnhRelocInfoRes {
    pub c_n_domain_indicator: CN_DomainIndicator,
    pub r_ab_id: RAB_ID,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<RAB_FailedItem_EnhRelocInfoResiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_FailedList(Vec<RAB_FailedList_Item>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_FailedList_EnhRelocInfoRes(Vec<RAB_FailedList_EnhRelocInfoRes_Item>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_FailedtoReportList(Vec<RAB_FailedtoReportList_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct RAB_ID(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_ModifyItem {
    pub r_ab_id: RAB_ID,
    pub requested_rab_parameter_values: Requested_RAB_Parameter_Values,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<RAB_ModifyItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_ModifyList(Vec<RAB_ModifyList_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_ModifyRequest {
    pub protocol_i_es: RAB_ModifyRequestprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RAB_ModifyRequestprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct RAB_Parameter_ExtendedGuaranteedBitrateList(Vec<ExtendedGuaranteedBitrate>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct RAB_Parameter_ExtendedMaxBitrateList(Vec<ExtendedMaxBitrate>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct RAB_Parameter_GuaranteedBitrateList(Vec<GuaranteedBitrate>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct RAB_Parameter_MaxBitrateList(Vec<MaxBitrate>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 7)]
pub struct RAB_Parameters {
    pub traffic_class: TrafficClass,
    pub r_ab_asymmetry_indicator: RAB_AsymmetryIndicator,
    pub max_bitrate: RAB_Parameter_MaxBitrateList,
    #[asn(optional_idx = 0)]
    pub guaranteed_bit_rate: Option<RAB_Parameter_GuaranteedBitrateList>,
    pub delivery_order: DeliveryOrder,
    pub max_sdu_size: MaxSDU_Size,
    pub s_du_parameters: SDU_Parameters,
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
    pub i_e_extensions: Option<RAB_ParametersiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_QueuedItem {
    pub r_ab_id: RAB_ID,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<RAB_QueuedItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_QueuedList(Vec<RAB_QueuedList_Item>);

pub type RAB_ReleaseFailedList = RAB_FailedList;

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_ReleaseItem {
    pub r_ab_id: RAB_ID,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<RAB_ReleaseItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_ReleaseList(Vec<RAB_ReleaseList_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_ReleaseRequest {
    pub protocol_i_es: RAB_ReleaseRequestprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RAB_ReleaseRequestprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct RAB_ReleasedItem {
    pub r_ab_id: RAB_ID,
    #[asn(optional_idx = 0)]
    pub dl_data_volumes: Option<DataVolumeList>,
    #[asn(optional_idx = 1)]
    pub d_l_gtp_pdu_sequence_number: Option<DL_GTP_PDU_SequenceNumber>,
    #[asn(optional_idx = 2)]
    pub u_l_gtp_pdu_sequence_number: Option<UL_GTP_PDU_SequenceNumber>,
    #[asn(optional_idx = 3)]
    pub i_e_extensions: Option<RAB_ReleasedItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct RAB_ReleasedItem_IuRelComp {
    pub r_ab_id: RAB_ID,
    #[asn(optional_idx = 0)]
    pub d_l_gtp_pdu_sequence_number: Option<DL_GTP_PDU_SequenceNumber>,
    #[asn(optional_idx = 1)]
    pub u_l_gtp_pdu_sequence_number: Option<UL_GTP_PDU_SequenceNumber>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<RAB_ReleasedItem_IuRelCompiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_ReleasedList(Vec<RAB_ReleasedList_Item>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_ReleasedList_IuRelComp(Vec<RAB_ReleasedList_IuRelComp_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_RelocationReleaseItem {
    pub r_ab_id: RAB_ID,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<RAB_RelocationReleaseItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_RelocationReleaseList(Vec<RAB_RelocationReleaseList_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 7)]
pub struct RAB_SetupItem_EnhRelocInfoReq {
    pub r_ab_id: RAB_ID,
    pub c_n_domain_indicator: CN_DomainIndicator,
    pub r_ab_parameters: RAB_Parameters,
    #[asn(optional_idx = 0)]
    pub data_volume_reporting_indication: Option<DataVolumeReportingIndication>,
    #[asn(optional_idx = 1)]
    pub p_dp_type_information: Option<PDP_TypeInformation>,
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
    pub i_e_extensions: Option<RAB_SetupItem_EnhRelocInfoReqiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct RAB_SetupItem_EnhRelocInfoRes {
    pub c_n_domain_indicator: CN_DomainIndicator,
    pub r_ab_id: RAB_ID,
    #[asn(optional_idx = 0)]
    pub data_forwarding_information: Option<TNLInformationEnhRelInfoRes>,
    #[asn(optional_idx = 1)]
    pub ass_rab_parameters: Option<Ass_RAB_Parameters>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<RAB_SetupItem_EnhRelocInfoResiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct RAB_SetupItem_EnhancedRelocCompleteReq {
    pub r_ab_id: RAB_ID,
    #[asn(optional_idx = 0)]
    pub transport_layer_address_req1: Option<TransportLayerAddress>,
    #[asn(optional_idx = 1)]
    pub iu_transport_association_req1: Option<IuTransportAssociation>,
    #[asn(optional_idx = 2)]
    pub ass_rab_parameters: Option<Ass_RAB_Parameters>,
    #[asn(optional_idx = 3)]
    pub i_e_extensions: Option<RAB_SetupItem_EnhancedRelocCompleteReqiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct RAB_SetupItem_EnhancedRelocCompleteRes {
    pub r_ab_id: RAB_ID,
    #[asn(optional_idx = 0)]
    pub r_ab_parameters: Option<RAB_Parameters>,
    pub user_plane_information: UserPlaneInformation,
    #[asn(optional_idx = 1)]
    pub transport_layer_address_res1: Option<TransportLayerAddress>,
    #[asn(optional_idx = 2)]
    pub iu_transport_association_res1: Option<IuTransportAssociation>,
    #[asn(optional_idx = 3)]
    pub rab2be_released_list: Option<RAB_ToBeReleasedList_EnhancedRelocCompleteRes>,
    #[asn(optional_idx = 4)]
    pub i_e_extensions: Option<RAB_SetupItem_EnhancedRelocCompleteResiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct RAB_SetupItem_RelocReq {
    pub r_ab_id: RAB_ID,
    #[asn(optional_idx = 0)]
    pub n_as_synchronisation_indicator: Option<NAS_SynchronisationIndicator>,
    pub r_ab_parameters: RAB_Parameters,
    #[asn(optional_idx = 1)]
    pub data_volume_reporting_indication: Option<DataVolumeReportingIndication>,
    #[asn(optional_idx = 2)]
    pub p_dp_type_information: Option<PDP_TypeInformation>,
    pub user_plane_information: UserPlaneInformation,
    pub transport_layer_address: TransportLayerAddress,
    pub iu_transport_association: IuTransportAssociation,
    #[asn(optional_idx = 3)]
    pub service_handover: Option<Service_Handover>,
    #[asn(optional_idx = 4)]
    pub i_e_extensions: Option<RAB_SetupItem_RelocReqiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct RAB_SetupItem_RelocReqAck {
    pub r_ab_id: RAB_ID,
    #[asn(optional_idx = 0)]
    pub transport_layer_address: Option<TransportLayerAddress>,
    #[asn(optional_idx = 1)]
    pub iu_transport_association: Option<IuTransportAssociation>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<RAB_SetupItem_RelocReqAckiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_SetupList_EnhRelocInfoReq(Vec<RAB_SetupList_EnhRelocInfoReq_Item>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_SetupList_EnhRelocInfoRes(Vec<RAB_SetupList_EnhRelocInfoRes_Item>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_SetupList_EnhancedRelocCompleteReq(Vec<RAB_SetupList_EnhancedRelocCompleteReq_Item>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_SetupList_EnhancedRelocCompleteRes(Vec<RAB_SetupList_EnhancedRelocCompleteRes_Item>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_SetupList_RelocReq(Vec<RAB_SetupList_RelocReq_Item>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_SetupList_RelocReqAck(Vec<RAB_SetupList_RelocReqAck_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct RAB_SetupOrModifiedItem {
    pub r_ab_id: RAB_ID,
    #[asn(optional_idx = 0)]
    pub transport_layer_address: Option<TransportLayerAddress>,
    #[asn(optional_idx = 1)]
    pub iu_transport_association: Option<IuTransportAssociation>,
    #[asn(optional_idx = 2)]
    pub dl_data_volumes: Option<DataVolumeList>,
    #[asn(optional_idx = 3)]
    pub i_e_extensions: Option<RAB_SetupOrModifiedItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_SetupOrModifiedList(Vec<RAB_SetupOrModifiedList_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 6)]
pub struct RAB_SetupOrModifyItemFirst {
    pub r_ab_id: RAB_ID,
    #[asn(optional_idx = 0)]
    pub n_as_synchronisation_indicator: Option<NAS_SynchronisationIndicator>,
    #[asn(optional_idx = 1)]
    pub r_ab_parameters: Option<RAB_Parameters>,
    #[asn(optional_idx = 2)]
    pub user_plane_information: Option<UserPlaneInformation>,
    #[asn(optional_idx = 3)]
    pub transport_layer_information: Option<TransportLayerInformation>,
    #[asn(optional_idx = 4)]
    pub service_handover: Option<Service_Handover>,
    #[asn(optional_idx = 5)]
    pub i_e_extensions: Option<RAB_SetupOrModifyItemFirstiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 7)]
pub struct RAB_SetupOrModifyItemSecond {
    #[asn(optional_idx = 0)]
    pub p_dp_type_information: Option<PDP_TypeInformation>,
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
    pub i_e_extensions: Option<RAB_SetupOrModifyItemSecondiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_SetupOrModifyList(Vec<RAB_SetupOrModifyList_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "16000000")]
pub struct RAB_SubflowCombinationBitRate(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_ToBeReleasedItem_EnhancedRelocCompleteRes {
    pub r_ab_id: RAB_ID,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<RAB_ToBeReleasedItem_EnhancedRelocCompleteResiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_ToBeReleasedList_EnhancedRelocCompleteRes(
    Vec<RAB_ToBeReleasedList_EnhancedRelocCompleteRes_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_TrCH_Mapping(Vec<RAB_TrCH_MappingItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_TrCH_MappingItem {
    pub r_ab_id: RAB_ID,
    pub tr_ch_id_list: TrCH_ID_List,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<RAB_TrCH_MappingItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct RABDataVolumeReport(Vec<RABDataVolumeReport_Item>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RABParametersList(Vec<RABParametersList_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RABased {
    pub rai_list: RAI_List,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<RABasediE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RABs_ContextFailedtoTransferItem {
    pub r_ab_id: RAB_ID,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<RABs_ContextFailedtoTransferItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RABs_failed_to_reportItem {
    pub r_ab_id: RAB_ID,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<RABs_failed_to_reportItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "1", sz_ub = "1")]
pub struct RAC(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAI {
    pub l_ai: LAI,
    pub r_ac: RAC,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<RAIiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct RAI_List(Vec<RAI>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum RAListofIdleModeUEs {
    #[asn(key = 0, extended = false)]
    notEmptyRAListofIdleModeUEs(NotEmptyRAListofIdleModeUEs),
    #[asn(key = 1, extended = false)]
    emptyFullRAListofIdleModeUEs(ENUMERATED_50),
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65536"
)]
pub struct RAListwithNoIdleModeUEsAnyMore(Vec<RAC>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RANAP_EnhancedRelocationInformationRequest {
    pub protocol_i_es: RANAP_EnhancedRelocationInformationRequestprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RANAP_EnhancedRelocationInformationRequestprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RANAP_EnhancedRelocationInformationResponse {
    pub protocol_i_es: RANAP_EnhancedRelocationInformationResponseprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RANAP_EnhancedRelocationInformationResponseprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = true)]
pub enum RANAP_PDU {
    #[asn(key = 0, extended = false)]
    initiatingMessage(InitiatingMessage),
    #[asn(key = 1, extended = false)]
    successfulOutcome(SuccessfulOutcome),
    #[asn(key = 2, extended = false)]
    unsuccessfulOutcome(UnsuccessfulOutcome),
    #[asn(key = 3, extended = false)]
    outcome(Outcome),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RANAP_RelocationInformation {
    pub protocol_i_es: RANAP_RelocationInformationprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RANAP_RelocationInformationprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct RAT_Type(u8);
impl RAT_Type {
    const UTRAN: u8 = 0u8;
    const GERAN: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65536"
)]
pub struct RAofIdleModeUEs(Vec<RAC>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct RIM_Transfer {
    pub r_im_information: RIMInformation,
    #[asn(optional_idx = 0)]
    pub r_im_routing_address: Option<RIMRoutingAddress>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<RIM_TransferiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct RIMInformation(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum RIMRoutingAddress {
    #[asn(key = 0, extended = false)]
    targetRNC_ID(TargetRNC_ID),
    #[asn(key = 1, extended = false)]
    gERAN_Cell_ID(GERAN_Cell_ID),
    #[asn(key = 0, extended = true)]
    targeteNB_ID(TargetENB_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct RNC_ID(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct RNCTraceInformation {
    pub trace_reference: TraceReference,
    pub trace_activation_indicator: ENUMERATED_51,
    #[asn(optional_idx = 0)]
    pub equipments_to_be_traced: Option<EquipmentsToBeTraced>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<RNCTraceInformationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
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
    pub i_e_extensions: Option<RNSAPRelocationParametersiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct RRC_Container(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "-30", ub = "46", extensible = true)]
pub struct RSRQ_Extension(i8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RSRQ_Type {
    pub all_symbols: BOOLEAN_52,
    pub wide_band: BOOLEAN_53,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct RSRVCC_HO_Indication(u8);
impl RSRVCC_HO_Indication {
    const PS_ONLY: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RSRVCC_Information {
    pub nonce: BIT_STRING_54,
    pub i_ms_information: OCTET_STRING_55,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<RSRVCC_InformationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct RSRVCC_Operation_Possible(u8);
impl RSRVCC_Operation_Possible {
    const RSRVCC_POSSIBLE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct RTLoadValue(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct RateControlAllowed(u8);
impl RateControlAllowed {
    const NOT_ALLOWED: u8 = 0u8;
    const ALLOWED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "NULL")]
pub struct RedirectAttemptFlag;

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct RedirectionCompleted(u8);
impl RedirectionCompleted {
    const REDIRECTION_COMPLETED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RedirectionIndication(Vec<RedirectionIndication_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "5")]
pub struct RejectCauseValue(u8);
impl RejectCauseValue {
    const P_LMN_NOT_ALLOWED: u8 = 0u8;
    const LOCATION_AREA_NOT_ALLOWED: u8 = 1u8;
    const ROAMING_NOT_ALLOWED_IN_THIS_LOCATION_AREA: u8 = 2u8;
    const NO_SUITABLE_CELL_IN_LOCATION_AREA: u8 = 3u8;
    const G_PRS_SERVICES_NOT_ALLOWED_IN_THIS_PLMN: u8 = 4u8;
    const C_S_PS_COORDINATION_REQUIRED: u8 = 5u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RelocationCancel {
    pub protocol_i_es: RelocationCancelprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RelocationCancelprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RelocationCancelAcknowledge {
    pub protocol_i_es: RelocationCancelAcknowledgeprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RelocationCancelAcknowledgeprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RelocationCommand {
    pub protocol_i_es: RelocationCommandprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RelocationCommandprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RelocationComplete {
    pub protocol_i_es: RelocationCompleteprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RelocationCompleteprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RelocationDetect {
    pub protocol_i_es: RelocationDetectprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RelocationDetectprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RelocationFailure {
    pub protocol_i_es: RelocationFailureprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RelocationFailureprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RelocationPreparationFailure {
    pub protocol_i_es: RelocationPreparationFailureprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RelocationPreparationFailureprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RelocationRequest {
    pub protocol_i_es: RelocationRequestprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RelocationRequestprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RelocationRequestAcknowledge {
    pub protocol_i_es: RelocationRequestAcknowledgeprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RelocationRequestAcknowledgeprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RelocationRequired {
    pub protocol_i_es: RelocationRequiredprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RelocationRequiredprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct RelocationRequirement(u8);
impl RelocationRequirement {
    const LOSSLESS: u8 = 0u8;
    const NONE: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct RelocationType(u8);
impl RelocationType {
    const UE_NOT_INVOLVED: u8 = 0u8;
    const UE_INVOLVED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct RepetitionNumber0(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "256")]
pub struct RepetitionNumber1(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "7")]
pub struct ReportAmount(u8);
impl ReportAmount {
    const N1: u8 = 0u8;
    const N2: u8 = 1u8;
    const N4: u8 = 2u8;
    const N8: u8 = 3u8;
    const N16: u8 = 4u8;
    const N32: u8 = 5u8;
    const N64: u8 = 6u8;
    const INFINITY: u8 = 7u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ReportArea(u8);
impl ReportArea {
    const SERVICE_AREA: u8 = 0u8;
    const GEOGRAPHICAL_AREA: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ReportChangeOfSAI(u8);
impl ReportChangeOfSAI {
    const REQUESTED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "12")]
pub struct ReportInterval(u8);
impl ReportInterval {
    const MS250: u8 = 0u8;
    const MS500: u8 = 1u8;
    const MS1000: u8 = 2u8;
    const MS2000: u8 = 3u8;
    const MS3000: u8 = 4u8;
    const MS4000: u8 = 5u8;
    const MS6000: u8 = 6u8;
    const MS12000: u8 = 7u8;
    const MS16000: u8 = 8u8;
    const MS20000: u8 = 9u8;
    const MS24000: u8 = 10u8;
    const MS32000: u8 = 11u8;
    const MS64000: u8 = 12u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RequestType {
    pub event: Event,
    pub report_area: ReportArea,
    #[asn(optional_idx = 0)]
    pub accuracy_code: Option<INTEGER_56>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct Requested_RAB_Parameter_ExtendedGuaranteedBitrateList(Vec<ExtendedGuaranteedBitrate>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct Requested_RAB_Parameter_ExtendedMaxBitrateList(Vec<ExtendedMaxBitrate>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct Requested_RAB_Parameter_GuaranteedBitrateList(Vec<GuaranteedBitrate>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct Requested_RAB_Parameter_MaxBitrateList(Vec<MaxBitrate>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct Requested_RAB_Parameter_Values {
    #[asn(optional_idx = 0)]
    pub requested_max_bitrates: Option<Requested_RAB_Parameter_MaxBitrateList>,
    #[asn(optional_idx = 1)]
    pub requested_guaranteed_bitrates: Option<Requested_RAB_Parameter_GuaranteedBitrateList>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<Requested_RAB_Parameter_ValuesiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "OCTET-STRING",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "201"
)]
pub struct RequestedGANSSAssistanceData(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "OCTET-STRING",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "38"
)]
pub struct RequestedGPSAssistanceData(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct RequestedLocationRelatedDataType(u8);
impl RequestedLocationRelatedDataType {
    const DECIPHERING_KEYS_UE_BASED_OTDOA: u8 = 0u8;
    const DECIPHERING_KEYS_ASSISTED_GPS: u8 = 1u8;
    const DEDICATED_ASSISTANCE_DATA_UE_BASED_OTDOA: u8 = 2u8;
    const DEDICATED_ASSISTANCE_DATA_ASSISTED_GPS: u8 = 3u8;
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "512"
)]
pub struct RequestedMBMSIPMulticastAddressandAPNRequest(Vec<MBMSIPMulticastAddressandAPNlist>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "128"
)]
pub struct RequestedMulticastServiceList(Vec<TMGI>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RerouteNASRequest {
    pub protocol_i_es: RerouteNASRequestprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RerouteNASRequestprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Reset {
    pub protocol_i_es: ResetprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<ResetprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ResetAcknowledge {
    pub protocol_i_es: ResetAcknowledgeprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<ResetAcknowledgeprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ResetResource {
    pub protocol_i_es: ResetResourceprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<ResetResourceprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ResetResourceAckItem {
    pub iu_sig_con_id: IuSignallingConnectionIdentifier,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<ResetResourceAckItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "250"
)]
pub struct ResetResourceAckList(Vec<ResetResourceAckList_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ResetResourceAcknowledge {
    pub protocol_i_es: ResetResourceAcknowledgeprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<ResetResourceAcknowledgeprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ResetResourceItem {
    pub iu_sig_con_id: IuSignallingConnectionIdentifier,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<ResetResourceItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "250"
)]
pub struct ResetResourceList(Vec<ResetResourceList_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct ResidualBitErrorRatio {
    pub mantissa: INTEGER_57,
    pub exponent: INTEGER_58,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<ResidualBitErrorRatioiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ResponseTime(u8);
impl ResponseTime {
    const LOWDELAY: u8 = 0u8;
    const DELAYTOLERANT: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct SAC(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct SAI {
    pub p_lm_nidentity: PLMNidentity,
    pub l_ac: LAC,
    pub s_ac: SAC,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<SAIiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct SAPI(u8);
impl SAPI {
    const SAPI_0: u8 = 0u8;
    const SAPI_3: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct SDU_ErrorRatio {
    pub mantissa: INTEGER_59,
    pub exponent: INTEGER_60,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<SDU_ErrorRatioiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct SDU_FormatInformationParameters(Vec<SDU_FormatInformationParameters_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "7")]
pub struct SDU_Parameters(Vec<SDU_Parameters_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct SGSN_Group_ID(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum SGSN_Group_Identity {
    #[asn(key = 0, extended = false)]
    null_NRI(Null_NRI),
    #[asn(key = 1, extended = false)]
    sGSN_Group_ID(SGSN_Group_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SNA_Access_Information {
    pub authorised_plm_ns: AuthorisedPLMNs,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<SNA_Access_InformationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct SNAC(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "32")]
pub struct SRB_ID(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct SRB_TrCH_Mapping(Vec<SRB_TrCH_MappingItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SRB_TrCH_MappingItem {
    pub s_rb_id: SRB_ID,
    pub tr_ch_id: TrCH_ID,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<SRB_TrCH_MappingItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SRNS_ContextRequest {
    pub protocol_i_es: SRNS_ContextRequestprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<SRNS_ContextRequestprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SRNS_ContextResponse {
    pub protocol_i_es: SRNS_ContextResponseprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<SRNS_ContextResponseprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SRNS_DataForwardCommand {
    pub protocol_i_es: SRNS_DataForwardCommandprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<SRNS_DataForwardCommandprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SRVCC_CSKeysRequest {
    pub protocol_i_es: SRVCC_CSKeysRequestprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<SRVCC_CSKeysRequestprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SRVCC_CSKeysResponse {
    pub protocol_i_es: SRVCC_CSKeysResponseprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<SRVCC_CSKeysResponseprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct SRVCC_HO_Indication(u8);
impl SRVCC_HO_Indication {
    const PS_AND_CS: u8 = 0u8;
    const CS_ONLY: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SRVCC_Information {
    pub nonce: BIT_STRING_61,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<SRVCC_InformationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct SRVCC_Operation_Possible(u8);
impl SRVCC_Operation_Possible {
    const SRVCC_POSSIBLE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct SRVCCSource(u8);
impl SRVCCSource {
    const V5_G: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SecurityModeCommand {
    pub protocol_i_es: SecurityModeCommandprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<SecurityModeCommandprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SecurityModeComplete {
    pub protocol_i_es: SecurityModeCompleteprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<SecurityModeCompleteprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SecurityModeReject {
    pub protocol_i_es: SecurityModeRejectprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<SecurityModeRejectprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct Service_Handover(u8);
impl Service_Handover {
    const HANDOVER_TO_GSM_SHOULD_BE_PERFORMED: u8 = 0u8;
    const HANDOVER_TO_GSM_SHOULD_NOT_BE_PERFORMED: u8 = 1u8;
    const HANDOVER_TO_GSM_SHALL_NOT_BE_PERFORMED: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ServiceType(u8);
impl ServiceType {
    const Q_MC_FOR_STREAMING_SERVICE: u8 = 0u8;
    const Q_MC_FOR_MSTI_SERVICE: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Session_Re_establishment_Indicator(u8);
impl Session_Re_establishment_Indicator {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1048575")]
pub struct SessionUpdateID(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Shared_Network_Information {
    pub p_lm_ns_in_shared_network: PLMNs_in_shared_network,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<Shared_Network_InformationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct SignallingIndication(u8);
impl SignallingIndication {
    const SIGNALLING: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct Source_ToTarget_TransparentContainer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct SourceBSS_ToTargetBSS_TransparentContainer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum SourceCellID {
    #[asn(key = 0, extended = false)]
    sourceUTRANCellID(SourceUTRANCellID),
    #[asn(key = 1, extended = false)]
    sourceGERANCellID(CGI),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum SourceID {
    #[asn(key = 0, extended = false)]
    sourceRNC_ID(SourceRNC_ID),
    #[asn(key = 1, extended = false)]
    sAI(SAI),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct SourceRNC_ID {
    pub p_lm_nidentity: PLMNidentity,
    pub r_nc_id: RNC_ID,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<SourceRNC_IDiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 10)]
pub struct SourceRNC_ToTargetRNC_TransparentContainer {
    pub r_rc_container: RRC_Container,
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
    pub r_ab_tr_ch_mapping: Option<RAB_TrCH_Mapping>,
    #[asn(optional_idx = 9)]
    pub i_e_extensions: Option<SourceRNC_ToTargetRNC_TransparentContaineriE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct SourceStatisticsDescriptor(u8);
impl SourceStatisticsDescriptor {
    const SPEECH: u8 = 0u8;
    const UNKNOWN: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct SourceUTRANCellID {
    pub p_lm_nidentity: PLMNidentity,
    pub u_tra_ncell_id: TargetCellId,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<SourceUTRANCellIDiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct SourceeNodeB_ToTargeteNodeB_TransparentContainer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct SubflowSDU_Size(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "256")]
pub struct SubscriberProfileIDforRFP(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SuccessfulOutcome {
    #[asn(key_field = true)]
    pub procedure_code: ProcedureCode,
    pub criticality: Criticality,
    pub value: SuccessfulOutcomevalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "1000000000", extensible = true)]
pub struct SupportedBitrate(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct SupportedRAB_ParameterBitrateList(Vec<SupportedBitrate>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct TAC(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct TAI {
    pub p_lm_nidentity: PLMNidentity,
    pub t_ac: TAC,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<TAIiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct TBCD_STRING(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct TMGI {
    pub p_lm_nidentity: PLMNidentity,
    pub service_id: OCTET_STRING_62,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<TMGIiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct TMSI(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TNLInformationEnhRelInfoReq {
    pub transport_layer_address: TransportLayerAddress,
    pub iu_transport_association: IuTransportAssociation,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<TNLInformationEnhRelInfoReqiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TNLInformationEnhRelInfoRes {
    pub dl_forwarding_transport_layer_address: TransportLayerAddress,
    pub dl_forwarding_transport_association: IuTransportAssociation,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<TNLInformationEnhRelInfoResiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct Target_ToSource_TransparentContainer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct TargetBSS_ToSourceBSS_TransparentContainer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "268435455")]
pub struct TargetCellId(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TargetENB_ID {
    pub p_lm_nidentity: PLMNidentity,
    pub e_nb_id: ENB_ID,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<TargetENB_IDiE_Extensions>,
    pub selected_tai: TAI,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum TargetID {
    #[asn(key = 0, extended = false)]
    targetRNC_ID(TargetRNC_ID),
    #[asn(key = 1, extended = false)]
    cGI(CGI),
    #[asn(key = 0, extended = true)]
    targeteNB_ID(TargetENB_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct TargetRNC_ID {
    pub l_ai: LAI,
    #[asn(optional_idx = 0)]
    pub r_ac: Option<RAC>,
    pub r_nc_id: RNC_ID,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<TargetRNC_IDiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct TargetRNC_ToSourceRNC_TransparentContainer {
    pub r_rc_container: RRC_Container,
    #[asn(optional_idx = 0)]
    pub d_rnti: Option<D_RNTI>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<TargetRNC_ToSourceRNC_TransparentContaineriE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct TargeteNodeB_ToSourceeNodeB_TransparentContainer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum TemporaryUE_ID {
    #[asn(key = 0, extended = false)]
    tMSI(TMSI),
    #[asn(key = 1, extended = false)]
    p_TMSI(P_TMSI),
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct Time_UE_StayedInCell(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "40950")]
pub struct Time_UE_StayedInCell_EnhancedGranularity(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "1", sz_ub = "1")]
pub struct TimeToMBMSDataTransfer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "1", sz_ub = "1")]
pub struct TimingDifferenceULDL(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct TrCH_ID {
    #[asn(optional_idx = 0)]
    pub d_ch_id: Option<DCH_ID>,
    #[asn(optional_idx = 1)]
    pub d_sch_id: Option<DSCH_ID>,
    #[asn(optional_idx = 2)]
    pub u_sch_id: Option<USCH_ID>,
    #[asn(optional_idx = 3)]
    pub i_e_extensions: Option<TrCH_IDiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "7")]
pub struct TrCH_ID_List(Vec<TrCH_ID>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct TraceDepth(u8);
impl TraceDepth {
    const MINIMUM: u8 = 0u8;
    const MEDIUM: u8 = 1u8;
    const MAXIMUM: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct TraceInformation {
    pub trace_reference: TraceReference,
    pub ue_identity: UE_ID,
    #[asn(optional_idx = 0)]
    pub trace_propagation_parameters: Option<TracePropagationParameters>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<TraceInformationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct TracePropagationParameters {
    pub trace_recording_session_reference: TraceRecordingSessionReference,
    pub trace_depth: TraceDepth,
    #[asn(optional_idx = 0)]
    pub list_of_interfaces_to_trace: Option<ListOfInterfacesToTrace>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<TracePropagationParametersiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TraceRecordingSessionInformation {
    pub trace_reference: TraceReference,
    pub trace_recording_session_reference: TraceRecordingSessionReference,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<TraceRecordingSessionInformationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct TraceRecordingSessionReference(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "2", sz_ub = "3")]
pub struct TraceReference(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "1", sz_ub = "1")]
pub struct TraceType(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct TrafficClass(u8);
impl TrafficClass {
    const CONVERSATIONAL: u8 = 0u8;
    const STREAMING: u8 = 1u8;
    const INTERACTIVE: u8 = 2u8;
    const BACKGROUND: u8 = 3u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct TrafficHandlingPriority(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct TransferDelay(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "1", sz_ub = "160")]
pub struct TransportLayerAddress(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TransportLayerInformation {
    pub transport_layer_address: TransportLayerAddress,
    pub iu_transport_association: IuTransportAssociation,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<TransportLayerInformationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "OCTET-STRING",
    sz_extensible = false,
    sz_lb = "3",
    sz_ub = "22"
)]
pub struct TriggerID(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "3")]
pub struct TriggeringMessage(u8);
impl TriggeringMessage {
    const INITIATING_MESSAGE: u8 = 0u8;
    const SUCCESSFUL_OUTCOME: u8 = 1u8;
    const UNSUCCESSFULL_OUTCOME: u8 = 2u8;
    const OUTCOME: u8 = 3u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct TunnelInformation {
    pub transport_layer_address: TransportLayerAddress,
    #[asn(optional_idx = 0)]
    pub u_dp_port_number: Option<Port_Number>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<TunnelInformationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct TypeOfError(u8);
impl TypeOfError {
    const NOT_UNDERSTOOD: u8 = 0u8;
    const MISSING: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct UE_AggregateMaximumBitRate {
    #[asn(optional_idx = 0)]
    pub u_e_aggregate_maximum_bit_rate_downlink: Option<UE_AggregateMaximumBitRateDownlink>,
    #[asn(optional_idx = 1)]
    pub u_e_aggregate_maximum_bit_rate_uplink: Option<UE_AggregateMaximumBitRateUplink>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "1000000000")]
pub struct UE_AggregateMaximumBitRateDownlink(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "1000000000")]
pub struct UE_AggregateMaximumBitRateUplink(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct UE_Application_Layer_Measurement_Capability(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UE_Application_Layer_Measurement_Configuration {
    pub application_layer_container_for_measurement_configuration: OCTET_STRING_63,
    pub area_scope_for_ue_application_layer_measurement_configuration:
        AreaScopeForUEApplicationLayerMeasurementConfiguration,
    pub service_type: ServiceType,
}

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct UE_History_Information(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum UE_ID {
    #[asn(key = 0, extended = false)]
    imsi(IMSI),
    #[asn(key = 1, extended = false)]
    imei(IMEI),
    #[asn(key = 0, extended = true)]
    imeisv(IMEISV),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UE_IsNotServed {
    pub permanent_nas_ue_id: PermanentNAS_UE_ID,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<UE_IsNotServediE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UE_IsServed {
    pub permanent_nas_ue_id: PermanentNAS_UE_ID,
    pub p_lm_nidentity: PLMNidentity,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<UE_IsServediE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct UE_Usage_Type(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum UERegistrationQueryResult {
    #[asn(key = 0, extended = false)]
    uE_IsServed(UE_IsServed),
    #[asn(key = 1, extended = false)]
    uE_IsNotServed(UE_IsNotServed),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct UESBI_Iu {
    #[asn(optional_idx = 0)]
    pub u_esbi_iu_a: Option<UESBI_IuA>,
    #[asn(optional_idx = 1)]
    pub u_esbi_iu_b: Option<UESBI_IuB>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<UESBI_IuiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "128")]
pub struct UESBI_IuA(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "128")]
pub struct UESBI_IuB(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UESpecificInformationIndication {
    pub protocol_i_es: UESpecificInformationIndicationprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<UESpecificInformationIndicationprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct UL_GTP_PDU_SequenceNumber(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct UL_N_PDU_SequenceNumber(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct UP_ModeVersions(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UPInformation {
    pub frame_seq_no_ul: FrameSequenceNumber,
    pub frame_seq_no_dl: FrameSequenceNumber,
    pub pdu14_frame_seq_no_ul: PDUType14FrameSequenceNumber,
    pub pdu14_frame_seq_no_dl: PDUType14FrameSequenceNumber,
    pub data_pdu_type: DataPDUType,
    pub upinitialisation_frame: UPInitialisationFrame,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<UPInformationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct UPInitialisationFrame(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct USCH_ID(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct UTRAN_CellID {
    pub p_lm_nidentity: PLMNidentity,
    pub cell_id: TargetCellId,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<UTRAN_CellIDiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct UeApplicationLayerMeasurementSupportIndication(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UeRadioCapabilityMatchRequest {
    pub protocol_i_es: UeRadioCapabilityMatchRequestprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<UeRadioCapabilityMatchRequestprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UeRadioCapabilityMatchResponse {
    pub protocol_i_es: UeRadioCapabilityMatchResponseprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<UeRadioCapabilityMatchResponseprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UeRegistrationQueryRequest {
    pub protocol_i_es: UeRegistrationQueryRequestprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<UeRegistrationQueryRequestprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UeRegistrationQueryResponse {
    pub protocol_i_es: UeRegistrationQueryResponseprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<UeRegistrationQueryResponseprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "128"
)]
pub struct UnsuccessfulLinking_IEs(Vec<UnsuccessfulLinking_IEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UnsuccessfulOutcome {
    #[asn(key_field = true)]
    pub procedure_code: ProcedureCode,
    pub criticality: Criticality,
    pub value: UnsuccessfulOutcomevalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4294967295")]
pub struct UnsuccessfullyTransmittedDataVolume(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UplinkInformationExchangeFailure {
    pub protocol_i_es: UplinkInformationExchangeFailureprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<UplinkInformationExchangeFailureprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UplinkInformationExchangeRequest {
    pub protocol_i_es: UplinkInformationExchangeRequestprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<UplinkInformationExchangeRequestprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UplinkInformationExchangeResponse {
    pub protocol_i_es: UplinkInformationExchangeResponseprotocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<UplinkInformationExchangeResponseprotocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UserPlaneInformation {
    pub user_plane_mode: UserPlaneMode,
    pub u_p_mode_versions: UP_ModeVersions,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<UserPlaneInformationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct UserPlaneMode(u8);
impl UserPlaneMode {
    const TRANSPARENT_MODE: u8 = 0u8;
    const SUPPORT_MODE_FOR_PREDEFINED_SDU_SIZES: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = true)]
pub enum VelocityEstimate {
    #[asn(key = 0, extended = false)]
    horizontalVelocity(HorizontalVelocity),
    #[asn(key = 1, extended = false)]
    horizontalWithVerticalVelocity(HorizontalWithVerticalVelocity),
    #[asn(key = 2, extended = false)]
    horizontalVelocityWithUncertainty(HorizontalVelocityWithUncertainty),
    #[asn(key = 3, extended = false)]
    horizontalWithVeritcalVelocityAndUncertainty(HorizontalWithVerticalVelocityAndUncertainty),
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct VerticalAccuracyCode(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct VerticalSpeedDirection(u8);
impl VerticalSpeedDirection {
    const UPWARD: u8 = 0u8;
    const DOWNWARD: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct VerticalVelocity {
    pub veritcal_speed: INTEGER_64,
    pub veritcal_speed_direction: VerticalSpeedDirection,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct VoiceSupportMatchIndicator(u8);
impl VoiceSupportMatchIndicator {
    const SUPPORTED: u8 = 0u8;
    const NOT_SUPPORTED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "10", sz_ub = "10")]
pub struct BIT_STRING_2(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "NULL")]
pub struct NULL_3;

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Additional_CSPS_coordination_informationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Additional_CSPS_coordination_informationiE_Extensions(
    Vec<Additional_CSPS_coordination_informationiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AllocationOrRetentionPriorityiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AllocationOrRetentionPriorityiE_Extensions(
    Vec<AllocationOrRetentionPriorityiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Alt_RAB_Parameter_SupportedGuaranteedBitrateInfiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Alt_RAB_Parameter_SupportedGuaranteedBitrateInfiE_Extensions(
    Vec<Alt_RAB_Parameter_SupportedGuaranteedBitrateInfiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Alt_RAB_Parameter_SupportedMaxBitrateInfiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Alt_RAB_Parameter_SupportedMaxBitrateInfiE_Extensions(
    Vec<Alt_RAB_Parameter_SupportedMaxBitrateInfiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum Alt_RAB_ParametersiE_Extensions_ItemextensionValue {
    #[asn(key = 172)]
    id_Alt_RAB_Parameter_ExtendedGuaranteedBitrateInf(
        Alt_RAB_Parameter_ExtendedGuaranteedBitrateInf,
    ),
    #[asn(key = 173)]
    id_Alt_RAB_Parameter_ExtendedMaxBitrateInf(Alt_RAB_Parameter_ExtendedMaxBitrateInf),
    #[asn(key = 214)]
    id_Alt_RAB_Parameter_SupportedGuaranteedBitrateInf(
        Alt_RAB_Parameter_SupportedGuaranteedBitrateInf,
    ),
    #[asn(key = 215)]
    id_Alt_RAB_Parameter_SupportedMaxBitrateInf(Alt_RAB_Parameter_SupportedMaxBitrateInf),
    #[asn(key = 158)]
    id_AlternativeRABConfiguration(RAB_Parameters),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Alt_RAB_ParametersiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: Alt_RAB_ParametersiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Alt_RAB_ParametersiE_Extensions(Vec<Alt_RAB_ParametersiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum Ass_RAB_ParametersiE_Extensions_ItemextensionValue {
    #[asn(key = 174)]
    id_Ass_RAB_Parameter_ExtendedGuaranteedBitrateList(
        Ass_RAB_Parameter_ExtendedGuaranteedBitrateList,
    ),
    #[asn(key = 175)]
    id_Ass_RAB_Parameter_ExtendedMaxBitrateList(Ass_RAB_Parameter_ExtendedMaxBitrateList),
    #[asn(key = 216)]
    id_Ass_RAB_Parameter_SupportedGuaranteedBitrateList(SupportedRAB_ParameterBitrateList),
    #[asn(key = 217)]
    id_Ass_RAB_Parameter_SupportedMaxBitrateList(SupportedRAB_ParameterBitrateList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Ass_RAB_ParametersiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: Ass_RAB_ParametersiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Ass_RAB_ParametersiE_Extensions(Vec<Ass_RAB_ParametersiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AuthorisedPLMNs_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AuthorisedPLMNs_ItemiE_Extensions(Vec<AuthorisedPLMNs_ItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct AuthorisedPLMNs_Item {
    pub p_lm_nidentity: PLMNidentity,
    #[asn(optional_idx = 0)]
    pub authorised_sn_as_list: Option<AuthorisedSNAs>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<AuthorisedPLMNs_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "1")]
pub struct BIT_STRING_4(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "56", sz_ub = "56")]
pub struct BIT_STRING_5(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "56", sz_ub = "56")]
pub struct BIT_STRING_6(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum CGIiE_Extensions_ItemextensionValue {
    #[asn(key = 55)]
    id_RAC(RAC),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CGIiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: CGIiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CGIiE_Extensions(Vec<CGIiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum CN_DeactivateTraceprotocolIEs_Itemvalue {
    #[asn(key = 65)]
    id_TraceReference(TraceReference),
    #[asn(key = 68)]
    id_TriggerID(TriggerID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CN_DeactivateTraceprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: CN_DeactivateTraceprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct CN_DeactivateTraceprotocolIEs(Vec<CN_DeactivateTraceprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CN_DeactivateTraceprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CN_DeactivateTraceprotocolExtensions(Vec<CN_DeactivateTraceprotocolExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum CN_InvokeTraceprotocolIEs_Itemvalue {
    #[asn(key = 19)]
    id_OMC_ID(OMC_ID),
    #[asn(key = 65)]
    id_TraceReference(TraceReference),
    #[asn(key = 66)]
    id_TraceType(TraceType),
    #[asn(key = 68)]
    id_TriggerID(TriggerID),
    #[asn(key = 69)]
    id_UE_ID(UE_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CN_InvokeTraceprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: CN_InvokeTraceprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct CN_InvokeTraceprotocolIEs(Vec<CN_InvokeTraceprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum CN_InvokeTraceprotocolExtensions_ItemextensionValue {
    #[asn(key = 244)]
    id_MDT_Configuration(MDT_Configuration),
    #[asn(key = 251)]
    id_Trace_Collection_Entity_IP_Addess(TransportLayerAddress),
    #[asn(key = 125)]
    id_TracePropagationParameters(TracePropagationParameters),
    #[asn(key = 292)]
    id_UE_Application_Layer_Measurement_Configuration(
        UE_Application_Layer_Measurement_Configuration,
    ),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CN_InvokeTraceprotocolExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: CN_InvokeTraceprotocolExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CN_InvokeTraceprotocolExtensions(Vec<CN_InvokeTraceprotocolExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CNMBMSLinkingInformationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CNMBMSLinkingInformationiE_Extensions(Vec<CNMBMSLinkingInformationiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellBasediE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellBasediE_Extensions(Vec<CellBasediE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellLoadInformationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellLoadInformationiE_Extensions(Vec<CellLoadInformationiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellLoadInformationGroupiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellLoadInformationGroupiE_Extensions(Vec<CellLoadInformationGroupiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum CommonIDprotocolIEs_Itemvalue {
    #[asn(key = 23)]
    id_PermanentNAS_UE_ID(PermanentNAS_UE_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CommonIDprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: CommonIDprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct CommonIDprotocolIEs(Vec<CommonIDprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum CommonIDprotocolExtensions_ItemextensionValue {
    #[asn(key = 234)]
    id_CSG_Membership_Status(CSG_Membership_Status),
    #[asn(key = 277)]
    id_LastE_UTRANPLMNIdentity(PLMNidentity),
    #[asn(key = 249)]
    id_Management_Based_MDT_Allowed(Management_Based_MDT_Allowed),
    #[asn(key = 263)]
    id_Management_Based_MDT_PLMN_List(MDT_PLMN_List),
    #[asn(key = 289)]
    id_PowerSavingIndicator(PowerSavingIndicator),
    #[asn(key = 272)]
    id_RSRVCC_Operation_Possible(RSRVCC_Operation_Possible),
    #[asn(key = 105)]
    id_SNA_Access_Information(SNA_Access_Information),
    #[asn(key = 228)]
    id_SRVCC_Operation_Possible(SRVCC_Operation_Possible),
    #[asn(key = 127)]
    id_SelectedPLMN_ID(PLMNidentity),
    #[asn(key = 202)]
    id_SubscriberProfileIDforRFP(SubscriberProfileIDforRFP),
    #[asn(key = 118)]
    id_UESBI_Iu(UESBI_Iu),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CommonIDprotocolExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: CommonIDprotocolExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CommonIDprotocolExtensions(Vec<CommonIDprotocolExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CriticalityDiagnosticsiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CriticalityDiagnosticsiE_Extensions(Vec<CriticalityDiagnosticsiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum CriticalityDiagnostics_IE_List_ItemiE_Extensions_ItemextensionValue {
    #[asn(key = 88)]
    id_MessageStructure(MessageStructure),
    #[asn(key = 93)]
    id_TypeOfError(TypeOfError),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CriticalityDiagnostics_IE_List_ItemiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: CriticalityDiagnostics_IE_List_ItemiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CriticalityDiagnostics_IE_List_ItemiE_Extensions(
    Vec<CriticalityDiagnostics_IE_List_ItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct CriticalityDiagnostics_IE_List_Item {
    pub i_e_criticality: Criticality,
    pub i_e_id: ProtocolIE_ID,
    #[asn(optional_idx = 0)]
    pub repetition_number: Option<RepetitionNumber0>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<CriticalityDiagnostics_IE_List_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DataVolumeList_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DataVolumeList_ItemiE_Extensions(Vec<DataVolumeList_ItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct DataVolumeList_Item {
    pub dl_unsuccessfully_transmitted_data_volume: UnsuccessfullyTransmittedDataVolume,
    #[asn(optional_idx = 0)]
    pub data_volume_reference: Option<DataVolumeReference>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<DataVolumeList_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DataVolumeReportprotocolIEs_Itemvalue {
    #[asn(key = 9)]
    id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 31)]
    id_RAB_DataVolumeReportList(RAB_DataVolumeReportList),
    #[asn(key = 72)]
    id_RAB_FailedtoReportList(RAB_FailedtoReportList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DataVolumeReportprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: DataVolumeReportprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct DataVolumeReportprotocolIEs(Vec<DataVolumeReportprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DataVolumeReportprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DataVolumeReportprotocolExtensions(Vec<DataVolumeReportprotocolExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DataVolumeReportRequestprotocolIEs_Itemvalue {
    #[asn(key = 33)]
    id_RAB_DataVolumeReportRequestList(RAB_DataVolumeReportRequestList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DataVolumeReportRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: DataVolumeReportRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct DataVolumeReportRequestprotocolIEs(Vec<DataVolumeReportRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DataVolumeReportRequestprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DataVolumeReportRequestprotocolExtensions(
    Vec<DataVolumeReportRequestprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DeltaRAListofIdleModeUEsiE_Extensions_ItemextensionValue {
    #[asn(key = 182)]
    id_LAListwithNoIdleModeUEsAnyMore(LAListofIdleModeUEs),
    #[asn(key = 181)]
    id_newLAListofIdleModeUEs(LAListofIdleModeUEs),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DeltaRAListofIdleModeUEsiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: DeltaRAListofIdleModeUEsiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DeltaRAListofIdleModeUEsiE_Extensions(Vec<DeltaRAListofIdleModeUEsiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DirectInformationTransferprotocolIEs_Itemvalue {
    #[asn(key = 3)]
    id_CN_DomainIndicator(CN_DomainIndicator),
    #[asn(key = 96)]
    id_GlobalCN_ID(GlobalCN_ID),
    #[asn(key = 86)]
    id_GlobalRNC_ID(GlobalRNC_ID),
    #[asn(key = 126)]
    id_InterSystemInformationTransferType(InterSystemInformationTransferType),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DirectInformationTransferprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: DirectInformationTransferprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct DirectInformationTransferprotocolIEs(Vec<DirectInformationTransferprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DirectInformationTransferprotocolExtensions_ItemextensionValue {
    #[asn(key = 171)]
    id_ExtendedRNC_ID(ExtendedRNC_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DirectInformationTransferprotocolExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: DirectInformationTransferprotocolExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DirectInformationTransferprotocolExtensions(
    Vec<DirectInformationTransferprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DirectTransferprotocolIEs_Itemvalue {
    #[asn(key = 15)]
    id_LAI(LAI),
    #[asn(key = 16)]
    id_NAS_PDU(NAS_PDU),
    #[asn(key = 55)]
    id_RAC(RAC),
    #[asn(key = 58)]
    id_SAI(SAI),
    #[asn(key = 59)]
    id_SAPI(SAPI),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DirectTransferprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: DirectTransferprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct DirectTransferprotocolIEs(Vec<DirectTransferprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DirectTransferprotocolExtensions_ItemextensionValue {
    #[asn(key = 241)]
    id_LGW_TransportLayerAddress(TransportLayerAddress),
    #[asn(key = 275)]
    id_LHN_ID(LHN_ID),
    #[asn(key = 128)]
    id_RedirectionCompleted(RedirectionCompleted),
    #[asn(key = 129)]
    id_RedirectionIndication(RedirectionIndication),
    #[asn(key = 273)]
    id_SIPTO_LGW_TransportLayerAddress(TransportLayerAddress),
    #[asn(key = 202)]
    id_SubscriberProfileIDforRFP(SubscriberProfileIDforRFP),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DirectTransferprotocolExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: DirectTransferprotocolExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DirectTransferprotocolExtensions(Vec<DirectTransferprotocolExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DirectTransferInformationItem_RANAP_RelocInfiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DirectTransferInformationItem_RANAP_RelocInfiE_Extensions(
    Vec<DirectTransferInformationItem_RANAP_RelocInfiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DirectTransferInformationList_RANAP_RelocInf_Item_Itemvalue {
    #[asn(key = 80)]
    id_DirectTransferInformationItem_RANAP_RelocInf(DirectTransferInformationItem_RANAP_RelocInf),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DirectTransferInformationList_RANAP_RelocInf_Item_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: DirectTransferInformationList_RANAP_RelocInf_Item_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct DirectTransferInformationList_RANAP_RelocInf_Item(
    Vec<DirectTransferInformationList_RANAP_RelocInf_Item_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "20", sz_ub = "20")]
pub struct BIT_STRING_7(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "28", sz_ub = "28")]
pub struct BIT_STRING_8(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "18", sz_ub = "18")]
pub struct BIT_STRING_9(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "21", sz_ub = "21")]
pub struct BIT_STRING_10(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct INTEGER_11(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum EUTRANFrequencies_ItemiE_Extensions_ItemextensionValue {
    #[asn(key = 271)]
    id_EARFCN_Extended(EARFCN_Extended),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EUTRANFrequencies_ItemiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: EUTRANFrequencies_ItemiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EUTRANFrequencies_ItemiE_Extensions(Vec<EUTRANFrequencies_ItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct EUTRANFrequencies_Item {
    pub earfcn: INTEGER_11,
    #[asn(optional_idx = 0)]
    pub meas_band: Option<MeasBand>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<EUTRANFrequencies_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EncryptionInformationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EncryptionInformationiE_Extensions(Vec<EncryptionInformationiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum EnhancedRelocationCompleteConfirmprotocolIEs_Itemvalue {
    #[asn(key = 35)]
    id_RAB_FailedList(RAB_FailedList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EnhancedRelocationCompleteConfirmprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: EnhancedRelocationCompleteConfirmprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct EnhancedRelocationCompleteConfirmprotocolIEs(
    Vec<EnhancedRelocationCompleteConfirmprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EnhancedRelocationCompleteConfirmprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EnhancedRelocationCompleteConfirmprotocolExtensions(
    Vec<EnhancedRelocationCompleteConfirmprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum EnhancedRelocationCompleteFailureprotocolIEs_Itemvalue {
    #[asn(key = 4)]
    id_Cause(Cause),
    #[asn(key = 9)]
    id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EnhancedRelocationCompleteFailureprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: EnhancedRelocationCompleteFailureprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct EnhancedRelocationCompleteFailureprotocolIEs(
    Vec<EnhancedRelocationCompleteFailureprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EnhancedRelocationCompleteFailureprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EnhancedRelocationCompleteFailureprotocolExtensions(
    Vec<EnhancedRelocationCompleteFailureprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum EnhancedRelocationCompleteRequestprotocolIEs_Itemvalue {
    #[asn(key = 79)]
    id_IuSigConId(IuSignallingConnectionIdentifier),
    #[asn(key = 196)]
    id_OldIuSigConId(IuSignallingConnectionIdentifier),
    #[asn(key = 188)]
    id_RAB_SetupList_EnhancedRelocCompleteReq(RAB_SetupList_EnhancedRelocCompleteReq),
    #[asn(key = 223)]
    id_Relocation_SourceExtendedRNC_ID(ExtendedRNC_ID),
    #[asn(key = 222)]
    id_Relocation_SourceRNC_ID(GlobalRNC_ID),
    #[asn(key = 213)]
    id_Relocation_TargetExtendedRNC_ID(ExtendedRNC_ID),
    #[asn(key = 212)]
    id_Relocation_TargetRNC_ID(GlobalRNC_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EnhancedRelocationCompleteRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: EnhancedRelocationCompleteRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct EnhancedRelocationCompleteRequestprotocolIEs(
    Vec<EnhancedRelocationCompleteRequestprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum EnhancedRelocationCompleteRequestprotocolExtensions_ItemextensionValue {
    #[asn(key = 203)]
    id_CSG_Id(CSG_Id),
    #[asn(key = 235)]
    id_Cell_Access_Mode(Cell_Access_Mode),
    #[asn(key = 5)]
    id_ChosenEncryptionAlgorithm(ChosenEncryptionAlgorithm),
    #[asn(key = 6)]
    id_ChosenIntegrityProtectionAlgorithm(ChosenIntegrityProtectionAlgorithm),
    #[asn(key = 250)]
    id_HigherBitratesThan16MbpsFlag(HigherBitratesThan16MbpsFlag),
    #[asn(key = 275)]
    id_LHN_ID(LHN_ID),
    #[asn(key = 262)]
    id_Tunnel_Information_for_BBF(TunnelInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EnhancedRelocationCompleteRequestprotocolExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: EnhancedRelocationCompleteRequestprotocolExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EnhancedRelocationCompleteRequestprotocolExtensions(
    Vec<EnhancedRelocationCompleteRequestprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum EnhancedRelocationCompleteResponseprotocolIEs_Itemvalue {
    #[asn(key = 9)]
    id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 190)]
    id_RAB_SetupList_EnhancedRelocCompleteRes(RAB_SetupList_EnhancedRelocCompleteRes),
    #[asn(key = 210)]
    id_RAB_ToBeReleasedList_EnhancedRelocCompleteRes(RAB_ToBeReleasedList_EnhancedRelocCompleteRes),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EnhancedRelocationCompleteResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: EnhancedRelocationCompleteResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct EnhancedRelocationCompleteResponseprotocolIEs(
    Vec<EnhancedRelocationCompleteResponseprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum EnhancedRelocationCompleteResponseprotocolExtensions_ItemextensionValue {
    #[asn(key = 234)]
    id_CSG_Membership_Status(CSG_Membership_Status),
    #[asn(key = 239)]
    id_MSISDN(MSISDN),
    #[asn(key = 233)]
    id_UE_AggregateMaximumBitRate(UE_AggregateMaximumBitRate),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EnhancedRelocationCompleteResponseprotocolExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: EnhancedRelocationCompleteResponseprotocolExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EnhancedRelocationCompleteResponseprotocolExtensions(
    Vec<EnhancedRelocationCompleteResponseprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ErrorIndicationprotocolIEs_Itemvalue {
    #[asn(key = 3)]
    id_CN_DomainIndicator(CN_DomainIndicator),
    #[asn(key = 4)]
    id_Cause(Cause),
    #[asn(key = 9)]
    id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 86)]
    id_GlobalRNC_ID(GlobalRNC_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ErrorIndicationprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ErrorIndicationprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ErrorIndicationprotocolIEs(Vec<ErrorIndicationprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ErrorIndicationprotocolExtensions_ItemextensionValue {
    #[asn(key = 171)]
    id_ExtendedRNC_ID(ExtendedRNC_ID),
    #[asn(key = 96)]
    id_GlobalCN_ID(GlobalCN_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ErrorIndicationprotocolExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: ErrorIndicationprotocolExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ErrorIndicationprotocolExtensions(Vec<ErrorIndicationprotocolExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "-120", ub = "165")]
pub struct INTEGER_12(i8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "-120", ub = "-25")]
pub struct INTEGER_13(i8);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ForwardSRNS_ContextprotocolIEs_Itemvalue {
    #[asn(key = 25)]
    id_RAB_ContextList(RAB_ContextList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ForwardSRNS_ContextprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ForwardSRNS_ContextprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ForwardSRNS_ContextprotocolIEs(Vec<ForwardSRNS_ContextprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ForwardSRNS_ContextprotocolExtensions_ItemextensionValue {
    #[asn(key = 103)]
    id_SourceRNC_PDCP_context_info(RRC_Container),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ForwardSRNS_ContextprotocolExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: ForwardSRNS_ContextprotocolExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ForwardSRNS_ContextprotocolExtensions(Vec<ForwardSRNS_ContextprotocolExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct ENUMERATED_14(u8);
impl ENUMERATED_14 {
    const HEIGHT: u8 = 0u8;
    const DEPTH: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "32767")]
pub struct INTEGER_15(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct INTEGER_16(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct INTEGER_17(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "179")]
pub struct INTEGER_18(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "179")]
pub struct INTEGER_19(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct INTEGER_20(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GA_EllipsoidArciE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GA_EllipsoidArciE_Extensions(Vec<GA_EllipsoidArciE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GA_PointiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GA_PointiE_Extensions(Vec<GA_PointiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GA_PointWithAltitudeiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GA_PointWithAltitudeiE_Extensions(Vec<GA_PointWithAltitudeiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct INTEGER_21(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct INTEGER_22(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GA_PointWithAltitudeAndUncertaintyEllipsoidiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GA_PointWithAltitudeAndUncertaintyEllipsoidiE_Extensions(
    Vec<GA_PointWithAltitudeAndUncertaintyEllipsoidiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GA_PointWithUnCertaintyiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GA_PointWithUnCertaintyiE_Extensions(Vec<GA_PointWithUnCertaintyiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct INTEGER_23(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct INTEGER_24(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GA_PointWithUnCertaintyEllipseiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GA_PointWithUnCertaintyEllipseiE_Extensions(
    Vec<GA_PointWithUnCertaintyEllipseiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GA_Polygon_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GA_Polygon_ItemiE_Extensions(Vec<GA_Polygon_ItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GA_Polygon_Item {
    pub geographical_coordinates: GeographicalCoordinates,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<GA_Polygon_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct INTEGER_25(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct INTEGER_26(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "179")]
pub struct INTEGER_27(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GERAN_Cell_IDiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GERAN_Cell_IDiE_Extensions(Vec<GERAN_Cell_IDiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GERAN_Iumode_RAB_Failed_RABAssgntResponse_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GERAN_Iumode_RAB_Failed_RABAssgntResponse_ItemiE_Extensions(
    Vec<GERAN_Iumode_RAB_Failed_RABAssgntResponse_ItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum GERAN_Iumode_RAB_FailedList_RABAssgntResponse_Item_Itemvalue {
    #[asn(key = 109)]
    id_GERAN_Iumode_RAB_Failed_RABAssgntResponse_Item(
        GERAN_Iumode_RAB_Failed_RABAssgntResponse_Item,
    ),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GERAN_Iumode_RAB_FailedList_RABAssgntResponse_Item_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: GERAN_Iumode_RAB_FailedList_RABAssgntResponse_Item_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct GERAN_Iumode_RAB_FailedList_RABAssgntResponse_Item(
    Vec<GERAN_Iumode_RAB_FailedList_RABAssgntResponse_Item_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct ENUMERATED_28(u8);
impl ENUMERATED_28 {
    const NORTH: u8 = 0u8;
    const SOUTH: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "8388607")]
pub struct INTEGER_29(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "-8388608", ub = "8388607")]
pub struct INTEGER_30(i32);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GeographicalCoordinatesiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GeographicalCoordinatesiE_Extensions(Vec<GeographicalCoordinatesiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "359")]
pub struct INTEGER_31(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "2047")]
pub struct INTEGER_32(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HorizontalVelocityiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct HorizontalVelocityiE_Extensions(Vec<HorizontalVelocityiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct INTEGER_33(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HorizontalVelocityWithUncertaintyiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct HorizontalVelocityWithUncertaintyiE_Extensions(
    Vec<HorizontalVelocityWithUncertaintyiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HorizontalWithVerticalVelocityiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct HorizontalWithVerticalVelocityiE_Extensions(
    Vec<HorizontalWithVerticalVelocityiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct INTEGER_34(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct INTEGER_35(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HorizontalWithVerticalVelocityAndUncertaintyiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct HorizontalWithVerticalVelocityAndUncertaintyiE_Extensions(
    Vec<HorizontalWithVerticalVelocityAndUncertaintyiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "7", sz_ub = "7")]
pub struct BIT_STRING_36(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IMEIGroupiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct IMEIGroupiE_Extensions(Vec<IMEIGroupiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "7", sz_ub = "7")]
pub struct BIT_STRING_37(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IMEISVGroupiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct IMEISVGroupiE_Extensions(Vec<IMEISVGroupiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "97")]
pub struct INTEGER_38(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "34")]
pub struct INTEGER_39(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum IRAT_Measurement_ConfigurationiE_Extensions_ItemextensionValue {
    #[asn(key = 279)]
    id_RSRQ_Extension(RSRQ_Extension),
    #[asn(key = 278)]
    id_RSRQ_Type(RSRQ_Type),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IRAT_Measurement_ConfigurationiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: IRAT_Measurement_ConfigurationiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct IRAT_Measurement_ConfigurationiE_Extensions(
    Vec<IRAT_Measurement_ConfigurationiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "100")]
pub struct INTEGER_40(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IRATmeasurementParametersiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct IRATmeasurementParametersiE_Extensions(Vec<IRATmeasurementParametersiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ImmediateMDTiE_Extensions_ItemextensionValue {
    #[asn(key = 265)]
    id_M4Report(M4Report),
    #[asn(key = 266)]
    id_M5Report(M5Report),
    #[asn(key = 267)]
    id_M6Report(M6Report),
    #[asn(key = 268)]
    id_M7Report(M7Report),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ImmediateMDTiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: ImmediateMDTiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ImmediateMDTiE_Extensions(Vec<ImmediateMDTiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum InformationTransferConfirmationprotocolIEs_Itemvalue {
    #[asn(key = 3)]
    id_CN_DomainIndicator(CN_DomainIndicator),
    #[asn(key = 9)]
    id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 86)]
    id_GlobalRNC_ID(GlobalRNC_ID),
    #[asn(key = 104)]
    id_InformationTransferID(InformationTransferID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InformationTransferConfirmationprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: InformationTransferConfirmationprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct InformationTransferConfirmationprotocolIEs(
    Vec<InformationTransferConfirmationprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum InformationTransferConfirmationprotocolExtensions_ItemextensionValue {
    #[asn(key = 171)]
    id_ExtendedRNC_ID(ExtendedRNC_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InformationTransferConfirmationprotocolExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: InformationTransferConfirmationprotocolExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct InformationTransferConfirmationprotocolExtensions(
    Vec<InformationTransferConfirmationprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum InformationTransferFailureprotocolIEs_Itemvalue {
    #[asn(key = 3)]
    id_CN_DomainIndicator(CN_DomainIndicator),
    #[asn(key = 4)]
    id_Cause(Cause),
    #[asn(key = 9)]
    id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 86)]
    id_GlobalRNC_ID(GlobalRNC_ID),
    #[asn(key = 104)]
    id_InformationTransferID(InformationTransferID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InformationTransferFailureprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: InformationTransferFailureprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct InformationTransferFailureprotocolIEs(Vec<InformationTransferFailureprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum InformationTransferFailureprotocolExtensions_ItemextensionValue {
    #[asn(key = 171)]
    id_ExtendedRNC_ID(ExtendedRNC_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InformationTransferFailureprotocolExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: InformationTransferFailureprotocolExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct InformationTransferFailureprotocolExtensions(
    Vec<InformationTransferFailureprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum InformationTransferIndicationprotocolIEs_Itemvalue {
    #[asn(key = 3)]
    id_CN_DomainIndicator(CN_DomainIndicator),
    #[asn(key = 96)]
    id_GlobalCN_ID(GlobalCN_ID),
    #[asn(key = 104)]
    id_InformationTransferID(InformationTransferID),
    #[asn(key = 106)]
    id_ProvidedData(ProvidedData),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InformationTransferIndicationprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: InformationTransferIndicationprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct InformationTransferIndicationprotocolIEs(
    Vec<InformationTransferIndicationprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InformationTransferIndicationprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct InformationTransferIndicationprotocolExtensions(
    Vec<InformationTransferIndicationprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum InitialUE_MessageprotocolIEs_Itemvalue {
    #[asn(key = 3)]
    id_CN_DomainIndicator(CN_DomainIndicator),
    #[asn(key = 86)]
    id_GlobalRNC_ID(GlobalRNC_ID),
    #[asn(key = 79)]
    id_IuSigConId(IuSignallingConnectionIdentifier),
    #[asn(key = 15)]
    id_LAI(LAI),
    #[asn(key = 16)]
    id_NAS_PDU(NAS_PDU),
    #[asn(key = 55)]
    id_RAC(RAC),
    #[asn(key = 58)]
    id_SAI(SAI),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InitialUE_MessageprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: InitialUE_MessageprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct InitialUE_MessageprotocolIEs(Vec<InitialUE_MessageprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum InitialUE_MessageprotocolExtensions_ItemextensionValue {
    #[asn(key = 203)]
    id_CSG_Id(CSG_Id),
    #[asn(key = 235)]
    id_Cell_Access_Mode(Cell_Access_Mode),
    #[asn(key = 291)]
    id_DCN_ID(DCN_ID),
    #[asn(key = 171)]
    id_ExtendedRNC_ID(ExtendedRNC_ID),
    #[asn(key = 108)]
    id_GERAN_Classmark(GERAN_Classmark),
    #[asn(key = 250)]
    id_HigherBitratesThan16MbpsFlag(HigherBitratesThan16MbpsFlag),
    #[asn(key = 241)]
    id_LGW_TransportLayerAddress(TransportLayerAddress),
    #[asn(key = 275)]
    id_LHN_ID(LHN_ID),
    #[asn(key = 130)]
    id_NAS_SequenceNumber(NAS_SequenceNumber),
    #[asn(key = 23)]
    id_PermanentNAS_UE_ID(PermanentNAS_UE_ID),
    #[asn(key = 166)]
    id_RedirectAttemptFlag(RedirectAttemptFlag),
    #[asn(key = 286)]
    id_SGSN_Group_Identity(SGSN_Group_Identity),
    #[asn(key = 273)]
    id_SIPTO_LGW_TransportLayerAddress(TransportLayerAddress),
    #[asn(key = 127)]
    id_SelectedPLMN_ID(PLMNidentity),
    #[asn(key = 262)]
    id_Tunnel_Information_for_BBF(TunnelInformation),
    #[asn(key = 294)]
    id_UE_Application_Layer_Measurement_Capability(UE_Application_Layer_Measurement_Capability),
    #[asn(key = 290)]
    id_UE_Usage_Type(UE_Usage_Type),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InitialUE_MessageprotocolExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: InitialUE_MessageprotocolExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct InitialUE_MessageprotocolExtensions(Vec<InitialUE_MessageprotocolExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum InitiatingMessagevalue {
    #[asn(key = 26)]
    id_CN_DeactivateTrace(CN_DeactivateTrace),
    #[asn(key = 16)]
    id_CN_InvokeTrace(CN_InvokeTrace),
    #[asn(key = 15)]
    id_CommonID(CommonID),
    #[asn(key = 7)]
    id_DataVolumeReport(DataVolumeReportRequest),
    #[asn(key = 34)]
    id_DirectInformationTransfer(DirectInformationTransfer),
    #[asn(key = 20)]
    id_DirectTransfer(DirectTransfer),
    #[asn(key = 22)]
    id_ErrorIndication(ErrorIndication),
    #[asn(key = 24)]
    id_ForwardSRNS_Context(ForwardSRNS_Context),
    #[asn(key = 31)]
    id_InformationTransfer(InformationTransferIndication),
    #[asn(key = 19)]
    id_InitialUE_Message(InitialUE_Message),
    #[asn(key = 1)]
    id_Iu_Release(Iu_ReleaseCommand),
    #[asn(key = 11)]
    id_Iu_ReleaseRequest(Iu_ReleaseRequest),
    #[asn(key = 30)]
    id_LocationRelatedData(LocationRelatedDataRequest),
    #[asn(key = 18)]
    id_LocationReport(LocationReport),
    #[asn(key = 17)]
    id_LocationReportingControl(LocationReportingControl),
    #[asn(key = 40)]
    id_MBMSCNDe_Registration_Procedure(MBMSCNDe_RegistrationRequest),
    #[asn(key = 41)]
    id_MBMSRABEstablishmentIndication(MBMSRABEstablishmentIndication),
    #[asn(key = 42)]
    id_MBMSRABRelease(MBMSRABReleaseRequest),
    #[asn(key = 39)]
    id_MBMSRegistration(MBMSRegistrationRequest),
    #[asn(key = 35)]
    id_MBMSSessionStart(MBMSSessionStart),
    #[asn(key = 37)]
    id_MBMSSessionStop(MBMSSessionStop),
    #[asn(key = 36)]
    id_MBMSSessionUpdate(MBMSSessionUpdate),
    #[asn(key = 38)]
    id_MBMSUELinking(MBMSUELinkingRequest),
    #[asn(key = 21)]
    id_OverloadControl(Overload),
    #[asn(key = 14)]
    id_Paging(Paging),
    #[asn(key = 0)]
    id_RAB_Assignment(RAB_AssignmentRequest),
    #[asn(key = 29)]
    id_RAB_ModifyRequest(RAB_ModifyRequest),
    #[asn(key = 10)]
    id_RAB_ReleaseRequest(RAB_ReleaseRequest),
    #[asn(key = 28)]
    id_RANAP_Relocation(RANAP_RelocationInformation),
    #[asn(key = 45)]
    id_RANAPenhancedRelocation(RANAP_EnhancedRelocationInformationRequest),
    #[asn(key = 4)]
    id_RelocationCancel(RelocationCancel),
    #[asn(key = 13)]
    id_RelocationComplete(RelocationComplete),
    #[asn(key = 12)]
    id_RelocationDetect(RelocationDetect),
    #[asn(key = 2)]
    id_RelocationPreparation(RelocationRequired),
    #[asn(key = 3)]
    id_RelocationResourceAllocation(RelocationRequest),
    #[asn(key = 49)]
    id_RerouteNASRequest(RerouteNASRequest),
    #[asn(key = 9)]
    id_Reset(Reset),
    #[asn(key = 27)]
    id_ResetResource(ResetResource),
    #[asn(key = 5)]
    id_SRNS_ContextTransfer(SRNS_ContextRequest),
    #[asn(key = 23)]
    id_SRNS_DataForward(SRNS_DataForwardCommand),
    #[asn(key = 46)]
    id_SRVCCPreparation(SRVCC_CSKeysRequest),
    #[asn(key = 6)]
    id_SecurityModeControl(SecurityModeCommand),
    #[asn(key = 32)]
    id_UESpecificInformation(UESpecificInformationIndication),
    #[asn(key = 47)]
    id_UeRadioCapabilityMatch(UeRadioCapabilityMatchRequest),
    #[asn(key = 48)]
    id_UeRegistrationQuery(UeRegistrationQueryRequest),
    #[asn(key = 33)]
    id_UplinkInformationExchange(UplinkInformationExchangeRequest),
    #[asn(key = 43)]
    id_enhancedRelocationComplete(EnhancedRelocationCompleteRequest),
    #[asn(key = 44)]
    id_enhancedRelocationCompleteConfirm(EnhancedRelocationCompleteConfirm),
    #[asn(key = 25)]
    id_privateMessage(PrivateMessage),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IntegrityProtectionInformationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct IntegrityProtectionInformationiE_Extensions(
    Vec<IntegrityProtectionInformationiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InterSystemInformation_TransparentContaineriE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct InterSystemInformation_TransparentContaineriE_Extensions(
    Vec<InterSystemInformation_TransparentContaineriE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "4")]
pub struct ENUMERATED_41(u8);
impl ENUMERATED_41 {
    const IU_CS: u8 = 0u8;
    const IU_PS: u8 = 1u8;
    const IUR: u8 = 2u8;
    const IUB: u8 = 3u8;
    const UU: u8 = 4u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InterfacesToTraceItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct InterfacesToTraceItemiE_Extensions(Vec<InterfacesToTraceItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum Iu_ReleaseCommandprotocolIEs_Itemvalue {
    #[asn(key = 4)]
    id_Cause(Cause),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Iu_ReleaseCommandprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: Iu_ReleaseCommandprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct Iu_ReleaseCommandprotocolIEs(Vec<Iu_ReleaseCommandprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum Iu_ReleaseCommandprotocolExtensions_ItemextensionValue {
    #[asn(key = 252)]
    id_End_Of_CSFB(End_Of_CSFB),
    #[asn(key = 277)]
    id_LastE_UTRANPLMNIdentity(PLMNidentity),
    #[asn(key = 254)]
    id_Out_Of_UTRAN(Out_Of_UTRAN),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Iu_ReleaseCommandprotocolExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: Iu_ReleaseCommandprotocolExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Iu_ReleaseCommandprotocolExtensions(Vec<Iu_ReleaseCommandprotocolExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum Iu_ReleaseCompleteprotocolIEs_Itemvalue {
    #[asn(key = 9)]
    id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 31)]
    id_RAB_DataVolumeReportList(RAB_DataVolumeReportList),
    #[asn(key = 44)]
    id_RAB_ReleasedList_IuRelComp(RAB_ReleasedList_IuRelComp),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Iu_ReleaseCompleteprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: Iu_ReleaseCompleteprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct Iu_ReleaseCompleteprotocolIEs(Vec<Iu_ReleaseCompleteprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Iu_ReleaseCompleteprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Iu_ReleaseCompleteprotocolExtensions(Vec<Iu_ReleaseCompleteprotocolExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum Iu_ReleaseRequestprotocolIEs_Itemvalue {
    #[asn(key = 4)]
    id_Cause(Cause),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Iu_ReleaseRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: Iu_ReleaseRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct Iu_ReleaseRequestprotocolIEs(Vec<Iu_ReleaseRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Iu_ReleaseRequestprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Iu_ReleaseRequestprotocolExtensions(Vec<Iu_ReleaseRequestprotocolExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct JoinedMBMSBearerService_IEs_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct JoinedMBMSBearerService_IEs_ItemiE_Extensions(
    Vec<JoinedMBMSBearerService_IEs_ItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct JoinedMBMSBearerService_IEs_Item {
    pub t_mgi: TMGI,
    pub m_bms_ptp_rab_id: MBMS_PTP_RAB_ID,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<JoinedMBMSBearerService_IEs_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LA_LIST_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LA_LIST_ItemiE_Extensions(Vec<LA_LIST_ItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LA_LIST_Item {
    pub l_ac: LAC,
    pub list_of_sn_as: ListOF_SNAs,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<LA_LIST_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LABasediE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LABasediE_Extensions(Vec<LABasediE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LAIiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LAIiE_Extensions(Vec<LAIiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "32767")]
pub struct INTEGER_42(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LastKnownServiceAreaiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LastKnownServiceAreaiE_Extensions(Vec<LastKnownServiceAreaiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum LastVisitedUTRANCell_ItemiE_Extensions_ItemextensionValue {
    #[asn(key = 257)]
    id_HO_Cause(Cause),
    #[asn(key = 253)]
    id_Time_UE_StayedInCell_EnhancedGranularity(Time_UE_StayedInCell_EnhancedGranularity),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LastVisitedUTRANCell_ItemiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: LastVisitedUTRANCell_ItemiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LastVisitedUTRANCell_ItemiE_Extensions(Vec<LastVisitedUTRANCell_ItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LeftMBMSBearerService_IEs_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LeftMBMSBearerService_IEs_ItemiE_Extensions(
    Vec<LeftMBMSBearerService_IEs_ItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LeftMBMSBearerService_IEs_Item {
    pub t_mgi: TMGI,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<LeftMBMSBearerService_IEs_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum LocationRelatedDataFailureprotocolIEs_Itemvalue {
    #[asn(key = 4)]
    id_Cause(Cause),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationRelatedDataFailureprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: LocationRelatedDataFailureprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct LocationRelatedDataFailureprotocolIEs(Vec<LocationRelatedDataFailureprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum LocationRelatedDataFailureprotocolExtensions_ItemextensionValue {
    #[asn(key = 9)]
    id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationRelatedDataFailureprotocolExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: LocationRelatedDataFailureprotocolExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LocationRelatedDataFailureprotocolExtensions(
    Vec<LocationRelatedDataFailureprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum LocationRelatedDataRequestprotocolIEs_Itemvalue {
    #[asn(key = 95)]
    id_LocationRelatedDataRequestType(LocationRelatedDataRequestType),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationRelatedDataRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: LocationRelatedDataRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct LocationRelatedDataRequestprotocolIEs(Vec<LocationRelatedDataRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum LocationRelatedDataRequestprotocolExtensions_ItemextensionValue {
    #[asn(key = 115)]
    id_LocationRelatedDataRequestTypeSpecificToGERANIuMode(
        LocationRelatedDataRequestTypeSpecificToGERANIuMode,
    ),
    #[asn(key = 185)]
    id_RequestedGANSSAssistanceData(RequestedGANSSAssistanceData),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationRelatedDataRequestprotocolExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: LocationRelatedDataRequestprotocolExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LocationRelatedDataRequestprotocolExtensions(
    Vec<LocationRelatedDataRequestprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum LocationRelatedDataResponseprotocolIEs_Itemvalue {
    #[asn(key = 94)]
    id_BroadcastAssistanceDataDecipheringKeys(BroadcastAssistanceDataDecipheringKeys),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationRelatedDataResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: LocationRelatedDataResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct LocationRelatedDataResponseprotocolIEs(Vec<LocationRelatedDataResponseprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum LocationRelatedDataResponseprotocolExtensions_ItemextensionValue {
    #[asn(key = 186)]
    id_BroadcastGANSSAssistanceDataDecipheringKeys(BroadcastAssistanceDataDecipheringKeys),
    #[asn(key = 9)]
    id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationRelatedDataResponseprotocolExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: LocationRelatedDataResponseprotocolExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LocationRelatedDataResponseprotocolExtensions(
    Vec<LocationRelatedDataResponseprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum LocationReportprotocolIEs_Itemvalue {
    #[asn(key = 0)]
    id_AreaIdentity(AreaIdentity),
    #[asn(key = 4)]
    id_Cause(Cause),
    #[asn(key = 57)]
    id_RequestType(RequestType),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationReportprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: LocationReportprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct LocationReportprotocolIEs(Vec<LocationReportprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum LocationReportprotocolExtensions_ItemextensionValue {
    #[asn(key = 122)]
    id_AccuracyFulfilmentIndicator(AccuracyFulfilmentIndicator),
    #[asn(key = 283)]
    id_BarometricPressure(BarometricPressure),
    #[asn(key = 285)]
    id_CivicAddress(CivicAddress),
    #[asn(key = 97)]
    id_LastKnownServiceArea(LastKnownServiceArea),
    #[asn(key = 119)]
    id_PositionData(PositionData),
    #[asn(key = 120)]
    id_PositionDataSpecificToGERANIuMode(PositionDataSpecificToGERANIuMode),
    #[asn(key = 165)]
    id_VelocityEstimate(VelocityEstimate),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationReportprotocolExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: LocationReportprotocolExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LocationReportprotocolExtensions(Vec<LocationReportprotocolExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum LocationReportingControlprotocolIEs_Itemvalue {
    #[asn(key = 57)]
    id_RequestType(RequestType),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationReportingControlprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: LocationReportingControlprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct LocationReportingControlprotocolIEs(Vec<LocationReportingControlprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum LocationReportingControlprotocolExtensions_ItemextensionValue {
    #[asn(key = 114)]
    id_ClientType(ClientType),
    #[asn(key = 164)]
    id_IncludeVelocity(IncludeVelocity),
    #[asn(key = 168)]
    id_PeriodicLocationInfo(PeriodicLocationInfo),
    #[asn(key = 113)]
    id_PositioningPriority(PositioningPriority),
    #[asn(key = 112)]
    id_ResponseTime(ResponseTime),
    #[asn(key = 111)]
    id_VerticalAccuracyCode(VerticalAccuracyCode),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationReportingControlprotocolExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: LocationReportingControlprotocolExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LocationReportingControlprotocolExtensions(
    Vec<LocationReportingControlprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationReportingTransferInformationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LocationReportingTransferInformationiE_Extensions(
    Vec<LocationReportingTransferInformationiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LoggedMDTiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LoggedMDTiE_Extensions(Vec<LoggedMDTiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M4_Collection_ParametersiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct M4_Collection_ParametersiE_Extensions(Vec<M4_Collection_ParametersiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "NULL")]
pub struct NULL_43;

#[derive(Debug, AperCodec)]
#[asn(type = "NULL")]
pub struct NULL_44;

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M6ReportiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct M6ReportiE_Extensions(Vec<M6ReportiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M7ReportiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct M7ReportiE_Extensions(Vec<M7ReportiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MBMSCNDe_RegistrationRequestprotocolIEs_Itemvalue {
    #[asn(key = 96)]
    id_GlobalCN_ID(GlobalCN_ID),
    #[asn(key = 153)]
    id_TMGI(TMGI),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSCNDe_RegistrationRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSCNDe_RegistrationRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSCNDe_RegistrationRequestprotocolIEs(
    Vec<MBMSCNDe_RegistrationRequestprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSCNDe_RegistrationRequestprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSCNDe_RegistrationRequestprotocolExtensions(
    Vec<MBMSCNDe_RegistrationRequestprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MBMSCNDe_RegistrationResponseprotocolIEs_Itemvalue {
    #[asn(key = 4)]
    id_Cause(Cause),
    #[asn(key = 9)]
    id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 86)]
    id_GlobalRNC_ID(GlobalRNC_ID),
    #[asn(key = 153)]
    id_TMGI(TMGI),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSCNDe_RegistrationResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSCNDe_RegistrationResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSCNDe_RegistrationResponseprotocolIEs(
    Vec<MBMSCNDe_RegistrationResponseprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MBMSCNDe_RegistrationResponseprotocolExtensions_ItemextensionValue {
    #[asn(key = 171)]
    id_ExtendedRNC_ID(ExtendedRNC_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSCNDe_RegistrationResponseprotocolExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: MBMSCNDe_RegistrationResponseprotocolExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSCNDe_RegistrationResponseprotocolExtensions(
    Vec<MBMSCNDe_RegistrationResponseprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSIPMulticastAddressandAPNlistiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSIPMulticastAddressandAPNlistiE_Extensions(
    Vec<MBMSIPMulticastAddressandAPNlistiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MBMSRABEstablishmentIndicationprotocolIEs_Itemvalue {
    #[asn(key = 154)]
    id_TransportLayerInformation(TransportLayerInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSRABEstablishmentIndicationprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSRABEstablishmentIndicationprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSRABEstablishmentIndicationprotocolIEs(
    Vec<MBMSRABEstablishmentIndicationprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSRABEstablishmentIndicationprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSRABEstablishmentIndicationprotocolExtensions(
    Vec<MBMSRABEstablishmentIndicationprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MBMSRABReleaseprotocolIEs_Itemvalue {
    #[asn(key = 4)]
    id_Cause(Cause),
    #[asn(key = 9)]
    id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSRABReleaseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSRABReleaseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSRABReleaseprotocolIEs(Vec<MBMSRABReleaseprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSRABReleaseprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSRABReleaseprotocolExtensions(Vec<MBMSRABReleaseprotocolExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MBMSRABReleaseFailureprotocolIEs_Itemvalue {
    #[asn(key = 4)]
    id_Cause(Cause),
    #[asn(key = 9)]
    id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSRABReleaseFailureprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSRABReleaseFailureprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSRABReleaseFailureprotocolIEs(Vec<MBMSRABReleaseFailureprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSRABReleaseFailureprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSRABReleaseFailureprotocolExtensions(
    Vec<MBMSRABReleaseFailureprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MBMSRABReleaseRequestprotocolIEs_Itemvalue {
    #[asn(key = 4)]
    id_Cause(Cause),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSRABReleaseRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSRABReleaseRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSRABReleaseRequestprotocolIEs(Vec<MBMSRABReleaseRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSRABReleaseRequestprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSRABReleaseRequestprotocolExtensions(
    Vec<MBMSRABReleaseRequestprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MBMSRegistrationFailureprotocolIEs_Itemvalue {
    #[asn(key = 4)]
    id_Cause(Cause),
    #[asn(key = 9)]
    id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 96)]
    id_GlobalCN_ID(GlobalCN_ID),
    #[asn(key = 153)]
    id_TMGI(TMGI),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSRegistrationFailureprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSRegistrationFailureprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSRegistrationFailureprotocolIEs(Vec<MBMSRegistrationFailureprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSRegistrationFailureprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSRegistrationFailureprotocolExtensions(
    Vec<MBMSRegistrationFailureprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MBMSRegistrationRequestprotocolIEs_Itemvalue {
    #[asn(key = 132)]
    id_APN(APN),
    #[asn(key = 86)]
    id_GlobalRNC_ID(GlobalRNC_ID),
    #[asn(key = 140)]
    id_IPMulticastAddress(IPMulticastAddress),
    #[asn(key = 151)]
    id_MBMSRegistrationRequestType(MBMSRegistrationRequestType),
    #[asn(key = 153)]
    id_TMGI(TMGI),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSRegistrationRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSRegistrationRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSRegistrationRequestprotocolIEs(Vec<MBMSRegistrationRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MBMSRegistrationRequestprotocolExtensions_ItemextensionValue {
    #[asn(key = 171)]
    id_ExtendedRNC_ID(ExtendedRNC_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSRegistrationRequestprotocolExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: MBMSRegistrationRequestprotocolExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSRegistrationRequestprotocolExtensions(
    Vec<MBMSRegistrationRequestprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MBMSRegistrationResponseprotocolIEs_Itemvalue {
    #[asn(key = 9)]
    id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 96)]
    id_GlobalCN_ID(GlobalCN_ID),
    #[asn(key = 153)]
    id_TMGI(TMGI),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSRegistrationResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSRegistrationResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSRegistrationResponseprotocolIEs(Vec<MBMSRegistrationResponseprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSRegistrationResponseprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSRegistrationResponseprotocolExtensions(
    Vec<MBMSRegistrationResponseprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MBMSSessionStartprotocolIEs_Itemvalue {
    #[asn(key = 135)]
    id_FrequenceLayerConvergenceFlag(FrequenceLayerConvergenceFlag),
    #[asn(key = 96)]
    id_GlobalCN_ID(GlobalCN_ID),
    #[asn(key = 79)]
    id_IuSigConId(IuSignallingConnectionIdentifier),
    #[asn(key = 143)]
    id_MBMSBearerServiceType(MBMSBearerServiceType),
    #[asn(key = 145)]
    id_MBMSServiceArea(MBMSServiceArea),
    #[asn(key = 146)]
    id_MBMSSessionDuration(MBMSSessionDuration),
    #[asn(key = 147)]
    id_MBMSSessionIdentity(MBMSSessionIdentity),
    #[asn(key = 157)]
    id_MBMSSessionRepetitionNumber(MBMSSessionRepetitionNumber),
    #[asn(key = 148)]
    id_PDP_TypeInformation(PDP_TypeInformation),
    #[asn(key = 149)]
    id_RAB_Parameters(RAB_Parameters),
    #[asn(key = 150)]
    id_RAListofIdleModeUEs(RAListofIdleModeUEs),
    #[asn(key = 153)]
    id_TMGI(TMGI),
    #[asn(key = 163)]
    id_TimeToMBMSDataTransfer(TimeToMBMSDataTransfer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionStartprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSSessionStartprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSSessionStartprotocolIEs(Vec<MBMSSessionStartprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MBMSSessionStartprotocolExtensions_ItemextensionValue {
    #[asn(key = 169)]
    id_MBMSCountingInformation(MBMSCountingInformation),
    #[asn(key = 201)]
    id_MBMSSynchronisationInformation(MBMSSynchronisationInformation),
    #[asn(key = 238)]
    id_PDP_TypeInformation_extension(PDP_TypeInformation_extension),
    #[asn(key = 276)]
    id_Session_Re_establishment_Indicator(Session_Re_establishment_Indicator),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionStartprotocolExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: MBMSSessionStartprotocolExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSSessionStartprotocolExtensions(Vec<MBMSSessionStartprotocolExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MBMSSessionStartFailureprotocolIEs_Itemvalue {
    #[asn(key = 4)]
    id_Cause(Cause),
    #[asn(key = 9)]
    id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionStartFailureprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSSessionStartFailureprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSSessionStartFailureprotocolIEs(Vec<MBMSSessionStartFailureprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionStartFailureprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSSessionStartFailureprotocolExtensions(
    Vec<MBMSSessionStartFailureprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MBMSSessionStartResponseprotocolIEs_Itemvalue {
    #[asn(key = 4)]
    id_Cause(Cause),
    #[asn(key = 9)]
    id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 154)]
    id_TransportLayerInformation(TransportLayerInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionStartResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSSessionStartResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSSessionStartResponseprotocolIEs(Vec<MBMSSessionStartResponseprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionStartResponseprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSSessionStartResponseprotocolExtensions(
    Vec<MBMSSessionStartResponseprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MBMSSessionStopprotocolIEs_Itemvalue {
    #[asn(key = 144)]
    id_MBMSCNDe_Registration(MBMSCNDe_Registration),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionStopprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSSessionStopprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSSessionStopprotocolIEs(Vec<MBMSSessionStopprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionStopprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSSessionStopprotocolExtensions(Vec<MBMSSessionStopprotocolExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MBMSSessionStopResponseprotocolIEs_Itemvalue {
    #[asn(key = 4)]
    id_Cause(Cause),
    #[asn(key = 9)]
    id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionStopResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSSessionStopResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSSessionStopResponseprotocolIEs(Vec<MBMSSessionStopResponseprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionStopResponseprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSSessionStopResponseprotocolExtensions(
    Vec<MBMSSessionStopResponseprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MBMSSessionUpdateprotocolIEs_Itemvalue {
    #[asn(key = 134)]
    id_DeltaRAListofIdleModeUEs(DeltaRAListofIdleModeUEs),
    #[asn(key = 152)]
    id_SessionUpdateID(SessionUpdateID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionUpdateprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSSessionUpdateprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSSessionUpdateprotocolIEs(Vec<MBMSSessionUpdateprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionUpdateprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSSessionUpdateprotocolExtensions(Vec<MBMSSessionUpdateprotocolExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MBMSSessionUpdateFailureprotocolIEs_Itemvalue {
    #[asn(key = 4)]
    id_Cause(Cause),
    #[asn(key = 9)]
    id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 152)]
    id_SessionUpdateID(SessionUpdateID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionUpdateFailureprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSSessionUpdateFailureprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSSessionUpdateFailureprotocolIEs(Vec<MBMSSessionUpdateFailureprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionUpdateFailureprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSSessionUpdateFailureprotocolExtensions(
    Vec<MBMSSessionUpdateFailureprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MBMSSessionUpdateResponseprotocolIEs_Itemvalue {
    #[asn(key = 4)]
    id_Cause(Cause),
    #[asn(key = 9)]
    id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 152)]
    id_SessionUpdateID(SessionUpdateID),
    #[asn(key = 154)]
    id_TransportLayerInformation(TransportLayerInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionUpdateResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSSessionUpdateResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSSessionUpdateResponseprotocolIEs(Vec<MBMSSessionUpdateResponseprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionUpdateResponseprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSSessionUpdateResponseprotocolExtensions(
    Vec<MBMSSessionUpdateResponseprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MBMSSynchronisationInformationiE_Extensions_ItemextensionValue {
    #[asn(key = 236)]
    id_IP_Source_Address(IPMulticastAddress),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSynchronisationInformationiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: MBMSSynchronisationInformationiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSSynchronisationInformationiE_Extensions(
    Vec<MBMSSynchronisationInformationiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MBMSUELinkingRequestprotocolIEs_Itemvalue {
    #[asn(key = 141)]
    id_JoinedMBMSBearerServicesList(JoinedMBMSBearerService_IEs),
    #[asn(key = 142)]
    id_LeftMBMSBearerServicesList(LeftMBMSBearerService_IEs),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSUELinkingRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSUELinkingRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSUELinkingRequestprotocolIEs(Vec<MBMSUELinkingRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSUELinkingRequestprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSUELinkingRequestprotocolExtensions(Vec<MBMSUELinkingRequestprotocolExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MBMSUELinkingResponseprotocolIEs_Itemvalue {
    #[asn(key = 9)]
    id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 155)]
    id_UnsuccessfulLinkingList(UnsuccessfulLinking_IEs),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSUELinkingResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSUELinkingResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSUELinkingResponseprotocolIEs(Vec<MBMSUELinkingResponseprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSUELinkingResponseprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSUELinkingResponseprotocolExtensions(
    Vec<MBMSUELinkingResponseprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MDT_ConfigurationiE_Extensions_ItemextensionValue {
    #[asn(key = 264)]
    id_SignallingBasedMDTPLMNList(MDT_PLMN_List),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MDT_ConfigurationiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: MDT_ConfigurationiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MDT_ConfigurationiE_Extensions(Vec<MDT_ConfigurationiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "NULL")]
pub struct NULL_45;

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MessageStructure_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MessageStructure_ItemiE_Extensions(Vec<MessageStructure_ItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct MessageStructure_Item {
    pub i_e_id: ProtocolIE_ID,
    #[asn(optional_idx = 0)]
    pub repetition_number: Option<RepetitionNumber1>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<MessageStructure_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NotEmptyRAListofIdleModeUEsiE_Extensions_ItemextensionValue {
    #[asn(key = 180)]
    id_LAofIdleModeUEs(LAListofIdleModeUEs),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NotEmptyRAListofIdleModeUEsiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: NotEmptyRAListofIdleModeUEsiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NotEmptyRAListofIdleModeUEsiE_Extensions(
    Vec<NotEmptyRAListofIdleModeUEsiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Offload_RAB_ParametersiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Offload_RAB_ParametersiE_Extensions(Vec<Offload_RAB_ParametersiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum Outcomevalue {
    #[asn(key = 38)]
    id_MBMSUELinking(MBMSUELinkingResponse),
    #[asn(key = 0)]
    id_RAB_Assignment(RAB_AssignmentResponse),
    #[asn(key = 46)]
    id_SRVCCPreparation(SRVCC_CSKeysResponse),
    #[asn(key = 47)]
    id_UeRadioCapabilityMatch(UeRadioCapabilityMatchResponse),
    #[asn(key = 48)]
    id_UeRegistrationQuery(UeRegistrationQueryResponse),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum OverloadprotocolIEs_Itemvalue {
    #[asn(key = 86)]
    id_GlobalRNC_ID(GlobalRNC_ID),
    #[asn(key = 18)]
    id_NumberOfSteps(NumberOfSteps),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct OverloadprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: OverloadprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct OverloadprotocolIEs(Vec<OverloadprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum OverloadprotocolExtensions_ItemextensionValue {
    #[asn(key = 3)]
    id_CN_DomainIndicator(CN_DomainIndicator),
    #[asn(key = 171)]
    id_ExtendedRNC_ID(ExtendedRNC_ID),
    #[asn(key = 96)]
    id_GlobalCN_ID(GlobalCN_ID),
    #[asn(key = 245)]
    id_Priority_Class_Indicator(Priority_Class_Indicator),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct OverloadprotocolExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: OverloadprotocolExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct OverloadprotocolExtensions(Vec<OverloadprotocolExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PLMNBasediE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PLMNBasediE_Extensions(Vec<PLMNBasediE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PLMNs_in_shared_network_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PLMNs_in_shared_network_ItemiE_Extensions(
    Vec<PLMNs_in_shared_network_ItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PLMNs_in_shared_network_Item {
    pub p_lm_nidentity: PLMNidentity,
    pub l_a_list: LA_LIST,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PLMNs_in_shared_network_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PagingprotocolIEs_Itemvalue {
    #[asn(key = 3)]
    id_CN_DomainIndicator(CN_DomainIndicator),
    #[asn(key = 76)]
    id_DRX_CycleLengthCoefficient(DRX_CycleLengthCoefficient),
    #[asn(key = 17)]
    id_NonSearchingIndication(NonSearchingIndication),
    #[asn(key = 21)]
    id_PagingAreaID(PagingAreaID),
    #[asn(key = 22)]
    id_PagingCause(PagingCause),
    #[asn(key = 23)]
    id_PermanentNAS_UE_ID(PermanentNAS_UE_ID),
    #[asn(key = 64)]
    id_TemporaryUE_ID(TemporaryUE_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PagingprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: PagingprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PagingprotocolIEs(Vec<PagingprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PagingprotocolExtensions_ItemextensionValue {
    #[asn(key = 229)]
    id_CSG_Id_List(CSG_Id_List),
    #[asn(key = 96)]
    id_GlobalCN_ID(GlobalCN_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PagingprotocolExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: PagingprotocolExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PagingprotocolExtensions(Vec<PagingprotocolExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "8639999", extensible = true)]
pub struct INTEGER_46(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "8639999", extensible = true)]
pub struct INTEGER_47(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PeriodicLocationInfoiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PeriodicLocationInfoiE_Extensions(Vec<PeriodicLocationInfoiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PositionDataiE_Extensions_ItemextensionValue {
    #[asn(key = 284)]
    id_Additional_PositioningDataSet(Additional_PositioningDataSet),
    #[asn(key = 184)]
    id_GANSS_PositioningDataSet(GANSS_PositioningDataSet),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PositionDataiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: PositionDataiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PositionDataiE_Extensions(Vec<PositionDataiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct INTEGER_48(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "OBJECT-IDENTIFIER")]
pub struct OBJECT_IDENTIFIER_49;

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PrivateMessageprivateIEs_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PrivateMessageprivateIEs(Vec<PrivateMessageprivateIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_AssignmentRequestprotocolIEs_Itemvalue {
    #[asn(key = 41)]
    id_RAB_ReleaseList(RAB_ReleaseList),
    #[asn(key = 54)]
    id_RAB_SetupOrModifyList(RAB_SetupOrModifyList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_AssignmentRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_AssignmentRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_AssignmentRequestprotocolIEs(Vec<RAB_AssignmentRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_AssignmentRequestprotocolExtensions_ItemextensionValue {
    #[asn(key = 239)]
    id_MSISDN(MSISDN),
    #[asn(key = 233)]
    id_UE_AggregateMaximumBitRate(UE_AggregateMaximumBitRate),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_AssignmentRequestprotocolExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RAB_AssignmentRequestprotocolExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_AssignmentRequestprotocolExtensions(
    Vec<RAB_AssignmentRequestprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_AssignmentResponseprotocolIEs_Itemvalue {
    #[asn(key = 9)]
    id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 35)]
    id_RAB_FailedList(RAB_FailedList),
    #[asn(key = 38)]
    id_RAB_QueuedList(RAB_QueuedList),
    #[asn(key = 39)]
    id_RAB_ReleaseFailedList(RAB_ReleaseFailedList),
    #[asn(key = 43)]
    id_RAB_ReleasedList(RAB_ReleasedList),
    #[asn(key = 52)]
    id_RAB_SetupOrModifiedList(RAB_SetupOrModifiedList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_AssignmentResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_AssignmentResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_AssignmentResponseprotocolIEs(Vec<RAB_AssignmentResponseprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_AssignmentResponseprotocolExtensions_ItemextensionValue {
    #[asn(key = 110)]
    id_GERAN_Iumode_RAB_FailedList_RABAssgntResponse(GERAN_Iumode_RAB_FailedList_RABAssgntResponse),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_AssignmentResponseprotocolExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RAB_AssignmentResponseprotocolExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_AssignmentResponseprotocolExtensions(
    Vec<RAB_AssignmentResponseprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_ContextFailedtoTransferList_Item_Itemvalue {
    #[asn(key = 84)]
    id_RAB_ContextFailedtoTransferItem(RABs_ContextFailedtoTransferItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ContextFailedtoTransferList_Item_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_ContextFailedtoTransferList_Item_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_ContextFailedtoTransferList_Item(Vec<RAB_ContextFailedtoTransferList_Item_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ContextItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_ContextItemiE_Extensions(Vec<RAB_ContextItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ContextItem_RANAP_RelocInfiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_ContextItem_RANAP_RelocInfiE_Extensions(
    Vec<RAB_ContextItem_RANAP_RelocInfiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_ContextList_Item_Itemvalue {
    #[asn(key = 24)]
    id_RAB_ContextItem(RAB_ContextItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ContextList_Item_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_ContextList_Item_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_ContextList_Item(Vec<RAB_ContextList_Item_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_ContextList_RANAP_RelocInf_Item_Itemvalue {
    #[asn(key = 82)]
    id_RAB_ContextItem_RANAP_RelocInf(RAB_ContextItem_RANAP_RelocInf),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ContextList_RANAP_RelocInf_Item_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_ContextList_RANAP_RelocInf_Item_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_ContextList_RANAP_RelocInf_Item(Vec<RAB_ContextList_RANAP_RelocInf_Item_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_DataForwardingItemiE_Extensions_ItemextensionValue {
    #[asn(key = 13)]
    id_IuTransportAssociation(IuTransportAssociation),
    #[asn(key = 67)]
    id_TransportLayerAddress(TransportLayerAddress),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_DataForwardingItemiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RAB_DataForwardingItemiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_DataForwardingItemiE_Extensions(Vec<RAB_DataForwardingItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_DataForwardingItem_SRNS_CtxReqiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_DataForwardingItem_SRNS_CtxReqiE_Extensions(
    Vec<RAB_DataForwardingItem_SRNS_CtxReqiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_DataForwardingList_Item_Itemvalue {
    #[asn(key = 26)]
    id_RAB_DataForwardingItem(RAB_DataForwardingItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_DataForwardingList_Item_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_DataForwardingList_Item_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_DataForwardingList_Item(Vec<RAB_DataForwardingList_Item_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_DataForwardingList_SRNS_CtxReq_Item_Itemvalue {
    #[asn(key = 27)]
    id_RAB_DataForwardingItem_SRNS_CtxReq(RAB_DataForwardingItem_SRNS_CtxReq),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_DataForwardingList_SRNS_CtxReq_Item_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_DataForwardingList_SRNS_CtxReq_Item_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_DataForwardingList_SRNS_CtxReq_Item(
    Vec<RAB_DataForwardingList_SRNS_CtxReq_Item_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_DataVolumeReportItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_DataVolumeReportItemiE_Extensions(Vec<RAB_DataVolumeReportItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_DataVolumeReportList_Item_Itemvalue {
    #[asn(key = 30)]
    id_RAB_DataVolumeReportItem(RAB_DataVolumeReportItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_DataVolumeReportList_Item_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_DataVolumeReportList_Item_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_DataVolumeReportList_Item(Vec<RAB_DataVolumeReportList_Item_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_DataVolumeReportRequestItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_DataVolumeReportRequestItemiE_Extensions(
    Vec<RAB_DataVolumeReportRequestItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_DataVolumeReportRequestList_Item_Itemvalue {
    #[asn(key = 32)]
    id_RAB_DataVolumeReportRequestItem(RAB_DataVolumeReportRequestItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_DataVolumeReportRequestList_Item_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_DataVolumeReportRequestList_Item_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_DataVolumeReportRequestList_Item(Vec<RAB_DataVolumeReportRequestList_Item_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_FailedItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_FailedItemiE_Extensions(Vec<RAB_FailedItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_FailedItem_EnhRelocInfoResiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_FailedItem_EnhRelocInfoResiE_Extensions(
    Vec<RAB_FailedItem_EnhRelocInfoResiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_FailedList_Item_Itemvalue {
    #[asn(key = 34)]
    id_RAB_FailedItem(RAB_FailedItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_FailedList_Item_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_FailedList_Item_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_FailedList_Item(Vec<RAB_FailedList_Item_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_FailedList_EnhRelocInfoRes_Item_Itemvalue {
    #[asn(key = 198)]
    id_RAB_FailedItem_EnhRelocInfoRes(RAB_FailedItem_EnhRelocInfoRes),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_FailedList_EnhRelocInfoRes_Item_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_FailedList_EnhRelocInfoRes_Item_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_FailedList_EnhRelocInfoRes_Item(Vec<RAB_FailedList_EnhRelocInfoRes_Item_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_FailedtoReportList_Item_Itemvalue {
    #[asn(key = 71)]
    id_RAB_FailedtoReportItem(RABs_failed_to_reportItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_FailedtoReportList_Item_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_FailedtoReportList_Item_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_FailedtoReportList_Item(Vec<RAB_FailedtoReportList_Item_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ModifyItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_ModifyItemiE_Extensions(Vec<RAB_ModifyItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_ModifyList_Item_Itemvalue {
    #[asn(key = 92)]
    id_RAB_ModifyItem(RAB_ModifyItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ModifyList_Item_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_ModifyList_Item_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_ModifyList_Item(Vec<RAB_ModifyList_Item_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_ModifyRequestprotocolIEs_Itemvalue {
    #[asn(key = 91)]
    id_RAB_ModifyList(RAB_ModifyList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ModifyRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_ModifyRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_ModifyRequestprotocolIEs(Vec<RAB_ModifyRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ModifyRequestprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_ModifyRequestprotocolExtensions(Vec<RAB_ModifyRequestprotocolExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_ParametersiE_Extensions_ItemextensionValue {
    #[asn(key = 176)]
    id_RAB_Parameter_ExtendedGuaranteedBitrateList(RAB_Parameter_ExtendedGuaranteedBitrateList),
    #[asn(key = 177)]
    id_RAB_Parameter_ExtendedMaxBitrateList(RAB_Parameter_ExtendedMaxBitrateList),
    #[asn(key = 218)]
    id_RAB_Parameter_SupportedGuaranteedBitrateList(SupportedRAB_ParameterBitrateList),
    #[asn(key = 219)]
    id_RAB_Parameter_SupportedMaxBitrateList(SupportedRAB_ParameterBitrateList),
    #[asn(key = 116)]
    id_SignallingIndication(SignallingIndication),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ParametersiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RAB_ParametersiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_ParametersiE_Extensions(Vec<RAB_ParametersiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_QueuedItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_QueuedItemiE_Extensions(Vec<RAB_QueuedItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_QueuedList_Item_Itemvalue {
    #[asn(key = 37)]
    id_RAB_QueuedItem(RAB_QueuedItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_QueuedList_Item_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_QueuedList_Item_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_QueuedList_Item(Vec<RAB_QueuedList_Item_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ReleaseItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_ReleaseItemiE_Extensions(Vec<RAB_ReleaseItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_ReleaseList_Item_Itemvalue {
    #[asn(key = 40)]
    id_RAB_ReleaseItem(RAB_ReleaseItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ReleaseList_Item_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_ReleaseList_Item_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_ReleaseList_Item(Vec<RAB_ReleaseList_Item_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_ReleaseRequestprotocolIEs_Itemvalue {
    #[asn(key = 41)]
    id_RAB_ReleaseList(RAB_ReleaseList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ReleaseRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_ReleaseRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_ReleaseRequestprotocolIEs(Vec<RAB_ReleaseRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ReleaseRequestprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_ReleaseRequestprotocolExtensions(Vec<RAB_ReleaseRequestprotocolExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ReleasedItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_ReleasedItemiE_Extensions(Vec<RAB_ReleasedItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ReleasedItem_IuRelCompiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_ReleasedItem_IuRelCompiE_Extensions(
    Vec<RAB_ReleasedItem_IuRelCompiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_ReleasedList_Item_Itemvalue {
    #[asn(key = 42)]
    id_RAB_ReleasedItem(RAB_ReleasedItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ReleasedList_Item_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_ReleasedList_Item_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_ReleasedList_Item(Vec<RAB_ReleasedList_Item_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_ReleasedList_IuRelComp_Item_Itemvalue {
    #[asn(key = 87)]
    id_RAB_ReleasedItem_IuRelComp(RAB_ReleasedItem_IuRelComp),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ReleasedList_IuRelComp_Item_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_ReleasedList_IuRelComp_Item_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_ReleasedList_IuRelComp_Item(Vec<RAB_ReleasedList_IuRelComp_Item_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_RelocationReleaseItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_RelocationReleaseItemiE_Extensions(Vec<RAB_RelocationReleaseItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_RelocationReleaseList_Item_Itemvalue {
    #[asn(key = 45)]
    id_RAB_RelocationReleaseItem(RAB_RelocationReleaseItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_RelocationReleaseList_Item_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_RelocationReleaseList_Item_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_RelocationReleaseList_Item(Vec<RAB_RelocationReleaseList_Item_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_SetupItem_EnhRelocInfoReqiE_Extensions_ItemextensionValue {
    #[asn(key = 231)]
    id_E_UTRAN_Service_Handover(E_UTRAN_Service_Handover),
    #[asn(key = 238)]
    id_PDP_TypeInformation_extension(PDP_TypeInformation_extension),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupItem_EnhRelocInfoReqiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RAB_SetupItem_EnhRelocInfoReqiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_SetupItem_EnhRelocInfoReqiE_Extensions(
    Vec<RAB_SetupItem_EnhRelocInfoReqiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupItem_EnhRelocInfoResiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_SetupItem_EnhRelocInfoResiE_Extensions(
    Vec<RAB_SetupItem_EnhRelocInfoResiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupItem_EnhancedRelocCompleteReqiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_SetupItem_EnhancedRelocCompleteReqiE_Extensions(
    Vec<RAB_SetupItem_EnhancedRelocCompleteReqiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_SetupItem_EnhancedRelocCompleteResiE_Extensions_ItemextensionValue {
    #[asn(key = 240)]
    id_Offload_RAB_Parameters(Offload_RAB_Parameters),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupItem_EnhancedRelocCompleteResiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RAB_SetupItem_EnhancedRelocCompleteResiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_SetupItem_EnhancedRelocCompleteResiE_Extensions(
    Vec<RAB_SetupItem_EnhancedRelocCompleteResiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_SetupItem_RelocReqiE_Extensions_ItemextensionValue {
    #[asn(key = 89)]
    id_Alt_RAB_Parameters(Alt_RAB_Parameters),
    #[asn(key = 231)]
    id_E_UTRAN_Service_Handover(E_UTRAN_Service_Handover),
    #[asn(key = 107)]
    id_GERAN_BSC_Container(GERAN_BSC_Container),
    #[asn(key = 240)]
    id_Offload_RAB_Parameters(Offload_RAB_Parameters),
    #[asn(key = 238)]
    id_PDP_TypeInformation_extension(PDP_TypeInformation_extension),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupItem_RelocReqiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RAB_SetupItem_RelocReqiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_SetupItem_RelocReqiE_Extensions(Vec<RAB_SetupItem_RelocReqiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_SetupItem_RelocReqAckiE_Extensions_ItemextensionValue {
    #[asn(key = 90)]
    id_Ass_RAB_Parameters(Ass_RAB_Parameters),
    #[asn(key = 13)]
    id_IuTransportAssociation(IuTransportAssociation),
    #[asn(key = 67)]
    id_TransportLayerAddress(TransportLayerAddress),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupItem_RelocReqAckiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RAB_SetupItem_RelocReqAckiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_SetupItem_RelocReqAckiE_Extensions(Vec<RAB_SetupItem_RelocReqAckiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_SetupList_EnhRelocInfoReq_Item_Itemvalue {
    #[asn(key = 193)]
    id_RAB_SetupItem_EnhRelocInfoReq(RAB_SetupItem_EnhRelocInfoReq),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupList_EnhRelocInfoReq_Item_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_SetupList_EnhRelocInfoReq_Item_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_SetupList_EnhRelocInfoReq_Item(Vec<RAB_SetupList_EnhRelocInfoReq_Item_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_SetupList_EnhRelocInfoRes_Item_Itemvalue {
    #[asn(key = 195)]
    id_RAB_SetupItem_EnhRelocInfoRes(RAB_SetupItem_EnhRelocInfoRes),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupList_EnhRelocInfoRes_Item_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_SetupList_EnhRelocInfoRes_Item_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_SetupList_EnhRelocInfoRes_Item(Vec<RAB_SetupList_EnhRelocInfoRes_Item_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_SetupList_EnhancedRelocCompleteReq_Item_Itemvalue {
    #[asn(key = 189)]
    id_RAB_SetupItem_EnhancedRelocCompleteReq(RAB_SetupItem_EnhancedRelocCompleteReq),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupList_EnhancedRelocCompleteReq_Item_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_SetupList_EnhancedRelocCompleteReq_Item_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_SetupList_EnhancedRelocCompleteReq_Item(
    Vec<RAB_SetupList_EnhancedRelocCompleteReq_Item_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_SetupList_EnhancedRelocCompleteRes_Item_Itemvalue {
    #[asn(key = 191)]
    id_RAB_SetupItem_EnhancedRelocCompleteRes(RAB_SetupItem_EnhancedRelocCompleteRes),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupList_EnhancedRelocCompleteRes_Item_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_SetupList_EnhancedRelocCompleteRes_Item_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_SetupList_EnhancedRelocCompleteRes_Item(
    Vec<RAB_SetupList_EnhancedRelocCompleteRes_Item_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_SetupList_RelocReq_Item_Itemvalue {
    #[asn(key = 47)]
    id_RAB_SetupItem_RelocReq(RAB_SetupItem_RelocReq),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupList_RelocReq_Item_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_SetupList_RelocReq_Item_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_SetupList_RelocReq_Item(Vec<RAB_SetupList_RelocReq_Item_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_SetupList_RelocReqAck_Item_Itemvalue {
    #[asn(key = 48)]
    id_RAB_SetupItem_RelocReqAck(RAB_SetupItem_RelocReqAck),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupList_RelocReqAck_Item_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_SetupList_RelocReqAck_Item_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_SetupList_RelocReqAck_Item(Vec<RAB_SetupList_RelocReqAck_Item_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_SetupOrModifiedItemiE_Extensions_ItemextensionValue {
    #[asn(key = 90)]
    id_Ass_RAB_Parameters(Ass_RAB_Parameters),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupOrModifiedItemiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RAB_SetupOrModifiedItemiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_SetupOrModifiedItemiE_Extensions(Vec<RAB_SetupOrModifiedItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_SetupOrModifiedList_Item_Itemvalue {
    #[asn(key = 51)]
    id_RAB_SetupOrModifiedItem(RAB_SetupOrModifiedItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupOrModifiedList_Item_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_SetupOrModifiedList_Item_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_SetupOrModifiedList_Item(Vec<RAB_SetupOrModifiedList_Item_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_SetupOrModifyItemFirstiE_Extensions_ItemextensionValue {
    #[asn(key = 242)]
    id_Correlation_ID(Correlation_ID),
    #[asn(key = 231)]
    id_E_UTRAN_Service_Handover(E_UTRAN_Service_Handover),
    #[asn(key = 274)]
    id_SIPTO_Correlation_ID(Correlation_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupOrModifyItemFirstiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RAB_SetupOrModifyItemFirstiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_SetupOrModifyItemFirstiE_Extensions(
    Vec<RAB_SetupOrModifyItemFirstiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_SetupOrModifyItemSecondiE_Extensions_ItemextensionValue {
    #[asn(key = 89)]
    id_Alt_RAB_Parameters(Alt_RAB_Parameters),
    #[asn(key = 107)]
    id_GERAN_BSC_Container(GERAN_BSC_Container),
    #[asn(key = 240)]
    id_Offload_RAB_Parameters(Offload_RAB_Parameters),
    #[asn(key = 238)]
    id_PDP_TypeInformation_extension(PDP_TypeInformation_extension),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupOrModifyItemSecondiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RAB_SetupOrModifyItemSecondiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_SetupOrModifyItemSecondiE_Extensions(
    Vec<RAB_SetupOrModifyItemSecondiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_SetupOrModifyList_Item_ItemfirstValue {
    #[asn(key = 53)]
    id_RAB_SetupOrModifyItem(RAB_SetupOrModifyItemFirst),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_SetupOrModifyList_Item_ItemsecondValue {
    #[asn(key = 53)]
    id_RAB_SetupOrModifyItem(RAB_SetupOrModifyItemSecond),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupOrModifyList_Item_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub first_criticality: Criticality,
    pub first_value: RAB_SetupOrModifyList_Item_ItemfirstValue,
    pub second_criticality: Criticality,
    pub second_value: RAB_SetupOrModifyList_Item_ItemsecondValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_SetupOrModifyList_Item(Vec<RAB_SetupOrModifyList_Item_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ToBeReleasedItem_EnhancedRelocCompleteResiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_ToBeReleasedItem_EnhancedRelocCompleteResiE_Extensions(
    Vec<RAB_ToBeReleasedItem_EnhancedRelocCompleteResiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_ToBeReleasedList_EnhancedRelocCompleteRes_Item_Itemvalue {
    #[asn(key = 209)]
    id_RAB_ToBeReleasedItem_EnhancedRelocCompleteRes(RAB_ToBeReleasedItem_EnhancedRelocCompleteRes),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ToBeReleasedList_EnhancedRelocCompleteRes_Item_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_ToBeReleasedList_EnhancedRelocCompleteRes_Item_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_ToBeReleasedList_EnhancedRelocCompleteRes_Item(
    Vec<RAB_ToBeReleasedList_EnhancedRelocCompleteRes_Item_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_TrCH_MappingItemiE_Extensions_ItemextensionValue {
    #[asn(key = 3)]
    id_CN_DomainIndicator(CN_DomainIndicator),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_TrCH_MappingItemiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RAB_TrCH_MappingItemiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_TrCH_MappingItemiE_Extensions(Vec<RAB_TrCH_MappingItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RABDataVolumeReport_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RABDataVolumeReport_ItemiE_Extensions(Vec<RABDataVolumeReport_ItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct RABDataVolumeReport_Item {
    pub dl_unsuccessfully_transmitted_data_volume: UnsuccessfullyTransmittedDataVolume,
    #[asn(optional_idx = 0)]
    pub data_volume_reference: Option<DataVolumeReference>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<RABDataVolumeReport_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RABParametersList_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RABParametersList_ItemiE_Extensions(Vec<RABParametersList_ItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct RABParametersList_Item {
    pub rab_id: RAB_ID,
    pub cn_domain: CN_DomainIndicator,
    #[asn(optional_idx = 0)]
    pub rab_data_volume_report: Option<RABDataVolumeReport>,
    #[asn(optional_idx = 1)]
    pub up_information: Option<UPInformation>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<RABParametersList_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RABasediE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RABasediE_Extensions(Vec<RABasediE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RABs_ContextFailedtoTransferItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RABs_ContextFailedtoTransferItemiE_Extensions(
    Vec<RABs_ContextFailedtoTransferItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RABs_failed_to_reportItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RABs_failed_to_reportItemiE_Extensions(Vec<RABs_failed_to_reportItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAIiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAIiE_Extensions(Vec<RAIiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ENUMERATED_50(u8);
impl ENUMERATED_50 {
    const EMPTYLIST: u8 = 0u8;
    const FULLLIST: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RANAP_EnhancedRelocationInformationRequestprotocolIEs_Itemvalue {
    #[asn(key = 133)]
    id_CNMBMSLinkingInformation(CNMBMSLinkingInformation),
    #[asn(key = 206)]
    id_GlobalCN_IDCS(GlobalCN_ID),
    #[asn(key = 207)]
    id_GlobalCN_IDPS(GlobalCN_ID),
    #[asn(key = 204)]
    id_OldIuSigConIdCS(IuSignallingConnectionIdentifier),
    #[asn(key = 205)]
    id_OldIuSigConIdPS(IuSignallingConnectionIdentifier),
    #[asn(key = 192)]
    id_RAB_SetupList_EnhRelocInfoReq(RAB_SetupList_EnhRelocInfoReq),
    #[asn(key = 105)]
    id_SNA_Access_Information(SNA_Access_Information),
    #[asn(key = 127)]
    id_SelectedPLMN_ID(PLMNidentity),
    #[asn(key = 61)]
    id_Source_ToTarget_TransparentContainer(SourceRNC_ToTargetRNC_TransparentContainer),
    #[asn(key = 118)]
    id_UESBI_Iu(UESBI_Iu),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RANAP_EnhancedRelocationInformationRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RANAP_EnhancedRelocationInformationRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RANAP_EnhancedRelocationInformationRequestprotocolIEs(
    Vec<RANAP_EnhancedRelocationInformationRequestprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RANAP_EnhancedRelocationInformationRequestprotocolExtensions_ItemextensionValue {
    #[asn(key = 261)]
    id_AnchorPLMN_ID(PLMNidentity),
    #[asn(key = 203)]
    id_CSG_Id(CSG_Id),
    #[asn(key = 234)]
    id_CSG_Membership_Status(CSG_Membership_Status),
    #[asn(key = 11)]
    id_EncryptionInformation(EncryptionInformation),
    #[asn(key = 12)]
    id_IntegrityProtectionInformation(IntegrityProtectionInformation),
    #[asn(key = 248)]
    id_RABParametersList(RABParametersList),
    #[asn(key = 233)]
    id_UE_AggregateMaximumBitRate(UE_AggregateMaximumBitRate),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RANAP_EnhancedRelocationInformationRequestprotocolExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value:
        RANAP_EnhancedRelocationInformationRequestprotocolExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RANAP_EnhancedRelocationInformationRequestprotocolExtensions(
    Vec<RANAP_EnhancedRelocationInformationRequestprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RANAP_EnhancedRelocationInformationResponseprotocolIEs_Itemvalue {
    #[asn(key = 9)]
    id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 197)]
    id_RAB_FailedList_EnhRelocInfoRes(RAB_FailedList_EnhRelocInfoRes),
    #[asn(key = 194)]
    id_RAB_SetupList_EnhRelocInfoRes(RAB_SetupList_EnhRelocInfoRes),
    #[asn(key = 63)]
    id_Target_ToSource_TransparentContainer(TargetRNC_ToSourceRNC_TransparentContainer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RANAP_EnhancedRelocationInformationResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RANAP_EnhancedRelocationInformationResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RANAP_EnhancedRelocationInformationResponseprotocolIEs(
    Vec<RANAP_EnhancedRelocationInformationResponseprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RANAP_EnhancedRelocationInformationResponseprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RANAP_EnhancedRelocationInformationResponseprotocolExtensions(
    Vec<RANAP_EnhancedRelocationInformationResponseprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RANAP_RelocationInformationprotocolIEs_Itemvalue {
    #[asn(key = 81)]
    id_DirectTransferInformationList_RANAP_RelocInf(DirectTransferInformationList_RANAP_RelocInf),
    #[asn(key = 83)]
    id_RAB_ContextList_RANAP_RelocInf(RAB_ContextList_RANAP_RelocInf),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RANAP_RelocationInformationprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RANAP_RelocationInformationprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RANAP_RelocationInformationprotocolIEs(Vec<RANAP_RelocationInformationprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RANAP_RelocationInformationprotocolExtensions_ItemextensionValue {
    #[asn(key = 247)]
    id_RNSAPRelocationParameters(RNSAPRelocationParameters),
    #[asn(key = 103)]
    id_SourceRNC_PDCP_context_info(RRC_Container),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RANAP_RelocationInformationprotocolExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RANAP_RelocationInformationprotocolExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RANAP_RelocationInformationprotocolExtensions(
    Vec<RANAP_RelocationInformationprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RIM_TransferiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RIM_TransferiE_Extensions(Vec<RIM_TransferiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct ENUMERATED_51(u8);
impl ENUMERATED_51 {
    const ACTIVATED: u8 = 0u8;
    const DEACTIVATED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RNCTraceInformationiE_Extensions_ItemextensionValue {
    #[asn(key = 256)]
    id_IMSI(IMSI),
    #[asn(key = 270)]
    id_Serving_Cell_Identifier(UTRAN_CellID),
    #[asn(key = 251)]
    id_Trace_Collection_Entity_IP_Addess(TransportLayerAddress),
    #[asn(key = 255)]
    id_TraceRecordingSessionReference(TraceRecordingSessionReference),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RNCTraceInformationiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RNCTraceInformationiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RNCTraceInformationiE_Extensions(Vec<RNCTraceInformationiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RNSAPRelocationParametersiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RNSAPRelocationParametersiE_Extensions(Vec<RNSAPRelocationParametersiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "BOOLEAN")]
pub struct BOOLEAN_52(bool);

#[derive(Debug, AperCodec)]
#[asn(type = "BOOLEAN")]
pub struct BOOLEAN_53(bool);

#[derive(Debug, AperCodec)]
#[asn(
    type = "BITSTRING",
    sz_extensible = false,
    sz_lb = "128",
    sz_ub = "128"
)]
pub struct BIT_STRING_54(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "OCTET-STRING",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "32"
)]
pub struct OCTET_STRING_55(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RSRVCC_InformationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RSRVCC_InformationiE_Extensions(Vec<RSRVCC_InformationiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RedirectionIndication_Itemvalue {
    #[asn(key = 280)]
    id_Additional_CSPS_coordination_information(Additional_CSPS_coordination_information),
    #[asn(key = 16)]
    id_NAS_PDU(NAS_PDU),
    #[asn(key = 130)]
    id_NAS_SequenceNumber(NAS_SequenceNumber),
    #[asn(key = 23)]
    id_PermanentNAS_UE_ID(PermanentNAS_UE_ID),
    #[asn(key = 131)]
    id_RejectCauseValue(RejectCauseValue),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RedirectionIndication_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RedirectionIndication_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RelocationCancelprotocolIEs_Itemvalue {
    #[asn(key = 4)]
    id_Cause(Cause),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationCancelprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RelocationCancelprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RelocationCancelprotocolIEs(Vec<RelocationCancelprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationCancelprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RelocationCancelprotocolExtensions(Vec<RelocationCancelprotocolExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RelocationCancelAcknowledgeprotocolIEs_Itemvalue {
    #[asn(key = 9)]
    id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationCancelAcknowledgeprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RelocationCancelAcknowledgeprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RelocationCancelAcknowledgeprotocolIEs(Vec<RelocationCancelAcknowledgeprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationCancelAcknowledgeprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RelocationCancelAcknowledgeprotocolExtensions(
    Vec<RelocationCancelAcknowledgeprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RelocationCommandprotocolIEs_Itemvalue {
    #[asn(key = 9)]
    id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 14)]
    id_L3_Information(L3_Information),
    #[asn(key = 28)]
    id_RAB_DataForwardingList(RAB_DataForwardingList),
    #[asn(key = 46)]
    id_RAB_RelocationReleaseList(RAB_RelocationReleaseList),
    #[asn(key = 63)]
    id_Target_ToSource_TransparentContainer(Target_ToSource_TransparentContainer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationCommandprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RelocationCommandprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RelocationCommandprotocolIEs(Vec<RelocationCommandprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RelocationCommandprotocolExtensions_ItemextensionValue {
    #[asn(key = 99)]
    id_InterSystemInformation_TransparentContainer(InterSystemInformation_TransparentContainer),
    #[asn(key = 260)]
    id_RSRVCC_Information(RSRVCC_Information),
    #[asn(key = 227)]
    id_SRVCC_Information(SRVCC_Information),
    #[asn(key = 162)]
    id_TargetBSS_ToSourceBSS_TransparentContainer(TargetBSS_ToSourceBSS_TransparentContainer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationCommandprotocolExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RelocationCommandprotocolExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RelocationCommandprotocolExtensions(Vec<RelocationCommandprotocolExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationCompleteprotocolIEs_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RelocationCompleteprotocolIEs(Vec<RelocationCompleteprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RelocationCompleteprotocolExtensions_ItemextensionValue {
    #[asn(key = 250)]
    id_HigherBitratesThan16MbpsFlag(HigherBitratesThan16MbpsFlag),
    #[asn(key = 275)]
    id_LHN_ID(LHN_ID),
    #[asn(key = 262)]
    id_Tunnel_Information_for_BBF(TunnelInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationCompleteprotocolExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RelocationCompleteprotocolExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RelocationCompleteprotocolExtensions(Vec<RelocationCompleteprotocolExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationDetectprotocolIEs_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RelocationDetectprotocolIEs(Vec<RelocationDetectprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationDetectprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RelocationDetectprotocolExtensions(Vec<RelocationDetectprotocolExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RelocationFailureprotocolIEs_Itemvalue {
    #[asn(key = 4)]
    id_Cause(Cause),
    #[asn(key = 9)]
    id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationFailureprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RelocationFailureprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RelocationFailureprotocolIEs(Vec<RelocationFailureprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RelocationFailureprotocolExtensions_ItemextensionValue {
    #[asn(key = 108)]
    id_GERAN_Classmark(GERAN_Classmark),
    #[asn(key = 100)]
    id_NewBSS_To_OldBSS_Information(NewBSS_To_OldBSS_Information),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationFailureprotocolExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RelocationFailureprotocolExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RelocationFailureprotocolExtensions(Vec<RelocationFailureprotocolExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RelocationPreparationFailureprotocolIEs_Itemvalue {
    #[asn(key = 4)]
    id_Cause(Cause),
    #[asn(key = 9)]
    id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationPreparationFailureprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RelocationPreparationFailureprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RelocationPreparationFailureprotocolIEs(
    Vec<RelocationPreparationFailureprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RelocationPreparationFailureprotocolExtensions_ItemextensionValue {
    #[asn(key = 99)]
    id_InterSystemInformation_TransparentContainer(InterSystemInformation_TransparentContainer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationPreparationFailureprotocolExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RelocationPreparationFailureprotocolExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RelocationPreparationFailureprotocolExtensions(
    Vec<RelocationPreparationFailureprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RelocationRequestprotocolIEs_Itemvalue {
    #[asn(key = 3)]
    id_CN_DomainIndicator(CN_DomainIndicator),
    #[asn(key = 4)]
    id_Cause(Cause),
    #[asn(key = 11)]
    id_EncryptionInformation(EncryptionInformation),
    #[asn(key = 12)]
    id_IntegrityProtectionInformation(IntegrityProtectionInformation),
    #[asn(key = 79)]
    id_IuSigConId(IuSignallingConnectionIdentifier),
    #[asn(key = 23)]
    id_PermanentNAS_UE_ID(PermanentNAS_UE_ID),
    #[asn(key = 49)]
    id_RAB_SetupList_RelocReq(RAB_SetupList_RelocReq),
    #[asn(key = 61)]
    id_Source_ToTarget_TransparentContainer(SourceRNC_ToTargetRNC_TransparentContainer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RelocationRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RelocationRequestprotocolIEs(Vec<RelocationRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RelocationRequestprotocolExtensions_ItemextensionValue {
    #[asn(key = 261)]
    id_AnchorPLMN_ID(PLMNidentity),
    #[asn(key = 133)]
    id_CNMBMSLinkingInformation(CNMBMSLinkingInformation),
    #[asn(key = 203)]
    id_CSG_Id(CSG_Id),
    #[asn(key = 234)]
    id_CSG_Membership_Status(CSG_Membership_Status),
    #[asn(key = 96)]
    id_GlobalCN_ID(GlobalCN_ID),
    #[asn(key = 239)]
    id_MSISDN(MSISDN),
    #[asn(key = 289)]
    id_PowerSavingIndicator(PowerSavingIndicator),
    #[asn(key = 105)]
    id_SNA_Access_Information(SNA_Access_Information),
    #[asn(key = 127)]
    id_SelectedPLMN_ID(PLMNidentity),
    #[asn(key = 233)]
    id_UE_AggregateMaximumBitRate(UE_AggregateMaximumBitRate),
    #[asn(key = 293)]
    id_UE_Application_Layer_Measurement_Configuration_For_Relocation(
        UE_Application_Layer_Measurement_Configuration_For_Relocation,
    ),
    #[asn(key = 118)]
    id_UESBI_Iu(UESBI_Iu),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationRequestprotocolExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RelocationRequestprotocolExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RelocationRequestprotocolExtensions(Vec<RelocationRequestprotocolExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RelocationRequestAcknowledgeprotocolIEs_Itemvalue {
    #[asn(key = 5)]
    id_ChosenEncryptionAlgorithm(ChosenEncryptionAlgorithm),
    #[asn(key = 6)]
    id_ChosenIntegrityProtectionAlgorithm(ChosenIntegrityProtectionAlgorithm),
    #[asn(key = 9)]
    id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 35)]
    id_RAB_FailedList(RAB_FailedList),
    #[asn(key = 50)]
    id_RAB_SetupList_RelocReqAck(RAB_SetupList_RelocReqAck),
    #[asn(key = 63)]
    id_Target_ToSource_TransparentContainer(TargetRNC_ToSourceRNC_TransparentContainer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationRequestAcknowledgeprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RelocationRequestAcknowledgeprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RelocationRequestAcknowledgeprotocolIEs(
    Vec<RelocationRequestAcknowledgeprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RelocationRequestAcknowledgeprotocolExtensions_ItemextensionValue {
    #[asn(key = 203)]
    id_CSG_Id(CSG_Id),
    #[asn(key = 100)]
    id_NewBSS_To_OldBSS_Information(NewBSS_To_OldBSS_Information),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationRequestAcknowledgeprotocolExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RelocationRequestAcknowledgeprotocolExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RelocationRequestAcknowledgeprotocolExtensions(
    Vec<RelocationRequestAcknowledgeprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RelocationRequiredprotocolIEs_Itemvalue {
    #[asn(key = 4)]
    id_Cause(Cause),
    #[asn(key = 7)]
    id_ClassmarkInformation2(ClassmarkInformation2),
    #[asn(key = 8)]
    id_ClassmarkInformation3(ClassmarkInformation3),
    #[asn(key = 20)]
    id_OldBSS_ToNewBSS_Information(OldBSS_ToNewBSS_Information),
    #[asn(key = 56)]
    id_RelocationType(RelocationType),
    #[asn(key = 61)]
    id_Source_ToTarget_TransparentContainer(Source_ToTarget_TransparentContainer),
    #[asn(key = 60)]
    id_SourceID(SourceID),
    #[asn(key = 62)]
    id_TargetID(TargetID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationRequiredprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RelocationRequiredprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RelocationRequiredprotocolIEs(Vec<RelocationRequiredprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RelocationRequiredprotocolExtensions_ItemextensionValue {
    #[asn(key = 203)]
    id_CSG_Id(CSG_Id),
    #[asn(key = 235)]
    id_Cell_Access_Mode(Cell_Access_Mode),
    #[asn(key = 108)]
    id_GERAN_Classmark(GERAN_Classmark),
    #[asn(key = 259)]
    id_RSRVCC_HO_Indication(RSRVCC_HO_Indication),
    #[asn(key = 226)]
    id_SRVCC_HO_Indication(SRVCC_HO_Indication),
    #[asn(key = 161)]
    id_SourceBSS_ToTargetBSS_TransparentContainer(SourceBSS_ToTargetBSS_TransparentContainer),
    #[asn(key = 293)]
    id_UE_Application_Layer_Measurement_Configuration_For_Relocation(
        UE_Application_Layer_Measurement_Configuration_For_Relocation,
    ),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationRequiredprotocolExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RelocationRequiredprotocolExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RelocationRequiredprotocolExtensions(Vec<RelocationRequiredprotocolExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct INTEGER_56(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum Requested_RAB_Parameter_ValuesiE_Extensions_ItemextensionValue {
    #[asn(key = 159)]
    id_AlternativeRABConfigurationRequest(AlternativeRABConfigurationRequest),
    #[asn(key = 179)]
    id_Requested_RAB_Parameter_ExtendedGuaranteedBitrateList(
        Requested_RAB_Parameter_ExtendedGuaranteedBitrateList,
    ),
    #[asn(key = 178)]
    id_Requested_RAB_Parameter_ExtendedMaxBitrateList(
        Requested_RAB_Parameter_ExtendedMaxBitrateList,
    ),
    #[asn(key = 221)]
    id_Requested_RAB_Parameter_SupportedGuaranteedBitrateList(SupportedRAB_ParameterBitrateList),
    #[asn(key = 220)]
    id_Requested_RAB_Parameter_SupportedMaxBitrateList(SupportedRAB_ParameterBitrateList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Requested_RAB_Parameter_ValuesiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: Requested_RAB_Parameter_ValuesiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Requested_RAB_Parameter_ValuesiE_Extensions(
    Vec<Requested_RAB_Parameter_ValuesiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RerouteNASRequestprotocolIEs_Itemvalue {
    #[asn(key = 287)]
    id_P_TMSI(P_TMSI),
    #[asn(key = 286)]
    id_SGSN_Group_Identity(SGSN_Group_Identity),
    #[asn(key = 290)]
    id_UE_Usage_Type(UE_Usage_Type),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RerouteNASRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RerouteNASRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RerouteNASRequestprotocolIEs(Vec<RerouteNASRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RerouteNASRequestprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RerouteNASRequestprotocolExtensions(Vec<RerouteNASRequestprotocolExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ResetprotocolIEs_Itemvalue {
    #[asn(key = 3)]
    id_CN_DomainIndicator(CN_DomainIndicator),
    #[asn(key = 4)]
    id_Cause(Cause),
    #[asn(key = 86)]
    id_GlobalRNC_ID(GlobalRNC_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ResetprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ResetprotocolIEs(Vec<ResetprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ResetprotocolExtensions_ItemextensionValue {
    #[asn(key = 171)]
    id_ExtendedRNC_ID(ExtendedRNC_ID),
    #[asn(key = 96)]
    id_GlobalCN_ID(GlobalCN_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetprotocolExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: ResetprotocolExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ResetprotocolExtensions(Vec<ResetprotocolExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ResetAcknowledgeprotocolIEs_Itemvalue {
    #[asn(key = 3)]
    id_CN_DomainIndicator(CN_DomainIndicator),
    #[asn(key = 9)]
    id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 86)]
    id_GlobalRNC_ID(GlobalRNC_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetAcknowledgeprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ResetAcknowledgeprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ResetAcknowledgeprotocolIEs(Vec<ResetAcknowledgeprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ResetAcknowledgeprotocolExtensions_ItemextensionValue {
    #[asn(key = 171)]
    id_ExtendedRNC_ID(ExtendedRNC_ID),
    #[asn(key = 96)]
    id_GlobalCN_ID(GlobalCN_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetAcknowledgeprotocolExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: ResetAcknowledgeprotocolExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ResetAcknowledgeprotocolExtensions(Vec<ResetAcknowledgeprotocolExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ResetResourceprotocolIEs_Itemvalue {
    #[asn(key = 3)]
    id_CN_DomainIndicator(CN_DomainIndicator),
    #[asn(key = 4)]
    id_Cause(Cause),
    #[asn(key = 86)]
    id_GlobalRNC_ID(GlobalRNC_ID),
    #[asn(key = 77)]
    id_IuSigConIdList(ResetResourceList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetResourceprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ResetResourceprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ResetResourceprotocolIEs(Vec<ResetResourceprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ResetResourceprotocolExtensions_ItemextensionValue {
    #[asn(key = 171)]
    id_ExtendedRNC_ID(ExtendedRNC_ID),
    #[asn(key = 96)]
    id_GlobalCN_ID(GlobalCN_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetResourceprotocolExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: ResetResourceprotocolExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ResetResourceprotocolExtensions(Vec<ResetResourceprotocolExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ResetResourceAckItemiE_Extensions_ItemextensionValue {
    #[asn(key = 282)]
    id_IuSigConIdRangeEnd(IuSignallingConnectionIdentifier),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetResourceAckItemiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: ResetResourceAckItemiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ResetResourceAckItemiE_Extensions(Vec<ResetResourceAckItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ResetResourceAckList_Item_Itemvalue {
    #[asn(key = 78)]
    id_IuSigConIdItem(ResetResourceAckItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetResourceAckList_Item_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ResetResourceAckList_Item_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ResetResourceAckList_Item(Vec<ResetResourceAckList_Item_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ResetResourceAcknowledgeprotocolIEs_Itemvalue {
    #[asn(key = 3)]
    id_CN_DomainIndicator(CN_DomainIndicator),
    #[asn(key = 9)]
    id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 86)]
    id_GlobalRNC_ID(GlobalRNC_ID),
    #[asn(key = 77)]
    id_IuSigConIdList(ResetResourceAckList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetResourceAcknowledgeprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ResetResourceAcknowledgeprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ResetResourceAcknowledgeprotocolIEs(Vec<ResetResourceAcknowledgeprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ResetResourceAcknowledgeprotocolExtensions_ItemextensionValue {
    #[asn(key = 171)]
    id_ExtendedRNC_ID(ExtendedRNC_ID),
    #[asn(key = 96)]
    id_GlobalCN_ID(GlobalCN_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetResourceAcknowledgeprotocolExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: ResetResourceAcknowledgeprotocolExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ResetResourceAcknowledgeprotocolExtensions(
    Vec<ResetResourceAcknowledgeprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ResetResourceItemiE_Extensions_ItemextensionValue {
    #[asn(key = 282)]
    id_IuSigConIdRangeEnd(IuSignallingConnectionIdentifier),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetResourceItemiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: ResetResourceItemiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ResetResourceItemiE_Extensions(Vec<ResetResourceItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ResetResourceList_Item_Itemvalue {
    #[asn(key = 78)]
    id_IuSigConIdItem(ResetResourceItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetResourceList_Item_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ResetResourceList_Item_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ResetResourceList_Item(Vec<ResetResourceList_Item_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "9")]
pub struct INTEGER_57(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "8")]
pub struct INTEGER_58(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResidualBitErrorRatioiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ResidualBitErrorRatioiE_Extensions(Vec<ResidualBitErrorRatioiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SAIiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SAIiE_Extensions(Vec<SAIiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "9")]
pub struct INTEGER_59(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "6")]
pub struct INTEGER_60(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SDU_ErrorRatioiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SDU_ErrorRatioiE_Extensions(Vec<SDU_ErrorRatioiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SDU_FormatInformationParameters_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SDU_FormatInformationParameters_ItemiE_Extensions(
    Vec<SDU_FormatInformationParameters_ItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct SDU_FormatInformationParameters_Item {
    #[asn(optional_idx = 0)]
    pub subflow_sdu_size: Option<SubflowSDU_Size>,
    #[asn(optional_idx = 1)]
    pub r_ab_subflow_combination_bit_rate: Option<RAB_SubflowCombinationBitRate>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<SDU_FormatInformationParameters_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SDU_Parameters_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SDU_Parameters_ItemiE_Extensions(Vec<SDU_Parameters_ItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct SDU_Parameters_Item {
    #[asn(optional_idx = 0)]
    pub s_du_error_ratio: Option<SDU_ErrorRatio>,
    pub residual_bit_error_ratio: ResidualBitErrorRatio,
    pub delivery_of_erroneous_sdu: DeliveryOfErroneousSDU,
    #[asn(optional_idx = 1)]
    pub s_du_format_information_parameters: Option<SDU_FormatInformationParameters>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<SDU_Parameters_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SNA_Access_InformationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SNA_Access_InformationiE_Extensions(Vec<SNA_Access_InformationiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SRB_TrCH_MappingItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SRB_TrCH_MappingItemiE_Extensions(Vec<SRB_TrCH_MappingItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SRNS_ContextRequestprotocolIEs_Itemvalue {
    #[asn(key = 29)]
    id_RAB_DataForwardingList_SRNS_CtxReq(RAB_DataForwardingList_SRNS_CtxReq),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SRNS_ContextRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: SRNS_ContextRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct SRNS_ContextRequestprotocolIEs(Vec<SRNS_ContextRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SRNS_ContextRequestprotocolExtensions_ItemextensionValue {
    #[asn(key = 167)]
    id_RAT_Type(RAT_Type),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SRNS_ContextRequestprotocolExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: SRNS_ContextRequestprotocolExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SRNS_ContextRequestprotocolExtensions(Vec<SRNS_ContextRequestprotocolExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SRNS_ContextResponseprotocolIEs_Itemvalue {
    #[asn(key = 9)]
    id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 85)]
    id_RAB_ContextFailedtoTransferList(RAB_ContextFailedtoTransferList),
    #[asn(key = 25)]
    id_RAB_ContextList(RAB_ContextList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SRNS_ContextResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: SRNS_ContextResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct SRNS_ContextResponseprotocolIEs(Vec<SRNS_ContextResponseprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SRNS_ContextResponseprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SRNS_ContextResponseprotocolExtensions(Vec<SRNS_ContextResponseprotocolExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SRNS_DataForwardCommandprotocolIEs_Itemvalue {
    #[asn(key = 28)]
    id_RAB_DataForwardingList(RAB_DataForwardingList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SRNS_DataForwardCommandprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: SRNS_DataForwardCommandprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct SRNS_DataForwardCommandprotocolIEs(Vec<SRNS_DataForwardCommandprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SRNS_DataForwardCommandprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SRNS_DataForwardCommandprotocolExtensions(
    Vec<SRNS_DataForwardCommandprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SRVCC_CSKeysRequestprotocolIEs_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct SRVCC_CSKeysRequestprotocolIEs(Vec<SRVCC_CSKeysRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SRVCC_CSKeysRequestprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SRVCC_CSKeysRequestprotocolExtensions(Vec<SRVCC_CSKeysRequestprotocolExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SRVCC_CSKeysResponseprotocolIEs_Itemvalue {
    #[asn(key = 9)]
    id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 224)]
    id_EncryptionKey(EncryptionKey),
    #[asn(key = 225)]
    id_IntegrityProtectionKey(IntegrityProtectionKey),
    #[asn(key = 227)]
    id_SRVCC_Information(SRVCC_Information),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SRVCC_CSKeysResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: SRVCC_CSKeysResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct SRVCC_CSKeysResponseprotocolIEs(Vec<SRVCC_CSKeysResponseprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SRVCC_CSKeysResponseprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SRVCC_CSKeysResponseprotocolExtensions(Vec<SRVCC_CSKeysResponseprotocolExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "BITSTRING",
    sz_extensible = false,
    sz_lb = "128",
    sz_ub = "128"
)]
pub struct BIT_STRING_61(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SRVCC_InformationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SRVCC_InformationiE_Extensions(Vec<SRVCC_InformationiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SecurityModeCommandprotocolIEs_Itemvalue {
    #[asn(key = 11)]
    id_EncryptionInformation(EncryptionInformation),
    #[asn(key = 12)]
    id_IntegrityProtectionInformation(IntegrityProtectionInformation),
    #[asn(key = 75)]
    id_KeyStatus(KeyStatus),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecurityModeCommandprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: SecurityModeCommandprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct SecurityModeCommandprotocolIEs(Vec<SecurityModeCommandprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecurityModeCommandprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SecurityModeCommandprotocolExtensions(Vec<SecurityModeCommandprotocolExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SecurityModeCompleteprotocolIEs_Itemvalue {
    #[asn(key = 5)]
    id_ChosenEncryptionAlgorithm(ChosenEncryptionAlgorithm),
    #[asn(key = 6)]
    id_ChosenIntegrityProtectionAlgorithm(ChosenIntegrityProtectionAlgorithm),
    #[asn(key = 9)]
    id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecurityModeCompleteprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: SecurityModeCompleteprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct SecurityModeCompleteprotocolIEs(Vec<SecurityModeCompleteprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecurityModeCompleteprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SecurityModeCompleteprotocolExtensions(Vec<SecurityModeCompleteprotocolExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SecurityModeRejectprotocolIEs_Itemvalue {
    #[asn(key = 4)]
    id_Cause(Cause),
    #[asn(key = 9)]
    id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecurityModeRejectprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: SecurityModeRejectprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct SecurityModeRejectprotocolIEs(Vec<SecurityModeRejectprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecurityModeRejectprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SecurityModeRejectprotocolExtensions(Vec<SecurityModeRejectprotocolExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Shared_Network_InformationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Shared_Network_InformationiE_Extensions(
    Vec<Shared_Network_InformationiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SourceRNC_IDiE_Extensions_ItemextensionValue {
    #[asn(key = 171)]
    id_ExtendedRNC_ID(ExtendedRNC_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SourceRNC_IDiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: SourceRNC_IDiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SourceRNC_IDiE_Extensions(Vec<SourceRNC_IDiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SourceRNC_ToTargetRNC_TransparentContaineriE_Extensions_ItemextensionValue {
    #[asn(key = 237)]
    id_CSFB_Information(CSFB_Information),
    #[asn(key = 121)]
    id_CellLoadInformationGroup(CellLoadInformationGroup),
    #[asn(key = 243)]
    id_IRAT_Measurement_Configuration(IRAT_Measurement_Configuration),
    #[asn(key = 277)]
    id_LastE_UTRANPLMNIdentity(PLMNidentity),
    #[asn(key = 156)]
    id_MBMSLinkingInformation(MBMSLinkingInformation),
    #[asn(key = 249)]
    id_Management_Based_MDT_Allowed(Management_Based_MDT_Allowed),
    #[asn(key = 263)]
    id_Management_Based_MDT_PLMN_List(MDT_PLMN_List),
    #[asn(key = 230)]
    id_PSRABtobeReplaced(RAB_ID),
    #[asn(key = 98)]
    id_SRB_TrCH_Mapping(SRB_TrCH_Mapping),
    #[asn(key = 227)]
    id_SRVCC_Information(SRVCC_Information),
    #[asn(key = 296)]
    id_SRVCCSource(SRVCCSource),
    #[asn(key = 202)]
    id_SubscriberProfileIDforRFP(SubscriberProfileIDforRFP),
    #[asn(key = 124)]
    id_TraceRecordingSessionInformation(TraceRecordingSessionInformation),
    #[asn(key = 200)]
    id_UE_History_Information(UE_History_Information),
    #[asn(key = 187)]
    id_d_RNTI_for_NoIuCSUP(D_RNTI),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SourceRNC_ToTargetRNC_TransparentContaineriE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: SourceRNC_ToTargetRNC_TransparentContaineriE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SourceRNC_ToTargetRNC_TransparentContaineriE_Extensions(
    Vec<SourceRNC_ToTargetRNC_TransparentContaineriE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SourceUTRANCellIDiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SourceUTRANCellIDiE_Extensions(Vec<SourceUTRANCellIDiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SuccessfulOutcomevalue {
    #[asn(key = 7)]
    id_DataVolumeReport(DataVolumeReport),
    #[asn(key = 31)]
    id_InformationTransfer(InformationTransferConfirmation),
    #[asn(key = 1)]
    id_Iu_Release(Iu_ReleaseComplete),
    #[asn(key = 30)]
    id_LocationRelatedData(LocationRelatedDataResponse),
    #[asn(key = 40)]
    id_MBMSCNDe_Registration_Procedure(MBMSCNDe_RegistrationResponse),
    #[asn(key = 42)]
    id_MBMSRABRelease(MBMSRABRelease),
    #[asn(key = 39)]
    id_MBMSRegistration(MBMSRegistrationResponse),
    #[asn(key = 35)]
    id_MBMSSessionStart(MBMSSessionStartResponse),
    #[asn(key = 37)]
    id_MBMSSessionStop(MBMSSessionStopResponse),
    #[asn(key = 36)]
    id_MBMSSessionUpdate(MBMSSessionUpdateResponse),
    #[asn(key = 45)]
    id_RANAPenhancedRelocation(RANAP_EnhancedRelocationInformationResponse),
    #[asn(key = 4)]
    id_RelocationCancel(RelocationCancelAcknowledge),
    #[asn(key = 2)]
    id_RelocationPreparation(RelocationCommand),
    #[asn(key = 3)]
    id_RelocationResourceAllocation(RelocationRequestAcknowledge),
    #[asn(key = 9)]
    id_Reset(ResetAcknowledge),
    #[asn(key = 27)]
    id_ResetResource(ResetResourceAcknowledge),
    #[asn(key = 5)]
    id_SRNS_ContextTransfer(SRNS_ContextResponse),
    #[asn(key = 6)]
    id_SecurityModeControl(SecurityModeComplete),
    #[asn(key = 33)]
    id_UplinkInformationExchange(UplinkInformationExchangeResponse),
    #[asn(key = 43)]
    id_enhancedRelocationComplete(EnhancedRelocationCompleteResponse),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TAIiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TAIiE_Extensions(Vec<TAIiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "3", sz_ub = "3")]
pub struct OCTET_STRING_62(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TMGIiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TMGIiE_Extensions(Vec<TMGIiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TNLInformationEnhRelInfoReqiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TNLInformationEnhRelInfoReqiE_Extensions(
    Vec<TNLInformationEnhRelInfoReqiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TNLInformationEnhRelInfoResiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TNLInformationEnhRelInfoResiE_Extensions(
    Vec<TNLInformationEnhRelInfoResiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargetENB_IDiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TargetENB_IDiE_Extensions(Vec<TargetENB_IDiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TargetRNC_IDiE_Extensions_ItemextensionValue {
    #[asn(key = 171)]
    id_ExtendedRNC_ID(ExtendedRNC_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargetRNC_IDiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: TargetRNC_IDiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TargetRNC_IDiE_Extensions(Vec<TargetRNC_IDiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TargetRNC_ToSourceRNC_TransparentContaineriE_Extensions_ItemextensionValue {
    #[asn(key = 295)]
    id_UeApplicationLayerMeasurementSupportIndication(
        UeApplicationLayerMeasurementSupportIndication,
    ),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargetRNC_ToSourceRNC_TransparentContaineriE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: TargetRNC_ToSourceRNC_TransparentContaineriE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TargetRNC_ToSourceRNC_TransparentContaineriE_Extensions(
    Vec<TargetRNC_ToSourceRNC_TransparentContaineriE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TrCH_IDiE_Extensions_ItemextensionValue {
    #[asn(key = 160)]
    id_E_DCH_MAC_d_Flow_ID(E_DCH_MAC_d_Flow_ID),
    #[asn(key = 117)]
    id_hS_DSCH_MAC_d_Flow_ID(HS_DSCH_MAC_d_Flow_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TrCH_IDiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: TrCH_IDiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TrCH_IDiE_Extensions(Vec<TrCH_IDiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TraceInformationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TraceInformationiE_Extensions(Vec<TraceInformationiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TracePropagationParametersiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TracePropagationParametersiE_Extensions(
    Vec<TracePropagationParametersiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TraceRecordingSessionInformationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TraceRecordingSessionInformationiE_Extensions(
    Vec<TraceRecordingSessionInformationiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TransportLayerInformationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TransportLayerInformationiE_Extensions(Vec<TransportLayerInformationiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TunnelInformationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TunnelInformationiE_Extensions(Vec<TunnelInformationiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "OCTET-STRING",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "1000"
)]
pub struct OCTET_STRING_63(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UE_IsNotServediE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UE_IsNotServediE_Extensions(Vec<UE_IsNotServediE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UE_IsServediE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UE_IsServediE_Extensions(Vec<UE_IsServediE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UESBI_IuiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UESBI_IuiE_Extensions(Vec<UESBI_IuiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UESpecificInformationIndicationprotocolIEs_Itemvalue {
    #[asn(key = 118)]
    id_UESBI_Iu(UESBI_Iu),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UESpecificInformationIndicationprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UESpecificInformationIndicationprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UESpecificInformationIndicationprotocolIEs(
    Vec<UESpecificInformationIndicationprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UESpecificInformationIndicationprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UESpecificInformationIndicationprotocolExtensions(
    Vec<UESpecificInformationIndicationprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UPInformationiE_Extensions_ItemextensionValue {
    #[asn(key = 269)]
    id_TimingDifferenceULDL(TimingDifferenceULDL),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UPInformationiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: UPInformationiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UPInformationiE_Extensions(Vec<UPInformationiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UTRAN_CellIDiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UTRAN_CellIDiE_Extensions(Vec<UTRAN_CellIDiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeRadioCapabilityMatchRequestprotocolIEs_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UeRadioCapabilityMatchRequestprotocolIEs(
    Vec<UeRadioCapabilityMatchRequestprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeRadioCapabilityMatchRequestprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UeRadioCapabilityMatchRequestprotocolExtensions(
    Vec<UeRadioCapabilityMatchRequestprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeRadioCapabilityMatchResponseprotocolIEs_Itemvalue {
    #[asn(key = 258)]
    id_VoiceSupportMatchIndicator(VoiceSupportMatchIndicator),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeRadioCapabilityMatchResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UeRadioCapabilityMatchResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UeRadioCapabilityMatchResponseprotocolIEs(
    Vec<UeRadioCapabilityMatchResponseprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeRadioCapabilityMatchResponseprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UeRadioCapabilityMatchResponseprotocolExtensions(
    Vec<UeRadioCapabilityMatchResponseprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeRegistrationQueryRequestprotocolIEs_Itemvalue {
    #[asn(key = 79)]
    id_IuSigConId(IuSignallingConnectionIdentifier),
    #[asn(key = 23)]
    id_PermanentNAS_UE_ID(PermanentNAS_UE_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeRegistrationQueryRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UeRegistrationQueryRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UeRegistrationQueryRequestprotocolIEs(Vec<UeRegistrationQueryRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeRegistrationQueryRequestprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UeRegistrationQueryRequestprotocolExtensions(
    Vec<UeRegistrationQueryRequestprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeRegistrationQueryResponseprotocolIEs_Itemvalue {
    #[asn(key = 281)]
    id_UERegistrationQueryResult(UERegistrationQueryResult),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeRegistrationQueryResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UeRegistrationQueryResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UeRegistrationQueryResponseprotocolIEs(Vec<UeRegistrationQueryResponseprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeRegistrationQueryResponseprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UeRegistrationQueryResponseprotocolExtensions(
    Vec<UeRegistrationQueryResponseprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UnsuccessfulLinking_IEs_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UnsuccessfulLinking_IEs_ItemiE_Extensions(
    Vec<UnsuccessfulLinking_IEs_ItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UnsuccessfulLinking_IEs_Item {
    pub t_mgi: TMGI,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<UnsuccessfulLinking_IEs_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UnsuccessfulOutcomevalue {
    #[asn(key = 31)]
    id_InformationTransfer(InformationTransferFailure),
    #[asn(key = 30)]
    id_LocationRelatedData(LocationRelatedDataFailure),
    #[asn(key = 42)]
    id_MBMSRABRelease(MBMSRABReleaseFailure),
    #[asn(key = 39)]
    id_MBMSRegistration(MBMSRegistrationFailure),
    #[asn(key = 35)]
    id_MBMSSessionStart(MBMSSessionStartFailure),
    #[asn(key = 36)]
    id_MBMSSessionUpdate(MBMSSessionUpdateFailure),
    #[asn(key = 2)]
    id_RelocationPreparation(RelocationPreparationFailure),
    #[asn(key = 3)]
    id_RelocationResourceAllocation(RelocationFailure),
    #[asn(key = 6)]
    id_SecurityModeControl(SecurityModeReject),
    #[asn(key = 33)]
    id_UplinkInformationExchange(UplinkInformationExchangeFailure),
    #[asn(key = 43)]
    id_enhancedRelocationComplete(EnhancedRelocationCompleteFailure),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkInformationExchangeFailureprotocolIEs_Itemvalue {
    #[asn(key = 3)]
    id_CN_DomainIndicator(CN_DomainIndicator),
    #[asn(key = 4)]
    id_Cause(Cause),
    #[asn(key = 9)]
    id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 96)]
    id_GlobalCN_ID(GlobalCN_ID),
    #[asn(key = 136)]
    id_InformationExchangeID(InformationExchangeID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkInformationExchangeFailureprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UplinkInformationExchangeFailureprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UplinkInformationExchangeFailureprotocolIEs(
    Vec<UplinkInformationExchangeFailureprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkInformationExchangeFailureprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UplinkInformationExchangeFailureprotocolExtensions(
    Vec<UplinkInformationExchangeFailureprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkInformationExchangeRequestprotocolIEs_Itemvalue {
    #[asn(key = 3)]
    id_CN_DomainIndicator(CN_DomainIndicator),
    #[asn(key = 86)]
    id_GlobalRNC_ID(GlobalRNC_ID),
    #[asn(key = 136)]
    id_InformationExchangeID(InformationExchangeID),
    #[asn(key = 137)]
    id_InformationExchangeType(InformationExchangeType),
    #[asn(key = 139)]
    id_InformationRequestType(InformationRequestType),
    #[asn(key = 123)]
    id_InformationTransferType(InformationTransferType),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkInformationExchangeRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UplinkInformationExchangeRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UplinkInformationExchangeRequestprotocolIEs(
    Vec<UplinkInformationExchangeRequestprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkInformationExchangeRequestprotocolExtensions_ItemextensionValue {
    #[asn(key = 171)]
    id_ExtendedRNC_ID(ExtendedRNC_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkInformationExchangeRequestprotocolExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: UplinkInformationExchangeRequestprotocolExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UplinkInformationExchangeRequestprotocolExtensions(
    Vec<UplinkInformationExchangeRequestprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkInformationExchangeResponseprotocolIEs_Itemvalue {
    #[asn(key = 3)]
    id_CN_DomainIndicator(CN_DomainIndicator),
    #[asn(key = 9)]
    id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 96)]
    id_GlobalCN_ID(GlobalCN_ID),
    #[asn(key = 136)]
    id_InformationExchangeID(InformationExchangeID),
    #[asn(key = 138)]
    id_InformationRequested(InformationRequested),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkInformationExchangeResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UplinkInformationExchangeResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UplinkInformationExchangeResponseprotocolIEs(
    Vec<UplinkInformationExchangeResponseprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkInformationExchangeResponseprotocolExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UplinkInformationExchangeResponseprotocolExtensions(
    Vec<UplinkInformationExchangeResponseprotocolExtensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserPlaneInformationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UserPlaneInformationiE_Extensions(Vec<UserPlaneInformationiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct INTEGER_64(u8);

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
