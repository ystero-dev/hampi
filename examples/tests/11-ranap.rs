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
    pub nri: Option<BIT_STRING_2>,
    #[asn(optional_idx = 3)]
    pub ue_is_attaching: Option<NULL_3>,
    #[asn(optional_idx = 4)]
    pub ie_extensions: Option<Additional_CSPS_coordination_informationIE_Extensions>,
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
    pub ie_extensions: Option<AllocationOrRetentionPriorityIE_Extensions>,
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
    pub ie_extensions: Option<Alt_RAB_Parameter_SupportedGuaranteedBitrateInfIE_Extensions>,
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
    pub ie_extensions: Option<Alt_RAB_Parameter_SupportedMaxBitrateInfIE_Extensions>,
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
    pub ie_extensions: Option<Alt_RAB_ParametersIE_Extensions>,
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
    SAI(SAI),
    #[asn(key = 1, extended = false)]
    GeographicalArea(GeographicalArea),
}

#[derive(Debug, AperCodec)]
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
    pub ie_extensions: Option<Ass_RAB_ParametersIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct AuthorisedPLMNs(Vec<AuthorisedPLMNs_Entry>);

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
    pub plm_nidentity: PLMNidentity,
    pub lac: LAC,
    pub ci: CI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CGIIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct CI(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CN_DeactivateTrace {
    pub protocol_i_es: CN_DeactivateTraceProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<CN_DeactivateTraceProtocolExtensions>,
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
    pub protocol_i_es: CN_InvokeTraceProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<CN_InvokeTraceProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CNMBMSLinkingInformation {
    pub joined_mbms_bearer_service_i_es: JoinedMBMSBearerService_IEs,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CNMBMSLinkingInformationIE_Extensions>,
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
    pub ie_extensions: Option<CellBasedIE_Extensions>,
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
    pub rt_load_value: Option<RTLoadValue>,
    #[asn(optional_idx = 1)]
    pub nrt_load_information_value: Option<NRTLoadInformationValue>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<CellLoadInformationIE_Extensions>,
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
    pub ie_extensions: Option<CellLoadInformationGroupIE_Extensions>,
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
    pub protocol_i_es: CommonIDProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<CommonIDProtocolExtensions>,
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
    pub ie_extensions: Option<CriticalityDiagnosticsIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct CriticalityDiagnostics_IE_List(Vec<CriticalityDiagnostics_IE_List_Entry>);

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
pub struct DataVolumeList(Vec<DataVolumeList_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct DataVolumeReference(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DataVolumeReport {
    pub protocol_i_es: DataVolumeReportProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<DataVolumeReportProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DataVolumeReportRequest {
    pub protocol_i_es: DataVolumeReportRequestProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<DataVolumeReportRequestProtocolExtensions>,
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
    pub ra_listwith_no_idle_mode_u_es_any_more: Option<RAListwithNoIdleModeUEsAnyMore>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<DeltaRAListofIdleModeUEsIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DirectInformationTransfer {
    pub protocol_i_es: DirectInformationTransferProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<DirectInformationTransferProtocolExtensions>,
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
    pub protocol_i_es: DirectTransferProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<DirectTransferProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DirectTransferInformationItem_RANAP_RelocInf {
    pub nas_pdu: NAS_PDU,
    pub sapi: SAPI,
    pub cn_domain_indicator: CN_DomainIndicator,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<DirectTransferInformationItem_RANAP_RelocInfIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "15")]
pub struct DirectTransferInformationList_RANAP_RelocInf(
    Vec<DirectTransferInformationList_RANAP_RelocInf_Entry>,
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
    MacroENB_ID(BIT_STRING_7),
    #[asn(key = 1, extended = false)]
    HomeENB_ID(BIT_STRING_8),
    #[asn(key = 0, extended = true)]
    Short_macroENB_ID(BIT_STRING_9),
    #[asn(key = 1, extended = true)]
    Long_macroENB_ID(BIT_STRING_10),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct EUTRANFrequencies(Vec<EUTRANFrequencies_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct EncryptionAlgorithm(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct EncryptionInformation {
    pub permitted_algorithms: PermittedEncryptionAlgorithms,
    pub key: EncryptionKey,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EncryptionInformationIE_Extensions>,
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
    pub protocol_i_es: EnhancedRelocationCompleteConfirmProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<EnhancedRelocationCompleteConfirmProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EnhancedRelocationCompleteFailure {
    pub protocol_i_es: EnhancedRelocationCompleteFailureProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<EnhancedRelocationCompleteFailureProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EnhancedRelocationCompleteRequest {
    pub protocol_i_es: EnhancedRelocationCompleteRequestProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<EnhancedRelocationCompleteRequestProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EnhancedRelocationCompleteResponse {
    pub protocol_i_es: EnhancedRelocationCompleteResponseProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<EnhancedRelocationCompleteResponseProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ErrorIndication {
    pub protocol_i_es: ErrorIndicationProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<ErrorIndicationProtocolExtensions>,
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
    pub protocol_i_es: ForwardSRNS_ContextProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<ForwardSRNS_ContextProtocolExtensions>,
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
    pub ie_extensions: Option<GA_EllipsoidArcIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GA_Point {
    pub geographical_coordinates: GeographicalCoordinates,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GA_PointIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GA_PointWithAltitude {
    pub geographical_coordinates: GeographicalCoordinates,
    pub altitude_and_direction: GA_AltitudeAndDirection,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GA_PointWithAltitudeIE_Extensions>,
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
    pub ie_extensions: Option<GA_PointWithAltitudeAndUncertaintyEllipsoidIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct GA_PointWithUnCertainty {
    pub geographical_coordinates: GeographicalCoordinates,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GA_PointWithUnCertaintyIE_Extensions>,
    pub uncertainty_code: INTEGER_23,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GA_PointWithUnCertaintyEllipse {
    pub geographical_coordinates: GeographicalCoordinates,
    pub uncertainty_ellipse: GA_UncertaintyEllipse,
    pub confidence: INTEGER_24,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GA_PointWithUnCertaintyEllipseIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "15")]
pub struct GA_Polygon(Vec<GA_Polygon_Entry>);

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
    pub lai: LAI,
    pub rac: RAC,
    pub ci: CI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GERAN_Cell_IDIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct GERAN_Classmark(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct GERAN_Iumode_RAB_Failed_RABAssgntResponse_Item {
    pub rab_id: RAB_ID,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub geran_classmark: Option<GERAN_Classmark>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<GERAN_Iumode_RAB_Failed_RABAssgntResponse_ItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct GERAN_Iumode_RAB_FailedList_RABAssgntResponse(
    Vec<GERAN_Iumode_RAB_FailedList_RABAssgntResponse_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct GTP_TEI(Vec<u8>);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GeographicalCoordinates {
    pub latitude_sign: ENUMERATED_28,
    pub latitude: INTEGER_29,
    pub longitude: INTEGER_30,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GeographicalCoordinatesIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalCN_ID {
    pub plm_nidentity: PLMNidentity,
    pub cn_id: CN_ID,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalRNC_ID {
    pub plm_nidentity: PLMNidentity,
    pub rnc_id: RNC_ID,
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
    pub ie_extensions: Option<HorizontalVelocityIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct HorizontalVelocityWithUncertainty {
    pub horizontal_speed_and_bearing: HorizontalSpeedAndBearing,
    pub uncertainty_speed: INTEGER_33,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<HorizontalVelocityWithUncertaintyIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct HorizontalWithVerticalVelocity {
    pub horizontal_speed_and_bearing: HorizontalSpeedAndBearing,
    pub veritcal_velocity: VerticalVelocity,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<HorizontalWithVerticalVelocityIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct HorizontalWithVerticalVelocityAndUncertainty {
    pub horizontal_speed_and_bearing: HorizontalSpeedAndBearing,
    pub veritcal_velocity: VerticalVelocity,
    pub horizontal_uncertainty_speed: INTEGER_34,
    pub vertical_uncertainty_speed: INTEGER_35,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<HorizontalWithVerticalVelocityAndUncertaintyIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct IMEI(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct IMEIGroup {
    pub imei: IMEI,
    pub imei_mask: BIT_STRING_36,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<IMEIGroupIE_Extensions>,
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
    pub imeisv: IMEISV,
    pub imeisv_mask: BIT_STRING_37,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<IMEISVGroupIE_Extensions>,
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
    pub rsrp: Option<INTEGER_38>,
    #[asn(optional_idx = 1)]
    pub rsrq: Option<INTEGER_39>,
    pub ira_tmeasurement_parameters: IRATmeasurementParameters,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<IRAT_Measurement_ConfigurationIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct IRATmeasurementParameters {
    pub measurement_duration: INTEGER_40,
    #[asn(optional_idx = 0)]
    pub eutran_frequencies: Option<EUTRANFrequencies>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<IRATmeasurementParametersIE_Extensions>,
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
    pub ie_extensions: Option<ImmediateMDTIE_Extensions>,
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
    MBMSIPMulticastAddressandAPNRequest(MBMSIPMulticastAddressandAPNRequest),
    #[asn(key = 1, extended = false)]
    PermanentNAS_UE_ID(PermanentNAS_UE_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum InformationRequested {
    #[asn(key = 0, extended = false)]
    RequestedMBMSIPMulticastAddressandAPNRequest(RequestedMBMSIPMulticastAddressandAPNRequest),
    #[asn(key = 1, extended = false)]
    RequestedMulticastServiceList(RequestedMulticastServiceList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct InformationTransferConfirmation {
    pub protocol_i_es: InformationTransferConfirmationProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<InformationTransferConfirmationProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct InformationTransferFailure {
    pub protocol_i_es: InformationTransferFailureProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<InformationTransferFailureProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1048575")]
pub struct InformationTransferID(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct InformationTransferIndication {
    pub protocol_i_es: InformationTransferIndicationProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<InformationTransferIndicationProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "0", extensible = true)]
pub enum InformationTransferType {
    #[asn(key = 0, extended = false)]
    RNCTraceInformation(RNCTraceInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct InitialUE_Message {
    pub protocol_i_es: InitialUE_MessageProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<InitialUE_MessageProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InitiatingMessage {
    #[asn(key_field = true)]
    pub procedure_code: ProcedureCode,
    pub criticality: Criticality,
    pub value: InitiatingMessageValue,
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
    pub ie_extensions: Option<IntegrityProtectionInformationIE_Extensions>,
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
    pub ie_extensions: Option<InterSystemInformation_TransparentContainerIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "0", extensible = true)]
pub enum InterSystemInformationTransferType {
    #[asn(key = 0, extended = false)]
    RIM_Transfer(RIM_Transfer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct InterfacesToTraceItem {
    pub interface: ENUMERATED_41,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<InterfacesToTraceItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Iu_ReleaseCommand {
    pub protocol_i_es: Iu_ReleaseCommandProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<Iu_ReleaseCommandProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Iu_ReleaseComplete {
    pub protocol_i_es: Iu_ReleaseCompleteProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<Iu_ReleaseCompleteProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Iu_ReleaseRequest {
    pub protocol_i_es: Iu_ReleaseRequestProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<Iu_ReleaseRequestProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "24", sz_ub = "24")]
pub struct IuSignallingConnectionIdentifier(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum IuTransportAssociation {
    #[asn(key = 0, extended = false)]
    GTP_TEI(GTP_TEI),
    #[asn(key = 1, extended = false)]
    BindingID(BindingID),
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "128"
)]
pub struct JoinedMBMSBearerService_IEs(Vec<JoinedMBMSBearerService_IEs_Entry>);

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
pub struct LA_LIST(Vec<LA_LIST_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LABased {
    pub lai_list: LAI_List,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<LABasedIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct LAC(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct LAI {
    pub plm_nidentity: PLMNidentity,
    pub lac: LAC,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<LAIIE_Extensions>,
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
    pub sai: SAI,
    pub age_of_sai: INTEGER_42,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<LastKnownServiceAreaIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LastVisitedUTRANCell_Item {
    pub utran_cell_id: UTRAN_CellID,
    pub cell_type: CellType,
    pub time_ue_stayed_in_cell: Time_UE_StayedInCell,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<LastVisitedUTRANCell_ItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "128"
)]
pub struct LeftMBMSBearerService_IEs(Vec<LeftMBMSBearerService_IEs_Entry>);

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
    pub protocol_i_es: LocationRelatedDataFailureProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<LocationRelatedDataFailureProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LocationRelatedDataRequest {
    pub protocol_i_es: LocationRelatedDataRequestProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<LocationRelatedDataRequestProtocolExtensions>,
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
    pub protocol_i_es: LocationRelatedDataResponseProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<LocationRelatedDataResponseProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LocationReport {
    pub protocol_i_es: LocationReportProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<LocationReportProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LocationReportingControl {
    pub protocol_i_es: LocationReportingControlProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<LocationReportingControlProtocolExtensions>,
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
    pub ie_extensions: Option<LocationReportingTransferInformationIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LoggedMDT {
    pub logging_interval: LoggingInterval,
    pub logging_duration: LoggingDuration,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<LoggedMDTIE_Extensions>,
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
    Periodic(MDT_Report_Parameters),
    #[asn(key = 1, extended = false)]
    Event1F(Event1F_Parameters),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum M2Report {
    #[asn(key = 0, extended = false)]
    Periodic(MDT_Report_Parameters),
    #[asn(key = 1, extended = false)]
    Event1I(Event1I_Parameters),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct M4_Collection_Parameters {
    pub m4_period: M4_Period,
    #[asn(optional_idx = 0)]
    pub m4_threshold: Option<M4_Threshold>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<M4_Collection_ParametersIE_Extensions>,
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
    All(NULL_43),
    #[asn(key = 1, extended = false)]
    M4_collection_parameters(M4_Collection_Parameters),
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
    When_available(NULL_44),
    #[asn(key = 1, extended = false)]
    M5_period(M5_Period),
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
    pub ie_extensions: Option<M6ReportIE_Extensions>,
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
    pub ie_extensions: Option<M7ReportIE_Extensions>,
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
    pub protocol_i_es: MBMSCNDe_RegistrationRequestProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSCNDe_RegistrationRequestProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSCNDe_RegistrationResponse {
    pub protocol_i_es: MBMSCNDe_RegistrationResponseProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSCNDe_RegistrationResponseProtocolExtensions>,
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
    pub tmgi: TMGI,
    pub ip_multicast_address: IPMulticastAddress,
    pub apn: APN,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<MBMSIPMulticastAddressandAPNlistIE_Extensions>,
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
    pub protocol_i_es: MBMSRABEstablishmentIndicationProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSRABEstablishmentIndicationProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSRABRelease {
    pub protocol_i_es: MBMSRABReleaseProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSRABReleaseProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSRABReleaseFailure {
    pub protocol_i_es: MBMSRABReleaseFailureProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSRABReleaseFailureProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSRABReleaseRequest {
    pub protocol_i_es: MBMSRABReleaseRequestProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSRABReleaseRequestProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSRegistrationFailure {
    pub protocol_i_es: MBMSRegistrationFailureProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSRegistrationFailureProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSRegistrationRequest {
    pub protocol_i_es: MBMSRegistrationRequestProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSRegistrationRequestProtocolExtensions>,
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
    pub protocol_i_es: MBMSRegistrationResponseProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSRegistrationResponseProtocolExtensions>,
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
    pub protocol_i_es: MBMSSessionStartProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSSessionStartProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSSessionStartFailure {
    pub protocol_i_es: MBMSSessionStartFailureProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSSessionStartFailureProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSSessionStartResponse {
    pub protocol_i_es: MBMSSessionStartResponseProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSSessionStartResponseProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSSessionStop {
    pub protocol_i_es: MBMSSessionStopProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSSessionStopProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSSessionStopResponse {
    pub protocol_i_es: MBMSSessionStopResponseProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSSessionStopResponseProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSSessionUpdate {
    pub protocol_i_es: MBMSSessionUpdateProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSSessionUpdateProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSSessionUpdateFailure {
    pub protocol_i_es: MBMSSessionUpdateFailureProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSSessionUpdateFailureProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSSessionUpdateResponse {
    pub protocol_i_es: MBMSSessionUpdateResponseProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSSessionUpdateResponseProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSSynchronisationInformation {
    pub mbmshc_indicator: MBMSHCIndicator,
    pub ip_multicast_address: IPMulticastAddress,
    pub gtpdlteid: GTP_TEI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<MBMSSynchronisationInformationIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSUELinkingRequest {
    pub protocol_i_es: MBMSUELinkingRequestProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSUELinkingRequestProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSUELinkingResponse {
    pub protocol_i_es: MBMSUELinkingResponseProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<MBMSUELinkingResponseProtocolExtensions>,
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
    pub ie_extensions: Option<MDT_ConfigurationIE_Extensions>,
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
    Cellbased(CellBased),
    #[asn(key = 1, extended = false)]
    Labased(LABased),
    #[asn(key = 2, extended = false)]
    Rabased(RABased),
    #[asn(key = 3, extended = false)]
    Plmn_area_based(NULL_45),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum MDTMode {
    #[asn(key = 0, extended = false)]
    ImmediateMDT(ImmediateMDT),
    #[asn(key = 1, extended = false)]
    LoggedMDT(LoggedMDT),
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
pub struct MessageStructure(Vec<MessageStructure_Entry>);

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
    pub ie_extensions: Option<NotEmptyRAListofIdleModeUEsIE_Extensions>,
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
    pub ie_extensions: Option<Offload_RAB_ParametersIE_Extensions>,
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
    pub value: OutcomeValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Overload {
    pub protocol_i_es: OverloadProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<OverloadProtocolExtensions>,
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
    pub ie_extensions: Option<PLMNBasedIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct PLMNList(Vec<PLMNidentity>);

pub type PLMNidentity = TBCD_STRING;

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct PLMNs_in_shared_network(Vec<PLMNs_in_shared_network_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Paging {
    pub protocol_i_es: PagingProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<PagingProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum PagingAreaID {
    #[asn(key = 0, extended = false)]
    LAI(LAI),
    #[asn(key = 1, extended = false)]
    RAI(RAI),
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
    pub ie_extensions: Option<PeriodicLocationInfoIE_Extensions>,
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
    IMSI(IMSI),
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
    pub ie_extensions: Option<PositionDataIE_Extensions>,
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
    Local(INTEGER_48),
    #[asn(key = 1, extended = false)]
    Global(OBJECT_IDENTIFIER_49),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PrivateMessage {
    pub private_i_es: PrivateMessagePrivateIEs,
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
    Shared_network_information(Shared_Network_Information),
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
    pub protocol_i_es: RAB_AssignmentRequestProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RAB_AssignmentRequestProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_AssignmentResponse {
    pub protocol_i_es: RAB_AssignmentResponseProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RAB_AssignmentResponseProtocolExtensions>,
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
pub struct RAB_ContextFailedtoTransferList(Vec<RAB_ContextFailedtoTransferList_Entry>);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_ContextList(Vec<RAB_ContextList_Entry>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_ContextList_RANAP_RelocInf(Vec<RAB_ContextList_RANAP_RelocInf_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_DataForwardingItem {
    pub rab_id: RAB_ID,
    pub transport_layer_address: TransportLayerAddress,
    pub iu_transport_association: IuTransportAssociation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RAB_DataForwardingItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_DataForwardingItem_SRNS_CtxReq {
    pub rab_id: RAB_ID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RAB_DataForwardingItem_SRNS_CtxReqIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_DataForwardingList(Vec<RAB_DataForwardingList_Entry>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_DataForwardingList_SRNS_CtxReq(Vec<RAB_DataForwardingList_SRNS_CtxReq_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct RAB_DataVolumeReportItem {
    pub rab_id: RAB_ID,
    #[asn(optional_idx = 0)]
    pub dl_unsuccessfully_transmitted_data_volume: Option<DataVolumeList>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<RAB_DataVolumeReportItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_DataVolumeReportList(Vec<RAB_DataVolumeReportList_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_DataVolumeReportRequestItem {
    pub rab_id: RAB_ID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RAB_DataVolumeReportRequestItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_DataVolumeReportRequestList(Vec<RAB_DataVolumeReportRequestList_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_FailedItem {
    pub rab_id: RAB_ID,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RAB_FailedItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_FailedItem_EnhRelocInfoRes {
    pub cn_domain_indicator: CN_DomainIndicator,
    pub rab_id: RAB_ID,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RAB_FailedItem_EnhRelocInfoResIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_FailedList(Vec<RAB_FailedList_Entry>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_FailedList_EnhRelocInfoRes(Vec<RAB_FailedList_EnhRelocInfoRes_Entry>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_FailedtoReportList(Vec<RAB_FailedtoReportList_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct RAB_ID(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_ModifyItem {
    pub rab_id: RAB_ID,
    pub requested_rab_parameter_values: Requested_RAB_Parameter_Values,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RAB_ModifyItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_ModifyList(Vec<RAB_ModifyList_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_ModifyRequest {
    pub protocol_i_es: RAB_ModifyRequestProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RAB_ModifyRequestProtocolExtensions>,
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_QueuedItem {
    pub rab_id: RAB_ID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RAB_QueuedItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_QueuedList(Vec<RAB_QueuedList_Entry>);

pub type RAB_ReleaseFailedList = RAB_FailedList;

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_ReleaseItem {
    pub rab_id: RAB_ID,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RAB_ReleaseItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_ReleaseList(Vec<RAB_ReleaseList_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_ReleaseRequest {
    pub protocol_i_es: RAB_ReleaseRequestProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RAB_ReleaseRequestProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_ReleasedList(Vec<RAB_ReleasedList_Entry>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_ReleasedList_IuRelComp(Vec<RAB_ReleasedList_IuRelComp_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_RelocationReleaseItem {
    pub rab_id: RAB_ID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RAB_RelocationReleaseItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_RelocationReleaseList(Vec<RAB_RelocationReleaseList_Entry>);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_SetupList_EnhRelocInfoReq(Vec<RAB_SetupList_EnhRelocInfoReq_Entry>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_SetupList_EnhRelocInfoRes(Vec<RAB_SetupList_EnhRelocInfoRes_Entry>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_SetupList_EnhancedRelocCompleteReq(
    Vec<RAB_SetupList_EnhancedRelocCompleteReq_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_SetupList_EnhancedRelocCompleteRes(
    Vec<RAB_SetupList_EnhancedRelocCompleteRes_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_SetupList_RelocReq(Vec<RAB_SetupList_RelocReq_Entry>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_SetupList_RelocReqAck(Vec<RAB_SetupList_RelocReqAck_Entry>);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_SetupOrModifiedList(Vec<RAB_SetupOrModifiedList_Entry>);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_SetupOrModifyList(Vec<RAB_SetupOrModifyList_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "16000000")]
pub struct RAB_SubflowCombinationBitRate(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAB_ToBeReleasedItem_EnhancedRelocCompleteRes {
    pub rab_id: RAB_ID,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RAB_ToBeReleasedItem_EnhancedRelocCompleteResIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RAB_ToBeReleasedList_EnhancedRelocCompleteRes(
    Vec<RAB_ToBeReleasedList_EnhancedRelocCompleteRes_Entry>,
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
    pub rab_id: RAB_ID,
    pub tr_ch_id_list: TrCH_ID_List,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RAB_TrCH_MappingItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct RABDataVolumeReport(Vec<RABDataVolumeReport_Entry>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct RABParametersList(Vec<RABParametersList_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RABased {
    pub rai_list: RAI_List,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RABasedIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RABs_ContextFailedtoTransferItem {
    pub rab_id: RAB_ID,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RABs_ContextFailedtoTransferItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RABs_failed_to_reportItem {
    pub rab_id: RAB_ID,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RABs_failed_to_reportItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "1", sz_ub = "1")]
pub struct RAC(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RAI {
    pub lai: LAI,
    pub rac: RAC,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RAIIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct RAI_List(Vec<RAI>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum RAListofIdleModeUEs {
    #[asn(key = 0, extended = false)]
    NotEmptyRAListofIdleModeUEs(NotEmptyRAListofIdleModeUEs),
    #[asn(key = 1, extended = false)]
    EmptyFullRAListofIdleModeUEs(ENUMERATED_50),
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
    pub protocol_i_es: RANAP_EnhancedRelocationInformationRequestProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RANAP_EnhancedRelocationInformationRequestProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RANAP_EnhancedRelocationInformationResponse {
    pub protocol_i_es: RANAP_EnhancedRelocationInformationResponseProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RANAP_EnhancedRelocationInformationResponseProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RANAP_RelocationInformation {
    pub protocol_i_es: RANAP_RelocationInformationProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RANAP_RelocationInformationProtocolExtensions>,
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
    pub rim_information: RIMInformation,
    #[asn(optional_idx = 0)]
    pub rim_routing_address: Option<RIMRoutingAddress>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<RIM_TransferIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct RIMInformation(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum RIMRoutingAddress {
    #[asn(key = 0, extended = false)]
    TargetRNC_ID(TargetRNC_ID),
    #[asn(key = 1, extended = false)]
    GERAN_Cell_ID(GERAN_Cell_ID),
    #[asn(key = 0, extended = true)]
    TargeteNB_ID(TargetENB_ID),
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
    pub ie_extensions: Option<RNCTraceInformationIE_Extensions>,
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
    pub ie_extensions: Option<RNSAPRelocationParametersIE_Extensions>,
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
    pub ims_information: OCTET_STRING_55,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RSRVCC_InformationIE_Extensions>,
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
pub struct RedirectionIndication(Vec<RedirectionIndication_Entry>);

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
    pub protocol_i_es: RelocationCancelProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RelocationCancelProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RelocationCancelAcknowledge {
    pub protocol_i_es: RelocationCancelAcknowledgeProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RelocationCancelAcknowledgeProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RelocationCommand {
    pub protocol_i_es: RelocationCommandProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RelocationCommandProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RelocationComplete {
    pub protocol_i_es: RelocationCompleteProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RelocationCompleteProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RelocationDetect {
    pub protocol_i_es: RelocationDetectProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RelocationDetectProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RelocationFailure {
    pub protocol_i_es: RelocationFailureProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RelocationFailureProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RelocationPreparationFailure {
    pub protocol_i_es: RelocationPreparationFailureProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RelocationPreparationFailureProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RelocationRequest {
    pub protocol_i_es: RelocationRequestProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RelocationRequestProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RelocationRequestAcknowledge {
    pub protocol_i_es: RelocationRequestAcknowledgeProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RelocationRequestAcknowledgeProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RelocationRequired {
    pub protocol_i_es: RelocationRequiredProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RelocationRequiredProtocolExtensions>,
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
    pub ie_extensions: Option<Requested_RAB_Parameter_ValuesIE_Extensions>,
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
    pub protocol_i_es: RerouteNASRequestProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<RerouteNASRequestProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Reset {
    pub protocol_i_es: ResetProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<ResetProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ResetAcknowledge {
    pub protocol_i_es: ResetAcknowledgeProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<ResetAcknowledgeProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ResetResource {
    pub protocol_i_es: ResetResourceProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<ResetResourceProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ResetResourceAckItem {
    pub iu_sig_con_id: IuSignallingConnectionIdentifier,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ResetResourceAckItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "250"
)]
pub struct ResetResourceAckList(Vec<ResetResourceAckList_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ResetResourceAcknowledge {
    pub protocol_i_es: ResetResourceAcknowledgeProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<ResetResourceAcknowledgeProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ResetResourceItem {
    pub iu_sig_con_id: IuSignallingConnectionIdentifier,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ResetResourceItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "250"
)]
pub struct ResetResourceList(Vec<ResetResourceList_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct ResidualBitErrorRatio {
    pub mantissa: INTEGER_57,
    pub exponent: INTEGER_58,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ResidualBitErrorRatioIE_Extensions>,
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
    pub plm_nidentity: PLMNidentity,
    pub lac: LAC,
    pub sac: SAC,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SAIIE_Extensions>,
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
    pub ie_extensions: Option<SDU_ErrorRatioIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct SDU_FormatInformationParameters(Vec<SDU_FormatInformationParameters_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "7")]
pub struct SDU_Parameters(Vec<SDU_Parameters_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct SGSN_Group_ID(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum SGSN_Group_Identity {
    #[asn(key = 0, extended = false)]
    Null_NRI(Null_NRI),
    #[asn(key = 1, extended = false)]
    SGSN_Group_ID(SGSN_Group_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SNA_Access_Information {
    pub authorised_plm_ns: AuthorisedPLMNs,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SNA_Access_InformationIE_Extensions>,
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
    pub srb_id: SRB_ID,
    pub tr_ch_id: TrCH_ID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SRB_TrCH_MappingItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SRNS_ContextRequest {
    pub protocol_i_es: SRNS_ContextRequestProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<SRNS_ContextRequestProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SRNS_ContextResponse {
    pub protocol_i_es: SRNS_ContextResponseProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<SRNS_ContextResponseProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SRNS_DataForwardCommand {
    pub protocol_i_es: SRNS_DataForwardCommandProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<SRNS_DataForwardCommandProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SRVCC_CSKeysRequest {
    pub protocol_i_es: SRVCC_CSKeysRequestProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<SRVCC_CSKeysRequestProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SRVCC_CSKeysResponse {
    pub protocol_i_es: SRVCC_CSKeysResponseProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<SRVCC_CSKeysResponseProtocolExtensions>,
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
    pub ie_extensions: Option<SRVCC_InformationIE_Extensions>,
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
    pub protocol_i_es: SecurityModeCommandProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<SecurityModeCommandProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SecurityModeComplete {
    pub protocol_i_es: SecurityModeCompleteProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<SecurityModeCompleteProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SecurityModeReject {
    pub protocol_i_es: SecurityModeRejectProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<SecurityModeRejectProtocolExtensions>,
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
    pub plm_ns_in_shared_network: PLMNs_in_shared_network,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<Shared_Network_InformationIE_Extensions>,
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
    SourceUTRANCellID(SourceUTRANCellID),
    #[asn(key = 1, extended = false)]
    SourceGERANCellID(CGI),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum SourceID {
    #[asn(key = 0, extended = false)]
    SourceRNC_ID(SourceRNC_ID),
    #[asn(key = 1, extended = false)]
    SAI(SAI),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct SourceRNC_ID {
    pub plm_nidentity: PLMNidentity,
    pub rnc_id: RNC_ID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SourceRNC_IDIE_Extensions>,
}

#[derive(Debug, AperCodec)]
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
    pub plm_nidentity: PLMNidentity,
    pub utra_ncell_id: TargetCellId,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SourceUTRANCellIDIE_Extensions>,
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
    pub value: SuccessfulOutcomeValue,
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
    pub plm_nidentity: PLMNidentity,
    pub tac: TAC,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TAIIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct TBCD_STRING(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct TMGI {
    pub plm_nidentity: PLMNidentity,
    pub service_id: OCTET_STRING_62,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TMGIIE_Extensions>,
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
    pub ie_extensions: Option<TNLInformationEnhRelInfoReqIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TNLInformationEnhRelInfoRes {
    pub dl_forwarding_transport_layer_address: TransportLayerAddress,
    pub dl_forwarding_transport_association: IuTransportAssociation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TNLInformationEnhRelInfoResIE_Extensions>,
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
    pub plm_nidentity: PLMNidentity,
    pub enb_id: ENB_ID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TargetENB_IDIE_Extensions>,
    pub selected_tai: TAI,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum TargetID {
    #[asn(key = 0, extended = false)]
    TargetRNC_ID(TargetRNC_ID),
    #[asn(key = 1, extended = false)]
    CGI(CGI),
    #[asn(key = 0, extended = true)]
    TargeteNB_ID(TargetENB_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct TargetRNC_ID {
    pub lai: LAI,
    #[asn(optional_idx = 0)]
    pub rac: Option<RAC>,
    pub rnc_id: RNC_ID,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<TargetRNC_IDIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct TargetRNC_ToSourceRNC_TransparentContainer {
    pub rrc_container: RRC_Container,
    #[asn(optional_idx = 0)]
    pub d_rnti: Option<D_RNTI>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<TargetRNC_ToSourceRNC_TransparentContainerIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct TargeteNodeB_ToSourceeNodeB_TransparentContainer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum TemporaryUE_ID {
    #[asn(key = 0, extended = false)]
    TMSI(TMSI),
    #[asn(key = 1, extended = false)]
    P_TMSI(P_TMSI),
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
    pub dch_id: Option<DCH_ID>,
    #[asn(optional_idx = 1)]
    pub dsch_id: Option<DSCH_ID>,
    #[asn(optional_idx = 2)]
    pub usch_id: Option<USCH_ID>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<TrCH_IDIE_Extensions>,
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
    pub ie_extensions: Option<TraceInformationIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct TracePropagationParameters {
    pub trace_recording_session_reference: TraceRecordingSessionReference,
    pub trace_depth: TraceDepth,
    #[asn(optional_idx = 0)]
    pub list_of_interfaces_to_trace: Option<ListOfInterfacesToTrace>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<TracePropagationParametersIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TraceRecordingSessionInformation {
    pub trace_reference: TraceReference,
    pub trace_recording_session_reference: TraceRecordingSessionReference,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TraceRecordingSessionInformationIE_Extensions>,
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
    pub ie_extensions: Option<TransportLayerInformationIE_Extensions>,
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
    pub udp_port_number: Option<Port_Number>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<TunnelInformationIE_Extensions>,
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
    pub ue_aggregate_maximum_bit_rate_downlink: Option<UE_AggregateMaximumBitRateDownlink>,
    #[asn(optional_idx = 1)]
    pub ue_aggregate_maximum_bit_rate_uplink: Option<UE_AggregateMaximumBitRateUplink>,
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
    Imsi(IMSI),
    #[asn(key = 1, extended = false)]
    Imei(IMEI),
    #[asn(key = 0, extended = true)]
    Imeisv(IMEISV),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UE_IsNotServed {
    pub permanent_nas_ue_id: PermanentNAS_UE_ID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UE_IsNotServedIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UE_IsServed {
    pub permanent_nas_ue_id: PermanentNAS_UE_ID,
    pub plm_nidentity: PLMNidentity,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UE_IsServedIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct UE_Usage_Type(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum UERegistrationQueryResult {
    #[asn(key = 0, extended = false)]
    UE_IsServed(UE_IsServed),
    #[asn(key = 1, extended = false)]
    UE_IsNotServed(UE_IsNotServed),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct UESBI_Iu {
    #[asn(optional_idx = 0)]
    pub uesbi_iu_a: Option<UESBI_IuA>,
    #[asn(optional_idx = 1)]
    pub uesbi_iu_b: Option<UESBI_IuB>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<UESBI_IuIE_Extensions>,
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
    pub protocol_i_es: UESpecificInformationIndicationProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<UESpecificInformationIndicationProtocolExtensions>,
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
    pub ie_extensions: Option<UPInformationIE_Extensions>,
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
    pub plm_nidentity: PLMNidentity,
    pub cell_id: TargetCellId,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UTRAN_CellIDIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct UeApplicationLayerMeasurementSupportIndication(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UeRadioCapabilityMatchRequest {
    pub protocol_i_es: UeRadioCapabilityMatchRequestProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<UeRadioCapabilityMatchRequestProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UeRadioCapabilityMatchResponse {
    pub protocol_i_es: UeRadioCapabilityMatchResponseProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<UeRadioCapabilityMatchResponseProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UeRegistrationQueryRequest {
    pub protocol_i_es: UeRegistrationQueryRequestProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<UeRegistrationQueryRequestProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UeRegistrationQueryResponse {
    pub protocol_i_es: UeRegistrationQueryResponseProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<UeRegistrationQueryResponseProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "128"
)]
pub struct UnsuccessfulLinking_IEs(Vec<UnsuccessfulLinking_IEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UnsuccessfulOutcome {
    #[asn(key_field = true)]
    pub procedure_code: ProcedureCode,
    pub criticality: Criticality,
    pub value: UnsuccessfulOutcomeValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4294967295")]
pub struct UnsuccessfullyTransmittedDataVolume(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UplinkInformationExchangeFailure {
    pub protocol_i_es: UplinkInformationExchangeFailureProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<UplinkInformationExchangeFailureProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UplinkInformationExchangeRequest {
    pub protocol_i_es: UplinkInformationExchangeRequestProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<UplinkInformationExchangeRequestProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UplinkInformationExchangeResponse {
    pub protocol_i_es: UplinkInformationExchangeResponseProtocolIEs,
    #[asn(optional_idx = 0)]
    pub protocol_extensions: Option<UplinkInformationExchangeResponseProtocolExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UserPlaneInformation {
    pub user_plane_mode: UserPlaneMode,
    pub up_mode_versions: UP_ModeVersions,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UserPlaneInformationIE_Extensions>,
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
    HorizontalVelocity(HorizontalVelocity),
    #[asn(key = 1, extended = false)]
    HorizontalWithVerticalVelocity(HorizontalWithVerticalVelocity),
    #[asn(key = 2, extended = false)]
    HorizontalVelocityWithUncertainty(HorizontalVelocityWithUncertainty),
    #[asn(key = 3, extended = false)]
    HorizontalWithVeritcalVelocityAndUncertainty(HorizontalWithVerticalVelocityAndUncertainty),
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
pub struct Additional_CSPS_coordination_informationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Additional_CSPS_coordination_informationIE_Extensions(
    Vec<Additional_CSPS_coordination_informationIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AllocationOrRetentionPriorityIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AllocationOrRetentionPriorityIE_Extensions(
    Vec<AllocationOrRetentionPriorityIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Alt_RAB_Parameter_SupportedGuaranteedBitrateInfIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Alt_RAB_Parameter_SupportedGuaranteedBitrateInfIE_Extensions(
    Vec<Alt_RAB_Parameter_SupportedGuaranteedBitrateInfIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Alt_RAB_Parameter_SupportedMaxBitrateInfIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Alt_RAB_Parameter_SupportedMaxBitrateInfIE_Extensions(
    Vec<Alt_RAB_Parameter_SupportedMaxBitrateInfIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Alt_RAB_ParametersIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: Alt_RAB_ParametersIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Alt_RAB_ParametersIE_Extensions(Vec<Alt_RAB_ParametersIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Ass_RAB_ParametersIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: Ass_RAB_ParametersIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Ass_RAB_ParametersIE_Extensions(Vec<Ass_RAB_ParametersIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AuthorisedPLMNs_EntryIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AuthorisedPLMNs_EntryIE_Extensions(Vec<AuthorisedPLMNs_EntryIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct AuthorisedPLMNs_Entry {
    pub plm_nidentity: PLMNidentity,
    #[asn(optional_idx = 0)]
    pub authorised_sn_as_list: Option<AuthorisedSNAs>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<AuthorisedPLMNs_EntryIE_Extensions>,
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
pub enum CGIIE_Extensions_EntryExtensionValue {
    #[asn(key = 55)]
    Id_RAC(RAC),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CGIIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: CGIIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CGIIE_Extensions(Vec<CGIIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum CN_DeactivateTraceProtocolIEs_EntryValue {
    #[asn(key = 65)]
    Id_TraceReference(TraceReference),
    #[asn(key = 68)]
    Id_TriggerID(TriggerID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CN_DeactivateTraceProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: CN_DeactivateTraceProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct CN_DeactivateTraceProtocolIEs(Vec<CN_DeactivateTraceProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CN_DeactivateTraceProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CN_DeactivateTraceProtocolExtensions(Vec<CN_DeactivateTraceProtocolExtensions_Entry>);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CN_InvokeTraceProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: CN_InvokeTraceProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct CN_InvokeTraceProtocolIEs(Vec<CN_InvokeTraceProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CN_InvokeTraceProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: CN_InvokeTraceProtocolExtensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CN_InvokeTraceProtocolExtensions(Vec<CN_InvokeTraceProtocolExtensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CNMBMSLinkingInformationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CNMBMSLinkingInformationIE_Extensions(Vec<CNMBMSLinkingInformationIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellBasedIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellBasedIE_Extensions(Vec<CellBasedIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellLoadInformationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellLoadInformationIE_Extensions(Vec<CellLoadInformationIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellLoadInformationGroupIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellLoadInformationGroupIE_Extensions(Vec<CellLoadInformationGroupIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum CommonIDProtocolIEs_EntryValue {
    #[asn(key = 23)]
    Id_PermanentNAS_UE_ID(PermanentNAS_UE_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CommonIDProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: CommonIDProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct CommonIDProtocolIEs(Vec<CommonIDProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CommonIDProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: CommonIDProtocolExtensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CommonIDProtocolExtensions(Vec<CommonIDProtocolExtensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CriticalityDiagnosticsIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CriticalityDiagnosticsIE_Extensions(Vec<CriticalityDiagnosticsIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum CriticalityDiagnostics_IE_List_EntryIE_Extensions_EntryExtensionValue {
    #[asn(key = 88)]
    Id_MessageStructure(MessageStructure),
    #[asn(key = 93)]
    Id_TypeOfError(TypeOfError),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CriticalityDiagnostics_IE_List_EntryIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: CriticalityDiagnostics_IE_List_EntryIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CriticalityDiagnostics_IE_List_EntryIE_Extensions(
    Vec<CriticalityDiagnostics_IE_List_EntryIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct CriticalityDiagnostics_IE_List_Entry {
    pub ie_criticality: Criticality,
    pub ie_id: ProtocolIE_ID,
    #[asn(optional_idx = 0)]
    pub repetition_number: Option<RepetitionNumber0>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<CriticalityDiagnostics_IE_List_EntryIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DataVolumeList_EntryIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DataVolumeList_EntryIE_Extensions(Vec<DataVolumeList_EntryIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct DataVolumeList_Entry {
    pub dl_unsuccessfully_transmitted_data_volume: UnsuccessfullyTransmittedDataVolume,
    #[asn(optional_idx = 0)]
    pub data_volume_reference: Option<DataVolumeReference>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<DataVolumeList_EntryIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DataVolumeReportProtocolIEs_EntryValue {
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 31)]
    Id_RAB_DataVolumeReportList(RAB_DataVolumeReportList),
    #[asn(key = 72)]
    Id_RAB_FailedtoReportList(RAB_FailedtoReportList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DataVolumeReportProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: DataVolumeReportProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct DataVolumeReportProtocolIEs(Vec<DataVolumeReportProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DataVolumeReportProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DataVolumeReportProtocolExtensions(Vec<DataVolumeReportProtocolExtensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DataVolumeReportRequestProtocolIEs_EntryValue {
    #[asn(key = 33)]
    Id_RAB_DataVolumeReportRequestList(RAB_DataVolumeReportRequestList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DataVolumeReportRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: DataVolumeReportRequestProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct DataVolumeReportRequestProtocolIEs(Vec<DataVolumeReportRequestProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DataVolumeReportRequestProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DataVolumeReportRequestProtocolExtensions(
    Vec<DataVolumeReportRequestProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DeltaRAListofIdleModeUEsIE_Extensions_EntryExtensionValue {
    #[asn(key = 182)]
    Id_LAListwithNoIdleModeUEsAnyMore(LAListofIdleModeUEs),
    #[asn(key = 181)]
    Id_newLAListofIdleModeUEs(LAListofIdleModeUEs),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DeltaRAListofIdleModeUEsIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: DeltaRAListofIdleModeUEsIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DeltaRAListofIdleModeUEsIE_Extensions(Vec<DeltaRAListofIdleModeUEsIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DirectInformationTransferProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: DirectInformationTransferProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct DirectInformationTransferProtocolIEs(Vec<DirectInformationTransferProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DirectInformationTransferProtocolExtensions_EntryExtensionValue {
    #[asn(key = 171)]
    Id_ExtendedRNC_ID(ExtendedRNC_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DirectInformationTransferProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: DirectInformationTransferProtocolExtensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DirectInformationTransferProtocolExtensions(
    Vec<DirectInformationTransferProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DirectTransferProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: DirectTransferProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct DirectTransferProtocolIEs(Vec<DirectTransferProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DirectTransferProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: DirectTransferProtocolExtensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DirectTransferProtocolExtensions(Vec<DirectTransferProtocolExtensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DirectTransferInformationItem_RANAP_RelocInfIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DirectTransferInformationItem_RANAP_RelocInfIE_Extensions(
    Vec<DirectTransferInformationItem_RANAP_RelocInfIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DirectTransferInformationList_RANAP_RelocInf_Entry_EntryValue {
    #[asn(key = 80)]
    Id_DirectTransferInformationItem_RANAP_RelocInf(DirectTransferInformationItem_RANAP_RelocInf),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DirectTransferInformationList_RANAP_RelocInf_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: DirectTransferInformationList_RANAP_RelocInf_Entry_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct DirectTransferInformationList_RANAP_RelocInf_Entry(
    Vec<DirectTransferInformationList_RANAP_RelocInf_Entry_Entry>,
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
pub enum EUTRANFrequencies_EntryIE_Extensions_EntryExtensionValue {
    #[asn(key = 271)]
    Id_EARFCN_Extended(EARFCN_Extended),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EUTRANFrequencies_EntryIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: EUTRANFrequencies_EntryIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EUTRANFrequencies_EntryIE_Extensions(Vec<EUTRANFrequencies_EntryIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct EUTRANFrequencies_Entry {
    pub earfcn: INTEGER_11,
    #[asn(optional_idx = 0)]
    pub meas_band: Option<MeasBand>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<EUTRANFrequencies_EntryIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EncryptionInformationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EncryptionInformationIE_Extensions(Vec<EncryptionInformationIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum EnhancedRelocationCompleteConfirmProtocolIEs_EntryValue {
    #[asn(key = 35)]
    Id_RAB_FailedList(RAB_FailedList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EnhancedRelocationCompleteConfirmProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: EnhancedRelocationCompleteConfirmProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct EnhancedRelocationCompleteConfirmProtocolIEs(
    Vec<EnhancedRelocationCompleteConfirmProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EnhancedRelocationCompleteConfirmProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EnhancedRelocationCompleteConfirmProtocolExtensions(
    Vec<EnhancedRelocationCompleteConfirmProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum EnhancedRelocationCompleteFailureProtocolIEs_EntryValue {
    #[asn(key = 4)]
    Id_Cause(Cause),
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EnhancedRelocationCompleteFailureProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: EnhancedRelocationCompleteFailureProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct EnhancedRelocationCompleteFailureProtocolIEs(
    Vec<EnhancedRelocationCompleteFailureProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EnhancedRelocationCompleteFailureProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EnhancedRelocationCompleteFailureProtocolExtensions(
    Vec<EnhancedRelocationCompleteFailureProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EnhancedRelocationCompleteRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: EnhancedRelocationCompleteRequestProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct EnhancedRelocationCompleteRequestProtocolIEs(
    Vec<EnhancedRelocationCompleteRequestProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EnhancedRelocationCompleteRequestProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: EnhancedRelocationCompleteRequestProtocolExtensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EnhancedRelocationCompleteRequestProtocolExtensions(
    Vec<EnhancedRelocationCompleteRequestProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum EnhancedRelocationCompleteResponseProtocolIEs_EntryValue {
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 190)]
    Id_RAB_SetupList_EnhancedRelocCompleteRes(RAB_SetupList_EnhancedRelocCompleteRes),
    #[asn(key = 210)]
    Id_RAB_ToBeReleasedList_EnhancedRelocCompleteRes(RAB_ToBeReleasedList_EnhancedRelocCompleteRes),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EnhancedRelocationCompleteResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: EnhancedRelocationCompleteResponseProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct EnhancedRelocationCompleteResponseProtocolIEs(
    Vec<EnhancedRelocationCompleteResponseProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum EnhancedRelocationCompleteResponseProtocolExtensions_EntryExtensionValue {
    #[asn(key = 234)]
    Id_CSG_Membership_Status(CSG_Membership_Status),
    #[asn(key = 239)]
    Id_MSISDN(MSISDN),
    #[asn(key = 233)]
    Id_UE_AggregateMaximumBitRate(UE_AggregateMaximumBitRate),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EnhancedRelocationCompleteResponseProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: EnhancedRelocationCompleteResponseProtocolExtensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EnhancedRelocationCompleteResponseProtocolExtensions(
    Vec<EnhancedRelocationCompleteResponseProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ErrorIndicationProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ErrorIndicationProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ErrorIndicationProtocolIEs(Vec<ErrorIndicationProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ErrorIndicationProtocolExtensions_EntryExtensionValue {
    #[asn(key = 171)]
    Id_ExtendedRNC_ID(ExtendedRNC_ID),
    #[asn(key = 96)]
    Id_GlobalCN_ID(GlobalCN_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ErrorIndicationProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: ErrorIndicationProtocolExtensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ErrorIndicationProtocolExtensions(Vec<ErrorIndicationProtocolExtensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "-120", ub = "165")]
pub struct INTEGER_12(i8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "-120", ub = "-25")]
pub struct INTEGER_13(i8);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ForwardSRNS_ContextProtocolIEs_EntryValue {
    #[asn(key = 25)]
    Id_RAB_ContextList(RAB_ContextList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ForwardSRNS_ContextProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ForwardSRNS_ContextProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ForwardSRNS_ContextProtocolIEs(Vec<ForwardSRNS_ContextProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ForwardSRNS_ContextProtocolExtensions_EntryExtensionValue {
    #[asn(key = 103)]
    Id_SourceRNC_PDCP_context_info(RRC_Container),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ForwardSRNS_ContextProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: ForwardSRNS_ContextProtocolExtensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ForwardSRNS_ContextProtocolExtensions(Vec<ForwardSRNS_ContextProtocolExtensions_Entry>);

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
pub struct GA_EllipsoidArcIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GA_EllipsoidArcIE_Extensions(Vec<GA_EllipsoidArcIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GA_PointIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GA_PointIE_Extensions(Vec<GA_PointIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GA_PointWithAltitudeIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GA_PointWithAltitudeIE_Extensions(Vec<GA_PointWithAltitudeIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct INTEGER_21(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct INTEGER_22(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GA_PointWithAltitudeAndUncertaintyEllipsoidIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GA_PointWithAltitudeAndUncertaintyEllipsoidIE_Extensions(
    Vec<GA_PointWithAltitudeAndUncertaintyEllipsoidIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GA_PointWithUnCertaintyIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GA_PointWithUnCertaintyIE_Extensions(Vec<GA_PointWithUnCertaintyIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct INTEGER_23(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct INTEGER_24(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GA_PointWithUnCertaintyEllipseIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GA_PointWithUnCertaintyEllipseIE_Extensions(
    Vec<GA_PointWithUnCertaintyEllipseIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GA_Polygon_EntryIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GA_Polygon_EntryIE_Extensions(Vec<GA_Polygon_EntryIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GA_Polygon_Entry {
    pub geographical_coordinates: GeographicalCoordinates,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GA_Polygon_EntryIE_Extensions>,
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
pub struct GERAN_Cell_IDIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GERAN_Cell_IDIE_Extensions(Vec<GERAN_Cell_IDIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GERAN_Iumode_RAB_Failed_RABAssgntResponse_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GERAN_Iumode_RAB_Failed_RABAssgntResponse_ItemIE_Extensions(
    Vec<GERAN_Iumode_RAB_Failed_RABAssgntResponse_ItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum GERAN_Iumode_RAB_FailedList_RABAssgntResponse_Entry_EntryValue {
    #[asn(key = 109)]
    Id_GERAN_Iumode_RAB_Failed_RABAssgntResponse_Item(
        GERAN_Iumode_RAB_Failed_RABAssgntResponse_Item,
    ),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GERAN_Iumode_RAB_FailedList_RABAssgntResponse_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: GERAN_Iumode_RAB_FailedList_RABAssgntResponse_Entry_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct GERAN_Iumode_RAB_FailedList_RABAssgntResponse_Entry(
    Vec<GERAN_Iumode_RAB_FailedList_RABAssgntResponse_Entry_Entry>,
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
pub struct GeographicalCoordinatesIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GeographicalCoordinatesIE_Extensions(Vec<GeographicalCoordinatesIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "359")]
pub struct INTEGER_31(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "2047")]
pub struct INTEGER_32(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HorizontalVelocityIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct HorizontalVelocityIE_Extensions(Vec<HorizontalVelocityIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct INTEGER_33(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HorizontalVelocityWithUncertaintyIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct HorizontalVelocityWithUncertaintyIE_Extensions(
    Vec<HorizontalVelocityWithUncertaintyIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HorizontalWithVerticalVelocityIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct HorizontalWithVerticalVelocityIE_Extensions(
    Vec<HorizontalWithVerticalVelocityIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct INTEGER_34(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct INTEGER_35(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HorizontalWithVerticalVelocityAndUncertaintyIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct HorizontalWithVerticalVelocityAndUncertaintyIE_Extensions(
    Vec<HorizontalWithVerticalVelocityAndUncertaintyIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "7", sz_ub = "7")]
pub struct BIT_STRING_36(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IMEIGroupIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct IMEIGroupIE_Extensions(Vec<IMEIGroupIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "7", sz_ub = "7")]
pub struct BIT_STRING_37(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IMEISVGroupIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct IMEISVGroupIE_Extensions(Vec<IMEISVGroupIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "97")]
pub struct INTEGER_38(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "34")]
pub struct INTEGER_39(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum IRAT_Measurement_ConfigurationIE_Extensions_EntryExtensionValue {
    #[asn(key = 279)]
    Id_RSRQ_Extension(RSRQ_Extension),
    #[asn(key = 278)]
    Id_RSRQ_Type(RSRQ_Type),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IRAT_Measurement_ConfigurationIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: IRAT_Measurement_ConfigurationIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct IRAT_Measurement_ConfigurationIE_Extensions(
    Vec<IRAT_Measurement_ConfigurationIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "100")]
pub struct INTEGER_40(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IRATmeasurementParametersIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct IRATmeasurementParametersIE_Extensions(
    Vec<IRATmeasurementParametersIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ImmediateMDTIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: ImmediateMDTIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ImmediateMDTIE_Extensions(Vec<ImmediateMDTIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InformationTransferConfirmationProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: InformationTransferConfirmationProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct InformationTransferConfirmationProtocolIEs(
    Vec<InformationTransferConfirmationProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum InformationTransferConfirmationProtocolExtensions_EntryExtensionValue {
    #[asn(key = 171)]
    Id_ExtendedRNC_ID(ExtendedRNC_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InformationTransferConfirmationProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: InformationTransferConfirmationProtocolExtensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct InformationTransferConfirmationProtocolExtensions(
    Vec<InformationTransferConfirmationProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InformationTransferFailureProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: InformationTransferFailureProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct InformationTransferFailureProtocolIEs(Vec<InformationTransferFailureProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum InformationTransferFailureProtocolExtensions_EntryExtensionValue {
    #[asn(key = 171)]
    Id_ExtendedRNC_ID(ExtendedRNC_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InformationTransferFailureProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: InformationTransferFailureProtocolExtensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct InformationTransferFailureProtocolExtensions(
    Vec<InformationTransferFailureProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InformationTransferIndicationProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: InformationTransferIndicationProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct InformationTransferIndicationProtocolIEs(
    Vec<InformationTransferIndicationProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InformationTransferIndicationProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct InformationTransferIndicationProtocolExtensions(
    Vec<InformationTransferIndicationProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InitialUE_MessageProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: InitialUE_MessageProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct InitialUE_MessageProtocolIEs(Vec<InitialUE_MessageProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InitialUE_MessageProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: InitialUE_MessageProtocolExtensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct InitialUE_MessageProtocolExtensions(Vec<InitialUE_MessageProtocolExtensions_Entry>);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IntegrityProtectionInformationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct IntegrityProtectionInformationIE_Extensions(
    Vec<IntegrityProtectionInformationIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InterSystemInformation_TransparentContainerIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct InterSystemInformation_TransparentContainerIE_Extensions(
    Vec<InterSystemInformation_TransparentContainerIE_Extensions_Entry>,
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
pub struct InterfacesToTraceItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct InterfacesToTraceItemIE_Extensions(Vec<InterfacesToTraceItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum Iu_ReleaseCommandProtocolIEs_EntryValue {
    #[asn(key = 4)]
    Id_Cause(Cause),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Iu_ReleaseCommandProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: Iu_ReleaseCommandProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct Iu_ReleaseCommandProtocolIEs(Vec<Iu_ReleaseCommandProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum Iu_ReleaseCommandProtocolExtensions_EntryExtensionValue {
    #[asn(key = 252)]
    Id_End_Of_CSFB(End_Of_CSFB),
    #[asn(key = 277)]
    Id_LastE_UTRANPLMNIdentity(PLMNidentity),
    #[asn(key = 254)]
    Id_Out_Of_UTRAN(Out_Of_UTRAN),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Iu_ReleaseCommandProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: Iu_ReleaseCommandProtocolExtensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Iu_ReleaseCommandProtocolExtensions(Vec<Iu_ReleaseCommandProtocolExtensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum Iu_ReleaseCompleteProtocolIEs_EntryValue {
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 31)]
    Id_RAB_DataVolumeReportList(RAB_DataVolumeReportList),
    #[asn(key = 44)]
    Id_RAB_ReleasedList_IuRelComp(RAB_ReleasedList_IuRelComp),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Iu_ReleaseCompleteProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: Iu_ReleaseCompleteProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct Iu_ReleaseCompleteProtocolIEs(Vec<Iu_ReleaseCompleteProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Iu_ReleaseCompleteProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Iu_ReleaseCompleteProtocolExtensions(Vec<Iu_ReleaseCompleteProtocolExtensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum Iu_ReleaseRequestProtocolIEs_EntryValue {
    #[asn(key = 4)]
    Id_Cause(Cause),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Iu_ReleaseRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: Iu_ReleaseRequestProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct Iu_ReleaseRequestProtocolIEs(Vec<Iu_ReleaseRequestProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Iu_ReleaseRequestProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Iu_ReleaseRequestProtocolExtensions(Vec<Iu_ReleaseRequestProtocolExtensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct JoinedMBMSBearerService_IEs_EntryIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct JoinedMBMSBearerService_IEs_EntryIE_Extensions(
    Vec<JoinedMBMSBearerService_IEs_EntryIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct JoinedMBMSBearerService_IEs_Entry {
    pub tmgi: TMGI,
    pub mbms_ptp_rab_id: MBMS_PTP_RAB_ID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<JoinedMBMSBearerService_IEs_EntryIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LA_LIST_EntryIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LA_LIST_EntryIE_Extensions(Vec<LA_LIST_EntryIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LA_LIST_Entry {
    pub lac: LAC,
    pub list_of_sn_as: ListOF_SNAs,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<LA_LIST_EntryIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LABasedIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LABasedIE_Extensions(Vec<LABasedIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LAIIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LAIIE_Extensions(Vec<LAIIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "32767")]
pub struct INTEGER_42(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LastKnownServiceAreaIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LastKnownServiceAreaIE_Extensions(Vec<LastKnownServiceAreaIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum LastVisitedUTRANCell_ItemIE_Extensions_EntryExtensionValue {
    #[asn(key = 257)]
    Id_HO_Cause(Cause),
    #[asn(key = 253)]
    Id_Time_UE_StayedInCell_EnhancedGranularity(Time_UE_StayedInCell_EnhancedGranularity),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LastVisitedUTRANCell_ItemIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: LastVisitedUTRANCell_ItemIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LastVisitedUTRANCell_ItemIE_Extensions(
    Vec<LastVisitedUTRANCell_ItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LeftMBMSBearerService_IEs_EntryIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LeftMBMSBearerService_IEs_EntryIE_Extensions(
    Vec<LeftMBMSBearerService_IEs_EntryIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LeftMBMSBearerService_IEs_Entry {
    pub tmgi: TMGI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<LeftMBMSBearerService_IEs_EntryIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum LocationRelatedDataFailureProtocolIEs_EntryValue {
    #[asn(key = 4)]
    Id_Cause(Cause),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationRelatedDataFailureProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: LocationRelatedDataFailureProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct LocationRelatedDataFailureProtocolIEs(Vec<LocationRelatedDataFailureProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum LocationRelatedDataFailureProtocolExtensions_EntryExtensionValue {
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationRelatedDataFailureProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: LocationRelatedDataFailureProtocolExtensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LocationRelatedDataFailureProtocolExtensions(
    Vec<LocationRelatedDataFailureProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum LocationRelatedDataRequestProtocolIEs_EntryValue {
    #[asn(key = 95)]
    Id_LocationRelatedDataRequestType(LocationRelatedDataRequestType),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationRelatedDataRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: LocationRelatedDataRequestProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct LocationRelatedDataRequestProtocolIEs(Vec<LocationRelatedDataRequestProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum LocationRelatedDataRequestProtocolExtensions_EntryExtensionValue {
    #[asn(key = 115)]
    Id_LocationRelatedDataRequestTypeSpecificToGERANIuMode(
        LocationRelatedDataRequestTypeSpecificToGERANIuMode,
    ),
    #[asn(key = 185)]
    Id_RequestedGANSSAssistanceData(RequestedGANSSAssistanceData),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationRelatedDataRequestProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: LocationRelatedDataRequestProtocolExtensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LocationRelatedDataRequestProtocolExtensions(
    Vec<LocationRelatedDataRequestProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum LocationRelatedDataResponseProtocolIEs_EntryValue {
    #[asn(key = 94)]
    Id_BroadcastAssistanceDataDecipheringKeys(BroadcastAssistanceDataDecipheringKeys),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationRelatedDataResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: LocationRelatedDataResponseProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct LocationRelatedDataResponseProtocolIEs(
    Vec<LocationRelatedDataResponseProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum LocationRelatedDataResponseProtocolExtensions_EntryExtensionValue {
    #[asn(key = 186)]
    Id_BroadcastGANSSAssistanceDataDecipheringKeys(BroadcastAssistanceDataDecipheringKeys),
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationRelatedDataResponseProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: LocationRelatedDataResponseProtocolExtensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LocationRelatedDataResponseProtocolExtensions(
    Vec<LocationRelatedDataResponseProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum LocationReportProtocolIEs_EntryValue {
    #[asn(key = 0)]
    Id_AreaIdentity(AreaIdentity),
    #[asn(key = 4)]
    Id_Cause(Cause),
    #[asn(key = 57)]
    Id_RequestType(RequestType),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationReportProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: LocationReportProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct LocationReportProtocolIEs(Vec<LocationReportProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationReportProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: LocationReportProtocolExtensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LocationReportProtocolExtensions(Vec<LocationReportProtocolExtensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum LocationReportingControlProtocolIEs_EntryValue {
    #[asn(key = 57)]
    Id_RequestType(RequestType),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationReportingControlProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: LocationReportingControlProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct LocationReportingControlProtocolIEs(Vec<LocationReportingControlProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationReportingControlProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: LocationReportingControlProtocolExtensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LocationReportingControlProtocolExtensions(
    Vec<LocationReportingControlProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationReportingTransferInformationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LocationReportingTransferInformationIE_Extensions(
    Vec<LocationReportingTransferInformationIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LoggedMDTIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LoggedMDTIE_Extensions(Vec<LoggedMDTIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M4_Collection_ParametersIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct M4_Collection_ParametersIE_Extensions(Vec<M4_Collection_ParametersIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "NULL")]
pub struct NULL_43;

#[derive(Debug, AperCodec)]
#[asn(type = "NULL")]
pub struct NULL_44;

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M6ReportIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct M6ReportIE_Extensions(Vec<M6ReportIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M7ReportIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct M7ReportIE_Extensions(Vec<M7ReportIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MBMSCNDe_RegistrationRequestProtocolIEs_EntryValue {
    #[asn(key = 96)]
    Id_GlobalCN_ID(GlobalCN_ID),
    #[asn(key = 153)]
    Id_TMGI(TMGI),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSCNDe_RegistrationRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSCNDe_RegistrationRequestProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSCNDe_RegistrationRequestProtocolIEs(
    Vec<MBMSCNDe_RegistrationRequestProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSCNDe_RegistrationRequestProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSCNDe_RegistrationRequestProtocolExtensions(
    Vec<MBMSCNDe_RegistrationRequestProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSCNDe_RegistrationResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSCNDe_RegistrationResponseProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSCNDe_RegistrationResponseProtocolIEs(
    Vec<MBMSCNDe_RegistrationResponseProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MBMSCNDe_RegistrationResponseProtocolExtensions_EntryExtensionValue {
    #[asn(key = 171)]
    Id_ExtendedRNC_ID(ExtendedRNC_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSCNDe_RegistrationResponseProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: MBMSCNDe_RegistrationResponseProtocolExtensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSCNDe_RegistrationResponseProtocolExtensions(
    Vec<MBMSCNDe_RegistrationResponseProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSIPMulticastAddressandAPNlistIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSIPMulticastAddressandAPNlistIE_Extensions(
    Vec<MBMSIPMulticastAddressandAPNlistIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MBMSRABEstablishmentIndicationProtocolIEs_EntryValue {
    #[asn(key = 154)]
    Id_TransportLayerInformation(TransportLayerInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSRABEstablishmentIndicationProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSRABEstablishmentIndicationProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSRABEstablishmentIndicationProtocolIEs(
    Vec<MBMSRABEstablishmentIndicationProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSRABEstablishmentIndicationProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSRABEstablishmentIndicationProtocolExtensions(
    Vec<MBMSRABEstablishmentIndicationProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MBMSRABReleaseProtocolIEs_EntryValue {
    #[asn(key = 4)]
    Id_Cause(Cause),
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSRABReleaseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSRABReleaseProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSRABReleaseProtocolIEs(Vec<MBMSRABReleaseProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSRABReleaseProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSRABReleaseProtocolExtensions(Vec<MBMSRABReleaseProtocolExtensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MBMSRABReleaseFailureProtocolIEs_EntryValue {
    #[asn(key = 4)]
    Id_Cause(Cause),
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSRABReleaseFailureProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSRABReleaseFailureProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSRABReleaseFailureProtocolIEs(Vec<MBMSRABReleaseFailureProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSRABReleaseFailureProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSRABReleaseFailureProtocolExtensions(
    Vec<MBMSRABReleaseFailureProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MBMSRABReleaseRequestProtocolIEs_EntryValue {
    #[asn(key = 4)]
    Id_Cause(Cause),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSRABReleaseRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSRABReleaseRequestProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSRABReleaseRequestProtocolIEs(Vec<MBMSRABReleaseRequestProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSRABReleaseRequestProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSRABReleaseRequestProtocolExtensions(
    Vec<MBMSRABReleaseRequestProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSRegistrationFailureProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSRegistrationFailureProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSRegistrationFailureProtocolIEs(Vec<MBMSRegistrationFailureProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSRegistrationFailureProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSRegistrationFailureProtocolExtensions(
    Vec<MBMSRegistrationFailureProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSRegistrationRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSRegistrationRequestProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSRegistrationRequestProtocolIEs(Vec<MBMSRegistrationRequestProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MBMSRegistrationRequestProtocolExtensions_EntryExtensionValue {
    #[asn(key = 171)]
    Id_ExtendedRNC_ID(ExtendedRNC_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSRegistrationRequestProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: MBMSRegistrationRequestProtocolExtensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSRegistrationRequestProtocolExtensions(
    Vec<MBMSRegistrationRequestProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MBMSRegistrationResponseProtocolIEs_EntryValue {
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 96)]
    Id_GlobalCN_ID(GlobalCN_ID),
    #[asn(key = 153)]
    Id_TMGI(TMGI),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSRegistrationResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSRegistrationResponseProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSRegistrationResponseProtocolIEs(Vec<MBMSRegistrationResponseProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSRegistrationResponseProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSRegistrationResponseProtocolExtensions(
    Vec<MBMSRegistrationResponseProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionStartProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSSessionStartProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSSessionStartProtocolIEs(Vec<MBMSSessionStartProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionStartProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: MBMSSessionStartProtocolExtensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSSessionStartProtocolExtensions(Vec<MBMSSessionStartProtocolExtensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MBMSSessionStartFailureProtocolIEs_EntryValue {
    #[asn(key = 4)]
    Id_Cause(Cause),
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionStartFailureProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSSessionStartFailureProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSSessionStartFailureProtocolIEs(Vec<MBMSSessionStartFailureProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionStartFailureProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSSessionStartFailureProtocolExtensions(
    Vec<MBMSSessionStartFailureProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MBMSSessionStartResponseProtocolIEs_EntryValue {
    #[asn(key = 4)]
    Id_Cause(Cause),
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 154)]
    Id_TransportLayerInformation(TransportLayerInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionStartResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSSessionStartResponseProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSSessionStartResponseProtocolIEs(Vec<MBMSSessionStartResponseProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionStartResponseProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSSessionStartResponseProtocolExtensions(
    Vec<MBMSSessionStartResponseProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MBMSSessionStopProtocolIEs_EntryValue {
    #[asn(key = 144)]
    Id_MBMSCNDe_Registration(MBMSCNDe_Registration),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionStopProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSSessionStopProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSSessionStopProtocolIEs(Vec<MBMSSessionStopProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionStopProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSSessionStopProtocolExtensions(Vec<MBMSSessionStopProtocolExtensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MBMSSessionStopResponseProtocolIEs_EntryValue {
    #[asn(key = 4)]
    Id_Cause(Cause),
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionStopResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSSessionStopResponseProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSSessionStopResponseProtocolIEs(Vec<MBMSSessionStopResponseProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionStopResponseProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSSessionStopResponseProtocolExtensions(
    Vec<MBMSSessionStopResponseProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MBMSSessionUpdateProtocolIEs_EntryValue {
    #[asn(key = 134)]
    Id_DeltaRAListofIdleModeUEs(DeltaRAListofIdleModeUEs),
    #[asn(key = 152)]
    Id_SessionUpdateID(SessionUpdateID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionUpdateProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSSessionUpdateProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSSessionUpdateProtocolIEs(Vec<MBMSSessionUpdateProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionUpdateProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSSessionUpdateProtocolExtensions(Vec<MBMSSessionUpdateProtocolExtensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MBMSSessionUpdateFailureProtocolIEs_EntryValue {
    #[asn(key = 4)]
    Id_Cause(Cause),
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 152)]
    Id_SessionUpdateID(SessionUpdateID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionUpdateFailureProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSSessionUpdateFailureProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSSessionUpdateFailureProtocolIEs(Vec<MBMSSessionUpdateFailureProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionUpdateFailureProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSSessionUpdateFailureProtocolExtensions(
    Vec<MBMSSessionUpdateFailureProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionUpdateResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSSessionUpdateResponseProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSSessionUpdateResponseProtocolIEs(Vec<MBMSSessionUpdateResponseProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSessionUpdateResponseProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSSessionUpdateResponseProtocolExtensions(
    Vec<MBMSSessionUpdateResponseProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MBMSSynchronisationInformationIE_Extensions_EntryExtensionValue {
    #[asn(key = 236)]
    Id_IP_Source_Address(IPMulticastAddress),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSSynchronisationInformationIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: MBMSSynchronisationInformationIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSSynchronisationInformationIE_Extensions(
    Vec<MBMSSynchronisationInformationIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MBMSUELinkingRequestProtocolIEs_EntryValue {
    #[asn(key = 141)]
    Id_JoinedMBMSBearerServicesList(JoinedMBMSBearerService_IEs),
    #[asn(key = 142)]
    Id_LeftMBMSBearerServicesList(LeftMBMSBearerService_IEs),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSUELinkingRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSUELinkingRequestProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSUELinkingRequestProtocolIEs(Vec<MBMSUELinkingRequestProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSUELinkingRequestProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSUELinkingRequestProtocolExtensions(
    Vec<MBMSUELinkingRequestProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MBMSUELinkingResponseProtocolIEs_EntryValue {
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 155)]
    Id_UnsuccessfulLinkingList(UnsuccessfulLinking_IEs),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSUELinkingResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMSUELinkingResponseProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMSUELinkingResponseProtocolIEs(Vec<MBMSUELinkingResponseProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSUELinkingResponseProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSUELinkingResponseProtocolExtensions(
    Vec<MBMSUELinkingResponseProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MDT_ConfigurationIE_Extensions_EntryExtensionValue {
    #[asn(key = 264)]
    Id_SignallingBasedMDTPLMNList(MDT_PLMN_List),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MDT_ConfigurationIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: MDT_ConfigurationIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MDT_ConfigurationIE_Extensions(Vec<MDT_ConfigurationIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "NULL")]
pub struct NULL_45;

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MessageStructure_EntryIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MessageStructure_EntryIE_Extensions(Vec<MessageStructure_EntryIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct MessageStructure_Entry {
    pub ie_id: ProtocolIE_ID,
    #[asn(optional_idx = 0)]
    pub repetition_number: Option<RepetitionNumber1>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<MessageStructure_EntryIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NotEmptyRAListofIdleModeUEsIE_Extensions_EntryExtensionValue {
    #[asn(key = 180)]
    Id_LAofIdleModeUEs(LAListofIdleModeUEs),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NotEmptyRAListofIdleModeUEsIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: NotEmptyRAListofIdleModeUEsIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NotEmptyRAListofIdleModeUEsIE_Extensions(
    Vec<NotEmptyRAListofIdleModeUEsIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Offload_RAB_ParametersIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Offload_RAB_ParametersIE_Extensions(Vec<Offload_RAB_ParametersIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum OverloadProtocolIEs_EntryValue {
    #[asn(key = 86)]
    Id_GlobalRNC_ID(GlobalRNC_ID),
    #[asn(key = 18)]
    Id_NumberOfSteps(NumberOfSteps),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct OverloadProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: OverloadProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct OverloadProtocolIEs(Vec<OverloadProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct OverloadProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: OverloadProtocolExtensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct OverloadProtocolExtensions(Vec<OverloadProtocolExtensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PLMNBasedIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PLMNBasedIE_Extensions(Vec<PLMNBasedIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PLMNs_in_shared_network_EntryIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PLMNs_in_shared_network_EntryIE_Extensions(
    Vec<PLMNs_in_shared_network_EntryIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PLMNs_in_shared_network_Entry {
    pub plm_nidentity: PLMNidentity,
    pub la_list: LA_LIST,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PLMNs_in_shared_network_EntryIE_Extensions>,
}

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PagingProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: PagingProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PagingProtocolIEs(Vec<PagingProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PagingProtocolExtensions_EntryExtensionValue {
    #[asn(key = 229)]
    Id_CSG_Id_List(CSG_Id_List),
    #[asn(key = 96)]
    Id_GlobalCN_ID(GlobalCN_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PagingProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: PagingProtocolExtensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PagingProtocolExtensions(Vec<PagingProtocolExtensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "8639999", extensible = true)]
pub struct INTEGER_46(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "8639999", extensible = true)]
pub struct INTEGER_47(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PeriodicLocationInfoIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PeriodicLocationInfoIE_Extensions(Vec<PeriodicLocationInfoIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PositionDataIE_Extensions_EntryExtensionValue {
    #[asn(key = 284)]
    Id_Additional_PositioningDataSet(Additional_PositioningDataSet),
    #[asn(key = 184)]
    Id_GANSS_PositioningDataSet(GANSS_PositioningDataSet),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PositionDataIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: PositionDataIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PositionDataIE_Extensions(Vec<PositionDataIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct INTEGER_48(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "OBJECT-IDENTIFIER")]
pub struct OBJECT_IDENTIFIER_49;

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PrivateMessagePrivateIEs_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PrivateMessagePrivateIEs(Vec<PrivateMessagePrivateIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_AssignmentRequestProtocolIEs_EntryValue {
    #[asn(key = 41)]
    Id_RAB_ReleaseList(RAB_ReleaseList),
    #[asn(key = 54)]
    Id_RAB_SetupOrModifyList(RAB_SetupOrModifyList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_AssignmentRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_AssignmentRequestProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_AssignmentRequestProtocolIEs(Vec<RAB_AssignmentRequestProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_AssignmentRequestProtocolExtensions_EntryExtensionValue {
    #[asn(key = 239)]
    Id_MSISDN(MSISDN),
    #[asn(key = 233)]
    Id_UE_AggregateMaximumBitRate(UE_AggregateMaximumBitRate),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_AssignmentRequestProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RAB_AssignmentRequestProtocolExtensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_AssignmentRequestProtocolExtensions(
    Vec<RAB_AssignmentRequestProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_AssignmentResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_AssignmentResponseProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_AssignmentResponseProtocolIEs(Vec<RAB_AssignmentResponseProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_AssignmentResponseProtocolExtensions_EntryExtensionValue {
    #[asn(key = 110)]
    Id_GERAN_Iumode_RAB_FailedList_RABAssgntResponse(GERAN_Iumode_RAB_FailedList_RABAssgntResponse),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_AssignmentResponseProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RAB_AssignmentResponseProtocolExtensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_AssignmentResponseProtocolExtensions(
    Vec<RAB_AssignmentResponseProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_ContextFailedtoTransferList_Entry_EntryValue {
    #[asn(key = 84)]
    Id_RAB_ContextFailedtoTransferItem(RABs_ContextFailedtoTransferItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ContextFailedtoTransferList_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_ContextFailedtoTransferList_Entry_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_ContextFailedtoTransferList_Entry(Vec<RAB_ContextFailedtoTransferList_Entry_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ContextItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_ContextItemIE_Extensions(Vec<RAB_ContextItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ContextItem_RANAP_RelocInfIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_ContextItem_RANAP_RelocInfIE_Extensions(
    Vec<RAB_ContextItem_RANAP_RelocInfIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_ContextList_Entry_EntryValue {
    #[asn(key = 24)]
    Id_RAB_ContextItem(RAB_ContextItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ContextList_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_ContextList_Entry_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_ContextList_Entry(Vec<RAB_ContextList_Entry_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_ContextList_RANAP_RelocInf_Entry_EntryValue {
    #[asn(key = 82)]
    Id_RAB_ContextItem_RANAP_RelocInf(RAB_ContextItem_RANAP_RelocInf),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ContextList_RANAP_RelocInf_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_ContextList_RANAP_RelocInf_Entry_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_ContextList_RANAP_RelocInf_Entry(Vec<RAB_ContextList_RANAP_RelocInf_Entry_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_DataForwardingItemIE_Extensions_EntryExtensionValue {
    #[asn(key = 13)]
    Id_IuTransportAssociation(IuTransportAssociation),
    #[asn(key = 67)]
    Id_TransportLayerAddress(TransportLayerAddress),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_DataForwardingItemIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RAB_DataForwardingItemIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_DataForwardingItemIE_Extensions(Vec<RAB_DataForwardingItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_DataForwardingItem_SRNS_CtxReqIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_DataForwardingItem_SRNS_CtxReqIE_Extensions(
    Vec<RAB_DataForwardingItem_SRNS_CtxReqIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_DataForwardingList_Entry_EntryValue {
    #[asn(key = 26)]
    Id_RAB_DataForwardingItem(RAB_DataForwardingItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_DataForwardingList_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_DataForwardingList_Entry_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_DataForwardingList_Entry(Vec<RAB_DataForwardingList_Entry_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_DataForwardingList_SRNS_CtxReq_Entry_EntryValue {
    #[asn(key = 27)]
    Id_RAB_DataForwardingItem_SRNS_CtxReq(RAB_DataForwardingItem_SRNS_CtxReq),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_DataForwardingList_SRNS_CtxReq_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_DataForwardingList_SRNS_CtxReq_Entry_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_DataForwardingList_SRNS_CtxReq_Entry(
    Vec<RAB_DataForwardingList_SRNS_CtxReq_Entry_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_DataVolumeReportItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_DataVolumeReportItemIE_Extensions(Vec<RAB_DataVolumeReportItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_DataVolumeReportList_Entry_EntryValue {
    #[asn(key = 30)]
    Id_RAB_DataVolumeReportItem(RAB_DataVolumeReportItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_DataVolumeReportList_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_DataVolumeReportList_Entry_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_DataVolumeReportList_Entry(Vec<RAB_DataVolumeReportList_Entry_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_DataVolumeReportRequestItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_DataVolumeReportRequestItemIE_Extensions(
    Vec<RAB_DataVolumeReportRequestItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_DataVolumeReportRequestList_Entry_EntryValue {
    #[asn(key = 32)]
    Id_RAB_DataVolumeReportRequestItem(RAB_DataVolumeReportRequestItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_DataVolumeReportRequestList_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_DataVolumeReportRequestList_Entry_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_DataVolumeReportRequestList_Entry(Vec<RAB_DataVolumeReportRequestList_Entry_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_FailedItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_FailedItemIE_Extensions(Vec<RAB_FailedItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_FailedItem_EnhRelocInfoResIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_FailedItem_EnhRelocInfoResIE_Extensions(
    Vec<RAB_FailedItem_EnhRelocInfoResIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_FailedList_Entry_EntryValue {
    #[asn(key = 34)]
    Id_RAB_FailedItem(RAB_FailedItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_FailedList_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_FailedList_Entry_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_FailedList_Entry(Vec<RAB_FailedList_Entry_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_FailedList_EnhRelocInfoRes_Entry_EntryValue {
    #[asn(key = 198)]
    Id_RAB_FailedItem_EnhRelocInfoRes(RAB_FailedItem_EnhRelocInfoRes),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_FailedList_EnhRelocInfoRes_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_FailedList_EnhRelocInfoRes_Entry_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_FailedList_EnhRelocInfoRes_Entry(Vec<RAB_FailedList_EnhRelocInfoRes_Entry_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_FailedtoReportList_Entry_EntryValue {
    #[asn(key = 71)]
    Id_RAB_FailedtoReportItem(RABs_failed_to_reportItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_FailedtoReportList_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_FailedtoReportList_Entry_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_FailedtoReportList_Entry(Vec<RAB_FailedtoReportList_Entry_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ModifyItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_ModifyItemIE_Extensions(Vec<RAB_ModifyItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_ModifyList_Entry_EntryValue {
    #[asn(key = 92)]
    Id_RAB_ModifyItem(RAB_ModifyItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ModifyList_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_ModifyList_Entry_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_ModifyList_Entry(Vec<RAB_ModifyList_Entry_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_ModifyRequestProtocolIEs_EntryValue {
    #[asn(key = 91)]
    Id_RAB_ModifyList(RAB_ModifyList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ModifyRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_ModifyRequestProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_ModifyRequestProtocolIEs(Vec<RAB_ModifyRequestProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ModifyRequestProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_ModifyRequestProtocolExtensions(Vec<RAB_ModifyRequestProtocolExtensions_Entry>);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ParametersIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RAB_ParametersIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_ParametersIE_Extensions(Vec<RAB_ParametersIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_QueuedItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_QueuedItemIE_Extensions(Vec<RAB_QueuedItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_QueuedList_Entry_EntryValue {
    #[asn(key = 37)]
    Id_RAB_QueuedItem(RAB_QueuedItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_QueuedList_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_QueuedList_Entry_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_QueuedList_Entry(Vec<RAB_QueuedList_Entry_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ReleaseItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_ReleaseItemIE_Extensions(Vec<RAB_ReleaseItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_ReleaseList_Entry_EntryValue {
    #[asn(key = 40)]
    Id_RAB_ReleaseItem(RAB_ReleaseItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ReleaseList_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_ReleaseList_Entry_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_ReleaseList_Entry(Vec<RAB_ReleaseList_Entry_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_ReleaseRequestProtocolIEs_EntryValue {
    #[asn(key = 41)]
    Id_RAB_ReleaseList(RAB_ReleaseList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ReleaseRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_ReleaseRequestProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_ReleaseRequestProtocolIEs(Vec<RAB_ReleaseRequestProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ReleaseRequestProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_ReleaseRequestProtocolExtensions(Vec<RAB_ReleaseRequestProtocolExtensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ReleasedItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_ReleasedItemIE_Extensions(Vec<RAB_ReleasedItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ReleasedItem_IuRelCompIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_ReleasedItem_IuRelCompIE_Extensions(
    Vec<RAB_ReleasedItem_IuRelCompIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_ReleasedList_Entry_EntryValue {
    #[asn(key = 42)]
    Id_RAB_ReleasedItem(RAB_ReleasedItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ReleasedList_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_ReleasedList_Entry_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_ReleasedList_Entry(Vec<RAB_ReleasedList_Entry_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_ReleasedList_IuRelComp_Entry_EntryValue {
    #[asn(key = 87)]
    Id_RAB_ReleasedItem_IuRelComp(RAB_ReleasedItem_IuRelComp),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ReleasedList_IuRelComp_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_ReleasedList_IuRelComp_Entry_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_ReleasedList_IuRelComp_Entry(Vec<RAB_ReleasedList_IuRelComp_Entry_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_RelocationReleaseItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_RelocationReleaseItemIE_Extensions(
    Vec<RAB_RelocationReleaseItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_RelocationReleaseList_Entry_EntryValue {
    #[asn(key = 45)]
    Id_RAB_RelocationReleaseItem(RAB_RelocationReleaseItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_RelocationReleaseList_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_RelocationReleaseList_Entry_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_RelocationReleaseList_Entry(Vec<RAB_RelocationReleaseList_Entry_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_SetupItem_EnhRelocInfoReqIE_Extensions_EntryExtensionValue {
    #[asn(key = 231)]
    Id_E_UTRAN_Service_Handover(E_UTRAN_Service_Handover),
    #[asn(key = 238)]
    Id_PDP_TypeInformation_extension(PDP_TypeInformation_extension),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupItem_EnhRelocInfoReqIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RAB_SetupItem_EnhRelocInfoReqIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_SetupItem_EnhRelocInfoReqIE_Extensions(
    Vec<RAB_SetupItem_EnhRelocInfoReqIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupItem_EnhRelocInfoResIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_SetupItem_EnhRelocInfoResIE_Extensions(
    Vec<RAB_SetupItem_EnhRelocInfoResIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupItem_EnhancedRelocCompleteReqIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_SetupItem_EnhancedRelocCompleteReqIE_Extensions(
    Vec<RAB_SetupItem_EnhancedRelocCompleteReqIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_SetupItem_EnhancedRelocCompleteResIE_Extensions_EntryExtensionValue {
    #[asn(key = 240)]
    Id_Offload_RAB_Parameters(Offload_RAB_Parameters),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupItem_EnhancedRelocCompleteResIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RAB_SetupItem_EnhancedRelocCompleteResIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_SetupItem_EnhancedRelocCompleteResIE_Extensions(
    Vec<RAB_SetupItem_EnhancedRelocCompleteResIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupItem_RelocReqIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RAB_SetupItem_RelocReqIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_SetupItem_RelocReqIE_Extensions(Vec<RAB_SetupItem_RelocReqIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_SetupItem_RelocReqAckIE_Extensions_EntryExtensionValue {
    #[asn(key = 90)]
    Id_Ass_RAB_Parameters(Ass_RAB_Parameters),
    #[asn(key = 13)]
    Id_IuTransportAssociation(IuTransportAssociation),
    #[asn(key = 67)]
    Id_TransportLayerAddress(TransportLayerAddress),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupItem_RelocReqAckIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RAB_SetupItem_RelocReqAckIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_SetupItem_RelocReqAckIE_Extensions(
    Vec<RAB_SetupItem_RelocReqAckIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_SetupList_EnhRelocInfoReq_Entry_EntryValue {
    #[asn(key = 193)]
    Id_RAB_SetupItem_EnhRelocInfoReq(RAB_SetupItem_EnhRelocInfoReq),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupList_EnhRelocInfoReq_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_SetupList_EnhRelocInfoReq_Entry_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_SetupList_EnhRelocInfoReq_Entry(Vec<RAB_SetupList_EnhRelocInfoReq_Entry_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_SetupList_EnhRelocInfoRes_Entry_EntryValue {
    #[asn(key = 195)]
    Id_RAB_SetupItem_EnhRelocInfoRes(RAB_SetupItem_EnhRelocInfoRes),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupList_EnhRelocInfoRes_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_SetupList_EnhRelocInfoRes_Entry_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_SetupList_EnhRelocInfoRes_Entry(Vec<RAB_SetupList_EnhRelocInfoRes_Entry_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_SetupList_EnhancedRelocCompleteReq_Entry_EntryValue {
    #[asn(key = 189)]
    Id_RAB_SetupItem_EnhancedRelocCompleteReq(RAB_SetupItem_EnhancedRelocCompleteReq),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupList_EnhancedRelocCompleteReq_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_SetupList_EnhancedRelocCompleteReq_Entry_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_SetupList_EnhancedRelocCompleteReq_Entry(
    Vec<RAB_SetupList_EnhancedRelocCompleteReq_Entry_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_SetupList_EnhancedRelocCompleteRes_Entry_EntryValue {
    #[asn(key = 191)]
    Id_RAB_SetupItem_EnhancedRelocCompleteRes(RAB_SetupItem_EnhancedRelocCompleteRes),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupList_EnhancedRelocCompleteRes_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_SetupList_EnhancedRelocCompleteRes_Entry_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_SetupList_EnhancedRelocCompleteRes_Entry(
    Vec<RAB_SetupList_EnhancedRelocCompleteRes_Entry_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_SetupList_RelocReq_Entry_EntryValue {
    #[asn(key = 47)]
    Id_RAB_SetupItem_RelocReq(RAB_SetupItem_RelocReq),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupList_RelocReq_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_SetupList_RelocReq_Entry_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_SetupList_RelocReq_Entry(Vec<RAB_SetupList_RelocReq_Entry_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_SetupList_RelocReqAck_Entry_EntryValue {
    #[asn(key = 48)]
    Id_RAB_SetupItem_RelocReqAck(RAB_SetupItem_RelocReqAck),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupList_RelocReqAck_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_SetupList_RelocReqAck_Entry_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_SetupList_RelocReqAck_Entry(Vec<RAB_SetupList_RelocReqAck_Entry_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_SetupOrModifiedItemIE_Extensions_EntryExtensionValue {
    #[asn(key = 90)]
    Id_Ass_RAB_Parameters(Ass_RAB_Parameters),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupOrModifiedItemIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RAB_SetupOrModifiedItemIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_SetupOrModifiedItemIE_Extensions(Vec<RAB_SetupOrModifiedItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_SetupOrModifiedList_Entry_EntryValue {
    #[asn(key = 51)]
    Id_RAB_SetupOrModifiedItem(RAB_SetupOrModifiedItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupOrModifiedList_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_SetupOrModifiedList_Entry_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_SetupOrModifiedList_Entry(Vec<RAB_SetupOrModifiedList_Entry_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_SetupOrModifyItemFirstIE_Extensions_EntryExtensionValue {
    #[asn(key = 242)]
    Id_Correlation_ID(Correlation_ID),
    #[asn(key = 231)]
    Id_E_UTRAN_Service_Handover(E_UTRAN_Service_Handover),
    #[asn(key = 274)]
    Id_SIPTO_Correlation_ID(Correlation_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupOrModifyItemFirstIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RAB_SetupOrModifyItemFirstIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_SetupOrModifyItemFirstIE_Extensions(
    Vec<RAB_SetupOrModifyItemFirstIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupOrModifyItemSecondIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RAB_SetupOrModifyItemSecondIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_SetupOrModifyItemSecondIE_Extensions(
    Vec<RAB_SetupOrModifyItemSecondIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_SetupOrModifyList_Entry_EntryFirstValue {
    #[asn(key = 53)]
    Id_RAB_SetupOrModifyItem(RAB_SetupOrModifyItemFirst),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_SetupOrModifyList_Entry_EntrySecondValue {
    #[asn(key = 53)]
    Id_RAB_SetupOrModifyItem(RAB_SetupOrModifyItemSecond),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_SetupOrModifyList_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub first_criticality: Criticality,
    pub first_value: RAB_SetupOrModifyList_Entry_EntryFirstValue,
    pub second_criticality: Criticality,
    pub second_value: RAB_SetupOrModifyList_Entry_EntrySecondValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_SetupOrModifyList_Entry(Vec<RAB_SetupOrModifyList_Entry_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ToBeReleasedItem_EnhancedRelocCompleteResIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_ToBeReleasedItem_EnhancedRelocCompleteResIE_Extensions(
    Vec<RAB_ToBeReleasedItem_EnhancedRelocCompleteResIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_ToBeReleasedList_EnhancedRelocCompleteRes_Entry_EntryValue {
    #[asn(key = 209)]
    Id_RAB_ToBeReleasedItem_EnhancedRelocCompleteRes(RAB_ToBeReleasedItem_EnhancedRelocCompleteRes),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_ToBeReleasedList_EnhancedRelocCompleteRes_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RAB_ToBeReleasedList_EnhancedRelocCompleteRes_Entry_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RAB_ToBeReleasedList_EnhancedRelocCompleteRes_Entry(
    Vec<RAB_ToBeReleasedList_EnhancedRelocCompleteRes_Entry_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RAB_TrCH_MappingItemIE_Extensions_EntryExtensionValue {
    #[asn(key = 3)]
    Id_CN_DomainIndicator(CN_DomainIndicator),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAB_TrCH_MappingItemIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RAB_TrCH_MappingItemIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAB_TrCH_MappingItemIE_Extensions(Vec<RAB_TrCH_MappingItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RABDataVolumeReport_EntryIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RABDataVolumeReport_EntryIE_Extensions(
    Vec<RABDataVolumeReport_EntryIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct RABDataVolumeReport_Entry {
    pub dl_unsuccessfully_transmitted_data_volume: UnsuccessfullyTransmittedDataVolume,
    #[asn(optional_idx = 0)]
    pub data_volume_reference: Option<DataVolumeReference>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<RABDataVolumeReport_EntryIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RABParametersList_EntryIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RABParametersList_EntryIE_Extensions(Vec<RABParametersList_EntryIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RABasedIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RABasedIE_Extensions(Vec<RABasedIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RABs_ContextFailedtoTransferItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RABs_ContextFailedtoTransferItemIE_Extensions(
    Vec<RABs_ContextFailedtoTransferItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RABs_failed_to_reportItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RABs_failed_to_reportItemIE_Extensions(
    Vec<RABs_failed_to_reportItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAIIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RAIIE_Extensions(Vec<RAIIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ENUMERATED_50(u8);
impl ENUMERATED_50 {
    const EMPTYLIST: u8 = 0u8;
    const FULLLIST: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RANAP_EnhancedRelocationInformationRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RANAP_EnhancedRelocationInformationRequestProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RANAP_EnhancedRelocationInformationRequestProtocolIEs(
    Vec<RANAP_EnhancedRelocationInformationRequestProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RANAP_EnhancedRelocationInformationRequestProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value:
        RANAP_EnhancedRelocationInformationRequestProtocolExtensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RANAP_EnhancedRelocationInformationRequestProtocolExtensions(
    Vec<RANAP_EnhancedRelocationInformationRequestProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RANAP_EnhancedRelocationInformationResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RANAP_EnhancedRelocationInformationResponseProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RANAP_EnhancedRelocationInformationResponseProtocolIEs(
    Vec<RANAP_EnhancedRelocationInformationResponseProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RANAP_EnhancedRelocationInformationResponseProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RANAP_EnhancedRelocationInformationResponseProtocolExtensions(
    Vec<RANAP_EnhancedRelocationInformationResponseProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RANAP_RelocationInformationProtocolIEs_EntryValue {
    #[asn(key = 81)]
    Id_DirectTransferInformationList_RANAP_RelocInf(DirectTransferInformationList_RANAP_RelocInf),
    #[asn(key = 83)]
    Id_RAB_ContextList_RANAP_RelocInf(RAB_ContextList_RANAP_RelocInf),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RANAP_RelocationInformationProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RANAP_RelocationInformationProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RANAP_RelocationInformationProtocolIEs(
    Vec<RANAP_RelocationInformationProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RANAP_RelocationInformationProtocolExtensions_EntryExtensionValue {
    #[asn(key = 247)]
    Id_RNSAPRelocationParameters(RNSAPRelocationParameters),
    #[asn(key = 103)]
    Id_SourceRNC_PDCP_context_info(RRC_Container),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RANAP_RelocationInformationProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RANAP_RelocationInformationProtocolExtensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RANAP_RelocationInformationProtocolExtensions(
    Vec<RANAP_RelocationInformationProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RIM_TransferIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RIM_TransferIE_Extensions(Vec<RIM_TransferIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct ENUMERATED_51(u8);
impl ENUMERATED_51 {
    const ACTIVATED: u8 = 0u8;
    const DEACTIVATED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RNCTraceInformationIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RNCTraceInformationIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RNCTraceInformationIE_Extensions(Vec<RNCTraceInformationIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RNSAPRelocationParametersIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RNSAPRelocationParametersIE_Extensions(
    Vec<RNSAPRelocationParametersIE_Extensions_Entry>,
);

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
pub struct RSRVCC_InformationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RSRVCC_InformationIE_Extensions(Vec<RSRVCC_InformationIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RedirectionIndication_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RedirectionIndication_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RelocationCancelProtocolIEs_EntryValue {
    #[asn(key = 4)]
    Id_Cause(Cause),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationCancelProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RelocationCancelProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RelocationCancelProtocolIEs(Vec<RelocationCancelProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationCancelProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RelocationCancelProtocolExtensions(Vec<RelocationCancelProtocolExtensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RelocationCancelAcknowledgeProtocolIEs_EntryValue {
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationCancelAcknowledgeProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RelocationCancelAcknowledgeProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RelocationCancelAcknowledgeProtocolIEs(
    Vec<RelocationCancelAcknowledgeProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationCancelAcknowledgeProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RelocationCancelAcknowledgeProtocolExtensions(
    Vec<RelocationCancelAcknowledgeProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationCommandProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RelocationCommandProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RelocationCommandProtocolIEs(Vec<RelocationCommandProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationCommandProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RelocationCommandProtocolExtensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RelocationCommandProtocolExtensions(Vec<RelocationCommandProtocolExtensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationCompleteProtocolIEs_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RelocationCompleteProtocolIEs(Vec<RelocationCompleteProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RelocationCompleteProtocolExtensions_EntryExtensionValue {
    #[asn(key = 250)]
    Id_HigherBitratesThan16MbpsFlag(HigherBitratesThan16MbpsFlag),
    #[asn(key = 275)]
    Id_LHN_ID(LHN_ID),
    #[asn(key = 262)]
    Id_Tunnel_Information_for_BBF(TunnelInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationCompleteProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RelocationCompleteProtocolExtensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RelocationCompleteProtocolExtensions(Vec<RelocationCompleteProtocolExtensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationDetectProtocolIEs_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RelocationDetectProtocolIEs(Vec<RelocationDetectProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationDetectProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RelocationDetectProtocolExtensions(Vec<RelocationDetectProtocolExtensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RelocationFailureProtocolIEs_EntryValue {
    #[asn(key = 4)]
    Id_Cause(Cause),
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationFailureProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RelocationFailureProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RelocationFailureProtocolIEs(Vec<RelocationFailureProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RelocationFailureProtocolExtensions_EntryExtensionValue {
    #[asn(key = 108)]
    Id_GERAN_Classmark(GERAN_Classmark),
    #[asn(key = 100)]
    Id_NewBSS_To_OldBSS_Information(NewBSS_To_OldBSS_Information),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationFailureProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RelocationFailureProtocolExtensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RelocationFailureProtocolExtensions(Vec<RelocationFailureProtocolExtensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RelocationPreparationFailureProtocolIEs_EntryValue {
    #[asn(key = 4)]
    Id_Cause(Cause),
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationPreparationFailureProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RelocationPreparationFailureProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RelocationPreparationFailureProtocolIEs(
    Vec<RelocationPreparationFailureProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RelocationPreparationFailureProtocolExtensions_EntryExtensionValue {
    #[asn(key = 99)]
    Id_InterSystemInformation_TransparentContainer(InterSystemInformation_TransparentContainer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationPreparationFailureProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RelocationPreparationFailureProtocolExtensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RelocationPreparationFailureProtocolExtensions(
    Vec<RelocationPreparationFailureProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RelocationRequestProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RelocationRequestProtocolIEs(Vec<RelocationRequestProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationRequestProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RelocationRequestProtocolExtensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RelocationRequestProtocolExtensions(Vec<RelocationRequestProtocolExtensions_Entry>);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationRequestAcknowledgeProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RelocationRequestAcknowledgeProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RelocationRequestAcknowledgeProtocolIEs(
    Vec<RelocationRequestAcknowledgeProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RelocationRequestAcknowledgeProtocolExtensions_EntryExtensionValue {
    #[asn(key = 203)]
    Id_CSG_Id(CSG_Id),
    #[asn(key = 100)]
    Id_NewBSS_To_OldBSS_Information(NewBSS_To_OldBSS_Information),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationRequestAcknowledgeProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RelocationRequestAcknowledgeProtocolExtensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RelocationRequestAcknowledgeProtocolExtensions(
    Vec<RelocationRequestAcknowledgeProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationRequiredProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RelocationRequiredProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RelocationRequiredProtocolIEs(Vec<RelocationRequiredProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelocationRequiredProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RelocationRequiredProtocolExtensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RelocationRequiredProtocolExtensions(Vec<RelocationRequiredProtocolExtensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct INTEGER_56(u8);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Requested_RAB_Parameter_ValuesIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: Requested_RAB_Parameter_ValuesIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Requested_RAB_Parameter_ValuesIE_Extensions(
    Vec<Requested_RAB_Parameter_ValuesIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RerouteNASRequestProtocolIEs_EntryValue {
    #[asn(key = 287)]
    Id_P_TMSI(P_TMSI),
    #[asn(key = 286)]
    Id_SGSN_Group_Identity(SGSN_Group_Identity),
    #[asn(key = 290)]
    Id_UE_Usage_Type(UE_Usage_Type),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RerouteNASRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RerouteNASRequestProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RerouteNASRequestProtocolIEs(Vec<RerouteNASRequestProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RerouteNASRequestProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RerouteNASRequestProtocolExtensions(Vec<RerouteNASRequestProtocolExtensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ResetProtocolIEs_EntryValue {
    #[asn(key = 3)]
    Id_CN_DomainIndicator(CN_DomainIndicator),
    #[asn(key = 4)]
    Id_Cause(Cause),
    #[asn(key = 86)]
    Id_GlobalRNC_ID(GlobalRNC_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ResetProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ResetProtocolIEs(Vec<ResetProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ResetProtocolExtensions_EntryExtensionValue {
    #[asn(key = 171)]
    Id_ExtendedRNC_ID(ExtendedRNC_ID),
    #[asn(key = 96)]
    Id_GlobalCN_ID(GlobalCN_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: ResetProtocolExtensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ResetProtocolExtensions(Vec<ResetProtocolExtensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ResetAcknowledgeProtocolIEs_EntryValue {
    #[asn(key = 3)]
    Id_CN_DomainIndicator(CN_DomainIndicator),
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 86)]
    Id_GlobalRNC_ID(GlobalRNC_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetAcknowledgeProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ResetAcknowledgeProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ResetAcknowledgeProtocolIEs(Vec<ResetAcknowledgeProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ResetAcknowledgeProtocolExtensions_EntryExtensionValue {
    #[asn(key = 171)]
    Id_ExtendedRNC_ID(ExtendedRNC_ID),
    #[asn(key = 96)]
    Id_GlobalCN_ID(GlobalCN_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetAcknowledgeProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: ResetAcknowledgeProtocolExtensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ResetAcknowledgeProtocolExtensions(Vec<ResetAcknowledgeProtocolExtensions_Entry>);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetResourceProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ResetResourceProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ResetResourceProtocolIEs(Vec<ResetResourceProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ResetResourceProtocolExtensions_EntryExtensionValue {
    #[asn(key = 171)]
    Id_ExtendedRNC_ID(ExtendedRNC_ID),
    #[asn(key = 96)]
    Id_GlobalCN_ID(GlobalCN_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetResourceProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: ResetResourceProtocolExtensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ResetResourceProtocolExtensions(Vec<ResetResourceProtocolExtensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ResetResourceAckItemIE_Extensions_EntryExtensionValue {
    #[asn(key = 282)]
    Id_IuSigConIdRangeEnd(IuSignallingConnectionIdentifier),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetResourceAckItemIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: ResetResourceAckItemIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ResetResourceAckItemIE_Extensions(Vec<ResetResourceAckItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ResetResourceAckList_Entry_EntryValue {
    #[asn(key = 78)]
    Id_IuSigConIdItem(ResetResourceAckItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetResourceAckList_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ResetResourceAckList_Entry_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ResetResourceAckList_Entry(Vec<ResetResourceAckList_Entry_Entry>);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetResourceAcknowledgeProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ResetResourceAcknowledgeProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ResetResourceAcknowledgeProtocolIEs(Vec<ResetResourceAcknowledgeProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ResetResourceAcknowledgeProtocolExtensions_EntryExtensionValue {
    #[asn(key = 171)]
    Id_ExtendedRNC_ID(ExtendedRNC_ID),
    #[asn(key = 96)]
    Id_GlobalCN_ID(GlobalCN_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetResourceAcknowledgeProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: ResetResourceAcknowledgeProtocolExtensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ResetResourceAcknowledgeProtocolExtensions(
    Vec<ResetResourceAcknowledgeProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ResetResourceItemIE_Extensions_EntryExtensionValue {
    #[asn(key = 282)]
    Id_IuSigConIdRangeEnd(IuSignallingConnectionIdentifier),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetResourceItemIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: ResetResourceItemIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ResetResourceItemIE_Extensions(Vec<ResetResourceItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ResetResourceList_Entry_EntryValue {
    #[asn(key = 78)]
    Id_IuSigConIdItem(ResetResourceItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetResourceList_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ResetResourceList_Entry_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ResetResourceList_Entry(Vec<ResetResourceList_Entry_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "9")]
pub struct INTEGER_57(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "8")]
pub struct INTEGER_58(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResidualBitErrorRatioIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ResidualBitErrorRatioIE_Extensions(Vec<ResidualBitErrorRatioIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SAIIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SAIIE_Extensions(Vec<SAIIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "9")]
pub struct INTEGER_59(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "6")]
pub struct INTEGER_60(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SDU_ErrorRatioIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SDU_ErrorRatioIE_Extensions(Vec<SDU_ErrorRatioIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SDU_FormatInformationParameters_EntryIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SDU_FormatInformationParameters_EntryIE_Extensions(
    Vec<SDU_FormatInformationParameters_EntryIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct SDU_FormatInformationParameters_Entry {
    #[asn(optional_idx = 0)]
    pub subflow_sdu_size: Option<SubflowSDU_Size>,
    #[asn(optional_idx = 1)]
    pub rab_subflow_combination_bit_rate: Option<RAB_SubflowCombinationBitRate>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<SDU_FormatInformationParameters_EntryIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SDU_Parameters_EntryIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SDU_Parameters_EntryIE_Extensions(Vec<SDU_Parameters_EntryIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SNA_Access_InformationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SNA_Access_InformationIE_Extensions(Vec<SNA_Access_InformationIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SRB_TrCH_MappingItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SRB_TrCH_MappingItemIE_Extensions(Vec<SRB_TrCH_MappingItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SRNS_ContextRequestProtocolIEs_EntryValue {
    #[asn(key = 29)]
    Id_RAB_DataForwardingList_SRNS_CtxReq(RAB_DataForwardingList_SRNS_CtxReq),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SRNS_ContextRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: SRNS_ContextRequestProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct SRNS_ContextRequestProtocolIEs(Vec<SRNS_ContextRequestProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SRNS_ContextRequestProtocolExtensions_EntryExtensionValue {
    #[asn(key = 167)]
    Id_RAT_Type(RAT_Type),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SRNS_ContextRequestProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: SRNS_ContextRequestProtocolExtensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SRNS_ContextRequestProtocolExtensions(Vec<SRNS_ContextRequestProtocolExtensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SRNS_ContextResponseProtocolIEs_EntryValue {
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 85)]
    Id_RAB_ContextFailedtoTransferList(RAB_ContextFailedtoTransferList),
    #[asn(key = 25)]
    Id_RAB_ContextList(RAB_ContextList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SRNS_ContextResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: SRNS_ContextResponseProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct SRNS_ContextResponseProtocolIEs(Vec<SRNS_ContextResponseProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SRNS_ContextResponseProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SRNS_ContextResponseProtocolExtensions(
    Vec<SRNS_ContextResponseProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SRNS_DataForwardCommandProtocolIEs_EntryValue {
    #[asn(key = 28)]
    Id_RAB_DataForwardingList(RAB_DataForwardingList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SRNS_DataForwardCommandProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: SRNS_DataForwardCommandProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct SRNS_DataForwardCommandProtocolIEs(Vec<SRNS_DataForwardCommandProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SRNS_DataForwardCommandProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SRNS_DataForwardCommandProtocolExtensions(
    Vec<SRNS_DataForwardCommandProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SRVCC_CSKeysRequestProtocolIEs_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct SRVCC_CSKeysRequestProtocolIEs(Vec<SRVCC_CSKeysRequestProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SRVCC_CSKeysRequestProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SRVCC_CSKeysRequestProtocolExtensions(Vec<SRVCC_CSKeysRequestProtocolExtensions_Entry>);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SRVCC_CSKeysResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: SRVCC_CSKeysResponseProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct SRVCC_CSKeysResponseProtocolIEs(Vec<SRVCC_CSKeysResponseProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SRVCC_CSKeysResponseProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SRVCC_CSKeysResponseProtocolExtensions(
    Vec<SRVCC_CSKeysResponseProtocolExtensions_Entry>,
);

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
pub struct SRVCC_InformationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SRVCC_InformationIE_Extensions(Vec<SRVCC_InformationIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SecurityModeCommandProtocolIEs_EntryValue {
    #[asn(key = 11)]
    Id_EncryptionInformation(EncryptionInformation),
    #[asn(key = 12)]
    Id_IntegrityProtectionInformation(IntegrityProtectionInformation),
    #[asn(key = 75)]
    Id_KeyStatus(KeyStatus),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecurityModeCommandProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: SecurityModeCommandProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct SecurityModeCommandProtocolIEs(Vec<SecurityModeCommandProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecurityModeCommandProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SecurityModeCommandProtocolExtensions(Vec<SecurityModeCommandProtocolExtensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SecurityModeCompleteProtocolIEs_EntryValue {
    #[asn(key = 5)]
    Id_ChosenEncryptionAlgorithm(ChosenEncryptionAlgorithm),
    #[asn(key = 6)]
    Id_ChosenIntegrityProtectionAlgorithm(ChosenIntegrityProtectionAlgorithm),
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecurityModeCompleteProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: SecurityModeCompleteProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct SecurityModeCompleteProtocolIEs(Vec<SecurityModeCompleteProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecurityModeCompleteProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SecurityModeCompleteProtocolExtensions(
    Vec<SecurityModeCompleteProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SecurityModeRejectProtocolIEs_EntryValue {
    #[asn(key = 4)]
    Id_Cause(Cause),
    #[asn(key = 9)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecurityModeRejectProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: SecurityModeRejectProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct SecurityModeRejectProtocolIEs(Vec<SecurityModeRejectProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecurityModeRejectProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SecurityModeRejectProtocolExtensions(Vec<SecurityModeRejectProtocolExtensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Shared_Network_InformationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Shared_Network_InformationIE_Extensions(
    Vec<Shared_Network_InformationIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SourceRNC_IDIE_Extensions_EntryExtensionValue {
    #[asn(key = 171)]
    Id_ExtendedRNC_ID(ExtendedRNC_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SourceRNC_IDIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: SourceRNC_IDIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SourceRNC_IDIE_Extensions(Vec<SourceRNC_IDIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SourceRNC_ToTargetRNC_TransparentContainerIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value:
        SourceRNC_ToTargetRNC_TransparentContainerIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SourceRNC_ToTargetRNC_TransparentContainerIE_Extensions(
    Vec<SourceRNC_ToTargetRNC_TransparentContainerIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SourceUTRANCellIDIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SourceUTRANCellIDIE_Extensions(Vec<SourceUTRANCellIDIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TAIIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TAIIE_Extensions(Vec<TAIIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "3", sz_ub = "3")]
pub struct OCTET_STRING_62(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TMGIIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TMGIIE_Extensions(Vec<TMGIIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TNLInformationEnhRelInfoReqIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TNLInformationEnhRelInfoReqIE_Extensions(
    Vec<TNLInformationEnhRelInfoReqIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TNLInformationEnhRelInfoResIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TNLInformationEnhRelInfoResIE_Extensions(
    Vec<TNLInformationEnhRelInfoResIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargetENB_IDIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TargetENB_IDIE_Extensions(Vec<TargetENB_IDIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TargetRNC_IDIE_Extensions_EntryExtensionValue {
    #[asn(key = 171)]
    Id_ExtendedRNC_ID(ExtendedRNC_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargetRNC_IDIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: TargetRNC_IDIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TargetRNC_IDIE_Extensions(Vec<TargetRNC_IDIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TargetRNC_ToSourceRNC_TransparentContainerIE_Extensions_EntryExtensionValue {
    #[asn(key = 295)]
    Id_UeApplicationLayerMeasurementSupportIndication(
        UeApplicationLayerMeasurementSupportIndication,
    ),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargetRNC_ToSourceRNC_TransparentContainerIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value:
        TargetRNC_ToSourceRNC_TransparentContainerIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TargetRNC_ToSourceRNC_TransparentContainerIE_Extensions(
    Vec<TargetRNC_ToSourceRNC_TransparentContainerIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TrCH_IDIE_Extensions_EntryExtensionValue {
    #[asn(key = 160)]
    Id_E_DCH_MAC_d_Flow_ID(E_DCH_MAC_d_Flow_ID),
    #[asn(key = 117)]
    Id_hS_DSCH_MAC_d_Flow_ID(HS_DSCH_MAC_d_Flow_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TrCH_IDIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: TrCH_IDIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TrCH_IDIE_Extensions(Vec<TrCH_IDIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TraceInformationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TraceInformationIE_Extensions(Vec<TraceInformationIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TracePropagationParametersIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TracePropagationParametersIE_Extensions(
    Vec<TracePropagationParametersIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TraceRecordingSessionInformationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TraceRecordingSessionInformationIE_Extensions(
    Vec<TraceRecordingSessionInformationIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TransportLayerInformationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TransportLayerInformationIE_Extensions(
    Vec<TransportLayerInformationIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TunnelInformationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TunnelInformationIE_Extensions(Vec<TunnelInformationIE_Extensions_Entry>);

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
pub struct UE_IsNotServedIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UE_IsNotServedIE_Extensions(Vec<UE_IsNotServedIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UE_IsServedIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UE_IsServedIE_Extensions(Vec<UE_IsServedIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UESBI_IuIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UESBI_IuIE_Extensions(Vec<UESBI_IuIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UESpecificInformationIndicationProtocolIEs_EntryValue {
    #[asn(key = 118)]
    Id_UESBI_Iu(UESBI_Iu),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UESpecificInformationIndicationProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UESpecificInformationIndicationProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UESpecificInformationIndicationProtocolIEs(
    Vec<UESpecificInformationIndicationProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UESpecificInformationIndicationProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UESpecificInformationIndicationProtocolExtensions(
    Vec<UESpecificInformationIndicationProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UPInformationIE_Extensions_EntryExtensionValue {
    #[asn(key = 269)]
    Id_TimingDifferenceULDL(TimingDifferenceULDL),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UPInformationIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: UPInformationIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UPInformationIE_Extensions(Vec<UPInformationIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UTRAN_CellIDIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UTRAN_CellIDIE_Extensions(Vec<UTRAN_CellIDIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeRadioCapabilityMatchRequestProtocolIEs_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UeRadioCapabilityMatchRequestProtocolIEs(
    Vec<UeRadioCapabilityMatchRequestProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeRadioCapabilityMatchRequestProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UeRadioCapabilityMatchRequestProtocolExtensions(
    Vec<UeRadioCapabilityMatchRequestProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeRadioCapabilityMatchResponseProtocolIEs_EntryValue {
    #[asn(key = 258)]
    Id_VoiceSupportMatchIndicator(VoiceSupportMatchIndicator),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeRadioCapabilityMatchResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UeRadioCapabilityMatchResponseProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UeRadioCapabilityMatchResponseProtocolIEs(
    Vec<UeRadioCapabilityMatchResponseProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeRadioCapabilityMatchResponseProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UeRadioCapabilityMatchResponseProtocolExtensions(
    Vec<UeRadioCapabilityMatchResponseProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeRegistrationQueryRequestProtocolIEs_EntryValue {
    #[asn(key = 79)]
    Id_IuSigConId(IuSignallingConnectionIdentifier),
    #[asn(key = 23)]
    Id_PermanentNAS_UE_ID(PermanentNAS_UE_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeRegistrationQueryRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UeRegistrationQueryRequestProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UeRegistrationQueryRequestProtocolIEs(Vec<UeRegistrationQueryRequestProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeRegistrationQueryRequestProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UeRegistrationQueryRequestProtocolExtensions(
    Vec<UeRegistrationQueryRequestProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeRegistrationQueryResponseProtocolIEs_EntryValue {
    #[asn(key = 281)]
    Id_UERegistrationQueryResult(UERegistrationQueryResult),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeRegistrationQueryResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UeRegistrationQueryResponseProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UeRegistrationQueryResponseProtocolIEs(
    Vec<UeRegistrationQueryResponseProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeRegistrationQueryResponseProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UeRegistrationQueryResponseProtocolExtensions(
    Vec<UeRegistrationQueryResponseProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UnsuccessfulLinking_IEs_EntryIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UnsuccessfulLinking_IEs_EntryIE_Extensions(
    Vec<UnsuccessfulLinking_IEs_EntryIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UnsuccessfulLinking_IEs_Entry {
    pub tmgi: TMGI,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UnsuccessfulLinking_IEs_EntryIE_Extensions>,
}

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkInformationExchangeFailureProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UplinkInformationExchangeFailureProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UplinkInformationExchangeFailureProtocolIEs(
    Vec<UplinkInformationExchangeFailureProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkInformationExchangeFailureProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UplinkInformationExchangeFailureProtocolExtensions(
    Vec<UplinkInformationExchangeFailureProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkInformationExchangeRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UplinkInformationExchangeRequestProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UplinkInformationExchangeRequestProtocolIEs(
    Vec<UplinkInformationExchangeRequestProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkInformationExchangeRequestProtocolExtensions_EntryExtensionValue {
    #[asn(key = 171)]
    Id_ExtendedRNC_ID(ExtendedRNC_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkInformationExchangeRequestProtocolExtensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: UplinkInformationExchangeRequestProtocolExtensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UplinkInformationExchangeRequestProtocolExtensions(
    Vec<UplinkInformationExchangeRequestProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkInformationExchangeResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UplinkInformationExchangeResponseProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UplinkInformationExchangeResponseProtocolIEs(
    Vec<UplinkInformationExchangeResponseProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkInformationExchangeResponseProtocolExtensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UplinkInformationExchangeResponseProtocolExtensions(
    Vec<UplinkInformationExchangeResponseProtocolExtensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserPlaneInformationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UserPlaneInformationIE_Extensions(Vec<UserPlaneInformationIE_Extensions_Entry>);

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
